//! `port-loader`: universal native ARM64 Windows x86-64 PE loader & runtime host
//! (`port-runner`), with optional per-game decompilation target support (`aw2-loader`
//! under `--features aw2`).
//!
//! Maps Windows x86-64 PE executables and DLLs natively on `aarch64-apple-darwin`
//! (or legacy `x86_64` under Rosetta 2). Windows APIs are provided by `win32-rt`;
//! bundled third-party DLLs from the game directory are loaded as PEs and executed via
//! `x86-runtime`'s ARM64/NEON block JIT and persistent `.aot` cache.
#![cfg_attr(test, allow(linker_messages))]

win64_abi::win64_mods! {
    mod builtin_image;
    mod hooks;
    mod live_ctl;
    mod loader;
    mod signals;
    mod symbols;
    mod target;
    mod thunks;
}

mod aot_dylib;
pub mod ios_host;
pub mod run_limits;
mod shader_prewarm;

#[derive(Debug)]
struct CliUsageError(String);

impl std::fmt::Display for CliUsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CliUsageError {}

fn usage_err(msg: impl Into<String>) -> anyhow::Error {
    anyhow::Error::new(CliUsageError(msg.into()))
}

fn emit_cli_json_error(code: i32, msg: &str) {
    #[cfg(feature = "cli")]
    {
        let env = port_factory::json_output::JsonEnvelope::error("port-runner", code, msg);
        run_limits::write_json_stdout(&env.to_json_pretty());
    }
    #[cfg(not(feature = "cli"))]
    {
        let escaped = msg.replace('\\', "\\\\").replace('"', "\\\"");
        run_limits::write_json_stdout(&format!(
            "{{\"schema_version\":1,\"command\":\"port-runner\",\"ok\":false,\"exit_code\":{code},\"data\":null,\"error\":\"{escaped}\"}}"
        ));
    }
}

/// Register any compiled-in optional [`x86_runtime::game_plugin::GamePlugin`] implementations.
/// Idempotent (deduplicated by plugin ID).
fn register_builtin_plugins() {
    #[cfg(feature = "aw2-core")]
    aw2_game::register_plugin();
}

pub fn run_cli() {
    let want_json = std::env::args().any(|a| a == "--json");
    if want_json {
        // Reserve original fd 1 exclusively for the `--json` envelope and send
        // guest/host stdout writes to stderr so they can never corrupt JSON stdout.
        run_limits::reserve_json_stdout();
    }
    register_builtin_plugins();
    init_boot_log();
    if let Err(e) = real_main() {
        let code = if e.downcast_ref::<CliUsageError>().is_some() {
            run_limits::exit_codes::USAGE_ERROR
        } else {
            run_limits::exit_codes::RUNTIME_ERROR
        };
        let msg = format!("{e:#}");
        boot_log_line(&format!("port-runner fatal startup error (exit {code}): {msg}"));
        if want_json {
            emit_cli_json_error(code, &msg);
        } else {
            let full = format!("port-runner fatal startup error: {msg}");
            eprintln!("{full}");
            write_last_crash(&full);
            show_startup_error_alert(&full);
        }
        std::process::exit(code);
    }
}

struct Args {
    manifest_path: Option<std::path::PathBuf>,
    game_dir: std::path::PathBuf,
    exe: String,
    pe_path: Option<std::path::PathBuf>,
    drive_c_dir: Option<std::path::PathBuf>,
    extra_mounts: Vec<String>,
    trace: bool,
    no_run: bool,
    json: bool,
    timeout_secs: Option<f64>,
    max_frames: Option<u64>,
    no_avx: bool,
    no_builtin_overrides: bool,
    no_game_hooks: bool,
    ops_report: bool,
    list_exports: bool,
    call_export: Option<String>,
    prewarm_aot: bool,
    prewarm_shaders: bool,
    emit_static_o: Option<std::path::PathBuf>,
    aot_to_o: Option<std::path::PathBuf>,
    out_path: Option<std::path::PathBuf>,
    static_platform: String,
    aot_max_funcs: usize,
    aot_dir: Option<std::path::PathBuf>,
    audio_harness: bool,
    diff_test: bool,
    game_args: Vec<String>,
    explicit_game_dir: bool,
    explicit_exe: bool,
}

fn usage() -> ! {
    eprintln!(
        "usage: port-runner [--manifest FILE] [--game-dir DIR] [--exe NAME] [--pe PATH] [--drive-c DIR] [--mount WIN=HOST] [--prewarm-aot] [--prewarm-shaders] [--aot-max-funcs N] [--aot-dir DIR] [--trace] [--no-run] [--json] [--timeout-secs SECS] [--max-frames N] [--ops-report] [--list-exports] [--call-export SYM] [--no-builtin-overrides] [--no-game-hooks] [--no-avx] [--audio-harness] [--diff-test] [-- ARGS...]\n\
         \n\
           --manifest FILE         declarative game.toml manifest (default: auto-discover ../Resources/game.toml or $PORT_MANIFEST)\n\
           --game-dir DIR          game/library directory (default: bundled ../Resources/Game, $PORT_GAME_DIR / $AW2_GAME_DIR, or current dir)\n\
           --exe NAME              executable or DLL inside the game dir\n\
           --pe PATH               direct path to any Windows x86-64 .exe or .dll file\n\
           --drive-c DIR           custom Windows C:\\ prefix directory (default: $TRANSLATOR_HOME / $AW2_DRIVE_C)\n\
           --mount WIN=HOST        mount a Windows drive or folder to a host directory (repeatable, or $PORT_MOUNTS / $AW2_MOUNTS)\n\
           --prewarm-aot           offline-precompile relocatable ARM64 .aot block caches for all mapped PEs and exit\n\
           --prewarm-shaders       precompile recorded Tier-1 shaders & Metal 4 PSOs for this GPU/OS build and exit\n\
           --emit-static-o DIR     offline-compile all mapped PEs into signed-ready ARM64 Mach-O .aot.o objects in DIR and exit\n\
           --aot-to-o FILE.aot     convert an existing v5 .aot cache file into an ARM64 Mach-O .o (use with --out FILE.o)\n\
           --out PATH              output path for --aot-to-o\n\
           --static-platform PLAT  Mach-O platform for --emit-static-o / --aot-to-o: ios (default), ios-sim, macos\n\
           --aot-max-funcs N       cap discovered PE function roots per module during --prewarm-aot (0 = all)\n\
           --aot-dir DIR           override AOT block-cache directory ($PORT_AOT_DIR / $AW2_DLL_AOT_DIR)\n\
           --ops-report            analyze target PE against the x86-64 -> ARM64 ops database\n\
           --list-exports          list exported functions of the target PE and exit\n\
           --call-export SYM       initialize target DLL/EXE and invoke export SYM (or #ORD) with optional integer/string args after `--`\n\
           --no-builtin-overrides  load real PE DLLs from directory instead of builtin overrides ($PORT_BUILTIN_OVERRIDES=0)\n\
           --no-game-hooks         disable optional compiled-in game plugin hooks/replacements (alias: --no-aw2-hooks)\n\
           --trace                 log every call through an import (target `trace`)\n\
           --no-run                map + resolve only; report unresolved imports and exit (exit 3 if unresolved > 0)\n\
           --json                  emit stable schema v1 JSON envelope on stdout\n\
           --timeout-secs SECS     terminate live run after SECS seconds with exit code 124 (positive float/int)\n\
           --max-frames N          stop live run cleanly (exit 0) after N presented frames (>= 1)\n\
           --no-avx                do not re-exec with ROSETTA_ADVERTISE_AVX=1\n\
           --audio-harness         enable plugin audio test harness when supported by the active game plugin\n\
           --diff-test             run differential tests (original vs Rust) and exit (exit 4 on failure)\n\
         \n\
         Env vars: PORT_AOT_DIR, PORT_BUILTIN_OVERRIDES, PORT_HEADLESS, PORT_SYMBOLS, PORT_AOT_DYLIB (legacy AW2_* aliases supported)\n\
         Logging:  RUST_LOG (default `info`), e.g. RUST_LOG=info,stub=warn,trace=info"
    );
    std::process::exit(run_limits::exit_codes::USAGE_ERROR);
}

fn default_game_dir() -> std::path::PathBuf {
    if let Some(p) = std::env::var_os("PORT_GAME_DIR")
        .or_else(|| std::env::var_os("AW2_GAME_DIR"))
        .filter(|p| !p.is_empty())
    {
        return p.into();
    }
    if let Some(bundled) = target::bundled_game_dir() {
        return bundled;
    }
    std::path::PathBuf::from(".")
}

fn parse_args() -> anyhow::Result<Args> {
    try_parse_args_from(std::env::args().skip(1))
}

#[cfg_attr(not(test), allow(dead_code))]
fn parse_args_from<I: IntoIterator<Item = String>>(iter: I) -> Args {
    try_parse_args_from(iter).unwrap_or_else(|e| {
        eprintln!("{e}");
        usage()
    })
}

fn try_parse_args_from<I: IntoIterator<Item = String>>(iter: I) -> anyhow::Result<Args> {
    let mut a = Args {
        manifest_path: None,
        game_dir: default_game_dir(),
        exe: x86_runtime::game_plugin::primary_default_exe()
            .unwrap_or_default()
            .to_string(),
        pe_path: None,
        drive_c_dir: None,
        extra_mounts: Vec::new(),
        trace: false,
        no_run: false,
        json: false,
        timeout_secs: None,
        max_frames: None,
        no_avx: false,
        no_builtin_overrides: false,
        no_game_hooks: false,
        ops_report: false,
        list_exports: false,
        call_export: None,
        prewarm_aot: false,
        prewarm_shaders: false,
        emit_static_o: None,
        aot_to_o: None,
        out_path: None,
        static_platform: "ios".into(),
        aot_max_funcs: 0,
        aot_dir: None,
        audio_harness: std::env::var_os("AW2_AUDIO_HARNESS").is_some_and(|v| v == "1"),
        diff_test: false,
        game_args: Vec::new(),
        explicit_game_dir: false,
        explicit_exe: false,
    };
    let mut it = iter.into_iter().peekable();
    // `cargo runner -- ...` / `cargo loader -- ...` passes a leading `--` through; ignore it.
    if it.peek().is_some_and(|s| s == "--") {
        it.next();
    }
    let next_val = |it: &mut std::iter::Peekable<_>, flag: &str| -> anyhow::Result<String> {
        it.next()
            .ok_or_else(|| usage_err(format!("{flag} requires a value")))
    };
    while let Some(s) = it.next() {
        match s.as_str() {
            "--manifest" => {
                a.manifest_path = Some(std::path::PathBuf::from(next_val(&mut it, "--manifest")?));
            }
            "--game-dir" => {
                a.game_dir = next_val(&mut it, "--game-dir")?.into();
                a.explicit_game_dir = true;
            }
            "--exe" => {
                a.exe = next_val(&mut it, "--exe")?;
                a.explicit_exe = true;
            }
            "--pe" => {
                let p = std::path::PathBuf::from(next_val(&mut it, "--pe")?);
                if !a.explicit_game_dir {
                    if let Some(parent) = p.parent().filter(|d| !d.as_os_str().is_empty()) {
                        a.game_dir = parent.to_path_buf();
                    } else {
                        a.game_dir = std::path::PathBuf::from(".");
                    }
                }
                if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                    a.exe = name.to_string();
                    a.explicit_exe = true;
                }
                a.pe_path = Some(p);
            }
            "--drive-c" => {
                a.drive_c_dir = Some(std::path::PathBuf::from(next_val(&mut it, "--drive-c")?));
            }
            "--mount" => {
                a.extra_mounts.push(next_val(&mut it, "--mount")?);
            }
            "--prewarm-aot" => a.prewarm_aot = true,
            "--prewarm-shaders" => a.prewarm_shaders = true,
            "--emit-static-o" => {
                a.emit_static_o = Some(std::path::PathBuf::from(next_val(&mut it, "--emit-static-o")?));
            }
            "--aot-to-o" => {
                a.aot_to_o = Some(std::path::PathBuf::from(next_val(&mut it, "--aot-to-o")?));
            }
            "--out" => {
                a.out_path = Some(std::path::PathBuf::from(next_val(&mut it, "--out")?));
            }
            "--static-platform" => {
                a.static_platform = next_val(&mut it, "--static-platform")?;
            }
            "--aot-max-funcs" => {
                let raw = next_val(&mut it, "--aot-max-funcs")?;
                a.aot_max_funcs = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --aot-max-funcs value {raw:?}")))?;
            }
            "--aot-dir" => {
                a.aot_dir = Some(std::path::PathBuf::from(next_val(&mut it, "--aot-dir")?));
            }
            "--trace" => a.trace = true,
            "--no-run" => a.no_run = true,
            "--json" => a.json = true,
            "--timeout-secs" => {
                let raw = next_val(&mut it, "--timeout-secs")?;
                let secs: f64 = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --timeout-secs value {raw:?}")))?;
                if !secs.is_finite() || secs <= 0.0 {
                    return Err(usage_err(format!(
                        "--timeout-secs must be a positive finite number, got {raw:?}"
                    )));
                }
                a.timeout_secs = Some(secs);
            }
            "--max-frames" => {
                let raw = next_val(&mut it, "--max-frames")?;
                let n: u64 = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --max-frames value {raw:?}")))?;
                if n == 0 {
                    return Err(usage_err("--max-frames must be >= 1"));
                }
                a.max_frames = Some(n);
            }
            "--no-avx" => a.no_avx = true,
            "--no-builtin-overrides" => a.no_builtin_overrides = true,
            "--no-game-hooks" | "--no-aw2-hooks" => a.no_game_hooks = true,
            "--ops-report" => a.ops_report = true,
            "--list-exports" => a.list_exports = true,
            "--call-export" => a.call_export = Some(next_val(&mut it, "--call-export")?),
            "--audio-harness" => a.audio_harness = true,
            "--diff-test" => a.diff_test = true,
            "--" => {
                a.game_args.extend(it.by_ref());
            }
            "-h" | "--help" => usage(),
            s if s.starts_with("-psn_") => {}
            "-NSDocumentRevisionsDebugMode" => {
                let _ = it.next();
            }
            other => {
                return Err(usage_err(format!("unknown argument {other:?}")));
            }
        }
    }
    Ok(a)
}

/// Fast header-only peek of a PE32+ file's `(image_base, page_aligned_size_of_image, is_dll)`.
fn peek_pe_preferred_window(path: &std::path::Path) -> Option<(usize, usize, bool)> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = [0u8; 4096];
    let n = f.read(&mut buf).ok()?;
    if n < 0x40 || &buf[0..2] != b"MZ" {
        return None;
    }
    let lfanew = u32::from_le_bytes(buf[0x3C..0x40].try_into().ok()?) as usize;
    if lfanew.checked_add(24 + 64)? > n || &buf[lfanew..lfanew + 4] != b"PE\0\0" {
        return None;
    }
    let characteristics = u16::from_le_bytes(buf[lfanew + 22..lfanew + 24].try_into().ok()?);
    let is_dll = (characteristics & 0x2000) != 0;
    let opt = lfanew + 24;
    let magic = u16::from_le_bytes(buf[opt..opt + 2].try_into().ok()?);
    if magic != 0x20B {
        return None;
    }
    let image_base = u64::from_le_bytes(buf[opt + 24..opt + 32].try_into().ok()?) as usize;
    let size_of_image = u32::from_le_bytes(buf[opt + 56..opt + 60].try_into().ok()?) as usize;
    let aligned_size = (size_of_image + loader::PAGE - 1) & !(loader::PAGE - 1);
    (image_base != 0 && aligned_size != 0).then_some((image_base, aligned_size, is_dll))
}

/// Rosetta only advertises AVX/AVX2/FMA/BMI in CPUID when this variable is
/// set at process start, so re-exec ourselves with it. Also probes the target
/// EXE's preferred base window `(probe_base, probe_size)` when deterministic
/// placement is requested (lifted shards, offline AOT prewarming, or matching
/// `.aot` caches), retrying ASLR if macOS placed host mappings there.
fn ensure_avx_env(args: &Args, probe_window: Option<(usize, usize)>) {
    let no_retry = std::env::var("PORT_NO_BASE_RETRY").is_ok_and(|v| v == "1");
    if let (false, Some((probe_base, probe_size))) = (no_retry, probe_window) {
        if (0x1_0000_0000..0x1_7F00_0000).contains(&probe_base)
            && probe_size > 0
            && probe_base.checked_add(probe_size).is_some_and(|e| e <= 0x1_7F00_0000)
        {
            unsafe {
                let p = libc::mmap(
                    probe_base as *mut libc::c_void,
                    probe_size,
                    libc::PROT_NONE,
                    libc::MAP_PRIVATE | libc::MAP_ANON,
                    -1,
                    0,
                );
                if p != libc::MAP_FAILED {
                    libc::munmap(p, probe_size);
                }
                let tries: u32 = std::env::var("AW2_ASLR_RETRY")
                    .or_else(|_| std::env::var("PORT_ASLR_RETRY"))
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                boot_log_line(&format!(
                    "mmap probe for 0x{probe_base:x}+0x{probe_size:x} returned 0x{:x} (AW2_ASLR_RETRY={tries})",
                    p as usize
                ));
                if p as usize != probe_base {
                    if tries < 16 {
                        if let Ok(exe) = std::env::current_exe() {
                            boot_log_line(&format!(
                                "0x{probe_base:x}+0x{probe_size:x} occupied (got 0x{:x}), re-execing ({}/16): {}",
                                p as usize,
                                tries + 1,
                                exe.display()
                            ));
                            use std::os::unix::process::CommandExt;
                            let err = std::process::Command::new(exe)
                                .args(std::env::args_os().skip(1))
                                .env("AW2_ASLR_RETRY", (tries + 1).to_string())
                                .env("PORT_ASLR_RETRY", (tries + 1).to_string())
                                .exec();
                            boot_log_line(&format!("re-exec with AW2_ASLR_RETRY failed: {err}"));
                        }
                    } else {
                        boot_log_line(&format!(
                            "warning: 0x{probe_base:x}+0x{probe_size:x} still occupied after 16 ASLR retries; continuing with relocated base"
                        ));
                    }
                }
            }
        }
    }
    #[cfg(target_arch = "x86_64")]
    {
        use std::os::unix::process::CommandExt;
        if args.no_avx || std::env::var_os("ROSETTA_ADVERTISE_AVX").is_some() {
            return;
        }
        let exe = match std::env::current_exe() {
            Ok(e) => e,
            Err(_) => return,
        };
        boot_log_line("re-execing x86_64 binary with ROSETTA_ADVERTISE_AVX=1");
        let err = std::process::Command::new(exe).args(std::env::args_os().skip(1)).env("ROSETTA_ADVERTISE_AVX", "1").exec();
        eprintln!("warning: re-exec with ROSETTA_ADVERTISE_AVX=1 failed: {err}");
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = args;
}

fn translated() -> bool {
    let mut v: libc::c_int = 0;
    let mut sz = std::mem::size_of::<libc::c_int>();
    let name = std::ffi::CString::new("sysctl.proc_translated").unwrap();
    unsafe { libc::sysctlbyname(name.as_ptr(), &mut v as *mut _ as *mut libc::c_void, &mut sz, std::ptr::null_mut(), 0) == 0 && v == 1 }
}

fn setup_filesystem(game_dir: &std::path::Path, game_win_dir: &str, args: &Args) -> anyhow::Result<()> {
    use win32_rt::path;
    let translator_root = path::init_translator_mounts(args.drive_c_dir.as_deref())?;
    path::add_mount(game_win_dir, game_dir);
    if let Ok(env_mounts) = std::env::var("PORT_MOUNTS").or_else(|_| std::env::var("AW2_MOUNTS")) {
        for spec in env_mounts.split(';').filter(|s| !s.trim().is_empty()) {
            let (win, host) = path::add_mount_spec(spec).map_err(|e| anyhow::anyhow!(e))?;
            log::info!("mount {win} -> {}", host.display());
        }
    }
    for spec in &args.extra_mounts {
        let (win, host) = path::add_mount_spec(spec).map_err(|e| anyhow::anyhow!(e))?;
        log::info!("mount {win} -> {}", host.display());
    }
    path::set_cwd(game_win_dir);
    log::info!("Translator root -> {}", translator_root.display());
    log::info!("Z:\\ -> /");
    log::info!("{game_win_dir} -> {}", game_dir.display());
    Ok(())
}

/// Exe image range for debuggers (tools/debug/lldb_aw2.py reads these).
#[no_mangle]
pub static AW2_EXE_BASE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[no_mangle]
pub static AW2_EXE_SIZE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
/// Set to non-zero by the debugger (tools/lldb.sh) to release
/// `AW2_WAIT_DEBUGGER`.
#[no_mangle]
pub static AW2_DEBUGGER_GO: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Debugger hook (tools/lldb.sh): called once the exe and its static
/// imports are mapped and replacement hooks/probes are installed, before any
/// Windows code (DLL init, TLS callbacks, entry point) runs.
///
/// With `AW2_WAIT_DEBUGGER=<seconds>` the loader waits here (polling
/// `AW2_DEBUGGER_GO`, at most that long) so a debugger can attach to the
/// running process and set breakpoints by exe VA. Attaching to a running
/// process is the supported way: launching under lldb wedged Rosetta's
/// oahd once (see tools/README.md, Debugging).
#[no_mangle]
#[inline(never)]
pub extern "C" fn aw2_debug_image_mapped(exe_base: usize, exe_size: usize) {
    use std::sync::atomic::Ordering;
    AW2_EXE_BASE.store(exe_base, Ordering::SeqCst);
    AW2_EXE_SIZE.store(exe_size, Ordering::SeqCst);
    let Some(secs) = std::env::var("PORT_WAIT_DEBUGGER")
        .or_else(|_| std::env::var("AW2_WAIT_DEBUGGER"))
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
    else {
        return;
    };
    let pid = std::process::id();
    log::warn!("WAIT_DEBUGGER: waiting up to {secs} s for a debugger: pid {pid} (tools/lldb.sh -p {pid})");
    eprintln!("port-runner: waiting for debugger, pid {pid}");
    let t0 = std::time::Instant::now();
    while AW2_DEBUGGER_GO.load(Ordering::SeqCst) == 0 && t0.elapsed().as_secs() < secs {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    if AW2_DEBUGGER_GO.load(Ordering::SeqCst) == 0 {
        log::warn!("WAIT_DEBUGGER: no debugger after {secs} s, continuing");
    }
}

fn raise_fd_limit() {
    unsafe {
        let mut rl: libc::rlimit = std::mem::zeroed();
        if libc::getrlimit(libc::RLIMIT_NOFILE, &mut rl) == 0 {
            let old_cur = rl.rlim_cur;
            let target: libc::rlim_t = 65536;
            let new_cur = if rl.rlim_max == libc::RLIM_INFINITY {
                target.max(old_cur)
            } else {
                target.min(rl.rlim_max).max(old_cur)
            };
            if new_cur > old_cur {
                rl.rlim_cur = new_cur;
                if libc::setrlimit(libc::RLIMIT_NOFILE, &rl) == 0 {
                    log::info!("RLIMIT_NOFILE raised: {old_cur} -> {new_cur} (max={})", rl.rlim_max);
                } else {
                    log::warn!("setrlimit(RLIMIT_NOFILE, {new_cur}) failed: errno={}", *libc::__error());
                }
            } else {
                log::info!("RLIMIT_NOFILE: cur={old_cur} max={}", rl.rlim_max);
            }
        }
    }
}

static BOOT_LOG_PATH: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
static BOOT_LOG_TEE: std::sync::OnceLock<std::sync::Mutex<std::fs::File>> = std::sync::OnceLock::new();

fn log_dir_path_for(dir_name: &str) -> std::path::PathBuf {
    if let Some(d) = std::env::var_os("PORT_LOG_DIR")
        .or_else(|| std::env::var_os("AW2_LOG_DIR"))
        .filter(|s| !s.is_empty())
    {
        return d.into();
    }
    if let Some(home) = std::env::var_os("HOME").map(std::path::PathBuf::from) {
        return home.join("Library/Logs").join(dir_name);
    }
    std::path::PathBuf::from("/tmp").join(dir_name)
}

fn log_dir_path() -> std::path::PathBuf {
    let dir_name = win32_rt::config::app_config().log_dir_name;
    log_dir_path_for(&dir_name)
}

fn sysctl_str(name: &str) -> String {
    let Ok(cname) = std::ffi::CString::new(name) else { return String::new() };
    let mut buf = [0u8; 256];
    let mut sz = buf.len();
    unsafe {
        if libc::sysctlbyname(cname.as_ptr(), buf.as_mut_ptr() as *mut libc::c_void, &mut sz, std::ptr::null_mut(), 0) == 0 && sz > 0 {
            let end = buf[..sz].iter().position(|&b| b == 0).unwrap_or(sz);
            return String::from_utf8_lossy(&buf[..end]).into_owned();
        }
    }
    String::new()
}

pub fn boot_log_line(msg: &str) {
    use std::io::Write;
    let Some(path) = BOOT_LOG_PATH.get() else { return };
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0.0, |d| d.as_secs_f64());
        let _ = writeln!(f, "[{secs:.3} pid={} boot] {msg}", std::process::id());
        let _ = f.flush();
    }
}

fn write_last_crash(msg: &str) {
    let dir = log_dir_path();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("last-crash.txt"), format!("{msg}\n"));
}

fn is_headless_env() -> bool {
    std::env::var("PORT_HEADLESS")
        .or_else(|_| std::env::var("AW2_HEADLESS"))
        .is_ok_and(|v| !v.trim().is_empty() && v.trim() != "0")
}

fn show_startup_error_alert(msg: &str) {
    if cfg!(target_os = "ios")
        || is_headless_env()
        || std::env::args().any(|a| {
            matches!(
                a.as_str(),
                "--no-run"
                    | "--json"
                    | "--timeout-secs"
                    | "--max-frames"
                    | "--diff-test"
                    | "--prewarm-aot"
                    | "--prewarm-shaders"
                    | "--emit-static-o"
                    | "--aot-to-o"
                    | "--list-exports"
                    | "--ops-report"
            )
        })
    {
        return;
    }
    let app_name = win32_rt::config::app_config().app_name;
    let log_path = BOOT_LOG_PATH
        .get()
        .cloned()
        .unwrap_or_else(|| log_dir_path().join("port-runner.log"));
    let body = format!(
        "{msg}\n\nLog file:\n{}",
        log_path.display()
    );
    let escaped = body.replace('\\', "\\\\").replace('"', "\\\"");
    let title = format!("{app_name} Failed to Start").replace('"', "\\\"");
    let script = format!(
        "display alert \"{title}\" message \"{escaped}\" as critical buttons {{\"OK\"}} default button \"OK\""
    );
    let _ = std::process::Command::new("/usr/bin/osascript")
        .arg("-e")
        .arg(script)
        .status();
}

fn cli_manifest_arg() -> Option<std::path::PathBuf> {
    let mut it = std::env::args_os().skip(1);
    while let Some(arg) = it.next() {
        if arg == "--" {
            break;
        }
        if arg == "--manifest" {
            return it.next().map(std::path::PathBuf::from);
        }
    }
    None
}

fn init_boot_log() {
    register_builtin_plugins();
    let explicit_manifest = cli_manifest_arg();
    let manifest = target::discover(explicit_manifest.as_deref());
    let boot_exe_norm = manifest
        .as_ref()
        .and_then(|m| m.exe.as_deref())
        .or_else(|| x86_runtime::game_plugin::primary_default_exe())
        .map(pe_image::normalize_dll_name)
        .unwrap_or_default();
    let boot_defaults = x86_runtime::game_plugin::find_plugin(
        manifest.as_ref().and_then(|m| m.id.as_deref()),
        &boot_exe_norm,
    )
    .map(|p| p.defaults());
    let log_file_name = manifest
        .as_ref()
        .and_then(|m| m.log_name.clone())
        .or_else(|| {
            boot_defaults
                .map(|d| d.log_name.to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| "port-runner.log".to_string());
    let log_dir_name = manifest
        .as_ref()
        .and_then(|m| m.log_dir.clone())
        .or_else(|| {
            boot_defaults
                .map(|d| d.log_dir.to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| win32_rt::config::app_config().log_dir_name);
    let log_dir = log_dir_path_for(&log_dir_name);
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = log_dir.join(&log_file_name);
    let prev_name = format!("{}.prev.log", log_file_name.trim_end_matches(".log"));
    let is_retry = std::env::var_os("PORT_ASLR_RETRY").is_some()
        || std::env::var_os("AW2_ASLR_RETRY").is_some()
        || std::env::var_os("PORT_LOG_APPEND").is_some()
        || std::env::var_os("AW2_LOG_APPEND").is_some();

    if !is_retry {
        if std::fs::metadata(&log_path).is_ok_and(|m| m.len() > 0) {
            let _ = std::fs::copy(&log_path, log_dir.join(prev_name));
        }
    }

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(!is_retry)
        .append(is_retry)
        .open(&log_path)
        .ok();

    let tmp_link = std::path::PathBuf::from("/tmp").join(&log_file_name);
    let _ = std::fs::remove_file(&tmp_link);
    #[cfg(unix)]
    let _ = std::os::unix::fs::symlink(&log_path, &tmp_link);

    let _ = BOOT_LOG_PATH.set(log_path.clone());
    std::env::set_var("PORT_LOG_APPEND", "1");
    std::env::set_var("AW2_LOG_APPEND", "1");

    // CLI inspection / offline prewarm modes must keep stdout/stderr connected to the
    // caller (even when piped or launched as a subprocess where isatty(2) == 0).
    let cli_stdout_mode = std::env::args().any(|a| {
        matches!(
            a.as_str(),
            "--prewarm-aot"
                | "--prewarm-shaders"
                | "--emit-static-o"
                | "--aot-to-o"
                | "--list-exports"
                | "--ops-report"
                | "--no-run"
                | "--json"
                | "--diff-test"
                | "--call-export"
                | "-h"
                | "--help"
        )
    });

    let child_stdio_mode = std::env::var_os("PORT_CHILD_STDIO").is_some_and(|v| v == "1");

    unsafe {
        let is_tty = libc::isatty(2) != 0;
        let cwd_root = std::env::current_dir().is_ok_and(|p| p == std::path::Path::new("/"));
        if cwd_root {
            let _ = std::env::set_current_dir(&log_dir);
        }
        if let Some(f) = file {
            if !is_tty && !cli_stdout_mode && !child_stdio_mode {
                use std::os::fd::AsRawFd;
                libc::dup2(f.as_raw_fd(), 1);
                libc::dup2(f.as_raw_fd(), 2);
            } else if let Ok(dup_f) = f.try_clone() {
                let _ = BOOT_LOG_TEE.set(std::sync::Mutex::new(dup_f));
            }
        }
    }

    let os_ver = sysctl_str("kern.osproductversion");
    let os_build = sysctl_str("kern.osversion");
    let hw_model = sysctl_str("hw.model");
    let exe = std::env::current_exe().ok();
    let cwd = std::env::current_dir().ok();
    let argv: Vec<String> = std::env::args().collect();
    let banner = manifest
        .as_ref()
        .and_then(|m| m.name.as_deref())
        .or_else(|| boot_defaults.map(|d| d.app_name).filter(|s| !s.is_empty()))
        .unwrap_or("port-runner");
    boot_log_line(&format!(
        "=== {banner} startup (pid={} ppid={} arch={} rosetta={} macOS={} ({}) model={} retry={:?}) ===",
        std::process::id(),
        unsafe { libc::getppid() },
        std::env::consts::ARCH,
        translated(),
        os_ver,
        os_build,
        hw_model,
        std::env::var("AW2_ASLR_RETRY").ok(),
    ));
    boot_log_line(&format!("exe={exe:?} cwd={cwd:?} argv={argv:?}"));
    boot_log_line(&format!("log_file={}", log_path.display()));

    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let bt = std::backtrace::Backtrace::force_capture();
        let thread = std::thread::current();
        let tname = thread.name().unwrap_or("<unnamed>");
        let msg = format!("PANIC on thread '{tname}': {info}\nBacktrace:\n{bt}");
        boot_log_line(&msg);
        write_last_crash(&msg);
        eprintln!("{msg}");
        prev_hook(info);
    }));
}

/// Feed the x86-64 interpreter profile (`x86_runtime::emu_prof`) into hitch reports
/// and the periodic `out/hitch/emu-prof-<pid>.log`.
fn register_emu_profiler() {
    use win32_rt::hitch::{FuncStartFn, SymbolizeFn};
    use x86_runtime::emu_prof;
    win32_rt::crash::set_guest_state_hook(x86_runtime::emu::current_guest_state);
    win32_rt::kernel32::memory::set_code_region_hook(x86_runtime::emu::note_code_region);
    if emu_prof::level() == 0 {
        return;
    }
    fn frame(out: &mut String, fs: FuncStartFn<'_>, sym: SymbolizeFn<'_>) {
        emu_prof::append_frame_report(out, &emu_prof::Resolver { func_start: fs, symbolize: sym });
    }
    fn cumulative(out: &mut String, fs: FuncStartFn<'_>, sym: SymbolizeFn<'_>) {
        emu_prof::append_cumulative_report(out, &emu_prof::Resolver { func_start: fs, symbolize: sym });
    }
    win32_rt::hitch::set_cpu_profiler(win32_rt::hitch::CpuProfilerHooks {
        frame_report: frame,
        cumulative_report: cumulative,
        on_frame: emu_prof::on_frame,
    });
}

fn bridge_port_env_vars() {
    // Mirror universal PORT_* env vars into legacy AW2_* aliases consumed by
    // lower-level crates (`win32-rt`, `dxil-metal`, `loader.rs`, `symbols.rs`).
    for (primary, legacy) in [
        ("PORT_AOT_DIR", "AW2_DLL_AOT_DIR"),
        ("PORT_BUILTIN_OVERRIDES", "AW2_BUILTIN_OVERRIDES"),
        ("PORT_HEADLESS", "AW2_HEADLESS"),
        ("PORT_SYMBOLS", "AW2_SYMBOLS"),
        ("PORT_AOT_DYLIB", "AW2_AOT_DYLIB"),
        ("PORT_V8_JITLESS", "AW2_V8_JITLESS"),
        ("PORT_GAME_DIR", "AW2_GAME_DIR"),
        ("PORT_SHADER_CACHE", "AW2_SHADER_CACHE"),
        ("PORT_PSO_CACHE", "AW2_PSO_CACHE"),
        ("PORT_PSO_MAX_ARCHIVES", "AW2_PSO_MAX_ARCHIVES"),
        ("PORT_SHADER_MANIFEST", "AW2_SHADER_MANIFEST"),
        ("PORT_PSO_MANIFEST", "AW2_PSO_MANIFEST"),
        ("PORT_SHADERD", "AW2_SHADERD"),
        ("PORT_REGISTRY_TSV", "AW2_REGISTRY_TSV"),
    ] {
        if let Some(val) = std::env::var_os(primary).filter(|v| !v.is_empty()) {
            std::env::set_var(legacy, val);
        }
    }
}

fn print_aot_prewarm_summary(reports: &[x86_runtime::emu::AotPrewarmReport]) {
    println!("=== Offline AOT Block-Cache Prewarm Summary ===");
    if reports.is_empty() {
        println!("  (no mapped PE modules)");
        return;
    }
    println!(
        "  {:<32} {:>8} {:>9} {:>12}  {}",
        "MODULE", "ROOTS", "BLOCKS", "CODE BYTES", "CACHE PATH / STATUS"
    );
    println!("  {}", "-".repeat(100));
    let mut prewarmed = 0usize;
    let mut skipped = 0usize;
    let mut total_roots = 0usize;
    let mut total_blocks = 0usize;
    let mut total_bytes = 0usize;
    for r in reports {
        total_roots += r.roots;
        total_blocks += r.blocks;
        total_bytes += r.code_bytes;
        let status = if let Some(ref p) = r.cache_path {
            prewarmed += 1;
            p.display().to_string()
        } else {
            skipped += 1;
            format!(
                "SKIPPED ({})",
                r.skipped_reason.as_deref().unwrap_or("unknown reason")
            )
        };
        println!(
            "  {:<32} {:>8} {:>9} {:>12}  {}",
            r.module, r.roots, r.blocks, r.code_bytes, status
        );
    }
    println!("  {}", "-".repeat(100));
    println!(
        "  Total: {} module(s) prewarmed, {} skipped, {} root(s), {} ARM64 block(s), {} bytes ({} KiB)",
        prewarmed,
        skipped,
        total_roots,
        total_blocks,
        total_bytes,
        total_bytes >> 10
    );
}

#[cfg(target_arch = "aarch64")]
fn print_static_macho_summary(reports: &[x86_runtime::emu::static_macho::StaticMachoReport]) {
    println!("=== Static ARM64 Mach-O (.aot.o) Emission Summary ===");
    if reports.is_empty() {
        println!("  (no static Mach-O objects emitted)");
        return;
    }
    println!(
        "  {:<28} {:<8} {:>8} {:>7} {:>8} {:>10}  {}",
        "MODULE", "PLAT", "BLOCKS", "SKIP", "LINKS", "OBJ BYTES", "OUTPUT PATH"
    );
    println!("  {}", "-".repeat(106));
    let mut total_blocks = 0usize;
    let mut total_skipped = 0usize;
    let mut total_links = 0usize;
    let mut total_bytes = 0usize;
    for r in reports {
        total_blocks += r.emitted_blocks;
        total_skipped += r.skipped_reloc_blocks;
        total_links += r.linked_direct;
        total_bytes += r.object_bytes;
        println!(
            "  {:<28} {:<8} {:>8} {:>7} {:>8} {:>10}  {}",
            r.module_name,
            r.platform,
            r.emitted_blocks,
            r.skipped_reloc_blocks,
            r.linked_direct,
            r.object_bytes,
            r.output_path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "<memory>".into())
        );
    }
    println!("  {}", "-".repeat(106));
    println!(
        "  Total: {} object(s), {} ARM64 block(s) ({} skipped reloc), {} direct link(s), {} bytes ({} KiB)",
        reports.len(),
        total_blocks,
        total_skipped,
        total_links,
        total_bytes,
        total_bytes >> 10
    );
}

/// Ensure the C-ABI static AOT symbols required by linked `.aot.o` objects stay alive
/// in static archives (`libport_loader.a`) even under aggressive linker dead-stripping.
#[inline(never)]
fn anchor_static_aot_symbols() {
    let reg = x86_runtime::emu::exec_mem::port_static_aot_register_desc as *const () as usize;
    std::hint::black_box(reg);
    #[cfg(target_arch = "aarch64")]
    {
        let hx = x86_runtime::emu::port_static_aot_host_xfer as *const () as usize;
        std::hint::black_box(hx);
    }
}

fn real_main() -> anyhow::Result<()> {
    run_with_args(parse_args()?, false)
}

fn init_embedded_log() {
    let log_dir = log_dir_path();
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = log_dir.join("port-runner.log");
    if let Ok(f) = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_path)
    {
        let _ = BOOT_LOG_PATH.set(log_path);
        let _ = BOOT_LOG_TEE.set(std::sync::Mutex::new(f));
    }
}

pub(crate) fn run_embedded(args: Args) -> anyhow::Result<()> {
    register_builtin_plugins();
    init_embedded_log();
    run_with_args(args, true)
}

fn run_with_args(mut args: Args, embedded_host: bool) -> anyhow::Result<()> {
    use anyhow::{bail, Context};
    use win32_rt as w;

    register_builtin_plugins();
    anchor_static_aot_symbols();
    raise_fd_limit();
    if let Some(ref aot_in) = args.aot_to_o {
        #[cfg(target_arch = "aarch64")]
        {
            use x86_runtime::emu::static_macho::{
                emit_macho_object_from_aot_file, MachoPlatform,
            };
            let plat = MachoPlatform::parse(&args.static_platform).ok_or_else(|| {
                anyhow::anyhow!(
                    "invalid --static-platform {:?} (expected ios, ios-sim, or macos)",
                    args.static_platform
                )
            })?;
            let out_path = args.out_path.clone().unwrap_or_else(|| aot_in.with_extension("aot.o"));
            let rep = emit_macho_object_from_aot_file(aot_in, &out_path, plat)
                .map_err(anyhow::Error::msg)?;
            print_static_macho_summary(&[rep]);
            return Ok(());
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            let _ = aot_in;
            bail!("--aot-to-o requires an aarch64 host");
        }
    }
    let child_process_mode = std::env::var_os("PORT_CHILD_PROCESS").is_some_and(|v| v == "1");
    if child_process_mode {
        x86_runtime::emu::set_disk_aot_disabled(true);
        for k in [
            "PORT_AOT_DIR",
            "AW2_DLL_AOT_DIR",
            "PORT_AOT_FALLBACK_DIR",
            "PORT_AOT_DYLIB",
            "AW2_AOT_DYLIB",
            "PORT_AOT_BUILD",
            "AW2_DLL_AOT_BUILD",
            "PORT_JIT_PROFILE",
            "AW2_JIT_PROFILE",
        ] {
            std::env::remove_var(k);
        }
        std::env::set_var("PORT_AOT", "0");
        std::env::set_var("AW2_DLL_AOT", "0");
    }
    let mut temp_static_aot_stage: Option<std::path::PathBuf> = None;
    if let Some(ref out_dir) = args.emit_static_o {
        // Static signed __TEXT cannot self-patch runtime literal slots or LSE2 traps:
        // emit position-independent blocks with zero literal relocations and inline unaligned TSO.
        std::env::set_var("AW2_JIT_PLAINCALL", "0");
        std::env::set_var("PORT_JIT_LSE2_TRAP", "0");
        std::env::set_var("PORT_HEADLESS", "1");
        std::env::set_var("AW2_HEADLESS", "1");
        if args.aot_dir.is_none() {
            let stage = out_dir.join(format!(".aot-stage-{}", std::process::id()));
            std::fs::create_dir_all(&stage)?;
            std::env::set_var("PORT_AOT_DIR", &stage);
            temp_static_aot_stage = Some(stage);
        }
    }
    if !child_process_mode {
        if let Some(ref dir) = args.aot_dir {
            std::env::set_var("PORT_AOT_DIR", dir);
        }
    }
    let (manifest, manifest_dir) = match target::discover_with_path(args.manifest_path.as_deref()) {
        Some((m, path)) => (Some(m), path.parent().map(|d| d.to_path_buf())),
        None => (None, None),
    };
    if let Some(m) = &manifest {
        if !args.explicit_exe {
            if let Some(exe) = &m.exe {
                args.exe = exe.clone();
            }
        }
        if let Some(app_id) = &m.steam_app_id {
            if std::env::var_os("STEAM_APPID").is_none() {
                std::env::set_var("STEAM_APPID", app_id);
                std::env::set_var("SteamAppId", app_id);
                std::env::set_var("SteamGameId", app_id);
            }
        }
        if !child_process_mode {
            if let Some(aot_dir) = &m.aot_dir {
                let trimmed = aot_dir.trim();
                if !trimmed.is_empty()
                    && args.aot_dir.is_none()
                    && std::env::var_os("PORT_AOT_DIR").is_none()
                    && std::env::var_os("AW2_DLL_AOT_DIR").is_none()
                {
                    let p = std::path::PathBuf::from(trimmed);
                    let resolved = if p.is_relative() {
                        manifest_dir.as_ref().map_or(p.clone(), |d| d.join(&p))
                    } else {
                        p
                    };
                    std::env::set_var("PORT_AOT_DIR", resolved);
                }
            }
            if let Some(aot) = &m.aot_dylibs {
                if !aot.is_empty()
                    && std::env::var_os("PORT_AOT_DYLIB").is_none()
                    && std::env::var_os("AW2_AOT_DYLIB").is_none()
                {
                    std::env::set_var("PORT_AOT_DYLIB", aot);
                }
            }
        }
    }
    if args.prewarm_aot || args.prewarm_shaders || args.emit_static_o.is_some() {
        std::env::set_var("PORT_HEADLESS", "1");
        std::env::set_var("AW2_HEADLESS", "1");
    } else if args.list_exports || args.call_export.is_some() || args.no_run {
        if std::env::var_os("PORT_HEADLESS").is_none() && std::env::var_os("AW2_HEADLESS").is_none() {
            std::env::set_var("PORT_HEADLESS", "1");
        }
    }
    if let Some(ref drive_c) = args.drive_c_dir {
        std::env::set_var("TRANSLATOR_HOME", drive_c);
        std::env::set_var("AW2_DRIVE_C", drive_c);
    }
    bridge_port_env_vars();

    if args.prewarm_shaders {
        let rep = d3d12_metal::metal::pso_manifest::prewarm_for_current_device(None, false)
            .map_err(anyhow::Error::msg)?;
        if args.json {
            #[cfg(feature = "cli")]
            {
                use port_factory::json_output::{JsonEnvelope, JsonObj};
                let data = JsonObj::new()
                    .str("device_identity", rep.device_identity.clone())
                    .bool("was_cold_device", rep.was_cold_device)
                    .usize("tier1_compiled", rep.tier1_compiled)
                    .usize("tier1_total", rep.tier1_total)
                    .usize("tier1_failed", rep.tier1_failed)
                    .usize("pso_compiled", rep.pso_compiled)
                    .usize("pso_total", rep.pso_total)
                    .usize("pso_failed", rep.pso_failed)
                    .opt_str(
                        "archive_written",
                        rep.archive_written.as_ref().map(|p| p.display().to_string()),
                    )
                    .u64("elapsed_ms", rep.elapsed_ms)
                    .build();
                run_limits::write_json_stdout(&JsonEnvelope::ok("port-runner", data).to_json_pretty());
            }
            return Ok(());
        }
        println!("=== Metal 4 Shader & PSO Prewarm Summary ===");
        println!("  Device / OS      : {}", if rep.device_identity.is_empty() { "(no manifests found)" } else { &rep.device_identity });
        println!("  Cold Device      : {}", rep.was_cold_device);
        println!("  Tier-1 Shaders   : {}/{} ready ({} failed)", rep.tier1_compiled, rep.tier1_total, rep.tier1_failed);
        println!("  Metal 4 PSOs     : {}/{} compiled ({} failed)", rep.pso_compiled, rep.pso_total, rep.pso_failed);
        if let Some(p) = &rep.archive_written {
            println!("  Archive Written  : {}", p.display());
        }
        println!("  Elapsed          : {} ms", rep.elapsed_ms);
        return Ok(());
    }

    if args.pe_path.is_none() && args.exe.is_empty() {
        return Err(usage_err("no target executable specified (pass --pe PATH, --manifest FILE, or --game-dir DIR --exe NAME)"));
    }

    let norm = pe_image::normalize_dll_name(&args.exe);
    let plugin = if args.no_game_hooks {
        None
    } else {
        x86_runtime::game_plugin::find_plugin(
            manifest.as_ref().and_then(|m| m.id.as_deref()),
            &norm,
        )
    };
    let plugin_defaults = plugin.map(|p| p.defaults());
    let preferred_base_retry = manifest
        .as_ref()
        .and_then(|m| m.preferred_base_retry)
        .or_else(|| plugin_defaults.map(|d| d.preferred_base_retry))
        .unwrap_or(false);
    if !embedded_host {
        let probe_pe_path = args
            .pe_path
            .clone()
            .unwrap_or_else(|| args.game_dir.join(&args.exe));
        let pe_win = peek_pe_preferred_window(&probe_pe_path);
        let probe_window = match pe_win {
            Some((pe_base, pe_size, false)) => {
                let aot_base = x86_runtime::emu::exec_mem::static_aot_preferred_base(&norm)
                    .or_else(|| x86_runtime::emu::aot_cached_preferred_base(&norm, pe_size));
                let prewarm_exe = args.prewarm_aot || args.emit_static_o.is_some();
                let live_run = !args.no_run && !args.list_exports && !args.ops_report;
                if preferred_base_retry || prewarm_exe || (live_run && aot_base.is_some()) {
                    let target_base = aot_base.unwrap_or(pe_base);
                    let target_size = if preferred_base_retry {
                        pe_size.max(0x0400_0000)
                    } else {
                        pe_size
                    };
                    Some((target_base, target_size))
                } else {
                    None
                }
            }
            Some(_) => None,
            None => preferred_base_retry.then_some((0x1_4000_0000, 0x0400_0000)),
        };
        ensure_avx_env(&args, probe_window);
    }
    {
        use std::io::Write;
        let child_stdio_mode = std::env::var_os("PORT_CHILD_STDIO").is_some_and(|v| v == "1");
        let child_log_stderr = std::env::var_os("PORT_CHILD_LOG_STDERR").is_some_and(|v| v == "1");
        let mut builder = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
        builder.format(move |buf, record| {
            let ts = buf.timestamp_millis();
            let line = format!("[{ts} {:5} {}] {}\n", record.level(), record.target(), record.args());
            if let Some(m) = BOOT_LOG_TEE.get() {
                if let Ok(mut f) = m.lock() {
                    let _ = f.write_all(line.as_bytes());
                }
            }
            if child_stdio_mode && !child_log_stderr {
                return Ok(());
            }
            buf.write_all(line.as_bytes())
        });
        let _ = builder.try_init();
    }
    if let Ok(cur_exe) = std::env::current_exe() {
        w::process::set_runner_exe(cur_exe);
    }
    w::process::init_trusted_host_env();
    w::process::lock_trusted_runner();
    w::process::init_exit_code_channel();
    if child_process_mode {
        log::info!(
            "port-runner: child-process mode — disk-sourced native AOT caches and dylibs disabled (using in-memory translator)"
        );
    }
    raise_fd_limit();

    x86_runtime::exe::set_gs_reader(w::teb::gs_read);
    #[cfg(not(target_arch = "x86_64"))]
    if let Some((ts, te)) = crate::builtin_image::host_text_range() {
        x86_runtime::exe::set_host_text_range(ts, te);
    }
    w::set_guest_dispatcher(x86_runtime::exe::call_va8);
    w::set_native_va_registrar(x86_runtime::exe::register_native_va);
    for d in w::builtin_dlls() {
        for e in d.exports {
            let p = e.ptr.0 as usize;
            if crate::builtin_image::is_host_code(p) {
                x86_runtime::exe::register_native_va(p as u64, p);
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        let leaf7 = std::arch::x86_64::__cpuid_count(7, 0);
        log::info!(
            "port-runner: arch=x86_64 Rosetta={} AVX2={} ROSETTA_ADVERTISE_AVX={:?}",
            translated(),
            leaf7.ebx & (1 << 5) != 0,
            std::env::var("ROSETTA_ADVERTISE_AVX").ok()
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        log::info!(
            "port-runner: arch={} Rosetta={} (native Apple Silicon)",
            std::env::consts::ARCH,
            translated()
        );
    }

    let game_dir = args.game_dir.canonicalize().with_context(|| format!("game dir {}", args.game_dir.display()))?;
    let exe_host = match &args.pe_path {
        Some(p) => p.canonicalize().with_context(|| format!("pe path {}", p.display()))?,
        None => game_dir.join(&args.exe),
    };
    if !exe_host.is_file() {
        bail!("{} not found", exe_host.display());
    }
    let exe_stem = std::path::Path::new(&args.exe)
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("Game")
        .to_string();
    let display_name = manifest
        .as_ref()
        .and_then(|m| m.name.clone())
        .or_else(|| {
            plugin_defaults
                .map(|d| d.app_name.to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| exe_stem.clone());
    let game_win_dir = manifest
        .as_ref()
        .and_then(|m| m.win_dir.clone())
        .unwrap_or_else(|| format!("C:\\Games\\{exe_stem}"));
    let computer_name = manifest
        .as_ref()
        .and_then(|m| m.computer_name.clone())
        .or_else(|| {
            plugin_defaults
                .map(|d| d.computer_name.to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| "WIN64PORT".into());
    let log_dir_name = manifest
        .as_ref()
        .and_then(|m| m.log_dir.clone())
        .or_else(|| {
            plugin_defaults
                .map(|d| d.log_dir.to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| "Translator".into());
    let legacy_remedy_migration = manifest
        .as_ref()
        .and_then(|m| m.legacy_remedy_migration)
        .or_else(|| plugin_defaults.map(|d| d.legacy_remedy_migration))
        .unwrap_or(false);
    let v8_jitless = manifest
        .as_ref()
        .and_then(|m| m.v8_jitless)
        .or_else(|| plugin_defaults.map(|d| d.v8_jitless))
        .unwrap_or(false);
    w::config::set_app_config(w::config::AppConfig {
        app_name: display_name,
        computer_name,
        log_dir_name,
        legacy_remedy_migration,
    });
    if let Some(m) = &manifest {
        w::middleware::configure_from_manifest(m);
    }
    let env_override: Option<String> = if args.no_builtin_overrides {
        Some("0".to_string())
    } else {
        std::env::var("PORT_BUILTIN_OVERRIDES")
            .or_else(|_| std::env::var("AW2_BUILTIN_OVERRIDES"))
            .ok()
            .filter(|v| !v.is_empty())
    };
    let plugin_dll_routes = plugin_defaults.map_or(&[][..], |d| d.dll_routes);
    let routing = match &manifest {
        Some(m) => m.routing_table(plugin_dll_routes, env_override.as_deref()),
        None => port_manifest::GameManifest::default()
            .routing_table(plugin_dll_routes, env_override.as_deref()),
    };
    if let Some(p) = plugin {
        p.on_pre_load();
    }
    live_ctl::install();
    setup_filesystem(&game_dir, &game_win_dir, &args)?;
    if let Some(child_cwd) = std::env::var("PORT_CHILD_CWD").ok().filter(|s| !s.is_empty()) {
        w::path::set_cwd(&child_cwd);
    }
    std::env::remove_var("PORT_CHILD_CWD");

    // Process-wide Windows state.
    let exe_win = std::env::var("PORT_CHILD_EXE_WIN")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("{game_win_dir}\\{}", args.exe));
    std::env::remove_var("PORT_CHILD_EXE_WIN");
    let cmdline = if let Some(c) = std::env::var("PORT_CHILD_CMDLINE").ok().filter(|s| !s.is_empty()) {
        c
    } else {
        let mut c = format!("\"{exe_win}\"");
        for a in &args.game_args {
            if a.contains(' ') {
                c.push_str(&format!(" \"{a}\""));
            } else {
                c.push(' ');
                c.push_str(a);
            }
        }
        c
    };
    std::env::remove_var("PORT_CHILD_CMDLINE");
    w::process::init(w::process::ProcessConfig { exe_path: exe_win.clone(), cmdline: cmdline.clone() });
    if let Ok(encoded_env) = std::env::var("PORT_CHILD_ENV") {
        std::env::remove_var("PORT_CHILD_ENV");
        if let Some(vars) = w::process::decode_env_list(&encoded_env) {
            w::process::replace_env(vars);
        } else {
            w::process::init_env(&[]);
        }
    } else {
        w::process::init_env(&[]);
    }
    let cwd_for_peb = format!("{}\\", w::path::cwd().trim_end_matches('\\'));
    unsafe { w::teb::set_process_parameters(&exe_win, &cmdline, &cwd_for_peb, w::process::env_block_w()) };
    w::teb::map_kuser_shared_data();

    let l = loader::init_with_routing(
        game_dir.clone(),
        game_win_dir.clone(),
        args.trace,
        routing,
        v8_jitless,
    );
    signals::install_handlers();
    register_emu_profiler();

    // Map the target PE (.exe or .dll) and, recursively, all statically imported DLLs.
    let pe_data = std::fs::read(&exe_host)?;
    #[cfg(feature = "cli")]
    let mut ops_report_json: Option<port_factory::json_output::JsonValue> = None;
    if args.ops_report {
        #[cfg(feature = "cli")]
        {
            let prof = port_factory::scanner::scan_pe_bytes(&norm, &pe_data)?;
            if args.json {
                ops_report_json = Some(port_factory::json_output::binary_profile_json(&prof, 8));
            } else {
                print!("{}", port_factory::scanner::format_profile_report(&prof, 8));
            }
        }
        #[cfg(not(feature = "cli"))]
        {
            bail!("--ops-report requires building `port-loader` with the `cli` feature enabled");
        }
    }
    let t0 = std::time::Instant::now();
    let exe = l.load_pe(&exe_host, &norm, exe_win.clone(), None)?;
    #[cfg(feature = "cli")]
    let load_elapsed_ms = t0.elapsed().as_millis() as u64;
    // fndb symbol tables for symbolize()/crash reports (background parse; skipped in offline prewarm).
    if !args.prewarm_aot && args.emit_static_o.is_none() {
        symbols::preload();
    }
    let pe = pe_image::Pe::parse(&pe_data)?;
    let h = &pe.headers;
    x86_runtime::exe::set_preferred_base(h.image_base, h.size_of_image as u64);
    x86_runtime::exe::set_base(exe.base);
    unsafe { w::teb::set_image_base(exe.base, h.subsystem) };
    w::thread::DEFAULT_STACK_SIZE.store((h.stack_reserve as usize).max(0x10_0000), std::sync::atomic::Ordering::Relaxed);
    let mods = l.modules();
    let pe_mod_count = mods.iter().filter(|m| !m.is_builtin()).count();
    let builtin_mod_count = mods.iter().filter(|m| m.is_builtin()).count();
    log::info!(
        "loaded {} modules ({} PE, {} builtin) in {:?}; {} unresolved imports have trap thunks",
        mods.len(),
        pe_mod_count,
        builtin_mod_count,
        t0.elapsed(),
        l.traps.load(std::sync::atomic::Ordering::Relaxed)
    );

    if args.list_exports {
        let exports = l.export_list(&exe);
        if args.json {
            #[cfg(feature = "cli")]
            {
                use port_factory::json_output::{JsonEnvelope, JsonObj, JsonValue};
                let items: Vec<JsonValue> = exports
                    .into_iter()
                    .map(|(ord, name, addr)| {
                        JsonObj::new()
                            .u64("ordinal", ord as u64)
                            .str("name", name)
                            .str("address", format!("0x{addr:x}"))
                            .str("rva", format!("0x{:x}", addr.saturating_sub(exe.base)))
                            .build()
                    })
                    .collect();
                let data = JsonObj::new()
                    .str("target", norm.clone())
                    .str("base", format!("0x{:x}", exe.base))
                    .usize("exports_count", items.len())
                    .val("exports", JsonValue::Array(items))
                    .build();
                run_limits::write_json_stdout(&JsonEnvelope::ok("port-runner", data).to_json_pretty());
            }
            return Ok(());
        }
        println!("=== Exports of {} ({} entries, base=0x{:x}) ===", norm, exports.len(), exe.base);
        for (ord, name, addr) in exports {
            println!("  #{:<5} 0x{:012x} (rva 0x{:08x})  {}", ord, addr, addr - exe.base, name);
        }
        return Ok(());
    }

    let active_replacements = plugin.and_then(|p| p.replacements());
    if let Some(ref reps) = active_replacements {
        let n = hooks::apply_registry(exe.base, reps)?;
        log::info!("{n} replacement hooks applied");
    } else {
        hooks::init_generic_hooks(exe.base);
        aot_dylib::load_aot_dylibs(&mods);
        log::info!(
            "module {norm} (base=0x{:x}, preferred=0x{:x}): running via hybrid AOT + x86-64 -> ARM64 translator",
            exe.base,
            h.image_base
        );
    }
    let prewarm_mode = args.prewarm_aot || args.emit_static_o.is_some();
    if prewarm_mode {
        let bundled = l.prewarm_map_bundled_plugins(&exe_host, manifest.as_ref());
        if !bundled.is_empty() {
            log::info!(
                "prewarm: mapped {} bundled plugin(s) without running TLS/DllMain",
                bundled.len()
            );
        }
    }
    if let Some(ref out_dir) = args.emit_static_o {
        #[cfg(target_arch = "aarch64")]
        {
            use x86_runtime::emu::static_macho::{
                emit_macho_object_from_aot_file, MachoPlatform,
            };
            let plat = MachoPlatform::parse(&args.static_platform).ok_or_else(|| {
                anyhow::anyhow!(
                    "invalid --static-platform {:?} (expected ios, ios-sim, or macos)",
                    args.static_platform
                )
            })?;
            std::fs::create_dir_all(out_dir)?;
            let prewarm_reports = x86_runtime::emu::aot_prewarm_all_mapped(args.aot_max_funcs);
            print_aot_prewarm_summary(&prewarm_reports);
            let mut obj_reports = Vec::new();
            for r in &prewarm_reports {
                let Some(ref aot_path) = r.cache_path else {
                    continue;
                };
                let safe_stem: String = r
                    .module
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '_' })
                    .collect();
                let obj_path = out_dir.join(format!("{safe_stem}.aot.o"));
                let rep = emit_macho_object_from_aot_file(aot_path, &obj_path, plat)
                    .map_err(anyhow::Error::msg)?;
                obj_reports.push(rep);
            }
            if let Some(ref stage) = temp_static_aot_stage {
                let _ = std::fs::remove_dir_all(stage);
            }
            print_static_macho_summary(&obj_reports);
            if obj_reports.is_empty() {
                bail!("--emit-static-o: no modules produced static Mach-O objects");
            }
            return Ok(());
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            let _ = (out_dir, temp_static_aot_stage);
            bail!("--emit-static-o requires an aarch64 host");
        }
    }
    if args.prewarm_aot {
        let reports = x86_runtime::emu::aot_prewarm_all_mapped(args.aot_max_funcs);
        if args.json {
            #[cfg(feature = "cli")]
            {
                use port_factory::json_output::{JsonEnvelope, JsonObj, JsonValue};
                let prewarmed = reports.iter().filter(|r| r.cache_path.is_some()).count();
                let skipped = reports.len().saturating_sub(prewarmed);
                let total_roots: usize = reports.iter().map(|r| r.roots).sum();
                let total_blocks: usize = reports.iter().map(|r| r.blocks).sum();
                let total_bytes: usize = reports.iter().map(|r| r.code_bytes).sum();
                let modules: Vec<JsonValue> = reports
                    .iter()
                    .map(|r| {
                        JsonObj::new()
                            .str("module", r.module.clone())
                            .usize("roots", r.roots)
                            .usize("blocks", r.blocks)
                            .usize("code_bytes", r.code_bytes)
                            .opt_str(
                                "cache_path",
                                r.cache_path.as_ref().map(|p| p.display().to_string()),
                            )
                            .opt_str("skipped_reason", r.skipped_reason.clone())
                            .build()
                    })
                    .collect();
                let data = JsonObj::new()
                    .usize("modules_prewarmed", prewarmed)
                    .usize("modules_skipped", skipped)
                    .usize("total_roots", total_roots)
                    .usize("total_blocks", total_blocks)
                    .usize("total_code_bytes", total_bytes)
                    .val("modules", JsonValue::Array(modules))
                    .build();
                run_limits::write_json_stdout(&JsonEnvelope::ok("port-runner", data).to_json_pretty());
            }
            return Ok(());
        }
        print_aot_prewarm_summary(&reports);
        return Ok(());
    }
    if args.diff_test {
        let (pass, fail) = hooks::run_difftests(active_replacements.as_ref());
        let code = if fail == 0 {
            run_limits::exit_codes::OK
        } else {
            run_limits::exit_codes::VERIFICATION_FAILED
        };
        if args.json {
            #[cfg(feature = "cli")]
            {
                use port_factory::json_output::{JsonEnvelope, JsonObj};
                let data = JsonObj::new()
                    .usize("passed", pass)
                    .usize("failed", fail)
                    .build();
                let err_msg = (fail > 0).then(|| format!("{fail} differential test(s) failed"));
                run_limits::write_json_stdout(
                    &JsonEnvelope::with_status("port-runner", code, data, err_msg).to_json_pretty(),
                );
            }
            std::process::exit(code);
        }
        log::info!("--diff-test: {pass} passed, {fail} failed");
        std::process::exit(code);
    }
    hooks::apply_probes(exe.base)?;
    hooks::start_reporter();
    if let Some(p) = plugin {
        let mut install_fn = |at: usize, end: Option<usize>, target: usize| {
            hooks::install(at, end, target, libc::PROT_READ | libc::PROT_EXEC)
                .map(|patch| patch.trampoline)
                .map_err(|e| e.to_string())
        };
        let mut ctx = x86_runtime::game_plugin::GamePostMapContext {
            exe_base: exe.base,
            preferred_exe_base: h.image_base,
            audio_harness: args.audio_harness,
            install_hook: &mut install_fn,
        };
        p.on_post_map(&mut ctx).map_err(anyhow::Error::msg)?;
    }

    aw2_debug_image_mapped(exe.base, exe.size);

    if args.no_run {
        let u = l.unresolved.lock().unwrap();
        let mut per: std::collections::BTreeMap<String, Vec<String>> = Default::default();
        for s in u.iter() {
            if let Some((d, n)) = s.split_once('!') {
                per.entry(d.to_string()).or_default().push(n.to_string());
            } else {
                per.entry("unknown".to_string()).or_default().push(s.clone());
            }
        }
        let unresolved_count: usize = per.values().map(Vec::len).sum();
        let code = if unresolved_count > 0 {
            run_limits::exit_codes::UNRESOLVED_IMPORTS
        } else {
            run_limits::exit_codes::OK
        };
        if args.json {
            #[cfg(feature = "cli")]
            {
                use port_factory::json_output::{port_runner_no_run_data_json, JsonEnvelope};
                let data = port_runner_no_run_data_json(
                    &norm,
                    exe.base,
                    h.image_base,
                    mods.len(),
                    pe_mod_count,
                    builtin_mod_count,
                    load_elapsed_ms,
                    &per,
                    ops_report_json,
                );
                let err_msg = (unresolved_count > 0).then(|| {
                    format!("{unresolved_count} unresolved static import(s) across {} DLL(s)", per.len())
                });
                run_limits::write_json_stdout(
                    &JsonEnvelope::with_status("port-runner", code, data, err_msg).to_json_pretty(),
                );
            }
            if code != run_limits::exit_codes::OK {
                std::process::exit(code);
            }
            return Ok(());
        }
        for (d, ns) in &per {
            log::info!("unresolved {d} ({}): {}", ns.len(), ns.join(" "));
        }
        log::info!("--no-run: not executing");
        if unresolved_count > 0 {
            log::error!(
                "--no-run: {unresolved_count} unresolved static import(s) across {} DLL(s); exiting with code {code}",
                per.len()
            );
            std::process::exit(code);
        }
        return Ok(());
    }

    // Install `--max-frames` / `--timeout-secs` / `--json` bounded-run controller
    // before spawning any Windows guest thread.
    if args.max_frames.is_some() || args.timeout_secs.is_some() || args.json {
        run_limits::install(run_limits::RunLimitsConfig {
            target_exe: norm.clone(),
            max_frames: args.max_frames,
            timeout_secs: args.timeout_secs,
            json_mode: args.json,
        });
        run_limits::start_timeout_watchdog();
    }

    // Everything Windows-side runs on a Windows thread whose stack size comes
    // from the PE header: DLL init (dependency order), exe TLS callbacks, entry.
    let stack = (h.stack_reserve as usize).max(0x20_0000);
    // AppKit must own the process main thread on macOS: set up NSApplication before
    // any Windows code or native first-launch shader progress UI runs
    // (no-op with PORT_HEADLESS=1 / AW2_HEADLESS=1, and skipped when UIKit already
    // owns the main thread in embedded iOS host mode).
    if !embedded_host {
        w::macos::init_main_thread();
    }

    // On first launch on a new Mac or after a macOS update (0 cached MTL4Archives for this
    // GPU+OS build), compile any recorded shader/PSO manifests in parallel before starting
    // the Windows entry point — showing a native macOS progress window in GUI launches so
    // double-clicked `.app` bundles never look frozen during first-time shader compilation.
    if args.call_export.is_none() {
        shader_prewarm::run_cold_device_prewarm();
    }
    if let Some(export_spec) = args.call_export.clone() {
        let call_args = args.game_args.clone();
        let exe_mod = exe.clone();
        log::info!("starting initial thread for --call-export {export_spec:?} (stack 0x{stack:x})");
        let th = w::thread::spawn(
            Box::new(move || {
                let l = loader::get();
                if !l.run_pending_init(true) {
                    log::error!("DLL initialization failed");
                    w::process::exit_process(0xC000_0142);
                }
                let Some(fn_addr) = l.resolve_export(&exe_mod, &export_spec) else {
                    log::error!("export {export_spec:?} not found in {}", exe_mod.name);
                    w::process::exit_process(2);
                };
                let mut gpr = [0u64; 8];
                let mut c_strings: Vec<std::ffi::CString> = Vec::new();
                for (i, raw) in call_args.iter().take(8).enumerate() {
                    if let Some(s) = raw.strip_prefix("s:") {
                        let cs = std::ffi::CString::new(s).unwrap_or_default();
                        gpr[i] = cs.as_ptr() as u64;
                        c_strings.push(cs);
                    } else if let Some(hex) = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
                        gpr[i] = u64::from_str_radix(hex, 16).unwrap_or(0);
                    } else if let Ok(v) = raw.parse::<u64>() {
                        gpr[i] = v;
                    } else if let Ok(v) = raw.parse::<i64>() {
                        gpr[i] = v as u64;
                    } else {
                        let cs = std::ffi::CString::new(raw.as_str()).unwrap_or_default();
                        gpr[i] = cs.as_ptr() as u64;
                        c_strings.push(cs);
                    }
                }
                log::info!(
                    "calling {}!{} @0x{:x} with args=[0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}]",
                    exe_mod.name,
                    export_spec,
                    fn_addr,
                    gpr[0],
                    gpr[1],
                    gpr[2],
                    gpr[3],
                );
                let t_call = std::time::Instant::now();
                let ret = unsafe {
                    x86_runtime::exe::call_va8(
                        fn_addr as u64,
                        gpr[0],
                        gpr[1],
                        gpr[2],
                        gpr[3],
                        gpr[4],
                        gpr[5],
                        gpr[6],
                        gpr[7],
                    )
                };
                let elapsed = t_call.elapsed();
                drop(c_strings);
                let cstr_preview = try_preview_cstr(ret as usize);
                if let Some(s) = cstr_preview {
                    println!(
                        "{}!{} returned 0x{:x} ({} / {:?}) in {:?}",
                        exe_mod.name, export_spec, ret, ret, s, elapsed
                    );
                } else {
                    println!(
                        "{}!{} returned 0x{:x} (u64={} i64={}) in {:?}",
                        exe_mod.name, export_spec, ret, ret, ret as i64, elapsed
                    );
                }
                w::process::exit_process(0)
            }),
            stack,
            false,
            true,
        )?;
        let code = w::macos::run_main_loop(&|| th.exit_code());
        w::process::exit_process(code);
    }

    log::info!("starting initial thread (stack 0x{stack:x})");
    let th = w::thread::spawn(
        Box::new(|| {
            let l = loader::get();
            if !l.run_pending_init(true) {
                log::error!("DLL initialization failed");
                w::process::exit_process(0xC000_0142); // STATUS_DLL_INIT_FAILED
            }
            let code = l.run_exe();
            log::info!("entry point returned {code}");
            w::process::exit_process(code)
        }),
        stack,
        false,
        true,
    )?;
    if embedded_host {
        // Embedded host (iOS UIApplication): UIKit owns the main thread; wait on this
        // background boot thread until the initial Windows thread finishes.
        let code = loop {
            if let Some(c) = th.exit_code() {
                break c;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        };
        log::info!("port-ios: Windows process exited with code {code}");
        w::process::record_exit_code(code);
        w::process::run_exit_hooks();
        return Ok(());
    }
    // The main thread now runs the AppKit event loop until the Windows
    // process exits (ExitProcess usually ends the process before that).
    let code = w::macos::run_main_loop(&|| th.exit_code());
    w::process::exit_process(code);
}

fn try_preview_cstr(addr: usize) -> Option<String> {
    use win32_rt::kernel32::memory::{query, MEM_COMMIT, PAGE_NOACCESS};
    let is_readable = |a: usize| -> bool {
        query(a).is_some_and(|m| m.state == MEM_COMMIT && m.protect != 0 && m.protect != PAGE_NOACCESS)
    };
    if addr < 0x10000 || !is_readable(addr) {
        return None;
    }
    let mut bytes = Vec::new();
    for i in 0..128usize {
        if !is_readable(addr + i) {
            return None;
        }
        let b = unsafe { *((addr + i) as *const u8) };
        if b == 0 {
            break;
        }
        if !(0x20..=0x7E).contains(&b) {
            return None;
        }
        bytes.push(b);
    }
    if bytes.is_empty() {
        None
    } else {
        String::from_utf8(bytes).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_prewarm_aot_flags() {
        let a = parse_args_from(
            [
                "--pe",
                "/tmp/game/sample.dll",
                "--prewarm-aot",
                "--aot-max-funcs",
                "128",
                "--aot-dir",
                "/tmp/aot-cache",
            ]
            .into_iter()
            .map(String::from),
        );
        assert!(a.prewarm_aot);
        assert_eq!(a.aot_max_funcs, 128);
        assert_eq!(a.aot_dir, Some(std::path::PathBuf::from("/tmp/aot-cache")));
        assert_eq!(a.pe_path, Some(std::path::PathBuf::from("/tmp/game/sample.dll")));
        assert_eq!(a.game_dir, std::path::PathBuf::from("/tmp/game"));
        assert_eq!(a.exe, "sample.dll");
    }

    #[test]
    fn parse_prewarm_shaders_flag() {
        let a = parse_args_from(["--prewarm-shaders"].into_iter().map(String::from));
        assert!(a.prewarm_shaders);
        assert!(!a.prewarm_aot);
    }

    #[test]
    fn parse_static_macho_flags() {
        let a = parse_args_from(
            [
                "--pe",
                "/tmp/game/sample.dll",
                "--emit-static-o",
                "/tmp/ios-aot",
                "--static-platform",
                "ios-sim",
            ]
            .into_iter()
            .map(String::from),
        );
        assert_eq!(a.emit_static_o, Some(std::path::PathBuf::from("/tmp/ios-aot")));
        assert_eq!(a.static_platform, "ios-sim");

        let b = parse_args_from(
            [
                "--aot-to-o",
                "/tmp/sample.aot",
                "--out",
                "/tmp/sample.aot.o",
                "--static-platform",
                "macos",
            ]
            .into_iter()
            .map(String::from),
        );
        assert_eq!(b.aot_to_o, Some(std::path::PathBuf::from("/tmp/sample.aot")));
        assert_eq!(b.out_path, Some(std::path::PathBuf::from("/tmp/sample.aot.o")));
        assert_eq!(b.static_platform, "macos");
    }

    #[test]
    fn parse_no_game_hooks_and_legacy_alias() {
        let a = parse_args_from(["--no-game-hooks"].into_iter().map(String::from));
        assert!(a.no_game_hooks);
        let b = parse_args_from(["--no-aw2-hooks"].into_iter().map(String::from));
        assert!(b.no_game_hooks);
        let c = parse_args_from(["--no-builtin-overrides"].into_iter().map(String::from));
        assert!(c.no_builtin_overrides);
        assert!(!c.no_game_hooks);
    }

    #[test]
    fn bridge_port_env_vars_populates_legacy_aliases() {
        let prev_port = std::env::var_os("PORT_AOT_DIR");
        let prev_legacy = std::env::var_os("AW2_DLL_AOT_DIR");
        std::env::set_var("PORT_AOT_DIR", "/tmp/port-bridge-aot");
        std::env::remove_var("AW2_DLL_AOT_DIR");
        bridge_port_env_vars();
        assert_eq!(
            std::env::var_os("AW2_DLL_AOT_DIR"),
            Some(std::ffi::OsString::from("/tmp/port-bridge-aot"))
        );
        match prev_port {
            Some(v) => std::env::set_var("PORT_AOT_DIR", v),
            None => std::env::remove_var("PORT_AOT_DIR"),
        }
        match prev_legacy {
            Some(v) => std::env::set_var("AW2_DLL_AOT_DIR", v),
            None => std::env::remove_var("AW2_DLL_AOT_DIR"),
        }
    }

    #[test]
    fn parse_json_and_run_limit_flags() {
        let a = try_parse_args_from(
            [
                "--pe",
                "/tmp/game/app.exe",
                "--json",
                "--timeout-secs",
                "12.5",
                "--max-frames",
                "60",
            ]
            .into_iter()
            .map(String::from),
        )
        .unwrap();
        assert!(a.json);
        assert_eq!(a.timeout_secs, Some(12.5));
        assert_eq!(a.max_frames, Some(60));

        // Reject 0 / negative / non-numeric values as usage errors.
        assert!(try_parse_args_from(["--timeout-secs", "0"].into_iter().map(String::from)).is_err());
        assert!(
            try_parse_args_from(["--timeout-secs", "-5"].into_iter().map(String::from)).is_err()
        );
        assert!(
            try_parse_args_from(["--timeout-secs", "nan"].into_iter().map(String::from)).is_err()
        );
        assert!(try_parse_args_from(["--max-frames", "0"].into_iter().map(String::from)).is_err());
        assert!(try_parse_args_from(["--max-frames", "abc"].into_iter().map(String::from)).is_err());
    }
}
