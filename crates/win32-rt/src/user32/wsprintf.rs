//! USER32's bounded integer/string formatter. It has its own format language:
//! for example, an unrecognized conversion emits the character and consumes no argument.

use crate::va::{win64_varargs_thunk, VaList};

const CAPACITY: usize = 1024;

#[derive(Clone, Copy)]
struct Text {
    ptr: *const u8,
    wide: bool,
}

impl Text {
    unsafe fn unit(self, i: usize) -> u16 {
        if self.wide {
            self.ptr.cast::<u16>().add(i).read_unaligned()
        } else {
            self.ptr.add(i).read().into()
        }
    }

    unsafe fn decimal(self, i: &mut usize) -> usize {
        let mut n = 0;
        while let Some(d @ 0..=9) = self.unit(*i).checked_sub(b'0' as u16) {
            n = (n * 10 + d as usize).min(CAPACITY);
            *i += 1;
        }
        n
    }
}

unsafe fn text_field(src: Text, out_wide: bool, limit: usize, single: bool) -> Vec<u16> {
    if src.ptr.is_null() {
        return "(null)".encode_utf16().take(limit).collect();
    }
    let mut result = Vec::new();
    let mut i = 0;
    while result.len() < limit {
        let ch = src.unit(i);
        if ch == 0 && !single {
            break;
        }
        i += 1;
        if src.wide == out_wide {
            result.push(ch);
        } else if src.wide {
            let codepoint = if !single
                && (0xd800..=0xdbff).contains(&ch)
                && (0xdc00..=0xdfff).contains(&src.unit(i))
            {
                let low = src.unit(i);
                i += 1;
                0x10000 + ((u32::from(ch) - 0xd800) << 10) + u32::from(low) - 0xdc00
            } else {
                u32::from(ch)
            };
            let c = char::from_u32(codepoint).unwrap_or(char::REPLACEMENT_CHARACTER);
            result.extend(c.encode_utf8(&mut [0; 4]).bytes().map(u16::from));
        } else {
            // The process ANSI code page is UTF-8. Decode one scalar at a time so
            // a precision-limited string need not have a NUL beyond the requested field.
            let mut bytes = [ch as u8, 0, 0, 0];
            let mut n = 1;
            let width = match bytes[0] {
                0xc2..=0xdf => 2,
                0xe0..=0xef => 3,
                0xf0..=0xf4 => 4,
                _ => 1,
            };
            while !single && n < width && src.unit(i) & 0xc0 == 0x80 {
                bytes[n] = src.unit(i) as u8;
                n += 1;
                i += 1;
            }
            let c = std::str::from_utf8(&bytes[..n])
                .ok()
                .and_then(|s| s.chars().next())
                .unwrap_or(char::REPLACEMENT_CHARACTER);
            result.extend_from_slice(c.encode_utf16(&mut [0; 2]));
        }
        if single {
            break;
        }
    }
    result.truncate(limit);
    result
}

unsafe fn format(slots: *const u64, wide: bool) -> i32 {
    let mut args = VaList::new(slots.cast());
    let dest = args.next_ptr::<u8>();
    let fmt = Text {
        ptr: args.next_ptr(),
        wide,
    };
    if dest.is_null() || fmt.ptr.is_null() {
        return 0;
    }

    let mut out = Vec::<u16>::new();
    let mut i = 0;
    while out.len() < CAPACITY {
        let unit = fmt.unit(i);
        if unit == 0 {
            break;
        }
        i += 1;
        if unit != b'%' as u16 {
            out.push(unit);
            continue;
        }
        let mut left = false;
        let mut prefix = false;
        let mut zero = false;
        loop {
            match u8::try_from(fmt.unit(i)) {
                Ok(b'-') => left = true,
                Ok(b'#') => prefix = true,
                Ok(b'0') => zero = true,
                _ => break,
            }
            i += 1;
        }
        let width = fmt.decimal(&mut i);
        let precision = if fmt.unit(i) == b'.' as u16 {
            i += 1;
            Some(fmt.decimal(&mut i))
        } else {
            None
        };
        let mut size = 4;
        let mut wide_arg = None;
        match u8::try_from(fmt.unit(i)) {
            Ok(b'h') => {
                size = 2;
                wide_arg = Some(false);
                i += 1;
            }
            Ok(b'l' | b'w') => {
                wide_arg = Some(true);
                i += 1;
            }
            Ok(b'I') => {
                i += 1;
                size = 8;
                if fmt.unit(i) == b'3' as u16 && fmt.unit(i + 1) == b'2' as u16 {
                    size = 4;
                    i += 2;
                } else if fmt.unit(i) == b'6' as u16 && fmt.unit(i + 1) == b'4' as u16 {
                    i += 2;
                }
            }
            _ => {}
        }
        let conv = fmt.unit(i);
        if conv == 0 {
            break;
        }
        i += 1;
        let remaining = CAPACITY - out.len();
        let field = match u8::try_from(conv) {
            Ok(c @ (b's' | b'S' | b'c' | b'C')) => {
                let single = c.eq_ignore_ascii_case(&b'c');
                let arg = args.next_u64();
                let src = Text {
                    ptr: if single {
                        (&arg as *const u64).cast()
                    } else {
                        arg as *const u8
                    },
                    wide: wide_arg.unwrap_or(wide != c.is_ascii_uppercase()),
                };
                if single && wide_arg.is_some() && src.unit(0) == 0 {
                    Vec::new()
                } else {
                    let limit = if single {
                        remaining
                    } else {
                        precision.unwrap_or(remaining).min(remaining)
                    };
                    text_field(src, wide, limit, single)
                }
            }
            Ok(c @ (b'd' | b'i' | b'u' | b'x' | b'X' | b'p')) => {
                let raw = args.next_u64();
                let signed = matches!(c, b'd' | b'i');
                let (signed_val, unsigned_val) = match (size, c) {
                    (_, b'p') | (8, _) => (raw as i64, raw),
                    (2, _) => (raw as i16 as i64, raw as u16 as u64),
                    _ => (raw as i32 as i64, raw as u32 as u64),
                };
                let digits = match c {
                    b'X' | b'p' => format!("{unsigned_val:X}"),
                    b'x' => format!("{unsigned_val:x}"),
                    _ if signed => signed_val.unsigned_abs().to_string(),
                    _ => unsigned_val.to_string(),
                };
                let sign = match c {
                    b'x' if prefix => "0x",
                    b'X' if prefix => "0X",
                    _ if signed && signed_val < 0 => "-",
                    _ => "",
                };
                let min_digits = if c == b'p' { 16 } else { 1 };
                let mut digits_width = precision.unwrap_or(0).max(min_digits);
                if zero && !left {
                    digits_width = digits_width.max(width.saturating_sub(sign.len()));
                }
                let mut field: Vec<u16> = sign.encode_utf16().collect();
                field.extend(std::iter::repeat_n(
                    b'0' as u16,
                    digits_width.saturating_sub(digits.len()),
                ));
                field.extend(digits.encode_utf16());
                field
            }
            _ => {
                out.push(conv);
                continue;
            }
        };
        let pad = width.saturating_sub(field.len());
        if !left {
            out.extend(std::iter::repeat_n(b' ' as u16, pad.min(remaining)));
        }
        out.extend(field.into_iter().take(CAPACITY - out.len()));
        if left {
            out.extend(std::iter::repeat_n(
                b' ' as u16,
                pad.min(CAPACITY - out.len()),
            ));
        }
    }

    let length = out.len();
    out.truncate(CAPACITY - 1);
    out.push(0);
    for (index, unit) in out.into_iter().enumerate() {
        if wide {
            dest.cast::<u16>().add(index).write_unaligned(unit);
        } else {
            dest.add(index).write(unit as u8);
        }
    }
    length as i32
}

unsafe extern "win64" fn narrow(args: *const u64) -> i32 {
    format(args, false)
}
unsafe extern "win64" fn wide(args: *const u64) -> i32 {
    format(args, true)
}

win64_varargs_thunk!(wsprintfA => narrow);
win64_varargs_thunk!(wsprintfW => wide);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unaligned_utf16_preserves_code_units_and_terminates() {
        let mut dest = [0xffu8; 10];
        let fmt = [0, b'%', 0, b's', 0, 0, 0];
        // Preserve even an unpaired surrogate when both buffers are UTF-16.
        let arg = [0, 0x12u8, 0xd8, 0, 0];
        unsafe {
            let slots = [
                dest.as_mut_ptr().add(1) as u64,
                fmt.as_ptr().add(1) as u64,
                arg.as_ptr().add(1) as u64,
            ];
            assert_eq!(wide(slots.as_ptr()), 1);
        }
        assert_eq!(dest, [0xff, 0x12, 0xd8, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn unknown_conversion_leaves_argument_for_next_field() {
        let mut dest = [0xffu8; 8];
        let slots = [dest.as_mut_ptr() as u64, c"%f:%i".as_ptr() as u64, 13];
        assert_eq!(unsafe { narrow(slots.as_ptr()) }, 4);
        assert_eq!(&dest[..5], b"f:13\0");
    }
}
