//! Northlight ECS world step (`0x141f818b0`) and phase runner (`0x1420ac1c0`), plus
//! opt-in **Threaded Server World Step** (`AW2_THREADED_WORLD_STEP=1`) and phase tracing
//! (`AW2_WORLD_STEP_TRACE=1`).
//!
//! ## Architecture
//!
//! `GameServer::vfunc_9` (`0x1418437a0`) advances two ECS worlds each frame:
//! - `"main world"` (`[0x14397da68]`)
//! - `"timeline world"` (`[0x14397da60]`)
//!
//! Each call invokes `world_step` (`0x141f818b0`) -> `ecs_runner_run` (`0x1420ac1c0`),
//! which walks the active gameloop's phase list (`SchedGraph` entries, stride `0x40`).
//! Phases with `count <= 1` and `(sys_desc[+0x110] != 0 || sys_desc[+0x111] != 0)` are
//! single-system serial barrier phases; all other phases dispatch a parallel DAG onto
//! `r::ThreadManager` (`[0x143a082f8]`) via `start_scheduling` (`0x1420a8fd0`).
//!
//! ## Threaded Server World Step (`AW2_THREADED_WORLD_STEP`)
//!
//! When `AW2_THREADED_WORLD_STEP=1` (or `2`), `world_step` offloads the server world step
//! onto a persistent Win32-adopted `ServerWorldStep` thread registered with Northlight's
//! `ThreadDesc` table (`0x143c4aa00`, mode `0xc` so `is_in_parallel_world_step` `0x141f81a50`
//! stays true and priorities `0..3` including PhysX are pumped) and `r::ThreadManager`
//! (`0x142485490`), while `GameThread` (`mode == 1`) cooperatively:
//! 1. Pumps `scheduler_pump_one` — servicing priority-4 `"Before UI Update Range"`
//!    (`0x143827b24`) jobs on the real `GameThread` (preventing priority-4 deadlock and
//!    keeping Cohtml/V8 UI thread-affine) as well as stealing priority-2 ECS DAG jobs.
//! 2. Executes any main-thread-affine serial barrier systems (`+0x110`, and optionally
//!    `+0x111` depending on `AW2_THREADED_WORLD_STEP_BARRIER`) synchronously on `GameThread`.
//! 3. Flushes `ServerWorldStep`'s scratch pad (`0x14241bbe0` -> `0x1424b3fa0`) and
//!    per-worker allocator reclaim (`0x143a082f0` -> `0x142475db0`) at step completion.

use crate::exe::{self, rd, wr};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::OnceLock;
use std::thread::Thread;
use std::time::{Duration, Instant};

const VA_MAIN_WORLD_PTR: u64 = 0x1_4397_da68;
const VA_TIMELINE_WORLD_PTR: u64 = 0x1_4397_da60;
const VA_ECS_MT_ENABLED: u64 = 0x1_4386_1638;
const VA_PRIO4_UI_SYS_ID: u64 = 0x1_4382_7b24;
const VA_JOB_SYSTEM_PTR: u64 = 0x1_43a0_82f8;
const VA_ALLOC_MGR_PTR: u64 = 0x1_43a0_82f0;
const VA_WORKER_DESC_TABLE: u64 = 0x1_43c4_aa00;
const VA_RENDER_CMD_DIRTY_BASE: u64 = 0x1_43c1_6ad0;

const VA_RUNNER_BIND: u64 = 0x1_420a_bf40;
const VA_RUNNER_UNBIND: u64 = 0x1_420a_bdc0;
const VA_START_SCHEDULING: u64 = 0x1_420a_8fd0;
const VA_REGISTER_THREAD_DESC: u64 = 0x1_4240_8090;
const VA_REGISTER_JOB_WORKER: u64 = 0x1_4248_5490;
const VA_SCRATCH_TLS: u64 = 0x1_4241_bbe0;
const VA_SCRATCH_FLUSH: u64 = 0x1_424b_3fa0;
const VA_ALLOC_RECLAIM: u64 = 0x1_4247_5db0;
const IAT_BAD_FUNCTION_CALL: u64 = 0x1_4264_5878;

// Thread-local flag indicating we are currently executing on the dedicated
// `ServerWorldStep` thread (so nested `ecs_runner_run` knows whether to route
// main-thread-affine barrier systems back to `GameThread`).
std::thread_local! {
    static IS_SERVER_WORLD_THREAD: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// `AW2_THREADED_WORLD_STEP`:
/// - `0` / unset: off (original inline execution on `GameThread`)
/// - `1`: offload `world_step` onto persistent `ServerWorldStep` thread while `GameThread`
///   cooperatively pumps priority-4 UI + priority-2 ECS jobs and main-thread barriers.
#[inline]
fn threaded_mode() -> u32 {
    static MODE: OnceLock<u32> = OnceLock::new();
    *MODE.get_or_init(|| {
        let v = std::env::var("AW2_THREADED_WORLD_STEP")
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .unwrap_or(0);
        if v != 0 {
            log::info!(
                "world_step: AW2_THREADED_WORLD_STEP={v} enabled (barrier_policy={})",
                barrier_policy()
            );
        }
        v
    })
}

/// `AW2_THREADED_WORLD_STEP_BARRIER`:
/// - `"110"` (default): route `+0x110` main-thread-affine barriers to `GameThread` while
///   running `+0x111` pure ECS structural flushes (`0x1410e6a70`) on `ServerWorldStep`.
/// - `"all"`: route both `+0x110` and `+0x111` serial barrier systems to `GameThread`.
/// - `"none"`: run all serial barrier systems directly on `ServerWorldStep` (experimental).
#[inline]
fn barrier_policy() -> &'static str {
    static POL: OnceLock<&'static str> = OnceLock::new();
    *POL.get_or_init(|| {
        match std::env::var("AW2_THREADED_WORLD_STEP_BARRIER")
            .ok()
            .map(|s| s.trim().to_ascii_lowercase())
            .as_deref()
        {
            Some("all") => "all",
            Some("none") => "none",
            _ => "110",
        }
    })
}

/// `AW2_WORLD_STEP_TRACE`: if set to `N > 0`, logs phase details (barrier flags, fn VA,
/// duration) for the first `N` world-step frames with non-trivial work (or `1` -> 4 frames).
#[inline]
fn trace_limit() -> u32 {
    static LIM: OnceLock<u32> = OnceLock::new();
    *LIM.get_or_init(|| {
        match std::env::var("AW2_WORLD_STEP_TRACE")
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
        {
            Some(0) | None => 0,
            Some(1) => 4,
            Some(n) => n,
        }
    })
}

static TRACE_LOGGED_STEPS: AtomicU32 = AtomicU32::new(0);
static TOTAL_OFFLOADED_STEPS: AtomicU64 = AtomicU64::new(0);
static TOTAL_MAIN_BARRIERS: AtomicU64 = AtomicU64::new(0);
static TOTAL_SERVER_BARRIERS: AtomicU64 = AtomicU64::new(0);
static TOTAL_GAME_PUMPED_JOBS: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// Persistent Win32 ServerWorldStep thread state machine
// ---------------------------------------------------------------------------

const ST_IDLE: u32 = 0;
const ST_RUNNING: u32 = 1;
const ST_NEED_MAIN_BARRIER: u32 = 2;
const ST_MAIN_BARRIER_DONE: u32 = 3;
const ST_STEP_DONE: u32 = 4;

const WORKER_SPIN: Duration = Duration::from_micros(60);

struct ServerWorldWorker {
    state: AtomicU32,
    ready: AtomicU32, // 0 = starting, 1 = registered & ready, 2 = init failed
    worker_wid: AtomicU32,
    worker_parked: AtomicBool,
    /// True while `ServerWorldStep` is running a DAG phase that contains the priority-4
    /// `"Before UI Update Range"` system (`0x143827b24`), signaling `GameThread` to reserve
    /// itself for instant priority-4 service instead of getting stuck behind a stolen prio-2 job.
    prio4_phase_active: AtomicBool,
    // Step request args (written before ST_IDLE -> ST_RUNNING Release):
    req_world_holder: AtomicU64,
    req_mode: AtomicU32,
    // Main-thread barrier handoff args (written before ST_RUNNING -> ST_NEED_MAIN_BARRIER Release):
    bar_fn: AtomicU64,
    bar_arg0: AtomicU64,
    bar_arg1: AtomicU64,
    bar_arg2: AtomicU64,
    // Worker thread handle for unpark:
    worker_thread: OnceLock<Thread>,
}

static WORKER: OnceLock<Option<&'static ServerWorldWorker>> = OnceLock::new();

#[cfg(target_os = "macos")]
fn set_worker_qos() {
    extern "C" {
        fn pthread_set_qos_class_self_np(qos_class: u32, relative_priority: i32) -> i32;
    }
    const QOS_CLASS_USER_INTERACTIVE: u32 = 0x21;
    unsafe {
        pthread_set_qos_class_self_np(QOS_CLASS_USER_INTERACTIVE, 0);
    }
}

#[cfg(not(target_os = "macos"))]
fn set_worker_qos() {}

/// Register the current Win32 thread with Northlight's `ThreadDesc` table (`0x143c4aa00`)
/// and `r::ThreadManager` (`[0x143a082f8]`), returning its worker index if valid (`< 64`).
unsafe fn register_server_world_thread() -> Option<u32> {
    set_worker_qos();
    let sched_mgr = rd::<u64>(VA_JOB_SYSTEM_PTR as usize);
    if sched_mgr == 0 {
        return None;
    }
    let inner_sched = rd::<u64>(sched_mgr as usize + 0x10);
    if inner_sched == 0 {
        return None;
    }

    let wid = crate::ns::r::thread::worker_index();
    if wid >= 64 {
        log::error!("world_step: ServerWorldStep got out-of-range worker_index {wid}");
        return None;
    }

    // Verify per-worker job pool slot exists in `inner_sched`.
    let pool_base = rd::<u64>(inner_sched as usize + ((wid as usize) << 6) + 0x48);
    if pool_base == 0 {
        log::error!("world_step: ServerWorldStep wid={wid} has null job pool");
        return None;
    }

    // Populate per-thread ThreadDesc in exe static TLS (`tls0 + 0xa86c0`, 0x100 bytes),
    // matching `0x141856b20` GameThread init, EXCEPT:
    // - `mode = 0xc` (parallel world step mode: `is_in_parallel_world_step` returns true,
    //   and `scheduler_pop_*` pops priorities 0..3 including PhysX + ECS).
    // - `is_singleton_mode = 0` so `DAT_143c4ac08[1]` stays bound to real `GameThread`.
    let tp = exe::tls_pointer();
    if tp == 0 {
        return None;
    }
    let tls0 = rd::<u64>(tp) as usize;
    if tls0 == 0 {
        return None;
    }
    let desc = tls0 + 0xa86c0;
    wr::<u32>(tls0 + 0xa86b8, rd::<u32>(tls0 + 0xa86b8) | 1);
    core::ptr::write_bytes(desc as *mut u8, 0, 0x100);
    wr::<u32>(desc, 0x0c); // mode = 0xc
    wr::<u32>(desc + 0x08, 0xffff_ffff);
    wr::<u32>(desc + 0x0c, 4); // priority_idx
    wr::<u8>(desc + 0x10, 0); // is_singleton_mode = 0
    let name = b"ServerWorldStep\0";
    core::ptr::copy_nonoverlapping(name.as_ptr(), (desc + 0x11) as *mut u8, name.len());

    // Publish in `DAT_143c4aa00[wid]` + register TLS exit destructor.
    exe::call_va!(VA_REGISTER_THREAD_DESC, (desc as u64));

    // Register with `r::ThreadManager` as worker mode 4 so Chase-Lev steal state is active.
    exe::call_va!(VA_REGISTER_JOB_WORKER, (sched_mgr, 4u64));

    log::info!(
        "world_step: ServerWorldStep thread ready (wid={wid}, mode=0xc, tid=0x{:x}, desc={:#x})",
        win32_rt::teb::thread_id(),
        desc,
    );
    Some(wid)
}

/// Flush per-thread scratch pad & allocator reclaim buffers after a world step on
/// `ServerWorldStep`, matching `r::ThreadManager::Worker::vfunc_1` (`0x1424070e0`) and
/// `GameServer::vfunc_9` (`0x1418437a0`).
unsafe fn flush_server_world_thread_scratch(wid: u32) {
    let scratch = exe::call_va!(VA_SCRATCH_TLS, () -> u64);
    if scratch != 0 && rd::<u64>(scratch as usize + 0x10) != 0 {
        exe::call_va!(VA_SCRATCH_FLUSH, (scratch + 8));
    }
    let alloc_mgr = rd::<u64>(VA_ALLOC_MGR_PTR as usize);
    if alloc_mgr != 0 {
        let slot = rd::<u64>(alloc_mgr as usize + 0x2d8 + (wid as usize) * 8);
        if slot != 0 {
            exe::call_va!(VA_ALLOC_RECLAIM, (slot, 0xffff_ffffu64));
        }
    }
    // Mark this worker's render-command flush slot so `Renderer::submit_frame` collects any
    // render commands pushed directly from `ServerWorldStep`.
    wr::<u8>(VA_RENDER_CMD_DIRTY_BASE as usize + (wid as usize) * 0x158, 1);
}

fn server_world_worker_loop(w: &'static ServerWorldWorker) {
    let _ = w.worker_thread.set(std::thread::current());
    let Some(wid) = (unsafe { register_server_world_thread() }) else {
        w.ready.store(2, Ordering::Release);
        return;
    };
    w.worker_wid.store(wid, Ordering::Relaxed);
    IS_SERVER_WORLD_THREAD.with(|c| c.set(true));
    w.ready.store(1, Ordering::Release);

    loop {
        // Wait for ST_RUNNING (brief spin for low wake latency, then park).
        let t0 = Instant::now();
        let mut spins = 0u32;
        loop {
            if w.state.load(Ordering::Acquire) == ST_RUNNING {
                break;
            }
            spins = spins.wrapping_add(1);
            if spins & 63 != 0 || t0.elapsed() < WORKER_SPIN {
                core::hint::spin_loop();
                continue;
            }
            w.worker_parked.store(true, Ordering::SeqCst);
            if w.state.load(Ordering::SeqCst) != ST_RUNNING {
                std::thread::park();
            }
            w.worker_parked.store(false, Ordering::SeqCst);
        }

        let world_holder = w.req_world_holder.load(Ordering::Relaxed);
        let mode = w.req_mode.load(Ordering::Relaxed);

        unsafe {
            world_step_inline(world_holder, mode);
            flush_server_world_thread_scratch(wid);
        }

        // Publish completion and wait for GameThread to return state to ST_IDLE.
        w.state.store(ST_STEP_DONE, Ordering::Release);
        while w.state.load(Ordering::Acquire) == ST_STEP_DONE {
            core::hint::spin_loop();
        }
    }
}

fn ensure_server_world_worker() -> Option<&'static ServerWorldWorker> {
    *WORKER.get_or_init(|| {
        // Only spawn when Northlight's job system is live (never during standalone `--diff-test`).
        let sched_mgr = unsafe { rd::<u64>(VA_JOB_SYSTEM_PTR as usize) };
        if sched_mgr == 0 {
            return None;
        }
        let w: &'static ServerWorldWorker = Box::leak(Box::new(ServerWorldWorker {
            state: AtomicU32::new(ST_IDLE),
            ready: AtomicU32::new(0),
            worker_wid: AtomicU32::new(u32::MAX),
            worker_parked: AtomicBool::new(false),
            prio4_phase_active: AtomicBool::new(false),
            req_world_holder: AtomicU64::new(0),
            req_mode: AtomicU32::new(0),
            bar_fn: AtomicU64::new(0),
            bar_arg0: AtomicU64::new(0),
            bar_arg1: AtomicU64::new(0),
            bar_arg2: AtomicU64::new(0),
            worker_thread: OnceLock::new(),
        }));
        let body: Box<dyn FnOnce() -> u32 + Send + 'static> = Box::new(move || {
            server_world_worker_loop(w);
            0
        });
        match win32_rt::thread::spawn(body, 2 * 1024 * 1024, false, false) {
            Ok(tobj) => {
                if let Ok(mut nm) = tobj.name.lock() {
                    *nm = "ServerWorldStep".to_string();
                }
            }
            Err(e) => {
                log::error!("world_step: failed to spawn ServerWorldStep Win32 thread: {e}");
                return None;
            }
        }
        // Wait for registration to complete (bounded).
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            match w.ready.load(Ordering::Acquire) {
                1 => return Some(w),
                2 => return None,
                _ => {
                    if Instant::now() > deadline {
                        log::error!("world_step: timed out waiting for ServerWorldStep init");
                        return None;
                    }
                    std::thread::yield_now();
                }
            }
        }
    })
}

/// Request `GameThread` to execute a main-thread-affine serial barrier system synchronously
/// while `ServerWorldStep` waits.
#[inline]
unsafe fn run_barrier_on_game_thread(
    w: &'static ServerWorldWorker,
    fn_ptr: u64,
    arg0: u64,
    arg1: u64,
    arg2: u64,
) {
    w.bar_fn.store(fn_ptr, Ordering::Relaxed);
    w.bar_arg0.store(arg0, Ordering::Relaxed);
    w.bar_arg1.store(arg1, Ordering::Relaxed);
    w.bar_arg2.store(arg2, Ordering::Relaxed);
    w.state.store(ST_NEED_MAIN_BARRIER, Ordering::Release);

    while w.state.load(Ordering::Acquire) != ST_MAIN_BARRIER_DONE {
        core::hint::spin_loop();
    }
    w.state.store(ST_RUNNING, Ordering::Release);
}

/// Check if the calling thread is currently registered as `GameThread` (`mode == 1`).
#[inline]
unsafe fn is_game_thread() -> bool {
    let wid = crate::ns::r::thread::worker_index() as usize;
    if wid >= 64 {
        return false;
    }
    let desc = rd::<usize>(VA_WORKER_DESC_TABLE as usize + wid * 8);
    desc != 0 && rd::<u32>(desc) == 1
}

/// Offload `world_step(world_holder, mode)` onto `ServerWorldStep` while `GameThread`
/// cooperatively services main-thread barriers and pumps priority-4 UI + priority-2 ECS jobs.
unsafe fn world_step_offloaded(
    w: &'static ServerWorldWorker,
    world_holder: u64,
    mode: u32,
) {
    let sched_mgr = rd::<usize>(VA_JOB_SYSTEM_PTR as usize);

    w.req_world_holder.store(world_holder, Ordering::Relaxed);
    w.req_mode.store(mode, Ordering::Relaxed);
    w.state.store(ST_RUNNING, Ordering::SeqCst);
    if w.worker_parked.load(Ordering::SeqCst) {
        if let Some(t) = w.worker_thread.get() {
            t.unpark();
        }
    }

    let mut pumped = 0u64;
    loop {
        match w.state.load(Ordering::Acquire) {
            ST_STEP_DONE => {
                w.state.store(ST_IDLE, Ordering::Release);
                break;
            }
            ST_NEED_MAIN_BARRIER => {
                let fn_ptr = w.bar_fn.load(Ordering::Relaxed);
                let a0 = w.bar_arg0.load(Ordering::Relaxed);
                let a1 = w.bar_arg1.load(Ordering::Relaxed);
                let a2 = w.bar_arg2.load(Ordering::Relaxed);
                exe::call_va!(fn_ptr, (a0, a1, a2));
                TOTAL_MAIN_BARRIERS.fetch_add(1, Ordering::Relaxed);
                w.state.store(ST_MAIN_BARRIER_DONE, Ordering::Release);
                while w.state.load(Ordering::Acquire) == ST_MAIN_BARRIER_DONE {
                    core::hint::spin_loop();
                }
            }
            _ => {
                if sched_mgr != 0 {
                    let job = if w.prio4_phase_active.load(Ordering::Relaxed) {
                        // Reserve GameThread for instant priority-4 UI dispatch during the UI
                        // DAG phase so `DAT_143827b24` never waits behind a stolen prio-2 job.
                        let inner_sched = rd::<usize>(sched_mgr + 0x10);
                        let j = crate::ns::r::thread::scheduler_pop_job(inner_sched, 4u16, 1u8);
                        if j != 0 {
                            crate::ns::r::thread::scheduler_run_job(inner_sched, j);
                        }
                        j
                    } else {
                        crate::ns::r::thread::scheduler_pump_one(sched_mgr)
                    };
                    if job != 0 {
                        pumped += 1;
                        continue;
                    }
                }
                core::hint::spin_loop();
            }
        }
    }

    let steps = TOTAL_OFFLOADED_STEPS.fetch_add(1, Ordering::Relaxed) + 1;
    let total_pumped = TOTAL_GAME_PUMPED_JOBS.fetch_add(pumped, Ordering::Relaxed) + pumped;
    if steps == 1 || steps == 100 || steps % 1000 == 0 {
        log::info!(
            "world_step (threaded): steps={steps} game_pumped_jobs={total_pumped} main_barriers={} server_barriers={} wid={}",
            TOTAL_MAIN_BARRIERS.load(Ordering::Relaxed),
            TOTAL_SERVER_BARRIERS.load(Ordering::Relaxed),
            w.worker_wid.load(Ordering::Relaxed),
        );
    }
}

// ---------------------------------------------------------------------------
// 1. 0x141f818b0 — world_step
// ---------------------------------------------------------------------------

/// Inline implementation of `sub_141f818b0` (exact match to disassembly).
#[inline]
unsafe fn world_step_inline(world_holder: u64, mode: u32) {
    let pre_fn = rd::<u64>(world_holder as usize + 0x90);
    if pre_fn != 0 {
        let vtbl = rd::<u64>(pre_fn as usize);
        let call_op = rd::<u64>(vtbl as usize + 0x10);
        let ecs_storage = rd::<u64>(world_holder as usize + 0x08);
        exe::call_va!(call_op, (pre_fn, ecs_storage));
    }

    let sel_fn = rd::<u64>(world_holder as usize + 0x50);
    let mut mode_slot: u32 = mode;
    if sel_fn == 0 {
        exe::call_iat!(IAT_BAD_FUNCTION_CALL, ());
        return;
    }

    let vtbl = rd::<u64>(sel_fn as usize);
    let call_op = rd::<u64>(vtbl as usize + 0x10);
    let mut gameloop_id: u8 = 0;
    let ecs_storage = rd::<u64>(world_holder as usize + 0x08);
    exe::call_va!(
        call_op,
        (
            sel_fn,
            (&mut gameloop_id as *mut u8) as u64,
            ecs_storage,
            (&mut mode_slot as *mut u32) as u64
        )
    );

    let ecs_storage_run = rd::<u64>(world_holder as usize + 0x08);
    wr::<u8>(ecs_storage_run as usize + 0x5e318, 1);
    let runner = world_holder.wrapping_add(0x98);
    if crate::emu::resolve_dispatch(0x1_420a_c1c0u64).0 == (ecs_runner_run as *const () as usize) {
        ecs_runner_run(runner, ecs_storage_run, gameloop_id as u64);
    } else {
        exe::call_va!(0x1_420a_c1c0u64, (runner, ecs_storage_run, gameloop_id as u64));
    }
    let ecs_storage_done = rd::<u64>(world_holder as usize + 0x08);
    wr::<u8>(ecs_storage_done as usize + 0x5e318, 0);
}

/// `sub_141f818b0` — original @ `0x141f818b0` (142 bytes).
/// Runs the world's pre-step callback (`[rcx + 0x90]`), selects the active gameloop via
/// `GameLoopSelector` (`[rcx + 0x50]`), sets `ecs_storage[+0x5e318] = 1`, runs
/// `ecs_runner_run` (`0x1420ac1c0`), and clears `ecs_storage[+0x5e318] = 0`.
pub unsafe extern "win64" fn world_step(world_holder: u64, mode: u32) {
    if threaded_mode() != 0 && mode == 0 && !IS_SERVER_WORLD_THREAD.with(|c| c.get()) {
        let main_w = rd::<u64>(VA_MAIN_WORLD_PTR as usize);
        let tl_w = rd::<u64>(VA_TIMELINE_WORLD_PTR as usize);
        if (world_holder == main_w || world_holder == tl_w) && is_game_thread() {
            if let Some(w) = ensure_server_world_worker() {
                world_step_offloaded(w, world_holder, mode);
                return;
            }
        }
    }
    world_step_inline(world_holder, mode);
}

// ---------------------------------------------------------------------------
// 2. 0x1420ac1c0 — ecs_runner_run
// ---------------------------------------------------------------------------

/// Returns `true` if `phase`'s system ID list (`[phase]` of length `cnt`) contains `target_sys`.
#[inline]
unsafe fn phase_contains_sys(phase: u64, cnt: u32, target_sys: u16) -> bool {
    if cnt == 0 || target_sys == 0 || target_sys == u16::MAX {
        return false;
    }
    let base = rd::<u64>(phase as usize) as usize;
    if base == 0 {
        return false;
    }
    let mut i = 0usize;
    let n = cnt as usize;
    while i < n {
        if rd::<u16>(base + i * 2) == target_sys {
            return true;
        }
        i += 1;
    }
    false
}

/// `sub_1420ac1c0` — original @ `0x1420ac1c0` (582 bytes).
/// Binds `(ecs_storage, gameloop_id)` on `runner` if changed, then iterates the gameloop's
/// phase table (`stride 0x40`): serial barrier phases (`count <= 1 && (f110 || f111)`) run
/// with `ecs_storage[+0x5e319] = 1`, while parallel DAG phases invoke `start_scheduling`
/// (`0x1420a8fd0`) and snapshot `ecs_storage[+0x48]` into `runner[+0x30]`.
pub unsafe extern "win64" fn ecs_runner_run(
    runner: u64,
    ecs_storage: u64,
    gameloop_id: u64,
) {
    let gid = gameloop_id as u8;
    let prev_storage = rd::<u64>(runner as usize + 0x20);
    let prev_gid = rd::<u8>(runner as usize + 0x28);
    if ecs_storage != prev_storage || gid != prev_gid {
        let g0 = rd::<u64>(runner as usize);
        let g1 = rd::<u64>(runner as usize + 0x08);
        exe::call_va!(VA_RUNNER_BIND, (ecs_storage, g0, g1, gid as u64));
        let old_storage = rd::<u64>(runner as usize + 0x20);
        if old_storage != 0 {
            let old_gid = rd::<u8>(runner as usize + 0x28);
            exe::call_va!(
                VA_RUNNER_UNBIND,
                (old_storage, rd::<u64>(runner as usize), rd::<u64>(runner as usize + 0x08), old_gid as u64)
            );
        }
        wr::<u64>(runner as usize + 0x20, ecs_storage);
        wr::<u8>(runner as usize + 0x28, gid);
    }

    let ctx_pair = [ecs_storage, runner];
    let loops_root = rd::<u64>(rd::<u64>(runner as usize) as usize);
    let loop_off = (gid as usize) * 0x28;
    let phases_begin = rd::<u64>(loops_root as usize + loop_off + 0x18);
    let phases_cnt = rd::<u32>(loops_root as usize + loop_off + 0x20) as u64;
    let phases_end = phases_begin.wrapping_add(phases_cnt << 6);

    let mt_enabled = rd::<u8>(VA_ECS_MT_ENABLED as usize) != 0;
    let on_server_thr = IS_SERVER_WORLD_THREAD.with(|c| c.get());
    let trace_this = if trace_limit() > 0 && phases_cnt >= 10 {
        let prev = TRACE_LOGGED_STEPS.fetch_add(1, Ordering::Relaxed);
        prev < trace_limit()
    } else {
        false
    };

    if mt_enabled {
        let mut phase = phases_begin;
        let mut phase_idx = 0u32;
        while phase != phases_end {
            let cnt = rd::<u32>(phase as usize + 0x08);
            let mut is_barrier = false;
            let mut f110 = 0u8;
            let mut f111 = 0u8;
            if cnt <= 1 {
                let sys_id = rd::<u16>(rd::<u64>(phase as usize) as usize);
                let sys_box = rd::<u64>(runner as usize + 0x08);
                let sys_table = rd::<u64>(sys_box as usize + 0x08);
                let sys_desc = sys_table.wrapping_add((sys_id as u64).wrapping_mul(0x130));
                f111 = rd::<u8>(sys_desc as usize + 0x111);
                f110 = rd::<u8>(sys_desc as usize + 0x110);
                is_barrier = f111 != 0 || f110 != 0;
            }

            if is_barrier {
                wr::<u8>(ecs_storage as usize + 0x5e319, 1);
                let sid = rd::<u16>(rd::<u64>(phase as usize) as usize);
                let sys_box = rd::<u64>(runner as usize + 0x08);
                let sys_table = rd::<u64>(sys_box as usize + 0x08);
                let sys_desc = sys_table.wrapping_add((sid as u64).wrapping_mul(0x130));
                let fn_ptr = rd::<u64>(sys_desc as usize + 0x120);
                let arg_r8 = rd::<u16>(sys_desc as usize) as u64;
                let t0 = if trace_this { Some(Instant::now()) } else { None };
                let jp = crate::jobprof::begin();

                let route_main = on_server_thr
                    && match barrier_policy() {
                        "all" => true,
                        "110" => f110 != 0,
                        _ => false,
                    };
                if route_main {
                    if let Some(Some(w)) = WORKER.get() {
                        run_barrier_on_game_thread(
                            w,
                            fn_ptr,
                            ctx_pair.as_ptr() as u64,
                            runner.wrapping_add(0x10),
                            arg_r8,
                        );
                    } else {
                        exe::call_va!(fn_ptr, (ctx_pair.as_ptr() as u64, runner.wrapping_add(0x10), arg_r8));
                    }
                } else {
                    if on_server_thr {
                        TOTAL_SERVER_BARRIERS.fetch_add(1, Ordering::Relaxed);
                    }
                    exe::call_va!(fn_ptr, (ctx_pair.as_ptr() as u64, runner.wrapping_add(0x10), arg_r8));
                }

                crate::jobprof::end(jp, crate::jobprof::K_SYSTEM, fn_ptr, sid, 0xffff);
                wr::<u8>(ecs_storage as usize + 0x5e319, 0);
                if let Some(t0) = t0 {
                    log::info!(
                        "world_step[gid={gid} p={phase_idx:02}] BARRIER sys={sid} fn={fn_ptr:#x} f110={f110} f111={f111} route_main={route_main} dur={:.1}us",
                        t0.elapsed().as_secs_f64() * 1e6,
                    );
                }
            } else {
                let t0 = if trace_this { Some(Instant::now()) } else { None };
                let marked_prio4 = if on_server_thr {
                    if let Some(Some(w)) = WORKER.get() {
                        let special = rd::<u16>(VA_PRIO4_UI_SYS_ID as usize);
                        let has = phase_contains_sys(phase, cnt, special);
                        if has {
                            w.prio4_phase_active.store(true, Ordering::Release);
                        }
                        has
                    } else {
                        false
                    }
                } else {
                    false
                };
                exe::call_native!(
                    VA_START_SCHEDULING,
                    crate::ns::r::scheduler::start_scheduling,
                    (0u64, runner, ctx_pair.as_ptr() as u64, phase) -> u64
                );
                if marked_prio4 {
                    if let Some(Some(w)) = WORKER.get() {
                        w.prio4_phase_active.store(false, Ordering::Release);
                    }
                }
                let grp_cnt = rd::<u64>(ecs_storage as usize + 0x48);
                wr::<u64>(runner as usize + 0x30, grp_cnt);
                if let Some(t0) = t0 {
                    log::info!(
                        "world_step[gid={gid} p={phase_idx:02}] DAG     cnt={cnt:3} phase={phase:#x} dur={:.1}us",
                        t0.elapsed().as_secs_f64() * 1e6,
                    );
                }
            }

            phase = phase.wrapping_add(0x40);
            phase_idx += 1;
        }
    } else {
        // Single-threaded fallback (`DAT_143861638 == 0`), exact match to `0x1420ac2f4..0x1420ac3f2`.
        let mut phase = phases_begin;
        while phase != phases_end {
            let cnt = rd::<u32>(phase as usize + 0x08);
            let is_barrier = if cnt <= 1 {
                let sid = rd::<u16>(rd::<u64>(phase as usize) as usize);
                let sys_box = rd::<u64>(runner as usize + 0x08);
                let sys_table = rd::<u64>(sys_box as usize + 0x08);
                let sys_desc = sys_table.wrapping_add((sid as u64).wrapping_mul(0x130));
                rd::<u8>(sys_desc as usize + 0x111) != 0 || rd::<u8>(sys_desc as usize + 0x110) != 0
            } else {
                false
            };
            wr::<u8>(ecs_storage as usize + 0x5e319, u8::from(is_barrier));
            let mut p = rd::<u64>(phase as usize);
            let p_end = p.wrapping_add((cnt as u64) * 2);
            while p != p_end {
                let sid = rd::<u16>(p as usize) as u64;
                let sys_box = rd::<u64>(runner as usize + 0x08);
                let sys_table = rd::<u64>(sys_box as usize + 0x08);
                let sys_desc = sys_table.wrapping_add(sid.wrapping_mul(0x130));
                let fn_ptr = rd::<u64>(sys_desc as usize + 0x120);
                exe::call_va!(fn_ptr, (ctx_pair.as_ptr() as u64, runner.wrapping_add(0x10), sid));
                p = p.wrapping_add(2);
            }
            wr::<u8>(ecs_storage as usize + 0x5e319, 0);
            if !is_barrier {
                wr::<u64>(runner as usize + 0x30, rd::<u64>(ecs_storage as usize + 0x48));
            }
            phase = phase.wrapping_add(0x40);
        }
    }
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

struct ByteGlobalGuard {
    va: u64,
    prev: u8,
}

impl ByteGlobalGuard {
    unsafe fn set(va: u64, val: u8) -> Self {
        let prev = rd::<u8>(va as usize);
        wr::<u8>(va as usize, val);
        Self { va, prev }
    }
}

impl Drop for ByteGlobalGuard {
    fn drop(&mut self) {
        unsafe { wr::<u8>(self.va as usize, self.prev) };
    }
}

pub unsafe fn diff_world_step(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u32);
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));

    static mut LOG: Vec<(u64, u64, u64)> = Vec::new();
    unsafe extern "win64" fn mock_pre(_self: u64, storage: u64) {
        (*core::ptr::addr_of_mut!(LOG)).push((1, storage, 0));
    }
    unsafe extern "win64" fn mock_sel(_self: u64, out_gid: u64, storage: u64, mode_ptr: u64) {
        let m = rd::<u32>(mode_ptr as usize);
        wr::<u8>(out_gid as usize, (m & 0xff) as u8);
        (*core::ptr::addr_of_mut!(LOG)).push((2, storage, m as u64));
    }
    unsafe extern "win64" fn mock_runner(runner: u64, storage: u64, gid: u64) {
        let in_step = rd::<u8>(storage as usize + 0x5e318) as u64;
        (*core::ptr::addr_of_mut!(LOG)).push((3, runner, (gid << 8) | in_step));
    }

    // Install a mock for `ecs_runner_run` (`0x1420ac1c0`) so we test `world_step` in isolation.
    let _mr = crate::difftest::Mock::new(0x1_420a_c1c0, mock_runner as *const ());

    // Provide callable guest addresses for the virtual methods by registering two unused VAs.
    // Note: in difftest, vtable entries pointing directly to host fns work via `register_native_va`.
    let pre_fn_addr = mock_pre as *const () as u64;
    let sel_fn_addr = mock_sel as *const () as u64;
    exe::register_native_va(pre_fn_addr, pre_fn_addr as usize);
    exe::register_native_va(sel_fn_addr, sel_fn_addr as usize);

    let mut pre_vtbl = [0u64; 4];
    pre_vtbl[2] = pre_fn_addr;
    let pre_obj = [pre_vtbl.as_ptr() as u64];

    let mut sel_vtbl = [0u64; 4];
    sel_vtbl[2] = sel_fn_addr;
    let sel_obj = [sel_vtbl.as_ptr() as u64];

    let mut storage = vec![0u8; 0x5e340];
    let storage_ptr = storage.as_mut_ptr() as u64;

    let run = |has_pre: bool, mode: u32, f: F| -> (Vec<(u64, u64, u64)>, u8) {
        (*core::ptr::addr_of_mut!(LOG)).clear();
        wr::<u8>(storage_ptr as usize + 0x5e318, 0);
        let mut holder = vec![0u8; 0x100];
        let h_ptr = holder.as_mut_ptr() as u64;
        wr::<u64>(h_ptr as usize + 0x08, storage_ptr);
        wr::<u64>(h_ptr as usize + 0x50, sel_obj.as_ptr() as u64);
        wr::<u64>(
            h_ptr as usize + 0x90,
            if has_pre { pre_obj.as_ptr() as u64 } else { 0 },
        );
        f(h_ptr, mode);
        let log_out = (*core::ptr::addr_of!(LOG)).clone();
        // Normalize runner pointer (stack holder + 0x98) to a relative offset check.
        let norm = log_out
            .into_iter()
            .map(|(tag, a, b)| (tag, if tag == 3 { a.wrapping_sub(h_ptr) } else { a }, b))
            .collect();
        (norm, rd::<u8>(storage_ptr as usize + 0x5e318))
    };

    let mut cases = 0u32;
    for &has_pre in &[false, true] {
        for &mode in &[0u32, 1, 2] {
            let o = run(has_pre, mode, orig_fn);
            let n = run(has_pre, mode, new_fn);
            crate::difftest::eq(&format!("world_step pre={has_pre} mode={mode}"), o, n)?;
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_ecs_runner_run(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64);
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));

    static mut TRACE: Vec<(u64, u64, u64, u64)> = Vec::new();
    unsafe extern "win64" fn mock_bind(st: u64, g0: u64, g1: u64, gid: u64) {
        (*core::ptr::addr_of_mut!(TRACE)).push((1, st, g0 ^ g1, gid));
    }
    unsafe extern "win64" fn mock_unbind(st: u64, g0: u64, g1: u64, gid: u64) {
        (*core::ptr::addr_of_mut!(TRACE)).push((2, st, g0 ^ g1, gid));
    }
    unsafe extern "win64" fn mock_start_sched(_rcx: u64, runner: u64, ctx_pair: u64, phase: u64) -> u64 {
        let st = rd::<u64>(ctx_pair as usize);
        let in_ser = rd::<u8>(st as usize + 0x5e319) as u64;
        (*core::ptr::addr_of_mut!(TRACE)).push((3, runner, phase, in_ser));
        0
    }
    unsafe extern "win64" fn mock_sys_fn(ctx_pair: u64, runner_10: u64, sid: u64) {
        let st = rd::<u64>(ctx_pair as usize);
        let in_ser = rd::<u8>(st as usize + 0x5e319) as u64;
        (*core::ptr::addr_of_mut!(TRACE)).push((4, runner_10, sid, in_ser));
    }

    let _mb = crate::difftest::Mock::new(VA_RUNNER_BIND, mock_bind as *const ());
    let _mu = crate::difftest::Mock::new(VA_RUNNER_UNBIND, mock_unbind as *const ());
    let _ms = crate::difftest::Mock::new(VA_START_SCHEDULING, mock_start_sched as *const ());

    let sys_fn_addr = mock_sys_fn as *const () as u64;
    exe::register_native_va(sys_fn_addr, sys_fn_addr as usize);

    // Build 3 system descriptors (stride 0x130):
    // sys 0: normal parallel (f110=0, f111=0)
    // sys 1: barrier f110=1
    // sys 2: barrier f111=1
    let mut sys_descs = vec![0u8; 3 * 0x130];
    let sd_ptr = sys_descs.as_mut_ptr() as u64;
    for i in 0..3u64 {
        let d = (sd_ptr + i * 0x130) as usize;
        wr::<u16>(d, i as u16);
        wr::<u64>(d + 0x120, sys_fn_addr);
    }
    wr::<u8>((sd_ptr + 0x130 + 0x110) as usize, 1);
    wr::<u8>((sd_ptr + 2 * 0x130 + 0x111) as usize, 1);
    let sys_box = [0u64, sd_ptr];

    // Build 3 phases (stride 0x40):
    // phase 0: [sys 0] (cnt=1, non-barrier -> DAG in MT mode)
    // phase 1: [sys 1] (cnt=1, barrier f110=1)
    // phase 2: [sys 0, sys 2] (cnt=2 -> DAG in MT mode)
    let p0_ids: [u16; 1] = [0];
    let p1_ids: [u16; 1] = [1];
    let p2_ids: [u16; 2] = [0, 2];
    let mut phases = vec![0u8; 3 * 0x40];
    let ph_ptr = phases.as_mut_ptr() as u64;
    wr::<u64>(ph_ptr as usize, p0_ids.as_ptr() as u64);
    wr::<u32>(ph_ptr as usize + 0x08, 1);
    wr::<u64>(ph_ptr as usize + 0x40, p1_ids.as_ptr() as u64);
    wr::<u32>(ph_ptr as usize + 0x48, 1);
    wr::<u64>(ph_ptr as usize + 0x80, p2_ids.as_ptr() as u64);
    wr::<u32>(ph_ptr as usize + 0x88, 2);

    // Gameloop table (gid=1 at offset 0x28):
    let mut loops_root = vec![0u8; 0x80];
    let lr_ptr = loops_root.as_mut_ptr() as u64;
    wr::<u64>(lr_ptr as usize + 0x28 + 0x18, ph_ptr);
    wr::<u32>(lr_ptr as usize + 0x28 + 0x20, 3);
    let loops_box = [lr_ptr];

    let mut storage = vec![0u8; 0x5e340];
    let st_ptr = storage.as_mut_ptr() as u64;
    wr::<u64>(st_ptr as usize + 0x48, 0x1234);

    let mut cases = 0u32;
    for &mt in &[0u8, 1u8] {
        let _g = ByteGlobalGuard::set(VA_ECS_MT_ENABLED, mt);
        let run = |f: F| -> (Vec<(u64, u64, u64, u64)>, u64, u8, u64, u8) {
            (*core::ptr::addr_of_mut!(TRACE)).clear();
            wr::<u8>(st_ptr as usize + 0x5e319, 0);
            let mut runner = [0u64; 8];
            runner[0] = loops_box.as_ptr() as u64;
            runner[1] = sys_box.as_ptr() as u64;
            runner[4] = 0; // prev_storage
            runner[5] = 0; // prev_gid
            runner[6] = 0; // saved grp_cnt
            let r_ptr = runner.as_mut_ptr() as u64;
            f(r_ptr, st_ptr, 1);
            let tr = (*core::ptr::addr_of!(TRACE))
                .iter()
                .map(|&(tag, a, b, c)| {
                    let na = if tag == 3 || tag == 4 { a.wrapping_sub(r_ptr) } else { a };
                    (tag, na, b, c)
                })
                .collect();
            (tr, runner[4], runner[5] as u8, runner[6], rd::<u8>(st_ptr as usize + 0x5e319))
        };
        let o = run(orig_fn);
        let n = run(new_fn);
        crate::difftest::eq(&format!("ecs_runner_run mt={mt}"), o, n)?;
        cases += 1;
    }
    Ok(cases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_contains_sys_matches_exact_id() {
        let ids: [u16; 4] = [10, 42, 777, 1024];
        let mut phase = [0u64; 8];
        phase[0] = ids.as_ptr() as u64;
        phase[1] = ids.len() as u64;
        let p = phase.as_ptr() as u64;
        unsafe {
            assert!(phase_contains_sys(p, 4, 777));
            assert!(phase_contains_sys(p, 4, 10));
            assert!(phase_contains_sys(p, 4, 1024));
            assert!(!phase_contains_sys(p, 4, 999));
            assert!(!phase_contains_sys(p, 0, 777));
            assert!(!phase_contains_sys(p, 4, 0));
            assert!(!phase_contains_sys(p, 4, u16::MAX));
        }
    }
}
