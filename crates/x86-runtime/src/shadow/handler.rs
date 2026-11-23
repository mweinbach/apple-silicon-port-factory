//! Write-fault tracking for probe runs. State is static (host image, never tracked).

use win32_rt::libc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const MODE_OFF: usize = 0;
/// Probe run: record the page into the run buffer, then re-enable writes.
pub const MODE_RUN: usize = 2;

pub static MODE: AtomicUsize = AtomicUsize::new(MODE_OFF);
/// Sorted `[lo, hi, flags]` (flags bit 0 = shared region, bits 8..11 = original protection).
pub static RANGES_PTR: AtomicUsize = AtomicUsize::new(0);
pub static RANGES_LEN: AtomicUsize = AtomicUsize::new(0);
pub static RUN_BUF: AtomicUsize = AtomicUsize::new(0);
pub static RUN_MAX: AtomicUsize = AtomicUsize::new(0);
/// Baseline area of the run buffer: page `i`'s content just before the run's first write
/// to it ([`super::run::Bufs::rebase`]).
pub static RUN_BASE: AtomicUsize = AtomicUsize::new(0);
/// First writer of page `i` of the run: `(pc, lr)` of the store that faulted on it
/// (`[u64; 2]` per page; 0 = none).
pub static RUN_WRITERS: AtomicUsize = AtomicUsize::new(0);
pub static PAGE: AtomicUsize = AtomicUsize::new(16384);
/// Shared diagnostic page (orchestrator faults: addr at +8, pc at +16, signal at +24).
pub static DIAG: AtomicUsize = AtomicUsize::new(0);

pub const EXIT_OK: i32 = 0;
pub const EXIT_SHARED: i32 = 90;
pub const EXIT_FAULT: i32 = 91;

/// Start of a run buffer (MAP_SHARED). The page list follows at [`HDR_BYTES`].
#[repr(C)]
pub struct RunHdr {
    pub done: u64,
    pub rax: u64,
    pub xmm0: u128,
    pub npages: u64,
    pub overflow: u64,
    pub shared_addr: u64,
    pub fault_addr: u64,
    pub fault_pc: u64,
    pub fault_sig: u64,
    pub t_us: u64,
    /// Host API a run reached that cannot work in a forked child (NUL-terminated;
    /// [`win32_rt::shadow::forbidden`]).
    pub host_api: [u8; win32_rt::shadow::HOST_API_LEN],
}

pub const HDR_BYTES: usize = 4096;

#[inline]
pub fn page() -> usize {
    PAGE.load(Ordering::Relaxed)
}

/// Tracked range containing `addr`: `(lo, hi, flags)`.
pub unsafe fn find_range(addr: usize) -> Option<(usize, usize, usize)> {
    let p = RANGES_PTR.load(Ordering::Relaxed) as *const [usize; 3];
    let n = RANGES_LEN.load(Ordering::Relaxed);
    if p.is_null() {
        return None;
    }
    let (mut lo, mut hi) = (0usize, n);
    while lo < hi {
        let mid = (lo + hi) / 2;
        let r = *p.add(mid);
        if addr < r[0] {
            hi = mid;
        } else if addr >= r[1] {
            lo = mid + 1;
        } else {
            return Some((r[0], r[1], r[2]));
        }
    }
    None
}

unsafe fn uc_pc(uc: *mut libc::c_void) -> u64 {
    if uc.is_null() {
        return 0;
    }
    let mc = (*(uc as *mut libc::ucontext_t)).uc_mcontext;
    if mc.is_null() {
        0
    } else {
        #[cfg(target_arch = "aarch64")]
        {
            (*mc).__ss.__pc
        }
        #[cfg(target_arch = "x86_64")]
        {
            (*mc).__ss.__rip
        }
    }
}

unsafe fn uc_lr(uc: *mut libc::c_void) -> u64 {
    if uc.is_null() {
        return 0;
    }
    let mc = (*(uc as *mut libc::ucontext_t)).uc_mcontext;
    if mc.is_null() {
        0
    } else {
        #[cfg(target_arch = "aarch64")]
        {
            (*mc).__ss.__lr
        }
        #[cfg(target_arch = "x86_64")]
        {
            0
        }
    }
}

/// Synchronous / abort signals handled in orchestrators and runs. Never chained to the
/// loader's handler: in a forked copy it takes the symbolizer's mutex (possibly held by a
/// thread that no longer exists: g3br children died recursing in it) and its SEH dispatch
/// could resume guest code above the probe, i.e. carry on as a copy of the game.
const SIGNALS: [i32; 7] = [libc::SIGSEGV, libc::SIGBUS, libc::SIGILL, libc::SIGFPE, libc::SIGTRAP, libc::SIGABRT, libc::SIGSYS];

extern "C" fn on_fault(sig: i32, info: *mut libc::siginfo_t, uc: *mut libc::c_void) {
    unsafe {
        let addr = (*info).si_addr as usize;
        if MODE.load(Ordering::Relaxed) == MODE_RUN {
            let h = RUN_BUF.load(Ordering::Relaxed) as *mut RunHdr;
            if sig == libc::SIGSEGV || sig == libc::SIGBUS {
                if let Some((_, _, flags)) = find_range(addr) {
                    let pg = addr & !(page() - 1);
                    if flags & 1 != 0 {
                        // The write would reach the game process (shared mapping).
                        (*h).shared_addr = addr as u64;
                        libc::_exit(EXIT_SHARED);
                    }
                    let n = (*h).npages as usize;
                    if n < RUN_MAX.load(Ordering::Relaxed) {
                        let list = (h as *mut u8).add(HDR_BYTES) as *mut u64;
                        *list.add(n) = pg as u64;
                        // Pre-write content (tracked pages are PROT_READ): what this run
                        // started from, including fork-child handler writes made before
                        // tracking began.
                        let base = RUN_BASE.load(Ordering::Relaxed);
                        if base != 0 {
                            core::ptr::copy_nonoverlapping(pg as *const u8, (base + n * page()) as *mut u8, page());
                        }
                        let w = RUN_WRITERS.load(Ordering::Relaxed) as *mut u64;
                        if !w.is_null() {
                            *w.add(2 * n) = uc_pc(uc);
                            *w.add(2 * n + 1) = uc_lr(uc);
                        }
                        (*h).npages = (n + 1) as u64;
                    } else {
                        (*h).overflow = 1;
                    }
                    let prot = ((flags >> 8) & 7) as i32;
                    if libc::mprotect(pg as *mut libc::c_void, page(), prot) == 0 {
                        return;
                    }
                }
            }
            // A genuine fault (or trap/abort) in a probe run: record it and end the run (the
            // game's own handlers must not run here: their I/O targets are closed).
            (*h).fault_addr = addr as u64;
            (*h).fault_pc = uc_pc(uc);
            (*h).fault_sig = sig as u64;
            libc::_exit(EXIT_FAULT);
        }
        // Orchestrator (never runs guest code): record for `orchestrator_died`.
        let d = DIAG.load(Ordering::Relaxed);
        if d != 0 {
            *((d + 8) as *mut u64) = addr as u64;
            *((d + 16) as *mut u64) = uc_pc(uc);
            *((d + 24) as *mut u64) = sig as u64;
        }
        if sig == libc::SIGSEGV || sig == libc::SIGBUS {
            // As before: the previous handler (the loader's) decides.
            return chain(sig, info, uc);
        }
        libc::_exit(EXIT_FAULT);
    }
}

static mut OLD_SEGV: libc::sigaction = unsafe { core::mem::zeroed() };
static mut OLD_BUS: libc::sigaction = unsafe { core::mem::zeroed() };

unsafe fn chain(sig: i32, info: *mut libc::siginfo_t, uc: *mut libc::c_void) {
    let old = if sig == libc::SIGBUS { &*(&raw const OLD_BUS) } else { &*(&raw const OLD_SEGV) };
    let h = old.sa_sigaction;
    if h == libc::SIG_DFL || h == libc::SIG_IGN || h == on_fault as *const () as usize {
        let mut dfl: libc::sigaction = core::mem::zeroed();
        dfl.sa_sigaction = libc::SIG_DFL;
        libc::sigaction(sig, &dfl, core::ptr::null_mut());
        return; // the access re-executes under the default action
    }
    if old.sa_flags & libc::SA_SIGINFO != 0 {
        let f: extern "C" fn(i32, *mut libc::siginfo_t, *mut libc::c_void) = core::mem::transmute(h);
        f(sig, info, uc);
    } else {
        let f: extern "C" fn(i32) = core::mem::transmute(h);
        f(sig);
    }
}

/// Install the tracking handler for [`SIGNALS`] (orchestrator, first thing after the fork;
/// inherited by runs).
pub unsafe fn install() {
    let mut sa: libc::sigaction = core::mem::zeroed();
    sa.sa_sigaction = on_fault as *const () as usize;
    sa.sa_flags = libc::SA_SIGINFO | libc::SA_NODEFER | libc::SA_ONSTACK;
    let mut unblock: libc::sigset_t = core::mem::zeroed();
    libc::sigemptyset(&mut unblock);
    for sig in SIGNALS {
        let old = match sig {
            libc::SIGSEGV => &raw mut OLD_SEGV,
            libc::SIGBUS => &raw mut OLD_BUS,
            _ => core::ptr::null_mut(),
        };
        libc::sigaction(sig, &sa, old);
        libc::sigaddset(&mut unblock, sig);
    }
    libc::pthread_sigmask(libc::SIG_UNBLOCK, &unblock, core::ptr::null_mut());
}
