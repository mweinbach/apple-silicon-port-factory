//! printf engine (`__stdio_common_v*printf*`, msvcrt `_vsnprintf`, ...).
//!
//! Formats into a `Vec<u16>` (wide) or `Vec<u8>` (narrow) of code units.
//! Windows semantics: `long` is 32 bit; `%I64`, `%I32`, `%I`, `%z`, `%t`,
//! `%j`, `%ll`, `%w` are supported; `%p` prints 16 uppercase hex digits;
//! `%n` is refused (invalid parameter), as in the UCRT default.
//!
//! Option bits (`_CRT_INTERNAL_PRINTF_*`):
//! 1 LEGACY_VSPRINTF_NULL_TERMINATION, 2 STANDARD_SNPRINTF_BEHAVIOR,
//! 4 LEGACY_WIDE_SPECIFIERS, 8 LEGACY_MSVCRT_COMPATIBILITY,
//! 0x10 LEGACY_THREE_DIGIT_EXPONENTS.

use crate::strings::strlen;
use crate::va::VaList;
use std::ffi::c_void;

pub const LEGACY_VSPRINTF_NULL_TERMINATION: u64 = 1;
pub const STANDARD_SNPRINTF_BEHAVIOR: u64 = 2;
pub const LEGACY_WIDE_SPECIFIERS: u64 = 4;
pub const LEGACY_MSVCRT_COMPATIBILITY: u64 = 8;
pub const LEGACY_THREE_DIGIT_EXPONENTS: u64 = 0x10;

/// Source format string as code units.
#[derive(Clone, Copy)]
pub enum Fmt {
    Narrow(*const u8),
    Wide(*const u16),
}

impl Fmt {
    unsafe fn at(&self, i: usize) -> u32 {
        match self {
            Fmt::Narrow(p) => *p.add(i) as u32,
            Fmt::Wide(p) => std::ptr::read_unaligned(p.add(i)) as u32,
        }
    }
    fn wide(&self) -> bool {
        matches!(self, Fmt::Wide(_))
    }
}

#[derive(Default, Clone, Copy)]
struct Spec {
    left: bool,
    plus: bool,
    space: bool,
    alt: bool,
    zero: bool,
    width: usize,
    prec: Option<usize>,
}

/// Output accumulator: code units (bytes for narrow, UTF-16 for wide).
pub struct Out {
    pub wide: bool,
    pub units: Vec<u32>,
}

impl Out {
    fn push_str(&mut self, s: &str) {
        if self.wide {
            self.units.extend(s.encode_utf16().map(|c| c as u32));
        } else {
            self.units.extend(s.bytes().map(|b| b as u32));
        }
    }
    fn push_units(&mut self, u: &[u32]) {
        self.units.extend_from_slice(u);
    }
    fn pad(&mut self, n: usize, c: u32) {
        self.units.extend(std::iter::repeat(c).take(n));
    }
}

fn pad_field(out: &mut Out, sp: &Spec, body: &[u32]) {
    let n = body.len();
    if sp.width > n && !sp.left {
        out.pad(sp.width - n, ' ' as u32);
    }
    out.push_units(body);
    if sp.width > n && sp.left {
        out.pad(sp.width - n, ' ' as u32);
    }
}

fn str_units(s: &str, wide: bool) -> Vec<u32> {
    if wide {
        s.encode_utf16().map(|c| c as u32).collect()
    } else {
        s.bytes().map(|b| b as u32).collect()
    }
}

/// Integer conversion with sign/prefix/precision/zero-padding.
fn fmt_int(out: &mut Out, sp: &Spec, neg: bool, mag: u64, conv: u8) {
    let digits = match conv {
        b'o' => format!("{mag:o}"),
        b'x' => format!("{mag:x}"),
        b'X' => format!("{mag:X}"),
        _ => format!("{mag}"),
    };
    let mut digits = if sp.prec == Some(0) && mag == 0 { String::new() } else { digits };
    if let Some(p) = sp.prec {
        while digits.len() < p {
            digits.insert(0, '0');
        }
    }
    let mut prefix = String::new();
    if neg {
        prefix.push('-');
    } else if matches!(conv, b'd' | b'i') {
        if sp.plus {
            prefix.push('+');
        } else if sp.space {
            prefix.push(' ');
        }
    }
    if sp.alt && mag != 0 {
        match conv {
            b'x' => prefix.push_str("0x"),
            b'X' => prefix.push_str("0X"),
            b'o' if !digits.starts_with('0') => digits.insert(0, '0'),
            _ => {}
        }
    }
    let total = prefix.len() + digits.len();
    if sp.zero && !sp.left && sp.prec.is_none() && sp.width > total {
        let z = "0".repeat(sp.width - total);
        let s = format!("{prefix}{z}{digits}");
        out.push_str(&s);
    } else {
        pad_field(out, sp, &str_units(&format!("{prefix}{digits}"), out.wide));
    }
}

/// C-style exponent formatting of `v` (finite, non-negative) with `prec`
/// fraction digits: `d.dddde+XX`.
fn exp_string(v: f64, prec: usize, upper: bool, three: bool) -> String {
    let s = format!("{:.*e}", prec, v);
    let (m, e) = s.split_once('e').unwrap();
    let e: i32 = e.parse().unwrap();
    let mind = if three { 3 } else { 2 };
    let es = format!("{}{:0width$}", if e < 0 { '-' } else { '+' }, e.unsigned_abs(), width = mind);
    format!("{m}{}{es}", if upper { 'E' } else { 'e' })
}

fn strip_trailing_zeros(s: &mut String) {
    let (mant, exp) = match s.find(['e', 'E']) {
        Some(i) => (s[..i].to_string(), s[i..].to_string()),
        None => (s.clone(), String::new()),
    };
    if mant.contains('.') {
        let t = mant.trim_end_matches('0').trim_end_matches('.').to_string();
        *s = t + &exp;
    }
}

fn hex_float(v: f64, prec: Option<usize>, upper: bool) -> String {
    let bits = v.to_bits();
    let exp_bits = ((bits >> 52) & 0x7FF) as i32;
    let mut mant = bits & ((1u64 << 52) - 1);
    let (lead, exp) = if exp_bits == 0 {
        if mant == 0 {
            (0, 0)
        } else {
            (0, -1022)
        }
    } else {
        (1, exp_bits - 1023)
    };
    let mut digits = 13usize;
    if let Some(p) = prec {
        if p < 13 {
            let shift = (13 - p) * 4;
            let round = (mant >> (shift - 1)) & 1;
            mant >>= shift;
            mant += round;
            digits = p;
        }
    }
    let mut frac = if digits == 0 { String::new() } else { format!("{:0width$x}", mant, width = digits) };
    if prec.is_none() {
        frac = frac.trim_end_matches('0').to_string();
    }
    let lead = lead + if digits > 0 && mant >> (digits * 4) != 0 { 1 } else { 0 };
    let s = if frac.is_empty() { format!("0x{lead}p{exp:+}") } else { format!("0x{lead}.{frac}p{exp:+}") };
    if upper {
        s.to_uppercase()
    } else {
        s
    }
}

fn fmt_float(out: &mut Out, sp: &Spec, v: f64, conv: u8, opts: u64) {
    let upper = conv.is_ascii_uppercase();
    let neg = v.is_sign_negative();
    let a = v.abs();
    let mut body;
    if v.is_nan() {
        body = if neg { "nan(ind)".to_string() } else { "nan".to_string() };
        if upper {
            body = body.to_uppercase();
        }
    } else if v.is_infinite() {
        body = if upper { "INF".into() } else { "inf".into() };
    } else {
        let three = opts & LEGACY_THREE_DIGIT_EXPONENTS != 0;
        let p = sp.prec.unwrap_or(6);
        body = match conv.to_ascii_lowercase() {
            b'f' => format!("{:.*}", p, a),
            b'e' => exp_string(a, p, upper, three),
            b'a' => hex_float(a, sp.prec, upper),
            _ => {
                // %g
                let p = if p == 0 { 1 } else { p };
                let x = if a == 0.0 {
                    0
                } else {
                    let s = format!("{:.*e}", p - 1, a);
                    s.split_once('e').unwrap().1.parse::<i32>().unwrap()
                };
                let mut s = if (x as i64) < p as i64 && x >= -4 {
                    format!("{:.*}", (p as i64 - 1 - x as i64) as usize, a)
                } else {
                    exp_string(a, p - 1, upper, three)
                };
                if !sp.alt {
                    strip_trailing_zeros(&mut s);
                }
                s
            }
        };
        if sp.alt && !body.contains('.') && conv.to_ascii_lowercase() != b'a' {
            match body.find(['e', 'E']) {
                Some(i) => body.insert(i, '.'),
                None => body.push('.'),
            }
        }
    }
    let sign = if neg && !(v.is_nan()) {
        "-"
    } else if v.is_nan() && neg {
        "-"
    } else if sp.plus {
        "+"
    } else if sp.space {
        " "
    } else {
        ""
    };
    let total = sign.len() + body.len();
    if sp.zero && !sp.left && v.is_finite() && sp.width > total {
        let (pfx, rest) = if body.starts_with("0x") || body.starts_with("0X") { body.split_at(2) } else { ("", body.as_str()) };
        let s = format!("{sign}{pfx}{}{rest}", "0".repeat(sp.width - total));
        out.push_str(&s);
    } else {
        pad_field(out, sp, &str_units(&format!("{sign}{body}"), out.wide));
    }
}

/// Core formatter. Returns `None` on an invalid format (e.g. `%n`).
pub unsafe fn format(fmt: Fmt, out_wide: bool, opts: u64, args: &mut VaList) -> Option<Vec<u32>> {
    let mut out = Out { wide: out_wide, units: Vec::new() };
    let fw = fmt.wide();
    // In wide functions, does plain %s/%c mean wide? (legacy MS behaviour)
    let wide_default = fw && opts & LEGACY_WIDE_SPECIFIERS != 0;
    let mut i = 0usize;
    loop {
        let c = fmt.at(i);
        if c == 0 {
            break;
        }
        if c != '%' as u32 {
            out.units.push(c);
            i += 1;
            continue;
        }
        i += 1;
        let mut sp = Spec::default();
        // Flags
        loop {
            match fmt.at(i) as u8 {
                b'-' => sp.left = true,
                b'+' => sp.plus = true,
                b' ' => sp.space = true,
                b'#' => sp.alt = true,
                b'0' => sp.zero = true,
                _ => break,
            }
            i += 1;
        }
        // Width
        if fmt.at(i) == '*' as u32 {
            let w = args.next_i32();
            if w < 0 {
                sp.left = true;
            }
            sp.width = w.unsigned_abs() as usize;
            i += 1;
        } else {
            while let Some(d) = char::from_u32(fmt.at(i)).and_then(|c| c.to_digit(10)) {
                sp.width = sp.width * 10 + d as usize;
                i += 1;
            }
        }
        // Precision
        if fmt.at(i) == '.' as u32 {
            i += 1;
            if fmt.at(i) == '*' as u32 {
                let p = args.next_i32();
                sp.prec = if p < 0 { None } else { Some(p as usize) };
                i += 1;
            } else {
                let mut p = 0usize;
                while let Some(d) = char::from_u32(fmt.at(i)).and_then(|c| c.to_digit(10)) {
                    p = p * 10 + d as usize;
                    i += 1;
                }
                sp.prec = Some(p);
            }
        }
        // Length: size in bytes of the integer argument; `wide_arg` for s/c.
        let mut size = 4usize;
        let mut wide_arg: Option<bool> = None;
        loop {
            match fmt.at(i) as u8 {
                b'h' => {
                    if fmt.at(i + 1) == 'h' as u32 {
                        size = 1;
                        i += 1;
                    } else {
                        size = 2;
                    }
                    wide_arg = Some(false);
                }
                b'l' => {
                    if fmt.at(i + 1) == 'l' as u32 {
                        size = 8;
                        i += 1;
                    } else {
                        size = 4;
                        wide_arg = Some(true);
                    }
                }
                b'w' => wide_arg = Some(true),
                b'L' | b'q' => size = 8,
                b'j' | b'z' | b't' => size = 8,
                b'I' => {
                    if fmt.at(i + 1) == '6' as u32 && fmt.at(i + 2) == '4' as u32 {
                        size = 8;
                        i += 2;
                    } else if fmt.at(i + 1) == '3' as u32 && fmt.at(i + 2) == '2' as u32 {
                        size = 4;
                        i += 2;
                    } else {
                        size = 8;
                    }
                }
                b'T' => {}
                _ => break,
            }
            i += 1;
        }
        let conv = fmt.at(i);
        i += 1;
        let conv8 = if conv < 0x80 { conv as u8 } else { 0 };
        match conv8 {
            b'%' => out.units.push('%' as u32),
            b'd' | b'i' => {
                let raw = args.next_u64();
                let v: i64 = match size {
                    1 => raw as i8 as i64,
                    2 => raw as i16 as i64,
                    4 => raw as i32 as i64,
                    _ => raw as i64,
                };
                fmt_int(&mut out, &sp, v < 0, v.unsigned_abs(), b'd');
            }
            b'u' | b'o' | b'x' | b'X' => {
                let raw = args.next_u64();
                let v: u64 = match size {
                    1 => raw as u8 as u64,
                    2 => raw as u16 as u64,
                    4 => raw as u32 as u64,
                    _ => raw,
                };
                fmt_int(&mut out, &sp, false, v, conv8);
            }
            b'p' => {
                let v = args.next_u64();
                let s = format!("{v:016X}");
                let u = str_units(&s, out.wide);
                pad_field(&mut out, &Spec { zero: false, ..sp }, &u);
            }
            b'c' | b'C' => {
                let v = args.next_u64();
                let is_wide = wide_arg.unwrap_or(if conv8 == b'c' { wide_default } else { !wide_default });
                let unit: Vec<u32> = if is_wide {
                    let ch = v as u16;
                    if out.wide {
                        vec![ch as u32]
                    } else {
                        String::from_utf16_lossy(&[ch]).bytes().map(|b| b as u32).collect()
                    }
                } else {
                    vec![v as u8 as u32]
                };
                pad_field(&mut out, &sp, &unit);
            }
            b's' | b'S' | b'Z' => {
                let p = args.next_u64() as usize;
                let is_wide = match wide_arg {
                    Some(w) => w,
                    None => {
                        if conv8 == b's' {
                            wide_default
                        } else {
                            // %S is the "other" width.
                            !wide_default
                        }
                    }
                };
                let units: Vec<u32> = if conv8 == b'Z' {
                    // ANSI_STRING/UNICODE_STRING*: Length(u16), MaxLength, Buffer.
                    if p == 0 {
                        str_units("(null)", out.wide)
                    } else {
                        let len = std::ptr::read_unaligned(p as *const u16) as usize;
                        let buf = std::ptr::read_unaligned((p + 8) as *const usize);
                        if buf == 0 {
                            str_units("(null)", out.wide)
                        } else if is_wide {
                            let s = read_wide_exact_unaligned(buf as *const u16, len / 2);
                            conv_wide(&s, out.wide)
                        } else {
                            let s = std::slice::from_raw_parts(buf as *const u8, len);
                            conv_narrow(s, out.wide)
                        }
                    }
                } else if p == 0 {
                    str_units("(null)", out.wide)
                } else if is_wide {
                    let s = read_wide_unaligned(p as *const u16, sp.prec);
                    conv_wide(&s, out.wide)
                } else {
                    let n = match sp.prec {
                        Some(pr) => (0..pr).take_while(|&k| *((p as *const u8).add(k)) != 0).count(),
                        None => strlen(p as *const u8),
                    };
                    conv_narrow(std::slice::from_raw_parts(p as *const u8, n), out.wide)
                };
                let units = match sp.prec {
                    Some(pr) if units.len() > pr => units[..pr].to_vec(),
                    _ => units,
                };
                pad_field(&mut out, &sp, &units);
            }
            b'e' | b'E' | b'f' | b'F' | b'g' | b'G' | b'a' | b'A' => {
                let v = args.next_f64();
                fmt_float(&mut out, &sp, v, conv8, opts);
            }
            b'n' => {
                let _ = args.next_u64();
                log::warn!("printf: %n is disabled (UCRT default) -> invalid parameter");
                super::runtime::invalid_parameter("printf %n");
                return None;
            }
            _ => {
                log::warn!("printf: unsupported conversion {:?}", char::from_u32(conv));
                super::runtime::invalid_parameter("printf: bad format");
                return None;
            }
        }
    }
    Some(out.units)
}

unsafe fn read_wide_unaligned(p: *const u16, max_units: Option<usize>) -> Vec<u16> {
    let mut v = Vec::new();
    let limit = max_units.unwrap_or(usize::MAX);
    let mut k = 0usize;
    while k < limit {
        let c = std::ptr::read_unaligned(p.add(k));
        if c == 0 {
            break;
        }
        v.push(c);
        k += 1;
    }
    v
}

unsafe fn read_wide_exact_unaligned(p: *const u16, count: usize) -> Vec<u16> {
    let mut v = Vec::with_capacity(count);
    for k in 0..count {
        v.push(std::ptr::read_unaligned(p.add(k)));
    }
    v
}

fn conv_wide(s: &[u16], out_wide: bool) -> Vec<u32> {
    if out_wide {
        s.iter().map(|&c| c as u32).collect()
    } else {
        // wide -> narrow: UTF-8 (our ACP is 65001).
        String::from_utf16_lossy(s).bytes().map(|b| b as u32).collect()
    }
}

fn conv_narrow(s: &[u8], out_wide: bool) -> Vec<u32> {
    if out_wide {
        if super::locale::c_locale_active() {
            s.iter().map(|&b| b as u32).collect()
        } else {
            String::from_utf8_lossy(s).encode_utf16().map(|c| c as u32).collect()
        }
    } else {
        s.iter().map(|&b| b as u32).collect()
    }
}

/// Narrow output as bytes.
pub fn units_to_bytes(u: &[u32]) -> Vec<u8> {
    u.iter().map(|&c| c as u8).collect()
}

// ---------------------------------------------------------------------------
// Buffer variants
// ---------------------------------------------------------------------------

/// Copy `res` into `buf[..count]` with `vsnprintf`-style semantics without assuming alignment.
unsafe fn store<T: Copy + From<u8>>(res: &[T], buf: *mut T, count: usize, opts: u64) -> i32 {
    let n = res.len();
    let zero: T = T::from(0u8);
    if buf.is_null() {
        return if count == 0 { n as i32 } else { -1 };
    }
    if n < count {
        std::ptr::copy_nonoverlapping(res.as_ptr() as *const u8, buf as *mut u8, n * std::mem::size_of::<T>());
        std::ptr::write_unaligned(buf.add(n), zero);
        return n as i32;
    }
    // Truncation (n >= count).
    std::ptr::copy_nonoverlapping(res.as_ptr() as *const u8, buf as *mut u8, count * std::mem::size_of::<T>());
    if opts & STANDARD_SNPRINTF_BEHAVIOR != 0 {
        if count > 0 {
            std::ptr::write_unaligned(buf.add(count - 1), zero);
        }
        return n as i32;
    }
    if opts & LEGACY_VSPRINTF_NULL_TERMINATION != 0 {
        return if n == count { n as i32 } else { -1 };
    }
    if count > 0 {
        std::ptr::write_unaligned(buf.add(count - 1), zero);
    }
    -1
}

unsafe fn fmt_narrow(opts: u64, fmt: *const u8, va: *const u8) -> Option<Vec<u8>> {
    if fmt.is_null() {
        super::runtime::invalid_parameter("printf: format is NULL");
        super::errno::set(super::errno::EINVAL);
        return None;
    }
    let mut a = VaList::new(va);
    format(Fmt::Narrow(fmt), false, opts, &mut a).map(|u| units_to_bytes(&u))
}

unsafe fn fmt_wide(opts: u64, fmt: *const u16, va: *const u8) -> Option<Vec<u16>> {
    if fmt.is_null() {
        super::runtime::invalid_parameter("wprintf: format is NULL");
        super::errno::set(super::errno::EINVAL);
        return None;
    }
    let mut a = VaList::new(va);
    format(Fmt::Wide(fmt), true, opts, &mut a).map(|u| u.iter().map(|&c| c as u16).collect())
}

/// Format a narrow Win32 printf format string using `va_list` without triggering
/// CRT invalid-parameter handlers (used by `user32!wsprintfA`).
pub(crate) unsafe fn format_narrow_va(fmt: *const u8, va: *const u8) -> Option<Vec<u8>> {
    if fmt.is_null() {
        return None;
    }
    let mut a = VaList::new(va);
    format(Fmt::Narrow(fmt), false, 0, &mut a).map(|u| units_to_bytes(&u))
}

/// Format a wide Win32 printf format string using `va_list` without triggering
/// CRT invalid-parameter handlers (used by `user32!wsprintfW`, where `%s`/`%c`
/// are wide and `%S`/`%C` are narrow).
pub(crate) unsafe fn format_wide_va(fmt: *const u16, va: *const u8) -> Option<Vec<u16>> {
    if fmt.is_null() {
        return None;
    }
    let mut a = VaList::new(va);
    format(Fmt::Wide(fmt), true, LEGACY_WIDE_SPECIFIERS, &mut a)
        .map(|u| u.iter().map(|&c| c as u16).collect())
}

pub unsafe extern "win64" fn __stdio_common_vsprintf(
    opts: u64,
    buf: *mut u8,
    count: usize,
    fmt: *const u8,
    _loc: *mut c_void,
    va: *const u8,
) -> i32 {
    match fmt_narrow(opts, fmt, va) {
        Some(r) => store(&r, buf, count, opts),
        None => -1,
    }
}

pub unsafe extern "win64" fn __stdio_common_vswprintf(
    opts: u64,
    buf: *mut u16,
    count: usize,
    fmt: *const u16,
    _loc: *mut c_void,
    va: *const u8,
) -> i32 {
    match fmt_wide(opts, fmt, va) {
        Some(r) => store(&r, buf, count, opts),
        None => -1,
    }
}

unsafe fn store_s<T: Copy + From<u8>>(res: &[T], buf: *mut T, size: usize) -> i32 {
    if buf.is_null() || size == 0 {
        super::runtime::invalid_parameter("sprintf_s: buffer");
        super::errno::set(super::errno::EINVAL);
        return -1;
    }
    if res.len() >= size {
        std::ptr::write_unaligned(buf, T::from(0u8));
        super::errno::set(super::errno::ERANGE);
        super::runtime::invalid_parameter("sprintf_s: buffer too small");
        return -1;
    }
    std::ptr::copy_nonoverlapping(res.as_ptr() as *const u8, buf as *mut u8, res.len() * std::mem::size_of::<T>());
    std::ptr::write_unaligned(buf.add(res.len()), T::from(0u8));
    res.len() as i32
}

pub unsafe extern "win64" fn __stdio_common_vsprintf_s(
    opts: u64,
    buf: *mut u8,
    size: usize,
    fmt: *const u8,
    _loc: *mut c_void,
    va: *const u8,
) -> i32 {
    match fmt_narrow(opts, fmt, va) {
        Some(r) => store_s(&r, buf, size),
        None => -1,
    }
}

pub unsafe extern "win64" fn __stdio_common_vswprintf_s(
    opts: u64,
    buf: *mut u16,
    size: usize,
    fmt: *const u16,
    _loc: *mut c_void,
    va: *const u8,
) -> i32 {
    match fmt_wide(opts, fmt, va) {
        Some(r) => store_s(&r, buf, size),
        None => -1,
    }
}

const TRUNCATE: usize = usize::MAX;

unsafe fn store_sn<T: Copy + From<u8>>(res: &[T], buf: *mut T, size: usize, count: usize) -> i32 {
    if buf.is_null() || size == 0 {
        super::runtime::invalid_parameter("_vsnprintf_s: buffer");
        super::errno::set(super::errno::EINVAL);
        return -1;
    }
    let n = res.len();
    let limit = if count == TRUNCATE { size - 1 } else { count.min(size - 1) };
    if n <= limit {
        std::ptr::copy_nonoverlapping(res.as_ptr(), buf, n);
        *buf.add(n) = T::from(0u8);
        return n as i32;
    }
    if count == TRUNCATE || count < size {
        std::ptr::copy_nonoverlapping(res.as_ptr(), buf, limit);
        *buf.add(limit) = T::from(0u8);
        if count == TRUNCATE {
            super::errno::set(80); // STRUNCATE
        }
        return -1;
    }
    *buf = T::from(0u8);
    super::errno::set(super::errno::ERANGE);
    super::runtime::invalid_parameter("_vsnprintf_s: buffer too small");
    -1
}

pub unsafe extern "win64" fn __stdio_common_vsnprintf_s(
    opts: u64,
    buf: *mut u8,
    size: usize,
    count: usize,
    fmt: *const u8,
    _loc: *mut c_void,
    va: *const u8,
) -> i32 {
    match fmt_narrow(opts, fmt, va) {
        Some(r) => store_sn(&r, buf, size, count),
        None => -1,
    }
}

pub unsafe extern "win64" fn __stdio_common_vsnwprintf_s(
    opts: u64,
    buf: *mut u16,
    size: usize,
    count: usize,
    fmt: *const u16,
    _loc: *mut c_void,
    va: *const u8,
) -> i32 {
    match fmt_wide(opts, fmt, va) {
        Some(r) => store_sn(&r, buf, size, count),
        None => -1,
    }
}

/// Stream printf (used by stdio.rs and msvcrt).
pub unsafe fn vfprintf_impl(opts: u64, stream: *mut c_void, fmt: *const u8, va: *const u8) -> i32 {
    match fmt_narrow(opts, fmt, va) {
        Some(r) => super::stdio::write_stream_bytes(stream, &r),
        None => -1,
    }
}

pub unsafe extern "win64" fn __stdio_common_vfprintf(
    opts: u64,
    stream: *mut c_void,
    fmt: *const u8,
    _loc: *mut c_void,
    va: *const u8,
) -> i32 {
    vfprintf_impl(opts, stream, fmt, va)
}

pub unsafe extern "win64" fn __stdio_common_vfprintf_s(
    opts: u64,
    stream: *mut c_void,
    fmt: *const u8,
    _loc: *mut c_void,
    va: *const u8,
) -> i32 {
    vfprintf_impl(opts, stream, fmt, va)
}

pub unsafe extern "win64" fn __stdio_common_vfwprintf(
    opts: u64,
    stream: *mut c_void,
    fmt: *const u16,
    _loc: *mut c_void,
    va: *const u8,
) -> i32 {
    match fmt_wide(opts, fmt, va) {
        Some(r) => {
            // Text streams in ANSI mode: wide chars go out via the locale's
            // multibyte encoding (UTF-8 here).
            let s = String::from_utf16_lossy(&r);
            let n = super::stdio::write_stream_bytes(stream, s.as_bytes());
            if n < 0 {
                n
            } else {
                r.len() as i32
            }
        }
        None => -1,
    }
}

pub unsafe extern "win64" fn __stdio_common_vfwprintf_s(
    opts: u64,
    stream: *mut c_void,
    fmt: *const u16,
    loc: *mut c_void,
    va: *const u8,
) -> i32 {
    __stdio_common_vfwprintf(opts, stream, fmt, loc, va)
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn f(fmt: &str, args: &[u64]) -> String {
        let z = std::ffi::CString::new(fmt).unwrap();
        let mut a = VaList::new(args.as_ptr() as *const u8);
        String::from_utf8(units_to_bytes(&format(Fmt::Narrow(z.as_ptr() as *const u8), false, 0, &mut a).unwrap())).unwrap()
    }

    #[test]
    fn integers_and_strings() {
        unsafe {
            assert_eq!(f("%d|%5d|%-5d|%05d", &[(-12i64) as u64, 42, 7, 3]), "-12|   42|7    |00003");
            assert_eq!(f("%x %X %#x %o", &[255, 255, 255, 8]), "ff FF 0xff 10");
            assert_eq!(f("%ld %lld %I64u", &[0xFFFF_FFFF, u64::MAX, u64::MAX]), "-1 -1 18446744073709551615");
            assert_eq!(f("%u", &[0xFFFF_FFFF_FFFF_FFFF]), "4294967295");
            let s = b"hello\0";
            assert_eq!(f("[%s] [%.3s] [%7s]", &[s.as_ptr() as u64, s.as_ptr() as u64, s.as_ptr() as u64]), "[hello] [hel] [  hello]");
            let w: Vec<u16> = "wide\0".encode_utf16().collect();
            assert_eq!(f("%ls %S", &[w.as_ptr() as u64, w.as_ptr() as u64]), "wide wide");
            assert_eq!(f("%s", &[0]), "(null)");
            assert_eq!(f("%p", &[0x1234]), "0000000000001234");
            assert_eq!(f("%c%%", &[b'A' as u64]), "A%");
            assert_eq!(f("%*d", &[(-4i64) as u64, 1]), "1   ");
        }
    }

    #[test]
    fn floats() {
        unsafe {
            let b = |v: f64| v.to_bits();
            assert_eq!(f("%f %.2f %e", &[b(1.5), b(3.14159), b(12345.678)]), "1.500000 3.14 1.234568e+04");
            assert_eq!(f("%g %g %g %g", &[b(0.0001), b(100000.0), b(1e6), b(1.5)]), "0.0001 100000 1e+06 1.5");
            assert_eq!(f("%G", &[b(1e-10)]), "1E-10");
            assert_eq!(f("%08.3f|%+.1f", &[b(-3.5), b(2.0)]), "-003.500|+2.0");
            assert_eq!(f("%f %f", &[b(f64::INFINITY), b(f64::NAN)]), "inf nan");
            assert_eq!(f("%a", &[b(1.0)]), "0x1p+0");
            assert_eq!(f("%.0f %#.0f", &[b(2.5), b(2.0)]), "2 2.");
        }
    }

    #[test]
    fn snprintf_semantics() {
        unsafe {
            let fmt = b"abcdef\0";
            let mut buf = [0xAAu8; 4];
            assert_eq!(__stdio_common_vsprintf(0, buf.as_mut_ptr(), 4, fmt.as_ptr(), std::ptr::null_mut(), std::ptr::null()), -1);
            assert_eq!(&buf, b"abc\0");
            assert_eq!(
                __stdio_common_vsprintf(STANDARD_SNPRINTF_BEHAVIOR, buf.as_mut_ptr(), 4, fmt.as_ptr(), std::ptr::null_mut(), std::ptr::null()),
                6
            );
            assert_eq!(__stdio_common_vsprintf(0, std::ptr::null_mut(), 0, fmt.as_ptr(), std::ptr::null_mut(), std::ptr::null()), 6);
        }
    }
}
