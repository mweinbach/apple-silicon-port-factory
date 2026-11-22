//! CRT errno (Windows numbering) and strerror.

use std::cell::Cell;

pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const ESRCH: i32 = 3;
pub const EINTR: i32 = 4;
pub const EIO: i32 = 5;
pub const ENXIO: i32 = 6;
pub const E2BIG: i32 = 7;
pub const ENOEXEC: i32 = 8;
pub const EBADF: i32 = 9;
pub const ECHILD: i32 = 10;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EFAULT: i32 = 14;
pub const EBUSY: i32 = 16;
pub const EEXIST: i32 = 17;
pub const EXDEV: i32 = 18;
pub const ENODEV: i32 = 19;
pub const ENOTDIR: i32 = 20;
pub const EISDIR: i32 = 21;
pub const EINVAL: i32 = 22;
pub const ENFILE: i32 = 23;
pub const EMFILE: i32 = 24;
pub const ENOTTY: i32 = 25;
pub const EFBIG: i32 = 27;
pub const ENOSPC: i32 = 28;
pub const ESPIPE: i32 = 29;
pub const EROFS: i32 = 30;
pub const EMLINK: i32 = 31;
pub const EPIPE: i32 = 32;
pub const EDOM: i32 = 33;
pub const ERANGE: i32 = 34;
pub const EDEADLK: i32 = 36;
pub const ENAMETOOLONG: i32 = 38;
pub const ENOLCK: i32 = 39;
pub const ENOSYS: i32 = 40;
pub const ENOTEMPTY: i32 = 41;
pub const EILSEQ: i32 = 42;
pub const STRUNCATE: i32 = 80;

thread_local! {
    static ERRNO: Cell<i32> = const { Cell::new(0) };
    static DOSERRNO: Cell<u32> = const { Cell::new(0) };
}

pub fn set(e: i32) {
    ERRNO.with(|c| c.set(e));
}
pub fn get() -> i32 {
    ERRNO.with(|c| c.get())
}

/// Map a host (macOS) errno to the Windows CRT value.
pub fn from_host(e: i32) -> i32 {
    match e {
        libc::EPERM => EPERM,
        libc::ENOENT => ENOENT,
        libc::ESRCH => ESRCH,
        libc::EINTR => EINTR,
        libc::EIO => EIO,
        libc::ENXIO => ENXIO,
        libc::E2BIG => E2BIG,
        libc::ENOEXEC => ENOEXEC,
        libc::EBADF => EBADF,
        libc::ECHILD => ECHILD,
        libc::EAGAIN => EAGAIN,
        libc::ENOMEM => ENOMEM,
        libc::EACCES => EACCES,
        libc::EFAULT => EFAULT,
        libc::EBUSY => EBUSY,
        libc::EEXIST => EEXIST,
        libc::EXDEV => EXDEV,
        libc::ENODEV => ENODEV,
        libc::ENOTDIR => ENOTDIR,
        libc::EISDIR => EISDIR,
        libc::EINVAL => EINVAL,
        libc::ENFILE => ENFILE,
        libc::EMFILE => EMFILE,
        libc::ENOTTY => ENOTTY,
        libc::EFBIG => EFBIG,
        libc::ENOSPC => ENOSPC,
        libc::ESPIPE => ESPIPE,
        libc::EROFS => EROFS,
        libc::EMLINK => EMLINK,
        libc::EPIPE => EPIPE,
        libc::EDOM => EDOM,
        libc::ERANGE => ERANGE,
        libc::EDEADLK => EDEADLK,
        libc::ENAMETOOLONG => ENAMETOOLONG,
        libc::ENOLCK => ENOLCK,
        libc::ENOSYS => ENOSYS,
        libc::ENOTEMPTY => ENOTEMPTY,
        libc::EILSEQ => EILSEQ,
        _ => EINVAL,
    }
}

/// Set errno from the current host errno; returns the Windows value.
pub fn set_from_host() -> i32 {
    let e = from_host(crate::types::last_errno());
    set(e);
    e
}

pub unsafe extern "win64" fn _errno() -> *mut i32 {
    ERRNO.with(|c| c.as_ptr())
}
pub unsafe extern "win64" fn __doserrno() -> *mut u32 {
    DOSERRNO.with(|c| c.as_ptr())
}
pub unsafe extern "win64" fn _get_errno(out: *mut i32) -> i32 {
    if out.is_null() {
        return EINVAL;
    }
    *out = get();
    0
}
pub unsafe extern "win64" fn _set_errno(v: i32) -> i32 {
    set(v);
    0
}
pub unsafe extern "win64" fn _get_doserrno(out: *mut u32) -> i32 {
    if out.is_null() {
        return EINVAL;
    }
    *out = DOSERRNO.with(|c| c.get());
    0
}
pub unsafe extern "win64" fn _set_doserrno(v: u32) -> i32 {
    DOSERRNO.with(|c| c.set(v));
    0
}

pub const MESSAGES: [&str; 43] = [
    "No error",
    "Operation not permitted",
    "No such file or directory",
    "No such process",
    "Interrupted function call",
    "Input/output error",
    "No such device or address",
    "Arg list too long",
    "Exec format error",
    "Bad file descriptor",
    "No child processes",
    "Resource temporarily unavailable",
    "Not enough space",
    "Permission denied",
    "Bad address",
    "Unknown error",
    "Resource device",
    "File exists",
    "Improper link",
    "No such device",
    "Not a directory",
    "Is a directory",
    "Invalid argument",
    "Too many open files in system",
    "Too many open files",
    "Inappropriate I/O control operation",
    "Unknown error",
    "File too large",
    "No space left on device",
    "Invalid seek",
    "Read-only file system",
    "Too many links",
    "Broken pipe",
    "Domain error",
    "Result too large",
    "Unknown error",
    "Resource deadlock avoided",
    "Unknown error",
    "Filename too long",
    "No locks available",
    "Function not implemented",
    "Directory not empty",
    "Illegal byte sequence",
];

pub fn message(e: i32) -> &'static str {
    if e >= 0 && (e as usize) < MESSAGES.len() {
        MESSAGES[e as usize]
    } else {
        "Unknown error"
    }
}

fn errlist() -> &'static [usize; 43] {
    static L: std::sync::OnceLock<[usize; 43]> = std::sync::OnceLock::new();
    L.get_or_init(|| {
        let mut a = [0usize; 43];
        for (i, m) in MESSAGES.iter().enumerate() {
            a[i] = crate::strings::leak_ansi_z(m) as usize;
        }
        a
    })
}

pub unsafe extern "win64" fn __sys_errlist() -> *const *const u8 {
    errlist().as_ptr() as *const *const u8
}
pub unsafe extern "win64" fn __sys_nerr() -> *const i32 {
    static N: i32 = 43;
    &N
}

pub unsafe extern "win64" fn strerror(e: i32) -> *const u8 {
    errlist().get(e as usize).copied().unwrap_or_else(|| errlist()[15]) as *const u8
}

pub unsafe extern "win64" fn strerror_s(buf: *mut u8, n: usize, e: i32) -> i32 {
    if buf.is_null() || n == 0 {
        return EINVAL;
    }
    crate::strings::write_ansi_trunc(message(e), buf, n);
    0
}

pub unsafe extern "win64" fn _wcserror(e: i32) -> *const u16 {
    static W: std::sync::OnceLock<Vec<usize>> = std::sync::OnceLock::new();
    let v = W.get_or_init(|| MESSAGES.iter().map(|m| crate::strings::leak_wide_z(m) as usize).collect());
    v.get(e as usize).copied().unwrap_or(v[15]) as *const u16
}

pub unsafe extern "win64" fn _wcserror_s(buf: *mut u16, n: usize, e: i32) -> i32 {
    if buf.is_null() || n == 0 {
        return EINVAL;
    }
    crate::strings::write_wide_trunc(message(e), buf, n);
    0
}
