//! `<string.h>`, `<wchar.h>` string functions and `<ctype.h>`/`<wctype.h>`.
//!
//! Classification and case mapping follow the current CRT locale: in the
//! "C" locale only ASCII is classified/folded (like ucrt); after
//! `setlocale(LC_ALL, "")` Unicode rules apply (see [`super::locale`]).

use super::errno::{self, EINVAL, ERANGE, STRUNCATE};
use super::runtime::invalid_parameter;
use crate::heap;
use std::ffi::c_void;

// ---------------------------------------------------------------------------
// ctype table (C locale): 257 entries, index 0 = EOF.
// ---------------------------------------------------------------------------

pub const _UPPER: u16 = 0x1;
pub const _LOWER: u16 = 0x2;
pub const _DIGIT: u16 = 0x4;
pub const _SPACE: u16 = 0x8;
pub const _PUNCT: u16 = 0x10;
pub const _CONTROL: u16 = 0x20;
pub const _BLANK: u16 = 0x40;
pub const _HEX: u16 = 0x80;
pub const _ALPHA: u16 = 0x100;
pub const _LEADBYTE: u16 = 0x8000;

const fn ctype_of(c: u8) -> u16 {
    let mut t = 0;
    if c.is_ascii_uppercase() {
        t |= _UPPER | _ALPHA;
    }
    if c.is_ascii_lowercase() {
        t |= _LOWER | _ALPHA;
    }
    if c.is_ascii_digit() {
        t |= _DIGIT;
    }
    if c == b' ' || (c >= 0x09 && c <= 0x0D) {
        t |= _SPACE;
    }
    // MSVC: _BLANK marks only the space character (tab is _SPACE|_CONTROL).
    if c == b' ' {
        t |= _BLANK;
    }
    if c < 0x20 || c == 0x7F {
        t |= _CONTROL;
    }
    if c.is_ascii_punctuation() {
        t |= _PUNCT;
    }
    if c.is_ascii_hexdigit() {
        t |= _HEX;
    }
    t
}

const fn build_ctype() -> [u16; 257] {
    let mut t = [0u16; 257];
    let mut i = 0;
    while i < 256 {
        t[i + 1] = ctype_of(i as u8);
        i += 1;
    }
    t
}

pub static CTYPE: [u16; 257] = build_ctype();

struct LocaleCaseTables {
    is_c: bool,
    icu_locale: &'static str,
    ctype: [u16; 257],
    lower: [u8; 256],
    upper: [u8; 256],
    wlower_latin1: [u16; 256],
    wupper_latin1: [u16; 256],
}

const fn build_ascii_lower() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        t[i] = if i >= b'A' as usize && i <= b'Z' as usize { (i + 32) as u8 } else { i as u8 };
        i += 1;
    }
    t
}

const fn build_ascii_upper() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        t[i] = if i >= b'a' as usize && i <= b'z' as usize { (i - 32) as u8 } else { i as u8 };
        i += 1;
    }
    t
}

const fn build_ascii_wlower() -> [u16; 256] {
    let mut t = [0u16; 256];
    let mut i = 0;
    while i < 256 {
        t[i] = if i >= b'A' as usize && i <= b'Z' as usize { (i + 32) as u16 } else { i as u16 };
        i += 1;
    }
    t
}

const fn build_ascii_wupper() -> [u16; 256] {
    let mut t = [0u16; 256];
    let mut i = 0;
    while i < 256 {
        t[i] = if i >= b'a' as usize && i <= b'z' as usize { (i - 32) as u16 } else { i as u16 };
        i += 1;
    }
    t
}

static C_TABLES: LocaleCaseTables = LocaleCaseTables {
    is_c: true,
    icu_locale: "",
    ctype: build_ctype(),
    lower: build_ascii_lower(),
    upper: build_ascii_upper(),
    wlower_latin1: build_ascii_wlower(),
    wupper_latin1: build_ascii_wupper(),
};

static ACTIVE_TABLES: std::sync::atomic::AtomicPtr<LocaleCaseTables> =
    std::sync::atomic::AtomicPtr::new(&C_TABLES as *const LocaleCaseTables as *mut LocaleCaseTables);

/// Build and cache the 257-entry MSVC `__pctype_func` classification table (`index 0 = EOF`),
/// 256-entry narrow `tolower`/`toupper` tables, and 256-entry Latin-1 wide `towlower`/`towupper`
/// tables for `(cp, icu_locale)`.
fn tables_for_locale(cp: u32, icu_locale: &str) -> &'static LocaleCaseTables {
    if cp == 0 {
        return &C_TABLES;
    }
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    static TABLES: OnceLock<Mutex<HashMap<(u32, String), &'static LocaleCaseTables>>> = OnceLock::new();
    let m = TABLES.get_or_init(|| Mutex::new(HashMap::new()));
    let key = (cp, icu_locale.to_string());
    let mut g = m.lock().unwrap();
    if let Some(&tbl) = g.get(&key) {
        return tbl;
    }
    let mut ctype = CTYPE;
    let mut lower = C_TABLES.lower;
    let mut upper = C_TABLES.upper;
    let mut wlower_latin1 = [0u16; 256];
    let mut wupper_latin1 = [0u16; 256];
    for u in 0u16..256 {
        wlower_latin1[u as usize] = crate::kernel32::string::case_map_unit_icu(icu_locale, u, false);
        wupper_latin1[u as usize] = crate::kernel32::string::case_map_unit_icu(icu_locale, u, true);
    }
    for b in 0u8..=0xFF {
        if cp == 65001 && b >= 0x80 {
            continue;
        }
        let is_lead = cp != 65001 && unsafe { crate::kernel32::string::IsDBCSLeadByteEx(cp, b) != 0 };
        if is_lead {
            ctype[b as usize + 1] = _LEADBYTE;
            lower[b as usize] = b;
            upper[b as usize] = b;
            continue;
        }
        let unit = if b < 0x80 {
            Some(b as u16)
        } else {
            crate::kernel32::string::decode_cp_to_utf16_strict(cp, &[b], true)
                .ok()
                .filter(|u| u.len() == 1)
                .map(|u| u[0])
        };
        let Some(u) = unit else { continue };
        if b >= 0x80 {
            ctype[b as usize + 1] = crate::kernel32::string::ctype1(u) & 0x1FF;
        }
        // Apply locale-specific ICU case mapping BEFORE any ASCII fallback so Turkic locales
        // (`tr_TR`, `az_*`) map `'i'` -> `'İ'` (CP1254 `0xDD`) and `'I'` -> `'ı'` (`0xFD`).
        let lo_u = crate::kernel32::string::case_map_unit_icu(icu_locale, u, false);
        lower[b as usize] = if lo_u == u {
            b
        } else if cp == 65001 {
            if lo_u < 0x80 { lo_u as u8 } else { b }
        } else {
            crate::kernel32::string::encode_utf16_to_cp(cp, &[lo_u], None)
                .ok()
                .filter(|(out, used)| !used && out.len() == 1)
                .map(|(out, _)| out[0])
                .unwrap_or(b)
        };
        let up_u = crate::kernel32::string::case_map_unit_icu(icu_locale, u, true);
        upper[b as usize] = if up_u == u {
            b
        } else if cp == 65001 {
            if up_u < 0x80 { up_u as u8 } else { b }
        } else {
            crate::kernel32::string::encode_utf16_to_cp(cp, &[up_u], None)
                .ok()
                .filter(|(out, used)| !used && out.len() == 1)
                .map(|(out, _)| out[0])
                .unwrap_or(b)
        };
    }
    let leaked_loc: &'static str = Box::leak(icu_locale.to_string().into_boxed_str());
    let leaked: &'static LocaleCaseTables = Box::leak(Box::new(LocaleCaseTables {
        is_c: false,
        icu_locale: leaked_loc,
        ctype,
        lower,
        upper,
        wlower_latin1,
        wupper_latin1,
    }));
    g.insert(key, leaked);
    leaked
}

/// Called by `ucrt::locale` whenever global `LC_CTYPE` changes so `__pctype_func`, `is*`,
/// `tolower`/`toupper`, `towlower`/`towupper`, and case-insensitive compares observe the new
/// tables lock-free.
pub(super) fn refresh_global_ctype_tables(cp: u32, icu_locale: &str) {
    let tbl = tables_for_locale(cp, icu_locale);
    ACTIVE_TABLES.store(tbl as *const LocaleCaseTables as *mut LocaleCaseTables, std::sync::atomic::Ordering::Release);
}

#[inline]
fn active_tables() -> &'static LocaleCaseTables {
    unsafe { &*ACTIVE_TABLES.load(std::sync::atomic::Ordering::Acquire) }
}

unsafe fn tables_for_opt_locale(l: *mut c_void) -> &'static LocaleCaseTables {
    if l.is_null() {
        active_tables()
    } else {
        let (cp, icu_loc) = super::locale::ctype_cp_and_icu(l);
        tables_for_locale(cp, &icu_loc)
    }
}

pub unsafe extern "win64" fn __pctype_func() -> *const u16 {
    active_tables().ctype.as_ptr().add(1)
}
pub unsafe extern "win64" fn __pwctype_func() -> *const u16 {
    CTYPE.as_ptr().add(1)
}

/// Classification of an `int` argument as ucrt does it (EOF and 0..=255) for `cp`.
fn cls_cp(c: i32, cp: u32) -> u16 {
    if (-1..=255).contains(&c) {
        tables_for_locale(cp, "").ctype[(c + 1) as usize]
    } else {
        0
    }
}

/// Classification of an `int` argument as ucrt does it (EOF and 0..=255).
fn cls(c: i32) -> u16 {
    if (-1..=255).contains(&c) {
        active_tables().ctype[(c + 1) as usize]
    } else {
        0
    }
}

pub unsafe extern "win64" fn _isctype(c: i32, mask: i32) -> i32 {
    (cls(c) & mask as u16) as i32
}
pub unsafe extern "win64" fn _isctype_l(c: i32, mask: i32, l: *mut c_void) -> i32 {
    (cls_cp(c, super::locale::locale_obj_cp(l)) & mask as u16) as i32
}

macro_rules! ctype_fn {
    ($($name:ident, $name_l:ident => $mask:expr;)*) => {$(
        pub unsafe extern "win64" fn $name(c: i32) -> i32 { (cls(c) & ($mask)) as i32 }
        pub unsafe extern "win64" fn $name_l(c: i32, l: *mut c_void) -> i32 {
            (cls_cp(c, super::locale::locale_obj_cp(l)) & ($mask)) as i32
        }
    )*};
}

ctype_fn! {
    isalpha, _isalpha_l => _ALPHA;
    isupper, _isupper_l => _UPPER;
    islower, _islower_l => _LOWER;
    isdigit, _isdigit_l => _DIGIT;
    isxdigit, _isxdigit_l => _HEX;
    isspace, _isspace_l => _SPACE;
    ispunct, _ispunct_l => _PUNCT;
    isalnum, _isalnum_l => _ALPHA | _DIGIT;
    isprint, _isprint_l => _ALPHA | _DIGIT | _PUNCT | _BLANK;
    isgraph, _isgraph_l => _ALPHA | _DIGIT | _PUNCT;
    iscntrl, _iscntrl_l => _CONTROL;
}
// @exports isalpha _isalpha_l isupper _isupper_l islower _islower_l isdigit _isdigit_l isxdigit _isxdigit_l
// @exports isspace _isspace_l ispunct _ispunct_l isalnum _isalnum_l isprint _isprint_l isgraph _isgraph_l
// @exports iscntrl _iscntrl_l

pub unsafe extern "win64" fn isblank(c: i32) -> i32 {
    (c == b' ' as i32 || c == b'\t' as i32) as i32
}
pub unsafe extern "win64" fn __isascii(c: i32) -> i32 {
    ((c as u32) < 0x80) as i32
}
pub unsafe extern "win64" fn __toascii(c: i32) -> i32 {
    c & 0x7F
}
pub unsafe extern "win64" fn __iscsym(c: i32) -> i32 {
    (cls(c) & (_ALPHA | _DIGIT) != 0 || c == b'_' as i32) as i32
}
pub unsafe extern "win64" fn __iscsymf(c: i32) -> i32 {
    (cls(c) & _ALPHA != 0 || c == b'_' as i32) as i32
}

#[inline(never)]
pub unsafe extern "win64" fn tolower(c: i32) -> i32 {
    if (0..=255).contains(&c) {
        active_tables().lower[c as usize] as i32
    } else {
        c
    }
}
#[inline(never)]
pub unsafe extern "win64" fn toupper(c: i32) -> i32 {
    if (0..=255).contains(&c) {
        active_tables().upper[c as usize] as i32
    } else {
        c
    }
}
pub unsafe extern "win64" fn _tolower(c: i32) -> i32 {
    c + 32
}
pub unsafe extern "win64" fn _toupper(c: i32) -> i32 {
    c - 32
}
pub unsafe extern "win64" fn _tolower_l(c: i32, l: *mut c_void) -> i32 {
    if !(0..=255).contains(&c) {
        return c;
    }
    if l.is_null() {
        return active_tables().lower[c as usize] as i32;
    }
    let (cp, icu_loc) = super::locale::ctype_cp_and_icu(l);
    tables_for_locale(cp, &icu_loc).lower[c as usize] as i32
}
pub unsafe extern "win64" fn _toupper_l(c: i32, l: *mut c_void) -> i32 {
    if !(0..=255).contains(&c) {
        return c;
    }
    if l.is_null() {
        return active_tables().upper[c as usize] as i32;
    }
    let (cp, icu_loc) = super::locale::ctype_cp_and_icu(l);
    tables_for_locale(cp, &icu_loc).upper[c as usize] as i32
}

// Wide classification
fn wch(c: u16) -> Option<char> {
    char::from_u32(c as u32)
}

#[inline]
fn wlow_with(tbl: &LocaleCaseTables, c: u16) -> u16 {
    if c < 256 {
        tbl.wlower_latin1[c as usize]
    } else if tbl.is_c {
        c
    } else {
        crate::kernel32::string::case_map_unit_icu(tbl.icu_locale, c, false)
    }
}

#[inline]
fn wup_with(tbl: &LocaleCaseTables, c: u16) -> u16 {
    if c < 256 {
        tbl.wupper_latin1[c as usize]
    } else if tbl.is_c {
        c
    } else {
        crate::kernel32::string::case_map_unit_icu(tbl.icu_locale, c, true)
    }
}

fn wlow(c: u16) -> u16 {
    wlow_with(active_tables(), c)
}
fn wup(c: u16) -> u16 {
    wup_with(active_tables(), c)
}

pub unsafe extern "win64" fn towlower(c: u16) -> u16 {
    wlow(c)
}
pub unsafe extern "win64" fn towupper(c: u16) -> u16 {
    wup(c)
}
pub unsafe extern "win64" fn _towlower_l(c: u16, l: *mut c_void) -> u16 {
    wlow_with(tables_for_opt_locale(l), c)
}
pub unsafe extern "win64" fn _towupper_l(c: u16, l: *mut c_void) -> u16 {
    wup_with(tables_for_opt_locale(l), c)
}

/// Wide ctype bits (same encoding as `_pctype`), Unicode-aware for c >= 0x80.
pub fn wctype_bits(c: u16) -> u16 {
    if c < 256 {
        return CTYPE[c as usize + 1];
    }
    let k = crate::kernel32::string::ctype1(c);
    // C1_* and _* share the low 9 bits (UPPER..ALPHA).
    k & 0x1FF
}

macro_rules! wctype_fn {
    ($($name:ident => $mask:expr;)*) => {$(
        pub unsafe extern "win64" fn $name(c: u16) -> i32 { (wctype_bits(c) & ($mask)) as i32 }
    )*};
}

wctype_fn! {
    iswalpha => _ALPHA;
    iswupper => _UPPER;
    iswlower => _LOWER;
    iswdigit => _DIGIT;
    iswxdigit => _HEX;
    iswspace => _SPACE;
    iswpunct => _PUNCT;
    iswalnum => _ALPHA | _DIGIT;
    iswprint => _ALPHA | _DIGIT | _PUNCT | _BLANK;
    iswgraph => _ALPHA | _DIGIT | _PUNCT;
    iswcntrl => _CONTROL;
}
// @exports iswalpha iswupper iswlower iswdigit iswxdigit iswspace iswpunct iswalnum iswprint iswgraph iswcntrl

pub unsafe extern "win64" fn iswblank(c: u16) -> i32 {
    (c == b' ' as u16 || c == b'\t' as u16 || wch(c) == Some('\u{3000}')) as i32
}
pub unsafe extern "win64" fn iswctype(c: u16, mask: u16) -> i32 {
    (wctype_bits(c) & mask) as i32
}
pub unsafe extern "win64" fn iswascii(c: u16) -> i32 {
    (c < 0x80) as i32
}
pub unsafe extern "win64" fn _iswctype_l(c: u16, mask: u16, _l: *mut c_void) -> i32 {
    (wctype_bits(c) & mask) as i32
}

// ---------------------------------------------------------------------------
// Narrow strings
// ---------------------------------------------------------------------------

#[inline(never)]
pub unsafe extern "win64" fn strlen(s: *const u8) -> usize {
    crate::strings::strlen(s)
}
pub unsafe extern "win64" fn strnlen(s: *const u8, n: usize) -> usize {
    let mut i = 0;
    while i < n && *s.add(i) != 0 {
        i += 1;
    }
    i
}
#[inline(never)]
pub unsafe extern "win64" fn strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let (x, y) = (*a.add(i), *b.add(i));
        if x != y {
            return if x < y { -1 } else { 1 };
        }
        if x == 0 {
            return 0;
        }
        i += 1;
    }
}
#[inline(never)]
pub unsafe extern "win64" fn strncmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let (x, y) = (*a.add(i), *b.add(i));
        if x != y {
            return if x < y { -1 } else { 1 };
        }
        if x == 0 {
            return 0;
        }
    }
    0
}
unsafe fn strnicmp_with(a: *const u8, b: *const u8, n: usize, tbl: &LocaleCaseTables) -> i32 {
    if a.is_null() || b.is_null() {
        errno::set(EINVAL);
        invalid_parameter("_strnicmp");
        return i32::MAX;
    }
    for i in 0..n {
        let (x, y) = (tbl.lower[*a.add(i) as usize], tbl.lower[*b.add(i) as usize]);
        if x != y {
            return x as i32 - y as i32;
        }
        if x == 0 {
            return 0;
        }
    }
    0
}

pub unsafe extern "win64" fn _stricmp(a: *const u8, b: *const u8) -> i32 {
    strnicmp_with(a, b, usize::MAX, active_tables())
}
pub unsafe extern "win64" fn _strnicmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    strnicmp_with(a, b, n, active_tables())
}
pub unsafe extern "win64" fn _stricmp_l(a: *const u8, b: *const u8, l: *mut c_void) -> i32 {
    strnicmp_with(a, b, usize::MAX, tables_for_opt_locale(l))
}
pub unsafe extern "win64" fn _strnicmp_l(a: *const u8, b: *const u8, n: usize, l: *mut c_void) -> i32 {
    strnicmp_with(a, b, n, tables_for_opt_locale(l))
}
pub unsafe extern "win64" fn _strcmpi(a: *const u8, b: *const u8) -> i32 {
    _stricmp(a, b)
}
unsafe fn narrow_coll_impl(a: *const u8, b: *const u8, max: Option<usize>, ignore_case: bool) -> i32 {
    let (is_c, cp, bcp47) = super::locale::collate_cp_and_bcp47(std::ptr::null_mut());
    if is_c {
        return match (max, ignore_case) {
            (None, false) => strcmp(a, b),
            (None, true) => _stricmp(a, b),
            (Some(n), false) => strncmp(a, b, n),
            (Some(n), true) => _strnicmp(a, b, n),
        };
    }
    let slice_a = match max {
        Some(n) => std::slice::from_raw_parts(a, strnlen(a, n)),
        None => std::slice::from_raw_parts(a, strlen(a)),
    };
    let slice_b = match max {
        Some(n) => std::slice::from_raw_parts(b, strnlen(b, n)),
        None => std::slice::from_raw_parts(b, strlen(b)),
    };
    let wa = crate::kernel32::string::decode_cp_to_utf16(cp, slice_a)
        .unwrap_or_else(|_| String::from_utf8_lossy(slice_a).encode_utf16().collect());
    let wb = crate::kernel32::string::decode_cp_to_utf16(cp, slice_b)
        .unwrap_or_else(|_| String::from_utf8_lossy(slice_b).encode_utf16().collect());
    let loc_w = crate::strings::to_wide_z(&bcp47);
    let flags = if ignore_case { 1 /* NORM_IGNORECASE */ } else { 0 };
    let r = crate::kernel32::string::CompareStringEx(
        loc_w.as_ptr(),
        flags,
        wa.as_ptr(),
        wa.len() as i32,
        wb.as_ptr(),
        wb.len() as i32,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        0,
    );
    if r == 0 { 0 } else { r - 2 }
}

unsafe fn mbstrlen_cp(s: *const u8, cp: u32) -> usize {
    if s.is_null() {
        errno::set(EINVAL);
        invalid_parameter("_mbstrlen");
        return 0;
    }
    if cp == 0 {
        return strlen(s);
    }
    let bytes = std::slice::from_raw_parts(s, strlen(s));
    if cp == 65001 {
        return match std::str::from_utf8(bytes) {
            Ok(st) => st.chars().count(),
            Err(_) => {
                errno::set(super::errno::EILSEQ);
                usize::MAX
            }
        };
    }
    let mut i = 0usize;
    let mut count = 0usize;
    while i < bytes.len() {
        if crate::kernel32::string::IsDBCSLeadByteEx(cp, bytes[i]) != 0 {
            if i + 1 >= bytes.len() {
                errno::set(super::errno::EILSEQ);
                return usize::MAX;
            }
            i += 2;
        } else {
            i += 1;
        }
        count += 1;
    }
    count
}

pub unsafe extern "win64" fn _mbstrlen(s: *const u8) -> usize {
    mbstrlen_cp(s, super::locale::mb_cp())
}

pub unsafe extern "win64" fn _mbstrlen_l(s: *const u8, l: *mut c_void) -> usize {
    mbstrlen_cp(s, super::locale::locale_obj_cp(l))
}

unsafe fn mbscmp_cp(mut s1: *const u8, mut s2: *const u8, cp: u32) -> i32 {
    if s1.is_null() || s2.is_null() {
        errno::set(EINVAL);
        invalid_parameter("_mbscmp");
        return i32::MAX; // _NLSCMPERROR
    }
    let is_dbcs = cp != 0 && cp != 65001;
    loop {
        let mut c1 = *s1 as u16;
        s1 = s1.add(1);
        if is_dbcs && c1 != 0 && crate::kernel32::string::IsDBCSLeadByteEx(cp, c1 as u8) != 0 {
            let t1 = *s1 as u16;
            if t1 == 0 {
                c1 = 0;
            } else {
                s1 = s1.add(1);
                c1 = (c1 << 8) | t1;
            }
        }
        let mut c2 = *s2 as u16;
        s2 = s2.add(1);
        if is_dbcs && c2 != 0 && crate::kernel32::string::IsDBCSLeadByteEx(cp, c2 as u8) != 0 {
            let t2 = *s2 as u16;
            if t2 == 0 {
                c2 = 0;
            } else {
                s2 = s2.add(1);
                c2 = (c2 << 8) | t2;
            }
        }
        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }
        if c1 == 0 {
            return 0;
        }
    }
}

pub unsafe extern "win64" fn _mbscmp(s1: *const u8, s2: *const u8) -> i32 {
    mbscmp_cp(s1, s2, super::locale::_getmbcp().max(0) as u32)
}

pub unsafe extern "win64" fn _mbscmp_l(s1: *const u8, s2: *const u8, l: *mut c_void) -> i32 {
    let cp = if l.is_null() {
        super::locale::_getmbcp().max(0) as u32
    } else {
        super::locale::locale_obj_cp(l)
    };
    mbscmp_cp(s1, s2, cp)
}

pub unsafe extern "win64" fn strcoll(a: *const u8, b: *const u8) -> i32 {
    narrow_coll_impl(a, b, None, false)
}
pub unsafe extern "win64" fn _stricoll(a: *const u8, b: *const u8) -> i32 {
    narrow_coll_impl(a, b, None, true)
}
pub unsafe extern "win64" fn _strncoll(a: *const u8, b: *const u8, n: usize) -> i32 {
    narrow_coll_impl(a, b, Some(n), false)
}
pub unsafe extern "win64" fn _strnicoll(a: *const u8, b: *const u8, n: usize) -> i32 {
    narrow_coll_impl(a, b, Some(n), true)
}
pub unsafe extern "win64" fn strxfrm(d: *mut u8, s: *const u8, n: usize) -> usize {
    let (is_c, cp, bcp47) = super::locale::collate_cp_and_bcp47(std::ptr::null_mut());
    if is_c {
        let len = strlen(s);
        if len < n && !d.is_null() {
            std::ptr::copy_nonoverlapping(s, d, len + 1);
        }
        return len;
    }
    let bytes = std::slice::from_raw_parts(s, strlen(s));
    if bytes.is_empty() {
        if n > 0 && !d.is_null() {
            *d = 0;
        }
        return 0;
    }
    let w = crate::kernel32::string::decode_cp_to_utf16(cp, bytes)
        .unwrap_or_else(|_| String::from_utf8_lossy(bytes).encode_utf16().collect());
    if w.is_empty() {
        if n > 0 && !d.is_null() {
            *d = 0;
        }
        return 0;
    }
    let loc_w = crate::strings::to_wide_z(&bcp47);
    const LCMAP_SORTKEY: u32 = 0x0000_0400;
    let need = crate::kernel32::string::LCMapStringEx(
        loc_w.as_ptr(),
        LCMAP_SORTKEY,
        w.as_ptr(),
        w.len() as i32,
        std::ptr::null_mut(),
        0,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        0,
    );
    if need <= 0 {
        if n > 0 && !d.is_null() {
            *d = 0;
        }
        return 0;
    }
    let mut key = vec![0u8; need as usize];
    crate::kernel32::string::LCMapStringEx(
        loc_w.as_ptr(),
        LCMAP_SORTKEY,
        w.as_ptr(),
        w.len() as i32,
        key.as_mut_ptr() as *mut u16,
        need,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        0,
    );
    let len = key.iter().position(|&b| b == 0).unwrap_or(key.len());
    if len < n && !d.is_null() {
        std::ptr::copy_nonoverlapping(key.as_ptr(), d, len);
        *d.add(len) = 0;
    }
    len
}

pub unsafe extern "win64" fn strcpy(d: *mut u8, s: *const u8) -> *mut u8 {
    std::ptr::copy(s, d, strlen(s) + 1);
    d
}
pub unsafe extern "win64" fn strcat(d: *mut u8, s: *const u8) -> *mut u8 {
    strcpy(d.add(strlen(d)), s);
    d
}
#[inline(never)]
pub unsafe extern "win64" fn strncpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n && *s.add(i) != 0 {
        *d.add(i) = *s.add(i);
        i += 1;
    }
    while i < n {
        *d.add(i) = 0;
        i += 1;
    }
    d
}
pub unsafe extern "win64" fn strncat(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
    let dl = strlen(d);
    let mut i = 0;
    while i < n && *s.add(i) != 0 {
        *d.add(dl + i) = *s.add(i);
        i += 1;
    }
    *d.add(dl + i) = 0;
    d
}

unsafe fn bad_s(what: &str, d: *mut u8, sz: usize, e: i32) -> i32 {
    if !d.is_null() && sz > 0 {
        *d = 0;
    }
    errno::set(e);
    invalid_parameter(what);
    e
}

pub unsafe extern "win64" fn strcpy_s(d: *mut u8, sz: usize, s: *const u8) -> i32 {
    if d.is_null() || sz == 0 {
        return bad_s("strcpy_s", std::ptr::null_mut(), 0, EINVAL);
    }
    if s.is_null() {
        return bad_s("strcpy_s", d, sz, EINVAL);
    }
    let n = strlen(s);
    if n >= sz {
        return bad_s("strcpy_s", d, sz, ERANGE);
    }
    std::ptr::copy(s, d, n + 1);
    0
}
pub unsafe extern "win64" fn strcat_s(d: *mut u8, sz: usize, s: *const u8) -> i32 {
    if d.is_null() || sz == 0 {
        return bad_s("strcat_s", std::ptr::null_mut(), 0, EINVAL);
    }
    if s.is_null() {
        return bad_s("strcat_s", d, sz, EINVAL);
    }
    let dl = strnlen(d, sz);
    if dl == sz {
        return bad_s("strcat_s", d, sz, EINVAL);
    }
    let n = strlen(s);
    if dl + n >= sz {
        return bad_s("strcat_s", d, sz, ERANGE);
    }
    std::ptr::copy(s, d.add(dl), n + 1);
    0
}
const TRUNCATE: usize = usize::MAX;
pub unsafe extern "win64" fn strncpy_s(d: *mut u8, sz: usize, s: *const u8, count: usize) -> i32 {
    if count == 0 && d.is_null() && sz == 0 {
        return 0;
    }
    if d.is_null() || sz == 0 {
        return bad_s("strncpy_s", std::ptr::null_mut(), 0, EINVAL);
    }
    if count == 0 {
        *d = 0;
        return 0;
    }
    if s.is_null() {
        return bad_s("strncpy_s", d, sz, EINVAL);
    }
    let n = strnlen(s, count.min(if count == TRUNCATE { sz } else { count }));
    if count == TRUNCATE {
        let n = strnlen(s, sz);
        if n >= sz {
            std::ptr::copy(s, d, sz - 1);
            *d.add(sz - 1) = 0;
            return STRUNCATE;
        }
        std::ptr::copy(s, d, n);
        *d.add(n) = 0;
        return 0;
    }
    if n >= sz {
        return bad_s("strncpy_s", d, sz, ERANGE);
    }
    std::ptr::copy(s, d, n);
    *d.add(n) = 0;
    0
}
pub unsafe extern "win64" fn strncat_s(d: *mut u8, sz: usize, s: *const u8, count: usize) -> i32 {
    if d.is_null() || sz == 0 {
        return bad_s("strncat_s", std::ptr::null_mut(), 0, EINVAL);
    }
    if count == 0 {
        return 0;
    }
    if s.is_null() {
        return bad_s("strncat_s", d, sz, EINVAL);
    }
    let dl = strnlen(d, sz);
    if dl == sz {
        return bad_s("strncat_s", d, sz, EINVAL);
    }
    let avail = sz - dl;
    if count == TRUNCATE {
        let n = strnlen(s, avail);
        if n >= avail {
            std::ptr::copy(s, d.add(dl), avail - 1);
            *d.add(sz - 1) = 0;
            return STRUNCATE;
        }
        std::ptr::copy(s, d.add(dl), n);
        *d.add(dl + n) = 0;
        return 0;
    }
    let n = strnlen(s, count);
    if n >= avail {
        return bad_s("strncat_s", d, sz, ERANGE);
    }
    std::ptr::copy(s, d.add(dl), n);
    *d.add(dl + n) = 0;
    0
}

pub unsafe extern "win64" fn _strdup(s: *const u8) -> *mut u8 {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    let n = strlen(s) + 1;
    let p = heap::alloc(n, false) as *mut u8;
    if !p.is_null() {
        std::ptr::copy_nonoverlapping(s, p, n);
    }
    p
}

fn in_set(set: *const u8, c: u8) -> bool {
    unsafe {
        let mut p = set;
        while *p != 0 {
            if *p == c {
                return true;
            }
            p = p.add(1);
        }
    }
    false
}

pub unsafe extern "win64" fn strspn(s: *const u8, set: *const u8) -> usize {
    let mut i = 0;
    while *s.add(i) != 0 && in_set(set, *s.add(i)) {
        i += 1;
    }
    i
}
pub unsafe extern "win64" fn strcspn(s: *const u8, set: *const u8) -> usize {
    let mut i = 0;
    while *s.add(i) != 0 && !in_set(set, *s.add(i)) {
        i += 1;
    }
    i
}
pub unsafe extern "win64" fn strpbrk(s: *const u8, set: *const u8) -> *const u8 {
    let i = strcspn(s, set);
    if *s.add(i) != 0 {
        s.add(i)
    } else {
        std::ptr::null()
    }
}

thread_local! {
    static STRTOK: std::cell::Cell<*mut u8> = const { std::cell::Cell::new(std::ptr::null_mut()) };
    static WCSTOK: std::cell::Cell<*mut u16> = const { std::cell::Cell::new(std::ptr::null_mut()) };
}

pub unsafe extern "win64" fn strtok_s(s: *mut u8, delim: *const u8, ctx: *mut *mut u8) -> *mut u8 {
    let mut p = if s.is_null() { *ctx } else { s };
    if p.is_null() {
        return std::ptr::null_mut();
    }
    p = p.add(strspn(p, delim));
    if *p == 0 {
        *ctx = p;
        return std::ptr::null_mut();
    }
    let start = p;
    p = p.add(strcspn(p, delim));
    if *p != 0 {
        *p = 0;
        p = p.add(1);
    }
    *ctx = p;
    start
}
pub unsafe extern "win64" fn strtok(s: *mut u8, delim: *const u8) -> *mut u8 {
    STRTOK.with(|c| {
        let mut ctx = c.get();
        let r = strtok_s(s, delim, &mut ctx);
        c.set(ctx);
        r
    })
}

pub unsafe extern "win64" fn __strncnt(s: *const u8, n: usize) -> usize {
    strnlen(s, n)
}

pub unsafe extern "win64" fn _strlwr(s: *mut u8) -> *mut u8 {
    let tbl = active_tables();
    let mut p = s;
    while *p != 0 {
        *p = tbl.lower[*p as usize];
        p = p.add(1);
    }
    s
}
pub unsafe extern "win64" fn _strupr(s: *mut u8) -> *mut u8 {
    let tbl = active_tables();
    let mut p = s;
    while *p != 0 {
        *p = tbl.upper[*p as usize];
        p = p.add(1);
    }
    s
}
pub unsafe extern "win64" fn _strlwr_s(s: *mut u8, sz: usize) -> i32 {
    if s.is_null() || strnlen(s, sz) == sz {
        return bad_s("_strlwr_s", s, sz, EINVAL);
    }
    _strlwr(s);
    0
}
pub unsafe extern "win64" fn _strupr_s(s: *mut u8, sz: usize) -> i32 {
    if s.is_null() || strnlen(s, sz) == sz {
        return bad_s("_strupr_s", s, sz, EINVAL);
    }
    _strupr(s);
    0
}
pub unsafe extern "win64" fn _strrev(s: *mut u8) -> *mut u8 {
    let n = strlen(s);
    std::slice::from_raw_parts_mut(s, n).reverse();
    s
}
pub unsafe extern "win64" fn _strset(s: *mut u8, c: i32) -> *mut u8 {
    std::ptr::write_bytes(s, c as u8, strlen(s));
    s
}
pub unsafe extern "win64" fn _strnset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    std::ptr::write_bytes(s, c as u8, strnlen(s, n));
    s
}

pub unsafe extern "win64" fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    crate::misc::vcruntime140::memcpy(dst, src, n)
}
pub unsafe extern "win64" fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    crate::misc::vcruntime140::memmove(dst, src, n)
}
pub unsafe extern "win64" fn memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    crate::misc::vcruntime140::memset(dst, c, n)
}
pub unsafe extern "win64" fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32 {
    crate::misc::vcruntime140::memcmp(a, b, n)
}
pub unsafe extern "win64" fn memchr(s: *const c_void, c: i32, n: usize) -> *mut c_void {
    crate::misc::vcruntime140::memchr(s, c, n)
}
pub unsafe extern "win64" fn strchr(s: *const std::ffi::c_char, c: i32) -> *mut std::ffi::c_char {
    crate::misc::vcruntime140::strchr(s, c)
}
pub unsafe extern "win64" fn strrchr(s: *const std::ffi::c_char, c: i32) -> *mut std::ffi::c_char {
    crate::misc::vcruntime140::strrchr(s, c)
}
pub unsafe extern "win64" fn strstr(
    haystack: *const std::ffi::c_char,
    needle: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    crate::misc::vcruntime140::strstr(haystack, needle)
}

pub unsafe extern "win64" fn memcpy_s(d: *mut c_void, dsz: usize, s: *const c_void, n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    if d.is_null() {
        errno::set(EINVAL);
        invalid_parameter("memcpy_s");
        return EINVAL;
    }
    if s.is_null() || dsz < n {
        std::ptr::write_bytes(d as *mut u8, 0, dsz);
        let e = if s.is_null() { EINVAL } else { ERANGE };
        errno::set(e);
        invalid_parameter("memcpy_s");
        return e;
    }
    // Match MSVC tolerance for overlapping/unaligned guest buffers without debug ub_checks panics.
    std::ptr::copy(s as *const u8, d as *mut u8, n);
    0
}
pub unsafe extern "win64" fn memmove_s(d: *mut c_void, dsz: usize, s: *const c_void, n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    if d.is_null() || s.is_null() || dsz < n {
        let e = if dsz < n { ERANGE } else { EINVAL };
        errno::set(e);
        invalid_parameter("memmove_s");
        return e;
    }
    std::ptr::copy(s as *const u8, d as *mut u8, n);
    0
}
pub unsafe extern "win64" fn wmemcpy_s(d: *mut u16, dsz: usize, s: *const u16, n: usize) -> i32 {
    memcpy_s(d as _, dsz * 2, s as _, n * 2)
}
pub unsafe extern "win64" fn wmemmove_s(d: *mut u16, dsz: usize, s: *const u16, n: usize) -> i32 {
    memmove_s(d as _, dsz * 2, s as _, n * 2)
}
pub unsafe extern "win64" fn _memicmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let (x, y) = ((*a.add(i)).to_ascii_lowercase(), (*b.add(i)).to_ascii_lowercase());
        if x != y {
            return x as i32 - y as i32;
        }
    }
    0
}
pub unsafe extern "win64" fn _memccpy(d: *mut u8, s: *const u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        *d.add(i) = *s.add(i);
        if *s.add(i) == c as u8 {
            return d.add(i + 1);
        }
    }
    std::ptr::null_mut()
}

// ---------------------------------------------------------------------------
// Wide strings
// ---------------------------------------------------------------------------

#[inline(never)]
pub unsafe extern "win64" fn wcslen(s: *const u16) -> usize {
    crate::strings::wcslen(s)
}
pub unsafe extern "win64" fn wcsnlen(s: *const u16, n: usize) -> usize {
    let mut i = 0;
    while i < n && s.add(i).read_unaligned() != 0 {
        i += 1;
    }
    i
}
pub unsafe extern "win64" fn wcscmp(a: *const u16, b: *const u16) -> i32 {
    wcsncmp(a, b, usize::MAX)
}
pub unsafe extern "win64" fn wcsncmp(a: *const u16, b: *const u16, n: usize) -> i32 {
    for i in 0..n {
        let (x, y) = (*a.add(i), *b.add(i));
        if x != y {
            return if x < y { -1 } else { 1 };
        }
        if x == 0 {
            return 0;
        }
    }
    0
}
unsafe fn wcsnicmp_with(a: *const u16, b: *const u16, n: usize, tbl: &LocaleCaseTables) -> i32 {
    if a.is_null() || b.is_null() {
        errno::set(EINVAL);
        invalid_parameter("_wcsnicmp");
        return i32::MAX;
    }
    for i in 0..n {
        let ca = std::ptr::read_unaligned(a.add(i));
        let cb = std::ptr::read_unaligned(b.add(i));
        let (x, y) = (wlow_with(tbl, ca), wlow_with(tbl, cb));
        if x != y {
            return x as i32 - y as i32;
        }
        if x == 0 {
            return 0;
        }
    }
    0
}

pub unsafe extern "win64" fn _wcsicmp(a: *const u16, b: *const u16) -> i32 {
    wcsnicmp_with(a, b, usize::MAX, active_tables())
}
pub unsafe extern "win64" fn _wcsnicmp(a: *const u16, b: *const u16, n: usize) -> i32 {
    wcsnicmp_with(a, b, n, active_tables())
}
pub unsafe extern "win64" fn _wcsicmp_l(a: *const u16, b: *const u16, l: *mut c_void) -> i32 {
    wcsnicmp_with(a, b, usize::MAX, tables_for_opt_locale(l))
}
pub unsafe extern "win64" fn _wcsnicmp_l(a: *const u16, b: *const u16, n: usize, l: *mut c_void) -> i32 {
    wcsnicmp_with(a, b, n, tables_for_opt_locale(l))
}
unsafe fn wide_coll_impl(a: *const u16, b: *const u16, max: Option<usize>, ignore_case: bool) -> i32 {
    let (is_c, _, bcp47) = super::locale::collate_cp_and_bcp47(std::ptr::null_mut());
    if is_c {
        return match (max, ignore_case) {
            (None, false) => wcscmp(a, b),
            (None, true) => _wcsicmp(a, b),
            (Some(n), false) => wcsncmp(a, b, n),
            (Some(n), true) => _wcsnicmp(a, b, n),
        };
    }
    let la = match max {
        Some(n) => wcsnlen(a, n) as i32,
        None => wcslen(a) as i32,
    };
    let lb = match max {
        Some(n) => wcsnlen(b, n) as i32,
        None => wcslen(b) as i32,
    };
    let loc_w = crate::strings::to_wide_z(&bcp47);
    let flags = if ignore_case { 1 /* NORM_IGNORECASE */ } else { 0 };
    let r = crate::kernel32::string::CompareStringEx(
        loc_w.as_ptr(),
        flags,
        a,
        la,
        b,
        lb,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        0,
    );
    if r == 0 { 0 } else { r - 2 }
}

pub unsafe extern "win64" fn wcscoll(a: *const u16, b: *const u16) -> i32 {
    wide_coll_impl(a, b, None, false)
}
pub unsafe extern "win64" fn _wcsicoll(a: *const u16, b: *const u16) -> i32 {
    wide_coll_impl(a, b, None, true)
}
pub unsafe extern "win64" fn _wcsncoll(a: *const u16, b: *const u16, n: usize) -> i32 {
    wide_coll_impl(a, b, Some(n), false)
}
pub unsafe extern "win64" fn _wcsnicoll(a: *const u16, b: *const u16, n: usize) -> i32 {
    wide_coll_impl(a, b, Some(n), true)
}
pub unsafe extern "win64" fn wcsxfrm(d: *mut u16, s: *const u16, n: usize) -> usize {
    let (is_c, _, bcp47) = super::locale::collate_cp_and_bcp47(std::ptr::null_mut());
    let slen = wcslen(s);
    if is_c {
        if slen < n && !d.is_null() {
            std::ptr::copy_nonoverlapping(s as *const u8, d as *mut u8, (slen + 1) * 2);
        }
        return slen;
    }
    if slen == 0 {
        if n > 0 && !d.is_null() {
            std::ptr::write_unaligned(d, 0);
        }
        return 0;
    }
    let loc_w = crate::strings::to_wide_z(&bcp47);
    const LCMAP_SORTKEY: u32 = 0x0000_0400;
    let need = crate::kernel32::string::LCMapStringEx(
        loc_w.as_ptr(),
        LCMAP_SORTKEY,
        s,
        slen as i32,
        std::ptr::null_mut(),
        0,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        0,
    );
    if need <= 0 {
        if n > 0 && !d.is_null() {
            std::ptr::write_unaligned(d, 0);
        }
        return 0;
    }
    let mut key = vec![0u8; need as usize];
    crate::kernel32::string::LCMapStringEx(
        loc_w.as_ptr(),
        LCMAP_SORTKEY,
        s,
        slen as i32,
        key.as_mut_ptr() as *mut u16,
        need,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        0,
    );
    let len = key.iter().position(|&b| b == 0).unwrap_or(key.len());
    if len < n && !d.is_null() {
        for (i, &b) in key[..len].iter().enumerate() {
            std::ptr::write_unaligned(d.add(i), b as u16);
        }
        std::ptr::write_unaligned(d.add(len), 0);
    }
    len
}
pub unsafe extern "win64" fn wcschr(s: *const u16, c: u16) -> *mut u16 {
    crate::misc::vcruntime140::wcschr(s, c)
}
pub unsafe extern "win64" fn wcsrchr(s: *const u16, c: u16) -> *mut u16 {
    crate::misc::vcruntime140::wcsrchr(s, c)
}
pub unsafe extern "win64" fn wcsstr(haystack: *const u16, needle: *const u16) -> *mut u16 {
    crate::misc::vcruntime140::wcsstr(haystack, needle)
}
pub unsafe extern "win64" fn wcscpy(d: *mut u16, s: *const u16) -> *mut u16 {
    std::ptr::copy(s, d, wcslen(s) + 1);
    d
}
pub unsafe extern "win64" fn wcscat(d: *mut u16, s: *const u16) -> *mut u16 {
    wcscpy(d.add(wcslen(d)), s);
    d
}
pub unsafe extern "win64" fn wcsncpy(d: *mut u16, s: *const u16, n: usize) -> *mut u16 {
    let mut i = 0;
    while i < n && *s.add(i) != 0 {
        *d.add(i) = *s.add(i);
        i += 1;
    }
    while i < n {
        *d.add(i) = 0;
        i += 1;
    }
    d
}
pub unsafe extern "win64" fn wcsncat(d: *mut u16, s: *const u16, n: usize) -> *mut u16 {
    let dl = wcslen(d);
    let mut i = 0;
    while i < n && *s.add(i) != 0 {
        *d.add(dl + i) = *s.add(i);
        i += 1;
    }
    *d.add(dl + i) = 0;
    d
}

unsafe fn bad_ws(what: &str, d: *mut u16, sz: usize, e: i32) -> i32 {
    if !d.is_null() && sz > 0 {
        *d = 0;
    }
    errno::set(e);
    invalid_parameter(what);
    e
}

pub unsafe extern "win64" fn wcscpy_s(d: *mut u16, sz: usize, s: *const u16) -> i32 {
    if d.is_null() || sz == 0 {
        return bad_ws("wcscpy_s", std::ptr::null_mut(), 0, EINVAL);
    }
    if s.is_null() {
        return bad_ws("wcscpy_s", d, sz, EINVAL);
    }
    let n = wcslen(s);
    if n >= sz {
        return bad_ws("wcscpy_s", d, sz, ERANGE);
    }
    std::ptr::copy(s, d, n + 1);
    0
}
pub unsafe extern "win64" fn wcscat_s(d: *mut u16, sz: usize, s: *const u16) -> i32 {
    if d.is_null() || sz == 0 {
        return bad_ws("wcscat_s", std::ptr::null_mut(), 0, EINVAL);
    }
    if s.is_null() {
        return bad_ws("wcscat_s", d, sz, EINVAL);
    }
    let dl = wcsnlen(d, sz);
    if dl == sz {
        return bad_ws("wcscat_s", d, sz, EINVAL);
    }
    let n = wcslen(s);
    if dl + n >= sz {
        return bad_ws("wcscat_s", d, sz, ERANGE);
    }
    std::ptr::copy(s, d.add(dl), n + 1);
    0
}
pub unsafe extern "win64" fn wcsncpy_s(d: *mut u16, sz: usize, s: *const u16, count: usize) -> i32 {
    if count == 0 && d.is_null() && sz == 0 {
        return 0;
    }
    if d.is_null() || sz == 0 {
        return bad_ws("wcsncpy_s", std::ptr::null_mut(), 0, EINVAL);
    }
    if count == 0 {
        *d = 0;
        return 0;
    }
    if s.is_null() {
        return bad_ws("wcsncpy_s", d, sz, EINVAL);
    }
    if count == TRUNCATE {
        let n = wcsnlen(s, sz);
        if n >= sz {
            std::ptr::copy(s, d, sz - 1);
            *d.add(sz - 1) = 0;
            return STRUNCATE;
        }
        std::ptr::copy(s, d, n);
        *d.add(n) = 0;
        return 0;
    }
    let n = wcsnlen(s, count);
    if n >= sz {
        return bad_ws("wcsncpy_s", d, sz, ERANGE);
    }
    std::ptr::copy(s, d, n);
    *d.add(n) = 0;
    0
}
pub unsafe extern "win64" fn wcsncat_s(d: *mut u16, sz: usize, s: *const u16, count: usize) -> i32 {
    if d.is_null() || sz == 0 {
        return bad_ws("wcsncat_s", std::ptr::null_mut(), 0, EINVAL);
    }
    if count == 0 {
        return 0;
    }
    if s.is_null() {
        return bad_ws("wcsncat_s", d, sz, EINVAL);
    }
    let dl = wcsnlen(d, sz);
    if dl == sz {
        return bad_ws("wcsncat_s", d, sz, EINVAL);
    }
    let avail = sz - dl;
    if count == TRUNCATE {
        let n = wcsnlen(s, avail);
        if n >= avail {
            std::ptr::copy(s, d.add(dl), avail - 1);
            *d.add(sz - 1) = 0;
            return STRUNCATE;
        }
        std::ptr::copy(s, d.add(dl), n);
        *d.add(dl + n) = 0;
        return 0;
    }
    let n = wcsnlen(s, count);
    if n >= avail {
        return bad_ws("wcsncat_s", d, sz, ERANGE);
    }
    std::ptr::copy(s, d.add(dl), n);
    *d.add(dl + n) = 0;
    0
}

pub unsafe extern "win64" fn _wcsdup(s: *const u16) -> *mut u16 {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    let n = wcslen(s) + 1;
    let p = heap::alloc(n * 2, false) as *mut u16;
    if !p.is_null() {
        std::ptr::copy_nonoverlapping(s as *const u8, p as *mut u8, n * 2);
    }
    p
}

fn win_set(set: *const u16, c: u16) -> bool {
    unsafe {
        let mut p = set;
        while *p != 0 {
            if *p == c {
                return true;
            }
            p = p.add(1);
        }
    }
    false
}
pub unsafe extern "win64" fn wcsspn(s: *const u16, set: *const u16) -> usize {
    let mut i = 0;
    while *s.add(i) != 0 && win_set(set, *s.add(i)) {
        i += 1;
    }
    i
}
pub unsafe extern "win64" fn wcscspn(s: *const u16, set: *const u16) -> usize {
    let mut i = 0;
    while *s.add(i) != 0 && !win_set(set, *s.add(i)) {
        i += 1;
    }
    i
}
pub unsafe extern "win64" fn wcspbrk(s: *const u16, set: *const u16) -> *const u16 {
    let i = wcscspn(s, set);
    if *s.add(i) != 0 {
        s.add(i)
    } else {
        std::ptr::null()
    }
}
pub unsafe extern "win64" fn wcstok_s(s: *mut u16, delim: *const u16, ctx: *mut *mut u16) -> *mut u16 {
    let mut p = if s.is_null() { *ctx } else { s };
    if p.is_null() {
        return std::ptr::null_mut();
    }
    p = p.add(wcsspn(p, delim));
    if *p == 0 {
        *ctx = p;
        return std::ptr::null_mut();
    }
    let start = p;
    p = p.add(wcscspn(p, delim));
    if *p != 0 {
        *p = 0;
        p = p.add(1);
    }
    *ctx = p;
    start
}
/// ISO `wcstok(s, delim, ctx)` (ucrt exports the 3-argument form).
pub unsafe extern "win64" fn wcstok(s: *mut u16, delim: *const u16, ctx: *mut *mut u16) -> *mut u16 {
    if ctx.is_null() {
        return WCSTOK.with(|c| {
            let mut x = c.get();
            let r = wcstok_s(s, delim, &mut x);
            c.set(x);
            r
        });
    }
    wcstok_s(s, delim, ctx)
}
pub unsafe extern "win64" fn _wcstok(s: *mut u16, delim: *const u16) -> *mut u16 {
    wcstok(s, delim, std::ptr::null_mut())
}

pub unsafe extern "win64" fn _wcslwr(s: *mut u16) -> *mut u16 {
    let tbl = active_tables();
    let mut p = s;
    loop {
        let c = std::ptr::read_unaligned(p);
        if c == 0 {
            break;
        }
        std::ptr::write_unaligned(p, wlow_with(tbl, c));
        p = p.add(1);
    }
    s
}
pub unsafe extern "win64" fn _wcsupr(s: *mut u16) -> *mut u16 {
    let tbl = active_tables();
    let mut p = s;
    loop {
        let c = std::ptr::read_unaligned(p);
        if c == 0 {
            break;
        }
        std::ptr::write_unaligned(p, wup_with(tbl, c));
        p = p.add(1);
    }
    s
}
pub unsafe extern "win64" fn _wcslwr_s(s: *mut u16, sz: usize) -> i32 {
    if s.is_null() || wcsnlen(s, sz) == sz {
        return bad_ws("_wcslwr_s", s, sz, EINVAL);
    }
    _wcslwr(s);
    0
}
pub unsafe extern "win64" fn _wcsupr_s(s: *mut u16, sz: usize) -> i32 {
    if s.is_null() || wcsnlen(s, sz) == sz {
        return bad_ws("_wcsupr_s", s, sz, EINVAL);
    }
    _wcsupr(s);
    0
}
pub unsafe extern "win64" fn _wcsrev(s: *mut u16) -> *mut u16 {
    let n = wcslen(s);
    std::slice::from_raw_parts_mut(s, n).reverse();
    s
}
pub unsafe extern "win64" fn _wcsset(s: *mut u16, c: u16) -> *mut u16 {
    for i in 0..wcslen(s) {
        *s.add(i) = c;
    }
    s
}
pub unsafe extern "win64" fn _wcsnset(s: *mut u16, c: u16, n: usize) -> *mut u16 {
    for i in 0..wcsnlen(s, n) {
        *s.add(i) = c;
    }
    s
}

// Wide versions of vcruntime-less helpers (wmem* are inline in headers, but
// some code imports them).
pub unsafe extern "win64" fn wmemset(d: *mut u16, c: u16, n: usize) -> *mut u16 {
    for i in 0..n {
        *d.add(i) = c;
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ctype_table() {
        unsafe {
            assert_ne!(isdigit(b'5' as i32), 0);
            assert_eq!(isdigit(b'a' as i32), 0);
            assert_ne!(isspace(b'\n' as i32), 0);
            assert_eq!(isalpha(-1), 0);
            assert_eq!(*__pctype_func().offset(-1), 0);

            // Narrow ctype & case mapping honor the active LC_CTYPE code page (CP1252 de-DE).
            assert_eq!(isalpha(0xE4), 0); // "C" locale: ASCII-only
            assert_eq!(isprint(0xE4), 0);
            assert!(!crate::ucrt::locale::setlocale(0, b"de-DE\0".as_ptr()).is_null());
            assert_ne!(isalpha(0xE4), 0); // 'ä' in CP1252
            assert_ne!(islower(0xE4), 0);
            assert_ne!(isprint(0xE4), 0);
            assert_ne!(*__pctype_func().add(0xE4) & _ALPHA, 0);
            assert_eq!(toupper(0xE4), 0xC4); // 'Ä'
            assert_eq!(tolower(0xC4), 0xE4); // 'ä'
            // Turkish locale (CP1254) applies Turkic dotted/dotless-I casing before ASCII rules.
            assert!(!crate::ucrt::locale::setlocale(0, b"tr-TR\0".as_ptr()).is_null());
            assert_eq!(toupper(b'i' as i32), 0xDD); // 'İ' in CP1254
            assert_eq!(tolower(b'I' as i32), 0xFD); // 'ı' in CP1254
            assert_eq!(toupper(0xFD), b'I' as i32); // 'ı' -> 'I'
            assert_eq!(tolower(0xDD), b'i' as i32); // 'İ' -> 'i'
            // UTF-8 locale leaves bytes >= 0x80 unclassified.
            assert!(!crate::ucrt::locale::setlocale(0, b"en-US.utf8\0".as_ptr()).is_null());
            assert_eq!(isalpha(0xE4), 0);
            assert_eq!(isprint(0xE4), 0);
            // Explicit _locale_t honors tr-TR narrow + wide casing even while global locale is "C".
            assert!(!crate::ucrt::locale::setlocale(0, b"C\0".as_ptr()).is_null());
            let l_tr = crate::ucrt::locale::_create_locale(0, b"tr-TR\0".as_ptr());
            assert!(!l_tr.is_null());
            assert_eq!(_toupper_l(b'i' as i32, l_tr), 0xDD);
            assert_eq!(_tolower_l(b'I' as i32, l_tr), 0xFD);
            assert_eq!(_towupper_l(b'i' as u16, l_tr), 0x0130); // 'İ'
            assert_eq!(_towlower_l(b'I' as u16, l_tr), 0x0131); // 'ı'
            assert_eq!(_towlower_l(0x0130, l_tr), b'i' as u16);
            assert_eq!(_towupper_l(0x0131, l_tr), b'I' as u16);
            let wi_up = [b'I' as u16, 0];
            let wi_lo = [b'i' as u16, 0];
            let wi_dotless = [0x0131u16, 0];
            let wi_dotted = [0x0130u16, 0];
            assert_ne!(_wcsicmp_l(wi_up.as_ptr(), wi_lo.as_ptr(), l_tr), 0);
            assert_eq!(_wcsicmp_l(wi_up.as_ptr(), wi_dotless.as_ptr(), l_tr), 0);
            assert_eq!(_wcsicmp_l(wi_dotted.as_ptr(), wi_lo.as_ptr(), l_tr), 0);
            assert_ne!(_stricmp_l(b"I\0".as_ptr(), b"i\0".as_ptr(), l_tr), 0);
            assert_eq!(_stricmp_l(b"I\0".as_ptr(), b"\xFD\0".as_ptr(), l_tr), 0);
            // Global "C" still folds ASCII I/i normally.
            assert_eq!(toupper(b'i' as i32), b'I' as i32);
            assert_eq!(towupper(b'i' as u16), b'I' as u16);
            assert_eq!(_wcsicmp(wi_up.as_ptr(), wi_lo.as_ptr()), 0);
            // Global tr-TR applies to towupper/towlower/_wcsicmp/_wcsupr/_wcslwr.
            assert!(!crate::ucrt::locale::setlocale(2 /* LC_CTYPE */, b"tr-TR\0".as_ptr()).is_null());
            assert_eq!(towupper(b'i' as u16), 0x0130);
            assert_eq!(towlower(b'I' as u16), 0x0131);
            assert_ne!(_wcsicmp(wi_up.as_ptr(), wi_lo.as_ptr()), 0);
            assert_eq!(_wcsicmp(wi_up.as_ptr(), wi_dotless.as_ptr()), 0);
            let mut wmut = [b'I' as u16, b'i' as u16, 0];
            _wcslwr(wmut.as_mut_ptr());
            assert_eq!(wmut, [0x0131, b'i' as u16, 0]);
            _wcsupr(wmut.as_mut_ptr());
            assert_eq!(wmut, [b'I' as u16, 0x0130, 0]);
            assert!(!crate::ucrt::locale::setlocale(2, b"C\0".as_ptr()).is_null());
            crate::ucrt::locale::_free_locale(l_tr);

            // LC_COLLATE = de-DE sorts 'ä' (CP1252 0xE4) before 'b' (whereas C byte order puts 0xE4 > 'b').
            assert!(!crate::ucrt::locale::setlocale(1 /* LC_COLLATE */, b"de-DE\0".as_ptr()).is_null());
            assert!(strcoll(b"\xE4\0".as_ptr(), b"b\0".as_ptr()) < 0);
            let wa: Vec<u16> = "ä\0".encode_utf16().collect();
            let wb: Vec<u16> = "b\0".encode_utf16().collect();
            assert!(wcscoll(wa.as_ptr(), wb.as_ptr()) < 0);
            let mut ka = [0u8; 64];
            let mut kb = [0u8; 64];
            let la = strxfrm(ka.as_mut_ptr(), b"\xE4\0".as_ptr(), ka.len());
            let lb = strxfrm(kb.as_mut_ptr(), b"b\0".as_ptr(), kb.len());
            assert!(la > 0 && lb > 0 && strcmp(ka.as_ptr(), kb.as_ptr()) < 0);
            // Empty source terminates destination with NUL in both strxfrm and wcsxfrm.
            ka[0] = b'X';
            assert_eq!(strxfrm(ka.as_mut_ptr(), b"\0".as_ptr(), ka.len()), 0);
            assert_eq!(ka[0], 0);
            // Unaligned (odd-addressed) destination buffer works for both non-empty and empty wcsxfrm.
            let mut raw_unaligned = [0xAAu8; 131];
            let odd_dst = raw_unaligned.as_mut_ptr().add(1) as *mut u16;
            let wlen = wcsxfrm(odd_dst, wa.as_ptr(), 64);
            assert!(wlen > 0);
            assert_eq!(std::ptr::read_unaligned(odd_dst.add(wlen)), 0);
            let wempty = [0u16];
            std::ptr::write_unaligned(odd_dst, 0x1234);
            assert_eq!(wcsxfrm(odd_dst, wempty.as_ptr(), 64), 0);
            assert_eq!(std::ptr::read_unaligned(odd_dst), 0);
            assert!(!crate::ucrt::locale::setlocale(1, b"C\0".as_ptr()).is_null());
            assert!(strcoll(b"\xE4\0".as_ptr(), b"b\0".as_ptr()) > 0);
        }
    }
    #[test]
    fn strtok_and_s_funcs() {
        unsafe {
            let mut s = *b"a,b,,c\0";
            let mut ctx = std::ptr::null_mut();
            let d = b",\0";
            let t1 = strtok_s(s.as_mut_ptr(), d.as_ptr(), &mut ctx);
            assert_eq!(crate::strings::from_ansi(t1), "a");
            let t2 = strtok_s(std::ptr::null_mut(), d.as_ptr(), &mut ctx);
            assert_eq!(crate::strings::from_ansi(t2), "b");
            let t3 = strtok_s(std::ptr::null_mut(), d.as_ptr(), &mut ctx);
            assert_eq!(crate::strings::from_ansi(t3), "c");
            assert!(strtok_s(std::ptr::null_mut(), d.as_ptr(), &mut ctx).is_null());
            let mut buf = [0u8; 4];
            assert_eq!(strncpy_s(buf.as_mut_ptr(), 4, b"hello\0".as_ptr(), TRUNCATE), STRUNCATE);
            assert_eq!(&buf, b"hel\0");

            // memcpy_s tolerates overlapping buffers and _wcsdup tolerates odd-aligned wide strings.
            let mut ov = *b"abcdefgh";
            assert_eq!(memcpy_s(ov.as_mut_ptr().add(2).cast(), 6, ov.as_ptr().add(1).cast(), 4), 0);
            assert_eq!(&ov, b"abbcdegh");
            let mut raw_w = [0u8; 11];
            for (i, u) in [b'H' as u16, b'i' as u16, 0u16].iter().enumerate() {
                raw_w[1 + i * 2..1 + (i + 1) * 2].copy_from_slice(&u.to_le_bytes());
            }
            let odd_ws = raw_w.as_ptr().add(1) as *const u16;
            assert!(!odd_ws.is_aligned());
            let dup = _wcsdup(odd_ws);
            assert!(!dup.is_null());
            assert_eq!(crate::strings::from_wide(dup), "Hi");
            crate::ucrt::heap::free(dup.cast());

            // UCRT memory and search exports resolve via api-ms-win-crt-string/private and tolerate overlap/unaligned inputs.
            for (dll, sym) in [
                ("api-ms-win-crt-string-l1-1-0.dll", "memset"),
                ("api-ms-win-crt-string-l1-1-0.dll", "memcpy"),
                ("api-ms-win-crt-string-l1-1-0.dll", "memmove"),
                ("api-ms-win-crt-string-l1-1-0.dll", "memcmp"),
                ("api-ms-win-crt-string-l1-1-0.dll", "memchr"),
                ("api-ms-win-crt-string-l1-1-0.dll", "strchr"),
                ("api-ms-win-crt-string-l1-1-0.dll", "strrchr"),
                ("api-ms-win-crt-string-l1-1-0.dll", "strstr"),
                ("api-ms-win-crt-string-l1-1-0.dll", "wcschr"),
                ("api-ms-win-crt-string-l1-1-0.dll", "wcsrchr"),
                ("api-ms-win-crt-private-l1-1-0.dll", "memset"),
                ("api-ms-win-crt-private-l1-1-0.dll", "memcpy"),
            ] {
                assert!(crate::lookup(dll, sym).is_some(), "missing {dll}!{sym}");
            }
            let mut m = *b"abcdefgh";
            assert_eq!(memset(m.as_mut_ptr().add(1).cast(), b'Z' as i32, 3), m.as_mut_ptr().add(1).cast());
            assert_eq!(&m, b"aZZZefgh");
            assert_eq!(memcpy(m.as_mut_ptr().add(2).cast(), m.as_ptr().cast(), 4), m.as_mut_ptr().add(2).cast());
            assert_eq!(&m, b"aZaZZZgh");
            assert_eq!(memmove(m.as_mut_ptr().cast(), m.as_ptr().add(2).cast(), 4), m.as_mut_ptr().cast());
            assert_eq!(&m, b"aZZZZZgh");
            assert_eq!(memcmp(m.as_ptr().cast(), b"aZZZ".as_ptr().cast(), 4), 0);
            assert_eq!(memchr(m.as_ptr().cast(), b'g' as i32, m.len()), m.as_mut_ptr().add(6).cast());
            let hay = b"abracadabra\0";
            assert_eq!(strchr(hay.as_ptr().cast(), b'r' as i32), hay.as_ptr().add(2) as *mut std::ffi::c_char);
            assert_eq!(strrchr(hay.as_ptr().cast(), b'r' as i32), hay.as_ptr().add(9) as *mut std::ffi::c_char);
            assert_eq!(strstr(hay.as_ptr().cast(), b"cada\0".as_ptr().cast()), hay.as_ptr().add(4) as *mut std::ffi::c_char);
            assert_eq!(wcschr(odd_ws, b'i' as u16), odd_ws.add(1) as *mut u16);
            assert_eq!(wcsrchr(odd_ws, b'H' as u16), odd_ws as *mut u16);
        }
    }

    #[test]
    fn mbstrlen_and_mbscmp_honor_locale_and_dbcs() {
        unsafe extern "win64" fn noop_iph(
            _: *const u16,
            _: *const u16,
            _: *const u16,
            _: u32,
            _: usize,
        ) {}
        struct IphGuard(usize);
        impl Drop for IphGuard {
            fn drop(&mut self) {
                unsafe {
                    crate::ucrt::runtime::_set_invalid_parameter_handler(self.0);
                }
            }
        }

        unsafe {
            let _iph = IphGuard(crate::ucrt::runtime::_set_invalid_parameter_handler(
                noop_iph as *const () as usize,
            ));

            // UTF-8 locale: _mbstrlen_l counts scalar values and rejects invalid UTF-8 with EILSEQ.
            let loc_u8 = crate::ucrt::locale::_create_locale(2 /* LC_CTYPE */, b"en-US.utf8\0".as_ptr());
            assert!(!loc_u8.is_null());
            assert_eq!(_mbstrlen_l("aäあ😀\0".as_ptr(), loc_u8), 4);
            errno::set(0);
            assert_eq!(_mbstrlen_l(b"\xFF\0".as_ptr(), loc_u8), usize::MAX);
            assert_eq!(errno::get(), super::super::errno::EILSEQ);
            crate::ucrt::locale::_free_locale(loc_u8);

            // DBCS CP932 (ja-JP): 2-byte "あ" (0x82 0xA0) is 1 char and sorts > 1-byte 0xB1 ('ｱ').
            let loc_ja = crate::ucrt::locale::_create_locale(2 /* LC_CTYPE */, b"ja-JP\0".as_ptr());
            assert!(!loc_ja.is_null());
            let ja_str: [u8; 4] = [0x82, 0xA0, b'x', 0];
            assert_eq!(_mbstrlen_l(ja_str.as_ptr(), loc_ja), 2);
            let dangling: [u8; 2] = [0x82, 0];
            errno::set(0);
            assert_eq!(_mbstrlen_l(dangling.as_ptr(), loc_ja), usize::MAX);
            assert_eq!(errno::get(), super::super::errno::EILSEQ);

            let prev_mbcp = crate::ucrt::locale::_getmbcp();
            assert_eq!(crate::ucrt::locale::_setmbcp(932), 0);
            let dbcs_a: [u8; 3] = [0x82, 0xA0, 0];
            let sbcs_kana: [u8; 2] = [0xB1, 0];
            // 0x82A0 > 0x00B1 in DBCS _mbscmp (opposite of raw unsigned byte strcmp!).
            assert_eq!(_mbscmp(dbcs_a.as_ptr(), sbcs_kana.as_ptr()), 1);
            assert_eq!(_mbscmp(sbcs_kana.as_ptr(), dbcs_a.as_ptr()), -1);
            assert_eq!(_mbscmp(dbcs_a.as_ptr(), dbcs_a.as_ptr()), 0);
            // Dangling lead byte before NUL compares equal to NUL (matching MSVC mbscmp).
            assert_eq!(_mbscmp(dangling.as_ptr(), b"\0".as_ptr()), 0);
            assert_eq!(crate::ucrt::locale::_setmbcp(prev_mbcp), 0);
            crate::ucrt::locale::_free_locale(loc_ja);

            // NULL pointers return 0 / i32::MAX (_NLSCMPERROR) and set EINVAL.
            errno::set(0);
            assert_eq!(_mbstrlen(std::ptr::null()), 0);
            assert_eq!(errno::get(), EINVAL);
            errno::set(0);
            assert_eq!(_mbscmp(std::ptr::null(), b"a\0".as_ptr()), i32::MAX);
            assert_eq!(errno::get(), EINVAL);
        }
    }
}
