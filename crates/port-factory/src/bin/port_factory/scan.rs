//! PE scanning, Win32/app/anti-cheat/.NET auditing, static Rust AOT, and raw shader CLI handlers.

use super::{usage, workspace_root, ArgCursor, DEFAULT_PROFILES_TSV};
use anyhow::{bail, Context, Result};
use pe_image::{ExportTarget, Pe};
use port_factory::anticheat::{audit_pe_anticheat, format_anticheat_report};
use port_factory::aot::{compile_pe_file_to_rust, AotOptions};
use port_factory::dotnet::{audit_dotnet_paths, strip_ready_to_run_file, DotNetRuntimeKind};
use port_factory::gpu_shaders::{
    audit_pe_app_capabilities, convert_shader_file, format_app_capability_report,
    inspect_shader_file, precompile_shaders_for_paths,
};
use port_factory::scaffold::generate_embeddable_aot_crate;
use port_factory::scanner::{
    collect_pe_files, format_profile_report, scan_pe_file, write_profiles_tsv,
};
use port_factory::win32_db::{audit_pe_imports, format_audit_report, Win32Database};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn cmd_scan(args: &[String]) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut record = false;
    let mut top_n = 8usize;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--record" => record = true,
            "--top" => top_n = cur.next_or_usage().parse().unwrap_or(8),
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        usage();
    }
    let mut files = Vec::new();
    for p in &paths {
        files.extend(collect_pe_files(p)?);
    }
    if files.is_empty() {
        bail!("no .exe or .dll files found in {:?}", paths);
    }
    let mut profiles = Vec::new();
    for f in &files {
        match scan_pe_file(f) {
            Ok(prof) => {
                print!("{}", format_profile_report(&prof, top_n));
                println!();
                profiles.push(prof);
            }
            Err(e) => {
                eprintln!("warning: skipping {}: {e:#}", f.display());
            }
        }
    }
    if record && !profiles.is_empty() {
        let out_path = Path::new(DEFAULT_PROFILES_TSV);
        write_profiles_tsv(out_path, &profiles)?;
        println!(
            "recorded {} binary profile(s) to {}",
            profiles.len(),
            out_path.display()
        );
    }
    Ok(())
}

pub fn cmd_win32_audit(args: &[String]) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for a in args {
        if a.starts_with('-') {
            bail!("unknown flag {a:?}");
        }
        paths.push(PathBuf::from(a));
    }
    if paths.is_empty() {
        usage();
    }
    let mut files = Vec::new();
    for p in &paths {
        files.extend(collect_pe_files(p)?);
    }
    if files.is_empty() {
        bail!("no .exe or .dll files found in {:?}", paths);
    }

    let root = workspace_root();
    let db = Win32Database::from_workspace_src(&root)?;

    let mut sibling_exports: HashMap<(String, String), u32> = HashMap::new();
    for f in &files {
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
                        sibling_exports.insert((dll_name.clone(), format!("#{}", e.ordinal)), rva);
                    }
                }
            }
        }
    }

    let mut total_imports = 0usize;
    let mut total_resolved = 0usize;
    for f in &files {
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
        println!("{}", format_audit_report(&audit));
    }
    if files.len() > 1 && total_imports > 0 {
        println!(
            "=== Aggregate Win32 Import Readiness Across {} Binaries: {}/{} ({:.2}%) ===",
            files.len(),
            total_resolved,
            total_imports,
            (total_resolved as f64 * 100.0) / (total_imports as f64)
        );
    }
    Ok(())
}

pub fn cmd_aot(args: &[String]) -> Result<()> {
    let mut pe_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut opts = AotOptions::default();

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" => out_path = Some(cur.next_path_or_usage()),
            "--max-fns" => {
                opts.max_functions = cur.next_or_usage().parse().context("--max-fns")?;
            }
            "--no-win64-attr" => opts.emit_win64_abi_attr = false,
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p => pe_path = Some(PathBuf::from(p)),
        }
    }
    let Some(pe_path) = pe_path else {
        usage();
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
    Ok(())
}

pub fn cmd_aot_crate(args: &[String]) -> Result<()> {
    let mut pe_path: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut crate_name: Option<String> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" => out_dir = Some(cur.next_path_or_usage()),
            "--name" => crate_name = Some(cur.next_or_usage().to_string()),
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p => pe_path = Some(PathBuf::from(p)),
        }
    }
    let (Some(pe_path), Some(out_dir)) = (pe_path, out_dir) else {
        usage();
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
    Ok(())
}

pub fn cmd_shader_inspect(args: &[String]) -> Result<()> {
    if args.is_empty() {
        usage();
    }
    for a in args {
        if a.starts_with('-') {
            bail!("unknown flag {a:?}");
        }
        let p = PathBuf::from(a);
        if p.is_file() {
            let insp = inspect_shader_file(&p)?;
            println!(
                "================================================================================"
            );
            print!("{}", insp.format_summary());
        } else if p.is_dir() {
            let rd = std::fs::read_dir(&p)?;
            for entry in rd.flatten() {
                let ep = entry.path();
                if ep.is_file() {
                    if let Ok(insp) = inspect_shader_file(&ep) {
                        println!("================================================================================");
                        print!("{}", insp.format_summary());
                    }
                }
            }
        } else {
            bail!("path does not exist: {}", p.display());
        }
    }
    Ok(())
}

pub fn cmd_shader_convert(args: &[String]) -> Result<()> {
    let mut input: Option<PathBuf> = None;
    let mut out_metallib: Option<PathBuf> = None;
    let mut out_msl: Option<PathBuf> = None;
    let mut entry: Option<String> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--out" | "-o" => out_metallib = Some(cur.next_path_or_usage()),
            "--msl-out" | "--msl" => out_msl = Some(cur.next_path_or_usage()),
            "--entry" | "-e" => entry = Some(cur.next_or_usage().to_string()),
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p => input = Some(PathBuf::from(p)),
        }
    }
    let Some(input) = input else {
        usage();
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
    Ok(())
}

pub fn cmd_app_audit(args: &[String]) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for a in args {
        if a.starts_with('-') {
            bail!("unknown flag {a:?}");
        }
        paths.push(PathBuf::from(a));
    }
    if paths.is_empty() {
        usage();
    }
    let mut files = Vec::new();
    for p in &paths {
        files.extend(collect_pe_files(p)?);
    }
    if files.is_empty() {
        bail!("no .exe or .dll files found in {:?}", paths);
    }
    let mut profiles = Vec::new();
    for f in &files {
        if let Ok(prof) = audit_pe_app_capabilities(f) {
            profiles.push(prof);
        }
    }
    print!("{}", format_app_capability_report(&profiles));
    Ok(())
}

pub fn cmd_precompile_shaders(args: &[String]) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut cache_dir: Option<PathBuf> = None;
    let mut limit: usize = usize::MAX;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--cache" => cache_dir = Some(cur.next_path_or_usage()),
            "--limit" => {
                limit = cur.next_or_usage().parse().context("invalid --limit")?;
            }
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        usage();
    }
    let summary = precompile_shaders_for_paths(&paths, cache_dir, limit)?;
    print!("{}", summary.format_report());
    Ok(())
}

pub fn cmd_anticheat_audit(args: &[String]) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for a in args {
        if a.starts_with('-') {
            bail!("unknown flag {a:?}");
        }
        paths.push(PathBuf::from(a));
    }
    if paths.is_empty() {
        usage();
    }
    let mut files = Vec::new();
    for p in &paths {
        files.extend(collect_pe_files(p)?);
    }
    if files.is_empty() {
        bail!("no .exe or .dll files found in {:?}", paths);
    }
    for f in &files {
        if let Ok(rep) = audit_pe_anticheat(f) {
            print!("{}", format_anticheat_report(&rep));
            println!();
        }
    }
    Ok(())
}

pub fn cmd_dotnet_audit(args: &[String]) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut strip_r2r_out: Option<PathBuf> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--strip-r2r" => strip_r2r_out = Some(cur.next_path_or_usage()),
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        usage();
    }

    if let Some(ref out_file) = strip_r2r_out {
        if paths.len() != 1 || !paths[0].is_file() {
            bail!("--strip-r2r requires a single input .NET PE file");
        }
        let stripped = strip_ready_to_run_file(&paths[0], out_file)?;
        println!(
            "ReadyToRun header stripped={stripped}: {} -> {}",
            paths[0].display(),
            out_file.display()
        );
        return Ok(());
    }

    let summary = audit_dotnet_paths(&paths)?;
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
    Ok(())
}
