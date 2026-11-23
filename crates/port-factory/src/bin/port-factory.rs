//! CLI tool (`port-factory` / `cargo opsdb`) for:
//! 1. The `x86-64 -> ARM64` operation translation database (`db/ops_x86_to_arm64.tsv`)
//! 2. The Wine-style Core Windows API mapping database (`db/win32_apis.tsv`)
//! 3. Multi-game / library PE binary compatibility & Win32 import auditing
//! 4. The Generalized Static AOT Compiler (`x86-64 PE -> Native ARM64 Rust`)
//! 5. Scaffolding new ops (`scaffold-op`), new DLL builtins (`scaffold-dll`),
//!    embeddable AOT Rust crates (`aot-crate`), and Windows VFS path routing (`vfs-resolve`)
//! 6. macOS / iOS `.app` & `.ipa` bundling, storage optimization, Steam library management,
//!    and the 5-stage Diagnostic & Verification Toolkit

mod port_factory;

use anyhow::{bail, Result};
use port_factory::{bundle, db, diag, scan, steam, usage};

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|s| s == "--") {
        args.remove(0);
    }
    let Some(cmd) = args.first().map(|s| s.as_str()) else {
        usage();
    };
    let rest = &args[1..];
    match cmd {
        "sync" => db::cmd_sync(rest),
        "stats" => db::cmd_stats(),
        "list" => db::cmd_list(rest),
        "explain" => db::cmd_explain(rest),
        "scan" => scan::cmd_scan(rest),
        "win32-sync" => db::cmd_win32_sync(rest),
        "win32-stats" => db::cmd_win32_stats(rest),
        "win32-audit" => scan::cmd_win32_audit(rest),
        "aot" => scan::cmd_aot(rest),
        "aot-crate" => scan::cmd_aot_crate(rest),
        "aot-prewarm" | "prewarm-aot" => bundle::cmd_aot_prewarm(rest),
        "aot-static-obj" | "static-aot-obj" => bundle::cmd_aot_static_obj(rest),
        "ios-bundle" | "ios-package" | "ios-ipa" => bundle::cmd_ios_bundle(rest),
        "ios-verify" => bundle::cmd_ios_verify(rest),
        "optimize-assets" | "storage-opt" | "opt-assets" => bundle::cmd_optimize_assets(rest),
        "launcher" | "ui" | "gui" => steam::cmd_launcher(rest),
        "launcher-app" => steam::cmd_launcher_app(rest),
        "steam-setup" => steam::cmd_steam_setup(rest),
        "steam-scan" | "steam-library" | "steam-browser" => steam::cmd_steam_scan(rest),
        "steam-download" | "steam-update" => steam::cmd_steam_download(rest),
        "steam-postinstall" => steam::cmd_steam_postinstall(rest),
        "steam-convert" => steam::cmd_steam_convert(rest),
        "steam-port" | "port" => steam::cmd_steam_port(rest),
        "bundles" | "bundle-list" => bundle::cmd_bundles(),
        "bundle-verify" | "verify-bundle" => bundle::cmd_bundle_verify(rest),
        "bundle-export" | "export-bundle" => bundle::cmd_bundle_export(rest),
        "game-profiles" | "games-audit" => db::cmd_game_profiles(rest),
        "app-audit" | "gpu-audit" => scan::cmd_app_audit(rest),
        "dotnet-audit" | "clr-audit" => scan::cmd_dotnet_audit(rest),
        "shader-formats" => db::cmd_shader_formats(rest),
        "shader-inspect" => scan::cmd_shader_inspect(rest),
        "shader-convert" => scan::cmd_shader_convert(rest),
        "precompile-shaders" | "shader-precompile" => scan::cmd_precompile_shaders(rest),
        "scaffold-op" => db::cmd_scaffold_op(rest),
        "scaffold-dll" => db::cmd_scaffold_dll(rest),
        "vfs-resolve" => db::cmd_vfs_resolve(rest),
        "anticheat-audit" => scan::cmd_anticheat_audit(rest),
        "doctor" => db::cmd_doctor(rest),
        "runtime-audit" | "mxcsr-audit" | "hazard-audit" => diag::cmd_runtime_audit(rest),
        "verify-x86" | "x86-verify" => diag::cmd_verify_x86(rest),
        "oracle" | "x86-oracle" => diag::cmd_oracle(rest),
        "bisect-runtime" | "bisect" => diag::cmd_bisect_runtime(rest),
        "live-probe" | "probe" => diag::cmd_live_probe(rest),
        "nan-trace" => diag::cmd_nan_trace(rest),
        "-h" | "--help" | "help" => usage(),
        other => bail!("unknown subcommand {other:?}"),
    }
}
