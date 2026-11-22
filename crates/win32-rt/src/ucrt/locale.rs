//! CRT locale. Two locales are supported: the initial "C" locale and the
//! user locale `English_United States.utf8` (what `setlocale(LC_ALL, "")`
//! yields on a Windows box with the UTF-8 ACP). Per-thread locales and
//! `_locale_t` objects are accepted but all share the global state.

use crate::strings::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

static C_LOCALE: AtomicBool = AtomicBool::new(true);

pub fn c_locale_active() -> bool {
    C_LOCALE.load(Ordering::Relaxed)
}

/// Multibyte code page of the current CRT locale (0 = "C": bytes map 1:1).
pub fn mb_cp() -> u32 {
    if c_locale_active() {
        0
    } else {
        65001
    }
}

const USER_LOCALE: &str = "English_United States.utf8";

fn names() -> &'static [(*mut u8, *mut u16); 2] {
    struct S([(*mut u8, *mut u16); 2]);
    unsafe impl Sync for S {}
    unsafe impl Send for S {}
    static N: std::sync::OnceLock<S> = std::sync::OnceLock::new();
    &N.get_or_init(|| S([(leak_ansi_z("C"), leak_wide_z("C")), (leak_ansi_z(USER_LOCALE), leak_wide_z(USER_LOCALE))])).0
}

fn apply(name: &str) -> Option<bool> {
    let l = name.to_ascii_lowercase();
    if l == "c" || l == "posix" {
        Some(true)
    } else if l.is_empty()
        || l.starts_with("en")
        || l.starts_with("english")
        || l.ends_with(".utf8")
        || l.ends_with(".utf-8")
        || l == ".65001"
    {
        Some(false)
    } else {
        None
    }
}

pub unsafe extern "win64" fn setlocale(cat: i32, name: *const u8) -> *mut u8 {
    if !(0..=5).contains(&cat) {
        return std::ptr::null_mut();
    }
    if !name.is_null() {
        let n = from_ansi(name);
        match apply(&n) {
            Some(c) => {
                log::debug!("setlocale({cat}, {n:?}) -> {}", if c { "C" } else { USER_LOCALE });
                C_LOCALE.store(c, Ordering::Relaxed);
            }
            None => {
                stub!("setlocale({cat}, {n:?}): locale not available");
                return std::ptr::null_mut();
            }
        }
    }
    names()[!c_locale_active() as usize].0
}

pub unsafe extern "win64" fn _wsetlocale(cat: i32, name: *const u16) -> *mut u16 {
    let a = from_wide_opt(name).map(|s| leak_ansi_z(&s));
    let r = setlocale(cat, a.unwrap_or(std::ptr::null_mut()));
    if r.is_null() {
        return std::ptr::null_mut();
    }
    names()[!c_locale_active() as usize].1
}

#[repr(C)]
pub struct Lconv {
    decimal_point: *const u8,
    thousands_sep: *const u8,
    grouping: *const u8,
    int_curr_symbol: *const u8,
    currency_symbol: *const u8,
    mon_decimal_point: *const u8,
    mon_thousands_sep: *const u8,
    mon_grouping: *const u8,
    positive_sign: *const u8,
    negative_sign: *const u8,
    int_frac_digits: u8,
    frac_digits: u8,
    p_cs_precedes: u8,
    p_sep_by_space: u8,
    n_cs_precedes: u8,
    n_sep_by_space: u8,
    p_sign_posn: u8,
    n_sign_posn: u8,
    w_decimal_point: *const u16,
    w_thousands_sep: *const u16,
    w_int_curr_symbol: *const u16,
    w_currency_symbol: *const u16,
    w_mon_decimal_point: *const u16,
    w_mon_thousands_sep: *const u16,
    w_positive_sign: *const u16,
    w_negative_sign: *const u16,
}
unsafe impl Sync for Lconv {}
unsafe impl Send for Lconv {}

fn lconv_c() -> &'static Lconv {
    static L: std::sync::OnceLock<Lconv> = std::sync::OnceLock::new();
    L.get_or_init(|| {
        let e = leak_ansi_z("");
        let we = leak_wide_z("");
        Lconv {
            decimal_point: leak_ansi_z("."),
            thousands_sep: e,
            grouping: e,
            int_curr_symbol: e,
            currency_symbol: e,
            mon_decimal_point: e,
            mon_thousands_sep: e,
            mon_grouping: e,
            positive_sign: e,
            negative_sign: e,
            int_frac_digits: 127,
            frac_digits: 127,
            p_cs_precedes: 127,
            p_sep_by_space: 127,
            n_cs_precedes: 127,
            n_sep_by_space: 127,
            p_sign_posn: 127,
            n_sign_posn: 127,
            w_decimal_point: leak_wide_z("."),
            w_thousands_sep: we,
            w_int_curr_symbol: we,
            w_currency_symbol: we,
            w_mon_decimal_point: we,
            w_mon_thousands_sep: we,
            w_positive_sign: we,
            w_negative_sign: we,
        }
    })
}

fn lconv_user() -> &'static Lconv {
    static L: std::sync::OnceLock<Lconv> = std::sync::OnceLock::new();
    L.get_or_init(|| Lconv {
        decimal_point: leak_ansi_z("."),
        thousands_sep: leak_ansi_z(","),
        grouping: leak_ansi_z("\x03"),
        int_curr_symbol: leak_ansi_z("USD"),
        currency_symbol: leak_ansi_z("$"),
        mon_decimal_point: leak_ansi_z("."),
        mon_thousands_sep: leak_ansi_z(","),
        mon_grouping: leak_ansi_z("\x03"),
        positive_sign: leak_ansi_z(""),
        negative_sign: leak_ansi_z("-"),
        int_frac_digits: 2,
        frac_digits: 2,
        p_cs_precedes: 1,
        p_sep_by_space: 0,
        n_cs_precedes: 1,
        n_sep_by_space: 0,
        p_sign_posn: 3,
        n_sign_posn: 0,
        w_decimal_point: leak_wide_z("."),
        w_thousands_sep: leak_wide_z(","),
        w_int_curr_symbol: leak_wide_z("USD"),
        w_currency_symbol: leak_wide_z("$"),
        w_mon_decimal_point: leak_wide_z("."),
        w_mon_thousands_sep: leak_wide_z(","),
        w_positive_sign: leak_wide_z(""),
        w_negative_sign: leak_wide_z("-"),
    })
}

pub unsafe extern "win64" fn localeconv() -> *const Lconv {
    if c_locale_active() {
        lconv_c()
    } else {
        lconv_user()
    }
}

pub unsafe extern "win64" fn ___lc_codepage_func() -> u32 {
    mb_cp()
}
pub unsafe extern "win64" fn ___lc_collate_cp_func() -> u32 {
    mb_cp()
}
pub unsafe extern "win64" fn ___mb_cur_max_func() -> i32 {
    if c_locale_active() {
        1
    } else {
        4
    }
}
pub unsafe extern "win64" fn ___mb_cur_max_l_func(_l: *mut c_void) -> i32 {
    ___mb_cur_max_func()
}

pub unsafe extern "win64" fn ___lc_locale_name_func() -> *const *const u16 {
    struct S([usize; 6]);
    static C: S = S([0; 6]);
    static U: std::sync::OnceLock<S> = std::sync::OnceLock::new();
    if c_locale_active() {
        C.0.as_ptr() as _
    } else {
        U.get_or_init(|| {
            let p = leak_wide_z("en-US") as usize;
            S([p; 6])
        })
        .0
        .as_ptr() as _
    }
}

static THREAD_LOCALE_MODE: AtomicI32 = AtomicI32::new(2); // _DISABLE_PER_THREAD_LOCALE

pub unsafe extern "win64" fn _configthreadlocale(flag: i32) -> i32 {
    let old = THREAD_LOCALE_MODE.load(Ordering::Relaxed);
    if flag == 1 || flag == 2 {
        THREAD_LOCALE_MODE.store(flag, Ordering::Relaxed);
    } else if flag != 0 && flag != -1 {
        return -1;
    }
    old
}
#[inline(never)]
pub unsafe extern "win64" fn _lock_locales() {}
#[inline(never)]
pub unsafe extern "win64" fn _unlock_locales() {}

/// `_locale_t` handles: a small heap object remembering which locale it is.
#[repr(C)]
struct LocaleObj {
    is_c: bool,
}

pub unsafe extern "win64" fn _create_locale(_cat: i32, name: *const u8) -> *mut c_void {
    match apply(&from_ansi(name)) {
        Some(c) => Box::into_raw(Box::new(LocaleObj { is_c: c })) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}
pub unsafe extern "win64" fn _wcreate_locale(_cat: i32, name: *const u16) -> *mut c_void {
    match apply(&from_wide(name)) {
        Some(c) => Box::into_raw(Box::new(LocaleObj { is_c: c })) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}
pub unsafe extern "win64" fn _free_locale(l: *mut c_void) {
    if !l.is_null() {
        drop(Box::from_raw(l as *mut LocaleObj));
    }
}
pub unsafe extern "win64" fn _get_current_locale() -> *mut c_void {
    Box::into_raw(Box::new(LocaleObj { is_c: c_locale_active() })) as *mut c_void
}
pub unsafe extern "win64" fn __acrt_get_qualified_locale(_a: *const c_void, _b: *mut c_void, _c: *mut c_void) -> i32 {
    0
}
pub unsafe extern "win64" fn _getmbcp() -> i32 {
    mb_cp() as i32
}
pub unsafe extern "win64" fn _setmbcp(cp: i32) -> i32 {
    if cp == 65001 || cp == -3 || cp == -4 || cp == 0 {
        0
    } else {
        stub!("_setmbcp({cp}) unsupported");
        -1
    }
}
