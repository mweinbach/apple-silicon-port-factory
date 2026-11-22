//! L4a (target/review/exec/g12/t3_test_plan.md): the Rust `__CxxFrameHandler4` port over the
//! record chain, with hand-built FuncInfo4 tables for lifted frames.
use super::*;
use win32_rt::misc::vcruntime140_1::__CxxFrameHandler4;
use win32_rt::unwind::CXX_EXCEPTION;
use core::sync::atomic::{AtomicU64, Ordering};

// ---------------------------------------------------------------------------
// FH4 table encoding
// ---------------------------------------------------------------------------

/// FH4 compressed unsigned (`FH4::ReadUnsigned`): 1 byte below 2^7, 2 bytes below 2^14.
fn cu(v: u32) -> Vec<u8> {
    if v < 0x80 {
        vec![(v << 1) as u8]
    } else {
        assert!(v < 0x4000);
        let x = (v << 2) | 1;
        vec![x as u8, (x >> 8) as u8]
    }
}

/// IP-to-state map: `(offset from fn begin where the state starts, state)` in order.
fn ip_map(entries: &[(u32, i32)]) -> Vec<u8> {
    let mut v = cu(entries.len() as u32);
    let mut prev = 0;
    for &(off, st) in entries {
        v.extend(cu(off - prev));
        v.extend(cu((st + 1) as u32));
        prev = off;
    }
    v
}

/// Unwind map: state `i` unwinds to state `i - 1`, running a type-1 entry (`DtorWithObj`:
/// `(dtor RVA, object offset)`) or nothing (type 0, e.g. a try block's entry state).
fn unwind_map(entries: &[Option<(u32, u32)>]) -> Vec<u8> {
    let mut v = cu(entries.len() as u32);
    for e in entries {
        match *e {
            Some((dtor, obj)) => {
                v.extend(cu((1 << 2) | 1));
                v.extend((dtor as i32).to_le_bytes());
                v.extend(cu(obj));
            }
            None => v.extend(cu(1 << 2)),
        }
    }
    v
}

/// One try block `[low, high]` (catch states up to `catch_high`) with one `catch(...)` handler
/// whose continuation is `fn begin + cont_off`.
fn try_map(img: &mut TestImage, low: u32, high: u32, catch_high: u32, funclet_rva: u32, cont_off: u32) -> Vec<u8> {
    let mut h = cu(1);
    h.push(0x10); // continuation count 1, function-relative; no type: catch(...)
    h.extend((funclet_rva as i32).to_le_bytes());
    h.extend(cu(cont_off));
    let handlers = img.blob(&h);
    let mut v = cu(1);
    v.extend(cu(low));
    v.extend(cu(high));
    v.extend(cu(catch_high));
    v.extend((img.rva(handlers) as i32).to_le_bytes());
    v
}

/// FuncInfo4 (not separated, not a catch funclet); returns its RVA.
fn func_info(img: &mut TestImage, unwind: Option<Vec<u8>>, tries: Option<Vec<u8>>, ips: Vec<u8>) -> u32 {
    let ip_va = img.blob(&ips);
    let ip_rva = img.rva(ip_va);
    let uw_rva = unwind.map(|u| {
        let va = img.blob(&u);
        img.rva(va)
    });
    let tr_rva = tries.map(|t| {
        let va = img.blob(&t);
        img.rva(va)
    });
    let mut fi = vec![(if uw_rva.is_some() { 0x08 } else { 0 }) | (if tr_rva.is_some() { 0x10 } else { 0 })];
    if let Some(r) = uw_rva {
        fi.extend((r as i32).to_le_bytes());
    }
    if let Some(r) = tr_rva {
        fi.extend((r as i32).to_le_bytes());
    }
    fi.extend((ip_rva as i32).to_le_bytes());
    let fi_va = img.blob(&fi);
    img.rva(fi_va)
}

/// A lifted fn's `EhMeta` whose UNWIND_INFO names `__CxxFrameHandler4` (the image's handler)
/// with FuncInfo4 `fi_rva`.
fn l_fh4(img: &mut TestImage, fi_rva: u32, shape: u8) -> &'static EhMeta {
    let va = img.phantom(0x40, |_| Unwind {
        ops: vec![(4, Op::Alloc(0x28))],
        handler: true,
        data: (fi_rva as i32).to_le_bytes().to_vec(),
    });
    Box::leak(Box::new(EhMeta {
        va,
        funcinfo_rva: fi_rva,
        handler: HANDLER_FH4,
        shape,
        frame_reg: 0,
        establisher_off: 0x28,
        conts: &[],
    }))
}

/// A thrown `int` (MSVC `ThrowInfo` with one simple catchable type) and the
/// `RaiseException` arguments of `throw 7`.
fn cxx_throw(img: &mut TestImage) -> &'static [u64; 4] {
    let mut td = vec![0u8; 16];
    td.extend_from_slice(b".H\0\0");
    let td_va = img.blob(&td);
    let mut ct = Vec::new();
    ct.extend(1u32.to_le_bytes()); // simple type
    ct.extend((img.rva(td_va) as i32).to_le_bytes());
    for x in [0i32, -1, 0, 4, 0] {
        ct.extend(x.to_le_bytes()); // PMD {0, -1, 0}, size 4, no copy ctor
    }
    let ct_va = img.blob(&ct);
    let mut cta = 1i32.to_le_bytes().to_vec();
    cta.extend((img.rva(ct_va) as i32).to_le_bytes());
    let cta_va = img.blob(&cta);
    let mut ti = vec![0u8; 12];
    ti.extend((img.rva(cta_va) as i32).to_le_bytes());
    let ti_va = img.blob(&ti);
    let obj = Box::leak(Box::new(7u32)) as *mut u32 as u64;
    Box::leak(Box::new([0x1993_0520, obj, ti_va, img.base]))
}

std::thread_local! {
    /// (this, establisher) of every test destructor call.
    static DTORS: RefCell<Vec<(u64, u64)>> = const { RefCell::new(Vec::new()) };
    /// (rcx, rdx) of every catch funclet call.
    static FUNCLETS: RefCell<Vec<(u64, u64)>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn dtor(this: u64, est: u64) -> u64 {
    DTORS.with(|d| d.borrow_mut().push((this, est)));
    0
}

/// `catch(...) {}`: returns 0 (= the handler's first continuation).
unsafe extern "win64" fn funclet(a1: u64, a2: u64) -> u64 {
    FUNCLETS.with(|f| f.borrow_mut().push((a1, a2)));
    0
}

/// `catch(...) { throw; }`.
unsafe extern "win64" fn funclet_rethrow(a1: u64, a2: u64) -> u64 {
    FUNCLETS.with(|f| f.borrow_mut().push((a1, a2)));
    static RETHROW: [u64; 4] = [0x1993_0520, 0, 0, 0];
    unsafe { RaiseException(CXX_EXCEPTION as u64, EXCEPTION_NONCONTINUABLE as u64, 4, RETHROW.as_ptr() as u64, 0, 0) };
    unreachable!("rethrow continued")
}

fn fh4_image() -> TestImage {
    TestImage::new(native_va(__CxxFrameHandler4 as *const () as usize))
}

/// Shared shape: `outer` -> L(FH4 `m`) -> E(no catch) -> L(raise `throw 7`). Returns the
/// plans (p[1] is the FH4 frame's).
fn fh4_plans(outer_target: u64, m: &'static EhMeta, e: &EFn, raise_meta: &'static EhMeta, args: &'static [u64; 4], site: u64) -> &'static mut [Plan] {
    let lv = lifted_va();
    let p = plans(4);
    p[3] = Plan {
        code: CXX_EXCEPTION as u64,
        meta: raise_meta as *const EhMeta as u64,
        id: 9,
        nargs: 4,
        args: args.as_ptr() as u64,
        ..Default::default()
    };
    p[2] = Plan { target: lv, next: pp(&p[3]), ..Default::default() };
    p[1] = Plan { target: e.va, next: pp(&p[2]), meta: m as *const EhMeta as u64, id: 2, site, ..Default::default() };
    p[0] = Plan { target: outer_target, next: pp(&p[1]), ..Default::default() };
    p
}

/// A thrown C++ exception crosses L(cleanup): its destructor runs once with
/// `this = establisher + off` and `rdx = establisher`; an outer E catches.
#[test]
fn fh4_dtor_runs_with_establisher_plus_off() {
    setup();
    let mut mimg = image();
    let c = e_frame(&mut mimg, 1, ACT_CATCH, CXX_EXCEPTION, 0);
    let e = e_frame(&mut mimg, 3, ACT_NONE, 0, 0);
    let raise_m = l_frame(&mut mimg, 9, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    mimg.finish();
    let mut img = fh4_image();
    let d = img.stub(native_va(dtor as *const () as usize));
    let uw = unwind_map(&[Some((img.rva(d), 0x20))]);
    let fi = func_info(&mut img, Some(uw), None, ip_map(&[(0x08, 0), (0x28, -1)]));
    let m = l_fh4(&mut img, fi, SHAPE_CLEANUP);
    let args = cxx_throw(&mut img);
    img.finish();
    for mode in MODES {
        force(mode);
        log_take();
        DTORS.with(|d| d.borrow_mut().clear());
        let p = fh4_plans(lifted_va(), m, &e, raise_m, args, 0);
        let ls0 = ls_top();
        let r = run_e(c.va, &p[0]);
        let log = log_take();
        assert_eq!(r, RETVAL, "mode {mode}: {log:#x?}");
        let dt = DTORS.with(|d| d.borrow_mut().split_off(0));
        assert_eq!(dt, vec![(p[1].est + 0x20, p[1].est)], "mode {mode}: one dtor, this = establisher + 0x20");
        assert_eq!(chain::depth(), 0);
        assert_eq!(ls_top(), ls0);
    }
    force(0);
}

/// `catch(...)` in a lifted frame: the funclet runs with `rdx = establisher` and the lifted
/// body resumes at the handler's continuation.
#[test]
fn fh4_catch_all_funclet_runs_and_continues() {
    setup();
    let mut mimg = image();
    let e = e_frame(&mut mimg, 3, ACT_NONE, 0, 0);
    let raise_m = l_frame(&mut mimg, 9, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    mimg.finish();
    let mut img = fh4_image();
    let f = img.stub(native_va(funclet as *const () as usize));
    let f_rva = img.rva(f);
    let tries = try_map(&mut img, 0, 0, 1, f_rva, L_CONT as u32);
    let fi = func_info(&mut img, None, Some(tries), ip_map(&[(0x08, 0), (0x28, -1)]));
    let m = l_fh4(&mut img, fi, SHAPE_TRY);
    let args = cxx_throw(&mut img);
    img.finish();
    for mode in MODES {
        force(mode);
        FUNCLETS.with(|f| f.borrow_mut().clear());
        let p = fh4_plans(0, m, &e, raise_m, args, 0);
        let ls0 = ls_top();
        let r = unsafe { lifted(pp(&p[1]), 0, 0, 0, 0, 0, 0, 0) };
        assert_eq!(r, 0x1A4D, "mode {mode}: landed");
        assert_eq!(p[1].land, m.va + L_CONT, "mode {mode}: the handler's continuation");
        assert_eq!(p[1].local, LOCAL_BASE | 2);
        let fl = FUNCLETS.with(|f| f.borrow_mut().split_off(0));
        assert_eq!(fl, vec![(f, p[1].est)], "mode {mode}: funclet(rcx = its VA, rdx = establisher)");
        assert_eq!(chain::depth(), 0);
        assert_eq!(ls_top(), ls0);
    }
    force(0);
}

/// The state of a lifted frame comes from its call site (state IP): two sites in different
/// states unwind different entries of the unwind map.
#[test]
fn fh4_state_ip_selects_state() {
    setup();
    let mut mimg = image();
    let c = e_frame(&mut mimg, 1, ACT_CATCH, CXX_EXCEPTION, 0);
    let e = e_frame(&mut mimg, 3, ACT_NONE, 0, 0);
    let raise_m = l_frame(&mut mimg, 9, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    mimg.finish();
    let mut img = fh4_image();
    let d = img.stub(native_va(dtor as *const () as usize));
    let uw = unwind_map(&[Some((img.rva(d), 0x10)), Some((img.rva(d), 0x18))]);
    let fi = func_info(&mut img, Some(uw), None, ip_map(&[(0x08, 0), (0x18, 1), (0x28, -1)]));
    let m = l_fh4(&mut img, fi, SHAPE_CLEANUP);
    let args = cxx_throw(&mut img);
    img.finish();
    for (site, want) in [(0x0Cu64, vec![0x10u64]), (0x1C, vec![0x18, 0x10]), (0x2C, vec![])] {
        DTORS.with(|d| d.borrow_mut().clear());
        let p = fh4_plans(lifted_va(), m, &e, raise_m, args, site);
        assert_eq!(run_e(c.va, &p[0]), RETVAL, "site +0x{site:x}");
        let dt: Vec<u64> = DTORS.with(|d| d.borrow_mut().split_off(0)).iter().map(|&(t, _)| t - p[1].est).collect();
        assert_eq!(dt, want, "site +0x{site:x}: destructed object offsets");
        assert_eq!(chain::depth(), 0);
    }
}

/// A throw leaving a lifted frame's `catch(...)` block (`throw;` or a new exception): the
/// same try does not catch it again, an outer E does, and each object of the frame is
/// destroyed once (the try's at the catch, the one before the try when the throw leaves).
/// Frame states: 0 = object A (+0x10), 1 = try entry, 2 = object B (+0x18) inside the try;
/// try [1, 2], catch state 3; the in-flight call is in state 2. In a child process: a
/// re-catch loop would not end.
fn throw_from_lifted_catch(test: &str, funclet_fn: usize) {
    setup();
    let mut mimg = image();
    let c = e_frame(&mut mimg, 1, ACT_CATCH, CXX_EXCEPTION, 0);
    let e = e_frame(&mut mimg, 3, ACT_NONE, 0, 0);
    let raise_m = l_frame(&mut mimg, 9, ACT_NONE, 0, 0, SHAPE_CLEANUP);
    mimg.finish();
    let mut img = fh4_image();
    let d = img.stub(native_va(dtor as *const () as usize));
    let f = img.stub(native_va(funclet_fn));
    let (d_rva, f_rva) = (img.rva(d), img.rva(f));
    let uw = unwind_map(&[Some((d_rva, 0x10)), None, Some((d_rva, 0x18))]);
    let tries = try_map(&mut img, 1, 2, 3, f_rva, L_CONT as u32);
    let fi = func_info(&mut img, Some(uw), Some(tries), ip_map(&[(0x08, 0), (0x10, 1), (0x18, 2), (0x28, -1)]));
    let m = l_fh4(&mut img, fi, SHAPE_TRY);
    let args = cxx_throw(&mut img);
    let new_args = cxx_throw(&mut img);
    img.finish();
    NEW_THROW.store(new_args.as_ptr() as u64, Ordering::Relaxed);
    let p = fh4_plans(lifted_va(), m, &e, raise_m, args, 0x1C);
    let p0 = pp(&p[0]);
    let (st, log) = in_child(test, || {
        let r = run_e(c.va, unsafe { &*(p0 as *const Plan) });
        let n = FUNCLETS.with(|f| f.borrow().len());
        let est = unsafe { (*(p0 as *const Plan).add(1)).est };
        let dt: Vec<u64> = DTORS.with(|d| d.borrow().iter().map(|&(t, _)| t.wrapping_sub(est)).collect());
        eprintln!("r=0x{r:x} funclets={n} dtors={dt:x?}");
        let code = if n > 1 {
            0x21
        } else if r != RETVAL || n != 1 {
            0x22
        } else if dt != [0x18, 0x10] {
            0x23
        } else {
            0
        };
        unsafe { libc::_exit(code) };
    });
    assert_eq!(st, 0, "0x21 = caught again by the same try; 0x22 = not caught by the outer E; 0x23 = destructors\n{log}");
}

static NEW_THROW: AtomicU64 = AtomicU64::new(0);

/// `catch(...) { throw 7; }` (a new exception object).
unsafe extern "win64" fn funclet_throw(a1: u64, a2: u64) -> u64 {
    FUNCLETS.with(|f| f.borrow_mut().push((a1, a2)));
    let args = NEW_THROW.load(Ordering::Relaxed);
    unsafe { RaiseException(CXX_EXCEPTION as u64, EXCEPTION_NONCONTINUABLE as u64, 4, args, 0, 0) };
    unreachable!("throw continued")
}

#[test]
fn fh4_rethrow_from_lifted_catch_reaches_outer() {
    throw_from_lifted_catch(
        concat!(module_path!(), "::fh4_rethrow_from_lifted_catch_reaches_outer"),
        funclet_rethrow as *const () as usize,
    );
}

#[test]
fn fh4_new_throw_from_lifted_catch_reaches_outer() {
    throw_from_lifted_catch(
        concat!(module_path!(), "::fh4_new_throw_from_lifted_catch_reaches_outer"),
        funclet_throw as *const () as usize,
    );
}
