//! AOT prewarm / static Mach-O emission, iOS/macOS bundle packaging & verification,
//! external SSD export, and storage asset optimization CLI handlers.

use super::{usage, workspace_root, ArgCursor};
use anyhow::{bail, Context, Result};
use port_factory::bundle_common::find_or_build_host_port_runner;
use port_factory::ios_bundle::{
    build_ios_bundle, format_ios_bundle_verify_report, verify_ios_bundle, IosBundleOptions,
    IosTarget,
};
use port_factory::steam_library::{
    export_app_bundle, format_bundle_verify_report, list_external_volumes, list_packaged_bundles,
    verify_app_bundle,
};
use port_factory::storage_opt::{
    format_storage_opt_report, optimize_game_assets, parse_manifest_storage_config,
    StorageOptOptions, StorageOptProfile,
};
use port_factory::vfs::WinFs;
use std::path::{Path, PathBuf};

pub fn cmd_aot_prewarm(args: &[String]) -> Result<()> {
    let mut pe_path: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut game_dir: Option<PathBuf> = None;
    let mut aot_dir: Option<PathBuf> = None;
    let mut max_funcs: Option<usize> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--manifest" => manifest_path = Some(cur.next_path_or_usage()),
            "--game-dir" => game_dir = Some(cur.next_path_or_usage()),
            "--aot-dir" => aot_dir = Some(cur.next_path_or_usage()),
            "--max-funcs" | "--aot-max-funcs" => {
                let raw = cur.next_or_usage();
                max_funcs = Some(
                    raw.parse()
                        .with_context(|| format!("invalid --max-funcs {raw:?}"))?,
                );
            }
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if pe_path.is_none() => pe_path = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
    }
    if pe_path.is_none() && manifest_path.is_none() {
        usage();
    }
    // Allow passing a package/game directory directly to `aot-prewarm <DIR>`:
    // discover the primary x86-64 PE (`.exe` preferred, else first plugin PE) and set `--game-dir`.
    if let Some(ref pe) = pe_path.clone() {
        if pe.is_dir() {
            if game_dir.is_none() {
                game_dir = Some(pe.clone());
            }
            if manifest_path.is_some() {
                pe_path = None;
            } else {
                let discovered = port_factory::scanner::collect_pe_files(pe)?;
                let primary = discovered
                    .iter()
                    .find(|p| {
                        p.extension()
                            .and_then(|e| e.to_str())
                            .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
                    })
                    .or_else(|| discovered.first())
                    .cloned()
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "no valid x86-64 PE32+ binaries (.exe/.dll/.pyd/.node/.ocx) found in {}",
                            pe.display()
                        )
                    })?;
                pe_path = Some(primary);
            }
        } else if !pe.is_file() {
            bail!("PE file or directory not found: {}", pe.display());
        } else if !port_factory::scanner::is_valid_x64_pe_file(pe) {
            bail!("not a valid x86-64 PE32+ binary: {}", pe.display());
        }
    }
    if let Some(ref mp) = manifest_path {
        if !mp.is_file() {
            bail!("manifest file not found: {}", mp.display());
        }
    }

    let runner = find_or_build_host_port_runner(&workspace_root())?;
    let mut cmd = std::process::Command::new(&runner);
    if let Some(ref mp) = manifest_path {
        cmd.arg("--manifest").arg(mp);
    }
    if let Some(ref gd) = game_dir {
        cmd.arg("--game-dir").arg(gd);
    }
    if let Some(ref pe) = pe_path {
        cmd.arg("--pe").arg(pe);
    }
    cmd.arg("--prewarm-aot");
    if let Some(ref dir) = aot_dir {
        cmd.arg("--aot-dir").arg(dir);
    }
    if let Some(n) = max_funcs {
        cmd.arg("--aot-max-funcs").arg(n.to_string());
    }
    let out = cmd
        .output()
        .with_context(|| format!("invoking {} --prewarm-aot", runner.display()))?;
    if !out.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }
    if !out.status.success() {
        if !out.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        }
        bail!("port-runner --prewarm-aot exited with status {}", out.status);
    }
    Ok(())
}

pub fn cmd_aot_static_obj(args: &[String]) -> Result<()> {
    let mut input: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut platform = "ios".to_string();
    let mut game_dir: Option<PathBuf> = None;
    let mut max_funcs: Option<usize> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--manifest" => manifest_path = Some(cur.next_path_or_usage()),
            "--out" | "-o" => out_path = Some(cur.next_path_or_usage()),
            "--platform" | "--static-platform" => platform = cur.next_or_usage().to_string(),
            "--game-dir" => game_dir = Some(cur.next_path_or_usage()),
            "--max-funcs" | "--aot-max-funcs" => {
                let raw = cur.next_or_usage();
                max_funcs = Some(
                    raw.parse()
                        .with_context(|| format!("invalid --max-funcs {raw:?}"))?,
                );
            }
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if input.is_none() => input = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
    }
    if input.is_none() && manifest_path.is_none() {
        usage();
    }
    if let Some(ref inp) = input {
        if !inp.is_file() {
            bail!("input file not found: {}", inp.display());
        }
    }
    if let Some(ref mp) = manifest_path {
        if !mp.is_file() {
            bail!("manifest file not found: {}", mp.display());
        }
    }
    let is_aot = input
        .as_ref()
        .and_then(|p| p.extension())
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("aot"));
    let out_target = out_path.unwrap_or_else(|| {
        if is_aot {
            input.as_ref().unwrap().with_extension("aot.o")
        } else {
            PathBuf::from("out/static-aot")
        }
    });

    let runner = find_or_build_host_port_runner(&workspace_root())?;
    let mut cmd = std::process::Command::new(&runner);
    if is_aot {
        cmd.arg("--aot-to-o")
            .arg(input.as_ref().unwrap())
            .arg("--out")
            .arg(&out_target)
            .arg("--static-platform")
            .arg(&platform);
    } else {
        if let Some(ref mp) = manifest_path {
            cmd.arg("--manifest").arg(mp);
        }
        if let Some(ref gd) = game_dir {
            cmd.arg("--game-dir").arg(gd);
        }
        if let Some(ref inp) = input {
            cmd.arg("--pe").arg(inp);
        }
        cmd.arg("--emit-static-o")
            .arg(&out_target)
            .arg("--static-platform")
            .arg(&platform);
        if let Some(n) = max_funcs {
            cmd.arg("--aot-max-funcs").arg(n.to_string());
        }
    }
    let out = cmd
        .output()
        .with_context(|| format!("invoking {} for aot-static-obj", runner.display()))?;
    if !out.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }
    if !out.status.success() {
        if !out.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        }
        bail!(
            "port-runner static Mach-O emission exited with status {}",
            out.status
        );
    }
    Ok(())
}

pub fn cmd_ios_bundle(args: &[String]) -> Result<()> {
    let mut source: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut target = IosTarget::Device;
    let mut title_override: Option<String> = None;
    let mut bundle_id: Option<String> = None;
    let mut exe_override: Option<String> = None;
    let mut manifest_override: Option<PathBuf> = None;
    let mut release = false;
    let mut deployment_target = "17.0".to_string();
    let mut emit_static_aot = true;
    let mut aot_max_funcs = 0usize;
    let mut extra_aot_objs: Vec<PathBuf> = Vec::new();
    let mut precompile_shaders = true;
    let mut bundle_shader_cache_from: Option<PathBuf> = None;
    let mut codesign_identity: Option<String> = None;
    let mut provisioning_profile: Option<PathBuf> = None;
    let mut emit_xcodegen_spec = false;
    let mut storage_opt = StorageOptOptions::from_profile(StorageOptProfile::None);

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" => out_path = Some(cur.next_path_or_usage()),
            "--target" | "--platform" => {
                let raw = cur.next_or_usage();
                target = IosTarget::parse(raw).ok_or_else(|| {
                    anyhow::anyhow!("invalid --target {raw:?} (expected device or sim)")
                })?;
            }
            "--title" => title_override = Some(cur.next_or_usage().to_string()),
            "--bundle-id" => bundle_id = Some(cur.next_or_usage().to_string()),
            "--exe" => exe_override = Some(cur.next_or_usage().to_string()),
            "--manifest" => manifest_override = Some(cur.next_path_or_usage()),
            "--release" => release = true,
            "--deployment-target" => deployment_target = cur.next_or_usage().to_string(),
            "--no-static-aot" => emit_static_aot = false,
            "--aot-max-funcs" | "--max-funcs" => {
                let raw = cur.next_or_usage();
                aot_max_funcs = raw
                    .parse()
                    .with_context(|| format!("invalid --aot-max-funcs {raw:?}"))?;
            }
            "--extra-aot-obj" => extra_aot_objs.push(cur.next_path_or_usage()),
            "--no-precompile-shaders" => precompile_shaders = false,
            "--bundle-shader-cache" => {
                bundle_shader_cache_from = Some(
                    cur.next_opt_value()
                        .map(PathBuf::from)
                        .unwrap_or_else(WinFs::default_shader_cache_dir),
                );
            }
            "--optimize-storage" | "--storage-profile" => {
                let raw = cur.next_or_usage();
                let prof = StorageOptProfile::parse(raw).ok_or_else(|| {
                    anyhow::anyhow!(
                        "invalid --optimize-storage {raw:?} (expected none, lossless, high, or compact)"
                    )
                })?;
                storage_opt = StorageOptOptions::from_profile(prof);
            }
            "--hevc-videos" => {
                storage_opt.hevc_videos = true;
                storage_opt.repack_archives = true;
            }
            "--no-hevc-videos" => storage_opt.hevc_videos = false,
            "--video-max-height" => {
                let raw = cur.next_or_usage();
                storage_opt.video_max_height = raw
                    .parse()
                    .with_context(|| format!("invalid --video-max-height {raw:?}"))?;
            }
            "--video-bitrate" => {
                let raw = cur.next_or_usage();
                storage_opt.video_bitrate_kbps = raw
                    .parse()
                    .with_context(|| format!("invalid --video-bitrate {raw:?}"))?;
            }
            "--max-tex-dim" => {
                let raw = cur.next_or_usage();
                storage_opt.max_tex_dim = raw
                    .parse()
                    .with_context(|| format!("invalid --max-tex-dim {raw:?}"))?;
                if storage_opt.max_tex_dim > 0 {
                    storage_opt.repack_archives = true;
                }
            }
            "--strip-unused-dlls" => storage_opt.strip_unused_dlls = true,
            "--keep-lang" => {
                storage_opt.keep_lang = Some(cur.next_or_usage().to_string());
                storage_opt.repack_archives = true;
            }
            "--strip-dlc" => {
                storage_opt.strip_dlc = true;
                storage_opt.repack_archives = true;
            }
            "--opt-jobs" => {
                let raw = cur.next_or_usage();
                storage_opt.jobs = raw
                    .parse()
                    .with_context(|| format!("invalid --opt-jobs {raw:?}"))?;
            }
            "--opt-dry-run" => storage_opt.dry_run = true,
            "--sign" | "--codesign" => codesign_identity = Some(cur.next_or_usage().to_string()),
            "--provisioning-profile" => provisioning_profile = Some(cur.next_path_or_usage()),
            "--xcode-project" | "--xcodegen" => emit_xcodegen_spec = true,
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if source.is_none() => source = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
    }

    let Some(source) = source else {
        usage();
    };
    let out_path = out_path.unwrap_or_else(|| {
        let stem = source
            .file_stem()
            .and_then(|s| s.to_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("PortHost");
        let ext = if target == IosTarget::Device {
            "ipa"
        } else {
            "app"
        };
        PathBuf::from(format!("out/ios/{stem}.{ext}"))
    });

    let mut opts = IosBundleOptions::new(workspace_root(), source, out_path);
    opts.target = target;
    opts.title_override = title_override;
    opts.bundle_id = bundle_id;
    opts.exe_override = exe_override;
    opts.manifest_override = manifest_override;
    opts.release = release;
    opts.deployment_target = deployment_target;
    opts.emit_static_aot = emit_static_aot;
    opts.aot_max_funcs = aot_max_funcs;
    opts.extra_aot_objs = extra_aot_objs;
    opts.precompile_shaders = precompile_shaders;
    opts.bundle_shader_cache_from = bundle_shader_cache_from;
    opts.codesign_identity = codesign_identity;
    opts.provisioning_profile = provisioning_profile;
    opts.emit_xcodegen_spec = emit_xcodegen_spec;
    if storage_opt.is_enabled() {
        opts.storage_opt = Some(storage_opt.clone());
    }

    let rep = build_ios_bundle(&opts)?;
    println!("=== iOS Bundle Built ===");
    println!("  Title              : {}", rep.title);
    println!("  Bundle ID          : {}", rep.bundle_id);
    println!(
        "  Target             : {} ({})",
        rep.target.as_str(),
        rep.target.rust_target()
    );
    println!("  Output             : {}", rep.output_path.display());
    println!("  Primary PE         : Game/{}", rep.primary_exe_rel);
    println!("  Static AOT Objects : {}", rep.static_aot_objects.len());
    println!("  Shaders Bundled    : {}", rep.shaders_bundled);
    if let Some(ref spec) = rep.xcodegen_spec_path {
        println!("  XcodeGen Spec      : {}", spec.display());
    }
    if let Some(ref srep) = rep.storage_opt_report {
        print!("{}", format_storage_opt_report(srep, &storage_opt));
    }
    print!("{}", format_ios_bundle_verify_report(&rep.verify));
    if !rep.verify.ok() {
        bail!(
            "iOS bundle verification reported {} issue(s)",
            rep.verify.issues.len()
        );
    }
    Ok(())
}

pub fn cmd_ios_verify(args: &[String]) -> Result<()> {
    let Some(path) = args.first() else {
        usage();
    };
    let rep = verify_ios_bundle(Path::new(path))?;
    print!("{}", format_ios_bundle_verify_report(&rep));
    if !rep.ok() {
        std::process::exit(1);
    }
    Ok(())
}

pub fn cmd_optimize_assets(args: &[String]) -> Result<()> {
    let mut src_dir: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut opts = StorageOptOptions::from_profile(StorageOptProfile::High);
    let mut profile_explicit = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" | "--dst" => out_dir = Some(cur.next_path_or_usage()),
            "--manifest" | "-m" => manifest_path = Some(cur.next_path_or_usage()),
            "--profile" => {
                let raw = cur.next_or_usage();
                let prof = StorageOptProfile::parse(raw).ok_or_else(|| {
                    anyhow::anyhow!(
                        "invalid --profile {raw:?} (expected lossless, high, or compact)"
                    )
                })?;
                opts = StorageOptOptions::from_profile(prof);
                profile_explicit = true;
            }
            "--hevc-videos" => {
                if !profile_explicit {
                    opts = StorageOptOptions::from_profile(StorageOptProfile::None);
                    profile_explicit = true;
                }
                opts.hevc_videos = true;
                opts.repack_archives = true;
            }
            "--no-hevc-videos" => opts.hevc_videos = false,
            "--video-max-height" => {
                let raw = cur.next_or_usage();
                opts.video_max_height = raw
                    .parse()
                    .with_context(|| format!("invalid --video-max-height {raw:?}"))?;
            }
            "--video-bitrate" => {
                let raw = cur.next_or_usage();
                opts.video_bitrate_kbps = raw
                    .parse()
                    .with_context(|| format!("invalid --video-bitrate {raw:?}"))?;
            }
            "--max-tex-dim" => {
                let raw = cur.next_or_usage();
                opts.max_tex_dim = raw
                    .parse()
                    .with_context(|| format!("invalid --max-tex-dim {raw:?}"))?;
                if opts.max_tex_dim > 0 {
                    opts.repack_archives = true;
                }
            }
            "--strip-unused-dlls" => opts.strip_unused_dlls = true,
            "--no-strip-unused-dlls" => opts.strip_unused_dlls = false,
            "--keep-lang" => {
                opts.keep_lang = Some(cur.next_or_usage().to_string());
                opts.repack_archives = true;
            }
            "--strip-dlc" => {
                opts.strip_dlc = true;
                opts.repack_archives = true;
            }
            "--jobs" | "-j" => {
                let raw = cur.next_or_usage();
                opts.jobs = raw
                    .parse()
                    .with_context(|| format!("invalid --jobs {raw:?}"))?;
            }
            "--dry-run" => opts.dry_run = true,
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if src_dir.is_none() => src_dir = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
    }

    let Some(src_dir) = src_dir else {
        usage();
    };
    let Some(out_dir) = out_dir else {
        bail!("optimize-assets requires --out <DST_DIR>");
    };

    opts.workspace_root = Some(workspace_root());
    if manifest_path.is_none() {
        let sibling = src_dir.join("game.toml");
        if sibling.is_file() {
            manifest_path = Some(sibling);
        }
    }
    if let Some(ref mp) = manifest_path {
        let text = std::fs::read_to_string(mp)
            .with_context(|| format!("reading manifest {}", mp.display()))?;
        let cfg = parse_manifest_storage_config(&text);
        opts.apply_manifest_config(&cfg);
    }

    let rep = optimize_game_assets(&src_dir, &out_dir, &opts, None)?;
    print!("{}", format_storage_opt_report(&rep, &opts));
    Ok(())
}

pub fn cmd_bundles() -> Result<()> {
    let root = workspace_root();
    let bundles = list_packaged_bundles(&root);
    println!("=== Packaged macOS .app Bundles ({}) ===", bundles.len());
    for b in &bundles {
        let gb = (b.size_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
        let mut fws = Vec::new();
        if b.has_metal_irconverter {
            fws.push("MSC");
        }
        if b.has_dxilconv {
            fws.push("DXILConv");
        }
        if b.has_moltenvk {
            fws.push("MoltenVK");
        }
        println!(
            "  - {:24} {:>6.2} GiB | runtime={:20} | aot={} | fw=[{}] | {}",
            format!("{}.app", b.name),
            gb,
            b.runtime,
            b.aot_modules,
            fws.join(","),
            b.path.display()
        );
    }
    let vols = list_external_volumes();
    println!("\n=== Mounted External Volumes ({}) ===", vols.len());
    if vols.is_empty() {
        println!("  (none mounted under /Volumes/)");
    } else {
        for v in &vols {
            let free_gb = (v.free_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
            let total_gb = (v.total_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
            println!(
                "  - {:24} {:>6.1} GiB free / {:>6.1} GiB total | {}",
                v.name,
                free_gb,
                total_gb,
                v.path.display()
            );
        }
    }
    Ok(())
}

pub fn cmd_bundle_verify(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("bundle-verify requires <PATH.app>");
    }
    let mut all_ok = true;
    for raw in args {
        let p = PathBuf::from(raw);
        let report = verify_app_bundle(&p)?;
        print!("{}", format_bundle_verify_report(&report));
        if !report.passed() {
            all_ok = false;
        }
    }
    if !all_ok {
        std::process::exit(1);
    }
    Ok(())
}

pub fn cmd_bundle_export(args: &[String]) -> Result<()> {
    let mut positional: Vec<PathBuf> = Vec::new();
    let mut force = false;
    for a in args {
        match a.as_str() {
            "--force" => force = true,
            other if other.starts_with('-') => bail!("unknown bundle-export flag {other:?}"),
            other => positional.push(PathBuf::from(other)),
        }
    }
    if positional.len() != 2 {
        bail!("bundle-export requires <PATH.app> <DEST_DIR_OR_SSD> [--force]");
    }
    export_app_bundle(&positional[0], &positional[1], force)?;
    Ok(())
}
