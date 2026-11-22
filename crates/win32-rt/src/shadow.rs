//! Child-side state for the `AW2_SHADOW` gameplay oracle (`aw2_game::shadow`).
//!
//! The oracle forks short-lived children that run a guest original and its native
//! replacement from the same snapshot. Inside those children:
//! - the clock is frozen ([`frozen_100ns`] / [`frozen_filetime`]), so two guest runs see the
//!   same time;
//! - the guest heap ([`crate::heap`]) allocates from a bump arena (identical addresses in
//!   every run, `free` is a no-op, no allocator metadata writes).
//!
//! In the game process all of this is off: each check is one relaxed load.

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

static FROZEN_100NS: AtomicU64 = AtomicU64::new(0);
static FROZEN_FT: AtomicU64 = AtomicU64::new(0);
static BUMP_CUR: AtomicUsize = AtomicUsize::new(0);
static BUMP_END: AtomicUsize = AtomicUsize::new(0);
static IN_CHILD: AtomicBool = AtomicBool::new(false);
static FORBID_REC: AtomicUsize = AtomicUsize::new(0);

/// `_exit` status of a shadow child that reached a host API it cannot use ([`forbidden`]).
pub const EXIT_HOST_UNSAFE: i32 = 92;
/// Bytes of the API name [`forbidden`] records (NUL-terminated, truncated).
pub const HOST_API_LEN: usize = 32;

/// Mark this process as a shadow orchestrator/run (forked; never the game process).
pub fn enter_child() {
    IN_CHILD.store(true, Ordering::Relaxed);
}

/// Whether this process is a shadow orchestrator/run.
#[inline(always)]
pub fn in_child() -> bool {
    IN_CHILD.load(Ordering::Relaxed)
}

/// Where [`forbidden`] records the API name ([`HOST_API_LEN`] bytes; the run's shared header).
pub fn set_forbid_record(addr: usize) {
    FORBID_REC.store(addr, Ordering::Relaxed);
}

/// Host services that cannot work in a forked child (CoreAudio, Metal, ...: their worker
/// threads, Mach ports and libdispatch state do not survive `fork`; touching them crashes or
/// hangs the child). In a shadow child: record `what` and `_exit` ([`EXIT_HOST_UNSAFE`]), so
/// the probe is skipped (`host_unsafe`). In the game process: one relaxed load.
#[inline(always)]
pub fn guard_host(what: &'static str) {
    if in_child() {
        forbidden(what)
    }
}

/// See [`guard_host`].
#[cold]
#[inline(never)]
pub fn forbidden(what: &str) -> ! {
    let rec = FORBID_REC.load(Ordering::Relaxed);
    if rec != 0 {
        let n = what.len().min(HOST_API_LEN - 1);
        unsafe {
            core::ptr::copy_nonoverlapping(what.as_ptr(), rec as *mut u8, n);
            *(rec as *mut u8).add(n) = 0;
        }
    }
    unsafe { libc::_exit(EXIT_HOST_UNSAFE) }
}

/// Orchestrator pid of the probe in flight (game process; 0 = none).
static PROBE_PID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

/// Game process: a probe orchestrator `pid` is in flight (0 = none).
pub fn set_probe_active(pid: i32) {
    PROBE_PID.store(pid, Ordering::Relaxed);
}

/// Orchestrator pid of the probe in flight in this (game) process, if any.
#[inline]
pub fn probe_active() -> Option<i32> {
    match PROBE_PID.load(Ordering::Relaxed) {
        0 => None,
        p => Some(p),
    }
}

/// Async-signal-safe stderr line: `parts` (text) and `nums` (decimal) interleaved,
/// text first. No allocation.
fn raw_line(parts: &[&str], nums: &[i64]) {
    let mut buf = [0u8; 512];
    let mut n = 0usize;
    let mut put = |b: &[u8], n: &mut usize| {
        let k = b.len().min(buf.len() - 1 - *n);
        buf[*n..*n + k].copy_from_slice(&b[..k]);
        *n += k;
    };
    for (i, p) in parts.iter().enumerate() {
        put(p.as_bytes(), &mut n);
        if let Some(&v) = nums.get(i) {
            let mut d = [0u8; 24];
            let mut j = d.len();
            let neg = v < 0;
            let mut u = v.unsigned_abs();
            loop {
                j -= 1;
                d[j] = b'0' + (u % 10) as u8;
                u /= 10;
                if u == 0 {
                    break;
                }
            }
            if neg {
                j -= 1;
                d[j] = b'-';
            }
            put(&d[j..], &mut n);
        }
    }
    buf[n] = b'\n';
    unsafe { libc::write(2, buf.as_ptr() as *const libc::c_void, n + 1) };
}

/// Game process: about to exit through `how` with `code` while a probe is in flight; say so
/// on stderr (the log may be buffered) with a host backtrace. No-op otherwise and in children.
pub fn note_exit(how: &str, code: i64) {
    if in_child() {
        return;
    }
    let Some(orch) = probe_active() else { return };
    let me = unsafe { libc::getpid() } as i64;
    raw_line(
        &["shadow: process ", " exiting (code ", ") while probe orchestrator ", " is in flight; via ", how],
        &[me, code, orch as i64],
    );
    let bt = std::backtrace::Backtrace::force_capture();
    let s = format!("shadow: exit backtrace:\n{bt}\n");
    unsafe { libc::write(2, s.as_ptr() as *const libc::c_void, s.len()) };
}

/// Signals whose default action terminates the process silently (no crash report).
const QUIET_FATAL: [i32; 11] = [
    libc::SIGHUP,
    libc::SIGINT,
    libc::SIGQUIT,
    libc::SIGTERM,
    libc::SIGUSR1,
    libc::SIGUSR2,
    libc::SIGPIPE,
    libc::SIGALRM,
    libc::SIGXCPU,
    libc::SIGXFSZ,
    libc::SIGVTALRM,
];

extern "C" fn on_quiet_fatal(sig: i32, info: *mut libc::siginfo_t, _uc: *mut libc::c_void) {
    unsafe {
        let (from, uid) = if info.is_null() { (0, 0) } else { ((*info).si_pid as i64, (*info).si_uid as i64) };
        raw_line(&["shadow: terminating signal ", " from pid ", " uid ", " (self ", ", probe orchestrator ", ")"], &[
            sig as i64,
            from,
            uid,
            libc::getpid() as i64,
            PROBE_PID.load(Ordering::Relaxed) as i64,
        ]);
        let mut dfl: libc::sigaction = core::mem::zeroed();
        dfl.sa_sigaction = libc::SIG_DFL;
        libc::sigaction(sig, &dfl, core::ptr::null_mut());
        libc::raise(sig);
    }
}

/// Game process with `AW2_SHADOW` on: log silent deaths. Signals still at `SIG_DFL` whose
/// default terminates get a handler that names the sender (`si_pid`) on stderr and then
/// re-raises; `exit()` (atexit) is reported while a probe is in flight. `exit_process`
/// calls [`note_exit`] itself.
pub fn install_death_diagnostics() {
    extern "C" fn at_exit() {
        note_exit("exit()", -1);
    }
    unsafe {
        libc::atexit(at_exit);
        for sig in QUIET_FATAL {
            let mut old: libc::sigaction = core::mem::zeroed();
            if libc::sigaction(sig, core::ptr::null(), &mut old) != 0 || old.sa_sigaction != libc::SIG_DFL {
                continue;
            }
            let mut sa: libc::sigaction = core::mem::zeroed();
            sa.sa_sigaction = on_quiet_fatal as *const () as usize;
            sa.sa_flags = libc::SA_SIGINFO;
            libc::sigaction(sig, &sa, core::ptr::null_mut());
        }
    }
}

/// Frozen interrupt time (100 ns), if inside a shadow child.
#[inline(always)]
pub fn frozen_100ns() -> Option<u64> {
    match FROZEN_100NS.load(Ordering::Relaxed) {
        0 => None,
        v => Some(v),
    }
}

/// Frozen system time (FILETIME, 100 ns since 1601), if inside a shadow child.
#[inline(always)]
pub fn frozen_filetime() -> Option<u64> {
    match FROZEN_FT.load(Ordering::Relaxed) {
        0 => None,
        v => Some(v),
    }
}

/// Freeze the clock at the current time (shadow children only; never in the game process).
pub fn freeze_clock() {
    let t = crate::kernel32::time::interrupt_time_100ns().max(1);
    let ft = crate::types::now_filetime().max(1);
    FROZEN_100NS.store(t, Ordering::Relaxed);
    FROZEN_FT.store(ft, Ordering::Relaxed);
    crate::kernel32::time::disable_qpc_fast_path();
}

/// Route guest heap allocations to `[base, base + len)` (shadow children only).
pub fn set_heap_arena(base: usize, len: usize) {
    BUMP_END.store(base + len, Ordering::Relaxed);
    BUMP_CUR.store((base + 15) & !15, Ordering::Relaxed);
}

/// Current bump arena `(base cursor, end)`; `(0, 0)` when off.
pub fn heap_arena() -> (usize, usize) {
    (BUMP_CUR.load(Ordering::Relaxed), BUMP_END.load(Ordering::Relaxed))
}

/// Whether guest heap allocations go to the bump arena.
#[inline(always)]
pub fn heap_on() -> bool {
    BUMP_END.load(Ordering::Relaxed) != 0
}

/// Bump-allocate `n` bytes (16-aligned, zero-filled: the arena is fresh anonymous memory
/// and never reused). Null when the arena is exhausted.
pub fn bump(n: usize) -> *mut u8 {
    let cur = BUMP_CUR.load(Ordering::Relaxed);
    let end = BUMP_END.load(Ordering::Relaxed);
    let next = match cur.checked_add((n + 15) & !15) {
        Some(v) if v <= end => v,
        _ => return core::ptr::null_mut(),
    };
    BUMP_CUR.store(next, Ordering::Relaxed);
    cur as *mut u8
}
