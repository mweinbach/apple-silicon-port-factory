//! scanf engine (`__stdio_common_vsscanf`, `__stdio_common_vswscanf`).
//!
//! Option bits: 1 = SECURECRT (`%s`/`%c`/`%[` take a buffer size argument
//! after the pointer), 2 = LEGACY_WIDE_SPECIFIERS, 4 = LEGACY_MSVCRT_COMPAT.

use crate::va::VaList;
use std::ffi::c_void;

const SECURE: u64 = 1;
const LEGACY_WIDE: u64 = 2;

struct Input {
    wide: bool,
    p: *const u8,
    len: usize,
    pos: usize,
}

impl Input {
    unsafe fn peek(&self) -> Option<u32> {
        if self.pos >= self.len {
            return None;
        }
        let c = if self.wide { *(self.p as *const u16).add(self.pos) as u32 } else { *self.p.add(self.pos) as u32 };
        if c == 0 {
            None
        } else {
            Some(c)
        }
    }
    fn bump(&mut self) {
        self.pos += 1;
    }
}

fn is_space(c: u32) -> bool {
    c == 0x20 || (0x09..=0x0D).contains(&c)
}

unsafe fn fmt_at(fmt: *const u8, wide: bool, i: usize) -> u32 {
    if wide {
        *(fmt as *const u16).add(i) as u32
    } else {
        *fmt.add(i) as u32
    }
}

/// Returns number of assigned fields, or -1 (EOF) on input failure before
/// the first conversion.
unsafe fn scan(input: &mut Input, fmt: *const u8, opts: u64, loc: *mut c_void, args: &mut VaList) -> i32 {
    let wide = input.wide;
    let wide_default = wide && opts & LEGACY_WIDE != 0;
    let mut assigned = 0;
    let mut converted_any = false;
    let mut i = 0usize;
    loop {
        let fc = fmt_at(fmt, wide, i);
        if fc == 0 {
            break;
        }
        if is_space(fc) {
            while input.peek().is_some_and(is_space) {
                input.bump();
            }
            i += 1;
            continue;
        }
        if fc != '%' as u32 {
            match input.peek() {
                Some(c) if c == fc => {
                    input.bump();
                    i += 1;
                    continue;
                }
                None if !converted_any => return -1,
                _ => break,
            }
        }
        i += 1;
        if fmt_at(fmt, wide, i) == '%' as u32 {
            while input.peek().is_some_and(is_space) {
                input.bump();
            }
            if input.peek() == Some('%' as u32) {
                input.bump();
                i += 1;
                continue;
            }
            break;
        }
        let mut suppress = false;
        if fmt_at(fmt, wide, i) == '*' as u32 {
            suppress = true;
            i += 1;
        }
        let mut width = 0usize;
        while let Some(d) = char::from_u32(fmt_at(fmt, wide, i)).and_then(|c| c.to_digit(10)) {
            width = width * 10 + d as usize;
            i += 1;
        }
        let mut size = 4usize; // int
        let mut is_double = false;
        let mut wide_arg: Option<bool> = None;
        loop {
            match fmt_at(fmt, wide, i) as u8 {
                b'h' => {
                    if fmt_at(fmt, wide, i + 1) == 'h' as u32 {
                        size = 1;
                        i += 1;
                    } else {
                        size = 2;
                    }
                    wide_arg = Some(false);
                }
                b'l' => {
                    if fmt_at(fmt, wide, i + 1) == 'l' as u32 {
                        size = 8;
                        i += 1;
                    } else {
                        is_double = true;
                        wide_arg = Some(true);
                    }
                }
                b'L' => {
                    size = 8;
                    is_double = true;
                }
                b'w' => wide_arg = Some(true),
                b'j' | b'z' | b't' => size = 8,
                b'I' => {
                    if fmt_at(fmt, wide, i + 1) == '6' as u32 && fmt_at(fmt, wide, i + 2) == '4' as u32 {
                        size = 8;
                        i += 2;
                    } else if fmt_at(fmt, wide, i + 1) == '3' as u32 && fmt_at(fmt, wide, i + 2) == '2' as u32 {
                        size = 4;
                        i += 2;
                    } else {
                        size = 8;
                    }
                }
                _ => break,
            }
            i += 1;
        }
        let conv = fmt_at(fmt, wide, i) as u8;
        i += 1;
        if !matches!(conv, b'c' | b'C' | b'[' | b'n') {
            while input.peek().is_some_and(is_space) {
                input.bump();
            }
        }
        if input.peek().is_none() && conv != b'n' {
            if !converted_any {
                return -1;
            }
            break;
        }
        let maxw = if width == 0 { usize::MAX } else { width };
        match conv {
            b'd' | b'i' | b'u' | b'o' | b'x' | b'X' | b'p' => {
                let mut base = match conv {
                    b'o' => 8,
                    b'x' | b'X' | b'p' => 16,
                    b'i' => 0,
                    _ => 10,
                };
                let mut s = String::new();
                let mut n = 0;
                if let Some(c) = input.peek() {
                    if (c == '+' as u32 || c == '-' as u32) && n < maxw {
                        s.push(c as u8 as char);
                        input.bump();
                        n += 1;
                    }
                }
                if (base == 0 || base == 16) && input.peek() == Some('0' as u32) && n < maxw {
                    input.bump();
                    n += 1;
                    if matches!(input.peek(), Some(0x78) | Some(0x58)) && n < maxw {
                        input.bump();
                        n += 1;
                        base = 16;
                    } else {
                        s.push('0');
                        if base == 0 {
                            base = 8;
                        }
                    }
                }
                if base == 0 {
                    base = 10;
                }
                while n < maxw {
                    match input.peek().and_then(char::from_u32) {
                        Some(c) if c.is_digit(base) => {
                            s.push(c);
                            input.bump();
                            n += 1;
                        }
                        _ => break,
                    }
                }
                let digits = s.trim_start_matches(['+', '-']);
                if digits.is_empty() {
                    break;
                }
                let neg = s.starts_with('-');
                let mag = u64::from_str_radix(digits, base).unwrap_or(u64::MAX);
                let v = if neg { (mag as i64).wrapping_neg() as u64 } else { mag };
                converted_any = true;
                if !suppress {
                    let p = args.next_ptr::<u8>();
                    let sz = if conv == b'p' { 8 } else { size };
                    match sz {
                        1 => *p = v as u8,
                        2 => *(p as *mut u16) = v as u16,
                        8 => *(p as *mut u64) = v,
                        _ => *(p as *mut u32) = v as u32,
                    }
                    assigned += 1;
                }
            }
            b'e' | b'E' | b'f' | b'F' | b'g' | b'G' | b'a' | b'A' => {
                let start = input.pos;
                let inp = &*input;
                let lim = maxw;
                let get = |k: usize| -> u32 {
                    if k >= lim {
                        return 0;
                    }
                    let q = Input { wide: inp.wide, p: inp.p, len: inp.len, pos: start + k };
                    q.peek().unwrap_or(0)
                };
                let mut dec_buf = [0u32; 8];
                let dec_len = if wide {
                    let u = super::locale::numeric_decimal_wide(loc);
                    let n = u.len().min(dec_buf.len());
                    for (idx, &c) in u[..n].iter().enumerate() {
                        dec_buf[idx] = c as u32;
                    }
                    n
                } else {
                    let b = super::locale::numeric_decimal_narrow(loc);
                    let n = b.len().min(dec_buf.len());
                    for (idx, &c) in b[..n].iter().enumerate() {
                        dec_buf[idx] = c as u32;
                    }
                    n
                };
                let (v, used, _) = super::convert::scan_float_with_dec(&get, &dec_buf[..dec_len]);
                if used == 0 {
                    break;
                }
                input.pos = start + used;
                converted_any = true;
                if !suppress {
                    let p = args.next_ptr::<u8>();
                    if is_double {
                        *(p as *mut f64) = v;
                    } else {
                        *(p as *mut f32) = v as f32;
                    }
                    assigned += 1;
                }
            }
            b's' | b'S' | b'c' | b'C' | b'[' => {
                // Character set for %[.
                let mut set: Vec<(u32, u32)> = Vec::new();
                let mut negate = false;
                if conv == b'[' {
                    if fmt_at(fmt, wide, i) == '^' as u32 {
                        negate = true;
                        i += 1;
                    }
                    let mut first = true;
                    loop {
                        let c = fmt_at(fmt, wide, i);
                        if c == 0 {
                            break;
                        }
                        if c == ']' as u32 && !first {
                            i += 1;
                            break;
                        }
                        first = false;
                        if fmt_at(fmt, wide, i + 1) == '-' as u32 && fmt_at(fmt, wide, i + 2) != ']' as u32 && fmt_at(fmt, wide, i + 2) != 0 {
                            let hi = fmt_at(fmt, wide, i + 2);
                            set.push((c.min(hi), c.max(hi)));
                            i += 3;
                        } else {
                            set.push((c, c));
                            i += 1;
                        }
                    }
                }
                let lower = conv.to_ascii_lowercase();
                let is_wide = wide_arg.unwrap_or(match conv {
                    b's' | b'c' | b'[' => wide_default,
                    _ => !wide_default,
                });
                let maxc = if lower == b'c' && width == 0 { 1 } else { maxw };
                let mut got: Vec<u32> = Vec::new();
                while got.len() < maxc {
                    let Some(c) = input.peek() else { break };
                    let ok = match lower {
                        b's' => !is_space(c),
                        b'c' => true,
                        _ => set.iter().any(|&(a, b)| c >= a && c <= b) != negate,
                    };
                    if !ok {
                        break;
                    }
                    got.push(c);
                    input.bump();
                }
                if got.is_empty() || (lower == b'c' && got.len() < maxc) && got.is_empty() {
                    break;
                }
                converted_any = true;
                if !suppress {
                    let p = args.next_ptr::<u8>();
                    let cap = if opts & SECURE != 0 { args.next_u64() as u32 as usize } else { usize::MAX };
                    let need = got.len() + if lower == b'c' { 0 } else { 1 };
                    if need > cap {
                        if cap > 0 {
                            if is_wide {
                                *(p as *mut u16) = 0;
                            } else {
                                *p = 0;
                            }
                        }
                        super::errno::set(super::errno::ENOMEM);
                        break;
                    }
                    // Convert between input width and destination width.
                    let units: Vec<u32> = match (wide, is_wide) {
                        (false, true) => {
                            let bytes: Vec<u8> = got.iter().map(|&c| c as u8).collect();
                            String::from_utf8_lossy(&bytes).encode_utf16().map(|c| c as u32).collect()
                        }
                        (true, false) => {
                            let w: Vec<u16> = got.iter().map(|&c| c as u16).collect();
                            String::from_utf16_lossy(&w).bytes().map(|b| b as u32).collect()
                        }
                        _ => got.clone(),
                    };
                    for (k, &u) in units.iter().enumerate() {
                        if is_wide {
                            *(p as *mut u16).add(k) = u as u16;
                        } else {
                            *p.add(k) = u as u8;
                        }
                    }
                    if lower != b'c' {
                        if is_wide {
                            *(p as *mut u16).add(units.len()) = 0;
                        } else {
                            *p.add(units.len()) = 0;
                        }
                    }
                    assigned += 1;
                }
            }
            b'n' => {
                if !suppress {
                    let p = args.next_ptr::<i32>();
                    *p = input.pos as i32;
                }
            }
            _ => {
                log::warn!("scanf: unsupported conversion {:?}", conv as char);
                break;
            }
        }
    }
    assigned
}

pub unsafe extern "win64" fn __stdio_common_vsscanf(
    opts: u64,
    buf: *const u8,
    count: usize,
    fmt: *const u8,
    loc: *mut c_void,
    va: *const u8,
) -> i32 {
    if buf.is_null() || fmt.is_null() {
        super::errno::set(super::errno::EINVAL);
        super::runtime::invalid_parameter("sscanf: NULL");
        return -1;
    }
    let mut input = Input { wide: false, p: buf, len: count, pos: 0 };
    let mut a = VaList::new(va);
    scan(&mut input, fmt, opts, loc, &mut a)
}

pub unsafe extern "win64" fn __stdio_common_vswscanf(
    opts: u64,
    buf: *const u16,
    count: usize,
    fmt: *const u16,
    loc: *mut c_void,
    va: *const u8,
) -> i32 {
    if buf.is_null() || fmt.is_null() {
        super::errno::set(super::errno::EINVAL);
        super::runtime::invalid_parameter("swscanf: NULL");
        return -1;
    }
    let mut input = Input { wide: true, p: buf as *const u8, len: count, pos: 0 };
    let mut a = VaList::new(va);
    scan(&mut input, fmt as *const u8, opts, loc, &mut a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sscanf_basic() {
        unsafe {
            let mut a: i32 = 0;
            let mut b: u64 = 0;
            let mut f: f32 = 0.0;
            let mut d: f64 = 0.0;
            let mut s = [0u8; 16];
            let args: [u64; 6] = [
                &mut a as *mut _ as u64,
                &mut b as *mut _ as u64,
                &mut f as *mut _ as u64,
                &mut d as *mut _ as u64,
                s.as_mut_ptr() as u64,
                16,
            ];
            let input = b"-42 0x1F 1.5 2.25e1 word rest\0";
            let fmt = b"%d %llx %f %lf %s\0";
            let n = __stdio_common_vsscanf(SECURE, input.as_ptr(), usize::MAX, fmt.as_ptr(), std::ptr::null_mut(), args.as_ptr() as *const u8);
            assert_eq!(n, 5);
            assert_eq!((a, b, f, d), (-42, 0x1F, 1.5, 22.5));
            assert_eq!(&s[..5], b"word\0");
            let n = __stdio_common_vsscanf(0, b"\0".as_ptr(), usize::MAX, fmt.as_ptr(), std::ptr::null_mut(), args.as_ptr() as *const u8);
            assert_eq!(n, -1);
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let args2: [u64; 2] = [&mut x as *mut _ as u64, &mut y as *mut _ as u64];
            let n = __stdio_common_vsscanf(0, b"10x20\0".as_ptr(), usize::MAX, b"%dx%d\0".as_ptr(), std::ptr::null_mut(), args2.as_ptr() as *const u8);
            assert_eq!((n, x, y), (2, 10, 20));

            // LC_NUMERIC = de-DE uses ',' in sscanf/swscanf float conversions.
            let loc_de = crate::ucrt::locale::_create_locale(4 /* LC_NUMERIC */, b"de-DE\0".as_ptr());
            assert!(!loc_de.is_null());
            let mut df: f64 = 0.0;
            let args_de: [u64; 1] = [&mut df as *mut _ as u64];
            let n = __stdio_common_vsscanf(0, b"1,75\0".as_ptr(), usize::MAX, b"%lf\0".as_ptr(), loc_de, args_de.as_ptr() as *const u8);
            assert_eq!((n, df), (1, 1.75));
            crate::ucrt::locale::_free_locale(loc_de);
        }
    }
}
