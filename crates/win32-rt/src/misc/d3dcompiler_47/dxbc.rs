//! Retail DXBC MD5 checksum, container parsing/building, chunk stripping, and
//! blob part extraction/replacement.

use crate::strings::le_u32;
use crate::types::{E_FAIL, E_INVALIDARG, HRESULT};

// ---------------------------------------------------------------------------
// Constants (<d3dcompiler.h>)
// ---------------------------------------------------------------------------

// D3DCOMPILER_STRIP_FLAGS
pub(super) const D3DCOMPILER_STRIP_REFLECTION_DATA: u32 = 0x0000_0001;
pub(super) const D3DCOMPILER_STRIP_DEBUG_INFO: u32 = 0x0000_0002;
pub(super) const D3DCOMPILER_STRIP_TEST_BLOBS: u32 = 0x0000_0004;
pub(super) const D3DCOMPILER_STRIP_PRIVATE_DATA: u32 = 0x0000_0008;
pub(super) const D3DCOMPILER_STRIP_ROOT_SIGNATURE: u32 = 0x0000_0010;

// D3D_BLOB_PART
pub(super) const D3D_BLOB_INPUT_SIGNATURE_BLOB: u32 = 0;
pub(super) const D3D_BLOB_OUTPUT_SIGNATURE_BLOB: u32 = 1;
pub(super) const D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB: u32 = 2;
pub(super) const D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB: u32 = 3;
pub(super) const D3D_BLOB_ALL_SIGNATURE_BLOB: u32 = 4;
pub(super) const D3D_BLOB_DEBUG_INFO: u32 = 5;
pub(super) const D3D_BLOB_LEGACY_SHADER: u32 = 6;
pub(super) const D3D_BLOB_XNA_PREPASS_SHADER: u32 = 7;
pub(super) const D3D_BLOB_XNA_SHADER: u32 = 8;
pub(super) const D3D_BLOB_PDB: u32 = 9;
pub(super) const D3D_BLOB_PRIVATE_DATA: u32 = 10;
pub(super) const D3D_BLOB_ROOT_SIGNATURE: u32 = 11;
pub(super) const D3D_BLOB_DEBUG_NAME: u32 = 12;
pub(super) const D3D_BLOB_TEST_ALTERNATE_SHADER: u32 = 0x8000;
pub(super) const D3D_BLOB_TEST_COMPILE_DETAILS: u32 = 0x8001;
pub(super) const D3D_BLOB_TEST_COMPILE_PERF: u32 = 0x8002;
pub(super) const D3D_BLOB_TEST_COMPILE_REPORT: u32 = 0x8003;

// ---------------------------------------------------------------------------
// Retail DXBC MD5 checksum & container parsing/building
// ---------------------------------------------------------------------------

// Precomputed standard MD5 sine table: floor(2^32 * abs(sin(i + 1))).
const MD5_K: [u32; 64] = [
    0xD76AA478, 0xE8C7B756, 0x242070DB, 0xC1BDCEEE,
    0xF57C0FAF, 0x4787C62A, 0xA8304613, 0xFD469501,
    0x698098D8, 0x8B44F7AF, 0xFFFF5BB1, 0x895CD7BE,
    0x6B901122, 0xFD987193, 0xA679438E, 0x49B40821,
    0xF61E2562, 0xC040B340, 0x265E5A51, 0xE9B6C7AA,
    0xD62F105D, 0x02441453, 0xD8A1E681, 0xE7D3FBC8,
    0x21E1CDE6, 0xC33707D6, 0xF4D50D87, 0x455A14ED,
    0xA9E3E905, 0xFCEFA3F8, 0x676F02D9, 0x8D2A4C8A,
    0xFFFA3942, 0x8771F681, 0x6D9D6122, 0xFDE5380C,
    0xA4BEEA44, 0x4BDECFA9, 0xF6BB4B60, 0xBEBFBC70,
    0x289B7EC6, 0xEAA127FA, 0xD4EF3085, 0x04881D05,
    0xD9D4D039, 0xE6DB99E5, 0x1FA27CF8, 0xC4AC5665,
    0xF4292244, 0x432AFF97, 0xAB9423A7, 0xFC93A039,
    0x655B59C3, 0x8F0CCC92, 0xFFEFF47D, 0x85845DD1,
    0x6FA87E4F, 0xFE2CE6E0, 0xA3014314, 0x4E0811A1,
    0xF7537E82, 0xBD3AF235, 0x2AD7D2BB, 0xEB86D391,
];

fn md5_transform(state: &mut [u32; 4], x: &[u32; 16]) {
    const S: [[u32; 4]; 4] = [
        [7, 12, 17, 22],
        [5, 9, 14, 20],
        [4, 11, 16, 23],
        [6, 10, 15, 21],
    ];
    let [mut a, mut b, mut c, mut d] = *state;
    for i in 0..64 {
        let (f, g) = match i / 16 {
            0 => ((b & c) | (!b & d), i),
            1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
            2 => (b ^ c ^ d, (3 * i + 5) % 16),
            _ => (c ^ (b | !d), (7 * i) % 16),
        };
        let t = d;
        d = c;
        c = b;
        b = b.wrapping_add(
            a.wrapping_add(f)
                .wrapping_add(MD5_K[i])
                .wrapping_add(x[g])
                .rotate_left(S[i / 16][i % 4]),
        );
        a = t;
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
}

#[inline]
fn block_words(b: &[u8; 64]) -> [u32; 16] {
    std::array::from_fn(|i| u32::from_le_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
}

/// Retail DXBC container checksum (modified MD5 over `container[20..size]`).
///
/// Unlike standard MD5, Microsoft's `ComputeHashRetail` places the bit length at
/// word 0 of the final padding block (before the remaining bytes when `rem < 56`)
/// and `(n << 1) | 1` in word 15.
pub(super) fn dxbc_checksum(data: &[u8]) -> [u8; 16] {
    let n = data.len();
    let mut state = [0x6745_2301u32, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476];
    let full = n / 64;
    for blk in 0..full {
        let b: &[u8; 64] = data[blk * 64..blk * 64 + 64].try_into().unwrap();
        md5_transform(&mut state, &block_words(b));
    }
    let rem = &data[full * 64..];
    let bits = (n as u32).wrapping_shl(3);
    let tail = 1u32 | (n as u32).wrapping_shl(1);
    let mut last = [0u8; 64];
    if rem.len() < 56 {
        last[0..4].copy_from_slice(&bits.to_le_bytes());
        last[4..4 + rem.len()].copy_from_slice(rem);
        last[4 + rem.len()] = 0x80;
        last[60..64].copy_from_slice(&tail.to_le_bytes());
        md5_transform(&mut state, &block_words(&last));
    } else {
        let mut row = [0u8; 64];
        row[..rem.len()].copy_from_slice(rem);
        row[rem.len()] = 0x80;
        md5_transform(&mut state, &block_words(&row));
        last[0..4].copy_from_slice(&bits.to_le_bytes());
        last[60..64].copy_from_slice(&tail.to_le_bytes());
        md5_transform(&mut state, &block_words(&last));
    }
    let mut out = [0u8; 16];
    for (i, s) in state.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&s.to_le_bytes());
    }
    out
}

pub(super) fn rd_cstr(b: &[u8], off: usize) -> Option<String> {
    let tail = b.get(off..)?;
    let end = tail.iter().position(|&c| c == 0).unwrap_or(tail.len());
    Some(String::from_utf8_lossy(&tail[..end]).into_owned())
}

pub(super) struct ParsedDxbc<'a> {
    pub(super) version: [u8; 4],
    pub(super) parts: Vec<([u8; 4], &'a [u8])>,
}

impl<'a> ParsedDxbc<'a> {
    pub(super) fn parse(bytes: &'a [u8]) -> Option<Self> {
        if bytes.len() < 32 || &bytes[0..4] != b"DXBC" {
            return None;
        }
        let version: [u8; 4] = bytes[20..24].try_into().ok()?;
        let declared_size = le_u32(bytes, 24)? as usize;
        let chunk_count = le_u32(bytes, 28)? as usize;
        if declared_size < 32 || declared_size > bytes.len() {
            return None;
        }
        let container = &bytes[..declared_size];
        let table_end = 32usize.checked_add(chunk_count.checked_mul(4)?)?;
        if chunk_count > 4096 || table_end > declared_size {
            return None;
        }
        let mut parts = Vec::with_capacity(chunk_count);
        for i in 0..chunk_count {
            let off = le_u32(container, 32 + i * 4)? as usize;
            let hdr_end = off.checked_add(8)?;
            if hdr_end > declared_size {
                return None;
            }
            let fourcc: [u8; 4] = container[off..off + 4].try_into().ok()?;
            let clen = le_u32(container, off + 4)? as usize;
            let payload_end = hdr_end.checked_add(clen)?;
            if payload_end > declared_size {
                return None;
            }
            parts.push((fourcc, &container[hdr_end..payload_end]));
        }
        Some(Self { version, parts })
    }

    pub(super) fn part(&self, fourcc: &[u8; 4]) -> Option<&'a [u8]> {
        self.parts.iter().find(|(f, _)| f == fourcc).map(|(_, d)| *d)
    }

    pub(super) fn first_of(&self, candidates: &[&[u8; 4]]) -> Option<([u8; 4], &'a [u8])> {
        for cand in candidates {
            if let Some(d) = self.part(cand) {
                return Some((**cand, d));
            }
        }
        None
    }
}

pub(super) fn build_dxbc_container(version: [u8; 4], parts: &[([u8; 4], &[u8])]) -> Option<Vec<u8>> {
    let header_len = 32usize.checked_add(parts.len().checked_mul(4)?)?;
    let total_len = parts.iter().try_fold(header_len, |acc, (_, payload)| {
        acc.checked_add(8usize.checked_add(payload.len())?)
    })?;
    let total_u32 = u32::try_from(total_len).ok()?;

    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(b"DXBC");
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(&version);
    out.extend_from_slice(&total_u32.to_le_bytes());
    out.extend_from_slice(&(parts.len() as u32).to_le_bytes());

    let mut off = header_len;
    for (_, payload) in parts {
        out.extend_from_slice(&(off as u32).to_le_bytes());
        off += 8 + payload.len();
    }
    for (fourcc, payload) in parts {
        out.extend_from_slice(fourcc);
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(payload);
    }

    let digest = dxbc_checksum(&out[20..]);
    out[4..20].copy_from_slice(&digest);
    Some(out)
}

#[cfg(target_os = "macos")]
pub(super) fn ensure_dxbc_signed(bytes: &mut [u8]) {
    if bytes.len() < 32 || &bytes[0..4] != b"DXBC" {
        return;
    }
    let Some(declared_size) = le_u32(bytes, 24).map(|v| v as usize) else {
        return;
    };
    if declared_size < 32 || declared_size > bytes.len() {
        return;
    }
    let digest = dxbc_checksum(&bytes[20..declared_size]);
    if bytes[4..20] != digest {
        bytes[4..20].copy_from_slice(&digest);
    }
}

fn should_strip_chunk(fourcc: [u8; 4], flags: u32) -> bool {
    if (flags & D3DCOMPILER_STRIP_REFLECTION_DATA) != 0 && matches!(&fourcc, b"RDEF" | b"STAT") {
        return true;
    }
    if (flags & D3DCOMPILER_STRIP_DEBUG_INFO) != 0
        && matches!(&fourcc, b"SDBG" | b"SPDB" | b"ILDB" | b"ILDN")
    {
        return true;
    }
    if (flags & D3DCOMPILER_STRIP_TEST_BLOBS) != 0 && matches!(&fourcc, b"TBLK" | b"TEST") {
        return true;
    }
    if (flags & D3DCOMPILER_STRIP_PRIVATE_DATA) != 0 && &fourcc == b"PRIV" {
        return true;
    }
    if (flags & D3DCOMPILER_STRIP_ROOT_SIGNATURE) != 0 && &fourcc == b"RTS0" {
        return true;
    }
    false
}

pub(super) fn strip_dxbc_container(bytes: &[u8], strip_flags: u32) -> Option<Vec<u8>> {
    let parsed = ParsedDxbc::parse(bytes)?;
    let kept: Vec<([u8; 4], &[u8])> = parsed
        .parts
        .into_iter()
        .filter(|(fcc, _)| !should_strip_chunk(*fcc, strip_flags))
        .collect();
    build_dxbc_container(parsed.version, &kept)
}

// ---------------------------------------------------------------------------
// Blob part helpers (`D3DGetBlobPart` / `D3DSetBlobPart`)
// ---------------------------------------------------------------------------

fn set_part_fourcc(part: u32) -> Option<[u8; 4]> {
    match part {
        D3D_BLOB_PRIVATE_DATA => Some(*b"PRIV"),
        D3D_BLOB_ROOT_SIGNATURE => Some(*b"RTS0"),
        D3D_BLOB_DEBUG_INFO => Some(*b"SDBG"),
        D3D_BLOB_PDB => Some(*b"SPDB"),
        D3D_BLOB_DEBUG_NAME => Some(*b"ILDN"),
        D3D_BLOB_TEST_ALTERNATE_SHADER => Some(*b"TBLK"),
        D3D_BLOB_TEST_COMPILE_DETAILS => Some(*b"TCMP"),
        D3D_BLOB_TEST_COMPILE_PERF => Some(*b"TPRF"),
        D3D_BLOB_TEST_COMPILE_REPORT => Some(*b"TRPT"),
        _ => None,
    }
}

pub(super) fn extract_blob_part(bytes: &[u8], part: u32) -> Result<Vec<u8>, HRESULT> {
    let dxbc = ParsedDxbc::parse(bytes).ok_or(E_FAIL)?;
    match part {
        D3D_BLOB_INPUT_SIGNATURE_BLOB => {
            let p = dxbc.first_of(&[b"ISG1", b"ISGN"]).ok_or(E_FAIL)?;
            build_dxbc_container(dxbc.version, &[p]).ok_or(E_FAIL)
        }
        D3D_BLOB_OUTPUT_SIGNATURE_BLOB => {
            let p = dxbc.first_of(&[b"OSG1", b"OSG5", b"OSGN"]).ok_or(E_FAIL)?;
            build_dxbc_container(dxbc.version, &[p]).ok_or(E_FAIL)
        }
        D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB => {
            let mut parts = Vec::with_capacity(2);
            if let Some(p) = dxbc.first_of(&[b"ISG1", b"ISGN"]) {
                parts.push(p);
            }
            if let Some(p) = dxbc.first_of(&[b"OSG1", b"OSG5", b"OSGN"]) {
                parts.push(p);
            }
            if parts.is_empty() {
                return Err(E_FAIL);
            }
            build_dxbc_container(dxbc.version, &parts).ok_or(E_FAIL)
        }
        D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB => {
            let p = dxbc.first_of(&[b"PSG1", b"PCSG"]).ok_or(E_FAIL)?;
            build_dxbc_container(dxbc.version, &[p]).ok_or(E_FAIL)
        }
        D3D_BLOB_ALL_SIGNATURE_BLOB => {
            let mut parts = Vec::with_capacity(3);
            if let Some(p) = dxbc.first_of(&[b"ISG1", b"ISGN"]) {
                parts.push(p);
            }
            if let Some(p) = dxbc.first_of(&[b"OSG1", b"OSG5", b"OSGN"]) {
                parts.push(p);
            }
            if let Some(p) = dxbc.first_of(&[b"PSG1", b"PCSG"]) {
                parts.push(p);
            }
            if parts.is_empty() {
                return Err(E_FAIL);
            }
            build_dxbc_container(dxbc.version, &parts).ok_or(E_FAIL)
        }
        D3D_BLOB_DEBUG_INFO => dxbc
            .first_of(&[b"SDBG", b"SPDB", b"ILDB"])
            .map(|(_, d)| d.to_vec())
            .ok_or(E_FAIL),
        D3D_BLOB_LEGACY_SHADER => dxbc.part(b"Aon9").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_XNA_PREPASS_SHADER => dxbc.part(b"XNAP").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_XNA_SHADER => dxbc.part(b"XNAS").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_PDB => dxbc
            .first_of(&[b"SPDB", b"ILDB"])
            .map(|(_, d)| d.to_vec())
            .ok_or(E_FAIL),
        D3D_BLOB_PRIVATE_DATA => dxbc.part(b"PRIV").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_ROOT_SIGNATURE => dxbc.part(b"RTS0").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_DEBUG_NAME => dxbc.part(b"ILDN").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_TEST_ALTERNATE_SHADER => dxbc
            .first_of(&[b"TBLK", b"TEST"])
            .map(|(_, d)| d.to_vec())
            .ok_or(E_FAIL),
        D3D_BLOB_TEST_COMPILE_DETAILS => dxbc.part(b"TCMP").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_TEST_COMPILE_PERF => dxbc.part(b"TPRF").map(|d| d.to_vec()).ok_or(E_FAIL),
        D3D_BLOB_TEST_COMPILE_REPORT => dxbc.part(b"TRPT").map(|d| d.to_vec()).ok_or(E_FAIL),
        _ => Err(E_INVALIDARG),
    }
}

pub(super) fn replace_blob_part(
    bytes: &[u8],
    part: u32,
    payload: Option<&[u8]>,
) -> Result<Vec<u8>, HRESULT> {
    let fourcc = set_part_fourcc(part).ok_or(E_INVALIDARG)?;
    let dxbc = ParsedDxbc::parse(bytes).ok_or(E_FAIL)?;
    let mut parts: Vec<([u8; 4], &[u8])> = Vec::with_capacity(dxbc.parts.len() + 1);
    let mut replaced = false;
    for (fcc, data) in dxbc.parts {
        if fcc == fourcc {
            if let Some(new_data) = payload {
                if !replaced {
                    parts.push((fourcc, new_data));
                    replaced = true;
                }
            }
        } else {
            parts.push((fcc, data));
        }
    }
    if !replaced {
        if let Some(new_data) = payload {
            parts.push((fourcc, new_data));
        }
    }
    build_dxbc_container(dxbc.version, &parts).ok_or(E_FAIL)
}
