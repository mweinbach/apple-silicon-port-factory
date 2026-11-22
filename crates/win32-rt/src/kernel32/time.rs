//! Time: performance counter, tick count, system/local time, sleeping.

use crate::types::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

/// Shared lock-free fast-path state for `QueryPerformanceCounter`, read both by Rust
/// [`interrupt_time_100ns`] and directly by inlined ARM64 JIT/AOT code.
#[repr(C, align(16))]
pub struct QpcFastState {
    /// `0` = uninitialized, `1` = raw `CNTVCT_EL0` fast path active, `2` = fallback (`mach`/shadow).
    pub mode: AtomicU64,
    /// `CNTVCT_EL0` ticks -> 100 ns as a Q64 fraction (`1e7 * 2^64 / CNTFRQ_EL0`).
    pub mult: AtomicU64,
    /// Offset added to scaled `CNTVCT_EL0` so raw reads match `mach_absolute_time` (100 ns).
    pub offset: AtomicU64,
}

static QPC_STATE: QpcFastState = QpcFastState {
    mode: AtomicU64::new(0),
    mult: AtomicU64::new(0),
    offset: AtomicU64::new(0),
};

/// Address of the process-wide [`QpcFastState`] for inlined ARM64 `QueryPerformanceCounter`.
/// Eagerly initializes the clock state when safe so the very first inlined call takes the fast path.
pub fn qpc_fast_state_ptr() -> usize {
    if QPC_STATE.mode.load(Ordering::Acquire) == 0 && crate::shadow::frozen_100ns().is_none() {
        let _ = init_clock_fast();
    }
    &QPC_STATE as *const QpcFastState as usize
}

/// Force `QueryPerformanceCounter` onto its slow path (used when shadow clock freezing activates).
pub fn disable_qpc_fast_path() {
    QPC_STATE.mode.store(2, Ordering::Release);
}

#[repr(C)]
struct MachTimebaseInfo {
    numer: u32,
    denom: u32,
}

extern "C" {
    fn mach_timebase_info(info: *mut MachTimebaseInfo) -> i32;
    fn mach_absolute_time() -> u64;
}

fn timebase() -> (u32, u32) {
    static TB: OnceLock<(u32, u32)> = OnceLock::new();
    *TB.get_or_init(|| {
        let mut info = MachTimebaseInfo { numer: 0, denom: 0 };
        unsafe { mach_timebase_info(&mut info) };
        (info.numer, info.denom)
    })
}

/// Tick source and scale for [`interrupt_time_100ns`].
struct Clock {
    /// Added to the scaled counter so raw reads agree with the
    /// `mach_absolute_time` clock (100 ns units).
    offset: u64,
    /// `CNTVCT_EL0` ticks -> 100 ns as a Q64 fraction (`1e7 / CNTFRQ_EL0`).
    mult: u64,
    /// Read the counter directly (no ISB) instead of `mach_absolute_time`.
    raw: bool,
}

fn mach_100ns() -> u64 {
    let t = unsafe { mach_absolute_time() };
    let (n, d) = timebase();
    ((t as u128 * n as u128 / d as u128) / 100) as u64
}

fn clock() -> &'static Clock {
    static C: OnceLock<Clock> = OnceLock::new();
    C.get_or_init(|| {
        // CNTVCT_EL0 runs at CNTFRQ_EL0, which need not match mach's timebase
        // (1 GHz vs 24 MHz on recent Apple silicon).
        let frq = cntfrq();
        let raw = cfg!(target_arch = "aarch64")
            && frq > QPC_FREQUENCY as u64
            && !std::env::var("AW2_QPC_ISB").is_ok_and(|v| v == "1");
        if !raw {
            return Clock { offset: 0, mult: 0, raw: false };
        }
        let mult = (((QPC_FREQUENCY as u128) << 64) / frq as u128) as u64;
        let scaled = ((cntvct() as u128 * mult as u128) >> 64) as u64;
        Clock { offset: mach_100ns().wrapping_sub(scaled), mult, raw }
    })
}

#[inline(always)]
fn cntfrq() -> u64 {
    #[cfg(target_arch = "aarch64")]
    {
        let v: u64;
        unsafe { std::arch::asm!("mrs {}, cntfrq_el0", out(reg) v, options(nomem, nostack, preserves_flags)) };
        v
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        0
    }
}

/// `CNTVCT_EL0` without the ISB that `mach_absolute_time` issues first: the
/// read may be performed a few instructions early, far below the 100 ns QPC
/// resolution that callers ever compare across threads.
#[inline(always)]
fn cntvct() -> u64 {
    #[cfg(target_arch = "aarch64")]
    {
        let v: u64;
        unsafe { std::arch::asm!("mrs {}, cntvct_el0", out(reg) v, options(nomem, nostack, preserves_flags)) };
        v
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        #[allow(deprecated)]
        unsafe {
            libc::mach_absolute_time()
        }
    }
}

#[cold]
#[inline(never)]
fn init_clock_fast() -> u64 {
    if crate::shadow::frozen_100ns().is_some() {
        QPC_STATE.mode.store(2, Ordering::Release);
        return 2;
    }
    let c = clock();
    if c.raw {
        QPC_STATE.mult.store(c.mult, Ordering::Relaxed);
        QPC_STATE.offset.store(c.offset, Ordering::Relaxed);
        QPC_STATE.mode.store(1, Ordering::Release);
        1
    } else {
        QPC_STATE.mode.store(2, Ordering::Release);
        2
    }
}

/// Monotonic time since boot in 100ns units (Windows "interrupt time").
#[inline(always)]
pub fn interrupt_time_100ns() -> u64 {
    if let Some(v) = crate::shadow::frozen_100ns() {
        return v;
    }
    let mut mode = QPC_STATE.mode.load(Ordering::Acquire);
    if mode == 0 {
        mode = init_clock_fast();
    }
    if mode == 1 {
        let mult = QPC_STATE.mult.load(Ordering::Relaxed);
        let offset = QPC_STATE.offset.load(Ordering::Relaxed);
        return (((cntvct() as u128 * mult as u128) >> 64) as u64).wrapping_add(offset);
    }
    mach_100ns()
}

pub const QPC_FREQUENCY: i64 = 10_000_000;

#[inline(never)]
pub unsafe extern "win64" fn QueryPerformanceCounter(out: *mut i64) -> BOOL {
    *out = interrupt_time_100ns() as i64;
    TRUE
}

#[inline(never)]
pub unsafe extern "win64" fn QueryPerformanceFrequency(out: *mut i64) -> BOOL {
    *out = QPC_FREQUENCY;
    TRUE
}

pub unsafe extern "win64" fn GetTickCount() -> u32 {
    (interrupt_time_100ns() / 10_000) as u32
}

pub unsafe extern "win64" fn GetTickCount64() -> u64 {
    interrupt_time_100ns() / 10_000
}

pub unsafe extern "win64" fn QueryInterruptTime(out: *mut u64) {
    *out = interrupt_time_100ns();
}

pub unsafe extern "win64" fn QueryUnbiasedInterruptTime(out: *mut u64) -> BOOL {
    *out = interrupt_time_100ns();
    TRUE
}

pub unsafe extern "win64" fn GetSystemTimeAsFileTime(ft: *mut FILETIME) {
    *ft = FILETIME::from_u64(now_filetime());
}

pub unsafe extern "win64" fn GetSystemTimePreciseAsFileTime(ft: *mut FILETIME) {
    *ft = FILETIME::from_u64(now_filetime());
}

// Civil date conversion (Howard Hinnant's algorithms).
pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp as i64 + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// FILETIME (100ns since 1601) -> SYSTEMTIME (no time zone conversion).
pub fn filetime_to_systemtime(ft: u64) -> SYSTEMTIME {
    let total_ms = ft / 10_000;
    let ms = (total_ms % 1000) as u16;
    let secs = total_ms / 1000;
    let days = (secs / 86400) as i64;
    let rem = secs % 86400;
    // 1601-01-01 is day -134774 relative to 1970-01-01.
    let (y, m, d) = civil_from_days(days - 134774);
    SYSTEMTIME {
        wYear: y as u16,
        wMonth: m as u16,
        wDayOfWeek: ((days + 1) % 7) as u16, // 1601-01-01 was a Monday
        wDay: d as u16,
        wHour: (rem / 3600) as u16,
        wMinute: (rem % 3600 / 60) as u16,
        wSecond: (rem % 60) as u16,
        wMilliseconds: ms,
    }
}

pub fn systemtime_to_filetime(st: &SYSTEMTIME) -> Option<u64> {
    if st.wMonth < 1 || st.wMonth > 12 || st.wDay < 1 || st.wDay > 31 || st.wHour > 23 || st.wMinute > 59 || st.wSecond > 59 || st.wMilliseconds > 999 || st.wYear < 1601 {
        return None;
    }
    let days = days_from_civil(st.wYear as i64, st.wMonth as u32, st.wDay as u32) + 134774;
    let secs = days as u64 * 86400 + st.wHour as u64 * 3600 + st.wMinute as u64 * 60 + st.wSecond as u64;
    Some((secs * 1000 + st.wMilliseconds as u64) * 10_000)
}

/// Host UTC offset in seconds for "now" (positive east of UTC).
pub fn local_utc_offset_secs(at_unix: i64) -> i64 {
    unsafe {
        let t: libc::time_t = at_unix;
        let mut tm: libc::tm = std::mem::zeroed();
        // localtime_r (reentrant) — NOT localtime(), which uses TSD slot 12 (our PEB).
        libc::localtime_r(&t, &mut tm);
        tm.tm_gmtoff
    }
}

fn now_unix() -> i64 {
    ((now_filetime() - EPOCH_DIFF_100NS) / 10_000_000) as i64
}

pub unsafe extern "win64" fn GetSystemTime(st: *mut SYSTEMTIME) {
    *st = filetime_to_systemtime(now_filetime());
}

pub unsafe extern "win64" fn GetLocalTime(st: *mut SYSTEMTIME) {
    let off = local_utc_offset_secs(now_unix());
    *st = filetime_to_systemtime((now_filetime() as i64 + off * 10_000_000) as u64);
}

pub unsafe extern "win64" fn FileTimeToSystemTime(ft: *const FILETIME, st: *mut SYSTEMTIME) -> BOOL {
    let v = (*ft).to_u64();
    if v >= 0x8000_0000_0000_0000 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    *st = filetime_to_systemtime(v);
    TRUE
}

pub unsafe extern "win64" fn SystemTimeToFileTime(st: *const SYSTEMTIME, ft: *mut FILETIME) -> BOOL {
    match systemtime_to_filetime(&*st) {
        Some(v) => {
            *ft = FILETIME::from_u64(v);
            TRUE
        }
        None => {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn FileTimeToLocalFileTime(ft: *const FILETIME, out: *mut FILETIME) -> BOOL {
    let v = (*ft).to_u64();
    let unix = (v as i64 - EPOCH_DIFF_100NS as i64) / 10_000_000;
    *out = FILETIME::from_u64((v as i64 + local_utc_offset_secs(unix) * 10_000_000) as u64);
    TRUE
}

pub unsafe extern "win64" fn LocalFileTimeToFileTime(ft: *const FILETIME, out: *mut FILETIME) -> BOOL {
    let v = (*ft).to_u64();
    let unix = (v as i64 - EPOCH_DIFF_100NS as i64) / 10_000_000;
    *out = FILETIME::from_u64((v as i64 - local_utc_offset_secs(unix) * 10_000_000) as u64);
    TRUE
}

pub unsafe extern "win64" fn SystemTimeToTzSpecificLocalTime(_tz: *const u8, utc: *const SYSTEMTIME, local: *mut SYSTEMTIME) -> BOOL {
    let Some(v) = systemtime_to_filetime(&*utc) else { return FALSE };
    let unix = (v as i64 - EPOCH_DIFF_100NS as i64) / 10_000_000;
    *local = filetime_to_systemtime((v as i64 + local_utc_offset_secs(unix) * 10_000_000) as u64);
    TRUE
}

pub unsafe extern "win64" fn CompareFileTime(a: *const FILETIME, b: *const FILETIME) -> i32 {
    match (*a).to_u64().cmp(&(*b).to_u64()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

const TIME_ZONE_ID_UNKNOWN: u32 = 0;
const TIME_ZONE_ID_STANDARD: u32 = 1;
const TIME_ZONE_ID_DAYLIGHT: u32 = 2;
const TIME_ZONE_ID_INVALID: u32 = 0xFFFF_FFFF;

fn days_in_month(y: i64, m: u32) -> u32 {
    let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
    (days_from_civil(ny, nm, 1) - days_from_civil(y, m, 1)) as u32
}

#[derive(Clone, Copy)]
struct LocalProbe {
    isdst: bool,
    gmtoff: i64,
}

unsafe fn probe_local_fast(t: i64) -> LocalProbe {
    let tt: libc::time_t = t;
    let mut tm: libc::tm = std::mem::zeroed();
    // localtime_r (reentrant) — NOT localtime(), which uses TSD slot 12 (our PEB).
    libc::localtime_r(&tt, &mut tm);
    LocalProbe { isdst: tm.tm_isdst > 0, gmtoff: tm.tm_gmtoff }
}

unsafe fn probe_local_full(t: i64) -> (LocalProbe, i64, String) {
    let tt: libc::time_t = t;
    let mut tm: libc::tm = std::mem::zeroed();
    libc::localtime_r(&tt, &mut tm);
    let name = if tm.tm_zone.is_null() {
        "UTC".into()
    } else {
        std::ffi::CStr::from_ptr(tm.tm_zone).to_string_lossy().into_owned()
    };
    (
        LocalProbe { isdst: tm.tm_isdst > 0, gmtoff: tm.tm_gmtoff },
        tm.tm_year as i64 + 1900,
        name,
    )
}

/// Encode a DST transition instant `trans_unix` using the pre-transition UTC offset
/// `pre_gmtoff` into Windows recurring `SYSTEMTIME` format (`wYear = 0`, `wDay` = 1..5).
/// Returns `(local_year, SYSTEMTIME)`.
fn encode_transition_systemtime(trans_unix: i64, pre_gmtoff: i64) -> (i64, SYSTEMTIME) {
    let local_secs = trans_unix + pre_gmtoff;
    let days = local_secs.div_euclid(86_400);
    let sec_of_day = local_secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    // 1970-01-01 (days == 0) was Thursday (4); Windows Sunday == 0 .. Saturday == 6.
    let dow = (days + 4).rem_euclid(7) as u16;
    let dim = days_in_month(y, m);
    // Windows recurring format: 1..4 = nth occurrence in month, 5 = last occurrence.
    let nth = if d + 7 > dim { 5 } else { (d - 1) / 7 + 1 } as u16;
    let st = SYSTEMTIME {
        wYear: 0,
        wMonth: m as u16,
        wDayOfWeek: dow,
        wDay: nth,
        wHour: (sec_of_day / 3600) as u16,
        wMinute: ((sec_of_day % 3600) / 60) as u16,
        wSecond: 0,
        wMilliseconds: 0,
    };
    (y, st)
}

unsafe fn write_wide_fixed32(dst: *mut u8, s: &str) {
    for (i, c) in s.encode_utf16().take(31).enumerate() {
        std::ptr::write_unaligned(dst.add(i * 2) as *mut u16, c);
    }
}

/// Populate `TIME_ZONE_INFORMATION` (172 bytes) for the specified Unix timestamp `now`.
unsafe fn fill_tzi_at(p: *mut u8, now: i64) -> u32 {
    if p.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return TIME_ZONE_ID_INVALID;
    }
    std::ptr::write_bytes(p, 0, 172);

    let (now_probe, year, now_name) = probe_local_full(now);

    // Scan the full local calendar year with ±15h UTC padding so every UTC offset
    // (-12..+14) is covered on minute-aligned boundaries.
    let jan1_utc = days_from_civil(year, 1, 1) * 86_400;
    let next_jan1_utc = days_from_civil(year + 1, 1, 1) * 86_400;
    let scan_start = jan1_utc - 15 * 3600;
    let scan_end = next_jan1_utc + 15 * 3600;

    let mut std_date = SYSTEMTIME::default();
    let mut dst_date = SYSTEMTIME::default();
    let mut has_std_trans = false;
    let mut has_dst_trans = false;
    let mut std_sample: Option<(i64, i64)> = None;
    let mut dst_sample: Option<(i64, i64)> = None;

    let mut prev_t = scan_start;
    let mut prev = probe_local_fast(prev_t);
    while prev_t < scan_end {
        let cur_t = (prev_t + 86_400).min(scan_end);
        let cur = probe_local_fast(cur_t);
        if cur.isdst != prev.isdst {
            // Binary search within (prev_t, cur_t] down to the exact minute of the flip.
            let mut lo_min = prev_t.div_euclid(60);
            let mut hi_min = cur_t.div_euclid(60);
            while hi_min - lo_min > 1 {
                let mid = lo_min + (hi_min - lo_min) / 2;
                if probe_local_fast(mid * 60).isdst == prev.isdst {
                    lo_min = mid;
                } else {
                    hi_min = mid;
                }
            }
            let pre_t = lo_min * 60;
            let trans_t = hi_min * 60;
            let pre_probe = probe_local_fast(pre_t);
            let post_probe = probe_local_fast(trans_t);
            let (ty, st) = encode_transition_systemtime(trans_t, pre_probe.gmtoff);
            if ty == year {
                if !pre_probe.isdst && post_probe.isdst {
                    // 0 -> 1: transition into Daylight Saving Time (DaylightDate).
                    dst_date = st;
                    has_dst_trans = true;
                    std_sample.get_or_insert((pre_t, pre_probe.gmtoff));
                    dst_sample.get_or_insert((trans_t, post_probe.gmtoff));
                } else if pre_probe.isdst && !post_probe.isdst {
                    // 1 -> 0: transition back to Standard Time (StandardDate).
                    std_date = st;
                    has_std_trans = true;
                    dst_sample.get_or_insert((pre_t, pre_probe.gmtoff));
                    std_sample.get_or_insert((trans_t, post_probe.gmtoff));
                }
            }
        }
        prev_t = cur_t;
        prev = cur;
    }

    if has_std_trans || has_dst_trans {
        let (std_t, std_off) = std_sample.unwrap_or((now, now_probe.gmtoff));
        let (dst_t, dst_off) = dst_sample.unwrap_or((now, now_probe.gmtoff));
        let (_, _, std_name) = probe_local_full(std_t);
        let (_, _, dst_name) = probe_local_full(dst_t);

        let bias = (-std_off / 60) as i32;
        let std_bias = 0i32;
        let dst_bias = ((std_off - dst_off) / 60) as i32;

        std::ptr::write_unaligned(p as *mut i32, bias);
        write_wide_fixed32(p.add(4), &std_name);
        std::ptr::write_unaligned(p.add(68) as *mut SYSTEMTIME, std_date);
        std::ptr::write_unaligned(p.add(84) as *mut i32, std_bias);
        write_wide_fixed32(p.add(88), &dst_name);
        std::ptr::write_unaligned(p.add(152) as *mut SYSTEMTIME, dst_date);
        std::ptr::write_unaligned(p.add(168) as *mut i32, dst_bias);

        if now_probe.isdst {
            TIME_ZONE_ID_DAYLIGHT
        } else {
            TIME_ZONE_ID_STANDARD
        }
    } else {
        let bias = (-now_probe.gmtoff / 60) as i32;
        std::ptr::write_unaligned(p as *mut i32, bias);
        write_wide_fixed32(p.add(4), &now_name);
        write_wide_fixed32(p.add(88), &now_name);
        TIME_ZONE_ID_UNKNOWN
    }
}

/// TIME_ZONE_INFORMATION (172 bytes).
pub unsafe fn fill_tzi(p: *mut u8) -> u32 {
    fill_tzi_at(p, now_unix())
}

pub unsafe extern "win64" fn GetTimeZoneInformation(p: *mut u8) -> u32 {
    fill_tzi(p)
}

pub unsafe extern "win64" fn GetDynamicTimeZoneInformation(p: *mut u8) -> u32 {
    if p.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return TIME_ZONE_ID_INVALID;
    }
    std::ptr::write_bytes(p, 0, 432);
    let r = fill_tzi(p);
    // TimeZoneKeyName[128] at 172 (copy StandardName[32] = 64 bytes),
    // DynamicDaylightTimeDisabled at 428 stays 0 (FALSE).
    std::ptr::copy_nonoverlapping(p.add(4), p.add(172), 64);
    r
}

fn precise_sleep() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("AW2_PRECISE_SLEEP").is_some_and(|v| v != "0"))
}

fn mach_sleep_ns(ns: u64) {
    unsafe extern "C" {
        fn mach_wait_until(deadline: u64) -> i32;
    }
    let (n, d) = timebase();
    if n == 0 {
        std::thread::sleep(std::time::Duration::from_nanos(ns));
        return;
    }
    let now = unsafe { mach_absolute_time() };
    let ticks = (ns as u128 * d as u128 / n as u128) as u64;
    unsafe {
        let _ = mach_wait_until(now.saturating_add(ticks));
    }
}

pub unsafe extern "win64" fn Sleep(ms: u32) {
    sleep_ms(ms);
}

pub fn sleep_ms(ms: u32) {
    if ms == 0 {
        std::thread::yield_now();
    } else if ms == INFINITE {
        loop {
            std::thread::park();
        }
    } else if ms <= 15 && precise_sleep() {
        mach_sleep_ns(ms as u64 * 1_000_000);
    } else {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }
}

pub unsafe extern "win64" fn SleepEx(ms: u32, alertable: BOOL) -> u32 {
    if alertable != 0 {
        if crate::handles::alertable_sleep(ms) {
            return WAIT_IO_COMPLETION;
        }
        return 0;
    }
    sleep_ms(ms);
    0
}

#[inline(never)]
pub unsafe extern "win64" fn SwitchToThread() -> BOOL {
    libc::sched_yield();
    TRUE
}

fn timeval_to_100ns(tv: libc::timeval) -> u64 {
    tv.tv_sec as u64 * 10_000_000 + tv.tv_usec as u64 * 10
}

static PROCESS_START: OnceLock<u64> = OnceLock::new();

pub fn process_start_filetime() -> u64 {
    *PROCESS_START.get_or_init(now_filetime)
}

pub unsafe extern "win64" fn GetProcessTimes(_h: HANDLE, create: *mut FILETIME, exit: *mut FILETIME, kernel: *mut FILETIME, user: *mut FILETIME) -> BOOL {
    let mut ru: libc::rusage = std::mem::zeroed();
    libc::getrusage(libc::RUSAGE_SELF, &mut ru);
    *create = FILETIME::from_u64(process_start_filetime());
    *exit = FILETIME::default();
    *kernel = FILETIME::from_u64(timeval_to_100ns(ru.ru_stime));
    *user = FILETIME::from_u64(timeval_to_100ns(ru.ru_utime));
    TRUE
}

fn current_thread_cpu_100ns() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut ts) };
    ts.tv_sec as u64 * 10_000_000 + ts.tv_nsec as u64 / 100
}

pub unsafe extern "win64" fn GetThreadTimes(h: HANDLE, create: *mut FILETIME, exit: *mut FILETIME, kernel: *mut FILETIME, user: *mut FILETIME) -> BOOL {
    let is_self = h == crate::handles::CURRENT_THREAD
        || crate::handles::get_as::<crate::thread::ThreadObj>(h).is_some_and(|t| t.tid.load(std::sync::atomic::Ordering::Relaxed) as u32 == crate::teb::thread_id());
    *create = FILETIME::from_u64(process_start_filetime());
    *exit = FILETIME::default();
    *kernel = FILETIME::default();
    if is_self {
        *user = FILETIME::from_u64(current_thread_cpu_100ns());
    } else {
        stub!("GetThreadTimes for another thread returns 0 CPU time");
        *user = FILETIME::default();
    }
    TRUE
}

pub unsafe extern "win64" fn QueryThreadCycleTime(h: HANDLE, cycles: *mut u64) -> BOOL {
    let is_self = h == crate::handles::CURRENT_THREAD
        || crate::handles::get_as::<crate::thread::ThreadObj>(h).is_some_and(|t| t.tid.load(std::sync::atomic::Ordering::Relaxed) as u32 == crate::teb::thread_id());
    if is_self {
        // Report CPU time in ns as "cycles" (monotonic, proportional).
        *cycles = current_thread_cpu_100ns() * 100;
    } else {
        stub!("QueryThreadCycleTime for another thread returns 0");
        *cycles = 0;
    }
    TRUE
}

pub unsafe extern "win64" fn QueryProcessCycleTime(_h: HANDLE, cycles: *mut u64) -> BOOL {
    let mut ru: libc::rusage = std::mem::zeroed();
    libc::getrusage(libc::RUSAGE_SELF, &mut ru);
    *cycles = (timeval_to_100ns(ru.ru_utime) + timeval_to_100ns(ru.ru_stime)) * 100;
    TRUE
}

pub unsafe extern "win64" fn GetTimeFormatA(
    _locale: u32,
    _flags: u32,
    time: *const SYSTEMTIME,
    _format: LPCSTR,
    time_str: LPSTR,
    cch_time: i32,
) -> i32 {
    let mut st = SYSTEMTIME::default();
    if time.is_null() {
        GetLocalTime(&mut st);
    } else {
        st = *time;
    }
    let formatted = format!("{:02}:{:02}:{:02}", st.wHour, st.wMinute, st.wSecond);
    let needed = (formatted.len() + 1) as i32;
    if cch_time == 0 {
        return needed;
    }
    if time_str.is_null() || cch_time < needed {
        crate::teb::set_last_error(crate::types::err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    crate::strings::write_ansi_trunc(&formatted, time_str, cch_time as usize);
    needed
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn civil_roundtrip() {
        let st = SYSTEMTIME { wYear: 2024, wMonth: 2, wDayOfWeek: 0, wDay: 29, wHour: 13, wMinute: 14, wSecond: 15, wMilliseconds: 16 };
        let ft = systemtime_to_filetime(&st).unwrap();
        let back = filetime_to_systemtime(ft);
        assert_eq!((back.wYear, back.wMonth, back.wDay, back.wHour, back.wMinute, back.wSecond, back.wMilliseconds), (2024, 2, 29, 13, 14, 15, 16));
        assert_eq!(back.wDayOfWeek, 4); // Thursday
        assert_eq!(filetime_to_systemtime(EPOCH_DIFF_100NS).wYear, 1970);
    }

    #[test]
    fn precise_sleep_default_off_and_mach_sleep_deadline() {
        if std::env::var_os("AW2_PRECISE_SLEEP").is_none() {
            assert!(!precise_sleep());
        }
        let t0 = std::time::Instant::now();
        mach_sleep_ns(1_000_000);
        assert!(t0.elapsed() >= std::time::Duration::from_micros(900));
    }

    unsafe fn read_tzi_fields(p: *const u8) -> (i32, String, SYSTEMTIME, i32, String, SYSTEMTIME, i32) {
        let bias = std::ptr::read_unaligned(p as *const i32);
        let read_w32 = |off: usize| -> String {
            let mut units = Vec::new();
            for i in 0..32 {
                let c = std::ptr::read_unaligned(p.add(off + i * 2) as *const u16);
                if c == 0 {
                    break;
                }
                units.push(c);
            }
            String::from_utf16_lossy(&units)
        };
        let std_name = read_w32(4);
        let std_date = std::ptr::read_unaligned(p.add(68) as *const SYSTEMTIME);
        let std_bias = std::ptr::read_unaligned(p.add(84) as *const i32);
        let dst_name = read_w32(88);
        let dst_date = std::ptr::read_unaligned(p.add(152) as *const SYSTEMTIME);
        let dst_bias = std::ptr::read_unaligned(p.add(168) as *const i32);
        (bias, std_name, std_date, std_bias, dst_name, dst_date, dst_bias)
    }

    fn st_tuple(st: SYSTEMTIME) -> (u16, u16, u16, u16, u16, u16, u16, u16) {
        (st.wYear, st.wMonth, st.wDayOfWeek, st.wDay, st.wHour, st.wMinute, st.wSecond, st.wMilliseconds)
    }

    extern "C" {
        fn tzset();
    }

    struct TzGuard(Option<std::ffi::OsString>);
    impl TzGuard {
        fn set(tz: &str) -> Self {
            let prev = std::env::var_os("TZ");
            std::env::set_var("TZ", tz);
            unsafe { tzset() };
            Self(prev)
        }
    }
    impl Drop for TzGuard {
        fn drop(&mut self) {
            match &self.0 {
                Some(v) => std::env::set_var("TZ", v),
                None => std::env::remove_var("TZ"),
            }
            unsafe { tzset() };
        }
    }

    #[test]
    fn timezone_information_and_dynamic_tzi_match_host_and_dst_rules() {
        unsafe {
            // Unaligned buffer (+1 byte offset) exercising both exported Win32 APIs on host TZ.
            let mut raw = [0xAAu8; 433];
            let p = raw.as_mut_ptr().add(1);
            let id = GetTimeZoneInformation(p);
            assert!(id <= TIME_ZONE_ID_DAYLIGHT, "unexpected TZ id {id}");
            let (bias, std_name, std_date, std_bias, dst_name, dst_date, dst_bias) = read_tzi_fields(p);
            assert!(!std_name.is_empty());
            assert!(!dst_name.is_empty());
            assert_eq!(std_bias, 0);
            assert!((-840..=720).contains(&bias), "bias out of range: {bias}");
            if id == TIME_ZONE_ID_UNKNOWN {
                assert_eq!(st_tuple(std_date), (0, 0, 0, 0, 0, 0, 0, 0));
                assert_eq!(st_tuple(dst_date), (0, 0, 0, 0, 0, 0, 0, 0));
                assert_eq!(dst_bias, 0);
            } else {
                assert_eq!(std_date.wYear, 0);
                assert!((1..=12).contains(&std_date.wMonth));
                assert!(std_date.wDayOfWeek <= 6);
                assert!((1..=5).contains(&std_date.wDay));
                assert!(std_date.wHour <= 23 && std_date.wMinute <= 59);
                assert_eq!(dst_date.wYear, 0);
                assert!((1..=12).contains(&dst_date.wMonth));
                assert!(dst_date.wDayOfWeek <= 6);
                assert!((1..=5).contains(&dst_date.wDay));
                assert!(dst_date.wHour <= 23 && dst_date.wMinute <= 59);
                assert_ne!(dst_bias, 0);
            }

            let dyn_id = GetDynamicTimeZoneInformation(p);
            assert_eq!(dyn_id, id);
            // TimeZoneKeyName at offset 172 should match StandardName.
            let mut key_units = Vec::new();
            for i in 0..32 {
                let c = std::ptr::read_unaligned(p.add(172 + i * 2) as *const u16);
                if c == 0 {
                    break;
                }
                key_units.push(c);
            }
            assert_eq!(String::from_utf16_lossy(&key_units), std_name);
            assert_eq!(*p.add(428), 0); // DynamicDaylightTimeDisabled == FALSE

            // Null pointer check.
            assert_eq!(GetTimeZoneInformation(std::ptr::null_mut()), TIME_ZONE_ID_INVALID);
            assert_eq!(GetDynamicTimeZoneInformation(std::ptr::null_mut()), TIME_ZONE_ID_INVALID);

            // Deterministic timezone probes across Northern, Southern, European, and non-DST zones.
            let jan_2026 = days_from_civil(2026, 1, 15) * 86_400 + 12 * 3600;
            let jul_2026 = days_from_civil(2026, 7, 15) * 86_400 + 12 * 3600;
            let mut buf = [0u8; 172];

            {
                let _g = TzGuard::set("America/New_York");
                assert_eq!(fill_tzi_at(buf.as_mut_ptr(), jan_2026), TIME_ZONE_ID_STANDARD);
                let (bias, sn, sd, sb, dn, dd, db) = read_tzi_fields(buf.as_ptr());
                assert_eq!((bias, sb, db), (300, 0, -60));
                assert_eq!((sn.as_str(), dn.as_str()), ("EST", "EDT"));
                // 2nd Sunday of March at 02:00 -> DST; 1st Sunday of November at 02:00 -> Standard.
                assert_eq!(st_tuple(dd), (0, 3, 0, 2, 2, 0, 0, 0));
                assert_eq!(st_tuple(sd), (0, 11, 0, 1, 2, 0, 0, 0));

                assert_eq!(fill_tzi_at(buf.as_mut_ptr(), jul_2026), TIME_ZONE_ID_DAYLIGHT);
            }

            {
                let _g = TzGuard::set("Europe/Berlin");
                assert_eq!(fill_tzi_at(buf.as_mut_ptr(), jan_2026), TIME_ZONE_ID_STANDARD);
                let (bias, sn, sd, sb, dn, dd, db) = read_tzi_fields(buf.as_ptr());
                assert_eq!((bias, sb, db), (-60, 0, -60));
                assert_eq!((sn.as_str(), dn.as_str()), ("CET", "CEST"));
                // Last Sunday of March at 02:00 -> DST; last Sunday of October at 03:00 -> Standard.
                assert_eq!(st_tuple(dd), (0, 3, 0, 5, 2, 0, 0, 0));
                assert_eq!(st_tuple(sd), (0, 10, 0, 5, 3, 0, 0, 0));

                assert_eq!(fill_tzi_at(buf.as_mut_ptr(), jul_2026), TIME_ZONE_ID_DAYLIGHT);
            }

            {
                let _g = TzGuard::set("Australia/Sydney");
                // Southern hemisphere: January is DST, July is Standard.
                assert_eq!(fill_tzi_at(buf.as_mut_ptr(), jan_2026), TIME_ZONE_ID_DAYLIGHT);
                let (bias, sn, sd, sb, dn, dd, db) = read_tzi_fields(buf.as_ptr());
                assert_eq!((bias, sb, db), (-600, 0, -60));
                assert_eq!((sn.as_str(), dn.as_str()), ("AEST", "AEDT"));
                // 1st Sunday of April at 03:00 -> Standard; 1st Sunday of October at 02:00 -> DST.
                assert_eq!(st_tuple(sd), (0, 4, 0, 1, 3, 0, 0, 0));
                assert_eq!(st_tuple(dd), (0, 10, 0, 1, 2, 0, 0, 0));

                assert_eq!(fill_tzi_at(buf.as_mut_ptr(), jul_2026), TIME_ZONE_ID_STANDARD);
            }

            {
                let _g = TzGuard::set("UTC");
                assert_eq!(fill_tzi_at(buf.as_mut_ptr(), jan_2026), TIME_ZONE_ID_UNKNOWN);
                let (bias, sn, sd, sb, dn, dd, db) = read_tzi_fields(buf.as_ptr());
                assert_eq!((bias, sb, db), (0, 0, 0));
                assert_eq!((sn.as_str(), dn.as_str()), ("UTC", "UTC"));
                assert_eq!(st_tuple(sd), (0, 0, 0, 0, 0, 0, 0, 0));
                assert_eq!(st_tuple(dd), (0, 0, 0, 0, 0, 0, 0, 0));
            }

            {
                let _g = TzGuard::set("Asia/Tokyo");
                assert_eq!(fill_tzi_at(buf.as_mut_ptr(), jul_2026), TIME_ZONE_ID_UNKNOWN);
                let (bias, sn, sd, sb, dn, dd, db) = read_tzi_fields(buf.as_ptr());
                assert_eq!((bias, sb, db), (-540, 0, 0));
                assert_eq!((sn.as_str(), dn.as_str()), ("JST", "JST"));
                assert_eq!(st_tuple(sd), (0, 0, 0, 0, 0, 0, 0, 0));
                assert_eq!(st_tuple(dd), (0, 0, 0, 0, 0, 0, 0, 0));
            }
        }
    }
}
