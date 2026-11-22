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

use crate::types::*;
use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering};
use crate::park::Parker;
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

/// State is only accessed while holding [`WAIT_LOCK`] (passed as `&WaitGuard`).
pub trait Waitable {
    /// Would a wait by thread `tid` be satisfied right now?
    fn is_signaled(&self, g: &WaitGuard, tid: u32) -> bool;
    /// Consume the signal (auto-reset event, semaphore count, mutex ownership).
    /// Returns true if the wait is "abandoned" (mutex).
    fn acquire(&self, g: &WaitGuard, tid: u32) -> bool;
    /// Consume the signal if the object is signaled: `Some(abandoned)`, or
    /// `None`. Objects whose state can also change without `WAIT_LOCK`
    /// (semaphore units) check and consume atomically.
    fn try_acquire(&self, g: &WaitGuard, tid: u32) -> Option<bool> {
        if self.is_signaled(g, tid) {
            Some(self.acquire(g, tid))
        } else {
            None
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

static WAIT_LOCK: crate::park::WordLock = crate::park::WordLock::new();

#[inline]
pub fn wait_lock() -> WaitGuard<'static> {
    WAIT_LOCK.lock();
    WaitGuard { _l: std::marker::PhantomData, _not_send: std::marker::PhantomData }
}

/// One per OS thread (pooled, never freed). `park` is armed with
/// `WAIT_LOCK` held and slept on after releasing it. Everything else is
/// read lock-free by notifiers: the wait
/// fields are written under `WAIT_LOCK` before the owner sleeps, and a
/// notifier only acts on a state change it made under `WAIT_LOCK` afterwards,
/// so the mutex orders the two.
struct Waiter {
    park: Parker,
    /// The slot belongs to a live thread.
    owned: AtomicBool,
    /// Registered for a wait (between [`Registration::new`] and its drop).
    active: AtomicBool,
    /// Match every object (alertable sleep, or more than `MAX_KEYS` objects).
    any: AtomicBool,
    /// Single object, not wait-all: one budgeted wake satisfies it.
    single: AtomicBool,
    nkeys: AtomicUsize,
    /// Object identities (`Arc` data pointers).
    keys: [AtomicUsize; MAX_KEYS],
    /// A wake has been sent and not yet observed (cleared, under
    /// `WAIT_LOCK`, right before the owner sleeps again). Budgeted wakes skip
    /// waiters with a pending wake so they reach a different thread.
    notified: AtomicBool,
}

const MAX_KEYS: usize = 8;
const MAX_WAITERS: usize = 1024;

impl Waiter {
    #[inline]
    fn matches(&self, key: usize) -> bool {
        if self.any.load(Ordering::Relaxed) {
            return true;
        }
        let n = self.nkeys.load(Ordering::Relaxed).min(MAX_KEYS);
        self.keys[..n].iter().any(|k| k.load(Ordering::Relaxed) == key)
    }
}

/// Append-only pool of per-thread waiters: notifiers scan it without taking
/// any lock, so no syscall ever runs under a lock that waiters also need.
static SLOTS: [AtomicPtr<Waiter>; MAX_WAITERS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; MAX_WAITERS];
static NSLOTS: AtomicUsize = AtomicUsize::new(0);

fn acquire_waiter() -> &'static Waiter {
    let n = NSLOTS.load(Ordering::Acquire).min(MAX_WAITERS);
    for s in &SLOTS[..n] {
        let p = s.load(Ordering::Acquire);
        if !p.is_null() {
            let w = unsafe { &*p };
            if w.owned.compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                return w;
            }
        }
    }
    let w: &'static Waiter = Box::leak(Box::new(Waiter {
        park: Parker::new(),
        owned: AtomicBool::new(true),
        active: AtomicBool::new(false),
        any: AtomicBool::new(false),
        single: AtomicBool::new(false),
        nkeys: AtomicUsize::new(0),
        keys: [const { AtomicUsize::new(0) }; MAX_KEYS],
        notified: AtomicBool::new(false),
    }));
    let i = NSLOTS.fetch_add(1, Ordering::AcqRel);
    assert!(i < MAX_WAITERS, "handles: more than {MAX_WAITERS} concurrent waiting threads");
    SLOTS[i].store(w as *const Waiter as *mut Waiter, Ordering::Release);
    w
}

struct WaiterHolder(&'static Waiter);
impl Drop for WaiterHolder {
    fn drop(&mut self) {
        self.0.active.store(false, Ordering::Release);
        self.0.owned.store(false, Ordering::Release);
    }
}

thread_local! {
    static WAITER: WaiterHolder = WaiterHolder(acquire_waiter());
}

#[inline]
fn for_each_waiter(mut f: impl FnMut(&'static Waiter) -> bool) {
    let n = NSLOTS.load(Ordering::Acquire).min(MAX_WAITERS);
    for s in &SLOTS[..n] {
        let p = s.load(Ordering::Acquire);
        if p.is_null() {
            continue;
        }
        let w = unsafe { &*p };
        if w.active.load(Ordering::Acquire) && !f(w) {
            break;
        }
    }
}

#[inline]
fn wake(w: &Waiter) {
    w.notified.store(true, Ordering::Release);
    w.park.unpark();
}

/// Call after changing any waitable state (while or after holding the lock).
/// Wakes every blocked thread; objects in this file use [`notify_object`].
pub fn notify_waiters() {
    for_each_waiter(|w| {
        wake(w);
        true
    });
}

/// Wake waiters of the object at `key` (see [`obj_key`]). Single-object
/// waiters are woken at most `budget` at a time (those with a wake already
/// pending don't count); multi-object / wait-all waiters are always woken
/// since they re-check the combined condition.
fn notify_object(key: usize, mut budget: usize) {
    for_each_waiter(|w| {
        if !w.matches(key) {
            return true;
        }
        if w.single.load(Ordering::Relaxed) {
            if budget == 0 || w.notified.swap(true, Ordering::AcqRel) {
                return true;
            }
            budget -= 1;
            w.park.unpark();
        } else {
            wake(w);
        }
        true
    });
}

#[inline]
fn obj_key<T: ?Sized>(p: *const T) -> usize {
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
    let mut pending = false;
    let mut cand: Option<&'static Waiter> = None;
    for_each_waiter(|w| {
        if !w.matches(key) {
            return true;
        }
        if w.single.load(Ordering::Relaxed) {
            if w.notified.load(Ordering::Acquire) {
                pending = true;
            } else if cand.is_none() {
                cand = Some(w);
            }
        } else {
            wake(w);
        }
        true
    });
    if !pending {
        if let Some(w) = cand {
            if !w.notified.swap(true, Ordering::AcqRel) {
                w.park.unpark();
            }
        }
    }
}

/// Registration of the current thread for the duration of one wait. Created
/// and dropped with `WAIT_LOCK` held.
struct Registration {
    waiter: &'static Waiter,
    /// Pool slot borrowed without TLS (wait during thread teardown).
    temp: bool,
    /// The object, when this is a single-object wait (`single`).
    key: usize,
    single: bool,
    /// The wait consumed a signal; otherwise a pending targeted wake must be
    /// passed on when we leave.
    consumed: bool,
}

impl Registration {
    fn new(objs: &[Arc<dyn KObject>], wait_all: bool) -> Self {
        let (waiter, temp) = match WAITER.try_with(|h| h.0) {
            Ok(w) => (w, false),
            Err(_) => (acquire_waiter(), true),
        };
        let any = objs.is_empty() || objs.len() > MAX_KEYS;
        let nkeys = if any { 0 } else { objs.len() };
        for (k, o) in waiter.keys.iter().zip(objs.iter()).take(nkeys) {
            k.store(obj_key(Arc::as_ptr(o)), Ordering::Relaxed);
        }
        let single = nkeys == 1 && !wait_all;
        waiter.nkeys.store(nkeys, Ordering::Relaxed);
        waiter.any.store(any, Ordering::Relaxed);
        waiter.single.store(single, Ordering::Relaxed);
        waiter.notified.store(false, Ordering::Relaxed);
        waiter.active.store(true, Ordering::Release);
        let key = if nkeys > 0 { obj_key(Arc::as_ptr(&objs[0])) } else { 0 };
        Registration { waiter, temp, key, single, consumed: false }
    }

    /// Before sleeping (with `WAIT_LOCK` held, after the state re-check).
    fn arm(&self) {
        self.waiter.notified.store(false, Ordering::Release);
        self.waiter.park.arm();
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        self.waiter.active.store(false, Ordering::Release);
        // A budgeted wake aimed at us (e.g. one semaphore unit) that we are
        // not going to use (timeout, APC) must reach another waiter. Only
        // single-object waiters receive budgeted wakes.
        if self.single && !self.consumed && self.waiter.notified.load(Ordering::Acquire) {
            notify_object(self.key, usize::MAX);
        }
        if self.temp {
            self.waiter.owned.store(false, Ordering::Release);
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

struct Table {
    next: usize,
    map: HashMap<usize, Arc<dyn KObject>>,
}

/// The handle table behind an [`RwLock`]: concurrent `handles::get` misses take
/// a shared read lock instead of serializing behind an exclusive lock, while
/// `insert` / `close` take the write lock.
#[inline]
fn table() -> &'static RwLock<Table> {
    static TABLE: OnceLock<RwLock<Table>> = OnceLock::new();
    TABLE.get_or_init(|| RwLock::new(Table { next: 0x100, map: HashMap::new() }))
}

pub fn insert(obj: Arc<dyn KObject>) -> HANDLE {
    let mut t = table().write().unwrap();
    let h = t.next;
    t.next += 4;
    t.map.insert(h, obj);
    h as HANDLE
}

pub const CURRENT_PROCESS: HANDLE = usize::MAX as HANDLE; // -1
pub const CURRENT_THREAD: HANDLE = (usize::MAX - 1) as HANDLE; // -2

pub fn get(h: HANDLE) -> Option<Arc<dyn KObject>> {
    if h == CURRENT_THREAD {
        return crate::thread::current_thread_object().map(|t| t as Arc<dyn KObject>);
    }
    let hv = h as usize;
    let gen = CLOSE_GEN.load(Ordering::Acquire);
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
    let any = o.as_any();
    if any.is::<Semaphore>() || any.is::<Event>() || any.is::<MutexObj>() || any.is::<WaitableTimer>() {
        let _ = HCACHE.try_with(|c| {
            let old = unsafe {
                std::mem::replace(&mut (*c.get())[slot], HEntry { h: hv, gen, obj: Some(o.clone()) })
            };
            drop(old);
        });
    }
    Some(o)
}

/// Bumped by every [`close`] (after the table removal, still under the write
/// lock). Handle values are never reused, so a per-thread cache entry stays
/// valid until then.
static CLOSE_GEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
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
        let r = t.map.remove(&(h as usize));
        if r.is_some() {
            CLOSE_GEN.fetch_add(1, Ordering::Release);
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

/// Generic WaitForMultipleObjects. `timeout_ms == INFINITE` waits forever.
pub fn wait_for(objs: &[Arc<dyn KObject>], wait_all: bool, timeout_ms: u32, alertable: bool) -> WaitResult {
    for o in objs {
        if o.waitable().is_none() {
            log::warn!("wait on non-waitable object type {}", o.type_name());
            return WaitResult::Failed(err::ERROR_INVALID_HANDLE);
        }
    }
    let tid = crate::teb::thread_id();
    let deadline = (timeout_ms != INFINITE).then(|| Instant::now() + Duration::from_millis(timeout_ms as u64));
    let mut g = wait_lock();
    let mut wait_scope: Option<crate::hitch::Scope> = None;
    // Declared after `g`: dropped (unregistered) while the lock is held.
    let mut reg_: Option<Registration> = None;
    loop {
        if alertable && crate::thread::has_pending_apc() {
            drop(reg_);
            drop(g);
            drop(wait_scope);
            crate::thread::run_pending_apcs();
            return WaitResult::IoCompletion;
        }
        if wait_all {
            if objs.iter().all(|o| o.waitable().unwrap().is_signaled(&g, tid)) {
                // Semaphore units can vanish without the lock: take those
                // first, all or nothing; the rest cannot change under it.
                let mut taken = 0usize;
                let mut ok = true;
                for o in objs {
                    if let Some(s) = o.as_any().downcast_ref::<Semaphore>() {
                        if !s.try_take() {
                            ok = false;
                            break;
                        }
                        taken += 1;
                    }
                }
                if ok {
                    let mut abandoned = false;
                    for o in objs {
                        if !o.as_any().is::<Semaphore>() {
                            abandoned |= o.waitable().unwrap().acquire(&g, tid);
                        }
                    }
                    if let Some(r) = reg_.as_mut() {
                        r.consumed = true;
                    }
                    return if abandoned { WaitResult::Abandoned(0) } else { WaitResult::Object(0) };
                }
                for o in objs {
                    if taken == 0 {
                        break;
                    }
                    if let Some(s) = o.as_any().downcast_ref::<Semaphore>() {
                        s.untake(&g);
                        taken -= 1;
                    }
                }
            }
        } else {
            for (i, o) in objs.iter().enumerate() {
                let w = o.waitable().unwrap();
                if let Some(ab) = w.try_acquire(&g, tid) {
                    // Cascade: units left over -> hand one wake on from here
                    // (the releaser only woke us).
                    let pass_on = sem_cascade()
                        && objs.len() == 1
                        && o.as_any().is::<Semaphore>()
                        && w.is_signaled(&g, tid);
                    if let Some(r) = reg_.as_mut() {
                        r.consumed = true;
                    }
                    if pass_on {
                        drop(reg_);
                        drop(g);
                        notify_semaphore(obj_key(Arc::as_ptr(o)));
                    }
                    return if ab { WaitResult::Abandoned(i) } else { WaitResult::Object(i) };
                }
            }
        }
        if timeout_ms == 0 {
            return WaitResult::Timeout;
        }
        if wait_scope.is_none() {
            wait_scope = Some(crate::hitch::Scope::enter(crate::hitch::ActKind::HandleWait, objs.len() as u64));
        }
        let r = reg_.get_or_insert_with(|| Registration::new(objs, wait_all));
        if deadline.is_some_and(|d| Instant::now() >= d) {
            return WaitResult::Timeout;
        }
        r.arm();
        let park = &r.waiter.park;
        drop(g);
        park.park(deadline);
        g = wait_lock();
    }
}

/// Sleep that can be interrupted by APCs (SleepEx alertable).
pub fn alertable_sleep(timeout_ms: u32) -> bool {
    let deadline = (timeout_ms != INFINITE).then(|| Instant::now() + Duration::from_millis(timeout_ms as u64));
    let mut g = wait_lock();
    let mut reg_: Option<Registration> = None;
    loop {
        if crate::thread::has_pending_apc() {
            drop(reg_);
            drop(g);
            crate::thread::run_pending_apcs();
            return true;
        }
        let r = reg_.get_or_insert_with(|| Registration::new(&[], false));
        if deadline.is_some_and(|d| Instant::now() >= d) {
            return false;
        }
        r.arm();
        let park = &r.waiter.park;
        drop(g);
        park.park(deadline);
        g = wait_lock();
    }
}

// ---------------------------------------------------------------------------
// Basic waitable objects
// ---------------------------------------------------------------------------

pub struct Event {
    pub manual_reset: bool,
    pub signaled: Guarded<bool>,
    pub name: Option<String>,
}

impl Event {
    pub fn new(manual_reset: bool, initial: bool, name: Option<String>) -> Self {
        Event { manual_reset, signaled: Guarded::new(initial), name }
    }
    pub fn set(&self) {
        let g = wait_lock();
        *self.signaled.get(&g) = true;
        drop(g);
        // An auto-reset event satisfies exactly one wait.
        notify_object(obj_key(self), if self.manual_reset { usize::MAX } else { 1 });
    }
    pub fn reset(&self) {
        let g = wait_lock();
        *self.signaled.get(&g) = false;
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
            notify_object(obj_key(self), 1);
        }
        true
    }
}

/// Count is atomic: [`Semaphore::try_take`] consumes a unit without `WAIT_LOCK`
/// (the job workers' `WaitForSingleObject` on a semaphore that already has
/// units). Increments stay under `WAIT_LOCK`: a waiter re-checks the count and
/// registers under the lock, so a release cannot slip between the two. A
/// lock-free take only ever lowers the count, which cannot strand a waiter.
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
    }

    /// Returns previous count or None if it would exceed max.
    pub fn release(&self, n: i32) -> Option<i32> {
        let g = wait_lock();
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
        drop(g);
        if sem_cascade() {
            notify_semaphore(obj_key(self));
        } else {
            notify_object(obj_key(self), n as usize);
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

    pub fn fire(&self, gen: u64, apc: Option<( crate::thread::ApcProc, usize, Arc<crate::thread::ThreadObj> )>) -> bool {
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
        notify_object(obj_key(self), if self.manual_reset { usize::MAX } else { 1 });
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
        let gen_before = CLOSE_GEN.load(Ordering::Acquire);
        let cached = HCACHE.with(|c| {
            let e = unsafe { &(*c.get())[slot] };
            e.h == hv && e.gen == gen_before && e.obj.as_ref().is_some_and(|o| Arc::ptr_eq(o, &timer))
        });
        assert!(cached, "WaitableTimer should be cached in HCACHE");

        // Second get hits HCACHE and returns the same object.
        let got2 = get(h).expect("cached handle must resolve");
        assert!(Arc::ptr_eq(&got2, &timer));

        // close() removes from TABLE and bumps CLOSE_GEN to invalidate HCACHE.
        assert!(close(h));
        assert!(!close(h));
        assert!(CLOSE_GEN.load(Ordering::Acquire) > gen_before);
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
}
