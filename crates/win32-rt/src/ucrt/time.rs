//! ucrt time: `time_t` (64-bit) conversions, `strftime`, timezone globals,
//! and the `_Gettnames`/`_Getdays`/`_Getmonths` locale tables used by
//! msvcp140's time facets. Locale data is the "C" locale.
//!
//! Local time uses `localtime_r` (reentrant) — never `localtime()`, whose
//! TSD key aliases a Windows TEB slot under gs.

use crate::strings::{cstr_slice, wide_slice};
use std::cell::RefCell;
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, Ordering};

/// Windows `struct tm`.
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
}

const DAYS_ABBR: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const DAYS: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
const MON_ABBR: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
const MONS: [&str; 12] =
    ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

const MAX_TIME64: i64 = 32535215999; // 3000-12-31 23:59:59 UTC

fn host_tm(t: i64, local: bool) -> Option<libc::tm> {
    unsafe {
        let tt: libc::time_t = t;
        let mut tm: libc::tm = std::mem::zeroed();
        let r = if local { libc::localtime_r(&tt, &mut tm) } else { libc::gmtime_r(&tt, &mut tm) };
        if r.is_null() {
            None
        } else {
            Some(tm)
        }
    }
}

fn from_host(h: &libc::tm) -> Tm {
    Tm {
        tm_sec: h.tm_sec,
        tm_min: h.tm_min,
        tm_hour: h.tm_hour,
        tm_mday: h.tm_mday,
        tm_mon: h.tm_mon,
        tm_year: h.tm_year,
        tm_wday: h.tm_wday,
        tm_yday: h.tm_yday,
        tm_isdst: (h.tm_isdst > 0) as i32,
    }
}

fn to_host(t: &Tm) -> libc::tm {
    let mut h: libc::tm = unsafe { std::mem::zeroed() };
    h.tm_sec = t.tm_sec;
    h.tm_min = t.tm_min;
    h.tm_hour = t.tm_hour;
    h.tm_mday = t.tm_mday;
    h.tm_mon = t.tm_mon;
    h.tm_year = t.tm_year;
    h.tm_isdst = t.tm_isdst;
    h
}

thread_local! {
    static TM_BUF: RefCell<Box<Tm>> = RefCell::new(Box::default());
}

fn tls_tm(v: Tm) -> *mut Tm {
    TM_BUF.with(|b| {
        let mut b = b.borrow_mut();
        **b = v;
        &mut **b as *mut Tm
    })
}

fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

pub unsafe extern "win64" fn _time64(out: *mut i64) -> i64 {
    let t = now();
    if !out.is_null() {
        *out = t;
    }
    t
}
pub unsafe extern "win64" fn _time32(out: *mut i32) -> i32 {
    let t = now() as i32;
    if !out.is_null() {
        *out = t;
    }
    t
}

fn valid_t(t: i64) -> bool {
    (0..=MAX_TIME64).contains(&t)
}

pub unsafe extern "win64" fn _gmtime64_s(out: *mut Tm, t: *const i64) -> i32 {
    if out.is_null() || t.is_null() {
        super::errno::set(super::errno::EINVAL);
        return super::errno::EINVAL;
    }
    if !valid_t(*t) {
        *out = Tm { tm_sec: -1, tm_min: -1, tm_hour: -1, tm_mday: -1, tm_mon: -1, tm_year: -1, tm_wday: -1, tm_yday: -1, tm_isdst: -1 };
        super::errno::set(super::errno::EINVAL);
        return super::errno::EINVAL;
    }
    *out = from_host(&host_tm(*t, false).unwrap());
    0
}
pub unsafe extern "win64" fn _gmtime64(t: *const i64) -> *mut Tm {
    let mut tm = Tm::default();
    if _gmtime64_s(&mut tm, t) != 0 {
        return std::ptr::null_mut();
    }
    tls_tm(tm)
}
pub unsafe extern "win64" fn _localtime64_s(out: *mut Tm, t: *const i64) -> i32 {
    if out.is_null() || t.is_null() {
        super::errno::set(super::errno::EINVAL);
        return super::errno::EINVAL;
    }
    if !valid_t(*t) {
        super::errno::set(super::errno::EINVAL);
        return super::errno::EINVAL;
    }
    match host_tm(*t, true) {
        Some(h) => {
            *out = from_host(&h);
            0
        }
        None => super::errno::EINVAL,
    }
}
pub unsafe extern "win64" fn _localtime64(t: *const i64) -> *mut Tm {
    let mut tm = Tm::default();
    if _localtime64_s(&mut tm, t) != 0 {
        return std::ptr::null_mut();
    }
    tls_tm(tm)
}
pub unsafe extern "win64" fn _gmtime32(t: *const i32) -> *mut Tm {
    let v = *t as i64;
    _gmtime64(&v)
}
pub unsafe extern "win64" fn _localtime32(t: *const i32) -> *mut Tm {
    let v = *t as i64;
    _localtime64(&v)
}

pub unsafe extern "win64" fn _mktime64(tm: *mut Tm) -> i64 {
    if tm.is_null() {
        super::errno::set(super::errno::EINVAL);
        return -1;
    }
    let mut h = to_host(&*tm);
    let t = libc::mktime(&mut h);
    if t < 0 || t > MAX_TIME64 {
        super::errno::set(super::errno::EINVAL);
        return -1;
    }
    *tm = from_host(&h);
    t
}
pub unsafe extern "win64" fn _mktime32(tm: *mut Tm) -> i32 {
    let t = _mktime64(tm);
    if t > i32::MAX as i64 {
        return -1;
    }
    t as i32
}
pub unsafe extern "win64" fn _mkgmtime64(tm: *mut Tm) -> i64 {
    if tm.is_null() {
        super::errno::set(super::errno::EINVAL);
        return -1;
    }
    let mut h = to_host(&*tm);
    let t = libc::timegm(&mut h);
    if t < 0 || t > MAX_TIME64 {
        return -1;
    }
    *tm = from_host(&h);
    (*tm).tm_isdst = 0;
    t
}
pub unsafe extern "win64" fn _mkgmtime32(tm: *mut Tm) -> i32 {
    _mkgmtime64(tm) as i32
}

pub extern "win64" fn _difftime64(a: i64, b: i64) -> f64 {
    (a - b) as f64
}
pub extern "win64" fn _difftime32(a: i32, b: i32) -> f64 {
    (a as i64 - b as i64) as f64
}

/// `clock()`: milliseconds of wall time since process start (Windows semantics).
pub extern "win64" fn clock() -> i32 {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    START.get_or_init(std::time::Instant::now).elapsed().as_millis() as i32
}

/// `struct __timeb64 { __time64_t time; unsigned short millitm; short timezone; short dstflag; }`
#[repr(C)]
pub struct Timeb64 {
    time: i64,
    millitm: u16,
    timezone: i16,
    dstflag: i16,
}

pub unsafe extern "win64" fn _ftime64_s(tb: *mut Timeb64) -> i32 {
    if tb.is_null() {
        return super::errno::EINVAL;
    }
    let d = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    _tzset();
    let h = host_tm(d.as_secs() as i64, true);
    *tb = Timeb64 {
        time: d.as_secs() as i64,
        millitm: d.subsec_millis() as u16,
        timezone: (TIMEZONE.load(Ordering::Relaxed) / 60) as i16,
        dstflag: h.map(|h| (h.tm_isdst > 0) as i16).unwrap_or(0),
    };
    0
}
pub unsafe extern "win64" fn _ftime64(tb: *mut Timeb64) {
    _ftime64_s(tb);
}

// ---------------------------------------------------------------------------
// Timezone globals
// ---------------------------------------------------------------------------

static DAYLIGHT: AtomicI32 = AtomicI32::new(0);
static TIMEZONE: AtomicI32 = AtomicI32::new(0);
static DSTBIAS: AtomicI32 = AtomicI32::new(-3600);
struct TzNames(std::cell::UnsafeCell<[*mut u8; 2]>);
unsafe impl Sync for TzNames {}
static TZNAME: TzNames = TzNames(std::cell::UnsafeCell::new([std::ptr::null_mut(); 2]));

pub unsafe extern "win64" fn _tzset() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let t = now();
        // Probe January and July to find standard vs daylight offsets.
        let year_start = t - (t % (365 * 86400));
        let jan = host_tm(year_start + 15 * 86400, true);
        let jul = host_tm(year_start + 196 * 86400, true);
        let (std_tm, dst_tm) = match (jan, jul) {
            (Some(a), Some(b)) if a.tm_gmtoff <= b.tm_gmtoff => (a, b),
            (Some(a), Some(b)) => (b, a),
            _ => return,
        };
        TIMEZONE.store(-(std_tm.tm_gmtoff as i32), Ordering::Relaxed);
        DAYLIGHT.store((dst_tm.tm_gmtoff != std_tm.tm_gmtoff) as i32, Ordering::Relaxed);
        DSTBIAS.store(-((dst_tm.tm_gmtoff - std_tm.tm_gmtoff) as i32), Ordering::Relaxed);
        let name = |h: &libc::tm| {
            if h.tm_zone.is_null() {
                "UTC".to_string()
            } else {
                std::ffi::CStr::from_ptr(h.tm_zone).to_string_lossy().into_owned()
            }
        };
        let names = &mut *TZNAME.0.get();
        names[0] = crate::strings::leak_ansi_z(&name(&std_tm));
        names[1] = crate::strings::leak_ansi_z(&name(&dst_tm));
    });
}

pub unsafe extern "win64" fn __daylight() -> *mut i32 {
    _tzset();
    DAYLIGHT.as_ptr()
}
pub unsafe extern "win64" fn __timezone() -> *mut i32 {
    _tzset();
    TIMEZONE.as_ptr()
}
pub unsafe extern "win64" fn __dstbias() -> *mut i32 {
    _tzset();
    DSTBIAS.as_ptr()
}
pub unsafe extern "win64" fn __tzname() -> *mut *mut u8 {
    _tzset();
    TZNAME.0.get() as *mut *mut u8
}
pub unsafe extern "win64" fn _get_timezone(p: *mut i32) -> i32 {
    _tzset();
    *p = TIMEZONE.load(Ordering::Relaxed);
    0
}
pub unsafe extern "win64" fn _get_daylight(p: *mut i32) -> i32 {
    _tzset();
    *p = DAYLIGHT.load(Ordering::Relaxed);
    0
}
pub unsafe extern "win64" fn _get_dstbias(p: *mut i32) -> i32 {
    _tzset();
    *p = DSTBIAS.load(Ordering::Relaxed);
    0
}

// ---------------------------------------------------------------------------
// strftime
// ---------------------------------------------------------------------------

fn iso_week(tm: &Tm) -> (i32, i32) {
    // ISO 8601 week number and year.
    let wday = (tm.tm_wday + 6) % 7; // Monday = 0
    let mut year = tm.tm_year + 1900;
    let mut week = (tm.tm_yday - wday + 10) / 7;
    let is_leap = |y: i32| (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    if week < 1 {
        year -= 1;
        let dec31_yday = if is_leap(year) { 365 } else { 364 };
        let dec31_wday = (wday - tm.tm_yday - 1).rem_euclid(7);
        week = (dec31_yday - dec31_wday + 10) / 7;
    } else {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if week == 53 && (days_in_year - tm.tm_yday) <= (3 - wday).max(0) {
            week = 1;
            year += 1;
        }
    }
    (week, year)
}

/// Format per C-locale strftime. Returns `None` for invalid input.
pub fn strftime_str(fmt: &[u32], tm: &Tm) -> Option<String> {
    let mut out = String::new();
    let mut i = 0;
    let wd = tm.tm_wday.clamp(0, 6) as usize;
    let mo = tm.tm_mon.clamp(0, 11) as usize;
    while i < fmt.len() {
        let c = fmt[i];
        if c != '%' as u32 {
            out.push(char::from_u32(c).unwrap_or('?'));
            i += 1;
            continue;
        }
        i += 1;
        let mut alt = false;
        if i < fmt.len() && fmt[i] == '#' as u32 {
            alt = true;
            i += 1;
        }
        if i >= fmt.len() {
            return None;
        }
        let spec = char::from_u32(fmt[i]).unwrap_or('?');
        i += 1;
        let num = |v: i32, w: usize| if alt { format!("{v}") } else { format!("{v:0w$}") };
        let h12 = if tm.tm_hour % 12 == 0 { 12 } else { tm.tm_hour % 12 };
        let s = match spec {
            'a' => DAYS_ABBR[wd].to_string(),
            'A' => DAYS[wd].to_string(),
            'b' | 'h' => MON_ABBR[mo].to_string(),
            'B' => MONS[mo].to_string(),
            'c' => {
                if alt {
                    format!("{}, {} {:02}, {} {:02}:{:02}:{:02}", DAYS[wd], MONS[mo], tm.tm_mday, tm.tm_year + 1900, tm.tm_hour, tm.tm_min, tm.tm_sec)
                } else {
                    format!("{:02}/{:02}/{:02} {:02}:{:02}:{:02}", tm.tm_mon + 1, tm.tm_mday, (tm.tm_year + 1900) % 100, tm.tm_hour, tm.tm_min, tm.tm_sec)
                }
            }
            'C' => num((tm.tm_year + 1900) / 100, 2),
            'd' => num(tm.tm_mday, 2),
            'D' => format!("{:02}/{:02}/{:02}", tm.tm_mon + 1, tm.tm_mday, (tm.tm_year + 1900) % 100),
            'e' => format!("{:2}", tm.tm_mday),
            'F' => format!("{:04}-{:02}-{:02}", tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday),
            'g' => num(iso_week(tm).1 % 100, 2),
            'G' => format!("{}", iso_week(tm).1),
            'H' => num(tm.tm_hour, 2),
            'I' => num(h12, 2),
            'j' => num(tm.tm_yday + 1, 3),
            'm' => num(tm.tm_mon + 1, 2),
            'M' => num(tm.tm_min, 2),
            'n' => "\n".into(),
            'p' => if tm.tm_hour < 12 { "AM" } else { "PM" }.into(),
            'r' => format!("{:02}:{:02}:{:02} {}", h12, tm.tm_min, tm.tm_sec, if tm.tm_hour < 12 { "AM" } else { "PM" }),
            'R' => format!("{:02}:{:02}", tm.tm_hour, tm.tm_min),
            'S' => num(tm.tm_sec, 2),
            't' => "\t".into(),
            'T' | 'X' => format!("{:02}:{:02}:{:02}", tm.tm_hour, tm.tm_min, tm.tm_sec),
            'u' => format!("{}", if tm.tm_wday == 0 { 7 } else { tm.tm_wday }),
            'U' => num((tm.tm_yday + 7 - tm.tm_wday) / 7, 2),
            'V' => num(iso_week(tm).0, 2),
            'w' => format!("{}", tm.tm_wday),
            'W' => num((tm.tm_yday + 7 - (tm.tm_wday + 6) % 7) / 7, 2),
            'x' => {
                if alt {
                    format!("{}, {} {:02}, {}", DAYS[wd], MONS[mo], tm.tm_mday, tm.tm_year + 1900)
                } else {
                    format!("{:02}/{:02}/{:02}", tm.tm_mon + 1, tm.tm_mday, (tm.tm_year + 1900) % 100)
                }
            }
            'y' => num((tm.tm_year + 1900) % 100, 2),
            'Y' => format!("{}", tm.tm_year + 1900),
            'z' => {
                let off = -unsafe {
                    _tzset();
                    TIMEZONE.load(Ordering::Relaxed)
                } + if tm.tm_isdst > 0 { -DSTBIAS.load(Ordering::Relaxed) } else { 0 };
                format!("{}{:02}{:02}", if off < 0 { '-' } else { '+' }, off.abs() / 3600, (off.abs() % 3600) / 60)
            }
            'Z' => unsafe {
                _tzset();
                let p = (*TZNAME.0.get())[(tm.tm_isdst > 0) as usize];
                if p.is_null() {
                    String::new()
                } else {
                    String::from_utf8_lossy(cstr_slice(p)).into_owned()
                }
            },
            '%' => "%".into(),
            _ => return None,
        };
        out.push_str(&s);
    }
    Some(out)
}

unsafe fn strftime_impl(buf: *mut u8, max: usize, fmt: *const u8, tm: *const Tm) -> usize {
    if buf.is_null() || fmt.is_null() || tm.is_null() || max == 0 {
        super::errno::set(super::errno::EINVAL);
        return 0;
    }
    let f: Vec<u32> = cstr_slice(fmt).iter().map(|&b| b as u32).collect();
    let Some(s) = strftime_str(&f, &*tm) else {
        *buf = 0;
        super::errno::set(super::errno::EINVAL);
        super::runtime::invalid_parameter("strftime: bad format");
        return 0;
    };
    if s.len() >= max {
        *buf = 0;
        super::errno::set(super::errno::ERANGE);
        return 0;
    }
    std::ptr::copy_nonoverlapping(s.as_ptr(), buf, s.len());
    *buf.add(s.len()) = 0;
    s.len()
}

unsafe fn wcsftime_impl(buf: *mut u16, max: usize, fmt: *const u16, tm: *const Tm) -> usize {
    if buf.is_null() || fmt.is_null() || tm.is_null() || max == 0 {
        super::errno::set(super::errno::EINVAL);
        return 0;
    }
    let f: Vec<u32> = wide_slice(fmt).iter().map(|&c| c as u32).collect();
    let Some(s) = strftime_str(&f, &*tm) else {
        *buf = 0;
        super::errno::set(super::errno::EINVAL);
        return 0;
    };
    let w: Vec<u16> = s.encode_utf16().collect();
    if w.len() >= max {
        *buf = 0;
        super::errno::set(super::errno::ERANGE);
        return 0;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
    *buf.add(w.len()) = 0;
    w.len()
}

pub unsafe extern "win64" fn strftime(buf: *mut u8, max: usize, fmt: *const u8, tm: *const Tm) -> usize {
    strftime_impl(buf, max, fmt, tm)
}
pub unsafe extern "win64" fn _strftime_l(buf: *mut u8, max: usize, fmt: *const u8, tm: *const Tm, _l: *mut c_void) -> usize {
    strftime_impl(buf, max, fmt, tm)
}
pub unsafe extern "win64" fn _Strftime(buf: *mut u8, max: usize, fmt: *const u8, tm: *const Tm, _names: *mut c_void) -> usize {
    strftime_impl(buf, max, fmt, tm)
}
pub unsafe extern "win64" fn wcsftime(buf: *mut u16, max: usize, fmt: *const u16, tm: *const Tm) -> usize {
    wcsftime_impl(buf, max, fmt, tm)
}
pub unsafe extern "win64" fn _wcsftime_l(buf: *mut u16, max: usize, fmt: *const u16, tm: *const Tm, _l: *mut c_void) -> usize {
    wcsftime_impl(buf, max, fmt, tm)
}
pub unsafe extern "win64" fn _Wcsftime(buf: *mut u16, max: usize, fmt: *const u16, tm: *const Tm, _names: *mut c_void) -> usize {
    wcsftime_impl(buf, max, fmt, tm)
}

pub unsafe extern "win64" fn asctime_s(buf: *mut u8, size: usize, tm: *const Tm) -> i32 {
    if buf.is_null() || tm.is_null() || size < 26 {
        return super::errno::EINVAL;
    }
    let t = &*tm;
    let s = format!(
        "{} {} {:2} {:02}:{:02}:{:02} {}\n",
        DAYS_ABBR[t.tm_wday.clamp(0, 6) as usize],
        MON_ABBR[t.tm_mon.clamp(0, 11) as usize],
        t.tm_mday,
        t.tm_hour,
        t.tm_min,
        t.tm_sec,
        t.tm_year + 1900
    );
    crate::strings::write_ansi_trunc(&s, buf, size);
    0
}

pub unsafe extern "win64" fn asctime(tm: *const Tm) -> *mut u8 {
    thread_local! {
        static BUF: std::cell::UnsafeCell<[u8; 32]> = const { std::cell::UnsafeCell::new([0u8; 32]) };
    }
    if tm.is_null() {
        super::errno::set(super::errno::EINVAL);
        return std::ptr::null_mut();
    }
    BUF.with(|b| {
        let ptr = b.get() as *mut u8;
        if asctime_s(ptr, 32, tm) != 0 {
            std::ptr::null_mut()
        } else {
            ptr
        }
    })
}

// ---------------------------------------------------------------------------
// Locale time tables (msvcp140 time_get / time_put)
// ---------------------------------------------------------------------------

fn malloc_copy(bytes: &[u8]) -> *mut u8 {
    unsafe {
        let p = crate::heap::alloc(bytes.len(), false) as *mut u8;
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
        p
    }
}

fn days_string() -> String {
    let mut s = String::new();
    for d in 0..7 {
        s.push_str(&format!(":{}:{}", DAYS_ABBR[d], DAYS[d]));
    }
    s
}
fn months_string() -> String {
    let mut s = String::new();
    for m in 0..12 {
        s.push_str(&format!(":{}:{}", MON_ABBR[m], MONS[m]));
    }
    s
}

pub unsafe extern "win64" fn _Getdays() -> *mut u8 {
    let mut b = days_string().into_bytes();
    b.push(0);
    malloc_copy(&b)
}
pub unsafe extern "win64" fn _Getmonths() -> *mut u8 {
    let mut b = months_string().into_bytes();
    b.push(0);
    malloc_copy(&b)
}
pub unsafe extern "win64" fn _W_Getdays() -> *mut u16 {
    let w = crate::strings::to_wide_z(&days_string());
    malloc_copy(std::slice::from_raw_parts(w.as_ptr() as *const u8, w.len() * 2)) as *mut u16
}
pub unsafe extern "win64" fn _W_Getmonths() -> *mut u16 {
    let w = crate::strings::to_wide_z(&months_string());
    malloc_copy(std::slice::from_raw_parts(w.as_ptr() as *const u8, w.len() * 2)) as *mut u16
}

/// `__crt_lc_time_data` (C locale). The caller frees the block with `free`;
/// the strings it points to are static.
#[repr(C)]
struct LcTimeData {
    wday_abbr: [*const u8; 7],
    wday: [*const u8; 7],
    month_abbr: [*const u8; 12],
    month: [*const u8; 12],
    ampm: [*const u8; 2],
    ww_sdatefmt: *const u8,
    ww_ldatefmt: *const u8,
    ww_timefmt: *const u8,
    ww_caltype: i32,
    refcount: i32,
    w_wday_abbr: [*const u16; 7],
    w_wday: [*const u16; 7],
    w_month_abbr: [*const u16; 12],
    w_month: [*const u16; 12],
    w_ampm: [*const u16; 2],
    w_ww_sdatefmt: *const u16,
    w_ww_ldatefmt: *const u16,
    w_ww_timefmt: *const u16,
    w_ww_locale_name: *const u16,
}

pub unsafe extern "win64" fn _Gettnames() -> *mut c_void {
    let a = |s: &str| crate::strings::leak_ansi_z(s) as *const u8;
    let w = |s: &str| crate::strings::leak_wide_z(s) as *const u16;
    let d = LcTimeData {
        wday_abbr: DAYS_ABBR.map(a),
        wday: DAYS.map(a),
        month_abbr: MON_ABBR.map(a),
        month: MONS.map(a),
        ampm: ["AM", "PM"].map(a),
        ww_sdatefmt: a("MM/dd/yy"),
        ww_ldatefmt: a("dddd, MMMM dd, yyyy"),
        ww_timefmt: a("HH:mm:ss"),
        ww_caltype: 1,
        refcount: 0,
        w_wday_abbr: DAYS_ABBR.map(w),
        w_wday: DAYS.map(w),
        w_month_abbr: MON_ABBR.map(w),
        w_month: MONS.map(w),
        w_ampm: ["AM", "PM"].map(w),
        w_ww_sdatefmt: w("MM/dd/yy"),
        w_ww_ldatefmt: w("dddd, MMMM dd, yyyy"),
        w_ww_timefmt: w("HH:mm:ss"),
        w_ww_locale_name: w(""),
    };
    let p = crate::heap::alloc(std::mem::size_of::<LcTimeData>(), false) as *mut LcTimeData;
    std::ptr::write(p, d);
    p as *mut c_void
}
pub unsafe extern "win64" fn _W_Gettnames() -> *mut c_void {
    _Gettnames()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gm_and_strftime() {
        unsafe {
            let t: i64 = 1_700_000_000; // 2023-11-14 22:13:20 UTC (Tuesday)
            let tm = *_gmtime64(&t);
            assert_eq!((tm.tm_year, tm.tm_mon, tm.tm_mday, tm.tm_hour, tm.tm_wday), (123, 10, 14, 22, 2));
            let mut tm2 = tm;
            assert_eq!(_mkgmtime64(&mut tm2), t);
            let f: Vec<u32> = "%Y-%m-%d %H:%M:%S %a %b %j %p %#d".chars().map(|c| c as u32).collect();
            assert_eq!(strftime_str(&f, &tm).unwrap(), "2023-11-14 22:13:20 Tue Nov 318 PM 14");
            let mut buf = [0u8; 8];
            assert_eq!(strftime(buf.as_mut_ptr(), 8, b"%Y-%m-%d\0".as_ptr(), &tm), 0, "too small -> 0");
            assert_eq!(std::mem::size_of::<LcTimeData>(), 704);
        }
    }
}
