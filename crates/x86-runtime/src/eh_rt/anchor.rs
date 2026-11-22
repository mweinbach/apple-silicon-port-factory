//! Host anchors: a non-local jump back into a live Rust frame (target/review/exec/g12/eh_lift_plan.md
//! §4.3), and the per-thread state a landing restores.
//!
//! [`enter`] saves the aarch64 callee-saved state (x19–x28, fp, lr, sp, d8–d15) into an
//! [`Anchor`] and calls `f(arg)`. If `f` returns, `enter` returns 0. A later [`jump`] to the
//! same anchor, made from anywhere deeper on the same thread while `enter` is still live,
//! returns from that `enter` call with the jump value instead: every Rust frame between is
//! discarded without running its `Drop` (panic=abort; there is no Rust unwinding).
//!
//! This is `setjmp`/`longjmp` without "returns twice": the caller sees an ordinary call that
//! returns once, with its callee-saved registers and stack slots exactly as at the call, so
//! the compiler's assumptions hold. The skipped frames' side state is what [`Snapshot`]
//! restores:
//! - every `CallCtx` field that a call protocol changes and restores (forwarded XMM / rax,
//!   rsp windows, lstack top, EH chain head); the rest is per-thread setup or a cache;
//! - the emulator's innermost running CPU (`CUR_CPU`) and guest stack allocation pointer.
//! The lstack range the abandoned frames used is zeroed again (its invariant: everything
//! below the arena top is zero).
use crate::callctx::{ctx, CallCtx};

/// Saved callee-saved state of one [`enter`] call.
#[repr(C, align(16))]
pub struct Anchor {
    /// x19..x28, x29 (fp), x30 (lr), sp, d8..d15.
    regs: [u64; 21],
}

impl Anchor {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { regs: [0; 21] }
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
core::arch::global_asm!(
    ".globl _aw2_anchor_enter",
    ".p2align 2",
    "_aw2_anchor_enter:",
    // x0 = anchor, x1 = f, x2 = arg. Save the state `jump` restores, as of this call.
    "stp x19, x20, [x0, #0]",
    "stp x21, x22, [x0, #16]",
    "stp x23, x24, [x0, #32]",
    "stp x25, x26, [x0, #48]",
    "stp x27, x28, [x0, #64]",
    "stp x29, x30, [x0, #80]",
    "mov x9, sp",
    "str x9, [x0, #96]",
    "stp d8, d9, [x0, #104]",
    "stp d10, d11, [x0, #120]",
    "stp d12, d13, [x0, #136]",
    "stp d14, d15, [x0, #152]",
    "stp x29, x30, [sp, #-16]!",
    "mov x29, sp",
    "mov x0, x2",
    "blr x1",
    "ldp x29, x30, [sp], #16",
    "mov x0, #0",
    "ret",
    "",
    ".globl _aw2_anchor_jump",
    ".p2align 2",
    "_aw2_anchor_jump:",
    // x0 = anchor, x1 = value (!= 0): return from that anchor's `enter` with x0 = value.
    "ldp x19, x20, [x0, #0]",
    "ldp x21, x22, [x0, #16]",
    "ldp x23, x24, [x0, #32]",
    "ldp x25, x26, [x0, #48]",
    "ldp x27, x28, [x0, #64]",
    "ldp x29, x30, [x0, #80]",
    "ldr x9, [x0, #96]",
    "mov sp, x9",
    "ldp d8, d9, [x0, #104]",
    "ldp d10, d11, [x0, #120]",
    "ldp d12, d13, [x0, #136]",
    "ldp d14, d15, [x0, #152]",
    "mov x0, x1",
    "ret",
);

#[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
unsafe extern "C" {
    fn aw2_anchor_enter(a: *mut Anchor, f: unsafe extern "C" fn(*mut u8), arg: *mut u8) -> u64;
    fn aw2_anchor_jump(a: *const Anchor, v: u64) -> !;
}

/// Call `f(arg)` under anchor `a`: 0 when `f` returns, else the value of the [`jump`] that
/// landed here. `a` must stay valid (and is only jumpable) while this call is live.
#[inline(always)]
pub unsafe fn enter(a: *mut Anchor, f: unsafe extern "C" fn(*mut u8), arg: *mut u8) -> u64 {
    #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
    {
        aw2_anchor_enter(a, f, arg)
    }
    #[cfg(not(all(target_arch = "aarch64", target_vendor = "apple")))]
    {
        let _ = a;
        f(arg);
        0
    }
}

/// Return from the live [`enter`] of `a` with `v` (made non-zero).
pub unsafe fn jump(a: *const Anchor, v: u64) -> ! {
    let v = if v == 0 { 1 } else { v };
    #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
    {
        aw2_anchor_jump(a, v)
    }
    #[cfg(not(all(target_arch = "aarch64", target_vendor = "apple")))]
    {
        let _ = (a, v);
        log::error!("anchor jump: not supported on this target");
        std::process::abort()
    }
}

/// [`enter`] for a closure: `Some(result)` when it returns, `None` when a jump landed (the
/// jump value is then in `*landed`).
#[inline(always)]
pub unsafe fn enter_with<R, F: FnOnce() -> R>(a: *mut Anchor, f: F, landed: &mut u64) -> Option<R> {
    struct Slot<R, F> {
        f: Option<F>,
        r: Option<R>,
    }
    unsafe extern "C" fn tramp<R, F: FnOnce() -> R>(p: *mut u8) {
        let s = unsafe { &mut *(p as *mut Slot<R, F>) };
        let f = s.f.take().unwrap_unchecked();
        s.r = Some(f());
    }
    let mut s = Slot::<R, F> { f: Some(f), r: None };
    let v = enter(a, tramp::<R, F>, &mut s as *mut Slot<R, F> as *mut u8);
    *landed = v;
    if v == 0 { s.r } else { None }
}

/// The per-thread state a landing restores (see the module doc).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Snapshot {
    pub xmm_in: [u128; 4],
    pub xmm_out: Option<u128>,
    pub rax_in: u64,
    pub incoming_rsp: (u64, u64),
    pub outgoing_rsp: (u64, u64),
    pub xmm_in_set: bool,
    pub ls_top: usize,
    /// Lifted budget low end (`CallCtx::ls_lo`): `emu::host_call` lowers/raises it with the anchor.
    pub ls_lo: usize,
    /// Stack-account anchor (`CallCtx::ls_anchor`).
    pub ls_anchor: usize,
    pub eh_top: usize,
    /// Emulator: innermost running CPU (`CUR_CPU`).
    pub cur_cpu: usize,
    /// Emulator: guest stack allocation pointer (`ThreadEmuState::sp`; 0 = no emulator state).
    pub emu_sp: u64,
}

/// `CallCtx` fields by role. Every field is listed here exactly once:
/// [`snapshot_fields_exhaustive`] destructures `CallCtx` without `..`, so adding a field fails
/// the build until it is classified (and, if a call protocol changes it, snapshotted).
/// - snapshotted: xmm_in, xmm_out, rax_in, incoming_rsp, outgoing_rsp, xmm_in_set, ls_top,
///   ls_lo, ls_anchor, eh_top (`ls_lo` / `ls_anchor`: the stack account, set and restored
///   around every guest -> host call, see `crate::lstack`);
/// - per-thread setup, constant while frames run: gs, ls_base, ls_flags, ls_hlo, ls_gstack;
/// - cache (any content is valid): dcache.
#[allow(dead_code)]
fn snapshot_fields_exhaustive(c: &CallCtx) {
    let CallCtx {
        xmm_in: _,
        xmm_out: _,
        rax_in: _,
        incoming_rsp: _,
        outgoing_rsp: _,
        xmm_in_set: _,
        gs: _,
        ls_top: _,
        ls_lo: _,
        ls_base: _,
        ls_flags: _,
        ls_anchor: _,
        ls_hlo: _,
        ls_gstack: _,
        eh_top: _,
        dcache: _,
    } = c;
}

/// Size of `CallCtx` the classification above was made for (compile-time guard, with the
/// destructuring, against a field added without updating [`Snapshot`]).
pub const SNAPSHOT_COVERS_BYTES: usize = core::mem::size_of::<CallCtx>();
const CLASSIFIED_FIELD_BYTES: usize = core::mem::size_of::<[u128; 4]>() // xmm_in
    + core::mem::size_of::<Option<u128>>() // xmm_out
    + 8 // rax_in
    + 16 + 16 // incoming_rsp, outgoing_rsp
    + 1 // xmm_in_set
    + 8 + 8 + 8 + 8 // gs, ls_top, ls_lo, ls_base
    + 4 // ls_flags
    + 8 + 8 + 16 // ls_anchor, ls_hlo, ls_gstack
    + 8 // eh_top
    + core::mem::size_of::<crate::callctx::DispatchSlot>() * crate::callctx::DCACHE_LEN;
const CALLCTX_ALIGN: usize = core::mem::align_of::<CallCtx>();
const _: () = assert!(
    SNAPSHOT_COVERS_BYTES == (CLASSIFIED_FIELD_BYTES + CALLCTX_ALIGN - 1) / CALLCTX_ALIGN * CALLCTX_ALIGN
);

impl Snapshot {
    /// The current state, with the emulator part given by the caller (it knows it, and the
    /// emulator's thread-locals are not free to read).
    #[inline(always)]
    pub fn take_with(cur_cpu: usize, emu_sp: u64) -> Self {
        Self::take_in(ctx(), cur_cpu, emu_sp)
    }

    #[inline(always)]
    pub fn take_in(c: &CallCtx, cur_cpu: usize, emu_sp: u64) -> Self {
        // The arena first: a snapshot of a thread without one yet would record top 0, and a
        // landing restoring it would leave the (by then set up) arena unusable for the rest
        // of the thread (every later frame on the local fallback slot).
        if c.ls_lo.get() == 0 {
            unsafe { crate::lstack::shadow_warm(c) };
        }
        Self {
            xmm_in: c.xmm_in.get(),
            xmm_out: c.xmm_out.get(),
            rax_in: c.rax_in.get(),
            incoming_rsp: c.incoming_rsp.get(),
            outgoing_rsp: c.outgoing_rsp.get(),
            xmm_in_set: c.xmm_in_set.get(),
            ls_top: c.ls_top.get(),
            ls_lo: c.ls_lo.get(),
            ls_anchor: c.ls_anchor.get(),
            eh_top: c.eh_top.get() as usize,
            cur_cpu,
            emu_sp,
        }
    }

    /// The current state, emulator part included.
    pub fn take() -> Self {
        let (cpu, sp) = crate::emu::eh_emu_state();
        Self::take_with(cpu, sp)
    }

    /// Restore after a landing: the `CallCtx` fields, the emulator state, and the lstack
    /// range abandoned frames used (zeroed, then the top restored).
    pub unsafe fn restore(&self) {
        let c = ctx();
        c.xmm_in.set(self.xmm_in);
        c.xmm_out.set(self.xmm_out);
        c.rax_in.set(self.rax_in);
        c.incoming_rsp.set(self.incoming_rsp);
        c.outgoing_rsp.set(self.outgoing_rsp);
        c.xmm_in_set.set(self.xmm_in_set);
        let top = c.ls_top.get();
        if self.ls_top != 0 && top != 0 && top < self.ls_top {
            crate::lstack::zero_bytes(top as *mut u8, self.ls_top - top);
        }
        c.ls_top.set(self.ls_top);
        c.ls_lo.set(self.ls_lo);
        c.ls_anchor.set(self.ls_anchor);
        c.eh_top.set(self.eh_top as *mut win32_rt::unwind::EhLink);
        crate::emu::eh_set_emu_state(self.cur_cpu, self.emu_sp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;

    #[test]
    fn anchor_set_returns_zero_then_jump_value() {
        unsafe {
            let mut a = Anchor::new();
            let mut landed = 0;
            assert_eq!(enter_with(&mut a, || 5u32, &mut landed), Some(5));
            assert_eq!(landed, 0, "a returning callee: enter returns 0");
            let ap: *const Anchor = &a;
            let r: Option<u32> = enter_with(&mut a, || jump(ap, 7), &mut landed);
            assert_eq!((r, landed), (None, 7), "a jump: enter returns the jump value");
            // A jump from several Rust frames deeper.
            #[inline(never)]
            fn deep(n: u32, a: *const Anchor) -> u32 {
                if n == 0 {
                    unsafe { jump(a, 9) }
                }
                core::hint::black_box(deep(n - 1, a)) + 1
            }
            let r = enter_with(&mut a, || deep(50, ap), &mut landed);
            assert_eq!((r, landed), (None, 9));
        }
    }

    /// x19..x28, d8..d15 and fp hold sentinels set right before `enter`; the callee clobbers
    /// all of them, then jumps; after the landing they hold the sentinels again.
    #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
    #[test]
    fn anchor_jump_preserves_callee_saved() {
        unsafe extern "C" fn clobber_and_jump(a: *mut u8) {
            unsafe {
                core::arch::asm!(
                    "mov x9, #0xdead",
                    "mov x19, x9", "mov x20, x9", "mov x21, x9", "mov x22, x9", "mov x23, x9",
                    "mov x24, x9", "mov x25, x9", "mov x26, x9", "mov x27, x9", "mov x28, x9",
                    "fmov d8, x9", "fmov d9, x9", "fmov d10, x9", "fmov d11, x9",
                    "fmov d12, x9", "fmov d13, x9", "fmov d14, x9", "fmov d15, x9",
                    "mov x29, x9",
                    "mov x1, #3",
                    "b {j}",
                    j = sym aw2_anchor_jump,
                    in("x0") a,
                    options(noreturn),
                );
            }
        }
        unsafe extern "C" {
            fn aw2_anchor_test_probe(a: *mut Anchor, f: unsafe extern "C" fn(*mut u8), out: *mut u64) -> u64;
        }
        let mut a = Anchor::new();
        let mut out = [0u64; 20];
        let v = unsafe { aw2_anchor_test_probe(&mut a, clobber_and_jump, out.as_mut_ptr()) };
        assert_eq!(v, 3);
        for (i, &x) in out[..10].iter().enumerate() {
            assert_eq!(x, 0x1900 + 19 + i as u64, "x{}", 19 + i);
        }
        for (i, &x) in out[10..18].iter().enumerate() {
            assert_eq!(x, 0x2800 + 8 + i as u64, "d{}", 8 + i);
        }
        assert_eq!(out[18], out[19], "fp after the landing == fp before enter");
    }

    // Test probe: set x19..x28 / d8..d15 to sentinels, record fp, `enter(a, f, a)`, then store
    // x19..x28, d8..d15 and fp to `out` (out[18] = fp after, out[19] = fp before) and return
    // enter's value. Callee-saved registers are saved/restored around the probe itself.
    #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
    core::arch::global_asm!(
        ".globl _aw2_anchor_test_probe",
        ".p2align 2",
        "_aw2_anchor_test_probe:",
        "stp x29, x30, [sp, #-160]!",
        "mov x29, sp",
        "stp x19, x20, [sp, #16]",
        "stp x21, x22, [sp, #32]",
        "stp x23, x24, [sp, #48]",
        "stp x25, x26, [sp, #64]",
        "stp x27, x28, [sp, #80]",
        "stp d8, d9, [sp, #96]",
        "stp d10, d11, [sp, #112]",
        "stp d12, d13, [sp, #128]",
        "stp d14, d15, [sp, #144]",
        "mov x19, #0x1913", "mov x20, #0x1914", "mov x21, #0x1915", "mov x22, #0x1916",
        "mov x23, #0x1917", "mov x24, #0x1918", "mov x25, #0x1919", "mov x26, #0x191a",
        "mov x27, #0x191b", "mov x28, x2",
        "mov x9, #0x2808", "fmov d8, x9", "mov x9, #0x2809", "fmov d9, x9",
        "mov x9, #0x280a", "fmov d10, x9", "mov x9, #0x280b", "fmov d11, x9",
        "mov x9, #0x280c", "fmov d12, x9", "mov x9, #0x280d", "fmov d13, x9",
        "mov x9, #0x280e", "fmov d14, x9", "mov x9, #0x280f", "fmov d15, x9",
        "str x29, [x28, #152]",
        // enter(a = x0, f = x1, arg = a)
        "mov x2, x0",
        "bl _aw2_anchor_enter",
        "stp x19, x20, [x28, #0]",
        "stp x21, x22, [x28, #16]",
        "stp x23, x24, [x28, #32]",
        "stp x25, x26, [x28, #48]",
        "mov x9, #0x191c",
        "stp x27, x9, [x28, #64]",
        "stp d8, d9, [x28, #80]",
        "stp d10, d11, [x28, #96]",
        "stp d12, d13, [x28, #112]",
        "stp d14, d15, [x28, #128]",
        "str x29, [x28, #144]",
        "ldp x19, x20, [sp, #16]",
        "ldp x21, x22, [sp, #32]",
        "ldp x23, x24, [sp, #48]",
        "ldp x25, x26, [sp, #64]",
        "ldp x27, x28, [sp, #80]",
        "ldp d8, d9, [sp, #96]",
        "ldp d10, d11, [sp, #112]",
        "ldp d12, d13, [sp, #128]",
        "ldp d14, d15, [sp, #144]",
        "ldp x29, x30, [sp], #160",
        "ret",
    );

    /// Nested `LiftedFrame`s abandoned by a jump to an outer anchor: the next outer frame's
    /// `Drop` restores the arena top and zeroes the abandoned range (lstack's non-LIFO
    /// safety); a landing's `Snapshot::restore` does the same at the landing.
    #[test]
    fn anchor_jump_discards_inner_lifted_frames() {
        use crate::simd_ops::{init_lifted_stack, LiftedFrame};
        unsafe {
            let c = ctx();
            // Outer frame: its window is live while the inner ones are abandoned.
            let mut outer = LiftedFrame::<256, 16>::new();
            let _orsp = init_lifted_stack(&mut outer, 0, 0, 0, 0);
            let top0 = c.ls_top.get();
            let mut a = Anchor::new();
            let ap: *const Anchor = &a;
            let deepest = Cell::new(0usize);
            #[inline(never)]
            unsafe fn nest(n: u32, a: *const Anchor, deepest: &Cell<usize>) {
                let mut f = LiftedFrame::<256, 16>::new();
                let rsp = unsafe { init_lifted_stack(&mut f, 0x11, 0x22, 0x33, 0x44) };
                // Dirty the frame's own window.
                unsafe { ((rsp - 0x40) as *mut u64).write(0x5555_5555) };
                deepest.set(ctx().ls_top.get());
                if n == 0 {
                    unsafe { jump(a, 1) }
                }
                unsafe { nest(n - 1, a, deepest) };
            }
            let snap = Snapshot::take();
            let mut landed = 0;
            let r = enter_with(&mut a, || nest(4, ap, &deepest), &mut landed);
            assert!(r.is_none() && landed == 1);
            assert!(deepest.get() < top0, "inner frames took arena space");
            assert_ne!(c.ls_top.get(), top0, "the abandoned frames' Drop did not run");
            snap.restore();
            assert_eq!(c.ls_top.get(), top0, "restore: arena top back at the outer frame");
            let span = top0 - deepest.get();
            let z = core::slice::from_raw_parts(deepest.get() as *const u64, span / 8);
            assert!(z.iter().all(|&w| w == 0), "restore: the abandoned range is zero again");
            // Without the restore, the outer frame's Drop cleans up instead (pop_slow).
            let r = enter_with(&mut a, || nest(2, ap, &deepest), &mut landed);
            assert!(r.is_none());
            assert_ne!(c.ls_top.get(), top0);
            drop(outer);
            let z = core::slice::from_raw_parts(deepest.get() as *const u64, (top0 - deepest.get()) / 8);
            assert!(z.iter().all(|&w| w == 0), "outer Drop: the abandoned range is zero again");
        }
    }

    #[test]
    fn snapshot_restores_every_callctx_field() {
        unsafe {
            let c = ctx();
            let want = Snapshot::take();
            let mut a = Anchor::new();
            let ap: *const Anchor = &a;
            let mut landed = 0;
            let r: Option<()> = enter_with(
                &mut a,
                || {
                    // Junk in every snapshotted field, as a deep callee could leave it.
                    c.xmm_in.set([1, 2, 3, 4]);
                    c.xmm_out.set(Some(5));
                    c.rax_in.set(6);
                    c.incoming_rsp.set((7, 8));
                    c.outgoing_rsp.set((9, 10));
                    c.xmm_in_set.set(!want.xmm_in_set);
                    c.eh_top.set(0x1238 as *mut _);
                    crate::emu::eh_set_emu_state(0x4440, want.emu_sp ^ 0x10);
                    jump(ap, 1)
                },
                &mut landed,
            );
            assert!(r.is_none());
            assert_ne!(Snapshot::take(), want, "the junk is visible before the restore");
            want.restore();
            assert_eq!(Snapshot::take(), want);
        }
    }

    #[test]
    fn snapshot_field_list_is_exhaustive() {
        // Compile-time: `snapshot_fields_exhaustive` + the `SNAPSHOT_COVERS_BYTES` assert.
        assert_eq!(core::mem::size_of::<CallCtx>(), SNAPSHOT_COVERS_BYTES);
        snapshot_fields_exhaustive(ctx());
    }
}
