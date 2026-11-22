//! msvcrt.dll — only imported by the mingw-built zlib1.dll. Functions with
//! the same ABI fall back to ucrtbase (see `crate::lookup`); this file has
//! the msvcrt-only entry points and the mem*/str* functions (which the UCRT
//! itself leaves to vcruntime140).

use crate::ucrt::stdio::FILE;
use std::ffi::c_void;

pub static EXPORTS: &[crate::Export] = include!(concat!(env!("OUT_DIR"), "/msvcrt_exports.rs"));

/// `FILE* __iob_func(void)`: the stdin/stdout/stderr array (48-byte stride).
pub unsafe extern "win64" fn __iob_func() -> *mut FILE {
    crate::ucrt::stdio::iob_base()
}

type OnExit = unsafe extern "win64" fn() -> i32;

static ONEXIT: std::sync::Mutex<Vec<usize>> = std::sync::Mutex::new(Vec::new());

/// `_onexit`: register with the process-wide exit table (run in LIFO order
/// by the UCRT exit path through `atexit`).
pub unsafe extern "win64" fn _onexit(f: OnExit) -> OnExit {
    ONEXIT.lock().unwrap().push(f as usize);
    crate::ucrt::runtime::atexit(f as usize);
    f
}

/// `__dllonexit(func, &begin, &end)`: mingw DLLs keep their own table; the
/// CRT grows it. We register with the process exit table instead (the DLL is
/// never unloaded), leaving the DLL's table untouched.
pub unsafe extern "win64" fn __dllonexit(f: OnExit, _begin: *mut *mut usize, _end: *mut *mut usize) -> OnExit {
    _onexit(f)
}

static LOCKS: [crate::ucrt::stdio::RecLock; 36] = [const { crate::ucrt::stdio::RecLock::new() }; 36];

pub unsafe extern "win64" fn _lock(n: i32) {
    if let Some(l) = LOCKS.get(n as usize) {
        l.acquire();
    }
}
pub unsafe extern "win64" fn _unlock(n: i32) {
    if let Some(l) = LOCKS.get(n as usize) {
        l.release();
    }
}

pub unsafe extern "win64" fn _amsg_exit(code: i32) {
    log::error!("msvcrt!_amsg_exit({code}): runtime error R60{code:02}");
    crate::process::exit_process(255)
}

/// msvcrt `_vsnprintf`: no NUL on truncation, returns -1.
pub unsafe extern "win64" fn _vsnprintf(buf: *mut u8, n: usize, fmt: *const u8, va: *const u8) -> i32 {
    use crate::ucrt::printf::*;
    __stdio_common_vsprintf(
        LEGACY_VSPRINTF_NULL_TERMINATION | LEGACY_MSVCRT_COMPATIBILITY | LEGACY_THREE_DIGIT_EXPONENTS,
        buf,
        n,
        fmt,
        std::ptr::null_mut(),
        va,
    )
}

pub unsafe extern "win64" fn vfprintf(fp: *mut c_void, fmt: *const u8, va: *const u8) -> i32 {
    use crate::ucrt::printf::*;
    vfprintf_impl(LEGACY_MSVCRT_COMPATIBILITY | LEGACY_THREE_DIGIT_EXPONENTS, fp, fmt, va)
}

#[inline(never)]
pub unsafe extern "win64" fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void {
    std::ptr::copy(s as *const u8, d as *mut u8, n);
    d
}
#[inline(never)]
pub unsafe extern "win64" fn memmove(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void {
    std::ptr::copy(s as *const u8, d as *mut u8, n);
    d
}
#[inline(never)]
pub unsafe extern "win64" fn memset(d: *mut c_void, c: i32, n: usize) -> *mut c_void {
    std::ptr::write_bytes(d as *mut u8, c as u8, n);
    d
}
#[inline(never)]
pub unsafe extern "win64" fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    libc::memcmp(a as *const libc::c_void, b as *const libc::c_void, n)
}
#[inline(never)]
pub unsafe extern "win64" fn memchr(p: *const c_void, c: i32, n: usize) -> *mut c_void {
    let s = std::slice::from_raw_parts(p as *const u8, n);
    match s.iter().position(|&b| b == c as u8) {
        Some(i) => (p as *mut u8).add(i) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}
