//! Number <-> string conversion (`strto*`, `ato*`, `_itoa`, ...), multibyte
//! conversion (`mbstowcs` & co., following the CRT locale code page) and
//! `<stdlib.h>` utilities (qsort, bsearch, rand).

use super::errno::{self, EILSEQ, EINVAL, ERANGE};
use super::locale::mb_cp;
use super::runtime::invalid_parameter;
use std::ffi::c_void;

// ---------------------------------------------------------------------------
// Integer parsing
// ---------------------------------------------------------------------------

fn is_space(c: u32) -> bool {
    c == 0x20 || (0x09..=0x0D).contains(&c)
}

/// Returns (magnitude, negative, units consumed (0 = no conversion), overflowed u64).
unsafe fn scan_int(get: &dyn Fn(usize) -> u32, mut base: u32) -> (u64, bool, usize, bool) {
    let mut i = 0;
    while is_space(get(i)) {
        i += 1;
    }
    let mut neg = false;
    match get(i) {
        0x2B => i += 1,
        0x2D => {
            neg = true;
            i += 1
        }
        _ => {}
    }
    let digit = |c: u32| -> Option<u32> {
        match c {
            0x30..=0x39 => Some(c - 0x30),
            0x41..=0x5A => Some(c - 0x41 + 10),
            0x61..=0x7A => Some(c - 0x61 + 10),
            _ => None,
        }
    };
    if (base == 0 || base == 16) && get(i) == 0x30 && (get(i + 1) | 0x20) == 0x78 && digit(get(i + 2)).is_some_and(|d| d < 16) {
        i += 2;
        base = 16;
    } else if base == 0 {
        base = if get(i) == 0x30 { 8 } else { 10 };
    }
    if !(2..=36).contains(&base) {
        return (0, false, 0, false);
    }
    let start = i;
    let mut v: u64 = 0;
    let mut over = false;
    while let Some(d) = digit(get(i)) {
        if d >= base {
            break;
        }
        match v.checked_mul(base as u64).and_then(|x| x.checked_add(d as u64)) {
            Some(n) => v = n,
            None => over = true,
        }
        i += 1;
    }
    if i == start {
        return (0, false, 0, false);
    }
    (v, neg, i, over)
}

/// Signed conversion clamped to [min, max].
unsafe fn to_signed(get: &dyn Fn(usize) -> u32, base: u32, min: i64, max: i64) -> (i64, usize) {
    let (v, neg, n, over) = scan_int(get, base);
    if n == 0 {
        return (0, 0);
    }
    let lim = if neg { (min as i128).unsigned_abs() as u64 } else { max as u64 };
    if over || v > lim {
        errno::set(ERANGE);
        return (if neg { min } else { max }, n);
    }
    (if neg { (v as i128).wrapping_neg() as i64 } else { v as i64 }, n)
}

/// Unsigned conversion; negative inputs wrap like C.
unsafe fn to_unsigned(get: &dyn Fn(usize) -> u32, base: u32, max: u64) -> (u64, usize) {
    let (v, neg, n, over) = scan_int(get, base);
    if n == 0 {
        return (0, 0);
    }
    if over || v > max {
        errno::set(ERANGE);
        return (max, n);
    }
    (if neg { v.wrapping_neg() & max } else { v }, n)
}

fn nget(s: *const u8) -> impl Fn(usize) -> u32 {
    move |i| unsafe { *s.add(i) as u32 }
}
fn wget(s: *const u16) -> impl Fn(usize) -> u32 {
    move |i| unsafe { *s.add(i) as u32 }
}

macro_rules! strto_signed {
    ($name:ident, $wname:ident, $ty:ty) => {
        pub unsafe extern "win64" fn $name(s: *const u8, end: *mut *const u8, base: i32) -> $ty {
            if s.is_null() {
                errno::set(EINVAL);
                invalid_parameter(stringify!($name));
                return 0;
            }
            let (v, n) = to_signed(&nget(s), base as u32, <$ty>::MIN as i64, <$ty>::MAX as i64);
            if !end.is_null() {
                *end = s.add(n);
            }
            v as $ty
        }
        pub unsafe extern "win64" fn $wname(s: *const u16, end: *mut *const u16, base: i32) -> $ty {
            if s.is_null() {
                errno::set(EINVAL);
                invalid_parameter(stringify!($wname));
                return 0;
            }
            let (v, n) = to_signed(&wget(s), base as u32, <$ty>::MIN as i64, <$ty>::MAX as i64);
            if !end.is_null() {
                *end = s.add(n);
            }
            v as $ty
        }
    };
}
macro_rules! strto_unsigned {
    ($name:ident, $wname:ident, $ty:ty) => {
        pub unsafe extern "win64" fn $name(s: *const u8, end: *mut *const u8, base: i32) -> $ty {
            if s.is_null() {
                errno::set(EINVAL);
                invalid_parameter(stringify!($name));
                return 0;
            }
            let (v, n) = to_unsigned(&nget(s), base as u32, <$ty>::MAX as u64);
            if !end.is_null() {
                *end = s.add(n);
            }
            v as $ty
        }
        pub unsafe extern "win64" fn $wname(s: *const u16, end: *mut *const u16, base: i32) -> $ty {
            if s.is_null() {
                errno::set(EINVAL);
                invalid_parameter(stringify!($wname));
                return 0;
            }
            let (v, n) = to_unsigned(&wget(s), base as u32, <$ty>::MAX as u64);
            if !end.is_null() {
                *end = s.add(n);
            }
            v as $ty
        }
    };
}

// `long` is 32-bit on Windows.
strto_signed!(strtol, wcstol, i32);
strto_signed!(strtoll, wcstoll, i64);
strto_signed!(_strtoi64, _wcstoi64, i64);
strto_signed!(strtoimax, wcstoimax, i64);
strto_unsigned!(strtoul, wcstoul, u32);
strto_unsigned!(strtoull, wcstoull, u64);
strto_unsigned!(_strtoui64, _wcstoui64, u64);
strto_unsigned!(strtoumax, wcstoumax, u64);
// @exports strtol wcstol strtoll wcstoll _strtoi64 _wcstoi64 strtoimax wcstoimax
// @exports strtoul wcstoul strtoull wcstoull _strtoui64 _wcstoui64 strtoumax wcstoumax

pub unsafe extern "win64" fn _strtol_l(s: *const u8, e: *mut *const u8, b: i32, _l: *mut c_void) -> i32 {
    strtol(s, e, b)
}
pub unsafe extern "win64" fn _strtoul_l(s: *const u8, e: *mut *const u8, b: i32, _l: *mut c_void) -> u32 {
    strtoul(s, e, b)
}

pub unsafe extern "win64" fn atoi(s: *const u8) -> i32 {
    strtol(s, std::ptr::null_mut(), 10)
}
pub unsafe extern "win64" fn atol(s: *const u8) -> i32 {
    strtol(s, std::ptr::null_mut(), 10)
}
pub unsafe extern "win64" fn atoll(s: *const u8) -> i64 {
    strtoll(s, std::ptr::null_mut(), 10)
}
pub unsafe extern "win64" fn _atoi64(s: *const u8) -> i64 {
    strtoll(s, std::ptr::null_mut(), 10)
}
pub unsafe extern "win64" fn _wtoi(s: *const u16) -> i32 {
    wcstol(s, std::ptr::null_mut(), 10)
}
pub unsafe extern "win64" fn _wtol(s: *const u16) -> i32 {
    wcstol(s, std::ptr::null_mut(), 10)
}
pub unsafe extern "win64" fn _wtoll(s: *const u16) -> i64 {
    wcstoll(s, std::ptr::null_mut(), 10)
}
pub unsafe extern "win64" fn _wtoi64(s: *const u16) -> i64 {
    wcstoll(s, std::ptr::null_mut(), 10)
}

// ---------------------------------------------------------------------------
// Floating point parsing
// ---------------------------------------------------------------------------

/// Parse the longest valid floating-point prefix. Returns (value, consumed).
pub fn scan_float(get: &dyn Fn(usize) -> u32) -> (f64, usize, bool) {
    let mut i = 0;
    while is_space(get(i)) {
        i += 1;
    }
    let start_num = i;
    let mut neg = false;
    match get(i) {
        0x2B => i += 1,
        0x2D => {
            neg = true;
            i += 1
        }
        _ => {}
    }
    let lower = |k: usize| -> u8 { (get(k) as u8).to_ascii_lowercase() };
    let matches = |k: usize, w: &str| w.bytes().enumerate().all(|(j, b)| get(k + j) < 0x80 && lower(k + j) == b);
    let sign = if neg { -1.0 } else { 1.0 };
    if matches(i, "infinity") {
        return (sign * f64::INFINITY, i + 8, false);
    }
    if matches(i, "inf") {
        return (sign * f64::INFINITY, i + 3, false);
    }
    if matches(i, "nan") {
        let mut j = i + 3;
        if get(j) == b'(' as u32 {
            let mut k = j + 1;
            while (get(k) as u8).is_ascii_alphanumeric() && get(k) < 0x80 || get(k) == b'_' as u32 {
                k += 1;
            }
            if get(k) == b')' as u32 {
                j = k + 1;
            }
        }
        let nan = if neg { -f64::NAN } else { f64::NAN };
        return (nan, j, false);
    }
    // Hex float
    if get(i) == b'0' as u32 && lower(i + 1) == b'x' {
        let mut j = i + 2;
        let hexd = |c: u32| (c as u8 as char).to_digit(16).filter(|_| c < 0x80);
        let mut mant: u64 = 0;
        let mut exp: i64 = 0;
        let mut any = false;
        let mut seen_dot = false;
        loop {
            let c = get(j);
            if let Some(d) = hexd(c) {
                any = true;
                if mant < (1u64 << 60) {
                    mant = mant * 16 + d as u64;
                    if seen_dot {
                        exp -= 4;
                    }
                } else if !seen_dot {
                    exp += 4;
                }
                j += 1;
            } else if c == b'.' as u32 && !seen_dot {
                seen_dot = true;
                j += 1;
            } else {
                break;
            }
        }
        if !any {
            // "0x" without digits: just the "0" converts.
            return (0.0 * sign, i + 1, false);
        }
        if lower(j) == b'p' {
            let mut k = j + 1;
            let mut eneg = false;
            if get(k) == b'+' as u32 || get(k) == b'-' as u32 {
                eneg = get(k) == b'-' as u32;
                k += 1;
            }
            if (get(k) as u8).is_ascii_digit() && get(k) < 0x80 {
                let mut e: i64 = 0;
                while (get(k) as u8).is_ascii_digit() && get(k) < 0x80 {
                    e = (e * 10 + (get(k) - 0x30) as i64).min(100_000);
                    k += 1;
                }
                exp += if eneg { -e } else { e };
                j = k;
            }
        }
        let v = sign * (mant as f64) * 2f64.powi(exp.clamp(-2000, 2000) as i32);
        let range = v.is_infinite() || (v == 0.0 && mant != 0);
        return (v, j, range);
    }
    // Decimal
    let mut s = String::new();
    if neg {
        s.push('-');
    }
    let mut j = i;
    let mut digits = 0;
    while (get(j) as u8).is_ascii_digit() && get(j) < 0x80 {
        s.push(get(j) as u8 as char);
        j += 1;
        digits += 1;
    }
    if get(j) == b'.' as u32 {
        let mut k = j + 1;
        let mut frac = String::from(".");
        while (get(k) as u8).is_ascii_digit() && get(k) < 0x80 {
            frac.push(get(k) as u8 as char);
            k += 1;
            digits += 1;
        }
        if digits > 0 {
            s.push_str(&frac);
            j = k;
        }
    }
    if digits == 0 {
        let _ = start_num;
        return (0.0, 0, false);
    }
    if lower(j) == b'e' {
        let mut k = j + 1;
        let mut e = String::from("e");
        if get(k) == b'+' as u32 || get(k) == b'-' as u32 {
            e.push(get(k) as u8 as char);
            k += 1;
        }
        if (get(k) as u8).is_ascii_digit() && get(k) < 0x80 {
            while (get(k) as u8).is_ascii_digit() && get(k) < 0x80 {
                e.push(get(k) as u8 as char);
                k += 1;
            }
            s.push_str(&e);
            j = k;
        }
    }
    let v: f64 = s.parse().unwrap_or(0.0);
    let nonzero_digits = s.bytes().take_while(|&b| b != b'e').any(|b| (b'1'..=b'9').contains(&b));
    let range = v.is_infinite() || (v == 0.0 && nonzero_digits) || (v != 0.0 && v.abs() < f64::MIN_POSITIVE);
    (v, j, range)
}

pub unsafe extern "win64" fn strtod(s: *const u8, end: *mut *const u8) -> f64 {
    if s.is_null() {
        errno::set(EINVAL);
        invalid_parameter("strtod");
        return 0.0;
    }
    let (v, n, range) = scan_float(&nget(s));
    if range {
        errno::set(ERANGE);
    }
    if !end.is_null() {
        *end = s.add(n);
    }
    v
}
pub unsafe extern "win64" fn strtof(s: *const u8, end: *mut *const u8) -> f32 {
    let v = strtod(s, end);
    let f = v as f32;
    if (f.is_infinite() && !v.is_infinite()) || (f == 0.0 && v != 0.0) {
        errno::set(ERANGE);
    }
    f
}
pub unsafe extern "win64" fn strtold(s: *const u8, end: *mut *const u8) -> f64 {
    strtod(s, end)
}
pub unsafe extern "win64" fn _strtod_l(s: *const u8, end: *mut *const u8, _l: *mut c_void) -> f64 {
    strtod(s, end)
}
pub unsafe extern "win64" fn _strtof_l(s: *const u8, end: *mut *const u8, _l: *mut c_void) -> f32 {
    strtof(s, end)
}
pub unsafe extern "win64" fn atof(s: *const u8) -> f64 {
    strtod(s, std::ptr::null_mut())
}
pub unsafe extern "win64" fn _atof_l(s: *const u8, _l: *mut c_void) -> f64 {
    atof(s)
}
pub unsafe extern "win64" fn wcstod(s: *const u16, end: *mut *const u16) -> f64 {
    if s.is_null() {
        errno::set(EINVAL);
        invalid_parameter("wcstod");
        return 0.0;
    }
    let (v, n, range) = scan_float(&wget(s));
    if range {
        errno::set(ERANGE);
    }
    if !end.is_null() {
        *end = s.add(n);
    }
    v
}
pub unsafe extern "win64" fn wcstof(s: *const u16, end: *mut *const u16) -> f32 {
    wcstod(s, end) as f32
}
pub unsafe extern "win64" fn wcstold(s: *const u16, end: *mut *const u16) -> f64 {
    wcstod(s, end)
}
pub unsafe extern "win64" fn _wtof(s: *const u16) -> f64 {
    wcstod(s, std::ptr::null_mut())
}

// ---------------------------------------------------------------------------
// Integer -> string
// ---------------------------------------------------------------------------

fn radix_str(mut v: u64, neg: bool, radix: u32) -> String {
    let mut buf = Vec::new();
    loop {
        let d = (v % radix as u64) as u8;
        buf.push(if d < 10 { b'0' + d } else { b'a' + d - 10 });
        v /= radix as u64;
        if v == 0 {
            break;
        }
    }
    if neg {
        buf.push(b'-');
    }
    buf.reverse();
    String::from_utf8(buf).unwrap()
}

/// MSVC semantics: negative sign only for radix 10 signed conversions.
fn int_str(v: i64, radix: u32, bits32: bool) -> String {
    if radix == 10 && v < 0 {
        radix_str(v.unsigned_abs(), true, 10)
    } else if bits32 {
        radix_str(v as u32 as u64, false, radix)
    } else {
        radix_str(v as u64, false, radix)
    }
}

unsafe fn put_a(s: &str, buf: *mut u8) -> *mut u8 {
    std::ptr::copy_nonoverlapping(s.as_ptr(), buf, s.len());
    *buf.add(s.len()) = 0;
    buf
}
unsafe fn put_w(s: &str, buf: *mut u16) -> *mut u16 {
    for (i, b) in s.bytes().enumerate() {
        *buf.add(i) = b as u16;
    }
    *buf.add(s.len()) = 0;
    buf
}
unsafe fn put_a_s(s: &str, buf: *mut u8, sz: usize, radix: i32) -> i32 {
    if buf.is_null() || sz == 0 || !(2..=36).contains(&radix) {
        errno::set(EINVAL);
        invalid_parameter("_itoa_s");
        return EINVAL;
    }
    if s.len() + 1 > sz {
        *buf = 0;
        errno::set(ERANGE);
        invalid_parameter("_itoa_s");
        return ERANGE;
    }
    put_a(s, buf);
    0
}
unsafe fn put_w_s(s: &str, buf: *mut u16, sz: usize, radix: i32) -> i32 {
    if buf.is_null() || sz == 0 || !(2..=36).contains(&radix) {
        errno::set(EINVAL);
        invalid_parameter("_itow_s");
        return EINVAL;
    }
    if s.len() + 1 > sz {
        *buf = 0;
        errno::set(ERANGE);
        invalid_parameter("_itow_s");
        return ERANGE;
    }
    put_w(s, buf);
    0
}

pub unsafe extern "win64" fn _itoa(v: i32, buf: *mut u8, r: i32) -> *mut u8 {
    put_a(&int_str(v as i64, r as u32, true), buf)
}
pub unsafe extern "win64" fn _ltoa(v: i32, buf: *mut u8, r: i32) -> *mut u8 {
    put_a(&int_str(v as i64, r as u32, true), buf)
}
pub unsafe extern "win64" fn _ultoa(v: u32, buf: *mut u8, r: i32) -> *mut u8 {
    put_a(&radix_str(v as u64, false, r as u32), buf)
}
pub unsafe extern "win64" fn _i64toa(v: i64, buf: *mut u8, r: i32) -> *mut u8 {
    put_a(&int_str(v, r as u32, false), buf)
}
pub unsafe extern "win64" fn _ui64toa(v: u64, buf: *mut u8, r: i32) -> *mut u8 {
    put_a(&radix_str(v, false, r as u32), buf)
}
pub unsafe extern "win64" fn _itoa_s(v: i32, buf: *mut u8, sz: usize, r: i32) -> i32 {
    put_a_s(&int_str(v as i64, r as u32, true), buf, sz, r)
}
pub unsafe extern "win64" fn _ltoa_s(v: i32, buf: *mut u8, sz: usize, r: i32) -> i32 {
    put_a_s(&int_str(v as i64, r as u32, true), buf, sz, r)
}
pub unsafe extern "win64" fn _ultoa_s(v: u32, buf: *mut u8, sz: usize, r: i32) -> i32 {
    put_a_s(&radix_str(v as u64, false, r as u32), buf, sz, r)
}
pub unsafe extern "win64" fn _i64toa_s(v: i64, buf: *mut u8, sz: usize, r: i32) -> i32 {
    put_a_s(&int_str(v, r as u32, false), buf, sz, r)
}
pub unsafe extern "win64" fn _ui64toa_s(v: u64, buf: *mut u8, sz: usize, r: i32) -> i32 {
    put_a_s(&radix_str(v, false, r as u32), buf, sz, r)
}
pub unsafe extern "win64" fn _itow(v: i32, buf: *mut u16, r: i32) -> *mut u16 {
    put_w(&int_str(v as i64, r as u32, true), buf)
}
pub unsafe extern "win64" fn _ltow(v: i32, buf: *mut u16, r: i32) -> *mut u16 {
    put_w(&int_str(v as i64, r as u32, true), buf)
}
pub unsafe extern "win64" fn _ultow(v: u32, buf: *mut u16, r: i32) -> *mut u16 {
    put_w(&radix_str(v as u64, false, r as u32), buf)
}
pub unsafe extern "win64" fn _i64tow(v: i64, buf: *mut u16, r: i32) -> *mut u16 {
    put_w(&int_str(v, r as u32, false), buf)
}
pub unsafe extern "win64" fn _ui64tow(v: u64, buf: *mut u16, r: i32) -> *mut u16 {
    put_w(&radix_str(v, false, r as u32), buf)
}
pub unsafe extern "win64" fn _itow_s(v: i32, buf: *mut u16, sz: usize, r: i32) -> i32 {
    put_w_s(&int_str(v as i64, r as u32, true), buf, sz, r)
}
pub unsafe extern "win64" fn _i64tow_s(v: i64, buf: *mut u16, sz: usize, r: i32) -> i32 {
    put_w_s(&int_str(v, r as u32, false), buf, sz, r)
}
pub unsafe extern "win64" fn _ui64tow_s(v: u64, buf: *mut u16, sz: usize, r: i32) -> i32 {
    put_w_s(&radix_str(v, false, r as u32), buf, sz, r)
}

// ---------------------------------------------------------------------------
// Multibyte <-> wide (CRT locale code page: 0 = "C" byte mapping, 65001 = UTF-8)
// ---------------------------------------------------------------------------

/// Decode one multibyte char. Ok((wchar(s), bytes)) / Err(()) = EILSEQ / Ok(None) incomplete.
fn mb_decode(bytes: &[u8]) -> Result<Option<(Vec<u16>, usize)>, ()> {
    if bytes.is_empty() {
        return Ok(None);
    }
    if mb_cp() == 0 {
        return Ok(Some((vec![bytes[0] as u16], 1)));
    }
    let b0 = bytes[0];
    let len = match b0 {
        0x00..=0x7F => 1,
        0xC2..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF4 => 4,
        _ => return Err(()),
    };
    if bytes.len() < len {
        if bytes[1..].iter().all(|&b| b & 0xC0 == 0x80) {
            return Ok(None);
        }
        return Err(());
    }
    match std::str::from_utf8(&bytes[..len]) {
        Ok(s) => Ok(Some((s.encode_utf16().collect(), len))),
        Err(_) => Err(()),
    }
}

fn wc_encode(c: u16) -> Result<Vec<u8>, ()> {
    if mb_cp() == 0 {
        if c < 256 {
            Ok(vec![c as u8])
        } else {
            Err(())
        }
    } else {
        match char::from_u32(c as u32) {
            Some(ch) => Ok(ch.to_string().into_bytes()),
            None => Err(()),
        }
    }
}

unsafe fn mbs_to_wcs(dst: *mut u16, src: *const u8, n: usize) -> usize {
    let bytes = crate::strings::cstr_slice(src);
    let mut out: Vec<u16> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        match mb_decode(&bytes[i..]) {
            Ok(Some((w, k))) => {
                out.extend(w);
                i += k;
            }
            _ => {
                errno::set(EILSEQ);
                return usize::MAX;
            }
        }
        if !dst.is_null() && out.len() >= n {
            break;
        }
    }
    if dst.is_null() {
        return out.len();
    }
    let k = out.len().min(n);
    std::ptr::copy_nonoverlapping(out.as_ptr(), dst, k);
    if k < n {
        *dst.add(k) = 0;
    }
    k
}

unsafe fn wcs_to_mbs(dst: *mut u8, src: *const u16, n: usize) -> usize {
    let w = crate::strings::wide_slice(src);
    let mut out: Vec<u8> = Vec::new();
    let s;
    if mb_cp() == 65001 {
        s = String::from_utf16(w).map_err(|_| ());
        match s {
            Ok(st) => {
                for ch in st.chars() {
                    let mut b = [0u8; 4];
                    let e = ch.encode_utf8(&mut b);
                    if !dst.is_null() && out.len() + e.len() > n {
                        break;
                    }
                    out.extend_from_slice(e.as_bytes());
                }
            }
            Err(()) => {
                errno::set(EILSEQ);
                return usize::MAX;
            }
        }
    } else {
        for &c in w {
            match wc_encode(c) {
                Ok(b) => {
                    if !dst.is_null() && out.len() + b.len() > n {
                        break;
                    }
                    out.extend(b);
                }
                Err(()) => {
                    errno::set(EILSEQ);
                    return usize::MAX;
                }
            }
        }
    }
    if dst.is_null() {
        return out.len();
    }
    std::ptr::copy_nonoverlapping(out.as_ptr(), dst, out.len());
    if out.len() < n {
        *dst.add(out.len()) = 0;
    }
    out.len()
}

pub unsafe extern "win64" fn mbstowcs(dst: *mut u16, src: *const u8, n: usize) -> usize {
    mbs_to_wcs(dst, src, n)
}
pub unsafe extern "win64" fn wcstombs(dst: *mut u8, src: *const u16, n: usize) -> usize {
    wcs_to_mbs(dst, src, n)
}

const TRUNCATE: usize = usize::MAX;

pub unsafe extern "win64" fn mbstowcs_s(ret: *mut usize, dst: *mut u16, sz: usize, src: *const u8, count: usize) -> i32 {
    if src.is_null() || (dst.is_null() && sz != 0) {
        errno::set(EINVAL);
        invalid_parameter("mbstowcs_s");
        return EINVAL;
    }
    let need = mbs_to_wcs(std::ptr::null_mut(), src, 0);
    if need == usize::MAX {
        if !dst.is_null() {
            *dst = 0;
        }
        return EILSEQ;
    }
    if dst.is_null() {
        if !ret.is_null() {
            *ret = need + 1;
        }
        return 0;
    }
    let want = if count == TRUNCATE { need.min(sz - 1) } else { need.min(count) };
    if want + 1 > sz {
        *dst = 0;
        errno::set(super::errno::ERANGE);
        invalid_parameter("mbstowcs_s");
        return super::errno::ERANGE;
    }
    let n = mbs_to_wcs(dst, src, want);
    *dst.add(n) = 0;
    if !ret.is_null() {
        *ret = n + 1;
    }
    if count == TRUNCATE && need > want {
        return super::errno::STRUNCATE;
    }
    0
}

pub unsafe extern "win64" fn wcstombs_s(ret: *mut usize, dst: *mut u8, sz: usize, src: *const u16, count: usize) -> i32 {
    if src.is_null() || (dst.is_null() && sz != 0) {
        errno::set(EINVAL);
        invalid_parameter("wcstombs_s");
        return EINVAL;
    }
    let need = wcs_to_mbs(std::ptr::null_mut(), src, 0);
    if need == usize::MAX {
        if !dst.is_null() {
            *dst = 0;
        }
        return EILSEQ;
    }
    if dst.is_null() {
        if !ret.is_null() {
            *ret = need + 1;
        }
        return 0;
    }
    let want = if count == TRUNCATE { need.min(sz - 1) } else { need.min(count) };
    if want + 1 > sz {
        *dst = 0;
        errno::set(super::errno::ERANGE);
        invalid_parameter("wcstombs_s");
        return super::errno::ERANGE;
    }
    let n = wcs_to_mbs(dst, src, want);
    *dst.add(n) = 0;
    if !ret.is_null() {
        *ret = n + 1;
    }
    if count == TRUNCATE && need > want {
        return super::errno::STRUNCATE;
    }
    0
}

pub unsafe extern "win64" fn mbtowc(dst: *mut u16, s: *const u8, n: usize) -> i32 {
    if s.is_null() {
        return 0;
    }
    if n == 0 {
        return -1;
    }
    if *s == 0 {
        if !dst.is_null() {
            *dst = 0;
        }
        return 0;
    }
    match mb_decode(std::slice::from_raw_parts(s, n.min(4))) {
        Ok(Some((w, k))) => {
            if !dst.is_null() {
                *dst = w[0];
            }
            k as i32
        }
        _ => {
            errno::set(EILSEQ);
            -1
        }
    }
}
pub unsafe extern "win64" fn mbrtowc(dst: *mut u16, s: *const u8, n: usize, _st: *mut u32) -> usize {
    if s.is_null() {
        return 0;
    }
    if n == 0 {
        return usize::MAX - 1;
    }
    if *s == 0 {
        if !dst.is_null() {
            *dst = 0;
        }
        return 0;
    }
    match mb_decode(std::slice::from_raw_parts(s, n.min(4))) {
        Ok(Some((w, k))) => {
            if !dst.is_null() {
                *dst = w[0];
            }
            k
        }
        Ok(None) => usize::MAX - 1, // (size_t)-2: incomplete
        Err(()) => {
            errno::set(EILSEQ);
            usize::MAX
        }
    }
}
pub unsafe extern "win64" fn mblen(s: *const u8, n: usize) -> i32 {
    mbtowc(std::ptr::null_mut(), s, n)
}
pub unsafe extern "win64" fn mbrlen(s: *const u8, n: usize, st: *mut u32) -> usize {
    mbrtowc(std::ptr::null_mut(), s, n, st)
}
pub unsafe extern "win64" fn wctomb(d: *mut u8, c: u16) -> i32 {
    if d.is_null() {
        return 0;
    }
    match wc_encode(c) {
        Ok(b) => {
            std::ptr::copy_nonoverlapping(b.as_ptr(), d, b.len());
            b.len() as i32
        }
        Err(()) => {
            errno::set(EILSEQ);
            -1
        }
    }
}
pub unsafe extern "win64" fn wctomb_s(ret: *mut i32, d: *mut u8, sz: usize, c: u16) -> i32 {
    match wc_encode(c) {
        Ok(b) => {
            if !d.is_null() {
                if b.len() > sz {
                    errno::set(ERANGE);
                    return ERANGE;
                }
                std::ptr::copy_nonoverlapping(b.as_ptr(), d, b.len());
            }
            if !ret.is_null() {
                *ret = b.len() as i32;
            }
            0
        }
        Err(()) => {
            if !ret.is_null() {
                *ret = -1;
            }
            errno::set(EILSEQ);
            EILSEQ
        }
    }
}
pub unsafe extern "win64" fn wcrtomb(d: *mut u8, c: u16, _st: *mut u32) -> usize {
    if d.is_null() {
        return 1;
    }
    match wctomb(d, c) {
        -1 => usize::MAX,
        n => n as usize,
    }
}
pub unsafe extern "win64" fn wcrtomb_s(ret: *mut usize, d: *mut u8, sz: usize, c: u16, _st: *mut u32) -> i32 {
    let mut n = 0i32;
    let r = wctomb_s(&mut n, d, sz, c);
    if !ret.is_null() {
        *ret = n as usize;
    }
    r
}
pub unsafe extern "win64" fn btowc(c: i32) -> u16 {
    if c == -1 {
        return 0xFFFF;
    }
    let b = [c as u8];
    match mb_decode(&b) {
        Ok(Some((w, 1))) => w[0],
        _ => 0xFFFF, // WEOF
    }
}
pub unsafe extern "win64" fn wctob(c: u16) -> i32 {
    match wc_encode(c) {
        Ok(b) if b.len() == 1 => b[0] as i32,
        _ => -1,
    }
}
pub unsafe extern "win64" fn mbsinit(_st: *const u32) -> i32 {
    1
}
pub unsafe extern "win64" fn mbsrtowcs(dst: *mut u16, src: *mut *const u8, n: usize, _st: *mut u32) -> usize {
    let r = mbs_to_wcs(dst, *src, n);
    if !dst.is_null() && r != usize::MAX {
        let consumed = crate::strings::strlen(*src);
        if r < n {
            *src = std::ptr::null();
        } else {
            let _ = consumed;
        }
    }
    r
}
pub unsafe extern "win64" fn wcsrtombs(dst: *mut u8, src: *mut *const u16, n: usize, _st: *mut u32) -> usize {
    let r = wcs_to_mbs(dst, *src, n);
    if !dst.is_null() && r != usize::MAX && r < n {
        *src = std::ptr::null();
    }
    r
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

type Cmp = unsafe extern "win64" fn(*const c_void, *const c_void) -> i32;
type CmpS = unsafe extern "win64" fn(*mut c_void, *const c_void, *const c_void) -> i32;

unsafe fn sort_with(base: *mut u8, n: usize, w: usize, cmp: &mut dyn FnMut(*const c_void, *const c_void) -> i32) {
    if n < 2 || w == 0 {
        return;
    }
    // Sort an index permutation, then apply it (elements may be any size).
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| cmp(base.add(a * w) as _, base.add(b * w) as _).cmp(&0));
    let tmp = std::slice::from_raw_parts(base, n * w).to_vec();
    for (dst, &src) in idx.iter().enumerate() {
        std::ptr::copy_nonoverlapping(tmp.as_ptr().add(src * w), base.add(dst * w), w);
    }
}

pub unsafe extern "win64" fn qsort(base: *mut c_void, n: usize, w: usize, cmp: Cmp) {
    sort_with(base as *mut u8, n, w, &mut |a, b| crate::call_guest8(cmp as usize, a as u64, b as u64, 0, 0, 0, 0, 0, 0) as i32);
}
pub unsafe extern "win64" fn qsort_s(base: *mut c_void, n: usize, w: usize, cmp: CmpS, ctx: *mut c_void) {
    sort_with(base as *mut u8, n, w, &mut |a, b| crate::call_guest8(cmp as usize, ctx as u64, a as u64, b as u64, 0, 0, 0, 0, 0) as i32);
}
pub unsafe extern "win64" fn bsearch(key: *const c_void, base: *const c_void, n: usize, w: usize, cmp: Cmp) -> *mut c_void {
    let (mut lo, mut hi) = (0usize, n);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let p = (base as *const u8).add(mid * w);
        let r = crate::call_guest8(cmp as usize, key as u64, p as u64, 0, 0, 0, 0, 0, 0) as i32;
        if r == 0 {
            return p as *mut c_void;
        } else if r < 0 {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    std::ptr::null_mut()
}
pub unsafe extern "win64" fn bsearch_s(key: *const c_void, base: *const c_void, n: usize, w: usize, cmp: CmpS, ctx: *mut c_void) -> *mut c_void {
    let (mut lo, mut hi) = (0usize, n);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let p = (base as *const u8).add(mid * w);
        let r = crate::call_guest8(cmp as usize, ctx as u64, key as u64, p as u64, 0, 0, 0, 0, 0) as i32;
        if r == 0 {
            return p as *mut c_void;
        } else if r < 0 {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    std::ptr::null_mut()
}

thread_local! {
    static RAND_NEXT: std::cell::Cell<u32> = const { std::cell::Cell::new(1) };
}

/// MSVC's LCG: per-thread state, 15-bit output.
pub unsafe extern "win64" fn rand() -> i32 {
    RAND_NEXT.with(|s| {
        let n = s.get().wrapping_mul(214013).wrapping_add(2531011);
        s.set(n);
        ((n >> 16) & 0x7FFF) as i32
    })
}
pub unsafe extern "win64" fn srand(seed: u32) {
    RAND_NEXT.with(|s| s.set(seed));
}
pub unsafe extern "win64" fn rand_s(out: *mut u32) -> i32 {
    if out.is_null() {
        errno::set(EINVAL);
        return EINVAL;
    }
    if !crate::det_entropy::fill(out as *mut u8, 4) {
        *out = libc::arc4random();
    }
    0
}

pub unsafe extern "win64" fn abs(v: i32) -> i32 {
    v.wrapping_abs()
}
pub unsafe extern "win64" fn labs(v: i32) -> i32 {
    v.wrapping_abs()
}
pub unsafe extern "win64" fn llabs(v: i64) -> i64 {
    v.wrapping_abs()
}
pub unsafe extern "win64" fn _abs64(v: i64) -> i64 {
    v.wrapping_abs()
}
pub unsafe extern "win64" fn imaxabs(v: i64) -> i64 {
    v.wrapping_abs()
}
/// `div_t div(int, int)`: 8-byte struct returned in RAX.
pub unsafe extern "win64" fn div(a: i32, b: i32) -> u64 {
    let q = a.wrapping_div(b);
    let r = a.wrapping_rem(b);
    (q as u32 as u64) | ((r as u32 as u64) << 32)
}
pub unsafe extern "win64" fn ldiv(a: i32, b: i32) -> u64 {
    div(a, b)
}
/// `lldiv_t` (16 bytes) is returned via hidden pointer in RCX.
pub unsafe extern "win64" fn lldiv(ret: *mut i64, a: i64, b: i64) -> *mut i64 {
    *ret = a.wrapping_div(b);
    *ret.add(1) = a.wrapping_rem(b);
    ret
}
pub unsafe extern "win64" fn _rotl(v: u32, s: i32) -> u32 {
    v.rotate_left(s as u32 & 31)
}
pub unsafe extern "win64" fn _rotr(v: u32, s: i32) -> u32 {
    v.rotate_right(s as u32 & 31)
}
pub unsafe extern "win64" fn _rotl64(v: u64, s: i32) -> u64 {
    v.rotate_left(s as u32 & 63)
}
pub unsafe extern "win64" fn _rotr64(v: u64, s: i32) -> u64 {
    v.rotate_right(s as u32 & 63)
}
pub unsafe extern "win64" fn _lrotl(v: u32, s: i32) -> u32 {
    v.rotate_left(s as u32 & 31)
}
pub unsafe extern "win64" fn _lrotr(v: u32, s: i32) -> u32 {
    v.rotate_right(s as u32 & 31)
}
pub unsafe extern "win64" fn _byteswap_ushort(v: u16) -> u16 {
    v.swap_bytes()
}
pub unsafe extern "win64" fn _byteswap_ulong(v: u32) -> u32 {
    v.swap_bytes()
}
pub unsafe extern "win64" fn _byteswap_uint64(v: u64) -> u64 {
    v.swap_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strtol_family() {
        unsafe {
            let mut e = std::ptr::null();
            assert_eq!(strtol(b"  -0x1Fz\0".as_ptr(), &mut e, 0), -31);
            assert_eq!(*e, b'z');
            assert_eq!(strtol(b"99999999999\0".as_ptr(), std::ptr::null_mut(), 10), i32::MAX);
            assert_eq!(errno::get(), ERANGE);
            assert_eq!(strtoul(b"-1\0".as_ptr(), std::ptr::null_mut(), 10), u32::MAX);
            assert_eq!(strtoull(b"0777\0".as_ptr(), std::ptr::null_mut(), 0), 0o777);
            assert_eq!(strtol(b"xyz\0".as_ptr(), &mut e, 10), 0);
        }
    }
    #[test]
    fn strtod_family() {
        unsafe {
            let mut e = std::ptr::null();
            let s = b"  3.25e2abc\0";
            assert_eq!(strtod(s.as_ptr(), &mut e), 325.0);
            assert_eq!(e.offset_from(s.as_ptr()), 8);
            assert_eq!(strtod(b"0x1.8p1\0".as_ptr(), std::ptr::null_mut()), 3.0);
            assert!(strtod(b"-inf\0".as_ptr(), std::ptr::null_mut()).is_infinite());
            assert!(strtod(b"nan(ind)\0".as_ptr(), std::ptr::null_mut()).is_nan());
            assert_eq!(strtod(b"1.\0".as_ptr(), &mut e), 1.0);
            assert_eq!(strtod(b".5\0".as_ptr(), std::ptr::null_mut()), 0.5);
        }
    }
    #[test]
    fn itoa_and_rand() {
        unsafe {
            let mut b = [0u8; 40];
            _itoa(-255, b.as_mut_ptr(), 16);
            assert_eq!(crate::strings::from_ansi(b.as_ptr()), "ffffff01");
            _itoa(-255, b.as_mut_ptr(), 10);
            assert_eq!(crate::strings::from_ansi(b.as_ptr()), "-255");
            srand(1);
            assert_eq!(rand(), 41); // MSVC's first rand() after srand(1)
        }
    }
}
