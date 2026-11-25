//! `<string.h>`, `<wchar.h>` string functions and `<ctype.h>`/`<wctype.h>`.
//!
//! Classification and case mapping follow the current CRT locale: in the
//! "C" locale only ASCII is classified/folded (like ucrt); after
//! `setlocale(LC_ALL, "")` Unicode rules apply (see [`super::locale`]).

use super::errno::{self, EINVAL, ERANGE, STRUNCATE};
use super::locale::c_locale_active;
use super::runtime::invalid_parameter;
use crate::heap;
use crate::strings::{wlower, wupper};
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

/// Build and cache the 257-entry MSVC `__pctype_func` classification table (`index 0 = EOF`)
/// for `cp`. `"C"` (`0`) and UTF-8 (`65001`) classify only ASCII bytes (`0x00..=0x7F`);
/// SBCS/DBCS code pages classify `0x80..=0xFF` via `kernel32::string` (marking DBCS lead bytes
/// with `_LEADBYTE` = `0x8000`).
fn ctype_table_for_cp(cp: u32) -> &'static [u16; 257] {
    if cp == 0 || cp == 65001 {
        return &CTYPE;
    }
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    static TABLES: OnceLock<Mutex<HashMap<u32, &'static [u16; 257]>>> = OnceLock::new();
    let m = TABLES.get_or_init(|| Mutex::new(HashMap::new()));
    let mut g = m.lock().unwrap();
    if let Some(&tbl) = g.get(&cp) {
        return tbl;
    }
    let mut tbl = CTYPE;
    for b in 0x80u8..=0xFF {
        let is_lead = unsafe { crate::kernel32::string::IsDBCSLeadByteEx(cp, b) != 0 };
        if is_lead {
            tbl[b as usize + 1] = _LEADBYTE;
            continue;
        }
        if let Ok(units) = crate::kernel32::string::decode_cp_to_utf16_strict(cp, &[b], true) {
            if units.len() == 1 {
                tbl[b as usize + 1] = crate::kernel32::string::ctype1(units[0]) & 0x1FF;
            }
        }
    }
    let leaked: &'static [u16; 257] = Box::leak(Box::new(tbl));
    g.insert(cp, leaked);
    leaked
}

pub unsafe extern "win64" fn __pctype_func() -> *const u16 {
    ctype_table_for_cp(super::locale::mb_cp()).as_ptr().add(1)
}
pub unsafe extern "win64" fn __pwctype_func() -> *const u16 {
    CTYPE.as_ptr().add(1)
}

/// Classification of an `int` argument as ucrt does it (EOF and 0..=255) for `cp`.
fn cls_cp(c: i32, cp: u32) -> u16 {
    if (-1..=255).contains(&c) {
        ctype_table_for_cp(cp)[(c + 1) as usize]
    } else {
        0
    }
}

/// Classification of an `int` argument as ucrt does it (EOF and 0..=255).
fn cls(c: i32) -> u16 {
    cls_cp(c, super::locale::mb_cp())
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

fn map_narrow_case(c: i32, cp: u32, to_upper: bool) -> i32 {
    if to_upper {
        if (b'a' as i32..=b'z' as i32).contains(&c) {
            return c - 32;
        }
    } else if (b'A' as i32..=b'Z' as i32).contains(&c) {
        return c + 32;
    }
    if !(0x80..=0xFF).contains(&c) || cp == 0 || cp == 65001 {
        return c;
    }
    let b = c as u8;
    if unsafe { crate::kernel32::string::IsDBCSLeadByteEx(cp, b) != 0 } {
        return c;
    }
    if let Ok(units) = crate::kernel32::string::decode_cp_to_utf16_strict(cp, &[b], true) {
        if units.len() == 1 {
            let mapped = if to_upper { wupper(units[0]) } else { wlower(units[0]) };
            if mapped != units[0] {
                if let Ok((out, used)) = crate::kernel32::string::encode_utf16_to_cp(cp, &[mapped], None) {
                    if !used && out.len() == 1 {
                        return out[0] as i32;
                    }
                }
            }
        }
    }
    c
}

#[inline(never)]
pub unsafe extern "win64" fn tolower(c: i32) -> i32 {
    map_narrow_case(c, super::locale::mb_cp(), false)
}
#[inline(never)]
pub unsafe extern "win64" fn toupper(c: i32) -> i32 {
    map_narrow_case(c, super::locale::mb_cp(), true)
}
pub unsafe extern "win64" fn _tolower(c: i32) -> i32 {
    c + 32
}
pub unsafe extern "win64" fn _toupper(c: i32) -> i32 {
    c - 32
}
pub unsafe extern "win64" fn _tolower_l(c: i32, l: *mut c_void) -> i32 {
    map_narrow_case(c, super::locale::locale_obj_cp(l), false)
}
pub unsafe extern "win64" fn _toupper_l(c: i32, l: *mut c_void) -> i32 {
    map_narrow_case(c, super::locale::locale_obj_cp(l), true)
}

// Wide classification
fn wch(c: u16) -> Option<char> {
    char::from_u32(c as u32)
}

fn wlow(c: u16) -> u16 {
    if c_locale_active() {
        if (b'A' as u16..=b'Z' as u16).contains(&c) {
            c + 32
        } else {
            c
        }
    } else {
        wlower(c)
    }
}
fn wup(c: u16) -> u16 {
    if c_locale_active() {
        if (b'a' as u16..=b'z' as u16).contains(&c) {
            c - 32
        } else {
            c
        }
    } else {
        wupper(c)
    }
}

pub unsafe extern "win64" fn towlower(c: u16) -> u16 {
    wlow(c)
}
pub unsafe extern "win64" fn towupper(c: u16) -> u16 {
    wup(c)
}
pub unsafe extern "win64" fn _towlower_l(c: u16, _l: *mut c_void) -> u16 {
    wlow(c)
}
pub unsafe extern "win64" fn _towupper_l(c: u16, _l: *mut c_void) -> u16 {
    wup(c)
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
pub unsafe extern "win64" fn _stricmp(a: *const u8, b: *const u8) -> i32 {
    _strnicmp(a, b, usize::MAX)
}
pub unsafe extern "win64" fn _strnicmp(a: *const u8, b: *const u8, n: usize) -> i32 {
    if a.is_null() || b.is_null() {
        errno::set(EINVAL);
        invalid_parameter("_strnicmp");
        return i32::MAX;
    }
    for i in 0..n {
        let (x, y) = ((*a.add(i)).to_ascii_lowercase(), (*b.add(i)).to_ascii_lowercase());
        if x != y {
            return x as i32 - y as i32;
        }
        if x == 0 {
            return 0;
        }
    }
    0
}
pub unsafe extern "win64" fn _stricmp_l(a: *const u8, b: *const u8, _l: *mut c_void) -> i32 {
    _stricmp(a, b)
}
pub unsafe extern "win64" fn _strnicmp_l(a: *const u8, b: *const u8, n: usize, _l: *mut c_void) -> i32 {
    _strnicmp(a, b, n)
}
pub unsafe extern "win64" fn _strcmpi(a: *const u8, b: *const u8) -> i32 {
    _stricmp(a, b)
}
pub unsafe extern "win64" fn strcoll(a: *const u8, b: *const u8) -> i32 {
    strcmp(a, b)
}
pub unsafe extern "win64" fn _stricoll(a: *const u8, b: *const u8) -> i32 {
    _stricmp(a, b)
}
pub unsafe extern "win64" fn _strncoll(a: *const u8, b: *const u8, n: usize) -> i32 {
    strncmp(a, b, n)
}
pub unsafe extern "win64" fn _strnicoll(a: *const u8, b: *const u8, n: usize) -> i32 {
    _strnicmp(a, b, n)
}
pub unsafe extern "win64" fn strxfrm(d: *mut u8, s: *const u8, n: usize) -> usize {
    let len = strlen(s);
    if len < n && !d.is_null() {
        std::ptr::copy_nonoverlapping(s, d, len + 1);
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
    let mut p = s;
    while *p != 0 {
        *p = (*p).to_ascii_lowercase();
        p = p.add(1);
    }
    s
}
pub unsafe extern "win64" fn _strupr(s: *mut u8) -> *mut u8 {
    let mut p = s;
    while *p != 0 {
        *p = (*p).to_ascii_uppercase();
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
    std::ptr::copy_nonoverlapping(s as *const u8, d as *mut u8, n);
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
    while i < n && *s.add(i) != 0 {
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
pub unsafe extern "win64" fn _wcsicmp(a: *const u16, b: *const u16) -> i32 {
    _wcsnicmp(a, b, usize::MAX)
}
pub unsafe extern "win64" fn _wcsnicmp(a: *const u16, b: *const u16, n: usize) -> i32 {
    if a.is_null() || b.is_null() {
        errno::set(EINVAL);
        invalid_parameter("_wcsnicmp");
        return i32::MAX;
    }
    for i in 0..n {
        let (x, y) = (wlow(*a.add(i)), wlow(*b.add(i)));
        if x != y {
            return x as i32 - y as i32;
        }
        if x == 0 {
            return 0;
        }
    }
    0
}
pub unsafe extern "win64" fn _wcsicmp_l(a: *const u16, b: *const u16, _l: *mut c_void) -> i32 {
    _wcsicmp(a, b)
}
pub unsafe extern "win64" fn _wcsnicmp_l(a: *const u16, b: *const u16, n: usize, _l: *mut c_void) -> i32 {
    _wcsnicmp(a, b, n)
}
pub unsafe extern "win64" fn wcscoll(a: *const u16, b: *const u16) -> i32 {
    if c_locale_active() {
        return wcscmp(a, b);
    }
    crate::kernel32::string::CompareStringW(0x400, 0x1000, a, -1, b, -1) - 2
}
pub unsafe extern "win64" fn _wcsicoll(a: *const u16, b: *const u16) -> i32 {
    _wcsicmp(a, b)
}
pub unsafe extern "win64" fn _wcsncoll(a: *const u16, b: *const u16, n: usize) -> i32 {
    wcsncmp(a, b, n)
}
pub unsafe extern "win64" fn _wcsnicoll(a: *const u16, b: *const u16, n: usize) -> i32 {
    _wcsnicmp(a, b, n)
}
pub unsafe extern "win64" fn wcsxfrm(d: *mut u16, s: *const u16, n: usize) -> usize {
    let len = wcslen(s);
    if len < n && !d.is_null() {
        std::ptr::copy_nonoverlapping(s, d, len + 1);
    }
    len
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
        std::ptr::copy_nonoverlapping(s, p, n);
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
    let mut p = s;
    while *p != 0 {
        *p = wlow(*p);
        p = p.add(1);
    }
    s
}
pub unsafe extern "win64" fn _wcsupr(s: *mut u16) -> *mut u16 {
    let mut p = s;
    while *p != 0 {
        *p = wup(*p);
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
            // UTF-8 locale leaves bytes >= 0x80 unclassified.
            assert!(!crate::ucrt::locale::setlocale(0, b"en-US.utf8\0".as_ptr()).is_null());
            assert_eq!(isalpha(0xE4), 0);
            assert_eq!(isprint(0xE4), 0);
            // Restore "C" locale for subsequent serial tests.
            assert!(!crate::ucrt::locale::setlocale(0, b"C\0".as_ptr()).is_null());
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
        }
    }
}
