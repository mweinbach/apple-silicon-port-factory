//! Thread, TLS/FLS, synchronization-object and wait APIs.

use crate::handles::{self, Event, KObject, MutexObj, Semaphore, WaitResult};
use crate::strings::*;
use crate::teb;
use crate::thread::{self, ThreadObj};
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::Ordering;
use std::sync::Arc;

fn set_err(e: u32) {
    teb::set_last_error(e);
}

pub const STILL_ACTIVE: u32 = 259;
const CREATE_SUSPENDED: u32 = 0x4;
const STACK_SIZE_PARAM_IS_A_RESERVATION: u32 = 0x10000;

pub unsafe extern "win64" fn CreateThread(
    _sec: *mut c_void,
    stack: usize,
    start: Option<thread::ThreadProc>,
    param: *mut c_void,
    flags: u32,
    tid_out: *mut u32,
) -> HANDLE {
    let Some(start) = start else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    };
    let _ = STACK_SIZE_PARAM_IS_A_RESERVATION;
    let size = if stack == 0 { thread::DEFAULT_STACK_SIZE.load(Ordering::Relaxed) } else { stack };
    let start_addr = start as usize;
    let param_u = param as usize;
    let body = Box::new(move || unsafe {
        crate::call_guest8(start_addr, param_u as u64, 0, 0, 0, 0, 0, 0, 0) as u32
    });
    match thread::spawn(body, size, flags & CREATE_SUSPENDED != 0, false) {
        Ok(t) => {
            let tid = t.tid.load(Ordering::Relaxed) as u32;
            log::debug!("CreateThread start={} param={param:p} -> tid 0x{tid:x}", crate::symbolize(start as usize));
            if !tid_out.is_null() {
                *tid_out = tid;
            }
            handles::insert(t)
        }
        Err(e) => {
            if !thread::NO_SPAWN.load(Ordering::Relaxed) {
                log::error!("CreateThread failed: {e}");
            }
            set_err(err::ERROR_NOT_ENOUGH_MEMORY);
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn CreateRemoteThread(p: HANDLE, sec: *mut c_void, stack: usize, start: Option<thread::ThreadProc>, param: *mut c_void, flags: u32, tid: *mut u32) -> HANDLE {
    if p != handles::CURRENT_PROCESS {
        stub!("CreateRemoteThread into another process");
        set_err(err::ERROR_ACCESS_DENIED);
        return std::ptr::null_mut();
    }
    CreateThread(sec, stack, start, param, flags, tid)
}

pub unsafe extern "win64" fn ExitThread(code: u32) {
    thread::exit_current_thread(code);
}

pub unsafe extern "win64" fn GetCurrentThread() -> HANDLE {
    handles::CURRENT_THREAD
}

#[inline(never)]
pub unsafe extern "win64" fn GetCurrentThreadId() -> u32 {
    teb::thread_id()
}

pub unsafe extern "win64" fn GetCurrentProcessorNumber() -> u32 {
    // No host API for the current CPU; any stable value in range is valid
    // for the per-CPU sharding callers use this for.
    teb::thread_id() / 4 % crate::process::num_cpus()
}

pub unsafe extern "win64" fn GetCurrentProcessorNumberEx(out: *mut u8) {
    *(out as *mut u16) = 0;
    *out.add(2) = GetCurrentProcessorNumber() as u8;
    *out.add(3) = 0;
}

fn thread_obj(h: HANDLE) -> Option<Arc<ThreadObj>> {
    if h == handles::CURRENT_THREAD {
        return thread::current_thread_object();
    }
    handles::get_as::<ThreadObj>(h)
}

pub unsafe extern "win64" fn GetThreadId(h: HANDLE) -> u32 {
    if h == handles::CURRENT_THREAD {
        return teb::thread_id();
    }
    match thread_obj(h) {
        Some(t) => t.tid.load(Ordering::Relaxed) as u32,
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            0
        }
    }
}

pub unsafe extern "win64" fn OpenThread(_access: u32, _inherit: BOOL, tid: u32) -> HANDLE {
    match thread::find_by_tid(tid) {
        Some(t) => handles::insert(t),
        None => {
            set_err(err::ERROR_INVALID_PARAMETER);
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn GetExitCodeThread(h: HANDLE, code: *mut u32) -> BOOL {
    match thread_obj(h) {
        Some(t) => {
            *code = t.exit_code().unwrap_or(STILL_ACTIVE);
            TRUE
        }
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn ResumeThread(h: HANDLE) -> u32 {
    match thread_obj(h) {
        Some(t) => match t.resume_checked() {
            Ok(prev) => prev,
            Err(e) => {
                set_err(e);
                u32::MAX
            }
        },
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            u32::MAX
        }
    }
}

pub unsafe extern "win64" fn SuspendThread(h: HANDLE) -> u32 {
    match thread_obj(h) {
        Some(t) => match t.suspend() {
            Ok(prev) => prev,
            Err(e) => {
                set_err(e);
                u32::MAX
            }
        },
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            u32::MAX
        }
    }
}

pub unsafe extern "win64" fn TerminateThread(_h: HANDLE, _code: u32) -> BOOL {
    stub!("TerminateThread unsupported");
    set_err(err::ERROR_ACCESS_DENIED);
    FALSE
}

pub unsafe extern "win64" fn GetThreadContext(h: HANDLE, ctx: *mut c_void) -> BOOL {
    if ctx.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if h != handles::CURRENT_THREAD && thread_obj(h).is_none() {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let st = crate::ntdll::NtGetContextThread(h, ctx as *mut crate::unwind::Context);
    if st == 0 {
        TRUE
    } else {
        set_err(err::ERROR_INVALID_PARAMETER);
        FALSE
    }
}

pub unsafe extern "win64" fn SetThreadContext(h: HANDLE, ctx: *const c_void) -> BOOL {
    if ctx.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if h != handles::CURRENT_THREAD && thread_obj(h).is_none() {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let st = crate::ntdll::NtSetContextThread(h, ctx as *const crate::unwind::Context);
    if st == 0 {
        TRUE
    } else {
        set_err(err::ERROR_INVALID_PARAMETER);
        FALSE
    }
}

pub unsafe extern "win64" fn SetThreadPriority(h: HANDLE, prio: i32) -> BOOL {
    match thread_obj(h) {
        Some(t) => {
            t.priority.store(prio, Ordering::Relaxed);
            // Mach precedence importance (thread::sched::importance_for). The
            // Wwise render thread additionally gets a time-constraint policy
            // from the WASAPI render client (audio::audioclient), which is
            // where its period is known; avrt.dll is not imported by the exe.
            match thread::sched::apply_win_priority(&t, prio) {
                Ok(()) => log::debug!("SetThreadPriority(tid 0x{:x}, {prio}) -> importance {}", t.tid.load(Ordering::Relaxed), thread::sched::importance_for(prio)),
                Err(kr) => log::debug!("SetThreadPriority(tid 0x{:x}, {prio}): recorded, host policy not applied ({kr})", t.tid.load(Ordering::Relaxed)),
            }
            TRUE
        }
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn GetThreadPriority(h: HANDLE) -> i32 {
    thread_obj(h).map(|t| t.priority.load(Ordering::Relaxed)).unwrap_or(0x7FFF_FFFF)
}

pub unsafe extern "win64" fn SetThreadPriorityBoost(_h: HANDLE, _d: BOOL) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetThreadAffinityMask(_h: HANDLE, _mask: usize) -> usize {
    stub!("thread affinity not applied");
    let n = crate::process::num_cpus();
    if n >= 64 { usize::MAX } else { (1usize << n) - 1 }
}

pub unsafe extern "win64" fn SetThreadGroupAffinity(_h: HANDLE, _a: *const c_void, prev: *mut u8) -> BOOL {
    stub!("thread group affinity not applied");
    if !prev.is_null() {
        std::ptr::write_bytes(prev, 0, 16);
        *(prev as *mut usize) = (1usize << crate::process::num_cpus()) - 1;
    }
    TRUE
}

pub unsafe extern "win64" fn SetThreadIdealProcessor(_h: HANDLE, _p: u32) -> u32 {
    0
}

pub unsafe extern "win64" fn SetThreadIdealProcessorEx(_h: HANDLE, _p: *const c_void, prev: *mut u8) -> BOOL {
    if !prev.is_null() {
        std::ptr::write_bytes(prev, 0, 4);
    }
    TRUE
}

pub unsafe extern "win64" fn SetThreadStackGuarantee(size: *mut u32) -> BOOL {
    if !size.is_null() {
        *size = 0;
    }
    TRUE
}

pub unsafe extern "win64" fn SetThreadDescription(h: HANDLE, name: LPCWSTR) -> HRESULT {
    let n = from_wide(name);
    if let Some(t) = thread_obj(h) {
        log::debug!("thread 0x{:x} description {:?}", t.tid.load(Ordering::Relaxed), n);
        if t.tid.load(Ordering::Relaxed) as u32 == teb::thread_id() {
            if let Ok(c) = std::ffi::CString::new(n.chars().take(63).collect::<String>()) {
                libc::pthread_setname_np(c.as_ptr());
            }
        }
        *t.name.lock().unwrap() = n;
    }
    S_OK
}

pub unsafe extern "win64" fn GetThreadDescription(h: HANDLE, out: *mut *mut u16) -> HRESULT {
    let n = thread_obj(h).map(|t| t.name.lock().unwrap().clone()).unwrap_or_default();
    let w = to_wide_z(&n);
    let p = crate::heap::alloc(w.len() * 2, false) as *mut u16;
    std::ptr::copy_nonoverlapping(w.as_ptr(), p, w.len());
    *out = p;
    S_OK
}

pub unsafe extern "win64" fn QueueUserAPC(f: Option<thread::ApcProc>, h: HANDLE, data: usize) -> u32 {
    match (f, thread_obj(h)) {
        (Some(f), Some(t)) => {
            thread::queue_apc(&t, f, data);
            1
        }
        _ => {
            set_err(err::ERROR_INVALID_HANDLE);
            0
        }
    }
}

pub unsafe extern "win64" fn BaseThreadInitThunk(
    _reserved: u32,
    start: *const c_void,
    param: *mut c_void,
) -> u32 {
    if start.is_null() {
        return 0;
    }
    let code = crate::call_guest8(start as usize, param as u64, 0, 0, 0, 0, 0, 0, 0) as u32;
    thread::exit_current_thread(code)
}

// ---------------------------------------------------------------------------
// TLS / FLS
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn TlsAlloc() -> u32 {
    let i = thread::tls_alloc();
    if i == thread::TLS_OUT_OF_INDEXES {
        set_err(err::ERROR_NO_MORE_ITEMS);
    }
    i
}

pub unsafe extern "win64" fn TlsFree(i: u32) -> BOOL {
    if thread::tls_free(i) {
        TRUE
    } else {
        set_err(err::ERROR_INVALID_PARAMETER);
        FALSE
    }
}

#[inline(never)]
pub unsafe extern "win64" fn TlsGetValue(i: u32) -> *mut c_void {
    match thread::tls_slot_ptr(teb::current(), i, false) {
        Some(p) => {
            set_err(0);
            *p as *mut c_void
        }
        None if (i as usize) < teb::TLS_MINIMUM_AVAILABLE + teb::TLS_EXPANSION_SLOTS => {
            set_err(0);
            std::ptr::null_mut()
        }
        None => {
            set_err(err::ERROR_INVALID_PARAMETER);
            std::ptr::null_mut()
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn TlsSetValue(i: u32, v: *mut c_void) -> BOOL {
    match thread::tls_slot_ptr(teb::current(), i, true) {
        Some(p) => {
            *p = v as usize;
            TRUE
        }
        None => {
            set_err(err::ERROR_INVALID_PARAMETER);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn FlsAlloc(cb: Option<thread::FlsCallback>) -> u32 {
    let i = thread::fls_alloc(cb);
    if i == thread::FLS_OUT_OF_INDEXES {
        set_err(err::ERROR_NO_MORE_ITEMS);
    }
    i
}

pub unsafe extern "win64" fn FlsFree(i: u32) -> BOOL {
    if thread::fls_free(i) {
        TRUE
    } else {
        set_err(err::ERROR_INVALID_PARAMETER);
        FALSE
    }
}

pub unsafe extern "win64" fn FlsGetValue(i: u32) -> *mut c_void {
    if !thread::fls_valid(i) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    set_err(0);
    thread::fls_get(i) as *mut c_void
}

pub unsafe extern "win64" fn FlsSetValue(i: u32, v: *mut c_void) -> BOOL {
    if !thread::fls_valid(i) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    thread::fls_set(i, v as usize);
    TRUE
}

// ---------------------------------------------------------------------------
// Fibers (cooperative user-mode context switching)
// ---------------------------------------------------------------------------
//
// Each created fiber runs on a dedicated coroutine pthread synchronized via a
// (Mutex, Condvar) handoff so only one fiber of a Windows thread executes at a
// time. Using a dedicated OS thread per fiber isolates both the host stack AND
// `x86-runtime`'s per-thread emulated guest stack (`TLS_EMU`), while handing
// off the owning Windows thread's TEB (`gs:0x30/0x58/0x60/0x20`) on every
// `SwitchToFiber` so `GetCurrentThreadId`, `TlsGetValue`, `GetLastError`, and
// inline `gs:[0x20]` (`GetFiberData`) stay identical across all fibers.

enum FiberCmd {
    Idle,
    Resume {
        teb: *mut u8,
        thread_obj: Option<Arc<ThreadObj>>,
        caller: Option<(Arc<FiberSync>, usize)>,
    },
    Exit,
}

unsafe impl Send for FiberCmd {}

struct FiberSync {
    mu: std::sync::Mutex<FiberCmd>,
    cv: std::sync::Condvar,
}

impl FiberSync {
    fn new() -> Self {
        Self { mu: std::sync::Mutex::new(FiberCmd::Idle), cv: std::sync::Condvar::new() }
    }

    fn send(&self, cmd: FiberCmd) {
        let mut g = self.mu.lock().unwrap();
        *g = cmd;
        self.cv.notify_one();
    }

    fn wait(&self) -> FiberCmd {
        let mut g = self.mu.lock().unwrap();
        loop {
            match std::mem::replace(&mut *g, FiberCmd::Idle) {
                FiberCmd::Idle => g = self.cv.wait(g).unwrap(),
                cmd => return cmd,
            }
        }
    }
}

/// Windows fiber control block.
///
/// Field 0 MUST be `data: usize` so inline `GetFiberData()` (`mov rax, gs:[0x20]; mov rax, [rax]`)
/// reads the fiber parameter directly.
#[repr(C)]
struct Fiber {
    data: usize,
    sync: Arc<FiberSync>,
    pthread: Option<libc::pthread_t>,
    stack_base: usize,
    stack_limit: usize,
    /// Most recent fiber that switched into this one, so returning from a fiber
    /// start routine cleanly resumes the caller instead of deadlocking.
    last_caller: Option<(Arc<FiberSync>, usize)>,
}

struct FiberStart {
    sync: Arc<FiberSync>,
    start: usize,
    param: usize,
}

extern "C" fn fiber_thread_main(arg: *mut c_void) -> *mut c_void {
    let start_info = unsafe { Box::from_raw(arg as *mut FiberStart) };
    match start_info.sync.wait() {
        FiberCmd::Exit | FiberCmd::Idle => return std::ptr::null_mut(),
        FiberCmd::Resume { teb, thread_obj, caller } => unsafe {
            let me = libc::pthread_self();
            let hi = libc::pthread_get_stackaddr_np(me) as usize;
            let lo = hi.saturating_sub(libc::pthread_get_stacksize_np(me));
            if !teb.is_null() {
                teb::set_field(teb, teb::off::STACK_BASE, hi);
                teb::set_field(teb, teb::off::STACK_LIMIT, lo);
                let my_fiber = teb::field::<usize>(teb, teb::off::FIBER_DATA) as *mut Fiber;
                if !my_fiber.is_null() {
                    (*my_fiber).stack_base = hi;
                    (*my_fiber).stack_limit = lo;
                    (*my_fiber).last_caller = caller;
                }
                teb::install(teb);
            }
            thread::install_current_thread_context(thread_obj);
            let _ = crate::call_guest8(start_info.start, start_info.param as u64, 0, 0, 0, 0, 0, 0, 0);
            // Fiber start routine returned without calling `SwitchToFiber`:
            // hand TEB + ThreadObj back to the most recent caller fiber so it
            // resumes cleanly instead of deadlocking.
            let cur_teb = teb::current_raw();
            let cur_obj = thread::take_current_thread_context();
            let cur_fiber = if cur_teb.is_null() {
                std::ptr::null_mut()
            } else {
                teb::field::<usize>(cur_teb, teb::off::FIBER_DATA) as *mut Fiber
            };
            let resume_caller = if !cur_fiber.is_null() {
                (*cur_fiber).last_caller.take()
            } else {
                None
            };
            if let Some((caller_sync, caller_ptr)) = resume_caller {
                if !cur_teb.is_null() {
                    let cp = caller_ptr as *mut Fiber;
                    if (*cp).stack_base != 0 {
                        teb::set_field(cur_teb, teb::off::STACK_BASE, (*cp).stack_base);
                        teb::set_field(cur_teb, teb::off::STACK_LIMIT, (*cp).stack_limit);
                    }
                    teb::set_fiber_data(cur_teb, caller_ptr);
                }
                teb::uninstall();
                caller_sync.send(FiberCmd::Resume {
                    teb: cur_teb,
                    thread_obj: cur_obj,
                    caller: None,
                });
            } else {
                teb::uninstall();
            }
        },
    }
    std::ptr::null_mut()
}

fn current_fiber_raw() -> usize {
    let t = teb::current();
    if t.is_null() {
        0
    } else {
        unsafe { teb::field::<usize>(t, teb::off::FIBER_DATA) }
    }
}

pub unsafe extern "win64" fn IsThreadAFiber() -> BOOL {
    (current_fiber_raw() != 0) as BOOL
}

pub unsafe extern "win64" fn ConvertThreadToFiberEx(param: *mut c_void, _flags: u32) -> *mut c_void {
    let t = teb::current();
    if !t.is_null() && teb::field::<usize>(t, teb::off::FIBER_DATA) != 0 {
        set_err(err::ERROR_ALREADY_EXISTS);
        return std::ptr::null_mut();
    }
    let stack_base = if !t.is_null() { teb::field::<usize>(t, teb::off::STACK_BASE) } else { 0 };
    let stack_limit = if !t.is_null() { teb::field::<usize>(t, teb::off::STACK_LIMIT) } else { 0 };
    let fiber = Box::into_raw(Box::new(Fiber {
        data: param as usize,
        sync: Arc::new(FiberSync::new()),
        pthread: None,
        stack_base,
        stack_limit,
        last_caller: None,
    })) as usize;
    if !t.is_null() {
        teb::set_fiber_data(t, fiber);
    }
    fiber as *mut c_void
}

pub unsafe extern "win64" fn ConvertThreadToFiber(param: *mut c_void) -> *mut c_void {
    ConvertThreadToFiberEx(param, 0)
}

pub unsafe extern "win64" fn ConvertFiberToThread() -> BOOL {
    let t = teb::current();
    let f = if t.is_null() { 0 } else { teb::field::<usize>(t, teb::off::FIBER_DATA) };
    if f == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    teb::set_fiber_data(t, 0);
    drop(Box::from_raw(f as *mut Fiber));
    TRUE
}

pub unsafe extern "win64" fn CreateFiberEx(
    commit: usize,
    reserve: usize,
    _flags: u32,
    start: Option<unsafe extern "win64" fn(*mut c_void)>,
    param: *mut c_void,
) -> *mut c_void {
    let Some(start_fn) = start else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    };
    let page = 0x4000usize;
    let want = reserve.max(commit).max(4 * 1024 * 1024);
    let stack_size = (want + page - 1) & !(page - 1);

    let sync = Arc::new(FiberSync::new());
    let arg = Box::into_raw(Box::new(FiberStart {
        sync: Arc::clone(&sync),
        start: start_fn as usize,
        param: param as usize,
    }));

    let mut attr: libc::pthread_attr_t = std::mem::zeroed();
    libc::pthread_attr_init(&mut attr);
    let _ = libc::pthread_attr_setstacksize(&mut attr, stack_size);
    let mut tid: libc::pthread_t = std::mem::zeroed();
    let rc = libc::pthread_create(&mut tid, &attr, fiber_thread_main, arg as *mut c_void);
    libc::pthread_attr_destroy(&mut attr);
    if rc != 0 {
        drop(Box::from_raw(arg));
        set_err(err::ERROR_NOT_ENOUGH_MEMORY);
        return std::ptr::null_mut();
    }

    Box::into_raw(Box::new(Fiber {
        data: param as usize,
        sync,
        pthread: Some(tid),
        stack_base: 0,
        stack_limit: 0,
        last_caller: None,
    })) as *mut c_void
}

pub unsafe extern "win64" fn CreateFiber(
    stack: usize,
    start: Option<unsafe extern "win64" fn(*mut c_void)>,
    param: *mut c_void,
) -> *mut c_void {
    CreateFiberEx(stack, stack, 0, start, param)
}

pub unsafe extern "win64" fn DeleteFiber(fiber: *mut c_void) {
    if !fiber.is_null() {
        let cur = current_fiber_raw();
        if fiber as usize == cur {
            ExitThread(0);
        } else {
            let boxed = Box::from_raw(fiber as *mut Fiber);
            boxed.sync.send(FiberCmd::Exit);
            if let Some(tid) = boxed.pthread {
                let _ = libc::pthread_join(tid, std::ptr::null_mut());
            }
        }
    }
}

pub unsafe extern "win64" fn SwitchToFiber(fiber: *mut c_void) {
    if fiber.is_null() {
        return;
    }
    let t = teb::current();
    let cur = if t.is_null() { 0 } else { teb::field::<usize>(t, teb::off::FIBER_DATA) };
    if cur == fiber as usize {
        return;
    }
    let old_ptr = if cur != 0 {
        cur as *mut Fiber
    } else {
        ConvertThreadToFiberEx(std::ptr::null_mut(), 0) as *mut Fiber
    };
    let new_ptr = fiber as *mut Fiber;
    if !t.is_null() {
        (*old_ptr).stack_base = teb::field::<usize>(t, teb::off::STACK_BASE);
        (*old_ptr).stack_limit = teb::field::<usize>(t, teb::off::STACK_LIMIT);
        if (*new_ptr).stack_base != 0 {
            teb::set_field(t, teb::off::STACK_BASE, (*new_ptr).stack_base);
            teb::set_field(t, teb::off::STACK_LIMIT, (*new_ptr).stack_limit);
        }
        teb::set_fiber_data(t, fiber as usize);
    }
    let old_sync = Arc::clone(&(*old_ptr).sync);
    let new_sync = Arc::clone(&(*new_ptr).sync);
    let thr_obj = thread::take_current_thread_context();
    teb::uninstall();
    new_sync.send(FiberCmd::Resume {
        teb: t,
        thread_obj: thr_obj,
        caller: Some((Arc::clone(&old_sync), old_ptr as usize)),
    });
    match old_sync.wait() {
        FiberCmd::Resume { teb: resumed_teb, thread_obj: resumed_obj, caller } => {
            if caller.is_some() {
                (*old_ptr).last_caller = caller;
            }
            if !resumed_teb.is_null() {
                teb::install(resumed_teb);
            }
            thread::install_current_thread_context(resumed_obj);
        }
        FiberCmd::Exit | FiberCmd::Idle => {
            libc::pthread_exit(std::ptr::null_mut());
        }
    }
}

// ---------------------------------------------------------------------------
// Events, mutexes, semaphores, waitable timers
// ---------------------------------------------------------------------------

fn named_create<T: KObject>(name: Option<String>, make: impl FnOnce() -> Arc<T>) -> HANDLE {
    let Some(n) = name else {
        return handles::insert(make());
    };
    let mut names = handles::named_objects().lock().unwrap();
    if let Some(existing) = names.get(&n) {
        if existing.as_any().is::<T>() {
            let h = handles::insert(existing.clone());
            drop(names);
            set_err(err::ERROR_ALREADY_EXISTS);
            return h;
        }
        drop(names);
        set_err(err::ERROR_INVALID_HANDLE);
        return std::ptr::null_mut();
    }
    let o = make();
    names.insert(n, o.clone());
    drop(names);
    set_err(0);
    handles::insert(o)
}

fn named_open<T: KObject>(name: Option<String>) -> HANDLE {
    let Some(n) = name else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    };
    let names = handles::named_objects().lock().unwrap();
    match names.get(&n) {
        Some(o) if o.as_any().is::<T>() => handles::insert(o.clone()),
        _ => {
            set_err(err::ERROR_FILE_NOT_FOUND);
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn CreateEventW(_s: *mut c_void, manual: BOOL, initial: BOOL, name: LPCWSTR) -> HANDLE {
    let n = from_wide_opt(name);
    named_create(n.clone(), || Arc::new(Event::new(manual != 0, initial != 0, n)))
}
pub unsafe extern "win64" fn CreateEventA(_s: *mut c_void, manual: BOOL, initial: BOOL, name: LPCSTR) -> HANDLE {
    let n = from_ansi_opt(name);
    named_create(n.clone(), || Arc::new(Event::new(manual != 0, initial != 0, n)))
}
const CREATE_EVENT_MANUAL_RESET: u32 = 1;
const CREATE_EVENT_INITIAL_SET: u32 = 2;
pub unsafe extern "win64" fn CreateEventExW(_s: *mut c_void, name: LPCWSTR, flags: u32, _access: u32) -> HANDLE {
    let n = from_wide_opt(name);
    named_create(n.clone(), || {
        Arc::new(Event::new(flags & CREATE_EVENT_MANUAL_RESET != 0, flags & CREATE_EVENT_INITIAL_SET != 0, n))
    })
}
pub unsafe extern "win64" fn CreateEventExA(_s: *mut c_void, name: LPCSTR, flags: u32, _access: u32) -> HANDLE {
    let n = from_ansi_opt(name);
    named_create(n.clone(), || {
        Arc::new(Event::new(flags & CREATE_EVENT_MANUAL_RESET != 0, flags & CREATE_EVENT_INITIAL_SET != 0, n))
    })
}
pub unsafe extern "win64" fn OpenEventW(_a: u32, _i: BOOL, name: LPCWSTR) -> HANDLE {
    named_open::<Event>(from_wide_opt(name))
}
pub unsafe extern "win64" fn OpenEventA(_a: u32, _i: BOOL, name: LPCSTR) -> HANDLE {
    named_open::<Event>(from_ansi_opt(name))
}

fn with_event(h: HANDLE, f: impl FnOnce(&Event)) -> BOOL {
    match handles::get_as::<Event>(h) {
        Some(e) => {
            f(&e);
            TRUE
        }
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn SetEvent(h: HANDLE) -> BOOL {
    with_event(h, |e| e.set())
}
pub unsafe extern "win64" fn ResetEvent(h: HANDLE) -> BOOL {
    with_event(h, |e| e.reset())
}
pub unsafe extern "win64" fn PulseEvent(h: HANDLE) -> BOOL {
    with_event(h, |e| e.pulse())
}

fn new_mutex(owned: bool, name: Option<String>) -> Arc<MutexObj> {
    let tid = teb::thread_id();
    Arc::new(MutexObj { state: handles::Guarded::new(if owned { (tid, 1, false) } else { (0, 0, false) }), name })
}

pub unsafe extern "win64" fn CreateMutexW(_s: *mut c_void, owned: BOOL, name: LPCWSTR) -> HANDLE {
    let n = from_wide_opt(name);
    named_create(n.clone(), || new_mutex(owned != 0, n))
}
pub unsafe extern "win64" fn CreateMutexA(_s: *mut c_void, owned: BOOL, name: LPCSTR) -> HANDLE {
    let n = from_ansi_opt(name);
    named_create(n.clone(), || new_mutex(owned != 0, n))
}
pub unsafe extern "win64" fn CreateMutexExW(_s: *mut c_void, name: LPCWSTR, flags: u32, _a: u32) -> HANDLE {
    let n = from_wide_opt(name);
    named_create(n.clone(), || new_mutex(flags & 1 != 0, n))
}
pub unsafe extern "win64" fn OpenMutexW(_a: u32, _i: BOOL, name: LPCWSTR) -> HANDLE {
    named_open::<MutexObj>(from_wide_opt(name))
}
pub unsafe extern "win64" fn ReleaseMutex(h: HANDLE) -> BOOL {
    match handles::get_as::<MutexObj>(h) {
        Some(m) if m.release(teb::thread_id()) => TRUE,
        Some(_) => {
            set_err(err::ERROR_NOT_OWNER);
            FALSE
        }
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

fn new_sem(initial: i32, max: i32, name: Option<String>) -> Arc<Semaphore> {
    Arc::new(Semaphore::new(initial, max, name))
}

pub unsafe extern "win64" fn CreateSemaphoreW(_s: *mut c_void, initial: i32, max: i32, name: LPCWSTR) -> HANDLE {
    if max <= 0 || initial < 0 || initial > max {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let n = from_wide_opt(name);
    named_create(n.clone(), || new_sem(initial, max, n))
}
pub unsafe extern "win64" fn CreateSemaphoreA(s: *mut c_void, initial: i32, max: i32, name: LPCSTR) -> HANDLE {
    let w = from_ansi_opt(name).map(|n| to_wide_z(&n));
    CreateSemaphoreW(s, initial, max, w.as_ref().map_or(std::ptr::null(), |v| v.as_ptr()))
}
pub unsafe extern "win64" fn CreateSemaphoreExW(s: *mut c_void, initial: i32, max: i32, name: LPCWSTR, _f: u32, _a: u32) -> HANDLE {
    CreateSemaphoreW(s, initial, max, name)
}
pub unsafe extern "win64" fn OpenSemaphoreW(_a: u32, _i: BOOL, name: LPCWSTR) -> HANDLE {
    named_open::<Semaphore>(from_wide_opt(name))
}
pub unsafe extern "win64" fn ReleaseSemaphore(h: HANDLE, n: i32, prev: *mut i32) -> BOOL {
    match handles::get_as::<Semaphore>(h) {
        Some(s) => match s.release(n) {
            Some(p) => {
                if !prev.is_null() {
                    *prev = p;
                }
                TRUE
            }
            None => {
                set_err(err::ERROR_TOO_MANY_POSTS);
                FALSE
            }
        },
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1;

pub unsafe extern "win64" fn CreateWaitableTimerExW(_s: *mut c_void, name: LPCWSTR, flags: u32, _access: u32) -> HANDLE {
    let n = from_wide_opt(name);
    named_create(n.clone(), || Arc::new(handles::WaitableTimer::new(flags & CREATE_WAITABLE_TIMER_MANUAL_RESET != 0, n)))
}

pub unsafe extern "win64" fn CreateWaitableTimerExA(_s: *mut c_void, name: LPCSTR, flags: u32, _access: u32) -> HANDLE {
    let n = from_ansi_opt(name);
    named_create(n.clone(), || Arc::new(handles::WaitableTimer::new(flags & CREATE_WAITABLE_TIMER_MANUAL_RESET != 0, n)))
}

pub unsafe extern "win64" fn CreateWaitableTimerW(s: *mut c_void, manual: BOOL, name: LPCWSTR) -> HANDLE {
    CreateWaitableTimerExW(s, name, if manual != 0 { CREATE_WAITABLE_TIMER_MANUAL_RESET } else { 0 }, 0)
}

pub unsafe extern "win64" fn CreateWaitableTimerA(s: *mut c_void, manual: BOOL, name: LPCSTR) -> HANDLE {
    CreateWaitableTimerExA(s, name, if manual != 0 { CREATE_WAITABLE_TIMER_MANUAL_RESET } else { 0 }, 0)
}

pub unsafe extern "win64" fn OpenWaitableTimerW(_access: u32, _inherit: BOOL, name: LPCWSTR) -> HANDLE {
    named_open::<handles::WaitableTimer>(from_wide_opt(name))
}

struct TimerApcData {
    routine: usize,
    user_arg: usize,
}

unsafe extern "win64" fn timer_apc_adapter(arg: usize) {
    let data = Arc::from_raw(arg as *const TimerApcData);
    let ft = now_filetime();
    crate::call_guest8(
        data.routine,
        data.user_arg as u64,
        ft as u32 as u64,
        (ft >> 32) as u32 as u64,
        0,
        0,
        0,
        0,
        0,
    );
}

struct ScheduledWaitableTimer {
    due: std::time::Instant,
    timer: std::sync::Weak<handles::WaitableTimer>,
    timer_ptr: usize,
    generation: u64,
    period: Option<std::time::Duration>,
    apc: Option<(Arc<TimerApcData>, Arc<ThreadObj>)>,
}

impl PartialEq for ScheduledWaitableTimer {
    fn eq(&self, other: &Self) -> bool {
        self.due == other.due && self.timer_ptr == other.timer_ptr && self.generation == other.generation
    }
}
impl Eq for ScheduledWaitableTimer {}
impl PartialOrd for ScheduledWaitableTimer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ScheduledWaitableTimer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.due.cmp(&self.due) // min-heap by due time
    }
}

struct WaitableTimerSched {
    heap: std::sync::Mutex<std::collections::BinaryHeap<ScheduledWaitableTimer>>,
    cv: std::sync::Condvar,
}

fn waitable_timer_sched() -> &'static WaitableTimerSched {
    static S: std::sync::OnceLock<&'static WaitableTimerSched> = std::sync::OnceLock::new();
    S.get_or_init(|| {
        let s: &'static WaitableTimerSched = Box::leak(Box::new(WaitableTimerSched {
            heap: std::sync::Mutex::new(std::collections::BinaryHeap::new()),
            cv: std::sync::Condvar::new(),
        }));
        std::thread::Builder::new()
            .name("win32-waitable-timer".into())
            .spawn(move || loop {
                let entry = {
                    let mut h = s.heap.lock().unwrap();
                    loop {
                        let now = std::time::Instant::now();
                        match h.peek() {
                            None => h = s.cv.wait(h).unwrap(),
                            Some(e) if e.due > now => {
                                let d = e.due - now;
                                h = s.cv.wait_timeout(h, d).unwrap().0;
                            }
                            Some(_) => break h.pop().unwrap(),
                        }
                    }
                };
                if let Some(timer) = entry.timer.upgrade() {
                    if timer.fire_signaled(entry.generation) {
                        if let Some((ref data, ref target)) = entry.apc {
                            let raw = Arc::into_raw(Arc::clone(data)) as usize;
                            thread::queue_apc(target, timer_apc_adapter, raw);
                        }
                        if let Some(p) = entry.period {
                            let next_due = (entry.due + p).max(std::time::Instant::now());
                            let mut h = s.heap.lock().unwrap();
                            h.push(ScheduledWaitableTimer {
                                due: next_due,
                                timer: entry.timer,
                                timer_ptr: entry.timer_ptr,
                                generation: entry.generation,
                                period: entry.period,
                                apc: entry.apc,
                            });
                            s.cv.notify_one();
                        }
                    }
                }
            })
            .expect("spawn win32-waitable-timer thread");
        s
    })
}

pub unsafe extern "win64" fn SetWaitableTimer(
    h: HANDLE,
    due_time: *const i64,
    period: i32,
    completion_routine: Option<handles::TimerApcRoutine>,
    arg_to_completion: *mut c_void,
    _resume: BOOL,
) -> BOOL {
    if due_time.is_null() || period < 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(timer) = handles::get_as::<handles::WaitableTimer>(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let v = std::ptr::read_unaligned(due_time);
    let delay = if v < 0 {
        std::time::Duration::from_nanos(v.unsigned_abs().saturating_mul(100))
    } else if v == 0 {
        std::time::Duration::ZERO
    } else {
        let now = now_filetime() as i64;
        std::time::Duration::from_nanos(((v - now).max(0) as u64).saturating_mul(100))
    };
    let gen = {
        let g = handles::wait_lock();
        *timer.signaled.get(&g) = delay.is_zero();
        let gen = timer.generation.get(&g);
        *gen += 1;
        *gen
    };
    let apc = completion_routine.and_then(|cb| {
        let target = thread::current_thread_object()?;
        let data = Arc::new(TimerApcData {
            routine: cb as usize,
            user_arg: arg_to_completion as usize,
        });
        Some((data, target))
    });
    let timer_ptr = Arc::as_ptr(&timer) as usize;
    if delay.is_zero() && period == 0 {
        if let Some((ref data, ref target)) = apc {
            let raw = Arc::into_raw(Arc::clone(data)) as usize;
            thread::queue_apc(target, timer_apc_adapter, raw);
        }
        timer.notify();
        return TRUE;
    }
    let period_dur = (period > 0).then(|| std::time::Duration::from_millis(period as u64));
    let s = waitable_timer_sched();
    let mut heap = s.heap.lock().unwrap();
    heap.retain(|e| e.timer_ptr != timer_ptr);
    heap.push(ScheduledWaitableTimer {
        due: std::time::Instant::now() + delay,
        timer: Arc::downgrade(&timer),
        timer_ptr,
        generation: gen,
        period: period_dur,
        apc,
    });
    drop(heap);
    s.cv.notify_one();
    TRUE
}

pub unsafe extern "win64" fn SetWaitableTimerEx(
    h: HANDLE,
    due_time: *const i64,
    period: i32,
    completion_routine: Option<handles::TimerApcRoutine>,
    arg_to_completion: *mut c_void,
    _wake_context: *mut c_void,
    _tolerable_delay: u32,
) -> BOOL {
    SetWaitableTimer(h, due_time, period, completion_routine, arg_to_completion, FALSE)
}

pub unsafe extern "win64" fn CancelWaitableTimer(h: HANDLE) -> BOOL {
    let Some(timer) = handles::get_as::<handles::WaitableTimer>(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    timer.cancel();
    let timer_ptr = Arc::as_ptr(&timer) as usize;
    let s = waitable_timer_sched();
    s.heap.lock().unwrap().retain(|e| e.timer_ptr != timer_ptr);
    TRUE
}

pub unsafe extern "win64" fn CloseHandle(h: HANDLE) -> BOOL {
    if crate::kernel32::file::close_handle_hook(h) || handles::close(h) {
        TRUE
    } else {
        log::debug!("CloseHandle({h:p}): invalid handle");
        set_err(err::ERROR_INVALID_HANDLE);
        FALSE
    }
}

pub unsafe extern "win64" fn DuplicateHandle(
    _sp: HANDLE,
    src: HANDLE,
    _tp: HANDLE,
    out: *mut HANDLE,
    _access: u32,
    _inherit: BOOL,
    options: u32,
) -> BOOL {
    const DUPLICATE_CLOSE_SOURCE: u32 = 1;
    let obj: Option<Arc<dyn KObject>> = if src == handles::CURRENT_THREAD {
        thread::current_thread_object().map(|t| t as Arc<dyn KObject>)
    } else if src == handles::CURRENT_PROCESS {
        if !out.is_null() {
            *out = handles::CURRENT_PROCESS;
        }
        return TRUE;
    } else {
        handles::get(src)
    };
    match obj {
        Some(o) => {
            if !out.is_null() {
                *out = handles::insert(o);
            }
            if options & DUPLICATE_CLOSE_SOURCE != 0 {
                handles::close(src);
            }
            TRUE
        }
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn SetHandleInformation(_h: HANDLE, _m: u32, _f: u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetHandleInformation(_h: HANDLE, f: *mut u32) -> BOOL {
    *f = 0;
    TRUE
}

// ---------------------------------------------------------------------------
// Waiting
// ---------------------------------------------------------------------------

fn resolve_waitables(hs: &[HANDLE]) -> Result<handles::WaitableList, u32> {
    let mut out = handles::WaitableList::with_capacity(hs.len());
    for &h in hs {
        if h == handles::CURRENT_PROCESS {
            return Err(err::ERROR_INVALID_HANDLE);
        }
        out.push(handles::get(h).ok_or(err::ERROR_INVALID_HANDLE)?);
    }
    Ok(out)
}

fn wait_result_code(r: WaitResult) -> u32 {
    match r {
        WaitResult::Object(i) => WAIT_OBJECT_0 + i as u32,
        WaitResult::Abandoned(i) => WAIT_ABANDONED_0 + i as u32,
        WaitResult::Timeout => WAIT_TIMEOUT,
        WaitResult::IoCompletion => WAIT_IO_COMPLETION,
        WaitResult::Failed(e) => {
            set_err(e);
            WAIT_FAILED
        }
    }
}

pub unsafe extern "win64" fn WaitForSingleObjectEx(h: HANDLE, ms: u32, alertable: BOOL) -> u32 {
    if h == handles::CURRENT_PROCESS {
        // Waiting on our own process never completes.
        if ms == INFINITE {
            loop {
                std::thread::park();
            }
        }
        crate::kernel32::time::sleep_ms(ms);
        return WAIT_TIMEOUT;
    }
    match handles::get(h) {
        Some(obj) => {
            if alertable == 0 {
                if let Some(s) = obj.as_any().downcast_ref::<Semaphore>() {
                    if s.try_take() {
                        return WAIT_OBJECT_0;
                    }
                } else if let Some(e) = obj.as_any().downcast_ref::<Event>() {
                    if e.try_take() {
                        return WAIT_OBJECT_0;
                    }
                }
            }
            let t0 = if ms != 0 { crate::hitch::span_begin() } else { 0 };
            let r = wait_result_code(handles::wait_for(std::slice::from_ref(&obj), false, ms, alertable != 0));
            crate::hitch::span_end(t0, crate::hitch::SPAN_OBJ_WAIT);
            r
        }
        None => {
            log::debug!("WaitForSingleObject on invalid handle {h:p}");
            set_err(err::ERROR_INVALID_HANDLE);
            WAIT_FAILED
        }
    }
}

pub unsafe extern "win64" fn WaitForSingleObject(h: HANDLE, ms: u32) -> u32 {
    WaitForSingleObjectEx(h, ms, FALSE)
}

pub unsafe extern "win64" fn WaitForMultipleObjectsEx(n: u32, hs: *const HANDLE, all: BOOL, ms: u32, alertable: BOOL) -> u32 {
    if n == 0 || n > 64 || hs.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return WAIT_FAILED;
    }
    let hs = std::slice::from_raw_parts(hs, n as usize);
    match resolve_waitables(hs) {
        Ok(objs) => {
            let t0 = if ms != 0 { crate::hitch::span_begin() } else { 0 };
            let r = wait_result_code(handles::wait_for(objs.as_slice(), all != 0, ms, alertable != 0));
            crate::hitch::span_end(t0, crate::hitch::SPAN_OBJ_WAIT);
            r
        }
        Err(e) => {
            set_err(e);
            WAIT_FAILED
        }
    }
}

pub unsafe extern "win64" fn WaitForMultipleObjects(n: u32, hs: *const HANDLE, all: BOOL, ms: u32) -> u32 {
    WaitForMultipleObjectsEx(n, hs, all, ms, FALSE)
}

pub unsafe extern "win64" fn SignalObjectAndWait(sig: HANDLE, wait: HANDLE, ms: u32, alertable: BOOL) -> u32 {
    if let Some(e) = handles::get_as::<Event>(sig) {
        e.set();
    } else if let Some(m) = handles::get_as::<MutexObj>(sig) {
        m.release(teb::thread_id());
    } else if let Some(s) = handles::get_as::<Semaphore>(sig) {
        s.release(1);
    } else {
        set_err(err::ERROR_INVALID_HANDLE);
        return WAIT_FAILED;
    }
    WaitForSingleObjectEx(wait, ms, alertable)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PingPong {
        main_fiber: *mut c_void,
        expected_tid: u32,
        steps: Vec<u32>,
    }

    unsafe extern "win64" fn worker_fiber(param: *mut c_void) {
        let st = &mut *(param as *mut PingPong);
        assert_eq!(GetCurrentThreadId(), st.expected_tid);
        assert_eq!(IsThreadAFiber(), TRUE);
        st.steps.push(2);
        SwitchToFiber(st.main_fiber);
        assert_eq!(GetCurrentThreadId(), st.expected_tid);
        st.steps.push(4);
        SwitchToFiber(st.main_fiber);
    }

    #[test]
    fn cooperative_fiber_ping_pong_preserves_teb() {
        std::thread::spawn(|| unsafe {
            let teb_buf = [0u8; 0x2000];
            let tid = teb::alloc_tid();
            let _ = teb::init_thread(
                teb_buf.as_ptr() as usize + teb_buf.len(),
                teb_buf.as_ptr() as usize,
                tid,
            );
            assert_eq!(IsThreadAFiber(), FALSE);
            let main_f = ConvertThreadToFiber(0xCAFE as *mut c_void);
            assert!(!main_f.is_null());
            assert_eq!(IsThreadAFiber(), TRUE);
            assert_eq!(*(main_f as *const usize), 0xCAFE);

            let mut state = PingPong { main_fiber: main_f, expected_tid: tid, steps: Vec::new() };
            let child = CreateFiber(0, Some(worker_fiber), &mut state as *mut PingPong as *mut c_void);
            assert!(!child.is_null());

            state.steps.push(1);
            SwitchToFiber(child);
            assert_eq!(GetCurrentThreadId(), tid);
            state.steps.push(3);
            SwitchToFiber(child);
            assert_eq!(GetCurrentThreadId(), tid);
            state.steps.push(5);

            assert_eq!(state.steps, vec![1, 2, 3, 4, 5]);
            DeleteFiber(child);
            assert_eq!(ConvertFiberToThread(), TRUE);
            assert_eq!(IsThreadAFiber(), FALSE);
            teb::uninstall();
        })
        .join()
        .unwrap();
    }

    #[test]
    fn get_and_set_thread_context_report_clean_debug_regs() {
        unsafe {
            let mut ctx = crate::unwind::Context::zeroed();
            ctx.context_flags = 0x0010_0010; // CONTEXT_AMD64 | CONTEXT_DEBUG_REGISTERS
            ctx.dr = [0xDEAD; 6];
            let cur = GetCurrentThread();
            assert_eq!(GetThreadContext(cur, &mut ctx as *mut _ as *mut c_void), TRUE);
            assert_eq!(ctx.context_flags, 0x0010_0010);
            assert_eq!(ctx.dr, [0; 6]);
            assert_eq!(SetThreadContext(cur, &ctx as *const _ as *const c_void), TRUE);
            assert_eq!(GetThreadContext(cur, std::ptr::null_mut()), FALSE);
            assert_eq!(GetThreadContext(0x1234_5678usize as HANDLE, &mut ctx as *mut _ as *mut c_void), FALSE);
        }
    }

    #[test]
    fn suspend_and_resume_live_thread_with_nested_counts() {
        use std::sync::atomic::{AtomicBool, AtomicU64};
        use std::time::{Duration, Instant};

        let counter = Arc::new(AtomicU64::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let c_worker = Arc::clone(&counter);
        let s_worker = Arc::clone(&stop);

        let obj = thread::spawn(
            Box::new(move || {
                while !s_worker.load(Ordering::Relaxed) {
                    c_worker.fetch_add(1, Ordering::Relaxed);
                    std::hint::spin_loop();
                }
                42
            }),
            64 * 1024,
            false,
            false,
        )
        .expect("spawn spinning worker");
        let h = handles::insert(obj);

        unsafe {
            // Wait until the worker has cleared its start gate and is actively spinning.
            let deadline = Instant::now() + Duration::from_secs(2);
            while counter.load(Ordering::Acquire) == 0 {
                assert!(Instant::now() < deadline, "worker did not start spinning");
                std::thread::sleep(Duration::from_millis(1));
            }

            // First suspend transitions 0 -> 1 (Mach thread_suspend).
            assert_eq!(SuspendThread(h), 0);
            // Nested suspend transitions 1 -> 2 (must not double-suspend Mach port).
            assert_eq!(SuspendThread(h), 1);

            let paused_at = counter.load(Ordering::SeqCst);
            std::thread::sleep(Duration::from_millis(30));
            assert_eq!(
                counter.load(Ordering::SeqCst),
                paused_at,
                "live thread kept running while suspended (count = 2)"
            );

            // First resume transitions 2 -> 1; thread must remain suspended.
            assert_eq!(ResumeThread(h), 2);
            std::thread::sleep(Duration::from_millis(20));
            assert_eq!(
                counter.load(Ordering::SeqCst),
                paused_at,
                "live thread resumed early while suspend count was still 1"
            );

            // Second resume transitions 1 -> 0 (Mach thread_resume).
            assert_eq!(ResumeThread(h), 1);
            // Extra resume on a running thread returns 0 without underflowing Mach.
            assert_eq!(ResumeThread(h), 0);

            let deadline = Instant::now() + Duration::from_secs(2);
            while counter.load(Ordering::Acquire) == paused_at {
                assert!(Instant::now() < deadline, "worker did not resume after ResumeThread");
                std::thread::sleep(Duration::from_millis(1));
            }

            stop.store(true, Ordering::Release);
            assert_eq!(WaitForSingleObject(h, 5_000), WAIT_OBJECT_0);
            let mut exit_code = 0u32;
            assert_eq!(GetExitCodeThread(h, &mut exit_code), TRUE);
            assert_eq!(exit_code, 42);

            // Exited thread cannot be suspended, and ResumeThread safely returns 0.
            assert_eq!(SuspendThread(h), u32::MAX);
            assert_eq!(ResumeThread(h), 0);
            assert_eq!(CloseHandle(h), TRUE);
        }
    }

    #[test]
    fn create_suspended_gate_and_self_suspend_do_not_deadlock() {
        use std::sync::atomic::{AtomicBool, AtomicU32};
        use std::time::{Duration, Instant};

        let stage = Arc::new(AtomicU32::new(0));
        let resumed_self = Arc::new(AtomicBool::new(false));
        let st_worker = Arc::clone(&stage);
        let rs_worker = Arc::clone(&resumed_self);

        let obj = thread::spawn(
            Box::new(move || unsafe {
                st_worker.store(1, Ordering::Release);
                // Self-suspend via CURRENT_THREAD pseudo-handle must not deadlock on mutex.
                let prev = SuspendThread(GetCurrentThread());
                assert_eq!(prev, 0);
                rs_worker.store(true, Ordering::Release);
                7
            }),
            64 * 1024,
            true,
            false,
        )
        .expect("spawn CREATE_SUSPENDED worker");
        let h = handles::insert(obj);

        unsafe {
            // Thread created suspended must stay blocked at start gate.
            std::thread::sleep(Duration::from_millis(20));
            assert_eq!(stage.load(Ordering::Acquire), 0);

            // Nested suspend before start: 1 -> 2.
            assert_eq!(SuspendThread(h), 1);
            assert_eq!(ResumeThread(h), 2);
            std::thread::sleep(Duration::from_millis(15));
            assert_eq!(stage.load(Ordering::Acquire), 0);

            // Final pre-start resume: 1 -> 0 releases start gate.
            assert_eq!(ResumeThread(h), 1);
            let deadline = Instant::now() + Duration::from_secs(2);
            while stage.load(Ordering::Acquire) == 0 {
                assert!(Instant::now() < deadline, "CREATE_SUSPENDED worker did not start");
                std::thread::sleep(Duration::from_millis(1));
            }

            // Wait briefly for worker to enter self-suspend, then resume it.
            let deadline = Instant::now() + Duration::from_secs(2);
            loop {
                if ResumeThread(h) == 1 {
                    break;
                }
                assert!(Instant::now() < deadline, "worker did not enter self-suspend");
                std::thread::sleep(Duration::from_millis(1));
            }

            assert_eq!(WaitForSingleObject(h, 5_000), WAIT_OBJECT_0);
            assert!(resumed_self.load(Ordering::Acquire));
            assert_eq!(CloseHandle(h), TRUE);
        }
    }

    #[test]
    fn fiber_return_resumes_caller_and_preserves_thread_context() {
        use std::sync::atomic::AtomicBool;

        struct ReturnState {
            expected_tid: u32,
            ran: AtomicBool,
            saw_thread_obj: AtomicBool,
        }
        unsafe extern "win64" fn returning_fiber(param: *mut c_void) {
            let st = &*(param as *const ReturnState);
            assert_eq!(GetCurrentThreadId(), st.expected_tid);
            assert_eq!(IsThreadAFiber(), TRUE);
            st.saw_thread_obj.store(thread::current_thread_object().is_some(), Ordering::Release);
            st.ran.store(true, Ordering::Release);
            // Return directly without calling SwitchToFiber: caller must resume cleanly.
        }

        let obj = thread::spawn(
            Box::new(|| unsafe {
                let tid = GetCurrentThreadId();
                assert!(thread::current_thread_object().is_some());
                let main_f = ConvertThreadToFiber(0xBEEF as *mut c_void);
                assert!(!main_f.is_null());

                let state = ReturnState {
                    expected_tid: tid,
                    ran: AtomicBool::new(false),
                    saw_thread_obj: AtomicBool::new(false),
                };
                let child = CreateFiber(0, Some(returning_fiber), &state as *const ReturnState as *mut c_void);
                assert!(!child.is_null());

                SwitchToFiber(child);

                assert!(state.ran.load(Ordering::Acquire));
                assert!(state.saw_thread_obj.load(Ordering::Acquire));
                assert_eq!(GetCurrentThreadId(), tid);
                assert!(thread::current_thread_object().is_some());
                assert_eq!(IsThreadAFiber(), TRUE);

                DeleteFiber(child);
                assert_eq!(ConvertFiberToThread(), TRUE);
                assert_eq!(IsThreadAFiber(), FALSE);
                123
            }),
            64 * 1024,
            false,
            false,
        )
        .expect("spawn fiber test thread");
        let h = handles::insert(obj);
        unsafe {
            assert_eq!(WaitForSingleObject(h, 5_000), WAIT_OBJECT_0);
            let mut code = 0u32;
            assert_eq!(GetExitCodeThread(h, &mut code), TRUE);
            assert_eq!(code, 123);
            assert_eq!(CloseHandle(h), TRUE);
        }
    }

    #[test]
    fn waitable_timer_shared_scheduler_and_apc() {
        use std::sync::atomic::AtomicUsize;

        unsafe extern "win64" fn on_timer_apc(arg: *mut c_void, _lo: u32, _hi: u32) {
            let slot = &*(arg as *const AtomicUsize);
            slot.fetch_add(1, Ordering::AcqRel);
        }

        let obj = thread::spawn(
            Box::new(|| unsafe {
                let h = CreateWaitableTimerW(std::ptr::null_mut(), TRUE, std::ptr::null());
                assert!(!h.is_null());

                // Arm then cancel before expiration -> must time out.
                let due_cancel = -500_000i64; // 50 ms
                assert_eq!(SetWaitableTimer(h, &due_cancel, 0, None, std::ptr::null_mut(), FALSE), TRUE);
                assert_eq!(CancelWaitableTimer(h), TRUE);
                assert_eq!(WaitForSingleObject(h, 15), WAIT_TIMEOUT);

                // Re-arm with APC routine (15 ms relative) and enter alertable sleep.
                let hits = AtomicUsize::new(0);
                let due_fire = -150_000i64; // 15 ms
                assert_eq!(
                    SetWaitableTimer(
                        h,
                        &due_fire,
                        0,
                        Some(on_timer_apc),
                        &hits as *const AtomicUsize as *mut c_void,
                        FALSE,
                    ),
                    TRUE
                );
                assert!(handles::alertable_sleep(5_000), "alertable sleep should wake for timer APC");
                assert_eq!(hits.load(Ordering::Acquire), 1);
                assert_eq!(WaitForSingleObject(h, 0), WAIT_OBJECT_0);
                assert_eq!(CloseHandle(h), TRUE);
                0
            }),
            64 * 1024,
            false,
            false,
        )
        .expect("spawn waitable timer test thread");
        let h = handles::insert(obj);
        unsafe {
            assert_eq!(WaitForSingleObject(h, 5_000), WAIT_OBJECT_0);
            assert_eq!(CloseHandle(h), TRUE);
        }
    }

    #[test]
    fn fls_lockfree_alloc_get_set_free() {
        use std::sync::atomic::AtomicUsize;

        unsafe extern "win64" fn fls_dtor(val: *mut c_void) {
            let slot = &*(val as *const AtomicUsize);
            slot.fetch_add(1, Ordering::AcqRel);
        }

        unsafe {
            let called = AtomicUsize::new(0);
            let idx = FlsAlloc(Some(fls_dtor));
            assert_ne!(idx, thread::FLS_OUT_OF_INDEXES);
            assert_eq!(FlsSetValue(idx, &called as *const AtomicUsize as *mut c_void), TRUE);
            assert_eq!(FlsGetValue(idx), &called as *const AtomicUsize as *mut c_void);
            assert_eq!(FlsFree(idx), TRUE);
            assert_eq!(called.load(Ordering::Acquire), 1);
            assert!(FlsGetValue(idx).is_null());
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
        }
    }
}
