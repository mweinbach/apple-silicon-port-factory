//! Shared CLI helpers, default paths, usage output, and subcommand module grouping
//! for the `port-factory` (`cargo opsdb`) binary.

pub mod bench;
pub mod bundle;
pub mod db;
pub mod diag;
pub mod scan;
pub mod steam;

use anyhow::Result;
use port_factory::json_output::{exit_codes, JsonEnvelope};
use std::path::{Path, PathBuf};

pub const DEFAULT_OPS_TSV: &str = "db/ops_x86_to_arm64.tsv";
pub const DEFAULT_PROFILES_TSV: &str = "db/ops_profiles.tsv";
pub const DEFAULT_WIN32_TSV: &str = "db/win32_apis.tsv";
pub const DEFAULT_STUB_INVENTORY_TSV: &str = "db/stub_inventory.tsv";
pub const DEFAULT_STUB_BACKLOG_MD: &str = "docs/stub-backlog.md";
pub const DEFAULT_SHADER_FORMATS_TSV: &str = "db/shader_formats.tsv";
pub const DEFAULT_GAME_PROFILES_TSV: &str = "db/game_profiles.tsv";
pub const DEFAULT_STEAM_BROWSER_HTML: &str = "out/steam_library_browser.html";

/// Error wrapper for invalid CLI arguments / missing required inputs (maps to exit code 2).
#[derive(Debug)]
pub struct UsageError(pub String);

impl std::fmt::Display for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for UsageError {}

pub fn usage_err(msg: impl Into<String>) -> anyhow::Error {
    anyhow::Error::new(UsageError(msg.into()))
}

/// Print a versioned [`JsonEnvelope`] to stdout and return its canonical exit code.
pub fn emit_json(env: &JsonEnvelope) -> i32 {
    println!("{}", env.to_json_pretty());
    env.exit_code
}

pub fn usage() -> ! {
    eprintln!(
        "usage: port-factory <command> [args...]\n\
         \n\
         Global output option (all audit, verify, inspect, and stats commands):\n\
           --json                               Emit stable schema v1 JSON envelope on stdout\n\
         \n\
         x86-64 -> ARM64 Ops Database Commands:\n\
           sync [--tsv PATH]                    Export canonical x86-64 -> ARM64 ops catalog to TSV\n\
           stats [--json]                       Print summary statistics of the x86-64 -> ARM64 database\n\
           list [--category CAT] [--search STR] List translated op families and their ARM64 patterns\n\
           explain <MNEMONIC_OR_ID>             Show full x86-64 -> ARM64 translation recipe & semantics\n\
           readiness <PE_OR_GAME_DIR> [--json]  Rank x86 dispatch gaps and Win32 imports per PE\n\
           scan <PE_FILE_OR_DIR...> [--record] [--top N] [--json]\n\
                                                Scan .exe/.dll binaries for ARM64 coverage & op mix\n\
         \n\
         Wine-Style Core Windows API Database Commands:\n\
           win32-sync [--tsv PATH]              Export canonical Win32 DLL & API mapping catalog to TSV\n\
           win32-stats [--search STR] [--json]  Print summary statistics of mapped Windows DLLs & APIs\n\
           win32-audit <PE_FILE_OR_DIR...> [--json]\n\
                                                Audit PE imports against the Win32 API database (exit 3 if unresolved)\n\
           stub-sync [--tsv PATH] [--backlog]   Export checked-in stub inventory (`db/stub_inventory.tsv`) & optional backlog\n\
           stub-backlog [--write] [--out PATH]  Generate `docs/stub-backlog.md` collecting `docs/stub-backlog.d/*.md`\n\
           stub-diff <BEFORE.json> <AFTER.json> Diff two runtime stub snapshots (or boot report JSONs)\n\
           report-diff <BEFORE.json> <AFTER.json> Diff two `port-runner --report-json` boot reports\n\
           audit-game <GAME_DIR_OR_PE> [--exe EXE] [--init-manifest [PATH]] [--force] [--record] [--id ID] [--name NAME] [--json]\n\
                                                Single-pass game readiness audit (imports by impl_kind, graphics API, anti-tamper, AOT coverage) & pre-filled game.toml\n\
         \n\
         Generalized Static AOT & Embedding Commands:\n\
           aot <PE_FILE> [--out PATH.rs] [--max-fns N]\n\
                                                Statically AOT-compile an x86-64 .exe/.dll to native Rust\n\
           aot-crate <PE_FILE> --out <DIR> [--name CRATE]\n\
                                                Generate a standalone, zero-dependency embeddable Rust crate\n\
           aot-prewarm [PE_FILE] [--manifest FILE] [--game-dir DIR] [--aot-dir DIR] [--max-funcs N]\n\
                                                Offline-precompile relocatable ARM64 .aot block caches via port-runner\n\
           aot-static-obj [PE_OR_AOT] --out <DIR_OR_O> [--platform ios|ios-sim|macos] [--manifest FILE] [--game-dir DIR] [--max-funcs N]\n\
                                                Compile PE or v5 .aot into Apple-signed ARM64 Mach-O .aot.o object(s) (zero MAP_JIT)\n\
           ios-bundle <PE_OR_DIR_OR_TOML> --out <PATH.app|PATH.ipa> [--target device|sim] [--title NAME] [--bundle-id ID] [--exe NAME] [--manifest FILE] [--release] [--deployment-target 17.0] [--no-static-aot] [--aot-max-funcs N] [--extra-aot-obj FILE.o] [--no-precompile-shaders] [--bundle-shader-cache [DIR]] [--optimize-storage none|lossless|high|compact] [--hevc-videos] [--no-hevc-videos] [--video-max-height N] [--video-bitrate KBPS] [--max-tex-dim N] [--strip-unused-dlls] [--keep-lang CODE] [--strip-dlc] [--opt-jobs N] [--opt-dry-run] [--sign ID] [--provisioning-profile FILE] [--xcode-project]\n\
                                                One-command iOS / iOS Simulator .app or .ipa packager with signed static .aot.o, Tier-1 shaders & optional storage optimization\n\
           ios-verify <PATH.app|PATH.ipa> [--json]\n\
                                                Verify an iOS .app bundle or .ipa archive (exit 4 on failure)\n\
           optimize-assets <SRC_DIR> --out <DST_DIR> [--profile lossless|high|compact] [--manifest FILE] [--hevc-videos] [--no-hevc-videos] [--video-max-height N] [--video-bitrate KBPS] [--max-tex-dim N] [--strip-unused-dlls] [--keep-lang CODE] [--strip-dlc] [--jobs N] [--dry-run]\n\
                                                Optimize any Windows game directory for Apple Silicon / iOS storage (HEVC Bink2, DDS mip cap, DLL strip, game repacker)\n\
         \n\
         Steam & PC Game Library Launcher, Downloader & One-Click .app Converter:\n\
           launcher [--port PORT] [--no-open] [--workspace DIR]\n\
                                                Start interactive local Launcher UI (manage Steam, convert .app, copy to SSD)\n\
           launcher-app [--out PATH.app]        Build double-clickable `out/apps/Port Factory.app` launcher bundle\n\
           steam-setup [--dir DIR]              Install/verify native Apple Silicon SteamRE/DepotDownloader\n\
           steam-scan [--html PATH] [--open] [--json]\n\
                                                Scan macOS Steam, CrossOver Steam, Heroic & owned library\n\
           steam-download <APPID_OR_NAME> [--dir DIR] [--qr | --user ACCOUNT] [--validate] [--branch BRANCH] [--dry-run]\n\
                                                Download or repair a Windows x64 Steam depot & run post-install setup\n\
           steam-postinstall <GAME_DIR> [--appid ID] [--out DIR]\n\
                                                Extract installscript.vdf registry & satisfy _CommonRedist\n\
           steam-convert <APPID_OR_NAME_OR_DIR> [--format app|aot-crate|shaders] [--out PATH] [--max-fns N] [--bundle-shader-cache [DIR]]\n\
                                                One-click convert a Steam game or steam.exe to macOS .app / AOT\n\
           steam-port <APPID_OR_NAME> [--qr | --user ACCOUNT] [--validate] [--skip-download] [--out PATH.app] [--copy-to /Volumes/SSD] [--dry-run]\n\
                                                Full end-to-end pipeline: Steam download -> .app bundle -> verify -> optional SSD copy\n\
           bundles [--json]                     List packaged .app bundles and mounted external SSDs\n\
           bundle-verify <PATH.app...> [--json] Verify an .app bundle is self-contained and portable (exit 4 on failure)\n\
           bundle-export <PATH.app> <DEST> [--force]\n\
                                                Verify and copy an .app bundle to an external SSD or target folder\n\
         \n\
         Top Games, Non-Gaming App, GPU Acceleration, Shader & .NET / CLR Commands:\n\
           game-profiles [--sync] [--json] [PE_FILES...]\n\
                                                List & audit top PC games & engine profiles (UE5, Source 2, RE, etc.)\n\
           app-audit <PE_FILE_OR_DIR...> [--json]\n\
                                                Audit desktop/CLI/IPC, GPU backends & embedded shaders\n\
           dotnet-audit <PE_FILE_OR_DIR...> [--strip-r2r OUT_FILE] [--json]\n\
                                                Audit .NET CLR / CoreCLR / ReadyToRun / Unity Mono / IL2CPP\n\
           shader-formats [--sync] [--json]     List standardized DX12/DX11/Vulkan/Metal shader file formats\n\
           shader-inspect <FILE_OR_DIR...> [--json]\n\
                                                Auto-detect & inspect raw shader files (.dxil/.dxbc/.spv/.wgsl)\n\
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
           vfs-resolve <WIN_PATH...> [--drive-c DIR] [--app-dir DIR] [--mount WIN=HOST] [--json]\n\
                                                Test & inspect Windows VFS path/env/mount resolution\n\
           anticheat-audit <PE_FILE_OR_DIR...> [--json]\n\
                                                Audit direct syscalls, VEH traps, entropy & anti-debug checks (exit 5 if Ring-0 driver required)\n\
           doctor [--check|--fix] [--json] [PE_FILE_OR_DIR...]\n\
                                                Verify db/*.tsv drift (exit 4 on mismatch; --fix syncs), VFS & pipelines\n\
         \n\
         Diagnostic & Verification Toolkit (Any Windows Game / DLL):\n\
           runtime-audit <PE_FILE_OR_DIR...> [--only SUBSTR] [--json]\n\
                                                Static audit for MXCSR/FPCR, x87, AVX narrowing, approx-FP, lock-RMW, WaitOnAddress & RWX hazards\n\
           verify-x86 <PE_FILE_OR_DIR...> [--oracle] [--oracle-sample N] [--rebuild] [--json] [x86-verify flags...]\n\
                                                Offline 4-tier differential verifier (exit 4 on critical mismatch)\n\
           oracle <SNAP_OR_DIR...> [--max N] [--mxcsr HEX] [--quiet-matches] [--json]\n\
                                                Replay PORT_NAN_TRACE / x86-verify snapshots on real Rosetta 2 x86-64 hardware (exit 4 on mismatch)\n\
           bisect-runtime --cmd \"<REPRO>\" [--bisect-range MOD:LO..HI] [--json] [bisect flags...]\n\
                                                Run execution-mode knob matrix & O(log N) PORT_AOT_SKIP/PORT_JIT_SKIP binary search\n\
           live-probe --manifest TOML|--pe EXE [--game-dir DIR] [--script FILE] [--step CMD...] [--repeat N] [--json]\n\
                                                Frame-paced PORT_CONTROL_DIR automation, PNG capture & pass/fail repro harness\n\
           nan-trace <PE_OR_TOML> [--game-dir DIR] [--module MOD] [--watch LO..HI] [--dump-dir DIR] [--log FILE] [--armed] [--run-oracle] [-- <game-args...>]\n\
                                                Launch port-runner pre-configured with PORT_NAN_TRACE + PORT_NAN_BOUNDARY (and optional Rosetta oracle)\n\
           bench [--mode aot|jit] [--only W1,W2] [--tolerance 0.25] [--baseline FILE] [--out FILE] [--runner PATH] [--update-baseline] [--no-compare] [--json]\n\
                                                Build & run Windows x64 guest benchmark workloads, emit JSON, and compare against chip-keyed baseline"
    );
    std::process::exit(exit_codes::USAGE_ERROR);
}

pub fn workspace_root() -> PathBuf {
    if let Some(ws) = std::env::var_os("PORT_FACTORY_WORKSPACE").filter(|v| !v.is_empty()) {
        return PathBuf::from(ws);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Lightweight cursor over CLI argument slices (`&[String]`) to simplify flag parsing.
pub struct ArgCursor<'a> {
    args: &'a [String],
    pos: usize,
}

impl<'a> ArgCursor<'a> {
    pub fn new(args: &'a [String]) -> Self {
        Self { args, pos: 0 }
    }

    pub fn next(&mut self) -> Option<&'a str> {
        let s = self.args.get(self.pos)?.as_str();
        self.pos += 1;
        Some(s)
    }

    pub fn next_or_err(&mut self, msg: &str) -> Result<&'a str> {
        self.next().ok_or_else(|| usage_err(msg))
    }

    pub fn next_path_or_err(&mut self, msg: &str) -> Result<PathBuf> {
        self.next_or_err(msg).map(PathBuf::from)
    }

    pub fn next_opt_value(&mut self) -> Option<&'a str> {
        let peek = self.args.get(self.pos)?.as_str();
        if peek.starts_with('-') {
            None
        } else {
            self.pos += 1;
            Some(peek)
        }
    }

    pub fn take_rest(&mut self) -> &'a [String] {
        let rest = &self.args[self.pos..];
        self.pos = self.args.len();
        rest
    }
}
