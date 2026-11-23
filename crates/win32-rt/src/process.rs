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
static RUNNER_EXE: Mutex<Option<std::path::PathBuf>> = Mutex::new(None);

/// Register the host `port-runner` binary path used by `CreateProcessW` on macOS
/// to spawn real Windows x86-64 child processes.
pub fn set_runner_exe(path: std::path::PathBuf) {
    *RUNNER_EXE.lock().unwrap_or_else(|e| e.into_inner()) = Some(path);
}

/// Resolve the host `port-runner` binary path for child guest process launch.
pub fn runner_exe() -> Option<std::path::PathBuf> {
    if let Ok(p) = std::env::var("PORT_RUNNER_EXE") {
        let pb = std::path::PathBuf::from(p.trim());
        if !pb.as_os_str().is_empty() && pb.is_file() {
            return Some(pb);
        }
    }
    if let Some(pb) = RUNNER_EXE.lock().unwrap_or_else(|e| e.into_inner()).clone() {
        if pb.is_file() {
            return Some(pb);
        }
    }
    if let Ok(cur) = std::env::current_exe() {
        let stem = cur.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if stem == "port-runner" || stem == "aw2-loader" {
            return Some(cur);
        }
    }
    None
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
}

// ---------------------------------------------------------------------------
// Exit hooks
// ---------------------------------------------------------------------------

static EXIT_HOOKS: std::sync::Mutex<Vec<fn()>> = std::sync::Mutex::new(Vec::new());
static EXIT_HOOKS_RAN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Code passed to [`exit_process`] (`u64::MAX`: not exiting through it).
static EXIT_CODE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);

/// The code the process is exiting with through [`exit_process`] (ExitProcess, CRT exit
/// paths, ...), or None (fatal faults, crash reports, still running). For exit hooks.
pub fn exit_code() -> Option<u32> {
    let c = EXIT_CODE.load(std::sync::atomic::Ordering::Acquire);
    (c != u64::MAX).then_some(c as u32)
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
    let user = std::env::var("USER").unwrap_or_else(|_| "player".into());
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
    for (k, v) in std::env::vars() {
        if let Some(name) = k.strip_prefix("PORT_ENV_").or_else(|| k.strip_prefix("AW2ENV_")) {
            vars.push((name.to_string(), v));
        }
    }
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
    let _ = EXIT_CODE.compare_exchange(u64::MAX, code as u64, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire);
    run_exit_hooks();
    crate::ucrt::stdio::flush_all();
    report_exit_code_to_parent(code);
    unsafe { libc::_exit(code as i32) }
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
}
