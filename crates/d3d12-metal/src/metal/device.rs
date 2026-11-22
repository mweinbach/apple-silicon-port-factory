//! The Metal device behind the single DXGI adapter / D3D12 device.

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

static LIVE_HEAP_BYTES: AtomicU64 = AtomicU64::new(0);
static LIVE_COMMITTED_BUF_BYTES: AtomicU64 = AtomicU64::new(0);
static LIVE_COMMITTED_TEX_BYTES: AtomicU64 = AtomicU64::new(0);
static LIVE_DESC_HEAP_BYTES: AtomicU64 = AtomicU64::new(0);
static LIVE_UPLOAD_RING_BYTES: AtomicU64 = AtomicU64::new(0);
static LIVE_ICB_BYTES: AtomicU64 = AtomicU64::new(0);

#[inline]
pub(crate) fn note_heap_alloc(bytes: u64) {
    LIVE_HEAP_BYTES.fetch_add(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_heap_free(bytes: u64) {
    LIVE_HEAP_BYTES.fetch_sub(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_cbuf_alloc(bytes: u64) {
    LIVE_COMMITTED_BUF_BYTES.fetch_add(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_cbuf_free(bytes: u64) {
    LIVE_COMMITTED_BUF_BYTES.fetch_sub(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_ctex_alloc(bytes: u64) {
    LIVE_COMMITTED_TEX_BYTES.fetch_add(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_ctex_free(bytes: u64) {
    LIVE_COMMITTED_TEX_BYTES.fetch_sub(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_desc_alloc(bytes: u64) {
    LIVE_DESC_HEAP_BYTES.fetch_add(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_desc_free(bytes: u64) {
    LIVE_DESC_HEAP_BYTES.fetch_sub(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_ring_alloc(bytes: u64) {
    LIVE_UPLOAD_RING_BYTES.fetch_add(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_ring_free(bytes: u64) {
    LIVE_UPLOAD_RING_BYTES.fetch_sub(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_icb_alloc(bytes: u64) {
    LIVE_ICB_BYTES.fetch_add(bytes, Ordering::Relaxed);
}
#[inline]
pub(crate) fn note_icb_free(bytes: u64) {
    LIVE_ICB_BYTES.fetch_sub(bytes, Ordering::Relaxed);
}

pub type MtlDevice = ProtocolObject<dyn MTLDevice>;

/// Floor of the reported budget: the game's `d3d12_init_and_check` warns
/// when `DedicatedVideoMemory <= 0x157BFFFFF` (~5.37 GiB).
pub const MIN_REPORTED_VRAM: u64 = 0x1_5800_0000;
/// Ceiling of the reported budget: unified-memory Macs report hundreds of
/// GiB of working set, which overflows/oversizes the game's pool math.
pub const MAX_REPORTED_VRAM: u64 = 48 << 30;

/// Thread-safe wrapper (MTLDevice is documented as thread-safe).
pub struct Device {
    pub dev: Retained<MtlDevice>,
    pub name: String,
    pub registry_id: u64,
    pub unified_memory: bool,
    pub recommended_working_set: u64,
    pub metal4: bool,
    /// Device-wide residency set: everything the GPU may touch through
    /// descriptor heaps or GPU VAs lives here (Metal 4 has no implicit
    /// residency). Commit is batched: see [`Device::commit_residency`].
    pub residency: Retained<ProtocolObject<dyn MTLResidencySet>>,
    residency_dirty: Mutex<bool>,
}
unsafe impl Send for Device {}
unsafe impl Sync for Device {}

#[link(name = "Metal", kind = "framework")]
extern "C" {}
#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}

/// The system default Metal device, or `None` (no GPU / no Metal 4).
pub fn device() -> Option<&'static Device> {
    static D: OnceLock<Option<Device>> = OnceLock::new();
    D.get_or_init(|| {
        objc2::rc::autoreleasepool(|_| {
            super::gpucapture::pre_device_init();
            let dev = MTLCreateSystemDefaultDevice()?;
            let metal4 = dev.supportsFamily(MTLGPUFamily::Metal4);
            if !metal4 {
                log::error!(target: "d3d", "{}: Metal 4 not supported; no D3D12 adapter", dev.name());
                return None;
            }
            let rsd = MTLResidencySetDescriptor::new();
            rsd.setLabel(Some(&objc2_foundation::NSString::from_str("port d3d12 device residency")));
            let residency = match dev.newResidencySetWithDescriptor_error(&rsd) {
                Ok(r) => r,
                Err(e) => {
                    log::error!(target: "d3d", "newResidencySet failed: {e}");
                    return None;
                }
            };
            let m = Device {
                name: dev.name().to_string(),
                registry_id: dev.registryID(),
                unified_memory: dev.hasUnifiedMemory(),
                recommended_working_set: dev.recommendedMaxWorkingSetSize(),
                metal4,
                residency,
                residency_dirty: Mutex::new(false),
                dev,
            };
            log::info!(
                target: "d3d",
                "Metal device: {} (registry {:#x}, unified {}, recommended working set {} MiB, reported VRAM {} MiB, translated {})",
                m.name,
                m.registry_id,
                m.unified_memory,
                m.recommended_working_set >> 20,
                m.reported_vram() >> 20,
                super::is_translated()
            );
            Some(m)
        })
    })
    .as_ref()
}

impl Device {
    /// What DXGI reports as `DedicatedVideoMemory` in `IDXGIAdapter::GetDesc*`:
    /// `PORT_VRAM_MB` / `AW2_VRAM_MB` if set; on iOS defaults to `MIN_REPORTED_VRAM`
    /// (~5.375 GiB, satisfying desktop minimum-VRAM startup gates without over-reporting
    /// 8+ GiB on unified-memory phones where CPU + GPU share one Jetsam limit); on macOS
    /// uses `recommendedMaxWorkingSetSize` clamped to [`MIN_REPORTED_VRAM`, `MAX_REPORTED_VRAM`].
    pub fn reported_vram(&self) -> u64 {
        if let Some(mb) = std::env::var("PORT_VRAM_MB")
            .or_else(|_| std::env::var("AW2_VRAM_MB"))
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
        {
            return mb << 20;
        }
        if cfg!(target_os = "ios") {
            return self.recommended_working_set.min(MIN_REPORTED_VRAM).max(MIN_REPORTED_VRAM);
        }
        self.recommended_working_set.clamp(MIN_REPORTED_VRAM, MAX_REPORTED_VRAM)
    }

    /// Live `(budget, current_usage)` for `IDXGIAdapter3::QueryVideoMemoryInfo`.
    ///
    /// On iOS, CPU heaps, static AOT code, Metal compiler scratch, and GPU allocations
    /// all share one process Jetsam ceiling. Computing the live GPU budget from
    /// `os_proc_available_memory()` (with a safety reserve) tells D3D12 texture/resource
    /// streamers to stop growing and evict mips before iOS sends `SIGKILL`.
    pub fn dynamic_vram_budget(&self) -> (u64, u64) {
        let reported = self.reported_vram();
        let gpu_used = self.current_allocated();
        if let Some(avail) = available_headroom_bytes() {
            let cap = reported.min(ios_gpu_budget_cap_bytes_for(process_phys_footprint(), avail));
            let budget = compute_ios_vram_budget(cap, gpu_used, avail, ios_jetsam_headroom_bytes());
            (budget, gpu_used)
        } else {
            (reported, gpu_used)
        }
    }

    /// Stable 16-bit PCI-style device id: FNV-1a of the device name.
    pub fn pci_device_id(&self) -> u32 {
        let mut h: u32 = 0x811c_9dc5;
        for b in self.name.bytes() {
            h ^= b as u32;
            h = h.wrapping_mul(0x0100_0193);
        }
        (h ^ (h >> 16)) & 0xFFFF
    }

    /// Bytes currently allocated on the device (including private DXR acceleration structures).
    pub fn current_allocated(&self) -> u64 {
        (self.dev.currentAllocatedSize() as u64).saturating_add(crate::rt::accel::stats().private_bytes)
    }

    pub fn add_resident(&self, a: &ProtocolObject<dyn MTLAllocation>) {
        let mut d = self.residency_dirty.lock().unwrap();
        self.residency.addAllocation(a);
        super::encstats::inc(&super::encstats::c::res_add);
        *d = true;
    }

    pub fn remove_resident(&self, a: &ProtocolObject<dyn MTLAllocation>) {
        let mut d = self.residency_dirty.lock().unwrap();
        self.residency.removeAllocation(a);
        super::encstats::inc(&super::encstats::c::res_remove);
        *d = true;
    }

    /// Commit pending residency changes (called before each queue submit).
    pub fn commit_residency(&self) {
        let mut d = self.residency_dirty.lock().unwrap();
        if *d {
            self.residency.commit();
            super::encstats::inc(&super::encstats::c::res_commit);
            *d = false;
        }
    }
}

/// Safety reserve kept below the iOS Jetsam ceiling (`PORT_IOS_JETSAM_HEADROOM_MB`, default 1536 MiB)
/// for CPU allocations, Metal shader compiler scratch, and level-load spikes.
pub fn ios_jetsam_headroom_bytes() -> u64 {
    static H: OnceLock<u64> = OnceLock::new();
    *H.get_or_init(|| {
        std::env::var("PORT_IOS_JETSAM_HEADROOM_MB")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .unwrap_or(1536)
            << 20
    })
}

/// Upper bound on DXGI `QueryVideoMemoryInfo` local budget on iOS.
///
/// If `PORT_IOS_GPU_BUDGET_CAP_MB` is set, uses that explicit cap. Otherwise scales
/// proportionally to the process's actual Jetsam ceiling (`phys_footprint + proc_avail`,
/// 45% clamped to `[1536 MiB, 4096 MiB]`) so 6 GiB devices stay near ~1.8 GiB while
/// 8+ GiB Jetsam devices (e.g. iPhone 17 Pro Max, ~8192 MiB ceiling) give D3D12 texture
/// streamers ~3.6 GiB instead of permanently reporting `CurrentUsage > Budget` and
/// triggering non-stop mip eviction/thrashing.
pub fn compute_ios_gpu_budget_cap(phys_footprint: u64, proc_avail: u64) -> u64 {
    const MIN_CAP: u64 = 1536 << 20;
    const MAX_CAP: u64 = 4096 << 20;
    let total_jetsam = phys_footprint.saturating_add(proc_avail);
    ((total_jetsam / 100).saturating_mul(45)).clamp(MIN_CAP, MAX_CAP)
}

pub fn ios_gpu_budget_cap_bytes_for(phys_footprint: u64, proc_avail: u64) -> u64 {
    static ENV_CAP: OnceLock<Option<u64>> = OnceLock::new();
    let explicit = *ENV_CAP.get_or_init(|| {
        std::env::var("PORT_IOS_GPU_BUDGET_CAP_MB")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .map(|mb| mb << 20)
    });
    explicit.unwrap_or_else(|| compute_ios_gpu_budget_cap(phys_footprint, proc_avail))
}

/// Pure iOS Jetsam-safe DXGI local memory budget formula (unit-tested on macOS too).
///
/// - When `proc_avail > headroom`: allows GPU usage to grow by `proc_avail - headroom`
///   up to `reported_vram`.
/// - When `proc_avail <= headroom`: pulls `budget` down below `gpu_used` by the shortfall
///   (down to a `512 MiB` floor) so D3D12 streamers observe `current_usage > budget` and
///   evict mips / transient pools proactively.
pub fn compute_ios_vram_budget(reported_vram: u64, gpu_used: u64, proc_avail: u64, headroom: u64) -> u64 {
    const MIN_BUDGET: u64 = 512 << 20;
    let raw = if proc_avail >= headroom {
        gpu_used.saturating_add(proc_avail - headroom)
    } else {
        gpu_used.saturating_sub(headroom - proc_avail)
    };
    raw.clamp(MIN_BUDGET.min(reported_vram), reported_vram)
}

/// Mach `task_vm_info` breakdown used by [`process_phys_footprint`] and [`log_memory_snapshot`].
#[derive(Clone, Copy, Default, Debug)]
struct VmSnapshot {
    phys_footprint: u64,
    internal: u64,
    compressed: u64,
    device: u64,
}

fn process_vm_snapshot() -> VmSnapshot {
    // Mach `TASK_VM_INFO` (flavor 22). Only the prefix up to `phys_footprint` is needed;
    // use a generous `u32` buffer matching `TASK_VM_INFO_COUNT` across Darwin versions.
    #[repr(C)]
    struct TaskVmInfoPrefix {
        virtual_size: u64,
        region_count: i32,
        page_size: i32,
        resident_size: u64,
        resident_size_peak: u64,
        device: u64,
        device_peak: u64,
        internal: u64,
        internal_peak: u64,
        external: u64,
        external_peak: u64,
        reusable: u64,
        reusable_peak: u64,
        purgeable_volatile_pmap: u64,
        purgeable_volatile_resident: u64,
        purgeable_volatile_virtual: u64,
        compressed: u64,
        compressed_peak: u64,
        compressed_lifetime: u64,
        phys_footprint: u64,
        _rest: [u64; 32],
    }
    const TASK_VM_INFO: i32 = 22;
    unsafe extern "C" {
        fn mach_task_self() -> u32;
        fn task_info(target_task: u32, flavor: i32, task_info_out: *mut u32, task_info_out_cnt: *mut u32) -> i32;
    }
    unsafe {
        let mut info: TaskVmInfoPrefix = std::mem::zeroed();
        let mut count = (std::mem::size_of::<TaskVmInfoPrefix>() / std::mem::size_of::<u32>()) as u32;
        let rc = task_info(mach_task_self(), TASK_VM_INFO, &mut info as *mut _ as *mut u32, &mut count);
        if rc == 0 {
            VmSnapshot {
                phys_footprint: info.phys_footprint,
                internal: info.internal,
                compressed: info.compressed,
                device: info.device,
            }
        } else {
            VmSnapshot::default()
        }
    }
}

/// Current process physical memory footprint in bytes (`task_vm_info.phys_footprint`),
/// the exact accounting metric iOS Jetsam enforces.
pub fn process_phys_footprint() -> u64 {
    process_vm_snapshot().phys_footprint
}

/// Remaining bytes before iOS Jetsam terminates this process (`os_proc_available_memory()`).
/// Returns `None` on macOS (no hard per-app Jetsam limit).
pub fn available_headroom_bytes() -> Option<u64> {
    #[cfg(target_os = "ios")]
    {
        unsafe extern "C" {
            fn os_proc_available_memory() -> usize;
        }
        Some(unsafe { os_proc_available_memory() } as u64)
    }
    #[cfg(not(target_os = "ios"))]
    {
        None
    }
}

/// Log a compact snapshot of Darwin physical footprint, iOS Jetsam headroom, and Metal GPU usage.
pub fn log_memory_snapshot(tag: &str) {
    let vm = process_vm_snapshot();
    let fp_mb = vm.phys_footprint as f64 / (1024.0 * 1024.0);
    let int_mb = vm.internal as f64 / (1024.0 * 1024.0);
    let cmp_mb = vm.compressed as f64 / (1024.0 * 1024.0);
    let dev_mb = vm.device as f64 / (1024.0 * 1024.0);
    let (budget_mb, gpu_mb) = device()
        .map(|d| {
            let (b, u) = d.dynamic_vram_budget();
            (b as f64 / (1024.0 * 1024.0), u as f64 / (1024.0 * 1024.0))
        })
        .unwrap_or((0.0, 0.0));
    let heaps_mb = LIVE_HEAP_BYTES.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0);
    let cbuf_mb = LIVE_COMMITTED_BUF_BYTES.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0);
    let ctex_mb = LIVE_COMMITTED_TEX_BYTES.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0);
    let desc_mb = LIVE_DESC_HEAP_BYTES.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0);
    let ring_mb = LIVE_UPLOAD_RING_BYTES.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0);
    let icb_mb = LIVE_ICB_BYTES.load(Ordering::Relaxed) as f64 / (1024.0 * 1024.0);
    if let Some(avail) = available_headroom_bytes() {
        let avail_mb = avail as f64 / (1024.0 * 1024.0);
        log::info!(
            target: "d3d",
            "mem[{tag}]: phys_footprint={fp_mb:.1} MiB (int={int_mb:.0} cmp={cmp_mb:.0} dev={dev_mb:.0}), ios_avail={avail_mb:.1} MiB, gpu_alloc={gpu_mb:.1} MiB (heaps={heaps_mb:.0} cbuf={cbuf_mb:.0} ctex={ctex_mb:.0} desc={desc_mb:.0} ring={ring_mb:.0} icb={icb_mb:.0}), dxgi_budget={budget_mb:.1} MiB"
        );
    } else {
        log::info!(
            target: "d3d",
            "mem[{tag}]: phys_footprint={fp_mb:.1} MiB (int={int_mb:.0} cmp={cmp_mb:.0} dev={dev_mb:.0}), gpu_alloc={gpu_mb:.1} MiB (heaps={heaps_mb:.0} cbuf={cbuf_mb:.0} ctex={ctex_mb:.0} desc={desc_mb:.0} ring={ring_mb:.0} icb={icb_mb:.0}), dxgi_budget={budget_mb:.1} MiB"
        );
    }
}

/// Current Darwin thermal state via `NSProcessInfo.processInfo.thermalState`:
/// `0` = Nominal, `1` = Fair, `2` = Serious, `3` = Critical (`-1` if unavailable).
pub fn thermal_state() -> i32 {
    use objc2::runtime::AnyObject;
    use objc2::{class, msg_send};
    objc2::rc::autoreleasepool(|_| unsafe {
        let info: *mut AnyObject = msg_send![class!(NSProcessInfo), processInfo];
        if info.is_null() {
            return -1;
        }
        let state: isize = msg_send![info, thermalState];
        state as i32
    })
}

/// Physical RAM in bytes.
pub fn physical_memory() -> u64 {
    let mut v: u64 = 0;
    let mut len = std::mem::size_of::<u64>();
    unsafe extern "C" {
        fn sysctlbyname(name: *const u8, oldp: *mut std::ffi::c_void, oldlenp: *mut usize, newp: *const std::ffi::c_void, newlen: usize) -> i32;
    }
    unsafe { sysctlbyname(b"hw.memsize\0".as_ptr(), &mut v as *mut u64 as *mut _, &mut len, std::ptr::null(), 0) };
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ios_vram_budget_tracks_jetsam_headroom() {
        let reported = MIN_REPORTED_VRAM; // 5504 MiB
        let headroom = 1024 << 20; // 1 GiB
        // Plenty of headroom (3 GiB avail, 1 GiB used) -> budget = 1 + 2 = 3 GiB
        assert_eq!(compute_ios_vram_budget(reported, 1024 << 20, 3072 << 20, headroom), 3072 << 20);
        // Very high headroom never exceeds reported VRAM
        assert_eq!(compute_ios_vram_budget(reported, 2048 << 20, 8192 << 20, headroom), reported);
        // Inside pressure band (512 MiB avail < 1024 MiB headroom, 2048 MiB used) -> budget drops to 1536 MiB (< gpu_used!)
        let pressured = compute_ios_vram_budget(reported, 2048 << 20, 512 << 20, headroom);
        assert_eq!(pressured, 1536 << 20);
        assert!(pressured < (2048 << 20));
        // Extreme pressure floors at 512 MiB (never zero)
        assert_eq!(compute_ios_vram_budget(reported, 600 << 20, 10 << 20, headroom), 512 << 20);
    }

    #[test]
    fn ios_gpu_budget_cap_scales_with_jetsam_ceiling() {
        // 4 GiB Jetsam ceiling -> 45% = ~1843 MiB (safe for 6 GiB iPhones)
        let cap_4g = compute_ios_gpu_budget_cap(2048 << 20, 2048 << 20);
        assert!(cap_4g >= (1800 << 20) && cap_4g <= (1900 << 20), "cap_4g={}", cap_4g >> 20);
        // 8 GiB Jetsam ceiling (iPhone 17 Pro Max: 5739 MiB phys + 2453 MiB avail) -> ~3686 MiB
        // so 2686 MiB GPU usage has real headroom instead of permanent panic-eviction at 1536 MiB!
        let cap_8g = compute_ios_gpu_budget_cap(5739 << 20, 2453 << 20);
        assert!(cap_8g >= (3600 << 20) && cap_8g <= (3800 << 20), "cap_8g={}", cap_8g >> 20);
        let budget_8g = compute_ios_vram_budget(cap_8g, 2686 << 20, 2453 << 20, 1536 << 20);
        assert!(budget_8g > (3500 << 20), "expected healthy headroom above 2686 MiB, got {} MiB", budget_8g >> 20);
        // Under real Jetsam pressure (avail < 1536 MiB), still pulls budget below gpu_used!
        let pressured_8g = compute_ios_vram_budget(cap_8g, 2686 << 20, 1024 << 20, 1536 << 20);
        assert!(pressured_8g < (2686 << 20), "pressured_8g={} MiB", pressured_8g >> 20);
        // Huge 16 GiB device caps cleanly at 4096 MiB
        assert_eq!(compute_ios_gpu_budget_cap(4096 << 20, 12288 << 20), 4096 << 20);
        // Tiny device floors at 1536 MiB
        assert_eq!(compute_ios_gpu_budget_cap(1024 << 20, 1024 << 20), 1536 << 20);
    }

    #[test]
    fn process_phys_footprint_nonzero_on_darwin() {
        let fp = process_phys_footprint();
        assert!(fp > 0, "expected non-zero Mach task_vm_info.phys_footprint, got {fp}");
    }

    #[test]
    fn thermal_state_in_valid_range() {
        let st = thermal_state();
        assert!((0..=3).contains(&st), "expected NSProcessInfoThermalState in 0..=3, got {st}");
    }
}
