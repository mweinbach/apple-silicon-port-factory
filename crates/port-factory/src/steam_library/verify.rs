//! Portable macOS `.app` bundle verifier (`bundle-verify`).

use super::macos_bundle::{is_macho_file, parse_vtool_macos_versions, version_major};
use crate::bundle_common::{
    codesign_verify_deep_strict, extract_plist_string, otool_install_names, otool_linked_libs,
    scan_shader_cache,
};
use anyhow::Result;
use port_manifest::GameManifest;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Outcome of a single `.app` portability check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundleCheckStatus {
    Pass,
    Warn,
    Fail,
}

/// One diagnostic check from [`verify_app_bundle`].
#[derive(Debug, Clone)]
pub struct BundleCheck {
    pub name: String,
    pub status: BundleCheckStatus,
    pub detail: String,
}

/// Full portability and self-containment verification report for a macOS `.app` bundle.
#[derive(Debug, Clone)]
pub struct BundleVerifyReport {
    pub app_path: PathBuf,
    pub checks: Vec<BundleCheck>,
}

impl BundleVerifyReport {
    pub fn passed(&self) -> bool {
        self.checks
            .iter()
            .all(|c| c.status != BundleCheckStatus::Fail)
    }

    pub fn fail_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.status == BundleCheckStatus::Fail)
            .count()
    }

    pub fn warn_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.status == BundleCheckStatus::Warn)
            .count()
    }
}

#[cfg(target_os = "macos")]
fn is_portable_dylib_dep(dep: &str) -> bool {
    dep.starts_with("/System/Library/")
        || dep.starts_with("/usr/lib/")
        || dep.starts_with("@rpath/")
        || dep.starts_with("@loader_path/")
        || dep.starts_with("@executable_path/")
}

/// Verify that a macOS `.app` bundle is genuinely self-contained and ready to copy to an
/// external SSD and launch on another Apple Silicon Mac without developer environment setup.
pub fn verify_app_bundle(app_path: &Path) -> Result<BundleVerifyReport> {
    let mut checks = Vec::new();
    let push = |checks: &mut Vec<BundleCheck>,
                name: &str,
                status: BundleCheckStatus,
                detail: String| {
        checks.push(BundleCheck {
            name: name.to_string(),
            status,
            detail,
        });
    };

    if !app_path.is_dir() {
        push(
            &mut checks,
            "bundle_root",
            BundleCheckStatus::Fail,
            format!("not a directory: {}", app_path.display()),
        );
        return Ok(BundleVerifyReport {
            app_path: app_path.to_path_buf(),
            checks,
        });
    }

    let canon_app = fs::canonicalize(app_path).unwrap_or_else(|_| app_path.to_path_buf());
    let contents = app_path.join("Contents");
    let macos_dir = contents.join("MacOS");
    let frameworks_dir = contents.join("Frameworks");
    let resources_dir = contents.join("Resources");

    // 1. Info.plist + CFBundleExecutable
    let plist_path = contents.join("Info.plist");
    let mut bundle_executable: Option<String> = None;
    if !plist_path.is_file() {
        push(
            &mut checks,
            "info_plist",
            BundleCheckStatus::Fail,
            "missing Contents/Info.plist".to_string(),
        );
    } else {
        let plist_xml = fs::read_to_string(&plist_path).unwrap_or_default();
        bundle_executable = extract_plist_string(&plist_xml, "CFBundleExecutable");
        #[cfg(target_os = "macos")]
        {
            let lint = Command::new("/usr/bin/plutil")
                .args(["-lint", &plist_path.to_string_lossy()])
                .output();
            match lint {
                Ok(o) if o.status.success() => push(
                    &mut checks,
                    "info_plist",
                    BundleCheckStatus::Pass,
                    format!(
                        "valid XML (CFBundleExecutable={})",
                        bundle_executable.as_deref().unwrap_or("<missing>")
                    ),
                ),
                Ok(o) => push(
                    &mut checks,
                    "info_plist",
                    BundleCheckStatus::Fail,
                    format!(
                        "plutil -lint failed: {}",
                        String::from_utf8_lossy(&o.stderr).trim()
                    ),
                ),
                Err(e) => push(
                    &mut checks,
                    "info_plist",
                    BundleCheckStatus::Warn,
                    format!("could not invoke plutil: {e}"),
                ),
            }
        }
        #[cfg(not(target_os = "macos"))]
        push(
            &mut checks,
            "info_plist",
            BundleCheckStatus::Pass,
            format!(
                "present (CFBundleExecutable={})",
                bundle_executable.as_deref().unwrap_or("<missing>")
            ),
        );
    }

    // 2. Core executables
    let launcher_path = bundle_executable.as_ref().map(|e| macos_dir.join(e));
    match launcher_path {
        Some(ref p) if p.is_file() => push(
            &mut checks,
            "launcher_executable",
            BundleCheckStatus::Pass,
            format!("{}", p.file_name().unwrap_or_default().to_string_lossy()),
        ),
        _ => push(
            &mut checks,
            "launcher_executable",
            BundleCheckStatus::Fail,
            format!(
                "missing Contents/MacOS/{}",
                bundle_executable.as_deref().unwrap_or("<unset>")
            ),
        ),
    }

    let runner_path = macos_dir.join("port-runner");
    let aw2_runner_path = macos_dir.join("aw2-loader");
    let has_port_runner = runner_path.is_file();
    if has_port_runner || aw2_runner_path.is_file() {
        let which = if has_port_runner {
            "port-runner"
        } else {
            "aw2-loader"
        };
        let sz_mb = fs::metadata(if has_port_runner {
            &runner_path
        } else {
            &aw2_runner_path
        })
        .map(|m| m.len() as f64 / (1024.0 * 1024.0))
        .unwrap_or(0.0);
        push(
            &mut checks,
            "runtime_binary",
            BundleCheckStatus::Pass,
            format!("{which} ({sz_mb:.1} MiB)"),
        );
    } else {
        push(
            &mut checks,
            "runtime_binary",
            BundleCheckStatus::Fail,
            "missing Contents/MacOS/port-runner (or aw2-loader)".to_string(),
        );
    }

    let shaderd_path = macos_dir.join("dxil-metald");
    if shaderd_path.is_file() {
        push(
            &mut checks,
            "shader_helper",
            BundleCheckStatus::Pass,
            "Contents/MacOS/dxil-metald present".to_string(),
        );
    } else {
        push(
            &mut checks,
            "shader_helper",
            BundleCheckStatus::Fail,
            "missing Contents/MacOS/dxil-metald".to_string(),
        );
    }

    // 3. Manifest & Game payload + symlink audit across Contents/
    let manifest_path = resources_dir.join("game.toml");
    if manifest_path.is_file() {
        let content = fs::read_to_string(&manifest_path).unwrap_or_default();
        let parsed = GameManifest::parse(&content);
        let exe_field = parsed.exe.unwrap_or_default();
        let overrides = parsed
            .builtin_overrides
            .unwrap_or_else(|| "(none)".to_string());
        push(
            &mut checks,
            "game_manifest",
            BundleCheckStatus::Pass,
            format!(
                "Contents/Resources/game.toml (exe={exe_field:?}, builtin_overrides={overrides:?})"
            ),
        );
    } else {
        push(
            &mut checks,
            "game_manifest",
            BundleCheckStatus::Fail,
            "missing Contents/Resources/game.toml".to_string(),
        );
    }

    let game_dir = resources_dir.join("Game");
    match fs::symlink_metadata(&game_dir) {
        Ok(m) if m.file_type().is_symlink() => push(
            &mut checks,
            "game_payload",
            BundleCheckStatus::Fail,
            "Contents/Resources/Game is a symlink (all game EXEs, DLLs, and assets must be bundled inside the .app)"
                .to_string(),
        ),
        Ok(m) if m.is_dir() => push(
            &mut checks,
            "game_payload",
            BundleCheckStatus::Pass,
            "Contents/Resources/Game is a self-contained directory".to_string(),
        ),
        _ => push(
            &mut checks,
            "game_payload",
            BundleCheckStatus::Fail,
            "missing Contents/Resources/Game directory".to_string(),
        ),
    }

    // Walk Contents/ without following symlinks and fail on any symlink escaping the bundle.
    let mut escaping_symlinks = Vec::new();
    let mut stack = vec![contents.clone()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in rd.flatten() {
            let Ok(ft) = entry.file_type() else {
                continue;
            };
            let p = entry.path();
            if ft.is_symlink() {
                let target_ok = fs::canonicalize(&p)
                    .map(|resolved| resolved.starts_with(&canon_app))
                    .unwrap_or(false);
                if !target_ok {
                    let rel = p.strip_prefix(app_path).unwrap_or(&p);
                    let raw_target = fs::read_link(&p)
                        .map(|t| t.display().to_string())
                        .unwrap_or_else(|_| "<unreadable>".to_string());
                    escaping_symlinks.push(format!("{} -> {raw_target}", rel.display()));
                }
            } else if ft.is_dir() {
                stack.push(p);
            }
        }
    }
    if escaping_symlinks.is_empty() {
        push(
            &mut checks,
            "external_symlinks",
            BundleCheckStatus::Pass,
            "zero symlinks escaping the .app bundle".to_string(),
        );
    } else {
        push(
            &mut checks,
            "external_symlinks",
            BundleCheckStatus::Fail,
            format!(
                "{} symlink(s) escape bundle or dangle: {}",
                escaping_symlinks.len(),
                escaping_symlinks
                    .iter()
                    .take(5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        );
    }

    // 4. Frameworks
    let msc = frameworks_dir.join("libmetalirconverter.dylib");
    let dxilconv = frameworks_dir.join("libdxilconv.dylib");
    let moltenvk = frameworks_dir.join("libMoltenVK.dylib");
    if msc.is_file() {
        let mut extras = Vec::new();
        if dxilconv.is_file() {
            extras.push("libdxilconv.dylib");
        }
        if moltenvk.is_file() {
            extras.push("libMoltenVK.dylib");
        }
        push(
            &mut checks,
            "frameworks",
            BundleCheckStatus::Pass,
            format!(
                "libmetalirconverter.dylib present{}",
                if extras.is_empty() {
                    String::new()
                } else {
                    format!(" (+ {})", extras.join(", "))
                }
            ),
        );
    } else {
        push(
            &mut checks,
            "frameworks",
            BundleCheckStatus::Fail,
            "missing Contents/Frameworks/libmetalirconverter.dylib".to_string(),
        );
    }

    // 5. Mach-O dylib link & minos/arch portability audit (`otool -L`, `vtool`, `lipo`)
    #[cfg(target_os = "macos")]
    {
        let mut macho_files = Vec::new();
        for dir in [&macos_dir, &frameworks_dir] {
            if let Ok(rd) = fs::read_dir(dir) {
                for entry in rd.flatten() {
                    let p = entry.path();
                    if p.is_file() && is_macho_file(&p) {
                        macho_files.push(p);
                    }
                }
            }
        }
        macho_files.sort();

        let mut bad_links = Vec::new();
        let mut bad_arch_or_minos = Vec::new();

        for macho in &macho_files {
            let rel = macho
                .strip_prefix(app_path)
                .unwrap_or(macho)
                .display()
                .to_string();
            let macho_s = macho.to_string_lossy().into_owned();

            // Architecture check (`arm64` required for Apple Silicon).
            if let Ok(out) = Command::new("lipo").args(["-archs", &macho_s]).output() {
                if out.status.success() {
                    let archs = String::from_utf8_lossy(&out.stdout);
                    if !archs.split_whitespace().any(|a| a == "arm64") {
                        bad_arch_or_minos.push(format!(
                            "{rel}: missing arm64 (archs={})",
                            archs.trim()
                        ));
                    }
                }
            }

            // `minos` check (`<= 15.x` so newer-beta-built binaries don't fail on macOS 15/26 Macs).
            let vtool_out = Command::new("vtool")
                .args(["-arch", "arm64", "-show-build", &macho_s])
                .output()
                .or_else(|_| Command::new("vtool").args(["-show-build", &macho_s]).output());
            if let Ok(out) = vtool_out {
                if out.status.success() {
                    for (minos, _sdk) in
                        parse_vtool_macos_versions(&String::from_utf8_lossy(&out.stdout))
                    {
                        if version_major(&minos).is_some_and(|m| m > 15) {
                            bad_arch_or_minos.push(format!("{rel}: minos={minos} (> 15.x)"));
                        }
                    }
                }
            }

            // Own install name (`otool -D`), so we ignore a dylib's self-ID line in `otool -L`.
            let own_ids = otool_install_names(macho, Some("arm64"));
            if let Ok(Some(deps)) = otool_linked_libs(macho, Some("arm64")) {
                for dep in deps {
                    if own_ids.contains(&dep) {
                        continue;
                    }
                    if !is_portable_dylib_dep(&dep) {
                        bad_links.push(format!("{rel} -> {dep}"));
                    }
                }
            }
        }

        if bad_links.is_empty() {
            push(
                &mut checks,
                "macho_dylib_links",
                BundleCheckStatus::Pass,
                format!(
                    "{} Mach-O file(s) link only macOS system or bundle-relative libraries",
                    macho_files.len()
                ),
            );
        } else {
            push(
                &mut checks,
                "macho_dylib_links",
                BundleCheckStatus::Fail,
                format!("non-portable dylib references: {}", bad_links.join("; ")),
            );
        }

        if bad_arch_or_minos.is_empty() {
            push(
                &mut checks,
                "macho_arch_minos",
                BundleCheckStatus::Pass,
                format!(
                    "all {} Mach-O file(s) include arm64 with portable minos (<= 15.x)",
                    macho_files.len()
                ),
            );
        } else {
            push(
                &mut checks,
                "macho_arch_minos",
                BundleCheckStatus::Fail,
                bad_arch_or_minos.join("; "),
            );
        }
    }

    // 6. Prewarmed AotCache & portable ShaderCache (zero host-specific `*.mtl4a` bloat)
    let aot_cache_dir = resources_dir.join("AotCache");
    let mut aot_count = 0usize;
    let mut aot_bytes = 0u64;
    if let Ok(rd) = fs::read_dir(&aot_cache_dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("aot") {
                aot_count += 1;
                aot_bytes += fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
            }
        }
    }
    if aot_count > 0 {
        push(
            &mut checks,
            "aot_cache",
            BundleCheckStatus::Pass,
            format!(
                "{aot_count} prewarmed .aot module(s) ({:.1} MiB)",
                aot_bytes as f64 / (1024.0 * 1024.0)
            ),
        );
    } else if has_port_runner {
        push(
            &mut checks,
            "aot_cache",
            BundleCheckStatus::Warn,
            "no prewarmed .aot files in Contents/Resources/AotCache (first launch will JIT on demand)"
                .to_string(),
        );
    }

    let shader_scan = scan_shader_cache(&resources_dir.join("ShaderCache"), app_path);
    if shader_scan.mtl4a_files.is_empty() {
        push(
            &mut checks,
            "shader_cache_portability",
            BundleCheckStatus::Pass,
            format!(
                "zero device-specific *.mtl4a archives ({} portable shader/manifest file(s))",
                shader_scan.portable_files
            ),
        );
    } else {
        push(
            &mut checks,
            "shader_cache_portability",
            BundleCheckStatus::Fail,
            format!(
                "found {} host-specific *.mtl4a archive(s) inside bundle (remove; cold Macs rebuild from pso_manifest): {}",
                shader_scan.mtl4a_files.len(),
                shader_scan
                    .mtl4a_files
                    .iter()
                    .take(3)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
    }

    // 7. Code signature verification
    #[cfg(target_os = "macos")]
    {
        match codesign_verify_deep_strict(app_path) {
            Ok(out) if out.status.success() => push(
                &mut checks,
                "codesign",
                BundleCheckStatus::Pass,
                "codesign --verify --deep --strict OK".to_string(),
            ),
            Ok(out) => push(
                &mut checks,
                "codesign",
                BundleCheckStatus::Fail,
                format!(
                    "codesign verify failed: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                ),
            ),
            Err(e) => push(
                &mut checks,
                "codesign",
                BundleCheckStatus::Warn,
                format!("could not invoke codesign: {e}"),
            ),
        }
    }

    // 8. Live `env_clear()` cold-launch checks
    if shaderd_path.is_file() && is_macho_file(&shaderd_path) {
        match Command::new(&shaderd_path)
            .arg("--version")
            .env_clear()
            .output()
        {
            Ok(out) if out.status.success() => {
                let ver = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if ver.starts_with("dxil-metald protocol v") {
                    push(
                        &mut checks,
                        "cold_dxil_metald",
                        BundleCheckStatus::Pass,
                        format!("env_clear() OK ({ver})"),
                    );
                } else {
                    push(
                        &mut checks,
                        "cold_dxil_metald",
                        BundleCheckStatus::Fail,
                        format!("unexpected dxil-metald --version output: {ver:?}"),
                    );
                }
            }
            Ok(out) => push(
                &mut checks,
                "cold_dxil_metald",
                BundleCheckStatus::Fail,
                format!(
                    "env_clear() failed (status {}): {}",
                    out.status,
                    String::from_utf8_lossy(&out.stderr).trim()
                ),
            ),
            Err(e) => push(
                &mut checks,
                "cold_dxil_metald",
                BundleCheckStatus::Fail,
                format!("failed to exec dxil-metald: {e}"),
            ),
        }
    }

    if let Some(ref launcher) = launcher_path {
        if launcher.is_file() && is_macho_file(launcher) {
            match Command::new(launcher)
                .arg("--aot-info")
                .env_clear()
                .output()
            {
                Ok(out) if out.status.success() => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let msc_ok = stdout.contains("Bundled Metal IR Conv  : true");
                    let helper_ok = stdout.contains("Bundled Shader Helper  : true");
                    if msc_ok && helper_ok {
                        push(
                            &mut checks,
                            "cold_launcher_info",
                            BundleCheckStatus::Pass,
                            "env_clear() --aot-info verified bundled helper + Metal IR converter"
                                .to_string(),
                        );
                    } else {
                        push(
                            &mut checks,
                            "cold_launcher_info",
                            BundleCheckStatus::Fail,
                            format!(
                                "--aot-info reported missing runtime components:\n{}",
                                stdout.trim()
                            ),
                        );
                    }
                }
                Ok(out) => push(
                    &mut checks,
                    "cold_launcher_info",
                    BundleCheckStatus::Fail,
                    format!(
                        "env_clear() --aot-info failed (status {}): {}",
                        out.status,
                        String::from_utf8_lossy(&out.stderr).trim()
                    ),
                ),
                Err(e) => push(
                    &mut checks,
                    "cold_launcher_info",
                    BundleCheckStatus::Fail,
                    format!("failed to exec launcher: {e}"),
                ),
            }
        }
    }

    Ok(BundleVerifyReport {
        app_path: app_path.to_path_buf(),
        checks,
    })
}

/// Format a [`BundleVerifyReport`] for CLI display.
pub fn format_bundle_verify_report(report: &BundleVerifyReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "=== Portable .app Bundle Verification: {} ===\n",
        report.app_path.display()
    ));
    for c in &report.checks {
        let tag = match c.status {
            BundleCheckStatus::Pass => "PASS",
            BundleCheckStatus::Warn => "WARN",
            BundleCheckStatus::Fail => "FAIL",
        };
        out.push_str(&format!("  [{tag}] {:26} {}\n", c.name, c.detail));
    }
    let verdict = if report.passed() {
        format!(
            "READY FOR SSD / ANOTHER APPLE SILICON MAC ({} checks, {} warning(s))",
            report.checks.len(),
            report.warn_count()
        )
    } else {
        format!(
            "NOT PORTABLE ({} failure(s), {} warning(s))",
            report.fail_count(),
            report.warn_count()
        )
    };
    out.push_str(&format!("Result: {verdict}\n"));
    out
}
