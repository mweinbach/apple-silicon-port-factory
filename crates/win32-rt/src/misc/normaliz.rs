//! normaliz.dll: full RFC 3492 Punycode (`xn--...`) and IDNA conversion
//! (`IdnToAscii`, `IdnToUnicode`, `IdnToNameprepUnicode`).

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_INVALID_PARAMETER: u32 = 87;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const ERROR_INVALID_NAME: u32 = 123;
const ERROR_NO_UNICODE_TRANSLATION: u32 = 1113;

const IDN_ALLOW_UNASSIGNED: u32 = 0x01;
const IDN_USE_STD3_ASCII_RULES: u32 = 0x02;
const IDN_EMAIL_ADDRESS: u32 = 0x04;
const IDN_RAW_PUNYCODE: u32 = 0x08;
const IDN_KNOWN_FLAGS: u32 =
    IDN_ALLOW_UNASSIGNED | IDN_USE_STD3_ASCII_RULES | IDN_EMAIL_ADDRESS | IDN_RAW_PUNYCODE;

// RFC 3492 Section 5 Bootstring parameters for Punycode
const BASE: u32 = 36;
const TMIN: u32 = 1;
const TMAX: u32 = 26;
const SKEW: u32 = 38;
const DAMP: u32 = 700;
const INITIAL_BIAS: u32 = 72;
const INITIAL_N: u32 = 128;

fn adapt(mut delta: u32, num_points: u32, first_time: bool) -> u32 {
    delta = if first_time { delta / DAMP } else { delta >> 1 };
    delta += delta / num_points;
    let mut k = 0u32;
    while delta > ((BASE - TMIN) * TMAX) / 2 {
        delta /= BASE - TMIN;
        k += BASE;
    }
    k + (((BASE - TMIN + 1) * delta) / (delta + SKEW))
}

fn encode_digit(d: u32) -> Result<char, u32> {
    match d {
        0..=25 => Ok((b'a' + d as u8) as char),
        26..=35 => Ok((b'0' + (d - 26) as u8) as char),
        _ => Err(ERROR_INVALID_NAME),
    }
}

fn decode_digit(b: u8) -> Result<u32, u32> {
    match b {
        b'a'..=b'z' => Ok((b - b'a') as u32),
        b'A'..=b'Z' => Ok((b - b'A') as u32),
        b'0'..=b'9' => Ok((b - b'0') as u32 + 26),
        _ => Err(ERROR_INVALID_NAME),
    }
}

/// RFC 3492 Section 6.3 Punycode encoder.
fn punycode_encode(input: &[char]) -> Result<String, u32> {
    let mut output = String::with_capacity(input.len() * 2);
    let mut n = INITIAL_N;
    let mut delta: u32 = 0;
    let mut bias = INITIAL_BIAS;

    for &c in input {
        if (c as u32) < 0x80 {
            output.push(c);
        }
    }
    let b = output.len() as u32;
    let mut h = b;
    if b > 0 {
        output.push('-');
    }

    while (h as usize) < input.len() {
        let m = input
            .iter()
            .map(|&c| c as u32)
            .filter(|&cp| cp >= n)
            .min()
            .ok_or(ERROR_INVALID_NAME)?;

        delta = delta
            .checked_add(
                (m - n)
                    .checked_mul(h + 1)
                    .ok_or(ERROR_INVALID_NAME)?,
            )
            .ok_or(ERROR_INVALID_NAME)?;
        n = m;

        for &c in input {
            let cp = c as u32;
            if cp < n {
                delta = delta.checked_add(1).ok_or(ERROR_INVALID_NAME)?;
            } else if cp == n {
                let mut q = delta;
                let mut k = BASE;
                loop {
                    let t = if k <= bias {
                        TMIN
                    } else if k >= bias + TMAX {
                        TMAX
                    } else {
                        k - bias
                    };
                    if q < t {
                        break;
                    }
                    let digit = t + ((q - t) % (BASE - t));
                    output.push(encode_digit(digit)?);
                    q = (q - t) / (BASE - t);
                    k += BASE;
                }
                output.push(encode_digit(q)?);
                bias = adapt(delta, h + 1, h == b);
                delta = 0;
                h += 1;
            }
        }
        delta = delta.checked_add(1).ok_or(ERROR_INVALID_NAME)?;
        n = n.checked_add(1).ok_or(ERROR_INVALID_NAME)?;
    }

    Ok(output)
}

/// RFC 3492 Section 6.2 Punycode decoder.
fn punycode_decode(input: &str) -> Result<String, u32> {
    if !input.is_ascii() {
        return Err(ERROR_INVALID_NAME);
    }
    let (basic, ext) = match input.rfind('-') {
        Some(pos) => (&input[..pos], &input.as_bytes()[pos + 1..]),
        None => ("", input.as_bytes()),
    };
    let mut output: Vec<char> = basic.chars().collect();
    let mut n = INITIAL_N;
    let mut i: u32 = 0;
    let mut bias = INITIAL_BIAS;
    let mut idx = 0usize;

    while idx < ext.len() {
        let oldi = i;
        let mut w: u32 = 1;
        let mut k = BASE;
        loop {
            if idx >= ext.len() {
                return Err(ERROR_INVALID_NAME);
            }
            let digit = decode_digit(ext[idx])?;
            idx += 1;
            i = i
                .checked_add(digit.checked_mul(w).ok_or(ERROR_INVALID_NAME)?)
                .ok_or(ERROR_INVALID_NAME)?;
            let t = if k <= bias {
                TMIN
            } else if k >= bias + TMAX {
                TMAX
            } else {
                k - bias
            };
            if digit < t {
                break;
            }
            w = w.checked_mul(BASE - t).ok_or(ERROR_INVALID_NAME)?;
            k += BASE;
        }
        let out_len_plus_1 = (output.len() as u32)
            .checked_add(1)
            .ok_or(ERROR_INVALID_NAME)?;
        bias = adapt(i - oldi, out_len_plus_1, oldi == 0);
        n = n
            .checked_add(i / out_len_plus_1)
            .ok_or(ERROR_INVALID_NAME)?;
        i %= out_len_plus_1;
        let ch = char::from_u32(n).ok_or(ERROR_INVALID_NAME)?;
        output.insert(i as usize, ch);
        i = i.checked_add(1).ok_or(ERROR_INVALID_NAME)?;
    }

    Ok(output.into_iter().collect())
}

fn is_idna_dot(c: char) -> bool {
    matches!(c, '.' | '\u{3002}' | '\u{FF0E}' | '\u{FF61}')
}

fn is_mapped_to_nothing(c: char) -> bool {
    matches!(
        c as u32,
        0x00AD
            | 0x034F
            | 0x1806
            | 0x180B..=0x180D
            | 0x200B
            | 0x2060
            | 0xFE00..=0xFE0F
            | 0xFEFF
    )
}

fn is_prohibited_char(c: char) -> bool {
    let cp = c as u32;
    matches!(
        cp,
        0x0000..=0x001F
            | 0x007F..=0x009F
            | 0xFDD0..=0xFDEF
            | 0xFFFE..=0xFFFF
            | 0xD800..=0xDFFF
    ) || (cp & 0xFFFE) == 0xFFFE
}

fn nameprep_label(label: &str) -> Result<String, u32> {
    let mut out = String::with_capacity(label.len());
    for c in label.chars() {
        if is_mapped_to_nothing(c) {
            continue;
        }
        if is_prohibited_char(c) {
            return Err(ERROR_INVALID_NAME);
        }
        // Map fullwidth ASCII forms (U+FF01..=U+FF5E) to ASCII and lowercase.
        let mapped = if (0xFF01..=0xFF5E).contains(&(c as u32)) {
            char::from_u32(c as u32 - 0xFEE0).unwrap_or(c)
        } else {
            c
        };
        for lc in mapped.to_lowercase() {
            out.push(lc);
        }
    }
    Ok(out)
}

fn check_std3_ascii(label: &str) -> Result<(), u32> {
    if label.starts_with('-') || label.ends_with('-') {
        return Err(ERROR_INVALID_NAME);
    }
    for b in label.bytes() {
        if b < 0x80 && !(b.is_ascii_alphanumeric() || b == b'-') {
            return Err(ERROR_INVALID_NAME);
        }
    }
    Ok(())
}

fn split_idna_labels(s: &str) -> (Vec<&str>, bool) {
    if s.is_empty() {
        return (Vec::new(), false);
    }
    let trailing_dot = s.chars().last().is_some_and(is_idna_dot);
    let body = if trailing_dot {
        let last_len = s.chars().last().unwrap().len_utf8();
        &s[..s.len() - last_len]
    } else {
        s
    };
    if body.is_empty() {
        return (Vec::new(), trailing_dot);
    }
    let labels: Vec<&str> = body.split(is_idna_dot).collect();
    (labels, trailing_dot)
}

fn convert_domain_to_ascii(domain: &str, flags: u32) -> Result<String, u32> {
    if (flags & IDN_RAW_PUNYCODE) != 0 {
        let chars: Vec<char> = domain.chars().collect();
        return punycode_encode(&chars);
    }
    if domain.is_empty() {
        return Err(ERROR_INVALID_NAME);
    }
    let std3 = (flags & IDN_USE_STD3_ASCII_RULES) != 0;
    let (labels, trailing_dot) = split_idna_labels(domain);
    if labels.is_empty() {
        // Standalone "." is accepted as root label.
        if trailing_dot && domain.chars().count() == 1 {
            return Ok(".".into());
        }
        return Err(ERROR_INVALID_NAME);
    }

    let mut out_labels = Vec::with_capacity(labels.len());
    for label in labels {
        if label.is_empty() {
            return Err(ERROR_INVALID_NAME);
        }
        let encoded = if label.is_ascii() {
            if std3 {
                check_std3_ascii(label)?;
            }
            if label.len() > 63 {
                return Err(ERROR_INVALID_NAME);
            }
            label.to_string()
        } else {
            let prepped = nameprep_label(label)?;
            if prepped.is_empty() {
                return Err(ERROR_INVALID_NAME);
            }
            if std3 {
                check_std3_ascii(&prepped)?;
            }
            if prepped.is_ascii() {
                if prepped.len() > 63 {
                    return Err(ERROR_INVALID_NAME);
                }
                prepped
            } else {
                if prepped
                    .as_bytes()
                    .get(..4)
                    .is_some_and(|p| p.eq_ignore_ascii_case(b"xn--"))
                {
                    return Err(ERROR_INVALID_NAME);
                }
                let chars: Vec<char> = prepped.chars().collect();
                let puny = punycode_encode(&chars)?;
                let ace = format!("xn--{puny}");
                if ace.len() > 63 {
                    return Err(ERROR_INVALID_NAME);
                }
                ace
            }
        };
        out_labels.push(encoded);
    }

    let mut joined = out_labels.join(".");
    if trailing_dot {
        joined.push('.');
    }
    Ok(joined)
}

fn convert_domain_to_unicode(domain: &str, flags: u32) -> Result<String, u32> {
    if (flags & IDN_RAW_PUNYCODE) != 0 {
        return punycode_decode(domain);
    }
    if domain.is_empty() {
        return Err(ERROR_INVALID_NAME);
    }
    let std3 = (flags & IDN_USE_STD3_ASCII_RULES) != 0;
    let (labels, trailing_dot) = split_idna_labels(domain);
    if labels.is_empty() {
        if trailing_dot && domain.chars().count() == 1 {
            return Ok(".".into());
        }
        return Err(ERROR_INVALID_NAME);
    }

    let mut out_labels = Vec::with_capacity(labels.len());
    for label in labels {
        if label.is_empty() {
            return Err(ERROR_INVALID_NAME);
        }
        if label.is_ascii()
            && label
                .as_bytes()
                .get(..4)
                .is_some_and(|p| p.eq_ignore_ascii_case(b"xn--"))
        {
            let body = &label[4..];
            if body.is_empty() {
                return Err(ERROR_INVALID_NAME);
            }
            let decoded = punycode_decode(body)?;
            // RFC 3490 Section 4.2 verification: decoded label must contain non-ASCII
            // and re-encode identically (case-insensitively) to the ACE payload.
            if decoded.is_ascii() {
                return Err(ERROR_INVALID_NAME);
            }
            let prepped = nameprep_label(&decoded)?;
            let reencoded = punycode_encode(&prepped.chars().collect::<Vec<_>>())?;
            if !reencoded.eq_ignore_ascii_case(body) {
                return Err(ERROR_INVALID_NAME);
            }
            if std3 {
                check_std3_ascii(&decoded)?;
            }
            out_labels.push(decoded);
        } else {
            if std3 && label.is_ascii() {
                check_std3_ascii(label)?;
            }
            out_labels.push(label.to_string());
        }
    }

    let mut joined = out_labels.join(".");
    if trailing_dot {
        joined.push('.');
    }
    Ok(joined)
}

fn convert_domain_to_nameprep(domain: &str, flags: u32) -> Result<String, u32> {
    if domain.is_empty() {
        return Err(ERROR_INVALID_NAME);
    }
    let std3 = (flags & IDN_USE_STD3_ASCII_RULES) != 0;
    let (labels, trailing_dot) = split_idna_labels(domain);
    if labels.is_empty() {
        if trailing_dot && domain.chars().count() == 1 {
            return Ok(".".into());
        }
        return Err(ERROR_INVALID_NAME);
    }
    let mut out_labels = Vec::with_capacity(labels.len());
    for label in labels {
        if label.is_empty() {
            return Err(ERROR_INVALID_NAME);
        }
        let prepped = nameprep_label(label)?;
        if prepped.is_empty() {
            return Err(ERROR_INVALID_NAME);
        }
        if std3 {
            check_std3_ascii(&prepped)?;
        }
        out_labels.push(prepped);
    }
    let mut joined = out_labels.join(".");
    if trailing_dot {
        joined.push('.');
    }
    Ok(joined)
}

fn convert_with_email<F>(input: &str, flags: u32, f: F) -> Result<String, u32>
where
    F: Fn(&str, u32) -> Result<String, u32>,
{
    if (flags & IDN_EMAIL_ADDRESS) != 0 {
        if let Some((local, domain)) = input.split_once('@') {
            if local.is_empty() || domain.contains('@') {
                return Err(ERROR_INVALID_NAME);
            }
            let conv_domain = f(domain, flags & !IDN_EMAIL_ADDRESS)?;
            return Ok(format!("{local}@{conv_domain}"));
        }
    }
    f(input, flags)
}

unsafe fn read_utf16_strict(src: *const u16, cch: i32) -> Result<(String, bool), u32> {
    let nul_terminated = cch == -1;
    let mut units = Vec::new();
    if nul_terminated {
        let mut i = 0usize;
        loop {
            let u = std::ptr::read_unaligned(src.add(i));
            if u == 0 {
                break;
            }
            units.push(u);
            i += 1;
        }
    } else {
        units.reserve(cch as usize);
        for i in 0..cch as usize {
            units.push(std::ptr::read_unaligned(src.add(i)));
        }
    }
    let s = String::from_utf16(&units).map_err(|_| ERROR_NO_UNICODE_TRANSLATION)?;
    Ok((s, nul_terminated))
}

unsafe fn idn_dispatch<F>(
    flags: u32,
    src: *const u16,
    cch: i32,
    dst: *mut u16,
    cap: i32,
    convert: F,
) -> i32
where
    F: Fn(&str, u32) -> Result<String, u32>,
{
    if src.is_null() || cch == 0 || cch < -1 || cap < 0 || (flags & !IDN_KNOWN_FLAGS) != 0 {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }
    if cap > 0 && dst.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }

    let (input, nul_terminated) = match read_utf16_strict(src, cch) {
        Ok(v) => v,
        Err(e) => {
            crate::teb::set_last_error(e);
            return 0;
        }
    };

    let converted = if input.is_empty() && nul_terminated {
        String::new()
    } else {
        match convert_with_email(&input, flags, convert) {
            Ok(v) => v,
            Err(e) => {
                crate::teb::set_last_error(e);
                return 0;
            }
        }
    };

    let mut out_units: Vec<u16> = converted.encode_utf16().collect();
    if nul_terminated {
        out_units.push(0);
    }
    let needed = out_units.len();
    if cap == 0 {
        return needed as i32;
    }
    if (cap as usize) < needed {
        crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    for (i, &u) in out_units.iter().enumerate() {
        std::ptr::write_unaligned(dst.add(i), u);
    }
    needed as i32
}

/// `IdnToAscii(dwFlags, lpUnicodeCharStr, cchUnicodeChar, lpASCIICharStr, cchASCIIChar)`.
pub unsafe extern "win64" fn IdnToAscii(
    flags: u32,
    src: *const u16,
    cch: i32,
    dst: *mut u16,
    cap: i32,
) -> i32 {
    idn_dispatch(flags, src, cch, dst, cap, convert_domain_to_ascii)
}

/// `IdnToUnicode(dwFlags, lpASCIICharStr, cchASCIIChar, lpUnicodeCharStr, cchUnicodeChar)`.
pub unsafe extern "win64" fn IdnToUnicode(
    flags: u32,
    src: *const u16,
    cch: i32,
    dst: *mut u16,
    cap: i32,
) -> i32 {
    idn_dispatch(flags, src, cch, dst, cap, convert_domain_to_unicode)
}

/// `IdnToNameprepUnicode(dwFlags, lpUnicodeCharStr, cchUnicodeChar, lpNameprepCharStr, cchNameprepChar)`.
pub unsafe extern "win64" fn IdnToNameprepUnicode(
    flags: u32,
    src: *const u16,
    cch: i32,
    dst: *mut u16,
    cap: i32,
) -> i32 {
    idn_dispatch(flags, src, cch, dst, cap, convert_domain_to_nameprep)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call_idn(
        f: unsafe extern "win64" fn(u32, *const u16, i32, *mut u16, i32) -> i32,
        flags: u32,
        input: &str,
    ) -> Result<String, u32> {
        let wide = crate::strings::to_wide_z(input);
        unsafe {
            let need = f(flags, wide.as_ptr(), -1, std::ptr::null_mut(), 0);
            if need <= 0 {
                return Err(crate::teb::last_error());
            }
            let mut buf = vec![0u16; need as usize];
            let got = f(flags, wide.as_ptr(), -1, buf.as_mut_ptr(), need);
            if got <= 0 {
                return Err(crate::teb::last_error());
            }
            assert_eq!(got, need);
            assert_eq!(buf[got as usize - 1], 0);
            Ok(String::from_utf16(&buf[..got as usize - 1]).unwrap())
        }
    }

    #[test]
    fn punycode_and_idna_roundtrip() {
        // RFC 3492 standard test vectors & internationalized domain names
        assert_eq!(call_idn(IdnToAscii, 0, "example.com").unwrap(), "example.com");
        assert_eq!(call_idn(IdnToAscii, 0, "München.de").unwrap(), "xn--mnchen-3ya.de");
        assert_eq!(call_idn(IdnToAscii, 0, "bücher.example").unwrap(), "xn--bcher-kva.example");
        assert_eq!(call_idn(IdnToAscii, 0, "例え.jp").unwrap(), "xn--r8jz45g.jp");
        assert_eq!(call_idn(IdnToAscii, 0, "テスト。日本").unwrap(), "xn--zckzah.xn--wgv71a");
        assert_eq!(call_idn(IdnToAscii, 0, "münchen.de.").unwrap(), "xn--mnchen-3ya.de.");

        // IdnToUnicode round-trips back to lowercase Unicode
        assert_eq!(call_idn(IdnToUnicode, 0, "xn--mnchen-3ya.de").unwrap(), "münchen.de");
        assert_eq!(call_idn(IdnToUnicode, 0, "xn--bcher-kva.example").unwrap(), "bücher.example");
        assert_eq!(call_idn(IdnToUnicode, 0, "xn--r8jz45g.jp").unwrap(), "例え.jp");
        assert_eq!(call_idn(IdnToUnicode, 0, "xn--zckzah.xn--wgv71a").unwrap(), "テスト.日本");

        // RFC 3492 Section 7.1 official test vectors (Russian, Greek, German, Japanese)
        assert_eq!(
            call_idn(IdnToAscii, 0, "почемужеонинеговорятпорусски.ru").unwrap(),
            "xn--b1abfaaepdrnnbgefbadotcwatmq2g4l.ru"
        );
        assert_eq!(
            call_idn(IdnToUnicode, 0, "XN--B1ABFAAEPDRNNBGEFBADOTCWATMQ2G4L.RU").unwrap(),
            "почемужеонинеговорятпорусски.RU"
        );
        assert_eq!(
            call_idn(IdnToAscii, 0, "ελληνικά.gr").unwrap(),
            "xn--hxargifdar.gr"
        );
        assert_eq!(
            call_idn(IdnToUnicode, 0, "xn--hxargifdar.gr").unwrap(),
            "ελληνικά.gr"
        );

        // Emoji / non-BMP surrogate pairs round-trip properly
        let emoji_ace = call_idn(IdnToAscii, 0, "🚀.example").unwrap();
        assert!(emoji_ace.starts_with("xn--"));
        assert_eq!(call_idn(IdnToUnicode, 0, &emoji_ace).unwrap(), "🚀.example");

        // IdnToNameprepUnicode lowercases, strips soft-hyphen, normalizes ideographic dot
        assert_eq!(
            call_idn(IdnToNameprepUnicode, 0, "Mü\u{00AD}nchen。DE").unwrap(),
            "münchen.de"
        );

        // Email address flag
        assert_eq!(
            call_idn(IdnToAscii, IDN_EMAIL_ADDRESS, "Alice@München.de").unwrap(),
            "Alice@xn--mnchen-3ya.de"
        );

        // Raw punycode mode
        let raw_enc = call_idn(IdnToAscii, IDN_RAW_PUNYCODE, "münchen").unwrap();
        assert_eq!(raw_enc, "mnchen-3ya");
        assert_eq!(call_idn(IdnToUnicode, IDN_RAW_PUNYCODE, &raw_enc).unwrap(), "münchen");

        // STD3 rules reject invalid hyphens & underscores
        assert_eq!(
            call_idn(IdnToAscii, IDN_USE_STD3_ASCII_RULES, "-bad.example"),
            Err(ERROR_INVALID_NAME)
        );
        assert_eq!(
            call_idn(IdnToAscii, IDN_USE_STD3_ASCII_RULES, "bad_host.example"),
            Err(ERROR_INVALID_NAME)
        );
        // Invalid punycode rejected
        assert_eq!(call_idn(IdnToUnicode, 0, "xn--!!!.com"), Err(ERROR_INVALID_NAME));
    }

    #[test]
    fn explicit_length_and_buffer_sizing() {
        unsafe {
            let src = crate::strings::to_wide("bücher.de");
            let len = IdnToAscii(0, src.as_ptr(), src.len() as i32, std::ptr::null_mut(), 0);
            assert_eq!(len, "xn--bcher-kva.de".len() as i32);

            let mut small = [0u16; 4];
            assert_eq!(IdnToAscii(0, src.as_ptr(), src.len() as i32, small.as_mut_ptr(), 4), 0);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);

            let mut out = vec![0u16; len as usize];
            let written = IdnToAscii(0, src.as_ptr(), src.len() as i32, out.as_mut_ptr(), len);
            assert_eq!(written, len);
            assert_eq!(String::from_utf16(&out).unwrap(), "xn--bcher-kva.de");
        }
    }
}
