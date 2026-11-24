//! PE scanning, Win32/app/anti-cheat/.NET auditing, static Rust AOT, and raw shader CLI handlers.

use super::{emit_json, usage_err, workspace_root, ArgCursor, DEFAULT_PROFILES_TSV};
use anyhow::{Context, Result};
use pe_image::{ExportTarget, Pe};
use port_factory::anticheat::{audit_pe_anticheat, format_anticheat_report, AntiCheatVerdict};
use port_factory::aot::{compile_pe_file_to_rust, AotOptions};
use port_factory::audit_game::{audit_game_dir, AuditGameOptions, ReadinessLevel};
use port_factory::dotnet::{audit_dotnet_paths, strip_ready_to_run_file, DotNetRuntimeKind};
use port_factory::gpu_shaders::{
    audit_pe_app_capabilities, convert_shader_file, format_app_capability_report,
    inspect_shader_file, precompile_shaders_for_paths,
};
use port_factory::json_output::{
    anticheat_audit_data_json, app_audit_data_json, audit_game_data_json, dotnet_audit_data_json,
    exit_codes, scan_data_json, shader_inspect_data_json, win32_audit_data_json, JsonEnvelope,
    JsonObj, JsonValue,
};
use port_factory::scaffold::generate_embeddable_aot_crate;
use port_factory::scanner::{
    collect_pe_files, format_profile_report, scan_pe_file, write_profiles_tsv,
};
use port_factory::win32_db::{audit_pe_imports, format_audit_report, ImplKind, Win32Database};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn collect_pe_targets_or_err(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for p in paths {
        files.extend(collect_pe_files(p)?);
    }
    if files.is_empty() {
        return Err(usage_err(format!(
            "no valid x86-64 PE32+ binaries (.exe/.dll/.pyd/.node/.ocx) found in {paths:?}"
        )));
    }
    Ok(files)
}

pub fn cmd_scan(args: &[String]) -> Result<i32> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut record = false;
    let mut top_n = 8usize;
    let mut json = false;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--record" => record = true,
            "--top" => {
                let raw = cur.next_or_err("--top requires N")?;
                top_n = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --top value {raw:?}")))?;
            }
            "--json" => json = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        return Err(usage_err(
            "usage: port-factory scan <PE_FILE_OR_DIR...> [--record] [--top N] [--json]",
        ));
    }
    let files = collect_pe_targets_or_err(&paths)?;
    let mut profiles = Vec::new();
    let mut skipped = Vec::new();
    for f in &files {
        match scan_pe_file(f) {
            Ok(prof) => {
                if !json {
                    print!("{}", format_profile_report(&prof, top_n));
                    println!();
                }
                profiles.push(prof);
            }
            Err(e) => {
                if !json {
                    eprintln!("warning: skipping {}: {e:#}", f.display());
                }
                skipped.push((f.display().to_string(), format!("{e:#}")));
            }
        }
    }
    let mut recorded_tsv: Option<String> = None;
    if record && !profiles.is_empty() {
        let out_path = Path::new(DEFAULT_PROFILES_TSV);
        write_profiles_tsv(out_path, &profiles)?;
        recorded_tsv = Some(out_path.display().to_string());
        if !json {
            println!(
                "recorded {} binary profile(s) to {}",
                profiles.len(),
                out_path.display()
            );
        }
    }
    if json {
        let data = scan_data_json(&profiles, &skipped, recorded_tsv.as_deref(), top_n);
        return Ok(emit_json(&JsonEnvelope::ok("scan", data)));
    }
    Ok(exit_codes::OK)
}

pub fn cmd_win32_audit(args: &[String]) -> Result<i32> {
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
        return Err(usage_err(
            "usage: port-factory win32-audit <PE_FILE_OR_DIR...> [--json]",
        ));
    }
    let files = collect_pe_targets_or_err(&paths)?;
    let (audits, total_imports, total_resolved, total_unresolved) = run_win32_audit_files(&files)?;

    let code = if total_unresolved > 0 {
        exit_codes::UNRESOLVED_IMPORTS
    } else {
        exit_codes::OK
    };

    if json {
        let data = win32_audit_data_json(&audits);
        let err_msg = (total_unresolved > 0).then(|| {
            format!("{total_unresolved} unresolved Win32 import(s) across {} binary(s)", audits.len())
        });
        return Ok(emit_json(&JsonEnvelope::with_status(
            "win32-audit",
            code,
            data,
            err_msg,
        )));
    }

    for audit in &audits {
        println!("{}", format_audit_report(audit));
    }
    if files.len() > 1 && total_imports > 0 {
        let total_real: usize = audits
            .iter()
            .map(|a| a.count_by_impl_kind(ImplKind::Real))
            .sum();
        let total_sibling: usize = audits.iter().map(|a| a.resolved_sibling).sum();
        let total_partial: usize = audits
            .iter()
            .map(|a| a.count_by_impl_kind(ImplKind::Partial))
            .sum();
        let total_stub: usize = audits
            .iter()
            .map(|a| a.count_by_impl_kind(ImplKind::Stub))
            .sum();
        let total_failing: usize = audits
            .iter()
            .map(|a| a.count_by_impl_kind(ImplKind::Failing))
            .sum();
        println!(
            "=== Aggregate Win32 Import Readiness Across {} Binaries: {}/{} resolved ({:.2}%), {}/{} real ({:.2}%) ===",
            files.len(),
            total_resolved,
            total_imports,
            (total_resolved as f64 * 100.0) / (total_imports as f64),
            total_real + total_sibling,
            total_imports,
            ((total_real + total_sibling) as f64 * 100.0) / (total_imports as f64)
        );
        println!(
            "  Breakdown: real={} (+ {} sibling PE), partial={}, stub={}, failing={}, unresolved={}",
            total_real,
            total_sibling,
            total_partial,
            total_stub,
            total_failing,
            total_unresolved
        );
    }
    Ok(code)
}

pub fn cmd_audit_game(args: &[String]) -> Result<i32> {
    let mut game_dir: Option<PathBuf> = None;
    let mut main_exe: Option<PathBuf> = None;
    let mut init_manifest = false;
    let mut manifest_out: Option<PathBuf> = None;
    let mut force = false;
    let mut record = false;
    let mut game_id: Option<String> = None;
    let mut game_name: Option<String> = None;
    let mut json = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--json" => json = true,
            "--force" => force = true,
            "--record" => record = true,
            "--init-manifest" => {
                init_manifest = true;
                if let Some(val) = cur.next_opt_value() {
                    manifest_out = Some(PathBuf::from(val));
                }
            }
            "--exe" => main_exe = Some(cur.next_path_or_err("--exe requires PATH_OR_NAME")?),
            "--id" => game_id = Some(cur.next_or_err("--id requires ID")?.to_string()),
            "--name" => game_name = Some(cur.next_or_err("--name requires NAME")?.to_string()),
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => {
                if game_dir.is_none() {
                    game_dir = Some(PathBuf::from(p));
                } else if main_exe.is_none() {
                    main_exe = Some(PathBuf::from(p));
                } else {
                    return Err(usage_err(format!("unexpected positional argument {p:?}")));
                }
            }
        }
    }

    let Some(game_dir) = game_dir else {
        return Err(usage_err(
            "usage: port-factory audit-game <GAME_DIR_OR_PE> [--exe EXE] [--init-manifest [PATH]] [--force] [--record] [--id ID] [--name NAME] [--json]",
        ));
    };
    if !game_dir.exists() {
        return Err(usage_err(format!(
            "game directory not found: {}",
            game_dir.display()
        )));
    }

    let root = workspace_root();
    let db = Win32Database::from_workspace_src(&root)?;
    let opts = AuditGameOptions {
        main_exe,
        game_id,
        game_name,
    };
    let report = audit_game_dir(&game_dir, &db, &opts)?;

    let mut manifest_written: Option<PathBuf> = None;
    if init_manifest {
        let out_path = manifest_out.unwrap_or_else(|| report.default_manifest_path(&root));
        report.write_manifest(&out_path, force)?;
        manifest_written = Some(out_path);
    }

    let mut recorded_tsv: Option<String> = None;
    if record && !report.profiles.is_empty() {
        let out_path = Path::new(DEFAULT_PROFILES_TSV);
        write_profiles_tsv(out_path, &report.profiles)?;
        recorded_tsv = Some(out_path.display().to_string());
    }

    let code = if report.readiness == ReadinessLevel::Blocked {
        exit_codes::INCOMPATIBLE
    } else if report.imports.unresolved_count > 0 {
        exit_codes::UNRESOLVED_IMPORTS
    } else {
        exit_codes::OK
    };

    if json {
        let data = audit_game_data_json(
            &report,
            manifest_written.as_deref(),
            recorded_tsv.as_deref(),
        );
        let err_msg = match code {
            exit_codes::INCOMPATIBLE => Some(format!(
                "Ring-0 kernel driver blocker detected: {}",
                report.anti_tamper.kernel_driver_imports.join(", ")
            )),
            exit_codes::UNRESOLVED_IMPORTS => Some(format!(
                "{} unresolved Win32 import(s) across {} unique symbol(s)",
                report.imports.unresolved_count,
                report.imports.unresolved_imports.len()
            )),
            _ => None,
        };
        return Ok(emit_json(&JsonEnvelope::with_status(
            "audit-game",
            code,
            data,
            err_msg,
        )));
    }

    print!("{}", report.format_human_report(manifest_written.as_deref()));
    if let Some(ref tsv) = recorded_tsv {
        println!(
            "  Recorded {} binary profile(s) to {tsv}",
            report.profiles.len()
        );
    }
    Ok(code)
}

pub(crate) fn run_win32_audit_files(
    files: &[PathBuf],
) -> Result<(Vec<port_factory::win32_db::Win32ImportAudit>, usize, usize, usize)> {
    let root = workspace_root();
    let db = Win32Database::from_workspace_src(&root)?;

    let mut sibling_exports: HashMap<(String, String), u32> = HashMap::new();
    for f in files {
        if let Ok(bytes) = std::fs::read(f) {
            if let Ok(pe) = Pe::parse(&bytes) {
                let dll_name = pe_image::normalize_dll_name(
                    f.file_name().and_then(|s| s.to_str()).unwrap_or(""),
                );
                if let Ok(exps) = pe.exports() {
                    for e in exps.entries {
                        let rva = match e.target {
                            ExportTarget::Rva(r) => r,
                            ExportTarget::Forwarder(_) => 0,
                        };
                        if let Some(n) = e.name {
                            sibling_exports.insert((dll_name.clone(), n), rva);
                        }
                        sibling_exports
                            .insert((dll_name.clone(), format!("#{}", e.ordinal)), rva);
                    }
                }
            }
        }
    }

    let mut audits = Vec::new();
    let mut total_imports = 0usize;
    let mut total_resolved = 0usize;
    let mut total_unresolved = 0usize;
    for f in files {
        let bytes = match std::fs::read(f) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let name = f
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown.pe");
        let audit = match audit_pe_imports(name, &bytes, &db, Some(&sibling_exports)) {
            Ok(a) => a,
            Err(_) => continue,
        };
        total_imports += audit.total_imports;
        total_resolved += audit.resolved_win32;
        total_unresolved += audit.unresolved.len();
        audits.push(audit);
    }
    Ok((audits, total_imports, total_resolved, total_unresolved))
}

pub fn cmd_aot(args: &[String]) -> Result<i32> {
    let mut pe_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut opts = AotOptions::default();

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" => out_path = Some(cur.next_path_or_err("--out requires PATH")?),
            "--max-fns" => {
                let raw = cur.next_or_err("--max-fns requires N")?;
                opts.max_functions = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --max-fns {raw:?}")))?;
            }
            "--no-win64-attr" => opts.emit_win64_abi_attr = false,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => pe_path = Some(PathBuf::from(p)),
        }
    }
    let Some(pe_path) = pe_path else {
        return Err(usage_err(
            "usage: port-factory aot <PE_FILE> [--out PATH.rs] [--max-fns N]",
        ));
    };

    let root = workspace_root();
    let db = Win32Database::from_workspace_src(&root)?;
    let res = compile_pe_file_to_rust(&pe_path, &db, &opts)?;

    let iat_thunks = res
        .functions
        .iter()
        .filter(|f| f.iat_thunk_rva.is_some())
        .count();
    let bound_imports = res
        .imports
        .iter()
        .filter(|i| i.rust_path.is_some())
        .count();
    let exported_fns = res
        .functions
        .iter()
        .filter(|f| !f.export_names.is_empty())
        .count();

    println!("=== Static AOT Compilation Report: {} ===", res.binary_name);
    println!("  Preferred ImageBase   : 0x{:016X}", res.image_base);
    println!("  SizeOfImage           : 0x{:08X}", res.size_of_image);
    println!("  EntryPoint RVA        : 0x{:08X}", res.entry_rva);
    println!(
        "  Discovered Functions  : {} ({} exported, {} IAT thunks)",
        res.functions.len(),
        exported_fns,
        iat_thunks
    );
    println!(
        "  Win32 IAT Bindings    : {}/{} resolved via Win32Database",
        bound_imports,
        res.imports.len()
    );
    println!(
        "  Instruction Lowering  : {}/{} instructions lowered ({:.2}%), {} fallback",
        res.lowered_instructions,
        res.total_instructions,
        res.lowering_coverage_pct(),
        res.fallback_instructions
    );
    println!(
        "  Generated Rust Source : {} bytes ({} lines)",
        res.rust_source.len(),
        res.rust_source.lines().count()
    );

    if let Some(out) = out_path {
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&out, &res.rust_source)
            .with_context(|| format!("writing {}", out.display()))?;
        println!("  Wrote AOT module to   : {}", out.display());
    }
    Ok(exit_codes::OK)
}

pub fn cmd_aot_crate(args: &[String]) -> Result<i32> {
    let mut pe_path: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut crate_name: Option<String> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" => out_dir = Some(cur.next_path_or_err("--out requires DIR")?),
            "--name" => crate_name = Some(cur.next_or_err("--name requires CRATE")?.to_string()),
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => pe_path = Some(PathBuf::from(p)),
        }
    }
    let (Some(pe_path), Some(out_dir)) = (pe_path, out_dir) else {
        return Err(usage_err(
            "usage: port-factory aot-crate <PE_FILE> --out <DIR> [--name CRATE]",
        ));
    };

    let root = workspace_root();
    let db = Win32Database::from_workspace_src(&root)?;
    let summary = generate_embeddable_aot_crate(&pe_path, &out_dir, crate_name.as_deref(), &db)?;

    println!(
        "=== Generated Embeddable AOT Rust Crate: {} ===",
        summary.crate_name
    );
    println!("  Output Directory      : {}", summary.out_dir.display());
    println!(
        "  Compiled Functions    : {} ({} exported)",
        summary.function_count, summary.export_count
    );
    println!(
        "  Instruction Lowering  : {}/{} (100% standalone Rust)",
        summary.lowered_instructions, summary.instruction_count
    );
    println!(
        "  Embedded PE Image     : {} bytes (src/sections.bin)",
        summary.image_bytes
    );
    println!(
        "  Usage in Host Rust    : `{}::init_embedded_module();` then call any export!",
        summary.crate_name.replace('-', "_")
    );
    Ok(exit_codes::OK)
}

pub fn cmd_shader_inspect(args: &[String]) -> Result<i32> {
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
        return Err(usage_err(
            "usage: port-factory shader-inspect <FILE_OR_DIR...> [--json]",
        ));
    }
    let mut inspections = Vec::new();
    for p in &paths {
        if p.is_file() {
            let insp = inspect_shader_file(p)?;
            if !json {
                println!(
                    "================================================================================"
                );
                print!("{}", insp.format_summary());
            }
            inspections.push(insp);
        } else if p.is_dir() {
            let rd = std::fs::read_dir(p)?;
            for entry in rd.flatten() {
                let ep = entry.path();
                if ep.is_file() {
                    if let Ok(insp) = inspect_shader_file(&ep) {
                        if !json {
                            println!(
                                "================================================================================"
                            );
                            print!("{}", insp.format_summary());
                        }
                        inspections.push(insp);
                    }
                }
            }
        } else {
            return Err(usage_err(format!("path does not exist: {}", p.display())));
        }
    }
    if json {
        let data = shader_inspect_data_json(&inspections);
        return Ok(emit_json(&JsonEnvelope::ok("shader-inspect", data)));
    }
    Ok(exit_codes::OK)
}

pub fn cmd_shader_convert(args: &[String]) -> Result<i32> {
    let mut input: Option<PathBuf> = None;
    let mut out_metallib: Option<PathBuf> = None;
    let mut out_msl: Option<PathBuf> = None;
    let mut entry: Option<String> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" => {
                out_metallib = Some(cur.next_path_or_err("--out requires OUT.metallib")?)
            }
            "--msl-out" | "--msl" => {
                out_msl = Some(cur.next_path_or_err("--msl-out requires OUT.metal")?)
            }
            "--entry" | "-e" => {
                entry = Some(cur.next_or_err("--entry requires NAME")?.to_string())
            }
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => input = Some(PathBuf::from(p)),
        }
    }
    let Some(input) = input else {
        return Err(usage_err(
            "usage: port-factory shader-convert <INPUT> [--out OUT.metallib] [--msl-out OUT.metal] [--entry NAME]",
        ));
    };
    let conv = convert_shader_file(
        &input,
        entry.as_deref(),
        out_metallib.as_deref(),
        out_msl.as_deref(),
    )?;
    println!("================================================================================");
    println!("=== Native Raw Shader -> Apple Metal (.metallib) Conversion ===");
    println!("================================================================================");
    println!("  Input File           : {}", input.display());
    println!(
        "  Source Format        : {} ({})",
        conv.source_format.code(),
        conv.source_format.spec().family
    );
    println!("  Pipeline Executed    : {}", conv.pipeline_used);
    println!("  Metal Stage          : {:?}", conv.compiled.stage);
    println!("  Metal Entry Function : {}", conv.compiled.function_name);
    println!(
        "  Resource Bindings    : {}",
        conv.compiled.reflection.resources.len()
    );
    if let Some(tg) = conv.compiled.reflection.threadgroup_size() {
        println!(
            "  Threadgroup Size     : [{} x {} x {}]",
            tg[0], tg[1], tg[2]
        );
    }
    println!(
        "  Compiled .metallib   : {} bytes (magic={:?})",
        conv.compiled.metallib.len(),
        String::from_utf8_lossy(&conv.compiled.metallib[..4.min(conv.compiled.metallib.len())])
    );
    if let Some(ref out_p) = out_metallib {
        println!("  Saved .metallib to   : {}", out_p.display());
    }
    if let Some(ref msl_p) = out_msl {
        println!("  Saved .metal MSL to  : {}", msl_p.display());
    }
    Ok(exit_codes::OK)
}

pub fn cmd_app_audit(args: &[String]) -> Result<i32> {
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
        return Err(usage_err(
            "usage: port-factory app-audit <PE_FILE_OR_DIR...> [--json]",
        ));
    }
    let files = collect_pe_targets_or_err(&paths)?;
    let mut profiles = Vec::new();
    for f in &files {
        if let Ok(prof) = audit_pe_app_capabilities(f) {
            profiles.push(prof);
        }
    }
    if json {
        let data = app_audit_data_json(&profiles);
        return Ok(emit_json(&JsonEnvelope::ok("app-audit", data)));
    }
    print!("{}", format_app_capability_report(&profiles));
    Ok(exit_codes::OK)
}

pub fn cmd_precompile_shaders(args: &[String]) -> Result<i32> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut cache_dir: Option<PathBuf> = None;
    let mut limit: usize = usize::MAX;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--cache" => cache_dir = Some(cur.next_path_or_err("--cache requires DIR")?),
            "--limit" => {
                let raw = cur.next_or_err("--limit requires N")?;
                limit = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --limit {raw:?}")))?;
            }
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        return Err(usage_err(
            "usage: port-factory precompile-shaders <FILE_OR_DIR...> [--cache DIR] [--limit N]",
        ));
    }
    let summary = precompile_shaders_for_paths(&paths, cache_dir, limit)?;
    print!("{}", summary.format_report());
    Ok(exit_codes::OK)
}

pub fn cmd_anticheat_audit(args: &[String]) -> Result<i32> {
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
        return Err(usage_err(
            "usage: port-factory anticheat-audit <PE_FILE_OR_DIR...> [--json]",
        ));
    }
    let files = collect_pe_targets_or_err(&paths)?;
    let mut reports = Vec::new();
    for f in &files {
        if let Ok(rep) = audit_pe_anticheat(f) {
            if !json {
                print!("{}", format_anticheat_report(&rep));
                println!();
            }
            reports.push(rep);
        }
    }
    let blocked = reports
        .iter()
        .filter(|r| r.verdict == AntiCheatVerdict::RequiresKernelDriverFallback)
        .count();
    let code = if blocked > 0 {
        exit_codes::INCOMPATIBLE
    } else {
        exit_codes::OK
    };
    if json {
        let data = anticheat_audit_data_json(&reports);
        let err_msg = (blocked > 0)
            .then(|| format!("{blocked} binary(s) require Ring-0 kernel driver fallback"));
        return Ok(emit_json(&JsonEnvelope::with_status(
            "anticheat-audit",
            code,
            data,
            err_msg,
        )));
    }
    Ok(code)
}

pub fn cmd_dotnet_audit(args: &[String]) -> Result<i32> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut strip_r2r_out: Option<PathBuf> = None;
    let mut json = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--strip-r2r" => {
                strip_r2r_out = Some(cur.next_path_or_err("--strip-r2r requires OUT_FILE")?)
            }
            "--json" => json = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        return Err(usage_err(
            "usage: port-factory dotnet-audit <PE_FILE_OR_DIR...> [--strip-r2r OUT_FILE] [--json]",
        ));
    }

    if let Some(ref out_file) = strip_r2r_out {
        if paths.len() != 1 || !paths[0].is_file() {
            return Err(usage_err("--strip-r2r requires a single input .NET PE file"));
        }
        let stripped = strip_ready_to_run_file(&paths[0], out_file)?;
        if json {
            let data = JsonObj::new()
                .str("input", paths[0].display().to_string())
                .str("output", out_file.display().to_string())
                .bool("stripped", stripped)
                .build();
            return Ok(emit_json(&JsonEnvelope::ok("dotnet-audit", data)));
        }
        println!(
            "ReadyToRun header stripped={stripped}: {} -> {}",
            paths[0].display(),
            out_file.display()
        );
        return Ok(exit_codes::OK);
    }

    let summary = audit_dotnet_paths(&paths)?;
    if json {
        let data = dotnet_audit_data_json(&summary);
        return Ok(emit_json(&JsonEnvelope::ok("dotnet-audit", data)));
    }

    println!("================================================================================");
    println!("=== .NET / CLR / CoreCLR / Unity Mono / IL2CPP Dependency Audit ===");
    println!("================================================================================");
    println!("  Total PE Binaries Scanned     : {}", summary.binaries.len());
    println!(
        "  Pure-IL Assemblies (ILONLY)   : {} (Direct Native ARM64 CoreCLR/Mono Ready)",
        summary.pure_il_count
    );
    println!(
        "  ReadyToRun Hybrid (CIL+x64)   : {} (Strip R2R header or AOT-translate)",
        summary.r2r_count
    );
    println!(
        "  Mixed-Mode C++/CLI Assemblies : {} (x86-64->ARM64 AOT + mscoree/coreclr bridge)",
        summary.mixed_mode_count
    );
    println!(
        "  .NET AppHost / Host Launchers : {} (hostfxr.dll / coreclr.dll bridge)",
        summary.apphost_count
    );
    println!(
        "  Unity Mono Binaries           : {} (mono-2.0-bdwgc.dll bridge)",
        summary.unity_mono_count
    );
    println!(
        "  Unity IL2CPP Native Binaries  : {} (GameAssembly.dll -> 100% ARM64 AOT)",
        summary.unity_il2cpp_count
    );
    println!(
        "  Pure Native Non-.NET Binaries : {}",
        summary.native_non_dotnet_count
    );
    if !summary.runtime_configs.is_empty() {
        println!(
            "  .runtimeconfig.json Files     : {}",
            summary.runtime_configs.len()
        );
    }
    if !summary.global_metadata_dat.is_empty() {
        println!(
            "  IL2CPP global-metadata.dat    : {}",
            summary.global_metadata_dat.len()
        );
    }
    println!("--------------------------------------------------------------------------------");
    println!(
        "  Builtin Win32 .NET Bridges    : mscoree.dll, hostfxr.dll, coreclr.dll, mono-2.0-bdwgc.dll"
    );
    println!(
        "  Host macOS ARM64 CoreCLR      : {}",
        summary
            .host_coreclr_dylib
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| {
                "builtin in-process bridge (install `brew install dotnet` for native ARM64 JIT)"
                    .to_string()
            })
    );
    println!(
        "  Host macOS ARM64 hostfxr      : {}",
        summary
            .host_hostfxr_dylib
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "builtin in-process bridge".to_string())
    );
    println!(
        "  Host macOS ARM64 Mono         : {}",
        summary
            .host_mono_dylib
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| {
                "builtin in-process bridge (install `brew install mono` for native ARM64 Mono JIT)"
                    .to_string()
            })
    );

    let interesting: Vec<_> = summary
        .binaries
        .iter()
        .filter(|b| b.kind != DotNetRuntimeKind::NativeNonDotNet || summary.binaries.len() <= 5)
        .collect();
    if !interesting.is_empty() {
        println!(
            "--------------------------------------------------------------------------------"
        );
        for b in interesting {
            let arch = if b.is_pe32_plus {
                "PE32+ (64-bit)"
            } else {
                "PE32 (AnyCPU/32-bit)"
            };
            println!("  * {} [{} — {}]", b.file_name, arch, b.kind.as_str());
            if let Some(ref ver) = b.metadata_version {
                println!(
                    "      CLR Metadata : {} (flags=0x{:x}, R2R={}, StrongName={}, streams={:?})",
                    ver,
                    b.clr_flags,
                    b.is_ready_to_run,
                    b.is_strong_name_signed,
                    b.metadata_streams
                );
            }
            if let Some(mode) = b.clr_mode {
                println!("      Execution    : {}", mode.as_str());
            }
            println!("      Strategy     : {}", b.recommendation);
        }
    }
    Ok(exit_codes::OK)
}

/// Run all 5 PE audits (scan, win32-audit, anticheat-audit, app-audit, dotnet-audit) and
/// return them as a combined JSON object for `doctor --json <PE...>`.
pub(crate) fn run_doctor_pe_audits_json(pe_args: &[String]) -> Result<JsonValue> {
    let paths: Vec<PathBuf> = pe_args.iter().map(PathBuf::from).collect();
    let files = collect_pe_targets_or_err(&paths)?;

    let mut scan_profiles = Vec::new();
    let mut scan_skipped = Vec::new();
    for f in &files {
        match scan_pe_file(f) {
            Ok(p) => scan_profiles.push(p),
            Err(e) => scan_skipped.push((f.display().to_string(), format!("{e:#}"))),
        }
    }
    let (win32_audits, _, _, _) = run_win32_audit_files(&files)?;

    let mut ac_reports = Vec::new();
    for f in &files {
        if let Ok(rep) = audit_pe_anticheat(f) {
            ac_reports.push(rep);
        }
    }
    let mut app_profiles = Vec::new();
    for f in &files {
        if let Ok(prof) = audit_pe_app_capabilities(f) {
            app_profiles.push(prof);
        }
    }
    let dotnet_summary = audit_dotnet_paths(&paths)?;

    Ok(JsonObj::new()
        .val("scan", scan_data_json(&scan_profiles, &scan_skipped, None, 8))
        .val("win32_audit", win32_audit_data_json(&win32_audits))
        .val("anticheat_audit", anticheat_audit_data_json(&ac_reports))
        .val("app_audit", app_audit_data_json(&app_profiles))
        .val("dotnet_audit", dotnet_audit_data_json(&dotnet_summary))
        .build())
}
