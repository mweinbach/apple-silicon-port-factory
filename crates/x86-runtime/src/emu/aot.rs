//! AOT translation cache for mapped PE images (`PORT_AOT` / `AW2_DLL_AOT`).
//! Translator side: `jit/aot.rs`.
//!
//! Submodules:
//! - `format`: `.aot` v5–v7 binary encoding/decoding, image/codegen hashing, cache-file lookup, multi-bank `Image`
//! - `cfg_discover`: PE32+ root discovery (`.pdata`, EntryPoint, Exports, TLS, CFG, DIR64 relocs) & parallel static CFG closure
//! - `jumptable`: RIP-relative switch / jump-table abstract interpretation
//! - `builder`: parallel block assembly, multi-bank chunk packing, `aot_prewarm_all_mapped`, `VerifyAotBatch`
//! - `runtime`: live AOT module directory, lock-free shared probe table, linking/revalidation/revival, `activate`
//! - `bg`: crash-loop guard, LRU cache eviction, background rebuild worker, JIT block profiling

mod bg;
mod builder;
pub mod callbacks;
mod cfg_discover;
mod format;
mod jumptable;
mod runtime;
#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

pub use bg::profile_dump;
pub(super) use bg::profile_note;
pub use builder::{aot_prewarm_all_mapped, AotPrewarmReport};
pub(super) use builder::VerifyAotBatch;
#[cfg(test)]
pub(super) use builder::{build_image_for_test, build_image_for_test_with_limits};
pub use callbacks::{
    discover_all_callback_roots, flush_runtime_callbacks, image_fingerprint,
    record_runtime_callback, CallbackKind, CallbackRoot, ImageFingerprint,
};
pub(super) use cfg_discover::discover_blocks_for_verify;
pub use format::aot_cached_preferred_base;
#[cfg_attr(not(test), allow(unused_imports))]
pub(super) use format::{codegen_hash, decode, AotKey};
#[cfg(test)]
pub(super) use format::encode_for_test;
pub use runtime::{activate, module_mapped};
pub(crate) use runtime::resolve_guest_va;
pub(super) use runtime::{
    active, counts, epoch, find, note_code_region, probe_table, revalidate, was_killed_at,
};
#[cfg(test)]
pub(super) use runtime::revive_enabled;
#[cfg(test)]
pub(super) use runtime::{
    test_dead, test_held, test_install, test_probe_check, test_rebased, test_set_revive,
    test_uninstall,
};

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

/// Conservative worker-thread count for live background `.aot` builds (`PORT_AOT=2`),
/// leaving plenty of headroom for active game threads while still parallelizing builds.
fn bg_threads() -> usize {
    let cores = std::thread::available_parallelism().map_or(1, |n| n.get());
    let conservative = if cores > 1 { (cores / 2).clamp(2, 4) } else { 1 };
    conservative.min(prewarm_threads()).max(1)
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
pub(super) const DEFAULT_AOT_CLOSURE: u32 = 128;

fn aot_closure_depth() -> u32 {
    env_u32_any(&["PORT_AOT_CLOSURE", "AW2_DLL_AOT_CLOSURE"], DEFAULT_AOT_CLOSURE)
}

/// Permanently set when running as a guest-spawned child runner (`PORT_CHILD_PROCESS=1`) so
/// disk-sourced native `.aot` caches, fallback roots, sidecars, and background builds are
/// bypassed completely and execution stays on the in-memory translator.
static DISK_AOT_DISABLED: AtomicBool = AtomicBool::new(false);

/// Permanently disable all disk-sourced native AOT caches, fallback roots, callback sidecars,
/// profiles, and background cache builds for this process.
pub fn set_disk_aot_disabled(disabled: bool) {
    if disabled {
        DISK_AOT_DISABLED.store(true, Ordering::Release);
    }
}

/// `true` when disk-sourced native AOT caches and sidecars are disabled for this process
/// (either latched via [`set_disk_aot_disabled`] or signalled by `PORT_CHILD_PROCESS=1`).
pub fn disk_aot_disabled() -> bool {
    DISK_AOT_DISABLED.load(Ordering::Acquire)
        || std::env::var_os("PORT_CHILD_PROCESS").is_some_and(|v| v == "1")
}

/// `PORT_AOT` / `AW2_DLL_AOT`: 0 = JIT only, 1 = load valid caches, 2 = load valid caches and
/// build missing (installed when done) or outgrown (for the next run) ones from the profile on a
/// background thread (implies `PORT_JIT_PROFILE=1`). Unset: 2 for game runs, 0 for
/// `--diff-test` / `--no-run` / `--prewarm-aot` and unit tests. Forced to 0 for one run by the
/// crash-loop guard (`bg::session_check`) and permanently forced to 0 when [`disk_aot_disabled`].
pub(super) fn aot_mode() -> u32 {
    #[cfg(test)]
    if TEST_AOT.with(|c| c.get()) {
        return 1;
    }
    if disk_aot_disabled() {
        return 0;
    }
    static V: OnceLock<u32> = OnceLock::new();
    *V.get_or_init(|| {
        if disk_aot_disabled() || crate::diag::jit_filter_active() || crate::diag::nan_trace_has_static_ranges() {
            return 0;
        }
        let set = std::env::var_os("PORT_AOT").is_some() || std::env::var_os("AW2_DLL_AOT").is_some();
        let mut v = env_u32_any(
            &["PORT_AOT", "AW2_DLL_AOT"],
            if !cfg!(test) && game_run() && build_request().is_none() { 2 } else { 0 },
        );
        if v != 0 && !bg::session_begin() {
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
    if disk_aot_disabled() {
        return false;
    }
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| {
        let on = !disk_aot_disabled()
            && (env_u32_any(&["PORT_JIT_PROFILE", "AW2_JIT_PROFILE"], 0) != 0 || aot_mode() == 2);
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
    if disk_aot_disabled() {
        return None;
    }
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
    if disk_aot_disabled() {
        return Vec::new();
    }
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

/// First activation with AOT on (start of the [`bg_delay`] for next-run rebuilds / sweep).
static ACTIVATED: OnceLock<std::time::Instant> = OnceLock::new();

/// Time left until [`bg_delay`] after the first activation.
fn bg_wait() -> std::time::Duration {
    bg_delay().saturating_sub(ACTIVATED.get().map_or(std::time::Duration::ZERO, |t| t.elapsed()))
}

extern "C" {
    fn pthread_set_qos_class_self_np(qos: u32, rel: i32) -> i32;
}

const QOS_CLASS_UTILITY: u32 = 0x11;
const QOS_CLASS_BACKGROUND: u32 = 0x09;

fn set_qos(q: u32) {
    unsafe { pthread_set_qos_class_self_np(q, 0) };
}
