//! pdh.dll: Performance Data Helper (PDH) queries backed by real macOS host
//! CPU, memory, and process telemetry.
//!
//! Recognizes standard Windows performance counter paths (`\Processor(*)\...`,
//! `\Memory\...`, `\Process(*)\...`, `\System\...`) and returns documented
//! `PDH_CSTATUS_NO_OBJECT` (`0xC0000BB8`) honestly for unsupported counter objects.

use crate::strings::from_wide;
use crate::types::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

pub const ERROR_SUCCESS: i32 = 0;
pub const PDH_CSTATUS_VALID_DATA: u32 = 0x0000_0000;
pub const PDH_CSTATUS_NO_MACHINE: i32 = 0x8000_07D0u32 as i32;
pub const PDH_CSTATUS_NO_INSTANCE: i32 = 0x8000_07D1u32 as i32;
pub const PDH_CSTATUS_NO_OBJECT: i32 = 0xC000_0BB8u32 as i32;
pub const PDH_CSTATUS_NO_COUNTER: i32 = 0xC000_0BB9u32 as i32;
pub const PDH_INVALID_HANDLE: i32 = 0xC000_0BBCu32 as i32;
pub const PDH_INVALID_ARGUMENT: i32 = 0xC000_0BBDu32 as i32;
pub const PDH_NOT_IMPLEMENTED: i32 = 0xC000_0BD3u32 as i32;
pub const PDH_NO_DATA: i32 = 0x8000_07D5u32 as i32;

pub const PDH_FMT_LONG: u32 = 0x0000_0100;
pub const PDH_FMT_DOUBLE: u32 = 0x0000_0200;
pub const PDH_FMT_LARGE: u32 = 0x0000_0400;
pub const PDH_FMT_NOSCALE: u32 = 0x0000_1000;
pub const PDH_FMT_1000: u32 = 0x0000_2000;
pub const PDH_FMT_NOCAP100: u32 = 0x0000_8000;
const PDH_FMT_TYPE_MASK: u32 = 0x0000_0F00;
const PDH_FMT_SUPPORTED_MODS: u32 = PDH_FMT_NOSCALE | PDH_FMT_1000 | PDH_FMT_NOCAP100;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProcessorTarget {
    Total,
    Core(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CounterKind {
    ProcessorTotalPct(ProcessorTarget),
    ProcessorUserPct(ProcessorTarget),
    ProcessorKernelPct(ProcessorTarget),
    ProcessorIdlePct(ProcessorTarget),
    MemoryAvailBytes,
    MemoryAvailKBytes,
    MemoryAvailMBytes,
    ProcessWorkingSet,
    ProcessWorkingSetPrivate,
    ProcessPrivateBytes,
    ProcessVirtualBytes,
    ProcessProcessorPct,
    SystemUptimeSec,
}

impl CounterKind {
    fn is_percentage(self) -> bool {
        matches!(
            self,
            Self::ProcessorTotalPct(_)
                | Self::ProcessorUserPct(_)
                | Self::ProcessorKernelPct(_)
                | Self::ProcessorIdlePct(_)
                | Self::ProcessProcessorPct
        )
    }
}

struct CounterState {
    query: usize,
    kind: CounterKind,
    last_value: Option<f64>,
    prev_sample_time: Option<Instant>,
    prev_cpu_sec: f64,
    prev_host_ticks: Option<[u32; 4]>,
}

struct QueryState {
    counters: Vec<usize>,
}

struct PdhState {
    queries: HashMap<usize, QueryState>,
    counters: HashMap<usize, CounterState>,
}

fn state() -> &'static Mutex<PdhState> {
    static S: OnceLock<Mutex<PdhState>> = OnceLock::new();
    S.get_or_init(|| {
        Mutex::new(PdhState {
            queries: HashMap::new(),
            counters: HashMap::new(),
        })
    })
}

static NEXT_HANDLE: AtomicUsize = AtomicUsize::new(0x5A00_0001);

fn parse_processor_instance(instance: Option<&str>) -> Result<ProcessorTarget, i32> {
    let raw = instance.map(str::trim).unwrap_or("");
    if raw.is_empty() {
        return Err(PDH_CSTATUS_NO_INSTANCE);
    }
    let lower = raw.to_ascii_lowercase();
    if lower == "_total" || lower.ends_with(",_total") {
        return Ok(ProcessorTarget::Total);
    }
    // Support both "\Processor(0)" and "\Processor Information(0,0)" formats.
    let idx_part = match lower.split_once(',') {
        Some(("0", core_str)) => core_str.trim(),
        Some(_) => return Err(PDH_CSTATUS_NO_INSTANCE),
        None => lower.as_str(),
    };
    let core = idx_part.parse::<usize>().map_err(|_| PDH_CSTATUS_NO_INSTANCE)?;
    let ncpus = crate::process::num_cpus().max(1) as usize;
    if core >= ncpus {
        return Err(PDH_CSTATUS_NO_INSTANCE);
    }
    Ok(ProcessorTarget::Core(core))
}

/// Returns the Windows PDH `\Process(<name>)` instance stem for the current process:
/// the guest `.exe` file stem when running a Windows PE via `ProcessConfig`, or the
/// host executable file stem otherwise (matching Windows PDH's extension-stripped name).
fn current_process_instance_name() -> Option<String> {
    if let Some(cfg) = crate::process::try_config() {
        let file = cfg
            .exe_path
            .rsplit(['\\', '/'])
            .next()
            .filter(|s| !s.is_empty())?;
        let stem = file
            .strip_suffix(".exe")
            .or_else(|| file.strip_suffix(".EXE"))
            .unwrap_or_else(|| {
                if let Some(dot) = file.rfind('.') {
                    if file[dot..].eq_ignore_ascii_case(".exe") {
                        &file[..dot]
                    } else {
                        file
                    }
                } else {
                    file
                }
            });
        return (!stem.is_empty()).then(|| stem.to_string());
    }
    let exe = std::env::current_exe().ok()?;
    let stem = exe.file_stem()?.to_string_lossy().into_owned();
    (!stem.is_empty()).then_some(stem)
}

/// Validates that a `\Process(<instance>)\...` counter targets the current process.
/// Rejects empty instances, `_Total`, other process names, and non-zero `#N` indices
/// honestly with `PDH_CSTATUS_NO_INSTANCE` rather than substituting the current process.
fn validate_process_instance(instance: Option<&str>) -> Result<(), i32> {
    let raw = instance.map(str::trim).unwrap_or("");
    if raw.is_empty() {
        return Err(PDH_CSTATUS_NO_INSTANCE);
    }
    let base = match raw.split_once('#') {
        Some((stem, idx)) => {
            if idx.trim() != "0" {
                return Err(PDH_CSTATUS_NO_INSTANCE);
            }
            stem.trim()
        }
        None => raw,
    };
    let Some(cur) = current_process_instance_name() else {
        return Err(PDH_CSTATUS_NO_INSTANCE);
    };
    if base.eq_ignore_ascii_case(&cur) {
        Ok(())
    } else {
        Err(PDH_CSTATUS_NO_INSTANCE)
    }
}

/// Returns true only when `machine` identifies the local host (`"."`, `"localhost"`,
/// loopback literal, the configured Windows `computer_name`, or the host's `gethostname`).
/// Remote machine names are rejected with `PDH_CSTATUS_NO_MACHINE` rather than silently
/// returning local telemetry.
fn is_local_machine(machine: &str) -> bool {
    let m = machine.trim();
    if m.is_empty() {
        return false;
    }
    if m == "."
        || m.eq_ignore_ascii_case("localhost")
        || m == "127.0.0.1"
        || m == "::1"
        || m.eq_ignore_ascii_case(&crate::config::app_config().computer_name)
    {
        return true;
    }
    let mut buf = [0u8; 256];
    if unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) } == 0 {
        let nul = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        if let Ok(host) = std::str::from_utf8(&buf[..nul]) {
            let host = host.trim();
            if !host.is_empty() {
                if m.eq_ignore_ascii_case(host) {
                    return true;
                }
                if let Some((short, _)) = host.split_once('.') {
                    if !short.is_empty() && m.eq_ignore_ascii_case(short) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn parse_counter_path(path: &str) -> Result<CounterKind, i32> {
    let trimmed = path.trim();
    // Validate optional \\machine prefix (must denote the local host) and strip leading '\'.
    let rest = if let Some(after_unc) = trimmed.strip_prefix("\\\\") {
        let Some((machine, tail)) = after_unc.split_once('\\') else {
            return Err(PDH_CSTATUS_NO_OBJECT);
        };
        if !is_local_machine(machine) {
            return Err(PDH_CSTATUS_NO_MACHINE);
        }
        tail
    } else if let Some(after_slash) = trimmed.strip_prefix('\\') {
        after_slash
    } else {
        return Err(PDH_CSTATUS_NO_OBJECT);
    };
    let (obj_part, counter_part) = match rest.split_once('\\') {
        Some((o, c)) => (o.trim(), c.trim()),
        None => return Err(PDH_CSTATUS_NO_COUNTER),
    };
    let (obj_raw, instance) = match obj_part.split_once('(') {
        Some((name, tail)) => match tail.strip_suffix(')') {
            Some(inst) => (name, Some(inst)),
            None => return Err(PDH_CSTATUS_NO_OBJECT),
        },
        None => (obj_part, None),
    };
    let obj_name = obj_raw.trim().to_ascii_lowercase();
    let ctr_name = counter_part.to_ascii_lowercase();
    match obj_name.as_str() {
        "processor" | "processor information" => {
            let target = parse_processor_instance(instance)?;
            match ctr_name.as_str() {
                "% processor time" | "% processor utility" => Ok(CounterKind::ProcessorTotalPct(target)),
                "% user time" => Ok(CounterKind::ProcessorUserPct(target)),
                "% privileged time" => Ok(CounterKind::ProcessorKernelPct(target)),
                "% idle time" => Ok(CounterKind::ProcessorIdlePct(target)),
                _ => Err(PDH_CSTATUS_NO_COUNTER),
            }
        }
        "memory" => match ctr_name.as_str() {
            "available bytes" => Ok(CounterKind::MemoryAvailBytes),
            "available kbytes" => Ok(CounterKind::MemoryAvailKBytes),
            "available mbytes" => Ok(CounterKind::MemoryAvailMBytes),
            // Darwin/XNU uses overcommit + dynamically resized swap files without a fixed
            // Windows-style system commit limit, so "% Committed Bytes In Use" cannot be
            // honestly mapped to physical RAM load and is rejected with PDH_CSTATUS_NO_COUNTER.
            _ => Err(PDH_CSTATUS_NO_COUNTER),
        },
        "process" => {
            validate_process_instance(instance)?;
            match ctr_name.as_str() {
                "working set" => Ok(CounterKind::ProcessWorkingSet),
                "working set - private" => Ok(CounterKind::ProcessWorkingSetPrivate),
                "private bytes" => Ok(CounterKind::ProcessPrivateBytes),
                "virtual bytes" => Ok(CounterKind::ProcessVirtualBytes),
                "% processor time" => Ok(CounterKind::ProcessProcessorPct),
                _ => Err(PDH_CSTATUS_NO_COUNTER),
            }
        }
        "system" => match ctr_name.as_str() {
            "system up time" => Ok(CounterKind::SystemUptimeSec),
            _ => Err(PDH_CSTATUS_NO_COUNTER),
        },
        _ => Err(PDH_CSTATUS_NO_OBJECT),
    }
}

/// Samples real host-wide (or per-core) CPU load ticks `[user, system, idle, nice]` from Mach.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn host_cpu_ticks(target: ProcessorTarget) -> Option<[u32; 4]> {
    const HOST_CPU_LOAD_INFO: i32 = 3;
    const PROCESSOR_CPU_LOAD_INFO: i32 = 2;
    extern "C" {
        fn mach_host_self() -> u32;
        static mach_task_self_: u32;
        fn host_statistics(
            host_priv: u32,
            flavor: i32,
            host_info_out: *mut i32,
            host_info_out_cnt: *mut u32,
        ) -> i32;
        fn host_processor_info(
            host: u32,
            flavor: i32,
            out_processor_count: *mut u32,
            out_processor_info: *mut *mut i32,
            out_processor_info_cnt: *mut u32,
        ) -> i32;
        fn mach_vm_deallocate(task: u32, addr: u64, size: u64) -> i32;
    }
    unsafe {
        match target {
            ProcessorTarget::Total => {
                let mut ticks = [0u32; 4];
                let mut count: u32 = 4;
                let kr = host_statistics(
                    mach_host_self(),
                    HOST_CPU_LOAD_INFO,
                    ticks.as_mut_ptr() as *mut i32,
                    &mut count,
                );
                if kr == 0 && count >= 4 {
                    Some(ticks)
                } else {
                    None
                }
            }
            ProcessorTarget::Core(idx) => {
                let mut proc_cnt: u32 = 0;
                let mut info_ptr: *mut i32 = std::ptr::null_mut();
                let mut info_cnt: u32 = 0;
                let kr = host_processor_info(
                    mach_host_self(),
                    PROCESSOR_CPU_LOAD_INFO,
                    &mut proc_cnt,
                    &mut info_ptr,
                    &mut info_cnt,
                );
                if kr != 0 || info_ptr.is_null() {
                    return None;
                }
                let result = if idx < proc_cnt as usize && (idx + 1) * 4 <= info_cnt as usize {
                    let base = info_ptr.add(idx * 4);
                    Some([
                        std::ptr::read_unaligned(base) as u32,
                        std::ptr::read_unaligned(base.add(1)) as u32,
                        std::ptr::read_unaligned(base.add(2)) as u32,
                        std::ptr::read_unaligned(base.add(3)) as u32,
                    ])
                } else {
                    None
                };
                let bytes = (info_cnt as u64) * 4;
                let _ = mach_vm_deallocate(mach_task_self_, info_ptr as u64, bytes);
                result
            }
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn host_cpu_ticks(_target: ProcessorTarget) -> Option<[u32; 4]> {
    None
}

fn process_cpu_seconds() -> (f64, f64) {
    unsafe {
        let mut ru: libc::rusage = std::mem::zeroed();
        if libc::getrusage(libc::RUSAGE_SELF, &mut ru) == 0 {
            let user = ru.ru_utime.tv_sec as f64 + (ru.ru_utime.tv_usec as f64) * 1e-6;
            let sys = ru.ru_stime.tv_sec as f64 + (ru.ru_stime.tv_usec as f64) * 1e-6;
            (user, sys)
        } else {
            (0.0, 0.0)
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ProcessVmMetrics {
    virtual_size: u64,
    resident_size: u64,
    internal: u64,
    phys_footprint: u64,
}

/// Samples live current process memory metrics from Mach `TASK_VM_INFO`
/// (not lifetime peak `ru_maxrss`):
/// - `resident_size`: current working set
/// - `internal`: current private anonymous resident pages ("Working Set - Private")
/// - `phys_footprint`: current private committed/dirty+compressed footprint ("Private Bytes")
/// - `virtual_size`: current virtual address space size ("Virtual Bytes")
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn process_vm_memory_metrics() -> Option<ProcessVmMetrics> {
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
    extern "C" {
        static mach_task_self_: u32;
        fn task_info(
            target_task: u32,
            flavor: i32,
            task_info_out: *mut u32,
            task_info_out_cnt: *mut u32,
        ) -> i32;
    }
    unsafe {
        let mut info: TaskVmInfoPrefix = std::mem::zeroed();
        let mut count = (std::mem::size_of::<TaskVmInfoPrefix>() / std::mem::size_of::<u32>()) as u32;
        let rc = task_info(
            mach_task_self_,
            TASK_VM_INFO,
            &mut info as *mut _ as *mut u32,
            &mut count,
        );
        let min_bytes = std::mem::offset_of!(TaskVmInfoPrefix, phys_footprint) + 8;
        if rc == 0 && (count as usize) * 4 >= min_bytes {
            Some(ProcessVmMetrics {
                virtual_size: info.virtual_size,
                resident_size: info.resident_size,
                internal: info.internal,
                phys_footprint: info.phys_footprint,
            })
        } else {
            None
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn process_vm_memory_metrics() -> Option<ProcessVmMetrics> {
    None
}

fn sample_counter(
    c: &mut CounterState,
    now: Instant,
    cached_total_ticks: &mut Option<Option<[u32; 4]>>,
    cached_vm_metrics: &mut Option<Option<ProcessVmMetrics>>,
) {
    let val = match c.kind {
        CounterKind::ProcessorTotalPct(target)
        | CounterKind::ProcessorUserPct(target)
        | CounterKind::ProcessorKernelPct(target)
        | CounterKind::ProcessorIdlePct(target) => {
            let cur_opt = match target {
                ProcessorTarget::Total => *cached_total_ticks.get_or_insert_with(|| host_cpu_ticks(ProcessorTarget::Total)),
                ProcessorTarget::Core(idx) => host_cpu_ticks(ProcessorTarget::Core(idx)),
            };
            let Some(cur) = cur_opt else {
                c.last_value = None;
                return;
            };
            let pct = if let Some(prev) = c.prev_host_ticks {
                let du = cur[0].wrapping_sub(prev[0]) as f64;
                let ds = cur[1].wrapping_sub(prev[1]) as f64;
                let di = cur[2].wrapping_sub(prev[2]) as f64;
                let dn = cur[3].wrapping_sub(prev[3]) as f64;
                let total = du + ds + di + dn;
                if total > 0.0 {
                    let raw = match c.kind {
                        CounterKind::ProcessorTotalPct(_) => ((du + ds + dn) / total) * 100.0,
                        CounterKind::ProcessorUserPct(_) => ((du + dn) / total) * 100.0,
                        CounterKind::ProcessorKernelPct(_) => (ds / total) * 100.0,
                        CounterKind::ProcessorIdlePct(_) => (di / total) * 100.0,
                        _ => 0.0,
                    };
                    raw.clamp(0.0, 100.0)
                } else {
                    c.last_value.unwrap_or(0.0)
                }
            } else {
                0.0
            };
            c.prev_host_ticks = Some(cur);
            c.prev_sample_time = Some(now);
            pct
        }
        CounterKind::ProcessProcessorPct => {
            let (u, s) = process_cpu_seconds();
            let cur_cpu = u + s;
            let pct = if let Some(prev_t) = c.prev_sample_time {
                let dt = now.saturating_duration_since(prev_t).as_secs_f64();
                if dt > 1e-6 {
                    let cpus = crate::process::num_cpus().max(1) as f64;
                    let delta = (cur_cpu - c.prev_cpu_sec).max(0.0);
                    ((delta / dt) * 100.0).clamp(0.0, 100.0 * cpus)
                } else {
                    c.last_value.unwrap_or(0.0)
                }
            } else {
                0.0
            };
            c.prev_sample_time = Some(now);
            c.prev_cpu_sec = cur_cpu;
            pct
        }
        CounterKind::MemoryAvailBytes
        | CounterKind::MemoryAvailKBytes
        | CounterKind::MemoryAvailMBytes => {
            let mut ms = crate::kernel32::sysinfo::MEMORYSTATUSEX {
                dwLength: std::mem::size_of::<crate::kernel32::sysinfo::MEMORYSTATUSEX>() as u32,
                ..unsafe { std::mem::zeroed() }
            };
            unsafe {
                crate::kernel32::sysinfo::GlobalMemoryStatusEx(&mut ms);
                let avail_phys = ms.ullAvailPhys;
                match c.kind {
                    CounterKind::MemoryAvailBytes => avail_phys as f64,
                    CounterKind::MemoryAvailKBytes => (avail_phys / 1024) as f64,
                    CounterKind::MemoryAvailMBytes => (avail_phys / (1024 * 1024)) as f64,
                    _ => 0.0,
                }
            }
        }
        CounterKind::ProcessWorkingSet
        | CounterKind::ProcessWorkingSetPrivate
        | CounterKind::ProcessPrivateBytes
        | CounterKind::ProcessVirtualBytes => {
            let Some(vm) = *cached_vm_metrics.get_or_insert_with(process_vm_memory_metrics) else {
                c.last_value = None;
                return;
            };
            match c.kind {
                CounterKind::ProcessWorkingSet => vm.resident_size as f64,
                CounterKind::ProcessWorkingSetPrivate => vm.internal as f64,
                CounterKind::ProcessPrivateBytes => vm.phys_footprint as f64,
                CounterKind::ProcessVirtualBytes => vm.virtual_size as f64,
                _ => 0.0,
            }
        }
        CounterKind::SystemUptimeSec => {
            (unsafe { crate::kernel32::time::GetTickCount64() } as f64) / 1000.0
        }
    };
    c.last_value = Some(val);
}

pub unsafe extern "win64" fn PdhOpenQueryW(
    data_source: LPCWSTR,
    _user_data: usize,
    ph_query: *mut usize,
) -> i32 {
    if ph_query.is_null() {
        return PDH_INVALID_ARGUMENT;
    }
    if !data_source.is_null() {
        let src = from_wide(data_source);
        std::ptr::write_unaligned(ph_query, 0);
        if src.is_empty() {
            return PDH_INVALID_ARGUMENT;
        }
        log::warn!("pdh: PdhOpenQueryW log-file data source unsupported: {src:?}");
        return PDH_NOT_IMPLEMENTED;
    }
    let q = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    state().lock().unwrap().queries.insert(q, QueryState { counters: Vec::new() });
    std::ptr::write_unaligned(ph_query, q);
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn PdhAddCounterW(
    h_query: usize,
    full_counter_path: LPCWSTR,
    _user_data: usize,
    ph_counter: *mut usize,
) -> i32 {
    if ph_counter.is_null() || full_counter_path.is_null() {
        return PDH_INVALID_ARGUMENT;
    }
    let path = from_wide(full_counter_path);
    let kind = match parse_counter_path(&path) {
        Ok(k) => k,
        Err(code) => {
            std::ptr::write_unaligned(ph_counter, 0);
            return code;
        }
    };
    let mut st = state().lock().unwrap();
    if !st.queries.contains_key(&h_query) {
        std::ptr::write_unaligned(ph_counter, 0);
        return PDH_INVALID_HANDLE;
    }
    let ch = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    st.queries.get_mut(&h_query).unwrap().counters.push(ch);
    st.counters.insert(
        ch,
        CounterState {
            query: h_query,
            kind,
            last_value: None,
            prev_sample_time: None,
            prev_cpu_sec: 0.0,
            prev_host_ticks: None,
        },
    );
    std::ptr::write_unaligned(ph_counter, ch);
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn PdhCollectQueryData(h_query: usize) -> i32 {
    let mut st = state().lock().unwrap();
    let Some(counter_ids) = st.queries.get(&h_query).map(|q| q.counters.clone()) else {
        return PDH_INVALID_HANDLE;
    };
    let now = Instant::now();
    let mut cached_total_ticks: Option<Option<[u32; 4]>> = None;
    let mut cached_vm_metrics: Option<Option<ProcessVmMetrics>> = None;
    for cid in counter_ids {
        if let Some(c) = st.counters.get_mut(&cid) {
            sample_counter(c, now, &mut cached_total_ticks, &mut cached_vm_metrics);
        }
    }
    ERROR_SUCCESS
}

/// Apply PDH formatting modifiers (`PDH_FMT_1000`, `PDH_FMT_NOSCALE`, `PDH_FMT_NOCAP100`)
/// and validate `dw_format` before formatting a sampled counter value.
fn apply_pdh_format(kind: CounterKind, raw_val: f64, dw_format: u32) -> Result<(u32, f64), i32> {
    let fmt_type = dw_format & PDH_FMT_TYPE_MASK;
    if !matches!(fmt_type, PDH_FMT_LONG | PDH_FMT_DOUBLE | PDH_FMT_LARGE) {
        return Err(PDH_INVALID_ARGUMENT);
    }
    let mods = dw_format & !PDH_FMT_TYPE_MASK;
    if (mods & !PDH_FMT_SUPPORTED_MODS) != 0 {
        return Err(PDH_INVALID_ARGUMENT);
    }
    let mut out = raw_val;
    if kind.is_percentage() && (mods & PDH_FMT_NOCAP100) == 0 {
        out = out.clamp(0.0, 100.0);
    }
    if (mods & PDH_FMT_1000) != 0 {
        out *= 1000.0;
    }
    Ok((fmt_type, out))
}

/// `PDH_FMT_COUNTERVALUE` (x64, 16 bytes): `CStatus: u32 @ 0`, pad @ 4, union (`i32`/`f64`/`i64`) @ 8.
pub unsafe extern "win64" fn PdhGetFormattedCounterValue(
    h_counter: usize,
    dw_format: u32,
    lpdw_type: *mut u32,
    p_value: *mut u8,
) -> i32 {
    if p_value.is_null() {
        return PDH_INVALID_ARGUMENT;
    }
    // Validate requested data type and modifiers before mutating output or claiming valid data.
    let fmt_type_check = dw_format & PDH_FMT_TYPE_MASK;
    let mods_check = dw_format & !PDH_FMT_TYPE_MASK;
    if !matches!(fmt_type_check, PDH_FMT_LONG | PDH_FMT_DOUBLE | PDH_FMT_LARGE)
        || (mods_check & !PDH_FMT_SUPPORTED_MODS) != 0
    {
        return PDH_INVALID_ARGUMENT;
    }
    let st = state().lock().unwrap();
    let Some(c) = st.counters.get(&h_counter) else {
        return PDH_INVALID_HANDLE;
    };
    let _ = c.query;
    let Some(val) = c.last_value else {
        std::ptr::write_unaligned(p_value as *mut u32, PDH_NO_DATA as u32);
        return PDH_NO_DATA;
    };
    let (fmt_type, scaled) = match apply_pdh_format(c.kind, val, dw_format) {
        Ok(x) => x,
        Err(code) => return code,
    };
    if !lpdw_type.is_null() {
        std::ptr::write_unaligned(lpdw_type, 0u32);
    }
    std::ptr::write_bytes(p_value, 0, 16);
    std::ptr::write_unaligned(p_value as *mut u32, PDH_CSTATUS_VALID_DATA);
    match fmt_type {
        PDH_FMT_LONG => std::ptr::write_unaligned(p_value.add(8) as *mut i32, scaled as i32),
        PDH_FMT_LARGE => std::ptr::write_unaligned(p_value.add(8) as *mut i64, scaled as i64),
        PDH_FMT_DOUBLE => std::ptr::write_unaligned(p_value.add(8) as *mut f64, scaled),
        _ => return PDH_INVALID_ARGUMENT,
    }
    ERROR_SUCCESS
}

pub unsafe extern "win64" fn PdhCloseQuery(h_query: usize) -> i32 {
    let mut st = state().lock().unwrap();
    let Some(q) = st.queries.remove(&h_query) else {
        return PDH_INVALID_HANDLE;
    };
    for cid in q.counters {
        st.counters.remove(&cid);
    }
    ERROR_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strings::to_wide_z;

    #[test]
    fn pdh_query_lifecycle_real_metrics_and_honest_unknown_counter() {
        unsafe {
            // Non-null log-file data sources must be rejected honestly rather than silently
            // returning live telemetry.
            let log_src = to_wide_z("C:\\perf_capture.blg");
            let mut q_log = 0xDEADusize;
            assert_eq!(PdhOpenQueryW(log_src.as_ptr(), 0, &mut q_log), PDH_NOT_IMPLEMENTED);
            assert_eq!(q_log, 0);

            let empty_src = to_wide_z("");
            let mut q_empty = 0xDEADusize;
            assert_eq!(PdhOpenQueryW(empty_src.as_ptr(), 0, &mut q_empty), PDH_INVALID_ARGUMENT);
            assert_eq!(q_empty, 0);

            let mut q = 0usize;
            assert_eq!(PdhOpenQueryW(std::ptr::null(), 0, &mut q), ERROR_SUCCESS);
            assert_ne!(q, 0);

            let mem_path = to_wide_z("\\Memory\\Available MBytes");
            let mut c_mem = 0usize;
            assert_eq!(PdhAddCounterW(q, mem_path.as_ptr(), 0, &mut c_mem), ERROR_SUCCESS);
            assert_ne!(c_mem, 0);

            let cpu_path = to_wide_z("\\Processor(_Total)\\% Processor Time");
            let mut c_cpu = 0usize;
            assert_eq!(PdhAddCounterW(q, cpu_path.as_ptr(), 0, &mut c_cpu), ERROR_SUCCESS);

            let user_path = to_wide_z("\\Processor(_Total)\\% User Time");
            let mut c_user = 0usize;
            assert_eq!(PdhAddCounterW(q, user_path.as_ptr(), 0, &mut c_user), ERROR_SUCCESS);

            let kern_path = to_wide_z("\\Processor(_Total)\\% Privileged Time");
            let mut c_kern = 0usize;
            assert_eq!(PdhAddCounterW(q, kern_path.as_ptr(), 0, &mut c_kern), ERROR_SUCCESS);

            let core0_path = to_wide_z("\\Processor(0)\\% Processor Time");
            let mut c_core0 = 0usize;
            assert_eq!(PdhAddCounterW(q, core0_path.as_ptr(), 0, &mut c_core0), ERROR_SUCCESS);

            let cur_proc = current_process_instance_name().expect("current process instance name");
            let proc_path = to_wide_z(&format!("\\Process({cur_proc})\\% Processor Time"));
            let mut c_proc = 0usize;
            assert_eq!(PdhAddCounterW(q, proc_path.as_ptr(), 0, &mut c_proc), ERROR_SUCCESS);

            let ws_path = to_wide_z(&format!("\\Process({cur_proc}#0)\\Working Set"));
            let mut c_ws = 0usize;
            assert_eq!(PdhAddCounterW(q, ws_path.as_ptr(), 0, &mut c_ws), ERROR_SUCCESS);

            let ws_priv_path = to_wide_z(&format!("\\Process({})\\Working Set - Private", cur_proc.to_ascii_uppercase()));
            let mut c_ws_priv = 0usize;
            assert_eq!(PdhAddCounterW(q, ws_priv_path.as_ptr(), 0, &mut c_ws_priv), ERROR_SUCCESS);

            let priv_path = to_wide_z(&format!("\\Process({cur_proc})\\Private Bytes"));
            let mut c_priv = 0usize;
            assert_eq!(PdhAddCounterW(q, priv_path.as_ptr(), 0, &mut c_priv), ERROR_SUCCESS);

            let virt_path = to_wide_z(&format!("\\Process({cur_proc})\\Virtual Bytes"));
            let mut c_virt = 0usize;
            assert_eq!(PdhAddCounterW(q, virt_path.as_ptr(), 0, &mut c_virt), ERROR_SUCCESS);

            // Other/nonexistent/aggregate Process instances must fail honestly with PDH_CSTATUS_NO_INSTANCE.
            for bad_proc in [
                "\\Process(other-process)\\Working Set",
                "\\Process(_Total)\\Working Set",
                "\\Process\\Working Set",
                &format!("\\Process({cur_proc}#1)\\Working Set"),
            ] {
                let w = to_wide_z(bad_proc);
                let mut ch = 1usize;
                assert_eq!(
                    PdhAddCounterW(q, w.as_ptr(), 0, &mut ch),
                    PDH_CSTATUS_NO_INSTANCE,
                    "expected PDH_CSTATUS_NO_INSTANCE for {bad_proc}"
                );
                assert_eq!(ch, 0);
            }

            // Out-of-range or missing processor instance fails honestly with PDH_CSTATUS_NO_INSTANCE.
            let bad_inst = to_wide_z("\\Processor(99999)\\% Processor Time");
            let mut c_bad_inst = 1usize;
            assert_eq!(PdhAddCounterW(q, bad_inst.as_ptr(), 0, &mut c_bad_inst), PDH_CSTATUS_NO_INSTANCE);
            assert_eq!(c_bad_inst, 0);

            // Local UNC machine prefixes (., localhost, configured Windows computer name) succeed,
            // while remote or empty machine names fail honestly with PDH_CSTATUS_NO_MACHINE.
            let local_comp = crate::config::app_config().computer_name;
            for local_path in [
                "\\\\.\\Memory\\Available MBytes".to_string(),
                "\\\\localhost\\Memory\\Available MBytes".to_string(),
                format!("\\\\{local_comp}\\Memory\\Available MBytes"),
            ] {
                let w = to_wide_z(&local_path);
                let mut ch = 0usize;
                assert_eq!(
                    PdhAddCounterW(q, w.as_ptr(), 0, &mut ch),
                    ERROR_SUCCESS,
                    "expected local machine path {local_path} to succeed"
                );
                assert_ne!(ch, 0);
            }
            for remote_path in [
                "\\\\other-host\\Processor(_Total)\\% Processor Time",
                "\\\\192.0.2.1\\Memory\\Available MBytes",
                "\\\\\\Memory\\Available MBytes",
            ] {
                let w = to_wide_z(remote_path);
                let mut ch = 1usize;
                assert_eq!(
                    PdhAddCounterW(q, w.as_ptr(), 0, &mut ch),
                    PDH_CSTATUS_NO_MACHINE,
                    "expected PDH_CSTATUS_NO_MACHINE for {remote_path}"
                );
                assert_eq!(ch, 0);
            }

            // Unsupported Memory counter (% Committed Bytes In Use has no Darwin fixed commit limit)
            // must fail honestly with PDH_CSTATUS_NO_COUNTER rather than substituting physical RAM load.
            let commit_path = to_wide_z("\\Memory\\% Committed Bytes In Use");
            let mut c_commit = 1usize;
            assert_eq!(PdhAddCounterW(q, commit_path.as_ptr(), 0, &mut c_commit), PDH_CSTATUS_NO_COUNTER);
            assert_eq!(c_commit, 0);

            // Unknown counter object fails honestly with PDH_CSTATUS_NO_OBJECT.
            let bad_path = to_wide_z("\\NoSuchPerfObject\\Foo");
            let mut c_bad = 1usize;
            assert_eq!(PdhAddCounterW(q, bad_path.as_ptr(), 0, &mut c_bad), PDH_CSTATUS_NO_OBJECT);
            assert_eq!(c_bad, 0);

            // Verify Mach host-wide and per-core ticks are real non-zero system counters.
            let t0 = host_cpu_ticks(ProcessorTarget::Total).expect("host_statistics must return ticks");
            assert!(t0.iter().map(|&v| v as u64).sum::<u64>() > 0);
            let core0_t0 = host_cpu_ticks(ProcessorTarget::Core(0)).expect("host_processor_info must return core 0 ticks");
            assert!(core0_t0.iter().map(|&v| v as u64).sum::<u64>() > 0);

            assert_eq!(PdhCollectQueryData(q), ERROR_SUCCESS);
            // Allow at least one host scheduler tick to elapse so host-wide delta is sampled.
            std::thread::sleep(std::time::Duration::from_millis(30));
            assert_eq!(PdhCollectQueryData(q), ERROR_SUCCESS);

            let mut val = [0u8; 16];
            let mut ty = 99u32;
            assert_eq!(
                PdhGetFormattedCounterValue(c_mem, PDH_FMT_LARGE, &mut ty, val.as_mut_ptr()),
                ERROR_SUCCESS
            );
            assert_eq!(std::ptr::read_unaligned(val.as_ptr() as *const u32), PDH_CSTATUS_VALID_DATA);
            assert!(std::ptr::read_unaligned(val.as_ptr().add(8) as *const i64) > 0);

            let read_pct = |ch: usize| -> f64 {
                let mut v = [0u8; 16];
                let mut t = 99u32;
                assert_eq!(PdhGetFormattedCounterValue(ch, PDH_FMT_DOUBLE, &mut t, v.as_mut_ptr()), ERROR_SUCCESS);
                assert_eq!(std::ptr::read_unaligned(v.as_ptr() as *const u32), PDH_CSTATUS_VALID_DATA);
                std::ptr::read_unaligned(v.as_ptr().add(8) as *const f64)
            };
            let cpu_pct = read_pct(c_cpu);
            let user_pct = read_pct(c_user);
            let kern_pct = read_pct(c_kern);
            let core0_pct = read_pct(c_core0);
            let proc_pct = read_pct(c_proc);
            assert!((0.0..=100.0).contains(&cpu_pct));
            assert!((0.0..=100.0).contains(&user_pct));
            assert!((0.0..=100.0).contains(&kern_pct));
            assert!((0.0..=100.0).contains(&core0_pct));
            assert!(proc_pct >= 0.0);
            // Total host CPU percentage equals user + kernel (from the exact same Mach tick snapshot).
            assert!((cpu_pct - (user_pct + kern_pct)).abs() <= 1e-6);

            // Process memory counters report live Mach TASK_VM_INFO metrics (not peak ru_maxrss),
            // distinguish virtual vs resident vs private bytes, and drop when large mappings are freed.
            let read_i64 = |ch: usize| -> i64 {
                let mut v = [0u8; 16];
                let mut t = 99u32;
                assert_eq!(PdhGetFormattedCounterValue(ch, PDH_FMT_LARGE, &mut t, v.as_mut_ptr()), ERROR_SUCCESS);
                assert_eq!(std::ptr::read_unaligned(v.as_ptr() as *const u32), PDH_CSTATUS_VALID_DATA);
                std::ptr::read_unaligned(v.as_ptr().add(8) as *const i64)
            };
            let ws_base = read_i64(c_ws);
            let ws_priv_base = read_i64(c_ws_priv);
            let priv_base = read_i64(c_priv);
            let virt_base = read_i64(c_virt);
            assert!(ws_base > 0 && ws_priv_base > 0 && priv_base > 0 && virt_base > 0);
            assert!(virt_base >= ws_base, "virtual bytes ({virt_base}) must be >= working set ({ws_base})");

            // Fault 32 MiB of anonymous private memory, sample, then unmap and verify current metrics drop.
            const ALLOC_BYTES: usize = 32 * 1024 * 1024;
            let mapped = libc::mmap(
                std::ptr::null_mut(),
                ALLOC_BYTES,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1,
                0,
            );
            assert_ne!(mapped, libc::MAP_FAILED);
            for off in (0..ALLOC_BYTES).step_by(4096) {
                std::ptr::write_volatile((mapped as *mut u8).add(off), 0xA5);
            }
            assert_eq!(PdhCollectQueryData(q), ERROR_SUCCESS);
            let ws_high = read_i64(c_ws);
            let ws_priv_high = read_i64(c_ws_priv);
            let priv_high = read_i64(c_priv);
            let virt_high = read_i64(c_virt);
            assert!(ws_high >= ws_base + (16 * 1024 * 1024));
            assert!(ws_priv_high >= ws_priv_base + (16 * 1024 * 1024));
            assert!(priv_high >= priv_base + (16 * 1024 * 1024));
            assert!(virt_high >= virt_base + (16 * 1024 * 1024));

            assert_eq!(libc::munmap(mapped, ALLOC_BYTES), 0);
            assert_eq!(PdhCollectQueryData(q), ERROR_SUCCESS);
            let ws_after = read_i64(c_ws);
            let ws_priv_after = read_i64(c_ws_priv);
            let priv_after = read_i64(c_priv);
            let virt_after = read_i64(c_virt);
            assert!(
                ws_high - ws_after >= 16 * 1024 * 1024,
                "Working Set must reflect released memory (high={ws_high}, after={ws_after})"
            );
            assert!(
                ws_priv_high - ws_priv_after >= 16 * 1024 * 1024,
                "Working Set - Private must drop on munmap (high={ws_priv_high}, after={ws_priv_after})"
            );
            assert!(
                priv_high - priv_after >= 16 * 1024 * 1024,
                "Private Bytes must drop on munmap (high={priv_high}, after={priv_after})"
            );
            assert!(
                virt_high - virt_after >= 16 * 1024 * 1024,
                "Virtual Bytes must drop on munmap (high={virt_high}, after={virt_after})"
            );

            // Formatting modifiers: PDH_FMT_1000 scales by 1000, default caps percentage counters
            // at 100 while PDH_FMT_NOCAP100 allows >100, and unsupported flags return PDH_INVALID_ARGUMENT.
            let mem_raw_d = read_pct(c_mem);
            let mut v1000 = [0u8; 16];
            assert_eq!(
                PdhGetFormattedCounterValue(c_mem, PDH_FMT_DOUBLE | PDH_FMT_1000, &mut ty, v1000.as_mut_ptr()),
                ERROR_SUCCESS
            );
            assert_eq!(std::ptr::read_unaligned(v1000.as_ptr() as *const u32), PDH_CSTATUS_VALID_DATA);
            let mem_1000 = std::ptr::read_unaligned(v1000.as_ptr().add(8) as *const f64);
            assert!((mem_1000 - mem_raw_d * 1000.0).abs() <= 1e-6);

            let mut vnoscale = [0u8; 16];
            assert_eq!(
                PdhGetFormattedCounterValue(c_mem, PDH_FMT_DOUBLE | PDH_FMT_NOSCALE, &mut ty, vnoscale.as_mut_ptr()),
                ERROR_SUCCESS
            );
            let mem_noscale = std::ptr::read_unaligned(vnoscale.as_ptr().add(8) as *const f64);
            assert!((mem_noscale - mem_raw_d).abs() <= 1e-6);

            // Seed >100% multi-core process CPU value and verify default 100% cap vs PDH_FMT_NOCAP100.
            {
                let mut st = state().lock().unwrap();
                st.counters.get_mut(&c_proc).unwrap().last_value = Some(245.5);
            }
            assert!((read_pct(c_proc) - 100.0).abs() <= 1e-6, "default formatting must cap percentage at 100");
            let mut v_nocap = [0u8; 16];
            assert_eq!(
                PdhGetFormattedCounterValue(
                    c_proc,
                    PDH_FMT_DOUBLE | PDH_FMT_NOCAP100,
                    &mut ty,
                    v_nocap.as_mut_ptr()
                ),
                ERROR_SUCCESS
            );
            assert!((std::ptr::read_unaligned(v_nocap.as_ptr().add(8) as *const f64) - 245.5).abs() <= 1e-6);
            let mut v_nocap_1000 = [0u8; 16];
            assert_eq!(
                PdhGetFormattedCounterValue(
                    c_proc,
                    PDH_FMT_DOUBLE | PDH_FMT_NOCAP100 | PDH_FMT_1000,
                    &mut ty,
                    v_nocap_1000.as_mut_ptr()
                ),
                ERROR_SUCCESS
            );
            assert!(
                (std::ptr::read_unaligned(v_nocap_1000.as_ptr().add(8) as *const f64) - 245_500.0).abs() <= 1e-6
            );

            // Unsupported or missing format flags must fail honestly with PDH_INVALID_ARGUMENT.
            for bad_fmt in [
                0u32,
                PDH_FMT_LONG | PDH_FMT_DOUBLE,
                PDH_FMT_DOUBLE | 0x0000_0010, // PDH_FMT_RAW
                PDH_FMT_DOUBLE | 0x0000_4000, // PDH_FMT_NODATA
                PDH_FMT_DOUBLE | 0x0001_0000, // unknown modifier
            ] {
                let mut bad_v = [0xAAu8; 16];
                assert_eq!(
                    PdhGetFormattedCounterValue(c_mem, bad_fmt, &mut ty, bad_v.as_mut_ptr()),
                    PDH_INVALID_ARGUMENT,
                    "expected PDH_INVALID_ARGUMENT for dw_format={bad_fmt:#x}"
                );
            }

            assert_eq!(PdhCloseQuery(q), ERROR_SUCCESS);
            assert_eq!(PdhCloseQuery(q), PDH_INVALID_HANDLE);
            assert_eq!(
                PdhGetFormattedCounterValue(c_mem, PDH_FMT_DOUBLE, std::ptr::null_mut(), val.as_mut_ptr()),
                PDH_INVALID_HANDLE
            );
        }
    }
}
