//! Windows threads: TEB setup, static TLS via the loader, thread objects,
//! APC queue, FLS and TLS slots.

use crate::handles::{self, Guarded, KObject, WaitGuard, Waitable};
use crate::teb;
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex, MutexGuard, Once, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

const TERM_NONE: u8 = 0;
const TERM_REQUESTED: u8 = 1;
const TERM_ACKED: u8 = 2;

pub type ThreadProc = unsafe extern "win64" fn(*mut c_void) -> u32;
pub type ApcProc = unsafe extern "win64" fn(usize);
pub type IoCompletionRoutine = unsafe extern "win64" fn(u32, u32, usize);

#[derive(Clone, Copy)]
pub enum ApcItem {
    User {
        func: ApcProc,
        param: usize,
    },
    IoCompletion {
        func: usize,
        err_code: u32,
        bytes: u32,
        overlapped: usize,
        flags: u32,
    },
}

/// Windows maximum thread suspend count (`MAXIMUM_SUSPEND_COUNT`).
pub const MAXIMUM_SUSPEND_COUNT: u32 = 127;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThreadPhase {
    /// Thread has been created but has not yet cleared `wait_resumed()`.
    Starting,
    /// Thread cleared `wait_resumed()` and is executing with a valid Mach port.
    Live,
    /// Thread entered teardown (`finish_current_thread`); never Mach-suspend again.
    Exiting,
}

struct SuspendState {
    count: u32,
    phase: ThreadPhase,
    /// True while the thread itself is parked inside `suspend()` on `resume_cv`.
    self_suspended: bool,
    /// Darwin Mach thread port (`pthread_mach_thread_np`), non-zero only while `Live`.
    mach_port: u32,
    /// True iff we currently hold an active Mach `thread_suspend` (depth 1) on `mach_port`.
    kernel_suspended: bool,
}

pub struct ThreadObj {
    pub tid: AtomicUsize,
    pub teb: AtomicUsize,
    pub exit_code: Guarded<Option<u32>>,
    pub apcs: Mutex<Vec<ApcItem>>,
    /// Lock-free mirror of `!apcs.is_empty()` for zero-overhead `has_pending_apc()`.
    pub has_apc: AtomicBool,
    pub name: Mutex<String>,
    suspend: Mutex<SuspendState>,
    resume_cv: Condvar,
    pub pthread: AtomicUsize,
    pub priority: std::sync::atomic::AtomicI32,
    /// Per-thread Windows affinity mask (`0` = inherit process affinity mask).
    pub affinity_mask: AtomicUsize,
    /// Per-thread Windows processor group (`0` on single-group systems).
    pub affinity_group: AtomicU16,
    /// Guest Win32 start address recorded by `CreateThread` (`ThreadQuerySetWin32StartAddress`).
    pub start_address: AtomicUsize,
    /// Creation timestamp in 100ns FILETIME ticks (`ThreadTimes`).
    pub creation_time: AtomicU64,
    /// Exit timestamp in 100ns FILETIME ticks (`0` while live; captured on thread teardown).
    pub exit_time: AtomicU64,
    /// Terminal kernel CPU time in 100ns ticks, snapshotted before clearing the Mach port.
    pub final_kernel_time: AtomicU64,
    /// Terminal user CPU time in 100ns ticks, snapshotted before clearing the Mach port.
    pub final_user_time: AtomicU64,
    /// Set by `NtSetInformationThread(ThreadHideFromDebugger)`.
    pub hide_from_debugger: AtomicBool,
    /// Asynchronous termination state (`TERM_NONE`, `TERM_REQUESTED`, `TERM_ACKED`).
    terminate_state: AtomicU8,
    /// Per-thread nesting depth of runtime critical sections where `SIGUSR1` must defer
    /// calling `pthread_exit` until all internal locks have been released. Mutated only
    /// by the owning thread and synchronized with `sigusr1_terminate_handler` via
    /// `compiler_fence(Ordering::SeqCst)`.
    defer_terminate: AtomicU32,
    /// Custom mmap stack allocation `(base_ptr, total_len)` owned by a `spawn`ed thread.
    /// Taken once by whichever exit path reaps it first (`finish_current_thread` or `terminate_thread`).
    stack_alloc: Mutex<Option<(usize, usize)>>,
    /// Current activity kind (`hitch::ActKind as u8`; 0 = idle). Lock-free for watchdog inspection.
    pub act_kind: AtomicU8,
    /// Monotonic timestamp (ns) when current activity began.
    pub act_start_ns: AtomicU64,
    /// Optional activity argument (e.g. fence value, lock address, byte count).
    pub act_arg: AtomicU64,
}

impl ThreadObj {
    fn new(suspended: bool) -> Arc<Self> {
        Arc::new(ThreadObj {
            tid: AtomicUsize::new(0),
            teb: AtomicUsize::new(0),
            exit_code: Guarded::new(None),
            apcs: Mutex::new(Vec::new()),
            has_apc: AtomicBool::new(false),
            name: Mutex::new(String::new()),
            suspend: Mutex::new(SuspendState {
                count: suspended as u32,
                phase: ThreadPhase::Starting,
                self_suspended: false,
                mach_port: 0,
                kernel_suspended: false,
            }),
            resume_cv: Condvar::new(),
            pthread: AtomicUsize::new(0),
            priority: std::sync::atomic::AtomicI32::new(0),
            affinity_mask: AtomicUsize::new(0),
            affinity_group: AtomicU16::new(0),
            start_address: AtomicUsize::new(0),
            creation_time: AtomicU64::new(crate::types::now_filetime()),
            exit_time: AtomicU64::new(0),
            final_kernel_time: AtomicU64::new(0),
            final_user_time: AtomicU64::new(0),
            hide_from_debugger: AtomicBool::new(false),
            terminate_state: AtomicU8::new(TERM_NONE),
            defer_terminate: AtomicU32::new(0),
            stack_alloc: Mutex::new(None),
            act_kind: AtomicU8::new(0),
            act_start_ns: AtomicU64::new(0),
            act_arg: AtomicU64::new(0),
        })
    }

    pub fn exit_code(&self) -> Option<u32> {
        let g = handles::wait_lock();
        *self.exit_code.get(&g)
    }

    /// Return this thread's live Darwin Mach port (`0` if not yet started or already exiting).
    pub fn mach_port(&self) -> u32 {
        let s = defer_lock(&self.suspend);
        if s.phase == ThreadPhase::Live {
            s.mach_port
        } else {
            0
        }
    }

    /// True iff this thread is currently held in a Mach `thread_suspend` (e.g. via `SuspendThread`).
    pub fn is_kernel_suspended(&self) -> bool {
        self.suspend.lock().unwrap_or_else(|e| e.into_inner()).kernel_suspended
    }

    fn is_current_thread(&self, mach_port: u32) -> bool {
        if current_thread_object().is_some_and(|c| std::ptr::eq(&*c, self)) {
            return true;
        }
        #[cfg(target_vendor = "apple")]
        if mach_port != 0 && mach_port == sched::current_port() {
            return true;
        }
        #[cfg(not(target_vendor = "apple"))]
        let _ = mach_port;
        let p = self.pthread.load(Ordering::Relaxed);
        p != 0 && p == (unsafe { libc::pthread_self() } as usize)
    }

    /// Suspends this thread, returning the previous suspend count or a Win32 error code.
    ///
    /// - Before the thread clears its `CREATE_SUSPENDED` start gate (`Starting`), only
    ///   the userspace suspend count is updated; the thread stays blocked on `resume_cv`.
    /// - Once `Live`, transitioning `0 -> 1` issues a single Mach `thread_suspend(mach_port)`
    ///   under `self.suspend`; nested suspends only increment `count`.
    /// - Self-suspension never calls Mach `thread_suspend` (which would deadlock while holding
    ///   `self.suspend`); it parks on `resume_cv`, releasing the mutex so another thread can
    ///   resume it cleanly.
    pub fn suspend(&self) -> Result<u32, u32> {
        let _defer = TerminateDeferGuard::enter();
        let mut s = self.suspend.lock().unwrap();
        if s.phase == ThreadPhase::Exiting {
            return Err(crate::types::err::ERROR_ACCESS_DENIED);
        }
        if s.count >= MAXIMUM_SUSPEND_COUNT {
            // STATUS_SUSPEND_COUNT_EXCEEDED -> ERROR_SIGNAL_REFUSED (156)
            return Err(156);
        }

        if self.is_current_thread(s.mach_port) {
            let prev = s.count;
            s.count += 1;
            s.self_suspended = true;
            while s.count > 0
                && s.phase != ThreadPhase::Exiting
                && self.terminate_state.load(Ordering::Acquire) == TERM_NONE
            {
                s = self.resume_cv.wait(s).unwrap();
            }
            s.self_suspended = false;
            return Ok(prev);
        }

        let prev = s.count;
        match s.phase {
            ThreadPhase::Starting => {
                s.count += 1;
                Ok(prev)
            }
            ThreadPhase::Live => {
                if prev == 0 && !s.self_suspended {
                    #[cfg(target_vendor = "apple")]
                    {
                        let port = s.mach_port;
                        if port == 0 {
                            return Err(crate::types::err::ERROR_ACCESS_DENIED);
                        }
                        let kr = sched::thread_suspend_port(port);
                        if kr != 0 {
                            return Err(crate::types::err::ERROR_ACCESS_DENIED);
                        }
                        s.kernel_suspended = true;
                    }
                }
                s.count += 1;
                Ok(prev)
            }
            ThreadPhase::Exiting => Err(crate::types::err::ERROR_ACCESS_DENIED),
        }
    }

    /// Decrements the suspend count and resumes execution when transitioning `1 -> 0`.
    /// Returns the previous suspend count, or a Win32 error code if Mach resume fails.
    pub fn resume_checked(&self) -> Result<u32, u32> {
        let mut s = defer_lock(&self.suspend);
        let prev = s.count;
        if prev == 0 {
            return Ok(0);
        }
        if prev == 1 && s.kernel_suspended {
            #[cfg(target_vendor = "apple")]
            {
                let port = s.mach_port;
                if port != 0 {
                    let kr = sched::thread_resume_port(port);
                    if kr != 0 {
                        s.kernel_suspended = false;
                        s.count = 0;
                        return Err(crate::types::err::ERROR_ACCESS_DENIED);
                    }
                }
            }
            s.kernel_suspended = false;
        }
        s.count -= 1;
        if s.count == 0 {
            self.resume_cv.notify_all();
        }
        Ok(prev)
    }

    /// Returns previous suspend count (or `u32::MAX` on error).
    pub fn resume(&self) -> u32 {
        self.resume_checked().unwrap_or(u32::MAX)
    }

    fn snapshot_times_from_port(&self, port: u32) {
        if let Some((kernel, user)) = sched::thread_cpu_times_100ns(port) {
            self.final_kernel_time.fetch_max(kernel, Ordering::AcqRel);
            self.final_user_time.fetch_max(user, Ordering::AcqRel);
        }
    }

    /// Return `(creation_time, exit_time, kernel_time, user_time)` in 100ns FILETIME ticks.
    /// Live threads query Mach `THREAD_BASIC_INFO` under the suspend lock; exited threads
    /// return the terminal snapshot captured before clearing the Mach port.
    pub fn times_100ns(&self) -> (u64, u64, u64, u64) {
        let created = self.creation_time.load(Ordering::Relaxed);
        let s = defer_lock(&self.suspend);
        match s.phase {
            ThreadPhase::Exiting => (
                created,
                self.exit_time.load(Ordering::Acquire),
                self.final_kernel_time.load(Ordering::Acquire),
                self.final_user_time.load(Ordering::Acquire),
            ),
            ThreadPhase::Live => {
                let port = if s.mach_port != 0 {
                    s.mach_port
                } else if self.is_current_thread(0) {
                    sched::current_port()
                } else {
                    0
                };
                let (k, u) = sched::thread_cpu_times_100ns(port).unwrap_or((
                    self.final_kernel_time.load(Ordering::Acquire),
                    self.final_user_time.load(Ordering::Acquire),
                ));
                (created, 0, k, u)
            }
            ThreadPhase::Starting => (created, 0, 0, 0),
        }
    }

    fn wait_resumed(&self) -> bool {
        let _defer = TerminateDeferGuard::enter();
        let mut s = self.suspend.lock().unwrap();
        while s.count > 0 && self.terminate_state.load(Ordering::Acquire) == TERM_NONE {
            s = self.resume_cv.wait(s).unwrap();
        }
        if self.terminate_state.load(Ordering::Acquire) != TERM_NONE {
            let _ = self.exit_time.compare_exchange(
                0,
                crate::types::now_filetime(),
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            s.phase = ThreadPhase::Exiting;
            // Clear ACT_SLOT before `_defer` drops so `thread_main` runs its normal
            // `CREATE_SUSPENDED` teardown (freeing TEB + reaping stack) instead of
            // short-circuiting through `pthread_exit`.
            ACT_SLOT.with(|c| c.set(std::ptr::null()));
            return false;
        }
        s.phase = ThreadPhase::Live;
        #[cfg(target_vendor = "apple")]
        {
            s.mach_port = sched::current_port();
        }
        true
    }

    fn mark_exiting(&self) {
        let mut s = defer_lock(&self.suspend);
        if s.phase == ThreadPhase::Exiting {
            return;
        }
        if s.mach_port != 0 {
            self.snapshot_times_from_port(s.mach_port);
        }
        let _ = self.exit_time.compare_exchange(
            0,
            crate::types::now_filetime(),
            Ordering::AcqRel,
            Ordering::Acquire,
        );
        #[cfg(target_vendor = "apple")]
        if s.kernel_suspended && s.mach_port != 0 {
            let _ = sched::thread_resume_port(s.mach_port);
            s.kernel_suspended = false;
        }
        s.phase = ThreadPhase::Exiting;
        s.mach_port = 0;
        s.count = 0;
        self.resume_cv.notify_all();
    }
}

impl Drop for ThreadObj {
    fn drop(&mut self) {
        let s = self.suspend.get_mut().unwrap_or_else(|e| e.into_inner());
        #[cfg(target_vendor = "apple")]
        if s.kernel_suspended && s.mach_port != 0 {
            let _ = sched::thread_resume_port(s.mach_port);
            s.kernel_suspended = false;
        }
    }
}

impl KObject for ThreadObj {
    fn type_name(&self) -> &'static str {
        "Thread"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn Waitable> {
        Some(self)
    }
}

impl Waitable for ThreadObj {
    fn is_signaled(&self, g: &WaitGuard, _tid: u32) -> bool {
        self.exit_code.get(g).is_some()
    }
    fn acquire(&self, _g: &WaitGuard, _tid: u32) -> bool {
        false
    }
}

thread_local! {
    static CURRENT: RefCell<Option<Arc<ThreadObj>>> = const { RefCell::new(None) };
    static ACT_SLOT: Cell<*const ThreadObj> = const { Cell::new(std::ptr::null()) };
    static FLS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

static THREADS: Mutex<Vec<Arc<ThreadObj>>> = Mutex::new(Vec::new());

/// Take the current thread's `ThreadObj` context (for cooperative fiber handoff).
pub(crate) fn take_current_thread_context() -> Option<Arc<ThreadObj>> {
    ACT_SLOT.with(|c| c.set(std::ptr::null()));
    CURRENT.with(|c| c.borrow_mut().take())
}

/// Install a `ThreadObj` context on the current OS thread (for cooperative fiber handoff).
pub(crate) fn install_current_thread_context(obj: Option<Arc<ThreadObj>>) {
    let ptr = obj.as_ref().map_or(std::ptr::null(), |o| &**o as *const ThreadObj);
    ACT_SLOT.with(|c| c.set(ptr));
    CURRENT.with(|c| *c.borrow_mut() = obj);
}

struct ReapReq {
    pthread: usize,
    mem: usize,
    len: usize,
}

fn stack_reaper_tx() -> &'static Mutex<mpsc::Sender<ReapReq>> {
    static TX: OnceLock<Mutex<mpsc::Sender<ReapReq>>> = OnceLock::new();
    TX.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<ReapReq>();
        let _ = std::thread::Builder::new()
            .name("win32-stack-reaper".into())
            .spawn(move || {
                while let Ok(req) = rx.recv() {
                    unsafe {
                        // `pthread_join` guarantees the thread has completely exited
                        // and is no longer executing on its custom mmap stack before munmap.
                        let rc = libc::pthread_join(req.pthread as libc::pthread_t, std::ptr::null_mut());
                        if rc == 0 {
                            libc::munmap(req.mem as *mut c_void, req.len);
                        } else {
                            log::warn!(
                                "stack reaper: pthread_join({:#x}) failed (rc={rc}); skipping munmap",
                                req.pthread
                            );
                        }
                    }
                }
            });
        Mutex::new(tx)
    })
}

fn enqueue_stack_reap_for(obj: &ThreadObj, pthread: usize) -> bool {
    if pthread == 0 {
        return false;
    }
    if let Some((mem, len)) = defer_lock(&obj.stack_alloc).take() {
        let _ = defer_lock(stack_reaper_tx()).send(ReapReq { pthread, mem, len });
        return true;
    }
    false
}

#[inline(always)]
unsafe fn try_exit_if_terminated(obj: *const ThreadObj) {
    if (*obj).terminate_state.load(Ordering::Acquire) == TERM_REQUESTED
        && (*obj)
            .terminate_state
            .compare_exchange(TERM_REQUESTED, TERM_ACKED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    {
        libc::pthread_exit(std::ptr::null_mut());
    }
}

/// Returns `true` if `TerminateThread` has requested termination of the current thread.
#[inline(always)]
pub(crate) fn is_current_terminate_requested() -> bool {
    let p = ACT_SLOT.with(|c| c.get());
    !p.is_null() && unsafe { (*p).terminate_state.load(Ordering::Acquire) != TERM_NONE }
}

/// Increment the current thread's termination-deferral nesting depth.
/// Must be paired 1-for-1 with [`defer_terminate_leave`].
#[inline(always)]
pub(crate) fn defer_terminate_enter() {
    let p = ACT_SLOT.with(|c| c.get());
    if !p.is_null() {
        unsafe {
            let d = (*p).defer_terminate.load(Ordering::Relaxed);
            (*p).defer_terminate.store(d.wrapping_add(1), Ordering::Relaxed);
        }
        std::sync::atomic::compiler_fence(Ordering::SeqCst);
    }
}

/// Decrement the current thread's termination-deferral nesting depth, exiting cleanly
/// at the outermost safe point if `TerminateThread` requested termination while deferred.
#[inline(always)]
pub(crate) fn defer_terminate_leave() {
    let p = ACT_SLOT.with(|c| c.get());
    if !p.is_null() {
        std::sync::atomic::compiler_fence(Ordering::SeqCst);
        unsafe {
            let prev = (*p).defer_terminate.load(Ordering::Relaxed);
            let next = prev.saturating_sub(1);
            (*p).defer_terminate.store(next, Ordering::Relaxed);
            std::sync::atomic::compiler_fence(Ordering::SeqCst);
            if next == 0 {
                try_exit_if_terminated(p);
            }
        }
    }
}

/// RAII guard that defers asynchronous `SIGUSR1` termination while a runtime lock
/// or critical section is held. `!Send` by virtue of holding `*const ThreadObj`.
pub(crate) struct TerminateDeferGuard {
    obj: *const ThreadObj,
}

impl TerminateDeferGuard {
    #[inline(always)]
    pub(crate) fn enter() -> Self {
        let p = ACT_SLOT.with(|c| c.get());
        if !p.is_null() {
            unsafe {
                let d = (*p).defer_terminate.load(Ordering::Relaxed);
                (*p).defer_terminate.store(d.wrapping_add(1), Ordering::Relaxed);
            }
            std::sync::atomic::compiler_fence(Ordering::SeqCst);
        }
        TerminateDeferGuard { obj: p }
    }
}

impl Drop for TerminateDeferGuard {
    #[inline(always)]
    fn drop(&mut self) {
        let p = self.obj;
        if !p.is_null() {
            std::sync::atomic::compiler_fence(Ordering::SeqCst);
            unsafe {
                let prev = (*p).defer_terminate.load(Ordering::Relaxed);
                let next = prev.saturating_sub(1);
                (*p).defer_terminate.store(next, Ordering::Relaxed);
                std::sync::atomic::compiler_fence(Ordering::SeqCst);
                if next == 0 && !ACT_SLOT.with(|c| c.get()).is_null() {
                    try_exit_if_terminated(p);
                }
            }
        }
    }
}

/// Wraps a lock guard so `TerminateDeferGuard` is entered BEFORE acquiring the lock
/// and dropped AFTER the lock guard unlocks (struct fields drop in declaration order).
pub(crate) struct DeferLockGuard<G> {
    guard: G,
    _defer: TerminateDeferGuard,
}

impl<G: Deref> Deref for DeferLockGuard<G> {
    type Target = G::Target;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<G: DerefMut> DerefMut for DeferLockGuard<G> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

#[inline]
pub(crate) fn defer_lock<T>(m: &Mutex<T>) -> DeferLockGuard<MutexGuard<'_, T>> {
    let _defer = TerminateDeferGuard::enter();
    let guard = m.lock().unwrap_or_else(|e| e.into_inner());
    DeferLockGuard { guard, _defer }
}

#[inline]
pub(crate) fn defer_read<T>(rw: &RwLock<T>) -> DeferLockGuard<RwLockReadGuard<'_, T>> {
    let _defer = TerminateDeferGuard::enter();
    let guard = rw.read().unwrap_or_else(|e| e.into_inner());
    DeferLockGuard { guard, _defer }
}

#[inline]
pub(crate) fn defer_write<T>(rw: &RwLock<T>) -> DeferLockGuard<RwLockWriteGuard<'_, T>> {
    let _defer = TerminateDeferGuard::enter();
    let guard = rw.write().unwrap_or_else(|e| e.into_inner());
    DeferLockGuard { guard, _defer }
}

extern "C" fn sigusr1_terminate_handler(_sig: libc::c_int) {
    // Async-signal-safe: reads only thread-local Cell pointer and atomics, zero mutexes.
    // While inside a runtime critical section (`defer_terminate > 0`), return without
    // calling `pthread_exit` (interrupting blocking syscalls with EINTR) so the thread
    // exits cleanly once all internal locks have been released.
    let p = ACT_SLOT.with(|c| c.get());
    if !p.is_null() {
        unsafe {
            if (*p).defer_terminate.load(Ordering::Relaxed) == 0 {
                try_exit_if_terminated(p);
            }
        }
    }
}

fn install_terminate_signal_handler() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe {
        let mut sa: libc::sigaction = std::mem::zeroed();
        sa.sa_sigaction = sigusr1_terminate_handler as *const () as usize;
        sa.sa_flags = 0; // Do not set SA_RESTART so blocking syscalls are interrupted immediately.
        libc::sigemptyset(&mut sa.sa_mask);
        libc::sigaction(libc::SIGUSR1, &sa, std::ptr::null_mut());
    });
}

/// Terminate `obj` with `code`, matching Windows `TerminateThread` semantics:
/// - Already-exited threads succeed without altering their exit code.
/// - Self-termination records `code`, signals waiters, skips `DLL_THREAD_DETACH`, and exits.
/// - `CREATE_SUSPENDED` threads abort before running their guest entry point.
/// - Live other threads are interrupted via `SIGUSR1` -> `pthread_exit` (deferred cleanly
///   past any held runtime locks), signaled with `code`, and their custom mmap stack is
///   safely reclaimed after `pthread_join`.
pub fn terminate_thread(obj: &Arc<ThreadObj>, code: u32) -> bool {
    if obj.exit_code().is_some() {
        return true;
    }
    if obj.is_current_thread(obj.mach_port()) {
        ACT_SLOT.with(|c| c.set(std::ptr::null()));
        obj.mark_exiting();
        let t = teb::current_raw();
        let pthread = unsafe { libc::pthread_self() as usize };
        obj.pthread.store(0, Ordering::Release);
        {
            let g = handles::wait_lock();
            let slot = obj.exit_code.get(&g);
            if slot.is_none() {
                *slot = Some(code);
            }
        }
        handles::notify_object_key(&**obj as *const ThreadObj as usize, usize::MAX);
        obj.teb.store(0, Ordering::Relaxed);
        defer_lock(&THREADS).retain(|o| !Arc::ptr_eq(o, obj));
        CURRENT.with(|c| *c.borrow_mut() = None);
        unsafe {
            teb::uninstall();
            if !t.is_null() {
                teb::free_teb(t);
            }
        }
        enqueue_stack_reap_for(obj, pthread);
        unsafe { libc::pthread_exit(std::ptr::null_mut()) }
    }

    install_terminate_signal_handler();
    let (was_starting, pthread) = {
        let mut s = defer_lock(&obj.suspend);
        if s.phase == ThreadPhase::Exiting {
            return true;
        }
        let _ = obj.terminate_state.compare_exchange(
            TERM_NONE,
            TERM_REQUESTED,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
        if s.phase == ThreadPhase::Starting {
            let _ = obj.exit_time.compare_exchange(
                0,
                crate::types::now_filetime(),
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            s.count = 0;
            s.phase = ThreadPhase::Exiting;
            obj.resume_cv.notify_all();
            (true, 0usize)
        } else {
            if s.mach_port != 0 {
                obj.snapshot_times_from_port(s.mach_port);
            }
            #[cfg(target_vendor = "apple")]
            if s.kernel_suspended && s.mach_port != 0 {
                let _ = sched::thread_resume_port(s.mach_port);
                s.kernel_suspended = false;
            }
            s.count = 0;
            s.self_suspended = false;
            obj.resume_cv.notify_all();
            (false, obj.pthread.load(Ordering::Acquire))
        }
    };

    if !was_starting {
        if pthread == 0 {
            if obj.exit_code().is_none()
                && obj
                    .terminate_state
                    .compare_exchange(TERM_REQUESTED, TERM_NONE, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
            {
                return false;
            }
        } else {
            let kill_rc = unsafe { libc::pthread_kill(pthread as libc::pthread_t, libc::SIGUSR1) };
            let mut confirmed = if kill_rc == libc::ESRCH {
                true
            } else if kill_rc == 0 {
                let deadline = std::time::Instant::now() + std::time::Duration::from_millis(200);
                let mut last_signal = std::time::Instant::now();
                while obj.terminate_state.load(Ordering::Acquire) != TERM_ACKED
                    && obj.exit_code().is_none()
                    && std::time::Instant::now() < deadline
                {
                    std::thread::sleep(std::time::Duration::from_micros(200));
                    if last_signal.elapsed() >= std::time::Duration::from_millis(2) {
                        let rc = unsafe { libc::pthread_kill(pthread as libc::pthread_t, libc::SIGUSR1) };
                        if rc == libc::ESRCH {
                            break;
                        }
                        last_signal = std::time::Instant::now();
                    }
                }
                obj.terminate_state.load(Ordering::Acquire) == TERM_ACKED
                    || obj.exit_code().is_some()
                    || unsafe { libc::pthread_kill(pthread as libc::pthread_t, 0) } == libc::ESRCH
            } else {
                false
            };
            if !confirmed {
                // Atomically cancel only if the target did not just commit to exiting.
                if obj
                    .terminate_state
                    .compare_exchange(TERM_REQUESTED, TERM_NONE, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    // Do not publish an exit code or signal waiters when the target thread
                    // has not confirmed termination (e.g. SIGUSR1 blocked or signal delivery
                    // failed) — otherwise callers waiting on the handle could free resources
                    // still in use by the running thread.
                    log::warn!(
                        "TerminateThread: target thread did not confirm termination (pthread_kill rc={kill_rc})"
                    );
                    return false;
                }
                confirmed = true;
            }
            let _ = confirmed;
        }
    }

    obj.mark_exiting();
    {
        let g = handles::wait_lock();
        let slot = obj.exit_code.get(&g);
        if slot.is_none() {
            *slot = Some(code);
        }
    }
    handles::notify_object_key(&**obj as *const ThreadObj as usize, usize::MAX);
    defer_lock(&THREADS).retain(|o| !Arc::ptr_eq(o, obj));
    if !was_starting && obj.terminate_state.load(Ordering::Acquire) == TERM_ACKED {
        let p = obj.pthread.swap(0, Ordering::AcqRel);
        enqueue_stack_reap_for(obj, p);
    }
    true
}

/// Fast, allocation-free entry into a tracked thread activity. Returns previous `(kind, start_ns, arg)`.
#[inline(always)]
pub fn enter_activity(kind: u8, start_ns: u64, arg: u64) -> (u8, u64, u64) {
    let p = ACT_SLOT.with(|c| c.get());
    if p.is_null() {
        return (0, 0, 0);
    }
    unsafe {
        let t = &*p;
        let pk = t.act_kind.load(Ordering::Relaxed);
        let ps = t.act_start_ns.load(Ordering::Relaxed);
        let pa = t.act_arg.load(Ordering::Relaxed);
        t.act_start_ns.store(start_ns, Ordering::Relaxed);
        t.act_arg.store(arg, Ordering::Relaxed);
        t.act_kind.store(kind, Ordering::Release);
        (pk, ps, pa)
    }
}

/// Fast, allocation-free exit from a tracked thread activity.
#[inline(always)]
pub fn leave_activity(prev: (u8, u64, u64)) {
    let p = ACT_SLOT.with(|c| c.get());
    if p.is_null() {
        return;
    }
    unsafe {
        let t = &*p;
        t.act_start_ns.store(prev.1, Ordering::Relaxed);
        t.act_arg.store(prev.2, Ordering::Relaxed);
        t.act_kind.store(prev.0, Ordering::Release);
    }
}

pub fn current_thread_object() -> Option<Arc<ThreadObj>> {
    CURRENT.with(|c| c.borrow().clone())
}

/// Return the current thread's `ThreadObj`, lazily attaching a lightweight
/// `ThreadObj` for host/test threads that did not enter via [`spawn`].
/// This guarantees issuing-thread APC queues (`ReadFileEx`, `WriteFileEx`,
/// `ReadDirectoryChangesW`) work on every thread, including the initial thread.
pub fn ensure_current_thread_object() -> Arc<ThreadObj> {
    if let Some(obj) = current_thread_object() {
        return obj;
    }
    let obj = ThreadObj::new(false);
    let teb_ptr = teb::current();
    let tid = teb::thread_id();
    obj.tid.store(tid as usize, Ordering::Relaxed);
    obj.teb.store(teb_ptr as usize, Ordering::Relaxed);
    obj.pthread.store(unsafe { libc::pthread_self() as usize }, Ordering::Relaxed);
    {
        let mut s = defer_lock(&obj.suspend);
        s.phase = ThreadPhase::Live;
        #[cfg(target_vendor = "apple")]
        {
            s.mach_port = sched::current_port();
        }
    }
    install_current_thread_context(Some(obj.clone()));
    obj
}

pub fn find_by_tid(tid: u32) -> Option<Arc<ThreadObj>> {
    defer_lock(&THREADS).iter().find(|t| t.tid.load(Ordering::Relaxed) as u32 == tid).cloned()
}

/// Snapshot of all live Windows thread objects.
pub fn all_threads() -> Vec<Arc<ThreadObj>> {
    defer_lock(&THREADS).clone()
}

/// All live TEBs (the loader extends static TLS of every thread when a DLL
/// with a TLS directory is loaded at runtime).
pub fn all_tebs() -> Vec<*mut u8> {
    defer_lock(&THREADS)
        .iter()
        .map(|t| t.teb.load(Ordering::Relaxed) as *mut u8)
        .filter(|t| !t.is_null())
        .collect()
}

/// Default stack size for new threads (from the exe's SizeOfStackReserve).
pub static DEFAULT_STACK_SIZE: AtomicUsize = AtomicUsize::new(1 << 20);

/// Per-thread host setup hook (the loader installs its sigaltstack here).
pub static THREAD_START_HOOK: OnceLock<fn()> = OnceLock::new();

struct StartInfo {
    obj: Arc<ThreadObj>,
    body: Box<dyn FnOnce() -> u32 + Send>,
    stack_lo: usize,
    stack_hi: usize,
    is_initial: bool,
}

/// Set in forked difftest children (single-threaded after `fork`): [`spawn`] fails with
/// `EAGAIN` and starts nothing, so `CreateThread` / `_beginthreadex` return failure
/// identically for the guest and lifted runs and no thread outlives the run that made it.
pub static NO_SPAWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Create a host thread that runs Windows code. `body` runs after the TEB and
/// static TLS are set up (and after DLL_THREAD_ATTACH unless `is_initial`).
pub fn spawn(
    body: Box<dyn FnOnce() -> u32 + Send>,
    stack_size: usize,
    suspended: bool,
    is_initial: bool,
) -> std::io::Result<Arc<ThreadObj>> {
    spawn_with_start_address(body, stack_size, suspended, is_initial, 0)
}

/// Like [`spawn`], publishing `start_address` on the `ThreadObj` BEFORE `pthread_create`
/// so `NtQueryInformationThread(ThreadQuerySetWin32StartAddress)` sees the entry address
/// immediately from `DLL_THREAD_ATTACH` and the thread's own entry function.
pub fn spawn_with_start_address(
    body: Box<dyn FnOnce() -> u32 + Send>,
    stack_size: usize,
    suspended: bool,
    is_initial: bool,
    start_address: usize,
) -> std::io::Result<Arc<ThreadObj>> {
    if NO_SPAWN.load(Ordering::Relaxed) {
        return Err(std::io::Error::from_raw_os_error(libc::EAGAIN));
    }
    let page = 0x4000usize;
    let size = ((stack_size.max(8 * 1024 * 1024)) + page - 1) & !(page - 1);
    let guard = page;
    let total = size + guard;
    unsafe {
        let mem = libc::mmap(
            std::ptr::null_mut(),
            total,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        if mem == libc::MAP_FAILED {
            return Err(std::io::Error::last_os_error());
        }
        libc::mprotect(mem, guard, libc::PROT_NONE);
        let lo = mem as usize + guard;
        let hi = lo + size;
        let obj = ThreadObj::new(suspended);
        obj.tid.store(teb::alloc_tid() as usize, Ordering::Relaxed);
        obj.start_address.store(start_address, Ordering::Release);
        *defer_lock(&obj.stack_alloc) = Some((mem as usize, total));
        defer_lock(&THREADS).push(obj.clone());
        let info = Box::new(StartInfo {
            obj: obj.clone(),
            body,
            stack_lo: lo,
            stack_hi: hi,
            is_initial,
        });
        let info_ptr = Box::into_raw(info);
        let mut attr: libc::pthread_attr_t = std::mem::zeroed();
        libc::pthread_attr_init(&mut attr);
        pthread_attr_setstack(&mut attr, lo as *mut c_void, size);
        // Keep joinable so the stack reaper can `pthread_join` and safely `munmap`
        // the custom mmap stack after the thread has completely exited.
        libc::pthread_attr_setdetachstate(&mut attr, libc::PTHREAD_CREATE_JOINABLE);
        let mut th: libc::pthread_t = std::mem::zeroed();
        let rc = libc::pthread_create(&mut th, &attr, thread_main, info_ptr as *mut c_void);
        libc::pthread_attr_destroy(&mut attr);
        if rc != 0 {
            drop(Box::from_raw(info_ptr));
            let _ = defer_lock(&obj.stack_alloc).take();
            libc::munmap(mem, total);
            defer_lock(&THREADS).retain(|o| !Arc::ptr_eq(o, &obj));
            obj.mark_exiting();
            return Err(std::io::Error::from_raw_os_error(rc));
        }
        {
            let s = defer_lock(&obj.suspend);
            if s.phase != ThreadPhase::Exiting {
                obj.pthread.store(th as usize, Ordering::Release);
            }
        }
        Ok(obj)
    }
}

extern "C" fn thread_main(arg: *mut c_void) -> *mut c_void {
    let info = unsafe { Box::from_raw(arg as *mut StartInfo) };
    // Ensure SIGUSR1 is unblocked on every spawned guest thread so TerminateThread
    // does not inherit a blocked signal mask from an embedding host process.
    unsafe {
        let mut set: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut set);
        libc::sigaddset(&mut set, libc::SIGUSR1);
        libc::pthread_sigmask(libc::SIG_UNBLOCK, &set, std::ptr::null_mut());
    }
    if let Some(h) = THREAD_START_HOOK.get() {
        h();
    }
    let obj = info.obj.clone();
    {
        let s = defer_lock(&obj.suspend);
        if s.phase != ThreadPhase::Exiting {
            obj.pthread.store(unsafe { libc::pthread_self() } as usize, Ordering::Release);
        }
    }
    struct ExitGuard(Arc<ThreadObj>);
    impl Drop for ExitGuard {
        fn drop(&mut self) {
            self.0.mark_exiting();
            self.0.pthread.store(0, Ordering::Release);
            let p = unsafe { libc::pthread_self() as usize };
            enqueue_stack_reap_for(&self.0, p);
        }
    }
    let _exit_guard = ExitGuard(obj.clone());
    let t = unsafe { teb::init_thread(info.stack_hi, info.stack_lo, obj.tid.load(Ordering::Relaxed) as u32) };
    obj.teb.store(t as usize, Ordering::Relaxed);
    ACT_SLOT.with(|c| c.set(&*obj as *const ThreadObj));
    CURRENT.with(|c| *c.borrow_mut() = Some(obj.clone()));
    if !obj.wait_resumed() {
        ACT_SLOT.with(|c| c.set(std::ptr::null()));
        CURRENT.with(|c| *c.borrow_mut() = None);
        obj.teb.store(0, Ordering::Relaxed);
        unsafe {
            teb::uninstall();
            if !t.is_null() {
                teb::free_teb(t);
            }
        }
        return std::ptr::null_mut();
    }
    sched::apply_self_qos(obj.priority.load(Ordering::Relaxed));
    let aff = obj.affinity_mask.load(Ordering::Relaxed);
    if aff != 0 {
        let pm = crate::kernel32::process::current_process_affinity_mask();
        sched::apply_affinity_mask(&obj, aff, pm);
    }
    if let Some(l) = crate::try_loader() {
        l.thread_attach(t, info.is_initial);
    }
    let code = (info.body)();
    finish_current_thread(code);
    std::ptr::null_mut()
}

/// Thread teardown: FLS callbacks, DLL_THREAD_DETACH, signal waiters, release
/// the borrowed TSD slots, and schedule custom stack reclamation.
pub fn finish_current_thread(code: u32) {
    ACT_SLOT.with(|c| c.set(std::ptr::null()));
    let obj = current_thread_object();
    if let Some(ref obj) = obj {
        obj.mark_exiting();
    }
    run_fls_callbacks();
    let t = teb::current_raw();
    if !t.is_null() {
        if let Some(l) = crate::try_loader() {
            l.thread_detach(t);
        }
    }
    if let Some(ref obj) = obj {
        obj.pthread.store(0, Ordering::Release);
        {
            let g = handles::wait_lock();
            *obj.exit_code.get(&g) = Some(code);
        }
        handles::notify_object_key(&**obj as *const ThreadObj as usize, usize::MAX);
        obj.teb.store(0, Ordering::Relaxed);
        defer_lock(&THREADS).retain(|o| !Arc::ptr_eq(o, obj));
    }
    CURRENT.with(|c| *c.borrow_mut() = None);
    unsafe {
        teb::uninstall();
        if !t.is_null() {
            teb::free_teb(t);
        }
    }
    if let Some(ref obj) = obj {
        let pthread = unsafe { libc::pthread_self() as usize };
        enqueue_stack_reap_for(obj, pthread);
    }
}

/// `ExitThread` from Windows code: tear down and exit the host thread.
pub fn exit_current_thread(code: u32) -> ! {
    finish_current_thread(code);
    unsafe { libc::pthread_exit(std::ptr::null_mut()) }
}

#[inline(always)]
pub fn has_pending_apc() -> bool {
    let p = ACT_SLOT.with(|c| c.get());
    if p.is_null() {
        return false;
    }
    unsafe { (*p).has_apc.load(Ordering::Acquire) }
}

pub fn run_pending_apcs() {
    if let Some(t) = current_thread_object() {
        loop {
            let next = {
                let mut q = defer_lock(&t.apcs);
                if q.is_empty() {
                    t.has_apc.store(false, Ordering::Release);
                    None
                } else {
                    let item = q.remove(0);
                    if q.is_empty() {
                        t.has_apc.store(false, Ordering::Release);
                    }
                    Some(item)
                }
            };
            match next {
                Some(ApcItem::User { func, param }) => unsafe {
                    crate::call_guest8(func as usize, param as u64, 0, 0, 0, 0, 0, 0, 0);
                },
                Some(ApcItem::IoCompletion { func, err_code, bytes, overlapped, flags }) => unsafe {
                    crate::call_guest8(
                        func,
                        err_code as u64,
                        bytes as u64,
                        overlapped as u64,
                        flags as u64,
                        0,
                        0,
                        0,
                        0,
                    );
                },
                None => break,
            }
        }
    }
}

pub fn queue_apc(t: &ThreadObj, f: ApcProc, param: usize) {
    {
        let mut q = defer_lock(&t.apcs);
        q.push(ApcItem::User { func: f, param });
        t.has_apc.store(true, Ordering::Release);
    }
    // `notify_thread_apc` issues a SeqCst fence before checking ALERTABLE_WAITERS,
    // waking ONLY this specific thread if it is in an alertable wait.
    handles::notify_thread_apc(t.tid.load(Ordering::Relaxed) as u32);
}

pub fn queue_io_completion_apc(t: &ThreadObj, func: IoCompletionRoutine, err_code: u32, bytes: u32, overlapped: usize) {
    queue_io_completion_apc_with_flags(t, func as usize, err_code, bytes, overlapped, 0);
}

/// Queue an I/O completion APC carrying an explicit 4th `dwFlags` argument
/// (`LPWSAOVERLAPPED_COMPLETION_ROUTINE(dwError, cbTransferred, lpOverlapped, dwFlags)`).
pub fn queue_io_completion_apc_with_flags(
    t: &ThreadObj,
    func: usize,
    err_code: u32,
    bytes: u32,
    overlapped: usize,
    flags: u32,
) {
    {
        let mut q = defer_lock(&t.apcs);
        q.push(ApcItem::IoCompletion { func, err_code, bytes, overlapped, flags });
        t.has_apc.store(true, Ordering::Release);
    }
    handles::notify_thread_apc(t.tid.load(Ordering::Relaxed) as u32);
}

// ---------------------------------------------------------------------------
// TLS slots (TEB.TlsSlots / TlsExpansionSlots)
// ---------------------------------------------------------------------------

static TLS_BITMAP: Mutex<[bool; teb::TLS_MINIMUM_AVAILABLE + teb::TLS_EXPANSION_SLOTS]> =
    Mutex::new([false; teb::TLS_MINIMUM_AVAILABLE + teb::TLS_EXPANSION_SLOTS]);

pub const TLS_OUT_OF_INDEXES: u32 = 0xFFFF_FFFF;

pub fn tls_alloc() -> u32 {
    let mut b = defer_lock(&TLS_BITMAP);
    match b.iter().position(|x| !*x) {
        Some(i) => {
            b[i] = true;
            // New slots must read as NULL in every thread.
            for t in all_tebs().into_iter().chain(std::iter::once(teb::current())) {
                unsafe { tls_slot_ptr(t, i as u32, false).map(|p| *p = 0) };
            }
            i as u32
        }
        None => TLS_OUT_OF_INDEXES,
    }
}

pub fn tls_free(i: u32) -> bool {
    let mut b = defer_lock(&TLS_BITMAP);
    match b.get_mut(i as usize) {
        Some(x) if *x => {
            *x = false;
            true
        }
        _ => false,
    }
}

pub fn tls_valid(i: u32) -> bool {
    defer_lock(&TLS_BITMAP).get(i as usize).copied().unwrap_or(false)
}

/// Pointer to slot `i` in `teb` (allocating the expansion array if `create`).
pub unsafe fn tls_slot_ptr(t: *mut u8, i: u32, create: bool) -> Option<*mut usize> {
    let i = i as usize;
    if i < teb::TLS_MINIMUM_AVAILABLE {
        return Some(t.add(teb::off::TLS_SLOTS + i * 8) as *mut usize);
    }
    let j = i - teb::TLS_MINIMUM_AVAILABLE;
    if j >= teb::TLS_EXPANSION_SLOTS {
        return None;
    }
    let mut exp: usize = teb::field(t, teb::off::TLS_EXPANSION_SLOTS);
    if exp == 0 {
        if !create {
            return None;
        }
        exp = libc::calloc(teb::TLS_EXPANSION_SLOTS, 8) as usize;
        teb::set_field(t, teb::off::TLS_EXPANSION_SLOTS, exp);
    }
    Some((exp as *mut usize).add(j))
}

// ---------------------------------------------------------------------------
// FLS (fiber-local storage; lock-free slot validity & callback lookup)
// ---------------------------------------------------------------------------

pub type FlsCallback = unsafe extern "win64" fn(*mut c_void);
const MAX_FLS_SLOTS: usize = 4096;
static FLS_ALLOC_LOCK: Mutex<()> = Mutex::new(());
static FLS_ALLOCATED: [AtomicBool; MAX_FLS_SLOTS] = [const { AtomicBool::new(false) }; MAX_FLS_SLOTS];
static FLS_CALLBACKS: [AtomicUsize; MAX_FLS_SLOTS] = [const { AtomicUsize::new(0) }; MAX_FLS_SLOTS];
pub const FLS_OUT_OF_INDEXES: u32 = 0xFFFF_FFFF;

pub fn fls_alloc(cb: Option<FlsCallback>) -> u32 {
    let _g = defer_lock(&FLS_ALLOC_LOCK);
    for i in 0..MAX_FLS_SLOTS {
        if !FLS_ALLOCATED[i].load(Ordering::Relaxed) {
            FLS_CALLBACKS[i].store(cb.map_or(0, |f| f as usize), Ordering::Relaxed);
            FLS_ALLOCATED[i].store(true, Ordering::Release);
            return i as u32;
        }
    }
    FLS_OUT_OF_INDEXES
}

pub fn fls_free(i: u32) -> bool {
    let idx = i as usize;
    if idx >= MAX_FLS_SLOTS {
        return false;
    }
    let cb = {
        let _g = defer_lock(&FLS_ALLOC_LOCK);
        if !FLS_ALLOCATED[idx].load(Ordering::Relaxed) {
            return false;
        }
        let cb = FLS_CALLBACKS[idx].swap(0, Ordering::Relaxed);
        FLS_ALLOCATED[idx].store(false, Ordering::Release);
        cb
    };
    // Windows calls the callback for the current thread's value (all threads, really).
    let v = fls_get(i);
    fls_set(i, 0);
    if cb != 0 && v != 0 {
        unsafe {
            crate::call_guest8(cb, v as u64, 0, 0, 0, 0, 0, 0, 0);
        }
    }
    true
}

#[inline]
pub fn fls_valid(i: u32) -> bool {
    FLS_ALLOCATED.get(i as usize).is_some_and(|a| a.load(Ordering::Acquire))
}

pub fn fls_get(i: u32) -> usize {
    FLS.with(|f| f.borrow().get(i as usize).copied().unwrap_or(0))
}

pub fn fls_set(i: u32, v: usize) {
    FLS.with(|f| {
        let mut f = f.borrow_mut();
        if f.len() <= i as usize {
            f.resize(i as usize + 1, 0);
        }
        f[i as usize] = v;
    })
}

pub(crate) fn run_fls_callbacks() {
    let vals: Vec<usize> = FLS.with(|f| std::mem::take(&mut *f.borrow_mut()));
    for (i, v) in vals.into_iter().enumerate() {
        if v == 0 || i >= MAX_FLS_SLOTS {
            continue;
        }
        if FLS_ALLOCATED[i].load(Ordering::Acquire) {
            let cb = FLS_CALLBACKS[i].load(Ordering::Acquire);
            if cb != 0 {
                unsafe {
                    crate::call_guest8(cb, v as u64, 0, 0, 0, 0, 0, 0, 0);
                }
            }
        }
    }
}

extern "C" {
    // Not bound by the libc crate for apple targets.
    fn pthread_attr_setstack(attr: *mut libc::pthread_attr_t, addr: *mut c_void, size: usize) -> libc::c_int;
}

// ---------------------------------------------------------------------------
// Host scheduling (SetThreadPriority, audio realtime promotion)
// ---------------------------------------------------------------------------

/// Mach scheduling of host threads. Win32 priorities map to
/// `THREAD_PRECEDENCE_POLICY` importance (relative to the task's base
/// priority); a realtime thread (the WASAPI render client's thread) gets
/// `THREAD_TIME_CONSTRAINT_POLICY`. Only plain mach/pthread calls (no TSD
/// slots 6/11/12, no CoreFoundation), so this is safe on Win32 threads.
pub mod sched {
    use super::ThreadObj;
    use std::sync::atomic::Ordering;

    const THREAD_TIME_CONSTRAINT_POLICY: i32 = 2;
    const THREAD_TIME_CONSTRAINT_POLICY_COUNT: u32 = 4;
    const THREAD_PRECEDENCE_POLICY: i32 = 3;
    const THREAD_PRECEDENCE_POLICY_COUNT: u32 = 1;
    #[cfg(target_os = "macos")]
    const THREAD_AFFINITY_POLICY: i32 = 4;
    #[cfg(target_os = "macos")]
    const THREAD_AFFINITY_POLICY_COUNT: u32 = 1;
    #[cfg(target_os = "macos")]
    const THREAD_AFFINITY_TAG_NULL: i32 = 0;

    /// `thread_time_constraint_policy_data_t` (all times in mach absolute units).
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct TimeConstraint {
        pub period: u32,
        pub computation: u32,
        pub constraint: u32,
        pub preemptible: i32,
    }

    #[repr(C)]
    struct Timebase {
        numer: u32,
        denom: u32,
    }

    extern "C" {
        fn pthread_mach_thread_np(t: libc::pthread_t) -> u32;
        fn thread_policy_set(thread: u32, flavor: i32, info: *const i32, count: u32) -> i32;
        fn thread_policy_get(thread: u32, flavor: i32, info: *mut i32, count: *mut u32, get_default: *mut u32) -> i32;
        fn mach_timebase_info(info: *mut Timebase) -> i32;
        #[cfg(target_vendor = "apple")]
        fn thread_suspend(target_act: u32) -> i32;
        #[cfg(target_vendor = "apple")]
        fn thread_resume(target_act: u32) -> i32;
    }

    #[cfg(target_vendor = "apple")]
    pub(super) fn thread_suspend_port(port: u32) -> i32 {
        unsafe { thread_suspend(port) }
    }

    #[cfg(target_vendor = "apple")]
    pub(super) fn thread_resume_port(port: u32) -> i32 {
        unsafe { thread_resume(port) }
    }

    fn cached_timebase() -> (u32, u32) {
        static TB: std::sync::OnceLock<(u32, u32)> = std::sync::OnceLock::new();
        *TB.get_or_init(|| {
            let mut tb = Timebase { numer: 0, denom: 0 };
            unsafe { mach_timebase_info(&mut tb) };
            (tb.numer, tb.denom)
        })
    }

    fn thread_priority_disabled() -> bool {
        static OFF: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *OFF.get_or_init(|| std::env::var("AW2_THREAD_PRIORITY").is_ok_and(|v| v == "0"))
    }

    #[cfg(target_vendor = "apple")]
    fn ios_normal_qos_initiated() -> bool {
        static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *ON.get_or_init(|| {
            cfg!(target_os = "ios")
                && std::env::var("PORT_IOS_NORMAL_QOS")
                    .is_ok_and(|v| v.eq_ignore_ascii_case("initiated"))
        })
    }

    /// Nanoseconds -> mach absolute time units (1:1 under Rosetta, 125/3 ns
    /// per tick on native Apple silicon).
    pub fn ns_to_abs(ns: u64) -> u32 {
        let (numer, denom) = cached_timebase();
        if numer == 0 || denom == 0 {
            return ns.min(u32::MAX as u64) as u32;
        }
        (ns as u128 * denom as u128 / numer as u128).min(u32::MAX as u128) as u32
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct MachTimeValue {
        seconds: i32,
        microseconds: i32,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct MachThreadBasicInfo {
        user_time: MachTimeValue,
        system_time: MachTimeValue,
        cpu_usage: i32,
        policy: i32,
        run_state: i32,
        flags: i32,
        suspend_count: i32,
        sleep_time: i32,
    }

    /// Query Mach `THREAD_BASIC_INFO` for `port`, returning `Some((kernel_100ns, user_100ns))`.
    pub fn thread_cpu_times_100ns(port: u32) -> Option<(u64, u64)> {
        if port == 0 {
            return None;
        }
        const THREAD_BASIC_INFO: i32 = 3;
        extern "C" {
            fn thread_info(target_act: u32, flavor: i32, thread_info_out: *mut i32, count: *mut u32) -> i32;
        }
        let mut info = MachThreadBasicInfo::default();
        let mut count = (std::mem::size_of::<MachThreadBasicInfo>() / std::mem::size_of::<u32>()) as u32;
        let kr = unsafe {
            thread_info(port, THREAD_BASIC_INFO, &mut info as *mut _ as *mut i32, &mut count)
        };
        if kr != 0 {
            return None;
        }
        let user = (info.user_time.seconds.max(0) as u64)
            .saturating_mul(10_000_000)
            .saturating_add((info.user_time.microseconds.max(0) as u64).saturating_mul(10));
        let kernel = (info.system_time.seconds.max(0) as u64)
            .saturating_mul(10_000_000)
            .saturating_add((info.system_time.microseconds.max(0) as u64).saturating_mul(10));
        Some((kernel, user))
    }

    /// Mach thread port of the current thread (no reference to release).
    pub fn current_port() -> u32 {
        unsafe { pthread_mach_thread_np(libc::pthread_self()) }
    }

    /// Mach port of a Win32 thread object (`None` if it has no host pthread yet).
    pub fn port_of(t: &ThreadObj) -> Option<u32> {
        let cur = super::current_thread_object();
        if cur.is_some_and(|c| std::ptr::eq(&*c, t)) {
            return Some(current_port());
        }
        {
            let s = super::defer_lock(&t.suspend);
            if s.phase == super::ThreadPhase::Exiting {
                return None;
            }
            if s.mach_port != 0 {
                return Some(s.mach_port);
            }
        }
        // Detached pthreads free their struct on exit.
        if t.exit_code().is_some() {
            return None;
        }
        match t.pthread.load(Ordering::Relaxed) {
            0 => None,
            p => Some(unsafe { pthread_mach_thread_np(p as libc::pthread_t) }),
        }
    }

    /// Precedence importance for a Win32 thread priority (IDLE -15 ..
    /// TIME_CRITICAL 15). The kernel clamps the result to the user range.
    pub fn importance_for(prio: i32) -> i32 {
        match prio {
            i32::MIN..=-15 => -16,     // THREAD_PRIORITY_IDLE
            -14..=-2 => -6,            // LOWEST (and IDLE+n)
            -1 => -3,                  // BELOW_NORMAL
            0 => 0,                    // NORMAL
            1 => 3,                    // ABOVE_NORMAL
            2..=14 => 6,               // HIGHEST
            15.. => 15,                // TIME_CRITICAL
        }
    }

    pub fn set_importance(port: u32, importance: i32) -> Result<(), i32> {
        let kr = unsafe { thread_policy_set(port, THREAD_PRECEDENCE_POLICY, &importance, THREAD_PRECEDENCE_POLICY_COUNT) };
        if kr == 0 { Ok(()) } else { Err(kr) }
    }

    pub fn importance(port: u32) -> Option<i32> {
        let mut v = 0i32;
        let mut n = THREAD_PRECEDENCE_POLICY_COUNT;
        let mut def = 0u32;
        let kr = unsafe { thread_policy_get(port, THREAD_PRECEDENCE_POLICY, &mut v, &mut n, &mut def) };
        (kr == 0).then_some(v)
    }

    /// Apply Darwin QoS class matching a Win32 priority on the calling thread.
    ///
    /// Keeps `NORMAL` (`prio == 0`) threads inside `QOS_CLASS_USER_INTERACTIVE`
    /// (relative priority `-2`) so the guest main thread and job-system workers
    /// never get stranded on E-cores behind locks held against the render or audio
    /// threads (which sit at `QOS_CLASS_USER_INTERACTIVE`, relative priority `0`).
    /// Set `PORT_IOS_NORMAL_QOS=initiated` to opt into `QOS_CLASS_USER_INITIATED`
    /// for `NORMAL` threads on iOS.
    pub fn apply_self_qos(prio: i32) {
        #[cfg(target_vendor = "apple")]
        {
            if thread_priority_disabled() {
                return;
            }
            let normal_initiated = ios_normal_qos_initiated();
            let (qos, rel) = match prio {
                i32::MIN..=-2 => (libc::qos_class_t::QOS_CLASS_UTILITY, 0),
                -1 => (libc::qos_class_t::QOS_CLASS_USER_INITIATED, 0),
                0 if normal_initiated => (libc::qos_class_t::QOS_CLASS_USER_INITIATED, 0),
                0 => (libc::qos_class_t::QOS_CLASS_USER_INTERACTIVE, -2),
                1.. => (libc::qos_class_t::QOS_CLASS_USER_INTERACTIVE, 0),
            };
            unsafe {
                libc::pthread_set_qos_class_self_np(qos, rel);
            }
        }
        #[cfg(not(target_vendor = "apple"))]
        {
            let _ = prio;
        }
    }

    /// Promote the calling thread (e.g. the game's presenting render thread) to
    /// `QOS_CLASS_USER_INTERACTIVE` so it receives P-core scheduling alongside the
    /// Metal command-stream encoder and present threads.
    pub fn promote_current_thread_interactive() {
        #[cfg(target_vendor = "apple")]
        {
            if thread_priority_disabled() {
                return;
            }
            unsafe {
                libc::pthread_set_qos_class_self_np(libc::qos_class_t::QOS_CLASS_USER_INTERACTIVE, 0);
            }
        }
    }

    /// Map a Win32 thread affinity mask to a Darwin `THREAD_AFFINITY_POLICY` cache-affinity
    /// tag (`THREAD_AFFINITY_TAG_NULL` when unconstrained to all process CPUs, or a
    /// deterministic positive tag for a strict CPU subset so threads sharing a mask share an
    /// L2/cluster affinity hint without lowering QoS). Best-effort: Apple Silicon XNU may
    /// return `KERN_NOT_SUPPORTED` when cluster scheduling overrides affinity tags.
    pub fn apply_affinity_mask(t: &ThreadObj, mask: usize, proc_mask: usize) {
        #[cfg(target_os = "macos")]
        {
            let Some(port) = port_of(t) else { return };
            let tag: i32 = if mask == 0 || mask == proc_mask {
                THREAD_AFFINITY_TAG_NULL
            } else if mask.count_ones() == 1 {
                (mask.trailing_zeros() as i32) + 1
            } else {
                let folded = ((mask as u32) ^ ((mask >> 32) as u32)) & 0x7FFF_FFFF;
                (folded.max(1)) as i32
            };
            let _ = unsafe { thread_policy_set(port, THREAD_AFFINITY_POLICY, &tag, THREAD_AFFINITY_POLICY_COUNT) };
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (t, mask, proc_mask);
        }
    }

    /// `SetThreadPriority`: apply the Win32 priority to the host thread.
    /// `AW2_THREAD_PRIORITY=0` keeps the old record-only behaviour.
    pub fn apply_win_priority(t: &ThreadObj, prio: i32) -> Result<(), i32> {
        if thread_priority_disabled() {
            return Ok(());
        }
        if super::current_thread_object().is_some_and(|c| std::ptr::eq(&*c, t)) {
            apply_self_qos(prio);
        }
        let Some(port) = port_of(t) else { return Err(-1) };
        set_importance(port, importance_for(prio))
    }

    /// Time-constraint policy for a periodic realtime thread: `period_ns`
    /// between wakeups, `computation_ns` of CPU per period, finished within
    /// `constraint_ns` of the wakeup.
    pub fn time_constraint_for(period_ns: u64, computation_ns: u64, constraint_ns: u64) -> TimeConstraint {
        TimeConstraint {
            period: ns_to_abs(period_ns),
            computation: ns_to_abs(computation_ns),
            constraint: ns_to_abs(constraint_ns.max(computation_ns)),
            preemptible: 1,
        }
    }

    pub fn set_time_constraint(port: u32, tc: &TimeConstraint) -> Result<(), i32> {
        let kr = unsafe { thread_policy_set(port, THREAD_TIME_CONSTRAINT_POLICY, tc as *const TimeConstraint as *const i32, THREAD_TIME_CONSTRAINT_POLICY_COUNT) };
        if kr == 0 { Ok(()) } else { Err(kr) }
    }

    /// Back to `THREAD_STANDARD_POLICY` (timesharing; drops a time constraint).
    pub fn set_standard_policy(port: u32) -> Result<(), i32> {
        const THREAD_STANDARD_POLICY: i32 = 1;
        let dummy = 0i32;
        let kr = unsafe { thread_policy_set(port, THREAD_STANDARD_POLICY, &dummy, 0) };
        if kr == 0 { Ok(()) } else { Err(kr) }
    }

    /// Current time-constraint policy, `None` if the thread is not realtime
    /// (the kernel reports the defaults with `get_default` set then).
    pub fn time_constraint(port: u32) -> Option<TimeConstraint> {
        let mut tc = TimeConstraint::default();
        let mut n = THREAD_TIME_CONSTRAINT_POLICY_COUNT;
        let mut def = 0u32;
        let kr = unsafe { thread_policy_get(port, THREAD_TIME_CONSTRAINT_POLICY, &mut tc as *mut TimeConstraint as *mut i32, &mut n, &mut def) };
        (kr == 0 && def == 0).then_some(tc)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn precedence_and_time_constraint_apply_to_the_host_thread() {
            std::thread::spawn(|| {
                let port = current_port();
                assert!(time_constraint(port).is_none(), "fresh thread already realtime");
                set_importance(port, importance_for(1)).unwrap();
                assert_eq!(importance(port), Some(3));
                // 512 frames at 48 kHz, half of it as computation.
                let period = 512_000_000_000u64 / 48_000;
                let tc = time_constraint_for(period, period / 2, period);
                set_time_constraint(port, &tc).unwrap();
                let got = time_constraint(port).expect("thread is realtime after promotion");
                // The kernel keeps the values in native ticks (24 MHz on Apple
                // silicon, also under Rosetta), so allow tick rounding.
                let close = |a: u32, b: u32| (a as i64 - b as i64).abs() <= (b as i64 / 1000).max(1);
                assert!(close(got.period, tc.period) && close(got.computation, tc.computation) && close(got.constraint, tc.constraint), "{got:?} vs {tc:?}");
                assert_eq!(got.preemptible, 1);
            })
            .join()
            .unwrap();
        }

        #[test]
        fn stack_reap_ownership_is_consumed_once_across_teardown_paths() {
            // Burst of short-lived threads exercises the race between creator publishing
            // `obj.pthread` and child running `finish_current_thread` + `ExitGuard::drop`.
            let mut objs = Vec::new();
            for i in 0u32..32 {
                let obj = super::super::spawn(Box::new(move || i), 64 * 1024, false, false)
                    .expect("spawn short-lived thread");
                objs.push((i, obj));
            }
            for (i, obj) in objs {
                let dl = std::time::Instant::now() + std::time::Duration::from_secs(5);
                while obj.exit_code().is_none() || super::super::defer_lock(&obj.stack_alloc).is_some() {
                    assert!(std::time::Instant::now() < dl, "short-lived thread {i} did not finish teardown");
                    std::thread::sleep(std::time::Duration::from_micros(200));
                }
                assert_eq!(obj.exit_code(), Some(i));
                // Once teardown ran, `stack_alloc` must be consumed and `pthread` must
                // never be overwritten with a stale handle by the creator.
                assert!(
                    super::super::defer_lock(&obj.stack_alloc).is_none(),
                    "stack_alloc must be consumed exactly once by teardown"
                );
                assert_eq!(obj.pthread.load(Ordering::Acquire), 0);
                // Calling `enqueue_stack_reap_for` again (simulating a second teardown path)
                // must be a no-op and never enqueue duplicate reclamation.
                assert!(!super::super::enqueue_stack_reap_for(&obj, 0x1234));
            }
        }
    }
}
