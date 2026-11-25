//! Guest execution entry points (`emu_run*`), main interpreter/JIT dispatch loop
//! (`emu_run_region`), pre-decoded fast-op block-chaining burst loop (`interp_burst`),
//! profiler guard, and fast-vs-generic differential verification helpers.

#[cfg(target_arch = "aarch64")]
use super::aot;
use super::block_cache::{
    fast_enabled, verify_enabled, BlkErr, BlockCache, CurCpuGuard, SpGuard, ThreadEmuState,
    BLK_MAX_INSNS, CUR_CPU, GUEST_FLOOR_MARGIN, GUEST_STACK_MIN_OFF, TLS_EMU,
};
use super::cpu::{
    guest_fault_dispatch, int3_returns, precise_faults, Cpu, GuestFpcrGuard, FAULT_PENDING,
    RET_SENTINEL,
};
use super::dispatch::{apply_exe_route, apply_fx, apply_xfer_tail, take_incoming_rsp};
#[cfg(test)]
use super::routes::{DLL_CHKSTK_OFF, DLL_STACK_POS_OFF};
use super::routes::{
    dll_chkstk_enabled, dll_chkstk_tail, dll_route_enabled, dll_route_gate, dll_stack_pos_enabled,
    dll_stack_pos_tail, dll_stub_route, exe_route, exe_route_gate, exe_route_off, in_exe_gate,
    is_routed_dll_chkstk, is_routed_dll_stack_pos, is_routed_dll_stub, lookup_routed_regfile,
    ExeRoute,
};
use super::{eh, fast, jit, jit_stats, note_null_call, regfile, regfile_ext};
use crate::exe::{rd, teb_self, wr};
use iced_x86::{ConditionCode, FlowControl, Instruction, Mnemonic, OpKind};

#[cold]
#[inline(never)]
fn report_fast_mismatch(ip: u64, ins: &Instruction, what: &str, fast_v: u64, gen_v: u64) {
    static SEEN: std::sync::Mutex<Option<std::collections::HashSet<u64>>> = std::sync::Mutex::new(None);
    let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
    if g.get_or_insert_with(Default::default).insert(ip) {
        log::error!("emu fast-path MISMATCH at {ip:#x} `{ins}`: {what} fast={fast_v:#x} generic={gen_v:#x}");
    }
}

/// Documented process exit code when an unimplemented x86-64 instruction is hit
/// with `--fail-on-unimpl-op` / `PORT_FAIL_ON_UNIMPL_OP=1` enabled.
pub const EXIT_UNIMPL_OP: u32 = 86;

/// Runtime override for fail-on-unimpl-op:
/// `0` = defer to `PORT_FAIL_ON_UNIMPL_OP` env var, `1` = force off, `2` = force on.
static FAIL_ON_UNIMPL_OP_OVERRIDE: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);

/// Explicitly enable or disable failing on unimplemented x86-64 instructions.
pub fn set_fail_on_unimpl_op(enabled: bool) {
    FAIL_ON_UNIMPL_OP_OVERRIDE.store(
        if enabled { 2 } else { 1 },
        std::sync::atomic::Ordering::Release,
    );
}

/// Clear any programmatic override so `PORT_FAIL_ON_UNIMPL_OP` controls behavior.
pub fn clear_fail_on_unimpl_op_override() {
    FAIL_ON_UNIMPL_OP_OVERRIDE.store(0, std::sync::atomic::Ordering::Release);
}

/// Parse a boolean env string (`0`/`false`/`no`/`off` -> false, any other non-empty -> true).
pub fn parse_env_bool(val: &str) -> bool {
    let t = val.trim();
    !t.is_empty()
        && !t.eq_ignore_ascii_case("0")
        && !t.eq_ignore_ascii_case("false")
        && !t.eq_ignore_ascii_case("no")
        && !t.eq_ignore_ascii_case("off")
}

/// Returns true if hitting an unimplemented x86-64 instruction in the generic
/// interpreter must abort the process with [`EXIT_UNIMPL_OP`].
pub fn fail_on_unimpl_op_enabled() -> bool {
    match FAIL_ON_UNIMPL_OP_OVERRIDE.load(std::sync::atomic::Ordering::Acquire) {
        1 => false,
        2 => true,
        _ => std::env::var("PORT_FAIL_ON_UNIMPL_OP")
            .ok()
            .is_some_and(|v| parse_env_bool(&v)),
    }
}

/// Handle an instruction the generic interpreter has no handler for.
///
/// When `--fail-on-unimpl-op` / `PORT_FAIL_ON_UNIMPL_OP=1` is active (default for
/// CLI bring-up runs and `live-probe`), logs the instruction, writes a crash report,
/// and terminates with [`EXIT_UNIMPL_OP`]. Otherwise logs once per guest IP and
/// continues as a no-op.
#[cold]
#[inline(never)]
fn report_unimplemented(ip: u64, mn: Mnemonic, ins: &Instruction, cpu: &Cpu) {
    if fail_on_unimpl_op_enabled() {
        let sym = win32_rt::symbolize(ip as usize);
        log::error!(
            "emu: UNIMPLEMENTED mnemonic {mn:?} at {ip:#x} ({sym}) `{ins}` — aborting (exit {EXIT_UNIMPL_OP}; disable with --no-fail-on-unimpl-op or PORT_FAIL_ON_UNIMPL_OP=0)"
        );
        eprintln!(
            "emu: UNIMPLEMENTED mnemonic {mn:?} at {ip:#x} ({sym}) `{ins}` — aborting (exit {EXIT_UNIMPL_OP})"
        );
        unsafe {
            let ctx = win32_rt::unwind::guest_context(&cpu.gpr, &cpu.xmm, ip, cpu.rflags());
            win32_rt::crash::report(
                &format!("unimplemented x86 instruction {mn:?} `{ins}` at {ip:#x}"),
                None,
                &ctx,
            );
        }
        win32_rt::process::exit_process(EXIT_UNIMPL_OP);
    }
    static SEEN: std::sync::Mutex<Option<std::collections::HashSet<u64>>> = std::sync::Mutex::new(None);
    let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
    if g.get_or_insert_with(Default::default).insert(ip) {
        log::error!("emu: UNIMPLEMENTED mnemonic {mn:?} at {ip:#x} `{ins}`");
    }
}

/// Compare a shadow CPU (fast path) against the real CPU (generic path).
#[cold]
#[inline(never)]
fn verify_fast(s: &Cpu, cpu: &Cpu, ip: u64, ins: &Instruction) {
    for i in 0..16 {
        if s.gpr[i] != cpu.gpr[i] {
            report_fast_mismatch(ip, ins, &format!("gpr[{i}]"), s.gpr[i], cpu.gpr[i]);
            return;
        }
        if s.last_bits[i] != cpu.last_bits[i] {
            report_fast_mismatch(ip, ins, &format!("last_bits[{i}]"), s.last_bits[i] as u64, cpu.last_bits[i] as u64);
            return;
        }
    }
    let fl = |c: &Cpu| (c.cf as u64) | (c.zf as u64) << 1 | (c.sf as u64) << 2 | (c.of as u64) << 3 | (c.pf() as u64) << 4;
    if fl(s) != fl(cpu) {
        report_fast_mismatch(ip, ins, "flags(cf,zf,sf,of,pf)", fl(s), fl(cpu));
        return;
    }
    if s.rip != cpu.rip {
        report_fast_mismatch(ip, ins, "rip", s.rip, cpu.rip);
        return;
    }
    for i in 0..16 {
        for (what, a, b) in [("xmm", s.xmm[i], cpu.xmm[i]), ("ymm_hi", s.ymm_hi[i], cpu.ymm_hi[i])] {
            if a != b {
                let half = if a as u64 != b as u64 { "lo" } else { "hi" };
                let sh = if half == "lo" { 0 } else { 64 };
                report_fast_mismatch(ip, ins, &format!("{what}[{i}].{half}"), (a >> sh) as u64, (b >> sh) as u64);
                return;
            }
        }
    }
}

/// Owns one interpreter invocation's counters and records them in [`crate::emu_prof`] on every exit path.
struct ProfGuard {
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    entry_ip: u64,
    steps: usize,
    misses: u64,
    flushed_steps: usize,
    flushed_misses: u64,
    sample_left: u32,
}

#[cfg(test)]
std::thread_local! {
    /// Guest instructions executed by the most recently finished `emu_run_region` on this thread.
    static TEST_LAST_STEPS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Cumulative guest instructions executed across `emu_run_region` runs on this thread.
    static TEST_TOTAL_STEPS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn test_last_steps() -> u64 {
    TEST_LAST_STEPS.with(|c| c.get())
}

#[cfg(test)]
pub(crate) fn test_total_steps() -> u64 {
    TEST_TOTAL_STEPS.with(|c| c.get())
}

impl Drop for ProfGuard {
    #[inline(always)]
    fn drop(&mut self) {
        #[cfg(test)]
        {
            let s = self.steps as u64;
            TEST_LAST_STEPS.with(|c| c.set(s));
            TEST_TOTAL_STEPS.with(|c| c.set(c.get().wrapping_add(s)));
        }
        if let Some(p) = self.prof {
            if self.steps > 0 {
                p.func_exit(self.entry_ip, (self.steps - self.flushed_steps) as u64, self.misses - self.flushed_misses);
            }
            p.sample_ctr.set(self.sample_left);
        }
    }
}

/// Execute guest `x86-64` instructions starting at `entry_ip` with `args`.
pub unsafe fn emu_run(entry_ip: u64, args: [u64; 8]) -> u64 {
    let xmm = [
        args[0] as u128,
        args[1] as u128,
        args[2] as u128,
        args[3] as u128,
    ];
    let (gpr0, _xmm0) = emu_run_full(entry_ip, args, xmm, [0; 8]);
    gpr0
}

/// Execute guest `x86-64` instructions starting at `entry_ip` with explicit GPR and XMM arguments,
/// returning `(rax, xmm0)`.
pub unsafe fn emu_run_with_xmm(entry_ip: u64, args: [u64; 8], xmm_args: [u128; 4]) -> (u64, u128) {
    emu_run_full(entry_ip, args, xmm_args, [0; 8])
}

/// Execute guest `x86-64` instructions starting at `entry_ip` with explicit GPR, XMM, and
/// stack arguments 9..16 (`stack_args[0..8]`), returning `(rax, xmm0)`. The guest starts with
/// rax = 0 (see [`emu_run_full_rax`]).
///
/// For Rust callers holding live state: the region's EH record is opaque (an exception
/// dispatch leaving its guest frames stops there, as at a Windows frame without unwind info).
#[inline]
pub unsafe fn emu_run_full(
    entry_ip: u64,
    args: [u64; 8],
    xmm_args: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    emu_run_region(entry_ip, args, xmm_args, stack_args, 0, true)
}

/// [`emu_run_full`] whose guest starts with `rax = rax_in`: a native (lifted) caller's rax at
/// its `call`, as a real x86 call leaves it. Lifted bodies return `get_rax_in()` on paths where
/// the original leaves rax untouched, and so does guest code: e.g. a lifted msvcp140
/// `std::exception` dtor calls `__std_exception_destroy` through its `jmp [IAT]` thunk (guest,
/// bound to guest vcruntime140) and returns the callee's untouched rax (the dtor's vftable
/// `lea`); with rax = 0 at the emulator entry the lifted dtor returned 0 where the original
/// returns the vftable (g15a difftest msvcp140 +0x1b6c / +0x1bec / +0x1c10).
/// A null `entry_ip` (`note_null_call`) still yields rax = 0, as the guest's `call 0` does.
///
/// For lifted callers (`call_va8*` bridges): the region's EH record is transparent, so an
/// exception dispatch continues from its outermost guest frame into the lifted caller.
#[inline]
pub unsafe fn emu_run_full_rax(
    entry_ip: u64,
    args: [u64; 8],
    xmm_args: [u128; 4],
    stack_args: [u64; 8],
    rax_in: u64,
) -> (u64, u128) {
    emu_run_region(entry_ip, args, xmm_args, stack_args, rax_in, false)
}

/// Maximum fast ops executed in one [`interp_burst`] invocation before returning to
/// `emu_run_region` to re-check outer budgets, profiler sampling, and epoch counters.
const INTERP_BURST_BUDGET: u32 = 8192;

#[cfg(test)]
std::thread_local! {
    /// Test-only: total [`interp_burst`] entries on this thread.
    pub(super) static TEST_INTERP_BURSTS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Test-only: total inter-block and self-loop block transitions performed inside [`interp_burst`].
    pub(super) static TEST_INTERP_BURST_CHAINS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// Outcome of an [`interp_burst`] run.
struct BurstExit {
    fx: fast::Fx,
    steps: u32,
    last_ip: u64,
    next_off: usize,
    next_left: usize,
}

/// Whether `fast::Cpu::rset` must maintain `Cpu::last_bits` (needed only when `DF_GAME_REPL`
/// hooks or differential verification / tests are active).
#[inline(always)]
fn fast_last_bits_on(verify: bool) -> bool {
    if cfg!(test) || verify {
        return true;
    }
    #[cfg(target_arch = "aarch64")]
    {
        jit::last_bits_enabled()
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        crate::exe::aw2_registry_active()
    }
}

/// Compute the loop-invariant portion of the mode-flags word for [`BlockCache::sync_fast_gen`].
#[inline(always)]
fn burst_base_mode_flags(jit_on: bool, dll_gate: (u64, u64)) -> u32 {
    let gate_hash = (((dll_gate.0 >> 12) ^ (dll_gate.1 >> 12)) as u32) << 16;
    #[cfg(not(test))]
    let fast_bit = u32::from(fast_enabled()) << 11;
    #[cfg(test)]
    let fast_bit = 0u32;
    (u32::from(jit_on) << 4)
        | (u32::from(crate::exe::aw2_registry_active()) << 5)
        | (u32::from(exe_route_off()) << 6)
        | (u32::from(dll_route_enabled()) << 7)
        | (u32::from(dll_chkstk_enabled()) << 8)
        | (u32::from(dll_stack_pos_enabled()) << 9)
        | fast_bit
        | gate_hash
}

/// Combine [`burst_base_mode_flags`] with per-burst dynamic bits (`jit_tag`, `aot`, test overrides).
#[inline(always)]
fn burst_mode_flags_from_base(base: u32, jit_ptr: *const jit::JitCache) -> u32 {
    #[cfg(target_arch = "aarch64")]
    let jit_tag = unsafe { (*jit_ptr).mode_tag() as u32 };
    #[cfg(not(target_arch = "aarch64"))]
    let jit_tag = {
        let _ = jit_ptr;
        0u32
    };
    #[cfg(target_arch = "aarch64")]
    let aot_on = u32::from(aot::active());
    #[cfg(not(target_arch = "aarch64"))]
    let aot_on = 0u32;
    #[cfg(test)]
    let test_bits = (u32::from(fast_enabled()) << 11)
        | (u32::from(DLL_CHKSTK_OFF.with(|c| c.get())) << 12)
        | (u32::from(DLL_STACK_POS_OFF.with(|c| c.get())) << 13)
        | (u32::from(regfile::test_off()) << 14);
    #[cfg(not(test))]
    let test_bits = 0u32;
    base | (jit_tag & 0xF) | (aot_on << 10) | test_bits
}

/// Compact out-of-line interpreter burst loop: executes pre-decoded [`fast::FastOp`] blocks
/// and chains directly across 0-hash self-loops, cached direct `(taken_slot, fall_slot)` edges,
/// and stamped 32-byte `BlkEntry` slots for `jcc`, `jmp`, `call`, `ret`, and fallthrough up
/// to `budget` ops (`<= INTERP_BURST_BUDGET`).
///
/// Only entered on blocks stamped with `fast_gen` (guaranteed `e.gen == 0`, non-generic start,
/// sticky-interpreted, non-host, non-hook/chkstk/IAT/DLL-stub). Straight-line non-branch runs
/// omit per-op `cpu.rip` stores and `(*nx).ip == cpu.rip` checks, materializing `cpu.rip` at
/// block ends, branches, calls, and exits. Exits cleanly back to `emu_run_region` on index
/// miss, `K_GENERIC`, host/special transfer, budget expiry, or fault.
#[inline(never)]
unsafe fn interp_burst(
    cpu: &mut Cpu,
    blocks_ptr: *mut BlockCache,
    mut cur_slot: usize,
    cur_off: usize,
    cur_len: usize,
    fast_gen: u32,
    budget: u32,
) -> BurstExit {
    #[cfg(test)]
    TEST_INTERP_BURSTS.with(|c| c.set(c.get().wrapping_add(1)));

    let ops_base = (*blocks_ptr).ops.as_ptr();
    let index_base = (*blocks_ptr).index.as_mut_ptr();
    let mut blk_start = ops_base.add(cur_off);
    let mut blk_ip = (*blk_start).ip;
    let mut blk_len = cur_len;
    let mut op = blk_start;
    let mut left = cur_len - 1;
    let mut n = 0u32;

    macro_rules! resolve_chain {
        ($target:expr, $is_fall:expr) => {{
            let tgt: u64 = $target;
            if tgt == blk_ip {
                Some((cur_slot, blk_start, blk_len))
            } else {
                let cur_ent = index_base.add(cur_slot);
                let cached = if $is_fall { (*cur_ent).fall_slot } else { (*cur_ent).taken_slot };
                let mut hit: Option<(usize, *const fast::FastOp, usize)> = None;
                if cached != 0 {
                    let s = (cached - 1) as usize;
                    let e = *index_base.add(s);
                    if e.ip == tgt && e.fast_gen == fast_gen {
                        let ptr = ops_base.add(e.off as usize);
                        if (*ptr).ip == tgt {
                            hit = Some((s, ptr, e.len as usize));
                        }
                    }
                }
                if hit.is_none() {
                    let s = BlockCache::slot(tgt);
                    let e = *index_base.add(s);
                    if e.ip == tgt && e.fast_gen == fast_gen {
                        let ptr = ops_base.add(e.off as usize);
                        if (*ptr).ip == tgt {
                            let enc = (s + 1) as u32;
                            if $is_fall {
                                (*cur_ent).fall_slot = enc;
                            } else {
                                (*cur_ent).taken_slot = enc;
                            }
                            hit = Some((s, ptr, e.len as usize));
                        }
                    }
                }
                hit
            }
        }};
    }

    loop {
        if left == 0 {
            cpu.rip = (*op).ip.wrapping_add((*op).len as u64);
        }
        let mut fx = fast::exec(cpu, &*op);
        n += 1;
        if matches!(fx, fast::Fx::FuseJcc) {
            // `fast::fuse` guarantees the next slot of this block is the paired `K_JCC` (end of block).
            op = op.add(1);
            left -= 1;
            cpu.rip = (*op).ip.wrapping_add((*op).len as u64);
            fx = fast::exec(cpu, &*op);
            n += 1;
        }
        match fx {
            fast::Fx::Done => {
                if left != 0 {
                    let nx = op.add(1);
                    debug_assert_eq!((*nx).ip, (*op).ip.wrapping_add((*op).len as u64));
                    if n < budget && (*nx).kind != fast::K_GENERIC {
                        op = nx;
                        left -= 1;
                        continue;
                    }
                    cpu.rip = (*nx).ip;
                    return BurstExit {
                        fx: fast::Fx::Done,
                        steps: n,
                        last_ip: (*op).ip,
                        next_off: nx.offset_from(ops_base) as usize,
                        next_left: left,
                    };
                }
                if n >= budget {
                    return BurstExit {
                        fx: fast::Fx::Done,
                        steps: n,
                        last_ip: (*op).ip,
                        next_off: 0,
                        next_left: 0,
                    };
                }
                let target = cpu.rip;
                let is_fall = target == (*op).ip.wrapping_add((*op).len as u64);
                if let Some((slot, ptr, len)) = resolve_chain!(target, is_fall) {
                    #[cfg(test)]
                    TEST_INTERP_BURST_CHAINS.with(|c| c.set(c.get().wrapping_add(1)));
                    cur_slot = slot;
                    blk_ip = target;
                    blk_start = ptr;
                    blk_len = len;
                    op = ptr;
                    left = len - 1;
                    continue;
                }
                return BurstExit {
                    fx: fast::Fx::Done,
                    steps: n,
                    last_ip: (*op).ip,
                    next_off: 0,
                    next_left: 0,
                };
            }
            fast::Fx::JccNear(target) | fast::Fx::Jmp(target) => {
                if n < budget {
                    if let Some((slot, ptr, len)) = resolve_chain!(target, false) {
                        #[cfg(test)]
                        TEST_INTERP_BURST_CHAINS.with(|c| c.set(c.get().wrapping_add(1)));
                        cpu.rip = target;
                        cur_slot = slot;
                        blk_ip = target;
                        blk_start = ptr;
                        blk_len = len;
                        op = ptr;
                        left = len - 1;
                        continue;
                    }
                }
                return BurstExit {
                    fx,
                    steps: n,
                    last_ip: (*op).ip,
                    next_off: 0,
                    next_left: 0,
                };
            }
            fast::Fx::Call(target) => {
                if n < budget {
                    let new_rsp = cpu.gpr[4].wrapping_sub(8);
                    if regfile_ext::is_valid_guest_range(new_rsp, 8) {
                        if let Some((slot, ptr, len)) = resolve_chain!(target, false) {
                            #[cfg(test)]
                            TEST_INTERP_BURST_CHAINS.with(|c| c.set(c.get().wrapping_add(1)));
                            cpu.gpr[4] = new_rsp;
                            cpu.write_mem(new_rsp, 64, cpu.rip);
                            cpu.rip = target;
                            cur_slot = slot;
                            blk_ip = target;
                            blk_start = ptr;
                            blk_len = len;
                            op = ptr;
                            left = len - 1;
                            continue;
                        }
                    }
                }
                return BurstExit {
                    fx,
                    steps: n,
                    last_ip: (*op).ip,
                    next_off: 0,
                    next_left: 0,
                };
            }
            fast::Fx::FuseJcc => unreachable!(),
        }
    }
}

/// One emulator run (see [`emu_run_full_rax`]); `eh_opaque` = its EH record's opacity.
#[inline(never)]
unsafe fn emu_run_region(
    entry_ip: u64,
    args: [u64; 8],
    xmm_args: [u128; 4],
    stack_args: [u64; 8],
    rax_in: u64,
    eh_opaque: bool,
) -> (u64, u128) {
    if entry_ip == 0 {
        // A host/lifted call through a null pointer (`call_va*(0)`): the loop below does not
        // run and the caller goes on with rax as is (natively a fault).
        note_null_call();
    }
    // Lifted bytes live below the innermost guest -> host transition (`crate::lstack` "Stack
    // account"): the region starts that much lower, where a native callee would sit.
    let lifted = crate::lstack::lifted_depth(crate::callctx::ctx()) as u64;
    let (sp_ptr, saved_sp, sp_top, stack_lo, stack_floor, stack_hi, blocks_ptr, jit_ptr, overflow) = TLS_EMU.with(|cell| {
        let opt = &mut *cell.get();
        let state = opt.get_or_insert_with(ThreadEmuState::new);
        if state.sp < state.stack_min || state.sp > state.stack_top {
            state.sp = state.stack_top;
        }
        let saved = state.sp;
        let start = saved.saturating_sub(lifted);
        let overflow = lifted != 0 && start < state.stack_min + GUEST_FLOOR_MARGIN;
        // Win64 ABI: at function entry (right after CALL pushes 8-byte return address),
        // RSP % 16 == 8, with 32-byte shadow space at [RSP+8..RSP+0x28] and stack args at [RSP+0x28..].
        let top = (start.wrapping_sub(320) & !0xFu64).wrapping_sub(8);
        if !overflow {
            state.sp = (top - 256) & !0xFu64;
        }
        let alloc_base = state.stack_min.saturating_sub(GUEST_STACK_MIN_OFF);
        let stack_floor = state.stack_min + GUEST_FLOOR_MARGIN;
        (
            &mut state.sp as *mut u64,
            saved,
            top,
            alloc_base,
            stack_floor,
            state.stack_top,
            &mut state.blocks as *mut BlockCache,
            &mut state.jit as *mut jit::JitCache,
            overflow,
        )
    });
    if overflow {
        // Nothing changed yet (guest sp, EH chain): as a guard-page hit at the callee's entry.
        crate::lstack::raise_stack_overflow(crate::lstack::Overflow::Region);
    }
    // Keep `TEB.StackBase` (`gs:[0x08]`) / `TEB.StackLimit` (`gs:[0x10]`) matching the active
    // heap guest stack when the TEB currently points at the host pthread's C stack, so unhooked
    // `__chkstk` helpers (e.g. inside packed binaries) never probe downward from the host stack
    // limit into the host guard page. Leave custom pivoted stacks (fibers / tests) untouched.
    let saved_teb_stack = {
        let teb = teb_self();
        if teb != 0 {
            let old_base = rd::<u64>(teb + 0x08);
            let old_limit = rd::<u64>(teb + 0x10);
            let host_sp = &overflow as *const bool as u64;
            if old_limit <= host_sp && host_sp <= old_base {
                let old_dealloc = rd::<u64>(teb + 0x1478);
                wr::<u64>(teb + 0x08, stack_hi);
                wr::<u64>(teb + 0x10, stack_floor);
                wr::<u64>(teb + 0x1478, stack_lo);
                Some((teb, old_base, old_limit, old_dealloc))
            } else {
                None
            }
        } else {
            None
        }
    };
    let _sp_guard = SpGuard { sp_ptr, saved_sp, saved_teb_stack };
    let (caller_rsp, rsp_limit) = take_incoming_rsp();

    wr::<u64>(sp_top as usize, RET_SENTINEL);
    wr::<u64>((sp_top + 0x08) as usize, 0);
    wr::<u64>((sp_top + 0x10) as usize, 0);
    wr::<u64>((sp_top + 0x18) as usize, 0);
    wr::<u64>((sp_top + 0x20) as usize, 0);
    wr::<u64>((sp_top + 0x28) as usize, args[4]);
    wr::<u64>((sp_top + 0x30) as usize, args[5]);
    wr::<u64>((sp_top + 0x38) as usize, args[6]);
    wr::<u64>((sp_top + 0x40) as usize, args[7]);
    for i in 0..8 {
        wr::<u64>((sp_top + 0x48 + (i as u64) * 8) as usize, stack_args[i]);
    }
    // Only populate high stack args [sp_top+0x88..sp_top+0x108) when forwarding from a guest/lifted
    // caller frame (`caller_rsp != 0`); direct `emu_run*` callers pass at most 16 args.
    if caller_rsp != 0 {
        for i in 0..16 {
            let addr = caller_rsp.wrapping_add(0x80 + (i as u64) * 8);
            let v = if addr.wrapping_add(8) <= rsp_limit { rd::<u64>(addr as usize) } else { 0 };
            wr::<u64>((sp_top + 0x88 + (i as u64) * 8) as usize, v);
        }
    }

    let mut cpu = Cpu {
        gpr: [0; 16],
        xmm: [0; 16],
        ymm_hi: [0; 16],
        rip: entry_ip,
        cf: false,
        zf: false,
        sf: false,
        of: false,
        pfv: 1,
        df: false,
        last_bits: [64; 16],
        jx: jit::JitScratch {
            teb: teb_self() as u64,
            mxcsr: win32_rt::ucrt::runtime::guest_mxcsr() & 0xFFFF,
            ..jit::JitScratch::default()
        },
    };
    let _fpcr_guard = GuestFpcrGuard::enter(&raw const cpu.jx.mxcsr);
    if entry_ip != 0 {
        cpu.gpr[0] = rax_in; // rax
    }
    cpu.gpr[1] = args[0]; // rcx
    cpu.gpr[2] = args[1]; // rdx
    cpu.gpr[8] = args[2]; // r8
    cpu.gpr[9] = args[3]; // r9
    cpu.gpr[4] = sp_top;  // rsp
    cpu.xmm[0] = xmm_args[0];
    cpu.xmm[1] = xmm_args[1];
    cpu.xmm[2] = xmm_args[2];
    cpu.xmm[3] = xmm_args[3];
    let _cur_cpu = CurCpuGuard(CUR_CPU.with(|c| c.replace(&cpu as *const Cpu)));
    let mut eh_rec = eh::EmuEntry::new(&cpu as *const Cpu, entry_ip, eh_opaque);
    let _eh_rec = eh_rec.push();

    let mut prev_ip = entry_ip;
    const MAX_STEPS: usize = usize::MAX / 2;

    // Interpreter profiler (see `emu_prof`). With profiling off `g.sample_left` starts at
    // u32::MAX, so the per-instruction cost is a single decrement + never-taken branch.
    let prof = crate::emu_prof::thread_prof();
    let prof_mnem = prof.is_some() && crate::emu_prof::level() >= 2;
    // Level 2 counts only generic-path mnemonics (fast-path candidates); level 3 counts all.
    let prof_mnem_fast = prof.is_some() && crate::emu_prof::level() >= 3;
    // Context for host calls made from translated code (`jit_host_xfer`).
    cpu.jx.ctx_sp = sp_ptr as usize;
    cpu.jx.ctx_blocks = blocks_ptr as usize;
    cpu.jx.ctx_prof = prof.map_or(0, |p| p as *const crate::emu_prof::ThreadProf as usize);
    cpu.jx.ctx_entry = entry_ip;
    let mut g = ProfGuard {
        prof,
        entry_ip,
        steps: 0,
        misses: 0,
        flushed_steps: 0,
        flushed_misses: 0,
        sample_left: prof.map_or(u32::MAX, |p| p.sample_ctr.get().max(1)),
    };

    let mut blk_off: usize = 0;
    let mut blk_left: usize = 0;
    let (ops_base, ins_base): (*const fast::FastOp, *const Instruction) =
        ((*blocks_ptr).ops.as_ptr(), (*blocks_ptr).arena.as_ptr());
    let verify = verify_enabled();
    cpu.jx.lb_on = u8::from(fast_last_bits_on(verify));
    // Translated code bypasses per-instruction verification and mnemonic counting.
    // Precise guest faults (difftest child only): interpreted, one instruction at a time.
    let precise = precise_faults();
    let nan_inst = crate::diag::nan_trace_installed();
    let mut pre: Option<Cpu> = None;
    let jit_on = jit::enabled() && !verify && !prof_mnem && !precise;
    // Diagnostic translator coverage counters (`AW2_JIT_STATS`, default off).
    let jstats = jit_on && jit_stats::enabled();
    let mut jstats_rip = false;
    // Set after a translator side exit: interpret the next instruction before re-entering.
    let mut jit_skip = false;
    let mut shadow: Option<Cpu> = None;
    // Primary-exe image span: guest block starts in it go through `exe_route` (import thunks,
    // special-ABI helpers). A zero span when no primary-exe replacement registry is active.
    let exe_gate = exe_route_gate();
    // Mapped-DLL span with IAT directories: guest block starts in it that are import stubs
    // go to their slot target (`dll_stub_route`). A zero span when routing is off.
    let dll_gate = dll_route_gate();
    let burst_base_flags = burst_base_mode_flags(jit_on, dll_gate);
    let mut route_hops: usize = 0;

    while cpu.rip != RET_SENTINEL && cpu.rip != 0 && g.steps < MAX_STEPS && route_hops < MAX_STEPS {
        if precise {
            if let Some((addr, write)) = FAULT_PENDING.with(|p| p.take()) {
                if let Some(p) = pre.as_ref() {
                    cpu.clone_from(p);
                }
                guest_fault_dispatch(&mut cpu, sp_ptr, addr, write);
                blk_left = 0;
                continue;
            }
            match pre.as_mut() {
                Some(p) => p.clone_from(&cpu),
                None => pre = Some(cpu.clone()),
            }
        }
        let cur_ip = cpu.rip;
        let nan_step = nan_inst && crate::diag::nan_trace_covers(cur_ip);
        // Hot-path bypass: a valid compiled block in `JitCache` (not `static_only`, no bound
        // static AOT, matching `va_gen`/`aot_epoch`/`static_aot_gen`) already passed `prep_block`
        // / `aot::check_block`, which rejects `dll_chkstk`, `dll_stack_pos`, `regfile`, and
        // `dll_stub` VAs. Skip the 5 routing probes when outside `exe_gate` and no `SKIP_VA` is armed.
        let fast_jit_code = if jit_on
            && !jit_skip
            && !nan_step
            && !in_exe_gate(exe_gate, cur_ip)
            && !regfile::has_skip_armed()
        {
            #[cfg(test)]
            let test_routed = is_routed_dll_chkstk(cur_ip)
                || is_routed_dll_stack_pos(cur_ip)
                || lookup_routed_regfile(cur_ip).is_some();
            #[cfg(not(test))]
            let test_routed = false;
            if test_routed {
                None
            } else {
                (*jit_ptr).lookup_fast_hit(cur_ip)
            }
        } else {
            None
        };
        if fast_jit_code.is_none() {
            if in_exe_gate(exe_gate, cur_ip) {
                let route = exe_route(&mut cpu, cur_ip, entry_ip);
                if route != ExeRoute::None {
                    if let Some(r) = apply_exe_route(&mut cpu, route, entry_ip, blocks_ptr, sp_ptr, prof) {
                        return r;
                    }
                    route_hops += 1;
                    continue;
                }
            }
            if is_routed_dll_chkstk(cur_ip) {
                if let Some(r) = dll_chkstk_tail(&mut cpu, sp_ptr) {
                    return r;
                }
                route_hops += 1;
                continue;
            }
            if is_routed_dll_stack_pos(cur_ip) {
                if let Some(r) = dll_stack_pos_tail(&mut cpu) {
                    return r;
                }
                route_hops += 1;
                continue;
            }
            if regfile::take_skip(cur_ip) {
                jit_skip = true;
            } else if let Some((rf, blk)) = lookup_routed_regfile(cur_ip) {
                let ex = regfile::run_chain(&mut cpu, rf, blk);
                match ex.kind {
                    regfile::RF_RET => {
                        if ex.va == RET_SENTINEL || ex.va == 0 {
                            if ex.va == 0 && entry_ip != 0 {
                                note_null_call();
                            }
                            return (cpu.abi_gpr(0), cpu.xmm[0]);
                        }
                        cpu.rip = ex.va;
                        route_hops += 1;
                        continue;
                    }
                    regfile::RF_JUMP => {
                        if ex.va == RET_SENTINEL || ex.va == 0 {
                            if ex.va == 0 && entry_ip != 0 {
                                note_null_call();
                            }
                            return (cpu.abi_gpr(0), cpu.xmm[0]);
                        }
                        if let Some(r) = apply_xfer_tail(&mut cpu, ex.va, false, entry_ip, blocks_ptr, sp_ptr, prof) {
                            return r;
                        }
                        route_hops += 1;
                        continue;
                    }
                    regfile::RF_CALL => {
                        if let Some(r) = apply_xfer_tail(&mut cpu, ex.va, true, entry_ip, blocks_ptr, sp_ptr, prof) {
                            return r;
                        }
                        route_hops += 1;
                        continue;
                    }
                    _ => {
                        cpu.rip = ex.va;
                        regfile::arm_skip(ex.va);
                        jit_skip = true;
                        route_hops += 1;
                        continue;
                    }
                }
            }
            if cur_ip.wrapping_sub(dll_gate.0) < dll_gate.1 {
                if let Some(t) = dll_stub_route(cur_ip) {
                    // The stub's `jmp [slot]`: host target = host tail call, guest = plain jump.
                    if let Some(r) = apply_fx(&mut cpu, fast::Fx::Jmp(t), entry_ip, blocks_ptr, sp_ptr, prof) {
                        return r;
                    }
                    route_hops += 1;
                    continue;
                }
            }
        }
        g.steps += 1;
        if verify {
            shadow = None;
        }
        g.sample_left = g.sample_left.wrapping_sub(1);
        if g.sample_left == 0 {
            g.sample_left = if prof.is_some() { crate::emu_prof::SAMPLE_PERIOD } else { u32::MAX };
            if let Some(p) = prof {
                p.sample_ip(cur_ip);
                p.func_progress(entry_ip, (g.steps - g.flushed_steps) as u64, g.misses - g.flushed_misses);
                g.flushed_steps = g.steps;
                g.flushed_misses = g.misses;
            }
        }
        let skipped_jit = core::mem::take(&mut jit_skip);
        if jit_on && !skipped_jit && !nan_step {
            if g.sample_left <= BLK_MAX_INSNS as u32 {
                // Take the profiler sample now rather than interpreting up to a block's
                // worth of instructions until the countdown reaches 0.
                g.sample_left = if prof.is_some() { crate::emu_prof::SAMPLE_PERIOD } else { u32::MAX };
                if let Some(p) = prof {
                    p.sample_ip(cur_ip);
                    p.func_progress(entry_ip, (g.steps - g.flushed_steps) as u64, g.misses - g.flushed_misses);
                    g.flushed_steps = g.steps;
                    g.flushed_misses = g.misses;
                }
            }
            let after_rip_exit = core::mem::take(&mut jstats_rip);
            if let Some(code) = fast_jit_code.or_else(|| (*jit_ptr).lookup(cur_ip, blocks_ptr)) {
                let mut budget = g.sample_left as i64 - 1;
                if nan_inst {
                    budget = budget.min(4096);
                }
                let (ex, executed) = (*jit_ptr).run(&mut cpu, code, budget);
                if jstats {
                    if after_rip_exit {
                        jit_stats::rip_rehit();
                    }
                    jit_stats::jit_run(ex, executed, cpu.jx.target, cpu.rip);
                    jstats_rip = ex == jit::EXIT_RIP;
                }
                g.steps = g.steps - 1 + executed as usize;
                g.sample_left = (g.sample_left as i64 - executed).max(1) as u32;
                blk_left = 0;
                let fx = match ex {
                    jit::EXIT_CALL => fast::Fx::Call(cpu.jx.target),
                    jit::EXIT_JMP => fast::Fx::Jmp(cpu.jx.target),
                    jit::EXIT_JCC => fast::Fx::JccNear(cpu.jx.target),
                    jit::EXIT_SIDE => {
                        jit_skip = true;
                        continue;
                    }
                    _ => continue,
                };
                if let Some(r) = apply_fx(&mut cpu, fx, entry_ip, blocks_ptr, sp_ptr, prof) {
                    return r;
                }
                continue;
            }
        }
        let mut head_slot: Option<(usize, usize)> = None;
        let cur_off = if blk_left != 0 && (*ops_base.add(blk_off)).ip == cur_ip {
            blk_left -= 1;
            blk_off += 1;
            blk_off - 1
        } else {
            match BlockCache::get(blocks_ptr, cur_ip, &mut g.misses) {
                Ok((off, n)) => {
                    blk_off = off + 1;
                    blk_left = n - 1;
                    head_slot = Some((BlockCache::slot(cur_ip), n));
                    off
                }
                Err(BlkErr::BadRip(mapped_ip)) => {
                    log::warn!(
                        "emu: invalid rip={:#x} (mapped={:#x}) from prev_ip={:#x} entry={:#x} step={}",
                        cur_ip,
                        mapped_ip,
                        prev_ip,
                        entry_ip,
                        g.steps
                    );
                    break;
                }
                Err(BlkErr::Invalid(bytes)) => {
                    log::warn!("emu: invalid instruction at rip={:#x} bytes={:02x?}", cur_ip, &bytes[..]);
                    break;
                }
            }
        };
        let op = &*ops_base.add(cur_off);
        prev_ip = cur_ip;
        cpu.rip = cur_ip.wrapping_add(op.len as u64);
        if jstats {
            jit_stats::interp(cur_ip, (*ins_base.add(cur_off)).mnemonic(), op.kind == fast::K_GENERIC);
        }

        if op.kind != fast::K_GENERIC {
            if prof_mnem_fast {
                if let Some(p) = prof {
                    p.mnemonic((*ins_base.add(cur_off)).mnemonic());
                }
            }
            if !verify || !fast::verifiable(op.kind) {
                let fx;
                if !verify && !prof_mnem_fast && !precise && !nan_step {
                    let burst_ok = if !prof_mnem && !jstats && !skipped_jit {
                        if let Some((slot, blk_len)) = head_slot {
                            let mflags = burst_mode_flags_from_base(burst_base_flags, jit_ptr);
                            let fast_gen = (*blocks_ptr).sync_fast_gen(mflags);
                            let ent = &mut *(*blocks_ptr).index.as_mut_ptr().add(slot);
                            if ent.ip == cur_ip
                                && ent.off as usize == cur_off
                                && ent.len as usize == blk_len
                                && ent.gen == 0
                            {
                                let stamped = if ent.fast_gen == fast_gen {
                                    true
                                } else {
                                    #[cfg(target_arch = "aarch64")]
                                    let sticky = !jit_on || (*jit_ptr).is_interp_sticky(cur_ip);
                                    #[cfg(not(target_arch = "aarch64"))]
                                    let sticky = true;
                                    let safe = sticky
                                        && !in_exe_gate(exe_gate, cur_ip)
                                        && !(crate::hooks::is_exe_chkstk_va(cur_ip)
                                            && crate::exe::aw2_registry_active())
                                        && !is_routed_dll_chkstk(cur_ip)
                                        && !is_routed_dll_stack_pos(cur_ip)
                                        && lookup_routed_regfile(cur_ip).is_none()
                                        && !is_routed_dll_stub(cur_ip)
                                        && BlockCache::resolve(blocks_ptr, cur_ip).is_none();
                                    if safe {
                                        ent.fast_gen = fast_gen;
                                    }
                                    safe
                                };
                                if stamped {
                                    Some((slot, blk_len, fast_gen))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    if let Some((slot, blk_len, fast_gen)) = burst_ok {
                        let step_cap = (MAX_STEPS - g.steps + 1).min(u32::MAX as usize) as u32;
                        let burst_budget = INTERP_BURST_BUDGET
                            .min(g.sample_left.max(1))
                            .min(step_cap)
                            .max(1);
                        let exit = interp_burst(
                            &mut cpu,
                            blocks_ptr,
                            slot,
                            cur_off,
                            blk_len,
                            fast_gen,
                            burst_budget,
                        );
                        let extra = (exit.steps - 1) as usize;
                        blk_off = exit.next_off;
                        blk_left = exit.next_left;
                        g.steps += extra;
                        g.sample_left = g.sample_left.saturating_sub(extra as u32).max(1);
                        prev_ip = exit.last_ip;
                        fx = exit.fx;
                    } else {
                        let (f, n, last) = fast::run_chain(&mut cpu, op, blk_left, g.sample_left.max(1));
                        let extra = (n - 1) as usize;
                        if jstats && extra != 0 {
                            jit_stats::interp_chain(extra as u64);
                        }
                        blk_off += extra;
                        blk_left -= extra;
                        g.steps += extra;
                        g.sample_left = g.sample_left.saturating_sub(extra as u32).max(1);
                        prev_ip = (*last).ip;
                        fx = f;
                    }
                } else {
                    let pre_fp = if nan_step {
                        Some(crate::diag::pre_ins(&cpu, &*ins_base.add(cur_off)))
                    } else {
                        None
                    };
                    let mut f = fast::exec(&mut cpu, op);
                    if let Some(ref pf) = pre_fp {
                        crate::diag::post_ins(pf, &cpu, cur_ip, &*ins_base.add(cur_off));
                    }
                    if let fast::Fx::FuseJcc = f {
                        // `fuse` guarantees the next slot of this block is the jcc.
                        let j = &*ops_base.add(blk_off);
                        blk_off += 1;
                        blk_left -= 1;
                        g.steps += 1;
                        prev_ip = j.ip;
                        cpu.rip = j.ip.wrapping_add(j.len as u64);
                        f = fast::exec(&mut cpu, j);
                    }
                    fx = f;
                }
                if let Some(r) = apply_fx(&mut cpu, fx, entry_ip, blocks_ptr, sp_ptr, prof) {
                    return r;
                }
                continue;
            }
            // Verify mode: run the fast op on a shadow copy, then let the generic
            // handler below execute for real and compare at the end of the iteration.
            let mut s = cpu.clone();
            if let fast::Fx::JccNear(t) = fast::exec(&mut s, op) {
                s.rip = t;
            }
            shadow = Some(s);
        }

        let ins = *ins_base.add(cur_off);

        let mn = ins.mnemonic();
        if prof_mnem {
            if let Some(p) = prof {
                p.mnemonic(mn);
            }
        }
        match mn {
            Mnemonic::Int3 | Mnemonic::Ud2 => {
                let code = if mn == Mnemonic::Int3 {
                    win32_rt::unwind::STATUS_BREAKPOINT
                } else {
                    win32_rt::unwind::STATUS_ILLEGAL_INSTRUCTION
                };
                let rflags = cpu.rflags();
                if let Some((new_rip, new_eflags)) = win32_rt::unwind::try_dispatch_veh_trap(
                    code,
                    cur_ip,
                    &mut cpu.gpr,
                    &mut cpu.xmm,
                    rflags,
                ) {
                    cpu.rip = new_rip;
                    cpu.set_rflags(new_eflags);
                    for lb in &mut cpu.last_bits {
                        *lb = 64;
                    }
                    continue;
                }
                if int3_returns() {
                    // A/B only (`AW2_EMU_INT3=return`): the old behaviour, the guest function
                    // "returns" whatever is in rax/xmm0.
                    break;
                }
                // Unhandled, as on Windows: no silent return (V8_Fatal -> OS::Abort -> int3
                // used to "return" and leave cohtml running on a corrupt heap).
                let mut rec = win32_rt::unwind::guest_exception_record(code, cur_ip, &[]);
                let mut ctx = win32_rt::unwind::guest_context(&cpu.gpr, &cpu.xmm, cur_ip, rflags);
                if win32_rt::unwind::emu_unhandled_exception(&mut rec, &mut ctx, true) {
                    cpu.gpr = ctx.gpr;
                    for i in 0..16 {
                        cpu.xmm[i] = u128::from_le_bytes(*ctx.xmm_mut(i));
                    }
                    cpu.rip = ctx.rip;
                    cpu.set_rflags(ctx.eflags);
                    for lb in &mut cpu.last_bits {
                        *lb = 64;
                    }
                    continue;
                }
                break;
            }
            // `int n` (`CD ib`): as on Windows (and as `lift_ops::trap` raises it): 0x29 is
            // `__fastfail` (STATUS_STACK_BUFFER_OVERRUN, information[0] = rcx; skips the vectored
            // handlers and is never resumed), 0x2c an assertion failure, 3 a breakpoint, every
            // other vector a #GP in user mode (STATUS_ACCESS_VIOLATION). It used to fall through
            // to the unimplemented no-op, so a fast-fail went on past the `int`.
            Mnemonic::Int => {
                let n = ins.immediate8();
                let code = match n {
                    0x29 => win32_rt::unwind::STATUS_STACK_BUFFER_OVERRUN,
                    0x2c => 0xC000_0420,
                    3 => win32_rt::unwind::STATUS_BREAKPOINT,
                    _ => win32_rt::unwind::STATUS_ACCESS_VIOLATION,
                };
                let fastfail = n == 0x29;
                let rflags = cpu.rflags();
                if !fastfail {
                    if let Some((new_rip, new_eflags)) =
                        win32_rt::unwind::try_dispatch_veh_trap(code, cur_ip, &mut cpu.gpr, &mut cpu.xmm, rflags)
                    {
                        cpu.rip = new_rip;
                        cpu.set_rflags(new_eflags);
                        for lb in &mut cpu.last_bits {
                            *lb = 64;
                        }
                        continue;
                    }
                }
                if int3_returns() {
                    break;
                }
                let info = [cpu.gpr[1]];
                let mut rec = win32_rt::unwind::guest_exception_record(code, cur_ip, if fastfail { &info[..] } else { &[] });
                let mut ctx = win32_rt::unwind::guest_context(&cpu.gpr, &cpu.xmm, cur_ip, rflags);
                if win32_rt::unwind::emu_unhandled_exception(&mut rec, &mut ctx, !fastfail) && !fastfail {
                    cpu.gpr = ctx.gpr;
                    for i in 0..16 {
                        cpu.xmm[i] = u128::from_le_bytes(*ctx.xmm_mut(i));
                    }
                    cpu.rip = ctx.rip;
                    cpu.set_rflags(ctx.eflags);
                    for lb in &mut cpu.last_bits {
                        *lb = 64;
                    }
                    continue;
                }
                break;
            }
            Mnemonic::Syscall | Mnemonic::Sysenter => {
                let ssn = cpu.gpr[0] as u32;
                let r10 = cpu.gpr[10];
                let rdx = cpu.gpr[2];
                let r8 = cpu.gpr[8];
                let r9 = cpu.gpr[9];
                let rsp = cpu.gpr[4];
                // Architectural x86-64 SYSCALL saves next RIP in RCX and RFLAGS in R11
                cpu.gpr[1] = cpu.rip;
                cpu.gpr[11] = cpu.rflags() as u64;
                let status_opt = if matches!(
                    ssn,
                    win32_rt::ntsyscall::ssn::NT_CONTINUE
                        | win32_rt::ntsyscall::ssn::NT_RAISE_EXCEPTION
                ) {
                    // Wrap control-transferring syscalls in an EH callout so `NtContinue` /
                    // `NtRaiseException` can land back into this emulated region via `eh::land`
                    // instead of attempting a native host `restore_context_asm` jump.
                    let host_fn = win32_rt::ntsyscall::dispatch_nt_syscall as *const () as usize;
                    eh::callout(&mut cpu, host_fn, *sp_ptr, move || {
                        let status = win32_rt::ntsyscall::dispatch_nt_syscall(
                            ssn, r10, rdx, r8, r9, rsp,
                        );
                        (status as i64 as u64, 0)
                    })
                    .map(|(st, _)| st)
                } else {
                    let status =
                        win32_rt::ntsyscall::dispatch_nt_syscall(ssn, r10, rdx, r8, r9, rsp);
                    Some(status as i64 as u64)
                };
                if let Some(status_u64) = status_opt {
                    cpu.gpr[0] = status_u64;
                    cpu.last_bits[0] = 64;
                    cpu.last_bits[1] = 64;
                    cpu.last_bits[11] = 64;
                } else {
                    // `eh::callout` landed with the restored guest `Context` already in `cpu`.
                    blk_left = 0;
                    continue;
                }
            }
            Mnemonic::Ret => {
                let rsp = cpu.gpr[4];
                let ret_addr = cpu.read_mem(rsp, 64);
                let imm = if ins.op_count() > 0 { ins.immediate16() as u64 } else { 0 };
                cpu.gpr[4] = rsp.wrapping_add(8 + imm);
                cpu.rip = ret_addr;
            }
            Mnemonic::Call => {
                let target = match ins.op_kind(0) {
                    OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16 => ins.near_branch_target(),
                    _ => cpu.read_op(&ins, 0, 64),
                };
                if let Some(r) = apply_fx(&mut cpu, fast::Fx::Call(target), entry_ip, blocks_ptr, sp_ptr, prof) {
                    return r;
                }
            }
            Mnemonic::Jmp => {
                let target = match ins.op_kind(0) {
                    OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16 => ins.near_branch_target(),
                    _ => cpu.read_op(&ins, 0, 64),
                };
                if let Some(r) = apply_fx(&mut cpu, fast::Fx::Jmp(target), entry_ip, blocks_ptr, sp_ptr, prof) {
                    return r;
                }
            }
            _ => {
                let cc = ins.condition_code();
                if cc != ConditionCode::None
                    && ins.flow_control() == FlowControl::ConditionalBranch
                    && !matches!(mn, Mnemonic::Loop | Mnemonic::Loope | Mnemonic::Loopne | Mnemonic::Jrcxz | Mnemonic::Jecxz)
                {
                    if cpu.eval_cc(cc) {
                        let target = ins.near_branch_target();
                        if ins.len() > 2 {
                            if let Some(r) = apply_fx(&mut cpu, fast::Fx::JccNear(target), entry_ip, blocks_ptr, sp_ptr, prof) {
                                return r;
                            }
                        } else {
                            cpu.rip = target;
                        }
                    }
                } else {
                    let pre_fp = if nan_step {
                        Some(crate::diag::pre_ins(&cpu, &ins))
                    } else {
                        None
                    };
                    if !cpu.exec_generic_ins(&ins) {
                        if let Some(p) = prof {
                            p.unhandled(cur_ip, mn);
                        }
                        report_unimplemented(cur_ip, mn, &ins, &cpu);
                    } else {
                        super::regfile_ext::record_x87_environment(&cpu, &ins);
                        if let Some(ref pf) = pre_fp {
                            crate::diag::post_ins(pf, &cpu, cur_ip, &ins);
                        }
                    }
                }
            }
        }
        if let Some(s) = shadow.take() {
            verify_fast(&s, &cpu, cur_ip, &ins);
        }
    }
    if precise {
        FAULT_PENDING.with(|p| p.set(None));
    }
    if g.steps >= MAX_STEPS {
        log::warn!("emu: hit MAX_STEPS ({}) at rip={:#x} (entry={:#x})", MAX_STEPS, cpu.rip, entry_ip);
    }
    if cpu.rip == 0 && entry_ip != 0 {
        // A guest `jmp`/`ret` to 0 ended the run (natively a fault).
        note_null_call();
    }
    (cpu.abi_gpr(0), cpu.xmm[0])
}
