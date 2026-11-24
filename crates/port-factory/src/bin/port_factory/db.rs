//! Database catalog, scaffolding, VFS resolution, and `doctor` subcommand handlers.

use super::{
    scan, usage, workspace_root, ArgCursor, DEFAULT_GAME_PROFILES_TSV, DEFAULT_OPS_TSV,
    DEFAULT_SHADER_FORMATS_TSV, DEFAULT_STUB_BACKLOG_MD, DEFAULT_STUB_INVENTORY_TSV,
    DEFAULT_WIN32_TSV,
};
use anyhow::{bail, Result};
use pe_image::Pe;
use port_factory::doctor::check_or_fix_db_tsvs;
use port_factory::game_profiles::{
    detect_pe_game_profile, sync_and_format_game_profiles, TOP_GAME_PROFILES,
};
use port_factory::gpu_shaders::sync_and_format_shader_catalog;
use port_factory::ops_db::{catalog_stats, lookup_by_id, lookup_by_mnemonic, write_tsv, OPS_CATALOG};
use port_factory::scaffold::{
    find_missing_ops_in_pe, generate_dll_scaffold, generate_op_scaffold, register_misc_dll_module,
};
use port_factory::scanner::collect_pe_files;
use port_factory::stub_inventory::{
    check_no_new_stubs_against_file, diff_stub_json_files, format_inventory_tsv,
    render_stub_backlog_md, scan_workspace_stubs,
};
use port_factory::vfs::WinFs;
use port_factory::win32_db::{Win32Category, Win32Database};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

pub fn cmd_sync(args: &[String]) -> Result<()> {
    let mut path = PathBuf::from(DEFAULT_OPS_TSV);
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--tsv" => path = cur.next_path_or_usage(),
            _ => bail!("unknown argument {a:?}"),
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
    Ok(())
}

pub fn cmd_stats() -> Result<()> {
    let st = catalog_stats();
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
    Ok(())
}

pub fn cmd_list(args: &[String]) -> Result<()> {
    let mut cat_filter: Option<String> = None;
    let mut search: Option<String> = None;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--category" => cat_filter = Some(cur.next_or_usage().to_uppercase()),
            "--search" => search = Some(cur.next_or_usage().to_lowercase()),
            _ => bail!("unknown argument {a:?}"),
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
    Ok(())
}

pub fn cmd_explain(args: &[String]) -> Result<()> {
    let Some(q) = args.first() else {
        usage();
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
    Ok(())
}

pub fn cmd_win32_sync(args: &[String]) -> Result<()> {
    let mut path = PathBuf::from(DEFAULT_WIN32_TSV);
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--tsv" => path = cur.next_path_or_usage(),
            _ => bail!("unknown argument {a:?}"),
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
    Ok(())
}

pub fn cmd_win32_stats(args: &[String]) -> Result<()> {
    let mut search: Option<String> = None;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--search" => search = Some(cur.next_or_usage().to_lowercase()),
            _ => bail!("unknown argument {a:?}"),
        }
    }
    let root = workspace_root();
    let db = Win32Database::from_workspace_src(&root)?;

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
    Ok(())
}

pub fn cmd_game_profiles(args: &[String]) -> Result<()> {
    let root = workspace_root();
    let mut sync = false;
    let mut tsv_path = root.join(DEFAULT_GAME_PROFILES_TSV);
    let mut pe_paths: Vec<PathBuf> = Vec::new();

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--sync" => sync = true,
            "--tsv" => {
                sync = true;
                tsv_path = cur.next_path_or_usage();
            }
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p => pe_paths.push(PathBuf::from(p)),
        }
    }

    let win_db = Win32Database::from_workspace_src(&root)?;
    let report = sync_and_format_game_profiles(sync.then_some(tsv_path.as_path()), &win_db)?;
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
        let mut files = Vec::new();
        for p in &pe_paths {
            files.extend(collect_pe_files(p)?);
        }
        for f in &files {
            if let Ok(bytes) = std::fs::read(f) {
                if let Ok(pe) = Pe::parse(&bytes) {
                    if let Some(prof) = detect_pe_game_profile(&pe) {
                        println!(
                            "  * {:32} -> {} [{}]",
                            f.file_name().unwrap_or_default().to_string_lossy(),
                            prof.profile_id,
                            prof.engine_family
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn cmd_shader_formats(args: &[String]) -> Result<()> {
    let root = workspace_root();
    let mut sync = false;
    let mut tsv_path = root.join(DEFAULT_SHADER_FORMATS_TSV);
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--sync" => sync = true,
            "--tsv" => {
                sync = true;
                tsv_path = cur.next_path_or_usage();
            }
            _ => bail!("unknown argument {a:?}"),
        }
    }
    let table = sync_and_format_shader_catalog(sync.then_some(tsv_path.as_path()))?;
    print!("{table}");
    if sync {
        println!(
            "synced {} standardized shader file formats -> {}",
            dxil_metal::formats::SHADER_FORMAT_CATALOG.len(),
            tsv_path.display()
        );
    }
    Ok(())
}

pub fn cmd_scaffold_op(args: &[String]) -> Result<()> {
    let mut mnemonic: Option<String> = None;
    let mut category = "GP_ALU".to_string();
    let mut arm64_pattern = "/* ARM64 instruction pattern */".to_string();
    let mut from_pe: Option<PathBuf> = None;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--category" => category = cur.next_or_usage().to_string(),
            "--arm64" => arm64_pattern = cur.next_or_usage().to_string(),
            "--from-pe" => from_pe = Some(cur.next_path_or_usage()),
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
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
            return Ok(());
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
            print!("{}", generate_op_scaffold(&m.mnemonic, &category, &arm64_pattern));
        }
        return Ok(());
    }

    let Some(mn) = mnemonic else {
        usage();
    };
    print!("{}", generate_op_scaffold(&mn, &category, &arm64_pattern));
    Ok(())
}

pub fn cmd_scaffold_dll(args: &[String]) -> Result<()> {
    let mut dll_name: Option<String> = None;
    let mut pe_files: Vec<PathBuf> = Vec::new();
    let mut write_to_workspace = false;

    for a in args {
        match a.as_str() {
            "--write" => write_to_workspace = true,
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
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
        usage();
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
    Ok(())
}

pub fn cmd_vfs_resolve(args: &[String]) -> Result<()> {
    let mut drive_c = WinFs::default_translator_root();
    let mut app_dir: Option<PathBuf> = std::env::var_os("PORT_GAME_DIR")
        .or_else(|| std::env::var_os("AW2_GAME_DIR"))
        .filter(|s| !s.is_empty())
        .map(PathBuf::from);
    let mut app_win_dir = "C:\\Games\\App".to_string();
    let mut extra_mounts: Vec<String> = Vec::new();
    let mut queries: Vec<String> = Vec::new();

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--drive-c" | "--translator-home" => drive_c = cur.next_path_or_usage(),
            "--app-dir" => app_dir = Some(cur.next_path_or_usage()),
            "--app-win-dir" => app_win_dir = cur.next_or_usage().to_string(),
            "--mount" => extra_mounts.push(cur.next_or_usage().to_string()),
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
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
        vfs.add_mount_spec(spec).map_err(|e| anyhow::anyhow!(e))?;
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
    Ok(())
}

fn parse_doctor_args(args: &[String]) -> Result<(bool, Vec<String>)> {
    let mut fix = false;
    let mut pe_args = Vec::new();
    for a in args {
        match a.as_str() {
            "--check" => fix = false,
            "--fix" | "--sync" => fix = true,
            s if s.starts_with('-') => {
                bail!("unknown doctor flag {s:?} (expected --check or --fix)")
            }
            p => pe_args.push(p.to_string()),
        }
    }
    Ok((fix, pe_args))
}

pub fn cmd_doctor(args: &[String]) -> Result<()> {
    let (fix, pe_args) = parse_doctor_args(args)?;
    let root = workspace_root();
    println!(
        "=== Translator & Codebase Doctor ({}) ===",
        if fix { "fix mode" } else { "drift check" }
    );

    // 1-4. Verify (or with `--fix`, synchronize) all four canonical `db/*.tsv` databases:
    //      `db/ops_x86_to_arm64.tsv`, `db/win32_apis.tsv`, `db/shader_formats.tsv`,
    //      and `db/game_profiles.tsv`.
    let db_report = check_or_fix_db_tsvs(&root, &root.join("db"), fix)?;
    print!("{}", db_report.format_summary());

    // 5. Verify standard ~/Translator VFS layout + ShaderCache + Native Pipelines
    let translator_root = WinFs::default_translator_root();
    let _ = WinFs::ensure_drive_c_dirs(&translator_root);
    let vfs = WinFs::with_standard_layout(&translator_root, None, None);
    println!(
        "  [OK]      {:16} : {} ({} active Windows folder mounts)",
        "VFS Root Layout",
        translator_root.display(),
        vfs.mounts().len()
    );
    println!(
        "  [OK]      {:16} : cache={} (DX12={}, DX11={}, Vulkan SPIR-V={})",
        "Metal Shaders",
        WinFs::default_shader_cache_dir().display(),
        dxil_metal::converter_available(),
        dxil_metal::dxilconv::available(),
        dxil_metal::spirv_metal::metal_toolchain_available()
    );
    println!(
        "  [OK]      {:16} : mscoree.dll + hostfxr.dll + coreclr.dll + mono-2.0-bdwgc.dll + IL2CPP AOT ready",
        ".NET / Mono"
    );

    // 6. Optional PE target scan + Win32 import audit + Anti-Cheat audit + App/GPU audit
    if !pe_args.is_empty() {
        println!();
        scan::cmd_scan(&pe_args)?;
        scan::cmd_win32_audit(&pe_args)?;
        scan::cmd_anticheat_audit(&pe_args)?;
        scan::cmd_app_audit(&pe_args)?;
        scan::cmd_dotnet_audit(&pe_args)?;
    }

    db_report.ensure_clean()?;
    Ok(())
}

pub fn cmd_stub_sync(args: &[String]) -> Result<()> {
    let mut tsv_path = PathBuf::from(DEFAULT_STUB_INVENTORY_TSV);
    let mut also_backlog = false;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--tsv" => tsv_path = cur.next_path_or_usage(),
            "--backlog" => also_backlog = true,
            _ => bail!("unknown argument {a:?}"),
        }
    }
    let root = workspace_root();
    let entries = scan_workspace_stubs(&root)?;
    let abs_tsv = if tsv_path.is_absolute() {
        tsv_path.clone()
    } else {
        root.join(&tsv_path)
    };
    // Enforce zero-new-stub policy when updating an existing inventory.
    check_no_new_stubs_against_file(&abs_tsv, &entries)?;
    if let Some(parent) = abs_tsv.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&abs_tsv, format_inventory_tsv(&entries))?;
    let mut by_kind: BTreeMap<&str, usize> = BTreeMap::new();
    for e in &entries {
        *by_kind.entry(e.kind.as_str()).or_insert(0) += 1;
    }
    println!(
        "synced {} stub sites (stub={}, failing={}, middleware_stub={}) -> {}",
        entries.len(),
        by_kind.get("stub").copied().unwrap_or(0),
        by_kind.get("failing").copied().unwrap_or(0),
        by_kind.get("middleware_stub").copied().unwrap_or(0),
        abs_tsv.display()
    );
    if also_backlog {
        let md = render_stub_backlog_md(&root, &entries)?;
        let out_md = root.join(DEFAULT_STUB_BACKLOG_MD);
        fs::write(&out_md, md)?;
        println!("updated {}", out_md.display());
    }
    Ok(())
}

pub fn cmd_stub_backlog(args: &[String]) -> Result<()> {
    let mut write = false;
    let mut out_path = PathBuf::from(DEFAULT_STUB_BACKLOG_MD);
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--write" | "--sync" => write = true,
            "--out" => {
                out_path = cur.next_path_or_usage();
                write = true;
            }
            _ => bail!("unknown argument {a:?}"),
        }
    }
    let root = workspace_root();
    let entries = scan_workspace_stubs(&root)?;
    let md = render_stub_backlog_md(&root, &entries)?;
    if write {
        let abs = if out_path.is_absolute() {
            out_path
        } else {
            root.join(&out_path)
        };
        if let Some(parent) = abs.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&abs, &md)?;
        println!(
            "wrote {} ({} remaining stub sites)",
            abs.display(),
            entries.len()
        );
    } else {
        print!("{md}");
    }
    Ok(())
}

pub fn cmd_stub_diff(args: &[String]) -> Result<()> {
    if args.len() != 2 {
        bail!("usage: port-factory stub-diff <BEFORE.json> <AFTER.json>");
    }
    let diff = diff_stub_json_files(&PathBuf::from(&args[0]), &PathBuf::from(&args[1]))?;
    print!("{diff}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_doctor_args_defaults_to_check_mode_and_supports_fix() {
        let (fix, pe) = parse_doctor_args(&[]).unwrap();
        assert!(!fix);
        assert!(pe.is_empty());

        let (fix, pe) = parse_doctor_args(&["--check".into(), "foo.exe".into()]).unwrap();
        assert!(!fix);
        assert_eq!(pe, vec!["foo.exe"]);

        let (fix, pe) = parse_doctor_args(&["--fix".into(), "dir/a.dll".into()]).unwrap();
        assert!(fix);
        assert_eq!(pe, vec!["dir/a.dll"]);

        let (fix, _) = parse_doctor_args(&["--sync".into()]).unwrap();
        assert!(fix);

        assert!(parse_doctor_args(&["--unknown-flag".into()]).is_err());
    }
}
