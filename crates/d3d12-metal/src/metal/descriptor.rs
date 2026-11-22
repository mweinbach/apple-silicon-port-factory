//! Descriptor heaps.
//!
//! * CBV/SRV/UAV and sampler heaps hold 24-byte `IRDescriptorTableEntry`s
//!   (the layout `dxil-metal`' converted shaders read). Shader-visible
//!   heaps live in a shared `MTLBuffer` (CPU handle = `contents`, GPU handle
//!   = `gpuAddress`, same index math); non-visible heaps are host memory
//!   that `CopyDescriptors*` memcpy from.
//! * RTV/DSV heaps are host memory holding 32-byte
//!   [`TargetView`](super::cmdlist::TargetView) records.

use super::resource::MtlBuffer;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

/// `D3D12_DESCRIPTOR_HEAP_TYPE`.
pub mod heap_kind {
    pub const CBV_SRV_UAV: u32 = 0;
    pub const SAMPLER: u32 = 1;
    pub const RTV: u32 = 2;
    pub const DSV: u32 = 3;
}

/// Bytes per descriptor (`GetDescriptorHandleIncrementSize`).
pub const RESOURCE_DESCRIPTOR_SIZE: u32 = 24;
pub const TARGET_DESCRIPTOR_SIZE: u32 = 32;

pub fn descriptor_size(kind: u32) -> Option<u32> {
    match kind {
        heap_kind::CBV_SRV_UAV | heap_kind::SAMPLER => Some(RESOURCE_DESCRIPTOR_SIZE),
        heap_kind::RTV | heap_kind::DSV => Some(TARGET_DESCRIPTOR_SIZE),
        _ => None,
    }
}

/// `PORT_DESC_HEAP_CACHED=1` / `AW2_DESC_HEAP_CACHED=1`: shader-visible heaps use
/// the default (cached) CPU mode instead of write-combined (A/B knob; shared storage
/// is CPU/GPU coherent on Apple silicon either way, and CPU reads of write-combined
/// memory are uncached).
fn heap_cpu_cached() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var("PORT_DESC_HEAP_CACHED")
            .or_else(|_| std::env::var("AW2_DESC_HEAP_CACHED"))
            .is_ok_and(|v| v == "1")
    })
}

enum Storage {
    Host(#[allow(dead_code)] Box<[u64]>),
    Gpu(Retained<MtlBuffer>),
}

pub struct DescriptorHeap {
    pub kind: u32,
    pub count: u32,
    pub shader_visible: bool,
    /// CPU handle of descriptor 0. After a direct CBV/SRV/UAV write, call
    /// [`bump_desc_heap_gen`] before recording commands that consume the new entry.
    pub cpu_base: usize,
    /// GPU handle of descriptor 0 (0 when not shader-visible).
    pub gpu_base: u64,
    gpu_bytes: u64,
    storage: Storage,
}
unsafe impl Send for DescriptorHeap {}
unsafe impl Sync for DescriptorHeap {}

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{OnceLock, RwLock};

static DESC_HEAP_GEN: AtomicU64 = AtomicU64::new(1);

#[inline]
pub(crate) fn desc_heap_gen() -> u64 {
    DESC_HEAP_GEN.load(Ordering::Acquire)
}

/// Publish completed descriptor writes to the compute binding cache. Unit tests
/// enable the cache per command list independently of the process-wide switch.
#[inline]
pub fn bump_desc_heap_gen() {
    if cfg!(test) || super::cmdlist::opt_cs_bind_intra_safe_env() {
        DESC_HEAP_GEN.fetch_add(1, Ordering::Release);
    }
}

#[derive(Clone, Copy)]
struct LiveHeap {
    gpu_base: u64,
    byte_len: u64,
    cpu_base: usize,
}

fn live_heaps() -> &'static RwLock<Vec<LiveHeap>> {
    static H: OnceLock<RwLock<Vec<LiveHeap>>> = OnceLock::new();
    H.get_or_init(|| RwLock::new(Vec::new()))
}

/// Resolve a shader-visible CBV/SRV/UAV heap GPU address to `(gpu_base, byte_len, cpu_base)`.
pub fn resolve_shader_heap(gpu_addr: u64) -> Option<(u64, u64, usize)> {
    if gpu_addr == 0 {
        return None;
    }
    let hs = live_heaps().read().unwrap();
    hs.iter()
        .find(|h| gpu_addr >= h.gpu_base && gpu_addr < h.gpu_base.saturating_add(h.byte_len))
        .map(|h| (h.gpu_base, h.byte_len, h.cpu_base))
}

impl DescriptorHeap {
    pub fn new(kind: u32, count: u32, shader_visible: bool, label: &str) -> Result<DescriptorHeap, String> {
        let size = descriptor_size(kind).ok_or_else(|| format!("descriptor heap type {kind}"))?;
        if shader_visible && !matches!(kind, heap_kind::CBV_SRV_UAV | heap_kind::SAMPLER) {
            return Err("RTV/DSV heaps cannot be shader-visible".into());
        }
        let bytes = (count.max(1) as usize) * size as usize;
        let (storage, cpu_base, gpu_base, gpu_bytes) = if shader_visible {
            let d = super::device().ok_or("no device")?;
            let cache = if heap_cpu_cached() { MTLResourceOptions::CPUCacheModeDefaultCache } else { MTLResourceOptions::CPUCacheModeWriteCombined };
            let opts = MTLResourceOptions::StorageModeShared | cache | MTLResourceOptions::HazardTrackingModeUntracked;
            let b = d.dev.newBufferWithLength_options(bytes, opts).ok_or_else(|| format!("newBuffer({bytes}) for a descriptor heap"))?;
            if !label.is_empty() {
                b.setLabel(Some(&objc2_foundation::NSString::from_str(label)));
            }
            unsafe { std::ptr::write_bytes(b.contents().as_ptr() as *mut u8, 0, bytes) };
            let alloc = b.length() as u64;
            super::device::note_desc_alloc(alloc);
            d.add_resident(ProtocolObject::from_ref(&*b));
            let cpu = b.contents().as_ptr() as usize;
            let gpu = b.gpuAddress();
            if kind == heap_kind::CBV_SRV_UAV {
                live_heaps().write().unwrap().push(LiveHeap { gpu_base: gpu, byte_len: bytes as u64, cpu_base: cpu });
                bump_desc_heap_gen();
            }
            (Storage::Gpu(b), cpu, gpu, alloc)
        } else {
            let mut v = vec![0u64; bytes.div_ceil(8)].into_boxed_slice();
            let cpu = v.as_mut_ptr() as usize;
            (Storage::Host(v), cpu, 0, 0)
        };
        Ok(DescriptorHeap { kind, count, shader_visible, cpu_base, gpu_base, gpu_bytes, storage })
    }

    pub fn descriptor_size(&self) -> u32 {
        descriptor_size(self.kind).unwrap()
    }

    pub fn buffer(&self) -> Option<&MtlBuffer> {
        match &self.storage {
            Storage::Gpu(b) => Some(b),
            Storage::Host(_) => None,
        }
    }
}

impl Drop for DescriptorHeap {
    fn drop(&mut self) {
        if self.shader_visible && self.kind == heap_kind::CBV_SRV_UAV {
            let mut hs = live_heaps().write().unwrap();
            hs.retain(|h| !(h.gpu_base == self.gpu_base && h.cpu_base == self.cpu_base));
            bump_desc_heap_gen();
        }
        if self.gpu_bytes != 0 {
            super::device::note_desc_free(self.gpu_bytes);
        }
        if let (Storage::Gpu(b), Some(d)) = (&self.storage, super::device()) {
            d.remove_resident(ProtocolObject::from_ref(&**b));
        }
    }
}

// ---------------------------------------------------------------------------
// UAV descriptor metadata tracking for `ClearUnorderedAccessView*`
// ---------------------------------------------------------------------------

#[derive(Default)]
struct UavTrack {
    by_handle: std::collections::HashMap<usize, (usize, Option<[u8; 40]>)>,
    by_res: std::collections::HashMap<usize, Option<[u8; 40]>>,
}

/// CPU descriptor handle -> (resource key, UAV desc) of `CreateUnorderedAccessView`
/// calls: `ClearUnorderedAccessView*` needs the view's format and range,
/// which the Metal descriptor bytes don't carry.
fn uav_descs() -> &'static std::sync::Mutex<UavTrack> {
    static M: OnceLock<std::sync::Mutex<UavTrack>> = OnceLock::new();
    M.get_or_init(Default::default)
}

/// Record the UAV descriptor metadata created at CPU descriptor `handle` for
/// `resource` (`None` for a null descriptor).
pub fn record_uav(handle: usize, resource: Option<&super::resource::Resource>, desc: Option<&[u8; 40]>) {
    if handle == 0 {
        return;
    }
    let res_key = resource.map_or(0, |r| {
        let key = r as *const super::resource::Resource as usize;
        let prev = r.note_uav_tracked_key(key);
        if prev != 0 && prev != key {
            forget_uav_resource(prev);
        }
        key
    });
    let mut m = uav_descs().lock().unwrap();
    m.by_handle.insert(handle, (res_key, desc.copied()));
    if res_key != 0 {
        m.by_res.insert(res_key, desc.copied());
    }
}

/// Remove tracked resource-fallback UAV metadata when a `Resource` is dropped.
pub(crate) fn forget_uav_resource(res_key: usize) {
    if res_key != 0 {
        uav_descs().lock().unwrap().by_res.remove(&res_key);
    }
}

/// The UAV created at CPU handle `cpu` (`(resource_key, desc)`).
pub fn uav_at(cpu: usize) -> Option<(usize, Option<[u8; 40]>)> {
    uav_descs().lock().unwrap().by_handle.get(&cpu).copied()
}

/// Latest UAV desc created for `resource` (fallback if a non-shader-visible CPU
/// handle was overwritten or reused for another resource).
pub fn uav_for_resource(resource: &super::resource::Resource) -> Option<Option<[u8; 40]>> {
    let res_key = resource as *const super::resource::Resource as usize;
    uav_descs().lock().unwrap().by_res.get(&res_key).copied()
}

/// Resolution result for [`resolve_uav_for_clear`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UavClearLookup {
    /// Descriptor resolved cleanly (exact handle match, resource fallback, or
    /// `None` whole-texture clear when no UAV descriptor was tracked).
    Found(Option<[u8; 40]>),
    /// Buffer UAV clear with no tracked UAV descriptor at `cpu` or for `resource`.
    MissingBufferView,
}

/// Resolve the `D3D12_UNORDERED_ACCESS_VIEW_DESC` for `ClearUnorderedAccessView*`
/// at CPU descriptor handle `cpu` targeting `resource`.
pub fn resolve_uav_for_clear(cpu: usize, resource: &super::resource::Resource, what: &str) -> UavClearLookup {
    let res_key = resource as *const super::resource::Resource as usize;
    let m = uav_descs().lock().unwrap();
    match m.by_handle.get(&cpu).copied() {
        Some((vr, d)) if vr == res_key => UavClearLookup::Found(d),
        Some((vr, _)) => {
            let fallback = m.by_res.get(&res_key).copied().flatten();
            log::warn!(
                target: "d3d",
                "{what}: UAV at {cpu:#x} was created for {vr:#x}, clearing {res_key:#x} with resource's own UAV desc (has_desc={})",
                fallback.is_some()
            );
            UavClearLookup::Found(fallback)
        }
        None => match m.by_res.get(&res_key).copied() {
            Some(d) => UavClearLookup::Found(d),
            None if !resource.desc.is_buffer() => UavClearLookup::Found(None),
            None => UavClearLookup::MissingBufferView,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metal::resource::Resource;
    use crate::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};

    #[test]
    fn uav_tracking_resolves_handle_fallback_and_drops_cleanly() {
        if super::super::device().is_none() {
            return;
        }
        let bdesc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: 64,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
            ..Default::default()
        };
        let tdesc = ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Width: 8,
            Height: 8,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: 28,
            SampleCount: 1,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        let buf_a = Resource::new_committed_default(&bdesc, &hp, "uav track buf a").unwrap();
        let buf_b = Resource::new_committed_default(&bdesc, &hp, "uav track buf b").unwrap();
        let tex = Resource::new_committed_default(&tdesc, &hp, "uav track tex").unwrap();

        let mut desc_a = [0u8; 40];
        desc_a[0..4].copy_from_slice(&42u32.to_le_bytes());
        desc_a[4..8].copy_from_slice(&1u32.to_le_bytes());
        desc_a[16..20].copy_from_slice(&16u32.to_le_bytes());

        let mut desc_b = [0u8; 40];
        desc_b[0..4].copy_from_slice(&41u32.to_le_bytes());
        desc_b[4..8].copy_from_slice(&1u32.to_le_bytes());
        desc_b[16..20].copy_from_slice(&8u32.to_le_bytes());

        // Missing buffer view traps cleanly, while untracked texture defaults to whole-resource None.
        assert_eq!(resolve_uav_for_clear(0xdead_0001, &buf_a, "test"), UavClearLookup::MissingBufferView);
        assert_eq!(resolve_uav_for_clear(0xdead_0001, &tex, "test"), UavClearLookup::Found(None));

        // Exact handle match.
        let h_a = 0xfeed_1000usize;
        record_uav(h_a, Some(&buf_a), Some(&desc_a));
        assert_eq!(resolve_uav_for_clear(h_a, &buf_a, "test"), UavClearLookup::Found(Some(desc_a)));
        assert_eq!(uav_for_resource(&buf_a), Some(Some(desc_a)));

        // Handle overwritten with buf_b -> clearing buf_a via h_a falls back to buf_a's desc.
        record_uav(h_a, Some(&buf_b), Some(&desc_b));
        assert_eq!(resolve_uav_for_clear(h_a, &buf_b, "test"), UavClearLookup::Found(Some(desc_b)));
        assert_eq!(resolve_uav_for_clear(h_a, &buf_a, "test"), UavClearLookup::Found(Some(desc_a)));

        // Unseen handle still resolves via resource fallback.
        assert_eq!(resolve_uav_for_clear(0xfeed_2000, &buf_a, "test"), UavClearLookup::Found(Some(desc_a)));

        // Dropping buf_a cleans up its resource-fallback entry.
        let buf_a_key = &buf_a as *const Resource as usize;
        drop(buf_a);
        assert!(!uav_descs().lock().unwrap().by_res.contains_key(&buf_a_key));
    }
}
