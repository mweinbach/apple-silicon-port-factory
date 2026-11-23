//! Clean per-thread arena for lifted guest stack frames ([`LiftedFrame`]).
//!
//! Every CFG-lifted function used to declare a `LiftedStack<256>` local, zeroed on
//! entry (a 2 KB `bzero` per call). Lifted code *does* read guest stack slots it never
//! wrote, and those reads must return 0: leaving the buffer uninitialized, or zeroing
//! only the `frame/8 + 64` top window, broke gameplay lighting, because the slots below
//! the window then held host-stack garbage (see `simd_ops::LiftedStack` and
//! `target/review/exec/lifted_stack.md`).
//!
//! [`LiftedFrame<N, F>`] gives the same "every unwritten slot reads 0" contract without the
//! per-call memset. `F` is the function's static frame in qwords (generator formula, written
//! into the shards by `tools/decomp_scale/lstack_rewrite.py`).
//! - Each thread gets one arena (`ARENA_BYTES`, `VirtualAlloc`-committed so guest
//!   `VirtualQuery` sees it as committed RW memory).
//! - Frames are carved LIFO from its top. The invariant: **everything below the arena top is
//!   zero.**
//! - On entry a frame takes `WIN` bytes: the argument qwords (`A`, 24 unless the function
//!   reads arg24+), the static frame and
//!   `LSTACK_SLACK` qwords, rounded up to 64-byte lines. Taking them needs no zeroing, since
//!   the memory is already clean.
//! - On exit (`Drop`) the frame zeroes exactly those `WIN` bytes again (`dc zva`, no call)
//!   and pops.
//! - Reads below the window land in callee territory, which is clean because callees clean
//!   up after themselves. They therefore still see 0, as they did with the zeroed 2 KB local.
//! - The contract holds as long as every write a function makes stays inside its own window.
//!   `AW2_LSTACK_CHECK=1` verifies this in gameplay. At each `Drop` it scans the rest of the
//!   old `N`-qword slot below the static frame. For each function it logs the deepest write
//!   below the static frame, with the function's symbol, whenever that depth grows. It
//!   flags `ESCAPES` when the write lies beyond the zeroed window. In check mode each
//!   frame also zeroes its whole slot at exit, so an escape can't leak into later reads.
//! - `AW2_LSTACK_ARENA=0` falls back to a zeroed local slot, i.e. the old behaviour, now
//!   zeroed with `dc zva` instead of `bzero`.
//!
//! Remaining differences from the zeroed local (all in undefined territory):
//! - Reads above the 24 argument qwords hit the caller's frame instead of host stack.
//! - A dead callee frame reads 0 right after the callee returns. Before, it held stale data
//!   until the next lifted call zeroed over it.
//!
//! Non-LIFO safety:
//! - Fibers are stubs (no stack switching).
//! - Exceptions that cross Rust frames are fatal on aarch64.
//! - If a frame's `Drop` was skipped anyway, the next outer `Drop` notices that the arena top
//!   moved and zeroes the abandoned range.
//!
//! TEB stack bounds (`StackBase`/`StackLimit`) still describe the host thread stack.
//! Consumers checked:
//! - The SEH dispatch/unwind range checks (`win32_rt::unwind`) only see establisher frames
//!   of PE code, and lifted frames never are one.
//! - `crash.rs` uses the bounds only for reports.
//! - Guest `VirtualQuery` reports the arena as committed private RW memory (allocation base
//!   = arena) instead of "thread stack".
//! - The N=1 big-frame arena (`simd_ops::alloc_big_lifted_frame`) has always lived off the
//!   host stack.
//!
//! Stack account (target/review/exec/g18/lstack_recursion_plan.md). Lifted frames stay in the
//! arena, and the lifted depth is charged to the thread's emulator stack, as native frames
//! would be:
//! - Anchor. [`anchor_enter`] / [`anchor_leave`] run around every guest -> host call
//!   (`emu::host_call` / `host_tail_call`, the JIT's `jit_host_xfer` included). They record the arena top
//!   in `CallCtx::ls_anchor` (0 = none: the arena's initial top). With them, `ls_lo` is set
//!   to the budget: the lifted bytes that fit between the guest position of the call and the
//!   guest floor (`emu::GUEST_FLOOR_MARGIN` above the emulator's `stack_min`). Both are
//!   restored after the call and by EH landings (`eh_rt::anchor::Snapshot`).
//! - Guest-visible position. A nested emulator region starts [`lifted_depth`] bytes below the
//!   guest position, so guest `rsp` (V8's stack checks, `GetCurrentStackPosition`) sees the
//!   lifted frames between. If that start would lie below the floor, the region raises
//!   `STATUS_STACK_OVERFLOW` instead of running.
//! - Guard. A frame entry beyond the budget raises `STATUS_STACK_OVERFLOW`
//!   (0xC00000FD, noncontinuable), as a guard-page hit would natively. The frame-entry fast path stays one compare: the budget
//!   is in `ls_lo`, and only `arena_slow` tells budget from arena exhaustion.
//! - Host stack. Each lifted frame also takes its Rust frame (with `LiftedFrame::buf`) on the
//!   host stack, about 8x its arena window, so the host stack runs out long before the guest
//!   budget. Frame entry therefore also compares the host `sp` with `CallCtx::ls_hlo` (the host
//!   thread stack's low end + [`HOST_MARGIN`]) and raises the same exception there, instead of
//!   a fatal SIGSEGV at the host guard page.
//! - The guest budget is not enforced in check mode (`AW2_LSTACK_CHECK=1`: `ls_lo` holds the
//!   sentinel) or on frames on the local fallback slot (arena off / exhausted); the host check is.
//! - Known conservative difference: each lifted frame is charged its whole window, the
//!   24 argument qwords included, while a native frame shares its caller's outgoing area. Deep lifted
//!   recursion therefore reaches the guest limit at roughly 70% of native depth.
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::callctx::{ctx, CallCtx};
use crate::simd_ops::StackBuf;

/// Qwords below the static frame that a frame's window still covers.
/// Was 64 (the generator's `need_words = frame/8 + 64` margin). ls2's gameplay CHECK run logged 0 writes below any
/// static frame. Statically, every rsp decrease in an arena fn is an immediate sub or push
/// counted in F, and the `sub rsp, reg` (chkstk) fns use the big `LiftedStack<1>` frame.
/// 8 keeps one line of margin.
pub const LSTACK_SLACK: usize = 8;
/// Default argument qwords above the entry `rsp`: return slot, home space, args 5..23
/// (`LiftedFrame`'s defaulted `A`).
const ARGS: usize = 24;
/// Per-thread arena size. Host stacks are >= 8 MiB and every lifted frame also takes its
/// `N`-qword local there, so this outlasts the host stack. Exhaustion falls back to the local.
pub const ARENA_BYTES: usize = 16 << 20;
/// Mapped-but-unused bytes kept below the deepest frame's slot.
const ARENA_MARGIN: usize = 64 << 10;
/// Zero, never-allocated bytes above the first frame. Callees read a little above a frame's
/// 24 argument qwords: `tail_stk` hands out `rsp + 0xc8` as the readable limit, which is
/// one qword above the frame top when `rsp` is the entry `rsp`, and `emu_run_full` copies
/// stack args up to that limit. Lifted code may also read incoming args beyond the 23rd.
/// On the host stack that was always mapped; in the arena it must be, too.
const ARENA_HEADROOM: usize = 64 << 10;

/// `CallCtx::ls_flags`: check mode (scan at `Drop`).
pub const LS_CHECK: u32 = 1;
/// `CallCtx::ls_flags`: `dc zva` unusable (block size isn't 64 bytes); zero with stores.
pub const LS_NO_ZVA: u32 = 2;

/// Check mode: `CallCtx::ls_lo` sentinel. `frame_base`'s fast test (`top < ls_lo + REACH`)
/// then always fails, so every frame entry goes through `arena_slow`, which plants
/// [`CANARY`]s above the new frame. The real arena low end is `ls_base + ARENA_MARGIN`
/// ([`arena_lo`]). No cost outside check mode: the fast path is unchanged.
const CHECK_LO: usize = usize::MAX / 2;
/// Check mode: the qwords `[top, top + 64)` above a new frame (the caller's `LSTACK_SLACK`,
/// never written: ls2's CHECK run logged 0 writes below any static frame) hold
/// `CANARY | i`, where `i` is the qword index above the frame's argument window: a read of
/// stack arg slot `A + i` returns it. Used as a pointer it faults non-canonically and the
/// crash handler names it ([`describe_canary`]); as an integer it shows up as a G/N
/// divergence in the shadow oracle. Re-zeroed when the frame pops.
pub const CANARY: u64 = 0xA5A5_57AC_0000_0000;
const CANARY_QWORDS: usize = 8;

/// The arena's usable low end (`ls_lo`, or the real bound behind the check-mode sentinel).
#[inline]
pub(crate) fn arena_lo(c: &CallCtx) -> usize {
    let lo = c.ls_lo.get();
    if lo == CHECK_LO {
        c.ls_base.get() + ARENA_MARGIN
    } else {
        lo
    }
}

/// Crash-handler hint for a fault address that came from a check-mode canary (top byte may
/// be stripped by TBI; low bits may include a displacement).
pub fn describe_canary(addr: u64) -> Option<String> {
    if addr & 0x00FF_FFFF_0000_0000 != CANARY & 0x00FF_FFFF_0000_0000 {
        return None;
    }
    let i = addr & 0xFFFF_FFFF;
    Some(format!(
        "address {addr:#x} is an AW2_LSTACK_CHECK canary: a lifted function read stack argument slot A+{} \
         (A = its argument window, 24 by default) beyond its window and used it as a pointer (low bits {i:#x} \
         = slot offset + displacement). Widen that function's window (LiftedFrame<N, F, A>) and its callers' \
         outgoing limit (call_stk_w::<LIM>).",
        i & 0x7
    ))
}

/// Check mode: plant the canaries above a new frame whose arena top is `top`.
#[cold]
unsafe fn place_canary(top: usize) {
    for i in 0..CANARY_QWORDS {
        ((top + i * 8) as *mut u64).write_volatile(CANARY | i as u64);
    }
}

const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_RELEASE: u32 = 0x8000;
const PAGE_NOACCESS: u32 = 0x01;
const PAGE_READWRITE: u32 = 0x04;
/// Reserved, never-committed bytes right past the arena end (one allocation granule).
pub const ARENA_GUARD: usize = 64 << 10;

/// A lifted function's guest stack frame. Declared by generated code as
/// `let mut stack = crate::simd_ops::LiftedFrame::<N, F>::new();` followed by
/// `crate::simd_ops::init_lifted_stack(&mut stack, ..)`.
/// - `N`: the old slot size in qwords (reads below the window stay mapped down to it).
/// - `F`: the static frame size in qwords.
/// - `A`: argument qwords above the entry `rsp` ([`StackBuf::ARGS`]): 24 (return slot, home
///   space, args 5..23) unless the function reads arg24+, in which case the generator emits
///   an even `A > max slot` (`LiftedFrame::<N, F, 26>`).
#[repr(C)]
pub struct LiftedFrame<const N: usize, const F: usize, const A: usize = ARGS> {
    /// Arena top before this frame (= entry `rsp` + `A` qwords); 0 = local fallback in `buf`.
    top: usize,
    /// Arena top while this frame is live (= `top - WIN`).
    low: usize,
    /// Fallback storage (arena off or exhausted), zeroed only when used.
    buf: MaybeUninit<[u64; N]>,
}

impl<const N: usize, const F: usize, const A: usize> LiftedFrame<N, F, A> {
    /// Arena bytes owned by the frame, zeroed again at exit: the argument qwords, the static
    /// frame and the slack, rounded up to 64-byte lines.
    pub const WIN: usize = ((F + LSTACK_SLACK + A) * 8 + 63) & !63;
    /// Bytes below the arena top that must be mapped for this frame: its window, or the whole
    /// `N`-qword slot the old zeroed local provided, whichever is larger.
    const REACH: usize = if Self::WIN > N * 8 { Self::WIN } else { N * 8 };
    /// Argument bytes above the entry `rsp`.
    const ARGS_BYTES: usize = {
        assert!(N >= A, "LiftedFrame: the local fallback slot must hold the argument window");
        A * 8
    };

    #[inline(always)]
    pub fn new() -> Self {
        Self { top: 0, low: 0, buf: MaybeUninit::uninit() }
    }
}

impl<const N: usize, const F: usize, const A: usize> Default for LiftedFrame<N, F, A> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, const F: usize, const A: usize> StackBuf for LiftedFrame<N, F, A> {
    const N: usize = N;
    const ARGS: usize = A;

    #[inline(always)]
    fn base(&self) -> *const u64 {
        self.buf.as_ptr() as *const u64
    }

    #[inline(always)]
    fn base_mut(&mut self) -> *mut u64 {
        let p = self.buf.as_mut_ptr() as *mut u64;
        // As in `LiftedStack::base_mut`: the contents are unknown to LLVM, not undef.
        unsafe { core::arch::asm!("/* {0} */", in(reg) p, options(nostack, preserves_flags)) };
        p
    }

    #[inline(always)]
    unsafe fn frame_base(&mut self) -> *mut u64 {
        let c = ctx();
        let mut top = c.ls_top.get();
        let sp = host_sp();
        if (top < c.ls_lo.get() + Self::REACH) | (sp < c.ls_hlo.get()) {
            top = arena_slow(c, Self::REACH, sp);
        }
        if top != 0 {
            let low = top - Self::WIN;
            c.ls_top.set(low);
            self.top = top;
            self.low = low;
            return (top - Self::ARGS_BYTES) as *mut u64;
        }
        // Local fallback: the zeroed slot, exactly as `LiftedStack<N>`.
        let p = self.base_mut();
        zero_slot(p as *mut u8, N * 8);
        p.add(N - A)
    }

    #[inline(always)]
    fn out_limit(&self, _rsp: u64, _big: u64) -> u64 {
        if self.top != 0 {
            self.top as u64
        } else {
            unsafe { self.base().add(N) as u64 }
        }
    }
}

impl<const N: usize, const F: usize, const A: usize> Drop for LiftedFrame<N, F, A> {
    #[inline(always)]
    fn drop(&mut self) {
        let top = self.top;
        if top != 0 {
            let c = ctx();
            if (c.ls_flags.get() != 0) | (c.ls_top.get() != self.low) {
                unsafe { pop_slow(c, top, self.low, N, F, A) };
            } else {
                unsafe { zero_lines(self.low, Self::WIN) };
                c.ls_top.set(top);
            }
        }
    }
}

/// `AW2_LSTACK_ARENA` / `AW2_LSTACK_CHECK`: 0 = arena off, 1 = on, 2 = on + check.
fn mode() -> u32 {
    static MODE: OnceLock<u32> = OnceLock::new();
    *MODE.get_or_init(|| {
        let on = std::env::var("AW2_LSTACK_ARENA").map(|v| v.trim() != "0").unwrap_or(true);
        let check = std::env::var("AW2_LSTACK_CHECK").map(|v| v.trim() == "1").unwrap_or(false);
        if !on {
            log::info!("lstack: clean arena off (AW2_LSTACK_ARENA=0): lifted frames use a zeroed local slot");
            if check {
                log::warn!("lstack: AW2_LSTACK_CHECK=1 ignored (needs the arena)");
            }
            0
        } else {
            log::info!(
                "lstack: clean per-thread arena for lifted frames ({} MiB/thread, window = static frame + {} qwords){} (AW2_LSTACK_ARENA=0 disables)",
                ARENA_BYTES >> 20,
                LSTACK_SLACK,
                if check { ", CHECK mode: escape scan at every frame exit, stack-arg canaries above every frame" } else { "" }
            );
            if check { 2 } else { 1 }
        }
    })
}

/// Set up this thread's arena if it has none yet. Returns the usable top for a frame
/// needing `reach` bytes, or 0 (arena off, failed, or exhausted: use the local slot).
/// Raises `STATUS_STACK_OVERFLOW` when the host stack (`sp` = the entering frame's) is
/// below its margin, or the frame would cross the lifted budget (see "Stack account").
#[cold]
#[inline(never)]
unsafe fn arena_slow(c: &CallCtx, reach: usize, sp: usize) -> usize {
    if c.ls_lo.get() == 0 {
        setup(c);
    }
    if sp < c.ls_hlo.get() && host_stack_bounds().is_some_and(|(lo, hi)| (lo..hi).contains(&sp)) {
        raise_stack_overflow(Overflow::HostStack);
    }
    let top = c.ls_top.get();
    if top == 0 {
        return 0; // arena off or its VirtualAlloc failed
    }
    let check = c.ls_lo.get() == CHECK_LO;
    if top >= arena_lo(c) + reach {
        if check {
            place_canary(top);
        }
        return top;
    }
    if !check && top >= real_lo(c) + reach {
        // Room left in the arena: the budget is what ran out.
        raise_stack_overflow(Overflow::Budget);
    }
    static ONCE: AtomicBool = AtomicBool::new(false);
    if !ONCE.swap(true, Ordering::Relaxed) {
        log::warn!(
            "lstack: arena exhausted ({} MiB deep); deeper frames use a zeroed local slot",
            ARENA_BYTES >> 20
        );
    }
    0
}

unsafe fn setup(c: &CallCtx) -> bool {
    if let Some((lo, hi)) = host_stack_bounds() {
        c.ls_hlo.set(lo + HOST_MARGIN.min((hi - lo) / 4));
    }
    let m = mode();
    if m == 0 {
        // `ls_lo != 0` with `ls_top == 0`: later frames on this thread skip `setup`.
        c.ls_lo.set(1);
        return false;
    }
    // Reserve the arena plus `ARENA_GUARD` and commit only the arena: the reserved, inaccessible
    // tail makes a lifted frame that walks up off its frames fault exactly at `arena_end()`
    // instead of running on into whatever mapping happens to follow (g15a v8 fuzz shard:
    // the `v8+0x6c3300` overrun faulted at 0x124a80000, not at its arena end, while focus
    // runs of the same function fault at the arena end and are listed as stack_top_overrun).
    let r = win32_rt::kernel32::memory::virtual_alloc(0, ARENA_BYTES + ARENA_GUARD, MEM_RESERVE, PAGE_NOACCESS)
        .and_then(|b| win32_rt::kernel32::memory::virtual_alloc(b, ARENA_BYTES, MEM_COMMIT, PAGE_READWRITE).map(|_| b));
    match r {
        Ok(b) => {
            c.ls_base.set(b);
            let top0 = (b + ARENA_BYTES - ARENA_HEADROOM) & !63;
            c.ls_top.set(top0);
            // Check mode: the sentinel sends every frame entry through `arena_slow` (canaries).
            // Otherwise the budget with no anchor: the whole guest stack.
            c.ls_lo.set(if m == 2 { CHECK_LO } else { budget_lo(c, top0, idle_room(c)) });
            let mut flags = if m == 2 { LS_CHECK } else { 0 };
            if !zva_ok() {
                flags |= LS_NO_ZVA;
            }
            c.ls_flags.set(flags);
            true
        }
        Err(e) => {
            // Never retry on this thread: `ls_lo != 0` with `ls_top == 0` keeps the fast check
            // failing and `arena_slow` returning 0.
            c.ls_lo.set(1);
            static ONCE: AtomicBool = AtomicBool::new(false);
            if !ONCE.swap(true, Ordering::Relaxed) {
                log::warn!("lstack: arena VirtualAlloc failed ({e:#x}); using zeroed local slots");
            }
            false
        }
    }
}

/// Set this thread's arena up now (if it never was): the shadow orchestrator (so the probe
/// runs fork from the set-up state) and EH landing snapshots (`eh_rt::anchor::Snapshot`).
pub(crate) unsafe fn shadow_warm(c: &CallCtx) {
    if c.ls_lo.get() == 0 {
        setup(c);
    }
}

/// One past the last byte of the calling thread's arena (set up now if it is not yet), or 0
/// without one (`AW2_LSTACK_ARENA=0`). A lifted frame that walks up off its frames faults
/// here (the dll_lift difftest's "stack_top_overrun" class).
pub fn arena_end() -> usize {
    let c = ctx();
    unsafe { shadow_warm(c) };
    match c.ls_base.get() {
        0 => 0,
        b => b + ARENA_BYTES,
    }
}

/// Lowest address a frame of the calling thread's arena can reach (set up now if it is not
/// yet), or 0 without one. Unlike `ls_lo` this ignores the budget.
pub fn arena_floor() -> usize {
    let c = ctx();
    unsafe { shadow_warm(c) };
    match c.ls_base.get() {
        0 => 0,
        b => b + ARENA_MARGIN,
    }
}

// ---------------------------------------------------------------------------
// Stack account (module doc)
// ---------------------------------------------------------------------------

/// Host stack kept free below the deepest lifted frame (at most a quarter of the stack): the
/// exception dispatch, the handlers it runs and non-lifted callees of the last frame use it.
pub const HOST_MARGIN: usize = 1 << 20;
/// `STATUS_STACK_OVERFLOW`.
pub const STATUS_STACK_OVERFLOW: u32 = 0xC000_00FD;

/// What ran out ([`raise_stack_overflow`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    /// A lifted frame entry with the host stack below `ls_hlo`.
    HostStack,
    /// A lifted frame entry past the guest budget (`ls_lo`).
    Budget,
    /// A nested emulator region whose start, charged with the lifted depth, is below the guest floor.
    Region,
}

/// Arena real low end (no budget, no check sentinel).
#[inline]
fn real_lo(c: &CallCtx) -> usize {
    c.ls_base.get() + ARENA_MARGIN
}

/// The arena top with no frame live (the anchor of a thread outside any guest -> host call).
#[inline]
fn arena_top0(c: &CallCtx) -> usize {
    (c.ls_base.get() + ARENA_BYTES - ARENA_HEADROOM) & !63
}

/// `ls_lo` for an anchor at arena top `anchor` with `room` guest bytes left below the call.
#[inline]
fn budget_lo(c: &CallCtx, anchor: usize, room: usize) -> usize {
    real_lo(c).max(anchor.saturating_sub(room))
}

/// Guest room of a thread outside any guest -> host call: its whole emulator stack.
fn idle_room(c: &CallCtx) -> usize {
    match c.ls_gstack.get() {
        (0, _) => crate::emu::GUEST_ROOM_MIN as usize,
        (floor, top) => top.saturating_sub(floor) as usize,
    }
}

/// Emulator state creation (`emu::ThreadEmuState::new`): the guest stack's floor and top.
pub(crate) fn note_guest_stack(c: &CallCtx, floor: u64, top: u64) {
    c.ls_gstack.set((floor, top));
}

/// Guest -> host call (`emu::host_call` / `host_tail_call`) with the guest stack allocation
/// pointer `guest_sp` for nested regions: anchor the account at the current arena top and set
/// the budget to the guest room below `guest_sp`. Returns what [`anchor_leave`] restores.
/// A `guest_sp` off the emulator stack sets no budget; one below the floor leaves no room.
#[inline]
pub(crate) fn anchor_enter(c: &CallCtx, guest_sp: u64) -> (usize, usize) {
    if c.ls_lo.get() == 0 {
        anchor_warm(c);
    }
    let prev = (c.ls_anchor.get(), c.ls_lo.get());
    let top = c.ls_top.get();
    if top != 0 {
        c.ls_anchor.set(top);
        if prev.1 != CHECK_LO {
            let (floor, gtop) = c.ls_gstack.get();
            let room = if floor == 0 || guest_sp > gtop {
                usize::MAX
            } else {
                guest_sp.saturating_sub(floor) as usize
            };
            c.ls_lo.set(budget_lo(c, top, room));
        }
    }
    prev
}

#[cold]
#[inline(never)]
fn anchor_warm(c: &CallCtx) {
    unsafe { setup(c) };
}

/// End of the guest -> host call [`anchor_enter`] began.
#[inline]
pub(crate) fn anchor_leave(c: &CallCtx, prev: (usize, usize)) {
    c.ls_anchor.set(prev.0);
    c.ls_lo.set(prev.1);
}

/// Lifted arena bytes live below the innermost guest -> host transition (0 without an arena):
/// a nested emulator region starts this much below the guest position.
#[inline]
pub(crate) fn lifted_depth(c: &CallCtx) -> usize {
    let top = c.ls_top.get();
    if top == 0 {
        return 0;
    }
    let a = match c.ls_anchor.get() {
        0 => arena_top0(c),
        a => a,
    };
    a.saturating_sub(top)
}

/// The calling frame's host stack pointer.
#[inline(always)]
fn host_sp() -> usize {
    let sp: usize;
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("mov {}, sp", out(reg) sp, options(nomem, nostack, preserves_flags))
    };
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) sp, options(nomem, nostack, preserves_flags))
    };
    sp
}

/// `[low, high)` of the calling thread's host stack.
#[cfg(target_vendor = "apple")]
fn host_stack_bounds() -> Option<(usize, usize)> {
    unsafe extern "C" {
        fn pthread_self() -> usize;
        fn pthread_get_stackaddr_np(t: usize) -> *mut u8;
        fn pthread_get_stacksize_np(t: usize) -> usize;
    }
    unsafe {
        let t = pthread_self();
        let hi = pthread_get_stackaddr_np(t) as usize;
        let size = pthread_get_stacksize_np(t);
        (hi != 0 && size != 0 && size < hi).then(|| (hi - size, hi))
    }
}

#[cfg(not(target_vendor = "apple"))]
fn host_stack_bounds() -> Option<(usize, usize)> {
    None
}

/// Raise `STATUS_STACK_OVERFLOW` (noncontinuable) on the EH chain, as a guard-page hit would.
#[cold]
#[inline(never)]
pub(crate) unsafe fn raise_stack_overflow(what: Overflow) -> ! {
    static LOGGED: AtomicU64 = AtomicU64::new(0);
    if LOGGED.fetch_add(1, Ordering::Relaxed) < 16 {
        let c = ctx();
        log::warn!(
            "lstack: stack overflow ({what:?}): arena top {:#x}, budget lo {:#x}, anchor {:#x}, lifted depth {:#x}, host sp {:#x} (limit {:#x}); raising STATUS_STACK_OVERFLOW",
            c.ls_top.get(),
            c.ls_lo.get(),
            c.ls_anchor.get(),
            lifted_depth(c),
            host_sp(),
            c.ls_hlo.get()
        );
    }
    win32_rt::unwind::raise_exception_raw(
        STATUS_STACK_OVERFLOW as u64,
        win32_rt::unwind::EXCEPTION_NONCONTINUABLE as u64,
        0,
        0,
    );
    log::error!("lstack: STATUS_STACK_OVERFLOW (noncontinuable) was continued");
    std::process::abort()
}

/// Release the thread's arena (`CallCtx` drop at thread exit).
pub fn release_arena(c: &CallCtx) {
    let b = c.ls_base.replace(0);
    c.ls_top.set(0);
    c.ls_lo.set(0);
    if b != 0 {
        unsafe {
            let _ = win32_rt::kernel32::memory::virtual_free(b, 0, MEM_RELEASE);
        }
    }
}

/// `Drop` slow path: check mode, `dc zva` unusable, or the arena top isn't where this
/// frame left it (inner frames abandoned without their `Drop`).
#[cold]
#[inline(never)]
unsafe fn pop_slow(c: &CallCtx, top: usize, low: usize, n: usize, f: usize, a: usize) {
    let pc = return_address();
    let cur = c.ls_top.get();
    if cur != low {
        let base = c.ls_base.get();
        if cur < low && cur >= base {
            zero_any(c, cur, low - cur);
        }
        static ONCE: AtomicBool = AtomicBool::new(false);
        if !ONCE.swap(true, Ordering::Relaxed) {
            log::warn!(
                "lstack: arena top {cur:#x} != expected {low:#x} at exit of {} (abandoned inner frames; cleaned)",
                win32_rt::symbolize(pc)
            );
        }
    }
    let win = top - low;
    if c.ls_flags.get() & LS_CHECK != 0 {
        if let Some(depth) = check_scan(top, n, f, a) {
            record_escape(pc, depth, f, n, win, a);
        }
        let reach = win.max(n * 8);
        zero_any(c, top - reach, reach);
        // This frame's canaries (`arena_slow`), in the caller's slack above `top`.
        core::ptr::write_bytes(top as *mut u64, 0, CANARY_QWORDS);
    } else {
        zero_any(c, low, win);
    }
    c.ls_top.set(top);
}

/// Deepest non-zero byte offset below the static frame within the old `n`-qword slot
/// (`[top - n*8, entry_rsp - f*8)`, entry `rsp` = `top - args*8`), in bytes below the static
/// frame, or `None` if clean. Check-mode canaries (left above an abandoned callee) don't count.
pub unsafe fn check_scan(top: usize, n: usize, f: usize, args: usize) -> Option<usize> {
    let base = top - args * 8;
    let slot_lo = top.saturating_sub(n * 8);
    let static_lo = base.saturating_sub(f * 8);
    let mut a = slot_lo;
    while a < static_lo {
        let v = core::ptr::read_volatile(a as *const u64);
        if v != 0 && v >> 32 != CANARY >> 32 {
            return Some(static_lo - a);
        }
        a += 8;
    }
    None
}

static CHECK_HITS: AtomicU64 = AtomicU64::new(0);
static CHECK_ESCAPES: AtomicU64 = AtomicU64::new(0);

fn record_escape(pc: usize, depth: usize, f: usize, n: usize, win: usize, args: usize) {
    static SEEN: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();
    let covered = win - args * 8 - f * 8;
    let below_win = depth > covered;
    CHECK_HITS.fetch_add(1, Ordering::Relaxed);
    if below_win {
        CHECK_ESCAPES.fetch_add(1, Ordering::Relaxed);
    }
    {
        let mut m = SEEN.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap_or_else(|e| e.into_inner());
        let e = m.entry(pc).or_insert(0);
        if depth <= *e {
            return;
        }
        *e = depth;
    }
    log::warn!(
        target: "aw2_game::lstack",
        "lstack check: {} (pc {:#x}) wrote {} B below its static frame (F={} qwords, window covers {} B below it, N={}){} [hits {} escapes {}]",
        win32_rt::symbolize(pc),
        unslid(pc),
        depth,
        f,
        covered,
        n,
        if below_win { " ESCAPES the window" } else { "" },
        CHECK_HITS.load(Ordering::Relaxed),
        CHECK_ESCAPES.load(Ordering::Relaxed),
    );
}

/// `pc` as a link-time address of the main executable (for `atos -o <binary> <addr>`).
fn unslid(pc: usize) -> usize {
    #[cfg(target_vendor = "apple")]
    {
        unsafe extern "C" {
            fn _dyld_get_image_vmaddr_slide(image_index: u32) -> isize;
        }
        pc.wrapping_sub(unsafe { _dyld_get_image_vmaddr_slide(0) } as usize)
    }
    #[cfg(not(target_vendor = "apple"))]
    {
        pc
    }
}

/// Return address of the calling (non-inlined) function: the lifted function's `Drop` site.
#[inline(always)]
fn return_address() -> usize {
    #[cfg(target_arch = "aarch64")]
    {
        let lr: usize;
        unsafe { core::arch::asm!("mov {}, x30", out(reg) lr, options(nomem, nostack, preserves_flags)) };
        lr
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        0
    }
}

/// 1 = `dc zva` zeroes 64-byte blocks, 2 = not usable, 0 = unknown.
static ZVA: AtomicU8 = AtomicU8::new(0);

#[inline(always)]
fn zva_ok() -> bool {
    match ZVA.load(Ordering::Relaxed) {
        1 => true,
        2 => false,
        _ => zva_detect(),
    }
}

#[cold]
#[inline(never)]
fn zva_detect() -> bool {
    #[cfg(target_arch = "aarch64")]
    let ok = {
        let v: u64;
        unsafe { core::arch::asm!("mrs {}, dczid_el0", out(reg) v, options(nomem, nostack, preserves_flags)) };
        // BS (bits 3:0) = log2(block size in 4-byte words) = 4 for 64 bytes; DZP (bit 4) clear.
        v & 0x1f == 4
    };
    #[cfg(not(target_arch = "aarch64"))]
    let ok = false;
    ZVA.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
    ok
}

/// Zero `bytes` (> 0, a multiple of 64) at the 64-byte aligned `addr` with `dc zva`. Only
/// valid when [`zva_ok`]; on other targets this is `write_bytes`.
#[inline(always)]
unsafe fn zero_lines(addr: usize, bytes: usize) {
    #[cfg(target_arch = "aarch64")]
    core::arch::asm!(
        "2:",
        "dc zva, {p}",
        "add {p}, {p}, #64",
        "cmp {p}, {e}",
        "b.lo 2b",
        p = inout(reg) addr => _,
        e = in(reg) addr + bytes,
        options(nostack),
    );
    #[cfg(not(target_arch = "aarch64"))]
    core::ptr::write_bytes(addr as *mut u8, 0, bytes);
}

/// Zero an arena range in the slow path (honours `LS_NO_ZVA` and alignment).
unsafe fn zero_any(c: &CallCtx, addr: usize, bytes: usize) {
    if bytes == 0 {
        return;
    }
    if c.ls_flags.get() & LS_NO_ZVA == 0 && addr & 63 == 0 && bytes & 63 == 0 {
        zero_lines(addr, bytes);
    } else {
        core::ptr::write_bytes(addr as *mut u8, 0, bytes);
    }
}

/// Zero `n` bytes at the 8-byte aligned `p` (`n` a multiple of 8): `dc zva` for the 64-byte
/// lines, 8-byte stores for the unaligned head and tail. Replaces the `bzero` call of the old
/// `MaybeUninit::zeroed()` slot.
#[inline(always)]
pub unsafe fn zero_bytes(p: *mut u8, n: usize) {
    debug_assert!(p as usize & 7 == 0 && n & 7 == 0);
    #[cfg(target_arch = "aarch64")]
    if zva_ok() {
        let s = p as usize;
        let e = s + n;
        let a = (s + 63) & !63;
        let b = e & !63;
        if a < b {
            // Volatile: keep LLVM from turning the head/tail loops back into a memset call.
            let mut q = s;
            while q < a {
                (q as *mut u64).write_volatile(0);
                q += 8;
            }
            zero_lines(a, b - a);
            let mut q = b;
            while q < e {
                (q as *mut u64).write_volatile(0);
                q += 8;
            }
            return;
        }
    }
    core::ptr::write_bytes(p, 0, n);
}

/// Out-of-line [`zero_bytes`] for the `LiftedFrame` local fallback.
#[inline(never)]
pub unsafe fn zero_slot(p: *mut u8, n: usize) {
    zero_bytes(p, n);
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARGS_BYTES: usize = ARGS * 8;

    unsafe fn all_zero(lo: usize, hi: usize) -> bool {
        (lo..hi).step_by(8).all(|a| core::ptr::read_volatile(a as *const u64) == 0)
    }

    #[test]
    fn arena_end_is_followed_by_an_inaccessible_guard() {
        use win32_rt::kernel32::memory::query;
        let end = arena_end();
        assert_ne!(end, 0, "arena should be on by default");
        let base = ctx().ls_base.get();
        assert_eq!(end, base + ARENA_BYTES);
        let last = query(end - 8).expect("arena page");
        assert_eq!((last.allocation_base, last.state, last.protect), (base, MEM_COMMIT, PAGE_READWRITE));
        let guard = query(end).expect("guard page");
        assert_eq!(guard.allocation_base, base, "guard belongs to the arena's reservation");
        assert_eq!(guard.state, MEM_RESERVE, "guard is reserved, never committed");
        assert!(guard.base_address + guard.region_size >= end + ARENA_GUARD);
    }

    #[test]
    fn frames_nest_and_leave_the_arena_clean() {
        unsafe {
            let c = ctx();
            let mut a = LiftedFrame::<256, 4>::new();
            let rsp_a = a.frame_base() as usize;
            let top0 = a.top;
            assert_ne!(top0, 0, "arena should be on by default");
            assert_eq!(rsp_a, top0 - ARGS_BYTES);
            assert_eq!(c.ls_top.get(), top0 - LiftedFrame::<256, 4>::WIN);
            assert_eq!(a.out_limit(rsp_a as u64, 0xc0), top0 as u64);
            // Fresh frame: the whole old slot (and the window) reads zero.
            assert!(all_zero(top0 - 256 * 8, top0));
            // Write the frame, the home space and an argument slot.
            for off in [8usize, 0x20, 0x48] {
                ((rsp_a + off) as *mut u64).write(0xA5A5);
            }
            ((rsp_a - 4 * 8) as *mut u64).write(0x1234);
            {
                let mut b = LiftedFrame::<512, 100>::new();
                let rsp_b = b.frame_base() as usize;
                assert_eq!(b.top, top0 - LiftedFrame::<256, 4>::WIN);
                assert!(rsp_b < rsp_a);
                // Below the callee's window: still zero, as with the zeroed local.
                assert!(all_zero(b.top - 512 * 8, b.top));
                ((rsp_b - 100 * 8) as *mut u64).write(7);
                ((rsp_b + 0x10) as *mut u64).write(9);
            }
            // The callee cleaned up after itself.
            assert_eq!(c.ls_top.get(), a.low);
            assert!(all_zero(top0 - 1024 * 8, a.low));
            assert_eq!(((rsp_a - 4 * 8) as *const u64).read(), 0x1234);
            drop(a);
            assert_eq!(c.ls_top.get(), top0);
            assert!(all_zero(top0 - 1024 * 8, top0));
        }
    }

    #[test]
    fn abandoned_inner_frame_is_cleaned_by_the_outer_drop() {
        unsafe {
            let c = ctx();
            let mut a = LiftedFrame::<256, 2>::new();
            a.frame_base();
            let top0 = a.top;
            let mut b = LiftedFrame::<256, 30>::new();
            let rsp_b = b.frame_base() as usize;
            ((rsp_b - 30 * 8) as *mut u64).write(0xdead);
            core::mem::forget(b);
            drop(a);
            assert_eq!(c.ls_top.get(), top0);
            assert!(all_zero(top0 - 2048, top0));
        }
    }

    #[test]
    fn exhausted_arena_falls_back_to_a_zeroed_local() {
        unsafe {
            let c = ctx();
            let mut warm = LiftedFrame::<256, 0>::new();
            warm.frame_base();
            drop(warm);
            let saved = (c.ls_lo.get(), c.ls_base.get());
            // No room left in the arena itself (the real low end at the top), not just in the
            // budget (that raises STATUS_STACK_OVERFLOW instead, see the stack-account tests).
            c.ls_lo.set(c.ls_top.get());
            c.ls_base.set(c.ls_top.get() - ARENA_MARGIN);
            let mut f = LiftedFrame::<256, 8>::new();
            // Dirty the local first: the fallback must zero it.
            (f.buf.as_mut_ptr() as *mut u64).write_bytes(0xff, 256);
            let rsp = f.frame_base() as usize;
            c.ls_lo.set(saved.0);
            c.ls_base.set(saved.1);
            assert_eq!(f.top, 0);
            let lo = f.buf.as_ptr() as usize;
            assert_eq!(rsp, lo + (256 - ARGS) * 8);
            assert!(all_zero(lo, lo + 256 * 8));
            assert_eq!(f.out_limit(rsp as u64, 0xc0), (lo + 256 * 8) as u64);
            let top = c.ls_top.get();
            drop(f);
            assert_eq!(c.ls_top.get(), top);
        }
    }

    #[test]
    fn init_lifted_stack_and_read_s5_use_the_arena_frame() {
        unsafe {
            let c = ctx();
            let mut f = LiftedFrame::<256, 6>::new();
            let rsp = crate::simd_ops::init_lifted_stack(&mut f, 5, 6, 7, 8) as usize;
            assert_ne!(f.top, 0);
            assert_eq!(rsp, f.top - ARGS_BYTES);
            for (i, v) in [5u64, 6, 7, 8].iter().enumerate() {
                assert_eq!(((rsp + (5 + i) * 8) as *const u64).read(), *v);
            }
            assert!(all_zero(rsp, rsp + 5 * 8));
            assert!(all_zero(rsp + 9 * 8, f.top));
            let out_rsp = rsp - 0x40;
            ((out_rsp + 0x20) as *mut u64).write(55);
            assert_eq!(crate::simd_ops::read_s5_call(out_rsp as u64, &f), 55);
            assert_eq!(c.outgoing_rsp.replace((0, 0)), (out_rsp as u64, f.top as u64));
        }
    }

    /// Guest code for the emu-call test: `rax = rcx + [rsp+0x28] + [rsp+0x30] + [rsp+0x48] +
    /// [rsp+0x88]` (args 5, 6, 9 and 17), placed outside the exe VA window.
    fn emu_sum_code() -> u64 {
        const CODE: [u8; 31] = [
            0x48, 0x8B, 0x44, 0x24, 0x28, // mov rax, [rsp+0x28]
            0x48, 0x03, 0x44, 0x24, 0x30, // add rax, [rsp+0x30]
            0x48, 0x03, 0x44, 0x24, 0x48, // add rax, [rsp+0x48]
            0x48, 0x03, 0x84, 0x24, 0x88, 0x00, 0x00, 0x00, // add rax, [rsp+0x88]
            0x48, 0x01, 0xC8, // add rax, rcx
            0xC3, // ret
            0xCC, 0xCC, 0xCC, 0xCC,
        ];
        loop {
            let m: &'static mut [u8] = Box::leak(vec![0xCCu8; 4096].into_boxed_slice());
            let p = m.as_mut_ptr() as u64;
            if p + 4096 > 0x1_4000_0000 && p < 0x1_4500_0000 {
                continue;
            }
            m[..CODE.len()].copy_from_slice(&CODE);
            return p;
        }
    }

    /// ls1 boot crash: the *first* arena frame (at the arena top) tail-calls / calls an emulated
    /// target. `tail_stk` passes `rsp + 0xc8` as the readable limit and `emu_run_full` copies
    /// stack args up to it, i.e. one qword above the frame top. That must stay mapped
    /// (`ARENA_HEADROOM`); the callee must see the frame's stack args.
    #[test]
    fn first_frame_calls_into_the_emulator_with_stack_args() {
        unsafe {
            let code = emu_sum_code();
            let c = ctx();
            let mut f = LiftedFrame::<256, 5>::new();
            let base = crate::simd_ops::init_lifted_stack(&mut f, 0x10, 0x20, 0x30, 0x40);
            assert_eq!(f.top, c.ls_base.get() + ARENA_BYTES - ARENA_HEADROOM, "first frame sits at the arena top");
            assert!(base + 0xc8 > f.top as u64, "tail_stk's window reaches above the frame top");
            ((base + 0x48) as *mut u64).write(0x100); // arg 9
            ((base + 0x88) as *mut u64).write(0x1000); // arg 17
            // Tail call at the entry rsp: the callee's args 5.. are this frame's args 5...
            let r = crate::simd_ops::tail_stk(code, 0, 7, 0, 0, 0, base);
            assert_eq!(r, 7 + 0x10 + 0x20 + 0x100 + 0x1000);
            // Normal call after `push rbx; sub rsp, 0x20` (the crashing fn's shape). Caller side:
            // arg 5 at [rsp+0x20], arg 6 at [rsp+0x28], arg 9 at [rsp+0x40], arg 17 at [rsp+0x80].
            let rsp = base - 0x28;
            for (k, v) in [(0x20u64, 1u64), (0x28, 2), (0x40, 3), (0x80, 4)] {
                ((rsp + k) as *mut u64).write(v);
            }
            let r = crate::simd_ops::call_stk(code, 0, 7, 0, 0, 0, rsp);
            assert_eq!(r, 7 + 1 + 2 + 3 + 4);
            assert_eq!(c.incoming_rsp.get(), (0, 0));
            assert_eq!(c.outgoing_rsp.get(), (0, 0));
        }
    }

    #[test]
    fn check_scan_reports_depth_below_the_static_frame() {
        unsafe {
            #[repr(align(64))]
            struct Slot([u64; 256]);
            let mut s = Slot([0; 256]);
            let top = s.0.as_mut_ptr().add(256) as usize;
            let base = top - ARGS_BYTES;
            assert_eq!(check_scan(top, 256, 10, ARGS), None);
            // Inside the static frame: not reported.
            ((base - 10 * 8) as *mut u64).write(1);
            assert_eq!(check_scan(top, 256, 10, ARGS), None);
            // 3 qwords below the static frame.
            ((base - 13 * 8) as *mut u64).write(1);
            assert_eq!(check_scan(top, 256, 10, ARGS), Some(24));
            // At the slot bottom.
            (top as *mut u64).sub(256).write(1);
            assert_eq!(check_scan(top, 256, 10, ARGS), Some((256 - 24 - 10) * 8));
            // A wide frame's static frame starts 2 qwords lower.
            assert_eq!(check_scan(top, 256, 10, 26), Some((256 - 26 - 10) * 8));
        }
    }

    /// `LiftedFrame::<N, F>` (every generated frame today) is `LiftedFrame::<N, F, 24>` with
    /// the old window, entry `rsp` and layout.
    #[test]
    fn default_frame_keeps_the_24_qword_window() {
        fn same_type(f: LiftedFrame<256, 45, 24>) -> LiftedFrame<256, 45, 24> {
            f
        }
        let f = same_type(LiftedFrame::<256, 45>::new());
        assert_eq!(<LiftedFrame<256, 45> as StackBuf>::ARGS, 24);
        assert_eq!(<crate::simd_ops::LiftedStack<1> as StackBuf>::ARGS, 24);
        assert_eq!(<[u64; 256] as StackBuf>::ARGS, 24);
        assert_eq!(LiftedFrame::<256, 45>::WIN, ((45 + LSTACK_SLACK + 24) * 8 + 63) & !63);
        assert_eq!(LiftedFrame::<256, 4>::WIN, 0x140);
        assert_eq!(core::mem::size_of::<LiftedFrame<256, 45>>(), 16 + 256 * 8);
        assert_eq!(LiftedFrame::<256, 45, 26>::WIN, ((45 + LSTACK_SLACK + 26) * 8 + 63) & !63);
        drop(f);
    }

    /// Check mode (`ls_lo` sentinel + `LS_CHECK`, set by hand on a fresh thread): every frame
    /// entry takes `arena_slow`, a read of stack arg slot `A + i` returns `CANARY | i`, the
    /// pops re-zero the canaries, and the crash hint decodes them.
    #[test]
    fn check_mode_canary_marks_reads_beyond_the_window() {
        std::thread::spawn(|| unsafe {
            let c = ctx();
            shadow_warm(c);
            assert_ne!(c.ls_top.get(), 0);
            c.ls_lo.set(CHECK_LO);
            c.ls_flags.set(c.ls_flags.get() | LS_CHECK);
            let top0 = c.ls_top.get();
            {
                let mut a = LiftedFrame::<256, 4>::new();
                let rsp_a = crate::simd_ops::init_lifted_stack(&mut a, 5, 6, 7, 8) as usize;
                assert_eq!(a.top, top0);
                assert_eq!(arena_lo(c), c.ls_base.get() + ARENA_MARGIN);
                assert_eq!(((rsp_a + 24 * 8) as *const u64).read(), CANARY);
                assert_eq!(((rsp_a + 25 * 8) as *const u64).read(), CANARY | 1);
                {
                    let mut b = LiftedFrame::<256, 6, 26>::new();
                    let rsp_b = crate::simd_ops::init_lifted_stack(&mut b, 0, 0, 0, 0) as usize;
                    assert_eq!(b.top, a.low);
                    // Wide frame: slots 24/25 are its own (zero), 26.. the canary.
                    assert_eq!(((rsp_b + 25 * 8) as *const u64).read(), 0);
                    assert_eq!(((rsp_b + 26 * 8) as *const u64).read(), CANARY);
                    assert_eq!(((rsp_b + 33 * 8) as *const u64).read(), CANARY | 7);
                }
                // The callee's pop re-zeroed its canaries in this frame's slack.
                assert!(all_zero(a.low, a.low + 64));
                assert_eq!(((rsp_a + 24 * 8) as *const u64).read(), CANARY);
            }
            assert_eq!(c.ls_top.get(), top0);
            assert!(all_zero(top0 - 2048, top0 + 64));
            // An abandoned callee's canaries are not reported as a write below the frame.
            assert_eq!(check_scan(top0, 256, 4, 24), None);
            (top0 as *mut u64).sub(200).write(CANARY | 3);
            assert_eq!(check_scan(top0, 256, 4, 24), None);
            (top0 as *mut u64).sub(200).write(0);
            assert!(describe_canary(CANARY | 2).unwrap().contains("slot A+2"));
            assert!(describe_canary((CANARY | 1) & 0x00FF_FFFF_FFFF_FFFF).is_some(), "TBI-stripped");
            assert!(describe_canary(0x1_4000_0000).is_none());
            c.ls_flags.set(c.ls_flags.get() & !LS_CHECK);
            c.ls_lo.set(arena_lo(c));
        })
        .join()
        .unwrap();
    }

    /// Caller outgoing area with arg `k` = `0xa000 + k` at `[rsp + (k-1)*8]` (args 5..32),
    /// published as the incoming window `(rsp, rsp + lim)` the way `call_stk*` does.
    fn publish_outgoing(area: &mut [u64; 40], lim: u64) -> u64 {
        for k in 5..=32 {
            area[k - 1] = 0xa000 + k as u64;
        }
        let rsp = area.as_ptr() as u64;
        ctx().incoming_rsp.set((rsp, rsp + lim));
        rsp
    }

    #[test]
    fn wide_frame_receives_args_up_to_its_window() {
        unsafe {
            let c = ctx();
            let mut area = [0u64; 40];
            // Default frame: args 9..23; slot 24 is the frame top (caller's territory).
            {
                publish_outgoing(&mut area, 0xd0);
                let mut f = LiftedFrame::<256, 6>::new();
                let rsp = crate::simd_ops::init_lifted_stack(&mut f, 5, 6, 7, 8) as usize;
                assert_eq!(rsp, f.top - 24 * 8);
                for k in 9..=23 {
                    assert_eq!(((rsp + k * 8) as *const u64).read(), 0xa000 + k as u64, "arg{k}");
                }
                assert_eq!(c.incoming_rsp.get(), (0, 0));
            }
            // A = 26 (max slot 25) behind a 0xd0 outgoing window: args 9..25.
            let top0 = c.ls_top.get();
            {
                publish_outgoing(&mut area, 0xd0);
                let mut f = LiftedFrame::<256, 6, 26>::new();
                let rsp = crate::simd_ops::init_lifted_stack(&mut f, 5, 6, 7, 8) as usize;
                assert_eq!(f.top, top0);
                assert_eq!(rsp, f.top - 26 * 8);
                assert_eq!(c.ls_top.get(), f.top - LiftedFrame::<256, 6, 26>::WIN);
                for (i, v) in [5u64, 6, 7, 8].iter().enumerate() {
                    assert_eq!(((rsp + (5 + i) * 8) as *const u64).read(), *v);
                }
                for k in 9..=25 {
                    assert_eq!(((rsp + k * 8) as *const u64).read(), 0xa000 + k as u64, "arg{k}");
                }
                assert_eq!(f.out_limit(rsp as u64, 0xc0), f.top as u64);
            }
            assert_eq!(c.ls_top.get(), top0);
            assert!(all_zero(top0 - 1024 * 8, top0));
            // The same frame behind today's 0xc0 window: arg24 arrives, arg25 stays 0.
            {
                publish_outgoing(&mut area, 0xc0);
                let mut f = LiftedFrame::<256, 6, 26>::new();
                let rsp = crate::simd_ops::init_lifted_stack(&mut f, 5, 6, 7, 8) as usize;
                assert_eq!(((rsp + 24 * 8) as *const u64).read(), 0xa000 + 24);
                assert_eq!(((rsp + 25 * 8) as *const u64).read(), 0);
            }
            assert!(all_zero(top0 - 1024 * 8, top0));
            // Big (N = 1) and local-slot frames take the window too.
            {
                publish_outgoing(&mut area, 0xd0);
                let mut s = crate::simd_ops::LiftedStack::<1, 26>::new();
                let rsp = crate::simd_ops::init_lifted_stack(&mut s, 5, 6, 7, 8) as usize;
                for k in 9..=25 {
                    assert_eq!(((rsp + k * 8) as *const u64).read(), 0xa000 + k as u64, "big arg{k}");
                }
            }
            {
                publish_outgoing(&mut area, 0xd0);
                let mut s = crate::simd_ops::LiftedStack::<256, 26>::new();
                let rsp = crate::simd_ops::init_lifted_stack(&mut s, 5, 6, 7, 8) as usize;
                assert_eq!(rsp, s.base() as usize + (256 - 26) * 8);
                assert_eq!(((rsp + 25 * 8) as *const u64).read(), 0xa000 + 25);
            }
        }
    }

    #[test]
    fn zero_bytes_handles_unaligned_heads_and_tails() {
        let mut v = vec![0xffu64; 300];
        for (start, len) in [(0usize, 300usize), (1, 256), (3, 9), (5, 200), (7, 1)] {
            v.iter_mut().for_each(|x| *x = !0);
            unsafe { zero_bytes(v.as_mut_ptr().add(start) as *mut u8, len * 8) };
            for (i, x) in v.iter().enumerate() {
                let inside = i >= start && i < start + len;
                assert_eq!(*x == 0, inside, "start {start} len {len} index {i}");
            }
        }
    }

    // -----------------------------------------------------------------------
    // Stack account
    // -----------------------------------------------------------------------

    type Fr = LiftedFrame<256, 40>;

    /// Guest `mov rax, rsp; ret`: returns its entry `rsp`.
    fn rsp_code() -> u64 {
        let p = crate::eh_rt::testutil::alloc(16, 16);
        unsafe { core::ptr::copy_nonoverlapping([0x48u8, 0x89, 0xE0, 0xC3, 0xCC, 0xCC].as_ptr(), p, 6) };
        p as u64
    }

    /// Guest `sub rsp, 0x28; mov rax, probe; call rax; add rsp, 0x28; ret` (rcx..r9 passed on).
    fn calls_probe_code() -> u64 {
        use iced_x86::code_asm::*;
        let probe_va = crate::eh_rt::testutil::native_va(probe as *const () as usize);
        let mut a = CodeAssembler::new(64).unwrap();
        a.sub(rsp, 0x28).unwrap();
        a.mov(rax, probe_va).unwrap();
        a.call(rax).unwrap();
        a.add(rsp, 0x28).unwrap();
        a.ret().unwrap();
        crate::eh_rt::testutil::place(a, 64).0
    }

    std::thread_local! {
        /// `probe` entries: (ls_anchor, ls_top, ls_lo).
        static PROBES: core::cell::RefCell<Vec<(usize, usize, usize)>> = const { core::cell::RefCell::new(Vec::new()) };
    }

    /// `k` nested lifted frames, then `f`.
    #[inline(never)]
    unsafe fn frames(k: usize, f: &mut dyn FnMut() -> u64) -> u64 {
        if k == 0 {
            return f();
        }
        let mut fr = Fr::new();
        let p = fr.frame_base();
        let r = frames(k - 1, f);
        drop(fr);
        core::hint::black_box(p);
        r
    }

    /// Host function called from guest code: logs the account, nests `k` lifted frames, then
    /// runs guest `next` with `(a0, a1)` and returns its rax.
    unsafe extern "win64" fn probe(k: u64, next: u64, a0: u64, a1: u64) -> u64 {
        let c = ctx();
        PROBES.with(|p| p.borrow_mut().push((c.ls_anchor.get(), c.ls_top.get(), c.ls_lo.get())));
        frames(k as usize, &mut || crate::emu::emu_run_full(next, [a0, a1, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0)
    }

    fn on_big_stack<F: FnOnce() + Send + 'static>(f: F) {
        std::thread::Builder::new().stack_size(64 << 20).spawn(f).unwrap().join().unwrap();
    }

    /// Lifted frames between a caller and a guest callee move the callee's `rsp` down by their
    /// windows, with no anchor (host -> lifted -> guest) and under a guest -> host anchor.
    #[test]
    fn lifted_depth_moves_the_guest_stack_position() {
        on_big_stack(|| unsafe {
            let c = ctx();
            let rspc = rsp_code();
            let run = |va: u64, a: [u64; 4]| crate::emu::emu_run_full(va, [a[0], a[1], a[2], a[3], 0, 0, 0, 0], [0; 4], [0; 8]).0;
            shadow_warm(c);
            let top0 = c.ls_top.get();
            assert_eq!(top0, arena_top0(c));
            let r0 = run(rspc, [0; 4]);
            let r3 = frames(3, &mut || run(rspc, [0; 4]));
            assert_eq!(r0 - r3, 3 * Fr::WIN as u64, "host -> 3 lifted -> guest");
            assert_eq!(c.ls_top.get(), top0);
            // Guest -> host (anchor) -> k lifted -> guest.
            let cp = calls_probe_code();
            let lo0 = c.ls_lo.get();
            let g0 = run(cp, [0, rspc, 0, 0]);
            let g2 = run(cp, [2, rspc, 0, 0]);
            assert_eq!(g0 - g2, 2 * Fr::WIN as u64, "guest -> host -> 2 lifted -> guest");
            // The anchored callee sits below its guest caller, not at the arena-wide depth.
            assert!(g0 < r0 && r0 - g0 < 0x1000, "g0 {g0:#x} r0 {r0:#x}");
            assert_eq!((c.ls_anchor.get(), c.ls_lo.get(), c.ls_top.get()), (0, lo0, top0));
            let p = PROBES.with(|p| p.take());
            assert_eq!(p.len(), 2);
            for (anchor, top, lo) in p {
                assert_eq!((anchor, top), (top0, top0), "anchor = arena top at the call");
                assert!(lo > lo0 && lo < top0, "budget below the guest caller: lo {lo:#x}, idle {lo0:#x}");
            }
        });
    }

    /// Nested guest -> host calls each anchor at their own arena top and restore the outer
    /// anchor and budget when they return.
    #[test]
    fn host_call_restores_the_anchor() {
        on_big_stack(|| unsafe {
            let c = ctx();
            shadow_warm(c);
            let rspc = rsp_code();
            let cp = calls_probe_code();
            let (top0, lo0) = (c.ls_top.get(), c.ls_lo.get());
            // probe(2, cp, 1, rspc) -> 2 frames -> guest cp -> probe(1, rspc) -> 1 frame -> guest rsp.
            let mut outer = (0, 0);
            let r = frames(1, &mut || {
                outer = (c.ls_top.get(), c.ls_lo.get());
                crate::emu::emu_run_full(cp, [2, cp, 1, rspc, 0, 0, 0, 0], [0; 4], [0; 8]).0
            });
            assert_ne!(r, 0);
            let p = PROBES.with(|p| p.take());
            assert_eq!(p.len(), 2, "{p:#x?}");
            let (a1, t1, lo1) = p[0];
            let (a2, t2, lo2) = p[1];
            assert_eq!(a1, t1);
            assert_eq!(t1, top0 - Fr::WIN, "outer call under one lifted frame");
            assert_eq!(a2, t2);
            assert_eq!(t2, t1 - 2 * Fr::WIN, "inner call under the outer call's two frames");
            // The inner budget is the outer one minus the inner guest caller's own frame (the
            // two lifted frames are charged in both the anchor and the guest position).
            assert!(lo2 > lo1 && lo2 - lo1 < 0x1000, "lo1 {lo1:#x} lo2 {lo2:#x}");
            assert_eq!(outer, (top0 - Fr::WIN, lo0));
            assert_eq!((c.ls_anchor.get(), c.ls_lo.get(), c.ls_top.get()), (0, lo0, top0));
            // anchor_enter / anchor_leave by hand, two deep.
            let s0 = anchor_enter(c, c.ls_gstack.get().1 - 0x1000);
            let (am, lm) = (c.ls_anchor.get(), c.ls_lo.get());
            let s1 = anchor_enter(c, c.ls_gstack.get().0 + 0x8000);
            assert_eq!(c.ls_lo.get(), top0 - 0x8000);
            anchor_leave(c, s1);
            assert_eq!((c.ls_anchor.get(), c.ls_lo.get()), (am, lm));
            anchor_leave(c, s0);
            assert_eq!((c.ls_anchor.get(), c.ls_lo.get()), (0, lo0));
        });
    }

    /// Lifted recursion until the frame entry raises; returns the depth reached.
    #[inline(never)]
    unsafe fn dive(n: &core::cell::Cell<usize>, min_sp: &core::cell::Cell<usize>) -> u64 {
        if n.get() > 1_000_000 {
            return 0;
        }
        let mut fr = Fr::new();
        let p = fr.frame_base();
        n.set(n.get() + 1);
        min_sp.set(min_sp.get().min(host_sp()));
        let r = dive(n, min_sp).wrapping_add(p as u64);
        drop(fr);
        r
    }

    fn catch_overflow<R>(f: impl FnOnce() -> R) -> Result<R, crate::eh_rt::host_try::Caught> {
        unsafe { crate::eh_rt::host_try::catch_code_if(STATUS_STACK_OVERFLOW, &mut |_| Some([0; 2]), f) }
    }

    #[test]
    fn lifted_recursion_past_the_guest_budget_raises_stack_overflow() {
        on_big_stack(|| unsafe {
            let c = ctx();
            shadow_warm(c);
            crate::emu::emu_run_full(rsp_code(), [0; 8], [0; 4], [0; 8]);
            let (floor, _) = c.ls_gstack.get();
            assert_ne!(floor, 0);
            let (top0, lo0, eh0) = (c.ls_top.get(), c.ls_lo.get(), c.eh_top.get());
            let n = core::cell::Cell::new(0usize);
            let min_sp = core::cell::Cell::new(usize::MAX);
            let room = 20 * Fr::WIN;
            let r = catch_overflow(|| {
                // As a guest -> host call made with `room` guest bytes above the floor.
                let _ = anchor_enter(c, floor + room as u64);
                dive(&n, &min_sp)
            });
            let e = r.err().expect("the recursion raised");
            assert_eq!(e.rec.code, STATUS_STACK_OVERFLOW);
            assert_eq!(e.rec.flags & win32_rt::unwind::EXCEPTION_NONCONTINUABLE, win32_rt::unwind::EXCEPTION_NONCONTINUABLE);
            // Frames enter while `top - REACH >= anchor - room`.
            let reach = Fr::WIN.max(256 * 8);
            assert_eq!(n.get(), (room - reach) / Fr::WIN + 1, "depth under a {room:#x}-byte budget");
            // The landing restored the account, the arena top (cleaned) and the chain.
            assert_eq!((c.ls_top.get(), c.ls_lo.get(), c.ls_anchor.get()), (top0, lo0, 0));
            assert_eq!(c.eh_top.get(), eh0);
            assert!(all_zero(top0 - 64 * 1024, top0));
            // The thread goes on: the next frames use the arena from the top again.
            let mut f = Fr::new();
            f.frame_base();
            assert_eq!(f.top, top0);
        });
    }

    #[test]
    fn lifted_recursion_past_the_host_stack_margin_raises_stack_overflow() {
        on_big_stack(|| unsafe {
            let c = ctx();
            shadow_warm(c);
            let (lo, hi) = host_stack_bounds().expect("host stack bounds");
            let sp = host_sp();
            assert!((lo..hi).contains(&sp));
            assert_eq!(c.ls_hlo.get(), lo + HOST_MARGIN, "64 MiB stack: the full margin");
            let (top0, lo0, hlo0) = (c.ls_top.get(), c.ls_lo.get(), c.ls_hlo.get());
            // Pretend the host stack ends 256 KiB below here.
            let limit = sp - (256 << 10);
            c.ls_hlo.set(limit);
            let n = core::cell::Cell::new(0usize);
            let min_sp = core::cell::Cell::new(usize::MAX);
            let r = catch_overflow(|| dive(&n, &min_sp));
            c.ls_hlo.set(hlo0);
            assert_eq!(r.err().expect("the recursion raised").rec.code, STATUS_STACK_OVERFLOW);
            assert!(n.get() > 10, "depth {}", n.get());
            assert!(min_sp.get() >= limit, "a frame entered below the host limit");
            assert_eq!((c.ls_top.get(), c.ls_lo.get()), (top0, lo0));
            assert!(all_zero(top0 - 256 * 1024, top0));
        });
    }

    /// A guest callee whose start, charged with the lifted depth, would fall below the guest
    /// floor raises before running; just above the floor it runs there.
    #[test]
    fn guest_region_below_the_floor_raises_stack_overflow() {
        on_big_stack(|| unsafe {
            let c = ctx();
            shadow_warm(c);
            let rspc = rsp_code();
            let run = || crate::emu::emu_run_full(rspc, [0; 8], [0; 4], [0; 8]).0;
            let r0 = run();
            let (floor, gtop) = c.ls_gstack.get();
            let top0 = c.ls_top.get();
            assert_eq!(top0, arena_top0(c));
            let room = (gtop - floor) as usize;
            // Fake a lifted depth (the arena below the top is clean and stays untouched).
            let inside = (room - 0x1000) & !63;
            c.ls_top.set(top0 - inside);
            let r1 = run();
            assert_eq!(r0 - r1, inside as u64);
            c.ls_top.set(top0 - ((room + 0x1000) & !63));
            let r = catch_overflow(run);
            c.ls_top.set(top0);
            assert_eq!(r.err().expect("the region raised").rec.code, STATUS_STACK_OVERFLOW);
            assert_eq!(run(), r0, "the guest stack pointer was not moved");
        });
    }
}
