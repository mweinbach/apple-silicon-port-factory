//! Fast, honest macOS `.app` code-signature verification and signing-receipt tracking.
//!
//! Large Windows PC games can ship 100+ GiB of read-only assets inside
//! `Contents/Resources/Game/`. Apple's single-threaded `codesign --verify --deep --strict`
//! hashes every byte on one core, which takes several minutes per invocation — and doing
//! that immediately after `codesign --force --sign -` already hashed the exact same payload
//! wastes another full disk read pass.
//!
//! This module avoids redundant single-threaded re-hashing **without silently skipping
//! verification**:
//! 1. Always runs `codesign --verify --deep --strict --ignore-resources <App>.app` (~20 ms)
//!    to verify every nested Mach-O binary (`Frameworks/*.dylib`, `dxil-metald`,
//!    `port-runner`, and the bundle launcher), `Info.plist`, symlink rules, and bundle layout.
//! 2. Reads the Mach-O `LC_CODE_SIGNATURE` `CS_CodeDirectory` from `CFBundleExecutable`,
//!    computes its exact full SHA-256 `CDHash`, and cryptographically verifies that
//!    `SHA256(Contents/_CodeSignature/CodeResources)` equals `CodeDirectory` special slot `-3`
//!    (`CSSLOT_RESOURCEDIR`).
//! 3. Walks `Contents/` to verify the exact sealed file set matches `CodeResources` `files2`
//!    (catching any added unsealed file, deleted sealed file, or modified symlink target).
//! 4. Always hashes all non-`Game/` resources (`game.toml`, manifests, shaders, AOT caches)
//!    byte-for-byte.
//! 5. For `Resources/Game/` files:
//!    - If `full_rehash` is false and an external signing/verification receipt matching this
//!      exact `(cdhash, coderesources_sha256)` exists with identical `(dev, ino, size, mtime)`
//!      for every file, reports the receipt match honestly (and tells the caller `--full`
//!      forces a full re-hash).
//!    - Otherwise (no receipt, `--full`, copied to another directory/SSD where `dev`/`ino`
//!      differs, or any file edited/touched), hashes every uncached/changed file in parallel
//!      across all CPU cores via hardware-accelerated `CommonCrypto` `CC_SHA256` and verifies
//!      every single digest against `CodeResources` `files2[path].hash2`.
//!
//! Receipts are stored strictly **outside** the `.app` bundle (default
//! `~/Library/Caches/port-factory/bundle-verify-receipts/<cdhash>.tsv`, overridable via
//! `PORT_VERIFY_CACHE_DIR`), so the signed `.app` bundle stays 100% pristine and valid for
//! Finder launch and Apple's `codesign --verify --deep --strict`.

use crate::bundle_common::{codesign_verify_deep_strict, extract_plist_string};
use anyhow::{anyhow, bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

// ============================================================================
// 1. Hardware-Accelerated SHA-256 (CommonCrypto on macOS, pure-Rust fallback)
// ============================================================================

#[cfg(target_os = "macos")]
mod sha256_impl {
    use std::ffi::c_void;
    use std::io::Read;
    use std::path::Path;

    #[repr(C)]
    struct CcSha256Ctx {
        count: [u32; 2],
        hash: [u32; 8],
        wbuf: [u32; 16],
    }

    extern "C" {
        fn CC_SHA256_Init(c: *mut CcSha256Ctx) -> i32;
        fn CC_SHA256_Update(c: *mut CcSha256Ctx, data: *const c_void, len: u32) -> i32;
        fn CC_SHA256_Final(md: *mut u8, c: *mut CcSha256Ctx) -> i32;
    }

    pub fn sha256_bytes(data: &[u8]) -> [u8; 32] {
        let mut ctx = CcSha256Ctx {
            count: [0; 2],
            hash: [0; 8],
            wbuf: [0; 16],
        };
        let mut out = [0u8; 32];
        unsafe {
            CC_SHA256_Init(&mut ctx);
            for chunk in data.chunks(1024 * 1024) {
                CC_SHA256_Update(&mut ctx, chunk.as_ptr().cast(), chunk.len() as u32);
            }
            CC_SHA256_Final(out.as_mut_ptr(), &mut ctx);
        }
        out
    }

    pub fn sha256_file(path: &Path) -> std::io::Result<[u8; 32]> {
        let mut file = std::fs::File::open(path)?;
        let mut ctx = CcSha256Ctx {
            count: [0; 2],
            hash: [0; 8],
            wbuf: [0; 16],
        };
        let mut buf = vec![0u8; 2 * 1024 * 1024];
        unsafe {
            CC_SHA256_Init(&mut ctx);
        }
        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }
            unsafe {
                CC_SHA256_Update(&mut ctx, buf.as_ptr().cast(), n as u32);
            }
        }
        let mut out = [0u8; 32];
        unsafe {
            CC_SHA256_Final(out.as_mut_ptr(), &mut ctx);
        }
        Ok(out)
    }
}

#[cfg(not(target_os = "macos"))]
mod sha256_impl {
    use std::io::Read;
    use std::path::Path;

    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    struct Sha256 {
        h: [u32; 8],
        buf: [u8; 64],
        buf_len: usize,
        total_bits: u64,
    }

    impl Sha256 {
        fn new() -> Self {
            Self {
                h: [
                    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c,
                    0x1f83d9ab, 0x5be0cd19,
                ],
                buf: [0; 64],
                buf_len: 0,
                total_bits: 0,
            }
        }

        fn compress(&mut self, block: &[u8; 64]) {
            let mut w = [0u32; 64];
            for i in 0..16 {
                w[i] = u32::from_be_bytes([
                    block[4 * i],
                    block[4 * i + 1],
                    block[4 * i + 2],
                    block[4 * i + 3],
                ]);
            }
            for i in 16..64 {
                let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
                let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
                w[i] = w[i - 16]
                    .wrapping_add(s0)
                    .wrapping_add(w[i - 7])
                    .wrapping_add(s1);
            }
            let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.h;
            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ ((!e) & g);
                let temp1 = h
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(K[i])
                    .wrapping_add(w[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let temp2 = s0.wrapping_add(maj);
                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(temp1);
                d = c;
                c = b;
                b = a;
                a = temp1.wrapping_add(temp2);
            }
            self.h[0] = self.h[0].wrapping_add(a);
            self.h[1] = self.h[1].wrapping_add(b);
            self.h[2] = self.h[2].wrapping_add(c);
            self.h[3] = self.h[3].wrapping_add(d);
            self.h[4] = self.h[4].wrapping_add(e);
            self.h[5] = self.h[5].wrapping_add(f);
            self.h[6] = self.h[6].wrapping_add(g);
            self.h[7] = self.h[7].wrapping_add(h);
        }

        fn update(&mut self, mut data: &[u8]) {
            self.total_bits = self.total_bits.wrapping_add((data.len() as u64) * 8);
            if self.buf_len > 0 {
                let take = (64 - self.buf_len).min(data.len());
                self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&data[..take]);
                self.buf_len += take;
                data = &data[take..];
                if self.buf_len == 64 {
                    let block = self.buf;
                    self.compress(&block);
                    self.buf_len = 0;
                }
            }
            while data.len() >= 64 {
                let block: [u8; 64] = data[..64].try_into().unwrap();
                self.compress(&block);
                data = &data[64..];
            }
            if !data.is_empty() {
                self.buf[..data.len()].copy_from_slice(data);
                self.buf_len = data.len();
            }
        }

        fn finalize(mut self) -> [u8; 32] {
            let bits = self.total_bits;
            self.buf[self.buf_len] = 0x80;
            self.buf_len += 1;
            if self.buf_len > 56 {
                for b in &mut self.buf[self.buf_len..] {
                    *b = 0;
                }
                let block = self.buf;
                self.compress(&block);
                self.buf = [0; 64];
                self.buf_len = 0;
            } else {
                for b in &mut self.buf[self.buf_len..56] {
                    *b = 0;
                }
            }
            self.buf[56..64].copy_from_slice(&bits.to_be_bytes());
            let block = self.buf;
            self.compress(&block);
            let mut out = [0u8; 32];
            for (i, w) in self.h.iter().enumerate() {
                out[4 * i..4 * i + 4].copy_from_slice(&w.to_be_bytes());
            }
            out
        }
    }

    pub fn sha256_bytes(data: &[u8]) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(data);
        h.finalize()
    }

    pub fn sha256_file(path: &Path) -> std::io::Result<[u8; 32]> {
        let mut file = std::fs::File::open(path)?;
        let mut h = Sha256::new();
        let mut buf = vec![0u8; 1024 * 1024];
        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }
            h.update(&buf[..n]);
        }
        Ok(h.finalize())
    }
}

pub use sha256_impl::{sha256_bytes, sha256_file};

pub fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn hex_decode_32(s: &str) -> Option<[u8; 32]> {
    let b = s.trim().as_bytes();
    if b.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    let nibble = |c: u8| -> Option<u8> {
        match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'a'..=b'f' => Some(c - b'a' + 10),
            b'A'..=b'F' => Some(c - b'A' + 10),
            _ => None,
        }
    };
    for i in 0..32 {
        out[i] = (nibble(b[2 * i])? << 4) | nibble(b[2 * i + 1])?;
    }
    Some(out)
}

// ============================================================================
// 2. Mach-O LC_CODE_SIGNATURE / CS_CodeDirectory Reader
// ============================================================================

/// Sealed cryptographic anchors extracted from a signed Mach-O `CS_CodeDirectory`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachoCodeDirInfo {
    /// Full 64-hex SHA-256 CDHash of the primary SHA-256 `CodeDirectory` blob.
    pub cdhash_full_hex: String,
    /// Special slot `-3` (`CSSLOT_RESOURCEDIR`): expected SHA-256 of `Contents/_CodeSignature/CodeResources`.
    pub resource_dir_sha256: [u8; 32],
}

fn read_u32_le(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn read_u32_be(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

/// Extract the SHA-256 `CDHash` and sealed `CodeResources` digest (special slot `-3`)
/// from a signed Mach-O 64-bit or universal (fat) executable.
pub fn read_macho_code_directory_info(macho_path: &Path) -> Result<MachoCodeDirInfo> {
    let data = fs::read(macho_path)
        .with_context(|| format!("reading Mach-O {}", macho_path.display()))?;
    parse_macho_code_directory_info(&data)
        .with_context(|| format!("parsing LC_CODE_SIGNATURE in {}", macho_path.display()))
}

pub fn parse_macho_code_directory_info(data: &[u8]) -> Result<MachoCodeDirInfo> {
    if data.len() < 32 {
        bail!("Mach-O file too short");
    }
    let slice = match read_u32_be(data, 0).unwrap_or(0) {
        0xCAFE_BABE => {
            // Fat/universal Mach-O: prefer arm64 (`0x0100_000C`), else first slice.
            let nfat = read_u32_be(data, 4).unwrap_or(0) as usize;
            let mut chosen: Option<(usize, usize)> = None;
            for i in 0..nfat {
                let entry_off = 8 + i * 20;
                let Some(cputype) = read_u32_be(data, entry_off) else {
                    break;
                };
                let Some(off) = read_u32_be(data, entry_off + 8) else {
                    break;
                };
                let Some(sz) = read_u32_be(data, entry_off + 12) else {
                    break;
                };
                let pair = (off as usize, sz as usize);
                if cputype == 0x0100_000C {
                    chosen = Some(pair);
                    break;
                }
                if chosen.is_none() {
                    chosen = Some(pair);
                }
            }
            let (off, sz) = chosen.ok_or_else(|| anyhow!("no slice in universal Mach-O"))?;
            data.get(off..off + sz)
                .ok_or_else(|| anyhow!("universal Mach-O slice out of bounds"))?
        }
        _ => data,
    };

    let magic_le = read_u32_le(slice, 0).unwrap_or(0);
    if magic_le != 0xFEED_FACF {
        bail!("unsupported Mach-O magic 0x{magic_le:08x} (expected 64-bit little-endian)");
    }
    let ncmds = read_u32_le(slice, 16).ok_or_else(|| anyhow!("truncated Mach-O header"))? as usize;
    let mut off = 32usize; // sizeof(mach_header_64)
    let mut sig_range: Option<(usize, usize)> = None;
    for _ in 0..ncmds {
        let cmd = read_u32_le(slice, off).ok_or_else(|| anyhow!("truncated load command"))?;
        let cmdsize =
            read_u32_le(slice, off + 4).ok_or_else(|| anyhow!("truncated load command size"))?
                as usize;
        if cmdsize < 8 || off + cmdsize > slice.len() {
            bail!("invalid Mach-O load command size {cmdsize}");
        }
        if cmd == 0x1D {
            // LC_CODE_SIGNATURE (linkedit_data_command)
            let dataoff = read_u32_le(slice, off + 8)
                .ok_or_else(|| anyhow!("truncated LC_CODE_SIGNATURE"))?
                as usize;
            let datasize = read_u32_le(slice, off + 12)
                .ok_or_else(|| anyhow!("truncated LC_CODE_SIGNATURE"))?
                as usize;
            sig_range = Some((dataoff, datasize));
            break;
        }
        off += cmdsize;
    }

    let (sig_off, sig_sz) =
        sig_range.ok_or_else(|| anyhow!("Mach-O lacks LC_CODE_SIGNATURE load command"))?;
    let sb = slice
        .get(sig_off..sig_off.saturating_add(sig_sz))
        .ok_or_else(|| anyhow!("LC_CODE_SIGNATURE range out of bounds"))?;

    let sb_magic = read_u32_be(sb, 0).ok_or_else(|| anyhow!("truncated SuperBlob"))?;
    if sb_magic != 0xFADE_0CC0 {
        bail!("unexpected SuperBlob magic 0x{sb_magic:08x}");
    }
    let sb_count = read_u32_be(sb, 8).ok_or_else(|| anyhow!("truncated SuperBlob count"))? as usize;

    for i in 0..sb_count {
        let idx_off = 12 + i * 8;
        let Some(blob_off) = read_u32_be(sb, idx_off + 4).map(|v| v as usize) else {
            break;
        };
        let Some(cd_magic) = read_u32_be(sb, blob_off) else {
            continue;
        };
        if cd_magic != 0xFADE_0C02 {
            continue;
        }
        let cd_len = read_u32_be(sb, blob_off + 4)
            .ok_or_else(|| anyhow!("truncated CodeDirectory length"))? as usize;
        let cd_blob = sb
            .get(blob_off..blob_off.saturating_add(cd_len))
            .ok_or_else(|| anyhow!("CodeDirectory blob out of bounds"))?;
        if cd_blob.len() < 40 {
            continue;
        }
        let hash_offset = read_u32_be(cd_blob, 16).unwrap_or(0) as usize;
        let n_special = read_u32_be(cd_blob, 24).unwrap_or(0) as usize;
        let hash_size = cd_blob[36] as usize;
        let hash_type = cd_blob[37];
        // hash_type == 2 => CS_HASHTYPE_SHA256 (32 bytes)
        if hash_type == 2 && hash_size == 32 && n_special >= 3 && hash_offset >= 3 * 32 {
            let slot3_start = hash_offset - 3 * 32;
            let slot3_slice = cd_blob
                .get(slot3_start..slot3_start + 32)
                .ok_or_else(|| anyhow!("CodeDirectory slot -3 out of bounds"))?;
            let mut resource_dir_sha256 = [0u8; 32];
            resource_dir_sha256.copy_from_slice(slot3_slice);
            let cdhash = sha256_bytes(cd_blob);
            return Ok(MachoCodeDirInfo {
                cdhash_full_hex: hex_encode(&cdhash),
                resource_dir_sha256,
            });
        }
    }

    bail!("no SHA-256 CodeDirectory with resource directory slot (-3) found")
}

// ============================================================================
// 3. CodeResources XML Plist (`files2`) Parser
// ============================================================================

/// One entry in `Contents/_CodeSignature/CodeResources` -> `files2`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CodeResourcesEntry {
    pub hash2: Option<[u8; 32]>,
    pub symlink: Option<String>,
    pub optional: bool,
    pub is_cdhash: bool,
}

fn xml_unescape(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp + 1..];
        if let Some(semi) = rest.find(';') {
            let ent = &rest[..semi];
            let decoded = match ent {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" => Some('\''),
                _ if ent.starts_with("#x") || ent.starts_with("#X") => {
                    u32::from_str_radix(&ent[2..], 16).ok().and_then(char::from_u32)
                }
                _ if ent.starts_with('#') => ent[1..].parse::<u32>().ok().and_then(char::from_u32),
                _ => None,
            };
            if let Some(ch) = decoded {
                out.push(ch);
            } else {
                out.push('&');
                out.push_str(ent);
                out.push(';');
            }
            rest = &rest[semi + 1..];
        } else {
            out.push('&');
        }
    }
    out.push_str(rest);
    out
}

fn base64_decode(raw: &str) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(raw.len() * 3 / 4);
    let mut vals = [0u8; 4];
    let mut count = 0usize;
    let mut pad = 0usize;

    for &b in raw.as_bytes() {
        if b.is_ascii_whitespace() {
            continue;
        }
        if b == b'=' {
            vals[count] = 0;
            count += 1;
            pad += 1;
        } else {
            let v = match b {
                b'A'..=b'Z' => b - b'A',
                b'a'..=b'z' => b - b'a' + 26,
                b'0'..=b'9' => b - b'0' + 52,
                b'+' => 62,
                b'/' => 63,
                other => bail!("invalid base64 byte 0x{other:02x}"),
            };
            vals[count] = v;
            count += 1;
        }
        if count == 4 {
            out.push((vals[0] << 2) | (vals[1] >> 4));
            if pad < 2 {
                out.push((vals[1] << 4) | (vals[2] >> 2));
            }
            if pad < 1 {
                out.push((vals[2] << 6) | vals[3]);
            }
            count = 0;
            pad = 0;
        }
    }
    if count != 0 {
        bail!("truncated base64 data");
    }
    Ok(out)
}

/// Extract the inner XML between the matching `<dict>` and `</dict>` starting at `start_pos`
/// (which must point to `<dict>`).
fn extract_balanced_dict(xml: &str, start_pos: usize) -> Option<(&str, usize)> {
    let rest = &xml[start_pos..];
    if !rest.starts_with("<dict>") {
        return None;
    }
    let mut depth = 0usize;
    let mut i = start_pos;
    let bytes = xml.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'<' {
            let tail = &xml[i..];
            if tail.starts_with("<dict>") {
                depth += 1;
                i += "<dict>".len();
                continue;
            } else if tail.starts_with("</dict>") {
                depth -= 1;
                if depth == 0 {
                    return Some((&xml[start_pos + "<dict>".len()..i], i + "</dict>".len()));
                }
                i += "</dict>".len();
                continue;
            }
        }
        i += 1;
    }
    None
}

fn find_next_tag<'a>(xml: &'a str, from: usize, tag: &str) -> Option<(String, usize)> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let rel_start = xml[from..].find(&open)?;
    let body_start = from + rel_start + open.len();
    let rel_end = xml[body_start..].find(&close)?;
    let body = &xml[body_start..body_start + rel_end];
    Some((body.to_string(), body_start + rel_end + close.len()))
}

/// Parse the `<key>files2</key><dict>...</dict>` map from `Contents/_CodeSignature/CodeResources`.
pub fn parse_code_resources_files2(plist_xml: &str) -> Result<BTreeMap<String, CodeResourcesEntry>> {
    let marker = "<key>files2</key>";
    let pos = plist_xml
        .find(marker)
        .ok_or_else(|| anyhow!("CodeResources missing <key>files2</key> section"))?;
    let after = pos + marker.len();
    let dict_rel = plist_xml[after..]
        .find("<dict>")
        .ok_or_else(|| anyhow!("CodeResources files2 missing <dict>"))?;
    let (files2_body, _) = extract_balanced_dict(plist_xml, after + dict_rel)
        .ok_or_else(|| anyhow!("unbalanced <dict> in CodeResources files2"))?;

    let mut map = BTreeMap::new();
    let mut cur = 0usize;
    while cur < files2_body.len() {
        let Some((raw_path, after_key)) = find_next_tag(files2_body, cur, "key") else {
            break;
        };
        let path_key = xml_unescape(raw_path.trim());

        // Value for each `files2` entry is either `<data>...</data>` (compact hash2) or `<dict>...</dict>`.
        let ws_trimmed = files2_body[after_key..].trim_start();
        let val_start = files2_body.len() - ws_trimmed.len();

        let mut entry = CodeResourcesEntry::default();
        if ws_trimmed.starts_with("<data>") {
            let (b64, next_cur) = find_next_tag(files2_body, val_start, "data")
                .ok_or_else(|| anyhow!("unterminated <data> for {path_key:?}"))?;
            let decoded = base64_decode(&b64)?;
            if decoded.len() == 32 {
                let mut h = [0u8; 32];
                h.copy_from_slice(&decoded);
                entry.hash2 = Some(h);
            }
            cur = next_cur;
        } else if ws_trimmed.starts_with("<dict>") {
            let (inner, next_cur) = extract_balanced_dict(files2_body, val_start)
                .ok_or_else(|| anyhow!("unbalanced <dict> for {path_key:?}"))?;
            let mut icur = 0usize;
            while icur < inner.len() {
                let Some((field, after_field)) = find_next_tag(inner, icur, "key") else {
                    break;
                };
                let frest = inner[after_field..].trim_start();
                let fval_start = inner.len() - frest.len();
                match field.trim() {
                    "hash2" => {
                        let (b64, next_i) = find_next_tag(inner, fval_start, "data")
                            .ok_or_else(|| anyhow!("missing <data> for hash2 of {path_key:?}"))?;
                        let decoded = base64_decode(&b64)?;
                        if decoded.len() == 32 {
                            let mut h = [0u8; 32];
                            h.copy_from_slice(&decoded);
                            entry.hash2 = Some(h);
                        }
                        icur = next_i;
                    }
                    "symlink" => {
                        let (target, next_i) = find_next_tag(inner, fval_start, "string")
                            .ok_or_else(|| anyhow!("missing <string> for symlink {path_key:?}"))?;
                        entry.symlink = Some(xml_unescape(&target));
                        icur = next_i;
                    }
                    "optional" => {
                        if frest.starts_with("<true/>") {
                            entry.optional = true;
                            icur = fval_start + "<true/>".len();
                        } else if frest.starts_with("<false/>") {
                            icur = fval_start + "<false/>".len();
                        } else {
                            icur = fval_start + 1;
                        }
                    }
                    "cdhash" => {
                        entry.is_cdhash = true;
                        if let Some((_, next_i)) = find_next_tag(inner, fval_start, "data") {
                            icur = next_i;
                        } else {
                            icur = fval_start + 1;
                        }
                    }
                    _ => {
                        // Skip `<data>`, `<string>`, `<true/>`, `<false/>`, `<dict>`, etc.
                        if frest.starts_with("<data>") {
                            icur = find_next_tag(inner, fval_start, "data")
                                .map_or(inner.len(), |(_, n)| n);
                        } else if frest.starts_with("<string>") {
                            icur = find_next_tag(inner, fval_start, "string")
                                .map_or(inner.len(), |(_, n)| n);
                        } else if frest.starts_with("<true/>") {
                            icur = fval_start + "<true/>".len();
                        } else if frest.starts_with("<false/>") {
                            icur = fval_start + "<false/>".len();
                        } else if frest.starts_with("<dict>") {
                            icur = extract_balanced_dict(inner, fval_start)
                                .map_or(inner.len(), |(_, n)| n);
                        } else {
                            icur = fval_start + 1;
                        }
                    }
                }
            }
            cur = next_cur;
        } else {
            break;
        }

        map.insert(path_key, entry);
    }

    Ok(map)
}

// ============================================================================
// 4. External Signing / Verification Receipt Cache (`CDHash`-keyed)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileStatKey {
    dev: u64,
    ino: u64,
    size: u64,
    mtime_sec: i64,
    mtime_nsec: i64,
}

#[cfg(unix)]
fn stat_file_key(path: &Path) -> std::io::Result<FileStatKey> {
    use std::os::unix::fs::MetadataExt;
    let m = fs::symlink_metadata(path)?;
    Ok(FileStatKey {
        dev: m.dev(),
        ino: m.ino(),
        size: m.len(),
        mtime_sec: m.mtime(),
        mtime_nsec: m.mtime_nsec(),
    })
}

#[cfg(not(unix))]
fn stat_file_key(path: &Path) -> std::io::Result<FileStatKey> {
    let m = fs::symlink_metadata(path)?;
    let d = m
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .unwrap_or_default();
    Ok(FileStatKey {
        dev: 0,
        ino: 0,
        size: m.len(),
        mtime_sec: d.as_secs() as i64,
        mtime_nsec: d.subsec_nanos() as i64,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptRecord {
    stat: FileStatKey,
    hash2: [u8; 32],
}

#[derive(Debug, Clone)]
struct VerifyReceipt {
    cdhash_full_hex: String,
    coderesources_sha256_hex: String,
    entries: BTreeMap<String, ReceiptRecord>,
}

/// Resolve the external bundle verification receipt cache directory (strictly outside `.app`).
pub fn verify_receipt_cache_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .filter(|v| !v.is_empty())
        .map(|h| PathBuf::from(h).join("Library/Caches/port-factory/bundle-verify-receipts"))
}

fn load_receipt(cache_dir: &Path, cdhash_hex: &str) -> Option<VerifyReceipt> {
    let p = cache_dir.join(format!("{cdhash_hex}.tsv"));
    let raw = fs::read_to_string(&p).ok()?;
    let mut got_cdhash = None;
    let mut got_cr = None;
    let mut entries = BTreeMap::new();
    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("# cdhash=") {
            got_cdhash = Some(rest.trim().to_string());
            continue;
        }
        if let Some(rest) = line.strip_prefix("# coderesources_sha256=") {
            got_cr = Some(rest.trim().to_string());
            continue;
        }
        if line.is_empty() || line.starts_with('#') || line.starts_with("rel_path\t") {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() != 7 {
            return None;
        }
        let rel = cols[0].to_string();
        let dev = cols[1].parse::<u64>().ok()?;
        let ino = cols[2].parse::<u64>().ok()?;
        let size = cols[3].parse::<u64>().ok()?;
        let mtime_sec = cols[4].parse::<i64>().ok()?;
        let mtime_nsec = cols[5].parse::<i64>().ok()?;
        let hash2 = hex_decode_32(cols[6])?;
        entries.insert(
            rel,
            ReceiptRecord {
                stat: FileStatKey {
                    dev,
                    ino,
                    size,
                    mtime_sec,
                    mtime_nsec,
                },
                hash2,
            },
        );
    }
    Some(VerifyReceipt {
        cdhash_full_hex: got_cdhash?,
        coderesources_sha256_hex: got_cr?,
        entries,
    })
}

fn write_receipt(cache_dir: &Path, receipt: &VerifyReceipt) -> Result<()> {
    fs::create_dir_all(cache_dir)
        .with_context(|| format!("creating receipt cache dir {}", cache_dir.display()))?;
    let mut out = String::new();
    use std::fmt::Write as _;
    let _ = writeln!(out, "# port-factory-verify-receipt v1");
    let _ = writeln!(out, "# cdhash={}", receipt.cdhash_full_hex);
    let _ = writeln!(
        out,
        "# coderesources_sha256={}",
        receipt.coderesources_sha256_hex
    );
    let _ = writeln!(out, "rel_path\tdev\tino\tsize\tmtime_sec\tmtime_nsec\thash2_hex");
    for (rel, rec) in &receipt.entries {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            rel,
            rec.stat.dev,
            rec.stat.ino,
            rec.stat.size,
            rec.stat.mtime_sec,
            rec.stat.mtime_nsec,
            hex_encode(&rec.hash2)
        );
    }
    let dst = cache_dir.join(format!("{}.tsv", receipt.cdhash_full_hex));
    let tmp = cache_dir.join(format!(
        "{}.{}.tmp",
        receipt.cdhash_full_hex,
        std::process::id()
    ));
    fs::write(&tmp, out).with_context(|| format!("writing receipt {}", tmp.display()))?;
    fs::rename(&tmp, &dst).with_context(|| format!("committing receipt {}", dst.display()))?;
    Ok(())
}

/// Returns `true` if `rel` (relative to `<App>.app/Contents/`) is excluded from `CodeResources`
/// `files2` by Apple's v2 default bundle resource rules (nested code directories, `Info.plist`,
/// `PkgInfo`, `_CodeSignature/`, `.DS_Store`, `*.lproj/locversion.plist`).
fn is_omitted_or_nested_code_rel(rel: &str) -> bool {
    if rel == "Info.plist"
        || rel == "PkgInfo"
        || rel == "_CodeSignature"
        || rel.starts_with("_CodeSignature/")
    {
        return true;
    }
    if rel == ".DS_Store" || rel.ends_with("/.DS_Store") {
        return true;
    }
    for prefix in [
        "MacOS/",
        "Frameworks/",
        "SharedFrameworks/",
        "PlugIns/",
        "Plug-ins/",
        "XPCServices/",
        "Helpers/",
        "Library/Automator/",
        "Library/Spotlight/",
        "Library/LoginItems/",
    ] {
        if rel.starts_with(prefix) || rel == prefix.trim_end_matches('/') {
            return true;
        }
    }
    if rel.starts_with("Resources/") && rel.ends_with(".lproj/locversion.plist") {
        return true;
    }
    false
}

/// Walk `<App>.app/Contents/` without following symlinks and collect every non-omitted regular file
/// and symlink keyed by its `Contents/`-relative forward-slash path.
fn collect_bundle_resource_entries(contents_dir: &Path) -> Result<BTreeMap<String, bool>> {
    // bool: true => symlink, false => regular file
    let mut found = BTreeMap::new();
    let mut stack = vec![contents_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in rd.flatten() {
            let Ok(ft) = entry.file_type() else {
                continue;
            };
            let p = entry.path();
            let rel = p
                .strip_prefix(contents_dir)
                .unwrap_or(&p)
                .to_string_lossy()
                .replace('\\', "/");
            if is_omitted_or_nested_code_rel(&rel) {
                continue;
            }
            if ft.is_symlink() {
                found.insert(rel, true);
            } else if ft.is_dir() {
                stack.push(p);
            } else if ft.is_file() {
                found.insert(rel, false);
            }
        }
    }
    Ok(found)
}

fn resolve_bundle_executable(app_path: &Path) -> Result<PathBuf> {
    let plist_path = app_path.join("Contents/Info.plist");
    let plist_xml = fs::read_to_string(&plist_path)
        .with_context(|| format!("reading {}", plist_path.display()))?;
    let exec_name = extract_plist_string(&plist_xml, "CFBundleExecutable")
        .ok_or_else(|| anyhow!("missing CFBundleExecutable in {}", plist_path.display()))?;
    Ok(app_path.join("Contents/MacOS").join(exec_name))
}

/// Record an external verification receipt immediately after `codesign --force --sign - <App>.app`
/// has sealed `Contents/_CodeSignature/CodeResources`, so a subsequent `bundle-verify` on the
/// unchanged payload does not have to re-hash 100+ GiB of read-only game assets.
///
/// Performs a fast (~20 ms) sanity check (`codesign --verify --deep --strict --ignore-resources`
/// plus `CodeDirectory` slot `-3` vs `SHA256(CodeResources)`) before recording the receipt.
pub fn record_bundle_signing_receipt(app_path: &Path) -> Result<Option<String>> {
    let cache_dir = verify_receipt_cache_dir();
    record_bundle_signing_receipt_in(app_path, cache_dir.as_deref())
}

/// Like [`record_bundle_signing_receipt`], writing the receipt into an explicit cache directory.
pub fn record_bundle_signing_receipt_in(
    app_path: &Path,
    cache_dir: Option<&Path>,
) -> Result<Option<String>> {
    let Some(cache_dir) = cache_dir else {
        return Ok(None);
    };
    #[cfg(target_os = "macos")]
    {
        let shallow = Command::new("codesign")
            .args(["--verify", "--deep", "--strict", "--ignore-resources"])
            .arg(app_path)
            .output()
            .context("running codesign --verify --ignore-resources")?;
        if !shallow.status.success() {
            bail!(
                "codesign shallow verify failed right after signing: {}",
                String::from_utf8_lossy(&shallow.stderr).trim()
            );
        }
    }

    let exec_path = resolve_bundle_executable(app_path)?;
    let cd = read_macho_code_directory_info(&exec_path)?;
    let cr_path = app_path.join("Contents/_CodeSignature/CodeResources");
    let cr_bytes = fs::read(&cr_path)
        .with_context(|| format!("reading {}", cr_path.display()))?;
    let cr_sha = sha256_bytes(&cr_bytes);
    if cr_sha != cd.resource_dir_sha256 {
        bail!("CodeResources SHA-256 does not match Mach-O CodeDirectory slot -3");
    }

    let cr_xml = String::from_utf8_lossy(&cr_bytes);
    let files2 = parse_code_resources_files2(&cr_xml)?;
    let contents_dir = app_path.join("Contents");

    let mut entries = BTreeMap::new();
    for (rel, item) in &files2 {
        if !rel.starts_with("Resources/Game/") {
            continue;
        }
        if let Some(hash2) = item.hash2 {
            let full = contents_dir.join(rel);
            if let Ok(stat) = stat_file_key(&full) {
                entries.insert(rel.clone(), ReceiptRecord { stat, hash2 });
            }
        }
    }

    let receipt = VerifyReceipt {
        cdhash_full_hex: cd.cdhash_full_hex.clone(),
        coderesources_sha256_hex: hex_encode(&cr_sha),
        entries,
    };
    write_receipt(&cache_dir, &receipt)?;
    Ok(Some(cd.cdhash_full_hex))
}

/// Outcome of fast, honest macOS bundle code-signature verification.
#[derive(Debug, Clone)]
pub struct FastCodesignVerifyResult {
    pub passed: bool,
    pub detail: String,
    pub cdhash_short: Option<String>,
    pub hashed_files: usize,
    pub hashed_bytes: u64,
    pub receipt_verified_files: usize,
    pub receipt_verified_bytes: u64,
    pub elapsed_secs: f64,
}

/// Verify a macOS `.app` bundle's code signature honestly and quickly.
///
/// - When `full_rehash == false`, unchanged `Resources/Game/` files matching an external
///   `(cdhash, coderesources_sha256, dev, ino, size, mtime)` receipt avoid redundant hashing
///   while all Mach-O binaries, `Info.plist`, `CodeResources` slot `-3`, complete file tree,
///   and non-`Game/` files are verified fresh every time.
/// - When `full_rehash == true`, or whenever any file lacks a matching receipt or has changed
///   `dev`/`ino`/`size`/`mtime`, every sealed file is hashed byte-for-byte in parallel across
///   all CPU cores and checked against `CodeResources` `files2`.
pub fn verify_bundle_codesign_fast(
    app_path: &Path,
    full_rehash: bool,
) -> Result<FastCodesignVerifyResult> {
    let cache_dir = verify_receipt_cache_dir();
    verify_bundle_codesign_fast_in(app_path, full_rehash, cache_dir.as_deref())
}

/// Like [`verify_bundle_codesign_fast`], using an explicit receipt cache directory (safe for
/// concurrent isolated tests without mutating process-global env vars).
pub fn verify_bundle_codesign_fast_in(
    app_path: &Path,
    full_rehash: bool,
    cache_dir: Option<&Path>,
) -> Result<FastCodesignVerifyResult> {
    let t0 = Instant::now();

    // Step 1: Verify all Mach-O binaries (nested dylibs, helpers, main launcher), Info.plist,
    // strict bundle structure, and symlink policy via Apple's `codesign`.
    let shallow = Command::new("codesign")
        .args(["--verify", "--deep", "--strict", "--ignore-resources"])
        .arg(app_path)
        .output()
        .context("invoking codesign --verify --deep --strict --ignore-resources")?;
    if !shallow.status.success() {
        return Ok(FastCodesignVerifyResult {
            passed: false,
            detail: format!(
                "codesign verify failed: {}",
                String::from_utf8_lossy(&shallow.stderr).trim()
            ),
            cdhash_short: None,
            hashed_files: 0,
            hashed_bytes: 0,
            receipt_verified_files: 0,
            receipt_verified_bytes: 0,
            elapsed_secs: t0.elapsed().as_secs_f64(),
        });
    }

    // Step 2: Cryptographically bind `Contents/_CodeSignature/CodeResources` to the signed
    // Mach-O `CodeDirectory` special slot `-3` (`CSSLOT_RESOURCEDIR`).
    let exec_path = match resolve_bundle_executable(app_path) {
        Ok(p) => p,
        Err(e) => {
            return Ok(FastCodesignVerifyResult {
                passed: false,
                detail: format!("codesign verify failed: {e}"),
                cdhash_short: None,
                hashed_files: 0,
                hashed_bytes: 0,
                receipt_verified_files: 0,
                receipt_verified_bytes: 0,
                elapsed_secs: t0.elapsed().as_secs_f64(),
            });
        }
    };
    let cd = match read_macho_code_directory_info(&exec_path) {
        Ok(info) => info,
        Err(_) => {
            // Unexpected signature format (e.g. SHA-1-only legacy binary): fall back honestly
            // to Apple's full `codesign --verify --deep --strict`.
            return fallback_apple_codesign_verify(app_path, t0);
        }
    };
    let cdhash_short = cd.cdhash_full_hex.chars().take(12).collect::<String>();

    let cr_path = app_path.join("Contents/_CodeSignature/CodeResources");
    let Ok(cr_bytes) = fs::read(&cr_path) else {
        return Ok(FastCodesignVerifyResult {
            passed: false,
            detail: "codesign verify failed: missing Contents/_CodeSignature/CodeResources"
                .to_string(),
            cdhash_short: Some(cdhash_short),
            hashed_files: 0,
            hashed_bytes: 0,
            receipt_verified_files: 0,
            receipt_verified_bytes: 0,
            elapsed_secs: t0.elapsed().as_secs_f64(),
        });
    };
    let cr_sha = sha256_bytes(&cr_bytes);
    if cr_sha != cd.resource_dir_sha256 {
        return Ok(FastCodesignVerifyResult {
            passed: false,
            detail: "codesign verify failed: Contents/_CodeSignature/CodeResources SHA-256 does not match Mach-O CodeDirectory slot -3".to_string(),
            cdhash_short: Some(cdhash_short),
            hashed_files: 0,
            hashed_bytes: 0,
            receipt_verified_files: 0,
            receipt_verified_bytes: 0,
            elapsed_secs: t0.elapsed().as_secs_f64(),
        });
    }
    let cr_sha_hex = hex_encode(&cr_sha);

    let cr_xml = String::from_utf8_lossy(&cr_bytes);
    let files2 = match parse_code_resources_files2(&cr_xml) {
        Ok(m) => m,
        Err(_) => return fallback_apple_codesign_verify(app_path, t0),
    };
    if files2
        .iter()
        .any(|(rel, e)| !is_omitted_or_nested_code_rel(rel) && e.is_cdhash)
    {
        // Nested code objects sealed by cdhash outside standard nested code dirs:
        // delegate to Apple's full check.
        return fallback_apple_codesign_verify(app_path, t0);
    }

    // Step 3: Walk `Contents/` and verify exact file/symlink set against `files2`.
    let contents_dir = app_path.join("Contents");
    let disk_entries = collect_bundle_resource_entries(&contents_dir)?;

    let mut unsealed_added = Vec::new();
    let mut bad_symlinks = Vec::new();
    for (rel, &is_sym) in &disk_entries {
        let Some(sealed) = files2.get(rel) else {
            unsealed_added.push(rel.clone());
            continue;
        };
        if is_sym {
            let raw_target = fs::read_link(contents_dir.join(rel))
                .ok()
                .map(|t| t.to_string_lossy().into_owned());
            if sealed.symlink.as_deref() != raw_target.as_deref() {
                bad_symlinks.push(format!(
                    "{rel} (expected {:?}, got {:?})",
                    sealed.symlink, raw_target
                ));
            }
        } else if sealed.hash2.is_none() {
            bad_symlinks.push(format!("{rel} (expected symlink, found regular file)"));
        }
    }
    if !unsealed_added.is_empty() {
        return Ok(FastCodesignVerifyResult {
            passed: false,
            detail: format!(
                "codesign verify failed: {} unsealed file(s) added: {}",
                unsealed_added.len(),
                unsealed_added
                    .iter()
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            cdhash_short: Some(cdhash_short),
            hashed_files: 0,
            hashed_bytes: 0,
            receipt_verified_files: 0,
            receipt_verified_bytes: 0,
            elapsed_secs: t0.elapsed().as_secs_f64(),
        });
    }
    if !bad_symlinks.is_empty() {
        return Ok(FastCodesignVerifyResult {
            passed: false,
            detail: format!(
                "codesign verify failed: sealed symlink mismatch: {}",
                bad_symlinks
                    .iter()
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            cdhash_short: Some(cdhash_short),
            hashed_files: 0,
            hashed_bytes: 0,
            receipt_verified_files: 0,
            receipt_verified_bytes: 0,
            elapsed_secs: t0.elapsed().as_secs_f64(),
        });
    }

    let disk_keys: BTreeSet<&str> = disk_entries.keys().map(String::as_str).collect();
    let mut missing_sealed = Vec::new();
    for (rel, entry) in &files2 {
        if is_omitted_or_nested_code_rel(rel) {
            continue;
        }
        if !entry.optional && !disk_keys.contains(rel.as_str()) {
            missing_sealed.push(rel.clone());
        }
    }
    if !missing_sealed.is_empty() {
        return Ok(FastCodesignVerifyResult {
            passed: false,
            detail: format!(
                "codesign verify failed: {} sealed resource(s) missing: {}",
                missing_sealed.len(),
                missing_sealed
                    .iter()
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            cdhash_short: Some(cdhash_short),
            hashed_files: 0,
            hashed_bytes: 0,
            receipt_verified_files: 0,
            receipt_verified_bytes: 0,
            elapsed_secs: t0.elapsed().as_secs_f64(),
        });
    }

    // Step 4: Partition regular files into:
    //   - `receipt_verified` (only `Resources/Game/*` when `!full_rehash` and `(cdhash, cr_sha, dev, ino, size, mtime, hash2)` all match)
    //   - `to_hash` (all non-`Game/` files ALWAYS, plus any uncached/changed/full-rehash `Game/` file)
    let active_receipt = if !full_rehash {
        cache_dir
            .and_then(|d| load_receipt(d, &cd.cdhash_full_hex))
            .filter(|r| {
                r.cdhash_full_hex == cd.cdhash_full_hex
                    && r.coderesources_sha256_hex == cr_sha_hex
            })
    } else {
        None
    };

    struct HashJob {
        rel: String,
        full_path: PathBuf,
        expected: [u8; 32],
        stat: Option<FileStatKey>,
        size: u64,
    }

    let mut to_hash: Vec<HashJob> = Vec::new();
    let mut receipt_verified_files = 0usize;
    let mut receipt_verified_bytes = 0u64;
    let mut game_receipt_entries: BTreeMap<String, ReceiptRecord> = BTreeMap::new();

    for (rel, &is_sym) in &disk_entries {
        if is_sym {
            continue;
        }
        let Some(expected) = files2.get(rel).and_then(|e| e.hash2) else {
            continue;
        };
        let full_path = contents_dir.join(rel);
        let stat = stat_file_key(&full_path).ok();
        let size = stat.as_ref().map_or(0, |s| s.size);

        let is_game_payload = rel.starts_with("Resources/Game/");
        if is_game_payload && !full_rehash {
            if let (Some(ref receipt), Some(ref cur_stat)) = (&active_receipt, &stat) {
                if let Some(rec) = receipt.entries.get(rel) {
                    if rec.stat == *cur_stat && rec.hash2 == expected {
                        receipt_verified_files += 1;
                        receipt_verified_bytes = receipt_verified_bytes.saturating_add(size);
                        game_receipt_entries.insert(rel.clone(), rec.clone());
                        continue;
                    }
                }
            }
        }

        to_hash.push(HashJob {
            rel: rel.clone(),
            full_path,
            expected,
            stat,
            size,
        });
    }

    // Sort largest files first for optimal parallel load balancing across worker threads.
    to_hash.sort_by(|a, b| b.size.cmp(&a.size));

    let hashed_files = to_hash.len();
    let hashed_bytes: u64 = to_hash.iter().map(|j| j.size).sum();

    // Step 5: Parallel SHA-256 verification across all cores.
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(to_hash.len().max(1));
    let next_idx = AtomicUsize::new(0);
    let mismatches: Mutex<Vec<String>> = Mutex::new(Vec::new());

    if !to_hash.is_empty() {
        std::thread::scope(|scope| {
            for _ in 0..threads {
                scope.spawn(|| loop {
                    let i = next_idx.fetch_add(1, Ordering::Relaxed);
                    let Some(job) = to_hash.get(i) else {
                        break;
                    };
                    match sha256_file(&job.full_path) {
                        Ok(got) if got == job.expected => {}
                        Ok(_) => {
                            mismatches
                                .lock()
                                .unwrap()
                                .push(format!("{} (SHA-256 mismatch)", job.rel));
                        }
                        Err(e) => {
                            mismatches
                                .lock()
                                .unwrap()
                                .push(format!("{} (read error: {e})", job.rel));
                        }
                    }
                });
            }
        });
    }

    let mut mismatches = mismatches.into_inner().unwrap();
    mismatches.sort();
    let elapsed = t0.elapsed().as_secs_f64();

    if !mismatches.is_empty() {
        return Ok(FastCodesignVerifyResult {
            passed: false,
            detail: format!(
                "codesign verify failed: {} sealed resource(s) modified/unreadable: {}",
                mismatches.len(),
                mismatches
                    .iter()
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            cdhash_short: Some(cdhash_short),
            hashed_files,
            hashed_bytes,
            receipt_verified_files,
            receipt_verified_bytes,
            elapsed_secs: elapsed,
        });
    }

    // All files verified! Update/save external receipt with every verified `Resources/Game/` file
    // so repeated `bundle-verify` / `bundle-export` runs on the unchanged bundle stay instant.
    if let Some(cdir) = cache_dir {
        for job in &to_hash {
            if job.rel.starts_with("Resources/Game/") {
                if let Some(ref st) = job.stat {
                    game_receipt_entries.insert(
                        job.rel.clone(),
                        ReceiptRecord {
                            stat: st.clone(),
                            hash2: job.expected,
                        },
                    );
                }
            }
        }
        if !game_receipt_entries.is_empty() {
            let _ = write_receipt(
                cdir,
                &VerifyReceipt {
                    cdhash_full_hex: cd.cdhash_full_hex,
                    coderesources_sha256_hex: cr_sha_hex,
                    entries: game_receipt_entries,
                },
            );
        }
    }

    let detail = if receipt_verified_files > 0 {
        format!(
            "codesign --verify --deep --strict --ignore-resources OK + CodeResources slot-3 OK; hashed {hashed_files} file(s) ({:.2} MiB) + {receipt_verified_files} Game/ file(s) ({:.2} GiB) unchanged since CDHash {cdhash_short} receipt ({elapsed:.2}s; pass --full to re-hash)",
            hashed_bytes as f64 / (1024.0 * 1024.0),
            receipt_verified_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
        )
    } else {
        let size_desc = if hashed_bytes >= 1024 * 1024 * 1024 {
            format!("{:.2} GiB", hashed_bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        } else {
            format!("{:.2} MiB", hashed_bytes as f64 / (1024.0 * 1024.0))
        };
        format!(
            "codesign --verify --deep --strict --ignore-resources OK + CodeResources slot-3 OK + parallel SHA-256 verified {hashed_files} sealed file(s) ({size_desc}) in {elapsed:.2}s ({threads} thread(s))"
        )
    };

    Ok(FastCodesignVerifyResult {
        passed: true,
        detail,
        cdhash_short: Some(cdhash_short),
        hashed_files,
        hashed_bytes,
        receipt_verified_files,
        receipt_verified_bytes,
        elapsed_secs: elapsed,
    })
}

fn fallback_apple_codesign_verify(app_path: &Path, t0: Instant) -> Result<FastCodesignVerifyResult> {
    let out = codesign_verify_deep_strict(app_path)?;
    let elapsed = t0.elapsed().as_secs_f64();
    if out.status.success() {
        Ok(FastCodesignVerifyResult {
            passed: true,
            detail: format!("codesign --verify --deep --strict OK ({elapsed:.2}s)"),
            cdhash_short: None,
            hashed_files: 0,
            hashed_bytes: 0,
            receipt_verified_files: 0,
            receipt_verified_bytes: 0,
            elapsed_secs: elapsed,
        })
    } else {
        Ok(FastCodesignVerifyResult {
            passed: false,
            detail: format!(
                "codesign verify failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
            cdhash_short: None,
            hashed_files: 0,
            hashed_bytes: 0,
            receipt_verified_files: 0,
            receipt_verified_bytes: 0,
            elapsed_secs: elapsed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle_common::{codesign_ad_hoc, codesign_ad_hoc_checked, TempDirGuard};

    #[test]
    fn sha256_matches_standard_test_vectors() {
        assert_eq!(
            hex_encode(&sha256_bytes(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex_encode(&sha256_bytes(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn parses_code_resources_files2_hashes_symlinks_and_xml_entities() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
  <key>files2</key>
  <dict>
    <key>Resources/Game/a &amp; b.bin</key>
    <dict>
      <key>hash2</key>
      <data>
      ungWv48Bz+pBQUDeXa4iI7ADYaOWF3qctBD/YfIAFa0=
      </data>
    </dict>
    <key>Resources/Game/link.bin</key>
    <dict>
      <key>symlink</key>
      <string>a &amp; b.bin</string>
    </dict>
    <key>Resources/en.lproj/InfoPlist.strings</key>
    <dict>
      <key>hash2</key>
      <data>47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=</data>
      <key>optional</key>
      <true/>
    </dict>
  </dict>
</dict>
</plist>"#;
        let map = parse_code_resources_files2(xml).unwrap();
        assert_eq!(map.len(), 3);
        let ab = map.get("Resources/Game/a & b.bin").unwrap();
        assert_eq!(
            ab.hash2.map(|h| hex_encode(&h)).as_deref(),
            Some("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        );
        assert!(!ab.optional);

        let sym = map.get("Resources/Game/link.bin").unwrap();
        assert_eq!(sym.symlink.as_deref(), Some("a & b.bin"));
        assert!(sym.hash2.is_none());

        let opt = map.get("Resources/en.lproj/InfoPlist.strings").unwrap();
        assert!(opt.optional);
        assert_eq!(
            opt.hash2.map(|h| hex_encode(&h)).as_deref(),
            Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn fast_codesign_verify_uses_external_receipt_and_catches_all_tampering() {
        let root = TempDirGuard::create_unique("port-factory-codesign-fast").unwrap();
        let receipt_cache = root.path().join("receipts");
        fs::create_dir_all(&receipt_cache).unwrap();

        let app_dir = root.path().join("SignFast.app");
        let contents = app_dir.join("Contents");
        let macos = contents.join("MacOS");
        let frameworks = contents.join("Frameworks");
        let resources = contents.join("Resources");
        let game_dir = resources.join("Game");
        fs::create_dir_all(&macos).unwrap();
        fs::create_dir_all(&frameworks).unwrap();
        fs::create_dir_all(&game_dir).unwrap();

        fs::write(
            contents.join("Info.plist"),
            crate::steam_library::generate_info_plist("SignFast", "SignFast", "1"),
        )
        .unwrap();
        fs::write(resources.join("game.toml"), "[app]\nid = \"signfast\"\n").unwrap();
        fs::write(game_dir.join("asset1.bin"), vec![0x42u8; 64 * 1024]).unwrap();
        fs::write(game_dir.join("asset2.bin"), vec![0x99u8; 32 * 1024]).unwrap();
        std::os::unix::fs::symlink("asset1.bin", game_dir.join("alias.bin")).unwrap();

        let stub_rs = root.path().join("stub.rs");
        fs::write(&stub_rs, "fn main() {}\n").unwrap();
        let exe = macos.join("SignFast");
        let helper = macos.join("port-runner");
        let st = Command::new("rustc")
            .args([
                "--edition=2021",
                &stub_rs.to_string_lossy(),
                "-o",
                &exe.to_string_lossy(),
            ])
            .output()
            .unwrap();
        assert!(st.status.success(), "{}", String::from_utf8_lossy(&st.stderr));
        fs::copy(&exe, &helper).unwrap();

        // Sign nested helper first, then the `.app` bundle ONCE (never double-signing `CFBundleExecutable`).
        codesign_ad_hoc(&helper);
        codesign_ad_hoc_checked(&app_dir).unwrap();

        // Record external signing receipt and ensure Apple's own `codesign --verify --deep --strict` still passes.
        let cdhash = record_bundle_signing_receipt_in(&app_dir, Some(&receipt_cache))
            .unwrap()
            .expect("receipt cdhash");
        assert_eq!(cdhash.len(), 64);
        assert!(receipt_cache.join(format!("{cdhash}.tsv")).is_file());
        let apple_verify = codesign_verify_deep_strict(&app_dir).unwrap();
        assert!(
            apple_verify.status.success(),
            "Apple codesign --verify failed: {}",
            String::from_utf8_lossy(&apple_verify.stderr)
        );

        // 1. Default fast verify hits the receipt for `Resources/Game/*` (zero redundant payload hashes)
        //    while still hashing non-Game files (`game.toml`).
        let fast = verify_bundle_codesign_fast_in(&app_dir, false, Some(&receipt_cache)).unwrap();
        assert!(fast.passed, "fast verify failed: {}", fast.detail);
        assert_eq!(fast.receipt_verified_files, 2);
        assert_eq!(fast.hashed_files, 1); // game.toml
        assert!(fast.detail.contains("pass --full to re-hash"));

        // 2. Explicit `full_rehash = true` bypasses the receipt and hashes all 3 regular sealed files in parallel.
        let full = verify_bundle_codesign_fast_in(&app_dir, true, Some(&receipt_cache)).unwrap();
        assert!(full.passed, "full verify failed: {}", full.detail);
        assert_eq!(full.receipt_verified_files, 0);
        assert_eq!(full.hashed_files, 3);
        assert!(full.detail.contains("parallel SHA-256 verified 3 sealed file(s)"));

        // 3. Tamper with a `Game/` payload file (different size/mtime): default verify MUST detect and FAIL!
        let orig_asset1 = fs::read(game_dir.join("asset1.bin")).unwrap();
        let mut tampered = orig_asset1.clone();
        tampered[0] ^= 0xFF;
        tampered.push(0x01);
        fs::write(game_dir.join("asset1.bin"), &tampered).unwrap();
        let res_tamper = verify_bundle_codesign_fast_in(&app_dir, false, Some(&receipt_cache)).unwrap();
        assert!(
            !res_tamper.passed,
            "modified Game/ payload must fail verification even without --full"
        );
        assert!(res_tamper.detail.contains("asset1.bin"), "{}", res_tamper.detail);
        fs::write(game_dir.join("asset1.bin"), &orig_asset1).unwrap();

        // 4. Inject an extra unsealed file: MUST FAIL!
        let extra = game_dir.join("injected.txt");
        fs::write(&extra, b"UNSEALED").unwrap();
        let res_added = verify_bundle_codesign_fast_in(&app_dir, false, Some(&receipt_cache)).unwrap();
        assert!(!res_added.passed, "unsealed file must fail verification");
        assert!(res_added.detail.contains("unsealed"), "{}", res_added.detail);
        fs::remove_file(&extra).unwrap();

        // 5. Delete a sealed file: MUST FAIL!
        let orig_asset2 = fs::read(game_dir.join("asset2.bin")).unwrap();
        fs::remove_file(game_dir.join("asset2.bin")).unwrap();
        let res_missing = verify_bundle_codesign_fast_in(&app_dir, false, Some(&receipt_cache)).unwrap();
        assert!(!res_missing.passed, "missing sealed file must fail verification");
        assert!(res_missing.detail.contains("missing"), "{}", res_missing.detail);
        fs::write(game_dir.join("asset2.bin"), &orig_asset2).unwrap();

        // 6. Tamper with `game.toml` (non-Game file is always hashed): MUST FAIL!
        let orig_toml = fs::read(resources.join("game.toml")).unwrap();
        fs::write(resources.join("game.toml"), b"[app]\nid = \"tampered\"\n").unwrap();
        let res_toml = verify_bundle_codesign_fast_in(&app_dir, false, Some(&receipt_cache)).unwrap();
        assert!(!res_toml.passed, "tampered game.toml must fail verification");
        assert!(res_toml.detail.contains("game.toml"), "{}", res_toml.detail);
        fs::write(resources.join("game.toml"), &orig_toml).unwrap();

        // 7. Tamper with `CodeResources` itself: slot -3 check MUST FAIL!
        let cr_path = contents.join("_CodeSignature/CodeResources");
        let orig_cr = fs::read(&cr_path).unwrap();
        let mut bad_cr = orig_cr.clone();
        bad_cr.extend_from_slice(b"\n<!-- tampered -->\n");
        fs::write(&cr_path, &bad_cr).unwrap();
        let res_cr = verify_bundle_codesign_fast_in(&app_dir, false, Some(&receipt_cache)).unwrap();
        assert!(!res_cr.passed, "tampered CodeResources must fail slot -3 check");
        assert!(res_cr.detail.contains("slot -3"), "{}", res_cr.detail);
        fs::write(&cr_path, &orig_cr).unwrap();
    }
}
