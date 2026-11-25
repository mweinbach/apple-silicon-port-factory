//! System information: CPU features, memory, version, topology.

use crate::strings::*;
use crate::teb::{OS_BUILD, OS_MAJOR, OS_MINOR};
use crate::types::*;
use std::ffi::c_void;

/// Synthetic x86_64 CPUID leaves (AVX2 + FMA + BMI1/2 + SSE4.2 + F16C + MOVBE + POPCNT + LZCNT + CX16)
/// used when running natively on `aarch64` without Rosetta.
pub fn synthetic_cpuid(leaf: u32, sub: u32) -> (u32, u32, u32, u32) {
    match (leaf, sub) {
        (0, _) => (0x16, 0x756e_6547, 0x6c65_746e, 0x4965_6e69),
        (1, _) => (
            0x0009_06ea,
            0x0010_0800,
            (1 << 0)
                | (1 << 9)
                | (1 << 12)
                | (1 << 13)
                | (1 << 19)
                | (1 << 20)
                | (1 << 22)
                | (1 << 23)
                | (1 << 26)
                | (1 << 27)
                | (1 << 28)
                | (1 << 29)
                | (1 << 30),
            (1 << 0)
                | (1 << 4)
                | (1 << 8)
                | (1 << 15)
                | (1 << 19)
                | (1 << 23)
                | (1 << 24)
                | (1 << 25)
                | (1 << 26),
        ),
        (7, 0) => (0, (1 << 3) | (1 << 5) | (1 << 8), 0, 0),
        (0x8000_0000, _) => (0x8000_0008, 0, 0, 0),
        (0x8000_0001, _) => (0, 0, 1 << 5, (1 << 27) | (1 << 29)),
        _ => (0, 0, 0, 0),
    }
}

#[cfg(target_arch = "x86_64")]
fn cpuid(leaf: u32, sub: u32) -> (u32, u32, u32, u32) {
    let r = std::arch::x86_64::__cpuid_count(leaf, sub);
    (r.eax, r.ebx, r.ecx, r.edx)
}

#[cfg(not(target_arch = "x86_64"))]
fn cpuid(leaf: u32, sub: u32) -> (u32, u32, u32, u32) {
    synthetic_cpuid(leaf, sub)
}

/// `IsProcessorFeaturePresent` from the CPUID that Rosetta (or synthetic CPUID on aarch64) reports.
pub fn processor_feature(f: u32) -> bool {
    let (_, _, c1, d1) = cpuid(1, 0);
    let (_, b7, _, _) = cpuid(7, 0);
    let (_, _, _, de1) = cpuid(0x8000_0001, 0);
    let osxsave = c1 >> 27 & 1 == 1;
    let xcr0 = if osxsave {
        #[cfg(target_arch = "x86_64")]
        {
            let lo: u32;
            let hi: u32;
            unsafe { std::arch::asm!("xgetbv", in("ecx") 0, out("eax") lo, out("edx") hi) };
            (hi as u64) << 32 | lo as u64
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            0x7u64
        }
    } else {
        0
    };
    let avx_os = xcr0 & 6 == 6;
    match f {
        0 => false,              // PF_FLOATING_POINT_PRECISION_ERRATA
        1 => false,              // PF_FLOATING_POINT_EMULATED
        2 => d1 >> 8 & 1 == 1,   // PF_COMPARE_EXCHANGE_DOUBLE (cx8)
        3 => d1 >> 23 & 1 == 1,  // PF_MMX_INSTRUCTIONS_AVAILABLE
        6 => d1 >> 25 & 1 == 1,  // PF_XMMI_INSTRUCTIONS_AVAILABLE (SSE)
        7 => false,              // PF_3DNOW
        8 => d1 >> 4 & 1 == 1,   // PF_RDTSC_INSTRUCTION_AVAILABLE
        9 => true,               // PF_PAE_ENABLED
        10 => d1 >> 26 & 1 == 1, // PF_XMMI64 (SSE2)
        12 => true,              // PF_NX_ENABLED
        13 => c1 & 1 == 1,       // PF_SSE3
        14 => c1 >> 13 & 1 == 1, // PF_COMPARE_EXCHANGE128
        15 => true,              // PF_COMPARE64_EXCHANGE128
        16 => false,             // PF_CHANNELS_ENABLED
        17 => osxsave,           // PF_XSAVE_ENABLED
        20 => false,             // PF_SECOND_LEVEL_ADDRESS_TRANSLATION
        21 => false,             // PF_VIRT_FIRMWARE_ENABLED
        22 => false,             // PF_RDWRFSGSBASE_AVAILABLE
        23 => true,              // PF_FASTFAIL_AVAILABLE (int 0x29; the loader decodes it)
        24 => false,             // PF_ARM_DIVIDE
        28 => c1 >> 30 & 1 == 1, // PF_RDRAND
        32 => de1 >> 27 & 1 == 1, // PF_RDTSCP
        36 => c1 >> 9 & 1 == 1,  // PF_SSSE3
        37 => c1 >> 19 & 1 == 1, // PF_SSE4_1
        38 => c1 >> 20 & 1 == 1, // PF_SSE4_2
        39 => c1 >> 28 & 1 == 1 && avx_os, // PF_AVX
        40 => b7 >> 5 & 1 == 1 && avx_os,  // PF_AVX2
        41 => b7 >> 16 & 1 == 1 && xcr0 & 0xe0 == 0xe0, // PF_AVX512F
        _ => false,
    }
}

pub unsafe extern "win64" fn IsProcessorFeaturePresent(f: u32) -> BOOL {
    processor_feature(f) as BOOL
}

#[repr(C)]
pub struct SYSTEM_INFO {
    pub wProcessorArchitecture: u16,
    pub wReserved: u16,
    pub dwPageSize: u32,
    pub lpMinimumApplicationAddress: usize,
    pub lpMaximumApplicationAddress: usize,
    pub dwActiveProcessorMask: usize,
    pub dwNumberOfProcessors: u32,
    pub dwProcessorType: u32,
    pub dwAllocationGranularity: u32,
    pub wProcessorLevel: u16,
    pub wProcessorRevision: u16,
}

pub fn fill_system_info(si: &mut SYSTEM_INFO) {
    let n = crate::process::num_cpus();
    let (a1, _, _, _) = cpuid(1, 0);
    si.wProcessorArchitecture = 9; // AMD64
    si.wReserved = 0;
    si.dwPageSize = 0x1000;
    si.lpMinimumApplicationAddress = 0x10000;
    si.lpMaximumApplicationAddress = 0x7FFF_FFFE_FFFF;
    si.dwActiveProcessorMask = if n >= 64 { usize::MAX } else { (1usize << n) - 1 };
    si.dwNumberOfProcessors = n;
    si.dwProcessorType = 8664;
    si.dwAllocationGranularity = 0x10000;
    si.wProcessorLevel = ((a1 >> 8) & 0xf) as u16;
    si.wProcessorRevision = (a1 & 0xffff) as u16;
}

pub unsafe extern "win64" fn GetSystemInfo(si: *mut SYSTEM_INFO) {
    let mut info = std::mem::zeroed();
    fill_system_info(&mut info);
    si.write_unaligned(info);
}

pub unsafe extern "win64" fn GetNativeSystemInfo(si: *mut SYSTEM_INFO) {
    GetSystemInfo(si);
}

#[repr(C)]
pub struct OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}

pub unsafe extern "win64" fn GetVersionExW(v: *mut OSVERSIONINFOEXW) -> BOOL {
    let size = (*v).dwOSVersionInfoSize;
    if size != 276 && size != 284 {
        crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    (*v).dwMajorVersion = OS_MAJOR;
    (*v).dwMinorVersion = OS_MINOR;
    (*v).dwBuildNumber = OS_BUILD;
    (*v).dwPlatformId = 2;
    (*v).szCSDVersion = [0; 128];
    if size == 284 {
        (*v).wServicePackMajor = 0;
        (*v).wServicePackMinor = 0;
        (*v).wSuiteMask = 0x100; // VER_SUITE_SINGLEUSERTS
        (*v).wProductType = 1; // VER_NT_WORKSTATION
        (*v).wReserved = 0;
    }
    TRUE
}

pub unsafe extern "win64" fn GetVersionExA(v: *mut u8) -> BOOL {
    let size = *(v as *const u32);
    if size != 148 && size != 156 {
        crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    let p = v as *mut u32;
    *p.add(1) = OS_MAJOR;
    *p.add(2) = OS_MINOR;
    *p.add(3) = OS_BUILD;
    *p.add(4) = 2;
    std::ptr::write_bytes(v.add(20), 0, 128);
    if size == 156 {
        std::ptr::write_bytes(v.add(148), 0, 8);
        *(v.add(152) as *mut u16) = 0x100;
        *v.add(154) = 1;
    }
    TRUE
}

pub unsafe extern "win64" fn GetVersion() -> u32 {
    (OS_BUILD << 16) | (OS_MINOR << 8) | OS_MAJOR
}

pub unsafe extern "win64" fn VerSetConditionMask(mask: u64, type_bits: u32, cond: u8) -> u64 {
    if type_bits == 0 {
        return mask;
    }
    let c = (cond & 7) as u64;
    let shift = if type_bits & 0x80 != 0 {
        7
    } else if type_bits & 0x40 != 0 {
        6
    } else if type_bits & 0x20 != 0 {
        5
    } else if type_bits & 0x10 != 0 {
        4
    } else if type_bits & 0x08 != 0 {
        3
    } else if type_bits & 0x04 != 0 {
        2
    } else if type_bits & 0x02 != 0 {
        1
    } else {
        0
    };
    mask | (c << (shift * 3))
}

fn ver_cmp(cond: u8, ord: std::cmp::Ordering) -> bool {
    use std::cmp::Ordering::*;
    match cond {
        1 => ord == Equal,                    // VER_EQUAL
        2 => ord == Greater,                  // VER_GREATER
        3 => ord != Less,                     // VER_GREATER_EQUAL
        4 => ord == Less,                     // VER_LESS
        5 => ord != Greater,                  // VER_LESS_EQUAL
        6 | 7 => ord == Equal,                // VER_AND / VER_OR (approx.)
        _ => false,
    }
}

pub unsafe extern "win64" fn VerifyVersionInfoW(v: *const OSVERSIONINFOEXW, type_mask: u32, cond_mask: u64) -> BOOL {
    let cond = |shift: u32| ((cond_mask >> (shift * 3)) & 7) as u8;
    let cur = (OS_MAJOR, OS_MINOR, 0u16, 0u16);
    let req = &*v;
    let mut ok = true;
    // Major/minor/SP are compared hierarchically like Windows does.
    let fields: [(u32, u32, u64, u64); 4] = [
        (0x02, 1, cur.0 as u64, req.dwMajorVersion as u64),
        (0x01, 0, cur.1 as u64, req.dwMinorVersion as u64),
        (0x20, 5, cur.2 as u64, req.wServicePackMajor as u64),
        (0x10, 4, cur.3 as u64, req.wServicePackMinor as u64),
    ];
    let present: Vec<_> = fields.iter().filter(|f| type_mask & f.0 != 0).collect();
    if let Some(first) = present.first() {
        let c = cond(first.1);
        let a: Vec<u64> = present.iter().map(|f| f.2).collect();
        let b: Vec<u64> = present.iter().map(|f| f.3).collect();
        ok &= ver_cmp(c, a.cmp(&b));
    }
    if type_mask & 0x04 != 0 {
        ok &= ver_cmp(cond(2), (OS_BUILD as u64).cmp(&(req.dwBuildNumber as u64)));
    }
    if type_mask & 0x08 != 0 {
        ok &= ver_cmp(cond(3), 2u64.cmp(&(req.dwPlatformId as u64)));
    }
    if type_mask & 0x80 != 0 {
        ok &= ver_cmp(cond(7), 1u64.cmp(&(req.wProductType as u64)));
    }
    if ok {
        TRUE
    } else {
        crate::teb::set_last_error(1150); // ERROR_OLD_WIN_VERSION
        FALSE
    }
}

pub unsafe extern "win64" fn VerifyVersionInfoA(v: *const u8, type_mask: u32, cond_mask: u64) -> BOOL {
    let mut w: OSVERSIONINFOEXW = std::mem::zeroed();
    let p = v as *const u32;
    w.dwMajorVersion = *p.add(1);
    w.dwMinorVersion = *p.add(2);
    w.dwBuildNumber = *p.add(3);
    w.dwPlatformId = *p.add(4);
    w.wServicePackMajor = *(v.add(148) as *const u16);
    w.wServicePackMinor = *(v.add(150) as *const u16);
    w.wSuiteMask = *(v.add(152) as *const u16);
    w.wProductType = *v.add(154);
    VerifyVersionInfoW(&w, type_mask, cond_mask)
}

#[repr(C)]
pub struct MEMORYSTATUSEX {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub ullTotalPhys: u64,
    pub ullAvailPhys: u64,
    pub ullTotalPageFile: u64,
    pub ullAvailPageFile: u64,
    pub ullTotalVirtual: u64,
    pub ullAvailVirtual: u64,
    pub ullAvailExtendedVirtual: u64,
}

/// Free + inactive + purgeable pages from the Mach VM statistics.
pub fn available_physical_memory() -> u64 {
    extern "C" {
        fn mach_host_self() -> libc::mach_port_t;
    }
    unsafe {
        let mut stats: libc::vm_statistics64 = std::mem::zeroed();
        let mut count = (std::mem::size_of::<libc::vm_statistics64>() / std::mem::size_of::<libc::integer_t>()) as u32;
        let host = mach_host_self();
        let rc = libc::host_statistics64(host, libc::HOST_VM_INFO64, &mut stats as *mut _ as *mut libc::integer_t, &mut count);
        if rc != 0 {
            return crate::process::physical_memory() / 2;
        }
        let page = libc::sysconf(libc::_SC_PAGESIZE) as u64;
        (stats.free_count as u64 + stats.inactive_count as u64 + stats.purgeable_count as u64) * page
    }
}

/// Total physical memory reported to Windows guests via `GlobalMemoryStatusEx` /
/// `GetPhysicallyInstalledSystemMemory`.
///
/// Desktop PC games frequently hard-gate startup on `>= 16 GiB` physical RAM even when
/// running with mobile-trimmed assets on unified-memory iOS devices (e.g. 8–12 GB iPhone).
/// `PORT_MIN_PHYS_MEM_GB` overrides the minimum floor (defaults to `16` on iOS, `0` elsewhere).
fn reported_physical_memory() -> (u64, bool) {
    let real = crate::process::physical_memory();
    let default_min_gb: u64 = if cfg!(target_os = "ios") { 16 } else { 0 };
    let min_gb = std::env::var("PORT_MIN_PHYS_MEM_GB")
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(default_min_gb);
    let floor = min_gb.saturating_mul(1024 * 1024 * 1024);
    if floor > real {
        static LOGGED: std::sync::Once = std::sync::Once::new();
        LOGGED.call_once(|| {
            log::info!(
                "sysinfo: host physical RAM {:.2} GiB < PORT_MIN_PHYS_MEM_GB={} GiB; reporting {} GiB to satisfy desktop minimum-RAM gate",
                real as f64 / (1024.0 * 1024.0 * 1024.0),
                min_gb,
                min_gb
            );
        });
        (floor, true)
    } else {
        (real, false)
    }
}

/// Compute honest `(avail_phys, memory_load_pct)` on iOS where the process faces a
/// per-app Jetsam ceiling (`phys_footprint + proc_avail`) much tighter than the
/// `reported_total` returned to pass desktop `>= 16 GiB` startup gates.
fn compute_ios_memory_status(reported_total: u64, phys_footprint: u64, proc_avail: u64) -> (u64, u32) {
    let avail = proc_avail.min(reported_total);
    let ceiling = phys_footprint.saturating_add(proc_avail);
    let load = if ceiling > 0 {
        ((phys_footprint.saturating_mul(100)) / ceiling).min(100) as u32
    } else if reported_total > 0 {
        (100u64.saturating_sub(avail.saturating_mul(100) / reported_total)).min(100) as u32
    } else {
        0
    };
    (avail, load)
}

pub unsafe extern "win64" fn GlobalMemoryStatusEx(m: *mut MEMORYSTATUSEX) -> BOOL {
    let size = std::mem::size_of::<MEMORYSTATUSEX>();
    if m.is_null() || m.cast::<u32>().read_unaligned() as usize != size {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut status: MEMORYSTATUSEX = std::mem::zeroed();
    status.dwLength = size as u32;
    let (total, floored) = reported_physical_memory();
    if let Some(proc_avail) = crate::d3d12::mtl::available_headroom_bytes() {
        let fp = crate::d3d12::mtl::process_phys_footprint();
        let (avail, load) = compute_ios_memory_status(total, fp, proc_avail);
        status.dwMemoryLoad = load;
        status.ullTotalPhys = total;
        status.ullAvailPhys = avail;
        // iOS has no user-process swap file; report honest Jetsam headroom instead of
        // fabricating 24+ GiB of free pagefile that suppresses engine OOM eviction.
        status.ullTotalPageFile = total;
        status.ullAvailPageFile = avail;
    } else {
        let avail = if floored {
            available_physical_memory().max(total / 2).min(total)
        } else {
            available_physical_memory().min(total)
        };
        status.dwMemoryLoad = if total > 0 { (100 - avail * 100 / total) as u32 } else { 0 };
        status.ullTotalPhys = total;
        status.ullAvailPhys = avail;
        status.ullTotalPageFile = total * 2;
        status.ullAvailPageFile = avail + total;
    }
    status.ullTotalVirtual = 0x7FFF_FFFE_0000;
    status.ullAvailVirtual = 0x7FF0_0000_0000;
    status.ullAvailExtendedVirtual = 0;
    m.write_unaligned(status);
    TRUE
}

pub unsafe extern "win64" fn GetPhysicallyInstalledSystemMemory(kb: *mut u64) -> BOOL {
    let (total, _) = reported_physical_memory();
    *kb = total / 1024;
    TRUE
}

fn sysctl_u32(name: &std::ffi::CStr) -> Option<u32> {
    let mut v: u32 = 0;
    let mut sz = 4usize;
    let rc = unsafe { libc::sysctlbyname(name.as_ptr(), &mut v as *mut u32 as *mut c_void, &mut sz, std::ptr::null_mut(), 0) };
    (rc == 0).then_some(v)
}

/// (performance core count, efficiency core count). Apple Silicon reports
/// perflevel0 = P cores, perflevel1 = E cores; no SMT.
pub fn core_classes() -> (u32, u32) {
    let n = crate::process::num_cpus();
    let p = sysctl_u32(c"hw.perflevel0.logicalcpu").unwrap_or(n).min(n);
    (p, n - p)
}

fn write_group_affinity(p: *mut u8, mask: usize) {
    unsafe {
        p.cast::<usize>().write_unaligned(mask);
        p.add(8).cast::<u16>().write_unaligned(0);
    }
}

/// Builds the GetLogicalProcessorInformationEx buffer for `relation`.
fn build_lpi_ex(relation: u32) -> Vec<u8> {
    let n = crate::process::num_cpus() as usize;
    let (pcores, _) = core_classes();
    let all_mask: usize = if n >= 64 { usize::MAX } else { (1usize << n) - 1 };
    let mut out = Vec::new();
    let want = |r: u32| relation == 0xFFFF || relation == r;
    // RelationProcessorCore (0): one per logical CPU (no SMT on Apple Silicon).
    if want(0) {
        for i in 0..n {
            let mut e = vec![0u8; 48];
            e[0..4].copy_from_slice(&0u32.to_le_bytes());
            e[4..8].copy_from_slice(&48u32.to_le_bytes());
            e[8] = 0; // Flags (LTP_PC_SMT = 1)
            e[9] = if i < pcores as usize { 1 } else { 0 }; // EfficiencyClass
            e[30..32].copy_from_slice(&1u16.to_le_bytes()); // GroupCount
            write_group_affinity(e[32..].as_mut_ptr(), 1usize << i);
            out.extend(e);
        }
    }
    // RelationNumaNode (1)
    if want(1) {
        let mut e = vec![0u8; 48];
        e[0..4].copy_from_slice(&1u32.to_le_bytes());
        e[4..8].copy_from_slice(&48u32.to_le_bytes());
        e[30..32].copy_from_slice(&1u16.to_le_bytes());
        write_group_affinity(e[32..].as_mut_ptr(), all_mask);
        out.extend(e);
    }
    // RelationCache (2): L1d/L1i per core, L2 shared, (no L3 on Apple Silicon; report SLC as L3)
    if want(2) {
        let mut push_cache = |level: u8, size: u32, ty: u32, mask: usize| {
            let mut e = vec![0u8; 56];
            e[0..4].copy_from_slice(&2u32.to_le_bytes());
            e[4..8].copy_from_slice(&56u32.to_le_bytes());
            e[8] = level;
            e[9] = 8; // associativity
            e[10..12].copy_from_slice(&128u16.to_le_bytes()); // line size
            e[12..16].copy_from_slice(&size.to_le_bytes());
            e[16..20].copy_from_slice(&ty.to_le_bytes());
            e[38..40].copy_from_slice(&1u16.to_le_bytes());
            write_group_affinity(e[40..].as_mut_ptr(), mask);
            out.extend(e);
        };
        for i in 0..n {
            push_cache(1, 64 * 1024, 2, 1 << i); // CacheData
            push_cache(1, 128 * 1024, 1, 1 << i); // CacheInstruction
        }
        push_cache(2, 16 * 1024 * 1024, 0, all_mask); // CacheUnified
        push_cache(3, 32 * 1024 * 1024, 0, all_mask);
    }
    // RelationProcessorPackage (3)
    if want(3) {
        let mut e = vec![0u8; 48];
        e[0..4].copy_from_slice(&3u32.to_le_bytes());
        e[4..8].copy_from_slice(&48u32.to_le_bytes());
        e[30..32].copy_from_slice(&1u16.to_le_bytes());
        write_group_affinity(e[32..].as_mut_ptr(), all_mask);
        out.extend(e);
    }
    // RelationGroup (4)
    if want(4) {
        let mut e = vec![0u8; 80];
        e[0..4].copy_from_slice(&4u32.to_le_bytes());
        e[4..8].copy_from_slice(&80u32.to_le_bytes());
        e[8..10].copy_from_slice(&1u16.to_le_bytes());
        e[10..12].copy_from_slice(&1u16.to_le_bytes());
        e[32] = n as u8;
        e[33] = n as u8;
        e[72..80].copy_from_slice(&(all_mask as u64).to_le_bytes());
        out.extend(e);
    }
    out
}

pub unsafe extern "win64" fn GetLogicalProcessorInformationEx(relation: u32, buf: *mut u8, len: *mut u32) -> BOOL {
    if len.is_null() || !matches!(relation, 0..=4 | 0xffff) {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let data = build_lpi_ex(relation);
    if buf.is_null() || (len.read_unaligned() as usize) < data.len() {
        len.write_unaligned(data.len() as u32);
        crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(data.as_ptr(), buf, data.len());
    len.write_unaligned(data.len() as u32);
    TRUE
}

/// Legacy SYSTEM_LOGICAL_PROCESSOR_INFORMATION (32 bytes each).
pub unsafe extern "win64" fn GetLogicalProcessorInformation(buf: *mut u8, len: *mut u32) -> BOOL {
    if len.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let n = crate::process::num_cpus() as usize;
    let all: usize = if n >= 64 { usize::MAX } else { (1usize << n) - 1 };
    let mut out: Vec<u8> = Vec::new();
    let mut push = |mask: usize, rel: u32, payload: [u8; 16]| {
        out.extend(mask.to_le_bytes());
        out.extend(rel.to_le_bytes());
        out.extend([0u8; 4]);
        out.extend(payload);
    };
    for i in 0..n {
        push(1 << i, 0, [0; 16]);
    }
    push(all, 3, [0; 16]);
    push(all, 1, [0; 16]);
    let mut cache = [0u8; 16];
    cache[0] = 2;
    cache[1] = 8;
    cache[2..4].copy_from_slice(&128u16.to_le_bytes());
    cache[4..8].copy_from_slice(&(16u32 << 20).to_le_bytes());
    push(all, 2, cache);
    if buf.is_null() || (len.read_unaligned() as usize) < out.len() {
        len.write_unaligned(out.len() as u32);
        crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(out.as_ptr(), buf, out.len());
    len.write_unaligned(out.len() as u32);
    TRUE
}

pub unsafe extern "win64" fn GetActiveProcessorCount(_group: u16) -> u32 {
    crate::process::num_cpus()
}

pub unsafe extern "win64" fn GetMaximumProcessorCount(_group: u16) -> u32 {
    crate::process::num_cpus()
}

pub unsafe extern "win64" fn GetActiveProcessorGroupCount() -> u16 {
    1
}

pub unsafe extern "win64" fn GetNumaHighestNodeNumber(n: *mut u32) -> BOOL {
    *n = 0;
    TRUE
}

pub unsafe extern "win64" fn GetNumaNodeProcessorMask(node: u8, mask: *mut u64) -> BOOL {
    let n = crate::process::num_cpus();
    *mask = if node == 0 { if n >= 64 { u64::MAX } else { (1u64 << n) - 1 } } else { 0 };
    TRUE
}

pub unsafe extern "win64" fn GetNumaNodeProcessorMaskEx(node: u16, aff: *mut u8) -> BOOL {
    let n = crate::process::num_cpus();
    write_group_affinity(aff, if node == 0 { (1usize << n) - 1 } else { 0 });
    TRUE
}

pub unsafe extern "win64" fn GetNumaProcessorNode(_cpu: u8, node: *mut u8) -> BOOL {
    *node = 0;
    TRUE
}

pub unsafe extern "win64" fn GetNumaProcessorNodeEx(_proc_num: *const u8, node_num: *mut u16) -> BOOL {
    if node_num.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    *node_num = 0;
    TRUE
}

pub unsafe extern "win64" fn GetSystemCpuSetInformation(
    info: *mut u8,
    buf_len: u32,
    ret_len: *mut u32,
    _process: HANDLE,
    _flags: u32,
) -> BOOL {
    let n = crate::process::num_cpus();
    let (pcores, _) = core_classes();
    let need = n * 32;
    if !ret_len.is_null() {
        *ret_len = need;
    }
    if info.is_null() || buf_len < need {
        crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::write_bytes(info, 0, need as usize);
    for i in 0..n {
        let p = info.add((i * 32) as usize);
        (p as *mut u32).write_unaligned(32); // Size
        (p.add(4) as *mut u32).write_unaligned(0); // Type = CpuSetInformation
        (p.add(8) as *mut u32).write_unaligned(0x100 + i); // Id
        (p.add(12) as *mut u16).write_unaligned(0); // Group
        *p.add(14) = i as u8; // LogicalProcessorIndex
        *p.add(15) = i as u8; // CoreIndex
        *p.add(16) = 0; // LastLevelCacheIndex
        *p.add(17) = 0; // NumaNodeIndex
        *p.add(18) = if i < pcores { 1 } else { 0 }; // EfficiencyClass
    }
    TRUE
}

pub unsafe extern "win64" fn GetThreadSelectedCpuSets(
    _thread: HANDLE,
    _cpu_set_ids: *mut u32,
    _cpu_set_id_count: u32,
    req_id_count: *mut u32,
) -> BOOL {
    if !req_id_count.is_null() {
        *req_id_count = 0;
    }
    TRUE
}

pub unsafe extern "win64" fn SetThreadSelectedCpuSets(_thread: HANDLE, _cpu_set_ids: *const u32, _cpu_set_id_count: u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetProcessDefaultCpuSets(_process: HANDLE, _cpu_set_ids: *const u32, _cpu_set_id_count: u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetProcessDefaultCpuSets(
    _process: HANDLE,
    _cpu_set_ids: *mut u32,
    _cpu_set_id_count: u32,
    req_id_count: *mut u32,
) -> BOOL {
    if !req_id_count.is_null() {
        *req_id_count = 0;
    }
    TRUE
}

pub unsafe extern "win64" fn SetThreadSelectedCpuSetMasks(_thread: HANDLE, _masks: *const u8, _count: u16) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetThreadSelectedCpuSetMasks(
    _thread: HANDLE,
    _masks: *mut u8,
    _count: u16,
    req_count: *mut u16,
) -> BOOL {
    if !req_count.is_null() {
        *req_count = 0;
    }
    TRUE
}

pub unsafe extern "win64" fn SetProcessDefaultCpuSetMasks(_process: HANDLE, _masks: *const u8, _count: u16) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetProcessDefaultCpuSetMasks(
    _process: HANDLE,
    _masks: *mut u8,
    _count: u16,
    req_count: *mut u16,
) -> BOOL {
    if !req_count.is_null() {
        *req_count = 0;
    }
    TRUE
}

pub unsafe extern "win64" fn GetProductInfo(
    _os_major: u32,
    _os_minor: u32,
    _sp_major: u32,
    _sp_minor: u32,
    out_product_type: *mut u32,
) -> BOOL {
    if out_product_type.is_null() {
        return FALSE;
    }
    *out_product_type = 0x0000_0030; // PRODUCT_PROFESSIONAL
    TRUE
}

pub unsafe extern "win64" fn GetEnabledXStateFeatures() -> u64 {
    0x7 // XSTATE_MASK_LEGACY (FPU + SSE) | XSTATE_MASK_AVX (matches CPUID/XCR0)
}

pub unsafe extern "win64" fn GetXStateFeaturesMask(_context: *const u8, feature_mask: *mut u64) -> BOOL {
    if !feature_mask.is_null() {
        *feature_mask = 0x7;
    }
    TRUE
}

pub unsafe extern "win64" fn GetLargePageMinimum() -> usize {
    0
}

pub unsafe extern "win64" fn GetComputerNameW(buf: *mut u16, size: *mut u32) -> BOOL {
    let name = crate::config::app_config().computer_name;
    let (ok, n) = write_wide_buf(&name, buf, *size as usize);
    *size = n as u32;
    if !ok {
        crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn GetComputerNameA(buf: *mut u8, size: *mut u32) -> BOOL {
    let name = crate::config::app_config().computer_name;
    let (ok, n) = write_ansi_buf(&name, buf, *size as usize);
    *size = n as u32;
    if !ok {
        crate::teb::set_last_error(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn GetComputerNameExW(_fmt: u32, buf: *mut u16, size: *mut u32) -> BOOL {
    GetComputerNameW(buf, size)
}

fn sysdir_result_w(s: &str, buf: *mut u16, size: u32) -> u32 {
    let (ok, n) = unsafe { write_wide_buf(s, buf, size as usize) };
    if ok {
        n as u32
    } else {
        n as u32
    }
}

fn sysdir_result_a(s: &str, buf: *mut u8, size: u32) -> u32 {
    let (_, n) = unsafe { write_ansi_buf(s, buf, size as usize) };
    n as u32
}

pub(crate) const SYSTEM_DIR: &str = "C:\\Windows\\system32";
#[allow(dead_code)]
pub(crate) const SYSTEM16_DIR: &str = "C:\\Windows\\System";
pub(crate) const WINDOWS_DIR: &str = "C:\\Windows";

pub unsafe extern "win64" fn GetSystemDirectoryW(buf: *mut u16, size: u32) -> u32 {
    sysdir_result_w(SYSTEM_DIR, buf, size)
}
pub unsafe extern "win64" fn GetSystemDirectoryA(buf: *mut u8, size: u32) -> u32 {
    sysdir_result_a(SYSTEM_DIR, buf, size)
}
pub unsafe extern "win64" fn GetWindowsDirectoryW(buf: *mut u16, size: u32) -> u32 {
    sysdir_result_w(WINDOWS_DIR, buf, size)
}
pub unsafe extern "win64" fn GetWindowsDirectoryA(buf: *mut u8, size: u32) -> u32 {
    sysdir_result_a(WINDOWS_DIR, buf, size)
}
pub unsafe extern "win64" fn GetSystemWindowsDirectoryW(buf: *mut u16, size: u32) -> u32 {
    sysdir_result_w(WINDOWS_DIR, buf, size)
}
pub unsafe extern "win64" fn GetSystemWindowsDirectoryA(buf: *mut u8, size: u32) -> u32 {
    sysdir_result_a(WINDOWS_DIR, buf, size)
}
pub unsafe extern "win64" fn GetSystemWow64DirectoryW(buf: *mut u16, size: u32) -> u32 {
    sysdir_result_w("C:\\Windows\\SysWOW64", buf, size)
}
pub unsafe extern "win64" fn GetSystemWow64DirectoryA(buf: *mut u8, size: u32) -> u32 {
    sysdir_result_a("C:\\Windows\\SysWOW64", buf, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ios_memory_status_reports_honest_jetsam_headroom_and_load() {
        let total = 16u64 << 30;
        // 3 GiB used, 3 GiB remaining before 6 GiB Jetsam -> 50% load, 3 GiB avail
        let (avail, load) = compute_ios_memory_status(total, 3 << 30, 3 << 30);
        assert_eq!(avail, 3 << 30);
        assert_eq!(load, 50);

        // 5 GiB used, 1 GiB remaining before 6 GiB Jetsam -> 83% load, 1 GiB avail (not fake 8 GiB!)
        let (avail, load) = compute_ios_memory_status(total, 5 << 30, 1 << 30);
        assert_eq!(avail, 1 << 30);
        assert_eq!(load, 83);
    }
}
