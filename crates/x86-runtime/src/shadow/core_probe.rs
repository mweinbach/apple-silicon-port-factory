//! Probe orchestration: fork from the game thread, orchestrator setup, the three runs,
//! reporting, and the collector thread.

use super::compare::{compare, Cmp, Detail};
use super::handler::{self, page};
use super::memmap;
use super::run::{self, Bufs, RunRes};
pub use super::types::{Outcome, Reason, Report, N_OUTCOMES};
use super::types::{POISON_G_FAIL, POISON_N_FAIL, POISON_RAN};
use super::{ArgRegs, Kind, ProbeCfg, State};
use win32_rt::libc;
use std::io::Write;
use std::sync::atomic::Ordering;

/// Sent by the game thread to the collector after a fork.
pub struct Pending {
    pub pid: i32,
    pub slot: usize,
    pub k: u32,
    pub fork_us: u64,
}

#[inline(always)]
fn current_sp() -> usize {
    let sp: usize;
    unsafe { core::arch::asm!("mov {}, sp", out(reg) sp, options(nomem, nostack, preserves_flags)) };
    sp
}

/// Memory classes of one probe, computed by the orchestrator before the runs.
pub(crate) struct Excl {
    /// Never tracked.
    pub hard: Vec<(usize, usize)>,
    /// Tracked, byte-ignored by the comparison.
    pub soft: Vec<(usize, usize)>,
    /// Scratch stacks (dead frames below the live tops): values pointing here are
    /// frame-layout dependent (`stack_layout`).
    pub scratch: Vec<(usize, usize)>,
    /// Windows of the scratch stacks right below the live tops, poisoned by G3/N3 (untracked
    /// except the page holding the top, whose bytes below the top are soft).
    pub poison: Vec<(usize, usize)>,
}

/// Bytes poisoned below each live stack top (G3/N3).
const POISON_WINDOW: usize = 1 << 20;

/// Ranges of the probing thread that must stay writable and untracked, and byte ranges the
/// comparison ignores. `hook_sp`: everything below it on this thread's stack is scratch.
unsafe fn exclusions(hook_sp: usize, cfg: &ProbeCfg, bufs: &Bufs) -> Excl {
    let jit = cfg.jit;
    let mut hard: Vec<(usize, usize)> = Vec::with_capacity(4096);
    let mut soft: Vec<(usize, usize)> = cfg.ignore.clone();
    let mut scratch: Vec<(usize, usize)> = Vec::with_capacity(8);
    let mut poison: Vec<(usize, usize)> = Vec::with_capacity(8);
    hard.extend_from_slice(&cfg.image.0);
    soft.extend_from_slice(&cfg.image.1);
    stage(31);
    // Host stack below the hook sp (signal frames, our own frames): rounded up to a page.
    let me = libc::pthread_self();
    let top = libc::pthread_get_stackaddr_np(me) as usize;
    let size = libc::pthread_get_stacksize_np(me);
    let pg = page();
    hard.push((top.saturating_sub(size).saturating_sub(pg * 4), (hook_sp + pg) & !(pg - 1)));
    scratch.push((top.saturating_sub(size), hook_sp));
    // pthread struct (TSD, errno) sits at the stack top of secondary threads.
    hard.push((top, top + 4 * pg));
    // Threads with a caller-supplied stack (guest threads: `pthread_attr_setstack`) get a
    // separately allocated struct. It holds the kernel thread port, which
    // `_pthread_fork_child` re-creates in every child (g1: the tag-30 nondet byte).
    let selfp = me as usize;
    hard.push((selfp & !(pg - 1), (selfp + 0x2000 + pg - 1) & !(pg - 1)));
    // parking_lot_core's bucket table (host-side queues behind WaitOnAddress / CVs / SRW
    // locks): unparking stamps the bucket's fair-timeout with the host clock.
    if let Some(r) = memmap::parking_lot_buckets() {
        soft.push(r);
    }
    let c = crate::callctx::ctx();
    let cp = c as *const crate::callctx::CallCtx as usize;
    soft.push((cp, cp + core::mem::size_of::<crate::callctx::CallCtx>()));
    // Poisoning window `[max(lo, hi - POISON_WINDOW), hi)`, the low end page-aligned up. It
    // reaches the live top itself: the callee frames of a probe start right below it, in the
    // page holding the top (tracked for the callers' frames above; the bytes below the top are
    // soft). Ending at the page boundary below the top (g4 renoir +0x4c650) left the probed
    // function's own frame unpoisoned, so a stale padding byte it copied to the heap scored
    // as a mismatch instead of uninit.
    let below = |lo: usize, hi: usize| -> Option<(usize, usize)> {
        let a = (hi.saturating_sub(POISON_WINDOW).max(lo) + pg - 1) & !(pg - 1);
        (hi > a).then_some((a, hi))
    };
    // Lifted arena scratch below the current top.
    let (lo, t) = (crate::lstack::arena_lo(c), c.ls_top.get());
    if lo != 0 && t > lo {
        crate::emu::out_split(&mut hard, &mut soft, lo, t);
        scratch.push((lo, t));
        poison.extend(below(lo, t));
    }
    // Big-frame arena: slots above the innermost live big frame are scratch (stale frames,
    // never zeroed); the frame list holds host-stack addresses (bookkeeping).
    if let Some(((blo, bhi), list)) = crate::simd_ops::shadow_big_arena(hook_sp) {
        if bhi > blo {
            crate::emu::out_split(&mut hard, &mut soft, blo, bhi);
            scratch.push((blo, bhi));
            let w = (blo + crate::simd_ops::BIG_FRAME_BYTES * 4).min(bhi);
            let (a, b) = ((blo + pg - 1) & !(pg - 1), w & !(pg - 1));
            if b > a {
                poison.push((a, b));
            }
        }
        soft.push(list);
    }
    stage(32);
    let (eb, esp) = crate::emu::shadow_prepare(jit, &mut hard, &mut soft);
    if esp > eb {
        scratch.push((eb, esp));
        poison.extend(below(eb, esp));
    }
    // If an alternate signal stack is active on this thread, keep it writable (`on_fault`
    // uses `SA_ONSTACK`; write-protecting an active altstack kills the child in the kernel
    // before `on_fault` can run).
    let mut alt: libc::stack_t = core::mem::zeroed();
    if libc::sigaltstack(core::ptr::null(), &mut alt) == 0
        && (alt.ss_flags & libc::SS_DISABLE) == 0
        && !alt.ss_sp.is_null()
        && alt.ss_size > 0
    {
        let lo = alt.ss_sp as usize;
        hard.push((lo, lo.saturating_add(alt.ss_size)));
    }
    stage(33);
    hard.extend_from_slice(&bufs.own_ranges());
    hard.push((diag_page(), diag_page() + 16384));
    Excl { hard, soft, scratch, poison }
}

/// Poison the windows (G3/N3 runs): every byte is complemented, so a value read from dead
/// stack always differs from the stale one G1/N saw (a fixed fill byte can equal it). G3 and
/// N3 see the same poisoned values.
unsafe fn poison_scratch(windows: &[(usize, usize)]) {
    for &(lo, hi) in windows {
        let mut p = lo;
        while p < hi && p & 7 != 0 {
            *(p as *mut u8) = !*(p as *const u8);
            p += 1;
        }
        while p + 8 <= hi {
            *(p as *mut u64) = !*(p as *const u64);
            p += 8;
        }
        while p < hi {
            *(p as *mut u8) = !*(p as *const u8);
            p += 1;
        }
    }
}

/// One-time lazy setup on this thread that a native run would otherwise do itself (and
/// the guest runs never do): done in the orchestrator, so all three runs fork from the
/// set-up state. g1: the first lifted frame on a thread VirtualAllocs its lstack arena
/// (host-side region bookkeeping showed up as 22 heap diffs in the first probe).
unsafe fn warm_up() {
    crate::lstack::shadow_warm(crate::callctx::ctx());
    #[cfg(target_arch = "aarch64")]
    {
        unsafe extern "win64" fn noop() -> u64 {
            0
        }
        let _ = crate::emu::shadow_run_lean(noop as *const () as usize, (0, 0), &[0; 8], &[0; 4], &[0; 8]);
    }
}

/// Orchestrator body (forked child of the game thread, or of the test thread for
/// [`blocking`]). Returns the report; the caller `_exit`s.
///
/// `win` is the caller's stack-argument window (`CallCtx::incoming_rsp`) as published when
/// the probe was taken: every run republishes it immediately before its body, as the real
/// call has it (lifted bodies take stack args 9.. from it, `emu_run_full` args 17..).
/// [`warm_up`] and anything else in the orchestrator may reset it.
#[allow(clippy::too_many_arguments)]
unsafe fn orchestrate(
    cfg: &ProbeCfg,
    keep_fd: i32,
    hook_sp: usize,
    win: (u64, u64),
    guest: &dyn Fn() -> (u64, u128),
    native: &dyn Fn() -> (u64, u128),
    ret_xmm: bool,
    rep: &mut Report,
) -> Option<(Bufs, Detail, Vec<memmap::Region>)> {
    let t0 = run::now_us();
    IN_CHILD.store(true, Ordering::Relaxed);
    win32_rt::shadow::enter_child();
    handler::DIAG.store(diag_page(), Ordering::Relaxed);
    // Faults/traps/aborts here and in the runs are recorded and `_exit`: never the loader's
    // handler (symbolizer mutex, SEH resume above the probe).
    handler::install();
    // Hard limit even if the game process (which kills us after the same limit) is gone.
    run::arm_watchdog(orch_limit_ms(cfg.timeout_ms) + 10_000);
    if run::parent_gone() {
        libc::_exit(run::EXIT_PARENT_GONE);
    }
    // Single-threaded after fork: thread creation fails (identically for G and N) instead
    // of starting a thread that would outlive the run and race the snapshot compare.
    win32_rt::thread::NO_SPAWN.store(true, Ordering::Relaxed);
    libc::setpgid(0, 0);
    stage(1);
    log::set_max_level(log::LevelFilter::Off);
    win32_rt::shadow::freeze_clock();
    warm_up();
    if keep_fd >= 0 {
        run::sandbox_fds(keep_fd);
    }
    stage(2);
    let pg = libc::sysconf(libc::_SC_PAGESIZE) as usize;
    handler::PAGE.store(pg, Ordering::Relaxed);
    let bufs = Bufs::new(cfg.maxpages)?;
    win32_rt::shadow::set_heap_arena(bufs.heap, run::HEAP_LEN);
    stage(3);
    let ex = exclusions(hook_sp, cfg, &bufs);
    stage(4);
    let regions = memmap::regions();
    let ranges = memmap::tracked_ranges(&regions, &ex.hard);
    handler::RANGES_PTR.store(ranges.as_ptr() as usize, Ordering::Relaxed);
    handler::RANGES_LEN.store(ranges.len(), Ordering::Relaxed);
    stage(5);
    // Runs protect `ranges` themselves right after their fork; the orchestrator stays
    // unprotected (a write fault inside fork() hits with signals blocked and is fatal).
    let tracked: usize = ranges.iter().map(|r| r[1] - r[0]).sum();
    super::hostalloc::prepare(bufs.host, run::HEAP_LEN);
    stage(6);
    let t1 = run::now_us();
    let guest = &|| {
        crate::emu::set_incoming_rsp(win.0, win.1);
        guest()
    };
    let native = &|| {
        crate::emu::set_incoming_rsp(win.0, win.1);
        native()
    };
    let r0 = run::run_one(&bufs, 0, cfg.timeout_ms, guest);
    stage(7);
    let r1 = run::run_one(&bufs, 1, cfg.timeout_ms, native);
    let r2 = run::run_one(&bufs, 2, cfg.timeout_ms, guest);
    stage(8);
    // G1 and N differ: rerun both with poisoned scratch stacks (G3, N3), so differences that
    // come from uninitialised stack memory can be told apart. Decided allocation-free, before
    // the pristine snapshot (the orchestrator's heap must not change until then).
    let mut res: [RunRes; run::NRUNS] = [r0, r1, r2, RunRes::default(), RunRes::default()];
    let all_ok = [r0.status, r1.status, r2.status] == [run::ST_OK; 3];
    let ret_differs = r0.rax != r1.rax || (ret_xmm && r0.xmm0 != r1.xmm0);
    let poisoned = all_ok && (ret_differs || bufs.pages_differ(0, 1, &ex.soft));
    if poisoned {
        let pw = &ex.poison;
        let g3 = || {
            poison_scratch(pw);
            guest()
        };
        let n3 = || {
            poison_scratch(pw);
            native()
        };
        res[3] = run::run_one(&bufs, 3, cfg.timeout_ms, &g3);
        res[4] = run::run_one(&bufs, 4, cfg.timeout_ms, &n3);
        rep.poison = POISON_RAN
            | if res[3].status != run::ST_OK { POISON_G_FAIL } else { 0 }
            | if res[4].status != run::ST_OK { POISON_N_FAIL } else { 0 };
    }
    stage(9);
    bufs.snapshot_pristine();
    rep.setup_us = t1 - t0;
    rep.tracked_mb = (tracked >> 20) as u64;
    let cmp = Cmp {
        bufs: &bufs,
        res,
        poisoned,
        ret_xmm,
        ret: cfg.ret,
        soft: ex.soft,
        scratch: ex.scratch.clone(),
        regions: &regions,
    };
    let mut d = compare(&cmp, rep);
    d.scratch = ex.scratch;
    d.poison = ex.poison;
    stage(10);
    core::mem::forget(ranges);
    Some((bufs, d, regions))
}

/// Shared diagnostic page (stage word at 0, panic text from 64): mapped by the game/test
/// process before the first fork, so an orchestrator that dies can be located.
fn diag_page() -> usize {
    static P: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *P.get_or_init(|| unsafe { run::map_anon(16384, true) })
}

#[inline]
fn stage(n: u32) {
    let p = diag_page();
    if p != 0 {
        unsafe {
            if n == 0 {
                core::ptr::write_bytes((p + 8) as *mut u8, 0, 24);
                *((p + 64) as *mut u8) = 0;
            }
            core::ptr::write_volatile(p as *mut u32, n)
        };
    }
}

/// Last orchestrator stage and text (fault info and panic message) for `orchestrator_died`.
pub fn diag() -> (u32, String) {
    let p = diag_page();
    if p == 0 {
        return (0, String::new());
    }
    unsafe {
        let st = core::ptr::read_volatile(p as *const u32);
        let f = |o: usize| core::ptr::read_volatile((p + o) as *const u64);
        let s = core::slice::from_raw_parts((p + 64) as *const u8, 4000);
        let n = s.iter().position(|&b| b == 0).unwrap_or(s.len());
        let mut out = String::new();
        if f(24) != 0 {
            out = format!("orch fault addr={:#x} pc={:#x} sig={}; ", f(8), f(16), f(24));
        }
        out += &String::from_utf8_lossy(&s[..n]);
        (st, out)
    }
}

/// Set in forked orchestrators/runs (never in the game/test process).
static IN_CHILD: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Panics in the orchestrator or a run must never unwind into the caller's code (the
/// child would carry on as a copy of the game/test process): record and `_exit`.
/// Installed once in the parent before the first fork: `set_hook` in a forked child can
/// deadlock on the hook lock if another thread held it at fork time.
fn ensure_panic_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = diag_page();
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if !IN_CHILD.load(Ordering::Relaxed) {
                return prev(info);
            }
            let p = diag_page();
            if p != 0 {
                let msg = format!("{info}");
                let n = msg.len().min(4000);
                unsafe {
                    core::ptr::copy_nonoverlapping(msg.as_ptr(), (p + 64) as *mut u8, n);
                    *((p + 64 + n) as *mut u8) = 0;
                }
            }
            unsafe { libc::_exit(97) };
        }));
    });
}

/// Wait for orchestrator `pid` and read its report from `fd` (blocking or not), killing it
/// after `limit_ms`. Returns (report if complete, wait status).
unsafe fn wait_orch(pid: i32, fd: i32, limit_ms: u64) -> (Option<Report>, i32) {
    let deadline = run::now_us() + limit_ms * 1000;
    let mut status = 0i32;
    loop {
        let (w, err) = run::waitpid_retry(pid, &mut status, libc::WNOHANG);
        if w == pid {
            break;
        }
        if w < 0 {
            status = if err > 0 { -err } else { -1 };
            break;
        }
        if run::now_us() > deadline {
            libc::kill(-pid, libc::SIGKILL);
            libc::kill(pid, libc::SIGKILL);
            let _ = run::waitpid_retry(pid, &mut status, 0);
            status = -2;
            break;
        }
        libc::usleep(500);
    }
    let fl = libc::fcntl(fd, libc::F_GETFL);
    libc::fcntl(fd, libc::F_SETFL, fl | libc::O_NONBLOCK);
    let mut buf = Report::default();
    let n = libc::read(fd, &mut buf as *mut Report as *mut libc::c_void, core::mem::size_of::<Report>());
    ((n as usize == core::mem::size_of::<Report>()).then_some(buf), status)
}

/// Orchestrator kill limit: all runs (G1, N, G2 and the poisoned G3, N3) plus setup.
fn orch_limit_ms(timeout_ms: u64) -> u64 {
    run::NRUNS as u64 * timeout_ms + 30_000
}

/// Region description for the dump.
fn region_name(regions: &[memmap::Region], heap: usize, a: usize) -> String {
    if (heap..heap + run::HEAP_LEN).contains(&a) {
        return "probe-heap".into();
    }
    let base = crate::exe::base();
    let size = crate::exe::image_size() as usize;
    if base != 0 && (base..base + size).contains(&a) {
        return format!("exe+{:#x} (va {:#x})", a - base, crate::exe::preferred_base() as usize + (a - base));
    }
    match memmap::region_of(regions, a) {
        Some(r) if memmap::is_malloc_tag(r.tag) => format!("malloc(tag {})", r.tag),
        Some(r) => format!("vm(tag {} share {})", r.tag, r.share),
        None => "?".into(),
    }
}

unsafe fn hex(p: usize, n: usize) -> String {
    (0..n).map(|i| format!("{:02x}", *((p + i) as *const u8))).collect::<Vec<_>>().join("")
}

/// Mismatch/nondet dump file.
unsafe fn write_dump(
    cfg: &ProbeCfg,
    regs: &ArgRegs,
    win: (u64, u64),
    rep: &Report,
    d: &Detail,
    bufs: &Bufs,
    regions: &[memmap::Region],
) {
    if cfg.dump_dir.is_empty() {
        return;
    }
    let safe: String = cfg.name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).take(48).collect();
    let path = format!(
        "{}/{}_{:x}_{}_{}_{}.txt",
        cfg.dump_dir,
        Outcome::from_u32(rep.outcome).name(),
        cfg.va,
        safe,
        rep.k,
        rep.seq
    );
    let Ok(mut f) = std::fs::File::create(&path) else { return };
    let _ = writeln!(f, "va={:#x} name={} k={} outcome={} reason={}", cfg.va, cfg.name, rep.k, rep.outcome().name(), rep.reason().name());
    let _ = writeln!(f, "gpr={:x?}\nxmm={:x?}\nstack={:x?}", regs.gpr, regs.xmm, regs.stack);
    // Caller window replayed in every run: stack args 9.. at rsp+0x40 (as lifted bodies copy them).
    let (wr, wl) = win;
    let n = if wr != 0 && wl > wr + 0x40 { (((wl - wr - 0x40) / 8) as usize).min(40) } else { 0 };
    let wargs: Vec<u64> = (0..n).map(|i| core::ptr::read_unaligned((wr + 0x40 + 8 * i as u64) as *const u64)).collect();
    let _ = writeln!(f, "window=({wr:#x}, {wl:#x}) args9..={wargs:x?}");
    // First-writer pcs below: `atos -o <binary> -l <image> <pc>`.
    extern "C" {
        static _mh_execute_header: u8;
    }
    let _ = writeln!(f, "image={:#x}", core::ptr::addr_of!(_mh_execute_header) as usize);
    let _ = writeln!(
        f,
        "ret: rax_mask={:#x} ({}) xmm0={} ({}) -- bits outside are not compared",
        rep.ret_mask,
        super::RetUse::src_name(rep.ret_src & 15),
        if rep.ret_xmm != 0 { "compared" } else { "ignored" },
        super::RetUse::src_name(rep.ret_src >> 4)
    );
    for (r, n) in ["G1", "N ", "G2"].iter().enumerate() {
        let _ = writeln!(
            f,
            "{n}: rax={:#x} xmm0={:#x} pages={} t={}us exit={} wait={:#x} fault_sig={} fault_addr={:#x} fault_pc={:#x}",
            rep.rax[r],
            rep.xmm0[r],
            rep.pages[r],
            rep.t_us[r],
            rep.run_exit(r).describe(),
            rep.run_status[r],
            rep.fault_sig[r],
            rep.fault_addr[r],
            rep.fault_pc[r]
        );
    }
    let poisoned = rep.poison & POISON_RAN != 0;
    if poisoned {
        let _ = writeln!(
            f,
            "G3 (poisoned): rax={:#x}{}  N3 (poisoned): rax={:#x}{}  uninit ranges G1/G3={} N/N3={}",
            rep.rax_p[0],
            if rep.poison & POISON_G_FAIL != 0 { " FAILED" } else { "" },
            rep.rax_p[1],
            if rep.poison & POISON_N_FAIL != 0 { " FAILED" } else { "" },
            rep.uninit_ranges[0],
            rep.uninit_ranges[1]
        );
    } else {
        let _ = writeln!(f, "G3/N3 (poisoned): not run");
    }
    let _ = writeln!(f, "-- scratch stacks: {:x?}", d.scratch);
    let _ = writeln!(f, "-- poisoning windows: {:x?}", d.poison);
    let _ = writeln!(
        f,
        "-- layout: alloc={} stack={} uncovered diff ranges={}",
        rep.alloc_layout_ranges,
        rep.stack_layout_ranges,
        d.uncovered.len()
    );
    let pg = page();
    let _ = writeln!(f, "-- ignored ranges within 1 MiB of the first diff: {:x?}", d.soft_near);
    let rows: &[(usize, &str)] =
        if poisoned { &[(0, "G1"), (1, "N "), (2, "G2"), (3, "G3"), (4, "N3")] } else { &[(0, "G1"), (1, "N "), (2, "G2")] };
    for (label, list) in [
        ("diff G1/N", &d.diffs),
        ("noise G1/G2", &d.noise),
        ("uninit G1/G3", &d.uninit[0]),
        ("uninit N/N3", &d.uninit[1]),
    ] {
        if !poisoned && label.starts_with("uninit") {
            continue;
        }
        let _ = writeln!(f, "-- {label}: {} ranges", list.len());
        for r in list.iter().take(64) {
            let a = r.lo & !15;
            let n = ((r.hi - a + 15) & !15).min(64);
            let pa = (a & !(pg - 1)) as u64;
            let Some(&(p, v)) = d.views.get(&pa) else { continue };
            let off = a - pa as usize;
            let n = n.min(pg - off);
            let _ = writeln!(f, "[{:#x},{:#x}) {}", r.lo, r.hi, region_name(regions, bufs.heap, r.lo));
            let fw = |k: usize| match bufs.first_writer(k, r.lo) {
                Some((pc, lr)) => format!("pc={pc:#x} lr={lr:#x}"),
                None => "-".to_string(),
            };
            let _ = writeln!(f, "  first write to the page: G1 {}  N {}", fw(0), fw(1));
            let _ = writeln!(f, "  pristine {}", hex(p + off, n));
            for &(k, nm) in rows {
                let _ = writeln!(f, "  {nm}       {}", hex(v[k] + off, n));
            }
        }
    }
}

/// Orchestrator process entry: run, report over `fd`, `_exit`.
#[allow(clippy::too_many_arguments)]
unsafe fn orchestrator_main(
    cfg: &ProbeCfg,
    fd: i32,
    hook_sp: usize,
    win: (u64, u64),
    regs: &ArgRegs,
    ret_xmm: bool,
    rep: &mut Report,
    guest: &dyn Fn() -> (u64, u128),
    native: &dyn Fn() -> (u64, u128),
) -> ! {
    match orchestrate(cfg, fd, hook_sp, win, guest, native, ret_xmm, rep) {
        Some((bufs, d, regions)) => {
            if !matches!(rep.outcome(), Outcome::Equal | Outcome::Skipped) {
                write_dump(cfg, regs, win, rep, &d, &bufs, &regions);
            }
        }
        None => {
            rep.outcome = Outcome::Skipped as u32;
            rep.reason = Reason::Setup as u32;
        }
    }
    let p = rep as *const Report as *const libc::c_void;
    libc::write(fd, p, core::mem::size_of::<Report>());
    libc::_exit(0)
}

/// Fork the orchestrator from the game thread. Returns whether it started.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn launch(
    st: &State,
    slot: usize,
    k: u32,
    _kind: Kind,
    regs: &ArgRegs,
    ret_xmm: bool,
    guest: &dyn Fn() -> (u64, u128),
    native: &dyn Fn() -> (u64, u128),
) -> bool {
    let win = crate::emu::peek_incoming_rsp();
    let hook_sp = current_sp();
    let s = st.slot(slot);
    let mut cfg = ProbeCfg::new(s.info.va, &s.info.name);
    cfg.ret = st.ret_use(s.info.va);
    memmap::parking_lot_sym();
    cfg.timeout_ms = st.spec.timeout_ms;
    cfg.maxpages = st.spec.maxpages;
    cfg.jit = st.spec.jit;
    cfg.dump_dir = st.spec.dir.clone();
    memmap::image_ranges(&mut cfg.image.0, &mut cfg.image.1);
    let mut rep = Report {
        seq: st.seq.fetch_add(1, Ordering::Relaxed),
        va: s.info.va,
        slot: slot as u32,
        k,
        outcome: Outcome::Skipped as u32,
        reason: Reason::OrchestratorDied as u32,
        ..Default::default()
    };
    stage(0);
    ensure_panic_hook();
    run::PARENT.store(libc::getpid(), Ordering::Relaxed);
    let t0 = run::now_us();
    let pid = libc::fork();
    if pid == 0 {
        orchestrator_main(&cfg, st.pipe.1, hook_sp, win, regs, ret_xmm, &mut rep, guest, native);
    }
    let fork_us = run::now_us() - t0;
    if pid < 0 {
        super::ARMED.store(true, Ordering::Relaxed);
        return false;
    }
    win32_rt::shadow::set_probe_active(pid);
    if let Ok(g) = st.pending.lock() {
        if let Some(tx) = g.as_ref() {
            let _ = tx.send(Pending { pid, slot, k, fork_us });
        }
    }
    true
}

/// [`super::probe_blocking`]: orchestrator forked from the calling thread; waits for it.
pub(crate) unsafe fn blocking(
    cfg: &ProbeCfg,
    regs: &ArgRegs,
    ret_xmm: bool,
    guest: &dyn Fn() -> (u64, u128),
    native: &dyn Fn() -> (u64, u128),
) -> Report {
    let win = crate::emu::peek_incoming_rsp();
    let hook_sp = current_sp();
    let mut own = cfg.clone();
    memmap::image_ranges(&mut own.image.0, &mut own.image.1);
    memmap::parking_lot_sym();
    let cfg = &own;
    let mut fds = [0i32; 2];
    let mut rep = Report {
        va: cfg.va,
        outcome: Outcome::Skipped as u32,
        reason: Reason::OrchestratorDied as u32,
        ..Default::default()
    };
    if libc::pipe(fds.as_mut_ptr()) != 0 {
        rep.reason = Reason::ForkFailed as u32;
        return rep;
    }
    stage(0);
    ensure_panic_hook();
    run::PARENT.store(libc::getpid(), Ordering::Relaxed);
    let pid = libc::fork();
    if pid == 0 {
        libc::close(fds[0]);
        orchestrator_main(cfg, fds[1], hook_sp, win, regs, ret_xmm, &mut rep, guest, native);
    }
    libc::close(fds[1]);
    if pid < 0 {
        libc::close(fds[0]);
        rep.reason = Reason::ForkFailed as u32;
        return rep;
    }
    let (got, status) = wait_orch(pid, fds[0], orch_limit_ms(cfg.timeout_ms));
    libc::close(fds[0]);
    let mut r = got.unwrap_or(rep);
    r.orch_status = status;
    r.orch_stage = diag().0;
    r
}

// ---------------------------------------------------------------------------------------------
// Collector (game process)
// ---------------------------------------------------------------------------------------------

fn summary(st: &State) -> String {
    let mut s = String::from(
        "va\tname\tgroup\tsamples\tequal\tmismatch\tabi_narrow\tnondet\tskipped\talloc_layout\tret_dead\tstack_layout\tuninit\n",
    );
    for sl in st.slot_list() {
        let c: Vec<u32> = sl.counts.iter().map(|x| x.load(Ordering::Relaxed)).collect();
        if c.iter().all(|&x| x == 0) {
            continue;
        }
        s += &format!(
            "{:#x}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            sl.info.va,
            sl.info.name,
            sl.info.group,
            sl.samples.load(Ordering::Relaxed),
            c[0],
            c[1],
            c[2],
            c[3],
            c[4],
            c[5],
            c[6],
            c[7],
            c[8]
        );
    }
    s
}

pub(crate) fn collector(rx: std::sync::mpsc::Receiver<Pending>) {
    let Some(st) = super::state_wait() else { return };
    super::adopt_extra(st);
    let log_path = format!("{}/shadow.log", st.spec.dir);
    let mut logf = std::fs::OpenOptions::new().create(true).append(true).open(&log_path).ok();
    let mut last_summary = std::time::Instant::now();
    while let Ok(p) = rx.recv() {
        let (got, status) = unsafe { wait_orch(p.pid, st.pipe.0, orch_limit_ms(st.spec.timeout_ms)) };
        win32_rt::shadow::set_probe_active(0);
        let mut rep = Report {
            va: st.slot(p.slot).info.va,
            slot: p.slot as u32,
            k: p.k,
            outcome: Outcome::Skipped as u32,
            reason: Reason::OrchestratorDied as u32,
            ..Default::default()
        };
        if let Some(buf) = got {
            rep = buf;
        }
        rep.fork_us = p.fork_us;
        rep.orch_status = status;
        rep.orch_stage = diag().0;
        if rep.reason() == Reason::OrchestratorDied {
            log::warn!("shadow: orchestrator died: status {status:#x} stage {} {}", rep.orch_stage, diag().1);
        }
        let s = st.slot(p.slot);
        let o = rep.outcome();
        s.counts[o as usize].fetch_add(1, Ordering::Relaxed);
        match o {
            Outcome::Equal => {
                let e = s.counts[0].load(Ordering::Relaxed);
                if e >= st.spec.stop_equal {
                    s.retired.store(true, Ordering::Relaxed);
                }
                let ev = s.every.load(Ordering::Relaxed);
                s.every.store((ev * 2).min(1 << 20), Ordering::Relaxed);
            }
            Outcome::Skipped if rep.reason() == Reason::DoubleTimeout => {
                if s.dto.fetch_add(1, Ordering::Relaxed) + 1 >= st.spec.stop_dto {
                    s.retired.store(true, Ordering::Relaxed);
                }
            }
            _ => {}
        }
        let line = format!(
            "va={:#x} name={} k={} out={}{} rax g1={:#x} n={:#x} g2={:#x} pages g1={} n={} g2={} diff={}r/{}B first={:#x} heap={} alloc_layout={} stack_layout={} uninit={}/{} poison={} p_rax g3={:#x} n3={:#x} native_consistent={} t g1={}us n={}us g2={}us fork={}us setup={}us tracked={}MB ret={:#x}/{}/{}{}",
            rep.va,
            s.info.name,
            rep.k,
            o.name(),
            if rep.reason() == Reason::HostUnsafe {
                format!("({}:{})", rep.reason().name(), rep.host_api_name())
            } else if rep.reason != 0 {
                format!("({})", rep.reason().name())
            } else {
                String::new()
            },
            rep.rax[0],
            rep.rax[1],
            rep.rax[2],
            rep.pages[0],
            rep.pages[1],
            rep.pages[2],
            rep.diff_ranges,
            rep.diff_bytes,
            rep.first_diff,
            rep.heap_ranges,
            rep.alloc_layout_ranges,
            rep.stack_layout_ranges,
            rep.uninit_ranges[0],
            rep.uninit_ranges[1],
            rep.poison,
            rep.rax_p[0],
            rep.rax_p[1],
            rep.native_consistent,
            rep.t_us[0],
            rep.t_us[1],
            rep.t_us[2],
            rep.fork_us,
            rep.setup_us,
            rep.tracked_mb,
            rep.ret_mask,
            rep.ret_xmm,
            rep.ret_src,
            // The process's first probe also pays one-time lazy setup in the runs.
            if rep.seq == 0 { " first_probe=1" } else { "" }
        );
        if let Some(f) = logf.as_mut() {
            let _ = writeln!(f, "{line}");
        }
        if o == Outcome::Mismatch {
            log::warn!("shadow: {line}");
        } else {
            log::info!("shadow: {line}");
        }
        if last_summary.elapsed().as_secs() >= 10 {
            let _ = std::fs::write(format!("{}/summary.txt", st.spec.dir), summary(st));
            last_summary = std::time::Instant::now();
        }
        std::thread::sleep(std::time::Duration::from_millis(st.spec.rate_ms));
        let live = |st: &State| st.slot_list().iter().any(|s| !s.retired.load(Ordering::Relaxed));
        if live(st) {
            super::ARMED.store(true, Ordering::Relaxed);
        } else {
            let _ = std::fs::write(format!("{}/summary.txt", st.spec.dir), summary(st));
            log::warn!("shadow: every selected function is retired; summary in {}/summary.txt", st.spec.dir);
            // Idle until a late `register_dll` slot re-arms. Publish idle first, then
            // re-check: a slot added in between is seen here or sees `idle` there.
            st.idle.store(true, Ordering::SeqCst);
            if live(st) && st.idle.swap(false, Ordering::SeqCst) {
                super::ARMED.store(true, Ordering::Relaxed);
            }
        }
    }
}
