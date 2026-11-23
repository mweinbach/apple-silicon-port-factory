//! CLI tool (`port-factory` / `cargo opsdb`) for:
//! 1. The `x86-64 -> ARM64` operation translation database (`db/ops_x86_to_arm64.tsv`)
//! 2. The Wine-style Core Windows API mapping database (`db/win32_apis.tsv`)
//! 3. Multi-game / library PE binary compatibility & Win32 import auditing
//! 4. The Generalized Static AOT Compiler (`x86-64 PE -> Native ARM64 Rust`)
//! 5. Scaffolding new ops (`scaffold-op`), new DLL builtins (`scaffold-dll`),
//!    embeddable AOT Rust crates (`aot-crate`), and Windows VFS path routing (`vfs-resolve`)

use anyhow::{bail, Context, Result};
use pe_image::{ExportTarget, Pe};
use port_factory::anticheat::{audit_pe_anticheat, format_anticheat_report};
use port_factory::aot::{compile_pe_file_to_rust, AotOptions};
use port_factory::diag_toolkit::{
    audit_pe_runtime_hazards, find_or_build_x86_verify, format_hazard_reports,
    resolve_debug_script, NanTraceSpec,
};
use port_factory::doctor::check_or_fix_db_tsvs;
use port_factory::dotnet::{audit_dotnet_paths, strip_ready_to_run_file, DotNetRuntimeKind};
use port_factory::game_profiles::{
    detect_pe_game_profile, sync_and_format_game_profiles, TOP_GAME_PROFILES,
};
use port_factory::gpu_shaders::{
    audit_pe_app_capabilities, convert_shader_file, format_app_capability_report,
    inspect_shader_file, precompile_shaders_for_paths, sync_and_format_shader_catalog,
};
use port_factory::ios_bundle::{
    build_ios_bundle, format_ios_bundle_verify_report, verify_ios_bundle, IosBundleOptions,
    IosTarget,
};
use port_factory::launcher::{build_launcher_app_bundle, run_launcher, LauncherOptions};
use port_factory::ops_db::{catalog_stats, lookup_by_id, lookup_by_mnemonic, write_tsv, OPS_CATALOG};
use port_factory::scaffold::{
    find_missing_ops_in_pe, generate_dll_scaffold, generate_embeddable_aot_crate,
    generate_op_scaffold, register_misc_dll_module,
};
use port_factory::scanner::{collect_pe_files, format_profile_report, scan_pe_file, write_profiles_tsv};
use port_factory::steam_library::{
    apply_post_install, convert_steam_game, default_apps_dir, default_steam_downloads_dir,
    download_steam_game, export_app_bundle, find_depot_downloader, format_bundle_verify_report,
    format_steam_scan_report, install_depot_downloader, list_external_volumes,
    list_packaged_bundles, port_steam_game, resolve_steam_target_app_id, sanitize_game_title,
    scan_all_steam_libraries, verify_app_bundle, write_steam_browser_html, ConversionFormat,
    SteamConvertOptions, SteamDownloadOptions, SteamPortOptions,
};
use port_factory::storage_opt::{
    format_storage_opt_report, optimize_game_assets, parse_manifest_storage_config,
    StorageOptOptions, StorageOptProfile,
};
use port_factory::vfs::WinFs;
use port_factory::win32_db::{audit_pe_imports, format_audit_report, Win32Category, Win32Database};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

const DEFAULT_OPS_TSV: &str = "db/ops_x86_to_arm64.tsv";
const DEFAULT_PROFILES_TSV: &str = "db/ops_profiles.tsv";
const DEFAULT_WIN32_TSV: &str = "db/win32_apis.tsv";
const DEFAULT_SHADER_FORMATS_TSV: &str = "db/shader_formats.tsv";
const DEFAULT_GAME_PROFILES_TSV: &str = "db/game_profiles.tsv";
const DEFAULT_STEAM_BROWSER_HTML: &str = "out/steam_library_browser.html";

fn usage() -> ! {
    eprintln!(
        "usage: port-factory <command> [args...]\n\
         \n\
         x86-64 -> ARM64 Ops Database Commands:\n\
           sync [--tsv PATH]                    Export canonical x86-64 -> ARM64 ops catalog to TSV\n\
           stats                                Print summary statistics of the x86-64 -> ARM64 database\n\
           list [--category CAT] [--search STR] List translated op families and their ARM64 patterns\n\
           explain <MNEMONIC_OR_ID>             Show full x86-64 -> ARM64 translation recipe & semantics\n\
           scan <PE_FILE_OR_DIR...> [--record] [--top N]\n\
                                                Scan .exe/.dll binaries for ARM64 coverage & op mix\n\
         \n\
         Wine-Style Core Windows API Database Commands:\n\
           win32-sync [--tsv PATH]              Export canonical Win32 DLL & API mapping catalog to TSV\n\
           win32-stats [--search STR]           Print summary statistics of mapped Windows DLLs & APIs\n\
           win32-audit <PE_FILE_OR_DIR...>      Audit PE imports against the Win32 API database\n\
         \n\
         Generalized Static AOT & Embedding Commands:\n\
           aot <PE_FILE> [--out PATH.rs] [--max-fns N]\n\
                                                Statically AOT-compile an x86-64 .exe/.dll to native Rust\n\
           aot-crate <PE_FILE> --out <DIR> [--name CRATE]\n\
                                                Generate a standalone, zero-dependency embeddable Rust crate\n\
           aot-prewarm <PE_FILE> [--game-dir DIR] [--aot-dir DIR] [--max-funcs N]\n\
                                                Offline-precompile relocatable ARM64 .aot block caches via port-runner\n\
           aot-static-obj <PE_OR_AOT> --out <DIR_OR_O> [--platform ios|ios-sim|macos] [--game-dir DIR] [--max-funcs N]\n\
                                                Compile PE or v5 .aot into Apple-signed ARM64 Mach-O .aot.o object(s) (zero MAP_JIT)\n\
           ios-bundle <PE_OR_DIR_OR_TOML> --out <PATH.app|PATH.ipa> [--target device|sim] [--title NAME] [--bundle-id ID] [--exe NAME] [--manifest FILE] [--release] [--deployment-target 17.0] [--no-static-aot] [--aot-max-funcs N] [--extra-aot-obj FILE.o] [--no-precompile-shaders] [--bundle-shader-cache [DIR]] [--optimize-storage none|lossless|high|compact] [--hevc-videos] [--no-hevc-videos] [--video-max-height N] [--video-bitrate KBPS] [--max-tex-dim N] [--strip-unused-dlls] [--keep-lang CODE] [--strip-dlc] [--opt-jobs N] [--opt-dry-run] [--sign ID] [--provisioning-profile FILE] [--xcode-project]\n\
                                                One-command iOS / iOS Simulator .app or .ipa packager with signed static .aot.o, Tier-1 shaders & optional storage optimization\n\
           ios-verify <PATH.app|PATH.ipa>       Verify an iOS .app bundle or .ipa archive (ARM64 iOS Mach-O, static AOT, shaders, signature)\n\
           optimize-assets <SRC_DIR> --out <DST_DIR> [--profile lossless|high|compact] [--manifest FILE] [--hevc-videos] [--no-hevc-videos] [--video-max-height N] [--video-bitrate KBPS] [--max-tex-dim N] [--strip-unused-dlls] [--keep-lang CODE] [--strip-dlc] [--jobs N] [--dry-run]\n\
                                                Optimize any Windows game directory for Apple Silicon / iOS storage (HEVC Bink2, DDS mip cap, DLL strip, game repacker)\n\
         \n\
         Steam & PC Game Library Launcher, Downloader & One-Click .app Converter:\n\
           launcher [--port PORT] [--no-open] [--workspace DIR]\n\
                                                Start interactive local Launcher UI (manage Steam, convert .app, copy to SSD)\n\
           launcher-app [--out PATH.app]        Build double-clickable `out/apps/Port Factory.app` launcher bundle\n\
           steam-setup [--dir DIR]              Install/verify native Apple Silicon SteamRE/DepotDownloader\n\
           steam-scan [--html PATH] [--open]    Scan macOS Steam, CrossOver Steam, Heroic & owned library\n\
           steam-download <APPID_OR_NAME> [--dir DIR] [--qr | --user ACCOUNT] [--validate] [--branch BRANCH] [--dry-run]\n\
                                                Download or repair a Windows x64 Steam depot & run post-install setup\n\
           steam-postinstall <GAME_DIR> [--appid ID] [--out DIR]\n\
                                                Extract installscript.vdf registry & satisfy _CommonRedist\n\
           steam-convert <APPID_OR_NAME_OR_DIR> [--format app|aot-crate|shaders] [--out PATH] [--max-fns N] [--bundle-shader-cache [DIR]]\n\
                                                One-click convert a Steam game or steam.exe to macOS .app / AOT\n\
           steam-port <APPID_OR_NAME> [--qr | --user ACCOUNT] [--validate] [--skip-download] [--out PATH.app] [--copy-to /Volumes/SSD] [--dry-run]\n\
                                                Full end-to-end pipeline: Steam download -> .app bundle -> verify -> optional SSD copy\n\
           bundles                              List packaged .app bundles and mounted external SSDs\n\
           bundle-verify <PATH.app>             Verify an .app bundle is self-contained and portable to another Apple Silicon Mac\n\
           bundle-export <PATH.app> <DEST> [--force]\n\
                                                Verify and copy an .app bundle to an external SSD or target folder\n\
         \n\
         Top Games, Non-Gaming App, GPU Acceleration, Shader & .NET / CLR Commands:\n\
           game-profiles [--sync] [PE_FILES...] List & audit top PC games & engine profiles (UE5, Source 2, RE, etc.)\n\
           app-audit <PE_FILE_OR_DIR...>        Audit desktop/CLI/IPC, GPU backends & embedded shaders\n\
           dotnet-audit <PE_FILE_OR_DIR...> [--strip-r2r OUT_FILE]\n\
                                                Audit .NET CLR / CoreCLR / ReadyToRun / Unity Mono / IL2CPP\n\
           shader-formats [--sync]              List standardized DX12/DX11/Vulkan/Metal shader file formats\n\
           shader-inspect <FILE_OR_DIR...>      Auto-detect & inspect raw shader files (.dxil/.dxbc/.spv/.wgsl)\n\
           shader-convert <INPUT> [--out OUT.metallib] [--msl-out OUT.metal] [--entry NAME]\n\
                                                Convert raw DX12/DX11/Vulkan/WGSL shader file to Metal .metallib\n\
           precompile-shaders <FILE_OR_DIR...> [--cache DIR] [--limit N]\n\
                                                AOT extract & convert DX12/DX11/Vulkan shaders to Metal .metallib\n\
         \n\
         Extensibility & Scaffolding Commands (Add New Ops / DLLs / VFS / Anti-Cheat):\n\
           scaffold-op <MNEMONIC> [--category CAT] [--arm64 PATTERN] [--from-pe PE_FILE]\n\
                                                Generate ready-to-paste OpRecord & AOT lowering code\n\
           scaffold-dll <DLL_NAME> [PE_FILES...] [--write]\n\
                                                Generate a new Win32/middleware DLL builtin module\n\
           vfs-resolve <WIN_PATH...> [--drive-c DIR] [--app-dir DIR] [--mount WIN=HOST]\n\
                                                Test & inspect Windows VFS path/env/mount resolution\n\
           anticheat-audit <PE_FILE_OR_DIR...>  Audit direct syscalls, VEH traps, entropy & anti-debug checks\n\
           doctor [--check|--fix] [PE_FILE_OR_DIR...]\n\
                                                Verify db/*.tsv drift (exits non-zero on mismatch; --fix syncs), VFS & pipelines\n\
         \n\
         Diagnostic & Verification Toolkit (Any Windows Game / DLL):\n\
           runtime-audit <PE_FILE_OR_DIR...>    Static audit for MXCSR/FPCR, x87, AVX narrowing, approx-FP, lock-RMW, WaitOnAddress & RWX hazards\n\
           verify-x86 <PE_FILE_OR_DIR...> [--oracle] [--oracle-sample N] [--rebuild] [x86-verify flags...]\n\
                                                Offline 4-tier differential verifier (Generic vs FastOp vs JIT vs AOT + linked CFG + optional Rosetta 2 oracle)\n\
           oracle <SNAP_OR_DIR...> [--max N] [--mxcsr HEX] [--quiet-matches]\n\
                                                Replay PORT_NAN_TRACE / x86-verify snapshots on real Rosetta 2 x86-64 hardware\n\
           bisect-runtime --cmd \"<REPRO>\" [--bisect-range MOD:LO..HI] [bisect flags...]\n\
                                                Run execution-mode knob matrix & O(log N) PORT_AOT_SKIP/PORT_JIT_SKIP binary search\n\
           live-probe --manifest TOML|--pe EXE [--game-dir DIR] [--script FILE] [--step CMD...] [--repeat N]\n\
                                                Frame-paced PORT_CONTROL_DIR automation, PNG capture & pass/fail repro harness\n\
           nan-trace <PE_OR_TOML> [--game-dir DIR] [--module MOD] [--watch LO..HI] [--dump-dir DIR] [--log FILE] [--armed] [--run-oracle] [-- <game-args...>]\n\
                                                Launch port-runner pre-configured with PORT_NAN_TRACE + PORT_NAN_BOUNDARY (and optional Rosetta oracle)\n"
    );
    std::process::exit(2);
}

fn workspace_root() -> PathBuf {
    if let Some(ws) = std::env::var_os("PORT_FACTORY_WORKSPACE").filter(|v| !v.is_empty()) {
        return PathBuf::from(ws);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|s| s == "--") {
        args.remove(0);
    }
    let Some(cmd) = args.first().map(|s| s.as_str()) else {
        usage();
    };
    match cmd {
        "sync" => cmd_sync(&args[1..]),
        "stats" => cmd_stats(),
        "list" => cmd_list(&args[1..]),
        "explain" => cmd_explain(&args[1..]),
        "scan" => cmd_scan(&args[1..]),
        "win32-sync" => cmd_win32_sync(&args[1..]),
        "win32-stats" => cmd_win32_stats(&args[1..]),
        "win32-audit" => cmd_win32_audit(&args[1..]),
        "aot" => cmd_aot(&args[1..]),
        "aot-crate" => cmd_aot_crate(&args[1..]),
        "aot-prewarm" | "prewarm-aot" => cmd_aot_prewarm(&args[1..]),
        "aot-static-obj" | "static-aot-obj" => cmd_aot_static_obj(&args[1..]),
        "ios-bundle" | "ios-package" | "ios-ipa" => cmd_ios_bundle(&args[1..]),
        "ios-verify" => cmd_ios_verify(&args[1..]),
        "optimize-assets" | "storage-opt" | "opt-assets" => cmd_optimize_assets(&args[1..]),
        "launcher" | "ui" | "gui" => cmd_launcher(&args[1..]),
        "launcher-app" => cmd_launcher_app(&args[1..]),
        "steam-setup" => cmd_steam_setup(&args[1..]),
        "steam-scan" | "steam-library" | "steam-browser" => cmd_steam_scan(&args[1..]),
        "steam-download" | "steam-update" => cmd_steam_download(&args[1..]),
        "steam-postinstall" => cmd_steam_postinstall(&args[1..]),
        "steam-convert" => cmd_steam_convert(&args[1..]),
        "steam-port" | "port" => cmd_steam_port(&args[1..]),
        "bundles" | "bundle-list" => cmd_bundles(),
        "bundle-verify" | "verify-bundle" => cmd_bundle_verify(&args[1..]),
        "bundle-export" | "export-bundle" => cmd_bundle_export(&args[1..]),
        "game-profiles" | "games-audit" => cmd_game_profiles(&args[1..]),
        "app-audit" | "gpu-audit" => cmd_app_audit(&args[1..]),
        "dotnet-audit" | "clr-audit" => cmd_dotnet_audit(&args[1..]),
        "shader-formats" => cmd_shader_formats(&args[1..]),
        "shader-inspect" => cmd_shader_inspect(&args[1..]),
        "shader-convert" => cmd_shader_convert(&args[1..]),
        "precompile-shaders" | "shader-precompile" => cmd_precompile_shaders(&args[1..]),
        "scaffold-op" => cmd_scaffold_op(&args[1..]),
        "scaffold-dll" => cmd_scaffold_dll(&args[1..]),
        "vfs-resolve" => cmd_vfs_resolve(&args[1..]),
        "anticheat-audit" => cmd_anticheat_audit(&args[1..]),
        "doctor" => cmd_doctor(&args[1..]),
        "runtime-audit" | "mxcsr-audit" | "hazard-audit" => cmd_runtime_audit(&args[1..]),
        "verify-x86" | "x86-verify" => cmd_verify_x86(&args[1..]),
        "oracle" | "x86-oracle" => cmd_oracle(&args[1..]),
        "bisect-runtime" | "bisect" => cmd_bisect_runtime(&args[1..]),
        "live-probe" | "probe" => cmd_live_probe(&args[1..]),
        "nan-trace" => cmd_nan_trace(&args[1..]),
        "-h" | "--help" | "help" => usage(),
        other => bail!("unknown subcommand {other:?}"),
    }
}

fn cmd_sync(args: &[String]) -> Result<()> {
    let mut path = PathBuf::from(DEFAULT_OPS_TSV);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--tsv" => {
                path = PathBuf::from(it.next().unwrap_or_else(|| usage()));
            }
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

fn cmd_stats() -> Result<()> {
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

fn cmd_list(args: &[String]) -> Result<()> {
    let mut cat_filter: Option<String> = None;
    let mut search: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--category" => cat_filter = Some(it.next().unwrap_or_else(|| usage()).to_uppercase()),
            "--search" => search = Some(it.next().unwrap_or_else(|| usage()).to_lowercase()),
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

fn cmd_explain(args: &[String]) -> Result<()> {
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

fn cmd_scan(args: &[String]) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut record = false;
    let mut top_n = 8usize;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--record" => record = true,
            "--top" => {
                top_n = it
                    .next()
                    .unwrap_or_else(|| usage())
                    .parse()
                    .unwrap_or(8);
            }
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

fn cmd_win32_sync(args: &[String]) -> Result<()> {
    let mut path = PathBuf::from(DEFAULT_WIN32_TSV);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--tsv" => {
                path = PathBuf::from(it.next().unwrap_or_else(|| usage()));
            }
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

fn cmd_win32_stats(args: &[String]) -> Result<()> {
    let mut search: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--search" => search = Some(it.next().unwrap_or_else(|| usage()).to_lowercase()),
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

fn cmd_win32_audit(args: &[String]) -> Result<()> {
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

fn cmd_aot(args: &[String]) -> Result<()> {
    let mut pe_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut opts = AotOptions::default();

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" | "-o" => {
                out_path = Some(PathBuf::from(it.next().unwrap_or_else(|| usage())));
            }
            "--max-fns" => {
                opts.max_functions = it
                    .next()
                    .unwrap_or_else(|| usage())
                    .parse()
                    .context("--max-fns")?;
            }
            "--no-win64-attr" => {
                opts.emit_win64_abi_attr = false;
            }
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

fn cmd_aot_crate(args: &[String]) -> Result<()> {
    let mut pe_path: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut crate_name: Option<String> = None;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" | "-o" => {
                out_dir = Some(PathBuf::from(it.next().unwrap_or_else(|| usage())));
            }
            "--name" => {
                crate_name = Some(it.next().unwrap_or_else(|| usage()).clone());
            }
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

    println!("=== Generated Embeddable AOT Rust Crate: {} ===", summary.crate_name);
    println!("  Output Directory      : {}", summary.out_dir.display());
    println!("  Compiled Functions    : {} ({} exported)", summary.function_count, summary.export_count);
    println!(
        "  Instruction Lowering  : {}/{} (100% standalone Rust)",
        summary.lowered_instructions, summary.instruction_count
    );
    println!("  Embedded PE Image     : {} bytes (src/sections.bin)", summary.image_bytes);
    println!("  Usage in Host Rust    : `{}::init_embedded_module();` then call any export!", summary.crate_name.replace('-', "_"));
    Ok(())
}

fn find_or_build_port_runner(workspace_root: &Path) -> Result<PathBuf> {
    if let Some(p) = std::env::var_os("PORT_RUNNER").filter(|v| !v.is_empty()) {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Ok(pb);
        }
    }
    let mut candidates = vec![
        workspace_root.join("target/aarch64-apple-darwin/release/port-runner"),
        workspace_root.join("target/release/port-runner"),
        workspace_root.join("target/aarch64-apple-darwin/debug/port-runner"),
        workspace_root.join("target/debug/port-runner"),
    ];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("port-runner"));
        }
    }
    let mut existing: Vec<(PathBuf, std::time::SystemTime)> = candidates
        .into_iter()
        .filter(|p| p.is_file())
        .map(|p| {
            let mt = std::fs::metadata(&p)
                .and_then(|m| m.modified())
                .unwrap_or(std::time::UNIX_EPOCH);
            (p, mt)
        })
        .collect();
    if !existing.is_empty() {
        existing.sort_by_key(|(_, mt)| std::cmp::Reverse(*mt));
        return Ok(existing.remove(0).0);
    }

    let status = std::process::Command::new("cargo")
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
        .context("building port-runner via cargo")?;
    if !status.success() {
        bail!("failed to build port-runner (cargo exited with {status})");
    }
    for rel in [
        "target/aarch64-apple-darwin/debug/port-runner",
        "target/debug/port-runner",
    ] {
        let p = workspace_root.join(rel);
        if p.is_file() {
            return Ok(p);
        }
    }
    bail!("port-runner binary not found after cargo build")
}

fn cmd_aot_prewarm(args: &[String]) -> Result<()> {
    let mut pe_path: Option<PathBuf> = None;
    let mut game_dir: Option<PathBuf> = None;
    let mut aot_dir: Option<PathBuf> = None;
    let mut max_funcs: Option<usize> = None;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--game-dir" => {
                game_dir = Some(PathBuf::from(it.next().unwrap_or_else(|| usage())));
            }
            "--aot-dir" => {
                aot_dir = Some(PathBuf::from(it.next().unwrap_or_else(|| usage())));
            }
            "--max-funcs" | "--aot-max-funcs" => {
                let raw = it.next().unwrap_or_else(|| usage());
                max_funcs = Some(raw.parse().with_context(|| format!("invalid --max-funcs {raw:?}"))?);
            }
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if pe_path.is_none() => pe_path = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
    }
    let Some(pe_path) = pe_path else {
        usage();
    };
    if !pe_path.is_file() {
        bail!("PE file not found: {}", pe_path.display());
    }

    let runner = find_or_build_port_runner(&workspace_root())?;
    let mut cmd = std::process::Command::new(&runner);
    if let Some(ref gd) = game_dir {
        cmd.arg("--game-dir").arg(gd);
    }
    cmd.arg("--pe").arg(&pe_path);
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

fn cmd_aot_static_obj(args: &[String]) -> Result<()> {
    let mut input: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut platform = "ios".to_string();
    let mut game_dir: Option<PathBuf> = None;
    let mut max_funcs: Option<usize> = None;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" | "-o" => {
                out_path = Some(PathBuf::from(it.next().unwrap_or_else(|| usage())));
            }
            "--platform" | "--static-platform" => {
                platform = it.next().unwrap_or_else(|| usage()).clone();
            }
            "--game-dir" => {
                game_dir = Some(PathBuf::from(it.next().unwrap_or_else(|| usage())));
            }
            "--max-funcs" | "--aot-max-funcs" => {
                let raw = it.next().unwrap_or_else(|| usage());
                max_funcs = Some(raw.parse().with_context(|| format!("invalid --max-funcs {raw:?}"))?);
            }
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if input.is_none() => input = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
    }
    let Some(input) = input else {
        usage();
    };
    if !input.is_file() {
        bail!("input file not found: {}", input.display());
    }
    let is_aot = input
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("aot"));
    let out_target = out_path.unwrap_or_else(|| {
        if is_aot {
            input.with_extension("aot.o")
        } else {
            PathBuf::from("out/static-aot")
        }
    });

    let runner = find_or_build_port_runner(&workspace_root())?;
    let mut cmd = std::process::Command::new(&runner);
    if is_aot {
        cmd.arg("--aot-to-o")
            .arg(&input)
            .arg("--out")
            .arg(&out_target)
            .arg("--static-platform")
            .arg(&platform);
    } else {
        if let Some(ref gd) = game_dir {
            cmd.arg("--game-dir").arg(gd);
        }
        cmd.arg("--pe")
            .arg(&input)
            .arg("--emit-static-o")
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
        bail!("port-runner static Mach-O emission exited with status {}", out.status);
    }
    Ok(())
}

fn cmd_ios_bundle(args: &[String]) -> Result<()> {
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

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" | "-o" => {
                i += 1;
                out_path = Some(PathBuf::from(args.get(i).unwrap_or_else(|| usage())));
            }
            "--target" | "--platform" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                target = IosTarget::parse(raw)
                    .ok_or_else(|| anyhow::anyhow!("invalid --target {raw:?} (expected device or sim)"))?;
            }
            "--title" => {
                i += 1;
                title_override = Some(args.get(i).unwrap_or_else(|| usage()).clone());
            }
            "--bundle-id" => {
                i += 1;
                bundle_id = Some(args.get(i).unwrap_or_else(|| usage()).clone());
            }
            "--exe" => {
                i += 1;
                exe_override = Some(args.get(i).unwrap_or_else(|| usage()).clone());
            }
            "--manifest" => {
                i += 1;
                manifest_override = Some(PathBuf::from(args.get(i).unwrap_or_else(|| usage())));
            }
            "--release" => release = true,
            "--deployment-target" => {
                i += 1;
                deployment_target = args.get(i).unwrap_or_else(|| usage()).clone();
            }
            "--no-static-aot" => emit_static_aot = false,
            "--aot-max-funcs" | "--max-funcs" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                aot_max_funcs = raw.parse().with_context(|| format!("invalid --aot-max-funcs {raw:?}"))?;
            }
            "--extra-aot-obj" => {
                i += 1;
                extra_aot_objs.push(PathBuf::from(args.get(i).unwrap_or_else(|| usage())));
            }
            "--no-precompile-shaders" => precompile_shaders = false,
            "--bundle-shader-cache" => {
                if let Some(next) = args.get(i + 1).filter(|s| !s.starts_with('-')) {
                    bundle_shader_cache_from = Some(PathBuf::from(next));
                    i += 1;
                } else {
                    bundle_shader_cache_from = Some(WinFs::default_shader_cache_dir());
                }
            }
            "--optimize-storage" | "--storage-profile" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                let prof = StorageOptProfile::parse(raw).ok_or_else(|| {
                    anyhow::anyhow!("invalid --optimize-storage {raw:?} (expected none, lossless, high, or compact)")
                })?;
                storage_opt = StorageOptOptions::from_profile(prof);
            }
            "--hevc-videos" => {
                storage_opt.hevc_videos = true;
                storage_opt.repack_archives = true;
            }
            "--no-hevc-videos" => storage_opt.hevc_videos = false,
            "--video-max-height" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                storage_opt.video_max_height =
                    raw.parse().with_context(|| format!("invalid --video-max-height {raw:?}"))?;
            }
            "--video-bitrate" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                storage_opt.video_bitrate_kbps =
                    raw.parse().with_context(|| format!("invalid --video-bitrate {raw:?}"))?;
            }
            "--max-tex-dim" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                storage_opt.max_tex_dim =
                    raw.parse().with_context(|| format!("invalid --max-tex-dim {raw:?}"))?;
                if storage_opt.max_tex_dim > 0 {
                    storage_opt.repack_archives = true;
                }
            }
            "--strip-unused-dlls" => storage_opt.strip_unused_dlls = true,
            "--keep-lang" => {
                i += 1;
                storage_opt.keep_lang = Some(args.get(i).unwrap_or_else(|| usage()).clone());
                storage_opt.repack_archives = true;
            }
            "--strip-dlc" => {
                storage_opt.strip_dlc = true;
                storage_opt.repack_archives = true;
            }
            "--opt-jobs" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                storage_opt.jobs = raw.parse().with_context(|| format!("invalid --opt-jobs {raw:?}"))?;
            }
            "--opt-dry-run" => storage_opt.dry_run = true,
            "--sign" | "--codesign" => {
                i += 1;
                codesign_identity = Some(args.get(i).unwrap_or_else(|| usage()).clone());
            }
            "--provisioning-profile" => {
                i += 1;
                provisioning_profile = Some(PathBuf::from(args.get(i).unwrap_or_else(|| usage())));
            }
            "--xcode-project" | "--xcodegen" => emit_xcodegen_spec = true,
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if source.is_none() => source = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
        i += 1;
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
        let ext = if target == IosTarget::Device { "ipa" } else { "app" };
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
    println!("  Target             : {} ({})", rep.target.as_str(), rep.target.rust_target());
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
        bail!("iOS bundle verification reported {} issue(s)", rep.verify.issues.len());
    }
    Ok(())
}

fn cmd_ios_verify(args: &[String]) -> Result<()> {
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

fn cmd_optimize_assets(args: &[String]) -> Result<()> {
    let mut src_dir: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut opts = StorageOptOptions::from_profile(StorageOptProfile::High);
    let mut profile_explicit = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" | "-o" | "--dst" => {
                i += 1;
                out_dir = Some(PathBuf::from(args.get(i).unwrap_or_else(|| usage())));
            }
            "--manifest" | "-m" => {
                i += 1;
                manifest_path = Some(PathBuf::from(args.get(i).unwrap_or_else(|| usage())));
            }
            "--profile" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                let prof = StorageOptProfile::parse(raw).ok_or_else(|| {
                    anyhow::anyhow!("invalid --profile {raw:?} (expected lossless, high, or compact)")
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
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                opts.video_max_height =
                    raw.parse().with_context(|| format!("invalid --video-max-height {raw:?}"))?;
            }
            "--video-bitrate" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                opts.video_bitrate_kbps =
                    raw.parse().with_context(|| format!("invalid --video-bitrate {raw:?}"))?;
            }
            "--max-tex-dim" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                opts.max_tex_dim =
                    raw.parse().with_context(|| format!("invalid --max-tex-dim {raw:?}"))?;
                if opts.max_tex_dim > 0 {
                    opts.repack_archives = true;
                }
            }
            "--strip-unused-dlls" => opts.strip_unused_dlls = true,
            "--no-strip-unused-dlls" => opts.strip_unused_dlls = false,
            "--keep-lang" => {
                i += 1;
                opts.keep_lang = Some(args.get(i).unwrap_or_else(|| usage()).clone());
                opts.repack_archives = true;
            }
            "--strip-dlc" => {
                opts.strip_dlc = true;
                opts.repack_archives = true;
            }
            "--jobs" | "-j" => {
                i += 1;
                let raw = args.get(i).unwrap_or_else(|| usage());
                opts.jobs = raw.parse().with_context(|| format!("invalid --jobs {raw:?}"))?;
            }
            "--dry-run" => opts.dry_run = true,
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if src_dir.is_none() => src_dir = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
        i += 1;
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

fn cmd_scaffold_op(args: &[String]) -> Result<()> {
    let mut mnemonic: Option<String> = None;
    let mut category = "GP_ALU".to_string();
    let mut arm64_pattern = "/* ARM64 instruction pattern */".to_string();
    let mut from_pe: Option<PathBuf> = None;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--category" => category = it.next().unwrap_or_else(|| usage()).clone(),
            "--arm64" => arm64_pattern = it.next().unwrap_or_else(|| usage()).clone(),
            "--from-pe" => from_pe = Some(PathBuf::from(it.next().unwrap_or_else(|| usage()))),
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            m => mnemonic = Some(m.to_string()),
        }
    }

    if let Some(pe_path) = from_pe {
        let missing = find_missing_ops_in_pe(&pe_path)?;
        if missing.is_empty() {
            println!("100.00% of instructions in {} are already covered!", pe_path.display());
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

fn cmd_scaffold_dll(args: &[String]) -> Result<()> {
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
        println!("// Pass `--write` to create this file and register `pub mod {stem};` automatically.\n");
        print!("{src}");
    }
    Ok(())
}

fn cmd_vfs_resolve(args: &[String]) -> Result<()> {
    let mut drive_c = WinFs::default_translator_root();
    let mut app_dir: Option<PathBuf> = std::env::var_os("PORT_GAME_DIR")
        .or_else(|| std::env::var_os("AW2_GAME_DIR"))
        .filter(|s| !s.is_empty())
        .map(PathBuf::from);
    let mut app_win_dir = "C:\\Games\\App".to_string();
    let mut extra_mounts: Vec<String> = Vec::new();
    let mut queries: Vec<String> = Vec::new();

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--drive-c" | "--translator-home" => {
                drive_c = PathBuf::from(it.next().unwrap_or_else(|| usage()))
            }
            "--app-dir" => app_dir = Some(PathBuf::from(it.next().unwrap_or_else(|| usage()))),
            "--app-win-dir" => app_win_dir = it.next().unwrap_or_else(|| usage()).clone(),
            "--mount" => extra_mounts.push(it.next().unwrap_or_else(|| usage()).clone()),
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

fn cmd_doctor(args: &[String]) -> Result<()> {
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
        cmd_scan(&pe_args)?;
        cmd_win32_audit(&pe_args)?;
        cmd_anticheat_audit(&pe_args)?;
        cmd_app_audit(&pe_args)?;
        cmd_dotnet_audit(&pe_args)?;
    }

    db_report.ensure_clean()?;
    Ok(())
}

fn cmd_game_profiles(args: &[String]) -> Result<()> {
    let root = workspace_root();
    let mut sync = false;
    let mut tsv_path = root.join(DEFAULT_GAME_PROFILES_TSV);
    let mut pe_paths: Vec<PathBuf> = Vec::new();

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--sync" => sync = true,
            "--tsv" => {
                sync = true;
                tsv_path = PathBuf::from(it.next().unwrap_or_else(|| usage()));
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

fn cmd_shader_formats(args: &[String]) -> Result<()> {
    let root = workspace_root();
    let mut sync = false;
    let mut tsv_path = root.join(DEFAULT_SHADER_FORMATS_TSV);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--sync" => sync = true,
            "--tsv" => {
                sync = true;
                tsv_path = PathBuf::from(it.next().unwrap_or_else(|| usage()));
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

fn cmd_shader_inspect(args: &[String]) -> Result<()> {
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
            println!("================================================================================");
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

fn cmd_shader_convert(args: &[String]) -> Result<()> {
    let mut input: Option<PathBuf> = None;
    let mut out_metallib: Option<PathBuf> = None;
    let mut out_msl: Option<PathBuf> = None;
    let mut entry: Option<String> = None;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" | "-o" => {
                out_metallib = Some(PathBuf::from(it.next().unwrap_or_else(|| usage())));
            }
            "--msl-out" | "--msl" => {
                out_msl = Some(PathBuf::from(it.next().unwrap_or_else(|| usage())));
            }
            "--entry" | "-e" => {
                entry = Some(it.next().unwrap_or_else(|| usage()).clone());
            }
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

fn cmd_app_audit(args: &[String]) -> Result<()> {
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

fn cmd_precompile_shaders(args: &[String]) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut cache_dir: Option<PathBuf> = None;
    let mut limit: usize = usize::MAX;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--cache" => cache_dir = Some(PathBuf::from(it.next().unwrap_or_else(|| usage()))),
            "--limit" => {
                limit = it
                    .next()
                    .unwrap_or_else(|| usage())
                    .parse()
                    .context("invalid --limit")?;
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

fn cmd_anticheat_audit(args: &[String]) -> Result<()> {
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

fn cmd_dotnet_audit(args: &[String]) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut strip_r2r_out: Option<PathBuf> = None;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--strip-r2r" => {
                strip_r2r_out = Some(PathBuf::from(it.next().unwrap_or_else(|| usage())));
            }
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
    println!("  Pure-IL Assemblies (ILONLY)   : {} (Direct Native ARM64 CoreCLR/Mono Ready)", summary.pure_il_count);
    println!("  ReadyToRun Hybrid (CIL+x64)   : {} (Strip R2R header or AOT-translate)", summary.r2r_count);
    println!("  Mixed-Mode C++/CLI Assemblies : {} (x86-64->ARM64 AOT + mscoree/coreclr bridge)", summary.mixed_mode_count);
    println!("  .NET AppHost / Host Launchers : {} (hostfxr.dll / coreclr.dll bridge)", summary.apphost_count);
    println!("  Unity Mono Binaries           : {} (mono-2.0-bdwgc.dll bridge)", summary.unity_mono_count);
    println!("  Unity IL2CPP Native Binaries  : {} (GameAssembly.dll -> 100% ARM64 AOT)", summary.unity_il2cpp_count);
    println!("  Pure Native Non-.NET Binaries : {}", summary.native_non_dotnet_count);
    if !summary.runtime_configs.is_empty() {
        println!("  .runtimeconfig.json Files     : {}", summary.runtime_configs.len());
    }
    if !summary.global_metadata_dat.is_empty() {
        println!("  IL2CPP global-metadata.dat    : {}", summary.global_metadata_dat.len());
    }
    println!("--------------------------------------------------------------------------------");
    println!("  Builtin Win32 .NET Bridges    : mscoree.dll, hostfxr.dll, coreclr.dll, mono-2.0-bdwgc.dll");
    println!(
        "  Host macOS ARM64 CoreCLR      : {}",
        summary
            .host_coreclr_dylib
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "builtin in-process bridge (install `brew install dotnet` for native ARM64 JIT)".to_string())
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
            .unwrap_or_else(|| "builtin in-process bridge (install `brew install mono` for native ARM64 Mono JIT)".to_string())
    );

    let interesting: Vec<_> = summary
        .binaries
        .iter()
        .filter(|b| b.kind != DotNetRuntimeKind::NativeNonDotNet || summary.binaries.len() <= 5)
        .collect();
    if !interesting.is_empty() {
        println!("--------------------------------------------------------------------------------");
        for b in interesting {
            let arch = if b.is_pe32_plus { "PE32+ (64-bit)" } else { "PE32 (AnyCPU/32-bit)" };
            println!("  * {} [{} — {}]", b.file_name, arch, b.kind.as_str());
            if let Some(ref ver) = b.metadata_version {
                println!(
                    "      CLR Metadata : {} (flags=0x{:x}, R2R={}, StrongName={}, streams={:?})",
                    ver, b.clr_flags, b.is_ready_to_run, b.is_strong_name_signed, b.metadata_streams
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

fn cmd_steam_scan(args: &[String]) -> Result<()> {
    let mut html_out: Option<PathBuf> = None;
    let mut open_browser = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--html" | "-o" if i + 1 < args.len() => {
                html_out = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--open" => {
                open_browser = true;
                i += 1;
            }
            _ => {
                i += 1;
            }
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

fn cmd_launcher(args: &[String]) -> Result<()> {
    let mut port: u16 = 7420;
    let mut open_browser = true;
    let mut ws = workspace_root();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--port" | "-p" if i + 1 < args.len() => {
                port = args[i + 1]
                    .parse()
                    .with_context(|| format!("invalid --port {:?}", args[i + 1]))?;
                i += 2;
            }
            "--workspace" | "-w" if i + 1 < args.len() => {
                ws = PathBuf::from(&args[i + 1]);
                i += 2;
            }
            "--no-open" => {
                open_browser = false;
                i += 1;
            }
            other => bail!("unknown launcher flag {other:?}"),
        }
    }

    run_launcher(LauncherOptions {
        workspace_root: ws,
        port,
        open_browser,
    })
}

fn cmd_launcher_app(args: &[String]) -> Result<()> {
    let root = workspace_root();
    let mut out_app = default_apps_dir(&root).join("Port Factory.app");
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" | "-o" if i + 1 < args.len() => {
                out_app = PathBuf::from(&args[i + 1]);
                i += 2;
            }
            other => bail!("unknown launcher-app flag {other:?}"),
        }
    }
    let built = build_launcher_app_bundle(&root, &out_app)?;
    println!("Built interactive launcher app bundle: {}", built.display());
    println!("Double-click it in Finder or run: open \"{}\"", built.display());
    Ok(())
}

fn cmd_steam_setup(args: &[String]) -> Result<()> {
    let mut dir: Option<PathBuf> = None;
    let mut force = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--dir" | "-o" if i + 1 < args.len() => {
                dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--force" => {
                force = true;
                i += 1;
            }
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
    println!("Ready to download Windows x64 Steam games with: {}", bin.display());
    Ok(())
}

fn cmd_steam_download(args: &[String]) -> Result<()> {
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

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--dir" | "-o" if i + 1 < args.len() => {
                output_dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--user" | "-u" if i + 1 < args.len() => {
                username = Some(args[i + 1].clone());
                use_qr = false;
                i += 2;
            }
            "--qr" => {
                use_qr = true;
                i += 1;
            }
            "--validate" | "--repair" => {
                validate = true;
                i += 1;
            }
            "--branch" | "-b" if i + 1 < args.len() => {
                branch = args[i + 1].clone();
                i += 2;
            }
            "--dry-run" => {
                dry_run = true;
                i += 1;
            }
            other if !other.starts_with('-') && target.is_none() => {
                target = Some(other.to_string());
                i += 1;
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

fn cmd_steam_port(args: &[String]) -> Result<()> {
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

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--dir" if i + 1 < args.len() => {
                download_dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--out" | "-o" if i + 1 < args.len() => {
                output_app = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--copy-to" if i + 1 < args.len() => {
                copy_to = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--user" | "-u" if i + 1 < args.len() => {
                username = Some(args[i + 1].clone());
                use_qr = false;
                i += 2;
            }
            "--branch" | "-b" if i + 1 < args.len() => {
                branch = args[i + 1].clone();
                i += 2;
            }
            "--qr" => {
                use_qr = true;
                i += 1;
            }
            "--validate" | "--repair" => {
                validate = true;
                i += 1;
            }
            "--skip-download" => {
                skip_download = true;
                i += 1;
            }
            "--no-shaders" => {
                precompile_shaders = false;
                i += 1;
            }
            "--dry-run" => {
                dry_run = true;
                i += 1;
            }
            other if !other.starts_with('-') && target.is_none() => {
                target = Some(other.to_string());
                i += 1;
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

fn cmd_bundles() -> Result<()> {
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

fn cmd_bundle_export(args: &[String]) -> Result<()> {
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

fn cmd_steam_postinstall(args: &[String]) -> Result<()> {
    if args.is_empty() {
        bail!("steam-postinstall requires <GAME_DIR> [--appid ID] [--out DIR]");
    }
    let mut game_dir: Option<PathBuf> = None;
    let mut app_id = "0".to_string();
    let mut out_dir: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--appid" if i + 1 < args.len() => {
                app_id = args[i + 1].clone();
                i += 2;
            }
            "--out" | "-o" if i + 1 < args.len() => {
                out_dir = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            other if !other.starts_with('-') && game_dir.is_none() => {
                game_dir = Some(PathBuf::from(other));
                i += 1;
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

fn cmd_steam_convert(args: &[String]) -> Result<()> {
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

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--format" | "-f" if i + 1 < args.len() => {
                format = ConversionFormat::parse(&args[i + 1])?;
                i += 2;
            }
            "--out" | "-o" if i + 1 < args.len() => {
                output_path = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--max-fns" if i + 1 < args.len() => {
                max_aot_functions = args[i + 1]
                    .parse()
                    .with_context(|| format!("invalid --max-fns value {:?}", args[i + 1]))?;
                i += 2;
            }
            "--copy-game" => {
                // Self-contained copy into `Contents/Resources/Game` is always enabled.
                i += 1;
            }
            "--no-shaders" => {
                precompile_shaders = false;
                i += 1;
            }
            "--bundle-shader-cache" => {
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    bundle_shader_cache_from = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
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
                    i += 1;
                }
            }
            other if !other.starts_with('-') && target.is_none() => {
                target = Some(other.to_string());
                i += 1;
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
    println!(
        "  Win32 API Coverage : {:.2}%",
        res.win32_readiness_pct
    );
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

fn cmd_bundle_verify(args: &[String]) -> Result<()> {
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

// ---------------------------------------------------------------------------
// Diagnostic & Verification Toolkit Subcommands
// ---------------------------------------------------------------------------

fn cmd_runtime_audit(args: &[String]) -> Result<()> {
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        bail!("usage: port-factory runtime-audit <PE_FILE_OR_DIR...> [--only SUBSTR]");
    }
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut only_filter: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--only" => {
                only_filter = Some(
                    it.next()
                        .context("--only requires a substring")?
                        .to_ascii_lowercase(),
                );
            }
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        bail!("runtime-audit requires at least one PE file or directory");
    }
    let mut files = Vec::new();
    for p in &paths {
        files.extend(collect_pe_files(p)?);
    }
    if let Some(ref sub) = only_filter {
        files.retain(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().to_ascii_lowercase().contains(sub.as_str()))
                .unwrap_or(false)
        });
    }
    if files.is_empty() {
        bail!("no PE32+ .exe/.dll files found in {:?}", paths);
    }
    let mut reports = Vec::new();
    for f in &files {
        match audit_pe_runtime_hazards(f) {
            Ok(r) => reports.push(r),
            Err(e) => eprintln!("[SKIP] {}: {e}", f.display()),
        }
    }
    print!("{}", format_hazard_reports(&reports));
    Ok(())
}

fn cmd_verify_x86(args: &[String]) -> Result<()> {
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!(
            "usage: port-factory verify-x86 <PE_FILE_OR_DIR...> [options]\n\
             Toolkit options:\n\
             \x20 --oracle              Export FP/SIMD samples and verify on Rosetta 2 x86-64 hardware\n\
             \x20 --oracle-dir DIR      Directory for Rosetta 2 oracle snapshots\n\
             \x20 --oracle-sample N     Samples per binary (default 32 when --oracle is set)\n\
             \x20 --rebuild             Force release rebuild of x86-verify before running\n\
             \x20 (All other flags pass straight through to x86-verify: --trials, --max-blocks,\n\
             \x20  --rva LO..HI, --fp-only, --only SUBSTR, --fast-nan, --strict-nan-payload,\n\
             \x20  --no-fast, --no-jit, --no-aot, --no-linked, --threads N)"
        );
        return Ok(());
    }
    let root = workspace_root();
    let mut run_oracle = false;
    let mut force_build = false;
    let mut oracle_dir: Option<PathBuf> = None;
    let mut oracle_sample: Option<usize> = None;
    let mut pass_args: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--oracle" => run_oracle = true,
            "--rebuild" => force_build = true,
            "--oracle-dir" => {
                i += 1;
                let d = args.get(i).context("--oracle-dir requires DIR")?;
                oracle_dir = Some(PathBuf::from(d));
            }
            "--oracle-sample" => {
                i += 1;
                let raw = args.get(i).context("--oracle-sample requires N")?;
                oracle_sample = Some(raw.parse().context("invalid --oracle-sample")?);
            }
            other => pass_args.push(other.to_string()),
        }
        i += 1;
    }

    let auto_oracle_dir = if run_oracle && oracle_dir.is_none() {
        let dir = std::env::temp_dir().join(format!("port_factory_oracle_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir)?;
        oracle_dir = Some(dir.clone());
        Some(dir)
    } else {
        None
    };

    if let Some(ref od) = oracle_dir {
        pass_args.push("--oracle-dir".to_string());
        pass_args.push(od.to_string_lossy().into_owned());
        let n = oracle_sample.unwrap_or(if run_oracle { 32 } else { 0 });
        if n > 0 {
            pass_args.push("--oracle-sample".to_string());
            pass_args.push(n.to_string());
        }
    }

    let bin = find_or_build_x86_verify(&root, force_build)?;
    let status = std::process::Command::new(&bin)
        .args(&pass_args)
        .status()
        .with_context(|| format!("running {}", bin.display()))?;
    if !status.success() {
        if let Some(ref td) = auto_oracle_dir {
            let _ = std::fs::remove_dir_all(td);
        }
        std::process::exit(status.code().unwrap_or(1));
    }

    if run_oracle {
        if let Some(ref od) = oracle_dir {
            println!("\n=== Phase 2: Rosetta 2 x86-64 Hardware Oracle ===");
            let script = resolve_debug_script(&root, "x86_oracle.py")?;
            let max_n = oracle_sample.unwrap_or(32).saturating_mul(16).max(256);
            let ostatus = std::process::Command::new("python3")
                .arg(&script)
                .arg(od)
                .arg("--max")
                .arg(max_n.to_string())
                .arg("--quiet-matches")
                .status()
                .with_context(|| format!("running {}", script.display()))?;
            if let Some(ref td) = auto_oracle_dir {
                let _ = std::fs::remove_dir_all(td);
            }
            if !ostatus.success() {
                std::process::exit(ostatus.code().unwrap_or(1));
            }
        }
    }
    Ok(())
}

fn run_debug_python(script_name: &str, args: &[String]) -> Result<()> {
    let root = workspace_root();
    let script = resolve_debug_script(&root, script_name)?;
    let status = std::process::Command::new("python3")
        .arg(&script)
        .args(args)
        .status()
        .with_context(|| format!("running {}", script.display()))?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

fn cmd_oracle(args: &[String]) -> Result<()> {
    run_debug_python("x86_oracle.py", args)
}

fn cmd_bisect_runtime(args: &[String]) -> Result<()> {
    run_debug_python("bisect_runtime.py", args)
}

fn cmd_live_probe(args: &[String]) -> Result<()> {
    run_debug_python("live_probe.py", args)
}

fn cmd_nan_trace(args: &[String]) -> Result<()> {
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!(
            "usage: port-factory nan-trace <PE_OR_MANIFEST_TOML> [options] [-- <game-args...>]\n\
             Options:\n\
             \x20 --game-dir DIR        Game install directory\n\
             \x20 --module MOD          Filter PORT_NAN_TRACE to module name (repeatable)\n\
             \x20 --range SPEC          Filter PORT_NAN_TRACE to VA/RVA range (repeatable)\n\
             \x20 --watch LO..HI        Watch memory writes overlapping guest range [LO, HI)\n\
             \x20 --host SUBSTR         Log host calls matching symbol substring\n\
             \x20 --dump-dir DIR        Dump Rosetta 2 oracle snap_*.json files to DIR\n\
             \x20 --log FILE            Write JSONL NaN trace events to FILE\n\
             \x20 --stores              Also trace FP NaN memory stores\n\
             \x20 --max N               Stop recording after N trace events\n\
             \x20 --armed               Start tracing immediately at boot (default: armed=0; arm via PORT_CONTROL_DIR `nantrace on`)\n\
             \x20 --no-boundary         Disable PORT_NAN_BOUNDARY=1 middleware boundary logging\n\
             \x20 --control-dir DIR     Enable PORT_CONTROL_DIR=DIR\n\
             \x20 --run-oracle          Automatically run Rosetta 2 x86_oracle.py on --dump-dir after exit"
        );
        return Ok(());
    }
    let root = workspace_root();
    let mut target: Option<PathBuf> = None;
    let mut game_dir: Option<PathBuf> = None;
    let mut control_dir: Option<PathBuf> = None;
    let mut boundary = true;
    let mut run_oracle = false;
    let mut spec = NanTraceSpec::default();
    let mut game_args: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--" => {
                game_args.extend(args[i + 1..].iter().cloned());
                break;
            }
            "--game-dir" => {
                i += 1;
                game_dir = Some(PathBuf::from(args.get(i).context("--game-dir requires DIR")?));
            }
            "--module" => {
                i += 1;
                spec.modules
                    .push(args.get(i).context("--module requires MOD")?.clone());
            }
            "--range" => {
                i += 1;
                spec.ranges
                    .push(args.get(i).context("--range requires SPEC")?.clone());
            }
            "--watch" => {
                i += 1;
                spec.watch_mem = Some(args.get(i).context("--watch requires LO..HI")?.clone());
            }
            "--host" => {
                i += 1;
                spec.host_filter = Some(args.get(i).context("--host requires SUBSTR")?.clone());
            }
            "--dump-dir" => {
                i += 1;
                spec.dump_dir = Some(PathBuf::from(
                    args.get(i).context("--dump-dir requires DIR")?,
                ));
            }
            "--log" => {
                i += 1;
                spec.log_file = Some(PathBuf::from(args.get(i).context("--log requires FILE")?));
            }
            "--stores" => spec.trace_stores = true,
            "--max" => {
                i += 1;
                let raw = args.get(i).context("--max requires N")?;
                spec.max_events = Some(raw.parse().context("invalid --max")?);
            }
            "--armed" => spec.start_armed = true,
            "--no-boundary" => boundary = false,
            "--control-dir" => {
                i += 1;
                control_dir = Some(PathBuf::from(
                    args.get(i).context("--control-dir requires DIR")?,
                ));
            }
            "--run-oracle" => run_oracle = true,
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if target.is_none() => target = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
        i += 1;
    }

    let target = target.context("nan-trace requires <PE_OR_MANIFEST_TOML>")?;
    if let Some(ref dd) = spec.dump_dir {
        std::fs::create_dir_all(dd)?;
    }
    let trace_env = spec.to_env_string();
    let runner = find_or_build_port_runner(&root)?;
    let mut cmd = std::process::Command::new(&runner);
    let is_toml = target
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("toml"));
    if is_toml {
        cmd.arg("--manifest").arg(&target);
    } else {
        cmd.arg("--pe").arg(&target);
    }
    if let Some(ref gd) = game_dir {
        cmd.arg("--game-dir").arg(gd);
    }
    if !game_args.is_empty() {
        cmd.arg("--").args(&game_args);
    }
    cmd.env("PORT_NAN_TRACE", &trace_env);
    if boundary {
        cmd.env("PORT_NAN_BOUNDARY", "1");
    }
    if let Some(ref cd) = control_dir {
        std::fs::create_dir_all(cd)?;
        cmd.env("PORT_CONTROL_DIR", cd);
    }
    eprintln!("port-factory nan-trace: PORT_NAN_TRACE={trace_env}");
    let status = cmd
        .status()
        .with_context(|| format!("running {}", runner.display()))?;

    if let Some(ref dd) = spec.dump_dir {
        if run_oracle {
            eprintln!("\n=== Running Rosetta 2 x86-64 oracle on {} ===", dd.display());
            run_debug_python(
                "x86_oracle.py",
                &[dd.to_string_lossy().into_owned(), "--quiet-matches".into()],
            )?;
        } else {
            eprintln!(
                "Tip: replay captured snapshots on Rosetta 2 via: cargo opsdb -- oracle {}",
                dd.display()
            );
        }
    }
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
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

        let (fix, pe) =
            parse_doctor_args(&["--check".into(), "foo.exe".into()]).unwrap();
        assert!(!fix);
        assert_eq!(pe, vec!["foo.exe"]);

        let (fix, pe) =
            parse_doctor_args(&["--fix".into(), "dir/a.dll".into()]).unwrap();
        assert!(fix);
        assert_eq!(pe, vec!["dir/a.dll"]);

        let (fix, _) = parse_doctor_args(&["--sync".into()]).unwrap();
        assert!(fix);

        assert!(parse_doctor_args(&["--unknown-flag".into()]).is_err());
    }
}
