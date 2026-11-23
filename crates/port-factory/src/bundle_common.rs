//! Shared macOS and iOS bundle staging, signing, shader-cache, Mach-O inspection,
//! and temporary-directory helpers used across `steam_library`, `ios_bundle`,
//! `launcher`, and the `port-factory` CLI.

use anyhow::{bail, Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
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

/// Recursively copy `src` into `dst`, skipping `.DepotDownloader`, `.DS_Store`, and an optional
/// canonicalized path (`skip_canon`, e.g. an output `.app` located inside the source tree).
pub fn copy_dir_recursive(src: &Path, dst: &Path, skip_canon: Option<&Path>) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".DepotDownloader" || name == ".DS_Store" {
            continue;
        }
        let sp = entry.path();
        if let Some(skip) = skip_canon {
            if fs::canonicalize(&sp).ok().as_deref() == Some(skip) {
                continue;
            }
        }
        let dp = dst.join(&name);
        let ft = entry.file_type()?;
        if ft.is_dir() {
            copy_dir_recursive(&sp, &dp, skip_canon)?;
        } else if ft.is_file() {
            fs::copy(&sp, &dp)?;
        }
    }
    Ok(())
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
}
