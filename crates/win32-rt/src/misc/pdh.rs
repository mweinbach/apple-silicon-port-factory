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
pub const PDH_CSTATUS_NO_INSTANCE: i32 = 0x8000_07D1u32 as i32;
pub const PDH_CSTATUS_NO_OBJECT: i32 = 0xC000_0BB8u32 as i32;
pub const PDH_CSTATUS_NO_COUNTER: i32 = 0xC000_0BB9u32 as i32;
pub const PDH_INVALID_HANDLE: i32 = 0xC000_0BBCu32 as i32;
pub const PDH_INVALID_ARGUMENT: i32 = 0xC000_0BBDu32 as i32;
pub const PDH_NO_DATA: i32 = 0x8000_07D5u32 as i32;

pub const PDH_FMT_LONG: u32 = 0x0000_0100;
pub const PDH_FMT_DOUBLE: u32 = 0x0000_0200;
pub const PDH_FMT_LARGE: u32 = 0x0000_0400;
const PDH_FMT_TYPE_MASK: u32 = 0x0000_0F00;

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
    MemoryCommitPct,
    ProcessWorkingSet,
    ProcessPrivateBytes,
    ProcessProcessorPct,
    SystemUptimeSec,
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

fn parse_counter_path(path: &str) -> Result<CounterKind, i32> {
    let trimmed = path.trim();
    // Strip optional \\machine prefix and leading '\'.
    let rest = if let Some(after_unc) = trimmed.strip_prefix("\\\\") {
        match after_unc.find('\\') {
            Some(pos) => &after_unc[pos + 1..],
            None => return Err(PDH_CSTATUS_NO_OBJECT),
        }
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
            "% committed bytes in use" => Ok(CounterKind::MemoryCommitPct),
            _ => Err(PDH_CSTATUS_NO_COUNTER),
        },
        "process" => match ctr_name.as_str() {
            "working set" | "working set - private" => Ok(CounterKind::ProcessWorkingSet),
            "private bytes" | "virtual bytes" => Ok(CounterKind::ProcessPrivateBytes),
            "% processor time" => Ok(CounterKind::ProcessProcessorPct),
            _ => Err(PDH_CSTATUS_NO_COUNTER),
        },
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

fn process_rss_bytes() -> u64 {
    unsafe {
        let mut ru: libc::rusage = std::mem::zeroed();
        if libc::getrusage(libc::RUSAGE_SELF, &mut ru) == 0 && ru.ru_maxrss > 0 {
            // macOS reports ru_maxrss in bytes.
            ru.ru_maxrss as u64
        } else {
            0
        }
    }
}

fn sample_counter(
    c: &mut CounterState,
    now: Instant,
    cached_total_ticks: &mut Option<Option<[u32; 4]>>,
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
        | CounterKind::MemoryAvailMBytes
        | CounterKind::MemoryCommitPct => {
            let mut ms = crate::kernel32::sysinfo::MEMORYSTATUSEX {
                dwLength: std::mem::size_of::<crate::kernel32::sysinfo::MEMORYSTATUSEX>() as u32,
                ..unsafe { std::mem::zeroed() }
            };
            unsafe {
                crate::kernel32::sysinfo::GlobalMemoryStatusEx(&mut ms);
                let mem_load = ms.dwMemoryLoad;
                let avail_phys = ms.ullAvailPhys;
                match c.kind {
                    CounterKind::MemoryAvailBytes => avail_phys as f64,
                    CounterKind::MemoryAvailKBytes => (avail_phys / 1024) as f64,
                    CounterKind::MemoryAvailMBytes => (avail_phys / (1024 * 1024)) as f64,
                    CounterKind::MemoryCommitPct => mem_load as f64,
                    _ => 0.0,
                }
            }
        }
        CounterKind::ProcessWorkingSet | CounterKind::ProcessPrivateBytes => {
            process_rss_bytes() as f64
        }
        CounterKind::SystemUptimeSec => {
            (unsafe { crate::kernel32::time::GetTickCount64() } as f64) / 1000.0
        }
    };
    c.last_value = Some(val);
}

pub unsafe extern "win64" fn PdhOpenQueryW(
    _data_source: LPCWSTR,
    _user_data: usize,
    ph_query: *mut usize,
) -> i32 {
    if ph_query.is_null() {
        return PDH_INVALID_ARGUMENT;
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
    for cid in counter_ids {
        if let Some(c) = st.counters.get_mut(&cid) {
            sample_counter(c, now, &mut cached_total_ticks);
        }
    }
    ERROR_SUCCESS
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
    let st = state().lock().unwrap();
    let Some(c) = st.counters.get(&h_counter) else {
        return PDH_INVALID_HANDLE;
    };
    let _ = c.query;
    let Some(val) = c.last_value else {
        std::ptr::write_unaligned(p_value as *mut u32, PDH_NO_DATA as u32);
        return PDH_NO_DATA;
    };
    if !lpdw_type.is_null() {
        std::ptr::write_unaligned(lpdw_type, 0u32);
    }
    std::ptr::write_bytes(p_value, 0, 16);
    std::ptr::write_unaligned(p_value as *mut u32, PDH_CSTATUS_VALID_DATA);
    match dw_format & PDH_FMT_TYPE_MASK {
        PDH_FMT_LONG => std::ptr::write_unaligned(p_value.add(8) as *mut i32, val as i32),
        PDH_FMT_LARGE => std::ptr::write_unaligned(p_value.add(8) as *mut i64, val as i64),
        PDH_FMT_DOUBLE | 0 => std::ptr::write_unaligned(p_value.add(8) as *mut f64, val),
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

            let proc_path = to_wide_z("\\Process(self)\\% Processor Time");
            let mut c_proc = 0usize;
            assert_eq!(PdhAddCounterW(q, proc_path.as_ptr(), 0, &mut c_proc), ERROR_SUCCESS);

            // Out-of-range or missing processor instance fails honestly with PDH_CSTATUS_NO_INSTANCE.
            let bad_inst = to_wide_z("\\Processor(99999)\\% Processor Time");
            let mut c_bad_inst = 1usize;
            assert_eq!(PdhAddCounterW(q, bad_inst.as_ptr(), 0, &mut c_bad_inst), PDH_CSTATUS_NO_INSTANCE);
            assert_eq!(c_bad_inst, 0);

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

            let mut read_pct = |ch: usize| -> f64 {
                let mut v = [0u8; 16];
                assert_eq!(PdhGetFormattedCounterValue(ch, PDH_FMT_DOUBLE, &mut ty, v.as_mut_ptr()), ERROR_SUCCESS);
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

            assert_eq!(PdhCloseQuery(q), ERROR_SUCCESS);
            assert_eq!(PdhCloseQuery(q), PDH_INVALID_HANDLE);
            assert_eq!(
                PdhGetFormattedCounterValue(c_mem, PDH_FMT_DOUBLE, std::ptr::null_mut(), val.as_mut_ptr()),
                PDH_INVALID_HANDLE
            );
        }
    }
}
