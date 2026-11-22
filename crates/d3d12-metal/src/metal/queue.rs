//! ID3D12CommandQueue on `MTL4CommandQueue`.
//!
//! Every queue has an internal timeline (`MTLSharedEvent`) signaled after
//! each submission; objects whose lifetime must outlast GPU use are parked
//! with [`Queue::defer_release`] until the timeline passes.

use super::fence::{Fence, MtlEvent};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::Mutex;

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

/// `AW2_FENCE_SIGNAL_COALESCE`: coalesce mid-frame D3D12 `ID3D12CommandQueue::Signal`
/// calls on `ListType::Direct` (`q0`) (`fence_sig_coalesced`), flushing eagerly on
/// cross-queue `Wait`, CPU `Wait` / `SetEventOnCompletion`, `signal_cpu`, `Present` /
/// `last_submitted`, `wait_idle`, and `GetCompletedValue` spin-polls. Default **off**
/// (`AW2_FENCE_SIGNAL_COALESCE=1` enables).
pub fn fence_signal_coalesce() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("AW2_FENCE_SIGNAL_COALESCE").is_some_and(|v| v == "1"))
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
    next: AtomicU64,
    deferred: Mutex<Vec<(u64, Deferred)>>,
    /// Serializes submissions so timeline values are monotonic.
    submit: Mutex<()>,
    /// Strongest GPU wait enqueued since the last submission (diagnostics):
    /// 0 none, 1 already reached, 2 on another queue's signal, 3 on a CPU
    /// signal not yet made.
    pub pending_wait: AtomicU8,
    registered_active: AtomicBool,
    unsignaled: AtomicBool,
    pending_log_count: AtomicUsize,
    pending_pass_logs: Mutex<Vec<super::passtiming::PassLog>>,
    pub(crate) opt_submit_coalesce: AtomicBool,
    is_swapchain_internal: bool,
    pub(crate) opt_fence_signal_coalesce: AtomicBool,
    commit_epoch: AtomicU64,
    has_deferred_signals: AtomicBool,
    deferred_signals: Mutex<Vec<DeferredFenceSig>>,
}
unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}

fn active_queues() -> &'static Mutex<Vec<usize>> {
    static Q: std::sync::OnceLock<Mutex<Vec<usize>>> = std::sync::OnceLock::new();
    Q.get_or_init(Default::default)
}

/// Park `obj` until all currently live queues have finished work submitted so far.
pub fn defer_release_all<T: Send + 'static>(obj: T) {
    let qs = active_queues().lock().unwrap().clone();
    if qs.is_empty() {
        return;
    }
    let arc = std::sync::Arc::new(Mutex::new(obj));
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

fn perf_meter_callback(queue_kind: u32, n_cbs: u32, _commit_ns: u64, gpu_start_ns: u64, gpu_end_ns: u64) {
    // Track merged non-overlapping GPU busy time on Direct queues (`ListType::Direct as u32 == 0`).
    if queue_kind != ListType::Direct as u32 || gpu_end_ns <= gpu_start_ns {
        return;
    }
    if let Ok(mut st) = PERF_METER.lock() {
        let start = gpu_start_ns.max(st.last_end_ns);
        if gpu_end_ns > start {
            st.busy_ns = st.busy_ns.saturating_add(gpu_end_ns - start);
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

impl Queue {
    pub fn new(kind: ListType, label: &str) -> Option<Queue> {
        let d = super::device()?;
        let desc = MTL4CommandQueueDescriptor::new();
        desc.setLabel(Some(&objc2_foundation::NSString::from_str(label)));
        let q = d.dev.newMTL4CommandQueueWithDescriptor_error(&desc).ok()?;
        q.addResidencySet(&d.residency);
        let lower = label.to_ascii_lowercase();
        let is_swapchain_internal = lower.contains("swapchain") || lower.contains("present");
        let allow_direct_coalesce = kind == ListType::Direct && !is_swapchain_internal;
        Some(Queue {
            q,
            kind,
            timeline: Fence::new(0)?,
            next: AtomicU64::new(1),
            deferred: Mutex::new(Vec::new()),
            submit: Mutex::new(()),
            pending_wait: AtomicU8::new(0),
            registered_active: AtomicBool::new(false),
            unsignaled: AtomicBool::new(false),
            pending_log_count: AtomicUsize::new(0),
            pending_pass_logs: Mutex::new(Vec::new()),
            opt_submit_coalesce: AtomicBool::new(kind == ListType::Direct && submit_coalesce()),
            is_swapchain_internal,
            opt_fence_signal_coalesce: AtomicBool::new(allow_direct_coalesce && fence_signal_coalesce()),
            commit_epoch: AtomicU64::new(0),
            has_deferred_signals: AtomicBool::new(false),
            deferred_signals: Mutex::new(Vec::new()),
        })
    }

    #[cfg(test)]
    pub(crate) fn set_submit_coalesce(&self, on: bool) {
        if !on {
            let _g = self.submit.lock().unwrap();
            let _ = self.flush_timeline_locked();
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
            q.has_deferred_signals.store(!ds.is_empty(), Ordering::Release);
            drop(ds);
            if fence_ptr != 0 {
                let f = unsafe { &*(fence_ptr as *const Fence) };
                f.pending_sig_queue.store(0, Ordering::Release);
                f.pending_sig_value.store(0, Ordering::Release);
                f.poll_count.store(0, Ordering::Release);
                f.ev.setSignaledValue(v);
            }
            if let Some((ev, gv)) = greater_sig {
                q.q.signalEvent_value(ProtocolObject::from_ref(&*ev), gv);
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

    /// Emit all deferred `ID3D12CommandQueue::Signal` events in enqueue order.
    /// Caller must hold `self.submit`.
    fn flush_fence_signals_locked(&self) {
        if !self.has_deferred_signals.swap(false, Ordering::AcqRel) {
            return;
        }
        let entries = {
            let mut ds = self.deferred_signals.lock().unwrap();
            std::mem::take(&mut *ds)
        };
        if entries.is_empty() {
            return;
        }
        let cur_epoch = self.commit_epoch.load(Ordering::Acquire);
        let me = self as *const Queue as usize;
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
            }
            if entry.enqueued_epoch < cur_epoch {
                super::encstats::inc(&super::encstats::c::fence_sig_coalesced);
            }
            self.q.signalEvent_value(ProtocolObject::from_ref(&*entry.ev), entry.val);
        }
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
    /// or `PassLog`s are awaiting a timeline value. Caller must hold `self.submit`.
    fn flush_timeline_locked(&self) -> u64 {
        self.flush_fence_signals_locked();
        let logs = if self.pending_log_count.load(Ordering::Acquire) > 0 {
            let mut pl = self.pending_pass_logs.lock().unwrap();
            self.pending_log_count.store(0, Ordering::Release);
            std::mem::take(&mut *pl)
        } else {
            Vec::new()
        };
        if !self.unsignaled.swap(false, Ordering::AcqRel) && logs.is_empty() {
            return self.next.load(Ordering::Acquire) - 1;
        }
        let v = self.next.fetch_add(1, Ordering::AcqRel);
        self.q.signalEvent_value(ProtocolObject::from_ref(&*self.timeline.ev), v);
        if !logs.is_empty() {
            self.timeline.on_completion(v, Box::new(move || super::passtiming::resolve(logs)));
        }
        self.collect();
        v
    }

    /// Commit closed command buffers in order and **always** advance and signal
    /// `self.timeline` eagerly (`v >= 1`). Used by `Swapchain` (`sq`), `PresentThread`
    /// (`pq`), offscreen/main present blits, and internal/test callers that synchronize
    /// directly on the returned timeline value.
    pub fn submit(&self, cbs: &[&MtlCommandBuffer]) -> u64 {
        self.submit_with_logs(cbs, Vec::new())
    }

    /// Commit closed command buffers in order and eagerly signal `self.timeline`,
    /// folding in any previously deferred `unsignaled` command buffers or `PassLog`s.
    pub fn submit_with_logs(&self, cbs: &[&MtlCommandBuffer], mut logs: Vec<super::passtiming::PassLog>) -> u64 {
        self.ensure_registered_active();
        let _g = self.submit.lock().unwrap();
        if self.pending_log_count.load(Ordering::Acquire) > 0 {
            let mut pl = self.pending_pass_logs.lock().unwrap();
            self.pending_log_count.store(0, Ordering::Release);
            if !pl.is_empty() {
                pl.append(&mut logs);
                logs = std::mem::take(&mut *pl);
            }
        }
        self.unsignaled.store(false, Ordering::Release);
        if let Some(d) = super::device() {
            d.commit_residency();
        }
        if !cbs.is_empty() {
            let mut ptrs: Vec<NonNull<MtlCommandBuffer>> = cbs.iter().map(|c| NonNull::from(*c)).collect();
            let hook = FEEDBACK_HOOK.load(Ordering::Relaxed);
            if hook != 0 {
                commit_with_feedback(&self.q, &mut ptrs, self.kind, hook);
            } else {
                unsafe { self.q.commit_count(NonNull::new(ptrs.as_mut_ptr()).unwrap(), ptrs.len()) };
            }
            self.commit_epoch.fetch_add(1, Ordering::AcqRel);
            SUBMIT_EPOCH.fetch_add(1, Ordering::AcqRel);
        }
        self.flush_fence_signals_locked();
        let v = self.next.fetch_add(1, Ordering::AcqRel);
        self.q.signalEvent_value(ProtocolObject::from_ref(&*self.timeline.ev), v);
        if !logs.is_empty() {
            self.timeline.on_completion(v, Box::new(move || super::passtiming::resolve(logs)));
        }
        self.collect();
        v
    }

    /// Submit D3D12 `ExecuteCommandLists` command buffers. When `AW2_SUBMIT_COALESCE=1`
    /// is enabled on a `ListType::Direct` queue (`q0`), skips per-call `signalEvent_value`
    /// timeline drains (`submit_nosig`) and batches `PassLog` callbacks until demand
    /// (`last_submitted`, `defer_release`, `wait_idle`, `submit`, or 8 pending `PassLog`s).
    pub fn submit_d3d_lists(&self, cbs: &[&MtlCommandBuffer], logs: Vec<super::passtiming::PassLog>) -> u64 {
        let coalesce = self.kind == ListType::Direct && self.opt_submit_coalesce.load(Ordering::Relaxed);
        if !coalesce {
            return self.submit_with_logs(cbs, logs);
        }
        self.ensure_registered_active();
        let _g = self.submit.lock().unwrap();
        if cbs.is_empty() && logs.is_empty() {
            super::encstats::inc(&super::encstats::c::submit_nosig);
            self.collect();
            return self.next.load(Ordering::Acquire) - 1;
        }
        if !cbs.is_empty() {
            if let Some(d) = super::device() {
                d.commit_residency();
            }
            let mut ptrs: Vec<NonNull<MtlCommandBuffer>> = cbs.iter().map(|c| NonNull::from(*c)).collect();
            let hook = FEEDBACK_HOOK.load(Ordering::Relaxed);
            if hook != 0 {
                commit_with_feedback(&self.q, &mut ptrs, self.kind, hook);
            } else {
                unsafe { self.q.commit_count(NonNull::new(ptrs.as_mut_ptr()).unwrap(), ptrs.len()) };
            }
            self.commit_epoch.fetch_add(1, Ordering::AcqRel);
            SUBMIT_EPOCH.fetch_add(1, Ordering::AcqRel);
            self.unsignaled.store(true, Ordering::Release);
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
            return self.flush_timeline_locked();
        }
        super::encstats::inc(&super::encstats::c::submit_nosig);
        self.collect();
        self.next.load(Ordering::Acquire) - 1
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
        let mut cbs = Vec::with_capacity(lists.len());
        let coalesce = self.kind == ListType::Direct && self.opt_submit_coalesce.load(Ordering::Relaxed);
        let pt_on = super::passtiming::enabled();
        let mut pass_logs = Vec::new();
        let mut wait_sampled: Option<u8> = None;
        for (i, list) in lists.iter_mut().enumerate() {
            if list.is_open() {
                log::error!(target: "d3d", "ExecuteCommandLists: list {i} is still open (not Closed); skipped");
                continue;
            }
            if list.kind != self.kind && self.kind != ListType::Direct {
                log::error!(target: "d3d", "ExecuteCommandLists: {:?} list on a {:?} queue", list.kind, self.kind);
            }
            if coalesce && list.has_commands() && !list.has_gpu_work_recorded() {
                let _ = list.take_pass_log();
                super::encstats::inc(&super::encstats::c::submit_coalesced);
                continue;
            }
            if pt_on {
                if let Some(mut p) = list.take_pass_log() {
                    let wait = *wait_sampled.get_or_insert_with(|| {
                        self.pending_wait.swap(0, Ordering::Relaxed)
                    });
                    p.submitted(t_ecl, self.kind as u8, wait);
                    pass_logs.push(p);
                }
            }
            if list.has_commands() {
                cbs.push(&*list.cb);
            }
        }
        let submitted_cbs = cbs.len();
        let timeline_value = self.submit_d3d_lists(&cbs, pass_logs);
        ExecuteResult { submitted_cbs, timeline_value }
    }

    /// Value the timeline will reach once everything submitted so far is done.
    pub fn last_submitted(&self) -> u64 {
        if self.opt_submit_coalesce.load(Ordering::Acquire)
            || self.unsignaled.load(Ordering::Acquire)
            || self.pending_log_count.load(Ordering::Acquire) > 0
            || self.has_deferred_signals.load(Ordering::Acquire)
        {
            let _g = self.submit.lock().unwrap();
            return self.flush_timeline_locked();
        }
        self.next.load(Ordering::Acquire) - 1
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
        if self.kind == ListType::Direct
            && !self.is_swapchain_internal
            && self.opt_fence_signal_coalesce.load(O::Relaxed)
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
                f.pending_sig_queue.store(me, O::Release);
            } else {
                ds.push(DeferredFenceSig {
                    ev_key,
                    ev: f.ev.clone(),
                    fence_ptr: f_ptr,
                    val: v,
                    enqueued_epoch: cur_epoch,
                });
                self.has_deferred_signals.store(true, O::Release);
                drop(ds);
                f.pending_sig_ns.store(now_ns, O::Release);
                f.poll_epoch.store(cur_epoch, O::Release);
                f.poll_count.store(0, O::Release);
                f.pending_sig_value.store(v, O::Release);
                f.pending_sig_queue.store(me, O::Release);
            }
            return;
        }
        self.q.signalEvent_value(ProtocolObject::from_ref(&*f.ev), v);
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
        // Cross-queue wait (or `AW2_LIST_BARRIER_TIGHT=1` same-queue event wait):
        // flush `sig_q`'s deferred fence signals BEFORE acquiring `self.submit` so we
        // never lock `sig_q.submit` inside `self.submit` and `f.ev` is guaranteed to be
        // signaled on `sig_q` prior to `self.q.waitForEvent_value(&f.ev, v)`.
        let pending_q = f.pending_sig_queue.load(O::Acquire);
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
        self.pending_wait.fetch_max(kind, O::Relaxed);
        self.q.waitForEvent_value(ProtocolObject::from_ref(&*f.ev), v);
    }

    /// Keep `obj` alive until all work submitted so far has completed.
    pub fn defer_release(&self, obj: Deferred) {
        let v = self.last_submitted();
        if self.timeline.completed() >= v {
            drop(obj);
            return;
        }
        self.deferred.lock().unwrap().push((v, obj));
    }

    /// Move deferred objects whose submissions have completed into `out` (dropping outside locks).
    fn drain_completed_into(&self, out: &mut Vec<Deferred>) {
        let done = self.timeline.completed();
        let mut d = self.deferred.lock().unwrap();
        if d.iter().any(|(v, _)| *v <= done) {
            let prev = std::mem::take(&mut *d);
            for (v, obj) in prev {
                if v <= done {
                    out.push(obj);
                } else {
                    d.push((v, obj));
                }
            }
        }
    }

    /// Drop deferred objects whose submissions have completed.
    pub fn collect(&self) {
        let mut expired = Vec::new();
        self.drain_completed_into(&mut expired);
        drop(expired);
    }

    /// Block until the queue is idle.
    pub fn wait_idle(&self) {
        let v = self.last_submitted();
        self.timeline.wait_cpu(v, u64::MAX);
        self.collect();
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        let addr = self as *const Queue as usize;
        active_queues().lock().unwrap().retain(|&p| p != addr);
        if self.unsignaled.load(Ordering::Acquire)
            || self.pending_log_count.load(Ordering::Acquire) > 0
            || self.has_deferred_signals.load(Ordering::Acquire)
        {
            let _g = self.submit.lock().unwrap();
            let _ = self.flush_timeline_locked();
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
        // `!has_gpu_work_recorded()` without altering `has_commands()` (`AW2_EMPTY_ENC_ELIDE` stays OFF).
        l_empty.begin(&a_empty).unwrap();
        l_empty.barrier_list(&[Barrier::Uav(0)]);
        l_empty.close().unwrap();
        assert!(l_empty.has_commands());
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

        assert_eq!(
            q.next.load(Ordering::Acquire),
            1,
            "lazy timeline coalescing must emit 0 timeline signals during burst submit"
        );
        assert!(q.unsignaled.load(Ordering::Acquire));
        assert!(super::super::encstats::c::submit_nosig.load(Ordering::Relaxed) >= nosig0 + 4);
        if had_logs {
            assert_eq!(q.pending_log_count.load(Ordering::Acquire), 3);
            assert!(super::super::encstats::c::submit_coalesced.load(Ordering::Relaxed) >= coal0 + 2);
        }

        // `defer_release` / `last_submitted` flushes ALL 3 submissions onto a SINGLE timeline signal (`v == 1`).
        struct DropTracker(Arc<AtomicBool>);
        impl Drop for DropTracker {
            fn drop(&mut self) {
                self.0.store(true, Ordering::Release);
            }
        }
        let dropped = Arc::new(AtomicBool::new(false));
        q.defer_release(Box::new(DropTracker(dropped.clone())));
        assert_eq!(q.next.load(Ordering::Acquire), 2, "flush must advance timeline by exactly 1 tick");
        assert!(!q.unsignaled.load(Ordering::Acquire));
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
    }

    #[test]
    fn fence_signal_coalesce_batches_q0_signals_flushes_cross_queue_cpu_and_spin_polls_on_gpu() {
        use super::super::cmdlist::{Barrier, CommandAllocator, CommandList};
        use super::super::resource::Resource;
        use crate::types::{dim, heap_type, HeapProperties, ResourceDesc};

        if super::super::device().is_none() {
            return;
        }
        if std::env::var_os("AW2_FENCE_SIGNAL_COALESCE").is_none() {
            assert!(!fence_signal_coalesce(), "AW2_FENCE_SIGNAL_COALESCE must default OFF");
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
}

