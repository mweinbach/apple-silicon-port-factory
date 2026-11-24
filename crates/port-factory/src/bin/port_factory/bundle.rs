//! AOT prewarm / static Mach-O emission, iOS/macOS bundle packaging & verification,
//! external SSD export, and storage asset optimization CLI handlers.

use super::{emit_json, usage_err, workspace_root, ArgCursor};
use anyhow::{bail, Context, Result};
use port_factory::bundle_common::find_or_build_host_port_runner;
use port_factory::ios_bundle::{
    build_ios_bundle, format_ios_bundle_verify_report, verify_ios_bundle, IosBundleOptions,
    IosTarget,
};
use port_factory::json_output::{
    bundle_verify_data_json, bundles_list_data_json, exit_codes, ios_verify_data_json, JsonEnvelope,
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
use std::path::PathBuf;

pub fn cmd_aot_prewarm(args: &[String]) -> Result<i32> {
    let mut pe_path: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut game_dir: Option<PathBuf> = None;
    let mut aot_dir: Option<PathBuf> = None;
    let mut max_funcs: Option<usize> = None;
    let mut json = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--manifest" => manifest_path = Some(cur.next_path_or_err("--manifest requires FILE")?),
            "--game-dir" => game_dir = Some(cur.next_path_or_err("--game-dir requires DIR")?),
            "--aot-dir" => aot_dir = Some(cur.next_path_or_err("--aot-dir requires DIR")?),
            "--max-funcs" | "--aot-max-funcs" => {
                let raw = cur.next_or_err("--max-funcs requires N")?;
                max_funcs = Some(
                    raw.parse()
                        .map_err(|_| usage_err(format!("invalid --max-funcs {raw:?}")))?,
                );
            }
            "--json" => json = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p if pe_path.is_none() => pe_path = Some(PathBuf::from(p)),
            other => return Err(usage_err(format!("unexpected argument {other:?}"))),
        }
    }
    if pe_path.is_none() && manifest_path.is_none() {
        return Err(usage_err(
            "usage: port-factory aot-prewarm [PE_FILE] [--manifest FILE] [--game-dir DIR] [--aot-dir DIR] [--max-funcs N] [--json]",
        ));
    }
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
                        usage_err(format!(
                            "no valid x86-64 PE32+ binaries (.exe/.dll/.pyd/.node/.ocx) found in {}",
                            pe.display()
                        ))
                    })?;
                pe_path = Some(primary);
            }
        } else if !pe.is_file() {
            return Err(usage_err(format!(
                "PE file or directory not found: {}",
                pe.display()
            )));
        } else if !port_factory::scanner::is_valid_x64_pe_file(pe) {
            return Err(usage_err(format!(
                "not a valid x86-64 PE32+ binary: {}",
                pe.display()
            )));
        }
    }
    if let Some(ref mp) = manifest_path {
        if !mp.is_file() {
            return Err(usage_err(format!("manifest file not found: {}", mp.display())));
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
    if json {
        cmd.arg("--json");
    }
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
        if !out.stderr.is_empty() && !json {
            eprintln!("{}", String::from_utf8_lossy(&out.stderr));
        }
        return Ok(out.status.code().unwrap_or(exit_codes::RUNTIME_ERROR));
    }
    Ok(exit_codes::OK)
}

pub fn cmd_aot_static_obj(args: &[String]) -> Result<i32> {
    let mut input: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut platform = "ios".to_string();
    let mut game_dir: Option<PathBuf> = None;
    let mut max_funcs: Option<usize> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--manifest" => manifest_path = Some(cur.next_path_or_err("--manifest requires FILE")?),
            "--out" | "-o" => out_path = Some(cur.next_path_or_err("--out requires PATH")?),
            "--platform" | "--static-platform" => {
                platform = cur.next_or_err("--platform requires PLATFORM")?.to_string()
            }
            "--game-dir" => game_dir = Some(cur.next_path_or_err("--game-dir requires DIR")?),
            "--max-funcs" | "--aot-max-funcs" => {
                let raw = cur.next_or_err("--max-funcs requires N")?;
                max_funcs = Some(
                    raw.parse()
                        .map_err(|_| usage_err(format!("invalid --max-funcs {raw:?}")))?,
                );
            }
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p if input.is_none() => input = Some(PathBuf::from(p)),
            other => return Err(usage_err(format!("unexpected argument {other:?}"))),
        }
    }
    if input.is_none() && manifest_path.is_none() {
        return Err(usage_err(
            "usage: port-factory aot-static-obj [PE_OR_AOT] --out <DIR_OR_O> [--platform ios|ios-sim|macos]",
        ));
    }
    if let Some(ref inp) = input {
        if !inp.is_file() {
            return Err(usage_err(format!("input file not found: {}", inp.display())));
        }
    }
    if let Some(ref mp) = manifest_path {
        if !mp.is_file() {
            return Err(usage_err(format!("manifest file not found: {}", mp.display())));
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
    Ok(exit_codes::OK)
}

pub fn cmd_ios_bundle(args: &[String]) -> Result<i32> {
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
            "--out" | "-o" => out_path = Some(cur.next_path_or_err("--out requires PATH")?),
            "--target" | "--platform" => {
                let raw = cur.next_or_err("--target requires device|sim")?;
                target = IosTarget::parse(raw)
                    .ok_or_else(|| usage_err(format!("invalid --target {raw:?} (expected device or sim)")))?;
            }
            "--title" => title_override = Some(cur.next_or_err("--title requires NAME")?.to_string()),
            "--bundle-id" => {
                bundle_id = Some(cur.next_or_err("--bundle-id requires ID")?.to_string())
            }
            "--exe" => exe_override = Some(cur.next_or_err("--exe requires NAME")?.to_string()),
            "--manifest" => {
                manifest_override = Some(cur.next_path_or_err("--manifest requires FILE")?)
            }
            "--release" => release = true,
            "--deployment-target" => {
                deployment_target = cur
                    .next_or_err("--deployment-target requires VER")?
                    .to_string()
            }
            "--no-static-aot" => emit_static_aot = false,
            "--aot-max-funcs" | "--max-funcs" => {
                let raw = cur.next_or_err("--aot-max-funcs requires N")?;
                aot_max_funcs = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --aot-max-funcs {raw:?}")))?;
            }
            "--extra-aot-obj" => {
                extra_aot_objs.push(cur.next_path_or_err("--extra-aot-obj requires FILE.o")?)
            }
            "--no-precompile-shaders" => precompile_shaders = false,
            "--bundle-shader-cache" => {
                bundle_shader_cache_from = Some(
                    cur.next_opt_value()
                        .map(PathBuf::from)
                        .unwrap_or_else(WinFs::default_shader_cache_dir),
                );
            }
            "--optimize-storage" | "--storage-profile" => {
                let raw = cur.next_or_err("--optimize-storage requires PROFILE")?;
                let prof = StorageOptProfile::parse(raw).ok_or_else(|| {
                    usage_err(format!(
                        "invalid --optimize-storage {raw:?} (expected none, lossless, high, or compact)"
                    ))
                })?;
                storage_opt = StorageOptOptions::from_profile(prof);
            }
            "--hevc-videos" => {
                storage_opt.hevc_videos = true;
                storage_opt.repack_archives = true;
            }
            "--no-hevc-videos" => storage_opt.hevc_videos = false,
            "--video-max-height" => {
                let raw = cur.next_or_err("--video-max-height requires N")?;
                storage_opt.video_max_height = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --video-max-height {raw:?}")))?;
            }
            "--video-bitrate" => {
                let raw = cur.next_or_err("--video-bitrate requires KBPS")?;
                storage_opt.video_bitrate_kbps = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --video-bitrate {raw:?}")))?;
            }
            "--max-tex-dim" => {
                let raw = cur.next_or_err("--max-tex-dim requires N")?;
                storage_opt.max_tex_dim = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --max-tex-dim {raw:?}")))?;
                if storage_opt.max_tex_dim > 0 {
                    storage_opt.repack_archives = true;
                }
            }
            "--strip-unused-dlls" => storage_opt.strip_unused_dlls = true,
            "--keep-lang" => {
                storage_opt.keep_lang = Some(cur.next_or_err("--keep-lang requires CODE")?.to_string());
                storage_opt.repack_archives = true;
            }
            "--strip-dlc" => {
                storage_opt.strip_dlc = true;
                storage_opt.repack_archives = true;
            }
            "--opt-jobs" => {
                let raw = cur.next_or_err("--opt-jobs requires N")?;
                storage_opt.jobs = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --opt-jobs {raw:?}")))?;
            }
            "--opt-dry-run" => storage_opt.dry_run = true,
            "--sign" | "--codesign" => {
                codesign_identity = Some(cur.next_or_err("--sign requires ID")?.to_string())
            }
            "--provisioning-profile" => {
                provisioning_profile =
                    Some(cur.next_path_or_err("--provisioning-profile requires FILE")?)
            }
            "--xcode-project" | "--xcodegen" => emit_xcodegen_spec = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p if source.is_none() => source = Some(PathBuf::from(p)),
            other => return Err(usage_err(format!("unexpected argument {other:?}"))),
        }
    }

    let Some(source) = source else {
        return Err(usage_err(
            "usage: port-factory ios-bundle <PE_OR_DIR_OR_TOML> --out <PATH.app|PATH.ipa>",
        ));
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
        eprintln!(
            "Error: iOS bundle verification reported {} issue(s)",
            rep.verify.issues.len()
        );
        return Ok(exit_codes::VERIFICATION_FAILED);
    }
    Ok(exit_codes::OK)
}

pub fn cmd_ios_verify(args: &[String]) -> Result<i32> {
    let mut path: Option<PathBuf> = None;
    let mut json = false;
    for a in args {
        match a.as_str() {
            "--json" => json = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p if path.is_none() => path = Some(PathBuf::from(p)),
            other => return Err(usage_err(format!("unexpected argument {other:?}"))),
        }
    }
    let Some(path) = path else {
        return Err(usage_err(
            "usage: port-factory ios-verify <PATH.app|PATH.ipa> [--json]",
        ));
    };
    let rep = verify_ios_bundle(&path)?;
    let code = if rep.ok() {
        exit_codes::OK
    } else {
        exit_codes::VERIFICATION_FAILED
    };
    if json {
        let data = ios_verify_data_json(&rep);
        let err_msg = (!rep.ok()).then(|| {
            format!("iOS bundle verification reported {} issue(s)", rep.issues.len())
        });
        return Ok(emit_json(&JsonEnvelope::with_status(
            "ios-verify",
            code,
            data,
            err_msg,
        )));
    }
    print!("{}", format_ios_bundle_verify_report(&rep));
    Ok(code)
}

pub fn cmd_optimize_assets(args: &[String]) -> Result<i32> {
    let mut src_dir: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut opts = StorageOptOptions::from_profile(StorageOptProfile::High);
    let mut profile_explicit = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" | "--dst" => out_dir = Some(cur.next_path_or_err("--out requires DIR")?),
            "--manifest" | "-m" => {
                manifest_path = Some(cur.next_path_or_err("--manifest requires FILE")?)
            }
            "--profile" => {
                let raw = cur.next_or_err("--profile requires PROFILE")?;
                let prof = StorageOptProfile::parse(raw).ok_or_else(|| {
                    usage_err(format!(
                        "invalid --profile {raw:?} (expected lossless, high, or compact)"
                    ))
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
                let raw = cur.next_or_err("--video-max-height requires N")?;
                opts.video_max_height = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --video-max-height {raw:?}")))?;
            }
            "--video-bitrate" => {
                let raw = cur.next_or_err("--video-bitrate requires KBPS")?;
                opts.video_bitrate_kbps = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --video-bitrate {raw:?}")))?;
            }
            "--max-tex-dim" => {
                let raw = cur.next_or_err("--max-tex-dim requires N")?;
                opts.max_tex_dim = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --max-tex-dim {raw:?}")))?;
                if opts.max_tex_dim > 0 {
                    opts.repack_archives = true;
                }
            }
            "--strip-unused-dlls" => opts.strip_unused_dlls = true,
            "--no-strip-unused-dlls" => opts.strip_unused_dlls = false,
            "--keep-lang" => {
                opts.keep_lang = Some(cur.next_or_err("--keep-lang requires CODE")?.to_string());
                opts.repack_archives = true;
            }
            "--strip-dlc" => {
                opts.strip_dlc = true;
                opts.repack_archives = true;
            }
            "--jobs" | "-j" => {
                let raw = cur.next_or_err("--jobs requires N")?;
                opts.jobs = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --jobs {raw:?}")))?;
            }
            "--dry-run" => opts.dry_run = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p if src_dir.is_none() => src_dir = Some(PathBuf::from(p)),
            other => return Err(usage_err(format!("unexpected argument {other:?}"))),
        }
    }

    let Some(src_dir) = src_dir else {
        return Err(usage_err(
            "usage: port-factory optimize-assets <SRC_DIR> --out <DST_DIR>",
        ));
    };
    let Some(out_dir) = out_dir else {
        return Err(usage_err("optimize-assets requires --out <DST_DIR>"));
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
    Ok(exit_codes::OK)
}

pub fn cmd_bundles(args: &[String]) -> Result<i32> {
    let mut json = false;
    for a in args {
        match a.as_str() {
            "--json" => json = true,
            other => return Err(usage_err(format!("unknown bundles flag {other:?}"))),
        }
    }
    let root = workspace_root();
    let bundles = list_packaged_bundles(&root);
    let vols = list_external_volumes();
    if json {
        let data = bundles_list_data_json(&bundles, &vols);
        return Ok(emit_json(&JsonEnvelope::ok("bundles", data)));
    }
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
    Ok(exit_codes::OK)
}

pub fn cmd_bundle_verify(args: &[String]) -> Result<i32> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut json = false;
    for a in args {
        match a.as_str() {
            "--json" => json = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        return Err(usage_err("bundle-verify requires <PATH.app> [--json]"));
    }
    let mut reports = Vec::new();
    let mut all_ok = true;
    for p in &paths {
        let report = verify_app_bundle(p)?;
        if !json {
            print!("{}", format_bundle_verify_report(&report));
        }
        if !report.passed() {
            all_ok = false;
        }
        reports.push(report);
    }
    let code = if all_ok {
        exit_codes::OK
    } else {
        exit_codes::VERIFICATION_FAILED
    };
    if json {
        let data = bundle_verify_data_json(&reports);
        let failed = reports.iter().filter(|r| !r.passed()).count();
        let err_msg = (!all_ok).then(|| format!("{failed} bundle(s) failed verification"));
        return Ok(emit_json(&JsonEnvelope::with_status(
            "bundle-verify",
            code,
            data,
            err_msg,
        )));
    }
    Ok(code)
}

pub fn cmd_bundle_export(args: &[String]) -> Result<i32> {
    let mut positional: Vec<PathBuf> = Vec::new();
    let mut force = false;
    for a in args {
        match a.as_str() {
            "--force" => force = true,
            other if other.starts_with('-') => {
                return Err(usage_err(format!("unknown bundle-export flag {other:?}")))
            }
            other => positional.push(PathBuf::from(other)),
        }
    }
    if positional.len() != 2 {
        return Err(usage_err(
            "bundle-export requires <PATH.app> <DEST_DIR_OR_SSD> [--force]",
        ));
    }
    export_app_bundle(&positional[0], &positional[1], force)?;
    Ok(exit_codes::OK)
}
