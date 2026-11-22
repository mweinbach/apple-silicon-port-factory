//! L3 (target/review/exec/g12/t3_test_plan.md): dispatch and unwind over the EH record chain
//! with a mock language handler (no FH4).
//!
//! Frames: E = guest code in a [`TestImage`] run by the emulator, L = a Rust fn in generated
//! lifted-code style (`LiftedFrame`, `init_lifted_stack`, [`EhFrame`]) whose VA is a phantom
//! `.pdata` entry of the image. Every frame's UNWIND_INFO names the image's handler stub, which
//! jumps to [`mock`]; the handler data is a [`Spec`] (frame id, what to do, which code).
//!
//! Each frame takes a [`Plan`] in rcx: call `target(next)` or, with `target == 0`, raise
//! `code`, and leave what it saw in the plan.
use super::chain;
use super::testimg::{Op, TestImage, Unwind};
use super::testutil::native_va;
use crate::dll_native::lift::eh::{EhFrame, EhMeta, HANDLER_FH4, HANDLER_NONE, SHAPE_CLEANUP, SHAPE_EMPTY, SHAPE_TRY};
use crate::eh_rt::anchor::Snapshot;
use crate::simd_ops::{init_lifted_stack, LiftedFrame};
use win32_rt::libc;
use win32_rt::unwind::{
    Context, DispatcherContext, ExceptionRecord, RaiseException, RtlUnwindEx, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_TARGET_UNWIND, EXCEPTION_UNWINDING, STATUS_LONGJMP, STATUS_UNWIND_CONSOLIDATE,
};
use core::cell::RefCell;
use iced_x86::code_asm::*;

// ---------------------------------------------------------------------------
// Mock language handler
// ---------------------------------------------------------------------------

/// Handler data of every test frame.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Spec {
    id: u32,
    action: u32,
    /// Exception code the action applies to.
    code: u32,
    /// `ACT_CONSOLIDATE`: code the consolidate callback raises (0 = return `cont`).
    rethrow: u32,
    /// Continuation VA of a catch.
    cont: u64,
}

/// Log the visit only.
const ACT_NONE: u32 = 0;
/// Catch `code`: `RtlUnwindEx(frame, cont, rec, RETVAL, ctx)`, as FH4's `UnwindNestedFrames`.
const ACT_CATCH: u32 = 1;
/// Catch `code` with a `STATUS_UNWIND_CONSOLIDATE` unwind whose callback (the catch block)
/// raises `rethrow`.
const ACT_CONSOLIDATE: u32 = 2;
/// Must never be dispatched to (fork children): `_exit(EXIT_VISITED)`.
const ACT_EXIT: u32 = 3;

const EXIT_VISITED: i32 = 0x55;
const RETVAL: u64 = 0x5a5a;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    /// Dispatch (search) phase.
    D,
    /// Unwind phase, not the target frame.
    U,
    /// Unwind phase, target frame (`EXCEPTION_TARGET_UNWIND`).
    T,
}
use Phase::{D, T, U};

#[derive(Clone, Copy, Debug)]
struct Visit {
    id: u32,
    phase: Phase,
    code: u32,
    frame: u64,
    pc: u64,
    /// ExceptionInformation[0..2].
    info: [u64; 2],
}

std::thread_local! {
    static LOG: RefCell<Vec<Visit>> = const { RefCell::new(Vec::new()) };
}

fn log_take() -> Vec<Visit> {
    LOG.with(|l| core::mem::take(&mut *l.borrow_mut()))
}

fn phases(log: &[Visit]) -> Vec<(u32, Phase, u32)> {
    log.iter().map(|v| (v.id, v.phase, v.code)).collect()
}

unsafe extern "win64" fn mock(rec: *mut ExceptionRecord, frame: u64, ctx: *mut Context, dc: *mut DispatcherContext) -> u64 {
    unsafe {
        let s: Spec = core::ptr::read_unaligned((*dc).handler_data as *const Spec);
        let r = &mut *rec;
        let phase = if r.flags & EXCEPTION_UNWINDING == 0 {
            D
        } else if r.flags & EXCEPTION_TARGET_UNWIND != 0 {
            T
        } else {
            U
        };
        LOG.with(|l| l.borrow_mut().push(Visit { id: s.id, phase, code: r.code, frame, pc: (*dc).control_pc, info: [r.information[0], r.information[1]] }));
        if s.action == ACT_EXIT {
            libc::_exit(EXIT_VISITED);
        }
        if phase != D || r.code != s.code {
            return 1; // ExceptionContinueSearch
        }
        match s.action {
            ACT_CATCH => {
                RtlUnwindEx(frame, s.cont, rec as u64, RETVAL, ctx as u64, 0);
                unreachable!("RtlUnwindEx returned");
            }
            ACT_CONSOLIDATE => {
                let mut information = [0u64; 15];
                information[0] = consolidate_cb as *const () as u64;
                information[1] = s.cont;
                information[2] = s.rethrow as u64;
                let cr = Box::leak(Box::new(ExceptionRecord {
                    code: STATUS_UNWIND_CONSOLIDATE,
                    flags: EXCEPTION_NONCONTINUABLE,
                    record: rec,
                    address: core::ptr::null_mut(),
                    number_parameters: 3,
                    _pad: 0,
                    information,
                }));
                RtlUnwindEx(frame, s.cont, cr as *mut ExceptionRecord as u64, RETVAL, ctx as u64, 0);
                unreachable!("RtlUnwindEx returned");
            }
            _ => 1,
        }
    }
}

/// The "catch block" of `ACT_CONSOLIDATE`: rethrow, or return the continuation.
unsafe extern "win64" fn consolidate_cb(rec: *mut ExceptionRecord) -> u64 {
    unsafe {
        let r = &*rec;
        if r.information[2] != 0 {
            RaiseException(r.information[2], 0, 0, 0, 0, 0);
            unreachable!("rethrow continued");
        }
        r.information[1]
    }
}

fn spec(id: u32, action: u32, code: u32, rethrow: u32, cont: u64) -> Vec<u8> {
    let mut v = Vec::new();
    for x in [id, action, code, rethrow] {
        v.extend_from_slice(&x.to_le_bytes());
    }
    v.extend_from_slice(&cont.to_le_bytes());
    v
}

// ---------------------------------------------------------------------------
// Plans and frames
// ---------------------------------------------------------------------------

/// What one frame does / saw (field offsets are used by the guest code).
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
struct Plan {
    /// +0: callee VA (0 = raise `code` here).
    target: u64,
    /// +8: the callee's plan.
    next: u64,
    /// +16: exception code to raise.
    code: u64,
    /// +24: L: its `EhMeta`.
    meta: u64,
    /// +32: E: rbx at the exit (continuation or normal path).
    rbx: u64,
    /// +40: E: rax at the exit.
    rax: u64,
    /// +48: E: `RaiseException` VA.
    raise: u64,
    /// +56: L: frame id (for its sentinels).
    id: u64,
    /// +64: L: arena local at establisher+0x20 after a landing.
    local: u64,
    /// +72: L: a Rust local after a landing.
    nv: u64,
    /// +80: L: the landing continuation.
    land: u64,
    /// +88: EstablisherFrame (E: rsp after the prolog; L: `EhMeta::establisher`).
    est: u64,
    /// L: call-site offset in its phantom range (0 = `L_SITE`).
    site: u64,
    /// L raising: `RaiseException` argument count / array (0 = none).
    nargs: u64,
    args: u64,
}

fn plans(n: usize) -> &'static mut [Plan] {
    Box::leak(vec![Plan::default(); n].into_boxed_slice())
}

fn pp(p: &Plan) -> u64 {
    p as *const Plan as u64
}

const RBX_BASE: u64 = 0xB0B0_0000_0000_0000;
const LOCAL_BASE: u64 = 0xA7E0_0000_0000_0000;
const NV_BASE: u64 = 0x10CA_0000_0000_0000;

/// An E frame of a test image.
struct EFn {
    va: u64,
    /// Return VA of its `RaiseException` call (ControlPc of a raising frame).
    raise_ret: u64,
    /// Return VA of its callee call.
    call_ret: u64,
}

/// E frame `id`: `push rbx; sub rsp, 0x30`, rbx = `RBX_BASE | id`, then the plan; exit writes
/// rbx / rax into the plan (catch continuation: rax = the unwind's return value).
fn e_frame(img: &mut TestImage, id: u32, action: u32, code: u32, rethrow: u32) -> EFn {
    let mut a = CodeAssembler::new(64).unwrap();
    let mut after_push = a.create_label();
    let mut body = a.create_label();
    let mut raise = a.create_label();
    let mut call_ret = a.create_label();
    let mut raise_ret = a.create_label();
    let mut cont = a.create_label();
    let mut out = a.create_label();
    a.push(rbx).unwrap();
    a.set_label(&mut after_push).unwrap();
    a.sub(rsp, 0x30).unwrap();
    a.set_label(&mut body).unwrap();
    a.mov(qword_ptr(rsp + 0x40), rcx).unwrap();
    a.mov(qword_ptr(rcx + 88), rsp).unwrap();
    a.mov(rbx, RBX_BASE | id as u64).unwrap();
    a.mov(rax, qword_ptr(rcx)).unwrap();
    a.test(rax, rax).unwrap();
    a.jz(raise).unwrap();
    a.mov(rcx, qword_ptr(rcx + 8)).unwrap();
    a.call(rax).unwrap();
    a.set_label(&mut call_ret).unwrap();
    a.mov(eax, 1).unwrap();
    a.jmp(out).unwrap();
    a.set_label(&mut raise).unwrap();
    a.mov(r10, rcx).unwrap();
    a.mov(ecx, dword_ptr(r10 + 16)).unwrap();
    a.xor(edx, edx).unwrap();
    a.xor(r8d, r8d).unwrap();
    a.xor(r9d, r9d).unwrap();
    a.call(qword_ptr(r10 + 48)).unwrap();
    a.set_label(&mut raise_ret).unwrap();
    a.mov(eax, 2).unwrap();
    a.jmp(out).unwrap();
    a.set_label(&mut cont).unwrap();
    a.nop().unwrap();
    a.set_label(&mut out).unwrap();
    a.mov(rcx, qword_ptr(rsp + 0x40)).unwrap();
    a.mov(qword_ptr(rcx + 32), rbx).unwrap();
    a.mov(qword_ptr(rcx + 40), rax).unwrap();
    a.add(rsp, 0x30).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();
    let (va, r) = img.func(a, |r| Unwind {
        ops: vec![(1, Op::Push(3)), (5, Op::Alloc(0x30))],
        handler: true,
        data: spec(id, action, code, rethrow, r.label_ip(&cont).unwrap()),
    });
    assert_eq!(r.label_ip(&after_push).unwrap() - va, 1, "push rbx is 1 byte");
    assert_eq!(r.label_ip(&body).unwrap() - va, 5, "sub rsp, 0x30 is 4 bytes");
    EFn { va, raise_ret: r.label_ip(&raise_ret).unwrap(), call_ret: r.label_ip(&call_ret).unwrap() }
}

/// State IP of an L frame's in-flight call / catch continuation (offsets in its phantom range).
const L_SITE: u64 = 0x10;
const L_CONT: u64 = 0x30;

/// L frame `id`'s `EhMeta` (phantom `.pdata` entry: `sub rsp, 0x28`, handler = the stub).
fn l_frame(img: &mut TestImage, id: u32, action: u32, code: u32, rethrow: u32, shape: u8) -> &'static EhMeta {
    let va = img.phantom(0x40, |va| Unwind {
        ops: vec![(4, Op::Alloc(0x28))],
        handler: true,
        data: spec(id, action, code, rethrow, va + L_CONT),
    });
    Box::leak(Box::new(EhMeta {
        va,
        funcinfo_rva: 0,
        handler: HANDLER_FH4,
        shape,
        frame_reg: 0,
        establisher_off: 0x28,
        conts: &[],
    }))
}

/// A lifted fn in generated-code style, driven by its plan (rcx): frame, `EhFrame` record,
/// an arena local and a Rust local; then raise (a host callout of the lifted body:
/// `RaiseException` called directly), a catching call (`SHAPE_TRY`), or a plain call.
#[allow(clippy::too_many_arguments)]
#[inline(never)]
unsafe extern "win64" fn lifted(plan: u64, _rdx: u64, _r8: u64, _r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
    unsafe {
        let p = plan as *mut Plan;
        let meta: &'static EhMeta = &*((*p).meta as *const EhMeta);
        let mut stack = LiftedFrame::<256, 16>::new();
        let entry_rsp = init_lifted_stack(&mut stack, a5, a6, a7, a8);
        let mut eh = EhFrame::new(meta, entry_rsp);
        let _eh = eh.push();
        let est = meta.establisher(entry_rsp);
        (*p).est = est;
        // Garbage in the frame's other arena words: the walk must never read them.
        for k in 0..4u64 {
            ((est + 8 * k) as *mut u64).write(0xDEAD_0000_0000_0000 | k);
        }
        ((est + 0x20) as *mut u64).write(LOCAL_BASE | (*p).id);
        let nv = core::hint::black_box(NV_BASE | (*p).id);
        let site = meta.va + if (*p).site != 0 { (*p).site } else { L_SITE };
        let (target, next) = ((*p).target, (*p).next);
        if target == 0 {
            eh.site(site);
            let flags = if (*p).nargs != 0 { EXCEPTION_NONCONTINUABLE as u64 } else { 0 };
            RaiseException((*p).code, flags, (*p).nargs, (*p).args, 0, 0);
            return 2;
        }
        if meta.shape == SHAPE_TRY {
            match eh.call(site, target, next, 0, 0, 0, 0, 0, 0, 0) {
                Ok(v) => v,
                Err(l) => {
                    (*p).land = l.cont_va;
                    (*p).rax = crate::dll_native::lift::eh::landed_rax();
                    (*p).nv = nv;
                    (*p).local = ((est + 0x20) as *const u64).read();
                    0x1A4D
                }
            }
        } else {
            eh.site(site);
            crate::exe::call_va8(target, next, 0, 0, 0, 0, 0, 0, 0)
        }
    }
}

/// A Rust API calling guest code directly (`emu_run_full`: opaque region).
unsafe extern "win64" fn rust_api(plan: u64) -> u64 {
    unsafe {
        let p = plan as *const Plan;
        crate::emu::emu_run_full((*p).target, [(*p).next, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0
    }
}

unsafe extern "win64" fn ret7() -> u64 {
    7
}

fn setup() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        win32_rt::set_guest_dispatcher(crate::exe::call_va8);
        win32_rt::set_native_va_registrar(crate::exe::register_native_va);
        chain::register_hooks();
    });
}

fn image() -> TestImage {
    TestImage::new(native_va(mock as *const () as usize))
}

fn raise_va() -> u64 {
    native_va(RaiseException as *const () as usize)
}

fn lifted_va() -> u64 {
    native_va(lifted as *const () as usize)
}

fn run_e(va: u64, p: &Plan) -> u64 {
    unsafe { crate::emu::emu_run_full(va, [pp(p), 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 }
}

fn ls_top() -> usize {
    let c = crate::callctx::ctx();
    unsafe { crate::lstack::shadow_warm(c) };
    c.ls_top.get()
}

const MODES: [u8; 2] = [2, 1];

fn force(mode: u8) {
    crate::emu::eh::test_force_jit(mode);
}

fn visit(log: &[Visit], id: u32, phase: Phase) -> Visit {
    *log.iter().find(|v| v.id == id && v.phase == phase).unwrap_or_else(|| panic!("no ({id}, {phase:?}) visit: {log:#x?}"))
}

// ---------------------------------------------------------------------------
// Cases
// ---------------------------------------------------------------------------

/// a. E(catch) -> E(throw) in one region: the continuation runs in the same `emu_run_full`,
/// rax = the unwind's return value, rbx = the virtually unwound value.
#[test]
fn throw_e_caught_e_same_region() {
    setup();
    const CODE: u32 = 0xE0AA_0A01;
    let mut img = image();
    let c = e_frame(&mut img, 1, ACT_CATCH, CODE, 0);
    let t = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    img.finish();
    let raise = raise_va();
    for mode in MODES {
        force(mode);
        log_take();
        let p = plans(2);
        p[1] = Plan { code: CODE as u64, raise, ..Default::default() };
        p[0] = Plan { target: t.va, next: pp(&p[1]), ..Default::default() };
        let ls0 = ls_top();
        let r = run_e(c.va, &p[0]);
        let log = log_take();
        assert_eq!(r, RETVAL, "mode {mode}: {log:#x?}");
        assert_eq!(p[0].rax, RETVAL);
        assert_eq!(p[0].rbx, RBX_BASE | 1, "mode {mode}: rbx restored from E(throw)'s push slot");
        assert_eq!(phases(&log), vec![(3, D, CODE), (1, D, CODE), (3, U, CODE), (1, T, CODE)], "mode {mode}");
        let d3 = visit(&log, 3, D);
        assert_eq!((d3.frame, d3.pc), (p[1].est, t.raise_ret), "mode {mode}: E(throw) frame / ControlPc");
        let d1 = visit(&log, 1, D);
        assert_eq!((d1.frame, d1.pc), (p[0].est, c.call_ret), "mode {mode}: E(catch) frame / ControlPc");
        assert_eq!(chain::depth(), 0);
        assert_eq!(ls_top(), ls0);
    }
    force(0);
}

/// b. E(catch) -> L(cleanup) -> E(throw): L's handler runs in both phases with its arena
/// establisher and ControlPc = its call site; the outer E resumes through its region record;
/// the Rust frames in between are discarded with lstack and the chain restored.
#[test]
fn throw_e_through_l_cleanup_caught_e() {
    setup();
    const CODE: u32 = 0xE0AA_0B01;
    let mut img = image();
    let c = e_frame(&mut img, 1, ACT_CATCH, CODE, 0);
    let t = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    let m = l_frame(&mut img, 2, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    img.finish();
    let (raise, lv) = (raise_va(), lifted_va());
    for mode in MODES {
        force(mode);
        log_take();
        let p = plans(3);
        p[2] = Plan { code: CODE as u64, raise, ..Default::default() };
        p[1] = Plan { target: t.va, next: pp(&p[2]), meta: m as *const EhMeta as u64, id: 2, ..Default::default() };
        p[0] = Plan { target: lv, next: pp(&p[1]), ..Default::default() };
        let ls0 = ls_top();
        let r = run_e(c.va, &p[0]);
        let log = log_take();
        assert_eq!(r, RETVAL, "mode {mode}: {log:#x?}");
        assert_eq!(p[0].rbx, RBX_BASE | 1, "mode {mode}");
        assert_eq!(
            phases(&log),
            vec![(3, D, CODE), (2, D, CODE), (1, D, CODE), (3, U, CODE), (2, U, CODE), (1, T, CODE)],
            "mode {mode}"
        );
        for ph in [D, U] {
            let v = visit(&log, 2, ph);
            assert_eq!(v.frame, p[1].est, "mode {mode}: L establisher ({ph:?})");
            assert_eq!(v.pc, m.va + L_SITE, "mode {mode}: L ControlPc = its call site ({ph:?})");
        }
        assert!(
            chain::hook_frame_ok(p[1].est),
            "mode {mode}: L's establisher 0x{:x} is in the lstack arena (base 0x{:x}, top now 0x{:x}; emu stack {:x?})",
            p[1].est,
            crate::callctx::ctx().ls_base.get(),
            ls_top(),
            crate::emu::eh::stack_range()
        );
        assert_eq!(visit(&log, 1, T).frame, p[0].est);
        assert_eq!(chain::depth(), 0, "mode {mode}: chain back to the outer region's");
        assert_eq!(ls_top(), ls0, "mode {mode}: lstack top restored");
    }
    force(0);
}

/// c. L(try) -> E(throw): the catching call returns `Err(Landing { cont })` and L continues
/// with its Rust locals and arena locals intact.
#[test]
fn throw_e_caught_in_l_try() {
    setup();
    const CODE: u32 = 0xE0AA_0C01;
    let mut img = image();
    let t = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    let m = l_frame(&mut img, 2, ACT_CATCH, CODE, 0, SHAPE_TRY);
    img.finish();
    let raise = raise_va();
    for mode in MODES {
        force(mode);
        log_take();
        let p = plans(2);
        p[1] = Plan { code: CODE as u64, raise, ..Default::default() };
        p[0] = Plan { target: t.va, next: pp(&p[1]), meta: m as *const EhMeta as u64, id: 2, ..Default::default() };
        let ls0 = ls_top();
        let r = unsafe { lifted(pp(&p[0]), 0, 0, 0, 0, 0, 0, 0) };
        let log = log_take();
        assert_eq!(r, 0x1A4D, "mode {mode}: {log:#x?}");
        assert_eq!(p[0].land, m.va + L_CONT, "mode {mode}: landing continuation");
        assert_eq!(p[0].rax, RETVAL, "mode {mode}: rax at the landing = RtlUnwindEx's ReturnValue");
        assert_eq!(p[0].nv, NV_BASE | 2, "mode {mode}: Rust local");
        assert_eq!(p[0].local, LOCAL_BASE | 2, "mode {mode}: arena local at establisher+0x20");
        assert_eq!(phases(&log), vec![(3, D, CODE), (2, D, CODE), (3, U, CODE), (2, T, CODE)], "mode {mode}");
        let t2 = visit(&log, 2, T);
        assert_eq!((t2.frame, t2.pc), (p[0].est, m.va + L_SITE), "mode {mode}");
        assert_eq!(chain::depth(), 0, "mode {mode}");
        assert_eq!(ls_top(), ls0, "mode {mode}");
    }
    force(0);
}

/// An L frame's `__except` scope instruction in generated-code style (`seh_guard_lines`): one
/// guarded access at `L_SITE` to `addr` (`write`: `wu32` of 0x1234_5678, else `ru64`).
#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
#[allow(clippy::too_many_arguments)]
#[inline(never)]
unsafe extern "win64" fn lifted_guarded(plan: u64, addr: u64, write: u64, _r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
    unsafe {
        let p = plan as *mut Plan;
        let meta: &'static EhMeta = &*((*p).meta as *const EhMeta);
        let mut stack = LiftedFrame::<256, 16>::new();
        let entry_rsp = init_lifted_stack(&mut stack, a5, a6, a7, a8);
        let mut eh = EhFrame::new(meta, entry_rsp);
        let _eh = eh.push();
        let est = meta.establisher(entry_rsp);
        (*p).est = est;
        ((est + 0x20) as *mut u64).write(LOCAL_BASE | (*p).id);
        let nv = core::hint::black_box(NV_BASE | (*p).id);
        let site = meta.va + L_SITE;
        let r = if write != 0 { eh.wu32(site, addr, 0x1234_5678).map(|()| 0x5701) } else { eh.ru64(site, addr) };
        match r {
            Ok(v) => v,
            Err(l) => {
                (*p).land = l.cont_va;
                (*p).rax = crate::dll_native::lift::eh::landed_rax();
                (*p).nv = nv;
                (*p).local = ((est + 0x20) as *const u64).read();
                0x1A4D
            }
        }
    }
}

/// A guarded access that faults inside an L frame's `__except` scope raises
/// STATUS_ACCESS_VIOLATION into that frame at the instruction (ControlPc = the site,
/// Information = [write, address]) and lands at its continuation with the frame's state; a
/// mapped access raises nothing.
#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
#[test]
fn guarded_fault_in_l_seh_scope_lands_at_except() {
    setup();
    crate::dll_native::lift::guarded::install_test_fault_handler();
    const AV: u32 = win32_rt::unwind::STATUS_ACCESS_VIOLATION;
    let mut img = image();
    let m = l_frame(&mut img, 2, ACT_CATCH, AV, 0, SHAPE_EMPTY);
    img.finish();
    let meta = m as *const EhMeta as u64;
    // unmapped (__PAGEZERO): an aligned load, an unaligned store
    for (addr, write) in [(0x10u64, 0u64), (0x13, 1)] {
        log_take();
        let p = plans(1);
        p[0] = Plan { meta, id: 2, ..Default::default() };
        let ls0 = ls_top();
        let r = unsafe { lifted_guarded(pp(&p[0]), addr, write, 0, 0, 0, 0, 0) };
        let log = log_take();
        assert_eq!(r, 0x1A4D, "write {write}: {log:#x?}");
        assert_eq!(p[0].land, m.va + L_CONT, "write {write}: landing continuation");
        assert_eq!(p[0].rax, RETVAL, "write {write}: rax at the landing = RtlUnwindEx's ReturnValue");
        assert_eq!(p[0].nv, NV_BASE | 2, "write {write}: Rust local");
        assert_eq!(p[0].local, LOCAL_BASE | 2, "write {write}: arena local at establisher+0x20");
        assert_eq!(phases(&log), vec![(2, D, AV), (2, T, AV)], "write {write}");
        let d = visit(&log, 2, D);
        assert_eq!((d.frame, d.pc), (p[0].est, m.va + L_SITE), "write {write}: establisher / ControlPc = the instruction");
        assert_eq!(d.info, [write, addr], "write {write}: ExceptionInformation");
        assert_eq!(chain::depth(), 0, "write {write}");
        assert_eq!(ls_top(), ls0, "write {write}");
    }
    let cell: &'static mut u64 = Box::leak(Box::new(0x0BAD_F00D_0000_0001));
    let c = cell as *mut u64 as u64;
    let p = plans(1);
    p[0] = Plan { meta, id: 2, ..Default::default() };
    assert_eq!(unsafe { lifted_guarded(pp(&p[0]), c, 0, 0, 0, 0, 0, 0) }, 0x0BAD_F00D_0000_0001);
    assert_eq!(unsafe { lifted_guarded(pp(&p[0]), c, 1, 0, 0, 0, 0, 0) }, 0x5701);
    assert_eq!(unsafe { core::ptr::read_volatile(c as *const u64) }, 0x0BAD_F00D_1234_5678);
    assert_eq!(p[0].land, 0, "no landing");
    assert!(log_take().is_empty(), "no dispatch for a mapped access");
}

/// E frame `id` whose body loads a dword from the plan's `next` (an invalid address in the
/// tests): `push rbx; sub rsp, 0x30`, rbx = `RBX_BASE | id`, then `mov eax, [rax]`; the normal
/// path exits with rax = 2, the catch continuation with the unwind's return value. Returns the
/// frame and the VA of the faulting instruction.
fn e_fault_frame(img: &mut TestImage, id: u32, action: u32, code: u32) -> (EFn, u64) {
    let mut a = CodeAssembler::new(64).unwrap();
    let mut fault = a.create_label();
    let mut cont = a.create_label();
    let mut out = a.create_label();
    a.push(rbx).unwrap();
    a.sub(rsp, 0x30).unwrap();
    a.mov(qword_ptr(rsp + 0x40), rcx).unwrap();
    a.mov(qword_ptr(rcx + 88), rsp).unwrap();
    a.mov(rbx, RBX_BASE | id as u64).unwrap();
    a.mov(rax, qword_ptr(rcx + 8)).unwrap();
    a.set_label(&mut fault).unwrap();
    a.mov(eax, dword_ptr(rax)).unwrap();
    a.mov(eax, 2).unwrap();
    a.jmp(out).unwrap();
    a.set_label(&mut cont).unwrap();
    a.nop().unwrap();
    a.set_label(&mut out).unwrap();
    a.mov(rcx, qword_ptr(rsp + 0x40)).unwrap();
    a.mov(qword_ptr(rcx + 32), rbx).unwrap();
    a.mov(qword_ptr(rcx + 40), rax).unwrap();
    a.add(rsp, 0x30).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();
    let (va, r) = img.func(a, |r| Unwind {
        ops: vec![(1, Op::Push(3)), (5, Op::Alloc(0x30))],
        handler: true,
        data: spec(id, action, code, 0, r.label_ip(&cont).unwrap()),
    });
    (EFn { va, raise_ret: 0, call_ret: 0 }, r.label_ip(&fault).unwrap())
}

/// Precise guest faults (`emu::set_precise_faults`, difftest child only): an invalid access
/// at a guest instruction (null page, non-canonical) is dispatched as STATUS_ACCESS_VIOLATION
/// from that instruction (ControlPc = it, Information = [write, address], the frame's
/// establisher) and an `__except` of the guest frame lands at its continuation with the
/// frame's nonvolatiles (rbx) and rax = the unwind's return value, in the same region.
#[test]
fn precise_guest_fault_lands_in_guest_except() {
    setup();
    const AV: u32 = win32_rt::unwind::STATUS_ACCESS_VIOLATION;
    let mut img = image();
    let (f, fault_va) = e_fault_frame(&mut img, 1, ACT_CATCH, AV);
    img.finish();
    crate::emu::set_precise_faults_local(true);
    assert!(crate::emu::precise_faults());
    for mode in MODES {
        force(mode);
        for bad in [0x40u64, 0x8000_0000_1000_0000] {
            log_take();
            let p = plans(1);
            p[0] = Plan { next: bad, ..Default::default() };
            let ls0 = ls_top();
            let n0 = crate::emu::PRECISE_FAULT_DISPATCHES.load(core::sync::atomic::Ordering::Relaxed);
            let r = run_e(f.va, &p[0]);
            let log = log_take();
            assert_eq!(r, RETVAL, "mode {mode} addr {bad:#x}: {log:#x?}");
            assert_eq!(p[0].rax, RETVAL, "mode {mode} addr {bad:#x}: rax at the continuation");
            assert_eq!(p[0].rbx, RBX_BASE | 1, "mode {mode} addr {bad:#x}: rbx at the continuation");
            assert_eq!(phases(&log), vec![(1, D, AV), (1, T, AV)], "mode {mode} addr {bad:#x}");
            let d = visit(&log, 1, D);
            assert_eq!((d.frame, d.pc), (p[0].est, fault_va), "mode {mode} addr {bad:#x}: establisher / ControlPc = the instruction");
            assert_eq!(d.info, [0, bad], "mode {mode} addr {bad:#x}: ExceptionInformation");
            assert!(crate::emu::PRECISE_FAULT_DISPATCHES.load(core::sync::atomic::Ordering::Relaxed) > n0);
            assert_eq!(chain::depth(), 0, "mode {mode}");
            assert_eq!(ls_top(), ls0, "mode {mode}");
        }
    }
    crate::emu::set_precise_faults_local(false);
    force(0);
}

/// The default path never takes precise guest faults: the mode is off unless the difftest
/// child turns it on, and the same invalid accesses dispatch nothing (tests: the lenient
/// emulator reads 0 and the normal path returns 2), with the translator modes as usual.
#[test]
fn precise_guest_faults_default_off() {
    setup();
    const AV: u32 = win32_rt::unwind::STATUS_ACCESS_VIOLATION;
    let mut img = image();
    let (f, _) = e_fault_frame(&mut img, 1, ACT_CATCH, AV);
    img.finish();
    assert!(!crate::emu::precise_faults(), "precise guest faults are off by default");
    for mode in MODES {
        force(mode);
        for bad in [0x40u64, 0x8000_0000_1000_0000] {
            log_take();
            let p = plans(1);
            p[0] = Plan { next: bad, ..Default::default() };
            let r = run_e(f.va, &p[0]);
            let log = log_take();
            assert_eq!(r, 2, "mode {mode} addr {bad:#x}: normal path: {log:#x?}");
            assert_eq!(p[0].rbx, RBX_BASE | 1);
            assert!(log.is_empty(), "mode {mode} addr {bad:#x}: no dispatch: {log:#x?}");
        }
    }
    force(0);
}

/// A landing on a thread whose first lifted frame is the catching one (no lstack arena
/// before it): later frames still get arena establishers (the landing snapshot must not
/// record the pre-arena top 0).
#[test]
fn landing_on_fresh_thread_keeps_arena() {
    setup();
    const CODE: u32 = 0xE0AA_0C02;
    let mut img = image();
    let t = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    let m = l_frame(&mut img, 2, ACT_CATCH, CODE, 0, SHAPE_TRY);
    let m_plain = l_frame(&mut img, 4, ACT_NONE, 0, 0, SHAPE_TRY);
    img.finish();
    let (raise, r7) = (raise_va(), native_va(ret7 as *const () as usize));
    let (m, m_plain) = (m as *const EhMeta as u64, m_plain as *const EhMeta as u64);
    std::thread::spawn(move || {
        assert_eq!(crate::callctx::ctx().ls_base.get(), 0, "fresh thread");
        let p = plans(2);
        p[1] = Plan { code: CODE as u64, raise, ..Default::default() };
        p[0] = Plan { target: t.va, next: pp(&p[1]), meta: m, id: 2, ..Default::default() };
        assert_eq!(unsafe { lifted(pp(&p[0]), 0, 0, 0, 0, 0, 0, 0) }, 0x1A4D);
        assert!(chain::hook_frame_ok(p[0].est), "catching frame 0x{:x}", p[0].est);
        let q = plans(1);
        q[0] = Plan { target: r7, meta: m_plain, id: 4, ..Default::default() };
        assert_eq!(unsafe { lifted(pp(&q[0]), 0, 0, 0, 0, 0, 0, 0) }, 7);
        assert!(chain::hook_frame_ok(q[0].est), "frame after the landing 0x{:x} (ls_top 0x{:x})", q[0].est, ls_top());
        assert_eq!(chain::depth(), 0);
    })
    .join()
    .unwrap();
}

/// Run `f` (the fatal part of test `test`, a full `module_path!()::name`) in a child process;
/// its exit status and output. The child is this test binary re-run on that one test with
/// `AW2_EH_CHILD=<test>`, where `in_child` runs `f` directly (killed after 20 s): a fresh,
/// single-threaded process, unlike a `fork` of the multi-threaded harness, whose child
/// inherits locks held by the other test threads.
fn in_child(test: &str, f: impl FnOnce()) -> (i32, String) {
    let name = test.split_once("::").map_or(test, |(_, rest)| rest);
    if std::env::var("AW2_EH_CHILD").as_deref() == Ok(name) {
        unsafe { libc::alarm(20) };
        f();
        unsafe { libc::_exit(0x66) };
    }
    let out = std::process::Command::new(std::env::current_exe().expect("test binary path"))
        .args([name, "--exact", "--nocapture", "--test-threads=1"])
        .env("AW2_EH_CHILD", name)
        .env("AW2_CRASH_DIR", std::env::temp_dir().join("aw2-eh-tests"))
        .stdin(std::process::Stdio::null())
        .output()
        .expect("spawn child test process");
    let log = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    match out.status.code() {
        Some(c) => (c, log),
        None => panic!("child {name} did not exit: {:?}\n{log}", out.status),
    }
}

/// d. E(catch) -> Rust API -> `emu_run_full` (opaque) -> E(throw): the walk stops at the opaque
/// region and the exception is unhandled (exit code = exception code), never caught above.
#[test]
fn throw_through_opaque_host_frame_is_fatal() {
    setup();
    const CODE: u32 = 0xE0AA_0D01;
    let mut img = image();
    let c = e_frame(&mut img, 1, ACT_EXIT, CODE, 0);
    let t = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    img.finish();
    let raise = raise_va();
    let api = native_va(rust_api as *const () as usize);
    let p = plans(3);
    p[2] = Plan { code: CODE as u64, raise, ..Default::default() };
    p[1] = Plan { target: t.va, next: pp(&p[2]), ..Default::default() };
    p[0] = Plan { target: api, next: pp(&p[1]), ..Default::default() };
    let p0 = pp(&p[0]);
    let (st, log) = in_child(concat!(module_path!(), "::throw_through_opaque_host_frame_is_fatal"), || {
        run_e(c.va, unsafe { &*(p0 as *const Plan) });
    });
    assert_eq!(st, (CODE & 0xFF) as i32, "unhandled exit (0x{st:x}); {EXIT_VISITED:#x} = the outer E was dispatched to\n{log}");
}

/// e. E1(catch CODE2) -> L(try: catch CODE1, its catch block rethrows CODE2) -> E2(throw CODE1):
/// the rethrow's dispatch starts at the consolidate crossing (L), never revisits E2, and is
/// caught by E1; L's landing state is released.
#[test]
fn rethrow_from_catch_skips_unwound_frames() {
    setup();
    const CODE1: u32 = 0xE0AA_0E01;
    const CODE2: u32 = 0xE0AA_0E02;
    let mut img = image();
    let e1 = e_frame(&mut img, 1, ACT_CATCH, CODE2, 0);
    let e2 = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    let m = l_frame(&mut img, 2, ACT_CONSOLIDATE, CODE1, CODE2, SHAPE_TRY);
    let m_plain = l_frame(&mut img, 4, ACT_NONE, 0, 0, SHAPE_TRY);
    img.finish();
    let (raise, lv) = (raise_va(), lifted_va());
    let r7 = native_va(ret7 as *const () as usize);
    for mode in MODES {
        force(mode);
        log_take();
        let p = plans(3);
        p[2] = Plan { code: CODE1 as u64, raise, ..Default::default() };
        p[1] = Plan { target: e2.va, next: pp(&p[2]), meta: m as *const EhMeta as u64, id: 2, ..Default::default() };
        p[0] = Plan { target: lv, next: pp(&p[1]), ..Default::default() };
        let ls0 = ls_top();
        let r = run_e(e1.va, &p[0]);
        let log = log_take();
        assert_eq!(r, RETVAL, "mode {mode}: {log:#x?}");
        assert_eq!(p[0].rbx, RBX_BASE | 1);
        assert_eq!(p[1].land, 0, "mode {mode}: L never resumed (its catch block rethrew)");
        const CONS: u32 = STATUS_UNWIND_CONSOLIDATE;
        assert_eq!(
            phases(&log),
            vec![
                (3, D, CODE1),
                (2, D, CODE1),
                (3, U, CONS),
                (2, T, CONS),
                (2, D, CODE2),
                (1, D, CODE2),
                (2, U, CODE2),
                (1, T, CODE2),
            ],
            "mode {mode}"
        );
        assert_eq!(chain::depth(), 0);
        assert_eq!(ls_top(), ls0);
        // No stale landing state: a fresh catching call returns normally.
        let q = plans(1);
        q[0] = Plan { target: r7, meta: m_plain as *const EhMeta as u64, id: 4, ..Default::default() };
        assert_eq!(unsafe { lifted(pp(&q[0]), 0, 0, 0, 0, 0, 0, 0) }, 7);
        assert_eq!(q[0].land, 0);
        assert_eq!(chain::depth(), 0);
    }
    force(0);
}

/// f. E -> L -> E -> L(raise via a host callout of the lifted body): the visit order is the
/// record order; lifted frames come from their records (their arena words are garbage).
#[test]
fn chain_order_across_four_regions() {
    setup();
    const CODE: u32 = 0xE0AA_0F01;
    let mut img = image();
    let ea = e_frame(&mut img, 1, ACT_CATCH, CODE, 0);
    let eb = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    let m1 = l_frame(&mut img, 2, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    let m2 = l_frame(&mut img, 4, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    img.finish();
    let lv = lifted_va();
    for mode in MODES {
        force(mode);
        log_take();
        let p = plans(4);
        p[3] = Plan { code: CODE as u64, meta: m2 as *const EhMeta as u64, id: 4, ..Default::default() };
        p[2] = Plan { target: lv, next: pp(&p[3]), ..Default::default() };
        p[1] = Plan { target: eb.va, next: pp(&p[2]), meta: m1 as *const EhMeta as u64, id: 2, ..Default::default() };
        p[0] = Plan { target: lv, next: pp(&p[1]), ..Default::default() };
        let ls0 = ls_top();
        let r = run_e(ea.va, &p[0]);
        let log = log_take();
        assert_eq!(r, RETVAL, "mode {mode}: {log:#x?}");
        assert_eq!(p[0].rbx, RBX_BASE | 1);
        assert_eq!(
            phases(&log),
            vec![
                (4, D, CODE),
                (3, D, CODE),
                (2, D, CODE),
                (1, D, CODE),
                (4, U, CODE),
                (3, U, CODE),
                (2, U, CODE),
                (1, T, CODE),
            ],
            "mode {mode}"
        );
        for (id, k, m) in [(2u32, 1usize, m1), (4, 3, m2)] {
            let v = visit(&log, id, D);
            assert_eq!((v.frame, v.pc), (p[k].est, m.va + L_SITE), "mode {mode}: L{id}");
        }
        let v3 = visit(&log, 3, D);
        assert_eq!((v3.frame, v3.pc), (p[2].est, eb.call_ret), "mode {mode}: inner E");
        assert_eq!(chain::depth(), 0);
        assert_eq!(ls_top(), ls0);
    }
    force(0);
}

/// g. Establisher frames: the emulator stack and the lstack arena pass, a heap address does
/// not; a dispatch reaching a heap establisher stops (`EXCEPTION_STACK_INVALID`, unhandled)
/// without calling that frame's handler.
#[test]
fn establisher_bounds_accept_emu_stack_and_arena_only() {
    setup();
    const CODE: u32 = 0xE0AA_0A07;
    let mut img = image();
    let e = e_frame(&mut img, 1, ACT_NONE, 0, 0);
    let m = l_frame(&mut img, 2, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    let m_bad = l_frame(&mut img, 5, ACT_EXIT, 0, 0, SHAPE_CLEANUP);
    img.finish();
    let r7 = native_va(ret7 as *const () as usize);
    let p = plans(2);
    p[0] = Plan { target: r7, ..Default::default() };
    assert_eq!(run_e(e.va, &p[0]), 1);
    assert!(chain::hook_frame_ok(p[0].est), "emulator stack frame 0x{:x}", p[0].est);
    p[1] = Plan { target: r7, meta: m as *const EhMeta as u64, id: 2, ..Default::default() };
    unsafe { lifted(pp(&p[1]), 0, 0, 0, 0, 0, 0, 0) };
    assert!(chain::hook_frame_ok(p[1].est), "lstack arena frame 0x{:x}", p[1].est);
    let heap = Box::leak(Box::new([0u64; 64]));
    let heap_rsp = heap.as_ptr() as u64 + 0x100;
    assert!(!chain::hook_frame_ok(heap_rsp - 0x28), "heap frame accepted");
    let (st, log) = in_child(concat!(module_path!(), "::establisher_bounds_accept_emu_stack_and_arena_only"), || unsafe {
        let mut eh = EhFrame::new(m_bad, heap_rsp);
        let _g = eh.push();
        eh.site(m_bad.va + L_SITE);
        RaiseException(CODE as u64, 0, 0, 0, 0, 0);
    });
    assert_eq!(st, (CODE & 0xFF) as i32, "exit 0x{st:x}; {EXIT_VISITED:#x} = the heap frame's handler ran\n{log}");
}

/// h. No throw: `EhFrame::call` / `call_xmm` give the results and `CallCtx` end state of
/// `exe::call_va8` / `simd_ops::call_va8_xmm` (guest callee, host callee).
#[test]
fn nothrow_call_caught_is_call_va8() {
    setup();
    let mut img = image();
    let mut a = CodeAssembler::new(64).unwrap();
    a.lea(rax, qword_ptr(rcx + rdx * 2)).unwrap();
    a.add(rax, r8).unwrap();
    a.add(rax, r9).unwrap();
    a.paddq(xmm0, xmm1).unwrap();
    a.ret().unwrap();
    let (guest, _) = img.func(a, |_| Unwind::leaf());
    let m = l_frame(&mut img, 6, ACT_NONE, 0, 0, SHAPE_TRY);
    img.finish();
    unsafe extern "win64" fn host_sum(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64) -> u64 {
        a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f + 7 * g + 8 * h
    }
    let host = native_va(host_sum as *const () as usize);
    let xmm: [u128; 4] = [0x11 | (0x22 << 64), 0x33 | (0x44 << 64), 5, 6];
    for mode in MODES {
        force(mode);
        let mut stack = LiftedFrame::<256, 16>::new();
        let entry_rsp = unsafe { init_lifted_stack(&mut stack, 0, 0, 0, 0) };
        let mut eh = EhFrame::new(m, entry_rsp);
        let _g = unsafe { eh.push() };
        for (name, t) in [("guest", guest), ("host", host)] {
            unsafe {
                let s0 = Snapshot::take();
                let want = crate::exe::call_va8(t, 1, 2, 3, 4, 5, 6, 7, 8);
                let s1 = Snapshot::take();
                s0.restore();
                let got = eh.call(m.va + L_SITE, t, 1, 2, 3, 4, 5, 6, 7, 8).expect("no landing");
                let s2 = Snapshot::take();
                assert_eq!(got, want, "mode {mode} {name}");
                assert_eq!(s2, s1, "mode {mode} {name}: CallCtx end state");

                let s0 = Snapshot::take();
                let want = crate::simd_ops::call_va8_xmm(t, 1, 2, 3, 4, 5, 6, 7, 8, xmm);
                let s1 = Snapshot::take();
                s0.restore();
                let got = eh.call_xmm(m.va + L_SITE, t, 1, 2, 3, 4, 5, 6, 7, 8, xmm).expect("no landing");
                let s2 = Snapshot::take();
                assert_eq!(got.0, want.0, "mode {mode} {name} (xmm)");
                if t == guest {
                    assert_eq!(got.1, want.1, "mode {mode} {name}: xmm result");
                }
                assert_eq!(s2, s1, "mode {mode} {name} (xmm): CallCtx end state");
            }
        }
        assert_eq!(chain::depth(), 1, "only the frame's own record");
    }
    force(0);
    assert_eq!(unsafe { crate::exe::call_va8(guest, 1, 2, 3, 4, 0, 0, 0, 0) }, 1 + 4 + 3 + 4);
}

/// EstablisherFrame of a lifted fn with a frame register: the prolog's rsp delta up to
/// `SET_FPREG` (later allocations excluded).
#[test]
fn lifted_establisher_with_frame_register() {
    setup();
    let mut img = image();
    let va = img.phantom(0x40, |_| Unwind {
        ops: vec![(1, Op::Push(5)), (5, Op::Alloc(0x20)), (10, Op::SetFp(5, 2)), (14, Op::Alloc(0x40))],
        handler: false,
        data: Vec::new(),
    });
    img.finish();
    let m = EhMeta { va, funcinfo_rva: 0, handler: HANDLER_FH4, shape: SHAPE_CLEANUP, frame_reg: 5, establisher_off: 0, conts: &[] };
    assert_eq!(win32_rt::unwind::establisher_rsp_delta(va), Some(0x28));
    assert_eq!(m.establisher(0x10_0000), 0x10_0000 - 0x28);
}

/// `catch_cxx` inspect function of the tests: the record's parameter count, tagged.
unsafe fn note_nparams(rec: &ExceptionRecord) -> [u64; 2] {
    [0x7E57, rec.number_parameters as u64]
}

/// Host boundary ([`crate::eh_rt::host_try::catch_cxx`]) -> L(cleanup) -> E(throw C++): the
/// boundary catches after L's and E's cleanup ran (unwind phase, boundary = target), its
/// inspect function saw the exception in the dispatch phase, and the call's per-thread state
/// is back. Other codes pass through it; a returning closure gives `Ok`.
#[test]
fn host_boundary_catches_cxx_after_cleanup() {
    use crate::eh_rt::host_try::catch_cxx;
    setup();
    const CXX: u32 = win32_rt::unwind::CXX_EXCEPTION;
    let mut img = image();
    let t = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    let m = l_frame(&mut img, 2, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    img.finish();
    let raise = raise_va();
    for mode in MODES {
        force(mode);
        log_take();
        // Through a lifted frame (the difftest's lifted run).
        let p = plans(2);
        p[1] = Plan { code: CXX as u64, raise, ..Default::default() };
        p[0] = Plan { target: t.va, next: pp(&p[1]), meta: m as *const EhMeta as u64, id: 2, ..Default::default() };
        let ls0 = ls_top();
        let r = unsafe { catch_cxx(note_nparams, || lifted(pp(&p[0]), 0, 0, 0, 0, 0, 0, 0)) };
        let log = log_take();
        let c = match r {
            Err(c) => c,
            Ok(v) => panic!("mode {mode}: returned 0x{v:x}: {log:#x?}"),
        };
        assert_eq!((c.rec.code, c.note), (CXX, [0x7E57, 0]), "mode {mode}");
        assert_eq!(phases(&log), vec![(3, D, CXX), (2, D, CXX), (3, U, CXX), (2, U, CXX)], "mode {mode}");
        assert_eq!(chain::depth(), 0, "mode {mode}: boundary popped");
        assert_eq!(ls_top(), ls0, "mode {mode}: lstack top restored");
        // A guest entry (transparent region, the difftest's guest run).
        let q = plans(1);
        q[0] = Plan { code: CXX as u64, raise, ..Default::default() };
        let r = unsafe { catch_cxx(note_nparams, || crate::emu::emu_run_full_rax(t.va, [pp(&q[0]), 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], 0)) };
        let log = log_take();
        assert!(r.is_err(), "mode {mode}: guest throw not caught: {log:#x?}");
        assert_eq!(phases(&log), vec![(3, D, CXX), (3, U, CXX)], "mode {mode}");
        assert_eq!(chain::depth(), 0, "mode {mode}");
        // No throw.
        assert_eq!(unsafe { catch_cxx(note_nparams, || 5u32) }.ok(), Some(5));
        assert_eq!(chain::depth(), 0);
    }
    force(0);
}

/// A non-C++ exception is not caught by the boundary: it reaches the catching E frame
/// outside it.
#[test]
fn host_boundary_passes_other_codes() {
    use crate::eh_rt::host_try::catch_cxx;
    setup();
    const CODE: u32 = 0xE0AA_0B07;
    let mut img = image();
    let c = e_frame(&mut img, 1, ACT_CATCH, CODE, 0);
    let t = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    img.finish();
    let raise = raise_va();
    // E(catch) -> lifted-style Rust callee (via call_va8) -> boundary -> E(throw CODE).
    unsafe extern "win64" fn bounded(plan: u64) -> u64 {
        unsafe {
            let p = plan as *const Plan;
            match catch_cxx(note_nparams, || {
                crate::emu::emu_run_full_rax((*p).target, [(*p).next, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], 0).0
            }) {
                Ok(v) => v,
                Err(_) => 0xBAD,
            }
        }
    }
    let b = native_va(bounded as *const () as usize);
    for mode in MODES {
        force(mode);
        log_take();
        let p = plans(3);
        p[2] = Plan { code: CODE as u64, raise, ..Default::default() };
        p[1] = Plan { target: t.va, next: pp(&p[2]), ..Default::default() };
        p[0] = Plan { target: b, next: pp(&p[1]), ..Default::default() };
        let r = run_e(c.va, &p[0]);
        let log = log_take();
        assert_eq!(r, RETVAL, "mode {mode}: {log:#x?}");
        assert_eq!(phases(&log), vec![(3, D, CODE), (1, D, CODE), (3, U, CODE), (1, T, CODE)], "mode {mode}");
        assert_eq!(chain::depth(), 0, "mode {mode}");
    }
    force(0);
}

// ---------------------------------------------------------------------------
// setjmp / longjmp
// ---------------------------------------------------------------------------

/// A longjmp's inputs (guest code reads the offsets).
#[repr(C)]
struct Lj {
    /// +0x00: the `_JUMP_BUFFER` (16-aligned, 256 bytes).
    jb: u64,
    /// +0x08: the longjmp value.
    value: u64,
    /// +0x10: `RtlUnwind` VA.
    unwind: u64,
    /// +0x18: the guest `setjmp` VA (E setjmp frames).
    setjmp: u64,
    /// +0x20: the `STATUS_LONGJMP` record (`ExceptionInformation[0]` = jb, set by the longjmp).
    rec: ExceptionRecord,
}

fn lj(value: u64, setjmp: u64) -> &'static mut Lj {
    #[repr(C, align(16))]
    struct Jb([u8; 256]);
    let jb = Box::leak(Box::new(Jb([0; 256]))) as *mut Jb as u64;
    let mut information = [0u64; 15];
    information[0] = jb;
    Box::leak(Box::new(Lj {
        jb,
        value,
        unwind: native_va(win32_rt::unwind::RtlUnwind as *const () as usize),
        setjmp,
        rec: ExceptionRecord {
            code: STATUS_LONGJMP,
            flags: 0,
            record: core::ptr::null_mut(),
            address: core::ptr::null_mut(),
            number_parameters: 1,
            _pad: 0,
            information,
        },
    }))
}

/// vcruntime's `longjmp` with a frame (rcx = [`Lj`]): `RtlUnwind(jb.Frame, jb.Rip,
/// {STATUS_LONGJMP, jb}, value)`.
fn e_longjmp(img: &mut TestImage) -> u64 {
    let mut a = CodeAssembler::new(64).unwrap();
    a.sub(rsp, 0x28).unwrap();
    a.mov(r10, rcx).unwrap();
    a.mov(rax, qword_ptr(r10)).unwrap();
    a.mov(qword_ptr(r10 + 0x40), rax).unwrap();
    a.mov(rcx, qword_ptr(rax)).unwrap();
    a.mov(rdx, qword_ptr(rax + 0x50)).unwrap();
    a.lea(r8, qword_ptr(r10 + 0x20)).unwrap();
    a.mov(r9, qword_ptr(r10 + 8)).unwrap();
    a.call(qword_ptr(r10 + 0x10)).unwrap();
    a.int3().unwrap();
    img.func(a, |_| Unwind { ops: vec![(4, Op::Alloc(0x28))], handler: false, data: Vec::new() }).0
}

/// vcruntime's `__intrinsic_setjmp` (rcx = buffer, rdx = frame; x87 control word left out,
/// MxCsr the default).
fn e_setjmp(img: &mut TestImage) -> u64 {
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(qword_ptr(rcx), rdx).unwrap();
    a.mov(qword_ptr(rcx + 0x08), rbx).unwrap();
    a.mov(qword_ptr(rcx + 0x18), rbp).unwrap();
    a.mov(qword_ptr(rcx + 0x20), rsi).unwrap();
    a.mov(qword_ptr(rcx + 0x28), rdi).unwrap();
    a.mov(qword_ptr(rcx + 0x30), r12).unwrap();
    a.mov(qword_ptr(rcx + 0x38), r13).unwrap();
    a.mov(qword_ptr(rcx + 0x40), r14).unwrap();
    a.mov(qword_ptr(rcx + 0x48), r15).unwrap();
    a.lea(r8, qword_ptr(rsp + 8)).unwrap();
    a.mov(qword_ptr(rcx + 0x10), r8).unwrap();
    a.mov(r8, qword_ptr(rsp)).unwrap();
    a.mov(qword_ptr(rcx + 0x50), r8).unwrap();
    a.mov(dword_ptr(rcx + 0x58), 0x1F80).unwrap();
    for (k, x) in [xmm6, xmm7, xmm8, xmm9, xmm10, xmm11, xmm12, xmm13, xmm14, xmm15].into_iter().enumerate() {
        a.movdqa(xmmword_ptr(rcx + 0x60 + 16 * k as i32), x).unwrap();
    }
    a.xor(eax, eax).unwrap();
    a.ret().unwrap();
    img.func(a, |_| Unwind::leaf()).0
}

/// E frame `id` (handler: the mock) calling setjmp (plan.args = [`Lj`]); on the 0 return
/// rbx is clobbered and it calls `target(next)`; the exit writes rbx / rax into the plan.
fn e_setjmp_frame(img: &mut TestImage, id: u32) -> u64 {
    let mut a = CodeAssembler::new(64).unwrap();
    let mut out = a.create_label();
    a.push(rbx).unwrap();
    a.sub(rsp, 0x30).unwrap();
    a.mov(qword_ptr(rsp + 0x40), rcx).unwrap();
    a.mov(qword_ptr(rcx + 88), rsp).unwrap();
    a.mov(rbx, RBX_BASE | id as u64).unwrap();
    a.mov(r10, qword_ptr(rcx + 112)).unwrap();
    a.mov(rcx, qword_ptr(r10)).unwrap();
    a.mov(rdx, rsp).unwrap();
    a.call(qword_ptr(r10 + 0x18)).unwrap();
    a.mov(rcx, qword_ptr(rsp + 0x40)).unwrap();
    a.test(eax, eax).unwrap();
    a.jnz(out).unwrap();
    a.mov(rbx, 0xC10B_u64).unwrap();
    a.mov(rax, qword_ptr(rcx)).unwrap();
    a.mov(rcx, qword_ptr(rcx + 8)).unwrap();
    a.call(rax).unwrap();
    a.mov(eax, 1).unwrap();
    a.set_label(&mut out).unwrap();
    a.mov(rcx, qword_ptr(rsp + 0x40)).unwrap();
    a.mov(qword_ptr(rcx + 32), rbx).unwrap();
    a.mov(qword_ptr(rcx + 40), rax).unwrap();
    a.add(rsp, 0x30).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();
    img.func(a, |_| Unwind {
        ops: vec![(1, Op::Push(3)), (5, Op::Alloc(0x30))],
        handler: true,
        data: spec(id, ACT_NONE, 0, 0, 0),
    })
    .0
}

/// Offsets in a lifted setjmp fn's phantom range: the setjmp call, its return address (the
/// continuation) and the catching call.
const SJ_SITE: u64 = 0x08;
const SJ_CONT: u64 = 0x0D;
const SJ_CALL: u64 = 0x10;
const XMM6: u128 = 0x6666_0000_0000_0006 | (0x6767_0000_0000_0007 << 64);

/// A lifted fn with a setjmp in generated-code style: `EhFrame::setjmp` in place of the call,
/// a catching call after it, and the continuation's landing arm restoring the body's locals
/// from `longjmp_regs`. Exit: plan.rax = the setjmp result, rbx / local (r12) / nv + land
/// (xmm6 halves) = the locals at the continuation.
#[allow(clippy::too_many_arguments)]
#[inline(never)]
unsafe extern "win64" fn lifted_setjmp(plan: u64, _rdx: u64, _r8: u64, _r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
    use crate::dll_native::lift::eh::longjmp_regs;
    unsafe {
        let p = plan as *mut Plan;
        let meta: &'static EhMeta = &*((*p).meta as *const EhMeta);
        let mut stack = LiftedFrame::<256, 16>::new();
        let entry_rsp = init_lifted_stack(&mut stack, a5, a6, a7, a8);
        let mut eh = EhFrame::new(meta, entry_rsp);
        let _eh = eh.push();
        let body_sp = meta.establisher(entry_rsp);
        (*p).est = body_sp;
        let l = (*p).args as *const Lj;
        // Body locals (iced's register names are constants in this module).
        let (mut g_bx, mut g_12, mut g_x6) = (RBX_BASE | (*p).id, 0x1212_u64, XMM6);
        let mut g_ax = eh.setjmp(meta.va + SJ_SITE, (*l).jb, body_sp, meta.va + SJ_CONT, body_sp, [g_bx, 0, 0, 0, g_12, 0, 0, 0], [g_x6, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        loop {
            // The continuation block.
            if g_ax != 0 {
                (*p).rax = g_ax;
                (*p).rbx = g_bx;
                (*p).local = g_12;
                (*p).nv = g_x6 as u64;
                (*p).land = (g_x6 >> 64) as u64;
                return 0x5E7;
            }
            g_bx = 0xC10B;
            g_12 = 0xC10C;
            g_x6 = 0;
            g_ax = match eh.call(meta.va + SJ_CALL, (*p).target, (*p).next, 0, 0, 0, 0, 0, 0, 0) {
                Ok(v) => v,
                Err(l) => match l.cont_va {
                    c if c == meta.va + SJ_CONT => {
                        let j = longjmp_regs();
                        g_ax = j.gpr[0];
                        g_bx = j.gpr[3];
                        g_12 = j.gpr[12];
                        g_x6 = j.xmm[0];
                        continue;
                    }
                    _ => l.unknown(meta.va),
                },
            };
            core::hint::black_box((g_bx, g_12, g_x6));
            return g_ax;
        }
    }
}

/// L(setjmp, no language handler) -> L(cleanup) -> E(longjmp): the unwind runs L(cleanup)'s
/// handler with STATUS_LONGJMP (no dispatch phase), stops at the setjmp frame (its record's
/// establisher = the buffer's Frame) and resumes at the continuation with rax = the value and
/// the locals the buffer saved; chain and lstack are back.
#[test]
fn longjmp_lands_in_lifted_setjmp() {
    setup();
    const VALUE: u64 = 0x77;
    let mut img = image();
    let lj_va = e_longjmp(&mut img);
    let m = l_frame(&mut img, 2, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    let sva = img.phantom(0x40, |_| Unwind { ops: vec![(4, Op::Alloc(0x28))], handler: false, data: Vec::new() });
    img.finish();
    let sm: &'static EhMeta = Box::leak(Box::new(EhMeta {
        va: sva,
        funcinfo_rva: 0,
        handler: HANDLER_NONE,
        shape: SHAPE_EMPTY,
        frame_reg: 0,
        establisher_off: 0x28,
        conts: &[],
    }));
    let lv = lifted_va();
    for mode in MODES {
        force(mode);
        log_take();
        let l = lj(VALUE, 0);
        let p = plans(2);
        p[1] = Plan { target: lj_va, next: l as *mut Lj as u64, meta: m as *const EhMeta as u64, id: 2, ..Default::default() };
        p[0] = Plan { target: lv, next: pp(&p[1]), meta: sm as *const EhMeta as u64, id: 1, args: l as *mut Lj as u64, ..Default::default() };
        let ls0 = ls_top();
        let r = unsafe { lifted_setjmp(pp(&p[0]), 0, 0, 0, 0, 0, 0, 0) };
        let log = log_take();
        assert_eq!(r, 0x5E7, "mode {mode}: {log:#x?}");
        assert_eq!(p[0].rax, VALUE, "mode {mode}: rax = the longjmp value");
        assert_eq!(p[0].rbx, RBX_BASE | 1, "mode {mode}: rbx from the buffer");
        assert_eq!(p[0].local, 0x1212, "mode {mode}: r12 from the buffer");
        assert_eq!((p[0].nv, p[0].land), (XMM6 as u64, (XMM6 >> 64) as u64), "mode {mode}: xmm6 from the buffer");
        let jb = |i: u64| unsafe { ((l.jb + 8 * i) as *const u64).read() };
        assert_eq!((jb(0), jb(2), jb(10)), (p[0].est, p[0].est, sva + SJ_CONT), "mode {mode}: Frame / Rsp / Rip");
        assert_eq!(phases(&log), vec![(2, U, STATUS_LONGJMP)], "mode {mode}");
        assert_eq!(visit(&log, 2, U).frame, p[1].est, "mode {mode}");
        assert_eq!(chain::depth(), 0, "mode {mode}");
        assert_eq!(ls_top(), ls0, "mode {mode}");
    }
    force(0);
}

/// E(setjmp) -> L(cleanup) -> E(longjmp): the guest frame resumes after its setjmp call with
/// rax = the value and rbx from the buffer (emulated region landing); its handler sees the
/// target unwind.
#[test]
fn longjmp_lands_in_guest_setjmp() {
    setup();
    const VALUE: u64 = 0x79;
    let mut img = image();
    let lj_va = e_longjmp(&mut img);
    let sj_va = e_setjmp(&mut img);
    let f = e_setjmp_frame(&mut img, 1);
    let m = l_frame(&mut img, 2, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    img.finish();
    let lv = lifted_va();
    for mode in MODES {
        force(mode);
        log_take();
        let l = lj(VALUE, sj_va);
        let p = plans(2);
        p[1] = Plan { target: lj_va, next: l as *mut Lj as u64, meta: m as *const EhMeta as u64, id: 2, ..Default::default() };
        p[0] = Plan { target: lv, next: pp(&p[1]), args: l as *mut Lj as u64, ..Default::default() };
        let ls0 = ls_top();
        let r = run_e(f, &p[0]);
        let log = log_take();
        assert_eq!(r, VALUE, "mode {mode}: {log:#x?}");
        assert_eq!(p[0].rbx, RBX_BASE | 1, "mode {mode}: rbx from the buffer");
        assert_eq!(phases(&log), vec![(2, U, STATUS_LONGJMP), (1, T, STATUS_LONGJMP)], "mode {mode}");
        assert_eq!(visit(&log, 1, T).frame, p[0].est, "mode {mode}");
        assert_eq!(chain::depth(), 0, "mode {mode}");
        assert_eq!(ls_top(), ls0, "mode {mode}");
    }
    force(0);
}

// ---------------------------------------------------------------------------
// Stack backtrace over the chain
// ---------------------------------------------------------------------------

/// E frame calling `RtlCaptureStackBackTrace(0, 8, rcx, 0)` (rcx = the output buffer);
/// returns its VA and the return VA of that call.
fn e_backtrace(img: &mut TestImage) -> (u64, u64) {
    let bt = native_va(win32_rt::unwind::RtlCaptureStackBackTrace as *const () as usize);
    let mut a = CodeAssembler::new(64).unwrap();
    let mut body = a.create_label();
    let mut ret = a.create_label();
    a.sub(rsp, 0x28).unwrap();
    a.set_label(&mut body).unwrap();
    a.mov(r8, rcx).unwrap();
    a.xor(ecx, ecx).unwrap();
    a.mov(edx, 8).unwrap();
    a.xor(r9d, r9d).unwrap();
    a.mov(rax, bt).unwrap();
    a.call(rax).unwrap();
    a.set_label(&mut ret).unwrap();
    a.add(rsp, 0x28).unwrap();
    a.ret().unwrap();
    let (va, r) = img.func(a, |_| Unwind { ops: vec![(4, Op::Alloc(0x28))], handler: false, data: Vec::new() });
    assert_eq!(r.label_ip(&body).unwrap() - va, 4, "sub rsp, 0x28 is 4 bytes");
    (va, r.label_ip(&ret).unwrap())
}

/// `RtlCaptureStackBackTrace` called by guest code under E -> L -> E walks the record chain
/// as a dispatch does: the inner region frame by frame, past its `RET_SENTINEL` (never
/// reported) to the lifted frame's record, reported at the end of its in-flight call
/// instruction (its return address), then the outer region at its call's return address;
/// the Rust frames in between are invisible. The chain's end ends the trace.
#[test]
fn backtrace_walks_regions_and_lifted_records() {
    setup();
    let mut img = image();
    let (bt_va, bt_ret) = e_backtrace(&mut img);
    let outer = e_frame(&mut img, 1, ACT_NONE, 0, 0);
    let m = l_frame(&mut img, 2, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    img.finish();
    // A real call instruction at L's call site: `call r11` (3 bytes).
    unsafe { core::ptr::copy_nonoverlapping([0x41u8, 0xFF, 0xD3].as_ptr(), (m.va + L_SITE) as *mut u8, 3) };
    let lv = lifted_va();
    for mode in MODES {
        force(mode);
        log_take();
        let buf: &'static mut [u64; 8] = Box::leak(Box::new([0u64; 8]));
        let p = plans(2);
        p[1] = Plan { target: bt_va, next: buf.as_mut_ptr() as u64, meta: m as *const EhMeta as u64, id: 2, ..Default::default() };
        p[0] = Plan { target: lv, next: pp(&p[1]), ..Default::default() };
        let ls0 = ls_top();
        run_e(outer.va, &p[0]);
        assert_eq!(
            buf[..4],
            [bt_ret, m.va + L_SITE + 3, outer.call_ret, 0],
            "mode {mode}: inner E (its call's return), L (end of its call at the state IP), outer E; no sentinel, nothing past the chain"
        );
        assert!(log_take().is_empty(), "mode {mode}: a backtrace calls no handler");
        assert_eq!(chain::depth(), 0, "mode {mode}");
        assert_eq!(ls_top(), ls0, "mode {mode}");
    }
    force(0);
}

// ---------------------------------------------------------------------------
// L5 (T4, the exe): `exe::try_call` / `call_va8_caught` on the chain
// ---------------------------------------------------------------------------

/// An MSVC C++ throw's `ExceptionInformation` over a hand-built `ThrowInfo` (one trivially
/// copyable catchable type `name` of `size` bytes, no destructor; all RVAs from `base`).
struct FakeThrow {
    ti: u64,
    obj: u64,
    args: [u64; 4],
}

fn fake_cxx(name: &[u8], size: i32, value: u64) -> &'static FakeThrow {
    let buf: &'static mut [u64; 64] = Box::leak(Box::new([0u64; 64]));
    let base = buf.as_mut_ptr() as u64;
    let w32 = |off: u64, v: i32| unsafe { ((base + off) as *mut i32).write_unaligned(v) };
    // ThrowInfo { attributes, pmfnUnwind, pForwardCompat, pCatchableTypeArray } at +0x00.
    w32(0x0c, 0x20);
    // CatchableTypeArray { 1, [+0x40] } at +0x20.
    w32(0x20, 1);
    w32(0x24, 0x40);
    // CatchableType { properties = simple type, pType = +0x80, PMD { 0, -1, 0 }, size, copy = 0 }.
    w32(0x40, 1);
    w32(0x44, 0x80);
    w32(0x4c, -1);
    w32(0x54, size);
    // TypeDescriptor { vftable, spare, name } at +0x80.
    unsafe { core::ptr::copy_nonoverlapping(name.as_ptr(), (base + 0x90) as *mut u8, name.len()) };
    // The object at +0x100.
    unsafe { ((base + 0x100) as *mut u64).write(value) };
    Box::leak(Box::new(FakeThrow { ti: base, obj: base + 0x100, args: [0x1993_0520, base + 0x100, base, base] }))
}

const OBJ_VALUE: u64 = 0x0B1E_C700_DEAD_BEEF;

/// `exe::try_call` (a hand-written `Throws::Caught` replacement's call) -> L(cleanup) -> E ->
/// L(throw C++ `TestErr`): the try boundary catches with a matching catch list after every
/// frame's cleanup ran, copies the object (dispatch phase) and leaves the chain and lstack as
/// they were. With a catch list that does not match, the exception passes the boundary and
/// reaches the next one above it.
#[test]
fn exe_try_call_catches_matching_cxx_after_cleanup() {
    use crate::eh_rt::host_try::catch_cxx;
    setup();
    const CXX: u32 = win32_rt::unwind::CXX_EXCEPTION;
    let mut img = image();
    let e = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    let mc = l_frame(&mut img, 2, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    let mr = l_frame(&mut img, 4, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    img.finish();
    let lv = lifted_va();
    let ft = fake_cxx(b".?AVTestErr@@", 8, OBJ_VALUE);
    for mode in MODES {
        force(mode);
        for matching in [true, false] {
            log_take();
            let p = plans(3);
            p[2] = Plan { code: CXX as u64, meta: mr as *const EhMeta as u64, id: 4, nargs: 4, args: ft.args.as_ptr() as u64, ..Default::default() };
            p[1] = Plan { target: lv, next: pp(&p[2]), ..Default::default() };
            p[0] = Plan { target: e.va, next: pp(&p[1]), meta: mc as *const EhMeta as u64, id: 2, ..Default::default() };
            let ls0 = ls_top();
            let list: &[&[u8]] = if matching { &[b".?AVOther@@", b".?AVTestErr@@"] } else { &[b".?AVOther@@"] };
            let outer = unsafe { catch_cxx(note_nparams, || crate::exe::try_call(lv as usize, &[pp(&p[0])], list)) };
            let log = log_take();
            assert_eq!(
                phases(&log),
                vec![(4, D, CXX), (3, D, CXX), (2, D, CXX), (4, U, CXX), (3, U, CXX), (2, U, CXX)],
                "mode {mode} matching {matching}"
            );
            match (matching, outer) {
                (true, Ok(Err(x))) => {
                    assert_eq!((x.throw_info, x.object, x.matched, x.size), (ft.ti, ft.obj, 1, 8), "mode {mode}");
                    assert_eq!(x.value[..8], OBJ_VALUE.to_le_bytes(), "mode {mode}: object copied in the dispatch phase");
                }
                (false, Err(c)) => assert_eq!((c.rec.code, c.note), (CXX, [0x7E57, 4]), "mode {mode}: reached the outer boundary"),
                (m, r) => panic!("mode {mode} matching {m}: {:?}: {log:#x?}", r.map(|r| r.map(|v| v.rax).map_err(|x| x.matched)).map_err(|c| c.rec.code)),
            }
            assert_eq!(chain::depth(), 0, "mode {mode} matching {matching}: boundaries popped");
            assert_eq!(ls_top(), ls0, "mode {mode} matching {matching}: lstack top restored");
        }
        // No throw: the callee's result.
        let q = plans(1);
        q[0] = Plan { target: native_va(ret7 as *const () as usize), meta: mc as *const EhMeta as u64, id: 2, ..Default::default() };
        let r = unsafe { crate::exe::try_call(lv as usize, &[pp(&q[0])], &[]) };
        assert_eq!(r.map(|v| v.rax).ok(), Some(7), "mode {mode}");
        assert_eq!(chain::depth(), 0);
    }
    force(0);
}

/// `exe::try_call` of guest code (the `aot_va` fallback of an exe function that is not lifted):
/// the callee runs as a transparent emulator region, so a C++ throw below it reaches the try
/// boundary (`catch (...)`) instead of stopping at an opaque region.
#[test]
fn exe_try_call_catch_all_through_guest_entry() {
    setup();
    const CXX: u32 = win32_rt::unwind::CXX_EXCEPTION;
    let mut img = image();
    let e = e_frame(&mut img, 3, ACT_NONE, 0, 0);
    let mr = l_frame(&mut img, 4, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    img.finish();
    let lv = lifted_va();
    let ft = fake_cxx(b".?AVTestErr@@", 8, OBJ_VALUE);
    for mode in MODES {
        force(mode);
        log_take();
        let p = plans(2);
        p[1] = Plan { code: CXX as u64, meta: mr as *const EhMeta as u64, id: 4, nargs: 4, args: ft.args.as_ptr() as u64, ..Default::default() };
        p[0] = Plan { target: lv, next: pp(&p[1]), ..Default::default() };
        let ls0 = ls_top();
        let r = unsafe { crate::exe::try_call(e.va as usize, &[pp(&p[0])], &[]) };
        let log = log_take();
        match r {
            Err(x) => assert_eq!((x.matched, x.size, x.value[..8].to_vec()), (u32::MAX, 8, OBJ_VALUE.to_le_bytes().to_vec()), "mode {mode}"),
            Ok(v) => panic!("mode {mode}: returned 0x{:x}: {log:#x?}", v.rax),
        }
        assert_eq!(phases(&log), vec![(4, D, CXX), (3, D, CXX), (4, U, CXX), (3, U, CXX)], "mode {mode}");
        assert_eq!(chain::depth(), 0, "mode {mode}");
        assert_eq!(ls_top(), ls0, "mode {mode}");
    }
    force(0);
}

mod fh4;
