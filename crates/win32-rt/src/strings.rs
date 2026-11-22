//! String helpers: UTF-16 (`WCHAR`) and "ANSI" (we run with ACP = CP_UTF8).

/// Windows code page we report for CP_ACP / CP_OEMCP. UTF-8 (65001) keeps
/// the `*A` APIs lossless; Windows 10 1903+ supports this as process ACP.
pub const ACP: u32 = 65001;

pub unsafe fn wcslen(p: *const u16) -> usize {
    if p.is_null() {
        return 0;
    }
    let mut n = 0;
    while *p.add(n) != 0 {
        n += 1;
    }
    n
}

pub unsafe fn strlen(p: *const u8) -> usize {
    if p.is_null() {
        return 0;
    }
    libc::strlen(p as *const libc::c_char)
}

pub unsafe fn wide_slice<'a>(p: *const u16) -> &'a [u16] {
    if p.is_null() {
        return &[];
    }
    std::slice::from_raw_parts(p, wcslen(p))
}

pub unsafe fn cstr_slice<'a>(p: *const u8) -> &'a [u8] {
    if p.is_null() {
        return &[];
    }
    std::slice::from_raw_parts(p, strlen(p))
}

/// Read a NUL-terminated UTF-16 string (lossy).
pub unsafe fn from_wide(p: *const u16) -> String {
    String::from_utf16_lossy(wide_slice(p))
}

/// Read a NUL-terminated UTF-16 string, `None` if the pointer is NULL.
pub unsafe fn from_wide_opt(p: *const u16) -> Option<String> {
    (!p.is_null()).then(|| from_wide(p))
}

/// Read a NUL-terminated ANSI (UTF-8) string (lossy).
pub unsafe fn from_ansi(p: *const u8) -> String {
    String::from_utf8_lossy(cstr_slice(p)).into_owned()
}

pub unsafe fn from_ansi_opt(p: *const u8) -> Option<String> {
    (!p.is_null()).then(|| from_ansi(p))
}

pub fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

pub fn to_wide_z(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Copy `s` into a WCHAR buffer of `cap` elements with the common Win32
/// convention: on success returns chars written (excluding NUL); if too small
/// returns the required size *including* the NUL and writes nothing.
pub unsafe fn write_wide_buf(s: &str, buf: *mut u16, cap: usize) -> (bool, usize) {
    let w = to_wide(s);
    if buf.is_null() || cap < w.len() + 1 {
        return (false, w.len() + 1);
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
    *buf.add(w.len()) = 0;
    (true, w.len())
}

pub unsafe fn write_ansi_buf(s: &str, buf: *mut u8, cap: usize) -> (bool, usize) {
    let b = s.as_bytes();
    if buf.is_null() || cap < b.len() + 1 {
        return (false, b.len() + 1);
    }
    std::ptr::copy_nonoverlapping(b.as_ptr(), buf, b.len());
    *buf.add(b.len()) = 0;
    (true, b.len())
}

/// Copy with truncation (GetModuleFileName-style): always NUL-terminates if
/// cap > 0; returns (truncated?, chars copied excluding NUL).
pub unsafe fn write_wide_trunc(s: &str, buf: *mut u16, cap: usize) -> (bool, usize) {
    if cap == 0 || buf.is_null() {
        return (true, 0);
    }
    let w = to_wide(s);
    let n = w.len().min(cap - 1);
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, n);
    *buf.add(n) = 0;
    (n < w.len(), n)
}

pub unsafe fn write_ansi_trunc(s: &str, buf: *mut u8, cap: usize) -> (bool, usize) {
    if cap == 0 || buf.is_null() {
        return (true, 0);
    }
    let b = s.as_bytes();
    let n = b.len().min(cap - 1);
    std::ptr::copy_nonoverlapping(b.as_ptr(), buf, n);
    *buf.add(n) = 0;
    (n < b.len(), n)
}

/// Leak a NUL-terminated UTF-16 copy (for process-lifetime strings).
pub fn leak_wide_z(s: &str) -> *mut u16 {
    Box::leak(to_wide_z(s).into_boxed_slice()).as_mut_ptr()
}

pub fn leak_ansi_z(s: &str) -> *mut u8 {
    let mut v = s.as_bytes().to_vec();
    v.push(0);
    Box::leak(v.into_boxed_slice()).as_mut_ptr()
}

/// Decode bytes in code page `cp` to a String. Supports UTF-8, ASCII,
/// Latin-1 (28591) and Windows-1252 (approximated as Latin-1 except the
/// 0x80-0x9F range).
pub fn decode_cp(cp: u32, bytes: &[u8]) -> Result<String, ()> {
    match cp {
        0 | 1 | 2 | 3 | 65001 => std::str::from_utf8(bytes).map(|s| s.to_string()).map_err(|_| ()),
        20127 => Ok(bytes.iter().map(|&b| (b & 0x7f) as char).collect()),
        28591 | 1252 | 437 | 850 => Ok(bytes.iter().map(|&b| cp1252_to_char(b)).collect()),
        _ => std::str::from_utf8(bytes).map(|s| s.to_string()).map_err(|_| ()),
    }
}

fn cp1252_to_char(b: u8) -> char {
    const HI: [u16; 32] = [
        0x20AC, 0x81, 0x201A, 0x0192, 0x201E, 0x2026, 0x2020, 0x2021, 0x02C6, 0x2030, 0x0160, 0x2039,
        0x0152, 0x8D, 0x017D, 0x8F, 0x90, 0x2018, 0x2019, 0x201C, 0x201D, 0x2022, 0x2013, 0x2014,
        0x02DC, 0x2122, 0x0161, 0x203A, 0x0153, 0x9D, 0x017E, 0x0178,
    ];
    if (0x80..0xA0).contains(&b) {
        char::from_u32(HI[(b - 0x80) as usize] as u32).unwrap_or('?')
    } else {
        b as char
    }
}

/// Encode to code page `cp`. Non-representable chars become '?'.
pub fn encode_cp(cp: u32, s: &str) -> Vec<u8> {
    match cp {
        28591 | 1252 | 437 | 850 | 20127 => s
            .chars()
            .map(|c| if (c as u32) < 0x80 || ((c as u32) >= 0xA0 && (c as u32) < 0x100 && cp != 20127) { c as u8 } else { b'?' })
            .collect(),
        _ => s.as_bytes().to_vec(),
    }
}

/// ASCII-only case folding used by the C locale `_stricmp` & friends.
pub fn ascii_lower(c: u8) -> u8 {
    c.to_ascii_lowercase()
}

/// Simple Unicode lowercase for a UTF-16 unit (BMP, non-surrogate).
pub fn wlower(c: u16) -> u16 {
    if c < 0x80 {
        return (c as u8).to_ascii_lowercase() as u16;
    }
    match char::from_u32(c as u32) {
        Some(ch) => {
            let mut it = ch.to_lowercase();
            match (it.next(), it.next()) {
                (Some(l), None) if (l as u32) < 0x10000 => l as u32 as u16,
                _ => c,
            }
        }
        None => c,
    }
}

pub fn wupper(c: u16) -> u16 {
    if c < 0x80 {
        return (c as u8).to_ascii_uppercase() as u16;
    }
    match char::from_u32(c as u32) {
        Some(ch) => {
            let mut it = ch.to_uppercase();
            match (it.next(), it.next()) {
                (Some(l), None) if (l as u32) < 0x10000 => l as u32 as u16,
                _ => c,
            }
        }
        None => c,
    }
}
