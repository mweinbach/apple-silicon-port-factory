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

/// Initialize the Windows environment. We do NOT pass the host environment
/// through wholesale (host paths would confuse Windows code); only a
/// Windows-looking base set plus `AW2ENV_<NAME>=value` overrides.
pub fn init_env(extra: &[(String, String)]) {
    let mut e = ENV.lock().unwrap();
    let user = std::env::var("USER").unwrap_or_else(|_| "player".into());
    let computer = crate::config::app_config().computer_name;
    let base: Vec<(String, String)> = vec![
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
    e.clear();
    e.extend(base);
    for (k, v) in std::env::vars() {
        if let Some(name) = k.strip_prefix("PORT_ENV_").or_else(|| k.strip_prefix("AW2ENV_")) {
            e.push((name.to_string(), v));
        }
    }
    e.extend(extra.iter().cloned());
    e.sort_by_key(|(k, _)| k.to_ascii_uppercase());
    e.dedup_by(|a, b| a.0.eq_ignore_ascii_case(&b.0));
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
    ENV_GEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

pub static ENV_GEN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub fn env_vars() -> Vec<(String, String)> {
    ENV.lock().unwrap().clone()
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
}
