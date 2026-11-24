//! Kernel object handle table and the generic wait machinery.
//!
//! Handles are small multiples of 4 (like Windows). Waitable objects keep
//! their state under one global `WAIT_LOCK`, which keeps
//! WaitForMultipleObjects with bWaitAll atomic.
//!
//! Wakeups are targeted: each blocked thread registers itself (with the
//! objects it waits on) in a small registry and sleeps on its own parking
//! word ([`Parker`]; a notifier only enters the kernel if the thread is
//! really asleep).
//! The basic objects in this file wake only the waiters of the object whose
//! state changed, and for objects that satisfy a bounded number of waits
//! (semaphore count, auto-reset event, mutex) only that many single-object
//! waiters. A global `ReleaseSemaphore` used to wake every blocked thread in
//! the process (the job system has ~20 workers), which showed up as ~13% of
//! the scheduler thread in `__psynch_cvbroad`. [`notify_waiters`] still wakes
//! everyone for state owned by other modules (threads, APCs, message queues).

use crate::park::{Parker, WordLock};
use crate::types::*;
use std::any::Any;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};

pub trait KObject: Any + Send + Sync {
    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    /// Waitable objects return themselves.
    fn waitable(&self) -> Option<&dyn Waitable> {
        None
    }
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
    /// and sleeping). Budgeted wakes skip waiters with a pending wake so they reach
    /// a different thread.
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

#[inline]
fn collect_matching(bucket: &WaitBucket, match_key: Option<usize>, budget: &mut usize, batch: &mut WakeBatch) {
    if bucket.count.load(Ordering::Acquire) == 0 {
        return;
    }
    bucket.lock.lock();
    let entries = unsafe { &*bucket.entries.get() };
    for (k, w) in entries.iter() {
        if let Some(target) = match_key {
            if *k != target {
                continue;
            }
        }
        if !w.active.load(Ordering::Acquire) {
            continue;
        }
        if match_key.is_some() && w.single.load(Ordering::Relaxed) {
            if *budget == 0 || w.notified.swap(true, Ordering::AcqRel) {
                continue;
            }
            *budget -= 1;
            batch.push(Arc::clone(w));
        } else {
            w.notified.store(true, Ordering::Release);
            batch.push(Arc::clone(w));
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
/// always woken so they re-check their combined condition.
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
        if *k == target && w.active.load(Ordering::Acquire) {
            w.notified.store(true, Ordering::Release);
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

/// Semaphore wake cascade (default; `AW2_SEM_CASCADE=0` for A/B): releases
/// wake at most one waiter and only if no earlier wake is still in flight;
/// each woken waiter that leaves units behind wakes the next (see
/// [`wait_for`]). Moves the kernel wakes of a burst of
/// `ReleaseSemaphore(…, 1)` calls off the releasing thread (the job
/// scheduler's main thread): gameplay ~103 -> ~105 fps, ~40% fewer kernel
/// wakes.
fn sem_cascade() -> bool {
    static C: OnceLock<bool> = OnceLock::new();
    *C.get_or_init(|| !std::env::var("AW2_SEM_CASCADE").is_ok_and(|v| v == "0"))
}

/// Cascade-mode wake for the semaphore at `key`: multi-object waiters are
/// always woken; among single-object waiters, one is woken unless one already
/// has a wake pending (it will pass the wake on once it takes its unit).
fn notify_semaphore(key: usize) {
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
        let mut cand: Option<Arc<Waiter>> = None;
        b.lock.lock();
        let entries = unsafe { &*b.entries.get() };
        for (k, w) in entries.iter() {
            if *k != key || !w.active.load(Ordering::Acquire) {
                continue;
            }
            if w.single.load(Ordering::Relaxed) {
                if w.notified.load(Ordering::Acquire) {
                    pending = true;
                } else if cand.is_none() {
                    cand = Some(Arc::clone(w));
                }
            } else {
                w.notified.store(true, Ordering::Release);
                batch.push(Arc::clone(w));
            }
        }
        if !pending {
            if let Some(w) = cand {
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

/// Registration of the current thread for the duration of one wait.
struct Registration {
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

impl Registration {
    fn new(objs: &[Arc<dyn KObject>], wait_all: bool, alertable: bool, tid: u32) -> Self {
        let waiter = WAITER.try_with(Arc::clone).unwrap_or_else(|_| Arc::new(Waiter::new()));
        let single = objs.len() == 1 && !wait_all;
        waiter.single.store(single, Ordering::Relaxed);
        waiter.notified.store(false, Ordering::Relaxed);
        waiter.active.store(true, Ordering::Release);

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
            waiter,
            keys,
            nkeys,
            in_any,
            alertable_tid,
            single,
            consumed: false,
        }
    }

    /// Arm before checking condition / sleeping. Includes a SeqCst fence so
    /// lock-free notifiers (`Event::set`, `Semaphore::release`, `queue_apc`)
    /// are strictly ordered against our subsequent state check.
    #[inline]
    fn arm(&self) {
        self.waiter.notified.store(false, Ordering::Release);
        self.waiter.park.arm();
        std::sync::atomic::fence(Ordering::SeqCst);
    }
}

impl Drop for Registration {
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
        // A budgeted wake aimed at us (e.g. one semaphore unit or auto-reset event)
        // that we are not going to use (timeout, APC) must reach one other waiter.
        if self.single && !self.consumed && self.waiter.notified.load(Ordering::Acquire) {
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

pub const HANDLE_FLAG_INHERIT: u32 = 0x1;
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: u32 = 0x2;

struct Table {
    next: usize,
    map: HashMap<usize, Arc<dyn KObject>>,
    flags: HashMap<usize, u32>,
}

/// The handle table behind an [`RwLock`]: concurrent `handles::get` misses take
/// a shared read lock instead of serializing behind an exclusive lock, while
/// `insert` / `close` take the write lock.
#[inline]
fn table() -> &'static RwLock<Table> {
    static TABLE: OnceLock<RwLock<Table>> = OnceLock::new();
    TABLE.get_or_init(|| RwLock::new(Table { next: 0x100, map: HashMap::new(), flags: HashMap::new() }))
}

pub fn insert(obj: Arc<dyn KObject>) -> HANDLE {
    insert_with_flags(obj, 0)
}

pub fn insert_with_flags(obj: Arc<dyn KObject>, flags: u32) -> HANDLE {
    let mut t = table().write().unwrap();
    let h = t.next;
    t.next += 4;
    t.map.insert(h, obj);
    if flags != 0 {
        t.flags.insert(h, flags & (HANDLE_FLAG_INHERIT | HANDLE_FLAG_PROTECT_FROM_CLOSE));
    }
    h as HANDLE
}

pub fn get_flags(h: HANDLE) -> Option<u32> {
    if h == CURRENT_PROCESS || h == CURRENT_THREAD {
        return Some(0);
    }
    let t = table().read().unwrap();
    if !t.map.contains_key(&(h as usize)) {
        return None;
    }
    Some(t.flags.get(&(h as usize)).copied().unwrap_or(0))
}

pub fn set_flags(h: HANDLE, mask: u32, flags: u32) -> bool {
    if h == CURRENT_PROCESS || h == CURRENT_THREAD {
        return true;
    }
    let mut t = table().write().unwrap();
    if !t.map.contains_key(&(h as usize)) {
        return false;
    }
    let cur = t.flags.get(&(h as usize)).copied().unwrap_or(0);
    let next = (cur & !mask) | (flags & mask);
    let next = next & (HANDLE_FLAG_INHERIT | HANDLE_FLAG_PROTECT_FROM_CLOSE);
    if next == 0 {
        t.flags.remove(&(h as usize));
    } else {
        t.flags.insert(h as usize, next);
    }
    true
}

pub fn is_protected_from_close(h: HANDLE) -> bool {
    get_flags(h).is_some_and(|f| (f & HANDLE_FLAG_PROTECT_FROM_CLOSE) != 0)
}

pub const CURRENT_PROCESS: HANDLE = usize::MAX as HANDLE; // -1
pub const CURRENT_THREAD: HANDLE = (usize::MAX - 1) as HANDLE; // -2

#[inline]
fn is_hcache_type(o: &dyn KObject) -> bool {
    let any = o.as_any();
    any.is::<Semaphore>() || any.is::<Event>() || any.is::<MutexObj>() || any.is::<WaitableTimer>()
}

pub fn get(h: HANDLE) -> Option<Arc<dyn KObject>> {
    if h == CURRENT_THREAD {
        return crate::thread::current_thread_object().map(|t| t as Arc<dyn KObject>);
    }
    let hv = h as usize;
    let gen = SYNC_CLOSE_GEN.load(Ordering::Acquire);
    let slot = (hv >> 2) & (HCACHE_LEN - 1);
    if let Ok(Some(o)) = HCACHE.try_with(|c| {
        let e = unsafe { &(*c.get())[slot] };
        if e.h == hv && e.gen == gen {
            e.obj.clone()
        } else {
            None
        }
    }) {
        return Some(o);
    }
    let o = table().read().unwrap().map.get(&hv).cloned()?;
    // Only synchronization objects (trivial Drop: a cached reference may
    // outlive CloseHandle until the entry is next touched).
    if is_hcache_type(&*o) {
        let _ = HCACHE.try_with(|c| {
            let old = unsafe {
                std::mem::replace(&mut (*c.get())[slot], HEntry { h: hv, gen, obj: Some(o.clone()) })
            };
            drop(old);
        });
    }
    Some(o)
}

/// Bumped ONLY when a cached synchronization handle (`Semaphore`, `Event`,
/// `MutexObj`, `WaitableTimer`) is closed, so closing files/pipes/mappings
/// never flushes every thread's 128-entry `HCACHE`.
static SYNC_CLOSE_GEN: AtomicU64 = AtomicU64::new(0);
const HCACHE_LEN: usize = 128;

struct HEntry {
    h: usize,
    gen: u64,
    obj: Option<Arc<dyn KObject>>,
}

thread_local! {
    static HCACHE: std::cell::UnsafeCell<[HEntry; HCACHE_LEN]> =
        std::cell::UnsafeCell::new(std::array::from_fn(|_| HEntry { h: 0, gen: 0, obj: None }));
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
    // Drop the object (arbitrary Drop code) after releasing the table write lock.
    let removed = {
        let mut t = table().write().unwrap();
        if t.flags.get(&(h as usize)).is_some_and(|&f| (f & HANDLE_FLAG_PROTECT_FROM_CLOSE) != 0) {
            return false;
        }
        let r = t.map.remove(&(h as usize));
        if let Some(ref obj) = r {
            t.flags.remove(&(h as usize));
            if is_hcache_type(&**obj) {
                SYNC_CLOSE_GEN.fetch_add(1, Ordering::Release);
            }
        }
        r
    };
    removed.is_some()
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
/// never touches the global `WAIT_LOCK`, only the object's sharded waiter bucket.
fn wait_single_lockfree(obj: &Arc<dyn KObject>, timeout: Option<Duration>) -> Option<WaitResult> {
    enum FastObj<'a> {
        Sem(&'a Semaphore),
        Ev(&'a Event),
    }
    let fast = if let Some(s) = obj.as_any().downcast_ref::<Semaphore>() {
        FastObj::Sem(s)
    } else if let Some(e) = obj.as_any().downcast_ref::<Event>() {
        FastObj::Ev(e)
    } else {
        return None;
    };
    let key = obj.waitable().unwrap().wait_key();
    let try_one = || match fast {
        FastObj::Sem(s) => s.try_take(),
        FastObj::Ev(e) => e.try_take(),
    };
    let pass_on_sem = || match fast {
        FastObj::Sem(s) => sem_cascade() && s.count.load(Ordering::Acquire) > 0,
        FastObj::Ev(_) => false,
    };

    if try_one() {
        return Some(WaitResult::Object(0));
    }
    if matches!(timeout, Some(d) if d.is_zero()) {
        return Some(WaitResult::Timeout);
    }

    let deadline = timeout.map(|d| Instant::now() + d);
    let _wait_scope = crate::hitch::Scope::enter(crate::hitch::ActKind::HandleWait, 1);
    let mut reg = Registration::new(std::slice::from_ref(obj), false, false, 0);
    loop {
        reg.arm();
        if try_one() {
            reg.consumed = true;
            let pass_on = pass_on_sem();
            drop(reg);
            if pass_on {
                notify_semaphore(key);
            }
            return Some(WaitResult::Object(0));
        }
        if deadline.is_some_and(|d| Instant::now() >= d) {
            return Some(WaitResult::Timeout);
        }
        reg.waiter.park.park(deadline);
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
            notify_semaphore(k);
        }
        return res;
    }
    if matches!(timeout, Some(d) if d.is_zero()) {
        return WaitResult::Timeout;
    }

    let deadline = timeout.map(|d| Instant::now() + d);
    let _wait_scope = crate::hitch::Scope::enter(crate::hitch::ActKind::HandleWait, objs.len() as u64);
    let mut reg = Registration::new(objs, wait_all, alertable, tid);
    loop {
        // Arm BEFORE re-checking state so lock-free notifiers (Event::set,
        // Semaphore::release, queue_apc) racing with us cannot lose a wakeup.
        reg.arm();
        if alertable && crate::thread::has_pending_apc() {
            drop(reg);
            drop(g);
            crate::thread::run_pending_apcs();
            return WaitResult::IoCompletion;
        }
        if let Some((res, pass_on)) = try_satisfy_locked(objs, wait_all, &g, tid) {
            reg.consumed = true;
            drop(reg);
            drop(g);
            if let Some(k) = pass_on {
                notify_semaphore(k);
            }
            return res;
        }
        if deadline.is_some_and(|d| Instant::now() >= d) {
            return WaitResult::Timeout;
        }
        let park = Arc::clone(&reg.waiter);
        drop(g);
        park.park.park(deadline);
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
    let reg = Registration::new(&[], false, true, tid);
    loop {
        reg.arm();
        if crate::thread::has_pending_apc() {
            drop(reg);
            crate::thread::run_pending_apcs();
            return true;
        }
        if deadline.is_some_and(|d| Instant::now() >= d) {
            return false;
        }
        reg.waiter.park.park(deadline);
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

pub struct Event {
    pub manual_reset: bool,
    signaled: AtomicBool,
    pub name: Option<String>,
}

impl Event {
    pub fn new(manual_reset: bool, initial: bool, name: Option<String>) -> Self {
        Event { manual_reset, signaled: AtomicBool::new(initial), name }
    }

    #[inline]
    pub fn try_take(&self) -> bool {
        if self.manual_reset {
            self.signaled.load(Ordering::Acquire)
        } else {
            self.signaled.compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed).is_ok()
        }
    }

    #[inline]
    fn untake(&self) {
        if !self.manual_reset {
            self.signaled.store(true, Ordering::Release);
            notify_object_key(obj_key(self), 1);
        }
    }

    pub fn set(&self) {
        self.signaled.store(true, Ordering::Release);
        // An auto-reset event satisfies exactly one wait.
        notify_object_key(obj_key(self), if self.manual_reset { usize::MAX } else { 1 });
    }
    /// Set the event and return whether it was previously signaled (`NtSetEvent`).
    pub fn set_returning_prev(&self) -> bool {
        let prev = self.signaled.swap(true, Ordering::AcqRel);
        notify_object_key(obj_key(self), if self.manual_reset { usize::MAX } else { 1 });
        prev
    }
    pub fn reset(&self) {
        self.signaled.store(false, Ordering::Release);
    }
    /// Reset the event and return whether it was previously signaled (`NtResetEvent`).
    pub fn reset_returning_prev(&self) -> bool {
        self.signaled.swap(false, Ordering::AcqRel)
    }
    /// PulseEvent: release current waiters then reset. Approximated by
    /// set + (yield) + reset.
    pub fn pulse(&self) {
        self.set();
        std::thread::yield_now();
        self.reset();
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
        self.signaled.load(Ordering::Acquire)
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

/// Lock-free semaphore count: both [`Semaphore::try_take`] and
/// [`Semaphore::release`] update `count` atomically without taking `WAIT_LOCK`,
/// ordering against sharded waiter buckets via a `SeqCst` fence.
pub struct Semaphore {
    count: std::sync::atomic::AtomicI32,
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
        self.count.load(Ordering::Acquire) > 0
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
        Semaphore { count: std::sync::atomic::AtomicI32::new(initial), max, name }
    }

    /// Take one unit if available (no lock).
    #[inline]
    pub fn try_take(&self) -> bool {
        let mut c = self.count.load(Ordering::Relaxed);
        while c > 0 {
            match self.count.compare_exchange_weak(c, c - 1, Ordering::Acquire, Ordering::Relaxed) {
                Ok(_) => return true,
                Err(v) => c = v,
            }
        }
        false
    }

    /// Return a unit taken by a wait-all that could not complete (lock held).
    fn untake(&self, _g: &WaitGuard) {
        self.count.fetch_add(1, Ordering::Release);
        notify_object_key(obj_key(self), 1);
    }

    /// Returns previous count or None if it would exceed max.
    pub fn release(&self, n: i32) -> Option<i32> {
        let mut prev = self.count.load(Ordering::Relaxed);
        loop {
            if n <= 0 || prev.checked_add(n).map_or(true, |v| v > self.max) {
                return None;
            }
            match self.count.compare_exchange_weak(prev, prev + n, Ordering::Release, Ordering::Relaxed) {
                Ok(_) => break,
                Err(v) => prev = v,
            }
        }
        if sem_cascade() {
            notify_semaphore(obj_key(self));
        } else {
            notify_object_key(obj_key(self), n as usize);
        }
        Some(prev)
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
        let mut g = self.map.lock().unwrap_or_else(|e| e.into_inner());
        g.get_or_insert_with(HashMap::new).insert(id, val);
        id
    }

    /// Insert `val` at a caller-chosen key, returning any previously stored value.
    pub(crate) fn insert_at(&self, id: usize, val: T) -> Option<T> {
        let mut g = self.map.lock().unwrap_or_else(|e| e.into_inner());
        g.get_or_insert_with(HashMap::new).insert(id, val)
    }

    /// Clone the value stored at `id`, if present.
    pub(crate) fn get_cloned(&self, id: usize) -> Option<T>
    where
        T: Clone,
    {
        let g = self.map.lock().unwrap_or_else(|e| e.into_inner());
        g.as_ref()?.get(&id).cloned()
    }

    /// Borrow the value stored at `id` while holding the table lock.
    pub(crate) fn with<R>(&self, id: usize, f: impl FnOnce(&T) -> R) -> Option<R> {
        let g = self.map.lock().unwrap_or_else(|e| e.into_inner());
        let val = g.as_ref()?.get(&id)?;
        Some(f(val))
    }

    /// Mutably borrow the value stored at `id` while holding the table lock.
    pub(crate) fn with_mut<R>(&self, id: usize, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        let mut g = self.map.lock().unwrap_or_else(|e| e.into_inner());
        let val = g.as_mut()?.get_mut(&id)?;
        Some(f(val))
    }

    /// Remove and return the value stored at `id` (dropped outside the table lock).
    pub(crate) fn remove(&self, id: usize) -> Option<T> {
        let mut g = self.map.lock().unwrap_or_else(|e| e.into_inner());
        g.as_mut()?.remove(&id)
    }

    /// Remove all entries from the table, dropping them outside the table lock.
    pub(crate) fn clear(&self) {
        let old = {
            let mut g = self.map.lock().unwrap_or_else(|e| e.into_inner());
            g.take()
        };
        drop(old);
    }

    /// Retain only entries satisfying `predicate`.
    pub(crate) fn retain(&self, mut predicate: impl FnMut(&usize, &mut T) -> bool) {
        let mut g = self.map.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(m) = g.as_mut() {
            m.retain(|k, v| predicate(k, v));
        }
    }

    /// True if `id` is currently present in the table.
    pub(crate) fn contains(&self, id: usize) -> bool {
        let g = self.map.lock().unwrap_or_else(|e| e.into_inner());
        g.as_ref().is_some_and(|m| m.contains_key(&id))
    }

    /// Number of live entries currently in the table.
    pub(crate) fn len(&self) -> usize {
        let g = self.map.lock().unwrap_or_else(|e| e.into_inner());
        g.as_ref().map_or(0, HashMap::len)
    }

    /// True when the table holds no live entries.
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Snapshot all currently live handle IDs.
    pub(crate) fn keys(&self) -> Vec<usize> {
        let g = self.map.lock().unwrap_or_else(|e| e.into_inner());
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
        let gen_before = SYNC_CLOSE_GEN.load(Ordering::Acquire);
        let cached = HCACHE.with(|c| {
            let e = unsafe { &(*c.get())[slot] };
            e.h == hv && e.gen == gen_before && e.obj.as_ref().is_some_and(|o| Arc::ptr_eq(o, &timer))
        });
        assert!(cached, "WaitableTimer should be cached in HCACHE");

        // Closing a non-sync handle must NOT bump SYNC_CLOSE_GEN or flush HCACHE.
        let non_sync = insert(Arc::new(Opaque("file_like")));
        assert!(close(non_sync));
        assert_eq!(
            SYNC_CLOSE_GEN.load(Ordering::Acquire),
            gen_before,
            "closing non-sync handle must not bump SYNC_CLOSE_GEN"
        );

        // Second get hits HCACHE and returns the same object.
        let got2 = get(h).expect("cached handle must resolve");
        assert!(Arc::ptr_eq(&got2, &timer));

        // close() removes from TABLE and bumps SYNC_CLOSE_GEN to invalidate HCACHE.
        assert!(close(h));
        assert!(!close(h));
        assert!(SYNC_CLOSE_GEN.load(Ordering::Acquire) > gen_before);
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
        // Insert 96 shared objects (> old 32-entry HCACHE) across all 4 cached waitable types.
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
}
