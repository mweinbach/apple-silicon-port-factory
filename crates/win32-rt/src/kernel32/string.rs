//! Code pages, string comparison/mapping and locale information (NLS),
//! backed natively by Apple's system ICU (`/usr/lib/libicucore.A.dylib`).
//!
//! Default process ANSI/OEM code page is UTF-8 (65001, matching Windows 10 1903+
//! "Use Unicode UTF-8 for worldwide language support"), while explicit Windows
//! code pages (1250–1258, 932, 936, 949, 950, 437, 850, 866, 28591, etc.) and
//! arbitrary Windows LCIDs / BCP-47 locale names are resolved dynamically via
//! `libicucore`.

use crate::strings::*;
use crate::teb;
use crate::types::*;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::ffi::{c_char, c_void, CStr, CString};
use std::sync::OnceLock;

fn set_err(e: u32) {
    teb::set_last_error(e);
}

pub const CP_ACP: u32 = 0;
pub const CP_OEMCP: u32 = 1;
pub const CP_MACCP: u32 = 2;
pub const CP_THREAD_ACP: u32 = 3;
pub const CP_UTF7: u32 = 65000;
pub const CP_UTF8: u32 = 65001;

const MB_ERR_INVALID_CHARS: u32 = 0x8;
const WC_ERR_INVALID_CHARS: u32 = 0x80;

pub const LOCALE_NAME: &str = "en-US";
pub const LCID_EN_US: u32 = 0x0409;

// ---------------------------------------------------------------------------
// Dynamic Apple libicucore.A.dylib loader (macOS + iOS)
// ---------------------------------------------------------------------------

mod icu {
    use super::*;

    pub const UDAT_FULL: i32 = 0;
    pub const UDAT_MEDIUM: i32 = 2;
    pub const UDAT_SHORT: i32 = 3;
    pub const UDAT_NONE: i32 = -1;

    pub const UDAT_ERAS: i32 = 0;
    pub const UDAT_MONTHS: i32 = 1;
    pub const UDAT_SHORT_MONTHS: i32 = 2;
    pub const UDAT_WEEKDAYS: i32 = 3;
    pub const UDAT_SHORT_WEEKDAYS: i32 = 4;
    pub const UDAT_AM_PMS: i32 = 5;
    pub const UDAT_SHORTER_WEEKDAYS: i32 = 20;

    pub const UNUM_DECIMAL: i32 = 1;
    pub const UNUM_CURRENCY: i32 = 2;

    pub const UNUM_DECIMAL_SEPARATOR_SYMBOL: i32 = 0;
    pub const UNUM_GROUPING_SEPARATOR_SYMBOL: i32 = 1;
    pub const UNUM_PERCENT_SYMBOL: i32 = 3;
    pub const UNUM_ZERO_DIGIT_SYMBOL: i32 = 4;
    pub const UNUM_MINUS_SIGN_SYMBOL: i32 = 6;
    pub const UNUM_CURRENCY_SYMBOL: i32 = 8;
    pub const UNUM_INTL_CURRENCY_SYMBOL: i32 = 9;
    pub const UNUM_MONETARY_SEPARATOR_SYMBOL: i32 = 10;
    pub const UNUM_PERMILL_SYMBOL: i32 = 12;
    pub const UNUM_INFINITY_SYMBOL: i32 = 14;
    pub const UNUM_NAN_SYMBOL: i32 = 15;
    pub const UNUM_MONETARY_GROUPING_SEPARATOR_SYMBOL: i32 = 17;

    pub const UNUM_MAX_FRACTION_DIGITS: i32 = 6;
    pub const UNUM_GROUPING_SIZE: i32 = 10;
    pub const UNUM_SECONDARY_GROUPING_SIZE: i32 = 15;

    pub const UCAL_FIRST_DAY_OF_WEEK: i32 = 1;

    pub const UCOL_ALTERNATE_HANDLING: i32 = 1;
    pub const UCOL_CASE_FIRST: i32 = 2;
    pub const UCOL_CASE_LEVEL: i32 = 3;
    pub const UCOL_STRENGTH: i32 = 5;
    pub const UCOL_NUMERIC_COLLATION: i32 = 7;

    pub const UCOL_PRIMARY: i32 = 0;
    pub const UCOL_SECONDARY: i32 = 1;
    pub const UCOL_TERTIARY: i32 = 2;
    pub const UCOL_ON: i32 = 17;
    pub const UCOL_SHIFTED: i32 = 20;
    pub const UCOL_LOWER_FIRST: i32 = 24;

    pub const ULOC_LAYOUT_RTL: i32 = 1;
    pub const UMS_US: i32 = 1;

    pub type UConverterToUCallback = unsafe extern "C" fn(
        *const c_void,
        *mut c_void,
        *const c_char,
        i32,
        i32,
        *mut i32,
    );

    pub type UConverterFromUCallback = unsafe extern "C" fn(
        *const c_void,
        *mut c_void,
        *const u16,
        i32,
        i32,
        i32,
        *mut i32,
    );

    pub struct IcuNls {
        pub uloc_to_language_tag: unsafe extern "C" fn(*const c_char, *mut c_char, i32, i8, *mut i32) -> i32,
        pub uloc_for_language_tag: unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32, *mut i32) -> i32,
        pub uloc_get_lcid: unsafe extern "C" fn(*const c_char) -> u32,
        pub uloc_get_locale_for_lcid: unsafe extern "C" fn(u32, *mut c_char, i32, *mut i32) -> i32,
        pub uloc_get_language: unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32) -> i32,
        pub uloc_get_country: unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32) -> i32,
        pub uloc_get_script: unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32) -> i32,
        pub uloc_get_parent: unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32) -> i32,
        pub uloc_get_iso3_language: unsafe extern "C" fn(*const c_char) -> *const c_char,
        pub uloc_get_iso3_country: unsafe extern "C" fn(*const c_char) -> *const c_char,
        pub uloc_get_display_name: unsafe extern "C" fn(*const c_char, *const c_char, *mut u16, i32, *mut i32) -> i32,
        pub uloc_get_display_language: unsafe extern "C" fn(*const c_char, *const c_char, *mut u16, i32, *mut i32) -> i32,
        pub uloc_get_display_country: unsafe extern "C" fn(*const c_char, *const c_char, *mut u16, i32, *mut i32) -> i32,
        pub uloc_get_char_orientation: unsafe extern "C" fn(*const c_char, *mut i32) -> i32,
        pub uloc_count_available: unsafe extern "C" fn() -> i32,
        pub uloc_get_available: unsafe extern "C" fn(i32) -> *const c_char,
        pub ulocdata_get_measurement_system: unsafe extern "C" fn(*const c_char, *mut i32) -> i32,
        pub unum_open: unsafe extern "C" fn(i32, *const u16, i32, *const c_char, *mut c_void, *mut i32) -> *mut c_void,
        pub unum_get_symbol: unsafe extern "C" fn(*const c_void, i32, *mut u16, i32, *mut i32) -> i32,
        pub unum_get_attribute: unsafe extern "C" fn(*const c_void, i32) -> i32,
        pub unum_to_pattern: unsafe extern "C" fn(*const c_void, i8, *mut u16, i32, *mut i32) -> i32,
        pub unum_close: unsafe extern "C" fn(*mut c_void),
        pub udat_open: unsafe extern "C" fn(i32, i32, *const c_char, *const u16, i32, *const u16, i32, *mut i32) -> *mut c_void,
        pub udat_to_pattern: unsafe extern "C" fn(*const c_void, i8, *mut u16, i32, *mut i32) -> i32,
        pub udat_get_symbols: unsafe extern "C" fn(*const c_void, i32, i32, *mut u16, i32, *mut i32) -> i32,
        pub udat_close: unsafe extern "C" fn(*mut c_void),
        pub udatpg_open: unsafe extern "C" fn(*const c_char, *mut i32) -> *mut c_void,
        pub udatpg_get_best_pattern: unsafe extern "C" fn(*mut c_void, *const u16, i32, *mut u16, i32, *mut i32) -> i32,
        pub udatpg_close: unsafe extern "C" fn(*mut c_void),
        pub ucal_open: unsafe extern "C" fn(*const u16, i32, *const c_char, i32, *mut i32) -> *mut c_void,
        pub ucal_get_attribute: unsafe extern "C" fn(*const c_void, i32) -> i32,
        pub ucal_close: unsafe extern "C" fn(*mut c_void),
        pub ucurr_for_locale: unsafe extern "C" fn(*const c_char, *mut u16, i32, *mut i32) -> i32,
        pub ucurr_get_name: unsafe extern "C" fn(*const u16, *const c_char, i32, *mut i8, *mut i32, *mut i32) -> *const u16,
        pub ucol_open: unsafe extern "C" fn(*const c_char, *mut i32) -> *mut c_void,
        pub ucol_set_attribute: unsafe extern "C" fn(*mut c_void, i32, i32, *mut i32),
        pub ucol_strcoll: unsafe extern "C" fn(*const c_void, *const u16, i32, *const u16, i32) -> i32,
        pub ucol_get_sort_key: unsafe extern "C" fn(*const c_void, *const u16, i32, *mut u8, i32) -> i32,
        pub ucol_close: unsafe extern "C" fn(*mut c_void),
        pub u_str_to_lower: unsafe extern "C" fn(*mut u16, i32, *const u16, i32, *const c_char, *mut i32) -> i32,
        pub u_str_to_upper: unsafe extern "C" fn(*mut u16, i32, *const u16, i32, *const c_char, *mut i32) -> i32,
        pub u_str_to_title: unsafe extern "C" fn(*mut u16, i32, *const u16, i32, *mut c_void, *const c_char, *mut i32) -> i32,
        pub utrans_open_u: unsafe extern "C" fn(*const u16, i32, i32, *const u16, i32, *mut c_void, *mut i32) -> *mut c_void,
        pub utrans_trans_uchars: unsafe extern "C" fn(*const c_void, *mut u16, *mut i32, i32, i32, *mut i32, *mut i32),
        pub utrans_close: unsafe extern "C" fn(*mut c_void),
        pub ucnv_open: unsafe extern "C" fn(*const c_char, *mut i32) -> *mut c_void,
        pub ucnv_to_uchars: unsafe extern "C" fn(*mut c_void, *mut u16, i32, *const c_char, i32, *mut i32) -> i32,
        pub ucnv_from_uchars: unsafe extern "C" fn(*mut c_void, *mut c_char, i32, *const u16, i32, *mut i32) -> i32,
        pub ucnv_set_to_u_callback: unsafe extern "C" fn(
            *mut c_void,
            UConverterToUCallback,
            *const c_void,
            *mut Option<UConverterToUCallback>,
            *mut *const c_void,
            *mut i32,
        ),
        pub ucnv_to_u_cb_stop: UConverterToUCallback,
        pub ucnv_set_subst_chars: unsafe extern "C" fn(*mut c_void, *const c_char, i8, *mut i32),
        pub ucnv_set_from_u_callback: unsafe extern "C" fn(
            *mut c_void,
            UConverterFromUCallback,
            *const c_void,
            *mut Option<UConverterFromUCallback>,
            *mut *const c_void,
            *mut i32,
        ),
        pub ucnv_from_u_cb_substitute: UConverterFromUCallback,
        pub ucnv_get_max_char_size: unsafe extern "C" fn(*const c_void) -> i8,
        pub ucnv_close: unsafe extern "C" fn(*mut c_void),
    }

    pub fn api() -> Option<&'static IcuNls> {
        static ICU: OnceLock<Option<IcuNls>> = OnceLock::new();
        ICU.get_or_init(|| unsafe {
            let mut handle: *mut c_void = std::ptr::null_mut();
            for p in [c"/usr/lib/libicucore.A.dylib", c"libicucore.A.dylib", c"libicucore.dylib"] {
                let h = libc::dlopen(p.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
                if !h.is_null() {
                    handle = h;
                    break;
                }
            }
            if handle.is_null() {
                return None;
            }
            macro_rules! sym {
                ($name:expr, $ty:ty) => {{
                    let p = libc::dlsym(handle, $name.as_ptr());
                    if p.is_null() {
                        return None;
                    }
                    std::mem::transmute::<*mut c_void, $ty>(p)
                }};
            }
            Some(IcuNls {
                uloc_to_language_tag: sym!(c"uloc_toLanguageTag", unsafe extern "C" fn(*const c_char, *mut c_char, i32, i8, *mut i32) -> i32),
                uloc_for_language_tag: sym!(c"uloc_forLanguageTag", unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32, *mut i32) -> i32),
                uloc_get_lcid: sym!(c"uloc_getLCID", unsafe extern "C" fn(*const c_char) -> u32),
                uloc_get_locale_for_lcid: sym!(c"uloc_getLocaleForLCID", unsafe extern "C" fn(u32, *mut c_char, i32, *mut i32) -> i32),
                uloc_get_language: sym!(c"uloc_getLanguage", unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32) -> i32),
                uloc_get_country: sym!(c"uloc_getCountry", unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32) -> i32),
                uloc_get_script: sym!(c"uloc_getScript", unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32) -> i32),
                uloc_get_parent: sym!(c"uloc_getParent", unsafe extern "C" fn(*const c_char, *mut c_char, i32, *mut i32) -> i32),
                uloc_get_iso3_language: sym!(c"uloc_getISO3Language", unsafe extern "C" fn(*const c_char) -> *const c_char),
                uloc_get_iso3_country: sym!(c"uloc_getISO3Country", unsafe extern "C" fn(*const c_char) -> *const c_char),
                uloc_get_display_name: sym!(c"uloc_getDisplayName", unsafe extern "C" fn(*const c_char, *const c_char, *mut u16, i32, *mut i32) -> i32),
                uloc_get_display_language: sym!(c"uloc_getDisplayLanguage", unsafe extern "C" fn(*const c_char, *const c_char, *mut u16, i32, *mut i32) -> i32),
                uloc_get_display_country: sym!(c"uloc_getDisplayCountry", unsafe extern "C" fn(*const c_char, *const c_char, *mut u16, i32, *mut i32) -> i32),
                uloc_get_char_orientation: sym!(c"uloc_getCharacterOrientation", unsafe extern "C" fn(*const c_char, *mut i32) -> i32),
                uloc_count_available: sym!(c"uloc_countAvailable", unsafe extern "C" fn() -> i32),
                uloc_get_available: sym!(c"uloc_getAvailable", unsafe extern "C" fn(i32) -> *const c_char),
                ulocdata_get_measurement_system: sym!(c"ulocdata_getMeasurementSystem", unsafe extern "C" fn(*const c_char, *mut i32) -> i32),
                unum_open: sym!(c"unum_open", unsafe extern "C" fn(i32, *const u16, i32, *const c_char, *mut c_void, *mut i32) -> *mut c_void),
                unum_get_symbol: sym!(c"unum_getSymbol", unsafe extern "C" fn(*const c_void, i32, *mut u16, i32, *mut i32) -> i32),
                unum_get_attribute: sym!(c"unum_getAttribute", unsafe extern "C" fn(*const c_void, i32) -> i32),
                unum_to_pattern: sym!(c"unum_toPattern", unsafe extern "C" fn(*const c_void, i8, *mut u16, i32, *mut i32) -> i32),
                unum_close: sym!(c"unum_close", unsafe extern "C" fn(*mut c_void)),
                udat_open: sym!(c"udat_open", unsafe extern "C" fn(i32, i32, *const c_char, *const u16, i32, *const u16, i32, *mut i32) -> *mut c_void),
                udat_to_pattern: sym!(c"udat_toPattern", unsafe extern "C" fn(*const c_void, i8, *mut u16, i32, *mut i32) -> i32),
                udat_get_symbols: sym!(c"udat_getSymbols", unsafe extern "C" fn(*const c_void, i32, i32, *mut u16, i32, *mut i32) -> i32),
                udat_close: sym!(c"udat_close", unsafe extern "C" fn(*mut c_void)),
                udatpg_open: sym!(c"udatpg_open", unsafe extern "C" fn(*const c_char, *mut i32) -> *mut c_void),
                udatpg_get_best_pattern: sym!(c"udatpg_getBestPattern", unsafe extern "C" fn(*mut c_void, *const u16, i32, *mut u16, i32, *mut i32) -> i32),
                udatpg_close: sym!(c"udatpg_close", unsafe extern "C" fn(*mut c_void)),
                ucal_open: sym!(c"ucal_open", unsafe extern "C" fn(*const u16, i32, *const c_char, i32, *mut i32) -> *mut c_void),
                ucal_get_attribute: sym!(c"ucal_getAttribute", unsafe extern "C" fn(*const c_void, i32) -> i32),
                ucal_close: sym!(c"ucal_close", unsafe extern "C" fn(*mut c_void)),
                ucurr_for_locale: sym!(c"ucurr_forLocale", unsafe extern "C" fn(*const c_char, *mut u16, i32, *mut i32) -> i32),
                ucurr_get_name: sym!(c"ucurr_getName", unsafe extern "C" fn(*const u16, *const c_char, i32, *mut i8, *mut i32, *mut i32) -> *const u16),
                ucol_open: sym!(c"ucol_open", unsafe extern "C" fn(*const c_char, *mut i32) -> *mut c_void),
                ucol_set_attribute: sym!(c"ucol_setAttribute", unsafe extern "C" fn(*mut c_void, i32, i32, *mut i32)),
                ucol_strcoll: sym!(c"ucol_strcoll", unsafe extern "C" fn(*const c_void, *const u16, i32, *const u16, i32) -> i32),
                ucol_get_sort_key: sym!(c"ucol_getSortKey", unsafe extern "C" fn(*const c_void, *const u16, i32, *mut u8, i32) -> i32),
                ucol_close: sym!(c"ucol_close", unsafe extern "C" fn(*mut c_void)),
                u_str_to_lower: sym!(c"u_strToLower", unsafe extern "C" fn(*mut u16, i32, *const u16, i32, *const c_char, *mut i32) -> i32),
                u_str_to_upper: sym!(c"u_strToUpper", unsafe extern "C" fn(*mut u16, i32, *const u16, i32, *const c_char, *mut i32) -> i32),
                u_str_to_title: sym!(c"u_strToTitle", unsafe extern "C" fn(*mut u16, i32, *const u16, i32, *mut c_void, *const c_char, *mut i32) -> i32),
                utrans_open_u: sym!(c"utrans_openU", unsafe extern "C" fn(*const u16, i32, i32, *const u16, i32, *mut c_void, *mut i32) -> *mut c_void),
                utrans_trans_uchars: sym!(c"utrans_transUChars", unsafe extern "C" fn(*const c_void, *mut u16, *mut i32, i32, i32, *mut i32, *mut i32)),
                utrans_close: sym!(c"utrans_close", unsafe extern "C" fn(*mut c_void)),
                ucnv_open: sym!(c"ucnv_open", unsafe extern "C" fn(*const c_char, *mut i32) -> *mut c_void),
                ucnv_to_uchars: sym!(c"ucnv_toUChars", unsafe extern "C" fn(*mut c_void, *mut u16, i32, *const c_char, i32, *mut i32) -> i32),
                ucnv_from_uchars: sym!(c"ucnv_fromUChars", unsafe extern "C" fn(*mut c_void, *mut c_char, i32, *const u16, i32, *mut i32) -> i32),
                ucnv_set_to_u_callback: sym!(
                    c"ucnv_setToUCallBack",
                    unsafe extern "C" fn(
                        *mut c_void,
                        UConverterToUCallback,
                        *const c_void,
                        *mut Option<UConverterToUCallback>,
                        *mut *const c_void,
                        *mut i32,
                    )
                ),
                ucnv_to_u_cb_stop: sym!(c"UCNV_TO_U_CALLBACK_STOP", UConverterToUCallback),
                ucnv_set_subst_chars: sym!(c"ucnv_setSubstChars", unsafe extern "C" fn(*mut c_void, *const c_char, i8, *mut i32)),
                ucnv_set_from_u_callback: sym!(
                    c"ucnv_setFromUCallBack",
                    unsafe extern "C" fn(
                        *mut c_void,
                        UConverterFromUCallback,
                        *const c_void,
                        *mut Option<UConverterFromUCallback>,
                        *mut *const c_void,
                        *mut i32,
                    )
                ),
                ucnv_from_u_cb_substitute: sym!(c"UCNV_FROM_U_CALLBACK_SUBSTITUTE", UConverterFromUCallback),
                ucnv_get_max_char_size: sym!(c"ucnv_getMaxCharSize", unsafe extern "C" fn(*const c_void) -> i8),
                ucnv_close: sym!(c"ucnv_close", unsafe extern "C" fn(*mut c_void)),
            })
        })
        .as_ref()
    }

    pub fn c_str(s: &str) -> CString {
        CString::new(s).unwrap_or_else(|_| CString::new("en_US").unwrap())
    }

    pub unsafe fn call_u16_str(f: impl FnOnce(*mut u16, i32, *mut i32) -> i32) -> Option<String> {
        let mut buf = [0u16; 128];
        let mut st = 0i32;
        let n = f(buf.as_mut_ptr(), buf.len() as i32, &mut st);
        if st <= 0 && n >= 0 {
            let len = (n as usize).min(buf.len());
            return Some(String::from_utf16_lossy(&buf[..len]));
        }
        None
    }

    pub unsafe fn call_ascii_str(f: impl FnOnce(*mut c_char, i32, *mut i32) -> i32) -> Option<String> {
        let mut buf = [0 as c_char; 128];
        let mut st = 0i32;
        let n = f(buf.as_mut_ptr(), buf.len() as i32, &mut st);
        if st <= 0 && n >= 0 {
            let len = (n as usize).min(buf.len() - 1);
            buf[len] = 0;
            return Some(CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned());
        }
        None
    }

    pub fn transliterate(id: &str, input: &[u16]) -> Option<Vec<u16>> {
        let a = api()?;
        let id_u16: Vec<u16> = id.encode_utf16().collect();
        unsafe {
            let mut st = 0i32;
            let tr = (a.utrans_open_u)(
                id_u16.as_ptr(),
                id_u16.len() as i32,
                0, // UTRANS_FORWARD
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                &mut st,
            );
            if tr.is_null() || st > 0 {
                return None;
            }
            let cap = (input.len() * 4 + 32).max(64);
            let mut buf = vec![0u16; cap];
            buf[..input.len()].copy_from_slice(input);
            let mut text_len = input.len() as i32;
            let mut limit = input.len() as i32;
            st = 0;
            (a.utrans_trans_uchars)(tr, buf.as_mut_ptr(), &mut text_len, cap as i32, 0, &mut limit, &mut st);
            (a.utrans_close)(tr);
            if st <= 0 && text_len >= 0 {
                buf.truncate(text_len as usize);
                Some(buf)
            } else {
                None
            }
        }
    }
}

/// Map a Windows code page number to its ICU converter name.
pub(crate) fn icu_cp_name(cp: u32) -> Option<&'static str> {
    match cp {
        65001 => Some("UTF-8"),
        65000 => Some("UTF-7"),
        1200 => Some("UTF-16LE"),
        1201 => Some("UTF-16BE"),
        20127 => Some("US-ASCII"),
        1250 => Some("windows-1250"),
        1251 => Some("windows-1251"),
        1252 => Some("windows-1252"),
        1253 => Some("windows-1253"),
        1254 => Some("windows-1254"),
        1255 => Some("windows-1255"),
        1256 => Some("windows-1256"),
        1257 => Some("windows-1257"),
        1258 => Some("windows-1258"),
        932 => Some("windows-932"),
        936 => Some("windows-936"),
        949 => Some("windows-949"),
        950 => Some("windows-950"),
        874 => Some("windows-874"),
        437 => Some("IBM437"),
        737 => Some("IBM737"),
        775 => Some("IBM775"),
        850 => Some("IBM850"),
        852 => Some("IBM852"),
        855 => Some("IBM855"),
        857 => Some("IBM857"),
        860 => Some("IBM860"),
        861 => Some("IBM861"),
        862 => Some("IBM862"),
        863 => Some("IBM863"),
        865 => Some("IBM865"),
        866 => Some("IBM866"),
        869 => Some("IBM869"),
        28591 => Some("ISO-8859-1"),
        28592 => Some("ISO-8859-2"),
        28593 => Some("ISO-8859-3"),
        28594 => Some("ISO-8859-4"),
        28595 => Some("ISO-8859-5"),
        28596 => Some("ISO-8859-6"),
        28597 => Some("ISO-8859-7"),
        28598 => Some("ISO-8859-8"),
        28599 => Some("ISO-8859-9"),
        28605 => Some("ISO-8859-15"),
        _ => None,
    }
}

pub fn resolve_cp(cp: u32) -> u32 {
    match cp {
        CP_ACP | CP_OEMCP | CP_MACCP | CP_THREAD_ACP => ACP,
        c => c,
    }
}

pub fn valid_cp(cp: u32) -> bool {
    let c = resolve_cp(cp);
    icu_cp_name(c).is_some()
}

pub unsafe extern "win64" fn GetACP() -> u32 {
    ACP
}
pub unsafe extern "win64" fn GetOEMCP() -> u32 {
    ACP
}
pub unsafe extern "win64" fn IsValidCodePage(cp: u32) -> BOOL {
    valid_cp(cp) as BOOL
}
pub unsafe extern "win64" fn IsDBCSLeadByte(_b: u8) -> BOOL {
    FALSE
}
pub unsafe extern "win64" fn IsDBCSLeadByteEx(cp: u32, b: u8) -> BOOL {
    let is_lead = match resolve_cp(cp) {
        932 => (0x81..=0x9F).contains(&b) || (0xE0..=0xFC).contains(&b),
        936 | 949 | 950 => (0x81..=0xFE).contains(&b),
        _ => false,
    };
    is_lead as BOOL
}

pub(crate) fn cp_max_char_size(cp: u32) -> u32 {
    match resolve_cp(cp) {
        65001 => 4,
        65000 => 5,
        932 | 936 | 949 | 950 => 2,
        c => {
            if let (Some(a), Some(name)) = (icu::api(), icu_cp_name(c)) {
                let cn = icu::c_str(name);
                unsafe {
                    let mut st = 0i32;
                    let cnv = (a.ucnv_open)(cn.as_ptr(), &mut st);
                    if !cnv.is_null() && st <= 0 {
                        let m = (a.ucnv_get_max_char_size)(cnv).max(1) as u32;
                        (a.ucnv_close)(cnv);
                        return m;
                    }
                }
            }
            1
        }
    }
}

pub unsafe extern "win64" fn GetCPInfo(cp: u32, info: *mut u8) -> BOOL {
    if !valid_cp(cp) || info.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let c = resolve_cp(cp);
    std::ptr::write_bytes(info, 0, 20);
    (info as *mut u32).write_unaligned(cp_max_char_size(c));
    *info.add(4) = b'?';
    // Populate LeadByte ranges for DBCS Windows codepages.
    match c {
        932 => {
            *info.add(6) = 0x81;
            *info.add(7) = 0x9F;
            *info.add(8) = 0xE0;
            *info.add(9) = 0xFC;
        }
        936 | 949 | 950 => {
            *info.add(6) = 0x81;
            *info.add(7) = 0xFE;
        }
        _ => {}
    }
    TRUE
}

pub unsafe extern "win64" fn GetCPInfoExW(cp: u32, _flags: u32, info: *mut u8) -> BOOL {
    if GetCPInfo(cp, info) == 0 {
        return FALSE;
    }
    let c = resolve_cp(cp);
    (info.add(18) as *mut u16).write_unaligned(if c == 65001 { 0xFFFD } else { b'?' as u16 });
    (info.add(20) as *mut u32).write_unaligned(c);
    let name = match c {
        65001 => "65001 (UTF-8)",
        1250 => "1250  (ANSI - Central Europe)",
        1251 => "1251  (ANSI - Cyrillic)",
        1252 => "1252  (ANSI - Latin I)",
        1253 => "1253  (ANSI - Greek)",
        1254 => "1254  (ANSI - Turkish)",
        1255 => "1255  (ANSI - Hebrew)",
        1256 => "1256  (ANSI - Arabic)",
        1257 => "1257  (ANSI - Baltic)",
        1258 => "1258  (ANSI - Viet Nam)",
        932 => "932   (ANSI/OEM - Japanese Shift-JIS)",
        936 => "936   (ANSI/OEM - Simplified Chinese GBK)",
        949 => "949   (ANSI/OEM - Korean)",
        950 => "950   (ANSI/OEM - Traditional Chinese Big5)",
        437 => "437   (OEM - United States)",
        850 => "850   (OEM - Multilingual Latin I)",
        866 => "866   (OEM - Russian)",
        28591 => "28591 (ISO 8859-1 Latin I)",
        _ => icu_cp_name(c).unwrap_or(""),
    };
    write_wide_trunc(name, info.add(24) as *mut u16, 260);
    TRUE
}

pub(crate) fn decode_cp_to_utf16(cp: u32, src: &[u8]) -> Result<Vec<u16>, u32> {
    decode_cp_to_utf16_strict(cp, src, true)
}

pub(crate) fn decode_cp_to_utf16_strict(cp: u32, src: &[u8], strict: bool) -> Result<Vec<u16>, u32> {
    if let (Some(a), Some(name)) = (icu::api(), icu_cp_name(cp)) {
        let cn = icu::c_str(name);
        unsafe {
            let mut st = 0i32;
            let cnv = (a.ucnv_open)(cn.as_ptr(), &mut st);
            if !cnv.is_null() && st <= 0 {
                if strict {
                    let mut cb_st = 0i32;
                    (a.ucnv_set_to_u_callback)(
                        cnv,
                        a.ucnv_to_u_cb_stop,
                        std::ptr::null(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        &mut cb_st,
                    );
                }
                let mut out = vec![0u16; src.len() * 2 + 8];
                st = 0;
                let n = (a.ucnv_to_uchars)(
                    cnv,
                    out.as_mut_ptr(),
                    out.len() as i32,
                    src.as_ptr() as *const c_char,
                    src.len() as i32,
                    &mut st,
                );
                if st == 15 && n > 0 {
                    // U_BUFFER_OVERFLOW_ERROR
                    out.resize(n as usize + 1, 0);
                    st = 0;
                    let n2 = (a.ucnv_to_uchars)(
                        cnv,
                        out.as_mut_ptr(),
                        out.len() as i32,
                        src.as_ptr() as *const c_char,
                        src.len() as i32,
                        &mut st,
                    );
                    (a.ucnv_close)(cnv);
                    if st <= 0 && n2 >= 0 {
                        out.truncate(n2 as usize);
                        return Ok(out);
                    }
                    return Err(err::ERROR_NO_UNICODE_TRANSLATION);
                }
                (a.ucnv_close)(cnv);
                if st <= 0 && n >= 0 {
                    out.truncate(n as usize);
                    return Ok(out);
                }
                return Err(err::ERROR_NO_UNICODE_TRANSLATION);
            }
        }
    }
    let s = decode_cp(cp, src).map_err(|_| err::ERROR_NO_UNICODE_TRANSLATION)?;
    let units: Vec<u16> = s.encode_utf16().collect();
    if strict && units.contains(&0xFFFD) {
        return Err(err::ERROR_NO_UNICODE_TRANSLATION);
    }
    Ok(units)
}

struct FromUTrackCtx {
    used: bool,
    fallback: icu::UConverterFromUCallback,
}

unsafe extern "C" fn track_from_u_subst_cb(
    context: *const c_void,
    args: *mut c_void,
    code_units: *const u16,
    length: i32,
    code_point: i32,
    reason: i32,
    err: *mut i32,
) {
    let ctx = &mut *(context as *mut FromUTrackCtx);
    // UCNV_UNASSIGNED (0), UCNV_ILLEGAL (1), UCNV_IRREGULAR (2)
    if (0..=2).contains(&reason) {
        ctx.used = true;
    }
    (ctx.fallback)(std::ptr::null(), args, code_units, length, code_point, reason, err);
}

pub(crate) fn encode_utf16_to_cp(cp: u32, src: &[u16], default: Option<u8>) -> Result<(Vec<u8>, bool), u32> {
    if let (Some(a), Some(name)) = (icu::api(), icu_cp_name(cp)) {
        let cn = icu::c_str(name);
        unsafe {
            let mut st = 0i32;
            let cnv = (a.ucnv_open)(cn.as_ptr(), &mut st);
            if !cnv.is_null() && st <= 0 {
                // Configure converter substitution character (Windows default '?' or caller-supplied
                // byte) and track every substitution via ICU's FromU callback instead of scanning
                // output bytes (DBCS converters like CP932/CP936 use 0xFC 0xFC or 0x3F by default).
                let rep = default.unwrap_or(b'?');
                let mut sub_st = 0i32;
                (a.ucnv_set_subst_chars)(cnv, &rep as *const u8 as *const c_char, 1, &mut sub_st);
                let mut track = FromUTrackCtx {
                    used: false,
                    fallback: a.ucnv_from_u_cb_substitute,
                };
                let mut cb_st = 0i32;
                (a.ucnv_set_from_u_callback)(
                    cnv,
                    track_from_u_subst_cb,
                    &mut track as *mut FromUTrackCtx as *const c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut cb_st,
                );
                let mut out = vec![0u8; src.len() * 4 + 8];
                st = 0;
                let n = (a.ucnv_from_uchars)(
                    cnv,
                    out.as_mut_ptr() as *mut c_char,
                    out.len() as i32,
                    src.as_ptr(),
                    src.len() as i32,
                    &mut st,
                );
                if st == 15 && n > 0 {
                    out.resize(n as usize + 1, 0);
                    track.used = false;
                    st = 0;
                    let n2 = (a.ucnv_from_uchars)(
                        cnv,
                        out.as_mut_ptr() as *mut c_char,
                        out.len() as i32,
                        src.as_ptr(),
                        src.len() as i32,
                        &mut st,
                    );
                    (a.ucnv_close)(cnv);
                    if st <= 0 && n2 >= 0 {
                        out.truncate(n2 as usize);
                        return Ok((out, track.used));
                    }
                    return Err(err::ERROR_NO_UNICODE_TRANSLATION);
                }
                (a.ucnv_close)(cnv);
                if st <= 0 && n >= 0 {
                    out.truncate(n as usize);
                    return Ok((out, track.used));
                }
            }
        }
    }
    let s = String::from_utf16_lossy(src);
    let mut used = false;
    let enc: Vec<u8> = s
        .chars()
        .map(|ch| {
            let e = encode_cp(cp, ch.encode_utf8(&mut [0u8; 4]));
            if e == b"?" && ch != '?' {
                used = true;
                default.unwrap_or(b'?')
            } else {
                e[0]
            }
        })
        .collect();
    Ok((enc, used))
}

// ---------------------------------------------------------------------------
// MultiByteToWideChar / WideCharToMultiByte
// ---------------------------------------------------------------------------

pub unsafe fn mb_to_wide(cp: u32, flags: u32, src: &[u8]) -> Result<Vec<u16>, u32> {
    let cp = resolve_cp(cp);
    match cp {
        65001 | 1200 => {
            let s = match std::str::from_utf8(src) {
                Ok(s) => Cow::Borrowed(s),
                Err(_) if flags & MB_ERR_INVALID_CHARS != 0 => return Err(err::ERROR_NO_UNICODE_TRANSLATION),
                Err(_) => String::from_utf8_lossy(src),
            };
            Ok(s.encode_utf16().collect())
        }
        c if valid_cp(c) => decode_cp_to_utf16_strict(c, src, (flags & MB_ERR_INVALID_CHARS) != 0),
        _ => Err(err::ERROR_INVALID_PARAMETER),
    }
}

pub unsafe extern "win64" fn MultiByteToWideChar(cp: u32, flags: u32, src: *const u8, n: i32, dst: *mut u16, cap: i32) -> i32 {
    if src.is_null() || n == 0 || cap < 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let bytes = if n < 0 {
        std::slice::from_raw_parts(src, strlen(src) + 1)
    } else {
        std::slice::from_raw_parts(src, n as usize)
    };
    let w = match mb_to_wide(cp, flags, bytes) {
        Ok(w) => w,
        Err(e) => {
            set_err(e);
            return 0;
        }
    };
    if cap == 0 {
        return w.len() as i32;
    }
    if w.len() > cap as usize {
        std::ptr::copy_nonoverlapping(w.as_ptr(), dst, cap as usize);
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), dst, w.len());
    w.len() as i32
}

pub unsafe fn wide_to_mb(cp: u32, flags: u32, src: &[u16], default: Option<u8>) -> Result<(Vec<u8>, bool), u32> {
    let cp = resolve_cp(cp);
    match cp {
        65001 => {
            let mut out = Vec::with_capacity(src.len());
            for r in char::decode_utf16(src.iter().copied()) {
                match r {
                    Ok(c) => {
                        let mut b = [0u8; 4];
                        out.extend_from_slice(c.encode_utf8(&mut b).as_bytes());
                    }
                    Err(_) if flags & WC_ERR_INVALID_CHARS != 0 => return Err(err::ERROR_NO_UNICODE_TRANSLATION),
                    Err(_) => out.extend_from_slice("\u{FFFD}".as_bytes()),
                }
            }
            Ok((out, false))
        }
        c if valid_cp(c) => encode_utf16_to_cp(c, src, default),
        _ => Err(err::ERROR_INVALID_PARAMETER),
    }
}

pub unsafe extern "win64" fn WideCharToMultiByte(
    cp: u32,
    flags: u32,
    src: *const u16,
    n: i32,
    dst: *mut u8,
    cap: i32,
    default_char: *const u8,
    used_default: *mut BOOL,
) -> i32 {
    if src.is_null() || n == 0 || cap < 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let w = if n < 0 {
        std::slice::from_raw_parts(src, wcslen(src) + 1)
    } else {
        std::slice::from_raw_parts(src, n as usize)
    };
    let def = if default_char.is_null() { None } else { Some(*default_char) };
    let (b, used) = match wide_to_mb(cp, flags, w, def) {
        Ok(r) => r,
        Err(e) => {
            set_err(e);
            return 0;
        }
    };
    if !used_default.is_null() {
        *used_default = used as BOOL;
    }
    if cap == 0 {
        return b.len() as i32;
    }
    if b.len() > cap as usize {
        std::ptr::copy_nonoverlapping(b.as_ptr(), dst, cap as usize);
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(b.as_ptr(), dst, b.len());
    b.len() as i32
}

// ---------------------------------------------------------------------------
// lstr*
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn lstrlenA(s: LPCSTR) -> i32 {
    if s.is_null() {
        0
    } else {
        strlen(s) as i32
    }
}
pub unsafe extern "win64" fn lstrlenW(s: LPCWSTR) -> i32 {
    if s.is_null() {
        0
    } else {
        wcslen(s) as i32
    }
}
pub unsafe extern "win64" fn lstrcpyW(d: LPWSTR, s: LPCWSTR) -> LPWSTR {
    let n = wcslen(s);
    std::ptr::copy(s, d, n + 1);
    d
}
pub unsafe extern "win64" fn lstrcpyA(d: LPSTR, s: LPCSTR) -> LPSTR {
    let n = strlen(s);
    std::ptr::copy(s, d, n + 1);
    d
}
pub unsafe extern "win64" fn lstrcpynW(d: LPWSTR, s: LPCWSTR, n: i32) -> LPWSTR {
    if n <= 0 {
        return d;
    }
    let mut i = 0;
    while i + 1 < n as usize && *s.add(i) != 0 {
        *d.add(i) = *s.add(i);
        i += 1;
    }
    *d.add(i) = 0;
    d
}
pub unsafe extern "win64" fn lstrcatW(d: LPWSTR, s: LPCWSTR) -> LPWSTR {
    let n = wcslen(d);
    lstrcpyW(d.add(n), s);
    d
}
pub unsafe extern "win64" fn lstrcmpW(a: LPCWSTR, b: LPCWSTR) -> i32 {
    CompareStringW(0x400, 0, a, -1, b, -1) - 2
}
pub unsafe extern "win64" fn lstrcmpiW(a: LPCWSTR, b: LPCWSTR) -> i32 {
    CompareStringW(0x400, 1, a, -1, b, -1) - 2
}
pub unsafe extern "win64" fn lstrcmpA(a: LPCSTR, b: LPCSTR) -> i32 {
    CompareStringA(0x400, 0, a, -1, b, -1) - 2
}
pub unsafe extern "win64" fn lstrcmpiA(a: LPCSTR, b: LPCSTR) -> i32 {
    CompareStringA(0x400, 1, a, -1, b, -1) - 2
}

// ---------------------------------------------------------------------------
// CompareString
// ---------------------------------------------------------------------------

const NORM_IGNORECASE: u32 = 0x1;
const NORM_IGNORENONSPACE: u32 = 0x2;
const NORM_IGNORESYMBOLS: u32 = 0x4;
const SORT_DIGITSASNUMBERS: u32 = 0x8;
const LINGUISTIC_IGNORECASE: u32 = 0x10;
const LINGUISTIC_IGNOREDIACRITIC: u32 = 0x20;
const SORT_STRINGSORT: u32 = 0x1000;
const NORM_LINGUISTIC_CASING: u32 = 0x0800_0000;

pub const CSTR_LESS_THAN: i32 = 1;
pub const CSTR_EQUAL: i32 = 2;
pub const CSTR_GREATER_THAN: i32 = 3;

unsafe fn wslice<'a>(p: *const u16, n: i32) -> &'a [u16] {
    if p.is_null() {
        &[]
    } else if n < 0 {
        std::slice::from_raw_parts(p, wcslen(p))
    } else {
        // Explicit lengths may still include a terminating NUL; Windows stops there.
        let s = std::slice::from_raw_parts(p, n as usize);
        match s.iter().position(|&c| c == 0) {
            Some(i) => &s[..i],
            None => s,
        }
    }
}

fn strip_diacritic(c: char) -> char {
    // Latin-1 / Latin Extended-A base letters (enough for typical game strings).
    const MAP: &[(&str, char)] = &[
        ("àáâãäåāăą", 'a'),
        ("ÀÁÂÃÄÅĀĂĄ", 'A'),
        ("çćĉċč", 'c'),
        ("ÇĆĈĊČ", 'C'),
        ("èéêëēĕėęě", 'e'),
        ("ÈÉÊËĒĔĖĘĚ", 'E'),
        ("ìíîïĩīĭįı", 'i'),
        ("ÌÍÎÏĨĪĬĮİ", 'I'),
        ("ñńņňŉ", 'n'),
        ("ÑŃŅŇ", 'N'),
        ("òóôõöøōŏő", 'o'),
        ("ÒÓÔÕÖØŌŎŐ", 'O'),
        ("ùúûüũūŭůűų", 'u'),
        ("ÙÚÛÜŨŪŬŮŰŲ", 'U'),
        ("ýÿŷ", 'y'),
        ("ÝŸŶ", 'Y'),
        ("śŝşš", 's'),
        ("ŚŜŞŠ", 'S'),
        ("źżž", 'z'),
        ("ŹŻŽ", 'Z'),
    ];
    for (set, base) in MAP {
        if set.contains(c) {
            return *base;
        }
    }
    c
}

/// Collation key for one char: (primary, case). Lowercase sorts first.
fn coll(c: char, flags: u32) -> (u32, u8) {
    let c = if flags & (NORM_IGNORENONSPACE | LINGUISTIC_IGNOREDIACRITIC) != 0 { strip_diacritic(c) } else { c };
    let lower = c.to_lowercase().next().unwrap_or(c);
    let case = if c.is_uppercase() { 1 } else { 0 };
    // Punctuation/symbols sort before digits before letters (like Windows).
    let class = if lower.is_alphabetic() {
        3
    } else if lower.is_numeric() {
        2
    } else if lower.is_whitespace() {
        0
    } else {
        1
    };
    ((class << 24) | (lower as u32 & 0xFF_FFFF), case)
}

pub fn compare_linguistic(a: &[u16], b: &[u16], flags: u32) -> Ordering {
    let ignore_case = flags & (NORM_IGNORECASE | LINGUISTIC_IGNORECASE) != 0;
    let filt = |s: &[u16]| -> Vec<char> {
        char::decode_utf16(s.iter().copied())
            .map(|r| r.unwrap_or('\u{FFFD}'))
            .filter(|c| {
                if flags & NORM_IGNORESYMBOLS != 0 && !c.is_alphanumeric() {
                    return false;
                }
                // Word sort ignores hyphens and apostrophes.
                if flags & SORT_STRINGSORT == 0 && (*c == '-' || *c == '\'') {
                    return false;
                }
                true
            })
            .collect()
    };
    let (ca, cb) = (filt(a), filt(b));
    if flags & SORT_DIGITSASNUMBERS != 0 {
        if let Some(o) = compare_digits_as_numbers(&ca, &cb, flags) {
            return o;
        }
    }
    let ka: Vec<(u32, u8)> = ca.iter().map(|&c| coll(c, flags)).collect();
    let kb: Vec<(u32, u8)> = cb.iter().map(|&c| coll(c, flags)).collect();
    let p = ka.iter().map(|k| k.0).cmp(kb.iter().map(|k| k.0));
    if p != Ordering::Equal || ignore_case {
        return p;
    }
    let c = ka.iter().map(|k| k.1).cmp(kb.iter().map(|k| k.1));
    if c != Ordering::Equal {
        return c;
    }
    // Strings that differ only in ignored chars (hyphens) are ordered by the raw form.
    if flags & SORT_STRINGSORT == 0 {
        return a.len().cmp(&b.len());
    }
    Ordering::Equal
}

fn compare_digits_as_numbers(a: &[char], b: &[char], flags: u32) -> Option<Ordering> {
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i].is_ascii_digit() && b[j].is_ascii_digit() {
            let si = i;
            while i < a.len() && a[i].is_ascii_digit() {
                i += 1;
            }
            let sj = j;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            let na: String = a[si..i].iter().collect::<String>().trim_start_matches('0').to_string();
            let nb: String = b[sj..j].iter().collect::<String>().trim_start_matches('0').to_string();
            let o = na.len().cmp(&nb.len()).then(na.cmp(&nb));
            if o != Ordering::Equal {
                return Some(o);
            }
        } else {
            let o = coll(a[i], flags | NORM_IGNORECASE).0.cmp(&coll(b[j], flags | NORM_IGNORECASE).0);
            if o != Ordering::Equal {
                return Some(o);
            }
            i += 1;
            j += 1;
        }
    }
    Some((a.len() - i).cmp(&(b.len() - j)))
}

fn ord_to_cstr(o: Ordering) -> i32 {
    match o {
        Ordering::Less => CSTR_LESS_THAN,
        Ordering::Equal => CSTR_EQUAL,
        Ordering::Greater => CSTR_GREATER_THAN,
    }
}

pub unsafe extern "win64" fn CompareStringEx(
    locale: LPCWSTR,
    flags: u32,
    a: LPCWSTR,
    na: i32,
    b: LPCWSTR,
    nb: i32,
    _ver: *mut c_void,
    _res: *mut c_void,
    _lparam: isize,
) -> i32 {
    if a.is_null() || b.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let Some(loc) = resolve_locale_name_w(locale) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let _ = NORM_LINGUISTIC_CASING;
    let icu_loc = if loc.invariant { "" } else { loc.icu_id.as_str() };
    ord_to_cstr(compare_for_locale(icu_loc, wslice(a, na), wslice(b, nb), flags))
}

pub unsafe extern "win64" fn CompareStringW(lcid: u32, flags: u32, a: LPCWSTR, na: i32, b: LPCWSTR, nb: i32) -> i32 {
    if a.is_null() || b.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let Some(loc) = resolve_lcid(lcid) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let icu_loc = if loc.invariant { "" } else { loc.icu_id.as_str() };
    ord_to_cstr(compare_for_locale(icu_loc, wslice(a, na), wslice(b, nb), flags))
}

pub unsafe extern "win64" fn CompareStringA(lcid: u32, flags: u32, a: LPCSTR, na: i32, b: LPCSTR, nb: i32) -> i32 {
    if a.is_null() || b.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let conv = |p: LPCSTR, n: i32| -> Vec<u16> {
        let s = if n < 0 { cstr_slice(p) } else { std::slice::from_raw_parts(p, n as usize) };
        String::from_utf8_lossy(s).encode_utf16().collect()
    };
    let (wa, wb) = (conv(a, na), conv(b, nb));
    CompareStringW(lcid, flags, wa.as_ptr(), wa.len() as i32, wb.as_ptr(), wb.len() as i32)
}

pub unsafe extern "win64" fn CompareStringOrdinal(a: LPCWSTR, na: i32, b: LPCWSTR, nb: i32, ignore_case: BOOL) -> i32 {
    if a.is_null() || b.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let sa = if na < 0 { wide_slice(a) } else { std::slice::from_raw_parts(a, na as usize) };
    let sb = if nb < 0 { wide_slice(b) } else { std::slice::from_raw_parts(b, nb as usize) };
    let o = if ignore_case != 0 {
        sa.iter().map(|&c| wupper(c)).cmp(sb.iter().map(|&c| wupper(c)))
    } else {
        sa.cmp(sb)
    };
    ord_to_cstr(o)
}

// ---------------------------------------------------------------------------
// LCMapString
// ---------------------------------------------------------------------------

const LCMAP_LOWERCASE: u32 = 0x0000_0100;
const LCMAP_UPPERCASE: u32 = 0x0000_0200;
const LCMAP_TITLECASE: u32 = 0x0000_0300;
const LCMAP_SORTKEY: u32 = 0x0000_0400;
const LCMAP_BYTEREV: u32 = 0x0000_0800;
const LCMAP_HASH: u32 = 0x0004_0000;
const LCMAP_SORTSHOWHYPHENS: u32 = 0x0008_0000;
const LCMAP_HIRAGANA: u32 = 0x0010_0000;
const LCMAP_KATAKANA: u32 = 0x0020_0000;
const LCMAP_HALFWIDTH: u32 = 0x0040_0000;
const LCMAP_FULLWIDTH: u32 = 0x0080_0000;
const LCMAP_LINGUISTIC_CASING: u32 = 0x0100_0000;
const LCMAP_SIMPLIFIED_CHINESE: u32 = 0x0200_0000;
const LCMAP_TRADITIONAL_CHINESE: u32 = 0x0400_0000;
const NORM_IGNOREKANATYPE: u32 = 0x0001_0000;
const NORM_IGNOREWIDTH: u32 = 0x0002_0000;

/// Shared ICU collator setup so `CompareStringEx`/`CompareStringW` and
/// `LCMapStringEx(..., LCMAP_SORTKEY, ...)` always produce identical ordering.
unsafe fn configure_ucollator(a: &icu::IcuNls, col: *mut c_void, flags: u32) {
    let ignore_case = (flags & (NORM_IGNORECASE | LINGUISTIC_IGNORECASE)) != 0;
    let ignore_diacritics = (flags & (NORM_IGNORENONSPACE | LINGUISTIC_IGNOREDIACRITIC)) != 0;
    let strength = if ignore_diacritics {
        icu::UCOL_PRIMARY
    } else if ignore_case {
        icu::UCOL_SECONDARY
    } else {
        icu::UCOL_TERTIARY
    };
    let mut ast = 0i32;
    (a.ucol_set_attribute)(col, icu::UCOL_STRENGTH, strength, &mut ast);
    if ignore_diacritics && !ignore_case {
        ast = 0;
        (a.ucol_set_attribute)(col, icu::UCOL_CASE_LEVEL, icu::UCOL_ON, &mut ast);
    }
    ast = 0;
    (a.ucol_set_attribute)(col, icu::UCOL_CASE_FIRST, icu::UCOL_LOWER_FIRST, &mut ast);
    if (flags & NORM_IGNORESYMBOLS) != 0 {
        ast = 0;
        (a.ucol_set_attribute)(col, icu::UCOL_ALTERNATE_HANDLING, icu::UCOL_SHIFTED, &mut ast);
    }
    if (flags & SORT_DIGITSASNUMBERS) != 0 {
        ast = 0;
        (a.ucol_set_attribute)(col, icu::UCOL_NUMERIC_COLLATION, icu::UCOL_ON, &mut ast);
    }
}

/// Windows word-sort special-weight extraction: strips hyphen `'-'` and apostrophe `'\''`
/// from `s` for primary/secondary/tertiary comparison and records `(position, weight)` entries
/// (`'\''` = `0x02`, `'-'` = `0x03`) for 4th-level tie-breaking and sort-key suffix encoding.
fn word_sort_strip(s: &[u16]) -> (Vec<u16>, Vec<(u32, u8)>) {
    let mut stripped = Vec::with_capacity(s.len());
    let mut special = Vec::new();
    for (idx, &u) in s.iter().enumerate() {
        if u == b'\'' as u16 {
            special.push((idx as u32, 0x02));
        } else if u == b'-' as u16 {
            special.push((idx as u32, 0x03));
        } else {
            stripped.push(u);
        }
    }
    (stripped, special)
}

/// Append Windows word-sort 4th-level special weights to a NUL-terminated ICU sort key using
/// strictly non-zero bytes (`0x02..=0xFF`, big-endian base-254 position + weight) after a
/// `0x01` level separator so `memcmp` matches `compare_for_locale` 1-to-1 and never embeds `0x00`.
fn append_word_sort_weights(key: &mut Vec<u8>, special: &[(u32, u8)]) {
    if special.is_empty() {
        return;
    }
    if key.last() == Some(&0) {
        key.pop();
    }
    key.push(0x01);
    for &(pos, w) in special {
        // Encode `pos` in 4 big-endian base-254 digits shifted by +2 (range 0x02..=0xFF).
        let d3 = (pos % 254) as u8 + 2;
        let q3 = pos / 254;
        let d2 = (q3 % 254) as u8 + 2;
        let q2 = q3 / 254;
        let d1 = (q2 % 254) as u8 + 2;
        let d0 = ((q2 / 254) % 254) as u8 + 2;
        key.extend_from_slice(&[d0, d1, d2, d3, w]);
    }
    key.push(0x00);
}

/// Locale-aware comparison via Apple ICU (`ucol_strcoll`), matching `sort_key_for_locale`
/// (including Windows word sort vs `SORT_STRINGSORT`), falling back to `compare_linguistic`
/// if ICU is unavailable.
fn compare_for_locale(icu_locale: &str, lhs: &[u16], rhs: &[u16], flags: u32) -> Ordering {
    let word_sort = (flags & SORT_STRINGSORT) == 0 && (flags & NORM_IGNORESYMBOLS) == 0;
    if let Some(a) = icu::api() {
        let cloc = icu::c_str(icu_locale);
        unsafe {
            let mut st = 0i32;
            let col = (a.ucol_open)(cloc.as_ptr(), &mut st);
            if !col.is_null() && st <= 0 {
                configure_ucollator(a, col, flags);
                let (ord, lhs_sw, rhs_sw) = if word_sort {
                    let (ls, lsw) = word_sort_strip(lhs);
                    let (rs, rsw) = word_sort_strip(rhs);
                    let r = (a.ucol_strcoll)(col, ls.as_ptr(), ls.len() as i32, rs.as_ptr(), rs.len() as i32);
                    (r, lsw, rsw)
                } else {
                    let r = (a.ucol_strcoll)(col, lhs.as_ptr(), lhs.len() as i32, rhs.as_ptr(), rhs.len() as i32);
                    (r, Vec::new(), Vec::new())
                };
                (a.ucol_close)(col);
                return match ord {
                    x if x < 0 => Ordering::Less,
                    x if x > 0 => Ordering::Greater,
                    _ if word_sort => lhs_sw.cmp(&rhs_sw),
                    _ => Ordering::Equal,
                };
            }
        }
    }
    compare_linguistic(lhs, rhs, flags)
}

/// Locale-aware sort key via Apple ICU (`ucol_getSortKey`), matching `compare_for_locale`
/// (including Windows word sort vs `SORT_STRINGSORT`), falling back to `sort_key` if ICU is unavailable.
fn sort_key_for_locale(icu_locale: &str, s: &[u16], flags: u32) -> Vec<u8> {
    let word_sort = (flags & SORT_STRINGSORT) == 0 && (flags & NORM_IGNORESYMBOLS) == 0;
    if let Some(a) = icu::api() {
        let cloc = icu::c_str(icu_locale);
        unsafe {
            let mut st = 0i32;
            let col = (a.ucol_open)(cloc.as_ptr(), &mut st);
            if !col.is_null() && st <= 0 {
                configure_ucollator(a, col, flags);
                let (stripped, special) = if word_sort {
                    word_sort_strip(s)
                } else {
                    (Vec::new(), Vec::new())
                };
                let src = if word_sort { stripped.as_slice() } else { s };
                let mut buf = vec![0u8; src.len() * 6 + 16];
                let need = (a.ucol_get_sort_key)(col, src.as_ptr(), src.len() as i32, buf.as_mut_ptr(), buf.len() as i32);
                let got = if need as usize > buf.len() {
                    buf.resize(need as usize, 0);
                    let need2 = (a.ucol_get_sort_key)(col, src.as_ptr(), src.len() as i32, buf.as_mut_ptr(), buf.len() as i32);
                    (a.ucol_close)(col);
                    need2
                } else {
                    (a.ucol_close)(col);
                    need
                };
                if got > 0 {
                    buf.truncate(got as usize);
                    if word_sort {
                        append_word_sort_weights(&mut buf, &special);
                    }
                    return buf;
                }
            }
        }
    }
    sort_key(s, flags)
}

/// Sort key whose `memcmp` order matches [`compare_linguistic`].
pub fn sort_key(s: &[u16], flags: u32) -> Vec<u8> {
    let ignore_case = flags & (NORM_IGNORECASE | LINGUISTIC_IGNORECASE) != 0;
    let chars: Vec<char> = char::decode_utf16(s.iter().copied())
        .map(|r| r.unwrap_or('\u{FFFD}'))
        .filter(|c| !(flags & SORT_STRINGSORT == 0 && (*c == '-' || *c == '\'')))
        .filter(|c| !(flags & NORM_IGNORESYMBOLS != 0 && !c.is_alphanumeric()))
        .collect();
    let mut k = Vec::with_capacity(chars.len() * 5 + 4);
    for &c in &chars {
        let (p, _) = coll(c, flags);
        k.extend_from_slice(&p.to_be_bytes());
    }
    k.push(1);
    if !ignore_case {
        for &c in &chars {
            k.push(coll(c, flags).1 + 2);
        }
    }
    k.push(1);
    k.push(0);
    k
}

/// Map a single UTF-16 code unit to upper or lower case using Apple ICU with `icu_locale`
/// (e.g. `"tr_TR"` maps `'i'` -> `'İ'` and `'I'` -> `'ı'`). Returns `u` unchanged if the
/// mapping expands to multiple code units (standard C `toupper`/`towupper` contract).
pub(crate) fn case_map_unit_icu(icu_locale: &str, u: u16, to_upper: bool) -> u16 {
    let mode = if to_upper { LCMAP_UPPERCASE } else { LCMAP_LOWERCASE };
    if let Some(mapped) = icu_case_map(icu_locale, mode, &[u]) {
        if mapped.len() == 1 {
            return mapped[0];
        }
        return u;
    }
    if to_upper { wupper(u) } else { wlower(u) }
}

fn icu_case_map(icu_locale: &str, mode: u32, s: &[u16]) -> Option<Vec<u16>> {
    let a = icu::api()?;
    let cloc = icu::c_str(icu_locale);
    let mut buf = vec![0u16; s.len() * 2 + 16];
    unsafe {
        let mut st = 0i32;
        let run = |dst: *mut u16, cap: i32, st: &mut i32| -> i32 {
            match mode {
                LCMAP_LOWERCASE => (a.u_str_to_lower)(dst, cap, s.as_ptr(), s.len() as i32, cloc.as_ptr(), st),
                LCMAP_UPPERCASE => (a.u_str_to_upper)(dst, cap, s.as_ptr(), s.len() as i32, cloc.as_ptr(), st),
                LCMAP_TITLECASE => (a.u_str_to_title)(dst, cap, s.as_ptr(), s.len() as i32, std::ptr::null_mut(), cloc.as_ptr(), st),
                _ => -1,
            }
        };
        let n = run(buf.as_mut_ptr(), buf.len() as i32, &mut st);
        if st == 15 && n > 0 {
            buf.resize(n as usize + 1, 0);
            st = 0;
            let n2 = run(buf.as_mut_ptr(), buf.len() as i32, &mut st);
            if st <= 0 && n2 >= 0 {
                buf.truncate(n2 as usize);
                return Some(buf);
            }
            return None;
        }
        if st <= 0 && n >= 0 {
            buf.truncate(n as usize);
            Some(buf)
        } else {
            None
        }
    }
}

pub unsafe extern "win64" fn LCMapStringEx(
    locale: LPCWSTR,
    flags: u32,
    src: LPCWSTR,
    n: i32,
    dst: LPWSTR,
    cap: i32,
    _ver: *mut c_void,
    _res: *mut c_void,
    _sort_handle: isize,
) -> i32 {
    if src.is_null() || n == 0 || cap < 0 || (cap > 0 && dst.is_null()) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let Some(loc) = resolve_locale_name_w(locale) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let all_known_flags = LCMAP_LOWERCASE
        | LCMAP_UPPERCASE
        | LCMAP_TITLECASE
        | LCMAP_SORTKEY
        | LCMAP_BYTEREV
        | LCMAP_HASH
        | LCMAP_SORTSHOWHYPHENS
        | LCMAP_HIRAGANA
        | LCMAP_KATAKANA
        | LCMAP_HALFWIDTH
        | LCMAP_FULLWIDTH
        | LCMAP_LINGUISTIC_CASING
        | LCMAP_SIMPLIFIED_CHINESE
        | LCMAP_TRADITIONAL_CHINESE
        | NORM_IGNORECASE
        | NORM_IGNORENONSPACE
        | NORM_IGNORESYMBOLS
        | SORT_DIGITSASNUMBERS
        | LINGUISTIC_IGNORECASE
        | LINGUISTIC_IGNOREDIACRITIC
        | SORT_STRINGSORT
        | NORM_IGNOREKANATYPE
        | NORM_IGNOREWIDTH
        | NORM_LINGUISTIC_CASING;
    if (flags & !all_known_flags) != 0 {
        set_err(err::ERROR_INVALID_FLAGS);
        return 0;
    }
    // Mutually exclusive flag pairs per Microsoft Learn (`LCMapStringEx`).
    if ((flags & LCMAP_HIRAGANA) != 0 && (flags & LCMAP_KATAKANA) != 0)
        || ((flags & LCMAP_HALFWIDTH) != 0 && (flags & LCMAP_FULLWIDTH) != 0)
        || ((flags & LCMAP_SIMPLIFIED_CHINESE) != 0 && (flags & LCMAP_TRADITIONAL_CHINESE) != 0)
        || ((flags & LCMAP_SORTKEY) != 0 && (flags & LCMAP_HASH) != 0)
    {
        set_err(err::ERROR_INVALID_FLAGS);
        return 0;
    }
    let with_nul = n < 0;
    let s: Vec<u16> = if n < 0 {
        crate::strings::read_wide_units_z(src, DEFAULT_MAX_STR_LEN)
    } else {
        crate::strings::read_wide_slice(src, n as usize)
    };
    let icu_loc = if loc.invariant { "" } else { loc.icu_id.as_str() };
    if (flags & LCMAP_SORTKEY) != 0 {
        let k = sort_key_for_locale(icu_loc, &s, flags);
        if cap == 0 {
            return k.len() as i32;
        }
        if k.len() > cap as usize {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            return 0;
        }
        std::ptr::copy_nonoverlapping(k.as_ptr(), dst as *mut u8, k.len());
        return k.len() as i32;
    }
    if (flags & LCMAP_HASH) != 0 {
        // Microsoft Learn (`LCMapStringEx` with `LCMAP_HASH`): computes a locale-sensitive
        // 32-bit hash of the string's sort weights; `cchDest` is in bytes and must be >= 4.
        if cap == 0 {
            return 4;
        }
        if cap < 4 {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            return 0;
        }
        let k = sort_key_for_locale(icu_loc, &s, flags);
        let mut h: u32 = 0x811C_9DC5; // FNV-1a 32-bit over canonical sort-weight bytes
        for &b in &k {
            h ^= b as u32;
            h = h.wrapping_mul(0x0100_0193);
        }
        std::ptr::write_unaligned(dst as *mut u32, h);
        return 4;
    }

    let case_mode = flags & 0x300;
    let case_locale = if (flags & LCMAP_LINGUISTIC_CASING) != 0 { icu_loc } else { "" };
    let mut out: Vec<u16> = match case_mode {
        LCMAP_LOWERCASE => icu_case_map(case_locale, LCMAP_LOWERCASE, &s)
            .unwrap_or_else(|| s.iter().map(|&c| wlower(c)).collect()),
        LCMAP_UPPERCASE => icu_case_map(case_locale, LCMAP_UPPERCASE, &s)
            .unwrap_or_else(|| s.iter().map(|&c| wupper(c)).collect()),
        LCMAP_TITLECASE => icu_case_map(case_locale, LCMAP_TITLECASE, &s).unwrap_or_else(|| {
            let mut prev_alpha = false;
            s.iter()
                .map(|&c| {
                    let a = char::from_u32(c as u32).is_some_and(|ch| ch.is_alphabetic());
                    let r = if a && !prev_alpha { wupper(c) } else { wlower(c) };
                    prev_alpha = a;
                    r
                })
                .collect()
        }),
        _ => s,
    };

    if (flags & LCMAP_HIRAGANA) != 0 {
        out = icu::transliterate("Katakana-Hiragana", &out).unwrap_or_else(|| {
            out.into_iter()
                .map(|c| if (0x30A1..=0x30F6).contains(&c) { c - 0x60 } else { c })
                .collect()
        });
    } else if (flags & LCMAP_KATAKANA) != 0 {
        out = icu::transliterate("Hiragana-Katakana", &out).unwrap_or_else(|| {
            out.into_iter()
                .map(|c| if (0x3041..=0x3096).contains(&c) { c + 0x60 } else { c })
                .collect()
        });
    }

    if (flags & LCMAP_HALFWIDTH) != 0 {
        out = icu::transliterate("Fullwidth-Halfwidth", &out).unwrap_or_else(|| {
            out.into_iter()
                .map(|c| match c {
                    0x3000 => 0x0020,
                    0xFF01..=0xFF5E => c - 0xFEE0,
                    _ => c,
                })
                .collect()
        });
    } else if (flags & LCMAP_FULLWIDTH) != 0 {
        out = icu::transliterate("Halfwidth-Fullwidth", &out).unwrap_or_else(|| {
            out.into_iter()
                .map(|c| match c {
                    0x0020 => 0x3000,
                    0x0021..=0x007E => c + 0xFEE0,
                    _ => c,
                })
                .collect()
        });
    }

    if (flags & LCMAP_SIMPLIFIED_CHINESE) != 0 {
        if let Some(mapped) = icu::transliterate("Traditional-Simplified", &out) {
            out = mapped;
        }
    } else if (flags & LCMAP_TRADITIONAL_CHINESE) != 0 {
        if let Some(mapped) = icu::transliterate("Simplified-Traditional", &out) {
            out = mapped;
        }
    }

    if (flags & LCMAP_BYTEREV) != 0 {
        for c in out.iter_mut() {
            *c = c.swap_bytes();
        }
    }

    if with_nul {
        out.push(0);
    }
    if cap == 0 {
        return out.len() as i32;
    }
    if out.len() > cap as usize {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(out.as_ptr() as *const u8, dst as *mut u8, out.len() * 2);
    out.len() as i32
}

pub unsafe extern "win64" fn LCMapStringW(lcid: u32, flags: u32, src: LPCWSTR, n: i32, dst: LPWSTR, cap: i32) -> i32 {
    let Some(loc) = resolve_lcid(lcid) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let name_w = to_wide_z(&loc.bcp47);
    LCMapStringEx(name_w.as_ptr(), flags, src, n, dst, cap, std::ptr::null_mut(), std::ptr::null_mut(), 0)
}

pub unsafe extern "win64" fn LCMapStringA(lcid: u32, flags: u32, src: LPCSTR, n: i32, dst: LPSTR, cap: i32) -> i32 {
    if src.is_null() || n == 0 || cap < 0 || (cap > 0 && dst.is_null()) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    const LOCALE_USE_CP_ACP_FLAG: u32 = 0x4000_0000;
    let bytes = if n < 0 { std::slice::from_raw_parts(src, strlen(src) + 1) } else { std::slice::from_raw_parts(src, n as usize) };
    let cp = if (flags & LOCALE_USE_CP_ACP_FLAG) != 0 {
        ACP
    } else {
        resolve_lcid(lcid)
            .and_then(|loc| locale_info_for(&loc, 0x1004))
            .and_then(|s| s.parse::<u32>().ok())
            .filter(|&c| c != 0 && valid_cp(c))
            .unwrap_or(ACP)
    };
    let w = decode_cp_to_utf16(cp, bytes)
        .unwrap_or_else(|_| String::from_utf8_lossy(bytes).encode_utf16().collect());
    let inner_flags = flags & !LOCALE_USE_CP_ACP_FLAG;
    if inner_flags & (LCMAP_SORTKEY | LCMAP_HASH) != 0 {
        return LCMapStringW(lcid, inner_flags, w.as_ptr(), w.len() as i32, dst as LPWSTR, cap);
    }
    let need = LCMapStringW(lcid, inner_flags, w.as_ptr(), w.len() as i32, std::ptr::null_mut(), 0);
    if need <= 0 {
        return 0;
    }
    let mut out = vec![0u16; need as usize];
    LCMapStringW(lcid, inner_flags, w.as_ptr(), w.len() as i32, out.as_mut_ptr(), need);
    let b = encode_utf16_to_cp(cp, &out, Some(b'?'))
        .map(|(v, _)| v)
        .unwrap_or_else(|_| String::from_utf16_lossy(&out).into_bytes());
    if cap == 0 {
        return b.len() as i32;
    }
    if b.len() > cap as usize {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(b.as_ptr(), dst, b.len());
    b.len() as i32
}

// ---------------------------------------------------------------------------
// GetStringType
// ---------------------------------------------------------------------------

pub const C1_UPPER: u16 = 0x1;
pub const C1_LOWER: u16 = 0x2;
pub const C1_DIGIT: u16 = 0x4;
pub const C1_SPACE: u16 = 0x8;
pub const C1_PUNCT: u16 = 0x10;
pub const C1_CNTRL: u16 = 0x20;
pub const C1_BLANK: u16 = 0x40;
pub const C1_XDIGIT: u16 = 0x80;
pub const C1_ALPHA: u16 = 0x100;
pub const C1_DEFINED: u16 = 0x200;

/// CT_CTYPE1 classification of a UTF-16 unit.
pub fn ctype1(c: u16) -> u16 {
    let Some(ch) = char::from_u32(c as u32) else {
        return 0; // lone surrogate
    };
    let mut t = C1_DEFINED;
    if ch.is_uppercase() {
        t |= C1_UPPER;
    }
    if ch.is_lowercase() {
        t |= C1_LOWER;
    }
    if ch.is_ascii_digit() || (ch.is_numeric() && ch.to_digit(10).is_some()) {
        t |= C1_DIGIT;
    }
    if ch.is_whitespace() || (0x09..=0x0D).contains(&c) {
        t |= C1_SPACE;
    }
    if c == 0x20 || c == 0x09 || c == 0xA0 || c == 0x3000 {
        t |= C1_BLANK;
    }
    if ch.is_control() {
        t |= C1_CNTRL;
    }
    if ch.is_ascii_hexdigit() {
        t |= C1_XDIGIT;
    }
    if ch.is_alphabetic() {
        t |= C1_ALPHA;
    }
    if !ch.is_alphanumeric() && !ch.is_whitespace() && !ch.is_control() && c != 0 {
        t |= C1_PUNCT;
    }
    if c == 0 {
        t |= C1_CNTRL;
    }
    t
}

fn ctype2(c: u16) -> u16 {
    let Some(ch) = char::from_u32(c as u32) else { return 0 };
    if ch.is_alphabetic() {
        1 // C2_LEFTTORIGHT
    } else if ch.is_ascii_digit() {
        3 // C2_EUROPENUMBER
    } else if ch.is_whitespace() {
        0xA // C2_WHITESPACE
    } else {
        0xB // C2_OTHERNEUTRAL
    }
}

fn ctype3(c: u16) -> u16 {
    let Some(ch) = char::from_u32(c as u32) else { return 0 };
    let mut t = 0;
    if ch.is_alphabetic() {
        t |= 0x8000; // C3_ALPHA
    }
    if !ch.is_alphanumeric() && !ch.is_whitespace() && !ch.is_control() {
        t |= 0x8; // C3_SYMBOL
    }
    if (0x300..0x370).contains(&c) {
        t |= 0x1; // C3_NONSPACING
    }
    if (0xFF00..0xFFEF).contains(&c) || (0x3000..0x30FF).contains(&c) {
        t |= 0x80; // C3_FULLWIDTH
    }
    t
}

pub unsafe extern "win64" fn GetStringTypeW(info: u32, src: LPCWSTR, n: i32, out: *mut u16) -> BOOL {
    if src.is_null() || out.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let s = if n < 0 { std::slice::from_raw_parts(src, wcslen(src) + 1) } else { std::slice::from_raw_parts(src, n as usize) };
    let f = match info {
        1 => ctype1,
        2 => ctype2,
        4 => ctype3,
        _ => {
            set_err(err::ERROR_INVALID_FLAGS);
            return FALSE;
        }
    };
    for (i, &c) in s.iter().enumerate() {
        *out.add(i) = f(c);
    }
    TRUE
}

pub unsafe extern "win64" fn GetStringTypeExW(_lcid: u32, info: u32, src: LPCWSTR, n: i32, out: *mut u16) -> BOOL {
    GetStringTypeW(info, src, n, out)
}

/// ANSI version: one output word per input *byte*. With ACP = UTF-8,
/// multi-byte sequences classify each byte (continuation bytes = 0), which
/// matches Windows' behaviour for DBCS trail bytes closely enough.
pub unsafe extern "win64" fn GetStringTypeA(_lcid: u32, info: u32, src: LPCSTR, n: i32, out: *mut u16) -> BOOL {
    if src.is_null() || out.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let s = if n < 0 { std::slice::from_raw_parts(src, strlen(src) + 1) } else { std::slice::from_raw_parts(src, n as usize) };
    for (i, &b) in s.iter().enumerate() {
        let c = if b < 0x80 { b as u16 } else { 0xFFFF };
        *out.add(i) = if c == 0xFFFF {
            0
        } else {
            match info {
                1 => ctype1(c),
                2 => ctype2(c),
                4 => ctype3(c),
                _ => 0,
            }
        };
    }
    TRUE
}

pub unsafe extern "win64" fn GetStringTypeExA(lcid: u32, info: u32, src: LPCSTR, n: i32, out: *mut u16) -> BOOL {
    GetStringTypeA(lcid, info, src, n, out)
}

pub unsafe extern "win64" fn FoldStringW(_flags: u32, src: LPCWSTR, n: i32, dst: LPWSTR, cap: i32) -> i32 {
    LCMapStringW(0x400, 0, src, n, dst, cap)
}

// ---------------------------------------------------------------------------
// Locale resolution & ICU-backed NLS queries
// ---------------------------------------------------------------------------

pub const LOCALE_USER_DEFAULT: u32 = 0x400;
pub const LOCALE_SYSTEM_DEFAULT: u32 = 0x800;
pub const LOCALE_CUSTOM_DEFAULT: u32 = 0xC00;
pub const LOCALE_INVARIANT: u32 = 0x7F;

#[derive(Clone, Debug)]
pub(crate) struct ResolvedLocale {
    pub invariant: bool,
    pub lcid: u32,
    pub icu_id: String,
    pub bcp47: String,
}

impl ResolvedLocale {
    fn invariant_loc() -> Self {
        Self {
            invariant: true,
            lcid: LOCALE_INVARIANT,
            icu_id: String::new(),
            bcp47: String::new(),
        }
    }
    fn en_us() -> Self {
        Self {
            invariant: false,
            lcid: LCID_EN_US,
            icu_id: "en_US".into(),
            bcp47: "en-US".into(),
        }
    }
}

fn icu_to_bcp47(icu_id: &str) -> String {
    if let Some(a) = icu::api() {
        let c = icu::c_str(icu_id);
        if let Some(tag) = unsafe { icu::call_ascii_str(|b, cap, st| (a.uloc_to_language_tag)(c.as_ptr(), b, cap, 0, st)) } {
            if !tag.is_empty() && tag != "und" {
                return tag;
            }
        }
    }
    icu_id.replace('_', "-")
}

pub(crate) fn resolve_lcid(lcid: u32) -> Option<ResolvedLocale> {
    let base = lcid & 0xFFFF;
    match base {
        0 | LOCALE_USER_DEFAULT | LOCALE_SYSTEM_DEFAULT | LOCALE_CUSTOM_DEFAULT | LCID_EN_US | 0x09 => {
            return Some(ResolvedLocale::en_us());
        }
        LOCALE_INVARIANT => return Some(ResolvedLocale::invariant_loc()),
        _ => {}
    }
    if let Some(a) = icu::api() {
        let icu_id = unsafe {
            icu::call_ascii_str(|b, cap, st| (a.uloc_get_locale_for_lcid)(lcid, b, cap, st))
                .filter(|s| !s.is_empty())
                .or_else(|| {
                    if lcid != base {
                        icu::call_ascii_str(|b, cap, st| (a.uloc_get_locale_for_lcid)(base, b, cap, st))
                            .filter(|s| !s.is_empty())
                    } else {
                        None
                    }
                })
        };
        if let Some(icu_id) = icu_id {
            let bcp47 = icu_to_bcp47(&icu_id);
            return Some(ResolvedLocale { invariant: false, lcid: base, icu_id, bcp47 });
        }
    }
    None
}

pub(crate) fn resolve_locale_tag(name: &str) -> Option<ResolvedLocale> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Some(ResolvedLocale::invariant_loc());
    }
    let lc = trimmed.to_ascii_lowercase();
    if lc == "!x-sys-default-locale" || lc == "en-us" || lc == "en_us" {
        return Some(ResolvedLocale::en_us());
    }
    // Validate BCP-47 syntax before passing to ICU so arbitrary garbage is rejected cleanly.
    let norm = trimmed.replace('_', "-");
    let mut parts = norm.split('-');
    let lang = parts.next()?;
    if !(2..=3).contains(&lang.len()) || !lang.bytes().all(|b| b.is_ascii_alphabetic()) {
        return None;
    }
    for p in parts {
        if p.is_empty() || p.len() > 8 || !p.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return None;
        }
    }
    let a = icu::api()?;
    let c_in = icu::c_str(&norm);
    unsafe {
        let mut parsed_len = 0i32;
        let icu_id = icu::call_ascii_str(|b, cap, st| {
            (a.uloc_for_language_tag)(c_in.as_ptr(), b, cap, &mut parsed_len, st)
        })
        .filter(|s| !s.is_empty())?;
        // Verify ICU actually recognizes the primary language (uloc_getISO3Language is non-empty).
        let c_icu = icu::c_str(&icu_id);
        let iso3 = (a.uloc_get_iso3_language)(c_icu.as_ptr());
        if iso3.is_null() || CStr::from_ptr(iso3).to_bytes().is_empty() {
            return None;
        }
        let lcid = (a.uloc_get_lcid)(c_icu.as_ptr());
        let bcp47 = icu_to_bcp47(&icu_id);
        Some(ResolvedLocale {
            invariant: false,
            lcid: if lcid != 0 { lcid } else { 0x1000 }, // LOCALE_CUSTOM_UNSPECIFIED
            icu_id,
            bcp47,
        })
    }
}

unsafe fn resolve_locale_name_w(name: LPCWSTR) -> Option<ResolvedLocale> {
    if name.is_null() {
        return Some(ResolvedLocale::en_us());
    }
    let s = from_wide(name);
    resolve_locale_tag(&s)
}

/// Convert an ICU date/time pattern to a Windows NLS picture string.
fn icu_pattern_to_win(pat: &str, force_4digit_year: bool) -> String {
    let chars: Vec<char> = pat.chars().collect();
    let mut out = String::with_capacity(pat.len() + 4);
    let mut i = 0;
    let mut in_quote = false;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' {
            in_quote = !in_quote;
            out.push(c);
            i += 1;
            continue;
        }
        if in_quote {
            out.push(c);
            i += 1;
            continue;
        }
        let mut n = 1;
        while i + n < chars.len() && chars[i + n] == c {
            n += 1;
        }
        match c {
            'E' | 'e' | 'c' => {
                if n >= 4 {
                    out.push_str("dddd");
                } else {
                    out.push_str("ddd");
                }
            }
            'a' => out.push_str("tt"),
            'y' => {
                if force_4digit_year || n != 2 {
                    out.push_str("yyyy");
                } else {
                    out.push_str("yy");
                }
            }
            'L' => {
                for _ in 0..n.min(4) {
                    out.push('M');
                }
            }
            'G' => out.push_str("gg"),
            'z' | 'Z' | 'v' | 'V' | 'O' => {} // strip timezone suffix from Windows time/date format
            _ => {
                for _ in 0..n {
                    out.push(c);
                }
            }
        }
        i += n;
    }
    out.trim().to_string()
}

fn default_ansi_cp_for(lang: &str, country: &str) -> u32 {
    match lang {
        "ja" => 932,
        "zh" => match country {
            "TW" | "HK" | "MO" => 950,
            _ => 936,
        },
        "ko" => 949,
        "ru" | "uk" | "bg" | "be" | "sr" | "mk" | "kk" | "ky" => 1251,
        "pl" | "cs" | "sk" | "hu" | "ro" | "hr" | "sl" | "sq" | "bs" => 1250,
        "el" => 1253,
        "tr" | "az" => 1254,
        "he" | "iw" => 1255,
        "ar" | "fa" | "ur" => 1256,
        "et" | "lv" | "lt" => 1257,
        "vi" => 1258,
        "th" => 874,
        _ => 1252,
    }
}

fn default_oem_cp_for(lang: &str, country: &str) -> u32 {
    match lang {
        "ja" => 932,
        "zh" => match country {
            "TW" | "HK" | "MO" => 950,
            _ => 936,
        },
        "ko" => 949,
        "en" if country == "US" => 437,
        "ru" | "uk" | "bg" | "be" | "kk" => 866,
        "pl" | "cs" | "sk" | "hu" | "ro" | "hr" | "sl" => 852,
        "el" => 737,
        "tr" => 857,
        "he" | "iw" => 862,
        "ar" => 720,
        "th" => 874,
        "et" | "lv" | "lt" => 775,
        _ => 850,
    }
}

fn extract_sep(pat: &str, fallback: &str) -> String {
    let mut in_quote = false;
    for ch in pat.chars() {
        if ch == '\'' {
            in_quote = !in_quote;
            continue;
        }
        if !in_quote && matches!(ch, '/' | '.' | '-' | ':' | ' ') {
            return ch.to_string();
        }
    }
    fallback.to_string()
}

fn date_order(pat: &str) -> &'static str {
    let mut m = usize::MAX;
    let mut d = usize::MAX;
    let mut y = usize::MAX;
    for (i, ch) in pat.chars().enumerate() {
        if ch == 'M' && m == usize::MAX {
            m = i;
        } else if ch == 'd' && d == usize::MAX {
            d = i;
        } else if ch == 'y' && y == usize::MAX {
            y = i;
        }
    }
    if y < m && y < d {
        "2" // YMD
    } else if d < m {
        "1" // DMY
    } else {
        "0" // MDY
    }
}

#[derive(Clone, Copy)]
struct CurrencyPlacement {
    p_cs_precedes: u8,
    p_sep_by_space: u8,
    n_cs_precedes: u8,
    n_sep_by_space: u8,
    p_sign_posn: u8,
    n_sign_posn: u8,
    win_icurrency: u8,
    win_inegcurr: u8,
}

fn parse_currency_subpattern(sub: &str, is_neg: bool) -> (u8, u8, u8) {
    let chars: Vec<char> = sub.chars().filter(|&c| c != '\'').collect();
    let curr_pos = chars.iter().position(|&c| c == '¤');
    let num_start = chars.iter().position(|&c| c == '#' || c == '0');
    let num_end = chars.iter().rposition(|&c| c == '#' || c == '0').map(|i| i + 1);
    let (Some(cp), Some(ns), Some(ne)) = (curr_pos, num_start, num_end) else {
        return (1, 0, if is_neg { 1 } else { 3 });
    };
    let cs_precedes = if cp < ns { 1u8 } else { 0u8 };
    let between = if cs_precedes == 1 {
        &chars[cp + 1..ns]
    } else if ne <= cp {
        &chars[ne..cp]
    } else {
        &[]
    };
    let sep_by_space = if between.iter().any(|c| c.is_whitespace()) { 1u8 } else { 0u8 };
    let sign_posn = if !is_neg {
        if cs_precedes == 1 { 3 } else { 1 }
    } else if chars.contains(&'(') && chars.contains(&')') {
        0
    } else {
        let mp = chars.iter().position(|&c| c == '-' || c == '\u{2212}').unwrap_or(0);
        if mp < cp && mp < ns {
            1
        } else if mp > cp && mp >= ne {
            2
        } else if cs_precedes == 1 {
            4
        } else {
            3
        }
    };
    (cs_precedes, sep_by_space, sign_posn)
}

fn parse_currency_pattern(pat: &str) -> CurrencyPlacement {
    let (pos_sub, neg_sub_owned) = match pat.split_once(';') {
        Some((p, n)) => (p, n.to_string()),
        None => (pat, format!("-{pat}")),
    };
    let (p_cs_precedes, p_sep_by_space, p_sign_posn) = parse_currency_subpattern(pos_sub, false);
    let (n_cs_precedes, n_sep_by_space, n_sign_posn) = parse_currency_subpattern(&neg_sub_owned, true);
    let win_icurrency = match (p_cs_precedes, p_sep_by_space) {
        (1, 0) => 0, // $1.1
        (0, 0) => 1, // 1.1$
        (1, 1) => 2, // $ 1.1
        _ => 3,      // 1.1 $
    };
    let win_inegcurr = match (n_cs_precedes, n_sep_by_space, n_sign_posn) {
        (1, 0, 0) => 0,  // ($1.1)
        (1, 0, 1) => 1,  // -$1.1
        (1, 0, 4) => 2,  // $-1.1
        (1, 0, 2) => 3,  // $1.1-
        (0, 0, 0) => 4,  // (1.1$)
        (0, 0, 1) => 5,  // -1.1$
        (0, 0, 3) => 6,  // 1.1-$
        (0, 0, 2) => 7,  // 1.1$-
        (0, 1, 1) => 8,  // -1.1 $
        (1, 1, 1) => 9,  // -$ 1.1
        (0, 1, 2) => 10, // 1.1 $-
        (1, 1, 2) => 11, // $ 1.1-
        (1, 1, 4) => 12, // $ -1.1
        (0, 1, 3) => 13, // 1.1- $
        (1, 1, 0) => 14, // ($ 1.1)
        (0, 1, 0) => 15, // (1.1 $)
        _ => 1,
    };
    CurrencyPlacement {
        p_cs_precedes,
        p_sep_by_space,
        n_cs_precedes,
        n_sep_by_space,
        p_sign_posn,
        n_sign_posn,
        win_icurrency,
        win_inegcurr,
    }
}

/// Query a Windows `LCTYPE` value for any resolved locale using Apple ICU.
pub(crate) fn locale_info_for(loc: &ResolvedLocale, lctype: u32) -> Option<String> {
    if loc.invariant {
        return locale_info_en_or_inv(lctype, true).map(|c| c.into_owned());
    }
    let t = lctype & 0x0FFF_FFFF & !0x0800_0000;
    // Fast path for en-US fallback if ICU isn't loaded.
    let Some(a) = icu::api() else {
        return locale_info_en_or_inv(lctype, false).map(|c| c.into_owned());
    };
    let cloc = icu::c_str(&loc.icu_id);
    let en_us = c"en_US";

    let num_sym = |style: i32, sym: i32| -> Option<String> {
        unsafe {
            let mut st = 0i32;
            let nf = (a.unum_open)(style, std::ptr::null(), 0, cloc.as_ptr(), std::ptr::null_mut(), &mut st);
            if nf.is_null() || st > 0 {
                return None;
            }
            let r = icu::call_u16_str(|b, cap, s| (a.unum_get_symbol)(nf, sym, b, cap, s));
            (a.unum_close)(nf);
            r
        }
    };
    let num_attr = |style: i32, attr: i32| -> Option<i32> {
        unsafe {
            let mut st = 0i32;
            let nf = (a.unum_open)(style, std::ptr::null(), 0, cloc.as_ptr(), std::ptr::null_mut(), &mut st);
            if nf.is_null() || st > 0 {
                return None;
            }
            let v = (a.unum_get_attribute)(nf, attr);
            (a.unum_close)(nf);
            Some(v)
        }
    };
    let curr_placement = || -> CurrencyPlacement {
        unsafe {
            let mut st = 0i32;
            let nf = (a.unum_open)(icu::UNUM_CURRENCY, std::ptr::null(), 0, cloc.as_ptr(), std::ptr::null_mut(), &mut st);
            if !nf.is_null() && st <= 0 {
                let pat = icu::call_u16_str(|b, cap, s| (a.unum_to_pattern)(nf, 0, b, cap, s));
                (a.unum_close)(nf);
                if let Some(p) = pat {
                    return parse_currency_pattern(&p);
                }
            }
        }
        parse_currency_pattern("¤#,##0.00")
    };
    let date_pat = |tstyle: i32, dstyle: i32, force4y: bool| -> Option<String> {
        unsafe {
            let mut st = 0i32;
            let df = (a.udat_open)(tstyle, dstyle, cloc.as_ptr(), std::ptr::null(), 0, std::ptr::null(), 0, &mut st);
            if df.is_null() || st > 0 {
                return None;
            }
            let raw = icu::call_u16_str(|b, cap, s| (a.udat_to_pattern)(df, 0, b, cap, s));
            (a.udat_close)(df);
            raw.map(|p| icu_pattern_to_win(&p, force4y))
        }
    };
    let best_pat = |skel: &str| -> Option<String> {
        unsafe {
            let mut st = 0i32;
            let pg = (a.udatpg_open)(cloc.as_ptr(), &mut st);
            if pg.is_null() || st > 0 {
                return None;
            }
            let sk: Vec<u16> = skel.encode_utf16().collect();
            let raw = icu::call_u16_str(|b, cap, s| {
                (a.udatpg_get_best_pattern)(pg, sk.as_ptr(), sk.len() as i32, b, cap, s)
            });
            (a.udatpg_close)(pg);
            raw.map(|p| icu_pattern_to_win(&p, true))
        }
    };
    let date_sym = |sym_type: i32, idx: i32| -> Option<String> {
        unsafe {
            let mut st = 0i32;
            let df = (a.udat_open)(icu::UDAT_NONE, icu::UDAT_SHORT, cloc.as_ptr(), std::ptr::null(), 0, std::ptr::null(), 0, &mut st);
            if df.is_null() || st > 0 {
                return None;
            }
            let r = icu::call_u16_str(|b, cap, s| (a.udat_get_symbols)(df, sym_type, idx, b, cap, s));
            (a.udat_close)(df);
            r
        }
    };
    let lang = unsafe { icu::call_ascii_str(|b, cap, s| (a.uloc_get_language)(cloc.as_ptr(), b, cap, s)) }.unwrap_or_default();
    let country = unsafe { icu::call_ascii_str(|b, cap, s| (a.uloc_get_country)(cloc.as_ptr(), b, cap, s)) }.unwrap_or_default();

    let s: String = match t {
        0x01 | 0x09 => format!("{:04x}", loc.lcid & 0xFFFF),
        0x02 | 0x72 => unsafe {
            icu::call_u16_str(|b, cap, s| (a.uloc_get_display_name)(cloc.as_ptr(), en_us.as_ptr(), b, cap, s))
                .unwrap_or_else(|| loc.bcp47.clone())
        },
        0x03 => unsafe {
            let p = (a.uloc_get_iso3_language)(cloc.as_ptr());
            let iso3 = if p.is_null() { "" } else { CStr::from_ptr(p).to_str().unwrap_or("") };
            if lang == "en" && country == "US" {
                "ENU".into()
            } else {
                iso3.to_ascii_uppercase()
            }
        },
        0x04 => unsafe {
            icu::call_u16_str(|b, cap, s| (a.uloc_get_display_language)(cloc.as_ptr(), cloc.as_ptr(), b, cap, s))
                .unwrap_or_else(|| lang.clone())
        },
        0x05 | 0x0A => geo_by_iso2(&country).map(|g| g.dialing.to_string()).unwrap_or_else(|| "1".into()),
        0x06 | 0x75 | 0x1002 => unsafe {
            icu::call_u16_str(|b, cap, s| (a.uloc_get_display_country)(cloc.as_ptr(), en_us.as_ptr(), b, cap, s))
                .unwrap_or_default()
        },
        0x07 | 0x68 => unsafe {
            let p = (a.uloc_get_iso3_country)(cloc.as_ptr());
            if p.is_null() { String::new() } else { CStr::from_ptr(p).to_string_lossy().to_ascii_uppercase() }
        },
        0x08 => unsafe {
            icu::call_u16_str(|b, cap, s| (a.uloc_get_display_country)(cloc.as_ptr(), cloc.as_ptr(), b, cap, s))
                .unwrap_or_default()
        },
        0x0B => default_oem_cp_for(&lang, &country).to_string(),
        0x0C => {
            let dec = num_sym(icu::UNUM_DECIMAL, icu::UNUM_DECIMAL_SEPARATOR_SYMBOL).unwrap_or_else(|| ".".into());
            if dec == "," { ";".into() } else { ",".into() }
        }
        0x0D => unsafe {
            let mut st = 0i32;
            let ms = (a.ulocdata_get_measurement_system)(cloc.as_ptr(), &mut st);
            if st <= 0 && ms == icu::UMS_US { "1".into() } else { "0".into() }
        },
        0x0E => num_sym(icu::UNUM_DECIMAL, icu::UNUM_DECIMAL_SEPARATOR_SYMBOL).unwrap_or_else(|| ".".into()),
        0x0F => num_sym(icu::UNUM_DECIMAL, icu::UNUM_GROUPING_SEPARATOR_SYMBOL).unwrap_or_else(|| ",".into()),
        0x10 | 0x18 => {
            let style = if t == 0x18 { icu::UNUM_CURRENCY } else { icu::UNUM_DECIMAL };
            let g1 = num_attr(style, icu::UNUM_GROUPING_SIZE).unwrap_or(3).max(0);
            let g2 = num_attr(style, icu::UNUM_SECONDARY_GROUPING_SIZE).unwrap_or(0).max(0);
            if g1 == 0 {
                "0".into()
            } else if g2 > 0 && g2 != g1 {
                format!("{g1};{g2};0")
            } else {
                format!("{g1};0")
            }
        }
        0x11 => "2".into(),
        0x12 => "1".into(),
        0x13 => {
            let zero = num_sym(icu::UNUM_DECIMAL, icu::UNUM_ZERO_DIGIT_SYMBOL)
                .and_then(|s| s.chars().next())
                .unwrap_or('0');
            let base = zero as u32;
            (0..10u32).filter_map(|d| char::from_u32(base + d)).collect()
        }
        0x14 => num_sym(icu::UNUM_CURRENCY, icu::UNUM_CURRENCY_SYMBOL).unwrap_or_else(|| "$".into()),
        0x15 => num_sym(icu::UNUM_CURRENCY, icu::UNUM_INTL_CURRENCY_SYMBOL).unwrap_or_else(|| "USD".into()),
        0x16 => num_sym(icu::UNUM_CURRENCY, icu::UNUM_MONETARY_SEPARATOR_SYMBOL)
            .or_else(|| num_sym(icu::UNUM_DECIMAL, icu::UNUM_DECIMAL_SEPARATOR_SYMBOL))
            .unwrap_or_else(|| ".".into()),
        0x17 => num_sym(icu::UNUM_CURRENCY, icu::UNUM_MONETARY_GROUPING_SEPARATOR_SYMBOL)
            .or_else(|| num_sym(icu::UNUM_DECIMAL, icu::UNUM_GROUPING_SEPARATOR_SYMBOL))
            .unwrap_or_else(|| ",".into()),
        0x19 | 0x1A => num_attr(icu::UNUM_CURRENCY, icu::UNUM_MAX_FRACTION_DIGITS).unwrap_or(2).max(0).to_string(),
        0x1B => curr_placement().win_icurrency.to_string(),
        0x1C => curr_placement().win_inegcurr.to_string(),
        0x1D => {
            let p = date_pat(icu::UDAT_NONE, icu::UDAT_SHORT, true).unwrap_or_else(|| "M/d/yyyy".into());
            extract_sep(&p, "/")
        }
        0x1E => {
            let p = date_pat(icu::UDAT_MEDIUM, icu::UDAT_NONE, false).unwrap_or_else(|| "h:mm:ss tt".into());
            extract_sep(&p, ":")
        }
        0x1F => date_pat(icu::UDAT_NONE, icu::UDAT_SHORT, true).unwrap_or_else(|| "M/d/yyyy".into()),
        0x20 => date_pat(icu::UDAT_NONE, icu::UDAT_FULL, true).unwrap_or_else(|| "dddd, MMMM d, yyyy".into()),
        0x21 | 0x22 => {
            let p = date_pat(icu::UDAT_NONE, icu::UDAT_SHORT, true).unwrap_or_else(|| "M/d/yyyy".into());
            date_order(&p).into()
        }
        0x23 => {
            let p = date_pat(icu::UDAT_SHORT, icu::UDAT_NONE, false).unwrap_or_default();
            if p.contains('H') { "1".into() } else { "0".into() }
        }
        0x24 | 0x25 | 0x27 => "0".into(),
        0x26 => "1".into(),
        0x28 => date_sym(icu::UDAT_AM_PMS, 0).unwrap_or_else(|| "AM".into()),
        0x29 => date_sym(icu::UDAT_AM_PMS, 1).unwrap_or_else(|| "PM".into()),
        0x2A..=0x30 => {
            let win_d = (t - 0x2A) as i32; // 0=Mon..6=Sun
            let icu_d = ((win_d + 1) % 7) + 1; // 2=Mon..7=Sat, 1=Sun
            date_sym(icu::UDAT_WEEKDAYS, icu_d).unwrap_or_default()
        }
        0x31..=0x37 => {
            let win_d = (t - 0x31) as i32;
            let icu_d = ((win_d + 1) % 7) + 1;
            date_sym(icu::UDAT_SHORT_WEEKDAYS, icu_d).unwrap_or_default()
        }
        0x38..=0x43 => date_sym(icu::UDAT_MONTHS, (t - 0x38) as i32).unwrap_or_default(),
        0x44..=0x4F => date_sym(icu::UDAT_SHORT_MONTHS, (t - 0x44) as i32).unwrap_or_default(),
        0x50 => String::new(),
        0x51 => num_sym(icu::UNUM_DECIMAL, icu::UNUM_MINUS_SIGN_SYMBOL).unwrap_or_else(|| "-".into()),
        0x52 => curr_placement().p_sign_posn.to_string(),
        0x53 => curr_placement().n_sign_posn.to_string(),
        0x54 => curr_placement().p_cs_precedes.to_string(),
        0x55 => curr_placement().p_sep_by_space.to_string(),
        0x56 => curr_placement().n_cs_precedes.to_string(),
        0x57 => curr_placement().n_sep_by_space.to_string(),
        0x59 => lang,
        0x5A => country,
        0x5B => geo_by_iso2(&country).map(|g| g.geoid.to_string()).unwrap_or_else(|| "244".into()),
        0x5C | 0x6E | 0x7A | 0x7B => loc.bcp47.clone(),
        0x5D => "HH:mm:ss".into(),
        0x5E => format!("{:04x}:{:08x}", loc.lcid & 0xFFFF, loc.lcid & 0xFFFF),
        0x60..=0x66 => {
            let win_d = (t - 0x60) as i32;
            let icu_d = ((win_d + 1) % 7) + 1;
            date_sym(icu::UDAT_SHORTER_WEEKDAYS, icu_d)
                .or_else(|| date_sym(icu::UDAT_SHORT_WEEKDAYS, icu_d))
                .unwrap_or_default()
        }
        0x67 => unsafe {
            let p = (a.uloc_get_iso3_language)(cloc.as_ptr());
            if p.is_null() { lang } else { CStr::from_ptr(p).to_string_lossy().into_owned() }
        },
        0x69 => num_sym(icu::UNUM_DECIMAL, icu::UNUM_NAN_SYMBOL).unwrap_or_else(|| "NaN".into()),
        0x6A => num_sym(icu::UNUM_DECIMAL, icu::UNUM_INFINITY_SYMBOL).unwrap_or_else(|| "Infinity".into()),
        0x6B => {
            let neg = num_sym(icu::UNUM_DECIMAL, icu::UNUM_MINUS_SIGN_SYMBOL).unwrap_or_else(|| "-".into());
            let inf = num_sym(icu::UNUM_DECIMAL, icu::UNUM_INFINITY_SYMBOL).unwrap_or_else(|| "Infinity".into());
            format!("{neg}{inf}")
        }
        0x6C => unsafe {
            let sc = icu::call_ascii_str(|b, cap, s| (a.uloc_get_script)(cloc.as_ptr(), b, cap, s))
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| match lang.as_str() {
                    "ja" => "Jpan".into(),
                    "zh" => "Hans".into(),
                    "ko" => "Kore".into(),
                    "ru" | "uk" | "bg" | "sr" => "Cyrl".into(),
                    "el" => "Grek".into(),
                    "ar" | "fa" | "ur" => "Arab".into(),
                    "he" => "Hebr".into(),
                    "th" => "Thai".into(),
                    _ => "Latn".into(),
                });
            format!("{sc};")
        },
        0x6D => unsafe {
            icu::call_ascii_str(|b, cap, s| (a.uloc_get_parent)(cloc.as_ptr(), b, cap, s))
                .map(|p| if p.is_empty() || p == "root" { String::new() } else { icu_to_bcp47(&p) })
                .unwrap_or_default()
        },
        0x6F | 0x74 | 0x1001 => unsafe {
            icu::call_u16_str(|b, cap, s| (a.uloc_get_display_language)(cloc.as_ptr(), en_us.as_ptr(), b, cap, s))
                .unwrap_or_else(|| lang.clone())
        },
        0x70 => unsafe {
            let mut st = 0i32;
            let orient = (a.uloc_get_char_orientation)(cloc.as_ptr(), &mut st);
            if st <= 0 && orient == icu::ULOC_LAYOUT_RTL { "1".into() } else { "0".into() }
        },
        0x71 => if country.is_empty() { "1".into() } else { "0".into() },
        0x73 => unsafe {
            icu::call_u16_str(|b, cap, s| (a.uloc_get_display_name)(cloc.as_ptr(), cloc.as_ptr(), b, cap, s))
                .unwrap_or_else(|| loc.bcp47.clone())
        },
        0x76 => num_sym(icu::UNUM_DECIMAL, icu::UNUM_PERCENT_SYMBOL).unwrap_or_else(|| "%".into()),
        0x77 => num_sym(icu::UNUM_DECIMAL, icu::UNUM_PERMILL_SYMBOL).unwrap_or_else(|| "\u{2030}".into()),
        0x78 => best_pat("MMMMd").unwrap_or_else(|| "MMMM d".into()),
        0x79 => date_pat(icu::UDAT_SHORT, icu::UDAT_NONE, false).unwrap_or_else(|| "h:mm tt".into()),
        0x7C => best_pat("yMMMMd").unwrap_or_else(|| "MMMM d, yyyy".into()),
        0x7E => date_sym(icu::UDAT_AM_PMS, 0).and_then(|s| s.chars().next()).map(|c| c.to_string()).unwrap_or_else(|| "A".into()),
        0x7F => date_sym(icu::UDAT_AM_PMS, 1).and_then(|s| s.chars().next()).map(|c| c.to_string()).unwrap_or_else(|| "P".into()),
        0x1003 => date_pat(icu::UDAT_MEDIUM, icu::UDAT_NONE, false).unwrap_or_else(|| "h:mm:ss tt".into()),
        0x1004 => default_ansi_cp_for(&lang, &country).to_string(),
        0x1005 => "1".into(),
        0x1006 => best_pat("yMMMM").unwrap_or_else(|| "MMMM yyyy".into()),
        0x1007 => unsafe {
            // LOCALE_SENGCURRNAME
            let mut st = 0i32;
            let mut code = [0u16; 8];
            let cn = (a.ucurr_for_locale)(cloc.as_ptr(), code.as_mut_ptr(), 8, &mut st);
            if st <= 0 && cn > 0 {
                let mut is_choice = 0i8;
                let mut len = 0i32;
                st = 0;
                let p = (a.ucurr_get_name)(code.as_ptr(), en_us.as_ptr(), 1, &mut is_choice, &mut len, &mut st);
                if !p.is_null() && st <= 0 && len > 0 {
                    String::from_utf16_lossy(std::slice::from_raw_parts(p, len as usize))
                } else {
                    num_sym(icu::UNUM_CURRENCY, icu::UNUM_INTL_CURRENCY_SYMBOL).unwrap_or_default()
                }
            } else {
                String::new()
            }
        },
        0x1008 => unsafe {
            // LOCALE_SNATIVECURRNAME
            let mut st = 0i32;
            let mut code = [0u16; 8];
            let cn = (a.ucurr_for_locale)(cloc.as_ptr(), code.as_mut_ptr(), 8, &mut st);
            if st <= 0 && cn > 0 {
                let mut is_choice = 0i8;
                let mut len = 0i32;
                st = 0;
                let p = (a.ucurr_get_name)(code.as_ptr(), cloc.as_ptr(), 1, &mut is_choice, &mut len, &mut st);
                if !p.is_null() && st <= 0 && len > 0 {
                    String::from_utf16_lossy(std::slice::from_raw_parts(p, len as usize))
                } else {
                    num_sym(icu::UNUM_CURRENCY, icu::UNUM_CURRENCY_SYMBOL).unwrap_or_default()
                }
            } else {
                String::new()
            }
        },
        0x1009 => "1".into(), // LOCALE_ICALENDARTYPE (1 = Gregorian)
        0x100A => "1".into(), // LOCALE_IOPTIONALCALENDAR
        0x100B => "1".into(), // LOCALE_IPAPERSIZE (1=Letter for US/CA, 9=A4 elsewhere)
        0x100C => unsafe {
            // LOCALE_IFIRSTDAYOFWEEK: 0=Mon..6=Sun (ICU: 1=Sun, 2=Mon..7=Sat)
            let mut st = 0i32;
            let cal = (a.ucal_open)(std::ptr::null(), 0, cloc.as_ptr(), 0, &mut st);
            if !cal.is_null() && st <= 0 {
                let fdow = (a.ucal_get_attribute)(cal, icu::UCAL_FIRST_DAY_OF_WEEK);
                (a.ucal_close)(cal);
                if (1..=7).contains(&fdow) {
                    ((fdow + 5) % 7).to_string()
                } else {
                    "0".into()
                }
            } else {
                "6".into()
            }
        },
        0x100D => "0".into(),     // LOCALE_IFIRSTWEEKOFYEAR
        0x100E => String::new(),  // LOCALE_SSORTNAME
        0x100F => "1".into(),     // LOCALE_IDIGITSUBSTITUTION / legacy
        0x1010 => "1".into(),
        0x1011 => "10000".into(), // LOCALE_IDEFAULTMACCODEPAGE
        0x1012 => "037".into(),   // LOCALE_IDEFAULTEBCDICCODEPAGE
        0x1013 => String::new(),  // LOCALE_SSORTNAME
        0x1014 => "1".into(),     // LOCALE_IDIGITSUBSTITUTION (1=None)
        0x1015..=0x1018 => String::new(),
        _ => return None,
    };
    Some(s)
}

/// en-US / invariant static locale data fallback.
pub fn locale_info(lctype: u32, invariant: bool) -> Option<Cow<'static, str>> {
    locale_info_en_or_inv(lctype, invariant)
}

fn locale_info_en_or_inv(lctype: u32, invariant: bool) -> Option<Cow<'static, str>> {
    use std::borrow::Cow::Borrowed as B;
    const DAYS: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    const MONTHS: [&str; 12] =
        ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let t = lctype & 0x0FFF_FFFF & !0x0800_0000;
    Some(match t {
        0x01 | 0x09 => B(if invariant { "007f" } else { "0409" }),
        0x02 | 0x72 => B(if invariant { "Invariant Language (Invariant Country)" } else { "English (United States)" }),
        0x03 => B(if invariant { "IVL" } else { "ENU" }),
        0x04 | 0x6F | 0x74 | 0x1001 => B(if invariant { "Invariant Language" } else { "English" }),
        0x05 | 0x0A => B("1"),
        0x06 | 0x08 | 0x75 | 0x1002 => B(if invariant { "Invariant Country" } else { "United States" }),
        0x07 | 0x68 => B(if invariant { "IVC" } else { "USA" }),
        0x0B => B("437"),
        0x0C => B(","),
        0x0D => B("1"),
        0x0E | 0x16 => B("."),
        0x0F | 0x17 => B(","),
        0x10 | 0x18 => B("3;0"),
        0x11 | 0x19 | 0x1A => B("2"),
        0x12 => B("1"),
        0x13 => B("0123456789"),
        0x14 => B(if invariant { "\u{a4}" } else { "$" }),
        0x15 => B(if invariant { "XDR" } else { "USD" }),
        0x1B | 0x1C => B("0"),
        0x1D => B("/"),
        0x1E => B(":"),
        0x1F => B(if invariant { "MM/dd/yyyy" } else { "M/d/yyyy" }),
        0x20 => B(if invariant { "dddd, dd MMMM yyyy" } else { "dddd, MMMM d, yyyy" }),
        0x21..=0x25 | 0x27 => B("0"),
        0x26 => B("1"),
        0x28 => B("AM"),
        0x29 => B("PM"),
        0x2A..=0x30 => B(DAYS[(t - 0x2A) as usize]),
        0x31..=0x37 => B(&DAYS[(t - 0x31) as usize][..3]),
        0x38..=0x43 => B(MONTHS[(t - 0x38) as usize]),
        0x44..=0x4F => B(&MONTHS[(t - 0x44) as usize][..3]),
        0x50 => B(""),
        0x51 => B("-"),
        0x52 => B("3"),
        0x53 => B("0"),
        0x54..=0x57 => B("1"),
        0x59 => B(if invariant { "iv" } else { "en" }),
        0x5A => B(if invariant { "IV" } else { "US" }),
        0x5B => B("244"),
        0x5C | 0x6E | 0x7A | 0x7B => B(if invariant { "" } else { LOCALE_NAME }),
        0x5D => B("HH:mm:ss"),
        0x5E => B("0409:00000409"),
        0x60..=0x66 => B(&DAYS[(t - 0x60) as usize][..2]),
        0x67 => B("eng"),
        0x69 => B("NaN"),
        0x6A => B("Infinity"),
        0x6B => B("-Infinity"),
        0x6C => B("Latn;"),
        0x6D => B(if invariant { "" } else { "en" }),
        0x70 | 0x71 => B("0"),
        0x73 => B(if invariant { "Invariant Language (Invariant Country)" } else { "English (United States)" }),
        0x76 => B("%"),
        0x77 => B("\u{2030}"),
        0x78 => B("MMMM d"),
        0x79 => B(if invariant { "HH:mm" } else { "h:mm tt" }),
        0x7C => B("MMMM d, yyyy"),
        0x7E => B("A"),
        0x7F => B("P"),
        0x1003 => B(if invariant { "HH:mm:ss" } else { "h:mm:ss tt" }),
        0x1004 => B("1252"),
        0x1005 => B("1"),
        0x1006 => B("MMMM yyyy"),
        0x1007 => B(if invariant { "International Monetary Fund" } else { "US Dollar" }),
        0x1008 => B(if invariant { "International Monetary Fund" } else { "US Dollar" }),
        0x1009 | 0x100A | 0x100B => B("1"),
        0x100C => B("6"),
        0x100D => B("0"),
        0x100E | 0x1013 => B(""),
        0x100F | 0x1010 | 0x1014 => B("1"),
        0x1011 => B("10000"),
        0x1012 => B("037"),
        0x1015..=0x1018 => B(""),
        _ => return None,
    })
}

const LOCALE_RETURN_NUMBER: u32 = 0x2000_0000;
const LOCALE_FONTSIGNATURE: u32 = 0x0058;

unsafe fn get_locale_info_resolved(loc: &ResolvedLocale, lctype: u32, out: *mut u16, cap: i32) -> i32 {
    if cap < 0 || (cap > 0 && out.is_null()) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let base_type = lctype & 0x0FFF_FFFF & !0x0800_0000;
    if base_type == LOCALE_FONTSIGNATURE {
        // LOCALESIGNATURE is 16 WCHARs (32 bytes): 4 DWORD USB + 2 DWORD CSB + 2 DWORD default CSB.
        if cap == 0 {
            return 16;
        }
        if cap < 16 {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            return 0;
        }
        let mut sig = [0u32; 8];
        sig[0] = 0x0000_0003; // Basic Latin + Latin-1 Supplement
        sig[4] = 0x0000_0001; // CP1252 bit
        sig[6] = 0x0000_0001;
        std::ptr::copy_nonoverlapping(sig.as_ptr() as *const u8, out as *mut u8, 32);
        return 16;
    }
    let Some(v) = locale_info_for(loc, lctype) else {
        // Documented Windows behavior: invalid/unknown LCTYPE returns 0 with ERROR_INVALID_FLAGS.
        set_err(err::ERROR_INVALID_FLAGS);
        return 0;
    };
    if (lctype & LOCALE_RETURN_NUMBER) != 0 {
        let n: u32 = if (lctype & 0xFFFF) == 0x01 || (lctype & 0xFFFF) == 0x09 {
            u32::from_str_radix(&v, 16).unwrap_or(0)
        } else {
            v.parse().unwrap_or(0)
        };
        if cap == 0 {
            return 2;
        }
        if cap < 2 {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            return 0;
        }
        (out as *mut u32).write_unaligned(n);
        return 2;
    }
    let w = to_wide_z(&v);
    if cap == 0 {
        return w.len() as i32;
    }
    if w.len() > cap as usize {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr() as *const u8, out as *mut u8, w.len() * 2);
    w.len() as i32
}

/// `Some(invariant?)` for locale names we serve.
pub unsafe fn locale_name_supported(name: LPCWSTR) -> Option<bool> {
    resolve_locale_name_w(name).map(|l| l.invariant)
}

pub unsafe extern "win64" fn GetLocaleInfoW(lcid: u32, lctype: u32, out: *mut u16, cap: i32) -> i32 {
    let Some(loc) = resolve_lcid(lcid) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    get_locale_info_resolved(&loc, lctype, out, cap)
}

pub unsafe extern "win64" fn GetLocaleInfoEx(name: LPCWSTR, lctype: u32, out: *mut u16, cap: i32) -> i32 {
    let Some(loc) = resolve_locale_name_w(name) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    get_locale_info_resolved(&loc, lctype, out, cap)
}

pub unsafe extern "win64" fn GetLocaleInfoA(lcid: u32, lctype: u32, out: *mut u8, cap: i32) -> i32 {
    if cap < 0 || (cap > 0 && out.is_null()) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let Some(loc) = resolve_lcid(lcid) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    if (lctype & LOCALE_RETURN_NUMBER) != 0 {
        let mut tmp = [0u16; 2];
        let r = get_locale_info_resolved(&loc, lctype, tmp.as_mut_ptr(), 2);
        if r == 0 {
            return 0;
        }
        if cap == 0 {
            return 4;
        }
        if cap < 4 {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            return 0;
        }
        std::ptr::copy_nonoverlapping(tmp.as_ptr() as *const u8, out, 4);
        return 4;
    }
    let Some(v) = locale_info_for(&loc, lctype) else {
        set_err(err::ERROR_INVALID_FLAGS);
        return 0;
    };
    const LOCALE_USE_CP_ACP: u32 = 0x4000_0000;
    let cp = if (lctype & LOCALE_USE_CP_ACP) != 0 {
        ACP
    } else {
        locale_info_for(&loc, 0x1004)
            .and_then(|s| s.parse::<u32>().ok())
            .filter(|&c| c != 0 && valid_cp(c))
            .unwrap_or(ACP)
    };
    let encoded: Vec<u8> = if cp == 65001 {
        v.into_bytes()
    } else {
        let units = to_wide(&v);
        encode_utf16_to_cp(cp, &units, Some(b'?'))
            .map(|(b, _)| b)
            .unwrap_or_else(|_| v.into_bytes())
    };
    let need = encoded.len() as i32 + 1;
    if cap == 0 {
        return need;
    }
    if need > cap {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    if !encoded.is_empty() {
        std::ptr::copy_nonoverlapping(encoded.as_ptr(), out, encoded.len());
    }
    *out.add(encoded.len()) = 0;
    need
}

std::thread_local! {
    static THREAD_LOCALE: std::cell::Cell<u32> = const { std::cell::Cell::new(LCID_EN_US) };
}

pub unsafe extern "win64" fn GetUserDefaultLCID() -> u32 {
    LCID_EN_US
}
pub unsafe extern "win64" fn GetSystemDefaultLCID() -> u32 {
    LCID_EN_US
}
pub unsafe extern "win64" fn GetThreadLocale() -> u32 {
    THREAD_LOCALE.with(|c| c.get())
}
pub unsafe extern "win64" fn SetThreadLocale(l: u32) -> BOOL {
    let Some(loc) = resolve_lcid(l) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    THREAD_LOCALE.with(|c| c.set(loc.lcid));
    TRUE
}
pub unsafe extern "win64" fn GetUserDefaultLangID() -> u16 {
    LCID_EN_US as u16
}
pub unsafe extern "win64" fn GetSystemDefaultLangID() -> u16 {
    LCID_EN_US as u16
}
pub unsafe extern "win64" fn GetUserDefaultUILanguage() -> u16 {
    LCID_EN_US as u16
}
pub unsafe extern "win64" fn GetSystemDefaultUILanguage() -> u16 {
    LCID_EN_US as u16
}
pub unsafe extern "win64" fn GetThreadUILanguage() -> u16 {
    LCID_EN_US as u16
}
pub unsafe extern "win64" fn SetThreadUILanguage(l: u16) -> u16 {
    l
}

unsafe fn put_name(out: *mut u16, cap: i32, s: &str) -> i32 {
    let w = to_wide_z(s);
    if cap == 0 {
        return w.len() as i32;
    }
    if out.is_null() || (cap as usize) < w.len() {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr() as *const u8, out as *mut u8, w.len() * 2);
    w.len() as i32
}

pub unsafe extern "win64" fn GetUserDefaultLocaleName(out: *mut u16, cap: i32) -> i32 {
    put_name(out, cap, LOCALE_NAME)
}
pub unsafe extern "win64" fn GetSystemDefaultLocaleName(out: *mut u16, cap: i32) -> i32 {
    put_name(out, cap, LOCALE_NAME)
}

pub unsafe extern "win64" fn LCIDToLocaleName(lcid: u32, out: *mut u16, cap: i32, _flags: u32) -> i32 {
    match resolve_lcid(lcid) {
        Some(loc) => put_name(out, cap, &loc.bcp47),
        None => {
            set_err(err::ERROR_INVALID_PARAMETER);
            0
        }
    }
}

pub unsafe extern "win64" fn LocaleNameToLCID(name: LPCWSTR, _flags: u32) -> u32 {
    match resolve_locale_name_w(name) {
        Some(loc) => loc.lcid,
        None => {
            set_err(err::ERROR_INVALID_PARAMETER);
            0
        }
    }
}

pub unsafe extern "win64" fn ResolveLocaleName(name: LPCWSTR, out: *mut u16, cap: i32) -> i32 {
    match resolve_locale_name_w(name) {
        Some(loc) => put_name(out, cap, &loc.bcp47),
        None => put_name(out, cap, LOCALE_NAME),
    }
}

pub unsafe extern "win64" fn IsValidLocale(lcid: u32, _flags: u32) -> BOOL {
    resolve_lcid(lcid).is_some() as BOOL
}
pub unsafe extern "win64" fn IsValidLocaleName(name: LPCWSTR) -> BOOL {
    resolve_locale_name_w(name).is_some() as BOOL
}

type LocaleEnumProcW = unsafe extern "win64" fn(*mut u16) -> BOOL;
type LocaleEnumProcEx = unsafe extern "win64" fn(*mut u16, u32, isize) -> BOOL;

pub unsafe extern "win64" fn EnumSystemLocalesW(f: Option<LocaleEnumProcW>, _flags: u32) -> BOOL {
    if let Some(f) = f {
        if let Some(a) = icu::api() {
            let count = (a.uloc_count_available)();
            let mut seen = std::collections::HashSet::new();
            for i in 0..count {
                let p = (a.uloc_get_available)(i);
                if p.is_null() {
                    continue;
                }
                let lcid = (a.uloc_get_lcid)(p);
                if lcid != 0 && seen.insert(lcid) {
                    let mut s = to_wide_z(&format!("{lcid:08x}"));
                    if crate::call_guest8(f as usize, s.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0) == 0 {
                        break;
                    }
                }
            }
            return TRUE;
        }
        let mut s = to_wide_z("00000409");
        crate::call_guest8(f as usize, s.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0);
    }
    TRUE
}
pub unsafe extern "win64" fn EnumSystemLocalesA(f: Option<unsafe extern "win64" fn(*mut u8) -> BOOL>, _flags: u32) -> BOOL {
    if let Some(f) = f {
        if let Some(a) = icu::api() {
            let count = (a.uloc_count_available)();
            let mut seen = std::collections::HashSet::new();
            for i in 0..count {
                let p = (a.uloc_get_available)(i);
                if p.is_null() {
                    continue;
                }
                let lcid = (a.uloc_get_lcid)(p);
                if lcid != 0 && seen.insert(lcid) {
                    let mut s = format!("{lcid:08x}\0").into_bytes();
                    if crate::call_guest8(f as usize, s.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0) == 0 {
                        break;
                    }
                }
            }
            return TRUE;
        }
        let mut s = *b"00000409\0";
        crate::call_guest8(f as usize, s.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0);
    }
    TRUE
}
pub unsafe extern "win64" fn EnumSystemLocalesEx(f: Option<LocaleEnumProcEx>, _flags: u32, lparam: isize, _r: *mut c_void) -> BOOL {
    if let Some(f) = f {
        if let Some(a) = icu::api() {
            let count = (a.uloc_count_available)();
            for i in 0..count {
                let p = (a.uloc_get_available)(i);
                if p.is_null() {
                    continue;
                }
                let icu_id = CStr::from_ptr(p).to_string_lossy();
                let tag = icu_to_bcp47(&icu_id);
                let mut s = to_wide_z(&tag);
                if crate::call_guest8(f as usize, s.as_mut_ptr() as u64, 1, lparam as u64, 0, 0, 0, 0, 0) == 0 {
                    break;
                }
            }
            return TRUE;
        }
        let mut s = to_wide_z(LOCALE_NAME);
        crate::call_guest8(f as usize, s.as_mut_ptr() as u64, 1, lparam as u64, 0, 0, 0, 0, 0);
    }
    TRUE
}

/// `MUI_LANGUAGE_NAME` multi-string ("en-US\0\0") for the *PreferredUILanguages family.
unsafe fn ui_languages(flags: u32, num: *mut u32, buf: *mut u16, size: *mut u32) -> BOOL {
    let s: Vec<u16> = if flags & 0x4 != 0 {
        // MUI_LANGUAGE_ID
        to_wide("0409").into_iter().chain([0, 0]).collect()
    } else {
        to_wide(LOCALE_NAME).into_iter().chain([0, 0]).collect()
    };
    if !num.is_null() {
        *num = 1;
    }
    if buf.is_null() || *size == 0 {
        *size = s.len() as u32;
        return TRUE;
    }
    if (*size as usize) < s.len() {
        *size = s.len() as u32;
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(s.as_ptr(), buf, s.len());
    *size = s.len() as u32;
    TRUE
}

pub unsafe extern "win64" fn GetUserPreferredUILanguages(flags: u32, num: *mut u32, buf: *mut u16, size: *mut u32) -> BOOL {
    ui_languages(flags, num, buf, size)
}
pub unsafe extern "win64" fn GetSystemPreferredUILanguages(flags: u32, num: *mut u32, buf: *mut u16, size: *mut u32) -> BOOL {
    ui_languages(flags, num, buf, size)
}
pub unsafe extern "win64" fn GetThreadPreferredUILanguages(flags: u32, num: *mut u32, buf: *mut u16, size: *mut u32) -> BOOL {
    ui_languages(flags, num, buf, size)
}
pub unsafe extern "win64" fn GetProcessPreferredUILanguages(flags: u32, num: *mut u32, buf: *mut u16, size: *mut u32) -> BOOL {
    ui_languages(flags, num, buf, size)
}
pub unsafe extern "win64" fn SetProcessPreferredUILanguages(_f: u32, _b: LPCWSTR, _n: *mut u32) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetThreadPreferredUILanguages(_f: u32, _b: LPCWSTR, _n: *mut u32) -> BOOL {
    TRUE
}

// ---------------------------------------------------------------------------
// Geographical location table (Microsoft Learn: Table of Geographical Locations)
// ---------------------------------------------------------------------------

struct GeoEntry {
    geoid: i32,
    iso2: &'static str,
    un_num: &'static str,
    dialing: &'static str,
    default_lang: &'static str,
    lat: &'static str,
    lon: &'static str,
}

const GEO_TABLE: &[GeoEntry] = &[
    GeoEntry { geoid: 2, iso2: "AG", un_num: "028", dialing: "1", default_lang: "en", lat: "17.078", lon: "-61.807" },
    GeoEntry { geoid: 3, iso2: "AF", un_num: "004", dialing: "93", default_lang: "ps", lat: "33.831", lon: "66.026" },
    GeoEntry { geoid: 4, iso2: "DZ", un_num: "012", dialing: "213", default_lang: "ar", lat: "28.163", lon: "2.632" },
    GeoEntry { geoid: 5, iso2: "AZ", un_num: "031", dialing: "994", default_lang: "az", lat: "40.288", lon: "47.532" },
    GeoEntry { geoid: 6, iso2: "AL", un_num: "008", dialing: "355", default_lang: "sq", lat: "41.145", lon: "20.069" },
    GeoEntry { geoid: 7, iso2: "AM", un_num: "051", dialing: "374", default_lang: "hy", lat: "40.289", lon: "44.948" },
    GeoEntry { geoid: 8, iso2: "AD", un_num: "020", dialing: "376", default_lang: "ca", lat: "42.541", lon: "1.576" },
    GeoEntry { geoid: 9, iso2: "AO", un_num: "024", dialing: "244", default_lang: "pt", lat: "-12.296", lon: "17.544" },
    GeoEntry { geoid: 10, iso2: "AS", un_num: "016", dialing: "1", default_lang: "en", lat: "-14.307", lon: "-170.718" },
    GeoEntry { geoid: 11, iso2: "AR", un_num: "032", dialing: "54", default_lang: "es", lat: "-35.376", lon: "-65.167" },
    GeoEntry { geoid: 12, iso2: "AU", un_num: "036", dialing: "61", default_lang: "en", lat: "-25.730", lon: "134.491" },
    GeoEntry { geoid: 14, iso2: "AT", un_num: "040", dialing: "43", default_lang: "de", lat: "47.588", lon: "14.142" },
    GeoEntry { geoid: 17, iso2: "BH", un_num: "048", dialing: "973", default_lang: "ar", lat: "26.019", lon: "50.562" },
    GeoEntry { geoid: 18, iso2: "BB", un_num: "052", dialing: "1", default_lang: "en", lat: "13.180", lon: "-59.559" },
    GeoEntry { geoid: 19, iso2: "BW", un_num: "072", dialing: "267", default_lang: "en", lat: "-22.186", lon: "23.814" },
    GeoEntry { geoid: 20, iso2: "BM", un_num: "060", dialing: "1", default_lang: "en", lat: "32.313", lon: "-64.756" },
    GeoEntry { geoid: 21, iso2: "BE", un_num: "056", dialing: "32", default_lang: "nl", lat: "50.643", lon: "4.664" },
    GeoEntry { geoid: 22, iso2: "BS", un_num: "044", dialing: "1", default_lang: "en", lat: "24.628", lon: "-78.034" },
    GeoEntry { geoid: 23, iso2: "BD", un_num: "050", dialing: "880", default_lang: "bn", lat: "23.845", lon: "90.281" },
    GeoEntry { geoid: 24, iso2: "BZ", un_num: "084", dialing: "501", default_lang: "en", lat: "17.219", lon: "-88.703" },
    GeoEntry { geoid: 25, iso2: "BA", un_num: "070", dialing: "387", default_lang: "bs", lat: "44.168", lon: "17.787" },
    GeoEntry { geoid: 26, iso2: "BO", un_num: "068", dialing: "591", default_lang: "es", lat: "-16.715", lon: "-64.666" },
    GeoEntry { geoid: 27, iso2: "MM", un_num: "104", dialing: "95", default_lang: "my", lat: "21.154", lon: "96.506" },
    GeoEntry { geoid: 28, iso2: "BJ", un_num: "204", dialing: "229", default_lang: "fr", lat: "9.647", lon: "2.339" },
    GeoEntry { geoid: 29, iso2: "BY", un_num: "112", dialing: "375", default_lang: "be", lat: "53.538", lon: "28.046" },
    GeoEntry { geoid: 32, iso2: "BR", un_num: "076", dialing: "55", default_lang: "pt", lat: "-10.773", lon: "-53.090" },
    GeoEntry { geoid: 34, iso2: "BT", un_num: "064", dialing: "975", default_lang: "dz", lat: "27.415", lon: "90.429" },
    GeoEntry { geoid: 35, iso2: "BG", un_num: "100", dialing: "359", default_lang: "bg", lat: "42.761", lon: "25.231" },
    GeoEntry { geoid: 37, iso2: "BN", un_num: "096", dialing: "673", default_lang: "ms", lat: "4.525", lon: "114.754" },
    GeoEntry { geoid: 39, iso2: "CA", un_num: "124", dialing: "1", default_lang: "en", lat: "62.832", lon: "-95.913" },
    GeoEntry { geoid: 40, iso2: "KH", un_num: "116", dialing: "855", default_lang: "km", lat: "12.714", lon: "104.923" },
    GeoEntry { geoid: 42, iso2: "LK", un_num: "144", dialing: "94", default_lang: "si", lat: "7.612", lon: "80.703" },
    GeoEntry { geoid: 45, iso2: "CN", un_num: "156", dialing: "86", default_lang: "zh", lat: "36.553", lon: "103.975" },
    GeoEntry { geoid: 46, iso2: "CL", un_num: "152", dialing: "56", default_lang: "es", lat: "-37.743", lon: "-71.364" },
    GeoEntry { geoid: 49, iso2: "CM", un_num: "120", dialing: "237", default_lang: "fr", lat: "5.689", lon: "12.724" },
    GeoEntry { geoid: 51, iso2: "CO", un_num: "170", dialing: "57", default_lang: "es", lat: "3.900", lon: "-73.077" },
    GeoEntry { geoid: 54, iso2: "CR", un_num: "188", dialing: "506", default_lang: "es", lat: "9.971", lon: "-84.192" },
    GeoEntry { geoid: 56, iso2: "CU", un_num: "192", dialing: "53", default_lang: "es", lat: "21.622", lon: "-79.035" },
    GeoEntry { geoid: 59, iso2: "CY", un_num: "196", dialing: "357", default_lang: "el", lat: "35.043", lon: "33.219" },
    GeoEntry { geoid: 61, iso2: "DK", un_num: "208", dialing: "45", default_lang: "da", lat: "55.963", lon: "10.046" },
    GeoEntry { geoid: 65, iso2: "DO", un_num: "214", dialing: "1", default_lang: "es", lat: "18.895", lon: "-70.493" },
    GeoEntry { geoid: 66, iso2: "EC", un_num: "218", dialing: "593", default_lang: "es", lat: "-1.425", lon: "-78.765" },
    GeoEntry { geoid: 67, iso2: "EG", un_num: "818", dialing: "20", default_lang: "ar", lat: "26.494", lon: "29.871" },
    GeoEntry { geoid: 68, iso2: "IE", un_num: "372", dialing: "353", default_lang: "en", lat: "53.175", lon: "-8.151" },
    GeoEntry { geoid: 70, iso2: "EE", un_num: "233", dialing: "372", default_lang: "et", lat: "58.674", lon: "25.542" },
    GeoEntry { geoid: 72, iso2: "SV", un_num: "222", dialing: "503", default_lang: "es", lat: "13.736", lon: "-88.867" },
    GeoEntry { geoid: 73, iso2: "ET", un_num: "231", dialing: "251", default_lang: "am", lat: "8.626", lon: "39.637" },
    GeoEntry { geoid: 75, iso2: "CZ", un_num: "203", dialing: "420", default_lang: "cs", lat: "49.742", lon: "15.338" },
    GeoEntry { geoid: 77, iso2: "FI", un_num: "246", dialing: "358", default_lang: "fi", lat: "64.504", lon: "26.211" },
    GeoEntry { geoid: 78, iso2: "FJ", un_num: "242", dialing: "679", default_lang: "en", lat: "-17.819", lon: "177.973" },
    GeoEntry { geoid: 84, iso2: "FR", un_num: "250", dialing: "33", default_lang: "fr", lat: "46.564", lon: "2.552" },
    GeoEntry { geoid: 88, iso2: "GE", un_num: "268", dialing: "995", default_lang: "ka", lat: "42.176", lon: "43.517" },
    GeoEntry { geoid: 89, iso2: "GH", un_num: "288", dialing: "233", default_lang: "en", lat: "7.960", lon: "-1.207" },
    GeoEntry { geoid: 90, iso2: "GI", un_num: "292", dialing: "350", default_lang: "en", lat: "36.132", lon: "-5.353" },
    GeoEntry { geoid: 93, iso2: "GL", un_num: "304", dialing: "299", default_lang: "kl", lat: "74.710", lon: "-41.391" },
    GeoEntry { geoid: 94, iso2: "DE", un_num: "276", dialing: "49", default_lang: "de", lat: "51.106", lon: "10.381" },
    GeoEntry { geoid: 98, iso2: "GR", un_num: "300", dialing: "30", default_lang: "el", lat: "39.075", lon: "22.955" },
    GeoEntry { geoid: 99, iso2: "GT", un_num: "320", dialing: "502", default_lang: "es", lat: "15.695", lon: "-90.359" },
    GeoEntry { geoid: 103, iso2: "HT", un_num: "332", dialing: "509", default_lang: "fr", lat: "18.935", lon: "-72.685" },
    GeoEntry { geoid: 104, iso2: "HK", un_num: "344", dialing: "852", default_lang: "zh", lat: "22.352", lon: "114.139" },
    GeoEntry { geoid: 106, iso2: "HN", un_num: "340", dialing: "504", default_lang: "es", lat: "14.819", lon: "-86.615" },
    GeoEntry { geoid: 108, iso2: "HR", un_num: "191", dialing: "385", default_lang: "hr", lat: "45.075", lon: "16.405" },
    GeoEntry { geoid: 109, iso2: "HU", un_num: "348", dialing: "36", default_lang: "hu", lat: "47.161", lon: "19.395" },
    GeoEntry { geoid: 110, iso2: "IS", un_num: "352", dialing: "354", default_lang: "is", lat: "64.997", lon: "-18.570" },
    GeoEntry { geoid: 111, iso2: "ID", un_num: "360", dialing: "62", default_lang: "id", lat: "-2.222", lon: "117.240" },
    GeoEntry { geoid: 113, iso2: "IN", un_num: "356", dialing: "91", default_lang: "hi", lat: "22.758", lon: "79.566" },
    GeoEntry { geoid: 116, iso2: "IR", un_num: "364", dialing: "98", default_lang: "fa", lat: "32.565", lon: "54.274" },
    GeoEntry { geoid: 117, iso2: "IL", un_num: "376", dialing: "972", default_lang: "he", lat: "31.448", lon: "35.060" },
    GeoEntry { geoid: 118, iso2: "IT", un_num: "380", dialing: "39", default_lang: "it", lat: "42.791", lon: "12.078" },
    GeoEntry { geoid: 121, iso2: "IQ", un_num: "368", dialing: "964", default_lang: "ar", lat: "33.048", lon: "43.772" },
    GeoEntry { geoid: 122, iso2: "JP", un_num: "392", dialing: "81", default_lang: "ja", lat: "37.549", lon: "137.968" },
    GeoEntry { geoid: 124, iso2: "JM", un_num: "388", dialing: "1", default_lang: "en", lat: "18.151", lon: "-77.319" },
    GeoEntry { geoid: 126, iso2: "JO", un_num: "400", dialing: "962", default_lang: "ar", lat: "31.245", lon: "36.783" },
    GeoEntry { geoid: 129, iso2: "KE", un_num: "404", dialing: "254", default_lang: "sw", lat: "0.530", lon: "37.858" },
    GeoEntry { geoid: 130, iso2: "KG", un_num: "417", dialing: "996", default_lang: "ky", lat: "41.465", lon: "74.555" },
    GeoEntry { geoid: 131, iso2: "KP", un_num: "408", dialing: "850", default_lang: "ko", lat: "40.143", lon: "127.187" },
    GeoEntry { geoid: 134, iso2: "KR", un_num: "410", dialing: "82", default_lang: "ko", lat: "36.369", lon: "127.821" },
    GeoEntry { geoid: 136, iso2: "KW", un_num: "414", dialing: "965", default_lang: "ar", lat: "29.349", lon: "47.590" },
    GeoEntry { geoid: 137, iso2: "KZ", un_num: "398", dialing: "7", default_lang: "kk", lat: "48.160", lon: "67.204" },
    GeoEntry { geoid: 138, iso2: "LA", un_num: "418", dialing: "856", default_lang: "lo", lat: "18.501", lon: "103.765" },
    GeoEntry { geoid: 139, iso2: "LB", un_num: "422", dialing: "961", default_lang: "ar", lat: "33.920", lon: "35.888" },
    GeoEntry { geoid: 140, iso2: "LV", un_num: "428", dialing: "371", default_lang: "lv", lat: "56.854", lon: "24.929" },
    GeoEntry { geoid: 141, iso2: "LT", un_num: "440", dialing: "370", default_lang: "lt", lat: "55.336", lon: "23.897" },
    GeoEntry { geoid: 143, iso2: "SK", un_num: "703", dialing: "421", default_lang: "sk", lat: "48.707", lon: "19.486" },
    GeoEntry { geoid: 145, iso2: "LI", un_num: "438", dialing: "423", default_lang: "de", lat: "47.152", lon: "9.555" },
    GeoEntry { geoid: 147, iso2: "LU", un_num: "442", dialing: "352", default_lang: "fr", lat: "49.771", lon: "6.095" },
    GeoEntry { geoid: 148, iso2: "LY", un_num: "434", dialing: "218", default_lang: "ar", lat: "27.044", lon: "18.023" },
    GeoEntry { geoid: 151, iso2: "MO", un_num: "446", dialing: "853", default_lang: "zh", lat: "22.157", lon: "113.559" },
    GeoEntry { geoid: 152, iso2: "MD", un_num: "498", dialing: "373", default_lang: "ro", lat: "47.199", lon: "28.465" },
    GeoEntry { geoid: 154, iso2: "MN", un_num: "496", dialing: "976", default_lang: "mn", lat: "46.836", lon: "103.067" },
    GeoEntry { geoid: 158, iso2: "MC", un_num: "492", dialing: "377", default_lang: "fr", lat: "43.738", lon: "7.425" },
    GeoEntry { geoid: 159, iso2: "MA", un_num: "504", dialing: "212", default_lang: "ar", lat: "31.838", lon: "-6.374" },
    GeoEntry { geoid: 163, iso2: "MT", un_num: "470", dialing: "356", default_lang: "mt", lat: "35.888", lon: "14.442" },
    GeoEntry { geoid: 164, iso2: "OM", un_num: "512", dialing: "968", default_lang: "ar", lat: "20.605", lon: "56.095" },
    GeoEntry { geoid: 166, iso2: "MX", un_num: "484", dialing: "52", default_lang: "es", lat: "23.950", lon: "-102.533" },
    GeoEntry { geoid: 167, iso2: "MY", un_num: "458", dialing: "60", default_lang: "ms", lat: "3.788", lon: "102.313" },
    GeoEntry { geoid: 175, iso2: "NG", un_num: "566", dialing: "234", default_lang: "en", lat: "9.593", lon: "8.105" },
    GeoEntry { geoid: 176, iso2: "NL", un_num: "528", dialing: "31", default_lang: "nl", lat: "52.247", lon: "5.512" },
    GeoEntry { geoid: 177, iso2: "NO", un_num: "578", dialing: "47", default_lang: "nb", lat: "64.455", lon: "11.503" },
    GeoEntry { geoid: 178, iso2: "NP", un_num: "524", dialing: "977", default_lang: "ne", lat: "28.253", lon: "83.944" },
    GeoEntry { geoid: 182, iso2: "NI", un_num: "558", dialing: "505", default_lang: "es", lat: "12.840", lon: "-85.035" },
    GeoEntry { geoid: 183, iso2: "NZ", un_num: "554", dialing: "64", default_lang: "en", lat: "-42.633", lon: "172.371" },
    GeoEntry { geoid: 185, iso2: "PY", un_num: "600", dialing: "595", default_lang: "es", lat: "-23.236", lon: "-58.391" },
    GeoEntry { geoid: 187, iso2: "PE", un_num: "604", dialing: "51", default_lang: "es", lat: "-9.164", lon: "-74.375" },
    GeoEntry { geoid: 190, iso2: "PK", un_num: "586", dialing: "92", default_lang: "ur", lat: "29.967", lon: "69.386" },
    GeoEntry { geoid: 191, iso2: "PL", un_num: "616", dialing: "48", default_lang: "pl", lat: "52.125", lon: "19.399" },
    GeoEntry { geoid: 192, iso2: "PA", un_num: "591", dialing: "507", default_lang: "es", lat: "8.517", lon: "-80.112" },
    GeoEntry { geoid: 193, iso2: "PT", un_num: "620", dialing: "351", default_lang: "pt", lat: "39.599", lon: "-8.501" },
    GeoEntry { geoid: 197, iso2: "QA", un_num: "634", dialing: "974", default_lang: "ar", lat: "25.315", lon: "51.191" },
    GeoEntry { geoid: 200, iso2: "RO", un_num: "642", dialing: "40", default_lang: "ro", lat: "45.844", lon: "24.969" },
    GeoEntry { geoid: 201, iso2: "PH", un_num: "608", dialing: "63", default_lang: "en", lat: "11.772", lon: "122.879" },
    GeoEntry { geoid: 202, iso2: "PR", un_num: "630", dialing: "1", default_lang: "es", lat: "18.221", lon: "-66.414" },
    GeoEntry { geoid: 203, iso2: "RU", un_num: "643", dialing: "7", default_lang: "ru", lat: "61.988", lon: "96.689" },
    GeoEntry { geoid: 205, iso2: "SA", un_num: "682", dialing: "966", default_lang: "ar", lat: "24.127", lon: "44.544" },
    GeoEntry { geoid: 209, iso2: "ZA", un_num: "710", dialing: "27", default_lang: "en", lat: "-28.996", lon: "25.085" },
    GeoEntry { geoid: 212, iso2: "SI", un_num: "705", dialing: "386", default_lang: "sl", lat: "46.120", lon: "14.817" },
    GeoEntry { geoid: 215, iso2: "SG", un_num: "702", dialing: "65", default_lang: "en", lat: "1.351", lon: "103.817" },
    GeoEntry { geoid: 217, iso2: "ES", un_num: "724", dialing: "34", default_lang: "es", lat: "40.227", lon: "-3.646" },
    GeoEntry { geoid: 221, iso2: "SE", un_num: "752", dialing: "46", default_lang: "sv", lat: "62.785", lon: "16.749" },
    GeoEntry { geoid: 222, iso2: "SY", un_num: "760", dialing: "963", default_lang: "ar", lat: "35.013", lon: "38.506" },
    GeoEntry { geoid: 223, iso2: "CH", un_num: "756", dialing: "41", default_lang: "de", lat: "46.798", lon: "8.234" },
    GeoEntry { geoid: 224, iso2: "AE", un_num: "784", dialing: "971", default_lang: "ar", lat: "23.913", lon: "54.329" },
    GeoEntry { geoid: 225, iso2: "TT", un_num: "780", dialing: "1", default_lang: "en", lat: "10.428", lon: "-61.264" },
    GeoEntry { geoid: 227, iso2: "TH", un_num: "764", dialing: "66", default_lang: "th", lat: "15.107", lon: "101.007" },
    GeoEntry { geoid: 228, iso2: "TJ", un_num: "762", dialing: "992", default_lang: "tg", lat: "38.530", lon: "71.031" },
    GeoEntry { geoid: 234, iso2: "TN", un_num: "788", dialing: "216", default_lang: "ar", lat: "34.111", lon: "9.555" },
    GeoEntry { geoid: 235, iso2: "TR", un_num: "792", dialing: "90", default_lang: "tr", lat: "39.060", lon: "35.180" },
    GeoEntry { geoid: 237, iso2: "TW", un_num: "158", dialing: "886", default_lang: "zh", lat: "23.753", lon: "120.955" },
    GeoEntry { geoid: 238, iso2: "TM", un_num: "795", dialing: "993", default_lang: "tk", lat: "39.122", lon: "59.376" },
    GeoEntry { geoid: 241, iso2: "UA", un_num: "804", dialing: "380", default_lang: "uk", lat: "49.016", lon: "31.383" },
    GeoEntry { geoid: 242, iso2: "GB", un_num: "826", dialing: "44", default_lang: "en", lat: "54.150", lon: "-2.888" },
    GeoEntry { geoid: 244, iso2: "US", un_num: "840", dialing: "1", default_lang: "en", lat: "39.450", lon: "-98.908" },
    GeoEntry { geoid: 246, iso2: "UY", un_num: "858", dialing: "598", default_lang: "es", lat: "-32.799", lon: "-56.013" },
    GeoEntry { geoid: 247, iso2: "UZ", un_num: "860", dialing: "998", default_lang: "uz", lat: "41.750", lon: "63.170" },
    GeoEntry { geoid: 249, iso2: "VE", un_num: "862", dialing: "58", default_lang: "es", lat: "7.125", lon: "-66.166" },
    GeoEntry { geoid: 251, iso2: "VN", un_num: "704", dialing: "84", default_lang: "vi", lat: "16.647", lon: "106.298" },
    GeoEntry { geoid: 261, iso2: "YE", un_num: "887", dialing: "967", default_lang: "ar", lat: "15.905", lon: "47.590" },
    GeoEntry { geoid: 263, iso2: "ZW", un_num: "716", dialing: "263", default_lang: "en", lat: "-19.000", lon: "29.869" },
    GeoEntry { geoid: 271, iso2: "RS", un_num: "688", dialing: "381", default_lang: "sr", lat: "44.205", lon: "20.790" },
    GeoEntry { geoid: 270, iso2: "ME", un_num: "499", dialing: "382", default_lang: "sr", lat: "42.789", lon: "19.245" },
    GeoEntry { geoid: 19618, iso2: "MK", un_num: "807", dialing: "389", default_lang: "mk", lat: "41.600", lon: "21.700" },
];

fn geo_by_id(geo: i32) -> Option<&'static GeoEntry> {
    GEO_TABLE.iter().find(|e| e.geoid == geo)
}

fn geo_by_iso2(iso2: &str) -> Option<&'static GeoEntry> {
    GEO_TABLE.iter().find(|e| e.iso2.eq_ignore_ascii_case(iso2))
}

pub unsafe extern "win64" fn GetUserGeoID(_class: u32) -> i32 {
    244 // United States
}

pub unsafe extern "win64" fn GetGeoInfoW(geo: i32, ty: u32, out: *mut u16, cap: i32, lang: u16) -> i32 {
    if cap < 0 || (cap > 0 && out.is_null()) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let Some(entry) = geo_by_id(geo) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let region_icu = format!("{}_{}", entry.default_lang, entry.iso2);
    let disp_icu = if lang != 0 {
        resolve_lcid(lang as u32).map(|l| l.icu_id).unwrap_or_else(|| "en_US".into())
    } else {
        "en_US".into()
    };
    let region_loc = resolve_locale_tag(&format!("{}-{}", entry.default_lang, entry.iso2))
        .unwrap_or_else(|| ResolvedLocale {
            invariant: false,
            lcid: LCID_EN_US,
            icu_id: region_icu.clone(),
            bcp47: format!("{}-{}", entry.default_lang, entry.iso2),
        });
    let val: String = match ty {
        1 => entry.geoid.to_string(),                  // GEO_NATION
        2 => entry.lat.into(),                         // GEO_LATITUDE
        3 => entry.lon.into(),                         // GEO_LONGITUDE
        4 => entry.iso2.into(),                        // GEO_ISO2
        5 => {
            // GEO_ISO3
            if let Some(a) = icu::api() {
                let c = icu::c_str(&region_icu);
                let p = (a.uloc_get_iso3_country)(c.as_ptr());
                if !p.is_null() {
                    CStr::from_ptr(p).to_string_lossy().to_ascii_uppercase()
                } else {
                    entry.iso2.into()
                }
            } else {
                entry.iso2.into()
            }
        }
        6 => region_loc.bcp47.clone(),                 // GEO_RFC1766
        7 => format!("{:08x}", region_loc.lcid),       // GEO_LCID
        8 | 9 => {
            // GEO_FRIENDLYNAME / GEO_OFFICIALNAME localized to `lang` via ICU
            if let Some(a) = icu::api() {
                let c_reg = icu::c_str(&region_icu);
                let c_disp = icu::c_str(&disp_icu);
                icu::call_u16_str(|b, c, st| (a.uloc_get_display_country)(c_reg.as_ptr(), c_disp.as_ptr(), b, c, st))
                    .unwrap_or_else(|| entry.iso2.into())
            } else {
                entry.iso2.into()
            }
        }
        10 => String::new(),                           // GEO_TIMEZONES
        11 => entry.default_lang.into(),               // GEO_OFFICIALLANGUAGES
        12 => entry.un_num.into(),                     // GEO_ISO_UN_NUMBER
        13 => String::new(),                           // GEO_PARENT
        14 => entry.dialing.into(),                    // GEO_DIALINGCODE
        15 => locale_info_for(&region_loc, 0x15).unwrap_or_default(), // GEO_CURRENCYCODE
        16 => locale_info_for(&region_loc, 0x14).unwrap_or_default(), // GEO_CURRENCYSYMBOL
        17 => entry.iso2.into(),                       // GEO_NAME
        18 => entry.geoid.to_string(),                 // GEO_ID
        _ => {
            set_err(err::ERROR_INVALID_FLAGS);
            return 0;
        }
    };
    put_name(out, cap, &val)
}

// ---------------------------------------------------------------------------
// Date / time formatting
// ---------------------------------------------------------------------------

fn format_picture(pic: &str, st: &SYSTEMTIME) -> String {
    const DAYS: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    const MONTHS: [&str; 12] =
        ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let chars: Vec<char> = pic.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' {
            i += 1;
            while i < chars.len() {
                if chars[i] == '\'' {
                    if i + 1 < chars.len() && chars[i + 1] == '\'' {
                        out.push('\'');
                        i += 2;
                        continue;
                    }
                    break;
                }
                out.push(chars[i]);
                i += 1;
            }
            i += 1;
            continue;
        }
        let mut n = 1;
        while i + n < chars.len() && chars[i + n] == c {
            n += 1;
        }
        let h12 = if st.wHour % 12 == 0 { 12 } else { st.wHour % 12 };
        match c {
            'd' => match n {
                1 => out.push_str(&st.wDay.to_string()),
                2 => out.push_str(&format!("{:02}", st.wDay)),
                3 => out.push_str(&DAYS[st.wDayOfWeek as usize % 7][..3]),
                _ => out.push_str(DAYS[st.wDayOfWeek as usize % 7]),
            },
            'M' => {
                let m = (st.wMonth.max(1) - 1) as usize % 12;
                match n {
                    1 => out.push_str(&st.wMonth.to_string()),
                    2 => out.push_str(&format!("{:02}", st.wMonth)),
                    3 => out.push_str(&MONTHS[m][..3]),
                    _ => out.push_str(MONTHS[m]),
                }
            }
            'y' => match n {
                1 => out.push_str(&(st.wYear % 100).to_string()),
                2 => out.push_str(&format!("{:02}", st.wYear % 100)),
                _ => out.push_str(&format!("{:04}", st.wYear)),
            },
            'g' => out.push_str("A.D."),
            'h' => out.push_str(&if n == 1 { h12.to_string() } else { format!("{h12:02}") }),
            'H' => out.push_str(&if n == 1 { st.wHour.to_string() } else { format!("{:02}", st.wHour) }),
            'm' => out.push_str(&if n == 1 { st.wMinute.to_string() } else { format!("{:02}", st.wMinute) }),
            's' => out.push_str(&if n == 1 { st.wSecond.to_string() } else { format!("{:02}", st.wSecond) }),
            't' => {
                let s = if st.wHour < 12 { "AM" } else { "PM" };
                out.push_str(if n == 1 { &s[..1] } else { s });
            }
            _ => {
                for _ in 0..n {
                    out.push(c);
                }
            }
        }
        i += n;
    }
    out
}

unsafe fn current_or(st: *const SYSTEMTIME) -> SYSTEMTIME {
    if st.is_null() {
        let mut s: SYSTEMTIME = std::mem::zeroed();
        crate::kernel32::time::GetLocalTime(&mut s);
        s
    } else {
        *st
    }
}

unsafe fn out_wide(s: &str, out: *mut u16, cap: i32) -> i32 {
    let w = to_wide_z(s);
    if cap == 0 {
        return w.len() as i32;
    }
    if w.len() > cap as usize {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), out, w.len());
    w.len() as i32
}

const DATE_LONGDATE: u32 = 0x2;
const DATE_YEARMONTH: u32 = 0x8;
const TIME_NOMINUTESORSECONDS: u32 = 0x1;
const TIME_NOSECONDS: u32 = 0x2;
const TIME_NOTIMEMARKER: u32 = 0x4;
const TIME_FORCE24HOURFORMAT: u32 = 0x8;

pub unsafe extern "win64" fn GetDateFormatEx(
    _locale: LPCWSTR,
    flags: u32,
    st: *const SYSTEMTIME,
    fmt: LPCWSTR,
    out: *mut u16,
    cap: i32,
    _cal: LPCWSTR,
) -> i32 {
    let st = current_or(st);
    let pic = match from_wide_opt(fmt) {
        Some(f) => f,
        None if flags & DATE_LONGDATE != 0 => "dddd, MMMM d, yyyy".into(),
        None if flags & DATE_YEARMONTH != 0 => "MMMM yyyy".into(),
        None => "M/d/yyyy".into(),
    };
    out_wide(&format_picture(&pic, &st), out, cap)
}

pub unsafe extern "win64" fn GetDateFormatW(_lcid: u32, flags: u32, st: *const SYSTEMTIME, fmt: LPCWSTR, out: *mut u16, cap: i32) -> i32 {
    GetDateFormatEx(std::ptr::null(), flags, st, fmt, out, cap, std::ptr::null())
}

pub unsafe extern "win64" fn GetDateFormatA(lcid: u32, flags: u32, st: *const SYSTEMTIME, fmt: LPCSTR, out: *mut u8, cap: i32) -> i32 {
    let f = from_ansi_opt(fmt).map(|s| to_wide_z(&s));
    let mut w = [0u16; 256];
    let n = GetDateFormatW(lcid, flags, st, f.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()), w.as_mut_ptr(), 256);
    if n == 0 {
        return 0;
    }
    let s = String::from_utf16_lossy(&w[..n as usize - 1]);
    if cap == 0 {
        return s.len() as i32 + 1;
    }
    if s.len() as i32 + 1 > cap {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    write_ansi_buf(&s, out, cap as usize);
    s.len() as i32 + 1
}

pub unsafe extern "win64" fn GetTimeFormatEx(_locale: LPCWSTR, flags: u32, st: *const SYSTEMTIME, fmt: LPCWSTR, out: *mut u16, cap: i32) -> i32 {
    let st = current_or(st);
    let mut pic = from_wide_opt(fmt).unwrap_or_else(|| "h:mm:ss tt".into());
    if flags & TIME_FORCE24HOURFORMAT != 0 {
        pic = pic.replace('h', "H");
    }
    if flags & TIME_NOTIMEMARKER != 0 {
        pic = pic.replace(" tt", "").replace('t', "");
    }
    if flags & (TIME_NOSECONDS | TIME_NOMINUTESORSECONDS) != 0 {
        pic = pic.replace(":ss", "").replace(":s", "");
    }
    if flags & TIME_NOMINUTESORSECONDS != 0 {
        pic = pic.replace(":mm", "").replace(":m", "");
    }
    out_wide(&format_picture(&pic, &st), out, cap)
}

pub unsafe extern "win64" fn GetTimeFormatW(_lcid: u32, flags: u32, st: *const SYSTEMTIME, fmt: LPCWSTR, out: *mut u16, cap: i32) -> i32 {
    GetTimeFormatEx(std::ptr::null(), flags, st, fmt, out, cap)
}

/// Parse a Windows `LOCALE_SGROUPING` string into right-to-left group sizes.
/// A trailing `0` in the returned slice means "stop grouping remaining leading digits";
/// no trailing `0` means "repeat the last group size" (matching `LOCALE_SGROUPING` where a
/// trailing `;0` repeats, while omitting `;0` stops after the listed groups).
fn parse_locale_grouping(s: &str) -> Vec<usize> {
    let mut parts: Vec<usize> = Vec::new();
    for part in s.split(';') {
        if let Ok(n) = part.trim().parse::<usize>() {
            parts.push(n);
        }
    }
    if parts.is_empty() || parts[0] == 0 {
        return Vec::new();
    }
    if let Some(z) = parts.iter().position(|&n| n == 0) {
        // `;0` in LOCALE_SGROUPING repeats the preceding group size.
        parts.truncate(z);
        parts
    } else {
        // No trailing `;0` in LOCALE_SGROUPING: stop grouping remaining leading digits.
        parts.push(0);
        parts
    }
}

/// Decode Windows `NUMBERFMTW.Grouping` integer into right-to-left group sizes:
/// `0` -> `[]`, `3` -> `[3]` (repeat 3), `32` -> `[3, 2]` (3, then repeat 2),
/// `30` -> `[3, 0]` (3 once, then stop), `320` -> `[3, 2, 0]` (3, then 2, then stop).
fn parse_numberfmt_grouping(g: u32) -> Vec<usize> {
    if g == 0 {
        return Vec::new();
    }
    let s = g.to_string();
    let mut out = Vec::with_capacity(s.len());
    for b in s.bytes() {
        let d = (b - b'0') as usize;
        out.push(d);
        if d == 0 {
            break;
        }
    }
    out
}

fn format_number_string(
    raw: &str,
    num_digits: usize,
    leading_zero: bool,
    groups: &[usize],
    dec_sep: &str,
    thou_sep: &str,
    neg_order: u32,
) -> Option<String> {
    if raw.is_empty() {
        return None;
    }
    let (neg, rest) = if let Some(r) = raw.strip_prefix('-') { (true, r) } else { (false, raw) };
    if rest.is_empty() {
        return None;
    }
    let (int_raw, frac_raw) = match rest.split_once('.') {
        Some((i, f)) => {
            if f.contains('.') {
                return None;
            }
            (i, f)
        }
        None => (rest, ""),
    };
    if !int_raw.bytes().all(|b| b.is_ascii_digit()) || !frac_raw.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    // Round half-up to `num_digits` fractional digits.
    let mut int_digits: Vec<u8> = int_raw.bytes().map(|b| b - b'0').collect();
    let mut frac_digits: Vec<u8> = frac_raw.bytes().map(|b| b - b'0').collect();
    let round_up = frac_digits.get(num_digits).is_some_and(|&d| d >= 5);
    frac_digits.truncate(num_digits);
    while frac_digits.len() < num_digits {
        frac_digits.push(0);
    }
    if round_up {
        let mut carry = 1u8;
        for d in frac_digits.iter_mut().rev() {
            let sum = *d + carry;
            if sum >= 10 {
                *d = sum - 10;
                carry = 1;
            } else {
                *d = sum;
                carry = 0;
                break;
            }
        }
        if carry > 0 {
            for d in int_digits.iter_mut().rev() {
                let sum = *d + carry;
                if sum >= 10 {
                    *d = sum - 10;
                    carry = 1;
                } else {
                    *d = sum;
                    carry = 0;
                    break;
                }
            }
            if carry > 0 {
                int_digits.insert(0, 1);
            }
        }
    }
    // Strip leading zeros on integer part.
    let first_nz = int_digits.iter().position(|&d| d != 0).unwrap_or(int_digits.len());
    let int_trimmed = &int_digits[first_nz..];

    let int_str: String = if int_trimmed.is_empty() {
        if leading_zero || num_digits == 0 {
            "0".into()
        } else {
            String::new()
        }
    } else if groups.is_empty() {
        int_trimmed.iter().map(|&d| (b'0' + d) as char).collect()
    } else {
        // Group digits from right to left using `groups`, stopping if a trailing `0` marker
        // is reached and otherwise repeating the last group size.
        let mut rev_chunks: Vec<String> = Vec::new();
        let mut end = int_trimmed.len();
        let mut gi = 0usize;
        while end > 0 {
            let sz = groups[gi.min(groups.len() - 1)];
            if sz == 0 {
                let chunk: String = int_trimmed[..end].iter().map(|&d| (b'0' + d) as char).collect();
                rev_chunks.push(chunk);
                break;
            }
            let start = end.saturating_sub(sz);
            let chunk: String = int_trimmed[start..end].iter().map(|&d| (b'0' + d) as char).collect();
            rev_chunks.push(chunk);
            end = start;
            if gi + 1 < groups.len() {
                gi += 1;
            }
        }
        rev_chunks.reverse();
        rev_chunks.join(thou_sep)
    };

    let mut body = int_str;
    if num_digits > 0 {
        body.push_str(dec_sep);
        for d in frac_digits {
            body.push((b'0' + d) as char);
        }
    }

    if !neg {
        Some(body)
    } else {
        Some(match neg_order {
            0 => format!("({body})"),
            1 => format!("-{body}"),
            2 => format!("- {body}"),
            3 => format!("{body}-"),
            4 => format!("{body} -"),
            _ => return None,
        })
    }
}

pub unsafe extern "win64" fn GetNumberFormatEx(
    locale: LPCWSTR,
    flags: u32,
    value: LPCWSTR,
    fmt: *const c_void,
    out: *mut u16,
    cap: i32,
) -> i32 {
    if value.is_null() || cap < 0 || (cap > 0 && out.is_null()) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let Some(loc) = resolve_locale_name_w(locale) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let (num_digits, leading_zero, groups, dec_sep, thou_sep, neg_order) = if !fmt.is_null() {
        if flags != 0 {
            set_err(err::ERROR_INVALID_FLAGS);
            return 0;
        }
        // Win64 NUMBERFMTW layout (40 bytes):
        //   0: UINT NumDigits
        //   4: UINT LeadingZero
        //   8: UINT Grouping
        //  16: LPWSTR lpDecimalSep
        //  24: LPWSTR lpThousandSep
        //  32: UINT NegativeOrder
        let p = fmt as *const u8;
        let num_digits = std::ptr::read_unaligned(p as *const u32);
        let leading_zero = std::ptr::read_unaligned(p.add(4) as *const u32);
        let grouping = std::ptr::read_unaligned(p.add(8) as *const u32);
        let dec_ptr = std::ptr::read_unaligned(p.add(16) as *const LPCWSTR);
        let thou_ptr = std::ptr::read_unaligned(p.add(24) as *const LPCWSTR);
        let neg_order = std::ptr::read_unaligned(p.add(32) as *const u32);
        if dec_ptr.is_null() || thou_ptr.is_null() || num_digits > 9 || leading_zero > 1 || neg_order > 4 {
            set_err(err::ERROR_INVALID_PARAMETER);
            return 0;
        }
        (
            num_digits as usize,
            leading_zero != 0,
            parse_numberfmt_grouping(grouping),
            from_wide(dec_ptr),
            from_wide(thou_ptr),
            neg_order,
        )
    } else {
        let nd = locale_info_for(&loc, 0x11).and_then(|s| s.parse::<usize>().ok()).unwrap_or(2);
        let lz = locale_info_for(&loc, 0x12).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1) != 0;
        let gr = locale_info_for(&loc, 0x10).map(|s| parse_locale_grouping(&s)).unwrap_or_else(|| vec![3]);
        let ds = locale_info_for(&loc, 0x0E).unwrap_or_else(|| ".".into());
        let ts = locale_info_for(&loc, 0x0F).unwrap_or_else(|| ",".into());
        let no = locale_info_for(&loc, 0x1005).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1);
        (nd, lz, gr, ds, ts, no)
    };
    let raw = from_wide(value);
    let Some(formatted) = format_number_string(&raw, num_digits, leading_zero, &groups, &dec_sep, &thou_sep, neg_order) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    out_wide(&formatted, out, cap)
}

pub unsafe extern "win64" fn GetNumberFormatW(lcid: u32, flags: u32, value: LPCWSTR, fmt: *const c_void, out: *mut u16, cap: i32) -> i32 {
    let Some(loc) = resolve_lcid(lcid) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let name_w = to_wide_z(&loc.bcp47);
    GetNumberFormatEx(name_w.as_ptr(), flags, value, fmt, out, cap)
}

const CAL_RETURN_NUMBER: u32 = 0x2000_0000;

pub unsafe extern "win64" fn GetCalendarInfoEx(
    locale: LPCWSTR,
    cal: u32,
    _reserved: LPCWSTR,
    ty: u32,
    out: *mut u16,
    cap: i32,
    val: *mut u32,
) -> i32 {
    if cap < 0 || (cap > 0 && out.is_null()) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let Some(loc) = resolve_locale_name_w(locale) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    // Only Gregorian calendars (`CAL_GREGORIAN = 1`, `CAL_GREGORIAN_US = 2`) are backed here;
    // reject non-Gregorian or unknown calendar IDs (e.g. `CAL_JAPAN = 3`) honestly with
    // `ERROR_INVALID_PARAMETER` rather than returning fabricated Gregorian data.
    // Explicitly pin `@calendar=gregorian` on `cal_loc.icu_id` so locales whose default ICU
    // calendar is non-Gregorian (e.g. `th-TH` Buddhist, `ar-SA` Islamic) still query Gregorian
    // eras, patterns, and month names when `cal` is 1 or 2.
    let mut cal_loc: ResolvedLocale = match cal {
        1 => loc,
        2 => ResolvedLocale::en_us(),
        _ => {
            set_err(err::ERROR_INVALID_PARAMETER);
            return 0;
        }
    };
    let base_icu = if cal_loc.invariant { "en_US" } else { cal_loc.icu_id.as_str() };
    cal_loc.icu_id = if base_icu.contains('@') {
        format!("{base_icu};calendar=gregorian")
    } else {
        format!("{base_icu}@calendar=gregorian")
    };
    let cal_id = cal;
    let return_num = (ty & CAL_RETURN_NUMBER) != 0;
    let base = ty & 0xFFFF;
    let era_str = || -> String {
        if let Some(a) = icu::api() {
            let cloc = icu::c_str(&cal_loc.icu_id);
            let mut st = 0i32;
            let df = (a.udat_open)(icu::UDAT_NONE, icu::UDAT_SHORT, cloc.as_ptr(), std::ptr::null(), 0, std::ptr::null(), 0, &mut st);
            if !df.is_null() && st <= 0 {
                let s = icu::call_u16_str(|b, c, e| (a.udat_get_symbols)(df, icu::UDAT_ERAS, 1, b, c, e));
                (a.udat_close)(df);
                if let Some(s) = s {
                    if !s.is_empty() {
                        return s;
                    }
                }
            }
        }
        "A.D.".into()
    };

    let res_str: String = match base {
        0x01 => cal_id.to_string(),                                      // CAL_ICALINTVALUE
        0x02 => "Gregorian Calendar".into(),                             // CAL_SCALNAME
        0x03 => "0".into(),                                              // CAL_IYEAROFFSETRANGE
        0x04 => era_str(),                                               // CAL_SERASTRING
        0x05 => locale_info_for(&cal_loc, 0x1F).unwrap_or_else(|| "M/d/yyyy".into()), // CAL_SSHORTDATE
        0x06 => locale_info_for(&cal_loc, 0x20).unwrap_or_else(|| "dddd, MMMM d, yyyy".into()), // CAL_SLONGDATE
        0x07..=0x0D => locale_info_for(&cal_loc, 0x2A + (base - 0x07)).unwrap_or_default(), // CAL_SDAYNAME1..7
        0x0E..=0x14 => locale_info_for(&cal_loc, 0x31 + (base - 0x0E)).unwrap_or_default(), // CAL_SABBREVDAYNAME1..7
        0x15..=0x20 => locale_info_for(&cal_loc, 0x38 + (base - 0x15)).unwrap_or_default(), // CAL_SMONTHNAME1..12
        0x21 => String::new(),                                           // CAL_SMONTHNAME13 (Gregorian has 12)
        0x22..=0x2D => locale_info_for(&cal_loc, 0x44 + (base - 0x22)).unwrap_or_default(), // CAL_SABBREVMONTHNAME1..12
        0x2E => String::new(),                                           // CAL_SABBREVMONTHNAME13
        0x2F => locale_info_for(&cal_loc, 0x1006).unwrap_or_else(|| "MMMM yyyy".into()), // CAL_SYEARMONTH
        0x30 => "2049".into(),                                           // CAL_ITWODIGITYEARMAX
        0x31..=0x37 => locale_info_for(&cal_loc, 0x60 + (base - 0x31)).unwrap_or_default(), // CAL_SSHORTESTDAYNAME1..7
        0x38 => locale_info_for(&cal_loc, 0x78).unwrap_or_else(|| "MMMM d".into()), // CAL_SMONTHDAY
        0x39 => era_str(),                                               // CAL_SABBREVERASTRING
        0x3A => locale_info_for(&cal_loc, 0x7C).unwrap_or_else(|| "MMMM d, yyyy".into()), // CAL_SRELATIVELONGDATE
        0x3B => "A.D.".into(),                                           // CAL_SENGLISHERANAME
        0x3C => "AD".into(),                                             // CAL_SENGLISHABBREVERANAME
        _ => {
            set_err(err::ERROR_INVALID_FLAGS);
            return 0;
        }
    };

    if return_num {
        if val.is_null() {
            set_err(err::ERROR_INVALID_PARAMETER);
            return 0;
        }
        let n = res_str.parse::<u32>().unwrap_or(0);
        std::ptr::write_unaligned(val, n);
        return 2;
    }
    out_wide(&res_str, out, cap)
}

pub unsafe extern "win64" fn GetCalendarInfoW(lcid: u32, cal: u32, ty: u32, out: *mut u16, cap: i32, val: *mut u32) -> i32 {
    let Some(loc) = resolve_lcid(lcid) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let name_w = to_wide_z(&loc.bcp47);
    GetCalendarInfoEx(name_w.as_ptr(), cal, std::ptr::null(), ty, out, cap, val)
}

// ---------------------------------------------------------------------------
// PathCch* (`api-ms-win-core-path-l1-1-0.dll` / `kernelbase.dll`)
// ---------------------------------------------------------------------------

const PATHCCH_ALLOW_LONG_PATHS: u32 = 0x0000_0001;
const PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS: u32 = 0x0000_0002;
const PATHCCH_FORCE_DISABLE_LONG_NAME_PROCESS: u32 = 0x0000_0004;
const PATHCCH_DO_NOT_NORMALIZE_SEGMENTS: u32 = 0x0000_0008;
const PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH: u32 = 0x0000_0010;
const PATHCCH_ENSURE_TRAILING_SLASH: u32 = 0x0000_0020;
const PATHCCH_MAX_CCH: usize = 0x8000;
const MAX_PATH_CCH: usize = 260;
const STRSAFE_E_INSUFFICIENT_BUFFER: HRESULT = 0x8007_007Au32 as i32;
const HRESULT_FILENAME_EXCED_RANGE: HRESULT = 0x8007_00CEu32 as i32;

fn is_ascii_alpha_u16(c: u16) -> bool {
    (b'A' as u16..=b'Z' as u16).contains(&c) || (b'a' as u16..=b'z' as u16).contains(&c)
}

fn is_sep_u16(c: u16) -> bool {
    c == b'\\' as u16 || c == b'/' as u16
}

fn eq_ascii_ci(slice: &[u16], ascii: &[u8]) -> bool {
    slice.len() == ascii.len()
        && slice.iter().zip(ascii.iter()).all(|(&w, &b)| {
            w < 128 && (w as u8).eq_ignore_ascii_case(&b)
        })
}

/// Compute the root length (in UTF-16 code units) for `PathCchSkipRoot`.
/// Returns `None` if `s` has no recognized Windows root prefix.
fn pathcch_root_len(s: &[u16]) -> Option<usize> {
    let n = s.len();
    if n == 0 {
        return None;
    }
    // Extended-length `\\?\` or `\\.\`
    if n >= 4
        && s[0] == b'\\' as u16
        && s[1] == b'\\' as u16
        && (s[2] == b'?' as u16 || s[2] == b'.' as u16)
        && s[3] == b'\\' as u16
    {
        let rest = &s[4..];
        // `\\?\UNC\server\share[\]`
        if rest.len() >= 4 && eq_ascii_ci(&rest[..4], b"UNC\\") {
            let mut i = 8;
            // server
            let srv_start = i;
            while i < n && s[i] != b'\\' as u16 {
                i += 1;
            }
            if i == srv_start {
                return Some(8);
            }
            if i < n {
                i += 1; // skip '\'
                let share_start = i;
                while i < n && s[i] != b'\\' as u16 {
                    i += 1;
                }
                if i > share_start && i < n && s[i] == b'\\' as u16 {
                    i += 1;
                }
            }
            return Some(i);
        }
        // `\\?\X:[\]`
        if rest.len() >= 2 && is_ascii_alpha_u16(rest[0]) && rest[1] == b':' as u16 {
            return Some(if rest.len() >= 3 && rest[2] == b'\\' as u16 { 7 } else { 6 });
        }
        // `\\?\Volume{...}[\]` or device name `\\.\Device[\]`
        let mut i = 4;
        while i < n && s[i] != b'\\' as u16 {
            i += 1;
        }
        if i < n && s[i] == b'\\' as u16 {
            i += 1;
        }
        return Some(i);
    }
    // UNC `\\server\share[\]`
    if n >= 2 && s[0] == b'\\' as u16 && s[1] == b'\\' as u16 {
        let mut i = 2;
        let srv_start = i;
        while i < n && s[i] != b'\\' as u16 {
            i += 1;
        }
        if i == srv_start {
            return Some(2);
        }
        if i < n {
            i += 1; // skip '\'
            while i < n && s[i] != b'\\' as u16 {
                i += 1;
            }
            if i < n && s[i] == b'\\' as u16 {
                i += 1;
            }
        }
        return Some(i);
    }
    // Drive root `X:\` or `X:`
    if n >= 2 && is_ascii_alpha_u16(s[0]) && s[1] == b':' as u16 {
        return Some(if n >= 3 && is_sep_u16(s[2]) { 3 } else { 2 });
    }
    // Single leading backslash `\`
    if s[0] == b'\\' as u16 {
        return Some(1);
    }
    None
}

pub unsafe extern "win64" fn PathCchSkipRoot(psz_path: LPCWSTR, ppsz_root_end: *mut LPCWSTR) -> HRESULT {
    if ppsz_root_end.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(ppsz_root_end, std::ptr::null());
    if psz_path.is_null() {
        return E_INVALIDARG;
    }
    // Read into an aligned Vec<u16> without assuming 2-byte alignment of `psz_path`
    // and without lossy UTF-8 conversion so every UTF-16 code unit offset is exact.
    let units = crate::strings::read_wide_units_z(psz_path, PATHCCH_MAX_CCH);
    match pathcch_root_len(&units) {
        Some(off) => {
            // Advance by byte offset so odd-aligned guest buffers keep exact provenance.
            let end_ptr = (psz_path as *const u8).add(off * 2) as LPCWSTR;
            std::ptr::write_unaligned(ppsz_root_end, end_ptr);
            S_OK
        }
        None => E_INVALIDARG,
    }
}

/// Default Win32 `PathCch` segment-name normalization: strip trailing spaces and
/// periods from an ordinary segment, retaining a single trailing period only when
/// immediately preceded by the `*` wildcard character.
fn normalize_pathcch_segment(part: &str) -> &str {
    let trimmed = part.trim_end_matches([' ', '.']);
    let had_trailing_dot = part.as_bytes().get(trimmed.len()) == Some(&b'.');
    if had_trailing_dot && trimmed.as_bytes().last() == Some(&b'*') {
        &part[..trimmed.len() + 1]
    } else {
        trimmed
    }
}

fn canonicalize_pathcch(raw: &str, flags: u32) -> String {
    // PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH implies PATHCCH_DO_NOT_NORMALIZE_SEGMENTS.
    let preserve_segments =
        (flags & (PATHCCH_DO_NOT_NORMALIZE_SEGMENTS | PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH)) != 0;
    let normalized = raw.replace('/', "\\");
    let wide: Vec<u16> = normalized.encode_utf16().collect();
    let root_len = pathcch_root_len(&wide).unwrap_or(0);
    let root_str = String::from_utf16_lossy(&wide[..root_len]);
    let rest_str = String::from_utf16_lossy(&wide[root_len..]);

    let has_drive_only_root = root_str.len() == 2 && root_str.as_bytes()[1] == b':';
    let rooted = !root_str.is_empty() && !has_drive_only_root;

    let mut segs: Vec<&str> = Vec::new();
    for part in rest_str.split('\\') {
        if part.is_empty() || part == "." {
            continue;
        }
        // Exact "." and ".." components are always resolved prior to segment-name
        // normalization, preserving drive and UNC root boundaries.
        if part == ".." {
            if let Some(&last) = segs.last() {
                if last != ".." {
                    segs.pop();
                    continue;
                }
            }
            if rooted {
                continue;
            }
            segs.push("..");
            continue;
        }
        let seg = if preserve_segments {
            part
        } else {
            normalize_pathcch_segment(part)
        };
        if !seg.is_empty() {
            segs.push(seg);
        }
    }

    let mut out = root_str;
    if !segs.is_empty() {
        if !out.is_empty() && !out.ends_with('\\') && !has_drive_only_root {
            out.push('\\');
        }
        out.push_str(&segs.join("\\"));
    }

    let want_ext = (flags & PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH) != 0
        || ((flags & PATHCCH_ALLOW_LONG_PATHS) != 0 && out.encode_utf16().count() >= MAX_PATH_CCH);
    if want_ext && !out.starts_with("\\\\?\\") && !out.starts_with("\\\\.\\") {
        let b = out.as_bytes();
        if out.starts_with("\\\\") {
            out = format!("\\\\?\\UNC\\{}", &out[2..]);
        } else if b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && b[2] == b'\\' {
            out = format!("\\\\?\\{out}");
        }
    }

    if (flags & PATHCCH_ENSURE_TRAILING_SLASH) != 0 && !out.is_empty() && !out.ends_with('\\') {
        out.push('\\');
    }
    out
}

pub unsafe extern "win64" fn PathCchCombineEx(
    psz_path_out: LPWSTR,
    cch_path_out: usize,
    psz_path_in: LPCWSTR,
    psz_more: LPCWSTR,
    dw_flags: u32,
) -> HRESULT {
    let valid_flags = PATHCCH_ALLOW_LONG_PATHS
        | PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS
        | PATHCCH_FORCE_DISABLE_LONG_NAME_PROCESS
        | PATHCCH_DO_NOT_NORMALIZE_SEGMENTS
        | PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH
        | PATHCCH_ENSURE_TRAILING_SLASH;
    if psz_path_out.is_null() || cch_path_out == 0 || cch_path_out > PATHCCH_MAX_CCH {
        return E_INVALIDARG;
    }
    if (dw_flags & !valid_flags) != 0 {
        return E_INVALIDARG;
    }
    if (dw_flags & PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS) != 0
        && (dw_flags & PATHCCH_FORCE_DISABLE_LONG_NAME_PROCESS) != 0
    {
        return E_INVALIDARG;
    }
    if (dw_flags & PATHCCH_ALLOW_LONG_PATHS) != 0
        && (dw_flags & PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH) != 0
    {
        return E_INVALIDARG;
    }

    let p_in = crate::strings::read_wide_z_opt(psz_path_in, PATHCCH_MAX_CCH).unwrap_or_default();
    let p_more = crate::strings::read_wide_z_opt(psz_more, PATHCCH_MAX_CCH).unwrap_or_default();

    let combined = if p_in.is_empty() && p_more.is_empty() {
        String::new()
    } else if p_in.is_empty() {
        p_more
    } else if p_more.is_empty() {
        p_in
    } else {
        let more_norm = p_more.replace('/', "\\");
        let more_w: Vec<u16> = more_norm.encode_utf16().collect();
        let in_norm = p_in.replace('/', "\\");
        let in_w: Vec<u16> = in_norm.encode_utf16().collect();
        if let Some(mroot) = pathcch_root_len(&more_w) {
            if mroot == 1 && more_norm.starts_with('\\') && !more_norm.starts_with("\\\\") {
                // Single leading backslash: combine with root of `psz_path_in`.
                if let Some(iroot) = pathcch_root_len(&in_w) {
                    let root_part = String::from_utf16_lossy(&in_w[..iroot]);
                    let trimmed_more = more_norm.trim_start_matches('\\');
                    if root_part.ends_with('\\') {
                        format!("{root_part}{trimmed_more}")
                    } else {
                        format!("{root_part}\\{trimmed_more}")
                    }
                } else {
                    more_norm
                }
            } else {
                // Fully rooted `psz_more` replaces `psz_path_in`.
                more_norm
            }
        } else if in_norm.ends_with('\\') {
            format!("{in_norm}{more_norm}")
        } else {
            format!("{in_norm}\\{more_norm}")
        }
    };

    let result = if combined.is_empty() {
        String::new()
    } else {
        canonicalize_pathcch(&combined, dw_flags)
    };
    let wide: Vec<u16> = result.encode_utf16().collect();
    let long_allowed = (dw_flags
        & (PATHCCH_ALLOW_LONG_PATHS
            | PATHCCH_FORCE_ENABLE_LONG_NAME_PROCESS
            | PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH))
        != 0;
    if !long_allowed && wide.len() + 1 > MAX_PATH_CCH {
        std::ptr::write_unaligned(psz_path_out, 0);
        return HRESULT_FILENAME_EXCED_RANGE;
    }
    if wide.len() + 1 > cch_path_out {
        std::ptr::write_unaligned(psz_path_out, 0);
        return STRSAFE_E_INSUFFICIENT_BUFFER;
    }
    if !wide.is_empty() {
        std::ptr::copy_nonoverlapping(
            wide.as_ptr() as *const u8,
            psz_path_out as *mut u8,
            wide.len() * 2,
        );
    }
    std::ptr::write_unaligned((psz_path_out as *mut u8).add(wide.len() * 2) as *mut u16, 0);
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mb_roundtrip() {
        unsafe {
            let src = "héllo\0";
            let n = MultiByteToWideChar(CP_UTF8, 0, src.as_ptr(), -1, std::ptr::null_mut(), 0);
            assert_eq!(n, 6);
            let mut w = vec![0u16; n as usize];
            MultiByteToWideChar(CP_UTF8, 0, src.as_ptr(), -1, w.as_mut_ptr(), n);
            let m = WideCharToMultiByte(CP_UTF8, 0, w.as_ptr(), -1, std::ptr::null_mut(), 0, std::ptr::null(), std::ptr::null_mut());
            assert_eq!(m as usize, src.len());
            // MB_ERR_INVALID_CHARS
            let bad = [0xFFu8, 0];
            assert_eq!(MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, bad.as_ptr(), -1, std::ptr::null_mut(), 0), 0);
        }
    }
    #[test]
    fn compare() {
        let w = |s: &str| to_wide(s);
        assert_eq!(compare_linguistic(&w("apple"), &w("Apple"), 0), Ordering::Less);
        assert_eq!(compare_linguistic(&w("apple"), &w("APPLE"), NORM_IGNORECASE), Ordering::Equal);
        assert_eq!(compare_linguistic(&w("b"), &w("A"), 0), Ordering::Greater);
        let ka = sort_key(&w("apple"), 0);
        let kb = sort_key(&w("Apple"), 0);
        let kc = sort_key(&w("b"), 0);
        assert!(ka < kb && kb < kc);

        // Windows word sort (flags = 0) vs string sort (SORT_STRINGSORT = 0x1000) via ICU:
        // word sort treats '-' and '\'' as 4th-level tie-breakers so "co-op" > "con" and
        // "coop" < "co-op", while other punctuation ("a!b" < "ab") keeps primary weight.
        assert_eq!(compare_for_locale("en_US", &w("co-op"), &w("con"), 0), Ordering::Greater);
        assert_eq!(compare_for_locale("en_US", &w("coop"), &w("co-op"), 0), Ordering::Less);
        assert_eq!(compare_for_locale("en_US", &w("a'b"), &w("a-b"), 0), Ordering::Less);
        assert_eq!(compare_for_locale("en_US", &w("a!b"), &w("ab"), 0), Ordering::Less);
        assert_eq!(compare_for_locale("en_US", &w("co-op"), &w("con"), SORT_STRINGSORT), Ordering::Less);
        assert_eq!(compare_for_locale("en_US", &w("co-op"), &w("coop"), NORM_IGNORESYMBOLS), Ordering::Equal);

        // Sort keys must match compare_for_locale 1-to-1 in both word sort and string sort.
        let k_coop_w = sort_key_for_locale("en_US", &w("coop"), 0);
        let k_co_op_w = sort_key_for_locale("en_US", &w("co-op"), 0);
        let k_con_w = sort_key_for_locale("en_US", &w("con"), 0);
        assert!(k_con_w < k_coop_w && k_coop_w < k_co_op_w);
        assert!(!k_co_op_w[..k_co_op_w.len() - 1].contains(&0));
        let k_co_op_s = sort_key_for_locale("en_US", &w("co-op"), SORT_STRINGSORT);
        let k_con_s = sort_key_for_locale("en_US", &w("con"), SORT_STRINGSORT);
        assert!(k_co_op_s < k_con_s);
    }
    #[test]
    fn date_picture() {
        let st = SYSTEMTIME { wYear: 2023, wMonth: 10, wDayOfWeek: 5, wDay: 27, wHour: 13, wMinute: 5, wSecond: 9, wMilliseconds: 0 };
        assert_eq!(format_picture("dddd, MMMM d, yyyy", &st), "Friday, October 27, 2023");
        assert_eq!(format_picture("h:mm:ss tt", &st), "1:05:09 PM");
        assert_eq!(format_picture("'at' HH", &st), "at 13");
    }
    #[test]
    fn path_cch_skip_root_and_combine() {
        unsafe {
            let p1 = to_wide_z("C:\\Users\\alice\\file.txt");
            let mut end: LPCWSTR = std::ptr::null();
            assert_eq!(PathCchSkipRoot(p1.as_ptr(), &mut end), S_OK);
            assert_eq!(from_wide(end), "Users\\alice\\file.txt");

            let unc = to_wide_z("\\\\server\\share\\dir\\a.bin");
            assert_eq!(PathCchSkipRoot(unc.as_ptr(), &mut end), S_OK);
            assert_eq!(from_wide(end), "dir\\a.bin");

            let ext_unc = to_wide_z("\\\\?\\UNC\\srv\\shr\\sub");
            assert_eq!(PathCchSkipRoot(ext_unc.as_ptr(), &mut end), S_OK);
            assert_eq!(from_wide(end), "sub");

            let rel = to_wide_z("relative\\path");
            assert_eq!(PathCchSkipRoot(rel.as_ptr(), &mut end), E_INVALIDARG);
            assert!(end.is_null());

            let mut out = [0u16; 512];
            let base = to_wide_z("C:\\Games\\App\\bin");
            let more = to_wide_z("..\\data\\./cfg.ini");
            assert_eq!(PathCchCombineEx(out.as_mut_ptr(), out.len(), base.as_ptr(), more.as_ptr(), 0), S_OK);
            assert_eq!(from_wide(out.as_ptr()), "C:\\Games\\App\\data\\cfg.ini");

            // Leading single backslash keeps root of pszPathIn
            let root_rel = to_wide_z("\\Windows\\System32");
            assert_eq!(
                PathCchCombineEx(out.as_mut_ptr(), out.len(), base.as_ptr(), root_rel.as_ptr(), PATHCCH_ENSURE_TRAILING_SLASH),
                S_OK
            );
            assert_eq!(from_wide(out.as_ptr()), "C:\\Windows\\System32\\");

            // Extended length prefix
            assert_eq!(
                PathCchCombineEx(out.as_mut_ptr(), out.len(), base.as_ptr(), more.as_ptr(), PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH),
                S_OK
            );
            assert_eq!(from_wide(out.as_ptr()), "\\\\?\\C:\\Games\\App\\data\\cfg.ini");

            // Insufficient buffer
            let mut tiny = [0u16; 8];
            assert_eq!(
                PathCchCombineEx(tiny.as_mut_ptr(), tiny.len(), base.as_ptr(), more.as_ptr(), 0),
                STRSAFE_E_INSUFFICIENT_BUFFER
            );
            assert_eq!(tiny[0], 0);

            // Empty string & null input rejected cleanly.
            let empty = to_wide_z("");
            let mut empty_end: LPCWSTR = p1.as_ptr();
            assert_eq!(PathCchSkipRoot(empty.as_ptr(), &mut empty_end), E_INVALIDARG);
            assert!(empty_end.is_null());
            assert_eq!(PathCchSkipRoot(std::ptr::null(), &mut empty_end), E_INVALIDARG);
            assert!(empty_end.is_null());

            // Unaligned (odd-byte) guest UTF-16 buffers AND odd-aligned ppsz_root_end out-pointer
            // across drive, UNC, extended UNC, lone backslash, empty, and unpaired surrogate units.
            let mut raw_buf = [0xAAu8; 256];
            let mut raw_out_ptr = [0x55u8; 16];
            let odd_out_ptr = raw_out_ptr.as_mut_ptr().add(1) as *mut LPCWSTR;

            let write_odd_units = |buf: &mut [u8], units: &[u16]| -> LPCWSTR {
                let dst = buf.as_mut_ptr().add(1);
                for (i, &u) in units.iter().enumerate() {
                    std::ptr::write_unaligned(dst.add(i * 2) as *mut u16, u);
                }
                std::ptr::write_unaligned(dst.add(units.len() * 2) as *mut u16, 0u16);
                dst as LPCWSTR
            };

            // Odd-aligned drive path + unpaired surrogate (0xD800) preserved without loss.
            let mut drive_units = to_wide("C:\\dir\\");
            drive_units.push(0xD800);
            drive_units.extend(to_wide("tail.txt"));
            let odd_in = write_odd_units(&mut raw_buf, &drive_units);
            assert_eq!(PathCchSkipRoot(odd_in, odd_out_ptr), S_OK);
            let got_ptr = std::ptr::read_unaligned(odd_out_ptr);
            assert_eq!(got_ptr as usize, (odd_in as usize) + 3 * 2);
            let got_units = crate::strings::read_wide_units_z(got_ptr, 64);
            assert_eq!(got_units, drive_units[3..]);

            // Odd-aligned drive-relative `D:foo` skips 2 code units (`D:`).
            let drive_rel = to_wide("D:foo");
            let odd_in = write_odd_units(&mut raw_buf, &drive_rel);
            assert_eq!(PathCchSkipRoot(odd_in, odd_out_ptr), S_OK);
            let got_ptr = std::ptr::read_unaligned(odd_out_ptr);
            assert_eq!(got_ptr as usize, (odd_in as usize) + 2 * 2);
            assert_eq!(crate::strings::read_wide_z(got_ptr, 64), "foo");

            // Odd-aligned UNC `\\srv\shr\rest` skips `\\srv\shr\`.
            let unc_units = to_wide("\\\\srv\\shr\\rest");
            let odd_in = write_odd_units(&mut raw_buf, &unc_units);
            assert_eq!(PathCchSkipRoot(odd_in, odd_out_ptr), S_OK);
            let got_ptr = std::ptr::read_unaligned(odd_out_ptr);
            assert_eq!(got_ptr as usize, (odd_in as usize) + 10 * 2);
            assert_eq!(crate::strings::read_wide_z(got_ptr, 64), "rest");

            // Odd-aligned extended UNC `\\?\UNC\host\share\leaf`.
            let ext_unc_units = to_wide("\\\\?\\UNC\\host\\share\\leaf");
            let odd_in = write_odd_units(&mut raw_buf, &ext_unc_units);
            assert_eq!(PathCchSkipRoot(odd_in, odd_out_ptr), S_OK);
            let got_ptr = std::ptr::read_unaligned(odd_out_ptr);
            assert_eq!(got_ptr as usize, (odd_in as usize) + 19 * 2);
            assert_eq!(crate::strings::read_wide_z(got_ptr, 64), "leaf");

            // Odd-aligned empty string fails and writes NULL to odd_out_ptr.
            let odd_empty = write_odd_units(&mut raw_buf, &[]);
            assert_eq!(PathCchSkipRoot(odd_empty, odd_out_ptr), E_INVALIDARG);
            assert!(std::ptr::read_unaligned(odd_out_ptr).is_null());

            // PathCchCombineEx with odd-aligned inputs AND odd-aligned output buffer.
            let mut raw_in1 = [0u8; 64];
            let mut raw_in2 = [0u8; 64];
            let mut raw_comb = [0xCCu8; 128];
            let odd_base = write_odd_units(&mut raw_in1, &to_wide("C:\\Alpha\\Beta"));
            let odd_more = write_odd_units(&mut raw_in2, &to_wide("..\\Gamma\\file.bin"));
            let odd_comb = raw_comb.as_mut_ptr().add(1) as LPWSTR;
            assert_eq!(PathCchCombineEx(odd_comb, 60, odd_base, odd_more, 0), S_OK);
            assert_eq!(crate::strings::read_wide_z(odd_comb, 60), "C:\\Alpha\\Gamma\\file.bin");
        }
    }

    #[test]
    fn path_cch_combine_ex_dot_resolution_and_segment_normalization() {
        let cases: &[(&str, &str, u32, &str)] = &[
            // Exact "." and ".." always resolve, including with 0x8 and 0x10.
            ("C:\\Games\\App\\bin", "..\\data\\./cfg.ini", 0, "C:\\Games\\App\\data\\cfg.ini"),
            (
                "C:\\Games\\App\\bin",
                "..\\data\\./cfg.ini",
                PATHCCH_DO_NOT_NORMALIZE_SEGMENTS,
                "C:\\Games\\App\\data\\cfg.ini",
            ),
            (
                "C:\\Games\\App\\bin",
                "..\\data\\./cfg.ini",
                PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH,
                "\\\\?\\C:\\Games\\App\\data\\cfg.ini",
            ),
            // Trailing spaces and periods normalized by default, preserved by 0x8 and implied by 0x10.
            ("C:\\dir \\sub", "..\\file. . ", 0, "C:\\dir\\file"),
            (
                "C:\\dir \\sub",
                "..\\file. . ",
                PATHCCH_DO_NOT_NORMALIZE_SEGMENTS,
                "C:\\dir \\file. . ",
            ),
            (
                "C:\\dir \\sub",
                "..\\file. . ",
                PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH,
                "\\\\?\\C:\\dir \\file. . ",
            ),
            ("C:\\a\\b..", ".\\c...\\d ", 0, "C:\\a\\b\\c\\d"),
            (
                "C:\\a\\b..",
                ".\\c...\\d ",
                PATHCCH_DO_NOT_NORMALIZE_SEGMENTS,
                "C:\\a\\b..\\c...\\d ",
            ),
            // Windows wildcard rule: retain a single trailing period immediately after '*' only.
            ("C:\\a", "*...", 0, "C:\\a\\*."),
            ("C:\\a", "*.", 0, "C:\\a\\*."),
            ("C:\\a", "foo?. .", 0, "C:\\a\\foo?"),
            ("C:\\a", "foo?. .", PATHCCH_DO_NOT_NORMALIZE_SEGMENTS, "C:\\a\\foo?. ."),
            ("C:\\a", "*...", PATHCCH_DO_NOT_NORMALIZE_SEGMENTS, "C:\\a\\*..."),
            // Drive and UNC root boundaries preserved under default, 0x8, and 0x10.
            ("C:\\dir", "..\\..\\leaf", 0, "C:\\leaf"),
            ("C:\\dir", "..\\..\\leaf", PATHCCH_DO_NOT_NORMALIZE_SEGMENTS, "C:\\leaf"),
            ("C:\\dir", "..\\..", PATHCCH_DO_NOT_NORMALIZE_SEGMENTS, "C:\\"),
            ("\\\\srv\\shr\\dir", "..\\..\\leaf", 0, "\\\\srv\\shr\\leaf"),
            (
                "\\\\srv\\shr\\dir",
                "..\\..\\leaf",
                PATHCCH_DO_NOT_NORMALIZE_SEGMENTS,
                "\\\\srv\\shr\\leaf",
            ),
            (
                "\\\\srv\\shr\\dir",
                "..\\..\\leaf. ",
                PATHCCH_ENSURE_IS_EXTENDED_LENGTH_PATH,
                "\\\\?\\UNC\\srv\\shr\\leaf. ",
            ),
            ("\\\\srv\\shr\\dir", "..\\..", PATHCCH_DO_NOT_NORMALIZE_SEGMENTS, "\\\\srv\\shr\\"),
        ];

        unsafe {
            let mut out = [0u16; 512];
            for &(base, more, flags, expected) in cases {
                let base_w = to_wide_z(base);
                let more_w = to_wide_z(more);
                assert_eq!(
                    PathCchCombineEx(out.as_mut_ptr(), out.len(), base_w.as_ptr(), more_w.as_ptr(), flags),
                    S_OK,
                    "PathCchCombineEx({base:?}, {more:?}, {flags:#x})"
                );
                assert_eq!(
                    from_wide(out.as_ptr()),
                    expected,
                    "PathCchCombineEx({base:?}, {more:?}, {flags:#x})"
                );
            }
        }
    }

    #[test]
    fn lcmapstring_ex_hash_casing_and_transliteration() {
        unsafe {
            let en = to_wide_z("en-US");
            let s_hi = to_wide_z("Hello");
            let s_lo = to_wide_z("hello");

            // LCMAP_HASH returns 4 bytes and honors NORM_IGNORECASE.
            assert_eq!(
                LCMapStringEx(en.as_ptr(), LCMAP_HASH, s_hi.as_ptr(), -1, std::ptr::null_mut(), 0, std::ptr::null_mut(), std::ptr::null_mut(), 0),
                4
            );
            let mut h1 = 0u32;
            let mut h2 = 0u32;
            let mut h3 = 0u32;
            assert_eq!(
                LCMapStringEx(en.as_ptr(), LCMAP_HASH | NORM_IGNORECASE, s_hi.as_ptr(), -1, &mut h1 as *mut u32 as LPWSTR, 4, std::ptr::null_mut(), std::ptr::null_mut(), 0),
                4
            );
            assert_eq!(
                LCMapStringEx(en.as_ptr(), LCMAP_HASH | NORM_IGNORECASE, s_lo.as_ptr(), -1, &mut h2 as *mut u32 as LPWSTR, 4, std::ptr::null_mut(), std::ptr::null_mut(), 0),
                4
            );
            assert_eq!(
                LCMapStringEx(en.as_ptr(), LCMAP_HASH, s_hi.as_ptr(), -1, &mut h3 as *mut u32 as LPWSTR, 4, std::ptr::null_mut(), std::ptr::null_mut(), 0),
                4
            );
            assert_eq!(h1, h2);
            // Insufficient buffer fails with ERROR_INSUFFICIENT_BUFFER.
            assert_eq!(
                LCMapStringEx(en.as_ptr(), LCMAP_HASH, s_hi.as_ptr(), -1, &mut h1 as *mut u32 as LPWSTR, 2, std::ptr::null_mut(), std::ptr::null_mut(), 0),
                0
            );
            assert_eq!(teb::last_error(), err::ERROR_INSUFFICIENT_BUFFER);

            // LCMapStringA preserves raw 4-byte binary hash output without UTF-16->UTF-8 conversion.
            assert_eq!(LCMapStringA(0x0409, LCMAP_HASH, b"Hello\0".as_ptr(), -1, std::ptr::null_mut(), 0), 4);
            let mut ha = 0u32;
            assert_eq!(
                LCMapStringA(0x0409, LCMAP_HASH, b"Hello\0".as_ptr(), -1, &mut ha as *mut u32 as LPSTR, 4),
                4
            );
            assert_eq!(ha, h3);
            assert_eq!(
                LCMapStringA(0x0409, LCMAP_HASH, b"Hello\0".as_ptr(), -1, &mut ha as *mut u32 as LPSTR, 2),
                0
            );
            assert_eq!(teb::last_error(), err::ERROR_INSUFFICIENT_BUFFER);

            // Kana & Width & Chinese transliterations
            let mut out = [0u16; 64];
            let kata = to_wide_z("アイウ");
            let n = LCMapStringEx(en.as_ptr(), LCMAP_HIRAGANA, kata.as_ptr(), -1, out.as_mut_ptr(), 64, std::ptr::null_mut(), std::ptr::null_mut(), 0);
            assert!(n > 0);
            assert_eq!(from_wide(out.as_ptr()), "あいう");

            let hira = to_wide_z("あいう");
            let n = LCMapStringEx(en.as_ptr(), LCMAP_KATAKANA, hira.as_ptr(), -1, out.as_mut_ptr(), 64, std::ptr::null_mut(), std::ptr::null_mut(), 0);
            assert!(n > 0);
            assert_eq!(from_wide(out.as_ptr()), "アイウ");

            let fw = to_wide_z("ＡＢＣ１２３");
            let n = LCMapStringEx(en.as_ptr(), LCMAP_HALFWIDTH, fw.as_ptr(), -1, out.as_mut_ptr(), 64, std::ptr::null_mut(), std::ptr::null_mut(), 0);
            assert!(n > 0);
            assert_eq!(from_wide(out.as_ptr()), "ABC123");

            let hw = to_wide_z("ABC");
            let n = LCMapStringEx(en.as_ptr(), LCMAP_FULLWIDTH, hw.as_ptr(), -1, out.as_mut_ptr(), 64, std::ptr::null_mut(), std::ptr::null_mut(), 0);
            assert!(n > 0);
            assert_eq!(from_wide(out.as_ptr()), "ＡＢＣ");

            let trad = to_wide_z("繁體");
            let n = LCMapStringEx(en.as_ptr(), LCMAP_SIMPLIFIED_CHINESE, trad.as_ptr(), -1, out.as_mut_ptr(), 64, std::ptr::null_mut(), std::ptr::null_mut(), 0);
            assert!(n > 0);
            assert_eq!(from_wide(out.as_ptr()), "繁体");

            // Turkish linguistic casing
            let tr = to_wide_z("tr-TR");
            let tr_in = to_wide_z("i");
            let n = LCMapStringEx(
                tr.as_ptr(),
                LCMAP_UPPERCASE | LCMAP_LINGUISTIC_CASING,
                tr_in.as_ptr(),
                -1,
                out.as_mut_ptr(),
                64,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
            );
            assert!(n > 0);
            assert_eq!(from_wide(out.as_ptr()), "İ");

            // Mutually exclusive flags fail with ERROR_INVALID_FLAGS.
            assert_eq!(
                LCMapStringEx(
                    en.as_ptr(),
                    LCMAP_HIRAGANA | LCMAP_KATAKANA,
                    hira.as_ptr(),
                    -1,
                    out.as_mut_ptr(),
                    64,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                ),
                0
            );
            assert_eq!(teb::last_error(), err::ERROR_INVALID_FLAGS);

            // CompareStringEx / CompareStringW must match LCMapStringEx(LCMAP_SORTKEY) ordering.
            let get_sort_key = |loc: &[u16], s: &[u16], flags: u32| -> Vec<u8> {
                let need = LCMapStringEx(
                    loc.as_ptr(),
                    LCMAP_SORTKEY | flags,
                    s.as_ptr(),
                    -1,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                );
                assert!(need > 0);
                let mut k = vec![0u8; need as usize];
                let got = LCMapStringEx(
                    loc.as_ptr(),
                    LCMAP_SORTKEY | flags,
                    s.as_ptr(),
                    -1,
                    k.as_mut_ptr() as LPWSTR,
                    need,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                );
                assert_eq!(got, need);
                k
            };
            let ae = to_wide_z("ä");
            let zz = to_wide_z("z");
            let sv = to_wide_z("sv-SE");
            // en-US: 'ä' sorts before 'z' in BOTH CompareStringEx/W and LCMAP_SORTKEY.
            assert_eq!(CompareStringEx(en.as_ptr(), 0, ae.as_ptr(), -1, zz.as_ptr(), -1, std::ptr::null_mut(), std::ptr::null_mut(), 0), CSTR_LESS_THAN);
            assert_eq!(CompareStringW(0x0409, 0, ae.as_ptr(), -1, zz.as_ptr(), -1), CSTR_LESS_THAN);
            assert!(get_sort_key(&en, &ae, 0) < get_sort_key(&en, &zz, 0));
            // sv-SE: 'ä' sorts after 'z' in BOTH CompareStringEx and LCMAP_SORTKEY.
            assert_eq!(CompareStringEx(sv.as_ptr(), 0, ae.as_ptr(), -1, zz.as_ptr(), -1, std::ptr::null_mut(), std::ptr::null_mut(), 0), CSTR_GREATER_THAN);
            assert!(get_sort_key(&sv, &ae, 0) > get_sort_key(&sv, &zz, 0));
            // Numeric collation & ignore-diacritics-case-sensitive consistency.
            let f2 = to_wide_z("file2");
            let f10 = to_wide_z("file10");
            assert_eq!(CompareStringEx(en.as_ptr(), SORT_DIGITSASNUMBERS, f2.as_ptr(), -1, f10.as_ptr(), -1, std::ptr::null_mut(), std::ptr::null_mut(), 0), CSTR_LESS_THAN);
            assert!(get_sort_key(&en, &f2, SORT_DIGITSASNUMBERS) < get_sort_key(&en, &f10, SORT_DIGITSASNUMBERS));
            let a_lo = to_wide_z("a");
            let a_up = to_wide_z("A");
            assert_eq!(CompareStringEx(en.as_ptr(), NORM_IGNORENONSPACE, ae.as_ptr(), -1, a_lo.as_ptr(), -1, std::ptr::null_mut(), std::ptr::null_mut(), 0), CSTR_EQUAL);
            assert_eq!(get_sort_key(&en, &ae, NORM_IGNORENONSPACE), get_sort_key(&en, &a_lo, NORM_IGNORENONSPACE));
            assert_eq!(CompareStringEx(en.as_ptr(), NORM_IGNORENONSPACE, a_lo.as_ptr(), -1, a_up.as_ptr(), -1, std::ptr::null_mut(), std::ptr::null_mut(), 0), CSTR_LESS_THAN);
            assert!(get_sort_key(&en, &a_lo, NORM_IGNORENONSPACE) < get_sort_key(&en, &a_up, NORM_IGNORENONSPACE));
        }
    }

    #[test]
    fn get_locale_info_real_locales_and_lctypes() {
        unsafe {
            let query_w = |lcid: u32, lctype: u32| -> String {
                let mut buf = [0u16; 128];
                let n = GetLocaleInfoW(lcid, lctype, buf.as_mut_ptr(), 128);
                assert!(n > 0, "GetLocaleInfoW({lcid:#x}, {lctype:#x}) failed");
                from_wide(buf.as_ptr())
            };
            // de-DE (0x0407): real German data (not en-US!)
            assert_eq!(query_w(0x0407, 0x0E), ","); // LOCALE_SDECIMAL
            assert_eq!(query_w(0x0407, 0x0F), "."); // LOCALE_STHOUSAND
            assert_eq!(query_w(0x0407, 0x14), "€"); // LOCALE_SCURRENCY
            assert_eq!(query_w(0x0407, 0x15), "EUR"); // LOCALE_SINTLSYMBOL
            assert_eq!(query_w(0x0407, 0x2A), "Montag"); // LOCALE_SDAYNAME1
            assert_eq!(query_w(0x0407, 0x38), "Januar"); // LOCALE_SMONTHNAME1
            assert_eq!(query_w(0x0407, 0x0D), "0"); // metric

            // ja-JP (0x0411)
            assert_eq!(query_w(0x0411, 0x15), "JPY");
            assert_eq!(query_w(0x0411, 0x2A), "月曜日");
            assert_eq!(query_w(0x0411, 0x1004), "932");

            // GetLocaleInfoEx with fr-FR
            let fr = to_wide_z("fr-FR");
            let mut buf = [0u16; 128];
            assert!(GetLocaleInfoEx(fr.as_ptr(), 0x2A, buf.as_mut_ptr(), 128) > 0);
            assert_eq!(from_wide(buf.as_ptr()), "lundi");

            // Invalid LCID returns 0 + ERROR_INVALID_PARAMETER (never silently en-US!)
            assert_eq!(GetLocaleInfoW(0x9999, 0x0E, buf.as_mut_ptr(), 128), 0);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Unknown LCTYPE returns 0 + ERROR_INVALID_FLAGS
            assert_eq!(GetLocaleInfoW(0x0409, 0x7FFF, buf.as_mut_ptr(), 128), 0);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_FLAGS);

            // LOCALE_FONTSIGNATURE (0x58) returns 16 WCHARs
            assert_eq!(GetLocaleInfoW(0x0409, 0x58, buf.as_mut_ptr(), 128), 16);

            // GetLocaleInfoA encodes with the locale's default ANSI code page unless LOCALE_USE_CP_ACP.
            let mut abuf = [0u8; 8];
            assert_eq!(GetLocaleInfoA(0x0407, 0x14, std::ptr::null_mut(), 0), 2); // CP1252 "€\0"
            assert_eq!(GetLocaleInfoA(0x0407, 0x14, abuf.as_mut_ptr(), 2), 2);
            assert_eq!(&abuf[..2], &[0x80, 0x00]);
            // With LOCALE_USE_CP_ACP (0x4000_0000), uses ACP (UTF-8 = 4 bytes "E2 82 AC 00").
            assert_eq!(GetLocaleInfoA(0x0407, 0x14 | 0x4000_0000, std::ptr::null_mut(), 0), 4);
            assert_eq!(GetLocaleInfoA(0x0407, 0x14 | 0x4000_0000, abuf.as_mut_ptr(), 4), 4);
            assert_eq!(&abuf[..4], &[0xE2, 0x82, 0xAC, 0x00]);
        }
    }

    #[test]
    fn get_geo_info_real_nations_and_errors() {
        unsafe {
            let query = |geo: i32, ty: u32, lang: u16| -> String {
                let mut buf = [0u16; 128];
                let n = GetGeoInfoW(geo, ty, buf.as_mut_ptr(), 128, lang);
                assert!(n > 0, "GetGeoInfoW({geo}, {ty}) failed");
                from_wide(buf.as_ptr())
            };
            assert_eq!(query(94, 4, 0), "DE");
            assert_eq!(query(94, 5, 0), "DEU");
            assert_eq!(query(94, 15, 0), "EUR");
            assert_eq!(query(94, 14, 0), "49");
            assert_eq!(query(94, 8, 0x0409), "Germany");
            assert_eq!(query(94, 8, 0x0407), "Deutschland");

            assert_eq!(query(122, 4, 0), "JP");
            assert_eq!(query(122, 15, 0), "JPY");
            assert_eq!(query(242, 4, 0), "GB");
            assert_eq!(query(242, 15, 0), "GBP");
            assert_eq!(query(244, 4, 0), "US");
            assert_eq!(query(77, 4, 0), "FI");

            // Invalid GeoID fails cleanly with ERROR_INVALID_PARAMETER (never answers US data!)
            let mut buf = [0u16; 64];
            assert_eq!(GetGeoInfoW(99999, 4, buf.as_mut_ptr(), 64, 0), 0);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
            assert_eq!(GetGeoInfoW(94, 999, buf.as_mut_ptr(), 64, 0), 0);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_FLAGS);
        }
    }

    #[test]
    fn get_number_format_ex_custom_and_locale() {
        #[repr(C)]
        struct NumberFmtW {
            num_digits: u32,
            leading_zero: u32,
            grouping: u32,
            dec_sep: LPCWSTR,
            thou_sep: LPCWSTR,
            neg_order: u32,
        }
        unsafe {
            let dec = to_wide_z(",");
            let thou = to_wide_z("_");
            let fmt = NumberFmtW {
                num_digits: 2,
                leading_zero: 0,
                grouping: 32,
                dec_sep: dec.as_ptr(),
                thou_sep: thou.as_ptr(),
                neg_order: 0, // (1.1)
            };
            let mut out = [0u16; 64];

            // Custom grouping (3, then 2) + half-up rounding + negative order 0 "(...)"
            let v1 = to_wide_z("-1234567.895");
            assert!(GetNumberFormatEx(std::ptr::null(), 0, v1.as_ptr(), &fmt as *const _ as *const c_void, out.as_mut_ptr(), 64) > 0);
            assert_eq!(from_wide(out.as_ptr()), "(12_34_567,90)");

            // Trailing-zero termination marker in NUMBERFMTW.Grouping (30 -> 123456_789; 320 -> 1234_56_789)
            let v_big = to_wide_z("123456789");
            let fmt_30 = NumberFmtW { grouping: 30, ..fmt };
            assert!(GetNumberFormatEx(std::ptr::null(), 0, v_big.as_ptr(), &fmt_30 as *const _ as *const c_void, out.as_mut_ptr(), 64) > 0);
            assert_eq!(from_wide(out.as_ptr()), "123456_789,00");
            let fmt_320 = NumberFmtW { grouping: 320, ..fmt };
            assert!(GetNumberFormatEx(std::ptr::null(), 0, v_big.as_ptr(), &fmt_320 as *const _ as *const c_void, out.as_mut_ptr(), 64) > 0);
            assert_eq!(from_wide(out.as_ptr()), "1234_56_789,00");

            // LeadingZero = 0 omits leading zero for 0.5
            let v2 = to_wide_z("0.5");
            assert!(GetNumberFormatEx(std::ptr::null(), 0, v2.as_ptr(), &fmt as *const _ as *const c_void, out.as_mut_ptr(), 64) > 0);
            assert_eq!(from_wide(out.as_ptr()), ",50");

            // Locale default formatting for de-DE vs en-US and secondary grouping for en-IN/hi-IN
            let de = to_wide_z("de-DE");
            let v3 = to_wide_z("1234.5");
            assert!(GetNumberFormatEx(de.as_ptr(), 0, v3.as_ptr(), std::ptr::null(), out.as_mut_ptr(), 64) > 0);
            assert_eq!(from_wide(out.as_ptr()), "1.234,50");

            let en_in = to_wide_z("en-IN");
            let v4 = to_wide_z("123456789");
            assert!(GetNumberFormatEx(en_in.as_ptr(), 0, v4.as_ptr(), std::ptr::null(), out.as_mut_ptr(), 64) > 0);
            assert_eq!(from_wide(out.as_ptr()), "12,34,56,789.00");
            assert!(GetLocaleInfoEx(en_in.as_ptr(), 0x10, out.as_mut_ptr(), 64) > 0);
            assert_eq!(from_wide(out.as_ptr()), "3;2;0");

            // Non-zero flags with custom NUMBERFMT fails with ERROR_INVALID_FLAGS
            assert_eq!(GetNumberFormatEx(std::ptr::null(), 1, v3.as_ptr(), &fmt as *const _ as *const c_void, out.as_mut_ptr(), 64), 0);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_FLAGS);
        }
    }

    #[test]
    fn get_calendar_info_ex_all_types() {
        unsafe {
            let de = to_wide_z("de-DE");
            let mut buf = [0u16; 128];
            let mut num = 0u32;

            assert_eq!(GetCalendarInfoEx(de.as_ptr(), 1, std::ptr::null(), 0x30 | CAL_RETURN_NUMBER, std::ptr::null_mut(), 0, &mut num), 2);
            assert_eq!(num, 2049);

            assert_eq!(GetCalendarInfoEx(de.as_ptr(), 1, std::ptr::null(), 0x01 | CAL_RETURN_NUMBER, std::ptr::null_mut(), 0, &mut num), 2);
            assert_eq!(num, 1);

            assert!(GetCalendarInfoEx(de.as_ptr(), 1, std::ptr::null(), 0x07, buf.as_mut_ptr(), 128, std::ptr::null_mut()) > 0);
            assert_eq!(from_wide(buf.as_ptr()), "Montag");

            assert!(GetCalendarInfoEx(de.as_ptr(), 1, std::ptr::null(), 0x15, buf.as_mut_ptr(), 128, std::ptr::null_mut()) > 0);
            assert_eq!(from_wide(buf.as_ptr()), "Januar");

            assert!(GetCalendarInfoEx(de.as_ptr(), 1, std::ptr::null(), 0x02, buf.as_mut_ptr(), 128, std::ptr::null_mut()) > 0);
            assert_eq!(from_wide(buf.as_ptr()), "Gregorian Calendar");

            // Invalid CALTYPE returns 0 + ERROR_INVALID_FLAGS
            assert_eq!(GetCalendarInfoEx(de.as_ptr(), 1, std::ptr::null(), 0x999, buf.as_mut_ptr(), 128, std::ptr::null_mut()), 0);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_FLAGS);

            // CAL_GREGORIAN_US (2) returns US English Gregorian names.
            assert!(GetCalendarInfoEx(de.as_ptr(), 2, std::ptr::null(), 0x07, buf.as_mut_ptr(), 128, std::ptr::null_mut()) > 0);
            assert_eq!(from_wide(buf.as_ptr()), "Monday");

            // Locales whose default ICU calendar is non-Gregorian (e.g. th-TH Buddhist) still
            // return localized Gregorian era & month data when CAL_GREGORIAN (1) is requested.
            let th = to_wide_z("th-TH");
            assert!(GetCalendarInfoEx(th.as_ptr(), 1, std::ptr::null(), 0x04 /* CAL_SERASTRING */, buf.as_mut_ptr(), 128, std::ptr::null_mut()) > 1);
            assert!(!from_wide(buf.as_ptr()).is_empty());
            assert!(GetCalendarInfoEx(th.as_ptr(), 1, std::ptr::null(), 0x15 /* CAL_SMONTHNAME1 */, buf.as_mut_ptr(), 128, std::ptr::null_mut()) > 1);
            assert_eq!(from_wide(buf.as_ptr()), "มกราคม");

            // Non-Gregorian calendars (e.g. CAL_JAPAN = 3) are rejected honestly with ERROR_INVALID_PARAMETER.
            let ja = to_wide_z("ja-JP");
            assert_eq!(GetCalendarInfoEx(ja.as_ptr(), 3, std::ptr::null(), 0x02, buf.as_mut_ptr(), 128, std::ptr::null_mut()), 0);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
            assert_eq!(GetCalendarInfoEx(de.as_ptr(), 0, std::ptr::null(), 0x02, buf.as_mut_ptr(), 128, std::ptr::null_mut()), 0);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
        }
    }

    #[test]
    fn encode_cp_tracks_icu_substitutions_across_dbcs_and_sbcs() {
        // CP932 (Shift-JIS): '€' is unmappable; ICU default would be FC FC unless configured.
        let (bytes, used) = encode_utf16_to_cp(932, &to_wide("A€B"), None).unwrap();
        assert!(used, "CP932 must report substitution for unmappable '€'");
        assert_eq!(bytes, b"A?B");

        // Caller-supplied default byte must be respected for DBCS too.
        let (bytes_custom, used_custom) = encode_utf16_to_cp(932, &to_wide("A€B"), Some(b'_')).unwrap();
        assert!(used_custom);
        assert_eq!(bytes_custom, b"A_B");

        // CP936 (GBK) / CP949 / CP950 / CP1252: unmappable supplementary char reports substitution.
        for cp in [932u32, 936, 949, 950, 1252] {
            let (b, u) = encode_utf16_to_cp(cp, &to_wide("X😀Y"), Some(b'!')).unwrap();
            assert!(u, "cp {cp} must report substitution for '😀'");
            assert_eq!(b, b"X!Y", "cp {cp} must emit caller default byte");
        }

        // Literal '?' and 0x1A must NOT falsely trigger used_default.
        let (lit_1252, used_1252) = encode_utf16_to_cp(1252, &to_wide("?\x1A€"), None).unwrap();
        assert!(!used_1252);
        assert_eq!(lit_1252, vec![b'?', 0x1A, 0x80]);
        let (lit_932, used_932) = encode_utf16_to_cp(932, &to_wide("?あ"), None).unwrap();
        assert!(!used_932);
        assert_eq!(lit_932, vec![b'?', 0x82, 0xA0]);

        // Strict MultiByteToWideChar(MB_ERR_INVALID_CHARS) rejects lone DBCS lead bytes and
        // invalid trail sequences with ERROR_NO_UNICODE_TRANSLATION; non-strict succeeds lossily.
        unsafe {
            let lone_lead = [0x81u8];
            let bad_pair = [0x81u8, 0x20];
            let mut wbuf = [0u16; 8];
            for cp in [932u32, 936, 949, 950] {
                assert_eq!(
                    MultiByteToWideChar(cp, MB_ERR_INVALID_CHARS, lone_lead.as_ptr(), 1, wbuf.as_mut_ptr(), 8),
                    0,
                    "cp {cp} strict decode must reject lone lead byte"
                );
                assert_eq!(teb::last_error(), err::ERROR_NO_UNICODE_TRANSLATION);
                assert_eq!(
                    MultiByteToWideChar(cp, MB_ERR_INVALID_CHARS, bad_pair.as_ptr(), 2, wbuf.as_mut_ptr(), 8),
                    0,
                    "cp {cp} strict decode must reject invalid DBCS sequence"
                );
                assert_eq!(teb::last_error(), err::ERROR_NO_UNICODE_TRANSLATION);
                assert!(
                    MultiByteToWideChar(cp, 0, lone_lead.as_ptr(), 1, wbuf.as_mut_ptr(), 8) > 0,
                    "cp {cp} non-strict decode must substitute"
                );
            }
        }
    }
}
