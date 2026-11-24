//! Crash reports.
//!
//! [`report`] runs on fatal events: unhandled exceptions (`unwind.rs`
//! `fatal_unhandled`), faults in host code and `__fastfail` (loader
//! `signals.rs`), CRT abort/watson/invalid-parameter and unresolved-import
//! traps. It prints, through the log and into
//! `out/crash/crash-<pid>-<time>.txt` (`AW2_CRASH_DIR` overrides):
//!
//! * the exception (code, address, parameters, kind of access violation);
//! * all general-purpose registers, annotated when they point into an image;
//! * the bytes around RIP and a disassembly of the next instructions;
//! * a raw scan of the stack for return addresses into PE images;
//! * the faulting thread's backtrace, then a backtrace of every other
//!   Windows thread (and the AppKit main thread).
//!
//! Frames are symbolized by the loader: `module+0xrva [va ...]` plus the
//! fndb name/owner/status for exe frames (needs `out/symbols/*.syms`, see
//! `tools/debug/export_symbols.py`), builtin export names for our thunks and
//! demangled Rust names for host frames.
//!
//! The walker mixes two strategies: PE frames unwind with `.pdata`
//! (`virtual_unwind`), every other frame (Rust/system libraries, JIT code)
//! follows the `rbp` chain (macOS x86_64 code keeps frame pointers). The two
//! meet naturally: an `extern "win64"` export called by the guest pushes the
//! guest `rbp` and its `[rbp+8]` is the guest return address; a guest callback
//! preserves the host `rbp`.
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

/// [`walk`] over several stack ranges (a thread's guest TEB stack and its host pthread
/// stack): a frame may live in any of them. The stack pointer must grow from frame to frame
/// within one range; moving to another range is allowed (bounded by `max`).
pub unsafe fn walk_in(ctx: &Context, ranges: &[(u64, u64)], max: usize) -> Vec<u64> {
    let range_of = |p: u64| ranges.iter().position(|&(lo, hi)| p >= lo && p < hi);
    let in_stack = |p: u64| range_of(p).is_some();
    let mut out = Vec::new();
    let mut cur = *ctx;
    while out.len() < max && cur.rip != 0 {
        out.push(cur.rip);
        let mut next = cur;
        let rsp = cur.gpr[RSP];
        if loader_module(cur.rip) {
            if !in_stack(rsp) || read_u64(rsp).is_none() {
                break;
            }
            match unwind::lookup_function_entry(cur.rip) {
                Some((base, f)) => {
                    unwind::virtual_unwind(0, base, cur.rip, f, &mut next);
                }
                None => {
                    // Leaf function (or a builtin thunk): return address on top.
                    next.rip = read_u64(rsp).unwrap_or(0);
                    next.gpr[RSP] = rsp + 8;
                }
            }
        } else {
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
        }
        let nsp = next.gpr[RSP];
        if next.gpr[RSP] <= rsp && (range_of(nsp).is_none() || range_of(nsp) == range_of(rsp)) {
            break;
        }
        cur = next;
    }
    out
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

/// Guest frames for a snapshot: the interpreter's current guest IP followed by plausible
/// guest return addresses found on the guest stack (heuristic scan, innermost first).
fn guest_frames(guest: Option<([u64; 16], u64)>) -> Vec<u64> {
    let Some((gpr, rip)) = guest else { return Vec::new() };
    let mut out = vec![rip];
    let rsp = gpr[RSP];
    for i in 0..1024u64 {
        let Some(v) = read_u64(rsp.wrapping_add(i * 8)) else { break };
        if v >= 0x10000 && loader_module(v) && follows_call(v) {
            out.push(v);
            if out.len() >= 12 {
                break;
            }
        }
    }
    out
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

/// Interrupt `pthread`, walk its stack while it is parked, release it.
unsafe fn snapshot_walk(pthread: usize, max: usize) -> Result<Vec<u64>, &'static str> {
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
    let host = walk_in(&ctx, &[bounds, pthread_bounds(pthread as libc::pthread_t)], max);
    // Guest stack is read while the target is still parked.
    let mut frames = guest_frames(guest);
    frames.extend(host);
    SNAP.target.store(0, Ordering::Release);
    SNAP.state.store(SNAP_RELEASED, Ordering::Release);
    let t1 = Instant::now();
    while SNAP.state.load(Ordering::Acquire) != SNAP_IDLE && t1.elapsed() < Duration::from_millis(100) {
        std::thread::sleep(Duration::from_micros(50));
    }
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
// Report
// ---------------------------------------------------------------------------

/// Thread id of the thread currently writing a report (0 = none).
static REPORTER: AtomicUsize = AtomicUsize::new(0);
static BUF: Mutex<String> = Mutex::new(String::new());

fn line(s: impl AsRef<str>) {
    let s = s.as_ref();
    log::error!(target: "win32_rt::crash", "{s}");
    if let Ok(mut b) = BUF.try_lock() {
        b.push_str(s);
        b.push('\n');
    }
}

fn crash_dir() -> std::path::PathBuf {
    if let Some(p) = std::env::var_os("AW2_CRASH_DIR") {
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
    if text.is_empty() {
        return;
    }
    let dir = crash_dir();
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let path = dir.join(format!("crash-{}-{secs}.txt", std::process::id()));
    match std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&path, &text)) {
        Ok(()) => {
            let log_dir_name = crate::config::app_config().log_dir_name;
            if let Some(home) = std::env::var_os("HOME").map(std::path::PathBuf::from) {
                let log_dir = home.join("Library/Logs").join(&log_dir_name);
                let _ = std::fs::create_dir_all(&log_dir);
                let _ = std::fs::write(log_dir.join("last-crash.txt"), &text);
            }
            let p = path.canonicalize().unwrap_or(path);
            log::error!(target: "win32_rt::crash", "crash report written to {}", p.display());
            eprintln!("crash report written to {}", p.display());
        }
        Err(e) => log::error!(target: "win32_rt::crash", "could not write crash report to {}: {e}", path.display()),
    }
}

fn self_tid() -> usize {
    let mut tid = 0u64;
    unsafe { libc::pthread_threadid_np(0 as libc::pthread_t, &mut tid) };
    tid as usize
}

fn thread_label(pthread: usize) -> String {
    let mut name = [0u8; 64];
    unsafe { libc::pthread_getname_np(pthread as libc::pthread_t, name.as_mut_ptr() as *mut libc::c_char, name.len()) };
    let n = std::ffi::CStr::from_bytes_until_nul(&name).map(|c| c.to_string_lossy().into_owned()).unwrap_or_default();
    if n.is_empty() {
        String::new()
    } else {
        format!(" pthread-name={n:?}")
    }
}

fn sym(pc: u64) -> String {
    crate::symbolize(pc as usize)
}

/// Print a crash report. Safe to call from any thread; concurrent reports
/// are serialized and a fault inside the report just flushes what exists.
pub unsafe fn report(title: &str, rec: Option<&ExceptionRecord>, ctx: &Context) {
    let me = self_tid();
    if let Err(owner) = REPORTER.compare_exchange(0, me, Ordering::AcqRel, Ordering::Acquire) {
        if owner == me {
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
    line(format!(
        "pid {} thread 0x{win_tid:x} {wname:?} (host tid {me}{})",
        std::process::id(),
        thread_label(libc::pthread_self() as usize)
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
    code_at(ctx.rip);
    stack_scan(ctx.gpr[RSP]);
    guest_state();

    let [b, p] = current_stack_ranges();
    line(format!("backtrace (faulting thread, stack 0x{:x}..0x{:x}, host 0x{:x}..0x{:x}):", b.0, b.1, p.0, p.1));
    let frames = walk_in(ctx, &[b, p], 64);
    for (i, pc) in frames.iter().enumerate() {
        line(format!("  #{i:<2} {}", sym(*pc)));
    }
    if frames.len() < 8 {
        // Short walk (frame chain broken or on an unexpected stack): libSystem's unwinder
        // from here, through the signal frame. Starts with the reporter's own frames.
        line("host backtrace (libSystem unwinder; the first frames are the crash reporter's):");
        for (i, pc) in host_backtrace(64).iter().enumerate() {
            line(format!("  #{i:<2} {}", sym(*pc)));
        }
    }
    other_threads();
    line(format!("==================== end of crash report ({:?}) ====================", t_start.elapsed()));
    flush_file();
    // Flush traces etc. now: the process may die without reaching ExitProcess.
    crate::process::call_exit_hooks();
    REPORTER.store(0, Ordering::Release);
}

unsafe fn registers(ctx: &Context) {
    const NAMES: [&str; 16] =
        ["rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"];
    line("registers:");
    for row in [[0usize, 3, 1, 2], [6, 7, 5, 4], [8, 9, 10, 11], [12, 13, 14, 15]] {
        let mut s = String::from(" ");
        for i in row {
            let _ = write!(s, " {:>3}={:016x}", NAMES[i], ctx.gpr[i]);
        }
        line(s);
    }
    line(format!("  rip={:016x} eflags={:08x} mxcsr={:08x}", ctx.rip, ctx.eflags, ctx.mx_csr));
    for (i, n) in NAMES.iter().enumerate() {
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

/// Guest x86-64 registers of the innermost interpreter frame on this thread, if any.
unsafe fn guest_state() {
    let Some(f) = GUEST_STATE.get() else { return };
    let Some((gpr, rip)) = f() else { return };
    const NAMES: [&str; 16] =
        ["rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"];
    line(format!("guest interpreter state (innermost emu frame; rip = after current instruction): rip={rip:#x} {}", sym(rip)));
    for row in [[0usize, 3, 1, 2], [6, 7, 5, 4], [8, 9, 10, 11], [12, 13, 14, 15]] {
        let mut s = String::from(" ");
        for i in row {
            let _ = write!(s, " {:>3}={:016x}", NAMES[i], gpr[i]);
        }
        line(s);
    }
    for (i, n) in NAMES.iter().enumerate() {
        let v = gpr[i];
        if i == RSP || v < 0x10000 {
            continue;
        }
        let s = sym(v);
        if s != format!("0x{v:x}") {
            line(format!("  guest {n} -> {s}"));
        }
    }
    code_at(rip);
    if !loader_module(rip) {
        if let Some(caller) = read_u64(gpr[RSP]) {
            if loader_module(caller) {
                line(format!("guest caller @[rsp] = {caller:#x} {}:", sym(caller)));
                code_at(caller.saturating_sub(0x18));
            }
        }
    }
    line("guest stack:");
    stack_scan(gpr[RSP]);
    if std::env::var_os("PORT_CRASH_DISASM").is_some_and(|v| v == "1") {
        disasm_guest_frames(&gpr, rip);
    }
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

unsafe fn disasm_guest_frames(gpr: &[u64; 16], rip: u64) {
    let mut seen = Vec::<u64>::new();
    let mut dump_once = |pc: u64, label: &str| {
        if !loader_module(pc) {
            return;
        }
        let key = crate::try_loader()
            .and_then(|l| l.lookup_function_entry(pc))
            .map(|(b, rf)| b + (*rf).begin_rva as u64)
            .unwrap_or(pc & !0xF);
        if seen.contains(&key) {
            return;
        }
        seen.push(key);
        disasm_function_around(pc, label);
    };
    dump_once(rip, "rip");
    let rsp = gpr[RSP];
    let mut count = 0usize;
    for i in 0..256u64 {
        let Some(v) = read_u64(rsp + i * 8) else { break };
        if loader_module(v) && follows_call(v) {
            dump_once(v, &format!("stack[+0x{:x}]", i * 8));
            count += 1;
            if count >= 8 {
                break;
            }
        }
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

unsafe fn code_at(rip: u64) {
    let mut around = [0u8; 32];
    if safe_read(rip.wrapping_sub(16), &mut around) {
        let hex: Vec<String> =
            around.iter().enumerate().map(|(i, b)| if i == 16 { format!("[{b:02x}]") } else { format!("{b:02x}") }).collect();
        line(format!("code @rip-16: {}", hex.join(" ")));
    }
    let mut code = [0u8; 48];
    let n = (0..=3).map(|k| 48 - k * 16).find(|&n| safe_read(rip, &mut code[..n])).unwrap_or(0);
    if n == 0 {
        line(format!("code @rip: 0x{rip:x} is not readable"));
        return;
    }
    if let Some(l) = crate::try_loader() {
        let lines = l.disassemble(rip, &code[..n], 6);
        if !lines.is_empty() {
            line("disassembly @rip:");
            for (i, s) in lines.iter().enumerate() {
                line(format!("  {} {s}", if i == 0 { "=>" } else { "  " }));
            }
        }
    }
}

/// Return addresses into PE images among the top 4 KiB of the stack (helps
/// when the unwind chain is broken).
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

unsafe fn other_threads() {
    let me = libc::pthread_self() as usize;
    let mut targets: Vec<(String, usize)> = Vec::new();
    for teb in crate::thread::all_tebs() {
        let tid: usize = crate::teb::field(teb, crate::teb::off::CLIENT_ID_THREAD);
        let Some(obj) = crate::thread::find_by_tid(tid as u32) else { continue };
        let pt = obj.pthread.load(Ordering::Relaxed);
        if pt == 0 || pt == me {
            continue;
        }
        let name = obj.name.try_lock().map(|n| n.clone()).unwrap_or_default();
        targets.push((format!("thread 0x{tid:x} {name:?}"), pt));
    }
    let main = MAIN_PTHREAD.load(Ordering::Relaxed);
    if main != 0 && main != me && !targets.iter().any(|t| t.1 == main) {
        targets.push(("main thread (AppKit)".into(), main));
    }
    if targets.is_empty() {
        return;
    }
    line(format!("other threads ({}):", targets.len()));
    for (label, pt) in targets {
        line(format!("--- {label}{}", thread_label(pt)));
        match snapshot_walk(pt, 32) {
            Ok(frames) => {
                for (i, pc) in frames.iter().enumerate() {
                    line(format!("  #{i:<2} {}", sym(*pc)));
                }
            }
            Err(e) => line(format!("  ({e})")),
        }
    }
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
}
