//! One-command iOS / iPadOS `.app` and `.ipa` packager & verifier (Milestone #3).
//!
//! Assembles any Windows x86-64 game or executable into a self-contained Apple Silicon
//! iOS / iOS Simulator app bundle (`<App>.app`) or sideloadable/TestFlight archive
//! (`<App>.ipa`) with zero `MAP_JIT` dependencies:
//!
//! 1. Builds (or reuses) `libport_loader.a` for `aarch64-apple-ios` or `aarch64-apple-ios-sim`.
//! 2. Runs `port-runner --emit-static-o` to offline-compile the target PE and its mapped
//!    non-builtin DLLs into Apple-signed ARM64 Mach-O `.aot.o` objects (Milestone #2).
//! 3. Compiles `ios/PortHost/{PortHostApp,MetalSurfaceView}.swift` and links all `.aot.o`
//!    objects + `libport_loader.a` with `-Xlinker -dead_strip` into the flat iOS executable.
//! 4. Stages the flat iOS bundle layout (`Info.plist`, `PkgInfo`, `game.toml`, `Game/`,
//!    portable Tier-1 `ShaderCache/`, optional `embedded.mobileprovision`), code-signs the
//!    bundle, optionally zips `Payload/<App>.app` into `.ipa`, and optionally emits an
//!    XcodeGen `project.yml` spec for GUI signing/deployment in Xcode.
//! 5. Verifies any `.app` or `.ipa` bundle (`verify_ios_bundle`) for ARM64 iOS Mach-O
//!    platform tags, embedded static AOT modules, zero macOS framework leaks, zero host-specific
//!    `*.mtl4a` shader cache bloat, valid `game.toml`, and valid code signature.

use crate::gpu_shaders::precompile_shaders_for_paths;
use crate::steam_library::{copy_ios_shader_cache, sanitize_game_title};
use crate::storage_opt::{
    optimize_game_assets, parse_manifest_storage_config, StorageOptOptions, StorageOptReport,
};
use anyhow::{bail, Context, Result};
use port_manifest::GameManifest;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Target iOS Mach-O platform for bundle compilation and linking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IosTarget {
    /// Physical iPhone / iPad (`aarch64-apple-ios`, `iphoneos` SDK, Mach-O `PLATFORM_IOS = 2`).
    Device,
    /// Apple Silicon iOS Simulator (`aarch64-apple-ios-sim`, `iphonesimulator` SDK, Mach-O `PLATFORM_IOSSIMULATOR = 7`).
    Simulator,
}

impl IosTarget {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "device" | "ios" | "iphoneos" | "ipad" | "ipados" => Some(Self::Device),
            "sim" | "simulator" | "ios-sim" | "iossim" | "iphonesimulator" => Some(Self::Simulator),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Device => "device",
            Self::Simulator => "sim",
        }
    }

    pub fn rust_target(self) -> &'static str {
        match self {
            Self::Device => "aarch64-apple-ios",
            Self::Simulator => "aarch64-apple-ios-sim",
        }
    }

    pub fn sdk_name(self) -> &'static str {
        match self {
            Self::Device => "iphoneos",
            Self::Simulator => "iphonesimulator",
        }
    }

    pub fn static_macho_platform(self) -> &'static str {
        match self {
            Self::Device => "ios",
            Self::Simulator => "ios-sim",
        }
    }

    pub fn swift_triple(self, deployment_target: &str) -> String {
        match self {
            Self::Device => format!("arm64-apple-ios{deployment_target}"),
            Self::Simulator => format!("arm64-apple-ios{deployment_target}-simulator"),
        }
    }

    pub fn expected_macho_platform_id(self) -> u32 {
        match self {
            Self::Device => 2,    // PLATFORM_IOS
            Self::Simulator => 7, // PLATFORM_IOSSIMULATOR
        }
    }

    pub fn dt_platform_name(self) -> &'static str {
        match self {
            Self::Device => "iphoneos",
            Self::Simulator => "iphonesimulator",
        }
    }
}

/// Options for packaging a Windows x86-64 game/PE into an iOS `.app` or `.ipa`.
#[derive(Debug, Clone)]
pub struct IosBundleOptions {
    pub workspace_root: PathBuf,
    /// Input Windows `.exe`, game directory, or `game.toml` manifest.
    pub source: PathBuf,
    /// Optional executable filename inside the game directory (required only when `source` is
    /// a directory without a `game.toml` declaring `exe`).
    pub exe_override: Option<String>,
    /// Optional declarative `game.toml` manifest to bundle and use for overrides.
    pub manifest_override: Option<PathBuf>,
    /// Display title override (default: derived from `game.toml` or `.exe` stem).
    pub title_override: Option<String>,
    /// Bundle identifier override (default: `com.portfactory.<sanitized_id>`).
    pub bundle_id: Option<String>,
    pub target: IosTarget,
    /// Output `.app` directory or `.ipa` archive path.
    pub out_path: PathBuf,
    /// Use `--release` when building `libport_loader.a` (default: `false` for fast iteration).
    pub release: bool,
    /// Minimum iOS deployment target (default: `"17.0"`).
    pub deployment_target: String,
    /// Offline-compile mapped PEs into signed static ARM64 `.aot.o` objects and link them (default: `true`).
    pub emit_static_aot: bool,
    /// Cap discovered PE function roots per module during static AOT compilation (0 = all).
    pub aot_max_funcs: usize,
    /// Optional additional `.aot.o` object files to link directly into the iOS executable.
    pub extra_aot_objs: Vec<PathBuf>,
    /// Precompile embedded PE shaders into `<App>.app/ShaderCache` (default: `true`).
    pub precompile_shaders: bool,
    /// Optional existing host shader cache root (e.g. `~/Translator/ShaderCache`) whose portable
    /// Tier-1 `.aw2s` shaders & manifests are copied into `<App>.app/ShaderCache` (no `*.mtl4a`).
    pub bundle_shader_cache_from: Option<PathBuf>,
    /// Optional `codesign` identity (default: `"-"` ad-hoc; pass an Apple Development /
    /// Distribution identity for real device sideloading/TestFlight).
    pub codesign_identity: Option<String>,
    /// Optional `.mobileprovision` file copied to `<App>.app/embedded.mobileprovision`.
    pub provisioning_profile: Option<PathBuf>,
    /// When `true`, also emit a ready-to-open XcodeGen `project.yml` next to `out_path`.
    pub emit_xcodegen_spec: bool,
    /// Optional generalized storage optimization pass applied while staging game assets
    /// directly inside `<App>.app/Game/` (`--optimize-storage lossless|high|compact`,
    /// `--hevc-videos`, `--max-tex-dim`, etc.).
    pub storage_opt: Option<StorageOptOptions>,
}

impl IosBundleOptions {
    pub fn new(workspace_root: PathBuf, source: PathBuf, out_path: PathBuf) -> Self {
        Self {
            workspace_root,
            source,
            exe_override: None,
            manifest_override: None,
            title_override: None,
            bundle_id: None,
            target: IosTarget::Device,
            out_path,
            release: false,
            deployment_target: "17.0".to_string(),
            emit_static_aot: true,
            aot_max_funcs: 0,
            extra_aot_objs: Vec::new(),
            precompile_shaders: true,
            bundle_shader_cache_from: None,
            codesign_identity: None,
            provisioning_profile: None,
            emit_xcodegen_spec: false,
            storage_opt: None,
        }
    }
}

/// Summary returned by [`build_ios_bundle`].
#[derive(Debug, Clone)]
pub struct IosBundleReport {
    pub title: String,
    pub bundle_id: String,
    pub executable_name: String,
    pub target: IosTarget,
    pub output_path: PathBuf,
    pub is_ipa: bool,
    pub primary_exe_rel: String,
    pub static_aot_objects: Vec<PathBuf>,
    pub shaders_bundled: usize,
    pub signed: bool,
    pub xcodegen_spec_path: Option<PathBuf>,
    pub storage_opt_report: Option<StorageOptReport>,
    pub verify: IosBundleVerifyReport,
}

/// Verification report returned by [`verify_ios_bundle`].
#[derive(Debug, Clone)]
pub struct IosBundleVerifyReport {
    pub bundle_path: PathBuf,
    pub is_ipa: bool,
    pub bundle_id: String,
    pub executable_name: String,
    pub macho_arch: String,
    pub macho_platform_id: u32,
    pub macho_platform_name: String,
    pub static_aot_modules: usize,
    pub has_game_toml: bool,
    pub has_game_dir: bool,
    pub bundled_exe_present: bool,
    pub portable_shader_files: usize,
    pub host_mtl4a_files: usize,
    pub macos_framework_leaks: Vec<String>,
    pub codesign_ok: bool,
    pub issues: Vec<String>,
}

impl IosBundleVerifyReport {
    pub fn ok(&self) -> bool {
        self.issues.is_empty()
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn sanitize_identifier(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if (ch == '-' || ch == '_' || ch == ' ') && !out.ends_with('-') {
            out.push('-');
        }
    }
    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() {
        "app".to_string()
    } else {
        trimmed.to_string()
    }
}

fn sanitize_exec_name(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        }
    }
    if out.is_empty() {
        "PortHost".to_string()
    } else {
        out
    }
}

/// Render a complete, fullscreen-ready iOS/iPadOS `Info.plist`.
pub fn render_ios_info_plist(
    display_name: &str,
    exec_name: &str,
    bundle_id: &str,
    deployment_target: &str,
    target: IosTarget,
) -> String {
    let disp = xml_escape(display_name);
    let exec = xml_escape(exec_name);
    let bid = xml_escape(bundle_id);
    let min_os = xml_escape(deployment_target);
    let dt_plat = target.dt_platform_name();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>{disp}</string>
    <key>CFBundleExecutable</key>
    <string>{exec}</string>
    <key>CFBundleIdentifier</key>
    <string>{bid}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{disp}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>DTPlatformName</key>
    <string>{dt_plat}</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
    <key>MinimumOSVersion</key>
    <string>{min_os}</string>
    <key>UIDeviceFamily</key>
    <array>
        <integer>1</integer>
        <integer>2</integer>
    </array>
    <key>UIRequiredDeviceCapabilities</key>
    <array>
        <string>arm64</string>
        <string>metal</string>
    </array>
    <key>UILaunchScreen</key>
    <dict/>
    <key>UIApplicationSceneManifest</key>
    <dict>
        <key>UIApplicationSupportsMultipleScenes</key>
        <false/>
        <key>UISceneConfigurations</key>
        <dict>
            <key>UIWindowSceneSessionRoleApplication</key>
            <array>
                <dict>
                    <key>UISceneConfigurationName</key>
                    <string>Default Configuration</string>
                </dict>
            </array>
        </dict>
    </dict>
    <key>UIRequiresFullScreen</key>
    <true/>
    <key>UIStatusBarHidden</key>
    <true/>
    <key>UIViewControllerBasedStatusBarAppearance</key>
    <true/>
    <key>UIApplicationSupportsIndirectInputEvents</key>
    <true/>
    <key>GCSupportsControllerUserInteraction</key>
    <true/>
    <key>LSSupportsGameMode</key>
    <true/>
    <key>GCSupportsGameMode</key>
    <true/>
    <key>CADisableMinimumFrameDurationOnPhone</key>
    <true/>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UISupportedInterfaceOrientations~ipad</key>
    <array>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
</dict>
</plist>
"#
    )
}

/// Render a clean XcodeGen `project.yml` spec so developers can open, sign, and deploy the
/// packaged iOS target directly in Xcode if desired.
pub fn render_xcodegen_spec(
    target_name: &str,
    bundle_id: &str,
    deployment_target: &str,
    workspace_root: &Path,
    aot_objects: &[PathBuf],
    game_toml: &Path,
    game_dir: Option<&Path>,
    shader_cache_dir: Option<&Path>,
    release: bool,
) -> String {
    let profile = if release { "release" } else { "debug" };
    let port_host_dir = workspace_root.join("ios/PortHost");
    let lib_dir_device = workspace_root.join(format!("target/aarch64-apple-ios/{profile}"));
    let lib_dir_sim = workspace_root.join(format!("target/aarch64-apple-ios-sim/{profile}"));
    let mut ld_flags = vec!["\"-lport_loader\"".to_string(), "\"-lz\"".to_string()];
    for obj in aot_objects {
        ld_flags.push(format!("\"{}\"", obj.display()));
    }
    let mut resources = vec![format!(
        "      - path: \"{}\"\n        buildPhase: resources",
        game_toml.display()
    )];
    if let (Some(dir), Ok(text)) = (game_toml.parent(), fs::read_to_string(game_toml)) {
        let manifest = GameManifest::parse(&text);
        for (src_rel, _) in &manifest.ios_seed_files {
            let seed = dir.join(src_rel);
            if seed.is_file() {
                resources.push(format!(
                    "      - path: \"{}\"\n        buildPhase: resources",
                    seed.display()
                ));
            }
        }
    }
    if let Some(gd) = game_dir {
        resources.push(format!(
            "      - path: \"{}\"\n        type: folder\n        buildPhase: resources",
            gd.display()
        ));
    }
    if let Some(sc) = shader_cache_dir {
        resources.push(format!(
            "      - path: \"{}\"\n        type: folder\n        buildPhase: resources",
            sc.display()
        ));
    }
    format!(
        r#"name: {target_name}
options:
  bundleIdPrefix: com.portfactory
  deploymentTarget:
    iOS: "{deployment_target}"
targets:
  {target_name}:
    type: application
    platform: iOS
    deploymentTarget: "{deployment_target}"
    sources:
      - path: "{host_dir}/PortHostApp.swift"
      - path: "{host_dir}/MetalSurfaceView.swift"
      - path: "{host_dir}/KeyboardBridge.swift"
      - path: "{host_dir}/PhoneHaptics.swift"
      - path: "{host_dir}/TouchOverlayView.swift"
{resources}
    settings:
      base:
        PRODUCT_BUNDLE_IDENTIFIER: "{bundle_id}"
        SWIFT_OBJC_BRIDGING_HEADER: "{host_dir}/PortHost-Bridging-Header.h"
        CODE_SIGN_ENTITLEMENTS: "{host_dir}/PortHost.entitlements"
        INFOPLIST_FILE: "{host_dir}/Info.plist"
        DEAD_CODE_STRIPPING: YES
        "LIBRARY_SEARCH_PATHS[sdk=iphoneos*]":
          - "{lib_dev}"
        "LIBRARY_SEARCH_PATHS[sdk=iphonesimulator*]":
          - "{lib_sim}"
        OTHER_LDFLAGS:
          - {ld_flags}
    dependencies:
      - sdk: Metal.framework
      - sdk: QuartzCore.framework
      - sdk: UIKit.framework
      - sdk: Foundation.framework
      - sdk: CoreFoundation.framework
      - sdk: AudioToolbox.framework
      - sdk: AVFAudio.framework
      - sdk: GameController.framework
      - sdk: CoreHaptics.framework
      - sdk: VideoToolbox.framework
      - sdk: CoreMedia.framework
      - sdk: CoreVideo.framework
      - sdk: Security.framework
"#,
        host_dir = port_host_dir.display(),
        resources = resources.join("\n"),
        lib_dev = lib_dir_device.display(),
        lib_sim = lib_dir_sim.display(),
        ld_flags = ld_flags.join("\n          - "),
    )
}

fn resolve_input(opts: &IosBundleOptions) -> Result<(PathBuf, PathBuf, String, Option<PathBuf>, GameManifest)> {
    let src = fs::canonicalize(&opts.source)
        .with_context(|| format!("resolving source {}", opts.source.display()))?;
    let mut manifest_path = opts.manifest_override.clone();
    let mut manifest = GameManifest::default();
    let mut loaded_manifest = false;

    let (game_dir, primary_exe) = if src.is_file() {
        let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
        if ext == "toml" {
            let text = fs::read_to_string(&src)?;
            manifest = GameManifest::parse(&text);
            loaded_manifest = true;
            manifest_path = Some(src.clone());
            let dir = src.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
            let exe_name = opts
                .exe_override
                .clone()
                .or_else(|| manifest.exe.clone())
                .ok_or_else(|| anyhow::anyhow!("{} does not specify `exe` (pass --exe NAME)", src.display()))?;
            let pe = dir.join(&exe_name);
            if !pe.is_file() {
                bail!("executable {} referenced by {} not found", pe.display(), src.display());
            }
            (dir, pe)
        } else {
            let dir = src.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
            if manifest_path.is_none() {
                let sibling = dir.join("game.toml");
                if sibling.is_file() {
                    manifest_path = Some(sibling);
                }
            }
            (dir, src)
        }
    } else if src.is_dir() {
        if manifest_path.is_none() {
            let candidate = src.join("game.toml");
            if candidate.is_file() {
                manifest_path = Some(candidate);
            }
        }
        if let Some(ref mp) = manifest_path {
            if let Ok(text) = fs::read_to_string(mp) {
                manifest = GameManifest::parse(&text);
                loaded_manifest = true;
            }
        }
        let exe_name = opts.exe_override.clone().or_else(|| manifest.exe.clone());
        let pe = if let Some(ref rel) = exe_name {
            let p = src.join(rel);
            if !p.is_file() {
                bail!("specified executable {} not found inside {}", rel, src.display());
            }
            p
        } else {
            // Find the largest top-level `.exe` inside the directory.
            let mut exes: Vec<(PathBuf, u64)> = fs::read_dir(&src)?
                .flatten()
                .filter_map(|e| {
                    let p = e.path();
                    let is_exe = p
                        .extension()
                        .and_then(|x| x.to_str())
                        .is_some_and(|x| x.eq_ignore_ascii_case("exe"));
                    if is_exe && p.is_file() {
                        let len = e.metadata().map(|m| m.len()).unwrap_or(0);
                        Some((p, len))
                    } else {
                        None
                    }
                })
                .collect();
            exes.sort_by_key(|(_, len)| std::cmp::Reverse(*len));
            exes.into_iter()
                .next()
                .map(|(p, _)| p)
                .ok_or_else(|| anyhow::anyhow!("no .exe found in {} (pass --exe NAME)", src.display()))?
        };
        (src, pe)
    } else {
        bail!("source path does not exist: {}", src.display());
    };

    if !loaded_manifest {
        if let Some(ref mp) = manifest_path {
            if let Ok(text) = fs::read_to_string(mp) {
                manifest = GameManifest::parse(&text);
            }
        }
    }

    let exe_rel = primary_exe
        .strip_prefix(&game_dir)
        .unwrap_or(&primary_exe)
        .to_string_lossy()
        .replace('\\', "/");

    Ok((game_dir, primary_exe, exe_rel, manifest_path, manifest))
}

fn ensure_ios_staticlib(workspace_root: &Path, target: IosTarget, release: bool, deployment_target: &str) -> Result<PathBuf> {
    let profile = if release { "release" } else { "debug" };
    let lib_path = workspace_root
        .join("target")
        .join(target.rust_target())
        .join(profile)
        .join("libport_loader.a");

    let mut cmd = Command::new("cargo");
    cmd.current_dir(workspace_root)
        .env("IPHONEOS_DEPLOYMENT_TARGET", deployment_target)
        .args([
            "build",
            "-p",
            "port-loader",
            "--lib",
            "--no-default-features",
            "--target",
            target.rust_target(),
        ]);
    if release {
        cmd.arg("--release");
    }
    let status = cmd
        .status()
        .with_context(|| format!("running cargo build for {}", target.rust_target()))?;
    if !status.success() {
        bail!("cargo build for {} failed with status {status}", target.rust_target());
    }
    if !lib_path.is_file() {
        bail!("expected static library not found at {}", lib_path.display());
    }
    Ok(lib_path)
}

fn ensure_host_port_runner(workspace_root: &Path) -> Result<PathBuf> {
    if let Some(p) = std::env::var_os("PORT_RUNNER").filter(|v| !v.is_empty()) {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Ok(pb);
        }
    }
    let mut existing: Vec<(PathBuf, std::time::SystemTime)> = [
        "target/aarch64-apple-darwin/release/port-runner",
        "target/aarch64-apple-darwin/debug/port-runner",
        "target/release/port-runner",
        "target/debug/port-runner",
    ]
    .iter()
    .map(|rel| workspace_root.join(rel))
    .filter(|p| p.is_file())
    .map(|p| {
        let mt = p
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::UNIX_EPOCH);
        (p, mt)
    })
    .collect();
    existing.sort_by_key(|(_, mt)| std::cmp::Reverse(*mt));
    if let Some((newest, _)) = existing.into_iter().next() {
        return Ok(newest);
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
        .context("building host port-runner for static Mach-O AOT emission")?;
    if !status.success() {
        bail!("building host port-runner failed with status {status}");
    }
    let p = workspace_root.join("target/aarch64-apple-darwin/debug/port-runner");
    if !p.is_file() {
        bail!("host port-runner binary not found at {}", p.display());
    }
    Ok(p)
}

fn copy_dir_recursive(src: &Path, dst: &Path, skip_canon: Option<&Path>) -> Result<()> {
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

/// Package a Windows x86-64 executable/game directory into a complete iOS `.app` or `.ipa`
/// bundle with statically linked signed ARM64 `.aot.o` code and precompiled Tier-1 shaders.
pub fn build_ios_bundle(opts: &IosBundleOptions) -> Result<IosBundleReport> {
    let (game_dir, primary_exe, exe_rel, manifest_src, parsed_manifest) = resolve_input(opts)?;

    let raw_title = opts
        .title_override
        .clone()
        .or(parsed_manifest.name.clone())
        .or_else(|| {
            primary_exe
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "PortHost".to_string());
    let title = sanitize_game_title(&raw_title);
    let exec_name = sanitize_exec_name(&title);
    let slug = parsed_manifest
        .id
        .clone()
        .map(|s| sanitize_identifier(&s))
        .unwrap_or_else(|| sanitize_identifier(&title));
    let bundle_id = opts
        .bundle_id
        .clone()
        .unwrap_or_else(|| format!("com.portfactory.{slug}"));

    let is_ipa = opts
        .out_path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("ipa"));

    let stage_root = std::env::temp_dir().join(format!(
        "port-ios-bundle-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos())
    ));
    fs::create_dir_all(&stage_root)?;
    struct CleanStage(PathBuf);
    impl Drop for CleanStage {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    let _clean = CleanStage(stage_root.clone());

    let app_dir = if is_ipa {
        stage_root.join("Payload").join(format!("{title}.app"))
    } else {
        if opts.out_path.exists() {
            fs::remove_dir_all(&opts.out_path)
                .with_context(|| format!("replacing existing {}", opts.out_path.display()))?;
        }
        if let Some(parent) = opts.out_path.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        opts.out_path.clone()
    };
    fs::create_dir_all(&app_dir)?;

    // 1. Write or copy `game.toml` (and any `[ios] seed_file` assets) into `<App>.app/`.
    let bundled_manifest = app_dir.join("game.toml");
    if let Some(ref src_m) = manifest_src {
        fs::copy(src_m, &bundled_manifest)
            .with_context(|| format!("copying manifest {}", src_m.display()))?;
        if let Some(src_dir) = src_m.parent() {
            for (src_rel, _) in &parsed_manifest.ios_seed_files {
                let src_file = src_dir.join(src_rel);
                if src_file.is_file() {
                    let dst_file = app_dir.join(src_rel);
                    if let Some(parent) = dst_file.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    fs::copy(&src_file, &dst_file).with_context(|| {
                        format!("copying iOS seed file {}", src_file.display())
                    })?;
                }
            }
        }
    } else {
        let fallback_manifest = GameManifest {
            id: Some(slug.clone()),
            name: Some(title.clone()),
            exe: Some(exe_rel.clone()),
            win_dir: Some(format!("C:\\Games\\{exec_name}")),
            preferred_base_retry: Some(false),
            ..GameManifest::default()
        };
        fs::write(&bundled_manifest, fallback_manifest.to_toml())?;
    }

    // 2. Build (or reuse) `libport_loader.a` for the requested iOS target.
    let staticlib = ensure_ios_staticlib(
        &opts.workspace_root,
        opts.target,
        opts.release,
        &opts.deployment_target,
    )?;
    let lib_dir = staticlib
        .parent()
        .ok_or_else(|| anyhow::anyhow!("staticlib has no parent directory"))?;

    // 3. Offline-compile mapped PEs into signed-ready ARM64 Mach-O `.aot.o` objects.
    let mut aot_objects: Vec<PathBuf> = opts.extra_aot_objs.clone();
    let aot_stage_dir = stage_root.join("aot_objs");
    if opts.emit_static_aot {
        let runner = ensure_host_port_runner(&opts.workspace_root)?;
        fs::create_dir_all(&aot_stage_dir)?;
        let mut cmd = Command::new(&runner);
        cmd.arg("--manifest")
            .arg(&bundled_manifest)
            .arg("--game-dir")
            .arg(&game_dir)
            .arg("--pe")
            .arg(&primary_exe)
            .arg("--emit-static-o")
            .arg(&aot_stage_dir)
            .arg("--static-platform")
            .arg(opts.target.static_macho_platform());
        if opts.aot_max_funcs > 0 {
            cmd.arg("--aot-max-funcs").arg(opts.aot_max_funcs.to_string());
        }
        let out = cmd
            .output()
            .with_context(|| format!("running {} --emit-static-o", runner.display()))?;
        if !out.stdout.is_empty() {
            print!("{}", String::from_utf8_lossy(&out.stdout));
        }
        if !out.status.success() {
            bail!(
                "port-runner --emit-static-o failed (status {}): {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        let mut emitted: Vec<PathBuf> = fs::read_dir(&aot_stage_dir)?
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("o"))
            .collect();
        emitted.sort();
        aot_objects.extend(emitted);
    }

    // 4. Compile & link the native ARM64 iOS host binary (`ios/PortHost/*.swift` + `.aot.o` + `libport_loader.a`).
    let sdk_out = Command::new("xcrun")
        .args(["--sdk", opts.target.sdk_name(), "--show-sdk-path"])
        .output()
        .with_context(|| format!("resolving {} SDK path via xcrun", opts.target.sdk_name()))?;
    if !sdk_out.status.success() {
        bail!(
            "xcrun --sdk {} --show-sdk-path failed: {}",
            opts.target.sdk_name(),
            String::from_utf8_lossy(&sdk_out.stderr).trim()
        );
    }
    let sdk_path = String::from_utf8_lossy(&sdk_out.stdout).trim().to_string();

    let host_dir = opts.workspace_root.join("ios/PortHost");
    let bridging_header = host_dir.join("PortHost-Bridging-Header.h");
    let swift_sources = [
        host_dir.join("PortHostApp.swift"),
        host_dir.join("MetalSurfaceView.swift"),
        host_dir.join("KeyboardBridge.swift"),
        host_dir.join("PhoneHaptics.swift"),
        host_dir.join("TouchOverlayView.swift"),
    ];
    let entitlements = host_dir.join("PortHost.entitlements");
    if !bridging_header.is_file() {
        bail!("required iOS host file missing: {}", bridging_header.display());
    }
    for req in &swift_sources {
        if !req.is_file() {
            bail!("required iOS host file missing: {}", req.display());
        }
    }

    let exec_path = app_dir.join(&exec_name);
    let mut swift = Command::new("xcrun");
    swift
        .args(["--sdk", opts.target.sdk_name(), "swiftc"])
        .arg("-target")
        .arg(opts.target.swift_triple(&opts.deployment_target))
        .arg("-sdk")
        .arg(&sdk_path)
        .arg("-import-objc-header")
        .arg(&bridging_header);
    for src in &swift_sources {
        swift.arg(src);
    }
    if opts.release {
        swift.arg("-O");
    }
    for obj in &aot_objects {
        swift.arg(obj);
    }
    swift
        .arg("-L")
        .arg(lib_dir)
        .arg("-lport_loader")
        .arg("-lz")
        .args(["-Xlinker", "-dead_strip"]);
    let mut frameworks = vec![
        "Metal",
        "QuartzCore",
        "UIKit",
        "Foundation",
        "CoreFoundation",
        "AudioToolbox",
        "AVFAudio",
        "GameController",
        "CoreHaptics",
        "VideoToolbox",
        "CoreMedia",
        "CoreVideo",
        "Security",
    ];
    if opts.target == IosTarget::Device {
        frameworks.push("MetalFX");
    }
    for fw in frameworks {
        swift.arg("-framework").arg(fw);
    }
    swift.arg("-o").arg(&exec_path);

    let link_out = swift.output().context("invoking xcrun swiftc to link iOS binary")?;
    if !link_out.status.success() {
        bail!(
            "iOS swiftc link failed (status {}):\n{}\n{}",
            link_out.status,
            String::from_utf8_lossy(&link_out.stdout),
            String::from_utf8_lossy(&link_out.stderr),
        );
    }

    // 5. Write `Info.plist`, `PkgInfo`, and optional `embedded.mobileprovision`.
    fs::write(
        app_dir.join("Info.plist"),
        render_ios_info_plist(&title, &exec_name, &bundle_id, &opts.deployment_target, opts.target),
    )?;
    fs::write(app_dir.join("PkgInfo"), b"APPL????")?;
    if let Some(ref prof) = opts.provisioning_profile {
        fs::copy(prof, app_dir.join("embedded.mobileprovision"))
            .with_context(|| format!("copying provisioning profile {}", prof.display()))?;
    }

    // 6. Always copy or optimize all game files (EXEs, DLLs, assets) directly inside `<App>.app/Game/`.
    let bundle_game_dir = app_dir.join("Game");
    let mut storage_opt_report: Option<StorageOptReport> = None;
    let active_sopt = opts.storage_opt.as_ref().filter(|o| o.is_enabled()).map(|base| {
        let mut sopt = base.clone();
        sopt.workspace_root = Some(opts.workspace_root.clone());
        if let Ok(text) = fs::read_to_string(&bundled_manifest) {
            let mcfg = parse_manifest_storage_config(&text);
            sopt.apply_manifest_config(&mcfg);
        }
        // Always keep any DLL whose static AOT object is linked into the executable.
        for obj in &aot_objects {
            if let Some(fname) = obj.file_name().and_then(|s| s.to_str()) {
                let lower = fname.to_ascii_lowercase();
                if let Some(mod_name) = lower.strip_suffix(".aot.o") {
                    if mod_name.ends_with(".dll") {
                        sopt.keep_dlls.insert(mod_name.to_string());
                    }
                }
            }
        }
        sopt
    });

    let skip_self = fs::canonicalize(&app_dir).ok();
    if let Some(ref sopt) = active_sopt {
        let rep = optimize_game_assets(&game_dir, &bundle_game_dir, sopt, skip_self.as_deref())
            .with_context(|| format!("optimizing game files from {}", game_dir.display()))?;
        storage_opt_report = Some(rep);
    } else {
        copy_dir_recursive(&game_dir, &bundle_game_dir, skip_self.as_deref())
            .with_context(|| format!("copying game files from {}", game_dir.display()))?;
    }

    // 7. Precompile embedded PE shaders & copy optional portable Tier-1 ShaderCache (no `*.mtl4a`).
    let shader_cache_dir = app_dir.join("ShaderCache");
    fs::create_dir_all(&shader_cache_dir)?;
    let mut shaders_bundled = 0usize;
    if opts.precompile_shaders {
        if let Ok(stats) = precompile_shaders_for_paths(
            std::slice::from_ref(&game_dir),
            Some(shader_cache_dir.clone()),
            1024,
        ) {
            shaders_bundled += stats.active_wave32_converted;
        }
    }
    if let Some(ref src_cache) = opts.bundle_shader_cache_from {
        if src_cache.is_dir() {
            shaders_bundled += copy_ios_shader_cache(src_cache, &shader_cache_dir)?;
        }
    }

    // 8. Code-sign the `<App>.app` bundle.
    let identity = opts.codesign_identity.as_deref().unwrap_or("-");
    let mut cs = Command::new("codesign");
    cs.args(["--force", "--sign", identity]);
    // Device builds attach the extended-virtual-addressing + increased-memory-limit entitlements.
    // iOS Simulator rejects restricted `com.apple.developer.kernel.*` entitlements on ad-hoc signed
    // bundles, so Simulator bundles use plain ad-hoc signing.
    if opts.target == IosTarget::Device && entitlements.is_file() {
        cs.arg("--entitlements").arg(&entitlements);
    }
    cs.arg(&app_dir);
    let cs_out = cs.output().context("running codesign on iOS .app bundle")?;
    let signed = cs_out.status.success();
    if !signed {
        bail!(
            "codesign failed on {} (status {}): {}",
            app_dir.display(),
            cs_out.status,
            String::from_utf8_lossy(&cs_out.stderr).trim()
        );
    }

    // 9. Package `.ipa` archive (`Payload/<App>.app`) when requested.
    if is_ipa {
        if let Some(parent) = opts.out_path.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        if opts.out_path.exists() {
            fs::remove_file(&opts.out_path)?;
        }
        let abs_ipa = if opts.out_path.is_absolute() {
            opts.out_path.clone()
        } else {
            std::env::current_dir()?.join(&opts.out_path)
        };
        let zip_out = Command::new("/usr/bin/zip")
            .current_dir(&stage_root)
            .args(["-ryq"])
            .arg(&abs_ipa)
            .arg("Payload")
            .output()
            .context("packaging Payload/<App>.app into .ipa via /usr/bin/zip")?;
        if !zip_out.status.success() {
            bail!(
                "zip failed creating {} (status {}): {}",
                abs_ipa.display(),
                zip_out.status,
                String::from_utf8_lossy(&zip_out.stderr).trim()
            );
        }
    }

    // 10. Optional XcodeGen `project.yml` emission next to `out_path`.
    let xcodegen_spec_path = if opts.emit_xcodegen_spec {
        let parent = opts
            .out_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let aot_persist_dir = parent.join(format!("{exec_name}_StaticAot"));
        let mut persisted_objs = Vec::new();
        if !aot_objects.is_empty() {
            fs::create_dir_all(&aot_persist_dir)?;
            for obj in &aot_objects {
                if let Some(fname) = obj.file_name() {
                    let dst = aot_persist_dir.join(fname);
                    fs::copy(obj, &dst)?;
                    persisted_objs.push(dst);
                }
            }
        }
        let spec_path = parent.join(format!("{exec_name}.project.yml"));
        let yaml = render_xcodegen_spec(
            &exec_name,
            &bundle_id,
            &opts.deployment_target,
            &opts.workspace_root,
            &persisted_objs,
            &bundled_manifest,
            Some(&bundle_game_dir),
            Some(&shader_cache_dir),
            opts.release,
        );
        fs::write(&spec_path, yaml)?;
        Some(spec_path)
    } else {
        None
    };

    // 11. Run built-in iOS bundle verification on the finished output.
    let verify = verify_ios_bundle(&opts.out_path)?;

    Ok(IosBundleReport {
        title,
        bundle_id,
        executable_name: exec_name,
        target: opts.target,
        output_path: opts.out_path.clone(),
        is_ipa,
        primary_exe_rel: exe_rel,
        static_aot_objects: aot_objects,
        shaders_bundled,
        signed,
        xcodegen_spec_path,
        storage_opt_report,
        verify,
    })
}

/// Inspect a Mach-O 64-bit binary header and extract `(arch_name, platform_id, platform_name)`.
fn parse_macho_header_platform(bytes: &[u8]) -> Result<(String, u32, String)> {
    if bytes.len() < 32 {
        bail!("binary too short ({} bytes) to be a Mach-O executable", bytes.len());
    }
    let rd32 = |off: usize| -> u32 { u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap()) };
    let magic = rd32(0);
    if magic != 0xFEED_FACF {
        bail!("unexpected Mach-O magic {magic:#010x} (expected MH_MAGIC_64 0xfeedfacf)");
    }
    let cputype = rd32(4);
    let arch = match cputype {
        0x0100_000C => "arm64".to_string(),
        0x0100_0007 => "x86_64".to_string(),
        other => format!("cputype({other:#x})"),
    };
    let ncmds = rd32(16) as usize;
    let mut off = 32usize;
    let mut plat_id = 0u32;
    for _ in 0..ncmds {
        if off + 8 > bytes.len() {
            break;
        }
        let cmd = rd32(off);
        let cmdsize = rd32(off + 4) as usize;
        if cmdsize < 8 || off + cmdsize > bytes.len() {
            break;
        }
        if cmd == 0x32 && cmdsize >= 24 {
            // LC_BUILD_VERSION
            plat_id = rd32(off + 8);
            break;
        }
        off += cmdsize;
    }
    let plat_name = match plat_id {
        1 => "macOS",
        2 => "iOS",
        7 => "iOS-Simulator",
        0 => "unknown",
        _ => "other",
    }
    .to_string();
    Ok((arch, plat_id, plat_name))
}

fn plist_string_value(xml: &str, key: &str) -> Option<String> {
    let needle = format!("<key>{key}</key>");
    let idx = xml.find(&needle)?;
    let rest = &xml[idx + needle.len()..];
    let start = rest.find("<string>")? + "<string>".len();
    let tail = &rest[start..];
    let end = tail.find("</string>")?;
    Some(tail[..end].trim().to_string())
}

fn count_shader_files(dir: &Path, portable: &mut usize, mtl4a: &mut usize) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() {
            count_shader_files(&p, portable, mtl4a);
        } else if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            if ext.eq_ignore_ascii_case("aw2s") {
                *portable += 1;
            } else if ext.eq_ignore_ascii_case("mtl4a") {
                *mtl4a += 1;
            }
        }
    }
}

/// Verify an iOS `.app` bundle or `.ipa` archive for ARM64 iOS Mach-O platform validity,
/// embedded static AOT modules, zero macOS framework leaks, flat bundle layout, portable
/// shader cache hygiene (zero host-specific `*.mtl4a` files), and valid code signature.
fn read_plist_as_xml(path: &Path) -> Result<String, String> {
    let raw = fs::read(path).map_err(|e| e.to_string())?;
    if raw.starts_with(b"bplist") {
        let out = Command::new("/usr/bin/plutil")
            .args(["-convert", "xml1", "-o", "-"])
            .arg(path)
            .output()
            .map_err(|e| format!("plutil failed: {e}"))?;
        if !out.status.success() {
            return Err(format!(
                "plutil -convert xml1 failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        return String::from_utf8(out.stdout).map_err(|e| e.to_string());
    }
    String::from_utf8(raw).map_err(|e| e.to_string())
}

pub fn verify_ios_bundle(path: &Path) -> Result<IosBundleVerifyReport> {
    if !path.exists() {
        bail!("bundle not found: {}", path.display());
    }
    let is_ipa = path.is_file()
        && path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("ipa"));

    let mut temp_extract: Option<PathBuf> = None;
    let app_dir = if is_ipa {
        let tmp = std::env::temp_dir().join(format!(
            "port-ios-verify-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos())
        ));
        fs::create_dir_all(&tmp)?;
        let out = Command::new("/usr/bin/unzip")
            .args(["-qq"])
            .arg(path)
            .arg("-d")
            .arg(&tmp)
            .output()
            .context("extracting .ipa via /usr/bin/unzip")?;
        if !out.status.success() {
            let _ = fs::remove_dir_all(&tmp);
            bail!(
                "unzip failed on {}: {}",
                path.display(),
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        let payload = tmp.join("Payload");
        let found = fs::read_dir(&payload)
            .ok()
            .and_then(|mut rd| {
                rd.find_map(|e| {
                    let p = e.ok()?.path();
                    (p.is_dir() && p.extension().and_then(|x| x.to_str()) == Some("app"))
                        .then_some(p)
                })
            })
            .ok_or_else(|| {
                let _ = fs::remove_dir_all(&tmp);
                anyhow::anyhow!("{} does not contain Payload/<App>.app", path.display())
            })?;
        temp_extract = Some(tmp);
        found
    } else {
        path.to_path_buf()
    };

    struct Cleanup(Option<PathBuf>);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            if let Some(ref p) = self.0 {
                let _ = fs::remove_dir_all(p);
            }
        }
    }
    let _cleanup = Cleanup(temp_extract);

    let mut issues = Vec::new();
    let plist_path = app_dir.join("Info.plist");
    let plist_xml = match read_plist_as_xml(&plist_path) {
        Ok(s) => s,
        Err(e) => {
            issues.push(format!("missing Info.plist: {e}"));
            String::new()
        }
    };
    let bundle_id = plist_string_value(&plist_xml, "CFBundleIdentifier").unwrap_or_default();
    let executable_name = plist_string_value(&plist_xml, "CFBundleExecutable").unwrap_or_default();
    if bundle_id.is_empty() {
        issues.push("Info.plist missing CFBundleIdentifier".into());
    }
    if executable_name.is_empty() {
        issues.push("Info.plist missing CFBundleExecutable".into());
    }
    if !plist_xml.is_empty() && !plist_xml.contains("<key>UILaunchScreen</key>") {
        issues.push("Info.plist missing UILaunchScreen (iOS will letterbox to 320x480)".into());
    }
    if !plist_xml.is_empty() && !plist_xml.contains("<key>UIApplicationSceneManifest</key>") {
        issues.push("Info.plist missing UIApplicationSceneManifest (required for UIScene lifecycle on iOS 26+)".into());
    }
    if plist_xml.contains("<key>UIFileSharingEnabled</key>")
        || plist_xml.contains("<key>LSSupportsOpeningDocumentsInPlace</key>")
    {
        issues.push("Info.plist exposes Documents file sharing; game payloads must be bundled inside Game/".into());
    }

    let exec_path = app_dir.join(&executable_name);
    let mut macho_arch = String::from("missing");
    let mut macho_platform_id = 0u32;
    let mut macho_platform_name = String::from("unknown");
    let mut static_aot_modules = 0usize;
    let mut macos_framework_leaks = Vec::new();

    if executable_name.is_empty() || !exec_path.is_file() {
        issues.push(format!("main executable missing: {}", exec_path.display()));
    } else {
        let bin_bytes = fs::read(&exec_path)?;
        match parse_macho_header_platform(&bin_bytes) {
            Ok((arch, pid, pname)) => {
                macho_arch = arch;
                macho_platform_id = pid;
                macho_platform_name = pname;
                if macho_arch != "arm64" {
                    issues.push(format!("executable arch is {macho_arch}, expected arm64"));
                }
                if macho_platform_id != 2 && macho_platform_id != 7 {
                    issues.push(format!(
                        "executable LC_BUILD_VERSION platform is {macho_platform_name} ({macho_platform_id}), expected iOS (2) or iOS-Simulator (7)"
                    ));
                }
            }
            Err(e) => issues.push(format!("invalid Mach-O executable: {e}")),
        }

        if let Ok(nm_out) = Command::new("nm").arg("-n").arg(&exec_path).output() {
            if nm_out.status.success() {
                let text = String::from_utf8_lossy(&nm_out.stdout);
                static_aot_modules = text
                    .lines()
                    .filter(|l| l.contains("_port_static_aot_desc_"))
                    .count();
            }
        }
        if let Ok(ot_out) = Command::new("otool").arg("-L").arg(&exec_path).output() {
            if ot_out.status.success() {
                let text = String::from_utf8_lossy(&ot_out.stdout);
                for forbidden in ["AppKit.framework", "IOKit.framework", "CoreAudio.framework"] {
                    if text.contains(forbidden) {
                        macos_framework_leaks.push(forbidden.to_string());
                    }
                }
            }
        }
        if !macos_framework_leaks.is_empty() {
            issues.push(format!(
                "links macOS-only framework(s): {}",
                macos_framework_leaks.join(", ")
            ));
        }
    }

    let game_toml_path = app_dir.join("game.toml");
    let has_game_toml = game_toml_path.is_file();
    if !has_game_toml {
        issues.push("missing flat bundle game.toml".into());
    }
    let game_dir = app_dir.join("Game");
    let game_meta = fs::symlink_metadata(&game_dir).ok();
    let has_game_dir = game_meta
        .as_ref()
        .is_some_and(|m| m.is_dir() && !m.file_type().is_symlink());
    if game_meta
        .as_ref()
        .is_some_and(|m| m.file_type().is_symlink())
    {
        issues.push("Game/ must be a real bundled directory inside the .app, not a symlink".into());
    } else if !has_game_dir {
        issues.push("missing flat bundle Game/ directory".into());
    }
    let mut bundled_exe_present = false;
    let game_dir_has_files = has_game_dir
        && fs::read_dir(&game_dir)
            .ok()
            .and_then(|mut rd| rd.next())
            .is_some();
    if has_game_dir && !game_dir_has_files {
        issues.push("Game/ directory is empty (all game EXEs, DLLs, and assets must be bundled inside the .app)".into());
    }
    if has_game_toml {
        if let Ok(text) = fs::read_to_string(&game_toml_path) {
            let m = GameManifest::parse(&text);
            if let Some(exe_rel) = m.exe {
                bundled_exe_present = game_dir.join(&exe_rel).is_file();
                if !bundled_exe_present {
                    issues.push(format!("Game/{exe_rel} referenced by game.toml is missing"));
                }
            }
        }
    }

    let mut portable_shader_files = 0usize;
    let mut host_mtl4a_files = 0usize;
    count_shader_files(
        &app_dir.join("ShaderCache"),
        &mut portable_shader_files,
        &mut host_mtl4a_files,
    );
    if host_mtl4a_files > 0 {
        issues.push(format!(
            "ShaderCache contains {host_mtl4a_files} host-specific *.mtl4a archive(s) (not portable to iOS)"
        ));
    }

    let codesign_ok = Command::new("codesign")
        .args(["--verify", "--deep", "--strict"])
        .arg(&app_dir)
        .output()
        .is_ok_and(|o| o.status.success());

    Ok(IosBundleVerifyReport {
        bundle_path: path.to_path_buf(),
        is_ipa,
        bundle_id,
        executable_name,
        macho_arch,
        macho_platform_id,
        macho_platform_name,
        static_aot_modules,
        has_game_toml,
        has_game_dir,
        bundled_exe_present,
        portable_shader_files,
        host_mtl4a_files,
        macos_framework_leaks,
        codesign_ok,
        issues,
    })
}

pub fn format_ios_bundle_verify_report(r: &IosBundleVerifyReport) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "=== iOS Bundle Verification ({}) ===\n",
        r.bundle_path.display()
    ));
    s.push_str(&format!(
        "  Format              : {}\n",
        if r.is_ipa { ".ipa archive" } else { ".app bundle" }
    ));
    s.push_str(&format!("  Bundle ID           : {}\n", r.bundle_id));
    s.push_str(&format!("  Executable          : {}\n", r.executable_name));
    s.push_str(&format!(
        "  Mach-O Target       : {} / {} (platform {})\n",
        r.macho_arch, r.macho_platform_name, r.macho_platform_id
    ));
    s.push_str(&format!(
        "  Static AOT Modules  : {} linked in signed __TEXT\n",
        r.static_aot_modules
    ));
    s.push_str(&format!(
        "  Flat Bundle Layout  : game.toml={} Game/={} primary_exe={}\n",
        r.has_game_toml, r.has_game_dir, r.bundled_exe_present
    ));
    s.push_str(&format!(
        "  ShaderCache         : {} portable .aw2s, {} host .mtl4a\n",
        r.portable_shader_files, r.host_mtl4a_files
    ));
    s.push_str(&format!(
        "  Code Signature      : {}\n",
        if r.codesign_ok { "valid" } else { "unsigned / invalid" }
    ));
    if r.issues.is_empty() {
        s.push_str("  Verdict             : [OK] Ready for iOS\n");
    } else {
        s.push_str(&format!(
            "  Verdict             : [FAIL] {} issue(s)\n",
            r.issues.len()
        ));
        for i in &r.issues {
            s.push_str(&format!("    - {i}\n"));
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ios_target_metadata_matches_apple_sdks() {
        assert_eq!(IosTarget::parse("device"), Some(IosTarget::Device));
        assert_eq!(IosTarget::parse("ios"), Some(IosTarget::Device));
        assert_eq!(IosTarget::parse("sim"), Some(IosTarget::Simulator));
        assert_eq!(IosTarget::parse("ios-sim"), Some(IosTarget::Simulator));
        assert_eq!(IosTarget::parse("bogus"), None);

        assert_eq!(IosTarget::Device.rust_target(), "aarch64-apple-ios");
        assert_eq!(IosTarget::Simulator.rust_target(), "aarch64-apple-ios-sim");
        assert_eq!(IosTarget::Device.swift_triple("17.0"), "arm64-apple-ios17.0");
        assert_eq!(
            IosTarget::Simulator.swift_triple("17.0"),
            "arm64-apple-ios17.0-simulator"
        );
        assert_eq!(IosTarget::Device.expected_macho_platform_id(), 2);
        assert_eq!(IosTarget::Simulator.expected_macho_platform_id(), 7);
    }

    #[test]
    fn info_plist_and_xcodegen_spec_contain_required_ios_keys() {
        let plist = render_ios_info_plist(
            "Test & Game",
            "TestGame",
            "com.portfactory.test-game",
            "17.0",
            IosTarget::Device,
        );
        assert!(plist.contains("<string>Test &amp; Game</string>"));
        assert!(plist.contains("<key>UILaunchScreen</key>"));
        assert!(plist.contains("<string>arm64</string>"));
        assert!(plist.contains("<string>metal</string>"));
        assert!(plist.contains("<string>iphoneos</string>"));
        assert!(!plist.contains("<key>UIFileSharingEnabled</key>"));
        assert!(!plist.contains("<key>LSSupportsOpeningDocumentsInPlace</key>"));

        let yaml = render_xcodegen_spec(
            "TestGame",
            "com.portfactory.test-game",
            "17.0",
            Path::new("/workspace"),
            &[PathBuf::from("/workspace/out/foo.dll.aot.o")],
            Path::new("/workspace/out/game.toml"),
            Some(Path::new("/workspace/out/Game")),
            Some(Path::new("/workspace/out/ShaderCache")),
            true,
        );
        assert!(yaml.contains("PRODUCT_BUNDLE_IDENTIFIER: \"com.portfactory.test-game\""));
        assert!(yaml.contains("DEAD_CODE_STRIPPING: YES"));
        assert!(yaml.contains("foo.dll.aot.o"));
        assert!(yaml.contains("target/aarch64-apple-ios/release"));
    }

    #[test]
    fn verify_ios_bundle_catches_macos_binary_and_mtl4a_bloat() {
        let tmp = std::env::temp_dir().join(format!("port-ios-verify-unit-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let app = tmp.join("Bad.app");
        fs::create_dir_all(app.join("Game")).unwrap();
        fs::create_dir_all(app.join("ShaderCache/pso")).unwrap();
        fs::write(
            app.join("Info.plist"),
            render_ios_info_plist("Bad", "BadBin", "com.portfactory.bad", "17.0", IosTarget::Device),
        )
        .unwrap();
        // Synthesize a minimal ARM64 Mach-O header tagged as macOS (platform 1), NOT iOS!
        let mut macho = vec![0u8; 64];
        macho[0..4].copy_from_slice(&0xFEED_FACFu32.to_le_bytes());
        macho[4..8].copy_from_slice(&0x0100_000Cu32.to_le_bytes());
        macho[16..20].copy_from_slice(&1u32.to_le_bytes()); // ncmds = 1
        macho[32..36].copy_from_slice(&0x32u32.to_le_bytes()); // LC_BUILD_VERSION
        macho[36..40].copy_from_slice(&24u32.to_le_bytes());
        macho[40..44].copy_from_slice(&1u32.to_le_bytes()); // PLATFORM_MACOS
        fs::write(app.join("BadBin"), &macho).unwrap();
        fs::write(app.join("ShaderCache/pso/host.mtl4a"), b"bloat").unwrap();

        let rep = verify_ios_bundle(&app).unwrap();
        assert!(!rep.ok());
        assert_eq!(rep.macho_platform_id, 1);
        assert_eq!(rep.host_mtl4a_files, 1);
        assert!(rep.issues.iter().any(|i| i.contains("platform is macOS")));
        assert!(rep.issues.iter().any(|i| i.contains("missing flat bundle game.toml")));
        assert!(rep.issues.iter().any(|i| i.contains("Game/ directory is empty")));
        assert!(rep.issues.iter().any(|i| i.contains("host-specific *.mtl4a")));

        // Also ensure Documents file sharing keys and symlinked Game/ fail verification.
        let bad_plist = render_ios_info_plist("Bad", "BadBin", "com.portfactory.bad", "17.0", IosTarget::Device)
            .replace("</dict>", "    <key>UIFileSharingEnabled</key>\n    <true/>\n</dict>");
        fs::write(app.join("Info.plist"), bad_plist).unwrap();
        let ext_dir = tmp.join("ExternalGame");
        fs::create_dir_all(&ext_dir).unwrap();
        fs::remove_dir_all(app.join("Game")).unwrap();
        std::os::unix::fs::symlink(&ext_dir, app.join("Game")).unwrap();
        let rep2 = verify_ios_bundle(&app).unwrap();
        assert!(!rep2.ok());
        assert!(rep2.issues.iter().any(|i| i.contains("Documents file sharing")));
        assert!(rep2.issues.iter().any(|i| i.contains("not a symlink")));
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn copy_dir_recursive_skips_output_bundle_inside_source_dir() {
        let tmp = std::env::temp_dir().join(format!("port-ios-copy-self-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        fs::write(tmp.join("demo.exe"), b"MZ").unwrap();
        let app = tmp.join("Out.app");
        let dst = app.join("Game");
        fs::create_dir_all(&app).unwrap();
        let skip = fs::canonicalize(&app).unwrap();
        copy_dir_recursive(&tmp, &dst, Some(&skip)).unwrap();
        assert!(dst.join("demo.exe").is_file());
        assert!(!dst.join("Out.app").exists());
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn copy_ios_shader_cache_skips_manifest_pso_and_mtl4a() {
        let tmp = std::env::temp_dir().join(format!("port-ios-sc-trim-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let src = tmp.join("src");
        let dst = tmp.join("dst");
        fs::create_dir_all(src.join("ir4.0.1-test")).unwrap();
        fs::create_dir_all(src.join("manifest")).unwrap();
        fs::create_dir_all(src.join("pso/pso_manifest")).unwrap();
        fs::write(src.join("ir4.0.1-test/abc.aw2s"), b"ir").unwrap();
        fs::write(src.join("ir4.0.1-test/bad.mtl4a"), b"host").unwrap();
        fs::write(src.join("manifest/raw.dxil"), b"dxil").unwrap();
        fs::write(src.join("pso/pso_manifest/p.bin"), b"pso").unwrap();

        let copied = copy_ios_shader_cache(&src, &dst).unwrap();
        assert_eq!(copied, 1);
        assert!(dst.join("ir4.0.1-test/abc.aw2s").is_file());
        assert!(!dst.join("ir4.0.1-test/bad.mtl4a").exists());
        assert!(!dst.join("manifest").exists());
        assert!(!dst.join("pso").exists());
        let _ = fs::remove_dir_all(&tmp);
    }
}
