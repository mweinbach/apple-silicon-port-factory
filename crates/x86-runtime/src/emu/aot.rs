//! AOT translation cache for mapped PE images (`PORT_AOT` / `AW2_DLL_AOT`). Design:
//! `target/review/aot/DESIGN.md`. Translator side: `jit_aot.rs`.
//!
//! - **Profile-free offline prewarm** ([`aot_prewarm_all_mapped`], `port-runner --prewarm-aot`):
//!   discovers function roots directly from each mapped PE image in memory (`.pdata`
//!   `RUNTIME_FUNCTION` entries, `AddressOfEntryPoint`, Export Directory function RVAs, TLS
//!   callbacks, and Load Config CFG function tables), expands each root with the static CFG
//!   closure (`PORT_AOT_CLOSURE` / `AW2_DLL_AOT_CLOSURE`, default 128) including `.pdata`-less
//!   leaf callees and switch/jump tables, translates them in AOT emission mode (with multi-bank
//!   stub banks every 112 MiB for large binaries), and writes `<cache>/<module>-<key>.aot`
//!   without executing `DllMain` or `WinMain`.
//! - **Profile** (`PORT_JIT_PROFILE=1` / `AW2_JIT_PROFILE=1`): every image block the JIT
//!   translates is counted; `<cache>/profile/<module>-<imagehash>.tsv` (`rva<TAB>count`) is
//!   merged every 60 s and at exit.
//! - **Build** (`PORT_AOT_BUILD` / `AW2_DLL_AOT_BUILD=1|all|<module substrings,...>`): at the
//!   loader's first activation point (all static images mapped, relocated and import-bound,
//!   before any DllMain) the profiled blocks of each selected module (+ a static closure of
//!   depth `PORT_AOT_CLOSURE` / `AW2_DLL_AOT_CLOSURE`, default 128) are translated in AOT
//!   emission mode and written to `<cache>/<module>-<key>.aot`; then the process exits.
//! - **Load** (`PORT_AOT=1` / `AW2_DLL_AOT=1`): at the same point (and after later
//!   `LoadLibrary`s), a cache whose key (image hash, base, size, codegen source hash, codegen
//!   knobs) matches is copied into a `MAP_JIT` region behind freshly emitted shared stubs;
//!   host-fn relocations are applied, host-callable assumptions checked, blocks linked to each
//!   other, and the blocks published in a global directory. Per-thread caches install AOT
//!   blocks on an index miss ([`find`]) instead of translating them. With `AW2_DLL_AOT_PROBE`
//!   (default 1) the live blocks are also in a lock-free shared table ([`probe_table`]) that
//!   the `lookup_*` stubs search after a miss in the thread's index, continuing in AOT code
//!   without leaving translated code.
//! - **Invalidation**: after every `va_gen` change, assumptions and relocations are
//!   re-checked once ([`revalidate`]); failing blocks are marked dead and their incoming
//!   AOT links are unpatched back to the fallbacks; dead blocks whose checks pass again are
//!   revived (`PORT_AOT_REVIVE` / `AW2_DLL_AOT_REVIVE`, default 1). An image overlapping
//!   run-time code is killed entirely ([`note_code_region`], bumps [`epoch`] so per-thread
//!   caches flush). Dead code is never freed.
//! - `PORT_AOT_VERIFY=1` / `AW2_DLL_AOT_VERIFY=1`: every block is re-translated at load and
//!   compared word for word (relocated slots excepted) with the file, plus its links,
//!   relocations and assumptions; mismatching blocks are dropped. Background-built caches are
//!   verified at their first load anyway (all blocks below 50K, else a 2% sample), then
//!   flagged done.
//! - **Default** (`PORT_AOT` / `AW2_DLL_AOT` unset): mode 2 for game runs (`--diff-test` /
//!   `--no-run` / `--prewarm-aot` / builds / unit tests: off), cache dir
//!   `~/Library/Caches/port-factory/aot` (`PORT_AOT_DIR` / `AW2_DLL_AOT_DIR`, with read
//!   fallback to legacy `~/Library/Caches/aw2-port/aot`). Crash-loop guard: a run that dies
//!   before it is established (120 s uptime, `PORT_AOT_GUARD_SECS` / `AW2_DLL_AOT_GUARD_SECS`,
//!   or a clean exit) leaves its `session-<pid>.lock`; after two in a row one run goes without
//!   AOT (`PORT_AOT_GUARD=0` / `AW2_DLL_AOT_GUARD=0`: off). Next-run rebuilds wait
//!   `PORT_AOT_BG_DELAY` / `AW2_DLL_AOT_BG_DELAY` (60 s) and run at background QoS; then a
//!   sweep removes cache files unused for 30 days and the least recently used beyond 256 MiB;
//!   a written cache replaces the module's caches for other keys. Profile counts are halved
//!   past 2^32.

use super::jit::{self, AotReloc, AotStubOffs, AOT_STUBS_BYTES, RELOC_HOST_FN, RELOC_QPC_STATE, RELOC_SLOT_VAL};
use super::BlockCache;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock, RwLock};

use super::exec_mem::{alloc_jit, icache_invalidate, jit_writable, jit_write_protect};

// ---------------------------------------------------------------------------------------------
// Knobs
// ---------------------------------------------------------------------------------------------

fn env_u32_any(names: &[&str], default: u32) -> u32 {
    for name in names {
        if let Some(v) = std::env::var(name).ok().and_then(|s| s.trim().parse().ok()) {
            return v;
        }
    }
    default
}

fn env_var_any(names: &[&str]) -> Option<String> {
    for name in names {
        if let Ok(v) = std::env::var(name) {
            return Some(v);
        }
    }
    None
}

#[cfg(test)]
std::thread_local! {
    /// Test-only: AOT lookups/revalidation on for this thread (in-memory test modules).
    static TEST_AOT: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// Test-only: revalidations run on this thread don't revive blocks.
    static TEST_NO_REVIVE: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// Test-only: override [`verify_mode`] on this thread without polluting global `OnceLock`.
    static TEST_VERIFY: core::cell::Cell<Option<u32>> = const { core::cell::Cell::new(None) };
    /// Test-only: import-slot relocations rebased by installs on this thread.
    static TEST_REBASED: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    /// Test-only: blocks installed on this thread with a held import slot.
    static TEST_HELD: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    /// Test-only: blocks verified by installs on this thread.
    static TEST_VERIFIED: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    /// Test-only: verify mismatches observed by installs on this thread.
    static TEST_VERIFY_MISMATCHES: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

/// `PORT_AOT_THREADS` / `AW2_DLL_AOT_THREADS`: worker threads for offline `.aot` prewarming
/// and builds (0 or unset = `std::thread::available_parallelism()`, clamped to `1..=64`).
fn prewarm_threads() -> usize {
    let default = std::thread::available_parallelism().map_or(1, |n| n.get());
    let raw = env_u32_any(&["PORT_AOT_THREADS", "AW2_DLL_AOT_THREADS"], 0) as usize;
    (if raw == 0 { default } else { raw }).clamp(1, 64)
}

/// `PORT_AOT_JUMPTABLES` / `AW2_DLL_AOT_JUMPTABLES` (default 1): enable RIP-relative switch /
/// jump-table discovery during offline static CFG closure.
fn jumptables_enabled() -> bool {
    env_u32_any(&["PORT_AOT_JUMPTABLES", "AW2_DLL_AOT_JUMPTABLES"], 1) != 0
}

/// `PORT_AOT_RELOC_ROOTS` (default 1): harvest executable-section code targets from PE Base
/// Relocation (`DataDirectory[5]`, `IMAGE_REL_BASED_DIR64`) entries (with a conservative
/// read-only/data pointer scan fallback when `.reloc` is stripped).
fn reloc_roots_enabled() -> bool {
    env_u32_any(&["PORT_AOT_RELOC_ROOTS"], 1) != 0
}

/// Default static CFG closure depth (BFS steps from each root; overridable via
/// `PORT_AOT_CLOSURE` / `AW2_DLL_AOT_CLOSURE`). Deep enough to traverse full intra-function
/// CFGs (loops, diamond chains, switch handlers, and `.pdata`-less leaf callees).
const DEFAULT_AOT_CLOSURE: u32 = 128;

fn aot_closure_depth() -> u32 {
    env_u32_any(&["PORT_AOT_CLOSURE", "AW2_DLL_AOT_CLOSURE"], DEFAULT_AOT_CLOSURE)
}

/// `PORT_AOT` / `AW2_DLL_AOT`: 0 = JIT only, 1 = load valid caches, 2 = load valid caches and
/// build missing (installed when done) or outgrown (for the next run) ones from the profile on a
/// background thread (implies `PORT_JIT_PROFILE=1`). Unset: 2 for game runs, 0 for
/// `--diff-test` / `--no-run` / `--prewarm-aot` and unit tests. Forced to 0 for one run by the
/// crash-loop guard ([`session_check`]).
pub(super) fn aot_mode() -> u32 {
    #[cfg(test)]
    if TEST_AOT.with(|c| c.get()) {
        return 1;
    }
    static V: OnceLock<u32> = OnceLock::new();
    *V.get_or_init(|| {
        if crate::diag::jit_filter_active() || crate::diag::nan_trace_has_static_ranges() {
            return 0;
        }
        let set = std::env::var_os("PORT_AOT").is_some() || std::env::var_os("AW2_DLL_AOT").is_some();
        let mut v = env_u32_any(
            &["PORT_AOT", "AW2_DLL_AOT"],
            if !cfg!(test) && game_run() && build_request().is_none() { 2 } else { 0 },
        );
        if v != 0 && !session_begin() {
            v = 0;
        }
        if v != 0 {
            let fallbacks = fallback_cache_dirs();
            if fallbacks.is_empty() {
                log::info!(
                    "emu aot: AOT translation caches enabled (PORT_AOT={v}{}, dir {})",
                    if set { "" } else { ", default" },
                    cache_dir().display()
                );
            } else {
                let fb_list: Vec<String> = fallbacks.iter().map(|p| p.display().to_string()).collect();
                log::info!(
                    "emu aot: AOT translation caches enabled (PORT_AOT={v}{}, dir {}, fallback [{}])",
                    if set { "" } else { ", default" },
                    cache_dir().display(),
                    fb_list.join(", ")
                );
            }
        }
        v
    })
}

/// Not a `--diff-test` / `--no-run` / `--prewarm-aot` / `--emit-static-o` loader invocation (those keep AOT off unless asked).
fn game_run() -> bool {
    !std::env::args().any(|a| {
        a == "--diff-test" || a == "--no-run" || a == "--prewarm-aot" || a == "--emit-static-o"
    })
}

/// `PORT_AOT_VERIFY=1` / `AW2_DLL_AOT_VERIFY=1`: re-translate and compare every block at load.
fn verify_mode() -> u32 {
    #[cfg(test)]
    if let Some(v) = TEST_VERIFY.with(|c| c.get()) {
        return v;
    }
    static V: OnceLock<u32> = OnceLock::new();
    *V.get_or_init(|| env_u32_any(&["PORT_AOT_VERIFY", "AW2_DLL_AOT_VERIFY"], 0))
}

/// `PORT_JIT_PROFILE=1` / `AW2_JIT_PROFILE=1` (or AOT mode 2): record and dump the translated image-block profile.
fn profile_enabled() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| {
        let on = env_u32_any(&["PORT_JIT_PROFILE", "AW2_JIT_PROFILE"], 0) != 0 || aot_mode() == 2;
        if on {
            log::info!(
                "emu aot: recording the JIT block profile (PORT_JIT_PROFILE=1 / PORT_AOT=2) -> {}",
                cache_dir().join("profile").display()
            );
        }
        on
    })
}

/// `PORT_AOT_BG_DELAY` / `AW2_DLL_AOT_BG_DELAY` (seconds, default 60): next-run rebuilds and the cache sweep
/// wait this long after the first activation.
fn bg_delay() -> std::time::Duration {
    std::time::Duration::from_secs(env_u32_any(&["PORT_AOT_BG_DELAY", "AW2_DLL_AOT_BG_DELAY"], 60) as u64)
}

/// `PORT_AOT_BUILD` / `AW2_DLL_AOT_BUILD`: None = off; empty = every mapped module with a profile;
/// otherwise lower-case module-name substrings.
fn build_request() -> Option<Vec<String>> {
    let v = env_var_any(&["PORT_AOT_BUILD", "AW2_DLL_AOT_BUILD"])?;
    let v = v.trim().to_ascii_lowercase();
    match v.as_str() {
        "" | "0" => None,
        "1" | "all" => Some(Vec::new()),
        _ => Some(v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()),
    }
}

/// Set while [`aot_prewarm_all_mapped`] is translating blocks so `jit::aot_probe_enabled`
/// matches normal runtime runs even outside `--prewarm-aot` CLI invocations.
static PREWARMING: AtomicBool = AtomicBool::new(false);

/// `PORT_AOT_BUILD` / `AW2_DLL_AOT_BUILD` or `--prewarm-aot` / `--emit-static-o` is active (this process builds caches).
#[cfg_attr(test, allow(dead_code))]
pub(super) fn building() -> bool {
    static V: OnceLock<bool> = OnceLock::new();
    PREWARMING.load(Ordering::Acquire)
        || *V.get_or_init(|| {
            build_request().is_some()
                || std::env::args().any(|a| a == "--prewarm-aot" || a == "--emit-static-o")
        })
}

fn cache_dir() -> PathBuf {
    if let Some(d) = std::env::var_os("PORT_AOT_DIR")
        .or_else(|| std::env::var_os("AW2_DLL_AOT_DIR"))
        .filter(|s| !s.is_empty())
    {
        return d.into();
    }
    let home = std::env::var_os("HOME").unwrap_or_default();
    Path::new(&home).join("Library/Caches/port-factory/aot")
}

/// Legacy AOT cache directory (`~/Library/Caches/aw2-port/aot`) consulted as a graceful read
/// fallback when neither `PORT_AOT_DIR` nor `AW2_DLL_AOT_DIR` is explicitly set.
fn legacy_cache_dir() -> Option<PathBuf> {
    if std::env::var_os("PORT_AOT_DIR").is_some_and(|s| !s.is_empty())
        || std::env::var_os("AW2_DLL_AOT_DIR").is_some_and(|s| !s.is_empty())
    {
        return None;
    }
    let home = std::env::var_os("HOME")?;
    let legacy = Path::new(&home).join("Library/Caches/aw2-port/aot");
    (legacy != cache_dir()).then_some(legacy)
}

/// Ordered read-only fallback AOT cache directories (never written to or touched):
/// 1. `PORT_AOT_FALLBACK_DIR` (e.g. bundled `.app/Contents/Resources/AotCache`)
/// 2. Auto-discovered `<current_exe>/../Resources/AotCache` (when present)
/// 3. Legacy `~/Library/Caches/aw2-port/aot` (when no explicit writable dir override is set)
fn fallback_cache_dirs_for(primary: &Path) -> Vec<PathBuf> {
    let primary_canon = std::fs::canonicalize(primary).ok();
    let mut dirs = Vec::new();
    let mut push_unique = |cand: PathBuf| {
        if cand.as_os_str().is_empty() || cand == primary {
            return;
        }
        let cand_canon = std::fs::canonicalize(&cand).ok();
        if primary_canon.is_some() && cand_canon == primary_canon {
            return;
        }
        if dirs.iter().any(|existing: &PathBuf| {
            existing == &cand
                || (cand_canon.is_some() && std::fs::canonicalize(existing).ok() == cand_canon)
        }) {
            return;
        }
        dirs.push(cand);
    };

    if let Some(fb) = std::env::var_os("PORT_AOT_FALLBACK_DIR").filter(|s| !s.is_empty()) {
        push_unique(PathBuf::from(fb));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            // Flat iOS `.app` bundle layout (`<App>.app/AotCache`).
            let flat_bundled = parent.join("AotCache");
            if flat_bundled.is_dir() {
                push_unique(flat_bundled);
            }
            // macOS `.app` bundle layout (`<App>.app/Contents/Resources/AotCache`).
            let bundled = parent.join("../Resources/AotCache");
            if bundled.is_dir() {
                push_unique(bundled);
            }
        }
    }
    if let Some(legacy) = legacy_cache_dir() {
        push_unique(legacy);
    }
    dirs
}

fn fallback_cache_dirs() -> Vec<PathBuf> {
    fallback_cache_dirs_for(&cache_dir())
}

// ---------------------------------------------------------------------------------------------
// Cache maintenance: crash-loop guard, eviction
// ---------------------------------------------------------------------------------------------

/// Consecutive runs that did not exit cleanly after which one run goes without AOT.
const GUARD_STRIKES: u32 = 2;

/// Crash-loop guard verdict for a starting run ([`session_check`]).
#[derive(Debug, PartialEq, Eq)]
enum Session {
    /// AOT on (this run's lock created); `strikes` consecutive unclean runs before it,
    /// `stale` locks of dead processes removed.
    Go { strikes: u32, stale: usize },
    /// AOT off for this run (no lock; the strike counter starts over).
    Skip { stale: usize },
}

fn session_lock(dir: &Path, pid: u32) -> PathBuf {
    dir.join(format!("session-{pid}.lock"))
}

/// Remove the `session-<pid>.lock` files in `dir` of processes that are gone (`alive`
/// false, or `pid` itself: a reused pid), i.e. runs that did not exit cleanly; count
/// consecutive starts that found one in `session.strikes`; decide. On [`Session::Go`],
/// `pid`'s lock is created ([`session_end`] removes it on a clean exit).
fn session_check(dir: &Path, pid: u32, alive: impl Fn(u32) -> bool) -> Session {
    let _ = std::fs::create_dir_all(dir);
    let mut stale = 0;
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let f = e.file_name();
            let Some(p) = f.to_str().and_then(|s| s.strip_prefix("session-")).and_then(|s| s.strip_suffix(".lock")).and_then(|s| s.parse::<u32>().ok()) else {
                continue;
            };
            if (p == pid || !alive(p)) && std::fs::remove_file(e.path()).is_ok() {
                stale += 1;
            }
        }
    }
    let sf = dir.join("session.strikes");
    let prev = std::fs::read_to_string(&sf).ok().and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(0);
    let strikes = if stale > 0 { prev + 1 } else { 0 };
    if strikes >= GUARD_STRIKES {
        let _ = std::fs::write(&sf, "0\n");
        return Session::Skip { stale };
    }
    let _ = std::fs::write(&sf, format!("{strikes}\n"));
    let _ = std::fs::write(session_lock(dir, pid), format!("{pid}\n"));
    Session::Go { strikes, stale }
}

/// Crash-loop guard of a game run (`PORT_AOT_GUARD` / `AW2_DLL_AOT_GUARD`, default 1):
/// false = no AOT caches this run. Only runs that die before they are established count:
/// this run's lock is released after `PORT_AOT_GUARD_SECS` / `AW2_DLL_AOT_GUARD_SECS`
/// (default 120) of uptime or at a clean exit, whichever comes first (harness SIGTERM /
/// force quit later is no strike).
fn session_begin() -> bool {
    if cfg!(test)
        || !game_run()
        || build_request().is_some()
        || env_u32_any(&["PORT_AOT_GUARD", "AW2_DLL_AOT_GUARD"], 1) == 0
    {
        return true;
    }
    match session_check(&cache_dir(), std::process::id(), pid_alive) {
        Session::Go { strikes, stale } => {
            win32_rt::process::register_exit_hook(session_end);
            let secs = env_u32_any(&["PORT_AOT_GUARD_SECS", "AW2_DLL_AOT_GUARD_SECS"], 120) as u64;
            let _ = std::thread::Builder::new().name("port-aot-guard".into()).spawn(move || {
                set_qos(QOS_CLASS_BACKGROUND);
                std::thread::sleep(std::time::Duration::from_secs(secs));
                session_release(&format!("{secs} s uptime"));
            });
            if stale > 0 {
                log::warn!("emu aot: the previous run died before it was established ({strikes} in a row; after {GUARD_STRIKES} one run goes without AOT caches)");
            }
            true
        }
        Session::Skip { stale } => {
            log::warn!(
                "emu aot: {GUARD_STRIKES} runs in a row died before they were established ({stale} stale session lock(s)): AOT caches off for this run (crash-loop guard; PORT_AOT_GUARD=0 disables it)"
            );
            false
        }
    }
}

/// Remove this run's session lock (once): the run is established, a later death is no
/// strike.
fn session_release(why: &str) {
    static DONE: AtomicBool = AtomicBool::new(false);
    if DONE.swap(true, Ordering::AcqRel) {
        return;
    }
    let _ = std::fs::remove_file(session_lock(&cache_dir(), std::process::id()));
    log::info!("emu aot: session established ({why}); crash-loop guard lock released");
}

fn pid_alive(pid: u32) -> bool {
    extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    const EPERM: i32 = 1;
    if pid == 0 || pid > i32::MAX as u32 {
        return false;
    }
    unsafe { kill(pid as i32, 0) == 0 || std::io::Error::last_os_error().raw_os_error() == Some(EPERM) }
}

/// Exit hook: a clean exit removes this run's session lock (crash paths run the exit hooks
/// too, without an exit code or with a failure code).
fn session_end() {
    if let Some(c) = win32_rt::process::exit_code().filter(|&c| clean_exit_code(c)) {
        session_release(&format!("clean exit, code {c}"));
    }
}

/// `ExitProcess` codes that count as a clean exit: not an NTSTATUS error / warning, not
/// `abort()`'s 3 or `terminate()`'s 255.
fn clean_exit_code(c: u32) -> bool {
    c < 0x8000_0000 && c != 3 && c != 255
}

/// The sweep removes cache files not modified for this long ...
const EVICT_MAX_AGE: std::time::Duration = std::time::Duration::from_secs(30 * 24 * 3600);
/// ... and then the least recently modified ones until the rest fits this (4 GiB to accommodate
/// multi-bank `.aot` caches for 100+ MiB game executables without immediate eviction).
const EVICT_CAP: u64 = 4 << 30;

/// Eviction candidates in cache dir `dir`: `*.aot`, `profile/*.tsv` and leftover
/// `*.tmp<pid>` files, with size and mtime (loads bump a cache's mtime: [`touch`]).
fn cache_files(dir: &Path) -> Vec<(PathBuf, u64, std::time::SystemTime)> {
    let mut v = Vec::new();
    for d in [dir.to_path_buf(), dir.join("profile")] {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            let ext = p.extension().and_then(|x| x.to_str()).unwrap_or("");
            if !(ext == "aot" || ext == "tsv" || ext.starts_with("tmp")) {
                continue;
            }
            let Ok(md) = e.metadata() else { continue };
            if md.is_file() {
                v.push((p, md.len(), md.modified().unwrap_or(std::time::UNIX_EPOCH)));
            }
        }
    }
    v
}

/// Remove the cache files of `dir` not modified for `max_age`, then the least recently
/// modified ones until the rest fits in `cap` bytes. Returns `[removed files, removed
/// bytes, kept files, kept bytes]`.
fn evict(dir: &Path, now: std::time::SystemTime, max_age: std::time::Duration, cap: u64) -> [u64; 4] {
    let mut files = cache_files(dir);
    files.sort_by_key(|f| f.2);
    let mut kept: u64 = files.iter().map(|f| f.1).sum();
    let mut out = [0u64; 4];
    for (p, len, mt) in &files {
        let old = now.duration_since(*mt).is_ok_and(|age| age > max_age);
        if (old || kept > cap) && std::fs::remove_file(p).is_ok() {
            out[0] += 1;
            out[1] += len;
            kept -= len;
        }
    }
    out[2] = files.len() as u64 - out[0];
    out[3] = kept;
    out
}

/// After `keep` (`<dir>/<name>-<key>.aot`) was written: remove `name`'s caches for other
/// keys (other loader builds / knobs / image versions). Returns the number removed.
fn delete_other_keys(dir: &Path, name: &str, keep: &Path) -> usize {
    let pre = format!("{name}-");
    let Ok(rd) = std::fs::read_dir(dir) else { return 0 };
    let mut n = 0;
    for e in rd.flatten() {
        let f = e.file_name();
        let Some(id) = f.to_str().and_then(|s| s.strip_prefix(pre.as_str())).and_then(|s| s.strip_suffix(".aot")) else {
            continue;
        };
        if id.len() == 16 && id.bytes().all(|b| b.is_ascii_hexdigit()) && e.path() != keep && std::fs::remove_file(e.path()).is_ok() {
            n += 1;
        }
    }
    n
}

/// Bump `path`'s mtime (a loaded cache is in use: the sweep evicts least recently used).
fn touch(path: &Path) {
    if let Ok(f) = std::fs::File::options().write(true).open(path) {
        let _ = f.set_modified(std::time::SystemTime::now());
    }
}

/// First activation with AOT on (start of the [`bg_delay`] for next-run rebuilds / sweep).
static ACTIVATED: OnceLock<std::time::Instant> = OnceLock::new();

/// Time left until [`bg_delay`] after the first activation.
fn bg_wait() -> std::time::Duration {
    bg_delay().saturating_sub(ACTIVATED.get().map_or(std::time::Duration::ZERO, |t| t.elapsed()))
}

/// Once per process: after [`bg_delay`], at background QoS, [`evict`] the cache dir.
fn sweep_start() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = std::thread::Builder::new().name("port-aot-sweep".into()).spawn(|| {
            set_qos(QOS_CLASS_BACKGROUND);
            std::thread::sleep(bg_wait());
            let r = evict(&cache_dir(), std::time::SystemTime::now(), EVICT_MAX_AGE, EVICT_CAP);
            log::info!(
                "emu aot: cache sweep: removed {} files ({} KiB; unused for {} days / over the {} MiB cap), kept {} files ({} KiB)",
                r[0],
                r[1] >> 10,
                EVICT_MAX_AGE.as_secs() / 86400,
                EVICT_CAP >> 20,
                r[2],
                r[3] >> 10
            );
        });
    });
}

extern "C" {
    fn pthread_set_qos_class_self_np(qos: u32, rel: i32) -> i32;
}

const QOS_CLASS_UTILITY: u32 = 0x11;
const QOS_CLASS_BACKGROUND: u32 = 0x09;

fn set_qos(q: u32) {
    unsafe { pthread_set_qos_class_self_np(q, 0) };
}

// ---------------------------------------------------------------------------------------------
// Hashing / keys
// ---------------------------------------------------------------------------------------------

const MAGIC: [u8; 8] = *b"AW2AOT\0\x01";
/// Legacy single-bank format version (still cleanly decoded and upgraded to 1-bank `Image`).
const FORMAT_VERSION_V5: u32 = 5;
/// Multi-bank format version (stores `stub_banks` table for >112 MiB executables).
const FORMAT_VERSION: u32 = 6;
/// Bump on any codegen-relevant change the source hash can't see.
const AOT_CODEGEN_VERSION: u64 = 3;

/// Small 128-bit streaming hash (cache keys / file checksums, not cryptographic).
struct H128 {
    a: u64,
    b: u64,
    n: u64,
}

impl H128 {
    fn new(seed: u64) -> Self {
        H128 { a: 0x9E37_79B9_7F4A_7C15 ^ seed, b: 0xC2B2_AE3D_27D4_EB4F ^ seed.rotate_left(32), n: 0 }
    }
    fn word(&mut self, w: u64) {
        self.a = (self.a ^ w).wrapping_mul(0x0000_0100_0000_01B3).rotate_left(29);
        self.b = (self.b.wrapping_add(w) ^ (self.b >> 29)).wrapping_mul(0xFF51_AFD7_ED55_8CCD);
        self.n += 1;
    }
    fn bytes(&mut self, d: &[u8]) {
        let mut ch = d.chunks_exact(8);
        for c in ch.by_ref() {
            self.word(u64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]));
        }
        let r = ch.remainder();
        if !r.is_empty() {
            let mut t = [0u8; 8];
            t[..r.len()].copy_from_slice(r);
            self.word(u64::from_le_bytes(t) ^ ((r.len() as u64) << 56));
        }
        self.word(d.len() as u64);
    }
    fn finish(&self) -> [u64; 2] {
        let f = |x: u64| {
            let x = (x ^ (x >> 33)).wrapping_mul(0xFF51_AFD7_ED55_8CCD);
            let x = (x ^ (x >> 33)).wrapping_mul(0xC4CE_B9FE_1A85_EC53);
            x ^ (x >> 33)
        };
        [f(self.a ^ self.n), f(self.b ^ self.a.rotate_left(17))]
    }
}

/// Hash of the translator / decoder sources (embedded), the format and the `Cpu` layout:
/// any change to them invalidates every cache, unrelated rebuilds keep them.
fn codegen_hash() -> u64 {
    static V: OnceLock<u64> = OnceLock::new();
    *V.get_or_init(|| {
        const SRC: [&[u8]; 15] = [
            include_bytes!("jit.rs"),
            include_bytes!("jit_vec.rs"),
            include_bytes!("jit_ext.rs"),
            include_bytes!("jit_more.rs"),
            include_bytes!("jit_xmm.rs"),
            include_bytes!("jit_lazy.rs"),
            include_bytes!("jit_aot.rs"),
            include_bytes!("fast.rs"),
            include_bytes!("fast_ext.rs"),
            include_bytes!("fast_more.rs"),
            include_bytes!("mod.rs"),
            include_bytes!("mem.rs"),
            include_bytes!("simd_mov.rs"),
            include_bytes!("simd_fp.rs"),
            include_bytes!("regfile.rs"),
        ];
        let mut h = H128::new(AOT_CODEGEN_VERSION);
        for s in SRC {
            h.bytes(s);
        }
        h.word(core::mem::size_of::<super::Cpu>() as u64);
        h.word(FORMAT_VERSION as u64);
        h.finish()[0]
    })
}

/// Everything a module's translations depend on besides the block set and the
/// host-callable state (assumptions / relocations). Any mismatch: the cache is ignored.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(super) struct AotKey {
    /// Hash of the mapped, relocated executable sections (+ image size).
    pub(super) image_hash: [u64; 2],
    pub(super) image_base: u64,
    pub(super) image_size: u64,
    pub(super) codegen_hash: u64,
    pub(super) knobs: u64,
}

impl AotKey {
    fn words(&self) -> [u64; 6] {
        [self.image_hash[0], self.image_hash[1], self.image_base, self.image_size, self.codegen_hash, self.knobs]
    }
    fn id(&self) -> u64 {
        let mut h = H128::new(0xA07);
        for w in self.words() {
            h.word(w);
        }
        h.finish()[0]
    }
}

/// Executable sections `[start, end)` of the PE image mapped at `[base, base + size)`
/// (empty if the headers are not a valid PE).
unsafe fn exec_sections(base: usize, size: usize) -> Vec<(usize, usize)> {
    const CNT_CODE: u32 = 0x20;
    const MEM_EXECUTE: u32 = 0x2000_0000;
    let mut out = Vec::new();
    if base == 0 || size < 0x400 {
        return out;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return out;
    }
    let nt = rd32(0x3C) as usize;
    if nt + 24 > size || rd32(nt) != 0x0000_4550 {
        return out;
    }
    let nsec = rd16(nt + 6) as usize;
    let table = nt + 24 + rd16(nt + 20) as usize;
    if table + nsec * 40 > size {
        return out;
    }
    for i in 0..nsec {
        let e = table + i * 40;
        let vsize = rd32(e + 8) as usize;
        let va = rd32(e + 12) as usize;
        let ch = rd32(e + 36);
        if ch & (MEM_EXECUTE | CNT_CODE) == 0 || vsize == 0 {
            continue;
        }
        let end = va.saturating_add(vsize).min(size);
        if va < end {
            out.push((base + va, base + end));
        }
    }
    out
}

/// Non-executable initialized data sections `[start, end)` of the PE image mapped at
/// `[base, base + size)`, excluding PE headers `[0..0x400)` and discardable sections.
unsafe fn data_sections(base: usize, size: usize) -> Vec<(usize, usize)> {
    const CNT_CODE: u32 = 0x20;
    const CNT_INIT_DATA: u32 = 0x40;
    const MEM_DISCARDABLE: u32 = 0x0200_0000;
    const MEM_EXECUTE: u32 = 0x2000_0000;
    let mut out = Vec::new();
    if base == 0 || size < 0x400 {
        return out;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return out;
    }
    let nt = rd32(0x3C) as usize;
    let Some(nt_end) = nt.checked_add(24) else {
        return out;
    };
    if nt_end > size || rd32(nt) != 0x0000_4550 {
        return out;
    }
    let nsec = rd16(nt + 6) as usize;
    let Some(table) = nt_end.checked_add(rd16(nt + 20) as usize) else {
        return out;
    };
    let Some(table_end) = nsec.checked_mul(40).and_then(|n| table.checked_add(n)) else {
        return out;
    };
    if table_end > size {
        return out;
    }
    for i in 0..nsec {
        let e = table + i * 40;
        let vsize = rd32(e + 8) as usize;
        let va = rd32(e + 12) as usize;
        let ch = rd32(e + 36);
        if (ch & (MEM_EXECUTE | CNT_CODE | MEM_DISCARDABLE)) != 0
            || (ch & CNT_INIT_DATA) == 0
            || vsize == 0
        {
            continue;
        }
        let start = va.max(0x400);
        let end = va.saturating_add(vsize).min(size);
        if start < end {
            out.push((base + start, base + end));
        }
    }
    out
}

unsafe fn image_hash(base: usize, size: usize) -> [u64; 2] {
    let mut h = H128::new(size as u64);
    for (s, e) in exec_sections(base, size) {
        h.word((s - base) as u64);
        h.bytes(core::slice::from_raw_parts(s as *const u8, e - s));
    }
    h.finish()
}

unsafe fn module_key(base: usize, size: usize) -> AotKey {
    module_key_hashed(base, size, None)
}

/// [`module_key`] with the image hash already computed (`h`, e.g. at activation).
unsafe fn module_key_hashed(base: usize, size: usize, h: Option<[u64; 2]>) -> AotKey {
    AotKey {
        image_hash: h.unwrap_or_else(|| image_hash(base, size)),
        image_base: base as u64,
        image_size: size as u64,
        codegen_hash: codegen_hash(),
        knobs: jit::aot_knob_bits(),
    }
}

fn cache_file(name: &str, key: &AotKey) -> PathBuf {
    cache_dir().join(format!("{name}-{:016x}.aot", key.id()))
}

fn profile_file(name: &str, image_hash: &[u64; 2]) -> PathBuf {
    cache_dir().join("profile").join(format!("{name}-{:016x}.tsv", image_hash[0]))
}

// ---------------------------------------------------------------------------------------------
// Image (one module's translations; file format)
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(super) struct BlockEnt {
    pub(super) ip: u64,
    /// Byte offset of the block's hot main-path code (`main` words) from region start.
    pub(super) code_off: u32,
    /// Byte offset of the block's cold out-of-line code (`words - main` words) from region start.
    pub(super) cold_off: u32,
    pub(super) words: u32,
    pub(super) main: u32,
    pub(super) guest: u32,
    pub(super) host_entry: bool,
    pub(super) link_lo: u32,
    pub(super) link_n: u32,
    pub(super) reloc_lo: u32,
    pub(super) reloc_n: u32,
    pub(super) assume_lo: u32,
    pub(super) assume_n: u32,
}

/// A direct-link site (`b`/`bl`), built pointing at its out-of-line fallback.
/// `cond_off != u32::MAX` names a hot conditional branch (`b.cond`/`cbz`/`cbnz`) pointing
/// at `site_off` that `link_pass` directly patches to `target` when within ±1 MiB.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct LinkEnt {
    pub(super) site_off: u32,
    pub(super) fallback_off: u32,
    pub(super) cond_off: u32,
    pub(super) target: u64,
}

/// A relocated 8-byte literal slot ([`AotReloc`]): `RELOC_HOST_FN` holds
/// `inline_host_fn(va).0` (whose `.1` must equal `bail`), `RELOC_SLOT_VAL` holds `va`.
/// `src` != 0: `va` is the value of that guest import slot (rebased at load).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct RelocEnt {
    pub(super) slot_off: u32,
    pub(super) kind: u8,
    pub(super) bail: bool,
    pub(super) va: u64,
    pub(super) src: u64,
}

#[inline(always)]
fn word_to_off(w: u32, main: u32, hot_off: u32, cold_off: u32) -> u32 {
    if w < main {
        hot_off + w * 4
    } else {
        cold_off + (w - main) * 4
    }
}

impl RelocEnt {
    fn of_split(hot_off: u32, cold_off: u32, main: u32, r: &AotReloc) -> Self {
        RelocEnt {
            slot_off: word_to_off(r.word, main, hot_off, cold_off),
            kind: r.kind,
            bail: r.bail,
            va: r.va,
            src: r.src,
        }
    }
}

pub(super) struct Image {
    pub(super) stubs: AotStubOffs,
    /// Region byte offsets of every `AOT_STUBS_BYTES` stub bank (16-byte aligned, strictly
    /// ascending, always starting at `0`). Bank `0` lives at `[0..AOT_STUBS_BYTES)`; each
    /// additional bank `k >= 1` occupies `[stub_banks[k]..stub_banks[k] + AOT_STUBS_BYTES)`
    /// inside `code` (reserved as NOP words on disk, stamped with stubs at load / Mach-O emit).
    pub(super) stub_banks: Vec<u32>,
    pub(super) blocks: Vec<BlockEnt>,
    pub(super) links: Vec<LinkEnt>,
    pub(super) relocs: Vec<RelocEnt>,
    /// `(va, host-callable)`.
    pub(super) assume: Vec<(u64, bool)>,
    /// Code words; word 0 sits at region offset [`AOT_STUBS_BYTES`].
    pub(super) code: Vec<u32>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            stubs: AotStubOffs::default(),
            stub_banks: vec![0],
            blocks: Vec::new(),
            links: Vec::new(),
            relocs: Vec::new(),
            assume: Vec::new(),
            code: Vec::new(),
        }
    }
}

/// Virtual region base AOT builds are emitted for (never mapped: only differences matter,
/// all branches out of a block are PC-relative).
const BUILD_VBASE: usize = 0x2000_0000_0000;
/// Maximum byte span of one stub bank (112 MiB, safely inside ARM64's ±128 MiB `B`/`BL`
/// reach with headroom for `AOT_STUBS_BYTES`, chunks, and a per-bank LSE2 patch slab).
const AOT_BANK_SPAN: usize = 112 << 20;
/// Total code ceiling per module across all stub banks (2 GiB: covers 200+ MiB Windows
/// executables while staying well inside Mach-O 32-bit file-offset limits).
const MAX_CODE: usize = 2048 << 20;
/// Per-bank LSE2 exact-TSO unaligned-trap patch slab in dynamic `MAP_JIT` regions.
const AOT_LSE2_SLAB: usize = 256 << 10;

impl Image {
    fn code_bytes(&self) -> usize {
        self.code.len() * 4
    }

    pub(super) fn stub_banks(&self) -> &[u32] {
        const SINGLE: [u32; 1] = [0];
        if self.stub_banks.is_empty() {
            &SINGLE
        } else {
            &self.stub_banks
        }
    }

    /// Bank index (`0..stub_banks().len()`) owning region byte offset `off`.
    #[inline]
    pub(super) fn bank_index_of(&self, off: u32) -> usize {
        let banks = self.stub_banks();
        if banks.len() <= 1 {
            0
        } else {
            banks.partition_point(|&b| b <= off).saturating_sub(1)
        }
    }

    /// Region byte offset of the stub bank owning region byte offset `off`.
    #[inline]
    pub(super) fn bank_of_off(&self, off: u32) -> u32 {
        self.stub_banks()[self.bank_index_of(off)]
    }

    /// Exclusive flat region byte offset ending bank `bank_idx`.
    #[inline]
    pub(super) fn bank_end(&self, bank_idx: usize) -> usize {
        let banks = self.stub_banks();
        if bank_idx + 1 < banks.len() {
            banks[bank_idx + 1] as usize
        } else {
            AOT_STUBS_BYTES + self.code_bytes()
        }
    }

    /// Open a new 16-byte-aligned stub bank at the current end of `self.code`, reserving
    /// `AOT_STUBS_BYTES` placeholder NOP words for the bank's shared stubs.
    fn start_new_bank(&mut self) -> usize {
        while self.code.len() % 4 != 0 {
            self.code.push(0xD503_201F);
        }
        let bank_off = AOT_STUBS_BYTES + self.code_bytes();
        debug_assert!(bank_off <= u32::MAX as usize && bank_off % 16 == 0);
        if self.stub_banks.is_empty() {
            self.stub_banks.push(0);
        }
        self.stub_banks.push(bank_off as u32);
        self.code.resize(self.code.len() + AOT_STUBS_BYTES / 4, 0xD503_201F);
        bank_off
    }

    /// Flush a chunk of blocks into `self` inside stub bank `bank_off`, with hot main paths
    /// packed contiguously first and cold out-of-line tails packed immediately after (keeping
    /// all hot<->cold branches well inside ±32 KiB `tbz`/`tbnz` reach and letting consecutive
    /// hot blocks fall through).
    fn push_chunk(&mut self, chunk: Vec<(u64, usize, jit::AotBlock)>, offs: &AotStubOffs, bank_off: usize) {
        if chunk.is_empty() {
            return;
        }
        // 1. Lay out all hot main paths back-to-back (4-byte aligned, zero wasted NOPs).
        let base_word = AOT_STUBS_BYTES / 4 + self.code.len();
        let mut hot_words = Vec::with_capacity(chunk.len());
        let mut cur_w = base_word;
        for &(_, _, ref b) in &chunk {
            hot_words.push(cur_w);
            cur_w += b.main as usize;
        }
        // 2. Lay out cold tails immediately after the chunk's hot section, matching each
        //    block's `main % 4` mod-16 alignment so 8B reloc slots and 16B vector literals
        //    preserve their exact alignment relative to the 16B-aligned region start.
        let mut cold_words = Vec::with_capacity(chunk.len());
        let mut cold_pads = Vec::with_capacity(chunk.len());
        for &(_, _, ref b) in &chunk {
            let cold_len = b.words.len() - b.main as usize;
            if cold_len == 0 {
                cold_words.push(cur_w);
                cold_pads.push(0usize);
            } else {
                let want = (b.main as usize) & 3;
                let pad = (want + 4 - (cur_w & 3)) & 3;
                cur_w += pad;
                cold_words.push(cur_w);
                cold_pads.push(pad);
                cur_w += cold_len;
            }
        }
        // 3. Rebase branches (to this bank's stubs + hot/cold split) and append hot, then cold.
        let mut cold_tail = Vec::with_capacity(chunk.len());
        for (i, (ip, from_off, mut b)) in chunk.into_iter().enumerate() {
            let hot_off = (hot_words[i] * 4) as u32;
            let cold_off = (cold_words[i] * 4) as u32;
            split_and_rebase_block(&mut b, from_off, 0, hot_off as usize, cold_off as usize, bank_off, offs);
            let main = b.main;
            self.blocks.push(BlockEnt {
                ip,
                code_off: hot_off,
                cold_off,
                words: b.words.len() as u32,
                main,
                guest: b.guest,
                host_entry: b.host_entry,
                link_lo: self.links.len() as u32,
                link_n: b.links.len() as u32,
                reloc_lo: self.relocs.len() as u32,
                reloc_n: b.relocs.len() as u32,
                assume_lo: self.assume.len() as u32,
                assume_n: b.assume.len() as u32,
            });
            self.links.extend(b.links.iter().map(|&(s, f, c, t)| LinkEnt {
                site_off: word_to_off(s, main, hot_off, cold_off),
                fallback_off: word_to_off(f, main, hot_off, cold_off),
                cond_off: if c == u32::MAX { u32::MAX } else { word_to_off(c, main, hot_off, cold_off) },
                target: t,
            }));
            self.relocs.extend(b.relocs.iter().map(|r| RelocEnt::of_split(hot_off, cold_off, main, r)));
            self.assume.extend_from_slice(&b.assume);
            self.code.extend_from_slice(&b.words[..main as usize]);
            cold_tail.push((cold_pads[i], b));
        }
        for (pad, b) in cold_tail {
            for _ in 0..pad {
                self.code.push(0xD503_201F);
            }
            self.code.extend_from_slice(&b.words[b.main as usize..]);
        }
        // Keep the next chunk 16-byte aligned.
        while self.code.len() % 4 != 0 {
            self.code.push(0xD503_201F);
        }
    }

    pub(super) fn links_of(&self, b: &BlockEnt) -> &[LinkEnt] {
        &self.links[b.link_lo as usize..(b.link_lo + b.link_n) as usize]
    }
    pub(super) fn relocs_of(&self, b: &BlockEnt) -> &[RelocEnt] {
        &self.relocs[b.reloc_lo as usize..(b.reloc_lo + b.reloc_n) as usize]
    }
    pub(super) fn assume_of(&self, b: &BlockEnt) -> &[(u64, bool)] {
        &self.assume[b.assume_lo as usize..(b.assume_lo + b.assume_n) as usize]
    }
    fn hot_words_of(&self, b: &BlockEnt) -> &[u32] {
        let w = (b.code_off as usize - AOT_STUBS_BYTES) / 4;
        &self.code[w..w + b.main as usize]
    }
    fn cold_words_of(&self, b: &BlockEnt) -> &[u32] {
        let cold_len = (b.words - b.main) as usize;
        if cold_len == 0 {
            return &[];
        }
        let w = (b.cold_off as usize - AOT_STUBS_BYTES) / 4;
        &self.code[w..w + cold_len]
    }

    /// Structural sanity (a corrupt or hostile file must not make us patch outside the
    /// region, overlap a stub bank, or exceed ARM64 ±128 MiB stub-branch range).
    fn valid(&self) -> bool {
        let Some(end) = (AOT_STUBS_BYTES as u64).checked_add(self.code_bytes() as u64) else {
            return false;
        };
        if end > u32::MAX as u64 {
            return false;
        }
        let banks = self.stub_banks();
        if banks.is_empty() || banks[0] != 0 {
            return false;
        }
        for (bi, &bo) in banks.iter().enumerate() {
            if bo % 16 != 0 {
                return false;
            }
            let bo64 = bo as u64;
            let b_end = self.bank_end(bi) as u64;
            if bo64 + AOT_STUBS_BYTES as u64 > b_end || b_end > end {
                return false;
            }
            if bi > 0 && bo <= banks[bi - 1] {
                return false;
            }
            // Every code word inside a bank must stay within ARM64's ±128 MiB `B`/`BL` range
            // of that bank's stubs at `bo`.
            if b_end - bo64 > (128u64 << 20) {
                return false;
            }
        }
        let in_code = |off: u32, len: u64, align: u32| -> bool {
            if off % align != 0 {
                return false;
            }
            let bi = self.bank_index_of(off);
            let lo = banks[bi] as u64 + AOT_STUBS_BYTES as u64;
            let hi = self.bank_end(bi) as u64;
            let off64 = off as u64;
            off64 >= lo && off64.checked_add(len).is_some_and(|e| e <= hi)
        };
        let range = |lo: u32, n: u32, len: usize| (lo as u64 + n as u64) <= len as u64;
        self.blocks.iter().all(|b| {
            b.main <= b.words
                && in_code(b.code_off, b.main as u64 * 4, 4)
                && (b.words == b.main
                    || (in_code(b.cold_off, (b.words - b.main) as u64 * 4, 4)
                        && self.bank_index_of(b.code_off) == self.bank_index_of(b.cold_off)))
                && range(b.link_lo, b.link_n, self.links.len())
                && range(b.reloc_lo, b.reloc_n, self.relocs.len())
                && range(b.assume_lo, b.assume_n, self.assume.len())
        }) && self.links.iter().all(|l| {
            in_code(l.site_off, 4, 4)
                && in_code(l.fallback_off, 4, 4)
                && (l.cond_off == u32::MAX || in_code(l.cond_off, 4, 4))
        }) && self.relocs.iter().all(|r| {
            in_code(r.slot_off, 8, 8)
                && r.kind <= RELOC_QPC_STATE
                && (r.kind == RELOC_HOST_FN || r.kind == RELOC_QPC_STATE || r.src != 0)
        })
    }
}

fn put32(w: &mut Vec<u8>, v: u32) {
    w.extend_from_slice(&v.to_le_bytes());
}
fn put64(w: &mut Vec<u8>, v: u64) {
    w.extend_from_slice(&v.to_le_bytes());
}

struct Rd<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> Rd<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.b.get(self.p..self.p.checked_add(n)?)?;
        self.p += n;
        Some(s)
    }
    fn u32(&mut self) -> Option<u32> {
        let s = self.take(4)?;
        Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn u64(&mut self) -> Option<u64> {
        let s = self.take(8)?;
        Some(u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]))
    }
}

/// Header flag: built in the background during a run; verify at its first load
/// ([`load_module`]), then cleared in the file.
const FLAG_VERIFY_PENDING: u32 = 1;

/// Serialise (little-endian, v6): header, stub banks, blocks, links, relocs, assumptions,
/// code, checksum.
fn encode(name: &str, key: &AotKey, img: &Image, flags: u32) -> Vec<u8> {
    encode_with_version(name, key, img, flags, FORMAT_VERSION)
}

fn encode_with_version(name: &str, key: &AotKey, img: &Image, flags: u32, version: u32) -> Vec<u8> {
    let base = key.image_base;
    let banks = img.stub_banks();
    let cap = 256
        + name.len()
        + banks.len() * 4
        + img.blocks.len() * 52
        + img.links.len() * 20
        + img.relocs.len() * 24
        + img.assume.len() * 16
        + img.code_bytes();
    let mut w = Vec::with_capacity(cap);
    w.extend_from_slice(&MAGIC);
    put32(&mut w, version);
    put32(&mut w, flags);
    for v in key.words() {
        put64(&mut w, v);
    }
    put32(&mut w, name.len() as u32);
    w.extend_from_slice(name.as_bytes());
    for s in img.stubs {
        put32(&mut w, s);
    }
    put32(&mut w, AOT_STUBS_BYTES as u32);
    if version >= FORMAT_VERSION {
        put32(&mut w, banks.len() as u32);
        for &bo in banks {
            put32(&mut w, bo);
        }
    }
    for n in [img.blocks.len(), img.links.len(), img.relocs.len(), img.assume.len(), img.code.len()] {
        put32(&mut w, n as u32);
    }
    for b in &img.blocks {
        for v in [
            b.ip.wrapping_sub(base) as u32,
            b.code_off,
            b.cold_off,
            b.words,
            b.main,
            b.guest,
            b.host_entry as u32,
            b.link_lo,
            b.link_n,
            b.reloc_lo,
            b.reloc_n,
            b.assume_lo,
            b.assume_n,
        ] {
            put32(&mut w, v);
        }
    }
    for l in &img.links {
        put32(&mut w, l.site_off);
        put32(&mut w, l.fallback_off);
        put32(&mut w, l.cond_off);
        put64(&mut w, l.target);
    }
    for r in &img.relocs {
        put32(&mut w, r.slot_off);
        put32(&mut w, r.bail as u32 | (r.kind as u32) << 8);
        put64(&mut w, r.va);
        put64(&mut w, r.src);
    }
    for &(va, host) in &img.assume {
        put64(&mut w, va);
        put64(&mut w, host as u64);
    }
    for &c in &img.code {
        put32(&mut w, c);
    }
    let mut h = H128::new(0xC5);
    h.bytes(&w);
    put64(&mut w, h.finish()[0]);
    w
}

/// Replace the header flags of an encoded file (checksum recomputed). False: too short.
fn set_flags(bytes: &mut [u8], flags: u32) -> bool {
    if bytes.len() < 24 {
        return false;
    }
    bytes[12..16].copy_from_slice(&flags.to_le_bytes());
    let n = bytes.len() - 8;
    let mut h = H128::new(0xC5);
    h.bytes(&bytes[..n]);
    bytes[n..].copy_from_slice(&h.finish()[0].to_le_bytes());
    true
}

/// Parse an encoded v5 (single-bank, upgraded cleanly) or v6 (multi-bank) `.aot` file:
/// `(module name, key, image, header flags)`.
pub(super) fn decode(bytes: &[u8]) -> Option<(String, AotKey, Image, u32)> {
    if bytes.len() < 16 {
        return None;
    }
    let (body, sum) = bytes.split_at(bytes.len() - 8);
    let mut h = H128::new(0xC5);
    h.bytes(body);
    if h.finish()[0].to_le_bytes() != sum {
        return None;
    }
    let mut r = Rd { b: body, p: 0 };
    if r.take(8)? != MAGIC {
        return None;
    }
    let version = r.u32()?;
    if version != FORMAT_VERSION_V5 && version != FORMAT_VERSION {
        return None;
    }
    let flags = r.u32()?;
    let mut k = [0u64; 6];
    for v in k.iter_mut() {
        *v = r.u64()?;
    }
    let key = AotKey { image_hash: [k[0], k[1]], image_base: k[2], image_size: k[3], codegen_hash: k[4], knobs: k[5] };
    let nlen = r.u32()? as usize;
    let name = String::from_utf8(r.take(nlen)?.to_vec()).ok()?;
    let mut img = Image::default();
    for s in img.stubs.iter_mut() {
        *s = r.u32()?;
    }
    if r.u32()? as usize != AOT_STUBS_BYTES {
        return None;
    }
    if version >= FORMAT_VERSION {
        let nbanks = r.u32()? as usize;
        if nbanks == 0 || nbanks.checked_mul(4).is_none_or(|b| b > body.len() - r.p) {
            return None;
        }
        img.stub_banks.clear();
        img.stub_banks.reserve(nbanks);
        for _ in 0..nbanks {
            img.stub_banks.push(r.u32()?);
        }
    } else {
        img.stub_banks = vec![0];
    }
    let (nb, nl, nr, na, nc) = (r.u32()? as usize, r.u32()? as usize, r.u32()? as usize, r.u32()? as usize, r.u32()? as usize);
    // Table sizes must fit the remaining body before allocating (overflow-safe).
    let need = nb
        .checked_mul(52)?
        .checked_add(nl.checked_mul(20)?)?
        .checked_add(nr.checked_mul(24)?)?
        .checked_add(na.checked_mul(16)?)?
        .checked_add(nc.checked_mul(4)?)?;
    if need > body.len() - r.p {
        return None;
    }
    img.blocks.reserve(nb);
    for _ in 0..nb {
        let mut v = [0u32; 13];
        for x in v.iter_mut() {
            *x = r.u32()?;
        }
        img.blocks.push(BlockEnt {
            ip: key.image_base.wrapping_add(v[0] as u64),
            code_off: v[1],
            cold_off: v[2],
            words: v[3],
            main: v[4],
            guest: v[5],
            host_entry: v[6] != 0,
            link_lo: v[7],
            link_n: v[8],
            reloc_lo: v[9],
            reloc_n: v[10],
            assume_lo: v[11],
            assume_n: v[12],
        });
    }
    img.links.reserve(nl);
    for _ in 0..nl {
        img.links.push(LinkEnt {
            site_off: r.u32()?,
            fallback_off: r.u32()?,
            cond_off: r.u32()?,
            target: r.u64()?,
        });
    }
    img.relocs.reserve(nr);
    for _ in 0..nr {
        let (slot_off, fl, va, src) = (r.u32()?, r.u32()?, r.u64()?, r.u64()?);
        if fl & !0x3FF != 0 {
            return None;
        }
        img.relocs.push(RelocEnt { slot_off, kind: (fl >> 8) as u8, bail: fl & 1 != 0, va, src });
    }
    img.assume.reserve(na);
    for _ in 0..na {
        img.assume.push((r.u64()?, r.u64()? != 0));
    }
    img.code.reserve(nc);
    for _ in 0..nc {
        img.code.push(r.u32()?);
    }
    if r.p != body.len() || !img.valid() {
        return None;
    }
    Some((name, key, img, flags))
}

fn write_atomic(path: &Path, data: &[u8]) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension(format!("tmp{}", std::process::id()));
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)
}

// ---------------------------------------------------------------------------------------------
// Build
// ---------------------------------------------------------------------------------------------

/// Virtual block offset from `vbase` used during parallel translation. Every stub sits in
/// `[0, AOT_STUBS_BYTES)` (`4 KiB`) and every block is at most `AOT_MAX_BLOCK_BYTES`
/// (`256 KiB`), so `64 MiB` cleanly separates PC-relative `b`/`bl` branches to shared stubs
/// from intra-block branches while staying well inside ARM64's ±128 MiB branch range.
const PARALLEL_VOFF: usize = 64 << 20;
/// Minimum candidate blocks before spawning parallel worker threads for block translation.
const PARALLEL_MIN_BLOCKS: usize = 64;
/// Minimum BFS frontier size before spawning parallel worker threads for CFG discovery.
const PARALLEL_DISCOVER_MIN_BLOCKS: usize = 512;
/// Stack size for parallel prewarm / build worker threads (macOS default is 512 KiB).
const PARALLEL_WORKER_STACK: usize = 8 << 20;

/// Reusable per-worker [`BlockCache`] pool for parallel offline prewarming / builds.
/// Each [`BlockCache`] reserves ~74 MiB of lazily-committed virtual memory; reusing one
/// pool across BFS levels, translation, and all mapped modules avoids repeated `mmap`/
/// `munmap` TLB shootdowns while giving every worker exclusive `&mut BlockCache` ownership.
struct WorkerCaches(Vec<Box<BlockCache>>);

impl WorkerCaches {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn get_one(&mut self) -> *mut BlockCache {
        if self.0.is_empty() {
            self.0.push(Box::new(BlockCache::new()));
        }
        &mut *self.0[0]
    }
    fn ensure(&mut self, n: usize) -> &mut [Box<BlockCache>] {
        while self.0.len() < n {
            self.0.push(Box::new(BlockCache::new()));
        }
        &mut self.0[..n]
    }
}

/// Rebase a block's PC-relative instructions when placing its hot main path `[0, main)` at
/// `hot_off` and its cold out-of-line tail `[main, words.len())` at `cold_off` inside stub
/// bank `target_bank_off` (translated originally as a single contiguous block at `from_off`
/// targeting stubs at `from_bank_off`). Only instruction words in `[0, b.cold_code_end)` are
/// scanned; literal-pool data words are untouched.
fn split_and_rebase_block(
    b: &mut jit::AotBlock,
    from_off: usize,
    from_bank_off: usize,
    hot_off: usize,
    cold_off: usize,
    target_bank_off: usize,
    offs: &AotStubOffs,
) {
    let from_rel_word = ((from_off - from_bank_off) / 4) as i64;
    let hot_word = (hot_off / 4) as i64;
    let cold_word = (cold_off / 4) as i64;
    let target_bank_word = (target_bank_off / 4) as i64;
    let main = b.main as usize;
    let delta = cold_word - (hot_word + main as i64);
    let stubs_words = (AOT_STUBS_BYTES / 4) as i64;
    let scan_end = (b.cold_code_end as usize).min(b.words.len());
    if from_off - from_bank_off == hot_off - target_bank_off && delta == 0 {
        return;
    }
    for i in 0..scan_end {
        let w = b.words[i];
        let in_hot = i < main;
        if (w & 0x7C00_0000) == 0x1400_0000 {
            // ARM64 B or BL <imm26>.
            let imm26 = ((((w & 0x03FF_FFFF) as i32) << 6) >> 6) as i64;
            let local_t = i as i64 + imm26;
            let stub_t = from_rel_word + local_t;
            if (0..stubs_words).contains(&stub_t)
                && offs.iter().any(|&st| st != u32::MAX && st as i64 == stub_t)
            {
                let new_pc = if in_hot { hot_word + i as i64 } else { cold_word + (i - main) as i64 };
                let new_off = (target_bank_word + stub_t) - new_pc;
                debug_assert!((-(1i64 << 25)..(1i64 << 25)).contains(&new_off), "imm26 stub branch overflow {new_off}");
                b.words[i] = (w & 0xFC00_0000) | ((new_off as u32) & 0x03FF_FFFF);
            } else if delta != 0 {
                if in_hot && local_t >= main as i64 {
                    let new_off = imm26 + delta;
                    b.words[i] = (w & 0xFC00_0000) | ((new_off as u32) & 0x03FF_FFFF);
                } else if !in_hot && local_t < main as i64 {
                    let new_off = imm26 - delta;
                    b.words[i] = (w & 0xFC00_0000) | ((new_off as u32) & 0x03FF_FFFF);
                }
            }
        } else if delta != 0
            && ((w & 0xFF00_0010) == 0x5400_0000
                || (w & 0x7E00_0000) == 0x3400_0000
                || (w & 0xFF00_0000) == 0x5800_0000
                || (w & 0xFF00_0000) == 0x9C00_0000)
        {
            // ARM64 B.cond / CBZ / CBNZ / LDR Xt,lit / LDR Qt,lit (<imm19> in bits 23..5).
            let imm19 = ((((w >> 5) & 0x7_FFFF) as i32) << 13) >> 13;
            let local_t = i as i64 + imm19 as i64;
            let new_off = if in_hot && local_t >= main as i64 {
                Some(imm19 as i64 + delta)
            } else if !in_hot && local_t < main as i64 {
                Some(imm19 as i64 - delta)
            } else {
                None
            };
            if let Some(off) = new_off {
                debug_assert!((-(1i64 << 18)..(1i64 << 18)).contains(&off), "imm19 overflow {off}");
                b.words[i] = (w & 0xFF00_001F) | (((off as u32) & 0x7_FFFF) << 5);
            }
        } else if delta != 0 && (w & 0x7E00_0000) == 0x3600_0000 {
            // ARM64 TBZ / TBNZ (<imm14> in bits 18..5, ±32 KiB).
            let imm14 = ((((w >> 5) & 0x3FFF) as i32) << 18) >> 18;
            let local_t = i as i64 + imm14 as i64;
            let new_off = if in_hot && local_t >= main as i64 {
                Some(imm14 as i64 + delta)
            } else if !in_hot && local_t < main as i64 {
                Some(imm14 as i64 - delta)
            } else {
                None
            };
            if let Some(off) = new_off {
                debug_assert!((-(1i64 << 13)..(1i64 << 13)).contains(&off), "imm14 overflow {off}");
                b.words[i] = (w & 0xFFF8_001F) | (((off as u32) & 0x3FFF) << 5);
            }
        }
    }
}

/// Chunked multi-bank hot/cold assembler shared by [`build_image`] and
/// [`build_image_with_threads`] so single-threaded and multi-threaded AOT builds produce
/// bit-for-bit identical images across arbitrarily large binaries.
struct ChunkAssembler {
    pending: Vec<(u64, usize, jit::AotBlock)>,
    hot_w: usize,
    cold_w: usize,
    cur_bank_off: usize,
    bank_span: usize,
    max_code: usize,
}

impl ChunkAssembler {
    fn with_limits(bank_span: usize, max_code: usize) -> Self {
        Self {
            pending: Vec::new(),
            hot_w: 0,
            cold_w: 0,
            cur_bank_off: 0,
            bank_span,
            max_code,
        }
    }

    fn push(&mut self, img: &mut Image, offs: &AotStubOffs, ip: u64, from_off: usize, b: jit::AotBlock) -> bool {
        const MAX_CHUNK_HOT: usize = 3072; // 12 KiB of dense hot code per chunk
        const MAX_CHUNK_SPAN: usize = 7000; // < 8192 words (±32 KiB tbz/tbnz limit)
        let main = b.main as usize;
        let cold = (b.words.len() - main) + 3;
        if !self.pending.is_empty()
            && (self.hot_w + main > MAX_CHUNK_HOT || self.hot_w + self.cold_w + main + cold > MAX_CHUNK_SPAN)
        {
            self.flush(img, offs);
        }
        // Worst-case bytes added by the pending chunk + this block + 16B chunk trailing pad.
        let est_after = AOT_STUBS_BYTES + img.code_bytes() + (self.hot_w + self.cold_w + main + cold + 3) * 4;
        if est_after > self.cur_bank_off + self.bank_span {
            self.flush(img, offs);
            let after_flush = AOT_STUBS_BYTES + img.code_bytes();
            let block_bytes = (main + cold + 3) * 4;
            // Only open a new bank if the current bank actually contains code (prevents empty banks).
            if after_flush > self.cur_bank_off + AOT_STUBS_BYTES {
                if after_flush + AOT_STUBS_BYTES + block_bytes > self.max_code {
                    log::warn!(
                        "emu aot: code limit ({} MiB) reached at {ip:#x}; remaining blocks left to the JIT",
                        self.max_code >> 20
                    );
                    return false;
                }
                self.cur_bank_off = img.start_new_bank();
            }
        }
        let est_after = AOT_STUBS_BYTES + img.code_bytes() + (self.hot_w + self.cold_w + main + cold + 3) * 4;
        if est_after > self.max_code {
            self.flush(img, offs);
            log::warn!(
                "emu aot: code limit ({} MiB) reached at {ip:#x}; remaining blocks left to the JIT",
                self.max_code >> 20
            );
            return false;
        }
        self.hot_w += main;
        self.cold_w += cold;
        self.pending.push((ip, from_off, b));
        true
    }

    fn flush(&mut self, img: &mut Image, offs: &AotStubOffs) {
        if !self.pending.is_empty() {
            let chunk = core::mem::take(&mut self.pending);
            img.push_chunk(chunk, offs, self.cur_bank_off);
            self.hot_w = 0;
            self.cold_w = 0;
        }
    }
}

/// Translate `ips` (sorted, deduplicated) in AOT emission mode into an image laid out for a
/// region at `vbase`. Blocks the translator rejects are skipped (the JIT handles them).
unsafe fn build_image(bc: *mut BlockCache, ips: &[u64], vbase: usize) -> Option<Image> {
    build_image_with_limits(bc, ips, vbase, AOT_BANK_SPAN, MAX_CODE)
}

unsafe fn build_image_with_limits(
    bc: *mut BlockCache,
    ips: &[u64],
    vbase: usize,
    bank_span: usize,
    max_code: usize,
) -> Option<Image> {
    let mut stub_buf = vec![0u32; AOT_STUBS_BYTES / 4];
    let offs = jit::aot_emit_stubs(stub_buf.as_mut_ptr(), vbase)?;
    let mut img = Image { stubs: offs, ..Image::default() };
    let mut asm = ChunkAssembler::with_limits(bank_span, max_code);
    let mut scratch = Vec::new();
    for &ip in ips {
        if let Some(b) = jit::aot_translate(bc, ip, vbase + PARALLEL_VOFF, vbase, &offs, &mut scratch) {
            if !asm.push(&mut img, &offs, ip, PARALLEL_VOFF, b) {
                break;
            }
        }
    }
    asm.flush(&mut img, &offs);
    Some(img)
}

#[cfg(test)]
pub(super) unsafe fn build_image_for_test(bc: *mut BlockCache, ips: &[u64]) -> Option<Image> {
    build_image(bc, ips, BUILD_VBASE)
}

#[cfg(test)]
pub(super) unsafe fn build_image_for_test_with_limits(
    bc: *mut BlockCache,
    ips: &[u64],
    bank_span: usize,
    max_code: usize,
) -> Option<Image> {
    build_image_with_limits(bc, ips, BUILD_VBASE, bank_span, max_code)
}

#[cfg(test)]
pub(super) fn encode_for_test(name: &str, key: &AotKey, img: &Image) -> Vec<u8> {
    encode(name, key, img, 0)
}

/// Translate `ips` (sorted, deduplicated) in AOT emission mode using up to `threads` worker
/// threads, assembling blocks in exact `ips` order so output is bit-for-bit identical to
/// [`build_image`] regardless of thread count.
///
/// Workers never touch executable pages: each worker exclusively borrows a [`BlockCache`]
/// from `pool` and a private heap scratch buffer, translates chunks at [`PARALLEL_VOFF`],
/// and the main thread rebases stub branches to each block's deterministic stub bank via
/// [`split_and_rebase_block`].
unsafe fn build_image_with_threads(
    pool: &mut WorkerCaches,
    ips: &[u64],
    vbase: usize,
    threads: usize,
) -> Option<Image> {
    build_image_with_threads_and_limits(pool, ips, vbase, threads, AOT_BANK_SPAN, MAX_CODE)
}

unsafe fn build_image_with_threads_and_limits(
    pool: &mut WorkerCaches,
    ips: &[u64],
    vbase: usize,
    threads: usize,
    bank_span: usize,
    max_code: usize,
) -> Option<Image> {
    if threads <= 1 || ips.len() < 2 {
        return build_image_with_limits(pool.get_one(), ips, vbase, bank_span, max_code);
    }

    let mut stub_buf = vec![0u32; AOT_STUBS_BYTES / 4];
    let offs = jit::aot_emit_stubs(stub_buf.as_mut_ptr(), vbase)?;
    // Ensure process-wide codegen knob OnceLocks are initialized on the caller thread
    // before spawning workers.
    let _ = jit::aot_knob_bits();

    let workers = threads.min(ips.len());
    let chunk_len = (ips.len() / (workers * 8)).clamp(1, 256);
    let chunks: Vec<&[u64]> = ips.chunks(chunk_len).collect();
    let n_chunks = chunks.len();
    let next_chunk = AtomicUsize::new(0);
    let worker_bcs = pool.ensure(workers);

    let mut all_chunks: Vec<(usize, Vec<(u64, jit::AotBlock)>)> = Vec::with_capacity(n_chunks);
    std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(workers);
        for (wi, local_bc) in worker_bcs.iter_mut().enumerate() {
            let chunks_ref = &chunks;
            let next_ref = &next_chunk;
            let h = std::thread::Builder::new()
                .name(format!("port-aot-w{wi}"))
                .stack_size(PARALLEL_WORKER_STACK)
                .spawn_scoped(scope, move || {
                    let bcp: *mut BlockCache = &mut **local_bc;
                    let mut scratch = Vec::with_capacity(jit::AOT_MAX_BLOCK_BYTES / 4);
                    let mut done: Vec<(usize, Vec<(u64, jit::AotBlock)>)> = Vec::new();
                    loop {
                        let ci = next_ref.fetch_add(1, Ordering::Relaxed);
                        if ci >= n_chunks {
                            break;
                        }
                        let slice = chunks_ref[ci];
                        let mut out = Vec::with_capacity(slice.len());
                        for &ip in slice {
                            if let Some(b) = jit::aot_translate(
                                bcp,
                                ip,
                                vbase + PARALLEL_VOFF,
                                vbase,
                                &offs,
                                &mut scratch,
                            ) {
                                out.push((ip, b));
                            }
                        }
                        done.push((ci, out));
                    }
                    done
                })
                .expect("spawn parallel AOT worker");
            handles.push(h);
        }
        for h in handles {
            all_chunks.extend(h.join().expect("parallel AOT worker panicked"));
        }
    });

    all_chunks.sort_unstable_by_key(|(ci, _)| *ci);
    let mut img = Image { stubs: offs, ..Image::default() };
    let mut asm = ChunkAssembler::with_limits(bank_span, max_code);
    'assemble: for (_, blocks) in all_chunks {
        for (ip, b) in blocks {
            if !asm.push(&mut img, &offs, ip, PARALLEL_VOFF, b) {
                break 'assemble;
            }
        }
    }
    asm.flush(&mut img, &offs);
    Some(img)
}

// ---------------------------------------------------------------------------------------------
// Static CFG successor & RIP-relative switch / jump-table discovery
// ---------------------------------------------------------------------------------------------

/// Safety cap on jump-table entries scanned per indirect branch.
const MAX_JT_ENTRIES: usize = 4096;
/// Maximum number of hint-upgrade re-analyses per already-seen block during CFG closure.
const MAX_JT_REANALYZE: u8 = 2;

/// Abstract GPR value tracked across basic blocks for RIP-relative switch / jump-table
/// discovery during offline static CFG closure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JtVal {
    Unknown,
    /// Known constant address (`lea reg, [rip + disp]`, `mov reg, imm64`, const arithmetic).
    Const(u64),
    /// Switch index with optional exclusive upper bound `0..bound` (from `cmp`/`ja` etc. or
    /// bitmask `and reg, mask`).
    Index { bound: Option<usize> },
    /// Pre-scaled switch index (`shl reg, 2|3` or `lea reg, [idx*4|8]`): `idx * scale`.
    ScaledIndex { scale: u8, indir: Option<(u64, u8)>, bound: Option<usize> },
    /// Secondary index loaded from an 8-bit or 16-bit indirection table:
    /// `idx2 = *(u8|u16)(tbl_va + idx * elem_bytes)`.
    IndirIndex { tbl_va: u64, elem_bytes: u8, bound: Option<usize> },
    /// Address of a jump-table slot (`lea rPtr, [rTbl + idx*stride]` or `add rTbl, scaled_idx`),
    /// ready for `mov reg, [rPtr]` / `movsxd reg, [rPtr]` / `jmp qword ptr [rPtr]`.
    TablePtr { tbl_va: u64, stride: u8, indir: Option<(u64, u8)>, bound: Option<usize> },
    /// 32-bit offset loaded from a jump table: `off = *(u32|i32)(tbl_va + idx * 4)`.
    /// `assumed_base_reg`: `Some(r)` when `r` was unknown (e.g. MSVC hoisted `__ImageBase`
    /// in `[rBase + idx*4 + rva_tbl]`) and `tbl_va` was formed as `image_base + disp`.
    TableOff32 {
        tbl_va: u64,
        signed: bool,
        assumed_base_reg: Option<u8>,
        indir: Option<(u64, u8)>,
        bound: Option<usize>,
    },
    /// Resolved 32-bit-offset jump target (`base_va ± entry`), ready for `jmp reg`.
    Target32 {
        base_va: u64,
        tbl_va: u64,
        signed: bool,
        sub_from_base: bool,
        assumed_base: bool,
        indir: Option<(u64, u8)>,
        bound: Option<usize>,
    },
    /// Resolved 64-bit-pointer jump target (`*(u64)(tbl_va + idx * 8)`), ready for `jmp reg`.
    Target64 {
        tbl_va: u64,
        indir: Option<(u64, u8)>,
        bound: Option<usize>,
    },
}

type JtRegs = [JtVal; 16];

fn gpr_idx(reg: iced_x86::Register) -> Option<usize> {
    if reg == iced_x86::Register::None {
        return None;
    }
    let full = reg.full_register();
    full.is_gpr64().then(|| full.number())
}

fn op_imm(ins: &iced_x86::Instruction, op: u32) -> Option<u64> {
    use iced_x86::OpKind as K;
    match ins.op_kind(op) {
        K::Immediate8 => Some(ins.immediate8() as u64),
        K::Immediate16 => Some(ins.immediate16() as u64),
        K::Immediate32 => Some(ins.immediate32() as u64),
        K::Immediate64 => Some(ins.immediate64()),
        K::Immediate8to16 => Some(ins.immediate8to16() as u16 as u64),
        K::Immediate8to32 => Some(ins.immediate8to32() as u32 as u64),
        K::Immediate8to64 => Some(ins.immediate8to64() as u64),
        K::Immediate32to64 => Some(ins.immediate32to64() as u64),
        _ => None,
    }
}

fn extract_idx(v: JtVal) -> (Option<(u64, u8)>, Option<usize>) {
    match v {
        JtVal::Index { bound } => (None, bound),
        JtVal::IndirIndex { tbl_va, elem_bytes, bound } => (Some((tbl_va, elem_bytes)), bound),
        _ => (None, None),
    }
}

fn pack_hint(regs: &JtRegs) -> Option<Box<JtRegs>> {
    if regs.iter().all(|v| matches!(v, JtVal::Unknown)) {
        None
    } else {
        Some(Box::new(*regs))
    }
}

fn jt_val_rank(v: JtVal) -> u8 {
    match v {
        JtVal::Unknown => 0,
        JtVal::Index { bound: None } => 1,
        JtVal::Index { bound: Some(_) } => 2,
        JtVal::IndirIndex { bound: None, .. } | JtVal::ScaledIndex { bound: None, .. } => 3,
        JtVal::IndirIndex { bound: Some(_), .. } | JtVal::ScaledIndex { bound: Some(_), .. } => 4,
        JtVal::Const(_) => 5,
        JtVal::TablePtr { .. } => 6,
        JtVal::TableOff32 { .. } => 7,
        JtVal::Target32 { .. } | JtVal::Target64 { .. } => 8,
    }
}

/// Merge incoming hint `new` into `dst`, keeping richer register knowledge from both
/// predecessors. Returns `true` if any register strictly gained information.
fn merge_jt_regs(dst: &mut JtRegs, new: &JtRegs) -> bool {
    let mut improved = false;
    for (d, &n) in dst.iter_mut().zip(new.iter()) {
        if *d == n || matches!(n, JtVal::Unknown) {
            continue;
        }
        let merged = match (*d, n) {
            (JtVal::Unknown, v) => v,
            (JtVal::Index { bound: b1 }, JtVal::Index { bound: b2 }) => {
                let b = match (b1, b2) {
                    (Some(x), Some(y)) => Some(x.max(y)),
                    (Some(x), None) | (None, Some(x)) => Some(x),
                    (None, None) => None,
                };
                JtVal::Index { bound: b }
            }
            (old, cand) if jt_val_rank(cand) > jt_val_rank(old) => cand,
            (old, _) => old,
        };
        if merged != *d {
            *d = merged;
            improved = true;
        }
    }
    improved
}

fn adjust_index_bound(bound: Option<usize>, signed_delta: i64) -> JtVal {
    if signed_delta == 0 {
        return JtVal::Index { bound };
    }
    if signed_delta < 0 {
        let sub = (-signed_delta) as u64;
        if sub < MAX_JT_ENTRIES as u64 {
            return JtVal::Index {
                bound: bound.and_then(|b| b.checked_sub(sub as usize)).filter(|&b| b > 0),
            };
        }
    } else if (signed_delta as u64) < MAX_JT_ENTRIES as u64 {
        return JtVal::Index {
            bound: bound.map(|b| b.saturating_add(signed_delta as usize).min(MAX_JT_ENTRIES)),
        };
    }
    JtVal::Unknown
}

fn combine_add_target(lhs: JtVal, lhs_reg: usize, rhs: JtVal, rhs_reg: usize, img_base: u64) -> JtVal {
    let pair = |off: JtVal, base_val: JtVal, base_reg: usize| -> Option<JtVal> {
        let JtVal::TableOff32 { tbl_va, signed, assumed_base_reg, indir, bound } = off else {
            return None;
        };
        match (assumed_base_reg, base_val) {
            (None, JtVal::Const(base_va)) => Some(JtVal::Target32 {
                base_va,
                tbl_va,
                signed,
                sub_from_base: false,
                assumed_base: false,
                indir,
                bound,
            }),
            (Some(breg), _) if breg as usize == base_reg => Some(JtVal::Target32 {
                base_va: img_base,
                tbl_va,
                signed,
                sub_from_base: false,
                assumed_base: true,
                indir,
                bound,
            }),
            _ => None,
        }
    };
    if let Some(t) = pair(lhs, rhs, rhs_reg) {
        return t;
    }
    if let Some(t) = pair(rhs, lhs, lhs_reg) {
        return t;
    }
    match (lhs, rhs) {
        (JtVal::Const(a), JtVal::Const(b)) => JtVal::Const(a.wrapping_add(b)),
        (JtVal::Const(tbl_va), JtVal::ScaledIndex { scale, indir, bound })
        | (JtVal::ScaledIndex { scale, indir, bound }, JtVal::Const(tbl_va)) => {
            JtVal::TablePtr { tbl_va, stride: scale, indir, bound }
        }
        _ => JtVal::Unknown,
    }
}

fn eval_mem_load(
    ins: &iced_x86::Instruction,
    regs: &JtRegs,
    signed: bool,
    img_base: u64,
    img_size: u64,
) -> JtVal {
    if ins.is_ip_rel_memory_operand() {
        return JtVal::Unknown;
    }
    let msize = ins.memory_size().size();
    let scale = ins.memory_index_scale();
    let mbase = gpr_idx(ins.memory_base());
    let midx = gpr_idx(ins.memory_index());
    let disp = ins.memory_displacement64();

    // Pre-computed table slot pointer: `mov eax, [rPtr]` / `movsxd rax, [rPtr]` / `jmp [rPtr]`.
    if midx.is_none() && disp == 0 {
        if let Some(b) = mbase {
            if let JtVal::TablePtr { tbl_va, stride, indir, bound } = regs[b] {
                if msize == 4 && stride == 4 {
                    return JtVal::TableOff32 {
                        tbl_va,
                        signed,
                        assumed_base_reg: None,
                        indir,
                        bound,
                    };
                }
                if !signed && msize == 8 && stride == 8 {
                    return JtVal::Target64 { tbl_va, indir, bound };
                }
            }
        }
        return JtVal::Unknown;
    }

    let Some(ix) = midx else {
        return JtVal::Unknown;
    };

    // Pre-scaled index in `[rTbl + rScaledIdx]` (scale == 1, e.g. after `shl rIdx, 2|3`).
    if scale == 1 {
        if let Some(b) = mbase {
            let scaled_pair = match (regs[b], regs[ix]) {
                (JtVal::Const(c), JtVal::ScaledIndex { scale: s, indir, bound }) => {
                    Some((c.wrapping_add(disp), s, indir, bound))
                }
                (JtVal::ScaledIndex { scale: s, indir, bound }, JtVal::Const(c)) => {
                    Some((c.wrapping_add(disp), s, indir, bound))
                }
                _ => None,
            };
            if let Some((tbl_va, s, indir, bound)) = scaled_pair {
                if msize == 4 && s == 4 {
                    return JtVal::TableOff32 {
                        tbl_va,
                        signed,
                        assumed_base_reg: None,
                        indir,
                        bound,
                    };
                }
                if !signed && msize == 8 && s == 8 {
                    return JtVal::Target64 { tbl_va, indir, bound };
                }
            }
        }
    }

    let (indir, bound) = extract_idx(regs[ix]);

    if msize == 4 && scale == 4 {
        match mbase {
            Some(b) => match regs[b] {
                JtVal::Const(c) => JtVal::TableOff32 {
                    tbl_va: c.wrapping_add(disp),
                    signed,
                    assumed_base_reg: None,
                    indir,
                    bound,
                },
                _ if disp > 0 && disp < img_size => JtVal::TableOff32 {
                    tbl_va: img_base.wrapping_add(disp),
                    signed,
                    assumed_base_reg: Some(b as u8),
                    indir,
                    bound,
                },
                _ => JtVal::Unknown,
            },
            None if disp >= img_base && disp < img_base.saturating_add(img_size) => {
                JtVal::TableOff32 {
                    tbl_va: disp,
                    signed,
                    assumed_base_reg: None,
                    indir,
                    bound,
                }
            }
            None => JtVal::Unknown,
        }
    } else if !signed && msize == 8 && scale == 8 {
        match mbase {
            Some(b) => match regs[b] {
                JtVal::Const(c) => JtVal::Target64 { tbl_va: c.wrapping_add(disp), indir, bound },
                _ => JtVal::Unknown,
            },
            None if disp >= img_base && disp < img_base.saturating_add(img_size) => {
                JtVal::Target64 { tbl_va: disp, indir, bound }
            }
            None => JtVal::Unknown,
        }
    } else {
        JtVal::Unknown
    }
}

fn eval_mem_movzx(
    ins: &iced_x86::Instruction,
    regs: &JtRegs,
    img_base: u64,
    img_size: u64,
) -> JtVal {
    if ins.is_ip_rel_memory_operand() {
        return JtVal::Unknown;
    }
    let msize = ins.memory_size().size();
    if msize != 1 && msize != 2 {
        return JtVal::Unknown;
    }
    let scale = ins.memory_index_scale();
    if scale != msize as u32 {
        return JtVal::Unknown;
    }
    let mbase = gpr_idx(ins.memory_base());
    let midx = gpr_idx(ins.memory_index());
    let disp = ins.memory_displacement64();

    match (mbase, midx) {
        (Some(b), Some(ix)) => {
            if let JtVal::Const(c) = regs[b] {
                let (_, bound) = extract_idx(regs[ix]);
                return JtVal::IndirIndex {
                    tbl_va: c.wrapping_add(disp),
                    elem_bytes: msize as u8,
                    bound,
                };
            }
            if scale == 1 {
                if let JtVal::Const(c) = regs[ix] {
                    let (_, bound) = extract_idx(regs[b]);
                    return JtVal::IndirIndex {
                        tbl_va: c.wrapping_add(disp),
                        elem_bytes: 1,
                        bound,
                    };
                }
            }
            // Hoisted __ImageBase heuristic: `movzx rIdx2, byte/word ptr [rBase + rIdx*s + rva]`.
            if disp > 0 && disp < img_size {
                let idx_reg = if scale == 1
                    && matches!(regs[b], JtVal::Index { bound: Some(_) })
                    && !matches!(regs[ix], JtVal::Index { bound: Some(_) })
                {
                    b
                } else {
                    ix
                };
                let (_, bound) = extract_idx(regs[idx_reg]);
                return JtVal::IndirIndex {
                    tbl_va: img_base.wrapping_add(disp),
                    elem_bytes: msize as u8,
                    bound,
                };
            }
            JtVal::Unknown
        }
        (None, Some(ix)) if disp >= img_base && disp < img_base.saturating_add(img_size) => {
            let (_, bound) = extract_idx(regs[ix]);
            JtVal::IndirIndex { tbl_va: disp, elem_bytes: msize as u8, bound }
        }
        _ => JtVal::Unknown,
    }
}

/// Validate that `va` lies inside an executable section of the module and begins with a
/// valid decoded x86-64 instruction.
unsafe fn valid_code_target(va: u64, text: &[(usize, usize)]) -> bool {
    let Some(&(_s, e)) = text.iter().find(|&&(s, e)| (s as u64..e as u64).contains(&va)) else {
        return false;
    };
    let avail = (e as u64 - va).min(15) as usize;
    if avail == 0 {
        return false;
    }
    let bytes = core::slice::from_raw_parts(va as usize as *const u8, avail);
    let mut dec = iced_x86::Decoder::with_ip(64, bytes, va, iced_x86::DecoderOptions::NONE);
    !dec.decode().is_invalid()
}

/// Validate that `va` lies inside an executable section, does not start with `0xCC` (`int3`)
/// or `[0x00, 0x00]` (`add [rax], al`) padding, and decodes cleanly via [`valid_code_target`].
unsafe fn is_plausible_code_entry(va: u64, text: &[(usize, usize)]) -> bool {
    let Some(&(_s, e)) = text.iter().find(|&&(s, e)| (s as u64..e as u64).contains(&va)) else {
        return false;
    };
    let avail = (e as u64 - va).min(15) as usize;
    if avail == 0 {
        return false;
    }
    let b0 = core::ptr::read_unaligned(va as usize as *const u8);
    if b0 == 0xCC {
        return false;
    }
    if b0 == 0x00
        && avail >= 2
        && core::ptr::read_unaligned((va as usize + 1) as *const u8) == 0x00
    {
        return false;
    }
    valid_code_target(va, text)
}

/// True if `va` lies at a clean function entry boundary inside an executable section:
/// 16-byte aligned relative to the executable section start (`((va - s) & 0xF) == 0`, matching
/// `(va & 0xF) == 0` for page-aligned PEs) OR immediately preceded within the same executable
/// section by `0xCC` (`int3`), `0x90` (`nop`), or `0xC3` (`ret`), AND passes
/// [`is_plausible_code_entry`].
unsafe fn is_clean_code_entry(va: u64, text: &[(usize, usize)]) -> bool {
    let Some(&(s, _e)) = text.iter().find(|&&(s, e)| (s as u64..e as u64).contains(&va)) else {
        return false;
    };
    let clean_boundary = ((va - s as u64) & 0xF) == 0
        || (va > s as u64
            && matches!(
                core::ptr::read_unaligned((va as usize - 1) as *const u8),
                0xCC | 0x90 | 0xC3
            ));
    clean_boundary && is_plausible_code_entry(va, text)
}

/// Read and validate switch / jump-table entries from a mapped module `[base, base + size)`.
/// Stops immediately on the first out-of-range, non-code, self-table, or cross-function
/// entry. Deduplicates targets deterministically.
unsafe fn resolve_jt_targets(
    kind: JtVal,
    base: usize,
    size: usize,
    text: &[(usize, usize)],
    fmap: &FuncMap,
    cur_func: Option<u32>,
) -> Vec<u64> {
    let img_lo = (base as u64).saturating_add(0x100);
    let img_hi = (base as u64).saturating_add(size as u64);
    let in_img = |addr: u64, len: u64| -> bool {
        addr >= img_lo && addr.checked_add(len).is_some_and(|end| end <= img_hi)
    };

    let (base_va, tbl_va, stride, signed, sub_from_base, assumed_base, indir, bound) = match kind {
        JtVal::Target32 { base_va, tbl_va, signed, sub_from_base, assumed_base, indir, bound } => {
            (base_va, tbl_va, 4u64, signed, sub_from_base, assumed_base, indir, bound)
        }
        JtVal::Target64 { tbl_va, indir, bound } => {
            (0u64, tbl_va, 8u64, false, false, false, indir, bound)
        }
        _ => return Vec::new(),
    };

    let limit = match bound {
        Some(0) => return Vec::new(),
        Some(n) => n.min(MAX_JT_ENTRIES),
        None => MAX_JT_ENTRIES,
    };

    let mut targets = Vec::new();
    let mut raw_count = 0usize;
    let mut max_jt_idx = 0usize;

    for i in 0..limit {
        let jt_idx: usize = if let Some((idx_tbl, elem_bytes)) = indir {
            let Some(off) = (i as u64).checked_mul(elem_bytes as u64) else { break };
            let Some(addr) = idx_tbl.checked_add(off) else { break };
            if !in_img(addr, elem_bytes as u64) {
                break;
            }
            let v = if elem_bytes == 1 {
                core::ptr::read_unaligned(addr as usize as *const u8) as usize
            } else {
                core::ptr::read_unaligned(addr as usize as *const u16) as usize
            };
            if v >= MAX_JT_ENTRIES {
                break;
            }
            v
        } else {
            i
        };

        let Some(off) = (jt_idx as u64).checked_mul(stride) else { break };
        let Some(entry_addr) = tbl_va.checked_add(off) else { break };
        if !in_img(entry_addr, stride) {
            break;
        }
        let target = if stride == 4 {
            let raw = core::ptr::read_unaligned(entry_addr as usize as *const u32);
            let disp = if signed { raw as i32 as i64 as u64 } else { raw as u64 };
            if sub_from_base {
                base_va.wrapping_sub(disp)
            } else {
                base_va.wrapping_add(disp)
            }
        } else {
            core::ptr::read_unaligned(entry_addr as usize as *const u64)
        };

        max_jt_idx = max_jt_idx.max(jt_idx);
        let tbl_end = tbl_va.saturating_add((max_jt_idx as u64 + 1).saturating_mul(stride));
        if target >= tbl_va && target < tbl_end {
            // A switch handler cannot jump inside its own jump-table data (guards against
            // zero-padded tables placed inside `.text`).
            break;
        }
        if !fmap.is_empty() {
            let rva = target.wrapping_sub(base as u64);
            if rva >= size as u64 || fmap.root_of(rva as u32) != cur_func {
                break;
            }
        }
        if !valid_code_target(target, text) {
            break;
        }
        raw_count += 1;
        if !targets.contains(&target) {
            targets.push(target);
        }
    }

    // Hoisted __ImageBase heuristic requires at least 2 valid entries (or an explicit bound of 1)
    // to avoid false positives on unrelated indexed dword loads.
    if assumed_base && raw_count < 2 && bound != Some(1) {
        return Vec::new();
    }
    targets
}

/// Kind of static CFG edge produced by [`analyze_block`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EdgeKind {
    /// Normal intra-function edge (fallthrough or conditional branch).
    Intra,
    /// Unconditional direct jump (intra-function, or tail-call into a `.pdata`-less leaf).
    TailBranch,
    /// Direct `call rel32` target (followed for `.pdata`-less leaf callees / intra-function calls).
    DirectCall,
    /// Switch / jump-table case target (strictly within the same `.pdata` function).
    JumpTable,
    /// `.pdata`-less leaf function entry loaded via `LEA reg, [rip + disp32]`.
    LeaLeaf,
}

/// Emit [`EdgeKind::LeaLeaf`] edges for `LEA reg, [rip + disp32]` instructions targeting
/// `.pdata`-less clean leaf function entries inside `text`.
#[inline]
unsafe fn push_lea_leaf_edges(
    insns: &[iced_x86::Instruction],
    img_base: u64,
    img_size: u64,
    text: &[(usize, usize)],
    fmap: &FuncMap,
    edges: &mut Vec<(u64, EdgeKind, Option<Box<JtRegs>>)>,
) {
    for ins in insns {
        if ins.mnemonic() == iced_x86::Mnemonic::Lea && ins.is_ip_rel_memory_operand() {
            let ea = ins.ip_rel_memory_address();
            if !text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ea)) {
                continue;
            }
            let rva = ea.wrapping_sub(img_base);
            if rva < img_size
                && (fmap.is_empty() || fmap.root_of(rva as u32).is_none())
                && is_clean_code_entry(ea, text)
            {
                edges.push((ea, EdgeKind::LeaLeaf, None));
            }
        }
    }
}

/// Result of analysing one basic block during static CFG discovery.
#[derive(Default)]
struct BlockStep {
    /// `(successor_ip, edge_kind, outgoing_register_hint)`.
    edges: Vec<(u64, EdgeKind, Option<Box<JtRegs>>)>,
    /// Number of unique targets discovered via a switch / jump table at this block's terminator.
    jt_targets: usize,
    /// True if this block ends in an indirect jump (`jmp reg` / `jmp [mem]`), so arrival of
    /// richer register hints later in BFS can re-analyse the dispatch.
    ends_indirect_jmp: bool,
}

/// Static successors of the block at `ip`, including RIP-relative switch / jump-table targets
/// when `jt_on` is true, direct call targets, `LEA reg, [rip + disp32]` `.pdata`-less leaf
/// targets, and outgoing register/bound hints.
#[allow(clippy::too_many_arguments)]
unsafe fn analyze_block(
    bc: *mut BlockCache,
    ip: u64,
    in_regs: Option<&JtRegs>,
    base: usize,
    size: usize,
    text: &[(usize, usize)],
    fmap: &FuncMap,
    jt_on: bool,
) -> BlockStep {
    use iced_x86::{ConditionCode as CC, FlowControl as F, Mnemonic as M, OpKind as K};
    let mut dummy = 0u64;
    let Ok((off, n)) = BlockCache::get(bc, ip, &mut dummy) else {
        return BlockStep::default();
    };
    let insns = &(&(*bc).arena)[off..off + n];
    let last = &insns[n - 1];
    let (tgt, next) = (last.near_branch_target(), last.next_ip());
    let fc = last.flow_control();
    let img_base = base as u64;
    let img_size = size as u64;

    if !jt_on {
        let mut step = BlockStep::default();
        match fc {
            F::Next | F::IndirectCall => step.edges.push((next, EdgeKind::Intra, None)),
            F::Call => {
                step.edges.push((next, EdgeKind::Intra, None));
                if tgt != 0 {
                    step.edges.push((tgt, EdgeKind::DirectCall, None));
                }
            }
            F::ConditionalBranch => {
                step.edges.push((tgt, EdgeKind::Intra, None));
                step.edges.push((next, EdgeKind::Intra, None));
            }
            F::UnconditionalBranch if tgt != 0 => {
                step.edges.push((tgt, EdgeKind::TailBranch, None));
            }
            _ => {}
        }
        push_lea_leaf_edges(insns, img_base, img_size, text, fmap, &mut step.edges);
        return step;
    }

    let mut regs: JtRegs = in_regs.copied().unwrap_or([JtVal::Unknown; 16]);
    let mut alias: [u16; 16] = core::array::from_fn(|i| 1u16 << i);
    // `(alias_mask, const_bound, reversed)`: `reversed = false` means `cmp reg, const`;
    // `reversed = true` means `cmp const, reg`.
    let mut last_cmp: Option<(u16, u64, bool)> = None;

    let clobber_alias = |alias: &mut [u16; 16], r: usize| {
        let bit = !(1u16 << r);
        for m in alias.iter_mut() {
            *m &= bit;
        }
        alias[r] = 1u16 << r;
    };
    let link_alias = |alias: &mut [u16; 16], dst: usize, src: usize| {
        if dst != src {
            let bit = !(1u16 << dst);
            for m in alias.iter_mut() {
                *m &= bit;
            }
            let cls = alias[src] | (1u16 << dst);
            for (k, m) in alias.iter_mut().enumerate() {
                if cls & (1u16 << k) != 0 {
                    *m = cls;
                }
            }
        }
    };
    let clear_cmp_reg = |last_cmp: &mut Option<(u16, u64, bool)>, r: usize| {
        if let Some((mut mask, imm, rev)) = *last_cmp {
            mask &= !(1u16 << r);
            *last_cmp = (mask != 0).then_some((mask, imm, rev));
        }
    };
    let copy_cmp_alias = |last_cmp: &mut Option<(u16, u64, bool)>, dst: usize, src: usize| {
        if let Some((mut mask, imm, rev)) = *last_cmp {
            if mask & (1u16 << src) != 0 {
                mask |= 1u16 << dst;
            } else {
                mask &= !(1u16 << dst);
            }
            *last_cmp = (mask != 0).then_some((mask, imm, rev));
        }
    };

    for ins in insns {
        let mn = ins.mnemonic();
        if mn == M::Cmp {
            last_cmp = if ins.op_count() >= 2 && ins.op_kind(0) == K::Register {
                match (gpr_idx(ins.op0_register()), op_imm(ins, 1)) {
                    (Some(r), Some(imm)) => Some((alias[r], imm, false)),
                    (Some(r0), None) if ins.op_kind(1) == K::Register => {
                        match gpr_idx(ins.op1_register()) {
                            Some(r1) => match (regs[r0], regs[r1]) {
                                (_, JtVal::Const(imm)) => Some((alias[r0], imm, false)),
                                (JtVal::Const(imm), _) => Some((alias[r1], imm, true)),
                                _ => None,
                            },
                            None => None,
                        }
                    }
                    _ => None,
                }
            } else {
                None
            };
            continue;
        }

        let modifies_flags = (ins.rflags_written()
            | ins.rflags_cleared()
            | ins.rflags_set()
            | ins.rflags_undefined())
            != 0;
        if modifies_flags {
            last_cmp = None;
        }

        match mn {
            M::Lea => {
                if let Some(dst) = gpr_idx(ins.op0_register()) {
                    let new_val = if ins.is_ip_rel_memory_operand() {
                        clobber_alias(&mut alias, dst);
                        JtVal::Const(ins.ip_rel_memory_address())
                    } else {
                        let mbase = gpr_idx(ins.memory_base());
                        let midx = gpr_idx(ins.memory_index());
                        let scale = ins.memory_index_scale();
                        let disp = ins.memory_displacement64();
                        match (mbase, midx, scale, disp) {
                            (Some(b), None, _, 0) => {
                                let v = regs[b];
                                link_alias(&mut alias, dst, b);
                                copy_cmp_alias(&mut last_cmp, dst, b);
                                regs[dst] = v;
                                continue;
                            }
                            (Some(b), None, _, d) => {
                                clobber_alias(&mut alias, dst);
                                match regs[b] {
                                    JtVal::Const(c) => JtVal::Const(c.wrapping_add(d)),
                                    JtVal::Index { bound } => {
                                        let sd = if ins.op0_register().size() == 4 {
                                            d as i32 as i64
                                        } else {
                                            d as i64
                                        };
                                        adjust_index_bound(bound, sd)
                                    }
                                    _ => JtVal::Unknown,
                                }
                            }
                            (None, Some(ix), s @ (4 | 8), 0) => {
                                clobber_alias(&mut alias, dst);
                                let (indir, bound) = extract_idx(regs[ix]);
                                JtVal::ScaledIndex { scale: s as u8, indir, bound }
                            }
                            (Some(b), Some(ix), s @ (4 | 8), d) => {
                                clobber_alias(&mut alias, dst);
                                if let JtVal::Const(c) = regs[b] {
                                    let (indir, bound) = extract_idx(regs[ix]);
                                    JtVal::TablePtr {
                                        tbl_va: c.wrapping_add(d),
                                        stride: s as u8,
                                        indir,
                                        bound,
                                    }
                                } else {
                                    JtVal::Unknown
                                }
                            }
                            (Some(b), Some(ix), 1, 0) => {
                                clobber_alias(&mut alias, dst);
                                combine_add_target(regs[b], b, regs[ix], ix, img_base)
                            }
                            _ => {
                                clobber_alias(&mut alias, dst);
                                JtVal::Unknown
                            }
                        }
                    };
                    regs[dst] = new_val;
                    clear_cmp_reg(&mut last_cmp, dst);
                }
            }
            M::Mov => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        match ins.op_kind(1) {
                            K::Register => {
                                if let Some(src) = gpr_idx(ins.op1_register()) {
                                    let v = regs[src];
                                    regs[dst] = v;
                                    link_alias(&mut alias, dst, src);
                                    copy_cmp_alias(&mut last_cmp, dst, src);
                                } else {
                                    clobber_alias(&mut alias, dst);
                                    regs[dst] = JtVal::Unknown;
                                    clear_cmp_reg(&mut last_cmp, dst);
                                }
                            }
                            K::Memory => {
                                let v = eval_mem_load(ins, &regs, false, img_base, img_size);
                                clobber_alias(&mut alias, dst);
                                regs[dst] = v;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                            _ => {
                                clobber_alias(&mut alias, dst);
                                regs[dst] = match op_imm(ins, 1) {
                                    Some(imm) => JtVal::Const(imm),
                                    None => JtVal::Unknown,
                                };
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                        }
                    }
                }
            }
            M::Movsxd | M::Movsx => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        match ins.op_kind(1) {
                            K::Register => {
                                if let Some(src) = gpr_idx(ins.op1_register()) {
                                    let v = regs[src];
                                    regs[dst] = v;
                                    link_alias(&mut alias, dst, src);
                                    copy_cmp_alias(&mut last_cmp, dst, src);
                                } else {
                                    clobber_alias(&mut alias, dst);
                                    regs[dst] = JtVal::Unknown;
                                    clear_cmp_reg(&mut last_cmp, dst);
                                }
                            }
                            K::Memory => {
                                let v = eval_mem_load(ins, &regs, true, img_base, img_size);
                                clobber_alias(&mut alias, dst);
                                regs[dst] = v;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                            _ => {
                                clobber_alias(&mut alias, dst);
                                regs[dst] = JtVal::Unknown;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                        }
                    }
                }
            }
            M::Movzx => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        match ins.op_kind(1) {
                            K::Register => {
                                if let Some(src) = gpr_idx(ins.op1_register()) {
                                    let v = regs[src];
                                    regs[dst] = v;
                                    link_alias(&mut alias, dst, src);
                                    copy_cmp_alias(&mut last_cmp, dst, src);
                                } else {
                                    clobber_alias(&mut alias, dst);
                                    regs[dst] = JtVal::Unknown;
                                    clear_cmp_reg(&mut last_cmp, dst);
                                }
                            }
                            K::Memory => {
                                let v = eval_mem_movzx(ins, &regs, img_base, img_size);
                                clobber_alias(&mut alias, dst);
                                regs[dst] = v;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                            _ => {
                                clobber_alias(&mut alias, dst);
                                regs[dst] = JtVal::Unknown;
                                clear_cmp_reg(&mut last_cmp, dst);
                            }
                        }
                    }
                }
            }
            M::Cdqe => {
                // `movsxd rax, eax`: preserves regs[0], alias[0], and last_cmp.
            }
            M::Add => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let new_val = if ins.op_kind(1) == K::Register {
                            match gpr_idx(ins.op1_register()) {
                                Some(src) => {
                                    combine_add_target(regs[dst], dst, regs[src], src, img_base)
                                }
                                None => JtVal::Unknown,
                            }
                        } else if let Some(imm) = op_imm(ins, 1) {
                            let is32 = ins.op0_register().size() == 4;
                            match regs[dst] {
                                JtVal::Const(c) => {
                                    if is32 {
                                        JtVal::Const((c as u32).wrapping_add(imm as u32) as u64)
                                    } else {
                                        JtVal::Const(c.wrapping_add(imm))
                                    }
                                }
                                JtVal::Index { bound } => {
                                    let sd = if is32 { imm as i32 as i64 } else { imm as i64 };
                                    adjust_index_bound(bound, sd)
                                }
                                _ => JtVal::Unknown,
                            }
                        } else {
                            JtVal::Unknown
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Sub => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let is32 = ins.op0_register().size() == 4;
                        let new_val = if ins.op_kind(1) == K::Register {
                            match gpr_idx(ins.op1_register()) {
                                Some(src) if src == dst => JtVal::Const(0),
                                Some(src) => match (regs[dst], regs[src]) {
                                    (JtVal::Const(a), JtVal::Const(b)) => {
                                        if is32 {
                                            JtVal::Const((a as u32).wrapping_sub(b as u32) as u64)
                                        } else {
                                            JtVal::Const(a.wrapping_sub(b))
                                        }
                                    }
                                    (
                                        JtVal::Const(base_va),
                                        JtVal::TableOff32 {
                                            tbl_va,
                                            signed,
                                            assumed_base_reg: None,
                                            indir,
                                            bound,
                                        },
                                    ) => JtVal::Target32 {
                                        base_va,
                                        tbl_va,
                                        signed,
                                        sub_from_base: true,
                                        assumed_base: false,
                                        indir,
                                        bound,
                                    },
                                    (
                                        _,
                                        JtVal::TableOff32 {
                                            tbl_va,
                                            signed,
                                            assumed_base_reg: Some(breg),
                                            indir,
                                            bound,
                                        },
                                    ) if breg as usize == dst => JtVal::Target32 {
                                        base_va: img_base,
                                        tbl_va,
                                        signed,
                                        sub_from_base: true,
                                        assumed_base: true,
                                        indir,
                                        bound,
                                    },
                                    (JtVal::Index { bound }, JtVal::Const(imm)) => {
                                        let sd = if is32 { imm as i32 as i64 } else { imm as i64 };
                                        adjust_index_bound(bound, sd.saturating_neg())
                                    }
                                    _ => JtVal::Unknown,
                                },
                                None => JtVal::Unknown,
                            }
                        } else if let Some(imm) = op_imm(ins, 1) {
                            match regs[dst] {
                                JtVal::Const(c) => {
                                    if is32 {
                                        JtVal::Const((c as u32).wrapping_sub(imm as u32) as u64)
                                    } else {
                                        JtVal::Const(c.wrapping_sub(imm))
                                    }
                                }
                                JtVal::Index { bound } => {
                                    let sd = if is32 { imm as i32 as i64 } else { imm as i64 };
                                    adjust_index_bound(bound, sd.saturating_neg())
                                }
                                _ => JtVal::Unknown,
                            }
                        } else {
                            JtVal::Unknown
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Dec => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let is32 = ins.op0_register().size() == 4;
                        let new_val = match regs[dst] {
                            JtVal::Const(c) => {
                                if is32 {
                                    JtVal::Const((c as u32).wrapping_sub(1) as u64)
                                } else {
                                    JtVal::Const(c.wrapping_sub(1))
                                }
                            }
                            JtVal::Index { bound } => adjust_index_bound(bound, -1),
                            _ => JtVal::Unknown,
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Inc => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let is32 = ins.op0_register().size() == 4;
                        let new_val = match regs[dst] {
                            JtVal::Const(c) => {
                                if is32 {
                                    JtVal::Const((c as u32).wrapping_add(1) as u64)
                                } else {
                                    JtVal::Const(c.wrapping_add(1))
                                }
                            }
                            JtVal::Index { bound } => adjust_index_bound(bound, 1),
                            _ => JtVal::Unknown,
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Xor => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let new_val = if ins.op_kind(1) == K::Register
                            && gpr_idx(ins.op1_register()) == Some(dst)
                        {
                            JtVal::Const(0)
                        } else {
                            JtVal::Unknown
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::Shl => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        let new_val = match op_imm(ins, 1) {
                            Some(2) => {
                                let (indir, bound) = extract_idx(regs[dst]);
                                JtVal::ScaledIndex { scale: 4, indir, bound }
                            }
                            Some(3) => {
                                let (indir, bound) = extract_idx(regs[dst]);
                                JtVal::ScaledIndex { scale: 8, indir, bound }
                            }
                            _ => JtVal::Unknown,
                        };
                        clobber_alias(&mut alias, dst);
                        regs[dst] = new_val;
                    }
                }
            }
            M::And => {
                if ins.op_kind(0) == K::Register {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        clobber_alias(&mut alias, dst);
                        regs[dst] = match op_imm(ins, 1) {
                            Some(imm) if imm > 0 && imm < MAX_JT_ENTRIES as u64 => {
                                JtVal::Index { bound: Some(imm as usize + 1) }
                            }
                            _ => JtVal::Unknown,
                        };
                    }
                }
            }
            M::Test
            | M::Bt
            | M::Nop
            | M::Push
            | M::Jmp
            | M::Ret
            | M::Int3
            | M::Ud2
            | M::Pause
            | M::Mfence
            | M::Lfence
            | M::Sfence => {}
            M::Call => {
                // Win64 ABI volatile registers are clobbered; callee-saved regs stay intact.
                for r in [0usize, 1, 2, 8, 9, 10, 11] {
                    clobber_alias(&mut alias, r);
                    regs[r] = JtVal::Unknown;
                    clear_cmp_reg(&mut last_cmp, r);
                }
            }
            M::Xchg => {
                for op in [0u32, 1] {
                    if ins.op_kind(op) == K::Register {
                        if let Some(r) = gpr_idx(ins.op_register(op)) {
                            clobber_alias(&mut alias, r);
                            regs[r] = JtVal::Unknown;
                            clear_cmp_reg(&mut last_cmp, r);
                        }
                    }
                }
            }
            M::Mul | M::Div | M::Idiv | M::Cdq | M::Cqo => {
                for r in [0usize, 2] {
                    clobber_alias(&mut alias, r);
                    regs[r] = JtVal::Unknown;
                    clear_cmp_reg(&mut last_cmp, r);
                }
            }
            _ => {
                if ins.flow_control() != F::ConditionalBranch
                    && ins.op_count() > 0
                    && ins.op_kind(0) == K::Register
                {
                    if let Some(dst) = gpr_idx(ins.op0_register()) {
                        clobber_alias(&mut alias, dst);
                        regs[dst] = JtVal::Unknown;
                        clear_cmp_reg(&mut last_cmp, dst);
                    }
                }
            }
        }
    }

    let mut step = BlockStep::default();
    match fc {
        F::Next | F::IndirectCall => {
            step.edges.push((next, EdgeKind::Intra, pack_hint(&regs)));
        }
        F::Call => {
            step.edges.push((next, EdgeKind::Intra, pack_hint(&regs)));
            if tgt != 0 {
                step.edges.push((tgt, EdgeKind::DirectCall, None));
            }
        }
        F::ConditionalBranch => {
            let mut taken_regs = regs;
            let mut fall_regs = regs;
            if let Some((mask, imm, rev)) = last_cmp {
                let bound_le = (imm < MAX_JT_ENTRIES as u64).then(|| imm as usize + 1);
                let bound_lt =
                    (imm > 0 && imm <= MAX_JT_ENTRIES as u64).then(|| imm as usize);
                let cc = if !rev {
                    last.condition_code()
                } else {
                    // `cmp const, reg` swaps the direction of ordered comparisons.
                    match last.condition_code() {
                        CC::a => CC::b,
                        CC::ae => CC::be,
                        CC::b => CC::a,
                        CC::be => CC::ae,
                        CC::g => CC::l,
                        CC::ge => CC::le,
                        CC::l => CC::g,
                        CC::le => CC::ge,
                        other => other,
                    }
                };
                let (taken_b, fall_b) = match cc {
                    CC::a | CC::g => (None, bound_le),
                    CC::ae | CC::ge => (None, bound_lt),
                    CC::be | CC::le => (bound_le, None),
                    CC::b | CC::l => (bound_lt, None),
                    _ => (None, None),
                };
                if let Some(b) = taken_b {
                    for (r, v) in taken_regs.iter_mut().enumerate() {
                        if mask & (1u16 << r) != 0 {
                            *v = JtVal::Index { bound: Some(b) };
                        }
                    }
                }
                if let Some(b) = fall_b {
                    for (r, v) in fall_regs.iter_mut().enumerate() {
                        if mask & (1u16 << r) != 0 {
                            *v = JtVal::Index { bound: Some(b) };
                        }
                    }
                }
            }
            step.edges.push((tgt, EdgeKind::Intra, pack_hint(&taken_regs)));
            step.edges.push((next, EdgeKind::Intra, pack_hint(&fall_regs)));
        }
        F::UnconditionalBranch if tgt != 0 => {
            step.edges.push((tgt, EdgeKind::TailBranch, pack_hint(&regs)));
        }
        F::IndirectBranch if last.mnemonic() == M::Jmp => {
            step.ends_indirect_jmp = true;
            let jt_kind = match last.op_kind(0) {
                K::Register => gpr_idx(last.op0_register()).map(|r| regs[r]).unwrap_or(JtVal::Unknown),
                K::Memory => eval_mem_load(last, &regs, false, img_base, img_size),
                _ => JtVal::Unknown,
            };
            if matches!(jt_kind, JtVal::Target32 { .. } | JtVal::Target64 { .. }) {
                let cur_func = fmap.root_of(ip.wrapping_sub(img_base) as u32);
                let targets = resolve_jt_targets(jt_kind, base, size, text, fmap, cur_func);
                step.jt_targets = targets.len();
                for t in targets {
                    step.edges.push((t, EdgeKind::JumpTable, None));
                }
            }
        }
        _ => {}
    }
    push_lea_leaf_edges(insns, img_base, img_size, text, fmap, &mut step.edges);
    step
}

/// PE32+ data directory `idx` of the image at `[base, base + size)`: `(rva, size)`, None if
/// absent or out of the image.
unsafe fn data_dir(base: usize, size: usize, idx: usize) -> Option<(u32, u32)> {
    if base == 0 || size < 0x400 {
        return None;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return None;
    }
    let nt = rd32(0x3C) as usize;
    let opt = nt + 24;
    if opt + 112 + 16 * 8 > size || rd32(nt) != 0x0000_4550 || rd16(opt) != 0x20B {
        return None;
    }
    if rd32(opt + 108) as usize <= idx {
        return None;
    }
    let (rva, len) = (rd32(opt + 112 + idx * 8), rd32(opt + 112 + idx * 8 + 4));
    (len != 0 && rva as u64 + len as u64 <= size as u64).then_some((rva, len))
}

/// `.pdata` function map of a mapped PE32+ image: every RUNTIME_FUNCTION range with the
/// begin RVA of its chain root. Not every RUNTIME_FUNCTION is a function start: entries
/// with `UNW_FLAG_CHAININFO` (fragments the compiler split off a function: cold parts,
/// shrink-wrapped regions) and `RUNTIME_FUNCTION_INDIRECT` entries are merged into the
/// function they continue, so all fragments of one root count as one function even when
/// they are not contiguous.
struct FuncMap {
    /// `(begin, end, root)` RVAs, sorted by `begin`.
    ranges: Vec<(u32, u32, u32)>,
    /// Entries merged into another root.
    chained: usize,
    /// Entries whose chain is unreadable or cyclic (kept as their own root).
    broken: usize,
}

impl FuncMap {
    unsafe fn read(base: usize, size: usize) -> FuncMap {
        const UNW_FLAG_CHAININFO: u8 = 4;
        let mut fm = FuncMap { ranges: Vec::new(), chained: 0, broken: 0 };
        let Some((rva, len)) = data_dir(base, size, 3) else { return fm };
        let fits = |off: u64, n: u64| off.checked_add(n).is_some_and(|e| e <= size as u64);
        let rd8 = |off: u64| *((base as u64 + off) as *const u8);
        let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
        let entry = |off: u64| (rd32(off), rd32(off + 4), rd32(off + 8));
        for i in 0..len as u64 / 12 {
            let (b, e, mut ui) = entry(rva as u64 + i * 12);
            if b >= e || !fits(b as u64, (e - b) as u64) {
                continue;
            }
            let (mut root, mut ok, mut hops) = (b, true, 0);
            loop {
                hops += 1;
                if hops > 32 {
                    ok = false;
                    break;
                }
                let next = if ui & 1 != 0 {
                    // RUNTIME_FUNCTION_INDIRECT: UnwindData is the RVA of the real entry.
                    ui as u64 & !1
                } else {
                    let u = ui as u64;
                    if !fits(u, 4) {
                        ok = false;
                        break;
                    }
                    if (rd8(u) >> 3) & UNW_FLAG_CHAININFO == 0 {
                        break;
                    }
                    // The parent RUNTIME_FUNCTION follows the (even-padded) unwind codes.
                    u + 4 + ((rd8(u + 2) as u64 + 1) & !1) * 2
                };
                if !fits(next, 12) {
                    ok = false;
                    break;
                }
                let (b2, _, u2) = entry(next);
                root = b2;
                ui = u2;
            }
            if !ok {
                fm.broken += 1;
                root = b;
            } else if root != b {
                fm.chained += 1;
            }
            fm.ranges.push((b, e, root));
        }
        fm.ranges.sort_unstable();
        fm
    }

    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// Root begin RVA of the function containing `rva` (None: no `.pdata` entry, e.g. a
    /// leaf function).
    fn root_of(&self, rva: u32) -> Option<u32> {
        let i = self.ranges.partition_point(|r| r.0 <= rva);
        let &(_, e, root) = self.ranges.get(i.checked_sub(1)?)?;
        (rva < e).then_some(root)
    }

    /// Distinct roots (functions).
    fn functions(&self) -> usize {
        self.ranges.iter().filter(|r| r.0 == r.2).count()
    }
}

/// Closure statistics of [`discover`].
#[derive(Default)]
struct Closure {
    /// Static successors left out because they lie in another `.pdata` function.
    cross: usize,
    /// Indirect switch / jump tables discovered during static CFG closure.
    jt_tables: usize,
    /// Unique switch / jump-table case targets added to the static closure.
    jt_targets: usize,
    /// Direct `.pdata`-less leaf callees (calls / tail-jumps) discovered during closure.
    leaf_callees: usize,
    /// `.pdata`-less leaf function entries discovered via `LEA reg, [rip + disp32]`.
    lea_leaves: usize,
    functions: usize,
    chained: usize,
    broken: usize,
}

/// Block set for a module: the seed block IPs plus their static successors (including
/// RIP-relative switch / jump-table targets and `.pdata`-less leaf callees) inside the
/// module's executable sections, up to `depth` steps away. With a `.pdata` map (unless
/// `PORT_AOT_PDATA=0` / `AW2_DLL_AOT_PDATA=0`) intra-function successors stay in the same
/// function (chain roots merged), while direct calls / tail-jumps into `.pdata`-less leaf
/// functions are followed so leaf helpers aren't left out of AOT caches.
unsafe fn discover(bc: *mut BlockCache, base: usize, size: usize, prof: &[u32], depth: u32) -> (Vec<u64>, Closure) {
    discover_inner(Some(bc), None, base, size, prof, depth, 1)
}

/// Parallel-capable version of [`discover`]: expands large BFS frontiers across up to
/// `threads` worker threads reusing `pool`, merging results in deterministic order.
unsafe fn discover_with_threads(
    pool: &mut WorkerCaches,
    base: usize,
    size: usize,
    prof: &[u32],
    depth: u32,
    threads: usize,
) -> (Vec<u64>, Closure) {
    discover_inner(None, Some(pool), base, size, prof, depth, threads)
}

unsafe fn discover_inner(
    bc: Option<*mut BlockCache>,
    mut pool: Option<&mut WorkerCaches>,
    base: usize,
    size: usize,
    prof: &[u32],
    depth: u32,
    threads: usize,
) -> (Vec<u64>, Closure) {
    let text = exec_sections(base, size);
    let in_text = |ip: u64| text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip));
    let fmap = if env_u32_any(&["PORT_AOT_PDATA", "AW2_DLL_AOT_PDATA"], 1) != 0 {
        FuncMap::read(base, size)
    } else {
        FuncMap { ranges: Vec::new(), chained: 0, broken: 0 }
    };
    let jt_on = jumptables_enabled();
    let func = |ip: u64| fmap.root_of(ip.wrapping_sub(base as u64) as u32);
    let mut st = Closure {
        functions: fmap.functions(),
        chained: fmap.chained,
        broken: fmap.broken,
        ..Closure::default()
    };
    let mut seen: HashSet<u64> = HashSet::new();
    // Best register hint each analyzed block has been seen with (populated lazily when hints exist).
    let mut seen_hints: HashMap<u64, JtRegs> = HashMap::new();
    // Blocks known to end in an indirect `jmp` (eligible for hint-change re-analysis).
    let mut indirect_blocks: HashSet<u64> = HashSet::new();
    // Indirect-branch blocks that already counted toward `st.jt_tables` (prevents double-count).
    let mut jt_blocks: HashSet<u64> = HashSet::new();
    // Per-block count of hint-upgrade re-analyses (capped at `MAX_JT_REANALYZE`).
    let mut reanalyzed: HashMap<u64, u8> = HashMap::new();
    // `(ip, incoming_hint, is_reanalysis)`.
    let mut frontier: Vec<(u64, Option<Box<JtRegs>>, bool)> = prof
        .iter()
        .map(|&r| (base as u64 + r as u64, None, false))
        .filter(|&(ip, _, _)| in_text(ip))
        .collect();

    for d in 0..=depth {
        let mut batch: Vec<(u64, Option<Box<JtRegs>>)> = Vec::new();
        for (ip, hint, is_re) in frontier {
            if is_re {
                if d == depth {
                    continue;
                }
                batch.push((ip, hint));
            } else {
                if !seen.insert(ip) || d == depth {
                    continue;
                }
                if let Some(ref h) = hint {
                    seen_hints.insert(ip, **h);
                }
                batch.push((ip, hint));
            }
        }
        if batch.is_empty() {
            break;
        }

        let steps: Vec<BlockStep> = if threads > 1
            && batch.len() >= PARALLEL_DISCOVER_MIN_BLOCKS
            && pool.is_some()
        {
            let workers = threads.min(batch.len());
            let chunk_len = (batch.len() / (workers * 8)).clamp(1, 256);
            let chunks: Vec<&[(u64, Option<Box<JtRegs>>)]> = batch.chunks(chunk_len).collect();
            let n_chunks = chunks.len();
            let next_chunk = AtomicUsize::new(0);
            let worker_bcs = pool.as_deref_mut().unwrap().ensure(workers);
            let mut all_chunks: Vec<(usize, Vec<BlockStep>)> = Vec::with_capacity(n_chunks);
            std::thread::scope(|scope| {
                let mut handles = Vec::with_capacity(workers);
                for (wi, local_bc) in worker_bcs.iter_mut().enumerate() {
                    let chunks_ref = &chunks;
                    let next_ref = &next_chunk;
                    let text_ref = &text;
                    let fmap_ref = &fmap;
                    let h = std::thread::Builder::new()
                        .name(format!("port-aot-d{wi}"))
                        .stack_size(PARALLEL_WORKER_STACK)
                        .spawn_scoped(scope, move || {
                            let bcp: *mut BlockCache = &mut **local_bc;
                            let mut done: Vec<(usize, Vec<BlockStep>)> = Vec::new();
                            loop {
                                let ci = next_ref.fetch_add(1, Ordering::Relaxed);
                                if ci >= n_chunks {
                                    break;
                                }
                                let slice = chunks_ref[ci];
                                let mut out = Vec::with_capacity(slice.len());
                                for (ip, hint) in slice {
                                    out.push(analyze_block(
                                        bcp,
                                        *ip,
                                        hint.as_deref(),
                                        base,
                                        size,
                                        text_ref,
                                        fmap_ref,
                                        jt_on,
                                    ));
                                }
                                done.push((ci, out));
                            }
                            done
                        })
                        .expect("spawn parallel CFG discovery worker");
                    handles.push(h);
                }
                for h in handles {
                    all_chunks.extend(h.join().expect("parallel CFG discovery worker panicked"));
                }
            });
            all_chunks.sort_unstable_by_key(|(ci, _)| *ci);
            let mut flat = Vec::with_capacity(batch.len());
            for (_, chunk_steps) in all_chunks {
                flat.extend(chunk_steps);
            }
            flat
        } else {
            let bcp = match (bc, pool.as_deref_mut()) {
                (Some(p), _) => p,
                (None, Some(p)) => p.get_one(),
                (None, None) => unreachable!("discover_inner requires bc or pool"),
            };
            batch
                .iter()
                .map(|(ip, hint)| {
                    analyze_block(bcp, *ip, hint.as_deref(), base, size, &text, &fmap, jt_on)
                })
                .collect()
        };

        let mut next: Vec<(u64, Option<Box<JtRegs>>, bool)> = Vec::new();
        let mut next_pos: HashMap<u64, usize> = HashMap::new();
        for ((ip, _), step) in batch.into_iter().zip(steps) {
            let f = func(ip);
            if step.ends_indirect_jmp {
                indirect_blocks.insert(ip);
            }
            if step.jt_targets > 0 && jt_blocks.insert(ip) {
                st.jt_tables += 1;
            }
            for (s, kind, mut out_hint) in step.edges {
                if !in_text(s) {
                    continue;
                }
                let mut is_leaf_entry = false;
                if !fmap.is_empty() {
                    let sf = func(s);
                    match kind {
                        EdgeKind::Intra | EdgeKind::JumpTable => {
                            if sf != f {
                                st.cross += 1;
                                continue;
                            }
                        }
                        EdgeKind::DirectCall => {
                            if sf.is_some() && sf != f {
                                st.cross += 1;
                                continue;
                            }
                            if sf.is_none() && f.is_some() {
                                is_leaf_entry = true;
                            }
                            out_hint = None;
                        }
                        EdgeKind::TailBranch => {
                            if sf != f {
                                if sf.is_none() && valid_code_target(s, &text) {
                                    // Direct tail-jump from a `.pdata` function into a
                                    // `.pdata`-less leaf helper: follow cleanly without leaking
                                    // caller switch hints.
                                    is_leaf_entry = true;
                                    out_hint = None;
                                } else {
                                    st.cross += 1;
                                    continue;
                                }
                            }
                        }
                        EdgeKind::LeaLeaf => {
                            if sf.is_some() {
                                continue;
                            }
                            out_hint = None;
                        }
                    }
                }

                if seen.contains(&s) {
                    // Re-analyze an already-seen block if it receives strictly richer register
                    // hints (e.g. a switch dispatch block reached first without base/bound
                    // hints, or an intermediate block on the way to one).
                    if jt_on {
                        if let Some(ref incoming) = out_hint {
                            let count = reanalyzed.get(&s).copied().unwrap_or(0);
                            if count < MAX_JT_REANALYZE {
                                let prev = seen_hints.entry(s).or_insert([JtVal::Unknown; 16]);
                                let upgraded = merge_jt_regs(prev, incoming);
                                let indirect_alt = !upgraded
                                    && indirect_blocks.contains(&s)
                                    && *prev != **incoming;
                                if upgraded || indirect_alt {
                                    let re_hint = if indirect_alt { **incoming } else { *prev };
                                    if let Some(&idx) = next_pos.get(&s) {
                                        if let Some(ref mut cur) = next[idx].1 {
                                            merge_jt_regs(cur, &re_hint);
                                        } else {
                                            next[idx].1 = Some(Box::new(re_hint));
                                        }
                                    } else {
                                        reanalyzed.insert(s, count + 1);
                                        next_pos.insert(s, next.len());
                                        next.push((s, Some(Box::new(re_hint)), true));
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }

                if let Some(&idx) = next_pos.get(&s) {
                    if let Some(ref incoming) = out_hint {
                        if let Some(ref mut cur) = next[idx].1 {
                            merge_jt_regs(cur, incoming);
                        } else {
                            next[idx].1 = out_hint;
                        }
                    }
                    continue;
                }
                if kind == EdgeKind::JumpTable {
                    st.jt_targets += 1;
                }
                if is_leaf_entry {
                    st.leaf_callees += 1;
                }
                if kind == EdgeKind::LeaLeaf {
                    st.lea_leaves += 1;
                }
                next_pos.insert(s, next.len());
                next.push((s, out_hint, false));
            }
        }
        frontier = next;
    }
    let mut v: Vec<u64> = seen.into_iter().collect();
    v.sort_unstable();
    (v, st)
}

/// PE32+ `AddressOfEntryPoint` RVA of the image at `[base, base + size)`, if present and non-zero.
unsafe fn entry_point_rva(base: usize, size: usize) -> Option<u32> {
    if base == 0 || size < 0x400 {
        return None;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return None;
    }
    let nt = rd32(0x3C) as usize;
    let opt = nt.checked_add(24)?;
    if opt.checked_add(20)? > size || rd32(nt) != 0x0000_4550 || rd16(opt) != 0x20B {
        return None;
    }
    let entry = rd32(opt + 16);
    (entry != 0 && (entry as usize) < size).then_some(entry)
}

/// Exported function RVAs from the PE32+ Export Directory (index 0) at `[base, base + size)`,
/// skipping empty slots and forwarder strings.
unsafe fn export_function_rvas(base: usize, size: usize) -> Vec<u32> {
    let mut out = Vec::new();
    let Some((dir_rva, dir_len)) = data_dir(base, size, 0) else {
        return out;
    };
    if dir_len < 40 {
        return out;
    }
    let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
    let n_funcs = rd32(dir_rva as u64 + 20) as u64;
    let funcs_rva = rd32(dir_rva as u64 + 28) as u64;
    let Some(table_bytes) = n_funcs.checked_mul(4) else {
        return out;
    };
    let Some(table_end) = funcs_rva.checked_add(table_bytes) else {
        return out;
    };
    if table_end > size as u64 {
        return out;
    }
    let dir_start = dir_rva as u64;
    let dir_end = dir_start + dir_len as u64;
    for i in 0..n_funcs {
        let rva = rd32(funcs_rva + i * 4);
        if rva == 0 {
            continue;
        }
        let rva64 = rva as u64;
        // Forwarder exports point to a "DLL.Symbol" ASCII string inside the export directory.
        if rva64 >= dir_start && rva64 < dir_end {
            continue;
        }
        if rva64 < size as u64 {
            out.push(rva);
        }
    }
    out
}

/// OptionalHeader `ImageBase` of a PE32+ image mapped at `[base, base + size)`.
unsafe fn opt_image_base(base: usize, size: usize) -> Option<u64> {
    if base == 0 || size < 0x400 {
        return None;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return None;
    }
    let nt = rd32(0x3C) as usize;
    let opt = nt.checked_add(24)?;
    if opt.checked_add(32)? > size || rd32(nt) != 0x0000_4550 || rd16(opt) != 0x20B {
        return None;
    }
    Some(core::ptr::read_unaligned((base + opt + 24) as *const u64))
}

/// Convert a VA inside a mapped PE32+ image (`[base, base + size)`, or fallback to
/// `OptionalHeader.ImageBase` if unrelocated) into an in-bounds RVA.
#[inline]
fn pe_va_to_rva(base: usize, size: usize, opt_base: Option<u64>, va: u64) -> Option<u32> {
    let b = base as u64;
    let sz = size as u64;
    if va >= b && va - b < sz {
        return u32::try_from(va - b).ok();
    }
    if let Some(ob) = opt_base {
        if ob != 0 && ob != b && va >= ob && va - ob < sz {
            return u32::try_from(va - ob).ok();
        }
    }
    None
}

/// TLS callback RVAs from `IMAGE_TLS_DIRECTORY64::AddressOfCallBacks` (DataDirectory[9]).
unsafe fn tls_callback_rvas(base: usize, size: usize) -> Vec<u32> {
    const MAX_TLS_CALLBACKS: usize = 1024;
    let mut out = Vec::new();
    let Some((dir_rva, _dir_len)) = data_dir(base, size, 9) else {
        return out;
    };
    if (dir_rva as u64).saturating_add(32) > size as u64 {
        return out;
    }
    let rd64 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u64);
    let opt_base = opt_image_base(base, size);
    let cb_table_va = rd64(dir_rva as u64 + 24);
    if cb_table_va == 0 {
        return out;
    }
    let Some(table_rva) = pe_va_to_rva(base, size, opt_base, cb_table_va) else {
        return out;
    };
    for i in 0..MAX_TLS_CALLBACKS {
        let off = (table_rva as u64).saturating_add((i as u64) * 8);
        if off.saturating_add(8) > size as u64 {
            break;
        }
        let cb_va = rd64(off);
        if cb_va == 0 {
            break;
        }
        let Some(rva) = pe_va_to_rva(base, size, opt_base, cb_va) else {
            break;
        };
        out.push(rva);
    }
    out
}

/// Control Flow Guard (CFG) valid indirect-call target RVAs from
/// `IMAGE_LOAD_CONFIG_DIRECTORY64` (DataDirectory[10]): `GuardCFFunctionTable` (respecting
/// `IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK` extra bytes per entry) and
/// `GuardLongJumpTargetTable` where present.
unsafe fn load_config_cfg_rvas(base: usize, size: usize) -> Vec<u32> {
    const GUARD_CF_FUNCTION_TABLE_PRESENT: u32 = 0x0000_0400;
    const GUARD_CF_LONGJUMP_TABLE_PRESENT: u32 = 0x0001_0000;
    const MAX_CFG_ENTRIES: usize = 4_000_000;
    let mut out = Vec::new();
    let Some((dir_rva, _dir_len)) = data_dir(base, size, 10) else {
        return out;
    };
    if (dir_rva as u64).saturating_add(4) > size as u64 {
        return out;
    }
    let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
    let rd64 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u64);
    let avail = (size as u64).saturating_sub(dir_rva as u64);
    let lc_size = (rd32(dir_rva as u64) as u64).min(avail);
    if lc_size < 0x94 {
        return out;
    }
    let opt_base = opt_image_base(base, size);
    let guard_flags = rd32(dir_rva as u64 + 0x90);

    if (guard_flags & GUARD_CF_FUNCTION_TABLE_PRESENT) != 0 {
        let tbl_va = rd64(dir_rva as u64 + 0x80);
        let count = rd64(dir_rva as u64 + 0x88);
        let extra = ((guard_flags & 0xF000_0000) >> 28) as u64;
        let stride = 4u64 + extra;
        if tbl_va != 0 && count != 0 {
            if let Some(tbl_rva) = pe_va_to_rva(base, size, opt_base, tbl_va) {
                let max_fit = (size as u64).saturating_sub(tbl_rva as u64) / stride;
                let n = (count.min(max_fit) as usize).min(MAX_CFG_ENTRIES);
                out.reserve(n);
                for i in 0..n {
                    let off = tbl_rva as u64 + (i as u64) * stride;
                    let rva = rd32(off);
                    if rva != 0 && (rva as u64) < size as u64 {
                        out.push(rva);
                    }
                }
            }
        }
    }

    if lc_size >= 0xC0 && (guard_flags & GUARD_CF_LONGJUMP_TABLE_PRESENT) != 0 {
        let tbl_va = rd64(dir_rva as u64 + 0xB0);
        let count = rd64(dir_rva as u64 + 0xB8);
        if tbl_va != 0 && count != 0 {
            if let Some(tbl_rva) = pe_va_to_rva(base, size, opt_base, tbl_va) {
                let max_fit = (size as u64).saturating_sub(tbl_rva as u64) / 4;
                let n = (count.min(max_fit) as usize).min(MAX_CFG_ENTRIES);
                for i in 0..n {
                    let off = tbl_rva as u64 + (i as u64) * 4;
                    let rva = rd32(off);
                    if rva != 0 && (rva as u64) < size as u64 {
                        out.push(rva);
                    }
                }
            }
        }
    }

    out
}

/// Executable-section target RVAs referenced by PE Base Relocations (`DataDirectory[5]`,
/// `IMAGE_REL_BASED_DIR64`) — typically C++ vtables, function-pointer tables, and global
/// callback tables in `.rdata`/`.data` pointing to `.pdata`-less leaf methods and adjustor
/// thunks.
///
/// When `DataDirectory[5]` is absent/empty (stripped `.reloc`), falls back to a conservative
/// 8-byte-aligned scan of non-executable initialized data sections for pointers into `text`
/// that land on clean function boundaries.
unsafe fn base_reloc_code_rvas(base: usize, size: usize, text: &[(usize, usize)]) -> Vec<u32> {
    const IMAGE_REL_BASED_DIR64: u16 = 10;
    const MAX_RELOC_ENTRIES: usize = 4_000_000;

    let mut out = Vec::new();
    if base == 0 || size < 0x400 || text.is_empty() {
        return out;
    }
    let opt_base = opt_image_base(base, size);

    if let Some((dir_rva, dir_len)) = data_dir(base, size, 5).filter(|&(r, _)| r != 0) {
        let Some(dir_end) = (dir_rva as u64).checked_add(dir_len as u64) else {
            return out;
        };
        if dir_end > size as u64 {
            return out;
        }
        let rd16 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u16);
        let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
        let rd64 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u64);

        let mut off = dir_rva as u64;
        while off.checked_add(8).is_some_and(|e| e <= dir_end) {
            let page_rva = rd32(off);
            let block_size = rd32(off + 4);
            if block_size < 8 {
                break;
            }
            let Some(next_off) = off.checked_add(block_size as u64) else {
                break;
            };
            if next_off > dir_end {
                break;
            }
            let n_entries = ((block_size - 8) / 2) as u64;
            for i in 0..n_entries {
                let entry = rd16(off + 8 + i * 2);
                let ty = entry >> 12;
                if ty != IMAGE_REL_BASED_DIR64 {
                    continue;
                }
                let rel_off = (entry & 0x0FFF) as u32;
                let Some(slot_rva) = page_rva.checked_add(rel_off) else {
                    continue;
                };
                if (slot_rva as u64) + 8 > size as u64 {
                    continue;
                }
                let va = rd64(slot_rva as u64);
                let Some(target_rva) = pe_va_to_rva(base, size, opt_base, va) else {
                    continue;
                };
                let target_ip = base as u64 + target_rva as u64;
                if is_plausible_code_entry(target_ip, text) {
                    out.push(target_rva);
                    if out.len() >= MAX_RELOC_ENTRIES {
                        return out;
                    }
                }
            }
            off = next_off;
        }
        return out;
    }

    // Fallback when `DataDirectory[5]` is absent/empty (stripped `.reloc`): scan 8-byte-aligned
    // slots in non-executable initialized data sections for pointers into `text` landing on
    // clean function boundaries. Require a non-zero `OptionalHeader.ImageBase` so synthetic
    // images with zeroed headers cannot turn small integers into bogus RVAs.
    if !opt_base.is_some_and(|ob| ob != 0) {
        return out;
    }
    let rd64 = |addr: usize| core::ptr::read_unaligned(addr as *const u64);
    for (sec_s, sec_e) in data_sections(base, size) {
        let Some(rel_s) = (sec_s - base).checked_add(7).map(|x| x & !7) else {
            continue;
        };
        let rel_e = sec_e - base;
        let mut rel = rel_s;
        while rel.checked_add(8).is_some_and(|e| e <= rel_e) {
            let va = rd64(base + rel);
            if let Some(target_rva) = pe_va_to_rva(base, size, opt_base, va) {
                let target_ip = base as u64 + target_rva as u64;
                if is_clean_code_entry(target_ip, text) {
                    out.push(target_rva);
                    if out.len() >= MAX_RELOC_ENTRIES {
                        return out;
                    }
                }
            }
            rel += 8;
        }
    }
    out
}

/// Discover profile-free function root RVAs directly from a mapped PE32+ image:
/// 1. Every `.pdata` (`RUNTIME_FUNCTION`) entry's `begin_rva`,
/// 2. `AddressOfEntryPoint`,
/// 3. Export Directory function RVAs (excluding forwarders and non-code data exports),
/// 4. TLS Directory callback RVAs (`AddressOfCallBacks`),
/// 5. Load Config Control Flow Guard (`GuardCFFunctionTable` / `GuardLongJumpTargetTable`),
/// 6. PE Base Relocation (`DataDirectory[5]`, `IMAGE_REL_BASED_DIR64`) executable-section
///    targets (vtables / function-pointer tables; with a stripped-`.reloc` fallback scan),
///    when `PORT_AOT_RELOC_ROOTS` is enabled (default 1).
///
/// Filtered to executable sections, sorted deterministically, deduplicated, and capped at
/// `max_roots` when `max_roots > 0`.
unsafe fn discover_pe_roots(base: usize, size: usize, max_roots: usize) -> Vec<u32> {
    let text = exec_sections(base, size);
    if text.is_empty() {
        return Vec::new();
    }
    let in_text = |rva: u32| -> bool {
        let ip = base as u64 + rva as u64;
        text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip))
    };
    let mut roots = Vec::new();
    let fmap = FuncMap::read(base, size);
    for &(begin, _, _) in &fmap.ranges {
        if in_text(begin) {
            roots.push(begin);
        }
    }
    if let Some(ep) = entry_point_rva(base, size) {
        if in_text(ep) {
            roots.push(ep);
        }
    }
    for rva in export_function_rvas(base, size)
        .into_iter()
        .chain(tls_callback_rvas(base, size))
        .chain(load_config_cfg_rvas(base, size))
    {
        if in_text(rva) {
            roots.push(rva);
        }
    }
    if reloc_roots_enabled() {
        roots.extend(base_reloc_code_rvas(base, size, &text));
    }
    roots.sort_unstable();
    roots.dedup();
    if max_roots > 0 && roots.len() > max_roots {
        roots.truncate(max_roots);
    }
    roots
}

/// Outcome of profile-free offline `.aot` block-cache precompilation for one mapped PE module.
#[derive(Debug, Clone)]
pub struct AotPrewarmReport {
    pub module: String,
    pub roots: usize,
    pub blocks: usize,
    pub code_bytes: usize,
    pub cache_path: Option<PathBuf>,
    pub skipped_reason: Option<String>,
}

/// Profile-free offline `.aot` block-cache precompiler for every mapped PE module registered
/// via [`module_mapped`]. Discovers function roots directly from each mapped PE image
/// (`.pdata` `RUNTIME_FUNCTION` entries, `AddressOfEntryPoint`, and Export Directory code
/// RVAs), expands each root with the static CFG closure (`PORT_AOT_CLOSURE` /
/// `AW2_DLL_AOT_CLOSURE`, default 2), translates via `jit.rs`'s AOT emitter, and writes the
/// standard `<cache>/<module>-<key>.aot` file loaded by [`activate`].
pub fn aot_prewarm_all_mapped(max_roots_per_module: usize) -> Vec<AotPrewarmReport> {
    struct PrewarmGuard(bool);
    impl Drop for PrewarmGuard {
        fn drop(&mut self) {
            PREWARMING.store(self.0, Ordering::Release);
        }
    }
    let _guard = PrewarmGuard(PREWARMING.swap(true, Ordering::AcqRel));

    let mapped: Vec<(String, usize, usize)> = match MAPPED.lock() {
        Ok(v) => v.iter().map(|m| (m.name.clone(), m.base, m.size)).collect(),
        Err(_) => return Vec::new(),
    };
    if mapped.is_empty() {
        return Vec::new();
    }

    let depth = aot_closure_depth();
    let threads = prewarm_threads();
    let mut pool = WorkerCaches::new();
    let mut reports = Vec::with_capacity(mapped.len());

    for (name, base, size) in mapped {
        if crate::exe::aw2_registry_active() && base == crate::exe::base() {
            log::info!(
                "emu aot: prewarm {name}: skipped (native replacement registry active for exe)"
            );
            reports.push(AotPrewarmReport {
                module: name,
                roots: 0,
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("native replacement registry active for exe".into()),
            });
            continue;
        }

        let roots = unsafe { discover_pe_roots(base, size, max_roots_per_module) };
        if roots.is_empty() {
            log::info!("emu aot: prewarm {name}: no executable code roots found; skipped");
            reports.push(AotPrewarmReport {
                module: name,
                roots: 0,
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("no executable code roots found in PE".into()),
            });
            continue;
        }

        let key = unsafe { module_key(base, size) };
        let t0 = std::time::Instant::now();
        let (ips, cl) = unsafe { discover_with_threads(&mut pool, base, size, &roots, depth, threads) };
        if ips.is_empty() {
            reports.push(AotPrewarmReport {
                module: name,
                roots: roots.len(),
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("no translatable blocks discovered".into()),
            });
            continue;
        }
        let build_threads = if ips.len() >= PARALLEL_MIN_BLOCKS { threads } else { 1 };
        let Some(img) = (unsafe { build_image_with_threads(&mut pool, &ips, BUILD_VBASE, build_threads) }) else {
            log::error!("emu aot: prewarm {name}: shared stubs don't fit; not built");
            reports.push(AotPrewarmReport {
                module: name,
                roots: roots.len(),
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("shared stubs did not fit".into()),
            });
            continue;
        };
        if img.blocks.is_empty() {
            reports.push(AotPrewarmReport {
                module: name,
                roots: roots.len(),
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("0 blocks translated".into()),
            });
            continue;
        }
        let path = cache_file(&name, &key);
        match write_atomic(&path, &encode(&name, &key, &img, 0)) {
            Ok(()) => {
                let evicted = delete_other_keys(&cache_dir(), &name, &path);
                log::info!(
                    "emu aot: prewarm {name}: built {} blocks ({} roots, {} candidates; closure depth {depth}: {} jump tables / {} targets, {} leaf callees, {} lea leaves, {} cross-function successors skipped, .pdata {} functions, {} chained fragments merged, {} broken; {} bank(s), {build_threads} thread(s)), {} KiB code, {} links, {} relocs, {} assumptions in {:.2}s -> {} ({evicted} cache(s) of other keys removed)",
                    img.blocks.len(),
                    roots.len(),
                    ips.len(),
                    cl.jt_tables,
                    cl.jt_targets,
                    cl.leaf_callees,
                    cl.lea_leaves,
                    cl.cross,
                    cl.functions,
                    cl.chained,
                    cl.broken,
                    img.stub_banks().len(),
                    img.code_bytes() >> 10,
                    img.links.len(),
                    img.relocs.len(),
                    img.assume.len(),
                    t0.elapsed().as_secs_f64(),
                    path.display(),
                );
                reports.push(AotPrewarmReport {
                    module: name,
                    roots: roots.len(),
                    blocks: img.blocks.len(),
                    code_bytes: img.code_bytes(),
                    cache_path: Some(path),
                    skipped_reason: None,
                });
            }
            Err(e) => {
                log::error!("emu aot: prewarm {name}: writing {}: {e}", path.display());
                reports.push(AotPrewarmReport {
                    module: name,
                    roots: roots.len(),
                    blocks: img.blocks.len(),
                    code_bytes: img.code_bytes(),
                    cache_path: None,
                    skipped_reason: Some(format!("write failed: {e}")),
                });
            }
        }
    }

    reports
}

/// Profile RVAs of a module (most frequent first), or empty.
fn read_profile(name: &str, image_hash: &[u64; 2]) -> Vec<u32> {
    let mut map = read_tsv(&profile_file(name, image_hash));
    if map.is_empty() {
        for fb_dir in fallback_cache_dirs() {
            let fb = fb_dir.join("profile").join(format!("{name}-{:016x}.tsv", image_hash[0]));
            let candidate = read_tsv(&fb);
            if !candidate.is_empty() {
                map = candidate;
                break;
            }
        }
    }
    let mut v: Vec<(u32, u64)> = map.into_iter().collect();
    v.sort_unstable_by_key(|&(rva, n)| (core::cmp::Reverse(n), rva));
    v.into_iter().map(|(rva, _)| rva).collect()
}

/// Build the caches of the selected pending modules (`PORT_AOT_BUILD` / `AW2_DLL_AOT_BUILD`).
unsafe fn build_all(pending: &[(String, usize, usize)], sel: &[String]) {
    let depth = aot_closure_depth();
    let threads = prewarm_threads();
    let mut pool = WorkerCaches::new();
    for (name, base, size) in pending {
        if !sel.is_empty() && !sel.iter().any(|s| name.contains(s.as_str())) {
            continue;
        }
        let (base, size) = (*base, *size);
        let key = module_key(base, size);
        let prof = read_profile(name, &key.image_hash);
        if prof.is_empty() {
            log::info!("emu aot: {name}: no profile ({}); skipped", profile_file(name, &key.image_hash).display());
            continue;
        }
        let t0 = std::time::Instant::now();
        let (ips, cl) = discover_with_threads(&mut pool, base, size, &prof, depth, threads);
        let build_threads = if ips.len() >= PARALLEL_MIN_BLOCKS { threads } else { 1 };
        let Some(img) = build_image_with_threads(&mut pool, &ips, BUILD_VBASE, build_threads) else {
            log::error!("emu aot: {name}: shared stubs don't fit; not built");
            continue;
        };
        let path = cache_file(name, &key);
        match write_atomic(&path, &encode(name, &key, &img, 0)) {
            Ok(()) => log::info!(
                "emu aot: {name}: built {} blocks ({} profiled, {} candidates; closure depth {depth}: {} jump tables / {} targets, {} leaf callees, {} lea leaves, {} cross-function successors skipped, .pdata {} functions, {} chained fragments merged, {} broken; {} bank(s), {build_threads} thread(s)), {} KiB code, {} links, {} relocs, {} assumptions in {:.2}s -> {} ({} cache(s) of other keys removed)",
                img.blocks.len(),
                prof.len(),
                ips.len(),
                cl.jt_tables,
                cl.jt_targets,
                cl.leaf_callees,
                cl.lea_leaves,
                cl.cross,
                cl.functions,
                cl.chained,
                cl.broken,
                img.stub_banks().len(),
                img.code_bytes() >> 10,
                img.links.len(),
                img.relocs.len(),
                img.assume.len(),
                t0.elapsed().as_secs_f64(),
                path.display(),
                delete_other_keys(&cache_dir(), name, &path)
            ),
            Err(e) => log::error!("emu aot: {name}: writing {}: {e}", path.display()),
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Runtime directory
// ---------------------------------------------------------------------------------------------

struct Module {
    name: String,
    base: usize,
    size: usize,
    /// `MAP_JIT` region: bank 0 stubs at offset 0, followed by per-bank code and LSE2 slabs.
    region: usize,
    /// Total byte length of the `MAP_JIT` allocation starting at `region`.
    region_len: usize,
    /// Per-bank runtime byte shift relative to `img`'s flat offsets:
    /// `host_addr(off_in_bank_k) == region + off + bank_shifts[k]`.
    /// Single-bank images have `[0]`; multi-bank images insert a 256 KiB LSE2 slab after each
    /// bank so every instruction stays well within ±128 MiB `b_reach` of its bank's slab.
    bank_shifts: Vec<usize>,
    img: Image,
    /// Per block: 0 = alive, [`DEAD_PERM`] or [`DEAD_ASSUME`].
    dead: Box<[AtomicU8]>,
    /// Per link: currently patched to its target.
    linked: Vec<bool>,
    /// Per block: has a held import slot ([`rebase_block`]); never verified (its
    /// re-translation sees a different slot state, its guarded call is still correct).
    slot_held: Vec<bool>,
    /// Image overlapped run-time code: every block dead.
    killed: bool,
    /// Removed (tests).
    gone: bool,
    /// Test module installed by this thread (tests only touch their own modules).
    #[cfg_attr(not(test), allow(dead_code))]
    owner: Option<std::thread::ThreadId>,
}

impl Module {
    /// Translate a flat `Image` byte offset `img_off` to its live `MAP_JIT` host address.
    #[inline]
    fn host_addr(&self, img_off: u32) -> usize {
        if self.bank_shifts.len() <= 1 {
            self.region + img_off as usize
        } else {
            let bi = self.img.bank_index_of(img_off);
            self.region + img_off as usize + self.bank_shifts[bi]
        }
    }

    /// Translate a live `MAP_JIT` host PC back to a flat `Image` byte offset, or `None`
    /// if `pc` lies in an LSE2 patch slab or outside mapped banks.
    fn img_off_of_pc(&self, pc: usize) -> Option<u64> {
        if pc < self.region || pc >= self.region + self.region_len {
            return None;
        }
        let rel = pc - self.region;
        let banks = self.img.stub_banks();
        if self.bank_shifts.len() <= 1 {
            let end = AOT_STUBS_BYTES + self.img.code_bytes();
            return (rel < end).then_some(rel as u64);
        }
        for (bi, &bo) in banks.iter().enumerate() {
            let shift = self.bank_shifts[bi];
            let rt_lo = bo as usize + shift;
            let rt_hi = self.img.bank_end(bi) + shift;
            if rel >= rt_lo && rel < rt_hi {
                return Some((rel - shift) as u64);
            }
        }
        None
    }
}

struct Dir {
    modules: Vec<Module>,
    /// Live block IP -> (module, block).
    map: HashMap<u64, (u32, u32)>,
    /// Target IP -> patched link sites `(module, link)` pointing at its block.
    incoming: HashMap<u64, Vec<(u32, u32)>>,
    /// Shared probe table published in [`PROBE`] (kept in sync with `map` / `dead`).
    probe: ProbeTab,
}

// ---------------------------------------------------------------------------------------------
// Shared probe table (`AW2_DLL_AOT_PROBE`, read by the `lookup_*` stubs)
// ---------------------------------------------------------------------------------------------

/// Published probe table (0 = none): `[shift, mask, (ip, fast) * (mask + 1)]` (u64 words),
/// open addressing with linear probing from `(ip * JMUL) >> shift`; `ip == 0` ends a chain.
/// `fast` = code of the live, non-host-entry block owning `ip` in [`Dir::map`], else 0 (a
/// tombstone: entries are never removed, so chains never break). Writers hold the `DIR`
/// write lock; growth publishes a new table (old ones are leaked: stubs of running
/// translations may still read them until their next `JitCache::run`).
static PROBE: AtomicUsize = AtomicUsize::new(0);

/// The current [`PROBE`] table for [`super::jit::JitScratch::aot_dir`].
/// Falls back to the bound static AOT probe table when running without `MAP_JIT`.
#[inline]
pub(super) fn probe_table() -> usize {
    let p = PROBE.load(Ordering::Acquire);
    if p != 0 {
        p
    } else {
        super::exec_mem::static_probe_table()
    }
}

const PROBE_MIN_BITS: u32 = 10;

/// Writer-side state of the published probe table.
#[derive(Default)]
struct ProbeTab {
    /// Table address (0 = none yet).
    base: usize,
    /// Entries (a power of two).
    cap: usize,
    /// Entries with `ip != 0` (live or tombstone); kept at most `cap / 2`.
    used: usize,
}

impl ProbeTab {
    /// Entry holding `ip`, or the empty entry ending its chain.
    unsafe fn slot(&self, ip: u64) -> &[AtomicU64; 2] {
        let ents = (self.base + 16) as *const [AtomicU64; 2];
        let mut i = (ip.wrapping_mul(jit::JMUL) >> (64 - self.cap.trailing_zeros())) as usize;
        loop {
            let e = &*ents.add(i);
            let k = e[0].load(Ordering::Relaxed);
            if k == ip || k == 0 {
                return e;
            }
            i = (i + 1) & (self.cap - 1);
        }
    }
}

impl Dir {
    /// Probe-table value for `ip`: the code of its live, non-host-entry owner, else 0.
    fn probe_val(&self, ip: u64) -> u64 {
        let Some(&(mi, bi)) = self.map.get(&ip) else { return 0 };
        let m = &self.modules[mi as usize];
        let b = &m.img.blocks[bi as usize];
        if m.gone || m.killed || b.host_entry || m.dead[bi as usize].load(Ordering::Relaxed) != 0 {
            return 0;
        }
        m.host_addr(b.code_off) as u64
    }

    /// Bring the probe entry of `ip` in line with the map (after any change to its owner or
    /// the owner's `dead` flag). Existing entries are always updated (kills reach every
    /// reader); new ones only with [`jit::aot_probe_enabled`].
    unsafe fn probe_sync(&mut self, ip: u64) {
        let v = self.probe_val(ip);
        if self.probe.base != 0 {
            let e = self.probe.slot(ip);
            if e[0].load(Ordering::Relaxed) == ip {
                e[1].store(v, Ordering::Release);
                return;
            }
            if v != 0 && jit::aot_probe_enabled() && 2 * (self.probe.used + 1) <= self.probe.cap {
                // `fast` before `ip`: a reader that sees the new `ip` with the old (0)
                // `fast` just misses.
                e[1].store(v, Ordering::Relaxed);
                e[0].store(ip, Ordering::Release);
                self.probe.used += 1;
                return;
            }
        }
        if v != 0 && jit::aot_probe_enabled() {
            self.probe_rebuild();
        }
    }

    /// Build a table sized for every live entry of the map and publish it.
    unsafe fn probe_rebuild(&mut self) {
        let live: Vec<(u64, u64)> = self.map.keys().filter_map(|&ip| Some((ip, self.probe_val(ip))).filter(|&(_, v)| v != 0)).collect();
        let bits = (2 * live.len()).max(2).next_power_of_two().trailing_zeros().max(PROBE_MIN_BITS);
        let cap = 1usize << bits;
        let p = Box::leak(vec![0u64; 2 + 2 * cap].into_boxed_slice()).as_mut_ptr();
        *p = 64 - bits as u64;
        *p.add(1) = cap as u64 - 1;
        let t = ProbeTab { base: p as usize, cap, used: live.len() };
        for &(ip, v) in &live {
            let e = t.slot(ip);
            e[1].store(v, Ordering::Relaxed);
            e[0].store(ip, Ordering::Relaxed);
        }
        PROBE.store(t.base, Ordering::Release);
        self.probe = t;
    }
}

/// [`Module::dead`]: verify mismatch, stale import slot or module killed (never revived).
const DEAD_PERM: u8 = 1;
/// [`Module::dead`]: host-callable assumption / relocation check failed; re-checked on every
/// `va_gen` change and revived when it holds again ([`revalidate_dir`]).
const DEAD_ASSUME: u8 = 2;
/// [`Module::dead`]: a [`DEAD_ASSUME`] block whose checks pass again while revival is off
/// (`AW2_DLL_AOT_REVIVE=0`; counted once in [`AotCounts::would_revive`]); revived like
/// [`DEAD_ASSUME`] blocks if revival is on at a later pass.
const DEAD_HELD: u8 = 3;

/// `PORT_AOT_REVIVE` / `AW2_DLL_AOT_REVIVE` (default 1): revive [`DEAD_ASSUME`] blocks whose
/// checks pass again (0: only count them, see [`AotCounts::would_revive`]).
pub(super) fn revive_enabled() -> bool {
    #[cfg(test)]
    if TEST_NO_REVIVE.with(|c| c.get()) {
        return false;
    }
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| env_u32_any(&["PORT_AOT_REVIVE", "AW2_DLL_AOT_REVIVE"], 1) != 0)
}

/// Process-wide AOT invalidation counters (`AW2_JIT_STATS` report, [`counts`]).
pub(super) struct AotCounts {
    pub kills: u64,
    pub revived: u64,
    pub would_revive: u64,
}

static KILLS: AtomicU64 = AtomicU64::new(0);
static REVIVED: AtomicU64 = AtomicU64::new(0);
static WOULD_REVIVE: AtomicU64 = AtomicU64::new(0);

pub(super) fn counts() -> AotCounts {
    AotCounts { kills: KILLS.load(Ordering::Relaxed), revived: REVIVED.load(Ordering::Relaxed), would_revive: WOULD_REVIVE.load(Ordering::Relaxed) }
}

/// AOT lookups can hit (a dynamic or static AOT cache is installed).
#[inline]
pub(super) fn active() -> bool {
    (HAVE.load(Ordering::Relaxed) && aot_mode() != 0) || super::exec_mem::has_bound_static_aot()
}

static DIR: RwLock<Option<Dir>> = RwLock::new(None);
/// Serialises revalidation and module installation (lock order: REVALIDATE, then DIR).
static REVALIDATE: Mutex<()> = Mutex::new(());
/// `va_gen` every live block was last checked against.
static CHECKED_VG: AtomicU32 = AtomicU32::new(0);
/// Bumped when a module is killed (per-thread caches may hold links into it).
static EPOCH: AtomicU32 = AtomicU32::new(0);
/// Any module installed.
static HAVE: AtomicBool = AtomicBool::new(false);

/// AOT module kills so far ([`super::jit::JitCache::run`] flushes when it changes).
#[inline(always)]
pub(super) fn epoch() -> u32 {
    EPOCH.load(Ordering::Relaxed)
}

/// Code memory of all AOT regions writable on this thread while alive.
struct JitWrite;

impl JitWrite {
    unsafe fn new() -> Self {
        jit_write_protect(false);
        JitWrite
    }
}

impl Drop for JitWrite {
    fn drop(&mut self) {
        unsafe { jit_write_protect(true) };
    }
}

/// Atomically re-point the `b`/`bl` (or fallthrough `nop`) at `site` to `target` (link bit kept).
unsafe fn store_b(site: usize, target: usize) {
    let cell = &*(site as *const AtomicU32);
    let old = cell.load(Ordering::Relaxed);
    let link_bit = if old == 0xD503_201F { 0 } else { old & 0x8000_0000 };
    let off = (((target as i64 - site as i64) >> 2) as u32) & 0x03FF_FFFF;
    cell.store(0x1400_0000 | link_bit | off, Ordering::Relaxed);
}

/// Patch a `b.cond` / `cbz` / `cbnz` at `cond_site` directly to `target` if within ±1 MiB.
unsafe fn try_patch_cond_branch(cond_site: usize, target: usize) -> Option<usize> {
    static NO_COND: OnceLock<bool> = OnceLock::new();
    if *NO_COND.get_or_init(|| std::env::var("PORT_AOT_NO_COND_PATCH").is_ok_and(|v| v.trim() != "0" && !v.trim().is_empty())) {
        return None;
    }
    let off = (target as i64 - cond_site as i64) >> 2;
    if !(-(1i64 << 18)..(1i64 << 18)).contains(&off) {
        return None;
    }
    let cell = &*(cond_site as *const AtomicU32);
    let old = cell.load(Ordering::Relaxed);
    debug_assert!(
        (old & 0xFF00_0010) == 0x5400_0000 || (old & 0x7E00_0000) == 0x3400_0000,
        "unexpected cond_off insn {old:#010x}"
    );
    let new_w = (old & 0xFF00_001F) | (((off as u32) & 0x7_FFFF) << 5);
    cell.store(new_w, Ordering::Relaxed);
    Some(cond_site)
}

fn reach(site: usize, target: usize) -> bool {
    let d = target as i64 - site as i64;
    (-(128i64 << 20)..(128i64 << 20)).contains(&d)
}

/// Host-callable (or emulator-routed DLL helper/stub) now, by the same rule as `jit::resolved_host`.
fn host_now(va: u64) -> bool {
    super::resolve_host_callable(va).filter(|&h| !super::thunk_elided(va, h)).is_some()
        || super::is_routed_dll_chkstk(va)
        || super::is_routed_dll_stack_pos(va)
        || super::is_routed_regfile(va)
        || super::is_routed_dll_stub(va)
}

/// Shared code for guest `ip`: `(code, host_entry)`, or None (JIT it). Called from
/// [`super::jit::JitCache`] on an index miss; the caller installs
/// `JEnt { ip, fast: if host_entry { 0 } else { code }, gen: 0, code }`.
pub(super) fn find(ip: u64) -> Option<(usize, bool)> {
    if aot_mode() != 0 && HAVE.load(Ordering::Relaxed) {
        let vg = crate::exe::va_gen();
        if CHECKED_VG.load(Ordering::Acquire) != vg {
            revalidate(vg);
        }
        if let Ok(g) = DIR.read() {
            if let Some(d) = g.as_ref() {
                if let Some(&(mi, bi)) = d.map.get(&ip) {
                    let m = &d.modules[mi as usize];
                    if m.dead[bi as usize].load(Ordering::Acquire) == 0
                        && !super::mem::in_jit(super::to_mapped_addr(ip))
                    {
                        let b = &m.img.blocks[bi as usize];
                        return Some((m.host_addr(b.code_off), b.host_entry));
                    }
                }
            }
        }
    }
    super::exec_mem::lookup_static_aot(ip)
}

/// Re-check a live block's assumptions and relocations against the current host-callable
/// state (memoised per pass); relocation slots whose function moved go to `rewrites`.
unsafe fn check_block(
    m: &Module,
    b: &BlockEnt,
    hm: &mut HashMap<u64, bool>,
    fm: &mut HashMap<u64, Option<(usize, bool)>>,
    rewrites: &mut Vec<(usize, u64)>,
) -> Result<(), &'static str> {
    if super::is_routed_dll_chkstk(b.ip)
        || super::is_routed_dll_stack_pos(b.ip)
        || super::is_routed_regfile(b.ip)
        || super::is_routed_dll_stub(b.ip)
    {
        return Err("routed dll helper");
    }
    for &(va, host) in m.img.assume_of(b) {
        if *hm.entry(va).or_insert_with(|| host_now(va)) != host {
            return Err("host-callable assumption");
        }
    }
    for r in m.img.relocs_of(b) {
        if r.va == SLOT_DISABLED {
            // Held import slot whose guard never matches (rebase_block): nothing to check.
            continue;
        }
        if r.kind == RELOC_SLOT_VAL {
            // The value itself (rebased at load); its HOST_FN sibling checks callability.
            let slot = m.host_addr(r.slot_off);
            if (*(slot as *const AtomicU64)).load(Ordering::Relaxed) != r.va {
                rewrites.push((slot, r.va));
            }
            continue;
        }
        if r.kind == RELOC_QPC_STATE {
            let h = win32_rt::kernel32::time::qpc_fast_state_ptr() as u64;
            let slot = m.host_addr(r.slot_off);
            if (*(slot as *const AtomicU64)).load(Ordering::Relaxed) != h {
                rewrites.push((slot, h));
            }
            continue;
        }
        match *fm.entry(r.va).or_insert_with(|| jit::inline_host_fn(r.va)) {
            Some((h, bail)) if bail == r.bail => {
                let slot = m.host_addr(r.slot_off);
                if (*(slot as *const AtomicU64)).load(Ordering::Relaxed) != h as u64 {
                    rewrites.push((slot, h as u64));
                }
            }
            _ => return Err("inline host fn"),
        }
    }
    Ok(())
}

/// Mark block `bi` of module `mi` dead (`why`: [`DEAD_PERM`] / [`DEAD_ASSUME`]) and unpatch
/// every AOT link site pointing at it (code must be writable on this thread).
unsafe fn kill(d: &mut Dir, mi: usize, bi: usize, why: u8, patched: &mut Vec<usize>) {
    let ip = {
        let m = &d.modules[mi];
        m.dead[bi].store(why, Ordering::Release);
        m.img.blocks[bi].ip
    };
    d.probe_sync(ip);
    if let Some(v) = d.incoming.remove(&ip) {
        for (sm, sl) in v {
            let m = &mut d.modules[sm as usize];
            let l = m.img.links[sl as usize];
            let site = m.host_addr(l.site_off);
            if l.cond_off != u32::MAX {
                let cond_site = m.host_addr(l.cond_off);
                if let Some(cs) = try_patch_cond_branch(cond_site, site) {
                    patched.push(cs);
                }
            }
            store_b(site, m.host_addr(l.fallback_off));
            m.linked[sl as usize] = false;
            patched.push(site);
        }
    }
}

/// Patch every unlinked AOT link site whose target is a live, linkable block in range
/// (code must be writable on this thread). Returns the patched sites.
unsafe fn link_pass(d: &mut Dir) -> Vec<usize> {
    static NO_LINK: OnceLock<bool> = OnceLock::new();
    if *NO_LINK.get_or_init(|| std::env::var("PORT_AOT_NO_LINK").is_ok_and(|v| v.trim() != "0" && !v.trim().is_empty())) {
        return Vec::new();
    }
    let mut todo = Vec::new();
    for (mi, m) in d.modules.iter().enumerate() {
        if m.gone || m.killed {
            continue;
        }
        for b in &m.img.blocks {
            let lo = b.link_lo as usize;
            let hi = lo + b.link_n as usize;
            for li in lo..hi {
                if m.linked[li] {
                    continue;
                }
                let l = m.img.links[li];
                let Some(&(tm, tb)) = d.map.get(&l.target) else { continue };
                let t = &d.modules[tm as usize];
                let tb_ = &t.img.blocks[tb as usize];
                if t.gone || t.dead[tb as usize].load(Ordering::Relaxed) != 0 || tb_.host_entry {
                    continue;
                }
                let (site, code) = (m.host_addr(l.site_off), t.host_addr(tb_.code_off));
                if reach(site, code) {
                    let cond_site = if l.cond_off == u32::MAX { 0 } else { m.host_addr(l.cond_off) };
                    todo.push((mi, li, site, code, l.target, cond_site));
                }
            }
        }
    }
    let mut patched = Vec::with_capacity(todo.len());
    for (mi, li, site, code, target, cond_site) in todo {
        let cell = &*(site as *const AtomicU32);
        let old = cell.load(Ordering::Relaxed);
        if code == site + 4 && (old & 0xFC00_0000) == 0x1400_0000 {
            // Fallthrough directly into the immediately-following hot block: 0-cycle `nop`
            // avoids taking a branch and keeps Apple Silicon's 8-wide fetch group intact!
            cell.store(0xD503_201F, Ordering::Relaxed);
        } else {
            store_b(site, code);
        }
        if cond_site != 0 {
            if let Some(cs) = try_patch_cond_branch(cond_site, code) {
                patched.push(cs);
            }
        }
        d.modules[mi].linked[li] = true;
        d.incoming.entry(target).or_default().push((mi as u32, li as u32));
        patched.push(site);
    }
    patched
}

/// After a `va_gen` change (from `JitCache::run` / [`find`]): re-check every live block's
/// assumptions and relocations once per generation; kill failing blocks (unpatching their
/// incoming links) and rewrite moved host-fn slots. Threads still inside a killed block
/// finish it with the old decisions (the JIT's window today); killed code is never freed.
pub(super) fn revalidate(vg: u32) {
    if aot_mode() == 0 || CHECKED_VG.load(Ordering::Acquire) == vg {
        return;
    }
    let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
    let vg = crate::exe::va_gen();
    if CHECKED_VG.load(Ordering::Acquire) == vg {
        return;
    }
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    if let Some(d) = g.as_mut() {
        unsafe { revalidate_dir(d) };
    }
    CHECKED_VG.store(vg, Ordering::Release);
}

/// Re-check every live and every [`DEAD_ASSUME`] block: kill live blocks that fail, revive
/// dead ones that pass again (relocation slots rewritten, back in the map, re-linked; with
/// `AW2_DLL_AOT_VERIFY` re-verified first). Returns the number of blocks killed.
unsafe fn revalidate_dir(d: &mut Dir) -> usize {
    let (mut hm, mut fm) = (HashMap::new(), HashMap::new());
    let (mut kills, mut rewrites, mut revive) = (Vec::new(), Vec::new(), Vec::new());
    for (mi, m) in d.modules.iter().enumerate() {
        if m.gone || m.killed {
            continue;
        }
        for (bi, b) in m.img.blocks.iter().enumerate() {
            match m.dead[bi].load(Ordering::Relaxed) {
                0 => {
                    if check_block(m, b, &mut hm, &mut fm, &mut rewrites).is_err() {
                        kills.push((mi, bi));
                    }
                }
                st @ (DEAD_ASSUME | DEAD_HELD) => {
                    let mut rw = Vec::new();
                    if check_block(m, b, &mut hm, &mut fm, &mut rw).is_ok() {
                        revive.push((mi, bi, rw));
                    } else if st == DEAD_HELD {
                        m.dead[bi].store(DEAD_ASSUME, Ordering::Relaxed);
                    }
                }
                _ => {}
            }
        }
    }
    // Blocks whose checks started passing in this pass (DEAD_HELD ones were counted before).
    let would = revive.iter().filter(|&&(mi, bi, _)| d.modules[mi].dead[bi].load(Ordering::Relaxed) == DEAD_ASSUME).count();
    WOULD_REVIVE.fetch_add(would as u64, Ordering::Relaxed);
    if !revive_enabled() {
        for &(mi, bi, _) in &revive {
            d.modules[mi].dead[bi].store(DEAD_HELD, Ordering::Relaxed);
        }
        revive.clear();
    }
    if kills.is_empty() && rewrites.is_empty() && revive.is_empty() {
        return 0;
    }
    let mut patched = Vec::new();
    let mut revived = 0usize;
    {
        let _w = JitWrite::new();
        for &(slot, h) in &rewrites {
            (*(slot as *const AtomicU64)).store(h, Ordering::Relaxed);
        }
        for &(mi, bi) in &kills {
            kill(d, mi, bi, DEAD_ASSUME, &mut patched);
        }
        let mut vbc: Option<Box<BlockCache>> = None;
        let mut scratch = Vec::new();
        for (mi, bi, rw) in revive {
            for &(slot, h) in &rw {
                (*(slot as *const AtomicU64)).store(h, Ordering::Relaxed);
            }
            let m = &d.modules[mi];
            let b = m.img.blocks[bi];
            if verify_mode() != 0 && !m.slot_held[bi] {
                let bc = vbc.get_or_insert_with(|| Box::new(BlockCache::new()));
                let bcp: *mut BlockCache = &mut **bc;
                let bank_vbase = m.host_addr(m.img.bank_of_off(b.code_off));
                if let Err(e) = verify_block(bcp, bank_vbase, &m.img, &b, &mut scratch) {
                    log::warn!("emu aot: {}: revived block {:#x} fails verify: {e}", m.name, b.ip);
                    m.dead[bi].store(DEAD_PERM, Ordering::Release);
                    continue;
                }
            }
            icache_invalidate(m.host_addr(b.code_off) as *mut u8, b.main as usize * 4);
            if b.words > b.main {
                icache_invalidate(m.host_addr(b.cold_off) as *mut u8, (b.words - b.main) as usize * 4);
            }
            m.dead[bi].store(0, Ordering::Release);
            // Another module's live block may own the IP (overlapping images): keep it.
            let owner_live = d.map.get(&b.ip).is_some_and(|&(m2, b2)| (m2 as usize, b2 as usize) != (mi, bi) && d.modules[m2 as usize].dead[b2 as usize].load(Ordering::Relaxed) == 0);
            if !owner_live {
                d.map.insert(b.ip, (mi as u32, bi as u32));
                d.probe_sync(b.ip);
            }
            revived += 1;
        }
        if revived != 0 {
            patched.extend(link_pass(d));
        }
    }
    for s in patched {
        icache_invalidate(s as *mut u8, 4);
    }
    KILLS.fetch_add(kills.len() as u64, Ordering::Relaxed);
    REVIVED.fetch_add(revived as u64, Ordering::Relaxed);
    if !kills.is_empty() || would != 0 {
        log::info!(
            "emu aot: {} block(s) invalidated, {} revived ({} would revive{}), {} host-fn slot(s) rewritten after a host dispatch change",
            kills.len(),
            revived,
            would,
            if revive_enabled() { "" } else { "; AW2_DLL_AOT_REVIVE=0" },
            rewrites.len()
        );
    }
    kills.len()
}

/// Executable memory changed (`mem::note_code_region`): a module whose image now overlaps
/// a tracked run-time code region is killed entirely.
pub(super) fn note_code_region() {
    if !HAVE.load(Ordering::Relaxed) {
        return;
    }
    let hit = |m: &Module| !m.gone && !m.killed && super::mem::jit_overlaps(m.base, m.base + m.size);
    {
        let Ok(g) = DIR.read() else { return };
        if !g.as_ref().is_some_and(|d| d.modules.iter().any(hit)) {
            return;
        }
    }
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    let Some(d) = g.as_mut() else { return };
    let mut patched = Vec::new();
    unsafe {
        let _w = JitWrite::new();
        for mi in 0..d.modules.len() {
            if !hit(&d.modules[mi]) {
                continue;
            }
            d.modules[mi].killed = true;
            log::warn!("emu aot: {} overlaps run-time generated code; its AOT code is dropped", d.modules[mi].name);
            for bi in 0..d.modules[mi].img.blocks.len() {
                if d.modules[mi].dead[bi].load(Ordering::Relaxed) != DEAD_PERM {
                    kill(d, mi, bi, DEAD_PERM, &mut patched);
                }
            }
        }
    }
    for s in patched {
        unsafe { icache_invalidate(s as *mut u8, 4) };
    }
    EPOCH.fetch_add(1, Ordering::Release);
}

/// Guest import slot `src` of a [`RelocEnt`] may be read (same rule as `const_slot_plain`).
fn import_slot_ok(src: u64) -> bool {
    let b = crate::exe::base() as u64;
    let in_image = (b..b.wrapping_add(crate::exe::image_size())).contains(&src) || crate::dll_native::in_mapped_image(src);
    (in_image || cfg!(test)) && (0x1_0000_0000..1u64 << 47).contains(&src) && src & 7 == 0
}

/// Point block `b`'s import-slot relocations (`src` != 0) at the slot's current value (it
/// differs between processes when the slot holds a host-side thunk VA) and rewrite its
/// `RELOC_SLOT_VAL` words (code writable on this thread; `RELOC_HOST_FN` words follow in
/// [`check_block`]). Each slot is read once per install (`slots`).
///
/// A slot whose current value is not inline-callable with the recorded bail (e.g. a
/// delay-load resolver thunk before the first call, where a background build saw the
/// resolved target) is *held*: the code compares the slot with the recorded value at run
/// time, so its calls take the generic path while they differ. If the recorded value is
/// still inline-callable here it is kept (the inline path applies once the slot holds it);
/// otherwise the slot's relocations become [`SLOT_DISABLED`] (guard never matches, host fn
/// word unused). Held slots go to `held` as `(slot, recorded, current)`.
///
/// Returns the number of relocations moved, or None (stale block) if a slot is unreadable.
unsafe fn rebase_block(
    img: &mut Image,
    region: usize,
    bank_shifts: &[usize],
    b: &BlockEnt,
    slots: &mut HashMap<u64, u64>,
    held: &mut Vec<(u64, u64, u64)>,
) -> Option<usize> {
    let (lo, hi) = (b.reloc_lo as usize, (b.reloc_lo + b.reloc_n) as usize);
    // (slot, keep the recorded value) for this block's held slots.
    let mut keep: Vec<(u64, bool)> = Vec::new();
    for i in lo..hi {
        let r = img.relocs[i];
        if r.src == 0 {
            continue;
        }
        if !import_slot_ok(r.src) {
            return None;
        }
        let cur = *slots.entry(r.src).or_insert_with(|| core::ptr::read_volatile(r.src as *const u64));
        // The slot's HOST_FN relocation decides callability (a SLOT_VAL one is only the
        // guard value / bail target of the same call).
        let callable = |v: u64| matches!(jit::inline_host_fn(v), Some((_, bail)) if bail == r.bail);
        if r.kind == RELOC_HOST_FN && r.va != SLOT_DISABLED && cur != r.va && !callable(cur) && !keep.iter().any(|k| k.0 == r.src) {
            keep.push((r.src, callable(r.va)));
            held.push((r.src, r.va, cur));
        }
    }
    let mut moved = 0;
    for i in lo..hi {
        let r = img.relocs[i];
        if r.src == 0 || r.va == SLOT_DISABLED {
            continue;
        }
        let new = match keep.iter().find(|k| k.0 == r.src) {
            Some(&(_, true)) => continue,
            Some(&(_, false)) => SLOT_DISABLED,
            None => slots[&r.src],
        };
        if new == r.va {
            continue;
        }
        img.relocs[i].va = new;
        if r.kind == RELOC_SLOT_VAL {
            let shift = if bank_shifts.len() <= 1 { 0 } else { bank_shifts[img.bank_index_of(r.slot_off)] };
            (*((region + r.slot_off as usize + shift) as *const AtomicU64)).store(new, Ordering::Relaxed);
        }
        moved += (new != SLOT_DISABLED) as usize;
    }
    Some(moved)
}

/// [`RelocEnt::va`] of a held import slot's relocations whose recorded value is not
/// inline-callable in this process ([`rebase_block`]): the guard compares the slot with
/// this (no user-space VA), so the inline path is never taken.
const SLOT_DISABLED: u64 = u64::MAX;

/// Re-translate block `b` at its final address inside its stub bank (`bank_vbase`) and
/// compare it with the image.
unsafe fn verify_block(bc: *mut BlockCache, bank_vbase: usize, img: &Image, b: &BlockEnt, scratch: &mut Vec<u32>) -> Result<(), String> {
    let ho = b.code_off;
    let co = b.cold_off;
    let bank_off = img.bank_of_off(ho);
    let ho_rel = (ho - bank_off) as usize;
    let co_rel = (co - bank_off) as usize;
    let Some(mut t) = jit::aot_translate(bc, b.ip, bank_vbase + ho_rel, bank_vbase, &img.stubs, scratch) else {
        return Err("no longer translatable".into());
    };
    if t.words.len() != b.words as usize || t.main != b.main || t.guest != b.guest || t.host_entry != b.host_entry {
        return Err(format!(
            "shape: words {}/{} main {}/{} guest {}/{} host {}/{}",
            t.words.len(),
            b.words,
            t.main,
            b.main,
            t.guest,
            b.guest,
            t.host_entry,
            b.host_entry
        ));
    }
    split_and_rebase_block(&mut t, ho_rel, 0, ho_rel, co_rel, 0, &img.stubs);
    let main = b.main;
    let links: Vec<LinkEnt> = t
        .links
        .iter()
        .map(|&(s, f, c, tg)| LinkEnt {
            site_off: word_to_off(s, main, ho, co),
            fallback_off: word_to_off(f, main, ho, co),
            cond_off: if c == u32::MAX { u32::MAX } else { word_to_off(c, main, ho, co) },
            target: tg,
        })
        .collect();
    if links[..] != *img.links_of(b) {
        return Err("links differ".into());
    }
    let relocs: Vec<RelocEnt> = t.relocs.iter().map(|r| RelocEnt::of_split(ho, co, main, r)).collect();
    // Compared by (site, kind, bail, guest VA, import slot): host pointers are not part of
    // a RelocEnt (the slot words themselves are skipped below).
    let file_relocs = img.relocs_of(b);
    if relocs[..] != *file_relocs {
        let i = relocs.iter().zip(file_relocs).position(|(x, y)| x != y).unwrap_or(relocs.len().min(file_relocs.len()));
        return Err(format!(
            "relocations differ ({} now, {} in the file; first difference #{i}: now {:?}, file {:?})",
            relocs.len(),
            file_relocs.len(),
            relocs.get(i),
            file_relocs.get(i)
        ));
    }
    if t.assume[..] != *img.assume_of(b) {
        return Err("assumptions differ".into());
    }
    let skip: HashSet<usize> = t.relocs.iter().flat_map(|r| [r.word as usize, r.word as usize + 1]).collect();
    let main_u = main as usize;
    let file_hot = img.hot_words_of(b);
    for (i, (&x, &y)) in t.words[..main_u].iter().zip(file_hot).enumerate() {
        if x != y && !skip.contains(&i) {
            return Err(format!("word {i}: now {x:#010x} (main), file {y:#010x}"));
        }
    }
    let file_cold = img.cold_words_of(b);
    for (ci, (&x, &y)) in t.words[main_u..].iter().zip(file_cold).enumerate() {
        let i = main_u + ci;
        if x != y && !skip.contains(&i) {
            return Err(format!("word {i}: now {x:#010x} (out-of-line), file {y:#010x}"));
        }
    }
    Ok(())
}

/// Which blocks [`install_image`] re-translates and compares ([`verify_block`]).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Verify {
    None,
    All,
    /// About `pct`% of the blocks, picked by hashing the block IP with `seed`.
    Sample { pct: u32, seed: u64 },
}

impl Verify {
    fn picks(self, ip: u64) -> bool {
        match self {
            Verify::None => false,
            Verify::All => true,
            Verify::Sample { pct, seed } => {
                let mut x = ip ^ seed;
                x = (x ^ (x >> 33)).wrapping_mul(0xFF51_AFD7_ED55_8CCD);
                x = (x ^ (x >> 33)).wrapping_mul(0xC4CE_B9FE_1A85_EC53);
                (x ^ (x >> 33)) % 100 < pct as u64
            }
        }
    }
}

/// [`install_image`] result.
#[derive(Clone, Copy, Debug)]
struct Installed {
    live: usize,
    /// Blocks re-translated and compared ...
    verified: usize,
    /// ... of which mismatched (dropped).
    mismatches: usize,
}

/// Copy `img` behind fresh stubs into a new `MAP_JIT` region, check/relocate/link it and
/// publish its live blocks (None: not installed).
unsafe fn install_image(name: &str, base: usize, size: usize, img: Image, verify: Verify, test: bool) -> Option<Installed> {
    if !test && super::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!("emu aot: {name}: overlaps run-time generated/modified code; skipping AOT install");
        return None;
    }
    if !jit_writable() {
        log::info!(
            "emu aot: {name}: MAP_JIT unavailable; skipping dynamic .aot install (using static AOT / interpreter)"
        );
        return None;
    }
    let code_bytes = img.code_bytes();
    let banks_snap: Vec<(usize, usize)> = img
        .stub_banks()
        .iter()
        .enumerate()
        .map(|(bi, &bo)| (bo as usize, img.bank_end(bi)))
        .collect();
    let nbanks = banks_snap.len();
    let mut bank_shifts: Vec<usize> = Vec::with_capacity(nbanks);
    let mut bank_rt_offs: Vec<usize> = Vec::with_capacity(nbanks);
    let mut bank_slab_offs: Vec<usize> = Vec::with_capacity(nbanks);
    let mut cursor = 0usize;
    for &(bo, b_end) in &banks_snap {
        cursor = (cursor + 15) & !15;
        bank_rt_offs.push(cursor);
        bank_shifts.push(cursor - bo);
        let bank_len = b_end - bo;
        let slab_off = (cursor + bank_len + 15) & !15;
        bank_slab_offs.push(slab_off);
        cursor = slab_off + AOT_LSE2_SLAB;
    }
    let len = (cursor + 0x3FFF) & !0x3FFF;
    let Some(p) = alloc_jit(len) else {
        log::error!("emu aot: {name}: mmap(MAP_JIT, {len:#x}) failed");
        return None;
    };
    let region = p as usize;
    for bi in 0..nbanks {
        let rt_base = region + bank_rt_offs[bi];
        let bank_len = banks_snap[bi].1 - banks_snap[bi].0;
        let code_lo = rt_base + AOT_STUBS_BYTES;
        let code_hi = rt_base + bank_len;
        let slab_lo = region + bank_slab_offs[bi];
        let slab_hi = slab_lo + AOT_LSE2_SLAB;
        // Guard `b_reach` (±128 MiB) so `patch_lse2_unaligned` can reach every slot in the slab
        // from every instruction in this bank.
        if code_lo < code_hi && reach(code_lo, slab_hi) && reach(slab_hi, code_lo) {
            jit::register_lse2_region(code_lo, code_hi, slab_lo, slab_hi);
        }
    }
    let t0 = std::time::Instant::now();
    let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
    let n = img.blocks.len();
    let nlinks = img.links.len();
    let mut m = Module {
        name: name.to_string(),
        base,
        size,
        region,
        region_len: len,
        bank_shifts,
        img,
        dead: (0..n).map(|_| AtomicU8::new(0)).collect(),
        linked: vec![false; nlinks],
        slot_held: vec![false; n],
        killed: false,
        gone: false,
        owner: test.then(|| std::thread::current().id()),
    };
    let w = JitWrite::new();
    for bi in 0..nbanks {
        let rt_base = region + bank_rt_offs[bi];
        if jit::aot_emit_stubs(rt_base as *mut u32, rt_base) != Some(m.img.stubs) {
            drop(w);
            log::warn!("emu aot: {name}: shared stub layout differs from the cache (bank {bi}); ignored");
            return None;
        }
        let code_start_off = banks_snap[bi].0 + AOT_STUBS_BYTES;
        let code_end_off = banks_snap[bi].1;
        if code_end_off > code_start_off {
            let word_lo = (code_start_off - AOT_STUBS_BYTES) / 4;
            let word_len = (code_end_off - code_start_off) / 4;
            core::ptr::copy_nonoverlapping(
                m.img.code.as_ptr().add(word_lo),
                (rt_base + AOT_STUBS_BYTES) as *mut u32,
                word_len,
            );
        }
    }
    let (mut bad_verify, mut bad_check, mut bad_slot, mut rebased) = (0usize, 0usize, 0usize, 0usize);
    let mut slots = HashMap::new();
    // Blocks with a held import slot (see rebase_block): correct (guarded) but not verified,
    // since their re-translation now sees a different slot state.
    let mut held_blocks = 0usize;
    let mut held = Vec::new();
    for bi in 0..n {
        let b = m.img.blocks[bi];
        held.clear();
        match rebase_block(&mut m.img, region, &m.bank_shifts, &b, &mut slots, &mut held) {
            Some(k) => rebased += k,
            None => {
                m.dead[bi].store(DEAD_PERM, Ordering::Relaxed);
                bad_slot += 1;
                continue;
            }
        }
        if held.is_empty() {
            continue;
        }
        m.slot_held[bi] = true;
        if held_blocks < 4 {
            for &(slot, rec, cur) in &held {
                let off = if m.img.relocs_of(&b).iter().any(|r| r.src == slot && r.va == SLOT_DISABLED) {
                    "; recorded value not callable here: disabled"
                } else {
                    ""
                };
                log::info!(
                    "emu aot: {name}: block {:#x}: import slot {slot:#x} holds {cur:#x}, cache recorded {rec:#x} (guarded call takes the generic path{off})",
                    b.ip
                );
            }
        }
        held_blocks += 1;
    }
    #[cfg(test)]
    {
        TEST_REBASED.with(|c| c.set(c.get() + rebased));
        TEST_HELD.with(|c| c.set(c.get() + held_blocks));
    }
    // Assumptions first: a block built for another host-callable state is dead-but-revivable
    // (its re-translation now would differ, so it is not verified until revived).
    let (mut hm, mut fm, mut rewrites) = (HashMap::new(), HashMap::new(), Vec::new());
    for bi in 0..n {
        if m.dead[bi].load(Ordering::Relaxed) != 0 {
            continue;
        }
        let b = m.img.blocks[bi];
        if check_block(&m, &b, &mut hm, &mut fm, &mut rewrites).is_err() {
            m.dead[bi].store(DEAD_ASSUME, Ordering::Relaxed);
            bad_check += 1;
        }
    }
    for &(slot, h) in &rewrites {
        (*(slot as *const AtomicU64)).store(h, Ordering::Relaxed);
    }
    let mut verified = 0usize;
    if verify != Verify::None {
        let mut bc = Box::new(BlockCache::new());
        let bcp: *mut BlockCache = &mut *bc;
        let mut scratch = Vec::new();
        for bi in 0..n {
            let b = m.img.blocks[bi];
            if m.dead[bi].load(Ordering::Relaxed) != 0 || m.slot_held[bi] || !verify.picks(b.ip) {
                continue;
            }
            verified += 1;
            let bank_vbase = m.host_addr(m.img.bank_of_off(b.code_off));
            if let Err(e) = verify_block(bcp, bank_vbase, &m.img, &b, &mut scratch) {
                if bad_verify < 8 {
                    log::warn!("emu aot: {name}: verify mismatch in block {:#x}: {e}", b.ip);
                }
                bad_verify += 1;
                m.dead[bi].store(DEAD_PERM, Ordering::Relaxed);
            }
        }
    }
    #[cfg(test)]
    {
        TEST_VERIFIED.with(|c| c.set(c.get() + verified));
        TEST_VERIFY_MISMATCHES.with(|c| c.set(c.get() + bad_verify));
    }
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    let d = g.get_or_insert_with(|| Dir { modules: Vec::new(), map: HashMap::new(), incoming: HashMap::new(), probe: ProbeTab::default() });
    let mi = d.modules.len() as u32;
    let mut live_ips = Vec::new();
    let aot_filt = crate::diag::aot_filter_active();
    let mut filt_skipped = 0usize;
    for (bi, b) in m.img.blocks.iter().enumerate() {
        if m.dead[bi].load(Ordering::Relaxed) == 0 {
            if aot_filt && !crate::diag::aot_allows(b.ip) {
                m.dead[bi].store(DEAD_PERM, Ordering::Relaxed);
                filt_skipped += 1;
                continue;
            }
            d.map.insert(b.ip, (mi, bi as u32));
            live_ips.push(b.ip);
        }
    }
    if filt_skipped != 0 {
        log::info!("emu aot: {name}: PORT_AOT_SKIP/ALLOW excluded {filt_skipped}/{n} blocks (falling back to live JIT)");
    }
    let live = live_ips.len();
    d.modules.push(m);
    let patched = link_pass(d);
    let linked = d.modules[mi as usize].linked.iter().filter(|&&b| b).count();
    let miss = unlinked_reasons(d, mi as usize);
    drop(w);
    icache_invalidate(region as *mut u8, len);
    for &s in &patched {
        if !(region..region + len).contains(&s) {
            icache_invalidate(s as *mut u8, 4);
        }
    }
    // Lock-free readers (the stubs' probe) only see the code once it is coherent.
    for ip in live_ips {
        d.probe_sync(ip);
    }
    HAVE.store(true, Ordering::Release);
    drop(g);
    log::info!(
        "emu aot: {name}: {live}/{n} blocks live ({} KiB code across {nbanks} bank(s) at {region:#x}; {bad_verify} verify mismatches ({verified} verified), {bad_check} stale (revivable), {bad_slot} stale import slots, {held_blocks} slot state differs (guarded, unverified); {rebased} slot relocations rebased; {linked}/{nlinks} links patched, unpatched: {} target not in a cache, {} target dead, {} host-callable target, {} out of b range; {:.1} ms)",
        code_bytes >> 10,
        miss[0],
        miss[1],
        miss[2],
        miss[3],
        t0.elapsed().as_secs_f64() * 1e3
    );
    Some(Installed { live, verified, mismatches: bad_verify })
}

/// Why module `mi`'s unpatched link sites are unpatched: `[target not in any loaded cache
/// (not built, or in a module not loaded yet), target block dead, target host-callable
/// (never a direct-link target), target out of `b` range]`.
fn unlinked_reasons(d: &Dir, mi: usize) -> [usize; 4] {
    let m = &d.modules[mi];
    let mut miss = [0usize; 4];
    for (li, l) in m.img.links.iter().enumerate() {
        if m.linked[li] {
            continue;
        }
        let k = match d.map.get(&l.target) {
            Some(&(tm, tb)) => {
                let t = &d.modules[tm as usize];
                if t.dead[tb as usize].load(Ordering::Relaxed) != 0 {
                    1
                } else if t.img.blocks[tb as usize].host_entry {
                    2
                } else {
                    3
                }
            }
            // Blocks are sorted by IP; dead ones are not in the directory map.
            None => match m.img.blocks.binary_search_by_key(&l.target, |b| b.ip) {
                Ok(_) => 1,
                Err(_) => 0,
            },
        };
        miss[k] += 1;
    }
    miss
}

fn read_cache_file_in(
    primary_dir: &Path,
    fallback_dirs: &[PathBuf],
    name: &str,
    key: &AotKey,
) -> (PathBuf, Option<(Vec<u8>, bool)>) {
    let fname = format!("{name}-{:016x}.aot", key.id());
    let primary = primary_dir.join(&fname);
    if let Ok(bytes) = std::fs::read(&primary) {
        return (primary, Some((bytes, false)));
    }
    for fb_dir in fallback_dirs {
        let fb = fb_dir.join(&fname);
        if let Ok(bytes) = std::fs::read(&fb) {
            log::info!("emu aot: {name}: loading cache from read-only fallback {}", fb.display());
            return (fb, Some((bytes, true)));
        }
    }
    (primary, None)
}

/// Read an existing cache file for `(name, key)` from writable [`cache_dir`], falling back
/// read-only to [`fallback_cache_dirs`] (`PORT_AOT_FALLBACK_DIR`, bundled `Contents/Resources/AotCache`,
/// and legacy `~/Library/Caches/aw2-port/aot`).
fn read_cache_file(name: &str, key: &AotKey) -> (PathBuf, Option<(Vec<u8>, bool)>) {
    let primary_dir = cache_dir();
    let fallbacks = fallback_cache_dirs_for(&primary_dir);
    read_cache_file_in(&primary_dir, &fallbacks, name, key)
}

/// Load the cache of image `name` mapped at `[base, base + size)` if it matches `key`
/// (computed at activation). Returns the IPs of the installed image's blocks (live or not),
/// or None if no valid cache was installed.
unsafe fn load_module(name: &str, base: usize, size: usize, key: &AotKey) -> Option<Vec<u64>> {
    if super::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!("emu aot: {name}: overlaps run-time generated/modified code; skipping AOT load");
        return None;
    }
    let t0 = std::time::Instant::now();
    let (path, hit) = read_cache_file(name, key);
    let Some((bytes, from_fallback)) = hit else {
        log::info!("emu aot: {name}: no cache for this build/knobs ({})", path.display());
        return None;
    };
    let Some((fname, fkey, img, flags)) = decode(&bytes) else {
        log::warn!("emu aot: {name}: {} is corrupt; ignored", path.display());
        return None;
    };
    if fname != name || fkey != *key {
        log::warn!("emu aot: {name}: {} has a different key; ignored", path.display());
        return None;
    }
    log::info!("emu aot: {name}: read {} KiB in {:.1} ms", bytes.len() >> 10, t0.elapsed().as_secs_f64() * 1e3);
    if !from_fallback {
        touch(&path);
    }
    let ips: Vec<u64> = img.blocks.iter().map(|b| b.ip).collect();
    let pending = flags & FLAG_VERIFY_PENDING != 0;
    let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos() as u64);
    let verify = load_verify(verify_mode() != 0, pending, ips.len(), seed);
    let r = install_image(name, base, size, img, verify, false)?;
    if pending {
        let cleared = !from_fallback && r.mismatches == 0 && {
            let mut b = bytes;
            set_flags(&mut b, flags & !FLAG_VERIFY_PENDING) && write_atomic(&path, &b).is_ok()
        };
        log::info!(
            "emu aot: {name}: first load of a background-built cache: {} of {} blocks verified ({}), {} mismatches, {} live; {}",
            r.verified,
            ips.len(),
            match verify {
                Verify::Sample { pct, .. } => format!("{pct}% sample"),
                _ => "all".into(),
            },
            r.mismatches,
            r.live,
            if cleared {
                "flag cleared"
            } else if from_fallback {
                "read-only fallback left untouched"
            } else {
                "verified again at the next load"
            }
        );
    }
    Some(ips)
}

/// Blocks above which the first load of a background-built cache verifies a sample ...
const FIRST_LOAD_FULL_MAX: usize = 50_000;
/// ... of this many percent of the blocks.
const FIRST_LOAD_SAMPLE_PCT: u32 = 2;

/// What [`load_module`] verifies: everything with `AW2_DLL_AOT_VERIFY=1` (`explicit`);
/// a background-built cache's first load (`pending`): all of a module with fewer than
/// [`FIRST_LOAD_FULL_MAX`] blocks, else a [`FIRST_LOAD_SAMPLE_PCT`]% sample (load cost
/// bounded); otherwise nothing.
fn load_verify(explicit: bool, pending: bool, blocks: usize, seed: u64) -> Verify {
    if explicit || (pending && blocks < FIRST_LOAD_FULL_MAX) {
        Verify::All
    } else if pending {
        Verify::Sample { pct: FIRST_LOAD_SAMPLE_PCT, seed }
    } else {
        Verify::None
    }
}

// ---------------------------------------------------------------------------------------------
// Background builds (`AW2_DLL_AOT=2`)
// ---------------------------------------------------------------------------------------------

/// One background build: module, its activation-time key, and whether the result is
/// installed into this process (no valid cache was loaded) or only written for the next run
/// (the loaded cache's profile grew).
struct BgJob {
    name: String,
    base: usize,
    size: usize,
    key: AotKey,
    install: bool,
}

/// Pending jobs and whether the builder thread is running (both under this lock).
static BG: Mutex<(Vec<BgJob>, bool)> = Mutex::new((Vec::new(), false));

/// A loaded cache is rebuilt for the next run when more than 1/`BG_STALE_DIV` of the
/// module's profiled blocks, or more than `BG_STALE_MAX` of them, are not in it.
const BG_STALE_DIV: usize = 10;
const BG_STALE_MAX: usize = 2000;

/// `AW2_DLL_AOT=2` at activation: the background job for module `name` given the load
/// result (`loaded`: block IPs of the installed cache), or None.
fn bg_job(name: &str, base: usize, size: usize, key: AotKey, loaded: Option<&[u64]>) -> Option<BgJob> {
    if super::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!("emu aot: {name}: overlaps run-time generated/modified code; skipping background AOT build");
        return None;
    }
    let prof = read_profile(name, &key.image_hash);
    if prof.is_empty() {
        log::info!("emu aot: {name}: no profile yet (recording; AW2_DLL_AOT=2 builds at a later start)");
        return None;
    }
    let install = match loaded {
        None => true,
        Some(ips) => {
            let have: HashSet<u64> = ips.iter().copied().collect();
            let new = prof.iter().filter(|&&rva| !have.contains(&(base as u64 + rva as u64))).count();
            if new * BG_STALE_DIV <= prof.len() && new <= BG_STALE_MAX {
                return None;
            }
            log::info!("emu aot: {name}: {new} of {} profiled blocks not in the cache; rebuilding it in the background for the next run", prof.len());
            false
        }
    };
    Some(BgJob { name: name.to_string(), base, size, key, install })
}

/// Queue `jobs` for the builder thread (started on demand; exits when the queue is empty).
fn bg_enqueue(jobs: Vec<BgJob>) {
    let mut g = BG.lock().unwrap_or_else(|e| e.into_inner());
    g.0.extend(jobs);
    if g.1 || g.0.is_empty() {
        return;
    }
    g.1 = true;
    let spawned = std::thread::Builder::new().name("port-aot-build".into()).stack_size(16 << 20).spawn(bg_main);
    if let Err(e) = spawned {
        log::error!("emu aot: cannot start the background build thread: {e}");
        g.0.clear();
        g.1 = false;
    }
}

/// Builder thread: jobs whose result is installed now first (utility QoS); next-run
/// rebuilds wait until [`bg_delay`] after the first activation and run at background QoS.
fn bg_main() {
    let mut bc = Box::new(BlockCache::new());
    let bcp: *mut BlockCache = &mut *bc;
    let mut qos = 0;
    loop {
        let job = {
            let mut g = BG.lock().unwrap_or_else(|e| e.into_inner());
            if g.0.is_empty() {
                g.1 = false;
                return;
            }
            match g.0.iter().position(|j| j.install) {
                Some(i) => Some(g.0.remove(i)),
                None if bg_wait().is_zero() => Some(g.0.remove(0)),
                None => None,
            }
        };
        let Some(job) = job else {
            // Re-checked every second: an install job (late LoadLibrary) goes first.
            std::thread::sleep(bg_wait().min(std::time::Duration::from_secs(1)));
            continue;
        };
        let want = if job.install { QOS_CLASS_UTILITY } else { QOS_CLASS_BACKGROUND };
        if want != qos {
            set_qos(want);
            qos = want;
        }
        unsafe { bg_build(bcp, job) };
    }
}

/// Build `job`'s module from its profile on this (background) thread; discard the result if
/// the image's code changed since activation; write the cache (flagged for verification at
/// its first load, [`FLAG_VERIFY_PENDING`]); install it if requested.
unsafe fn bg_build(bcp: *mut BlockCache, job: BgJob) {
    let BgJob { name, base, size, key, install } = job;
    if super::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!("emu aot: {name}: overlaps run-time generated/modified code; skipping background AOT build");
        return;
    }
    let t0 = std::time::Instant::now();
    let prof = read_profile(&name, &key.image_hash);
    let depth = aot_closure_depth();
    let (ips, _) = discover(bcp, base, size, &prof, depth);
    if super::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!("emu aot: {name}: became run-time modified during discovery; aborting background AOT build");
        return;
    }
    let Some(img) = build_image(bcp, &ips, BUILD_VBASE) else {
        log::error!("emu aot: {name}: shared stubs don't fit; not built");
        return;
    };
    if super::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!("emu aot: {name}: became run-time modified during build; result discarded");
        return;
    }
    if image_hash(base, size) != key.image_hash {
        log::warn!("emu aot: {name}: code changed during the background build; result discarded");
        return;
    }
    let build_s = t0.elapsed().as_secs_f64();
    let path = cache_file(&name, &key);
    let mut evicted = 0;
    match write_atomic(&path, &encode(&name, &key, &img, FLAG_VERIFY_PENDING)) {
        Ok(()) => evicted = delete_other_keys(&cache_dir(), &name, &path),
        Err(e) => log::error!("emu aot: {name}: writing {}: {e}", path.display()),
    }
    log::info!(
        "emu aot: {name}: background build: {} blocks ({} profiled, {} candidates), {} KiB code in {build_s:.2}s -> {}{}{}",
        img.blocks.len(),
        prof.len(),
        ips.len(),
        img.code_bytes() >> 10,
        path.display(),
        if install { "; installing" } else { " (next run)" },
        if evicted != 0 { format!("; {evicted} cache(s) of other keys removed") } else { String::new() }
    );
    if install {
        install_image(&name, base, size, img, if verify_mode() != 0 { Verify::All } else { Verify::None }, false);
    }
}

// ---------------------------------------------------------------------------------------------
// Mapped images / activation (loader entry points)
// ---------------------------------------------------------------------------------------------

struct Mapped {
    name: String,
    base: usize,
    size: usize,
    active: bool,
    /// Image hash at activation (`AW2_JIT_PROFILE`): names the profile file, so it matches
    /// the build-time key even if the code is patched later.
    phash: Option<[u64; 2]>,
}

static MAPPED: Mutex<Vec<Mapped>> = Mutex::new(Vec::new());

/// A PE image was mapped, relocated and import-bound (loader).
pub fn module_mapped(name: &str, base: usize, size: usize) {
    let _ = super::exec_mem::bind_static_aot_module(name, base, size);
    if let Ok(mut v) = MAPPED.lock() {
        v.push(Mapped { name: name.to_ascii_lowercase(), base, size, active: false, phash: None });
    }
}

/// Loader activation point (before DllMains run, after the AW2 registry is applied):
/// build caches (`AW2_DLL_AOT_BUILD`, then exit) or load them (`AW2_DLL_AOT=1|2`; with 2,
/// missing / outgrown caches are rebuilt in the background) for every image mapped since
/// the last call.
pub fn activate() {
    crate::pc_sample::start(resolve_host_pc, resolve_guest_va);
    crate::diag::install_boundary_hook();
    let mut hashes: Vec<Option<[u64; 2]>> = Vec::new();
    let pending: Vec<(String, usize, usize)> = match MAPPED.lock() {
        Ok(mut v) => v
            .iter_mut()
            .filter(|m| !m.active)
            .map(|m| {
                m.active = true;
                if jit_writable() && profile_enabled() {
                    m.phash = Some(unsafe { image_hash(m.base, m.size) });
                }
                hashes.push(m.phash);
                (m.name.clone(), m.base, m.size)
            })
            .collect(),
        Err(_) => return,
    };
    if pending.is_empty() {
        return;
    }
    if let Some(sel) = build_request() {
        unsafe { build_all(&pending, &sel) };
        log::info!("emu aot: AW2_DLL_AOT_BUILD done; exiting");
        std::process::exit(0);
    }
    if aot_mode() == 0 || !jit_writable() {
        return;
    }
    ACTIVATED.get_or_init(std::time::Instant::now);
    if aot_mode() == 2 {
        sweep_start();
    }
    let mut jobs = Vec::new();
    for ((name, base, size), h) in pending.iter().zip(hashes) {
        let (base, size) = (*base, *size);
        if crate::exe::aw2_registry_active() && base == crate::exe::base() {
            // Every AlanWake2.exe function is a native replacement, and the emulator routes the
            // exe import thunks / special-ABI helpers natively (`emu::exe_route`). So the exe
            // gets no translation cache: older caches only hold import-thunk blocks, and the
            // shared probe table would otherwise run them from translated DLL code.
            log::info!("emu aot: {name}: skipped (AW2 exe: native replacements, import thunks routed by the emulator)");
            continue;
        }
        let key = unsafe { module_key_hashed(base, size, h) };
        let loaded = unsafe { load_module(name, base, size, &key) };
        if aot_mode() == 2 {
            jobs.extend(bg_job(name, base, size, key, loaded.as_deref()));
        }
    }
    if !jobs.is_empty() {
        bg_enqueue(jobs);
    }
}

/// Diagnostics ([`crate::pc_sample`], [`crate::diag`]): the mapped PE image (name, base)
/// containing guest address `va`. Never blocks.
pub(crate) fn resolve_guest_va(va: usize) -> Option<(String, usize)> {
    let v = MAPPED.try_lock().ok()?;
    v.iter().find(|m| va >= m.base && va < m.base + m.size).map(|m| (m.name.clone(), m.base))
}

/// Diagnostics ([`crate::pc_sample`]): the module, image base and guest block IP whose live
/// AOT translation contains host address `pc` (`ip` 0: the region's shared stubs or padding).
/// Never blocks (sampled PCs may belong to a thread that holds [`DIR`]).
fn resolve_host_pc(pc: usize) -> Option<(String, usize, u64)> {
    let g = DIR.try_read().ok()?;
    let d = g.as_ref()?;
    for m in &d.modules {
        if m.gone || pc < m.region || pc >= m.region + m.region_len {
            continue;
        }
        let Some(off) = m.img_off_of_pc(pc) else {
            return Some((m.name.clone(), m.base, 0));
        };
        let bs = &m.img.blocks;
        let i = bs.partition_point(|b| b.code_off as u64 <= off);
        let mut ip = match i.checked_sub(1).map(|j| &bs[j]) {
            Some(b) if off < b.code_off as u64 + b.main as u64 * 4 => b.ip,
            _ => 0,
        };
        if ip == 0 {
            let ci = bs.partition_point(|b| b.cold_off as u64 <= off);
            if let Some(b) = ci.checked_sub(1).map(|j| &bs[j]) {
                if b.words > b.main && off < b.cold_off as u64 + (b.words - b.main) as u64 * 4 {
                    ip = b.ip;
                }
            }
        }
        return Some((m.name.clone(), m.base, ip));
    }
    None
}

// ---------------------------------------------------------------------------------------------
// Profile
// ---------------------------------------------------------------------------------------------

static PROFILE: Mutex<Option<HashMap<u64, u32>>> = Mutex::new(None);

/// `JitCache` translated the image block at `ip` (`gen == 0`).
#[inline]
pub(super) fn profile_note(ip: u64) {
    if profile_enabled() {
        profile_add(ip);
    }
}

#[cold]
#[inline(never)]
fn profile_add(ip: u64) {
    static START: std::sync::Once = std::sync::Once::new();
    START.call_once(|| {
        win32_rt::process::register_exit_hook(profile_dump);
        let _ = std::thread::Builder::new().name("port-jit-profile".into()).spawn(|| loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
            profile_dump();
        });
    });
    if let Ok(mut p) = PROFILE.lock() {
        *p.get_or_insert_with(HashMap::new).entry(ip).or_insert(0) += 1;
    }
}

/// Profile counts are halved (rounding up: no block drops out) once their total exceeds
/// this, so they stay bounded across runs and recent runs keep their weight.
const PROFILE_HALVE_AT: u64 = 1 << 32;

/// Add `counts` to the merged profile `all`; halve it past [`PROFILE_HALVE_AT`] (true).
fn profile_merge(all: &mut HashMap<u32, u64>, counts: impl IntoIterator<Item = (u32, u64)>) -> bool {
    for (rva, n) in counts {
        let e = all.entry(rva).or_insert(0);
        *e = e.saturating_add(n);
    }
    let total = all.values().fold(0u64, |a, &n| a.saturating_add(n));
    if total <= PROFILE_HALVE_AT {
        return false;
    }
    for n in all.values_mut() {
        *n = n.div_ceil(2);
    }
    true
}

fn read_tsv(path: &Path) -> HashMap<u32, u64> {
    let mut m = HashMap::new();
    if let Ok(s) = std::fs::read_to_string(path) {
        for line in s.lines() {
            let mut it = line.split('\t');
            let (Some(a), Some(b)) = (it.next(), it.next()) else { continue };
            let (Ok(rva), Ok(n)) = (u32::from_str_radix(a.trim().trim_start_matches("0x"), 16), b.trim().parse::<u64>()) else { continue };
            *m.entry(rva).or_insert(0) += n;
        }
    }
    m
}

/// Merge the profile recorded since the last dump into
/// `<cache>/profile/<module>-<imagehash>.tsv` (one file per mapped image). Exit hook and
/// periodic; never blocks on a lock held elsewhere (crash paths).
pub fn profile_dump() {
    let taken = match PROFILE.try_lock() {
        Ok(mut g) => g.take(),
        Err(_) => None,
    };
    let Some(p) = taken else { return };
    let mods: Vec<(String, usize, usize, Option<[u64; 2]>)> = match MAPPED.try_lock() {
        Ok(v) => v.iter().map(|m| (m.name.clone(), m.base, m.size, m.phash)).collect(),
        Err(_) => return,
    };
    let mut per: HashMap<usize, HashMap<u32, u64>> = HashMap::new();
    for (ip, n) in p {
        let a = ip as usize;
        if let Some(i) = mods.iter().position(|m| a >= m.1 && a - m.1 < m.2) {
            *per.entry(i).or_default().entry((a - mods[i].1) as u32).or_insert(0) += n as u64;
        }
    }
    for (i, counts) in per {
        let (name, base, size, phash) = &mods[i];
        let path = profile_file(name, &phash.unwrap_or_else(|| unsafe { image_hash(*base, *size) }));
        let mut all = read_tsv(&path);
        profile_merge(&mut all, counts);
        let mut v: Vec<(u32, u64)> = all.into_iter().collect();
        v.sort_unstable_by_key(|&(rva, n)| (core::cmp::Reverse(n), rva));
        let mut s = String::with_capacity(v.len() * 16);
        for (rva, n) in &v {
            s += &format!("{rva:#x}\t{n}\n");
        }
        match write_atomic(&path, s.as_bytes()) {
            Ok(()) => log::info!("emu aot: profile {name}: {} blocks -> {}", v.len(), path.display()),
            Err(e) => log::warn!("emu aot: profile {name}: {}: {e}", path.display()),
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Offline Differential Verification Helpers (`emu::verify`)
// ---------------------------------------------------------------------------------------------

/// Discover all PE roots and reachable basic block IPs in a mapped PE image `[base, base+size)`.
pub(super) unsafe fn discover_blocks_for_verify(
    base: usize,
    size: usize,
    max_roots: usize,
    depth: u32,
    threads: usize,
) -> (Vec<u32>, Vec<u64>) {
    let roots = discover_pe_roots(base, size, max_roots);
    if roots.is_empty() {
        return (roots, Vec::new());
    }
    let mut pool = WorkerCaches::new();
    let (ips, _cl) = discover_with_threads(&mut pool, base, size, &roots, depth, threads.max(1));
    (roots, ips)
}

/// Self-contained in-memory AOT region for offline differential verification of real PE blocks
/// (exercises `build_image_with_threads`, `ChunkAssembler`, `split_and_rebase_block`,
/// `aot_emit_stubs`, and intra-batch `link_pass` including fallthrough-`nop` and `cond_patch`).
pub(super) struct VerifyAotBatch {
    region: usize,
    region_len: usize,
    bank_shifts: Vec<usize>,
    img: Image,
    /// `ip -> (bank_enter_stub, block_code_addr, block_idx)`
    blocks: HashMap<u64, (usize, usize, usize)>,
}

impl VerifyAotBatch {
    pub(super) unsafe fn build(ips: &[u64], threads: usize) -> Option<Self> {
        if ips.is_empty() || !jit_writable() {
            return None;
        }
        let mut sorted = ips.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        let mut pool = WorkerCaches::new();
        let build_threads = if sorted.len() >= PARALLEL_MIN_BLOCKS { threads.max(1) } else { 1 };
        let img = build_image_with_threads(&mut pool, &sorted, BUILD_VBASE, build_threads)?;
        let banks_snap: Vec<(usize, usize)> = img
            .stub_banks()
            .iter()
            .enumerate()
            .map(|(bi, &bo)| (bo as usize, img.bank_end(bi)))
            .collect();
        let nbanks = banks_snap.len();
        let mut bank_shifts = Vec::with_capacity(nbanks);
        let mut bank_rt_offs = Vec::with_capacity(nbanks);
        let mut cursor = 0usize;
        for &(bo, b_end) in &banks_snap {
            cursor = (cursor + 15) & !15;
            bank_rt_offs.push(cursor);
            bank_shifts.push(cursor - bo);
            cursor += b_end - bo;
        }
        let len = (cursor + 0x3FFF) & !0x3FFF;
        let region = alloc_jit(len)? as usize;
        {
            let _w = JitWrite::new();
            for bi in 0..nbanks {
                let rt_base = region + bank_rt_offs[bi];
                if jit::aot_emit_stubs(rt_base as *mut u32, rt_base) != Some(img.stubs) {
                    drop(_w);
                    super::exec_mem::free_jit(region as *mut u8, len);
                    return None;
                }
                let code_start_off = banks_snap[bi].0 + AOT_STUBS_BYTES;
                let code_end_off = banks_snap[bi].1;
                if code_end_off > code_start_off {
                    let word_lo = (code_start_off - AOT_STUBS_BYTES) / 4;
                    let word_len = (code_end_off - code_start_off) / 4;
                    core::ptr::copy_nonoverlapping(
                        img.code.as_ptr().add(word_lo),
                        (rt_base + AOT_STUBS_BYTES) as *mut u32,
                        word_len,
                    );
                }
            }
        }
        icache_invalidate(region as *mut u8, len);
        let mut blocks = HashMap::with_capacity(img.blocks.len());
        for (bi, b) in img.blocks.iter().enumerate() {
            let bank_idx = img.bank_index_of(b.code_off);
            let shift = bank_shifts[bank_idx];
            let bank_rt_base = region + bank_rt_offs[bank_idx];
            let enter = bank_rt_base + img.stubs[0] as usize * 4;
            let code = region + b.code_off as usize + shift;
            blocks.insert(b.ip, (enter, code, bi));
        }
        Some(Self { region, region_len: len, bank_shifts, img, blocks })
    }

    #[inline]
    fn host_addr(&self, off: u32) -> usize {
        let shift = if self.bank_shifts.len() <= 1 {
            0
        } else {
            self.bank_shifts[self.img.bank_index_of(off)]
        };
        self.region + off as usize + shift
    }

    #[inline]
    pub(super) fn block_entry(&self, ip: u64) -> Option<(usize, usize)> {
        self.blocks.get(&ip).map(|&(enter, code, _)| (enter, code))
    }

    #[inline]
    pub(super) fn contains(&self, ip: u64) -> bool {
        self.blocks.contains_key(&ip)
    }

    /// Apply intra-batch `link_pass` (direct `b`/`bl`, fallthrough `nop`, and `try_patch_cond_branch`).
    /// Returns `(total_linked, fallthrough_nops, cond_branches_patched)`.
    pub(super) unsafe fn apply_intra_links(&mut self) -> (usize, usize, usize) {
        let mut total = 0usize;
        let mut nops = 0usize;
        let mut conds = 0usize;
        {
            let _w = JitWrite::new();
            for b in &self.img.blocks {
                let lo = b.link_lo as usize;
                let hi = lo + b.link_n as usize;
                for li in lo..hi {
                    let l = self.img.links[li];
                    let Some(&(_, code, tb)) = self.blocks.get(&l.target) else {
                        continue;
                    };
                    if self.img.blocks[tb].host_entry {
                        continue;
                    }
                    let site = self.host_addr(l.site_off);
                    if !reach(site, code) {
                        continue;
                    }
                    let cell = &*(site as *const AtomicU32);
                    let old = cell.load(Ordering::Relaxed);
                    if code == site + 4 && (old & 0xFC00_0000) == 0x1400_0000 {
                        cell.store(0xD503_201F, Ordering::Relaxed);
                        nops += 1;
                    } else {
                        store_b(site, code);
                    }
                    if l.cond_off != u32::MAX {
                        let cond_site = self.host_addr(l.cond_off);
                        if try_patch_cond_branch(cond_site, code).is_some() {
                            conds += 1;
                        }
                    }
                    total += 1;
                }
            }
        }
        icache_invalidate(self.region as *mut u8, self.region_len);
        (total, nops, conds)
    }
}

impl Drop for VerifyAotBatch {
    fn drop(&mut self) {
        unsafe { super::exec_mem::free_jit(self.region as *mut u8, self.region_len) };
    }
}

// ---------------------------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------------------------

/// Test: translate the blocks at `ips` with this thread's knobs into an in-memory image,
/// run `mid` (e.g. change guest state between "build" and "load"), install it
/// (cross-checked like `AW2_DLL_AOT_VERIFY=1`) and turn AOT lookups on for this thread.
/// Returns `(live blocks, relocations, links)`.
#[cfg(test)]
pub(super) unsafe fn test_install(ips: &[u64], mid: impl FnOnce()) -> (usize, usize, usize) {
    TEST_AOT.with(|c| c.set(true));
    let mut ips = ips.to_vec();
    ips.sort_unstable();
    ips.dedup();
    let mut bc = Box::new(BlockCache::new());
    let bcp: *mut BlockCache = &mut *bc;
    let img = build_image(bcp, &ips, BUILD_VBASE).expect("aot stubs");
    // Round-trip through the file format.
    let (lo, hi) = (ips.first().copied().unwrap_or(0), ips.last().copied().unwrap_or(0) + 1);
    let key = AotKey { image_base: lo, image_size: hi - lo, ..AotKey::default() };
    let (name, key2, img, flags) = decode(&encode("test", &key, &img, 0)).expect("aot image decodes");
    assert!(name == "test" && key2 == key && flags == 0);
    let (relocs, links) = (img.relocs.len(), img.links.len());
    mid();
    let live = install_image("test", lo as usize, (hi - lo) as usize, img, Verify::All, true).map_or(0, |r| r.live);
    (live, relocs, links)
}

/// Test: revalidations run on this thread revive blocks (`on`, the default) or not.
#[cfg(test)]
pub(super) fn test_set_revive(on: bool) {
    TEST_NO_REVIVE.with(|c| c.set(!on));
}

/// Test: import-slot relocations rebased by installs on this thread so far.
#[cfg(test)]
pub(super) fn test_rebased() -> usize {
    TEST_REBASED.with(|c| c.get())
}

/// Test: blocks installed on this thread so far with a held import slot.
#[cfg(test)]
pub(super) fn test_held() -> usize {
    TEST_HELD.with(|c| c.get())
}

/// Test: dead blocks in this thread's installed test modules.
#[cfg(test)]
pub(super) fn test_dead() -> usize {
    let me = Some(std::thread::current().id());
    let g = DIR.read().unwrap_or_else(|e| e.into_inner());
    g.as_ref().map_or(0, |d| {
        d.modules.iter().filter(|m| m.owner == me && !m.gone).map(|m| m.dead.iter().filter(|x| x.load(Ordering::Relaxed) != 0).count()).sum()
    })
}

/// Test: the published probe table (what the stubs read) agrees with the directory for
/// every block of this thread's test modules: live non-host-entry owners present with their
/// code, everything else absent or 0. Returns the number of live entries checked.
#[cfg(test)]
pub(super) fn test_probe_check() -> Result<usize, String> {
    let me = Some(std::thread::current().id());
    let g = DIR.read().unwrap_or_else(|e| e.into_inner());
    let Some(d) = g.as_ref() else { return Ok(0) };
    let base = PROBE.load(Ordering::Acquire);
    if base != d.probe.base {
        return Err(format!("published table {base:#x} is not the directory's {:#x}", d.probe.base));
    }
    let mut live = 0;
    for m in d.modules.iter().filter(|m| m.owner == me && !m.gone) {
        for b in m.img.blocks.iter() {
            let want = d.probe_val(b.ip);
            let got = if base == 0 {
                0
            } else {
                let e = unsafe { d.probe.slot(b.ip) };
                if e[0].load(Ordering::Relaxed) == b.ip { e[1].load(Ordering::Relaxed) } else { 0 }
            };
            if got != want {
                return Err(format!("block {:#x}: probe table has {got:#x}, directory says {want:#x}", b.ip));
            }
            live += (want != 0) as usize;
        }
    }
    Ok(live)
}

/// Test: remove this thread's test modules (their code stays mapped) and turn AOT lookups
/// off for this thread.
#[cfg(test)]
pub(super) fn test_uninstall() {
    TEST_AOT.with(|c| c.set(false));
    let me = Some(std::thread::current().id());
    let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    let Some(d) = g.as_mut() else { return };
    let gone: Vec<u32> = (0..d.modules.len() as u32).filter(|&i| d.modules[i as usize].owner == me && !d.modules[i as usize].gone).collect();
    for &i in &gone {
        d.modules[i as usize].gone = true;
    }
    let ips: Vec<u64> = d.map.iter().filter(|(_, &(m, _))| gone.contains(&m)).map(|(&ip, _)| ip).collect();
    d.map.retain(|_, &mut (m, _)| !gone.contains(&m));
    for ip in ips {
        unsafe { d.probe_sync(ip) };
    }
    for v in d.incoming.values_mut() {
        v.retain(|&(m, _)| !gone.contains(&m));
    }
}

/// `.pdata` map: chained (`UNW_FLAG_CHAININFO`, odd unwind-code count padding, two hops)
/// and indirect entries merge into their root; cyclic / out-of-image chains stay their own
/// root; gaps (leaf code) have no function.
#[test]
fn funcmap_merges_chained_unwind_entries() {
    let mut img = vec![0u8; 0x2000];
    let w16 = |img: &mut Vec<u8>, o: usize, v: u16| img[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |img: &mut Vec<u8>, o: usize, v: u32| img[o..o + 4].copy_from_slice(&v.to_le_bytes());
    w16(&mut img, 0, 0x5A4D);
    w32(&mut img, 0x3C, 0x40);
    w32(&mut img, 0x40, 0x4550);
    w16(&mut img, 0x44, 0x8664);
    w16(&mut img, 0x54, 240);
    let opt = 0x58;
    w16(&mut img, opt, 0x20B);
    w32(&mut img, opt + 108, 16);
    let rf = |img: &mut Vec<u8>, o: usize, b: u32, e: u32, u: u32| {
        w32(img, o, b);
        w32(img, o + 4, e);
        w32(img, o + 8, u);
    };
    let pdata = [
        (0x100, 0x180, 0x1800),         // root
        (0x400, 0x420, 0x1810),         // chained -> 0x100
        (0x180, 0x1A0, 0x1820),         // chained (1 code) -> 0x400 -> 0x100
        (0x200, 0x240, 0x1000 | 1),     // indirect -> entry 0 (0x100)
        (0x300, 0x340, 0x1840),         // cyclic chain
        (0x500, 0x520, 0x00FF_FFF0),    // unwind info out of the image
        (0x600, 0x640, 0x1850),         // root (EH handler flag only)
    ];
    for (i, &(b, e, u)) in pdata.iter().enumerate() {
        rf(&mut img, 0x1000 + 12 * i, b, e, u);
    }
    w32(&mut img, opt + 112 + 3 * 8, 0x1000);
    w32(&mut img, opt + 112 + 3 * 8 + 4, 12 * pdata.len() as u32);
    img[0x1800] = 0x01;
    img[0x1810] = 0x01 | 4 << 3;
    rf(&mut img, 0x1814, 0x100, 0x180, 0x1800);
    img[0x1820] = 0x01 | 4 << 3;
    img[0x1822] = 1;
    rf(&mut img, 0x1828, 0x400, 0x420, 0x1810);
    img[0x1840] = 0x01 | 4 << 3;
    rf(&mut img, 0x1844, 0x300, 0x340, 0x1840);
    img[0x1850] = 0x01 | 1 << 3;
    let fm = unsafe { FuncMap::read(img.as_ptr() as usize, img.len()) };
    assert_eq!((fm.ranges.len(), fm.chained, fm.broken, fm.functions()), (7, 3, 2, 4));
    for (rva, want) in [
        (0x100, Some(0x100)),
        (0x17F, Some(0x100)),
        (0x180, Some(0x100)),
        (0x410, Some(0x100)),
        (0x210, Some(0x100)),
        (0x310, Some(0x300)),
        (0x510, Some(0x500)),
        (0x610, Some(0x600)),
        (0x260, None),
        (0x420, None),
        (0x50, None),
    ] {
        assert_eq!(fm.root_of(rva), want, "rva {rva:#x}");
    }
}

/// Fresh scratch directory for the cache-maintenance tests.
#[cfg(test)]
fn test_dir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("aw2-aot-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// Crash-loop guard: a run whose lock survives (no clean exit) is a strike; the second
/// strike in a row skips AOT for one run (no lock) and starts over; live processes' locks
/// are kept (concurrent instances); a lock with the starting pid is stale (pid reuse).
#[test]
fn aot_session_guard_skips_one_run_after_two_unclean_exits() {
    let d = test_dir("session");
    let lock = |p: u32| session_lock(&d, p).exists();
    let dead = |_: u32| false;
    assert_eq!(session_check(&d, 100, dead), Session::Go { strikes: 0, stale: 0 });
    assert!(lock(100));
    // 100 crashed (lock left): first strike.
    assert_eq!(session_check(&d, 101, dead), Session::Go { strikes: 1, stale: 1 });
    assert!(!lock(100) && lock(101));
    // 101 crashed too: this run goes without AOT and creates no lock.
    assert_eq!(session_check(&d, 102, dead), Session::Skip { stale: 1 });
    assert!(!lock(101) && !lock(102));
    // The run after the skipped one starts over.
    assert_eq!(session_check(&d, 103, dead), Session::Go { strikes: 0, stale: 0 });
    // 103 exits cleanly (session_end removes its lock): no strike.
    std::fs::remove_file(session_lock(&d, 103)).unwrap();
    assert_eq!(session_check(&d, 104, dead), Session::Go { strikes: 0, stale: 0 });
    // 104 still running: a concurrent start leaves its lock alone.
    assert_eq!(session_check(&d, 105, |p| p == 104), Session::Go { strikes: 0, stale: 0 });
    assert!(lock(104) && lock(105));
    // Both crash; a later start whose pid was reused from one of them counts both.
    assert_eq!(session_check(&d, 105, dead), Session::Go { strikes: 1, stale: 2 });
    assert!(lock(105) && !lock(104));
    // Unrelated files are ignored.
    std::fs::write(d.join("session-x.lock"), "").unwrap();
    std::fs::remove_file(session_lock(&d, 105)).unwrap();
    assert_eq!(session_check(&d, 106, dead), Session::Go { strikes: 0, stale: 0 });
    assert!(d.join("session-x.lock").exists());
    assert!(clean_exit_code(0) && clean_exit_code(1) && !clean_exit_code(3) && !clean_exit_code(255));
    assert!(!clean_exit_code(0xC000_0005) && !clean_exit_code(0x8000_0003));
    assert!(pid_alive(std::process::id()) && !pid_alive(0));
    let _ = std::fs::remove_dir_all(&d);
}

/// Sweep: files unused for longer than the age limit go first, then the least recently
/// used until the rest fits the cap; only cache files (`*.aot`, `profile/*.tsv`, leftover
/// `*.tmp<pid>`) are candidates.
#[test]
fn aot_evict_removes_old_then_least_recently_used_over_cap() {
    let d = test_dir("evict");
    std::fs::create_dir_all(d.join("profile")).unwrap();
    let now = std::time::SystemTime::now();
    let day = std::time::Duration::from_secs(86400);
    let put = |rel: &str, len: usize, age: std::time::Duration| {
        let p = d.join(rel);
        std::fs::write(&p, vec![0u8; len]).unwrap();
        std::fs::File::options().write(true).open(&p).unwrap().set_modified(now - age).unwrap();
    };
    put("a.dll-0000000000000001.aot", 100, day * 40);
    put("b.dll-0000000000000002.aot", 200, day * 2);
    put("c.dll-0000000000000003.aot", 300, day);
    put("profile/c.dll-0000000000000003.tsv", 50, day / 24);
    put("c.dll-0000000000000004.tmp4242", 10, day * 3);
    put("session-1.lock", 5, day * 90);
    put("session.strikes", 5, day * 90);
    put("notes.txt", 5, day * 90);
    let r = evict(&d, now, day * 30, 400);
    // a: too old; then over the cap (560): the tmp file (3 days), b (2 days) -> 350 bytes.
    assert_eq!(r, [3, 310, 2, 350]);
    for (rel, there) in [
        ("a.dll-0000000000000001.aot", false),
        ("b.dll-0000000000000002.aot", false),
        ("c.dll-0000000000000004.tmp4242", false),
        ("c.dll-0000000000000003.aot", true),
        ("profile/c.dll-0000000000000003.tsv", true),
        ("session-1.lock", true),
        ("session.strikes", true),
        ("notes.txt", true),
    ] {
        assert_eq!(d.join(rel).exists(), there, "{rel}");
    }
    // Within limits: nothing to do.
    assert_eq!(evict(&d, now, day * 30, 400), [0, 0, 2, 350]);
    // touch() makes a file the most recently used.
    touch(&d.join("c.dll-0000000000000003.aot"));
    assert_eq!(evict(&d, now + std::time::Duration::from_secs(1), day * 30, 300), [1, 50, 1, 300]);
    assert!(d.join("c.dll-0000000000000003.aot").exists());
    let _ = std::fs::remove_dir_all(&d);
}

/// After a write, only the same module's caches with other keys are removed.
#[test]
fn aot_delete_other_keys_only_touches_the_module() {
    let d = test_dir("keys");
    let names = [
        "m.dll-0123456789abcdef.aot",
        "m.dll-fedcba9876543210.aot",
        "m.dll-0123.aot",
        "m.dll-0123456789abcdeg.aot",
        "m.dll.x-0123456789abcdef.aot",
        "n.dll-0123456789abcdef.aot",
        "m.dll-0123456789abcdef.tsv",
    ];
    for n in names {
        std::fs::write(d.join(n), "x").unwrap();
    }
    let keep = d.join("m.dll-fedcba9876543210.aot");
    assert_eq!(delete_other_keys(&d, "m.dll", &keep), 1);
    for n in names {
        assert_eq!(d.join(n).exists(), n != "m.dll-0123456789abcdef.aot", "{n}");
    }
    assert_eq!(delete_other_keys(&d, "m.dll", &keep), 0);
    let _ = std::fs::remove_dir_all(&d);
}

/// Header flags round-trip and can be rewritten in place (checksum follows); the
/// first-load verify selection is full below the block limit, a ~2% sample above it.
#[test]
fn aot_verify_pending_flag_and_first_load_selection() {
    let key = AotKey { image_base: 0x1_4000_0000, image_size: 0x1000, codegen_hash: 7, ..AotKey::default() };
    let mut b = encode("m.dll", &key, &Image::default(), FLAG_VERIFY_PENDING);
    let (n, k, _, f) = decode(&b).expect("decodes");
    assert!(n == "m.dll" && k == key && f == FLAG_VERIFY_PENDING);
    assert!(set_flags(&mut b, 0));
    let (n, k, _, f) = decode(&b).expect("decodes after the flag rewrite");
    assert!(n == "m.dll" && k == key && f == 0);
    b[20] ^= 1;
    assert!(decode(&b).is_none(), "checksum must cover the header");
    assert!(!set_flags(&mut [0u8; 8], 0));
    assert_eq!(load_verify(false, false, 10, 1), Verify::None);
    assert_eq!(load_verify(true, false, 1 << 20, 1), Verify::All);
    assert_eq!(load_verify(false, true, FIRST_LOAD_FULL_MAX - 1, 1), Verify::All);
    let s = load_verify(false, true, FIRST_LOAD_FULL_MAX, 1);
    assert_eq!(s, Verify::Sample { pct: FIRST_LOAD_SAMPLE_PCT, seed: 1 });
    let picked = (0..100_000u64).filter(|&i| s.picks(0x1_4000_0000 + i * 16)).count();
    assert!((1_500..=2_500).contains(&picked), "{picked} of 100000 sampled");
    let other = Verify::Sample { pct: FIRST_LOAD_SAMPLE_PCT, seed: 2 };
    assert!((0..100_000u64).any(|i| s.picks(i * 16) != other.picks(i * 16)), "seed must change the sample");
}

/// Profile merge adds counts and halves them (rounding up, once per merge) past the cap.
#[test]
fn aot_profile_merge_halves_past_the_cap() {
    let mut all: HashMap<u32, u64> = [(1, 5), (2, 1)].into_iter().collect();
    assert!(!profile_merge(&mut all, [(2, 2), (3, 1)]));
    assert_eq!((all[&1], all[&2], all[&3]), (5, 3, 1));
    // Total 2^32 + 9: halved to 2^31 + 3, 2, 1.
    assert!(profile_merge(&mut all, [(1, 1 << 32)]));
    assert_eq!((all[&1], all[&2], all[&3]), ((1u64 << 31) + 3, 2, 1));
    assert!(!profile_merge(&mut all, []));
    // Still over the cap after one halving: halved again at the next merge.
    assert!(profile_merge(&mut all, [(2, 3 << 32)]));
    assert!(profile_merge(&mut all, []));
    assert!(!profile_merge(&mut all, []));
}

/// Profile-free offline prewarm on an in-memory synthetic PE32+ image:
/// discovers roots from `.pdata`, `AddressOfEntryPoint`, and Export Directory (skipping
/// forwarders and non-code data exports), expands static CFG closure, writes the `.aot`
/// cache file to `PORT_AOT_DIR`, and loads it back cleanly via `activate()`.
#[test]
fn aot_prewarm_synthetic_pe_writes_and_loads_via_activate() {
    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            struct EnvGuard {
                port_dir: Option<std::ffi::OsString>,
                aw2_dir: Option<std::ffi::OsString>,
            }
            impl Drop for EnvGuard {
                fn drop(&mut self) {
                    match &self.port_dir {
                        Some(v) => std::env::set_var("PORT_AOT_DIR", v),
                        None => std::env::remove_var("PORT_AOT_DIR"),
                    }
                    match &self.aw2_dir {
                        Some(v) => std::env::set_var("AW2_DLL_AOT_DIR", v),
                        None => std::env::remove_var("AW2_DLL_AOT_DIR"),
                    }
                }
            }
            let _env_guard = EnvGuard {
                port_dir: std::env::var_os("PORT_AOT_DIR"),
                aw2_dir: std::env::var_os("AW2_DLL_AOT_DIR"),
            };

            let d = test_dir("prewarm");
            std::env::set_var("PORT_AOT_DIR", &d);
            std::env::remove_var("AW2_DLL_AOT_DIR");

            // Synthetic PE32+ image (16 KiB):
            //   .text  @ RVA 0x1000..0x2000 (code + trailing padding for BlockCache::fill)
            //   .rdata @ RVA 0x2000..0x3000 (Export Directory @ 0x2000, .pdata @ 0x2200)
            let mut img = vec![0u8; 0x4000];
            let w16 = |buf: &mut [u8], o: usize, v: u16| buf[o..o + 2].copy_from_slice(&v.to_le_bytes());
            let w32 = |buf: &mut [u8], o: usize, v: u32| buf[o..o + 4].copy_from_slice(&v.to_le_bytes());

            w16(&mut img, 0, 0x5A4D); // MZ
            w32(&mut img, 0x3C, 0x40); // e_lfanew
            w32(&mut img, 0x40, 0x0000_4550); // PE\0\0
            w16(&mut img, 0x44, 0x8664); // AMD64
            w16(&mut img, 0x46, 2); // NumberOfSections
            w16(&mut img, 0x54, 240); // SizeOfOptionalHeader

            let opt = 0x58;
            w16(&mut img, opt, 0x20B); // PE32+
            w32(&mut img, opt + 16, 0x1000); // AddressOfEntryPoint
            w32(&mut img, opt + 56, 0x4000); // SizeOfImage
            w32(&mut img, opt + 60, 0x400); // SizeOfHeaders
            w32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes
            // DataDirectory[0]: Export Directory @ 0x2000 (size 0x80)
            w32(&mut img, opt + 112, 0x2000);
            w32(&mut img, opt + 112 + 4, 0x80);
            // DataDirectory[3]: Exception Directory (.pdata) @ 0x2200 (2 entries * 12 = 24 bytes)
            w32(&mut img, opt + 112 + 3 * 8, 0x2200);
            w32(&mut img, opt + 112 + 3 * 8 + 4, 24);

            // Section table at opt + 240 = 0x148
            let sec0 = opt + 240;
            img[sec0..sec0 + 5].copy_from_slice(b".text");
            w32(&mut img, sec0 + 8, 0x1000); // VirtualSize
            w32(&mut img, sec0 + 12, 0x1000); // VirtualAddress
            w32(&mut img, sec0 + 36, 0x6000_0020); // MEM_EXECUTE | MEM_READ | CNT_CODE

            let sec1 = sec0 + 40;
            img[sec1..sec1 + 6].copy_from_slice(b".rdata");
            w32(&mut img, sec1 + 8, 0x1000);
            w32(&mut img, sec1 + 12, 0x2000);
            w32(&mut img, sec1 + 36, 0x4000_0040); // MEM_READ | CNT_INITIALIZED_DATA

            // Code in .text:
            //   0x1000 (EntryPoint + .pdata[0]): test ecx, ecx; jz +6 (0x100a); mov eax, 1; ret; mov eax, 2; ret
            let fn0: &[u8] = &[0x85, 0xC9, 0x74, 0x06, 0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3, 0xB8, 0x02, 0x00, 0x00, 0x00, 0xC3];
            img[0x1000..0x1000 + fn0.len()].copy_from_slice(fn0);
            //   0x1040 (Export Directory ONLY): lea eax, [rcx + rdx]; ret
            let fn1: &[u8] = &[0x8D, 0x04, 0x11, 0xC3];
            img[0x1040..0x1040 + fn1.len()].copy_from_slice(fn1);
            //   0x1080 (.pdata[1] ONLY): add rcx, 7; mov rax, rcx; ret
            let fn2: &[u8] = &[0x48, 0x83, 0xC1, 0x07, 0x48, 0x89, 0xC8, 0xC3];
            img[0x1080..0x1080 + fn2.len()].copy_from_slice(fn2);

            // Export Directory @ 0x2000:
            w32(&mut img, 0x2000 + 20, 3); // NumberOfFunctions
            w32(&mut img, 0x2000 + 28, 0x2030); // AddressOfFunctions
            w32(&mut img, 0x2030, 0x1040); // exported code function
            w32(&mut img, 0x2034, 0x2060); // forwarder inside export directory (must be skipped)
            w32(&mut img, 0x2038, 0x2400); // data export in .rdata (must be skipped)

            // .pdata @ 0x2200:
            w32(&mut img, 0x2200, 0x1000);
            w32(&mut img, 0x2204, 0x1020);
            w32(&mut img, 0x2208, 0x2300);
            w32(&mut img, 0x220C, 0x1080);
            w32(&mut img, 0x2210, 0x10A0);
            w32(&mut img, 0x2214, 0x2300);
            img[0x2300] = 0x01; // UNWIND_INFO version 1, flags 0

            let base = img.as_ptr() as usize;
            let size = img.len();
            assert!((0x1_0000..0x8000_0000_0000).contains(&base), "unexpected test buffer address {base:#x}");

            // Root discovery covers .pdata + EntryPoint + Exports (excluding forwarders & data)
            // and respects deterministic max_roots capping.
            assert_eq!(unsafe { discover_pe_roots(base, size, 0) }, vec![0x1000, 0x1040, 0x1080]);
            assert_eq!(unsafe { discover_pe_roots(base, size, 2) }, vec![0x1000, 0x1040]);

            let mod_name = "synth_prewarm_test.dll";
            module_mapped(mod_name, base, size);

            let reports = aot_prewarm_all_mapped(0);
            let rep = reports
                .iter()
                .find(|r| r.module == mod_name)
                .expect("synthetic module should appear in prewarm report");
            assert_eq!(rep.roots, 3, "expected 3 unique code roots");
            assert_eq!(rep.blocks, 5, "expected 5 translated blocks (3 roots + 2 CFG successors)");
            assert!(rep.code_bytes > 0);
            assert!(rep.skipped_reason.is_none(), "unexpected skip: {:?}", rep.skipped_reason);
            let cache_path = rep.cache_path.as_ref().expect("cache file path");
            assert!(cache_path.is_file(), "expected .aot cache file at {}", cache_path.display());
            assert!(cache_path.starts_with(&d), "cache file should live inside PORT_AOT_DIR");

            // Load back via activate() and confirm all 5 blocks are live in the AOT directory.
            TEST_AOT.with(|c| c.set(true));
            activate();
            for rva in [0x1000u64, 0x1004, 0x100a, 0x1040, 0x1080] {
                assert!(
                    find(base as u64 + rva).is_some(),
                    "prewarmed block at RVA {rva:#x} was not loaded by activate()"
                );
            }

            // Tear down global test registrations so the synthetic buffer can be dropped safely.
            TEST_AOT.with(|c| c.set(false));
            if let Ok(mut v) = MAPPED.lock() {
                v.retain(|m| m.name != mod_name);
            }
            {
                let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
                let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
                if let Some(dir) = g.as_mut() {
                    let gone: Vec<u32> = (0..dir.modules.len() as u32)
                        .filter(|&i| dir.modules[i as usize].name == mod_name && !dir.modules[i as usize].gone)
                        .collect();
                    for &i in &gone {
                        dir.modules[i as usize].gone = true;
                    }
                    let ips: Vec<u64> = dir.map.iter().filter(|(_, &(m, _))| gone.contains(&m)).map(|(&ip, _)| ip).collect();
                    dir.map.retain(|_, &mut (m, _)| !gone.contains(&m));
                    for ip in ips {
                        unsafe { dir.probe_sync(ip) };
                    }
                    for v in dir.incoming.values_mut() {
                        v.retain(|&(m, _)| !gone.contains(&m));
                    }
                }
            }
            let _ = std::fs::remove_dir_all(&d);
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Default AOT cache directory precedence: `PORT_AOT_DIR` > `AW2_DLL_AOT_DIR` >
/// `~/Library/Caches/port-factory/aot`, with legacy `~/Library/Caches/aw2-port/aot` fallback
/// only when neither override is set.
#[test]
fn aot_cache_dir_env_precedence_and_legacy_fallback() {
    struct EnvGuard {
        port_dir: Option<std::ffi::OsString>,
        aw2_dir: Option<std::ffi::OsString>,
    }
    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.port_dir {
                Some(v) => std::env::set_var("PORT_AOT_DIR", v),
                None => std::env::remove_var("PORT_AOT_DIR"),
            }
            match &self.aw2_dir {
                Some(v) => std::env::set_var("AW2_DLL_AOT_DIR", v),
                None => std::env::remove_var("AW2_DLL_AOT_DIR"),
            }
        }
    }
    let _guard = EnvGuard {
        port_dir: std::env::var_os("PORT_AOT_DIR"),
        aw2_dir: std::env::var_os("AW2_DLL_AOT_DIR"),
    };

    std::env::remove_var("PORT_AOT_DIR");
    std::env::remove_var("AW2_DLL_AOT_DIR");
    assert!(cache_dir().ends_with("Library/Caches/port-factory/aot"));
    assert!(legacy_cache_dir().is_some_and(|p| p.ends_with("Library/Caches/aw2-port/aot")));

    std::env::set_var("AW2_DLL_AOT_DIR", "/tmp/legacy-aot-override");
    assert_eq!(cache_dir(), PathBuf::from("/tmp/legacy-aot-override"));
    assert!(legacy_cache_dir().is_none());

    std::env::set_var("PORT_AOT_DIR", "/tmp/port-aot-override");
    assert_eq!(cache_dir(), PathBuf::from("/tmp/port-aot-override"));
    assert!(legacy_cache_dir().is_none());

    // Verify graceful read fallback from primary dir to legacy dir.
    let root = test_dir("legacy-fallback");
    let primary = root.join("port-factory-aot");
    let legacy = root.join("aw2-port-aot");
    std::fs::create_dir_all(&primary).unwrap();
    std::fs::create_dir_all(&legacy).unwrap();
    let key = AotKey { image_base: 0x1_8000_0000, image_size: 0x2000, codegen_hash: 42, ..AotKey::default() };
    let fname = format!("sample.dll-{:016x}.aot", key.id());
    std::fs::write(legacy.join(&fname), b"legacy-payload").unwrap();

    let (hit_path, hit) = read_cache_file_in(&primary, std::slice::from_ref(&legacy), "sample.dll", &key);
    assert_eq!(hit_path, legacy.join(&fname));
    assert_eq!(hit.as_ref().map(|(b, fb)| (b.as_slice(), *fb)), Some((&b"legacy-payload"[..], true)));

    // Primary takes precedence once present.
    std::fs::write(primary.join(&fname), b"primary-payload").unwrap();
    let (hit_path, hit) = read_cache_file_in(&primary, std::slice::from_ref(&legacy), "sample.dll", &key);
    assert_eq!(hit_path, primary.join(&fname));
    assert_eq!(hit.as_ref().map(|(b, fb)| (b.as_slice(), *fb)), Some((&b"primary-payload"[..], false)));

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn aot_read_cache_file_loads_from_readonly_fallback_without_writing() {
    use std::os::unix::fs::PermissionsExt;

    struct EnvGuard {
        port_dir: Option<std::ffi::OsString>,
        aw2_dir: Option<std::ffi::OsString>,
        fb_dir: Option<std::ffi::OsString>,
    }
    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (k, v) in [
                ("PORT_AOT_DIR", &self.port_dir),
                ("AW2_DLL_AOT_DIR", &self.aw2_dir),
                ("PORT_AOT_FALLBACK_DIR", &self.fb_dir),
            ] {
                match v {
                    Some(val) => std::env::set_var(k, val),
                    None => std::env::remove_var(k),
                }
            }
        }
    }
    struct PermsRestore(PathBuf, PathBuf);
    impl Drop for PermsRestore {
        fn drop(&mut self) {
            let _ = std::fs::set_permissions(&self.0, std::fs::Permissions::from_mode(0o755));
            let _ = std::fs::set_permissions(&self.1, std::fs::Permissions::from_mode(0o644));
        }
    }

    let root = test_dir("readonly-fallback");
    let primary = root.join("writable-user");
    let fallback = root.join("readonly-bundle");
    std::fs::create_dir_all(&primary).unwrap();
    std::fs::create_dir_all(&fallback).unwrap();

    let key = AotKey { image_base: 0x1_9000_0000, image_size: 0x1000, codegen_hash: 99, ..AotKey::default() };
    let fname = format!("bundled.dll-{:016x}.aot", key.id());
    let fb_file = fallback.join(&fname);
    std::fs::write(&fb_file, b"bundled-aot-bytes").unwrap();
    let mtime_before = std::fs::metadata(&fb_file).unwrap().modified().unwrap();

    // Lock the fallback file + directory read-only so any accidental write/touch fails visibly.
    std::fs::set_permissions(&fb_file, std::fs::Permissions::from_mode(0o444)).unwrap();
    std::fs::set_permissions(&fallback, std::fs::Permissions::from_mode(0o555)).unwrap();
    let _restore = PermsRestore(fallback.clone(), fb_file.clone());

    let _env = EnvGuard {
        port_dir: std::env::var_os("PORT_AOT_DIR"),
        aw2_dir: std::env::var_os("AW2_DLL_AOT_DIR"),
        fb_dir: std::env::var_os("PORT_AOT_FALLBACK_DIR"),
    };
    std::env::set_var("PORT_AOT_DIR", &primary);
    std::env::remove_var("AW2_DLL_AOT_DIR");
    std::env::set_var("PORT_AOT_FALLBACK_DIR", &fallback);

    let (hit_path, hit) = read_cache_file("bundled.dll", &key);
    assert_eq!(hit_path, fb_file);
    assert_eq!(hit.as_ref().map(|(b, fb)| (b.as_slice(), *fb)), Some((&b"bundled-aot-bytes"[..], true)));

    // Verify zero mutation inside the read-only fallback directory.
    let entries: Vec<_> = std::fs::read_dir(&fallback).unwrap().filter_map(|e| e.ok()).collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(std::fs::metadata(&fb_file).unwrap().modified().unwrap(), mtime_before);

    drop(_env);
    drop(_restore);
    let _ = std::fs::remove_dir_all(&root);
}

// ---------------------------------------------------------------------------------------------
// Helpers & tests for parallel prewarm, jump-table CFG discovery, and multi-module verification
// ---------------------------------------------------------------------------------------------

#[cfg(test)]
struct TestEnvGuard {
    saved: Vec<(&'static str, Option<std::ffi::OsString>)>,
}

#[cfg(test)]
impl TestEnvGuard {
    fn new(vars: &[&'static str]) -> Self {
        let saved = vars.iter().map(|&k| (k, std::env::var_os(k))).collect();
        Self { saved }
    }
}

#[cfg(test)]
impl Drop for TestEnvGuard {
    fn drop(&mut self) {
        for (k, v) in &self.saved {
            match v {
                Some(val) => std::env::set_var(k, val),
                None => std::env::remove_var(k),
            }
        }
    }
}

#[cfg(test)]
struct TestMappedBuf {
    ptr: *mut u8,
    len: usize,
}

#[cfg(test)]
impl TestMappedBuf {
    /// Allocate an anonymous RW page-aligned buffer at a realistic 64-bit guest VA
    /// (`>= 0x1_0000_0000`, `< 1 << 47`) suitable for synthetic PE images and IAT slots.
    fn alloc(size: usize, hint: usize) -> Self {
        extern "C" {
            fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut u8;
            fn munmap(addr: *mut u8, len: usize) -> i32;
        }
        let _ = munmap;
        let len = (size + 0x3FFF) & !0x3FFF;
        // MAP_PRIVATE (0x2) | MAP_ANON (0x1000), PROT_READ | PROT_WRITE (3).
        let p = unsafe { mmap(hint as *mut u8, len, 3, 0x2 | 0x1000, -1, 0) };
        assert!(!p.is_null() && p as isize != -1, "test mmap({len:#x}) failed");
        let addr = p as usize;
        assert!(
            (0x1_0000_0000..1usize << 47).contains(&addr),
            "test mmap landed outside expected 64-bit user range: {addr:#x}"
        );
        Self { ptr: p, len }
    }

    fn base(&self) -> usize {
        self.ptr as usize
    }

    fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

#[cfg(test)]
impl Drop for TestMappedBuf {
    fn drop(&mut self) {
        extern "C" {
            fn munmap(addr: *mut u8, len: usize) -> i32;
        }
        unsafe {
            munmap(self.ptr, self.len);
        }
    }
}

/// Write a minimal valid PE32+ header into `buf` with two sections:
/// `.text` at `[text_rva, text_rva + text_size)` and `.rdata` at `[rdata_rva, rdata_rva + rdata_size)`.
#[cfg(test)]
fn init_synth_pe(
    buf: &mut [u8],
    preferred_base: u64,
    entry_rva: u32,
    text_rva: u32,
    text_size: u32,
    rdata_rva: u32,
    rdata_size: u32,
    export_dir: Option<(u32, u32)>,
    pdata_dir: Option<(u32, u32)>,
) {
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    w16(buf, 0, 0x5A4D); // MZ
    w32(buf, 0x3C, 0x40); // e_lfanew
    w32(buf, 0x40, 0x0000_4550); // PE\0\0
    w16(buf, 0x44, 0x8664); // AMD64
    w16(buf, 0x46, 2); // NumberOfSections
    w16(buf, 0x54, 240); // SizeOfOptionalHeader

    let opt = 0x58;
    w16(buf, opt, 0x20B); // PE32+
    w32(buf, opt + 16, entry_rva); // AddressOfEntryPoint
    w64(buf, opt + 24, preferred_base); // ImageBase
    w32(buf, opt + 56, buf.len() as u32); // SizeOfImage
    w32(buf, opt + 60, 0x400); // SizeOfHeaders
    w32(buf, opt + 108, 16); // NumberOfRvaAndSizes
    if let Some((rva, len)) = export_dir {
        w32(buf, opt + 112, rva);
        w32(buf, opt + 112 + 4, len);
    }
    if let Some((rva, len)) = pdata_dir {
        w32(buf, opt + 112 + 3 * 8, rva);
        w32(buf, opt + 112 + 3 * 8 + 4, len);
    }

    let sec0 = opt + 240;
    buf[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(buf, sec0 + 8, text_size);
    w32(buf, sec0 + 12, text_rva);
    w32(buf, sec0 + 36, 0x6000_0020); // MEM_EXECUTE | MEM_READ | CNT_CODE

    let sec1 = sec0 + 40;
    buf[sec1..sec1 + 6].copy_from_slice(b".rdata");
    w32(buf, sec1 + 8, rdata_size);
    w32(buf, sec1 + 12, rdata_rva);
    w32(buf, sec1 + 36, 0x4000_0040); // MEM_READ | CNT_INITIALIZED_DATA
}

#[cfg(test)]
fn teardown_test_modules(names: &[&str]) {
    if let Ok(mut v) = MAPPED.lock() {
        v.retain(|m| !names.iter().any(|&n| m.name == n));
    }
    let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    if let Some(dir) = g.as_mut() {
        let gone: Vec<u32> = (0..dir.modules.len() as u32)
            .filter(|&i| names.iter().any(|&n| dir.modules[i as usize].name == n) && !dir.modules[i as usize].gone)
            .collect();
        for &i in &gone {
            dir.modules[i as usize].gone = true;
        }
        let ips: Vec<u64> = dir.map.iter().filter(|(_, &(m, _))| gone.contains(&m)).map(|(&ip, _)| ip).collect();
        dir.map.retain(|_, &mut (m, _)| !gone.contains(&m));
        for ip in ips {
            unsafe { dir.probe_sync(ip) };
        }
        for v in dir.incoming.values_mut() {
            v.retain(|&(m, _)| !gone.contains(&m));
        }
    }
}

/// Helper: encode a 7-byte RIP-relative `lea reg, [rip + disp32]` pointing to `target_va`.
#[cfg(test)]
fn emit_lea_rip(code: &mut Vec<u8>, code_base_va: u64, reg: u8, target_va: u64) {
    let rex = 0x48 | ((reg >> 3) << 2); // REX.W + REX.R
    let modrm = ((reg & 7) << 3) | 0x05; // mod=00, rm=101 (RIP-relative)
    let next_ip = code_base_va + code.len() as u64 + 7;
    let disp = (target_va as i64 - next_ip as i64) as i32;
    code.push(rex);
    code.push(0x8D);
    code.push(modrm);
    code.extend_from_slice(&disp.to_le_bytes());
}

/// Helper: encode a 6-byte RIP-relative `call qword ptr [rip + disp32]` targeting `slot_va`.
#[cfg(test)]
fn emit_call_rip_slot(code: &mut Vec<u8>, code_base_va: u64, slot_va: u64) {
    let next_ip = code_base_va + code.len() as u64 + 6;
    let disp = (slot_va as i64 - next_ip as i64) as i32;
    code.extend_from_slice(&[0xFF, 0x15]);
    code.extend_from_slice(&disp.to_le_bytes());
}

/// Pattern 1 (MSVC / Clang `rva32` image-base-relative switch tables):
/// - 1A: `cmp ecx, 3; lea rdx, [rip + __ImageBase]; ja default; mov eax, [rdx + rcx*4 + rva_tbl]; add rax, rdx; jmp rax`
/// - 1B: separate `lea rBase, [rip + __ImageBase]` + `lea r8, [rip + tbl]` + `movsxd rax, [r8 + rcx*4]` + `add rax, rBase` + `jmp rax`
/// - 1C: MSVC hoisted `__ImageBase` (`r11` undefined in dispatch block, `[r11 + rcx*4 + rva_tbl]` + `add rax, r11; jmp rax`)
#[test]
fn aot_jump_table_discovery_msvc_rva32() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3000_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 36)));

    // --- Function 1A @ 0x1000 (MSVC classic with intervening `lea` between `cmp` and `ja`) ---
    let f1a_cases: [u32; 4] = [0x1080, 0x1090, 0x10A0, 0x10B0];
    let f1a_default: u32 = 0x10C0;
    let tbl_1a_rva: u32 = 0x4100;
    for (i, &rva) in f1a_cases.iter().enumerate() {
        buf[tbl_1a_rva as usize + i * 4..tbl_1a_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    // Trailing garbage word (must NOT be followed because `cmp ecx, 3; ja` caps at 4).
    buf[tbl_1a_rva as usize + 16..tbl_1a_rva as usize + 20].copy_from_slice(&0x10D0u32.to_le_bytes());

    let mut c1a = Vec::new();
    // mov eax, ecx; cmp eax, 3; lea rdx, [rip + __ImageBase]; ja default
    c1a.extend_from_slice(&[0x89, 0xC8]); // mov eax, ecx
    c1a.extend_from_slice(&[0x83, 0xF8, 0x03]); // cmp eax, 3
    emit_lea_rip(&mut c1a, base64 + 0x1000, 2 /* rdx */, base64);
    let ja_next = 0x1000 + c1a.len() as u32 + 6;
    c1a.extend_from_slice(&[0x0F, 0x87]);
    c1a.extend_from_slice(&((f1a_default as i32) - (ja_next as i32)).to_le_bytes());
    // Dispatch block: mov eax, dword ptr [rdx + rax*4 + tbl_1a_rva]; add rax, rdx; jmp rax
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, dword_ptr(rdx + rax * 4 + tbl_1a_rva as i32)).unwrap();
    a.add(rax, rdx).unwrap();
    a.jmp(rax).unwrap();
    let d1a = a.assemble(base64 + 0x1000 + c1a.len() as u64).unwrap();
    c1a.extend_from_slice(&d1a);
    buf[0x1000..0x1000 + c1a.len()].copy_from_slice(&c1a);
    for (i, &rva) in f1a_cases.iter().enumerate() {
        let b = [0xB8, (10 + i as u8), 0, 0, 0, 0xC3]; // mov eax, 10+i; ret
        buf[rva as usize..rva as usize + b.len()].copy_from_slice(&b);
    }
    buf[f1a_default as usize..f1a_default as usize + 6].copy_from_slice(&[0xB8, 99, 0, 0, 0, 0xC3]);
    buf[0x10D0..0x10D6].copy_from_slice(&[0xB8, 77, 0, 0, 0, 0xC3]); // out-of-bound decoy

    // --- Function 1B @ 0x1200 (Separate `lea rBase` and `lea rTbl`, `movsxd`) ---
    let f1b_cases: [u32; 4] = [0x1280, 0x1290, 0x12A0, 0x12B0];
    let f1b_default: u32 = 0x12C0;
    let tbl_1b_rva: u32 = 0x4140;
    for (i, &rva) in f1b_cases.iter().enumerate() {
        buf[tbl_1b_rva as usize + i * 4..tbl_1b_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    let mut c1b = Vec::new();
    c1b.extend_from_slice(&[0x83, 0xF9, 0x04]); // cmp ecx, 4
    let jae_next = 0x1200 + c1b.len() as u32 + 6;
    c1b.extend_from_slice(&[0x0F, 0x83]); // jae default
    c1b.extend_from_slice(&((f1b_default as i32) - (jae_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c1b, base64 + 0x1200, 2 /* rdx */, base64);
    emit_lea_rip(&mut c1b, base64 + 0x1200, 8 /* r8 */, base64 + tbl_1b_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(r8 + rcx * 4)).unwrap();
    a.add(rax, rdx).unwrap();
    a.jmp(rax).unwrap();
    c1b.extend_from_slice(&a.assemble(base64 + 0x1200 + c1b.len() as u64).unwrap());
    buf[0x1200..0x1200 + c1b.len()].copy_from_slice(&c1b);
    for (i, &rva) in f1b_cases.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (20 + i as u8), 0, 0, 0, 0xC3]);
    }
    buf[f1b_default as usize..f1b_default as usize + 6].copy_from_slice(&[0xB8, 98, 0, 0, 0, 0xC3]);

    // --- Function 1C @ 0x1400 (MSVC hoisted `__ImageBase` in r11, undefined in this block) ---
    let f1c_cases: [u32; 3] = [0x1480, 0x1490, 0x14A0];
    let f1c_default: u32 = 0x14B0;
    let tbl_1c_rva: u32 = 0x4180;
    for (i, &rva) in f1c_cases.iter().enumerate() {
        buf[tbl_1c_rva as usize + i * 4..tbl_1c_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    let mut c1c = Vec::new();
    c1c.extend_from_slice(&[0x83, 0xF9, 0x02]); // cmp ecx, 2
    let ja_next = 0x1400 + c1c.len() as u32 + 6;
    c1c.extend_from_slice(&[0x0F, 0x87]); // ja default
    c1c.extend_from_slice(&((f1c_default as i32) - (ja_next as i32)).to_le_bytes());
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, dword_ptr(r11 + rcx * 4 + tbl_1c_rva as i32)).unwrap();
    a.add(rax, r11).unwrap();
    a.jmp(rax).unwrap();
    c1c.extend_from_slice(&a.assemble(base64 + 0x1400 + c1c.len() as u64).unwrap());
    buf[0x1400..0x1400 + c1c.len()].copy_from_slice(&c1c);
    for (i, &rva) in f1c_cases.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (30 + i as u8), 0, 0, 0, 0xC3]);
    }
    buf[f1c_default as usize..f1c_default as usize + 6].copy_from_slice(&[0xB8, 97, 0, 0, 0, 0xC3]);

    // .pdata entries for the 3 functions.
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1300), (0x1400, 0x1500)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01; // UNWIND_INFO v1

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1200, 0x1400], 2) };
    assert_eq!(clos.jt_tables, 3, "expected 3 MSVC rva32 jump tables discovered");
    assert_eq!(clos.jt_targets, 11, "expected 4 + 4 + 3 = 11 unique jump-table targets");
    for &rva in f1a_cases.iter().chain(f1b_cases.iter()).chain(f1c_cases.iter()) {
        assert!(ips.contains(&(base64 + rva as u64)), "missing switch case RVA {rva:#x}");
    }
    assert!(
        !ips.contains(&(base64 + 0x10D0)),
        "out-of-bounds 5th entry (past `cmp eax, 3; ja`) must NOT be followed"
    );
}

/// Pattern 2 (direct 64-bit pointer tables):
/// - 2A: `cmp ecx, 3; ja default; lea r8, [rip + tbl]; jmp qword ptr [r8 + rcx*8]`
/// - 2B: unbounded `lea r8, [rip + tbl]; mov rax, qword ptr [r8 + rcx*8]; jmp rax` (stops at 0)
#[test]
fn aot_jump_table_discovery_direct_ptr64() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3100_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 24)));

    // --- 2A @ 0x1000: bounded `jmp qword ptr [r8 + rcx*8]` ---
    let f2a_cases: [u32; 4] = [0x1080, 0x1090, 0x10A0, 0x10B0];
    let f2a_default: u32 = 0x10C0;
    let tbl_2a_rva: u32 = 0x4100;
    for (i, &rva) in f2a_cases.iter().enumerate() {
        let va = base64 + rva as u64;
        buf[tbl_2a_rva as usize + i * 8..tbl_2a_rva as usize + i * 8 + 8].copy_from_slice(&va.to_le_bytes());
    }
    let mut c2a = Vec::new();
    c2a.extend_from_slice(&[0x83, 0xF9, 0x03]); // cmp ecx, 3
    let ja_next = 0x1000 + c2a.len() as u32 + 6;
    c2a.extend_from_slice(&[0x0F, 0x87]);
    c2a.extend_from_slice(&((f2a_default as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c2a, base64 + 0x1000, 8 /* r8 */, base64 + tbl_2a_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.jmp(qword_ptr(r8 + rcx * 8)).unwrap();
    c2a.extend_from_slice(&a.assemble(base64 + 0x1000 + c2a.len() as u64).unwrap());
    buf[0x1000..0x1000 + c2a.len()].copy_from_slice(&c2a);
    for (i, &rva) in f2a_cases.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, i as u8, 0, 0, 0, 0xC3]);
    }
    buf[f2a_default as usize..f2a_default as usize + 6].copy_from_slice(&[0xB8, 99, 0, 0, 0, 0xC3]);

    // --- 2B @ 0x1200: unbounded `mov rax, qword ptr [r8 + rcx*8]; jmp rax` (stops at zero entry) ---
    let f2b_cases: [u32; 3] = [0x1280, 0x1290, 0x12A0];
    let tbl_2b_rva: u32 = 0x4180;
    for (i, &rva) in f2b_cases.iter().enumerate() {
        let va = base64 + rva as u64;
        buf[tbl_2b_rva as usize + i * 8..tbl_2b_rva as usize + i * 8 + 8].copy_from_slice(&va.to_le_bytes());
    }
    // Entry 3 is 0 (terminates conservative scan); entry 4 is a decoy that must not be reached.
    let decoy = base64 + 0x12B0;
    buf[tbl_2b_rva as usize + 32..tbl_2b_rva as usize + 40].copy_from_slice(&decoy.to_le_bytes());
    let mut c2b = Vec::new();
    emit_lea_rip(&mut c2b, base64 + 0x1200, 8 /* r8 */, base64 + tbl_2b_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(rax, qword_ptr(r8 + rcx * 8)).unwrap();
    a.jmp(rax).unwrap();
    c2b.extend_from_slice(&a.assemble(base64 + 0x1200 + c2b.len() as u64).unwrap());
    buf[0x1200..0x1200 + c2b.len()].copy_from_slice(&c2b);
    for (i, &rva) in f2b_cases.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (10 + i as u8), 0, 0, 0, 0xC3]);
    }
    buf[0x12B0..0x12B6].copy_from_slice(&[0xB8, 77, 0, 0, 0, 0xC3]);

    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1300)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1200], 2) };
    assert_eq!(clos.jt_tables, 2);
    assert_eq!(clos.jt_targets, 7);
    for &rva in f2a_cases.iter().chain(f2b_cases.iter()) {
        assert!(ips.contains(&(base64 + rva as u64)), "missing ptr64 switch case RVA {rva:#x}");
    }
    assert!(!ips.contains(&(base64 + 0x12B0)), "scan must stop at zero terminator");
}

/// Pattern 3 (compact 8-bit and 16-bit index indirection tables -> 32-bit RVA jump table).
#[test]
fn aot_jump_table_discovery_compact_indirection_u8_u16() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3200_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 24)));

    // --- 3A @ 0x1000: 8-bit indirection table (8 cases -> 3 unique handlers) ---
    let h3a: [u32; 3] = [0x1080, 0x1090, 0x10A0];
    let def3a: u32 = 0x10B0;
    let idx8_rva: u32 = 0x4100;
    let jmp3a_rva: u32 = 0x4120;
    let map8: [u8; 8] = [0, 1, 2, 1, 0, 2, 2, 1];
    buf[idx8_rva as usize..idx8_rva as usize + 8].copy_from_slice(&map8);
    for (i, &rva) in h3a.iter().enumerate() {
        buf[jmp3a_rva as usize + i * 4..jmp3a_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    let mut c3a = Vec::new();
    c3a.extend_from_slice(&[0x83, 0xF9, 0x07]); // cmp ecx, 7
    let ja_next = 0x1000 + c3a.len() as u32 + 6;
    c3a.extend_from_slice(&[0x0F, 0x87]);
    c3a.extend_from_slice(&((def3a as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c3a, base64 + 0x1000, 2 /* rdx */, base64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movzx(eax, byte_ptr(rdx + rcx + idx8_rva as i32)).unwrap();
    a.mov(eax, dword_ptr(rdx + rax * 4 + jmp3a_rva as i32)).unwrap();
    a.add(rax, rdx).unwrap();
    a.jmp(rax).unwrap();
    c3a.extend_from_slice(&a.assemble(base64 + 0x1000 + c3a.len() as u64).unwrap());
    buf[0x1000..0x1000 + c3a.len()].copy_from_slice(&c3a);
    for (i, &rva) in h3a.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, i as u8, 0, 0, 0, 0xC3]);
    }
    buf[def3a as usize..def3a as usize + 6].copy_from_slice(&[0xB8, 99, 0, 0, 0, 0xC3]);

    // --- 3B @ 0x1200: 16-bit indirection table (6 cases -> 4 unique handlers) ---
    let h3b: [u32; 4] = [0x1280, 0x1290, 0x12A0, 0x12B0];
    let def3b: u32 = 0x12C0;
    let idx16_rva: u32 = 0x4160;
    let jmp3b_rva: u32 = 0x4180;
    let map16: [u16; 6] = [3, 0, 2, 1, 3, 2];
    for (i, &v) in map16.iter().enumerate() {
        buf[idx16_rva as usize + i * 2..idx16_rva as usize + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
    }
    for (i, &rva) in h3b.iter().enumerate() {
        buf[jmp3b_rva as usize + i * 4..jmp3b_rva as usize + i * 4 + 4].copy_from_slice(&rva.to_le_bytes());
    }
    let mut c3b = Vec::new();
    c3b.extend_from_slice(&[0x83, 0xF9, 0x05]); // cmp ecx, 5
    let ja_next = 0x1200 + c3b.len() as u32 + 6;
    c3b.extend_from_slice(&[0x0F, 0x87]);
    c3b.extend_from_slice(&((def3b as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c3b, base64 + 0x1200, 2 /* rdx */, base64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movzx(eax, word_ptr(rdx + rcx * 2 + idx16_rva as i32)).unwrap();
    a.mov(eax, dword_ptr(rdx + rax * 4 + jmp3b_rva as i32)).unwrap();
    a.add(rax, rdx).unwrap();
    a.jmp(rax).unwrap();
    c3b.extend_from_slice(&a.assemble(base64 + 0x1200 + c3b.len() as u64).unwrap());
    buf[0x1200..0x1200 + c3b.len()].copy_from_slice(&c3b);
    for (i, &rva) in h3b.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (20 + i as u8), 0, 0, 0, 0xC3]);
    }
    buf[def3b as usize..def3b as usize + 6].copy_from_slice(&[0xB8, 98, 0, 0, 0, 0xC3]);

    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1300)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1200], 2) };
    assert_eq!(clos.jt_tables, 2);
    assert_eq!(clos.jt_targets, 7, "expected 3 + 4 unique deduplicated handlers");
    for &rva in h3a.iter().chain(h3b.iter()) {
        assert!(ips.contains(&(base64 + rva as u64)), "missing indirection handler RVA {rva:#x}");
    }
}

/// Pattern 4 (Clang/GCC PIC signed 32-bit table-base-relative offsets):
/// - 4A: `lea rcx, [rip + tbl]; movsxd rax, [rcx + rdx*4]; add rax, rcx; jmp rax` with NEGATIVE offsets (handlers before `.rdata` table)
/// - 4B: symmetric `add rcx, rax; jmp rcx` with `and edx, 3` mask bound
/// - 4C: `lea rax, [rcx + rax]; jmp rax`
#[test]
fn aot_jump_table_discovery_clang_pic_signed32() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3300_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 36)));

    // --- 4A @ 0x1000: negative signed-32 offsets from `.rdata` table at 0x4100 back into `.text` ---
    let h4a: [u32; 4] = [0x1080, 0x1090, 0x10A0, 0x10B0];
    let def4a: u32 = 0x10C0;
    let tbl_4a_rva: u32 = 0x4100;
    for (i, &rva) in h4a.iter().enumerate() {
        let rel = (rva as i32) - (tbl_4a_rva as i32); // negative!
        assert!(rel < 0);
        buf[tbl_4a_rva as usize + i * 4..tbl_4a_rva as usize + i * 4 + 4].copy_from_slice(&rel.to_le_bytes());
    }
    let mut c4a = Vec::new();
    c4a.extend_from_slice(&[0x83, 0xFA, 0x03]); // cmp edx, 3
    let ja_next = 0x1000 + c4a.len() as u32 + 6;
    c4a.extend_from_slice(&[0x0F, 0x87]);
    c4a.extend_from_slice(&((def4a as i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c4a, base64 + 0x1000, 1 /* rcx */, base64 + tbl_4a_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(rcx + rdx * 4)).unwrap();
    a.add(rax, rcx).unwrap();
    a.jmp(rax).unwrap();
    c4a.extend_from_slice(&a.assemble(base64 + 0x1000 + c4a.len() as u64).unwrap());
    buf[0x1000..0x1000 + c4a.len()].copy_from_slice(&c4a);
    for (i, &rva) in h4a.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, i as u8, 0, 0, 0, 0xC3]);
    }
    buf[def4a as usize..def4a as usize + 6].copy_from_slice(&[0xB8, 99, 0, 0, 0, 0xC3]);

    // --- 4B @ 0x1200: `and edx, 3` mask + symmetric `add rcx, rax; jmp rcx` ---
    let h4b: [u32; 4] = [0x1280, 0x1290, 0x12A0, 0x12B0];
    let tbl_4b_rva: u32 = 0x4140;
    for (i, &rva) in h4b.iter().enumerate() {
        let rel = (rva as i32) - (tbl_4b_rva as i32);
        buf[tbl_4b_rva as usize + i * 4..tbl_4b_rva as usize + i * 4 + 4].copy_from_slice(&rel.to_le_bytes());
    }
    let mut c4b = Vec::new();
    c4b.extend_from_slice(&[0x83, 0xE2, 0x03]); // and edx, 3
    emit_lea_rip(&mut c4b, base64 + 0x1200, 1 /* rcx */, base64 + tbl_4b_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(rcx + rdx * 4)).unwrap();
    a.add(rcx, rax).unwrap();
    a.jmp(rcx).unwrap();
    c4b.extend_from_slice(&a.assemble(base64 + 0x1200 + c4b.len() as u64).unwrap());
    buf[0x1200..0x1200 + c4b.len()].copy_from_slice(&c4b);
    for (i, &rva) in h4b.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (10 + i as u8), 0, 0, 0, 0xC3]);
    }

    // --- 4C @ 0x1400: `lea rax, [rcx + rax]; jmp rax` with `jbe` taken-edge bound ---
    let h4c: [u32; 3] = [0x1480, 0x1490, 0x14A0];
    let tbl_4c_rva: u32 = 0x4180;
    let disp_4c: u32 = 0x1430;
    for (i, &rva) in h4c.iter().enumerate() {
        let rel = (rva as i32) - (tbl_4c_rva as i32);
        buf[tbl_4c_rva as usize + i * 4..tbl_4c_rva as usize + i * 4 + 4].copy_from_slice(&rel.to_le_bytes());
    }
    let mut c4c = Vec::new();
    c4c.extend_from_slice(&[0x83, 0xFA, 0x02]); // cmp edx, 2
    let jbe_next = 0x1400 + c4c.len() as u32 + 6;
    c4c.extend_from_slice(&[0x0F, 0x86]); // jbe disp_4c
    c4c.extend_from_slice(&((disp_4c as i32) - (jbe_next as i32)).to_le_bytes());
    c4c.extend_from_slice(&[0xB8, 97, 0, 0, 0, 0xC3]); // default fallthrough
    buf[0x1400..0x1400 + c4c.len()].copy_from_slice(&c4c);

    let mut d4c = Vec::new();
    emit_lea_rip(&mut d4c, base64 + disp_4c as u64, 1 /* rcx */, base64 + tbl_4c_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(rcx + rdx * 4)).unwrap();
    a.lea(rax, ptr(rcx + rax)).unwrap();
    a.jmp(rax).unwrap();
    d4c.extend_from_slice(&a.assemble(base64 + disp_4c as u64 + d4c.len() as u64).unwrap());
    buf[disp_4c as usize..disp_4c as usize + d4c.len()].copy_from_slice(&d4c);
    for (i, &rva) in h4c.iter().enumerate() {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (20 + i as u8), 0, 0, 0, 0xC3]);
    }

    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1300), (0x1400, 0x1500)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1200, 0x1400], 2) };
    assert_eq!(clos.jt_tables, 3);
    assert_eq!(clos.jt_targets, 11);
    for &rva in h4a.iter().chain(h4b.iter()).chain(h4c.iter()) {
        assert!(ips.contains(&(base64 + rva as u64)), "missing Clang PIC case RVA {rva:#x}");
    }
}

/// Soundness / negative tests for jump-table discovery:
/// - IAT jump thunk (`jmp qword ptr [rip + slot]`) must NOT be mistaken for a jump table;
/// - Virtual-call dispatch (`mov rax, [rcx]; jmp qword ptr [rax + 0x10]`) must NOT trigger;
/// - A jump-table entry pointing into another `.pdata` function stops the scan cleanly.
#[test]
fn aot_jump_table_discovery_ignores_non_switch_indirect_branches() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_3400_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x4400, 48)));

    // 1. IAT jump thunk @ 0x1000: `jmp qword ptr [rip + 0x4100]`
    let iat_slot = base64 + 0x4100;
    let target_in_text = base64 + 0x1080;
    buf[0x4100..0x4108].copy_from_slice(&target_in_text.to_le_bytes());
    let disp = (iat_slot as i64 - (base64 + 0x1006) as i64) as i32;
    buf[0x1000..0x1002].copy_from_slice(&[0xFF, 0x25]);
    buf[0x1002..0x1006].copy_from_slice(&disp.to_le_bytes());
    buf[0x1080..0x1086].copy_from_slice(&[0xB8, 1, 0, 0, 0, 0xC3]);

    // 2. Vtable jump @ 0x1100: `mov rax, [rcx]; jmp qword ptr [rax + 0x10]`
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(rax, qword_ptr(rcx)).unwrap();
    a.jmp(qword_ptr(rax + 0x10)).unwrap();
    let vtbl = a.assemble(base64 + 0x1100).unwrap();
    buf[0x1100..0x1100 + vtbl.len()].copy_from_slice(&vtbl);

    // 3. Switch @ 0x1200 whose 3rd entry crosses into another `.pdata` function @ 0x1300.
    let tbl_rva: u32 = 0x4200;
    for (i, &rva) in [0x1280u32, 0x1290, 0x1300, 0x12A0].iter().enumerate() {
        let va = base64 + rva as u64;
        buf[tbl_rva as usize + i * 8..tbl_rva as usize + i * 8 + 8].copy_from_slice(&va.to_le_bytes());
    }
    let mut c3 = Vec::new();
    emit_lea_rip(&mut c3, base64 + 0x1200, 8 /* r8 */, base64 + tbl_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.jmp(qword_ptr(r8 + rcx * 8)).unwrap();
    c3.extend_from_slice(&a.assemble(base64 + 0x1200 + c3.len() as u64).unwrap());
    buf[0x1200..0x1200 + c3.len()].copy_from_slice(&c3);
    for &rva in &[0x1280u32, 0x1290, 0x12A0, 0x1300] {
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, 5, 0, 0, 0, 0xC3]);
    }

    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    for (i, (s, e)) in [(0x1000u32, 0x10C0u32), (0x1100, 0x1140), (0x1200, 0x12E0), (0x1300, 0x1340)].iter().enumerate() {
        w32(buf, 0x4400 + i * 12, *s);
        w32(buf, 0x4400 + i * 12 + 4, *e);
        w32(buf, 0x4400 + i * 12 + 8, 0x4500);
    }
    buf[0x4500] = 0x01;

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000, 0x1100, 0x1200], 2) };
    assert_eq!(clos.jt_tables, 1, "only the real switch table should be discovered");
    assert_eq!(clos.jt_targets, 2, "scan must stop at the cross-function entry 0x1300");
    assert!(!ips.contains(&target_in_text), "IAT thunk must not trigger jump-table discovery");
    assert!(ips.contains(&(base64 + 0x1280)) && ips.contains(&(base64 + 0x1290)));
    assert!(!ips.contains(&(base64 + 0x1300)), "cross-function entry must be rejected");
    assert!(!ips.contains(&(base64 + 0x12A0)), "scan must stop at first invalid/cross-function entry");
}

/// Regression test: `cmp reg, -1` (`imm == u64::MAX`) followed by `ja`/`jbe` must never overflow
/// when computing jump-table bounds during offline CFG discovery.
#[test]
fn aot_jump_table_discovery_handles_cmp_u64_max_without_overflow() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0x4000, 0x2_3450_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x1000, 0x2000, 0x1000, None, None);

    let mut a = CodeAssembler::new(64).unwrap();
    let mut lbl_out = a.create_label();
    a.cmp(rax, -1i32).unwrap();
    a.ja(lbl_out).unwrap();
    a.mov(eax, 1).unwrap();
    a.ret().unwrap();
    a.set_label(&mut lbl_out).unwrap();
    a.mov(eax, 2).unwrap();
    a.ret().unwrap();
    let code = a.assemble(base64 + 0x1000).unwrap();
    buf[0x1000..0x1000 + code.len()].copy_from_slice(&code);

    let mut bc = Box::new(BlockCache::new());
    let (ips, clos) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000], 2) };
    assert!(ips.contains(&(base64 + 0x1000)));
    assert_eq!(clos.jt_tables, 0);
}

/// Parallel `.aot` prewarming produces bit-for-bit identical `.aot` images across 1, 2, 4,
/// and 8 worker threads, passes `Verify::All` with 0 mismatches, and benchmarks wall-clock
/// speedup on a multi-thousand-block synthetic PE image.
#[test]
fn aot_parallel_prewarm_deterministic_and_verified() {
    use iced_x86::code_asm::*;

    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            // Synthesize a realistic 640-function PE (~1,920 basic blocks) with branches,
            // calls, literal loads, and a host-builtin IAT call so every stub branch kind,
            // link site, and literal pool relocation is exercised under parallel translation.
            const NFUNCS: usize = 2048;
            const FUNC_STRIDE: usize = 0x40;
            const TEXT_RVA: u32 = 0x1000;
            const TEXT_SIZE: u32 = 0x2_1000; // > 2048 * 0x40 = 0x2_0000 + padding
            const RDATA_RVA: u32 = 0x2_2000;
            const RDATA_SIZE: u32 = 0xA000;
            const IMG_SIZE: usize = 0x2_C000;

            let mut pe = TestMappedBuf::alloc(IMG_SIZE, 0x2_3500_0000);
            let base = pe.base();
            let base64 = base as u64;
            let buf = pe.as_slice_mut();
            let pdata_rva: u32 = RDATA_RVA + 0x100;
            init_synth_pe(
                buf,
                base64,
                TEXT_RVA,
                TEXT_RVA,
                TEXT_SIZE,
                RDATA_RVA,
                RDATA_SIZE,
                None,
                Some((pdata_rva, (NFUNCS * 12) as u32)),
            );

            // Register a plain builtin VA for IAT calls inside the synthetic functions.
            let h_va = base64 + RDATA_RVA as u64 + 0x80;
            crate::exe::register_native_va(h_va, win32_rt::msvcrt::memset as *const () as usize);
            let iat_slot = base64 + RDATA_RVA as u64;
            buf[RDATA_RVA as usize..RDATA_RVA as usize + 8].copy_from_slice(&h_va.to_le_bytes());

            let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
            let unwind_rva: u32 = RDATA_RVA + 0x7000;
            buf[unwind_rva as usize] = 0x01;

            for i in 0..NFUNCS {
                let rva = TEXT_RVA + (i * FUNC_STRIDE) as u32;
                let va = base64 + rva as u64;
                let mut code = Vec::new();
                if i % 8 == 0 {
                    // Includes an indirect IAT call to the host builtin + conditional branch.
                    code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]); // sub rsp, 0x28
                    code.extend_from_slice(&[0x85, 0xC9]); // test ecx, ecx
                    code.extend_from_slice(&[0x74, 0x0B]); // jz +11
                    emit_call_rip_slot(&mut code, va, iat_slot); // 6 bytes
                    code.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]); // add rsp, 0x28; ret
                    // else branch:
                    code.extend_from_slice(&[0xB8]);
                    code.extend_from_slice(&(i as u32).to_le_bytes());
                    code.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
                } else {
                    // Conditional diamond + local arithmetic (3 basic blocks per function).
                    let mut a = CodeAssembler::new(64).unwrap();
                    let mut l_alt = a.create_label();
                    a.cmp(ecx, (i & 15) as i32).unwrap();
                    a.ja(l_alt).unwrap();
                    a.lea(eax, ptr(rcx + rdx + (i as i32))).unwrap();
                    a.imul_3(eax, eax, (i as i32) | 1).unwrap();
                    a.ret().unwrap();
                    a.set_label(&mut l_alt).unwrap();
                    a.mov(eax, edx).unwrap();
                    a.xor(eax, (i as i32) ^ 0x5A).unwrap();
                    a.ret().unwrap();
                    code = a.assemble(va).unwrap();
                }
                assert!(code.len() <= FUNC_STRIDE, "fn {i} exceeded stride");
                buf[rva as usize..rva as usize + code.len()].copy_from_slice(&code);
                let pe_off = pdata_rva as usize + i * 12;
                w32(buf, pe_off, rva);
                w32(buf, pe_off + 4, rva + FUNC_STRIDE as u32);
                w32(buf, pe_off + 8, unwind_rva);
            }

            let roots = unsafe { discover_pe_roots(base, pe.len, 0) };
            assert_eq!(roots.len(), NFUNCS);

            let key = unsafe { module_key(base, pe.len) };
            let mut pool_1 = WorkerCaches::new();

            let t_seq = std::time::Instant::now();
            let (ips_1, _) = unsafe { discover_with_threads(&mut pool_1, base, pe.len, &roots, 2, 1) };
            let img_1 = unsafe { build_image_with_threads(&mut pool_1, &ips_1, BUILD_VBASE, 1) }
                .expect("1-thread build");
            let dt_seq = t_seq.elapsed();
            let bytes_1 = encode("synth_par.dll", &key, &img_1, 0);
            assert!(ips_1.len() >= NFUNCS * 2, "expected multi-block closure, got {}", ips_1.len());
            assert!(img_1.blocks.len() >= NFUNCS * 2);
            assert!(!img_1.links.is_empty());

            let mut timings = Vec::new();
            for threads in [2usize, 4, 8] {
                let mut pool_p = WorkerCaches::new();
                let t_par = std::time::Instant::now();
                let (ips_p, _) = unsafe { discover_with_threads(&mut pool_p, base, pe.len, &roots, 2, threads) };
                let img_p = unsafe { build_image_with_threads(&mut pool_p, &ips_p, BUILD_VBASE, threads) }
                    .expect("parallel build");
                let dt_par = t_par.elapsed();
                timings.push((threads, dt_par));
                assert_eq!(ips_p, ips_1, "discovered IPs diverged at {threads} threads");
                let bytes_p = encode("synth_par.dll", &key, &img_p, 0);
                assert_eq!(
                    bytes_p, bytes_1,
                    "encoded .aot output differed between 1 thread and {threads} threads"
                );
            }

            // Verify every single block of the parallel-built image via `install_image(..., Verify::All)`.
            let (_, _, img_dec, _) = decode(&bytes_1).expect("decode parallel .aot");
            let total_blocks = img_dec.blocks.len();
            let inst = unsafe { install_image("synth_par.dll", base, pe.len, img_dec, Verify::All, true) }
                .expect("install parallel .aot");
            assert_eq!(inst.mismatches, 0, "parallel .aot had verify mismatches");
            assert_eq!(inst.verified, total_blocks, "expected all blocks verified");
            assert_eq!(inst.live, total_blocks, "expected all blocks live");

            let summary: Vec<String> = timings
                .iter()
                .map(|&(t, dt)| {
                    format!(
                        "{t}t={:.2} ms ({:.2}x)",
                        dt.as_secs_f64() * 1e3,
                        dt_seq.as_secs_f64() / dt.as_secs_f64().max(1e-9)
                    )
                })
                .collect();
            eprintln!(
                "aot parallel benchmark ({total_blocks} blocks): 1t={:.2} ms, {}",
                dt_seq.as_secs_f64() * 1e3,
                summary.join(", "),
            );

            test_uninstall();
            crate::exe::unregister_native_va(h_va);
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Multi-module `.aot` prewarm + reload verification test under `PORT_AOT_VERIFY=1` semantics:
/// - Maps TWO synthetic PE32+ modules (`synth_app_v.exe` and `synth_helper_v.dll`) at
///   relocated/shifted bases (`!= ImageBase`) with PE DIR64 base relocations applied;
/// - Exercises cross-module indirect IAT calls, host-builtin IAT relocations, MSVC rva32 &
///   Clang PIC signed-32 switch jump tables, and intra-module AOT direct links;
/// - Prewarms both modules with `aot_prewarm_all_mapped`, moves host-builtin IAT slot VAs,
///   reloads both `.aot` caches with full block verification (`verify_mode() == 1`), verifies
///   0 verify mismatches and non-zero rebased relocations, and executes every switch case
///   comparing AOT output against the pure interpreter oracle;
/// - Then shifts `synth_helper_v.dll` to a SECOND relocated base address (`dll_base_2 != dll_base_1`),
///   confirms cache-key isolation on the un-prewarmed shifted base, updates the `.exe`'s
///   cross-module IAT slots, re-prewarms and reloads under full verification (0 mismatches),
///   and re-executes every switch case to confirm exact return values.
#[test]
fn aot_multi_module_prewarm_reload_verify_and_execute() {
    use iced_x86::code_asm::*;

    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            let _env = TestEnvGuard::new(&[
                "PORT_AOT_DIR",
                "AW2_DLL_AOT_DIR",
                "PORT_AOT_THREADS",
                "PORT_AOT_CLOSURE",
            ]);
            let d = test_dir("multi-verify");
            std::env::set_var("PORT_AOT_DIR", &d);
            std::env::remove_var("AW2_DLL_AOT_DIR");
            std::env::set_var("PORT_AOT_THREADS", "4");
            std::env::set_var("PORT_AOT_CLOSURE", "3");

            let exe_name = "synth_app_v.exe";
            let dll_name = "synth_helper_v.dll";

            // Scratch guest buffer for host `memset` calls + 3 distinct host-builtin VAs so
            // we can move the IAT slots across reloads and exercise `rebase_block`.
            let scratch_mem = TestMappedBuf::alloc(0x4000, 0x2_3800_0000);
            let scratch_va = scratch_mem.base() as u64;
            let h_memset_1 = scratch_va + 0x1000;
            let h_memset_2 = scratch_va + 0x1020;
            let h_memset_3 = scratch_va + 0x1040;
            let memset_ptr = win32_rt::msvcrt::memset as *const () as usize;
            crate::exe::register_native_va(h_memset_1, memset_ptr);
            crate::exe::register_native_va(h_memset_2, memset_ptr);
            crate::exe::register_native_va(h_memset_3, memset_ptr);

            // Helper: build `synth_helper_v.dll` at a given shifted buffer with PE DIR64
            // relocations applied from preferred `ImageBase = 0x1_8000_0000`.
            // Exports:
            //   - 0x1000 (`compute_a(rcx)`): returns `rcx * 3 + 7`
            //   - 0x1040 (`switch_b(rcx = case, rdx = val)`): Clang PIC signed-32 switch table:
            //       0 -> val + 100
            //       1 -> val * 2 + 11
            //       2 -> reads PE-relocated 64-bit constant pointer -> returns `*ptr + val`
            //       3 -> calls host `memset(scratch, 0x2A, 16)` via IAT, returns `*(u32*)scratch + val`
            //       default -> 0xBEEF
            let build_helper_dll = |pe: &mut TestMappedBuf, host_va: u64, work_buf: u64| {
                const PREF: u64 = 0x1_8000_0000;
                let base = pe.base() as u64;
                let delta = base.wrapping_sub(PREF);
                let buf = pe.as_slice_mut();
                buf.fill(0);
                // Export dir @ 0x4000, .pdata @ 0x4200, IAT slot @ 0x4080,
                // Clang signed32 switch table @ 0x4100, PE-relocated 64-bit ptr @ 0x4180 -> 0x4190 (u64=500).
                init_synth_pe(
                    buf,
                    PREF,
                    0x1000,
                    0x1000,
                    0x3000,
                    0x4000,
                    0x3000,
                    Some((0x4000, 0x60)),
                    Some((0x4200, 24)),
                );
                let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
                let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

                // Export directory: 2 exports (0x1000, 0x1040).
                w32(buf, 0x4000 + 20, 2);
                w32(buf, 0x4000 + 28, 0x4030);
                w32(buf, 0x4030, 0x1000);
                w32(buf, 0x4034, 0x1040);

                // IAT slot @ 0x4080 -> host memset VA.
                w64(buf, 0x4080, host_va);
                // PE-relocated 64-bit pointer @ 0x4180 initially holding `PREF + 0x4190`, relocated by `delta`.
                w64(buf, 0x4180, PREF + 0x4190);
                w64(buf, 0x4190, 500);
                // Apply DIR64 base relocation to 0x4180 and update OptionalHeader.ImageBase.
                let orig = u64::from_le_bytes(buf[0x4180..0x4188].try_into().unwrap());
                w64(buf, 0x4180, orig.wrapping_add(delta));
                w64(buf, 0x58 + 24, base);

                // Export 0 @ 0x1000 (`compute_a`): lea rax, [rcx + rcx*2 + 7]; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.lea(rax, ptr(rcx + rcx * 2 + 7)).unwrap();
                a.ret().unwrap();
                let fn0 = a.assemble(base + 0x1000).unwrap();
                buf[0x1000..0x1000 + fn0.len()].copy_from_slice(&fn0);

                // Export 1 @ 0x1040 (`switch_b`): Clang PIC signed-32 switch table at 0x4100.
                let cases: [u32; 4] = [0x10C0, 0x10E0, 0x1100, 0x1140];
                let def_rva: u32 = 0x11A0;
                for (i, &rva) in cases.iter().enumerate() {
                    let rel = (rva as i32) - 0x4100i32;
                    w32(buf, 0x4100 + i * 4, rel as u32);
                }
                let mut sw = Vec::new();
                sw.extend_from_slice(&[0x83, 0xF9, 0x03]); // cmp ecx, 3
                let ja_next = 0x1040 + sw.len() as u32 + 6;
                sw.extend_from_slice(&[0x0F, 0x87]);
                sw.extend_from_slice(&((def_rva as i32) - (ja_next as i32)).to_le_bytes());
                emit_lea_rip(&mut sw, base + 0x1040, 8 /* r8 */, base + 0x4100);
                let mut a = CodeAssembler::new(64).unwrap();
                a.movsxd(rax, dword_ptr(r8 + rcx * 4)).unwrap();
                a.add(rax, r8).unwrap();
                a.jmp(rax).unwrap();
                sw.extend_from_slice(&a.assemble(base + 0x1040 + sw.len() as u64).unwrap());
                buf[0x1040..0x1040 + sw.len()].copy_from_slice(&sw);

                // Case 0 @ 0x10C0: lea rax, [rdx + 100]; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.lea(rax, ptr(rdx + 100)).unwrap();
                a.ret().unwrap();
                let b0 = a.assemble(base + 0x10C0).unwrap();
                buf[0x10C0..0x10C0 + b0.len()].copy_from_slice(&b0);

                // Case 1 @ 0x10E0: lea rax, [rdx*2 + 11]; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.lea(rax, ptr(rdx * 2 + 11)).unwrap();
                a.ret().unwrap();
                let b1 = a.assemble(base + 0x10E0).unwrap();
                buf[0x10E0..0x10E0 + b1.len()].copy_from_slice(&b1);

                // Case 2 @ 0x1100: lea r8, [rip + 0x4180]; mov r8, [r8]; mov rax, [r8]; add rax, rdx; ret
                let mut b2 = Vec::new();
                emit_lea_rip(&mut b2, base + 0x1100, 8 /* r8 */, base + 0x4180);
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(r8, qword_ptr(r8)).unwrap();
                a.mov(rax, qword_ptr(r8)).unwrap();
                a.add(rax, rdx).unwrap();
                a.ret().unwrap();
                b2.extend_from_slice(&a.assemble(base + 0x1100 + b2.len() as u64).unwrap());
                buf[0x1100..0x1100 + b2.len()].copy_from_slice(&b2);

                // Case 3 @ 0x1140: call host memset(work_buf, 0x2A, 16) via IAT @ 0x4080; return *(u8*)work_buf + rdx
                let mut b3 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.push(rbx).unwrap();
                a.sub(rsp, 0x20).unwrap();
                a.mov(rbx, rdx).unwrap();
                a.mov(rcx, work_buf).unwrap();
                a.mov(edx, 0x2A).unwrap();
                a.mov(r8d, 16).unwrap();
                b3.extend_from_slice(&a.assemble(base + 0x1140).unwrap());
                emit_call_rip_slot(&mut b3, base + 0x1140, base + 0x4080);
                let mut a = CodeAssembler::new(64).unwrap();
                a.movzx(eax, byte_ptr(rax)).unwrap();
                a.add(rax, rbx).unwrap();
                a.add(rsp, 0x20).unwrap();
                a.pop(rbx).unwrap();
                a.ret().unwrap();
                b3.extend_from_slice(&a.assemble(base + 0x1140 + b3.len() as u64).unwrap());
                buf[0x1140..0x1140 + b3.len()].copy_from_slice(&b3);

                // Default @ 0x11A0: mov eax, 0xBEEF; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(eax, 0xBEEF).unwrap();
                a.ret().unwrap();
                let bd = a.assemble(base + 0x11A0).unwrap();
                buf[0x11A0..0x11A0 + bd.len()].copy_from_slice(&bd);

                // .pdata
                w32(buf, 0x4200, 0x1000);
                w32(buf, 0x4204, 0x1030);
                w32(buf, 0x4208, 0x4300);
                w32(buf, 0x420C, 0x1040);
                w32(buf, 0x4210, 0x11E0);
                w32(buf, 0x4214, 0x4300);
                buf[0x4300] = 0x01;
            };

            // Helper: build `synth_app_v.exe` at a shifted base (from preferred `0x1_4000_0000`)
            // with PE DIR64 base relocations, MSVC rva32 switch table, cross-module IAT calls
            // to `synth_helper_v.dll`, host-builtin IAT call, and intra-module local helper call.
            // Entry @ 0x1000(rcx = outer_case, rdx = arg1, r8 = arg2):
            //   0 -> calls dll!compute_a(arg1) via IAT @ 0x4080
            //   1 -> calls dll!switch_b(arg1, arg2) via IAT @ 0x4088
            //   2 -> calls host memset(work_buf + 0x100, 0x33, 8) via IAT @ 0x4090, returns *(u8*)ptr + arg1
            //   3 -> direct call to local helper @ 0x1300(arg1) + *(u64*)*(u64*)(reloc_ptr @ 0x4180)
            //   default -> 0xDEAD_0000
            let build_app_exe = |pe: &mut TestMappedBuf,
                                 dll_base: u64,
                                 host_va: u64,
                                 work_buf: u64| {
                const PREF: u64 = 0x1_4000_0000;
                let base = pe.base() as u64;
                let delta = base.wrapping_sub(PREF);
                let buf = pe.as_slice_mut();
                buf.fill(0);
                init_synth_pe(
                    buf,
                    PREF,
                    0x1000,
                    0x1000,
                    0x3000,
                    0x4000,
                    0x3000,
                    None,
                    Some((0x4200, 24)),
                );
                let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
                let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

                // IAT slots @ 0x4080..0x4098:
                w64(buf, 0x4080, dll_base + 0x1000); // dll!compute_a
                w64(buf, 0x4088, dll_base + 0x1040); // dll!switch_b
                w64(buf, 0x4090, host_va); // host memset

                // PE-relocated 64-bit constant pointer @ 0x4180 -> 0x4190 (u64=1000).
                w64(buf, 0x4180, PREF + 0x4190);
                w64(buf, 0x4190, 1000);
                let orig = u64::from_le_bytes(buf[0x4180..0x4188].try_into().unwrap());
                w64(buf, 0x4180, orig.wrapping_add(delta));
                w64(buf, 0x58 + 24, base);

                // MSVC rva32 switch table @ 0x4100:
                let cases: [u32; 4] = [0x1080, 0x10C0, 0x1100, 0x1180];
                let def_rva: u32 = 0x1200;
                for (i, &rva) in cases.iter().enumerate() {
                    w32(buf, 0x4100 + i * 4, rva);
                }

                // Entry @ 0x1000: cmp ecx, 3; ja def; lea r10, [rip + __ImageBase]; mov eax, [r10 + rcx*4 + 0x4100]; add rax, r10; jmp rax
                let mut sw = Vec::new();
                sw.extend_from_slice(&[0x83, 0xF9, 0x03]); // cmp ecx, 3
                let ja_next = 0x1000 + sw.len() as u32 + 6;
                sw.extend_from_slice(&[0x0F, 0x87]);
                sw.extend_from_slice(&((def_rva as i32) - (ja_next as i32)).to_le_bytes());
                emit_lea_rip(&mut sw, base + 0x1000, 10 /* r10 */, base);
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(eax, dword_ptr(r10 + rcx * 4 + 0x4100)).unwrap();
                a.add(rax, r10).unwrap();
                a.jmp(rax).unwrap();
                sw.extend_from_slice(&a.assemble(base + 0x1000 + sw.len() as u64).unwrap());
                buf[0x1000..0x1000 + sw.len()].copy_from_slice(&sw);

                // Case 0 @ 0x1080: sub rsp, 0x28; mov rcx, rdx; call [iat_compute_a]; add rsp, 0x28; ret
                let mut b0 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.sub(rsp, 0x28).unwrap();
                a.mov(rcx, rdx).unwrap();
                b0.extend_from_slice(&a.assemble(base + 0x1080).unwrap());
                emit_call_rip_slot(&mut b0, base + 0x1080, base + 0x4080);
                b0.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
                buf[0x1080..0x1080 + b0.len()].copy_from_slice(&b0);

                // Case 1 @ 0x10C0: sub rsp, 0x28; mov rcx, rdx; mov rdx, r8; call [iat_switch_b]; add rsp, 0x28; ret
                let mut b1 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.sub(rsp, 0x28).unwrap();
                a.mov(rcx, rdx).unwrap();
                a.mov(rdx, r8).unwrap();
                b1.extend_from_slice(&a.assemble(base + 0x10C0).unwrap());
                emit_call_rip_slot(&mut b1, base + 0x10C0, base + 0x4088);
                b1.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]);
                buf[0x10C0..0x10C0 + b1.len()].copy_from_slice(&b1);

                // Case 2 @ 0x1100: call host memset(work_buf + 0x100, 0x33, 8) via IAT @ 0x4090
                let mut b2 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.push(rbx).unwrap();
                a.sub(rsp, 0x20).unwrap();
                a.mov(rbx, rdx).unwrap();
                a.mov(rcx, work_buf + 0x100).unwrap();
                a.mov(edx, 0x33).unwrap();
                a.mov(r8d, 8).unwrap();
                b2.extend_from_slice(&a.assemble(base + 0x1100).unwrap());
                emit_call_rip_slot(&mut b2, base + 0x1100, base + 0x4090);
                let mut a = CodeAssembler::new(64).unwrap();
                a.movzx(eax, byte_ptr(rax)).unwrap();
                a.add(rax, rbx).unwrap();
                a.add(rsp, 0x20).unwrap();
                a.pop(rbx).unwrap();
                a.ret().unwrap();
                b2.extend_from_slice(&a.assemble(base + 0x1100 + b2.len() as u64).unwrap());
                buf[0x1100..0x1100 + b2.len()].copy_from_slice(&b2);

                // Case 3 @ 0x1180: direct call local helper @ 0x1300 + read PE-relocated ptr @ 0x4180
                let mut b3 = Vec::new();
                let mut a = CodeAssembler::new(64).unwrap();
                a.sub(rsp, 0x28).unwrap();
                a.mov(rcx, rdx).unwrap();
                a.call(base + 0x1300).unwrap();
                b3.extend_from_slice(&a.assemble(base + 0x1180).unwrap());
                emit_lea_rip(&mut b3, base + 0x1180, 9 /* r9 */, base + 0x4180);
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(r9, qword_ptr(r9)).unwrap();
                a.add(rax, qword_ptr(r9)).unwrap();
                a.add(rsp, 0x28).unwrap();
                a.ret().unwrap();
                b3.extend_from_slice(&a.assemble(base + 0x1180 + b3.len() as u64).unwrap());
                buf[0x1180..0x1180 + b3.len()].copy_from_slice(&b3);

                // Default @ 0x1200: mov eax, 0xDEAD0000; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(eax, 0xDEAD_0000u32).unwrap();
                a.ret().unwrap();
                let bd = a.assemble(base + 0x1200).unwrap();
                buf[0x1200..0x1200 + bd.len()].copy_from_slice(&bd);

                // Local helper @ 0x1300: lea rax, [rcx * 4 + 5]; ret
                let mut a = CodeAssembler::new(64).unwrap();
                a.lea(rax, ptr(rcx * 4 + 5)).unwrap();
                a.ret().unwrap();
                let hl = a.assemble(base + 0x1300).unwrap();
                buf[0x1300..0x1300 + hl.len()].copy_from_slice(&hl);

                // .pdata
                w32(buf, 0x4200, 0x1000);
                w32(buf, 0x4204, 0x1240);
                w32(buf, 0x4208, 0x4300);
                w32(buf, 0x420C, 0x1300);
                w32(buf, 0x4210, 0x1320);
                w32(buf, 0x4214, 0x4300);
                buf[0x4300] = 0x01;
            };

            let run_all_cases = |exe_entry: u64, mode: u8| -> Vec<u64> {
                let inputs: [(u64, u64, u64); 8] = [
                    (0, 10, 0),    // compute_a(10) = 37
                    (1, 0, 25),    // switch_b(0, 25) = 125
                    (1, 1, 25),    // switch_b(1, 25) = 61
                    (1, 2, 25),    // switch_b(2, 25) = 525
                    (1, 3, 25),    // switch_b(3, 25) = 0x2A + 25 = 67
                    (1, 9, 25),    // switch_b(9, 25) = 0xBEEF
                    (2, 7, 0),     // exe host memset + 7 = 0x33 + 7 = 58
                    (3, 12, 0),    // local helper(12)=53 + 1000 = 1053
                ];
                jit::force(mode);
                let mut out = Vec::with_capacity(inputs.len() + 1);
                for (a0, a1, a2) in inputs {
                    let (r, _) = unsafe { super::emu_run_full(exe_entry, [a0, a1, a2, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
                    out.push(r);
                }
                let (def_r, _) = unsafe { super::emu_run_full(exe_entry, [42, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
                out.push(def_r);
                jit::force(0);
                out
            };

            let expected = vec![37, 125, 61, 525, 67, 0xBEEF, 58, 1053, 0xDEAD_0000];

            // --- Phase 1: Map both modules at shifted/relocated bases & prewarm ---
            let mut dll1 = TestMappedBuf::alloc(0x8000, 0x2_3900_0000);
            let mut exe1 = TestMappedBuf::alloc(0x8000, 0x2_3A00_0000);
            build_helper_dll(&mut dll1, h_memset_1, scratch_va);
            build_app_exe(&mut exe1, dll1.base() as u64, h_memset_1, scratch_va);

            // Confirm interpreter oracle matches expected values.
            assert_eq!(run_all_cases(exe1.base() as u64 + 0x1000, 2), expected);

            module_mapped(exe_name, exe1.base(), exe1.len);
            module_mapped(dll_name, dll1.base(), dll1.len);
            let reps = aot_prewarm_all_mapped(0);
            let exe_rep = reps.iter().find(|r| r.module == exe_name).expect("exe report");
            let dll_rep = reps.iter().find(|r| r.module == dll_name).expect("dll report");
            assert!(exe_rep.cache_path.as_ref().is_some_and(|p| p.is_file()));
            assert!(dll_rep.cache_path.as_ref().is_some_and(|p| p.is_file()));
            assert!(exe_rep.blocks >= 8, "expected exe roots + all 4 switch cases + default, got {}", exe_rep.blocks);
            assert!(dll_rep.blocks >= 8, "expected dll exports + all 4 switch cases + default, got {}", dll_rep.blocks);

            // Move host-builtin IAT slots in BOTH modules to `h_memset_2` before reload to exercise
            // AOT relocation rebasing (`RELOC_HOST_FN` + `RELOC_SLOT_VAL`).
            exe1.as_slice_mut()[0x4090..0x4098].copy_from_slice(&h_memset_2.to_le_bytes());
            dll1.as_slice_mut()[0x4080..0x4088].copy_from_slice(&h_memset_2.to_le_bytes());

            // Reload both modules with full `PORT_AOT_VERIFY=1` verification.
            let v0 = TEST_VERIFIED.with(|c| c.get());
            let m0 = TEST_VERIFY_MISMATCHES.with(|c| c.get());
            let r0 = TEST_REBASED.with(|c| c.get());
            TEST_AOT.with(|c| c.set(true));
            TEST_VERIFY.with(|c| c.set(Some(1)));
            activate();
            let verified1 = TEST_VERIFIED.with(|c| c.get()) - v0;
            let mismatches1 = TEST_VERIFY_MISMATCHES.with(|c| c.get()) - m0;
            let rebased1 = TEST_REBASED.with(|c| c.get()) - r0;
            assert_eq!(mismatches1, 0, "expected 0 verify mismatches on phase-1 reload");
            assert_eq!(
                verified1,
                exe_rep.blocks + dll_rep.blocks,
                "expected every block of both modules verified"
            );
            assert!(
                !jit::plaincall_enabled() || rebased1 >= 2,
                "expected host-builtin IAT relocations rebased in both modules, got {rebased1}"
            );

            // Confirm switch-case blocks in BOTH modules are live in the AOT directory.
            for rva in [0x1000u64, 0x1080, 0x10C0, 0x1100, 0x1180, 0x1200, 0x1300] {
                assert!(find(exe1.base() as u64 + rva).is_some(), "exe block {rva:#x} missing");
            }
            for rva in [0x1000u64, 0x1040, 0x10C0, 0x10E0, 0x1100, 0x1140, 0x11A0] {
                assert!(find(dll1.base() as u64 + rva).is_some(), "dll block {rva:#x} missing");
            }

            // Execute all switch cases via loaded AOT blocks and verify exact results & AOT hits.
            let installs0 = jit::test_aot_installs();
            assert_eq!(run_all_cases(exe1.base() as u64 + 0x1000, 1), expected);
            assert!(
                jit::test_aot_installs() - installs0 > 0,
                "expected loaded AOT blocks to execute"
            );

            // --- Phase 2: Shift BOTH modules to new relocated base addresses ---
            teardown_test_modules(&[exe_name, dll_name]);
            let mut dll2 = TestMappedBuf::alloc(0x8000, 0x2_3C00_0000);
            let mut exe2 = TestMappedBuf::alloc(0x8000, 0x2_3D00_0000);
            assert_ne!(dll2.base(), dll1.base());
            assert_ne!(exe2.base(), exe1.base());
            build_helper_dll(&mut dll2, h_memset_2, scratch_va);
            build_app_exe(&mut exe2, dll2.base() as u64, h_memset_2, scratch_va);

            // Cache-key safety: loading shifted bases before re-prewarming must cleanly miss
            // the base-1 `.aot` files (no stale wrong-base blocks loaded).
            let key_exe2 = unsafe { module_key(exe2.base(), exe2.len) };
            let key_dll2 = unsafe { module_key(dll2.base(), dll2.len) };
            assert!(
                unsafe { load_module(exe_name, exe2.base(), exe2.len, &key_exe2) }.is_none(),
                "shifted exe base must not reuse old-base cache"
            );
            assert!(
                unsafe { load_module(dll_name, dll2.base(), dll2.len, &key_dll2) }.is_none(),
                "shifted dll base must not reuse old-base cache"
            );

            // Cross-module base shift: rebind `exe1`'s IAT slots to `dll2` (shifted DLL base!)
            // while keeping `exe1` on its existing base-1 `.aot` cache and moving host IAT to `h_memset_3`.
            exe1.as_slice_mut()[0x4080..0x4088].copy_from_slice(&(dll2.base() as u64 + 0x1000).to_le_bytes());
            exe1.as_slice_mut()[0x4088..0x4090].copy_from_slice(&(dll2.base() as u64 + 0x1040).to_le_bytes());
            exe1.as_slice_mut()[0x4090..0x4098].copy_from_slice(&h_memset_3.to_le_bytes());

            // Prewarm shifted `dll2` (and shifted `exe2`), move host IAT slots to `h_memset_3`,
            // reload under `PORT_AOT_VERIFY=1`, verify 0 mismatches, and execute!
            module_mapped(exe_name, exe2.base(), exe2.len);
            module_mapped(dll_name, dll2.base(), dll2.len);
            let reps2 = aot_prewarm_all_mapped(0);
            let exe2_rep = reps2.iter().find(|r| r.module == exe_name).expect("exe2 report");
            let dll2_rep = reps2.iter().find(|r| r.module == dll_name).expect("dll2 report");
            assert_eq!(exe2_rep.blocks, exe_rep.blocks);
            assert_eq!(dll2_rep.blocks, dll_rep.blocks);

            exe2.as_slice_mut()[0x4090..0x4098].copy_from_slice(&h_memset_3.to_le_bytes());
            dll2.as_slice_mut()[0x4080..0x4088].copy_from_slice(&h_memset_3.to_le_bytes());

            let v1 = TEST_VERIFIED.with(|c| c.get());
            let m1 = TEST_VERIFY_MISMATCHES.with(|c| c.get());
            activate();
            assert_eq!(TEST_VERIFY_MISMATCHES.with(|c| c.get()) - m1, 0, "0 mismatches on shifted reload");
            assert_eq!(
                TEST_VERIFIED.with(|c| c.get()) - v1,
                exe2_rep.blocks + dll2_rep.blocks,
                "all shifted blocks verified"
            );

            let installs1 = jit::test_aot_installs();
            assert_eq!(run_all_cases(exe2.base() as u64 + 0x1000, 1), expected);
            assert!(jit::test_aot_installs() - installs1 > 0);
            // Also execute `exe1` calling across to shifted `dll2`!
            assert_eq!(run_all_cases(exe1.base() as u64 + 0x1000, 1), expected);

            // Clean teardown.
            TEST_VERIFY.with(|c| c.set(None));
            TEST_AOT.with(|c| c.set(false));
            teardown_test_modules(&[exe_name, dll_name]);
            crate::exe::unregister_native_va(h_memset_1);
            crate::exe::unregister_native_va(h_memset_2);
            crate::exe::unregister_native_va(h_memset_3);
            let _ = std::fs::remove_dir_all(&d);
            drop(scratch_mem);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn bg_job_and_load_skip_runtime_modified_code_regions() {
    // Use an isolated high VA far from any other test's mapped code buffers (SLACK = 16 MiB).
    let base: usize = 0x6_8000_0000;
    let size: usize = 0x10000;
    super::mem::note_code_region(base, base + size);
    assert!(super::mem::jit_overlaps(base, base + size));
    let key = AotKey { image_hash: [0x11, 0x22], image_base: base as u64, image_size: size as u64, codegen_hash: 0x33, knobs: 0 };
    assert!(bg_job("runtime_mod.exe", base, size, key, None).is_none());
    assert!(unsafe { load_module("runtime_mod.exe", base, size, &key) }.is_none());
}

/// Multi-bank AOT image test (using a small test `bank_span` = 16 KiB):
/// - Verifies multiple stub banks emit cleanly with 16-byte alignment and no block overlaps;
/// - Verifies every stub branch in every bank targets its OWN bank's stub offsets within range;
/// - Verifies v6 `.aot` round-trips cleanly and legacy v5 `.aot` decodes & upgrades cleanly;
/// - Verifies single- vs multi-threaded multi-bank builds are bit-for-bit deterministic;
/// - Installs the multi-bank image with `Verify::All` (0 mismatches) and executes functions
///   in every bank against the interpreter oracle.
#[test]
fn aot_multibank_chunk_assembler_roundtrip_determinism_and_exec() {
    use iced_x86::code_asm::*;

    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            const NFUNCS: usize = 160;
            const FUNC_STRIDE: usize = 0x40;
            const TEXT_RVA: u32 = 0x1000;
            const TEXT_SIZE: u32 = 0x4000;
            const RDATA_RVA: u32 = 0x5000;
            const RDATA_SIZE: u32 = 0x2000;
            const IMG_SIZE: usize = 0x8000;
            const TEST_BANK_SPAN: usize = 16 << 10; // 16 KiB per bank -> forces >= 3 banks!

            let mut pe = TestMappedBuf::alloc(IMG_SIZE, 0x2_3E00_0000);
            let base = pe.base();
            let base64 = base as u64;
            let buf = pe.as_slice_mut();
            let pdata_rva: u32 = RDATA_RVA + 0x100;
            init_synth_pe(
                buf,
                base64,
                TEXT_RVA,
                TEXT_RVA,
                TEXT_SIZE,
                RDATA_RVA,
                RDATA_SIZE,
                None,
                Some((pdata_rva, (NFUNCS * 12) as u32)),
            );

            let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
            let unwind_rva: u32 = RDATA_RVA + 0x1800;
            buf[unwind_rva as usize] = 0x01;

            for i in 0..NFUNCS {
                let rva = TEXT_RVA + (i * FUNC_STRIDE) as u32;
                let va = base64 + rva as u64;
                let mut a = CodeAssembler::new(64).unwrap();
                let mut l_alt = a.create_label();
                a.cmp(ecx, (i & 7) as i32).unwrap();
                a.ja(l_alt).unwrap();
                a.lea(eax, ptr(rcx + rdx + (i as i32))).unwrap();
                a.imul_3(eax, eax, ((i as i32) * 2) | 1).unwrap();
                a.ret().unwrap();
                a.set_label(&mut l_alt).unwrap();
                a.mov(eax, edx).unwrap();
                a.add(eax, (i as i32) + 100).unwrap();
                a.ret().unwrap();
                let code = a.assemble(va).unwrap();
                assert!(code.len() <= FUNC_STRIDE);
                buf[rva as usize..rva as usize + code.len()].copy_from_slice(&code);
                let pe_off = pdata_rva as usize + i * 12;
                w32(buf, pe_off, rva);
                w32(buf, pe_off + 4, rva + FUNC_STRIDE as u32);
                w32(buf, pe_off + 8, unwind_rva);
            }

            let roots = unsafe { discover_pe_roots(base, pe.len, 0) };
            assert_eq!(roots.len(), NFUNCS);
            let mut pool1 = WorkerCaches::new();
            let (ips, _) = unsafe { discover_with_threads(&mut pool1, base, pe.len, &roots, DEFAULT_AOT_CLOSURE, 1) };

            let img_1 = unsafe {
                build_image_with_threads_and_limits(&mut pool1, &ips, BUILD_VBASE, 1, TEST_BANK_SPAN, MAX_CODE)
            }
            .expect("1-thread multi-bank build");
            assert!(
                img_1.stub_banks.len() >= 3,
                "expected >=3 stub banks at 16 KiB span, got {}",
                img_1.stub_banks.len()
            );
            assert!(img_1.valid(), "multi-bank image must pass structural validation");
            assert_eq!(img_1.stub_banks[0], 0);
            for w in img_1.stub_banks.windows(2) {
                assert!(w[1] > w[0] && w[1] % 16 == 0);
                assert!((w[1] - w[0]) as usize <= TEST_BANK_SPAN);
            }

            // Verify every block's stub branches target its OWN stub bank within `TEST_BANK_SPAN`.
            let stubs_words = (AOT_STUBS_BYTES / 4) as i64;
            let mut blocks_per_bank = vec![0usize; img_1.stub_banks.len()];
            let mut stub_branches_checked = 0usize;
            for b in &img_1.blocks {
                let bi = img_1.bank_index_of(b.code_off);
                blocks_per_bank[bi] += 1;
                let bank_off = img_1.stub_banks[bi] as i64;
                let bank_w = bank_off / 4;
                // Check hot words and cold words (up to any literal slots; scan all B/BL targeting a stub range).
                let check_slice = |words: &[u32], base_off: u32, count: &mut usize| {
                    let base_w = (base_off as i64) / 4;
                    for (i, &w) in words.iter().enumerate() {
                        if (w & 0x7C00_0000) == 0x1400_0000 {
                            let imm26 = ((((w & 0x03FF_FFFF) as i32) << 6) >> 6) as i64;
                            let target_w = base_w + i as i64 + imm26;
                            if target_w >= bank_w && target_w < bank_w + stubs_words {
                                let stub_w = (target_w - bank_w) as u32;
                                assert!(
                                    img_1.stubs.contains(&stub_w),
                                    "branch into bank {bi} stub region hit non-stub word {stub_w}"
                                );
                                assert!(
                                    imm26.abs() * 4 <= TEST_BANK_SPAN as i64,
                                    "stub branch exceeded bank span: {}",
                                    imm26 * 4
                                );
                                *count += 1;
                            } else {
                                // Must NEVER target a different bank's stub area!
                                for (obi, &obo) in img_1.stub_banks.iter().enumerate() {
                                    if obi != bi {
                                        let obw = (obo as i64) / 4;
                                        assert!(
                                            !(target_w >= obw && target_w < obw + stubs_words),
                                            "block in bank {bi} branched into bank {obi}'s stubs!"
                                        );
                                    }
                                }
                            }
                        }
                    }
                };
                check_slice(img_1.hot_words_of(b), b.code_off, &mut stub_branches_checked);
                check_slice(img_1.cold_words_of(b), b.cold_off, &mut stub_branches_checked);
            }
            assert!(
                blocks_per_bank.iter().all(|&c| c > 0),
                "every bank should hold blocks: {blocks_per_bank:?}"
            );
            assert!(stub_branches_checked > 0, "expected stub branches verified across banks");

            // Determinism: 1, 2, 4 threads must produce bit-for-bit identical v6 `.aot` bytes!
            let key = unsafe { module_key(base, pe.len) };
            let bytes_1 = encode("multibank.dll", &key, &img_1, 0);
            for threads in [2usize, 4] {
                let mut pool_p = WorkerCaches::new();
                let img_p = unsafe {
                    build_image_with_threads_and_limits(&mut pool_p, &ips, BUILD_VBASE, threads, TEST_BANK_SPAN, MAX_CODE)
                }
                .expect("parallel multi-bank build");
                assert_eq!(img_p.stub_banks, img_1.stub_banks);
                assert_eq!(
                    encode("multibank.dll", &key, &img_p, 0),
                    bytes_1,
                    "multi-bank .aot differed at {threads} threads"
                );
            }

            // Round-trip v6 decode AND legacy v5 single-bank decode upgrade.
            let (name_dec, key_dec, img_dec, flags_dec) = decode(&bytes_1).expect("decode v6 multi-bank");
            assert_eq!(name_dec, "multibank.dll");
            assert_eq!(key_dec, key);
            assert_eq!(flags_dec, 0);
            assert_eq!(img_dec.stub_banks, img_1.stub_banks);
            assert_eq!(img_dec.blocks, img_1.blocks);
            assert_eq!(img_dec.code, img_1.code);

            let single_img = unsafe { build_image(pool1.get_one(), &ips[..4], BUILD_VBASE) }.expect("single-bank");
            let v5_bytes = encode_with_version("legacy_v5.dll", &key, &single_img, FLAG_VERIFY_PENDING, FORMAT_VERSION_V5);
            let (v5_name, v5_key, v5_img, v5_flags) = decode(&v5_bytes).expect("decode legacy v5");
            assert_eq!(v5_name, "legacy_v5.dll");
            assert_eq!(v5_key, key);
            assert_eq!(v5_flags, FLAG_VERIFY_PENDING);
            assert_eq!(v5_img.stub_banks, vec![0]);
            assert_eq!(v5_img.blocks, single_img.blocks);

            // Install the multi-bank image with Verify::All, then execute functions from Bank 0,
            // Bank 1, and the last bank comparing AOT output against the interpreter oracle!
            TEST_AOT.with(|c| c.set(true));
            let total_blocks = img_dec.blocks.len();
            let inst = unsafe { install_image("multibank.dll", base, pe.len, img_dec, Verify::All, true) }
                .expect("install multi-bank image");
            assert_eq!(inst.mismatches, 0, "multi-bank Verify::All had mismatches");
            assert_eq!(inst.verified, total_blocks);
            assert_eq!(inst.live, total_blocks);

            // Pick one function entry IP from each bank and execute both AOT and interpreter.
            let sample_funcs = [0usize, NFUNCS / 2, NFUNCS - 1];
            for fi in sample_funcs {
                let entry = base64 + TEXT_RVA as u64 + (fi * FUNC_STRIDE) as u64;
                assert!(find(entry).is_some(), "func {fi} missing from AOT dir");
                for (a0, a1) in [(2u64, 15u64), (20u64, 33u64)] {
                    jit::force(2);
                    let (exp, _) = unsafe { super::emu_run_full(entry, [a0, a1, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
                    jit::force(1);
                    let (got, _) = unsafe { super::emu_run_full(entry, [a0, a1, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
                    jit::force(0);
                    assert_eq!(got, exp, "multi-bank func {fi} mismatch for args ({a0}, {a1})");
                }
            }

            test_uninstall();
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Deep CFG closure + `.pdata`-less leaf callees (direct call & tail-jump) + TLS/CFG Load Config
/// root harvesting + switch/jump-table dispatch re-analysis & register const/sub/shl hints.
#[test]
fn aot_deep_closure_leaf_callees_tls_cfg_roots_and_jt_reanalysis() {
    use iced_x86::code_asm::*;

    let mut pe = TestMappedBuf::alloc(0xA000, 0x2_3F00_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    // .text @ 0x1000..0x5000, .rdata @ 0x5000..0x9000, .pdata @ 0x6000 (3 entries = 36 bytes)
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x4000, 0x5000, 0x4000, None, Some((0x6000, 36)));
    let opt = 0x58;
    // DataDirectory[9]: TLS Directory @ 0x6100 (40 bytes)
    w32(buf, opt + 112 + 9 * 8, 0x6100);
    w32(buf, opt + 112 + 9 * 8 + 4, 40);
    // DataDirectory[10]: Load Config Directory @ 0x6200 (DataDirectory.Size=0x40 like link.exe, struct Size=0xC0)
    w32(buf, opt + 112 + 10 * 8, 0x6200);
    w32(buf, opt + 112 + 10 * 8 + 4, 0x40);

    // --- 1. Deep `.pdata` function @ 0x1000: 8-hop chain calling `.pdata`-less leaf @ 0x3000
    //        and tail-jumping to `.pdata`-less leaf @ 0x3100 ---
    let mut a = CodeAssembler::new(64).unwrap();
    let mut hops: Vec<CodeLabel> = (0..8).map(|_| a.create_label()).collect();
    for i in 0..8 {
        a.cmp(ecx, i as i32).unwrap();
        let lbl = hops[i];
        a.je(lbl).unwrap();
    }
    a.call(base64 + 0x3000).unwrap();
    a.jmp(base64 + 0x3100).unwrap();
    for (i, lbl) in hops.iter_mut().enumerate() {
        a.set_label(lbl).unwrap();
        a.mov(eax, (100 + i) as u32).unwrap();
        a.ret().unwrap();
    }
    let fn0 = a.assemble(base64 + 0x1000).unwrap();
    buf[0x1000..0x1000 + fn0.len()].copy_from_slice(&fn0);

    // `.pdata`-less leaf callee @ 0x3000 (calls nested `.pdata`-less leaf @ 0x3040):
    let mut a = CodeAssembler::new(64).unwrap();
    a.call(base64 + 0x3040).unwrap();
    a.add(eax, 1).unwrap();
    a.ret().unwrap();
    let leaf0 = a.assemble(base64 + 0x3000).unwrap();
    buf[0x3000..0x3000 + leaf0.len()].copy_from_slice(&leaf0);
    buf[0x3040..0x3046].copy_from_slice(&[0xB8, 7, 0, 0, 0, 0xC3]); // mov eax, 7; ret
    // `.pdata`-less tail-call leaf @ 0x3100:
    buf[0x3100..0x3106].copy_from_slice(&[0xB8, 9, 0, 0, 0, 0xC3]);

    // --- 2. Switch dispatch re-analysis @ 0x1800 ---
    //    Block 0x1800 (guard): cmp ecx, 2; lea r8, [rip + tbl_re]; jbe 0x1840; ret
    //    Block 0x1840 (dispatch): movsxd rax, [r8 + rcx*4]; add rax, r8; jmp rax
    //    We seed BOTH 0x1840 (unhinted first!) and 0x1800 to force re-analysis of 0x1840!
    let re_cases: [u32; 3] = [0x1880, 0x1890, 0x18A0];
    let tbl_re_rva: u32 = 0x5100;
    for (i, &rva) in re_cases.iter().enumerate() {
        let rel = (rva as i32) - (tbl_re_rva as i32);
        w32(buf, tbl_re_rva as usize + i * 4, rel as u32);
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (40 + i as u8), 0, 0, 0, 0xC3]);
    }
    let mut c2 = Vec::new();
    c2.extend_from_slice(&[0x83, 0xF9, 0x02]); // cmp ecx, 2
    emit_lea_rip(&mut c2, base64 + 0x1800, 8 /* r8 */, base64 + tbl_re_rva as u64);
    let jbe_next = 0x1800 + c2.len() as u32 + 6;
    c2.extend_from_slice(&[0x0F, 0x86]); // jbe 0x1840
    c2.extend_from_slice(&((0x1840i32) - (jbe_next as i32)).to_le_bytes());
    c2.push(0xC3); // ret
    buf[0x1800..0x1800 + c2.len()].copy_from_slice(&c2);

    let mut a = CodeAssembler::new(64).unwrap();
    a.movsxd(rax, dword_ptr(r8 + rcx * 4)).unwrap();
    a.add(rax, r8).unwrap();
    a.jmp(rax).unwrap();
    let disp2 = a.assemble(base64 + 0x1840).unwrap();
    buf[0x1840..0x1840 + disp2.len()].copy_from_slice(&disp2);

    // --- 3. Switch with `sub` + const-register `cmp` + `shl rcx, 2` + `sub r8, rax` ---
    let sub_cases: [u32; 3] = [0x2080, 0x2090, 0x20A0];
    let tbl_sub_rva: u32 = 0x5200;
    for (i, &rva) in sub_cases.iter().enumerate() {
        // Positive backward distance from tbl_sub_rva: target = tbl_va - entry!
        let dist = tbl_sub_rva - rva;
        w32(buf, tbl_sub_rva as usize + i * 4, dist);
        buf[rva as usize..rva as usize + 6].copy_from_slice(&[0xB8, (50 + i as u8), 0, 0, 0, 0xC3]);
    }
    // Decoy 4th entry (must NOT be followed because `mov edx, 2; cmp ecx, edx; ja default` caps at 3).
    w32(buf, tbl_sub_rva as usize + 12, tbl_sub_rva - 0x20B0);
    buf[0x20B0..0x20B6].copy_from_slice(&[0xB8, 0xEE, 0, 0, 0, 0xC3]);
    buf[0x20C0..0x20C6].copy_from_slice(&[0xB8, 0xFF, 0, 0, 0, 0xC3]); // default

    let mut c3 = Vec::new();
    let mut a = CodeAssembler::new(64).unwrap();
    a.sub(ecx, 10).unwrap();
    a.mov(edx, 2).unwrap();
    a.cmp(ecx, edx).unwrap(); // register const cmp!
    c3.extend_from_slice(&a.assemble(base64 + 0x2000).unwrap());
    let ja_next = 0x2000 + c3.len() as u32 + 6;
    c3.extend_from_slice(&[0x0F, 0x87]); // ja 0x20C0
    c3.extend_from_slice(&((0x20C0i32) - (ja_next as i32)).to_le_bytes());
    emit_lea_rip(&mut c3, base64 + 0x2000, 8 /* r8 */, base64 + tbl_sub_rva as u64);
    let mut a = CodeAssembler::new(64).unwrap();
    a.shl(rcx, 2).unwrap(); // pre-scaled index!
    a.mov(eax, dword_ptr(r8 + rcx)).unwrap();
    a.sub(r8, rax).unwrap(); // sub_from_base!
    a.jmp(r8).unwrap();
    c3.extend_from_slice(&a.assemble(base64 + 0x2000 + c3.len() as u64).unwrap());
    buf[0x2000..0x2000 + c3.len()].copy_from_slice(&c3);

    // --- 4. TLS callbacks (0x3200, 0x3240) & CFG GuardCF/LongJump targets (0x3300, 0x3340, 0x3380) ---
    for rva in [0x3200usize, 0x3240, 0x3300, 0x3340, 0x3380] {
        buf[rva..rva + 6].copy_from_slice(&[0xB8, 1, 0, 0, 0, 0xC3]);
    }
    // TLS Directory @ 0x6100 -> callback array @ 0x6140
    w64(buf, 0x6100 + 24, base64 + 0x6140);
    w64(buf, 0x6140, base64 + 0x3200);
    w64(buf, 0x6148, base64 + 0x3240);
    w64(buf, 0x6150, 0);

    // Load Config @ 0x6200: GuardCFFunctionTable @ 0x6300 with 5-byte entries (extra=1) +
    // GuardLongJumpTargetTable @ 0x6340.
    w32(buf, 0x6200, 0xC0); // Size
    w64(buf, 0x6200 + 0x80, base64 + 0x6300);
    w64(buf, 0x6200 + 0x88, 2);
    w32(buf, 0x6200 + 0x90, 0x0000_0400 | 0x0001_0000 | (1 << 28)); // CF_TABLE | LONGJUMP | extra=1
    w64(buf, 0x6200 + 0xB0, base64 + 0x6340);
    w64(buf, 0x6200 + 0xB8, 1);
    w32(buf, 0x6300, 0x3300);
    buf[0x6304] = 0x01;
    w32(buf, 0x6305, 0x3340); // intentionally unaligned 5-byte stride!
    buf[0x6309] = 0x02;
    w32(buf, 0x6340, 0x3380);

    // .pdata entries: cover ONLY functions 0x1000..0x1400, 0x1800..0x1900, 0x2000..0x2100.
    for (i, (s, e)) in [(0x1000u32, 0x1400u32), (0x1800, 0x1900), (0x2000, 0x2100)].iter().enumerate() {
        w32(buf, 0x6000 + i * 12, *s);
        w32(buf, 0x6000 + i * 12 + 4, *e);
        w32(buf, 0x6000 + i * 12 + 8, 0x6080);
    }
    buf[0x6080] = 0x01;

    // 1. Verify `discover_pe_roots` harvests `.pdata` + TLS callbacks + CFG function/longjmp tables!
    let roots = unsafe { discover_pe_roots(base, pe.len, 0) };
    assert_eq!(
        roots,
        vec![0x1000, 0x1800, 0x2000, 0x3200, 0x3240, 0x3300, 0x3340, 0x3380],
        "expected .pdata + TLS + CFG roots"
    );

    // 2. Verify shallow depth 2 misses the deep chain's leaf callees, whereas default depth 128
    //    discovers all leaf callees (direct call, nested leaf call, and tail-jump leaf).
    let mut bc = Box::new(BlockCache::new());
    let (shallow_ips, shallow_cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000], 2) };
    assert!(!shallow_ips.contains(&(base64 + 0x3000)), "depth 2 should not reach 8-hop leaf callee");
    assert_eq!(shallow_cl.leaf_callees, 0);

    let (deep_ips, deep_cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000], DEFAULT_AOT_CLOSURE) };
    assert!(deep_ips.contains(&(base64 + 0x3000)), "deep closure must reach direct `.pdata`-less leaf callee");
    assert!(deep_ips.contains(&(base64 + 0x3040)), "deep closure must follow nested `.pdata`-less leaf callee");
    assert!(deep_ips.contains(&(base64 + 0x3100)), "deep closure must reach tail-jump `.pdata`-less leaf");
    assert!(deep_cl.leaf_callees >= 2, "expected >=2 leaf callees, got {}", deep_cl.leaf_callees);

    // 3. Verify switch dispatch re-analysis when `0x1840` is seeded unhinted BEFORE `0x1800`
    //    reaches it with `r8 = Const(tbl_re)` + `rcx = Index { bound: Some(3) }`!
    let (re_ips, re_cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x1840, 0x1800], DEFAULT_AOT_CLOSURE) };
    assert_eq!(re_cl.jt_tables, 1, "re-analyzed switch dispatch should be discovered once");
    assert_eq!(re_cl.jt_targets, 3);
    for &rva in &re_cases {
        assert!(re_ips.contains(&(base64 + rva as u64)), "missing re-analyzed switch case {rva:#x}");
    }

    // 4. Verify `sub` + const-register `cmp` + `shl` scaled-index + `sub rBase, rOff` jump table!
    let (sub_ips, sub_cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x2000], DEFAULT_AOT_CLOSURE) };
    assert_eq!(sub_cl.jt_tables, 1, "expected sub/shl/const-cmp switch table discovered");
    assert_eq!(sub_cl.jt_targets, 3);
    for &rva in &sub_cases {
        assert!(sub_ips.contains(&(base64 + rva as u64)), "missing sub/shl switch case {rva:#x}");
    }
    assert!(!sub_ips.contains(&(base64 + 0x20B0)), "out-of-bounds decoy past const-reg cmp must NOT be followed");
}

/// PE Base Relocation (`DataDirectory[5]`, `IMAGE_REL_BASED_DIR64`) vtable/function-pointer
/// root discovery, `PORT_AOT_RELOC_ROOTS=0` gating, rejection of non-code / padding / invalid
/// targets, and stripped-`.reloc` fallback scan of initialized data sections.
#[test]
fn aot_base_reloc_dir64_vtable_roots_and_stripped_fallback() {
    let _env = TestEnvGuard::new(&["PORT_AOT_RELOC_ROOTS"]);
    std::env::remove_var("PORT_AOT_RELOC_ROOTS");

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_4000_0000);
    let base = pe.base();
    let base64 = base as u64;
    let opt = 0x58usize;
    let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

    {
        let buf = pe.as_slice_mut();

        // .text @ 0x1000..0x4000, .rdata @ 0x4000..0x7000, .pdata @ 0x5000 (1 entry = 12 bytes).
        init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x5000, 12)));
        w32(buf, 0x5000, 0x1000);
        w32(buf, 0x5004, 0x1040);
        w32(buf, 0x5008, 0x5040);
        buf[0x5040] = 0x01; // UNWIND_INFO v1

        // Entry function @ 0x1000 (in .pdata).
        buf[0x1000..0x1006].copy_from_slice(&[0xB8, 1, 0, 0, 0, 0xC3]); // mov eax, 1; ret
        // `.pdata`-less leaf virtual method @ 0x1200: mov al, 1; ret
        buf[0x1200..0x1203].copy_from_slice(&[0xB0, 0x01, 0xC3]);
        // `.pdata`-less adjustor thunk @ 0x1203 (immediately after `ret`, not 16-byte aligned):
        // sub rcx, 8; ret
        buf[0x1203..0x1208].copy_from_slice(&[0x48, 0x83, 0xE9, 0x08, 0xC3]);
        // Padding / invalid targets in `.text` that must be rejected:
        buf[0x1240..0x1244].copy_from_slice(&[0xCC, 0xCC, 0xCC, 0xCC]); // int3 padding
        // 0x1260..0x1268 left zeroed (`0x00, 0x00` padding)
        buf[0x1280..0x1284].copy_from_slice(&[0x06, 0x07, 0x0E, 0x16]); // invalid 64-bit opcode (PUSH ES)
        // Additional valid 16-byte-aligned leaf @ 0x12C0 (pointed to by non-DIR64 relocs only):
        buf[0x12C0..0x12C3].copy_from_slice(&[0xB0, 0x02, 0xC3]);
        // Unaligned non-boundary code @ 0x1305 (preceded by 0x42, NOT 0xCC/0x90/0xC3):
        buf[0x1304] = 0x42;
        buf[0x1305..0x130B].copy_from_slice(&[0xB8, 3, 0, 0, 0, 0xC3]);

        // Vtable / pointer slots in `.rdata` @ 0x4100:
        w64(buf, 0x4100, base64 + 0x1200); // valid leaf
        w64(buf, 0x4108, base64 + 0x1203); // valid adjustor thunk
        w64(buf, 0x4110, base64 + 0x4800); // non-code `.rdata` pointer
        w64(buf, 0x4118, base64 + 0x1240); // `0xCC` padding
        w64(buf, 0x4120, base64 + 0x1260); // `[0x00, 0x00]` padding
        w64(buf, 0x4128, base64 + 0x1280); // invalid opcode
        w64(buf, 0x4130, base64 + 0x12C0); // valid leaf, only referenced by ABSOLUTE/HIGHLOW relocs

        // Base Relocation Directory (`DataDirectory[5]`) @ 0x6000:
        // Block 0: page_rva = 0x4000, 8 u16 entries (block_size = 24), followed by zero block (8B).
        w32(buf, opt + 112 + 5 * 8, 0x6000);
        w32(buf, opt + 112 + 5 * 8 + 4, 32);
        w32(buf, 0x6000, 0x4000); // page_rva
        w32(buf, 0x6004, 24); // block_size
        let relocs: [u16; 8] = [
            (10 << 12) | 0x100, // DIR64 -> 0x1200 (accept)
            (10 << 12) | 0x108, // DIR64 -> 0x1203 (accept)
            (10 << 12) | 0x110, // DIR64 -> .rdata (reject)
            (10 << 12) | 0x118, // DIR64 -> 0xCC (reject)
            (10 << 12) | 0x120, // DIR64 -> 0x00,0x00 (reject)
            (10 << 12) | 0x128, // DIR64 -> invalid opcode (reject)
            (0 << 12) | 0x130,  // ABSOLUTE padding -> ignore
            (3 << 12) | 0x130,  // HIGHLOW -> ignore
        ];
        for (i, &ent) in relocs.iter().enumerate() {
            w16(buf, 0x6008 + i * 2, ent);
        }
        // Trailing zero block (`block_size = 0`) at 0x6018 must terminate cleanly without looping.
    }

    let roots = unsafe { discover_pe_roots(base, pe.len, 0) };
    assert_eq!(
        roots,
        vec![0x1000, 0x1200, 0x1203],
        "expected .pdata root + DIR64 valid code targets only"
    );

    // Gating with `PORT_AOT_RELOC_ROOTS=0` disables `.reloc` root harvesting.
    std::env::set_var("PORT_AOT_RELOC_ROOTS", "0");
    let gated = unsafe { discover_pe_roots(base, pe.len, 0) };
    assert_eq!(gated, vec![0x1000], "PORT_AOT_RELOC_ROOTS=0 must disable reloc root discovery");
    std::env::remove_var("PORT_AOT_RELOC_ROOTS");

    // Stripped `.reloc` fallback: clear DataDirectory[5] and add a pointer to unaligned non-boundary
    // code (`0x1305`); fallback must recover clean entries (0x1200, 0x1203, 0x12C0) and reject 0x1305.
    {
        let buf = pe.as_slice_mut();
        w32(buf, opt + 112 + 5 * 8, 0);
        w32(buf, opt + 112 + 5 * 8 + 4, 0);
        w64(buf, 0x4138, base64 + 0x1305);
    }
    let fallback_roots = unsafe { discover_pe_roots(base, pe.len, 0) };
    assert_eq!(
        fallback_roots,
        vec![0x1000, 0x1200, 0x1203, 0x12C0],
        "stripped-.reloc fallback must find clean-boundary code pointers and reject unaligned/padding targets"
    );
}

/// `LEA reg, [rip + disp32]` discovery of `.pdata`-less leaf callbacks/methods via
/// [`EdgeKind::LeaLeaf`] (both with `jt_on = true` and `jt_on = false`), and rejection of
/// `.pdata`-covered targets, non-code sections, padding, invalid encodings, and unaligned
/// non-boundary data constants embedded in `.text`.
#[test]
fn aot_lea_rip_leaf_discovery_and_text_data_rejection() {
    let _env = TestEnvGuard::new(&["PORT_AOT_JUMPTABLES", "AW2_DLL_AOT_JUMPTABLES"]);
    std::env::remove_var("PORT_AOT_JUMPTABLES");
    std::env::remove_var("AW2_DLL_AOT_JUMPTABLES");

    let mut pe = TestMappedBuf::alloc(0x8000, 0x2_4100_0000);
    let base = pe.base();
    let base64 = base as u64;
    let buf = pe.as_slice_mut();
    let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());

    // .text @ 0x1000..0x4000, .rdata @ 0x4000..0x7000, .pdata @ 0x5000 (2 entries = 24 bytes).
    init_synth_pe(buf, base64, 0x1000, 0x1000, 0x3000, 0x4000, 0x3000, None, Some((0x5000, 24)));
    for (i, (s, e)) in [(0x1000u32, 0x1100u32), (0x1200, 0x1240)].iter().enumerate() {
        w32(buf, 0x5000 + i * 12, *s);
        w32(buf, 0x5000 + i * 12 + 4, *e);
        w32(buf, 0x5000 + i * 12 + 8, 0x5080);
    }
    buf[0x5080] = 0x01;

    // 1. Valid `.pdata`-less 16-byte-aligned leaf @ 0x2000: `mov al, 1; ret`
    buf[0x2000..0x2003].copy_from_slice(&[0xB0, 0x01, 0xC3]);
    // 2. Valid `.pdata`-less leaf @ 0x2045 (not 16-byte aligned, preceded by `0xCC`): `xor eax, eax; ret`
    buf[0x2044] = 0xCC;
    buf[0x2045..0x2048].copy_from_slice(&[0x31, 0xC0, 0xC3]);
    // 3. `.pdata`-covered function @ 0x1200 (must NOT be added via LeaLeaf when seeded from 0x1000):
    buf[0x1200..0x1206].copy_from_slice(&[0xB8, 7, 0, 0, 0, 0xC3]);
    // 4. `.text` data constants that MUST be rejected:
    buf[0x2100..0x2104].copy_from_slice(&[0xCC, 0xCC, 0xCC, 0xCC]); // `0xCC` padding
    // 0x2120 left zeroed (`[0x00, 0x00]` constant)
    buf[0x2140..0x2144].copy_from_slice(&[0x06, 0x07, 0x0E, 0x16]); // invalid 64-bit opcode
    buf[0x2162] = 0x55; // non-boundary preceding byte (not 0xCC/0x90/0xC3)
    buf[0x2163..0x2166].copy_from_slice(&[0xB0, 0x01, 0xC3]); // unaligned data constant in `.text`
    // 5. `.rdata` bytes @ 0x4200 (outside `.text`):
    buf[0x4200..0x4203].copy_from_slice(&[0xB0, 0x01, 0xC3]);

    // Caller @ 0x1000 loads addresses via RIP-relative `LEA` without calling any of them.
    let mut c = Vec::new();
    emit_lea_rip(&mut c, base64 + 0x1000, 0 /* rax */, base64 + 0x2000);
    emit_lea_rip(&mut c, base64 + 0x1000, 1 /* rcx */, base64 + 0x2045);
    emit_lea_rip(&mut c, base64 + 0x1000, 2 /* rdx */, base64 + 0x1200);
    emit_lea_rip(&mut c, base64 + 0x1000, 8 /* r8  */, base64 + 0x2100);
    emit_lea_rip(&mut c, base64 + 0x1000, 9 /* r9  */, base64 + 0x2120);
    emit_lea_rip(&mut c, base64 + 0x1000, 10 /* r10 */, base64 + 0x2140);
    emit_lea_rip(&mut c, base64 + 0x1000, 11 /* r11 */, base64 + 0x2163);
    emit_lea_rip(&mut c, base64 + 0x1000, 0 /* rax */, base64 + 0x4200);
    c.push(0xC3); // ret
    buf[0x1000..0x1000 + c.len()].copy_from_slice(&c);

    let mut bc = Box::new(BlockCache::new());
    for jt_flag in ["1", "0"] {
        std::env::set_var("PORT_AOT_JUMPTABLES", jt_flag);
        let (ips, cl) = unsafe { discover(&mut *bc, base, pe.len, &[0x1000], DEFAULT_AOT_CLOSURE) };
        assert_eq!(cl.lea_leaves, 2, "jt={jt_flag}: expected exactly 2 LEA leaf functions");
        assert!(ips.contains(&(base64 + 0x2000)), "jt={jt_flag}: missing 16B-aligned LEA leaf");
        assert!(ips.contains(&(base64 + 0x2045)), "jt={jt_flag}: missing post-0xCC LEA leaf");
        for rejected in [0x1200u64, 0x2100, 0x2120, 0x2140, 0x2163, 0x4200] {
            assert!(
                !ips.contains(&(base64 + rejected)),
                "jt={jt_flag}: target {rejected:#x} must NOT be falsely discovered"
            );
        }
    }
}
