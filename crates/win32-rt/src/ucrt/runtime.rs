//! CRT startup/shutdown: initializers, onexit tables, argv/environ, exit,
//! signals, floating-point control, threads, invalid-parameter handling.

use super::errno::{self, EINVAL};
use crate::heap;
use crate::process as pr;
use crate::strings::*;
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

type Pvfv = unsafe extern "win64" fn();
type Pifv = unsafe extern "win64" fn() -> i32;

// ---------------------------------------------------------------------------
// Initializers
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn _initterm(mut b: *const Option<Pvfv>, e: *const Option<Pvfv>) {
    while b < e {
        if let Some(f) = *b {
            crate::call_guest8(f as usize, 0, 0, 0, 0, 0, 0, 0, 0);
        }
        b = b.add(1);
    }
}

pub unsafe extern "win64" fn _initterm_e(mut b: *const Option<Pifv>, e: *const Option<Pifv>) -> i32 {
    while b < e {
        if let Some(f) = *b {
            let r = crate::call_guest8(f as usize, 0, 0, 0, 0, 0, 0, 0, 0) as i32;
            if r != 0 {
                log::warn!("_initterm_e: initializer {:p} returned {r}", f as *const ());
                return r;
            }
        }
        b = b.add(1);
    }
    0
}

// ---------------------------------------------------------------------------
// onexit tables. Pointers are stored raw (only ucrt code touches them).
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct OnexitTable {
    first: *mut usize,
    last: *mut usize,
    end: *mut usize,
}

static TABLE_LOCK: Mutex<()> = Mutex::new(());

pub unsafe extern "win64" fn _initialize_onexit_table(t: *mut OnexitTable) -> i32 {
    if t.is_null() {
        return -1;
    }
    if !(*t).first.is_null() {
        return 0; // already initialized
    }
    (*t).first = std::ptr::null_mut();
    (*t).last = std::ptr::null_mut();
    (*t).end = std::ptr::null_mut();
    0
}

pub unsafe extern "win64" fn _register_onexit_function(t: *mut OnexitTable, f: usize) -> i32 {
    if t.is_null() {
        return -1;
    }
    let _g = TABLE_LOCK.lock().unwrap();
    let t = &mut *t;
    if t.last == t.end {
        let count = if t.first.is_null() { 0 } else { t.end.offset_from(t.first) as usize };
        let new_count = (count * 2).max(32);
        let used = if t.first.is_null() { 0 } else { t.last.offset_from(t.first) as usize };
        let p = heap::realloc(t.first as *mut c_void, new_count * 8, false, false) as *mut usize;
        if p.is_null() {
            return -1;
        }
        t.first = p;
        t.last = p.add(used);
        t.end = p.add(new_count);
    }
    *t.last = f;
    t.last = t.last.add(1);
    0
}

pub unsafe extern "win64" fn _execute_onexit_table(t: *mut OnexitTable) -> i32 {
    if t.is_null() {
        return -1;
    }
    let (first, last) = {
        let _g = TABLE_LOCK.lock().unwrap();
        let r = ((*t).first, (*t).last);
        (*t).first = std::ptr::null_mut();
        (*t).last = std::ptr::null_mut();
        (*t).end = std::ptr::null_mut();
        r
    };
    if first.is_null() {
        return 0;
    }
    let mut p = last;
    while p > first {
        p = p.sub(1);
        let f = *p;
        if f != 0 {
            crate::call_guest8(f, 0, 0, 0, 0, 0, 0, 0, 0);
        }
    }
    heap::free(first as *mut c_void);
    0
}

struct Global(std::cell::UnsafeCell<OnexitTable>);
unsafe impl Sync for Global {}
static ATEXIT: Global = Global(std::cell::UnsafeCell::new(OnexitTable {
    first: std::ptr::null_mut(),
    last: std::ptr::null_mut(),
    end: std::ptr::null_mut(),
}));
static QUICK_EXIT: Global = Global(std::cell::UnsafeCell::new(OnexitTable {
    first: std::ptr::null_mut(),
    last: std::ptr::null_mut(),
    end: std::ptr::null_mut(),
}));

pub unsafe extern "win64" fn _crt_atexit(f: usize) -> i32 {
    _register_onexit_function(ATEXIT.0.get(), f)
}
pub unsafe extern "win64" fn atexit(f: usize) -> i32 {
    _register_onexit_function(ATEXIT.0.get(), f)
}
pub unsafe extern "win64" fn _crt_at_quick_exit(f: usize) -> i32 {
    _register_onexit_function(QUICK_EXIT.0.get(), f)
}
pub unsafe extern "win64" fn at_quick_exit(f: usize) -> i32 {
    _register_onexit_function(QUICK_EXIT.0.get(), f)
}

static TLS_EXIT_CB: AtomicUsize = AtomicUsize::new(0);

pub unsafe extern "win64" fn _register_thread_local_exe_atexit_callback(cb: usize) {
    TLS_EXIT_CB.store(cb, Ordering::SeqCst);
}

static EXITING: AtomicI32 = AtomicI32::new(0);

/// `exit`/`_cexit` common path: thread_local dtors, atexit, stdio flush.
unsafe fn run_exit_handlers() {
    if EXITING.swap(1, Ordering::SeqCst) != 0 {
        return;
    }
    let cb = TLS_EXIT_CB.swap(0, Ordering::SeqCst);
    if cb != 0 {
        crate::call_guest8(cb, 0, 0 /* DLL_PROCESS_DETACH */, 0, 0, 0, 0, 0, 0);
    }
    _execute_onexit_table(ATEXIT.0.get());
    super::stdio::flush_all();
}

pub unsafe extern "win64" fn exit(code: i32) -> ! {
    log::info!("exit({code})");
    run_exit_handlers();
    pr::exit_process(code as u32)
}
pub unsafe extern "win64" fn _exit(code: i32) -> ! {
    pr::exit_process(code as u32)
}
pub unsafe extern "win64" fn _Exit(code: i32) -> ! {
    pr::exit_process(code as u32)
}
pub unsafe extern "win64" fn quick_exit(code: i32) -> ! {
    _execute_onexit_table(QUICK_EXIT.0.get());
    pr::exit_process(code as u32)
}
pub unsafe extern "win64" fn _cexit() {
    run_exit_handlers();
}
pub unsafe extern "win64" fn _c_exit() {}

// ---------------------------------------------------------------------------
// argv / environ / command line
// ---------------------------------------------------------------------------

struct Args {
    argc: i32,
    argv: *mut *mut u8,
    wargv: *mut *mut u16,
}
unsafe impl Send for Args {}
unsafe impl Sync for Args {}

fn args() -> &'static Args {
    static A: OnceLock<Args> = OnceLock::new();
    A.get_or_init(|| {
        let v = pr::split_cmdline(&pr::config().cmdline);
        let mut a: Vec<*mut u8> = v.iter().map(|s| leak_ansi_z(s)).collect();
        a.push(std::ptr::null_mut());
        let mut w: Vec<*mut u16> = v.iter().map(|s| leak_wide_z(s)).collect();
        w.push(std::ptr::null_mut());
        Args {
            argc: v.len() as i32,
            argv: Box::leak(a.into_boxed_slice()).as_mut_ptr(),
            wargv: Box::leak(w.into_boxed_slice()).as_mut_ptr(),
        }
    })
}

struct Slot<T>(std::cell::UnsafeCell<T>);
unsafe impl<T> Sync for Slot<T> {}

static ARGC: Slot<i32> = Slot(std::cell::UnsafeCell::new(0));
static ARGV: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));
static WARGV: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));
static ENVIRON: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));
static WENVIRON: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));
static INITIAL_ENVIRON: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));
static INITIAL_WENVIRON: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));
static INITIAL_ENV_GEN: AtomicU32 = AtomicU32::new(u32::MAX);
static MUTABLE_ENV_INIT_GEN: AtomicU32 = AtomicU32::new(u32::MAX);
static ENV_TABLES_LOCK: Mutex<()> = Mutex::new(());
static ACMDLN: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));
static WCMDLN: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));
static PGMPTR: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));
static WPGMPTR: Slot<usize> = Slot(std::cell::UnsafeCell::new(0));

unsafe fn ensure_args() {
    let a = args();
    *ARGC.0.get() = a.argc;
    *ARGV.0.get() = a.argv as usize;
    *WARGV.0.get() = a.wargv as usize;
    *ACMDLN.0.get() = pr::cmdline_a() as usize;
    *WCMDLN.0.get() = pr::cmdline_w() as usize;
    if *PGMPTR.0.get() == 0 {
        *PGMPTR.0.get() = leak_ansi_z(&pr::config().exe_path) as usize;
        *WPGMPTR.0.get() = leak_wide_z(&pr::config().exe_path) as usize;
    }
}

fn build_narrow_env_table(vars: &[(String, String)]) -> *mut *mut u8 {
    let mut a: Vec<*mut u8> = vars.iter().map(|(k, v)| leak_ansi_z(&format!("{k}={v}"))).collect();
    a.push(std::ptr::null_mut());
    Box::leak(a.into_boxed_slice()).as_mut_ptr()
}

fn build_wide_env_table(vars: &[(String, String)]) -> *mut *mut u16 {
    let mut w: Vec<*mut u16> = vars.iter().map(|(k, v)| leak_wide_z(&format!("{k}={v}"))).collect();
    w.push(std::ptr::null_mut());
    Box::leak(w.into_boxed_slice()).as_mut_ptr()
}

unsafe fn ensure_initial_environ_locked() {
    let init_gen = pr::env_init_gen();
    if INITIAL_ENV_GEN.load(Ordering::Acquire) == init_gen
        && *INITIAL_ENVIRON.0.get() != 0
        && *INITIAL_WENVIRON.0.get() != 0
    {
        return;
    }
    let vars = pr::initial_env_vars();
    *INITIAL_ENVIRON.0.get() = build_narrow_env_table(&vars) as usize;
    *INITIAL_WENVIRON.0.get() = build_wide_env_table(&vars) as usize;
    INITIAL_ENV_GEN.store(init_gen, Ordering::Release);
}

unsafe fn rebuild_environ_locked() {
    let vars = pr::env_vars();
    *ENVIRON.0.get() = build_narrow_env_table(&vars) as usize;
    *WENVIRON.0.get() = build_wide_env_table(&vars) as usize;
    MUTABLE_ENV_INIT_GEN.store(pr::env_init_gen(), Ordering::Release);
}

/// Rebuild the mutable CRT `environ` arrays from the process environment.
/// Preserves the immutable initial environment snapshots separately.
pub unsafe fn rebuild_environ() {
    let _g = ENV_TABLES_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    rebuild_environ_locked();
}

pub unsafe extern "win64" fn _configure_narrow_argv(_mode: i32) -> i32 {
    ensure_args();
    0
}
pub unsafe extern "win64" fn _configure_wide_argv(_mode: i32) -> i32 {
    ensure_args();
    0
}
pub unsafe extern "win64" fn _initialize_narrow_environment() -> i32 {
    let _g = ENV_TABLES_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ensure_initial_environ_locked();
    rebuild_environ_locked();
    0
}
pub unsafe extern "win64" fn _initialize_wide_environment() -> i32 {
    let _g = ENV_TABLES_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ensure_initial_environ_locked();
    rebuild_environ_locked();
    0
}
pub unsafe extern "win64" fn _get_initial_narrow_environment() -> *mut *mut u8 {
    let _g = ENV_TABLES_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ensure_initial_environ_locked();
    *INITIAL_ENVIRON.0.get() as *mut *mut u8
}
pub unsafe extern "win64" fn _get_initial_wide_environment() -> *mut *mut u16 {
    let _g = ENV_TABLES_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ensure_initial_environ_locked();
    *INITIAL_WENVIRON.0.get() as *mut *mut u16
}
pub unsafe extern "win64" fn __p___argc() -> *mut i32 {
    ensure_args();
    ARGC.0.get()
}
pub unsafe extern "win64" fn __p___argv() -> *mut usize {
    ensure_args();
    ARGV.0.get()
}
pub unsafe extern "win64" fn __p___wargv() -> *mut usize {
    ensure_args();
    WARGV.0.get()
}
pub unsafe extern "win64" fn __p__environ() -> *mut usize {
    let _g = ENV_TABLES_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let init_gen = pr::env_init_gen();
    if *ENVIRON.0.get() == 0 || MUTABLE_ENV_INIT_GEN.load(Ordering::Acquire) != init_gen {
        ensure_initial_environ_locked();
        rebuild_environ_locked();
    }
    ENVIRON.0.get()
}
pub unsafe extern "win64" fn __p__wenviron() -> *mut usize {
    let _g = ENV_TABLES_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let init_gen = pr::env_init_gen();
    if *WENVIRON.0.get() == 0 || MUTABLE_ENV_INIT_GEN.load(Ordering::Acquire) != init_gen {
        ensure_initial_environ_locked();
        rebuild_environ_locked();
    }
    WENVIRON.0.get()
}
pub unsafe extern "win64" fn __p__acmdln() -> *mut usize {
    ensure_args();
    ACMDLN.0.get()
}
pub unsafe extern "win64" fn __p__wcmdln() -> *mut usize {
    ensure_args();
    WCMDLN.0.get()
}
pub unsafe extern "win64" fn __p__pgmptr() -> *mut usize {
    ensure_args();
    PGMPTR.0.get()
}
pub unsafe extern "win64" fn __p__wpgmptr() -> *mut usize {
    ensure_args();
    WPGMPTR.0.get()
}
pub unsafe extern "win64" fn _get_pgmptr(out: *mut usize) -> i32 {
    ensure_args();
    *out = *PGMPTR.0.get();
    0
}
pub unsafe extern "win64" fn _get_wpgmptr(out: *mut usize) -> i32 {
    ensure_args();
    *out = *WPGMPTR.0.get();
    0
}

/// Skip argv[0] (quoted or not) and following blanks, like the CRT.
fn skip_program_name<T: Copy + Into<u32>>(s: &[T]) -> usize {
    let mut i = 0;
    let mut inq = false;
    while i < s.len() {
        let c: u32 = s[i].into();
        if c == 0 {
            return i;
        }
        if c == b'"' as u32 {
            inq = !inq;
        } else if !inq && (c == b' ' as u32 || c == b'\t' as u32) {
            break;
        }
        i += 1;
    }
    while i < s.len() {
        let c: u32 = s[i].into();
        if c == b' ' as u32 || c == b'\t' as u32 {
            i += 1;
        } else {
            break;
        }
    }
    i
}

pub unsafe extern "win64" fn _get_narrow_winmain_command_line() -> *mut u8 {
    let p = pr::cmdline_a();
    let s = std::slice::from_raw_parts(p, strlen(p) + 1);
    p.add(skip_program_name(s))
}
pub unsafe extern "win64" fn _get_wide_winmain_command_line() -> *mut u16 {
    let p = pr::cmdline_w();
    let s = std::slice::from_raw_parts(p, wcslen(p) + 1);
    p.add(skip_program_name(s))
}

// ---------------------------------------------------------------------------
// App type / error mode / misc no-ops
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn _set_app_type(_t: i32) {}
static ERROR_MODE: AtomicI32 = AtomicI32::new(0);
pub unsafe extern "win64" fn _set_error_mode(m: i32) -> i32 {
    if m == 3 {
        return ERROR_MODE.load(Ordering::Relaxed);
    }
    ERROR_MODE.swap(m, Ordering::Relaxed)
}
pub unsafe extern "win64" fn __setusermatherr(_f: *const c_void) {}
pub unsafe extern "win64" fn _set_abort_behavior(flags: u32, mask: u32) -> u32 {
    static B: AtomicUsize = AtomicUsize::new(1);
    let old = B.load(Ordering::Relaxed) as u32;
    B.store(((old & !mask) | (flags & mask)) as usize, Ordering::Relaxed);
    old
}
pub unsafe extern "win64" fn _getpid() -> i32 {
    std::process::id() as i32
}

// ---------------------------------------------------------------------------
// Invalid parameter / assert / abort / terminate
// ---------------------------------------------------------------------------

static INVALID_PARAM_HANDLER: AtomicUsize = AtomicUsize::new(0);
static TERMINATE_HANDLER: AtomicUsize = AtomicUsize::new(0);

pub unsafe extern "win64" fn _set_invalid_parameter_handler(h: usize) -> usize {
    INVALID_PARAM_HANDLER.swap(h, Ordering::SeqCst)
}
pub unsafe extern "win64" fn _get_invalid_parameter_handler() -> usize {
    INVALID_PARAM_HANDLER.load(Ordering::SeqCst)
}
pub unsafe extern "win64" fn _set_thread_local_invalid_parameter_handler(h: usize) -> usize {
    INVALID_PARAM_HANDLER.swap(h, Ordering::SeqCst)
}
pub unsafe extern "win64" fn _get_thread_local_invalid_parameter_handler() -> usize {
    INVALID_PARAM_HANDLER.load(Ordering::SeqCst)
}

/// What ucrt functions call on bad arguments: invoke the user handler if
/// any (and return), else terminate like Windows (fast-fail).
pub fn invalid_parameter(what: &str) {
    let h = INVALID_PARAM_HANDLER.load(Ordering::SeqCst);
    if h != 0 {
        log::warn!("CRT invalid parameter ({what}); calling user handler");
        unsafe {
            let f: unsafe extern "win64" fn(*const u16, *const u16, *const u16, u32, usize) = std::mem::transmute(h);
            f(std::ptr::null(), std::ptr::null(), std::ptr::null(), 0, 0);
        }
        return;
    }
    log::error!("CRT invalid parameter ({what}) with no handler installed: terminating (STATUS_INVALID_CRUNTIME_PARAMETER)");
    unsafe { crate::crash::report_here(&format!("CRT invalid parameter ({what})")) };
    pr::exit_process(0xC000_0417)
}


pub unsafe extern "win64" fn _invalid_parameter_noinfo() {
    invalid_parameter("_invalid_parameter_noinfo");
}
pub unsafe extern "win64" fn _invalid_parameter_noinfo_noreturn() -> ! {
    invalid_parameter("_invalid_parameter_noinfo_noreturn");
    pr::exit_process(0xC000_0417)
}
pub unsafe extern "win64" fn _invalid_parameter(e: *const u16, f: *const u16, file: *const u16, line: u32, _r: usize) {
    invalid_parameter(&format!(
        "{} in {} at {}:{}",
        from_wide_opt(e).unwrap_or_default(),
        from_wide_opt(f).unwrap_or_default(),
        from_wide_opt(file).unwrap_or_default(),
        line
    ));
}
pub unsafe extern "win64" fn _invoke_watson(_e: *const u16, _f: *const u16, _file: *const u16, _line: u32, _r: usize) -> ! {
    log::error!("_invoke_watson: fatal CRT error");
    crate::crash::report_here("_invoke_watson");
    pr::exit_process(0xC000_0409)
}

pub unsafe extern "win64" fn _wassert(msg: *const u16, file: *const u16, line: u32) {
    log::error!("Assertion failed: {}, file {}, line {line}", from_wide(msg), from_wide(file));
    abort()
}
pub unsafe extern "win64" fn _assert(msg: *const u8, file: *const u8, line: u32) {
    log::error!("Assertion failed: {}, file {}, line {line}", from_ansi(msg), from_ansi(file));
    abort()
}

pub unsafe extern "win64" fn abort() -> ! {
    log::error!("abort() called");
    crate::crash::report_here("abort()");
    raise(SIGABRT);
    pr::exit_process(3)
}

pub unsafe extern "win64" fn terminate() -> ! {
    let h = TERMINATE_HANDLER.load(Ordering::SeqCst);
    log::error!("terminate() called");
    if h != 0 {
        let f: unsafe extern "win64" fn() = std::mem::transmute(h);
        f();
    }
    abort()
}
pub unsafe extern "win64" fn set_terminate(h: usize) -> usize {
    TERMINATE_HANDLER.swap(h, Ordering::SeqCst)
}
pub unsafe extern "win64" fn _get_terminate() -> usize {
    TERMINATE_HANDLER.load(Ordering::SeqCst)
}

// ---------------------------------------------------------------------------
// Signals
// ---------------------------------------------------------------------------

pub const SIGINT: i32 = 2;
pub const SIGILL: i32 = 4;
pub const SIGFPE: i32 = 8;
pub const SIGSEGV: i32 = 11;
pub const SIGTERM: i32 = 15;
pub const SIGBREAK: i32 = 21;
pub const SIGABRT: i32 = 22;
const SIG_DFL: usize = 0;
const SIG_IGN: usize = 1;
const SIG_ERR: usize = usize::MAX;

static SIGNALS: Mutex<[usize; 23]> = Mutex::new([0; 23]);

pub unsafe extern "win64" fn signal(sig: i32, h: usize) -> usize {
    let idx = match sig {
        SIGINT | SIGILL | SIGFPE | SIGSEGV | SIGTERM | SIGBREAK | SIGABRT => sig as usize,
        6 => SIGABRT as usize, // SIGABRT_COMPAT
        _ => {
            errno::set(EINVAL);
            return SIG_ERR;
        }
    };
    let mut s = SIGNALS.lock().unwrap();
    std::mem::replace(&mut s[idx], h)
}

pub unsafe extern "win64" fn raise(sig: i32) -> i32 {
    let h = {
        let s = SIGNALS.lock().unwrap();
        if (sig as usize) < s.len() {
            s[sig as usize]
        } else {
            errno::set(EINVAL);
            return -1;
        }
    };
    match h {
        SIG_IGN => 0,
        SIG_DFL => {
            log::error!("raise({sig}): default action, terminating with code 3");
            pr::exit_process(3)
        }
        f => {
            if sig != SIGFPE && sig != SIGILL && sig != SIGSEGV {
                SIGNALS.lock().unwrap()[sig as usize] = SIG_DFL;
            }
            let f: unsafe extern "win64" fn(i32) = std::mem::transmute(f);
            f(sig);
            0
        }
    }
}

pub unsafe extern "win64" fn __pxcptinfoptrs() -> *mut *mut c_void {
    thread_local!(static P: std::cell::Cell<*mut c_void> = const { std::cell::Cell::new(std::ptr::null_mut()) });
    P.with(|c| c.as_ptr())
}

pub unsafe extern "win64" fn __fpecode() -> *mut i32 {
    thread_local!(static P: std::cell::Cell<i32> = const { std::cell::Cell::new(0) });
    P.with(|c| c.as_ptr())
}

/// `__C_specific_handler`-adjacent filter used by the exe's startup code:
/// with no CRT signal handler for the exception, keep searching.
pub unsafe extern "win64" fn _seh_filter_exe(code: u32, _ptrs: *mut c_void) -> i32 {
    log::debug!("_seh_filter_exe({code:#x}) -> EXCEPTION_CONTINUE_SEARCH");
    0
}
pub unsafe extern "win64" fn _seh_filter_dll(code: u32, ptrs: *mut c_void) -> i32 {
    _seh_filter_exe(code, ptrs)
}
pub unsafe extern "win64" fn _XcptFilter(code: u32, ptrs: *mut c_void) -> i32 {
    _seh_filter_exe(code, ptrs)
}

// ---------------------------------------------------------------------------
// Floating point control (MXCSR on x64)
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "x86_64"))]
thread_local! {
    static MXCSR_TLS: std::cell::Cell<u32> = const { std::cell::Cell::new(0x1F80) };
}

#[cfg(target_arch = "x86_64")]
fn get_mxcsr() -> u32 {
    let mut v: u32 = 0;
    unsafe { std::arch::asm!("stmxcsr [{}]", in(reg) &mut v, options(nostack)) };
    v
}
#[cfg(not(target_arch = "x86_64"))]
fn get_mxcsr() -> u32 {
    MXCSR_TLS.with(|c| c.get())
}

#[cfg(target_arch = "x86_64")]
fn set_mxcsr(v: u32) {
    unsafe { std::arch::asm!("ldmxcsr [{}]", in(reg) &v, options(nostack)) };
}
#[cfg(not(target_arch = "x86_64"))]
fn set_mxcsr(v: u32) {
    MXCSR_TLS.with(|c| c.set(v));
}

/// Current guest thread's x86 MXCSR register (synchronized with `x86-runtime`).
pub fn guest_mxcsr() -> u32 {
    get_mxcsr()
}

/// Update the guest thread's x86 MXCSR register (synchronized with `x86-runtime`).
pub fn set_guest_mxcsr(v: u32) {
    set_mxcsr(v & 0xFFFF);
}

const EM_INEXACT: u32 = 0x1;
const EM_UNDERFLOW: u32 = 0x2;
const EM_OVERFLOW: u32 = 0x4;
const EM_ZERODIVIDE: u32 = 0x8;
const EM_INVALID: u32 = 0x10;
const EM_DENORMAL: u32 = 0x80000;
const MCW_RC: u32 = 0x300;
const MCW_DN: u32 = 0x0300_0000;
const MCW_PC: u32 = 0x30000;
const MCW_IC: u32 = 0x40000;

fn mxcsr_to_cw(m: u32) -> u32 {
    let mut cw = 0;
    if m & 0x80 != 0 {
        cw |= EM_INVALID;
    }
    if m & 0x100 != 0 {
        cw |= EM_DENORMAL;
    }
    if m & 0x200 != 0 {
        cw |= EM_ZERODIVIDE;
    }
    if m & 0x400 != 0 {
        cw |= EM_OVERFLOW;
    }
    if m & 0x800 != 0 {
        cw |= EM_UNDERFLOW;
    }
    if m & 0x1000 != 0 {
        cw |= EM_INEXACT;
    }
    cw |= match (m >> 13) & 3 {
        0 => 0,
        1 => 0x100,
        2 => 0x200,
        _ => 0x300,
    };
    cw |= match (m & 0x8000 != 0, m & 0x40 != 0) {
        (true, true) => 0x0100_0000,
        (false, true) => 0x0200_0000,
        (true, false) => 0x0300_0000,
        _ => 0,
    };
    cw
}

fn cw_to_mxcsr(cw: u32, old: u32) -> u32 {
    let mut m = old & !(0x1F80 | 0x6000 | 0x8000 | 0x40);
    if cw & EM_INVALID != 0 {
        m |= 0x80;
    }
    if cw & EM_DENORMAL != 0 {
        m |= 0x100;
    }
    if cw & EM_ZERODIVIDE != 0 {
        m |= 0x200;
    }
    if cw & EM_OVERFLOW != 0 {
        m |= 0x400;
    }
    if cw & EM_UNDERFLOW != 0 {
        m |= 0x800;
    }
    if cw & EM_INEXACT != 0 {
        m |= 0x1000;
    }
    m |= ((cw & MCW_RC) >> 8) << 13;
    m |= match cw & MCW_DN {
        0x0100_0000 => 0x8040,
        0x0200_0000 => 0x40,
        0x0300_0000 => 0x8000,
        _ => 0,
    };
    m
}

fn controlfp(new: u32, mask: u32) -> u32 {
    let mask = mask & !(MCW_PC | MCW_IC); // not applicable on x64
    let old = get_mxcsr();
    let cur = mxcsr_to_cw(old);
    if mask != 0 {
        let cw = (cur & !mask) | (new & mask);
        set_mxcsr(cw_to_mxcsr(cw, old));
        return cw;
    }
    cur
}

pub unsafe extern "win64" fn _controlfp(new: u32, mask: u32) -> u32 {
    controlfp(new, mask)
}
pub unsafe extern "win64" fn _control87(new: u32, mask: u32) -> u32 {
    controlfp(new, mask)
}
pub unsafe extern "win64" fn __control87_2(new: u32, mask: u32, x87: *mut u32, sse: *mut u32) -> i32 {
    let v = controlfp(new, mask);
    if !x87.is_null() {
        *x87 = v;
    }
    if !sse.is_null() {
        *sse = v;
    }
    1
}
pub unsafe extern "win64" fn _controlfp_s(cur: *mut u32, new: u32, mask: u32) -> i32 {
    if mask & MCW_PC != 0 && new & mask & MCW_PC != 0 {
        // Precision control is not supported on x64.
        if !cur.is_null() {
            *cur = controlfp(0, 0);
        }
        errno::set(EINVAL);
        return EINVAL;
    }
    let v = controlfp(new, mask);
    if !cur.is_null() {
        *cur = v;
    }
    0
}

fn status_from_mxcsr(m: u32) -> u32 {
    let mut s = 0;
    if m & 0x1 != 0 {
        s |= 0x10; // _SW_INVALID
    }
    if m & 0x2 != 0 {
        s |= 0x80000; // _SW_DENORMAL
    }
    if m & 0x4 != 0 {
        s |= 0x8; // _SW_ZERODIVIDE
    }
    if m & 0x8 != 0 {
        s |= 0x4; // _SW_OVERFLOW
    }
    if m & 0x10 != 0 {
        s |= 0x2; // _SW_UNDERFLOW
    }
    if m & 0x20 != 0 {
        s |= 0x1; // _SW_INEXACT
    }
    s
}

pub unsafe extern "win64" fn _clearfp() -> u32 {
    let m = get_mxcsr();
    set_mxcsr(m & !0x3F);
    status_from_mxcsr(m)
}
pub unsafe extern "win64" fn _statusfp() -> u32 {
    status_from_mxcsr(get_mxcsr())
}
pub unsafe extern "win64" fn _statusfp2(x87: *mut u32, sse: *mut u32) {
    let s = status_from_mxcsr(get_mxcsr());
    if !x87.is_null() {
        *x87 = 0;
    }
    if !sse.is_null() {
        *sse = s;
    }
}
pub unsafe extern "win64" fn _fpreset() {
    set_mxcsr(0x1F80);
}
pub unsafe extern "win64" fn fegetround() -> i32 {
    // MSVC FE_* values: TONEAREST 0, DOWNWARD 0x100, UPWARD 0x200, TOWARDZERO 0x300
    (((get_mxcsr() >> 13) & 3) << 8) as i32
}
pub unsafe extern "win64" fn __fpe_flt_rounds() -> i32 {
    // C99/C11 FLT_ROUNDS mapping from MXCSR RC bits (14:13):
    // 0 (TONEAREST) -> 1, 1 (DOWNWARD) -> 3, 2 (UPWARD) -> 2, 3 (TOWARDZERO) -> 0.
    match (get_mxcsr() >> 13) & 3 {
        0 => 1,
        1 => 3,
        2 => 2,
        3 => 0,
        _ => -1,
    }
}
pub unsafe extern "win64" fn fesetround(r: i32) -> i32 {
    if r & !0x300 != 0 {
        return 1;
    }
    let m = get_mxcsr() & !0x6000;
    set_mxcsr(m | (((r as u32) >> 8) << 13));
    0
}
pub unsafe extern "win64" fn fegetenv(env: *mut u32) -> i32 {
    *env = controlfp(0, 0);
    *env.add(1) = status_from_mxcsr(get_mxcsr());
    0
}
pub unsafe extern "win64" fn fesetenv(env: *const u32) -> i32 {
    controlfp(*env, 0xFFFF_FFFF);
    0
}
pub unsafe extern "win64" fn feclearexcept(_e: i32) -> i32 {
    _clearfp();
    0
}
pub unsafe extern "win64" fn fetestexcept(e: i32) -> i32 {
    (status_from_mxcsr(get_mxcsr()) as i32) & e
}
pub unsafe extern "win64" fn feholdexcept(env: *mut u32) -> i32 {
    fegetenv(env);
    _clearfp();
    controlfp(0x8001F, 0x8001F);
    0
}

// ---------------------------------------------------------------------------
// Threads
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn _beginthreadex(
    sec: *mut c_void,
    stack: u32,
    start: Option<crate::thread::ThreadProc>,
    arg: *mut c_void,
    flags: u32,
    tid: *mut u32,
) -> usize {
    let h = crate::kernel32::threads::CreateThread(sec, stack as usize, start, arg, flags, tid);
    if h.is_null() {
        errno::set(errno::EAGAIN);
    }
    h as usize
}

struct BeginThread {
    f: unsafe extern "win64" fn(*mut c_void),
    arg: *mut c_void,
}

unsafe extern "win64" fn begin_thread_tramp(p: *mut c_void) -> u32 {
    let b = Box::from_raw(p as *mut BeginThread);
    crate::call_guest8(b.f as usize, b.arg as u64, 0, 0, 0, 0, 0, 0, 0);
    0
}

/// Register internal `ucrt::runtime` host trampolines in `VA_DISPATCH` so `CreateThread` ->
/// `call_guest8` -> `emu::call_va8` dispatches them natively instead of refusing host `__TEXT`.
pub(crate) fn register_host_thunks() {
    let addr = begin_thread_tramp as *const () as usize;
    crate::register_native_va(addr as u64, addr);
}

pub unsafe extern "win64" fn _beginthread(f: unsafe extern "win64" fn(*mut c_void), stack: u32, arg: *mut c_void) -> usize {
    if (f as usize) == 0 {
        errno::set(EINVAL);
        return usize::MAX;
    }
    register_host_thunks();
    crate::debug_assert_guest_callable(begin_thread_tramp as *const () as usize, "ucrt::_beginthread");
    let p = Box::into_raw(Box::new(BeginThread { f, arg })) as *mut c_void;
    let h = crate::kernel32::threads::CreateThread(std::ptr::null_mut(), stack as usize, Some(begin_thread_tramp), p, 0, std::ptr::null_mut());
    if h.is_null() {
        drop(Box::from_raw(p as *mut BeginThread));
        errno::set(errno::EAGAIN);
        return usize::MAX;
    }
    h as usize
}

pub unsafe extern "win64" fn _endthreadex(code: u32) -> ! {
    crate::thread::exit_current_thread(code)
}
pub unsafe extern "win64" fn _endthread() -> ! {
    crate::thread::exit_current_thread(0)
}

// ---------------------------------------------------------------------------
// new handler
// ---------------------------------------------------------------------------

static NEW_HANDLER: AtomicUsize = AtomicUsize::new(0);
static NEW_MODE: AtomicI32 = AtomicI32::new(0);

pub unsafe extern "win64" fn _set_new_handler(h: usize) -> usize {
    NEW_HANDLER.swap(h, Ordering::SeqCst)
}
pub unsafe extern "win64" fn _query_new_handler() -> usize {
    NEW_HANDLER.load(Ordering::SeqCst)
}
pub unsafe extern "win64" fn _callnewh(size: usize) -> i32 {
    let h = NEW_HANDLER.load(Ordering::SeqCst);
    if h == 0 {
        return 0;
    }
    let r = crate::call_guest8(h, size as u64, 0, 0, 0, 0, 0, 0, 0) as i32;
    (r != 0) as i32
}
pub unsafe extern "win64" fn _set_new_mode(m: i32) -> i32 {
    NEW_MODE.swap(m, Ordering::SeqCst)
}
pub unsafe extern "win64" fn _query_new_mode() -> i32 {
    NEW_MODE.load(Ordering::SeqCst)
}

// ---------------------------------------------------------------------------
// conio
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn _cputs(s: *const u8) -> i32 {
    use std::io::Write;
    let _ = std::io::stderr().write_all(cstr_slice(s));
    0
}

pub unsafe extern "win64" fn _getch() -> i32 {
    let fd = crate::kernel32::file::console_input_fd();
    let is_tty = libc::isatty(fd) != 0;
    let mut saved: libc::termios = std::mem::zeroed();
    let changed_termios = if is_tty && libc::tcgetattr(fd, &mut saved) == 0 {
        let mut raw = saved;
        raw.c_iflag &= !(libc::ICRNL | libc::INLCR | libc::IGNCR | libc::IXON | libc::ISTRIP);
        raw.c_lflag &= !(libc::ICANON | libc::ECHO | libc::IEXTEN);
        raw.c_cc[libc::VMIN] = 1;
        raw.c_cc[libc::VTIME] = 0;
        libc::tcsetattr(fd, libc::TCSANOW, &raw) == 0
    } else {
        false
    };
    let mut b = 0u8;
    let r = loop {
        let n = libc::read(fd, &mut b as *mut u8 as *mut c_void, 1);
        if n < 0 && *libc::__error() == libc::EINTR {
            continue;
        }
        break n;
    };
    if changed_termios {
        let _ = libc::tcsetattr(fd, libc::TCSANOW, &saved);
    }
    if r == 1 {
        b as i32
    } else {
        -1
    }
}

pub unsafe extern "win64" fn _kbhit() -> i32 {
    let fd = crate::kernel32::file::console_input_fd();
    let is_tty = libc::isatty(fd) != 0;
    let mut saved: libc::termios = std::mem::zeroed();
    let changed_termios =
        if is_tty && libc::tcgetattr(fd, &mut saved) == 0 && (saved.c_lflag & libc::ICANON) != 0 {
            let mut noncanon = saved;
            noncanon.c_lflag &= !libc::ICANON;
            noncanon.c_cc[libc::VMIN] = 1;
            noncanon.c_cc[libc::VTIME] = 0;
            libc::tcsetattr(fd, libc::TCSANOW, &noncanon) == 0
        } else {
            false
        };
    let mut pfd = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };
    let r = loop {
        let n = libc::poll(&mut pfd, 1, 0);
        if n < 0 && *libc::__error() == libc::EINTR {
            continue;
        }
        break n;
    };
    if changed_termios {
        let _ = libc::tcsetattr(fd, libc::TCSANOW, &saved);
    }
    if r > 0 && (pfd.revents & libc::POLLIN) != 0 {
        1
    } else {
        0
    }
}

/// Resolve the Windows command interpreter (`%COMSPEC%`, defaulting to
/// `C:\Windows\System32\cmd.exe`) if a real guest executable exists at that path.
fn resolve_comspec() -> Option<String> {
    #[cfg(target_os = "ios")]
    {
        return None;
    }
    #[cfg(not(target_os = "ios"))]
    {
        let comspec = pr::get_env("COMSPEC")
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "C:\\Windows\\System32\\cmd.exe".into());
        let host = crate::path::to_host(&comspec)?;
        if host.is_file() && !crate::path::is_protected_host_write(&host) {
            Some(comspec)
        } else {
            None
        }
    }
}

pub unsafe extern "win64" fn system(cmd: *const u8) -> i32 {
    super::stdio::flush_all();
    let comspec = resolve_comspec();
    if cmd.is_null() {
        // Microsoft Learn (`system`): when `command` is NULL, returns nonzero if a command
        // interpreter (`%COMSPEC%` / `cmd.exe`) is found, and 0 if none is available.
        return if comspec.is_some() { 1 } else { 0 };
    }
    let cmd_str = from_ansi(cmd);
    if cmd_str.len() > 32767 {
        errno::set(7); // E2BIG
        return -1;
    }
    let Some(interp) = comspec else {
        // Microsoft Learn (`system`): if the command interpreter cannot be found, returns -1
        // and sets `errno` to `ENOENT`. We never escape to the host POSIX `/bin/sh`.
        log::debug!("system({cmd_str:?}): COMSPEC/cmd.exe not present in virtual drive -> ENOENT");
        errno::set(errno::ENOENT);
        return -1;
    };
    #[cfg(not(target_os = "ios"))]
    {
        let app_w = to_wide_z(&interp);
        let mut cmdline_w = to_wide_z(&format!("\"{interp}\" /c {cmd_str}"));
        let mut si = [0u8; 104];
        (si.as_mut_ptr() as *mut u32).write_unaligned(104);
        let mut pi = [0u8; 24];
        let ok = crate::kernel32::process::CreateProcessW(
            app_w.as_ptr(),
            cmdline_w.as_mut_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            FALSE,
            0,
            std::ptr::null_mut(),
            std::ptr::null(),
            si.as_mut_ptr() as *mut c_void,
            pi.as_mut_ptr() as *mut c_void,
        );
        if ok == 0 {
            errno::set(errno::ENOENT);
            return -1;
        }
        let h_proc = (pi.as_ptr() as *const HANDLE).read_unaligned();
        let h_thr = (pi.as_ptr().add(8) as *const HANDLE).read_unaligned();
        let _ = crate::kernel32::threads::WaitForSingleObject(h_proc, INFINITE);
        let mut code = 0u32;
        let _ = crate::kernel32::process::GetExitCodeProcess(h_proc, &mut code);
        let _ = crate::kernel32::threads::CloseHandle(h_thr);
        let _ = crate::kernel32::threads::CloseHandle(h_proc);
        code as i32
    }
    #[cfg(target_os = "ios")]
    {
        let _ = interp;
        errno::set(errno::ENOENT);
        -1
    }
}

pub unsafe extern "win64" fn __threadid() -> u32 {
    crate::teb::thread_id()
}
pub unsafe extern "win64" fn __threadhandle() -> HANDLE {
    crate::handles::CURRENT_THREAD
}

pub unsafe extern "win64" fn _crt_debugger_hook(_r: i32) {}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn collect_wide_env(mut p: *mut *mut u16) -> Vec<String> {
        assert!(!p.is_null(), "wide env table pointer must not be null");
        let mut out = Vec::new();
        while !(*p).is_null() {
            out.push(from_wide(*p));
            p = p.add(1);
        }
        out
    }

    unsafe fn collect_narrow_env(mut p: *mut *mut u8) -> Vec<String> {
        assert!(!p.is_null(), "narrow env table pointer must not be null");
        let mut out = Vec::new();
        while !(*p).is_null() {
            out.push(from_ansi(*p));
            p = p.add(1);
        }
        out
    }

    #[test]
    fn initial_environment_exports_snapshot_isolation_unicode_and_empty() {
        unsafe {
            // 1. Verify actual DLL exports resolve via both ucrtbase.dll and api-ms-win-crt-runtime-l1-1-0.dll.
            let wide_export = crate::lookup("ucrtbase.dll", "_get_initial_wide_environment")
                .expect("ucrtbase.dll!_get_initial_wide_environment must be exported");
            let narrow_export = crate::lookup("ucrtbase.dll", "_get_initial_narrow_environment")
                .expect("ucrtbase.dll!_get_initial_narrow_environment must be exported");
            assert_eq!(
                crate::lookup("api-ms-win-crt-runtime-l1-1-0.dll", "_get_initial_wide_environment"),
                Some(wide_export)
            );
            assert_eq!(
                crate::lookup("api-ms-win-crt-runtime-l1-1-0.dll", "_get_initial_narrow_environment"),
                Some(narrow_export)
            );
            let call_initial_wide: unsafe extern "win64" fn() -> *mut *mut u16 =
                std::mem::transmute(wide_export);
            let call_initial_narrow: unsafe extern "win64" fn() -> *mut *mut u8 =
                std::mem::transmute(narrow_export);

            // 2. Simulate guest child process environment override (including wide/non-ASCII & surrogate pair).
            let child_vars = vec![
                ("CHILD_KEY".to_string(), "orig".to_string()),
                ("TO_DELETE".to_string(), "gone".to_string()),
                ("UNICODE_VAL".to_string(), "café_日本語_🚀".to_string()),
            ];
            let encoded = pr::encode_env_list(&child_vars);
            let decoded = pr::decode_env_list(&encoded).expect("decode_env_list");
            pr::replace_env(decoded);

            assert_eq!(_initialize_wide_environment(), 0);
            assert_eq!(_initialize_narrow_environment(), 0);

            let init_w = call_initial_wide();
            let init_a = call_initial_narrow();
            assert_eq!(call_initial_wide(), init_w, "initial wide pointer must be stable");
            assert_eq!(call_initial_narrow(), init_a, "initial narrow pointer must be stable");

            let expected = vec![
                "CHILD_KEY=orig".to_string(),
                "TO_DELETE=gone".to_string(),
                "UNICODE_VAL=café_日本語_🚀".to_string(),
            ];
            assert_eq!(collect_wide_env(init_w), expected);
            assert_eq!(collect_narrow_env(init_a), expected);

            // Verify raw UTF-16 encoding of the surrogate pair in UNICODE_VAL entry.
            let unicode_w_entry = wide_slice(*init_w.add(2));
            let expected_utf16: Vec<u16> = "UNICODE_VAL=café_日本語_🚀".encode_utf16().collect();
            assert_eq!(unicode_w_entry, expected_utf16.as_slice());

            // Mutable environ tables must be distinct allocations from initial snapshots.
            let mut_w_before = *__p__wenviron() as *mut *mut u16;
            let mut_a_before = *__p__environ() as *mut *mut u8;
            assert_ne!(mut_w_before, init_w);
            assert_ne!(mut_a_before, init_a);
            assert_eq!(collect_wide_env(mut_w_before), expected);
            assert_eq!(collect_narrow_env(mut_a_before), expected);

            // 3. Mutate via _wputenv, _putenv, SetEnvironmentVariableW, and direct mutable table write.
            let add_w = to_wide_z("ADDED_W=new_value");
            assert_eq!(super::super::env::_wputenv(add_w.as_ptr()), 0);
            assert_eq!(super::super::env::_putenv(b"CHILD_KEY=mutated\0".as_ptr()), 0);
            let del_w = to_wide_z("TO_DELETE=");
            assert_eq!(super::super::env::_wputenv(del_w.as_ptr()), 0);
            let k32_k = to_wide_z("K32_ONLY");
            let k32_v = to_wide_z("from_kernel32");
            assert_ne!(
                crate::kernel32::process::SetEnvironmentVariableW(k32_k.as_ptr(), k32_v.as_ptr()),
                0
            );

            // Mutable CRT environ reflects CRT mutations.
            let mut_w_after = *__p__wenviron() as *mut *mut u16;
            let mut_a_after = *__p__environ() as *mut *mut u8;
            let mut_w_entries = collect_wide_env(mut_w_after);
            let mut_a_entries = collect_narrow_env(mut_a_after);
            assert!(mut_w_entries.contains(&"ADDED_W=new_value".to_string()));
            assert!(mut_w_entries.contains(&"CHILD_KEY=mutated".to_string()));
            assert!(!mut_w_entries.iter().any(|s| s.starts_with("TO_DELETE=")));
            assert_eq!(mut_a_entries, mut_w_entries);

            // Direct in-place clobber of mutable table must not affect initial snapshot.
            *mut_w_after = std::ptr::null_mut();
            *mut_a_after = std::ptr::null_mut();

            // Initial snapshot pointers and contents remain completely unchanged.
            assert_eq!(call_initial_wide(), init_w);
            assert_eq!(call_initial_narrow(), init_a);
            assert_eq!(collect_wide_env(call_initial_wide()), expected);
            assert_eq!(collect_narrow_env(call_initial_narrow()), expected);

            // 4. Empty environment returns non-null, immediately NULL-terminated tables.
            pr::replace_env(Vec::new());
            let empty_w = call_initial_wide();
            let empty_a = call_initial_narrow();
            assert!(!empty_w.is_null());
            assert!(!empty_a.is_null());
            assert!((*empty_w).is_null(), "empty wide initial env must be NULL-terminated at [0]");
            assert!((*empty_a).is_null(), "empty narrow initial env must be NULL-terminated at [0]");
            assert!(collect_wide_env(empty_w).is_empty());
            assert!(collect_narrow_env(empty_a).is_empty());

            // 5. Restore standard default environment for any subsequent serial tests.
            pr::init_env(&[]);
            let restored_w = call_initial_wide();
            let restored_entries = collect_wide_env(restored_w);
            assert!(
                restored_entries.iter().any(|s| s == "OS=Windows_NT"),
                "init_env snapshot should include default Windows environment"
            );
        }
    }

    #[test]
    fn fpe_flt_rounds_tracks_mxcsr_rounding_mode() {
        unsafe {
            let saved = fegetround();
            assert_eq!(fesetround(0x000), 0); // FE_TONEAREST
            assert_eq!(__fpe_flt_rounds(), 1);
            assert_eq!(fesetround(0x300), 0); // FE_TOWARDZERO
            assert_eq!(__fpe_flt_rounds(), 0);
            assert_eq!(fesetround(0x200), 0); // FE_UPWARD
            assert_eq!(__fpe_flt_rounds(), 2);
            assert_eq!(fesetround(0x100), 0); // FE_DOWNWARD
            assert_eq!(__fpe_flt_rounds(), 3);
            assert_eq!(fesetround(saved), 0);
        }
    }

    #[test]
    fn getch_kbhit_reads_from_console_pipe_and_eof() {
        use crate::kernel32::file::{GetStdHandle, SetStdHandle};
        use crate::kernel32::threads::CloseHandle;
        const STD_INPUT_HANDLE: u32 = -10i32 as u32;
        unsafe {
            let mut fds = [0i32; 2];
            assert_eq!(libc::pipe(fds.as_mut_ptr()), 0);
            let old_in = GetStdHandle(STD_INPUT_HANDLE);
            let h_read = crate::kernel32::file::wrap_owned_fd(
                std::os::fd::FromRawFd::from_raw_fd(fds[0]),
                crate::kernel32::file::FileKind::Pipe,
                "test-getch-pipe".into(),
                0x8000_0000,
            );
            assert_eq!(SetStdHandle(STD_INPUT_HANDLE, h_read), TRUE);

            assert_eq!(_kbhit(), 0);
            assert_eq!(libc::write(fds[1], b"Az".as_ptr() as *const c_void, 2), 2);
            assert_eq!(_kbhit(), 1);
            assert_eq!(_getch(), b'A' as i32);
            assert_eq!(_getch(), b'z' as i32);
            assert_eq!(_kbhit(), 0);
            libc::close(fds[1]);
            assert_eq!(_getch(), -1);

            assert_eq!(SetStdHandle(STD_INPUT_HANDLE, old_in), TRUE);
            CloseHandle(h_read);
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn kbhit_and_getch_detect_single_keystroke_on_canonical_pty() {
        use crate::kernel32::file::{GetStdHandle, SetStdHandle};
        use crate::kernel32::threads::CloseHandle;
        const STD_INPUT_HANDLE: u32 = -10i32 as u32;
        unsafe {
            let master = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
            assert!(master >= 0);
            assert_eq!(libc::grantpt(master), 0);
            assert_eq!(libc::unlockpt(master), 0);
            let slave_name = libc::ptsname(master);
            assert!(!slave_name.is_null());
            let slave = libc::open(slave_name, libc::O_RDWR | libc::O_NOCTTY);
            assert!(slave >= 0);

            // Ensure canonical input + CR->NL translation are enabled (normal host terminal state).
            let mut tio: libc::termios = std::mem::zeroed();
            assert_eq!(libc::tcgetattr(slave, &mut tio), 0);
            tio.c_iflag |= libc::ICRNL;
            tio.c_lflag |= libc::ICANON | libc::ECHO;
            assert_eq!(libc::tcsetattr(slave, libc::TCSANOW, &tio), 0);

            let old_in = GetStdHandle(STD_INPUT_HANDLE);
            let h_tty = crate::kernel32::file::wrap_owned_fd(
                std::os::fd::FromRawFd::from_raw_fd(slave),
                crate::kernel32::file::FileKind::Console,
                "test-kbhit-pty".into(),
                0x8000_0000,
            );
            assert_eq!(SetStdHandle(STD_INPUT_HANDLE, h_tty), TRUE);

            // Empty canonical PTY reports 0.
            assert_eq!(_kbhit(), 0);
            // Write a single character WITHOUT newline; _kbhit must report 1 and preserve ICANON.
            assert_eq!(libc::write(master, b"A".as_ptr() as *const c_void, 1), 1);
            // Allow kernel PTY line discipline to deliver byte to slave queue.
            let mut ready = 0;
            for _ in 0..20 {
                ready = _kbhit();
                if ready != 0 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            assert_eq!(ready, 1, "_kbhit must detect single keystroke on ICANON PTY before Enter");

            let mut after_kbhit: libc::termios = std::mem::zeroed();
            assert_eq!(libc::tcgetattr(slave, &mut after_kbhit), 0);
            assert_ne!(after_kbhit.c_lflag & libc::ICANON, 0, "_kbhit must restore ICANON");

            assert_eq!(_getch(), b'A' as i32);
            assert_eq!(_kbhit(), 0);

            let mut after_getch: libc::termios = std::mem::zeroed();
            assert_eq!(libc::tcgetattr(slave, &mut after_getch), 0);
            assert_ne!(after_getch.c_lflag & libc::ICANON, 0, "_getch must restore ICANON");
            assert_ne!(after_getch.c_iflag & libc::ICRNL, 0, "_getch must restore ICRNL");

            // Enter key (0x0D) arriving while _getch() is active must NOT be translated to LF (0x0A).
            let writer = std::thread::spawn(move || {
                let mut cur: libc::termios = std::mem::zeroed();
                for _ in 0..200 {
                    if libc::tcgetattr(slave, &mut cur) == 0 && (cur.c_iflag & libc::ICRNL) == 0 {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(2));
                }
                assert_eq!(libc::write(master, b"\r".as_ptr() as *const c_void, 1), 1);
            });
            assert_eq!(_getch(), b'\r' as i32, "_getch must return CR (0x0D) for Enter on ICRNL PTY");
            writer.join().unwrap();

            let mut after_cr: libc::termios = std::mem::zeroed();
            assert_eq!(libc::tcgetattr(slave, &mut after_cr), 0);
            assert_ne!(after_cr.c_lflag & libc::ICANON, 0, "_getch must restore ICANON after CR");
            assert_ne!(after_cr.c_iflag & libc::ICRNL, 0, "_getch must restore ICRNL after CR");

            assert_eq!(SetStdHandle(STD_INPUT_HANDLE, old_in), TRUE);
            CloseHandle(h_tty);
            libc::close(master);
        }
    }

    #[test]
    fn system_matches_windows_comspec_contract() {
        unsafe {
            let prev = pr::get_env("COMSPEC");
            pr::set_env("COMSPEC", Some("C:\\NonExistent\\cmd.exe"));
            assert_eq!(system(std::ptr::null()), 0);
            assert_eq!(system(b"echo hello\0".as_ptr()), -1);
            assert_eq!(errno::get(), errno::ENOENT);
            let huge = format!("{}\0", "a".repeat(33000));
            assert_eq!(system(huge.as_ptr()), -1);
            assert_eq!(errno::get(), 7); // E2BIG
            pr::set_env("COMSPEC", prev.as_deref());
        }
    }
}
