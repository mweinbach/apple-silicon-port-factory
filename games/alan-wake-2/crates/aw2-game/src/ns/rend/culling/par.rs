//! Persistent native helper pool for the per-instance / per-submesh cull loops.
//!
//! Gated by `AW2_PAR_CULL` (default off): unset/`0` = serial (the original single-threaded
//! loops), `1` = 4 lanes (caller + 3 helpers), `2..=16` = that many lanes in total.
//!
//! [`for_chunks`] runs `f(ctx, 0..n)`. Serially that is chunk order `0, 1, .., n-1`, i.e. the
//! original element order. In parallel the chunks are claimed dynamically by the caller and the
//! helpers; the kernels only use it where every chunk writes disjoint outputs and reads nothing
//! another chunk writes, so the result is bit-identical either way. Helpers run only native
//! code (plain guest-memory loads/stores, no emulator calls); the caller waits for all chunks
//! (acquire on the `done` release sequence) before returning, so the job's own release points
//! (scheduler push, SeqCst pass-counter `fetch_sub`) still publish everything.
//!
//! Helpers spin briefly after each task (jobs come in bursts) and then park; there is no
//! per-call thread spawn. One task at a time: a second concurrent caller just runs serially.

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::thread::Thread;
use std::time::{Duration, Instant};

/// `f(ctx, chunk)`.
pub(super) type ChunkFn = unsafe fn(*const (), usize);

const MAX_LANES: usize = 16;
const SPIN: Duration = Duration::from_micros(40);

struct Pool {
    busy: AtomicBool,
    open: AtomicBool,
    gen: AtomicU64,
    func: AtomicUsize,
    ctx: AtomicUsize,
    nchunks: AtomicUsize,
    next: AtomicUsize,
    done: AtomicUsize,
    active: AtomicUsize,
    parked: Vec<AtomicBool>,
    /// Per helper: entered its loop / last task generation it picked up / chunks it ran
    /// (stall diagnostics only).
    started: Vec<AtomicBool>,
    seen: Vec<AtomicU64>,
    chunks: Vec<AtomicU64>,
    threads: OnceLock<Vec<Thread>>,
}

static POOL: OnceLock<&'static Pool> = OnceLock::new();
static FORCE_LANES: AtomicUsize = AtomicUsize::new(0);
static CALLER_IDLE: AtomicBool = AtomicBool::new(false);
/// Tasks actually dispatched to the pool / chunks run by helper threads (diagnostics, tests).
pub(super) static PAR_TASKS: AtomicU64 = AtomicU64::new(0);
pub(super) static HELPER_CHUNKS: AtomicU64 = AtomicU64::new(0);
/// Test-only (`ForceLanes` caller-idle runs): how long the idle caller waits for the helpers
/// before finishing the task itself and recording a [`take_stall`] diagnostic.
const IDLE_WAIT: Duration = Duration::from_secs(10);
static STALL: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// The diagnostic of the last caller-idle task whose helpers did not finish in time.
pub(super) fn take_stall() -> Option<String> {
    STALL.lock().ok()?.take()
}

fn pool_state(p: &Pool, n: usize) -> String {
    let flags = |v: &[AtomicBool]| v.iter().map(|b| if b.load(Ordering::SeqCst) { '1' } else { '0' }).collect::<String>();
    let seen: Vec<u64> = p.seen.iter().map(|g| g.load(Ordering::SeqCst)).collect();
    let chunks: Vec<u64> = p.chunks.iter().map(|c| c.load(Ordering::SeqCst)).collect();
    format!(
        "helpers started={} parked={} seen_gen={seen:?} chunks={chunks:?}; gen={} open={} busy={} next={} done={} of {n} active={}",
        flags(&p.started),
        flags(&p.parked),
        p.gen.load(Ordering::SeqCst),
        p.open.load(Ordering::SeqCst),
        p.busy.load(Ordering::SeqCst),
        p.next.load(Ordering::SeqCst),
        p.done.load(Ordering::SeqCst),
        p.active.load(Ordering::SeqCst),
    )
}

fn env_lanes() -> usize {
    static V: OnceLock<usize> = OnceLock::new();
    *V.get_or_init(|| {
        let n = std::env::var("AW2_PAR_CULL").ok().and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0);
        let lanes = match n {
            0 => 0,
            1 => 4,
            n => n.min(MAX_LANES),
        };
        if lanes > 1 {
            log::info!("AW2_PAR_CULL: parallel cull loops with {lanes} lanes");
        }
        lanes
    })
}

/// Lanes for the parallel cull paths (`< 2` = serial).
#[inline]
pub(super) fn lanes() -> usize {
    let f = FORCE_LANES.load(Ordering::Relaxed);
    if f != 0 {
        f
    } else {
        env_lanes()
    }
}

/// Test hook: force `lanes` (0 = back to the env setting) and, with `caller_idle`, make the
/// submitting thread wait instead of claiming chunks so the helpers run every chunk.
pub(super) struct ForceLanes;
impl ForceLanes {
    pub(super) fn new(lanes: usize, caller_idle: bool) -> ForceLanes {
        FORCE_LANES.store(lanes.min(MAX_LANES), Ordering::SeqCst);
        CALLER_IDLE.store(caller_idle, Ordering::SeqCst);
        ForceLanes
    }
}
impl Drop for ForceLanes {
    fn drop(&mut self) {
        FORCE_LANES.store(0, Ordering::SeqCst);
        CALLER_IDLE.store(false, Ordering::SeqCst);
    }
}

#[cfg(target_os = "macos")]
fn set_helper_qos() {
    extern "C" {
        fn pthread_set_qos_class_self_np(qos_class: u32, relative_priority: i32) -> i32;
    }
    const QOS_CLASS_USER_INTERACTIVE: u32 = 0x21;
    unsafe {
        pthread_set_qos_class_self_np(QOS_CLASS_USER_INTERACTIVE, 0);
    }
}
#[cfg(not(target_os = "macos"))]
fn set_helper_qos() {}

fn pool(lanes: usize) -> &'static Pool {
    POOL.get_or_init(|| {
        let helpers = lanes.clamp(2, MAX_LANES) - 1;
        let p: &'static Pool = Box::leak(Box::new(Pool {
            busy: AtomicBool::new(false),
            open: AtomicBool::new(false),
            gen: AtomicU64::new(0),
            func: AtomicUsize::new(0),
            ctx: AtomicUsize::new(0),
            nchunks: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
            done: AtomicUsize::new(0),
            active: AtomicUsize::new(0),
            parked: (0..helpers).map(|_| AtomicBool::new(false)).collect(),
            started: (0..helpers).map(|_| AtomicBool::new(false)).collect(),
            seen: (0..helpers).map(|_| AtomicU64::new(0)).collect(),
            chunks: (0..helpers).map(|_| AtomicU64::new(0)).collect(),
            threads: OnceLock::new(),
        }));
        let mut threads = Vec::with_capacity(helpers);
        let gen0 = p.gen.load(Ordering::SeqCst);
        for i in 0..helpers {
            let h = std::thread::Builder::new()
                .name(format!("aw2-par-cull-{i}"))
                .stack_size(256 * 1024)
                .spawn(move || helper(p, i, gen0))
                .expect("spawn aw2-par-cull helper");
            threads.push(h.thread().clone());
        }
        let _ = p.threads.set(threads);
        p
    })
}

fn helper(p: &'static Pool, idx: usize, gen0: u64) {
    set_helper_qos();
    p.started[idx].store(true, Ordering::SeqCst);
    // The generation when the pool spawned us, not when this thread first runs: the first task
    // may already be posted (gen bumped) by then, and must not be taken as seen.
    let mut last = gen0;
    loop {
        let t0 = Instant::now();
        let mut spins = 0u32;
        loop {
            let g = p.gen.load(Ordering::SeqCst);
            if g != last {
                last = g;
                break;
            }
            spins = spins.wrapping_add(1);
            if spins & 63 != 0 || t0.elapsed() < SPIN {
                std::hint::spin_loop();
                continue;
            }
            // Dekker with the submitter (gen bump, then `parked` check): no lost wakeups.
            p.parked[idx].store(true, Ordering::SeqCst);
            if p.gen.load(Ordering::SeqCst) == last {
                std::thread::park();
            }
            p.parked[idx].store(false, Ordering::SeqCst);
        }
        p.seen[idx].store(last, Ordering::Relaxed);
        // Dekker with the submitter's close (`open = false`, then wait `active == 0`): either
        // it waits for us, or we see the task closed and never touch it.
        p.active.fetch_add(1, Ordering::SeqCst);
        if p.open.load(Ordering::SeqCst) {
            let f: ChunkFn = unsafe { std::mem::transmute(p.func.load(Ordering::Acquire)) };
            let ctx = p.ctx.load(Ordering::Acquire) as *const ();
            let n = p.nchunks.load(Ordering::Acquire);
            let mut ran = 0u64;
            loop {
                let i = p.next.fetch_add(1, Ordering::AcqRel);
                if i >= n {
                    break;
                }
                unsafe { f(ctx, i) };
                ran += 1;
                HELPER_CHUNKS.fetch_add(1, Ordering::Relaxed);
                p.done.fetch_add(1, Ordering::Release);
            }
            p.chunks[idx].fetch_add(ran, Ordering::Relaxed);
        }
        p.active.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Run `f(ctx, i)` for `i in 0..n`: in order on this thread when parallelism is off (or the
/// pool is busy with another caller), else spread over the helper pool.
///
/// # Safety
/// `f` must be safe to call concurrently for distinct chunks with `ctx` (disjoint outputs, no
/// emulator calls), and `ctx` must stay valid until this returns.
pub(super) unsafe fn for_chunks(n: usize, ctx: *const (), f: ChunkFn) {
    let lanes = lanes();
    if lanes < 2 || n < 2 {
        for i in 0..n {
            f(ctx, i);
        }
        return;
    }
    let p = pool(lanes);
    if p.busy.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
        for i in 0..n {
            f(ctx, i);
        }
        return;
    }
    // A late helper may still be inside the previous (closed, exhausted) task.
    while p.active.load(Ordering::SeqCst) != 0 {
        std::hint::spin_loop();
    }
    p.func.store(f as usize, Ordering::Relaxed);
    p.ctx.store(ctx as usize, Ordering::Relaxed);
    p.nchunks.store(n, Ordering::Relaxed);
    p.next.store(0, Ordering::Relaxed);
    p.done.store(0, Ordering::Relaxed);
    p.open.store(true, Ordering::SeqCst);
    p.gen.fetch_add(1, Ordering::SeqCst);
    PAR_TASKS.fetch_add(1, Ordering::Relaxed);
    if let Some(threads) = p.threads.get() {
        for (i, t) in threads.iter().enumerate() {
            if p.parked[i].load(Ordering::SeqCst) {
                t.unpark();
            }
        }
    }
    if !CALLER_IDLE.load(Ordering::Relaxed) {
        loop {
            let i = p.next.fetch_add(1, Ordering::AcqRel);
            if i >= n {
                break;
            }
            f(ctx, i);
            p.done.fetch_add(1, Ordering::Release);
        }
    } else {
        idle_wait(p, n, ctx, f);
    }
    while p.done.load(Ordering::Acquire) != n {
        std::hint::spin_loop();
    }
    p.open.store(false, Ordering::SeqCst);
    while p.active.load(Ordering::SeqCst) != 0 {
        std::hint::spin_loop();
    }
    p.busy.store(false, Ordering::Release);
}

/// [`ForceLanes`] caller-idle wait (tests only): the helpers must run every chunk. If they have
/// not finished after [`IDLE_WAIT`], record the pool state for [`take_stall`] and run the
/// unclaimed chunks here; chunks a helper claimed but never finished make it panic with the
/// state instead (`ctx` is on the caller's stack: returning would leave the helper with it).
#[cold]
#[inline(never)]
unsafe fn idle_wait(p: &Pool, n: usize, ctx: *const (), f: ChunkFn) {
    let t0 = Instant::now();
    while p.done.load(Ordering::Acquire) != n {
        if t0.elapsed() > IDLE_WAIT {
            let state = pool_state(p, n);
            log::error!("aw2-par-cull: caller-idle task stalled after {IDLE_WAIT:?}: {state}");
            if let Ok(mut s) = STALL.lock() {
                *s = Some(format!("helpers did not finish a caller-idle task in {IDLE_WAIT:?}: {state}"));
            }
            loop {
                let i = p.next.fetch_add(1, Ordering::AcqRel);
                if i >= n {
                    break;
                }
                f(ctx, i);
                p.done.fetch_add(1, Ordering::Release);
            }
            let t1 = Instant::now();
            while p.done.load(Ordering::Acquire) != n {
                if t1.elapsed() > IDLE_WAIT {
                    panic!("aw2-par-cull: claimed chunks never finished: {}", pool_state(p, n));
                }
                std::hint::spin_loop();
            }
            return;
        }
        std::hint::spin_loop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn mark(ctx: *const (), i: usize) {
        let cells = &*(ctx as *const [AtomicU64; 64]);
        cells[i].fetch_add(1, Ordering::Relaxed);
    }

    /// The helpers must run a caller-idle task even when it is the pool's first task, posted
    /// before the freshly spawned helper threads get scheduled (they used to take its
    /// generation as already seen and park: the `rend::` difftest hung at
    /// evaluateSubmeshLodAndCulling_142259370). Repeated tasks must all complete too.
    #[test]
    fn caller_idle_tasks_run_on_helpers_from_the_first_one() {
        let _f = ForceLanes::new(4, true);
        let _ = take_stall();
        for round in 0..50 {
            let cells: [AtomicU64; 64] = std::array::from_fn(|_| AtomicU64::new(0));
            let h0 = HELPER_CHUNKS.load(Ordering::SeqCst);
            unsafe { for_chunks(64, &cells as *const _ as *const (), mark) };
            assert_eq!(take_stall(), None, "round {round}");
            assert!(cells.iter().all(|c| c.load(Ordering::Relaxed) == 1), "round {round}: every chunk exactly once");
            assert!(HELPER_CHUNKS.load(Ordering::SeqCst) >= h0 + 64, "round {round}: helpers ran every chunk");
        }
        // Every helper thread enters its loop eventually. The rounds above may all have been
        // served by the helpers the OS scheduled first (a busy machine can leave a freshly
        // spawned thread unscheduled for longer than 50 short tasks take), so wait for the
        // rest with a bound instead of asserting at once.
        let p = POOL.get().expect("pool");
        let t0 = Instant::now();
        while !p.started.iter().all(|s| s.load(Ordering::SeqCst)) && t0.elapsed() < IDLE_WAIT {
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(p.started.iter().all(|s| s.load(Ordering::SeqCst)), "helpers not started after {IDLE_WAIT:?}: {}", pool_state(p, 0));
    }
}
