//! Difftests for the AW2 exe guest-entry routing (`exe_route`, `jit_guest_target`; see the
//! "AW2 exe guest entries" section in `mod.rs`). Run by the loader's `--diff-test`
//! (`AW2_DIFF_ONLY=exe_route`).
//!
//! Each case runs the same guest caller stub twice through [`emu_run_full`]:
//! - routing on (production): the exe import thunk or special-ABI helper is routed natively;
//! - routing off (`EXE_ROUTE_OFF`, the oracle): the exe's original x86 runs.
//! It compares `rax`/`xmm0` and written memory, and checks that only the routed run used
//! `exe_route`. All routed runs go first: an oracle run may leave a translation of the exe
//! block in this thread's JIT cache, and translated callers reach that directly.

use crate::difftest::{Mock, Snapshot, SKIP};
use crate::emu::{emu_run_full, to_mapped_addr, EXE_ROUTED, EXE_ROUTE_OFF};
use crate::exe::{rd, wr};

/// `j__Query_perf_frequency` -> `MSVCP140!_Query_perf_frequency` (guest PE DLL).
const THUNK_QPF: u64 = 0x1_4183_3bfd;
/// `j___std_exception_copy` -> `VCRUNTIME140!__std_exception_copy` (guest PE DLL).
const THUNK_EXC_COPY: u64 = 0x1_4183_ac92;

/// A difftest: name (matched by `AW2_DIFF_ONLY`) and body.
pub struct ExeRouteDiff {
    pub name: &'static str,
    pub run: unsafe fn() -> Result<u32, String>,
}

/// The exe-routing difftests (`--diff-test`).
pub fn difftests() -> Vec<ExeRouteDiff> {
    vec![
        ExeRouteDiff { name: "exe_route::import_thunk", run: diff_import_thunk },
        ExeRouteDiff { name: "exe_route::check_cookie", run: diff_check_cookie },
        ExeRouteDiff { name: "exe_route::guard_dispatch", run: diff_guard_dispatch },
    ]
}

/// Guest code bytes in fresh, leaked memory (unique addresses: no stale decode/translation).
fn guest_code(bytes: &[u8]) -> u64 {
    let mut v = bytes.to_vec();
    v.extend_from_slice(&[0xCC; 16]);
    Box::leak(v.into_boxed_slice()).as_ptr() as u64
}

/// `call qword ptr [rip+1]; ret; dq target`: calls `target` (an exe VA) indirectly.
fn call_stub(target: u64) -> u64 {
    let mut c = vec![0xFF, 0x15, 0x01, 0x00, 0x00, 0x00, 0xC3];
    c.extend_from_slice(&target.to_le_bytes());
    guest_code(&c)
}

/// Run `entry` with routing on (`off = false`) or off; returns `((rax, xmm0), routed count)`.
unsafe fn run(entry: u64, args: [u64; 8], off: bool) -> ((u64, u128), u64) {
    EXE_ROUTE_OFF.with(|c| c.set(off));
    let before = EXE_ROUTED.with(|c| c.get());
    let r = emu_run_full(entry, args, [0; 4], [0; 8]);
    let routed = EXE_ROUTED.with(|c| c.get()).wrapping_sub(before);
    EXE_ROUTE_OFF.with(|c| c.set(false));
    (r, routed)
}

/// Routing on vs off for each `(entry, args)`: same `(rax, xmm0)`, routed only when on.
/// `post` reads the side effects to compare after each run (`None` = no memory to compare).
/// `direct_ok`: the thunk's slot target is itself a native (`resolve_host_callable(thunk)`, e.g. a
/// lifted DLL function), so a guest call through the thunk may dispatch it directly without
/// reaching `exe_route`; a routed-on run that routed nothing is then accepted (still compared).
unsafe fn on_vs_off(
    what: &str,
    cases: &[(u64, [u64; 8])],
    direct_ok: bool,
    pre: &dyn Fn(usize),
    post: &dyn Fn(usize) -> Vec<u8>,
) -> Result<u32, String> {
    let mut on = Vec::with_capacity(cases.len());
    for (i, &(entry, args)) in cases.iter().enumerate() {
        pre(i);
        let (r, routed) = run(entry, args, false);
        if routed == 0 && !direct_ok {
            return Err(format!("{what} case {i}: routing on, but nothing was routed (entry {entry:#x})"));
        }
        on.push((r, post(i)));
    }
    for (i, &(entry, args)) in cases.iter().enumerate() {
        pre(i);
        let (r, routed) = run(entry, args, true);
        if routed != 0 {
            return Err(format!("{what} case {i}: routing off, but {routed} entries were routed"));
        }
        let mem = post(i);
        if (r, &mem) != (on[i].0, &on[i].1) {
            return Err(format!(
                "{what} case {i} (entry {entry:#x}, args {:x?}): routed {:x?} mem {:02x?} != original {:x?} mem {:02x?}",
                &args[..4], on[i].0, on[i].1, r, mem
            ));
        }
    }
    Ok(cases.len() as u32)
}

unsafe fn thunk_slot_bound(thunk: u64) -> Result<(), String> {
    let m = to_mapped_addr(thunk);
    if rd::<u16>(m) != 0x25ff {
        return Err(format!("{SKIP} 0x{thunk:x} is not an import thunk"));
    }
    let slot = (m as i64 + 6 + rd::<i32>(m + 2) as i64) as usize;
    if rd::<u64>(slot) == 0 {
        return Err(format!("{SKIP} import slot of 0x{thunk:x} is not bound"));
    }
    Ok(())
}

/// Exe import thunks whose slot is guest PE DLL code. Each is entered both as the run entry
/// (lifted `call_va8(thunk)` -> `emu_run_full(thunk)`) and through a guest indirect call (DLL
/// code calling an exe vtable thunk).
unsafe fn diff_import_thunk() -> Result<u32, String> {
    thunk_slot_bound(THUNK_QPF)?;
    thunk_slot_bound(THUNK_EXC_COPY)?;
    let mut n = 0;
    // _Query_perf_frequency(): rax = QueryPerformanceFrequency (constant).
    let qpf_stub = call_stub(THUNK_QPF);
    n += on_vs_off(
        "j__Query_perf_frequency",
        &[(THUNK_QPF, [0; 8]), (qpf_stub, [0; 8])],
        crate::emu::resolve_host_callable(THUNK_QPF).is_some(),
        &|_| {},
        &|_| Vec::new(),
    )?;
    // __std_exception_copy(from, to) with _DoFree = 0: to = { from->_What, false }.
    // `__std_exception_data` = { const char* _What; bool _DoFree; } (16 bytes).
    let buf = Box::leak(vec![0u8; 0x80].into_boxed_slice()).as_mut_ptr() as usize;
    let (from, to) = (buf, buf + 0x24); // `to` under-aligned (4) on purpose
    let copy_stub = call_stub(THUNK_EXC_COPY);
    let whats = [0x1234_5678_9abc_def0u64, buf as u64 + 0x60, 0];
    let mut cases = Vec::new();
    for (k, _) in whats.iter().enumerate() {
        let args = [from as u64, to as u64, 0, 0, 0, 0, 0, 0];
        cases.push((if k % 2 == 0 { THUNK_EXC_COPY } else { copy_stub }, args));
        cases.push((if k % 2 == 0 { copy_stub } else { THUNK_EXC_COPY }, args));
    }
    n += on_vs_off(
        "j___std_exception_copy",
        &cases,
        crate::emu::resolve_host_callable(THUNK_EXC_COPY).is_some(),
        &|i| {
            wr::<u64>(from, whats[i / 2]);
            wr::<u64>(from + 8, 0); // _DoFree = false (no allocation)
            for o in (0..16).step_by(4) {
                wr::<u32>(to + o, 0xA5A5_A5A5);
            }
        },
        &|_| (0..16).map(|o| rd::<u8>(to + o)).collect(),
    )?;
    Ok(n)
}

/// `__security_cookie` 0x143822fc0 / `__report_gsfailure` 0x141835cc8.
const COOKIE: u64 = 0x1_4382_2fc0;
const REPORT_GSFAILURE: u64 = 0x1_4183_5cc8;

/// Stand-in for `__report_gsfailure` (which ends the process): same function on both sides.
unsafe extern "win64" fn gsfailure_fixture(rcx: u64) -> u64 {
    rcx.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0x77
}

/// `__security_check_cookie` 0x1418353d0 through a guest call:
/// `mov rax, 0x1111222233334444; call [__security_check_cookie]; xor rax, rcx; ret`.
/// Success leaves rax intact and rcx rotated left by 16. Failure (wrong value, or a cookie
/// whose top 16 bits aren't zero) tail-jumps to the `__report_gsfailure` mock.
unsafe fn diff_check_cookie() -> Result<u32, String> {
    let _gs = Mock::new(REPORT_GSFAILURE, gsfailure_fixture as *const ())?;
    let cookie_p = crate::exe::ptr::<u8>(COOKIE) as *mut u8;
    let snap = Snapshot::take(cookie_p, 8);
    let live = rd::<u64>(cookie_p as usize);
    let mut c = vec![0x48, 0xB8];
    c.extend_from_slice(&0x1111_2222_3333_4444u64.to_le_bytes());
    c.extend_from_slice(&[0xFF, 0x15, 0x04, 0x00, 0x00, 0x00, 0x48, 0x31, 0xC8, 0xC3]);
    c.extend_from_slice(&0x1_4183_53d0u64.to_le_bytes());
    let stub = guest_code(&c);
    // (cookie value to install, rcx)
    let probes: [(u64, u64); 8] = [
        (live, live),
        (live, live ^ 1),
        (live, live ^ (1 << 63)),
        (live, 0),
        (0x0000_2b99_2ddf_a232, 0x0000_2b99_2ddf_a232),
        (0x0000_2b99_2ddf_a232, 0x0000_2b99_2ddf_a233),
        (0xabcd_0000_1234_5678, 0xabcd_0000_1234_5678), // equal, top 16 bits set: restore path
        (0x0001_0000_0000_0000, 0x0001_0000_0000_0000),
    ];
    let cases: Vec<(u64, [u64; 8])> = probes.iter().map(|&(_, rcx)| (stub, [rcx, 0, 0, 0, 0, 0, 0, 0])).collect();
    let r = on_vs_off(
        "__security_check_cookie",
        &cases,
        false,
        &|i| wr::<u64>(cookie_p as usize, probes[i].0),
        &|_| Vec::new(),
    );
    snap.restore();
    r
}

/// `_guard_dispatch_icall` 0x142547700 (`jmp rax`) and its dispatch-pointer thunk 0x142547720
/// (`jmp [0x142647390]`) through a guest call:
/// `mov rax, callee; call [helper]; ret` with callee `lea rax, [rcx+rcx*2]; add rax, rdx; ret`.
unsafe fn diff_guard_dispatch() -> Result<u32, String> {
    let callee = guest_code(&[0x48, 0x8D, 0x04, 0x49, 0x48, 0x01, 0xD0, 0xC3]);
    let stub_for = |helper: u64| {
        let mut c = vec![0x48, 0xB8];
        c.extend_from_slice(&callee.to_le_bytes());
        c.extend_from_slice(&[0xFF, 0x15, 0x01, 0x00, 0x00, 0x00, 0xC3]);
        c.extend_from_slice(&helper.to_le_bytes());
        guest_code(&c)
    };
    let s700 = stub_for(0x1_4254_7700);
    let s720 = stub_for(0x1_4254_7720);
    // 0x142547720 = `jmp [0x142647390]`: only comparable while the CFG dispatch pointer holds
    // the plain dispatcher (the loader doesn't enable CFG).
    let fptr_plain = rd::<u64>(to_mapped_addr(0x1_4264_7390)) == 0x1_4254_7700;
    let mut cases = Vec::new();
    for (rcx, rdx) in [(1u64, 2u64), (0x7fff_ffff_ffff_ffff, 5), (0, u64::MAX)] {
        cases.push((s700, [rcx, rdx, 0, 0, 0, 0, 0, 0]));
        if fptr_plain {
            cases.push((s720, [rcx, rdx, 0, 0, 0, 0, 0, 0]));
        }
    }
    on_vs_off("_guard_dispatch_icall", &cases, false, &|_| {}, &|_| Vec::new())
}
