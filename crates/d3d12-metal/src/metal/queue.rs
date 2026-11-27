//! ID3D12CommandQueue on `MTL4CommandQueue`.
//!
//! Every queue has an internal timeline (`MTLSharedEvent`) signaled after
//! each submission; objects whose lifetime must outlast GPU use are parked
//! with [`Queue::defer_release`] until the timeline passes.

use super::fence::{Fence, MtlEvent};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::Message;
use objc2_metal::*;
use std::collections::VecDeque;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};

pub type MtlQueue = ProtocolObject<dyn MTL4CommandQueue>;
pub type MtlCommandBuffer = ProtocolObject<dyn MTL4CommandBuffer>;

/// Result of [`Queue::execute_command_lists`] / [`Queue::execute_command_lists_at`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecuteResult {
    pub submitted_cbs: usize,
    pub timeline_value: u64,
}

/// Capture the `ExecuteCommandLists` submission timestamp when pass-timing is active (0 otherwise).
#[inline]
pub fn capture_submit_timestamp() -> u64 {
    if super::passtiming::enabled() {
        super::passtiming::mach_now()
    } else {
        0
    }
}

/// `AW2_SUBMIT_COALESCE`: elide 0-work command-buffer commits (`submit_coalesced`),
/// skip per-`ExecuteCommandLists` `signalEvent_value` timeline drains (`submit_nosig`)
/// on `ListType::Direct` (`q0`) by signaling `Queue::timeline` lazily on demand
/// (`last_submitted` / `Present` / `defer_release` / `wait_idle`), and batch `PassLog`
/// completion callbacks onto that single timeline signal while keeping `Queue::submit`
/// (`sq`/`pq`) unconditionally eager. Default **on** (`AW2_SUBMIT_COALESCE=0` disables).
pub fn submit_coalesce() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("AW2_SUBMIT_COALESCE").is_none_or(|v| v != "0"))
}

/// `PORT_FENCE_SIGNAL_COALESCE` (legacy alias `AW2_FENCE_SIGNAL_COALESCE`):
/// coalesce mid-frame D3D12 `ID3D12CommandQueue::Signal` calls on `ListType::Direct`
/// (`q0`) (`fence_sig_coalesced`) alongside buffered `submit_d3d_lists` commits,
/// flushing eagerly on cross-queue `Wait`, CPU `Wait` / `SetEventOnCompletion`,
/// `signal_cpu`, `Present` / `flush_timeline`, `wait_idle`, CS idle timeout, and
/// `GetCompletedValue` spin-polls. Default **on** (`=0` disables).
pub fn fence_signal_coalesce() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        std::env::var_os("PORT_FENCE_SIGNAL_COALESCE")
            .or_else(|| std::env::var_os("AW2_FENCE_SIGNAL_COALESCE"))
            .is_none_or(|v| v != "0")
    })
}

/// `PORT_ASYNC_COMMIT`: opt-in offload of Direct-queue `MTL4CommandQueue::commit:count:`
/// (plus fused ordered `signalEvent`/`waitForEvent` ops) to a dedicated in-order worker
/// thread. Default **off** (`PORT_ASYNC_COMMIT=1` enables): under `PORT_MESH_SHADERS=1`,
/// cross-thread commit handoff regressed Cauldron Lake p95 (+1.1–1.6 ms) and >16 ms tail
/// frames (2.5–4x) relative to inline commit with `PORT_QUEUE_START_STAGE_EXACT=1`.
pub fn async_commit() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("PORT_ASYNC_COMMIT").is_some_and(|v| v != "0"))
}

/// `D3D12_COMMAND_LIST_TYPE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListType {
    Direct = 0,
    Bundle = 1,
    Compute = 2,
    Copy = 3,
}

impl ListType {
    pub fn from_raw(v: u32) -> Option<ListType> {
        Some(match v {
            0 => ListType::Direct,
            1 => ListType::Bundle,
            2 => ListType::Compute,
            3 => ListType::Copy,
            _ => return None,
        })
    }
}

type Deferred = Box<dyn Send>;

struct DeferredFenceSig {
    ev_key: usize,
    ev: Retained<MtlEvent>,
    fence_ptr: usize,
    val: u64,
    enqueued_epoch: u64,
}

pub struct Queue {
    pub q: Retained<MtlQueue>,
    pub kind: ListType,
    pub timeline: Fence,
    /// Packed `(next << 1) | (unsignaled as u64)` so `last_submitted()` observes
    /// `(next, unsignaled)` atomically with zero tearing and zero locks.
    timeline_state: AtomicU64,
    has_deferred: AtomicBool,
    deferred: Mutex<Vec<(u64, Deferred)>>,
    /// Serializes submissions so timeline values are monotonic.
    submit: Mutex<()>,
    /// Strongest GPU wait enqueued since the last submission (diagnostics):
    /// 0 none, 1 already reached, 2 on another queue's signal, 3 on a CPU
    /// signal not yet made.
    pub pending_wait: AtomicU8,
    registered_active: AtomicBool,
    pending_log_count: AtomicUsize,
    pending_pass_logs: Mutex<Vec<super::passtiming::PassLog>>,
    pub(crate) opt_submit_coalesce: AtomicBool,
    is_swapchain_internal: bool,
    pub(crate) opt_fence_signal_coalesce: AtomicBool,
    commit_epoch: AtomicU64,
    has_deferred_signals: AtomicBool,
    deferred_signals: Mutex<Vec<DeferredFenceSig>>,
    /// Buffered closed `(MTL4CommandBuffer, alloc_key)` pairs from adjacent
    /// `submit_d3d_lists` calls coalesced into a single `MTL4CommandQueue::commit:count:`
    /// batch when ordering, fence, and signal semantics allow.
    has_pending_cbs: AtomicBool,
    pending_cbs: Mutex<Vec<(Retained<MtlCommandBuffer>, usize)>>,
    /// Optional dedicated in-order commit worker for non-swapchain Direct queues so
    /// slow MetalFX `preCommit` calls never stall `d3d12-metal-cs`.
    commit_worker: Option<Arc<CommitWorker>>,
}
unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}

/// Maximum buffered `MTL4CommandBuffer`s per queue before forcing a batched commit
/// (keeps GPU execution overlapping CPU encoding while cutting per-frame commits ~2-3x).
const MAX_COALESCED_CBS: usize = 4;

/// Total uncommitted command buffers currently buffered across all live queues.
static GLOBAL_PENDING_CBS: AtomicUsize = AtomicUsize::new(0);
/// Total command buffers currently queued or executing on async [`CommitWorker`] threads.
static GLOBAL_INFLIGHT_COMMIT_CBS: AtomicUsize = AtomicUsize::new(0);
/// Total live queues currently holding deferred fence signals.
static GLOBAL_PENDING_SIGS: AtomicUsize = AtomicUsize::new(0);

/// Lock-free check for whether any queue currently holds buffered uncommitted command buffers.
#[inline]
pub fn has_pending_cbs() -> bool {
    GLOBAL_PENDING_CBS.load(Ordering::Acquire) > 0
}

/// Lock-free check for whether any command buffer is either buffered in `pending_cbs`,
/// still in flight inside an async [`CommitWorker`], or holds active recording-level mesh
/// ICB ownership that must be released on `CommandList::begin` / `CommandAllocator::reset`.
#[inline]
pub fn has_uncommitted_cbs() -> bool {
    GLOBAL_PENDING_CBS.load(Ordering::Acquire) > 0
        || GLOBAL_INFLIGHT_COMMIT_CBS.load(Ordering::Acquire) > 0
        || crate::mesh::has_active_mesh_icbs()
}

/// Lock-free check for whether any queue currently holds buffered command buffers or deferred signals.
#[inline]
pub fn has_pending_work() -> bool {
    GLOBAL_PENDING_CBS.load(Ordering::Acquire) > 0
        || GLOBAL_PENDING_SIGS.load(Ordering::Acquire) > 0
}

fn active_queues() -> &'static Mutex<Vec<usize>> {
    static Q: OnceLock<Mutex<Vec<usize>>> = OnceLock::new();
    Q.get_or_init(Default::default)
}

/// Wrapper allowing externally synchronized Metal 4 retained objects to cross into
/// [`CommitWorker`].
struct SendMtl<T>(T);
unsafe impl<T> Send for SendMtl<T> {}
unsafe impl<T> Sync for SendMtl<T> {}

enum CommitOp {
    Commit(Vec<(Retained<MtlCommandBuffer>, usize)>),
    CommitAndSignals {
        cbs: Vec<(Retained<MtlCommandBuffer>, usize)>,
        signals: Vec<(Retained<MtlEvent>, u64)>,
    },
    Signal(Retained<MtlEvent>, u64),
    Wait(Retained<MtlEvent>, u64),
}
unsafe impl Send for CommitOp {}

struct CommitWorkerInner {
    queue: VecDeque<CommitOp>,
    /// `(cb_ptr, alloc_key)` of the batch currently inside `commit_cbs`.
    active_cbs: Vec<(usize, usize)>,
    active_op: bool,
    waiters: usize,
    shutdown: bool,
}

/// Dedicated per-queue in-order Metal 4 commit/event thread.
struct CommitWorker {
    q: SendMtl<Retained<MtlQueue>>,
    kind: ListType,
    is_swapchain_internal: bool,
    inner: Mutex<CommitWorkerInner>,
    work_cv: Condvar,
    done_cv: Condvar,
    in_flight_ops: AtomicUsize,
    in_flight_cbs: AtomicUsize,
    started: AtomicBool,
    handle: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl CommitWorker {
    fn new(q: Retained<MtlQueue>, kind: ListType, is_swapchain_internal: bool) -> Arc<Self> {
        Arc::new(Self {
            q: SendMtl(q),
            kind,
            is_swapchain_internal,
            inner: Mutex::new(CommitWorkerInner {
                queue: VecDeque::new(),
                active_cbs: Vec::new(),
                active_op: false,
                waiters: 0,
                shutdown: false,
            }),
            work_cv: Condvar::new(),
            done_cv: Condvar::new(),
            in_flight_ops: AtomicUsize::new(0),
            in_flight_cbs: AtomicUsize::new(0),
            started: AtomicBool::new(false),
            handle: Mutex::new(None),
        })
    }

    #[inline]
    fn ensure_started(self: &Arc<Self>) {
        if self.started.load(Ordering::Acquire) {
            return;
        }
        let mut h = self.handle.lock().unwrap();
        if self.started.load(Ordering::Relaxed) {
            return;
        }
        let me = Arc::clone(self);
        let jh = std::thread::Builder::new()
            .name("d3d12-metal-commit".into())
            .spawn(move || me.run())
            .expect("spawn d3d12-metal-commit thread");
        *h = Some(jh);
        self.started.store(true, Ordering::Release);
    }

    fn run(self: Arc<Self>) {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        unsafe {
            libc::pthread_set_qos_class_self_np(
                libc::qos_class_t::QOS_CLASS_USER_INTERACTIVE,
                0,
            );
        }
        loop {
            let op = {
                let mut g = self.inner.lock().unwrap();
                loop {
                    if let Some(op) = g.queue.pop_front() {
                        g.active_op = true;
                        let op_cbs = match &op {
                            CommitOp::Commit(cbs) | CommitOp::CommitAndSignals { cbs, .. } => {
                                Some(cbs.as_slice())
                            }
                            _ => None,
                        };
                        if let Some(cbs) = op_cbs {
                            g.active_cbs.clear();
                            g.active_cbs.reserve(cbs.len());
                            for (c, ak) in cbs {
                                g.active_cbs.push((Retained::as_ptr(c) as usize, *ak));
                            }
                        }
                        break op;
                    }
                    if g.shutdown {
                        return;
                    }
                    g = self.work_cv.wait(g).unwrap();
                }
            };
            let committed_cbs = match &op {
                CommitOp::Commit(cbs) | CommitOp::CommitAndSignals { cbs, .. } => cbs.len(),
                _ => 0,
            };
            objc2::rc::autoreleasepool(|_| match op {
                CommitOp::Commit(cbs) => {
                    commit_retained_cbs(&self.q.0, &cbs, self.kind, self.is_swapchain_internal);
                    drop(cbs);
                }
                CommitOp::CommitAndSignals { cbs, signals } => {
                    if !cbs.is_empty() {
                        commit_retained_cbs(&self.q.0, &cbs, self.kind, self.is_swapchain_internal);
                        drop(cbs);
                    }
                    for (ev, val) in signals {
                        self.q.0.signalEvent_value(ProtocolObject::from_ref(&*ev), val);
                    }
                }
                CommitOp::Signal(ev, val) => {
                    self.q.0.signalEvent_value(ProtocolObject::from_ref(&*ev), val);
                }
                CommitOp::Wait(ev, val) => {
                    self.q.0.waitForEvent_value(ProtocolObject::from_ref(&*ev), val);
                }
            });
            let mut g = self.inner.lock().unwrap();
            g.active_op = false;
            if committed_cbs > 0 {
                g.active_cbs.clear();
                self.in_flight_cbs.fetch_sub(committed_cbs, Ordering::AcqRel);
                GLOBAL_INFLIGHT_COMMIT_CBS.fetch_sub(committed_cbs, Ordering::AcqRel);
            }
            self.in_flight_ops.fetch_sub(1, Ordering::AcqRel);
            if g.waiters > 0 {
                self.done_cv.notify_all();
            }
        }
    }

    fn enqueue_commit(self: &Arc<Self>, cbs: Vec<(Retained<MtlCommandBuffer>, usize)>) {
        if cbs.is_empty() {
            return;
        }
        self.ensure_started();
        let n = cbs.len();
        self.in_flight_ops.fetch_add(1, Ordering::AcqRel);
        self.in_flight_cbs.fetch_add(n, Ordering::AcqRel);
        GLOBAL_INFLIGHT_COMMIT_CBS.fetch_add(n, Ordering::AcqRel);
        let mut g = self.inner.lock().unwrap();
        g.queue.push_back(CommitOp::Commit(cbs));
        self.work_cv.notify_one();
    }

    fn enqueue_commit_and_signals(
        self: &Arc<Self>,
        cbs: Vec<(Retained<MtlCommandBuffer>, usize)>,
        mut signals: Vec<(Retained<MtlEvent>, u64)>,
    ) {
        if cbs.is_empty() && signals.is_empty() {
            return;
        }
        if signals.is_empty() {
            self.enqueue_commit(cbs);
            return;
        }
        if cbs.is_empty() && signals.len() == 1 {
            let (ev, val) = signals.pop().unwrap();
            self.enqueue_signal(ev, val);
            return;
        }
        self.ensure_started();
        let n = cbs.len();
        self.in_flight_ops.fetch_add(1, Ordering::AcqRel);
        if n > 0 {
            self.in_flight_cbs.fetch_add(n, Ordering::AcqRel);
            GLOBAL_INFLIGHT_COMMIT_CBS.fetch_add(n, Ordering::AcqRel);
        }
        let mut g = self.inner.lock().unwrap();
        g.queue.push_back(CommitOp::CommitAndSignals { cbs, signals });
        self.work_cv.notify_one();
    }

    fn enqueue_signal(self: &Arc<Self>, ev: Retained<MtlEvent>, val: u64) {
        self.ensure_started();
        self.in_flight_ops.fetch_add(1, Ordering::AcqRel);
        let mut g = self.inner.lock().unwrap();
        g.queue.push_back(CommitOp::Signal(ev, val));
        self.work_cv.notify_one();
    }

    fn enqueue_wait(self: &Arc<Self>, ev: Retained<MtlEvent>, val: u64) {
        self.ensure_started();
        self.in_flight_ops.fetch_add(1, Ordering::AcqRel);
        let mut g = self.inner.lock().unwrap();
        g.queue.push_back(CommitOp::Wait(ev, val));
        self.work_cv.notify_one();
    }

    fn wait_if_cb_or_alloc_in_flight(&self, cb_ptr: usize, alloc_key: usize) {
        if self.in_flight_cbs.load(Ordering::Acquire) == 0 {
            return;
        }
        let matches = |g: &CommitWorkerInner| {
            g.active_cbs
                .iter()
                .any(|&(p, ak)| (cb_ptr != 0 && p == cb_ptr) || (alloc_key != 0 && (ak == 0 || ak == alloc_key)))
                || g.queue.iter().any(|op| match op {
                    CommitOp::Commit(cbs) | CommitOp::CommitAndSignals { cbs, .. } => {
                        cbs.iter().any(|(c, ak)| {
                            (cb_ptr != 0 && Retained::as_ptr(c) as usize == cb_ptr)
                                || (alloc_key != 0 && (*ak == 0 || *ak == alloc_key))
                        })
                    }
                    _ => false,
                })
        };
        let mut g = self.inner.lock().unwrap();
        if !matches(&g) {
            return;
        }
        g.waiters += 1;
        while matches(&g) {
            g = self.done_cv.wait(g).unwrap();
        }
        g.waiters -= 1;
    }

    fn drain(&self) {
        if self.in_flight_ops.load(Ordering::Acquire) == 0 {
            return;
        }
        let mut g = self.inner.lock().unwrap();
        if !g.active_op && g.queue.is_empty() {
            return;
        }
        g.waiters += 1;
        while g.active_op || !g.queue.is_empty() {
            g = self.done_cv.wait(g).unwrap();
        }
        g.waiters -= 1;
    }

    fn shutdown_and_join(&self) {
        if !self.started.load(Ordering::Acquire) {
            return;
        }
        {
            let mut g = self.inner.lock().unwrap();
            g.shutdown = true;
            self.work_cv.notify_all();
        }
        if let Some(jh) = self.handle.lock().unwrap().take() {
            let _ = jh.join();
        }
    }
}

struct DeferredReaper {
    pending: Mutex<Vec<Deferred>>,
    cv: Condvar,
}

fn deferred_reaper() -> &'static DeferredReaper {
    static REAPER: OnceLock<DeferredReaper> = OnceLock::new();
    REAPER.get_or_init(|| {
        let _ = std::thread::Builder::new()
            .name("d3d12-deferred-reaper".into())
            .spawn(|| {
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                unsafe {
                    // Match `USER_INITIATED` so dropping GPU resources (which touches
                    // `Device::residency_dirty`) never priority-inverts `d3d12-metal-cs`.
                    libc::pthread_set_qos_class_self_np(
                        libc::qos_class_t::QOS_CLASS_USER_INITIATED,
                        0,
                    );
                }
                let r = deferred_reaper();
                loop {
                    let batch = {
                        let mut g = r.pending.lock().unwrap();
                        while g.is_empty() {
                            g = r.cv.wait(g).unwrap();
                        }
                        std::mem::take(&mut *g)
                    };
                    objc2::rc::autoreleasepool(|_| drop(batch));
                }
            });
        DeferredReaper { pending: Mutex::new(Vec::new()), cv: Condvar::new() }
    })
}

/// Hand completed deferred releases to the background reaper thread so Metal
/// resource/heap deallocation never stalls `d3d12-metal-cs`.
#[inline]
fn reap_deferred_async(mut expired: Vec<Deferred>) {
    if expired.is_empty() {
        return;
    }
    let r = deferred_reaper();
    let mut g = r.pending.lock().unwrap();
    if g.is_empty() {
        *g = std::mem::take(&mut expired);
    } else {
        g.append(&mut expired);
    }
    r.cv.notify_one();
}

/// Inline capacity of [`PreparedSubmit`] (covers the common `1..=8` list `ExecuteCommandLists`
/// without heap allocation).
const PREPARED_INLINE_CAP: usize = 8;

/// Command buffers and pass-timing metadata extracted from closed [`CommandList`]s
/// so callers can release per-list locks before committing to Metal.
pub struct PreparedSubmit {
    inline: [Option<(Retained<MtlCommandBuffer>, usize)>; PREPARED_INLINE_CAP],
    inline_len: usize,
    overflow: Vec<(Retained<MtlCommandBuffer>, usize)>,
    pass_logs: Vec<super::passtiming::PassLog>,
    any_ml_work: bool,
    t_ecl: u64,
    coalesce: bool,
    pt_on: bool,
    wait_sampled: Option<u8>,
}

impl PreparedSubmit {
    #[inline]
    fn push_cb(&mut self, cb: Retained<MtlCommandBuffer>, alloc_key: usize) {
        if self.overflow.is_empty() && self.inline_len < PREPARED_INLINE_CAP {
            self.inline[self.inline_len] = Some((cb, alloc_key));
            self.inline_len += 1;
        } else {
            if self.overflow.is_empty() {
                self.overflow.reserve(PREPARED_INLINE_CAP * 2);
                for slot in &mut self.inline[..self.inline_len] {
                    if let Some(item) = slot.take() {
                        self.overflow.push(item);
                    }
                }
                self.inline_len = 0;
            }
            self.overflow.push((cb, alloc_key));
        }
    }

    #[inline]
    fn len(&self) -> usize {
        if !self.overflow.is_empty() {
            self.overflow.len()
        } else {
            self.inline_len
        }
    }
}

/// Wrapper allowing `Arc<DropOnly<T>>` to be `Send` for any `T: Send` without a `Mutex`.
///
/// SOUNDNESS: `DropOnly<T>` never exposes `&T` (shared references are impossible), so `T`
/// is only accessed when the final `Arc` drops `DropOnly<T>` with exclusive ownership
/// synchronized by `Arc`'s Release/Acquire refcount decrement.
struct DropOnly<T>(T);
unsafe impl<T: Send> Sync for DropOnly<T> {}

/// Park `obj` until all currently live queues have finished work submitted so far.
pub fn defer_release_all<T: Send + 'static>(obj: T) {
    let qs = active_queues().lock().unwrap().clone();
    if qs.is_empty() {
        return;
    }
    if qs.len() == 1 {
        // SAFETY: `qs[0]` was in `active_queues` and caller keeps live queues valid across deferral.
        let q = unsafe { &*(qs[0] as *const Queue) };
        q.defer_release(Box::new(obj));
        return;
    }
    let arc = std::sync::Arc::new(DropOnly(obj));
    for p in qs {
        // SAFETY: `p` is in `active_queues` until `Queue::drop`.
        let q = unsafe { &*(p as *const Queue) };
        q.defer_release(Box::new(arc.clone()));
    }
}

/// Reap completed deferred releases across every live queue (including idle Copy/Compute
/// queues that haven't submitted recently so they never pin retired GPU allocations).
pub fn collect_all() -> usize {
    let mut expired: Vec<Deferred> = Vec::new();
    {
        let qs = active_queues().lock().unwrap();
        for &p in qs.iter() {
            // SAFETY: `active_queues` lock prevents any listed `Queue` from completing `Drop`.
            let q = unsafe { &*(p as *const Queue) };
            q.drain_completed_into(&mut expired);
        }
    }
    let n = expired.len();
    drop(expired);
    n
}

/// GPU timeline sink for diagnostics: `f(queue_kind, n_command_buffers, commit_ns,
/// gpu_start_ns, gpu_end_ns)`, all on the host `CLOCK_UPTIME_RAW` nanosecond clock
/// (Metal's `GPUStartTime`/`GPUEndTime` share the mach host timebase). Called from a Metal
/// callback thread once the GPU has finished each commit.
pub type GpuFeedbackHook = fn(u32, u32, u64, u64, u64);

static FEEDBACK_HOOK: AtomicUsize = AtomicUsize::new(0);
static SUBMIT_EPOCH: AtomicU64 = AtomicU64::new(1);

/// Monotonic process-wide epoch incremented whenever non-empty command buffers are committed.
#[inline]
pub fn submit_epoch() -> u64 {
    SUBMIT_EPOCH.load(Ordering::Acquire)
}

/// Install the GPU timeline sink (diagnostic only; when unset each submit pays one relaxed load).
pub fn set_feedback_hook(f: GpuFeedbackHook) {
    FEEDBACK_HOOK.store(f as usize, Ordering::Release);
}

#[derive(Default)]
struct PerfMeterState {
    last_end_ns: u64,
    busy_ns: u64,
    submits: u64,
    cbs: u64,
}

static PERF_METER: Mutex<PerfMeterState> = Mutex::new(PerfMeterState {
    last_end_ns: 0,
    busy_ns: 0,
    submits: 0,
    cbs: 0,
});
static PERF_METER_TOTAL_BUSY_NS: AtomicU64 = AtomicU64::new(0);

fn perf_meter_callback(queue_kind: u32, n_cbs: u32, _commit_ns: u64, gpu_start_ns: u64, gpu_end_ns: u64) {
    // Track merged non-overlapping GPU busy time on Direct queues (`ListType::Direct as u32 == 0`).
    if queue_kind != ListType::Direct as u32 || gpu_end_ns <= gpu_start_ns {
        return;
    }
    if let Ok(mut st) = PERF_METER.lock() {
        let start = gpu_start_ns.max(st.last_end_ns);
        if gpu_end_ns > start {
            let delta = gpu_end_ns - start;
            st.busy_ns = st.busy_ns.saturating_add(delta);
            PERF_METER_TOTAL_BUSY_NS.fetch_add(delta, Ordering::Relaxed);
            st.last_end_ns = gpu_end_ns;
        }
        st.submits = st.submits.saturating_add(1);
        st.cbs = st.cbs.saturating_add(n_cbs as u64);
    }
}

/// Enable lightweight Direct-queue GPU busy-time metering via `MTL4CommitFeedback`
/// unless another feedback hook is already installed.
pub fn enable_perf_meter() {
    let _ = FEEDBACK_HOOK.compare_exchange(
        0,
        (perf_meter_callback as GpuFeedbackHook) as usize,
        Ordering::AcqRel,
        Ordering::Acquire,
    );
}

/// Non-destructively read the monotonic cumulative Direct-queue GPU busy time in
/// nanoseconds when the built-in perf meter is active, or `None` if GPU busy
/// metering is disabled or another feedback hook owns the slot.
pub fn perf_meter_total_busy_ns() -> Option<u64> {
    if FEEDBACK_HOOK.load(Ordering::Relaxed) == (perf_meter_callback as GpuFeedbackHook) as usize {
        Some(PERF_METER_TOTAL_BUSY_NS.load(Ordering::Relaxed))
    } else {
        None
    }
}

/// Return and reset `(gpu_busy_ns, submits, cbs)` accumulated since the previous call.
pub fn take_perf_meter_sample() -> (u64, u64, u64) {
    let mut st = PERF_METER.lock().unwrap();
    let out = (st.busy_ns, st.submits, st.cbs);
    st.busy_ns = 0;
    st.submits = 0;
    st.cbs = 0;
    out
}

extern "C" {
    fn clock_gettime_nsec_np(clock_id: u32) -> u64;
}
/// `CLOCK_UPTIME_RAW` on Darwin.
const CLOCK_UPTIME_RAW: u32 = 8;

#[inline]
pub(crate) fn now_uptime_ns() -> u64 {
    unsafe { clock_gettime_nsec_np(CLOCK_UPTIME_RAW) }
}

/// Commit with a feedback handler that forwards GPU start/end times to `hook`.
#[cold]
#[inline(never)]
fn commit_with_feedback(q: &MtlQueue, ptrs: &mut [NonNull<MtlCommandBuffer>], kind: ListType, hook: usize) {
    let hook: GpuFeedbackHook = unsafe { std::mem::transmute(hook) };
    let n = ptrs.len() as u32;
    let commit_ns = now_uptime_ns();
    let block = block2::RcBlock::new(move |fb: NonNull<ProtocolObject<dyn MTL4CommitFeedback>>| {
        let fb = unsafe { fb.as_ref() };
        let s = (fb.GPUStartTime() * 1e9) as u64;
        let e = (fb.GPUEndTime() * 1e9) as u64;
        hook(kind as u32, n, commit_ns, s, e);
    });
    let opts = MTL4CommitOptions::new();
    unsafe {
        opts.addFeedbackHandler(&*block as *const _ as *mut _);
        q.commit_count_options(NonNull::new(ptrs.as_mut_ptr()).unwrap(), ptrs.len(), &opts);
    }
}

#[cfg(test)]
static TEST_COMMIT_CALLS: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static TEST_COMMIT_CBS: AtomicUsize = AtomicUsize::new(0);

#[inline]
fn record_test_commit(n: usize) {
    #[cfg(test)]
    {
        TEST_COMMIT_CALLS.fetch_add(1, Ordering::Relaxed);
        TEST_COMMIT_CBS.fetch_add(n, Ordering::Relaxed);
    }
    #[cfg(not(test))]
    let _ = n;
}

/// Commit `cbs` to `q` using a stack-allocated pointer array for `<= 32` command
/// buffers (falling back to heap allocation only for larger batches).
///
/// Swapchain-internal queues (`pq` / `sq`) are excluded from the built-in Direct-queue
/// `perf_meter_callback` so async present blits do not contaminate `q0`'s `gpu_busy`
/// timestamps or submit/CB counts (custom diagnostic feedback hooks still observe them).
#[inline]
fn should_install_feedback(hook: usize, is_swapchain_internal: bool) -> bool {
    hook != 0
        && !(is_swapchain_internal && hook == (perf_meter_callback as GpuFeedbackHook) as usize)
}

#[inline]
fn commit_cbs(q: &MtlQueue, cbs: &[&MtlCommandBuffer], kind: ListType, is_swapchain_internal: bool) {
    const STACK_CAP: usize = 32;
    let hook = FEEDBACK_HOOK.load(Ordering::Relaxed);
    let attach_fb = should_install_feedback(hook, is_swapchain_internal);
    if cbs.len() <= STACK_CAP {
        let mut stack: [std::mem::MaybeUninit<NonNull<MtlCommandBuffer>>; STACK_CAP] =
            [const { std::mem::MaybeUninit::uninit() }; STACK_CAP];
        for (dst, &cb) in stack.iter_mut().zip(cbs.iter()) {
            dst.write(NonNull::from(cb));
        }
        // SAFETY: the first `cbs.len()` slots were just initialized above.
        let slice = unsafe {
            std::slice::from_raw_parts_mut(stack.as_mut_ptr() as *mut NonNull<MtlCommandBuffer>, cbs.len())
        };
        if attach_fb {
            commit_with_feedback(q, slice, kind, hook);
        } else {
            unsafe { q.commit_count(NonNull::new(slice.as_mut_ptr()).unwrap(), slice.len()) };
        }
    } else {
        let mut ptrs: Vec<NonNull<MtlCommandBuffer>> = cbs.iter().map(|c| NonNull::from(*c)).collect();
        if attach_fb {
            commit_with_feedback(q, &mut ptrs, kind, hook);
        } else {
            unsafe { q.commit_count(NonNull::new(ptrs.as_mut_ptr()).unwrap(), ptrs.len()) };
        }
    }
}

/// Commit retained command buffers onto `q`, flushing device residency first.
fn commit_retained_cbs(
    q: &MtlQueue,
    cbs: &[(Retained<MtlCommandBuffer>, usize)],
    kind: ListType,
    is_swapchain_internal: bool,
) {
    if cbs.is_empty() {
        return;
    }
    if let Some(d) = super::device() {
        d.commit_residency();
    }
    const STACK_CAP: usize = 32;
    let n = cbs.len();
    if n <= STACK_CAP {
        let mut stack: [std::mem::MaybeUninit<&MtlCommandBuffer>; STACK_CAP] =
            [const { std::mem::MaybeUninit::uninit() }; STACK_CAP];
        for (dst, (cb, _)) in stack.iter_mut().zip(cbs.iter()) {
            dst.write(&**cb);
        }
        // SAFETY: the first `n` slots were just initialized above, and `cbs`
        // keeps every `MTL4CommandBuffer` alive across `commit_cbs`.
        let slice = unsafe {
            std::slice::from_raw_parts(stack.as_ptr() as *const &MtlCommandBuffer, n)
        };
        commit_cbs(q, slice, kind, is_swapchain_internal);
    } else {
        let refs: Vec<&MtlCommandBuffer> = cbs.iter().map(|(cb, _)| &**cb).collect();
        commit_cbs(q, &refs, kind, is_swapchain_internal);
    }
}

impl Queue {
    #[inline]
    fn pack_timeline_state(next: u64, unsignaled: bool) -> u64 {
        (next << 1) | u64::from(unsignaled)
    }

    #[inline]
    fn unpack_timeline_state(st: u64) -> (u64, bool) {
        (st >> 1, (st & 1) != 0)
    }

    #[inline]
    fn is_unsignaled(&self) -> bool {
        (self.timeline_state.load(Ordering::Acquire) & 1) != 0
    }

    pub fn new(kind: ListType, label: &str) -> Option<Queue> {
        Self::new_inner(kind, label, async_commit())
    }

    #[cfg(test)]
    fn new_with_async_commit(kind: ListType, label: &str, use_async_commit: bool) -> Option<Queue> {
        Self::new_inner(kind, label, use_async_commit)
    }

    fn new_inner(kind: ListType, label: &str, use_async_commit: bool) -> Option<Queue> {
        let d = super::device()?;
        let desc = MTL4CommandQueueDescriptor::new();
        desc.setLabel(Some(&objc2_foundation::NSString::from_str(label)));
        let q = d.dev.newMTL4CommandQueueWithDescriptor_error(&desc).ok()?;
        q.addResidencySet(&d.residency);
        let lower = label.to_ascii_lowercase();
        let is_swapchain_internal = lower.contains("swapchain") || lower.contains("present");
        let allow_direct_coalesce = kind == ListType::Direct && !is_swapchain_internal;
        let commit_worker = if allow_direct_coalesce && use_async_commit {
            Some(CommitWorker::new(q.clone(), kind, is_swapchain_internal))
        } else {
            None
        };
        Some(Queue {
            q,
            kind,
            timeline: Fence::new(0)?,
            timeline_state: AtomicU64::new(Self::pack_timeline_state(1, false)),
            has_deferred: AtomicBool::new(false),
            deferred: Mutex::new(Vec::new()),
            submit: Mutex::new(()),
            pending_wait: AtomicU8::new(0),
            registered_active: AtomicBool::new(false),
            pending_log_count: AtomicUsize::new(0),
            pending_pass_logs: Mutex::new(Vec::new()),
            opt_submit_coalesce: AtomicBool::new(kind == ListType::Direct && submit_coalesce()),
            is_swapchain_internal,
            opt_fence_signal_coalesce: AtomicBool::new(allow_direct_coalesce && fence_signal_coalesce()),
            commit_epoch: AtomicU64::new(0),
            has_deferred_signals: AtomicBool::new(false),
            deferred_signals: Mutex::new(Vec::new()),
            has_pending_cbs: AtomicBool::new(false),
            pending_cbs: Mutex::new(Vec::new()),
            commit_worker,
        })
    }

    #[cfg(test)]
    pub(crate) fn set_submit_coalesce(&self, on: bool) {
        if !on {
            {
                let _g = self.submit.lock().unwrap();
                let _ = self.flush_timeline_locked();
            }
            self.collect();
        }
        self.opt_submit_coalesce.store(on && self.kind == ListType::Direct, Ordering::Release);
    }

    #[cfg(test)]
    pub(crate) fn set_fence_signal_coalesce(&self, on: bool) {
        let allow = on && self.kind == ListType::Direct && !self.is_swapchain_internal;
        if !allow {
            let _g = self.submit.lock().unwrap();
            self.flush_fence_signals_locked();
        }
        self.opt_fence_signal_coalesce.store(allow, Ordering::Release);
    }

    #[inline]
    fn with_live_queue<R>(q_addr: usize, f: impl FnOnce(&Queue) -> R) -> Option<R> {
        if q_addr == 0 {
            return None;
        }
        let aq = active_queues().lock().unwrap();
        if !aq.contains(&q_addr) {
            return None;
        }
        // SAFETY: `q_addr` remains valid while present in `active_queues`, and `f`
        // only acquires `q.submit` (never `active_queues`), preserving lock order.
        let q = unsafe { &*(q_addr as *const Queue) };
        Some(f(q))
    }

    pub(crate) fn commit_epoch_of(q_addr: usize) -> u64 {
        Self::with_live_queue(q_addr, |q| q.commit_epoch.load(Ordering::Acquire)).unwrap_or(0)
    }

    pub(crate) fn flush_fence_signals_by_addr(q_addr: usize) {
        let _ = Self::with_live_queue(q_addr, |q| {
            if q.has_deferred_signals.load(Ordering::Acquire) {
                let _g = q.submit.lock().unwrap();
                q.flush_fence_signals_locked();
            }
        });
    }

    #[inline]
    fn dispatch_commit_locked(&self, cbs: Vec<(Retained<MtlCommandBuffer>, usize)>) {
        if cbs.is_empty() {
            return;
        }
        record_test_commit(cbs.len());
        if let Some(cw) = &self.commit_worker {
            cw.enqueue_commit(cbs);
        } else {
            commit_retained_cbs(&self.q, &cbs, self.kind, self.is_swapchain_internal);
        }
    }

    /// Dispatch a commit batch and any immediately-following event signals as a single
    /// fused [`CommitOp`] on [`CommitWorker`] (one mutex acquisition and one condvar wake)
    /// when async commit is active, or inline in strict FIFO order otherwise.
    #[inline]
    fn dispatch_commit_and_signals_locked(
        &self,
        cbs: Vec<(Retained<MtlCommandBuffer>, usize)>,
        signals: Vec<(Retained<MtlEvent>, u64)>,
    ) {
        if !cbs.is_empty() {
            record_test_commit(cbs.len());
        }
        if let Some(cw) = &self.commit_worker {
            cw.enqueue_commit_and_signals(cbs, signals);
        } else {
            if !cbs.is_empty() {
                commit_retained_cbs(&self.q, &cbs, self.kind, self.is_swapchain_internal);
            }
            for (ev, val) in signals {
                self.q.signalEvent_value(ProtocolObject::from_ref(&*ev), val);
            }
        }
    }

    #[inline]
    fn dispatch_wait_locked(&self, ev: &MtlEvent, val: u64) {
        if let Some(cw) = &self.commit_worker {
            cw.enqueue_wait(ev.retain(), val);
        } else {
            self.q.waitForEvent_value(ProtocolObject::from_ref(ev), val);
        }
    }

    /// Take any buffered uncommitted command buffers without dispatching them yet.
    /// Caller must hold `self.submit`.
    #[inline]
    fn take_pending_cbs_locked(&self) -> Vec<(Retained<MtlCommandBuffer>, usize)> {
        if !self.has_pending_cbs.load(Ordering::Acquire) {
            return Vec::new();
        }
        let cbs = {
            let mut pc = self.pending_cbs.lock().unwrap();
            self.has_pending_cbs.store(false, Ordering::Release);
            std::mem::take(&mut *pc)
        };
        if !cbs.is_empty() {
            GLOBAL_PENDING_CBS.fetch_sub(cbs.len(), Ordering::AcqRel);
        }
        cbs
    }

    /// Flush any buffered uncommitted command buffers on `self.q` in a single
    /// `MTL4CommandQueue::commit:count:` call. Caller must hold `self.submit`.
    fn flush_pending_cbs_locked(&self) {
        let cbs = self.take_pending_cbs_locked();
        if !cbs.is_empty() {
            self.dispatch_commit_locked(cbs);
        }
    }

    /// Ensure `cb` is no longer buffered in `pending_cbs`, neither `cb` nor `alloc_key`
    /// is still in flight on an async [`CommitWorker`], and any prior recording-level
    /// mesh ICB ownership for `cb` is released before `CommandList::begin(&alloc)`.
    pub(crate) fn flush_if_cb_or_alloc_in_flight(cb: &MtlCommandBuffer, alloc_key: usize) {
        if !has_uncommitted_cbs() {
            return;
        }
        let target = cb as *const MtlCommandBuffer as usize;
        if GLOBAL_PENDING_CBS.load(Ordering::Acquire) > 0
            || GLOBAL_INFLIGHT_COMMIT_CBS.load(Ordering::Acquire) > 0
        {
            const INLINE_W: usize = 4;
            let mut inline_w: [Option<Arc<CommitWorker>>; INLINE_W] = [const { None }; INLINE_W];
            let mut inline_len = 0usize;
            let mut overflow_w: Vec<Arc<CommitWorker>> = Vec::new();
            {
                let aq = active_queues().lock().unwrap();
                for &p in aq.iter() {
                    // SAFETY: `active_queues` lock prevents any listed `Queue` from completing `Drop`.
                    let q = unsafe { &*(p as *const Queue) };
                    if q.has_pending_cbs.load(Ordering::Acquire) {
                        let _g = q.submit.lock().unwrap();
                        let found = {
                            let pc = q.pending_cbs.lock().unwrap();
                            pc.iter().any(|(c, _)| Retained::as_ptr(c) as usize == target)
                        };
                        if found {
                            q.flush_pending_cbs_locked();
                        }
                    }
                    if let Some(cw) = &q.commit_worker {
                        if cw.in_flight_cbs.load(Ordering::Acquire) > 0 {
                            if inline_len < INLINE_W {
                                inline_w[inline_len] = Some(Arc::clone(cw));
                                inline_len += 1;
                            } else {
                                overflow_w.push(Arc::clone(cw));
                            }
                        }
                    }
                }
            }
            for slot in &mut inline_w[..inline_len] {
                if let Some(cw) = slot.take() {
                    cw.wait_if_cb_or_alloc_in_flight(target, alloc_key);
                }
            }
            for cw in overflow_w {
                cw.wait_if_cb_or_alloc_in_flight(target, alloc_key);
            }
        }
        if crate::mesh::has_active_mesh_icbs() {
            crate::mesh::release_mesh_icbs_for_cb(target);
        }
    }

    /// Flush buffered command buffers on whichever live queue holds a CB recorded
    /// with `alloc_key` (or an unknown `0` alloc_key), wait for any matching async
    /// [`CommitWorker`] batch to return from `commit:count:`, and release recording-level
    /// mesh ICB ownership for `alloc_key` before `CommandAllocator::reset()` or `Drop`.
    pub(crate) fn flush_if_alloc_pending(alloc_key: usize) {
        if !has_uncommitted_cbs() {
            return;
        }
        if GLOBAL_PENDING_CBS.load(Ordering::Acquire) > 0
            || GLOBAL_INFLIGHT_COMMIT_CBS.load(Ordering::Acquire) > 0
        {
            const INLINE_W: usize = 4;
            let mut inline_w: [Option<Arc<CommitWorker>>; INLINE_W] = [const { None }; INLINE_W];
            let mut inline_len = 0usize;
            let mut overflow_w: Vec<Arc<CommitWorker>> = Vec::new();
            {
                let aq = active_queues().lock().unwrap();
                for &p in aq.iter() {
                    // SAFETY: `active_queues` lock prevents any listed `Queue` from completing `Drop`.
                    let q = unsafe { &*(p as *const Queue) };
                    if q.has_pending_cbs.load(Ordering::Acquire) {
                        let _g = q.submit.lock().unwrap();
                        let found = {
                            let pc = q.pending_cbs.lock().unwrap();
                            pc.iter().any(|(_, ak)| *ak == 0 || *ak == alloc_key)
                        };
                        if found {
                            q.flush_pending_cbs_locked();
                        }
                    }
                    if let Some(cw) = &q.commit_worker {
                        if cw.in_flight_cbs.load(Ordering::Acquire) > 0 {
                            if inline_len < INLINE_W {
                                inline_w[inline_len] = Some(Arc::clone(cw));
                                inline_len += 1;
                            } else {
                                overflow_w.push(Arc::clone(cw));
                            }
                        }
                    }
                }
            }
            for slot in &mut inline_w[..inline_len] {
                if let Some(cw) = slot.take() {
                    cw.wait_if_cb_or_alloc_in_flight(0, alloc_key);
                }
            }
            for cw in overflow_w {
                cw.wait_if_cb_or_alloc_in_flight(0, alloc_key);
            }
        }
        if crate::mesh::has_active_mesh_icbs() {
            crate::mesh::release_mesh_icbs_for_alloc(alloc_key);
        }
    }

    #[inline]
    fn set_has_deferred_signals(&self, on: bool) {
        let prev = self.has_deferred_signals.swap(on, Ordering::AcqRel);
        if on && !prev {
            GLOBAL_PENDING_SIGS.fetch_add(1, Ordering::AcqRel);
        } else if !on && prev {
            GLOBAL_PENDING_SIGS.fetch_sub(1, Ordering::AcqRel);
        }
    }

    /// Flush buffered uncommitted command buffers on every live queue (called before
    /// `CommandAllocator::reset()` or on `cs::flush()`).
    pub fn flush_all_pending_cbs() {
        if !has_pending_cbs() {
            return;
        }
        let aq = active_queues().lock().unwrap();
        for &p in aq.iter() {
            // SAFETY: `active_queues` lock prevents any listed `Queue` from completing `Drop`.
            let q = unsafe { &*(p as *const Queue) };
            if q.has_pending_cbs.load(Ordering::Acquire) {
                let _g = q.submit.lock().unwrap();
                q.flush_pending_cbs_locked();
            }
        }
    }

    /// Flush buffered command buffers AND deferred fence signals on every live queue
    /// (called when the CS thread idles so no pending GPU commit or signal stays buffered).
    pub fn flush_all_pending_work() {
        if !has_pending_work() {
            return;
        }
        let aq = active_queues().lock().unwrap();
        for &p in aq.iter() {
            // SAFETY: `active_queues` lock prevents any listed `Queue` from completing `Drop`.
            let q = unsafe { &*(p as *const Queue) };
            if q.has_pending_cbs.load(Ordering::Acquire)
                || q.has_deferred_signals.load(Ordering::Acquire)
            {
                let _g = q.submit.lock().unwrap();
                let cbs = q.take_pending_cbs_locked();
                let sigs = q.take_deferred_fence_signals_locked();
                q.dispatch_commit_and_signals_locked(cbs, sigs);
            }
        }
    }

    /// Resolve any pending deferred GPU signal on `q_addr` for `ev_key` before a CPU
    /// `Fence::signal_cpu(v)`: cancels stale `<= v` GPU signals so a later GPU flush
    /// can never overwrite `v` backward, or emits `> v` onto `q_addr` after setting `v`.
    pub(crate) fn resolve_for_cpu_signal(
        q_addr: usize,
        ev_key: usize,
        fence_ptr: usize,
        v: u64,
    ) -> bool {
        Self::with_live_queue(q_addr, |q| {
            let _g = q.submit.lock().unwrap();
            let mut ds = q.deferred_signals.lock().unwrap();
            let mut greater_sig: Option<(Retained<MtlEvent>, u64)> = None;
            ds.retain(|e| {
                if e.ev_key == ev_key {
                    if e.val > v {
                        greater_sig = Some((e.ev.clone(), e.val));
                    }
                    false
                } else {
                    true
                }
            });
            q.set_has_deferred_signals(!ds.is_empty());
            drop(ds);
            if fence_ptr != 0 {
                // Drain any in-flight async signal ops before mutating `ev` on CPU so
                // a previously queued `signalEvent(ev, <= v)` cannot overwrite `v`.
                if let Some(cw) = &q.commit_worker {
                    cw.drain();
                }
                let f = unsafe { &*(fence_ptr as *const Fence) };
                f.pending_sig_queue.store(0, Ordering::Release);
                f.pending_sig_value.store(0, Ordering::Release);
                f.poll_count.store(0, Ordering::Release);
                f.emitted_sig_value.fetch_max(v, Ordering::AcqRel);
                f.ev.setSignaledValue(v);
            }
            if let Some((ev, gv)) = greater_sig {
                let cbs = q.take_pending_cbs_locked();
                if fence_ptr != 0 {
                    let f = unsafe { &*(fence_ptr as *const Fence) };
                    f.emitted_sig_value.fetch_max(gv, Ordering::AcqRel);
                }
                q.dispatch_commit_and_signals_locked(cbs, vec![(ev, gv)]);
            }
            true
        })
        .unwrap_or(false)
    }

    pub(crate) fn detach_dropped_fence(q_addr: usize, ev_key: usize, fence_ptr: usize) {
        let _ = Self::with_live_queue(q_addr, |q| {
            let _g = q.submit.lock().unwrap();
            let mut ds = q.deferred_signals.lock().unwrap();
            for e in ds.iter_mut() {
                if e.ev_key == ev_key && e.fence_ptr == fence_ptr {
                    e.fence_ptr = 0;
                }
            }
        });
    }

    /// Drain all deferred `ID3D12CommandQueue::Signal` entries in enqueue order,
    /// updating per-fence state and returning `(event, value)` pairs ready to emit.
    /// Caller must hold `self.submit`.
    fn take_deferred_fence_signals_locked(&self) -> Vec<(Retained<MtlEvent>, u64)> {
        if !self.has_deferred_signals.load(Ordering::Acquire) {
            return Vec::new();
        }
        self.set_has_deferred_signals(false);
        let entries = {
            let mut ds = self.deferred_signals.lock().unwrap();
            std::mem::take(&mut *ds)
        };
        if entries.is_empty() {
            return Vec::new();
        }
        let cur_epoch = self.commit_epoch.load(Ordering::Acquire);
        let me = self as *const Queue as usize;
        let mut out = Vec::with_capacity(entries.len());
        for entry in entries {
            if entry.fence_ptr != 0 {
                let f = unsafe { &*(entry.fence_ptr as *const Fence) };
                let _ = f.pending_sig_queue.compare_exchange(
                    me,
                    0,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                );
                f.pending_sig_value.store(0, Ordering::Release);
                f.poll_count.store(0, Ordering::Release);
                f.emitted_sig_value.fetch_max(entry.val, Ordering::AcqRel);
            }
            if entry.enqueued_epoch < cur_epoch {
                super::encstats::inc(&super::encstats::c::fence_sig_coalesced);
            }
            out.push((entry.ev, entry.val));
        }
        out
    }

    /// Emit all deferred `ID3D12CommandQueue::Signal` events in enqueue order
    /// (fusing any pending CB commit + signals into one `CommitOp`).
    /// Caller must hold `self.submit`.
    fn flush_fence_signals_locked(&self) {
        if !self.has_deferred_signals.load(Ordering::Acquire) {
            return;
        }
        let sigs = self.take_deferred_fence_signals_locked();
        if sigs.is_empty() {
            return;
        }
        // Any buffered CBs submitted prior to these signals MUST commit before signaling.
        let cbs = self.take_pending_cbs_locked();
        self.dispatch_commit_and_signals_locked(cbs, sigs);
    }

    #[inline]
    fn ensure_registered_active(&self) {
        if !self.registered_active.swap(true, Ordering::AcqRel) {
            let addr = self as *const Queue as usize;
            let mut aq = active_queues().lock().unwrap();
            if !aq.contains(&addr) {
                aq.push(addr);
            }
        }
    }

    /// Emit `signalEvent_value(&self.timeline.ev, v)` if any committed command buffers
    /// or `PassLog`s are awaiting a timeline value, fusing pending CBs, deferred fence
    /// signals, and the timeline signal into a single `CommitOp`. Caller must hold
    /// `self.submit` and run [`Self::collect`] AFTER releasing `self.submit`.
    fn flush_timeline_locked(&self) -> u64 {
        let cbs = self.take_pending_cbs_locked();
        let mut sigs = self.take_deferred_fence_signals_locked();
        let logs = if self.pending_log_count.load(Ordering::Acquire) > 0 {
            let mut pl = self.pending_pass_logs.lock().unwrap();
            self.pending_log_count.store(0, Ordering::Release);
            std::mem::take(&mut *pl)
        } else {
            Vec::new()
        };
        let (v, was_unsignaled) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
        if !was_unsignaled && logs.is_empty() {
            self.dispatch_commit_and_signals_locked(cbs, sigs);
            return v - 1;
        }
        sigs.push((self.timeline.ev.clone(), v));
        self.dispatch_commit_and_signals_locked(cbs, sigs);
        self.timeline_state
            .store(Self::pack_timeline_state(v + 1, false), Ordering::Release);
        if !logs.is_empty() {
            self.timeline
                .on_completion_no_flush(v, Box::new(move || super::passtiming::resolve(logs)));
        }
        v
    }

    /// Commit closed command buffers in order and **always** advance and signal
    /// `self.timeline` eagerly (`v >= 1`). Used by `Swapchain` (`sq`), `PresentThread`
    /// (`pq`), offscreen/main present blits, and internal/test callers that synchronize
    /// directly on the returned timeline value.
    pub fn submit(&self, cbs: &[&MtlCommandBuffer]) -> u64 {
        self.submit_with_logs(cbs, Vec::new())
    }

    #[inline]
    fn retire_mesh_icbs_for_ptrs(&self, cb_ptrs: &[usize]) {
        if !crate::mesh::has_active_mesh_icbs() {
            return;
        }
        for &ptr in cb_ptrs {
            if let Some(lease) = crate::mesh::retain_mesh_icbs_for_cb(ptr) {
                self.defer_release(Box::new(lease));
            }
        }
    }

    /// Commit closed command buffers in order and eagerly signal `self.timeline`,
    /// folding in any previously deferred `unsignaled` command buffers or `PassLog`s.
    pub fn submit_with_logs(&self, cbs: &[&MtlCommandBuffer], mut logs: Vec<super::passtiming::PassLog>) -> u64 {
        self.ensure_registered_active();
        let cb_ptrs: Vec<usize> = if crate::mesh::has_active_mesh_icbs() {
            cbs.iter().map(|&cb| cb as *const MtlCommandBuffer as usize).collect()
        } else {
            Vec::new()
        };
        let v = {
            let _g = self.submit.lock().unwrap();
            if self.pending_log_count.load(Ordering::Acquire) > 0 {
                let mut pl = self.pending_pass_logs.lock().unwrap();
                self.pending_log_count.store(0, Ordering::Release);
                if !pl.is_empty() {
                    pl.append(&mut logs);
                    logs = std::mem::take(&mut *pl);
                }
            }
            let pending = self.take_pending_cbs_locked();
            let mut sigs = self.take_deferred_fence_signals_locked();
            let (v, _) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
            if self.commit_worker.is_some() {
                let mut combined = pending;
                if !cbs.is_empty() {
                    combined.reserve(cbs.len());
                    for &cb in cbs {
                        combined.push((cb.retain(), 0));
                    }
                    self.commit_epoch.fetch_add(1, Ordering::AcqRel);
                    SUBMIT_EPOCH.fetch_add(1, Ordering::AcqRel);
                }
                sigs.push((self.timeline.ev.clone(), v));
                self.dispatch_commit_and_signals_locked(combined, sigs);
            } else {
                if !pending.is_empty() || !cbs.is_empty() {
                    if let Some(d) = super::device() {
                        d.commit_residency();
                    }
                    if pending.is_empty() {
                        record_test_commit(cbs.len());
                        commit_cbs(&self.q, cbs, self.kind, self.is_swapchain_internal);
                        self.commit_epoch.fetch_add(1, Ordering::AcqRel);
                        SUBMIT_EPOCH.fetch_add(1, Ordering::AcqRel);
                    } else if cbs.is_empty() {
                        let refs: Vec<&MtlCommandBuffer> = pending.iter().map(|(c, _)| &**c).collect();
                        record_test_commit(refs.len());
                        commit_cbs(&self.q, &refs, self.kind, self.is_swapchain_internal);
                    } else {
                        let mut combined: Vec<&MtlCommandBuffer> =
                            Vec::with_capacity(pending.len() + cbs.len());
                        combined.extend(pending.iter().map(|(c, _)| &**c));
                        combined.extend_from_slice(cbs);
                        record_test_commit(combined.len());
                        commit_cbs(&self.q, &combined, self.kind, self.is_swapchain_internal);
                        self.commit_epoch.fetch_add(1, Ordering::AcqRel);
                        SUBMIT_EPOCH.fetch_add(1, Ordering::AcqRel);
                    }
                }
                for (ev, sv) in sigs {
                    self.q.signalEvent_value(ProtocolObject::from_ref(&*ev), sv);
                }
                self.q.signalEvent_value(ProtocolObject::from_ref(&*self.timeline.ev), v);
            }
            self.timeline_state
                .store(Self::pack_timeline_state(v + 1, false), Ordering::Release);
            if !logs.is_empty() {
                self.timeline
                    .on_completion_no_flush(v, Box::new(move || super::passtiming::resolve(logs)));
            }
            v
        };
        if !cb_ptrs.is_empty() {
            self.retire_mesh_icbs_for_ptrs(&cb_ptrs);
        }
        self.collect_async();
        v
    }

    /// Submit D3D12 `ExecuteCommandLists` command buffers. When `AW2_SUBMIT_COALESCE=1`
    /// is enabled on a `ListType::Direct` queue (`q0`), buffers adjacent command buffers
    /// into a single Metal `commit:count:` batch, skips per-call `signalEvent_value`
    /// timeline drains (`submit_nosig`), and batches `PassLog` callbacks until an ordering
    /// or demand boundary (`signal`, non-elided `wait`, `flush_timeline`, `Present`,
    /// `wait_idle`, `submit`, CB reuse, `MAX_COALESCED_CBS`, or 8 pending `PassLog`s).
    pub fn submit_d3d_lists(&self, cbs: &[&MtlCommandBuffer], logs: Vec<super::passtiming::PassLog>) -> u64 {
        self.submit_d3d_lists_inner(cbs, None, logs, false)
    }

    fn submit_d3d_lists_inner(
        &self,
        cbs: &[&MtlCommandBuffer],
        alloc_keys: Option<&[usize]>,
        logs: Vec<super::passtiming::PassLog>,
        eager_flush: bool,
    ) -> u64 {
        let coalesce = self.kind == ListType::Direct
            && !self.is_swapchain_internal
            && self.opt_submit_coalesce.load(Ordering::Relaxed);
        if !coalesce {
            return self.submit_with_logs(cbs, logs);
        }
        self.ensure_registered_active();
        let cb_ptrs: Vec<usize> = if crate::mesh::has_active_mesh_icbs() {
            cbs.iter().map(|&cb| cb as *const MtlCommandBuffer as usize).collect()
        } else {
            Vec::new()
        };
        let out = {
            let _g = self.submit.lock().unwrap();
            if cbs.is_empty() && logs.is_empty() {
                super::encstats::inc(&super::encstats::c::submit_nosig);
                let (next, _) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
                next - 1
            } else {
                if !cbs.is_empty() {
                    // Flush first if any incoming CB is already buffered (re-execution of the
                    // same command buffer) or appending would exceed our batch cap.
                    if self.has_pending_cbs.load(Ordering::Acquire) {
                        let need_flush = {
                            let pc = self.pending_cbs.lock().unwrap();
                            pc.len() + cbs.len() > MAX_COALESCED_CBS
                                || cbs.iter().any(|&cb| {
                                    let ptr = cb as *const MtlCommandBuffer as usize;
                                    pc.iter().any(|(c, _)| Retained::as_ptr(c) as usize == ptr)
                                })
                        };
                        if need_flush {
                            self.flush_pending_cbs_locked();
                        }
                    }
                    let (had_existing, new_len) = {
                        let mut pc = self.pending_cbs.lock().unwrap();
                        let had = !pc.is_empty();
                        pc.reserve(cbs.len());
                        for (idx, &cb) in cbs.iter().enumerate() {
                            let ak = alloc_keys.and_then(|ks| ks.get(idx).copied()).unwrap_or(0);
                            pc.push((cb.retain(), ak));
                        }
                        self.has_pending_cbs.store(true, Ordering::Release);
                        (had, pc.len())
                    };
                    GLOBAL_PENDING_CBS.fetch_add(cbs.len(), Ordering::AcqRel);
                    if had_existing {
                        super::encstats::inc(&super::encstats::c::submit_coalesced);
                    }
                    self.commit_epoch.fetch_add(1, Ordering::AcqRel);
                    SUBMIT_EPOCH.fetch_add(1, Ordering::AcqRel);
                    let (next, _) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
                    self.timeline_state
                        .store(Self::pack_timeline_state(next, true), Ordering::Release);
                    if eager_flush || new_len >= MAX_COALESCED_CBS {
                        self.flush_pending_cbs_locked();
                    }
                }
                let total_logs = if !logs.is_empty() {
                    let mut pl = self.pending_pass_logs.lock().unwrap();
                    if !pl.is_empty() {
                        super::encstats::inc(&super::encstats::c::submit_coalesced);
                    }
                    pl.extend(logs);
                    let len = pl.len();
                    self.pending_log_count.store(len, Ordering::Release);
                    len
                } else {
                    self.pending_log_count.load(Ordering::Relaxed)
                };
                if total_logs >= 8 {
                    self.flush_timeline_locked()
                } else {
                    super::encstats::inc(&super::encstats::c::submit_nosig);
                    let (next, _) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
                    next - 1
                }
            }
        };
        if !cb_ptrs.is_empty() {
            self.retire_mesh_icbs_for_ptrs(&cb_ptrs);
        }
        self.collect_async();
        out
    }

    /// Begin gathering closed [`CommandList`]s for submission so callers can drop
    /// per-list locks before calling [`Self::submit_prepared`].
    #[inline]
    pub fn begin_prepare(&self, t_ecl: u64) -> PreparedSubmit {
        PreparedSubmit {
            inline: [const { None }; PREPARED_INLINE_CAP],
            inline_len: 0,
            overflow: Vec::new(),
            pass_logs: Vec::new(),
            any_ml_work: false,
            t_ecl,
            coalesce: self.kind == ListType::Direct
                && self.opt_submit_coalesce.load(Ordering::Relaxed),
            pt_on: super::passtiming::enabled(),
            wait_sampled: None,
        }
    }

    /// Extract the closed command buffer and pass-timing log from one [`CommandList`].
    #[inline]
    pub fn prepare_list(
        &self,
        batch: &mut PreparedSubmit,
        list: &mut super::cmdlist::CommandList,
        idx: usize,
    ) {
        if list.is_open() {
            log::error!(target: "d3d", "ExecuteCommandLists: list {idx} is still open (not Closed); skipped");
            return;
        }
        if list.kind != self.kind && self.kind != ListType::Direct {
            log::error!(target: "d3d", "ExecuteCommandLists: {:?} list on a {:?} queue", list.kind, self.kind);
        }
        if batch.coalesce && list.has_commands() && !list.has_gpu_work_recorded() {
            let _ = list.take_pass_log();
            super::encstats::inc(&super::encstats::c::submit_coalesced);
            return;
        }
        if batch.pt_on {
            if let Some(mut p) = list.take_pass_log() {
                let wait = *batch
                    .wait_sampled
                    .get_or_insert_with(|| self.pending_wait.swap(0, Ordering::Relaxed));
                p.submitted(batch.t_ecl, self.kind as u8, wait);
                batch.pass_logs.push(p);
            }
        }
        if list.has_commands() {
            batch.any_ml_work |= list.has_ml_work_recorded();
            batch.push_cb(list.cb.retain(), list.alloc_key());
        }
    }

    /// Submit a [`PreparedSubmit`] batch after all [`CommandList`] locks have been released.
    pub fn submit_prepared(&self, mut batch: PreparedSubmit) -> ExecuteResult {
        let submitted_cbs = batch.len();
        let coalesce = self.kind == ListType::Direct
            && !self.is_swapchain_internal
            && self.opt_submit_coalesce.load(Ordering::Relaxed);
        if !coalesce {
            let timeline_value = if !batch.overflow.is_empty() {
                let refs: Vec<&MtlCommandBuffer> =
                    batch.overflow.iter().map(|(c, _)| &**c).collect();
                self.submit_with_logs(&refs, batch.pass_logs)
            } else {
                let mut stack: [std::mem::MaybeUninit<&MtlCommandBuffer>; PREPARED_INLINE_CAP] =
                    [const { std::mem::MaybeUninit::uninit() }; PREPARED_INLINE_CAP];
                for (dst, slot) in stack.iter_mut().zip(batch.inline[..batch.inline_len].iter()) {
                    dst.write(&*slot.as_ref().unwrap().0);
                }
                let refs = unsafe {
                    std::slice::from_raw_parts(
                        stack.as_ptr() as *const &MtlCommandBuffer,
                        batch.inline_len,
                    )
                };
                self.submit_with_logs(refs, batch.pass_logs)
            };
            return ExecuteResult { submitted_cbs, timeline_value };
        }
        self.ensure_registered_active();
        let cb_ptrs: Vec<usize> = if crate::mesh::has_active_mesh_icbs() && submitted_cbs > 0 {
            if !batch.overflow.is_empty() {
                batch.overflow.iter().map(|(c, _)| Retained::as_ptr(c) as usize).collect()
            } else {
                batch.inline[..batch.inline_len]
                    .iter()
                    .filter_map(|s| s.as_ref().map(|(c, _)| Retained::as_ptr(c) as usize))
                    .collect()
            }
        } else {
            Vec::new()
        };
        let out = {
            let _g = self.submit.lock().unwrap();
            if submitted_cbs == 0 && batch.pass_logs.is_empty() {
                super::encstats::inc(&super::encstats::c::submit_nosig);
                let (next, _) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
                next - 1
            } else {
                if submitted_cbs > 0 {
                    if self.has_pending_cbs.load(Ordering::Acquire) {
                        let need_flush = {
                            let pc = self.pending_cbs.lock().unwrap();
                            pc.len() + submitted_cbs > MAX_COALESCED_CBS
                                || if !batch.overflow.is_empty() {
                                    batch.overflow.iter().any(|(cb, _)| {
                                        let ptr = Retained::as_ptr(cb) as usize;
                                        pc.iter().any(|(c, _)| Retained::as_ptr(c) as usize == ptr)
                                    })
                                } else {
                                    batch.inline[..batch.inline_len].iter().any(|slot| {
                                        let ptr = Retained::as_ptr(&slot.as_ref().unwrap().0) as usize;
                                        pc.iter().any(|(c, _)| Retained::as_ptr(c) as usize == ptr)
                                    })
                                }
                        };
                        if need_flush {
                            self.flush_pending_cbs_locked();
                        }
                    }
                    let (had_existing, new_len) = {
                        let mut pc = self.pending_cbs.lock().unwrap();
                        let had = !pc.is_empty();
                        pc.reserve(submitted_cbs);
                        if !batch.overflow.is_empty() {
                            pc.append(&mut batch.overflow);
                        } else {
                            for slot in &mut batch.inline[..batch.inline_len] {
                                if let Some(item) = slot.take() {
                                    pc.push(item);
                                }
                            }
                        }
                        self.has_pending_cbs.store(true, Ordering::Release);
                        (had, pc.len())
                    };
                    GLOBAL_PENDING_CBS.fetch_add(submitted_cbs, Ordering::AcqRel);
                    if had_existing {
                        super::encstats::inc(&super::encstats::c::submit_coalesced);
                    }
                    self.commit_epoch.fetch_add(1, Ordering::AcqRel);
                    SUBMIT_EPOCH.fetch_add(1, Ordering::AcqRel);
                    let (next, _) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
                    self.timeline_state
                        .store(Self::pack_timeline_state(next, true), Ordering::Release);
                    if batch.any_ml_work || new_len >= MAX_COALESCED_CBS {
                        self.flush_pending_cbs_locked();
                    }
                }
                let total_logs = if !batch.pass_logs.is_empty() {
                    let mut pl = self.pending_pass_logs.lock().unwrap();
                    if !pl.is_empty() {
                        super::encstats::inc(&super::encstats::c::submit_coalesced);
                    }
                    pl.extend(batch.pass_logs);
                    let len = pl.len();
                    self.pending_log_count.store(len, Ordering::Release);
                    len
                } else {
                    self.pending_log_count.load(Ordering::Relaxed)
                };
                if total_logs >= 8 {
                    self.flush_timeline_locked()
                } else {
                    super::encstats::inc(&super::encstats::c::submit_nosig);
                    let (next, _) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
                    next - 1
                }
            }
        };
        if !cb_ptrs.is_empty() {
            self.retire_mesh_icbs_for_ptrs(&cb_ptrs);
        }
        self.collect_async();
        ExecuteResult { submitted_cbs, timeline_value: out }
    }

    /// Submit closed D3D12 command lists, handling submit coalescing, pass-timing
    /// log collection, and `MTL4CommandBuffer` submission inside `d3d12-metal`.
    pub fn execute_command_lists(&self, lists: &mut [&mut super::cmdlist::CommandList]) -> ExecuteResult {
        self.execute_command_lists_at(lists, capture_submit_timestamp())
    }

    /// Same as [`Self::execute_command_lists`], using a caller-captured `t_ecl`
    /// timestamp from [`capture_submit_timestamp`] (for async command-stream threads).
    pub fn execute_command_lists_at(
        &self,
        lists: &mut [&mut super::cmdlist::CommandList],
        t_ecl: u64,
    ) -> ExecuteResult {
        let mut batch = self.begin_prepare(t_ecl);
        for (i, list) in lists.iter_mut().enumerate() {
            self.prepare_list(&mut batch, list, i);
        }
        self.submit_prepared(batch)
    }

    /// Value the timeline will reach once everything submitted so far is done.
    ///
    /// Lock-free and non-flushing: when a coalesced batch is pending without a
    /// timeline signal yet (`unsignaled == true`), returns the upcoming signal
    /// value (`next`) so [`Self::defer_release`] keeps retired objects alive until
    /// that coalesced batch completes on GPU without defeating `AW2_SUBMIT_COALESCE`.
    /// Callers that wait on `self.timeline` (`wait_idle`, cross-queue `Swapchain`
    /// sync) MUST call [`Self::flush_timeline`] so the GPU signal is emitted.
    #[inline]
    pub fn last_submitted(&self) -> u64 {
        let (next, unsignaled) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
        if unsignaled {
            next
        } else {
            next - 1
        }
    }

    /// Flush any pending coalesced command buffers, pass logs, or deferred fence
    /// signals onto `self.q` and return the timeline value guaranteed to be
    /// signaled once all submitted work completes on GPU.
    pub fn flush_timeline(&self) -> u64 {
        if self.has_pending_cbs.load(Ordering::Acquire)
            || self.is_unsignaled()
            || self.pending_log_count.load(Ordering::Acquire) > 0
            || self.has_deferred_signals.load(Ordering::Acquire)
        {
            let v = {
                let _g = self.submit.lock().unwrap();
                self.flush_timeline_locked()
            };
            self.collect_async();
            return v;
        }
        let (next, _) = Self::unpack_timeline_state(self.timeline_state.load(Ordering::Acquire));
        next - 1
    }

    pub fn signal(&self, f: &Fence, v: u64) {
        use std::sync::atomic::Ordering as O;
        self.ensure_registered_active();
        let me = self as *const Queue as usize;
        // If `f` had a deferred signal on a DIFFERENT queue, flush that queue BEFORE
        // acquiring `self.submit` so cross-queue signals remain strictly ordered.
        let prev_pending_q = f.pending_sig_queue.load(O::Acquire);
        if prev_pending_q != 0 && prev_pending_q != me {
            Self::flush_fence_signals_by_addr(prev_pending_q);
        }
        let _g = self.submit.lock().unwrap();
        if self.kind != ListType::Direct && self.pending_log_count.load(O::Relaxed) > 0 {
            let _ = self.flush_timeline_locked();
        }
        if f.sig_queue.swap(me, O::AcqRel) != me {
            f.sig_value.store(v, O::Release);
        } else {
            f.sig_value.fetch_max(v, O::AcqRel);
        }
        let has_unmet_wait = {
            let wt = f.wait_target.load(O::SeqCst);
            wt != 0 && f.emitted_sig_value.load(O::Acquire) < wt
        };
        if self.kind == ListType::Direct
            && !self.is_swapchain_internal
            && self.opt_fence_signal_coalesce.load(O::Relaxed)
            && !has_unmet_wait
        {
            let ev_key = f.ev_key();
            let f_ptr = f as *const Fence as usize;
            let cur_epoch = self.commit_epoch.load(O::Acquire);
            let now_ns = now_uptime_ns();
            let mut ds = self.deferred_signals.lock().unwrap();
            if let Some(existing) = ds.iter_mut().find(|e| e.ev_key == ev_key) {
                existing.val = existing.val.max(v);
                existing.fence_ptr = f_ptr;
                super::encstats::inc(&super::encstats::c::fence_sig_coalesced);
                let max_v = existing.val;
                drop(ds);
                f.pending_sig_value.store(max_v, O::Release);
                f.pending_sig_queue.store(me, O::SeqCst);
            } else {
                ds.push(DeferredFenceSig {
                    ev_key,
                    ev: f.ev.clone(),
                    fence_ptr: f_ptr,
                    val: v,
                    enqueued_epoch: cur_epoch,
                });
                self.set_has_deferred_signals(true);
                drop(ds);
                f.pending_sig_ns.store(now_ns, O::Release);
                f.poll_epoch.store(cur_epoch, O::Release);
                f.poll_count.store(0, O::Release);
                f.pending_sig_value.store(v, O::Release);
                f.pending_sig_queue.store(me, O::SeqCst);
            }
            // Dekker `SeqCst` handshake with `Queue::wait_with`, `Fence::wait_cpu`,
            // and `Fence::on_completion`: if a concurrent GPU or CPU waiter
            // published `wait_target` while we were deferring, flush immediately so
            // it can never deadlock even with `AW2_D3D_CS=0`.
            let wt_after = f.wait_target.load(O::SeqCst);
            if wt_after != 0 && f.emitted_sig_value.load(O::Acquire) < wt_after {
                self.flush_fence_signals_locked();
            }
            return;
        }
        let cbs = self.take_pending_cbs_locked();
        let mut sigs = self.take_deferred_fence_signals_locked();
        f.emitted_sig_value.fetch_max(v, O::AcqRel);
        sigs.push((f.ev.clone(), v));
        self.dispatch_commit_and_signals_locked(cbs, sigs);
    }

    /// `ID3D12CommandQueue::Wait`. A wait on a value this queue itself
    /// already signalled is dropped: work after it starts in a later command
    /// list, whose first encoder waits for all earlier work on the queue
    /// (the list-start full queue barrier, `FullWhy::ListStart`), which is
    /// what the event round trip enforced. Removing that barrier requires
    /// restoring the wait.
    pub fn wait(&self, f: &Fence, v: u64) {
        self.wait_with(f, v, super::cmdlist::opt_list_barrier_tight_env());
    }

    pub(crate) fn wait_with(&self, f: &Fence, v: u64, list_barrier_tight: bool) {
        use std::sync::atomic::Ordering as O;
        let me = self as *const Queue as usize;
        let same_q_elided = !list_barrier_tight
            && f.sig_queue.load(O::Acquire) == me
            && f.sig_value.load(O::Acquire) >= v;
        if !same_q_elided && f.completed_raw() < v {
            // Publish the pending GPU wait before reading `pending_sig_queue`
            // (Dekker `SeqCst` handshake with `Queue::signal`) so a producer
            // signaling `f` concurrently or later flushes eagerly instead of
            // deferring behind this wait (including when `AW2_D3D_CS=0`).
            f.wait_target.fetch_max(v, O::SeqCst);
        }
        // Cross-queue wait (or `AW2_LIST_BARRIER_TIGHT=1` same-queue event wait):
        // flush `sig_q`'s deferred fence signals BEFORE acquiring `self.submit` so we
        // never lock `sig_q.submit` inside `self.submit` and `f.ev` is guaranteed to be
        // signaled on `sig_q` prior to `self.q.waitForEvent_value(&f.ev, v)`.
        let pending_q = f.pending_sig_queue.load(O::SeqCst);
        if pending_q != 0
            && (pending_q != me || list_barrier_tight)
            && f.pending_sig_value.load(O::Acquire) >= v
            && f.completed_raw() < v
        {
            Self::flush_fence_signals_by_addr(pending_q);
        }
        let _g = self.submit.lock().unwrap();
        if !list_barrier_tight
            && f.sig_queue.load(O::Acquire) == me
            && f.sig_value.load(O::Acquire) >= v
        {
            super::encstats::inc(&super::encstats::c::qw_elided);
            return;
        }
        super::encstats::inc(&super::encstats::c::qw_event);
        let kind = if f.completed_raw() >= v {
            1
        } else if f.sig_queue.load(O::Acquire) != 0 {
            2
        } else {
            3
        };
        self.flush_pending_cbs_locked();
        self.flush_fence_signals_locked();
        self.pending_wait.fetch_max(kind, O::Relaxed);
        self.dispatch_wait_locked(&f.ev, v);
    }

    /// Keep `obj` alive until all work submitted so far has completed.
    pub fn defer_release(&self, obj: Deferred) {
        let v = self.last_submitted();
        if self.timeline.completed() >= v {
            drop(obj);
            return;
        }
        let mut d = self.deferred.lock().unwrap();
        d.push((v, obj));
        self.has_deferred.store(true, Ordering::Release);
    }

    /// Move deferred objects whose submissions have completed into `out` (dropping outside locks).
    fn drain_completed_into(&self, out: &mut Vec<Deferred>) {
        if !self.has_deferred.load(Ordering::Acquire) {
            return;
        }
        let done = self.timeline.completed();
        let mut d = self.deferred.lock().unwrap();
        if d.is_empty() {
            self.has_deferred.store(false, Ordering::Release);
            return;
        }
        if d.iter().any(|(v, _)| *v <= done) {
            let prev = std::mem::take(&mut *d);
            for (v, obj) in prev {
                if v <= done {
                    out.push(obj);
                } else {
                    d.push((v, obj));
                }
            }
            if d.is_empty() {
                self.has_deferred.store(false, Ordering::Release);
            }
        }
    }

    /// Drop deferred objects whose submissions have completed synchronously on the
    /// calling thread (used by `wait_idle`, iOS Jetsam reclamation, and tests).
    pub fn collect(&self) {
        if !self.has_deferred.load(Ordering::Acquire) {
            return;
        }
        let mut expired = Vec::new();
        self.drain_completed_into(&mut expired);
        drop(expired);
    }

    /// Reap completed deferred objects asynchronously on a background utility thread
    /// so Metal resource/heap deallocation never stalls `d3d12-metal-cs`.
    pub fn collect_async(&self) {
        if !self.has_deferred.load(Ordering::Acquire) {
            return;
        }
        let mut expired = Vec::new();
        self.drain_completed_into(&mut expired);
        if !expired.is_empty() {
            reap_deferred_async(expired);
        }
    }

    /// Block until the queue is idle.
    pub fn wait_idle(&self) {
        let v = self.flush_timeline();
        self.timeline.wait_cpu(v, u64::MAX);
        self.collect();
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        let addr = self as *const Queue as usize;
        active_queues().lock().unwrap().retain(|&p| p != addr);
        if self.has_pending_cbs.load(Ordering::Acquire)
            || self.is_unsignaled()
            || self.pending_log_count.load(Ordering::Acquire) > 0
            || self.has_deferred_signals.load(Ordering::Acquire)
        {
            let _g = self.submit.lock().unwrap();
            let _ = self.flush_timeline_locked();
        }
        if let Some(cw) = &self.commit_worker {
            cw.shutdown_and_join();
        }
        // Ensure any GPU work using deferred resources has completed before
        // `self.deferred` drops (even if no submit or `wait_idle` followed a
        // coalesced `submit_d3d_lists` + `defer_release`).
        let max_v = self
            .deferred
            .get_mut()
            .unwrap()
            .iter()
            .map(|(v, _)| *v)
            .max()
            .unwrap_or(0);
        if max_v > 0 && self.timeline.completed() < max_v {
            self.timeline.wait_cpu(max_v, u64::MAX);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_barrier_tight_restores_same_queue_fence_wait_events() {
        if super::super::device().is_none() {
            return;
        }
        let q = Queue::new(ListType::Direct, "g32 same-queue wait test").unwrap();
        let f = Fence::new(0).unwrap();
        q.signal(&f, 7);

        let elided0 = super::super::encstats::c::qw_elided.load(Ordering::Relaxed);
        let event0 = super::super::encstats::c::qw_event.load(Ordering::Relaxed);

        // Legacy list-start barrier mode elides same-queue waits already signaled.
        q.wait_with(&f, 7, false);
        assert!(super::super::encstats::c::qw_elided.load(Ordering::Relaxed) > elided0);
        assert_eq!(super::super::encstats::c::qw_event.load(Ordering::Relaxed), event0);

        // `AW2_LIST_BARRIER_TIGHT=1` must NOT elide same-queue fence waits because
        // `FullWhy::ListStart` (`All -> All`) is replaced by stage-masked sync.
        q.wait_with(&f, 7, true);
        assert!(super::super::encstats::c::qw_event.load(Ordering::Relaxed) > event0);
        q.submit(&[]);
        q.wait_idle();
        f.wait_cpu(7, u64::MAX);
        assert_eq!(f.completed(), 7);
    }

    #[test]
    fn submit_coalesce_batches_signals_and_pass_logs_until_demand_on_gpu() {
        use super::super::cmdlist::{Barrier, CommandAllocator, CommandList};
        use super::super::passtiming::PassLog;
        use super::super::resource::Resource;
        use crate::types::{dim, heap_type, HeapProperties, ResourceDesc};
        use std::sync::Arc;

        if super::super::device().is_none() {
            return;
        }
        let _stats_guard = super::super::encstats::test_lock();
        let make_buf = |ty: u32| {
            let d = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: 64,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            let hp = HeapProperties { Type: ty, ..Default::default() };
            Resource::new_committed(&d, &hp, MTLTextureUsage::empty(), "g34 queue test buf").unwrap()
        };

        let q = Queue::new(ListType::Direct, "g34 submit coalesce test").unwrap();
        q.set_submit_coalesce(true);

        let a_empty = CommandAllocator::new(ListType::Direct).unwrap();
        let a0 = CommandAllocator::new(ListType::Direct).unwrap();
        let a1 = CommandAllocator::new(ListType::Direct).unwrap();
        let a2 = CommandAllocator::new(ListType::Direct).unwrap();

        let mut l_empty = CommandList::new(ListType::Direct).unwrap();
        let mut l0 = CommandList::new(ListType::Direct).unwrap();
        let mut l1 = CommandList::new(ListType::Direct).unwrap();
        let mut l2 = CommandList::new(ListType::Direct).unwrap();

        let up = make_buf(heap_type::UPLOAD);
        let mid0 = make_buf(heap_type::DEFAULT);
        let mid1 = make_buf(heap_type::DEFAULT);
        let rb = make_buf(heap_type::READBACK);
        unsafe {
            *(up.buffer().unwrap().contents().as_ptr() as *mut u32) = 0xcafe_d00d;
        }

        // 1) Zero-GPU-work command list (`L0`–`L2` barrier/state-only lists) MUST report
        // `!has_gpu_work_recorded()` and elide submission when `PORT_EMPTY_ENC_ELIDE` is ON.
        l_empty.begin(&a_empty).unwrap();
        l_empty.barrier_list(&[Barrier::Uav(0)]);
        l_empty.close().unwrap();
        assert_eq!(l_empty.has_commands(), !l_empty.opts.opt_empty_enc_elide);
        assert!(!l_empty.has_gpu_work_recorded());

        // 2) Record 3 real-GPU dependent Blit submissions (`up -> mid0 -> mid1 -> rb`).
        l0.begin(&a0).unwrap();
        l0.copy_buffer(&mid0, 0, &up, 0, 16).unwrap();
        l0.close().unwrap();
        assert!(l0.has_gpu_work_recorded());

        l1.begin(&a1).unwrap();
        l1.barrier_list(&[Barrier::Transition(&mid0 as *const Resource as usize, 0x400, 0x800)]);
        l1.copy_buffer(&mid1, 0, &mid0, 0, 16).unwrap();
        l1.close().unwrap();
        assert!(l1.has_gpu_work_recorded());

        l2.begin(&a2).unwrap();
        l2.barrier_list(&[Barrier::Transition(&mid1 as *const Resource as usize, 0x400, 0x800)]);
        l2.copy_buffer(&rb, 0, &mid1, 0, 16).unwrap();
        l2.close().unwrap();
        assert!(l2.has_gpu_work_recorded());

        let nosig0 = super::super::encstats::c::submit_nosig.load(Ordering::Relaxed);
        let coal0 = super::super::encstats::c::submit_coalesced.load(Ordering::Relaxed);

        let log_vec = || PassLog::acquire().into_iter().collect::<Vec<_>>();
        let logs0 = log_vec();
        let logs1 = log_vec();
        let logs2 = log_vec();
        let had_logs = !logs0.is_empty() && !logs1.is_empty() && !logs2.is_empty();

        // Empty D3D submission + 3 real-GPU D3D submissions MUST NOT emit `signalEvent_value` yet!
        assert_eq!(q.submit_d3d_lists(&[], Vec::new()), 0);
        assert_eq!(q.submit_d3d_lists(&[&l0.cb], logs0), 0);
        assert_eq!(q.submit_d3d_lists(&[&l1.cb], logs1), 0);
        assert_eq!(q.submit_d3d_lists(&[&l2.cb], logs2), 0);

        let (next_val, unsig) = Queue::unpack_timeline_state(q.timeline_state.load(Ordering::Acquire));
        assert_eq!(
            next_val,
            1,
            "lazy timeline coalescing must emit 0 timeline signals during burst submit"
        );
        assert!(unsig);
        assert!(super::super::encstats::c::submit_nosig.load(Ordering::Relaxed) >= nosig0 + 4);
        if had_logs {
            assert_eq!(q.pending_log_count.load(Ordering::Acquire), 3);
            assert!(super::super::encstats::c::submit_coalesced.load(Ordering::Relaxed) >= coal0 + 2);
        }

        // `defer_release` / `last_submitted` MUST NOT flush the coalesced batch:
        // it parks the deferred object on the upcoming coalesced signal value (`1`)
        // while keeping `unsignaled == true` and 0 timeline signals emitted.
        struct DropTracker(Arc<AtomicBool>);
        impl Drop for DropTracker {
            fn drop(&mut self) {
                self.0.store(true, Ordering::Release);
            }
        }
        let dropped = Arc::new(AtomicBool::new(false));
        assert_eq!(q.last_submitted(), 1, "last_submitted must return upcoming coalesced signal value without flushing");
        q.defer_release(Box::new(DropTracker(dropped.clone())));
        let (next_after_defer, unsig_after_defer) =
            Queue::unpack_timeline_state(q.timeline_state.load(Ordering::Acquire));
        assert_eq!(next_after_defer, 1, "defer_release must not force an immediate timeline flush");
        assert!(unsig_after_defer, "coalesced batch must stay unsignaled after defer_release");
        assert!(!dropped.load(Ordering::Acquire), "deferred object must remain live until coalesced batch completes");
        if had_logs {
            assert_eq!(q.pending_log_count.load(Ordering::Acquire), 3);
        }

        // Demand (`flush_timeline` / `wait_idle`) flushes ALL 3 submissions onto a SINGLE timeline signal (`v == 1`).
        assert_eq!(q.flush_timeline(), 1);
        let (next_after_flush, unsig_after_flush) =
            Queue::unpack_timeline_state(q.timeline_state.load(Ordering::Acquire));
        assert_eq!(next_after_flush, 2, "flush must advance timeline by exactly 1 tick");
        assert!(!unsig_after_flush);
        assert_eq!(q.pending_log_count.load(Ordering::Acquire), 0);

        q.wait_idle();
        assert_eq!(q.timeline.completed(), 1);
        assert!(dropped.load(Ordering::Acquire));
        let word = unsafe { *(rb.buffer().unwrap().contents().as_ptr() as *const u32) };
        assert_eq!(word, 0xcafe_d00d);

        // Direct `Queue::submit` (used by `sq`/`pq`/internal callers) MUST ALWAYS signal eagerly (`v >= 1`),
        // even when `opt_submit_coalesce` is active on a `ListType::Direct` queue!
        assert_eq!(q.submit(&[]), 2);
        q.timeline.wait_cpu(2, u64::MAX);
        assert_eq!(q.timeline.completed(), 2);

        // Verify `Queue::drop` flushes an unsignaled coalesced batch and waits for GPU completion
        // before dropping deferred resources even when no further submit or `wait_idle` happens.
        let dropped_on_drop = Arc::new(AtomicBool::new(false));
        {
            let q_drop = Queue::new(ListType::Direct, "g35 drop flush test").unwrap();
            q_drop.set_submit_coalesce(true);
            let a_d = CommandAllocator::new(ListType::Direct).unwrap();
            let mut l_d = CommandList::new(ListType::Direct).unwrap();
            l_d.begin(&a_d).unwrap();
            l_d.copy_buffer(&rb, 0, &up, 0, 16).unwrap();
            l_d.close().unwrap();
            assert_eq!(q_drop.submit_d3d_lists(&[&l_d.cb], Vec::new()), 0);
            assert!(q_drop.is_unsignaled());
            q_drop.defer_release(Box::new(DropTracker(dropped_on_drop.clone())));
            assert!(q_drop.is_unsignaled());
            assert!(!dropped_on_drop.load(Ordering::Acquire));
        }
        assert!(dropped_on_drop.load(Ordering::Acquire), "Queue::drop must flush & wait before releasing deferred items");
    }

    #[test]
    fn fence_signal_coalesce_batches_q0_signals_flushes_cross_queue_cpu_and_spin_polls_on_gpu() {
        use super::super::cmdlist::{Barrier, CommandAllocator, CommandList};
        use super::super::resource::Resource;
        use crate::types::{dim, heap_type, HeapProperties, ResourceDesc};

        if super::super::device().is_none() {
            return;
        }
        if std::env::var_os("PORT_FENCE_SIGNAL_COALESCE").is_none()
            && std::env::var_os("AW2_FENCE_SIGNAL_COALESCE").is_none()
        {
            assert!(fence_signal_coalesce(), "PORT_FENCE_SIGNAL_COALESCE must default ON");
        }
        let _stats_guard = super::super::encstats::test_lock();
        let make_buf = |ty: u32| {
            let d = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: 64,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            let hp = HeapProperties { Type: ty, ..Default::default() };
            Resource::new_committed(&d, &hp, MTLTextureUsage::empty(), "g37 fence coalesce buf").unwrap()
        };

        let q0 = Queue::new(ListType::Direct, "D3D12 Direct queue").unwrap();
        q0.set_submit_coalesce(true);
        q0.set_fence_signal_coalesce(true);

        // Internal swapchain queues MUST reject fence signal coalescing even if requested!
        let sq = Queue::new(ListType::Direct, "swapchain staging").unwrap();
        sq.set_fence_signal_coalesce(true);
        assert!(!sq.opt_fence_signal_coalesce.load(Ordering::Acquire));

        let q_copy = Queue::new(ListType::Copy, "D3D12 Copy queue").unwrap();

        let up = make_buf(heap_type::UPLOAD);
        let mid0 = make_buf(heap_type::DEFAULT);
        let mid1 = make_buf(heap_type::DEFAULT);
        let rb_copy = make_buf(heap_type::READBACK);
        let rb_spin = make_buf(heap_type::READBACK);
        unsafe {
            *(up.buffer().unwrap().contents().as_ptr() as *mut u32) = 0xfeed_c0de;
        }

        let a0 = CommandAllocator::new(ListType::Direct).unwrap();
        let a1 = CommandAllocator::new(ListType::Direct).unwrap();
        let a2 = CommandAllocator::new(ListType::Direct).unwrap();
        let a3 = CommandAllocator::new(ListType::Direct).unwrap();
        let ac = CommandAllocator::new(ListType::Copy).unwrap();

        let mut l0 = CommandList::new(ListType::Direct).unwrap();
        let mut l1 = CommandList::new(ListType::Direct).unwrap();
        let mut l2 = CommandList::new(ListType::Direct).unwrap();
        let mut l3 = CommandList::new(ListType::Direct).unwrap();
        let mut lc = CommandList::new(ListType::Copy).unwrap();

        // (a) Multiple `submit_d3d_lists` + `q0.signal(&f, 1..4)` coalesce (`fence_sig_coalesced > 0`)
        //     while same-queue `q0.wait(&f, 3)` still elides via `qw_elided`!
        l0.begin(&a0).unwrap();
        l0.copy_buffer(&mid0, 0, &up, 0, 16).unwrap();
        l0.close().unwrap();

        l1.begin(&a1).unwrap();
        l1.barrier_list(&[Barrier::Transition(&mid0 as *const Resource as usize, 0x400, 0x800)]);
        l1.copy_buffer(&mid1, 0, &mid0, 0, 16).unwrap();
        l1.close().unwrap();

        l2.begin(&a2).unwrap();
        l2.barrier_list(&[Barrier::Transition(&mid1 as *const Resource as usize, 0x400, 0x800)]);
        l2.copy_buffer(&rb_spin, 0, &mid1, 0, 16).unwrap();
        l2.close().unwrap();

        l3.begin(&a3).unwrap();
        l3.barrier_list(&[Barrier::Uav(0)]);
        l3.close().unwrap();

        let f = Fence::new(0).unwrap();
        let sig_coal0 = super::super::encstats::c::fence_sig_coalesced.load(Ordering::Relaxed);
        let elided0 = super::super::encstats::c::qw_elided.load(Ordering::Relaxed);

        q0.submit_d3d_lists(&[&l0.cb], Vec::new());
        q0.signal(&f, 1);
        q0.submit_d3d_lists(&[&l1.cb], Vec::new());
        q0.signal(&f, 2);
        // Cross-queue copy (`q_copy.wait(&f, 2)`) MUST eagerly flush `q0`'s pending `>= 2` signal!
        // First verify `f` is currently buffered in `q0` (`has_deferred_signals == true`, `fence_sig_coalesced >= 1`).
        assert!(q0.has_deferred_signals.load(Ordering::Acquire));
        assert!(super::super::encstats::c::fence_sig_coalesced.load(Ordering::Relaxed) >= sig_coal0 + 1);

        // (b) Cross-queue `q_copy.wait(&f, 2)` eagerly flushes `q0`'s signal before `q_copy` waits
        //     and transfers exact GPU bytes `mid1 -> rb_copy`!
        lc.begin(&ac).unwrap();
        lc.copy_buffer(&rb_copy, 0, &mid1, 0, 16).unwrap();
        lc.close().unwrap();
        q_copy.wait(&f, 2);
        assert!(!q0.has_deferred_signals.load(Ordering::Acquire), "cross-queue wait must eagerly flush q0 deferred signals");
        q_copy.submit(&[&lc.cb]);
        q_copy.wait_idle();
        let copy_word = unsafe { *(rb_copy.buffer().unwrap().contents().as_ptr() as *const u32) };
        assert_eq!(copy_word, 0xfeed_c0de);

        // Continue with signals `3` and `4` on `f`, plus same-queue `q0.wait(&f, 3)` (`qw_elided`++):
        let sig_coal1 = super::super::encstats::c::fence_sig_coalesced.load(Ordering::Relaxed);
        q0.submit_d3d_lists(&[&l2.cb], Vec::new());
        q0.signal(&f, 3);
        q0.wait_with(&f, 3, false);
        assert!(super::super::encstats::c::qw_elided.load(Ordering::Relaxed) > elided0, "same-queue wait must still elide via qw_elided");
        q0.submit_d3d_lists(&[&l3.cb], Vec::new());
        q0.signal(&f, 4);
        assert!(super::super::encstats::c::fence_sig_coalesced.load(Ordering::Relaxed) >= sig_coal1 + 1);
        assert!(q0.has_deferred_signals.load(Ordering::Acquire));

        // (c) `f_cpu.signal_cpu(10)` cancels stale deferred `<= 10` GPU signal so a subsequent flush
        //     NEVER regresses `f_cpu.completed()` backward from `10`!
        let f_cpu = Fence::new(0).unwrap();
        q0.signal(&f_cpu, 5);
        assert_eq!(f_cpu.pending_sig_value.load(Ordering::Acquire), 5);
        f_cpu.signal_cpu(10);
        assert_eq!(f_cpu.pending_sig_queue.load(Ordering::Acquire), 0);
        assert_eq!(f_cpu.completed(), 10);

        // (d) `Fence::completed()` spin-poll loop (`for _ in 0..16 { if f.completed() >= 4 { break; } }`)
        //     keeps `< 8` mid-frame polls buffered, then flushes automatically at poll 8 and reaches `4`
        //     without calling `wait_cpu` or `last_submitted`!
        for _ in 0..4 {
            let _ = f.completed();
        }
        assert!(
            q0.has_deferred_signals.load(Ordering::Acquire),
            "< 8 polls at same commit epoch must NOT break coalescing"
        );
        let mut reached = false;
        for _ in 0..16 {
            if f.completed() >= 4 {
                reached = true;
                break;
            }
        }
        assert!(!q0.has_deferred_signals.load(Ordering::Acquire), "spin-poll loop must automatically flush deferred signals");
        if !reached {
            // In the rare event GPU execution took > 1ms under heavy system load, poll without wait_cpu/last_submitted.
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
            while std::time::Instant::now() < deadline {
                if f.completed() >= 4 {
                    reached = true;
                    break;
                }
                std::thread::sleep(std::time::Duration::from_micros(100));
            }
        }
        assert!(reached, "Fence::completed() spin-poll must reach 4 without wait_cpu or last_submitted");
        let spin_word = unsafe { *(rb_spin.buffer().unwrap().contents().as_ptr() as *const u32) };
        assert_eq!(spin_word, 0xfeed_c0de);

        // Verify `f_cpu` remained at `10` (never regressed to `5`) even after `q0` flushed and idled!
        q0.wait_idle();
        assert_eq!(f_cpu.completed(), 10);
    }

    #[test]
    fn execute_command_lists_coalesces_empty_and_runs_gpu_copy() {
        use super::super::cmdlist::{Barrier, CommandAllocator, CommandList};
        use super::super::resource::Resource;
        use crate::types::{dim, heap_type, HeapProperties, ResourceDesc};

        if super::super::device().is_none() {
            return;
        }
        let _stats_guard = super::super::encstats::test_lock();
        let make_buf = |ty: u32| {
            let d = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: 64,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            let hp = HeapProperties { Type: ty, ..Default::default() };
            Resource::new_committed_default(&d, &hp, "execute_command_lists test buf").unwrap()
        };

        let q = Queue::new(ListType::Direct, "execute_command_lists test queue").unwrap();
        q.set_submit_coalesce(true);

        let up = make_buf(heap_type::UPLOAD);
        let rb = make_buf(heap_type::READBACK);
        assert!(up.allocated_bytes() >= 64);
        unsafe {
            *(up.cpu_ptr().unwrap() as *mut u32) = 0x1357_2468;
        }

        let a_empty = CommandAllocator::new(ListType::Direct).unwrap();
        let a_copy = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l_empty = CommandList::new(ListType::Direct).unwrap();
        let mut l_copy = CommandList::new(ListType::Direct).unwrap();

        l_empty.begin(&a_empty).unwrap();
        l_empty.barrier_list(&[Barrier::Uav(0)]);
        l_empty.close().unwrap();

        l_copy.begin(&a_copy).unwrap();
        l_copy.copy_buffer(&rb, 0, &up, 0, 16).unwrap();
        l_copy.close().unwrap();

        let res = q.execute_command_lists(&mut [&mut l_empty, &mut l_copy]);
        assert_eq!(res.submitted_cbs, 1, "empty barrier-only list should be coalesced");
        q.wait_idle();
        let word = unsafe { *(rb.cpu_ptr().unwrap() as *const u32) };
        assert_eq!(word, 0x1357_2468);
    }

    #[test]
    fn submit_d3d_lists_coalesces_adjacent_commits_and_preserves_fence_signal_wait_and_cb_reuse() {
        use super::super::cmdlist::{Barrier, CommandAllocator, CommandList};
        use super::super::resource::Resource;
        use crate::types::{dim, heap_type, HeapProperties, ResourceDesc};

        if super::super::device().is_none() {
            return;
        }
        let _stats_guard = super::super::encstats::test_lock();
        let make_buf = |ty: u32| {
            let d = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: 64,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            let hp = HeapProperties { Type: ty, ..Default::default() };
            Resource::new_committed_default(&d, &hp, "submit coalesce cb test buf").unwrap()
        };

        let q = Queue::new(ListType::Direct, "submit coalesce cb test queue").unwrap();
        q.set_submit_coalesce(true);
        q.set_fence_signal_coalesce(false);

        let up0 = make_buf(heap_type::UPLOAD);
        let up1 = make_buf(heap_type::UPLOAD);
        let mid0 = make_buf(heap_type::DEFAULT);
        let mid1 = make_buf(heap_type::DEFAULT);
        let rb0 = make_buf(heap_type::READBACK);
        let rb1 = make_buf(heap_type::READBACK);
        let rb_reuse = make_buf(heap_type::READBACK);
        unsafe {
            *(up0.cpu_ptr().unwrap() as *mut u32) = 0x1122_3344;
            *(up1.cpu_ptr().unwrap() as *mut u32) = 0xDEAD_BEEF;
        }

        let a0 = CommandAllocator::new(ListType::Direct).unwrap();
        let a1 = CommandAllocator::new(ListType::Direct).unwrap();
        let a2 = CommandAllocator::new(ListType::Direct).unwrap();
        let a3 = CommandAllocator::new(ListType::Direct).unwrap();
        let a4 = CommandAllocator::new(ListType::Direct).unwrap();
        let a5 = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l0 = CommandList::new(ListType::Direct).unwrap();
        let mut l1 = CommandList::new(ListType::Direct).unwrap();
        let mut l2 = CommandList::new(ListType::Direct).unwrap();
        let mut l3 = CommandList::new(ListType::Direct).unwrap();

        l0.begin(&a0).unwrap();
        l0.copy_buffer(&mid0, 0, &up0, 0, 16).unwrap();
        l0.close().unwrap();

        l1.begin(&a1).unwrap();
        l1.barrier_list(&[Barrier::Transition(&mid0 as *const Resource as usize, 0x400, 0x800)]);
        l1.copy_buffer(&mid1, 0, &mid0, 0, 16).unwrap();
        l1.close().unwrap();

        l2.begin(&a2).unwrap();
        l2.barrier_list(&[Barrier::Transition(&mid1 as *const Resource as usize, 0x400, 0x800)]);
        l2.copy_buffer(&rb0, 0, &mid1, 0, 16).unwrap();
        l2.close().unwrap();

        l3.begin(&a3).unwrap();
        l3.copy_buffer(&rb1, 0, &up1, 0, 16).unwrap();
        l3.close().unwrap();

        // Snapshot commit counters so we can count exact Metal `commit_cbs` calls.
        let calls0 = TEST_COMMIT_CALLS.load(Ordering::Relaxed);
        let cbs0 = TEST_COMMIT_CBS.load(Ordering::Relaxed);

        // 1) Three adjacent `submit_d3d_lists` calls buffer into `pending_cbs` with ZERO Metal commits yet.
        assert_eq!(q.submit_d3d_lists(&[&l0.cb], Vec::new()), 0);
        assert_eq!(q.submit_d3d_lists(&[&l1.cb], Vec::new()), 0);
        assert_eq!(q.submit_d3d_lists(&[&l2.cb], Vec::new()), 0);
        assert!(q.has_pending_cbs.load(Ordering::Acquire));
        assert_eq!(q.pending_cbs.lock().unwrap().len(), 3);
        assert_eq!(
            TEST_COMMIT_CALLS.load(Ordering::Relaxed) - calls0,
            0,
            "adjacent submits must stay buffered until boundary"
        );
        assert_eq!(TEST_COMMIT_CBS.load(Ordering::Relaxed) - cbs0, 0);

        // 2) `q.signal(&f, 1)` MUST flush all 3 buffered CBs in a SINGLE Metal commit BEFORE signaling.
        let f = Fence::new(0).unwrap();
        q.signal(&f, 1);
        assert!(!q.has_pending_cbs.load(Ordering::Acquire));
        assert_eq!(
            TEST_COMMIT_CALLS.load(Ordering::Relaxed) - calls0,
            1,
            "3 adjacent submits must coalesce into 1 Metal commit"
        );
        assert_eq!(TEST_COMMIT_CBS.load(Ordering::Relaxed) - cbs0, 3);

        // Wait on fence `f == 1` — `rb0` MUST already contain the result of `l0 -> l1 -> l2`.
        f.wait_cpu(1, u64::MAX);
        assert_eq!(unsafe { *(rb0.cpu_ptr().unwrap() as *const u32) }, 0x1122_3344);

        // 3) Same-queue elided `wait_with(&f, 1, false)` preserves buffering, whereas
        //    non-elided `wait_with(&f_ext, 1, false)` flushes BEFORE encoding `waitForEvent`.
        assert_eq!(q.submit_d3d_lists(&[&l3.cb], Vec::new()), 0);
        assert!(q.has_pending_cbs.load(Ordering::Acquire));
        q.wait_with(&f, 1, false);
        assert!(
            q.has_pending_cbs.load(Ordering::Acquire),
            "elided same-queue wait must not break submit coalescing"
        );
        let f_ext = Fence::new(1).unwrap();
        q.wait_with(&f_ext, 1, false);
        assert!(
            !q.has_pending_cbs.load(Ordering::Acquire),
            "non-elided fence wait must flush pending CBs before waiting"
        );

        // 4) Re-beginning a `CommandList` whose `cb` is still buffered in `pending_cbs`
        //    MUST flush `pending_cbs` before `beginCommandBufferWithAllocator` (no Metal error).
        l3.begin(&a4).unwrap();
        l3.copy_buffer(&rb1, 0, &up1, 0, 16).unwrap();
        l3.close().unwrap();
        assert_eq!(q.submit_d3d_lists(&[&l3.cb], Vec::new()), 0);
        assert!(q.has_pending_cbs.load(Ordering::Acquire));
        l3.begin(&a5).unwrap();
        assert!(
            !q.has_pending_cbs.load(Ordering::Acquire),
            "CommandList::begin on a buffered CB must flush pending_cbs first"
        );
        l3.copy_buffer(&rb_reuse, 0, &up1, 0, 16).unwrap();
        l3.close().unwrap();
        let _ = q.submit_d3d_lists(&[&l3.cb], Vec::new());
        q.wait_idle();
        assert_eq!(unsafe { *(rb1.cpu_ptr().unwrap() as *const u32) }, 0xDEAD_BEEF);
        assert_eq!(unsafe { *(rb_reuse.cpu_ptr().unwrap() as *const u32) }, 0xDEAD_BEEF);

        // 5) With `fence_signal_coalesce(true)` (default ON), interleaved
        //    `submit_d3d_lists` + `signal` calls on `q` coalesce BOTH command buffers
        //    and fence signals into ONE single Metal commit while preserving fence order.
        q.set_fence_signal_coalesce(true);
        let a6 = CommandAllocator::new(ListType::Direct).unwrap();
        let a7 = CommandAllocator::new(ListType::Direct).unwrap();
        let a8 = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l4 = CommandList::new(ListType::Direct).unwrap();
        let mut l5 = CommandList::new(ListType::Direct).unwrap();
        let mut l6 = CommandList::new(ListType::Direct).unwrap();
        let rb_sig = make_buf(heap_type::READBACK);
        let f_sig = Fence::new(0).unwrap();

        l4.begin(&a6).unwrap();
        l4.copy_buffer(&mid0, 0, &up1, 0, 16).unwrap();
        l4.close().unwrap();
        l5.begin(&a7).unwrap();
        l5.barrier_list(&[Barrier::Transition(&mid0 as *const Resource as usize, 0x400, 0x800)]);
        l5.copy_buffer(&rb_sig, 0, &mid0, 0, 16).unwrap();
        l5.close().unwrap();

        let calls_sig0 = TEST_COMMIT_CALLS.load(Ordering::Relaxed);
        let cbs_sig0 = TEST_COMMIT_CBS.load(Ordering::Relaxed);
        let tl_before = q.last_submitted();
        assert_eq!(q.submit_d3d_lists(&[&l4.cb], Vec::new()), tl_before);
        q.signal(&f_sig, 1);
        assert_eq!(q.submit_d3d_lists(&[&l5.cb], Vec::new()), tl_before);
        q.signal(&f_sig, 2);
        assert!(q.has_pending_cbs.load(Ordering::Acquire));
        assert!(q.has_deferred_signals.load(Ordering::Acquire));
        assert_eq!(TEST_COMMIT_CALLS.load(Ordering::Relaxed) - calls_sig0, 0);

        // CPU wait on `f_sig == 2` flushes both CBs in ONE Metal commit and signals `f_sig = 2`.
        f_sig.wait_cpu(2, u64::MAX);
        assert_eq!(
            TEST_COMMIT_CALLS.load(Ordering::Relaxed) - calls_sig0,
            1,
            "interleaved ECL+Signal pairs must coalesce into 1 Metal commit"
        );
        assert_eq!(TEST_COMMIT_CBS.load(Ordering::Relaxed) - cbs_sig0, 2);
        assert_eq!(unsafe { *(rb_sig.cpu_ptr().unwrap() as *const u32) }, 0xDEAD_BEEF);

        // 6) Resetting an unrelated free `CommandAllocator` MUST NOT flush `pending_cbs`,
        //    whereas resetting the allocator backing a buffered CB MUST flush before `reset()`.
        let a_free = CommandAllocator::new(ListType::Direct).unwrap();
        let a_used = CommandAllocator::new(ListType::Direct).unwrap();
        l6.begin(&a_used).unwrap();
        l6.copy_buffer(&rb_sig, 0, &up0, 0, 16).unwrap();
        l6.close().unwrap();
        let calls_alloc0 = TEST_COMMIT_CALLS.load(Ordering::Relaxed);
        let _ = q.execute_command_lists(&mut [&mut l6]);
        assert!(q.has_pending_cbs.load(Ordering::Acquire));
        a_free.reset();
        assert!(
            q.has_pending_cbs.load(Ordering::Acquire),
            "resetting an unrelated allocator must preserve pending_cbs coalescing"
        );
        assert_eq!(TEST_COMMIT_CALLS.load(Ordering::Relaxed) - calls_alloc0, 0);
        a_used.reset();
        assert!(
            !q.has_pending_cbs.load(Ordering::Acquire),
            "resetting the backing allocator must flush pending_cbs first"
        );
        assert_eq!(TEST_COMMIT_CALLS.load(Ordering::Relaxed) - calls_alloc0, 1);
        q.wait_idle();

        // 7) A command list flagged with `has_ml_work` coalesces with prior buffered CBs
        //    and flushes eagerly inside `execute_command_lists` instead of staying buffered.
        l6.begin(&a8).unwrap();
        l6.copy_buffer(&rb_sig, 0, &up0, 0, 16).unwrap();
        l6.has_ml_work = true;
        l6.close().unwrap();
        let calls_ml0 = TEST_COMMIT_CALLS.load(Ordering::Relaxed);
        let _ = q.execute_command_lists(&mut [&mut l6]);
        assert!(
            !q.has_pending_cbs.load(Ordering::Acquire),
            "ML command list must flush eagerly on ExecuteCommandLists"
        );
        assert_eq!(TEST_COMMIT_CALLS.load(Ordering::Relaxed) - calls_ml0, 1);
        q.wait_idle();
        assert_eq!(unsafe { *(rb_sig.cpu_ptr().unwrap() as *const u32) }, 0x1122_3344);
    }

    /// Regression test for cross-queue waits enqueued BEFORE their producer signal
    /// when fence-signal coalescing is ON and no CS worker thread is running
    /// (`AW2_D3D_CS=0` inline mode):
    /// `copy.Wait(f, v)` -> `copy.Signal(g, v)` -> `direct.Signal(f, v)` -> CPU wait / poll `g`.
    #[test]
    fn cross_queue_wait_before_signal_makes_progress_without_cs_thread_on_gpu() {
        use super::super::cmdlist::{CommandAllocator, CommandList};
        use super::super::resource::Resource;
        use crate::types::{dim, heap_type, HeapProperties, ResourceDesc};
        use std::sync::Arc;

        if super::super::device().is_none() {
            return;
        }
        let direct = Queue::new(ListType::Direct, "cross-q wait-before-sig direct").unwrap();
        let copy = Queue::new(ListType::Copy, "cross-q wait-before-sig copy").unwrap();
        direct.set_submit_coalesce(true);
        direct.set_fence_signal_coalesce(true);

        let f = Fence::new(0).unwrap();
        let g = Fence::new(0).unwrap();

        let make_buf = |ty: u32| -> Resource {
            let d = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: 256,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            let hp = HeapProperties { Type: ty, ..Default::default() };
            Resource::new_committed_default(&d, &hp, "cross-q wait-before-sig buf").unwrap()
        };
        let up = make_buf(heap_type::UPLOAD);
        let rb = make_buf(heap_type::READBACK);
        unsafe { *(up.cpu_ptr().unwrap() as *mut u32) = 0xCAFE_BABE };
        let a = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.begin(&a).unwrap();
        l.copy_buffer(&rb, 0, &up, 0, 16).unwrap();
        l.close().unwrap();

        // 1) CPU spin-poll on `g.completed()` after `copy.Wait(f, 1)` -> `copy.Signal(g, 1)`
        //    -> `direct.Execute([l])` -> `direct.Signal(f, 1)`.
        copy.wait_with(&f, 1, false);
        copy.signal(&g, 1);
        let _ = direct.execute_command_lists(&mut [&mut l]);
        direct.signal(&f, 1);
        assert!(
            !direct.has_pending_cbs.load(Ordering::Acquire)
                && !direct.has_deferred_signals.load(Ordering::Acquire),
            "direct.signal(f, 1) must flush eagerly when copy already waits on f >= 1"
        );
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while g.completed() < 1 {
            assert!(
                std::time::Instant::now() < deadline,
                "cross-queue wait-before-signal deadlocked on g.completed()"
            );
            std::thread::yield_now();
        }
        assert_eq!(unsafe { *(rb.cpu_ptr().unwrap() as *const u32) }, 0xCAFE_BABE);

        // 2) Blocking `g.wait_cpu(2, 5000)` on a wait-before-signal chain.
        copy.wait_with(&f, 2, false);
        copy.signal(&g, 2);
        direct.signal(&f, 2);
        assert!(
            g.wait_cpu(2, 5000),
            "cross-queue wait-before-signal deadlocked on g.wait_cpu(2)"
        );

        // 3) `g.on_completion(3, ...)` callback (`SetEventOnCompletion`) on a wait-before-signal chain.
        copy.wait_with(&f, 3, false);
        copy.signal(&g, 3);
        direct.signal(&f, 3);
        let fired = Arc::new(AtomicBool::new(false));
        let fired_c = fired.clone();
        g.on_completion(3, Box::new(move || fired_c.store(true, Ordering::Release)));
        let deadline3 = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while !fired.load(Ordering::Acquire) {
            assert!(
                std::time::Instant::now() < deadline3,
                "cross-queue wait-before-signal deadlocked on g.on_completion(3)"
            );
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        // 4) Once `f` has reached `wait_target` (3), subsequent signals without new
        //    external waiters resume coalescing normally.
        assert!(f.wait_cpu(3, 5000));
        direct.signal(&f, 4);
        direct.signal(&f, 5);
        assert!(
            direct.has_deferred_signals.load(Ordering::Acquire),
            "fence signals must resume coalescing once prior wait_target is satisfied"
        );
        assert!(f.wait_cpu(5, 5000));

        // 5) CPU `SetEventOnCompletion` (`on_completion`) registered BEFORE `direct.signal`
        //    must publish `wait_target` so the subsequent signal flushes eagerly with no CS thread.
        let fired6 = Arc::new(AtomicBool::new(false));
        let fired6_c = fired6.clone();
        f.on_completion(6, Box::new(move || fired6_c.store(true, Ordering::Release)));
        direct.signal(&f, 6);
        assert!(
            !direct.has_deferred_signals.load(Ordering::Acquire),
            "direct.signal(f, 6) must flush eagerly when on_completion(6) was registered first"
        );
        let deadline6 = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while !fired6.load(Ordering::Acquire) {
            assert!(
                std::time::Instant::now() < deadline6,
                "on_completion(6) registered before direct.signal(f, 6) deadlocked"
            );
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        // 6) Blocking `wait_cpu(7)` started on another thread BEFORE `direct.signal(f, 7)`
        //    must also publish `wait_target` so `direct.signal(f, 7)` flushes eagerly.
        let f_shared = Arc::new(Fence::new(0).unwrap());
        let f_bg = f_shared.clone();
        let entered = Arc::new(AtomicBool::new(false));
        let entered_bg = entered.clone();
        let waiter = std::thread::spawn(move || {
            entered_bg.store(true, Ordering::Release);
            f_bg.wait_cpu(7, 5000)
        });
        // Wait until background thread has entered `wait_cpu` and published `wait_target = 7`.
        let wait_pub = std::time::Instant::now() + std::time::Duration::from_secs(2);
        while !entered.load(Ordering::Acquire) || f_shared.wait_target.load(Ordering::Acquire) < 7 {
            assert!(std::time::Instant::now() < wait_pub);
            std::thread::yield_now();
        }
        direct.signal(&f_shared, 7);
        assert!(
            !direct.has_deferred_signals.load(Ordering::Acquire),
            "direct.signal(f_shared, 7) must flush eagerly when wait_cpu(7) is already waiting"
        );
        // Once `emitted_sig_value` reaches `wait_target` (7), subsequent signals resume coalescing.
        direct.signal(&f_shared, 8);
        assert!(
            direct.has_deferred_signals.load(Ordering::Acquire),
            "subsequent signals must resume coalescing once waited value is emitted"
        );
        assert!(
            waiter.join().unwrap(),
            "background wait_cpu(7) registered before direct.signal(7) deadlocked"
        );
        assert!(f_shared.wait_cpu(8, 5000));

        direct.wait_idle();
        copy.wait_idle();
    }

    #[test]
    fn collect_async_offloads_expired_drops_and_submit_prepared_handles_inline_and_overflow() {
        use super::super::cmdlist::{CommandAllocator, CommandList};
        use super::super::resource::Resource;
        use crate::types::{dim, heap_type, HeapProperties, ResourceDesc};
        use std::sync::Arc;
        use std::thread::ThreadId;

        if super::super::device().is_none() {
            return;
        }
        let q = Queue::new(ListType::Direct, "async collect + prepared submit test").unwrap();
        q.set_submit_coalesce(true);

        // 1) Verify `collect_async` drops expired items off the calling thread and resets `has_deferred`.
        struct RecordThreadDrop {
            slot: Arc<Mutex<Option<ThreadId>>>,
        }
        impl Drop for RecordThreadDrop {
            fn drop(&mut self) {
                *self.slot.lock().unwrap() = Some(std::thread::current().id());
            }
        }

        let make_buf = |ty: u32| -> Resource {
            let d = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: 64,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            let hp = HeapProperties { Type: ty, ..Default::default() };
            Resource::new_committed_default(&d, &hp, "prepared submit test buf").unwrap()
        };
        let up = make_buf(heap_type::UPLOAD);
        let rb = make_buf(heap_type::READBACK);
        unsafe { *(up.cpu_ptr().unwrap() as *mut u32) = 0x55AA_1234 };

        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.begin(&alloc).unwrap();
        l.copy_buffer(&rb, 0, &up, 0, 16).unwrap();
        l.close().unwrap();

        let mut batch = q.begin_prepare(0);
        q.prepare_list(&mut batch, &mut l, 0);
        let res = q.submit_prepared(batch);
        assert_eq!(res.submitted_cbs, 1);

        let drop_tid = Arc::new(Mutex::new(None));
        q.defer_release(Box::new(RecordThreadDrop { slot: drop_tid.clone() }));
        assert!(q.has_deferred.load(Ordering::Acquire));

        let v = q.flush_timeline();
        assert!(q.timeline.wait_cpu(v, 5000));
        let caller_tid = std::thread::current().id();
        q.collect_async();
        assert!(
            !q.has_deferred.load(Ordering::Acquire),
            "has_deferred must clear once all expired items are drained"
        );
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        let dropped_on = loop {
            if let Some(tid) = *drop_tid.lock().unwrap() {
                break tid;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "background reaper did not drop expired deferred item in time"
            );
            std::thread::sleep(std::time::Duration::from_millis(1));
        };
        assert_ne!(
            dropped_on, caller_tid,
            "collect_async must drop expired deferred items off the submission thread"
        );
        assert_eq!(unsafe { *(rb.cpu_ptr().unwrap() as *const u32) }, 0x55AA_1234);

        // 2) Verify `PreparedSubmit` overflow path (> PREPARED_INLINE_CAP = 8 lists) executes all copies.
        const N_OVERFLOW: usize = PREPARED_INLINE_CAP + 2;
        let allocs: Vec<_> = (0..N_OVERFLOW)
            .map(|_| CommandAllocator::new(ListType::Direct).unwrap())
            .collect();
        let mut lists: Vec<_> = (0..N_OVERFLOW)
            .map(|_| CommandList::new(ListType::Direct).unwrap())
            .collect();
        let rbs: Vec<_> = (0..N_OVERFLOW).map(|_| make_buf(heap_type::READBACK)).collect();
        let mut ov_batch = q.begin_prepare(0);
        for i in 0..N_OVERFLOW {
            lists[i].begin(&allocs[i]).unwrap();
            lists[i].copy_buffer(&rbs[i], 0, &up, 0, 16).unwrap();
            lists[i].close().unwrap();
            q.prepare_list(&mut ov_batch, &mut lists[i], i);
        }
        let ov_res = q.submit_prepared(ov_batch);
        assert_eq!(ov_res.submitted_cbs, N_OVERFLOW);
        q.wait_idle();
        for (i, r) in rbs.iter().enumerate() {
            assert_eq!(
                unsafe { *(r.cpu_ptr().unwrap() as *const u32) },
                0x55AA_1234,
                "overflow PreparedSubmit list {i} copy failed"
            );
        }
    }

    #[test]
    fn async_commit_worker_preserves_order_reuse_cross_queue_and_cpu_signal_on_gpu() {
        use super::super::cmdlist::{Barrier, CommandAllocator, CommandList};
        use super::super::resource::Resource;
        use crate::types::{dim, heap_type, HeapProperties, ResourceDesc};

        if super::super::device().is_none() {
            return;
        }
        if std::env::var_os("PORT_ASYNC_COMMIT").is_none() {
            assert!(!async_commit(), "PORT_ASYNC_COMMIT must default OFF");
            let default_direct = Queue::new(ListType::Direct, "default direct").unwrap();
            assert!(
                default_direct.commit_worker.is_none(),
                "default Direct queue must commit inline when PORT_ASYNC_COMMIT is unset"
            );
        }
        let direct = Queue::new_with_async_commit(ListType::Direct, "async commit direct", true).unwrap();
        let present =
            Queue::new_with_async_commit(ListType::Direct, "swapchain present queue", true).unwrap();
        let copy = Queue::new_with_async_commit(ListType::Copy, "async commit copy", true).unwrap();
        assert!(direct.commit_worker.is_some(), "opt-in Direct queue must use CommitWorker");
        assert!(present.commit_worker.is_none(), "swapchain-internal queue must not spawn CommitWorker");
        assert!(copy.commit_worker.is_none(), "Copy queue must not spawn CommitWorker");

        let make_buf = |ty: u32| -> Resource {
            let d = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: 64,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            let hp = HeapProperties { Type: ty, ..Default::default() };
            Resource::new_committed_default(&d, &hp, "async commit test buf").unwrap()
        };
        let up1 = make_buf(heap_type::UPLOAD);
        let up2 = make_buf(heap_type::UPLOAD);
        let mid = make_buf(heap_type::DEFAULT);
        let rb = make_buf(heap_type::READBACK);
        unsafe {
            *(up1.cpu_ptr().unwrap() as *mut u32) = 0x1111_2222;
            *(up2.cpu_ptr().unwrap() as *mut u32) = 0x3333_4444;
        }

        let a1 = CommandAllocator::new(ListType::Direct).unwrap();
        let a2 = CommandAllocator::new(ListType::Direct).unwrap();
        let a_copy = CommandAllocator::new(ListType::Copy).unwrap();
        let mut l_ml = CommandList::new(ListType::Direct).unwrap();
        let mut l_post = CommandList::new(ListType::Direct).unwrap();
        let mut l_copy = CommandList::new(ListType::Copy).unwrap();

        // 1) ML-flagged list triggers eager dispatch onto `CommitWorker`; a second list
        //    on a different allocator records + submits right behind it, then a Copy queue
        //    waits on `direct.timeline` before copying `mid` -> `rb` (cross-queue wait-before-signal).
        l_ml.begin(&a1).unwrap();
        l_ml.copy_buffer(&mid, 0, &up1, 0, 16).unwrap();
        l_ml.has_ml_work = true;
        l_ml.close().unwrap();
        let _ = direct.execute_command_lists(&mut [&mut l_ml]);

        l_post.begin(&a2).unwrap();
        l_post.barrier_list(&[Barrier::Transition(&mid as *const Resource as usize, 0x400, 0x400)]);
        l_post.copy_buffer(&mid, 0, &up2, 0, 16).unwrap();
        l_post.close().unwrap();
        let _ = direct.execute_command_lists(&mut [&mut l_post]);

        let vm = direct.flush_timeline();
        copy.wait(&direct.timeline, vm);
        l_copy.begin(&a_copy).unwrap();
        l_copy.copy_buffer(&rb, 0, &mid, 0, 16).unwrap();
        l_copy.close().unwrap();
        let _ = copy.execute_command_lists(&mut [&mut l_copy]);
        copy.wait_idle();
        assert_eq!(
            unsafe { *(rb.cpu_ptr().unwrap() as *const u32) },
            0x3333_4444,
            "async CommitWorker must preserve strict FIFO commit + cross-queue timeline order"
        );

        // 2) Immediate re-begin of `l_ml` on `a1` right after eager async submit + allocator reset
        //    must safely synchronize against `CommitWorker` with 0 Metal validation errors.
        l_ml.begin(&a1).unwrap();
        l_ml.copy_buffer(&rb, 0, &up1, 0, 16).unwrap();
        l_ml.has_ml_work = true;
        l_ml.close().unwrap();
        let _ = direct.execute_command_lists(&mut [&mut l_ml]);
        // Re-begin `l_ml` immediately while `CommitWorker` may still be committing it:
        l_ml.begin(&a2).unwrap();
        l_ml.copy_buffer(&rb, 0, &up2, 0, 16).unwrap();
        l_ml.close().unwrap();
        let _ = direct.execute_command_lists(&mut [&mut l_ml]);
        direct.wait_idle();
        a1.reset();
        a2.reset();
        assert_eq!(unsafe { *(rb.cpu_ptr().unwrap() as *const u32) }, 0x3333_4444);

        // 3) `Fence::signal_cpu` must drain `CommitWorker` so an earlier GPU signal cannot
        //    overwrite the CPU-signaled value backward.
        let f_cpu = Fence::new(0).unwrap();
        direct.signal(&f_cpu, 1);
        let _ = direct.flush_timeline();
        f_cpu.signal_cpu(5);
        direct.wait_idle();
        assert_eq!(f_cpu.completed(), 5);
    }

    #[test]
    fn perf_meter_excludes_swapchain_internal_queues() {
        let pm = (perf_meter_callback as GpuFeedbackHook) as usize;
        assert!(should_install_feedback(pm, false), "q0 must install perf_meter_callback");
        assert!(
            !should_install_feedback(pm, true),
            "swapchain-internal present/staging queues must be excluded from q0 perf_meter"
        );
        fn custom_hook(_: u32, _: u32, _: u64, _: u64, _: u64) {}
        let custom = (custom_hook as GpuFeedbackHook) as usize;
        assert!(
            should_install_feedback(custom, true),
            "custom diagnostic feedback hooks must still observe swapchain-internal queues"
        );
        assert!(!should_install_feedback(0, false));
        assert!(!should_install_feedback(0, true));
    }
}

