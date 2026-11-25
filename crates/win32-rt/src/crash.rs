//! Crash reports.
//!
//! [`report`] runs on fatal events: unhandled exceptions (`unwind.rs`
//! `fatal_unhandled`), faults in host code and `__fastfail` (loader
//! `signals.rs`), CRT abort/watson/invalid-parameter and unresolved-import
//! traps. It prints, through the log and into
//! `out/crash/crash-<pid>-<time>.txt` plus a structured
//! `out/crash/crash-<pid>-<time>.json` beside it (`PORT_CRASH_DIR` /
//! `AW2_CRASH_DIR` overrides):
//!
//! * the exception (code, address, parameters, kind of access violation);
//! * all general-purpose registers, annotated when they point into an image;
//! * the bytes around RIP and a disassembly of the next instructions;
//! * the faulting thread's `.pdata`-unwound guest and host backtraces (with a
//!   stack-scan fallback only when `.pdata` unwinding stops early);
//! * a `.pdata`-unwound guest + host backtrace of every other Windows thread
//!   (and the AppKit main thread).
//!
//! Frames are symbolized by the loader: `module+0xrva [va ...]` plus the
//! fndb name/owner/status for exe frames (needs `out/symbols/*.syms`, see
//! `tools/debug/export_symbols.py`), builtin export names for our thunks and
//! demangled Rust names for host frames.
//!
//! The walker mixes two strategies: PE frames (and dynamic `RtlAddFunctionTable`
//! frames) unwind with `.pdata` (`safe_virtual_unwind`), every other frame
//! (Rust/system libraries, JIT code) follows the `rbp`/`fp` chain (macOS keeps
//! frame pointers). On ARM64, guest x86-64 state captured via [`GUEST_STATE`]
//! is unwound from `.pdata` as well, both on fatal crashes and in `SIGUSR2`
//! thread snapshots (used by `other_threads` and `hitch.rs` stall reports).
//!
//! Other threads are captured with `SIGUSR2`: the handler stores the
//! interrupted context and parks (at most 2 s) while the reporting thread
//! walks its stack.

use crate::unwind::{self, Context, ExceptionRecord, RBP, RSP};
use std::cell::UnsafeCell;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Guest register snapshot `(gpr[16], rip)` of the calling thread's innermost running
/// x86-64 interpreter frame (`rip` is the address after the current instruction).
pub type GuestStateFn = fn() -> Option<([u64; 16], u64)>;
static GUEST_STATE: std::sync::OnceLock<GuestStateFn> = std::sync::OnceLock::new();

/// Register the provider for the "guest interpreter state" section of crash reports.
pub fn set_guest_state_hook(f: GuestStateFn) {
    let _ = GUEST_STATE.set(f);
}

/// Snapshot the calling thread's innermost guest x86-64 interpreter state as a
/// [`Context`] suitable for [`report`], if the thread is currently running guest code.
pub fn capture_guest_context() -> Option<Context> {
    let (gpr, rip) = GUEST_STATE.get()?()?;
    let mut ctx = Context::zeroed();
    ctx.context_flags = unwind::CONTEXT_FULL | unwind::CONTEXT_SEGMENTS;
    ctx.gpr = gpr;
    ctx.rip = rip;
    ctx.mx_csr = 0x1F80;
    Some(ctx)
}

/// Best-effort guest caller PC for native traps called from guest code on ARM64
/// (prefers `[rsp]` when guest RIP is outside a mapped PE module, else guest RIP).
pub fn guest_caller_hint() -> Option<u64> {
    let (gpr, rip) = GUEST_STATE.get()?()?;
    if loader_module(rip) {
        return Some(rip);
    }
    if let Some(caller) = read_u64(gpr[RSP]) {
        if loader_module(caller) {
            return Some(caller);
        }
    }
    (rip != 0).then_some(rip)
}

// ---------------------------------------------------------------------------
// Memory and register helpers
// ---------------------------------------------------------------------------

extern "C" {
    static mach_task_self_: u32;
    fn mach_vm_read_overwrite(task: u32, addr: u64, size: u64, data: u64, outsize: *mut u64) -> i32;
}

/// Copy memory without faulting (false if any byte is unreadable).
pub fn safe_read(addr: u64, buf: &mut [u8]) -> bool {
    if addr < 0x1000 || buf.is_empty() {
        return false;
    }
    let mut got = 0u64;
    let kr = unsafe { mach_vm_read_overwrite(mach_task_self_, addr, buf.len() as u64, buf.as_mut_ptr() as u64, &mut got) };
    kr == 0 && got == buf.len() as u64
}

pub fn read_u64(addr: u64) -> Option<u64> {
    let mut b = [0u8; 8];
    safe_read(addr, &mut b).then(|| u64::from_le_bytes(b))
}

// Darwin x86_64 mcontext layout (as presented by Rosetta).
const MC_SS: usize = 16; // __ss: rax rbx rcx rdx rdi rsi rbp rsp r8..r15 rip rflags cs fs gs
const MC_FS: usize = 16 + 168; // x86_float_state64; FXSAVE image at +8
const UC_MCONTEXT: usize = 48;

/// CONTEXT from a Darwin x86_64 `mcontext_t` (integer, control and FP state).
pub unsafe fn context_from_mcontext(mc: *const u8) -> Context {
    let ss = mc.add(MC_SS) as *const u64;
    let mut ctx = Context::zeroed();
    ctx.context_flags = unwind::CONTEXT_FULL | unwind::CONTEXT_SEGMENTS;
    // (CONTEXT index, __ss index)
    for (c, s) in [(0, 0), (1, 2), (2, 3), (3, 1), (4, 7), (5, 6), (6, 5), (7, 4)] {
        ctx.gpr[c] = *ss.add(s);
    }
    for i in 0..8 {
        ctx.gpr[8 + i] = *ss.add(8 + i);
    }
    ctx.rip = *ss.add(16);
    ctx.eflags = *ss.add(17) as u32;
    ctx.seg_cs = 0x33;
    ctx.seg_ss = 0x2B;
    ctx.seg_ds = 0x2B;
    ctx.seg_es = 0x2B;
    ctx.seg_fs = 0x53;
    ctx.seg_gs = 0x2B;
    std::ptr::copy_nonoverlapping(mc.add(MC_FS + 8), ctx.flt_save.as_mut_ptr(), 512);
    ctx.mx_csr = std::ptr::read_unaligned(ctx.flt_save.as_ptr().add(24) as *const u32);
    ctx
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn context_from_host_mcontext(mc: *const u8) -> Context {
    // Darwin arm64 __darwin_mcontext64:
    // +0:  __es (16 bytes: __far u64, __esr u32, __exception u32)
    // +16: __ss (arm_thread_state64: __x[29], __fp, __lr, __sp, __pc, __cpsr, __pad)
    let ss = mc.add(16) as *const u64;
    let mut ctx = Context::zeroed();
    ctx.context_flags = unwind::CONTEXT_FULL | unwind::CONTEXT_SEGMENTS;
    ctx.gpr[unwind::RAX] = *ss.add(0);
    ctx.gpr[unwind::RCX] = *ss.add(0);
    ctx.gpr[unwind::RDX] = *ss.add(1);
    ctx.gpr[unwind::R8] = *ss.add(2);
    ctx.gpr[unwind::R9] = *ss.add(3);
    ctx.gpr[RBP] = *ss.add(29) & 0x0000_FFFF_FFFF_FFFF;
    ctx.gpr[RSP] = *ss.add(31) & 0x0000_FFFF_FFFF_FFFF;
    ctx.rip = *ss.add(32) & 0x0000_FFFF_FFFF_FFFF;
    ctx.mx_csr = 0x1F80;
    ctx
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe fn context_from_host_mcontext(mc: *const u8) -> Context {
    context_from_mcontext(mc)
}

/// Stack range of the calling thread: the Windows stack from the TEB, or the
/// pthread stack for non-Windows threads.
pub fn current_stack_bounds() -> (u64, u64) {
    unsafe {
        let teb = crate::teb::current_raw();
        if !teb.is_null() {
            let hi: usize = crate::teb::field(teb, crate::teb::off::STACK_BASE);
            let dealloc: usize = crate::teb::field(teb, crate::teb::off::DEALLOCATION_STACK);
            let limit: usize = crate::teb::field(teb, crate::teb::off::STACK_LIMIT);
            let lo = if dealloc != 0 { dealloc } else { limit };
            if hi > lo {
                return (lo as u64, hi as u64);
            }
        }
        let me = libc::pthread_self();
        let hi = libc::pthread_get_stackaddr_np(me) as u64;
        let size = libc::pthread_get_stacksize_np(me) as u64;
        (hi.saturating_sub(size), hi)
    }
}

fn loader_module(pc: u64) -> bool {
    crate::try_loader().is_some_and(|l| l.module_from_address(pc as usize).is_some())
}

/// Plausible return address: preceded by `call rel32` (E8) or an indirect
/// `call` (FF /2, 2..7 bytes incl. ModRM/SIB/displacement), or an ARM64 `bl`/`blr`.
pub fn follows_call(ret: u64) -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        // Strip PAC bits if present (top byte / high bits above 48-bit VA).
        let clean = ret & 0x0000_FFFF_FFFF_FFFF;
        let mut insn_bytes = [0u8; 4];
        if clean >= 4 && safe_read(clean - 4, &mut insn_bytes) {
            let insn = u32::from_le_bytes(insn_bytes);
            // ARM64 `bl imm26` (0x94000000..=0x97ffffff) or `blr Rn` / `blraa` / `blrab`
            if (insn >> 26) == 0b100101 || (insn & 0xFFFF_FC1F) == 0xD63F_0000 || (insn & 0xFEFF_0000) == 0xD63F_0000 {
                return true;
            }
        }
    }
    let mut b = [0u8; 7];
    if ret < 7 || !safe_read(ret - 7, &mut b) {
        return false;
    }
    // b[7 - k] is the byte k bytes before `ret`.
    if b[2] == 0xE8 {
        return true;
    }
    (2..=7).any(|k| b[7 - k] == 0xFF && (b[8 - k] >> 3) & 7 == 2)
}

// ---------------------------------------------------------------------------
// Mixed guest/host stack walk
// ---------------------------------------------------------------------------

/// Walk from `ctx`: `.pdata` unwinding for frames inside PE images, the
/// frame-pointer chain elsewhere. `bounds` is the thread's stack range.
pub unsafe fn walk(ctx: &Context, bounds: (u64, u64), max: usize) -> Vec<u64> {
    walk_in(ctx, &[bounds], max)
}

/// pthread stack of `pt` (host frames of a Windows thread whose TEB stack is separate).
fn pthread_bounds(pt: libc::pthread_t) -> (u64, u64) {
    unsafe {
        let hi = libc::pthread_get_stackaddr_np(pt) as u64;
        let size = libc::pthread_get_stacksize_np(pt) as u64;
        (hi.saturating_sub(size), hi)
    }
}

/// Stack ranges of the calling thread: [`current_stack_bounds`] and its pthread stack.
pub fn current_stack_ranges() -> [(u64, u64); 2] {
    [current_stack_bounds(), pthread_bounds(unsafe { libc::pthread_self() })]
}

/// Sentinel return address pushed on top of a guest frame by `x86-runtime` when
/// entering `emu_run_full`. Reaching it means the guest stack unwound cleanly to
/// the host↔guest boundary.
const EMU_RET_SENTINEL: u64 = 0xDEAD_BEEF_CAFE_0000;

/// Single unwound stack frame with its unwind provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalkFrame {
    pub pc: u64,
    pub in_pe: bool,
    pub unwound_by: &'static str,
}

/// Result of unwinding a guest x86-64 stack from `(gpr, rip)` via `.pdata`.
#[derive(Debug, Clone)]
pub struct GuestWalk {
    pub frames: Vec<WalkFrame>,
    pub method: &'static str,
    pub complete: bool,
}

/// Lookup `.pdata` for `pc` without blocking or running guest callbacks.
/// Returns `(image_base, unwind_pc, runtime_function)`.
fn lookup_pdata(pc: u64, is_return_addr: bool) -> Option<(u64, u64, pe_image::RuntimeFunction)> {
    if pc < 0x1000 || pc == EMU_RET_SENTINEL {
        return None;
    }
    if is_return_addr {
        let prev = pc.saturating_sub(1);
        if let Some((base, rf)) = unwind::lookup_function_entry_no_callbacks(prev) {
            return Some((base, prev, rf));
        }
        return unwind::lookup_function_entry_no_callbacks(pc).map(|(b, rf)| (b, pc, rf));
    }
    if let Some((base, rf)) = unwind::lookup_function_entry_no_callbacks(pc) {
        let begin = base.wrapping_add(rf.begin_rva as u64);
        if follows_call(pc) {
            let prev = pc.saturating_sub(1);
            if pc == begin {
                if let Some((pbase, prf)) = unwind::lookup_function_entry_no_callbacks(prev) {
                    if pbase.wrapping_add(prf.end_rva as u64) == pc {
                        return Some((pbase, prev, prf));
                    }
                }
            } else {
                return Some((base, prev, rf));
            }
        }
        return Some((base, pc, rf));
    }
    if follows_call(pc) {
        let prev = pc.saturating_sub(1);
        if let Some((base, rf)) = unwind::lookup_function_entry_no_callbacks(prev) {
            return Some((base, prev, rf));
        }
    }
    None
}

fn is_guest_code_pc(pc: u64) -> bool {
    pc >= 0x10000
        && pc != EMU_RET_SENTINEL
        && (loader_module(pc)
            || unwind::lookup_function_entry_no_callbacks(pc).is_some()
            || unwind::lookup_function_entry_no_callbacks(pc.saturating_sub(1)).is_some())
}

/// [`walk`] over several stack ranges (a thread's guest TEB stack and its host pthread
/// stack): a frame may live in any of them. The stack pointer must grow from frame to frame
/// within one range; moving to another range is allowed (bounded by `max`).
pub unsafe fn walk_in(ctx: &Context, ranges: &[(u64, u64)], max: usize) -> Vec<u64> {
    walk_in_detailed(ctx, ranges, max).0.into_iter().map(|f| f.pc).collect()
}

/// Mixed `.pdata` (guest x86-64) + frame-pointer (host) stack walk returning per-frame
/// unwind metadata and whether the walk terminated cleanly.
pub unsafe fn walk_in_detailed(ctx: &Context, ranges: &[(u64, u64)], max: usize) -> (Vec<WalkFrame>, bool) {
    let mut eff_ranges: Vec<(u64, u64)> = ranges.iter().copied().filter(|&(lo, hi)| lo < hi).collect();
    if !eff_ranges.iter().any(|&(lo, hi)| ctx.gpr[RSP] >= lo && ctx.gpr[RSP] < hi)
        && ctx.gpr[RSP] >= 0x10000
        && read_u64(ctx.gpr[RSP]).is_some()
    {
        eff_ranges.push((ctx.gpr[RSP], ctx.gpr[RSP].saturating_add(16 << 20)));
    }
    let range_of = |p: u64| eff_ranges.iter().position(|&(lo, hi)| p >= lo && p < hi);
    let in_stack = |p: u64| range_of(p).is_some();
    let mut out = Vec::new();
    let mut cur = *ctx;
    let mut complete = true;
    while out.len() < max && cur.rip != 0 && cur.rip != EMU_RET_SENTINEL {
        let is_return_addr = !out.is_empty();
        let pdata = lookup_pdata(cur.rip, is_return_addr);
        let in_pe = loader_module(cur.rip) || pdata.is_some();
        let rsp = cur.gpr[RSP];
        if in_pe {
            let mut next = cur;
            match pdata {
                Some((base, lookup_pc, rf)) => {
                    out.push(WalkFrame { pc: cur.rip, in_pe: true, unwound_by: "pdata" });
                    if !in_stack(rsp) || read_u64(rsp).is_none() {
                        complete = false;
                        break;
                    }
                    if !unwind::safe_virtual_unwind(base, lookup_pc, rf, &mut next) {
                        complete = false;
                        break;
                    }
                }
                None => {
                    // Leaf function (or a builtin thunk): return address on top.
                    out.push(WalkFrame { pc: cur.rip, in_pe: true, unwound_by: "leaf" });
                    if !in_stack(rsp) {
                        complete = false;
                        break;
                    }
                    let Some(ret) = read_u64(rsp) else {
                        complete = false;
                        break;
                    };
                    let Some(next_rsp) = rsp.checked_add(8) else {
                        complete = false;
                        break;
                    };
                    next.rip = ret;
                    next.gpr[RSP] = next_rsp;
                }
            }
            if next.rip == 0 || next.rip == EMU_RET_SENTINEL {
                break;
            }
            let nsp = next.gpr[RSP];
            if nsp <= rsp && (range_of(nsp).is_none() || range_of(nsp) == range_of(rsp)) {
                complete = false;
                break;
            }
            cur = next;
        } else {
            out.push(WalkFrame { pc: cur.rip, in_pe: false, unwound_by: "fp" });
            let mut next = cur;
            let fp = cur.gpr[RBP] & 0x0000_FFFF_FFFF_FFFF;
            let same = range_of(fp).is_some() && range_of(fp) == range_of(rsp);
            if fp & 7 != 0 || (same && fp < rsp) || !in_stack(fp) {
                break;
            }
            let (Some(next_fp), Some(ret)) = (read_u64(fp), read_u64(fp + 8)) else { break };
            let next_fp = next_fp & 0x0000_FFFF_FFFF_FFFF;
            let ret = ret & 0x0000_FFFF_FFFF_FFFF;
            if !follows_call(ret) {
                break;
            }
            next.rip = ret;
            next.gpr[RSP] = fp + 16;
            next.gpr[RBP] = next_fp;
            let nsp = next.gpr[RSP];
            if nsp <= rsp && (range_of(nsp).is_none() || range_of(nsp) == range_of(rsp)) {
                break;
            }
            cur = next;
        }
    }
    (out, complete)
}

/// Unwind guest x86-64 frames from `(gpr, rip)` using `.pdata` (`safe_virtual_unwind`)
/// and leaf `[rsp]` unwinding, with zero heuristic stack scanning.
pub fn walk_guest_state(
    guest: Option<([u64; 16], u64)>,
    ranges: &[(u64, u64)],
    max: usize,
) -> GuestWalk {
    let Some((gpr, rip)) = guest else {
        return GuestWalk { frames: Vec::new(), method: "none", complete: true };
    };
    if max == 0 {
        return GuestWalk { frames: Vec::new(), method: "none", complete: true };
    }

    let mut eff_ranges: Vec<(u64, u64)> = ranges.iter().copied().filter(|&(lo, hi)| lo < hi).collect();
    if !eff_ranges.iter().any(|&(lo, hi)| gpr[RSP] >= lo && gpr[RSP] < hi)
        && gpr[RSP] >= 0x10000
        && read_u64(gpr[RSP]).is_some()
    {
        eff_ranges.push(bounds_for_rsp(gpr[RSP]));
    }
    let range_of = |p: u64| eff_ranges.iter().position(|&(lo, hi)| p >= lo && p < hi);
    let in_stack = |p: u64| range_of(p).is_some();

    let mut cur = Context::zeroed();
    cur.gpr = gpr;
    cur.rip = rip;

    let mut frames = Vec::new();
    let mut is_return_addr = false;
    let mut used_pdata = false;
    let mut used_leaf = false;
    let mut complete = true;

    // If `rip` is outside guest PE code (e.g. null call or native trap/thunk), check `[rsp]`
    // for the guest caller return address so unwinding still recovers the call chain.
    if !is_guest_code_pc(cur.rip) {
        if cur.rip != 0 && cur.rip != EMU_RET_SENTINEL {
            frames.push(WalkFrame { pc: cur.rip, in_pe: false, unwound_by: "leaf" });
        }
        if in_stack(cur.gpr[RSP]) {
            if let Some(caller) = read_u64(cur.gpr[RSP]) {
                if is_guest_code_pc(caller) {
                    cur.rip = caller;
                    cur.gpr[RSP] = cur.gpr[RSP].wrapping_add(8);
                    is_return_addr = true;
                } else {
                    complete = caller == 0 || caller == EMU_RET_SENTINEL;
                    cur.rip = 0;
                }
            } else {
                complete = false;
                cur.rip = 0;
            }
        } else {
            complete = false;
            cur.rip = 0;
        }
    }

    while frames.len() < max && cur.rip != 0 && cur.rip != EMU_RET_SENTINEL {
        let pdata = lookup_pdata(cur.rip, is_return_addr);
        let in_pe = loader_module(cur.rip) || pdata.is_some();
        if !in_pe {
            break;
        }
        let rsp = cur.gpr[RSP];
        match pdata {
            Some((base, lookup_pc, rf)) => {
                frames.push(WalkFrame { pc: cur.rip, in_pe: true, unwound_by: "pdata" });
                used_pdata = true;
                if !in_stack(rsp) || read_u64(rsp).is_none() {
                    complete = false;
                    break;
                }
                let mut next = cur;
                if !unwind::safe_virtual_unwind(base, lookup_pc, rf, &mut next) {
                    complete = false;
                    break;
                }
                if next.rip == 0 || next.rip == EMU_RET_SENTINEL {
                    break;
                }
                let nsp = next.gpr[RSP];
                if nsp <= rsp && (range_of(nsp).is_none() || range_of(nsp) == range_of(rsp)) {
                    complete = false;
                    break;
                }
                cur = next;
                is_return_addr = true;
            }
            None => {
                // Leaf function without `.pdata`: return address sits at `[rsp]`.
                frames.push(WalkFrame { pc: cur.rip, in_pe: true, unwound_by: "leaf" });
                used_leaf = true;
                if !in_stack(rsp) {
                    complete = false;
                    break;
                }
                let Some(ret) = read_u64(rsp) else {
                    complete = false;
                    break;
                };
                let Some(next_rsp) = rsp.checked_add(8) else {
                    complete = false;
                    break;
                };
                if ret == 0 || ret == EMU_RET_SENTINEL {
                    break;
                }
                if !is_guest_code_pc(ret) {
                    complete = false;
                    break;
                }
                cur.rip = ret;
                cur.gpr[RSP] = next_rsp;
                is_return_addr = true;
            }
        }
    }

    let method = match (used_pdata, used_leaf) {
        (true, true) => "pdata+leaf",
        (true, false) => "pdata",
        (false, true) => "leaf",
        (false, false) => "none",
    };
    GuestWalk { frames, method, complete }
}

// ---------------------------------------------------------------------------
// Other threads (SIGUSR2 snapshot)
// ---------------------------------------------------------------------------

const SNAP_IDLE: u32 = 0;
const SNAP_REQUESTED: u32 = 1;
const SNAP_CAPTURED: u32 = 2;
const SNAP_RELEASED: u32 = 3;

struct Snapshot {
    state: AtomicU32,
    target: AtomicUsize,
    ctx: UnsafeCell<Context>,
    bounds: UnsafeCell<(u64, u64)>,
    guest: UnsafeCell<Option<([u64; 16], u64)>>,
}
unsafe impl Sync for Snapshot {}

static SNAP: Snapshot = Snapshot {
    state: AtomicU32::new(SNAP_IDLE),
    target: AtomicUsize::new(0),
    ctx: UnsafeCell::new(unsafe { std::mem::zeroed() }),
    bounds: UnsafeCell::new((0, 0)),
    guest: UnsafeCell::new(None),
};

/// Structured thread snapshot returned by [`snapshot_walk_detailed`].
struct ThreadSnapshot {
    guest: GuestWalk,
    host: Vec<WalkFrame>,
}

/// pthread of the process main thread (AppKit), recorded by [`install`].
static MAIN_PTHREAD: AtomicUsize = AtomicUsize::new(0);
/// True once [`install`] has registered the `SIGUSR2` snapshot handler.
static SNAP_HANDLER_INSTALLED: AtomicBool = AtomicBool::new(false);

extern "C" fn snapshot_handler(_sig: libc::c_int, _info: *mut libc::siginfo_t, uc: *mut libc::c_void) {
    unsafe {
        if SNAP.target.load(Ordering::Acquire) != libc::pthread_self() as usize
            || SNAP.state.load(Ordering::Acquire) != SNAP_REQUESTED
        {
            return;
        }
        let mc = *((uc as *const u8).add(UC_MCONTEXT) as *const *const u8);
        *SNAP.ctx.get() = context_from_host_mcontext(mc);
        *SNAP.bounds.get() = current_stack_bounds();
        *SNAP.guest.get() = GUEST_STATE.get().and_then(|f| f());
        SNAP.state.store(SNAP_CAPTURED, Ordering::Release);
        // Park until the reporter has walked this stack (bounded: the
        // reporter may itself be stuck, e.g. on a lock this thread holds).
        let ts = libc::timespec { tv_sec: 0, tv_nsec: 100_000 };
        for _ in 0..20_000 {
            if SNAP.state.load(Ordering::Acquire) == SNAP_RELEASED {
                break;
            }
            libc::nanosleep(&ts, std::ptr::null_mut());
        }
        SNAP.state.store(SNAP_IDLE, Ordering::Release);
    }
}

/// Install the SIGUSR2 snapshot handler. Call once from the main thread.
pub fn install() {
    unsafe {
        MAIN_PTHREAD.store(libc::pthread_self() as usize, Ordering::Relaxed);
        let mut sa: libc::sigaction = std::mem::zeroed();
        sa.sa_sigaction = snapshot_handler as *const () as usize;
        sa.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK | libc::SA_RESTART;
        libc::sigemptyset(&mut sa.sa_mask);
        libc::sigaction(libc::SIGUSR2, &sa, std::ptr::null_mut());
        SNAP_HANDLER_INSTALLED.store(true, Ordering::Release);
    }
}

static SNAP_LOCK: Mutex<()> = Mutex::new(());

/// Interrupt `pthread`, unwind its guest (`.pdata`) and host (`fp`) stacks while
/// parked, then release it.
unsafe fn snapshot_walk_detailed(pthread: usize, max: usize) -> Result<ThreadSnapshot, &'static str> {
    if !SNAP_HANDLER_INSTALLED.load(Ordering::Acquire) {
        return Err("SIGUSR2 snapshot handler not installed");
    }
    let _guard = SNAP_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if SNAP.state.load(Ordering::Acquire) != SNAP_IDLE {
        // A previous target timed out and has not left the handler yet.
        std::thread::sleep(Duration::from_millis(50));
        SNAP.state.store(SNAP_IDLE, Ordering::Release);
    }
    SNAP.target.store(pthread, Ordering::Release);
    SNAP.state.store(SNAP_REQUESTED, Ordering::Release);
    if libc::pthread_kill(pthread as libc::pthread_t, libc::SIGUSR2) != 0 {
        SNAP.state.store(SNAP_IDLE, Ordering::Release);
        return Err("pthread_kill failed (thread gone?)");
    }
    let t0 = Instant::now();
    while SNAP.state.load(Ordering::Acquire) != SNAP_CAPTURED {
        if t0.elapsed() > Duration::from_millis(500) {
            SNAP.target.store(0, Ordering::Release);
            SNAP.state.store(SNAP_IDLE, Ordering::Release);
            return Err("no response to SIGUSR2 within 500 ms");
        }
        std::thread::sleep(Duration::from_micros(100));
    }
    let ctx = *SNAP.ctx.get();
    let bounds = *SNAP.bounds.get();
    let guest = (*SNAP.guest.get()).take();
    let ranges = [bounds, pthread_bounds(pthread as libc::pthread_t)];
    let (host, _) = walk_in_detailed(&ctx, &ranges, max);
    // Unwind guest frames from `.pdata` while the target thread is still parked.
    let guest_walk = walk_guest_state(guest, &ranges, max);
    SNAP.target.store(0, Ordering::Release);
    SNAP.state.store(SNAP_RELEASED, Ordering::Release);
    let t1 = Instant::now();
    while SNAP.state.load(Ordering::Acquire) != SNAP_IDLE && t1.elapsed() < Duration::from_millis(100) {
        std::thread::sleep(Duration::from_micros(50));
    }
    Ok(ThreadSnapshot { guest: guest_walk, host })
}

/// Interrupt `pthread`, walk its stack while it is parked, release it.
unsafe fn snapshot_walk(pthread: usize, max: usize) -> Result<Vec<u64>, &'static str> {
    let snap = snapshot_walk_detailed(pthread, max)?;
    let mut frames: Vec<u64> = snap.guest.frames.iter().map(|f| f.pc).collect();
    frames.extend(snap.host.iter().map(|f| f.pc));
    Ok(frames)
}

/// Walk the calling thread's mixed guest/host call stack (up to `max` frames).
#[inline(never)]
pub fn walk_current(max: usize) -> Vec<u64> {
    unsafe {
        let mut ctx = Context::zeroed();
        unwind::RtlCaptureContext(&mut ctx);
        walk_in(&ctx, &current_stack_ranges(), max)
    }
}

/// Interrupt `pthread` via `SIGUSR2` and walk its mixed guest/host stack (unless a crash report is running).
pub unsafe fn snapshot_thread(pthread: usize, max: usize) -> Result<Vec<u64>, &'static str> {
    if REPORTER.load(Ordering::Acquire) != 0 {
        return Err("crash report in progress");
    }
    snapshot_walk(pthread, max)
}

/// Process main thread `pthread_t` recorded by [`install`].
pub fn main_pthread() -> usize {
    MAIN_PTHREAD.load(Ordering::Relaxed)
}

/// Formatted `pthread_getname_np` suffix for `pthread`.
pub fn pthread_label(pthread: usize) -> String {
    thread_label(pthread)
}

// ---------------------------------------------------------------------------
// Report (text + structured JSON sidecar)
// ---------------------------------------------------------------------------

const REG_NAMES: [&str; 16] = [
    "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi",
    "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
];

#[derive(Debug, Clone)]
struct FrameJson {
    index: usize,
    address: u64,
    symbol: String,
    in_pe: bool,
    unwound_by: &'static str,
}

#[derive(Debug, Clone)]
struct ExceptionJson {
    code: u32,
    description: String,
    address: u64,
    parameters: Vec<u64>,
    access_violation: Option<(&'static str, u64)>,
}

#[derive(Debug, Clone, Default)]
struct CodeAtJson {
    address: u64,
    readable: bool,
    bytes_around: Option<String>,
    disassembly: Vec<String>,
}

#[derive(Debug, Clone)]
struct GuestStateJson {
    rip: u64,
    symbol: String,
    gpr: [u64; 16],
    unwind_method: &'static str,
    unwind_complete: bool,
    frames: Vec<FrameJson>,
}

#[derive(Debug, Clone)]
struct OtherThreadJson {
    win_tid: u32,
    name: String,
    pthread_name: String,
    label: String,
    error: Option<String>,
    guest_unwind_method: &'static str,
    guest_unwind_complete: bool,
    guest_frames: Vec<FrameJson>,
    host_frames: Vec<FrameJson>,
}

#[derive(Debug, Clone)]
struct WatchdogJson {
    frames_presented: u32,
    graphics_api: &'static str,
    last_heartbeat_kind: &'static str,
    last_heartbeat_tid: u32,
    last_stage_detail: String,
}

#[derive(Debug, Clone)]
struct CrashReportJson {
    kind: String,
    title: String,
    timestamp_unix_secs: u64,
    elapsed_ms: u64,
    pid: u32,
    truncated: bool,
    watchdog: WatchdogJson,
    win_tid: u32,
    thread_name: String,
    host_tid: usize,
    pthread_name: String,
    exception: Option<ExceptionJson>,
    gpr: [u64; 16],
    rip: u64,
    eflags: u32,
    mxcsr: u32,
    code_at_rip: CodeAtJson,
    stack_ranges: [(u64, u64); 2],
    backtrace_complete: bool,
    backtrace_frames: Vec<FrameJson>,
    host_fallback_frames: Vec<FrameJson>,
    guest_state: Option<GuestStateJson>,
    other_threads: Vec<OtherThreadJson>,
}

fn json_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn write_frames_json(out: &mut String, indent: &str, frames: &[FrameJson]) {
    if frames.is_empty() {
        out.push_str("[]");
        return;
    }
    out.push_str("[\n");
    for (i, f) in frames.iter().enumerate() {
        let comma = if i + 1 < frames.len() { "," } else { "" };
        let _ = writeln!(
            out,
            "{indent}  {{\"index\":{},\"address\":\"0x{:x}\",\"symbol\":{},\"in_pe\":{},\"unwound_by\":{}}}{comma}",
            f.index,
            f.address,
            json_quote(&f.symbol),
            f.in_pe,
            json_quote(f.unwound_by),
        );
    }
    let _ = write!(out, "{indent}]");
}

impl CrashReportJson {
    fn to_json(&self) -> String {
        let mut out = String::with_capacity(4096);
        let _ = writeln!(out, "{{");
        let _ = writeln!(out, "  \"schema_version\": 1,");
        let _ = writeln!(out, "  \"kind\": {},", json_quote(&self.kind));
        let _ = writeln!(out, "  \"title\": {},", json_quote(&self.title));
        let _ = writeln!(out, "  \"timestamp_unix_secs\": {},", self.timestamp_unix_secs);
        let _ = writeln!(out, "  \"elapsed_ms\": {},", self.elapsed_ms);
        let _ = writeln!(out, "  \"pid\": {},", self.pid);
        let _ = writeln!(out, "  \"arch\": {},", json_quote(std::env::consts::ARCH));
        let _ = writeln!(out, "  \"truncated\": {},", self.truncated);

        // Watchdog / pre-first-frame status
        let _ = writeln!(out, "  \"watchdog\": {{");
        let _ = writeln!(out, "    \"frames_presented\": {},", self.watchdog.frames_presented);
        let _ = writeln!(out, "    \"graphics_api\": {},", json_quote(self.watchdog.graphics_api));
        let _ = writeln!(out, "    \"last_heartbeat_kind\": {},", json_quote(self.watchdog.last_heartbeat_kind));
        let _ = writeln!(out, "    \"last_heartbeat_tid\": {},", self.watchdog.last_heartbeat_tid);
        let _ = writeln!(out, "    \"last_stage_detail\": {}", json_quote(&self.watchdog.last_stage_detail));
        let _ = writeln!(out, "  }},");

        // Faulting thread
        let _ = writeln!(out, "  \"faulting_thread\": {{");
        let _ = writeln!(out, "    \"win_tid\": {},", self.win_tid);
        let _ = writeln!(out, "    \"name\": {},", json_quote(&self.thread_name));
        let _ = writeln!(out, "    \"host_tid\": {},", self.host_tid);
        let _ = writeln!(out, "    \"pthread_name\": {}", json_quote(&self.pthread_name));
        let _ = writeln!(out, "  }},");

        // Exception
        match &self.exception {
            Some(ex) => {
                let _ = writeln!(out, "  \"exception\": {{");
                let _ = writeln!(out, "    \"code\": \"0x{:08x}\",", ex.code);
                let _ = writeln!(out, "    \"description\": {},", json_quote(&ex.description));
                let _ = writeln!(out, "    \"address\": \"0x{:x}\",", ex.address);
                let params: Vec<String> = ex.parameters.iter().map(|p| format!("\"0x{p:x}\"")).collect();
                let _ = writeln!(out, "    \"parameters\": [{}],", params.join(", "));
                match ex.access_violation {
                    Some((op, addr)) => {
                        let _ = writeln!(
                            out,
                            "    \"access_violation\": {{\"operation\": {}, \"address\": \"0x{:x}\"}}",
                            json_quote(op),
                            addr
                        );
                    }
                    None => {
                        let _ = writeln!(out, "    \"access_violation\": null");
                    }
                }
                let _ = writeln!(out, "  }},");
            }
            None => {
                let _ = writeln!(out, "  \"exception\": null,");
            }
        }

        // Registers
        let _ = writeln!(out, "  \"registers\": {{");
        for (i, n) in REG_NAMES.iter().enumerate() {
            let _ = writeln!(out, "    \"{n}\": \"0x{:016x}\",", self.gpr[i]);
        }
        let _ = writeln!(out, "    \"rip\": \"0x{:016x}\",", self.rip);
        let _ = writeln!(out, "    \"eflags\": \"0x{:08x}\",", self.eflags);
        let _ = writeln!(out, "    \"mxcsr\": \"0x{:08x}\"", self.mxcsr);
        let _ = writeln!(out, "  }},");

        // Code at RIP
        let _ = writeln!(out, "  \"code_at_rip\": {{");
        let _ = writeln!(out, "    \"address\": \"0x{:x}\",", self.code_at_rip.address);
        let _ = writeln!(out, "    \"readable\": {},", self.code_at_rip.readable);
        match &self.code_at_rip.bytes_around {
            Some(b) => { let _ = writeln!(out, "    \"bytes_around\": {},", json_quote(b)); }
            None => { let _ = writeln!(out, "    \"bytes_around\": null,"); }
        }
        let dis: Vec<String> = self.code_at_rip.disassembly.iter().map(|s| json_quote(s)).collect();
        let _ = writeln!(out, "    \"disassembly\": [{}]", dis.join(", "));
        let _ = writeln!(out, "  }},");

        // Primary backtrace
        let _ = writeln!(out, "  \"backtrace\": {{");
        let _ = writeln!(
            out,
            "    \"stack_ranges\": [[\"0x{:x}\", \"0x{:x}\"], [\"0x{:x}\", \"0x{:x}\"]],",
            self.stack_ranges[0].0,
            self.stack_ranges[0].1,
            self.stack_ranges[1].0,
            self.stack_ranges[1].1,
        );
        let _ = writeln!(out, "    \"complete\": {},", self.backtrace_complete);
        let _ = write!(out, "    \"frames\": ");
        write_frames_json(&mut out, "    ", &self.backtrace_frames);
        let _ = writeln!(out, ",");
        let _ = write!(out, "    \"host_fallback_frames\": ");
        write_frames_json(&mut out, "    ", &self.host_fallback_frames);
        let _ = writeln!(out);
        let _ = writeln!(out, "  }},");

        // Guest interpreter state (ARM64)
        match &self.guest_state {
            Some(gs) => {
                let _ = writeln!(out, "  \"guest_state\": {{");
                let _ = writeln!(out, "    \"rip\": \"0x{:x}\",", gs.rip);
                let _ = writeln!(out, "    \"symbol\": {},", json_quote(&gs.symbol));
                let _ = writeln!(out, "    \"registers\": {{");
                for (i, n) in REG_NAMES.iter().enumerate() {
                    let comma = if i + 1 < REG_NAMES.len() { "," } else { "" };
                    let _ = writeln!(out, "      \"{n}\": \"0x{:016x}\"{comma}", gs.gpr[i]);
                }
                let _ = writeln!(out, "    }},");
                let _ = writeln!(out, "    \"unwind_method\": {},", json_quote(gs.unwind_method));
                let _ = writeln!(out, "    \"unwind_complete\": {},", gs.unwind_complete);
                let _ = write!(out, "    \"frames\": ");
                write_frames_json(&mut out, "    ", &gs.frames);
                let _ = writeln!(out);
                let _ = writeln!(out, "  }},");
            }
            None => {
                let _ = writeln!(out, "  \"guest_state\": null,");
            }
        }

        // Other threads
        if self.other_threads.is_empty() {
            let _ = writeln!(out, "  \"other_threads\": []");
        } else {
            let _ = writeln!(out, "  \"other_threads\": [");
            for (i, t) in self.other_threads.iter().enumerate() {
                let comma = if i + 1 < self.other_threads.len() { "," } else { "" };
                let _ = writeln!(out, "    {{");
                let _ = writeln!(out, "      \"win_tid\": {},", t.win_tid);
                let _ = writeln!(out, "      \"name\": {},", json_quote(&t.name));
                let _ = writeln!(out, "      \"pthread_name\": {},", json_quote(&t.pthread_name));
                let _ = writeln!(out, "      \"label\": {},", json_quote(&t.label));
                match &t.error {
                    Some(e) => { let _ = writeln!(out, "      \"error\": {},", json_quote(e)); }
                    None => { let _ = writeln!(out, "      \"error\": null,"); }
                }
                let _ = writeln!(out, "      \"guest_unwind_method\": {},", json_quote(t.guest_unwind_method));
                let _ = writeln!(out, "      \"guest_unwind_complete\": {},", t.guest_unwind_complete);
                let _ = write!(out, "      \"guest_frames\": ");
                write_frames_json(&mut out, "      ", &t.guest_frames);
                let _ = writeln!(out, ",");
                let _ = write!(out, "      \"host_frames\": ");
                write_frames_json(&mut out, "      ", &t.host_frames);
                let _ = writeln!(out);
                let _ = writeln!(out, "    }}{comma}");
            }
            let _ = writeln!(out, "  ]");
        }
        out.push_str("}\n");
        out
    }
}

/// Thread id of the thread currently writing a report (0 = none).
static REPORTER: AtomicUsize = AtomicUsize::new(0);
static BUF: Mutex<String> = Mutex::new(String::new());
static JSON_BUF: Mutex<Option<CrashReportJson>> = Mutex::new(None);

fn line(s: impl AsRef<str>) {
    let s = s.as_ref();
    log::error!(target: "win32_rt::crash", "{s}");
    if let Ok(mut b) = BUF.try_lock() {
        b.push_str(s);
        b.push('\n');
    }
}

fn with_json(f: impl FnOnce(&mut CrashReportJson)) {
    if let Ok(mut g) = JSON_BUF.try_lock() {
        if let Some(ref mut report) = *g {
            f(report);
        }
    }
}

fn crash_dir() -> std::path::PathBuf {
    if let Some(p) = std::env::var_os("PORT_CRASH_DIR").or_else(|| std::env::var_os("AW2_CRASH_DIR")) {
        return p.into();
    }
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    if repo_root.join("Cargo.toml").is_file() {
        return repo_root.join("out/crash");
    }
    let log_dir_name = crate::config::app_config().log_dir_name;
    if let Some(home) = std::env::var_os("HOME").map(std::path::PathBuf::from) {
        return home.join("Library/Logs").join(&log_dir_name).join("crash");
    }
    std::path::PathBuf::from("/tmp").join(&log_dir_name).join("crash")
}

fn flush_file() {
    let text = match BUF.try_lock() {
        Ok(mut b) => std::mem::take(&mut *b),
        Err(_) => return,
    };
    let json_data = JSON_BUF.try_lock().ok().and_then(|mut g| g.take());
    if text.is_empty() && json_data.is_none() {
        return;
    }
    let dir = crash_dir();
    let secs = json_data.as_ref().map(|j| j.timestamp_unix_secs).unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs())
    });
    let stem = format!("crash-{}-{secs}", std::process::id());
    let path = dir.join(format!("{stem}.txt"));
    let json_path = dir.join(format!("{stem}.json"));
    let json_text = json_data.as_ref().map(CrashReportJson::to_json);
    match std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&path, &text)) {
        Ok(()) => {
            let json_written = json_text
                .as_deref()
                .map(|js| std::fs::write(&json_path, js).is_ok())
                .unwrap_or(false);
            let has_override =
                std::env::var_os("PORT_CRASH_DIR").is_some() || std::env::var_os("AW2_CRASH_DIR").is_some();
            if !cfg!(test) && !has_override {
                let log_dir_name = crate::config::app_config().log_dir_name;
                if let Some(home) = std::env::var_os("HOME").map(std::path::PathBuf::from) {
                    let log_dir = home.join("Library/Logs").join(&log_dir_name);
                    let _ = std::fs::create_dir_all(&log_dir);
                    let _ = std::fs::write(log_dir.join("last-crash.txt"), &text);
                    if let Some(ref js) = json_text {
                        let _ = std::fs::write(log_dir.join("last-crash.json"), js);
                    }
                }
            }
            let p = path.canonicalize().unwrap_or(path);
            if json_written {
                let jp = json_path.canonicalize().unwrap_or(json_path);
                log::error!(
                    target: "win32_rt::crash",
                    "crash report written to {} (json: {})",
                    p.display(),
                    jp.display()
                );
                eprintln!("crash report written to {} (json: {})", p.display(), jp.display());
            } else {
                log::error!(target: "win32_rt::crash", "crash report written to {}", p.display());
                eprintln!("crash report written to {}", p.display());
            }
        }
        Err(e) => log::error!(target: "win32_rt::crash", "could not write crash report to {}: {e}", path.display()),
    }
}

fn self_tid() -> usize {
    let mut tid = 0u64;
    unsafe { libc::pthread_threadid_np(0 as libc::pthread_t, &mut tid) };
    tid as usize
}

fn pthread_name_raw(pthread: usize) -> String {
    let mut name = [0u8; 64];
    unsafe { libc::pthread_getname_np(pthread as libc::pthread_t, name.as_mut_ptr() as *mut libc::c_char, name.len()) };
    std::ffi::CStr::from_bytes_until_nul(&name)
        .map(|c| c.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn thread_label(pthread: usize) -> String {
    let n = pthread_name_raw(pthread);
    if n.is_empty() {
        String::new()
    } else {
        format!(" pthread-name={n:?}")
    }
}

fn sym(pc: u64) -> String {
    crate::symbolize(pc as usize)
}

fn to_frame_json(frames: &[WalkFrame]) -> Vec<FrameJson> {
    frames
        .iter()
        .enumerate()
        .map(|(index, f)| FrameJson {
            index,
            address: f.pc,
            symbol: sym(f.pc),
            in_pe: f.in_pe,
            unwound_by: f.unwound_by,
        })
        .collect()
}

/// Print a crash report (and write both `.txt` and `.json` sidecar files). Safe to
/// call from any thread; concurrent reports are serialized and a fault inside the
/// report just flushes what exists.
pub unsafe fn report(title: &str, rec: Option<&ExceptionRecord>, ctx: &Context) {
    let me = self_tid();
    if let Err(owner) = REPORTER.compare_exchange(0, me, Ordering::AcqRel, Ordering::Acquire) {
        if owner == me {
            with_json(|j| j.truncated = true);
            line(format!("!!! fault inside the crash reporter ({title}); report truncated"));
            flush_file();
            return;
        }
        // Another thread is reporting: wait for it, then report this one too.
        let t0 = Instant::now();
        while REPORTER.compare_exchange(0, me, Ordering::AcqRel, Ordering::Acquire).is_err() {
            if t0.elapsed() > Duration::from_secs(10) {
                return;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    let t_start = Instant::now();
    let timestamp_unix_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    if let Some(l) = crate::try_loader() {
        l.wait_symbols();
    }
    line(format!("==================== AW2 CRASH REPORT: {title} ===================="));
    let reason_kind = if title.starts_with("unimplemented import ") {
        "unimplemented_import"
    } else if title.contains("stall") || title.contains("hang") {
        "stall"
    } else {
        "crash"
    };
    crate::process::record_exit_reason(reason_kind, title);
    let teb = crate::teb::current_raw();
    let win_tid = if teb.is_null() { 0 } else { crate::teb::thread_id() };
    let wname = crate::thread::current_thread_object()
        .and_then(|t| t.name.try_lock().ok().map(|n| n.clone()))
        .unwrap_or_default();
    let self_pt = libc::pthread_self() as usize;
    let pt_name = pthread_name_raw(self_pt);
    let wd = crate::hitch::watchdog_state();
    let [b, p] = current_stack_ranges();

    let exception_json = rec.map(|rec| {
        let description = unwind::describe_exception(rec);
        let params: Vec<u64> = rec.information[..(rec.number_parameters as usize).min(rec.information.len())].to_vec();
        let av = if rec.code == unwind::STATUS_ACCESS_VIOLATION && rec.number_parameters >= 2 {
            let op = match rec.information[0] {
                0 => "read",
                1 => "write",
                8 => "execute",
                _ => "access",
            };
            Some((op, rec.information[1]))
        } else {
            None
        };
        ExceptionJson {
            code: rec.code,
            description,
            address: rec.address as u64,
            parameters: params,
            access_violation: av,
        }
    });

    if let Ok(mut g) = JSON_BUF.try_lock() {
        *g = Some(CrashReportJson {
            kind: reason_kind.to_string(),
            title: title.to_string(),
            timestamp_unix_secs,
            elapsed_ms: 0,
            pid: std::process::id(),
            truncated: false,
            watchdog: WatchdogJson {
                frames_presented: crate::present::present_count(),
                graphics_api: crate::present::last_present_api().label(),
                last_heartbeat_kind: wd.last_heartbeat_kind.label(),
                last_heartbeat_tid: wd.last_heartbeat_tid,
                last_stage_detail: wd.last_stage_detail,
            },
            win_tid,
            thread_name: wname.clone(),
            host_tid: me,
            pthread_name: pt_name,
            exception: exception_json,
            gpr: ctx.gpr,
            rip: ctx.rip,
            eflags: ctx.eflags,
            mxcsr: ctx.mx_csr,
            code_at_rip: CodeAtJson { address: ctx.rip, ..Default::default() },
            stack_ranges: [b, p],
            backtrace_complete: true,
            backtrace_frames: Vec::new(),
            host_fallback_frames: Vec::new(),
            guest_state: None,
            other_threads: Vec::new(),
        });
    }

    line(format!(
        "pid {} thread 0x{win_tid:x} {wname:?} (host tid {me}{})",
        std::process::id(),
        thread_label(self_pt)
    ));
    if let Some(rec) = rec {
        line(unwind::describe_exception(rec));
        if rec.code == unwind::STATUS_ACCESS_VIOLATION && rec.number_parameters >= 2 {
            let kind = match rec.information[0] {
                0 => "read from",
                1 => "write to",
                8 => "execute (DEP) at",
                _ => "access to",
            };
            line(format!("access violation: {kind} 0x{:x}", rec.information[1]));
        }
    }
    registers(ctx);
    let code_json = code_at(ctx.rip);
    with_json(|j| j.code_at_rip = code_json);

    let guest_json = guest_state(&[b, p]);
    with_json(|j| j.guest_state = guest_json);

    line(format!("backtrace (faulting thread, stack 0x{:x}..0x{:x}, host 0x{:x}..0x{:x}):", b.0, b.1, p.0, p.1));
    let (frames, complete) = walk_in_detailed(ctx, &[b, p], 64);
    let frames_json = to_frame_json(&frames);
    for f in &frames_json {
        line(format!("  #{:<2} {} [{}]", f.index, f.symbol, f.unwound_by));
    }
    with_json(|j| {
        j.backtrace_complete = complete;
        j.backtrace_frames = frames_json;
    });
    if !complete && frames.len() <= 1 {
        stack_scan(ctx.gpr[RSP]);
    }
    if frames.len() < 8 {
        // Short walk (frame chain broken or on an unexpected stack): libSystem's unwinder
        // from here, through the signal frame. Starts with the reporter's own frames.
        line("host backtrace (libSystem unwinder; the first frames are the crash reporter's):");
        let hb = host_backtrace(64);
        let hb_json: Vec<FrameJson> = hb
            .iter()
            .enumerate()
            .map(|(index, &pc)| FrameJson {
                index,
                address: pc,
                symbol: sym(pc),
                in_pe: loader_module(pc),
                unwound_by: "libsystem",
            })
            .collect();
        for f in &hb_json {
            line(format!("  #{:<2} {}", f.index, f.symbol));
        }
        with_json(|j| j.host_fallback_frames = hb_json);
    }
    let others = other_threads();
    with_json(|j| {
        j.other_threads = others;
        j.elapsed_ms = t_start.elapsed().as_millis() as u64;
    });
    line(format!("==================== end of crash report ({:?}) ====================", t_start.elapsed()));
    flush_file();
    // Flush traces etc. now: the process may die without reaching ExitProcess.
    crate::process::call_exit_hooks();
    REPORTER.store(0, Ordering::Release);
}

unsafe fn registers(ctx: &Context) {
    line("registers:");
    for row in [[0usize, 3, 1, 2], [6, 7, 5, 4], [8, 9, 10, 11], [12, 13, 14, 15]] {
        let mut s = String::from(" ");
        for i in row {
            let _ = write!(s, " {:>3}={:016x}", REG_NAMES[i], ctx.gpr[i]);
        }
        line(s);
    }
    line(format!("  rip={:016x} eflags={:08x} mxcsr={:08x}", ctx.rip, ctx.eflags, ctx.mx_csr));
    for (i, n) in REG_NAMES.iter().enumerate() {
        let v = ctx.gpr[i];
        if i == RSP || v < 0x10000 {
            continue;
        }
        let s = sym(v);
        if s != format!("0x{v:x}") {
            line(format!("  {n} -> {s}"));
        }
    }
}

/// Guest x86-64 registers and `.pdata`-unwound backtrace of the innermost interpreter
/// frame on this thread, if any.
unsafe fn guest_state(ranges: &[(u64, u64)]) -> Option<GuestStateJson> {
    let f = GUEST_STATE.get()?;
    let (gpr, rip) = f()?;
    let rip_sym = sym(rip);
    line(format!("guest interpreter state (innermost emu frame; rip = after current instruction): rip={rip:#x} {rip_sym}"));
    for row in [[0usize, 3, 1, 2], [6, 7, 5, 4], [8, 9, 10, 11], [12, 13, 14, 15]] {
        let mut s = String::from(" ");
        for i in row {
            let _ = write!(s, " {:>3}={:016x}", REG_NAMES[i], gpr[i]);
        }
        line(s);
    }
    for (i, n) in REG_NAMES.iter().enumerate() {
        let v = gpr[i];
        if i == RSP || v < 0x10000 {
            continue;
        }
        let s = sym(v);
        if s != format!("0x{v:x}") {
            line(format!("  guest {n} -> {s}"));
        }
    }
    let _ = code_at(rip);
    if !loader_module(rip) {
        if let Some(caller) = read_u64(gpr[RSP]) {
            if loader_module(caller) {
                line(format!("guest caller @[rsp] = {caller:#x} {}:", sym(caller)));
                let _ = code_at(caller.saturating_sub(0x18));
            }
        }
    }
    let walk = walk_guest_state(Some((gpr, rip)), ranges, 64);
    let frames_json = to_frame_json(&walk.frames);
    line(format!(
        "guest backtrace ({}, {} frame(s){}):",
        walk.method,
        frames_json.len(),
        if walk.complete { "" } else { ", incomplete" }
    ));
    for frame in &frames_json {
        line(format!("  #{:<2} {} [{}]", frame.index, frame.symbol, frame.unwound_by));
    }
    if !walk.complete && walk.frames.len() <= 1 {
        line("guest stack scan fallback (unwinder stopped early):");
        stack_scan(gpr[RSP]);
    }
    if std::env::var_os("PORT_CRASH_DISASM").is_some_and(|v| v == "1") {
        disasm_guest_frames(&walk.frames, rip);
    }
    Some(GuestStateJson {
        rip,
        symbol: rip_sym,
        gpr,
        unwind_method: walk.method,
        unwind_complete: walk.complete,
        frames: frames_json,
    })
}

unsafe fn disasm_function_around(pc: u64, label: &str) {
    let Some(l) = crate::try_loader() else { return };
    let (start, end) = match l.lookup_function_entry(pc) {
        Some((base, rf)) if !rf.is_null() => {
            let b = base + (*rf).begin_rva as u64;
            let e = base + (*rf).end_rva as u64;
            if e.saturating_sub(b) <= 0x800 {
                (b, e)
            } else {
                (pc.saturating_sub(0x180).max(b), pc.saturating_add(0x80).min(e))
            }
        }
        _ => (pc.saturating_sub(0x80), pc.saturating_add(0x40)),
    };
    let len = (end.saturating_sub(start) as usize).clamp(16, 0x800);
    let mut buf = vec![0u8; len];
    if !safe_read(start, &mut buf) {
        return;
    }
    let insns = l.disassemble(start, &buf, 300);
    if insns.is_empty() {
        return;
    }
    line(format!("--- disasm {label} (pc={pc:#x} {}, fn={start:#x}..{end:#x}) ---", sym(pc)));
    for s in &insns {
        line(format!("  {s}"));
    }
}

unsafe fn disasm_guest_frames(frames: &[WalkFrame], rip: u64) {
    let mut seen = Vec::<u64>::new();
    let mut dump_once = |pc: u64, label: &str| {
        if !loader_module(pc) {
            return;
        }
        let key = unwind::lookup_function_entry_no_callbacks(pc)
            .or_else(|| unwind::lookup_function_entry_no_callbacks(pc.saturating_sub(1)))
            .map(|(b, rf)| b + rf.begin_rva as u64)
            .unwrap_or(pc & !0xF);
        if seen.contains(&key) {
            return;
        }
        seen.push(key);
        disasm_function_around(pc, label);
    };
    dump_once(rip, "rip");
    for (idx, frame) in frames.iter().take(8).enumerate() {
        dump_once(frame.pc, &format!("frame#{idx}"));
    }
    if let Ok(extra) = std::env::var("PORT_CRASH_DISASM_ADDRS") {
        for tok in extra.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let hex = tok.strip_prefix("0x").or_else(|| tok.strip_prefix("0X")).unwrap_or(tok);
            if let Ok(va) = u64::from_str_radix(hex, 16) {
                disasm_function_around(va, tok);
            }
        }
    }
}

unsafe fn code_at(rip: u64) -> CodeAtJson {
    let mut out = CodeAtJson { address: rip, readable: false, bytes_around: None, disassembly: Vec::new() };
    let mut around = [0u8; 32];
    if safe_read(rip.wrapping_sub(16), &mut around) {
        let hex: Vec<String> =
            around.iter().enumerate().map(|(i, b)| if i == 16 { format!("[{b:02x}]") } else { format!("{b:02x}") }).collect();
        let joined = hex.join(" ");
        line(format!("code @rip-16: {joined}"));
        out.bytes_around = Some(joined);
    }
    let mut code = [0u8; 48];
    let n = (0..=3).map(|k| 48 - k * 16).find(|&n| safe_read(rip, &mut code[..n])).unwrap_or(0);
    if n == 0 {
        line(format!("code @rip: 0x{rip:x} is not readable"));
        return out;
    }
    out.readable = true;
    if let Some(l) = crate::try_loader() {
        let lines = l.disassemble(rip, &code[..n], 6);
        if !lines.is_empty() {
            line("disassembly @rip:");
            for (i, s) in lines.iter().enumerate() {
                line(format!("  {} {s}", if i == 0 { "=>" } else { "  " }));
            }
            out.disassembly = lines;
        }
    }
    out
}

/// Return addresses into PE images among the top 4 KiB of the stack (fallback
/// when `.pdata` unwinding cannot step past a corrupt frame).
unsafe fn stack_scan(rsp: u64) {
    let mut first = true;
    let mut shown = 0;
    for i in 0..512u64 {
        let Some(v) = read_u64(rsp + i * 8) else { break };
        if !loader_module(v) {
            continue;
        }
        if first {
            line(format!("stack scan (image addresses in [rsp, rsp+0x1000), rsp=0x{rsp:x}):"));
            first = false;
        }
        line(format!("  [rsp+0x{:03x}] {}", i * 8, sym(v)));
        shown += 1;
        if shown >= 16 {
            break;
        }
    }
}

unsafe fn other_threads() -> Vec<OtherThreadJson> {
    let me = libc::pthread_self() as usize;
    let mut targets: Vec<(u32, String, String, usize)> = Vec::new();
    for teb in crate::thread::all_tebs() {
        let tid: usize = crate::teb::field(teb, crate::teb::off::CLIENT_ID_THREAD);
        let Some(obj) = crate::thread::find_by_tid(tid as u32) else { continue };
        let pt = obj.pthread.load(Ordering::Relaxed);
        if pt == 0 || pt == me {
            continue;
        }
        let name = obj.name.try_lock().map(|n| n.clone()).unwrap_or_default();
        targets.push((tid as u32, name.clone(), format!("thread 0x{tid:x} {name:?}"), pt));
    }
    let main = MAIN_PTHREAD.load(Ordering::Relaxed);
    if main != 0 && main != me && !targets.iter().any(|t| t.3 == main) {
        targets.push((0, String::new(), "main thread (AppKit)".into(), main));
    }
    if targets.is_empty() {
        return Vec::new();
    }
    line(format!("other threads ({}):", targets.len()));
    let mut out = Vec::with_capacity(targets.len());
    for (win_tid, name, label, pt) in targets {
        let pt_name = pthread_name_raw(pt);
        line(format!("--- {label}{}", thread_label(pt)));
        match snapshot_walk_detailed(pt, 32) {
            Ok(snap) => {
                let guest_frames = to_frame_json(&snap.guest.frames);
                let host_frames = to_frame_json(&snap.host);
                let mut idx = 0usize;
                for f in &guest_frames {
                    line(format!("  #{idx:<2} {} [{}]", f.symbol, f.unwound_by));
                    idx += 1;
                }
                for f in &host_frames {
                    line(format!("  #{idx:<2} {} [{}]", f.symbol, f.unwound_by));
                    idx += 1;
                }
                out.push(OtherThreadJson {
                    win_tid,
                    name,
                    pthread_name: pt_name,
                    label,
                    error: None,
                    guest_unwind_method: snap.guest.method,
                    guest_unwind_complete: snap.guest.complete,
                    guest_frames,
                    host_frames,
                });
            }
            Err(e) => {
                line(format!("  ({e})"));
                out.push(OtherThreadJson {
                    win_tid,
                    name,
                    pthread_name: pt_name,
                    label,
                    error: Some(e.to_string()),
                    guest_unwind_method: "none",
                    guest_unwind_complete: false,
                    guest_frames: Vec::new(),
                    host_frames: Vec::new(),
                });
            }
        }
    }
    out
}

extern "C" {
    fn backtrace(buf: *mut *mut libc::c_void, size: libc::c_int) -> libc::c_int;
}

/// Return addresses of the calling thread from libSystem's `backtrace` (unwind info, crosses
/// signal frames).
unsafe fn host_backtrace(max: usize) -> Vec<u64> {
    let mut buf = vec![std::ptr::null_mut::<libc::c_void>(); max];
    let n = backtrace(buf.as_mut_ptr(), max as libc::c_int).max(0) as usize;
    buf[..n.min(max)].iter().map(|&p| p as u64).collect()
}

/// Capture the caller's context and report (CRT abort paths etc.).
#[inline(never)]
pub unsafe fn report_here(title: &str) {
    let mut ctx = Context::zeroed();
    unwind::RtlCaptureContext(&mut ctx);
    report(title, None, &ctx);
}

/// Stack range containing `rsp`: a Windows thread's stack if one matches.
fn bounds_for_rsp(rsp: u64) -> (u64, u64) {
    for teb in crate::thread::all_tebs() {
        unsafe {
            let hi: usize = crate::teb::field(teb, crate::teb::off::STACK_BASE);
            let lo: usize = crate::teb::field(teb, crate::teb::off::DEALLOCATION_STACK);
            if (lo as u64) <= rsp && rsp < hi as u64 {
                return (lo as u64, hi as u64);
            }
        }
    }
    (rsp, rsp.saturating_add(64 << 20))
}

/// Debugger entry (`aw2report` in tools/debug/lldb_aw2.py): print the mixed
/// guest/host backtrace of a stopped thread given its registers. Output goes
/// to the log (process stderr). Returns the number of frames.
#[no_mangle]
#[inline(never)]
pub extern "C" fn aw2_debug_backtrace(rip: u64, rsp: u64, rbp: u64) -> usize {
    let mut ctx = Context::zeroed();
    ctx.rip = rip;
    ctx.gpr[RSP] = rsp;
    ctx.gpr[RBP] = rbp;
    let frames = unsafe { walk(&ctx, bounds_for_rsp(rsp), 96) };
    log::error!(target: "win32_rt::crash", "backtrace (debugger) rip=0x{rip:x} rsp=0x{rsp:x} rbp=0x{rbp:x}:");
    for (i, pc) in frames.iter().enumerate() {
        log::error!(target: "win32_rt::crash", "  #{i:<2} {}", sym(*pc));
    }
    frames.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_read_reads_and_rejects() {
        let v: u64 = 0x1122_3344_5566_7788;
        assert_eq!(read_u64(&v as *const u64 as u64), Some(v));
        assert_eq!(read_u64(0), None);
        assert_eq!(read_u64(8), None);
        // An unmapped page in the middle of an otherwise valid region.
        unsafe {
            let p = libc::mmap(std::ptr::null_mut(), 0x2000, libc::PROT_NONE, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
            assert_ne!(p, libc::MAP_FAILED);
            assert_eq!(read_u64(p as u64), None);
            libc::munmap(p, 0x2000);
        }
    }

    #[inline(never)]
    fn depth(n: u32, out: &mut Vec<u64>) {
        if n == 0 {
            unsafe {
                let mut ctx = Context::zeroed();
                unwind::RtlCaptureContext(&mut ctx);
                *out = walk(&ctx, current_stack_bounds(), 64);
            }
        } else {
            depth(n - 1, out);
            std::hint::black_box(&out);
        }
    }

    #[test]
    fn walks_host_frame_pointer_chain() {
        let mut frames = Vec::new();
        depth(5, &mut frames);
        // RtlCaptureContext's caller + 5 recursive frames + the test harness.
        assert!(frames.len() >= 7, "only {} frames: {frames:x?}", frames.len());
        let mut w = frames.clone();
        w.dedup();
        assert!(w.len() < frames.len(), "recursion should repeat a return address: {frames:x?}");
    }

    #[test]
    fn snapshots_another_thread() {
        install();
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        let (ptx, prx) = std::sync::mpsc::channel::<usize>();
        let h = std::thread::spawn(move || {
            ptx.send(unsafe { libc::pthread_self() } as usize).unwrap();
            let _ = rx.recv();
        });
        let pt = prx.recv().unwrap();
        std::thread::sleep(Duration::from_millis(20));
        let frames = unsafe { snapshot_walk(pt, 32) }.expect("snapshot");
        tx.send(()).unwrap();
        h.join().unwrap();
        // Blocked in the kernel, then several host frames up to thread start.
        assert!(frames.len() >= 3, "{frames:x?}");
    }

    #[test]
    fn context_from_mcontext_maps_registers() {
        let mut mc = vec![0u8; 16 + 168 + 8 + 512];
        for i in 0..18u64 {
            mc[16 + i as usize * 8..][..8].copy_from_slice(&(0x100 + i).to_le_bytes());
        }
        let ctx = unsafe { context_from_mcontext(mc.as_ptr()) };
        assert_eq!(ctx.gpr[unwind::RAX], 0x100);
        assert_eq!(ctx.gpr[unwind::RBX], 0x101);
        assert_eq!(ctx.gpr[unwind::RCX], 0x102);
        assert_eq!(ctx.gpr[unwind::RDX], 0x103);
        assert_eq!(ctx.gpr[unwind::RDI], 0x104);
        assert_eq!(ctx.gpr[unwind::RSI], 0x105);
        assert_eq!(ctx.gpr[RBP], 0x106);
        assert_eq!(ctx.gpr[RSP], 0x107);
        assert_eq!(ctx.gpr[15], 0x10f);
        assert_eq!(ctx.rip, 0x110);
        assert_eq!(ctx.eflags, 0x111);
    }

    #[test]
    fn walk_guest_state_unwinds_multi_frame_pdata_and_ignores_stale_stack_entries() {
        // Build a synthetic image buffer containing 3 functions with x64 .pdata + UNWIND_INFO:
        // - fn0 (inner):  push rbx; push rbp; sub rsp, 0x28  (frame = 0x28 + 16 = 0x38, plus ret)
        // - fn1 (middle): push rbp; mov rbp, rsp; sub rsp, 0x100  (UWOP_SET_FPREG + UWOP_ALLOC_LARGE + UWOP_PUSH_NONVOL rbp)
        // - fn2 (outer):  sub rsp, 0x28  (returns to EMU_RET_SENTINEL)
        let mut img = vec![0x90u8; 0x2000];
        // Plant `call rel32` (0xE8 00 00 00 00) right before each return site so follows_call() is true.
        // fn0: 0x0100..0x0140, pc = 0x0120 (after prolog of 0x10 bytes)
        img[0x011b] = 0xE8;
        // fn1: 0x0200..0x0260, ret_pc = 0x0230 (after prolog of 0x10 bytes)
        img[0x022b] = 0xE8;
        // fn2: 0x0300..0x0360, ret_pc = 0x0330 (after prolog of 0x10 bytes)
        img[0x032b] = 0xE8;
        // Stale fake return address inside fn2 (0x0350) preceded by a `call rel32`:
        // a heuristic stack scanner would falsely report this!
        img[0x034b] = 0xE8;

        // UNWIND_INFO for fn0 at 0x0800:
        // version=1, flags=0, prolog=0x10, count=3, frame_reg=0
        // codes (reverse prolog order):
        //   off=0x10, UWOP_ALLOC_SMALL (2), info=4 => (4+1)*8 = 0x28
        //   off=0x08, UWOP_PUSH_NONVOL (0), info=5 (rbp)
        //   off=0x04, UWOP_PUSH_NONVOL (0), info=3 (rbx)
        img[0x0800..0x080a].copy_from_slice(&[
            1, 0x10, 3, 0,
            0x10, (4 << 4) | 2,
            0x08, (5 << 4) | 0,
            0x04, (3 << 4) | 0,
        ]);

        // UNWIND_INFO for fn1 at 0x0840:
        // version=1, flags=0, prolog=0x10, count=4, frame_reg=5 (rbp), frame_off=0
        // codes:
        //   off=0x10, UWOP_ALLOC_LARGE (1), info=0, slot16=0x20 (0x20 * 8 = 0x100)
        //   off=0x08, UWOP_SET_FPREG (3), info=0 (restores rsp = rbp)
        //   off=0x04, UWOP_PUSH_NONVOL (0), info=5 (rbp)
        img[0x0840..0x084c].copy_from_slice(&[
            1, 0x10, 4, 5,
            0x10, (0 << 4) | 1,
            0x20, 0x00,
            0x08, (0 << 4) | 3,
            0x04, (5 << 4) | 0,
        ]);

        // UNWIND_INFO for fn2 at 0x0880:
        // version=1, flags=0, prolog=0x10, count=1, frame_reg=0
        // codes:
        //   off=0x10, UWOP_ALLOC_SMALL (2), info=4 => 0x28
        img[0x0880..0x0886].copy_from_slice(&[
            1, 0x10, 1, 0,
            0x10, (4 << 4) | 2,
        ]);

        let base = img.as_ptr() as u64;
        let mut table = [
            pe_image::RuntimeFunction { begin_rva: 0x0100, end_rva: 0x0140, unwind_rva: 0x0800 },
            pe_image::RuntimeFunction { begin_rva: 0x0200, end_rva: 0x0260, unwind_rva: 0x0840 },
            pe_image::RuntimeFunction { begin_rva: 0x0300, end_rva: 0x0360, unwind_rva: 0x0880 },
        ];
        unsafe {
            assert_eq!(unwind::RtlAddFunctionTable(table.as_mut_ptr(), table.len() as u32, base), 1);
        }

        // Build the exact guest stack layout (64-bit slots, low address = stack top):
        // Index 0..5   : fn0 local 0x28 bytes (5 qwords)
        // Index 5      : fn0 saved rbp (points to fn1's rbp slot = stack_base + 72*8)
        // Index 6      : fn0 saved rbx (0xBB)
        // Index 7      : fn0 return address -> fn1 (base + 0x0230)
        // Index 8..40  : fn1 local 0x100 bytes (32 qwords) -- plant stale return addr at index 12!
        // Index 40     : fn1 saved rbp (0x5555) -- fn1's rbp points here!
        // Index 41     : fn1 return address -> fn2 (base + 0x0330)
        // Index 42..47 : fn2 local 0x28 bytes (5 qwords)
        // Index 47     : fn2 return address -> EMU_RET_SENTINEL
        let mut stack = vec![0u64; 64];
        let stack_base = stack.as_ptr() as u64;
        let stack_end = stack_base + (stack.len() as u64) * 8;

        let fn1_rbp = stack_base + 40 * 8;
        let stale_ret = base + 0x0350;
        stack[5] = fn1_rbp;
        stack[6] = 0xBB;
        stack[7] = base + 0x0230;
        stack[12] = stale_ret; // stale return address inside fn1's local frame
        stack[40] = 0x5555;
        stack[41] = base + 0x0330;
        stack[47] = EMU_RET_SENTINEL;

        let mut gpr = [0u64; 16];
        gpr[RSP] = stack_base;
        gpr[RBP] = 0x1111; // fn0 does not use rbp as frame reg; saved fn1_rbp is popped on unwind!

        let walk = walk_guest_state(Some((gpr, base + 0x0120)), &[(stack_base, stack_end)], 32);
        unsafe {
            assert_eq!(unwind::RtlDeleteFunctionTable(table.as_mut_ptr()), 1);
        }

        let pcs: Vec<u64> = walk.frames.iter().map(|f| f.pc).collect();
        assert_eq!(walk.method, "pdata", "walk={walk:?}");
        assert!(walk.complete, "expected clean termination at EMU_RET_SENTINEL: {walk:?}");
        assert_eq!(
            pcs,
            vec![base + 0x0120, base + 0x0230, base + 0x0330],
            "expected exact .pdata-unwound chain without stale frame {stale_ret:#x}"
        );
        assert!(walk.frames.iter().all(|f| f.unwound_by == "pdata" && f.in_pe));
        assert!(!pcs.contains(&stale_ret), "stale return address leaked into unwound frames");
    }

    #[test]
    fn walk_guest_state_handles_corrupt_stack_without_faulting() {
        let mut img = vec![0x90u8; 0x1000];
        img[0x0800..0x0806].copy_from_slice(&[1, 0x08, 1, 0, 0x08, (4 << 4) | 2]);
        let base = img.as_ptr() as u64;
        let mut table = [pe_image::RuntimeFunction {
            begin_rva: 0x0100,
            end_rva: 0x0140,
            unwind_rva: 0x0800,
        }];
        unsafe {
            assert_eq!(unwind::RtlAddFunctionTable(table.as_mut_ptr(), 1, base), 1);
        }
        // Unmapped PROT_NONE page as guest stack pointer.
        let unmapped = unsafe {
            let p = libc::mmap(
                std::ptr::null_mut(),
                0x4000,
                libc::PROT_NONE,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1,
                0,
            );
            assert_ne!(p, libc::MAP_FAILED);
            p as u64
        };
        let mut gpr = [0u64; 16];
        gpr[RSP] = unmapped;
        let walk = walk_guest_state(Some((gpr, base + 0x0120)), &[(unmapped, unmapped + 0x4000)], 16);
        unsafe {
            libc::munmap(unmapped as *mut libc::c_void, 0x4000);
            assert_eq!(unwind::RtlDeleteFunctionTable(table.as_mut_ptr()), 1);
        }
        assert_eq!(walk.frames.len(), 1);
        assert_eq!(walk.frames[0].pc, base + 0x0120);
        assert!(!walk.complete);
    }

    #[test]
    fn report_writes_json_sidecar_beside_text_file() {
        let dir = std::env::temp_dir().join(format!("win32-rt-crash-json-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let prev_port = std::env::var_os("PORT_CRASH_DIR");
        let prev_aw2 = std::env::var_os("AW2_CRASH_DIR");
        std::env::set_var("PORT_CRASH_DIR", &dir);
        std::env::set_var("AW2_CRASH_DIR", &dir);

        let mut rec = unsafe { std::mem::zeroed::<ExceptionRecord>() };
        rec.code = unwind::STATUS_ACCESS_VIOLATION;
        rec.address = 0x14000_1234usize as *mut _;
        rec.number_parameters = 2;
        rec.information[0] = 0; // read
        rec.information[1] = 0xDEAD_BEEF;

        let mut ctx = Context::zeroed();
        unsafe { unwind::RtlCaptureContext(&mut ctx) };
        unsafe { report("unit test access violation", Some(&rec), &ctx) };

        match prev_port {
            Some(v) => std::env::set_var("PORT_CRASH_DIR", v),
            None => std::env::remove_var("PORT_CRASH_DIR"),
        }
        match prev_aw2 {
            Some(v) => std::env::set_var("AW2_CRASH_DIR", v),
            None => std::env::remove_var("AW2_CRASH_DIR"),
        }

        let entries: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .map(|e| e.path())
            .collect();
        let txt_path = entries.iter().find(|p| p.extension().is_some_and(|e| e == "txt")).cloned();
        let json_path = entries.iter().find(|p| p.extension().is_some_and(|e| e == "json")).cloned();
        assert!(txt_path.is_some(), "expected .txt crash file in {entries:?}");
        assert!(json_path.is_some(), "expected .json crash sidecar in {entries:?}");
        let txt_path = txt_path.unwrap();
        let json_path = json_path.unwrap();
        assert_eq!(
            txt_path.file_stem(),
            json_path.file_stem(),
            "expected matching filename stem for text and JSON crash reports"
        );
        let txt = std::fs::read_to_string(&txt_path).unwrap();
        let js = std::fs::read_to_string(&json_path).unwrap();
        let _ = std::fs::remove_dir_all(&dir);

        assert!(txt.contains("AW2 CRASH REPORT: unit test access violation"), "{txt}");
        assert!(js.contains("\"schema_version\": 1"), "{js}");
        assert!(js.contains("\"kind\": \"crash\""), "{js}");
        assert!(js.contains("\"title\": \"unit test access violation\""), "{js}");
        assert!(js.contains("\"code\": \"0xc0000005\""), "{js}");
        assert!(js.contains("\"operation\": \"read\""), "{js}");
        assert!(js.contains("\"address\": \"0xdeadbeef\""), "{js}");
        assert!(js.contains("\"watchdog\":"), "{js}");
        assert!(js.contains("\"backtrace\":"), "{js}");
        assert!(js.trim_start().starts_with('{') && js.trim_end().ends_with('}'), "{js}");
    }
}
