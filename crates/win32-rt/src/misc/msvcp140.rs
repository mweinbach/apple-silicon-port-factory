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
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::Mutex;

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

#[inline(never)]
pub unsafe extern "win64" fn _Thrd_yield() {
    let _ = crate::kernel32::time::SwitchToThread();
}

pub unsafe extern "win64" fn _Thrd_hardware_concurrency() -> u32 {
    crate::process::num_cpus().max(1) as u32
}

static SHARED_PTR_SPIN: AtomicBool = AtomicBool::new(false);

pub unsafe extern "win64" fn _Lock_shared_ptr_spin_lock() {
    let mut spins = 0u32;
    while SHARED_PTR_SPIN
        .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        spins = spins.wrapping_add(1);
        if spins < 64 {
            std::hint::spin_loop();
        } else {
            std::thread::yield_now();
            spins = 0;
        }
    }
}

pub unsafe extern "win64" fn _Unlock_shared_ptr_spin_lock() {
    SHARED_PTR_SPIN.store(false, Ordering::Release);
}

/// MSVC Dinkumware `short _FExp(float *px, float y, short eoff)`:
/// computes `*px = exp(*px) * y * 2^eoff` and returns the MSVC floating-point
/// class code (`0` = zero, `-1` = finite, `1` = inf, `2` = NaN).
unsafe extern "win64" fn fexp_impl(px: *mut f32, y: f32, eoff: i16) -> i16 {
    if px.is_null() {
        return 0;
    }
    let x = std::ptr::read_unaligned(px);
    if x.is_nan() || y.is_nan() {
        std::ptr::write_unaligned(px, f32::NAN);
        return 2; // _NANCODE
    }
    if y == 0.0 {
        std::ptr::write_unaligned(px, 0.0f32.copysign(y));
        return 0; // FP_ZERO
    }
    let scaled_x = (x as f64) + (eoff as f64) * std::f64::consts::LN_2;
    let r = (scaled_x.exp() * (y as f64)) as f32;
    std::ptr::write_unaligned(px, r);
    if r.is_nan() {
        2
    } else if r.is_infinite() {
        1 // _INFCODE
    } else if r == 0.0 {
        0 // FP_ZERO
    } else {
        -1 // _FINITE
    }
}
crate::win64_bridge!(fn fexp_impl(*mut f32, f32, i16) -> i16);

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

/// MSVC x64 `std::exception` / `std::runtime_error` (`0x18` bytes).
#[repr(C)]
struct StdException {
    vtable: *const Ptr,
    data: super::vcruntime140::StdExceptionData,
}
const _: () = assert!(std::mem::size_of::<StdException>() == 0x18);

/// MSVC x64 `std::error_category` (`0x10` bytes).
#[repr(C)]
struct StdErrorCategory {
    vtable: *const Ptr,
    addr: usize,
}
const _: () = assert!(std::mem::size_of::<StdErrorCategory>() == 0x10);
unsafe impl Sync for StdErrorCategory {}

/// MSVC x64 `std::error_code` / `std::error_condition` (`0x10` bytes).
#[repr(C)]
#[derive(Clone, Copy)]
struct StdErrorCode {
    val: i32,
    _pad: u32,
    cat: *const StdErrorCategory,
}
const _: () = assert!(std::mem::size_of::<StdErrorCode>() == 0x10);

/// MSVC x64 `std::system_error` (`0x28` bytes, inherits `std::runtime_error`).
#[repr(C)]
struct StdSystemError {
    base: StdException,
    code: StdErrorCode,
}
const _: () = assert!(std::mem::size_of::<StdSystemError>() == 0x28);

unsafe fn write_msvc_string(out: *mut [u8; 32], text: &str) {
    if out.is_null() {
        return;
    }
    let bytes = text.as_bytes();
    let mut raw = [0u8; 32];
    if bytes.len() < 16 {
        raw[..bytes.len()].copy_from_slice(bytes);
        raw[bytes.len()] = 0;
        raw[16..24].copy_from_slice(&(bytes.len() as u64).to_le_bytes());
        raw[24..32].copy_from_slice(&15u64.to_le_bytes());
    } else {
        let cap = bytes.len();
        let buf = crate::ucrt::heap::malloc(cap + 1) as *mut u8;
        if !buf.is_null() {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
            *buf.add(bytes.len()) = 0;
            raw[0..8].copy_from_slice(&(buf as usize as u64).to_le_bytes());
            raw[16..24].copy_from_slice(&(bytes.len() as u64).to_le_bytes());
            raw[24..32].copy_from_slice(&(cap as u64).to_le_bytes());
        } else {
            raw[24..32].copy_from_slice(&15u64.to_le_bytes());
        }
    }
    std::ptr::write_unaligned(out, raw);
}

fn system_error_message(win_err: i32) -> String {
    let posix = unsafe { winerror_map(win_err) };
    if posix != 0 {
        let msg = unsafe { CStr::from_ptr(syserror_map(posix)) }.to_string_lossy();
        if msg != "unknown error" {
            return msg.into_owned();
        }
    }
    format!("system error {win_err}")
}

unsafe extern "win64" fn error_category_deleting_dtor(
    this: *mut StdErrorCategory,
    _flags: u32,
) -> *mut StdErrorCategory {
    this
}

unsafe extern "win64" fn system_category_name(_this: *const StdErrorCategory) -> *const c_char {
    c"system".as_ptr()
}

unsafe extern "win64" fn generic_category_name(_this: *const StdErrorCategory) -> *const c_char {
    c"generic".as_ptr()
}

unsafe extern "win64" fn system_category_message(
    _this: *const StdErrorCategory,
    out: *mut [u8; 32],
    errval: i32,
) -> *mut [u8; 32] {
    let msg = system_error_message(errval);
    write_msvc_string(out, &msg);
    out
}

unsafe extern "win64" fn generic_category_message(
    _this: *const StdErrorCategory,
    out: *mut [u8; 32],
    errval: i32,
) -> *mut [u8; 32] {
    let msg = CStr::from_ptr(syserror_map(errval)).to_string_lossy();
    write_msvc_string(out, &msg);
    out
}

unsafe extern "win64" fn system_category_default_error_condition(
    this: *const StdErrorCategory,
    out: *mut StdErrorCode,
    errval: i32,
) -> *mut StdErrorCode {
    if !out.is_null() {
        let posix = winerror_map(errval);
        let cond = if posix != 0 {
            StdErrorCode { val: posix, _pad: 0, cat: &GENERIC_CATEGORY }
        } else {
            StdErrorCode { val: errval, _pad: 0, cat: this }
        };
        std::ptr::write_unaligned(out, cond);
    }
    out
}

unsafe extern "win64" fn generic_category_default_error_condition(
    this: *const StdErrorCategory,
    out: *mut StdErrorCode,
    errval: i32,
) -> *mut StdErrorCode {
    if !out.is_null() {
        std::ptr::write_unaligned(out, StdErrorCode { val: errval, _pad: 0, cat: this });
    }
    out
}

unsafe extern "win64" fn error_category_equiv_code_int(
    this: *const StdErrorCategory,
    code: *const StdErrorCode,
    errval: i32,
) -> u8 {
    if this.is_null() || code.is_null() {
        return 0;
    }
    let c = std::ptr::read_unaligned(code);
    if c.val != errval || c.cat.is_null() {
        return 0;
    }
    let this_addr: usize = read_field(this, std::mem::offset_of!(StdErrorCategory, addr));
    let other_addr: usize = read_field(c.cat, std::mem::offset_of!(StdErrorCategory, addr));
    (this_addr == other_addr) as u8
}

unsafe extern "win64" fn error_category_equiv_int_cond(
    this: *const StdErrorCategory,
    errval: i32,
    cond: *const StdErrorCode,
) -> u8 {
    if this.is_null() || cond.is_null() {
        return 0;
    }
    let mut def = StdErrorCode { val: 0, _pad: 0, cat: std::ptr::null() };
    let vtable: *const Ptr = read_field(this, std::mem::offset_of!(StdErrorCategory, vtable));
    if !vtable.is_null() {
        let f = std::ptr::read_unaligned(vtable.add(3)).0 as usize;
        if f != 0 {
            let _ = crate::call_guest8(
                f,
                this as u64,
                &mut def as *mut StdErrorCode as u64,
                errval as u32 as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }
    let rhs = std::ptr::read_unaligned(cond);
    if def.val != rhs.val || def.cat.is_null() || rhs.cat.is_null() {
        return 0;
    }
    let a: usize = read_field(def.cat, std::mem::offset_of!(StdErrorCategory, addr));
    let b: usize = read_field(rhs.cat, std::mem::offset_of!(StdErrorCategory, addr));
    (a == b) as u8
}

// MSVC reverses overloaded virtuals of the same name: slot 4 = equivalent(error_code, int),
// slot 5 = equivalent(int, error_condition).
static SYSTEM_CATEGORY_VTABLE: [Ptr; 6] = [
    Ptr(error_category_deleting_dtor as *const ()),
    Ptr(system_category_name as *const ()),
    Ptr(system_category_message as *const ()),
    Ptr(system_category_default_error_condition as *const ()),
    Ptr(error_category_equiv_code_int as *const ()),
    Ptr(error_category_equiv_int_cond as *const ()),
];

static GENERIC_CATEGORY_VTABLE: [Ptr; 6] = [
    Ptr(error_category_deleting_dtor as *const ()),
    Ptr(generic_category_name as *const ()),
    Ptr(generic_category_message as *const ()),
    Ptr(generic_category_default_error_condition as *const ()),
    Ptr(error_category_equiv_code_int as *const ()),
    Ptr(error_category_equiv_int_cond as *const ()),
];

static SYSTEM_CATEGORY: StdErrorCategory = StdErrorCategory {
    vtable: SYSTEM_CATEGORY_VTABLE.as_ptr(),
    addr: 0x1fc7_0001,
};

static GENERIC_CATEGORY: StdErrorCategory = StdErrorCategory {
    vtable: GENERIC_CATEGORY_VTABLE.as_ptr(),
    addr: 0x1fc7_0002,
};

unsafe extern "win64" fn exception_deleting_dtor(
    this: *mut StdException,
    flags: u32,
) -> *mut StdException {
    if !this.is_null() {
        let mut data: super::vcruntime140::StdExceptionData =
            read_field(this, std::mem::offset_of!(StdException, data));
        super::vcruntime140::__std_exception_destroy(&mut data);
        write_field(this, std::mem::offset_of!(StdException, data), data);
        if (flags & 1) != 0 {
            crate::ucrt::heap::free(this.cast());
        }
    }
    this
}

unsafe extern "win64" fn exception_what(this: *const StdException) -> *const c_char {
    if !this.is_null() {
        let data: super::vcruntime140::StdExceptionData =
            read_field(this, std::mem::offset_of!(StdException, data));
        if !data.what.is_null() {
            return data.what;
        }
    }
    c"Unknown exception".as_ptr()
}

static EXCEPTION_VTABLE: [Ptr; 2] = [
    Ptr(exception_deleting_dtor as *const ()),
    Ptr(exception_what as *const ()),
];

static RUNTIME_ERROR_VTABLE: [Ptr; 2] = [
    Ptr(exception_deleting_dtor as *const ()),
    Ptr(exception_what as *const ()),
];

static SYSTEM_ERROR_VTABLE: [Ptr; 2] = [
    Ptr(exception_deleting_dtor as *const ()),
    Ptr(exception_what as *const ()),
];

unsafe extern "win64" fn exception_copy_ctor(
    dst: *mut StdException,
    src: *const StdException,
) -> *mut StdException {
    if !dst.is_null() && !src.is_null() {
        write_field(dst, std::mem::offset_of!(StdException, vtable), EXCEPTION_VTABLE.as_ptr());
        let src_data: super::vcruntime140::StdExceptionData =
            read_field(src, std::mem::offset_of!(StdException, data));
        let mut dst_data = super::vcruntime140::StdExceptionData {
            what: std::ptr::null(),
            do_free: 0,
        };
        super::vcruntime140::__std_exception_copy(&src_data, &mut dst_data);
        write_field(dst, std::mem::offset_of!(StdException, data), dst_data);
    }
    dst
}

unsafe extern "win64" fn runtime_error_copy_ctor(
    dst: *mut StdException,
    src: *const StdException,
) -> *mut StdException {
    if !dst.is_null() && !src.is_null() {
        write_field(dst, std::mem::offset_of!(StdException, vtable), RUNTIME_ERROR_VTABLE.as_ptr());
        let src_data: super::vcruntime140::StdExceptionData =
            read_field(src, std::mem::offset_of!(StdException, data));
        let mut dst_data = super::vcruntime140::StdExceptionData {
            what: std::ptr::null(),
            do_free: 0,
        };
        super::vcruntime140::__std_exception_copy(&src_data, &mut dst_data);
        write_field(dst, std::mem::offset_of!(StdException, data), dst_data);
    }
    dst
}

unsafe extern "win64" fn system_error_copy_ctor(
    dst: *mut StdSystemError,
    src: *const StdSystemError,
) -> *mut StdSystemError {
    if !dst.is_null() && !src.is_null() {
        write_field(
            dst,
            std::mem::offset_of!(StdSystemError, base) + std::mem::offset_of!(StdException, vtable),
            SYSTEM_ERROR_VTABLE.as_ptr(),
        );
        let src_data: super::vcruntime140::StdExceptionData = read_field(
            src,
            std::mem::offset_of!(StdSystemError, base) + std::mem::offset_of!(StdException, data),
        );
        let mut dst_data = super::vcruntime140::StdExceptionData {
            what: std::ptr::null(),
            do_free: 0,
        };
        super::vcruntime140::__std_exception_copy(&src_data, &mut dst_data);
        write_field(
            dst,
            std::mem::offset_of!(StdSystemError, base) + std::mem::offset_of!(StdException, data),
            dst_data,
        );
        let code: StdErrorCode = read_field(src, std::mem::offset_of!(StdSystemError, code));
        write_field(dst, std::mem::offset_of!(StdSystemError, code), code);
    }
    dst
}

/// Invoked solely via `ThrowInfo::pmfn_unwind` when `destruct_exception_object` tears down
/// the heap-allocated thrown `StdSystemError` after its catch block completes.
unsafe extern "win64" fn system_error_unwind_dtor(this: *mut StdSystemError) {
    if !this.is_null() {
        let mut data: super::vcruntime140::StdExceptionData = read_field(
            this,
            std::mem::offset_of!(StdSystemError, base) + std::mem::offset_of!(StdException, data),
        );
        super::vcruntime140::__std_exception_destroy(&mut data);
        write_field(
            this,
            std::mem::offset_of!(StdSystemError, base) + std::mem::offset_of!(StdException, data),
            data,
        );
        crate::ucrt::heap::free(this.cast());
    }
}

#[repr(C)]
struct MsvcTypeDescriptor<const N: usize> {
    vftable: usize,
    spare: usize,
    name: [u8; N],
}

#[repr(C)]
struct MsvcCatchableType {
    properties: u32,
    p_type: i32,
    mdisp: i32,
    pdisp: i32,
    vdisp: i32,
    size_or_offset: i32,
    copy_function: i32,
}
const _: () = assert!(std::mem::size_of::<MsvcCatchableType>() == 28);

#[repr(C)]
struct MsvcCatchableTypeArray3 {
    n_catchable_types: i32,
    types: [i32; 3],
}

#[repr(C)]
struct MsvcThrowInfo {
    attributes: u32,
    pmfn_unwind: i32,
    p_forward_compat: i32,
    p_catchable_type_array: i32,
}
const _: () = assert!(std::mem::size_of::<MsvcThrowInfo>() == 16);

#[repr(C)]
struct SystemErrorThrowSlab {
    td_system_error: MsvcTypeDescriptor<23>,
    td_runtime_error: MsvcTypeDescriptor<24>,
    td_exception: MsvcTypeDescriptor<20>,
    ct_system_error: MsvcCatchableType,
    ct_runtime_error: MsvcCatchableType,
    ct_exception: MsvcCatchableType,
    cta: MsvcCatchableTypeArray3,
    ti_heap: MsvcThrowInfo,
    ti_nofree: MsvcThrowInfo,
}

struct SyncSystemErrorSlab(UnsafeCell<SystemErrorThrowSlab>);
unsafe impl Sync for SyncSystemErrorSlab {}

const fn mk_td<const N: usize>(bytes: &[u8; N]) -> MsvcTypeDescriptor<N> {
    MsvcTypeDescriptor { vftable: 0, spare: 0, name: *bytes }
}

const fn mk_ct(size: i32) -> MsvcCatchableType {
    MsvcCatchableType {
        properties: 0,
        p_type: 0,
        mdisp: 0,
        pdisp: -1,
        vdisp: 0,
        size_or_offset: size,
        copy_function: 0,
    }
}

/// Static `__DATA`-resident `ThrowInfo` slab for `std::system_error` so all RVAs — both
/// metadata offsets and `__TEXT` copy-ctor/dtor function offsets — fit within `i32`.
static SYSTEM_ERROR_THROW_SLAB: SyncSystemErrorSlab = SyncSystemErrorSlab(UnsafeCell::new(
    SystemErrorThrowSlab {
        td_system_error: mk_td(b".?AVsystem_error@std@@\0"),
        td_runtime_error: mk_td(b".?AVruntime_error@std@@\0"),
        td_exception: mk_td(b".?AVexception@std@@\0"),
        ct_system_error: mk_ct(std::mem::size_of::<StdSystemError>() as i32),
        ct_runtime_error: mk_ct(std::mem::size_of::<StdException>() as i32),
        ct_exception: mk_ct(std::mem::size_of::<StdException>() as i32),
        cta: MsvcCatchableTypeArray3 { n_catchable_types: 3, types: [0; 3] },
        ti_heap: MsvcThrowInfo {
            attributes: 0,
            pmfn_unwind: 0,
            p_forward_compat: 0,
            p_catchable_type_array: 0,
        },
        ti_nofree: MsvcThrowInfo {
            attributes: 0,
            pmfn_unwind: 0,
            p_forward_compat: 0,
            p_catchable_type_array: 0,
        },
    },
));

static SYSTEM_ERROR_SLAB_INIT: std::sync::Once = std::sync::Once::new();

fn system_error_throw_slab() -> &'static SystemErrorThrowSlab {
    SYSTEM_ERROR_SLAB_INIT.call_once(|| unsafe {
        let slab = &mut *SYSTEM_ERROR_THROW_SLAB.0.get();
        let base = slab as *const SystemErrorThrowSlab as usize;
        let data_rva = |p: *const u8| i32::try_from((p as isize) - (base as isize)).unwrap_or(0);
        let fn_rva = |f: *const ()| i32::try_from((f as usize as isize) - (base as isize)).unwrap_or(0);

        slab.ct_system_error.p_type = data_rva(std::ptr::addr_of!(slab.td_system_error).cast());
        slab.ct_runtime_error.p_type = data_rva(std::ptr::addr_of!(slab.td_runtime_error).cast());
        slab.ct_exception.p_type = data_rva(std::ptr::addr_of!(slab.td_exception).cast());

        slab.ct_system_error.copy_function = fn_rva(system_error_copy_ctor as *const ());
        slab.ct_runtime_error.copy_function = fn_rva(runtime_error_copy_ctor as *const ());
        slab.ct_exception.copy_function = fn_rva(exception_copy_ctor as *const ());

        slab.cta.types = [
            data_rva(std::ptr::addr_of!(slab.ct_system_error).cast()),
            data_rva(std::ptr::addr_of!(slab.ct_runtime_error).cast()),
            data_rva(std::ptr::addr_of!(slab.ct_exception).cast()),
        ];

        let cta_rva = data_rva(std::ptr::addr_of!(slab.cta).cast());
        slab.ti_heap.pmfn_unwind = fn_rva(system_error_unwind_dtor as *const ());
        slab.ti_heap.p_catchable_type_array = cta_rva;
        slab.ti_nofree.pmfn_unwind = 0;
        slab.ti_nofree.p_catchable_type_array = cta_rva;
    });
    unsafe { &*SYSTEM_ERROR_THROW_SLAB.0.get() }
}

unsafe fn init_system_error(dst: *mut StdSystemError, win_err: i32) {
    let msg = system_error_message(win_err);
    let len = msg.len() + 1;
    let buf = crate::ucrt::heap::malloc(len) as *mut c_char;
    let data = if !buf.is_null() {
        std::ptr::copy_nonoverlapping(msg.as_ptr(), buf as *mut u8, msg.len());
        *(buf as *mut u8).add(msg.len()) = 0;
        super::vcruntime140::StdExceptionData { what: buf, do_free: 1 }
    } else {
        super::vcruntime140::StdExceptionData {
            what: c"system error".as_ptr(),
            do_free: 0,
        }
    };
    write_field(
        dst,
        std::mem::offset_of!(StdSystemError, base) + std::mem::offset_of!(StdException, vtable),
        SYSTEM_ERROR_VTABLE.as_ptr(),
    );
    write_field(
        dst,
        std::mem::offset_of!(StdSystemError, base) + std::mem::offset_of!(StdException, data),
        data,
    );
    write_field(
        dst,
        std::mem::offset_of!(StdSystemError, code),
        StdErrorCode { val: win_err, _pad: 0, cat: &SYSTEM_CATEGORY },
    );
}

unsafe extern "win64" fn xbad_alloc() {
    log::error!("std::_Xbad_alloc()");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

unsafe extern "win64" fn xget_last_error() {
    ensure_vtables_registered();
    let err = crate::teb::last_error() as i32;
    let slab = system_error_throw_slab();
    let base = slab as *const SystemErrorThrowSlab as usize;
    let obj = crate::ucrt::heap::calloc(1, std::mem::size_of::<StdSystemError>())
        as *mut StdSystemError;
    if !obj.is_null() {
        init_system_error(obj, err);
        super::vcruntime140::throw_cxx_exception_with_base(
            obj.cast(),
            std::ptr::addr_of!(slab.ti_heap).cast(),
            base,
        );
    } else {
        let mut fallback = StdSystemError {
            base: StdException {
                vtable: SYSTEM_ERROR_VTABLE.as_ptr(),
                data: super::vcruntime140::StdExceptionData {
                    what: c"system error".as_ptr(),
                    do_free: 0,
                },
            },
            code: StdErrorCode { val: err, _pad: 0, cat: &SYSTEM_CATEGORY },
        };
        super::vcruntime140::throw_cxx_exception_with_base(
            std::ptr::addr_of_mut!(fallback).cast(),
            std::ptr::addr_of!(slab.ti_nofree).cast(),
            base,
        );
    }
}

unsafe extern "win64" fn execute_once(
    flag: *mut c_void,
    callback: Option<crate::sync::InitOnceFn>,
    pv: *mut c_void,
) -> i32 {
    let Some(cb) = callback else {
        crate::teb::set_last_error(crate::types::err::ERROR_INVALID_PARAMETER);
        return 0;
    };
    if flag.is_null() {
        crate::teb::set_last_error(crate::types::err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    crate::sync::InitOnceExecuteOnce(flag, cb, pv, std::ptr::null_mut())
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

unsafe extern "win64" fn xoverflow_error(msg: *const c_char) {
    let s = if msg.is_null() { "" } else { CStr::from_ptr(msg).to_str().unwrap_or("") };
    log::error!("std::_Xoverflow_error({s:?})");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

unsafe extern "win64" fn xruntime_error(msg: *const c_char) {
    let s = if msg.is_null() { "" } else { CStr::from_ptr(msg).to_str().unwrap_or("") };
    log::error!("std::_Xruntime_error({s:?})");
    super::vcruntime140::_CxxThrowException(std::ptr::null_mut(), std::ptr::null());
}

pub static RAISE_HANDLER: AtomicUsize = AtomicUsize::new(0);

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
pub static ID_CTYPE_W: LocaleId = LocaleId(UnsafeCell::new([8, 0]));

static NEXT_LOCALE_ID: AtomicUsize = AtomicUsize::new(9);

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
        let eff = if name.is_null() || *name == 0 { c"C".as_ptr() } else { name };
        yarn_assign(this.add(0x58) as *mut *mut u8, eff);
    }
    this
}

unsafe extern "win64" fn locinfo_ctor_cat(this: *mut u8, _cat: i32, name: *const c_char) -> *mut u8 {
    locinfo_ctor(this, name)
}

unsafe extern "win64" fn locinfo_getname(this: *const u8) -> *const c_char {
    if this.is_null() {
        return c"C".as_ptr();
    }
    let p = std::ptr::read_unaligned(this.add(0x58) as *const *const c_char);
    if p.is_null() || *p == 0 { c"C".as_ptr() } else { p }
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

unsafe fn unaligned_atomic_u32_fetch_add(ptr: *mut u8, delta: u32) -> u32 {
    if (ptr as usize).is_multiple_of(std::mem::align_of::<AtomicU32>()) {
        (*(ptr as *const AtomicU32)).fetch_add(delta, Ordering::AcqRel)
    } else {
        static UNALIGNED_REF_LOCK: Mutex<()> = Mutex::new(());
        let _g = UNALIGNED_REF_LOCK.lock().unwrap();
        let prev = std::ptr::read_unaligned(ptr as *const u32);
        std::ptr::write_unaligned(ptr as *mut u32, prev.wrapping_add(delta));
        prev
    }
}

unsafe extern "win64" fn facet_deleting_dtor(this: *mut Facet, flags: u32) -> *mut Facet {
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(Facet, vtable), FACET_VTABLE.as_ptr());
        if (flags & 1) != 0 {
            crate::ucrt::heap::free(this.cast());
        }
    }
    this
}

unsafe extern "win64" fn facet_ctor(this: *mut Facet, refs: usize) -> *mut Facet {
    ensure_vtables_registered();
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(Facet, vtable), FACET_VTABLE.as_ptr());
        write_field(this, std::mem::offset_of!(Facet, refs), refs as u32);
        write_field(this, std::mem::offset_of!(Facet, _pad), 0u32);
    }
    this
}

unsafe extern "win64" fn facet_dtor(this: *mut Facet) {
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(Facet, vtable), FACET_VTABLE.as_ptr());
    }
}

unsafe extern "win64" fn facet_incref(this: *mut Facet) {
    if !this.is_null() {
        let refs_ptr = field_ptr::<_, u8>(this, std::mem::offset_of!(Facet, refs));
        unaligned_atomic_u32_fetch_add(refs_ptr, 1);
    }
}

unsafe extern "win64" fn facet_decref(this: *mut Facet) -> *mut Facet {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let refs_ptr = field_ptr::<_, u8>(this, std::mem::offset_of!(Facet, refs));
    if unaligned_atomic_u32_fetch_add(refs_ptr, u32::MAX) == 1 {
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

unsafe extern "win64" fn locimp_new_bool(transparent: u8) -> *mut Locimp {
    ensure_vtables_registered();
    let dst = crate::ucrt::heap::calloc(1, std::mem::size_of::<Locimp>()) as *mut Locimp;
    if dst.is_null() {
        return dst;
    }
    write_field(dst, std::mem::offset_of!(Locimp, vtable), FACET_VTABLE.as_ptr());
    write_field(dst, std::mem::offset_of!(Locimp, refs), 1u32);
    write_field(dst, std::mem::offset_of!(Locimp, catmask), 0x3fi32);
    write_field(dst, std::mem::offset_of!(Locimp, transparent), transparent);
    write_field(dst, std::mem::offset_of!(Locimp, cloc_inline_name), b'C');
    dst
}

unsafe extern "win64" fn locimp_makeloc(
    lobj: *const u8,
    cat: i32,
    ptrimp: *mut Locimp,
    _ptrloc: *const c_void,
) -> *mut Locimp {
    let imp = if ptrimp.is_null() { locimp_new_bool(0) } else { ptrimp };
    if imp.is_null() {
        return imp;
    }
    if cat != 0 {
        let cur_cat: i32 = read_field(imp, std::mem::offset_of!(Locimp, catmask));
        write_field(imp, std::mem::offset_of!(Locimp, catmask), cur_cat | cat);
    }
    if !lobj.is_null() {
        let name = locinfo_getname(lobj);
        if !name.is_null() {
            yarn_assign(field_ptr::<_, *mut u8>(imp, std::mem::offset_of!(Locimp, name)), name);
        }
    }
    // Populate ctype<char> and ctype<wchar_t> facets when _M_CTYPE (2) is requested.
    if (cat & 2) != 0 {
        let mut f_char: *const Facet = std::ptr::null();
        let _ = ctype_getcat(&mut f_char, std::ptr::null());
        if !f_char.is_null() {
            let id = locale_id_operator_usize(ID_CTYPE_D.0.get() as *mut usize);
            locimp_addfac(imp, f_char as *mut Facet, id);
        }
        let mut f_wide: *const Facet = std::ptr::null();
        let _ = ctype_w_getcat(&mut f_wide, std::ptr::null());
        if !f_wide.is_null() {
            let id = locale_id_operator_usize(ID_CTYPE_W.0.get() as *mut usize);
            locimp_addfac(imp, f_wide as *mut Facet, id);
        }
    }
    imp
}

unsafe extern "win64" fn locimp_new(src: *const Locimp) -> *mut Locimp {
    ensure_vtables_registered();
    let dst = crate::ucrt::heap::calloc(1, std::mem::size_of::<Locimp>()) as *mut Locimp;
    if dst.is_null() {
        return dst;
    }
    write_field(dst, std::mem::offset_of!(Locimp, vtable), FACET_VTABLE.as_ptr());
    write_field(dst, std::mem::offset_of!(Locimp, refs), 1u32);
    write_field(dst, std::mem::offset_of!(Locimp, catmask), 0x3fi32);
    write_field(dst, std::mem::offset_of!(Locimp, cloc_inline_name), b'C');
    if !src.is_null() {
        let count: usize = read_field(src, std::mem::offset_of!(Locimp, facetcount));
        let src_vec: *mut *mut Facet = read_field(src, std::mem::offset_of!(Locimp, facetvec));
        if count > 0 && !src_vec.is_null() {
            let vec = crate::ucrt::heap::calloc(count, std::mem::size_of::<*mut Facet>()) as *mut *mut Facet;
            if !vec.is_null() {
                for i in 0..count {
                    let f = std::ptr::read_unaligned(src_vec.add(i));
                    if !f.is_null() {
                        facet_incref(f);
                    }
                    std::ptr::write_unaligned(vec.add(i), f);
                }
                write_field(dst, std::mem::offset_of!(Locimp, facetvec), vec);
                write_field(dst, std::mem::offset_of!(Locimp, facetcount), count);
            }
        }
    }
    dst
}

unsafe extern "win64" fn locimp_addfac(this: *mut Locimp, fac: *mut Facet, id: usize) {
    if this.is_null() || fac.is_null() {
        return;
    }
    let mut cur_count: usize = read_field(this, std::mem::offset_of!(Locimp, facetcount));
    let mut cur_vec: *mut *mut Facet = read_field(this, std::mem::offset_of!(Locimp, facetvec));
    if cur_count <= id {
        let new_count = (id + 1).max(40);
        let new_vec = crate::ucrt::heap::realloc(
            cur_vec.cast(),
            new_count * std::mem::size_of::<*mut Facet>(),
        ) as *mut *mut Facet;
        if new_vec.is_null() {
            return;
        }
        for i in cur_count..new_count {
            std::ptr::write_unaligned(new_vec.add(i), std::ptr::null_mut());
        }
        write_field(this, std::mem::offset_of!(Locimp, facetvec), new_vec);
        write_field(this, std::mem::offset_of!(Locimp, facetcount), new_count);
        cur_vec = new_vec;
        cur_count = new_count;
    }
    let _ = cur_count;
    facet_incref(fac);
    let old = std::ptr::read_unaligned(cur_vec.add(id));
    if !old.is_null() {
        let del = facet_decref(old);
        if !del.is_null() {
            let vtbl: *const Ptr = read_field(del, std::mem::offset_of!(Facet, vtable));
            if !vtbl.is_null() {
                let dtor = std::ptr::read_unaligned(vtbl).0 as usize;
                let _ = crate::call_guest8(dtor, del as u64, 1, 0, 0, 0, 0, 0, 0);
            }
        }
    }
    std::ptr::write_unaligned(cur_vec.add(id), fac);
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
        write_field(this, std::mem::offset_of!(CtypeChar, vtable), FACET_VTABLE.as_ptr());
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

/// `std::ctype<wchar_t>` (`0x60` bytes on MSVC x64: `facet` + `_Ctypevec` + `_Cvtvec`).
#[repr(C)]
pub struct CtypeW {
    pub vtable: *const Ptr,
    pub refs: AtomicU32,
    pub _pad0: u32,
    pub page: u32,
    pub _pad1: u32,
    pub table: *const u16,
    pub delfl: i32,
    pub _pad2: u32,
    pub locale_name: *mut u16,
    pub cvt: Cvtvec,
}
const _: () = assert!(std::mem::size_of::<CtypeW>() == 0x60);

static CTYPE_W_VTABLE: [Ptr; 15] = [
    Ptr(ctype_w_deleting_dtor as *const ()),
    Ptr(facet_incref as *const ()),
    Ptr(facet_decref as *const ()),
    Ptr(ctype_w_do_is_range as *const ()),
    Ptr(ctype_w_do_is_char as *const ()),
    Ptr(ctype_w_do_scan_is as *const ()),
    Ptr(ctype_w_do_scan_not as *const ()),
    Ptr(ctype_w_do_tolower_range as *const ()),
    Ptr(ctype_w_do_tolower_char as *const ()),
    Ptr(ctype_w_do_toupper_range as *const ()),
    Ptr(ctype_w_do_toupper_char as *const ()),
    Ptr(ctype_w_do_widen_range as *const ()),
    Ptr(ctype_w_do_widen_char as *const ()),
    Ptr(ctype_w_do_narrow_range as *const ()),
    Ptr(ctype_w_do_narrow_char as *const ()),
];

unsafe extern "win64" fn ctype_w_deleting_dtor(this: *mut CtypeW, flags: u32) -> *mut CtypeW {
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(CtypeW, vtable), FACET_VTABLE.as_ptr());
        if (flags & 1) != 0 {
            crate::ucrt::heap::free(this.cast());
        }
    }
    this
}

#[inline]
fn wchar_ctype_mask(ch: u16) -> u16 {
    let mut m = 0u16;
    if ch < 256 {
        let p = unsafe { crate::ucrt::string::__pctype_func() };
        if !p.is_null() {
            return unsafe { *p.add(ch as usize) };
        }
    }
    if let Some(c) = char::from_u32(ch as u32) {
        if c.is_uppercase() {
            m |= 0x0101; // _ALPHA | _UPPER
        }
        if c.is_lowercase() {
            m |= 0x0102; // _ALPHA | _LOWER
        }
        if c.is_alphabetic() {
            m |= 0x0100; // _ALPHA
        }
        if c.is_ascii_digit() {
            m |= 0x0004; // _DIGIT
        }
        if c.is_whitespace() {
            m |= 0x0008; // _SPACE
        }
        if c.is_control() {
            m |= 0x0020; // _CONTROL
        }
    }
    m
}

#[inline]
fn wchar_to_lower(ch: u16) -> u16 {
    if (b'A' as u16..=b'Z' as u16).contains(&ch) {
        ch + 32
    } else if let Some(c) = char::from_u32(ch as u32) {
        let mut it = c.to_lowercase();
        if let (Some(first), None) = (it.next(), it.next()) {
            if (first as u32) <= 0xFFFF {
                return first as u16;
            }
        }
        ch
    } else {
        ch
    }
}

#[inline]
fn wchar_to_upper(ch: u16) -> u16 {
    if (b'a' as u16..=b'z' as u16).contains(&ch) {
        ch - 32
    } else if let Some(c) = char::from_u32(ch as u32) {
        let mut it = c.to_uppercase();
        if let (Some(first), None) = (it.next(), it.next()) {
            if (first as u32) <= 0xFFFF {
                return first as u16;
            }
        }
        ch
    } else {
        ch
    }
}

unsafe extern "win64" fn ctype_w_do_is_char(_this: *const CtypeW, mask: u16, ch: u16) -> u8 {
    ((wchar_ctype_mask(ch) & mask) != 0) as u8
}

unsafe extern "win64" fn ctype_w_do_is_range(
    _this: *const CtypeW,
    mut first: *const u16,
    last: *const u16,
    mut dest: *mut u16,
) -> *const u16 {
    while first < last {
        let ch = std::ptr::read_unaligned(first);
        if !dest.is_null() {
            std::ptr::write_unaligned(dest, wchar_ctype_mask(ch));
            dest = dest.add(1);
        }
        first = first.add(1);
    }
    last
}

unsafe extern "win64" fn ctype_w_do_scan_is(
    _this: *const CtypeW,
    mask: u16,
    mut first: *const u16,
    last: *const u16,
) -> *const u16 {
    while first < last {
        if (wchar_ctype_mask(std::ptr::read_unaligned(first)) & mask) != 0 {
            return first;
        }
        first = first.add(1);
    }
    last
}

unsafe extern "win64" fn ctype_w_do_scan_not(
    _this: *const CtypeW,
    mask: u16,
    mut first: *const u16,
    last: *const u16,
) -> *const u16 {
    while first < last {
        if (wchar_ctype_mask(std::ptr::read_unaligned(first)) & mask) == 0 {
            return first;
        }
        first = first.add(1);
    }
    last
}

unsafe extern "win64" fn ctype_w_do_tolower_char(_this: *const CtypeW, ch: u16) -> u16 {
    wchar_to_lower(ch)
}

unsafe extern "win64" fn ctype_w_do_tolower_range(
    _this: *const CtypeW,
    mut first: *mut u16,
    last: *const u16,
) -> *const u16 {
    while (first as *const u16) < last {
        let ch = std::ptr::read_unaligned(first);
        std::ptr::write_unaligned(first, wchar_to_lower(ch));
        first = first.add(1);
    }
    last
}

unsafe extern "win64" fn ctype_w_do_toupper_char(_this: *const CtypeW, ch: u16) -> u16 {
    wchar_to_upper(ch)
}

unsafe extern "win64" fn ctype_w_do_toupper_range(
    _this: *const CtypeW,
    mut first: *mut u16,
    last: *const u16,
) -> *const u16 {
    while (first as *const u16) < last {
        let ch = std::ptr::read_unaligned(first);
        std::ptr::write_unaligned(first, wchar_to_upper(ch));
        first = first.add(1);
    }
    last
}

unsafe extern "win64" fn ctype_w_do_widen_char(_this: *const CtypeW, ch: u8) -> u16 {
    ch as u16
}

unsafe extern "win64" fn ctype_w_do_widen_range(
    _this: *const CtypeW,
    mut first: *const u8,
    last: *const u8,
    mut dest: *mut u16,
) -> *const u8 {
    while first < last {
        if !dest.is_null() {
            std::ptr::write_unaligned(dest, *first as u16);
            dest = dest.add(1);
        }
        first = first.add(1);
    }
    last
}

unsafe extern "win64" fn ctype_w_do_narrow_char(_this: *const CtypeW, ch: u16, dflt: u8) -> u8 {
    if ch <= 0xFF { ch as u8 } else { dflt }
}

unsafe extern "win64" fn ctype_w_do_narrow_range(
    _this: *const CtypeW,
    mut first: *const u16,
    last: *const u16,
    dflt: u8,
    mut dest: *mut u8,
) -> *const u16 {
    while first < last {
        let ch = std::ptr::read_unaligned(first);
        if !dest.is_null() {
            *dest = if ch <= 0xFF { ch as u8 } else { dflt };
            dest = dest.add(1);
        }
        first = first.add(1);
    }
    last
}

unsafe extern "win64" fn ctype_w_getcat(ppf: *mut *const Facet, _ploc: *const c_void) -> usize {
    ensure_vtables_registered();
    if !ppf.is_null() && std::ptr::read_unaligned(ppf).is_null() {
        let p = crate::ucrt::heap::calloc(1, std::mem::size_of::<CtypeW>()) as *mut CtypeW;
        if !p.is_null() {
            write_field(p, std::mem::offset_of!(CtypeW, vtable), CTYPE_W_VTABLE.as_ptr());
            write_field(p, std::mem::offset_of!(CtypeW, refs), 0u32);
            write_field(p, std::mem::offset_of!(CtypeW, page), 0u32);
            write_field(p, std::mem::offset_of!(CtypeW, table), crate::ucrt::string::__pctype_func());
            write_field(p, std::mem::offset_of!(CtypeW, delfl), 0i32);
            locinfo_getcvt(std::ptr::null(), field_ptr::<_, Cvtvec>(p, std::mem::offset_of!(CtypeW, cvt)));
            std::ptr::write_unaligned(ppf, p.cast());
        }
    }
    2 // _X_CTYPE
}

unsafe extern "win64" fn ctype_w_toupper_range(
    this: *const CtypeW,
    first: *mut u16,
    last: *const u16,
) -> *const u16 {
    if !this.is_null() {
        let vtable: *const Ptr = read_field(this, std::mem::offset_of!(CtypeW, vtable));
        if !vtable.is_null() && vtable != CTYPE_W_VTABLE.as_ptr() {
            let f = std::ptr::read_unaligned(vtable.add(9)).0 as usize;
            if f != 0 {
                return crate::call_guest8(f, this as u64, first as u64, last as u64, 0, 0, 0, 0, 0)
                    as *const u16;
            }
        }
    }
    ctype_w_do_toupper_range(this, first, last)
}

unsafe extern "win64" fn ctype_w_widen_char(this: *const CtypeW, ch: u8) -> u16 {
    if !this.is_null() {
        let vtable: *const Ptr = read_field(this, std::mem::offset_of!(CtypeW, vtable));
        if !vtable.is_null() && vtable != CTYPE_W_VTABLE.as_ptr() {
            let f = std::ptr::read_unaligned(vtable.add(12)).0 as usize;
            if f != 0 {
                return crate::call_guest8(f, this as u64, ch as u64, 0, 0, 0, 0, 0, 0) as u16;
            }
        }
    }
    ctype_w_do_widen_char(this, ch)
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
        write_field(this, std::mem::offset_of!(CodecvtU32, vtable), CODECVT_U32_VTABLE.as_ptr());
        write_field(this, std::mem::offset_of!(CodecvtU32, refs), refs as u32);
        write_field(this, std::mem::offset_of!(CodecvtU32, _pad), 0u32);
        write_field(this, std::mem::offset_of!(CodecvtU32, maxcode), 0x10ffffu32);
        write_field(this, std::mem::offset_of!(CodecvtU32, mode), 4u32);
    }
    this
}

unsafe extern "win64" fn codecvt_w_ctor(this: *mut CodecvtW, refs: usize) -> *mut CodecvtW {
    ensure_vtables_registered();
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(CodecvtW, vtable), CODECVT_W_VTABLE.as_ptr());
        write_field(this, std::mem::offset_of!(CodecvtW, refs), refs as u32);
        write_field(this, std::mem::offset_of!(CodecvtW, _pad), 0u32);
        locinfo_getcvt(std::ptr::null(), field_ptr::<_, Cvtvec>(this, std::mem::offset_of!(CodecvtW, cvt)));
    }
    this
}

unsafe extern "win64" fn codecvt_w_getcat(ppf: *mut *const Facet, _ploc: *const c_void) -> usize {
    ensure_vtables_registered();
    if !ppf.is_null() && std::ptr::read_unaligned(ppf).is_null() {
        let p = crate::ucrt::heap::calloc(1, std::mem::size_of::<CodecvtW>()) as *mut CodecvtW;
        if !p.is_null() {
            codecvt_w_ctor(p, 0);
            std::ptr::write_unaligned(ppf, p.cast());
        }
    }
    2 // _X_CTYPE
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

#[inline]
unsafe fn read_field<S, T>(base: *const S, offset: usize) -> T {
    std::ptr::read_unaligned((base as *const u8).add(offset) as *const T)
}

#[inline]
unsafe fn write_field<S, T>(base: *mut S, offset: usize, val: T) {
    std::ptr::write_unaligned((base as *mut u8).add(offset) as *mut T, val)
}

#[inline]
unsafe fn field_ptr<S, T>(base: *mut S, offset: usize) -> *mut T {
    (base as *mut u8).add(offset) as *mut T
}

unsafe extern "win64" fn streambuf_ctor(this: *mut BasicStreambuf) -> *mut BasicStreambuf {
    ensure_vtables_registered();
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(BasicStreambuf, vtable), BASIC_STREAMBUF_VTABLE.as_ptr());
        write_field(this, std::mem::offset_of!(BasicStreambuf, gfirst), std::ptr::null_mut::<u8>());
        write_field(this, std::mem::offset_of!(BasicStreambuf, pfirst), std::ptr::null_mut::<u8>());
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, igfirst),
            field_ptr::<_, *mut u8>(this, std::mem::offset_of!(BasicStreambuf, gfirst)),
        );
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, ipfirst),
            field_ptr::<_, *mut u8>(this, std::mem::offset_of!(BasicStreambuf, pfirst)),
        );
        write_field(this, std::mem::offset_of!(BasicStreambuf, gnext), std::ptr::null_mut::<u8>());
        write_field(this, std::mem::offset_of!(BasicStreambuf, pnext), std::ptr::null_mut::<u8>());
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, ignext),
            field_ptr::<_, *mut u8>(this, std::mem::offset_of!(BasicStreambuf, gnext)),
        );
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, ipnext),
            field_ptr::<_, *mut u8>(this, std::mem::offset_of!(BasicStreambuf, pnext)),
        );
        write_field(this, std::mem::offset_of!(BasicStreambuf, gcount), 0i32);
        write_field(this, std::mem::offset_of!(BasicStreambuf, pcount), 0i32);
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, igcount),
            field_ptr::<_, i32>(this, std::mem::offset_of!(BasicStreambuf, gcount)),
        );
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, ipcount),
            field_ptr::<_, i32>(this, std::mem::offset_of!(BasicStreambuf, pcount)),
        );
        let loc = crate::ucrt::heap::calloc(1, 16) as *mut [usize; 2];
        if !loc.is_null() {
            std::ptr::write_unaligned((loc as *mut usize).add(1), locale_init(1) as usize);
        }
        write_field(this, std::mem::offset_of!(BasicStreambuf, plocale), loc);
    }
    this
}

unsafe extern "win64" fn streambuf_dtor(this: *mut BasicStreambuf) {
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(BasicStreambuf, vtable), BASIC_STREAMBUF_VTABLE.as_ptr());
        let plocale: *mut [usize; 2] = read_field(this, std::mem::offset_of!(BasicStreambuf, plocale));
        if !plocale.is_null() {
            crate::ucrt::heap::free(plocale.cast());
            write_field(this, std::mem::offset_of!(BasicStreambuf, plocale), std::ptr::null_mut::<[usize; 2]>());
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
    let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambuf, vtable));
    if vtable.is_null() {
        return -1;
    }
    let underflow = std::ptr::read_unaligned(vtable.add(6)).0 as usize;
    if crate::call_guest8(underflow, this as u64, 0, 0, 0, 0, 0, 0, 0) as i32 == -1 {
        return -1;
    }
    let cnt: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
    let nxt: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    if cnt.is_null() || nxt.is_null() {
        return -1;
    }
    let p = std::ptr::read_unaligned(nxt);
    if p.is_null() {
        return -1;
    }
    let cur_cnt = std::ptr::read_unaligned(cnt);
    std::ptr::write_unaligned(cnt, cur_cnt.wrapping_sub(1));
    std::ptr::write_unaligned(nxt, p.add(1));
    std::ptr::read_unaligned(p) as i32
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
        let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
        let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
        let src = if !ignext.is_null() { std::ptr::read_unaligned(ignext) } else { std::ptr::null_mut() };
        let cur_cnt = if !igcount.is_null() { std::ptr::read_unaligned(igcount) } else { 0 };
        let avail = if !src.is_null() { cur_cnt.max(0) as i64 } else { 0 };
        if avail > 0 {
            let n = rem.min(avail) as usize;
            std::ptr::copy_nonoverlapping(src, dst, n);
            std::ptr::write_unaligned(ignext, src.add(n));
            std::ptr::write_unaligned(igcount, cur_cnt.wrapping_sub(n as i32));
            dst = dst.add(n);
            rem -= n as i64;
        } else {
            let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambuf, vtable));
            if vtable.is_null() {
                break;
            }
            let uflow = std::ptr::read_unaligned(vtable.add(7)).0 as usize;
            let c = crate::call_guest8(uflow, this as u64, 0, 0, 0, 0, 0, 0, 0) as i32;
            if c == -1 {
                break;
            }
            std::ptr::write_unaligned(dst, c as u8);
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
        let ipnext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipnext));
        let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipcount));
        let dst = if !ipnext.is_null() { std::ptr::read_unaligned(ipnext) } else { std::ptr::null_mut() };
        let cur_cnt = if !ipcount.is_null() { std::ptr::read_unaligned(ipcount) } else { 0 };
        let avail = if !dst.is_null() { cur_cnt.max(0) as i64 } else { 0 };
        if avail > 0 {
            let n = rem.min(avail) as usize;
            std::ptr::copy_nonoverlapping(src, dst, n);
            std::ptr::write_unaligned(ipnext, dst.add(n));
            std::ptr::write_unaligned(ipcount, cur_cnt.wrapping_sub(n as i32));
            src = src.add(n);
            rem -= n as i64;
        } else {
            let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambuf, vtable));
            if vtable.is_null() {
                break;
            }
            let overflow = std::ptr::read_unaligned(vtable.add(3)).0 as usize;
            let ch = std::ptr::read_unaligned(src);
            if crate::call_guest8(overflow, this as u64, ch as u64, 0, 0, 0, 0, 0, 0) as i32 == -1 {
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
    _off: i64,
    _way: i32,
    _mode: i32,
) -> *mut [i64; 3] {
    if !out.is_null() {
        std::ptr::write_unaligned(out, [-1, 0, 0]);
    }
    out
}

unsafe extern "win64" fn streambuf_seekpos(
    _this: *mut BasicStreambuf,
    out: *mut [i64; 3],
    _pos: *const [i64; 3],
    _mode: i32,
) -> *mut [i64; 3] {
    if !out.is_null() {
        std::ptr::write_unaligned(out, [-1, 0, 0]);
    }
    out
}

unsafe extern "win64" fn streambuf_copy_ctor(
    this: *mut BasicStreambuf,
    rhs: *const BasicStreambuf,
) -> *mut BasicStreambuf {
    streambuf_ctor(this);
    if !this.is_null() && !rhs.is_null() {
        let rhs_igfirst: *mut *mut u8 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, igfirst));
        if !rhs_igfirst.is_null() {
            write_field(this, std::mem::offset_of!(BasicStreambuf, gfirst), std::ptr::read_unaligned(rhs_igfirst));
        }
        let rhs_ipfirst: *mut *mut u8 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, ipfirst));
        if !rhs_ipfirst.is_null() {
            write_field(this, std::mem::offset_of!(BasicStreambuf, pfirst), std::ptr::read_unaligned(rhs_ipfirst));
        }
        let rhs_ignext: *mut *mut u8 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, ignext));
        if !rhs_ignext.is_null() {
            write_field(this, std::mem::offset_of!(BasicStreambuf, gnext), std::ptr::read_unaligned(rhs_ignext));
        }
        let rhs_ipnext: *mut *mut u8 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, ipnext));
        if !rhs_ipnext.is_null() {
            write_field(this, std::mem::offset_of!(BasicStreambuf, pnext), std::ptr::read_unaligned(rhs_ipnext));
        }
        let rhs_igcount: *mut i32 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, igcount));
        if !rhs_igcount.is_null() {
            write_field(this, std::mem::offset_of!(BasicStreambuf, gcount), std::ptr::read_unaligned(rhs_igcount));
        }
        let rhs_ipcount: *mut i32 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, ipcount));
        if !rhs_ipcount.is_null() {
            write_field(this, std::mem::offset_of!(BasicStreambuf, pcount), std::ptr::read_unaligned(rhs_ipcount));
        }
    }
    this
}

unsafe extern "win64" fn streambuf_copy_assign(
    this: *mut BasicStreambuf,
    rhs: *const BasicStreambuf,
) -> *mut BasicStreambuf {
    if !this.is_null() && !rhs.is_null() && (this as *const BasicStreambuf) != rhs {
        let lhs_igfirst: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, igfirst));
        let rhs_igfirst: *mut *mut u8 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, igfirst));
        if !lhs_igfirst.is_null() && !rhs_igfirst.is_null() {
            std::ptr::write_unaligned(lhs_igfirst, std::ptr::read_unaligned(rhs_igfirst));
        }
        let lhs_ipfirst: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipfirst));
        let rhs_ipfirst: *mut *mut u8 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, ipfirst));
        if !lhs_ipfirst.is_null() && !rhs_ipfirst.is_null() {
            std::ptr::write_unaligned(lhs_ipfirst, std::ptr::read_unaligned(rhs_ipfirst));
        }
        let lhs_ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
        let rhs_ignext: *mut *mut u8 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, ignext));
        if !lhs_ignext.is_null() && !rhs_ignext.is_null() {
            std::ptr::write_unaligned(lhs_ignext, std::ptr::read_unaligned(rhs_ignext));
        }
        let lhs_ipnext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipnext));
        let rhs_ipnext: *mut *mut u8 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, ipnext));
        if !lhs_ipnext.is_null() && !rhs_ipnext.is_null() {
            std::ptr::write_unaligned(lhs_ipnext, std::ptr::read_unaligned(rhs_ipnext));
        }
        let lhs_igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
        let rhs_igcount: *mut i32 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, igcount));
        if !lhs_igcount.is_null() && !rhs_igcount.is_null() {
            std::ptr::write_unaligned(lhs_igcount, std::ptr::read_unaligned(rhs_igcount));
        }
        let lhs_ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipcount));
        let rhs_ipcount: *mut i32 = read_field(rhs, std::mem::offset_of!(BasicStreambuf, ipcount));
        if !lhs_ipcount.is_null() && !rhs_ipcount.is_null() {
            std::ptr::write_unaligned(lhs_ipcount, std::ptr::read_unaligned(rhs_ipcount));
        }
    }
    this
}

unsafe extern "win64" fn streambuf_init_ptrs(
    this: *mut BasicStreambuf,
    gf: *mut *mut u8,
    gn: *mut *mut u8,
    gc: *mut i32,
    pf: *mut *mut u8,
    pn: *mut *mut u8,
    pc: *mut i32,
) {
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(BasicStreambuf, igfirst), gf);
        write_field(this, std::mem::offset_of!(BasicStreambuf, ignext), gn);
        write_field(this, std::mem::offset_of!(BasicStreambuf, igcount), gc);
        write_field(this, std::mem::offset_of!(BasicStreambuf, ipfirst), pf);
        write_field(this, std::mem::offset_of!(BasicStreambuf, ipnext), pn);
        write_field(this, std::mem::offset_of!(BasicStreambuf, ipcount), pc);
    }
}

unsafe extern "win64" fn streambuf_gnavail(this: *const BasicStreambuf) -> i64 {
    if this.is_null() {
        return 0;
    }
    let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
    if ignext.is_null() || igcount.is_null() || std::ptr::read_unaligned(ignext).is_null() {
        0
    } else {
        std::ptr::read_unaligned(igcount) as i64
    }
}

unsafe extern "win64" fn streambuf_pnavail(this: *const BasicStreambuf) -> i64 {
    if this.is_null() {
        return 0;
    }
    let ipnext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipnext));
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipcount));
    if ipnext.is_null() || ipcount.is_null() || std::ptr::read_unaligned(ipnext).is_null() {
        0
    } else {
        std::ptr::read_unaligned(ipcount) as i64
    }
}

unsafe extern "win64" fn streambuf_gndec(this: *mut BasicStreambuf) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    if ignext.is_null() {
        return std::ptr::null_mut();
    }
    let cur = std::ptr::read_unaligned(ignext);
    if cur.is_null() {
        return std::ptr::null_mut();
    }
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
    if !igcount.is_null() {
        let cnt = std::ptr::read_unaligned(igcount);
        std::ptr::write_unaligned(igcount, cnt.wrapping_add(1));
    }
    let p = cur.wrapping_sub(1);
    std::ptr::write_unaligned(ignext, p);
    p
}

unsafe extern "win64" fn streambuf_gninc(this: *mut BasicStreambuf) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    if ignext.is_null() {
        return std::ptr::null_mut();
    }
    let p = std::ptr::read_unaligned(ignext);
    if p.is_null() {
        return std::ptr::null_mut();
    }
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
    if !igcount.is_null() {
        let cnt = std::ptr::read_unaligned(igcount);
        std::ptr::write_unaligned(igcount, cnt.wrapping_sub(1));
    }
    std::ptr::write_unaligned(ignext, p.wrapping_add(1));
    p
}

unsafe extern "win64" fn streambuf_gbump(this: *mut BasicStreambuf, off: i32) {
    if this.is_null() {
        return;
    }
    let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
    if ignext.is_null() || igcount.is_null() {
        return;
    }
    let cur = std::ptr::read_unaligned(igcount);
    std::ptr::write_unaligned(igcount, cur.wrapping_sub(off));
    let p = std::ptr::read_unaligned(ignext);
    if !p.is_null() {
        std::ptr::write_unaligned(ignext, p.wrapping_offset(off as isize));
    }
}

unsafe extern "win64" fn streambuf_pbump(this: *mut BasicStreambuf, off: i32) {
    if this.is_null() {
        return;
    }
    let ipnext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipnext));
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipcount));
    if ipnext.is_null() || ipcount.is_null() {
        return;
    }
    let cur = std::ptr::read_unaligned(ipcount);
    std::ptr::write_unaligned(ipcount, cur.wrapping_sub(off));
    let p = std::ptr::read_unaligned(ipnext);
    if !p.is_null() {
        std::ptr::write_unaligned(ipnext, p.wrapping_offset(off as isize));
    }
}

unsafe extern "win64" fn streambuf_setg(
    this: *mut BasicStreambuf,
    first: *mut u8,
    next: *mut u8,
    last: *mut u8,
) {
    if this.is_null() {
        return;
    }
    let igfirst: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, igfirst));
    let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
    if !igfirst.is_null() {
        std::ptr::write_unaligned(igfirst, first);
    }
    if !ignext.is_null() {
        std::ptr::write_unaligned(ignext, next);
    }
    if !igcount.is_null() {
        let cnt = if next.is_null() || last.is_null() {
            0
        } else {
            (last as isize).wrapping_sub(next as isize) as i32
        };
        std::ptr::write_unaligned(igcount, cnt);
    }
}

unsafe extern "win64" fn streambuf_setp3(
    this: *mut BasicStreambuf,
    first: *mut u8,
    next: *mut u8,
    last: *mut u8,
) {
    if this.is_null() {
        return;
    }
    let ipfirst: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipfirst));
    let ipnext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipnext));
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipcount));
    if !ipfirst.is_null() {
        std::ptr::write_unaligned(ipfirst, first);
    }
    if !ipnext.is_null() {
        std::ptr::write_unaligned(ipnext, next);
    }
    if !ipcount.is_null() {
        let cnt = if next.is_null() || last.is_null() {
            0
        } else {
            (last as isize).wrapping_sub(next as isize) as i32
        };
        std::ptr::write_unaligned(ipcount, cnt);
    }
}

unsafe extern "win64" fn streambuf_setp2(
    this: *mut BasicStreambuf,
    first: *mut u8,
    last: *mut u8,
) {
    streambuf_setp3(this, first, first, last);
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
    let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
    let p = if !ignext.is_null() { std::ptr::read_unaligned(ignext) } else { std::ptr::null_mut() };
    let cur_cnt = if !igcount.is_null() { std::ptr::read_unaligned(igcount) } else { 0 };
    if !p.is_null() && cur_cnt > 0 {
        std::ptr::write_unaligned(igcount, cur_cnt.wrapping_sub(1));
        std::ptr::write_unaligned(ignext, p.add(1));
        std::ptr::read_unaligned(p) as i32
    } else {
        let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambuf, vtable));
        if vtable.is_null() {
            return -1;
        }
        let uflow = std::ptr::read_unaligned(vtable.add(7)).0 as usize;
        crate::call_guest8(uflow, this as u64, 0, 0, 0, 0, 0, 0, 0) as i32
    }
}

unsafe extern "win64" fn streambuf_sgetc(this: *mut BasicStreambuf) -> i32 {
    if this.is_null() {
        return -1;
    }
    let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
    let p = if !ignext.is_null() { std::ptr::read_unaligned(ignext) } else { std::ptr::null_mut() };
    let cur_cnt = if !igcount.is_null() { std::ptr::read_unaligned(igcount) } else { 0 };
    if !p.is_null() && cur_cnt > 0 {
        std::ptr::read_unaligned(p) as i32
    } else {
        let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambuf, vtable));
        if vtable.is_null() {
            return -1;
        }
        let underflow = std::ptr::read_unaligned(vtable.add(6)).0 as usize;
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
    let ipnext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipnext));
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipcount));
    let p = if !ipnext.is_null() { std::ptr::read_unaligned(ipnext) } else { std::ptr::null_mut() };
    let cur_cnt = if !ipcount.is_null() { std::ptr::read_unaligned(ipcount) } else { 0 };
    if !p.is_null() && cur_cnt > 0 {
        std::ptr::write_unaligned(ipcount, cur_cnt.wrapping_sub(1));
        std::ptr::write_unaligned(ipnext, p.add(1));
        std::ptr::write_unaligned(p, ch);
        ch as i32
    } else {
        let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambuf, vtable));
        if vtable.is_null() {
            return -1;
        }
        let overflow = std::ptr::read_unaligned(vtable.add(3)).0 as usize;
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
    let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambuf, vtable));
    if vtable.is_null() {
        return 0;
    }
    let xsputn = std::ptr::read_unaligned(vtable.add(9)).0 as usize;
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
        std::ptr::write_bytes(this as *mut u8, 0, std::mem::size_of::<BasicIos>());
        write_field(this, std::mem::offset_of!(BasicIos, vtable), BASIC_IOS_VTABLE.as_ptr());
    }
    this
}

unsafe extern "win64" fn ios_ctor_sb(
    this: *mut BasicIos,
    sb: *mut BasicStreambuf,
) -> *mut BasicIos {
    ios_ctor(this);
    ios_init(this, sb);
    this
}

unsafe extern "win64" fn ios_init_export(
    this: *mut BasicIos,
    sb: *mut BasicStreambuf,
    _isstd: u8,
) {
    ios_init(this, sb);
}

unsafe extern "win64" fn ios_base_bool(this: *const BasicIos) -> u8 {
    (!this.is_null() && (read_field::<BasicIos, i32>(this, std::mem::offset_of!(BasicIos, mystate)) & 6) == 0) as u8
}

unsafe extern "win64" fn ios_base_flags(this: *const BasicIos) -> i32 {
    if this.is_null() { 0 } else { read_field(this, std::mem::offset_of!(BasicIos, fmtfl)) }
}

unsafe extern "win64" fn ios_base_setf1(this: *mut BasicIos, newfmt: i32) -> i32 {
    if this.is_null() {
        return 0;
    }
    let old: i32 = read_field(this, std::mem::offset_of!(BasicIos, fmtfl));
    write_field(this, std::mem::offset_of!(BasicIos, fmtfl), (old | newfmt) & 0xFFFF);
    old
}

unsafe extern "win64" fn ios_base_setf2(this: *mut BasicIos, newfmt: i32, mask: i32) -> i32 {
    if this.is_null() {
        return 0;
    }
    let old: i32 = read_field(this, std::mem::offset_of!(BasicIos, fmtfl));
    write_field(
        this,
        std::mem::offset_of!(BasicIos, fmtfl),
        ((old & !mask) | (newfmt & mask)) & 0xFFFF,
    );
    old
}

unsafe extern "win64" fn ios_base_width_get(this: *const BasicIos) -> i64 {
    if this.is_null() { 0 } else { read_field(this, std::mem::offset_of!(BasicIos, wide)) }
}

unsafe extern "win64" fn ios_base_width_set(this: *mut BasicIos, newwidth: i64) -> i64 {
    if this.is_null() {
        return 0;
    }
    let old: i64 = read_field(this, std::mem::offset_of!(BasicIos, wide));
    write_field(this, std::mem::offset_of!(BasicIos, wide), newwidth);
    old
}

unsafe extern "win64" fn ios_fill_get(this: *const BasicIos) -> u8 {
    if this.is_null() { b' ' } else { read_field(this, std::mem::offset_of!(BasicIos, fillch)) }
}

unsafe extern "win64" fn ios_fill_set(this: *mut BasicIos, ch: u8) -> u8 {
    if this.is_null() {
        return b' ';
    }
    let old: u8 = read_field(this, std::mem::offset_of!(BasicIos, fillch));
    write_field(this, std::mem::offset_of!(BasicIos, fillch), ch);
    old
}

unsafe extern "win64" fn ios_rdbuf_get(this: *const BasicIos) -> *mut BasicStreambuf {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        read_field(this, std::mem::offset_of!(BasicIos, mystrbuf))
    }
}

unsafe extern "win64" fn ios_rdbuf_set(
    this: *mut BasicIos,
    sb: *mut BasicStreambuf,
) -> *mut BasicStreambuf {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let old: *mut BasicStreambuf = read_field(this, std::mem::offset_of!(BasicIos, mystrbuf));
    write_field(this, std::mem::offset_of!(BasicIos, mystrbuf), sb);
    write_field(this, std::mem::offset_of!(BasicIos, mystate), if sb.is_null() { 4i32 } else { 0i32 });
    old
}

unsafe extern "win64" fn ios_tie_get(this: *const BasicIos) -> *mut c_void {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        read_field(this, std::mem::offset_of!(BasicIos, tiestr))
    }
}

unsafe extern "win64" fn ios_copyfmt(
    this: *mut BasicIos,
    rhs: *const BasicIos,
) -> *mut BasicIos {
    if this.is_null() || rhs.is_null() || (this as *const BasicIos) == rhs {
        return this;
    }
    write_field::<_, i32>(this, std::mem::offset_of!(BasicIos, fmtfl), read_field(rhs, std::mem::offset_of!(BasicIos, fmtfl)));
    write_field::<_, i64>(this, std::mem::offset_of!(BasicIos, prec), read_field(rhs, std::mem::offset_of!(BasicIos, prec)));
    write_field::<_, i64>(this, std::mem::offset_of!(BasicIos, wide), read_field(rhs, std::mem::offset_of!(BasicIos, wide)));
    write_field::<_, u8>(this, std::mem::offset_of!(BasicIos, fillch), read_field(rhs, std::mem::offset_of!(BasicIos, fillch)));
    write_field::<_, *mut c_void>(this, std::mem::offset_of!(BasicIos, tiestr), read_field(rhs, std::mem::offset_of!(BasicIos, tiestr)));
    write_field::<_, i32>(this, std::mem::offset_of!(BasicIos, except), read_field(rhs, std::mem::offset_of!(BasicIos, except)));
    let rhs_ploc: *mut [usize; 2] = read_field(rhs, std::mem::offset_of!(BasicIos, ploc));
    if !rhs_ploc.is_null() {
        let loc_imp = std::ptr::read_unaligned((rhs_ploc as *const usize).add(1)) as *mut Locimp;
        if !loc_imp.is_null() {
            let refs_ptr = field_ptr::<_, u8>(loc_imp, std::mem::offset_of!(Locimp, refs));
            unaligned_atomic_u32_fetch_add(refs_ptr, 1);
        }
        let mut dst_ploc: *mut [usize; 2] = read_field(this, std::mem::offset_of!(BasicIos, ploc));
        if dst_ploc.is_null() {
            dst_ploc = crate::ucrt::heap::calloc(1, 16) as *mut [usize; 2];
            write_field(this, std::mem::offset_of!(BasicIos, ploc), dst_ploc);
        }
        if !dst_ploc.is_null() {
            std::ptr::write_unaligned((dst_ploc as *mut usize).add(1), loc_imp as usize);
        }
    }
    this
}

unsafe extern "win64" fn ios_dtor(this: *mut BasicIos) {
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(BasicIos, vtable), BASIC_IOS_VTABLE.as_ptr());
        let ploc: *mut [usize; 2] = read_field(this, std::mem::offset_of!(BasicIos, ploc));
        if !ploc.is_null() {
            crate::ucrt::heap::free(ploc.cast());
            write_field(this, std::mem::offset_of!(BasicIos, ploc), std::ptr::null_mut::<[usize; 2]>());
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
    write_field(ios, std::mem::offset_of!(BasicIos, vtable), BASIC_IOS_VTABLE.as_ptr());
    write_field(ios, std::mem::offset_of!(BasicIos, mystate), if sb.is_null() { 4i32 } else { 0i32 });
    write_field(ios, std::mem::offset_of!(BasicIos, except), 0i32);
    write_field(ios, std::mem::offset_of!(BasicIos, fmtfl), 0x201i32);
    write_field(ios, std::mem::offset_of!(BasicIos, prec), 6i64);
    write_field(ios, std::mem::offset_of!(BasicIos, wide), 0i64);
    let loc = crate::ucrt::heap::calloc(1, 16) as *mut [usize; 2];
    if !loc.is_null() {
        std::ptr::write_unaligned((loc as *mut usize).add(1), locale_init(1) as usize);
    }
    write_field(ios, std::mem::offset_of!(BasicIos, ploc), loc);
    write_field(ios, std::mem::offset_of!(BasicIos, mystrbuf), sb);
    write_field(ios, std::mem::offset_of!(BasicIos, tiestr), std::ptr::null_mut::<c_void>());
    write_field(ios, std::mem::offset_of!(BasicIos, fillch), b' ');
}

unsafe extern "win64" fn ios_good(this: *const BasicIos) -> u8 {
    (!this.is_null() && read_field::<BasicIos, i32>(this, std::mem::offset_of!(BasicIos, mystate)) == 0) as u8
}

unsafe extern "win64" fn ios_setstate(this: *mut BasicIos, state: i32, _reraise: u8) {
    if this.is_null() {
        return;
    }
    let sb: *mut BasicStreambuf = read_field(this, std::mem::offset_of!(BasicIos, mystrbuf));
    let extra = if sb.is_null() { 4 } else { 0 };
    let cur: i32 = read_field(this, std::mem::offset_of!(BasicIos, mystate));
    write_field(this, std::mem::offset_of!(BasicIos, mystate), (cur | state | extra) & 0x17);
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
    if ios.is_null() {
        return 0;
    }
    let mystate: i32 = read_field(ios, std::mem::offset_of!(BasicIos, mystate));
    let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
    if mystate != 0 || sb.is_null() {
        ios_setstate(ios, 2, 0);
        return 0;
    }
    let fmtfl: i32 = read_field(ios, std::mem::offset_of!(BasicIos, fmtfl));
    if noskip == 0 && (fmtfl & 1) != 0 {
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
    (read_field::<BasicIos, i32>(ios, std::mem::offset_of!(BasicIos, mystate)) == 0) as u8
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
    if !ios.is_null() {
        let mystate: i32 = read_field(ios, std::mem::offset_of!(BasicIos, mystate));
        let fmtfl: i32 = read_field(ios, std::mem::offset_of!(BasicIos, fmtfl));
        let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
        if mystate == 0 && (fmtfl & 2) != 0 && !sb.is_null() {
            let vtable: *const Ptr = read_field(sb, std::mem::offset_of!(BasicStreambuf, vtable));
            if !vtable.is_null() {
                let sync = std::ptr::read_unaligned(vtable.add(13)).0 as usize;
                if crate::call_guest8(sync, sb as u64, 0, 0, 0, 0, 0, 0, 0) as i32 == -1 {
                    ios_setstate(ios, 4, 0);
                }
            }
        }
    }
}

unsafe extern "win64" fn ostream_flush(this: *mut u8) -> *mut u8 {
    if this.is_null() {
        return this;
    }
    let ios = stream_ios(this);
    if !ios.is_null() {
        let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
        if !sb.is_null() {
            let vtable: *const Ptr = read_field(sb, std::mem::offset_of!(BasicStreambuf, vtable));
            if !vtable.is_null() {
                let sync = std::ptr::read_unaligned(vtable.add(13)).0 as usize;
                if crate::call_guest8(sync, sb as u64, 0, 0, 0, 0, 0, 0, 0) as i32 == -1 {
                    ios_setstate(ios, 4, 0);
                }
            }
        }
    }
    this
}

unsafe extern "win64" fn ostream_put(this: *mut u8, ch: u8) -> *mut u8 {
    if this.is_null() {
        return this;
    }
    let ios = stream_ios(this);
    if !ios.is_null() {
        let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
        if !sb.is_null() && streambuf_sputc(sb, ch) == -1 {
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
    if !ios.is_null() {
        let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
        if !sb.is_null() && streambuf_sputn(sb, s, count) != count {
            ios_setstate(ios, 4, 0);
        }
    }
    this
}

unsafe fn ostream_write_str(this: *mut u8, s: &str) -> *mut u8 {
    ostream_write(this, s.as_ptr(), s.len() as i64)
}

fn format_uint_with_flags(fmtfl: i32, val: u64) -> String {
    let base = fmtfl & 0x0E00;
    let upper = (fmtfl & 0x0004) != 0;
    let showbase = (fmtfl & 0x0008) != 0;
    match base {
        0x0800 => {
            let prefix = if showbase && val != 0 {
                if upper { "0X" } else { "0x" }
            } else {
                ""
            };
            if upper { format!("{prefix}{val:X}") } else { format!("{prefix}{val:x}") }
        }
        0x0400 => {
            let prefix = if showbase && val != 0 { "0" } else { "" };
            format!("{prefix}{val:o}")
        }
        _ => format!("{val}"),
    }
}

fn format_int_with_flags(fmtfl: i32, val: i64) -> String {
    let base = fmtfl & 0x0E00;
    if base == 0x0800 || base == 0x0400 {
        // Per C++ standard, oct/hex formats signed integers as their unsigned representation.
        format_uint_with_flags(fmtfl, val as u64)
    } else if val >= 0 && (fmtfl & 0x0020) != 0 {
        format!("+{val}")
    } else {
        format!("{val}")
    }
}

fn pad_formatted(s: String, fmtfl: i32, width: i64, fill: char) -> String {
    let w = width.max(0) as usize;
    let len = s.chars().count();
    if w <= len {
        return s;
    }
    let pad_n = w - len;
    let pad: String = std::iter::repeat_n(fill, pad_n).collect();
    if (fmtfl & 0x01C0) == 0x0040 {
        // ios_base::left
        format!("{s}{pad}")
    } else {
        format!("{pad}{s}")
    }
}

unsafe fn ostream_write_padded(this: *mut u8, raw: String) -> *mut u8 {
    let ios = stream_ios(this);
    let out = if !ios.is_null() {
        let w: i64 = read_field(ios, std::mem::offset_of!(BasicIos, wide));
        let fmtfl: i32 = read_field(ios, std::mem::offset_of!(BasicIos, fmtfl));
        let fill = read_field::<BasicIos, u8>(ios, std::mem::offset_of!(BasicIos, fillch)) as char;
        write_field(ios, std::mem::offset_of!(BasicIos, wide), 0i64);
        pad_formatted(raw, fmtfl, w, fill)
    } else {
        raw
    };
    ostream_write_str(this, &out)
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

unsafe extern "win64" fn ostream_shl_ios_base_manip(
    this: *mut u8,
    manip: usize,
) -> *mut u8 {
    if this.is_null() || manip == 0 {
        return this;
    }
    let ios = stream_ios(this);
    if !ios.is_null() {
        let _ = crate::call_guest8(manip, ios as u64, 0, 0, 0, 0, 0, 0, 0);
    }
    this
}

unsafe extern "win64" fn ostream_shl_i16(this: *mut u8, v: i16) -> *mut u8 {
    let ios = stream_ios(this);
    let fmtfl: i32 = if ios.is_null() { 0x201 } else { read_field(ios, std::mem::offset_of!(BasicIos, fmtfl)) };
    let s = if (fmtfl & 0x0E00) == 0x0800 || (fmtfl & 0x0E00) == 0x0400 {
        format_uint_with_flags(fmtfl, (v as u16) as u64)
    } else {
        format_int_with_flags(fmtfl, v as i64)
    };
    ostream_write_padded(this, s)
}

unsafe extern "win64" fn ostream_shl_u16(this: *mut u8, v: u16) -> *mut u8 {
    let ios = stream_ios(this);
    let fmtfl: i32 = if ios.is_null() { 0x201 } else { read_field(ios, std::mem::offset_of!(BasicIos, fmtfl)) };
    ostream_write_padded(this, format_uint_with_flags(fmtfl, v as u64))
}

unsafe extern "win64" fn ostream_shl_i32(this: *mut u8, v: i32) -> *mut u8 {
    let ios = stream_ios(this);
    let fmtfl: i32 = if ios.is_null() { 0x201 } else { read_field(ios, std::mem::offset_of!(BasicIos, fmtfl)) };
    let s = if (fmtfl & 0x0E00) == 0x0800 || (fmtfl & 0x0E00) == 0x0400 {
        format_uint_with_flags(fmtfl, (v as u32) as u64)
    } else {
        format_int_with_flags(fmtfl, v as i64)
    };
    ostream_write_padded(this, s)
}

unsafe extern "win64" fn ostream_shl_ptr(this: *mut u8, p: *const c_void) -> *mut u8 {
    let s = format!("{:016X}", p as usize);
    ostream_write_padded(this, s)
}

unsafe extern "win64" fn ostream_tellp(
    this: *mut u8,
    out: *mut [i64; 3],
) -> *mut [i64; 3] {
    if !out.is_null() {
        std::ptr::write_unaligned(out, [-1, 0, 0]);
    }
    if this.is_null() || out.is_null() {
        return out;
    }
    let ios = stream_ios(this);
    if !ios.is_null() {
        let mystate: i32 = read_field(ios, std::mem::offset_of!(BasicIos, mystate));
        let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
        if (mystate & 6) == 0 && !sb.is_null() {
            let vtable: *const Ptr = read_field(sb, std::mem::offset_of!(BasicStreambuf, vtable));
            if !vtable.is_null() {
                let seekoff = std::ptr::read_unaligned(vtable.add(10)).0 as usize;
                if seekoff != 0 {
                    let _ = crate::call_guest8(seekoff, sb as u64, out as u64, 0, 1, 2, 0, 0, 0);
                }
            }
        }
    }
    out
}

unsafe extern "win64" fn ostream_shl_f64(this: *mut u8, v: f64) -> *mut u8 {
    let s = format!("{v}");
    ostream_write_str(this, &s)
}
crate::win64_bridge!(fn ostream_shl_f64(*mut u8, f64) -> *mut u8);

unsafe extern "win64" fn ostream_shl_i64(this: *mut u8, v: i64) -> *mut u8 {
    let ios = stream_ios(this);
    let fmtfl: i32 = if ios.is_null() { 0x201 } else { read_field(ios, std::mem::offset_of!(BasicIos, fmtfl)) };
    ostream_write_padded(this, format_int_with_flags(fmtfl, v))
}

unsafe extern "win64" fn ostream_shl_u64(this: *mut u8, v: u64) -> *mut u8 {
    let ios = stream_ios(this);
    let fmtfl: i32 = if ios.is_null() { 0x201 } else { read_field(ios, std::mem::offset_of!(BasicIos, fmtfl)) };
    ostream_write_padded(this, format_uint_with_flags(fmtfl, v))
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

// ---------------------------------------------------------------------------
// 7b. Wide Streams (`basic_streambuf<wchar_t>`, `basic_ios<wchar_t>`,
//     `basic_ostream<wchar_t>`, `basic_iostream<wchar_t>`)
// ---------------------------------------------------------------------------

const WEOF_U16: u16 = 0xFFFF;

/// `std::basic_streambuf<wchar_t>` (`0x68` bytes on MSVC x64; `gcount`/`pcount` count `wchar_t`s).
#[repr(C)]
pub struct BasicStreambufW {
    pub vtable: *const Ptr,
    pub gfirst: *mut u16,
    pub pfirst: *mut u16,
    pub igfirst: *mut *mut u16,
    pub ipfirst: *mut *mut u16,
    pub gnext: *mut u16,
    pub pnext: *mut u16,
    pub ignext: *mut *mut u16,
    pub ipnext: *mut *mut u16,
    pub gcount: i32,
    pub pcount: i32,
    pub igcount: *mut i32,
    pub ipcount: *mut i32,
    pub plocale: *mut [usize; 2],
}
const _: () = assert!(std::mem::size_of::<BasicStreambufW>() == 0x68);

static BASIC_STREAMBUF_W_VTABLE: [Ptr; 15] = [
    Ptr(streambuf_w_deleting_dtor as *const ()),
    Ptr(streambuf_w_lock as *const ()),
    Ptr(streambuf_w_unlock as *const ()),
    Ptr(streambuf_w_overflow as *const ()),
    Ptr(streambuf_w_pbackfail as *const ()),
    Ptr(streambuf_w_showmanyc as *const ()),
    Ptr(streambuf_w_underflow as *const ()),
    Ptr(streambuf_w_uflow as *const ()),
    Ptr(streambuf_w_xsgetn as *const ()),
    Ptr(streambuf_w_xsputn as *const ()),
    Ptr(streambuf_w_seekoff as *const ()),
    Ptr(streambuf_w_seekpos as *const ()),
    Ptr(streambuf_w_setbuf as *const ()),
    Ptr(streambuf_w_sync as *const ()),
    Ptr(streambuf_w_imbue as *const ()),
];

unsafe extern "win64" fn streambuf_w_ctor(this: *mut BasicStreambufW) -> *mut BasicStreambufW {
    ensure_vtables_registered();
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(BasicStreambufW, vtable), BASIC_STREAMBUF_W_VTABLE.as_ptr());
        write_field(this, std::mem::offset_of!(BasicStreambufW, gfirst), std::ptr::null_mut::<u16>());
        write_field(this, std::mem::offset_of!(BasicStreambufW, pfirst), std::ptr::null_mut::<u16>());
        write_field(
            this,
            std::mem::offset_of!(BasicStreambufW, igfirst),
            field_ptr::<_, *mut u16>(this, std::mem::offset_of!(BasicStreambufW, gfirst)),
        );
        write_field(
            this,
            std::mem::offset_of!(BasicStreambufW, ipfirst),
            field_ptr::<_, *mut u16>(this, std::mem::offset_of!(BasicStreambufW, pfirst)),
        );
        write_field(this, std::mem::offset_of!(BasicStreambufW, gnext), std::ptr::null_mut::<u16>());
        write_field(this, std::mem::offset_of!(BasicStreambufW, pnext), std::ptr::null_mut::<u16>());
        write_field(
            this,
            std::mem::offset_of!(BasicStreambufW, ignext),
            field_ptr::<_, *mut u16>(this, std::mem::offset_of!(BasicStreambufW, gnext)),
        );
        write_field(
            this,
            std::mem::offset_of!(BasicStreambufW, ipnext),
            field_ptr::<_, *mut u16>(this, std::mem::offset_of!(BasicStreambufW, pnext)),
        );
        write_field(this, std::mem::offset_of!(BasicStreambufW, gcount), 0i32);
        write_field(this, std::mem::offset_of!(BasicStreambufW, pcount), 0i32);
        write_field(
            this,
            std::mem::offset_of!(BasicStreambufW, igcount),
            field_ptr::<_, i32>(this, std::mem::offset_of!(BasicStreambufW, gcount)),
        );
        write_field(
            this,
            std::mem::offset_of!(BasicStreambufW, ipcount),
            field_ptr::<_, i32>(this, std::mem::offset_of!(BasicStreambufW, pcount)),
        );
        let loc = crate::ucrt::heap::calloc(1, 16) as *mut [usize; 2];
        if !loc.is_null() {
            std::ptr::write_unaligned((loc as *mut usize).add(1), locale_init(1) as usize);
        }
        write_field(this, std::mem::offset_of!(BasicStreambufW, plocale), loc);
    }
    this
}

unsafe extern "win64" fn streambuf_w_dtor(this: *mut BasicStreambufW) {
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(BasicStreambufW, vtable), BASIC_STREAMBUF_W_VTABLE.as_ptr());
        let plocale: *mut [usize; 2] = read_field(this, std::mem::offset_of!(BasicStreambufW, plocale));
        if !plocale.is_null() {
            crate::ucrt::heap::free(plocale.cast());
            write_field(this, std::mem::offset_of!(BasicStreambufW, plocale), std::ptr::null_mut::<[usize; 2]>());
        }
    }
}

unsafe extern "win64" fn streambuf_w_deleting_dtor(
    this: *mut BasicStreambufW,
    flags: u32,
) -> *mut BasicStreambufW {
    streambuf_w_dtor(this);
    if (flags & 1) != 0 && !this.is_null() {
        crate::ucrt::heap::free(this.cast());
    }
    this
}

unsafe extern "win64" fn streambuf_w_lock(_this: *mut BasicStreambufW) {}
unsafe extern "win64" fn streambuf_w_unlock(_this: *mut BasicStreambufW) {}
unsafe extern "win64" fn streambuf_w_overflow(_this: *mut BasicStreambufW, _ch: u16) -> u16 {
    WEOF_U16
}
unsafe extern "win64" fn streambuf_w_pbackfail(_this: *mut BasicStreambufW, _ch: u16) -> u16 {
    WEOF_U16
}
unsafe extern "win64" fn streambuf_w_showmanyc(_this: *mut BasicStreambufW) -> i64 {
    0
}
unsafe extern "win64" fn streambuf_w_underflow(_this: *mut BasicStreambufW) -> u16 {
    WEOF_U16
}

unsafe extern "win64" fn streambuf_w_uflow(this: *mut BasicStreambufW) -> u16 {
    if this.is_null() {
        return WEOF_U16;
    }
    let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambufW, vtable));
    if vtable.is_null() {
        return WEOF_U16;
    }
    let underflow = std::ptr::read_unaligned(vtable.add(6)).0 as usize;
    if crate::call_guest8(underflow, this as u64, 0, 0, 0, 0, 0, 0, 0) as u16 == WEOF_U16 {
        return WEOF_U16;
    }
    let cnt: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, igcount));
    let nxt: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ignext));
    if cnt.is_null() || nxt.is_null() {
        return WEOF_U16;
    }
    let p = std::ptr::read_unaligned(nxt);
    if p.is_null() {
        return WEOF_U16;
    }
    let cur_cnt = std::ptr::read_unaligned(cnt);
    std::ptr::write_unaligned(cnt, cur_cnt.wrapping_sub(1));
    std::ptr::write_unaligned(nxt, p.add(1));
    std::ptr::read_unaligned(p)
}

unsafe extern "win64" fn streambuf_w_xsgetn(
    this: *mut BasicStreambufW,
    mut dst: *mut u16,
    count: i64,
) -> i64 {
    if this.is_null() || dst.is_null() || count <= 0 {
        return 0;
    }
    let mut rem = count;
    while rem > 0 {
        let ignext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ignext));
        let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, igcount));
        let src = if !ignext.is_null() { std::ptr::read_unaligned(ignext) } else { std::ptr::null_mut() };
        let cur_cnt = if !igcount.is_null() { std::ptr::read_unaligned(igcount) } else { 0 };
        let avail = if !src.is_null() { cur_cnt.max(0) as i64 } else { 0 };
        if avail > 0 {
            let n = rem.min(avail) as usize;
            std::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, n * 2);
            std::ptr::write_unaligned(ignext, src.add(n));
            std::ptr::write_unaligned(igcount, cur_cnt.wrapping_sub(n as i32));
            dst = dst.add(n);
            rem -= n as i64;
        } else {
            let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambufW, vtable));
            if vtable.is_null() {
                break;
            }
            let uflow = std::ptr::read_unaligned(vtable.add(7)).0 as usize;
            let c = crate::call_guest8(uflow, this as u64, 0, 0, 0, 0, 0, 0, 0) as u16;
            if c == WEOF_U16 {
                break;
            }
            std::ptr::write_unaligned(dst, c);
            dst = dst.add(1);
            rem -= 1;
        }
    }
    count - rem
}

unsafe extern "win64" fn streambuf_w_xsputn(
    this: *mut BasicStreambufW,
    mut src: *const u16,
    count: i64,
) -> i64 {
    if this.is_null() || src.is_null() || count <= 0 {
        return 0;
    }
    let mut rem = count;
    while rem > 0 {
        let ipnext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipnext));
        let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipcount));
        let dst = if !ipnext.is_null() { std::ptr::read_unaligned(ipnext) } else { std::ptr::null_mut() };
        let cur_cnt = if !ipcount.is_null() { std::ptr::read_unaligned(ipcount) } else { 0 };
        let avail = if !dst.is_null() { cur_cnt.max(0) as i64 } else { 0 };
        if avail > 0 {
            let n = rem.min(avail) as usize;
            std::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, n * 2);
            std::ptr::write_unaligned(ipnext, dst.add(n));
            std::ptr::write_unaligned(ipcount, cur_cnt.wrapping_sub(n as i32));
            src = src.add(n);
            rem -= n as i64;
        } else {
            let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambufW, vtable));
            if vtable.is_null() {
                break;
            }
            let overflow = std::ptr::read_unaligned(vtable.add(3)).0 as usize;
            let ch = std::ptr::read_unaligned(src);
            if crate::call_guest8(overflow, this as u64, ch as u64, 0, 0, 0, 0, 0, 0) as u16
                == WEOF_U16
            {
                break;
            }
            src = src.add(1);
            rem -= 1;
        }
    }
    count - rem
}

unsafe extern "win64" fn streambuf_w_seekoff(
    _this: *mut BasicStreambufW,
    out: *mut [i64; 3],
    _off: i64,
    _way: i32,
    _mode: i32,
) -> *mut [i64; 3] {
    if !out.is_null() {
        std::ptr::write_unaligned(out, [-1, 0, 0]);
    }
    out
}

unsafe extern "win64" fn streambuf_w_seekpos(
    _this: *mut BasicStreambufW,
    out: *mut [i64; 3],
    _pos: *const [i64; 3],
    _mode: i32,
) -> *mut [i64; 3] {
    if !out.is_null() {
        std::ptr::write_unaligned(out, [-1, 0, 0]);
    }
    out
}

unsafe extern "win64" fn streambuf_w_setbuf(
    this: *mut BasicStreambufW,
    _buf: *mut u16,
    _count: i64,
) -> *mut BasicStreambufW {
    this
}

unsafe extern "win64" fn streambuf_w_sync(_this: *mut BasicStreambufW) -> i32 {
    0
}

unsafe extern "win64" fn streambuf_w_imbue(_this: *mut BasicStreambufW, _loc: *const c_void) {}

unsafe extern "win64" fn streambuf_w_sputc(this: *mut BasicStreambufW, ch: u16) -> u16 {
    if this.is_null() {
        return WEOF_U16;
    }
    let ipnext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipnext));
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipcount));
    let p = if !ipnext.is_null() { std::ptr::read_unaligned(ipnext) } else { std::ptr::null_mut() };
    let cur_cnt = if !ipcount.is_null() { std::ptr::read_unaligned(ipcount) } else { 0 };
    if !p.is_null() && cur_cnt > 0 {
        std::ptr::write_unaligned(ipcount, cur_cnt.wrapping_sub(1));
        std::ptr::write_unaligned(ipnext, p.add(1));
        std::ptr::write_unaligned(p, ch);
        ch
    } else {
        let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambufW, vtable));
        if vtable.is_null() {
            return WEOF_U16;
        }
        let overflow = std::ptr::read_unaligned(vtable.add(3)).0 as usize;
        crate::call_guest8(overflow, this as u64, ch as u64, 0, 0, 0, 0, 0, 0) as u16
    }
}

unsafe extern "win64" fn streambuf_w_sputn(
    this: *mut BasicStreambufW,
    s: *const u16,
    count: i64,
) -> i64 {
    if this.is_null() {
        return 0;
    }
    let vtable: *const Ptr = read_field(this, std::mem::offset_of!(BasicStreambufW, vtable));
    if vtable.is_null() {
        return 0;
    }
    let xsputn = std::ptr::read_unaligned(vtable.add(9)).0 as usize;
    crate::call_guest8(xsputn, this as u64, s as u64, count as u64, 0, 0, 0, 0, 0) as i64
}

unsafe extern "win64" fn streambuf_w_pninc(this: *mut BasicStreambufW) -> *mut u16 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ipnext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipnext));
    if ipnext.is_null() {
        return std::ptr::null_mut();
    }
    let p = std::ptr::read_unaligned(ipnext);
    if p.is_null() {
        return std::ptr::null_mut();
    }
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipcount));
    if !ipcount.is_null() {
        let cur = std::ptr::read_unaligned(ipcount);
        std::ptr::write_unaligned(ipcount, cur.wrapping_sub(1));
    }
    std::ptr::write_unaligned(ipnext, p.add(1));
    p
}

unsafe extern "win64" fn streambuf_w_eback(this: *const BasicStreambufW) -> *mut u16 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let igfirst: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, igfirst));
    if igfirst.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(igfirst) }
}

unsafe extern "win64" fn streambuf_w_gptr(this: *const BasicStreambufW) -> *mut u16 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ignext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ignext));
    if ignext.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(ignext) }
}

unsafe extern "win64" fn streambuf_w_egptr(this: *const BasicStreambufW) -> *mut u16 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ignext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ignext));
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, igcount));
    if ignext.is_null() || igcount.is_null() {
        return std::ptr::null_mut();
    }
    let p = std::ptr::read_unaligned(ignext);
    if p.is_null() {
        std::ptr::null_mut()
    } else {
        p.wrapping_offset(std::ptr::read_unaligned(igcount) as isize)
    }
}

unsafe extern "win64" fn streambuf_w_pbase(this: *const BasicStreambufW) -> *mut u16 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ipfirst: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipfirst));
    if ipfirst.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(ipfirst) }
}

unsafe extern "win64" fn streambuf_w_pptr(this: *const BasicStreambufW) -> *mut u16 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ipnext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipnext));
    if ipnext.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(ipnext) }
}

unsafe extern "win64" fn streambuf_w_epptr(this: *const BasicStreambufW) -> *mut u16 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ipnext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipnext));
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipcount));
    if ipnext.is_null() || ipcount.is_null() {
        return std::ptr::null_mut();
    }
    let p = std::ptr::read_unaligned(ipnext);
    if p.is_null() {
        std::ptr::null_mut()
    } else {
        p.wrapping_offset(std::ptr::read_unaligned(ipcount) as isize)
    }
}

unsafe extern "win64" fn streambuf_w_gbump(this: *mut BasicStreambufW, off: i32) {
    if this.is_null() {
        return;
    }
    let ignext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ignext));
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, igcount));
    if ignext.is_null() || igcount.is_null() {
        return;
    }
    let cur = std::ptr::read_unaligned(igcount);
    std::ptr::write_unaligned(igcount, cur.wrapping_sub(off));
    let p = std::ptr::read_unaligned(ignext);
    if !p.is_null() {
        std::ptr::write_unaligned(ignext, p.wrapping_offset(off as isize));
    }
}

unsafe extern "win64" fn streambuf_w_setg(
    this: *mut BasicStreambufW,
    first: *mut u16,
    next: *mut u16,
    last: *mut u16,
) {
    if this.is_null() {
        return;
    }
    let igfirst: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, igfirst));
    let ignext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ignext));
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, igcount));
    if !igfirst.is_null() {
        std::ptr::write_unaligned(igfirst, first);
    }
    if !ignext.is_null() {
        std::ptr::write_unaligned(ignext, next);
    }
    if !igcount.is_null() {
        let cnt = if next.is_null() || last.is_null() {
            0
        } else {
            ((last as isize).wrapping_sub(next as isize) / 2) as i32
        };
        std::ptr::write_unaligned(igcount, cnt);
    }
}

unsafe extern "win64" fn streambuf_w_setp3(
    this: *mut BasicStreambufW,
    first: *mut u16,
    next: *mut u16,
    last: *mut u16,
) {
    if this.is_null() {
        return;
    }
    let ipfirst: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipfirst));
    let ipnext: *mut *mut u16 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipnext));
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambufW, ipcount));
    if !ipfirst.is_null() {
        std::ptr::write_unaligned(ipfirst, first);
    }
    if !ipnext.is_null() {
        std::ptr::write_unaligned(ipnext, next);
    }
    if !ipcount.is_null() {
        let cnt = if next.is_null() || last.is_null() {
            0
        } else {
            ((last as isize).wrapping_sub(next as isize) / 2) as i32
        };
        std::ptr::write_unaligned(ipcount, cnt);
    }
}

unsafe extern "win64" fn streambuf_w_setp2(
    this: *mut BasicStreambufW,
    first: *mut u16,
    last: *mut u16,
) {
    streambuf_w_setp3(this, first, first, last);
}

/// `std::basic_ios<wchar_t>` (`0x60` bytes on MSVC x64).
#[repr(C)]
pub struct BasicIosW {
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
    pub mystrbuf: *mut BasicStreambufW,
    pub tiestr: *mut c_void,
    pub fillch: u16,
    pub _pad1: [u8; 6],
}
const _: () = assert!(std::mem::size_of::<BasicIosW>() == 0x60);
const _: () = assert!(std::mem::offset_of!(BasicIos, prec) == std::mem::offset_of!(BasicIosW, prec));
const _: () = assert!(std::mem::offset_of!(BasicIos, wide) == std::mem::offset_of!(BasicIosW, wide));

static BASIC_IOS_W_VTABLE: [Ptr; 1] = [Ptr(ios_w_deleting_dtor as *const ())];

unsafe extern "win64" fn ios_w_ctor(this: *mut BasicIosW) -> *mut BasicIosW {
    ensure_vtables_registered();
    if !this.is_null() {
        std::ptr::write_bytes(this as *mut u8, 0, std::mem::size_of::<BasicIosW>());
        write_field(this, std::mem::offset_of!(BasicIosW, vtable), BASIC_IOS_W_VTABLE.as_ptr());
    }
    this
}

unsafe extern "win64" fn ios_w_dtor(this: *mut BasicIosW) {
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(BasicIosW, vtable), BASIC_IOS_W_VTABLE.as_ptr());
        let ploc: *mut [usize; 2] = read_field(this, std::mem::offset_of!(BasicIosW, ploc));
        if !ploc.is_null() {
            crate::ucrt::heap::free(ploc.cast());
            write_field(this, std::mem::offset_of!(BasicIosW, ploc), std::ptr::null_mut::<[usize; 2]>());
        }
    }
}

unsafe extern "win64" fn ios_w_deleting_dtor(this: *mut BasicIosW, flags: u32) -> *mut BasicIosW {
    ios_w_dtor(this);
    if (flags & 1) != 0 && !this.is_null() {
        crate::ucrt::heap::free(this.cast());
    }
    this
}

unsafe fn ios_w_init(ios: *mut BasicIosW, sb: *mut BasicStreambufW) {
    if ios.is_null() {
        return;
    }
    write_field(ios, std::mem::offset_of!(BasicIosW, vtable), BASIC_IOS_W_VTABLE.as_ptr());
    write_field(ios, std::mem::offset_of!(BasicIosW, mystate), if sb.is_null() { 4i32 } else { 0i32 });
    write_field(ios, std::mem::offset_of!(BasicIosW, except), 0i32);
    write_field(ios, std::mem::offset_of!(BasicIosW, fmtfl), 0x201i32);
    write_field(ios, std::mem::offset_of!(BasicIosW, prec), 6i64);
    write_field(ios, std::mem::offset_of!(BasicIosW, wide), 0i64);
    let loc = crate::ucrt::heap::calloc(1, 16) as *mut [usize; 2];
    if !loc.is_null() {
        std::ptr::write_unaligned((loc as *mut usize).add(1), locale_init(1) as usize);
    }
    write_field(ios, std::mem::offset_of!(BasicIosW, ploc), loc);
    write_field(ios, std::mem::offset_of!(BasicIosW, mystrbuf), sb);
    write_field(ios, std::mem::offset_of!(BasicIosW, tiestr), std::ptr::null_mut::<c_void>());
    write_field(ios, std::mem::offset_of!(BasicIosW, fillch), b' ' as u16);
}

unsafe extern "win64" fn ios_w_setstate(this: *mut BasicIosW, state: i32, _reraise: u8) {
    if this.is_null() {
        return;
    }
    let sb: *mut BasicStreambufW = read_field(this, std::mem::offset_of!(BasicIosW, mystrbuf));
    let extra = if sb.is_null() { 4 } else { 0 };
    let cur: i32 = read_field(this, std::mem::offset_of!(BasicIosW, mystate));
    write_field(this, std::mem::offset_of!(BasicIosW, mystate), (cur | state | extra) & 0x17);
}

unsafe extern "win64" fn ios_w_fill_get(this: *const BasicIosW) -> u16 {
    if this.is_null() { b' ' as u16 } else { read_field(this, std::mem::offset_of!(BasicIosW, fillch)) }
}

unsafe extern "win64" fn ios_w_fill_set(this: *mut BasicIosW, ch: u16) -> u16 {
    if this.is_null() {
        return b' ' as u16;
    }
    let old: u16 = read_field(this, std::mem::offset_of!(BasicIosW, fillch));
    write_field(this, std::mem::offset_of!(BasicIosW, fillch), ch);
    old
}

unsafe extern "win64" fn ios_w_rdbuf_get(this: *const BasicIosW) -> *mut BasicStreambufW {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        read_field(this, std::mem::offset_of!(BasicIosW, mystrbuf))
    }
}

unsafe extern "win64" fn ios_w_tie_get(this: *const BasicIosW) -> *mut c_void {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        read_field(this, std::mem::offset_of!(BasicIosW, tiestr))
    }
}

#[inline]
unsafe fn stream_ios_w(stream: *mut u8) -> *mut BasicIosW {
    stream_ios(stream) as *mut BasicIosW
}

unsafe extern "win64" fn iostream_w_ctor(
    this: *mut u8,
    sb: *mut BasicStreambufW,
    most_derived: i32,
) -> *mut u8 {
    ensure_vtables_registered();
    if this.is_null() {
        return this;
    }
    if most_derived != 0 {
        std::ptr::write_unaligned(this as *mut *const i32, IOSTREAM_IN_VBTABLE.as_ptr());
        std::ptr::write_unaligned(this.add(0x10) as *mut *const i32, IOSTREAM_OUT_VBTABLE.as_ptr());
        std::ptr::write_bytes(this.add(0x20), 0, std::mem::size_of::<BasicIosW>());
    }
    std::ptr::write_unaligned(this.add(8) as *mut u64, 0);
    let ios = stream_ios_w(this);
    if !ios.is_null() {
        std::ptr::write_unaligned((ios as *mut u8).sub(4) as *mut i32, 0);
        ios_w_init(ios, sb);
    }
    this
}

unsafe extern "win64" fn ostream_w_osfx(this: *mut u8) {
    if this.is_null() {
        return;
    }
    let ios = stream_ios_w(this);
    if !ios.is_null() {
        let mystate: i32 = read_field(ios, std::mem::offset_of!(BasicIosW, mystate));
        let fmtfl: i32 = read_field(ios, std::mem::offset_of!(BasicIosW, fmtfl));
        let sb: *mut BasicStreambufW = read_field(ios, std::mem::offset_of!(BasicIosW, mystrbuf));
        if mystate == 0 && (fmtfl & 2) != 0 && !sb.is_null() {
            let vtable: *const Ptr = read_field(sb, std::mem::offset_of!(BasicStreambufW, vtable));
            if !vtable.is_null() {
                let sync = std::ptr::read_unaligned(vtable.add(13)).0 as usize;
                if crate::call_guest8(sync, sb as u64, 0, 0, 0, 0, 0, 0, 0) as i32 == -1 {
                    ios_w_setstate(ios, 4, 0);
                }
            }
        }
    }
}

unsafe extern "win64" fn ostream_w_flush(this: *mut u8) -> *mut u8 {
    if this.is_null() {
        return this;
    }
    let ios = stream_ios_w(this);
    if !ios.is_null() {
        let sb: *mut BasicStreambufW = read_field(ios, std::mem::offset_of!(BasicIosW, mystrbuf));
        if !sb.is_null() {
            let vtable: *const Ptr = read_field(sb, std::mem::offset_of!(BasicStreambufW, vtable));
            if !vtable.is_null() {
                let sync = std::ptr::read_unaligned(vtable.add(13)).0 as usize;
                if crate::call_guest8(sync, sb as u64, 0, 0, 0, 0, 0, 0, 0) as i32 == -1 {
                    ios_w_setstate(ios, 4, 0);
                }
            }
        }
    }
    this
}

unsafe extern "win64" fn ostream_w_shl_u32(this: *mut u8, v: u32) -> *mut u8 {
    if this.is_null() {
        return this;
    }
    let ios = stream_ios_w(this);
    let (fmtfl, w, fill, sb) = if !ios.is_null() {
        let f: i32 = read_field(ios, std::mem::offset_of!(BasicIosW, fmtfl));
        let wi: i64 = read_field(ios, std::mem::offset_of!(BasicIosW, wide));
        let fillch: u16 = read_field(ios, std::mem::offset_of!(BasicIosW, fillch));
        let fl = char::from_u32(fillch as u32).unwrap_or(' ');
        write_field(ios, std::mem::offset_of!(BasicIosW, wide), 0i64);
        let sb: *mut BasicStreambufW = read_field(ios, std::mem::offset_of!(BasicIosW, mystrbuf));
        (f, wi, fl, sb)
    } else {
        (0x201, 0, ' ', std::ptr::null_mut())
    };
    let s = pad_formatted(format_uint_with_flags(fmtfl, v as u64), fmtfl, w, fill);
    let units: Vec<u16> = s.encode_utf16().collect();
    if !ios.is_null() && !sb.is_null() && !units.is_empty() {
        if streambuf_w_sputn(sb, units.as_ptr(), units.len() as i64) != units.len() as i64 {
            ios_w_setstate(ios, 4, 0);
        }
    }
    this
}

pub(crate) fn register_static_vtables() {
    crate::register_native_va(
        chore_work_callback as *const () as usize as u64,
        chore_work_callback as *const () as usize,
    );
    crate::register_native_va(
        noop_method as *const () as usize as u64,
        noop_method as *const () as usize,
    );
    crate::register_native_va(
        smanip_setw_fn as *const () as usize as u64,
        smanip_setw_fn as *const () as usize,
    );
    crate::register_native_va(
        smanip_setprecision_fn as *const () as usize as u64,
        smanip_setprecision_fn as *const () as usize,
    );
    for fn_addr in [
        system_error_unwind_dtor as *const () as usize,
        system_error_copy_ctor as *const () as usize,
        runtime_error_copy_ctor as *const () as usize,
        exception_copy_ctor as *const () as usize,
    ] {
        crate::register_native_va(fn_addr as u64, fn_addr);
    }
    for p in FACET_VTABLE
        .iter()
        .chain(EXCEPTION_VTABLE.iter())
        .chain(RUNTIME_ERROR_VTABLE.iter())
        .chain(SYSTEM_ERROR_VTABLE.iter())
        .chain(SYSTEM_CATEGORY_VTABLE.iter())
        .chain(GENERIC_CATEGORY_VTABLE.iter())
        .chain(CTYPE_CHAR_VTABLE.iter())
        .chain(CTYPE_W_VTABLE.iter())
        .chain(CODECVT_U32_VTABLE.iter())
        .chain(CODECVT_W_VTABLE.iter())
        .chain(BASIC_STREAMBUF_VTABLE.iter())
        .chain(STDIO_STREAMBUF_VTABLE.iter())
        .chain(BASIC_IOS_VTABLE.iter())
        .chain(BASIC_STREAMBUF_W_VTABLE.iter())
        .chain(BASIC_IOS_W_VTABLE.iter())
    {
        let fn_addr = p.0 as usize;
        crate::register_native_va(fn_addr as u64, fn_addr);
    }
}

fn ensure_vtables_registered() {
    static REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if REGISTERED.load(std::sync::atomic::Ordering::Acquire) {
        return;
    }
    if crate::has_native_va_registrar() {
        register_static_vtables();
        REGISTERED.store(true, std::sync::atomic::Ordering::Release);
    }
}

pub static ID_CODECVT_D: LocaleId = LocaleId(UnsafeCell::new([5, 0]));
pub static ID_NUMPUNCT_W: LocaleId = LocaleId(UnsafeCell::new([6, 0]));
pub static ID_TIME_PUT_D: LocaleId = LocaleId(UnsafeCell::new([7, 0]));
pub static LOCALE_ID_CNT: AtomicU32 = AtomicU32::new(9);

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
    let ios = stream_ios(this);
    let fmtfl: i32 = if ios.is_null() { 0x201 } else { read_field(ios, std::mem::offset_of!(BasicIos, fmtfl)) };
    ostream_write_padded(this, format_uint_with_flags(fmtfl, v as u64))
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
    if !ppf.is_null() && std::ptr::read_unaligned(ppf).is_null() {
        let p = crate::ucrt::heap::calloc(1, std::mem::size_of::<CodecvtU32>()) as *mut CodecvtU32;
        if !p.is_null() {
            (*p).vtable = CODECVT_U32_VTABLE.as_ptr();
            (*p).refs = AtomicU32::new(0);
            std::ptr::write_unaligned(ppf, p.cast());
        }
    }
    2
}
unsafe extern "win64" fn time_put_getcat(ppf: *mut *const Facet, _ploc: *const c_void) -> usize {
    ensure_vtables_registered();
    if !ppf.is_null() && std::ptr::read_unaligned(ppf).is_null() {
        let p = crate::ucrt::heap::calloc(1, std::mem::size_of::<Facet>()) as *mut Facet;
        if !p.is_null() {
            (*p).vtable = FACET_VTABLE.as_ptr();
            (*p).refs = AtomicU32::new(0);
            std::ptr::write_unaligned(ppf, p);
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
        write_field(this, std::mem::offset_of!(BasicStreambuf, gfirst), std::ptr::null_mut::<u8>());
        write_field(this, std::mem::offset_of!(BasicStreambuf, pfirst), std::ptr::null_mut::<u8>());
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, igfirst),
            field_ptr::<_, *mut u8>(this, std::mem::offset_of!(BasicStreambuf, gfirst)),
        );
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, ipfirst),
            field_ptr::<_, *mut u8>(this, std::mem::offset_of!(BasicStreambuf, pfirst)),
        );
        write_field(this, std::mem::offset_of!(BasicStreambuf, gnext), std::ptr::null_mut::<u8>());
        write_field(this, std::mem::offset_of!(BasicStreambuf, pnext), std::ptr::null_mut::<u8>());
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, ignext),
            field_ptr::<_, *mut u8>(this, std::mem::offset_of!(BasicStreambuf, gnext)),
        );
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, ipnext),
            field_ptr::<_, *mut u8>(this, std::mem::offset_of!(BasicStreambuf, pnext)),
        );
        write_field(this, std::mem::offset_of!(BasicStreambuf, gcount), 0i32);
        write_field(this, std::mem::offset_of!(BasicStreambuf, pcount), 0i32);
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, igcount),
            field_ptr::<_, i32>(this, std::mem::offset_of!(BasicStreambuf, gcount)),
        );
        write_field(
            this,
            std::mem::offset_of!(BasicStreambuf, ipcount),
            field_ptr::<_, i32>(this, std::mem::offset_of!(BasicStreambuf, pcount)),
        );
    }
}
unsafe extern "win64" fn streambuf_pninc(this: *mut BasicStreambuf) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ipnext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipnext));
    if ipnext.is_null() {
        return std::ptr::null_mut();
    }
    let p = std::ptr::read_unaligned(ipnext);
    if p.is_null() {
        return std::ptr::null_mut();
    }
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipcount));
    if !ipcount.is_null() {
        let cur = std::ptr::read_unaligned(ipcount);
        std::ptr::write_unaligned(ipcount, cur.wrapping_sub(1));
    }
    std::ptr::write_unaligned(ipnext, p.add(1));
    p
}
unsafe extern "win64" fn streambuf_eback(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let igfirst: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, igfirst));
    if igfirst.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(igfirst) }
}
unsafe extern "win64" fn streambuf_gptr(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    if ignext.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(ignext) }
}
unsafe extern "win64" fn streambuf_egptr(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ignext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ignext));
    let igcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, igcount));
    if ignext.is_null() || igcount.is_null() {
        return std::ptr::null_mut();
    }
    let p = std::ptr::read_unaligned(ignext);
    if p.is_null() {
        std::ptr::null_mut()
    } else {
        p.wrapping_offset(std::ptr::read_unaligned(igcount) as isize)
    }
}
unsafe extern "win64" fn streambuf_pbase(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ipfirst: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipfirst));
    if ipfirst.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(ipfirst) }
}
unsafe extern "win64" fn streambuf_pptr(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ipnext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipnext));
    if ipnext.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(ipnext) }
}
unsafe extern "win64" fn streambuf_epptr(this: *const BasicStreambuf) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ipnext: *mut *mut u8 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipnext));
    let ipcount: *mut i32 = read_field(this, std::mem::offset_of!(BasicStreambuf, ipcount));
    if ipnext.is_null() || ipcount.is_null() {
        return std::ptr::null_mut();
    }
    let p = std::ptr::read_unaligned(ipnext);
    if p.is_null() {
        std::ptr::null_mut()
    } else {
        p.wrapping_offset(std::ptr::read_unaligned(ipcount) as isize)
    }
}
unsafe extern "win64" fn streambuf_getloc(_this: *const BasicStreambuf, out: *mut [usize; 2]) -> *mut [usize; 2] {
    if !out.is_null() {
        std::ptr::write_unaligned(out, [0, locale_init(1) as usize]);
    }
    out
}
unsafe extern "win64" fn ios_clear(this: *mut BasicIos, state: i32, _reraise: u8) {
    if !this.is_null() {
        write_field(this, std::mem::offset_of!(BasicIos, mystate), state & 0x17);
    }
}
unsafe extern "win64" fn istream_read(this: *mut u8, dst: *mut u8, count: i64) -> *mut u8 {
    if this.is_null() || dst.is_null() || count <= 0 {
        return this;
    }
    let ios = stream_ios(this);
    if !ios.is_null() {
        let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
        if !sb.is_null() {
            let got = streambuf_xsgetn(sb, dst, count);
            std::ptr::write_unaligned(this.add(8) as *mut i64, got);
            if got != count {
                ios_setstate(ios, 3, 0);
            }
        }
    }
    this
}
unsafe extern "win64" fn istream_seekg_pos(this: *mut u8, pos: *const [i64; 3]) -> *mut u8 {
    if this.is_null() {
        return this;
    }
    let ios = stream_ios(this);
    if ios.is_null() {
        return this;
    }
    // Per C++11 [istream.unformatted]: seekg clears eofbit first, then calls rdbuf()->pubseekpos
    // if !fail(), setting failbit when the returned fpos converts to streamoff(-1).
    let cur: i32 = read_field(ios, std::mem::offset_of!(BasicIos, mystate));
    let cleared = cur & !1;
    write_field(ios, std::mem::offset_of!(BasicIos, mystate), cleared);
    if (cleared & 6) == 0 {
        let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
        let mut out_pos = [-1i64, 0, 0];
        if !sb.is_null() && !pos.is_null() {
            let vtable: *const Ptr = read_field(sb, std::mem::offset_of!(BasicStreambuf, vtable));
            if !vtable.is_null() {
                let seekpos = std::ptr::read_unaligned(vtable.add(11)).0 as usize;
                if seekpos != 0 {
                    let pos_copy = std::ptr::read_unaligned(pos);
                    let _ = crate::call_guest8(
                        seekpos,
                        sb as u64,
                        &mut out_pos as *mut [i64; 3] as u64,
                        &pos_copy as *const [i64; 3] as u64,
                        1, // ios_base::in
                        0,
                        0,
                        0,
                        0,
                    );
                }
            }
        }
        if out_pos[0].wrapping_add(out_pos[1]) == -1 {
            ios_setstate(ios, 2, 0);
        }
    }
    this
}

unsafe extern "win64" fn istream_seekg(this: *mut u8, off: i64, way: i32) -> *mut u8 {
    if this.is_null() {
        return this;
    }
    let ios = stream_ios(this);
    if ios.is_null() {
        return this;
    }
    let cur: i32 = read_field(ios, std::mem::offset_of!(BasicIos, mystate));
    let cleared = cur & !1;
    write_field(ios, std::mem::offset_of!(BasicIos, mystate), cleared);
    if (cleared & 6) == 0 {
        let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
        let mut out_pos = [-1i64, 0, 0];
        if !sb.is_null() {
            let vtable: *const Ptr = read_field(sb, std::mem::offset_of!(BasicStreambuf, vtable));
            if !vtable.is_null() {
                let seekoff = std::ptr::read_unaligned(vtable.add(10)).0 as usize;
                if seekoff != 0 {
                    let _ = crate::call_guest8(
                        seekoff,
                        sb as u64,
                        &mut out_pos as *mut [i64; 3] as u64,
                        off as u64,
                        way as u32 as u64,
                        1, // ios_base::in
                        0,
                        0,
                        0,
                    );
                }
            }
        }
        if out_pos[0].wrapping_add(out_pos[1]) == -1 {
            ios_setstate(ios, 2, 0);
        }
    }
    this
}

unsafe extern "win64" fn istream_tellg(this: *mut u8, out: *mut [i64; 3]) -> *mut [i64; 3] {
    let mut local_out = [-1i64, 0, 0];
    if !this.is_null() && !out.is_null() {
        let ios = stream_ios(this);
        if !ios.is_null() {
            let mystate: i32 = read_field(ios, std::mem::offset_of!(BasicIos, mystate));
            let sb: *mut BasicStreambuf = read_field(ios, std::mem::offset_of!(BasicIos, mystrbuf));
            if (mystate & 6) == 0 && !sb.is_null() {
                let vtable: *const Ptr = read_field(sb, std::mem::offset_of!(BasicStreambuf, vtable));
                if !vtable.is_null() {
                    let seekoff = std::ptr::read_unaligned(vtable.add(10)).0 as usize;
                    if seekoff != 0 {
                        let _ = crate::call_guest8(
                            seekoff,
                            sb as u64,
                            &mut local_out as *mut [i64; 3] as u64,
                            0,
                            1, // ios_base::cur
                            1, // ios_base::in
                            0,
                            0,
                            0,
                        );
                    }
                }
            }
        }
    }
    if !out.is_null() {
        std::ptr::write_unaligned(out, local_out);
    }
    out
}
unsafe extern "win64" fn smanip_setw_fn(ios: *mut BasicIos, width: i64) {
    if !ios.is_null() {
        write_field(ios, std::mem::offset_of!(BasicIos, wide), width);
    }
}

unsafe extern "win64" fn smanip_setw(out: *mut [usize; 2], val: i64) -> *mut [usize; 2] {
    ensure_vtables_registered();
    if !out.is_null() {
        std::ptr::write_unaligned(out, [smanip_setw_fn as *const () as usize, val as usize]);
    }
    out
}

unsafe extern "win64" fn smanip_setprecision_fn(ios: *mut BasicIos, prec: i64) {
    if !ios.is_null() {
        write_field(ios, std::mem::offset_of!(BasicIos, prec), prec);
    }
}

unsafe extern "win64" fn smanip_setprecision(out: *mut [usize; 2], val: i64) -> *mut [usize; 2] {
    ensure_vtables_registered();
    if !out.is_null() {
        std::ptr::write_unaligned(out, [smanip_setprecision_fn as *const () as usize, val as usize]);
    }
    out
}
unsafe extern "win64" fn time_put_put(
    _this: *const c_void,
    out_ret: *mut [usize; 2],
    out_it: *const [usize; 2],
) -> *mut [usize; 2] {
    if !out_ret.is_null() && !out_it.is_null() {
        std::ptr::write_unaligned(out_ret, std::ptr::read_unaligned(out_it));
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
    Export { name: "??0?$basic_ios@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z", ptr: Ptr(ios_ctor_sb as *const ()) },
    Export { name: "??0?$basic_ios@_WU?$char_traits@_W@std@@@std@@IEAA@XZ", ptr: Ptr(ios_w_ctor as *const ()) },
    Export { name: "??0?$basic_iostream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@@Z", ptr: Ptr(iostream_ctor as *const ()) },
    Export { name: "??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@QEAA@PEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@1@@Z", ptr: Ptr(iostream_w_ctor as *const ()) },
    Export { name: "??0?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@_N@Z", ptr: Ptr(istream_ctor as *const ()) },
    Export { name: "??0?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA@PEAV?$basic_streambuf@DU?$char_traits@D@std@@@1@_N@Z", ptr: Ptr(ostream_ctor as *const ()) },
    Export { name: "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@AEBV01@@Z", ptr: Ptr(streambuf_copy_ctor as *const ()) },
    Export { name: "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@XZ", ptr: Ptr(streambuf_ctor as *const ()) },
    Export { name: "??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAA@XZ", ptr: Ptr(streambuf_w_ctor as *const ()) },
    Export { name: "??0?$codecvt@_UDU_Mbstatet@@@std@@QEAA@_K@Z", ptr: Ptr(codecvt_u32_ctor as *const ()) },
    Export { name: "??0?$codecvt@_WDU_Mbstatet@@@std@@QEAA@_K@Z", ptr: Ptr(codecvt_w_ctor as *const ()) },
    Export { name: "??0_Locinfo@std@@QEAA@HPEBD@Z", ptr: Ptr(locinfo_ctor_cat as *const ()) },
    Export { name: "??0_Locinfo@std@@QEAA@PEBD@Z", ptr: Ptr(locinfo_ctor as *const ()) },
    Export { name: "??0_Lockit@std@@QEAA@H@Z", ptr: Ptr(lockit_ctor as *const ()) },
    Export { name: "??0facet@locale@std@@IEAA@_K@Z", ptr: Ptr(facet_ctor as *const ()) },
    Export { name: "??0task_continuation_context@Concurrency@@AEAA@XZ", ptr: Ptr(task_continuation_context_ctor as *const ()) },
    Export { name: "??1?$basic_ios@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(ios_dtor as *const ()) },
    Export { name: "??1?$basic_ios@_WU?$char_traits@_W@std@@@std@@UEAA@XZ", ptr: Ptr(ios_w_dtor as *const ()) },
    Export { name: "??1?$basic_iostream@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(iostream_dtor as *const ()) },
    Export { name: "??1?$basic_iostream@_WU?$char_traits@_W@std@@@std@@UEAA@XZ", ptr: Ptr(iostream_dtor as *const ()) },
    Export { name: "??1?$basic_istream@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(istream_dtor as *const ()) },
    Export { name: "??1?$basic_ostream@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(ostream_dtor as *const ()) },
    Export { name: "??1?$basic_streambuf@DU?$char_traits@D@std@@@std@@UEAA@XZ", ptr: Ptr(streambuf_dtor as *const ()) },
    Export { name: "??1?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@UEAA@XZ", ptr: Ptr(streambuf_w_dtor as *const ()) },
    Export { name: "??1?$codecvt@_UDU_Mbstatet@@@std@@MEAA@XZ", ptr: Ptr(facet_dtor as *const ()) },
    Export { name: "??1?$codecvt@_WDU_Mbstatet@@@std@@MEAA@XZ", ptr: Ptr(facet_dtor as *const ()) },
    Export { name: "??1_Locinfo@std@@QEAA@XZ", ptr: Ptr(locinfo_dtor as *const ()) },
    Export { name: "??1_Lockit@std@@QEAA@XZ", ptr: Ptr(lockit_dtor as *const ()) },
    Export { name: "??1facet@locale@std@@MEAA@XZ", ptr: Ptr(facet_dtor as *const ()) },
    Export { name: "??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAAEAV01@AEBV01@@Z", ptr: Ptr(streambuf_copy_assign as *const ()) },
    Export { name: "??4?$_Yarn@D@std@@QEAAAEAV01@PEBD@Z", ptr: Ptr(yarn_assign as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@F@Z", ptr: Ptr(ostream_shl_i16 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@G@Z", ptr: Ptr(ostream_shl_u16 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@H@Z", ptr: Ptr(ostream_shl_i32 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@I@Z", ptr: Ptr(ostream_shl_u32 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@J@Z", ptr: Ptr(ostream_shl_i32 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@K@Z", ptr: Ptr(ostream_shl_u32 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@M@Z", ptr: Ptr(crate::bridge_entry!(ostream_shl_f32)) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@N@Z", ptr: Ptr(crate::bridge_entry!(ostream_shl_f64)) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@P6AAEAV01@AEAV01@@Z@Z", ptr: Ptr(ostream_shl_manip as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@P6AAEAVios_base@1@AEAV21@@Z@Z", ptr: Ptr(ostream_shl_ios_base_manip as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@PEBX@Z", ptr: Ptr(ostream_shl_ptr as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_J@Z", ptr: Ptr(ostream_shl_i64 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_K@Z", ptr: Ptr(ostream_shl_u64 as *const ()) },
    Export { name: "??6?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV01@_N@Z", ptr: Ptr(ostream_shl_bool as *const ()) },
    Export { name: "??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@I@Z", ptr: Ptr(ostream_w_shl_u32 as *const ()) },
    Export { name: "??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@P6AAEAVios_base@1@AEAV21@@Z@Z", ptr: Ptr(ostream_shl_ios_base_manip as *const ()) },
    Export { name: "??Bid@locale@std@@QEAA_KXZ", ptr: Ptr(locale_id_operator_usize as *const ()) },
    Export { name: "??Bios_base@std@@QEBA_NXZ", ptr: Ptr(ios_base_bool as *const ()) },
    Export { name: "?GetCurrentThreadId@platform@details@Concurrency@@YAJXZ", ptr: Ptr(concurrency_get_current_thread_id as *const ()) },
    Export { name: "?ReportUnhandledError@_ExceptionHolder@details@Concurrency@@AEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_Addfac@_Locimp@locale@std@@AEAAXPEAVfacet@23@_K@Z", ptr: Ptr(locimp_addfac as *const ()) },
    Export { name: "?_CallInContext@_ContextCallback@details@Concurrency@@QEBAXV?$function@$$A6AXXZ@std@@_N@Z", ptr: Ptr(call_in_context as *const ()) },
    Export { name: "?_Capture@_ContextCallback@details@Concurrency@@AEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_Decref@facet@locale@std@@UEAAPEAV_Facet_base@3@XZ", ptr: Ptr(facet_decref as *const ()) },
    Export { name: "?_Execute_once@std@@YAHAEAUonce_flag@1@P6AHPEAX1PEAPEAX@Z1@Z", ptr: Ptr(execute_once as *const ()) },
    Export { name: "?_Fiopen@std@@YAPEAU_iobuf@@PEBDHH@Z", ptr: Ptr(fiopen_a as *const ()) },
    Export { name: "?_Fiopen@std@@YAPEAU_iobuf@@PEB_WHH@Z", ptr: Ptr(fiopen_w as *const ()) },
    Export { name: "?_Getcat@?$codecvt@DDU_Mbstatet@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z", ptr: Ptr(codecvt_d_getcat as *const ()) },
    Export { name: "?_Getcat@?$codecvt@_WDU_Mbstatet@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z", ptr: Ptr(codecvt_w_getcat as *const ()) },
    Export { name: "?_Getcat@?$ctype@D@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z", ptr: Ptr(ctype_getcat as *const ()) },
    Export { name: "?_Getcat@?$ctype@_W@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z", ptr: Ptr(ctype_w_getcat as *const ()) },
    Export { name: "?_Getcat@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z", ptr: Ptr(time_put_getcat as *const ()) },
    Export { name: "?_Getcoll@_Locinfo@std@@QEBA?AU_Collvec@@XZ", ptr: Ptr(locinfo_getcoll as *const ()) },
    Export { name: "?_Getcvt@_Locinfo@std@@QEBA?AU_Cvtvec@@XZ", ptr: Ptr(locinfo_getcvt as *const ()) },
    Export { name: "?_Getfalse@_Locinfo@std@@QEBAPEBDXZ", ptr: Ptr(locinfo_getfalse as *const ()) },
    Export { name: "?_Getgloballocale@locale@std@@CAPEAV_Locimp@12@XZ", ptr: Ptr(locale_getgloballocale as *const ()) },
    Export { name: "?_Getlconv@_Locinfo@std@@QEBAPEBUlconv@@XZ", ptr: Ptr(locinfo_getlconv as *const ()) },
    Export { name: "?_Getname@_Locinfo@std@@QEBAPEBDXZ", ptr: Ptr(locinfo_getname as *const ()) },
    Export { name: "?_Gettrue@_Locinfo@std@@QEBAPEBDXZ", ptr: Ptr(locinfo_gettrue as *const ()) },
    Export { name: "?_Gnavail@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBA_JXZ", ptr: Ptr(streambuf_gnavail as *const ()) },
    Export { name: "?_Gndec@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ", ptr: Ptr(streambuf_gndec as *const ()) },
    Export { name: "?_Gninc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ", ptr: Ptr(streambuf_gninc as *const ()) },
    Export { name: "?_Id_cnt@id@locale@std@@0HA", ptr: Ptr(&LOCALE_ID_CNT as *const AtomicU32 as *const ()) },
    Export { name: "?_Incref@facet@locale@std@@UEAAXXZ", ptr: Ptr(facet_incref as *const ()) },
    Export { name: "?_Init@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXPEAPEAD0PEAH001@Z", ptr: Ptr(streambuf_init_ptrs as *const ()) },
    Export { name: "?_Init@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXXZ", ptr: Ptr(streambuf_init_empty as *const ()) },
    Export { name: "?_Init@locale@std@@CAPEAV_Locimp@12@_N@Z", ptr: Ptr(locale_init as *const ()) },
    Export { name: "?_Ipfx@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA_N_N@Z", ptr: Ptr(istream_ipfx as *const ()) },
    Export { name: "?_IsCurrentOriginSTA@_ContextCallback@details@Concurrency@@CA_NXZ", ptr: Ptr(is_current_origin_sta as *const ()) },
    Export { name: "?_Locimp_Addfac@_Locimp@locale@std@@CAXPEAV123@PEAVfacet@23@_K@Z", ptr: Ptr(locimp_addfac as *const ()) },
    Export { name: "?_Lock@?$basic_streambuf@DU?$char_traits@D@std@@@std@@UEAAXXZ", ptr: Ptr(streambuf_lock as *const ()) },
    Export { name: "?_Lock@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@UEAAXXZ", ptr: Ptr(streambuf_w_lock as *const ()) },
    Export { name: "?_LogCancelTask@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogScheduleTask@_TaskEventLogger@details@Concurrency@@QEAAX_N@Z", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogTaskCompleted@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogTaskExecutionCompleted@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogWorkItemCompleted@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_LogWorkItemStarted@_TaskEventLogger@details@Concurrency@@QEAAXXZ", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?_Makeloc@_Locimp@locale@std@@CAPEAV123@AEBV_Locinfo@3@HPEAV123@PEBV23@@Z", ptr: Ptr(locimp_makeloc as *const ()) },
    Export { name: "?_New_Locimp@_Locimp@locale@std@@CAPEAV123@AEBV123@@Z", ptr: Ptr(locimp_new as *const ()) },
    Export { name: "?_New_Locimp@_Locimp@locale@std@@CAPEAV123@_N@Z", ptr: Ptr(locimp_new_bool as *const ()) },
    Export { name: "?_Osfx@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAXXZ", ptr: Ptr(ostream_osfx as *const ()) },
    Export { name: "?_Osfx@?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAXXZ", ptr: Ptr(ostream_w_osfx as *const ()) },
    Export { name: "?_Pnavail@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBA_JXZ", ptr: Ptr(streambuf_pnavail as *const ()) },
    Export { name: "?_Pninc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAPEADXZ", ptr: Ptr(streambuf_pninc as *const ()) },
    Export { name: "?_Pninc@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAAPEA_WXZ", ptr: Ptr(streambuf_w_pninc as *const ()) },
    Export { name: "?_Raise_handler@std@@3P6AXAEBVexception@stdext@@@ZEA", ptr: Ptr(&RAISE_HANDLER as *const AtomicUsize as *const ()) },
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
    Export { name: "?_Unlock@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@UEAAXXZ", ptr: Ptr(streambuf_w_unlock as *const ()) },
    Export { name: "?_W_Getdays@_Locinfo@std@@QEBAPEBGXZ", ptr: Ptr(locinfo_w_getdays as *const ()) },
    Export { name: "?_W_Getmonths@_Locinfo@std@@QEBAPEBGXZ", ptr: Ptr(locinfo_w_getmonths as *const ()) },
    Export { name: "?_Winerror_map@std@@YAHH@Z", ptr: Ptr(winerror_map as *const ()) },
    Export { name: "?_XGetLastError@std@@YAXXZ", ptr: Ptr(xget_last_error as *const ()) },
    Export { name: "?_Xbad_alloc@std@@YAXXZ", ptr: Ptr(xbad_alloc as *const ()) },
    Export { name: "?_Xbad_function_call@std@@YAXXZ", ptr: Ptr(xbad_function_call as *const ()) },
    Export { name: "?_Xinvalid_argument@std@@YAXPEBD@Z", ptr: Ptr(xinvalid_argument as *const ()) },
    Export { name: "?_Xlength_error@std@@YAXPEBD@Z", ptr: Ptr(xlength_error as *const ()) },
    Export { name: "?_Xout_of_range@std@@YAXPEBD@Z", ptr: Ptr(xout_of_range as *const ()) },
    Export { name: "?_Xoverflow_error@std@@YAXPEBD@Z", ptr: Ptr(xoverflow_error as *const ()) },
    Export { name: "?_Xregex_error@std@@YAXW4error_type@regex_constants@1@@Z", ptr: Ptr(xregex_error as *const ()) },
    Export { name: "?_Xruntime_error@std@@YAXPEBD@Z", ptr: Ptr(xruntime_error as *const ()) },
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
    Export { name: "?copyfmt@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAAEAV12@AEBV12@@Z", ptr: Ptr(ios_copyfmt as *const ()) },
    Export { name: "?cout@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A", ptr: Ptr(&COUT as *const StaticOstream as *const ()) },
    Export { name: "?eback@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_eback as *const ()) },
    Export { name: "?eback@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEBAPEA_WXZ", ptr: Ptr(streambuf_w_eback as *const ()) },
    Export { name: "?egptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_egptr as *const ()) },
    Export { name: "?egptr@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEBAPEA_WXZ", ptr: Ptr(streambuf_w_egptr as *const ()) },
    Export { name: "?epptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_epptr as *const ()) },
    Export { name: "?epptr@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEBAPEA_WXZ", ptr: Ptr(streambuf_w_epptr as *const ()) },
    Export { name: "?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAADD@Z", ptr: Ptr(ios_fill_set as *const ()) },
    Export { name: "?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADXZ", ptr: Ptr(ios_fill_get as *const ()) },
    Export { name: "?fill@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAA_W_W@Z", ptr: Ptr(ios_w_fill_set as *const ()) },
    Export { name: "?fill@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEBA_WXZ", ptr: Ptr(ios_w_fill_get as *const ()) },
    Export { name: "?flags@ios_base@std@@QEBAHXZ", ptr: Ptr(ios_base_flags as *const ()) },
    Export { name: "?flush@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@XZ", ptr: Ptr(ostream_flush as *const ()) },
    Export { name: "?flush@?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV12@XZ", ptr: Ptr(ostream_w_flush as *const ()) },
    Export { name: "?gbump@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXH@Z", ptr: Ptr(streambuf_gbump as *const ()) },
    Export { name: "?gbump@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAAXH@Z", ptr: Ptr(streambuf_w_gbump as *const ()) },
    Export { name: "?getloc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEBA?AVlocale@2@XZ", ptr: Ptr(streambuf_getloc as *const ()) },
    Export { name: "?getloc@ios_base@std@@QEBA?AVlocale@2@XZ", ptr: Ptr(streambuf_getloc as *const ()) },
    Export { name: "?good@ios_base@std@@QEBA_NXZ", ptr: Ptr(ios_good as *const ()) },
    Export { name: "?gptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_gptr as *const ()) },
    Export { name: "?gptr@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEBAPEA_WXZ", ptr: Ptr(streambuf_w_gptr as *const ()) },
    Export { name: "?id@?$codecvt@DDU_Mbstatet@@@std@@2V0locale@2@A", ptr: Ptr(&ID_CODECVT_D as *const LocaleId as *const ()) },
    Export { name: "?id@?$codecvt@_WDU_Mbstatet@@@std@@2V0locale@2@A", ptr: Ptr(&ID_CODECVT_W as *const LocaleId as *const ()) },
    Export { name: "?id@?$collate@D@std@@2V0locale@2@A", ptr: Ptr(&ID_COLLATE_D as *const LocaleId as *const ()) },
    Export { name: "?id@?$ctype@D@std@@2V0locale@2@A", ptr: Ptr(&ID_CTYPE_D as *const LocaleId as *const ()) },
    Export { name: "?id@?$ctype@_W@std@@2V0locale@2@A", ptr: Ptr(&ID_CTYPE_W as *const LocaleId as *const ()) },
    Export { name: "?id@?$numpunct@D@std@@2V0locale@2@A", ptr: Ptr(&ID_NUMPUNCT_D as *const LocaleId as *const ()) },
    Export { name: "?id@?$numpunct@_W@std@@2V0locale@2@A", ptr: Ptr(&ID_NUMPUNCT_W as *const LocaleId as *const ()) },
    Export { name: "?id@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A", ptr: Ptr(&ID_TIME_PUT_D as *const LocaleId as *const ()) },
    Export { name: "?imbue@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAXAEBVlocale@2@@Z", ptr: Ptr(streambuf_imbue as *const ()) },
    Export { name: "?imbue@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@MEAAXAEBVlocale@2@@Z", ptr: Ptr(streambuf_w_imbue as *const ()) },
    Export { name: "?in@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEAD3AEAPEAD@Z", ptr: Ptr(codecvt_d_in_out as *const ()) },
    Export { name: "?in@?$codecvt@_UDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEA_U3AEAPEA_U@Z", ptr: Ptr(codecvt_u32_in as *const ()) },
    Export { name: "?in@?$codecvt@_WDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEA_W3AEAPEA_W@Z", ptr: Ptr(codecvt_w_in as *const ()) },
    Export { name: "?init@?$basic_ios@DU?$char_traits@D@std@@@std@@IEAAXPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@_N@Z", ptr: Ptr(ios_init_export as *const ()) },
    Export { name: "?out@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEBD1AEAPEBDPEAD3AEAPEAD@Z", ptr: Ptr(codecvt_d_in_out as *const ()) },
    Export { name: "?out@?$codecvt@_UDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEB_U1AEAPEB_UPEAD3AEAPEAD@Z", ptr: Ptr(codecvt_u32_out as *const ()) },
    Export { name: "?out@?$codecvt@_WDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEB_W1AEAPEB_WPEAD3AEAPEAD@Z", ptr: Ptr(codecvt_w_out as *const ()) },
    Export { name: "?pbackfail@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAHH@Z", ptr: Ptr(streambuf_pbackfail as *const ()) },
    Export { name: "?pbase@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_pbase as *const ()) },
    Export { name: "?pbase@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEBAPEA_WXZ", ptr: Ptr(streambuf_w_pbase as *const ()) },
    Export { name: "?pbump@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXH@Z", ptr: Ptr(streambuf_pbump as *const ()) },
    Export { name: "?pptr@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEBAPEADXZ", ptr: Ptr(streambuf_pptr as *const ()) },
    Export { name: "?pptr@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEBAPEA_WXZ", ptr: Ptr(streambuf_w_pptr as *const ()) },
    Export { name: "?put@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@D@Z", ptr: Ptr(ostream_put as *const ()) },
    Export { name: "?put@?$time_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@QEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBUtm@@PEBD3@Z", ptr: Ptr(time_put_put as *const ()) },
    Export { name: "?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@PEAV32@@Z", ptr: Ptr(ios_rdbuf_set as *const ()) },
    Export { name: "?rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@XZ", ptr: Ptr(ios_rdbuf_get as *const ()) },
    Export { name: "?rdbuf@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEBAPEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@XZ", ptr: Ptr(ios_w_rdbuf_get as *const ()) },
    Export { name: "?read@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEAD_J@Z", ptr: Ptr(istream_read as *const ()) },
    Export { name: "?sbumpc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ", ptr: Ptr(streambuf_sbumpc as *const ()) },
    Export { name: "?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@V?$fpos@U_Mbstatet@@@2@@Z", ptr: Ptr(istream_seekg_pos as *const ()) },
    Export { name: "?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@_JH@Z", ptr: Ptr(istream_seekg as *const ()) },
    Export { name: "?seekoff@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA?AV?$fpos@U_Mbstatet@@@2@_JHH@Z", ptr: Ptr(streambuf_seekoff as *const ()) },
    Export { name: "?seekpos@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA?AV?$fpos@U_Mbstatet@@@2@V32@H@Z", ptr: Ptr(streambuf_seekpos as *const ()) },
    Export { name: "?setbuf@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAPEAV12@PEAD_J@Z", ptr: Ptr(streambuf_setbuf as *const ()) },
    Export { name: "?setbuf@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@MEAAPEAV12@PEA_W_J@Z", ptr: Ptr(streambuf_w_setbuf as *const ()) },
    Export { name: "?setf@ios_base@std@@QEAAHH@Z", ptr: Ptr(ios_base_setf1 as *const ()) },
    Export { name: "?setf@ios_base@std@@QEAAHHH@Z", ptr: Ptr(ios_base_setf2 as *const ()) },
    Export { name: "?setg@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXPEAD00@Z", ptr: Ptr(streambuf_setg as *const ()) },
    Export { name: "?setg@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAAXPEA_W00@Z", ptr: Ptr(streambuf_w_setg as *const ()) },
    Export { name: "?setp@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXPEAD00@Z", ptr: Ptr(streambuf_setp3 as *const ()) },
    Export { name: "?setp@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXPEAD0@Z", ptr: Ptr(streambuf_setp2 as *const ()) },
    Export { name: "?setp@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAAXPEA_W00@Z", ptr: Ptr(streambuf_w_setp3 as *const ()) },
    Export { name: "?setp@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAAXPEA_W0@Z", ptr: Ptr(streambuf_w_setp2 as *const ()) },
    Export { name: "?setprecision@std@@YA?AU?$_Smanip@_J@1@_J@Z", ptr: Ptr(smanip_setprecision as *const ()) },
    Export { name: "?setstate@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXH_N@Z", ptr: Ptr(ios_setstate as *const ()) },
    Export { name: "?setstate@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAXH_N@Z", ptr: Ptr(ios_w_setstate as *const ()) },
    Export { name: "?setw@std@@YA?AU?$_Smanip@_J@1@_J@Z", ptr: Ptr(smanip_setw as *const ()) },
    Export { name: "?sgetc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ", ptr: Ptr(streambuf_sgetc as *const ()) },
    Export { name: "?showmanyc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA_JXZ", ptr: Ptr(streambuf_showmanyc as *const ()) },
    Export { name: "?showmanyc@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@MEAA_JXZ", ptr: Ptr(streambuf_w_showmanyc as *const ()) },
    Export { name: "?snextc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHXZ", ptr: Ptr(streambuf_snextc as *const ()) },
    Export { name: "?sputc@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAAHD@Z", ptr: Ptr(streambuf_sputc as *const ()) },
    Export { name: "?sputc@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@QEAAG_W@Z", ptr: Ptr(streambuf_w_sputc as *const ()) },
    Export { name: "?sputn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@QEAA_JPEBD_J@Z", ptr: Ptr(streambuf_sputn as *const ()) },
    Export { name: "?sputn@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@QEAA_JPEB_W_J@Z", ptr: Ptr(streambuf_w_sputn as *const ()) },
    Export { name: "?swap@?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAAXAEAV12@@Z", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?swap@?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAXAEAV12@@Z", ptr: Ptr(noop_method as *const ()) },
    Export { name: "?sync@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAHXZ", ptr: Ptr(streambuf_sync as *const ()) },
    Export { name: "?sync@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@MEAAHXZ", ptr: Ptr(streambuf_w_sync as *const ()) },
    Export { name: "?tellg@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@XZ", ptr: Ptr(istream_tellg as *const ()) },
    Export { name: "?tellp@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@XZ", ptr: Ptr(ostream_tellp as *const ()) },
    Export { name: "?tie@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBAPEAV?$basic_ostream@DU?$char_traits@D@std@@@2@XZ", ptr: Ptr(ios_tie_get as *const ()) },
    Export { name: "?tie@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEBAPEAV?$basic_ostream@_WU?$char_traits@_W@std@@@2@XZ", ptr: Ptr(ios_w_tie_get as *const ()) },
    Export { name: "?tolower@?$ctype@D@std@@QEBADD@Z", ptr: Ptr(ctype_tolower_char as *const ()) },
    Export { name: "?tolower@?$ctype@D@std@@QEBAPEBDPEADPEBD@Z", ptr: Ptr(ctype_tolower_range as *const ()) },
    Export { name: "?toupper@?$ctype@_W@std@@QEBAPEB_WPEA_WPEB_W@Z", ptr: Ptr(ctype_w_toupper_range as *const ()) },
    Export { name: "?uflow@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAHXZ", ptr: Ptr(streambuf_uflow as *const ()) },
    Export { name: "?uflow@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@MEAAGXZ", ptr: Ptr(streambuf_w_uflow as *const ()) },
    Export { name: "?uncaught_exception@std@@YA_NXZ", ptr: Ptr(std_uncaught_exception as *const ()) },
    Export { name: "?uncaught_exceptions@std@@YAHXZ", ptr: Ptr(std_uncaught_exceptions as *const ()) },
    Export { name: "?underflow@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAAHXZ", ptr: Ptr(streambuf_underflow as *const ()) },
    Export { name: "?unshift@?$codecvt@DDU_Mbstatet@@@std@@QEBAHAEAU_Mbstatet@@PEAD1AEAPEAD@Z", ptr: Ptr(codecvt_unshift as *const ()) },
    Export { name: "?widen@?$basic_ios@DU?$char_traits@D@std@@@std@@QEBADD@Z", ptr: Ptr(ios_widen as *const ()) },
    Export { name: "?widen@?$ctype@_W@std@@QEBA_WD@Z", ptr: Ptr(ctype_w_widen_char as *const ()) },
    Export { name: "?width@ios_base@std@@QEAA_J_J@Z", ptr: Ptr(ios_base_width_set as *const ()) },
    Export { name: "?width@ios_base@std@@QEBA_JXZ", ptr: Ptr(ios_base_width_get as *const ()) },
    Export { name: "?write@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@PEBD_J@Z", ptr: Ptr(ostream_write as *const ()) },
    Export { name: "?xsgetn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA_JPEAD_J@Z", ptr: Ptr(streambuf_xsgetn as *const ()) },
    Export { name: "?xsgetn@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@MEAA_JPEA_W_J@Z", ptr: Ptr(streambuf_w_xsgetn as *const ()) },
    Export { name: "?xsputn@?$basic_streambuf@DU?$char_traits@D@std@@@std@@MEAA_JPEBD_J@Z", ptr: Ptr(streambuf_xsputn as *const ()) },
    Export { name: "?xsputn@?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@MEAA_JPEB_W_J@Z", ptr: Ptr(streambuf_w_xsputn as *const ()) },
    Export { name: "_FExp", ptr: Ptr(crate::bridge_entry!(fexp_impl)) },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msvcp140_runtime_locale_and_streams_roundtrip() {
        unsafe {
            assert!(_Thrd_hardware_concurrency() >= 1);

            // Spin lock acquires and releases cleanly across threads.
            _Lock_shared_ptr_spin_lock();
            _Unlock_shared_ptr_spin_lock();

            // _FExp scales exp(*px) * y * 2^eoff and returns MSVC FP class codes.
            let mut x = 0.0f32;
            assert_eq!(fexp_impl(&mut x, 1.5, 2), -1); // 1.5 * 4 = 6.0 (_FINITE)
            assert!((x - 6.0).abs() < 1e-5);
            let mut z = 1.0f32;
            assert_eq!(fexp_impl(&mut z, 0.0, 0), 0); // FP_ZERO
            assert_eq!(z, 0.0);
            let mut ov = 200.0f32;
            assert_eq!(fexp_impl(&mut ov, 1.0, 0), 1); // _INFCODE
            assert!(ov.is_infinite());

            // _Locinfo(int, const char*), _Getname, _New_Locimp(bool), _Makeloc, ctype<wchar_t>.
            let mut li = [0u8; 0x68];
            locinfo_ctor_cat(li.as_mut_ptr(), 0x3f, c"en-US".as_ptr());
            assert_eq!(CStr::from_ptr(locinfo_getname(li.as_ptr())).to_bytes(), b"en-US");
            let imp = locimp_new_bool(1);
            assert!(!imp.is_null());
            assert_eq!((*imp).transparent, 1);
            let imp2 = locimp_makeloc(li.as_ptr(), 0x3f, imp, std::ptr::null());
            assert_eq!(imp2, imp);
            // Unaligned caller-supplied Locimp passed to _Makeloc / _New_Locimp / facet helpers.
            let mut raw_imp = [0u8; std::mem::size_of::<Locimp>() + 8];
            let unaligned_imp = raw_imp.as_mut_ptr().add(1) as *mut Locimp;
            assert!(!unaligned_imp.is_aligned());
            facet_ctor(unaligned_imp as *mut Facet, 2);
            facet_incref(unaligned_imp as *mut Facet);
            assert_eq!(read_field::<_, u32>(unaligned_imp, std::mem::offset_of!(Locimp, refs)), 3);
            assert!(facet_decref(unaligned_imp as *mut Facet).is_null());
            assert_eq!(read_field::<_, u32>(unaligned_imp, std::mem::offset_of!(Locimp, refs)), 2);
            let imp_u = locimp_makeloc(li.as_ptr(), 0x3f, unaligned_imp, std::ptr::null());
            assert_eq!(imp_u, unaligned_imp);
            assert_eq!(read_field::<_, i32>(unaligned_imp, std::mem::offset_of!(Locimp, catmask)), 0x3f);
            let u_name: *mut c_char = read_field(unaligned_imp, std::mem::offset_of!(Locimp, name));
            assert!(!u_name.is_null());
            assert_eq!(CStr::from_ptr(u_name).to_bytes(), b"en-US");
            let cloned_from_unaligned = locimp_new(unaligned_imp);
            assert!(!cloned_from_unaligned.is_null());
            assert!(read_field::<_, usize>(cloned_from_unaligned, std::mem::offset_of!(Locimp, facetcount)) > 0);
            facet_dtor(unaligned_imp as *mut Facet);
            let mut wfac: *const Facet = std::ptr::null();
            assert_eq!(ctype_w_getcat(&mut wfac, std::ptr::null()), 2);
            assert!(!wfac.is_null());
            let cw = wfac as *const CtypeW;
            assert_eq!(ctype_w_widen_char(cw, b'z'), b'z' as u16);
            let mut wbuf: [u16; 4] = [b'a' as u16, b'B' as u16, b'z' as u16, b'9' as u16];
            let end = ctype_w_toupper_range(cw, wbuf.as_mut_ptr(), wbuf.as_ptr().add(4));
            assert_eq!(end, wbuf.as_ptr().add(4));
            assert_eq!(wbuf, [b'A' as u16, b'B' as u16, b'Z' as u16, b'9' as u16]);
            // Unaligned caller-supplied ctype<wchar_t> and ctype<char> destroying destructors (flags == 0).
            let mut raw_cw = [0u8; std::mem::size_of::<CtypeW>() + 8];
            let unaligned_cw = raw_cw.as_mut_ptr().add(1) as *mut CtypeW;
            assert!(!unaligned_cw.is_aligned());
            write_field(unaligned_cw, std::mem::offset_of!(CtypeW, vtable), CTYPE_W_VTABLE.as_ptr());
            assert_eq!(ctype_w_deleting_dtor(unaligned_cw, 0), unaligned_cw);
            assert_eq!(
                read_field::<_, *const Ptr>(unaligned_cw, std::mem::offset_of!(CtypeW, vtable)),
                FACET_VTABLE.as_ptr()
            );
            let mut raw_cc = [0u8; std::mem::size_of::<CtypeChar>() + 8];
            let unaligned_cc = raw_cc.as_mut_ptr().add(1) as *mut CtypeChar;
            assert!(!unaligned_cc.is_aligned());
            write_field(unaligned_cc, std::mem::offset_of!(CtypeChar, vtable), CTYPE_CHAR_VTABLE.as_ptr());
            assert_eq!(ctype_deleting_dtor(unaligned_cc, 0), unaligned_cc);
            assert_eq!(
                read_field::<_, *const Ptr>(unaligned_cc, std::mem::offset_of!(CtypeChar, vtable)),
                FACET_VTABLE.as_ptr()
            );
            locinfo_dtor(li.as_mut_ptr());

            // Narrow basic_streambuf + basic_ios + ios_base helpers.
            let mut sb: BasicStreambuf = std::mem::zeroed();
            streambuf_ctor(&mut sb);
            let mut gmem = *b"abcd";
            let mut pmem = [0u8; 16];
            streambuf_setg(&mut sb, gmem.as_mut_ptr(), gmem.as_mut_ptr(), gmem.as_mut_ptr().add(4));
            streambuf_setp2(&mut sb, pmem.as_mut_ptr(), pmem.as_mut_ptr().add(16));
            assert_eq!(streambuf_gnavail(&sb), 4);
            assert_eq!(streambuf_pnavail(&sb), 16);
            assert_eq!(*streambuf_gninc(&mut sb), b'a');
            assert_eq!(streambuf_gnavail(&sb), 3);
            assert_eq!(*streambuf_gndec(&mut sb), b'a');
            assert_eq!(streambuf_gnavail(&sb), 4);
            streambuf_gbump(&mut sb, 2);
            assert_eq!(*streambuf_gptr(&sb), b'c');
            streambuf_pbump(&mut sb, 1);
            assert_eq!(streambuf_pnavail(&sb), 15);

            // Copy ctor + copy assign preserve buffer pointers cleanly (including unaligned objects).
            let mut sb_copy: BasicStreambuf = std::mem::zeroed();
            streambuf_copy_ctor(&mut sb_copy, &sb);
            assert_eq!(streambuf_gnavail(&sb_copy), 2);
            assert_eq!(streambuf_pnavail(&sb_copy), 15);
            streambuf_dtor(&mut sb_copy);

            let mut raw_sb1 = vec![0xAAu8; std::mem::size_of::<BasicStreambuf>() + 8];
            let mut raw_sb2 = vec![0xBBu8; std::mem::size_of::<BasicStreambuf>() + 8];
            let mut raw_sb3 = vec![0xCCu8; std::mem::size_of::<BasicStreambuf>() + 8];
            let u_sb1 = raw_sb1.as_mut_ptr().add(1) as *mut BasicStreambuf;
            let u_sb2 = raw_sb2.as_mut_ptr().add(3) as *mut BasicStreambuf;
            let u_sb3 = raw_sb3.as_mut_ptr().add(5) as *mut BasicStreambuf;
            assert!(!u_sb1.is_aligned() && !u_sb2.is_aligned() && !u_sb3.is_aligned());
            streambuf_ctor(u_sb1);
            let mut ug = *b"wxyz";
            let mut up = [0u8; 8];
            streambuf_setg(u_sb1, ug.as_mut_ptr(), ug.as_mut_ptr(), ug.as_mut_ptr().add(4));
            streambuf_setp2(u_sb1, up.as_mut_ptr(), up.as_mut_ptr().add(8));
            assert_eq!(streambuf_gnavail(u_sb1), 4);
            assert_eq!(streambuf_pnavail(u_sb1), 8);
            assert_eq!(*streambuf_gninc(u_sb1), b'w');
            assert_eq!(streambuf_gnavail(u_sb1), 3);
            assert_eq!(*streambuf_gndec(u_sb1), b'w');
            streambuf_gbump(u_sb1, 1);
            streambuf_pbump(u_sb1, 2);
            assert_eq!(streambuf_gnavail(u_sb1), 3);
            assert_eq!(streambuf_pnavail(u_sb1), 6);
            streambuf_copy_ctor(u_sb2, u_sb1);
            assert_eq!(streambuf_gnavail(u_sb2), 3);
            assert_eq!(streambuf_pnavail(u_sb2), 6);
            assert_eq!(streambuf_gptr(u_sb2), ug.as_mut_ptr().add(1));
            streambuf_ctor(u_sb3);
            streambuf_copy_assign(u_sb3, u_sb2);
            assert_eq!(streambuf_gnavail(u_sb3), 3);
            assert_eq!(streambuf_pnavail(u_sb3), 6);
            assert_eq!(streambuf_sputc(u_sb3, b'!'), b'!' as i32);
            assert_eq!(up[2], b'!');
            assert_eq!(streambuf_sgetc(u_sb3), b'x' as i32);
            assert_eq!(streambuf_sbumpc(u_sb3), b'x' as i32);
            streambuf_dtor(u_sb3);
            streambuf_dtor(u_sb2);
            streambuf_dtor(u_sb1);

            // Wide basic_streambuf + basic_iostream + ostream formatting & manipulators.
            let mut wsb: BasicStreambufW = std::mem::zeroed();
            streambuf_w_ctor(&mut wsb);
            let mut wpmem = [0u16; 32];
            streambuf_w_setp2(&mut wsb, wpmem.as_mut_ptr(), wpmem.as_mut_ptr().add(32));
            let mut wiostr = [0u8; 0x20 + std::mem::size_of::<BasicIosW>()];
            iostream_w_ctor(wiostr.as_mut_ptr(), &mut wsb, 1);
            let wios = stream_ios_w(wiostr.as_mut_ptr());
            assert_eq!(ios_base_bool(wios as *const BasicIos), 1);
            assert_eq!(ios_w_fill_set(wios, b'0' as u16), b' ' as u16);
            assert_eq!(ios_w_fill_get(wios), b'0' as u16);
            ios_base_setf2(wios as *mut BasicIos, 0x0800 | 0x0004, 0x0E00 | 0x0004); // hex | uppercase
            assert_eq!(ios_base_width_set(wios as *mut BasicIos, 4), 0);
            assert_eq!(ios_base_width_get(wios as *const BasicIos), 4);
            ostream_w_shl_u32(wiostr.as_mut_ptr(), 0x2A);
            assert_eq!(ios_base_width_get(wios as *const BasicIos), 0);
            let written = (streambuf_w_pptr(&wsb) as usize - wpmem.as_ptr() as usize) / 2;
            let text = String::from_utf16_lossy(&wpmem[..written]);
            assert_eq!(text, "002A");
            assert_eq!(streambuf_w_sputc(&mut wsb, b'!' as u16), b'!' as u16);
            assert_eq!(wpmem[written], b'!' as u16);
            ios_w_dtor(wios);
            streambuf_w_dtor(&mut wsb);

            // Unaligned wide-stream objects (odd byte offset, !is_aligned()) must never UB/abort.
            let mut raw_wsb = vec![0xCCu8; std::mem::size_of::<BasicStreambufW>() + 8];
            let unaligned_wsb = raw_wsb.as_mut_ptr().add(1) as *mut BasicStreambufW;
            assert!(!unaligned_wsb.is_aligned());
            streambuf_w_ctor(unaligned_wsb);

            let mut raw_wbuf = [0u8; 66];
            let unaligned_g = raw_wbuf.as_mut_ptr().add(1) as *mut u16;
            for (i, u) in "ABCD".encode_utf16().enumerate() {
                std::ptr::write_unaligned(unaligned_g.add(i), u);
            }
            let unaligned_p = raw_wbuf.as_mut_ptr().add(17) as *mut u16;
            streambuf_w_setg(unaligned_wsb, unaligned_g, unaligned_g, unaligned_g.add(4));
            streambuf_w_setp2(unaligned_wsb, unaligned_p, unaligned_p.add(16));
            assert_eq!(streambuf_w_eback(unaligned_wsb), unaligned_g);
            assert_eq!(streambuf_w_gptr(unaligned_wsb), unaligned_g);
            assert_eq!(streambuf_w_egptr(unaligned_wsb), unaligned_g.add(4));
            assert_eq!(streambuf_w_pbase(unaligned_wsb), unaligned_p);
            assert_eq!(streambuf_w_pptr(unaligned_wsb), unaligned_p);
            assert_eq!(streambuf_w_epptr(unaligned_wsb), unaligned_p.add(16));

            let mut got_w = [0u16; 2];
            assert_eq!(streambuf_w_xsgetn(unaligned_wsb, got_w.as_mut_ptr(), 2), 2);
            assert_eq!(got_w, [b'A' as u16, b'B' as u16]);
            streambuf_w_gbump(unaligned_wsb, 1);
            assert_eq!(streambuf_w_gptr(unaligned_wsb), unaligned_g.add(3));
            assert_eq!(streambuf_w_sputc(unaligned_wsb, b'X' as u16), b'X' as u16);
            let pn = streambuf_w_pninc(unaligned_wsb);
            assert_eq!(pn, unaligned_p.add(1));
            std::ptr::write_unaligned(pn, b'Y' as u16);
            assert_eq!(std::ptr::read_unaligned(unaligned_p), b'X' as u16);
            assert_eq!(std::ptr::read_unaligned(unaligned_p.add(1)), b'Y' as u16);

            let mut raw_wiostr = vec![0xDDu8; 0x20 + std::mem::size_of::<BasicIosW>() + 8];
            let unaligned_wiostr = raw_wiostr.as_mut_ptr().add(3);
            assert!(!(unaligned_wiostr as *const BasicIosW).is_aligned());
            iostream_w_ctor(unaligned_wiostr, unaligned_wsb, 1);
            let unaligned_wios = stream_ios_w(unaligned_wiostr);
            assert!(!unaligned_wios.is_aligned());
            assert_eq!(ios_w_rdbuf_get(unaligned_wios), unaligned_wsb);
            assert!(ios_w_tie_get(unaligned_wios).is_null());
            assert_eq!(ios_base_bool(unaligned_wios as *const BasicIos), 1);
            assert_eq!(ios_w_fill_set(unaligned_wios, b'*' as u16), b' ' as u16);
            assert_eq!(ios_w_fill_get(unaligned_wios), b'*' as u16);
            ios_base_setf2(unaligned_wios as *mut BasicIos, 0x0800 | 0x0004, 0x0E00 | 0x0004);
            ios_base_width_set(unaligned_wios as *mut BasicIos, 3);
            ostream_w_shl_u32(unaligned_wiostr, 0xF);
            assert_eq!(std::ptr::read_unaligned(unaligned_p.add(2)), b'*' as u16);
            assert_eq!(std::ptr::read_unaligned(unaligned_p.add(3)), b'*' as u16);
            assert_eq!(std::ptr::read_unaligned(unaligned_p.add(4)), b'F' as u16);
            ostream_w_flush(unaligned_wiostr);
            ostream_w_osfx(unaligned_wiostr);
            ios_w_setstate(unaligned_wios, 1, 0);
            ios_w_dtor(unaligned_wios);
            streambuf_w_dtor(unaligned_wsb);

            let mut raw_ios_standalone = vec![0xEEu8; std::mem::size_of::<BasicIosW>() + 8];
            let unaligned_ios_standalone = raw_ios_standalone.as_mut_ptr().add(3) as *mut BasicIosW;
            assert!(!unaligned_ios_standalone.is_aligned());
            ios_w_ctor(unaligned_ios_standalone);
            ios_w_dtor(unaligned_ios_standalone);

            // Narrow basic_ios ctor(sb) + rdbuf + copyfmt + tellp.
            let mut ios1: BasicIos = std::mem::zeroed();
            ios_ctor_sb(&mut ios1, &mut sb);
            assert_eq!(ios_rdbuf_get(&ios1), &mut sb as *mut BasicStreambuf);
            assert_eq!(ios_fill_get(&ios1), b' ');
            assert_eq!(ios_base_bool(&ios1), 1);
            assert_eq!(ios_rdbuf_set(&mut ios1, std::ptr::null_mut()), &mut sb as *mut BasicStreambuf);
            assert_eq!(ios_base_bool(&ios1), 0);
            ios_dtor(&mut ios1);
            streambuf_dtor(&mut sb);

            // Verify every newly added Spider-Man 2 msvcp140 symbol resolves via lookup().
            for sym in [
                "_FExp",
                "_Lock_shared_ptr_spin_lock",
                "_Thrd_hardware_concurrency",
                "_Unlock_shared_ptr_spin_lock",
                "?_Raise_handler@std@@3P6AXAEBVexception@stdext@@@ZEA",
                "?_Xoverflow_error@std@@YAXPEBD@Z",
                "?_Xruntime_error@std@@YAXPEBD@Z",
                "??0_Locinfo@std@@QEAA@HPEBD@Z",
                "?_Getname@_Locinfo@std@@QEBAPEBDXZ",
                "?_New_Locimp@_Locimp@locale@std@@CAPEAV123@_N@Z",
                "?_Makeloc@_Locimp@locale@std@@CAPEAV123@AEBV_Locinfo@3@HPEAV123@PEBV23@@Z",
                "?id@?$ctype@_W@std@@2V0locale@2@A",
                "?_Getcat@?$ctype@_W@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z",
                "?toupper@?$ctype@_W@std@@QEBAPEB_WPEA_WPEB_W@Z",
                "?widen@?$ctype@_W@std@@QEBA_WD@Z",
                "??Bios_base@std@@QEBA_NXZ",
                "?flags@ios_base@std@@QEBAHXZ",
                "?setf@ios_base@std@@QEAAHH@Z",
                "?setf@ios_base@std@@QEAAHHH@Z",
                "?width@ios_base@std@@QEBA_JXZ",
                "?width@ios_base@std@@QEAA_J_J@Z",
                "?setw@std@@YA?AU?$_Smanip@_J@1@_J@Z",
                "?setprecision@std@@YA?AU?$_Smanip@_J@1@_J@Z",
                "?tellp@?$basic_ostream@DU?$char_traits@D@std@@@std@@QEAA?AV?$fpos@U_Mbstatet@@@2@XZ",
                "??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@QEAA@PEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@1@@Z",
                "??6?$basic_ostream@_WU?$char_traits@_W@std@@@std@@QEAAAEAV01@I@Z",
            ] {
                assert!(crate::lookup("msvcp140.dll", sym).is_some(), "missing {sym}");
            }
        }
    }

    #[test]
    fn smanip_setw_and_setprecision_format_wide_and_narrow_streams() {
        unsafe {
            type SmanipFn = unsafe extern "win64" fn(*mut BasicIos, i64);

            // Wide stream: format hex values with std::setw + fill('0'), matching
            // PCI\VEN_106B&DEV_C140&SUBSYS_106B0001&REV_01 style formatting.
            let mut wsb: BasicStreambufW = std::mem::zeroed();
            streambuf_w_ctor(&mut wsb);
            let mut wpmem = [0u16; 64];
            streambuf_w_setp2(&mut wsb, wpmem.as_mut_ptr(), wpmem.as_mut_ptr().add(64));
            let mut wiostr = [0u8; 0x20 + std::mem::size_of::<BasicIosW>()];
            iostream_w_ctor(wiostr.as_mut_ptr(), &mut wsb, 1);
            let wios = stream_ios_w(wiostr.as_mut_ptr());

            ios_base_setf2(wios as *mut BasicIos, 0x0800, 0x0E00); // std::hex
            ios_base_setf1(wios as *mut BasicIos, 0x0004); // std::uppercase
            ios_w_fill_set(wios, b'0' as u16);

            let mut m = [0usize; 2];
            assert_eq!(smanip_setw(&mut m, 2), &mut m as *mut [usize; 2]);
            let sw_fn: SmanipFn = std::mem::transmute(m[0]);
            sw_fn(wios as *mut BasicIos, m[1] as i64);
            assert_eq!(ios_base_width_get(wios as *const BasicIos), 2);
            ostream_w_shl_u32(wiostr.as_mut_ptr(), 0x1);
            assert_eq!(ios_base_width_get(wios as *const BasicIos), 0);

            assert_eq!(smanip_setw(&mut m, 8), &mut m as *mut [usize; 2]);
            let sw_fn: SmanipFn = std::mem::transmute(m[0]);
            sw_fn(wios as *mut BasicIos, m[1] as i64);
            ostream_w_shl_u32(wiostr.as_mut_ptr(), 0x00AB_C140);

            let written = (streambuf_w_pptr(&wsb) as usize - wpmem.as_ptr() as usize) / 2;
            assert_eq!(String::from_utf16_lossy(&wpmem[..written]), "0100ABC140");

            // setprecision updates ios_base::prec on both BasicIosW and BasicIos.
            assert_eq!(smanip_setprecision(&mut m, 12), &mut m as *mut [usize; 2]);
            let sp_fn: SmanipFn = std::mem::transmute(m[0]);
            sp_fn(wios as *mut BasicIos, m[1] as i64);
            assert_eq!(read_field::<BasicIosW, i64>(wios, std::mem::offset_of!(BasicIosW, prec)), 12);

            ios_w_dtor(wios);
            streambuf_w_dtor(&mut wsb);

            // Narrow stream: std::setw + hex + uppercase + u32/u64 formatting.
            let mut sb: BasicStreambuf = std::mem::zeroed();
            streambuf_ctor(&mut sb);
            let mut pmem = [0u8; 64];
            streambuf_setp2(&mut sb, pmem.as_mut_ptr(), pmem.as_mut_ptr().add(64));
            let mut ostr = [0u8; 0x10 + std::mem::size_of::<BasicIos>()];
            ostream_ctor(ostr.as_mut_ptr(), &mut sb, 0, 1);
            let ios = stream_ios(ostr.as_mut_ptr());
            ios_base_setf2(ios, 0x0800 | 0x0004, 0x0E00 | 0x0004);
            write_field(ios, std::mem::offset_of!(BasicIos, fillch), b'0');

            smanip_setw(&mut m, 4);
            let sw_fn: SmanipFn = std::mem::transmute(m[0]);
            sw_fn(ios, m[1] as i64);
            ostream_shl_u32(ostr.as_mut_ptr(), 0x1A);

            smanip_setw(&mut m, 6);
            sw_fn(ios, m[1] as i64);
            ostream_shl_u64(ostr.as_mut_ptr(), 0x2B);

            smanip_setprecision(&mut m, 9);
            let sp_fn: SmanipFn = std::mem::transmute(m[0]);
            sp_fn(ios, m[1] as i64);
            assert_eq!(read_field::<BasicIos, i64>(ios, std::mem::offset_of!(BasicIos, prec)), 9);

            let n = streambuf_pptr(&sb) as usize - pmem.as_ptr() as usize;
            assert_eq!(std::str::from_utf8(&pmem[..n]).unwrap(), "001A00002B");

            ios_dtor(ios);
            streambuf_dtor(&mut sb);
        }
    }

    #[test]
    fn fill_seekg_tellg_codecvt_w_getcat_and_execute_once() {
        use std::sync::atomic::AtomicI32;

        static SEEKOFF_CALLS: AtomicI32 = AtomicI32::new(0);
        static SEEKPOS_CALLS: AtomicI32 = AtomicI32::new(0);
        static LAST_SEEKOFF_ARGS: Mutex<(i64, i32, i32)> = Mutex::new((0, 0, 0));
        static LAST_SEEKPOS_ARGS: Mutex<([i64; 3], i32)> = Mutex::new(([0, 0, 0], 0));
        static CUR_POS: AtomicI32 = AtomicI32::new(12);

        unsafe extern "win64" fn custom_seekoff(
            _this: *mut BasicStreambuf,
            out: *mut [i64; 3],
            off: i64,
            way: i32,
            mode: i32,
        ) -> *mut [i64; 3] {
            SEEKOFF_CALLS.fetch_add(1, Ordering::SeqCst);
            *LAST_SEEKOFF_ARGS.lock().unwrap() = (off, way, mode);
            let next = match way {
                0 => off,                                               // beg
                1 => CUR_POS.load(Ordering::SeqCst) as i64 + off,       // cur
                2 => 100 + off,                                         // end
                _ => -1,
            };
            if next < 0 {
                std::ptr::write_unaligned(out, [-1, 0, 0]);
            } else {
                CUR_POS.store(next as i32, Ordering::SeqCst);
                std::ptr::write_unaligned(out, [next, 0, 0]);
            }
            out
        }

        unsafe extern "win64" fn custom_seekpos(
            _this: *mut BasicStreambuf,
            out: *mut [i64; 3],
            pos: *const [i64; 3],
            mode: i32,
        ) -> *mut [i64; 3] {
            SEEKPOS_CALLS.fetch_add(1, Ordering::SeqCst);
            let p = std::ptr::read_unaligned(pos);
            *LAST_SEEKPOS_ARGS.lock().unwrap() = (p, mode);
            let total = p[0].wrapping_add(p[1]);
            if total < 0 {
                std::ptr::write_unaligned(out, [-1, 0, 0]);
            } else {
                CUR_POS.store(total as i32, Ordering::SeqCst);
                std::ptr::write_unaligned(out, [total, 0, p[2]]);
            }
            out
        }

        unsafe {
            // 1. Export resolution for all 5 new msvcp140 symbols.
            for sym in [
                "?fill@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAADD@Z",
                "?seekg@?$basic_istream@DU?$char_traits@D@std@@@std@@QEAAAEAV12@V?$fpos@U_Mbstatet@@@2@@Z",
                "?_Getcat@?$codecvt@_WDU_Mbstatet@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z",
                "?_XGetLastError@std@@YAXXZ",
                "?_Execute_once@std@@YAHAEAUonce_flag@1@P6AHPEAX1PEAPEAX@Z1@Z",
            ] {
                assert!(crate::lookup("msvcp140.dll", sym).is_some(), "missing {sym}");
            }

            // 2. basic_ios<char>::fill(char) on unaligned BasicIos + null safety.
            assert_eq!(ios_fill_set(std::ptr::null_mut(), b'*'), b' ');
            let mut raw_ios = vec![0u8; std::mem::size_of::<BasicIos>() + 8];
            let u_ios = raw_ios.as_mut_ptr().add(3) as *mut BasicIos;
            assert!(!u_ios.is_aligned());
            ios_ctor(u_ios);
            ios_init(u_ios, std::ptr::null_mut());
            assert_eq!(ios_fill_get(u_ios), b' ');
            assert_eq!(ios_fill_set(u_ios, b'0'), b' ');
            assert_eq!(ios_fill_get(u_ios), b'0');
            assert_eq!(ios_fill_set(u_ios, b'_'), b'0');
            assert_eq!(ios_fill_get(u_ios), b'_');
            ios_dtor(u_ios);

            // 3. codecvt<wchar_t, char, _Mbstatet>::_Getcat + unaligned ctors.
            let mut raw_ppf = [0u8; std::mem::size_of::<*const Facet>() + 8];
            let u_ppf = raw_ppf.as_mut_ptr().add(3) as *mut *const Facet;
            assert!(!u_ppf.is_aligned());
            std::ptr::write_unaligned(u_ppf, std::ptr::null());
            assert_eq!(codecvt_w_getcat(u_ppf, std::ptr::null()), 2);
            let fac = std::ptr::read_unaligned(u_ppf);
            assert!(!fac.is_null());
            let cw = fac as *mut CodecvtW;
            assert_eq!(read_field::<_, *const Ptr>(cw, std::mem::offset_of!(CodecvtW, vtable)), CODECVT_W_VTABLE.as_ptr());
            assert_eq!(read_field::<_, u32>(cw, std::mem::offset_of!(CodecvtW, refs)), 0);
            // Calling _Getcat again when *ppf is non-null must not overwrite it.
            assert_eq!(codecvt_w_getcat(u_ppf, std::ptr::null()), 2);
            assert_eq!(std::ptr::read_unaligned(u_ppf), fac);
            // Verify codecvt_w_in works on the allocated facet.
            let src = b"Hi";
            let mut fnxt: *const u8 = std::ptr::null();
            let mut dst = [0u16; 2];
            let mut tnxt: *mut u16 = std::ptr::null_mut();
            let mut st = 0u64;
            assert_eq!(
                codecvt_w_in(cw, &mut st, src.as_ptr(), src.as_ptr().add(2), &mut fnxt, dst.as_mut_ptr(), dst.as_mut_ptr().add(2), &mut tnxt),
                0
            );
            assert_eq!(dst, [b'H' as u16, b'i' as u16]);
            codecvt_deleting_dtor(fac as *mut Facet, 1);

            let mut raw_cw = vec![0u8; std::mem::size_of::<CodecvtW>() + 8];
            let u_cw = raw_cw.as_mut_ptr().add(1) as *mut CodecvtW;
            assert!(!u_cw.is_aligned());
            codecvt_w_ctor(u_cw, 3);
            assert_eq!(read_field::<_, u32>(u_cw, std::mem::offset_of!(CodecvtW, refs)), 3);

            let mut raw_cu32 = vec![0u8; std::mem::size_of::<CodecvtU32>() + 8];
            let u_cu32 = raw_cu32.as_mut_ptr().add(1) as *mut CodecvtU32;
            assert!(!u_cu32.is_aligned());
            codecvt_u32_ctor(u_cu32, 2);
            assert_eq!(read_field::<_, u32>(u_cu32, std::mem::offset_of!(CodecvtU32, maxcode)), 0x10ffff);

            // 4. basic_istream<char>::seekg(fpos), seekg(off, way), and tellg() on unaligned objects.
            SEEKOFF_CALLS.store(0, Ordering::SeqCst);
            SEEKPOS_CALLS.store(0, Ordering::SeqCst);
            CUR_POS.store(12, Ordering::SeqCst);

            let mut custom_vt = BASIC_STREAMBUF_VTABLE;
            custom_vt[10] = Ptr(custom_seekoff as *const ());
            custom_vt[11] = Ptr(custom_seekpos as *const ());

            let mut raw_sb = vec![0u8; std::mem::size_of::<BasicStreambuf>() + 8];
            let u_sb = raw_sb.as_mut_ptr().add(3) as *mut BasicStreambuf;
            assert!(!u_sb.is_aligned());
            streambuf_ctor(u_sb);
            write_field(u_sb, std::mem::offset_of!(BasicStreambuf, vtable), custom_vt.as_ptr());

            let mut raw_istr = vec![0u8; 0x18 + std::mem::size_of::<BasicIos>() + 8];
            let u_istr = raw_istr.as_mut_ptr().add(5);
            istream_ctor(u_istr, u_sb, 0, 1);
            let istr_ios = stream_ios(u_istr);
            assert!(!istr_ios.is_aligned());

            // tellg dispatches vtable[10] with (0, cur=1, in=1) into unaligned fpos output.
            let mut raw_pos = [0u8; 24 + 8];
            let u_pos = raw_pos.as_mut_ptr().add(3) as *mut [i64; 3];
            assert!(!u_pos.is_aligned());
            assert_eq!(istream_tellg(u_istr, u_pos), u_pos);
            assert_eq!(std::ptr::read_unaligned(u_pos), [12, 0, 0]);
            assert_eq!(*LAST_SEEKOFF_ARGS.lock().unwrap(), (0, 1, 1));

            // seekg(fpos) clears eofbit first and invokes vtable[11] with mode=in(1).
            ios_setstate(istr_ios, 1, 0); // eofbit
            assert_eq!(ios_good(istr_ios), 0);
            std::ptr::write_unaligned(u_pos, [42, 0, 7]);
            assert_eq!(istream_seekg_pos(u_istr, u_pos), u_istr);
            assert_eq!(ios_good(istr_ios), 1);
            assert_eq!(*LAST_SEEKPOS_ARGS.lock().unwrap(), ([42, 0, 7], 1));
            assert_eq!(CUR_POS.load(Ordering::SeqCst), 42);

            // seekg(off, way) clears eofbit and invokes vtable[10] with (off, way, in=1).
            ios_setstate(istr_ios, 1, 0);
            assert_eq!(istream_seekg(u_istr, -10, 2 /* end */), u_istr);
            assert_eq!(ios_good(istr_ios), 1);
            assert_eq!(*LAST_SEEKOFF_ARGS.lock().unwrap(), (-10, 2, 1));
            assert_eq!(CUR_POS.load(Ordering::SeqCst), 90);

            // seekg(fpos(-1)) sets failbit (2); subsequent tellg returns [-1, 0, 0] and
            // subsequent seekg(pos) does not invoke seekpos while failbit is set.
            std::ptr::write_unaligned(u_pos, [-1, 0, 0]);
            istream_seekg_pos(u_istr, u_pos);
            assert_eq!(read_field::<_, i32>(istr_ios, std::mem::offset_of!(BasicIos, mystate)) & 2, 2);
            let prev_seekpos = SEEKPOS_CALLS.load(Ordering::SeqCst);
            let prev_seekoff = SEEKOFF_CALLS.load(Ordering::SeqCst);
            std::ptr::write_unaligned(u_pos, [5, 0, 0]);
            istream_seekg_pos(u_istr, u_pos);
            assert_eq!(SEEKPOS_CALLS.load(Ordering::SeqCst), prev_seekpos);
            istream_tellg(u_istr, u_pos);
            assert_eq!(std::ptr::read_unaligned(u_pos), [-1, 0, 0]);
            assert_eq!(SEEKOFF_CALLS.load(Ordering::SeqCst), prev_seekoff);

            // Clear failbit and verify null pos sets failbit.
            ios_clear(istr_ios, 0, 0);
            istream_seekg_pos(u_istr, std::ptr::null());
            assert_eq!(read_field::<_, i32>(istr_ios, std::mem::offset_of!(BasicIos, mystate)) & 2, 2);

            ios_dtor(istr_ios);
            streambuf_dtor(u_sb);

            // 5. std::_Execute_once: null validation, retry on failure, single execution on success, unaligned flag.
            unsafe extern "win64" fn once_cb(
                _once: *mut c_void,
                param: *mut c_void,
                _ctx: *mut *mut c_void,
            ) -> i32 {
                let state = &*(param as *const AtomicI32);
                let prev = state.fetch_add(1, Ordering::SeqCst);
                if prev == 0 { 0 } else { 1 } // Fail first attempt, succeed second.
            }

            crate::teb::set_last_error(0);
            assert_eq!(execute_once(std::ptr::null_mut(), Some(once_cb), std::ptr::null_mut()), 0);
            assert_eq!(crate::teb::last_error(), crate::types::err::ERROR_INVALID_PARAMETER);

            let mut raw_flag = [0u8; 16];
            let u_flag = raw_flag.as_mut_ptr().add(3) as *mut c_void;
            assert!(!(u_flag as *const usize).is_aligned());
            crate::teb::set_last_error(0);
            assert_eq!(execute_once(u_flag, None, std::ptr::null_mut()), 0);
            assert_eq!(crate::teb::last_error(), crate::types::err::ERROR_INVALID_PARAMETER);

            let calls = AtomicI32::new(0);
            let param = (&calls as *const AtomicI32) as *mut c_void;
            // First call returns 0 (failed init), leaving flag retryable.
            assert_eq!(execute_once(u_flag, Some(once_cb), param), 0);
            assert_eq!(calls.load(Ordering::SeqCst), 1);
            // Second call succeeds and marks once_flag complete.
            assert_eq!(execute_once(u_flag, Some(once_cb), param), 1);
            assert_eq!(calls.load(Ordering::SeqCst), 2);
            // Subsequent calls return 1 without re-running callback.
            assert_eq!(execute_once(u_flag, Some(once_cb), param), 1);
            assert_eq!(execute_once(u_flag, Some(once_cb), param), 1);
            assert_eq!(calls.load(Ordering::SeqCst), 2);
        }
    }

    #[test]
    fn xget_last_error_system_error_metadata_and_lifecycle() {
        unsafe {
            let slab = system_error_throw_slab();
            let base = slab as *const SystemErrorThrowSlab as usize;
            let ti = std::ptr::addr_of!(slab.ti_heap) as *const u8;
            let attrs = std::ptr::read_unaligned(ti as *const u32);
            let unwind_rva = std::ptr::read_unaligned(ti.add(4) as *const i32);
            let cta_rva = std::ptr::read_unaligned(ti.add(12) as *const i32);
            assert_eq!(attrs, 0);
            assert_ne!(unwind_rva, 0);
            assert_eq!(
                (base as isize + unwind_rva as isize) as usize,
                system_error_unwind_dtor as *const () as usize
            );
            assert!(cta_rva > 0);

            let cta = (base as isize + cta_rva as isize) as *const i32;
            assert_eq!(std::ptr::read_unaligned(cta), 3);

            let expected: [(&str, i32, usize); 3] = [
                (
                    ".?AVsystem_error@std@@",
                    0x28,
                    system_error_copy_ctor as *const () as usize,
                ),
                (
                    ".?AVruntime_error@std@@",
                    0x18,
                    runtime_error_copy_ctor as *const () as usize,
                ),
                (
                    ".?AVexception@std@@",
                    0x18,
                    exception_copy_ctor as *const () as usize,
                ),
            ];
            let mut copy_fns = [0usize; 3];
            for (i, &(want_name, want_size, want_copy)) in expected.iter().enumerate() {
                let ct_rva = std::ptr::read_unaligned(cta.add(1 + i));
                let ct = (base as isize + ct_rva as isize) as *const u8;
                let props = std::ptr::read_unaligned(ct as *const u32);
                let td_rva = std::ptr::read_unaligned(ct.add(4) as *const i32);
                let pmd = [
                    std::ptr::read_unaligned(ct.add(8) as *const i32),
                    std::ptr::read_unaligned(ct.add(12) as *const i32),
                    std::ptr::read_unaligned(ct.add(16) as *const i32),
                ];
                let sz = std::ptr::read_unaligned(ct.add(20) as *const i32);
                let copy_rva = std::ptr::read_unaligned(ct.add(24) as *const i32);
                assert_eq!(props, 0);
                assert_eq!(pmd, [0, -1, 0]);
                assert_eq!(sz, want_size);
                assert_ne!(copy_rva, 0);
                let copy_va = (base as isize + copy_rva as isize) as usize;
                assert_eq!(copy_va, want_copy);
                copy_fns[i] = copy_va;

                let td = (base as isize + td_rva as isize) as *const u8;
                let got_name = CStr::from_ptr(td.add(16) as *const c_char).to_str().unwrap();
                assert_eq!(got_name, want_name);
            }

            // Construct a heap-allocated StdSystemError for ERROR_ACCESS_DENIED (5 -> EACCES 13).
            let obj = crate::ucrt::heap::calloc(1, std::mem::size_of::<StdSystemError>())
                as *mut StdSystemError;
            assert!(!obj.is_null());
            init_system_error(obj, 5);
            assert_eq!(
                CStr::from_ptr(exception_what(obj.cast())).to_str().unwrap(),
                "permission denied"
            );
            let code: StdErrorCode = read_field(obj, std::mem::offset_of!(StdSystemError, code));
            assert_eq!(code.val, 5);
            assert_eq!(code.cat, &SYSTEM_CATEGORY as *const StdErrorCategory);
            assert_eq!(CStr::from_ptr(system_category_name(code.cat)).to_str().unwrap(), "system");

            // error_category virtuals: default_error_condition, equivalent, message.
            let mut cond = StdErrorCode { val: 0, _pad: 0, cat: std::ptr::null() };
            system_category_default_error_condition(code.cat, &mut cond, 5);
            assert_eq!(cond.val, 13); // EACCES
            assert_eq!(cond.cat, &GENERIC_CATEGORY as *const StdErrorCategory);
            assert_eq!(error_category_equiv_code_int(code.cat, &code, 5), 1);
            assert_eq!(error_category_equiv_int_cond(code.cat, 5, &cond), 1);

            let mut msg_str = [0u8; 32];
            system_category_message(code.cat, &mut msg_str, 5);
            let msg_len = u64::from_le_bytes(msg_str[16..24].try_into().unwrap()) as usize;
            let msg_cap = u64::from_le_bytes(msg_str[24..32].try_into().unwrap()) as usize;
            assert_eq!(msg_len, "permission denied".len());
            assert!(msg_cap >= 16);
            let msg_ptr = u64::from_le_bytes(msg_str[0..8].try_into().unwrap()) as *mut c_void;
            assert_eq!(CStr::from_ptr(msg_ptr.cast()).to_str().unwrap(), "permission denied");
            crate::ucrt::heap::free(msg_ptr);

            // Unmapped Win32 error formats fallback string and preserves system category.
            let mut unmapped_cond = StdErrorCode { val: 0, _pad: 0, cat: std::ptr::null() };
            system_category_default_error_condition(code.cat, &mut unmapped_cond, 99_999);
            assert_eq!(unmapped_cond.val, 99_999);
            assert_eq!(unmapped_cond.cat, &SYSTEM_CATEGORY as *const StdErrorCategory);
            assert_eq!(system_error_message(99_999), "system error 99999");

            // Copy-construct into unaligned catch buffers for all 3 catchable types via ThrowInfo copy_fn RVAs.
            let mut raw_se = [0u8; std::mem::size_of::<StdSystemError>() + 8];
            let u_se = raw_se.as_mut_ptr().add(3) as *mut StdSystemError;
            assert!(!u_se.is_aligned());
            let _ = crate::call_guest8(copy_fns[0], u_se as u64, obj as u64, 0, 0, 0, 0, 0, 0);
            assert_eq!(
                CStr::from_ptr(exception_what(u_se.cast())).to_str().unwrap(),
                "permission denied"
            );
            assert_eq!(
                read_field::<_, StdErrorCode>(u_se, std::mem::offset_of!(StdSystemError, code)).val,
                5
            );
            exception_deleting_dtor(u_se.cast(), 0);

            let mut raw_re = [0u8; std::mem::size_of::<StdException>() + 8];
            let u_re = raw_re.as_mut_ptr().add(3) as *mut StdException;
            let _ = crate::call_guest8(copy_fns[1], u_re as u64, obj as u64, 0, 0, 0, 0, 0, 0);
            assert_eq!(
                CStr::from_ptr(exception_what(u_re)).to_str().unwrap(),
                "permission denied"
            );
            exception_deleting_dtor(u_re, 0);

            let mut raw_ex = [0u8; std::mem::size_of::<StdException>() + 8];
            let u_ex = raw_ex.as_mut_ptr().add(1) as *mut StdException;
            let _ = crate::call_guest8(copy_fns[2], u_ex as u64, obj as u64, 0, 0, 0, 0, 0, 0);
            assert_eq!(
                CStr::from_ptr(exception_what(u_ex)).to_str().unwrap(),
                "permission denied"
            );
            exception_deleting_dtor(u_ex, 0);

            // Destroy the original thrown object through vcruntime140::destruct_exception_object.
            let rec = crate::unwind::ExceptionRecord {
                code: crate::unwind::CXX_EXCEPTION,
                flags: crate::unwind::EXCEPTION_NONCONTINUABLE,
                record: std::ptr::null_mut(),
                address: std::ptr::null_mut(),
                number_parameters: 4,
                _pad: 0,
                information: [0x1993_0520, obj as u64, ti as u64, base as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            };
            crate::misc::vcruntime140::destruct_exception_object(&rec);
        }
    }
}
