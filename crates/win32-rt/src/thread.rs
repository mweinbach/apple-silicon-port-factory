//! Windows threads: TEB setup, static TLS via the loader, thread objects,
//! APC queue, FLS and TLS slots.

use crate::handles::{self, Guarded, KObject, WaitGuard, Waitable};
use crate::teb;
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex, OnceLock};

pub type ThreadProc = unsafe extern "win64" fn(*mut c_void) -> u32;
pub type ApcProc = unsafe extern "win64" fn(usize);

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
    pub apcs: Mutex<Vec<(ApcProc, usize)>>,
    /// Lock-free mirror of `!apcs.is_empty()` for zero-overhead `has_pending_apc()`.
    pub has_apc: AtomicBool,
    pub name: Mutex<String>,
    suspend: Mutex<SuspendState>,
    resume_cv: Condvar,
    pub pthread: AtomicUsize,
    pub priority: std::sync::atomic::AtomicI32,
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
            act_kind: AtomicU8::new(0),
            act_start_ns: AtomicU64::new(0),
            act_arg: AtomicU64::new(0),
        })
    }

    pub fn exit_code(&self) -> Option<u32> {
        let g = handles::wait_lock();
        *self.exit_code.get(&g)
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
            while s.count > 0 && s.phase != ThreadPhase::Exiting {
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
        let mut s = self.suspend.lock().unwrap();
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

    fn wait_resumed(&self) {
        let mut s = self.suspend.lock().unwrap();
        while s.count > 0 {
            s = self.resume_cv.wait(s).unwrap();
        }
        s.phase = ThreadPhase::Live;
        #[cfg(target_vendor = "apple")]
        {
            s.mach_port = sched::current_port();
        }
    }

    fn mark_exiting(&self) {
        let mut s = self.suspend.lock().unwrap();
        if s.phase == ThreadPhase::Exiting {
            return;
        }
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
    /// Custom mmap stack allocation `(base_ptr, total_len)` owned by a `thread::spawn`ed
    /// thread, handed off to the stack reaper on exit.
    static STACK_ALLOC: Cell<Option<(usize, usize)>> = const { Cell::new(None) };
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
                        let _ = libc::pthread_join(req.pthread as libc::pthread_t, std::ptr::null_mut());
                        libc::munmap(req.mem as *mut c_void, req.len);
                    }
                }
            });
        Mutex::new(tx)
    })
}

fn enqueue_current_stack_reap() {
    if let Some((mem, len)) = STACK_ALLOC.with(|c| c.take()) {
        let pthread = unsafe { libc::pthread_self() as usize };
        let _ = stack_reaper_tx().lock().unwrap().send(ReapReq { pthread, mem, len });
    }
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

pub fn find_by_tid(tid: u32) -> Option<Arc<ThreadObj>> {
    THREADS.lock().unwrap().iter().find(|t| t.tid.load(Ordering::Relaxed) as u32 == tid).cloned()
}

/// Snapshot of all live Windows thread objects.
pub fn all_threads() -> Vec<Arc<ThreadObj>> {
    THREADS.lock().unwrap().clone()
}

/// All live TEBs (the loader extends static TLS of every thread when a DLL
/// with a TLS directory is loaded at runtime).
pub fn all_tebs() -> Vec<*mut u8> {
    THREADS
        .lock()
        .unwrap()
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
    stack_mem: usize,
    stack_total: usize,
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
        THREADS.lock().unwrap().push(obj.clone());
        let info = Box::new(StartInfo {
            obj: obj.clone(),
            body,
            stack_mem: mem as usize,
            stack_total: total,
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
            libc::munmap(mem, total);
            THREADS.lock().unwrap().retain(|o| !Arc::ptr_eq(o, &obj));
            obj.mark_exiting();
            return Err(std::io::Error::from_raw_os_error(rc));
        }
        obj.pthread.store(th as usize, Ordering::Relaxed);
        Ok(obj)
    }
}

extern "C" fn thread_main(arg: *mut c_void) -> *mut c_void {
    let info = unsafe { Box::from_raw(arg as *mut StartInfo) };
    STACK_ALLOC.with(|c| c.set(Some((info.stack_mem, info.stack_total))));
    if let Some(h) = THREAD_START_HOOK.get() {
        h();
    }
    let obj = info.obj.clone();
    obj.pthread.store(unsafe { libc::pthread_self() } as usize, Ordering::Relaxed);
    struct ExitGuard(Arc<ThreadObj>);
    impl Drop for ExitGuard {
        fn drop(&mut self) {
            self.0.mark_exiting();
            enqueue_current_stack_reap();
        }
    }
    let _exit_guard = ExitGuard(obj.clone());
    let t = unsafe { teb::init_thread(info.stack_hi, info.stack_lo, obj.tid.load(Ordering::Relaxed) as u32) };
    obj.teb.store(t as usize, Ordering::Relaxed);
    ACT_SLOT.with(|c| c.set(&*obj as *const ThreadObj));
    CURRENT.with(|c| *c.borrow_mut() = Some(obj.clone()));
    obj.wait_resumed();
    sched::apply_self_qos(obj.priority.load(Ordering::Relaxed));
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
    if let Some(obj) = current_thread_object() {
        obj.mark_exiting();
    }
    ACT_SLOT.with(|c| c.set(std::ptr::null()));
    run_fls_callbacks();
    let t = teb::current_raw();
    if !t.is_null() {
        if let Some(l) = crate::try_loader() {
            l.thread_detach(t);
        }
    }
    if let Some(obj) = current_thread_object() {
        obj.pthread.store(0, Ordering::Release);
        {
            let g = handles::wait_lock();
            *obj.exit_code.get(&g) = Some(code);
        }
        handles::notify_object_key(&*obj as *const ThreadObj as usize, usize::MAX);
        obj.teb.store(0, Ordering::Relaxed);
        THREADS.lock().unwrap().retain(|o| !Arc::ptr_eq(o, &obj));
    }
    CURRENT.with(|c| *c.borrow_mut() = None);
    unsafe {
        teb::uninstall();
        if !t.is_null() {
            teb::free_teb(t);
        }
    }
    enqueue_current_stack_reap();
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
                let mut q = t.apcs.lock().unwrap();
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
                Some((f, p)) => unsafe {
                    crate::call_guest8(f as usize, p as u64, 0, 0, 0, 0, 0, 0, 0);
                },
                None => break,
            }
        }
    }
}

pub fn queue_apc(t: &ThreadObj, f: ApcProc, param: usize) {
    {
        let mut q = t.apcs.lock().unwrap();
        q.push((f, param));
        t.has_apc.store(true, Ordering::Release);
    }
    // `notify_thread_apc` issues a SeqCst fence before checking ALERTABLE_WAITERS,
    // waking ONLY this specific thread if it is in an alertable wait.
    handles::notify_thread_apc(t.tid.load(Ordering::Relaxed) as u32);
}

// ---------------------------------------------------------------------------
// TLS slots (TEB.TlsSlots / TlsExpansionSlots)
// ---------------------------------------------------------------------------

static TLS_BITMAP: Mutex<[bool; teb::TLS_MINIMUM_AVAILABLE + teb::TLS_EXPANSION_SLOTS]> =
    Mutex::new([false; teb::TLS_MINIMUM_AVAILABLE + teb::TLS_EXPANSION_SLOTS]);

pub const TLS_OUT_OF_INDEXES: u32 = 0xFFFF_FFFF;

pub fn tls_alloc() -> u32 {
    let mut b = TLS_BITMAP.lock().unwrap();
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
    let mut b = TLS_BITMAP.lock().unwrap();
    match b.get_mut(i as usize) {
        Some(x) if *x => {
            *x = false;
            true
        }
        _ => false,
    }
}

pub fn tls_valid(i: u32) -> bool {
    TLS_BITMAP.lock().unwrap().get(i as usize).copied().unwrap_or(false)
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
    let _g = FLS_ALLOC_LOCK.lock().unwrap();
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
        let _g = FLS_ALLOC_LOCK.lock().unwrap();
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

fn run_fls_callbacks() {
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
            let s = t.suspend.lock().unwrap();
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
    }
}
