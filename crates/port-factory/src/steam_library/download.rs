//! Steam Windows x64 depot downloader (`steam-setup`, `steam-download`, `steam-port`),
//! packaged `.app` inventory, external SSD discovery, and verified bundle export.

use super::discovery::{
    default_apps_dir, default_steam_downloads_dir, dir_size_bytes_shallow_fast, extract_json_field,
    extract_json_str_after_colon, known_steam_app_metadata, resolve_steam_target_app_id,
    sanitize_game_title, scan_all_steam_libraries,
};
use super::macos_bundle::{convert_steam_game, ConversionFormat, SteamConvertOptions};
use super::vdf::apply_post_install;
use super::verify::{format_bundle_verify_report, verify_app_bundle, BundleVerifyReport};
use crate::bundle_common::{codesign_verify_deep_strict, set_executable_mode};
use anyhow::{bail, Context, Result};
use port_manifest::GameManifest;
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEPOT_DOWNLOADER_FALLBACK_TAG: &str = "DepotDownloader_3.4.0";

/// Locate an installed `DepotDownloader` binary across env override, sibling `.app` directory,
/// `~/.local/opt/depotdownloader`, `~/.local/bin`, `$PATH`, and standard prefixes.
pub fn find_depot_downloader() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("PORT_DEPOT_DOWNLOADER")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
    {
        if p.is_file() {
            return Some(p);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for name in ["DepotDownloader", "depotdownloader"] {
                let cand = dir.join(name);
                if cand.is_file() {
                    return Some(cand);
                }
            }
        }
    }
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        for rel in [
            ".local/opt/depotdownloader/DepotDownloader",
            ".local/bin/DepotDownloader",
            ".local/bin/depotdownloader",
        ] {
            let cand = home.join(rel);
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    which_binary("DepotDownloader").or_else(|| which_binary("depotdownloader"))
}

/// Parse the official `SteamRE/DepotDownloader` GitHub release JSON to extract
/// `(tag_name, download_url)` for the native macOS archive matching `arch` (`"arm64"` or `"x64"`).
pub fn parse_depot_downloader_release_asset(
    release_json: &str,
    arch: &str,
) -> Option<(String, String)> {
    let tag = extract_json_field(release_json, "tag_name")
        .unwrap_or_else(|| DEPOT_DOWNLOADER_FALLBACK_TAG.to_string());
    let target_name = format!("DepotDownloader-macos-{arch}.zip");
    for chunk in release_json.split("\"browser_download_url\"").skip(1) {
        if let Some(url) = extract_json_str_after_colon(chunk) {
            if url.ends_with(&target_name) {
                return Some((tag, url));
            }
        }
    }
    None
}

/// Download and install the standalone native macOS `SteamRE/DepotDownloader` release
/// (zero `.NET` runtime dependency; native Apple Silicon `arm64` binary) into
/// `~/.local/opt/depotdownloader/DepotDownloader` (or `install_dir`) and symlink `~/.local/bin/DepotDownloader`.
pub fn install_depot_downloader(install_dir: Option<&Path>) -> Result<PathBuf> {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME environment variable is not set")?;
    let target_dir = install_dir
        .map(Path::to_path_buf)
        .unwrap_or_else(|| home.join(".local/opt/depotdownloader"));
    fs::create_dir_all(&target_dir)
        .with_context(|| format!("creating {}", target_dir.display()))?;

    let arch = if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "x64"
    };
    let fallback_url = format!(
        "https://github.com/SteamRE/DepotDownloader/releases/download/{DEPOT_DOWNLOADER_FALLBACK_TAG}/DepotDownloader-macos-{arch}.zip"
    );

    let (tag, zip_url) = Command::new("/usr/bin/curl")
        .args([
            "-fsSL",
            "-H",
            "Accept: application/vnd.github+json",
            "-H",
            "User-Agent: port-factory",
            "https://api.github.com/repos/SteamRE/DepotDownloader/releases/latest",
        ])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            let body = String::from_utf8_lossy(&o.stdout);
            parse_depot_downloader_release_asset(&body, arch)
        })
        .unwrap_or_else(|| (DEPOT_DOWNLOADER_FALLBACK_TAG.to_string(), fallback_url));

    println!("=== Installing Native macOS ({arch}) SteamRE/DepotDownloader ({tag}) ===");
    println!("  Download URL : {zip_url}");
    let tmp_zip = std::env::temp_dir().join(format!(
        "depotdownloader_{}_{}.zip",
        std::process::id(),
        arch
    ));
    let dl = Command::new("/usr/bin/curl")
        .args(["-fSL", "-o", &tmp_zip.to_string_lossy(), &zip_url])
        .status()
        .context("invoking /usr/bin/curl to download DepotDownloader")?;
    if !dl.success() {
        let _ = fs::remove_file(&tmp_zip);
        bail!("failed to download DepotDownloader from {zip_url} (exit {dl})");
    }

    let extract = Command::new("/usr/bin/unzip")
        .args([
            "-qo",
            &tmp_zip.to_string_lossy(),
            "-d",
            &target_dir.to_string_lossy(),
        ])
        .status()
        .context("invoking /usr/bin/unzip for DepotDownloader")?;
    let _ = fs::remove_file(&tmp_zip);
    if !extract.success() {
        bail!(
            "failed to extract DepotDownloader archive into {}",
            target_dir.display()
        );
    }

    let bin_path = target_dir.join("DepotDownloader");
    if !bin_path.is_file() {
        bail!(
            "extracted archive did not contain expected binary {}",
            bin_path.display()
        );
    }

    set_executable_mode(&bin_path)?;

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("/usr/bin/xattr")
            .args(["-dr", "com.apple.quarantine", &target_dir.to_string_lossy()])
            .status();
    }

    // Best-effort symlink into ~/.local/bin/DepotDownloader when using default install dir.
    if install_dir.is_none() {
        let local_bin = home.join(".local/bin");
        if fs::create_dir_all(&local_bin).is_ok() {
            let link_path = local_bin.join("DepotDownloader");
            let _ = fs::remove_file(&link_path);
            let _ = std::os::unix::fs::symlink(&bin_path, &link_path);
        }
    }

    let probe = Command::new(&bin_path)
        .arg("--version")
        .output()
        .with_context(|| format!("verifying installed {}", bin_path.display()))?;
    let ver_line = String::from_utf8_lossy(&probe.stdout).trim().to_string();
    if !probe.status.success() {
        bail!(
            "installed {} failed --version verification (status {}): {}",
            bin_path.display(),
            probe.status,
            String::from_utf8_lossy(&probe.stderr).trim()
        );
    }

    println!("  Installed    : {} ({ver_line})", bin_path.display());
    Ok(bin_path)
}

/// Options for downloading or updating a Windows 64-bit Steam game depot on macOS.
#[derive(Debug, Clone)]
pub struct SteamDownloadOptions {
    pub app_id: u32,
    pub output_dir: PathBuf,
    pub username: Option<String>,
    pub branch: String,
    pub use_qr: bool,
    pub validate: bool,
    pub dry_run: bool,
}

pub(super) fn parse_qr_cached_username(stdout: &str) -> Option<String> {
    const MARKER: &str = "login with -username ";
    let idx = stdout.find(MARKER)?;
    let rest = &stdout[idx + MARKER.len()..];
    let user = rest.split_whitespace().next()?.trim();
    (!user.is_empty()).then(|| user.to_string())
}

fn build_depot_downloader_args(opts: &SteamDownloadOptions, default_user: &str) -> Vec<String> {
    let mut args = vec![
        "-app".to_string(),
        opts.app_id.to_string(),
        "-os".to_string(),
        "windows".to_string(),
        "-osarch".to_string(),
        "64".to_string(),
        "-branch".to_string(),
        opts.branch.clone(),
        "-dir".to_string(),
        opts.output_dir.to_string_lossy().into_owned(),
    ];
    if opts.validate {
        args.push("-validate".to_string());
    }
    if opts.use_qr {
        args.push("-qr".to_string());
    } else if !default_user.eq_ignore_ascii_case("anonymous") {
        args.push("-username".to_string());
        args.push(default_user.to_string());
        args.push("-remember-password".to_string());
    }
    args
}

/// Download or verify/repair a Windows 64-bit (`-os windows -osarch 64`) Steam game by AppID
/// and run post-install preparation (`steam_appid.txt`, `steam_registry.tsv`, metadata stamp).
pub fn download_steam_game(
    workspace_root: &Path,
    opts: &SteamDownloadOptions,
) -> Result<PathBuf> {
    let scan = scan_all_steam_libraries(workspace_root)?;
    let default_user = opts
        .username
        .clone()
        .or_else(|| {
            scan.users
                .iter()
                .find(|u| u.most_recent)
                .map(|u| u.account_name.clone())
        })
        .or_else(|| scan.users.first().map(|u| u.account_name.clone()))
        .unwrap_or_else(|| "anonymous".to_string());

    let owned_title = scan
        .owned_games
        .iter()
        .find(|o| o.app_id == opts.app_id)
        .map(|o| o.title.clone());
    let (known_title, engine) = known_steam_app_metadata(opts.app_id)
        .unwrap_or(("Steam Game", "Windows x86-64 Engine"));
    let title = owned_title.unwrap_or_else(|| known_title.to_string());

    let dd_args = build_depot_downloader_args(opts, &default_user);

    println!("=== Port Factory Steam Windows-x64 Depot Downloader ===");
    println!("  AppID         : {} ({})", opts.app_id, title);
    println!("  Engine Profile: {}", engine);
    println!(
        "  Auth Mode     : {}",
        if opts.use_qr {
            "Steam Mobile App QR Code (-qr)".to_string()
        } else {
            format!("Steam Account ({default_user}, -remember-password)")
        }
    );
    println!("  Target OS/Arch: windows / x86_64 (64-bit)");
    println!("  Validate Files: {}", opts.validate);
    println!("  Destination   : {}", opts.output_dir.display());

    if opts.dry_run {
        println!("\n[Dry-Run] Planned download & post-install commands:");
        println!("  1. DepotDownloader {}", dd_args.join(" "));
        println!(
            "  2. port-factory steam-postinstall \"{}\" --appid {}",
            opts.output_dir.display(),
            opts.app_id
        );
        return Ok(opts.output_dir.clone());
    }

    fs::create_dir_all(&opts.output_dir)?;

    let depot_downloader = match find_depot_downloader() {
        Some(bin) => Some(bin),
        None => {
            if which_binary("steamcmd").is_some() {
                None
            } else {
                println!(
                    "  [Info] DepotDownloader not found; downloading official native macOS release..."
                );
                Some(install_depot_downloader(None)?)
            }
        }
    };

    if let Some(dd_bin) = depot_downloader {
        println!("  Backend       : DepotDownloader ({})", dd_bin.display());
        if opts.use_qr {
            // Stream stdout live while watching for DepotDownloader's known `-qr` race where QR
            // approval succeeds ("Success! Next time you can login with -username <acct> ...")
            // and a lingering QR poll disconnects Steam3 before manifests finish downloading.
            use std::io::Read as _;
            use std::process::Stdio;
            let mut child = Command::new(&dd_bin)
                .args(&dd_args)
                .stdout(Stdio::piped())
                .spawn()
                .with_context(|| format!("running {}", dd_bin.display()))?;
            let mut captured = String::new();
            if let Some(mut out) = child.stdout.take() {
                let mut buf = [0u8; 2048];
                let mut stdout_handle = std::io::stdout();
                loop {
                    match out.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            let _ = stdout_handle.write_all(&buf[..n]);
                            let _ = stdout_handle.flush();
                            captured.push_str(&String::from_utf8_lossy(&buf[..n]));
                        }
                        Err(_) => break,
                    }
                }
            }
            let status = child
                .wait()
                .with_context(|| format!("waiting for {}", dd_bin.display()))?;
            if !status.success() {
                if let Some(authed_user) = parse_qr_cached_username(&captured) {
                    println!(
                        "  [Info] QR login succeeded for '{authed_user}' before transient disconnect; resuming automatically with cached token..."
                    );
                    let mut retry_opts = opts.clone();
                    retry_opts.use_qr = false;
                    retry_opts.username = Some(authed_user.clone());
                    let retry_args = build_depot_downloader_args(&retry_opts, &authed_user);
                    let retry_status = Command::new(&dd_bin)
                        .args(&retry_args)
                        .status()
                        .with_context(|| format!("retrying {}", dd_bin.display()))?;
                    if !retry_status.success() {
                        bail!("DepotDownloader exited with status {retry_status}");
                    }
                } else {
                    bail!("DepotDownloader exited with status {status}");
                }
            }
        } else {
            let status = Command::new(&dd_bin)
                .args(&dd_args)
                .status()
                .with_context(|| format!("running {}", dd_bin.display()))?;
            if !status.success() {
                bail!("DepotDownloader exited with status {status}");
            }
        }
    } else if let Some(scmd_bin) = which_binary("steamcmd") {
        println!("  Backend       : SteamCMD ({})", scmd_bin.display());
        let status = Command::new(&scmd_bin)
            .args([
                "+@sSteamCmdForcePlatformType",
                "windows",
                "+force_install_dir",
                &opts.output_dir.to_string_lossy(),
                "+login",
                &default_user,
                "+app_update",
                &opts.app_id.to_string(),
                "validate",
                "+quit",
            ])
            .status()
            .with_context(|| format!("running {}", scmd_bin.display()))?;
        if !status.success() {
            bail!("steamcmd exited with status {status}");
        }
    } else {
        bail!("neither DepotDownloader nor steamcmd is available");
    }

    let _ = fs::write(
        opts.output_dir.join(".port-factory-game.tsv"),
        format!("app_id\t{}\ntitle\t{}\n", opts.app_id, title),
    );
    let post = apply_post_install(&opts.output_dir, &opts.app_id.to_string(), &opts.output_dir)?;
    println!(
        "  Post-Install  : {} registry keys extracted, {} redistributables satisfied natively",
        post.registry_entries.len(),
        post.bypassable_redistributables.len()
    );
    Ok(opts.output_dir.clone())
}

/// Options for the one-command end-to-end Steam -> SSD-portable `.app` workflow (`steam-port`).
#[derive(Debug, Clone)]
pub struct SteamPortOptions {
    pub target: String,
    pub download_dir: Option<PathBuf>,
    pub output_app: Option<PathBuf>,
    pub username: Option<String>,
    pub branch: String,
    pub use_qr: bool,
    pub validate: bool,
    pub skip_download: bool,
    pub precompile_shaders: bool,
    pub copy_to: Option<PathBuf>,
    pub dry_run: bool,
}

/// Summary of an end-to-end `steam-port` run.
#[derive(Debug, Clone)]
pub struct SteamPortResult {
    pub app_id: u32,
    pub title: String,
    pub game_dir: PathBuf,
    pub app_bundle: PathBuf,
    pub verify_report: Option<BundleVerifyReport>,
    pub exported_path: Option<PathBuf>,
}

/// End-to-end Steam game port workflow:
/// 1. Download / update / repair Windows x64 depot via `DepotDownloader` (unless `skip_download`).
/// 2. Convert into a self-contained SSD-portable `.app` (`--copy-game --max-fns 0`).
/// 3. Verify portability with [`verify_app_bundle`].
/// 4. Optionally copy the verified `.app` to an external SSD volume (`copy_to`).
pub fn port_steam_game(workspace_root: &Path, opts: &SteamPortOptions) -> Result<SteamPortResult> {
    let report = scan_all_steam_libraries(workspace_root)?;
    let (app_id, title, existing_win_dir) = resolve_steam_target_app_id(&report, &opts.target)?;
    let clean_title = sanitize_game_title(&title);

    let game_dir = opts
        .download_dir
        .clone()
        .or(existing_win_dir.clone())
        .unwrap_or_else(|| default_steam_downloads_dir().join(&clean_title));
    let app_bundle = opts
        .output_app
        .clone()
        .unwrap_or_else(|| default_apps_dir(workspace_root).join(format!("{clean_title}.app")));

    if opts.dry_run {
        println!("=== Port Factory One-Command Steam Port Plan ===");
        println!("  Game          : {title} (AppID {app_id})");
        println!("  Game Dir      : {}", game_dir.display());
        println!("  Output Bundle : {}", app_bundle.display());
        if !opts.skip_download {
            let dl_opts = SteamDownloadOptions {
                app_id,
                output_dir: game_dir.clone(),
                username: opts.username.clone(),
                branch: opts.branch.clone(),
                use_qr: opts.use_qr,
                validate: opts.validate || existing_win_dir.is_some(),
                dry_run: true,
            };
            let _ = download_steam_game(workspace_root, &dl_opts)?;
        } else {
            println!(
                "  Step 1        : skip download (use existing {})",
                game_dir.display()
            );
        }
        println!(
            "  Step 2        : steam-convert \"{}\" --format app --copy-game --max-fns 0 --out \"{}\"",
            game_dir.display(),
            app_bundle.display()
        );
        println!("  Step 3        : bundle-verify \"{}\"", app_bundle.display());
        if let Some(ref dest) = opts.copy_to {
            println!(
                "  Step 4        : bundle-export \"{}\" \"{}\"",
                app_bundle.display(),
                dest.display()
            );
        }
        return Ok(SteamPortResult {
            app_id,
            title,
            game_dir,
            app_bundle,
            verify_report: None,
            exported_path: None,
        });
    }

    if !opts.skip_download {
        let dl_opts = SteamDownloadOptions {
            app_id,
            output_dir: game_dir.clone(),
            username: opts.username.clone(),
            branch: opts.branch.clone(),
            use_qr: opts.use_qr,
            validate: opts.validate || existing_win_dir.is_some(),
            dry_run: false,
        };
        download_steam_game(workspace_root, &dl_opts)?;
    }

    let convert_target = if existing_win_dir
        .as_ref()
        .is_some_and(|d| d == &game_dir)
    {
        app_id.to_string()
    } else {
        game_dir.to_string_lossy().into_owned()
    };

    let convert_opts = SteamConvertOptions {
        target: convert_target,
        format: ConversionFormat::MacOsApp,
        output_path: Some(app_bundle.clone()),
        precompile_shaders: opts.precompile_shaders,
        max_aot_functions: 0,
        bundle_shader_cache_from: None,
    };
    convert_steam_game(workspace_root, &convert_opts)?;

    let verify = verify_app_bundle(&app_bundle)?;
    print!("{}", format_bundle_verify_report(&verify));
    if !verify.passed() {
        bail!(
            "packaged bundle {} failed portability verification ({} failure(s))",
            app_bundle.display(),
            verify.fail_count()
        );
    }

    let exported_path = if let Some(ref dest) = opts.copy_to {
        Some(export_app_bundle(&app_bundle, dest, false)?)
    } else {
        None
    };

    Ok(SteamPortResult {
        app_id,
        title,
        game_dir,
        app_bundle,
        verify_report: Some(verify),
        exported_path,
    })
}

/// Summary of a packaged macOS `.app` bundle managed by Port Factory.
#[derive(Debug, Clone)]
pub struct PackagedBundleInfo {
    pub name: String,
    pub path: PathBuf,
    pub runtime: String,
    pub game_exe: String,
    pub aot_modules: usize,
    pub has_metal_irconverter: bool,
    pub has_dxilconv: bool,
    pub has_moltenvk: bool,
    pub is_symlinked_game: bool,
    pub size_bytes: u64,
}

/// Enumerate packaged `.app` bundles in `<workspace>/out/apps/` and `~/Games/PortFactory/Apps/`.
pub fn list_packaged_bundles(workspace_root: &Path) -> Vec<PackagedBundleInfo> {
    let mut dirs = vec![default_apps_dir(workspace_root)];
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        let user_apps = home.join("Games/PortFactory/Apps");
        if user_apps.is_dir() && !dirs.contains(&user_apps) {
            dirs.push(user_apps);
        }
    }

    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for root in dirs {
        let Ok(rd) = fs::read_dir(&root) else {
            continue;
        };
        let mut entries: Vec<PathBuf> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.is_dir()
                    && p.extension()
                        .and_then(|s| s.to_str())
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("app"))
            })
            .collect();
        entries.sort();

        for app in entries {
            let canon = fs::canonicalize(&app).unwrap_or_else(|_| app.clone());
            if !seen.insert(canon) {
                continue;
            }
            let contents = app.join("Contents");
            let macos = contents.join("MacOS");
            let fw = contents.join("Frameworks");
            let res = contents.join("Resources");

            let runtime = if macos.join("port-runner").is_file() {
                "port-runner".to_string()
            } else if macos.join("aw2-loader").is_file() {
                "aw2-loader".to_string()
            } else if macos.join("port-factory").is_file() {
                // Skip the Port Factory launcher bundle itself when listing ported game apps.
                continue;
            } else {
                "unknown".to_string()
            };

            let game_exe = fs::read_to_string(res.join("game.toml"))
                .ok()
                .and_then(|s| GameManifest::parse(&s).exe)
                .unwrap_or_default();

            let aot_modules = fs::read_dir(res.join("AotCache"))
                .ok()
                .map(|r| {
                    r.flatten()
                        .filter(|e| {
                            e.path()
                                .extension()
                                .and_then(|s| s.to_str())
                                == Some("aot")
                        })
                        .count()
                })
                .unwrap_or(0);

            let is_symlinked_game = fs::symlink_metadata(res.join("Game"))
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);

            let name = app
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("App")
                .to_string();

            out.push(PackagedBundleInfo {
                name,
                size_bytes: dir_size_bytes_shallow_fast(&app),
                path: app,
                runtime,
                game_exe,
                aot_modules,
                has_metal_irconverter: fw.join("libmetalirconverter.dylib").is_file(),
                has_dxilconv: fw.join("libdxilconv.dylib").is_file(),
                has_moltenvk: fw.join("libMoltenVK.dylib").is_file(),
                is_symlinked_game,
            });
        }
    }
    out
}

/// Mounted external volume under `/Volumes/` suitable as an SSD copy destination.
#[derive(Debug, Clone)]
pub struct ExternalVolumeInfo {
    pub name: String,
    pub path: PathBuf,
    pub free_bytes: u64,
    pub total_bytes: u64,
}

/// List non-system mounted volumes under `/Volumes/` (external SSDs, USB drives, disk images).
pub fn list_external_volumes() -> Vec<ExternalVolumeInfo> {
    let mut out = Vec::new();
    let volumes_root = Path::new("/Volumes");
    let Ok(rd) = fs::read_dir(volumes_root) else {
        return out;
    };
    let mut entries: Vec<PathBuf> = rd
        .flatten()
        .filter_map(|e| {
            let ft = e.file_type().ok()?;
            if ft.is_symlink() || !ft.is_dir() {
                return None;
            }
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with('.')
                || matches!(
                    name.as_str(),
                    "Recovery" | "Preboot" | "VM" | "Update" | "Macintosh HD"
                )
            {
                return None;
            }
            Some(e.path())
        })
        .collect();
    entries.sort();

    for vol in entries {
        let name = vol
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Volume")
            .to_string();
        let (total_bytes, free_bytes) = df_volume_bytes(&vol).unwrap_or((0, 0));
        out.push(ExternalVolumeInfo {
            name,
            path: vol,
            free_bytes,
            total_bytes,
        });
    }
    out
}

fn df_volume_bytes(path: &Path) -> Option<(u64, u64)> {
    let out = Command::new("/bin/df")
        .args(["-k", &path.to_string_lossy()])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text.lines().nth(1)?;
    let cols: Vec<&str> = line.split_whitespace().collect();
    if cols.len() < 4 {
        return None;
    }
    let total_kb = cols[1].parse::<u64>().ok()?;
    let avail_kb = cols[3].parse::<u64>().ok()?;
    Some((total_kb.saturating_mul(1024), avail_kb.saturating_mul(1024)))
}

/// Verify and copy a self-contained `.app` bundle to an external SSD or target folder.
/// Refuses to export a non-portable bundle unless `force == true`.
pub fn export_app_bundle(app_path: &Path, dest: &Path, force: bool) -> Result<PathBuf> {
    if !app_path.is_dir() {
        bail!("source bundle does not exist: {}", app_path.display());
    }
    let report = verify_app_bundle(app_path)?;
    if !report.passed() && !force {
        bail!(
            "refusing to export non-portable bundle {} (pass --force to override):\n{}",
            app_path.display(),
            format_bundle_verify_report(&report)
        );
    }

    let app_name = app_path
        .file_name()
        .context("source bundle has no file name")?;
    let dest_app = if dest
        .extension()
        .and_then(|s| s.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("app"))
    {
        dest.to_path_buf()
    } else {
        fs::create_dir_all(dest)
            .with_context(|| format!("creating export destination {}", dest.display()))?;
        dest.join(app_name)
    };

    if let Some(parent) = dest_app.parent() {
        fs::create_dir_all(parent)?;
    }

    println!(
        "Exporting {} -> {} ...",
        app_path.display(),
        dest_app.display()
    );
    let rsync_status = Command::new("/usr/bin/rsync")
        .env("COPYFILE_DISABLE", "1")
        .args([
            "-aW",
            "--delete",
            &format!("{}/", app_path.display()),
            &format!("{}/", dest_app.display()),
        ])
        .status();

    let copied = match rsync_status {
        Ok(st) if st.success() => true,
        _ => {
            let _ = fs::remove_dir_all(&dest_app);
            Command::new("/bin/cp")
                .env("COPYFILE_DISABLE", "1")
                .args([
                    "-R",
                    &app_path.to_string_lossy(),
                    &dest_app.to_string_lossy(),
                ])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
    };
    if !copied {
        bail!(
            "failed to copy {} to {}",
            app_path.display(),
            dest_app.display()
        );
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("/usr/bin/xattr")
            .args(["-dr", "com.apple.quarantine", &dest_app.to_string_lossy()])
            .status();
        if let Ok(o) = codesign_verify_deep_strict(&dest_app) {
            if !o.status.success() {
                eprintln!(
                    "warning: codesign verify on exported bundle {}: {}",
                    dest_app.display(),
                    String::from_utf8_lossy(&o.stderr).trim()
                );
            }
        }
    }

    println!("Exported verified bundle to {}", dest_app.display());
    Ok(dest_app)
}

fn which_binary(name: &str) -> Option<PathBuf> {
    if let Some(path_env) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_env) {
            let p = dir.join(name);
            if p.is_file() {
                return Some(p);
            }
        }
    }
    for prefix in ["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin"] {
        let p = Path::new(prefix).join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}
