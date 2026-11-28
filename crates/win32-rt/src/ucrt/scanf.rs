//! scanf engine (`__stdio_common_vsscanf`, `__stdio_common_vswscanf`,
//! `__stdio_common_vfscanf`, `__stdio_common_vfwscanf`).
//!
//! Option bits: 1 = SECURECRT (`%s`/`%c`/`%[` take a buffer size argument
//! after the pointer), 2 = LEGACY_WIDE_SPECIFIERS, 4 = LEGACY_MSVCRT_COMPAT.

use crate::va::VaList;
use std::cell::RefCell;
use std::ffi::c_void;

const SECURE: u64 = 1;
const LEGACY_WIDE: u64 = 2;
const WEOF: u32 = 0xFFFF;

struct StreamBuf {
    ahead: Vec<u32>,
    eof: bool,
}

enum Source {
    Mem {
        p: *const u8,
        len: usize,
    },
    Stream {
        fp: *mut super::stdio::FILE,
        cp: u32,
        buf: RefCell<StreamBuf>,
    },
}

struct Input {
    wide: bool,
    src: Source,
    pos: usize,
}

impl Input {
    fn from_mem(wide: bool, p: *const u8, len: usize) -> Self {
        Self { wide, src: Source::Mem { p, len }, pos: 0 }
    }

    fn from_stream(wide: bool, fp: *mut super::stdio::FILE, cp: u32) -> Self {
        Self {
            wide,
            src: Source::Stream { fp, cp, buf: RefCell::new(StreamBuf { ahead: Vec::new(), eof: false }) },
            pos: 0,
        }
    }

    unsafe fn read_stream_unit(wide: bool, fp: *mut super::stdio::FILE, cp: u32) -> Option<u32> {
        if wide {
            let c = super::stdio::read_stream_wide_cp(fp, cp);
            if c == WEOF { None } else { Some(c) }
        } else {
            let c = super::stdio::fgetc(fp);
            if c < 0 { None } else { Some(c as u32) }
        }
    }

    unsafe fn peek_at(&self, off: usize) -> Option<u32> {
        match &self.src {
            Source::Mem { p, len } => {
                let idx = self.pos.checked_add(off)?;
                if idx >= *len {
                    return None;
                }
                let c = if self.wide {
                    (*p as *const u16).add(idx).read_unaligned() as u32
                } else {
                    *p.add(idx) as u32
                };
                if c == 0 { None } else { Some(c) }
            }
            Source::Stream { fp, cp, buf } => {
                let mut sb = buf.borrow_mut();
                while sb.ahead.len() <= off && !sb.eof {
                    match Self::read_stream_unit(self.wide, *fp, *cp) {
                        Some(c) => sb.ahead.push(c),
                        None => sb.eof = true,
                    }
                }
                sb.ahead.get(off).copied()
            }
        }
    }

    unsafe fn peek(&self) -> Option<u32> {
        self.peek_at(0)
    }

    fn bump(&mut self) {
        self.advance(1);
    }

    fn advance(&mut self, n: usize) {
        match &mut self.src {
            Source::Mem { .. } => {
                self.pos += n;
            }
            Source::Stream { fp, cp, buf } => {
                let mut sb = buf.borrow_mut();
                let drain = n.min(sb.ahead.len());
                sb.ahead.drain(..drain);
                let mut consumed = drain;
                for _ in 0..(n - drain) {
                    if sb.eof {
                        break;
                    }
                    match unsafe { Self::read_stream_unit(self.wide, *fp, *cp) } {
                        Some(_) => consumed += 1,
                        None => {
                            sb.eof = true;
                            break;
                        }
                    }
                }
                self.pos += consumed;
            }
        }
    }

    /// Push any unconsumed lookahead units back onto the backing `FILE*` stream
    /// in reverse order so subsequent reads see the exact next unconsumed unit.
    /// Leaves `feof(fp)` intact when lookahead is empty.
    unsafe fn flush_unget(&mut self) {
        if let Source::Stream { fp, cp, buf } = &self.src {
            let mut sb = buf.borrow_mut();
            for c in sb.ahead.drain(..).rev() {
                if self.wide {
                    let _ = super::stdio::unget_stream_wide_cp(c, *fp, *cp);
                } else {
                    let _ = super::stdio::ungetc(c as i32, *fp);
                }
            }
        }
    }
}

fn is_space(c: u32) -> bool {
    c == 0x20 || (0x09..=0x0D).contains(&c)
}

unsafe fn fmt_at(fmt: *const u8, wide: bool, i: usize) -> u32 {
    if wide {
        (fmt as *const u16).add(i).read_unaligned() as u32
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
            match input.peek() {
                Some(c) if c == '%' as u32 => {
                    input.bump();
                    i += 1;
                    continue;
                }
                None if !converted_any => return -1,
                _ => break,
            }
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
                    if p.is_null() {
                        super::errno::set(super::errno::EINVAL);
                        super::runtime::invalid_parameter("scanf: NULL output");
                        return -1;
                    }
                    let sz = if conv == b'p' { 8 } else { size };
                    match sz {
                        1 => p.write_unaligned(v as u8),
                        2 => (p as *mut u16).write_unaligned(v as u16),
                        8 => (p as *mut u64).write_unaligned(v),
                        _ => (p as *mut u32).write_unaligned(v as u32),
                    }
                    assigned += 1;
                }
            }
            b'e' | b'E' | b'f' | b'F' | b'g' | b'G' | b'a' | b'A' => {
                let inp = &*input;
                let lim = maxw;
                let get = |k: usize| -> u32 {
                    if k >= lim {
                        return 0;
                    }
                    inp.peek_at(k).unwrap_or(0)
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
                input.advance(used);
                converted_any = true;
                if !suppress {
                    let p = args.next_ptr::<u8>();
                    if p.is_null() {
                        super::errno::set(super::errno::EINVAL);
                        super::runtime::invalid_parameter("scanf: NULL output");
                        return -1;
                    }
                    if is_double {
                        (p as *mut f64).write_unaligned(v);
                    } else {
                        (p as *mut f32).write_unaligned(v as f32);
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
                let cp = super::locale::locale_obj_cp(loc);
                let mut units: Vec<u32> = Vec::new();
                let mut conv_failed = false;
                let char_ok = |c: u32| -> bool {
                    match lower {
                        b's' => !is_space(c),
                        b'c' => true,
                        _ => set.iter().any(|&(a, b)| c >= a && c <= b) != negate,
                    }
                };
                match (wide, is_wide) {
                    (false, true) => {
                        // Narrow input -> wide UTF-16 destination: bound by destination UTF-16
                        // units (`maxc`) so supplementary characters never overflow `%lc`/`%Nlc`.
                        while units.len() < maxc {
                            let Some(b0) = input.peek() else { break };
                            if !char_ok(b0) {
                                break;
                            }
                            let need_bytes = if cp == 0 {
                                1usize
                            } else if cp == 65001 {
                                match b0 as u8 {
                                    0x00..=0x7F => 1,
                                    0xC2..=0xDF => 2,
                                    0xE0..=0xEF => 3,
                                    0xF0..=0xF4 => 4,
                                    _ => {
                                        super::errno::set(super::errno::EILSEQ);
                                        conv_failed = true;
                                        break;
                                    }
                                }
                            } else if crate::kernel32::string::IsDBCSLeadByteEx(cp, b0 as u8) != 0 {
                                2
                            } else {
                                1
                            };
                            let mut seq = [b0 as u8, 0, 0, 0];
                            let mut complete = true;
                            for k in 1..need_bytes {
                                match input.peek_at(k) {
                                    Some(bk) => seq[k] = bk as u8,
                                    None => {
                                        complete = false;
                                        break;
                                    }
                                }
                            }
                            if !complete {
                                super::errno::set(super::errno::EILSEQ);
                                conv_failed = true;
                                break;
                            }
                            match super::convert::mb_decode_cp(cp, &seq[..need_bytes]) {
                                Ok(Some((w, used))) => {
                                    if units.len() + w.len() > maxc {
                                        if lower == b'c' || units.is_empty() {
                                            super::errno::set(super::errno::EILSEQ);
                                            conv_failed = true;
                                        }
                                        break;
                                    }
                                    units.extend(w.into_iter().map(|u| u as u32));
                                    input.advance(used);
                                }
                                Ok(None) | Err(()) => {
                                    super::errno::set(super::errno::EILSEQ);
                                    conv_failed = true;
                                    break;
                                }
                            }
                        }
                    }
                    (true, false) => {
                        // Wide UTF-16 input -> narrow byte destination: bound by destination
                        // byte count (`maxc`) so multibyte encodings never overflow `%hc`/`%Nhc`.
                        while units.len() < maxc {
                            let Some(u0) = input.peek() else { break };
                            if !char_ok(u0) {
                                break;
                            }
                            let u0_16 = u0 as u16;
                            let (enc, step) = if (0xD800..=0xDBFF).contains(&u0_16) {
                                match input.peek_at(1) {
                                    Some(u1) if (0xDC00..=0xDFFF).contains(&(u1 as u16)) => {
                                        (super::convert::wc_encode_slice_cp(cp, &[u0_16, u1 as u16]), 2)
                                    }
                                    _ => (super::convert::wc_encode_cp(cp, u0_16), 1),
                                }
                            } else {
                                (super::convert::wc_encode_cp(cp, u0_16), 1)
                            };
                            match enc {
                                Ok(bytes) => {
                                    if units.len() + bytes.len() > maxc {
                                        if lower == b'c' || units.is_empty() {
                                            super::errno::set(super::errno::EILSEQ);
                                            conv_failed = true;
                                        }
                                        break;
                                    }
                                    units.extend(bytes.into_iter().map(|b| b as u32));
                                    input.advance(step);
                                }
                                Err(()) => {
                                    super::errno::set(super::errno::EILSEQ);
                                    conv_failed = true;
                                    break;
                                }
                            }
                        }
                    }
                    _ => {
                        while units.len() < maxc {
                            let Some(c) = input.peek() else { break };
                            if !char_ok(c) {
                                break;
                            }
                            units.push(c);
                            input.bump();
                        }
                    }
                }
                if conv_failed || units.is_empty() {
                    break;
                }
                converted_any = true;
                if !suppress {
                    let p = args.next_ptr::<u8>();
                    let cap = if opts & SECURE != 0 { args.next_u64() as u32 as usize } else { usize::MAX };
                    if p.is_null() {
                        super::errno::set(super::errno::EINVAL);
                        super::runtime::invalid_parameter("scanf: NULL output");
                        return -1;
                    }
                    let need = units.len() + if lower == b'c' { 0 } else { 1 };
                    if need > cap {
                        if cap > 0 {
                            if is_wide {
                                (p as *mut u16).write_unaligned(0);
                            } else {
                                *p = 0;
                            }
                        }
                        super::errno::set(super::errno::ENOMEM);
                        break;
                    }
                    for (k, &u) in units.iter().enumerate() {
                        if is_wide {
                            (p as *mut u16).add(k).write_unaligned(u as u16);
                        } else {
                            *p.add(k) = u as u8;
                        }
                    }
                    if lower != b'c' {
                        if is_wide {
                            (p as *mut u16).add(units.len()).write_unaligned(0);
                        } else {
                            *p.add(units.len()) = 0;
                        }
                    }
                    assigned += 1;
                }
            }
            b'n' => {
                if !suppress {
                    let p = args.next_ptr::<u8>();
                    if p.is_null() {
                        super::errno::set(super::errno::EINVAL);
                        super::runtime::invalid_parameter("scanf: NULL output");
                        return -1;
                    }
                    match size {
                        1 => p.write_unaligned(input.pos as u8),
                        2 => (p as *mut u16).write_unaligned(input.pos as u16),
                        8 => (p as *mut u64).write_unaligned(input.pos as u64),
                        _ => (p as *mut u32).write_unaligned(input.pos as u32),
                    }
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
    let mut input = Input::from_mem(false, buf, count);
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
    let mut input = Input::from_mem(true, buf as *const u8, count);
    let mut a = VaList::new(va);
    scan(&mut input, fmt as *const u8, opts, loc, &mut a)
}

pub unsafe extern "win64" fn __stdio_common_vfscanf(
    opts: u64,
    fp: *mut super::stdio::FILE,
    fmt: *const u8,
    loc: *mut c_void,
    va: *const u8,
) -> i32 {
    if fp.is_null() || fmt.is_null() {
        super::errno::set(super::errno::EINVAL);
        super::runtime::invalid_parameter("fscanf: NULL");
        return -1;
    }
    let mut input = Input::from_stream(false, fp, super::locale::locale_obj_cp(loc));
    let mut a = VaList::new(va);
    let r = scan(&mut input, fmt, opts, loc, &mut a);
    input.flush_unget();
    r
}

pub unsafe extern "win64" fn __stdio_common_vfwscanf(
    opts: u64,
    fp: *mut super::stdio::FILE,
    fmt: *const u16,
    loc: *mut c_void,
    va: *const u8,
) -> i32 {
    if fp.is_null() || fmt.is_null() {
        super::errno::set(super::errno::EINVAL);
        super::runtime::invalid_parameter("fwscanf: NULL");
        return -1;
    }
    let mut input = Input::from_stream(true, fp, super::locale::locale_obj_cp(loc));
    let mut a = VaList::new(va);
    let r = scan(&mut input, fmt as *const u8, opts, loc, &mut a);
    input.flush_unget();
    r
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

            // LC_CTYPE = de-DE (CP1252) & ja-JP (CP932) apply to cross-width %ls/%hs/%lc conversions.
            let loc_de_ct = crate::ucrt::locale::_create_locale(2 /* LC_CTYPE */, b"de-DE\0".as_ptr());
            assert!(!loc_de_ct.is_null());
            let mut wbuf = [0u16; 16];
            let cp1252_in: [u8; 3] = [0xE4, 0x80, 0];
            let args_w = [wbuf.as_mut_ptr() as u64];
            assert_eq!(
                __stdio_common_vsscanf(0, cp1252_in.as_ptr(), usize::MAX, b"%ls\0".as_ptr(), loc_de_ct, args_w.as_ptr() as *const u8),
                1
            );
            assert_eq!(&wbuf[..3], &[0x00E4, 0x20AC, 0]);

            let wide_in: Vec<u16> = "ä€\0".encode_utf16().collect();
            let fmt_hs: Vec<u16> = "%hs\0".encode_utf16().collect();
            let mut nbuf = [0u8; 16];
            let args_n = [nbuf.as_mut_ptr() as u64];
            assert_eq!(
                __stdio_common_vswscanf(0, wide_in.as_ptr(), usize::MAX, fmt_hs.as_ptr(), loc_de_ct, args_n.as_ptr() as *const u8),
                1
            );
            assert_eq!(&nbuf[..3], &[0xE4, 0x80, 0]);
            crate::ucrt::locale::_free_locale(loc_de_ct);

            let loc_ja_ct = crate::ucrt::locale::_create_locale(2 /* LC_CTYPE */, b"ja-JP\0".as_ptr());
            assert!(!loc_ja_ct.is_null());
            let mut wc = 0u16;
            let cp932_in: [u8; 3] = [0x82, 0xA0, 0];
            let args_wc = [&mut wc as *mut u16 as u64];
            assert_eq!(
                __stdio_common_vsscanf(0, cp932_in.as_ptr(), usize::MAX, b"%lc\0".as_ptr(), loc_ja_ct, args_wc.as_ptr() as *const u8),
                1
            );
            assert_eq!(wc, 0x3042);
            let wide_ja: Vec<u16> = "あ\0".encode_utf16().collect();
            assert_eq!(
                __stdio_common_vswscanf(0, wide_ja.as_ptr(), usize::MAX, fmt_hs.as_ptr(), loc_ja_ct, args_n.as_ptr() as *const u8),
                1
            );
            assert_eq!(&nbuf[..3], &[0x82, 0xA0, 0]);
            // Wide -> narrow %hc must reject 2-byte CP932 char instead of overflowing 1-byte dest.
            let mut ncan = [0xAAu8, 0xEF];
            let fmt_hc: Vec<u16> = "%hc\0".encode_utf16().collect();
            let fmt_2hc: Vec<u16> = "%2hc\0".encode_utf16().collect();
            let args_ncan = [ncan.as_mut_ptr() as u64];
            crate::ucrt::errno::set(0);
            assert_eq!(
                __stdio_common_vswscanf(0, wide_ja.as_ptr(), usize::MAX, fmt_hc.as_ptr(), loc_ja_ct, args_ncan.as_ptr() as *const u8),
                0
            );
            assert_eq!(crate::ucrt::errno::get(), crate::ucrt::errno::EILSEQ);
            assert_eq!(ncan, [0xAA, 0xEF]);
            let mut ncan2 = [0u8, 0, 0xEF];
            let args_ncan2 = [ncan2.as_mut_ptr() as u64];
            assert_eq!(
                __stdio_common_vswscanf(0, wide_ja.as_ptr(), usize::MAX, fmt_2hc.as_ptr(), loc_ja_ct, args_ncan2.as_ptr() as *const u8),
                1
            );
            assert_eq!(ncan2, [0x82, 0xA0, 0xEF]);
            crate::ucrt::locale::_free_locale(loc_ja_ct);

            // UTF-8 supplementary scalar ("😀" = 2 UTF-16 surrogates) must not overflow %lc (1 wchar_t).
            let loc_u8 = crate::ucrt::locale::_create_locale(2 /* LC_CTYPE */, b"de-DE.utf8\0".as_ptr());
            assert!(!loc_u8.is_null());
            let emoji = "😀\0";
            let mut wcan = [0xAAAAu16, 0xBEEF];
            let args_wcan = [wcan.as_mut_ptr() as u64];
            crate::ucrt::errno::set(0);
            assert_eq!(
                __stdio_common_vsscanf(0, emoji.as_ptr(), usize::MAX, b"%lc\0".as_ptr(), loc_u8, args_wcan.as_ptr() as *const u8),
                0
            );
            assert_eq!(crate::ucrt::errno::get(), crate::ucrt::errno::EILSEQ);
            assert_eq!(wcan, [0xAAAA, 0xBEEF]);

            // %2lc fits the 2 UTF-16 surrogate units cleanly without touching the trailing canary.
            let mut wcan2 = [0u16, 0, 0xBEEF];
            let args_wcan2 = [wcan2.as_mut_ptr() as u64];
            assert_eq!(
                __stdio_common_vsscanf(0, emoji.as_ptr(), usize::MAX, b"%2lc\0".as_ptr(), loc_u8, args_wcan2.as_ptr() as *const u8),
                1
            );
            assert_eq!(wcan2, [0xD83D, 0xDE00, 0xBEEF]);

            // %2lc on "a😀" rejects instead of overwriting past 2 wchar_t slots.
            let a_emoji = "a😀\0";
            let mut wcan3 = [0xAAAAu16, 0xBBBB, 0xBEEF];
            let args_wcan3 = [wcan3.as_mut_ptr() as u64];
            crate::ucrt::errno::set(0);
            assert_eq!(
                __stdio_common_vsscanf(0, a_emoji.as_ptr(), usize::MAX, b"%2lc\0".as_ptr(), loc_u8, args_wcan3.as_ptr() as *const u8),
                0
            );
            assert_eq!(crate::ucrt::errno::get(), crate::ucrt::errno::EILSEQ);
            assert_eq!(wcan3, [0xAAAA, 0xBBBB, 0xBEEF]);

            // %2ls on "a😀" stores only L"a\0" (1 unit <= 2) and leaves "😀" unconsumed.
            let mut wcan4 = [0xAAAAu16, 0xBBBB, 0xCCCC, 0xBEEF];
            let args_wcan4 = [wcan4.as_mut_ptr() as u64];
            assert_eq!(
                __stdio_common_vsscanf(0, a_emoji.as_ptr(), usize::MAX, b"%2ls\0".as_ptr(), loc_u8, args_wcan4.as_ptr() as *const u8),
                1
            );
            assert_eq!(&wcan4[..2], &[b'a' as u16, 0]);
            assert_eq!(wcan4[3], 0xBEEF);

            // %1ls on "😀" fails cleanly without overflowing.
            let mut wcan5 = [0xAAAAu16, 0xBEEF];
            let args_wcan5 = [wcan5.as_mut_ptr() as u64];
            crate::ucrt::errno::set(0);
            assert_eq!(
                __stdio_common_vsscanf(0, emoji.as_ptr(), usize::MAX, b"%1ls\0".as_ptr(), loc_u8, args_wcan5.as_ptr() as *const u8),
                0
            );
            assert_eq!(crate::ucrt::errno::get(), crate::ucrt::errno::EILSEQ);
            assert_eq!(wcan5, [0xAAAA, 0xBEEF]);
            crate::ucrt::locale::_free_locale(loc_u8);
        }
    }

    #[test]
    fn vfscanf_stream_roundtrip_and_pushback() {
        unsafe {
            use crate::ucrt::stdio;

            // Helper: populate a temporary binary stream and rewind to 0.
            let make_stream = |bytes: &[u8]| -> *mut stdio::FILE {
                let fp = stdio::tmpfile();
                assert!(!fp.is_null());
                if !bytes.is_empty() {
                    let nw = stdio::fwrite(bytes.as_ptr().cast(), 1, bytes.len(), fp);
                    assert_eq!(nw, bytes.len());
                }
                stdio::rewind(fp);
                fp
            };

            // Install a scoped no-op invalid-parameter handler so NULL-arg checks return -1 + EINVAL
            // rather than fast-failing the test process.
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
            let _iph = IphGuard(crate::ucrt::runtime::_set_invalid_parameter_handler(
                noop_iph as *const () as usize,
            ));

            // Null stream or null format returns EOF (-1) and sets EINVAL.
            let dummy_args = [0u64; 1];
            crate::ucrt::errno::set(0);
            assert_eq!(
                __stdio_common_vfscanf(
                    0,
                    std::ptr::null_mut(),
                    b"%d\0".as_ptr(),
                    std::ptr::null_mut(),
                    dummy_args.as_ptr() as *const u8,
                ),
                -1
            );
            assert_eq!(crate::ucrt::errno::get(), crate::ucrt::errno::EINVAL);

            let fp = make_stream(b"  -42 0x2A 3.5 token [abc]123rest");
            crate::ucrt::errno::set(0);
            assert_eq!(
                __stdio_common_vfscanf(
                    0,
                    fp,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    dummy_args.as_ptr() as *const u8,
                ),
                -1
            );
            assert_eq!(crate::ucrt::errno::get(), crate::ucrt::errno::EINVAL);

            // Null output pointer returns -1 + EINVAL without crashing.
            let fp_null_out = make_stream(b"123");
            crate::ucrt::errno::set(0);
            assert_eq!(
                __stdio_common_vfscanf(
                    0,
                    fp_null_out,
                    b"%d\0".as_ptr(),
                    std::ptr::null_mut(),
                    dummy_args.as_ptr() as *const u8,
                ),
                -1
            );
            assert_eq!(crate::ucrt::errno::get(), crate::ucrt::errno::EINVAL);
            stdio::fclose(fp_null_out);

            // Scan into unaligned output buffers with SECURE flag + %n + %[ set.
            let mut raw = [0u8; 64];
            let p_int = raw.as_mut_ptr().add(1);
            let p_hex = raw.as_mut_ptr().add(7);
            let p_dbl = raw.as_mut_ptr().add(17);
            let p_str = raw.as_mut_ptr().add(27);
            let p_set = raw.as_mut_ptr().add(40);
            let p_n = raw.as_mut_ptr().add(49);
            assert!(!(p_int as *const i32).is_aligned());
            assert!(!(p_hex as *const u64).is_aligned());
            assert!(!(p_dbl as *const f64).is_aligned());
            assert!(!(p_n as *const i16).is_aligned());

            let args: [u64; 8] = [
                p_int as u64,
                p_hex as u64,
                p_dbl as u64,
                p_str as u64,
                12,
                p_set as u64,
                8,
                p_n as u64,
            ];
            let n = __stdio_common_vfscanf(
                SECURE,
                fp,
                b"%d %llx %lf %s [%[a-z]]%hn\0".as_ptr(),
                std::ptr::null_mut(),
                args.as_ptr() as *const u8,
            );
            assert_eq!(n, 5);
            assert_eq!((p_int as *const i32).read_unaligned(), -42);
            assert_eq!((p_hex as *const u64).read_unaligned(), 0x2A);
            assert_eq!((p_dbl as *const f64).read_unaligned(), 3.5);
            assert_eq!(std::slice::from_raw_parts(p_str, 6), b"token\0");
            assert_eq!(std::slice::from_raw_parts(p_set, 4), b"abc\0");
            assert_eq!((p_n as *const i16).read_unaligned(), 26);
            // Next unconsumed byte after ']' is '1', followed by "23rest".
            let mut tail = [0u8; 7];
            for b in &mut tail {
                let c = stdio::fgetc(fp);
                assert!(c >= 0);
                *b = c as u8;
            }
            assert_eq!(&tail, b"123rest");
            assert_eq!(stdio::fgetc(fp), -1);
            assert_ne!(stdio::feof(fp), 0);
            stdio::fclose(fp);

            // Multi-byte float lookahead backtrack ("1.5e+x") must push back "e+x" in exact order.
            let fp_bt = make_stream(b"1.5e+x");
            let mut fv: f64 = 0.0;
            let args_bt = [&mut fv as *mut f64 as u64];
            assert_eq!(
                __stdio_common_vfscanf(
                    0,
                    fp_bt,
                    b"%lf\0".as_ptr(),
                    std::ptr::null_mut(),
                    args_bt.as_ptr() as *const u8,
                ),
                1
            );
            assert_eq!(fv, 1.5);
            assert_eq!(stdio::fgetc(fp_bt), b'e' as i32);
            assert_eq!(stdio::fgetc(fp_bt), b'+' as i32);
            assert_eq!(stdio::fgetc(fp_bt), b'x' as i32);
            assert_eq!(stdio::fgetc(fp_bt), -1);
            stdio::fclose(fp_bt);

            // Matching failure returns 0 (not EOF) and leaves the offending byte in the stream.
            let fp_mf = make_stream(b"xyz");
            let mut iv = 777i32;
            let args_mf = [&mut iv as *mut i32 as u64];
            assert_eq!(
                __stdio_common_vfscanf(
                    0,
                    fp_mf,
                    b"%d\0".as_ptr(),
                    std::ptr::null_mut(),
                    args_mf.as_ptr() as *const u8,
                ),
                0
            );
            assert_eq!(iv, 777);
            assert_eq!(stdio::fgetc(fp_mf), b'x' as i32);
            stdio::fclose(fp_mf);

            // Reading to EOF without trailing bytes preserves feof(fp) == 1; empty/EOF returns -1.
            let fp_eof = make_stream(b"99");
            let mut iv2 = 0i32;
            let args_eof = [&mut iv2 as *mut i32 as u64];
            assert_eq!(
                __stdio_common_vfscanf(
                    0,
                    fp_eof,
                    b"%d\0".as_ptr(),
                    std::ptr::null_mut(),
                    args_eof.as_ptr() as *const u8,
                ),
                1
            );
            assert_eq!(iv2, 99);
            assert_ne!(stdio::feof(fp_eof), 0);
            assert_eq!(
                __stdio_common_vfscanf(
                    0,
                    fp_eof,
                    b"%d\0".as_ptr(),
                    std::ptr::null_mut(),
                    args_eof.as_ptr() as *const u8,
                ),
                -1
            );
            assert_eq!(
                __stdio_common_vfscanf(
                    0,
                    fp_eof,
                    b"%%\0".as_ptr(),
                    std::ptr::null_mut(),
                    args_eof.as_ptr() as *const u8,
                ),
                -1
            );
            stdio::fclose(fp_eof);

            // Explicit LC_NUMERIC = de-DE uses ',' decimal separator on FILE* streams.
            let loc_de = crate::ucrt::locale::_create_locale(4 /* LC_NUMERIC */, b"de-DE\0".as_ptr());
            assert!(!loc_de.is_null());
            let fp_de = make_stream(b"2,25!");
            let mut df = 0.0f64;
            let args_de = [&mut df as *mut f64 as u64];
            assert_eq!(
                __stdio_common_vfscanf(
                    0,
                    fp_de,
                    b"%lf\0".as_ptr(),
                    loc_de,
                    args_de.as_ptr() as *const u8,
                ),
                1
            );
            assert_eq!(df, 2.25);
            assert_eq!(stdio::fgetc(fp_de), b'!' as i32);
            stdio::fclose(fp_de);
            crate::ucrt::locale::_free_locale(loc_de);

            // Verify API-set resolution for both new UCRT exports.
            assert!(crate::lookup("api-ms-win-crt-stdio-l1-1-0.dll", "__stdio_common_vfscanf").is_some());
            assert!(crate::lookup("api-ms-win-crt-stdio-l1-1-0.dll", "__stdio_common_vfwscanf").is_some());
        }
    }

    #[test]
    fn vfwscanf_stream_roundtrip_unicode_and_locale() {
        unsafe {
            use crate::ucrt::stdio;

            // Narrow CP1252 file scanned via __stdio_common_vfwscanf with explicit de-DE _locale_t.
            let loc_de_ct = crate::ucrt::locale::_create_locale(2 /* LC_CTYPE */, b"de-DE\0".as_ptr());
            assert!(!loc_de_ct.is_null());
            let fp_cp1252 = stdio::tmpfile();
            assert!(!fp_cp1252.is_null());
            let raw_1252: &[u8] = &[b'7', b' ', 0xE4, 0x80, b' ', b'Z']; // "7 ä€ Z" in CP1252
            assert_eq!(stdio::fwrite(raw_1252.as_ptr().cast(), 1, raw_1252.len(), fp_cp1252), raw_1252.len());
            stdio::rewind(fp_cp1252);

            // Unaligned wide format string (odd byte offset).
            let fmt_units: Vec<u16> = "%d %ls\0".encode_utf16().collect();
            let mut raw_fmt = vec![0xAAu8; fmt_units.len() * 2 + 3];
            let odd_fmt = raw_fmt.as_mut_ptr().add(1) as *mut u16;
            assert!(!odd_fmt.is_aligned());
            for (i, &u) in fmt_units.iter().enumerate() {
                odd_fmt.add(i).write_unaligned(u);
            }

            let mut iv = 0i32;
            let mut wbuf = [0u16; 8];
            let args: [u64; 3] = [&mut iv as *mut i32 as u64, wbuf.as_mut_ptr() as u64, 8];
            assert_eq!(
                __stdio_common_vfwscanf(
                    SECURE,
                    fp_cp1252,
                    odd_fmt,
                    loc_de_ct,
                    args.as_ptr() as *const u8,
                ),
                2
            );
            assert_eq!(iv, 7);
            assert_eq!(&wbuf[..3], &[0x00E4, 0x20AC, 0]);
            // Lookahead space before 'Z' was pushed back cleanly.
            assert_eq!(stdio::fgetc(fp_cp1252), b' ' as i32);
            assert_eq!(stdio::fgetc(fp_cp1252), b'Z' as i32);
            stdio::fclose(fp_cp1252);
            crate::ucrt::locale::_free_locale(loc_de_ct);

            // Unicode (_O_U16TEXT) stream with lookahead backtrack pushes back UTF-16LE units cleanly.
            let fp_u16 = stdio::tmpfile();
            assert!(!fp_u16.is_null());
            let u16_units: Vec<u16> = "1.25e+Q".encode_utf16().collect();
            let mut le_bytes = Vec::with_capacity(u16_units.len() * 2);
            for u in &u16_units {
                le_bytes.extend_from_slice(&u.to_le_bytes());
            }
            assert_eq!(stdio::fwrite(le_bytes.as_ptr().cast(), 1, le_bytes.len(), fp_u16), le_bytes.len());
            stdio::rewind(fp_u16);
            let fd = stdio::_fileno(fp_u16);
            assert!(stdio::_setmode(fd, 0x0002_0000 /* _O_U16TEXT */) >= 0);

            let fmt_f: Vec<u16> = "%lf\0".encode_utf16().collect();
            let mut fv = 0.0f64;
            let args_f = [&mut fv as *mut f64 as u64];
            assert_eq!(
                __stdio_common_vfwscanf(
                    0,
                    fp_u16,
                    fmt_f.as_ptr(),
                    std::ptr::null_mut(),
                    args_f.as_ptr() as *const u8,
                ),
                1
            );
            assert_eq!(fv, 1.25);
            assert_eq!(stdio::fgetwc(fp_u16), b'e' as u32);
            assert_eq!(stdio::fgetwc(fp_u16), b'+' as u32);
            assert_eq!(stdio::fgetwc(fp_u16), b'Q' as u32);
            assert_eq!(stdio::fgetwc(fp_u16), WEOF);
            stdio::fclose(fp_u16);
        }
    }
}
