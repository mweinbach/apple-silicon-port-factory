//! Process-wide state: command line, environment block, exit.

use std::sync::{Mutex, OnceLock};

pub struct ProcessConfig {
    /// Windows path of the main executable, e.g. `C:\Games\AlanWake2\AlanWake2.exe`.
    pub exe_path: String,
    /// Full Windows command line (exe path quoted + args).
    pub cmdline: String,
}

static CONFIG: OnceLock<ProcessConfig> = OnceLock::new();
static MANIFEST_GAME_ARGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[cfg(not(target_os = "ios"))]
pub const CS_CDHASH_LEN: usize = 20;
#[cfg(not(target_os = "ios"))]
const CS_OPS_STATUS: libc::c_uint = 0;
#[cfg(not(target_os = "ios"))]
const CS_OPS_CDHASH: libc::c_uint = 5;
#[cfg(not(target_os = "ios"))]
const CS_VALID: u32 = 0x0000_0001;
#[cfg(not(target_os = "ios"))]
const CS_KILLED: u32 = 0x0100_0000;
#[cfg(not(target_os = "ios"))]
const POSIX_SPAWN_START_SUSPENDED: libc::c_short = 0x0080;
#[cfg(not(target_os = "ios"))]
const POSIX_SPAWN_CLOEXEC_DEFAULT: libc::c_short = 0x4000;

#[cfg(not(target_os = "ios"))]
extern "C" {
    fn csops(
        pid: libc::pid_t,
        ops: libc::c_uint,
        useraddr: *mut std::ffi::c_void,
        usersize: libc::size_t,
    ) -> libc::c_int;
}

/// Kernel-backed macOS code-signing identity (`csops`) of a trusted native executable.
#[cfg(not(target_os = "ios"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunnerCodeIdentity {
    pub cdhash: [u8; CS_CDHASH_LEN],
    pub cs_flags: u32,
}

/// Immutable pinned identity of the native host runner used by `CreateProcessW`.
#[cfg(not(target_os = "ios"))]
#[derive(Clone, Debug)]
pub struct TrustedRunner {
    pub exe_path: std::path::PathBuf,
    pub identity: RunnerCodeIdentity,
    pub dev_ino: (u64, u64),
}

#[cfg(not(target_os = "ios"))]
struct TrustedRunnerState {
    runner: Option<TrustedRunner>,
    locked: bool,
}

#[cfg(not(target_os = "ios"))]
static TRUSTED_RUNNER: Mutex<TrustedRunnerState> = Mutex::new(TrustedRunnerState {
    runner: None,
    locked: false,
});

#[cfg(target_os = "ios")]
static RUNNER_EXE_IOS: Mutex<Option<std::path::PathBuf>> = Mutex::new(None);

/// Minimal platform-necessary host environment keys allowed to propagate from the
/// immutable startup host environment into child processes spawned by `CreateProcessW`
/// or host helper tools. Strictly excludes loader/dylib/JIT/AOT/debug override knobs
/// (e.g. `DYLD_*`, `LD_*`, `PORT_*`, `AW2_*`, `METAL_*`, arbitrary host secrets or markers).
const TRUSTED_CHILD_HOST_ENV_KEYS: &[&str] = &[
    "HOME",
    "USER",
    "LOGNAME",
    "TMPDIR",
    "PATH",
    "LANG",
    "LC_ALL",
    "LC_CTYPE",
    "TZ",
    "TERM",
    "__CF_USER_TEXT_ENCODING",
    "TRANSLATOR_HOME",
];

struct TrustedHostEnv {
    child_env: Vec<(String, String)>,
    user: String,
    headless: bool,
    #[cfg_attr(target_os = "ios", allow(dead_code))]
    runner_exe_override: Option<std::path::PathBuf>,
    aot_cache_dirs: Vec<std::path::PathBuf>,
    port_env_overrides: Vec<(String, String)>,
}

static TRUSTED_HOST_ENV: OnceLock<TrustedHostEnv> = OnceLock::new();

fn capture_trusted_host_env_from<I>(iter: I) -> TrustedHostEnv
where
    I: IntoIterator<Item = (String, String)>,
{
    let mut child_env = Vec::new();
    let mut user: Option<String> = None;
    let mut home: Option<std::path::PathBuf> = None;
    let mut headless = false;
    let mut runner_exe_override: Option<std::path::PathBuf> = None;
    let mut aot_cache_dirs = Vec::new();
    let mut port_env_overrides = Vec::new();

    for (k, v) in iter {
        if TRUSTED_CHILD_HOST_ENV_KEYS.contains(&k.as_str()) {
            if k == "USER" && !v.is_empty() {
                user = Some(v.clone());
            }
            if k == "HOME" && !v.is_empty() {
                home = Some(std::path::PathBuf::from(&v));
            }
            child_env.push((k, v));
            continue;
        }
        if (k == "PORT_HEADLESS" || k == "AW2_HEADLESS") && v == "1" {
            headless = true;
            continue;
        }
        if k == "PORT_RUNNER_EXE" {
            let trimmed = v.trim();
            if !trimmed.is_empty() {
                runner_exe_override = Some(std::path::PathBuf::from(trimmed));
            }
            continue;
        }
        if matches!(k.as_str(), "PORT_AOT_DIR" | "AW2_DLL_AOT_DIR" | "PORT_AOT_FALLBACK_DIR") {
            let trimmed = v.trim();
            if !trimmed.is_empty() {
                aot_cache_dirs.push(std::path::PathBuf::from(trimmed));
            }
            continue;
        }
        if let Some(name) = k.strip_prefix("PORT_ENV_").or_else(|| k.strip_prefix("AW2ENV_")) {
            port_env_overrides.push((name.to_string(), v));
        }
    }

    if let Some(ref h) = home {
        aot_cache_dirs.push(h.join("Library/Caches/port-factory/aot"));
        aot_cache_dirs.push(h.join("Library/Caches/aw2-port/aot"));
    }

    child_env.sort_by(|a, b| a.0.cmp(&b.0));
    child_env.dedup_by(|a, b| a.0 == b.0);
    TrustedHostEnv {
        child_env,
        user: user.unwrap_or_else(|| "player".into()),
        headless,
        runner_exe_override,
        aot_cache_dirs,
        port_env_overrides,
    }
}

fn trusted_host_env() -> &'static TrustedHostEnv {
    TRUSTED_HOST_ENV.get_or_init(|| capture_trusted_host_env_from(std::env::vars()))
}

/// Snapshot the minimal trusted host environment once at startup before any guest code runs,
/// and protect the current native host executable and host AOT cache roots from guest VFS mutation.
pub fn init_trusted_host_env() {
    let env = trusted_host_env();
    if let Ok(cur_exe) = std::env::current_exe() {
        crate::path::protect_native_runner(&cur_exe);
    }
    for dir in &env.aot_cache_dirs {
        crate::path::protect_native_subtree(dir);
    }
}

/// Return the immutable allowlisted host environment for spawning native child processes.
pub fn trusted_child_host_env() -> Vec<(String, String)> {
    trusted_host_env().child_env.clone()
}

/// Return whether the host runner was started in headless mode (`PORT_HEADLESS=1` / `AW2_HEADLESS=1`).
pub fn trusted_host_headless() -> bool {
    trusted_host_env().headless
}

#[cfg(not(target_os = "ios"))]
pub fn kill_and_reap_pid(pid: libc::pid_t) {
    if pid <= 0 {
        return;
    }
    unsafe {
        let _ = libc::kill(pid, libc::SIGKILL);
        let mut status: libc::c_int = 0;
        loop {
            let r = libc::waitpid(pid, &mut status, 0);
            if r < 0 && crate::types::last_errno() == libc::EINTR {
                continue;
            }
            break;
        }
    }
}

/// Query the kernel-backed dynamic code-signing status and 20-byte CDHash for `pid` via `csops`.
/// Fails closed unless `CS_VALID` is set, `CS_KILLED` is clear, and CDHash is non-zero.
#[cfg(not(target_os = "ios"))]
pub fn query_pid_code_identity(pid: libc::pid_t) -> Result<RunnerCodeIdentity, String> {
    let mut cs_flags: u32 = 0;
    let st_rc = unsafe {
        csops(
            pid,
            CS_OPS_STATUS,
            (&mut cs_flags as *mut u32).cast::<std::ffi::c_void>(),
            std::mem::size_of::<u32>(),
        )
    };
    if st_rc != 0 {
        return Err(format!("csops(CS_OPS_STATUS, pid={pid}) failed errno={}", crate::types::last_errno()));
    }
    if (cs_flags & CS_VALID) == 0 || (cs_flags & CS_KILLED) != 0 {
        return Err(format!("pid={pid} code signature not valid (cs_flags={cs_flags:#010x})"));
    }
    let mut cdhash = [0u8; CS_CDHASH_LEN];
    let cd_rc = unsafe {
        csops(
            pid,
            CS_OPS_CDHASH,
            cdhash.as_mut_ptr().cast::<std::ffi::c_void>(),
            cdhash.len(),
        )
    };
    if cd_rc != 0 {
        return Err(format!("csops(CS_OPS_CDHASH, pid={pid}) failed errno={}", crate::types::last_errno()));
    }
    if cdhash.iter().all(|&b| b == 0) {
        return Err(format!("pid={pid} returned all-zero CDHash"));
    }
    Ok(RunnerCodeIdentity { cdhash, cs_flags })
}

/// Verify that a suspended child process (`POSIX_SPAWN_START_SUSPENDED`) has a kernel-valid
/// code signature whose CDHash exactly matches `expected` before resuming it.
#[cfg(not(target_os = "ios"))]
pub fn verify_suspended_child_identity(
    pid: libc::pid_t,
    expected: &RunnerCodeIdentity,
) -> Result<(), String> {
    let actual = query_pid_code_identity(pid)?;
    let mut diff = 0u8;
    for (a, b) in actual.cdhash.iter().zip(expected.cdhash.iter()) {
        diff |= a ^ b;
    }
    if diff != 0 {
        return Err(format!(
            "child pid={pid} CDHash mismatch (expected {}, got {})",
            hex_cdhash(&expected.cdhash),
            hex_cdhash(&actual.cdhash)
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "ios"))]
fn hex_cdhash(bytes: &[u8; CS_CDHASH_LEN]) -> String {
    let mut s = String::with_capacity(CS_CDHASH_LEN * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

/// Capture the kernel-backed CDHash of an executable path without running any userspace code
/// in it: if it is the current process binary (same `(st_dev, st_ino)`), reads `csops(getpid())`
/// directly; otherwise spawns it once suspended (`POSIX_SPAWN_START_SUSPENDED`), queries `csops`,
/// and immediately kills and reaps the probe process before returning.
#[cfg(not(target_os = "ios"))]
pub fn capture_runner_identity(path: &std::path::Path) -> Result<TrustedRunner, String> {
    use std::os::unix::fs::MetadataExt;
    let canon = std::fs::canonicalize(path)
        .map_err(|e| format!("canonicalize runner {}: {e}", path.display()))?;
    let meta = std::fs::metadata(&canon)
        .map_err(|e| format!("stat runner {}: {e}", canon.display()))?;
    if !meta.is_file() {
        return Err(format!("runner {} is not a regular file", canon.display()));
    }
    let dev_ino = (meta.dev(), meta.ino());

    let same_as_self = std::env::current_exe()
        .ok()
        .and_then(|p| std::fs::metadata(p).ok())
        .is_some_and(|m| (m.dev(), m.ino()) == dev_ino);

    let identity = if same_as_self {
        query_pid_code_identity(unsafe { libc::getpid() })?
    } else {
        probe_executable_code_identity_suspended(&canon)?
    };
    Ok(TrustedRunner { exe_path: canon, identity, dev_ino })
}

#[cfg(not(target_os = "ios"))]
fn probe_executable_code_identity_suspended(canon: &std::path::Path) -> Result<RunnerCodeIdentity, String> {
    use std::ffi::CString;
    let c_path = CString::new(canon.as_os_str().as_encoded_bytes())
        .map_err(|_| format!("runner path contains NUL: {}", canon.display()))?;
    unsafe {
        let mut attr: libc::posix_spawnattr_t = std::ptr::null_mut();
        let rc = libc::posix_spawnattr_init(&mut attr);
        if rc != 0 {
            return Err(format!("posix_spawnattr_init failed: {rc}"));
        }
        let mut actions: libc::posix_spawn_file_actions_t = std::ptr::null_mut();
        let rc = libc::posix_spawn_file_actions_init(&mut actions);
        if rc != 0 {
            libc::posix_spawnattr_destroy(&mut attr);
            return Err(format!("posix_spawn_file_actions_init failed: {rc}"));
        }
        let setup_res = (|| -> Result<(), String> {
            let flags = POSIX_SPAWN_START_SUSPENDED | POSIX_SPAWN_CLOEXEC_DEFAULT;
            let rc = libc::posix_spawnattr_setflags(&mut attr, flags);
            if rc != 0 {
                return Err(format!("posix_spawnattr_setflags failed: {rc}"));
            }
            for (fd, mode) in [(0, libc::O_RDONLY), (1, libc::O_WRONLY), (2, libc::O_WRONLY)] {
                let rc = libc::posix_spawn_file_actions_addopen(
                    &mut actions,
                    fd,
                    c"/dev/null".as_ptr(),
                    mode,
                    0,
                );
                if rc != 0 {
                    return Err(format!("posix_spawn_file_actions_addopen(fd={fd}) failed: {rc}"));
                }
            }
            Ok(())
        })();
        if let Err(e) = setup_res {
            libc::posix_spawn_file_actions_destroy(&mut actions);
            libc::posix_spawnattr_destroy(&mut attr);
            return Err(e);
        }
        let argv = [c_path.as_ptr() as *mut libc::c_char, std::ptr::null_mut()];
        let envp = [std::ptr::null_mut()];
        let mut pid: libc::pid_t = 0;
        let rc = libc::posix_spawn(
            &mut pid,
            c_path.as_ptr(),
            &actions,
            &attr,
            argv.as_ptr(),
            envp.as_ptr(),
        );
        libc::posix_spawn_file_actions_destroy(&mut actions);
        libc::posix_spawnattr_destroy(&mut attr);
        if rc != 0 {
            return Err(format!("posix_spawn probe for {} failed: errno={rc}", canon.display()));
        }
        let id_res = query_pid_code_identity(pid);
        // Probe child is never resumed; kill and reap immediately.
        kill_and_reap_pid(pid);
        id_res
    }
}

/// Register and pin the host `port-runner` binary path and its kernel-backed signing identity
/// BEFORE guest code runs. Fails closed if the binary cannot be resolved or validated.
pub fn set_runner_exe(path: std::path::PathBuf) {
    #[cfg(not(target_os = "ios"))]
    {
        let mut st = TRUSTED_RUNNER.lock().unwrap_or_else(|e| e.into_inner());
        if st.locked {
            log::warn!("set_runner_exe ignored after trusted runner was locked");
            return;
        }
        match capture_runner_identity(&path) {
            Ok(tr) => {
                crate::path::protect_native_runner(&tr.exe_path);
                st.runner = Some(tr);
            }
            Err(e) => {
                log::warn!("set_runner_exe({}): failed to pin kernel code identity: {e}", path.display());
                st.runner = None;
            }
        }
    }
    #[cfg(target_os = "ios")]
    {
        crate::path::protect_native_runner(&path);
        *RUNNER_EXE_IOS.lock().unwrap_or_else(|e| e.into_inner()) = Some(path);
    }
}

/// Lock the pinned trusted runner identity immutably before guest code begins executing.
pub fn lock_trusted_runner() {
    #[cfg(not(target_os = "ios"))]
    {
        let _ = trusted_runner();
        let mut st = TRUSTED_RUNNER.lock().unwrap_or_else(|e| e.into_inner());
        st.locked = true;
    }
}

/// Return the pinned host `TrustedRunner` (canonical path + kernel-backed CDHash), capturing
/// it lazily once from startup overrides or `current_exe` (`port-runner`/`aw2-loader`) if not
/// yet locked. Returns `None` (fail closed) if no valid signed runner identity can be pinned.
#[cfg(not(target_os = "ios"))]
pub fn trusted_runner() -> Option<TrustedRunner> {
    let mut st = TRUSTED_RUNNER.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref tr) = st.runner {
        return Some(tr.clone());
    }
    if st.locked {
        return None;
    }
    let candidate = trusted_host_env()
        .runner_exe_override
        .clone()
        .or_else(|| {
            let cur = std::env::current_exe().ok()?;
            let stem = cur.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            (stem == "port-runner" || stem == "aw2-loader").then_some(cur)
        })?;
    match capture_runner_identity(&candidate) {
        Ok(tr) => {
            crate::path::protect_native_runner(&tr.exe_path);
            st.runner = Some(tr.clone());
            Some(tr)
        }
        Err(e) => {
            log::warn!("trusted_runner({}): failed to capture kernel identity: {e}", candidate.display());
            None
        }
    }
}

/// Resolve the host `port-runner` binary path for child guest process launch.
pub fn runner_exe() -> Option<std::path::PathBuf> {
    #[cfg(not(target_os = "ios"))]
    {
        trusted_runner().map(|tr| tr.exe_path)
    }
    #[cfg(target_os = "ios")]
    {
        RUNNER_EXE_IOS.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }
}

/// Replace the entire Windows environment block (used when booting a child process
/// launched via `CreateProcessW` with a custom or inherited Windows environment).
pub fn replace_env(mut vars: Vec<(String, String)>) {
    vars.sort_by_key(|(k, _)| k.to_ascii_uppercase());
    vars.dedup_by(|a, b| a.0.eq_ignore_ascii_case(&b.0));
    *INITIAL_ENV.lock().unwrap() = vars.clone();
    *ENV.lock().unwrap() = vars;
    ENV_GEN.fetch_add(1, std::sync::atomic::Ordering::Release);
    ENV_INIT_GEN.fetch_add(1, std::sync::atomic::Ordering::Release);
}

/// Length-prefixed serialization of `(key, value)` pairs for passing a Windows
/// environment block across `CreateProcessW` -> `port-runner` without temp files.
pub fn encode_env_list(vars: &[(String, String)]) -> String {
    let mut out = String::new();
    for (k, v) in vars {
        out.push_str(&format!("{}:{}:{}{}", k.len(), v.len(), k, v));
    }
    out
}

/// Decode the format produced by [`encode_env_list`].
pub fn decode_env_list(raw: &str) -> Option<Vec<(String, String)>> {
    let mut vars = Vec::new();
    let mut rest = raw;
    while !rest.is_empty() {
        let (klen_s, after_k) = rest.split_once(':')?;
        let (vlen_s, after_v) = after_k.split_once(':')?;
        let klen: usize = klen_s.parse().ok()?;
        let vlen: usize = vlen_s.parse().ok()?;
        if after_v.len() < klen.checked_add(vlen)? {
            return None;
        }
        let k = after_v.get(..klen)?.to_string();
        let v = after_v.get(klen..klen + vlen)?.to_string();
        vars.push((k, v));
        rest = after_v.get(klen + vlen..)?;
    }
    Some(vars)
}

/// Register default guest command-line arguments from a declarative `game.toml`
/// manifest (`[runtime] args = "..."`). Merged into the Windows command line by
/// [`init`] and `teb::set_process_parameters` without overriding explicit CLI flags.
pub fn set_manifest_game_args(args: Vec<String>) {
    *MANIFEST_GAME_ARGS.lock().unwrap_or_else(|e| e.into_inner()) = args;
}

/// Append any registered manifest game args onto `cmdline` unless already present.
pub fn merge_manifest_args(cmdline: &str) -> String {
    let defaults = MANIFEST_GAME_ARGS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if defaults.is_empty() {
        return cmdline.to_string();
    }
    let existing = split_cmdline(cmdline);
    let existing_args = if existing.len() > 1 { &existing[1..] } else { &[] };
    let mut out = cmdline.to_string();
    for arg in defaults {
        if arg.is_empty() {
            continue;
        }
        let already_set = if let Some((k, _)) = arg.split_once('=') {
            let prefix = format!("{k}=");
            existing_args.iter().any(|e| {
                e.len() >= prefix.len() && e[..prefix.len()].eq_ignore_ascii_case(&prefix)
            })
        } else {
            existing_args.iter().any(|e| e.eq_ignore_ascii_case(&arg))
        };
        if already_set {
            continue;
        }
        if arg.contains(' ') || arg.contains('\t') {
            out.push_str(&format!(" \"{arg}\""));
        } else {
            out.push(' ');
            out.push_str(&arg);
        }
    }
    out
}

pub fn init(mut cfg: ProcessConfig) {
    cfg.cmdline = merge_manifest_args(&cfg.cmdline);
    let _ = CONFIG.set(cfg);
    // D3D12 API trace (d3d12_metal::trace): flush buffered records at exit.
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    register_exit_hook(d3d12_metal::trace::flush);
    // Global stub registry: emit ranked stub summary table (and optional JSON/diff) at exit.
    register_exit_hook(crate::stub_registry::log_exit_summary);
}

// ---------------------------------------------------------------------------
// Exit hooks
// ---------------------------------------------------------------------------

static EXIT_HOOKS: std::sync::Mutex<Vec<fn()>> = std::sync::Mutex::new(Vec::new());
static EXIT_HOOKS_RAN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Code passed to [`exit_process`] (`u64::MAX`: not exiting through it).
static EXIT_CODE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
/// Optional fatal/abnormal exit reason recorded before exit hooks run
/// (`(exit_kind, detail)` such as `("crash", "access violation")` or `("unimplemented_import", "foo!Bar")`).
static EXIT_REASON: std::sync::Mutex<Option<(String, String)>> = std::sync::Mutex::new(None);

/// Record a structured exit kind and detail (first writer wins; used by crash reports,
/// unimplemented-import traps, and stall watchdogs before invoking exit hooks).
pub fn record_exit_reason(kind: &str, detail: impl Into<String>) {
    if let Ok(mut g) = EXIT_REASON.try_lock() {
        if g.is_none() {
            *g = Some((kind.to_string(), detail.into()));
        }
    }
}

/// Return `(exit_kind, detail)` for boot/exit reports.
pub fn exit_reason() -> (String, String) {
    if let Ok(g) = EXIT_REASON.try_lock() {
        if let Some((ref k, ref d)) = *g {
            return (k.clone(), d.clone());
        }
    }
    match exit_code() {
        Some(0) => ("clean_exit".to_string(), String::new()),
        Some(c) => ("nonzero_exit".to_string(), format!("exit code {c} (0x{c:x})")),
        None => ("unknown".to_string(), String::new()),
    }
}

/// The code the process is exiting with through [`exit_process`] (ExitProcess, CRT exit
/// paths, ...), or None (provisional crash reports before exit, or still running). For exit hooks.
pub fn exit_code() -> Option<u32> {
    let c = EXIT_CODE.load(std::sync::atomic::Ordering::Acquire);
    (c != u64::MAX).then_some(c as u32)
}

/// Record the final Windows exit code (first writer wins) before running exit hooks
/// on paths that terminate via `libc::_exit` directly (e.g. fatal signal handlers).
pub fn record_exit_code(code: u32) {
    let _ = EXIT_CODE.compare_exchange(
        u64::MAX,
        code as u64,
        std::sync::atomic::Ordering::AcqRel,
        std::sync::atomic::Ordering::Acquire,
    );
}

/// Map a 32-bit Windows exit/NTSTATUS code to an 8-bit POSIX exit status without
/// silently collapsing non-zero codes whose low byte is 0 into `0`.
pub const fn posix_exit_code(win_code: u32) -> i32 {
    if win_code == 0 {
        0
    } else {
        let low = (win_code & 0xFF) as i32;
        if low == 0 { 1 } else { low }
    }
}

/// Register a function to run once when the process ends: ExitProcess,
/// TerminateProcess(self), CRT exit paths, fatal faults and crash reports
/// (best effort). Hooks must be quick and must not call back into the game.
pub fn register_exit_hook(f: fn()) {
    let mut v = EXIT_HOOKS.lock().unwrap_or_else(|p| p.into_inner());
    if !v.iter().any(|&g| g as usize == f as usize) {
        v.push(f);
    }
}

/// Run the exit hooks (once; later calls are no-ops). Panics in a hook are
/// caught; a hook list locked by a crashing thread is skipped.
pub fn run_exit_hooks() {
    if EXIT_HOOKS_RAN.swap(true, std::sync::atomic::Ordering::AcqRel) {
        return;
    }
    call_exit_hooks();
}

/// Run the exit hooks now without consuming them (crash reports: the
/// application's unhandled-exception filter may still continue execution,
/// so the hooks must run again at the real exit).
pub fn call_exit_hooks() {
    let hooks: Vec<fn()> = match EXIT_HOOKS.try_lock() {
        Ok(v) => v.clone(),
        Err(std::sync::TryLockError::Poisoned(p)) => p.into_inner().clone(),
        Err(std::sync::TryLockError::WouldBlock) => return,
    };
    for h in hooks {
        let _ = std::panic::catch_unwind(h);
    }
}

pub fn config() -> &'static ProcessConfig {
    CONFIG.get().expect("process config not initialized")
}

pub fn try_config() -> Option<&'static ProcessConfig> {
    CONFIG.get()
}

pub fn cmdline_w() -> *mut u16 {
    static W: OnceLock<usize> = OnceLock::new();
    *W.get_or_init(|| crate::strings::leak_wide_z(&config().cmdline) as usize) as *mut u16
}

pub fn cmdline_a() -> *mut u8 {
    static A: OnceLock<usize> = OnceLock::new();
    *A.get_or_init(|| crate::strings::leak_ansi_z(&config().cmdline) as usize) as *mut u8
}

/// Split a Windows command line the way `CommandLineToArgvW` / the CRT do.
pub fn split_cmdline(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    // argv[0]: quotes toggle, no escape processing.
    let mut cur = String::new();
    let mut inq = false;
    while i < chars.len() {
        let c = chars[i];
        if c == '"' {
            inq = !inq;
        } else if (c == ' ' || c == '\t') && !inq {
            break;
        } else {
            cur.push(c);
        }
        i += 1;
    }
    args.push(cur);
    loop {
        while i < chars.len() && (chars[i] == ' ' || chars[i] == '\t') {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }
        let mut cur = String::new();
        let mut inq = false;
        while i < chars.len() {
            let c = chars[i];
            if c == '\\' {
                let mut n = 0;
                while i < chars.len() && chars[i] == '\\' {
                    n += 1;
                    i += 1;
                }
                if i < chars.len() && chars[i] == '"' {
                    cur.extend(std::iter::repeat('\\').take(n / 2));
                    if n % 2 == 1 {
                        cur.push('"');
                        i += 1;
                    }
                } else {
                    cur.extend(std::iter::repeat('\\').take(n));
                }
                continue;
            }
            if c == '"' {
                if inq && i + 1 < chars.len() && chars[i + 1] == '"' {
                    cur.push('"');
                    i += 2;
                    continue;
                }
                inq = !inq;
                i += 1;
                continue;
            }
            if (c == ' ' || c == '\t') && !inq {
                break;
            }
            cur.push(c);
            i += 1;
        }
        args.push(cur);
    }
    args
}

// ---------------------------------------------------------------------------
// Environment (Windows semantics: case-insensitive names, sorted block)
// ---------------------------------------------------------------------------

static ENV: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
static INITIAL_ENV: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());

/// Initialize the Windows environment. We do NOT pass the host environment
/// through wholesale (host paths would confuse Windows code); only a
/// Windows-looking base set plus `AW2ENV_<NAME>=value` overrides.
pub fn init_env(extra: &[(String, String)]) {
    let trusted = trusted_host_env();
    let user = trusted.user.clone();
    let computer = crate::config::app_config().computer_name;
    let mut vars: Vec<(String, String)> = vec![
        ("ALLUSERSPROFILE".into(), "C:\\ProgramData".into()),
        ("APPDATA".into(), format!("C:\\users\\{user}\\AppData\\Roaming")),
        ("CommonProgramFiles".into(), "C:\\Program Files\\Common Files".into()),
        ("CommonProgramFiles(x86)".into(), "C:\\Program Files (x86)\\Common Files".into()),
        ("CommonProgramW6432".into(), "C:\\Program Files\\Common Files".into()),
        ("COMPUTERNAME".into(), computer),
        ("ComSpec".into(), "C:\\Windows\\system32\\cmd.exe".into()),
        ("HOMEDRIVE".into(), "C:".into()),
        ("HOMEPATH".into(), format!("\\users\\{user}")),
        ("LOCALAPPDATA".into(), format!("C:\\users\\{user}\\AppData\\Local")),
        ("NUMBER_OF_PROCESSORS".into(), num_cpus().to_string()),
        ("OS".into(), "Windows_NT".into()),
        ("Path".into(), "C:\\Windows\\system32;C:\\Windows".into()),
        ("PATHEXT".into(), ".COM;.EXE;.BAT;.CMD".into()),
        ("PROCESSOR_ARCHITECTURE".into(), "AMD64".into()),
        ("ProgramData".into(), "C:\\ProgramData".into()),
        ("ProgramFiles".into(), "C:\\Program Files".into()),
        ("ProgramFiles(x86)".into(), "C:\\Program Files (x86)".into()),
        ("ProgramW6432".into(), "C:\\Program Files".into()),
        ("PUBLIC".into(), "C:\\users\\Public".into()),
        ("SystemDrive".into(), "C:".into()),
        ("SystemRoot".into(), "C:\\Windows".into()),
        ("TEMP".into(), format!("C:\\users\\{user}\\AppData\\Local\\Temp")),
        ("TMP".into(), format!("C:\\users\\{user}\\AppData\\Local\\Temp")),
        ("USERNAME".into(), user.clone()),
        ("USERPROFILE".into(), format!("C:\\users\\{user}")),
        ("windir".into(), "C:\\Windows".into()),
    ];
    vars.extend(trusted.port_env_overrides.iter().cloned());
    vars.extend(extra.iter().cloned());
    vars.sort_by_key(|(k, _)| k.to_ascii_uppercase());
    vars.dedup_by(|a, b| a.0.eq_ignore_ascii_case(&b.0));
    *INITIAL_ENV.lock().unwrap() = vars.clone();
    *ENV.lock().unwrap() = vars;
    ENV_GEN.fetch_add(1, std::sync::atomic::Ordering::Release);
    ENV_INIT_GEN.fetch_add(1, std::sync::atomic::Ordering::Release);
}

pub fn get_env(name: &str) -> Option<String> {
    ENV.lock().unwrap().iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.clone())
}

pub fn set_env(name: &str, value: Option<&str>) {
    let mut e = ENV.lock().unwrap();
    e.retain(|(k, _)| !k.eq_ignore_ascii_case(name));
    if let Some(v) = value {
        e.push((name.to_string(), v.to_string()));
        e.sort_by_key(|(k, _)| k.to_ascii_uppercase());
    }
    ENV_GEN.fetch_add(1, std::sync::atomic::Ordering::Release);
}

pub static ENV_GEN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// Generation counter incremented only on process environment initialization
/// (`init_env` / `replace_env`), never on runtime `set_env` mutations.
pub static ENV_INIT_GEN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub fn env_vars() -> Vec<(String, String)> {
    ENV.lock().unwrap().clone()
}

pub fn initial_env_vars() -> Vec<(String, String)> {
    INITIAL_ENV.lock().unwrap().clone()
}

pub fn env_init_gen() -> u32 {
    ENV_INIT_GEN.load(std::sync::atomic::Ordering::Acquire)
}

/// Build a fresh double-NUL-terminated UTF-16 environment block (caller frees with libc::free).
pub fn env_block_w() -> *mut u16 {
    let mut v: Vec<u16> = Vec::new();
    for (k, val) in env_vars() {
        v.extend(format!("{k}={val}").encode_utf16());
        v.push(0);
    }
    v.push(0);
    if v.len() == 1 {
        v.push(0);
    }
    unsafe {
        let p = libc::malloc(v.len() * 2) as *mut u16;
        std::ptr::copy_nonoverlapping(v.as_ptr(), p, v.len());
        p
    }
}

pub fn env_block_a() -> *mut u8 {
    let mut v: Vec<u8> = Vec::new();
    for (k, val) in env_vars() {
        v.extend(format!("{k}={val}").bytes());
        v.push(0);
    }
    v.push(0);
    if v.len() == 1 {
        v.push(0);
    }
    unsafe {
        let p = libc::malloc(v.len()) as *mut u8;
        std::ptr::copy_nonoverlapping(v.as_ptr(), p, v.len());
        p
    }
}

// ---------------------------------------------------------------------------
// Host info
// ---------------------------------------------------------------------------

pub fn num_cpus() -> u32 {
    static N: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *N.get_or_init(|| {
        let host = std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(4);
        std::env::var("PORT_NUM_CPUS")
            .ok()
            .and_then(|v| v.trim().parse::<u32>().ok())
            .filter(|&n| n >= 1)
            .map(|n| n.min(64))
            .unwrap_or(host)
    })
}

pub fn physical_memory() -> u64 {
    let mut v: u64 = 0;
    let mut sz = std::mem::size_of::<u64>();
    unsafe {
        libc::sysctlbyname(c"hw.memsize".as_ptr(), &mut v as *mut u64 as *mut _, &mut sz, std::ptr::null_mut(), 0);
    }
    v
}

/// Optional POSIX pipe write-end fd inherited from a parent `CreateProcessW` call
/// for reporting the full 32-bit Windows exit code back to the parent.
static EXIT_CODE_PIPE_FD: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);

/// Capture `PORT_EXIT_CODE_FD` from the process environment (if launched by `CreateProcessW`),
/// immediately mark the fd `FD_CLOEXEC`, and strip the env var so grandchildren never inherit it.
pub fn init_exit_code_channel() {
    #[cfg(not(target_os = "ios"))]
    {
        let raw = std::env::var("PORT_EXIT_CODE_FD").ok();
        std::env::remove_var("PORT_EXIT_CODE_FD");
        if let Some(s) = raw {
            if let Ok(fd) = s.trim().parse::<i32>() {
                if fd >= 0 {
                    unsafe {
                        let fl = libc::fcntl(fd, libc::F_GETFD);
                        if fl >= 0 {
                            let _ = libc::fcntl(fd, libc::F_SETFD, fl | libc::FD_CLOEXEC);
                            EXIT_CODE_PIPE_FD.store(fd, std::sync::atomic::Ordering::Release);
                        }
                    }
                }
            }
        }
    }
}

/// Write the full 32-bit Windows exit code to the parent's exit-code pipe (at most once).
fn report_exit_code_to_parent(code: u32) {
    let fd = EXIT_CODE_PIPE_FD.swap(-1, std::sync::atomic::Ordering::AcqRel);
    if fd < 0 {
        return;
    }
    let bytes = code.to_le_bytes();
    let mut off = 0usize;
    while off < bytes.len() {
        let r = unsafe {
            libc::write(
                fd,
                bytes.as_ptr().add(off) as *const std::ffi::c_void,
                bytes.len() - off,
            )
        };
        if r < 0 {
            if crate::types::last_errno() == libc::EINTR {
                continue;
            }
            break;
        }
        if r == 0 {
            break;
        }
        off += r as usize;
    }
    unsafe {
        let _ = libc::close(fd);
    }
}

/// Terminate the process with a Windows exit code.
pub fn exit_process(code: u32) -> ! {
    if crate::shadow::in_child() {
        // Shadow probe child (a fork copy of the game): no exit hooks or log flushes.
        unsafe { libc::_exit(code as i32) }
    }
    crate::shadow::note_exit("exit_process", code as i64);
    log::info!("process exit, code {code} (0x{code:x})");
    record_exit_code(code);
    run_exit_hooks();
    crate::ucrt::stdio::flush_all();
    report_exit_code_to_parent(code);
    unsafe { libc::_exit(posix_exit_code(code)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cmdline_split() {
        assert_eq!(
            split_cmdline(r#""C:\a b\x.exe" -foo "bar baz" a\\\"b c\\d"#),
            vec![r"C:\a b\x.exe", "-foo", "bar baz", "a\\\"b", r"c\\d"]
        );
    }

    static HOOK_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    fn hook() {
        HOOK_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    #[test]
    fn exit_hooks_run_once() {
        register_exit_hook(hook);
        register_exit_hook(hook); // deduplicated
        run_exit_hooks();
        run_exit_hooks();
        assert_eq!(HOOK_CALLS.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn manifest_game_args_merge_respects_explicit_cli_overrides() {
        set_manifest_game_args(vec![
            "-epicuserid=Default-Epic-User".into(),
            "-flag".into(),
            "spaced arg".into(),
        ]);
        assert_eq!(
            merge_manifest_args(r#""C:\Game\App.exe""#),
            r#""C:\Game\App.exe" -epicuserid=Default-Epic-User -flag "spaced arg""#
        );
        // Explicit -epicuserid=... on CLI overrides manifest default without duplicating.
        assert_eq!(
            merge_manifest_args(r#""C:\Game\App.exe" -EPICUSERID=custom123 -flag"#),
            r#""C:\Game\App.exe" -EPICUSERID=custom123 -flag "spaced arg""#
        );
        set_manifest_game_args(Vec::new());
    }

    #[test]
    fn exit_code_channel_sets_cloexec_strips_env_and_writes_u32_once() {
        let mut fds = [-1i32; 2];
        assert_eq!(unsafe { libc::pipe(fds.as_mut_ptr()) }, 0);
        let (rfd, wfd) = (fds[0], fds[1]);
        unsafe {
            // Simulate inherited write fd (FD_CLOEXEC cleared) + env var present.
            let fl = libc::fcntl(wfd, libc::F_GETFD);
            assert!(fl >= 0);
            assert_eq!(libc::fcntl(wfd, libc::F_SETFD, fl & !libc::FD_CLOEXEC), 0);
            std::env::set_var("PORT_EXIT_CODE_FD", wfd.to_string());
        }

        init_exit_code_channel();
        assert!(std::env::var("PORT_EXIT_CODE_FD").is_err(), "PORT_EXIT_CODE_FD must be removed from host env");
        let after_fl = unsafe { libc::fcntl(wfd, libc::F_GETFD) };
        assert_ne!(after_fl & libc::FD_CLOEXEC, 0, "exit-code fd must have FD_CLOEXEC set to avoid descendant leaks");

        // First write sends full 32-bit exit code and closes wfd; second write is a no-op.
        report_exit_code_to_parent(0xC000_013Au32);
        report_exit_code_to_parent(0x1111_2222u32);

        let mut buf = [0u8; 8];
        let mut got = 0usize;
        loop {
            let n = unsafe { libc::read(rfd, buf.as_mut_ptr().add(got) as *mut libc::c_void, buf.len() - got) };
            if n <= 0 {
                break;
            }
            got += n as usize;
        }
        unsafe {
            let _ = libc::close(rfd);
        }
        assert_eq!(got, 4);
        assert_eq!(u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]), 0xC000_013Au32);
    }

    #[test]
    fn trusted_child_host_env_filters_non_allowlisted_and_late_std_env_mutations() {
        let synthetic = vec![
            ("HOME".to_string(), "/tmp/fake-home".to_string()),
            ("PATH".to_string(), "/usr/bin:/bin".to_string()),
            ("TMPDIR".to_string(), "/tmp/fake-tmp".to_string()),
            ("USER".to_string(), "tester".to_string()),
            ("TRANSLATOR_HOME".to_string(), "/tmp/fake-translator".to_string()),
            ("PORT_AOT_DYLIB".to_string(), "marker-must-not-pass".to_string()),
            ("AW2_AOT_DYLIB".to_string(), "marker-must-not-pass".to_string()),
            ("DYLD_INSERT_LIBRARIES".to_string(), "marker-must-not-pass".to_string()),
            ("PORT_CHILD_ENV".to_string(), "marker-must-not-pass".to_string()),
            ("HOST_ONLY_MARKER".to_string(), "host-secret-marker".to_string()),
            ("PORT_HEADLESS".to_string(), "1".to_string()),
        ];
        let snap = capture_trusted_host_env_from(synthetic);
        let keys: Vec<&str> = snap.child_env.iter().map(|(k, _)| k.as_str()).collect();
        assert!(keys.contains(&"HOME"));
        assert!(keys.contains(&"PATH"));
        assert!(keys.contains(&"TMPDIR"));
        assert!(keys.contains(&"USER"));
        assert!(keys.contains(&"TRANSLATOR_HOME"));
        assert!(!keys.contains(&"PORT_AOT_DYLIB"));
        assert!(!keys.contains(&"AW2_AOT_DYLIB"));
        assert!(!keys.contains(&"DYLD_INSERT_LIBRARIES"));
        assert!(!keys.contains(&"PORT_CHILD_ENV"));
        assert!(!keys.contains(&"HOST_ONLY_MARKER"));
        assert!(snap.headless);
        assert_eq!(snap.user, "tester");

        // Late std::env mutations after startup snapshot must not appear in trusted_child_host_env().
        init_trusted_host_env();
        let late_key = "PORT_LATE_MARKER_TEST";
        let prev = std::env::var_os(late_key);
        std::env::set_var(late_key, "late-value");
        let child_env = trusted_child_host_env();
        match prev {
            Some(v) => std::env::set_var(late_key, v),
            None => std::env::remove_var(late_key),
        }
        assert!(child_env.iter().all(|(k, _)| k != late_key));
    }
}
