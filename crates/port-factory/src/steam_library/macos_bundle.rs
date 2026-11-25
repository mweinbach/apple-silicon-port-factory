//! One-click macOS `.app` / AOT crate / Metal shader converter (`steam-convert`),
//! framework staging (`libmetalirconverter`, `libdxilconv`, `libMoltenVK`), manifest
//! generation, and native bundle launcher codegen.

use super::discovery::{
    inspect_game_directory, scan_all_steam_libraries, GameLibrarySource, InstalledGameEntry,
};
use super::html::html_escape;
use super::vdf::apply_post_install;
use crate::aot::{compile_pe_file_to_rust, AotOptions};
use crate::bundle_common::{
    codesign_ad_hoc, copy_game_payload, copy_portable_shader_cache, find_or_build_release_binary,
    otool_install_names, set_executable_mode,
};
use crate::gpu_shaders::precompile_shaders_for_paths;
use crate::scaffold::generate_embeddable_aot_crate;
use crate::scanner::collect_pe_files;
use crate::win32_db::Win32Database;
use anyhow::{bail, Context, Result};
use pe_image::Pe;
use port_manifest::GameManifest;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const BUNDLE_LAUNCHER_TEMPLATE: &str = include_str!("../../assets/bundle_launcher.rs.tmpl");

/// Target conversion format for `steam-convert`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionFormat {
    /// Self-contained macOS `.app` bundle with `port-runner`, `dxil-metald`,
    /// `steam_appid.txt`, `steam_registry.tsv`, prewarmed `.aot` block caches,
    /// and pre-converted Metal shaders.
    MacOsApp,
    /// Standalone embeddable Rust AOT crate (`Cargo.toml` + `src/lib.rs`).
    AotCrate,
    /// Pre-compiled Metal `.metallib` shader cache only.
    MetalShaders,
}

impl ConversionFormat {
    pub fn parse(s: &str) -> Result<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "app" | "macos" | "macos-app" | "bundle" | "hybrid" => Ok(Self::MacOsApp),
            "aot" | "aot-crate" | "rust" | "crate" => Ok(Self::AotCrate),
            "shaders" | "metal" | "metallib" | "shaders-only" => Ok(Self::MetalShaders),
            other => bail!(
                "unknown conversion format {other:?} (expected: app, aot-crate, or shaders)"
            ),
        }
    }
}

/// Options for one-click converting an installed Steam game or Windows `.exe` directory.
#[derive(Debug, Clone)]
pub struct SteamConvertOptions {
    pub target: String,
    pub format: ConversionFormat,
    pub output_path: Option<PathBuf>,
    pub precompile_shaders: bool,
    pub max_aot_functions: usize,
    /// Optional host shader cache root (e.g. `~/Translator/ShaderCache`) whose portable
    /// Tier-1 entries (`ir*`, `manifest/`, `pso/pso_manifest/`, excluding host-specific
    /// `*.mtl4a` archives) are copied into `Contents/Resources/ShaderCache/`.
    pub bundle_shader_cache_from: Option<PathBuf>,
}

/// Environment inputs used to locate host runtime frameworks when staging an `.app` bundle.
#[derive(Debug, Clone)]
pub struct FrameworkDiscoveryEnv {
    pub home: Option<PathBuf>,
    pub metal_irconverter_dylib: Option<PathBuf>,
    pub aw2_dxilconv: Option<PathBuf>,
    pub port_vulkan_dylib: Option<PathBuf>,
    pub port_moltenvk_dylib: Option<PathBuf>,
    pub moltenvk_dylib: Option<PathBuf>,
    pub vulkan_sdk: Option<PathBuf>,
    /// Include standard macOS system prefixes (`/opt/homebrew/lib`, `/usr/local/lib`,
    /// `/opt/metal-shaderconverter/lib`). Set `false` in isolated unit tests.
    pub include_system_paths: bool,
}

impl Default for FrameworkDiscoveryEnv {
    fn default() -> Self {
        Self {
            home: None,
            metal_irconverter_dylib: None,
            aw2_dxilconv: None,
            port_vulkan_dylib: None,
            port_moltenvk_dylib: None,
            moltenvk_dylib: None,
            vulkan_sdk: None,
            include_system_paths: true,
        }
    }
}

impl FrameworkDiscoveryEnv {
    pub fn from_process_env() -> Self {
        let env_path = |key: &str| {
            std::env::var_os(key)
                .filter(|v| !v.is_empty())
                .map(PathBuf::from)
        };
        Self {
            home: env_path("HOME"),
            metal_irconverter_dylib: env_path("METAL_IRCONVERTER_DYLIB"),
            aw2_dxilconv: env_path("AW2_DXILCONV"),
            port_vulkan_dylib: env_path("PORT_VULKAN_DYLIB"),
            port_moltenvk_dylib: env_path("PORT_MOLTENVK_DYLIB"),
            moltenvk_dylib: env_path("MOLTENVK_DYLIB"),
            vulkan_sdk: env_path("VULKAN_SDK"),
            include_system_paths: true,
        }
    }
}

/// Runtime dynamic libraries copied into `<App>.app/Contents/Frameworks/`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StagedFrameworks {
    pub metal_irconverter: Option<PathBuf>,
    pub dxilconv: Option<PathBuf>,
    pub moltenvk: Option<PathBuf>,
}

impl StagedFrameworks {
    pub fn staged_names(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        if self.metal_irconverter.is_some() {
            out.push("libmetalirconverter.dylib");
        }
        if self.dxilconv.is_some() {
            out.push("libdxilconv.dylib");
        }
        if self.moltenvk.is_some() {
            out.push("libMoltenVK.dylib");
        }
        out
    }
}

fn push_unique_path(out: &mut Vec<PathBuf>, candidate: PathBuf) {
    if !candidate.as_os_str().is_empty() && !out.contains(&candidate) {
        out.push(candidate);
    }
}

/// Ordered candidate search paths for `libmetalirconverter.dylib`.
pub fn metal_irconverter_candidates(env: &FrameworkDiscoveryEnv) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Some(ref p) = env.metal_irconverter_dylib {
        push_unique_path(&mut out, p.clone());
    }
    if let Some(ref home) = env.home {
        push_unique_path(
            &mut out,
            home.join(".local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib"),
        );
    }
    if env.include_system_paths {
        push_unique_path(
            &mut out,
            PathBuf::from("/opt/homebrew/lib/libmetalirconverter.dylib"),
        );
        push_unique_path(
            &mut out,
            PathBuf::from("/usr/local/lib/libmetalirconverter.dylib"),
        );
        push_unique_path(
            &mut out,
            PathBuf::from("/opt/metal-shaderconverter/lib/libmetalirconverter.dylib"),
        );
    }
    out
}

/// Ordered candidate search paths for optional `libdxilconv.dylib` (DX11 SM 4/5 -> DXIL).
pub fn dxilconv_candidates(workspace_root: &Path, env: &FrameworkDiscoveryEnv) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Some(ref p) = env.aw2_dxilconv {
        push_unique_path(&mut out, p.clone());
    }
    if let Some(ref home) = env.home {
        push_unique_path(&mut out, home.join(".local/lib/libdxilconv.dylib"));
    }
    push_unique_path(
        &mut out,
        workspace_root.join("third_party/DirectXShaderCompiler/build/lib/libdxilconv.dylib"),
    );
    if env.include_system_paths {
        push_unique_path(&mut out, PathBuf::from("/opt/homebrew/lib/libdxilconv.dylib"));
        push_unique_path(&mut out, PathBuf::from("/usr/local/lib/libdxilconv.dylib"));
    }
    out
}

/// Ordered candidate search paths for optional `libMoltenVK.dylib` (Vulkan -> Metal).
pub fn moltenvk_candidates(env: &FrameworkDiscoveryEnv) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Some(ref p) = env.port_vulkan_dylib {
        push_unique_path(&mut out, p.clone());
    }
    if let Some(ref p) = env.port_moltenvk_dylib {
        push_unique_path(&mut out, p.clone());
    }
    if let Some(ref p) = env.moltenvk_dylib {
        push_unique_path(&mut out, p.clone());
    }
    if env.include_system_paths {
        push_unique_path(
            &mut out,
            PathBuf::from("/opt/homebrew/opt/molten-vk/lib/libMoltenVK.dylib"),
        );
        push_unique_path(
            &mut out,
            PathBuf::from("/opt/homebrew/lib/libMoltenVK.dylib"),
        );
        push_unique_path(&mut out, PathBuf::from("/usr/local/lib/libMoltenVK.dylib"));
    }
    if let Some(ref sdk) = env.vulkan_sdk {
        push_unique_path(&mut out, sdk.join("lib/libMoltenVK.dylib"));
    }
    if let Some(ref home) = env.home {
        push_unique_path(&mut out, home.join(".local/lib/libMoltenVK.dylib"));
        push_unique_path(
            &mut out,
            home.join("Library/Android/sdk/emulator/lib64/vulkan/libMoltenVK.dylib"),
        );
    }
    out
}

/// Returns `true` if `path` starts with a Mach-O or universal/fat Mach-O magic number.
pub fn is_macho_file(path: &Path) -> bool {
    use std::io::Read;
    let Ok(mut f) = fs::File::open(path) else {
        return false;
    };
    let mut magic = [0u8; 4];
    if f.read_exact(&mut magic).is_err() {
        return false;
    }
    matches!(
        u32::from_be_bytes(magic),
        0xFEED_FACE
            | 0xCEFA_EDFE
            | 0xFEED_FACF
            | 0xCFFA_EDFE
            | 0xCAFE_BABE
            | 0xBEBA_FECA
            | 0xCAFE_BABF
            | 0xBFBA_BECA
    )
}

/// Parse `(minos, sdk)` version pairs for all `platform MACOS` slices from `vtool -show-build`.
pub fn parse_vtool_macos_versions(stdout: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut in_macos = false;
    let mut cur_minos: Option<String> = None;
    let mut cur_sdk: Option<String> = None;

    let flush = |in_macos: &mut bool,
                 cur_minos: &mut Option<String>,
                 cur_sdk: &mut Option<String>,
                 out: &mut Vec<(String, String)>| {
        if *in_macos {
            if let Some(minos) = cur_minos.take() {
                let sdk = cur_sdk.take().unwrap_or_else(|| minos.clone());
                out.push((minos, sdk));
            }
        }
        *in_macos = false;
        *cur_minos = None;
        *cur_sdk = None;
    };

    for raw_line in stdout.lines() {
        let line = raw_line.trim();
        if line.starts_with("Load command ") || line.ends_with(':') {
            flush(&mut in_macos, &mut cur_minos, &mut cur_sdk, &mut out);
            continue;
        }
        if let Some(rest) = line.strip_prefix("platform ") {
            in_macos = rest.trim().eq_ignore_ascii_case("MACOS");
        } else if in_macos {
            if let Some(rest) = line.strip_prefix("minos ") {
                cur_minos = Some(rest.trim().to_string());
            } else if let Some(rest) = line.strip_prefix("sdk ") {
                cur_sdk = Some(rest.trim().to_string());
            }
        }
    }
    flush(&mut in_macos, &mut cur_minos, &mut cur_sdk, &mut out);
    out
}

pub(super) fn version_major(ver: &str) -> Option<u32> {
    ver.trim().split('.').next()?.parse::<u32>().ok()
}

/// Lower `LC_BUILD_VERSION` macOS `minos` to `14.0` (preserving `sdk`) when a staged dylib
/// was built on a newer beta/host macOS (`minos > 15`, e.g. `27.2`), so the bundle remains
/// portable to other Apple Silicon Macs running macOS 14/15/26+. Also normalizes non-relative
/// dylib install names to `@rpath/<filename>` for clean relocation.
fn normalize_staged_macho_framework(dst: &Path) -> Result<()> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = dst;
        return Ok(());
    }
    #[cfg(target_os = "macos")]
    {
        if !is_macho_file(dst) {
            return Ok(());
        }
        if let Ok(out) = Command::new("vtool")
            .args(["-show-build", &dst.to_string_lossy()])
            .output()
        {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let versions = parse_vtool_macos_versions(&stdout);
                if let Some((_, sdk)) = versions
                    .iter()
                    .find(|(minos, _)| version_major(minos).is_some_and(|m| m > 15))
                {
                    let dst_s = dst.to_string_lossy().into_owned();
                    let status = Command::new("vtool")
                        .args([
                            "-set-build-version",
                            "macos",
                            "14.0",
                            sdk.as_str(),
                            "-replace",
                            "-output",
                            &dst_s,
                            &dst_s,
                        ])
                        .status();
                    if let Ok(st) = status {
                        if !st.success() {
                            eprintln!(
                                "warning: vtool failed to lower minos on {} (status {st})",
                                dst.display()
                            );
                        }
                    }
                }
            }
        }

        // Normalize absolute install names (e.g. non-system host library paths) to `@rpath/<filename>`.
        if let Some(fname) = dst.file_name().and_then(|s| s.to_str()) {
            let needs_id_fix = otool_install_names(dst, None).iter().any(|id| {
                id.starts_with('/')
                    && !id.starts_with("/System/")
                    && !id.starts_with("/usr/lib/")
            });
            if needs_id_fix {
                let _ = Command::new("install_name_tool")
                    .args(["-id", &format!("@rpath/{fname}"), &dst.to_string_lossy()])
                    .status();
            }
        }
        Ok(())
    }
}

fn stage_first_existing(candidates: &[PathBuf], dst: &Path) -> Result<Option<PathBuf>> {
    for src in candidates {
        if src.is_file() {
            // Remove any stale destination file or symlink first so re-staging over a read-only
            // copy or dangling symlink never fails.
            if dst.exists() || dst.is_symlink() {
                let _ = fs::remove_file(dst);
            }
            fs::copy(src, dst).with_context(|| {
                format!("copying framework {} -> {}", src.display(), dst.display())
            })?;
            // Ensure staged frameworks are owner-writable and loadable/executable (e.g. Homebrew
            // Cellar dylibs are often 0444/0555, which would block `codesign --force --sign -`).
            set_executable_mode(dst)?;
            normalize_staged_macho_framework(dst)?;
            return Ok(Some(dst.to_path_buf()));
        }
    }
    Ok(None)
}

/// Discover and copy `libmetalirconverter.dylib` and optional `libdxilconv.dylib` /
/// `libMoltenVK.dylib` into `frameworks_dir` using an explicit [`FrameworkDiscoveryEnv`].
pub fn stage_bundle_frameworks_with_env(
    frameworks_dir: &Path,
    workspace_root: &Path,
    env: &FrameworkDiscoveryEnv,
) -> Result<StagedFrameworks> {
    fs::create_dir_all(frameworks_dir)
        .with_context(|| format!("creating {}", frameworks_dir.display()))?;

    let metal_irconverter = stage_first_existing(
        &metal_irconverter_candidates(env),
        &frameworks_dir.join("libmetalirconverter.dylib"),
    )?;
    let dxilconv = stage_first_existing(
        &dxilconv_candidates(workspace_root, env),
        &frameworks_dir.join("libdxilconv.dylib"),
    )?;
    let moltenvk = stage_first_existing(
        &moltenvk_candidates(env),
        &frameworks_dir.join("libMoltenVK.dylib"),
    )?;

    Ok(StagedFrameworks {
        metal_irconverter,
        dxilconv,
        moltenvk,
    })
}

/// Discover and copy host runtime frameworks into `<App>.app/Contents/Frameworks/`.
pub fn stage_bundle_frameworks(
    frameworks_dir: &Path,
    workspace_root: &Path,
) -> Result<StagedFrameworks> {
    let env = FrameworkDiscoveryEnv::from_process_env();
    stage_bundle_frameworks_with_env(frameworks_dir, workspace_root, &env)
}

/// Parameters for generating the native ARM64 `.app` launcher (`Contents/MacOS/<exec_ident>`).
#[derive(Debug, Clone)]
pub struct BundleLauncherConfig<'a> {
    pub clean_title: &'a str,
    pub app_id: &'a str,
    pub exe_rel: &'a str,
    pub image_base: u64,
    pub size_of_image: u32,
    pub aot_fn_count: usize,
    pub aot_dylib_name: &'a str,
    pub extra_aot_dylibs: &'a [String],
}

/// Generate a valid macOS `Contents/Info.plist` for a self-contained `.app` bundle.
pub fn generate_info_plist(clean_title: &str, exec_ident: &str, build_id: &str) -> String {
    let title_xml = html_escape(clean_title);
    let exec_xml = html_escape(exec_ident);
    let bundle_id = format!(
        "com.portfactory.steam.{}",
        exec_ident.to_ascii_lowercase()
    );
    let version = if build_id.trim().is_empty() {
        "1"
    } else {
        build_id.trim()
    };
    let version_xml = html_escape(version);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>{title_xml}</string>
    <key>CFBundleExecutable</key>
    <string>{exec_xml}</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>{bundle_id}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{title_xml}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>{version_xml}</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.games</string>
    <key>LSMinimumSystemVersion</key>
    <string>14.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
"#
    )
}

/// Generate the declarative `Contents/Resources/game.toml` manifest for an `.app` bundle.
pub fn generate_game_manifest_toml(
    clean_title: &str,
    exec_ident: &str,
    exe_rel: &str,
    app_id: &str,
) -> String {
    GameManifest {
        id: Some(exec_ident.to_ascii_lowercase()),
        name: Some(clean_title.to_string()),
        exe: Some(exe_rel.to_string()),
        win_dir: Some(format!("C:\\Games\\{exec_ident}")),
        log_name: Some(format!("{exec_ident}.log")),
        steam_app_id: Some(app_id.to_string()),
        aot_dir: Some("AotCache".to_string()),
        preferred_base_retry: Some(false),
        v8_jitless: Some(false),
        ..GameManifest::default()
    }
    .to_toml()
}

/// Look up a curated declarative manifest under `<workspace_root>/games/*/game.toml` matching
/// `app_id` (when non-empty and not `"custom"`) or `exe_rel` / executable filename.
///
/// Preserves curated `[runtime]`, `[middleware.*]`, `[dlls]`, `[storage]`, and `[ios]`
/// sections, updates `exe = "..."` to `exe_rel` if the bundled relative path differs, and
/// ensures `aot_dir = "AotCache"` is set.
pub fn find_repo_game_manifest(
    workspace_root: &Path,
    app_id: &str,
    exe_rel: &str,
) -> Option<String> {
    let games_dir = workspace_root.join("games");
    let mut entries: Vec<PathBuf> = fs::read_dir(&games_dir)
        .ok()?
        .flatten()
        .filter_map(|e| {
            let ft = e.file_type().ok()?;
            if ft.is_dir() && !ft.is_symlink() {
                let m = e.path().join("game.toml");
                if m.is_file() {
                    return Some(m);
                }
            }
            None
        })
        .collect();
    entries.sort();

    let want_app_id = (!app_id.trim().is_empty() && !app_id.eq_ignore_ascii_case("custom"))
        .then_some(app_id.trim());
    let want_exe_rel = exe_rel.replace('\\', "/");
    let want_exe_name = Path::new(&want_exe_rel)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&want_exe_rel);

    let mut matched: Option<GameManifest> = None;
    for manifest_path in entries {
        let Ok(content) = fs::read_to_string(&manifest_path) else {
            continue;
        };
        let parsed = GameManifest::parse(&content);

        let by_id = want_app_id.is_some_and(|want| {
            parsed
                .steam_app_id
                .as_deref()
                .is_some_and(|got| got.eq_ignore_ascii_case(want))
        });
        let by_exe = parsed.exe.as_deref().is_some_and(|got| {
            let got_norm = got.replace('\\', "/");
            let got_name = Path::new(&got_norm)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(&got_norm);
            got_norm.eq_ignore_ascii_case(&want_exe_rel)
                || got_name.eq_ignore_ascii_case(want_exe_name)
        });

        if by_id || by_exe {
            matched = Some(parsed);
            if by_id {
                break;
            }
        }
    }

    let mut manifest = matched?;
    manifest.exe = Some(want_exe_rel);
    if manifest.aot_dir.is_none() {
        manifest.aot_dir = Some("AotCache".to_string());
    }
    Some(manifest.to_toml())
}

/// Generate the Rust source for the self-contained `.app` launcher binary.
///
/// The launcher resolves `Contents/Frameworks/`, `Contents/Resources/`, and `Contents/MacOS/`
/// relative to `std::env::current_exe()` so double-clicking the `.app` from Finder works with
/// zero developer environment variables set:
/// - Sets `PORT_MANIFEST` when `Contents/Resources/game.toml` exists.
/// - Sets `PORT_AOT_DIR` / `AW2_DLL_AOT_DIR` to `Contents/Resources/AotCache`.
/// - Sets `PORT_SHADERD` / `AW2_SHADERD` when `Contents/MacOS/dxil-metald` exists, and prepends
///   `Contents/MacOS` to `PATH`.
/// - Sets `METAL_IRCONVERTER_DYLIB` when `Contents/Frameworks/libmetalirconverter.dylib` exists.
/// - Sets `AW2_DXILCONV` when `Contents/Frameworks/libdxilconv.dylib` exists.
/// - Sets `PORT_VULKAN_DYLIB` when `Contents/Frameworks/libMoltenVK.dylib` exists.
/// - Prepends `Contents/Frameworks` to `DYLD_FALLBACK_LIBRARY_PATH`.
/// - Preserves any explicit developer override already present in the environment.
pub fn generate_bundle_launcher_source(cfg: &BundleLauncherConfig<'_>) -> String {
    BUNDLE_LAUNCHER_TEMPLATE
        .replace("__CLEAN_TITLE__", cfg.clean_title)
        .replace("__APP_ID_DEBUG__", &format!("{:?}", cfg.app_id))
        .replace("__APP_ID__", cfg.app_id)
        .replace("__EXE_REL_DEBUG__", &format!("{:?}", cfg.exe_rel))
        .replace("__EXE_REL__", cfg.exe_rel)
        .replace("__IMAGE_BASE_HEX__", &format!("{:016X}", cfg.image_base))
        .replace("__SIZE_OF_IMAGE_HEX__", &format!("{:08X}", cfg.size_of_image))
        .replace("__AOT_FN_COUNT__", &cfg.aot_fn_count.to_string())
        .replace(
            "__AOT_DYLIB_NAME_DEBUG__",
            &format!("{:?}", cfg.aot_dylib_name),
        )
        .replace("__AOT_DYLIB_NAME__", cfg.aot_dylib_name)
        .replace(
            "__EXTRA_AOT_DYLIBS_DEBUG__",
            &format!("{:?}", cfg.extra_aot_dylibs),
        )
}

/// Result of converting a Steam game or Windows application.
#[derive(Debug, Clone)]
pub struct SteamConvertResult {
    pub app_id: String,
    pub game_name: String,
    pub source_dir: PathBuf,
    pub primary_exe: PathBuf,
    pub output_path: PathBuf,
    pub format: ConversionFormat,
    pub win32_readiness_pct: f64,
    pub shaders_converted: usize,
    pub registry_keys_seeded: usize,
    pub aot_functions_compiled: usize,
    pub aot_instructions_lowered: usize,
    pub aot_dylib_path: Option<PathBuf>,
    pub standalone_executable: Option<PathBuf>,
    pub staged_frameworks: StagedFrameworks,
}

/// Resolve a target (`AppID` like `2357570`, game name like `Overwatch`, `steam`, or path)
/// to an [`InstalledGameEntry`] and convert it in one click to the requested format.
pub fn convert_steam_game(
    workspace_root: &Path,
    opts: &SteamConvertOptions,
) -> Result<SteamConvertResult> {
    let report = scan_all_steam_libraries(workspace_root)?;
    let db = Win32Database::from_workspace_src(workspace_root)?;

    let q = opts.target.trim();
    let q_lower = q.to_ascii_lowercase();

    let resolved_entry = if let Some(found) = report.installed_games.iter().find(|g| {
        g.app_id.eq_ignore_ascii_case(q)
            || g.name.to_ascii_lowercase().contains(&q_lower)
            || (q_lower == "steam" && g.app_id == "steam-client-win64")
    }) {
        found.clone()
    } else {
        let p = PathBuf::from(q);
        if !p.exists() {
            bail!(
                "target {q:?} not found in installed Steam/CrossOver/Heroic libraries and path does not exist.\nRun `port-factory steam-scan` to list installed & owned games, or `port-factory steam-download <appid>` first."
            );
        }
        let dir = if p.is_file() {
            p.parent().unwrap_or(Path::new(".")).to_path_buf()
        } else {
            p.clone()
        };
        let name = dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("ConvertedGame")
            .to_string();
        let mut entry = inspect_game_directory(
            "custom",
            &name,
            GameLibrarySource::MacOsSteam,
            &dir,
            0,
            "local",
            "Windows",
            &db,
        )?;
        if p.is_file() {
            entry.primary_exe = Some(p);
        }
        entry
    };

    let primary_exe = resolved_entry.primary_exe.clone().ok_or_else(|| {
        anyhow::anyhow!(
            "no Windows .exe found in {} (game may be a macOS-only install; use `steam-download {}` to fetch the Windows x64 depot)",
            resolved_entry.install_dir.display(),
            resolved_entry.app_id
        )
    })?;

    let mut clean_title: String = resolved_entry
        .name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .trim()
        .to_string();
    if clean_title.is_empty() {
        clean_title = "ConvertedGame".to_string();
    }
    let mut exec_ident: String = clean_title
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    if exec_ident.is_empty() {
        exec_ident = "ConvertedGame".to_string();
    }

    match opts.format {
        ConversionFormat::AotCrate => {
            let out_dir = opts.output_path.clone().unwrap_or_else(|| {
                workspace_root
                    .join("target/converted")
                    .join(format!("{}-aot", exec_ident.to_ascii_lowercase()))
            });
            let crate_name = format!("{}_aot", exec_ident.to_ascii_lowercase());
            let summary = generate_embeddable_aot_crate(
                &primary_exe,
                &out_dir,
                Some(&crate_name),
                &db,
            )?;
            let post = apply_post_install(
                &resolved_entry.install_dir,
                &resolved_entry.app_id,
                &out_dir,
            )?;
            Ok(SteamConvertResult {
                app_id: resolved_entry.app_id.clone(),
                game_name: resolved_entry.name.clone(),
                source_dir: resolved_entry.install_dir.clone(),
                primary_exe,
                output_path: out_dir,
                format: ConversionFormat::AotCrate,
                win32_readiness_pct: resolved_entry.readiness_pct(),
                shaders_converted: 0,
                registry_keys_seeded: post.registry_entries.len(),
                aot_functions_compiled: summary.function_count,
                aot_instructions_lowered: summary.lowered_instructions,
                aot_dylib_path: None,
                standalone_executable: None,
                staged_frameworks: StagedFrameworks::default(),
            })
        }
        ConversionFormat::MetalShaders => {
            let out_dir = opts.output_path.clone().unwrap_or_else(|| {
                workspace_root
                    .join("target/converted")
                    .join(format!("{}-shaders", exec_ident.to_ascii_lowercase()))
            });
            let stats = precompile_shaders_for_paths(
                std::slice::from_ref(&resolved_entry.install_dir),
                Some(out_dir.clone()),
                1024,
            )?;
            Ok(SteamConvertResult {
                app_id: resolved_entry.app_id.clone(),
                game_name: resolved_entry.name.clone(),
                source_dir: resolved_entry.install_dir.clone(),
                primary_exe,
                output_path: out_dir,
                format: ConversionFormat::MetalShaders,
                win32_readiness_pct: resolved_entry.readiness_pct(),
                shaders_converted: stats.active_wave32_converted,
                registry_keys_seeded: 0,
                aot_functions_compiled: 0,
                aot_instructions_lowered: 0,
                aot_dylib_path: None,
                standalone_executable: None,
                staged_frameworks: StagedFrameworks::default(),
            })
        }
        ConversionFormat::MacOsApp => {
            let out_app = opts.output_path.clone().unwrap_or_else(|| {
                workspace_root
                    .join("target/converted")
                    .join(format!("{clean_title}.app"))
            });
            package_macos_game_bundle(
                workspace_root,
                &resolved_entry,
                &primary_exe,
                &clean_title,
                &exec_ident,
                &out_app,
                opts.precompile_shaders,
                opts.max_aot_functions,
                opts.bundle_shader_cache_from.as_deref(),
                &db,
            )
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn package_macos_game_bundle(
    workspace_root: &Path,
    entry: &InstalledGameEntry,
    primary_exe: &Path,
    clean_title: &str,
    exec_ident: &str,
    out_app: &Path,
    precompile_shaders: bool,
    max_aot_functions: usize,
    bundle_shader_cache_from: Option<&Path>,
    db: &Win32Database,
) -> Result<SteamConvertResult> {
    let contents_dir = out_app.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    let frameworks_dir = contents_dir.join("Frameworks");
    let resources_dir = contents_dir.join("Resources");
    let shader_cache_dir = resources_dir.join("ShaderCache");
    let aot_cache_dir = resources_dir.join("AotCache");
    let aot_dir = resources_dir.join("AOT");
    let symbols_dir = resources_dir.join("symbols");

    // Clean stale Mach-O binaries, frameworks, AOT cache, and signatures before repacking so
    // updated game builds and new runtimes never inherit obsolete artifacts.
    let _ = fs::remove_dir_all(contents_dir.join("_CodeSignature"));
    let _ = fs::remove_dir_all(&macos_dir);
    let _ = fs::remove_dir_all(&frameworks_dir);
    let _ = fs::remove_dir_all(&aot_cache_dir);
    fs::create_dir_all(&macos_dir)?;
    fs::create_dir_all(&frameworks_dir)?;
    fs::create_dir_all(&resources_dir)?;
    fs::create_dir_all(&shader_cache_dir)?;
    fs::create_dir_all(&aot_cache_dir)?;
    fs::create_dir_all(&aot_dir)?;
    fs::create_dir_all(&symbols_dir)?;

    let exe_rel = primary_exe
        .strip_prefix(&entry.install_dir)
        .unwrap_or_else(|_| Path::new(primary_exe.file_name().unwrap_or_default()))
        .to_string_lossy()
        .replace('\\', "/");

    // 1. Extract AppIcon.icns from the PE .rsrc section if available
    let icon_icns = resources_dir.join("AppIcon.icns");
    let extract_icon_py = workspace_root.join("tools/extract_pe_icon.py");
    if extract_icon_py.is_file() {
        let _ = Command::new("python3")
            .arg(&extract_icon_py)
            .arg(primary_exe)
            .arg(&icon_icns)
            .status();
    }

    // 2. Write Info.plist
    let info_plist = generate_info_plist(clean_title, exec_ident, &entry.build_id);
    fs::write(contents_dir.join("Info.plist"), info_plist)?;

    // 3. Copy or build lightweight universal `port-runner` (zero game-specific shards) and `dxil-metald`
    let loader_bin = find_or_build_release_binary(workspace_root, "port-runner")?;
    let shaderd_bin = find_or_build_release_binary(workspace_root, "dxil-metald")?;
    let dst_runner = macos_dir.join("port-runner");
    let dst_shaderd = macos_dir.join("dxil-metald");
    fs::copy(&loader_bin, &dst_runner)?;
    fs::copy(&shaderd_bin, &dst_shaderd)?;

    // Write declarative per-game manifest (`Contents/Resources/game.toml`), adopting any curated
    // repo manifest under `games/<id>/game.toml` (e.g. `builtin_overrides`) when available.
    let manifest_path = resources_dir.join("game.toml");
    let manifest_toml = find_repo_game_manifest(workspace_root, &entry.app_id, &exe_rel)
        .unwrap_or_else(|| {
            generate_game_manifest_toml(clean_title, exec_ident, &exe_rel, &entry.app_id)
        });
    fs::write(&manifest_path, manifest_toml)?;

    // Copy runtime frameworks (`libmetalirconverter.dylib`, optional `libdxilconv.dylib`,
    // optional `libMoltenVK.dylib`) into `Contents/Frameworks/` so cold launch from Finder
    // works with zero developer environment variables set.
    let staged_frameworks = stage_bundle_frameworks(&frameworks_dir, workspace_root)?;

    // 4. Apply Post-Install (`steam_appid.txt` + `steam_registry.tsv`)
    let post = apply_post_install(&entry.install_dir, &entry.app_id, &resources_dir)?;

    // 5. Copy/clone all game files (EXEs, DLLs, assets) into `Contents/Resources/Game`
    let bundle_game_dir = resources_dir.join("Game");
    let same_dir = fs::canonicalize(&entry.install_dir)
        .ok()
        .zip(fs::canonicalize(&bundle_game_dir).ok())
        .is_some_and(|(a, b)| a == b);
    if !same_dir {
        let skip_self = fs::canonicalize(out_app).ok();
        let copy_report =
            copy_game_payload(&entry.install_dir, &bundle_game_dir, skip_self.as_deref())?;
        let skipped = copy_report.skipped_symlinks();
        if skipped > 0 {
            eprintln!(
                "note: skipped {skipped} unsafe symlink(s) while copying game payload (escaping={}, dangling={}, cycle={})",
                copy_report.symlinks_skipped_escaping,
                copy_report.symlinks_skipped_dangling,
                copy_report.symlinks_skipped_cycle,
            );
        }
    }
    let _ = apply_post_install(&entry.install_dir, &entry.app_id, &bundle_game_dir);

    // 6. Shader Pre-Compilation & optional portable Tier-1 cache copy into `Contents/Resources/ShaderCache`
    let mut shaders_converted = 0usize;
    if precompile_shaders {
        if let Ok(stats) = precompile_shaders_for_paths(
            std::slice::from_ref(&entry.install_dir),
            Some(shader_cache_dir.clone()),
            1024,
        ) {
            shaders_converted = stats.active_wave32_converted;
        }
    }
    if let Some(src_cache) = bundle_shader_cache_from {
        if let Ok(copied) = copy_portable_shader_cache(src_cache, &shader_cache_dir) {
            shaders_converted += copied;
        }
    }

    // 6b. Offline ARM64 `.aot` block-cache prewarming into `Contents/Resources/AotCache`
    //     (best-effort: passes `--manifest` so curated `builtin_overrides` are prewarmed too).
    let prewarm_target_pe = bundle_game_dir.join(&exe_rel);
    let prewarm_pe = if prewarm_target_pe.is_file() {
        prewarm_target_pe.as_path()
    } else {
        primary_exe
    };
    let prewarm_game_dir = if bundle_game_dir.exists() {
        bundle_game_dir.as_path()
    } else {
        entry.install_dir.as_path()
    };
    match Command::new(&loader_bin)
        .arg("--manifest")
        .arg(&manifest_path)
        .arg("--game-dir")
        .arg(prewarm_game_dir)
        .arg("--pe")
        .arg(prewarm_pe)
        .arg("--prewarm-aot")
        .arg("--aot-dir")
        .arg(&aot_cache_dir)
        .output()
    {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if !stdout.trim().is_empty() {
                print!("{stdout}");
            }
        }
        Ok(out) => {
            eprintln!(
                "warning: offline .aot prewarm failed for {clean_title} (status {}): {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        Err(e) => {
            eprintln!(
                "warning: could not launch port-runner for offline .aot prewarm of {clean_title}: {e}"
            );
        }
    }

    // 7. Optional Prototype Static Rust AOT (`--max-fns N`, skipped cleanly when `max_aot_functions == 0`,
    //    since large games use `x86-runtime`'s prewarmed `.aot` block caches above).
    let ident_lower = exec_ident.to_ascii_lowercase();
    let mut aot_functions_compiled = 0usize;
    let mut aot_instructions_lowered = 0usize;
    let mut extra_aot_dylibs: Vec<String> = Vec::new();
    let (image_base, size_of_image, aot_dylib_name, aot_dylib_path) = if max_aot_functions == 0 {
        let pe_bytes = fs::read(primary_exe)
            .with_context(|| format!("reading primary PE {}", primary_exe.display()))?;
        let pe = Pe::parse(&pe_bytes)
            .with_context(|| format!("parsing primary PE {}", primary_exe.display()))?;
        (
            pe.headers.image_base,
            pe.headers.size_of_image,
            String::new(),
            None,
        )
    } else {
        let mut aot_opts = AotOptions::default();
        aot_opts.emit_win64_abi_attr = false;
        aot_opts.max_functions = max_aot_functions;

        let aot_out = compile_pe_file_to_rust(primary_exe, db, &aot_opts)?;
        aot_functions_compiled = aot_out.functions.len();
        aot_instructions_lowered = aot_out.lowered_instructions;

        let aot_rs_path = aot_dir.join(format!("{ident_lower}_aot.rs"));
        fs::write(&aot_rs_path, &aot_out.rust_source)?;

        // Write symbol table `.syms` for crash / profiler symbolication
        let exe_file_name = primary_exe
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("game.exe");
        let mut syms_txt = format!(
            "# aw2-syms v1 image={} base=0x{:x} count={}\n",
            exe_file_name,
            aot_out.image_base,
            aot_out.functions.len()
        );
        for f in &aot_out.functions {
            use std::fmt::Write as _;
            let _ = writeln!(
                syms_txt,
                "{:08x}\t{:08x}\taot\tVerified\t{}",
                f.begin_rva,
                f.end_rva.saturating_sub(f.begin_rva),
                f.rust_ident
            );
        }
        fs::write(symbols_dir.join(format!("{exe_file_name}.syms")), syms_txt)?;

        // Also AOT-compile companion `*_loader.dll` in the game directory if present
        for pe_file in collect_pe_files(&entry.install_dir).unwrap_or_default() {
            if &pe_file == primary_exe {
                continue;
            }
            let pe_file = &pe_file;
            let fname = pe_file
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            if fname.to_ascii_lowercase().contains("loader")
                && fname.to_ascii_lowercase().ends_with(".dll")
            {
                if let Ok(dll_aot) = compile_pe_file_to_rust(pe_file, db, &aot_opts) {
                    aot_functions_compiled += dll_aot.functions.len();
                    aot_instructions_lowered += dll_aot.lowered_instructions;
                    let stem = pe_file
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("companion")
                        .to_ascii_lowercase();
                    let dll_rs = aot_dir.join(format!("{stem}_aot.rs"));
                    let _ = fs::write(&dll_rs, &dll_aot.rust_source);
                    let dll_dylib = frameworks_dir.join(format!("lib{stem}_aot.dylib"));
                    let st = Command::new("rustc")
                        .args([
                            "--edition=2021",
                            "--crate-type",
                            "cdylib",
                            "-C",
                            "opt-level=2",
                            &dll_rs.to_string_lossy(),
                            "-o",
                            &dll_dylib.to_string_lossy(),
                        ])
                        .status();
                    if st.is_ok_and(|s| s.success()) {
                        extra_aot_dylibs.push(format!("lib{stem}_aot.dylib"));
                    }
                }
            }
        }

        // Compile primary AOT dynamic library (`Contents/Frameworks/lib<ident_lower>_aot.dylib`)
        let dylib_name = format!("lib{ident_lower}_aot.dylib");
        let dylib_path = frameworks_dir.join(&dylib_name);
        let dylib_status = Command::new("rustc")
            .args([
                "--edition=2021",
                "--crate-type",
                "cdylib",
                "-C",
                "opt-level=2",
                &aot_rs_path.to_string_lossy(),
                "-o",
                &dylib_path.to_string_lossy(),
            ])
            .status()
            .context("compiling native ARM64 AOT dylib with rustc")?;
        if !dylib_status.success() {
            bail!(
                "rustc failed to compile AOT dylib {}",
                dylib_path.display()
            );
        }
        (
            aot_out.image_base,
            aot_out.size_of_image,
            dylib_name,
            Some(dylib_path),
        )
    };

    // 8. Build lightweight native ARM64 launcher executable (`Contents/MacOS/<exec_ident>`).
    let standalone_bin_path = macos_dir.join(exec_ident);
    let standalone_rs_path = aot_dir.join(format!("{ident_lower}_standalone_main.rs"));
    let launcher_cfg = BundleLauncherConfig {
        clean_title,
        app_id: &entry.app_id,
        exe_rel: &exe_rel,
        image_base,
        size_of_image,
        aot_fn_count: aot_functions_compiled,
        aot_dylib_name: &aot_dylib_name,
        extra_aot_dylibs: &extra_aot_dylibs,
    };
    let standalone_src = generate_bundle_launcher_source(&launcher_cfg);
    fs::write(&standalone_rs_path, &standalone_src)?;

    let bin_status = Command::new("rustc")
        .args([
            "--edition=2021",
            "-C",
            "opt-level=2",
            &standalone_rs_path.to_string_lossy(),
            "-o",
            &standalone_bin_path.to_string_lossy(),
        ])
        .status()
        .context("compiling standalone native ARM64 Mach-O executable with rustc")?;
    if !bin_status.success() {
        bail!(
            "rustc failed to compile standalone Mach-O executable {}",
            standalone_bin_path.display()
        );
    }

    for bin in [
        &standalone_bin_path,
        &macos_dir.join("port-runner"),
        &macos_dir.join("dxil-metald"),
    ] {
        let _ = set_executable_mode(bin);
    }

    // 9. Ad-hoc sign all Mach-O binaries in Frameworks and MacOS, then the .app bundle
    if let Ok(rd) = fs::read_dir(&frameworks_dir) {
        for f in rd.flatten() {
            let p = f.path();
            if p.is_file() {
                codesign_ad_hoc(&p);
            }
        }
    }
    for bin in [
        macos_dir.join("dxil-metald"),
        macos_dir.join("port-runner"),
        standalone_bin_path.clone(),
    ] {
        if bin.is_file() {
            codesign_ad_hoc(&bin);
        }
    }
    codesign_ad_hoc(out_app);

    Ok(SteamConvertResult {
        app_id: entry.app_id.clone(),
        game_name: entry.name.clone(),
        source_dir: entry.install_dir.clone(),
        primary_exe: primary_exe.to_path_buf(),
        output_path: out_app.to_path_buf(),
        format: ConversionFormat::MacOsApp,
        win32_readiness_pct: entry.readiness_pct(),
        shaders_converted,
        registry_keys_seeded: post.registry_entries.len(),
        aot_functions_compiled,
        aot_instructions_lowered,
        aot_dylib_path,
        standalone_executable: Some(standalone_bin_path),
        staged_frameworks,
    })
}
