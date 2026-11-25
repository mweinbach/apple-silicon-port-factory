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

/// Read a NUL-terminated UTF-16 string into a `Vec<u16>` using unaligned reads.
/// Safe for arbitrary x86-64 guest pointers that may not be 2-byte aligned.
pub unsafe fn wide_vec_unaligned(p: *const u16) -> Vec<u16> {
    if p.is_null() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    loop {
        let u = p.add(i).read_unaligned();
        if u == 0 {
            break;
        }
        out.push(u);
        i += 1;
    }
    out
}

pub unsafe fn cstr_slice<'a>(p: *const u8) -> &'a [u8] {
    if p.is_null() {
        return &[];
    }
    std::slice::from_raw_parts(p, strlen(p))
}

/// Read a NUL-terminated UTF-16 string (lossy, unaligned-safe).
pub unsafe fn from_wide(p: *const u16) -> String {
    if p.is_aligned() {
        String::from_utf16_lossy(wide_slice(p))
    } else {
        read_wide_z(p, usize::MAX)
    }
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

// ---------------------------------------------------------------------------
// Unaligned, bounded, null-safe guest string & memory helpers
// ---------------------------------------------------------------------------

/// Default upper bound for NUL-terminated guest string reads when callers pass
/// `-1` / sentinel lengths.
pub const DEFAULT_MAX_STR_LEN: usize = 1024 * 1024;

/// Read a NUL-terminated UTF-16 string as raw `u16` code units (unaligned-safe,
/// bounded to `max_chars`). Returns an empty `Vec` if `ptr` is null.
pub unsafe fn read_wide_units_z(ptr: *const u16, max_chars: usize) -> Vec<u16> {
    if ptr.is_null() || max_chars == 0 {
        return Vec::new();
    }
    let mut units = Vec::new();
    for i in 0..max_chars {
        let u = std::ptr::read_unaligned(ptr.add(i));
        if u == 0 {
            break;
        }
        units.push(u);
    }
    units
}

/// Read a NUL-terminated UTF-16 string (unaligned-safe, bounded to `max_chars`).
/// Returns an empty `String` if `ptr` is null.
pub unsafe fn read_wide_z(ptr: *const u16, max_chars: usize) -> String {
    String::from_utf16_lossy(&read_wide_units_z(ptr, max_chars))
}

/// Read a NUL-terminated UTF-16 string (unaligned-safe, bounded), returning
/// `None` when `ptr` is null.
pub unsafe fn read_wide_z_opt(ptr: *const u16, max_chars: usize) -> Option<String> {
    (!ptr.is_null()).then(|| read_wide_z(ptr, max_chars))
}

/// Read exactly `len` UTF-16 code units unaligned (does not stop on embedded NULs).
pub unsafe fn read_wide_slice(ptr: *const u16, len: usize) -> Vec<u16> {
    if ptr.is_null() || len == 0 {
        return Vec::new();
    }
    let mut units = Vec::with_capacity(len.min(4096));
    for i in 0..len {
        units.push(std::ptr::read_unaligned(ptr.add(i)));
    }
    units
}

/// Read up to `len` UTF-16 code units (unaligned-safe, stopping at the first NUL).
pub unsafe fn read_wide_len(ptr: *const u16, len: usize) -> String {
    if ptr.is_null() || len == 0 {
        return String::new();
    }
    let mut units = Vec::with_capacity(len.min(4096));
    for i in 0..len {
        let u = std::ptr::read_unaligned(ptr.add(i));
        if u == 0 {
            break;
        }
        units.push(u);
    }
    String::from_utf16_lossy(&units)
}

/// Read a UTF-16 string where `len_i32 < 0` (e.g. `-1`) means NUL-terminated
/// and `len_i32 >= 0` specifies the exact maximum code-unit count.
/// Returns `None` if `ptr` is null.
pub unsafe fn read_wide_opt_len(ptr: *const u16, len_i32: i32) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    if len_i32 < 0 {
        Some(read_wide_z(ptr, DEFAULT_MAX_STR_LEN))
    } else {
        Some(read_wide_len(ptr, len_i32 as usize))
    }
}

/// Read a UTF-16 string where `len == 0` means NUL-terminated and `len > 0`
/// bounds the read to `len` code units. Returns `None` if `ptr` is null.
pub unsafe fn read_wide_zero_term(ptr: *const u16, len: usize) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    if len == 0 {
        Some(read_wide_z(ptr, DEFAULT_MAX_STR_LEN))
    } else {
        Some(read_wide_len(ptr, len))
    }
}

/// Write `s` as NUL-terminated UTF-16 into `dst` (up to `cap_chars` code units
/// including the NUL terminator, unaligned-safe). Returns the number of UTF-16
/// code units written *excluding* the NUL terminator.
pub unsafe fn write_wide_z(dst: *mut u16, cap_chars: usize, s: &str) -> usize {
    if dst.is_null() || cap_chars == 0 {
        return 0;
    }
    let units: Vec<u16> = s.encode_utf16().collect();
    let n = units.len().min(cap_chars - 1);
    for (i, &u) in units[..n].iter().enumerate() {
        std::ptr::write_unaligned(dst.add(i), u);
    }
    std::ptr::write_unaligned(dst.add(n), 0u16);
    n
}

/// Write `s` as NUL-terminated UTF-16 into a byte buffer of `cap_bytes` bytes
/// (unaligned-safe).
///
/// Returns `(bytes_written_excl_nul, true)` on success, or
/// `(required_bytes_incl_nul, false)` if `dst` is null or `cap_bytes` is too small.
pub unsafe fn write_wide_bytes_out(dst: *mut u8, cap_bytes: usize, s: &str) -> (usize, bool) {
    let units: Vec<u16> = s.encode_utf16().collect();
    let needed_bytes = (units.len() + 1) * 2;
    if dst.is_null() || cap_bytes < needed_bytes {
        return (needed_bytes, false);
    }
    let wdst = dst as *mut u16;
    for (i, &u) in units.iter().enumerate() {
        std::ptr::write_unaligned(wdst.add(i), u);
    }
    std::ptr::write_unaligned(wdst.add(units.len()), 0u16);
    (units.len() * 2, true)
}

/// Read a NUL-terminated byte string (unaligned-safe, bounded to `max_bytes`).
pub unsafe fn read_ansi_bytes_z(ptr: *const u8, max_bytes: usize) -> Vec<u8> {
    if ptr.is_null() || max_bytes == 0 {
        return Vec::new();
    }
    let mut bytes = Vec::new();
    for i in 0..max_bytes {
        let b = std::ptr::read_unaligned(ptr.add(i));
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    bytes
}

/// Read a NUL-terminated ANSI/UTF-8 string (unaligned-safe, bounded to `max_bytes`).
/// Returns an empty `String` if `ptr` is null.
pub unsafe fn read_ansi_z(ptr: *const u8, max_bytes: usize) -> String {
    String::from_utf8_lossy(&read_ansi_bytes_z(ptr, max_bytes)).into_owned()
}

/// Read a NUL-terminated ANSI/UTF-8 string (unaligned-safe, bounded), returning
/// `None` if `ptr` is null.
pub unsafe fn read_ansi_z_opt(ptr: *const u8, max_bytes: usize) -> Option<String> {
    (!ptr.is_null()).then(|| read_ansi_z(ptr, max_bytes))
}

/// Read up to `len` bytes as an ANSI/UTF-8 string, stopping at the first NUL.
pub unsafe fn read_ansi_len(ptr: *const u8, len: usize) -> String {
    if ptr.is_null() || len == 0 {
        return String::new();
    }
    let mut bytes = Vec::with_capacity(len.min(4096));
    for i in 0..len {
        let b = std::ptr::read_unaligned(ptr.add(i));
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Read an ANSI/UTF-8 string where `len_i32 < 0` (e.g. `-1`) means NUL-terminated
/// and `len_i32 >= 0` specifies the exact maximum byte length.
/// Returns `None` if `ptr` is null.
pub unsafe fn read_ansi_opt_len(ptr: *const u8, len_i32: i32) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    if len_i32 < 0 {
        Some(read_ansi_z(ptr, DEFAULT_MAX_STR_LEN))
    } else {
        Some(read_ansi_len(ptr, len_i32 as usize))
    }
}

/// Read an ANSI/UTF-8 string where `len == 0` means NUL-terminated and `len > 0`
/// bounds the read to `len` bytes. Returns `None` if `ptr` is null.
pub unsafe fn read_ansi_zero_term(ptr: *const u8, len: usize) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    if len == 0 {
        Some(read_ansi_z(ptr, DEFAULT_MAX_STR_LEN))
    } else {
        Some(read_ansi_len(ptr, len))
    }
}

/// Write `s` as NUL-terminated UTF-8/ANSI into `dst` (up to `cap_bytes` bytes
/// including the NUL terminator). Returns bytes written *excluding* NUL.
pub unsafe fn write_ansi_z(dst: *mut u8, cap_bytes: usize, s: &str) -> usize {
    if dst.is_null() || cap_bytes == 0 {
        return 0;
    }
    let bytes = s.as_bytes();
    let n = bytes.len().min(cap_bytes - 1);
    if n > 0 {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, n);
    }
    std::ptr::write_unaligned(dst.add(n), 0u8);
    n
}

/// Write `s` as NUL-terminated UTF-8/ANSI into a byte buffer of `cap_bytes`.
///
/// Returns `(bytes_written_excl_nul, true)` on success, or
/// `(required_bytes_incl_nul, false)` if `dst` is null or `cap_bytes` is too small.
pub unsafe fn write_ansi_bytes_out(dst: *mut u8, cap_bytes: usize, s: &str) -> (usize, bool) {
    let bytes = s.as_bytes();
    let needed = bytes.len() + 1;
    if dst.is_null() || cap_bytes < needed {
        return (needed, false);
    }
    if !bytes.is_empty() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
    }
    std::ptr::write_unaligned(dst.add(bytes.len()), 0u8);
    (bytes.len(), true)
}

// ---------------------------------------------------------------------------
// Endian slice readers & generic unaligned guest memory helpers
// ---------------------------------------------------------------------------

#[inline]
pub fn le_u16(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off.checked_add(2)?)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}

#[inline]
pub fn le_u32(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off.checked_add(4)?)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

#[inline]
pub fn le_u64(b: &[u8], off: usize) -> Option<u64> {
    let s = b.get(off..off.checked_add(8)?)?;
    Some(u64::from_le_bytes([
        s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7],
    ]))
}

#[inline]
pub fn be_u16(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off.checked_add(2)?)?;
    Some(u16::from_be_bytes([s[0], s[1]]))
}

#[inline]
pub fn be_u32(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off.checked_add(4)?)?;
    Some(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}

#[inline]
pub fn be_u64(b: &[u8], off: usize) -> Option<u64> {
    let s = b.get(off..off.checked_add(8)?)?;
    Some(u64::from_be_bytes([
        s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7],
    ]))
}

/// Unaligned guest memory read.
#[inline]
pub unsafe fn rd<T: Copy>(p: *const T) -> T {
    std::ptr::read_unaligned(p)
}

/// Unaligned guest memory write.
#[inline]
pub unsafe fn wr<T>(p: *mut T, v: T) {
    std::ptr::write_unaligned(p, v);
}

/// Write `v` unaligned to a COM/Win32 out-pointer, returning `E_POINTER` if null.
#[inline]
pub unsafe fn write_out<T>(out: *mut T, v: T) -> crate::types::HRESULT {
    if out.is_null() {
        return crate::types::E_POINTER;
    }
    std::ptr::write_unaligned(out, v);
    crate::types::S_OK
}

/// Zero-initialize an output pointer slot (`*pp = NULL`) if `pp` is non-null.
#[inline]
pub unsafe fn write_null_out<T>(pp: *mut *mut T) {
    if !pp.is_null() {
        std::ptr::write_unaligned(pp, std::ptr::null_mut());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unaligned_wide_readers_and_writers() {
        // Place UTF-16 data at an odd byte offset to exercise unaligned access.
        let mut raw = [0xCCu8; 64];
        unsafe {
            let unaligned = raw.as_mut_ptr().add(1) as *mut u16;
            assert_eq!(write_wide_z(unaligned, 16, "Hello"), 5);
            assert_eq!(read_wide_z(unaligned, 16), "Hello");
            assert_eq!(read_wide_z(unaligned, 3), "Hel");
            assert_eq!(read_wide_len(unaligned, 4), "Hell");
            assert_eq!(read_wide_opt_len(unaligned, -1).as_deref(), Some("Hello"));
            assert_eq!(read_wide_opt_len(unaligned, 2).as_deref(), Some("He"));
            assert_eq!(read_wide_opt_len(unaligned, 0).as_deref(), Some(""));
            assert_eq!(read_wide_opt_len(std::ptr::null(), -1), None);
            assert_eq!(read_wide_zero_term(unaligned, 0).as_deref(), Some("Hello"));
            assert_eq!(read_wide_zero_term(unaligned, 3).as_deref(), Some("Hel"));

            // Truncating write_wide_z.
            assert_eq!(write_wide_z(unaligned, 4, "Hello"), 3);
            assert_eq!(read_wide_z(unaligned, 16), "Hel");
            assert_eq!(write_wide_z(unaligned, 0, "Hello"), 0);
            assert_eq!(write_wide_z(std::ptr::null_mut(), 8, "Hello"), 0);

            // write_wide_bytes_out: insufficient vs exact fit.
            let bptr = raw.as_mut_ptr().add(3);
            assert_eq!(write_wide_bytes_out(bptr, 6, "Hi!"), (8, false));
            assert_eq!(write_wide_bytes_out(std::ptr::null_mut(), 16, "Hi!"), (8, false));
            assert_eq!(write_wide_bytes_out(bptr, 8, "Hi!"), (6, true));
            assert_eq!(read_wide_z(bptr as *const u16, 16), "Hi!");

            // Direct from_wide / from_wide_opt at a guaranteed odd address (aligned u16 base + 1 byte).
            let mut aligned_backing = [0xCCu16; 32];
            let odd = (aligned_backing.as_mut_ptr() as *mut u8).add(1) as *mut u16;
            assert!(!odd.is_aligned());
            let sample = "Odd-\u{03A9}-\u{1F680}";
            assert_eq!(write_wide_z(odd, 30, sample), sample.encode_utf16().count());
            assert_eq!(from_wide(odd), sample);
            assert_eq!(from_wide_opt(odd).as_deref(), Some(sample));
            assert_eq!(from_wide(std::ptr::null()), "");
            assert_eq!(from_wide_opt(std::ptr::null()), None);
        }
    }

    #[test]
    fn unaligned_ansi_readers_and_writers() {
        let mut buf = [0xAAu8; 32];
        unsafe {
            let p = buf.as_mut_ptr().add(1);
            assert_eq!(write_ansi_z(p, 16, "world"), 5);
            assert_eq!(read_ansi_z(p, 16), "world");
            assert_eq!(read_ansi_z(p, 3), "wor");
            assert_eq!(read_ansi_opt_len(p, -1).as_deref(), Some("world"));
            assert_eq!(read_ansi_opt_len(p, 2).as_deref(), Some("wo"));
            assert_eq!(read_ansi_opt_len(std::ptr::null(), -1), None);
            assert_eq!(read_ansi_zero_term(p, 0).as_deref(), Some("world"));
            assert_eq!(read_ansi_zero_term(p, 4).as_deref(), Some("worl"));

            assert_eq!(write_ansi_z(p, 4, "world"), 3);
            assert_eq!(read_ansi_z(p, 16), "wor");

            assert_eq!(write_ansi_bytes_out(p, 3, "abc"), (4, false));
            assert_eq!(write_ansi_bytes_out(p, 4, "abc"), (3, true));
            assert_eq!(read_ansi_z(p, 16), "abc");
        }
    }

    #[test]
    fn endian_slice_and_generic_unaligned_helpers() {
        let data = [0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        assert_eq!(le_u16(&data, 1), Some(0x0302));
        assert_eq!(le_u32(&data, 1), Some(0x0504_0302));
        assert_eq!(le_u64(&data, 1), Some(0x0908_0706_0504_0302));
        assert_eq!(be_u16(&data, 1), Some(0x0203));
        assert_eq!(be_u32(&data, 1), Some(0x0203_0405));
        assert_eq!(be_u64(&data, 1), Some(0x0203_0405_0607_0809));
        assert_eq!(le_u32(&data, 6), None);
        assert_eq!(be_u32(&data, usize::MAX), None);

        let mut raw = [0u8; 24];
        unsafe {
            let up = raw.as_mut_ptr().add(3) as *mut u64;
            wr(up, 0x1122_3344_5566_7788u64);
            assert_eq!(rd(up), 0x1122_3344_5566_7788u64);
            assert_eq!(write_out(up, 42u64), crate::types::S_OK);
            assert_eq!(rd(up), 42u64);
            assert_eq!(write_out::<u64>(std::ptr::null_mut(), 1), crate::types::E_POINTER);

            let mut sentinel = 0x1234usize as *mut u8;
            write_null_out(&mut sentinel);
            assert!(sentinel.is_null());
            write_null_out::<u8>(std::ptr::null_mut());
        }
    }
}
