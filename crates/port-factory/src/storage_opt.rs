//! Universal, game-agnostic storage optimizer for Windows x86-64 game directories.
//!
//! Available both as a standalone CLI command (`port-factory optimize-assets`) and as an
//! opt-in packaging step in `port-factory ios-bundle --optimize-storage <profile>`:
//!
//! 1. **Unused PC DLL stripping** (`--strip-unused-dlls`): omits Windows-only GPU vendor
//!    SDKs (DLSS/FSR/Streamline/NRD/NVAPI), D3D12 redistributables, and middleware already
//!    replaced by native `win32-rt` builtins, while always preserving DLLs compiled into
//!    static `.aot.o` modules, negative `builtin_overrides`, and `[storage].keep_dlls`.
//! 2. **Hardware HEVC video transcoding** (`--hevc-videos`): converts loose Bink 2 (`KB2f..KB2n`)
//!    `.bk2` / video streams into universal `KB2v` hardware-HEVC streams decoded on Apple's
//!    `VideoToolbox` Media Engine (`0%` CPU) via `win32-rt`'s `bink2w64.dll`.
//! 3. **Lossless DDS mipmap capping** (`--max-tex-dim <N>`): slices top mip levels from
//!    standard 2D `DDS` (DX10 & legacy DXT) textures without BCn re-encoding, reducing both
//!    disk footprint and iOS Jetsam GPU VRAM.
//! 4. **Declarative game-archive delegation**: when a game's `game.toml` defines a
//!    `[storage]` section with `archive_repacker` (e.g. `aw2-data` for Northlight
//!    `.rmdtoc`/`.rmdblob` packs), delegates packed archives to that external tool while
//!    keeping `crates/` 100% game-agnostic.

use anyhow::{bail, Context, Result};
use bink2_codec::hevc_container::{
    transcode_bink2_to_kb2v, HevcTranscodeOptions, TranscodeOutcome,
};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

/// Preset storage optimization profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StorageOptProfile {
    /// Disabled (copy assets untouched; default).
    #[default]
    None,
    /// Zero visual loss: strip unused PC DLLs, filter non-selected languages, repack archive slack.
    Lossless,
    /// Standalone mobile high quality: Lossless + 1080p hardware HEVC video + 2K DDS mip cap.
    High,
    /// Standalone mobile compact: Lossless + 720p hardware HEVC video + 1K DDS mip cap.
    Compact,
}

impl StorageOptProfile {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "none" | "off" | "false" => Some(Self::None),
            "lossless" | "trim" => Some(Self::Lossless),
            "high" | "balanced" | "2k" => Some(Self::High),
            "compact" | "mobile" | "1k" => Some(Self::Compact),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Lossless => "lossless",
            Self::High => "high",
            Self::Compact => "compact",
        }
    }
}

/// Declarative `[storage]` configuration parsed from a game's `game.toml`.
#[derive(Debug, Clone, Default)]
pub struct ManifestStorageConfig {
    /// Lowercased DLL filenames that must always be kept when stripping unused DLLs.
    pub keep_dlls: Vec<String>,
    /// Optional external archive repacker binary name (e.g. `"aw2-data"`).
    pub archive_repacker: Option<String>,
    /// Optional relative directory handled directly by `archive_repacker` (e.g. `"data_pack2"`).
    pub archive_dir: Option<String>,
}

/// Parse `[storage]` and negative `[runtime].builtin_overrides` entries from a `game.toml` string.
pub fn parse_manifest_storage_config(toml_text: &str) -> ManifestStorageConfig {
    let mut cfg = ManifestStorageConfig::default();
    let mut section = String::new();
    for raw_line in toml_text.lines() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if let Some(sec) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            section = sec.trim().to_ascii_lowercase();
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let key = k.trim().to_ascii_lowercase();
        let val = v.trim();
        let qualified = if section.is_empty() {
            key
        } else {
            format!("{section}.{key}")
        };
        match qualified.as_str() {
            "storage.keep_dlls" => {
                for item in parse_toml_string_list(val) {
                    let norm = item.to_ascii_lowercase();
                    if !norm.is_empty() && !cfg.keep_dlls.contains(&norm) {
                        cfg.keep_dlls.push(norm);
                    }
                }
            }
            "storage.archive_repacker" => {
                let s = val.trim_matches('"').trim_matches('\'').trim();
                if !s.is_empty() {
                    cfg.archive_repacker = Some(s.to_string());
                }
            }
            "storage.archive_dir" => {
                let s = val
                    .trim_matches('"')
                    .trim_matches('\'')
                    .trim()
                    .trim_matches('/')
                    .trim_matches('\\');
                if !s.is_empty() {
                    cfg.archive_dir = Some(s.replace('\\', "/"));
                }
            }
            "runtime.builtin_overrides" | "builtin_overrides" => {
                // Negative entries (`-physx3_x64.dll`) explicitly force the shipping guest DLL!
                let inner = val.trim_matches('"').trim_matches('\'');
                for tok in inner.split(',') {
                    let t = tok.trim();
                    if let Some(guest_dll) = t.strip_prefix('-') {
                        let norm = guest_dll.trim().to_ascii_lowercase();
                        if !norm.is_empty() && !cfg.keep_dlls.contains(&norm) {
                            cfg.keep_dlls.push(norm);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    cfg
}

fn parse_toml_string_list(raw: &str) -> Vec<String> {
    let trimmed = raw.trim();
    if let Some(inner) = trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        inner
            .split(',')
            .map(|s| s.trim().trim_matches('"').trim_matches('\'').trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        trimmed
            .trim_matches('"')
            .trim_matches('\'')
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

/// Complete options for [`optimize_game_assets`].
#[derive(Debug, Clone)]
pub struct StorageOptOptions {
    pub profile: StorageOptProfile,
    /// Transcode eligible Bink 2 (`KB2f..KB2n`) videos into `KB2v` hardware HEVC.
    pub hevc_videos: bool,
    /// Max vertical resolution for transcoded HEVC videos (`0` = keep original height).
    pub video_max_height: u32,
    /// Target bitrate in kbps for 1080p HEVC video (scaled proportionally for other sizes).
    pub video_bitrate_kbps: u32,
    /// Keyframe interval (GOP) in frames for transcoded HEVC videos.
    pub video_gop: u32,
    /// Cap loose/archived 2D DDS texture dimensions to at most `max_tex_dim` (`0` = disabled).
    pub max_tex_dim: u32,
    /// Omit unused Windows-only GPU vendor / debug / builtin-overridden DLLs.
    pub strip_unused_dlls: bool,
    /// Optional language code to keep (e.g. `"en"`), dropping other language packs.
    pub keep_lang: Option<String>,
    /// Optional: strip/stub optional DLC video assets when supported by the game repacker.
    pub strip_dlc: bool,
    /// Delegate packed game archives to `archive_repacker` when declared in `game.toml`.
    pub repack_archives: bool,
    /// Parallel worker threads for video transcoding and texture processing.
    pub jobs: usize,
    /// Report predicted actions without writing files to disk.
    pub dry_run: bool,
    /// Explicit DLL filenames (lowercased) that must never be stripped.
    pub keep_dlls: BTreeSet<String>,
    /// Optional archive repacker binary name from `[storage].archive_repacker`.
    pub archive_repacker: Option<String>,
    /// Optional archive subdir from `[storage].archive_dir`.
    pub archive_dir: Option<String>,
    /// Workspace root used to locate or build workspace repacker binaries.
    pub workspace_root: Option<PathBuf>,
}

impl Default for StorageOptOptions {
    fn default() -> Self {
        Self::from_profile(StorageOptProfile::None)
    }
}

impl StorageOptOptions {
    pub fn from_profile(profile: StorageOptProfile) -> Self {
        let jobs = std::thread::available_parallelism()
            .map(|n| n.get().clamp(1, 8))
            .unwrap_or(4);
        match profile {
            StorageOptProfile::None => Self {
                profile,
                hevc_videos: false,
                video_max_height: 1080,
                video_bitrate_kbps: 5000,
                video_gop: 60,
                max_tex_dim: 0,
                strip_unused_dlls: false,
                keep_lang: None,
                strip_dlc: false,
                repack_archives: false,
                jobs,
                dry_run: false,
                keep_dlls: BTreeSet::new(),
                archive_repacker: None,
                archive_dir: None,
                workspace_root: None,
            },
            StorageOptProfile::Lossless => Self {
                profile,
                hevc_videos: false,
                video_max_height: 1080,
                video_bitrate_kbps: 5000,
                video_gop: 60,
                max_tex_dim: 0,
                strip_unused_dlls: true,
                keep_lang: Some("en".to_string()),
                strip_dlc: false,
                repack_archives: true,
                jobs,
                dry_run: false,
                keep_dlls: BTreeSet::new(),
                archive_repacker: None,
                archive_dir: None,
                workspace_root: None,
            },
            StorageOptProfile::High => Self {
                profile,
                hevc_videos: true,
                video_max_height: 1080,
                video_bitrate_kbps: 5000,
                video_gop: 60,
                max_tex_dim: 2048,
                strip_unused_dlls: true,
                keep_lang: Some("en".to_string()),
                strip_dlc: false,
                repack_archives: true,
                jobs,
                dry_run: false,
                keep_dlls: BTreeSet::new(),
                archive_repacker: None,
                archive_dir: None,
                workspace_root: None,
            },
            StorageOptProfile::Compact => Self {
                profile,
                hevc_videos: true,
                video_max_height: 720,
                video_bitrate_kbps: 3000,
                video_gop: 60,
                max_tex_dim: 1024,
                strip_unused_dlls: true,
                keep_lang: Some("en".to_string()),
                strip_dlc: false,
                repack_archives: true,
                jobs,
                dry_run: false,
                keep_dlls: BTreeSet::new(),
                archive_repacker: None,
                archive_dir: None,
                workspace_root: None,
            },
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.profile != StorageOptProfile::None
            || self.hevc_videos
            || self.max_tex_dim > 0
            || self.strip_unused_dlls
            || self.keep_lang.is_some()
            || self.strip_dlc
            || self.repack_archives
    }

    pub fn apply_manifest_config(&mut self, cfg: &ManifestStorageConfig) {
        for d in &cfg.keep_dlls {
            self.keep_dlls.insert(d.to_ascii_lowercase());
        }
        if self.archive_repacker.is_none() {
            self.archive_repacker = cfg.archive_repacker.clone();
        }
        if self.archive_dir.is_none() {
            self.archive_dir = cfg.archive_dir.clone();
        }
    }
}

/// Summary report returned by [`optimize_game_assets`].
#[derive(Debug, Clone, Default)]
pub struct StorageOptReport {
    pub files_copied: usize,
    pub dlls_stripped: usize,
    pub dll_bytes_saved: u64,
    pub videos_transcoded: usize,
    pub video_bytes_before: u64,
    pub video_bytes_after: u64,
    pub textures_capped: usize,
    pub texture_bytes_before: u64,
    pub texture_bytes_after: u64,
    pub archive_repacker_ran: bool,
    pub total_bytes_before: u64,
    pub total_bytes_after: u64,
}

impl StorageOptReport {
    pub fn total_bytes_saved(&self) -> u64 {
        self.total_bytes_before.saturating_sub(self.total_bytes_after)
    }
}

/// Decide whether a PC `.dll` file can be safely omitted on Apple Silicon when
/// `--strip-unused-dlls` is enabled.
///
/// Any DLL listed in `keep_dlls` (e.g. compiled into static `.aot.o`, declared in
/// `[storage].keep_dlls`, or forced guest via `-foo.dll` in `builtin_overrides`) is ALWAYS kept.
pub fn should_strip_pc_dll(file_name: &str, keep_dlls: &BTreeSet<String>) -> bool {
    let lower = file_name.to_ascii_lowercase();
    if !lower.ends_with(".dll") {
        return false;
    }
    if keep_dlls.contains(&lower) {
        return false;
    }

    // Exact PC-only GPU vendor / debug / runtime-builtin-superseded DLLs.
    const EXACT_STRIP: &[&str] = &[
        "d3d12.dll",
        "d3d12core.dll",
        "d3d12sdklayers.dll",
        "dxcompiler.dll",
        "dxil.dll",
        "dstorage.dll",
        "dstoragecore.dll",
        "nrd.dll",
        "nri.dll",
        "nv_cluster_builder.dll",
        "nvapi64.dll",
        "nvcuda.dll",
        "opencl.dll",
        "bink2w64.dll",
        "eossdk-win64-shipping.dll",
        "xaudio2_9redist.dll",
        "libcurl.dll",
        "libprotobuf.dll",
        "zlib1.dll",
        "v8_zlib.dll",
        "nvcloth_x64.dll",
        "apex_clothing_x64.dll",
        "apexframework_x64.dll",
    ];
    if EXACT_STRIP.contains(&lower.as_str()) {
        return true;
    }

    // Prefix families for NVIDIA DLSS/Streamline/NVTX, AMD FSR/AGS, Razer Chroma, D3DCompiler.
    const PREFIX_STRIP: &[&str] = &[
        "nvngx_",
        "sl.",
        "nvtoolsext",
        "amd_fidelityfx_",
        "amd_ags_",
        "cchromaeditorlibrary",
        "d3dcompiler_",
    ];
    PREFIX_STRIP.iter().any(|p| lower.starts_with(p))
}

/// Result of lossless 2D DDS mipmap capping ([`cap_dds_mipmaps`]).
#[derive(Debug, Clone)]
pub struct DdsCapResult {
    pub bytes: Vec<u8>,
    pub orig_width: u32,
    pub orig_height: u32,
    pub new_width: u32,
    pub new_height: u32,
    pub orig_mips: u32,
    pub new_mips: u32,
    pub mips_dropped: u32,
    pub bytes_dropped: usize,
}

fn dds_format_metrics(bytes: &[u8]) -> Option<(usize, bool, u32)> {
    if bytes.len() < 128 || &bytes[0..4] != b"DDS " {
        return None;
    }
    let pf_flags = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
    let fourcc = &bytes[84..88];
    let rgb_bits = u32::from_le_bytes(bytes[88..92].try_into().unwrap());

    // DDPF_FOURCC = 0x4
    if (pf_flags & 0x4) != 0 {
        if fourcc == b"DX10" {
            if bytes.len() < 148 {
                return None;
            }
            let dxgi = u32::from_le_bytes(bytes[128..132].try_into().unwrap());
            let rdim = u32::from_le_bytes(bytes[132..136].try_into().unwrap());
            let misc = u32::from_le_bytes(bytes[136..140].try_into().unwrap());
            let arr = u32::from_le_bytes(bytes[140..144].try_into().unwrap());
            // Only standard single-slice 2D textures (rdim == 3, not cubemap, arraySize == 1).
            if rdim != 3 || (misc & 0x4) != 0 || arr != 1 {
                return None;
            }
            let (is_bc, bpe) = match dxgi {
                70 | 71 | 72 | 79 | 80 | 81 => (true, 8),               // BC1, BC4
                73 | 74 | 75 | 76 | 77 | 78 | 82 | 83 | 84 => (true, 16), // BC2, BC3, BC5
                94 | 95 | 96 | 97 | 98 | 99 => (true, 16),               // BC6H, BC7
                60 | 61 | 62 | 63 | 64 | 65 => (false, 1),               // R8 / A8
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 => (false, 2), // R16/RG8
                23..=43 | 87 | 88 | 90 | 91 | 93 => (false, 4),          // RGBA8/BGRA8/R32/RG16
                9..=14 => (false, 8),                                    // RGBA16
                1..=4 => (false, 16),                                    // RGBA32
                _ => return None,
            };
            return Some((148, is_bc, bpe));
        }
        let (is_bc, bpe) = match fourcc {
            b"DXT1" | b"BC4U" | b"BC4S" | b"ATI1" => (true, 8),
            b"DXT2" | b"DXT3" | b"DXT4" | b"DXT5" | b"BC5U" | b"BC5S" | b"ATI2" => (true, 16),
            _ => return None,
        };
        return Some((128, is_bc, bpe));
    }

    // Uncompressed RGB/Luminance/Alpha legacy DDS
    if rgb_bits >= 8 && rgb_bits <= 128 && (rgb_bits % 8) == 0 {
        return Some((128, false, rgb_bits / 8));
    }
    None
}

#[inline]
fn mip_byte_size(w: u32, h: u32, is_bc: bool, bpe: u32) -> (u32, usize) {
    if is_bc {
        let bw = w.div_ceil(4).max(1);
        let bh = h.div_ceil(4).max(1);
        let linear = bw.saturating_mul(bh).saturating_mul(bpe);
        (linear, linear as usize)
    } else {
        let pitch = w.saturating_mul(bpe);
        (pitch, (pitch as usize).saturating_mul(h as usize))
    }
}

/// Losslessly cap a 2D DDS (DX10 or legacy DXT) texture so `max(width, height) <= max_dim`
/// by slicing off top mip levels and updating the DDS header — zero BCn re-encoding.
///
/// Returns `None` if the texture is already within `max_dim`, has `<= 1` mip level, is a
/// cubemap / 3D volume / array texture, or uses an unrecognized format.
pub fn cap_dds_mipmaps(bytes: &[u8], max_dim: u32) -> Option<DdsCapResult> {
    if max_dim == 0 || bytes.len() < 128 || &bytes[0..4] != b"DDS " {
        return None;
    }
    let orig_height = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
    let orig_width = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
    let depth = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
    let orig_mips = u32::from_le_bytes(bytes[28..32].try_into().unwrap());
    let caps2 = u32::from_le_bytes(bytes[112..116].try_into().unwrap());

    if orig_mips <= 1 || orig_width.max(orig_height) <= max_dim {
        return None;
    }
    // Skip cubemaps (DDSCAPS2_CUBEMAP = 0x200) and 3D volume textures (0x200000 / depth > 1).
    if (caps2 & 0x200) != 0 || (caps2 & 0x200000) != 0 || depth > 1 {
        return None;
    }

    let (hdr_len, is_bc, bpe) = dds_format_metrics(bytes)?;
    let payload = bytes.get(hdr_len..)?;

    let mut w = orig_width;
    let mut h = orig_height;
    let mut rem_mips = orig_mips;
    let mut mips_dropped = 0u32;
    let mut bytes_dropped = 0usize;

    while w.max(h) > max_dim && rem_mips > 1 {
        let next_w = (w / 2).max(1);
        let next_h = (h / 2).max(1);
        if is_bc && (next_w < 4 || next_h < 4) {
            break;
        }
        let (_, mip_sz) = mip_byte_size(w, h, is_bc, bpe);
        bytes_dropped = bytes_dropped.checked_add(mip_sz)?;
        w = next_w;
        h = next_h;
        rem_mips -= 1;
        mips_dropped += 1;
    }

    if mips_dropped == 0 || bytes_dropped == 0 {
        return None;
    }
    let (new_pitch_or_linear, new_top_sz) = mip_byte_size(w, h, is_bc, bpe);
    if bytes_dropped + new_top_sz > payload.len() {
        return None;
    }

    let mut out = Vec::with_capacity(hdr_len + (payload.len() - bytes_dropped));
    out.extend_from_slice(&bytes[..hdr_len]);
    out.extend_from_slice(&payload[bytes_dropped..]);

    // Update DDS_HEADER fields: dwHeight (+12), dwWidth (+16), dwPitchOrLinearSize (+20), dwMipMapCount (+28).
    out[12..16].copy_from_slice(&h.to_le_bytes());
    out[16..20].copy_from_slice(&w.to_le_bytes());
    out[20..24].copy_from_slice(&new_pitch_or_linear.to_le_bytes());
    out[28..32].copy_from_slice(&rem_mips.to_le_bytes());

    Some(DdsCapResult {
        bytes: out,
        orig_width,
        orig_height,
        new_width: w,
        new_height: h,
        orig_mips,
        new_mips: rem_mips,
        mips_dropped,
        bytes_dropped,
    })
}

fn find_or_build_repacker(workspace_root: Option<&Path>, bin_name: &str) -> Result<PathBuf> {
    if let Some(root) = workspace_root {
        for rel in [
            format!("target/aarch64-apple-darwin/release/{bin_name}"),
            format!("target/release/{bin_name}"),
            format!("target/aarch64-apple-darwin/debug/{bin_name}"),
            format!("target/debug/{bin_name}"),
        ] {
            let p = root.join(rel);
            if p.is_file() {
                return Ok(p);
            }
        }
        // Build the workspace repacker binary automatically in release mode for speed.
        let status = Command::new("cargo")
            .current_dir(root)
            .args([
                "build",
                "--release",
                "--bin",
                bin_name,
                "--target",
                "aarch64-apple-darwin",
            ])
            .status()
            .with_context(|| format!("building archive repacker binary `{bin_name}`"))?;
        if status.success() {
            let built = root.join(format!("target/aarch64-apple-darwin/release/{bin_name}"));
            if built.is_file() {
                return Ok(built);
            }
        }
    }
    // Fallback to PATH lookup.
    Ok(PathBuf::from(bin_name))
}

fn dir_size_bytes(dir: &Path) -> u64 {
    let mut sum = 0u64;
    let Ok(rd) = fs::read_dir(dir) else {
        return 0;
    };
    for entry in rd.flatten() {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_symlink() {
            continue;
        }
        let p = entry.path();
        if ft.is_dir() {
            sum = sum.saturating_add(dir_size_bytes(&p));
        } else if ft.is_file() {
            sum = sum.saturating_add(entry.metadata().map(|m| m.len()).unwrap_or(0));
        }
    }
    sum
}

/// Optimize a Windows game directory from `src_dir` into `dst_dir` according to `opts`.
///
/// Works for any Windows game directory (loose `.bk2`, `.dds`, `.dll`, etc.) and automatically
/// delegates proprietary packed archives when `[storage].archive_repacker` is declared.
pub fn optimize_game_assets(
    src_dir: &Path,
    dst_dir: &Path,
    opts: &StorageOptOptions,
    skip_canon: Option<&Path>,
) -> Result<StorageOptReport> {
    if !src_dir.is_dir() {
        bail!("source game directory not found: {}", src_dir.display());
    }
    if !opts.dry_run {
        fs::create_dir_all(dst_dir)
            .with_context(|| format!("creating {}", dst_dir.display()))?;
    }

    let delegate_archive_dir = if opts.repack_archives && opts.archive_repacker.is_some() {
        opts.archive_dir.as_deref()
    } else {
        None
    };

    // 1. Walk `src_dir` and partition files into direct-copy, DLL-strip, loose-video, and loose-DDS.
    let mut all_files: Vec<(PathBuf, PathBuf, u64)> = Vec::new();
    let mut stack: Vec<(PathBuf, String)> = vec![(src_dir.to_path_buf(), String::new())];
    while let Some((cur, rel_prefix)) = stack.pop() {
        for entry in fs::read_dir(&cur)?.flatten() {
            let name_os = entry.file_name();
            let name = name_os.to_string_lossy();
            if name == ".DepotDownloader" || name == ".DS_Store" {
                continue;
            }
            let sp = entry.path();
            let Ok(ft) = entry.file_type() else {
                continue;
            };
            if ft.is_symlink() {
                continue;
            }
            if let Some(skip) = skip_canon {
                if fs::canonicalize(&sp).ok().as_deref() == Some(skip) {
                    continue;
                }
            }
            let rel = if rel_prefix.is_empty() {
                name.to_string()
            } else {
                format!("{rel_prefix}/{name}")
            };
            if ft.is_dir() {
                if let Some(arch_dir) = delegate_archive_dir {
                    if rel.eq_ignore_ascii_case(arch_dir) {
                        // Delegated directly to `archive_repacker` below — avoid double-copying!
                        continue;
                    }
                }
                stack.push((sp, rel));
            } else if ft.is_file() {
                let len = entry.metadata().map(|m| m.len()).unwrap_or(0);
                all_files.push((sp, PathBuf::from(rel), len));
            }
        }
    }
    all_files.sort_by(|a, b| a.1.cmp(&b.1));

    let mut report = StorageOptReport::default();
    let mut work_items: Vec<(PathBuf, PathBuf, u64)> = Vec::new();

    for (sp, rel, len) in all_files {
        report.total_bytes_before = report.total_bytes_before.saturating_add(len);
        let fname = rel.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if opts.strip_unused_dlls && should_strip_pc_dll(fname, &opts.keep_dlls) {
            report.dlls_stripped += 1;
            report.dll_bytes_saved = report.dll_bytes_saved.saturating_add(len);
            continue;
        }
        work_items.push((sp, rel, len));
    }

    // 2. Process loose files in parallel (HEVC Bink2 transcoding + DDS mip capping + copy).
    let idx_cursor = AtomicUsize::new(0);
    let files_copied = AtomicUsize::new(0);
    let videos_transcoded = AtomicUsize::new(0);
    let video_before = AtomicU64::new(0);
    let video_after = AtomicU64::new(0);
    let textures_capped = AtomicUsize::new(0);
    let tex_before = AtomicU64::new(0);
    let tex_after = AtomicU64::new(0);
    let total_after = AtomicU64::new(0);
    let first_err: Mutex<Option<anyhow::Error>> = Mutex::new(None);

    let worker_count = opts.jobs.max(1).min(work_items.len().max(1));
    let hevc_opts = HevcTranscodeOptions {
        max_height: opts.video_max_height,
        bitrate_kbps: opts.video_bitrate_kbps,
        gop_size: opts.video_gop,
        ..HevcTranscodeOptions::default()
    };

    std::thread::scope(|scope| {
        for _ in 0..worker_count {
            let items = &work_items;
            let cursor = &idx_cursor;
            let files_copied = &files_copied;
            let videos_transcoded = &videos_transcoded;
            let video_before = &video_before;
            let video_after = &video_after;
            let textures_capped = &textures_capped;
            let tex_before = &tex_before;
            let tex_after = &tex_after;
            let total_after = &total_after;
            let first_err = &first_err;
            let hevc_opts = &hevc_opts;

            scope.spawn(move || loop {
                if first_err.lock().unwrap().is_some() {
                    return;
                }
                let idx = cursor.fetch_add(1, Ordering::Relaxed);
                if idx >= items.len() {
                    return;
                }
                let (ref sp, ref rel, orig_len) = items[idx];
                let dp = dst_dir.join(rel);
                if !opts.dry_run {
                    if let Some(parent) = dp.parent() {
                        if let Err(e) = fs::create_dir_all(parent) {
                            *first_err.lock().unwrap() = Some(e.into());
                            return;
                        }
                    }
                }

                let ext = rel
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();

                // Loose Bink 2 video (.bk2) -> KB2v hardware HEVC
                if opts.hevc_videos && ext == "bk2" {
                    match fs::read(sp) {
                        Ok(raw) => match transcode_bink2_to_kb2v(&raw, hevc_opts) {
                            Ok(TranscodeOutcome::Transcoded { bytes, .. }) => {
                                let new_len = bytes.len() as u64;
                                if !opts.dry_run {
                                    if let Err(e) = fs::write(&dp, &bytes) {
                                        *first_err.lock().unwrap() = Some(e.into());
                                        return;
                                    }
                                }
                                videos_transcoded.fetch_add(1, Ordering::Relaxed);
                                video_before.fetch_add(orig_len, Ordering::Relaxed);
                                video_after.fetch_add(new_len, Ordering::Relaxed);
                                total_after.fetch_add(new_len, Ordering::Relaxed);
                                files_copied.fetch_add(1, Ordering::Relaxed);
                                continue;
                            }
                            Ok(TranscodeOutcome::Skipped(_)) | Err(_) => {}
                        },
                        Err(e) => {
                            *first_err.lock().unwrap() = Some(e.into());
                            return;
                        }
                    }
                }

                // Loose DDS texture (.dds) -> lossless mipmap cap
                if opts.max_tex_dim > 0 && ext == "dds" {
                    match fs::read(sp) {
                        Ok(raw) => {
                            if let Some(capped) = cap_dds_mipmaps(&raw, opts.max_tex_dim) {
                                let new_len = capped.bytes.len() as u64;
                                if !opts.dry_run {
                                    if let Err(e) = fs::write(&dp, &capped.bytes) {
                                        *first_err.lock().unwrap() = Some(e.into());
                                        return;
                                    }
                                }
                                textures_capped.fetch_add(1, Ordering::Relaxed);
                                tex_before.fetch_add(orig_len, Ordering::Relaxed);
                                tex_after.fetch_add(new_len, Ordering::Relaxed);
                                total_after.fetch_add(new_len, Ordering::Relaxed);
                                files_copied.fetch_add(1, Ordering::Relaxed);
                                continue;
                            }
                        }
                        Err(e) => {
                            *first_err.lock().unwrap() = Some(e.into());
                            return;
                        }
                    }
                }

                // Default: copy file unchanged
                if !opts.dry_run {
                    if let Err(e) = fs::copy(sp, &dp) {
                        *first_err.lock().unwrap() = Some(e.into());
                        return;
                    }
                }
                total_after.fetch_add(orig_len, Ordering::Relaxed);
                files_copied.fetch_add(1, Ordering::Relaxed);
            });
        }
    });

    if let Some(err) = first_err.into_inner().unwrap() {
        return Err(err);
    }

    report.files_copied = files_copied.load(Ordering::Relaxed);
    report.videos_transcoded = videos_transcoded.load(Ordering::Relaxed);
    report.video_bytes_before = video_before.load(Ordering::Relaxed);
    report.video_bytes_after = video_after.load(Ordering::Relaxed);
    report.textures_capped = textures_capped.load(Ordering::Relaxed);
    report.texture_bytes_before = tex_before.load(Ordering::Relaxed);
    report.texture_bytes_after = tex_after.load(Ordering::Relaxed);
    report.total_bytes_after = total_after.load(Ordering::Relaxed);

    // 3. Delegate packed proprietary game archives to the game's declared repacker binary.
    if opts.repack_archives {
        if let Some(ref repacker_name) = opts.archive_repacker {
            let arch_src_bytes = delegate_archive_dir
                .map(|d| dir_size_bytes(&src_dir.join(d)))
                .unwrap_or(0);
            report.total_bytes_before = report.total_bytes_before.saturating_add(arch_src_bytes);

            let bin = find_or_build_repacker(opts.workspace_root.as_deref(), repacker_name)?;
            let mut cmd = Command::new(&bin);
            cmd.arg("repack")
                .arg("--src")
                .arg(src_dir)
                .arg("--dst")
                .arg(dst_dir)
                .arg("--jobs")
                .arg(opts.jobs.max(1).to_string());
            if opts.hevc_videos {
                cmd.arg("--hevc-videos")
                    .arg("--video-max-height")
                    .arg(opts.video_max_height.to_string())
                    .arg("--video-bitrate")
                    .arg(opts.video_bitrate_kbps.to_string())
                    .arg("--video-gop")
                    .arg(opts.video_gop.to_string());
            }
            if opts.max_tex_dim > 0 {
                cmd.arg("--max-tex-dim").arg(opts.max_tex_dim.to_string());
            }
            if let Some(ref lang) = opts.keep_lang {
                cmd.arg("--keep-lang").arg(lang);
            }
            if opts.strip_dlc {
                cmd.arg("--strip-dlc");
            }
            if opts.dry_run {
                cmd.arg("--dry-run");
            }

            let status = cmd
                .status()
                .with_context(|| format!("running archive repacker `{}`", bin.display()))?;
            if !status.success() {
                bail!(
                    "archive repacker `{}` exited with status {status}",
                    bin.display()
                );
            }
            report.archive_repacker_ran = true;

            if !opts.dry_run {
                let arch_dst_bytes = delegate_archive_dir
                    .map(|d| dir_size_bytes(&dst_dir.join(d)))
                    .unwrap_or(0);
                report.total_bytes_after = report.total_bytes_after.saturating_add(arch_dst_bytes);
            }
        }
    }

    Ok(report)
}

pub fn format_storage_opt_report(r: &StorageOptReport, opts: &StorageOptOptions) -> String {
    let gb = |b: u64| (b as f64) / 1_000_000_000.0;
    let mb = |b: u64| (b as f64) / 1_000_000.0;
    let mut s = String::new();
    s.push_str(&format!(
        "=== Storage Optimization Report (profile: {}{}) ===\n",
        opts.profile.as_str(),
        if opts.dry_run { ", dry-run" } else { "" }
    ));
    s.push_str(&format!("  Loose Files Staged : {}\n", r.files_copied));
    if r.dlls_stripped > 0 {
        s.push_str(&format!(
            "  PC DLLs Stripped   : {} (-{:.1} MB)\n",
            r.dlls_stripped,
            mb(r.dll_bytes_saved)
        ));
    }
    if r.videos_transcoded > 0 {
        s.push_str(&format!(
            "  Loose Videos HEVC  : {} ({:.1} MB -> {:.1} MB)\n",
            r.videos_transcoded,
            mb(r.video_bytes_before),
            mb(r.video_bytes_after)
        ));
    }
    if r.textures_capped > 0 {
        s.push_str(&format!(
            "  Loose DDS Capped   : {} ({:.1} MB -> {:.1} MB)\n",
            r.textures_capped,
            mb(r.texture_bytes_before),
            mb(r.texture_bytes_after)
        ));
    }
    if r.archive_repacker_ran {
        s.push_str(&format!(
            "  Archive Repacker   : {} (completed)\n",
            opts.archive_repacker.as_deref().unwrap_or("custom")
        ));
    }
    if !opts.dry_run && r.total_bytes_before > 0 {
        let saved = r.total_bytes_saved();
        let pct = (saved as f64) * 100.0 / (r.total_bytes_before as f64);
        s.push_str(&format!(
            "  Total Footprint    : {:.2} GB -> {:.2} GB (saved {:.2} GB / {:.1}%)\n",
            gb(r.total_bytes_before),
            gb(r.total_bytes_after),
            gb(saved),
            pct
        ));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_synthetic_dds_dx10(width: u32, height: u32, mips: u32, dxgi: u32, bpe: u32) -> Vec<u8> {
        let mut out = vec![0u8; 148];
        out[0..4].copy_from_slice(b"DDS ");
        out[4..8].copy_from_slice(&124u32.to_le_bytes());
        out[8..12].copy_from_slice(&0x000A_1007u32.to_le_bytes());
        out[12..16].copy_from_slice(&height.to_le_bytes());
        out[16..20].copy_from_slice(&width.to_le_bytes());
        let (lin0, _) = mip_byte_size(width, height, true, bpe);
        out[20..24].copy_from_slice(&lin0.to_le_bytes());
        out[24..28].copy_from_slice(&1u32.to_le_bytes()); // depth
        out[28..32].copy_from_slice(&mips.to_le_bytes());
        out[76..80].copy_from_slice(&32u32.to_le_bytes());
        out[80..84].copy_from_slice(&0x4u32.to_le_bytes()); // DDPF_FOURCC
        out[84..88].copy_from_slice(b"DX10");
        out[128..132].copy_from_slice(&dxgi.to_le_bytes());
        out[132..136].copy_from_slice(&3u32.to_le_bytes()); // Texture2D
        out[136..140].copy_from_slice(&0u32.to_le_bytes()); // miscFlag
        out[140..144].copy_from_slice(&1u32.to_le_bytes()); // arraySize

        let (mut w, mut h) = (width, height);
        for m in 0..mips {
            let (_, sz) = mip_byte_size(w, h, true, bpe);
            out.extend(std::iter::repeat_n((m + 1) as u8, sz));
            w = (w / 2).max(1);
            h = (h / 2).max(1);
        }
        out
    }

    #[test]
    fn cap_dds_mipmaps_slices_top_mips_losslessly_and_skips_single_mip() {
        // 4096x4096 BC7 (16 B/block) with 4 mips -> cap at 1024 drops mips 0 (4096) & 1 (2048).
        let dds = make_synthetic_dds_dx10(4096, 4096, 4, 98, 16);
        let mip0 = 1024 * 1024 * 16;
        let mip1 = 512 * 512 * 16;
        let mip2 = 256 * 256 * 16;
        let mip3 = 128 * 128 * 16;
        assert_eq!(dds.len(), 148 + mip0 + mip1 + mip2 + mip3);

        let res = cap_dds_mipmaps(&dds, 1024).expect("should cap 4K -> 1K");
        assert_eq!(res.new_width, 1024);
        assert_eq!(res.new_height, 1024);
        assert_eq!(res.new_mips, 2);
        assert_eq!(res.mips_dropped, 2);
        assert_eq!(res.bytes_dropped, mip0 + mip1);
        assert_eq!(res.bytes.len(), 148 + mip2 + mip3);
        // Remaining mip 0 data must be byte-exact old mip 2 (filled with 0x03).
        assert!(res.bytes[148..148 + mip2].iter().all(|&b| b == 3));
        assert!(res.bytes[148 + mip2..].iter().all(|&b| b == 4));
        // Header updated cleanly.
        assert_eq!(u32::from_le_bytes(res.bytes[12..16].try_into().unwrap()), 1024);
        assert_eq!(u32::from_le_bytes(res.bytes[16..20].try_into().unwrap()), 1024);
        assert_eq!(u32::from_le_bytes(res.bytes[28..32].try_into().unwrap()), 2);

        // Single-mip LUTs (e.g. 2304x48 mips=1) must NEVER be modified.
        let lut = make_synthetic_dds_dx10(2048, 64, 1, 98, 16);
        assert!(cap_dds_mipmaps(&lut, 1024).is_none());
    }

    #[test]
    fn strip_pc_dll_respects_keep_list_and_manifest_overrides() {
        let toml = r#"
[runtime]
builtin_overrides = "-physx3_x64.dll,-pxfoundation_x64.dll"

[storage]
keep_dlls = ["v8.dll", "cohtml.WindowsDesktop.dll"]
archive_repacker = "aw2-data"
archive_dir = "data_pack2"
"#;
        let cfg = parse_manifest_storage_config(toml);
        assert_eq!(cfg.archive_repacker.as_deref(), Some("aw2-data"));
        assert_eq!(cfg.archive_dir.as_deref(), Some("data_pack2"));

        let mut opts = StorageOptOptions::from_profile(StorageOptProfile::High);
        opts.apply_manifest_config(&cfg);

        assert!(should_strip_pc_dll("nvngx_dlss.dll", &opts.keep_dlls));
        assert!(should_strip_pc_dll("amd_fidelityfx_upscaler_dx12.dll", &opts.keep_dlls));
        assert!(should_strip_pc_dll("D3D12Core.dll", &opts.keep_dlls));
        assert!(should_strip_pc_dll("EOSSDK-Win64-Shipping.dll", &opts.keep_dlls));
        assert!(should_strip_pc_dll("bink2w64.dll", &opts.keep_dlls));
        // Explicitly kept / guest DLLs are never stripped:
        assert!(!should_strip_pc_dll("v8.dll", &opts.keep_dlls));
        assert!(!should_strip_pc_dll("PhysX3_x64.dll", &opts.keep_dlls));
        assert!(!should_strip_pc_dll("RenoirCore.WindowsDesktop.dll", &opts.keep_dlls));
    }

    #[test]
    fn optimize_game_assets_end_to_end_strips_dlls_and_caps_loose_dds() {
        let tmp = std::env::temp_dir().join(format!("port-storage-opt-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let dst = tmp.join("dst");
        fs::create_dir_all(src.join("textures")).unwrap();
        fs::write(src.join("game.exe"), b"MZdemo").unwrap();
        fs::write(src.join("v8.dll"), b"MZkeep").unwrap();
        fs::write(src.join("nvngx_dlss.dll"), vec![0u8; 4096]).unwrap();
        let dds = make_synthetic_dds_dx10(2048, 2048, 3, 98, 16);
        fs::write(src.join("textures/hero.dds"), &dds).unwrap();

        let mut opts = StorageOptOptions::from_profile(StorageOptProfile::Compact);
        opts.keep_dlls.insert("v8.dll".to_string());
        let rep = optimize_game_assets(&src, &dst, &opts, None).unwrap();

        assert_eq!(rep.dlls_stripped, 1);
        assert_eq!(rep.dll_bytes_saved, 4096);
        assert_eq!(rep.textures_capped, 1);
        assert!(dst.join("game.exe").is_file());
        assert!(dst.join("v8.dll").is_file());
        assert!(!dst.join("nvngx_dlss.dll").exists());
        let out_dds = fs::read(dst.join("textures/hero.dds")).unwrap();
        assert_eq!(u32::from_le_bytes(out_dds[16..20].try_into().unwrap()), 1024);
        assert!(rep.total_bytes_after < rep.total_bytes_before);
        let _ = fs::remove_dir_all(&tmp);
    }
}
