//! Fast x86-64 PE32+ header probing and package-scoped recursive plugin discovery.
//!
//! Pure `std` + `pe-image` implementation (zero OS/GPU dependencies) shared between
//! `port-factory` (`scanner`, `aot-prewarm`, `win32-audit`, etc.) and `port-loader`
//! (`--prewarm-aot`, `--emit-static-o`, including `--no-default-features` iOS builds).

use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use pe_image::consts::{IMAGE_FILE_DLL, IMAGE_FILE_MACHINE_AMD64};

use crate::dll_map::{DllRoute, DllRoutingTable};

/// Recognized Windows PE binary extensions (executables + dynamically loaded plugins).
pub const PE_BINARY_EXTENSIONS: &[&str] = &["exe", "dll", "pyd", "node", "ocx"];

/// Recognized dynamically loadable Windows plugin library extensions (non-EXE).
pub const PLUGIN_LIBRARY_EXTENSIONS: &[&str] = &["dll", "pyd", "node", "ocx"];

/// Directory names skipped during recursive PE discovery (VCS, caches, logs, saves).
///
/// Intentionally does NOT skip `node_modules`, `plugins`, `bin`, `lib`, `ext`,
/// `resources`, or `site-packages`, since Electron (`.node`), Python (`.pyd`),
/// ActiveX (`.ocx`), and game plugins commonly reside there.
const SKIPPED_DIR_NAMES: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "__pycache__",
    "shadercache",
    "shader_cache",
    "aotcache",
    "aot_cache",
    "crashdumps",
    "logs",
    "savegames",
    "saves",
];

/// Maximum `e_lfanew` accepted by the fast header probe (guards against corrupt offsets).
const MAX_LFANEW: u64 = 16 * 1024 * 1024;

/// Summary extracted by [`probe_x64_pe_file`] reading at most 200 header bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeHeaderProbe {
    pub machine: u16,
    pub is_dll: bool,
    pub num_sections: u16,
    pub image_base: u64,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub entry_rva: u32,
}

/// Options controlling recursive x86-64 PE32+ discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeDiscoveryOptions {
    /// Maximum directory recursion depth relative to the root (0 = root dir only).
    pub max_depth: usize,
    /// Whether to include `.exe` files alongside plugin libraries (`.dll`, `.pyd`, `.node`, `.ocx`).
    pub include_exe: bool,
    /// Hard cap on returned files to avoid runaway scans on huge trees.
    pub max_files: usize,
}

impl Default for PeDiscoveryOptions {
    fn default() -> Self {
        Self {
            max_depth: 12,
            include_exe: true,
            max_files: 4096,
        }
    }
}

/// True if `path` has a recognized Windows PE binary extension (`.exe`, `.dll`, `.pyd`, `.node`, `.ocx`).
pub fn has_pe_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| {
            PE_BINARY_EXTENSIONS
                .iter()
                .any(|cand| ext.eq_ignore_ascii_case(cand))
        })
}

/// True if `path` has a Windows plugin library extension (`.dll`, `.pyd`, `.node`, `.ocx`).
pub fn has_plugin_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| {
            PLUGIN_LIBRARY_EXTENSIONS
                .iter()
                .any(|cand| ext.eq_ignore_ascii_case(cand))
        })
}

fn is_skipped_dir_name(name: &str) -> bool {
    SKIPPED_DIR_NAMES
        .iter()
        .any(|skip| name.eq_ignore_ascii_case(skip))
}

/// Fast header-only probe of a file on disk to determine whether it is a structurally
/// valid x86-64 PE32+ image.
///
/// Reads at most 200 bytes (64-byte DOS header + 136-byte PE/COFF/Optional header prefix),
/// so huge non-PE asset blobs or wrong-architecture binaries are rejected without ever
/// reading the full file into memory.
pub fn probe_x64_pe_file(path: &Path) -> Option<PeHeaderProbe> {
    let mut f = File::open(path).ok()?;
    let meta = f.metadata().ok()?;
    if !meta.is_file() {
        return None;
    }
    let file_len = meta.len();
    // Minimum size for DOS header (0x40) + PE sig (4) + COFF (20) + PE32+ opt header prefix (112).
    if file_len < 0x40 + 4 + 20 + 112 {
        return None;
    }

    let mut dos = [0u8; 64];
    f.read_exact(&mut dos).ok()?;
    if &dos[0..2] != b"MZ" {
        return None;
    }

    let e_lfanew = u32::from_le_bytes(dos[0x3C..0x40].try_into().ok()?) as u64;
    if e_lfanew < 0x40 || e_lfanew > MAX_LFANEW {
        return None;
    }
    // Need PE sig (4) + COFF header (20) + PE32+ standard & Windows fields (112) = 136 bytes.
    let hdr_end = e_lfanew.checked_add(136)?;
    if hdr_end > file_len {
        return None;
    }

    f.seek(SeekFrom::Start(e_lfanew)).ok()?;
    let mut hdr = [0u8; 136];
    f.read_exact(&mut hdr).ok()?;
    if &hdr[0..4] != b"PE\0\0" {
        return None;
    }

    let machine = u16::from_le_bytes([hdr[4], hdr[5]]);
    if machine != IMAGE_FILE_MACHINE_AMD64 {
        return None;
    }
    let num_sections = u16::from_le_bytes([hdr[6], hdr[7]]);
    let size_of_opt = u16::from_le_bytes([hdr[20], hdr[21]]) as u64;
    let characteristics = u16::from_le_bytes([hdr[22], hdr[23]]);
    if size_of_opt < 112 {
        return None;
    }

    let opt = &hdr[24..];
    let magic = u16::from_le_bytes([opt[0], opt[1]]);
    if magic != 0x020B {
        // Reject PE32 (0x10B), ROM (0x107), or garbage.
        return None;
    }

    let entry_rva = u32::from_le_bytes(opt[16..20].try_into().ok()?);
    let image_base = u64::from_le_bytes(opt[24..32].try_into().ok()?);
    let section_alignment = u32::from_le_bytes(opt[32..36].try_into().ok()?);
    let file_alignment = u32::from_le_bytes(opt[36..40].try_into().ok()?);
    let size_of_image = u32::from_le_bytes(opt[56..60].try_into().ok()?);
    let size_of_headers = u32::from_le_bytes(opt[60..64].try_into().ok()?);
    let num_rva_and_sizes = u32::from_le_bytes(opt[108..112].try_into().ok()?);

    if section_alignment == 0
        || file_alignment == 0
        || size_of_image == 0
        || size_of_headers == 0
        || num_rva_and_sizes > 0x100
    {
        return None;
    }

    // Verify the optional header and section header table fit within the file (catches truncated PEs).
    let sec_table_start = e_lfanew.checked_add(24)?.checked_add(size_of_opt)?;
    let sec_table_bytes = (num_sections as u64).checked_mul(40)?;
    let sec_table_end = sec_table_start.checked_add(sec_table_bytes)?;
    if sec_table_end > file_len || (size_of_headers as u64) > file_len {
        return None;
    }

    Some(PeHeaderProbe {
        machine,
        is_dll: (characteristics & IMAGE_FILE_DLL) != 0,
        num_sections,
        image_base,
        size_of_image,
        size_of_headers,
        entry_rva,
    })
}

/// Returns `true` if `path` is a valid x86-64 PE32+ binary (fast header-only probe).
pub fn is_valid_x64_pe_file(path: &Path) -> bool {
    probe_x64_pe_file(path).is_some()
}

/// Recursively collect valid x86-64 PE32+ binaries within `target`, strictly scoped
/// to `target` (bounded depth, no symlink loops, no escaping the root directory).
///
/// Only files with recognized extensions (`.exe` when `opts.include_exe` is true, plus
/// `.dll`, `.pyd`, `.node`, `.ocx`) are opened, and each candidate is validated using
/// [`probe_x64_pe_file`] (reading at most 200 bytes).
pub fn collect_x64_pe_files(target: &Path, opts: &PeDiscoveryOptions) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !target.exists() {
        return Ok(out);
    }
    if target.is_file() {
        if is_valid_x64_pe_file(target) {
            out.push(target.to_path_buf());
        }
        return Ok(out);
    }
    if !target.is_dir() {
        return Ok(out);
    }

    let root_canon = fs::canonicalize(target)?;
    let mut visited_dirs: HashSet<PathBuf> = HashSet::new();
    let mut seen_files: HashSet<PathBuf> = HashSet::new();
    visited_dirs.insert(root_canon.clone());

    walk_pe_dir(
        target,
        &root_canon,
        0,
        opts,
        &mut visited_dirs,
        &mut seen_files,
        &mut out,
    );
    out.sort();
    Ok(out)
}

fn walk_pe_dir(
    dir: &Path,
    root_canon: &Path,
    depth: usize,
    opts: &PeDiscoveryOptions,
    visited_dirs: &mut HashSet<PathBuf>,
    seen_files: &mut HashSet<PathBuf>,
    out: &mut Vec<PathBuf>,
) {
    if out.len() >= opts.max_files {
        return;
    }
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = rd.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        if out.len() >= opts.max_files {
            break;
        }
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        let p = entry.path();
        if ft.is_symlink() {
            // Never traverse directory symlinks (prevents loops and root escape).
            // File symlinks are accepted only if their canonical target stays inside `root_canon`.
            let Ok(canon) = fs::canonicalize(&p) else {
                continue;
            };
            if !canon.starts_with(root_canon) || !canon.is_file() {
                continue;
            }
            let ext_ok = if opts.include_exe {
                has_pe_extension(&p)
            } else {
                has_plugin_extension(&p)
            };
            if ext_ok && seen_files.insert(canon) && is_valid_x64_pe_file(&p) {
                out.push(p);
            }
            continue;
        }
        if ft.is_dir() {
            if depth >= opts.max_depth {
                continue;
            }
            if let Some(name) = entry.file_name().to_str() {
                if is_skipped_dir_name(name) {
                    continue;
                }
            }
            let Ok(canon) = fs::canonicalize(&p) else {
                continue;
            };
            if canon.starts_with(root_canon) && visited_dirs.insert(canon) {
                walk_pe_dir(
                    &p,
                    root_canon,
                    depth + 1,
                    opts,
                    visited_dirs,
                    seen_files,
                    out,
                );
            }
            continue;
        }
        if ft.is_file() {
            let ext_ok = if opts.include_exe {
                has_pe_extension(&p)
            } else {
                has_plugin_extension(&p)
            };
            if !ext_ok {
                continue;
            }
            if !is_valid_x64_pe_file(&p) {
                continue;
            }
            let dedup_key = fs::canonicalize(&p).unwrap_or_else(|_| p.clone());
            if dedup_key.starts_with(root_canon) && seen_files.insert(dedup_key) {
                out.push(p);
            }
        }
    }
}

/// Resolve a manifest-declared relative path strictly within `package_root`.
/// Rejects absolute paths, path traversal escaping `package_root`, and broken links.
fn resolve_scoped_rel(package_root: &Path, root_canon: &Path, rel: &str) -> Option<PathBuf> {
    let trimmed = rel.trim();
    if trimmed.is_empty() {
        return None;
    }
    let normalized = trimmed.replace('\\', "/");
    let rel_path = Path::new(&normalized);
    if rel_path.is_absolute() {
        return None;
    }
    // Also reject Windows drive-prefixed paths like `C:/foo`.
    let bytes = normalized.as_bytes();
    if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        return None;
    }
    let joined = package_root.join(rel_path);
    let canon = fs::canonicalize(&joined).ok()?;
    if !canon.starts_with(root_canon) {
        return None;
    }
    Some(canon)
}

/// Discover bundled x86-64 PE32+ plugins within `package_root`, combining:
/// 1. Explicitly declared `bundled_plugins` (relative file paths inside `package_root`),
/// 2. Explicitly declared `plugin_dirs` (relative directories inside `package_root`), and
/// 3. Automatic bounded recursive discovery of `.dll`, `.pyd`, `.node`, and `.ocx` files
///    within `package_root`.
///
/// Honors `routing` (when provided) by excluding `Disabled` DLLs and (when `has_builtin`
/// confirms a native implementation exists) `Builtin`-routed DLLs. Deduplicates by
/// canonical path and excludes `skip_primary_exe` if provided.
pub fn discover_bundled_plugins<F>(
    package_root: &Path,
    routing: Option<&DllRoutingTable>,
    has_builtin: F,
    plugin_dirs: &[String],
    bundled_plugins: &[String],
    skip_primary_exe: Option<&Path>,
) -> Vec<PathBuf>
where
    F: Fn(&str) -> bool,
{
    let Ok(root_canon) = fs::canonicalize(package_root) else {
        return Vec::new();
    };
    if !root_canon.is_dir() {
        return Vec::new();
    }

    let primary_canon = skip_primary_exe.and_then(|p| fs::canonicalize(p).ok());
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut candidates: Vec<PathBuf> = Vec::new();

    let mut push_candidate = |p: PathBuf| {
        let canon = fs::canonicalize(&p).unwrap_or_else(|_| p.clone());
        if !canon.starts_with(&root_canon) || !canon.is_file() {
            return;
        }
        if primary_canon.as_ref().is_some_and(|pe| pe == &canon) {
            return;
        }
        if seen.insert(canon.clone()) {
            candidates.push(canon);
        }
    };

    // 1. Explicitly declared bundled plugin files (accept any valid x64 PE32+ within root).
    for rel in bundled_plugins {
        if let Some(canon) = resolve_scoped_rel(package_root, &root_canon, rel) {
            if canon.is_file() && is_valid_x64_pe_file(&canon) {
                push_candidate(canon);
            }
        }
    }

    let lib_opts = PeDiscoveryOptions {
        include_exe: false,
        ..PeDiscoveryOptions::default()
    };

    // 2. Explicitly declared plugin directories within root.
    for rel_dir in plugin_dirs {
        if let Some(dir_canon) = resolve_scoped_rel(package_root, &root_canon, rel_dir) {
            if dir_canon.is_dir() {
                if let Ok(found) = collect_x64_pe_files(&dir_canon, &lib_opts) {
                    for p in found {
                        push_candidate(p);
                    }
                }
            }
        }
    }

    // 3. Automatic package-scoped recursive plugin discovery.
    if let Ok(found) = collect_x64_pe_files(&root_canon, &lib_opts) {
        for p in found {
            push_candidate(p);
        }
    }

    // Filter against routing table (`Disabled` / `Builtin` with available builtin).
    candidates.retain(|p| {
        let Some(tbl) = routing else {
            return true;
        };
        let fname = p
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        let norm = pe_image::normalize_dll_name(fname);
        match tbl.route(&norm) {
            DllRoute::Disabled => false,
            DllRoute::Builtin { .. } => {
                let target = tbl.resolve_target_dll(&norm);
                !has_builtin(&target)
            }
            DllRoute::Alias(_) => {
                let target = tbl.resolve_target_dll(&norm);
                if tbl.is_disabled(&target) {
                    return false;
                }
                !(tbl.is_builtin_override(&target) && has_builtin(&target))
            }
            DllRoute::Guest | DllRoute::GuestOnly => true,
        }
    });

    candidates.sort();
    candidates
}
