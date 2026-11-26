//! Kernel object handle table and the generic wait machinery.
//!
//! Handles are monotonically allocated small multiples of 4 starting at `0x100`
//! (like Windows). Lookups (`handles::get`, `handles::get_flags`, and the
//! zero-`Arc`-clone sync fast paths) are 100% lock-free over a 2-level chunked
//! table protected against concurrent `close()` by per-thread cache-line-aligned
//! hazard pointers, with synchronous `on_handle_close()` and `Drop` on `close()`.
//!
//! Single-object non-alertable waits on `Semaphore` and `Event` run directly on
//! each object's packed atomic futex word (adaptive bounded spin + Darwin
//! `__ulock_wait` / `__ulock_wake` with wake coalescing and `ULF_WAKE_ALL`
//! batching), avoiding `WAIT_LOCK`, wait buckets, and `Arc` refcount bouncing.
//! Multi-object, alertable, mutex, timer, and thread waits use sharded
//! cache-line-aligned wait buckets with LIFO wake selection preferring spinning
//! waiters and a stack-allocated [`WakeBatch`].

use crate::park::{self, Parker, WakeOp, WordLock};
use crate::types::*;
use std::any::Any;
use std::cell::{Cell, UnsafeCell};
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};

pub trait KObject: Any + Send + Sync {
    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    /// Waitable objects return themselves.
    fn waitable(&self) -> Option<&dyn Waitable> {
        None
    }
    /// Called when a handle table entry referencing this object is opened (created or duplicated).
    fn on_handle_open(&self) {}
    /// Called after a handle table entry referencing this object is closed (outside the handle table lock).
    fn on_handle_close(&self) {}
}

/// State is only accessed while holding [`WAIT_LOCK`] (passed as `&WaitGuard`),
/// except lock-free objects (`Event`, `Semaphore`) which synchronize via atomics.
pub trait Waitable {
    /// Would a wait by thread `tid` be satisfied right now?
    fn is_signaled(&self, g: &WaitGuard, tid: u32) -> bool;
    /// Consume the signal (auto-reset event, semaphore count, mutex ownership).
    /// Returns true if the wait is "abandoned" (mutex).
    fn acquire(&self, g: &WaitGuard, tid: u32) -> bool;
    /// Consume the signal if the object is signaled: `Some(abandoned)`, or
    /// `None`. Objects whose state can also change without `WAIT_LOCK`
    /// (semaphore units, events) check and consume atomically.
    fn try_acquire(&self, g: &WaitGuard, tid: u32) -> Option<bool> {
        if self.is_signaled(g, tid) {
            Some(self.acquire(g, tid))
        } else {
            None
        }
    }
    /// Wake identity key for targeted notifications. Defaults to the object's
    /// concrete address; wrapper waitables (e.g. `user32::message::QueueWaiter`)
    /// override this to return the underlying target's address.
    fn wait_key(&self) -> usize {
        obj_key(self)
    }
}

/// Small-buffer container for resolved waitable objects: stores up to 8 handles
/// inline on the stack (avoiding a heap `Vec` allocation on every
/// `WaitForMultipleObjects` / `MsgWaitForMultipleObjectsEx` call).
pub enum WaitableList {
    Inline {
        buf: [MaybeUninit<Arc<dyn KObject>>; 8],
        len: usize,
    },
    Heap(Vec<Arc<dyn KObject>>),
}

impl WaitableList {
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        if cap <= 8 {
            WaitableList::Inline {
                buf: [const { MaybeUninit::uninit() }; 8],
                len: 0,
            }
        } else {
            WaitableList::Heap(Vec::with_capacity(cap))
        }
    }

    #[inline]
    pub fn push(&mut self, obj: Arc<dyn KObject>) {
        match self {
            WaitableList::Inline { buf, len } if *len < 8 => {
                buf[*len].write(obj);
                *len += 1;
            }
            WaitableList::Inline { buf, len } => {
                let mut v = Vec::with_capacity(*len + 1);
                for slot in &mut buf[..*len] {
                    v.push(unsafe { slot.assume_init_read() });
                }
                *len = 0;
                v.push(obj);
                *self = WaitableList::Heap(v);
            }
            WaitableList::Heap(v) => v.push(obj),
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[Arc<dyn KObject>] {
        match self {
            WaitableList::Inline { buf, len } => unsafe {
                std::slice::from_raw_parts(buf.as_ptr() as *const Arc<dyn KObject>, *len)
            },
            WaitableList::Heap(v) => v.as_slice(),
        }
    }
}

impl Drop for WaitableList {
    fn drop(&mut self) {
        if let WaitableList::Inline { buf, len } = self {
            for slot in &mut buf[..*len] {
                unsafe { slot.assume_init_drop() };
            }
        }
    }
}

pub struct WaitState {
    _priv: (),
}

/// Proof that [`WAIT_LOCK`] is held (unlocks on drop).
pub struct WaitGuard<'a> {
    _l: std::marker::PhantomData<&'a WaitState>,
    /// Not `Send`: the lock word must be released by the thread that took it.
    _not_send: std::marker::PhantomData<*const ()>,
}

impl Drop for WaitGuard<'_> {
    #[inline]
    fn drop(&mut self) {
        WAIT_LOCK.unlock();
    }
}

static WAIT_LOCK: WordLock = WordLock::new();

#[inline]
pub fn wait_lock() -> WaitGuard<'static> {
    WAIT_LOCK.lock();
    WaitGuard { _l: std::marker::PhantomData, _not_send: std::marker::PhantomData }
}

/// Per-thread wait state (one `Arc<Waiter>` per OS thread, freed when the thread exits).
struct Waiter {
    park: Parker,
    /// Registered for an active wait.
    active: AtomicBool,
    /// Single object, not wait-all: one budgeted wake satisfies it.
    single: AtomicBool,
    /// A wake has been sent and not yet observed (cleared right before re-checking
    /// and sleeping). Budgeted and coalesced wakes skip waiters with a pending wake.
    notified: AtomicBool,
}

impl Waiter {
    fn new() -> Self {
        Waiter {
            park: Parker::new(),
            active: AtomicBool::new(false),
            single: AtomicBool::new(false),
            notified: AtomicBool::new(false),
        }
    }
}

const MAX_KEYS: usize = 8;
const WAIT_BUCKETS: usize = 64;

/// Cache-line aligned (128 B on Apple Silicon) wait bucket to prevent false sharing
/// between unrelated synchronization objects.
#[repr(align(128))]
struct WaitBucket {
    lock: WordLock,
    /// Fast lock-free count of entries currently in this bucket.
    count: AtomicUsize,
    entries: UnsafeCell<Vec<(usize, Arc<Waiter>)>>,
}

unsafe impl Sync for WaitBucket {}

impl WaitBucket {
    const fn new() -> Self {
        WaitBucket {
            lock: WordLock::new(),
            count: AtomicUsize::new(0),
            entries: UnsafeCell::new(Vec::new()),
        }
    }

    #[inline]
    fn insert(&self, key: usize, w: Arc<Waiter>) {
        self.lock.lock();
        unsafe { (*self.entries.get()).push((key, w)) };
        self.count.fetch_add(1, Ordering::SeqCst);
        self.lock.unlock();
    }

    #[inline]
    fn remove(&self, key: usize, w: &Arc<Waiter>) {
        self.lock.lock();
        let entries = unsafe { &mut *self.entries.get() };
        if let Some(pos) = entries.iter().position(|(k, e)| *k == key && Arc::ptr_eq(e, w)) {
            entries.swap_remove(pos);
            self.count.fetch_sub(1, Ordering::SeqCst);
        }
        self.lock.unlock();
    }
}

static BUCKETS: [WaitBucket; WAIT_BUCKETS] = [const { WaitBucket::new() }; WAIT_BUCKETS];
/// Overflow bucket for waits on > `MAX_KEYS` (8) objects at once.
static ANY_WAITERS: WaitBucket = WaitBucket::new();
/// Alertable waiters keyed by Windows `tid` (used by `notify_thread_apc`).
static ALERTABLE_WAITERS: WaitBucket = WaitBucket::new();

#[inline]
fn bucket_of(key: usize) -> usize {
    (key.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 58) & (WAIT_BUCKETS - 1)
}

thread_local! {
    static WAITER: Arc<Waiter> = Arc::new(Waiter::new());
}

/// Small stack buffer for collecting waiters to unpark outside any bucket lock.
struct WakeBatch {
    inline: [Option<Arc<Waiter>>; 16],
    len: usize,
    extra: Vec<Arc<Waiter>>,
}

impl WakeBatch {
    #[inline]
    fn new() -> Self {
        WakeBatch {
            inline: [const { None }; 16],
            len: 0,
            extra: Vec::new(),
        }
    }

    #[inline]
    fn push(&mut self, w: Arc<Waiter>) {
        if self.len < 16 {
            self.inline[self.len] = Some(w);
            self.len += 1;
        } else {
            self.extra.push(w);
        }
    }

    #[inline]
    fn unpark_all(mut self) {
        for slot in &mut self.inline[..self.len] {
            if let Some(w) = slot.take() {
                w.park.unpark();
            }
        }
        for w in self.extra {
            w.park.unpark();
        }
    }
}

/// Collect matching waiters from `bucket` into `batch`.
///
/// Iterates LIFO (`rev()`) and prefers already-spinning (`!is_sleeping()`) waiters
/// over deep-sleeping waiters for budgeted single-object wakes, turning kernel
/// `__ulock_wake` syscalls into cheap user-space spin hits. Multi-object wakes
/// coalesce redundant signals via `notified.swap(true, AcqRel)`.
#[inline]
fn collect_matching(bucket: &WaitBucket, match_key: Option<usize>, budget: &mut usize, batch: &mut WakeBatch) {
    if bucket.count.load(Ordering::Acquire) == 0 {
        return;
    }
    bucket.lock.lock();
    let entries = unsafe { &*bucket.entries.get() };
    // First pass (LIFO): wake all matching multi-object waiters and any matching
    // single-object waiters that are NOT yet sleeping (hot cache / spinning).
    // Defer sleeping single-object candidates only when budgeted.
    let limited_single = match_key.is_some() && *budget < usize::MAX;
    let mut deferred_sleeping = false;
    for (k, w) in entries.iter().rev() {
        if let Some(target) = match_key {
            if *k != target {
                continue;
            }
        }
        if !w.active.load(Ordering::Acquire) {
            continue;
        }
        if match_key.is_some() && w.single.load(Ordering::Relaxed) {
            if *budget == 0 {
                continue;
            }
            if limited_single && w.park.is_sleeping() {
                if !w.notified.load(Ordering::Acquire) {
                    deferred_sleeping = true;
                }
                continue;
            }
            if w.notified.swap(true, Ordering::AcqRel) {
                continue;
            }
            *budget -= 1;
            batch.push(Arc::clone(w));
        } else if !w.notified.swap(true, Ordering::AcqRel) {
            batch.push(Arc::clone(w));
        } else {
            park::record_coalesced_wake_stat();
        }
    }
    // Second pass (LIFO): if single-object budget remains, wake sleeping candidates.
    if deferred_sleeping && *budget > 0 {
        if let Some(target) = match_key {
            for (k, w) in entries.iter().rev() {
                if *budget == 0 {
                    break;
                }
                if *k != target || !w.active.load(Ordering::Acquire) || !w.single.load(Ordering::Relaxed) {
                    continue;
                }
                if w.notified.swap(true, Ordering::AcqRel) {
                    continue;
                }
                *budget -= 1;
                batch.push(Arc::clone(w));
            }
        }
    }
    bucket.lock.unlock();
}

/// Wake every blocked thread across all wait buckets (fallback for external
/// waitables that do not use targeted [`notify_object_key`] / [`notify_thread_apc`]).
pub fn notify_waiters() {
    let mut batch = WakeBatch::new();
    let mut unlimited = usize::MAX;
    collect_matching(&ANY_WAITERS, None, &mut unlimited, &mut batch);
    collect_matching(&ALERTABLE_WAITERS, None, &mut unlimited, &mut batch);
    for b in &BUCKETS {
        collect_matching(b, None, &mut unlimited, &mut batch);
    }
    batch.unpark_all();
}

/// Wake waiters registered on the specific waitable object identified by `key`
/// (`Waitable::wait_key`). Single-object waiters are woken at most `budget` at a
/// time (skipping any with a pending wake); multi-object / wait-all waiters are
/// always woken (coalescing duplicate pending wakes) so they re-check their
/// combined condition.
pub fn notify_object_key(key: usize, mut budget: usize) {
    // StoreLoad barrier pairing with `Registration::arm()`'s SeqCst fence so
    // any preceding state store (Event/Semaphore/Queue/Mutex/Thread) is strictly
    // ordered against our bucket count / waiter visibility checks.
    std::sync::atomic::fence(Ordering::SeqCst);
    let b = &BUCKETS[bucket_of(key)];
    let has_b = b.count.load(Ordering::Acquire) != 0;
    let has_any = ANY_WAITERS.count.load(Ordering::Acquire) != 0;
    if !has_b && !has_any {
        return;
    }
    let mut batch = WakeBatch::new();
    if has_b {
        collect_matching(b, Some(key), &mut budget, &mut batch);
    }
    if has_any {
        let mut unlimited = usize::MAX;
        collect_matching(&ANY_WAITERS, None, &mut unlimited, &mut batch);
    }
    batch.unpark_all();
}

/// Wake ONLY the specific thread `tid` if it is currently blocked in an
/// alertable wait or alertable sleep (used by `thread::queue_apc`).
pub fn notify_thread_apc(tid: u32) {
    std::sync::atomic::fence(Ordering::SeqCst);
    if ALERTABLE_WAITERS.count.load(Ordering::Acquire) == 0 {
        return;
    }
    let mut batch = WakeBatch::new();
    let target = tid as usize;
    ALERTABLE_WAITERS.lock.lock();
    let entries = unsafe { &*ALERTABLE_WAITERS.entries.get() };
    for (k, w) in entries.iter() {
        if *k == target && w.active.load(Ordering::Acquire) && !w.notified.swap(true, Ordering::AcqRel) {
            batch.push(Arc::clone(w));
        }
    }
    ALERTABLE_WAITERS.lock.unlock();
    batch.unpark_all();
}

#[inline]
pub fn obj_key<T: ?Sized>(p: *const T) -> usize {
    p as *const () as usize
}

/// Semaphore wake cascade (default; `PORT_SEM_CASCADE=0` / `AW2_SEM_CASCADE=0` for A/B):
/// releases coalesce to at most one in-flight kernel wake per burst (or one batched
/// `ULF_WAKE_ALL` when `n >= sleepers`); each woken waiter that leaves units behind
/// cascades to the next.
fn sem_cascade() -> bool {
    static C: OnceLock<bool> = OnceLock::new();
    *C.get_or_init(|| {
        let v = std::env::var("PORT_SEM_CASCADE")
            .ok()
            .or_else(|| std::env::var("AW2_SEM_CASCADE").ok());
        !v.is_some_and(|s| s.trim() == "0")
    })
}

/// Cascade-mode wake for complex waiters registered on the semaphore at `key`.
fn notify_semaphore_complex(key: usize) {
    std::sync::atomic::fence(Ordering::SeqCst);
    let b = &BUCKETS[bucket_of(key)];
    let has_b = b.count.load(Ordering::Acquire) != 0;
    let has_any = ANY_WAITERS.count.load(Ordering::Acquire) != 0;
    if !has_b && !has_any {
        return;
    }
    let mut batch = WakeBatch::new();
    if has_b {
        let mut pending = false;
        let mut spin_cand: Option<Arc<Waiter>> = None;
        let mut sleep_cand: Option<Arc<Waiter>> = None;
        b.lock.lock();
        let entries = unsafe { &*b.entries.get() };
        for (k, w) in entries.iter().rev() {
            if *k != key || !w.active.load(Ordering::Acquire) {
                continue;
            }
            if w.single.load(Ordering::Relaxed) {
                if w.notified.load(Ordering::Acquire) {
                    pending = true;
                } else if !w.park.is_sleeping() {
                    if spin_cand.is_none() {
                        spin_cand = Some(Arc::clone(w));
                    }
                } else if sleep_cand.is_none() {
                    sleep_cand = Some(Arc::clone(w));
                }
            } else if !w.notified.swap(true, Ordering::AcqRel) {
                batch.push(Arc::clone(w));
            }
        }
        if !pending {
            if let Some(w) = spin_cand.or(sleep_cand) {
                if !w.notified.swap(true, Ordering::AcqRel) {
                    batch.push(w);
                }
            }
        }
        b.lock.unlock();
    }
    if has_any {
        let mut unlimited = usize::MAX;
        collect_matching(&ANY_WAITERS, None, &mut unlimited, &mut batch);
    }
    batch.unpark_all();
}

/// Registration of the current thread for the duration of one bucketed wait.
struct Registration<'a> {
    objs: &'a [Arc<dyn KObject>],
    waiter: Arc<Waiter>,
    keys: [usize; MAX_KEYS],
    nkeys: usize,
    in_any: bool,
    alertable_tid: Option<u32>,
    single: bool,
    /// The wait consumed a signal; otherwise a pending targeted wake must be
    /// passed on to one other waiter when we leave.
    consumed: bool,
}

impl<'a> Registration<'a> {
    fn new(objs: &'a [Arc<dyn KObject>], wait_all: bool, alertable: bool, tid: u32) -> Self {
        let waiter = WAITER.try_with(Arc::clone).unwrap_or_else(|_| Arc::new(Waiter::new()));
        let single = objs.len() == 1 && !wait_all;
        waiter.single.store(single, Ordering::Relaxed);
        waiter.notified.store(false, Ordering::Relaxed);
        waiter.active.store(true, Ordering::Release);

        for o in objs {
            if let Some(s) = o.as_any().downcast_ref::<Semaphore>() {
                s.complex_waiters.fetch_add(1, Ordering::SeqCst);
            } else if let Some(e) = o.as_any().downcast_ref::<Event>() {
                e.complex_waiters.fetch_add(1, Ordering::SeqCst);
            }
        }

        let mut keys = [0usize; MAX_KEYS];
        let mut nkeys = 0usize;
        let mut in_any = false;

        if objs.len() > MAX_KEYS {
            in_any = true;
            ANY_WAITERS.insert(0, Arc::clone(&waiter));
        } else {
            for o in objs {
                let k = o.waitable().unwrap().wait_key();
                if !keys[..nkeys].contains(&k) {
                    keys[nkeys] = k;
                    nkeys += 1;
                    BUCKETS[bucket_of(k)].insert(k, Arc::clone(&waiter));
                }
            }
        }

        let alertable_tid = if alertable {
            ALERTABLE_WAITERS.insert(tid as usize, Arc::clone(&waiter));
            Some(tid)
        } else {
            None
        };

        Registration {
            objs,
            waiter,
            keys,
            nkeys,
            in_any,
            alertable_tid,
            single,
            consumed: false,
        }
    }

    /// Arm before checking condition / sleeping. `park.arm()` is ordered strictly
    /// BEFORE clearing `notified` so any notifier that observes `notified == false`
    /// via `swap(true, AcqRel)` is guaranteed to call `park.unpark()` strictly after
    /// `IDLE` was stored. The `SeqCst` fence pairs with lock-free notifiers.
    #[inline]
    fn arm(&self) {
        self.waiter.park.arm();
        self.waiter.notified.store(false, Ordering::Release);
        std::sync::atomic::fence(Ordering::SeqCst);
    }
}

impl Drop for Registration<'_> {
    fn drop(&mut self) {
        self.waiter.active.store(false, Ordering::Release);
        if self.in_any {
            ANY_WAITERS.remove(0, &self.waiter);
        } else {
            for &k in &self.keys[..self.nkeys] {
                BUCKETS[bucket_of(k)].remove(k, &self.waiter);
            }
        }
        if let Some(tid) = self.alertable_tid {
            ALERTABLE_WAITERS.remove(tid as usize, &self.waiter);
        }
        for o in self.objs {
            if let Some(s) = o.as_any().downcast_ref::<Semaphore>() {
                s.complex_waiters.fetch_sub(1, Ordering::Release);
            } else if let Some(e) = o.as_any().downcast_ref::<Event>() {
                e.complex_waiters.fetch_sub(1, Ordering::Release);
            }
        }
        // A budgeted wake aimed at us (e.g. one semaphore unit or auto-reset event)
        // that we are not going to use (timeout, APC) must reach one other waiter.
        if self.single
            && !self.consumed
            && self.nkeys > 0
            && self.waiter.notified.load(Ordering::Acquire)
        {
            notify_object_key(self.keys[0], 1);
        }
    }
}

/// A cell whose contents may only be touched while holding the wait lock.
pub struct Guarded<T>(std::cell::UnsafeCell<T>);
unsafe impl<T: Send> Sync for Guarded<T> {}
unsafe impl<T: Send> Send for Guarded<T> {}
impl<T> Guarded<T> {
    pub const fn new(v: T) -> Self {
        Guarded(std::cell::UnsafeCell::new(v))
    }
    #[allow(clippy::mut_from_ref)]
    pub fn get<'a>(&'a self, _g: &'a WaitGuard) -> &'a mut T {
        unsafe { &mut *self.0.get() }
    }
}

// ---------------------------------------------------------------------------
// Lock-free 2-level handle table with per-thread hazard pointers
// ---------------------------------------------------------------------------

pub const HANDLE_FLAG_INHERIT: u32 = 0x1;
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: u32 = 0x2;

pub const CURRENT_PROCESS: HANDLE = usize::MAX as HANDLE; // -1
pub const CURRENT_THREAD: HANDLE = (usize::MAX - 1) as HANDLE; // -2

const HANDLE_BASE: usize = 0x100;
const CHUNK_BITS: usize = 10;
const CHUNK_SIZE: usize = 1 << CHUNK_BITS; // 1024 handles per chunk
const MAX_CHUNKS: usize = 4096; // 4,194,304 monotonic handles in lock-free fast path
const MAX_FAST_INDEX: usize = MAX_CHUNKS * CHUNK_SIZE;

const SLOT_LIVE: u32 = 1 << 31;
const SLOT_FLAGS_MASK: u32 = HANDLE_FLAG_INHERIT | HANDLE_FLAG_PROTECT_FROM_CLOSE;

struct HandleSlot {
    /// Bit 31: `SLOT_LIVE` (1 iff slot currently holds an open handle).
    /// Bits 0..1: Win32 handle flags (`INHERIT` / `PROTECT_FROM_CLOSE`).
    state: AtomicU32,
    /// Inline object storage. Immutable while `SLOT_LIVE` is set; only dropped by
    /// `close()` after clearing `SLOT_LIVE`, issuing a `SeqCst` fence, and draining
    /// all active hazard pins matching this slot's handle value.
    obj: UnsafeCell<Option<Arc<dyn KObject>>>,
}

unsafe impl Sync for HandleSlot {}

impl HandleSlot {
    const fn new() -> Self {
        HandleSlot {
            state: AtomicU32::new(0),
            obj: UnsafeCell::new(None),
        }
    }
}

struct HandleChunk {
    slots: [HandleSlot; CHUNK_SIZE],
}

impl HandleChunk {
    fn new_boxed() -> *mut Self {
        let b: Box<Self> = Box::new(HandleChunk {
            slots: [const { HandleSlot::new() }; CHUNK_SIZE],
        });
        Box::into_raw(b)
    }
}

static CHUNKS: [AtomicPtr<HandleChunk>; MAX_CHUNKS] =
    [const { AtomicPtr::new(std::ptr::null_mut()) }; MAX_CHUNKS];
static LIVE_HANDLES: AtomicU32 = AtomicU32::new(0);

/// Cache-line-aligned per-thread hazard pointer slot (zero cross-core write
/// traffic during lock-free handle lookups).
#[repr(align(128))]
struct HazardSlot {
    /// Handle value currently pinned by this thread (`0` = none).
    pinned: AtomicUsize,
    /// Whether this slot is currently claimed by a live thread.
    in_use: AtomicBool,
}

impl HazardSlot {
    const fn new() -> Self {
        HazardSlot {
            pinned: AtomicUsize::new(0),
            in_use: AtomicBool::new(false),
        }
    }
}

const STATIC_HAZARD_SLOTS: usize = 256;
static HAZARD_STATIC: [HazardSlot; STATIC_HAZARD_SLOTS] =
    [const { HazardSlot::new() }; STATIC_HAZARD_SLOTS];
/// Highest index + 1 ever claimed in `HAZARD_STATIC` so `close()` scans only live slots.
static HAZARD_HIGH_WATER: AtomicUsize = AtomicUsize::new(0);

struct HazardOverflowNode {
    slot: HazardSlot,
    next: AtomicPtr<HazardOverflowNode>,
}

static HAZARD_OVERFLOW_HEAD: AtomicPtr<HazardOverflowNode> = AtomicPtr::new(std::ptr::null_mut());

struct ThreadHazardGuard {
    slot: &'static HazardSlot,
}

impl ThreadHazardGuard {
    fn acquire() -> Self {
        // Fast path: claim a free slot in the static 256-slot table.
        for (i, slot) in HAZARD_STATIC.iter().enumerate() {
            if !slot.in_use.load(Ordering::Relaxed)
                && slot
                    .in_use
                    .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
            {
                let mut hw = HAZARD_HIGH_WATER.load(Ordering::Relaxed);
                while i + 1 > hw {
                    match HAZARD_HIGH_WATER.compare_exchange_weak(
                        hw,
                        i + 1,
                        Ordering::Release,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => break,
                        Err(cur) => hw = cur,
                    }
                }
                return ThreadHazardGuard { slot };
            }
        }
        // Slow path (>256 concurrent threads): reuse or allocate an overflow node.
        let mut p = HAZARD_OVERFLOW_HEAD.load(Ordering::Acquire);
        while !p.is_null() {
            let node = unsafe { &*p };
            if !node.slot.in_use.load(Ordering::Relaxed)
                && node
                    .slot
                    .in_use
                    .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
            {
                return ThreadHazardGuard { slot: &node.slot };
            }
            p = node.next.load(Ordering::Acquire);
        }
        let boxed = Box::leak(Box::new(HazardOverflowNode {
            slot: HazardSlot {
                pinned: AtomicUsize::new(0),
                in_use: AtomicBool::new(true),
            },
            next: AtomicPtr::new(std::ptr::null_mut()),
        }));
        let raw = boxed as *mut HazardOverflowNode;
        let mut head = HAZARD_OVERFLOW_HEAD.load(Ordering::Acquire);
        loop {
            boxed.next.store(head, Ordering::Relaxed);
            match HAZARD_OVERFLOW_HEAD.compare_exchange_weak(
                head,
                raw,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(cur) => head = cur,
            }
        }
        ThreadHazardGuard { slot: &boxed.slot }
    }
}

impl Drop for ThreadHazardGuard {
    fn drop(&mut self) {
        self.slot.pinned.store(0, Ordering::Release);
        self.slot.in_use.store(false, Ordering::Release);
    }
}

thread_local! {
    static THREAD_HAZARD: ThreadHazardGuard = ThreadHazardGuard::acquire();
}

/// Wait until no thread holds a hazard pin on `hv`. Called by `close()` after
/// clearing `SLOT_LIVE` and issuing a `SeqCst` fence.
fn wait_hazards_clear(hv: usize) {
    let hw = HAZARD_HIGH_WATER.load(Ordering::Acquire).min(STATIC_HAZARD_SLOTS);
    for slot in &HAZARD_STATIC[..hw] {
        while slot.pinned.load(Ordering::Acquire) == hv {
            std::hint::spin_loop();
        }
    }
    let mut p = HAZARD_OVERFLOW_HEAD.load(Ordering::Acquire);
    while !p.is_null() {
        let node = unsafe { &*p };
        while node.slot.pinned.load(Ordering::Acquire) == hv {
            std::hint::spin_loop();
        }
        p = node.next.load(Ordering::Acquire);
    }
}

struct TableOverflow {
    next: usize,
    map: HashMap<usize, Arc<dyn KObject>>,
    flags: HashMap<usize, u32>,
}

/// Protects handle allocation (`next`), `close`, `set_flags`, and overflow
/// handles (`>= MAX_FAST_INDEX`). Never acquired by normal lock-free lookups.
#[inline]
fn table_mut() -> &'static RwLock<TableOverflow> {
    static TABLE: OnceLock<RwLock<TableOverflow>> = OnceLock::new();
    TABLE.get_or_init(|| {
        RwLock::new(TableOverflow {
            next: HANDLE_BASE,
            map: HashMap::new(),
            flags: HashMap::new(),
        })
    })
}

/// Bumped ONLY when an overflow sync handle (`>= MAX_FAST_INDEX`) is closed.
static OVERFLOW_SYNC_GEN: AtomicU64 = AtomicU64::new(0);

#[inline]
fn fast_slot(hv: usize) -> Option<&'static HandleSlot> {
    if (hv & 3) != 0 || hv < HANDLE_BASE {
        return None;
    }
    let idx = (hv - HANDLE_BASE) >> 2;
    if idx >= MAX_FAST_INDEX {
        return None;
    }
    let chunk = CHUNKS[idx >> CHUNK_BITS].load(Ordering::Acquire);
    if chunk.is_null() {
        return None;
    }
    Some(unsafe { &(*chunk).slots[idx & (CHUNK_SIZE - 1)] })
}

pub fn insert(obj: Arc<dyn KObject>) -> HANDLE {
    insert_with_flags(obj, 0)
}

/// Number of currently open kernel handles in the process handle table (lock-free).
pub fn live_count() -> u32 {
    LIVE_HANDLES.load(Ordering::Acquire)
}

pub fn insert_with_flags(obj: Arc<dyn KObject>, flags: u32) -> HANDLE {
    obj.on_handle_open();
    let masked_flags = flags & SLOT_FLAGS_MASK;
    let mut t = crate::thread::defer_write(table_mut());
    let hv = t.next;
    t.next += 4;
    let idx = (hv - HANDLE_BASE) >> 2;
    if idx < MAX_FAST_INDEX {
        let cidx = idx >> CHUNK_BITS;
        let sidx = idx & (CHUNK_SIZE - 1);
        let mut chunk = CHUNKS[cidx].load(Ordering::Relaxed);
        if chunk.is_null() {
            chunk = HandleChunk::new_boxed();
            CHUNKS[cidx].store(chunk, Ordering::Release);
        }
        let slot = unsafe { &(*chunk).slots[sidx] };
        unsafe { *slot.obj.get() = Some(obj) };
        slot.state.store(SLOT_LIVE | masked_flags, Ordering::Release);
    } else {
        t.map.insert(hv, obj);
        if masked_flags != 0 {
            t.flags.insert(hv, masked_flags);
        }
    }
    LIVE_HANDLES.fetch_add(1, Ordering::Relaxed);
    hv as HANDLE
}

pub fn get_flags(h: HANDLE) -> Option<u32> {
    if h == CURRENT_PROCESS || h == CURRENT_THREAD {
        return Some(0);
    }
    let hv = h as usize;
    if let Some(slot) = fast_slot(hv) {
        let s = slot.state.load(Ordering::Acquire);
        return if (s & SLOT_LIVE) != 0 {
            Some(s & SLOT_FLAGS_MASK)
        } else {
            None
        };
    }
    if (hv & 3) == 0 && hv >= HANDLE_BASE && ((hv - HANDLE_BASE) >> 2) >= MAX_FAST_INDEX {
        let t = crate::thread::defer_read(table_mut());
        if !t.map.contains_key(&hv) {
            return None;
        }
        return Some(t.flags.get(&hv).copied().unwrap_or(0));
    }
    None
}

pub fn set_flags(h: HANDLE, mask: u32, flags: u32) -> bool {
    if h == CURRENT_PROCESS || h == CURRENT_THREAD {
        return true;
    }
    let hv = h as usize;
    let mut t = crate::thread::defer_write(table_mut());
    if let Some(slot) = fast_slot(hv) {
        let cur = slot.state.load(Ordering::Relaxed);
        if (cur & SLOT_LIVE) == 0 {
            return false;
        }
        let next_flags = ((cur & !mask) | (flags & mask)) & SLOT_FLAGS_MASK;
        slot.state.store(SLOT_LIVE | next_flags, Ordering::Release);
        return true;
    }
    if !t.map.contains_key(&hv) {
        return false;
    }
    let cur = t.flags.get(&hv).copied().unwrap_or(0);
    let next = ((cur & !mask) | (flags & mask)) & SLOT_FLAGS_MASK;
    if next == 0 {
        t.flags.remove(&hv);
    } else {
        t.flags.insert(hv, next);
    }
    true
}

pub fn is_protected_from_close(h: HANDLE) -> bool {
    get_flags(h).is_some_and(|f| (f & HANDLE_FLAG_PROTECT_FROM_CLOSE) != 0)
}

#[inline]
fn is_hcache_type(o: &dyn KObject) -> bool {
    let any = o.as_any();
    any.is::<Semaphore>() || any.is::<Event>() || any.is::<MutexObj>() || any.is::<WaitableTimer>()
}

const HCACHE_LEN: usize = 256;

struct HEntry {
    h: usize,
    /// Direct pointer to the monotonic `HandleSlot::state` atomic (null if overflow).
    slot_state: *const AtomicU32,
    /// Generation check used only when `slot_state.is_null()`.
    overflow_gen: u64,
    obj: Option<Arc<dyn KObject>>,
}

impl HEntry {
    const fn empty() -> Self {
        HEntry {
            h: 0,
            slot_state: std::ptr::null(),
            overflow_gen: 0,
            obj: None,
        }
    }

    #[inline(always)]
    fn is_live(&self, hv: usize) -> bool {
        if self.h != hv {
            return false;
        }
        if !self.slot_state.is_null() {
            (unsafe { (*self.slot_state).load(Ordering::Acquire) } & SLOT_LIVE) != 0
        } else {
            self.overflow_gen == OVERFLOW_SYNC_GEN.load(Ordering::Acquire)
        }
    }
}

thread_local! {
    static HCACHE: UnsafeCell<[HEntry; HCACHE_LEN]> =
        const { UnsafeCell::new([const { HEntry::empty() }; HCACHE_LEN]) };
    #[cfg(debug_assertions)]
    static HCACHE_BORROWED: Cell<u32> = const { Cell::new(0) };
}

#[inline]
fn hcache_insert(hv: usize, slot_state: *const AtomicU32, overflow_gen: u64, obj: Arc<dyn KObject>) {
    let slot_idx = (hv >> 2) & (HCACHE_LEN - 1);
    let _ = HCACHE.try_with(|c| {
        #[cfg(debug_assertions)]
        debug_assert_eq!(
            HCACHE_BORROWED.with(Cell::get),
            0,
            "HCACHE mutated while sync reference borrowed"
        );
        let old = unsafe {
            std::mem::replace(
                &mut (*c.get())[slot_idx],
                HEntry {
                    h: hv,
                    slot_state,
                    overflow_gen,
                    obj: Some(obj),
                },
            )
        };
        drop(old);
    });
}

/// Lock-free lookup returning `(Arc<dyn KObject>, slot_state_ptr, overflow_gen)`.
fn lookup_uncached(hv: usize) -> Option<(Arc<dyn KObject>, *const AtomicU32, u64)> {
    if let Some(slot) = fast_slot(hv) {
        if (slot.state.load(Ordering::Acquire) & SLOT_LIVE) == 0 {
            return None;
        }
        let state_ptr = &slot.state as *const AtomicU32;
        if let Ok(found) = THREAD_HAZARD.try_with(|hz| {
            let _defer = crate::thread::TerminateDeferGuard::enter();
            hz.slot.pinned.store(hv, Ordering::Relaxed);
            std::sync::atomic::fence(Ordering::SeqCst);
            let res = if (slot.state.load(Ordering::Acquire) & SLOT_LIVE) != 0 {
                unsafe { (*slot.obj.get()).as_ref().cloned() }
            } else {
                None
            };
            hz.slot.pinned.store(0, Ordering::Release);
            res
        }) {
            return found.map(|o| (o, state_ptr, 0));
        }
        // Fallback during thread TLS teardown: serialize with `table_mut` read lock.
        let _t = crate::thread::defer_read(table_mut());
        if (slot.state.load(Ordering::Acquire) & SLOT_LIVE) != 0 {
            return unsafe { (*slot.obj.get()).as_ref().cloned() }.map(|o| (o, state_ptr, 0));
        }
        return None;
    }
    if (hv & 3) == 0 && hv >= HANDLE_BASE && ((hv - HANDLE_BASE) >> 2) >= MAX_FAST_INDEX {
        let gen = OVERFLOW_SYNC_GEN.load(Ordering::Acquire);
        let o = crate::thread::defer_read(table_mut()).map.get(&hv).cloned()?;
        return Some((o, std::ptr::null(), gen));
    }
    None
}

/// Zero-`Arc`-clone lookup for non-reentrant operations on cached sync objects
/// (`Event`, `Semaphore`). Uses the thread-local validated `HCACHE` with a single
/// read-only `Acquire` load of `HandleSlot::state` — zero `Arc::clone`, zero
/// `Arc::drop`, zero locks, zero `SeqCst` fences on hit!
///
/// SAFETY CONTRACT for `f`: `f` must NEVER call `handles::get` / `handles::close` /
/// guest code (anything that could mutate `HCACHE` on this thread).
#[inline]
fn with_sync_cached<R>(h: HANDLE, f: impl FnOnce(&Arc<dyn KObject>) -> R) -> Option<R> {
    let hv = h as usize;
    let slot_idx = (hv >> 2) & (HCACHE_LEN - 1);
    let mut hit_out: Option<R> = None;
    let mut hit_f = Some(f);
    let hit = HCACHE
        .try_with(|c| {
            let e = unsafe { &(*c.get())[slot_idx] };
            if e.is_live(hv) {
                if let Some(ref obj) = e.obj {
                    #[cfg(debug_assertions)]
                    HCACHE_BORROWED.with(|b| b.set(b.get() + 1));
                    let r = hit_f.take().unwrap()(obj);
                    #[cfg(debug_assertions)]
                    HCACHE_BORROWED.with(|b| b.set(b.get() - 1));
                    hit_out = Some(r);
                    return true;
                }
            }
            false
        })
        .unwrap_or(false);
    if hit {
        return hit_out;
    }
    let f = hit_f.unwrap();
    let (obj, state_ptr, gen) = lookup_uncached(hv)?;
    if is_hcache_type(&*obj) {
        hcache_insert(hv, state_ptr, gen, Arc::clone(&obj));
    }
    Some(f(&obj))
}

/// Zero-`Arc`-clone helper for `SetEvent` / `ResetEvent` / `PulseEvent` / `NtSetEvent` / `NtClearEvent`.
#[inline]
pub fn with_event<R>(h: HANDLE, f: impl FnOnce(&Event) -> R) -> Option<R> {
    let mut out = None;
    let _ = with_sync_cached(h, |obj| {
        if let Some(ev) = obj.as_any().downcast_ref::<Event>() {
            out = Some(f(ev));
        }
    });
    out
}

/// Zero-`Arc`-clone helper for `ReleaseSemaphore` / `NtReleaseSemaphore`.
#[inline]
pub fn with_semaphore<R>(h: HANDLE, f: impl FnOnce(&Semaphore) -> R) -> Option<R> {
    let mut out = None;
    let _ = with_sync_cached(h, |obj| {
        if let Some(sem) = obj.as_any().downcast_ref::<Semaphore>() {
            out = Some(f(sem));
        }
    });
    out
}

/// Zero-`Arc`-clone single-handle wait entry point for `WaitForSingleObjectEx` and
/// `NtWaitForSingleObject`.
///
/// For non-alertable waits on `Semaphore` or `Event`, borrows the validated
/// `HCACHE` entry directly and runs the direct futex fast path with zero `Arc`
/// refcount bouncing and zero global locks. Falls back to `wait_for_duration` for
/// alertable waits or other waitable types (`MutexObj`, `ThreadObj`, `WaitableTimer`, ...).
pub fn wait_single_handle(h: HANDLE, timeout: Option<Duration>, alertable: bool) -> WaitResult {
    if !alertable && h != CURRENT_THREAD && h != CURRENT_PROCESS {
        let mut fast_res: Option<WaitResult> = None;
        let mut fallback_obj: Option<Arc<dyn KObject>> = None;
        let found = with_sync_cached(h, |obj| {
            if let Some(s) = obj.as_any().downcast_ref::<Semaphore>() {
                fast_res = Some(s.wait_single_fast(timeout));
            } else if let Some(e) = obj.as_any().downcast_ref::<Event>() {
                fast_res = Some(e.wait_single_fast(timeout));
            } else {
                fallback_obj = Some(Arc::clone(obj));
            }
        });
        if let Some(res) = fast_res {
            return res;
        }
        if let Some(obj) = fallback_obj {
            return wait_for_duration(std::slice::from_ref(&obj), false, timeout, false);
        }
        if found.is_none() {
            return WaitResult::Failed(err::ERROR_INVALID_HANDLE);
        }
    }
    match get(h) {
        Some(obj) => wait_for_duration(std::slice::from_ref(&obj), false, timeout, alertable),
        None => WaitResult::Failed(err::ERROR_INVALID_HANDLE),
    }
}

pub fn get(h: HANDLE) -> Option<Arc<dyn KObject>> {
    if h == CURRENT_THREAD {
        return crate::thread::current_thread_object().map(|t| t as Arc<dyn KObject>);
    }
    let hv = h as usize;
    let slot_idx = (hv >> 2) & (HCACHE_LEN - 1);
    if let Ok(Some(o)) = HCACHE.try_with(|c| {
        let e = unsafe { &(*c.get())[slot_idx] };
        if e.is_live(hv) {
            e.obj.clone()
        } else {
            None
        }
    }) {
        return Some(o);
    }
    let (o, state_ptr, gen) = lookup_uncached(hv)?;
    // Only synchronization objects (trivial Drop: a cached reference may
    // outlive CloseHandle until the entry is next touched).
    if is_hcache_type(&*o) {
        hcache_insert(hv, state_ptr, gen, Arc::clone(&o));
    }
    Some(o)
}

pub fn get_as<T: KObject>(h: HANDLE) -> Option<Arc<T>> {
    let o = get(h)?;
    if o.as_any().is::<T>() {
        // SAFETY: type checked above; Arc<dyn KObject> -> Arc<T>.
        let raw = Arc::into_raw(o) as *const T;
        Some(unsafe { Arc::from_raw(raw) })
    } else {
        None
    }
}

pub fn close(h: HANDLE) -> bool {
    if h == CURRENT_PROCESS || h == CURRENT_THREAD {
        return true;
    }
    let hv = h as usize;
    // Drop the object (arbitrary Drop code) after releasing the table write lock.
    let removed = {
        let mut t = crate::thread::defer_write(table_mut());
        if let Some(slot) = fast_slot(hv) {
            let cur = slot.state.load(Ordering::Relaxed);
            if (cur & SLOT_LIVE) == 0 || (cur & HANDLE_FLAG_PROTECT_FROM_CLOSE) != 0 {
                return false;
            }
            slot.state.store(0, Ordering::Release);
            // Pair with reader's `pinned.store(hv); fence(SeqCst); slot.state.load(Acquire)`.
            std::sync::atomic::fence(Ordering::SeqCst);
            wait_hazards_clear(hv);
            let r = unsafe { (*slot.obj.get()).take() };
            if r.is_some() {
                LIVE_HANDLES.fetch_sub(1, Ordering::Relaxed);
            }
            r
        } else {
            if t.flags
                .get(&hv)
                .is_some_and(|&f| (f & HANDLE_FLAG_PROTECT_FROM_CLOSE) != 0)
            {
                return false;
            }
            let r = t.map.remove(&hv);
            if let Some(ref obj) = r {
                t.flags.remove(&hv);
                LIVE_HANDLES.fetch_sub(1, Ordering::Relaxed);
                if is_hcache_type(&**obj) {
                    OVERFLOW_SYNC_GEN.fetch_add(1, Ordering::Release);
                }
            }
            r
        }
    };
    if let Some(obj) = removed {
        obj.on_handle_close();
        true
    } else {
        false
    }
}

pub fn duplicate(h: HANDLE) -> Option<HANDLE> {
    let o = get(h)?;
    Some(insert(o))
}

// ---------------------------------------------------------------------------
// Waiting
// ---------------------------------------------------------------------------

pub enum WaitResult {
    Object(usize),
    Abandoned(usize),
    Timeout,
    Failed(u32),
    /// An APC/IO completion interrupted an alertable wait.
    IoCompletion,
}

/// Lock-free single-object wait for `Semaphore` or `Event` (non-alertable):
/// delegates directly to the object's packed futex fast path (never touches
/// `WAIT_LOCK` or wait buckets).
fn wait_single_lockfree(obj: &Arc<dyn KObject>, timeout: Option<Duration>) -> Option<WaitResult> {
    if let Some(s) = obj.as_any().downcast_ref::<Semaphore>() {
        Some(s.wait_single_fast(timeout))
    } else {
        obj.as_any()
            .downcast_ref::<Event>()
            .map(|e| e.wait_single_fast(timeout))
    }
}

/// Try to satisfy `objs` once under `WAIT_LOCK`. Returns `Some((res, pass_on_key))`.
fn try_satisfy_locked(
    objs: &[Arc<dyn KObject>],
    wait_all: bool,
    g: &WaitGuard,
    tid: u32,
) -> Option<(WaitResult, Option<usize>)> {
    if wait_all {
        if objs.iter().all(|o| o.waitable().unwrap().is_signaled(g, tid)) {
            // Lock-free objects (Semaphore, auto-reset Event) can change without
            // WAIT_LOCK: acquire them first, all-or-nothing with rollback.
            let mut taken_sems = 0usize;
            let mut taken_evs = 0usize;
            let mut ok = true;
            for o in objs {
                if let Some(s) = o.as_any().downcast_ref::<Semaphore>() {
                    if !s.try_take() {
                        ok = false;
                        break;
                    }
                    taken_sems += 1;
                } else if let Some(e) = o.as_any().downcast_ref::<Event>() {
                    if !e.try_take() {
                        ok = false;
                        break;
                    }
                    if !e.manual_reset {
                        taken_evs += 1;
                    }
                }
            }
            if ok {
                let mut abandoned = false;
                for o in objs {
                    if !o.as_any().is::<Semaphore>() && !o.as_any().is::<Event>() {
                        abandoned |= o.waitable().unwrap().acquire(g, tid);
                    }
                }
                let res = if abandoned { WaitResult::Abandoned(0) } else { WaitResult::Object(0) };
                return Some((res, None));
            }
            for o in objs {
                if taken_sems == 0 && taken_evs == 0 {
                    break;
                }
                if taken_sems > 0 {
                    if let Some(s) = o.as_any().downcast_ref::<Semaphore>() {
                        s.untake(g);
                        taken_sems -= 1;
                        continue;
                    }
                }
                if taken_evs > 0 {
                    if let Some(e) = o.as_any().downcast_ref::<Event>() {
                        if !e.manual_reset {
                            e.untake();
                            taken_evs -= 1;
                        }
                    }
                }
            }
        }
        None
    } else {
        for (i, o) in objs.iter().enumerate() {
            let w = o.waitable().unwrap();
            if let Some(ab) = w.try_acquire(g, tid) {
                let pass_on = (sem_cascade()
                    && objs.len() == 1
                    && o.as_any().is::<Semaphore>()
                    && w.is_signaled(g, tid))
                .then(|| w.wait_key());
                let res = if ab { WaitResult::Abandoned(i) } else { WaitResult::Object(i) };
                return Some((res, pass_on));
            }
        }
        None
    }
}

/// Generic WaitForMultipleObjects with sub-millisecond `Option<Duration>` timeout
/// (`None` = INFINITE).
pub fn wait_for_duration(
    objs: &[Arc<dyn KObject>],
    wait_all: bool,
    timeout: Option<Duration>,
    alertable: bool,
) -> WaitResult {
    for o in objs {
        if o.waitable().is_none() {
            log::warn!("wait on non-waitable object type {}", o.type_name());
            return WaitResult::Failed(err::ERROR_INVALID_HANDLE);
        }
    }
    if objs.len() == 1 && !wait_all && !alertable {
        if let Some(r) = wait_single_lockfree(&objs[0], timeout) {
            return r;
        }
    }
    let tid = crate::teb::thread_id();
    if alertable && crate::thread::has_pending_apc() {
        crate::thread::run_pending_apcs();
        return WaitResult::IoCompletion;
    }
    let mut g = wait_lock();
    if let Some((res, pass_on)) = try_satisfy_locked(objs, wait_all, &g, tid) {
        drop(g);
        if let Some(k) = pass_on {
            notify_semaphore_complex(k);
        }
        return res;
    }
    if matches!(timeout, Some(d) if d.is_zero()) {
        return WaitResult::Timeout;
    }

    let deadline = timeout.map(|d| Instant::now() + d);
    let _wait_scope = crate::hitch::Scope::enter(crate::hitch::ActKind::HandleWait, objs.len() as u64);
    let _term_defer = crate::thread::TerminateDeferGuard::enter();
    let mut reg = Registration::new(objs, wait_all, alertable, tid);
    loop {
        // Arm BEFORE re-checking state so lock-free notifiers (Event::set,
        // Semaphore::release, queue_apc) racing with us cannot lose a wakeup.
        reg.arm();
        if alertable && crate::thread::has_pending_apc() {
            drop(reg);
            drop(g);
            drop(_term_defer);
            crate::thread::run_pending_apcs();
            return WaitResult::IoCompletion;
        }
        if let Some((res, pass_on)) = try_satisfy_locked(objs, wait_all, &g, tid) {
            reg.consumed = true;
            drop(reg);
            drop(g);
            if let Some(k) = pass_on {
                notify_semaphore_complex(k);
            }
            return res;
        }
        if crate::thread::is_current_terminate_requested() {
            return WaitResult::Timeout;
        }
        if deadline.is_some_and(|d| Instant::now() >= d) {
            return WaitResult::Timeout;
        }
        let park = Arc::clone(&reg.waiter);
        drop(g);
        park.park.park(deadline);
        if crate::thread::is_current_terminate_requested() {
            return WaitResult::Timeout;
        }
        g = wait_lock();
    }
}

/// Generic WaitForMultipleObjects. `timeout_ms == INFINITE` waits forever.
pub fn wait_for(objs: &[Arc<dyn KObject>], wait_all: bool, timeout_ms: u32, alertable: bool) -> WaitResult {
    let timeout = (timeout_ms != INFINITE).then(|| Duration::from_millis(timeout_ms as u64));
    wait_for_duration(objs, wait_all, timeout, alertable)
}

/// Sleep that can be interrupted by APCs (`SleepEx` / `NtDelayExecution` alertable).
/// Registers only in `ALERTABLE_WAITERS` keyed by `tid` (never `WAIT_LOCK` or `ANY_WAITERS`).
pub fn alertable_sleep_duration(timeout: Option<Duration>) -> bool {
    if crate::thread::has_pending_apc() {
        crate::thread::run_pending_apcs();
        return true;
    }
    if matches!(timeout, Some(d) if d.is_zero()) {
        std::thread::yield_now();
        if crate::thread::has_pending_apc() {
            crate::thread::run_pending_apcs();
            return true;
        }
        return false;
    }
    let tid = crate::teb::thread_id();
    let deadline = timeout.map(|d| Instant::now() + d);
    let _term_defer = crate::thread::TerminateDeferGuard::enter();
    let reg = Registration::new(&[], false, true, tid);
    loop {
        reg.arm();
        if crate::thread::has_pending_apc() {
            drop(reg);
            drop(_term_defer);
            crate::thread::run_pending_apcs();
            return true;
        }
        if crate::thread::is_current_terminate_requested() {
            return false;
        }
        if deadline.is_some_and(|d| Instant::now() >= d) {
            return false;
        }
        reg.waiter.park.park(deadline);
        if crate::thread::is_current_terminate_requested() {
            return false;
        }
    }
}

/// Sleep that can be interrupted by APCs (SleepEx alertable).
pub fn alertable_sleep(timeout_ms: u32) -> bool {
    let timeout = (timeout_ms != INFINITE).then(|| Duration::from_millis(timeout_ms as u64));
    alertable_sleep_duration(timeout)
}

// ---------------------------------------------------------------------------
// Basic waitable objects
// ---------------------------------------------------------------------------

// Packed Event::state (AtomicU32):
//   bit   0:     EV_SIGNALED
//   bits  1..16: sleepers (15 bits, up to 32767 threads in `__ulock_wait`)
//   bits 16..31: waking   (15 bits, in-flight kernel wakes)
const EV_SIGNALED: u32 = 1;
const EV_SLEEPER_SHIFT: u32 = 1;
const EV_SLEEPER_INC: u32 = 1 << EV_SLEEPER_SHIFT;
const EV_SLEEPER_MAX: u32 = 0x7FFF;
const EV_SLEEPER_MASK: u32 = EV_SLEEPER_MAX << EV_SLEEPER_SHIFT;
const EV_WAKING_SHIFT: u32 = 16;
const EV_WAKING_MASK: u32 = 0x7FFF << EV_WAKING_SHIFT;

#[inline(always)]
fn ev_pack(signaled: bool, sleepers: u32, waking: u32) -> u32 {
    (signaled as u32) | (sleepers << EV_SLEEPER_SHIFT) | (waking << EV_WAKING_SHIFT)
}

pub struct Event {
    pub manual_reset: bool,
    state: AtomicU32,
    /// Packed `(gen: u32 [low 32] | claimed: u32 [high 32])` for race-free `PulseEvent`.
    pulse_word: AtomicU64,
    /// Active multi-object or alertable waiters registered in `BUCKETS`/`ANY_WAITERS`.
    complex_waiters: AtomicU32,
    pub name: Option<String>,
}

impl Event {
    pub fn new(manual_reset: bool, initial: bool, name: Option<String>) -> Self {
        Event {
            manual_reset,
            state: AtomicU32::new(if initial { EV_SIGNALED } else { 0 }),
            pulse_word: AtomicU64::new(0),
            complex_waiters: AtomicU32::new(0),
            name,
        }
    }

    #[inline]
    pub fn try_take(&self) -> bool {
        if self.manual_reset {
            (self.state.load(Ordering::Acquire) & EV_SIGNALED) != 0
        } else {
            let mut s = self.state.load(Ordering::Relaxed);
            while (s & EV_SIGNALED) != 0 {
                match self.state.compare_exchange_weak(
                    s,
                    s & !EV_SIGNALED,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => return true,
                    Err(v) => s = v,
                }
            }
            false
        }
    }

    #[inline]
    fn try_claim_pulse(&self, my_gen: u32) -> bool {
        let mut pw = self.pulse_word.load(Ordering::Acquire);
        let cur_gen = pw as u32;
        if cur_gen == my_gen {
            return false;
        }
        if self.manual_reset {
            return true;
        }
        loop {
            let cur_gen = pw as u32;
            let claimed = (pw >> 32) as u32;
            if cur_gen == my_gen || claimed == cur_gen {
                return false;
            }
            let next = (cur_gen as u64) | ((cur_gen as u64) << 32);
            match self.pulse_word.compare_exchange_weak(
                pw,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(v) => pw = v,
            }
        }
    }

    /// Direct futex + adaptive bounded spin single-object non-alertable wait.
    pub(crate) fn wait_single_fast(&self, timeout: Option<Duration>) -> WaitResult {
        if self.try_take() {
            return WaitResult::Object(0);
        }
        if matches!(timeout, Some(d) if d.is_zero()) {
            return WaitResult::Timeout;
        }
        let my_gen = self.pulse_word.load(Ordering::Acquire) as u32;
        let deadline = timeout.map(|d| Instant::now() + d);

        // Adaptive bounded spin directly on `self.state` (zero registration, zero locks).
        if !crate::thread::is_current_terminate_requested()
            && park::spin_until_u32_bit0(&self.state, deadline, || {
                self.try_take() || self.try_claim_pulse(my_gen)
            })
        {
            return WaitResult::Object(0);
        }

        let _wait_scope = crate::hitch::Scope::enter(crate::hitch::ActKind::HandleWait, 1);
        let _term_defer = crate::thread::TerminateDeferGuard::enter();
        let mut sleeping = false;

        loop {
            let s = self.state.load(Ordering::Acquire);
            let signaled = (s & EV_SIGNALED) != 0;
            let sleepers = (s & EV_SLEEPER_MASK) >> EV_SLEEPER_SHIFT;
            let waking = (s & EV_WAKING_MASK) >> EV_WAKING_SHIFT;

            if signaled {
                let new_signaled = self.manual_reset;
                let new_sleepers = if sleeping { sleepers - 1 } else { sleepers };
                let new_waking = if sleeping {
                    waking.saturating_sub(1).min(new_sleepers)
                } else {
                    waking
                };
                let next = ev_pack(new_signaled, new_sleepers, new_waking);
                if next == s
                    || self
                        .state
                        .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
                        .is_ok()
                {
                    return WaitResult::Object(0);
                }
                continue;
            }

            if self.try_claim_pulse(my_gen) {
                if sleeping {
                    self.unregister_sleeper();
                }
                return WaitResult::Object(0);
            }

            let timed_out = crate::thread::is_current_terminate_requested()
                || deadline.is_some_and(|d| Instant::now() >= d);
            if timed_out {
                if sleeping {
                    self.unregister_sleeper();
                }
                return WaitResult::Timeout;
            }

            let expected = if !sleeping {
                if sleepers >= EV_SLEEPER_MAX {
                    std::thread::yield_now();
                    continue;
                }
                let next = s + EV_SLEEPER_INC;
                if self
                    .state
                    .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
                    .is_err()
                {
                    continue;
                }
                sleeping = true;
                // StoreLoad barrier pairing with `pulse()`'s `pulse_word` update before
                // checking `sleepers` on `self.state`.
                std::sync::atomic::fence(Ordering::SeqCst);
                if self.try_claim_pulse(my_gen) {
                    self.unregister_sleeper();
                    return WaitResult::Object(0);
                }
                next
            } else if waking > 0 {
                let next = ev_pack(false, sleepers, waking - 1);
                if self
                    .state
                    .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
                    .is_err()
                {
                    continue;
                }
                next
            } else {
                s
            };

            match park::ulock_wait32(&self.state, expected, deadline) {
                Some(sleep_us) => {
                    let hit_deadline = deadline.is_some_and(|d| Instant::now() >= d);
                    park::record_sleep_outcome(sleep_us, hit_deadline);
                }
                None => {
                    self.unregister_sleeper();
                    return WaitResult::Timeout;
                }
            }
        }
    }

    /// Unregister one direct sleeper cleanly; if the event became signaled while a
    /// wake was in flight and other sleepers remain, pass the wake on.
    fn unregister_sleeper(&self) {
        let mut s = self.state.load(Ordering::Acquire);
        loop {
            let signaled = (s & EV_SIGNALED) != 0;
            let sleepers = (s & EV_SLEEPER_MASK) >> EV_SLEEPER_SHIFT;
            let waking = (s & EV_WAKING_MASK) >> EV_WAKING_SHIFT;
            let new_sleepers = sleepers.saturating_sub(1);
            let mut new_waking = waking.saturating_sub(1).min(new_sleepers);
            let mut wake_op = WakeOp::None;
            if signaled && new_sleepers > 0 && new_waking == 0 {
                if self.manual_reset {
                    new_waking = new_sleepers;
                    wake_op = WakeOp::All;
                } else {
                    new_waking = 1;
                    wake_op = WakeOp::One;
                }
            }
            let next = ev_pack(signaled, new_sleepers, new_waking);
            if self
                .state
                .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                if !matches!(wake_op, WakeOp::None) {
                    park::execute_ulock_wake32(&self.state, wake_op);
                }
                return;
            }
            s = self.state.load(Ordering::Acquire);
        }
    }

    #[inline]
    fn untake(&self) {
        if !self.manual_reset {
            self.set();
        }
    }

    pub fn set(&self) {
        let _ = self.set_returning_prev();
    }

    /// Set the event and return whether it was previously signaled (`NtSetEvent`).
    /// Redundant `SetEvent` calls on an already-signaled event return immediately
    /// without redundant `__ulock_wake` syscalls or bucket locks.
    pub fn set_returning_prev(&self) -> bool {
        let mut s = self.state.load(Ordering::Relaxed);
        let wake_op;
        loop {
            if (s & EV_SIGNALED) != 0 {
                park::record_coalesced_wake_stat();
                return true;
            }
            let sleepers = (s & EV_SLEEPER_MASK) >> EV_SLEEPER_SHIFT;
            let waking = (s & EV_WAKING_MASK) >> EV_WAKING_SHIFT;
            let (next, op) = if sleepers == 0 {
                (ev_pack(true, 0, 0), WakeOp::None)
            } else if self.manual_reset {
                // Batch-wake every sleeping waiter in one `ULF_WAKE_ALL` syscall.
                (ev_pack(true, sleepers, sleepers), WakeOp::All)
            } else if waking > 0 {
                // An earlier wake is still in flight; coalesce without a duplicate syscall.
                (ev_pack(true, sleepers, waking), WakeOp::Coalesced)
            } else {
                (ev_pack(true, sleepers, 1), WakeOp::One)
            };
            match self
                .state
                .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
            {
                Ok(_) => {
                    wake_op = op;
                    break;
                }
                Err(v) => s = v,
            }
        }
        park::execute_ulock_wake32(&self.state, wake_op);
        std::sync::atomic::fence(Ordering::SeqCst);
        if self.complex_waiters.load(Ordering::Acquire) != 0
            || ANY_WAITERS.count.load(Ordering::Acquire) != 0
        {
            notify_object_key(obj_key(self), if self.manual_reset { usize::MAX } else { 1 });
        }
        false
    }

    pub fn reset(&self) {
        self.state.fetch_and(!EV_SIGNALED, Ordering::Release);
    }

    /// Reset the event and return whether it was previously signaled (`NtResetEvent`).
    pub fn reset_returning_prev(&self) -> bool {
        (self.state.fetch_and(!EV_SIGNALED, Ordering::AcqRel) & EV_SIGNALED) != 0
    }

    /// PulseEvent: release currently waiting threads (all for manual-reset, one
    /// for auto-reset) and leave the event non-signaled.
    pub fn pulse(&self) {
        // 1. Advance pulse generation so any currently waiting single-object thread
        //    succeeds race-free even after `EV_SIGNALED` is cleared.
        let mut pw = self.pulse_word.load(Ordering::Relaxed);
        loop {
            let cur_gen = pw as u32;
            let next_gen = cur_gen.wrapping_add(1);
            let next_claimed = if self.manual_reset { 0 } else { cur_gen };
            let next = (next_gen as u64) | ((next_claimed as u64) << 32);
            match self
                .pulse_word
                .compare_exchange_weak(pw, next, Ordering::AcqRel, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(v) => pw = v,
            }
        }
        std::sync::atomic::fence(Ordering::SeqCst);

        // 2. Clear `EV_SIGNALED` (invalidating any ARM64 WFE monitor on `self.state`)
        //    and wake sleeping direct waiters.
        let mut s = self.state.load(Ordering::Relaxed);
        let wake_op;
        loop {
            let sleepers = (s & EV_SLEEPER_MASK) >> EV_SLEEPER_SHIFT;
            let waking = (s & EV_WAKING_MASK) >> EV_WAKING_SHIFT;
            let (next, op) = if sleepers == 0 {
                (s & !EV_SIGNALED, WakeOp::None)
            } else if self.manual_reset {
                (ev_pack(false, sleepers, sleepers), WakeOp::All)
            } else if waking > 0 {
                (ev_pack(false, sleepers, waking), WakeOp::Coalesced)
            } else {
                (ev_pack(false, sleepers, 1), WakeOp::One)
            };
            if next == s {
                // Force an RMW store to invalidate any spinning core's exclusive monitor.
                let cur = self.state.fetch_and(!EV_SIGNALED, Ordering::AcqRel);
                if cur == s {
                    wake_op = op;
                    break;
                }
                s = cur;
                continue;
            }
            match self
                .state
                .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
            {
                Ok(_) => {
                    wake_op = op;
                    break;
                }
                Err(v) => s = v,
            }
        }
        park::execute_ulock_wake32(&self.state, wake_op);

        // 3. Also wake any multi-object / alertable complex waiters.
        std::sync::atomic::fence(Ordering::SeqCst);
        if self.complex_waiters.load(Ordering::Acquire) != 0
            || ANY_WAITERS.count.load(Ordering::Acquire) != 0
        {
            self.state.fetch_or(EV_SIGNALED, Ordering::Release);
            notify_object_key(obj_key(self), if self.manual_reset { usize::MAX } else { 1 });
            std::thread::yield_now();
            self.state.fetch_and(!EV_SIGNALED, Ordering::Release);
        }
    }
}

impl KObject for Event {
    fn type_name(&self) -> &'static str {
        "Event"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn Waitable> {
        Some(self)
    }
}

impl Waitable for Event {
    fn is_signaled(&self, _g: &WaitGuard, _tid: u32) -> bool {
        (self.state.load(Ordering::Acquire) & EV_SIGNALED) != 0
    }
    fn acquire(&self, _g: &WaitGuard, _tid: u32) -> bool {
        self.try_take();
        false
    }
    fn try_acquire(&self, _g: &WaitGuard, _tid: u32) -> Option<bool> {
        self.try_take().then_some(false)
    }
}

pub struct MutexObj {
    /// (owner tid, recursion count, abandoned)
    pub state: Guarded<(u32, u32, bool)>,
    pub name: Option<String>,
}

impl KObject for MutexObj {
    fn type_name(&self) -> &'static str {
        "Mutex"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn Waitable> {
        Some(self)
    }
}

impl Waitable for MutexObj {
    fn is_signaled(&self, g: &WaitGuard, tid: u32) -> bool {
        let s = self.state.get(g);
        s.0 == 0 || s.0 == tid
    }
    fn acquire(&self, g: &WaitGuard, tid: u32) -> bool {
        let s = self.state.get(g);
        s.0 = tid;
        s.1 += 1;
        std::mem::take(&mut s.2)
    }
}

impl MutexObj {
    pub fn release(&self, tid: u32) -> bool {
        let g = wait_lock();
        let s = self.state.get(&g);
        if s.0 != tid {
            return false;
        }
        s.1 -= 1;
        if s.1 == 0 {
            s.0 = 0;
            drop(g);
            notify_object_key(obj_key(self), 1);
        }
        true
    }
}

// Packed Semaphore::state (AtomicU64):
//   bits  0..32: count    (u32, 0..=i32::MAX)
//   bits 32..48: sleepers (u16, threads actively sleeping in `__ulock_wait`)
//   bits 48..64: waking   (u16, in-flight kernel wakes not yet observed by a sleeper)
const SEM_COUNT_MASK: u64 = 0x0000_0000_FFFF_FFFF;
const SEM_SLEEPER_SHIFT: u32 = 32;
const SEM_SLEEPER_INC: u64 = 1u64 << SEM_SLEEPER_SHIFT;
const SEM_SLEEPER_MASK: u64 = 0x0000_FFFF_0000_0000;
const SEM_WAKING_SHIFT: u32 = 48;
const SEM_WAKING_MASK: u64 = 0xFFFF_0000_0000_0000;

#[inline(always)]
fn sem_pack(count: u32, sleepers: u16, waking: u16) -> u64 {
    (count as u64) | ((sleepers as u64) << SEM_SLEEPER_SHIFT) | ((waking as u64) << SEM_WAKING_SHIFT)
}

/// Lock-free semaphore with packed 64-bit `(count, sleepers, waking)` futex state:
/// single-object waits spin adaptively and block directly on `state` via
/// `UL_COMPARE_AND_WAIT64`, coalescing burst `ReleaseSemaphore` wakes and batching
/// multi-unit wakes with `ULF_WAKE_ALL` when `n >= sleepers`.
pub struct Semaphore {
    state: AtomicU64,
    /// Active multi-object or alertable waiters registered in `BUCKETS`/`ANY_WAITERS`.
    complex_waiters: AtomicU32,
    pub max: i32,
    pub name: Option<String>,
}

impl KObject for Semaphore {
    fn type_name(&self) -> &'static str {
        "Semaphore"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn Waitable> {
        Some(self)
    }
}

impl Waitable for Semaphore {
    fn is_signaled(&self, _g: &WaitGuard, _tid: u32) -> bool {
        (self.state.load(Ordering::Acquire) & SEM_COUNT_MASK) > 0
    }
    fn acquire(&self, _g: &WaitGuard, _tid: u32) -> bool {
        self.try_take();
        false
    }
    fn try_acquire(&self, _g: &WaitGuard, _tid: u32) -> Option<bool> {
        self.try_take().then_some(false)
    }
}

impl Semaphore {
    pub fn new(initial: i32, max: i32, name: Option<String>) -> Self {
        Semaphore {
            state: AtomicU64::new(initial.max(0) as u64),
            complex_waiters: AtomicU32::new(0),
            max,
            name,
        }
    }

    /// Take one unit if available (no lock, single CAS).
    #[inline]
    pub fn try_take(&self) -> bool {
        let mut s = self.state.load(Ordering::Relaxed);
        while (s & SEM_COUNT_MASK) != 0 {
            match self
                .state
                .compare_exchange_weak(s, s - 1, Ordering::Acquire, Ordering::Relaxed)
            {
                Ok(_) => return true,
                Err(v) => s = v,
            }
        }
        false
    }

    /// Direct futex + adaptive bounded spin single-object non-alertable wait.
    pub(crate) fn wait_single_fast(&self, timeout: Option<Duration>) -> WaitResult {
        if self.try_take() {
            return WaitResult::Object(0);
        }
        if matches!(timeout, Some(d) if d.is_zero()) {
            return WaitResult::Timeout;
        }
        let deadline = timeout.map(|d| Instant::now() + d);

        // Adaptive bounded spin directly on `self.state` (zero registration, zero locks).
        if !crate::thread::is_current_terminate_requested()
            && park::spin_until_u64_low32(&self.state, deadline, || {
                (self.state.load(Ordering::Relaxed) & SEM_COUNT_MASK) != 0 && self.try_take()
            })
        {
            return WaitResult::Object(0);
        }

        let _wait_scope = crate::hitch::Scope::enter(crate::hitch::ActKind::HandleWait, 1);
        let _term_defer = crate::thread::TerminateDeferGuard::enter();
        let mut sleeping = false;
        let cascade = sem_cascade();

        loop {
            let s = self.state.load(Ordering::Acquire);
            let count = (s & SEM_COUNT_MASK) as u32;
            let sleepers = ((s & SEM_SLEEPER_MASK) >> SEM_SLEEPER_SHIFT) as u16;
            let waking = ((s & SEM_WAKING_MASK) >> SEM_WAKING_SHIFT) as u16;

            if count > 0 {
                let new_count = count - 1;
                let new_sleepers = if sleeping { sleepers - 1 } else { sleepers };
                let mut new_waking = if sleeping {
                    waking.saturating_sub(1).min(new_sleepers)
                } else {
                    waking
                };
                let mut wake_op = WakeOp::None;
                if sleeping && cascade && new_count > 0 && new_sleepers > 0 && new_waking == 0 {
                    if new_count >= new_sleepers as u32 && new_sleepers > 1 {
                        new_waking = new_sleepers;
                        wake_op = WakeOp::All;
                    } else {
                        new_waking = 1;
                        wake_op = WakeOp::One;
                    }
                }
                let next = sem_pack(new_count, new_sleepers, new_waking);
                if self
                    .state
                    .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
                {
                    if !matches!(wake_op, WakeOp::None) {
                        park::execute_ulock_wake64(&self.state, wake_op);
                    }
                    if sleeping && cascade && new_count > 0 {
                        std::sync::atomic::fence(Ordering::SeqCst);
                        if self.complex_waiters.load(Ordering::Acquire) != 0
                            || ANY_WAITERS.count.load(Ordering::Acquire) != 0
                        {
                            notify_semaphore_complex(obj_key(self));
                        }
                    }
                    return WaitResult::Object(0);
                }
                continue;
            }

            let timed_out = crate::thread::is_current_terminate_requested()
                || deadline.is_some_and(|d| Instant::now() >= d);
            if timed_out {
                if sleeping {
                    self.unregister_sleeper();
                }
                return WaitResult::Timeout;
            }

            let expected = if !sleeping {
                if sleepers == u16::MAX {
                    std::thread::yield_now();
                    continue;
                }
                let next = s + SEM_SLEEPER_INC;
                if self
                    .state
                    .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
                    .is_err()
                {
                    continue;
                }
                sleeping = true;
                next
            } else if waking > 0 {
                // Woke from `__ulock_wait` but count was already taken (e.g. by a spinner):
                // clear our waking token before sleeping again so future releases wake us.
                let next = sem_pack(0, sleepers, waking - 1);
                if self
                    .state
                    .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
                    .is_err()
                {
                    continue;
                }
                next
            } else {
                s
            };

            match park::ulock_wait64(&self.state, expected, deadline) {
                Some(sleep_us) => {
                    let hit_deadline = deadline.is_some_and(|d| Instant::now() >= d);
                    park::record_sleep_outcome(sleep_us, hit_deadline);
                }
                None => {
                    self.unregister_sleeper();
                    return WaitResult::Timeout;
                }
            }
        }
    }

    /// Unregister one direct sleeper on timeout/termination, passing on a wake if
    /// units and other sleepers remain.
    fn unregister_sleeper(&self) {
        let mut s = self.state.load(Ordering::Acquire);
        loop {
            let count = (s & SEM_COUNT_MASK) as u32;
            let sleepers = ((s & SEM_SLEEPER_MASK) >> SEM_SLEEPER_SHIFT) as u16;
            let waking = ((s & SEM_WAKING_MASK) >> SEM_WAKING_SHIFT) as u16;
            let new_sleepers = sleepers.saturating_sub(1);
            let mut new_waking = waking.saturating_sub(1).min(new_sleepers);
            let mut wake_op = WakeOp::None;
            if count > 0 && new_sleepers > 0 && new_waking == 0 {
                if count >= new_sleepers as u32 && new_sleepers > 1 {
                    new_waking = new_sleepers;
                    wake_op = WakeOp::All;
                } else {
                    new_waking = 1;
                    wake_op = WakeOp::One;
                }
            }
            let next = sem_pack(count, new_sleepers, new_waking);
            if self
                .state
                .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                if !matches!(wake_op, WakeOp::None) {
                    park::execute_ulock_wake64(&self.state, wake_op);
                }
                return;
            }
            s = self.state.load(Ordering::Acquire);
        }
    }

    /// Return a unit taken by a wait-all that could not complete (lock held).
    fn untake(&self, _g: &WaitGuard) {
        let _ = self.release(1);
    }

    /// Returns previous count or None if it would exceed max.
    pub fn release(&self, n: i32) -> Option<i32> {
        if n <= 0 {
            return None;
        }
        let cascade = sem_cascade();
        let mut s = self.state.load(Ordering::Relaxed);
        let prev_count;
        let wake_op;
        loop {
            let cur = (s & SEM_COUNT_MASK) as i32;
            let new_count = match cur.checked_add(n) {
                Some(v) if v <= self.max => v as u32,
                _ => return None,
            };
            let sleepers = ((s & SEM_SLEEPER_MASK) >> SEM_SLEEPER_SHIFT) as u16;
            let waking = ((s & SEM_WAKING_MASK) >> SEM_WAKING_SHIFT) as u16;
            let (new_waking, op) = if sleepers == 0 {
                (0, WakeOp::None)
            } else if cascade {
                if waking > 0 {
                    // Coalesce burst releases: a wake is already in flight and will
                    // cascade if units remain when that sleeper runs.
                    (waking, WakeOp::Coalesced)
                } else if (n as u32) >= sleepers as u32 && sleepers > 1 {
                    // Batch-wake all sleepers in one `ULF_WAKE_ALL` syscall.
                    (sleepers, WakeOp::All)
                } else {
                    (1, WakeOp::One)
                }
            } else {
                let avail = sleepers.saturating_sub(waking) as usize;
                let need = (n as usize).min(avail);
                if need == 0 {
                    (waking, WakeOp::Coalesced)
                } else if need == avail && sleepers > 1 && (n as u32) >= sleepers as u32 {
                    (sleepers, WakeOp::All)
                } else if need == 1 {
                    (waking + 1, WakeOp::One)
                } else {
                    (waking + need as u16, WakeOp::Many(need))
                }
            };
            let next = sem_pack(new_count, sleepers, new_waking);
            match self
                .state
                .compare_exchange_weak(s, next, Ordering::AcqRel, Ordering::Relaxed)
            {
                Ok(_) => {
                    prev_count = cur;
                    wake_op = op;
                    break;
                }
                Err(v) => s = v,
            }
        }
        park::execute_ulock_wake64(&self.state, wake_op);
        std::sync::atomic::fence(Ordering::SeqCst);
        if self.complex_waiters.load(Ordering::Acquire) != 0
            || ANY_WAITERS.count.load(Ordering::Acquire) != 0
        {
            if cascade {
                notify_semaphore_complex(obj_key(self));
            } else {
                notify_object_key(obj_key(self), n as usize);
            }
        }
        Some(prev_count)
    }
}

pub type TimerApcRoutine = unsafe extern "win64" fn(*mut std::ffi::c_void, u32, u32);

pub struct WaitableTimer {
    pub manual_reset: bool,
    pub signaled: Guarded<bool>,
    pub generation: Guarded<u64>,
    pub name: Option<String>,
}

impl WaitableTimer {
    pub fn new(manual_reset: bool, name: Option<String>) -> Self {
        WaitableTimer {
            manual_reset,
            signaled: Guarded::new(false),
            generation: Guarded::new(0),
            name,
        }
    }

    pub fn cancel(&self) {
        let g = wait_lock();
        *self.generation.get(&g) += 1;
    }

    /// Notify waiters registered specifically on this timer.
    pub fn notify(&self) {
        notify_object_key(obj_key(self), if self.manual_reset { usize::MAX } else { 1 });
    }

    /// Mark timer signaled if `gen` still matches and wake waiters.
    pub fn fire_signaled(&self, gen: u64) -> bool {
        let g = wait_lock();
        if *self.generation.get(&g) != gen {
            return false;
        }
        *self.signaled.get(&g) = true;
        drop(g);
        self.notify();
        true
    }

    pub fn fire(&self, gen: u64, apc: Option<(crate::thread::ApcProc, usize, Arc<crate::thread::ThreadObj>)>) -> bool {
        let g = wait_lock();
        if *self.generation.get(&g) != gen {
            return false;
        }
        *self.signaled.get(&g) = true;
        drop(g);
        if let Some((proc_, param, ref target)) = apc {
            // queue_apc wakes the target's alertable wait itself.
            crate::thread::queue_apc(target, proc_, param);
        }
        self.notify();
        true
    }
}

impl KObject for WaitableTimer {
    fn type_name(&self) -> &'static str {
        "WaitableTimer"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn Waitable> {
        Some(self)
    }
}

impl Waitable for WaitableTimer {
    fn is_signaled(&self, g: &WaitGuard, _tid: u32) -> bool {
        *self.signaled.get(g)
    }
    fn acquire(&self, g: &WaitGuard, _tid: u32) -> bool {
        if !self.manual_reset {
            *self.signaled.get(g) = false;
        }
        false
    }
}

/// Named-object namespace (CreateEventW with a name, OpenEvent, ...).
pub fn named_objects() -> &'static Mutex<HashMap<String, Arc<dyn KObject>>> {
    static N: OnceLock<Mutex<HashMap<String, Arc<dyn KObject>>>> = OnceLock::new();
    N.get_or_init(|| Mutex::new(HashMap::new()))
}

/// A placeholder object for handles of things we only track (e.g. tokens).
pub struct Opaque(pub &'static str);
impl KObject for Opaque {
    fn type_name(&self) -> &'static str {
        self.0
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ---------------------------------------------------------------------------
// Generic typed handle registry (`HandleTable<T>`) for subsystem DLLs
// ---------------------------------------------------------------------------

/// Thread-safe, `const`-constructible handle table keyed by monotonically
/// allocated `usize` IDs (`base + k * step`).
///
/// Replaces per-DLL `Mutex<HashMap<usize, T>> + AtomicUsize` boilerplate across
/// WinHTTP, libcurl, Crypt32, FMOD, OpenAL, Bink2, Schannel, etc.
#[allow(dead_code)]
pub(crate) struct HandleTable<T> {
    next: AtomicUsize,
    step: usize,
    map: Mutex<Option<HashMap<usize, T>>>,
}

#[allow(dead_code)]
impl<T> HandleTable<T> {
    /// Create a new table allocating handle IDs starting at `base` with stride `step`
    /// (clamped to `>= 1`). Usable directly in `static` items without `OnceLock`.
    pub(crate) const fn new(base: usize, step: usize) -> Self {
        Self {
            next: AtomicUsize::new(if base == 0 { 1 } else { base }),
            step: if step == 0 { 1 } else { step },
            map: Mutex::new(None),
        }
    }

    fn alloc_id(&self) -> usize {
        loop {
            let id = self.next.fetch_add(self.step, Ordering::Relaxed);
            if id != 0 && id != usize::MAX {
                return id;
            }
        }
    }

    /// Insert `val` under a freshly allocated handle ID and return the ID.
    pub(crate) fn insert(&self, val: T) -> usize {
        let id = self.alloc_id();
        let mut g = crate::thread::defer_lock(&self.map);
        g.get_or_insert_with(HashMap::new).insert(id, val);
        id
    }

    /// Insert `val` at a caller-chosen key, returning any previously stored value.
    pub(crate) fn insert_at(&self, id: usize, val: T) -> Option<T> {
        let mut g = crate::thread::defer_lock(&self.map);
        g.get_or_insert_with(HashMap::new).insert(id, val)
    }

    /// Clone the value stored at `id`, if present.
    pub(crate) fn get_cloned(&self, id: usize) -> Option<T>
    where
        T: Clone,
    {
        let g = crate::thread::defer_lock(&self.map);
        g.as_ref()?.get(&id).cloned()
    }

    /// Borrow the value stored at `id` while holding the table lock.
    pub(crate) fn with<R>(&self, id: usize, f: impl FnOnce(&T) -> R) -> Option<R> {
        let g = crate::thread::defer_lock(&self.map);
        let val = g.as_ref()?.get(&id)?;
        Some(f(val))
    }

    /// Mutably borrow the value stored at `id` while holding the table lock.
    pub(crate) fn with_mut<R>(&self, id: usize, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        let mut g = crate::thread::defer_lock(&self.map);
        let val = g.as_mut()?.get_mut(&id)?;
        Some(f(val))
    }

    /// Remove and return the value stored at `id` (dropped outside the table lock).
    pub(crate) fn remove(&self, id: usize) -> Option<T> {
        let mut g = crate::thread::defer_lock(&self.map);
        g.as_mut()?.remove(&id)
    }

    /// Remove all entries from the table, dropping them outside the table lock.
    pub(crate) fn clear(&self) {
        let old = {
            let mut g = crate::thread::defer_lock(&self.map);
            g.take()
        };
        drop(old);
    }

    /// Retain only entries satisfying `predicate`.
    pub(crate) fn retain(&self, mut predicate: impl FnMut(&usize, &mut T) -> bool) {
        let mut g = crate::thread::defer_lock(&self.map);
        if let Some(m) = g.as_mut() {
            m.retain(|k, v| predicate(k, v));
        }
    }

    /// True if `id` is currently present in the table.
    pub(crate) fn contains(&self, id: usize) -> bool {
        let g = crate::thread::defer_lock(&self.map);
        g.as_ref().is_some_and(|m| m.contains_key(&id))
    }

    /// Number of live entries currently in the table.
    pub(crate) fn len(&self) -> usize {
        let g = crate::thread::defer_lock(&self.map);
        g.as_ref().map_or(0, HashMap::len)
    }

    /// True when the table holds no live entries.
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Snapshot all currently live handle IDs.
    pub(crate) fn keys(&self) -> Vec<usize> {
        let g = crate::thread::defer_lock(&self.map);
        g.as_ref()
            .map(|m| m.keys().copied().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waitable_timer_cached_and_invalidated_on_close() {
        let timer: Arc<dyn KObject> = Arc::new(WaitableTimer::new(false, None));
        let h = insert(timer.clone());
        let hv = h as usize;
        let slot = (hv >> 2) & (HCACHE_LEN - 1);

        // First get populates HCACHE for WaitableTimer.
        let got1 = get(h).expect("handle must resolve");
        assert!(Arc::ptr_eq(&got1, &timer));
        let cached = HCACHE.with(|c| {
            let e = unsafe { &(*c.get())[slot] };
            e.is_live(hv) && e.obj.as_ref().is_some_and(|o| Arc::ptr_eq(o, &timer))
        });
        assert!(cached, "WaitableTimer should be cached in HCACHE");

        // Closing an unrelated handle (sync or non-sync) must NOT invalidate `h`'s HCACHE entry.
        let non_sync = insert(Arc::new(Opaque("file_like")));
        assert!(close(non_sync));
        let other_ev = insert(Arc::new(Event::new(true, false, None)));
        assert!(close(other_ev));
        let still_cached = HCACHE.with(|c| {
            let e = unsafe { &(*c.get())[slot] };
            e.is_live(hv) && e.obj.as_ref().is_some_and(|o| Arc::ptr_eq(o, &timer))
        });
        assert!(still_cached, "closing unrelated handles must not invalidate cached handle");

        // Second get hits HCACHE and returns the same object.
        let got2 = get(h).expect("cached handle must resolve");
        assert!(Arc::ptr_eq(&got2, &timer));

        // close() clears SLOT_LIVE immediately, invalidating HCACHE.
        assert!(close(h));
        assert!(!close(h));
        assert!(get(h).is_none(), "closed WaitableTimer must not be returned from HCACHE");
    }

    #[test]
    fn close_drops_object_outside_table_lock() {
        struct ReentrantDrop {
            other: usize,
            ran: Arc<AtomicBool>,
        }
        impl KObject for ReentrantDrop {
            fn type_name(&self) -> &'static str {
                "ReentrantDrop"
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        impl Drop for ReentrantDrop {
            fn drop(&mut self) {
                assert!(get(self.other as HANDLE).is_some());
                let tmp = insert(Arc::new(Opaque("from_drop")));
                assert!(close(tmp));
                self.ran.store(true, Ordering::Release);
            }
        }

        let other = insert(Arc::new(Event::new(true, false, None)));
        let ran = Arc::new(AtomicBool::new(false));
        let h = insert(Arc::new(ReentrantDrop { other: other as usize, ran: ran.clone() }));
        assert!(close(h));
        assert!(ran.load(Ordering::Acquire));
        assert!(close(other));
    }

    #[test]
    fn concurrent_insert_get_close() {
        // Insert 96 shared objects across all 4 cached waitable types.
        let mut shared_vec: Vec<(usize, Arc<dyn KObject>)> = Vec::with_capacity(96);
        for i in 0..96usize {
            let obj: Arc<dyn KObject> = match i % 4 {
                0 => Arc::new(Semaphore::new(1, 4, None)),
                1 => Arc::new(Event::new(true, false, None)),
                2 => Arc::new(MutexObj { state: Guarded::new((0, 0, false)), name: None }),
                _ => Arc::new(WaitableTimer::new(false, None)),
            };
            let h = insert(obj.clone());
            shared_vec.push((h as usize, obj));
        }
        let shared = Arc::new(shared_vec);

        let mut threads = Vec::new();
        for tid in 0..8usize {
            let shared = shared.clone();
            threads.push(std::thread::spawn(move || {
                // Verify concurrent reads + HCACHE hits across all 96 shared handles.
                for round in 0..8 {
                    for (hv, expected) in shared.iter() {
                        let got = get(*hv as HANDLE).expect("shared handle must stay live");
                        assert!(Arc::ptr_eq(&got, expected));
                    }
                    // Insert per-thread handles, look them up, close half, verify invalidation.
                    let mut local: Vec<(usize, Arc<dyn KObject>)> = Vec::with_capacity(16);
                    for j in 0..16usize {
                        let obj: Arc<dyn KObject> = match (tid + round + j) % 4 {
                            0 => Arc::new(WaitableTimer::new(true, None)),
                            1 => Arc::new(Semaphore::new(0, 8, None)),
                            2 => Arc::new(Event::new(false, false, None)),
                            _ => Arc::new(MutexObj { state: Guarded::new((0, 0, false)), name: None }),
                        };
                        let h = insert(obj.clone());
                        local.push((h as usize, obj));
                    }
                    for (hv, expected) in &local {
                        let got = get(*hv as HANDLE).expect("local handle must resolve");
                        assert!(Arc::ptr_eq(&got, expected));
                    }
                    for (hv, _) in &local[..8] {
                        assert!(close(*hv as HANDLE));
                    }
                    for (hv, _) in &local[..8] {
                        assert!(get(*hv as HANDLE).is_none(), "closed local handle must return None");
                    }
                    for (hv, expected) in &local[8..] {
                        let got = get(*hv as HANDLE).expect("remaining local handle must resolve");
                        assert!(Arc::ptr_eq(&got, expected));
                    }
                    for (hv, _) in &local[8..] {
                        assert!(close(*hv as HANDLE));
                        assert!(get(*hv as HANDLE).is_none());
                    }
                }
            }));
        }

        for t in threads {
            t.join().unwrap();
        }
        for (hv, _) in shared.iter() {
            assert!(close(*hv as HANDLE));
            assert!(get(*hv as HANDLE).is_none());
        }
    }

    #[test]
    fn waitable_list_inline_and_spill_and_wait_all_rollback() {
        let mut list = WaitableList::with_capacity(4);
        for _ in 0..8 {
            list.push(Arc::new(Event::new(true, false, None)));
        }
        assert!(matches!(list, WaitableList::Inline { len: 8, .. }));
        assert_eq!(list.as_slice().len(), 8);
        list.push(Arc::new(Event::new(true, false, None)));
        assert!(matches!(list, WaitableList::Heap(_)));
        assert_eq!(list.as_slice().len(), 9);

        // wait_all rollback must restore both auto-reset Event and Semaphore when another object is unsignaled.
        let ev1 = Arc::new(Event::new(false, true, None));
        let sem = Arc::new(Semaphore::new(1, 1, None));
        let ev2 = Arc::new(Event::new(true, false, None));
        let objs: [Arc<dyn KObject>; 3] = [ev1.clone(), sem.clone(), ev2.clone()];
        assert!(matches!(wait_for(&objs, true, 0, false), WaitResult::Timeout));
        assert!(ev1.try_take(), "auto-reset event must remain signaled after wait_all rollback");
        assert!(sem.try_take(), "semaphore unit must remain available after wait_all rollback");
    }

    #[test]
    fn handle_table_crud_stride_and_clear_outside_lock() {
        let tbl: HandleTable<String> = HandleTable::new(0x5000, 4);
        assert!(tbl.is_empty());
        assert_eq!(tbl.len(), 0);

        let h1 = tbl.insert("alpha".to_string());
        let h2 = tbl.insert("beta".to_string());
        assert_eq!(h1, 0x5000);
        assert_eq!(h2, 0x5004);
        assert_eq!(tbl.len(), 2);
        assert!(tbl.contains(h1));
        assert_eq!(tbl.get_cloned(h1).as_deref(), Some("alpha"));
        assert_eq!(tbl.with(h2, |s| s.len()), Some(4));
        assert_eq!(
            tbl.with_mut(h2, |s| {
                s.push('!');
                s.clone()
            })
            .as_deref(),
            Some("beta!")
        );
        assert_eq!(tbl.get_cloned(h2).as_deref(), Some("beta!"));
        assert_eq!(tbl.get_cloned(0x9999), None);

        tbl.retain(|&k, _| k == h2);
        assert_eq!(tbl.len(), 1);
        assert!(!tbl.contains(h1));
        assert_eq!(tbl.keys(), vec![h2]);

        assert_eq!(tbl.remove(h2).as_deref(), Some("beta!"));
        assert!(tbl.is_empty());

        // insert_at + clear dropping entries cleanly.
        assert!(tbl.insert_at(0x7777, "pinned".to_string()).is_none());
        assert_eq!(tbl.insert_at(0x7777, "replaced".to_string()).as_deref(), Some("pinned"));
        assert_eq!(tbl.len(), 1);
        tbl.clear();
        assert!(tbl.is_empty());

        // Verify Drop during clear() runs outside the table lock (no deadlock).
        static DROP_TBL: HandleTable<DropProbe> = HandleTable::new(0x100, 1);
        struct DropProbe(Arc<AtomicBool>);
        impl Drop for DropProbe {
            fn drop(&mut self) {
                assert_eq!(DROP_TBL.len(), 0);
                self.0.store(true, Ordering::Release);
            }
        }
        let ran = Arc::new(AtomicBool::new(false));
        let _ = DROP_TBL.insert(DropProbe(ran.clone()));
        DROP_TBL.clear();
        assert!(ran.load(Ordering::Acquire));
    }

    #[test]
    fn auto_reset_event_stress_exact_conservation() {
        use std::sync::Barrier;

        let ev = Arc::new(Event::new(false, false, None));
        let h = insert(ev.clone());

        // 1. Redundant SetEvent calls before any waiter consumes must satisfy exactly 1 waiter.
        with_event(h, |e| {
            e.set();
            e.set();
            e.set();
        })
        .unwrap();
        assert!(matches!(wait_single_handle(h, Some(Duration::ZERO), false), WaitResult::Object(0)));
        assert!(matches!(wait_single_handle(h, Some(Duration::ZERO), false), WaitResult::Timeout));

        // 2. Exact 1:1 signal-to-wakeup conservation across 8 competing waiters and 400 rounds.
        let consumed = Arc::new(AtomicUsize::new(0));
        let exited = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        const WAITERS: usize = 8;
        const ROUNDS: usize = 400;
        let start = Arc::new(Barrier::new(WAITERS + 1));

        let mut threads = Vec::new();
        for _ in 0..WAITERS {
            let hv = h as usize;
            let c = Arc::clone(&consumed);
            let ex = Arc::clone(&exited);
            let st = Arc::clone(&stop);
            let b = Arc::clone(&start);
            threads.push(std::thread::spawn(move || {
                b.wait();
                loop {
                    let r = wait_single_handle(hv as HANDLE, Some(Duration::from_secs(5)), false);
                    assert!(matches!(r, WaitResult::Object(0)), "lost wakeup on auto-reset event");
                    if st.load(Ordering::Acquire) {
                        ex.fetch_add(1, Ordering::AcqRel);
                        break;
                    }
                    c.fetch_add(1, Ordering::AcqRel);
                }
            }));
        }

        start.wait();
        for round in 0..ROUNDS {
            with_event(h, |e| e.set()).unwrap();
            let target = round + 1;
            let dl = Instant::now() + Duration::from_secs(5);
            while consumed.load(Ordering::Acquire) < target {
                assert!(Instant::now() < dl, "auto-reset event waiter did not consume signal at round {round}");
                std::hint::spin_loop();
            }
            assert_eq!(consumed.load(Ordering::Acquire), target, "auto-reset event woke more than 1 waiter");
            if round % 50 == 0 {
                // Periodically let waiters park in `__ulock_wait` before next signal.
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        stop.store(true, Ordering::Release);
        // Auto-reset events are binary (not counting), so keep signaling until every waiter exits.
        let dl = Instant::now() + Duration::from_secs(5);
        while exited.load(Ordering::Acquire) < WAITERS {
            assert!(Instant::now() < dl, "waiter failed to exit on auto-reset shutdown");
            with_event(h, |e| e.set()).unwrap();
            std::thread::yield_now();
        }
        for t in threads {
            t.join().unwrap();
        }
        assert_eq!(consumed.load(Ordering::Acquire), ROUNDS);
        assert!(close(h));
    }

    #[test]
    fn manual_reset_event_batch_wake_and_reset_stress() {
        use std::sync::Barrier;

        let ev = Arc::new(Event::new(true, false, None));
        let h = insert(ev.clone());
        const WAITERS: usize = 10;
        const ROUNDS: usize = 60;
        let round_gate = Arc::new(Barrier::new(WAITERS + 1));
        let done_gate = Arc::new(Barrier::new(WAITERS + 1));

        let mut threads = Vec::new();
        for _ in 0..WAITERS {
            let hv = h as usize;
            let rg = Arc::clone(&round_gate);
            let dg = Arc::clone(&done_gate);
            threads.push(std::thread::spawn(move || {
                for _ in 0..ROUNDS {
                    rg.wait();
                    let r = wait_single_handle(hv as HANDLE, Some(Duration::from_secs(5)), false);
                    assert!(matches!(r, WaitResult::Object(0)), "lost wakeup on manual-reset event");
                    dg.wait();
                }
            }));
        }

        for r in 0..ROUNDS {
            with_event(h, |e| e.reset()).unwrap();
            round_gate.wait();
            if r % 3 == 0 {
                // Let waiters park in `__ulock_wait` to exercise batched `ULF_WAKE_ALL`.
                std::thread::sleep(Duration::from_millis(1));
            }
            with_event(h, |e| e.set()).unwrap();
            done_gate.wait();
        }
        for t in threads {
            t.join().unwrap();
        }
        assert!(close(h));
    }

    #[test]
    fn pulse_event_manual_and_auto_reset_semantics() {
        use std::sync::Barrier;

        // 1. Manual-reset PulseEvent releases ALL currently waiting threads and leaves event unsignaled.
        let manual = Arc::new(Event::new(true, false, None));
        let hm = insert(manual.clone());
        const N: usize = 6;
        let ready = Arc::new(Barrier::new(N + 1));
        let mut hs = Vec::new();
        for _ in 0..N {
            let hv = hm as usize;
            let b = Arc::clone(&ready);
            hs.push(std::thread::spawn(move || {
                b.wait();
                let r = wait_single_handle(hv as HANDLE, Some(Duration::from_secs(5)), false);
                assert!(matches!(r, WaitResult::Object(0)), "manual PulseEvent missed waiter");
            }));
        }
        ready.wait();
        std::thread::sleep(Duration::from_millis(10));
        with_event(hm, |e| e.pulse()).unwrap();
        for t in hs {
            t.join().unwrap();
        }
        // New waiter after PulseEvent must observe unsignaled state.
        assert!(matches!(
            wait_single_handle(hm, Some(Duration::ZERO), false),
            WaitResult::Timeout
        ));
        assert!(close(hm));

        // 2. Auto-reset PulseEvent releases EXACTLY ONE waiting thread and leaves event unsignaled.
        let auto = Arc::new(Event::new(false, false, None));
        let ha = insert(auto.clone());
        let woken = Arc::new(AtomicUsize::new(0));
        let ready2 = Arc::new(Barrier::new(N + 1));
        let mut hs2 = Vec::new();
        for _ in 0..N {
            let hv = ha as usize;
            let w = Arc::clone(&woken);
            let b = Arc::clone(&ready2);
            hs2.push(std::thread::spawn(move || {
                b.wait();
                let r = wait_single_handle(hv as HANDLE, Some(Duration::from_millis(200)), false);
                if matches!(r, WaitResult::Object(0)) {
                    w.fetch_add(1, Ordering::AcqRel);
                }
            }));
        }
        ready2.wait();
        std::thread::sleep(Duration::from_millis(15));
        with_event(ha, |e| e.pulse()).unwrap();
        std::thread::sleep(Duration::from_millis(30));
        assert_eq!(woken.load(Ordering::Acquire), 1, "auto-reset PulseEvent must release exactly 1 waiter");
        for t in hs2 {
            t.join().unwrap();
        }
        assert!(matches!(
            wait_single_handle(ha, Some(Duration::ZERO), false),
            WaitResult::Timeout
        ));
        assert!(close(ha));
    }

    #[test]
    fn semaphore_stress_counts_batching_timeouts_and_max_limit() {
        let sem = Arc::new(Semaphore::new(0, 64, None));
        let h = insert(sem.clone());

        // Max limit enforcement.
        assert_eq!(sem.release(64), Some(0));
        assert_eq!(sem.release(1), None);
        for _ in 0..64 {
            assert!(sem.try_take());
        }
        assert!(!sem.try_take());

        // Multi-producer (mix of single-unit bursts and multi-unit batch releases)
        // + multi-consumer (mix of infinite waits and short timeouts) with exact conservation.
        const CONSUMERS: usize = 8;
        const PRODUCERS: usize = 4;
        const PER_PRODUCER_UNITS: usize = 500;
        const TOTAL_UNITS: usize = PRODUCERS * PER_PRODUCER_UNITS;

        let taken = Arc::new(AtomicUsize::new(0));
        let done_producing = Arc::new(AtomicBool::new(false));
        let mut consumers = Vec::new();
        for cid in 0..CONSUMERS {
            let hv = h as usize;
            let taken = Arc::clone(&taken);
            let dp = Arc::clone(&done_producing);
            consumers.push(std::thread::spawn(move || {
                let mut i = 0usize;
                loop {
                    // Mix short timeouts and blocking waits to exercise sleeper unregister + wake pass-on.
                    let timeout = if (cid + i) % 5 == 0 {
                        Some(Duration::from_micros(200))
                    } else {
                        Some(Duration::from_millis(20))
                    };
                    i += 1;
                    match wait_single_handle(hv as HANDLE, timeout, false) {
                        WaitResult::Object(0) => {
                            taken.fetch_add(1, Ordering::AcqRel);
                        }
                        WaitResult::Timeout => {
                            if dp.load(Ordering::Acquire) && taken.load(Ordering::Acquire) >= TOTAL_UNITS {
                                break;
                            }
                        }
                        _ => panic!("unexpected wait result"),
                    }
                    if dp.load(Ordering::Acquire) && taken.load(Ordering::Acquire) >= TOTAL_UNITS {
                        break;
                    }
                }
            }));
        }

        let mut producers = Vec::new();
        for pid in 0..PRODUCERS {
            let hv = h as usize;
            producers.push(std::thread::spawn(move || {
                let mut rem = PER_PRODUCER_UNITS;
                let mut step = pid + 1;
                while rem > 0 {
                    let batch = match step % 4 {
                        0 => 1,
                        1 => 2,
                        2 => 4,
                        _ => 8,
                    }
                    .min(rem);
                    step += 1;
                    // Retry if semaphore temporarily hits max=64.
                    while with_semaphore(hv as HANDLE, |s| s.release(batch as i32))
                        .unwrap()
                        .is_none()
                    {
                        std::thread::yield_now();
                    }
                    rem -= batch;
                    if step % 17 == 0 {
                        std::thread::sleep(Duration::from_micros(50));
                    }
                }
            }));
        }

        for p in producers {
            p.join().unwrap();
        }
        done_producing.store(true, Ordering::Release);
        for c in consumers {
            c.join().unwrap();
        }
        assert_eq!(taken.load(Ordering::Acquire), TOTAL_UNITS);
        assert!(!sem.try_take(), "semaphore count must be exactly 0 after draining");
        assert!(close(h));
    }

    #[test]
    fn wait_for_multiple_all_and_any_mixed_with_single_waiters_stress() {
        let ev_a = Arc::new(Event::new(false, false, None));
        let ev_b = Arc::new(Event::new(true, false, None));
        let sem = Arc::new(Semaphore::new(0, 128, None));
        let h_a = insert(ev_a.clone());
        let h_b = insert(ev_b.clone());
        let h_s = insert(sem.clone());

        // 1. WaitForMultiple(wait_all=true) on auto-reset Event + manual-reset Event + Semaphore.
        let objs_all: Vec<Arc<dyn KObject>> = vec![ev_a.clone(), ev_b.clone(), sem.clone()];
        let all_done = Arc::new(AtomicBool::new(false));
        let ad = Arc::clone(&all_done);
        let t_all = std::thread::spawn(move || {
            let r = wait_for(&objs_all, true, 5_000, false);
            assert!(matches!(r, WaitResult::Object(0)), "wait_all must succeed when all 3 are signaled");
            ad.store(true, Ordering::Release);
        });
        std::thread::sleep(Duration::from_millis(10));
        ev_a.set();
        std::thread::sleep(Duration::from_millis(5));
        assert!(!all_done.load(Ordering::Acquire), "wait_all must not complete with only 1/3 signaled");
        ev_b.set();
        std::thread::sleep(Duration::from_millis(5));
        assert!(!all_done.load(Ordering::Acquire), "wait_all must not complete with only 2/3 signaled");
        sem.release(1).unwrap();
        t_all.join().unwrap();
        assert!(all_done.load(Ordering::Acquire));
        // Auto-reset ev_a and sem must be consumed; manual-reset ev_b stays signaled.
        assert!(!ev_a.try_take());
        assert!(!sem.try_take());
        assert!(ev_b.try_take());
        ev_b.reset();

        // 2. Concurrent single-object fast-path waiters AND multi-object wait_any waiters on the SAME Semaphore!
        const TOTAL: usize = 400;
        let got = Arc::new(AtomicUsize::new(0));
        let mut workers = Vec::new();
        for wid in 0..6usize {
            let g = Arc::clone(&got);
            let s_obj = sem.clone();
            let e_obj = ev_a.clone();
            let hs = h_s as usize;
            workers.push(std::thread::spawn(move || {
                let objs: [Arc<dyn KObject>; 2] = [e_obj, s_obj];
                loop {
                    if g.load(Ordering::Acquire) >= TOTAL {
                        break;
                    }
                    let res = if wid % 2 == 0 {
                        wait_single_handle(hs as HANDLE, Some(Duration::from_millis(10)), false)
                    } else {
                        wait_for_duration(&objs, false, Some(Duration::from_millis(10)), false)
                    };
                    match res {
                        WaitResult::Object(_) => {
                            g.fetch_add(1, Ordering::AcqRel);
                        }
                        WaitResult::Timeout => {}
                        _ => panic!("unexpected wait result"),
                    }
                }
            }));
        }
        for i in 0..TOTAL {
            if i % 4 == 0 {
                sem.release(1).unwrap();
            } else {
                with_semaphore(h_s, |s| s.release(1)).unwrap().unwrap();
            }
            if i % 32 == 0 {
                std::thread::sleep(Duration::from_micros(50));
            }
        }
        let dl = Instant::now() + Duration::from_secs(5);
        while got.load(Ordering::Acquire) < TOTAL {
            assert!(Instant::now() < dl, "mixed single+multi waiters lost wakeup (got {}/{TOTAL})", got.load(Ordering::Acquire));
            std::thread::sleep(Duration::from_millis(1));
        }
        for w in workers {
            w.join().unwrap();
        }
        assert!(close(h_a));
        assert!(close(h_b));
        assert!(close(h_s));
    }

    #[test]
    fn alertable_wait_single_and_multiple_apc_delivery() {
        static APC_HITS: AtomicUsize = AtomicUsize::new(0);
        unsafe extern "win64" fn test_apc(param: usize) {
            APC_HITS.fetch_add(param, Ordering::AcqRel);
        }

        APC_HITS.store(0, Ordering::SeqCst);
        let ev1 = Arc::new(Event::new(true, false, None));
        let ev2 = Arc::new(Event::new(true, false, None));
        let h1 = insert(ev1.clone());
        let h2 = insert(ev2.clone());
        let ready = Arc::new(AtomicBool::new(false));
        let r2 = Arc::clone(&ready);
        let e1 = ev1.clone();
        let e2 = ev2.clone();
        let h1_raw = h1 as usize;

        let t = crate::thread::spawn(
            Box::new(move || {
                r2.store(true, Ordering::Release);
                // 1. Alertable single-handle wait interrupted by APC.
                let res1 = wait_single_handle(h1_raw as HANDLE, Some(Duration::from_secs(5)), true);
                assert!(matches!(res1, WaitResult::IoCompletion), "alertable single wait must return IoCompletion");
                // 2. Alertable multi-object wait interrupted by APC.
                let objs: [Arc<dyn KObject>; 2] = [e1, e2];
                let res2 = wait_for_duration(&objs, false, Some(Duration::from_secs(5)), true);
                assert!(matches!(res2, WaitResult::IoCompletion), "alertable multi wait must return IoCompletion");
                0
            }),
            64 * 1024,
            false,
            false,
        )
        .expect("spawn alertable waiter");

        let dl = Instant::now() + Duration::from_secs(2);
        while !ready.load(Ordering::Acquire) {
            assert!(Instant::now() < dl);
            std::thread::sleep(Duration::from_millis(1));
        }
        std::thread::sleep(Duration::from_millis(10));
        crate::thread::queue_apc(&t, test_apc, 10);
        while APC_HITS.load(Ordering::Acquire) < 10 {
            assert!(Instant::now() < dl, "first APC did not run");
            std::thread::sleep(Duration::from_millis(1));
        }
        std::thread::sleep(Duration::from_millis(10));
        crate::thread::queue_apc(&t, test_apc, 32);

        let th = insert(t);
        assert!(matches!(wait_single_handle(th, Some(Duration::from_secs(5)), false), WaitResult::Object(0)));
        assert_eq!(APC_HITS.load(Ordering::Acquire), 42);
        assert!(close(th));
        assert!(close(h1));
        assert!(close(h2));
    }

    #[test]
    fn randomized_sync_stress_no_lost_wakeups() {
        // Seeded PRNG stress hammering Events, Semaphores, PulseEvent, WaitForMultiple,
        // timeouts, and lock-free handle lookups concurrently across 8 threads.
        let sem = Arc::new(Semaphore::new(0, 1024, None));
        let auto_ev = Arc::new(Event::new(false, false, None));
        let man_ev = Arc::new(Event::new(true, false, None));
        let h_sem = insert(sem.clone());
        let h_auto = insert(auto_ev.clone());
        let h_man = insert(man_ev.clone());

        let produced = Arc::new(AtomicUsize::new(0));
        let consumed = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));

        let mut threads = Vec::new();
        for tid in 0..8usize {
            let hs = h_sem as usize;
            let ha = h_auto as usize;
            let hm = h_man as usize;
            let s_arc = sem.clone();
            let a_arc = auto_ev.clone();
            let m_arc = man_ev.clone();
            let prod = Arc::clone(&produced);
            let cons = Arc::clone(&consumed);
            let st = Arc::clone(&stop);
            threads.push(std::thread::spawn(move || {
                let mut rng = 0x9E37_79B9_7F4A_7C15u64 ^ ((tid as u64 + 1).wrapping_mul(0xBF58_476D_1CE4_E5B9));
                let mut next_u64 = || {
                    rng ^= rng << 13;
                    rng ^= rng >> 7;
                    rng ^= rng << 17;
                    rng
                };
                let multi: [Arc<dyn KObject>; 2] = [a_arc, s_arc];
                for _ in 0..400 {
                    match next_u64() % 10 {
                        0..=2 => {
                            let n = ((next_u64() % 3) + 1) as i32;
                            if with_semaphore(hs as HANDLE, |s| s.release(n)).unwrap().is_some() {
                                prod.fetch_add(n as usize, Ordering::AcqRel);
                            }
                        }
                        3..=5 => {
                            let us = (next_u64() % 400) as u64;
                            if matches!(
                                wait_single_handle(hs as HANDLE, Some(Duration::from_micros(us)), false),
                                WaitResult::Object(0)
                            ) {
                                cons.fetch_add(1, Ordering::AcqRel);
                            }
                        }
                        6 => {
                            with_event(ha as HANDLE, |e| e.set()).unwrap();
                        }
                        7 => {
                            if next_u64() % 2 == 0 {
                                with_event(hm as HANDLE, |e| e.pulse()).unwrap();
                            } else {
                                with_event(hm as HANDLE, |e| e.set()).unwrap();
                                with_event(hm as HANDLE, |e| e.reset()).unwrap();
                            }
                        }
                        8 => {
                            if let WaitResult::Object(1) =
                                wait_for_duration(&multi, false, Some(Duration::from_micros(200)), false)
                            {
                                cons.fetch_add(1, Ordering::AcqRel);
                            }
                        }
                        _ => {
                            let _ = wait_single_handle(
                                ha as HANDLE,
                                Some(Duration::from_micros(100)),
                                false,
                            );
                            let _ = m_arc.try_take();
                        }
                    }
                }
                let _ = st.load(Ordering::Relaxed);
            }));
        }

        for t in threads {
            t.join().unwrap();
        }
        stop.store(true, Ordering::Release);
        // Drain every produced semaphore unit: proves exact conservation & zero corruption.
        let mut drained = consumed.load(Ordering::Acquire);
        let expected = produced.load(Ordering::Acquire);
        while sem.try_take() {
            drained += 1;
        }
        assert_eq!(drained, expected, "semaphore unit conservation violated under randomized stress");
        assert!(close(h_sem));
        assert!(close(h_auto));
        assert!(close(h_man));
    }
}
