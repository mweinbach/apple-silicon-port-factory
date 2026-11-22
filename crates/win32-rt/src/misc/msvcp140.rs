//! `msvcp140.dll` — Microsoft C++ Standard Library runtime (`std::*`, `Concurrency::*`).
//!
//! Implements all 133 symbols imported by `AlanWake2.exe` from `msvcp140.dll`:
//! - C11/C++11 threads, mutexes (`_Mtx_*` over `SRWLOCK`), condition variables (`_Cnd_*` over `CONDITION_VARIABLE`),
//!   timers (`_Query_perf_*`, `_Xtime_get_ticks`), and `std::_Random_device`.
//! - Error category tables (`_Syserror_map`, `_Winerror_map`) and `std::_X*` / `_Throw_*` helpers.
//! - `std::exception_ptr` (`__ExceptionPtr*`) and `std::uncaught_exception(s)`.
//! - `Concurrency::details` (`_ContextCallback`, `_TaskEventLogger`, `_Threadpool_chore`, `task_continuation_context`).
//! - `std::_Lockit`, `std::locale` (`_Locinfo`, `_Locimp`, `facet`, `id`, `_Yarn<char>`, `ctype<char>`,
//!   `codecvt<char32_t, char, mbstate_t>`, `codecvt<wchar_t, char, mbstate_t>`, `_Strcoll`, `_Strxfrm`).
//! - `std::basic_streambuf<char>`, `std::basic_ios<char>`, `std::basic_istream<char>`,
//!   `std::basic_ostream<char>`, `std::basic_iostream<char>`, `std::cout`, and `std::cerr`.

// @exports: _Cnd_broadcast, _Cnd_destroy_in_situ
// @exports: _Cnd_do_broadcast_at_thread_exit, _Cnd_init_in_situ
// @exports: _Cnd_register_at_thread_exit, _Cnd_signal, _Cnd_timedwait
// @exports: _Cnd_unregister_at_thread_exit, _Cnd_wait, _Mtx_current_owns
// @exports: _Mtx_destroy_in_situ, _Mtx_init_in_situ, _Mtx_lock, _Mtx_trylock
// @exports: _Mtx_unlock, _Query_perf_counter, _Query_perf_frequency, _Strcoll
// @exports: _Strxfrm, _Thrd_detach, _Thrd_id, _Thrd_join, _Thrd_yield
// @exports: _Xtime_get_ticks

use crate::{Export, Ptr};
use crate::kernel32::threadpool::{
    CloseThreadpoolWork, CreateThreadpoolWork, SubmitThreadpoolWork, TpWork,
};
use crate::kernel32::threads::{
    CloseHandle, GetCurrentThreadId, GetExitCodeThread, WaitForSingleObjectEx,
};
use crate::sync::{
    AcquireSRWLockExclusive, ReleaseSRWLockExclusive, SleepConditionVariableSRW,
    TryAcquireSRWLockExclusive, WakeAllConditionVariable, WakeConditionVariable,
};
use std::cell::UnsafeCell;
use std::ffi::{CStr, c_char, c_void};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Mutex, Once};

pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// 1. Mutexes (`_Mtx_*`), Condition Variables (`_Cnd_*`), Threads (`_Thrd_*`),
//    Timers (`_Query_perf_*`, `_Xtime_get_ticks`), and `_Random_device`
// ---------------------------------------------------------------------------

/// `msvcp140` `_Mtx_internal_imp_t` (`0x50` bytes, `0x1800125a0`).
#[repr(C)]
pub struct MtxInternal {
    pub ty: u32,
    pub _pad0: u32,
    pub cs_pad0: u64,
    pub srwlock: usize,
    pub cs_pad1: [u64; 6],
    pub thread_id: u32,
    pub count: i32,
}
const _: () = assert!(std::mem::size_of::<MtxInternal>() == 0x50);

/// `msvcp140` `_Cnd_internal_imp_t` (`0x10` bytes, `0x180012090`).
#[repr(C)]
pub struct CndInternal {
    pub pad: u64,
    pub cv: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Xtime {
    pub sec: i64,
    pub nsec: i32,
}

#[repr(C)]
pub struct ThrdT {
    pub hnd: *mut c_void,
    pub id: u32,
}

pub unsafe extern "win64" fn _Mtx_init_in_situ(mtx: *mut MtxInternal, ty: i32) {
    if mtx.is_null() {
        return;
    }
    (*mtx).ty = ty as u32;
    (*mtx).cs_pad0 = 0;
    (*mtx).srwlock = 0;
    (*mtx).thread_id = u32::MAX;
    (*mtx).count = 0;
}

pub unsafe extern "win64" fn _Mtx_destroy_in_situ(_mtx: *mut MtxInternal) {}

pub unsafe extern "win64" fn _Mtx_current_owns(mtx: *const MtxInternal) -> i32 {
    if mtx.is_null() {
        return 0;
    }
    let tid = GetCurrentThreadId();
    ((*mtx).count != 0 && (*mtx).thread_id == tid) as i32
}

pub unsafe extern "win64" fn _Mtx_lock(mtx: *mut MtxInternal) -> i32 {
    if mtx.is_null() {
        return 4; // _Thrd_error
    }
    let tid = GetCurrentThreadId();
    if (*mtx).thread_id == tid && ((*mtx).ty & 0x100) != 0 {
        (*mtx).count = (*mtx).count.wrapping_add(1);
        return 0; // _Thrd_success
    }
    AcquireSRWLockExclusive((&mut (*mtx).srwlock as *mut usize).cast());
    (*mtx).thread_id = tid;
    (*mtx).count = 1;
    0
}

pub unsafe extern "win64" fn _Mtx_trylock(mtx: *mut MtxInternal) -> i32 {
    if mtx.is_null() {
        return 4;
    }
    let tid = GetCurrentThreadId();
    if (*mtx).thread_id == tid {
        if ((*mtx).ty & 0x100) != 0 {
            (*mtx).count = (*mtx).count.wrapping_add(1);
            return 0;
        }
        return 3; // _Thrd_busy
    }
    if TryAcquireSRWLockExclusive((&mut (*mtx).srwlock as *mut usize).cast()) != 0 {
        (*mtx).thread_id = tid;
        (*mtx).count = 1;
        0
    } else {
        3
    }
}

pub unsafe extern "win64" fn _Mtx_unlock(mtx: *mut MtxInternal) -> i32 {
    if mtx.is_null() {
        return 4;
    }
    (*mtx).count = (*mtx).count.wrapping_sub(1);
    if (*mtx).count == 0 {
        (*mtx).thread_id = u32::MAX;
        ReleaseSRWLockExclusive((&mut (*mtx).srwlock as *mut usize).cast());
    }
    0
}

pub unsafe extern "win64" fn _Cnd_init_in_situ(cnd: *mut CndInternal) {
    if cnd.is_null() {
        return;
    }
    (*cnd).pad = 0;
    (*cnd).cv = 0;
}

pub unsafe extern "win64" fn _Cnd_destroy_in_situ(_cnd: *mut CndInternal) {}

pub unsafe extern "win64" fn _Cnd_signal(cnd: *mut CndInternal) -> i32 {
    if !cnd.is_null() {
        WakeConditionVariable((&mut (*cnd).cv as *mut usize).cast());
    }
    0
}

pub unsafe extern "win64" fn _Cnd_broadcast(cnd: *mut CndInternal) -> i32 {
    if !cnd.is_null() {
        WakeAllConditionVariable((&mut (*cnd).cv as *mut usize).cast());
    }
    0
}

pub unsafe extern "win64" fn _Cnd_wait(cnd: *mut CndInternal, mtx: *mut MtxInternal) -> i32 {
    if cnd.is_null() || mtx.is_null() {
        return 4;
    }
    (*mtx).thread_id = u32::MAX;
    (*mtx).count = (*mtx).count.wrapping_sub(1);
    let _ = SleepConditionVariableSRW(
        (&mut (*cnd).cv as *mut usize).cast(),
        (&mut (*mtx).srwlock as *mut usize).cast(),
        u32::MAX,
        0,
    );
    (*mtx).thread_id = GetCurrentThreadId();
    (*mtx).count = (*mtx).count.wrapping_add(1);
    0
}

pub unsafe extern "win64" fn _Cnd_timedwait(
    cnd: *mut CndInternal,
    mtx: *mut MtxInternal,
    target: *const Xtime,
) -> i32 {
    if target.is_null() {
        return _Cnd_wait(cnd, mtx);
    }
    if cnd.is_null() || mtx.is_null() {
        return 4;
    }
    let now_ticks = _Xtime_get_ticks();
    let now_ns = (now_ticks as i128) * 100;
    let t = std::ptr::read_unaligned(target);
    let target_ns = (t.sec as i128) * 1_000_000_000 + (t.nsec as i128);
    let diff_ms = if target_ns <= now_ns {
        0u32
    } else {
        (((target_ns - now_ns) + 999_999) / 1_000_000).min(u32::MAX as i128 - 1) as u32
    };
    (*mtx).thread_id = u32::MAX;
    (*mtx).count = (*mtx).count.wrapping_sub(1);
    let ok = SleepConditionVariableSRW(
        (&mut (*cnd).cv as *mut usize).cast(),
        (&mut (*mtx).srwlock as *mut usize).cast(),
        diff_ms,
        0,
    );
    (*mtx).thread_id = GetCurrentThreadId();
    (*mtx).count = (*mtx).count.wrapping_add(1);
    if ok != 0 {
        0 // _Thrd_success
    } else {
        2 // _Thrd_timedout
    }
}

struct AtThreadExitEntry {
    tid: u32,
    mtx: usize,
    cnd: usize,
    res: usize,
}

static AT_THREAD_EXIT: Mutex<Vec<AtThreadExitEntry>> = Mutex::new(Vec::new());

pub unsafe extern "win64" fn _Cnd_register_at_thread_exit(
    cnd: *mut CndInternal,
    mtx: *mut MtxInternal,
    p: *mut i32,
) {
    let tid = GetCurrentThreadId();
    AT_THREAD_EXIT.lock().unwrap().push(AtThreadExitEntry {
        tid,
        mtx: mtx as usize,
        cnd: cnd as usize,
        res: p as usize,
    });
}

pub unsafe extern "win64" fn _Cnd_unregister_at_thread_exit(mtx: *mut MtxInternal) {
    let target = mtx as usize;
    AT_THREAD_EXIT.lock().unwrap().retain(|e| e.mtx != target);
}

pub unsafe extern "win64" fn _Cnd_do_broadcast_at_thread_exit() {
    let tid = GetCurrentThreadId();
    let mut entries = Vec::new();
    {
        let mut g = AT_THREAD_EXIT.lock().unwrap();
        let mut i = 0;
        while i < g.len() {
            if g[i].tid == tid {
                entries.push(g.remove(i));
            } else {
                i += 1;
            }
        }
    }
    for e in entries {
        if e.res != 0 {
            *(e.res as *mut i32) = 1;
        }
        if e.mtx != 0 {
            _Mtx_unlock(e.mtx as *mut MtxInternal);
        }
        if e.cnd != 0 {
            _Cnd_broadcast(e.cnd as *mut CndInternal);
        }
    }
}

pub unsafe extern "win64" fn _Thrd_id() -> u32 {
    GetCurrentThreadId()
}

pub unsafe extern "win64" fn _Thrd_join(thr: *const ThrdT, res: *mut i32) -> i32 {
    if thr.is_null() {
        return 4;
    }
    let hnd = (*thr).hnd;
    if WaitForSingleObjectEx(hnd, u32::MAX, 0) == u32::MAX {
        return 4;
    }
    if !res.is_null() {
        let mut code = 0u32;
        if GetExitCodeThread(hnd, &mut code) == 0 {
            return 4;
        }
        *res = code as i32;
    }
    if CloseHandle(hnd) != 0 { 0 } else { 4 }
}

pub unsafe extern "win64" fn _Thrd_detach(thr: *const ThrdT) -> i32 {
    if thr.is_null() {
        return 4;
    }
    if CloseHandle((*thr).hnd) != 0 { 0 } else { 4 }
}

pub unsafe extern "win64" fn _Thrd_yield() {
    let _ = libc::sched_yield();
}

pub unsafe extern "win64" fn _Thrd_sleep(target: *const Xtime) {
    if target.is_null() {
        return;
    }
    let now_ticks = _Xtime_get_ticks();
    let now_ns = (now_ticks as i128) * 100;
    let t = std::ptr::read_unaligned(target);
    let target_ns = (t.sec as i128) * 1_000_000_000 + (t.nsec as i128);
    if target_ns > now_ns {
        let diff_us = (((target_ns - now_ns) + 999) / 1_000).min(1_000_000) as u32;
        let _ = libc::usleep(diff_us);
    }
}

pub unsafe extern "win64" fn _Mbrtowc(
    pwc: *mut u16,
    s: *const u8,
    n: usize,
    _ps: *mut u64,
    _cvt: *const c_void,
) -> i32 {
    if s.is_null() {
        return 0;
    }
    if n == 0 {
        return -2;
    }
    let b0 = *s;
    if b0 == 0 {
        if !pwc.is_null() {
            *pwc = 0;
        }
        return 0;
    }
    let rem = std::slice::from_raw_parts(s, n.min(4));
    let needed = if b0 < 0x80 {
        1
    } else if (b0 & 0xe0) == 0xc0 {
        2
    } else if (b0 & 0xf0) == 0xe0 {
        3
    } else if (b0 & 0xf8) == 0xf0 {
        4
    } else {
        if !pwc.is_null() {
            *pwc = b0 as u16;
        }
        return 1;
    };
    if rem.len() < needed {
        return -2;
    }
    if let Ok(st) = std::str::from_utf8(&rem[..needed]) {
        if let Some(ch) = st.chars().next() {
            if !pwc.is_null() {
                *pwc = ch as u16;
            }
            return needed as i32;
        }
    }
    if !pwc.is_null() {
        *pwc = b0 as u16;
    }
    1
}

pub unsafe extern "win64" fn _Query_perf_counter() -> i64 {
    let mut c = 0i64;
    crate::kernel32::time::QueryPerformanceCounter(&mut c);
    c
}

pub unsafe extern "win64" fn _Query_perf_frequency() -> i64 {
    let mut f = 0i64;
    crate::kernel32::time::QueryPerformanceFrequency(&mut f);
    f
}

pub unsafe extern "win64" fn _Xtime_get_ticks() -> i64 {
    let mut ft = 0u64;
    crate::kernel32::time::GetSystemTimePreciseAsFileTime((&mut ft as *mut u64).cast());
    // Subtract 116444736000000000 (100-ns ticks from 1601-01-01 to 1970-01-01).
    (ft as i64).wrapping_add(-116_444_736_000_000_000i64)
}

unsafe extern "win64" fn std_random_device() -> u32 {
    let mut v = 0u32;
    libc::arc4random_buf(&mut v as *mut u32 as *mut c_void, 4);
    v
}

// ---------------------------------------------------------------------------
// 2. `_Lockit` (`0x180035e20`, `0x180035ec0`)
// ---------------------------------------------------------------------------

static LOCKIT_LOCKS: [usize; 8] = [0; 8];

unsafe extern "win64" fn lockit_ctor(this: *mut i32, kind: i32) -> *mut i32 {
    if !this.is_null() {
        std::ptr::write_unaligned(this, kind);
    }
    let idx = (kind.max(0) as usize).min(7);
    let _ = &LOCKIT_LOCKS[idx];
    this
}

unsafe extern "win64" fn lockit_dtor(_this: *mut i32) {}

// ---------------------------------------------------------------------------
// 3. Error maps (`_Syserror_map`, `_Winerror_map`) and `std::_X*` throw helpers
// ---------------------------------------------------------------------------

static SYSERROR_MAP: &[(i32, &CStr)] = &[
    (0, c"success"),
    (102, c"address family not supported"),
    (100, c"address in use"),
    (101, c"address not available"),
    (113, c"already connected"),
    (7, c"argument list too long"),
    (33, c"argument out of domain"),
    (14, c"bad address"),
    (9, c"bad file descriptor"),
    (104, c"bad message"),
    (32, c"broken pipe"),
    (106, c"connection aborted"),
    (103, c"connection already in progress"),
    (107, c"connection refused"),
    (108, c"connection reset"),
    (18, c"cross device link"),
    (109, c"destination address required"),
    (16, c"device or resource busy"),
    (41, c"directory not empty"),
    (8, c"executable format error"),
    (17, c"file exists"),
    (27, c"file too large"),
    (38, c"filename too long"),
    (40, c"function not supported"),
    (110, c"host unreachable"),
    (111, c"identifier removed"),
    (42, c"illegal byte sequence"),
    (25, c"inappropriate io control operation"),
    (4, c"interrupted"),
    (22, c"invalid argument"),
    (29, c"invalid seek"),
    (5, c"io error"),
    (21, c"is a directory"),
    (115, c"message size"),
    (116, c"network down"),
    (117, c"network reset"),
    (118, c"network unreachable"),
    (119, c"no buffer space"),
    (10, c"no child process"),
    (121, c"no link"),
    (39, c"no lock available"),
    (120, c"no message available"),
    (122, c"no message"),
    (123, c"no protocol option"),
    (28, c"no space on device"),
    (124, c"no stream resources"),
    (6, c"no such device or address"),
    (19, c"no such device"),
    (2, c"no such file or directory"),
    (3, c"no such process"),
    (20, c"not a directory"),
    (128, c"not a socket"),
    (125, c"not a stream"),
    (126, c"not connected"),
    (12, c"not enough memory"),
    (129, c"not supported"),
    (105, c"operation canceled"),
    (112, c"operation in progress"),
    (1, c"operation not permitted"),
    (130, c"operation not supported"),
    (140, c"operation would block"),
    (133, c"owner dead"),
    (13, c"permission denied"),
    (134, c"protocol error"),
    (135, c"protocol not supported"),
    (30, c"read only file system"),
    (36, c"resource deadlock would occur"),
    (11, c"resource unavailable try again"),
    (34, c"result out of range"),
    (127, c"state not recoverable"),
    (137, c"stream timeout"),
    (139, c"text file busy"),
    (138, c"timed out"),
    (23, c"too many files open in system"),
    (24, c"too many files open"),
    (31, c"too many links"),
    (114, c"too many symbolic link levels"),
    (132, c"value too large"),
    (136, c"wrong protocol type"),
];

static WINERROR_MAP: &[(i32, i32)] = &[
    (1, 40), (2, 2), (3, 2), (4, 24), (5, 13), (6, 22), (8, 12), (12, 13),
    (14, 12), (15, 19), (16, 13), (17, 18), (19, 13), (20, 19), (21, 11),
    (25, 5), (29, 5), (30, 5), (32, 13), (33, 39), (39, 28), (50, 129),
    (53, 2), (55, 19), (67, 2), (80, 17), (82, 13), (87, 22), (109, 32),
    (110, 5), (111, 38), (112, 28), (121, 138), (123, 2), (131, 22),
    (142, 16), (145, 41), (170, 16), (183, 17), (206, 38), (212, 39),
    (258, 138), (267, 22), (995, 105), (998, 13), (1011, 5), (1012, 5),
    (1013, 5), (1237, 11), (1460, 138), (2401, 16), (2404, 16), (4393, 22),
    (10004, 4), (10009, 9), (10013, 13), (10014, 14), (10022, 22),
    (10024, 24), (10035, 140), (10036, 112), (10037, 103), (10038, 128),
    (10039, 109), (10040, 115), (10041, 136), (10042, 123), (10043, 135),
    (10045, 130), (10047, 102), (10048, 100), (10049, 101), (10050, 116),
    (10051, 118), (10052, 117), (10053, 106), (10054, 108), (10055, 119),
    (10056, 113), (10057, 126), (10060, 138), (10061, 107), (10063, 38),
    (10065, 110),
];

unsafe extern "win64" fn syserror_map(err: i32) -> *const c_char {
    for &(code, msg) in SYSERROR_MAP {
        if code == err {
            return msg.as_ptr();
        }
    }
    c"unknown error".as_ptr()
}

unsafe extern "win64" fn winerror_map(werr: i32) -> i32 {
    for &(w, pos) in WINERROR_MAP {
        if w == werr {
            return pos;
        }
    }
    0
}

unsafe extern "win64" fn throw_c_error(code: i32) {
    match code {
        1 | 2 => throw_cpp_error(6),
        3 => throw_cpp_error(0),
        4 => throw_cpp_error(1),
        _ => {}
    }
}

unsafe extern "win64" fn throw_cpp_error(code: i32) {
    log::error!("std::_Throw_Cpp_error({code})");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

unsafe extern "win64" fn xbad_alloc() {
    log::error!("std::_Xbad_alloc()");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

unsafe extern "win64" fn xbad_function_call() {
    log::error!("std::_Xbad_function_call()");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

unsafe extern "win64" fn xinvalid_argument(msg: *const c_char) {
    let s = if msg.is_null() { "" } else { CStr::from_ptr(msg).to_str().unwrap_or("") };
    log::error!("std::_Xinvalid_argument({s:?})");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

unsafe extern "win64" fn xlength_error(msg: *const c_char) {
    let s = if msg.is_null() { "" } else { CStr::from_ptr(msg).to_str().unwrap_or("") };
    log::error!("std::_Xlength_error({s:?})");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

unsafe extern "win64" fn xout_of_range(msg: *const c_char) {
    let s = if msg.is_null() { "" } else { CStr::from_ptr(msg).to_str().unwrap_or("") };
    log::error!("std::_Xout_of_range({s:?})");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

unsafe extern "win64" fn xregex_error(err: i32) {
    log::error!("std::_Xregex_error({err})");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

unsafe extern "win64" fn std_uncaught_exception() -> u8 {
    super::vcruntime140::__uncaught_exception()
}

unsafe extern "win64" fn std_uncaught_exceptions() -> i32 {
    super::vcruntime140::__uncaught_exceptions()
}

// ---------------------------------------------------------------------------
// 4. `std::exception_ptr` (`__ExceptionPtr*`, `0x180002440..0x180002710`)
// ---------------------------------------------------------------------------

#[repr(C)]
struct ExceptionPtr {
    ptr: *mut c_void,
    ctrl: *mut AtomicU32,
}

unsafe extern "win64" fn exception_ptr_create(dst: *mut ExceptionPtr) {
    if !dst.is_null() {
        (*dst).ptr = std::ptr::null_mut();
        (*dst).ctrl = std::ptr::null_mut();
    }
}

unsafe extern "win64" fn exception_ptr_destroy(dst: *mut ExceptionPtr) {
    if dst.is_null() {
        return;
    }
    let ctrl = (*dst).ctrl;
    (*dst).ptr = std::ptr::null_mut();
    (*dst).ctrl = std::ptr::null_mut();
    if !ctrl.is_null() && (*ctrl).fetch_sub(1, Ordering::AcqRel) == 1 {
        let _ = Box::from_raw(ctrl);
    }
}

unsafe extern "win64" fn exception_ptr_copy(dst: *mut ExceptionPtr, src: *const ExceptionPtr) {
    if dst.is_null() {
        return;
    }
    if src.is_null() {
        (*dst).ptr = std::ptr::null_mut();
        (*dst).ctrl = std::ptr::null_mut();
        return;
    }
    let ctrl = (*src).ctrl;
    if !ctrl.is_null() {
        (*ctrl).fetch_add(1, Ordering::Relaxed);
    }
    (*dst).ptr = (*src).ptr;
    (*dst).ctrl = ctrl;
}

unsafe extern "win64" fn exception_ptr_assign(dst: *mut ExceptionPtr, src: *const ExceptionPtr) {
    if dst.is_null() || dst as *const ExceptionPtr == src {
        return;
    }
    exception_ptr_destroy(dst);
    exception_ptr_copy(dst, src);
}

unsafe extern "win64" fn exception_ptr_to_bool(ep: *const ExceptionPtr) -> u8 {
    (!ep.is_null() && !(*ep).ptr.is_null()) as u8
}

unsafe extern "win64" fn exception_ptr_current_exception(dst: *mut ExceptionPtr) {
    if dst.is_null() {
        return;
    }
    let cur = *super::vcruntime140::__current_exception();
    if cur.is_null() {
        (*dst).ptr = std::ptr::null_mut();
        (*dst).ctrl = std::ptr::null_mut();
    } else {
        let ctrl = Box::into_raw(Box::new(AtomicU32::new(1)));
        (*dst).ptr = cur;
        (*dst).ctrl = ctrl;
    }
}

unsafe extern "win64" fn exception_ptr_copy_exception(
    dst: *mut ExceptionPtr,
    exc_obj: *const c_void,
    _throw_info: *const c_void,
) {
    if dst.is_null() {
        return;
    }
    if exc_obj.is_null() {
        (*dst).ptr = std::ptr::null_mut();
        (*dst).ctrl = std::ptr::null_mut();
    } else {
        let ctrl = Box::into_raw(Box::new(AtomicU32::new(1)));
        (*dst).ptr = exc_obj as *mut c_void;
        (*dst).ctrl = ctrl;
    }
}

unsafe extern "win64" fn exception_ptr_rethrow(ep: *const ExceptionPtr) {
    let ptr = if ep.is_null() { std::ptr::null_mut() } else { (*ep).ptr };
    super::vcruntime140::_CxxThrowException(ptr, std::ptr::null());
}

// ---------------------------------------------------------------------------
// 5. `Concurrency::details` (`0x180003b90`, `0x180012830..0x180012960`)
// ---------------------------------------------------------------------------

#[repr(C)]
struct ThreadpoolChore {
    work: *mut TpWork,
    callback: Option<unsafe extern "win64" fn(*mut c_void)>,
    context: *mut c_void,
}

unsafe extern "win64" fn task_continuation_context_ctor(this: *mut u8) -> *mut u8 {
    if !this.is_null() {
        std::ptr::write_unaligned(this as *mut u64, 0);
        *this.add(8) = 0;
    }
    this
}

unsafe extern "win64" fn concurrency_get_current_thread_id() -> u32 {
    GetCurrentThreadId()
}

unsafe extern "win64" fn noop_method() {}

unsafe extern "win64" fn is_current_origin_sta() -> u8 {
    0
}

unsafe extern "win64" fn call_in_context(
    _this: *const c_void,
    func: *mut usize,
    _ignore_disconnect: u8,
) {
    if func.is_null() {
        return;
    }
    // `func` is a `std::function<void()>` (`0x40` bytes); `_Mystorage._Ptrs[7]` at `+0x38`
    // holds the `_Func_base<void>*` pointer (`0x180012830`).
    let callable = std::ptr::read_unaligned(func.add(7));
    if callable != 0 {
        let vtable = std::ptr::read_unaligned(callable as *const *const usize);
        let do_call_va = *vtable.add(2);
        let _ = crate::call_guest8(do_call_va, callable as u64, 0, 0, 0, 0, 0, 0, 0);
    }
    let callable_after = std::ptr::read_unaligned(func.add(7));
    if callable_after != 0 {
        let vtable = std::ptr::read_unaligned(callable_after as *const *const usize);
        let delete_this_va = *vtable.add(4);
        let _ = crate::call_guest8(
            delete_this_va,
            callable_after as u64,
            (callable_after != func as usize) as u64,
            0, 0, 0, 0, 0, 0,
        );
        std::ptr::write_unaligned(func.add(7), 0);
    }
}

unsafe extern "win64" fn chore_work_callback(
    _instance: *mut c_void,
    context: *mut c_void,
    _work: *mut c_void,
) {
    let chore = context as *mut ThreadpoolChore;
    if !chore.is_null() {
        if let Some(cb) = (*chore).callback {
            let _ = crate::call_guest8(cb as usize, (*chore).context as u64, 0, 0, 0, 0, 0, 0, 0);
        }
    }
}

unsafe extern "win64" fn schedule_chore(chore: *mut ThreadpoolChore) -> i32 {
    ensure_vtables_registered();
    if chore.is_null() {
        return -1;
    }
    if (*chore).work.is_null() {
        let w = CreateThreadpoolWork(chore_work_callback, chore.cast(), std::ptr::null_mut());
        (*chore).work = w;
        if w.is_null() {
            return -1;
        }
    }
    SubmitThreadpoolWork((*chore).work);
    0
}

unsafe extern "win64" fn release_chore(chore: *mut ThreadpoolChore) {
    if chore.is_null() {
        return;
    }
    if !(*chore).work.is_null() {
        CloseThreadpoolWork((*chore).work);
        (*chore).work = std::ptr::null_mut();
    }
}

// ---------------------------------------------------------------------------
// 6. Locale & Facets (`_Locinfo`, `locale`, `_Locimp`, `facet`, `_Yarn<char>`,
//    `ctype<char>`, `codecvt`, `_Strcoll`, `_Strxfrm`)
// ---------------------------------------------------------------------------

#[repr(C, align(16))]
pub struct LocaleId(UnsafeCell<[u64; 2]>);
unsafe impl Sync for LocaleId {}

pub static ID_CODECVT_W: LocaleId = LocaleId(UnsafeCell::new([1, 0]));
pub static ID_COLLATE_D: LocaleId = LocaleId(UnsafeCell::new([2, 0]));
pub static ID_CTYPE_D: LocaleId = LocaleId(UnsafeCell::new([3, 0]));
pub static ID_NUMPUNCT_D: LocaleId = LocaleId(UnsafeCell::new([4, 0]));

static NEXT_LOCALE_ID: AtomicUsize = AtomicUsize::new(8);

unsafe extern "win64" fn locale_id_operator_usize(this: *mut usize) -> usize {
    if this.is_null() {
        return 0;
    }
    let cur = std::ptr::read_unaligned(this);
    if cur != 0 {
        return cur;
    }
    let next = NEXT_LOCALE_ID.fetch_add(1, Ordering::Relaxed);
    std::ptr::write_unaligned(this, next);
    next
}

unsafe extern "win64" fn yarn_assign(this: *mut *mut u8, s: *const c_char) -> *mut *mut u8 {
    if this.is_null() {
        return this;
    }
    let old = std::ptr::read_unaligned(this);
    if old as *const c_char == s {
        return this;
    }
    if !old.is_null() {
        crate::ucrt::heap::free(old.cast());
        std::ptr::write_unaligned(this, std::ptr::null_mut());
    }
    if !s.is_null() {
        let len = libc::strlen(s) + 1;
        let buf = crate::ucrt::heap::malloc(len) as *mut u8;
        if !buf.is_null() {
            std::ptr::copy_nonoverlapping(s.cast::<u8>(), buf, len);
            std::ptr::write_unaligned(this, buf);
        }
    }
    this
}

/// `_Locinfo` (`0x68` bytes, `0x1800044f0`).
#[repr(C)]
pub struct Locinfo {
    pub lock: i32,
    pub _pad0: i32,
    pub days: [*mut u8; 2],
    pub months: [*mut u8; 2],
    pub ampm: [*mut u8; 2],
    pub time_names: [*mut u8; 2],
    pub old_locname: [*mut u8; 2],
    pub new_locname: [*mut u8; 2],
}

unsafe extern "win64" fn locinfo_ctor(this: *mut u8, name: *const c_char) -> *mut u8 {
    if !this.is_null() {
        std::ptr::write_bytes(this, 0, 0x68);
        if !name.is_null() {
            yarn_assign(this.add(0x58) as *mut *mut u8, name);
        }
    }
    this
}

unsafe extern "win64" fn locinfo_dtor(this: *mut u8) {
    if this.is_null() {
        return;
    }
    for off in [0x08usize, 0x18, 0x28, 0x38, 0x48, 0x58] {
        let p = std::ptr::read_unaligned(this.add(off) as *const *mut c_void);
        if !p.is_null() {
            crate::ucrt::heap::free(p);
            std::ptr::write_unaligned(this.add(off) as *mut *mut c_void, std::ptr::null_mut());
        }
    }
}

#[repr(C)]
pub struct Collvec {
    pub page: u32,
    pub _pad: u32,
    pub locale_name: *mut u16,
}

#[repr(C)]
pub struct Cvtvec {
    pub page: u32,
    pub mb_cur_max: u32,
    pub is_clocale: i32,
    pub is_lead_byte: [u8; 32],
}

unsafe extern "win64" fn locinfo_getcoll(_this: *const u8, out: *mut Collvec) -> *mut Collvec {
    if !out.is_null() {
        std::ptr::write_unaligned(
            out,
            Collvec {
                page: 0,
                _pad: 0,
                locale_name: std::ptr::null_mut(),
            },
        );
    }
    out
}

unsafe extern "win64" fn locinfo_getcvt(_this: *const u8, out: *mut Cvtvec) -> *mut Cvtvec {
    if !out.is_null() {
        std::ptr::write_unaligned(
            out,
            Cvtvec {
                page: 65001, // UTF-8
                mb_cur_max: 4,
                is_clocale: 1,
                is_lead_byte: [0; 32],
            },
        );
    }
    out
}

unsafe extern "win64" fn locinfo_getfalse(_this: *const u8) -> *const c_char {
    c"false".as_ptr()
}

unsafe extern "win64" fn locinfo_gettrue(_this: *const u8) -> *const c_char {
    c"true".as_ptr()
}

unsafe extern "win64" fn locinfo_getlconv(_this: *const u8) -> *const crate::ucrt::locale::Lconv {
    crate::ucrt::locale::localeconv()
}

pub unsafe extern "win64" fn _Strcoll(
    s1: *const u8,
    e1: *const u8,
    s2: *const u8,
    e2: *const u8,
    _coll: *const Collvec,
) -> i32 {
    let l1 = (e1 as usize).saturating_sub(s1 as usize);
    let l2 = (e2 as usize).saturating_sub(s2 as usize);
    let a = if l1 == 0 || s1.is_null() { &[] } else { std::slice::from_raw_parts(s1, l1) };
    let b = if l2 == 0 || s2.is_null() { &[] } else { std::slice::from_raw_parts(s2, l2) };
    match a.cmp(b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

pub unsafe extern "win64" fn _Strxfrm(
    dst: *mut u8,
    dst_end: *mut u8,
    src: *const u8,
    src_end: *const u8,
    _coll: *const Collvec,
) -> usize {
    let src_len = (src_end as usize).saturating_sub(src as usize);
    let dst_cap = (dst_end as usize).saturating_sub(dst as usize);
    if !dst.is_null() && !src.is_null() && src_len <= dst_cap {
        std::ptr::copy_nonoverlapping(src, dst, src_len);
    }
    src_len
}

/// `std::locale::facet` (`0x10` bytes, `0x180004810`).
#[repr(C)]
pub struct Facet {
    pub vtable: *const Ptr,
    pub refs: AtomicU32,
    pub _pad: u32,
}

static FACET_VTABLE: [Ptr; 3] = [
    Ptr(facet_deleting_dtor as *const ()),
    Ptr(facet_incref as *const ()),
    Ptr(facet_decref as *const ()),
];

unsafe extern "win64" fn facet_deleting_dtor(this: *mut Facet, flags: u32) -> *mut Facet {
    if !this.is_null() {
        (*this).vtable = FACET_VTABLE.as_ptr();
        if (flags & 1) != 0 {
            crate::ucrt::heap::free(this.cast());
        }
    }
    this
}

unsafe extern "win64" fn facet_ctor(this: *mut Facet, refs: usize) -> *mut Facet {
    ensure_vtables_registered();
    if !this.is_null() {
        (*this).vtable = FACET_VTABLE.as_ptr();
        (*this).refs = AtomicU32::new(refs as u32);
        (*this)._pad = 0;
    }
    this
}

unsafe extern "win64" fn facet_dtor(this: *mut Facet) {
    if !this.is_null() {
        (*this).vtable = FACET_VTABLE.as_ptr();
    }
}

unsafe extern "win64" fn facet_incref(this: *mut Facet) {
    if !this.is_null() {
        (*this).refs.fetch_add(1, Ordering::Relaxed);
    }
}

unsafe extern "win64" fn facet_decref(this: *mut Facet) -> *mut Facet {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    if (*this).refs.fetch_sub(1, Ordering::AcqRel) == 1 {
        this
    } else {
        std::ptr::null_mut()
    }
}

/// `std::locale::_Locimp` (`0x38` bytes, `0x180011840`).
#[repr(C)]
pub struct Locimp {
    pub vtable: *const Ptr,
    pub refs: AtomicU32,
    pub _pad0: u32,
    pub facetvec: *mut *mut Facet,
    pub facetcount: usize,
    pub catmask: i32,
    pub transparent: u8,
    pub _pad1: [u8; 3],
    pub name: *mut c_char,
    pub cloc_inline_name: [u8; 8],
}
const _: () = assert!(std::mem::size_of::<Locimp>() == 0x38);

static GLOBAL_LOCIMP: AtomicUsize = AtomicUsize::new(0);

unsafe fn get_or_init_global_locimp() -> *mut Locimp {
    ensure_vtables_registered();
    let cur = GLOBAL_LOCIMP.load(Ordering::Acquire);
    if cur != 0 {
        return cur as *mut Locimp;
    }
    let p = crate::ucrt::heap::calloc(1, std::mem::size_of::<Locimp>()) as *mut Locimp;
    (*p).vtable = FACET_VTABLE.as_ptr();
    (*p).refs = AtomicU32::new(100);
    (*p).catmask = 0x3f;
    (*p).cloc_inline_name[0] = b'C';
    if GLOBAL_LOCIMP
        .compare_exchange(0, p as usize, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        crate::ucrt::heap::free(p.cast());
        return GLOBAL_LOCIMP.load(Ordering::Acquire) as *mut Locimp;
    }
    p
}

unsafe extern "win64" fn locale_getgloballocale() -> *mut Locimp {
    get_or_init_global_locimp()
}

unsafe extern "win64" fn locale_init(do_incref: u8) -> *mut Locimp {
    let loc = get_or_init_global_locimp();
    if do_incref != 0 && !loc.is_null() {
        (*loc).refs.fetch_add(1, Ordering::Relaxed);
    }
    loc
}

unsafe extern "win64" fn locimp_new(src: *const Locimp) -> *mut Locimp {
    ensure_vtables_registered();
    let dst = crate::ucrt::heap::calloc(1, std::mem::size_of::<Locimp>()) as *mut Locimp;
    if dst.is_null() {
        return dst;
    }
    (*dst).vtable = FACET_VTABLE.as_ptr();
    (*dst).refs = AtomicU32::new(1);
    (*dst).catmask = 0x3f;
    (*dst).cloc_inline_name[0] = b'C';
    if !src.is_null() && (*src).facetcount > 0 && !(*src).facetvec.is_null() {
        let count = (*src).facetcount;
        let vec = crate::ucrt::heap::calloc(count, std::mem::size_of::<*mut Facet>()) as *mut *mut Facet;
        if !vec.is_null() {
            for i in 0..count {
                let f = *(*src).facetvec.add(i);
                if !f.is_null() {
                    facet_incref(f);
                }
                *vec.add(i) = f;
            }
            (*dst).facetvec = vec;
            (*dst).facetcount = count;
        }
    }
    dst
}

unsafe extern "win64" fn locimp_addfac(this: *mut Locimp, fac: *mut Facet, id: usize) {
    if this.is_null() || fac.is_null() {
        return;
    }
    if (*this).facetcount <= id {
        let new_count = (id + 1).max(40);
        let new_vec = crate::ucrt::heap::realloc(
            (*this).facetvec.cast(),
            new_count * std::mem::size_of::<*mut Facet>(),
        ) as *mut *mut Facet;
        if new_vec.is_null() {
            return;
        }
        for i in (*this).facetcount..new_count {
            *new_vec.add(i) = std::ptr::null_mut();
        }
        (*this).facetvec = new_vec;
        (*this).facetcount = new_count;
    }
    facet_incref(fac);
    let old = *(*this).facetvec.add(id);
    if !old.is_null() {
        let del = facet_decref(old);
        if !del.is_null() {
            let vtbl = (*del).vtable;
            if !vtbl.is_null() {
                let dtor = (*vtbl.add(0)).0 as usize;
                let _ = crate::call_guest8(dtor, del as u64, 1, 0, 0, 0, 0, 0, 0);
            }
        }
    }
    *(*this).facetvec.add(id) = fac;
}

/// `std::ctype<char>` (`0x30` bytes, `0x180006010`).
#[repr(C)]
pub struct CtypeChar {
    pub vtable: *const Ptr,
    pub refs: AtomicU32,
    pub _pad0: u32,
    pub page: u32,
    pub _pad1: u32,
    pub table: *const u16,
    pub delfl: i32,
    pub _pad2: i32,
    pub locale_name: *mut u16,
}
const _: () = assert!(std::mem::size_of::<CtypeChar>() == 0x30);

static CTYPE_CHAR_VTABLE: [Ptr; 11] = [
    Ptr(ctype_deleting_dtor as *const ()),
    Ptr(facet_incref as *const ()),
    Ptr(facet_decref as *const ()),
    Ptr(ctype_do_tolower_range as *const ()),
    Ptr(ctype_do_tolower_char as *const ()),
    Ptr(ctype_do_toupper_range as *const ()),
    Ptr(ctype_do_toupper_char as *const ()),
    Ptr(ctype_do_widen_range as *const ()),
    Ptr(ctype_do_widen_char as *const ()),
    Ptr(ctype_do_narrow_range as *const ()),
    Ptr(ctype_do_narrow_char as *const ()),
];

unsafe extern "win64" fn ctype_deleting_dtor(this: *mut CtypeChar, flags: u32) -> *mut CtypeChar {
    if !this.is_null() {
        (*this).vtable = FACET_VTABLE.as_ptr();
        if (flags & 1) != 0 {
            crate::ucrt::heap::free(this.cast());
        }
    }
    this
}

unsafe extern "win64" fn ctype_do_tolower_char(_this: *const CtypeChar, ch: u8) -> u8 {
    ch.to_ascii_lowercase()
}

unsafe extern "win64" fn ctype_do_tolower_range(
    _this: *const CtypeChar,
    mut first: *mut u8,
    last: *const u8,
) -> *const u8 {
    while (first as *const u8) < last {
        *first = (*first).to_ascii_lowercase();
        first = first.add(1);
    }
    last
}

unsafe extern "win64" fn ctype_do_toupper_char(_this: *const CtypeChar, ch: u8) -> u8 {
    ch.to_ascii_uppercase()
}

unsafe extern "win64" fn ctype_do_toupper_range(
    _this: *const CtypeChar,
    mut first: *mut u8,
    last: *const u8,
) -> *const u8 {
    while (first as *const u8) < last {
        *first = (*first).to_ascii_uppercase();
        first = first.add(1);
    }
    last
}

unsafe extern "win64" fn ctype_do_widen_char(_this: *const CtypeChar, ch: u8) -> u8 {
    ch
}

unsafe extern "win64" fn ctype_do_widen_range(
    _this: *const CtypeChar,
    first: *const u8,
    last: *const u8,
    dest: *mut u8,
) -> *const u8 {
    let n = (last as usize).saturating_sub(first as usize);
    if n > 0 && !first.is_null() && !dest.is_null() {
        std::ptr::copy_nonoverlapping(first, dest, n);
    }
    last
}

unsafe extern "win64" fn ctype_do_narrow_char(_this: *const CtypeChar, ch: u8, _dflt: u8) -> u8 {
    ch
}

unsafe extern "win64" fn ctype_do_narrow_range(
    _this: *const CtypeChar,
    first: *const u8,
    last: *const u8,
    _dflt: u8,
    dest: *mut u8,
) -> *const u8 {
    let n = (last as usize).saturating_sub(first as usize);
    if n > 0 && !first.is_null() && !dest.is_null() {
        std::ptr::copy_nonoverlapping(first, dest, n);
    }
    last
}

unsafe extern "win64" fn ctype_getcat(
    ppf: *mut *const Facet,
    _ploc: *const c_void,
) -> usize {
    ensure_vtables_registered();
    if !ppf.is_null() && (*ppf).is_null() {
        let p = crate::ucrt::heap::calloc(1, std::mem::size_of::<CtypeChar>()) as *mut CtypeChar;
        if !p.is_null() {
            (*p).vtable = CTYPE_CHAR_VTABLE.as_ptr();
            (*p).refs = AtomicU32::new(0);
            (*p).page = 0;
            (*p).table = crate::ucrt::string::__pctype_func();
            (*p).delfl = 0;
            *ppf = p.cast();
        }
    }
    2 // _X_CTYPE
}

unsafe extern "win64" fn ctype_tolower_char(this: *const CtypeChar, ch: u8) -> u8 {
    let _ = this;
    ch.to_ascii_lowercase()
}

unsafe extern "win64" fn ctype_tolower_range(
    this: *const CtypeChar,
    first: *mut u8,
    last: *const u8,
) -> *const u8 {
    ctype_do_tolower_range(this, first, last)
}

/// `std::codecvt<char32_t, char, mbstate_t>` (`0x18` bytes) & `std::codecvt<wchar_t, char, mbstate_t>` (`0x40` bytes).
#[repr(C)]
pub struct CodecvtU32 {
    pub vtable: *const Ptr,
    pub refs: AtomicU32,
    pub _pad: u32,
    pub maxcode: u32,
    pub mode: u32,
}

#[repr(C)]
pub struct CodecvtW {
    pub vtable: *const Ptr,
    pub refs: AtomicU32,
    pub _pad: u32,
    pub cvt: Cvtvec,
}

static CODECVT_U32_VTABLE: [Ptr; 10] = [
    Ptr(codecvt_deleting_dtor as *const ()),
    Ptr(facet_incref as *const ()),
    Ptr(facet_decref as *const ()),
    Ptr(codecvt_always_noconv as *const ()),
    Ptr(codecvt_max_length as *const ()),
    Ptr(codecvt_encoding as *const ()),
    Ptr(codecvt_u32_in as *const ()),
    Ptr(codecvt_u32_out as *const ()),
    Ptr(codecvt_unshift as *const ()),
    Ptr(codecvt_length as *const ()),
];

static CODECVT_W_VTABLE: [Ptr; 10] = [
    Ptr(codecvt_deleting_dtor as *const ()),
    Ptr(facet_incref as *const ()),
    Ptr(facet_decref as *const ()),
    Ptr(codecvt_always_noconv as *const ()),
    Ptr(codecvt_max_length as *const ()),
    Ptr(codecvt_encoding as *const ()),
    Ptr(codecvt_w_in as *const ()),
    Ptr(codecvt_w_out as *const ()),
    Ptr(codecvt_unshift as *const ()),
    Ptr(codecvt_length as *const ()),
];

unsafe extern "win64" fn codecvt_deleting_dtor(this: *mut Facet, flags: u32) -> *mut Facet {
    facet_deleting_dtor(this, flags)
}

unsafe extern "win64" fn codecvt_always_noconv(_this: *const c_void) -> u8 {
    0
}

unsafe extern "win64" fn codecvt_max_length(_this: *const c_void) -> i32 {
    4
}

unsafe extern "win64" fn codecvt_encoding(_this: *const c_void) -> i32 {
    0
}

unsafe extern "win64" fn codecvt_unshift(
    _this: *const c_void,
    _state: *mut u64,
    to: *mut u8,
    _to_end: *mut u8,
    to_next: *mut *mut u8,
) -> i32 {
    if !to_next.is_null() {
        std::ptr::write_unaligned(to_next, to);
    }
    3 // noconv
}

unsafe extern "win64" fn codecvt_length(
    _this: *const c_void,
    _state: *mut u64,
    from: *const u8,
    from_end: *const u8,
    max: usize,
) -> i32 {
    let avail = (from_end as usize).saturating_sub(from as usize);
    avail.min(max) as i32
}

unsafe extern "win64" fn codecvt_u32_ctor(this: *mut CodecvtU32, refs: usize) -> *mut CodecvtU32 {
    ensure_vtables_registered();
    if !this.is_null() {
        (*this).vtable = CODECVT_U32_VTABLE.as_ptr();
        (*this).refs = AtomicU32::new(refs as u32);
        (*this)._pad = 0;
        (*this).maxcode = 0x10ffff;
        (*this).mode = 4;
    }
    this
}

unsafe extern "win64" fn codecvt_w_ctor(this: *mut CodecvtW, refs: usize) -> *mut CodecvtW {
    ensure_vtables_registered();
    if !this.is_null() {
        (*this).vtable = CODECVT_W_VTABLE.as_ptr();
        (*this).refs = AtomicU32::new(refs as u32);
        (*this)._pad = 0;
        locinfo_getcvt(std::ptr::null(), &mut (*this).cvt);
    }
    this
}

unsafe extern "win64" fn codecvt_u32_in(
    _this: *const CodecvtU32,
    _state: *mut u64,
    from: *const u8,
    from_end: *const u8,
    from_next: *mut *const u8,
    to: *mut u32,
    to_end: *mut u32,
    to_next: *mut *mut u32,
) -> i32 {
    let mut f = from;
    let mut t = to;
    let mut res = 0; // ok
    while f < from_end && t < to_end {
        let rem = std::slice::from_raw_parts(f, (from_end as usize) - (f as usize));
        let b0 = rem[0];
        let needed = if b0 < 0x80 {
            1
        } else if (b0 & 0xe0) == 0xc0 {
            2
        } else if (b0 & 0xf0) == 0xe0 {
            3
        } else if (b0 & 0xf8) == 0xf0 {
            4
        } else {
            res = 2; // error
            break;
        };
        if rem.len() < needed {
            res = 1; // partial
            break;
        }
        match std::str::from_utf8(&rem[..needed]).ok().and_then(|s| s.chars().next()) {
            Some(ch) => {
                std::ptr::write_unaligned(t, ch as u32);
                f = f.add(needed);
                t = t.add(1);
            }
            None => {
                res = 2;
                break;
            }
        }
    }
    if !from_next.is_null() {
        std::ptr::write_unaligned(from_next, f);
    }
    if !to_next.is_null() {
        std::ptr::write_unaligned(to_next, t);
    }
    res
}

unsafe extern "win64" fn codecvt_u32_out(
    _this: *const CodecvtU32,
    _state: *mut u64,
    from: *const u32,
    from_end: *const u32,
    from_next: *mut *const u32,
    to: *mut u8,
    to_end: *mut u8,
    to_next: *mut *mut u8,
) -> i32 {
    let mut f = from;
    let mut t = to;
    let mut res = 0;
    let mut buf = [0u8; 4];
    while f < from_end && t < to_end {
        let cp = std::ptr::read_unaligned(f);
        let Some(ch) = char::from_u32(cp) else {
            res = 2;
            break;
        };
        let enc = ch.encode_utf8(&mut buf).as_bytes();
        let avail = (to_end as usize).saturating_sub(t as usize);
        if avail < enc.len() {
            res = 1; // partial
            break;
        }
        std::ptr::copy_nonoverlapping(enc.as_ptr(), t, enc.len());
        t = t.add(enc.len());
        f = f.add(1);
    }
    if !from_next.is_null() {
        std::ptr::write_unaligned(from_next, f);
    }
    if !to_next.is_null() {
        std::ptr::write_unaligned(to_next, t);
    }
    res
}

unsafe extern "win64" fn codecvt_w_in(
    _this: *const CodecvtW,
    _state: *mut u64,
    from: *const u8,
    from_end: *const u8,
    from_next: *mut *const u8,
    to: *mut u16,
    to_end: *mut u16,
    to_next: *mut *mut u16,
) -> i32 {
    let mut f = from;
    let mut t = to;
    while f < from_end && t < to_end {
        std::ptr::write_unaligned(t, *f as u16);
        f = f.add(1);
        t = t.add(1);
    }
    if !from_next.is_null() {
        std::ptr::write_unaligned(from_next, f);
    }
    if !to_next.is_null() {
        std::ptr::write_unaligned(to_next, t);
    }
    0
}

unsafe extern "win64" fn codecvt_w_out(
    _this: *const CodecvtW,
    _state: *mut u64,
    from: *const u16,
    from_end: *const u16,
    from_next: *mut *const u16,
    to: *mut u8,
    to_end: *mut u8,
    to_next: *mut *mut u8,
) -> i32 {
    let mut f = from;
    let mut t = to;
    let mut buf = [0u8; 4];
    while f < from_end && t < to_end {
        let u = std::ptr::read_unaligned(f);
        let ch = char::from_u32(u as u32).unwrap_or('?');
        let enc = ch.encode_utf8(&mut buf).as_bytes();
        let avail = (to_end as usize).saturating_sub(t as usize);
        if avail < enc.len() {
            break;
        }
        std::ptr::copy_nonoverlapping(enc.as_ptr(), t, enc.len());
        t = t.add(enc.len());
        f = f.add(1);
    }
    if !from_next.is_null() {
        std::ptr::write_unaligned(from_next, f);
    }
    if !to_next.is_null() {
        std::ptr::write_unaligned(to_next, t);
    }
    0
}

// ---------------------------------------------------------------------------
// 7. Streams (`basic_streambuf`, `basic_ios`, `basic_istream`, `basic_ostream`,
//    `basic_iostream`, `std::cout`, `std::cerr`)
// ---------------------------------------------------------------------------

/// `std::basic_streambuf<char>` (`0x68` bytes, `0x180003c90`).
#[repr(C)]
pub struct BasicStreambuf {
    pub vtable: *const Ptr,
    pub gfirst: *mut u8,
    pub pfirst: *mut u8,
    pub igfirst: *mut *mut u8,
    pub ipfirst: *mut *mut u8,
    pub gnext: *mut u8,
    pub pnext: *mut u8,
    pub ignext: *mut *mut u8,
    pub ipnext: *mut *mut u8,
    pub gcount: i32,
    pub pcount: i32,
    pub igcount: *mut i32,
    pub ipcount: *mut i32,
    pub plocale: *mut [usize; 2],
}
const _: () = assert!(std::mem::size_of::<BasicStreambuf>() == 0x68);

static BASIC_STREAMBUF_VTABLE: [Ptr; 15] = [
    Ptr(streambuf_deleting_dtor as *const ()),
    Ptr(streambuf_lock as *const ()),
    Ptr(streambuf_unlock as *const ()),
    Ptr(streambuf_overflow as *const ()),
    Ptr(streambuf_pbackfail as *const ()),
    Ptr(streambuf_showmanyc as *const ()),
    Ptr(streambuf_underflow as *const ()),
    Ptr(streambuf_uflow as *const ()),
    Ptr(streambuf_xsgetn as *const ()),
    Ptr(streambuf_xsputn as *const ()),
    Ptr(streambuf_seekoff as *const ()),
    Ptr(streambuf_seekpos as *const ()),
    Ptr(streambuf_setbuf as *const ()),
    Ptr(streambuf_sync as *const ()),
    Ptr(streambuf_imbue as *const ()),
];

static STDIO_STREAMBUF_VTABLE: [Ptr; 15] = [
    Ptr(streambuf_deleting_dtor as *const ()),
    Ptr(streambuf_lock as *const ()),
    Ptr(streambuf_unlock as *const ()),
    Ptr(stdio_streambuf_overflow as *const ()),
    Ptr(streambuf_pbackfail as *const ()),
    Ptr(streambuf_showmanyc as *const ()),
    Ptr(streambuf_underflow as *const ()),
    Ptr(streambuf_uflow as *const ()),
    Ptr(streambuf_xsgetn as *const ()),
    Ptr(stdio_streambuf_xsputn as *const ()),
    Ptr(streambuf_seekoff as *const ()),
    Ptr(streambuf_seekpos as *const ()),
    Ptr(streambuf_setbuf as *const ()),
    Ptr(streambuf_sync as *const ()),
    Ptr(streambuf_imbue as *const ()),
];

unsafe extern "win64" fn streambuf_ctor(this: *mut BasicStreambuf) -> *mut BasicStreambuf {
    ensure_vtables_registered();
    if !this.is_null() {
        (*this).vtable = BASIC_STREAMBUF_VTABLE.as_ptr();
        (*this).gfirst = std::ptr::null_mut();
        (*this).pfirst = std::ptr::null_mut();
        (*this).igfirst = &mut (*this).gfirst;
        (*this).ipfirst = &mut (*this).pfirst;
        (*this).gnext = std::ptr::null_mut();
        (*this).pnext = std::ptr::null_mut();
        (*this).ignext = &mut (*this).gnext;
        (*this).ipnext = &mut (*this).pnext;
        (*this).gcount = 0;
        (*this).pcount = 0;
        (*this).igcount = &mut (*this).gcount;
        (*this).ipcount = &mut (*this).pcount;
        let loc = crate::ucrt::heap::calloc(1, 16) as *mut [usize; 2];
        if !loc.is_null() {
            (*loc)[1] = locale_init(1) as usize;
        }
        (*this).plocale = loc;
    }
    this
}

unsafe extern "win64" fn streambuf_dtor(this: *mut BasicStreambuf) {
    if !this.is_null() {
        (*this).vtable = BASIC_STREAMBUF_VTABLE.as_ptr();
        if !(*this).plocale.is_null() {
            crate::ucrt::heap::free((*this).plocale.cast());
            (*this).plocale = std::ptr::null_mut();
        }
    }
}

unsafe extern "win64" fn streambuf_deleting_dtor(
    this: *mut BasicStreambuf,
    flags: u32,
) -> *mut BasicStreambuf {
    streambuf_dtor(this);
    if (flags & 1) != 0 && !this.is_null() {
        crate::ucrt::heap::free(this.cast());
    }
    this
}

unsafe extern "win64" fn streambuf_lock(_this: *mut BasicStreambuf) {}
unsafe extern "win64" fn streambuf_unlock(_this: *mut BasicStreambuf) {}
unsafe extern "win64" fn streambuf_overflow(_this: *mut BasicStreambuf, _ch: i32) -> i32 {
    -1
}
unsafe extern "win64" fn streambuf_pbackfail(_this: *mut BasicStreambuf, _ch: i32) -> i32 {
    -1
}
unsafe extern "win64" fn streambuf_showmanyc(_this: *mut BasicStreambuf) -> i64 {
    0
}
unsafe extern "win64" fn streambuf_underflow(_this: *mut BasicStreambuf) -> i32 {
    -1
}

unsafe extern "win64" fn streambuf_uflow(this: *mut BasicStreambuf) -> i32 {
    if this.is_null() {
        return -1;
    }
    let vtable = (*this).vtable;
    let underflow = (*vtable.add(6)).0 as usize;
    if crate::call_guest8(underflow, this as u64, 0, 0, 0, 0, 0, 0, 0) as i32 == -1 {
        return -1;
    }
    let cnt = (*this).igcount;
    let nxt = (*this).ignext;
    if cnt.is_null() || nxt.is_null() || (*nxt).is_null() {
        return -1;
    }
    *cnt -= 1;
    let p = *nxt;
    *nxt = p.add(1);
    *p as i32
}

unsafe extern "win64" fn streambuf_xsgetn(
    this: *mut BasicStreambuf,
    mut dst: *mut u8,
    count: i64,
) -> i64 {
    if this.is_null() || dst.is_null() || count <= 0 {
        return 0;
    }
    let mut rem = count;
    while rem > 0 {
        let avail = if !(*this).ignext.is_null()
            && !(*(*this).ignext).is_null()
            && !(*this).igcount.is_null()
        {
            (*(*this).igcount).max(0) as i64
        } else {
            0
        };
        if avail > 0 {
            let n = rem.min(avail) as usize;
            let src = *(*this).ignext;
            std::ptr::copy_nonoverlapping(src, dst, n);
            *(*this).ignext = src.add(n);
            *(*this).igcount -= n as i32;
            dst = dst.add(n);
            rem -= n as i64;
        } else {
            let vtable = (*this).vtable;
            let uflow = (*vtable.add(7)).0 as usize;
            let c = crate::call_guest8(uflow, this as u64, 0, 0, 0, 0, 0, 0, 0) as i32;
            if c == -1 {
                break;
            }
            *dst = c as u8;
            dst = dst.add(1);
            rem -= 1;
        }
    }
    count - rem
}

unsafe extern "win64" fn streambuf_xsputn(
    this: *mut BasicStreambuf,
    mut src: *const u8,
    count: i64,
) -> i64 {
    if this.is_null() || src.is_null() || count <= 0 {
        return 0;
    }
    let mut rem = count;
    while rem > 0 {
        let avail = if !(*this).ipnext.is_null()
            && !(*(*this).ipnext).is_null()
            && !(*this).ipcount.is_null()
        {
            (*(*this).ipcount).max(0) as i64
        } else {
            0
        };
        if avail > 0 {
            let n = rem.min(avail) as usize;
            let dst = *(*this).ipnext;
            std::ptr::copy_nonoverlapping(src, dst, n);
            *(*this).ipnext = dst.add(n);
            *(*this).ipcount -= n as i32;
            src = src.add(n);
            rem -= n as i64;
        } else {
            let vtable = (*this).vtable;
            let overflow = (*vtable.add(3)).0 as usize;
            if crate::call_guest8(overflow, this as u64, *src as u64, 0, 0, 0, 0, 0, 0) as i32 == -1 {
                break;
            }
            src = src.add(1);
            rem -= 1;
        }
    }
    count - rem
}

unsafe extern "win64" fn streambuf_seekoff(
    _this: *mut BasicStreambuf,
    out: *mut [i64; 3],
) -> *mut [i64; 3] {
    if !out.is_null() {
        std::ptr::write_unaligned(out, [-1, 0, 0]);
    }
    out
}

unsafe extern "win64" fn streambuf_seekpos(
    _this: *mut BasicStreambuf,
    out: *mut [i64; 3],
) -> *mut [i64; 3] {
    if !out.is_null() {
        std::ptr::write_unaligned(out, [-1, 0, 0]);
    }
    out
}

unsafe extern "win64" fn streambuf_setbuf(
    this: *mut BasicStreambuf,
    _buf: *mut u8,
    _count: i64,
) -> *mut BasicStreambuf {
    this
}

unsafe extern "win64" fn streambuf_sync(_this: *mut BasicStreambuf) -> i32 {
    0
}

unsafe extern "win64" fn streambuf_imbue(_this: *mut BasicStreambuf, _loc: *const c_void) {}

unsafe extern "win64" fn streambuf_sbumpc(this: *mut BasicStreambuf) -> i32 {
    if this.is_null() {
        return -1;
    }
    if !(*this).ignext.is_null()
        && !(*(*this).ignext).is_null()
        && !(*this).igcount.is_null()
        && *(*this).igcount > 0
    {
        *(*this).igcount -= 1;
        let p = *(*this).ignext;
        *(*this).ignext = p.add(1);
        *p as i32
    } else {
        let vtable = (*this).vtable;
        let uflow = (*vtable.add(7)).0 as usize;
        crate::call_guest8(uflow, this as u64, 0, 0, 0, 0, 0, 0, 0) as i32
    }
}

unsafe extern "win64" fn streambuf_sgetc(this: *mut BasicStreambuf) -> i32 {
    if this.is_null() {
        return -1;
    }
    if !(*this).ignext.is_null()
        && !(*(*this).ignext).is_null()
        && !(*this).igcount.is_null()
        && *(*this).igcount > 0
    {
        *(*(*this).ignext) as i32
    } else {
        let vtable = (*this).vtable;
        let underflow = (*vtable.add(6)).0 as usize;
        crate::call_guest8(underflow, this as u64, 0, 0, 0, 0, 0, 0, 0) as i32
    }
}

unsafe extern "win64" fn streambuf_snextc(this: *mut BasicStreambuf) -> i32 {
    if streambuf_sbumpc(this) == -1 {
        -1
    } else {
        streambuf_sgetc(this)
    }
}

unsafe extern "win64" fn streambuf_sputc(this: *mut BasicStreambuf, ch: u8) -> i32 {
    if this.is_null() {
        return -1;
    }
    if !(*this).ipnext.is_null()
        && !(*(*this).ipnext).is_null()
        && !(*this).ipcount.is_null()
        && *(*this).ipcount > 0
    {
        *(*this).ipcount -= 1;
        let p = *(*this).ipnext;
        *(*this).ipnext = p.add(1);
        *p = ch;
        ch as i32
    } else {
        let vtable = (*this).vtable;
        let overflow = (*vtable.add(3)).0 as usize;
        crate::call_guest8(overflow, this as u64, ch as u64, 0, 0, 0, 0, 0, 0) as i32
    }
}

unsafe extern "win64" fn streambuf_sputn(
    this: *mut BasicStreambuf,
    s: *const u8,
    count: i64,
) -> i64 {
    if this.is_null() {
        return 0;
    }
    let vtable = (*this).vtable;
    let xsputn = (*vtable.add(9)).0 as usize;
    crate::call_guest8(xsputn, this as u64, s as u64, count as u64, 0, 0, 0, 0, 0) as i64
}

unsafe extern "win64" fn stdio_streambuf_overflow(_this: *mut BasicStreambuf, ch: i32) -> i32 {
    if ch == -1 {
        return 0;
    }
    let b = ch as u8;
    let _ = libc::write(1, (&b as *const u8).cast(), 1);
    ch
}

unsafe extern "win64" fn stdio_streambuf_xsputn(
    _this: *mut BasicStreambuf,
    src: *const u8,
    count: i64,
) -> i64 {
    if src.is_null() || count <= 0 {
        return 0;
    }
    let _ = libc::write(1, src.cast(), count as usize);
    count
}

/// `std::basic_ios<char>` (`0x60` bytes, `0x180003bb0`).
#[repr(C)]
pub struct BasicIos {
    pub vtable: *const Ptr,
    pub stdstr: usize,
    pub mystate: i32,
    pub except: i32,
    pub fmtfl: i32,
    pub _pad0: i32,
    pub prec: i64,
    pub wide: i64,
    pub arr: *mut c_void,
    pub calls: *mut c_void,
    pub ploc: *mut [usize; 2],
    pub mystrbuf: *mut BasicStreambuf,
    pub tiestr: *mut c_void,
    pub fillch: u8,
    pub _pad1: [u8; 7],
}
const _: () = assert!(std::mem::size_of::<BasicIos>() == 0x60);

static BASIC_IOS_VTABLE: [Ptr; 1] = [Ptr(ios_deleting_dtor as *const ())];

static ISTREAM_VBTABLE: [i32; 2] = [0, 0x18];
static OSTREAM_VBTABLE: [i32; 2] = [0, 0x10];
static IOSTREAM_IN_VBTABLE: [i32; 2] = [0, 0x20];
static IOSTREAM_OUT_VBTABLE: [i32; 2] = [0, 0x10];

unsafe extern "win64" fn ios_ctor(this: *mut BasicIos) -> *mut BasicIos {
    ensure_vtables_registered();
    if !this.is_null() {
        std::ptr::write_bytes(this, 0, 1);
        (*this).vtable = BASIC_IOS_VTABLE.as_ptr();
    }
    this
}

unsafe extern "win64" fn ios_dtor(this: *mut BasicIos) {
    if !this.is_null() {
        (*this).vtable = BASIC_IOS_VTABLE.as_ptr();
        if !(*this).ploc.is_null() {
            crate::ucrt::heap::free((*this).ploc.cast());
            (*this).ploc = std::ptr::null_mut();
        }
    }
}

unsafe extern "win64" fn ios_deleting_dtor(this: *mut BasicIos, flags: u32) -> *mut BasicIos {
    ios_dtor(this);
    if (flags & 1) != 0 && !this.is_null() {
        crate::ucrt::heap::free(this.cast());
    }
    this
}

unsafe fn ios_init(ios: *mut BasicIos, sb: *mut BasicStreambuf) {
    if ios.is_null() {
        return;
    }
    (*ios).vtable = BASIC_IOS_VTABLE.as_ptr();
    (*ios).mystate = if sb.is_null() { 4 } else { 0 };
    (*ios).except = 0;
    (*ios).fmtfl = 0x201;
    (*ios).prec = 6;
    (*ios).wide = 0;
    let loc = crate::ucrt::heap::calloc(1, 16) as *mut [usize; 2];
    if !loc.is_null() {
        (*loc)[1] = locale_init(1) as usize;
    }
    (*ios).ploc = loc;
    (*ios).mystrbuf = sb;
    (*ios).tiestr = std::ptr::null_mut();
    (*ios).fillch = b' ';
}

unsafe extern "win64" fn ios_good(this: *const BasicIos) -> u8 {
    (!this.is_null() && (*this).mystate == 0) as u8
}

unsafe extern "win64" fn ios_setstate(this: *mut BasicIos, state: i32, _reraise: u8) {
    if this.is_null() {
        return;
    }
    let extra = if (*this).mystrbuf.is_null() { 4 } else { 0 };
    (*this).mystate = ((*this).mystate | state | extra) & 0x17;
}

unsafe extern "win64" fn ios_widen(_this: *const BasicIos, ch: u8) -> u8 {
    ch
}

#[inline]
unsafe fn stream_ios(stream: *mut u8) -> *mut BasicIos {
    let vbtable = std::ptr::read_unaligned(stream as *const *const i32);
    if vbtable.is_null() {
        return std::ptr::null_mut();
    }
    let off = std::ptr::read_unaligned(vbtable.add(1)) as isize;
    stream.wrapping_offset(off) as *mut BasicIos
}

unsafe extern "win64" fn istream_ctor(
    this: *mut u8,
    sb: *mut BasicStreambuf,
    _isstd: u8,
    most_derived: i32,
) -> *mut u8 {
    ensure_vtables_registered();
    if this.is_null() {
        return this;
    }
    if most_derived != 0 {
        std::ptr::write_unaligned(this as *mut *const i32, ISTREAM_VBTABLE.as_ptr());
        std::ptr::write_bytes(this.add(0x18), 0, std::mem::size_of::<BasicIos>());
    }
    std::ptr::write_unaligned(this.add(8) as *mut u64, 0);
    let ios = stream_ios(this);
    if !ios.is_null() {
        std::ptr::write_unaligned((ios as *mut u8).sub(4) as *mut i32, 0);
        ios_init(ios, sb);
    }
    this
}

unsafe extern "win64" fn istream_dtor(_this_ios: *mut u8) {}

unsafe extern "win64" fn istream_ipfx(this: *mut u8, noskip: u8) -> u8 {
    if this.is_null() {
        return 0;
    }
    let ios = stream_ios(this);
    if ios.is_null() || (*ios).mystate != 0 || (*ios).mystrbuf.is_null() {
        if !ios.is_null() {
            ios_setstate(ios, 2, 0);
        }
        return 0;
    }
    if noskip == 0 && ((*ios).fmtfl & 1) != 0 {
        let sb = (*ios).mystrbuf;
        loop {
            let c = streambuf_sgetc(sb);
            if c == -1 {
                ios_setstate(ios, 1, 0);
                break;
            }
            if !(c as u8).is_ascii_whitespace() {
                break;
            }
            let _ = streambuf_sbumpc(sb);
        }
    }
    ((*ios).mystate == 0) as u8
}

unsafe extern "win64" fn iostream_ctor(
    this: *mut u8,
    sb: *mut BasicStreambuf,
    most_derived: i32,
) -> *mut u8 {
    ensure_vtables_registered();
    if this.is_null() {
        return this;
    }
    if most_derived != 0 {
        std::ptr::write_unaligned(this as *mut *const i32, IOSTREAM_IN_VBTABLE.as_ptr());
        std::ptr::write_unaligned(this.add(0x10) as *mut *const i32, IOSTREAM_OUT_VBTABLE.as_ptr());
        std::ptr::write_bytes(this.add(0x20), 0, std::mem::size_of::<BasicIos>());
    }
    std::ptr::write_unaligned(this.add(8) as *mut u64, 0);
    let ios = stream_ios(this);
    if !ios.is_null() {
        std::ptr::write_unaligned((ios as *mut u8).sub(4) as *mut i32, 0);
        ios_init(ios, sb);
    }
    this
}

unsafe extern "win64" fn iostream_dtor(_this_ios: *mut u8) {}

unsafe extern "win64" fn ostream_osfx(this: *mut u8) {
    if this.is_null() {
        return;
    }
    let ios = stream_ios(this);
    if !ios.is_null() && (*ios).mystate == 0 && ((*ios).fmtfl & 2) != 0 && !(*ios).mystrbuf.is_null() {
        let sb = (*ios).mystrbuf;
        let vtable = (*sb).vtable;
        let sync = (*vtable.add(13)).0 as usize;
        if crate::call_guest8(sync, sb as u64, 0, 0, 0, 0, 0, 0, 0) as i32 == -1 {
            ios_setstate(ios, 4, 0);
        }
    }
}

unsafe extern "win64" fn ostream_flush(this: *mut u8) -> *mut u8 {
    if this.is_null() {
        return this;
    }
    let ios = stream_ios(this);
    if !ios.is_null() && !(*ios).mystrbuf.is_null() {
        let sb = (*ios).mystrbuf;
        let vtable = (*sb).vtable;
        let sync = (*vtable.add(13)).0 as usize;
        if crate::call_guest8(sync, sb as u64, 0, 0, 0, 0, 0, 0, 0) as i32 == -1 {
            ios_setstate(ios, 4, 0);
        }
    }
    this
}

unsafe extern "win64" fn ostream_put(this: *mut u8, ch: u8) -> *mut u8 {
    if this.is_null() {
        return this;
    }
    let ios = stream_ios(this);
    if !ios.is_null() && !(*ios).mystrbuf.is_null() {
        if streambuf_sputc((*ios).mystrbuf, ch) == -1 {
            ios_setstate(ios, 4, 0);
        }
    }
    this
}

unsafe extern "win64" fn ostream_write(this: *mut u8, s: *const u8, count: i64) -> *mut u8 {
    if this.is_null() || s.is_null() || count <= 0 {
        return this;
    }
    let ios = stream_ios(this);
    if !ios.is_null() && !(*ios).mystrbuf.is_null() {
        if streambuf_sputn((*ios).mystrbuf, s, count) != count {
            ios_setstate(ios, 4, 0);
        }
    }
    this
}

unsafe fn ostream_write_str(this: *mut u8, s: &str) -> *mut u8 {
    ostream_write(this, s.as_ptr(), s.len() as i64)
}

unsafe extern "win64" fn ostream_shl_manip(
    this: *mut u8,
    manip: usize,
) -> *mut u8 {
    if manip == 0 {
        return this;
    }
    crate::call_guest8(manip, this as u64, 0, 0, 0, 0, 0, 0, 0) as *mut u8
}

unsafe extern "win64" fn ostream_shl_u16(this: *mut u8, v: u16) -> *mut u8 {
    let s = v.to_string();
    ostream_write_str(this, &s)
}

unsafe extern "win64" fn ostream_shl_i32(this: *mut u8, v: i32) -> *mut u8 {
    let s = v.to_string();
    ostream_write_str(this, &s)
}

unsafe extern "win64" fn ostream_shl_f64(this: *mut u8, v: f64) -> *mut u8 {
    let s = format!("{v}");
    ostream_write_str(this, &s)
}
crate::win64_bridge!(fn ostream_shl_f64(*mut u8, f64) -> *mut u8);

unsafe extern "win64" fn ostream_shl_i64(this: *mut u8, v: i64) -> *mut u8 {
    let s = v.to_string();
    ostream_write_str(this, &s)
}

unsafe extern "win64" fn ostream_shl_u64(this: *mut u8, v: u64) -> *mut u8 {
    let s = v.to_string();
    ostream_write_str(this, &s)
}

#[repr(C, align(16))]
pub struct StaticOstream {
    pub vbtable: *const i32,
    pub vtordisp: u64,
    pub ios: BasicIos,
}
unsafe impl Sync for StaticOstream {}
unsafe impl Sync for BasicStreambuf {}

pub struct SyncStreambuf(UnsafeCell<BasicStreambuf>);
unsafe impl Sync for SyncStreambuf {}

static COUT_BUF: SyncStreambuf = SyncStreambuf(UnsafeCell::new(BasicStreambuf {
    vtable: STDIO_STREAMBUF_VTABLE.as_ptr(),
    gfirst: std::ptr::null_mut(),
    pfirst: std::ptr::null_mut(),
    igfirst: std::ptr::null_mut(),
    ipfirst: std::ptr::null_mut(),
    gnext: std::ptr::null_mut(),
    pnext: std::ptr::null_mut(),
    ignext: std::ptr::null_mut(),
    ipnext: std::ptr::null_mut(),
    gcount: 0,
    pcount: 0,
    igcount: std::ptr::null_mut(),
    ipcount: std::ptr::null_mut(),
    plocale: std::ptr::null_mut(),
}));

pub static COUT: StaticOstream = StaticOstream {
    vbtable: OSTREAM_VBTABLE.as_ptr(),
    vtordisp: 0,
    ios: BasicIos {
        vtable: BASIC_IOS_VTABLE.as_ptr(),
        stdstr: 0,
        mystate: 0,
        except: 0,
        fmtfl: 0x201,
        _pad0: 0,
        prec: 6,
        wide: 0,
        arr: std::ptr::null_mut(),
        calls: std::ptr::null_mut(),
        ploc: std::ptr::null_mut(),
        mystrbuf: COUT_BUF.0.get(),
        tiestr: std::ptr::null_mut(),
        fillch: b' ',
        _pad1: [0; 7],
    },
};

pub static CERR: StaticOstream = StaticOstream {
    vbtable: OSTREAM_VBTABLE.as_ptr(),
    vtordisp: 0,
    ios: BasicIos {
        vtable: BASIC_IOS_VTABLE.as_ptr(),
        stdstr: 0,
        mystate: 0,
        except: 0,
        fmtfl: 0x203,
        _pad0: 0,
        prec: 6,
        wide: 0,
        arr: std::ptr::null_mut(),
        calls: std::ptr::null_mut(),
        ploc: std::ptr::null_mut(),
        mystrbuf: COUT_BUF.0.get(),
        tiestr: std::ptr::null_mut(),
        fillch: b' ',
        _pad1: [0; 7],
    },
};

fn ensure_vtables_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        crate::register_native_va(
            chore_work_callback as *const () as usize as u64,
            chore_work_callback as *const () as usize,
        );
        for p in FACET_VTABLE
            .iter()
            .chain(CTYPE_CHAR_VTABLE.iter())
            .chain(CODECVT_U32_VTABLE.iter())
            .chain(CODECVT_W_VTABLE.iter())
            .chain(BASIC_STREAMBUF_VTABLE.iter())
            .chain(STDIO_STREAMBUF_VTABLE.iter())
            .chain(BASIC_IOS_VTABLE.iter())
        {
            let fn_addr = p.0 as usize;
            crate::register_native_va(fn_addr as u64, fn_addr);
        }
    });
}

pub static ID_CODECVT_D: LocaleId = LocaleId(UnsafeCell::new([5, 0]));
pub static ID_NUMPUNCT_W: LocaleId = LocaleId(UnsafeCell::new([6, 0]));
pub static ID_TIME_PUT_D: LocaleId = LocaleId(UnsafeCell::new([7, 0]));
pub static LOCALE_ID_CNT: AtomicU32 = AtomicU32::new(8);

static W_DAYS_STR: [u16; 15] = [
    b':' as u16, b'S' as u16, b'u' as u16, b'n' as u16, b':' as u16, b'S' as u16, b'u' as u16,
    b'n' as u16, b'd' as u16, b'a' as u16, b'y' as u16, b':' as u16, b'M' as u16, b'o' as u16, 0,
];
static W_MONTHS_STR: [u16; 16] = [
    b':' as u16, b'J' as u16, b'a' as u16, b'n' as u16, b':' as u16, b'J' as u16, b'a' as u16,
    b'n' as u16, b'u' as u16, b'a' as u16, b'r' as u16, b'y' as u16, b':' as u16, b'F' as u16,
    b'e' as u16, 0,
];

unsafe extern "win64" fn locinfo_w_getdays(_this: *const u8) -> *const u16 {
    W_DAYS_STR.as_ptr()
}
unsafe extern "win64" fn locinfo_w_getmonths(_this: *const u8) -> *const u16 {
    W_MONTHS_STR.as_ptr()
}
unsafe extern "win64" fn yarn_c_str(this: *const *const c_char) -> *const c_char {
    if this.is_null() {
        return c"".as_ptr();
    }
    let p = std::ptr::read_unaligned(this);
    if p.is_null() { c"".as_ptr() } else { p }
}

unsafe extern "win64" fn ostream_ctor(
    this: *mut u8,
    sb: *mut BasicStreambuf,
    _isstd: u8,
    most_derived: i32,
) -> *mut u8 {
    ensure_vtables_registered();
    if this.is_null() {
        return this;
    }
    if most_derived != 0 {
        std::ptr::write_unaligned(this as *mut *const i32, OSTREAM_VBTABLE.as_ptr());
        std::ptr::write_bytes(this.add(0x10), 0, std::mem::size_of::<BasicIos>());
    }
    let ios = stream_ios(this);
    if !ios.is_null() {
        std::ptr::write_unaligned((ios as *mut u8).sub(4) as *mut i32, 0);
        ios_init(ios, sb);
    }
    this
}
unsafe extern "win64" fn ostream_dtor(_this_ios: *mut u8) {}

unsafe extern "win64" fn ostream_shl_u32(this: *mut u8, v: u32) -> *mut u8 {
    let s = v.to_string();
    ostream_write_str(this, &s)
}
unsafe extern "win64" fn ostream_shl_bool(this: *mut u8, v: u8) -> *mut u8 {
    let s = if v != 0 { "1" } else { "0" };
    ostream_write_str(this, s)
}
unsafe extern "win64" fn ostream_shl_f32(this: *mut u8, v: f32) -> *mut u8 {
    let s = format!("{v}");
    ostream_write_str(this, &s)
}
crate::win64_bridge!(fn ostream_shl_f32(*mut u8, f32) -> *mut u8);

unsafe extern "win64" fn fiopen_a(filename: *const c_char, mode: i32, _prot: i32) -> *mut c_void {
    let m = if (mode & 2) != 0 { c"wb".as_ptr() } else { c"rb".as_ptr() };
    crate::ucrt::stdio::fopen(filename.cast(), m.cast()).cast()
}
unsafe extern "win64" fn fiopen_w(filename: *const u16, mode: i32, _prot: i32) -> *mut c_void {
    let r_mode: [u16; 3] = [b'r' as u16, b'b' as u16, 0];
    let w_mode: [u16; 3] = [b'w' as u16, b'b' as u16, 0];
    let m = if (mode & 2) != 0 { w_mode.as_ptr() } else { r_mode.as_ptr() };
    crate::ucrt::stdio::_wfopen(filename, m).cast()
}

unsafe extern "win64" fn codecvt_d_getcat(ppf: *mut *const Facet, _ploc: *const c_void) -> usize {
    ensure_vtables_registered();
    if !ppf.is_null() && (*ppf).is_null() {
        let p = crate::ucrt::heap::calloc(1, std::mem::size_of::<CodecvtU32>()) as *mut CodecvtU32;
        if !p.is_null() {
            (*p).vtable = CODECVT_U32_VTABLE.as_ptr();
            (*p).refs = AtomicU32::new(0);
            *ppf = p.cast();
        }
    }
    2
}
unsafe extern "win64" fn time_put_getcat(ppf: *mut *const Facet, _ploc: *const c_void) -> usize {
    ensure_vtables_registered();
    if !ppf.is_null() && (*ppf).is_null() {
        let p = crate::ucrt::heap::calloc(1, std::mem::size_of::<Facet>()) as *mut Facet;
        if !p.is_null() {
            (*p).vtable = FACET_VTABLE.as_ptr();
            (*p).refs = AtomicU32::new(0);
            *ppf = p;
        }
    }
    5
}
unsafe extern "win64" fn codecvt_d_in_out(
    _this: *const c_void,
    _state: *mut u64,
    from: *const u8,
    _from_end: *const u8,
    from_next: *mut *const u8,
    to: *mut u8,
    _to_end: *mut u8,
    to_next: *mut *mut u8,
) -> i32 {
    if !from_next.is_null() {
        std::ptr::write_unaligned(from_next, from);
    }
    if !to_next.is_null() {
        std::ptr::write_unaligned(to_next, to);
    }
    3 // noconv
}

unsafe extern "win64" fn streambuf_init_empty(this: *mut BasicStreambuf) {
    if !this.is_null() {
        (*this).gfirst = std::ptr::null_mut();
        (*this).pfirst = std::ptr::null_mut();
        (*this).igfirst = &mut (*this).gfirst;
        (*this).ipfirst = &mut (*this).pfirst;
        (*this).gnext = std::ptr::null_mut();
        (*this).pnext = std::ptr::null_mut();
        (*this).ignext = &mut (*this).gnext;
        (*this).ipnext = &mut (*this).pnext;
        (*this).gcount = 0;
        (*this).pcount = 0;
        (*this).igcount = &mut (*this).gcount;
        (*this).ipcount = &mut (*this).pcount;
    }
}
unsafe extern "win64" fn streambuf_pninc(this: *mut BasicStreambuf) -> *mut u8 {
    if this.is_null() || (*this).ipnext.is_null() || (*(*this).ipnext).is_null() {
        return std::ptr::null_mut();
    }
    if !(*this).ipcount.is_null() {
        *(*this).ipcount -= 1;
    }
    let p = *(*this).ipnext;
    *(*this).ipnext = p.add(1);
    p
}
unsafe extern "win64" fn streambuf_eback(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() || (*this).igfirst.is_null() { std::ptr::null_mut() } else { *(*this).igfirst }
}
unsafe extern "win64" fn streambuf_gptr(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() || (*this).ignext.is_null() { std::ptr::null_mut() } else { *(*this).ignext }
}
unsafe extern "win64" fn streambuf_egptr(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() || (*this).ignext.is_null() || (*(*this).ignext).is_null() || (*this).igcount.is_null() {
        std::ptr::null_mut()
    } else {
        (*(*this).ignext).wrapping_offset((*(*this).igcount) as isize)
    }
}
unsafe extern "win64" fn streambuf_pbase(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() || (*this).ipfirst.is_null() { std::ptr::null_mut() } else { *(*this).ipfirst }
}
unsafe extern "win64" fn streambuf_pptr(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() || (*this).ipnext.is_null() { std::ptr::null_mut() } else { *(*this).ipnext }
}
unsafe extern "win64" fn streambuf_epptr(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() || (*this).ipnext.is_null() || (*(*this).ipnext).is_null() || (*this).ipcount.is_null() {
        std::ptr::null_mut()
    } else {
        (*(*this).ipnext).wrapping_offset((*(*this).ipcount) as isize)
    }
}
unsafe extern "win64" fn streambuf_getloc(_this: *const BasicStreambuf, out: *mut [usize; 2]) -> *mut [usize; 2] {
    if !out.is_null() {
        (*out)[0] = 0;
        (*out)[1] = locale_init(1) as usize;
    }
    out
}
unsafe extern "win64" fn ios_clear(this: *mut BasicIos, state: i32, _reraise: u8) {
    if !this.is_null() {
        (*this).mystate = state & 0x17;
    }
}
unsafe extern "win64" fn istream_read(this: *mut u8, dst: *mut u8, count: i64) -> *mut u8 {
    if this.is_null() || dst.is_null() || count <= 0 {
        return this;
    }
    let ios = stream_ios(this);
    if !ios.is_null() && !(*ios).mystrbuf.is_null() {
        let got = streambuf_xsgetn((*ios).mystrbuf, dst, count);
        std::ptr::write_unaligned(this.add(8) as *mut i64, got);
        if got != count {
            ios_setstate(ios, 3, 0);
        }
    }
    this
}
unsafe extern "win64" fn istream_seekg(this: *mut u8, _off: i64, _way: i32) -> *mut u8 {
    this
}
unsafe extern "win64" fn istream_tellg(_this: *mut u8, out: *mut [i64; 3]) -> *mut [i64; 3] {
    if !out.is_null() {
        std::ptr::write_unaligned(out, [0, 0, 0]);
    }
    out
}
unsafe extern "win64" fn smanip_noop(out: *mut [usize; 2], val: i64) -> *mut [usize; 2] {
    if !out.is_null() {
        (*out)[0] = noop_method as *const () as usize;
        (*out)[1] = val as usize;
    }
    out
}
unsafe extern "win64" fn time_put_put(
    _this: *const c_void,
    out_ret: *mut [usize; 2],
    out_it: *const [usize; 2],
) -> *mut [usize; 2] {
    if !out_ret.is_null() && !out_it.is_null() {
        *out_ret = *out_it;
    }
    out_ret
}
unsafe extern "win64" fn throw_future_error(_ec: *const c_void) {
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

// ---------------------------------------------------------------------------
// 8. Export Table (`EXTRA_EXPORTS`)
// ---------------------------------------------------------------------------

pub static EXTRA_EXPORTS: &[Export] = &[
    Export { name: "??0?$basic_ios@DU?$char_traits@D@std@@@std@@IEAA@XZ", ptr: Ptr(ios_ctor as *const ()) },
    Export { name: "??0?$basic_iostream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z", ptr: Ptr(iostream_ctor as *const ()) },
    Export { name: "??0?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@_N@Z", ptr: Ptr(istream_ctor as *const ()) },
    Export { name: "??0?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@_N@Z", ptr: Ptr(ostream_ctor as *const ()) },
    Export { name: "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@XZ", ptr: Ptr(streambuf_ctor as *const ()) },
    Export { name: "??0?$codecvt@_UDU_Mbstatet@@@std@@QEAA@_K@Z", ptr: Ptr(codecvt_u32_ctor as *const ()) },
    Export { name: "??0?$codecvt@_WDU_Mbstatet@@@std@@QEAA@_K@Z", ptr: Ptr(codecvt_w_ctor as *const ()) },
    Export { name: "??0_Locinfo@std@@QEAA@PEBD@Z", ptr: Ptr(locinfo_ctor as *const ()) },
    Export { name: "??0_Lockit@std@@QEAA@H@Z", ptr: Ptr(lockit_ctor as *const ()) },
    Export { name: "??0facet@locale@std@@IEAA@_K@Z", ptr: Ptr(facet_ctor as *const ()) },
    Export { name: "??0task_continuation_context@Concurrency@@AEAA@XZ", ptr: Ptr(task_continuation_context_ctor as *const ()) },
    Export { name: "??1?$basic_ios@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(ios_dtor as *const ()) },
    Export { name: "??1?$basic_iostream@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(iostream_dtor as *const ()) },
    Export { name: "??1?$basic_istream@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(istream_dtor as *const ()) },
    Export { name: "??1?$basic_ostream@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(ostream_dtor as *const ()) },
    Export { name: "??1?$basic_streambuf@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(streambuf_dtor as *const ()) },
    Export { name: "??1?$codecvt@_UDU_Mbstatet@@@std@@MEAA@XZ", ptr: Ptr(facet_dtor as *const ()) },
    Export { name: "??1?$codecvt@_WDU_Mbstatet@@@std@@MEAA@XZ", ptr: Ptr(facet_dtor as *const ()) },
    Export { name: "??1_Locinfo@std@@QEAA@XZ", ptr: Ptr(locinfo_dtor as *const ()) },
    Export { name: "??1_Lockit@std@@QEAA@XZ", ptr: Ptr(lockit_dtor as *const ()) },
    Export { name: "??1facet@locale@std@@MEAA@XZ", ptr: Ptr(facet_dtor as *const ()) },
    Export { name: "??4?$_Yarn@D@std@@QEAAAEAV01@PEBD@Z", ptr: Ptr(yarn_assign as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@G@Z", ptr: Ptr(ostream_shl_u16 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@H@Z", ptr: Ptr(ostream_shl_i32 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@I@Z", ptr: Ptr(ostream_shl_u32 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@K@Z", ptr: Ptr(ostream_shl_u32 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@M@Z", ptr: Ptr(crate::bridge_entry!(ostream_shl_f32)) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@N@Z", ptr: Ptr(crate::bridge_entry!(ostream_shl_f64)) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@P6AAEAV01@AEAV01@@Z@Z", ptr: Ptr(ostream_shl_manip as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@P6AAEAVios_base@1@AEAV21@@Z@Z", ptr: Ptr(ostream_shl_manip as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_J@Z", ptr: Ptr(ostream_shl_i64 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_K@Z", ptr: Ptr(ostream_shl_u64 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_N@Z", ptr: Ptr(ostream_shl_bool as *const ()) },
    Export { name: "??Bid@locale@std@@QEAA_KXZ", ptr: Ptr(locale_id_operator_usize as *const ()) },
    Export { name: "?GetCurrentThreadId@platform@details@Concurrency@@YAJXZ", ptr: Ptr(concurrency_get_current_thread_id as *const ()) },
    Export { name: "?ReportUnhandledError@_ExceptionHolder@details@Concurrency@@AEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_Addfac@_Locimp@locale@std@@AEAAXPEAVfacet@23@_K@Z", ptr: Ptr(locimp_addfac as *const ()) },
    Export { name: "?_CallInContext@_ContextCallback@details@Concurrency@@QEBAXV?$function@$$A6AXXZ@std@@_N@Z", ptr: Ptr(call_in_context as *const ()) },
    Export { name: "?_Capture@_ContextCallback@details@Concurrency@@AEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ", ptr: Ptr(facet_decref as *const ()) },
    Export { name: "?_Fiopen@std@@YAPEAU_iobuf@@PEBDHH@Z", ptr: Ptr(fiopen_a as *const ()) },
    Export { name: "?_Fiopen@std@@YAPEAU_iobuf@@PEB_WHH@Z", ptr: Ptr(fiopen_w as *const ()) },
    Export { name: "?_Getcat@?$codecvt@DDU_Mbstatet@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z", ptr: Ptr(codecvt_d_getcat as *const ()) },
    Export { name: "?_Getcat@?$ctype@D@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z", ptr: Ptr(ctype_getcat as *const ()) },
    Export { name: "?_Getcat@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z", ptr: Ptr(time_put_getcat as *const ()) },
    Export { name: "?_Getcoll@_Locinfo@std@@QEBA?AU_Collvec@@XZ", ptr: Ptr(locinfo_getcoll as *const ()) },
    Export { name: "?_Getcvt@_Locinfo@std@@QEBA?AU_Cvtvec@@XZ", ptr: Ptr(locinfo_getcvt as *const ()) },
    Export { name: "?_Getfalse@_Locinfo@std@@QEBAPEBDXZ", ptr: Ptr(locinfo_getfalse as *const ()) },
    Export { name: "?_Getgloballocale@locale@std@@CAPEAV_Locimp@12@XZ", ptr: Ptr(locale_getgloballocale as *const ()) },
    Export { name: "?_Getlconv@_Locinfo@std@@QEBAPEBUlconv@@XZ", ptr: Ptr(locinfo_getlconv as *const ()) },
    Export { name: "?_Gettrue@_Locinfo@std@@QEBAPEBDXZ", ptr: Ptr(locinfo_gettrue as *const ()) },
    Export { name: "?_Id_cnt@id@locale@std@@0HA", ptr: Ptr(&LOCALE_ID_CNT as *const AtomicU32 as *const ()) },
    Export { name: "?_Incref@facet@locale@std@@UEAAXXZ", ptr: Ptr(facet_incref as *const ()) },
    Export { name: "?_Init@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXXZ", ptr: Ptr(streambuf_init_empty as *const ()) },
    Export { name: "?_Init@locale@std@@CAPEAV_Locimp@12@_N@Z", ptr: Ptr(locale_init as *const ()) },
    Export { name: "?_Ipfx@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA_N_N@Z", ptr: Ptr(istream_ipfx as *const ()) },
    Export { name: "?_IsCurrentOriginSTA@_ContextCallback@details@Concurrency@@CA_NXZ", ptr: Ptr(is_current_origin_sta as *const ()) },
    Export { name: "?_Locimp_Addfac@_Locimp@locale@std@@CAXPEAV123@PEAVfacet@23@_K@Z", ptr: Ptr(locimp_addfac as *const ()) },
    Export { name: "?_Lock@?$basic_streambuf@DU?$char_traits@D@std@@@std@@UEAAXXZ", ptr: Ptr(streambuf_lock as *const ()) },
    Export { name: "?_LogCancelTask@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogScheduleTask@_TaskEventLogger@details@Concurrency@@QEAAX_N@Z", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogTaskCompleted@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogTaskExecutionCompleted@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogWorkItemCompleted@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogWorkItemStarted@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_New_Locimp@_Locimp@locale@std@@CAPEAV123@AEBV123@@Z", ptr: Ptr(locimp_new as *const ()) },
    Export { name: "?_Osfx@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAXXZ", ptr: Ptr(ostream_osfx as *const ()) },
    Export { name: "?_Pninc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ", ptr: Ptr(streambuf_pninc as *const ()) },
    Export { name: "?_Random_device@std@@YAIXZ", ptr: Ptr(std_random_device as *const ()) },
    Export { name: "?_Release_chore@details@Concurrency@@YAXPEAU_Threadpool_chore@12@@Z", ptr: Ptr(release_chore as *const ()) },
    Export { name: "?_ReportUnobservedException@details@Concurrency@@YAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_Reset@_ContextCallback@details@Concurrency@@AEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_Rethrow_future_exception@std@@YAXVexception_ptr@1@@Z", ptr: Ptr(exception_ptr_rethrow as *const ()) },
    Export { name: "?_Schedule_chore@details@Concurrency@@YAHPEAU_Threadpool_chore@12@@Z", ptr: Ptr(schedule_chore as *const ()) },
    Export { name: "?_Syserror_map@std@@YAPEBDH@Z", ptr: Ptr(syserror_map as *const ()) },
    Export { name: "?_Throw_C_error@std@@YAXH@Z", ptr: Ptr(throw_c_error as *const ()) },
    Export { name: "?_Throw_Cpp_error@std@@YAXH@Z", ptr: Ptr(throw_cpp_error as *const ()) },
    Export { name: "?_Throw_future_error@std@@YAXAEBVerror_code@1@@Z", ptr: Ptr(throw_future_error as *const ()) },
    Export { name: "?_Unlock@?$basic_streambuf@DU?$char_traits@D@std@@@std@@UEAAXXZ", ptr: Ptr(streambuf_unlock as *const ()) },
    Export { name: "?_W_Getdays@_Locinfo@std@@QEBAPEBGXZ", ptr: Ptr(locinfo_w_getdays as *const ()) },
    Export { name: "?_W_Getmonths@_Locinfo@std@@QEBAPEBGXZ", ptr: Ptr(locinfo_w_getmonths as *const ()) },
    Export { name: "?_Winerror_map@std@@YAHH@Z", ptr: Ptr(winerror_map as *const ()) },
    Export { name: "?_Xbad_alloc@std@@YAXXZ", ptr: Ptr(xbad_alloc as *const ()) },
    Export { name: "?_Xbad_function_call@std@@YAXXZ", ptr: Ptr(xbad_function_call as *const ()) },
    Export { name: "?_Xinvalid_argument@std@@YAXPEBD@Z", ptr: Ptr(xinvalid_argument as *const ()) },
    Export { name: "?_Xlength_error@std@@YAXPEBD@Z", ptr: Ptr(xlength_error as *const ()) },
    Export { name: "?_Xout_of_range@std@@YAXPEBD@Z", ptr: Ptr(xout_of_range as *const ()) },
    Export { name: "?_Xregex_error@std@@YAXW4error_type@regex_constants@1@@Z", ptr: Ptr(xregex_error as *const ()) },
    Export { name: "?__ExceptionPtrAssign@@YAXPEAXPEBX@Z", ptr: Ptr(exception_ptr_assign as *const ()) },
    Export { name: "?__ExceptionPtrCopy@@YAXPEAXPEBX@Z", ptr: Ptr(exception_ptr_copy as *const ()) },
    Export { name: "?__ExceptionPtrCopyException@@YAXPEAXPEBX1@Z", ptr: Ptr(exception_ptr_copy_exception as *const ()) },
    Export { name: "?__ExceptionPtrCreate@@YAXPEAX@Z", ptr: Ptr(exception_ptr_create as *const ()) },
    Export { name: "?__ExceptionPtrCurrentException@@YAXPEAX@Z", ptr: Ptr(exception_ptr_current_exception as *const ()) },
    Export { name: "?__ExceptionPtrDestroy@@YAXPEAX@Z", ptr: Ptr(exception_ptr_destroy as *const ()) },
    Export { name: "?__ExceptionPtrRethrow@@YAXPEBX@Z", ptr: Ptr(exception_ptr_rethrow as *const ()) },
    Export { name: "?__ExceptionPtrToBool@@YA_NPEBX@Z", ptr: Ptr(exception_ptr_to_bool as *const ()) },
    Export { name: "?always_noconv@codecvt_base@std@@QEBA_NXZ", ptr: Ptr(codecvt_always_noconv as *const ()) },
    Export { name: "?c_str@?$_Yarn@D@std@@QEBAPEBDXZ", ptr: Ptr(yarn_c_str as *const ()) },
    Export { name: "?cerr@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A", ptr: Ptr(&CERR as *const StaticOstream as *const ()) },
    Export { name: "?clear@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z", ptr: Ptr(ios_clear as *const ()) },
    Export { name: "?cout@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A", ptr: Ptr(&COUT as *const StaticOstream as *const ()) },
    Export { name: "?eback@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_eback as *const ()) },
    Export { name: "?egptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_egptr as *const ()) },
    Export { name: "?epptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_epptr as *const ()) },
    Export { name: "?flush@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@XZ", ptr: Ptr(ostream_flush as *const ()) },
    Export { name: "?getloc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEBA?AVlocale@2@XZ", ptr: Ptr(streambuf_getloc as *const ()) },
    Export { name: "?getloc@ios_base@std@@QEBA?AVlocale@2@XZ", ptr: Ptr(streambuf_getloc as *const ()) },
    Export { name: "?good@ios_base@std@@QEBA_NXZ", ptr: Ptr(ios_good as *const ()) },
    Export { name: "?gptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_gptr as *const ()) },
    Export { name: "?id@?$codecvt@DDU_Mbstatet@@@std@@2V0locale@2@A", ptr: Ptr(&ID_CODECVT_D as *const LocaleId as *const ()) },
    Export { name: "?id@?$codecvt@_WDU_Mbstatet@@@std@@2V0locale@2@A", ptr: Ptr(&ID_CODECVT_W as *const LocaleId as *const ()) },
    Export { name: "?id@?$collate@D@std@@2V0locale@2@A", ptr: Ptr(&ID_COLLATE_D as *const LocaleId as *const ()) },
    Export { name: "?id@?$ctype@D@std@@2V0locale@2@A", ptr: Ptr(&ID_CTYPE_D as *const LocaleId as *const ()) },
    Export { name: "?id@?$numpunct@D@std@@2V0locale@2@A", ptr: Ptr(&ID_NUMPUNCT_D as *const LocaleId as *const ()) },
    Export { name: "?id@?$numpunct@_W@std@@2V0locale@2@A", ptr: Ptr(&ID_NUMPUNCT_W as *const LocaleId as *const ()) },
    Export { name: "?id@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A", ptr: Ptr(&ID_TIME_PUT_D as *const LocaleId as *const ()) },
    Export { name: "?imbue@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAXAEBVlocale@2@@Z", ptr: Ptr(streambuf_imbue as *const ()) },
    Export { name: "?in@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEAD3AEAPEAD@Z", ptr: Ptr(codecvt_d_in_out as *const ()) },
    Export { name: "?in@?$codecvt@_UDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEA_U3AEAPEA_U@Z", ptr: Ptr(codecvt_u32_in as *const ()) },
    Export { name: "?in@?$codecvt@_WDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEA_W3AEAPEA_W@Z", ptr: Ptr(codecvt_w_in as *const ()) },
    Export { name: "?out@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEAD3AEAPEAD@Z", ptr: Ptr(codecvt_d_in_out as *const ()) },
    Export { name: "?out@?$codecvt@_UDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEB_U1AEAPEB_UPEAD3AEAPEAD@Z", ptr: Ptr(codecvt_u32_out as *const ()) },
    Export { name: "?out@?$codecvt@_WDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEB_W1AEAPEB_WPEAD3AEAPEAD@Z", ptr: Ptr(codecvt_w_out as *const ()) },
    Export { name: "?pbase@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_pbase as *const ()) },
    Export { name: "?pptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_pptr as *const ()) },
    Export { name: "?put@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@D@Z", ptr: Ptr(ostream_put as *const ()) },
    Export { name: "?put@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBUtm@@PEBD3@Z", ptr: Ptr(time_put_put as *const ()) },
    Export { name: "?read@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEAD_J@Z", ptr: Ptr(istream_read as *const ()) },
    Export { name: "?sbumpc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ", ptr: Ptr(streambuf_sbumpc as *const ()) },
    Export { name: "?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@_JH@Z", ptr: Ptr(istream_seekg as *const ()) },
    Export { name: "?setbuf@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAPEAV12@PEAD_J@Z", ptr: Ptr(streambuf_setbuf as *const ()) },
    Export { name: "?setprecision@std@@YA?AU?$_Smanip@_J@1@_J@Z", ptr: Ptr(smanip_noop as *const ()) },
    Export { name: "?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z", ptr: Ptr(ios_setstate as *const ()) },
    Export { name: "?setw@std@@YA?AU?$_Smanip@_J@1@_J@Z", ptr: Ptr(smanip_noop as *const ()) },
    Export { name: "?sgetc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ", ptr: Ptr(streambuf_sgetc as *const ()) },
    Export { name: "?showmanyc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA_JXZ", ptr: Ptr(streambuf_showmanyc as *const ()) },
    Export { name: "?snextc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ", ptr: Ptr(streambuf_snextc as *const ()) },
    Export { name: "?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z", ptr: Ptr(streambuf_sputc as *const ()) },
    Export { name: "?sputn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA_JPEBD_J@Z", ptr: Ptr(streambuf_sputn as *const ()) },
    Export { name: "?swap@?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAAXAEAV12@@Z", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?swap@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXAEAV12@@Z", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?sync@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAHXZ", ptr: Ptr(streambuf_sync as *const ()) },
    Export { name: "?tellg@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@XZ", ptr: Ptr(istream_tellg as *const ()) },
    Export { name: "?tolower@?$ctype@D@std@@QEBADD@Z", ptr: Ptr(ctype_tolower_char as *const ()) },
    Export { name: "?tolower@?$ctype@D@std@@QEBAPEBDPEADPEBD@Z", ptr: Ptr(ctype_tolower_range as *const ()) },
    Export { name: "?uflow@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAHXZ", ptr: Ptr(streambuf_uflow as *const ()) },
    Export { name: "?uncaught_exception@std@@YA_NXZ", ptr: Ptr(std_uncaught_exception as *const ()) },
    Export { name: "?uncaught_exceptions@std@@YAHXZ", ptr: Ptr(std_uncaught_exceptions as *const ()) },
    Export { name: "?unshift@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEAD1AEAPEAD@Z", ptr: Ptr(codecvt_unshift as *const ()) },
    Export { name: "?widen@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADD@Z", ptr: Ptr(ios_widen as *const ()) },
    Export { name: "?write@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEBD_J@Z", ptr: Ptr(ostream_write as *const ()) },
    Export { name: "?xsgetn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA_JPEAD_J@Z", ptr: Ptr(streambuf_xsgetn as *const ()) },
    Export { name: "?xsputn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA_JPEBD_J@Z", ptr: Ptr(streambuf_xsputn as *const ()) },
];
