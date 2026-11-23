//! C++ EH of lifted DLL functions with a `__CxxFrameHandler4` FuncInfo
//! (`tools/decomp_scale/dll_lift.py`, target/review/exec/g12/eh_lift_plan.md T1-T3).
//!
//! The generated code of such a function:
//! - [`EhMeta`] per lifted EH function (the image's `EH_META`): FuncInfo4, frame register /
//!   establisher offset (from the primary UNWIND_INFO) and the catch continuations.
//! - [`EhFrame`] pushed in its prolog: the frame's `EH_LIFTED` record in this thread's EH
//!   chain (`crate::eh_rt`), so the unwinder visits it with its arena establisher and the
//!   FuncInfo's handler; unlinked when the function returns (or before a tail jump).
//! - [`EhFrame::site`] before every call: the state IP of the frame's in-flight call.
//! - [`EhFrame::call`] / [`EhFrame::call_xmm`] for calls inside try ranges: `Err(Landing)`
//!   names the catch continuation, and the generated arm resumes at that block of the lifted
//!   body (its guest registers are the body's locals).
//! - [`EhFrame::setjmp`] in place of a call to the image's `__intrinsic_setjmp` import: the
//!   `_JUMP_BUFFER` names this frame (its establisher) and the call's return address, which
//!   is a continuation. vcruntime's `longjmp` is an `RtlUnwind` with `STATUS_LONGJMP` to that
//!   frame (win32-rt `unwind::do_unwind`), which lands here like a catch; the continuation's
//!   arm puts the buffer's nonvolatiles back into the body's locals ([`longjmp_regs`]).
//!   Every call of such a function is a catching call (`try_ranges` None).
//! - SEH functions (`__C_specific_handler` scope table, [`HANDLER_SEH`]): the same record; the
//!   `__except` JumpTargets are the continuations, their arms take rax (the exception code)
//!   from [`landed_rax`]. Filters and `__finally` funclets stay guest code, called by the
//!   handler with this frame's establisher.
//! - Faulting instructions inside an SEH `__except` scope (`seh_guards`): their accesses are
//!   [`EhFrame::ru32`] / [`EhFrame::wu32`] etc. over the probes of [`super::guarded`]; a fault
//!   raises STATUS_ACCESS_VIOLATION (continuable, Information = [write, address]) into this
//!   frame with ControlPc = the instruction ([`EhFrame::access_violation`]), and a landing
//!   resumes at the `__except` block like a catching call's.
//! - Catch funclets: lifted like any FH4 function (their FuncInfo4 has `isCatch`; FH4 finds
//!   the parent frame through the funclet's home slot, which the lifted prolog writes).

/// C++ EH facts of one lifted function (generated `EH_META` of each lifted image).
#[derive(Debug)]
pub struct EhMeta {
    /// Function VA at the pinned base.
    pub va: u64,
    /// RVA of the FuncInfo4 (`DISPATCHER_CONTEXT::HandlerData` points at it); 0 = none.
    pub funcinfo_rva: u32,
    /// [`HANDLER_FH4`], [`HANDLER_FH4_GS`], [`HANDLER_SEH`] or [`HANDLER_NONE`].
    pub handler: u8,
    /// [`SHAPE_EMPTY`], [`SHAPE_CLEANUP`] or [`SHAPE_TRY`].
    pub shape: u8,
    /// UNWIND_INFO frame register (x86 number); 0 = none, the establisher is rsp after the prolog.
    pub frame_reg: u8,
    /// Frame register: EstablisherFrame = reg - this. No frame register: EstablisherFrame =
    /// entry rsp - this (static rsp delta of the prolog's unwind codes).
    pub establisher_off: u32,
    /// Catch continuation VAs that are block starts of the lifted body.
    pub conts: &'static [u64],
}

/// No language handler: the record exists for `setjmp` continuations only.
pub const HANDLER_NONE: u8 = 0;
/// Handler `__CxxFrameHandler4` (import).
pub const HANDLER_FH4: u8 = 1;
/// Handler: local `__GSHandlerCheck_EH4` (GS cookie check, then `__CxxFrameHandler4`).
pub const HANDLER_FH4_GS: u8 = 2;
/// Handler `__C_specific_handler` / `__GSHandlerCheck_SEH` (SEH scope table: `funcinfo_rva`
/// is the scope table's RVA, the continuations are its `__except` JumpTargets).
pub const HANDLER_SEH: u8 = 3;
/// FuncInfo4 without unwind map or try map.
pub const SHAPE_EMPTY: u8 = 0;
/// Unwind map (destructors / cleanup funclets), no try map.
pub const SHAPE_CLEANUP: u8 = 1;
/// Try map (catch blocks).
pub const SHAPE_TRY: u8 = 2;

use crate::eh_rt::anchor::{self, Anchor, Snapshot};
use super::guarded;
use win32_rt::unwind::{Context, EhLink, LiftedInfo, CONTEXT_AW2_LONGJMP, EH_LIFTED, STATUS_ACCESS_VIOLATION};
use core::cell::Cell;
use core::mem::MaybeUninit;

impl EhMeta {
    /// EstablisherFrame for entry rsp `entry_rsp` (the return-address slot).
    pub fn establisher(&self, entry_rsp: u64) -> u64 {
        if self.frame_reg == 0 {
            return entry_rsp.wrapping_sub(self.establisher_off as u64);
        }
        match win32_rt::unwind::establisher_rsp_delta(self.va) {
            Some(d) => entry_rsp.wrapping_sub(d),
            None => {
                log::error!(
                    target: "win32_rt::seh",
                    "dll_lift EH: lifted fn 0x{:x}: no usable UNWIND_INFO for its frame-register establisher",
                    self.va
                );
                0
            }
        }
    }
}

/// `EH_LIFTED` chain record of one live lifted function with a FuncInfo (pushed by its
/// prolog; see `crate::eh_rt::chain`).
#[repr(C)]
pub struct EhFrame {
    link: EhLink,
    meta: &'static EhMeta,
    entry_rsp: u64,
    /// ControlPc: the VA of the in-flight call.
    state_ip: Cell<u64>,
    /// The in-flight catching call ([`EhFrame::call`]); null = none.
    catch: Cell<*mut Catch>,
}

/// A catching call's landing state.
struct Catch {
    anchor: MaybeUninit<Anchor>,
    snap: MaybeUninit<Snapshot>,
    cont: u64,
}

/// Unlinks an [`EhFrame`] when its function returns.
pub struct EhFrameGuard(*mut EhLink);

impl Drop for EhFrameGuard {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { crate::eh_rt::chain::pop(self.0) }
    }
}

impl EhFrame {
    /// The record of a lifted fn with FuncInfo `meta`, entered with rsp `entry_rsp`.
    #[inline(always)]
    pub fn new(meta: &'static EhMeta, entry_rsp: u64) -> Self {
        Self {
            link: EhLink { next: core::ptr::null_mut(), kind: EH_LIFTED, flags: 0 },
            meta,
            entry_rsp,
            state_ip: Cell::new(meta.va),
            catch: Cell::new(core::ptr::null_mut()),
        }
    }

    /// Link as this thread's innermost record until the guard drops. `self` must not move.
    #[inline(always)]
    pub unsafe fn push(&mut self) -> EhFrameGuard {
        crate::eh_rt::chain::ensure_hooks();
        let p = &mut self.link as *mut EhLink;
        unsafe { crate::eh_rt::chain::push(p) };
        EhFrameGuard(p)
    }

    /// State IP of the in-flight call: the call instruction's VA.
    #[inline(always)]
    pub fn site(&self, site_va: u64) {
        self.state_ip.set(site_va);
    }

    /// Run `f` (a call inside a try range at `site_va`) so that a catch in this frame lands
    /// here: `Err(Landing)` with the catch continuation, after the per-thread state of the
    /// call is restored.
    #[inline(always)]
    pub unsafe fn catching<R, F: FnOnce() -> R>(&self, site_va: u64, f: F) -> Result<R, Landing> {
        self.state_ip.set(site_va);
        let mut k = Catch { anchor: MaybeUninit::uninit(), snap: MaybeUninit::uninit(), cont: 0 };
        let kp: *mut Catch = &mut k;
        unsafe {
            (*kp).snap.write(Snapshot::take());
            let prev = self.catch.replace(kp);
            let mut landed = 0;
            let r = anchor::enter_with((*kp).anchor.as_mut_ptr(), f, &mut landed);
            self.catch.set(prev);
            match r {
                Some(v) => Ok(v),
                None => {
                    (*kp).snap.assume_init_ref().restore();
                    Err(Landing { cont_va: core::ptr::read_volatile(&(*kp).cont) })
                }
            }
        }
    }

    /// A call inside a try range (`exe::call_va8` protocol); see [`EhFrame::catching`].
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn call(
        &self,
        site_va: u64,
        target: u64,
        rcx: u64,
        rdx: u64,
        r8: u64,
        r9: u64,
        arg5: u64,
        arg6: u64,
        arg7: u64,
        arg8: u64,
    ) -> Result<u64, Landing> {
        unsafe { self.catching(site_va, || crate::exe::call_va8(target, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)) }
    }

    /// [`EhFrame::call`] with XMM arguments / result (`simd_ops::call_va8_xmm` protocol).
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn call_xmm(
        &self,
        site_va: u64,
        target: u64,
        rcx: u64,
        rdx: u64,
        r8: u64,
        r9: u64,
        arg5: u64,
        arg6: u64,
        arg7: u64,
        arg8: u64,
        xmm: [u128; 4],
    ) -> Result<(u64, u128), Landing> {
        unsafe {
            self.catching(site_va, || {
                crate::simd_ops::call_va8_xmm(target, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, xmm)
            })
        }
    }

    /// `call __intrinsic_setjmp` at `site_va` (vcruntime: `rcx` = `_JUMP_BUFFER`, `rdx` = the
    /// caller's frame; returns 0): fill the buffer at `buf` as the guest helper would, with
    /// rsp / rip of the caller after the return (`rsp` is the body's rsp at the call,
    /// `next_va` the return address), its nonvolatiles `nv` = rbx, rbp, rsi, rdi, r12..r15 and
    /// `xmm` = xmm6..xmm15 (0 for registers the body never uses: no lifted state to restore).
    /// `Frame` is this record's establisher, the frame the longjmp unwind targets and finds in
    /// the chain; the compiler's `rdx` is the same value (rsp after the prolog / frame
    /// register base), a differing one is logged.
    #[inline(never)]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn setjmp(&self, site_va: u64, buf: u64, frame: u64, next_va: u64, rsp: u64, nv: [u64; 8], xmm: [u128; 10]) -> u64 {
        self.state_ip.set(site_va);
        let est = self.meta.establisher(self.entry_rsp);
        if frame != est {
            log::warn!(
                target: "win32_rt::seh",
                "dll_lift setjmp at 0x{site_va:x}: frame argument 0x{frame:x} != establisher 0x{est:x}; the buffer names the establisher"
            );
        }
        let q = |i: u64, v: u64| unsafe { core::ptr::write_unaligned((buf + 8 * i) as *mut u64, v) };
        q(0, est);
        q(1, nv[0]);
        q(2, rsp);
        for (i, v) in nv[1..].iter().enumerate() {
            q(3 + i as u64, *v);
        }
        q(10, next_va);
        unsafe {
            core::ptr::write_unaligned((buf + 88) as *mut u32, 0x1F80); // MxCsr
            core::ptr::write_unaligned((buf + 92) as *mut u16, 0x037F); // FpCsr: the emulator's fnstcw value
            core::ptr::write_unaligned((buf + 94) as *mut u16, 0);
            for (k, v) in xmm.iter().enumerate() {
                core::ptr::write_unaligned((buf + 96 + 16 * k as u64) as *mut u128, *v);
            }
        }
        0
    }
}

/// Guarded accesses of an `__except` scope instruction (gen_subsystems `seh_guard_lines`):
/// `$rd(site_va, a)` / `$wr(site_va, a, v)` access like `simd_ops::ru*` / `wu*`; a fault is
/// [`EhFrame::access_violation`] at `site_va`, retried when the exception is continued.
macro_rules! guarded_access {
    ($rd:ident, $wr:ident, $ld:ident, $st:ident, $t:ty, $bytes:expr) => {
        #[inline(always)]
        pub unsafe fn $rd(&self, site_va: u64, a: u64) -> Result<$t, Landing> {
            match unsafe { guarded::$ld(a) } {
                Some(v) => Ok(v),
                None => unsafe { self.fault_load(site_va, a, $bytes) }.map(|v| v as $t),
            }
        }
        #[inline(always)]
        pub unsafe fn $wr(&self, site_va: u64, a: u64, v: $t) -> Result<(), Landing> {
            if unsafe { guarded::$st(a, v) } {
                Ok(())
            } else {
                unsafe { self.fault_store(site_va, a, v as u64, $bytes) }
            }
        }
    };
}

impl EhFrame {
    guarded_access!(ru8, wu8, ld8, st8, u8, 1);
    guarded_access!(ru16, wu16, ld16, st16, u16, 2);
    guarded_access!(ru32, wu32, ld32, st32, u32, 4);
    guarded_access!(ru64, wu64, ld64, st64, u64, 8);

    #[cold]
    #[inline(never)]
    unsafe fn fault_load(&self, site_va: u64, a: u64, bytes: u8) -> Result<u64, Landing> {
        loop {
            unsafe { self.access_violation(site_va, a, false)? };
            if let Some(v) = unsafe { guarded::load(bytes, a) } {
                return Ok(v);
            }
        }
    }

    #[cold]
    #[inline(never)]
    unsafe fn fault_store(&self, site_va: u64, a: u64, v: u64, bytes: u8) -> Result<(), Landing> {
        loop {
            unsafe { self.access_violation(site_va, a, true)? };
            if unsafe { guarded::store(bytes, a, v) } {
                return Ok(());
            }
        }
    }

    /// The access of the instruction at `site_va` to `a` faulted: raise
    /// STATUS_ACCESS_VIOLATION (continuable; Information = [write, address], as the kernel's)
    /// with this frame's ControlPc = `site_va`, inside its `__except` scope. `Err(Landing)`:
    /// the `__except` body claimed it; `Ok`: a filter continued it (the access is retried).
    /// ExceptionAddress is the host raise site, not `site_va` (filters of the lifted
    /// `seh_guards` fns read the code only).
    #[cold]
    #[inline(never)]
    pub unsafe fn access_violation(&self, site_va: u64, a: u64, write: bool) -> Result<(), Landing> {
        GUARD_FAULTS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        note_seh_fault(a);
        let info = [write as u64, a];
        unsafe {
            self.catching(site_va, || {
                win32_rt::unwind::raise_exception_raw(
                    STATUS_ACCESS_VIOLATION as u64,
                    0,
                    2,
                    info.as_ptr() as u64,
                );
            })
        }
    }
}

/// Guarded accesses that faulted ([`EhFrame::access_violation`]), process-wide.
pub static GUARD_FAULTS: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

/// `fn(address)` called at every access violation raised for a guarded lifted access or a precise
/// guest fault (`emu::guest_fault_dispatch`), before the dispatch; 0 = none. Set only by the
/// dll_lift difftest child, which marks the run (`SEHF_TAG`).
pub static SEH_FAULT_HOOK: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

/// See [`SEH_FAULT_HOOK`].
#[inline]
pub fn note_seh_fault(addr: u64) {
    let h = SEH_FAULT_HOOK.load(core::sync::atomic::Ordering::Relaxed);
    if h != 0 {
        let f: fn(u64) = unsafe { core::mem::transmute(h) };
        f(addr);
    }
}

/// The registers a `longjmp` restored for the landing of a lifted `setjmp` continuation
/// (from the `_JUMP_BUFFER`, win32-rt `CONTEXT_AW2_LONGJMP`): context register numbering,
/// `gpr[0]` = rax = the longjmp value; `xmm[k]` = xmm(6+k).
#[derive(Debug, Clone, Copy)]
pub struct LongjmpRegs {
    pub gpr: [u64; 16],
    pub xmm: [u128; 10],
}

std::thread_local! {
    static LONGJMP: Cell<Option<LongjmpRegs>> = const { Cell::new(None) };
    /// rax of the last landing's context (the unwind's ReturnValue).
    static LANDED_RAX: Cell<u64> = const { Cell::new(0) };
}

/// For the landing arm of an SEH `__except` continuation: rax at the JumpTarget, the
/// `RtlUnwindEx` ReturnValue (`__C_specific_handler` passes the exception code).
#[inline(never)]
pub fn landed_rax() -> u64 {
    LANDED_RAX.with(|l| l.get())
}

/// For the landing arm of a `setjmp` continuation: the registers of the longjmp that landed.
#[cold]
#[inline(never)]
pub fn longjmp_regs() -> LongjmpRegs {
    match LONGJMP.with(|l| l.take()) {
        Some(r) => r,
        None => {
            log::error!(target: "win32_rt::seh", "dll_lift setjmp continuation reached without a longjmp landing");
            std::process::abort()
        }
    }
}

/// `EhHooks::lifted`.
pub(crate) unsafe fn lifted_info(r: *mut EhLink) -> LiftedInfo {
    let f = r as *const EhFrame;
    unsafe {
        LiftedInfo {
            fn_va: (*f).meta.va,
            establisher: (*f).meta.establisher((*f).entry_rsp),
            state_ip: (*f).state_ip.get(),
        }
    }
}

/// `EhHooks::land` for an `EH_LIFTED` record: resume the lifted body at continuation `ctx.rip`
/// (its guest registers are the body's locals, as at the in-flight call).
pub(crate) unsafe fn land(r: *mut EhLink, ctx: &Context) -> ! {
    let f = r as *const EhFrame;
    unsafe {
        let k = (*f).catch.get();
        if k.is_null() {
            log::error!(
                target: "win32_rt::seh",
                "dll_lift EH: landing at 0x{:x} in {}: no catching call in flight (call at 0x{:x} is outside every try range)",
                ctx.rip,
                describe(r),
                (*f).state_ip.get()
            );
            std::process::abort();
        }
        (*k).cont = ctx.rip;
        LANDED_RAX.with(|l| l.set(ctx.gpr[0]));
        if ctx.context_flags & CONTEXT_AW2_LONGJMP != 0 {
            let mut xmm = [0u128; 10];
            for (i, x) in xmm.iter_mut().enumerate() {
                *x = u128::from_le_bytes(ctx.xmm(6 + i));
            }
            LONGJMP.with(|l| l.set(Some(LongjmpRegs { gpr: ctx.gpr, xmm })));
        }
        anchor::jump((*k).anchor.as_ptr(), 1)
    }
}

/// `EhHooks::describe` for an `EH_LIFTED` record.
pub(crate) fn describe(r: *mut EhLink) -> String {
    let f = r as *const EhFrame;
    unsafe {
        format!(
            "lifted EH frame {} (call at 0x{:x})",
            win32_rt::symbolize((*f).meta.va as usize),
            (*f).state_ip.get()
        )
    }
}

/// A catch in this frame claimed an exception: resume the lifted body at `cont_va`.
#[derive(Debug, Clone, Copy)]
pub struct Landing {
    pub cont_va: u64,
}

impl Landing {
    /// A landing whose continuation is no block of the lifted body of `fn_va`.
    #[cold]
    #[inline(never)]
    pub fn unknown(&self, fn_va: u64) -> ! {
        log::error!(
            "dll_lift EH: landing at 0x{:x} in lifted fn 0x{fn_va:x}: not a continuation block of its body",
            self.cont_va
        );
        std::process::abort()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eh_rt::chain::{self, Innermost};
    use crate::simd_ops::{init_lifted_stack, LiftedFrame};
    use core::cell::RefCell;

    static META: EhMeta = EhMeta {
        va: 0x1_4000_9000,
        funcinfo_rva: 0,
        handler: HANDLER_FH4,
        shape: SHAPE_CLEANUP,
        frame_reg: 0,
        establisher_off: 0x48,
        conts: &[],
    };

    std::thread_local! {
        /// (entry rsp, innermost before the site, innermost after the site, depth).
        static OBS: RefCell<Vec<(u64, Option<LiftedInfo>, Option<LiftedInfo>, usize)>> =
            const { RefCell::new(Vec::new()) };
    }

    fn lifted_top() -> Option<LiftedInfo> {
        match chain::innermost() {
            Innermost::Lifted(li) => Some(li),
            _ => None,
        }
    }

    /// Generated-code shape of a lifted fn with an `EhMeta`: the record is pushed right after
    /// the frame is set up, and every call site stores its state IP.
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn lifted_with_meta(_rcx: u64, _rdx: u64, _r8: u64, _r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
        let mut stack = LiftedFrame::<256, 16>::new();
        let rsp = unsafe { init_lifted_stack(&mut stack, a5, a6, a7, a8) };
        let mut eh = EhFrame::new(&META, rsp);
        let _eh = unsafe { eh.push() };
        let before = lifted_top();
        eh.site(0x1_4000_9010);
        let after = lifted_top();
        OBS.with(|o| o.borrow_mut().push((rsp, before, after, chain::depth())));
        // A nested lifted fn without FuncInfo pushes nothing.
        unsafe { lifted_plain(0, 0, 0, 0, 0, 0, 0, 0) };
        rsp
    }

    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn lifted_plain(_rcx: u64, _rdx: u64, _r8: u64, _r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
        let mut stack = LiftedFrame::<256, 16>::new();
        let rsp = unsafe { init_lifted_stack(&mut stack, a5, a6, a7, a8) };
        OBS.with(|o| o.borrow_mut().push((rsp, None, lifted_top(), chain::depth())));
        rsp
    }

    #[test]
    fn lifted_eh_record_only_for_eh_meta_fns() {
        OBS.with(|o| o.borrow_mut().clear());
        assert_eq!(chain::depth(), 0);
        unsafe { lifted_with_meta(0, 0, 0, 0, 0, 0, 0, 0) };
        assert_eq!(chain::depth(), 0, "popped at return");
        let obs = OBS.with(|o| o.borrow().clone());
        assert_eq!(obs.len(), 2);
        let (rsp, before, after, depth) = obs[0];
        assert_eq!(depth, 1);
        let before = before.expect("innermost is the lifted record");
        assert_eq!(before.fn_va, META.va);
        assert_eq!(before.establisher, rsp - 0x48, "establisher = entry rsp - establisher_off");
        assert_eq!(before.state_ip, META.va, "state IP before the first site = fn entry");
        let base = crate::callctx::ctx().ls_base.get() as u64;
        assert!(
            base != 0 && before.establisher >= base && before.establisher < base + crate::lstack::ARENA_BYTES as u64,
            "establisher in the lstack arena"
        );
        assert_eq!(after.expect("still innermost").state_ip, 0x1_4000_9010, "site() writes state_ip");
        // The nested plain lifted fn: same depth, the record above is still the EH frame's.
        let (_, _, plain_top, plain_depth) = obs[1];
        assert_eq!(plain_depth, 1);
        assert_eq!(plain_top.map(|l| l.fn_va), Some(META.va));
    }
}
