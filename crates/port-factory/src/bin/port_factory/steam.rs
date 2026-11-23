//! Steam library, DepotDownloader, local launcher UI, and one-click Steam conversion handlers.

use super::{workspace_root, ArgCursor, DEFAULT_STEAM_BROWSER_HTML};
use anyhow::{bail, Context, Result};
use port_factory::launcher::{build_launcher_app_bundle, run_launcher, LauncherOptions};
use port_factory::steam_library::{
    apply_post_install, convert_steam_game, default_apps_dir, default_steam_downloads_dir,
    download_steam_game, find_depot_downloader, format_steam_scan_report, install_depot_downloader,
    port_steam_game, resolve_steam_target_app_id, sanitize_game_title, scan_all_steam_libraries,
    write_steam_browser_html, ConversionFormat, SteamConvertOptions, SteamDownloadOptions,
    SteamPortOptions,
};
use std::path::PathBuf;

pub fn cmd_steam_scan(args: &[String]) -> Result<()> {
    let mut html_out: Option<PathBuf> = None;
    let mut open_browser = false;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--html" | "-o" => {
                if let Some(p) = cur.next() {
                    html_out = Some(PathBuf::from(p));
                }
            }
            "--open" => open_browser = true,
            _ => {}
        }
    }

    let root = workspace_root();
    let report = scan_all_steam_libraries(&root)?;
    print!("{}", format_steam_scan_report(&report));

    let html_path = html_out.unwrap_or_else(|| root.join(DEFAULT_STEAM_BROWSER_HTML));
    write_steam_browser_html(&report, &html_path)?;
    println!(
        "\nInteractive Steam Library Browser generated at:\n  {}",
        html_path.display()
    );

    if open_browser {
        let _ = std::process::Command::new("open").arg(&html_path).status();
    }
    Ok(())
}

pub fn cmd_launcher(args: &[String]) -> Result<()> {
    let mut port: u16 = 7420;
    let mut open_browser = true;
    let mut ws = workspace_root();

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--port" | "-p" => {
                let raw = cur.next_or_err("--port requires PORT")?;
                port = raw
                    .parse()
                    .with_context(|| format!("invalid --port {raw:?}"))?;
            }
            "--workspace" | "-w" => {
                ws = PathBuf::from(cur.next_or_err("--workspace requires DIR")?);
            }
            "--no-open" => open_browser = false,
            other => bail!("unknown launcher flag {other:?}"),
        }
    }

    run_launcher(LauncherOptions {
        workspace_root: ws,
        port,
        open_browser,
    })
}

pub fn cmd_launcher_app(args: &[String]) -> Result<()> {
    let root = workspace_root();
    let mut out_app = default_apps_dir(&root).join("Port Factory.app");
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" => {
                out_app = PathBuf::from(cur.next_or_err("--out requires PATH.app")?);
            }
            other => bail!("unknown launcher-app flag {other:?}"),
        }
    }
    let built = build_launcher_app_bundle(&root, &out_app)?;
    println!("Built interactive launcher app bundle: {}", built.display());
    println!(
        "Double-click it in Finder or run: open \"{}\"",
        built.display()
    );
    Ok(())
}

pub fn cmd_steam_setup(args: &[String]) -> Result<()> {
    let mut dir: Option<PathBuf> = None;
    let mut force = false;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--dir" | "-o" => {
                dir = Some(PathBuf::from(cur.next_or_err("--dir requires DIR")?));
            }
            "--force" => force = true,
            other => bail!("unknown steam-setup flag {other:?}"),
        }
    }
    if !force {
        if let Some(existing) = find_depot_downloader() {
            println!("DepotDownloader is already installed: {}", existing.display());
            println!("(Pass --force to reinstall the latest native macOS release.)");
            return Ok(());
        }
    }
    let bin = install_depot_downloader(dir.as_deref())?;
    println!(
        "Ready to download Windows x64 Steam games with: {}",
        bin.display()
    );
    Ok(())
}

pub fn cmd_steam_download(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!(
            "steam-download requires <APPID_OR_NAME> [--dir DIR] [--qr | --user ACCOUNT] [--validate] [--branch BRANCH] [--dry-run]"
        );
    }
    let mut target: Option<String> = None;
    let mut output_dir: Option<PathBuf> = None;
    let mut username = std::env::var("STEAM_USER").ok();
    let mut branch = "public".to_string();
    let mut use_qr = false;
    let mut validate = false;
    let mut dry_run = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--dir" | "-o" => {
                output_dir = Some(PathBuf::from(cur.next_or_err("--dir requires DIR")?));
            }
            "--user" | "-u" => {
                username = Some(cur.next_or_err("--user requires ACCOUNT")?.to_string());
                use_qr = false;
            }
            "--qr" => use_qr = true,
            "--validate" | "--repair" => validate = true,
            "--branch" | "-b" => {
                branch = cur.next_or_err("--branch requires BRANCH")?.to_string();
            }
            "--dry-run" => dry_run = true,
            other if !other.starts_with('-') && target.is_none() => {
                target = Some(other.to_string());
            }
            other => bail!("unknown steam-download flag '{}'", other),
        }
    }

    let target = target.context("missing Steam <APPID_OR_NAME>")?;
    let root = workspace_root();
    let scan = scan_all_steam_libraries(&root)?;
    let (app_id, title, existing_win_dir) = resolve_steam_target_app_id(&scan, &target)?;
    let clean_title = sanitize_game_title(&title);

    let output_dir = output_dir
        .or_else(|| if validate { existing_win_dir.clone() } else { None })
        .unwrap_or_else(|| default_steam_downloads_dir().join(&clean_title));

    let opts = SteamDownloadOptions {
        app_id,
        output_dir,
        username,
        branch,
        use_qr,
        validate,
        dry_run,
    };

    let installed_dir = download_steam_game(&root, &opts)?;
    println!("  Download Dir  : {}", installed_dir.display());
    Ok(())
}

pub fn cmd_steam_port(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!(
            "steam-port requires <APPID_OR_NAME> [--qr | --user ACCOUNT] [--validate] [--skip-download] [--dir DIR] [--out PATH.app] [--copy-to /Volumes/SSD] [--dry-run]"
        );
    }
    let mut target: Option<String> = None;
    let mut download_dir: Option<PathBuf> = None;
    let mut output_app: Option<PathBuf> = None;
    let mut username = std::env::var("STEAM_USER").ok();
    let mut branch = "public".to_string();
    let mut use_qr = false;
    let mut validate = false;
    let mut skip_download = false;
    let mut precompile_shaders = true;
    let mut copy_to: Option<PathBuf> = None;
    let mut dry_run = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--dir" => {
                download_dir = Some(PathBuf::from(cur.next_or_err("--dir requires DIR")?));
            }
            "--out" | "-o" => {
                output_app = Some(PathBuf::from(cur.next_or_err("--out requires PATH.app")?));
            }
            "--copy-to" => {
                copy_to = Some(PathBuf::from(cur.next_or_err("--copy-to requires DEST")?));
            }
            "--user" | "-u" => {
                username = Some(cur.next_or_err("--user requires ACCOUNT")?.to_string());
                use_qr = false;
            }
            "--branch" | "-b" => {
                branch = cur.next_or_err("--branch requires BRANCH")?.to_string();
            }
            "--qr" => use_qr = true,
            "--validate" | "--repair" => validate = true,
            "--skip-download" => skip_download = true,
            "--no-shaders" => precompile_shaders = false,
            "--dry-run" => dry_run = true,
            other if !other.starts_with('-') && target.is_none() => {
                target = Some(other.to_string());
            }
            other => bail!("unknown steam-port flag {other:?}"),
        }
    }

    let target = target.context("missing <APPID_OR_NAME>")?;
    let root = workspace_root();
    let res = port_steam_game(
        &root,
        &SteamPortOptions {
            target,
            download_dir,
            output_app,
            username,
            branch,
            use_qr,
            validate,
            skip_download,
            precompile_shaders,
            copy_to,
            dry_run,
        },
    )?;

    if !dry_run {
        println!("\nPort complete: {} (AppID {})", res.title, res.app_id);
        println!("  App Bundle : {}", res.app_bundle.display());
        if let Some(ref exp) = res.exported_path {
            println!("  Exported   : {}", exp.display());
        }
    }
    Ok(())
}

pub fn cmd_steam_postinstall(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("steam-postinstall requires <GAME_DIR> [--appid ID] [--out DIR]");
    }
    let mut game_dir: Option<PathBuf> = None;
    let mut app_id = "0".to_string();
    let mut out_dir: Option<PathBuf> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--appid" => app_id = cur.next_or_err("--appid requires ID")?.to_string(),
            "--out" | "-o" => {
                out_dir = Some(PathBuf::from(cur.next_or_err("--out requires DIR")?));
            }
            other if !other.starts_with('-') && game_dir.is_none() => {
                game_dir = Some(PathBuf::from(other));
            }
            other => bail!("unknown steam-postinstall argument '{}'", other),
        }
    }

    let game_dir = game_dir.context("missing <GAME_DIR>")?;
    let out_dir = out_dir.unwrap_or_else(|| game_dir.clone());
    let summary = apply_post_install(&game_dir, &app_id, &out_dir)?;

    println!("================================================================================");
    println!(
        "Steam Post-Download & Post-Install Provisioning: {}",
        game_dir.display()
    );
    println!("================================================================================");
    println!(
        "  installscript.vdf files : {}",
        summary.installscript_files.len()
    );
    for vdf in &summary.installscript_files {
        println!("    - {}", vdf.display());
    }
    println!(
        "  Registry entries seeded : {}",
        summary.registry_entries.len()
    );
    for reg in summary.registry_entries.iter().take(15) {
        println!(
            "    - [{}] {} = {} ({})",
            reg.key_path, reg.value_name, reg.value_data, reg.value_type
        );
    }
    println!(
        "  Run Process entries     : {}",
        summary.run_processes.len()
    );
    for proc_entry in &summary.run_processes {
        println!("    - {}", proc_entry);
    }
    println!(
        "  Redistributables handled: {}",
        summary.bypassable_redistributables.len()
    );
    for r in &summary.bypassable_redistributables {
        println!("    - {}", r);
    }
    Ok(())
}

pub fn cmd_steam_convert(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!(
            "steam-convert requires <APPID_OR_NAME_OR_DIR> [--format app|aot-crate|shaders] [--out PATH]"
        );
    }
    let mut target: Option<String> = None;
    let mut format = ConversionFormat::MacOsApp;
    let mut output_path: Option<PathBuf> = None;
    let mut precompile_shaders = true;
    let mut max_aot_functions = 2000usize;
    let mut bundle_shader_cache_from: Option<PathBuf> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--format" | "-f" => {
                format = ConversionFormat::parse(cur.next_or_err("--format requires FORMAT")?)?;
            }
            "--out" | "-o" => {
                output_path = Some(PathBuf::from(cur.next_or_err("--out requires PATH")?));
            }
            "--max-fns" => {
                let raw = cur.next_or_err("--max-fns requires N")?;
                max_aot_functions = raw
                    .parse()
                    .with_context(|| format!("invalid --max-fns value {raw:?}"))?;
            }
            "--copy-game" => {
                // Self-contained copy into `Contents/Resources/Game` is always enabled.
            }
            "--no-shaders" => precompile_shaders = false,
            "--bundle-shader-cache" => {
                if let Some(next) = cur.next_opt_value() {
                    bundle_shader_cache_from = Some(PathBuf::from(next));
                } else {
                    let default_src = std::env::var_os("PORT_SHADER_CACHE")
                        .or_else(|| std::env::var_os("AW2_SHADER_CACHE"))
                        .map(PathBuf::from)
                        .or_else(|| {
                            std::env::var_os("HOME")
                                .map(|h| PathBuf::from(h).join("Translator/ShaderCache"))
                        })
                        .context("cannot resolve default shader cache directory; pass a path after --bundle-shader-cache")?;
                    bundle_shader_cache_from = Some(default_src);
                }
            }
            other if !other.starts_with('-') && target.is_none() => {
                target = Some(other.to_string());
            }
            other => bail!("unknown steam-convert flag '{}'", other),
        }
    }

    let target = target.context("missing <APPID_OR_NAME_OR_DIR>")?;
    let opts = SteamConvertOptions {
        target,
        format,
        output_path,
        precompile_shaders,
        max_aot_functions,
        bundle_shader_cache_from,
    };

    let root = workspace_root();
    let res = convert_steam_game(&root, &opts)?;
    println!("================================================================================");
    println!(
        "One-Click Steam Game & Client Converter: {} (AppID {})",
        res.game_name, res.app_id
    );
    println!("================================================================================");
    println!("  Format             : {:?}", res.format);
    println!("  Source Directory   : {}", res.source_dir.display());
    println!("  Primary Executable : {}", res.primary_exe.display());
    println!("  Output Artifact    : {}", res.output_path.display());
    if let Some(ref exe) = res.standalone_executable {
        println!("  Standalone Binary  : {}", exe.display());
    }
    if let Some(ref dylib) = res.aot_dylib_path {
        println!("  AOT Dynamic Library: {}", dylib.display());
    }
    if res.format == ConversionFormat::MacOsApp {
        let fw = res.staged_frameworks.staged_names();
        println!(
            "  Bundled Frameworks : {}",
            if fw.is_empty() {
                "none (host fallback)".to_string()
            } else {
                fw.join(", ")
            }
        );
    }
    println!(
        "  AOT Compilation    : {} function(s) compiled ({} instructions lowered to native ARM64)",
        res.aot_functions_compiled, res.aot_instructions_lowered
    );
    println!("  Win32 API Coverage : {:.2}%", res.win32_readiness_pct);
    println!(
        "  Post-Install Setup : {} registry key(s) provisioned in steam_registry.tsv",
        res.registry_keys_seeded
    );
    println!(
        "  Shader Pipeline    : {} Metal shader(s) pre-compiled",
        res.shaders_converted
    );
    Ok(())
}
