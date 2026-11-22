//! Per-thread parking word for kernel-object waits (`handles::wait_for`).
//!
//! Replaces a per-thread `std::sync::Condvar` (pthread/psynch). A waiter arms
//! its word under `WAIT_LOCK`, drops the lock, optionally spins, and only then
//! advertises `SLEEPING` and blocks in `__ulock_wait`. A notifier swaps the
//! word to `NOTIFIED` and enters the kernel only if the owner is really
//! asleep, so wakes that land while the waiter is still on its way to sleep
//! (or spinning) cost one atomic instead of a `__psynch_cvsignal`.
//!
//! `AW2_WAIT_SPIN_US` (A/B only): spin that long on the word before sleeping.
//! `AW2_WAKE_STATS=1` (diagnostics): log wake/sleep counts every ~2 s.

use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::Instant;

extern "C" {
    fn __ulock_wait(op: u32, addr: *mut c_void, value: u64, timeout_us: u32) -> i32;
    fn __ulock_wake(op: u32, addr: *mut c_void, wake_value: u64) -> i32;
}
const UL_COMPARE_AND_WAIT: u32 = 1;
const ULF_NO_ERRNO: u32 = 0x0100_0000;

const IDLE: u32 = 0;
const NOTIFIED: u32 = 1;
const SLEEPING: u32 = 2;

pub struct Parker {
    word: AtomicU32,
}

struct Config {
    spin_us: u64,
    stats: bool,
}

fn config() -> &'static Config {
    static C: OnceLock<Config> = OnceLock::new();
    C.get_or_init(|| Config {
        spin_us: std::env::var("AW2_WAIT_SPIN_US").ok().and_then(|v| v.parse().ok()).unwrap_or(0),
        stats: std::env::var("AW2_WAKE_STATS").is_ok_and(|v| v == "1"),
    })
}

static KERNEL_WAKES: AtomicU64 = AtomicU64::new(0);
static KERNEL_WAKE_NS: AtomicU64 = AtomicU64::new(0);
static CHEAP_WAKES: AtomicU64 = AtomicU64::new(0);
static SLEEPS: AtomicU64 = AtomicU64::new(0);
static SPIN_HITS: AtomicU64 = AtomicU64::new(0);
static NEXT_REPORT_NS: AtomicU64 = AtomicU64::new(0);

fn now_ns() -> u64 {
    static T0: OnceLock<Instant> = OnceLock::new();
    T0.get_or_init(Instant::now).elapsed().as_nanos() as u64
}

fn maybe_report(now: u64) {
    let next = NEXT_REPORT_NS.load(Ordering::Relaxed);
    if now < next || NEXT_REPORT_NS.compare_exchange(next, now + 2_000_000_000, Ordering::Relaxed, Ordering::Relaxed).is_err() {
        return;
    }
    let kw = KERNEL_WAKES.swap(0, Ordering::Relaxed);
    let ns = KERNEL_WAKE_NS.swap(0, Ordering::Relaxed);
    log::info!(
        "wake-stats: kernel_wakes={} ({:.2} us avg) cheap_wakes={} sleeps={} spin_hits={}",
        kw,
        if kw > 0 { ns as f64 / kw as f64 / 1000.0 } else { 0.0 },
        CHEAP_WAKES.swap(0, Ordering::Relaxed),
        SLEEPS.swap(0, Ordering::Relaxed),
        SPIN_HITS.swap(0, Ordering::Relaxed),
    );
}

impl Parker {
    pub const fn new() -> Self {
        Parker { word: AtomicU32::new(IDLE) }
    }

    fn addr(&self) -> *mut c_void {
        self.word.as_ptr() as *mut c_void
    }

    /// Reset before re-checking/sleeping. Call with `WAIT_LOCK` held, after
    /// the waiter is registered, so a notifier that changes state afterwards
    /// is guaranteed to see the registration and flip the word.
    pub fn arm(&self) {
        self.word.store(IDLE, Ordering::Release);
    }

    /// Block until [`unpark`](Self::unpark), a spurious wake, or `deadline`.
    /// Call without `WAIT_LOCK`; the caller re-checks its condition after.
    pub fn park(&self, deadline: Option<Instant>) {
        let cfg = config();
        if cfg.spin_us > 0 {
            let start = Instant::now();
            let mut i = 0u32;
            loop {
                if self.word.load(Ordering::Acquire) == NOTIFIED {
                    if cfg.stats {
                        SPIN_HITS.fetch_add(1, Ordering::Relaxed);
                    }
                    return;
                }
                std::hint::spin_loop();
                i = i.wrapping_add(1);
                if i % 64 == 0 && start.elapsed().as_micros() as u64 >= cfg.spin_us {
                    break;
                }
            }
        }
        if self.word.compare_exchange(IDLE, SLEEPING, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return; // already notified
        }
        loop {
            let timeout_us = match deadline {
                None => 0, // infinite
                Some(d) => {
                    let now = Instant::now();
                    if now >= d {
                        break;
                    }
                    ((d - now).as_micros().min(u32::MAX as u128) as u32).max(1)
                }
            };
            if cfg.stats {
                SLEEPS.fetch_add(1, Ordering::Relaxed);
            }
            unsafe { __ulock_wait(UL_COMPARE_AND_WAIT | ULF_NO_ERRNO, self.addr(), SLEEPING as u64, timeout_us) };
            if self.word.load(Ordering::Acquire) != SLEEPING {
                return;
            }
        }
        // Timed out: stop advertising SLEEPING so later notifiers skip the syscall.
        let _ = self.word.compare_exchange(SLEEPING, IDLE, Ordering::AcqRel, Ordering::Acquire);
    }

    /// Wake the owner; enters the kernel only if it is actually asleep.
    pub fn unpark(&self) {
        let prev = self.word.swap(NOTIFIED, Ordering::AcqRel);
        let cfg = config();
        if prev == SLEEPING {
            let t0 = cfg.stats.then(now_ns);
            unsafe { __ulock_wake(UL_COMPARE_AND_WAIT | ULF_NO_ERRNO, self.addr(), 0) };
            if let Some(t0) = t0 {
                let t1 = now_ns();
                KERNEL_WAKES.fetch_add(1, Ordering::Relaxed);
                KERNEL_WAKE_NS.fetch_add(t1 - t0, Ordering::Relaxed);
                maybe_report(t1);
            }
        } else if cfg.stats {
            CHEAP_WAKES.fetch_add(1, Ordering::Relaxed);
            maybe_report(now_ns());
        }
    }
}

/// Minimal mutex word for very short critical sections (`handles::WAIT_LOCK`, the handle
/// table): CAS fast path, a bounded spin while the holder is running, then `__ulock_wait`
/// (Drepper's three-state futex mutex). The pthread mutex it replaces never spins, so a
/// burst of job workers waking at once paid `__psynch_mutexwait` / `__psynch_mutexdrop`
/// kernel round trips for critical sections of a few dozen nanoseconds.
pub struct WordLock {
    /// 0 unlocked, 1 locked, 2 locked with (possible) sleepers.
    word: AtomicU32,
}

impl WordLock {
    pub const fn new() -> Self {
        WordLock { word: AtomicU32::new(0) }
    }

    #[inline]
    pub fn lock(&self) {
        if self.word.compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed).is_err() {
            self.lock_slow();
        }
    }

    #[cold]
    fn lock_slow(&self) {
        for _ in 0..200 {
            let s = self.word.load(Ordering::Relaxed);
            if s == 0 && self.word.compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                return;
            }
            std::hint::spin_loop();
        }
        // Mark contended and sleep until the word changes.
        while self.word.swap(2, Ordering::Acquire) != 0 {
            unsafe {
                __ulock_wait(UL_COMPARE_AND_WAIT | ULF_NO_ERRNO, self.word.as_ptr() as *mut c_void, 2, 0)
            };
        }
    }

    #[inline]
    pub fn unlock(&self) {
        if self.word.swap(0, Ordering::Release) == 2 {
            unsafe { __ulock_wake(UL_COMPARE_AND_WAIT | ULF_NO_ERRNO, self.word.as_ptr() as *mut c_void, 0) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn unpark_before_park_returns_immediately() {
        let p = Parker::new();
        p.arm();
        p.unpark();
        let t = Instant::now();
        p.park(None);
        assert!(t.elapsed() < Duration::from_millis(100));
    }

    #[test]
    fn park_times_out() {
        let p = Parker::new();
        p.arm();
        let t = Instant::now();
        p.park(Some(Instant::now() + Duration::from_millis(20)));
        assert!(t.elapsed() >= Duration::from_millis(19));
    }

    #[test]
    fn unpark_wakes_sleeper() {
        let p = Arc::new(Parker::new());
        p.arm();
        let p2 = p.clone();
        let h = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(20));
            p2.unpark();
        });
        let t = Instant::now();
        p.park(None);
        assert!(t.elapsed() < Duration::from_secs(5));
        h.join().unwrap();
    }
}
