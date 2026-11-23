//! Vista thread pool (`CreateThreadpoolWork` & co.) and `QueueUserWorkItem`.
//!
//! A dynamic pool of Windows threads (with TEBs, static TLS and
//! DLL_THREAD_ATTACH, like real pool threads) executes callbacks and scales up
//! with overflow workers (up to 32) when all workers block on nested waits.
//! Timers use one shared timer thread that enqueues expirations outside any
//! lock; waits use a single multiplexed wait-dispatcher thread backed by an
//! auto-reset wake event so arming/cancelling/re-arming waits never leaks OS
//! threads, stacks, or waiter slots.

use crate::handles;
use crate::thread;
use crate::types::*;
use std::any::Any;
use std::cell::Cell;
use std::collections::{BinaryHeap, VecDeque};
use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

type WorkCb = unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void);
type WaitCb = unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, u32);
type SimpleCb = unsafe extern "win64" fn(*mut c_void, *mut c_void);
type StartRoutine = unsafe extern "win64" fn(*mut c_void) -> u32;

type Job = Box<dyn FnOnce() + Send>;

const MAX_POOL_WORKERS: usize = 32;
const OVERFLOW_IDLE_TIMEOUT: Duration = Duration::from_secs(30);

struct PoolInner {
    q: VecDeque<Job>,
    total_workers: usize,
    idle_workers: usize,
    min_workers: usize,
}

struct Pool {
    state: Mutex<PoolInner>,
    cv: Condvar,
}

fn spawn_pool_worker(p: &'static Pool) -> std::io::Result<()> {
    let body: Box<dyn FnOnce() -> u32 + Send> = Box::new(move || loop {
        let job = {
            let mut st = p.state.lock().unwrap();
            loop {
                if let Some(j) = st.q.pop_front() {
                    break Some(j);
                }
                st.idle_workers += 1;
                if st.total_workers > st.min_workers {
                    let (guard, timeout) = p.cv.wait_timeout(st, OVERFLOW_IDLE_TIMEOUT).unwrap();
                    st = guard;
                    st.idle_workers -= 1;
                    if timeout.timed_out() && st.q.is_empty() && st.total_workers > st.min_workers {
                        st.total_workers -= 1;
                        break None;
                    }
                } else {
                    st = p.cv.wait(st).unwrap();
                    st.idle_workers -= 1;
                }
            }
        };
        match job {
            Some(j) => j(),
            None => return 0,
        }
    });
    thread::spawn(body, 256 * 1024, false, false).map(|_| ())
}

fn pool() -> &'static Pool {
    static P: OnceLock<&'static Pool> = OnceLock::new();
    P.get_or_init(|| {
        let min_workers = crate::process::num_cpus().clamp(2, 8) as usize;
        let p: &'static Pool = Box::leak(Box::new(Pool {
            state: Mutex::new(PoolInner {
                q: VecDeque::new(),
                total_workers: min_workers,
                idle_workers: 0,
                min_workers,
            }),
            cv: Condvar::new(),
        }));
        for _ in 0..min_workers {
            if let Err(e) = spawn_pool_worker(p) {
                log::error!("thread pool: cannot spawn worker: {e}");
                let mut st = p.state.lock().unwrap();
                st.total_workers = st.total_workers.saturating_sub(1);
            }
        }
        p
    })
}

fn maybe_spawn_overflow(p: &'static Pool) {
    let should_spawn = {
        let mut st = p.state.lock().unwrap();
        if st.idle_workers == 0 && st.total_workers < MAX_POOL_WORKERS {
            st.total_workers += 1;
            true
        } else {
            false
        }
    };
    if should_spawn {
        if let Err(e) = spawn_pool_worker(p) {
            log::error!("thread pool: cannot spawn overflow worker: {e}");
            let mut st = p.state.lock().unwrap();
            st.total_workers = st.total_workers.saturating_sub(1);
        }
    }
}

fn submit(job: Job) {
    let p = pool();
    let should_spawn = {
        let mut st = p.state.lock().unwrap();
        st.q.push_back(job);
        let need_overflow = st.q.len() > st.idle_workers && st.total_workers < MAX_POOL_WORKERS;
        if need_overflow {
            st.total_workers += 1;
        }
        p.cv.notify_one();
        need_overflow
    };
    if should_spawn {
        if let Err(e) = spawn_pool_worker(p) {
            log::error!("thread pool: cannot spawn overflow worker: {e}");
            let mut st = p.state.lock().unwrap();
            st.total_workers = st.total_workers.saturating_sub(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Callback instance (SetEventWhenCallbackReturns)
// ---------------------------------------------------------------------------

const CB_INST_MAGIC: u64 = 0x5450_4342_494E_5354; // "TPCBINST"

#[repr(C)]
struct CallbackInstance {
    magic: u64,
    event_on_return: HANDLE,
}

thread_local! {
    static CURRENT_CB_INST: Cell<*mut CallbackInstance> = const { Cell::new(std::ptr::null_mut()) };
}

struct CallbackInstanceGuard {
    inst: *mut CallbackInstance,
    prev: *mut CallbackInstance,
}

impl CallbackInstanceGuard {
    unsafe fn enter(inst: &mut CallbackInstance) -> Self {
        let ptr = inst as *mut CallbackInstance;
        let prev = CURRENT_CB_INST.with(|c| c.replace(ptr));
        Self { inst: ptr, prev }
    }
}

impl Drop for CallbackInstanceGuard {
    fn drop(&mut self) {
        CURRENT_CB_INST.with(|c| c.set(self.prev));
        let ev = unsafe { (*self.inst).event_on_return };
        if !ev.is_null() {
            unsafe {
                crate::kernel32::threads::SetEvent(ev);
            }
        }
    }
}

/// Run a threadpool callback with a live `PTP_CALLBACK_INSTANCE` so
/// `SetEventWhenCallbackReturns` defers signaling until the callback returns.
unsafe fn with_callback_instance<F: FnOnce(*mut c_void)>(f: F) {
    let mut inst = CallbackInstance {
        magic: CB_INST_MAGIC,
        event_on_return: std::ptr::null_mut(),
    };
    let inst_ptr = &mut inst as *mut CallbackInstance as *mut c_void;
    let _guard = CallbackInstanceGuard::enter(&mut inst);
    f(inst_ptr);
}

/// Outstanding-callback counter used by WaitForThreadpool*Callbacks.
struct Pending {
    n: Mutex<usize>,
    cv: Condvar,
}
impl Pending {
    fn new() -> Self {
        Pending { n: Mutex::new(0), cv: Condvar::new() }
    }
    fn inc(&self) {
        *self.n.lock().unwrap() += 1;
    }
    fn dec(&self) {
        let mut n = self.n.lock().unwrap();
        *n = n.saturating_sub(1);
        if *n == 0 {
            self.cv.notify_all();
        }
    }
    fn wait_zero(&self) {
        let mut n = self.n.lock().unwrap();
        while *n > 0 {
            n = self.cv.wait(n).unwrap();
        }
    }
}

#[derive(Clone, Copy)]
struct SendPtr(*mut c_void);
unsafe impl Send for SendPtr {}
unsafe impl Sync for SendPtr {}

unsafe fn clone_arc_from_raw<T>(ptr: *const T) -> Arc<T> {
    Arc::increment_strong_count(ptr);
    Arc::from_raw(ptr)
}

// ---------------------------------------------------------------------------
// Work
// ---------------------------------------------------------------------------

pub struct TpWork {
    cb: WorkCb,
    ctx: SendPtr,
    pending: Pending,
}

pub unsafe extern "win64" fn CreateThreadpoolWork(cb: WorkCb, ctx: *mut c_void, _env: *mut c_void) -> *mut TpWork {
    Arc::into_raw(Arc::new(TpWork { cb, ctx: SendPtr(ctx), pending: Pending::new() })) as *mut TpWork
}

pub unsafe extern "win64" fn SubmitThreadpoolWork(w: *mut TpWork) {
    if w.is_null() {
        return;
    }
    let arc = clone_arc_from_raw(w);
    let raw = w as u64;
    arc.pending.inc();
    submit(Box::new(move || {
        unsafe {
            with_callback_instance(|inst| {
                crate::call_guest8(arc.cb as usize, inst as u64, arc.ctx.0 as u64, raw, 0, 0, 0, 0, 0);
            });
        }
        arc.pending.dec();
    }));
}

pub unsafe extern "win64" fn WaitForThreadpoolWorkCallbacks(w: *mut TpWork, cancel: BOOL) {
    if w.is_null() {
        return;
    }
    if cancel != 0 {
        log::debug!("WaitForThreadpoolWorkCallbacks: cancel of queued items not supported; waiting instead");
    }
    (*w).pending.wait_zero();
}

pub unsafe extern "win64" fn CloseThreadpoolWork(w: *mut TpWork) {
    if !w.is_null() {
        drop(Arc::from_raw(w));
    }
}

pub unsafe extern "win64" fn TrySubmitThreadpoolCallback(cb: SimpleCb, ctx: *mut c_void, _env: *mut c_void) -> BOOL {
    let cb_addr = cb as usize;
    let c = SendPtr(ctx);
    submit(Box::new(move || {
        let c = c;
        unsafe {
            with_callback_instance(|inst| {
                crate::call_guest8(cb_addr, inst as u64, c.0 as u64, 0, 0, 0, 0, 0, 0);
            });
        }
    }));
    TRUE
}

pub unsafe extern "win64" fn QueueUserWorkItem(f: StartRoutine, ctx: *mut c_void, _flags: u32) -> BOOL {
    let f_addr = f as usize;
    let c = SendPtr(ctx);
    submit(Box::new(move || {
        let c = c;
        unsafe {
            crate::call_guest8(f_addr, c.0 as u64, 0, 0, 0, 0, 0, 0, 0);
        }
    }));
    TRUE
}

// ---------------------------------------------------------------------------
// Timers
// ---------------------------------------------------------------------------

pub struct TpTimer {
    cb: WorkCb,
    ctx: SendPtr,
    pending: Pending,
    generation: AtomicU64,
    /// (period_ms, armed)
    state: Mutex<(u32, bool)>,
    _owner: Option<Arc<dyn Any + Send + Sync>>,
}

struct TimerEntry {
    due: Instant,
    timer: Arc<TpTimer>,
    generation: u64,
}
impl PartialEq for TimerEntry {
    fn eq(&self, o: &Self) -> bool {
        self.due == o.due && Arc::ptr_eq(&self.timer, &o.timer) && self.generation == o.generation
    }
}
impl Eq for TimerEntry {}
impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for TimerEntry {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        o.due.cmp(&self.due) // min-heap
    }
}

struct TimerQueue {
    heap: Mutex<BinaryHeap<TimerEntry>>,
    cv: Condvar,
}

fn timers() -> &'static TimerQueue {
    static T: OnceLock<&'static TimerQueue> = OnceLock::new();
    T.get_or_init(|| {
        let tq: &'static TimerQueue = Box::leak(Box::new(TimerQueue {
            heap: Mutex::new(BinaryHeap::new()),
            cv: Condvar::new(),
        }));
        std::thread::Builder::new()
            .name("aw2-tp-timer".into())
            .spawn(move || loop {
                // Pop due entry and release `tq.heap` BEFORE touching timer state or calling `submit()`.
                let e = {
                    let mut h = tq.heap.lock().unwrap();
                    loop {
                        let now = Instant::now();
                        match h.peek() {
                            None => h = tq.cv.wait(h).unwrap(),
                            Some(e) if e.due > now => {
                                let d = e.due - now;
                                h = tq.cv.wait_timeout(h, d).unwrap().0;
                            }
                            Some(_) => break h.pop().unwrap(),
                        }
                    }
                };
                fire_timer(tq, e);
            })
            .expect("timer thread");
        tq
    })
}

fn fire_timer(tq: &TimerQueue, e: TimerEntry) {
    let next_due = {
        let mut st = e.timer.state.lock().unwrap();
        if !st.1 || e.timer.generation.load(Ordering::Acquire) != e.generation {
            return;
        }
        let period = st.0;
        if period > 0 {
            Some((e.due + Duration::from_millis(period as u64)).max(Instant::now()))
        } else {
            st.1 = false;
            None
        }
    };
    let t_arc = Arc::clone(&e.timer);
    let raw = Arc::as_ptr(&t_arc) as u64;
    let gen = e.generation;
    t_arc.pending.inc();
    submit(Box::new(move || {
        if t_arc.generation.load(Ordering::Acquire) == gen {
            unsafe {
                with_callback_instance(|inst| {
                    crate::call_guest8(t_arc.cb as usize, inst as u64, t_arc.ctx.0 as u64, raw, 0, 0, 0, 0, 0);
                });
            }
        }
        t_arc.pending.dec();
    }));
    if let Some(due) = next_due {
        if e.timer.generation.load(Ordering::Acquire) == e.generation {
            let mut h = tq.heap.lock().unwrap();
            if e.timer.generation.load(Ordering::Acquire) == e.generation {
                h.push(TimerEntry { due, timer: e.timer, generation: e.generation });
            }
        }
    }
}

fn create_timer_inner(cb: WorkCb, ctx: *mut c_void, owner: Option<Arc<dyn Any + Send + Sync>>) -> *mut TpTimer {
    Arc::into_raw(Arc::new(TpTimer {
        cb,
        ctx: SendPtr(ctx),
        pending: Pending::new(),
        generation: AtomicU64::new(0),
        state: Mutex::new((0, false)),
        _owner: owner,
    })) as *mut TpTimer
}

pub unsafe extern "win64" fn CreateThreadpoolTimer(cb: WorkCb, ctx: *mut c_void, _env: *mut c_void) -> *mut TpTimer {
    create_timer_inner(cb, ctx, None)
}

/// `due`: FILETIME; negative = relative 100ns, positive = absolute UTC; NULL = cancel.
pub unsafe extern "win64" fn SetThreadpoolTimer(t: *mut TpTimer, due: *const FILETIME, period: u32, _window: u32) {
    if t.is_null() {
        return;
    }
    let arc = clone_arc_from_raw(t);
    let gen = {
        let mut st = arc.state.lock().unwrap();
        let gen = arc.generation.fetch_add(1, Ordering::AcqRel) + 1;
        if due.is_null() {
            st.1 = false;
            gen
        } else {
            st.0 = period;
            st.1 = true;
            gen
        }
    };
    let tq = timers();
    let mut h = tq.heap.lock().unwrap();
    h.retain(|e| !Arc::ptr_eq(&e.timer, &arc));
    if !due.is_null() && arc.generation.load(Ordering::Acquire) == gen {
        let v = std::ptr::read_unaligned(due).to_u64() as i64;
        let delay = if v < 0 {
            Duration::from_nanos(v.unsigned_abs().saturating_mul(100))
        } else if v == 0 {
            Duration::ZERO
        } else {
            let now = now_filetime() as i64;
            Duration::from_nanos(((v - now).max(0) as u64).saturating_mul(100))
        };
        h.push(TimerEntry { due: Instant::now() + delay, timer: arc, generation: gen });
        tq.cv.notify_all();
    }
}

pub unsafe extern "win64" fn IsThreadpoolTimerSet(t: *mut TpTimer) -> BOOL {
    if t.is_null() {
        return FALSE;
    }
    (*t).state.lock().unwrap().1 as BOOL
}

pub unsafe extern "win64" fn WaitForThreadpoolTimerCallbacks(t: *mut TpTimer, cancel: BOOL) {
    if t.is_null() {
        return;
    }
    if cancel != 0 {
        {
            let mut st = (*t).state.lock().unwrap();
            (*t).generation.fetch_add(1, Ordering::AcqRel);
            st.1 = false;
        }
        let tq = timers();
        let ptr = t as *const TpTimer;
        tq.heap.lock().unwrap().retain(|e| Arc::as_ptr(&e.timer) != ptr);
    }
    (*t).pending.wait_zero();
}

pub unsafe extern "win64" fn CloseThreadpoolTimer(t: *mut TpTimer) {
    if t.is_null() {
        return;
    }
    let arc = Arc::from_raw(t);
    {
        let mut st = arc.state.lock().unwrap();
        arc.generation.fetch_add(1, Ordering::AcqRel);
        st.1 = false;
    }
    let tq = timers();
    tq.heap.lock().unwrap().retain(|e| !Arc::ptr_eq(&e.timer, &arc));
    drop(arc);
}

// ---------------------------------------------------------------------------
// Waits (multiplexed single-dispatcher design — zero thread/stack/slot leaks)
// ---------------------------------------------------------------------------

pub struct TpWait {
    cb: WaitCb,
    ctx: SendPtr,
    pending: Pending,
    generation: AtomicU64,
    _owner: Option<Arc<dyn Any + Send + Sync>>,
}

struct ArmedWait {
    wait: Arc<TpWait>,
    generation: u64,
    obj: Arc<dyn handles::KObject>,
    deadline: Option<Instant>,
}

struct WaitDispatcher {
    wake: Arc<handles::Event>,
    armed: Mutex<Vec<ArmedWait>>,
}

fn submit_wait_callback(wait: Arc<TpWait>, generation: u64, res: u32) {
    let raw = Arc::as_ptr(&wait) as u64;
    submit(Box::new(move || {
        if wait.generation.load(Ordering::Acquire) == generation {
            unsafe {
                with_callback_instance(|inst| {
                    crate::call_guest8(
                        wait.cb as usize,
                        inst as u64,
                        wait.ctx.0 as u64,
                        raw,
                        res as u64,
                        0,
                        0,
                        0,
                        0,
                    );
                });
            }
        }
        wait.pending.dec();
    }));
}

fn wait_dispatcher() -> &'static WaitDispatcher {
    static D: OnceLock<&'static WaitDispatcher> = OnceLock::new();
    D.get_or_init(|| {
        let disp: &'static WaitDispatcher = Box::leak(Box::new(WaitDispatcher {
            wake: Arc::new(handles::Event::new(false, false, None)),
            armed: Mutex::new(Vec::new()),
        }));
        std::thread::Builder::new()
            .name("win32-tp-wait".into())
            .spawn(move || {
                let wake_obj: Arc<dyn handles::KObject> = disp.wake.clone();
                let mut wait_objs: Vec<Arc<dyn handles::KObject>> = Vec::new();
                let mut meta: Vec<(usize, u64)> = Vec::new();
                let mut expired: Vec<(Arc<TpWait>, u64, u32)> = Vec::new();
                loop {
                    wait_objs.clear();
                    meta.clear();
                    expired.clear();
                    let earliest: Option<Instant>;
                    {
                        let mut armed = disp.armed.lock().unwrap();
                        let now = Instant::now();
                        let mut i = 0;
                        while i < armed.len() {
                            if armed[i].wait.generation.load(Ordering::Acquire) != armed[i].generation {
                                let removed = armed.swap_remove(i);
                                removed.wait.pending.dec();
                                continue;
                            }
                            if armed[i].deadline.is_some_and(|d| d <= now) {
                                // Poll object once before reporting timeout (e.g. 0ms timeout on already-signaled handle).
                                let r = handles::wait_for_duration(
                                    std::slice::from_ref(&armed[i].obj),
                                    false,
                                    Some(Duration::ZERO),
                                    false,
                                );
                                let res = match r {
                                    handles::WaitResult::Object(_) => WAIT_OBJECT_0,
                                    handles::WaitResult::Abandoned(_) => WAIT_ABANDONED_0,
                                    handles::WaitResult::Timeout => WAIT_TIMEOUT,
                                    _ => WAIT_FAILED,
                                };
                                let removed = armed.swap_remove(i);
                                expired.push((removed.wait, removed.generation, res));
                                continue;
                            }
                            i += 1;
                        }
                        if expired.is_empty() {
                            wait_objs.push(Arc::clone(&wake_obj));
                            let mut min_dl: Option<Instant> = None;
                            for entry in armed.iter() {
                                wait_objs.push(Arc::clone(&entry.obj));
                                meta.push((Arc::as_ptr(&entry.wait) as usize, entry.generation));
                                if let Some(d) = entry.deadline {
                                    min_dl = Some(match min_dl {
                                        Some(cur) => cur.min(d),
                                        None => d,
                                    });
                                }
                            }
                            earliest = min_dl;
                        } else {
                            earliest = None;
                        }
                    }
                    if !expired.is_empty() {
                        for (w, gen, res) in expired.drain(..) {
                            submit_wait_callback(w, gen, res);
                        }
                        continue;
                    }
                    let timeout = earliest.map(|d| d.saturating_duration_since(Instant::now()));
                    let r = handles::wait_for_duration(&wait_objs, false, timeout, false);
                    match r {
                        handles::WaitResult::Object(0) => {
                            // Wake event signaled (armed list updated).
                        }
                        handles::WaitResult::Object(idx) | handles::WaitResult::Abandoned(idx) if idx >= 1 => {
                            let res = if matches!(r, handles::WaitResult::Abandoned(_)) {
                                WAIT_ABANDONED_0
                            } else {
                                WAIT_OBJECT_0
                            };
                            let (target_ptr, target_gen) = meta[idx - 1];
                            let removed = {
                                let mut armed = disp.armed.lock().unwrap();
                                armed
                                    .iter()
                                    .position(|e| {
                                        Arc::as_ptr(&e.wait) as usize == target_ptr && e.generation == target_gen
                                    })
                                    .map(|pos| armed.swap_remove(pos))
                            };
                            if let Some(entry) = removed {
                                submit_wait_callback(entry.wait, entry.generation, res);
                            }
                        }
                        _ => {}
                    }
                }
            })
            .expect("spawn win32-tp-wait thread");
        disp
    })
}

fn disarm_wait(disp: &WaitDispatcher, ptr: *const TpWait) {
    let mut removed_count = 0usize;
    {
        let mut armed = disp.armed.lock().unwrap();
        let mut i = 0;
        while i < armed.len() {
            if Arc::as_ptr(&armed[i].wait) == ptr {
                armed.swap_remove(i);
                removed_count += 1;
            } else {
                i += 1;
            }
        }
    }
    if removed_count > 0 {
        for _ in 0..removed_count {
            unsafe { (*ptr).pending.dec() };
        }
        disp.wake.set();
    }
}

fn create_wait_inner(cb: WaitCb, ctx: *mut c_void, owner: Option<Arc<dyn Any + Send + Sync>>) -> *mut TpWait {
    Arc::into_raw(Arc::new(TpWait {
        cb,
        ctx: SendPtr(ctx),
        pending: Pending::new(),
        generation: AtomicU64::new(0),
        _owner: owner,
    })) as *mut TpWait
}

pub unsafe extern "win64" fn CreateThreadpoolWait(cb: WaitCb, ctx: *mut c_void, _env: *mut c_void) -> *mut TpWait {
    create_wait_inner(cb, ctx, None)
}

pub unsafe extern "win64" fn SetThreadpoolWait(w: *mut TpWait, h: HANDLE, timeout: *const FILETIME) {
    if w.is_null() {
        return;
    }
    let arc = clone_arc_from_raw(w);
    let generation = arc.generation.fetch_add(1, Ordering::AcqRel) + 1;
    let disp = wait_dispatcher();
    disarm_wait(disp, w);
    if h.is_null() {
        return;
    }
    let Some(obj) = handles::get(h) else {
        log::warn!("SetThreadpoolWait: invalid handle {h:p}");
        return;
    };
    if obj.waitable().is_none() {
        log::warn!("SetThreadpoolWait: non-waitable handle {h:p}");
        return;
    }
    let deadline = if timeout.is_null() {
        None
    } else {
        let v = std::ptr::read_unaligned(timeout).to_u64() as i64;
        if v < 0 {
            Some(Instant::now() + Duration::from_nanos(v.unsigned_abs().saturating_mul(100)))
        } else if v == 0 {
            Some(Instant::now())
        } else {
            let now = now_filetime() as i64;
            Some(Instant::now() + Duration::from_nanos(((v - now).max(0) as u64).saturating_mul(100)))
        }
    };
    arc.pending.inc();
    {
        let mut armed = disp.armed.lock().unwrap();
        if arc.generation.load(Ordering::Acquire) == generation {
            armed.push(ArmedWait { wait: arc, generation, obj, deadline });
        } else {
            drop(armed);
            arc.pending.dec();
            return;
        }
    }
    disp.wake.set();
}

pub unsafe extern "win64" fn WaitForThreadpoolWaitCallbacks(w: *mut TpWait, cancel: BOOL) {
    if w.is_null() {
        return;
    }
    if cancel != 0 {
        (*w).generation.fetch_add(1, Ordering::AcqRel);
        disarm_wait(wait_dispatcher(), w);
    }
    (*w).pending.wait_zero();
}

pub unsafe extern "win64" fn CloseThreadpoolWait(w: *mut TpWait) {
    if w.is_null() {
        return;
    }
    let arc = Arc::from_raw(w);
    arc.generation.fetch_add(1, Ordering::AcqRel);
    disarm_wait(wait_dispatcher(), Arc::as_ptr(&arc));
    drop(arc);
}

// ---------------------------------------------------------------------------
// Callback-instance helpers
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn SetEventWhenCallbackReturns(inst: *mut c_void, ev: HANDLE) {
    if ev.is_null() {
        return;
    }
    if !inst.is_null() {
        let ci = inst as *mut CallbackInstance;
        if std::ptr::read_unaligned(&(*ci).magic) == CB_INST_MAGIC {
            (*ci).event_on_return = ev;
            return;
        }
    }
    let cur = CURRENT_CB_INST.with(|c| c.get());
    if !cur.is_null() {
        (*cur).event_on_return = ev;
        return;
    }
    log::warn!("SetEventWhenCallbackReturns called outside active threadpool callback");
}

pub unsafe extern "win64" fn CallbackMayRunLong(_inst: *mut c_void) -> BOOL {
    maybe_spawn_overflow(pool());
    TRUE
}
pub unsafe extern "win64" fn DisassociateCurrentThreadFromCallback(_inst: *mut c_void) {}

pub unsafe extern "win64" fn CreateThreadpool(_r: *mut c_void) -> *mut c_void {
    // A distinct non-null token; all work goes to the shared pool.
    Box::into_raw(Box::new(0u64)) as *mut c_void
}
pub unsafe extern "win64" fn CloseThreadpool(p: *mut c_void) {
    if !p.is_null() {
        drop(Box::from_raw(p as *mut u64));
    }
}
pub unsafe extern "win64" fn SetThreadpoolThreadMaximum(_p: *mut c_void, _n: u32) {}
pub unsafe extern "win64" fn SetThreadpoolThreadMinimum(_p: *mut c_void, _n: u32) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn CreateThreadpoolCleanupGroup() -> *mut c_void {
    Box::into_raw(Box::new(0u64)) as *mut c_void
}
pub unsafe extern "win64" fn CloseThreadpoolCleanupGroupMembers(_g: *mut c_void, _cancel: BOOL, _ctx: *mut c_void) {
    stub!("cleanup groups do not track members");
}
pub unsafe extern "win64" fn CloseThreadpoolCleanupGroup(g: *mut c_void) {
    if !g.is_null() {
        drop(Box::from_raw(g as *mut u64));
    }
}

// ---------------------------------------------------------------------------
// Legacy timer queues & RegisterWaitForSingleObject
// ---------------------------------------------------------------------------

type WaitOrTimerCb = unsafe extern "win64" fn(*mut c_void, u8);

struct LegacyTimerCtx {
    cb: WaitOrTimerCb,
    param: SendPtr,
    tp_timer: AtomicPtr<TpTimer>,
}

unsafe extern "win64" fn legacy_timer_trampoline(_inst: *mut c_void, ctx: *mut c_void, _timer: *mut c_void) {
    let c = &*(ctx as *const LegacyTimerCtx);
    crate::call_guest8(c.cb as usize, c.param.0 as u64, 1, 0, 0, 0, 0, 0, 0);
}

pub unsafe extern "win64" fn CreateTimerQueue() -> HANDLE {
    Box::into_raw(Box::new(0u64)) as HANDLE
}

pub unsafe extern "win64" fn CreateTimerQueueTimer(
    out_timer: *mut HANDLE,
    _timer_queue: HANDLE,
    callback: Option<WaitOrTimerCb>,
    parameter: *mut c_void,
    due_time: u32,
    period: u32,
    _flags: u32,
) -> BOOL {
    if out_timer.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(cb) = callback else {
        std::ptr::write_unaligned(out_timer, std::ptr::null_mut());
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let ctx = Arc::new(LegacyTimerCtx {
        cb,
        param: SendPtr(parameter),
        tp_timer: AtomicPtr::new(std::ptr::null_mut()),
    });
    let ctx_ptr = Arc::as_ptr(&ctx) as *mut c_void;
    let owner: Arc<dyn std::any::Any + Send + Sync> = ctx.clone();
    let tp = create_timer_inner(legacy_timer_trampoline, ctx_ptr, Some(owner));
    ctx.tp_timer.store(tp, Ordering::Release);
    let due_100ns = -((due_time as i64) * 10_000);
    let ft = FILETIME::from_u64(due_100ns as u64);
    SetThreadpoolTimer(tp, &ft, period, 0);
    std::ptr::write_unaligned(out_timer, Arc::into_raw(ctx) as HANDLE);
    TRUE
}

pub unsafe extern "win64" fn ChangeTimerQueueTimer(
    _timer_queue: HANDLE,
    timer: HANDLE,
    due_time: u32,
    period: u32,
) -> BOOL {
    if timer.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let ctx = &*(timer as *const LegacyTimerCtx);
    let tp = ctx.tp_timer.load(Ordering::Acquire);
    let due_100ns = -((due_time as i64) * 10_000);
    let ft = FILETIME::from_u64(due_100ns as u64);
    SetThreadpoolTimer(tp, &ft, period, 0);
    TRUE
}

pub unsafe extern "win64" fn DeleteTimerQueueTimer(
    _timer_queue: HANDLE,
    timer: HANDLE,
    completion_event: HANDLE,
) -> BOOL {
    if !timer.is_null() {
        let ctx = Arc::from_raw(timer as *const LegacyTimerCtx);
        let tp = ctx.tp_timer.swap(std::ptr::null_mut(), Ordering::AcqRel);
        if !tp.is_null() {
            SetThreadpoolTimer(tp, std::ptr::null(), 0, 0);
            if completion_event as usize == usize::MAX {
                WaitForThreadpoolTimerCallbacks(tp, TRUE);
            }
            CloseThreadpoolTimer(tp);
        }
        drop(ctx);
    }
    if !completion_event.is_null() && completion_event as usize != usize::MAX {
        crate::kernel32::threads::SetEvent(completion_event);
    }
    TRUE
}

pub unsafe extern "win64" fn DeleteTimerQueueEx(timer_queue: HANDLE, completion_event: HANDLE) -> BOOL {
    if !timer_queue.is_null() {
        drop(Box::from_raw(timer_queue as *mut u64));
    }
    if !completion_event.is_null() && completion_event as usize != usize::MAX {
        crate::kernel32::threads::SetEvent(completion_event);
    }
    TRUE
}

pub unsafe extern "win64" fn DeleteTimerQueue(timer_queue: HANDLE) -> BOOL {
    DeleteTimerQueueEx(timer_queue, std::ptr::null_mut())
}

struct LegacyWaitCtx {
    cb: WaitOrTimerCb,
    param: SendPtr,
    tp_wait: AtomicPtr<TpWait>,
}

unsafe extern "win64" fn legacy_wait_trampoline(_inst: *mut c_void, ctx: *mut c_void, _wait: *mut c_void, wait_result: u32) {
    let c = &*(ctx as *const LegacyWaitCtx);
    let timed_out = (wait_result == WAIT_TIMEOUT) as u8;
    crate::call_guest8(c.cb as usize, c.param.0 as u64, timed_out as u64, 0, 0, 0, 0, 0, 0);
}

pub unsafe extern "win64" fn RegisterWaitForSingleObject(
    out_new_wait_object: *mut HANDLE,
    object: HANDLE,
    callback: Option<WaitOrTimerCb>,
    context: *mut c_void,
    milliseconds: u32,
    _flags: u32,
) -> BOOL {
    if out_new_wait_object.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(cb) = callback else {
        std::ptr::write_unaligned(out_new_wait_object, std::ptr::null_mut());
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let ctx = Arc::new(LegacyWaitCtx {
        cb,
        param: SendPtr(context),
        tp_wait: AtomicPtr::new(std::ptr::null_mut()),
    });
    let ctx_ptr = Arc::as_ptr(&ctx) as *mut c_void;
    let owner: Arc<dyn std::any::Any + Send + Sync> = ctx.clone();
    let tp = create_wait_inner(legacy_wait_trampoline, ctx_ptr, Some(owner));
    ctx.tp_wait.store(tp, Ordering::Release);
    let ft;
    let p_ft = if milliseconds == INFINITE {
        std::ptr::null()
    } else {
        let rel_100ns = -((milliseconds as i64) * 10_000);
        ft = FILETIME::from_u64(rel_100ns as u64);
        &ft as *const FILETIME
    };
    SetThreadpoolWait(tp, object, p_ft);
    std::ptr::write_unaligned(out_new_wait_object, Arc::into_raw(ctx) as HANDLE);
    TRUE
}

pub unsafe extern "win64" fn UnregisterWaitEx(wait_handle: HANDLE, completion_event: HANDLE) -> BOOL {
    if !wait_handle.is_null() {
        let ctx = Arc::from_raw(wait_handle as *const LegacyWaitCtx);
        let tp = ctx.tp_wait.swap(std::ptr::null_mut(), Ordering::AcqRel);
        if !tp.is_null() {
            SetThreadpoolWait(tp, std::ptr::null_mut(), std::ptr::null());
            if completion_event as usize == usize::MAX {
                WaitForThreadpoolWaitCallbacks(tp, TRUE);
            }
            CloseThreadpoolWait(tp);
        }
        drop(ctx);
    }
    if !completion_event.is_null() && completion_event as usize != usize::MAX {
        crate::kernel32::threads::SetEvent(completion_event);
    }
    TRUE
}

pub unsafe extern "win64" fn UnregisterWait(wait_handle: HANDLE) -> BOOL {
    UnregisterWaitEx(wait_handle, std::ptr::null_mut())
}

// ---------------------------------------------------------------------------
// Threadpool I/O (CreateThreadpoolIo / StartThreadpoolIo / CancelThreadpoolIo)
// ---------------------------------------------------------------------------

pub struct TpIo {
    _file: SendPtr,
    _cb: usize,
    _ctx: SendPtr,
    pending: std::sync::atomic::AtomicUsize,
}

pub unsafe extern "win64" fn CreateThreadpoolIo(
    fl: HANDLE,
    pfnio: *const c_void,
    pv: *mut c_void,
    _pcbe: *mut c_void,
) -> *mut TpIo {
    if fl.is_null() || fl == INVALID_HANDLE_VALUE || pfnio.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(TpIo {
        _file: SendPtr(fl),
        _cb: pfnio as usize,
        _ctx: SendPtr(pv),
        pending: std::sync::atomic::AtomicUsize::new(0),
    }))
}

pub unsafe extern "win64" fn StartThreadpoolIo(pio: *mut TpIo) {
    if !pio.is_null() {
        (*pio).pending.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

pub unsafe extern "win64" fn CancelThreadpoolIo(pio: *mut TpIo) {
    if !pio.is_null() {
        let _ = (*pio).pending.fetch_update(
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
            |v| v.checked_sub(1),
        );
    }
}

pub unsafe extern "win64" fn WaitForThreadpoolIoCallbacks(pio: *mut TpIo, cancel_pending: BOOL) {
    if !pio.is_null() && cancel_pending != 0 {
        (*pio).pending.store(0, std::sync::atomic::Ordering::Relaxed);
    }
}

pub unsafe extern "win64" fn CloseThreadpoolIo(pio: *mut TpIo) {
    if !pio.is_null() {
        drop(Box::from_raw(pio));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threadpool_io_lifecycle() {
        unsafe {
            assert!(CreateThreadpoolIo(std::ptr::null_mut(), 0x1000 as *const c_void, std::ptr::null_mut(), std::ptr::null_mut()).is_null());
            let pio = CreateThreadpoolIo(0x100 as HANDLE, 0x1000 as *const c_void, std::ptr::null_mut(), std::ptr::null_mut());
            assert!(!pio.is_null());
            StartThreadpoolIo(pio);
            assert_eq!((*pio).pending.load(std::sync::atomic::Ordering::Relaxed), 1);
            CancelThreadpoolIo(pio);
            assert_eq!((*pio).pending.load(std::sync::atomic::Ordering::Relaxed), 0);
            StartThreadpoolIo(pio);
            WaitForThreadpoolIoCallbacks(pio, TRUE);
            assert_eq!((*pio).pending.load(std::sync::atomic::Ordering::Relaxed), 0);
            CloseThreadpoolIo(pio);
        }
    }

    #[test]
    fn set_threadpool_wait_cancel_rearm_no_leak() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, ResetEvent, SetEvent};
        use std::sync::atomic::AtomicU32;

        struct WaitCtx {
            count: AtomicU32,
            last_res: AtomicU32,
        }
        unsafe extern "win64" fn on_wait(
            _inst: *mut c_void,
            ctx: *mut c_void,
            _wait: *mut c_void,
            wait_res: u32,
        ) {
            let c = &*(ctx as *const WaitCtx);
            c.last_res.store(wait_res, Ordering::Release);
            c.count.fetch_add(1, Ordering::AcqRel);
        }

        unsafe {
            let ctx = Box::new(WaitCtx {
                count: AtomicU32::new(0),
                last_res: AtomicU32::new(u32::MAX),
            });
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!ev.is_null());
            let w = CreateThreadpoolWait(on_wait, &*ctx as *const WaitCtx as *mut c_void, std::ptr::null_mut());
            assert!(!w.is_null());

            // 150 arm -> re-arm/cancel cycles must reuse the single dispatcher with zero slot/thread leaks.
            for _ in 0..150 {
                SetThreadpoolWait(w, ev, std::ptr::null());
                SetThreadpoolWait(w, std::ptr::null_mut(), std::ptr::null());
            }
            WaitForThreadpoolWaitCallbacks(w, TRUE);
            assert_eq!(ctx.count.load(Ordering::Acquire), 0);

            // Real signaled trigger.
            SetThreadpoolWait(w, ev, std::ptr::null());
            assert_eq!(SetEvent(ev), TRUE);
            WaitForThreadpoolWaitCallbacks(w, FALSE);
            assert_eq!(ctx.count.load(Ordering::Acquire), 1);
            assert_eq!(ctx.last_res.load(Ordering::Acquire), WAIT_OBJECT_0);

            // Real timeout trigger (10 ms relative).
            assert_eq!(ResetEvent(ev), TRUE);
            let rel = FILETIME::from_u64((-100_000i64) as u64);
            SetThreadpoolWait(w, ev, &rel);
            WaitForThreadpoolWaitCallbacks(w, FALSE);
            assert_eq!(ctx.count.load(Ordering::Acquire), 2);
            assert_eq!(ctx.last_res.load(Ordering::Acquire), WAIT_TIMEOUT);

            CloseThreadpoolWait(w);
            assert_eq!(CloseHandle(ev), TRUE);
        }
    }

    #[test]
    fn set_event_when_callback_returns_signals_after_return() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};
        use std::sync::atomic::AtomicBool;

        struct ReturnEventCtx {
            ev: usize,
            unsignaled_during_cb: AtomicBool,
            body_completed: AtomicBool,
        }
        unsafe extern "win64" fn work_cb(inst: *mut c_void, ctx: *mut c_void, _work: *mut c_void) {
            let c = &*(ctx as *const ReturnEventCtx);
            let ev = c.ev as HANDLE;
            SetEventWhenCallbackReturns(inst, ev);
            // Event must NOT be signaled yet while the callback is still running.
            let mid = WaitForSingleObject(ev, 0);
            c.unsignaled_during_cb.store(mid == WAIT_TIMEOUT, Ordering::Release);
            std::thread::sleep(Duration::from_millis(15));
            c.body_completed.store(true, Ordering::Release);
        }

        unsafe {
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!ev.is_null());
            let ctx = Box::new(ReturnEventCtx {
                ev: ev as usize,
                unsignaled_during_cb: AtomicBool::new(false),
                body_completed: AtomicBool::new(false),
            });
            let w = CreateThreadpoolWork(work_cb, &*ctx as *const ReturnEventCtx as *mut c_void, std::ptr::null_mut());
            assert!(!w.is_null());
            SubmitThreadpoolWork(w);
            assert_eq!(WaitForSingleObject(ev, 5_000), WAIT_OBJECT_0);
            assert!(ctx.unsignaled_during_cb.load(Ordering::Acquire), "event was signaled before callback returned");
            assert!(ctx.body_completed.load(Ordering::Acquire), "event was signaled before callback body completed");
            WaitForThreadpoolWorkCallbacks(w, FALSE);
            CloseThreadpoolWork(w);
            assert_eq!(CloseHandle(ev), TRUE);
        }
    }

    #[test]
    fn threadpool_overflow_workers_prevent_nested_deadlock() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, SetEvent, WaitForSingleObject};
        use std::sync::Barrier;

        struct NestedCtx {
            barrier: Arc<Barrier>,
            child_ev: usize,
        }
        unsafe extern "win64" fn child_cb(_inst: *mut c_void, ctx: *mut c_void, _work: *mut c_void) {
            let ev = ctx as HANDLE;
            SetEvent(ev);
        }
        unsafe extern "win64" fn parent_cb(_inst: *mut c_void, ctx: *mut c_void, _work: *mut c_void) {
            let c = Box::from_raw(ctx as *mut NestedCtx);
            // Occupy every core worker simultaneously before submitting a nested child item.
            c.barrier.wait();
            let cw = CreateThreadpoolWork(child_cb, c.child_ev as *mut c_void, std::ptr::null_mut());
            SubmitThreadpoolWork(cw);
            assert_eq!(WaitForSingleObject(c.child_ev as HANDLE, 5_000), WAIT_OBJECT_0);
            WaitForThreadpoolWorkCallbacks(cw, FALSE);
            CloseThreadpoolWork(cw);
        }

        let n = crate::process::num_cpus().clamp(2, 8) as usize;
        let barrier = Arc::new(Barrier::new(n));
        let mut works = Vec::with_capacity(n);
        let mut events = Vec::with_capacity(n);
        unsafe {
            for _ in 0..n {
                let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
                events.push(ev);
                let ctx = Box::into_raw(Box::new(NestedCtx {
                    barrier: Arc::clone(&barrier),
                    child_ev: ev as usize,
                }));
                let w = CreateThreadpoolWork(parent_cb, ctx as *mut c_void, std::ptr::null_mut());
                works.push(w);
                SubmitThreadpoolWork(w);
            }
            for w in works {
                WaitForThreadpoolWorkCallbacks(w, FALSE);
                CloseThreadpoolWork(w);
            }
            for ev in events {
                assert_eq!(CloseHandle(ev), TRUE);
            }
        }
    }
}
