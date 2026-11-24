//! Function replacement hooks.
//!
//! For every entry of `aw2_game::registry::REPLACEMENTS` the first bytes of
//! the original function are overwritten with `jmp rel32` to a small thunk
//! within ±2 GB, which does an absolute `jmp [rip]` to the Rust function.
//!
//! Safety checks (iced-x86):
//! * the patch must cover whole instructions (the remainder of the last
//!   displaced instruction is filled with `int3`);
//! * none of the displaced instructions may be a `ret`/`jmp`/`int3` padding
//!   before 5 bytes are covered (function too short to patch);
//! * when the exe's `.pdata` has a RUNTIME_FUNCTION for the address it must
//!   start exactly there (catches wrong VAs).
//!
//! **Trampolines** (calling the original from Rust, e.g. for differential
//! tests): the displaced instructions are re-encoded at a new address with
//! iced's `BlockEncoder` (which fixes up RIP-relative operands and relative
//! branches), followed by an absolute jump back to `va + patch_len`. The
//! trampoline address is recorded in [`trampoline_for`]. Limitation: a branch
//! from later in the function back into the displaced bytes would land in
//! `int3` padding; we scan the function for such branches and refuse to hook.
//!
//! **Call counters**: registry hooks (and probes) go through a counting thunk
//! (`lock inc qword [rip+ctr]; jmp [rip]`). Counters are only read by the
//! reporter thread ([`start_reporter`]).
//!
//! **Register-preserving shim**: registry hooks then enter the Rust function
//! through `shim::build_shim`, which keeps rcx/rdx/r8–r11/xmm0–5 intact for
//! the caller (MSVC callers of known leaves rely on untouched volatiles).
//!
//! **Probes** (`AW2_PROBE=0x14...,0x14...` or `AW2_PROBE=@file`, one VA per
//! line, optional name after it): pass-through counted hooks whose target is
//! the original (via its trampoline). Used to find out which functions run
//! during boot before choosing what to decompile. Unhookable VAs are skipped.
//!
//! **Reporter** (enabled by `AW2_PROBE` or `AW2_HOOK_STATS=1`): a host thread
//! polls the counters every 250 ms, logs `first call: <name> @<va>` the first
//! time a hooked function runs and, with `AW2_HOOK_STATS_FILE=<path>`,
//! rewrites `va<TAB>name<TAB>calls` totals (the process ends with `_exit`, so
//! there is no exit-time dump).
//!
//! **Differential tests** ([`run_difftests`], `aw2-loader --diff-test`): for
//! every registry entry with a `diff` function, call it with the original
//! (trampoline) and the Rust replacement; it feeds both identical inputs and
//! compares results/side effects.
//!
//! **Hook-safety lint** (`throws_lint::hook_lint`): each entry's `throws`
//! annotation is checked against `db/throws.tsv` (can-throw facts from
//! `tools/fndb.sh sync`). Findings are warnings at hook time and failures under
//! `--diff-test`.
//!
//! **Fence frames** (`shim::build_shim` with `fence`): entries with
//! `throws != No` (or every entry with `AW2_FENCE_ALL=1`) enter Rust through a
//! shim that also saves rbx/rbp/rsi/rdi/r12–r15/xmm6–15 and has a registered
//! RUNTIME_FUNCTION. An exception raised below the Rust replacement unwinds the
//! Rust frames like a `longjmp` (win32-rt `unwind::cross_host_frames`), so they
//! must not hold live `Drop` values across calls that can throw.
//!
//! **`exe::try_call`** ([`install_try_thunk`]): a call thunk with `.pdata`
//! whose language handler claims matching MSVC C++ exceptions for Rust
//! (docs/design/ADVICE_SEH.md §3.3).

#[cfg_attr(not(target_arch = "x86_64"), path = "hooks/patch.rs")]
mod patch;
#[cfg_attr(not(target_arch = "x86_64"), path = "hooks/shim.rs")]
mod shim;
#[cfg_attr(not(target_arch = "x86_64"), path = "hooks/throws_lint.rs")]
mod throws_lint;
#[cfg(test)]
#[cfg_attr(not(target_arch = "x86_64"), path = "hooks/tests.rs")]
mod tests;

#[allow(unused_imports)]
pub use patch::{install, install_ex, trampoline_for, Dest, Patch, ORIG_TRAMPOLINE_TAG};
pub use shim::install_try_thunk;

use anyhow::{bail, Result};
use patch::{
    hook_va, loader_lookup, record_trampoline, temp_patch, temp_unpatch, va_to_addr_with,
    FnLookup, COUNTERS, EXE_BASE, PATCH_PROT, PREFERRED_EXE_BASE,
};
use shim::{build_shim, place_shim};
use std::sync::atomic::Ordering;
use throws_lint::hook_lint;

/// Apply every replacement from a [`x86_runtime::game_plugin::ReplacementSet`] to the mapped exe.
pub fn apply_registry(
    exe_base: usize,
    set: &x86_runtime::game_plugin::ReplacementSet,
) -> Result<usize> {
    apply_registry_with(exe_base, set, &loader_lookup, libc::PROT_READ | libc::PROT_EXEC)
}

fn apply_registry_with(
    exe_base: usize,
    set: &x86_runtime::game_plugin::ReplacementSet,
    lookup: FnLookup,
    restore_prot: i32,
) -> Result<usize> {
    let default_hook_shards = cfg!(not(target_arch = "x86_64"));
    let hook_shards = std::env::var_os("AW2_HOOK_SHARDS")
        .map_or(default_hook_shards, |v| v != "0")
        || std::env::var_os("AW2_DIFF_SHARDS").is_some_and(|v| v != "0");
    let min_va = std::env::var("AW2_HOOK_MIN_VA")
        .ok()
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(0);
    let max_va = std::env::var("AW2_HOOK_MAX_VA")
        .ok()
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(u64::MAX);
    let has_va_filter = std::env::var_os("AW2_HOOK_MIN_VA").is_some()
        || std::env::var_os("AW2_HOOK_MAX_VA").is_some();
    let is_special_abi = |va: u64| set.special_abi_vas.contains(&va);
    let mut reps: Vec<&'static x86_runtime::registry_types::Replacement> = if hook_shards {
        set.all
            .iter()
            .copied()
            .filter(|r| !is_special_abi(r.va))
            .filter(|r| (!has_va_filter && r.diff.is_some()) || (r.va >= min_va && r.va <= max_va))
            .collect()
    } else {
        set.base
            .iter()
            .copied()
            .filter(|r| !is_special_abi(r.va) && r.va >= min_va && r.va <= max_va)
            .collect()
    };
    reps.sort_by_key(|r| r.va);
    PREFERRED_EXE_BASE.store(set.preferred_exe_base, Ordering::Relaxed);
    x86_runtime::exe::set_aw2_registry_active(true);
    x86_runtime::exe::set_base(exe_base);
    EXE_BASE.store(exe_base, Ordering::Relaxed);
    PATCH_PROT.store(restore_prot as usize, Ordering::Relaxed);
    x86_runtime::difftest::set_patcher(temp_patch, temp_unpatch);
    for w in reps.windows(2) {
        if w[0].va == w[1].va {
            bail!("duplicate replacement VA 0x{:x} ({} and {})", w[0].va, w[0].name, w[1].name);
        }
    }
    if std::env::var_os("AW2_HOOK_LINT").is_some_and(|v| v != "0") {
        hook_lint(set);
    }
    install_try_thunk();
    x86_runtime::exe::set_trampoline_lookup(trampoline_for);
    // On non-x86_64 hosts (`aarch64-apple-darwin`), register the active replacements
    // in the native VA dispatch table so `exe::call_va!` and `emu_run` resolve
    // verified replacement VAs directly to their native `aarch64` Rust functions
    // while executing unverified shard functions accurately via `emu.rs`.
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (lookup, restore_prot, build_shim, place_shim, hook_va);
        let pref = set.preferred_exe_base;
        let iat_window_end = pref.saturating_add(0x0300_0000);
        let is_unverified_iat_thunk = |r: &x86_runtime::registry_types::Replacement| -> bool {
            r.diff.is_none()
                && (pref..iat_window_end).contains(&r.va)
                && unsafe {
                    x86_runtime::exe::rd::<u16>(va_to_addr_with(exe_base, pref, r.va)) == 0x25ff
                }
        };
        let native_count = reps
            .iter()
            .filter(|r| !x86_runtime::simd_ops::is_aot_va_stub(r.ptr) && !is_unverified_iat_thunk(r))
            .count();
        x86_runtime::exe::register_native_vas(
            reps.iter()
                .filter(|r| {
                    !x86_runtime::simd_ops::is_aot_va_stub(r.ptr) && !is_unverified_iat_thunk(r)
                })
                .map(|r| (r.va, r.ptr as usize)),
        );
        for r in &reps {
            let addr = va_to_addr_with(exe_base, pref, r.va);
            record_trampoline(r.va, addr | ORIG_TRAMPOLINE_TAG);
        }
        let guest_jit = reps.iter().filter(|r| x86_runtime::simd_ops::is_aot_va_stub(r.ptr)).count();
        let (mut thunks_host, mut thunks_guest) = (0usize, 0usize);
        for r in reps
            .iter()
            .filter(|r| !x86_runtime::simd_ops::is_aot_va_stub(r.ptr) && is_unverified_iat_thunk(r))
        {
            if x86_runtime::emu::resolve_host_callable(r.va).is_some() {
                thunks_host += 1;
            } else {
                thunks_guest += 1;
            }
        }
        log::info!(
            "registered {} compiled native replacement functions in VA dispatch table ({} total in shards, {} guest-JIT, {} import thunks routed to their targets: {} host, {} guest DLL)",
            native_count,
            set.total_count,
            guest_jit,
            thunks_host + thunks_guest,
            thunks_host,
            thunks_guest,
        );
        return Ok(reps.len());
    }
    #[cfg(target_arch = "x86_64")]
    {
        let fence_all = std::env::var_os("AW2_FENCE_ALL").is_some_and(|v| v != "0");
        let count_hooks = std::env::var_os("AW2_HOOK_STATS").is_some_and(|v| v != "0")
            || std::env::var_os("AW2_PROBE").is_some();
        let rwx = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;
        let batched_span = if !reps.is_empty() && restore_prot != rwx {
            let first_page =
                va_to_addr_with(exe_base, set.preferred_exe_base, reps.first().unwrap().va)
                    & !0xFFF;
            let last_end =
                (va_to_addr_with(exe_base, set.preferred_exe_base, reps.last().unwrap().va)
                    + 64
                    + 0xFFF)
                    & !0xFFF;
            let span = last_end.saturating_sub(first_page);
            if span > 0 && unsafe { libc::mprotect(first_page as *mut libc::c_void, span, rwx) } == 0
            {
                Some((first_page, span))
            } else {
                None
            }
        } else {
            None
        };
        let loop_prot = if batched_span.is_some() { rwx } else { restore_prot };
        let mut fenced_count = 0usize;
        let mut hooked_count = 0usize;
        let mut hook_err = None;
        for r in &reps {
            let float_ret = set.float_return_vas.contains(&r.va);
            let fence = throws_lint::FENCES
                && (fence_all || r.throws != x86_runtime::registry_types::Throws::No);
            let shim = place_shim(&build_shim(r.ptr as usize, float_ret, fence));
            let p = match hook_va(
                exe_base,
                r.va,
                r.name,
                Dest::Fixed(shim),
                count_hooks,
                lookup,
                loop_prot,
            ) {
                Ok(p) => p,
                Err(e) if hook_shards && r.diff.is_none() => {
                    log::debug!("skipped unhookable shard entry {} @0x{:x}: {e}", r.name, r.va);
                    continue;
                }
                Err(e) => {
                    hook_err = Some(e);
                    break;
                }
            };
            if fence {
                fenced_count += 1;
            }
            hooked_count += 1;
            log::debug!(
                "hooked {} @0x{:x} ({:?}{}) -> {:p}, trampoline 0x{:x}",
                r.name,
                r.va,
                r.status,
                if fence { ", fenced" } else { "" },
                r.ptr,
                p.trampoline
            );
            record_trampoline(r.va, p.trampoline);
            if let Some(c) = p.counter {
                COUNTERS.lock().unwrap().push((r.va, r.name.to_string(), "replacement", c));
            }
        }
        if let Some((first_page, span)) = batched_span {
            unsafe {
                libc::mprotect(first_page as *mut libc::c_void, span, restore_prot);
            }
        }
        if let Some(e) = hook_err {
            return Err(e);
        }
        log::info!(
            "hooked {} functions from game replacement registry ({} fenced, {} total registered in shards)",
            hooked_count,
            fenced_count,
            set.total_count
        );
        Ok(hooked_count)
    }
}

/// Set up generic exe hook state (when no plugin [`ReplacementSet`] is active):
/// preferred base + patcher + try thunk + trampoline lookup.
pub fn init_generic_hooks(exe_base: usize) {
    let pref = x86_runtime::exe::preferred_base();
    if pref != 0 {
        PREFERRED_EXE_BASE.store(pref, Ordering::Relaxed);
    }
    EXE_BASE.store(exe_base, Ordering::Relaxed);
    x86_runtime::difftest::set_patcher(temp_patch, temp_unpatch);
    install_try_thunk();
    x86_runtime::exe::set_trampoline_lookup(trampoline_for);
}

/// Parse `AW2_PROBE`: `0xVA[,0xVA...]` or `@file` (one `0xVA [name]` per line,
/// `#` comments).
fn probe_list(spec: &str) -> Result<Vec<(u64, String)>> {
    let text;
    let body = if let Some(path) = spec.strip_prefix('@') {
        text = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("AW2_PROBE file {path}: {e}"))?;
        text.as_str()
    } else {
        spec
    };
    let mut out = Vec::new();
    for line in body.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if spec.starts_with('@') {
            let mut it = line.splitn(2, char::is_whitespace);
            let va = it.next().unwrap();
            let name = it.next().unwrap_or("").trim();
            let v = u64::from_str_radix(va.trim_start_matches("0x"), 16)
                .map_err(|_| anyhow::anyhow!("bad probe VA {va:?}"))?;
            out.push((v, if name.is_empty() { format!("sub_{v:x}") } else { name.to_string() }));
        } else {
            for tok in line.split([',', ' ']).filter(|t| !t.is_empty()) {
                let v = u64::from_str_radix(tok.trim_start_matches("0x"), 16)
                    .map_err(|_| anyhow::anyhow!("bad probe VA {tok:?}"))?;
                out.push((v, format!("sub_{v:x}")));
            }
        }
    }
    Ok(out)
}

/// Install pass-through counted probes from `AW2_PROBE` (see module docs).
/// Returns (installed, skipped).
pub fn apply_probes(exe_base: usize) -> Result<(usize, usize)> {
    let Ok(spec) = std::env::var("AW2_PROBE") else { return Ok((0, 0)) };
    let list = probe_list(&spec)?;
    let (mut ok, mut skipped) = (0, 0);
    let mut seen: std::collections::HashSet<u64> =
        COUNTERS.lock().unwrap().iter().map(|c| c.0).collect();
    for (va, name) in list {
        if !seen.insert(va) {
            continue; // already hooked (registry replacement or duplicate)
        }
        match hook_va(
            exe_base,
            va,
            &name,
            Dest::Trampoline,
            true,
            &loader_lookup,
            libc::PROT_READ | libc::PROT_EXEC,
        ) {
            Ok(p) => {
                ok += 1;
                if let Some(c) = p.counter {
                    COUNTERS.lock().unwrap().push((va, name, "probe", c));
                }
            }
            Err(e) => {
                skipped += 1;
                log::debug!("probe skipped: {e}");
            }
        }
    }
    log::info!("AW2_PROBE: {ok} probes installed, {skipped} skipped (unhookable)");
    Ok((ok, skipped))
}

/// Start the counter reporter thread if `AW2_PROBE` or `AW2_HOOK_STATS` is set.
pub fn start_reporter() {
    let enabled = std::env::var_os("AW2_PROBE").is_some()
        || std::env::var_os("AW2_HOOK_STATS").is_some_and(|v| v != "0");
    if !enabled || COUNTERS.lock().unwrap().is_empty() {
        return;
    }
    let file = std::env::var_os("AW2_HOOK_STATS_FILE").map(std::path::PathBuf::from);
    let spawn = std::thread::Builder::new().name("aw2-hook-stats".into()).spawn(move || {
        let mut seen: Vec<u64> = Vec::new();
        let mut last_total = u64::MAX;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(250));
            let counters = COUNTERS.lock().unwrap();
            let snap: Vec<u64> = counters.iter().map(|c| c.3.load(Ordering::Relaxed)).collect();
            if seen.len() < snap.len() {
                seen.resize(snap.len(), 0);
            }
            for (i, n) in snap.iter().enumerate() {
                if seen[i] == 0 && *n != 0 {
                    let (va, name, kind, _) = &counters[i];
                    log::info!(
                        target: "hooks",
                        "first call: {name} @0x{va:x} ({kind}, {n} calls so far)"
                    );
                }
                seen[i] = *n;
            }
            let total: u64 = snap.iter().sum();
            if let Some(path) = &file {
                if total != last_total {
                    let mut rows: Vec<_> = (0..snap.len()).filter(|&i| snap[i] != 0).collect();
                    rows.sort_by(|&a, &b| snap[b].cmp(&snap[a]));
                    let mut s = String::from("va\tname\tkind\tcalls\n");
                    for i in rows {
                        let (va, name, kind, n) =
                            (counters[i].0, &counters[i].1, counters[i].2, snap[i]);
                        s.push_str(&format!("0x{va:x}\t{name}\t{kind}\t{n}\n"));
                    }
                    let tmp = path.with_extension("tmp");
                    if std::fs::write(&tmp, s).is_ok() {
                        let _ = std::fs::rename(&tmp, path);
                    }
                }
            }
            drop(counters);
            last_total = total;
        }
    });
    if let Err(e) = spawn {
        log::warn!("hook stats reporter: {e}");
    }
}

/// Run every registry differential test (original via trampoline vs Rust).
/// Returns (passed, failed). Entries without a `diff` function are skipped.
pub fn run_difftests(set: Option<&x86_runtime::game_plugin::ReplacementSet>) -> (usize, usize) {
    x86_runtime::emu::with_frozen_tsc(|| run_difftests_with_frozen_clock(set))
}

fn run_difftests_with_frozen_clock(set: Option<&x86_runtime::game_plugin::ReplacementSet>) -> (usize, usize) {
    // AOT translation caches for the mapped images (AW2_DLL_AOT / AW2_DLL_AOT_BUILD).
    x86_runtime::emu::aot_activate();
    let _ = win32_rt::teb::map_kuser_shared_data();
    let teb = win32_rt::teb::current();
    unsafe {
        let mut tls_arr = x86_runtime::exe::tls_pointer() as usize;
        if tls_arr == 0 {
            tls_arr = Box::leak(vec![0usize; 64].into_boxed_slice()).as_mut_ptr() as usize;
            win32_rt::teb::set_field(teb, win32_rt::teb::off::TLS_POINTER, tls_arr);
            win32_rt::teb::gs_write(win32_rt::teb::off::TLS_POINTER, tls_arr);
        }
        if x86_runtime::exe::rd::<usize>(tls_arr) == 0 {
            let slot0 = Box::leak(vec![0u8; 0x80000].into_boxed_slice()).as_mut_ptr() as usize;
            x86_runtime::exe::wr::<usize>(tls_arr, slot0);
        }
    }
    let (mut pass, mut fail) = (0, 0);
    let diff_shards = std::env::var_os("AW2_DIFF_SHARDS").is_some_and(|v| v != "0");
    let diff_limit: usize = std::env::var("AW2_DIFF_LIMIT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(usize::MAX);
    let only: Vec<String> = std::env::var("AW2_DIFF_ONLY")
        .ok()
        .map(|s| {
            s.split(',').map(|t| t.trim().to_ascii_lowercase()).filter(|t| !t.is_empty()).collect()
        })
        .unwrap_or_default();
    if let Some(set) = set {
        let selected = |r: &x86_runtime::registry_types::Replacement| {
            only.is_empty()
                || only.iter().any(|t| {
                    let tv = t.trim_start_matches("0x").replace('_', "");
                    u64::from_str_radix(&tv, 16).is_ok_and(|v| v == r.va)
                        || r.name.to_ascii_lowercase().contains(t.as_str())
                })
        };
        for &r in &set.all {
            if pass + fail >= diff_limit {
                break;
            }
            if !selected(r) {
                continue;
            }
            if r.diff.is_none() && !diff_shards {
                continue;
            }
            let Some(orig) = trampoline_for(r.va) else {
                log::error!("difftest FAIL {} @0x{:x}: not hooked (no trampoline)", r.name, r.va);
                fail += 1;
                continue;
            };
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                if let Some(diff) = r.diff {
                    diff(orig, r.ptr as usize)
                } else {
                    x86_runtime::difftest::shard_leaf_difftest(r.va, orig, r.ptr as usize)
                }
            }));
            match res {
                Ok(Ok(cases)) => {
                    if r.diff.is_some() || pass < 25 || pass % 1000 == 0 {
                        log::info!(
                            "difftest ok   {} @0x{:x} ({:?}): {cases} cases (pass={})",
                            r.name,
                            r.va,
                            r.status,
                            pass + 1
                        );
                    }
                    pass += 1;
                }
                Ok(Err(e)) if e.starts_with(x86_runtime::difftest::SKIP) => {
                    if r.diff.is_some() {
                        log::warn!("difftest skip {} @0x{:x}: {e}", r.name, r.va);
                    }
                }
                Ok(Err(e)) => {
                    log::error!("difftest FAIL {} @0x{:x}: {e}", r.name, r.va);
                    fail += 1;
                }
                Err(payload) => {
                    let msg = payload
                        .downcast_ref::<&str>()
                        .copied()
                        .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
                        .unwrap_or("panic");
                    log::error!("difftest PANIC {} @0x{:x}: {msg}", r.name, r.va);
                    fail += 1;
                }
            }
        }
    }
    // Native replacements inside mapped DLLs (`x86_runtime::dll_native`): the original is the
    // guest code at the mapped address, entered through the emulator tag.
    #[cfg(not(target_arch = "x86_64"))]
    for d in x86_runtime::dll_native::difftests() {
        if pass + fail >= diff_limit {
            break;
        }
        let name = d.name.to_ascii_lowercase();
        if !only.is_empty() && !only.iter().any(|t| name.contains(t.as_str())) {
            continue;
        }
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            (d.diff)(d.addr | ORIG_TRAMPOLINE_TAG, d.new)
        }));
        match res {
            Ok(Ok(cases)) => {
                log::info!(
                    "difftest ok   {} @0x{:x} (dll_native): {cases} cases (pass={})",
                    d.name,
                    d.addr,
                    pass + 1
                );
                pass += 1;
            }
            Ok(Err(e)) if e.starts_with(x86_runtime::difftest::SKIP) => {
                log::warn!("difftest skip {} @0x{:x}: {e}", d.name, d.addr)
            }
            Ok(Err(e)) => {
                log::error!("difftest FAIL {} @0x{:x}: {e}", d.name, d.addr);
                fail += 1;
            }
            Err(_) => {
                log::error!("difftest PANIC {} @0x{:x}", d.name, d.addr);
                fail += 1;
            }
        }
    }
    // Exe guest-entry routing (`aw2_game::exe_route_diff`): import thunks and special-ABI CRT
    // helpers routed natively vs the exe's original x86 under the emulator.
    #[cfg(all(feature = "aw2-core", not(target_arch = "x86_64")))]
    if set.is_some() {
        for d in aw2_game::exe_route_diff::difftests() {
            if pass + fail >= diff_limit {
                break;
            }
            if !only.is_empty() && !only.iter().any(|t| d.name.contains(t.as_str())) {
                continue;
            }
            let res =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { (d.run)() }));
            match res {
                Ok(Ok(cases)) => {
                    log::info!(
                        "difftest ok   {} (exe route): {cases} cases (pass={})",
                        d.name,
                        pass + 1
                    );
                    pass += 1;
                }
                Ok(Err(e)) if e.starts_with(x86_runtime::difftest::SKIP) => {
                    log::warn!("difftest skip {}: {e}", d.name)
                }
                Ok(Err(e)) => {
                    log::error!("difftest FAIL {}: {e}", d.name);
                    fail += 1;
                }
                Err(_) => {
                    log::error!("difftest PANIC {}", d.name);
                    fail += 1;
                }
            }
        }
    }
    if let Some(set) = set {
        let lint_errors = hook_lint(set).errors;
        if lint_errors > 0 {
            log::error!(
                "difftest: hook-lint reported {lint_errors} error(s) (logged at hook time), counted as failures"
            );
            fail += lint_errors;
        }
    }
    // `--diff-test` exits without the exit hooks: dump the AW2_JIT_PROFILE profile here.
    x86_runtime::emu::aot_profile_dump();
    (pass, fail)
}
