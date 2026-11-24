//! ucrt stdio: low-level I/O (`_open`/`_read`/...) and `FILE` streams.
//!
//! * CRT file descriptors are host fds. A side table records the text/binary
//!   mode; text mode translates CRLF<->LF in `_read`/`_write`, except on
//!   ttys and pipes (so the host terminal output stays readable — deviation).
//! * `FILE` is the 48-byte legacy msvcrt layout (`_ptr,_cnt,_base,_flag,
//!   _file,_charbuf,_bufsiz,_tmpfname`) so `msvcrt!__iob_func()[n]` indexing
//!   by mingw code works; the real state lives in a Rust-side `Stream` keyed
//!   by the FILE pointer. `_cnt` is always 0 so inline fast paths fall back
//!   to function calls.
//! * One process-wide recursive lock serializes all stream operations and
//!   backs `_lock_file`/`_unlock_file` (simple, deadlock-free).

use super::errno::{self, EACCES, EBADF, EINVAL, ENOENT, EPIPE};
use crate::strings::{from_ansi, from_wide};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Condvar, Mutex};

// ---------------------------------------------------------------------------
// Recursive lock
// ---------------------------------------------------------------------------

pub struct RecLock {
    st: Mutex<(u64, u32)>,
    cv: Condvar,
}

fn host_tid() -> u64 {
    let mut t: u64 = 0;
    unsafe { libc::pthread_threadid_np(0 as libc::pthread_t, &mut t) };
    t
}

impl RecLock {
    pub const fn new() -> Self {
        RecLock { st: Mutex::new((0, 0)), cv: Condvar::new() }
    }
    pub fn acquire(&self) {
        let me = host_tid();
        let mut s = self.st.lock().unwrap();
        while s.1 > 0 && s.0 != me {
            s = self.cv.wait(s).unwrap();
        }
        s.0 = me;
        s.1 += 1;
    }
    pub fn release(&self) {
        let mut s = self.st.lock().unwrap();
        if s.1 == 0 {
            log::warn!("stdio: unlock without lock");
            return;
        }
        s.1 -= 1;
        if s.1 == 0 {
            s.0 = 0;
            self.cv.notify_all();
        }
    }
}

static STDIO_LOCK: RecLock = RecLock::new();

struct Locked;
impl Locked {
    fn new() -> Self {
        STDIO_LOCK.acquire();
        Locked
    }
}
impl Drop for Locked {
    fn drop(&mut self) {
        STDIO_LOCK.release();
    }
}

// ---------------------------------------------------------------------------
// Low-level I/O
// ---------------------------------------------------------------------------

pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1;
pub const O_RDWR: i32 = 2;
pub const O_APPEND: i32 = 8;
pub const O_RANDOM: i32 = 0x10;
pub const O_SEQUENTIAL: i32 = 0x20;
pub const O_TEMPORARY: i32 = 0x40;
pub const O_NOINHERIT: i32 = 0x80;
pub const O_CREAT: i32 = 0x100;
pub const O_TRUNC: i32 = 0x200;
pub const O_EXCL: i32 = 0x400;
pub const O_SHORT_LIVED: i32 = 0x1000;
pub const O_TEXT: i32 = 0x4000;
pub const O_BINARY: i32 = 0x8000;
pub const O_WTEXT: i32 = 0x10000;
pub const O_U16TEXT: i32 = 0x20000;
pub const O_U8TEXT: i32 = 0x40000;

#[derive(Clone, Copy)]
struct FdInfo {
    text: bool,
    /// Translation disabled for ttys/pipes.
    raw: bool,
    temporary: bool,
    /// Associated Win32 `HANDLE` for this CRT descriptor.
    ///
    /// In both directions the CRT fd and the `FileObj` own distinct close-on-exec POSIX fds
    /// (`F_DUPFD_CLOEXEC`) sharing the same kernel open file description (file offset, status
    /// flags, pipe buffer):
    /// - `_open_osfhandle(h, flags)` duplicates `h`'s fd for the CRT fd and stores `Some(h)`.
    /// - `_get_osfhandle(fd)` lazily duplicates `fd` into an owning `FileObj` handle and caches
    ///   `Some(h)`.
    ///
    /// Retiring `h` via `handles::close(h)` during `_close(fd)` or `_dup2(src, fd)` therefore
    /// never closes the CRT fd, while any surviving `DuplicateHandle` sibling keeps its own
    /// `Arc<FileObj>` and duplicated POSIX fd alive independently of the CRT fd.
    osf_handle: Option<usize>,
}

static FDS: Mutex<Option<HashMap<i32, (FdInfo, Option<std::path::PathBuf>)>>> = Mutex::new(None);
static FMODE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(O_TEXT);
static COMMODE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn fd_info(fd: i32) -> FdInfo {
    let mut g = FDS.lock().unwrap();
    let m = g.get_or_insert_with(HashMap::new);
    if let Some((i, _)) = m.get(&fd) {
        return *i;
    }
    // Std fds: text mode (CRT default), untranslated if tty/pipe.
    let raw = unsafe {
        let mut st: libc::stat = std::mem::zeroed();
        libc::fstat(fd, &mut st) == 0 && (st.st_mode & libc::S_IFMT) != libc::S_IFREG
    };
    let i = FdInfo { text: true, raw, temporary: false, osf_handle: None };
    if (0..=2).contains(&fd) {
        m.insert(fd, (i, None));
    }
    i
}

fn set_fd_info(fd: i32, i: FdInfo, path: Option<std::path::PathBuf>) {
    FDS.lock().unwrap().get_or_insert_with(HashMap::new).insert(fd, (i, path));
}

fn fd_valid(fd: i32) -> bool {
    fd >= 0 && unsafe { libc::fcntl(fd, libc::F_GETFD) } != -1
}

fn bad_fd() -> i32 {
    errno::set(EBADF);
    -1
}

pub fn open_host(win_path: &str, oflag: i32, pmode: i32) -> i32 {
    let Some(host) = crate::path::to_host(win_path) else {
        if crate::path::device_name(win_path) == Some("NUL") {
            return open_host_path(std::path::Path::new("/dev/null"), oflag, pmode);
        }
        errno::set(ENOENT);
        return -1;
    };
    open_host_path(&host, oflag, pmode)
}

fn open_host_path(host: &std::path::Path, oflag: i32, pmode: i32) -> i32 {
    let acc = oflag & 3;
    let mutating = acc != O_RDONLY || (oflag & (O_APPEND | O_CREAT | O_TRUNC | O_TEMPORARY)) != 0;
    if mutating && crate::path::is_protected_host_write(host) {
        errno::set(EACCES);
        return -1;
    }
    if (oflag & O_TEMPORARY) != 0 && crate::path::is_protected_host_remove_or_rename(host) {
        errno::set(EACCES);
        return -1;
    }
    let mut f = match acc {
        O_WRONLY => libc::O_WRONLY,
        O_RDWR => libc::O_RDWR,
        _ => libc::O_RDONLY,
    };
    if oflag & O_APPEND != 0 {
        f |= libc::O_APPEND;
    }
    if oflag & O_CREAT != 0 {
        f |= libc::O_CREAT;
    }
    if oflag & O_TRUNC != 0 {
        f |= libc::O_TRUNC;
    }
    if oflag & O_EXCL != 0 {
        f |= libc::O_EXCL;
    }
    f |= libc::O_CLOEXEC;
    let mode = if pmode & 0x80 != 0 { 0o644 } else { 0o444 }; // _S_IWRITE
    let c = std::ffi::CString::new(host.as_os_str().as_encoded_bytes()).unwrap();
    let fd = unsafe { libc::open(c.as_ptr(), f, mode as libc::c_uint) };
    if fd < 0 {
        errno::set_from_host();
        return -1;
    }
    let text = if oflag & O_BINARY != 0 {
        false
    } else if oflag & (O_TEXT | O_WTEXT | O_U16TEXT | O_U8TEXT) != 0 {
        true
    } else {
        FMODE.load(std::sync::atomic::Ordering::Relaxed) & O_TEXT != 0
    };
    if oflag & (O_WTEXT | O_U16TEXT | O_U8TEXT) != 0 {
        stub!("Unicode text modes (0x{oflag:x}) treated as plain text mode");
    }
    set_fd_info(fd, FdInfo { text, raw: false, temporary: oflag & O_TEMPORARY != 0, osf_handle: None }, Some(host.to_path_buf()));
    fd
}

pub unsafe extern "win64" fn _open(path: *const u8, oflag: i32, pmode: i32) -> i32 {
    if path.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    open_host(&from_ansi(path), oflag, pmode)
}
pub unsafe extern "win64" fn _wopen(path: *const u16, oflag: i32, pmode: i32) -> i32 {
    if path.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    open_host(&from_wide(path), oflag, pmode)
}
pub unsafe extern "win64" fn _sopen(path: *const u8, oflag: i32, _sh: i32, pmode: i32) -> i32 {
    _open(path, oflag, pmode)
}
pub unsafe extern "win64" fn _wsopen(path: *const u16, oflag: i32, _sh: i32, pmode: i32) -> i32 {
    _wopen(path, oflag, pmode)
}
pub unsafe extern "win64" fn _sopen_s(pfh: *mut i32, path: *const u8, oflag: i32, _sh: i32, pmode: i32) -> i32 {
    *pfh = _open(path, oflag, pmode);
    if *pfh < 0 {
        errno::get()
    } else {
        0
    }
}
pub unsafe extern "win64" fn _wsopen_s(pfh: *mut i32, path: *const u16, oflag: i32, _sh: i32, pmode: i32) -> i32 {
    *pfh = _wopen(path, oflag, pmode);
    if *pfh < 0 {
        errno::get()
    } else {
        0
    }
}
/// `errno_t _wsopen_dispatch(path, oflag, shflag, pmode, int* pfh, int secure)`
pub unsafe extern "win64" fn _wsopen_dispatch(path: *const u16, oflag: i32, _sh: i32, pmode: i32, pfh: *mut i32, _secure: i32) -> i32 {
    *pfh = _wopen(path, oflag, pmode);
    if *pfh < 0 {
        errno::get()
    } else {
        0
    }
}
pub unsafe extern "win64" fn _sopen_dispatch(path: *const u8, oflag: i32, _sh: i32, pmode: i32, pfh: *mut i32, _secure: i32) -> i32 {
    *pfh = _open(path, oflag, pmode);
    if *pfh < 0 {
        errno::get()
    } else {
        0
    }
}

pub unsafe extern "win64" fn _close(fd: i32) -> i32 {
    if !fd_valid(fd) {
        return bad_fd();
    }
    let ent = FDS.lock().unwrap().get_or_insert_with(HashMap::new).remove(&fd);
    if let Some((info, _)) = &ent {
        if let Some(h) = info.osf_handle {
            // Both `_open_osfhandle` and `_get_osfhandle` keep the CRT fd and the `FileObj`
            // on distinct duplicated POSIX fds sharing one open file description. Retiring `h`
            // never closes `fd`, so `libc::close(fd)` below closes `fd` cleanly once while any
            // `DuplicateHandle` sibling's `Arc<FileObj>` (and its own POSIX fd) stays alive.
            let _ = crate::handles::close(h as crate::types::HANDLE);
        }
    }
    let r = libc::close(fd);
    if let Some((i, Some(p))) = ent {
        if i.temporary && !crate::path::is_protected_host_remove_or_rename(&p) {
            let _ = std::fs::remove_file(p);
        }
    }
    if r < 0 {
        errno::set_from_host();
        -1
    } else {
        0
    }
}

/// Associate a CRT file descriptor with an existing Win32 file/pipe HANDLE,
/// transferring ownership of `osfhandle` to the returned CRT fd.
///
/// Allocates an independent POSIX fd (`F_DUPFD_CLOEXEC`) sharing the same kernel open
/// file description (offset, flags, pipe buffer) as `osfhandle`'s `FileObj`. Keeping the
/// CRT fd distinct from `FileObj`'s internal fd guarantees that `_close(fd)` and
/// `_dup2(src, fd)` retire `osfhandle` cleanly without ever double-closing or late-closing
/// a reused descriptor, even when duplicate Win32 handles share the same `Arc<FileObj>`.
pub unsafe extern "win64" fn _open_osfhandle(osfhandle: isize, flags: i32) -> i32 {
    use crate::kernel32::file::{file_fd_and_kind, FileKind};
    use crate::types::{HANDLE, INVALID_HANDLE_VALUE};
    let h = osfhandle as HANDLE;
    if h.is_null() || h == INVALID_HANDLE_VALUE {
        errno::set(EBADF);
        return -1;
    }
    let Some((raw_fd, kind)) = file_fd_and_kind(h) else {
        errno::set(EBADF);
        return -1;
    };
    if !fd_valid(raw_fd) {
        errno::set(EBADF);
        return -1;
    }
    let crt_fd = libc::fcntl(raw_fd, libc::F_DUPFD_CLOEXEC, 0);
    if crt_fd < 0 {
        errno::set_from_host();
        return -1;
    }
    if (flags & O_APPEND) != 0 {
        let cur = libc::fcntl(crt_fd, libc::F_GETFL);
        if cur >= 0 {
            let _ = libc::fcntl(crt_fd, libc::F_SETFL, cur | libc::O_APPEND);
        }
    }
    let text = if (flags & O_BINARY) != 0 {
        false
    } else if (flags & (O_TEXT | O_WTEXT | O_U16TEXT | O_U8TEXT)) != 0 {
        true
    } else {
        false
    };
    let raw = kind != FileKind::Disk;
    set_fd_info(
        crt_fd,
        FdInfo {
            text,
            raw,
            temporary: false,
            osf_handle: Some(osfhandle as usize),
        },
        None,
    );
    crt_fd
}

/// Return the Win32 HANDLE backing a CRT file descriptor without transferring ownership.
///
/// Lazily exported handles own an independent close-on-exec POSIX duplicate (`F_DUPFD_CLOEXEC`)
/// of `fd` sharing the same kernel open file description so `DuplicateHandle` siblings stay
/// valid after `_close(fd)` or `_dup2(other, fd)`.
pub unsafe extern "win64" fn _get_osfhandle(fd: i32) -> isize {
    use std::os::fd::{FromRawFd, OwnedFd};
    use crate::kernel32::file::{classify_host_fd, std_handle, wrap_owned_fd, FileKind};
    if !fd_valid(fd) {
        errno::set(EBADF);
        return -1;
    }
    let mut g = FDS.lock().unwrap();
    let m = g.get_or_insert_with(HashMap::new);
    if let Some((info, _)) = m.get(&fd) {
        if let Some(h) = info.osf_handle {
            if crate::handles::get(h as crate::types::HANDLE).is_some() {
                return h as isize;
            }
        }
    }
    if (0..=2).contains(&fd) {
        drop(g);
        return std_handle(fd as usize) as isize;
    }
    let dup_fd = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0);
    if dup_fd < 0 {
        errno::set_from_host();
        return -1;
    }
    let owned_fd = OwnedFd::from_raw_fd(dup_fd);
    let kind = classify_host_fd(dup_fd);
    let h = wrap_owned_fd(owned_fd, kind, format!("crt://fd/{fd}"), 0xC000_0000);
    if let Some((info, _)) = m.get_mut(&fd) {
        info.osf_handle = Some(h as usize);
    } else {
        let raw = kind != FileKind::Disk;
        m.insert(
            fd,
            (
                FdInfo {
                    text: true,
                    raw,
                    temporary: false,
                    osf_handle: Some(h as usize),
                },
                None,
            ),
        );
    }
    h as isize
}

/// Wait for a pipe fd to become ready for `events` (`POLLIN` or `POLLOUT`) when its shared
/// kernel open file description has `O_NONBLOCK` set (e.g. after an overlapped Win32 pipe
/// write on a handle sharing the same open file description). Scoped strictly to `FileKind::Pipe`
/// without toggling `O_NONBLOCK`. Restores `orig_errno` if `fd` is not a pipe.
unsafe fn wait_pipe_fd_ready(fd: i32, events: libc::c_short, orig_errno: i32) -> bool {
    use crate::kernel32::file::{classify_host_fd, FileKind};
    if classify_host_fd(fd) != FileKind::Pipe {
        *libc::__error() = orig_errno;
        return false;
    }
    loop {
        let mut pfd = libc::pollfd { fd, events, revents: 0 };
        let pr = libc::poll(&mut pfd, 1, -1);
        if pr >= 0 {
            return true;
        }
        if *libc::__error() == libc::EINTR {
            continue;
        }
        return false;
    }
}

/// Raw read with CRLF->LF translation in text mode.
pub unsafe fn read_fd(fd: i32, buf: *mut u8, n: usize) -> isize {
    let info = fd_info(fd);
    let r = loop {
        let k = libc::read(fd, buf as *mut c_void, n);
        if k >= 0 {
            break k;
        }
        let e = *libc::__error();
        if e == libc::EINTR {
            continue;
        }
        if (e == libc::EAGAIN || e == libc::EWOULDBLOCK)
            && wait_pipe_fd_ready(fd, libc::POLLIN, e)
        {
            continue;
        }
        errno::set_from_host();
        return -1;
    };
    if r == 0 || !info.text || info.raw {
        return r;
    }
    let s = std::slice::from_raw_parts_mut(buf, r as usize);
    let mut w = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'\r' {
            if i + 1 < s.len() {
                if s[i + 1] == b'\n' {
                    i += 1;
                    continue;
                }
            } else {
                // CR at end of buffer: peek one byte.
                let mut nb = 0u8;
                let k = libc::read(fd, &mut nb as *mut u8 as *mut c_void, 1);
                if k == 1 {
                    if nb == b'\n' {
                        s[w] = b'\n';
                        w += 1;
                        i += 1;
                        continue;
                    }
                    libc::lseek(fd, -1, libc::SEEK_CUR);
                }
            }
        }
        s[w] = s[i];
        w += 1;
        i += 1;
    }
    w as isize
}

/// Raw write with LF->CRLF translation in text mode. Returns source bytes.
pub unsafe fn write_fd(fd: i32, data: &[u8]) -> isize {
    let info = fd_info(fd);
    let tmp;
    let out: &[u8] = if info.text && !info.raw && data.contains(&b'\n') {
        let mut v = Vec::with_capacity(data.len() + 16);
        for &b in data {
            if b == b'\n' {
                v.push(b'\r');
            }
            v.push(b);
        }
        tmp = v;
        &tmp
    } else {
        data
    };
    let mut off = 0;
    while off < out.len() {
        let r = libc::write(fd, out[off..].as_ptr() as *const c_void, out.len() - off);
        if r < 0 {
            let e = *libc::__error();
            if e == libc::EINTR {
                continue;
            }
            if (e == libc::EAGAIN || e == libc::EWOULDBLOCK)
                && wait_pipe_fd_ready(fd, libc::POLLOUT, e)
            {
                continue;
            }
            errno::set_from_host();
            return -1;
        }
        if r == 0 {
            errno::set(EPIPE);
            return -1;
        }
        off += r as usize;
    }
    data.len() as isize
}

pub unsafe extern "win64" fn _read(fd: i32, buf: *mut c_void, n: u32) -> i32 {
    if !fd_valid(fd) {
        return bad_fd();
    }
    read_fd(fd, buf as *mut u8, n as usize) as i32
}

pub unsafe extern "win64" fn _write(fd: i32, buf: *const c_void, n: u32) -> i32 {
    if !fd_valid(fd) {
        return bad_fd();
    }
    write_fd(fd, std::slice::from_raw_parts(buf as *const u8, n as usize)) as i32
}

pub unsafe extern "win64" fn _lseeki64(fd: i32, off: i64, whence: i32) -> i64 {
    if !fd_valid(fd) {
        bad_fd();
        return -1;
    }
    let r = libc::lseek(fd, off, whence);
    if r < 0 {
        errno::set_from_host();
    }
    r
}
pub unsafe extern "win64" fn _lseek(fd: i32, off: i32, whence: i32) -> i32 {
    let r = _lseeki64(fd, off as i64, whence);
    if r > i32::MAX as i64 {
        errno::set(EINVAL);
        return -1;
    }
    r as i32
}
pub unsafe extern "win64" fn _telli64(fd: i32) -> i64 {
    _lseeki64(fd, 0, libc::SEEK_CUR)
}
pub unsafe extern "win64" fn _tell(fd: i32) -> i32 {
    _lseek(fd, 0, libc::SEEK_CUR)
}

pub unsafe extern "win64" fn _filelengthi64(fd: i32) -> i64 {
    let mut st: libc::stat = std::mem::zeroed();
    if libc::fstat(fd, &mut st) != 0 {
        bad_fd();
        return -1;
    }
    st.st_size
}
pub unsafe extern "win64" fn _filelength(fd: i32) -> i32 {
    _filelengthi64(fd) as i32
}

pub unsafe extern "win64" fn _eof(fd: i32) -> i32 {
    let cur = libc::lseek(fd, 0, libc::SEEK_CUR);
    let end = _filelengthi64(fd);
    if cur < 0 || end < 0 {
        return -1;
    }
    (cur >= end) as i32
}

pub unsafe extern "win64" fn _dup(fd: i32) -> i32 {
    if !fd_valid(fd) {
        return bad_fd();
    }
    let n = libc::dup(fd);
    if n < 0 {
        errno::set_from_host();
        return -1;
    }
    let i = fd_info(fd);
    set_fd_info(n, FdInfo { temporary: false, osf_handle: None, ..i }, None);
    n
}

pub unsafe extern "win64" fn _dup2(fd: i32, fd2: i32) -> i32 {
    if !fd_valid(fd) || fd2 < 0 {
        return bad_fd();
    }
    // Self-dup is a validated no-op that preserves `FDS[fd]` and its `osf_handle` association.
    if fd == fd2 {
        return 0;
    }
    let src_info = fd_info(fd);
    if libc::dup2(fd, fd2) < 0 {
        errno::set_from_host();
        return -1;
    }
    // Atomically swap `FDS[fd2]` after `libc::dup2` succeeds so failed dups never disturb
    // `fd2`, and retire any previous `osf_handle` outside the lock. Because both
    // `_open_osfhandle` and `_get_osfhandle` keep the CRT fd and `FileObj` on distinct
    // duplicated POSIX fds, closing `old_h` never closes the newly duplicated `fd2`, and any
    // retained `DuplicateHandle` sibling keeps its own `FileObj` fd alive on the original file.
    let prev = FDS
        .lock()
        .unwrap()
        .get_or_insert_with(HashMap::new)
        .insert(fd2, (FdInfo { temporary: false, osf_handle: None, ..src_info }, None));
    if let Some((old_info, old_path)) = prev {
        if let Some(old_h) = old_info.osf_handle {
            let _ = crate::handles::close(old_h as crate::types::HANDLE);
        }
        if old_info.temporary {
            if let Some(p) = old_path {
                if !crate::path::is_protected_host_remove_or_rename(&p) {
                    let _ = std::fs::remove_file(p);
                }
            }
        }
    }
    0
}

pub unsafe extern "win64" fn _isatty(fd: i32) -> i32 {
    // Windows returns nonzero for character devices.
    (libc::isatty(fd) != 0) as i32 * 0x40
}

pub unsafe extern "win64" fn _setmode(fd: i32, mode: i32) -> i32 {
    if !fd_valid(fd) {
        return bad_fd();
    }
    let i = fd_info(fd);
    let old = if i.text { O_TEXT } else { O_BINARY };
    let text = match mode {
        O_BINARY => false,
        O_TEXT | O_WTEXT | O_U16TEXT | O_U8TEXT => true,
        _ => {
            errno::set(EINVAL);
            super::runtime::invalid_parameter("_setmode");
            return -1;
        }
    };
    let path = FDS.lock().unwrap().get_or_insert_with(HashMap::new).get(&fd).and_then(|e| e.1.clone());
    set_fd_info(fd, FdInfo { text, ..i }, path);
    old
}

pub unsafe extern "win64" fn _set_fmode(mode: i32) -> i32 {
    if mode != O_TEXT && mode != O_BINARY && mode != O_WTEXT {
        errno::set(EINVAL);
        return EINVAL;
    }
    FMODE.store(mode, std::sync::atomic::Ordering::Relaxed);
    0
}
pub unsafe extern "win64" fn _get_fmode(p: *mut i32) -> i32 {
    if p.is_null() {
        return EINVAL;
    }
    *p = FMODE.load(std::sync::atomic::Ordering::Relaxed);
    0
}
pub unsafe extern "win64" fn __p__fmode() -> *mut i32 {
    FMODE.as_ptr()
}
pub unsafe extern "win64" fn __p__commode() -> *mut i32 {
    COMMODE.as_ptr()
}

pub unsafe extern "win64" fn _commit(fd: i32) -> i32 {
    if libc::fsync(fd) != 0 {
        errno::set_from_host();
        return -1;
    }
    0
}

pub unsafe extern "win64" fn _chsize_s(fd: i32, size: i64) -> i32 {
    if libc::ftruncate(fd, size) != 0 {
        return errno::set_from_host();
    }
    0
}
pub unsafe extern "win64" fn _chsize(fd: i32, size: i32) -> i32 {
    if _chsize_s(fd, size as i64) != 0 {
        -1
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// FILE streams
// ---------------------------------------------------------------------------

/// Legacy msvcrt FILE layout (48 bytes).
#[repr(C)]
pub struct FILE {
    _ptr: *mut u8,
    _cnt: i32,
    _base: *mut u8,
    _flag: i32,
    _file: i32,
    _charbuf: i32,
    _bufsiz: i32,
    _tmpfname: *mut u8,
}
const _: () = assert!(std::mem::size_of::<FILE>() == 48);

#[derive(PartialEq, Clone, Copy)]
enum BufMode {
    Full,
    Line,
    None,
}

struct Stream {
    fd: i32,
    readable: bool,
    writable: bool,
    eof: bool,
    err: bool,
    /// Pending write data (not yet passed to `write_fd`).
    wbuf: Vec<u8>,
    /// Read-ahead data (already translated); `rpos` = next byte.
    rbuf: Vec<u8>,
    rpos: usize,
    /// Pushed-back bytes (ungetc), LIFO.
    unget: Vec<u8>,
    mode: BufMode,
    bufsize: usize,
    is_static: bool,
}

struct StreamTable {
    map: HashMap<usize, Stream>,
}

static STREAMS: Mutex<Option<StreamTable>> = Mutex::new(None);

/// `__acrt_iob_func` / `__iob_func` array: stdin, stdout, stderr.
struct Iob(std::cell::UnsafeCell<[FILE; 3]>);
unsafe impl Sync for Iob {}
static IOB: Iob = Iob(std::cell::UnsafeCell::new(unsafe { std::mem::zeroed() }));

fn with_table<R>(f: impl FnOnce(&mut StreamTable) -> R) -> R {
    let mut g = STREAMS.lock().unwrap();
    let t = g.get_or_insert_with(|| {
        let mut map = HashMap::new();
        let base = IOB.0.get() as *mut FILE;
        for fd in 0..3 {
            let tty = unsafe { libc::isatty(fd) } != 0;
            unsafe {
                (*base.add(fd as usize))._file = fd;
                (*base.add(fd as usize))._flag = if fd == 0 { 1 } else { 2 };
            }
            map.insert(
                unsafe { base.add(fd as usize) } as usize,
                Stream {
                    fd,
                    readable: fd == 0,
                    writable: fd != 0,
                    eof: false,
                    err: false,
                    wbuf: Vec::new(),
                    rbuf: Vec::new(),
                    rpos: 0,
                    unget: Vec::new(),
                    // stderr unbuffered; stdout line-buffered on a tty.
                    mode: if fd == 2 { BufMode::None } else if tty { BufMode::Line } else { BufMode::Full },
                    bufsize: 4096,
                    is_static: true,
                },
            );
        }
        StreamTable { map }
    });
    f(t)
}

fn with_stream<R>(fp: *mut c_void, f: impl FnOnce(&mut Stream) -> R) -> Option<R> {
    with_table(|t| t.map.get_mut(&(fp as usize)).map(f))
}

pub unsafe extern "win64" fn __acrt_iob_func(i: u32) -> *mut FILE {
    with_table(|_| ());
    (IOB.0.get() as *mut FILE).add(i.min(2) as usize)
}

/// msvcrt `__iob_func()`: pointer to the FILE array (stride 48).
pub unsafe fn iob_base() -> *mut FILE {
    __acrt_iob_func(0)
}

impl Stream {
    unsafe fn flush(&mut self) -> bool {
        if self.wbuf.is_empty() {
            return true;
        }
        let data = std::mem::take(&mut self.wbuf);
        if write_fd(self.fd, &data) < 0 {
            self.err = true;
            return false;
        }
        true
    }

    /// Drop read-ahead, seeking the fd back so its position is logical.
    unsafe fn discard_read(&mut self) {
        let pending = (self.rbuf.len() - self.rpos) + self.unget.len();
        if pending > 0 {
            // Translated data: exact only for binary streams (same as Windows'
            // text-mode caveats for ftell).
            libc::lseek(self.fd, -(pending as i64), libc::SEEK_CUR);
        }
        self.rbuf.clear();
        self.rpos = 0;
        self.unget.clear();
    }

    unsafe fn write(&mut self, data: &[u8]) -> usize {
        if !self.writable {
            self.err = true;
            errno::set(EBADF);
            return 0;
        }
        if !self.rbuf.is_empty() || !self.unget.is_empty() {
            self.discard_read();
        }
        match self.mode {
            BufMode::None => {
                if !self.flush() {
                    return 0;
                }
                if write_fd(self.fd, data) < 0 {
                    self.err = true;
                    return 0;
                }
            }
            _ => {
                self.wbuf.extend_from_slice(data);
                let line_flush = self.mode == BufMode::Line && data.contains(&b'\n');
                if (line_flush || self.wbuf.len() >= self.bufsize) && !self.flush() {
                    return 0;
                }
            }
        }
        data.len()
    }

    unsafe fn fill(&mut self) -> bool {
        if self.eof {
            return false;
        }
        if !self.wbuf.is_empty() {
            self.flush();
        }
        // Reading stdin flushes stdout (interactive convenience; Windows does too).
        let cap = if self.mode == BufMode::None { 1 } else { self.bufsize.max(512) };
        self.rbuf.resize(cap, 0);
        self.rpos = 0;
        let n = read_fd(self.fd, self.rbuf.as_mut_ptr(), cap);
        if n <= 0 {
            self.rbuf.clear();
            if n == 0 {
                self.eof = true;
            } else {
                self.err = true;
            }
            return false;
        }
        self.rbuf.truncate(n as usize);
        true
    }

    unsafe fn getc(&mut self) -> Option<u8> {
        if !self.readable {
            self.err = true;
            errno::set(EBADF);
            return None;
        }
        if let Some(b) = self.unget.pop() {
            return Some(b);
        }
        if self.rpos >= self.rbuf.len() && !self.fill() {
            return None;
        }
        let b = self.rbuf[self.rpos];
        self.rpos += 1;
        Some(b)
    }

    unsafe fn read(&mut self, out: &mut [u8]) -> usize {
        let mut n = 0;
        while n < out.len() {
            if let Some(b) = self.unget.pop() {
                out[n] = b;
                n += 1;
                continue;
            }
            if self.rpos < self.rbuf.len() {
                let k = (self.rbuf.len() - self.rpos).min(out.len() - n);
                out[n..n + k].copy_from_slice(&self.rbuf[self.rpos..self.rpos + k]);
                self.rpos += k;
                n += k;
                continue;
            }
            if !self.readable {
                self.err = true;
                break;
            }
            // Large reads bypass the buffer.
            if out.len() - n >= self.bufsize {
                if !self.wbuf.is_empty() {
                    self.flush();
                }
                let r = read_fd(self.fd, out[n..].as_mut_ptr(), out.len() - n);
                if r <= 0 {
                    if r == 0 {
                        self.eof = true;
                    } else {
                        self.err = true;
                    }
                    break;
                }
                n += r as usize;
                continue;
            }
            if !self.fill() {
                break;
            }
        }
        n
    }

    unsafe fn tell(&mut self) -> i64 {
        let pos = libc::lseek(self.fd, 0, libc::SEEK_CUR);
        if pos < 0 {
            errno::set_from_host();
            return -1;
        }
        pos + self.wbuf.len() as i64 - (self.rbuf.len() - self.rpos) as i64 - self.unget.len() as i64
    }

    unsafe fn seek(&mut self, off: i64, whence: i32) -> i32 {
        if !self.flush() {
            return -1;
        }
        let (off, whence) = if whence == libc::SEEK_CUR {
            let cur = self.tell();
            (cur + off, libc::SEEK_SET)
        } else {
            (off, whence)
        };
        self.rbuf.clear();
        self.rpos = 0;
        self.unget.clear();
        if libc::lseek(self.fd, off, whence) < 0 {
            errno::set_from_host();
            return -1;
        }
        self.eof = false;
        0
    }
}

/// Write bytes to a stream (used by the printf family). Returns the byte
/// count or -1.
pub unsafe fn write_stream_bytes(fp: *mut c_void, data: &[u8]) -> i32 {
    let _l = Locked::new();
    match with_stream(fp, |s| {
        let n = s.write(data);
        if n == data.len() {
            n as i32
        } else {
            -1
        }
    }) {
        Some(r) => r,
        None => {
            errno::set(EINVAL);
            super::runtime::invalid_parameter("stream is not a FILE*");
            -1
        }
    }
}

/// Flush every stream (exit path).
pub fn flush_all() {
    let _l = Locked::new();
    with_table(|t| {
        for s in t.map.values_mut() {
            unsafe { s.flush() };
        }
    });
}

struct ModeFlags {
    oflag: i32,
    readable: bool,
    writable: bool,
}

fn parse_mode(m: &str) -> Option<ModeFlags> {
    let mut chars = m.trim_start().chars();
    let first = chars.next()?;
    let (mut oflag, mut r, mut w) = match first {
        'r' => (O_RDONLY, true, false),
        'w' => (O_WRONLY | O_CREAT | O_TRUNC, false, true),
        'a' => (O_WRONLY | O_CREAT | O_APPEND, false, true),
        _ => return None,
    };
    let rest: String = chars.collect();
    let (flags, ccs) = match rest.split_once(',') {
        Some((a, b)) => (a.to_string(), Some(b.trim().to_string())),
        None => (rest, None),
    };
    let mut binary = None;
    for c in flags.chars() {
        match c {
            '+' => {
                oflag = (oflag & !3) | O_RDWR;
                r = true;
                w = true;
            }
            'b' => binary = Some(true),
            't' => binary = Some(false),
            'c' | 'n' | 'N' | 'S' | 'R' | 'T' | 'D' | 'x' | 'e' | ' ' => {
                if c == 'x' {
                    oflag |= O_EXCL;
                }
                if c == 'D' || c == 'T' {
                    if c == 'D' {
                        oflag |= O_TEMPORARY;
                    }
                }
            }
            _ => return None,
        }
    }
    match binary {
        Some(true) => oflag |= O_BINARY,
        Some(false) => oflag |= O_TEXT,
        None => {}
    }
    if let Some(c) = ccs {
        let c = c.to_ascii_uppercase().replace(' ', "");
        if c.contains("UTF-16") || c.contains("UNICODE") {
            stub!("fopen ccs={c}: UTF-16 stream encoding not implemented; byte stream instead");
        }
        oflag = (oflag & !(O_TEXT)) | O_BINARY;
    }
    Some(ModeFlags { oflag, readable: r, writable: w })
}

fn new_stream(fd: i32, mf: &ModeFlags) -> *mut FILE {
    let f: *mut FILE = Box::into_raw(Box::new(unsafe { std::mem::zeroed::<FILE>() }));
    unsafe {
        (*f)._file = fd;
        (*f)._flag = (mf.readable as i32) | ((mf.writable as i32) << 1);
    }
    with_table(|t| {
        t.map.insert(
            f as usize,
            Stream {
                fd,
                readable: mf.readable,
                writable: mf.writable,
                eof: false,
                err: false,
                wbuf: Vec::new(),
                rbuf: Vec::new(),
                rpos: 0,
                unget: Vec::new(),
                mode: BufMode::Full,
                bufsize: 4096,
                is_static: false,
            },
        )
    });
    f
}

unsafe fn fopen_str(path: &str, mode: &str) -> *mut FILE {
    let Some(mf) = parse_mode(mode) else {
        errno::set(EINVAL);
        super::runtime::invalid_parameter("fopen: bad mode");
        return std::ptr::null_mut();
    };
    let fd = open_host(path, mf.oflag, 0x180);
    if fd < 0 {
        log::debug!("fopen({path:?}, {mode:?}) failed: errno {}", errno::get());
        return std::ptr::null_mut();
    }
    let _l = Locked::new();
    new_stream(fd, &mf)
}

pub unsafe extern "win64" fn fopen(path: *const u8, mode: *const u8) -> *mut FILE {
    if path.is_null() || mode.is_null() {
        errno::set(EINVAL);
        return std::ptr::null_mut();
    }
    fopen_str(&from_ansi(path), &from_ansi(mode))
}
pub unsafe extern "win64" fn _wfopen(path: *const u16, mode: *const u16) -> *mut FILE {
    if path.is_null() || mode.is_null() {
        errno::set(EINVAL);
        return std::ptr::null_mut();
    }
    fopen_str(&from_wide(path), &from_wide(mode))
}
pub unsafe extern "win64" fn _fsopen(path: *const u8, mode: *const u8, _sh: i32) -> *mut FILE {
    fopen(path, mode)
}
pub unsafe extern "win64" fn _wfsopen(path: *const u16, mode: *const u16, _sh: i32) -> *mut FILE {
    _wfopen(path, mode)
}
pub unsafe extern "win64" fn fopen_s(pf: *mut *mut FILE, path: *const u8, mode: *const u8) -> i32 {
    if pf.is_null() {
        return EINVAL;
    }
    *pf = fopen(path, mode);
    if (*pf).is_null() {
        errno::get()
    } else {
        0
    }
}
pub unsafe extern "win64" fn _wfopen_s(pf: *mut *mut FILE, path: *const u16, mode: *const u16) -> i32 {
    if pf.is_null() {
        return EINVAL;
    }
    *pf = _wfopen(path, mode);
    if (*pf).is_null() {
        errno::get()
    } else {
        0
    }
}

pub unsafe extern "win64" fn _fdopen(fd: i32, mode: *const u8) -> *mut FILE {
    let Some(mf) = parse_mode(&from_ansi(mode)) else {
        errno::set(EINVAL);
        return std::ptr::null_mut();
    };
    if !fd_valid(fd) {
        bad_fd();
        return std::ptr::null_mut();
    }
    let _l = Locked::new();
    new_stream(fd, &mf)
}
pub unsafe extern "win64" fn _wfdopen(fd: i32, mode: *const u16) -> *mut FILE {
    let m = crate::strings::leak_ansi_z(&from_wide(mode));
    _fdopen(fd, m)
}

pub unsafe extern "win64" fn fclose(fp: *mut FILE) -> i32 {
    let _l = Locked::new();
    let s = with_table(|t| {
        let st = t.map.get(&(fp as usize)).map(|s| s.is_static);
        match st {
            Some(true) => t.map.get_mut(&(fp as usize)).map(|s| {
                let ok = unsafe { s.flush() };
                (ok, -1)
            }),
            Some(false) => t.map.remove(&(fp as usize)).map(|mut s| {
                let ok = unsafe { s.flush() };
                (ok, s.fd)
            }),
            None => None,
        }
    });
    match s {
        None => {
            errno::set(EINVAL);
            super::runtime::invalid_parameter("fclose: not a FILE*");
            -1
        }
        Some((ok, fd)) => {
            let mut r = if ok { 0 } else { -1 };
            if fd >= 0 {
                if _close(fd) != 0 {
                    r = -1;
                }
                drop(Box::from_raw(fp));
            }
            r
        }
    }
}

pub unsafe extern "win64" fn _fcloseall() -> i32 {
    let fps: Vec<usize> = with_table(|t| t.map.iter().filter(|(_, s)| !s.is_static).map(|(k, _)| *k).collect());
    for f in &fps {
        fclose(*f as *mut FILE);
    }
    fps.len() as i32
}

pub unsafe extern "win64" fn fflush(fp: *mut FILE) -> i32 {
    if fp.is_null() {
        flush_all();
        return 0;
    }
    let _l = Locked::new();
    match with_stream(fp as *mut c_void, |s| s.flush()) {
        Some(true) => 0,
        _ => -1,
    }
}
pub unsafe extern "win64" fn _fflush_nolock(fp: *mut FILE) -> i32 {
    fflush(fp)
}
pub unsafe extern "win64" fn _flushall() -> i32 {
    flush_all();
    0
}

pub unsafe extern "win64" fn fwrite(buf: *const c_void, size: usize, count: usize, fp: *mut FILE) -> usize {
    let total = size.saturating_mul(count);
    if total == 0 {
        return 0;
    }
    let data = std::slice::from_raw_parts(buf as *const u8, total);
    let _l = Locked::new();
    match with_stream(fp as *mut c_void, |s| s.write(data)) {
        Some(n) => n / size,
        None => {
            errno::set(EINVAL);
            0
        }
    }
}
pub unsafe extern "win64" fn _fwrite_nolock(buf: *const c_void, size: usize, count: usize, fp: *mut FILE) -> usize {
    fwrite(buf, size, count, fp)
}

pub unsafe extern "win64" fn fread(buf: *mut c_void, size: usize, count: usize, fp: *mut FILE) -> usize {
    let total = size.saturating_mul(count);
    if total == 0 {
        return 0;
    }
    let out = std::slice::from_raw_parts_mut(buf as *mut u8, total);
    let _l = Locked::new();
    match with_stream(fp as *mut c_void, |s| s.read(out)) {
        Some(n) => n / size,
        None => {
            errno::set(EINVAL);
            0
        }
    }
}
pub unsafe extern "win64" fn _fread_nolock(buf: *mut c_void, size: usize, count: usize, fp: *mut FILE) -> usize {
    fread(buf, size, count, fp)
}
pub unsafe extern "win64" fn fread_s(buf: *mut c_void, bufsize: usize, size: usize, count: usize, fp: *mut FILE) -> usize {
    if size != 0 && count > bufsize / size {
        errno::set(super::errno::ERANGE);
        super::runtime::invalid_parameter("fread_s: buffer too small");
        return 0;
    }
    fread(buf, size, count, fp)
}

pub unsafe extern "win64" fn fgetc(fp: *mut FILE) -> i32 {
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| s.getc()).flatten().map(|b| b as i32).unwrap_or(-1)
}
pub unsafe extern "win64" fn getc(fp: *mut FILE) -> i32 {
    fgetc(fp)
}
pub unsafe extern "win64" fn _fgetc_nolock(fp: *mut FILE) -> i32 {
    fgetc(fp)
}
pub unsafe extern "win64" fn _getc_nolock(fp: *mut FILE) -> i32 {
    fgetc(fp)
}
pub unsafe extern "win64" fn getchar() -> i32 {
    fgetc(__acrt_iob_func(0))
}

pub unsafe extern "win64" fn fputc(c: i32, fp: *mut FILE) -> i32 {
    let b = [c as u8];
    if write_stream_bytes(fp as *mut c_void, &b) == 1 {
        c & 0xFF
    } else {
        -1
    }
}
pub unsafe extern "win64" fn putc(c: i32, fp: *mut FILE) -> i32 {
    fputc(c, fp)
}
pub unsafe extern "win64" fn _fputc_nolock(c: i32, fp: *mut FILE) -> i32 {
    fputc(c, fp)
}
pub unsafe extern "win64" fn _putc_nolock(c: i32, fp: *mut FILE) -> i32 {
    fputc(c, fp)
}
pub unsafe extern "win64" fn putchar(c: i32) -> i32 {
    fputc(c, __acrt_iob_func(1))
}

pub unsafe extern "win64" fn fputs(s: *const u8, fp: *mut FILE) -> i32 {
    if s.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    let d = crate::strings::cstr_slice(s);
    if write_stream_bytes(fp as *mut c_void, d) < 0 {
        -1
    } else {
        0
    }
}
pub unsafe extern "win64" fn puts(s: *const u8) -> i32 {
    if s.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    let mut d = crate::strings::cstr_slice(s).to_vec();
    d.push(b'\n');
    if write_stream_bytes(__acrt_iob_func(1) as *mut c_void, &d) < 0 {
        -1
    } else {
        0
    }
}

pub unsafe extern "win64" fn fgets(buf: *mut u8, n: i32, fp: *mut FILE) -> *mut u8 {
    if buf.is_null() || n <= 0 {
        errno::set(EINVAL);
        return std::ptr::null_mut();
    }
    let _l = Locked::new();
    let r = with_stream(fp as *mut c_void, |s| {
        let mut i = 0usize;
        while (i as i32) < n - 1 {
            match s.getc() {
                Some(b) => {
                    *buf.add(i) = b;
                    i += 1;
                    if b == b'\n' {
                        break;
                    }
                }
                None => break,
            }
        }
        *buf.add(i) = 0;
        i
    });
    match r {
        Some(0) | None => std::ptr::null_mut(),
        Some(_) => buf,
    }
}

pub unsafe extern "win64" fn ungetc(c: i32, fp: *mut FILE) -> i32 {
    if c == -1 {
        return -1;
    }
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| {
        s.unget.push(c as u8);
        s.eof = false;
        c & 0xFF
    })
    .unwrap_or(-1)
}
pub unsafe extern "win64" fn _ungetc_nolock(c: i32, fp: *mut FILE) -> i32 {
    ungetc(c, fp)
}

const WEOF: u32 = 0xFFFF;

pub unsafe extern "win64" fn fgetwc(fp: *mut FILE) -> u32 {
    let _l = Locked::new();
    let r = with_stream(fp as *mut c_void, |s| {
        let b0 = s.getc()?;
        if b0 < 0x80 || super::locale::c_locale_active() {
            return Some(b0 as u32);
        }
        // UTF-8 multibyte (our locale's codepage is 65001).
        let need = if b0 >= 0xF0 {
            3
        } else if b0 >= 0xE0 {
            2
        } else {
            1
        };
        let mut v = vec![b0];
        for _ in 0..need {
            v.push(s.getc()?);
        }
        let st = String::from_utf8_lossy(&v);
        st.encode_utf16().next().map(|c| c as u32)
    });
    r.flatten().unwrap_or(WEOF)
}
pub unsafe extern "win64" fn getwc(fp: *mut FILE) -> u32 {
    fgetwc(fp)
}
pub unsafe extern "win64" fn _fgetwc_nolock(fp: *mut FILE) -> u32 {
    fgetwc(fp)
}

pub unsafe extern "win64" fn fputwc(c: u16, fp: *mut FILE) -> u32 {
    let bytes: Vec<u8> = if c < 0x80 || super::locale::c_locale_active() && c < 0x100 {
        vec![c as u8]
    } else {
        String::from_utf16_lossy(&[c]).into_bytes()
    };
    if write_stream_bytes(fp as *mut c_void, &bytes) < 0 {
        WEOF
    } else {
        c as u32
    }
}
pub unsafe extern "win64" fn putwc(c: u16, fp: *mut FILE) -> u32 {
    fputwc(c, fp)
}
pub unsafe extern "win64" fn _fputwc_nolock(c: u16, fp: *mut FILE) -> u32 {
    fputwc(c, fp)
}

pub unsafe extern "win64" fn fputws(s: *const u16, fp: *mut FILE) -> i32 {
    let st = from_wide(s);
    if write_stream_bytes(fp as *mut c_void, st.as_bytes()) < 0 {
        -1
    } else {
        0
    }
}

pub unsafe extern "win64" fn ungetwc(c: u32, fp: *mut FILE) -> u32 {
    if c == WEOF {
        return WEOF;
    }
    let bytes: Vec<u8> = if c < 0x80 { vec![c as u8] } else { String::from_utf16_lossy(&[c as u16]).into_bytes() };
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| {
        for b in bytes.iter().rev() {
            s.unget.push(*b);
        }
        s.eof = false;
        c
    })
    .unwrap_or(WEOF)
}

pub unsafe extern "win64" fn fgetws(buf: *mut u16, n: i32, fp: *mut FILE) -> *mut u16 {
    if buf.is_null() || n <= 0 {
        return std::ptr::null_mut();
    }
    let mut i = 0usize;
    while (i as i32) < n - 1 {
        let c = fgetwc(fp);
        if c == WEOF {
            break;
        }
        *buf.add(i) = c as u16;
        i += 1;
        if c == '\n' as u32 {
            break;
        }
    }
    *buf.add(i) = 0;
    if i == 0 {
        std::ptr::null_mut()
    } else {
        buf
    }
}

pub unsafe extern "win64" fn feof(fp: *mut FILE) -> i32 {
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| (s.eof && s.unget.is_empty() && s.rpos >= s.rbuf.len()) as i32).unwrap_or(0)
}
pub unsafe extern "win64" fn ferror(fp: *mut FILE) -> i32 {
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| s.err as i32).unwrap_or(0)
}
pub unsafe extern "win64" fn clearerr(fp: *mut FILE) {
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| {
        s.err = false;
        s.eof = false;
    });
}
pub unsafe extern "win64" fn clearerr_s(fp: *mut FILE) -> i32 {
    clearerr(fp);
    0
}

pub unsafe extern "win64" fn _fileno(fp: *mut FILE) -> i32 {
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| s.fd).unwrap_or_else(|| {
        errno::set(EINVAL);
        -1
    })
}

pub unsafe extern "win64" fn _fseeki64(fp: *mut FILE, off: i64, whence: i32) -> i32 {
    if !(0..=2).contains(&whence) {
        errno::set(EINVAL);
        return -1;
    }
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| s.seek(off, whence)).unwrap_or(-1)
}
pub unsafe extern "win64" fn fseek(fp: *mut FILE, off: i32, whence: i32) -> i32 {
    _fseeki64(fp, off as i64, whence)
}
pub unsafe extern "win64" fn _fseek_nolock(fp: *mut FILE, off: i32, whence: i32) -> i32 {
    fseek(fp, off, whence)
}
pub unsafe extern "win64" fn _fseeki64_nolock(fp: *mut FILE, off: i64, whence: i32) -> i32 {
    _fseeki64(fp, off, whence)
}
pub unsafe extern "win64" fn _ftelli64(fp: *mut FILE) -> i64 {
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| s.tell()).unwrap_or(-1)
}
pub unsafe extern "win64" fn ftell(fp: *mut FILE) -> i32 {
    let r = _ftelli64(fp);
    if r > i32::MAX as i64 {
        errno::set(EINVAL);
        return -1;
    }
    r as i32
}
pub unsafe extern "win64" fn _ftell_nolock(fp: *mut FILE) -> i32 {
    ftell(fp)
}
pub unsafe extern "win64" fn _ftelli64_nolock(fp: *mut FILE) -> i64 {
    _ftelli64(fp)
}
pub unsafe extern "win64" fn rewind(fp: *mut FILE) {
    _fseeki64(fp, 0, 0);
    clearerr(fp);
}
pub unsafe extern "win64" fn fgetpos(fp: *mut FILE, pos: *mut i64) -> i32 {
    let p = _ftelli64(fp);
    if p < 0 {
        return -1;
    }
    *pos = p;
    0
}
pub unsafe extern "win64" fn fsetpos(fp: *mut FILE, pos: *const i64) -> i32 {
    _fseeki64(fp, *pos, 0)
}

pub unsafe extern "win64" fn setvbuf(fp: *mut FILE, _buf: *mut u8, mode: i32, size: usize) -> i32 {
    // _IOFBF 0, _IOLBF 0x40, _IONBF 4. The CRT treats _IOLBF as _IOFBF.
    let m = match mode {
        0 | 0x40 => BufMode::Full,
        4 => BufMode::None,
        _ => {
            errno::set(EINVAL);
            super::runtime::invalid_parameter("setvbuf: mode");
            return -1;
        }
    };
    let _l = Locked::new();
    with_stream(fp as *mut c_void, |s| {
        s.flush();
        s.mode = m;
        if size >= 2 {
            s.bufsize = size.min(1 << 20);
        }
        0
    })
    .unwrap_or(-1)
}
pub unsafe extern "win64" fn setbuf(fp: *mut FILE, buf: *mut u8) {
    if buf.is_null() {
        setvbuf(fp, buf, 4, 0);
    } else {
        setvbuf(fp, buf, 0, 512);
    }
}

pub unsafe extern "win64" fn _lock_file(_fp: *mut FILE) {
    STDIO_LOCK.acquire();
}
pub unsafe extern "win64" fn _unlock_file(_fp: *mut FILE) {
    STDIO_LOCK.release();
}

/// `_get_stream_buffer_pointers(FILE*, char*** base, char*** ptr, int** cnt)`
pub unsafe extern "win64" fn _get_stream_buffer_pointers(
    fp: *mut FILE,
    base: *mut *mut *mut u8,
    ptr: *mut *mut *mut u8,
    cnt: *mut *mut i32,
) -> i32 {
    if fp.is_null() {
        errno::set(EINVAL);
        return EINVAL;
    }
    if !base.is_null() {
        *base = &mut (*fp)._base;
    }
    if !ptr.is_null() {
        *ptr = &mut (*fp)._ptr;
    }
    if !cnt.is_null() {
        *cnt = &mut (*fp)._cnt;
    }
    0
}

pub unsafe extern "win64" fn _getmaxstdio() -> i32 {
    8192
}
pub unsafe extern "win64" fn _setmaxstdio(n: i32) -> i32 {
    n
}

pub unsafe extern "win64" fn tmpfile() -> *mut FILE {
    let dir = std::env::temp_dir().join(format!("aw2tmp-{}-{}", std::process::id(), libc::arc4random()));
    let c = std::ffi::CString::new(dir.as_os_str().as_encoded_bytes()).unwrap();
    let fd = libc::open(c.as_ptr(), libc::O_RDWR | libc::O_CREAT | libc::O_EXCL | libc::O_CLOEXEC, 0o600);
    if fd < 0 {
        errno::set_from_host();
        return std::ptr::null_mut();
    }
    libc::unlink(c.as_ptr());
    set_fd_info(fd, FdInfo { text: false, raw: false, temporary: false, osf_handle: None }, None);
    let _l = Locked::new();
    new_stream(fd, &ModeFlags { oflag: O_RDWR | O_BINARY, readable: true, writable: true })
}

pub unsafe extern "win64" fn perror(s: *const u8) {
    let msg = super::errno::message(errno::get());
    let line = if s.is_null() || *s == 0 { format!("{msg}\n") } else { format!("{}: {msg}\n", from_ansi(s)) };
    write_stream_bytes(__acrt_iob_func(2) as *mut c_void, line.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_roundtrip_text_mode() {
        let dir = std::env::temp_dir().join("aw2_stdio_test");
        std::fs::create_dir_all(&dir).unwrap();
        crate::path::add_mount("T:\\", &dir);
        unsafe {
            let f = fopen(b"T:\\a.txt\0".as_ptr(), b"w\0".as_ptr());
            assert!(!f.is_null());
            assert_eq!(fwrite(b"one\ntwo\n".as_ptr() as *const c_void, 1, 8, f), 8);
            assert_eq!(fclose(f), 0);
            assert_eq!(std::fs::read(dir.join("a.txt")).unwrap(), b"one\r\ntwo\r\n");
            let f = fopen(b"T:\\a.txt\0".as_ptr(), b"r\0".as_ptr());
            let mut buf = [0u8; 32];
            assert!(!fgets(buf.as_mut_ptr(), 32, f).is_null());
            assert_eq!(&buf[..5], b"one\n\0");
            assert_eq!(fgetc(f), b't' as i32);
            assert_eq!(ungetc(b'T' as i32, f), b'T' as i32);
            assert_eq!(fread(buf.as_mut_ptr() as *mut c_void, 1, 32, f), 4);
            assert_eq!(&buf[..4], b"Two\n");
            assert_ne!(feof(f), 0);
            fclose(f);
            let f = fopen(b"T:\\b.bin\0".as_ptr(), b"w+b\0".as_ptr());
            fwrite(b"abc\n".as_ptr() as *const c_void, 1, 4, f);
            assert_eq!(ftell(f), 4);
            assert_eq!(fseek(f, 1, 0), 0);
            assert_eq!(fgetc(f), b'b' as i32);
            fclose(f);
            assert_eq!(std::fs::read(dir.join("b.bin")).unwrap(), b"abc\n");
            assert!(fopen(b"T:\\missing\\x\0".as_ptr(), b"r\0".as_ptr()).is_null());
            assert_eq!(errno::get(), ENOENT);
        }
    }

    #[test]
    fn osfhandle_open_get_and_ownership() {
        use crate::kernel32::file::{CreateFileW, ReadFile, WriteFile};
        use crate::types::{HANDLE, INVALID_HANDLE_VALUE, TRUE};

        let dir = std::env::temp_dir().join(format!("aw2_osf_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        crate::path::add_mount("O:\\", &dir);
        unsafe {
            // Invalid handles / fds fail with EBADF and -1.
            assert_eq!(_open_osfhandle(-1, O_BINARY), -1);
            assert_eq!(errno::get(), EBADF);
            assert_eq!(_get_osfhandle(-1), -1);
            assert_eq!(errno::get(), EBADF);
            assert_eq!(_get_osfhandle(9999), -1);
            assert_eq!(errno::get(), EBADF);

            // Create a Win32 file handle, transfer ownership to CRT via _open_osfhandle.
            let p = crate::strings::to_wide_z("O:\\osf.bin");
            let h = CreateFileW(
                p.as_ptr(),
                0xC000_0000, // GENERIC_READ | GENERIC_WRITE
                3,
                std::ptr::null_mut(),
                2, // CREATE_ALWAYS
                0x80,
                std::ptr::null_mut(),
            );
            assert_ne!(h, INVALID_HANDLE_VALUE);
            let fd = _open_osfhandle(h as isize, O_BINARY);
            assert!(fd >= 0);
            // _get_osfhandle returns the exact underlying Win32 handle.
            assert_eq!(_get_osfhandle(fd), h as isize);

            // Write via CRT _write, read back via Win32 ReadFile after lseek!
            assert_eq!(_write(fd, b"hello-osf".as_ptr() as *const c_void, 9), 9);
            assert_eq!(_lseeki64(fd, 0, 0), 0);
            let mut rbuf = [0u8; 16];
            let mut got = 0u32;
            assert_eq!(ReadFile(h, rbuf.as_mut_ptr() as *mut c_void, 9, &mut got, std::ptr::null_mut()), TRUE);
            assert_eq!(got, 9);
            assert_eq!(&rbuf[..9], b"hello-osf");

            // Closing the CRT fd closes the owned Win32 HANDLE cleanly once.
            assert_eq!(_close(fd), 0);
            assert!(crate::handles::get(h).is_none());
            assert_eq!(_get_osfhandle(fd), -1);

            // Reverse direction: open via CRT _open, wrap lazily via _get_osfhandle, write via WriteFile, close via _close.
            let fd2 = _open(b"O:\\crt.bin\0".as_ptr(), O_RDWR | O_CREAT | O_TRUNC | O_BINARY, 0x180);
            assert!(fd2 >= 0);
            let h2 = _get_osfhandle(fd2) as HANDLE;
            assert_ne!(h2, INVALID_HANDLE_VALUE);
            assert_eq!(_get_osfhandle(fd2), h2 as isize); // cached
            let mut written = 0u32;
            assert_eq!(WriteFile(h2, b"from-win32".as_ptr() as *const c_void, 10, &mut written, std::ptr::null_mut()), TRUE);
            assert_eq!(written, 10);
            assert_eq!(_close(fd2), 0);
            assert!(crate::handles::get(h2).is_none());
            assert_eq!(std::fs::read(dir.join("crt.bin")).unwrap(), b"from-win32");

            // Standard descriptors return valid std handles.
            assert_ne!(_get_osfhandle(1), -1);
            let _ = std::fs::remove_dir_all(&dir);
        }
    }

    #[test]
    fn dup2_and_close_preserve_osfhandle_ownership_and_duplicate_handles() {
        use crate::kernel32::file::{CreateFileW, ReadFile, SetFilePointerEx, WriteFile};
        use crate::kernel32::threads::{CloseHandle, DuplicateHandle};
        use crate::types::{HANDLE, INVALID_HANDLE_VALUE, TRUE};

        let dir = std::env::temp_dir().join(format!("aw2_dup2_osf_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        crate::path::add_mount("D:\\", &dir);
        std::fs::write(dir.join("old1.bin"), b"OLD-ONE").unwrap();
        std::fs::write(dir.join("old2.bin"), b"OLD-TWO").unwrap();
        std::fs::write(dir.join("old3.bin"), b"OLD-THREE").unwrap();
        std::fs::write(dir.join("new.bin"), b"NEW-DATA").unwrap();
        std::fs::write(dir.join("crt_close.bin"), b"CRT-ORIG").unwrap();
        std::fs::write(dir.join("crt_dup2.bin"), b"CRT-DUP2").unwrap();
        std::fs::write(dir.join("crt_other.bin"), b"OTHER-FD").unwrap();

        unsafe {
            let open_win = |win_path: &str| -> HANDLE {
                let p = crate::strings::to_wide_z(win_path);
                let h = CreateFileW(
                    p.as_ptr(),
                    0xC000_0000,
                    3,
                    std::ptr::null_mut(),
                    3, // OPEN_EXISTING
                    0x80,
                    std::ptr::null_mut(),
                );
                assert_ne!(h, INVALID_HANDLE_VALUE);
                h
            };

            let read_all_crt = |fd: i32| -> Vec<u8> {
                assert_eq!(_lseeki64(fd, 0, 0), 0);
                let mut buf = [0u8; 32];
                let n = _read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() as u32);
                assert!(n >= 0, "_read({fd}) failed with errno {}", errno::get());
                buf[..n as usize].to_vec()
            };

            let read_win_from_start = |h: HANDLE| -> Vec<u8> {
                assert_eq!(SetFilePointerEx(h, 0, std::ptr::null_mut(), 0), TRUE);
                let mut buf = [0u8; 32];
                let mut got = 0u32;
                assert_eq!(
                    ReadFile(h, buf.as_mut_ptr() as *mut c_void, buf.len() as u32, &mut got, std::ptr::null_mut()),
                    TRUE
                );
                buf[..got as usize].to_vec()
            };

            let dup_win = |src: HANDLE| -> HANDLE {
                let mut dst: HANDLE = std::ptr::null_mut();
                assert_eq!(
                    DuplicateHandle(
                        crate::handles::CURRENT_PROCESS,
                        src,
                        crate::handles::CURRENT_PROCESS,
                        &mut dst,
                        0,
                        0,
                        2, // DUPLICATE_SAME_ACCESS
                    ),
                    TRUE
                );
                assert!(!dst.is_null() && dst != src);
                dst
            };

            // 1. Self-dup on invalid fd fails with EBADF; self-dup on _open_osfhandle fd is a
            //    validated no-op preserving both the descriptor and its Win32 handle association.
            assert_eq!(_dup2(-1, -1), -1);
            assert_eq!(errno::get(), EBADF);

            let h1 = open_win("D:\\old1.bin");
            let fd1 = _open_osfhandle(h1 as isize, O_BINARY);
            assert!(fd1 >= 0);
            assert_eq!(_dup2(fd1, fd1), 0);
            assert_eq!(_get_osfhandle(fd1), h1 as isize);
            assert!(crate::handles::get(h1).is_some());
            assert_eq!(read_all_crt(fd1), b"OLD-ONE");

            // 2. Failed _dup2 (invalid source or invalid dest) preserves valid target & handle.
            assert_eq!(_dup2(-1, fd1), -1);
            assert_eq!(errno::get(), EBADF);
            assert_eq!(_dup2(9999, fd1), -1);
            assert_eq!(errno::get(), EBADF);
            assert_eq!(_dup2(fd1, -1), -1);
            assert_eq!(errno::get(), EBADF);
            assert_eq!(_get_osfhandle(fd1), h1 as isize);
            assert!(crate::handles::get(h1).is_some());
            assert_eq!(read_all_crt(fd1), b"OLD-ONE");

            // 3. Replacing an _open_osfhandle target retires old Win32 handle without closing
            //    the newly duplicated descriptor, and closing later does not double-close.
            let fd_src = _open(b"D:\\new.bin\0".as_ptr(), O_RDWR | O_BINARY, 0x180);
            assert!(fd_src >= 0);
            assert_eq!(_dup2(fd_src, fd1), 0);
            assert!(crate::handles::get(h1).is_none(), "old owning handle must be retired");
            assert_eq!(read_all_crt(fd1), b"NEW-DATA");
            let h1_new = _get_osfhandle(fd1);
            assert_ne!(h1_new, -1);
            assert_ne!(h1_new, h1 as isize);
            assert_eq!(_close(fd1), 0);
            assert_eq!(_close(fd1), -1);
            assert_eq!(errno::get(), EBADF);

            // 4. Replacing an _open_osfhandle target when DuplicateHandle retains another handle
            //    to the same FileObj: old CRT handle retires, retained duplicate handle stays live
            //    on the original file, and closing the retained duplicate handle later does NOT
            //    close the newly duplicated CRT descriptor!
            let h2 = open_win("D:\\old2.bin");
            let h2_dup = dup_win(h2);
            let fd2 = _open_osfhandle(h2 as isize, O_BINARY);
            assert!(fd2 >= 0);
            assert_eq!(_dup2(fd_src, fd2), 0);
            assert!(crate::handles::get(h2).is_none());
            assert!(crate::handles::get(h2_dup).is_some());
            assert_eq!(read_win_from_start(h2_dup), b"OLD-TWO");
            assert_eq!(read_all_crt(fd2), b"NEW-DATA");

            // Dropping the last Arc<FileObj> via CloseHandle(h2_dup) must NOT affect fd2!
            assert_eq!(CloseHandle(h2_dup), TRUE);
            assert!(crate::handles::get(h2_dup).is_none());
            assert_eq!(read_all_crt(fd2), b"NEW-DATA");
            assert_eq!(_close(fd2), 0);
            assert_eq!(_close(fd_src), 0);

            // 5. _close on an _open_osfhandle fd with a retained DuplicateHandle closes the CRT
            //    fd immediately while leaving the retained Win32 duplicate handle usable.
            let h3 = open_win("D:\\old3.bin");
            let h3_dup = dup_win(h3);
            let fd3 = _open_osfhandle(h3 as isize, O_BINARY);
            assert!(fd3 >= 0);
            assert_eq!(_close(fd3), 0);
            assert!(crate::handles::get(h3).is_none());
            assert_eq!(_get_osfhandle(fd3), -1);
            assert_eq!(errno::get(), EBADF);
            assert_eq!(_close(fd3), -1);
            assert_eq!(errno::get(), EBADF);
            // Retained duplicate handle still reads the original file cleanly!
            assert_eq!(read_win_from_start(h3_dup), b"OLD-THREE");
            assert_eq!(CloseHandle(h3_dup), TRUE);

            // 6. Genuinely CRT-opened ordinary file (`_open`) -> `_get_osfhandle` ->
            //    `DuplicateHandle` -> `_close(fd)`:
            //    - Exported handle owns a distinct CLOEXEC POSIX duplicate sharing the open file
            //      description (shared file position before `_close`).
            //    - After `_close(fd)`, the cached handle retires and the CRT fd is closed, while
            //      the surviving sibling handle still reads and writes the original file.
            let fd4 = _open(b"D:\\crt_close.bin\0".as_ptr(), O_RDWR | O_BINARY, 0x180);
            assert!(fd4 >= 0);
            let h4 = _get_osfhandle(fd4) as HANDLE;
            assert_ne!(h4, INVALID_HANDLE_VALUE);
            assert_eq!(_get_osfhandle(fd4), h4 as isize); // cached
            let h4_obj = crate::kernel32::file::get_file(h4).expect("cached FileObj");
            assert_ne!(h4_obj.fd(), fd4, "_get_osfhandle must own a distinct duplicated POSIX fd");
            assert_ne!(libc::fcntl(h4_obj.fd(), libc::F_GETFD) & libc::FD_CLOEXEC, 0);
            drop(h4_obj);

            let h4_dup = dup_win(h4);
            // Shared open file description: seeking/reading via CRT fd moves h4_dup's position
            // and vice versa.
            let mut prefix = [0u8; 4];
            assert_eq!(_read(fd4, prefix.as_mut_ptr() as *mut c_void, 4), 4);
            assert_eq!(&prefix, b"CRT-");
            let mut pos = -1i64;
            assert_eq!(SetFilePointerEx(h4_dup, 0, &mut pos, 1), TRUE); // FILE_CURRENT
            assert_eq!(pos, 4);
            let mut rest = [0u8; 4];
            let mut got = 0u32;
            assert_eq!(
                ReadFile(h4_dup, rest.as_mut_ptr() as *mut c_void, 4, &mut got, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(got, 4);
            assert_eq!(&rest, b"ORIG");
            assert_eq!(_telli64(fd4), 8);

            // Closing the CRT fd retires h4 and closes fd4, while h4_dup still reads and writes!
            assert_eq!(_close(fd4), 0);
            assert!(crate::handles::get(h4).is_none());
            assert_eq!(_get_osfhandle(fd4), -1);
            assert_eq!(errno::get(), EBADF);
            assert_eq!(read_win_from_start(h4_dup), b"CRT-ORIG");
            let mut written = 0u32;
            assert_eq!(
                WriteFile(h4_dup, b"+ALIVE".as_ptr() as *const c_void, 6, &mut written, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(written, 6);
            assert_eq!(read_win_from_start(h4_dup), b"CRT-ORIG+ALIVE");
            assert_eq!(CloseHandle(h4_dup), TRUE);
            assert!(crate::handles::get(h4_dup).is_none());
            assert_eq!(std::fs::read(dir.join("crt_close.bin")).unwrap(), b"CRT-ORIG+ALIVE");

            // 7. Genuinely CRT-opened ordinary file (`_open`) -> `_get_osfhandle` ->
            //    `DuplicateHandle` -> `_dup2(other_fd, fd)`:
            //    - Shared file position before `_dup2`.
            //    - After `_dup2`, the surviving sibling handle still reads and writes the original
            //      file while `fd` points to `other_fd`.
            //    - After `CloseHandle(sibling)` drops the last `Arc<FileObj>`, destination `fd`
            //      and its new `_get_osfhandle(fd)` still read and write cleanly!
            let fd5 = _open(b"D:\\crt_dup2.bin\0".as_ptr(), O_RDWR | O_BINARY, 0x180);
            let fd_other = _open(b"D:\\crt_other.bin\0".as_ptr(), O_RDWR | O_BINARY, 0x180);
            assert!(fd5 >= 0 && fd_other >= 0);
            let h5 = _get_osfhandle(fd5) as HANDLE;
            assert_ne!(h5, INVALID_HANDLE_VALUE);
            let h5_dup = dup_win(h5);

            // Verify shared file position before `_dup2`.
            assert_eq!(_lseeki64(fd5, 4, 0), 4);
            let mut pos5 = -1i64;
            assert_eq!(SetFilePointerEx(h5_dup, 0, &mut pos5, 1), TRUE);
            assert_eq!(pos5, 4);

            assert_eq!(_dup2(fd_other, fd5), 0);
            assert!(crate::handles::get(h5).is_none(), "cached _get_osfhandle must retire on _dup2");
            assert!(crate::handles::get(h5_dup).is_some());

            // Sibling still reads and writes the original file after `_dup2`.
            assert_eq!(read_win_from_start(h5_dup), b"CRT-DUP2");
            let mut w5 = 0u32;
            assert_eq!(
                WriteFile(h5_dup, b"!OK".as_ptr() as *const c_void, 3, &mut w5, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(w5, 3);
            assert_eq!(read_win_from_start(h5_dup), b"CRT-DUP2!OK");

            // Destination fd now refers to `crt_other.bin`.
            assert_eq!(read_all_crt(fd5), b"OTHER-FD");

            // Closing the surviving sibling must NOT disturb destination `fd5` or its new handle!
            assert_eq!(CloseHandle(h5_dup), TRUE);
            assert!(crate::handles::get(h5_dup).is_none());
            assert_eq!(read_all_crt(fd5), b"OTHER-FD");

            let h5_new = _get_osfhandle(fd5) as HANDLE;
            assert_ne!(h5_new, INVALID_HANDLE_VALUE);
            assert_ne!(h5_new, h5);
            assert_eq!(_get_osfhandle(fd5), h5_new as isize);
            assert_eq!(read_win_from_start(h5_new), b"OTHER-FD");
            let mut w_other = 0u32;
            assert_eq!(
                WriteFile(h5_new, b"+NEW".as_ptr() as *const c_void, 4, &mut w_other, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(w_other, 4);
            assert_eq!(read_all_crt(fd5), b"OTHER-FD+NEW");

            assert_eq!(_close(fd5), 0);
            assert_eq!(_close(fd_other), 0);
            assert_eq!(std::fs::read(dir.join("crt_dup2.bin")).unwrap(), b"CRT-DUP2!OK");
            assert_eq!(std::fs::read(dir.join("crt_other.bin")).unwrap(), b"OTHER-FD+NEW");
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn crt_read_write_block_on_nonblocking_duplex_named_pipe_after_overlapped_write() {
        use crate::kernel32::file::{
            CreateFileW, CreateNamedPipeW, DisconnectNamedPipe, GetOverlappedResult, ReadFile,
            WriteFile, OVERLAPPED,
        };
        use crate::kernel32::threads::CloseHandle;
        use crate::types::{err, INVALID_HANDLE_VALUE, TRUE};
        use std::sync::mpsc;
        use std::time::{Duration, Instant};

        unsafe {
            let pname = crate::strings::to_wide_z(&format!(
                "\\\\.\\pipe\\aw2_crt_nb_pipe_test_{}",
                std::process::id()
            ));
            let srv = CreateNamedPipeW(
                pname.as_ptr(),
                0x4000_0003, // PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED
                0,
                1,
                4096,
                4096,
                0,
                std::ptr::null_mut(),
            );
            assert_ne!(srv, INVALID_HANDLE_VALUE);

            let cli = CreateFileW(
                pname.as_ptr(),
                0xC000_0000, // GENERIC_READ | GENERIC_WRITE
                0,
                std::ptr::null_mut(),
                3,           // OPEN_EXISTING
                0x4000_0000, // FILE_FLAG_OVERLAPPED
                std::ptr::null_mut(),
            );
            assert_ne!(cli, INVALID_HANDLE_VALUE);

            let cli_fd = _open_osfhandle(cli as isize, O_BINARY);
            assert!(cli_fd >= 0);

            // A 1-byte overlapped WriteFile permanently enables O_NONBLOCK on the shared
            // open file description (visible through cli_fd via F_DUPFD_CLOEXEC).
            let mut ov: OVERLAPPED = std::mem::zeroed();
            let mut nw = 0u32;
            let ok = WriteFile(cli, b"X".as_ptr() as *const c_void, 1, &mut nw, &mut ov);
            if ok == 0 {
                assert_eq!(crate::teb::last_error(), err::ERROR_IO_PENDING);
                assert_eq!(GetOverlappedResult(cli, &mut ov, &mut nw, TRUE), TRUE);
            }
            assert_eq!(nw, 1);

            let mut one = [0u8; 1];
            let mut nr = 0u32;
            assert_eq!(ReadFile(srv, one.as_mut_ptr() as *mut c_void, 1, &mut nr, std::ptr::null_mut()), TRUE);
            assert_eq!(nr, 1);
            assert_eq!(&one, b"X");
            assert_ne!(
                libc::fcntl(cli_fd, libc::F_GETFL) & libc::O_NONBLOCK,
                0,
                "expected shared O_NONBLOCK after overlapped pipe write"
            );

            // 1. CRT _read on an empty nonblocking pipe must wait for data (not fail with EAGAIN)
            //    and preserve ordinary short read counts.
            let (r_tx, r_rx) = mpsc::channel();
            let reader = std::thread::spawn(move || {
                let mut buf = [0u8; 32];
                let n = _read(cli_fd, buf.as_mut_ptr() as *mut c_void, buf.len() as u32);
                let e = errno::get();
                let _ = r_tx.send((n, e, buf));
            });
            std::thread::sleep(Duration::from_millis(25));
            assert!(
                matches!(r_rx.try_recv(), Err(mpsc::TryRecvError::Empty)),
                "_read on empty pipe returned prematurely instead of waiting"
            );
            let msg = b"hello-pipe";
            assert_eq!(
                WriteFile(srv, msg.as_ptr() as *const c_void, msg.len() as u32, &mut nw, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(nw as usize, msg.len());
            let (n, e, buf) = r_rx
                .recv_timeout(Duration::from_secs(5))
                .expect("_read on pipe hung after data arrived");
            reader.join().unwrap();
            assert_eq!(n as usize, msg.len(), "_read failed with errno={e}");
            assert_eq!(&buf[..n as usize], msg);

            // 2. Backpressured CRT _write must wait and resume after partial writes, delivering
            //    all 256 KiB intact and in exact order while keeping shared O_NONBLOCK intact.
            //    Shrink SO_SNDBUF on the socketpair client fd to guarantee backpressure
            //    independent of host socket defaults.
            let small_sndbuf: libc::c_int = 4096;
            assert_eq!(
                libc::setsockopt(
                    cli_fd,
                    libc::SOL_SOCKET,
                    libc::SO_SNDBUF,
                    &small_sndbuf as *const libc::c_int as *const c_void,
                    std::mem::size_of::<libc::c_int>() as libc::socklen_t,
                ),
                0
            );
            let (srv_fd, _) = crate::kernel32::file::file_fd_and_kind(srv)
                .expect("connected named pipe server must have a live POSIX fd");

            let payload: Vec<u8> = (0..256 * 1024usize)
                .map(|i| ((i.wrapping_mul(31).wrapping_add(7)) & 0xFF) as u8)
                .collect();
            let payload_send = payload.clone();
            let (w_tx, w_rx) = mpsc::channel();
            let writer = std::thread::spawn(move || {
                let n = _write(
                    cli_fd,
                    payload_send.as_ptr() as *const c_void,
                    payload_send.len() as u32,
                );
                let e = errno::get();
                let _ = w_tx.send((n, e));
            });
            std::thread::sleep(Duration::from_millis(25));
            assert!(
                matches!(w_rx.try_recv(), Err(mpsc::TryRecvError::Empty)),
                "backpressured _write returned prematurely instead of waiting"
            );

            let mut received = Vec::with_capacity(payload.len());
            let mut chunk = [0u8; 8192];
            let deadline = Instant::now() + Duration::from_secs(5);
            while received.len() < payload.len() {
                let mut pfd = libc::pollfd {
                    fd: srv_fd,
                    events: libc::POLLIN,
                    revents: 0,
                };
                let pr = loop {
                    let rem = deadline.saturating_duration_since(Instant::now());
                    assert!(
                        !rem.is_zero(),
                        "timed out draining backpressured _write ({}/{} bytes received; writer={:?})",
                        received.len(),
                        payload.len(),
                        w_rx.try_recv()
                    );
                    let timeout_ms = rem.as_millis().clamp(1, i32::MAX as u128) as libc::c_int;
                    let rc = libc::poll(&mut pfd, 1, timeout_ms);
                    if rc < 0 && *libc::__error() == libc::EINTR {
                        continue;
                    }
                    break rc;
                };
                assert!(
                    pr > 0,
                    "poll(srv_fd) failed or timed out (rc={pr}, errno={}, {}/{} bytes, writer={:?})",
                    *libc::__error(),
                    received.len(),
                    payload.len(),
                    w_rx.try_recv()
                );
                assert!(
                    (pfd.revents & libc::POLLIN) != 0,
                    "pipe peer closed/errored before full payload (revents={:#x}, {}/{} bytes, writer={:?})",
                    pfd.revents,
                    received.len(),
                    payload.len(),
                    w_rx.try_recv()
                );
                let want = (payload.len() - received.len()).min(chunk.len()) as u32;
                let mut got = 0u32;
                let rok = ReadFile(
                    srv,
                    chunk.as_mut_ptr() as *mut c_void,
                    want,
                    &mut got,
                    std::ptr::null_mut(),
                );
                assert_eq!(
                    rok,
                    TRUE,
                    "ReadFile(srv) failed with {} ({}/{} bytes, writer={:?})",
                    crate::teb::last_error(),
                    received.len(),
                    payload.len(),
                    w_rx.try_recv()
                );
                assert!(got > 0, "unexpected 0-byte ReadFile before full payload");
                received.extend_from_slice(&chunk[..got as usize]);
            }
            let (wn, we) = w_rx
                .recv_timeout(Duration::from_secs(5))
                .expect("backpressured _write hung while peer drained");
            writer.join().unwrap();
            assert_eq!(wn as usize, payload.len(), "_write failed with errno={we}");
            assert_eq!(received, payload);
            assert_ne!(
                libc::fcntl(cli_fd, libc::F_GETFL) & libc::O_NONBLOCK,
                0,
                "CRT retry path must not toggle shared O_NONBLOCK off"
            );

            // 3. Peer disconnect/close terminates waiting CRT _read with 0 (EOF) and causes
            //    subsequent CRT _write to fail cleanly with EPIPE.
            let (eof_tx, eof_rx) = mpsc::channel();
            let eof_reader = std::thread::spawn(move || {
                let mut buf = [0u8; 16];
                let n = _read(cli_fd, buf.as_mut_ptr() as *mut c_void, buf.len() as u32);
                let e = errno::get();
                let _ = eof_tx.send((n, e));
            });
            std::thread::sleep(Duration::from_millis(25));
            assert!(matches!(eof_rx.try_recv(), Err(mpsc::TryRecvError::Empty)));
            assert_eq!(DisconnectNamedPipe(srv), TRUE);
            assert_eq!(CloseHandle(srv), TRUE);

            let (eof_n, eof_e) = eof_rx
                .recv_timeout(Duration::from_secs(5))
                .expect("_read on pipe hung after peer closed");
            eof_reader.join().unwrap();
            assert_eq!(eof_n, 0, "expected EOF (0), got {eof_n} errno={eof_e}");

            assert_eq!(_write(cli_fd, b"after-eof".as_ptr() as *const c_void, 9), -1);
            assert_eq!(errno::get(), EPIPE);

            assert_eq!(_close(cli_fd), 0);
            assert!(crate::handles::get(cli).is_none());
        }
    }
}
