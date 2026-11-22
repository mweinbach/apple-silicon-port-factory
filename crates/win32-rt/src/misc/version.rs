//! version.dll: PE `RT_VERSION` (`VS_VERSIONINFO`) file version resource queries.
//!
//! Extracts the raw `RT_VERSION` (type 16) resource from PE32 / PE32+ files on disk
//! (via `crate::path::to_host`) and implements `GetFileVersionInfoSize{A,W,ExA,ExW}`,
//! `GetFileVersionInfo{A,W,ExA,ExW}`, and `VerQueryValue{A,W}` for `\`,
//! `\VarFileInfo\Translation`, and `\StringFileInfo\<langcp>\<key>`.

use crate::types::*;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_RESOURCE_TYPE_NOT_FOUND: u32 = 1813;
const ERROR_RESOURCE_DATA_NOT_FOUND: u32 = 1812;
const RT_VERSION: u32 = 16;

/// Magic marker placed after the UTF-16 `VS_VERSIONINFO` block when `GetFileVersionInfoA`
/// reserves trailing space for ANSI string copies (matching Windows' double-sized A buffer).
const ANSI_POOL_MAGIC: u32 = 0x5641_4E53; // "VANS"

#[inline]
fn align4(v: usize) -> usize {
    (v + 3) & !3
}

#[inline]
fn rd_u16(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off + 2)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}

#[inline]
fn rd_u32(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

/// Extract the first `RT_VERSION` (type 16) resource blob from raw PE32/PE32+ file bytes.
fn extract_rt_version_from_pe(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 0x40 || rd_u16(data, 0)? != 0x5A4D {
        return None;
    }
    let pe_off = rd_u32(data, 0x3C)? as usize;
    if rd_u32(data, pe_off)? != 0x0000_4550 {
        return None;
    }
    let n_sec = rd_u16(data, pe_off + 6)? as usize;
    let opt_sz = rd_u16(data, pe_off + 20)? as usize;
    let opt_off = pe_off + 24;
    let magic = rd_u16(data, opt_off)?;
    let dd_off = match magic {
        0x10B => opt_off + 96,  // PE32
        0x20B => opt_off + 112, // PE32+
        _ => return None,
    };
    let nrva_off = match magic {
        0x10B => opt_off + 92,
        0x20B => opt_off + 108,
        _ => return None,
    };
    let nrva = rd_u32(data, nrva_off)? as usize;
    if nrva <= 2 {
        return None;
    }
    let rsrc_rva = rd_u32(data, dd_off + 2 * 8)?;
    let rsrc_size = rd_u32(data, dd_off + 2 * 8 + 4)?;
    if rsrc_rva == 0 || rsrc_size == 0 {
        return None;
    }

    let sec_base = opt_off + opt_sz;
    let mut secs = Vec::with_capacity(n_sec);
    for i in 0..n_sec {
        let s = sec_base + i * 40;
        let vsize = rd_u32(data, s + 8)?;
        let va = rd_u32(data, s + 12)?;
        let rsize = rd_u32(data, s + 16)?;
        let roff = rd_u32(data, s + 20)?;
        secs.push((va, vsize, roff, rsize));
    }

    let rva_to_off = |rva: u32| -> Option<usize> {
        for &(va, vsize, roff, rsize) in &secs {
            let span = vsize.max(rsize);
            if rva >= va && rva < va.saturating_add(span) {
                let delta = rva - va;
                if delta < rsize {
                    return Some((roff + delta) as usize);
                }
            }
        }
        None
    };

    let root = rva_to_off(rsrc_rva)?;

    // Level 1: find RT_VERSION (id == 16).
    let find_entry = |dir_off: usize, want_id: Option<u32>| -> Option<u32> {
        let named = rd_u16(data, dir_off + 12)? as usize;
        let ids = rd_u16(data, dir_off + 14)? as usize;
        let entries = dir_off + 16;
        for i in 0..named.saturating_add(ids) {
            let e = entries + i * 8;
            let name_or_id = rd_u32(data, e)?;
            let off = rd_u32(data, e + 4)?;
            match want_id {
                Some(id) => {
                    if (name_or_id & 0x8000_0000) == 0 && name_or_id == id {
                        return Some(off);
                    }
                }
                None => return Some(off),
            }
        }
        None
    };

    let type_off = find_entry(root, Some(RT_VERSION))?;
    if type_off & 0x8000_0000 == 0 {
        return None;
    }
    let name_dir = root.checked_add((type_off & 0x7FFF_FFFF) as usize)?;
    // Level 2: first version resource name/ID (typically 1).
    let name_off = find_entry(name_dir, Some(1)).or_else(|| find_entry(name_dir, None))?;
    if name_off & 0x8000_0000 == 0 {
        return None;
    }
    let lang_dir = root.checked_add((name_off & 0x7FFF_FFFF) as usize)?;
    // Level 3: prefer en-US (0x0409), neutral (0), or first available language.
    let data_entry_off = find_entry(lang_dir, Some(0x0409))
        .or_else(|| find_entry(lang_dir, Some(0)))
        .or_else(|| find_entry(lang_dir, None))?;
    if data_entry_off & 0x8000_0000 != 0 {
        return None;
    }
    let de = root.checked_add(data_entry_off as usize)?;
    let blob_rva = rd_u32(data, de)?;
    let blob_size = rd_u32(data, de + 4)? as usize;
    let blob_off = rva_to_off(blob_rva)?;
    let blob = data.get(blob_off..blob_off.checked_add(blob_size)?)?;
    Some(blob.to_vec())
}

fn load_version_blob(win_file: &str) -> Option<Vec<u8>> {
    if win_file.is_empty() {
        return None;
    }
    let host = crate::path::to_host(win_file)?;
    let bytes = std::fs::read(&host).ok()?;
    extract_rt_version_from_pe(&bytes)
}

/// Parsed node inside a UTF-16 `VS_VERSIONINFO` tree.
struct VerNode {
    key: String,
    value_off: usize,
    value_len_raw: usize, // words for string (wType==1), bytes for binary (wType==0)
    is_string: bool,
    children_off: usize,
    end_off: usize,
}

fn parse_node(block: &[u8], off: usize, limit: usize) -> Option<VerNode> {
    if off + 6 > limit || off + 6 > block.len() {
        return None;
    }
    let w_length = rd_u16(block, off)? as usize;
    if w_length < 6 {
        return None;
    }
    let w_value_len = rd_u16(block, off + 2)? as usize;
    let w_type = rd_u16(block, off + 4)?;
    let end_off = (off + w_length).min(limit).min(block.len());
    let mut pos = off + 6;
    let mut key_u16 = Vec::new();
    while pos + 2 <= end_off {
        let ch = rd_u16(block, pos)?;
        pos += 2;
        if ch == 0 {
            break;
        }
        key_u16.push(ch);
    }
    let key = String::from_utf16_lossy(&key_u16);
    let value_off = align4(pos).min(end_off);
    let is_string = w_type == 1;
    let value_bytes = if is_string { w_value_len * 2 } else { w_value_len };
    let children_off = align4(value_off.saturating_add(value_bytes)).min(end_off);
    Some(VerNode {
        key,
        value_off,
        value_len_raw: w_value_len,
        is_string,
        children_off,
        end_off,
    })
}

/// Find a value in a UTF-16 `VS_VERSIONINFO` block for a sub-block path like
/// `"\\"`, `"\\VarFileInfo\\Translation"`, or `"\\StringFileInfo\\040904b0\\FileVersion"`.
/// Returns `(byte_offset, raw_len, is_string)`.
fn query_ver_node(block: &[u8], sub: &str) -> Option<(usize, usize, bool)> {
    let root = parse_node(block, 0, block.len())?;
    let trimmed = sub.trim_start_matches('\\');
    if trimmed.is_empty() {
        // "\" -> VS_FIXEDFILEINFO
        if root.value_len_raw == 0 || root.value_off + root.value_len_raw > root.end_off {
            return None;
        }
        return Some((root.value_off, root.value_len_raw, false));
    }
    let parts: Vec<&str> = trimmed.split('\\').filter(|s| !s.is_empty()).collect();
    let mut cur_start = root.children_off;
    let mut cur_end = root.end_off;
    for (depth, want) in parts.iter().enumerate() {
        let mut p = align4(cur_start);
        let mut found = None;
        while p + 6 <= cur_end {
            let Some(node) = parse_node(block, p, cur_end) else { break };
            if node.key.eq_ignore_ascii_case(want) {
                found = Some(node);
                break;
            }
            let next = align4(node.end_off);
            if next <= p {
                break;
            }
            p = next;
        }
        let node = found?;
        if depth + 1 == parts.len() {
            return Some((node.value_off, node.value_len_raw, node.is_string));
        }
        cur_start = node.children_off;
        cur_end = node.end_off;
    }
    None
}

/// Populate the trailing ANSI string pool inside a `GetFileVersionInfoA` buffer so
/// `VerQueryValueA` can return pointers directly inside the caller's buffer.
fn populate_ansi_pool(dst: &mut [u8], raw_len: usize) {
    let hdr_off = align4(raw_len);
    if dst.len() < hdr_off + 8 {
        return;
    }
    dst[hdr_off..hdr_off + 4].copy_from_slice(&ANSI_POOL_MAGIC.to_le_bytes());
    dst[hdr_off + 4..hdr_off + 8].copy_from_slice(&(raw_len as u32).to_le_bytes());
    let pool_base = hdr_off + 8;

    fn walk(src: &[u8], dst: &mut [u8], pool_base: usize, start: usize, end: usize) {
        let mut p = align4(start);
        while p + 6 <= end {
            let Some(node) = parse_node(src, p, end) else { break };
            if node.is_string && node.value_len_raw > 0 && node.value_off < end {
                let words = node.value_len_raw.min((node.end_off - node.value_off) / 2);
                let mut u16s = Vec::with_capacity(words);
                for i in 0..words {
                    if let Some(ch) = rd_u16(src, node.value_off + i * 2) {
                        u16s.push(ch);
                    }
                }
                let slot = pool_base + node.value_off / 2;
                let max_bytes = words.max(1);
                if slot + max_bytes <= dst.len() {
                    let s = String::from_utf16_lossy(&u16s);
                    let trimmed = s.trim_end_matches('\0');
                    let bytes: Vec<u8> = trimmed.chars().map(|c| if (c as u32) < 0x100 { c as u8 } else { b'?' }).collect();
                    let n = bytes.len().min(max_bytes - 1);
                    dst[slot..slot + n].copy_from_slice(&bytes[..n]);
                    dst[slot + n] = 0;
                }
            }
            if node.children_off < node.end_off {
                walk(src, dst, pool_base, node.children_off, node.end_off);
            }
            let next = align4(node.end_off);
            if next <= p {
                break;
            }
            p = next;
        }
    }

    let snapshot = dst[..raw_len].to_vec();
    if let Some(root) = parse_node(&snapshot, 0, raw_len) {
        walk(&snapshot, dst, pool_base, root.children_off, root.end_off);
    }
}

pub unsafe extern "win64" fn GetFileVersionInfoSizeW(file: LPCWSTR, handle: *mut u32) -> u32 {
    if !handle.is_null() {
        std::ptr::write_unaligned(handle, 0);
    }
    let Some(path) = crate::strings::from_wide_opt(file) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let Some(blob) = load_version_blob(&path) else {
        log::debug!("GetFileVersionInfoSizeW({path:?}) -> 0 (no RT_VERSION)");
        crate::teb::set_last_error(ERROR_RESOURCE_TYPE_NOT_FOUND);
        return 0;
    };
    let total = align4(blob.len()) + 8 + align4((blob.len() + 1) / 2);
    crate::teb::set_last_error(0);
    total as u32
}

pub unsafe extern "win64" fn GetFileVersionInfoSizeA(file: LPCSTR, handle: *mut u32) -> u32 {
    if !handle.is_null() {
        std::ptr::write_unaligned(handle, 0);
    }
    let Some(path) = crate::strings::from_ansi_opt(file) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    let Some(blob) = load_version_blob(&path) else {
        log::debug!("GetFileVersionInfoSizeA({path:?}) -> 0 (no RT_VERSION)");
        crate::teb::set_last_error(ERROR_RESOURCE_TYPE_NOT_FOUND);
        return 0;
    };
    let total = align4(blob.len()) + 8 + align4((blob.len() + 1) / 2);
    crate::teb::set_last_error(0);
    total as u32
}

pub unsafe extern "win64" fn GetFileVersionInfoSizeExW(_flags: u32, file: LPCWSTR, handle: *mut u32) -> u32 {
    GetFileVersionInfoSizeW(file, handle)
}

pub unsafe extern "win64" fn GetFileVersionInfoSizeExA(_flags: u32, file: LPCSTR, handle: *mut u32) -> u32 {
    GetFileVersionInfoSizeA(file, handle)
}

unsafe fn fill_version_info(path: &str, len: u32, data: *mut u8) -> BOOL {
    if data.is_null() || len == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(blob) = load_version_blob(path) else {
        crate::teb::set_last_error(ERROR_RESOURCE_TYPE_NOT_FOUND);
        return FALSE;
    };
    let cap = len as usize;
    let copy_n = blob.len().min(cap);
    let dst = std::slice::from_raw_parts_mut(data, cap);
    dst[..copy_n].copy_from_slice(&blob[..copy_n]);
    if copy_n < cap {
        dst[copy_n..].fill(0);
    }
    if copy_n == blob.len() {
        populate_ansi_pool(dst, blob.len());
    }
    crate::teb::set_last_error(0);
    TRUE
}

pub unsafe extern "win64" fn GetFileVersionInfoW(file: LPCWSTR, _handle: u32, len: u32, data: *mut u8) -> BOOL {
    let Some(path) = crate::strings::from_wide_opt(file) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    fill_version_info(&path, len, data)
}

pub unsafe extern "win64" fn GetFileVersionInfoA(file: LPCSTR, _handle: u32, len: u32, data: *mut u8) -> BOOL {
    let Some(path) = crate::strings::from_ansi_opt(file) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    fill_version_info(&path, len, data)
}

pub unsafe extern "win64" fn GetFileVersionInfoExW(_flags: u32, file: LPCWSTR, handle: u32, len: u32, data: *mut u8) -> BOOL {
    GetFileVersionInfoW(file, handle, len, data)
}

pub unsafe extern "win64" fn GetFileVersionInfoExA(_flags: u32, file: LPCSTR, handle: u32, len: u32, data: *mut u8) -> BOOL {
    GetFileVersionInfoA(file, handle, len, data)
}

pub unsafe extern "win64" fn VerQueryValueW(block: *const u8, sub: LPCWSTR, out: *mut *mut u8, len: *mut u32) -> BOOL {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if !len.is_null() {
        std::ptr::write_unaligned(len, 0);
    }
    if block.is_null() || sub.is_null() {
        crate::teb::set_last_error(ERROR_RESOURCE_DATA_NOT_FOUND);
        return FALSE;
    }
    let root_len = std::ptr::read_unaligned(block as *const u16) as usize;
    if root_len < 6 {
        crate::teb::set_last_error(ERROR_RESOURCE_DATA_NOT_FOUND);
        return FALSE;
    }
    let slice = std::slice::from_raw_parts(block, root_len);
    let sub_str = crate::strings::from_wide(sub);
    let Some((off, raw_len, _is_str)) = query_ver_node(slice, &sub_str) else {
        crate::teb::set_last_error(ERROR_RESOURCE_DATA_NOT_FOUND);
        return FALSE;
    };
    if !out.is_null() {
        std::ptr::write_unaligned(out, block.add(off) as *mut u8);
    }
    if !len.is_null() {
        std::ptr::write_unaligned(len, raw_len as u32);
    }
    TRUE
}

pub unsafe extern "win64" fn VerQueryValueA(block: *const u8, sub: LPCSTR, out: *mut *mut u8, len: *mut u32) -> BOOL {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if !len.is_null() {
        std::ptr::write_unaligned(len, 0);
    }
    if block.is_null() || sub.is_null() {
        crate::teb::set_last_error(ERROR_RESOURCE_DATA_NOT_FOUND);
        return FALSE;
    }
    let root_len = std::ptr::read_unaligned(block as *const u16) as usize;
    if root_len < 6 {
        crate::teb::set_last_error(ERROR_RESOURCE_DATA_NOT_FOUND);
        return FALSE;
    }
    let slice = std::slice::from_raw_parts(block, root_len);
    let sub_str = crate::strings::from_ansi(sub);
    let Some((off, raw_len, is_str)) = query_ver_node(slice, &sub_str) else {
        crate::teb::set_last_error(ERROR_RESOURCE_DATA_NOT_FOUND);
        return FALSE;
    };
    if !is_str {
        if !out.is_null() {
            std::ptr::write_unaligned(out, block.add(off) as *mut u8);
        }
        if !len.is_null() {
            std::ptr::write_unaligned(len, raw_len as u32);
        }
        return TRUE;
    }
    // String value: look up the ANSI pool placed right after the raw UTF-16 resource.
    let mut hdr_off = align4(root_len);
    // The raw resource may have trailing padding beyond root.wLength; scan up to 64 bytes for magic.
    let mut found_pool = None;
    for delta in (0..=64).step_by(4) {
        let p = hdr_off + delta;
        let magic = std::ptr::read_unaligned(block.add(p) as *const u32);
        if magic == ANSI_POOL_MAGIC {
            hdr_off = p;
            found_pool = Some(hdr_off + 8);
            break;
        }
    }
    let Some(pool_base) = found_pool else {
        crate::teb::set_last_error(ERROR_RESOURCE_DATA_NOT_FOUND);
        return FALSE;
    };
    let ansi_ptr = block.add(pool_base + off / 2);
    let mut n = 0usize;
    while n < raw_len && *ansi_ptr.add(n) != 0 {
        n += 1;
    }
    if !out.is_null() {
        std::ptr::write_unaligned(out, ansi_ptr as *mut u8);
    }
    if !len.is_null() {
        std::ptr::write_unaligned(len, (n + 1) as u32);
    }
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_node(out: &mut Vec<u8>, key: &str, value_len_field: u16, w_type: u16, value: &[u8], children: &[u8]) {
        let start = out.len();
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&value_len_field.to_le_bytes());
        out.extend_from_slice(&w_type.to_le_bytes());
        for ch in key.encode_utf16().chain(std::iter::once(0)) {
            out.extend_from_slice(&ch.to_le_bytes());
        }
        while out.len() % 4 != 0 {
            out.push(0);
        }
        out.extend_from_slice(value);
        while out.len() % 4 != 0 {
            out.push(0);
        }
        out.extend_from_slice(children);
        let total = (out.len() - start) as u16;
        out[start..start + 2].copy_from_slice(&total.to_le_bytes());
    }

    fn build_sample_vs_versioninfo() -> Vec<u8> {
        // VS_FIXEDFILEINFO (52 bytes)
        let mut fixed = vec![0u8; 52];
        fixed[0..4].copy_from_slice(&0xFEEF_04BDu32.to_le_bytes());
        fixed[8..12].copy_from_slice(&0x0002_000Fu32.to_le_bytes()); // FileVersionMS = 2.15
        fixed[12..16].copy_from_slice(&0x0001_0064u32.to_le_bytes()); // FileVersionLS = 1.100

        let mut str_entry = Vec::new();
        let fv: Vec<u8> = "2.15.1.100\0".encode_utf16().flat_map(|w| w.to_le_bytes()).collect();
        push_node(&mut str_entry, "FileVersion", 11, 1, &fv, &[]);

        let mut str_table = Vec::new();
        push_node(&mut str_table, "040904b0", 0, 1, &[], &str_entry);

        let mut sfi = Vec::new();
        push_node(&mut sfi, "StringFileInfo", 0, 1, &[], &str_table);

        let mut trans = Vec::new();
        push_node(&mut trans, "Translation", 4, 0, &[0x09, 0x04, 0xB0, 0x04], &[]);
        let mut vfi = Vec::new();
        push_node(&mut vfi, "VarFileInfo", 0, 1, &[], &trans);

        let mut children = Vec::new();
        children.extend_from_slice(&sfi);
        while children.len() % 4 != 0 {
            children.push(0);
        }
        children.extend_from_slice(&vfi);

        let mut root = Vec::new();
        push_node(&mut root, "VS_VERSION_INFO", 52, 0, &fixed, &children);
        root
    }

    #[test]
    fn ver_query_value_a_and_w_roundtrip() {
        let raw = build_sample_vs_versioninfo();
        let total = align4(raw.len()) + 8 + align4((raw.len() + 1) / 2);
        let mut buf = vec![0u8; total];
        buf[..raw.len()].copy_from_slice(&raw);
        populate_ansi_pool(&mut buf, raw.len());

        unsafe {
            // Query "\" -> VS_FIXEDFILEINFO
            let mut ptr: *mut u8 = std::ptr::null_mut();
            let mut len = 0u32;
            assert_eq!(VerQueryValueA(buf.as_ptr(), b"\\\0".as_ptr(), &mut ptr, &mut len), TRUE);
            assert_eq!(len, 52);
            assert_eq!(std::ptr::read_unaligned(ptr as *const u32), 0xFEEF_04BD);

            // Query "\VarFileInfo\Translation"
            assert_eq!(
                VerQueryValueA(buf.as_ptr(), b"\\VarFileInfo\\Translation\0".as_ptr(), &mut ptr, &mut len),
                TRUE
            );
            assert_eq!(len, 4);
            assert_eq!(std::ptr::read_unaligned(ptr as *const u16), 0x0409);
            assert_eq!(std::ptr::read_unaligned(ptr.add(2) as *const u16), 0x04B0);

            // Query "\StringFileInfo\040904b0\FileVersion" (ANSI)
            assert_eq!(
                VerQueryValueA(buf.as_ptr(), b"\\StringFileInfo\\040904b0\\FileVersion\0".as_ptr(), &mut ptr, &mut len),
                TRUE
            );
            assert_eq!(crate::strings::from_ansi(ptr), "2.15.1.100");
            assert_eq!(len, 11);

            // Query same key via VerQueryValueW
            let wsub: Vec<u16> = "\\StringFileInfo\\040904b0\\FileVersion\0".encode_utf16().collect();
            assert_eq!(VerQueryValueW(buf.as_ptr(), wsub.as_ptr(), &mut ptr, &mut len), TRUE);
            assert_eq!(crate::strings::from_wide(ptr as *const u16), "2.15.1.100");
            assert_eq!(len, 11);
        }
    }
}
