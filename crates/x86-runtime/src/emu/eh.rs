//! EH chain records of emulated regions (`crate::eh_rt`, target/review/exec/g12/eh_lift_plan.md
//! §4.2): one [`EmuEntry`] per emulator run, linked while the run is live.
//!
//! While the region's guest code is suspended in a host call ([`callout`]), the record knows
//! the exact guest state of that call (the calling guest frame's CONTEXT: return address,
//! rsp, every register) and owns the anchor a landing in this region jumps to. A landing is
//! only possible then: an exception dispatched in a newer frame is caught by a guest frame
//! of this region, the unwinder hands the target context to [`land`], and the host call
//! "returns" into the interpreter with that guest state loaded.
use super::{Cpu, TLS_EMU};
use crate::callctx::ctx;
use crate::eh_rt::anchor::{self, Anchor, Snapshot};
use win32_rt::unwind::{Context, EhLink, CONTEXT_FULL, EH_EMU_ENTRY, EH_F_OPAQUE};
use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

/// Offset of `xmm0` in `CONTEXT::FltSave` (XMM_SAVE_AREA32).
const FLT_XMM0: usize = 0xA0;
/// Offset of `MxCsr` in `CONTEXT::FltSave`.
const FLT_MXCSR: usize = 0x18;

/// Guest state a landing loads into the region's CPU.
struct Landing {
    gpr: [u64; 16],
    xmm: [u128; 16],
    rip: u64,
    eflags: u32,
    mxcsr: u32,
}

/// `EH_EMU_ENTRY` record (see the module doc). Lives in the emulator run's frame.
#[repr(C)]
pub(crate) struct EmuEntry {
    link: EhLink,
    cpu: *const Cpu,
    entry_ip: u64,
    /// Host function of the in-flight host call; 0 = the region is running guest code.
    callout_fn: Cell<usize>,
    anchor: UnsafeCell<MaybeUninit<Anchor>>,
    snap: UnsafeCell<MaybeUninit<Snapshot>>,
    landing: UnsafeCell<MaybeUninit<Landing>>,
}

/// Unlinks an [`EmuEntry`] when its run ends.
pub(crate) struct EmuEntryGuard(*mut EhLink);

impl Drop for EmuEntryGuard {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { crate::eh_rt::chain::pop(self.0) }
    }
}

impl EmuEntry {
    #[inline(always)]
    pub(super) fn new(cpu: *const Cpu, entry_ip: u64, opaque: bool) -> Self {
        Self {
            link: EhLink {
                next: core::ptr::null_mut(),
                kind: EH_EMU_ENTRY,
                flags: if opaque { EH_F_OPAQUE } else { 0 },
            },
            cpu,
            entry_ip,
            callout_fn: Cell::new(0),
            anchor: UnsafeCell::new(MaybeUninit::uninit()),
            snap: UnsafeCell::new(MaybeUninit::uninit()),
            landing: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Link as this thread's innermost record until the guard drops. `self` must not move.
    #[inline(always)]
    pub(crate) unsafe fn push(&mut self) -> EmuEntryGuard {
        crate::eh_rt::chain::ensure_hooks();
        let p = &mut self.link as *mut EhLink;
        unsafe { crate::eh_rt::chain::push(p) };
        EmuEntryGuard(p)
    }
}

/// Run host call `f` of the region whose CPU is `cpu` as a callout of its record: a landing
/// in this region returns `None` with the landing's guest state loaded into `cpu` and the
/// per-thread state of the call restored. `emu_sp` = the guest stack allocation pointer the
/// call starts with. Without this region's record innermost (never expected), a plain call.
#[inline(always)]
pub(super) unsafe fn callout<F: FnOnce() -> (u64, u128)>(
    cpu: &mut Cpu,
    host_fn: usize,
    emu_sp: u64,
    f: F,
) -> Option<(u64, u128)> {
    let c = ctx();
    let e = c.eh_top.get() as *mut EmuEntry;
    let cp = cpu as *mut Cpu as *const Cpu;
    unsafe {
        if e.is_null() || (*e).link.kind != EH_EMU_ENTRY || (*e).cpu != cp {
            return Some(f());
        }
        (*e).callout_fn.set(host_fn);
        (*(*e).snap.get()).write(Snapshot::take_in(c, cp as usize, emu_sp));
        let mut landed = 0;
        let r = anchor::enter_with((*(*e).anchor.get()).as_mut_ptr(), f, &mut landed);
        (*e).callout_fn.set(0);
        if r.is_none() {
            (*(*e).snap.get()).assume_init_ref().restore();
            let l = (*(*e).landing.get()).assume_init_ref();
            cpu.gpr = l.gpr;
            cpu.xmm = l.xmm;
            cpu.clear_all_ymm_hi();
            cpu.rip = l.rip;
            cpu.set_rflags(l.eflags);
            if l.mxcsr != 0 {
                let m = l.mxcsr & 0xFFFF;
                cpu.jx.mxcsr = m;
                win32_rt::ucrt::runtime::set_guest_mxcsr(m);
                super::apply_guest_mxcsr_hw(m);
            }
            cpu.last_bits = [64; 16];
        }
        r
    }
}

/// `EhHooks::callout`: the guest state of the region's in-flight host call.
pub(crate) unsafe fn callout_context(r: *mut EhLink, out: &mut Context) -> bool {
    let e = r as *const EmuEntry;
    unsafe {
        if (*e).callout_fn.get() == 0 {
            return false;
        }
        let cpu = (*e).cpu;
        let gpr = core::ptr::read_volatile(&(*cpu).gpr);
        let xmm = core::ptr::read_volatile(&(*cpu).xmm);
        let rip = core::ptr::read_volatile(&(*cpu).rip);
        let eflags = (*cpu).rflags();
        *out = Context::zeroed();
        out.context_flags = CONTEXT_FULL;
        out.gpr = gpr;
        out.rip = rip;
        out.eflags = eflags;
        let mx = (*cpu).jx.mxcsr & 0xFFFF;
        out.mx_csr = mx;
        out.seg_cs = 0x33;
        out.seg_ss = 0x2B;
        out.seg_ds = 0x2B;
        out.seg_es = 0x2B;
        out.seg_gs = 0x2B;
        out.seg_fs = 0x53;
        out.flt_save[FLT_MXCSR..FLT_MXCSR + 4].copy_from_slice(&mx.to_le_bytes());
        for (i, x) in xmm.iter().enumerate() {
            out.flt_save[FLT_XMM0 + 16 * i..FLT_XMM0 + 16 * (i + 1)].copy_from_slice(&x.to_le_bytes());
        }
        true
    }
}

/// `EhHooks::callout_fn`.
pub(crate) unsafe fn callout_fn(r: *mut EhLink) -> usize {
    unsafe { (*(r as *const EmuEntry)).callout_fn.get() }
}

/// `EhHooks::land` for an `EH_EMU_ENTRY`: resume the region at guest state `ctx`.
pub(crate) unsafe fn land(r: *mut EhLink, ctx: &Context) -> ! {
    let e = r as *mut EmuEntry;
    unsafe {
        if (*e).callout_fn.get() == 0 {
            log::error!(
                target: "win32_rt::seh",
                "EH landing at 0x{:x} in {}: the region is not suspended in a host call",
                ctx.rip,
                describe(r)
            );
            std::process::abort();
        }
        let mut xmm = [0u128; 16];
        for (i, x) in xmm.iter_mut().enumerate() {
            let mut b = [0u8; 16];
            b.copy_from_slice(&ctx.flt_save[FLT_XMM0 + 16 * i..FLT_XMM0 + 16 * (i + 1)]);
            *x = u128::from_le_bytes(b);
        }
        let flt_mxcsr = u32::from_le_bytes(
            ctx.flt_save[FLT_MXCSR..FLT_MXCSR + 4].try_into().unwrap_or([0; 4]),
        );
        let mxcsr = if ctx.mx_csr != 0 { ctx.mx_csr } else { flt_mxcsr };
        #[cfg(target_arch = "aarch64")]
        super::aot::record_runtime_callback(ctx.rip, super::aot::CallbackKind::EhContinuation);
        (*(*e).landing.get()).write(Landing { gpr: ctx.gpr, xmm, rip: ctx.rip, eflags: ctx.eflags, mxcsr });
        anchor::jump((*(*e).anchor.get()).as_ptr(), 1)
    }
}

/// `EhHooks::describe` for an `EH_EMU_ENTRY`.
pub(crate) fn describe(r: *mut EhLink) -> String {
    let e = r as *const EmuEntry;
    unsafe {
        let f = (*e).callout_fn.get();
        format!(
            "emulated region entered at {}{}{}",
            win32_rt::symbolize((*e).entry_ip as usize),
            if (*e).link.flags & EH_F_OPAQUE != 0 { " by a Rust API" } else { "" },
            if f != 0 { format!(", in a host call to {}", win32_rt::symbolize(f)) } else { String::new() }
        )
    }
}

/// This thread's emulator stack `[lo, hi)` (empty without emulator state).
pub fn stack_range() -> (u64, u64) {
    TLS_EMU.with(|cell| unsafe {
        match (*cell.get()).as_ref() {
            Some(s) => {
                let lo = s._stack.as_ptr() as u64;
                (lo, lo + (s._stack.len() as u64) * 8)
            }
            None => (0, 0),
        }
    })
}

/// Force the translator on/off for this thread (eh_rt tests; see `jit::force`).
#[cfg(all(test, target_arch = "aarch64"))]
pub(crate) fn test_force_jit(mode: u8) {
    super::jit::force(mode);
}

#[cfg(all(test, target_arch = "aarch64"))]
mod tests {
    use super::super::jit;
    use crate::eh_rt::chain::{self, Innermost};
    use crate::eh_rt::testutil::{native_va, place};
    use win32_rt::unwind::{Context, EH_EMU_ENTRY, EH_F_OPAQUE};
    use core::cell::RefCell;
    use iced_x86::code_asm::*;

    std::thread_local! {
        /// (depth, innermost kind, innermost flags) seen by each `probe` call.
        static SEEN: RefCell<Vec<(usize, u32, u32)>> = const { RefCell::new(Vec::new()) };
        static CALLOUT: RefCell<Option<Box<Context>>> = const { RefCell::new(None) };
    }

    /// Records the chain state; with `a != 0` calls guest code `a` (a nested region through
    /// the lifted-code bridge).
    unsafe extern "C" fn probe(a: u64) -> u64 {
        let head = crate::callctx::ctx().eh_top.get();
        let (k, f) = if head.is_null() { (0, 0) } else { unsafe { ((*head).kind, (*head).flags) } };
        SEEN.with(|s| s.borrow_mut().push((chain::depth(), k, f)));
        if a != 0 {
            unsafe { crate::exe::call_va8(a, 0, 0, 0, 0, 0, 0, 0, 0) };
        }
        7
    }

    /// Guest fn: `probe(rcx)`, returning its result.
    fn call_probe_blob(probe_va: u64) -> u64 {
        let mut a = CodeAssembler::new(64).unwrap();
        a.sub(rsp, 0x28).unwrap();
        a.mov(rax, probe_va).unwrap();
        a.call(rax).unwrap();
        a.add(rsp, 0x28).unwrap();
        a.ret().unwrap();
        place(a, 128).0
    }

    #[test]
    fn emu_entry_record_is_balanced() {
        let pv = native_va(probe as *const () as usize);
        let g = call_probe_blob(pv);
        for mode in [2u8, 1] {
            jit::force(mode);
            SEEN.with(|s| s.borrow_mut().clear());
            assert_eq!(chain::depth(), 0);
            unsafe {
                // A Rust API entering the emulator directly: one opaque region.
                assert_eq!(super::super::emu_run_full(g, [0; 8], [0; 4], [0; 8]).0, 7);
                assert_eq!(chain::depth(), 0, "popped after the run");
                // Lifted code through the bridge: transparent.
                assert_eq!(crate::exe::call_va8(g, 0, 0, 0, 0, 0, 0, 0, 0), 7);
                assert_eq!(chain::depth(), 0);
                // guest -> host -> guest: depth 2 inside the inner region.
                super::super::emu_run_full(g, [g, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
                assert_eq!(chain::depth(), 0);
            }
            let seen = SEEN.with(|s| s.borrow().clone());
            assert_eq!(
                seen,
                vec![
                    (1, EH_EMU_ENTRY, EH_F_OPAQUE),
                    (1, EH_EMU_ENTRY, 0),
                    (1, EH_EMU_ENTRY, EH_F_OPAQUE),
                    (2, EH_EMU_ENTRY, 0),
                ],
                "mode {mode}"
            );
        }
        jit::force(0);
        assert!(crate::callctx::ctx().eh_top.get().is_null());
    }

    unsafe extern "C" fn callout_probe() -> u64 {
        let c = match chain::innermost() {
            Innermost::Callout(c) => Some(c),
            other => panic!("innermost record in a host call: {other:?}"),
        };
        CALLOUT.with(|s| *s.borrow_mut() = c);
        0
    }

    const NV: [(AsmRegister64, usize, u64); 8] = [
        (rbx, 3, 0x0B0B_0000_0000_0003),
        (rbp, 5, 0x0B0B_0000_0000_0005),
        (rsi, 6, 0x0B0B_0000_0000_0006),
        (rdi, 7, 0x0B0B_0000_0000_0007),
        (r12, 12, 0x0B0B_0000_0000_000C),
        (r13, 13, 0x0B0B_0000_0000_000D),
        (r14, 14, 0x0B0B_0000_0000_000E),
        (r15, 15, 0x0B0B_0000_0000_000F),
    ];

    fn xmm_reg(i: usize) -> AsmRegisterXmm {
        [xmm6, xmm7, xmm8, xmm9, xmm10, xmm11, xmm12, xmm13, xmm14, xmm15][i - 6]
    }

    fn xmm_sentinel(i: usize) -> u64 {
        0x5E5E_0000_0000_0000 | i as u64
    }

    /// The record of a region in a host call holds the calling guest frame's exact state:
    /// rip = the return VA, rsp = the guest rsp at (and after) the call, nonvolatile GPRs and
    /// xmm6..15 as the guest left them.
    #[test]
    fn callout_record_holds_guest_state() {
        let pv = native_va(callout_probe as *const () as usize);
        let mut a = CodeAssembler::new(64).unwrap();
        let mut after = a.create_label();
        a.sub(rsp, 0x28).unwrap();
        for (r, _, v) in NV {
            a.mov(r, v).unwrap();
        }
        for i in 6..16 {
            a.mov(rax, xmm_sentinel(i)).unwrap();
            a.movq(xmm_reg(i), rax).unwrap();
        }
        a.mov(qword_ptr(rcx), rsp).unwrap();
        a.mov(rax, pv).unwrap();
        a.call(rax).unwrap();
        a.set_label(&mut after).unwrap();
        a.add(rsp, 0x28).unwrap();
        a.ret().unwrap();
        let (g, res) = place(a, 512);
        let ret_va = res.label_ip(&after).unwrap();
        for mode in [2u8, 1] {
            jit::force(mode);
            CALLOUT.with(|s| *s.borrow_mut() = None);
            let mut rsp_out = 0u64;
            unsafe {
                super::super::emu_run_full(g, [&mut rsp_out as *mut u64 as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
            }
            let c = CALLOUT.with(|s| s.borrow_mut().take()).expect("probe ran");
            assert_eq!(c.rip, ret_va, "mode {mode}: rip = return VA");
            assert_eq!(c.gpr[4], rsp_out, "mode {mode}: rsp = guest rsp at the call");
            for (_, i, v) in NV {
                assert_eq!(c.gpr[i], v, "mode {mode}: gpr {i}");
            }
            for i in 6..16 {
                let lo = u64::from_le_bytes(c.flt_save[0xA0 + 16 * i..0xA0 + 16 * i + 8].try_into().unwrap());
                assert_eq!(lo, xmm_sentinel(i), "mode {mode}: xmm{i}");
            }
            assert_eq!(chain::depth(), 0);
        }
        jit::force(0);
    }

    /// 1e5 mixed calls (direct runs, bridge runs, nested runs) leave the chain empty: the
    /// no-throw path pushes and pops exactly once per run.
    #[test]
    fn records_balanced_after_many_calls() {
        let pv = native_va(probe as *const () as usize);
        let g = call_probe_blob(pv);
        for i in 0..100_000u64 {
            unsafe {
                match i % 3 {
                    0 => {
                        super::super::emu_run_full(g, [0; 8], [0; 4], [0; 8]);
                    }
                    1 => {
                        crate::exe::call_va8(g, 0, 0, 0, 0, 0, 0, 0, 0);
                    }
                    _ => {
                        super::super::emu_run_full(g, [g, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
                    }
                }
            }
            if i % 1024 == 0 {
                SEEN.with(|s| s.borrow_mut().clear());
            }
        }
        assert_eq!(chain::depth(), 0);
        assert!(crate::callctx::ctx().eh_top.get().is_null());
    }
}

/// L2: the raise context of guest code calling `RaiseException` / `RtlCaptureContext`.
#[cfg(all(test, target_arch = "aarch64"))]
mod raise_tests {
    use super::super::jit;
    use crate::eh_rt::chain;
    use crate::eh_rt::testutil::{native_va, place};
    use win32_rt::unwind::{Context, ExceptionRecord};
    use core::ffi::c_void;
    use iced_x86::code_asm::*;
    use std::sync::Mutex;

    const CODE: u64 = 0xE0AA_0001;

    /// (exception record, context) the test VEH saw for [`CODE`].
    static SEEN: Mutex<Option<(u32, u64, Box<Context>)>> = Mutex::new(None);

    unsafe extern "win64" fn veh(p: *mut [*mut c_void; 2]) -> i32 {
        unsafe {
            let rec = &*((*p)[0] as *const ExceptionRecord);
            if rec.code as u64 != CODE {
                return 0; // EXCEPTION_CONTINUE_SEARCH
            }
            let ctx = &*((*p)[1] as *const Context);
            *SEEN.lock().unwrap() = Some((rec.code, rec.address as u64, Box::new(*ctx)));
            -1 // EXCEPTION_CONTINUE_EXECUTION
        }
    }

    const NV: [(AsmRegister64, usize, u64); 8] = [
        (rbx, 3, 0x0C0C_0000_0000_0003),
        (rbp, 5, 0x0C0C_0000_0000_0005),
        (rsi, 6, 0x0C0C_0000_0000_0006),
        (rdi, 7, 0x0C0C_0000_0000_0007),
        (r12, 12, 0x0C0C_0000_0000_000C),
        (r13, 13, 0x0C0C_0000_0000_000D),
        (r14, 14, 0x0C0C_0000_0000_000E),
        (r15, 15, 0x0C0C_0000_0000_000F),
    ];

    fn xmm_reg(i: usize) -> AsmRegisterXmm {
        [xmm6, xmm7, xmm8, xmm9, xmm10, xmm11, xmm12, xmm13, xmm14, xmm15][i - 6]
    }

    fn xmm_sentinel(i: usize) -> u64 {
        0x6E6E_0000_0000_0000 | i as u64
    }

    /// Guest fn(buf): set nonvolatiles, `[buf] = rsp`, call `target(CODE, 0, 0, 0)` (for
    /// `RtlCaptureContext`: `target(buf + 0x100)`), then store rbx..r15 at `buf + 8..` and the
    /// low halves of xmm6..15 at `buf + 0x48..`. Returns (blob, return VA of the call).
    fn blob(target: u64, capture: bool) -> (u64, u64) {
        let mut a = CodeAssembler::new(64).unwrap();
        let mut after = a.create_label();
        a.sub(rsp, 0x28).unwrap();
        a.mov(qword_ptr(rsp + 0x30), rcx).unwrap();
        for (r, _, v) in NV {
            a.mov(r, v).unwrap();
        }
        for i in 6..16 {
            a.mov(rax, xmm_sentinel(i)).unwrap();
            a.movq(xmm_reg(i), rax).unwrap();
        }
        a.mov(qword_ptr(rcx), rsp).unwrap();
        if capture {
            a.add(rcx, 0x100).unwrap();
        } else {
            a.mov(ecx, CODE as u32).unwrap();
            a.xor(edx, edx).unwrap();
            a.xor(r8d, r8d).unwrap();
            a.xor(r9d, r9d).unwrap();
        }
        a.mov(rax, target).unwrap();
        a.call(rax).unwrap();
        a.set_label(&mut after).unwrap();
        a.mov(rax, qword_ptr(rsp + 0x30)).unwrap();
        for (k, (r, _, _)) in NV.iter().enumerate() {
            a.mov(qword_ptr(rax + 8 + 8 * k as i32), *r).unwrap();
        }
        for i in 6..16 {
            a.movq(qword_ptr(rax + 0x48 + 8 * (i as i32 - 6)), xmm_reg(i)).unwrap();
        }
        a.add(rsp, 0x28).unwrap();
        a.ret().unwrap();
        let (g, res) = place(a, 1024);
        (g, res.label_ip(&after).unwrap())
    }

    #[repr(C, align(16))]
    struct Buf([u64; 0x100]);

    fn check_continued(b: &Buf, mode: u8) {
        for (k, (_, _, v)) in NV.iter().enumerate() {
            assert_eq!(b.0[1 + k], *v, "mode {mode}: nonvolatile {k} after the call");
        }
        for i in 6..16 {
            assert_eq!(b.0[9 + i - 6], xmm_sentinel(i), "mode {mode}: xmm{i} after the call");
        }
    }

    /// Guest code raising through `RaiseException`: the dispatcher sees the calling guest
    /// frame's exact context, and a continued exception resumes the guest after the call.
    #[test]
    fn raise_from_guest_uses_callout_context() {
        let rv = native_va(win32_rt::unwind::RaiseException as *const () as usize);
        let (g, ret_va) = blob(rv, false);
        let h = unsafe { win32_rt::unwind::AddVectoredExceptionHandler(1, veh) };
        for mode in [2u8, 1] {
            jit::force(mode);
            *SEEN.lock().unwrap() = None;
            let mut b = Buf([0; 0x100]);
            unsafe { super::super::emu_run_full(g, [&mut b as *mut Buf as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
            let (code, addr, c) = SEEN.lock().unwrap().take().expect("VEH saw the exception");
            assert_eq!(code as u64, CODE);
            assert_eq!(c.rip, ret_va, "mode {mode}: rip = the guest return VA");
            assert_eq!(addr, ret_va, "mode {mode}: ExceptionAddress");
            assert_eq!(c.gpr[4], b.0[0], "mode {mode}: rsp = the guest rsp");
            for (_, i, v) in NV {
                assert_eq!(c.gpr[i], v, "mode {mode}: CONTEXT gpr {i}");
            }
            for i in 6..16 {
                let lo = u64::from_le_bytes(c.flt_save[0xA0 + 16 * i..0xA0 + 16 * i + 8].try_into().unwrap());
                assert_eq!(lo, xmm_sentinel(i), "mode {mode}: CONTEXT xmm{i}");
            }
            check_continued(&b, mode);
            assert_eq!(chain::depth(), 0);
        }
        jit::force(0);
        unsafe { win32_rt::unwind::RemoveVectoredExceptionHandler(h) };
    }

    std::thread_local! {
        static HOST_CAP: core::cell::RefCell<Option<Box<Context>>> = const { core::cell::RefCell::new(None) };
    }

    /// A Rust callee of guest code capturing a context for itself.
    unsafe extern "C" fn rust_captures(_out: u64) -> u64 {
        let mut c = Box::new(Context::zeroed());
        unsafe { win32_rt::unwind::RtlCaptureContext(&mut *c) };
        HOST_CAP.with(|h| *h.borrow_mut() = Some(c));
        0
    }

    /// `RtlCaptureContext` called by guest code returns the calling guest frame's context;
    /// called by Rust code (even inside a guest host call) it keeps today's host capture.
    #[test]
    fn raise_without_callout_keeps_host_capture() {
        let cv = native_va(win32_rt::unwind::RtlCaptureContext as *const () as usize);
        let (g, ret_va) = blob(cv, true);
        let rv = native_va(rust_captures as *const () as usize);
        let (g2, ret2) = blob(rv, true);
        for mode in [2u8, 1] {
            jit::force(mode);
            let mut b = Buf([0; 0x100]);
            unsafe { super::super::emu_run_full(g, [&mut b as *mut Buf as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
            let c = unsafe { &*(b.0.as_ptr().add(0x20) as *const Context) };
            assert_eq!(c.rip, ret_va, "mode {mode}: guest caller: rip = the return VA");
            assert_eq!(c.gpr[4], b.0[0], "mode {mode}: guest caller: rsp");
            for (_, i, v) in NV {
                assert_eq!(c.gpr[i], v, "mode {mode}: guest caller: gpr {i}");
            }
            check_continued(&b, mode);

            HOST_CAP.with(|h| *h.borrow_mut() = None);
            let mut b2 = Buf([0; 0x100]);
            unsafe { super::super::emu_run_full(g2, [&mut b2 as *mut Buf as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
            let hc = HOST_CAP.with(|h| h.borrow_mut().take()).expect("captured");
            assert_ne!(hc.rip, ret2, "mode {mode}: Rust caller: not the guest frame");
            assert_ne!(hc.gpr[4], b2.0[0], "mode {mode}: Rust caller: host sp, not the guest rsp");
            check_continued(&b2, mode);
        }
        jit::force(0);
        // No emulator at all: the host capture.
        let mut c = Context::zeroed();
        unsafe { win32_rt::unwind::RtlCaptureContext(&mut c) };
        assert_ne!(c.rip, 0);
        assert_eq!(chain::depth(), 0);
    }

    /// Direct `NtContinue` syscall (`SSN = 0x0043`) and normal `call ntdll!NtContinue` outside
    /// `NTCONTINUE_STACK` (no active exception callback) must land back in the emulated region
    /// with the restored guest `Context` instead of falling back to native `restore_context_asm`.
    #[test]
    fn direct_ntcontinue_syscall_and_call_resume_outside_capture_scope() {
        #[repr(C, align(16))]
        struct Out {
            rax: u64,
            rbx: u64,
            r12: u64,
            rsp: u64,
            rflags: u64,
            mxcsr: u32,
            _pad: u32,
            xmm0: u128,
            xmm6: u128,
        }

        // Resume target: writes restored registers into `[r15]` (`Out`) and returns `0xBEEF_CAFE`.
        let mut target_asm = CodeAssembler::new(64).unwrap();
        target_asm.mov(qword_ptr(r15 + 0x00), rax).unwrap();
        target_asm.mov(qword_ptr(r15 + 0x08), rbx).unwrap();
        target_asm.mov(qword_ptr(r15 + 0x10), r12).unwrap();
        target_asm.mov(qword_ptr(r15 + 0x18), rsp).unwrap();
        target_asm.pushfq().unwrap();
        target_asm.pop(rcx).unwrap();
        target_asm.mov(qword_ptr(r15 + 0x20), rcx).unwrap();
        target_asm.stmxcsr(dword_ptr(r15 + 0x28)).unwrap();
        target_asm.movdqu(xmmword_ptr(r15 + 0x30), xmm0).unwrap();
        target_asm.movdqu(xmmword_ptr(r15 + 0x40), xmm6).unwrap();
        target_asm.mov(rax, 0xBEEF_CAFEu64).unwrap();
        target_asm.ret().unwrap();
        let (target_va, _) = place(target_asm, 256);

        // 1. Direct syscall `NtContinue(rcx=ctx, rdx=0)`: if it returns instead of transferring
        //    control, `mov eax, 0xDEAD; ret` makes the test fail loudly.
        let mut sc_asm = CodeAssembler::new(64).unwrap();
        sc_asm.mov(r10, rcx).unwrap();
        sc_asm.mov(eax, win32_rt::ntsyscall::ssn::NT_CONTINUE).unwrap();
        sc_asm.syscall().unwrap();
        sc_asm.ret().unwrap();
        let (sc_va, _) = place(sc_asm, 128);

        // 2. Normal `call ntdll!NtContinue` outside `NTCONTINUE_STACK`.
        let nt_cont_va = native_va(win32_rt::ntdll::NtContinue as *const () as usize);
        let mut call_asm = CodeAssembler::new(64).unwrap();
        call_asm.sub(rsp, 0x28).unwrap();
        call_asm.mov(rax, nt_cont_va).unwrap();
        call_asm.call(rax).unwrap();
        call_asm.add(rsp, 0x28).unwrap();
        call_asm.ret().unwrap();
        let (call_va, _) = place(call_asm, 128);

        // 3. Direct syscall `NtContinue(NULL, 0)` must return STATUS_INVALID_PARAMETER.
        for mode in [2u8, 1] {
            jit::force(mode);
            let (null_res, _) = unsafe { super::super::emu_run_full(sc_va, [0; 8], [0; 4], [0; 8]) };
            assert_eq!(
                null_res as u32,
                0xC000_000Du32,
                "mode {mode}: direct NtContinue(NULL) must return STATUS_INVALID_PARAMETER"
            );

            for (label, entry_va) in [("direct-syscall", sc_va), ("call-ntdll", call_va)] {
                let mut resume_stack = [0u64; 64];
                // Place `RET_SENTINEL` at top of resume_stack so `ret` exits `emu_run_full`.
                let ret_slot = 48;
                resume_stack[ret_slot] = super::super::RET_SENTINEL;
                let resume_rsp = &resume_stack[ret_slot] as *const u64 as u64;

                let mut out = Out {
                    rax: 0,
                    rbx: 0,
                    r12: 0,
                    rsp: 0,
                    rflags: 0,
                    mxcsr: 0,
                    _pad: 0,
                    xmm0: 0,
                    xmm6: 0,
                };
                // Non-default MXCSR: FTZ (0x8000) | RZ rounding (0x6000) | default masked exceptions (0x1F80).
                const TEST_MXCSR: u32 = 0xDF80;
                win32_rt::ucrt::runtime::set_guest_mxcsr(0x1F80);
                let mut ctx = Context::zeroed();
                ctx.context_flags = win32_rt::unwind::CONTEXT_FULL;
                ctx.rip = target_va;
                ctx.gpr[0] = 0x1111_2222_3333_4444; // rax
                ctx.gpr[3] = 0x5555_6666_7777_8888; // rbx
                ctx.gpr[4] = resume_rsp; // rsp
                ctx.gpr[12] = 0x9999_AAAA_BBBB_CCCC; // r12
                ctx.gpr[15] = &mut out as *mut Out as u64; // r15
                ctx.eflags = 0x241; // CF (0x1) | ZF (0x40) | IF (0x200)
                ctx.mx_csr = TEST_MXCSR;
                *ctx.xmm_mut(0) = 0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210u128.to_le_bytes();
                *ctx.xmm_mut(6) = 0xAAAA_BBBB_CCCC_DDDD_1111_2222_3333_4444u128.to_le_bytes();

                let (ret, _) = unsafe {
                    super::super::emu_run_full(
                        entry_va,
                        [&ctx as *const Context as u64, 0, 0, 0, 0, 0, 0, 0],
                        [0; 4],
                        [0; 8],
                    )
                };
                let tls_mxcsr = win32_rt::ucrt::runtime::guest_mxcsr();
                win32_rt::ucrt::runtime::set_guest_mxcsr(0x1F80);
                assert_eq!(ret, 0xBEEF_CAFE, "{label} mode {mode}: unexpected return");
                assert_eq!(out.rax, 0x1111_2222_3333_4444, "{label} mode {mode}: rax");
                assert_eq!(out.rbx, 0x5555_6666_7777_8888, "{label} mode {mode}: rbx");
                assert_eq!(out.r12, 0x9999_AAAA_BBBB_CCCC, "{label} mode {mode}: r12");
                assert_eq!(out.rsp, resume_rsp, "{label} mode {mode}: rsp");
                assert_eq!(out.rflags & 0x41, 0x41, "{label} mode {mode}: CF|ZF in rflags");
                assert_eq!(out.mxcsr, TEST_MXCSR, "{label} mode {mode}: mxcsr");
                assert_eq!(tls_mxcsr, TEST_MXCSR, "{label} mode {mode}: thread-local guest_mxcsr");
                assert_eq!(
                    out.xmm0,
                    0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210u128,
                    "{label} mode {mode}: xmm0"
                );
                assert_eq!(
                    out.xmm6,
                    0xAAAA_BBBB_CCCC_DDDD_1111_2222_3333_4444u128,
                    "{label} mode {mode}: xmm6"
                );
                assert_eq!(chain::depth(), 0, "{label} mode {mode}: EH chain must be balanced");
            }
        }
        jit::force(0);

        // 4. Calling `dispatch_nt_syscall(NT_CONTINUE, ...)` from Rust with no emulator callout
        //    and no active capture scope must fail cleanly instead of jumping to x86 RIP on ARM64.
        let mut fake = Context::zeroed();
        fake.rip = target_va;
        let st = unsafe {
            win32_rt::ntsyscall::dispatch_nt_syscall(
                win32_rt::ntsyscall::ssn::NT_CONTINUE,
                &fake as *const Context as u64,
                0,
                0,
                0,
                0,
            )
        };
        assert_eq!(st as u32, 0xC000_000Du32);
    }
}
