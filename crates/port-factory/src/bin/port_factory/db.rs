//! Database catalog, scaffolding, VFS resolution, and `doctor` subcommand handlers.

use super::{
    emit_json, scan, usage_err, workspace_root, ArgCursor, DEFAULT_GAME_PROFILES_TSV,
    DEFAULT_OPS_TSV, DEFAULT_SHADER_FORMATS_TSV, DEFAULT_WIN32_TSV,
};
use anyhow::{bail, Result};
use pe_image::Pe;
use port_factory::doctor::check_or_fix_db_tsvs;
use port_factory::game_profiles::{
    audit_all_game_profiles, detect_pe_game_profile, sync_and_format_game_profiles,
    TOP_GAME_PROFILES,
};
use port_factory::gpu_shaders::sync_and_format_shader_catalog;
use port_factory::json_output::{
    doctor_data_json, exit_codes, game_profiles_data_json, ops_stats_data_json,
    shader_formats_data_json, vfs_resolve_data_json, win32_stats_data_json, JsonEnvelope,
};
use port_factory::ops_db::{catalog_stats, lookup_by_id, lookup_by_mnemonic, write_tsv, OPS_CATALOG};
use port_factory::scaffold::{
    find_missing_ops_in_pe, generate_dll_scaffold, generate_op_scaffold, register_misc_dll_module,
};
use port_factory::scanner::collect_pe_files;
use port_factory::vfs::WinFs;
use port_factory::win32_db::{Win32Category, Win32Database};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub fn cmd_sync(args: &[String]) -> Result<i32> {
    let mut path = PathBuf::from(DEFAULT_OPS_TSV);
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--tsv" => path = cur.next_path_or_err("--tsv requires PATH")?,
            _ => return Err(usage_err(format!("unknown argument {a:?}"))),
        }
    }
    write_tsv(&path)?;
    let st = catalog_stats();
    println!(
        "synced {} op families ({} x86-64 mnemonics, {} Rosetta-verified vectors) -> {}",
        st.total_records,
        st.total_mnemonics,
        st.total_verified_vectors,
        path.display()
    );
    Ok(exit_codes::OK)
}

pub fn cmd_stats(args: &[String]) -> Result<i32> {
    let mut json = false;
    for a in args {
        match a.as_str() {
            "--json" => json = true,
            _ => return Err(usage_err(format!("unknown argument {a:?}"))),
        }
    }
    let st = catalog_stats();
    if json {
        let data = ops_stats_data_json(&st, OPS_CATALOG);
        return Ok(emit_json(&JsonEnvelope::ok("stats", data)));
    }
    println!("=== x86-64 -> ARM64 Operation Translation Database ===");
    println!("  Op families          : {}", st.total_records);
    println!("  JIT_ARM64 families   : {}", st.jit_arm64_records);
    println!("  Unique x86 mnemonics : {}", st.total_mnemonics);
    println!("  FastOp kinds covered : K_1..=K_124 (100% of decoded FastOp kinds)");
    println!("  Verified vectors     : {}", st.total_verified_vectors);
    println!();
    println!(
        "  {:28} {:>6} {:>18}  {}",
        "OP_ID", "KIND", "CATEGORY", "ARM64_PATTERN"
    );
    println!("  {}", "-".repeat(100));
    for r in OPS_CATALOG {
        println!(
            "  {:28} {:>6} {:>18}  {}",
            r.id,
            format!("K_{}", r.fast_kind_id),
            r.category.as_str(),
            r.arm64_pattern
        );
    }
    Ok(exit_codes::OK)
}

pub fn cmd_list(args: &[String]) -> Result<i32> {
    let mut cat_filter: Option<String> = None;
    let mut search: Option<String> = None;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--category" => {
                cat_filter = Some(cur.next_or_err("--category requires CAT")?.to_uppercase())
            }
            "--search" => search = Some(cur.next_or_err("--search requires STR")?.to_lowercase()),
            _ => return Err(usage_err(format!("unknown argument {a:?}"))),
        }
    }
    for r in OPS_CATALOG {
        if let Some(c) = &cat_filter {
            if !r.category.as_str().contains(c.as_str()) {
                continue;
            }
        }
        if let Some(q) = &search {
            let hay = format!(
                "{} {} {} {} {}",
                r.id, r.fast_kind_name, r.x86_mnemonics, r.arm64_pattern, r.semantics
            )
            .to_lowercase();
            if !hay.contains(q.as_str()) {
                continue;
            }
        }
        println!(
            "{:28} [K_{:<3} {:18}] widths={:<11} forms={:<14} -> {}",
            r.id,
            r.fast_kind_id,
            r.category.as_str(),
            r.widths,
            r.operand_forms,
            r.arm64_pattern
        );
    }
    Ok(exit_codes::OK)
}

pub fn cmd_explain(args: &[String]) -> Result<i32> {
    let Some(q) = args.first() else {
        return Err(usage_err("usage: port-factory explain <MNEMONIC_OR_ID>"));
    };
    let mut matches = Vec::new();
    if let Some(r) = lookup_by_id(q) {
        matches.push(r);
    } else {
        matches.extend(lookup_by_mnemonic(q));
    }
    if matches.is_empty() {
        bail!("no x86-64 -> ARM64 op entry found for {q:?}");
    }
    for r in matches {
        println!(
            "=== {} (FastOp K_{}: {}) ===",
            r.id, r.fast_kind_id, r.fast_kind_name
        );
        println!("  Category         : {}", r.category.as_str());
        println!("  Execution Tier   : {}", r.tier.as_str());
        println!("  Sub-op Selector  : {}", r.sub_op);
        println!("  Encodings        : {}", r.encodings);
        println!("  Operand Forms    : {}", r.operand_forms);
        println!("  Bit Widths       : {}", r.widths);
        println!("  x86-64 Mnemonics : {}", r.x86_mnemonics);
        println!("  ARM64 Lowering   : {}", r.arm64_pattern);
        println!("  Semantics        : {}", r.semantics);
        println!("  Implementation   : {}", r.source_file);
        println!("  Verified Vectors : {}", r.verified_vectors);
        println!();
    }
    Ok(exit_codes::OK)
}

pub fn cmd_win32_sync(args: &[String]) -> Result<i32> {
    let mut path = PathBuf::from(DEFAULT_WIN32_TSV);
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--tsv" => path = cur.next_path_or_err("--tsv requires PATH")?,
            _ => return Err(usage_err(format!("unknown argument {a:?}"))),
        }
    }
    let root = workspace_root();
    let db = Win32Database::from_workspace_src(&root)?;
    db.write_tsv(&path)?;

    let mut dlls: BTreeMap<&str, usize> = BTreeMap::new();
    for e in &db.entries {
        *dlls.entry(e.dll.as_str()).or_insert(0) += 1;
    }
    println!(
        "synced {} Win32 API mappings across {} Windows DLLs -> {}",
        db.entries.len(),
        dlls.len(),
        path.display()
    );
    Ok(exit_codes::OK)
}

pub fn cmd_win32_stats(args: &[String]) -> Result<i32> {
    let mut search: Option<String> = None;
    let mut json = false;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--search" => search = Some(cur.next_or_err("--search requires STR")?.to_lowercase()),
            "--json" => json = true,
            _ => return Err(usage_err(format!("unknown argument {a:?}"))),
        }
    }
    let root = workspace_root();
    let db = Win32Database::from_workspace_src(&root)?;

    if json {
        let data = win32_stats_data_json(&db.entries, search.as_deref());
        return Ok(emit_json(&JsonEnvelope::ok("win32-stats", data)));
    }

    let mut by_cat: BTreeMap<Win32Category, usize> = BTreeMap::new();
    let mut by_dll: BTreeMap<(&str, Win32Category), usize> = BTreeMap::new();
    for e in &db.entries {
        *by_cat.entry(e.category).or_insert(0) += 1;
        *by_dll.entry((e.dll.as_str(), e.category)).or_insert(0) += 1;
    }

    println!("=== Wine-Style Core Windows API Mapping Database ===");
    println!("  Total mapped Win32 APIs : {}", db.entries.len());
    println!("  Total Windows DLLs      : {}", by_dll.len());
    println!();
    println!("  {:24} {:>10}", "CATEGORY", "APIS");
    println!("  {}", "-".repeat(38));
    for (cat, count) in &by_cat {
        println!("  {:24} {:>10}", cat.as_str(), count);
    }
    println!();
    println!("  {:34} {:24} {:>8}", "DLL", "CATEGORY", "EXPORTS");
    println!("  {}", "-".repeat(70));
    for ((dll, cat), count) in &by_dll {
        println!("  {:34} {:24} {:>8}", dll, cat.as_str(), count);
    }

    if let Some(q) = search {
        println!();
        println!("  Matching APIs for {:?}:", q);
        for e in &db.entries {
            let hay = format!("{} {} {}", e.dll, e.symbol, e.rust_path).to_lowercase();
            if hay.contains(&q) {
                println!(
                    "    {:24} ! {:36} -> {} ({})",
                    e.dll, e.symbol, e.rust_path, e.source_file
                );
            }
        }
    }
    Ok(exit_codes::OK)
}

pub fn cmd_game_profiles(args: &[String]) -> Result<i32> {
    let root = workspace_root();
    let mut sync = false;
    let mut json = false;
    let mut tsv_path = root.join(DEFAULT_GAME_PROFILES_TSV);
    let mut pe_paths: Vec<PathBuf> = Vec::new();

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--sync" => sync = true,
            "--tsv" => {
                sync = true;
                tsv_path = cur.next_path_or_err("--tsv requires PATH")?;
            }
            "--json" => json = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => pe_paths.push(PathBuf::from(p)),
        }
    }

    let win_db = Win32Database::from_workspace_src(&root)?;
    let report = sync_and_format_game_profiles(sync.then_some(tsv_path.as_path()), &win_db)?;

    let mut detected = Vec::new();
    if !pe_paths.is_empty() {
        let mut files = Vec::new();
        for p in &pe_paths {
            files.extend(collect_pe_files(p)?);
        }
        for f in &files {
            if let Ok(bytes) = std::fs::read(f) {
                if let Ok(pe) = Pe::parse(&bytes) {
                    if let Some(prof) = detect_pe_game_profile(&pe) {
                        detected.push((
                            f.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                            prof.profile_id.to_string(),
                            prof.engine_family.to_string(),
                        ));
                    }
                }
            }
        }
    }

    if json {
        let audits = audit_all_game_profiles(&win_db);
        let synced_str = sync.then(|| tsv_path.display().to_string());
        let data = game_profiles_data_json(&audits, synced_str.as_deref(), &detected);
        return Ok(emit_json(&JsonEnvelope::ok("game-profiles", data)));
    }

    print!("{report}");
    if sync {
        println!(
            "synced {} top game & engine profiles -> {}",
            TOP_GAME_PROFILES.len(),
            tsv_path.display()
        );
    }

    if !pe_paths.is_empty() {
        println!();
        println!("=== Detected Game / Engine Profiles for Target Binaries ===");
        for (fname, profile_id, engine) in &detected {
            println!("  * {:32} -> {} [{}]", fname, profile_id, engine);
        }
    }
    Ok(exit_codes::OK)
}

pub fn cmd_shader_formats(args: &[String]) -> Result<i32> {
    let root = workspace_root();
    let mut sync = false;
    let mut json = false;
    let mut tsv_path = root.join(DEFAULT_SHADER_FORMATS_TSV);
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--sync" => sync = true,
            "--tsv" => {
                sync = true;
                tsv_path = cur.next_path_or_err("--tsv requires PATH")?;
            }
            "--json" => json = true,
            _ => return Err(usage_err(format!("unknown argument {a:?}"))),
        }
    }
    let table = sync_and_format_shader_catalog(sync.then_some(tsv_path.as_path()))?;
    if json {
        let synced_str = sync.then(|| tsv_path.display().to_string());
        let data = shader_formats_data_json(
            dxil_metal::formats::SHADER_FORMAT_CATALOG,
            synced_str.as_deref(),
        );
        return Ok(emit_json(&JsonEnvelope::ok("shader-formats", data)));
    }
    print!("{table}");
    if sync {
        println!(
            "synced {} standardized shader file formats -> {}",
            dxil_metal::formats::SHADER_FORMAT_CATALOG.len(),
            tsv_path.display()
        );
    }
    Ok(exit_codes::OK)
}

pub fn cmd_scaffold_op(args: &[String]) -> Result<i32> {
    let mut mnemonic: Option<String> = None;
    let mut category = "GP_ALU".to_string();
    let mut arm64_pattern = "/* ARM64 instruction pattern */".to_string();
    let mut from_pe: Option<PathBuf> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--category" => {
                category = cur.next_or_err("--category requires CAT")?.to_string()
            }
            "--arm64" => arm64_pattern = cur.next_or_err("--arm64 requires PATTERN")?.to_string(),
            "--from-pe" => from_pe = Some(cur.next_path_or_err("--from-pe requires PE_FILE")?),
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            m => mnemonic = Some(m.to_string()),
        }
    }

    if let Some(pe_path) = from_pe {
        let missing = find_missing_ops_in_pe(&pe_path)?;
        if missing.is_empty() {
            println!(
                "100.00% of instructions in {} are already covered!",
                pe_path.display()
            );
            return Ok(exit_codes::OK);
        }
        println!(
            "=== Untranslated / Non-JIT Ops in {} ({} mnemonics) ===",
            pe_path.display(),
            missing.len()
        );
        for m in &missing {
            println!(
                "  {:16} count={:<6} encodings={:?} sample={:?}",
                m.mnemonic, m.count, m.sample_encodings, m.sample_disasm
            );
        }
        println!();
        for m in missing.iter().take(3) {
            print!(
                "{}",
                generate_op_scaffold(&m.mnemonic, &category, &arm64_pattern)
            );
        }
        return Ok(exit_codes::OK);
    }

    let Some(mn) = mnemonic else {
        return Err(usage_err(
            "usage: port-factory scaffold-op <MNEMONIC> [--category CAT] [--arm64 PATTERN] [--from-pe PE_FILE]",
        ));
    };
    print!("{}", generate_op_scaffold(&mn, &category, &arm64_pattern));
    Ok(exit_codes::OK)
}

pub fn cmd_scaffold_dll(args: &[String]) -> Result<i32> {
    let mut dll_name: Option<String> = None;
    let mut pe_files: Vec<PathBuf> = Vec::new();
    let mut write_to_workspace = false;

    for a in args {
        match a.as_str() {
            "--write" => write_to_workspace = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => {
                if dll_name.is_none() {
                    dll_name = Some(p.to_string());
                } else {
                    pe_files.push(PathBuf::from(p));
                }
            }
        }
    }
    let Some(dll_name) = dll_name else {
        return Err(usage_err(
            "usage: port-factory scaffold-dll <DLL_NAME> [PE_FILES...] [--write]",
        ));
    };

    let root = workspace_root();
    let db = Win32Database::from_workspace_src(&root)?;
    let (stem, src, count) = generate_dll_scaffold(&dll_name, &pe_files, &db)?;

    if write_to_workspace {
        let path = register_misc_dll_module(&root, &stem, &src)?;
        println!(
            "wrote {} exports for {} -> {} (and registered `pub mod {stem};` in misc/mod.rs)",
            count,
            dll_name,
            path.display()
        );
    } else {
        println!("// === Preview: crates/win32-rt/src/misc/{stem}.rs ({count} exports) ===");
        println!(
            "// Pass `--write` to create this file and register `pub mod {stem};` automatically.\n"
        );
        print!("{src}");
    }
    Ok(exit_codes::OK)
}

pub fn cmd_vfs_resolve(args: &[String]) -> Result<i32> {
    let mut drive_c = WinFs::default_translator_root();
    let mut app_dir: Option<PathBuf> = std::env::var_os("PORT_GAME_DIR")
        .or_else(|| std::env::var_os("AW2_GAME_DIR"))
        .filter(|s| !s.is_empty())
        .map(PathBuf::from);
    let mut app_win_dir = "C:\\Games\\App".to_string();
    let mut extra_mounts: Vec<String> = Vec::new();
    let mut queries: Vec<String> = Vec::new();
    let mut json = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--drive-c" | "--translator-home" => {
                drive_c = cur.next_path_or_err("--drive-c requires DIR")?
            }
            "--app-dir" => app_dir = Some(cur.next_path_or_err("--app-dir requires DIR")?),
            "--app-win-dir" => {
                app_win_dir = cur.next_or_err("--app-win-dir requires PATH")?.to_string()
            }
            "--mount" => extra_mounts.push(cur.next_or_err("--mount requires WIN=HOST")?.to_string()),
            "--json" => json = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            q => queries.push(q.to_string()),
        }
    }

    let _ = WinFs::ensure_drive_c_dirs(&drive_c);
    let mut vfs = WinFs::with_standard_layout(
        &drive_c,
        app_dir.as_deref(),
        app_dir.as_ref().map(|_| app_win_dir.as_str()),
    );
    for spec in &extra_mounts {
        vfs.add_mount_spec(spec).map_err(|e| usage_err(e))?;
    }

    if json {
        let mounts: Vec<(String, String, String)> = vfs
            .mounts()
            .iter()
            .map(|m| {
                (
                    m.display.clone(),
                    m.host.display().to_string(),
                    m.description.clone(),
                )
            })
            .collect();
        let resolutions: Vec<(String, String, Option<(String, String, bool)>)> = queries
            .iter()
            .map(|q| {
                let abs = vfs.absolute_win_path(q);
                let resolved = vfs.to_host(q).map(|(host, mount)| {
                    let exists = host.exists();
                    (host.display().to_string(), mount.display.clone(), exists)
                });
                (q.clone(), abs, resolved)
            })
            .collect();
        let data = vfs_resolve_data_json(vfs.cwd(), &mounts, &resolutions);
        return Ok(emit_json(&JsonEnvelope::ok("vfs-resolve", data)));
    }

    println!("=== Windows Virtual Filesystem (VFS) Mount Table ===");
    println!("  Current Working Dir : {}", vfs.cwd());
    for m in vfs.mounts() {
        println!(
            "  {:24} -> {:44} ({})",
            m.display,
            m.host.display(),
            m.description
        );
    }

    if !queries.is_empty() {
        println!();
        println!("=== Path Resolution Results ===");
        for q in &queries {
            let abs = vfs.absolute_win_path(q);
            match vfs.to_host(q) {
                Some((host, mount)) => {
                    let exists = host.exists();
                    println!(
                        "  {:36} -> {} (win={}, mount={}, exists={})",
                        q,
                        host.display(),
                        abs,
                        mount.display,
                        exists
                    );
                }
                None => {
                    println!("  {:36} -> UNMOUNTED (win={})", q, abs);
                }
            }
        }
    }
    Ok(exit_codes::OK)
}

fn parse_doctor_args(args: &[String]) -> Result<(bool, bool, Vec<String>)> {
    let mut fix = false;
    let mut json = false;
    let mut pe_args = Vec::new();
    for a in args {
        match a.as_str() {
            "--check" => fix = false,
            "--fix" | "--sync" => fix = true,
            "--json" => json = true,
            s if s.starts_with('-') => {
                return Err(usage_err(format!(
                    "unknown doctor flag {s:?} (expected --check, --fix, or --json)"
                )));
            }
            p => pe_args.push(p.to_string()),
        }
    }
    Ok((fix, json, pe_args))
}

pub fn cmd_doctor(args: &[String]) -> Result<i32> {
    let (fix, json, pe_args) = parse_doctor_args(args)?;
    let root = workspace_root();

    // 1-4. Verify (or with `--fix`, synchronize) all four canonical `db/*.tsv` databases:
    //      `db/ops_x86_to_arm64.tsv`, `db/win32_apis.tsv`, `db/shader_formats.tsv`,
    //      and `db/game_profiles.tsv`.
    let db_report = check_or_fix_db_tsvs(&root, &root.join("db"), fix)?;

    // 5. Verify standard ~/Translator VFS layout + ShaderCache + Native Pipelines
    let translator_root = WinFs::default_translator_root();
    let _ = WinFs::ensure_drive_c_dirs(&translator_root);
    let vfs = WinFs::with_standard_layout(&translator_root, None, None);
    let shader_cache = WinFs::default_shader_cache_dir();
    let dx12_avail = dxil_metal::converter_available();
    let dx11_avail = dxil_metal::dxilconv::available();
    let vk_spirv_avail = dxil_metal::spirv_metal::metal_toolchain_available();

    if json {
        let pe_audits = if !pe_args.is_empty() {
            Some(scan::run_doctor_pe_audits_json(&pe_args)?)
        } else {
            None
        };
        let data = doctor_data_json(
            fix,
            &db_report,
            &translator_root.display().to_string(),
            vfs.mounts().len(),
            &shader_cache.display().to_string(),
            dx12_avail,
            dx11_avail,
            vk_spirv_avail,
            pe_audits,
        );
        let (code, err_msg) = match db_report.ensure_clean() {
            Ok(()) => (exit_codes::OK, None),
            Err(e) => (exit_codes::VERIFICATION_FAILED, Some(format!("{e:#}"))),
        };
        return Ok(emit_json(&JsonEnvelope::with_status(
            "doctor", code, data, err_msg,
        )));
    }

    println!(
        "=== Translator & Codebase Doctor ({}) ===",
        if fix { "fix mode" } else { "drift check" }
    );
    print!("{}", db_report.format_summary());
    println!(
        "  [OK]      {:16} : {} ({} active Windows folder mounts)",
        "VFS Root Layout",
        translator_root.display(),
        vfs.mounts().len()
    );
    println!(
        "  [OK]      {:16} : cache={} (DX12={}, DX11={}, Vulkan SPIR-V={})",
        "Metal Shaders",
        shader_cache.display(),
        dx12_avail,
        dx11_avail,
        vk_spirv_avail
    );
    println!(
        "  [OK]      {:16} : mscoree.dll + hostfxr.dll + coreclr.dll + mono-2.0-bdwgc.dll + IL2CPP AOT ready",
        ".NET / Mono"
    );

    // 6. Optional PE target scan + Win32 import audit + Anti-Cheat audit + App/GPU audit
    if !pe_args.is_empty() {
        println!();
        let _ = scan::cmd_scan(&pe_args)?;
        let _ = scan::cmd_win32_audit(&pe_args)?;
        let _ = scan::cmd_anticheat_audit(&pe_args)?;
        let _ = scan::cmd_app_audit(&pe_args)?;
        let _ = scan::cmd_dotnet_audit(&pe_args)?;
    }

    if let Err(e) = db_report.ensure_clean() {
        eprintln!("Error: {e:#}");
        return Ok(exit_codes::VERIFICATION_FAILED);
    }
    Ok(exit_codes::OK)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_doctor_args_defaults_to_check_mode_and_supports_fix_and_json() {
        let (fix, json, pe) = parse_doctor_args(&[]).unwrap();
        assert!(!fix);
        assert!(!json);
        assert!(pe.is_empty());

        let (fix, json, pe) =
            parse_doctor_args(&["--check".into(), "--json".into(), "foo.exe".into()]).unwrap();
        assert!(!fix);
        assert!(json);
        assert_eq!(pe, vec!["foo.exe"]);

        let (fix, json, pe) = parse_doctor_args(&["--fix".into(), "dir/a.dll".into()]).unwrap();
        assert!(fix);
        assert!(!json);
        assert_eq!(pe, vec!["dir/a.dll"]);

        let (fix, _, _) = parse_doctor_args(&["--sync".into()]).unwrap();
        assert!(fix);

        assert!(parse_doctor_args(&["--unknown-flag".into()]).is_err());
    }
}
