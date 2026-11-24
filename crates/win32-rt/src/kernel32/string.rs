//! Code pages, string comparison/mapping and locale information.
//!
//! The process locale is fixed to **en-US** (LCID 0x409) and the ANSI/OEM
//! code page to UTF-8 (65001, like Windows' "Use Unicode UTF-8 for worldwide
//! language support" option). Linguistic comparison is a simplified model of
//! the Windows sort (case-insensitive primary order, lowercase-before-uppercase
//! tie-break); it is not a full NLS/ICU collation.

use crate::strings::*;
use crate::teb;
use crate::types::*;
use std::cmp::Ordering;
use std::ffi::c_void;

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

pub fn resolve_cp(cp: u32) -> u32 {
    match cp {
        CP_ACP | CP_OEMCP | CP_MACCP | CP_THREAD_ACP => ACP,
        c => c,
    }
}

pub fn valid_cp(cp: u32) -> bool {
    matches!(resolve_cp(cp), 65001 | 1252 | 437 | 850 | 28591 | 20127 | 1200 | 1201)
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
pub unsafe extern "win64" fn IsDBCSLeadByteEx(_cp: u32, _b: u8) -> BOOL {
    FALSE
}

fn max_char_size(cp: u32) -> u32 {
    match resolve_cp(cp) {
        65001 => 4,
        65000 => 5,
        _ => 1,
    }
}

pub unsafe extern "win64" fn GetCPInfo(cp: u32, info: *mut u8) -> BOOL {
    if !valid_cp(cp) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    std::ptr::write_bytes(info, 0, 18);
    (info as *mut u32).write_unaligned(max_char_size(cp));
    *info.add(4) = b'?';
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
        1252 => "1252  (ANSI - Latin I)",
        437 => "437   (OEM - United States)",
        _ => "",
    };
    write_wide_trunc(name, info.add(24) as *mut u16, 260);
    TRUE
}

// ---------------------------------------------------------------------------
// MultiByteToWideChar / WideCharToMultiByte
// ---------------------------------------------------------------------------

pub unsafe fn mb_to_wide(cp: u32, flags: u32, src: &[u8]) -> Result<Vec<u16>, u32> {
    let cp = resolve_cp(cp);
    let s = match cp {
        65001 | 1200 => match std::str::from_utf8(src) {
            Ok(s) => std::borrow::Cow::Borrowed(s),
            Err(_) if flags & MB_ERR_INVALID_CHARS != 0 => return Err(err::ERROR_NO_UNICODE_TRANSLATION),
            Err(_) => String::from_utf8_lossy(src),
        },
        c if valid_cp(c) => std::borrow::Cow::Owned(decode_cp(c, src).map_err(|_| err::ERROR_NO_UNICODE_TRANSLATION)?),
        _ => return Err(err::ERROR_INVALID_PARAMETER),
    };
    Ok(s.encode_utf16().collect())
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
        c if valid_cp(c) => {
            let s = String::from_utf16_lossy(src);
            let mut used = false;
            let enc: Vec<u8> = s
                .chars()
                .map(|ch| {
                    let e = encode_cp(c, ch.encode_utf8(&mut [0u8; 4]));
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
    _locale: LPCWSTR,
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
    let _ = NORM_LINGUISTIC_CASING;
    ord_to_cstr(compare_linguistic(wslice(a, na), wslice(b, nb), flags))
}

pub unsafe extern "win64" fn CompareStringW(_lcid: u32, flags: u32, a: LPCWSTR, na: i32, b: LPCWSTR, nb: i32) -> i32 {
    CompareStringEx(std::ptr::null(), flags, a, na, b, nb, std::ptr::null_mut(), std::ptr::null_mut(), 0)
}

pub unsafe extern "win64" fn CompareStringA(lcid: u32, flags: u32, a: LPCSTR, na: i32, b: LPCSTR, nb: i32) -> i32 {
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

const LCMAP_LOWERCASE: u32 = 0x100;
const LCMAP_UPPERCASE: u32 = 0x200;
const LCMAP_TITLECASE: u32 = 0x300;
const LCMAP_SORTKEY: u32 = 0x400;
const LCMAP_BYTEREV: u32 = 0x800;
const LCMAP_HASH: u32 = 0x40000;

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

pub unsafe extern "win64" fn LCMapStringEx(
    _locale: LPCWSTR,
    flags: u32,
    src: LPCWSTR,
    n: i32,
    dst: LPWSTR,
    cap: i32,
    _ver: *mut c_void,
    _res: *mut c_void,
    _sort_handle: isize,
) -> i32 {
    if src.is_null() || n == 0 || cap < 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let with_nul = n < 0;
    let s = if n < 0 { wide_slice(src) } else { std::slice::from_raw_parts(src, n as usize) };
    if flags & LCMAP_SORTKEY != 0 {
        let k = sort_key(s, flags);
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
    if flags & LCMAP_HASH != 0 {
        stub!("LCMAP_HASH unsupported");
        set_err(err::ERROR_INVALID_FLAGS);
        return 0;
    }
    let mut out: Vec<u16> = match flags & 0x300 {
        LCMAP_LOWERCASE => s.iter().map(|&c| wlower(c)).collect(),
        LCMAP_UPPERCASE => s.iter().map(|&c| wupper(c)).collect(),
        LCMAP_TITLECASE => {
            let mut prev_alpha = false;
            s.iter()
                .map(|&c| {
                    let a = char::from_u32(c as u32).is_some_and(|ch| ch.is_alphabetic());
                    let r = if a && !prev_alpha { wupper(c) } else { wlower(c) };
                    prev_alpha = a;
                    r
                })
                .collect()
        }
        _ => s.to_vec(),
    };
    if flags & LCMAP_BYTEREV != 0 {
        for c in out.iter_mut() {
            *c = c.swap_bytes();
        }
    }
    if flags & !(0x300 | LCMAP_BYTEREV | 0x0100_0000 /* LINGUISTIC_CASING */ | 0x1 | 0x2 | 0x4 | 0x10000 | 0x20000) != 0 {
        stub!("LCMapString flags {flags:#x} partially honoured");
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
    std::ptr::copy(out.as_ptr(), dst, out.len());
    out.len() as i32
}

pub unsafe extern "win64" fn LCMapStringW(_lcid: u32, flags: u32, src: LPCWSTR, n: i32, dst: LPWSTR, cap: i32) -> i32 {
    LCMapStringEx(std::ptr::null(), flags, src, n, dst, cap, std::ptr::null_mut(), std::ptr::null_mut(), 0)
}

pub unsafe extern "win64" fn LCMapStringA(lcid: u32, flags: u32, src: LPCSTR, n: i32, dst: LPSTR, cap: i32) -> i32 {
    let bytes = if n < 0 { std::slice::from_raw_parts(src, strlen(src) + 1) } else { std::slice::from_raw_parts(src, n as usize) };
    let w: Vec<u16> = String::from_utf8_lossy(bytes).encode_utf16().collect();
    if flags & LCMAP_SORTKEY != 0 {
        return LCMapStringW(lcid, flags, w.as_ptr(), w.len() as i32, dst as LPWSTR, cap);
    }
    let need = LCMapStringW(lcid, flags, w.as_ptr(), w.len() as i32, std::ptr::null_mut(), 0);
    if need <= 0 {
        return 0;
    }
    let mut out = vec![0u16; need as usize];
    LCMapStringW(lcid, flags, w.as_ptr(), w.len() as i32, out.as_mut_ptr(), need);
    let b = String::from_utf16_lossy(&out).into_bytes();
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
// Locale
// ---------------------------------------------------------------------------

pub const LOCALE_USER_DEFAULT: u32 = 0x400;
pub const LOCALE_SYSTEM_DEFAULT: u32 = 0x800;
pub const LOCALE_INVARIANT: u32 = 0x7F;

/// en-US locale data for `GetLocaleInfo*` (`None` = unknown LCTYPE).
pub fn locale_info(lctype: u32, invariant: bool) -> Option<std::borrow::Cow<'static, str>> {
    use std::borrow::Cow::Borrowed as B;
    const DAYS: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    const MONTHS: [&str; 12] =
        ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let t = lctype & 0x0FFF_FFFF & !0x0800_0000;
    Some(match t {
        0x01 => B(if invariant { "007f" } else { "0409" }),
        0x02 | 0x72 => B(if invariant { "Invariant Language (Invariant Country)" } else { "English (United States)" }),
        0x03 => B(if invariant { "IVL" } else { "ENU" }),
        0x04 | 0x1001 => B(if invariant { "Invariant Language" } else { "English" }),
        0x05 | 0x0A => B("1"),
        0x06 | 0x08 | 0x1002 => B(if invariant { "Invariant Country" } else { "United States" }),
        0x07 => B(if invariant { "IVC" } else { "USA" }),
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
        0x1B => B("0"),
        0x1C => B("0"),
        0x1D => B("/"),
        0x1E => B(":"),
        0x1F => B(if invariant { "MM/dd/yyyy" } else { "M/d/yyyy" }),
        0x20 => B(if invariant { "dddd, dd MMMM yyyy" } else { "dddd, MMMM d, yyyy" }),
        0x21 => B("0"),
        0x22 => B("0"),
        0x23 => B("0"),
        0x24 => B("0"),
        0x25 => B("0"),
        0x26 => B("1"),
        0x27 => B("0"),
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
        0x54 | 0x55 | 0x56 | 0x57 => B("1"),
        0x59 => B(if invariant { "iv" } else { "en" }),
        0x5A => B(if invariant { "IV" } else { "US" }),
        0x5B => B("244"),
        0x5C | 0x6E => B(if invariant { "" } else { LOCALE_NAME }),
        0x67 => B("en"),
        0x69 => B("NaN"),
        0x6A => B("Infinity"),
        0x6B => B("-Infinity"),
        0x6C => B("Latn;"),
        0x6D => B(if invariant { "" } else { "en" }),
        0x70 => B("0"),
        0x73 | 0x74 => B("English"),
        0x75 => B("United States"),
        0x76 => B("%"),
        0x77 => B("\u{2030}"),
        0x78 => B("0"),
        0x79 => B("h:mm tt"),
        0x7A => B(if invariant { "" } else { "en-US" }),
        0x7B => B("1"),
        0x1003 => B("h:mm:ss tt"),
        0x1004 => B("1252"),
        0x1005 => B("0"),
        0x1006 => B("MMMM yyyy"),
        0x1007 => B("United States"),
        0x1008 => B("English (United States)"),
        0x1009 => B("1"),
        0x100A => B("1"),
        0x100C => B("6"),
        0x100D => B("0"),
        0x1010 => B("1"),
        0x1011 => B("10000"),
        0x1012 => B("037"),
        0x1014 => B("1"),
        0x1015..=0x1018 => B(""),
        _ => return None,
    })
}

const LOCALE_RETURN_NUMBER: u32 = 0x2000_0000;

unsafe fn get_locale_info(invariant: bool, lctype: u32, out: *mut u16, cap: i32) -> i32 {
    let Some(v) = locale_info(lctype, invariant) else {
        stub!("GetLocaleInfo LCTYPE {lctype:#x} unknown");
        set_err(err::ERROR_INVALID_FLAGS);
        return 0;
    };
    if lctype & LOCALE_RETURN_NUMBER != 0 {
        let n: u32 = if (lctype & 0xFFFF) == 0x01 { u32::from_str_radix(&v, 16).unwrap_or(0) } else { v.parse().unwrap_or(0) };
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
    std::ptr::copy_nonoverlapping(w.as_ptr(), out, w.len());
    w.len() as i32
}

fn lcid_supported(lcid: u32) -> Option<bool> {
    match lcid {
        0 | LOCALE_USER_DEFAULT | LOCALE_SYSTEM_DEFAULT | LCID_EN_US | 0x09 => Some(false),
        LOCALE_INVARIANT => Some(true),
        _ => None,
    }
}

/// `Some(invariant?)` for locale names we serve.
pub unsafe fn locale_name_supported(name: LPCWSTR) -> Option<bool> {
    if name.is_null() {
        return Some(false);
    }
    let n = from_wide(name);
    match n.to_ascii_lowercase().as_str() {
        "" => Some(true),
        "en-us" | "en" | "!x-sys-default-locale" => Some(false),
        _ => None,
    }
}

pub unsafe extern "win64" fn GetLocaleInfoW(lcid: u32, lctype: u32, out: *mut u16, cap: i32) -> i32 {
    let inv = match lcid_supported(lcid) {
        Some(i) => i,
        None => {
            stub!("locale {lcid:#x} not available; answering with en-US data");
            false
        }
    };
    get_locale_info(inv, lctype, out, cap)
}

pub unsafe extern "win64" fn GetLocaleInfoEx(name: LPCWSTR, lctype: u32, out: *mut u16, cap: i32) -> i32 {
    let inv = match locale_name_supported(name) {
        Some(i) => i,
        None => {
            set_err(err::ERROR_INVALID_PARAMETER);
            return 0;
        }
    };
    get_locale_info(inv, lctype, out, cap)
}

pub unsafe extern "win64" fn GetLocaleInfoA(lcid: u32, lctype: u32, out: *mut u8, cap: i32) -> i32 {
    let inv = lcid_supported(lcid).unwrap_or(false);
    let Some(v) = locale_info(lctype, inv) else {
        set_err(err::ERROR_INVALID_FLAGS);
        return 0;
    };
    if lctype & LOCALE_RETURN_NUMBER != 0 {
        let mut tmp = [0u16; 2];
        let r = get_locale_info(inv, lctype, tmp.as_mut_ptr(), 2);
        if cap >= 4 {
            std::ptr::copy_nonoverlapping(tmp.as_ptr() as *const u8, out, 4);
        }
        return r * 2;
    }
    let need = v.len() as i32 + 1;
    if cap == 0 {
        return need;
    }
    if need > cap {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    write_ansi_buf(&v, out, cap as usize);
    need
}

pub unsafe extern "win64" fn GetUserDefaultLCID() -> u32 {
    LCID_EN_US
}
pub unsafe extern "win64" fn GetSystemDefaultLCID() -> u32 {
    LCID_EN_US
}
pub unsafe extern "win64" fn GetThreadLocale() -> u32 {
    LCID_EN_US
}
pub unsafe extern "win64" fn SetThreadLocale(_l: u32) -> BOOL {
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
    if cap == 0 && out.is_null() {
        return w.len() as i32;
    }
    if (cap as usize) < w.len() {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), out, w.len());
    w.len() as i32
}

pub unsafe extern "win64" fn GetUserDefaultLocaleName(out: *mut u16, cap: i32) -> i32 {
    put_name(out, cap, LOCALE_NAME)
}
pub unsafe extern "win64" fn GetSystemDefaultLocaleName(out: *mut u16, cap: i32) -> i32 {
    put_name(out, cap, LOCALE_NAME)
}

pub unsafe extern "win64" fn LCIDToLocaleName(lcid: u32, out: *mut u16, cap: i32, _flags: u32) -> i32 {
    match lcid_supported(lcid) {
        Some(true) => put_name(out, cap, ""),
        Some(false) => put_name(out, cap, LOCALE_NAME),
        None => {
            set_err(err::ERROR_INVALID_PARAMETER);
            0
        }
    }
}

pub unsafe extern "win64" fn LocaleNameToLCID(name: LPCWSTR, _flags: u32) -> u32 {
    match locale_name_supported(name) {
        Some(true) => LOCALE_INVARIANT,
        Some(false) => LCID_EN_US,
        None => {
            set_err(err::ERROR_INVALID_PARAMETER);
            0
        }
    }
}

pub unsafe extern "win64" fn ResolveLocaleName(name: LPCWSTR, out: *mut u16, cap: i32) -> i32 {
    match locale_name_supported(name) {
        Some(true) => put_name(out, cap, ""),
        Some(false) => put_name(out, cap, LOCALE_NAME),
        None => put_name(out, cap, LOCALE_NAME),
    }
}

pub unsafe extern "win64" fn IsValidLocale(lcid: u32, _flags: u32) -> BOOL {
    lcid_supported(lcid).is_some() as BOOL
}
pub unsafe extern "win64" fn IsValidLocaleName(name: LPCWSTR) -> BOOL {
    locale_name_supported(name).is_some() as BOOL
}

type LocaleEnumProcW = unsafe extern "win64" fn(*mut u16) -> BOOL;
type LocaleEnumProcEx = unsafe extern "win64" fn(*mut u16, u32, isize) -> BOOL;

pub unsafe extern "win64" fn EnumSystemLocalesW(f: Option<LocaleEnumProcW>, _flags: u32) -> BOOL {
    if let Some(f) = f {
        let mut s = to_wide_z("00000409");
        crate::call_guest8(f as usize, s.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0);
    }
    TRUE
}
pub unsafe extern "win64" fn EnumSystemLocalesA(f: Option<unsafe extern "win64" fn(*mut u8) -> BOOL>, _flags: u32) -> BOOL {
    if let Some(f) = f {
        let mut s = *b"00000409\0";
        crate::call_guest8(f as usize, s.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0);
    }
    TRUE
}
pub unsafe extern "win64" fn EnumSystemLocalesEx(f: Option<LocaleEnumProcEx>, _flags: u32, lparam: isize, _r: *mut c_void) -> BOOL {
    if let Some(f) = f {
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

pub unsafe extern "win64" fn GetUserGeoID(_class: u32) -> i32 {
    244 // United States
}

pub unsafe extern "win64" fn GetGeoInfoW(geo: i32, ty: u32, out: *mut u16, cap: i32, _lang: u16) -> i32 {
    if geo != 244 {
        stub!("GetGeoInfo for geo id {geo} answered with US data");
    }
    let v = match ty {
        1 => "244",                                   // GEO_NATION
        2 => "39.45",                                 // GEO_LATITUDE
        3 => "-98.908",                               // GEO_LONGITUDE
        4 => "US",                                    // GEO_ISO2
        5 => "USA",                                   // GEO_ISO3
        6 => "1",                                     // GEO_RFC1766? (unused)
        7 => "1033",                                  // GEO_LCID
        8 => "United States",                         // GEO_FRIENDLYNAME
        9 => "United States of America",              // GEO_OFFICIALNAME
        10 | 11 | 12 => "",                           // TIMEZONES/OFFICIALLANGUAGES/ISO_UN_NUMBER
        13 => "",                                     // GEO_PARENT
        14 => "US",                                   // GEO_DIALINGCODE? best effort
        15 => "USD",                                  // GEO_CURRENCYCODE
        16 => "$",                                    // GEO_CURRENCYSYMBOL
        17 => "US",                                   // GEO_NAME
        _ => {
            set_err(err::ERROR_INVALID_FLAGS);
            return 0;
        }
    };
    put_name(out, cap, v)
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

pub unsafe extern "win64" fn GetNumberFormatEx(_locale: LPCWSTR, _flags: u32, value: LPCWSTR, fmt: *const c_void, out: *mut u16, cap: i32) -> i32 {
    if !fmt.is_null() {
        stub!("custom NUMBERFMT ignored");
    }
    let v = from_wide(value);
    let neg = v.starts_with('-');
    let digits = v.trim_start_matches('-');
    let (int, frac) = digits.split_once('.').unwrap_or((digits, ""));
    let mut grouped = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }
    let frac2: String = frac.chars().chain(std::iter::repeat('0')).take(2).collect();
    let s = format!("{}{}.{}", if neg { "-" } else { "" }, if grouped.is_empty() { "0" } else { &grouped }, frac2);
    out_wide(&s, out, cap)
}

pub unsafe extern "win64" fn GetNumberFormatW(_lcid: u32, flags: u32, value: LPCWSTR, fmt: *const c_void, out: *mut u16, cap: i32) -> i32 {
    GetNumberFormatEx(std::ptr::null(), flags, value, fmt, out, cap)
}

pub unsafe extern "win64" fn GetCalendarInfoEx(_l: LPCWSTR, _cal: u32, _r: LPCWSTR, ty: u32, out: *mut u16, cap: i32, val: *mut u32) -> i32 {
    // CAL_ITWODIGITYEARMAX (0x30) with CAL_RETURN_NUMBER (0x20000000) is what the CRT asks for.
    if ty & 0xFFFF == 0x30 {
        if ty & 0x2000_0000 != 0 && !val.is_null() {
            *val = 2049;
            return 2;
        }
        return out_wide("2049", out, cap);
    }
    stub!("GetCalendarInfoEx type {ty:#x}");
    set_err(err::ERROR_INVALID_FLAGS);
    0
}

pub unsafe extern "win64" fn GetCalendarInfoW(_lcid: u32, cal: u32, ty: u32, out: *mut u16, cap: i32, val: *mut u32) -> i32 {
    GetCalendarInfoEx(std::ptr::null(), cal, std::ptr::null(), ty, out, cap, val)
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
}
