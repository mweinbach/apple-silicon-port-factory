//! ID3D12Fence on `MTLSharedEvent`.
//!
//! * GPU signal/wait: `MTL4CommandQueue signalEvent/waitForEvent`.
//! * CPU signal: `setSignaledValue`.
//! * `SetEventOnCompletion`: one process-wide `MTLSharedEventListener`; the
//!   callback runs on a libdispatch thread (no TEB), so callers must pass a
//!   TEB-free action (win32-rt's `Event::set` qualifies).

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::OnceLock;

pub type MtlEvent = ProtocolObject<dyn MTLSharedEvent>;

pub struct Fence {
    pub ev: Retained<MtlEvent>,
    /// Queue (address) that last enqueued a GPU signal of this fence, and the
    /// highest value it enqueued; 0 after a CPU signal.
    pub(crate) sig_queue: AtomicUsize,
    pub(crate) sig_value: AtomicU64,
    /// Non-zero when `AW2_FENCE_SIGNAL_COALESCE=1` holds an unflushed GPU signal
    /// for this fence on `q0` (`Queue` address).
    pub(crate) pending_sig_queue: AtomicUsize,
    pub(crate) pending_sig_value: AtomicU64,
    pub(crate) pending_sig_ns: AtomicU64,
    pub(crate) poll_epoch: AtomicU64,
    pub(crate) poll_count: AtomicU32,
    /// Highest fence value waited on by a non-elided `Queue::wait_with`,
    /// `Fence::wait_cpu`, or `Fence::on_completion`.
    /// When `emitted_sig_value < wait_target`, `Queue::signal` flushes eagerly
    /// instead of deferring so GPU and CPU waits registered before their
    /// producer signals always make forward progress (including with `AW2_D3D_CS=0`).
    pub(crate) wait_target: AtomicU64,
    /// Highest fence value actually emitted to Metal (`signalEvent_value` or
    /// `setSignaledValue`), excluding still-deferred coalesced signals.
    pub(crate) emitted_sig_value: AtomicU64,
}
unsafe impl Send for Fence {}
unsafe impl Sync for Fence {}

struct Listener(Retained<MTLSharedEventListener>);
unsafe impl Send for Listener {}
unsafe impl Sync for Listener {}

fn listener() -> &'static MTLSharedEventListener {
    static L: OnceLock<Listener> = OnceLock::new();
    &L.get_or_init(|| Listener(MTLSharedEventListener::new())).0
}

impl Fence {
    pub fn new(initial: u64) -> Option<Fence> {
        let d = super::device()?;
        let ev = d.dev.newSharedEvent()?;
        ev.setSignaledValue(initial);
        Some(Fence {
            ev,
            sig_queue: Default::default(),
            sig_value: Default::default(),
            pending_sig_queue: Default::default(),
            pending_sig_value: Default::default(),
            pending_sig_ns: Default::default(),
            poll_epoch: Default::default(),
            poll_count: Default::default(),
            wait_target: Default::default(),
            emitted_sig_value: AtomicU64::new(initial),
        })
    }

    #[inline]
    pub(crate) fn ev_key(&self) -> usize {
        Retained::as_ptr(&self.ev) as usize
    }

    #[inline]
    pub(crate) fn completed_raw(&self) -> u64 {
        self.ev.signaledValue()
    }

    #[inline]
    fn flush_pending_for(&self, target_v: u64) {
        let q_addr = self.pending_sig_queue.load(Ordering::SeqCst);
        if q_addr != 0 && self.ev.signaledValue() < target_v {
            super::queue::Queue::flush_fence_signals_by_addr(q_addr);
        }
        // If this fence is still incomplete, flush any buffered CBs / deferred
        // signals across live queues so transitive cross-queue dependencies
        // always make progress even when the CS thread is disabled (`AW2_D3D_CS=0`).
        if self.ev.signaledValue() < target_v && super::queue::has_pending_work() {
            super::queue::Queue::flush_all_pending_work();
        }
    }

    pub fn completed(&self) -> u64 {
        let raw = self.ev.signaledValue();
        let q_addr = self.pending_sig_queue.load(Ordering::Acquire);
        if q_addr == 0 {
            return raw;
        }
        let pv = self.pending_sig_value.load(Ordering::Acquire);
        if pv <= raw {
            return raw;
        }
        let cur_epoch = super::queue::Queue::commit_epoch_of(q_addr);
        let prev_epoch = self.poll_epoch.swap(cur_epoch, Ordering::AcqRel);
        let polls = if prev_epoch == cur_epoch {
            self.poll_count.fetch_add(1, Ordering::AcqRel) + 1
        } else {
            self.poll_count.store(1, Ordering::Release);
            1
        };
        let now_ns = super::queue::now_uptime_ns();
        let enq_ns = self.pending_sig_ns.load(Ordering::Acquire);
        if polls >= 8 || now_ns.saturating_sub(enq_ns) >= 500_000 {
            super::queue::Queue::flush_fence_signals_by_addr(q_addr);
            return self.ev.signaledValue();
        }
        raw
    }

    /// `ID3D12Fence::Signal` (CPU side).
    pub fn signal_cpu(&self, v: u64) {
        let q_addr = self.pending_sig_queue.load(Ordering::Acquire);
        if q_addr != 0
            && super::queue::Queue::resolve_for_cpu_signal(
                q_addr,
                self.ev_key(),
                self as *const Fence as usize,
                v,
            )
        {
            self.sig_queue.store(0, Ordering::Release);
            return;
        }
        self.pending_sig_queue.store(0, Ordering::Release);
        self.pending_sig_value.store(0, Ordering::Release);
        self.poll_count.store(0, Ordering::Release);
        self.sig_queue.store(0, Ordering::Release);
        self.emitted_sig_value.fetch_max(v, Ordering::AcqRel);
        self.ev.setSignaledValue(v);
    }

    /// Block until the fence reaches `v` (or `timeout_ms`; `u64::MAX` =
    /// forever). Returns true when reached.
    pub fn wait_cpu(&self, v: u64, timeout_ms: u64) -> bool {
        if self.completed_raw() >= v {
            return true;
        }
        // Publish CPU wait target before reading `pending_sig_queue` (Dekker
        // `SeqCst` handshake with `Queue::signal`) so a signal issued on
        // another thread concurrently or later flushes eagerly even with
        // `AW2_D3D_CS=0`.
        self.wait_target.fetch_max(v, Ordering::SeqCst);
        self.flush_pending_for(v);
        if self.completed_raw() >= v {
            return true;
        }
        if timeout_ms == u64::MAX {
            loop {
                if self.ev.waitUntilSignaledValue_timeoutMS(v, 1000) {
                    return true;
                }
            }
        }
        self.ev.waitUntilSignaledValue_timeoutMS(v, timeout_ms)
    }

    /// Run `f` once the fence reaches `v` (immediately if it already has).
    /// `f` runs on an arbitrary host thread.
    pub fn on_completion(&self, v: u64, f: Box<dyn FnOnce() + Send>) {
        if self.completed_raw() >= v {
            f();
            return;
        }
        // Publish CPU callback wait target before reading `pending_sig_queue`
        // (Dekker `SeqCst` handshake with `Queue::signal`) so registering
        // `SetEventOnCompletion(f, v, event)` before `Queue::signal(f, v)`
        // flushes eagerly even with `AW2_D3D_CS=0`.
        self.wait_target.fetch_max(v, Ordering::SeqCst);
        self.flush_pending_for(v);
        self.on_completion_no_flush(v, f);
    }

    /// Internal variant for `Queue`'s own `timeline` fence called while holding
    /// `Queue::submit` (cannot re-enter queue flush locks).
    pub(crate) fn on_completion_no_flush(&self, v: u64, f: Box<dyn FnOnce() + Send>) {
        if self.completed_raw() >= v {
            f();
            return;
        }
        let cell = std::sync::Mutex::new(Some(f));
        let block = block2::RcBlock::new(move |_e: NonNull<ProtocolObject<dyn MTLSharedEvent>>, _v: u64| {
            if let Some(f) = cell.lock().unwrap().take() {
                f();
            }
        });
        unsafe { self.ev.notifyListener_atValue_block(listener(), v, &*block as *const _ as *mut _) };
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        let q_addr = self.pending_sig_queue.load(Ordering::Acquire);
        if q_addr != 0 {
            super::queue::Queue::detach_dropped_fence(
                q_addr,
                self.ev_key(),
                self as *const Fence as usize,
            );
        }
    }
}
