//! Pluggable per-game extension hooks for `x86-runtime`.
//!
//! Keeps `x86-runtime` 100% free of game-specific VAs, hand-decompiled namespaces,
//! and static shards. A game crate (such as `aw2-game`) registers its tables once at
//! startup; generic PE runners leave every hook empty.

use crate::registry_types::Replacement;
use std::sync::atomic::{AtomicBool, Ordering};
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
