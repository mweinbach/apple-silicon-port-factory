//! Shared macOS and iOS bundle staging, signing, shader-cache, Mach-O inspection,
//! and temporary-directory helpers used across `steam_library`, `ios_bundle`,
//! `launcher`, and the `port-factory` CLI.

use anyhow::{bail, Context, Result};
use std::collections::{BTreeSet, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

/// RAII guard that recursively removes a temporary directory when dropped.
#[derive(Debug)]
pub struct TempDirGuard(PathBuf);

impl TempDirGuard {
    /// Wrap an existing directory path so it is deleted on drop.
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    /// Create a fresh unique directory under `std::env::temp_dir()` with `prefix`.
    pub fn create_unique(prefix: &str) -> Result<Self> {
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let dir = std::env::temp_dir().join(format!(
            "{prefix}-{}-{nanos}-{seq}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir)
            .with_context(|| format!("creating temp directory {}", dir.display()))?;
        Ok(Self(dir))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Extract the `<string>...</string>` value immediately following `<key>{key}</key>` in an XML plist.
pub fn extract_plist_string(plist_xml: &str, key: &str) -> Option<String> {
    let needle = format!("<key>{key}</key>");
    let pos = plist_xml.find(&needle)?;
    let rest = &plist_xml[pos + needle.len()..];
    let s_start = rest.find("<string>")?;
    let after = &rest[s_start + "<string>".len()..];
    let s_end = after.find("</string>")?;
    Some(after[..s_end].trim().to_string())
}

/// Set `0o755` permissions on a staged executable or dynamic library (Unix only).
pub fn set_executable_mode(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let meta = fs::metadata(path)
            .with_context(|| format!("reading metadata for {}", path.display()))?;
        let mut perms = meta.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)
            .with_context(|| format!("setting 0755 permissions on {}", path.display()))?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

/// Summary of entries copied or skipped by [`copy_dir_recursive_with_report`] and
/// [`copy_game_payload`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CopyDirReport {
    pub files_copied: usize,
    pub dirs_created: usize,
    pub symlinks_preserved: usize,
    pub symlinks_skipped_escaping: usize,
    pub symlinks_skipped_dangling: usize,
    pub symlinks_skipped_cycle: usize,
}

impl CopyDirReport {
    /// Total number of unsafe symlinks skipped during the copy.
    pub fn skipped_symlinks(&self) -> usize {
        self.symlinks_skipped_escaping
            + self.symlinks_skipped_dangling
            + self.symlinks_skipped_cycle
    }
}

struct PendingSymlink {
    src_path: PathBuf,
    dst_path: PathBuf,
    rel_path: PathBuf,
    rel_parent: PathBuf,
}

/// Normalize `rel_parent.join(raw_target)` lexically relative to the root directory.
/// Returns `None` if `raw_target` is empty, absolute, has a root/prefix component,
/// or traverses above the root directory via `..`.
fn normalize_relative_symlink_target(rel_parent: &Path, raw_target: &Path) -> Option<PathBuf> {
    if raw_target.as_os_str().is_empty() || raw_target.is_absolute() {
        return None;
    }
    let mut parts: Vec<&OsStr> = Vec::new();
    for comp in rel_parent.components().chain(raw_target.components()) {
        match comp {
            Component::Normal(s) => parts.push(s),
            Component::CurDir => {}
            Component::ParentDir => {
                parts.pop()?;
            }
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    let mut out = PathBuf::new();
    for p in parts {
        out.push(p);
    }
    Some(out)
}

fn is_symlink_loop_error(err: &std::io::Error) -> bool {
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        err.raw_os_error() == Some(62) // ELOOP
    }
    #[cfg(target_os = "linux")]
    {
        err.raw_os_error() == Some(40) // ELOOP
    }
    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "linux"
    )))]
    {
        let _ = err;
        false
    }
}

/// Returns `true` if adding a directory symlink located in `link_parent` pointing to
/// `link_target` would create a cycle in the destination directory graph given the
/// already-accepted acyclic directory symlinks `(parent_dir, target_dir)`.
fn creates_dir_cycle(
    link_parent: &Path,
    link_target: &Path,
    accepted_dir_links: &[(PathBuf, PathBuf)],
) -> bool {
    // Direct self/ancestor cycle (e.g. `loop -> .`, `sub/up -> ..`).
    if link_parent.starts_with(link_target) {
        return true;
    }
    let mut stack = vec![link_target.to_path_buf()];
    let mut visited = HashSet::new();
    visited.insert(link_target.to_path_buf());
    while let Some(cur) = stack.pop() {
        if link_parent.starts_with(&cur) {
            return true;
        }
        for (p, t) in accepted_dir_links {
            if p.starts_with(&cur) && visited.insert(t.clone()) {
                stack.push(t.clone());
            }
        }
    }
    false
}

#[allow(clippy::too_many_arguments)]
fn walk_copy_dir(
    cur_src: &Path,
    cur_dst: &Path,
    rel_parent: &Path,
    dst_root_canon: &Path,
    skip_canon: Option<&Path>,
    active_dirs: &mut HashSet<PathBuf>,
    pending_symlinks: &mut Vec<PendingSymlink>,
    report: &mut CopyDirReport,
) -> Result<()> {
    fs::create_dir_all(cur_dst)
        .with_context(|| format!("creating directory {}", cur_dst.display()))?;

    let mut entries: Vec<fs::DirEntry> = fs::read_dir(cur_src)
        .with_context(|| format!("reading directory {}", cur_src.display()))?
        .collect::<std::io::Result<Vec<_>>>()
        .with_context(|| format!("iterating directory {}", cur_src.display()))?;
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let name = entry.file_name();
        if name == ".DepotDownloader" || name == ".DS_Store" {
            continue;
        }
        let sp = entry.path();
        let dp = cur_dst.join(&name);
        let ft = entry
            .file_type()
            .with_context(|| format!("reading file type for {}", sp.display()))?;

        if ft.is_symlink() {
            // Never recurse through symlinks during directory copy; evaluate safety once
            // all real files and directories exist in `dst`.
            pending_symlinks.push(PendingSymlink {
                src_path: sp,
                dst_path: dp,
                rel_path: rel_parent.join(&name),
                rel_parent: rel_parent.to_path_buf(),
            });
            continue;
        }

        if ft.is_dir() {
            let sp_canon = fs::canonicalize(&sp)
                .with_context(|| format!("canonicalizing directory {}", sp.display()))?;
            if skip_canon.is_some_and(|skip| sp_canon == skip || sp_canon.starts_with(skip))
                || sp_canon == dst_root_canon
                || sp_canon.starts_with(dst_root_canon)
            {
                continue;
            }
            if !active_dirs.insert(sp_canon.clone()) {
                continue;
            }
            report.dirs_created += 1;
            let child_rel = rel_parent.join(&name);
            let res = walk_copy_dir(
                &sp,
                &dp,
                &child_rel,
                dst_root_canon,
                skip_canon,
                active_dirs,
                pending_symlinks,
                report,
            );
            active_dirs.remove(&sp_canon);
            res?;
        } else if ft.is_file() {
            if let Some(skip) = skip_canon {
                if fs::canonicalize(&sp).ok().as_deref() == Some(skip) {
                    continue;
                }
            }
            if dp.is_symlink() {
                let _ = fs::remove_file(&dp);
            }
            fs::copy(&sp, &dp)
                .with_context(|| format!("copying {} -> {}", sp.display(), dp.display()))?;
            report.files_copied += 1;
        }
    }
    Ok(())
}

/// Recursively copy `src` into `dst`, skipping `.DepotDownloader`, `.DS_Store`, and an optional
/// canonicalized path (`skip_canon`, e.g. an output `.app` located inside the source tree).
///
/// Symlinks are never followed during directory traversal:
/// - Dangling symlinks are skipped.
/// - Symlinks that escape `src` (or use absolute / root-escaping paths) are skipped.
/// - Symlinks that form loops (`ELOOP`, ancestor links, or mutual directory cycles) are skipped.
/// - Safe interior relative symlinks that resolve inside `src` without forming cycles are
///   preserved in `dst`.
pub fn copy_dir_recursive_with_report(
    src: &Path,
    dst: &Path,
    skip_canon: Option<&Path>,
) -> Result<CopyDirReport> {
    let src_root_canon = fs::canonicalize(src)
        .with_context(|| format!("canonicalizing source directory {}", src.display()))?;
    fs::create_dir_all(dst)
        .with_context(|| format!("creating destination directory {}", dst.display()))?;
    let dst_root_canon = fs::canonicalize(dst)
        .with_context(|| format!("canonicalizing destination directory {}", dst.display()))?;

    let mut active_dirs = HashSet::new();
    active_dirs.insert(src_root_canon.clone());
    let mut pending_symlinks = Vec::new();
    let mut report = CopyDirReport::default();

    walk_copy_dir(
        src,
        dst,
        Path::new(""),
        &dst_root_canon,
        skip_canon,
        &mut active_dirs,
        &mut pending_symlinks,
        &mut report,
    )?;

    pending_symlinks.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));

    let mut accepted_dir_links: Vec<(PathBuf, PathBuf)> = Vec::new();
    let mut created_symlinks: Vec<PathBuf> = Vec::new();

    for link in &pending_symlinks {
        let Ok(raw_target) = fs::read_link(&link.src_path) else {
            report.symlinks_skipped_dangling += 1;
            continue;
        };

        let canon_target = match fs::canonicalize(&link.src_path) {
            Ok(p) => p,
            Err(err) => {
                if is_symlink_loop_error(&err) {
                    report.symlinks_skipped_cycle += 1;
                } else {
                    report.symlinks_skipped_dangling += 1;
                }
                continue;
            }
        };

        if !canon_target.starts_with(&src_root_canon)
            || skip_canon.is_some_and(|skip| canon_target.starts_with(skip))
            || canon_target.starts_with(&dst_root_canon)
        {
            report.symlinks_skipped_escaping += 1;
            continue;
        }

        let Some(lex_target_rel) =
            normalize_relative_symlink_target(&link.rel_parent, &raw_target)
        else {
            report.symlinks_skipped_escaping += 1;
            continue;
        };

        if lex_target_rel == link.rel_path {
            report.symlinks_skipped_cycle += 1;
            continue;
        }

        let Ok(target_meta) = fs::metadata(&canon_target) else {
            report.symlinks_skipped_dangling += 1;
            continue;
        };
        if !target_meta.is_file() && !target_meta.is_dir() {
            report.symlinks_skipped_dangling += 1;
            continue;
        }

        let dst_parent_canon = dst_root_canon.join(&link.rel_parent);
        let Ok(target_rel_canon) = canon_target.strip_prefix(&src_root_canon) else {
            report.symlinks_skipped_escaping += 1;
            continue;
        };
        let dst_target_canon = dst_root_canon.join(target_rel_canon);

        if target_meta.is_dir() {
            if creates_dir_cycle(&dst_parent_canon, &dst_target_canon, &accepted_dir_links) {
                report.symlinks_skipped_cycle += 1;
                continue;
            }
        }

        #[cfg(unix)]
        {
            if link.dst_path.is_symlink() || link.dst_path.is_file() {
                let _ = fs::remove_file(&link.dst_path);
            }
            std::os::unix::fs::symlink(&raw_target, &link.dst_path).with_context(|| {
                format!(
                    "creating relative symlink {} -> {}",
                    link.dst_path.display(),
                    raw_target.display()
                )
            })?;
            if target_meta.is_dir() {
                accepted_dir_links.push((dst_parent_canon, dst_target_canon));
            }
            created_symlinks.push(link.dst_path.clone());
            report.symlinks_preserved += 1;
        }
        #[cfg(not(unix))]
        {
            let _ = (&dst_parent_canon, &dst_target_canon, &accepted_dir_links);
            report.symlinks_skipped_dangling += 1;
        }
    }

    // Post-copy fixed-point check: drop any preserved symlink in `dst` that dangles or escapes
    // `dst_root_canon` (e.g. a relative link chained through an intermediate link or directory
    // such as `.DepotDownloader` that was skipped).
    loop {
        let mut removed_any = false;
        created_symlinks.retain(|dp| {
            let ok = fs::canonicalize(dp)
                .map(|resolved| resolved.starts_with(&dst_root_canon))
                .unwrap_or(false);
            if !ok {
                let _ = fs::remove_file(dp);
                report.symlinks_preserved = report.symlinks_preserved.saturating_sub(1);
                report.symlinks_skipped_dangling += 1;
                removed_any = true;
                false
            } else {
                true
            }
        });
        if !removed_any {
            break;
        }
    }

    Ok(report)
}

/// Recursively copy `src` into `dst`, skipping `.DepotDownloader`, `.DS_Store`, unsafe symlinks,
/// and an optional canonicalized path (`skip_canon`, e.g. an output `.app` inside the source tree).
pub fn copy_dir_recursive(src: &Path, dst: &Path, skip_canon: Option<&Path>) -> Result<()> {
    copy_dir_recursive_with_report(src, dst, skip_canon)?;
    Ok(())
}

/// Replace `dst` with a fresh, self-contained copy of the game directory `src`.
///
/// Removes any stale file, symlink, or directory at `dst` first, then delegates to
/// [`copy_dir_recursive_with_report`] so escaping, dangling, and cyclic symlinks are skipped
/// while safe interior relative symlinks are preserved.
pub fn copy_game_payload(
    src: &Path,
    dst: &Path,
    skip_canon: Option<&Path>,
) -> Result<CopyDirReport> {
    if let Ok(meta) = fs::symlink_metadata(dst) {
        if meta.file_type().is_symlink() || meta.is_file() {
            fs::remove_file(dst)
                .with_context(|| format!("removing stale game payload {}", dst.display()))?;
        } else if meta.is_dir() {
            fs::remove_dir_all(dst)
                .with_context(|| format!("removing stale game payload {}", dst.display()))?;
        }
    }
    copy_dir_recursive_with_report(src, dst, skip_canon).with_context(|| {
        format!(
            "copying game assets from {} to {}",
            src.display(),
            dst.display()
        )
    })
}

/// Recursively copy non-symlink files from `src` to `dst`, skipping host-specific `*.mtl4a` files.
fn copy_dir_skip_mtl4a(src: &Path, dst: &Path) -> Result<usize> {
    fs::create_dir_all(dst)?;
    let mut count = 0usize;
    for entry in fs::read_dir(src)?.flatten() {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_symlink() {
            continue;
        }
        let src_p = entry.path();
        let dst_p = dst.join(entry.file_name());
        if ft.is_dir() {
            count += copy_dir_skip_mtl4a(&src_p, &dst_p)?;
        } else if ft.is_file() {
            if src_p
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("mtl4a"))
            {
                continue;
            }
            fs::copy(&src_p, &dst_p)?;
            count += 1;
        }
    }
    Ok(count)
}

/// Copy portable Tier-1 shader caches (`ir*`, `manifest/`, `pso/pso_manifest/`) from `src_root`
/// into `dst_root`, deliberately skipping GPU/macOS-build-specific `*.mtl4a` archives so `.app`
/// bundles stay compact and 100% cross-Mac portable. Returns total files copied.
pub fn copy_portable_shader_cache(src_root: &Path, dst_root: &Path) -> Result<usize> {
    if !src_root.is_dir() {
        return Ok(0);
    }
    fs::create_dir_all(dst_root)
        .with_context(|| format!("creating {}", dst_root.display()))?;

    let mut total = 0usize;
    for entry in fs::read_dir(src_root)?.flatten() {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_symlink() || !ft.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with("ir") || name_str == "manifest" {
            let dst_sub = dst_root.join(&name);
            let _ = fs::remove_dir_all(&dst_sub);
            total += copy_dir_skip_mtl4a(&entry.path(), &dst_sub)?;
        }
    }

    let pso_manifest_src = src_root.join("pso/pso_manifest");
    if pso_manifest_src.is_dir() {
        let pso_manifest_dst = dst_root.join("pso/pso_manifest");
        let _ = fs::remove_dir_all(&pso_manifest_dst);
        total += copy_dir_skip_mtl4a(&pso_manifest_src, &pso_manifest_dst)?;
    }

    Ok(total)
}

/// Copy only runtime-required Tier-1 IR shader caches (`ir*`, e.g. `.aw2s` / `.metallib`)
/// into an iOS `<App>.app/ShaderCache`.
///
/// iOS runs in `CacheOnly` mode and never executes offline DXIL prewarming or macOS PSO
/// manifest replay, so skipping `manifest/` (raw DXIL) and `pso/` saves ~130+ MB per bundle
/// with zero runtime impact.
pub fn copy_ios_shader_cache(src_root: &Path, dst_root: &Path) -> Result<usize> {
    if !src_root.is_dir() {
        return Ok(0);
    }
    fs::create_dir_all(dst_root)
        .with_context(|| format!("creating {}", dst_root.display()))?;

    let mut total = 0usize;
    for entry in fs::read_dir(src_root)?.flatten() {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_symlink() || !ft.is_dir() {
            continue;
        }
        let name = entry.file_name();
        if name.to_string_lossy().starts_with("ir") {
            let dst_sub = dst_root.join(&name);
            let _ = fs::remove_dir_all(&dst_sub);
            total += copy_dir_skip_mtl4a(&entry.path(), &dst_sub)?;
        }
    }

    Ok(total)
}

/// Inventory of portable shader files vs host-specific `*.mtl4a` archives inside a bundle's
/// `ShaderCache/` tree.
#[derive(Debug, Clone, Default)]
pub struct ShaderCacheScan {
    /// Count of `.aw2s` Tier-1 portable shader files.
    pub aw2s_files: usize,
    /// Count of all non-`*.mtl4a` regular files (including `.aw2s` and manifest files).
    pub portable_files: usize,
    /// Relative paths (stripped against `rel_base`) of any forbidden host-specific `*.mtl4a` files.
    pub mtl4a_files: Vec<String>,
}

/// Walk a bundle's `ShaderCache` directory without following symlinks and tally portable shader
/// files vs host-specific `*.mtl4a` archives.
pub fn scan_shader_cache(shader_cache_dir: &Path, rel_base: &Path) -> ShaderCacheScan {
    let mut scan = ShaderCacheScan::default();
    if !shader_cache_dir.is_dir() {
        return scan;
    }
    let mut stack = vec![shader_cache_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
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
                stack.push(p);
            } else if ft.is_file() {
                let ext = p.extension().and_then(|s| s.to_str());
                if ext.is_some_and(|e| e.eq_ignore_ascii_case("mtl4a")) {
                    scan.mtl4a_files.push(
                        p.strip_prefix(rel_base)
                            .unwrap_or(&p)
                            .display()
                            .to_string(),
                    );
                } else {
                    scan.portable_files += 1;
                    if ext.is_some_and(|e| e.eq_ignore_ascii_case("aw2s")) {
                        scan.aw2s_files += 1;
                    }
                }
            }
        }
    }
    scan
}

/// Locate an existing host `port-runner` binary (`$PORT_RUNNER`, newest built artifact in
/// `target/`, or sibling of `current_exe()`), building a debug `aarch64-apple-darwin` binary
/// via `cargo` if none is present.
pub fn find_or_build_host_port_runner(workspace_root: &Path) -> Result<PathBuf> {
    if let Some(p) = std::env::var_os("PORT_RUNNER").filter(|v| !v.is_empty()) {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Ok(pb);
        }
    }
    let mut candidates = vec![
        workspace_root.join("target/aarch64-apple-darwin/release/port-runner"),
        workspace_root.join("target/release/port-runner"),
        workspace_root.join("target/aarch64-apple-darwin/debug/port-runner"),
        workspace_root.join("target/debug/port-runner"),
    ];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("port-runner"));
        }
    }
    let mut existing: Vec<(PathBuf, std::time::SystemTime)> = candidates
        .into_iter()
        .filter(|p| p.is_file())
        .map(|p| {
            let mt = fs::metadata(&p)
                .and_then(|m| m.modified())
                .unwrap_or(std::time::UNIX_EPOCH);
            (p, mt)
        })
        .collect();
    if !existing.is_empty() {
        existing.sort_by_key(|(_, mt)| std::cmp::Reverse(*mt));
        return Ok(existing.remove(0).0);
    }

    let status = Command::new("cargo")
        .current_dir(workspace_root)
        .args([
            "build",
            "-p",
            "port-loader",
            "--bin",
            "port-runner",
            "--target",
            "aarch64-apple-darwin",
        ])
        .status()
        .context("building host port-runner via cargo")?;
    if !status.success() {
        bail!("failed to build port-runner (cargo exited with {status})");
    }
    for rel in [
        "target/aarch64-apple-darwin/debug/port-runner",
        "target/debug/port-runner",
    ] {
        let p = workspace_root.join(rel);
        if p.is_file() {
            return Ok(p);
        }
    }
    bail!("port-runner binary not found after cargo build")
}

/// Build a release `aarch64-apple-darwin` workspace binary (`port-runner`, `dxil-metald`, etc.),
/// falling back to any pre-existing release/debug artifact if the release build cannot run.
pub fn find_or_build_release_binary(workspace_root: &Path, bin_name: &str) -> Result<PathBuf> {
    let pkg = match bin_name {
        "dxil-metald" => "dxil-metal",
        "port-factory" => "port-factory",
        _ => "port-loader",
    };
    let status = Command::new("cargo")
        .current_dir(workspace_root)
        .args([
            "build",
            "--release",
            "--target",
            "aarch64-apple-darwin",
            "-p",
            pkg,
            "--bin",
            bin_name,
        ])
        .status();
    if status.is_ok_and(|s| s.success()) {
        for rel in [
            format!("target/aarch64-apple-darwin/release/{bin_name}"),
            format!("target/release/{bin_name}"),
        ] {
            let p = workspace_root.join(rel);
            if p.is_file() {
                return Ok(p);
            }
        }
    }
    for rel in [
        format!("target/aarch64-apple-darwin/release/{bin_name}"),
        format!("target/release/{bin_name}"),
        format!("target/aarch64-apple-darwin/debug/{bin_name}"),
        format!("target/debug/{bin_name}"),
    ] {
        let p = workspace_root.join(rel);
        if p.is_file() {
            return Ok(p);
        }
    }
    bail!("failed to build {bin_name}")
}

/// Best-effort ad-hoc `codesign --force --sign - <path>`.
pub fn codesign_ad_hoc(path: &Path) {
    let _ = Command::new("codesign")
        .args(["--force", "--sign", "-"])
        .arg(path)
        .status();
}

/// Sign `path` with the given `codesign` identity and optional entitlements plist.
pub fn codesign_with_identity(
    path: &Path,
    identity: &str,
    entitlements: Option<&Path>,
) -> Result<Output> {
    let mut cmd = Command::new("codesign");
    cmd.args(["--force", "--sign", identity]);
    if let Some(ent) = entitlements.filter(|p| p.is_file()) {
        cmd.arg("--entitlements").arg(ent);
    }
    cmd.arg(path);
    cmd.output()
        .with_context(|| format!("running codesign on {}", path.display()))
}

/// Verify a bundle's code signature via `codesign --verify --deep --strict <path>`.
pub fn codesign_verify_deep_strict(path: &Path) -> std::io::Result<Output> {
    Command::new("codesign")
        .args(["--verify", "--deep", "--strict"])
        .arg(path)
        .output()
}

/// Run `otool [-arch <arch>] -L <macho_path>` and return the linked library paths.
pub fn otool_linked_libs(macho_path: &Path, arch: Option<&str>) -> std::io::Result<Option<Vec<String>>> {
    let mut cmd = Command::new("otool");
    if let Some(a) = arch {
        cmd.args(["-arch", a]);
    }
    let out = cmd.arg("-L").arg(macho_path).output()?;
    if !out.status.success() {
        return Ok(None);
    }
    let mut deps = Vec::new();
    for raw in String::from_utf8_lossy(&out.stdout).lines() {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.ends_with(':') {
            continue;
        }
        let dep = trimmed
            .split(" (compatibility version")
            .next()
            .unwrap_or(trimmed)
            .trim();
        if !dep.is_empty() {
            deps.push(dep.to_string());
        }
    }
    Ok(Some(deps))
}

/// Run `otool [-arch <arch>] -D <macho_path>` and return the dylib's own install name(s).
pub fn otool_install_names(macho_path: &Path, arch: Option<&str>) -> BTreeSet<String> {
    let mut cmd = Command::new("otool");
    if let Some(a) = arch {
        cmd.args(["-arch", a]);
    }
    cmd.arg("-D")
        .arg(macho_path)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.ends_with(':'))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_plist_string_finds_key_value_and_handles_missing() {
        let xml = r#"
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.portfactory.demo</string>
    <key>CFBundleExecutable</key>
    <string> DemoApp </string>
</dict>"#;
        assert_eq!(
            extract_plist_string(xml, "CFBundleIdentifier").as_deref(),
            Some("com.portfactory.demo")
        );
        assert_eq!(
            extract_plist_string(xml, "CFBundleExecutable").as_deref(),
            Some("DemoApp")
        );
        assert_eq!(extract_plist_string(xml, "MissingKey"), None);
    }

    #[test]
    fn temp_dir_guard_cleans_up_on_drop() {
        let path = {
            let guard = TempDirGuard::create_unique("port-factory-common-test").unwrap();
            let p = guard.path().to_path_buf();
            assert!(p.is_dir());
            fs::write(p.join("probe.txt"), b"ok").unwrap();
            p
        };
        assert!(!path.exists());
    }

    #[cfg(unix)]
    #[test]
    fn copy_game_payload_filters_unsafe_symlinks_and_preserves_safe_relative_links() {
        use std::os::unix::fs::symlink;

        let root = TempDirGuard::create_unique("port-factory-copy-symlinks").unwrap();
        let outside_dir = root.path().join("outside");
        let game_dir = root.path().join("game_src");
        let dst_dir = root.path().join("game_dst");

        fs::create_dir_all(&outside_dir).unwrap();
        fs::create_dir_all(game_dir.join("data")).unwrap();
        fs::create_dir_all(game_dir.join("sub")).unwrap();
        fs::create_dir_all(game_dir.join("dir_a")).unwrap();
        fs::create_dir_all(game_dir.join("dir_b")).unwrap();
        fs::create_dir_all(game_dir.join(".DepotDownloader")).unwrap();

        let outside_secret = outside_dir.join("secret.txt");
        fs::write(&outside_secret, b"OUTSIDE_SECRET").unwrap();
        fs::write(game_dir.join("Game.exe"), b"MZ").unwrap();
        fs::write(game_dir.join("data/real.txt"), b"REAL_DATA").unwrap();
        fs::write(game_dir.join("sub/nested.txt"), b"NESTED").unwrap();
        fs::write(game_dir.join(".DepotDownloader/meta.bin"), b"META").unwrap();
        fs::write(game_dir.join(".DS_Store"), b"DS").unwrap();

        // 1. Escaping symlinks (relative escape, absolute escape, absolute-into-src, root-bounce).
        symlink("../outside/secret.txt", game_dir.join("escape_rel.txt")).unwrap();
        symlink(&outside_secret, game_dir.join("escape_abs.txt")).unwrap();
        symlink(&outside_dir, game_dir.join("escape_dir")).unwrap();
        symlink(
            fs::canonicalize(game_dir.join("data/real.txt")).unwrap(),
            game_dir.join("escape_abs_inside.txt"),
        )
        .unwrap();
        symlink(
            "../game_src/data/real.txt",
            game_dir.join("escape_bounce.txt"),
        )
        .unwrap();

        // 2. Dangling symlinks + links chained through skipped entries.
        symlink("missing_target.txt", game_dir.join("dangling.txt")).unwrap();
        symlink(
            "escape_abs_inside.txt",
            game_dir.join("chained_to_skipped.txt"),
        )
        .unwrap();
        symlink(
            ".DepotDownloader/meta.bin",
            game_dir.join("chained_to_depot.txt"),
        )
        .unwrap();

        // 3. Cycles (self dir loop, ancestor dir loop, mutual ELOOP links, mutual dir cycle).
        symlink(".", game_dir.join("self_dir_loop")).unwrap();
        symlink("..", game_dir.join("sub/ancestor_loop")).unwrap();
        symlink("eloop_b", game_dir.join("eloop_a")).unwrap();
        symlink("eloop_a", game_dir.join("eloop_b")).unwrap();
        symlink("../dir_b", game_dir.join("dir_a/to_b")).unwrap();
        symlink("../dir_a", game_dir.join("dir_b/to_a")).unwrap();

        // 4. Safe interior relative symlinks (file, parent-relative file, acyclic directory).
        symlink("data/real.txt", game_dir.join("safe_file_link.txt")).unwrap();
        symlink("../data/real.txt", game_dir.join("sub/safe_parent_file.txt")).unwrap();
        symlink("../data", game_dir.join("sub/safe_dir_link")).unwrap();

        // Seed a stale symlink at `dst_dir` to ensure `copy_game_payload` replaces it cleanly.
        symlink(&outside_dir, &dst_dir).unwrap();

        let report = copy_game_payload(&game_dir, &dst_dir, None).unwrap();

        // Regular files copied; metadata stripped.
        assert_eq!(fs::read(dst_dir.join("Game.exe")).unwrap(), b"MZ");
        assert_eq!(fs::read(dst_dir.join("data/real.txt")).unwrap(), b"REAL_DATA");
        assert_eq!(fs::read(dst_dir.join("sub/nested.txt")).unwrap(), b"NESTED");
        assert!(fs::symlink_metadata(dst_dir.join(".DepotDownloader")).is_err());
        assert!(fs::symlink_metadata(dst_dir.join(".DS_Store")).is_err());

        // All escaping, dangling, chained-to-skipped, and self/ancestor/ELOOP symlinks skipped.
        for skipped in [
            "escape_rel.txt",
            "escape_abs.txt",
            "escape_dir",
            "escape_abs_inside.txt",
            "escape_bounce.txt",
            "dangling.txt",
            "chained_to_skipped.txt",
            "chained_to_depot.txt",
            "self_dir_loop",
            "sub/ancestor_loop",
            "eloop_a",
            "eloop_b",
        ] {
            assert!(
                fs::symlink_metadata(dst_dir.join(skipped)).is_err(),
                "unsafe symlink {skipped} should have been skipped"
            );
        }

        // Mutual directory cycle must be broken (`dir_a/to_b` is processed first, so `dir_b/to_a` is skipped).
        assert!(
            fs::symlink_metadata(dst_dir.join("dir_b/to_a")).is_err(),
            "cycle-closing link dir_b/to_a must be skipped"
        );
        assert!(
            !dst_dir.join("dir_a/to_b/to_a").exists(),
            "mutual directory cycle must not remain traversable"
        );

        // Safe interior relative symlinks are preserved as relative symlinks resolving inside `dst_dir`.
        let dst_canon = fs::canonicalize(&dst_dir).unwrap();
        for (rel, expected_raw) in [
            ("safe_file_link.txt", "data/real.txt"),
            ("sub/safe_parent_file.txt", "../data/real.txt"),
            ("sub/safe_dir_link", "../data"),
        ] {
            let p = dst_dir.join(rel);
            let meta = fs::symlink_metadata(&p)
                .unwrap_or_else(|e| panic!("expected safe link {rel} to be preserved: {e}"));
            assert!(meta.file_type().is_symlink(), "{rel} should remain a symlink");
            assert_eq!(fs::read_link(&p).unwrap(), Path::new(expected_raw));
            let resolved = fs::canonicalize(&p).unwrap();
            assert!(
                resolved.starts_with(&dst_canon),
                "preserved symlink {rel} resolved outside dst: {}",
                resolved.display()
            );
        }
        assert_eq!(
            fs::read(dst_dir.join("safe_file_link.txt")).unwrap(),
            b"REAL_DATA"
        );
        assert_eq!(
            fs::read(dst_dir.join("sub/safe_parent_file.txt")).unwrap(),
            b"REAL_DATA"
        );
        assert_eq!(
            fs::read(dst_dir.join("sub/safe_dir_link/real.txt")).unwrap(),
            b"REAL_DATA"
        );

        assert_eq!(report.files_copied, 3);
        assert_eq!(report.symlinks_preserved, 4); // 3 safe links + acyclic `dir_a/to_b`
        assert_eq!(report.symlinks_skipped_escaping, 5);
        assert_eq!(report.symlinks_skipped_dangling, 3);
        assert_eq!(report.symlinks_skipped_cycle, 5);
    }
}
