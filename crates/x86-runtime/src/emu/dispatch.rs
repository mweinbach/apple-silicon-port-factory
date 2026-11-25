//! Win64 ↔ host call bridges (`host_call_asm`, `call_va8`, `call_host_dispatched`, `lean_call`,
//! `xfer_plain`) and unified guest control-transfer dispatch (`apply_fx`, `apply_xfer_tail`,
//! `apply_exe_route`, `jit_host_xfer`).

use super::block_cache::BlockCache;
use super::cpu::{
    precise_faults, ClearFpcrGuard, Cpu, HostFpcrGuard, FAULT_PENDING, RET_SENTINEL,
};
use super::interp::emu_run_full;
#[cfg(not(target_arch = "x86_64"))]
use super::interp::emu_run_full_rax;
#[cfg(not(target_arch = "x86_64"))]
use super::routes::{dll_stack_pos_va8, resolve_dispatch_in};
#[cfg(target_arch = "x86_64")]
use super::routes::resolve_host_callable;
#[cfg(target_arch = "aarch64")]
use super::routes::{DF_SHADOW, LEAN_FLAGS};
use super::routes::{
    dll_chkstk_call, dll_chkstk_tail, dll_stack_pos_call, dll_stack_pos_tail,
    exe_import_thunk_target, exe_route_gate, exe_route_off, exe_routed_inc, in_exe_gate,
    is_routed_dll_chkstk, is_routed_dll_stack_pos, lookup_routed_regfile, resolve_dispatch,
    resolve_dll_stub_target, to_mapped_addr, ExeRoute, DF_FLOAT_RET, DF_GAME_REPL, DF_HOST,
    DF_MARK, DF_PLAIN, DF_TRACE, DF_VALID,
};
use super::{eh, fast, jit, jit_stats, note_null_call, regfile};
use crate::exe::{rd, wr};

#[inline(always)]
pub fn set_incoming_rsp(rsp: u64, rsp_limit: u64) {
    crate::callctx::ctx().incoming_rsp.set((rsp, rsp_limit));
}

#[inline(always)]
pub fn take_incoming_rsp() -> (u64, u64) {
    crate::callctx::ctx().incoming_rsp.replace((0, 0))
}

/// The published caller window, left in place (shadow probes replay it in every run while
/// the real call still consumes it).
#[inline(always)]
pub fn peek_incoming_rsp() -> (u64, u64) {
    crate::callctx::ctx().incoming_rsp.get()
}

#[inline(always)]
pub fn set_outgoing_rsp(rsp: u64, rsp_limit: u64) {
    crate::callctx::ctx().outgoing_rsp.set((rsp, rsp_limit));
}

#[inline(always)]
pub fn take_outgoing_rsp() -> (u64, u64) {
    crate::callctx::ctx().outgoing_rsp.replace((0, 0))
}

#[inline(always)]
pub unsafe fn read_caller_extra_args(caller_rsp: u64, rsp_limit: u64) -> [u64; 8] {
    // The window is the caller's own outgoing-argument area (its lifted stack or a host
    // buffer), written by this thread just before the call: plain loads, no TSO ordering.
    // No caller rsp (no stack args published): 74% of gameplay calls (cvP,
    // `AW2_CALLVA_PROF`). Answered inline; this used to call the cold partial reader.
    if caller_rsp == 0 {
        return [0; 8];
    }
    let base = caller_rsp.wrapping_add(0x40);
    if base <= base.wrapping_add(0x40) && base.wrapping_add(0x40) <= rsp_limit {
        return core::ptr::read_unaligned(base as *const [u64; 8]);
    }
    read_caller_extra_args_partial(caller_rsp, rsp_limit)
}

#[cold]
#[inline(never)]
unsafe fn read_caller_extra_args_partial(caller_rsp: u64, rsp_limit: u64) -> [u64; 8] {
    let mut out = [0u64; 8];
    if caller_rsp != 0 {
        for i in 0..8 {
            let addr = caller_rsp.wrapping_add(0x40 + (i as u64) * 8);
            if addr.wrapping_add(8) <= rsp_limit {
                out[i] = core::ptr::read_unaligned(addr as *const u64);
            }
        }
    }
    out
}

/// [`host_call_asm`]'s argument image (x0..x7 = `gpr`, q0..q3 = `xmm`, stack arguments
/// 9..16 = `stack_args` at `[sp + 0x00..0x40]`) built by an ordinary AAPCS64 call, without
/// the callee-saved fence. Returns `x0`. Only for `DF_LEAN` targets.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
#[allow(improper_ctypes, improper_ctypes_definitions)]
pub(crate) unsafe fn lean_call(host_fn: usize, gpr: &[u64; 8], xmm: &[u128; 4], stack_args: &[u64; 8]) -> u64 {
    use core::arch::aarch64::uint64x2_t;
    // AAPCS64: the 8 integer args take x0..x7, the 4 short vectors q0..q3 (independent
    // register counter), and the integer args after the 8th go to the stack in order.
    type Lean = unsafe extern "C" fn(
        u64, u64, u64, u64, u64, u64, u64, u64,
        uint64x2_t, uint64x2_t, uint64x2_t, uint64x2_t,
        u64, u64, u64, u64, u64, u64, u64, u64,
    ) -> u64;
    let f: Lean = core::mem::transmute(core::hint::black_box(host_fn));
    let q = |v: u128| -> uint64x2_t { core::mem::transmute(v) };
    f(
        gpr[0], gpr[1], gpr[2], gpr[3], gpr[4], gpr[5], gpr[6], gpr[7],
        q(xmm[0]), q(xmm[1]), q(xmm[2]), q(xmm[3]),
        stack_args[0], stack_args[1], stack_args[2], stack_args[3],
        stack_args[4], stack_args[5], stack_args[6], stack_args[7],
    )
}

/// [`call_host_dispatched_in`] for a `LEAN_FLAGS` target (`DF_GAME_REPL`, no
/// `DF_FLOAT_RET` / `DF_MARK` / `DF_TRACE`) through [`lean_call`]: same `CallCtx` effects
/// (publishes `xmm`, consumes `xmm_out`) and result `(x0, set_xmm_ret value or xmm[0])`.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(crate) unsafe fn call_lean_in(
    c: &crate::callctx::CallCtx,
    host_fn: usize,
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    stack_args: &[u64; 8],
) -> (u64, u128) {
    c.xmm_in.set(*xmm);
    c.xmm_in_set.set(true);
    c.xmm_out.set(None);
    let r_gpr = lean_call(host_fn, gpr, xmm, stack_args);
    c.xmm_in_set.set(false);
    let r_xmm = match c.xmm_out.take() {
        Some(v) => v,
        None => xmm[0],
    };
    (r_gpr, r_xmm)
}

/// Call a guest or native function at `va_or_addr` with up to 8 Win64 integer/pointer arguments.
///
/// - On `x86_64`, calls the target directly via `extern "win64"`.
/// - On `aarch64`, checks `resolve_host_callable` first; if it is a compiled native Rust
///   function (game replacement or `win32-rt` builtin), invokes it directly; otherwise runs
///   the `iced-x86` interpreter starting at `to_mapped_addr(va_or_addr)`.
#[inline]
pub unsafe fn call_va8(
    va_or_addr: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
) -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let addr = match resolve_host_callable(va_or_addr) {
            Some(h) => h,
            None => {
                if crate::exe::in_host_text(va_or_addr as usize) {
                    log::error!(
                        "emu::call_va8: refused unregistered host __TEXT target {va_or_addr:#x}"
                    );
                    return 0;
                }
                to_mapped_addr(va_or_addr)
            }
        };
        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            core::mem::transmute(addr);
        f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // One per-thread context lookup for the whole call (each `ctx()` is a TSD
        // load + null checks that LLVM cannot merge across the stores in between).
        let c = crate::callctx::ctx();
        if crate::callprof::on() {
            let va = va_or_addr & !(1u64 << 46);
            crate::callprof::hit(c, va_or_addr, resolve_dispatch_in(c, va), crate::callprof::ENTRY_VA8);
        }
        let is_orig = (va_or_addr & (1u64 << 46)) != 0;
        let clean_va = va_or_addr & !(1u64 << 46);
        let (host_fn, flags) = if is_orig { (0, 0) } else { resolve_dispatch_in(c, clean_va) };
        #[cfg(target_arch = "aarch64")]
        if flags == LEAN_FLAGS {
            // Compiled replacement: the `DF_HOST` protocol of `call_va8_bridge` (rsp
            // windows, stack args 9..16, forwarded XMM, `rax_in`), host call via `lean_call`.
            let (mut out_rsp, mut out_limit) = c.outgoing_rsp.replace((0, 0));
            if out_rsp == 0 {
                (out_rsp, out_limit) = c.incoming_rsp.get();
            }
            let s_extra = read_caller_extra_args(out_rsp, out_limit);
            let fwd = c.xmm_in_set.get();
            let xmm = if fwd { c.xmm_in.get() } else { [rcx as u128, rdx as u128, r8 as u128, r9 as u128] };
            let prev_rax = c.rax_in.get();
            c.incoming_rsp.set((out_rsp, out_limit));
            let (r_gpr, r_xmm) =
                call_lean_in(c, host_fn, &[rcx, rdx, r8, r9, arg5, arg6, arg7, arg8], &xmm, &s_extra);
            c.rax_in.set(prev_rax);
            c.incoming_rsp.set((0, 0));
            if fwd && r_xmm != xmm[0] {
                crate::simd_ops::set_xmm_ret(r_xmm);
            }
            return r_gpr;
        }
        if flags == DF_VALID | DF_HOST | DF_PLAIN {
            // Integer-only builtin (<= 8 args, no XMM or stack args, never runs guest code):
            // the end state `call_va8_bridge` + `call_host_dispatched_in` leave, minus the
            // ABI shuffle:
            // - outgoing and incoming rsp windows consumed;
            // - forwarded XMM consumed (`xmm_in_set` false);
            // - no XMM result.
            // The bridge would also publish the builtin's leftover q0 as an XMM result when
            // XMM was forwarded; that's undefined after an integer function in Win64, and
            // callers of `call_va8` don't read it.
            // Taken even when stack args or forwarded XMM are pending: `call_lean_in`
            // leaves `xmm_in_set` up until a callee's first nested call, so the first
            // builtin call inside a lean-called function used to take the full bridge.
            c.outgoing_rsp.set((0, 0));
            c.incoming_rsp.set((0, 0));
            c.xmm_in_set.set(false);
            c.xmm_out.set(None);
            let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
                core::mem::transmute(host_fn);
            return f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
        }
        call_va8_bridge(c, is_orig, clean_va, host_fn, flags, [rcx, rdx, r8, r9, arg5, arg6, arg7, arg8])
    }
}

/// The generic part of [`call_va8`] (host functions through [`host_call_asm`], guest code
/// through [`emu_run_full`]). Out of line so the fast paths keep a small frame.
#[cfg(not(target_arch = "x86_64"))]
#[inline(never)]
unsafe fn call_va8_bridge(
    c: &crate::callctx::CallCtx,
    is_orig: bool,
    clean_va: u64,
    host_fn: usize,
    flags: u32,
    gpr: [u64; 8],
) -> u64 {
    #[cfg(target_arch = "aarch64")]
    if flags & DF_SHADOW != 0 {
        return shadow_lean_va8(c, clean_va, host_fn, gpr);
    }
    let [rcx, rdx, r8, r9, ..] = gpr;
    let (mut out_rsp, mut out_limit) = c.outgoing_rsp.replace((0, 0));
    if out_rsp == 0 {
        let (in_rsp, in_limit) = c.incoming_rsp.replace((0, 0));
        out_rsp = in_rsp;
        out_limit = in_limit;
    }
    let s_extra = read_caller_extra_args(out_rsp, out_limit);
    let fwd_xmm = if c.xmm_in_set.replace(false) { Some(c.xmm_in.get()) } else { None };
    let xmm = fwd_xmm.unwrap_or([rcx as u128, rdx as u128, r8 as u128, r9 as u128]);
    let prev_rax = c.rax_in.get();
    if !is_orig {
        if flags & DF_HOST != 0 {
            c.incoming_rsp.set((out_rsp, out_limit));
            let (r_gpr, r_xmm) = call_host_dispatched_in(c, clean_va, host_fn, flags, &gpr, &xmm, &s_extra);
            c.rax_in.set(prev_rax);
            c.incoming_rsp.set((0, 0));
            if fwd_xmm.is_some() && r_xmm != xmm[0] {
                crate::simd_ops::set_xmm_ret(r_xmm);
            }
            return r_gpr;
        }
        if is_routed_dll_stack_pos(clean_va) {
            let r_gpr = dll_stack_pos_va8(c);
            c.rax_in.set(prev_rax);
            if fwd_xmm.is_some() {
                crate::simd_ops::set_xmm_ret(xmm[0]);
            }
            return r_gpr;
        }
    }
    if crate::exe::in_host_text(clean_va as usize) {
        log::error!("emu::call_va8: refused unregistered host __TEXT target {clean_va:#x}");
        c.rax_in.set(prev_rax);
        return 0;
    }
    #[cfg(target_arch = "aarch64")]
    super::aot::record_runtime_callback(clean_va, super::aot::CallbackKind::GenericCallback);
    set_incoming_rsp(out_rsp, out_limit);
    let (r_gpr, r_xmm) = emu_run_full_rax(to_mapped_addr(clean_va) as u64, gpr, xmm, s_extra, prev_rax);
    crate::simd_ops::replace_rax_in(prev_rax);
    set_incoming_rsp(0, 0);
    if fwd_xmm.is_some() {
        crate::simd_ops::set_xmm_ret(r_xmm);
    }
    r_gpr
}

/// [`call_va8`]'s `LEAN_FLAGS` branch for a `DF_SHADOW` target: identical `CallCtx`
/// protocol and result, preceded by [`crate::shadow::probe_lean`] (which forks a probe
/// only when armed and the fn's sampling budget allows; otherwise it returns at once).
#[cfg(target_arch = "aarch64")]
#[inline(never)]
unsafe fn shadow_lean_va8(c: &crate::callctx::CallCtx, va: u64, host_fn: usize, gpr: [u64; 8]) -> u64 {
    let (mut out_rsp, mut out_limit) = c.outgoing_rsp.replace((0, 0));
    if out_rsp == 0 {
        (out_rsp, out_limit) = c.incoming_rsp.get();
    }
    let s_extra = read_caller_extra_args(out_rsp, out_limit);
    let fwd = c.xmm_in_set.get();
    let xmm = if fwd { c.xmm_in.get() } else { [gpr[0] as u128, gpr[1] as u128, gpr[2] as u128, gpr[3] as u128] };
    crate::shadow::probe_lean(c, va, host_fn, &gpr, &xmm, &s_extra, (out_rsp, out_limit), false);
    let prev_rax = c.rax_in.get();
    c.incoming_rsp.set((out_rsp, out_limit));
    let (r_gpr, r_xmm) = call_lean_in(c, host_fn, &gpr, &xmm, &s_extra);
    c.rax_in.set(prev_rax);
    c.incoming_rsp.set((0, 0));
    if fwd && r_xmm != xmm[0] {
        crate::simd_ops::set_xmm_ret(r_xmm);
    }
    r_gpr
}

/// `simd_ops::call_va8_xmm`'s `LEAN_FLAGS` branch for a `DF_SHADOW` target (see
/// [`shadow_lean_va8`]).
#[cfg(target_arch = "aarch64")]
#[inline(never)]
pub(crate) unsafe fn shadow_lean_xmm(
    c: &crate::callctx::CallCtx,
    va: u64,
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
) -> (u64, u128) {
    let (out_rsp, out_limit) = c.outgoing_rsp.replace((0, 0));
    let s_extra = read_caller_extra_args(out_rsp, out_limit);
    crate::shadow::probe_lean(c, va, host_fn, &gpr, &xmm, &s_extra, (out_rsp, out_limit), true);
    let prev_rax = c.rax_in.get();
    c.incoming_rsp.set((out_rsp, out_limit));
    let r = call_lean_in(c, host_fn, &gpr, &xmm, &s_extra);
    c.rax_in.set(prev_rax);
    c.incoming_rsp.set((0, 0));
    r
}

/// Shadow probe runs (forked children only): the guest original of `va` the way
/// `call_va8_bridge` runs an ORIG (bit 46) call, after the caller's rsp window is restored.
#[cfg_attr(target_arch = "x86_64", allow(dead_code))]
pub(crate) unsafe fn shadow_run_guest(
    entry: u64,
    rsp_window: (u64, u64),
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    s_extra: &[u64; 8],
) -> (u64, u128) {
    set_incoming_rsp(rsp_window.0, rsp_window.1);
    let r = emu_run_full(to_mapped_addr(entry) as u64, *gpr, *xmm, *s_extra);
    set_incoming_rsp(0, 0);
    r
}

/// Shadow probe runs (forked children only): the lean native call, as in [`shadow_lean_va8`].
#[cfg(target_arch = "aarch64")]
pub(crate) unsafe fn shadow_run_lean(
    host_fn: usize,
    rsp_window: (u64, u64),
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    s_extra: &[u64; 8],
) -> (u64, u128) {
    let c = crate::callctx::ctx();
    c.incoming_rsp.set(rsp_window);
    // Same incoming rax as the guest runs (`emu_run_full` starts with rax = 0): lifted
    // bodies return `get_rax_in()` on paths where the original leaves rax untouched.
    c.rax_in.set(0);
    let r = call_lean_in(c, host_fn, gpr, xmm, s_extra);
    c.incoming_rsp.set((0, 0));
    r
}

/// Call a native host function from guest code, timing it for the profiler when enabled.
#[inline(always)]
unsafe fn call_host_profiled(
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    ret_ip: u64,
    target_va: u64,
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let r = match prof {
        None => call_host_for_va(target_va, host_fn, gpr, xmm, stack_args),
        Some(p) => {
            let t0 = crate::emu_prof::now_ns();
            let r = call_host_for_va(target_va, host_fn, gpr, xmm, stack_args);
            p.host_call(host_fn, ret_ip, crate::emu_prof::now_ns().saturating_sub(t0));
            r
        }
    };
    if crate::diag::nan_trace_installed() && crate::diag::nan_trace_active() {
        let (_, flags) = resolve_dispatch(target_va);
        crate::diag::check_host_call(target_va, ret_ip, flags, &gpr, &xmm, r.0, r.1);
    }
    r
}

/// Tail-call native `host_fn` from guest code whose stack is at function-entry state
/// (return address on top): pops the return address, calls the host function with the
/// Win64 arguments, stores the result in `rax`/`xmm0`, and returns the popped return address.
#[inline(never)]
unsafe fn host_tail_call(
    cpu: &mut Cpu,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    target_va: u64,
    host_fn: usize,
) -> u64 {
    let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
    cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
    let rsp = cpu.gpr[4];
    let s5 = rd::<u64>((rsp + 0x20) as usize);
    let s6 = rd::<u64>((rsp + 0x28) as usize);
    let s7 = rd::<u64>((rsp + 0x30) as usize);
    let s8 = rd::<u64>((rsp + 0x38) as usize);
    let s_extra = [
        rd::<u64>((rsp + 0x40) as usize),
        rd::<u64>((rsp + 0x48) as usize),
        rd::<u64>((rsp + 0x50) as usize),
        rd::<u64>((rsp + 0x58) as usize),
        rd::<u64>((rsp + 0x60) as usize),
        rd::<u64>((rsp + 0x68) as usize),
        rd::<u64>((rsp + 0x70) as usize),
        rd::<u64>((rsp + 0x78) as usize),
    ];
    *sp_ptr = (rsp.wrapping_sub(256)) & !0xFu64;
    set_incoming_rsp(rsp, rsp.wrapping_add(0x100));
    let prev_rax = crate::simd_ops::replace_rax_in(cpu.gpr[0]);
    // The calling guest frame's state for an EH walk during the call (`eh::callout`).
    cpu.rip = ret_addr;
    // Only lifted `DF_GAME_REPL` functions (which take raw `u64` params without `win64-abi`
    // narrow-int guards) rely on `last_bits` argument masking; real Win32/CRT `extern "win64"`
    // functions already mask `u8`/`u16`/`bool` params via `win64-abi` and must see full 64-bit
    // guest registers even after partial-register ops (`mov cl, cl`, `add cl, imm8`).
    let mask_args = resolve_dispatch(target_va).1 & DF_GAME_REPL != 0;
    let gpr = if mask_args {
        [cpu.abi_gpr(1), cpu.abi_gpr(2), cpu.abi_gpr(8), cpu.abi_gpr(9), s5, s6, s7, s8]
    } else {
        [cpu.gpr[1], cpu.gpr[2], cpu.gpr[8], cpu.gpr[9], s5, s6, s7, s8]
    };
    let xmm = [cpu.xmm[0], cpu.xmm[1], cpu.xmm[2], cpu.xmm[3]];
    let lsa = crate::lstack::anchor_enter(crate::callctx::ctx(), *sp_ptr);
    let mxcsr_ptr = &mut cpu.jx.mxcsr as *mut u32;
    let r = {
        let _fpcr = HostFpcrGuard::enter(mxcsr_ptr);
        eh::callout(cpu, host_fn, *sp_ptr, move || {
            call_host_profiled(prof, ret_addr, target_va, host_fn, gpr, xmm, s_extra)
        })
    };
    crate::lstack::anchor_leave(crate::callctx::ctx(), lsa);
    crate::simd_ops::replace_rax_in(prev_rax);
    set_incoming_rsp(0, 0);
    match r {
        Some((r_gpr, r_xmm)) => {
            cpu.gpr[0] = r_gpr;
            cpu.xmm[0] = r_xmm;
            cpu.last_bits[0] = 64;
            ret_addr
        }
        // An EH landing in this region: continue at the landing's guest state.
        None => cpu.rip,
    }
}

/// Guest `call` into a native host function: passes the Win64 arguments (4 registers +
/// stack slots at `[rsp+0x20..]`), stores the result in `rax`/`xmm0`. `cpu.rip` must be
/// the return address. Nothing is pushed (`rsp` is unchanged), but the return address is
/// stored at `[rsp-8]`, where a real `call` would have put it: guest code that walks its own
/// frames from inside the callee reads it there (V8's `ExitFrame` takes its pc from
/// `[saved rsp - 8]`; a stale slot made `GcSafeFindCodeForInnerPointer` abort). The slot is
/// below the guest `rsp` (free under Win64, which has no red zone) and nested runs start at
/// `(rsp - 256) & !0xF`, so nothing live is overwritten. The `xfer_plain` fast path and the
/// translator's inline `plain_call`s do not store it: `DF_PLAIN` builtins never run guest code.
#[inline(never)]
unsafe fn host_call(
    cpu: &mut Cpu,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    target_va: u64,
    host_fn: usize,
) {
    let rsp = cpu.gpr[4];
    if (0x1_0008..0x8000_0000_0000).contains(&rsp) {
        wr::<u64>((rsp - 8) as usize, cpu.rip);
    }
    let s5 = rd::<u64>((rsp + 0x20) as usize);
    let s6 = rd::<u64>((rsp + 0x28) as usize);
    let s7 = rd::<u64>((rsp + 0x30) as usize);
    let s8 = rd::<u64>((rsp + 0x38) as usize);
    let s_extra = [
        rd::<u64>((rsp + 0x40) as usize),
        rd::<u64>((rsp + 0x48) as usize),
        rd::<u64>((rsp + 0x50) as usize),
        rd::<u64>((rsp + 0x58) as usize),
        rd::<u64>((rsp + 0x60) as usize),
        rd::<u64>((rsp + 0x68) as usize),
        rd::<u64>((rsp + 0x70) as usize),
        rd::<u64>((rsp + 0x78) as usize),
    ];
    *sp_ptr = (rsp.wrapping_sub(256)) & !0xFu64;
    set_incoming_rsp(rsp, rsp.wrapping_add(0x100));
    let prev_rax = crate::simd_ops::replace_rax_in(cpu.gpr[0]);
    let ret_ip = cpu.rip;
    let mask_args = resolve_dispatch(target_va).1 & DF_GAME_REPL != 0;
    let gpr = if mask_args {
        [cpu.abi_gpr(1), cpu.abi_gpr(2), cpu.abi_gpr(8), cpu.abi_gpr(9), s5, s6, s7, s8]
    } else {
        [cpu.gpr[1], cpu.gpr[2], cpu.gpr[8], cpu.gpr[9], s5, s6, s7, s8]
    };
    let xmm = [cpu.xmm[0], cpu.xmm[1], cpu.xmm[2], cpu.xmm[3]];
    let lsa = crate::lstack::anchor_enter(crate::callctx::ctx(), *sp_ptr);
    let mxcsr_ptr = &mut cpu.jx.mxcsr as *mut u32;
    let r = {
        let _fpcr = HostFpcrGuard::enter(mxcsr_ptr);
        eh::callout(cpu, host_fn, *sp_ptr, move || {
            call_host_profiled(prof, ret_ip, target_va, host_fn, gpr, xmm, s_extra)
        })
    };
    crate::lstack::anchor_leave(crate::callctx::ctx(), lsa);
    crate::simd_ops::replace_rax_in(prev_rax);
    set_incoming_rsp(0, 0);
    // `None`: an EH landing in this region; `cpu` holds the landing's guest state.
    if let Some((r_gpr, r_xmm)) = r {
        cpu.gpr[0] = r_gpr;
        cpu.xmm[0] = r_xmm;
        cpu.last_bits[0] = 64;
    }
}

/// Like [`call_host_with_xmm`], but preserves `xmm[0]` across integer game replacements
/// (registered via `hooks::in_game_repl_window`, excluding 6-byte `FF 25` IAT jump thunks to
/// builtin DLLs) unless the replacement explicitly wrote an XMM result via
/// `simd_ops::set_xmm_ret`, and preserves `rax` (`cpu.gpr[0]`) across float-returning
/// replacements that do not write `rax`.
#[inline]
pub unsafe fn call_host_for_va(
    target_va: u64,
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let (_, flags) = resolve_dispatch(target_va);
    call_host_dispatched(target_va, host_fn, flags, gpr, xmm, stack_args)
}

/// [`call_host_for_va`] with the target's `DF_*` flags already resolved
/// (see [`resolve_dispatch`]).
#[inline]
pub unsafe fn call_host_dispatched(
    target_va: u64,
    host_fn: usize,
    flags: u32,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let _fpcr = ClearFpcrGuard::enter();
    call_host_dispatched_in(crate::callctx::ctx(), target_va, host_fn, flags, &gpr, &xmm, &stack_args)
}

/// [`call_host_dispatched`] with the per-thread context already looked up.
#[inline(always)]
pub(crate) unsafe fn call_host_dispatched_in(
    c: &crate::callctx::CallCtx,
    target_va: u64,
    host_fn: usize,
    flags: u32,
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    stack_args: &[u64; 8],
) -> (u64, u128) {
    let is_game_repl = flags & DF_GAME_REPL != 0;
    let is_float_ret = flags & DF_FLOAT_RET != 0;
    // == simd_ops::set_tls_xmm_in(xmm)
    c.xmm_in.set(*xmm);
    c.xmm_in_set.set(true);
    c.xmm_out.set(None);
    let jp = if flags & DF_MARK != 0 { crate::jobprof::begin() } else { None };
    let (mut r_gpr, raw_q0) = host_call_asm(host_fn, gpr, xmm, stack_args);
    if flags & DF_MARK != 0 {
        crate::jobprof::end(jp, crate::jobprof::K_MARK, target_va, 0xffff, 0);
    }
    if flags & DF_TRACE != 0 && !is_game_repl {
        cold_trace_eat(target_va, gpr, r_gpr);
    }
    // == simd_ops::take_tls_xmm_out_opt()
    c.xmm_in_set.set(false);
    let tls_xmm = c.xmm_out.take();
    if is_game_repl && is_float_ret {
        r_gpr = c.rax_in.get();
    }
    let r_xmm = if is_float_ret {
        raw_q0
    } else {
        match tls_xmm {
            Some(v) => v,
            None if is_game_repl || (flags & DF_PLAIN != 0) => xmm[0],
            None => raw_q0,
        }
    };
    (r_gpr, r_xmm)
}

/// Invoke a native host function pointer `host_fn` passing GPR (`x0..x7` / Win64 args 1..8),
/// XMM/NEON (`q0..q3`), and stack arguments 9..16 (`stack_args[0..8]` at `[sp+0x00..sp+0x40]`),
/// returning `(rax/x0, xmm0/q0)`.
#[inline]
pub unsafe fn call_host_with_xmm(
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let (r_gpr, raw_q0) = call_host_with_xmm_raw(host_fn, gpr, xmm, stack_args);
    (r_gpr, crate::simd_ops::take_tls_xmm_out(raw_q0))
}

#[cold]
#[inline(never)]
fn cold_trace_eat(target_va: u64, gpr: &[u64; 8], r_gpr: u64) {
    let sym = win32_rt::symbolize(target_va as usize);
    log::info!(
        "[eat] {}({:#x}, {:#x}, {:#x}, {:#x}) -> {:#x}",
        sym, gpr[0], gpr[1], gpr[2], gpr[3], r_gpr
    );
}

#[inline]
unsafe fn call_host_with_xmm_raw(
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let _fpcr = ClearFpcrGuard::enter();
    crate::simd_ops::set_tls_xmm_in(xmm);
    host_call_asm(host_fn, &gpr, &xmm, &stack_args)
}

/// The register-level host call: `x0..x7` = `gpr`, `q0..q3` = `xmm`, stack args 9..16 at
/// `[sp]`, returning `(x0, q0)`. Does not touch the per-thread XMM forwarding state or
/// hardware `FPCR` (callers must hold [`HostFpcrGuard`] or [`ClearFpcrGuard`]).
#[inline(always)]
pub(super) unsafe fn host_call_asm(
    host_fn: usize,
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    stack_args: &[u64; 8],
) -> (u64, u128) {
    #[cfg(target_arch = "aarch64")]
    {
        use core::arch::aarch64::uint64x2_t;
        #[inline(always)]
        fn q(v: u128) -> uint64x2_t {
            unsafe { core::mem::transmute(v) }
        }
        // x19..x28 cannot be named as inline-asm operands (LLVM reserves
        // x19), so the fence for the arm64 callee-saved registers lives in
        // the asm string itself: everything below `blr x16` is guest-emulation
        // machinery (JIT-generated code, hand-written stubs, the interpreter)
        // that must not be assumed AAPCS64-clean. Without this fence a cached
        // spill pointer or TLS cell address in the calling Rust frame can come
        // back as a guest register value (observed: x24 = 0xffff..ffff).
        //
        // Arguments 1..8 and the four vector arguments are passed straight in
        // x0..x7 / q0..q3; only stack arguments 9..16 are copied from `x17` to
        // `[sp, #0x00..#0x40]` of the call frame.
        let r_x0: u64;
        let r_q0: uint64x2_t;
        core::arch::asm!(
            "sub sp, sp, #0xA0",
            "stp x29, x30, [sp, #0x90]",
            "add x29, sp, #0x90",
            "stp x19, x20, [sp, #0x40]",
            "stp x21, x22, [sp, #0x50]",
            "stp x23, x24, [sp, #0x60]",
            "stp x25, x26, [sp, #0x70]",
            "stp x27, x28, [sp, #0x80]",
            "ldp x9, x10, [x17, #0x00]",
            "stp x9, x10, [sp, #0x00]",
            "ldp x9, x10, [x17, #0x10]",
            "stp x9, x10, [sp, #0x10]",
            "ldp x9, x10, [x17, #0x20]",
            "stp x9, x10, [sp, #0x20]",
            "ldp x9, x10, [x17, #0x30]",
            "stp x9, x10, [sp, #0x30]",
            "blr x16",
            "ldp x19, x20, [sp, #0x40]",
            "ldp x21, x22, [sp, #0x50]",
            "ldp x23, x24, [sp, #0x60]",
            "ldp x25, x26, [sp, #0x70]",
            "ldp x27, x28, [sp, #0x80]",
            "ldp x29, x30, [sp, #0x90]",
            "add sp, sp, #0xA0",
            inout("x16") host_fn => _,
            inout("x17") stack_args.as_ptr() => _,
            inout("x0") gpr[0] => r_x0,
            inout("x1") gpr[1] => _, inout("x2") gpr[2] => _, inout("x3") gpr[3] => _,
            inout("x4") gpr[4] => _, inout("x5") gpr[5] => _, inout("x6") gpr[6] => _,
            inout("x7") gpr[7] => _,
            inout("v0") q(xmm[0]) => r_q0,
            inout("v1") q(xmm[1]) => _, inout("v2") q(xmm[2]) => _, inout("v3") q(xmm[3]) => _,
            out("x8") _, out("x9") _, out("x10") _, out("x11") _,
            out("x12") _, out("x13") _, out("x14") _, out("x15") _,
            out("v4") _, out("v5") _, out("v6") _, out("v7") _,
            out("v16") _, out("v17") _, out("v18") _, out("v19") _,
            out("v20") _, out("v21") _, out("v22") _, out("v23") _,
            out("v24") _, out("v25") _, out("v26") _, out("v27") _,
            out("v28") _, out("v29") _, out("v30") _, out("v31") _,
        );
        (r_x0, core::mem::transmute::<uint64x2_t, u128>(r_q0))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let (gpr, stack_args) = (*gpr, *stack_args);
        let _ = xmm;
        let f: unsafe extern "win64" fn(
            u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64,
        ) -> u64 = core::mem::transmute(host_fn);
        let r = f(
            gpr[0], gpr[1], gpr[2], gpr[3], gpr[4], gpr[5], gpr[6], gpr[7],
            stack_args[0], stack_args[1], stack_args[2], stack_args[3],
            stack_args[4], stack_args[5], stack_args[6], stack_args[7],
        );
        (r, r as u128)
    }
}

#[cfg(test)]
std::thread_local! {
    /// Test-only counter of `DF_VALID | DF_HOST | DF_PLAIN` leaf host calls taken on the
    /// direct `host_call_asm` fast path in [`apply_fx`] / [`apply_xfer_tail`].
    pub(super) static TEST_PLAIN_HOST_CALLS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// Direct leaf call into a `DF_VALID | DF_HOST | DF_PLAIN` integer builtin via
/// [`host_call_asm`], skipping `eh::callout`, `lstack` anchoring, `CallCtx` XMM/RSP window
/// setup, and extra stack-arg loads (`s_extra`). Only used when `prof.is_none() && !precise`.
#[inline(always)]
unsafe fn plain_host_call_asm(cpu: &mut Cpu, host_fn: usize, args_rsp: u64) {
    let r = args_rsp as usize;
    let gpr = [
        cpu.gpr[1],
        cpu.gpr[2],
        cpu.gpr[8],
        cpu.gpr[9],
        rd::<u64>(r + 0x20),
        rd::<u64>(r + 0x28),
        rd::<u64>(r + 0x30),
        rd::<u64>(r + 0x38),
    ];
    let _fpcr = ClearFpcrGuard::enter();
    let (rax, _q0) = host_call_asm(host_fn, &gpr, &[0u128; 4], &[0u64; 8]);
    cpu.gpr[0] = rax;
    cpu.last_bits[0] = 64;
    #[cfg(test)]
    TEST_PLAIN_HOST_CALLS.with(|c| c.set(c.get().wrapping_add(1)));
}

/// Check if `target` resolves to a pure-integer `DF_VALID | DF_HOST | DF_PLAIN` leaf builtin
/// eligible for [`plain_host_call_asm`] (when profiling and precise-fault mode are both off).
#[inline(always)]
fn plain_leaf_fn(
    target: u64,
    host_fn: usize,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
) -> Option<usize> {
    if prof.is_some() || precise_faults() || crate::diag::nan_trace_installed() {
        return None;
    }
    let (h, flags) = resolve_dispatch(target);
    if flags == DF_VALID | DF_HOST | DF_PLAIN && h == host_fn && h != 0 {
        Some(h)
    } else {
        None
    }
}

// ---------------------------------------------------------------------------------------------
// Unified guest control-transfer dispatch (shared between interpreter `apply_fx`/`apply_xfer_tail`
// and ARM64 JIT `jit_host_xfer`).
// ---------------------------------------------------------------------------------------------

/// Outcome of [`xfer_call`] / [`xfer_tail_loop`], specialized at compile time for interpreter
/// (`JIT = false`) vs translated-code `host_xfer` (`JIT = true`).
enum XferOutcome {
    /// Interpreter region finished and returns `(rax, xmm0)`.
    Return(u64, u128),
    /// Transfer complete; continue executing at `cpu.rip`.
    /// When `JIT`, `check_vagen` selects `jit_xfer_done(cpu)` vs returning `0`.
    Continue { check_vagen: bool },
    /// Translated code must exit to the interpreter with the given `jit::EXIT_*` code
    /// (`cpu.rip` / `cpu.jx.target` already set).
    #[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
    JitExit(u64),
}

impl XferOutcome {
    #[inline(always)]
    fn into_interp(self) -> Option<(u64, u128)> {
        match self {
            XferOutcome::Return(a, b) => Some((a, b)),
            XferOutcome::Continue { .. } => None,
            XferOutcome::JitExit(_) => unreachable!(),
        }
    }

    #[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
    #[inline(always)]
    fn into_jit(self, cpu: &mut Cpu) -> u64 {
        match self {
            XferOutcome::Continue { .. } if super::denorm::software_mode(cpu.jx.mxcsr) => jit::EXIT_RIP,
            XferOutcome::Continue { check_vagen: true } => jit_xfer_done(cpu),
            XferOutcome::Continue { check_vagen: false } => 0,
            XferOutcome::JitExit(code) => code,
            XferOutcome::Return(..) => unreachable!(),
        }
    }
}

/// Finish a `dll_chkstk_*` / `dll_stack_pos_*` sentinel return (`Some((r_gpr, r_xmm))`) for
/// either interpreter or JIT mode.
#[inline(always)]
fn finish_helper_return<const JIT: bool>(cpu: &mut Cpu, r: (u64, u128)) -> XferOutcome {
    if JIT {
        cpu.gpr[0] = r.0;
        cpu.xmm[0] = r.1;
        cpu.last_bits[0] = 64;
        cpu.rip = RET_SENTINEL;
        XferOutcome::JitExit(jit::EXIT_RIP)
    } else {
        XferOutcome::Return(r.0, r.1)
    }
}

/// Finish a `RET_SENTINEL` or `0` exit from a `DF_REGFILE` body.
#[inline(always)]
fn finish_regfile_sentinel<const JIT: bool>(cpu: &mut Cpu, va: u64, entry_ip: u64) -> XferOutcome {
    if va == 0 && entry_ip != 0 {
        note_null_call();
    }
    if JIT {
        cpu.rip = va;
        XferOutcome::JitExit(jit::EXIT_RIP)
    } else {
        XferOutcome::Return(cpu.abi_gpr(0), cpu.xmm[0])
    }
}

/// Shared tail-transfer routing loop for both [`apply_xfer_tail`] (`JIT = false`) and
/// [`jit_host_xfer`] (`JIT = true`): handles null calls, mapped-DLL IAT stubs, registered
/// primary-EXE `__chkstk`, byte-detected DLL `__chkstk`/`stack_pos`, `DF_REGFILE` chains,
/// and host tail calls in a single implementation with identical stats and tracing.
#[inline(always)]
unsafe fn xfer_tail_loop<const JIT: bool>(
    cpu: &mut Cpu,
    mut target: u64,
    mut from_call: bool,
    entry_ip: u64,
    blocks_ptr: *mut BlockCache,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    dynamic: bool,
) -> XferOutcome {
    let mut hops = 0u32;
    loop {
        if from_call && target == 0 {
            let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            note_null_call();
            cpu.gpr[0] = 0;
            cpu.last_bits[0] = 64;
            if ret_addr == RET_SENTINEL || ret_addr == 0 {
                if JIT {
                    cpu.rip = ret_addr;
                    return XferOutcome::JitExit(jit::EXIT_RIP);
                }
                return XferOutcome::Return(cpu.abi_gpr(0), cpu.xmm[0]);
            }
            if hops < 4096 && lookup_routed_regfile(ret_addr).is_some() {
                hops += 1;
                target = ret_addr;
                from_call = false;
                continue;
            }
            cpu.rip = ret_addr;
            if JIT && lookup_routed_regfile(ret_addr).is_some() {
                return XferOutcome::JitExit(jit::EXIT_RIP);
            }
            return XferOutcome::Continue { check_vagen: true };
        }
        let routed_stub = if let Some(t) = resolve_dll_stub_target(target) {
            target = t;
            true
        } else {
            false
        };
        if crate::hooks::is_exe_chkstk_va(target) && crate::exe::aw2_registry_active() {
            // Windows `__chkstk` tail call: return to the caller.
            let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            if ret_addr == RET_SENTINEL || ret_addr == 0 {
                if JIT {
                    cpu.rip = ret_addr;
                    return XferOutcome::JitExit(jit::EXIT_RIP);
                }
                return XferOutcome::Return(cpu.abi_gpr(0), cpu.xmm[0]);
            }
            if hops < 4096 && lookup_routed_regfile(ret_addr).is_some() {
                hops += 1;
                target = ret_addr;
                from_call = false;
                continue;
            }
            cpu.rip = ret_addr;
            if JIT && lookup_routed_regfile(ret_addr).is_some() {
                return XferOutcome::JitExit(jit::EXIT_RIP);
            }
            return XferOutcome::Continue { check_vagen: false };
        }
        if is_routed_dll_chkstk(target) {
            if JIT {
                jit_ras_publish(cpu);
            }
            if let Some(r) = dll_chkstk_tail(cpu, sp_ptr) {
                return finish_helper_return::<JIT>(cpu, r);
            }
            if hops < 4096 && lookup_routed_regfile(cpu.rip).is_some() {
                hops += 1;
                target = cpu.rip;
                from_call = false;
                continue;
            }
            if JIT && lookup_routed_regfile(cpu.rip).is_some() {
                return XferOutcome::JitExit(jit::EXIT_RIP);
            }
            return XferOutcome::Continue { check_vagen: true };
        }
        if is_routed_dll_stack_pos(target) {
            if JIT && !(0x1_0000..0x8000_0000_0000).contains(&(cpu.gpr[4] as usize)) {
                cpu.jx.target = target;
                return XferOutcome::JitExit(jit::EXIT_JMP);
            }
            if let Some(r) = dll_stack_pos_tail(cpu) {
                return finish_helper_return::<JIT>(cpu, r);
            }
            if hops < 4096 && lookup_routed_regfile(cpu.rip).is_some() {
                hops += 1;
                target = cpu.rip;
                from_call = false;
                continue;
            }
            if JIT && lookup_routed_regfile(cpu.rip).is_some() {
                return XferOutcome::JitExit(jit::EXIT_RIP);
            }
            return XferOutcome::Continue { check_vagen: true };
        }
        if let Some((rf, blk)) = lookup_routed_regfile(target) {
            if hops >= 4096 {
                cpu.rip = target;
                return if JIT {
                    XferOutcome::JitExit(jit::EXIT_RIP)
                } else {
                    XferOutcome::Continue { check_vagen: false }
                };
            }
            let ex = regfile::run_chain(cpu, rf, blk);
            match ex.kind {
                regfile::RF_RET => {
                    if ex.va == RET_SENTINEL || ex.va == 0 {
                        return finish_regfile_sentinel::<JIT>(cpu, ex.va, entry_ip);
                    }
                    cpu.rip = ex.va;
                    if JIT && lookup_routed_regfile(ex.va).is_some() {
                        return XferOutcome::JitExit(jit::EXIT_RIP);
                    }
                    return XferOutcome::Continue { check_vagen: true };
                }
                regfile::RF_JUMP => {
                    if ex.va == RET_SENTINEL || ex.va == 0 {
                        return finish_regfile_sentinel::<JIT>(cpu, ex.va, entry_ip);
                    }
                    hops += 1;
                    target = ex.va;
                    from_call = false;
                    continue;
                }
                regfile::RF_CALL => {
                    hops += 1;
                    target = ex.va;
                    from_call = true;
                    continue;
                }
                _ => {
                    cpu.rip = ex.va;
                    regfile::arm_skip(ex.va);
                    return if JIT {
                        XferOutcome::JitExit(jit::EXIT_SIDE)
                    } else {
                        XferOutcome::Continue { check_vagen: false }
                    };
                }
            }
        }
        if routed_stub || from_call || target != entry_ip {
            if let Some(host_fn) = BlockCache::resolve(blocks_ptr, target) {
                let ret_addr = if JIT {
                    if jit_stats::enabled() {
                        jit_stats::host_xfer(target, true, dynamic, cpu.read_mem(cpu.gpr[4], 64));
                    }
                    let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
                    if xfer_plain(cpu, target, cpu.gpr[4].wrapping_add(8)) {
                        cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
                        ret_addr
                    } else {
                        jit_ras_publish(cpu);
                        host_tail_call(cpu, sp_ptr, prof, target, host_fn)
                    }
                } else if let Some(h) = plain_leaf_fn(target, host_fn, prof) {
                    let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
                    cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
                    plain_host_call_asm(cpu, h, cpu.gpr[4]);
                    ret_addr
                } else {
                    host_tail_call(cpu, sp_ptr, prof, target, host_fn)
                };
                if ret_addr == RET_SENTINEL || ret_addr == 0 {
                    if JIT {
                        cpu.rip = ret_addr;
                        return XferOutcome::JitExit(jit::EXIT_RIP);
                    }
                    return XferOutcome::Return(cpu.gpr[0], cpu.xmm[0]);
                }
                if hops < 4096 && lookup_routed_regfile(ret_addr).is_some() {
                    hops += 1;
                    target = ret_addr;
                    from_call = false;
                    continue;
                }
                cpu.rip = ret_addr;
                if JIT && lookup_routed_regfile(ret_addr).is_some() {
                    return XferOutcome::JitExit(jit::EXIT_RIP);
                }
                return XferOutcome::Continue { check_vagen: true };
            }
        }
        break;
    }
    cpu.rip = if JIT { jit_guest_target(blocks_ptr, target) } else { target };
    XferOutcome::Continue { check_vagen: false }
}

/// Shared call-transfer routing for both [`apply_fx`] (`Fx::Call`, `JIT = false`) and
/// [`jit_host_xfer`] (`kind == 0`, `JIT = true`).
#[inline(always)]
unsafe fn xfer_call<const JIT: bool>(
    cpu: &mut Cpu,
    mut target: u64,
    entry_ip: u64,
    blocks_ptr: *mut BlockCache,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    dynamic: bool,
) -> XferOutcome {
    if let Some(t) = resolve_dll_stub_target(target) {
        target = t;
    }
    if target == 0 {
        note_null_call();
        cpu.gpr[0] = 0;
        cpu.last_bits[0] = 64;
        return XferOutcome::Continue { check_vagen: false };
    }
    if crate::hooks::is_exe_chkstk_va(target) && crate::exe::aw2_registry_active() {
        // Windows `__chkstk`: no-op.
        return XferOutcome::Continue { check_vagen: false };
    }
    if is_routed_dll_chkstk(target) {
        if JIT {
            jit_ras_publish(cpu);
        }
        if let Some(r) = dll_chkstk_call(cpu, sp_ptr) {
            return finish_helper_return::<JIT>(cpu, r);
        }
        return XferOutcome::Continue { check_vagen: true };
    }
    if is_routed_dll_stack_pos(target) {
        if JIT {
            let entry_rsp = cpu.gpr[4].wrapping_sub(8);
            if !(0x1_0000..0x8000_0000_0000).contains(&(entry_rsp as usize)) {
                cpu.jx.target = target;
                return XferOutcome::JitExit(jit::EXIT_CALL);
            }
        }
        if let Some(r) = dll_stack_pos_call(cpu) {
            return finish_helper_return::<JIT>(cpu, r);
        }
        return XferOutcome::Continue { check_vagen: true };
    }
    if let Some((rf, blk)) = lookup_routed_regfile(target) {
        let entry_rsp = cpu.gpr[4].wrapping_sub(8);
        if JIT && !(0x1_0000..0x8000_0000_0000).contains(&(entry_rsp as usize)) {
            cpu.jx.target = target;
            return XferOutcome::JitExit(jit::EXIT_CALL);
        }
        cpu.write_mem(entry_rsp, 64, cpu.rip);
        if !JIT && precise_faults() && FAULT_PENDING.with(|p| p.get().is_some()) {
            return XferOutcome::Continue { check_vagen: false };
        }
        cpu.gpr[4] = entry_rsp;
        let ex = regfile::run_chain(cpu, rf, blk);
        return match ex.kind {
            regfile::RF_RET => {
                if ex.va == RET_SENTINEL || ex.va == 0 {
                    return finish_regfile_sentinel::<JIT>(cpu, ex.va, entry_ip);
                }
                cpu.rip = ex.va;
                if JIT && lookup_routed_regfile(ex.va).is_some() {
                    return XferOutcome::JitExit(jit::EXIT_RIP);
                }
                XferOutcome::Continue { check_vagen: true }
            }
            regfile::RF_JUMP => {
                if ex.va == RET_SENTINEL || ex.va == 0 {
                    return finish_regfile_sentinel::<JIT>(cpu, ex.va, entry_ip);
                }
                xfer_tail_loop::<JIT>(cpu, ex.va, false, entry_ip, blocks_ptr, sp_ptr, prof, dynamic)
            }
            regfile::RF_CALL => {
                xfer_tail_loop::<JIT>(cpu, ex.va, true, entry_ip, blocks_ptr, sp_ptr, prof, dynamic)
            }
            _ => {
                cpu.rip = ex.va;
                regfile::arm_skip(ex.va);
                if JIT {
                    XferOutcome::JitExit(jit::EXIT_SIDE)
                } else {
                    XferOutcome::Continue { check_vagen: false }
                }
            }
        };
    }
    if let Some(host_fn) = BlockCache::resolve(blocks_ptr, target) {
        if JIT {
            if jit_stats::enabled() {
                jit_stats::host_xfer(target, false, dynamic, cpu.rip);
            }
            if xfer_plain(cpu, target, cpu.gpr[4]) {
                return XferOutcome::Continue { check_vagen: true };
            }
            jit_ras_publish(cpu);
            host_call(cpu, sp_ptr, prof, target, host_fn);
            return XferOutcome::Continue { check_vagen: true };
        }
        if let Some(h) = plain_leaf_fn(target, host_fn, prof) {
            plain_host_call_asm(cpu, h, cpu.gpr[4]);
        } else {
            host_call(cpu, sp_ptr, prof, target, host_fn);
        }
        return XferOutcome::Continue { check_vagen: false };
    }
    // Guest target: push return address and jump (`jit_guest_target` resolves exe import thunks).
    cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
    cpu.write_mem(cpu.gpr[4], 64, cpu.rip);
    cpu.rip = if JIT { jit_guest_target(blocks_ptr, target) } else { target };
    XferOutcome::Continue { check_vagen: false }
}

/// Apply a non-`None` [`ExeRoute`] in the run loop: `Some(result)` when the run is finished
/// (returned to the sentinel), `None` to continue at `cpu.rip`.
pub(super) unsafe fn apply_exe_route(
    cpu: &mut Cpu,
    route: ExeRoute,
    entry_ip: u64,
    blocks_ptr: *mut BlockCache,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
) -> Option<(u64, u128)> {
    if route != ExeRoute::None {
        exe_routed_inc();
    }
    match route {
        ExeRoute::None => None,
        // A host target becomes a host tail call, a guest target a plain jump.
        ExeRoute::Jmp(t) => apply_fx(cpu, fast::Fx::Jmp(t), entry_ip, blocks_ptr, sp_ptr, prof),
        ExeRoute::Ret => {
            let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            if ret_addr == RET_SENTINEL || ret_addr == 0 {
                return Some((cpu.abi_gpr(0), cpu.xmm[0]));
            }
            cpu.rip = ret_addr;
            None
        }
    }
}

#[inline]
pub(super) unsafe fn apply_xfer_tail(
    cpu: &mut Cpu,
    target: u64,
    from_call: bool,
    entry_ip: u64,
    blocks_ptr: *mut BlockCache,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
) -> Option<(u64, u128)> {
    xfer_tail_loop::<false>(cpu, target, from_call, entry_ip, blocks_ptr, sp_ptr, prof, false)
        .into_interp()
}

/// Apply a fast op's / translated block's control transfer (`cpu.rip` = next instruction):
/// host-function dispatch for `call`/`jmp`/near `jcc` targets, Windows `__chkstk`, and the
/// guest `call` push. Returns `Some((rax, xmm0))` when the guest function returned to the
/// interpreter's sentinel through a host tail call.
#[inline(always)]
pub(super) unsafe fn apply_fx(
    cpu: &mut Cpu,
    fx: fast::Fx,
    entry_ip: u64,
    blocks_ptr: *mut BlockCache,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
) -> Option<(u64, u128)> {
    match fx {
        fast::Fx::Done | fast::Fx::FuseJcc | fast::Fx::Yield => None,
        fast::Fx::JccNear(target) | fast::Fx::Jmp(target) => {
            apply_xfer_tail(cpu, target, false, entry_ip, blocks_ptr, sp_ptr, prof)
        }
        fast::Fx::Call(target) => {
            xfer_call::<false>(cpu, target, entry_ip, blocks_ptr, sp_ptr, prof, false).into_interp()
        }
    }
}

/// Before a host call from translated code that may run guest code again on this thread
/// (nested run of the same translation cache): store the live shadow-RAS top into the
/// cache (`jx.ctx_ras_cell`) so the nested run pushes above the caller's live entries. The
/// caller keeps its own top (`jx.ras`) afterwards; entries the nested run left are only
/// ever used after their guest address matched.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
pub(super) unsafe fn jit_ras_publish(cpu: &Cpu) {
    let cell = cpu.jx.ctx_ras_cell as *mut u64;
    if !cell.is_null() {
        *cell = cpu.jx.ras;
    }
}

/// C-ABI trampoline exported for statically linked, Apple-signed ARM64 Mach-O `.o`
/// AOT modules (`bl _port_static_aot_host_xfer` in `static_macho.rs`).
#[cfg(target_arch = "aarch64")]
#[no_mangle]
pub unsafe extern "C" fn port_static_aot_host_xfer(cpu: *mut Cpu, target: u64, kind: u64) -> u64 {
    jit_host_xfer(cpu, target, kind)
}

/// Host call (`kind & 1 == 0`, `cpu.rip` = return address) or tail jump (`kind & 1 == 1`)
/// to `target`, made from translated code through the translator's `host_xfer` stub
/// (`kind & 2`: dynamic target from `lookup_call`/`lookup_jmp`, used for `AW2_JIT_STATS`
/// only): exactly [`apply_fx`]'s `Fx::Call` / `Fx::Jmp` handling, with the run context from
/// `cpu.jx`. Returns 0 to continue in translated code at `cpu.rip` (after the host call, or
/// at the pushed/jumped-to guest target for targets that are not host functions), or
/// `jit::EXIT_RIP` to leave translated code (the guest function returned to the sentinel, or
/// the host dispatch table changed during the call).
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
pub(super) unsafe extern "C" fn jit_host_xfer(cpu: *mut Cpu, target: u64, kind: u64) -> u64 {
    let cpu = &mut *cpu;
    let dynamic = kind & 2 != 0;
    let is_tail = (kind & 1) != 0;
    let blocks = cpu.jx.ctx_blocks as *mut BlockCache;
    let sp_ptr = cpu.jx.ctx_sp as *mut u64;
    let prof = (cpu.jx.ctx_prof as *const crate::emu_prof::ThreadProf).as_ref();
    let entry_ip = cpu.jx.ctx_entry;
    let outcome = if is_tail {
        xfer_tail_loop::<true>(cpu, target, false, entry_ip, blocks, sp_ptr, prof, dynamic)
    } else {
        xfer_call::<true>(cpu, target, entry_ip, blocks, sp_ptr, prof, dynamic)
    };
    outcome.into_jit(cpu)
}

/// Guest target of a call/jmp from translated code (`jit_host_xfer`, not host-callable). An
/// exe import thunk whose slot holds guest code continues at the slot target directly,
/// as the run loop's `exe_route` would after an `EXIT_RIP`, so the exe thunk is never
/// translated. Any other target (including a host slot target, which the run loop handles)
/// is returned unchanged.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
unsafe fn jit_guest_target(blocks: *mut BlockCache, target: u64) -> u64 {
    if in_exe_gate(exe_route_gate(), target) {
        return jit_guest_thunk_target(blocks, target);
    }
    target
}

#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[cold]
#[inline(never)]
unsafe fn jit_guest_thunk_target(blocks: *mut BlockCache, target: u64) -> u64 {
    if exe_route_off() {
        return target;
    }
    match exe_import_thunk_target(target) {
        Some(t) if t != 0 && BlockCache::resolve(blocks, t).is_none() => {
            exe_routed_inc();
            t
        }
        _ => target,
    }
}

#[cfg(test)]
std::thread_local! {
    /// Test-only: host transfers served by [`xfer_plain`] on this thread.
    static TEST_XFER_PLAIN: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// Host transfers served by [`xfer_plain`] on this thread (tests).
#[cfg(test)]
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
pub(super) fn test_xfer_plain() -> u64 {
    TEST_XFER_PLAIN.with(|c| c.get())
}

#[cfg(test)]
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
pub(super) fn note_test_xfer_plain(n: u64) {
    TEST_XFER_PLAIN.with(|c| c.set(c.get() + n));
}

#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
fn is_qpc_host_fn(h: usize) -> bool {
    static QPC_FNS: std::sync::OnceLock<[usize; 4]> = std::sync::OnceLock::new();
    let fns = QPC_FNS.get_or_init(|| [
        win32_rt::kernel32::time::QueryPerformanceCounter as *const () as usize,
        win32_rt::ntdll::RtlQueryPerformanceCounter as *const () as usize,
        win32_rt::lookup("kernel32.dll", "QueryPerformanceCounter").map_or(0, |p| p as usize),
        win32_rt::lookup("ntdll.dll", "RtlQueryPerformanceCounter").map_or(0, |p| p as usize),
    ]);
    h != 0 && (h == fns[0] || h == fns[1] || h == fns[2] || h == fns[3])
}

/// `host_xfer` fast path for the targets the translator calls inline when they are static
/// (`jit::inline_host_fn`: `DF_PLAIN` builtins and `inline_variant`s) but that arrive
/// here through a register/memory-indirect call or jmp (`call rax`, `call [rbx+8]`, CFG
/// dispatch, ...): calls the function directly with the translator's inline `plain_call`
/// register contract (args = `abi_gpr` rcx/rdx/r8/r9 + `[rsp+0x20..0x40]` (`rsp` = `args_rsp`,
/// i.e. after popping the return address for tail jumps), xmm0..3; rax = x0, xmm0 = q0,
/// `last_bits[rax]` = full), without the bridge's per-thread setup, which these functions
/// never use (they never run guest code). Returns false (nothing done) when `target` has no
/// such function or a bail-out variant returned 0 (then make the real call).
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
unsafe fn xfer_plain(cpu: &mut Cpu, target: u64, args_rsp: u64) -> bool {
    let Some((h, bail, needs_xmm)) = jit::inline_host_fn_kind(target) else { return false };
    let r = args_rsp as usize;
    let gpr = [
        cpu.gpr[1],
        cpu.gpr[2],
        cpu.gpr[8],
        cpu.gpr[9],
        rd::<u64>(r + 0x20),
        rd::<u64>(r + 0x28),
        rd::<u64>(r + 0x30),
        rd::<u64>(r + 0x38),
    ];
    let xmm = if needs_xmm {
        [cpu.xmm[0], cpu.xmm[1], cpu.xmm[2], cpu.xmm[3]]
    } else {
        [0u128; 4]
    };
    let _fpcr = ClearFpcrGuard::enter();
    let (rax, q0) = host_call_asm(h, &gpr, &xmm, &[0; 8]);
    if bail && rax == 0 {
        return false;
    }
    if !bail {
        if is_qpc_host_fn(h) {
            jit::JitCache::cache_qpc_host(cpu.jx.table, target, win32_rt::kernel32::time::qpc_fast_state_ptr());
        } else {
            jit::JitCache::cache_plain_host(cpu.jx.table, target, h, needs_xmm);
        }
    }
    if jit_stats::enabled() {
        jit_stats::xfer_plain();
    }
    #[cfg(test)]
    TEST_XFER_PLAIN.with(|c| c.set(c.get() + 1));
    cpu.gpr[0] = rax;
    if needs_xmm {
        cpu.xmm[0] = q0;
    }
    cpu.last_bits[0] = 64;
    true
}

/// After a host call from translated code: keep running translated code unless the host
/// dispatch table changed meanwhile (translations embed host-callable decisions).
/// `Cpu::jx.ymm_dirty` is maintained incrementally across interpreter and translated writes
/// (nested `emu_run_region` runs use their own `Cpu`), so no 256-byte `ymm_hi` scan is needed.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline]
fn jit_xfer_done(cpu: &mut Cpu) -> u64 {
    #[cfg(debug_assertions)]
    for i in 0..16 {
        debug_assert!(
            cpu.ymm_hi[i] == 0 || (cpu.jx.ymm_dirty & (1u16 << i)) != 0,
            "ymm_dirty stale at reg {i} after jit_host_xfer: ymm_hi={:#x}, ymm_dirty={:#x}",
            cpu.ymm_hi[i],
            cpu.jx.ymm_dirty
        );
    }
    if crate::exe::va_gen() as u64 != cpu.jx.ctx_vagen {
        jit::EXIT_RIP
    } else {
        0
    }
}
