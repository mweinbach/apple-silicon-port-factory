//! Vista thread pool (`CreateThreadpoolWork` & co.) and `QueueUserWorkItem`.
//!
//! A small fixed pool of Windows threads (with TEBs, static TLS and
//! DLL_THREAD_ATTACH, like real pool threads) executes callbacks. Timers use
//! one host timer thread that enqueues expirations; waits use one Windows
//! thread per armed wait. Callback environments (priority, cleanup groups,
//! pools) are accepted but ignored.

use crate::handles;
use crate::thread;
use crate::types::*;
use std::collections::{BinaryHeap, VecDeque};
use std::ffi::c_void;
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

type WorkCb = unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void);
type WaitCb = unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void, u32);
type SimpleCb = unsafe extern "win64" fn(*mut c_void, *mut c_void);
type StartRoutine = unsafe extern "win64" fn(*mut c_void) -> u32;

type Job = Box<dyn FnOnce() + Send>;

struct Pool {
    q: Mutex<VecDeque<Job>>,
    cv: Condvar,
}

fn pool() -> &'static Pool {
    static P: OnceLock<&'static Pool> = OnceLock::new();
    P.get_or_init(|| {
        let p: &'static Pool = Box::leak(Box::new(Pool { q: Mutex::new(VecDeque::new()), cv: Condvar::new() }));
        let n = crate::process::num_cpus().clamp(2, 8);
        for _ in 0..n {
            let body: Box<dyn FnOnce() -> u32 + Send> = Box::new(move || loop {
                let job = {
                    let mut q = p.q.lock().unwrap();
                    loop {
                        if let Some(j) = q.pop_front() {
                            break j;
                        }
                        q = p.cv.wait(q).unwrap();
                    }
                };
                job();
            });
            if let Err(e) = thread::spawn(body, 256 * 1024, false, false) {
                log::error!("thread pool: cannot spawn worker: {e}");
            }
        }
        p
    })
}

fn submit(job: Job) {
    let p = pool();
    p.q.lock().unwrap().push_back(job);
    p.cv.notify_one();
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
        *n -= 1;
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

// ---------------------------------------------------------------------------
// Work
// ---------------------------------------------------------------------------

pub struct TpWork {
    cb: WorkCb,
    ctx: SendPtr,
    pending: Pending,
}

pub unsafe extern "win64" fn CreateThreadpoolWork(cb: WorkCb, ctx: *mut c_void, _env: *mut c_void) -> *mut TpWork {
    Box::into_raw(Box::new(TpWork { cb, ctx: SendPtr(ctx), pending: Pending::new() }))
}

pub unsafe extern "win64" fn SubmitThreadpoolWork(w: *mut TpWork) {
    let w = SendPtr(w as *mut c_void);
    let work = &*(w.0 as *const TpWork);
    work.pending.inc();
    submit(Box::new(move || {
        let w = w;
        let wk = &*(w.0 as *const TpWork);
        crate::call_guest8(wk.cb as usize, 0, wk.ctx.0 as u64, w.0 as u64, 0, 0, 0, 0, 0);
        wk.pending.dec();
    }));
}

pub unsafe extern "win64" fn WaitForThreadpoolWorkCallbacks(w: *mut TpWork, cancel: BOOL) {
    if cancel != 0 {
        log::debug!("WaitForThreadpoolWorkCallbacks: cancel of queued items not supported; waiting instead");
    }
    (*w).pending.wait_zero();
}

pub unsafe extern "win64" fn CloseThreadpoolWork(w: *mut TpWork) {
    // Objects stay valid while callbacks run; leak instead of racing them.
    let _ = w;
}

pub unsafe extern "win64" fn TrySubmitThreadpoolCallback(cb: SimpleCb, ctx: *mut c_void, _env: *mut c_void) -> BOOL {
    let cb_addr = cb as usize;
    let c = SendPtr(ctx);
    submit(Box::new(move || {
        let c = c;
        crate::call_guest8(cb_addr, 0, c.0 as u64, 0, 0, 0, 0, 0, 0);
    }));
    TRUE
}

pub unsafe extern "win64" fn QueueUserWorkItem(f: StartRoutine, ctx: *mut c_void, _flags: u32) -> BOOL {
    let f_addr = f as usize;
    let c = SendPtr(ctx);
    submit(Box::new(move || {
        let c = c;
        crate::call_guest8(f_addr, c.0 as u64, 0, 0, 0, 0, 0, 0, 0);
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
    /// (generation, period_ms, armed)
    state: Mutex<(u64, u32, bool)>,
}

struct TimerEntry {
    due: Instant,
    timer: SendPtr,
    generation: u64,
}
impl PartialEq for TimerEntry {
    fn eq(&self, o: &Self) -> bool {
        self.due == o.due
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
        let tq: &'static TimerQueue = Box::leak(Box::new(TimerQueue { heap: Mutex::new(BinaryHeap::new()), cv: Condvar::new() }));
        std::thread::Builder::new()
            .name("aw2-tp-timer".into())
            .spawn(move || {
                let mut h = tq.heap.lock().unwrap();
                loop {
                    let now = Instant::now();
                    match h.peek() {
                        None => h = tq.cv.wait(h).unwrap(),
                        Some(e) if e.due > now => {
                            let d = e.due - now;
                            h = tq.cv.wait_timeout(h, d).unwrap().0;
                        }
                        Some(_) => {
                            let e = h.pop().unwrap();
                            fire_timer(tq, &mut h, e);
                        }
                    }
                }
            })
            .expect("timer thread");
        tq
    })
}

fn fire_timer(_tq: &TimerQueue, heap: &mut BinaryHeap<TimerEntry>, e: TimerEntry) {
    let t = unsafe { &*(e.timer.0 as *const TpTimer) };
    let (generation, period, armed) = *t.state.lock().unwrap();
    if !armed || generation != e.generation {
        return;
    }
    if period > 0 {
        heap.push(TimerEntry { due: e.due + Duration::from_millis(period as u64), timer: e.timer, generation });
    } else {
        t.state.lock().unwrap().2 = false;
    }
    t.pending.inc();
    let tp = e.timer;
    submit(Box::new(move || {
        let tp = tp;
        let t = unsafe { &*(tp.0 as *const TpTimer) };
        unsafe { crate::call_guest8(t.cb as usize, 0, t.ctx.0 as u64, tp.0 as u64, 0, 0, 0, 0, 0) };
        t.pending.dec();
    }));
}

pub unsafe extern "win64" fn CreateThreadpoolTimer(cb: WorkCb, ctx: *mut c_void, _env: *mut c_void) -> *mut TpTimer {
    Box::into_raw(Box::new(TpTimer { cb, ctx: SendPtr(ctx), pending: Pending::new(), state: Mutex::new((0, 0, false)) }))
}

/// `due`: FILETIME; negative = relative 100ns, positive = absolute UTC; NULL = cancel.
pub unsafe extern "win64" fn SetThreadpoolTimer(t: *mut TpTimer, due: *const FILETIME, period: u32, _window: u32) {
    let tq = timers();
    let mut h = tq.heap.lock().unwrap();
    let mut st = (*t).state.lock().unwrap();
    st.0 += 1;
    if due.is_null() {
        st.2 = false;
        return;
    }
    let v = (*due).to_u64() as i64;
    let delay = if v < 0 {
        Duration::from_nanos((-v) as u64 * 100)
    } else if v == 0 {
        Duration::ZERO
    } else {
        let now = now_filetime() as i64;
        Duration::from_nanos((v - now).max(0) as u64 * 100)
    };
    st.1 = period;
    st.2 = true;
    h.push(TimerEntry { due: Instant::now() + delay, timer: SendPtr(t as *mut c_void), generation: st.0 });
    drop(st);
    tq.cv.notify_all();
}

pub unsafe extern "win64" fn IsThreadpoolTimerSet(t: *mut TpTimer) -> BOOL {
    (*t).state.lock().unwrap().2 as BOOL
}

pub unsafe extern "win64" fn WaitForThreadpoolTimerCallbacks(t: *mut TpTimer, cancel: BOOL) {
    if cancel != 0 {
        let mut st = (*t).state.lock().unwrap();
        st.0 += 1;
        st.2 = false;
    }
    (*t).pending.wait_zero();
}

pub unsafe extern "win64" fn CloseThreadpoolTimer(t: *mut TpTimer) {
    let mut st = (*t).state.lock().unwrap();
    st.0 += 1;
    st.2 = false;
}

// ---------------------------------------------------------------------------
// Waits
// ---------------------------------------------------------------------------

pub struct TpWait {
    cb: WaitCb,
    ctx: SendPtr,
    pending: Pending,
    generation: Mutex<u64>,
}

pub unsafe extern "win64" fn CreateThreadpoolWait(cb: WaitCb, ctx: *mut c_void, _env: *mut c_void) -> *mut TpWait {
    Box::into_raw(Box::new(TpWait { cb, ctx: SendPtr(ctx), pending: Pending::new(), generation: Mutex::new(0) }))
}

pub unsafe extern "win64" fn SetThreadpoolWait(w: *mut TpWait, h: HANDLE, timeout: *const FILETIME) {
    let generation = {
        let mut g = (*w).generation.lock().unwrap();
        *g += 1;
        *g
    };
    if h.is_null() {
        return;
    }
    let Some(obj) = handles::get(h) else {
        log::warn!("SetThreadpoolWait: invalid handle {h:p}");
        return;
    };
    let ms = if timeout.is_null() {
        INFINITE
    } else {
        let v = (*timeout).to_u64() as i64;
        if v < 0 {
            ((-v) / 10_000) as u32
        } else {
            ((v - now_filetime() as i64).max(0) / 10_000) as u32
        }
    };
    let wp = SendPtr(w as *mut c_void);
    (*w).pending.inc();
    let body: Box<dyn FnOnce() -> u32 + Send> = Box::new(move || {
        let wp = wp;
        let w = &*(wp.0 as *const TpWait);
        let r = handles::wait_for(&[obj], false, ms, false);
        if *w.generation.lock().unwrap() == generation {
            let res = match r {
                handles::WaitResult::Object(_) => WAIT_OBJECT_0,
                handles::WaitResult::Abandoned(_) => WAIT_ABANDONED_0,
                handles::WaitResult::Timeout => WAIT_TIMEOUT,
                _ => WAIT_FAILED,
            };
            crate::call_guest8(w.cb as usize, 0, w.ctx.0 as u64, wp.0 as u64, res as u64, 0, 0, 0, 0);
        }
        w.pending.dec();
        0
    });
    if let Err(e) = thread::spawn(body, 256 * 1024, false, false) {
        log::error!("SetThreadpoolWait: spawn failed: {e}");
        (*w).pending.dec();
    }
}

pub unsafe extern "win64" fn WaitForThreadpoolWaitCallbacks(w: *mut TpWait, cancel: BOOL) {
    if cancel != 0 {
        *(*w).generation.lock().unwrap() += 1;
        // Armed waits keep blocking on their handle; don't wait for them.
        return;
    }
    (*w).pending.wait_zero();
}

pub unsafe extern "win64" fn CloseThreadpoolWait(w: *mut TpWait) {
    *(*w).generation.lock().unwrap() += 1;
}

// ---------------------------------------------------------------------------
// Callback-instance helpers
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn SetEventWhenCallbackReturns(_inst: *mut c_void, ev: HANDLE) {
    // Callbacks run synchronously from the worker's point of view: signal now.
    crate::kernel32::threads::SetEvent(ev);
}
pub unsafe extern "win64" fn CallbackMayRunLong(_inst: *mut c_void) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn DisassociateCurrentThreadFromCallback(_inst: *mut c_void) {}

pub unsafe extern "win64" fn CreateThreadpool(_r: *mut c_void) -> *mut c_void {
    // A distinct non-null token; all work goes to the shared pool.
    Box::into_raw(Box::new(0u64)) as *mut c_void
}
pub unsafe extern "win64" fn CloseThreadpool(_p: *mut c_void) {}
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
pub unsafe extern "win64" fn CloseThreadpoolCleanupGroup(_g: *mut c_void) {}

// ---------------------------------------------------------------------------
// Legacy timer queues & RegisterWaitForSingleObject
// ---------------------------------------------------------------------------

type WaitOrTimerCb = unsafe extern "win64" fn(*mut c_void, u8);

struct LegacyTimerCtx {
    cb: WaitOrTimerCb,
    param: SendPtr,
    tp_timer: *mut TpTimer,
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
        *out_timer = std::ptr::null_mut();
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let ctx = Box::into_raw(Box::new(LegacyTimerCtx {
        cb,
        param: SendPtr(parameter),
        tp_timer: std::ptr::null_mut(),
    }));
    let tp = CreateThreadpoolTimer(legacy_timer_trampoline, ctx as *mut c_void, std::ptr::null_mut());
    (*ctx).tp_timer = tp;
    let due_100ns = -((due_time as i64) * 10_000);
    let ft = FILETIME::from_u64(due_100ns as u64);
    SetThreadpoolTimer(tp, &ft, period, 0);
    *out_timer = ctx as HANDLE;
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
    let due_100ns = -((due_time as i64) * 10_000);
    let ft = FILETIME::from_u64(due_100ns as u64);
    SetThreadpoolTimer(ctx.tp_timer, &ft, period, 0);
    TRUE
}

pub unsafe extern "win64" fn DeleteTimerQueueTimer(
    _timer_queue: HANDLE,
    timer: HANDLE,
    completion_event: HANDLE,
) -> BOOL {
    if !timer.is_null() {
        let ctx = &*(timer as *const LegacyTimerCtx);
        SetThreadpoolTimer(ctx.tp_timer, std::ptr::null(), 0, 0);
        if completion_event as usize == usize::MAX {
            WaitForThreadpoolTimerCallbacks(ctx.tp_timer, TRUE);
        }
        CloseThreadpoolTimer(ctx.tp_timer);
    }
    if !completion_event.is_null() && completion_event as usize != usize::MAX {
        crate::kernel32::threads::SetEvent(completion_event);
    }
    TRUE
}

pub unsafe extern "win64" fn DeleteTimerQueueEx(timer_queue: HANDLE, completion_event: HANDLE) -> BOOL {
    let _ = timer_queue;
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
    tp_wait: *mut TpWait,
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
        *out_new_wait_object = std::ptr::null_mut();
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let ctx = Box::into_raw(Box::new(LegacyWaitCtx {
        cb,
        param: SendPtr(context),
        tp_wait: std::ptr::null_mut(),
    }));
    let tp = CreateThreadpoolWait(legacy_wait_trampoline, ctx as *mut c_void, std::ptr::null_mut());
    (*ctx).tp_wait = tp;
    let ft;
    let p_ft = if milliseconds == INFINITE {
        std::ptr::null()
    } else {
        let rel_100ns = -((milliseconds as i64) * 10_000);
        ft = FILETIME::from_u64(rel_100ns as u64);
        &ft as *const FILETIME
    };
    SetThreadpoolWait(tp, object, p_ft);
    *out_new_wait_object = ctx as HANDLE;
    TRUE
}

pub unsafe extern "win64" fn UnregisterWaitEx(wait_handle: HANDLE, completion_event: HANDLE) -> BOOL {
    if !wait_handle.is_null() {
        let ctx = &*(wait_handle as *const LegacyWaitCtx);
        SetThreadpoolWait(ctx.tp_wait, std::ptr::null_mut(), std::ptr::null());
        CloseThreadpoolWait(ctx.tp_wait);
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
}
