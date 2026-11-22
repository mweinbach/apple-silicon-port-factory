//! Job/ECS-system timeline profiler (`AW2_JOB_PROF=<secs>[,<dur_secs>]`, diagnostic only).
//!
//! Records one event per ECS system call, scheduler job and `start_scheduling` graph run
//! for a window starting `<secs>` seconds after the first recorded call (default length
//! 2 s), then writes the raw timeline to `out/hitch/jobprof-<pid>.bin` for offline
//! critical-path analysis (`tools/jobprof.py`, `tools/jobprof_sys.py`).
//!
//! Off by default: when the variable is unset every hook costs one relaxed load and a
//! predictable branch.

use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::OnceLock;

/// A single-system call from `schedule_node_complete` (`sys_desc + 0x120`).
pub const K_SYSTEM: u16 = 0;
/// A per-group system call from `schedule_group_node_run` (`sys_desc + 0x128`).
pub const K_GROUP: u16 = 1;
/// A whole `start_scheduling` graph run (main thread).
pub const K_GRAPH: u16 = 2;
/// Any job run through `scheduler_run_job` (`id` = vtable run function VA).
pub const K_JOB: u16 = 3;
/// A `scheduler_wait_handle_*` wait (`id` = handle address, `sys` = 1 for the `_any` variant).
pub const K_WAITH: u16 = 4;
/// A Win32-side span reported through `win32_rt::hitch::set_span_hook`
/// (`id` = `hitch::SPAN_*`: 1 condition-variable wait, 2 Present, 3 object wait).
pub const K_EXT: u16 = 5;
/// A marked function call (`id` = original function VA).
pub const K_MARK: u16 = 6;
/// `propagate_render_pass_masks` saw render pass `sys` ready to execute (its context
/// counter at 0): span from the call's start to that moment (`id` = the pass's execute
/// callback VA).
pub const K_PASS_READY: u16 = 7;
/// `propagate_render_pass_masks` executing render pass `sys` (execute callback + mask
/// merge; `id` = the execute callback VA).
pub const K_PASS_EXEC: u16 = 8;
/// One whole `propagate_render_pass_masks` call (`sys` = poll rounds, capped; `id` = total
/// ns spent in the per-round poll callbacks).
pub const K_PROPAGATE: u16 = 9;
/// One Metal commit as executed by the GPU (`t0`/`t1` = GPU start/end, `id` = commit time on
/// the CS thread, `sys` = D3D12 queue type, `gid` = command buffers in the commit).
pub const K_GPU: u16 = 10;
/// A D3D12 queue operation (instant, `t0 == t1`): `sys` = `op << 8 | queue type` with `op` one
/// of `win32_rt::hitch::QOP_*`; `id` = fence value (or list count for ExecuteCommandLists);
/// `gid` = fence pointer bits 4..20.
pub const K_QOP: u16 = 11;
/// `propagate_render_pass_masks` running render pass `sys`'s exec callback (`entry + 0x50`):
/// `id` = the GfxGraph command's execute VA, or the callback VA for other passes.
pub const K_PASS_CB1: u16 = 12;

#[repr(C)]
#[derive(Clone, Copy)]
struct Event {
    t0: u64,
    t1: u64,
    id: u64,
    sys: u16,
    gid: u16,
    thr: u16,
    kind: u16,
}

const CAP: usize = 1 << 21;

struct State {
    /// `win32_rt::hitch::now_ns()` at init (shared clock with the Win32 span hook).
    epoch: u64,
    start_ns: u64,
    end_ns: u64,
    buf: Box<[core::cell::UnsafeCell<Event>]>,
    len: AtomicUsize,
    dumped: AtomicU32,
}

unsafe impl Sync for State {}

/// 0 = not initialized, 1 = off, 2 = on.
static MODE: AtomicU32 = AtomicU32::new(0);
static STATE: OnceLock<Option<State>> = OnceLock::new();
static NEXT_THR: AtomicU16 = AtomicU16::new(1);
static TOTAL: AtomicU64 = AtomicU64::new(0);
/// `AW2_JOB_PROF_KINDS=<hex>`: bit `1 << K_*` per event kind to record (default all).
static KINDS: AtomicU32 = AtomicU32::new(u32::MAX);

thread_local! {
    static THR: u16 = NEXT_THR.fetch_add(1, Ordering::Relaxed);
}

#[cold]
fn init() -> bool {
    let st = STATE.get_or_init(|| {
        let v = std::env::var("AW2_JOB_PROF").ok()?;
        let mut it = v.split(',').map(|s| s.trim().parse::<f64>().ok());
        let start = it.next().flatten()?;
        let dur = it.next().flatten().unwrap_or(2.0);
        let buf = (0..CAP)
            .map(|_| core::cell::UnsafeCell::new(Event { t0: 0, t1: 0, id: 0, sys: 0, gid: 0, thr: 0, kind: 0 }))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        log::warn!("jobprof: recording {dur:.1}s window starting {start:.1}s after first job");
        Some(State {
            epoch: win32_rt::hitch::now_ns(),
            start_ns: (start * 1e9) as u64,
            end_ns: ((start + dur) * 1e9) as u64,
            buf,
            len: AtomicUsize::new(0),
            dumped: AtomicU32::new(0),
        })
    });
    let on = st.is_some();
    if on {
        let kinds = std::env::var("AW2_JOB_PROF_KINDS")
            .ok()
            .and_then(|v| u32::from_str_radix(v.trim().trim_start_matches("0x"), 16).ok())
            .unwrap_or(u32::MAX);
        KINDS.store(kinds, Ordering::Relaxed);
        if kinds & (1 << K_EXT) != 0 {
            win32_rt::hitch::set_span_hook(ext_span);
        }
        if kinds & (1 << K_QOP) != 0 {
            win32_rt::hitch::set_queue_hook(queue_op);
        }
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if kinds & (1 << K_GPU) != 0 {
            win32_rt::hitch::set_gpu_hook(gpu_commit);
        }
    }
    MODE.store(if on { 2 } else { 1 }, Ordering::Relaxed);
    on
}

/// Win32 span hook target (absolute `hitch::now_ns` timestamps).
fn ext_span(id: u64, t0: u64, t1: u64) {
    if let Some(Some(s)) = STATE.get() {
        record_span(s, t0.saturating_sub(s.epoch), t1.saturating_sub(s.epoch), K_EXT, id, 0, 0);
    }
}

/// D3D12 queue-operation hook target.
fn queue_op(op: u32, kind: u32, a: u64, b: u64) {
    if let Some(Some(s)) = STATE.get() {
        let t = win32_rt::hitch::now_ns().saturating_sub(s.epoch);
        let (id, gid) = if op == win32_rt::hitch::QOP_ECL_CALL { (a, 0) } else { (b, (a >> 4) as u16) };
        record_span(s, t, t, K_QOP, id, ((op << 8) | kind) as u16, gid);
    }
}

/// Metal commit-feedback hook target (absolute host-clock nanoseconds).
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn gpu_commit(kind: u32, ncb: u32, commit: u64, start: u64, end: u64) {
    if let Some(Some(s)) = STATE.get() {
        let e = s.epoch;
        record_span(s, start.saturating_sub(e), end.saturating_sub(e), K_GPU, commit.saturating_sub(e), kind as u16, ncb as u16);
    }
}

/// `AW2_JOB_MARK=<hex va>[,<hex va>...]`: guest call targets the call bridges record as
/// [`K_MARK`] spans (`sys` = 0xffff). Consulted only when a dispatch-cache slot is filled.
pub fn marked(va: u64) -> bool {
    static SET: OnceLock<Vec<u64>> = OnceLock::new();
    SET.get_or_init(|| {
        std::env::var("AW2_JOB_MARK")
            .map(|v| {
                v.split(',')
                    .filter_map(|s| u64::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok())
                    .collect()
            })
            .unwrap_or_default()
    })
    .contains(&va)
}

/// Start timestamp for an event, or `None` when profiling is off.
#[inline(always)]
pub fn begin() -> Option<u64> {
    match MODE.load(Ordering::Relaxed) {
        1 => None,
        2 => Some(now()),
        _ => init().then(now),
    }
}

#[inline]
fn now() -> u64 {
    match STATE.get() {
        Some(Some(s)) => win32_rt::hitch::now_ns().saturating_sub(s.epoch),
        _ => 0,
    }
}

/// Finish an event started with [`begin`].
#[inline(always)]
pub fn end(t0: Option<u64>, kind: u16, id: u64, sys: u16, gid: u16) {
    if let Some(t0) = t0 {
        record(t0, kind, id, sys, gid);
    }
}

#[cold]
#[inline(never)]
fn record(t0: u64, kind: u16, id: u64, sys: u16, gid: u16) {
    if KINDS.load(Ordering::Relaxed) & (1 << kind) == 0 {
        return;
    }
    let Some(Some(s)) = STATE.get() else { return };
    let t1 = win32_rt::hitch::now_ns().saturating_sub(s.epoch);
    record_span(s, t0, t1, kind, id, sys, gid);
}

fn record_span(s: &State, t0: u64, t1: u64, kind: u16, id: u64, sys: u16, gid: u16) {
    if t1 < s.start_ns {
        return;
    }
    if t0 > s.end_ns {
        dump(s);
        return;
    }
    let i = s.len.fetch_add(1, Ordering::Relaxed);
    if i >= CAP {
        dump(s);
        return;
    }
    let thr = THR.with(|t| *t);
    unsafe { *s.buf[i].get() = Event { t0, t1, id, sys, gid, thr, kind } };
    TOTAL.fetch_add(1, Ordering::Relaxed);
}

#[cold]
fn dump(s: &State) {
    if s.dumped.swap(1, Ordering::AcqRel) != 0 {
        return;
    }
    // Let in-flight writers finish their slot.
    std::thread::sleep(std::time::Duration::from_millis(50));
    let n = s.len.load(Ordering::Acquire).min(CAP);
    let mut out = Vec::with_capacity(n * core::mem::size_of::<Event>());
    for e in &s.buf[..n] {
        let e = unsafe { *e.get() };
        out.extend_from_slice(&e.t0.to_le_bytes());
        out.extend_from_slice(&e.t1.to_le_bytes());
        out.extend_from_slice(&e.id.to_le_bytes());
        out.extend_from_slice(&e.sys.to_le_bytes());
        out.extend_from_slice(&e.gid.to_le_bytes());
        out.extend_from_slice(&e.thr.to_le_bytes());
        out.extend_from_slice(&e.kind.to_le_bytes());
    }
    let _ = std::fs::create_dir_all("out/hitch");
    let path = format!("out/hitch/jobprof-{}.bin", std::process::id());
    match std::fs::write(&path, &out) {
        Ok(()) => log::warn!("jobprof: wrote {n} events to {path}"),
        Err(e) => log::warn!("jobprof: write {path} failed: {e}"),
    }
}
