//! Per-thread parking word, adaptive bounded spinning, and `WordLock` for
//! kernel-object waits (`handles`) and user-mode sync (`sync`).
//!
//! Each OS thread owns a 128-byte cache-line-aligned [`Parker`] word so
//! `__ulock_wait` / `__ulock_wake` operate on distinct per-thread addresses with
//! zero XNU kernel ulock-bucket contention. Before entering `__ulock_wait`,
//! callers run a short, per-thread adaptive bounded spin (bounded by both
//! iteration count and monotonic cycle time via `CNTVCT_EL0` on ARM64, and
//! capped per wait object to at most 2 concurrent spinners to keep idle and
//! burst-tail CPU minimal). Threads that repeatedly enter long sleeps rapidly
//! decay their spin budget to `0` so idle CPU never regresses.
//!
//! `PORT_WAIT_SPIN_US` (alias `AW2_WAIT_SPIN_US`, default `16`, `0` = off):
//! maximum adaptive spin budget in microseconds before parking.
//! `PORT_WAKE_STATS=1` (alias `AW2_WAKE_STATS=1`): log wake/sleep/spin stats
//! every ~2 s.

use std::cell::Cell;
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

/// Default maximum adaptive spin budget (µs) before parking in `__ulock_wait`.
pub(crate) const DEFAULT_WAIT_SPIN_US: u64 = 16;
/// Hard upper bound on spin loop iterations regardless of clock speed.
const MAX_SPIN_ITERS: u32 = 1024;
/// Sleep duration (µs) above which a wait is considered an idle/long sleep for
/// adaptive decay.
const LONG_SLEEP_US: u64 = 200;
/// Number of long sleeps between brief 2 µs probes once an idle thread has
/// decayed to 0 spin budget.
const IDLE_PROBE_INTERVAL: u16 = 16;

const IDLE: u32 = 0;
const NOTIFIED: u32 = 1;
const SLEEPING: u32 = 2;

/// Cache-line aligned (128 B on Apple Silicon) per-thread parking word.
#[repr(align(128))]
pub struct Parker {
    word: AtomicU32,
}

struct Config {
    spin_max_us: u64,
    stats: bool,
}

fn env_u64_pair(primary: &str, alias: &str, default: u64) -> u64 {
    std::env::var(primary)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .or_else(|| std::env::var(alias).ok().and_then(|v| v.trim().parse().ok()))
        .unwrap_or(default)
}

fn env_flag_pair(primary: &str, alias: &str) -> bool {
    std::env::var(primary)
        .ok()
        .or_else(|| std::env::var(alias).ok())
        .is_some_and(|v| v.trim() == "1")
}

#[inline(always)]
fn config() -> &'static Config {
    static C: OnceLock<Config> = OnceLock::new();
    C.get_or_init(|| Config {
        spin_max_us: env_u64_pair("PORT_WAIT_SPIN_US", "AW2_WAIT_SPIN_US", DEFAULT_WAIT_SPIN_US),
        stats: env_flag_pair("PORT_WAKE_STATS", "AW2_WAKE_STATS"),
    })
}

static KERNEL_WAKES: AtomicU64 = AtomicU64::new(0);
static KERNEL_WAKE_NS: AtomicU64 = AtomicU64::new(0);
static CHEAP_WAKES: AtomicU64 = AtomicU64::new(0);
static COALESCED_WAKES: AtomicU64 = AtomicU64::new(0);
static SLEEPS: AtomicU64 = AtomicU64::new(0);
static SPIN_HITS: AtomicU64 = AtomicU64::new(0);
static NEXT_REPORT_NS: AtomicU64 = AtomicU64::new(0);

#[inline(always)]
fn now_ns() -> u64 {
    crate::hitch::now_ns()
}

fn maybe_report(now: u64) {
    let next = NEXT_REPORT_NS.load(Ordering::Relaxed);
    if now < next
        || NEXT_REPORT_NS
            .compare_exchange(next, now + 2_000_000_000, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
    {
        return;
    }
    let kw = KERNEL_WAKES.swap(0, Ordering::Relaxed);
    let ns = KERNEL_WAKE_NS.swap(0, Ordering::Relaxed);
    log::info!(
        "wake-stats: kernel_wakes={} ({:.2} us avg) cheap_wakes={} sleeps={} spin_hits={} coalesced={}",
        kw,
        if kw > 0 { ns as f64 / kw as f64 / 1000.0 } else { 0.0 },
        CHEAP_WAKES.swap(0, Ordering::Relaxed),
        SLEEPS.swap(0, Ordering::Relaxed),
        SPIN_HITS.swap(0, Ordering::Relaxed),
        COALESCED_WAKES.swap(0, Ordering::Relaxed),
    );
}

#[inline(always)]
pub(crate) fn record_spin_hit_stat() {
    if config().stats {
        SPIN_HITS.fetch_add(1, Ordering::Relaxed);
    }
}

#[inline(always)]
pub(crate) fn record_cheap_wake_stat() {
    if config().stats {
        CHEAP_WAKES.fetch_add(1, Ordering::Relaxed);
        maybe_report(now_ns());
    }
}

#[inline(always)]
pub(crate) fn record_coalesced_wake_stat() {
    if config().stats {
        COALESCED_WAKES.fetch_add(1, Ordering::Relaxed);
        maybe_report(now_ns());
    }
}

// ---------------------------------------------------------------------------
// Fast monotonic tick clock (zero-syscall CNTVCT_EL0 on ARM64)
// ---------------------------------------------------------------------------

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(crate) fn ticks_now() -> u64 {
    let v: u64;
    unsafe {
        std::arch::asm!("mrs {}, cntvct_el0", out(reg) v, options(nomem, nostack, preserves_flags));
    }
    v
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub(crate) fn ticks_now() -> u64 {
    crate::hitch::now_ns()
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn ticks_freq() -> u64 {
    static FREQ: OnceLock<u64> = OnceLock::new();
    *FREQ.get_or_init(|| {
        let f: u64;
        unsafe {
            std::arch::asm!("mrs {}, cntfrq_el0", out(reg) f, options(nomem, nostack, preserves_flags));
        }
        f.max(1)
    })
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(crate) fn us_to_ticks(us: u64) -> u64 {
    (us.saturating_mul(ticks_freq()) / 1_000_000).max(1)
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub(crate) fn us_to_ticks(us: u64) -> u64 {
    us.saturating_mul(1_000).max(1)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(crate) fn ticks_to_us(ticks: u64) -> u64 {
    ticks.saturating_mul(1_000_000) / ticks_freq()
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub(crate) fn ticks_to_us(ticks: u64) -> u64 {
    ticks / 1_000
}

// ---------------------------------------------------------------------------
// Per-thread adaptive bounded spin controller
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct ThreadSpinState {
    /// Current spin budget in microseconds (`0..=max_spin_us`).
    budget_us: u32,
    /// Consecutive spin misses followed by long sleeps.
    miss_streak: u16,
    /// Countdown between brief 2 µs probes while decayed to 0.
    probe_countdown: u16,
}

impl ThreadSpinState {
    const fn new() -> Self {
        Self {
            budget_us: 4,
            miss_streak: 0,
            probe_countdown: 0,
        }
    }

    #[inline]
    fn next_budget_us(&mut self, cap_us: u64) -> u64 {
        if cap_us == 0 {
            return 0;
        }
        let cap32 = cap_us.min(u32::MAX as u64) as u32;
        if self.budget_us > 0 {
            return self.budget_us.min(cap32) as u64;
        }
        if self.probe_countdown == 0 {
            self.probe_countdown = IDLE_PROBE_INTERVAL;
            return 2u64.min(cap_us);
        }
        self.probe_countdown -= 1;
        0
    }

    #[inline]
    fn on_spin_hit(&mut self, cap_us: u64) {
        if cap_us == 0 {
            return;
        }
        let cap32 = cap_us.min(u32::MAX as u64) as u32;
        self.miss_streak = 0;
        self.probe_countdown = 0;
        self.budget_us = self.budget_us.max(2).saturating_mul(2).min(cap32);
    }

    #[inline]
    fn on_sleep_done(&mut self, sleep_us: u64, timed_out: bool, cap_us: u64) {
        if cap_us == 0 {
            return;
        }
        let cap32 = cap_us.min(u32::MAX as u64) as u32;
        if timed_out || sleep_us >= LONG_SLEEP_US {
            self.miss_streak = self.miss_streak.saturating_add(1);
            if self.miss_streak >= 3 {
                self.budget_us = 0;
                self.probe_countdown = IDLE_PROBE_INTERVAL;
            } else {
                self.budget_us /= 2;
            }
        } else if sleep_us < cap_us.saturating_mul(2) {
            // Very short sleep that just missed our spin window: gently expand.
            self.miss_streak = 0;
            self.budget_us = self.budget_us.saturating_add(2).min(cap32);
        } else {
            self.miss_streak = self.miss_streak.saturating_add(1);
            self.budget_us = (self.budget_us * 3) / 4;
        }
    }
}

thread_local! {
    static SPIN_STATE: Cell<ThreadSpinState> = const { Cell::new(ThreadSpinState::new()) };
}

#[inline(always)]
pub(crate) fn spin_enabled() -> bool {
    config().spin_max_us > 0
}

/// Compute the current thread's adaptive spin budget (µs), capped by `cap_us`
/// and by any remaining time before `deadline`.
#[inline]
pub(crate) fn adaptive_spin_budget_us(deadline: Option<Instant>, cap_us: u64) -> u64 {
    let eff_cap = config().spin_max_us.min(cap_us);
    if eff_cap == 0 {
        return 0;
    }
    let mut budget = SPIN_STATE
        .try_with(|c| {
            let mut st = c.get();
            let b = st.next_budget_us(eff_cap);
            c.set(st);
            b
        })
        .unwrap_or(0);
    if budget == 0 {
        return 0;
    }
    if let Some(d) = deadline {
        let now = Instant::now();
        if now >= d {
            return 0;
        }
        let rem_us = (d - now).as_micros().min(u64::MAX as u128) as u64;
        if rem_us == 0 {
            return 0;
        }
        budget = budget.min(rem_us);
    }
    budget
}

#[inline]
pub(crate) fn record_spin_hit() {
    let cap = config().spin_max_us;
    let _ = SPIN_STATE.try_with(|c| {
        let mut st = c.get();
        st.on_spin_hit(cap);
        c.set(st);
    });
    record_spin_hit_stat();
}

#[inline]
pub(crate) fn record_sleep_outcome(sleep_us: u64, timed_out: bool) {
    let cap = config().spin_max_us;
    let _ = SPIN_STATE.try_with(|c| {
        let mut st = c.get();
        st.on_sleep_done(sleep_us, timed_out, cap);
        c.set(st);
    });
}

/// Bounded adaptive spin for wait predicates (events, semaphores, condvars,
/// `WaitOnAddress`, SRW locks). Strictly bounded by both `MAX_SPIN_ITERS` and
/// `budget_us` using zero-syscall `CNTVCT_EL0` ticks on ARM64.
#[inline]
pub(crate) fn spin_until(
    deadline: Option<Instant>,
    cap_us: u64,
    mut check: impl FnMut() -> bool,
) -> bool {
    let budget_us = adaptive_spin_budget_us(deadline, cap_us);
    if budget_us == 0 {
        return false;
    }
    let start = ticks_now();
    let max_ticks = us_to_ticks(budget_us);
    for i in 0..MAX_SPIN_ITERS {
        if check() {
            record_spin_hit();
            return true;
        }
        std::hint::spin_loop();
        if (i & 15) == 15 && ticks_now().wrapping_sub(start) >= max_ticks {
            return false;
        }
    }
    false
}

#[inline]
fn deadline_timeout_us(deadline: Option<Instant>) -> Option<u32> {
    match deadline {
        None => Some(0), // infinite
        Some(d) => {
            let now = Instant::now();
            if now >= d {
                None
            } else {
                Some(((d - now).as_micros().min(u32::MAX as u128) as u32).max(1))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Per-thread Parker
// ---------------------------------------------------------------------------

impl Default for Parker {
    fn default() -> Self {
        Self::new()
    }
}

impl Parker {
    pub const fn new() -> Self {
        Parker { word: AtomicU32::new(IDLE) }
    }

    fn addr(&self) -> *mut c_void {
        self.word.as_ptr() as *mut c_void
    }

    /// True if the owning thread is currently advertised as `SLEEPING` in `__ulock_wait`.
    #[inline]
    pub fn is_sleeping(&self) -> bool {
        self.word.load(Ordering::Relaxed) == SLEEPING
    }

    /// Reset before re-checking/sleeping.
    #[inline]
    pub fn arm(&self) {
        self.word.store(IDLE, Ordering::Release);
    }

    /// Block with adaptive bounded spin before sleeping in `__ulock_wait`.
    /// Returns `true` if `__ulock_wait` was interrupted by `EINTR` for a pending
    /// `TerminateThread` request.
    pub fn park(&self, deadline: Option<Instant>) -> bool {
        if self.word.load(Ordering::Acquire) == NOTIFIED {
            return false;
        }
        if spin_until(deadline, u64::MAX, || self.word.load(Ordering::Acquire) == NOTIFIED) {
            return false;
        }
        self.park_sleep_only(deadline)
    }

    /// Block directly in `__ulock_wait` without an additional spin phase (used
    /// when the caller already performed its object-level capped spin).
    /// Returns `true` if `__ulock_wait` was interrupted by `EINTR` for a pending
    /// `TerminateThread` request.
    pub fn park_sleep_only(&self, deadline: Option<Instant>) -> bool {
        if self.word.load(Ordering::Acquire) == NOTIFIED {
            return false;
        }
        if self
            .word
            .compare_exchange(IDLE, SLEEPING, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false; // already notified
        }
        let cfg = config();
        let sleep_start = ticks_now();
        let mut timed_out = false;
        let mut interrupted = false;
        loop {
            let Some(timeout_us) = deadline_timeout_us(deadline) else {
                timed_out = true;
                break;
            };
            if cfg.stats {
                SLEEPS.fetch_add(1, Ordering::Relaxed);
            }
            let rc = unsafe {
                __ulock_wait(
                    UL_COMPARE_AND_WAIT | ULF_NO_ERRNO,
                    self.addr(),
                    SLEEPING as u64,
                    timeout_us,
                )
            };
            if self.word.load(Ordering::Acquire) != SLEEPING {
                let sleep_us = ticks_to_us(ticks_now().wrapping_sub(sleep_start));
                record_sleep_outcome(sleep_us, false);
                return false;
            }
            if rc == -libc::EINTR && crate::thread::is_current_terminate_requested() {
                interrupted = true;
                break;
            }
        }
        // Timed out or interrupted for termination: stop advertising SLEEPING so later notifiers skip the syscall.
        let _ = self
            .word
            .compare_exchange(SLEEPING, IDLE, Ordering::AcqRel, Ordering::Acquire);
        let sleep_us = ticks_to_us(ticks_now().wrapping_sub(sleep_start));
        record_sleep_outcome(sleep_us, timed_out);
        interrupted
    }

    /// Wake the owner; enters the kernel only if it is actually asleep.
    #[inline]
    pub fn unpark(&self) {
        let prev = self.word.swap(NOTIFIED, Ordering::AcqRel);
        let cfg = config();
        if prev == SLEEPING {
            let t0 = cfg.stats.then(now_ns);
            unsafe { __ulock_wake(UL_COMPARE_AND_WAIT | ULF_NO_ERRNO, self.addr(), 0) };
            if let Some(t0) = t0 {
                let t1 = now_ns();
                KERNEL_WAKES.fetch_add(1, Ordering::Relaxed);
                KERNEL_WAKE_NS.fetch_add(t1.saturating_sub(t0), Ordering::Relaxed);
                maybe_report(t1);
            }
        } else if cfg.stats {
            if prev == NOTIFIED {
                COALESCED_WAKES.fetch_add(1, Ordering::Relaxed);
            } else {
                CHEAP_WAKES.fetch_add(1, Ordering::Relaxed);
            }
            maybe_report(now_ns());
        }
    }
}

/// Minimal mutex word for very short critical sections (`handles::WAIT_LOCK`, wait
/// queues): CAS fast path, bounded exponential-backoff spin while the holder is
/// running, then `__ulock_wait` (Drepper's three-state futex mutex).
pub struct WordLock {
    /// 0 unlocked, 1 locked, 2 locked with (possible) sleepers.
    word: AtomicU32,
}

impl Default for WordLock {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLock {
    pub const fn new() -> Self {
        WordLock { word: AtomicU32::new(0) }
    }

    #[inline]
    pub fn lock(&self) {
        crate::thread::defer_terminate_enter();
        if self
            .word
            .compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            self.lock_slow();
        }
    }

    #[cold]
    fn lock_slow(&self) {
        let mut backoff = 1u32;
        let mut total = 0u32;
        while total < 200 {
            let s = self.word.load(Ordering::Relaxed);
            if s == 0
                && self
                    .word
                    .compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
            {
                return;
            }
            let n = backoff.min(16);
            for _ in 0..n {
                std::hint::spin_loop();
            }
            total += n;
            backoff = (backoff << 1).min(16);
        }
        // Mark contended and sleep until the word changes.
        while self.word.swap(2, Ordering::Acquire) != 0 {
            unsafe {
                __ulock_wait(
                    UL_COMPARE_AND_WAIT | ULF_NO_ERRNO,
                    self.word.as_ptr() as *mut c_void,
                    2,
                    0,
                )
            };
        }
    }

    #[inline]
    pub fn unlock(&self) {
        if self.word.swap(0, Ordering::Release) == 2 {
            unsafe {
                __ulock_wake(
                    UL_COMPARE_AND_WAIT | ULF_NO_ERRNO,
                    self.word.as_ptr() as *mut c_void,
                    0,
                )
            };
        }
        crate::thread::defer_terminate_leave();
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

    #[test]
    fn adaptive_spin_decays_to_zero_on_long_sleeps_and_ramps_on_hits() {
        let mut st = ThreadSpinState::new();
        assert_eq!(st.next_budget_us(16), 4);
        // Three long sleeps must decay budget to 0.
        st.on_sleep_done(500, false, 16);
        assert_eq!(st.budget_us, 2);
        st.on_sleep_done(500, false, 16);
        assert_eq!(st.budget_us, 1);
        st.on_sleep_done(500, false, 16);
        assert_eq!(st.budget_us, 0);
        // Next 16 calls return 0 (no spinning while idle), then one 2 us probe.
        for _ in 0..IDLE_PROBE_INTERVAL {
            assert_eq!(st.next_budget_us(16), 0);
        }
        assert_eq!(st.next_budget_us(16), 2);
        // A spin hit immediately ramps up to cap.
        st.on_spin_hit(16);
        assert!(st.budget_us >= 4);
        for _ in 0..4 {
            st.on_spin_hit(16);
        }
        assert_eq!(st.budget_us, 16);
    }
}
