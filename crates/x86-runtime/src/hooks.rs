//! Pluggable per-game extension hooks for `x86-runtime`.
//!
//! Keeps `x86-runtime` 100% free of game-specific VAs, hand-decompiled namespaces,
//! and static shards. A game crate (such as `aw2-game`) registers its tables once at
//! startup; generic PE runners leave every hook empty.

use crate::registry_types::Replacement;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{OnceLock, RwLock};

static FLOAT_RETURN_VAS: RwLock<Vec<u64>> = RwLock::new(Vec::new());
static HAS_FLOAT_RETURN_VAS: AtomicBool = AtomicBool::new(false);
static LEAN_HOST_PROVIDER: OnceLock<fn() -> Vec<usize>> = OnceLock::new();
static PLAIN_NATIVE_CHECKS: RwLock<Vec<fn(usize) -> bool>> = RwLock::new(Vec::new());
static HAS_PLAIN_NATIVE_CHECKS: AtomicBool = AtomicBool::new(false);
static INLINE_VARIANT_HOOKS: RwLock<Vec<fn(usize) -> Option<(usize, bool)>>> = RwLock::new(Vec::new());
static HAS_INLINE_VARIANT_HOOKS: AtomicBool = AtomicBool::new(false);
static REENTER_ALLOC_VAS: RwLock<Vec<(u64, usize)>> = RwLock::new(Vec::new());
static HAS_REENTER_ALLOC_VAS: AtomicBool = AtomicBool::new(false);
static SHADOW_CANDIDATE_PROVIDER: OnceLock<fn() -> Vec<(&'static str, &'static Replacement)>> = OnceLock::new();

static HAS_EXE_CRT_ROUTES: AtomicBool = AtomicBool::new(false);
static EXE_CHKSTK_VA: AtomicU64 = AtomicU64::new(0);
static EXE_CHECK_COOKIE_VA: AtomicU64 = AtomicU64::new(0);
static EXE_SECURITY_COOKIE_VA: AtomicU64 = AtomicU64::new(0);
static EXE_REPORT_GSFAILURE_VA: AtomicU64 = AtomicU64::new(0);
static EXE_GUARD_DISPATCH_VA: AtomicU64 = AtomicU64::new(0);
static EXE_GUARD_DISPATCH_THUNK_VA: AtomicU64 = AtomicU64::new(0);

static GAME_REPL_LO: AtomicU64 = AtomicU64::new(0);
static GAME_REPL_HI: AtomicU64 = AtomicU64::new(0);
static EXE_IAT_THUNK_LO: AtomicU64 = AtomicU64::new(0);
static EXE_IAT_THUNK_HI: AtomicU64 = AtomicU64::new(0);

/// Special-ABI CRT/CFG helper VAs inside a primary executable image (e.g. MSVC
/// `__chkstk`, `__security_check_cookie`, `_guard_dispatch_icall`).
///
/// All fields default to `0` (disabled) so universal PE runners carry zero
/// game-specific virtual addresses.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ExeCrtRoutes {
    /// `__chkstk` VA (stack probe; takes size in `rax`).
    pub chkstk_va: u64,
    /// `__security_check_cookie` VA (preserves `rax`, checks `rcx` against `security_cookie_va`).
    pub check_cookie_va: u64,
    /// `__security_cookie` global qword VA read by `__security_check_cookie`.
    pub security_cookie_va: u64,
    /// `__report_gsfailure` VA (failure tail-jump target of `__security_check_cookie`).
    pub report_gsfailure_va: u64,
    /// `_guard_dispatch_icall` VA (`jmp rax`).
    pub guard_dispatch_va: u64,
    /// Secondary CFG dispatch-pointer thunk VA (e.g. `jmp [__guard_dispatch_icall_fptr]`).
    pub guard_dispatch_thunk_va: u64,
}

/// Register the primary executable's special-ABI CRT/CFG helper VAs (`0` = disabled).
pub fn register_exe_crt_routes(routes: ExeCrtRoutes) {
    EXE_CHKSTK_VA.store(routes.chkstk_va, Ordering::Release);
    EXE_CHECK_COOKIE_VA.store(routes.check_cookie_va, Ordering::Release);
    EXE_SECURITY_COOKIE_VA.store(routes.security_cookie_va, Ordering::Release);
    EXE_REPORT_GSFAILURE_VA.store(routes.report_gsfailure_va, Ordering::Release);
    EXE_GUARD_DISPATCH_VA.store(routes.guard_dispatch_va, Ordering::Release);
    EXE_GUARD_DISPATCH_THUNK_VA.store(routes.guard_dispatch_thunk_va, Ordering::Release);
    let active = routes.chkstk_va != 0
        || routes.check_cookie_va != 0
        || routes.security_cookie_va != 0
        || routes.report_gsfailure_va != 0
        || routes.guard_dispatch_va != 0
        || routes.guard_dispatch_thunk_va != 0;
    HAS_EXE_CRT_ROUTES.store(active, Ordering::Release);
    crate::exe::invalidate_dispatch();
}

/// True if any primary-executable CRT/CFG helper VA has been registered.
#[inline(always)]
pub fn has_exe_crt_routes() -> bool {
    HAS_EXE_CRT_ROUTES.load(Ordering::Relaxed)
}

/// Registered primary-executable `__chkstk` VA (`0` = none).
#[inline(always)]
pub fn exe_chkstk_va() -> u64 {
    EXE_CHKSTK_VA.load(Ordering::Relaxed)
}

/// True if `va` matches the registered primary-executable `__chkstk` VA.
#[inline(always)]
pub fn is_exe_chkstk_va(va: u64) -> bool {
    let v = EXE_CHKSTK_VA.load(Ordering::Relaxed);
    v != 0 && va == v
}

/// Registered primary-executable `__security_check_cookie` VA (`0` = none).
#[inline(always)]
pub fn exe_check_cookie_va() -> u64 {
    EXE_CHECK_COOKIE_VA.load(Ordering::Relaxed)
}

/// Registered primary-executable `__security_cookie` global VA (`0` = none).
#[inline(always)]
pub fn exe_security_cookie_va() -> u64 {
    EXE_SECURITY_COOKIE_VA.load(Ordering::Relaxed)
}

/// Registered primary-executable `__report_gsfailure` VA (`0` = none).
#[inline(always)]
pub fn exe_report_gsfailure_va() -> u64 {
    EXE_REPORT_GSFAILURE_VA.load(Ordering::Relaxed)
}

/// Registered primary-executable `_guard_dispatch_icall` VA (`0` = none).
#[inline(always)]
pub fn exe_guard_dispatch_va() -> u64 {
    EXE_GUARD_DISPATCH_VA.load(Ordering::Relaxed)
}

/// True if `va` is a registered primary-executable special-ABI CRT/CFG helper that must
/// never be dispatched as a standard Win64 host callable.
#[inline(always)]
pub fn is_exe_special_abi_va(va: u64) -> bool {
    if !HAS_EXE_CRT_ROUTES.load(Ordering::Relaxed) || va == 0 {
        return false;
    }
    va == EXE_CHECK_COOKIE_VA.load(Ordering::Relaxed)
        || va == EXE_CHKSTK_VA.load(Ordering::Relaxed)
        || va == EXE_GUARD_DISPATCH_VA.load(Ordering::Relaxed)
        || va == EXE_GUARD_DISPATCH_THUNK_VA.load(Ordering::Relaxed)
}

/// Register the primary executable's native-replacement VA window `[repl_lo, repl_hi)`
/// and its IAT-thunk exclusion sub-window `[thunk_lo, thunk_hi)`.
///
/// Both default to empty (`0..0`) for universal PE runners.
pub fn register_game_repl_window(repl_lo: u64, repl_hi: u64, thunk_lo: u64, thunk_hi: u64) {
    GAME_REPL_LO.store(repl_lo, Ordering::Release);
    GAME_REPL_HI.store(repl_hi, Ordering::Release);
    EXE_IAT_THUNK_LO.store(thunk_lo, Ordering::Release);
    EXE_IAT_THUNK_HI.store(thunk_hi, Ordering::Release);
    crate::exe::invalidate_dispatch();
}

/// True if `va` lies inside the registered primary-executable native-replacement window.
#[inline(always)]
pub fn in_game_repl_window(va: u64) -> bool {
    let lo = GAME_REPL_LO.load(Ordering::Relaxed);
    let hi = GAME_REPL_HI.load(Ordering::Relaxed);
    lo < hi && va >= lo && va < hi
}

/// True if `va` lies inside the registered primary-executable IAT-thunk exclusion window.
#[inline(always)]
pub fn in_exe_iat_thunk_window(va: u64) -> bool {
    let lo = EXE_IAT_THUNK_LO.load(Ordering::Relaxed);
    let hi = EXE_IAT_THUNK_HI.load(Ordering::Relaxed);
    lo < hi && va >= lo && va < hi
}

/// Register sorted VAs of replacement functions that return in `xmm0`.
pub fn register_float_return_vas(vas: &[u64]) {
    let mut v = FLOAT_RETURN_VAS.write().unwrap();
    v.extend_from_slice(vas);
    v.sort_unstable();
    v.dedup();
    if !v.is_empty() {
        HAS_FLOAT_RETURN_VAS.store(true, Ordering::Release);
    }
}

/// True if `va` was registered as returning in `xmm0`.
#[inline(always)]
pub fn is_float_return_va(va: u64) -> bool {
    if !HAS_FLOAT_RETURN_VAS.load(Ordering::Relaxed) {
        return false;
    }
    FLOAT_RETURN_VAS
        .read()
        .is_ok_and(|v| v.binary_search(&va).is_ok())
}

/// Register a lazy provider returning all compiled Rust replacement function pointers
/// eligible for direct `DF_LEAN` AAPCS64 calls on `aarch64`.
pub fn register_lean_targets_provider(provider: fn() -> Vec<usize>) {
    let _ = LEAN_HOST_PROVIDER.set(provider);
}

/// Collect registered `DF_LEAN` host pointers (empty for manifest-only games).
pub fn collect_lean_targets() -> Vec<usize> {
    LEAN_HOST_PROVIDER.get().map_or_else(Vec::new, |f| f())
}

/// Register a predicate marking additional host function pointers as `DF_PLAIN`
/// (e.g. hand-written native DLL leaf replacements).
pub fn register_plain_native_check(check: fn(usize) -> bool) {
    PLAIN_NATIVE_CHECKS.write().unwrap().push(check);
    HAS_PLAIN_NATIVE_CHECKS.store(true, Ordering::Release);
}

/// True if any registered plugin marks `host_ptr` as `DF_PLAIN`.
#[inline(always)]
pub fn is_extra_plain_native(host_ptr: usize) -> bool {
    if !HAS_PLAIN_NATIVE_CHECKS.load(Ordering::Relaxed) {
        return false;
    }
    PLAIN_NATIVE_CHECKS
        .read()
        .is_ok_and(|checks| checks.iter().any(|f| f(host_ptr)))
}

/// Register a JIT fast-path inline-variant resolver (`host_ptr -> (fast_ptr, fallible)`).
pub fn register_inline_variant_hook(hook: fn(usize) -> Option<(usize, bool)>) {
    INLINE_VARIANT_HOOKS.write().unwrap().push(hook);
    HAS_INLINE_VARIANT_HOOKS.store(true, Ordering::Release);
}

/// Query registered JIT inline-variant hooks.
#[inline(always)]
pub fn lookup_extra_inline_variant(host_ptr: usize) -> Option<(usize, bool)> {
    if !HAS_INLINE_VARIANT_HOOKS.load(Ordering::Relaxed) {
        return None;
    }
    INLINE_VARIANT_HOOKS
        .read()
        .ok()?
        .iter()
        .find_map(|f| f(host_ptr))
}

/// Register `(va, host_ptr)` pairs eligible for `AW2_JIT_THUNK_ELIDE` bit 2.
pub fn register_reenter_alloc_vas(pairs: &[(u64, usize)]) {
    let mut v = REENTER_ALLOC_VAS.write().unwrap();
    v.extend_from_slice(pairs);
    if !v.is_empty() {
        HAS_REENTER_ALLOC_VAS.store(true, Ordering::Release);
    }
}

/// True if `(va, host_ptr)` is in the registered re-entering allocator list.
#[inline(always)]
pub fn is_reenter_alloc_va(va: u64, host_ptr: usize) -> bool {
    if !HAS_REENTER_ALLOC_VAS.load(Ordering::Relaxed) {
        return false;
    }
    REENTER_ALLOC_VAS
        .read()
        .is_ok_and(|v| v.contains(&(va, host_ptr)))
}

/// Count of registered re-entering allocator VAs.
pub fn reenter_alloc_len() -> usize {
    if !HAS_REENTER_ALLOC_VAS.load(Ordering::Relaxed) {
        return 0;
    }
    REENTER_ALLOC_VAS.read().map_or(0, |v| v.len())
}

/// Register the candidate list provider for `shadow` live differential testing.
pub fn register_shadow_candidates_provider(provider: fn() -> Vec<(&'static str, &'static Replacement)>) {
    let _ = SHADOW_CANDIDATE_PROVIDER.set(provider);
}

/// Collect registered `(group, Replacement)` entries for `shadow`.
pub fn collect_shadow_candidates() -> Vec<(&'static str, &'static Replacement)> {
    SHADOW_CANDIDATE_PROVIDER.get().map_or_else(Vec::new, |f| f())
}
