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

use super::errno::{self, EACCES, EBADF, EILSEQ, EINVAL, ENOENT, EPIPE};
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TextMode {
    Binary,
    AnsiText,
    Utf16Le,
    Utf8,
}

impl TextMode {
    fn is_unicode(self) -> bool {
        matches!(self, TextMode::Utf16Le | TextMode::Utf8)
    }
}

#[derive(Clone, Copy)]
struct FdInfo {
    mode: TextMode,
    /// Original Windows CRT mode constant (`O_BINARY`, `O_TEXT`, `O_WTEXT`, `O_U16TEXT`, `O_U8TEXT`).
    win_mode: i32,
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

#[derive(Default)]
struct U8Pending {
    raw: Vec<u8>,
    low_surrogate: Option<u16>,
}

static FDS: Mutex<Option<HashMap<i32, (FdInfo, Option<std::path::PathBuf>)>>> = Mutex::new(None);
static FD_U8_PENDING: Mutex<Option<HashMap<i32, U8Pending>>> = Mutex::new(None);

fn clear_u8_pending(fd: i32) {
    if let Some(m) = FD_U8_PENDING.lock().unwrap().as_mut() {
        m.remove(&fd);
    }
}

fn take_u8_pending(fd: i32) -> U8Pending {
    FD_U8_PENDING
        .lock()
        .unwrap()
        .as_mut()
        .and_then(|m| m.remove(&fd))
        .unwrap_or_default()
}

fn store_u8_pending(fd: i32, pending: U8Pending) {
    if !pending.raw.is_empty() || pending.low_surrogate.is_some() {
        FD_U8_PENDING.lock().unwrap().get_or_insert_with(HashMap::new).insert(fd, pending);
    }
}

fn has_u8_pending(fd: i32) -> bool {
    FD_U8_PENDING
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|m| m.get(&fd))
        .is_some_and(|p| !p.raw.is_empty() || p.low_surrogate.is_some())
}
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
    let i = FdInfo { mode: TextMode::AnsiText, win_mode: O_TEXT, raw, temporary: false, osf_handle: None };
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

/// Detect or emit BOM for Unicode text modes (`_O_WTEXT`, `fopen(..., "..., ccs=...")`).
/// Matches MSVC UCRT BOM precedence: an existing BOM overrides the requested encoding;
/// creating/truncating a new writable file with BOM enabled writes the UTF-16LE or UTF-8 BOM.
unsafe fn apply_unicode_bom(fd: i32, oflag: i32, requested: TextMode, write_bom_on_new: bool, detect_bom: bool) -> TextMode {
    let mut st: libc::stat = std::mem::zeroed();
    if libc::fstat(fd, &mut st) != 0 || (st.st_mode & libc::S_IFMT) != libc::S_IFREG {
        return requested;
    }
    let acc = oflag & 3;
    let can_write = acc == O_WRONLY || acc == O_RDWR;
    if st.st_size == 0 {
        if can_write && write_bom_on_new {
            let bom: &[u8] = match requested {
                TextMode::Utf16Le => &[0xFF, 0xFE],
                TextMode::Utf8 => &[0xEF, 0xBB, 0xBF],
                _ => &[],
            };
            if !bom.is_empty() {
                let _ = libc::write(fd, bom.as_ptr() as *const c_void, bom.len());
            }
        }
        return requested;
    }
    if detect_bom {
        let mut head = [0u8; 3];
        let n = libc::pread(fd, head.as_mut_ptr() as *mut c_void, 3, 0);
        let is_append = (oflag & O_APPEND) != 0;
        if n >= 2 && head[0] == 0xFF && head[1] == 0xFE {
            if !is_append {
                let _ = libc::lseek(fd, 2, libc::SEEK_SET);
            }
            return TextMode::Utf16Le;
        }
        if n >= 3 && head[0] == 0xEF && head[1] == 0xBB && head[2] == 0xBF {
            if !is_append {
                let _ = libc::lseek(fd, 3, libc::SEEK_SET);
            }
            return TextMode::Utf8;
        }
    }
    requested
}

pub fn open_host(win_path: &str, oflag: i32, pmode: i32) -> i32 {
    open_host_bom(win_path, oflag, pmode, (oflag & O_WTEXT) != 0, (oflag & O_WTEXT) != 0)
}

fn open_host_bom(win_path: &str, oflag: i32, pmode: i32, write_bom: bool, detect_bom: bool) -> i32 {
    let Some(host) = crate::path::to_host(win_path) else {
        if crate::path::device_name(win_path) == Some("NUL") {
            return open_host_path(std::path::Path::new("/dev/null"), oflag, pmode, write_bom, detect_bom);
        }
        errno::set(ENOENT);
        return -1;
    };
    open_host_path(&host, oflag, pmode, write_bom, detect_bom)
}

fn open_host_path(host: &std::path::Path, oflag: i32, pmode: i32, write_bom: bool, detect_bom: bool) -> i32 {
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
    // If BOM detection is requested on a write-only append/existing file, open RDWR internally
    // so `pread` can inspect the BOM at offset 0.
    if detect_bom && acc == O_WRONLY && (oflag & O_TRUNC) == 0 {
        f = (f & !libc::O_ACCMODE) | libc::O_RDWR;
    }
    f |= libc::O_CLOEXEC;
    let mode = if pmode & 0x80 != 0 { 0o644 } else { 0o444 }; // _S_IWRITE
    let c = std::ffi::CString::new(host.as_os_str().as_encoded_bytes()).unwrap();
    let fd = unsafe { libc::open(c.as_ptr(), f, mode as libc::c_uint) };
    if fd < 0 {
        errno::set_from_host();
        return -1;
    }
    let (req_mode, win_mode) = if (oflag & O_BINARY) != 0 {
        (TextMode::Binary, O_BINARY)
    } else if (oflag & O_U8TEXT) != 0 {
        (TextMode::Utf8, O_U8TEXT)
    } else if (oflag & O_U16TEXT) != 0 {
        (TextMode::Utf16Le, O_U16TEXT)
    } else if (oflag & O_WTEXT) != 0 {
        (TextMode::Utf16Le, O_WTEXT)
    } else if (oflag & O_TEXT) != 0 {
        (TextMode::AnsiText, O_TEXT)
    } else {
        let fm = FMODE.load(std::sync::atomic::Ordering::Relaxed);
        if (fm & O_BINARY) != 0 {
            (TextMode::Binary, O_BINARY)
        } else if (fm & O_WTEXT) != 0 {
            (TextMode::Utf16Le, O_WTEXT)
        } else {
            (TextMode::AnsiText, O_TEXT)
        }
    };
    let final_mode = if req_mode.is_unicode() {
        unsafe { apply_unicode_bom(fd, oflag, req_mode, write_bom, detect_bom) }
    } else {
        req_mode
    };
    let final_win_mode = match (win_mode, final_mode) {
        (O_WTEXT, TextMode::Utf8) => O_U8TEXT,
        (O_WTEXT, TextMode::Utf16Le) => O_WTEXT,
        (O_U8TEXT, TextMode::Utf16Le) => O_U16TEXT,
        (O_U16TEXT, TextMode::Utf8) => O_U8TEXT,
        (w, _) => w,
    };
    set_fd_info(
        fd,
        FdInfo {
            mode: final_mode,
            win_mode: final_win_mode,
            raw: false,
            temporary: oflag & O_TEMPORARY != 0,
            osf_handle: None,
        },
        Some(host.to_path_buf()),
    );
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
    clear_u8_pending(fd);
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
    let (mode, win_mode) = if (flags & O_BINARY) != 0 {
        (TextMode::Binary, O_BINARY)
    } else if (flags & O_U8TEXT) != 0 {
        (TextMode::Utf8, O_U8TEXT)
    } else if (flags & O_U16TEXT) != 0 {
        (TextMode::Utf16Le, O_U16TEXT)
    } else if (flags & O_WTEXT) != 0 {
        (TextMode::Utf16Le, O_WTEXT)
    } else if (flags & O_TEXT) != 0 {
        (TextMode::AnsiText, O_TEXT)
    } else {
        (TextMode::Binary, O_BINARY)
    };
    let raw = kind != FileKind::Disk;
    set_fd_info(
        crt_fd,
        FdInfo {
            mode,
            win_mode,
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
    use crate::kernel32::file::{classify_host_fd, wrap_owned_fd, FileKind};
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
    let dup_fd = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3);
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
                    mode: TextMode::AnsiText,
                    win_mode: O_TEXT,
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

unsafe fn raw_read_retry(fd: i32, buf: *mut u8, n: usize) -> isize {
    loop {
        let k = libc::read(fd, buf as *mut c_void, n);
        if k >= 0 {
            return k;
        }
        let e = *libc::__error();
        if e == libc::EINTR {
            continue;
        }
        if (e == libc::EAGAIN || e == libc::EWOULDBLOCK) && wait_pipe_fd_ready(fd, libc::POLLIN, e) {
            continue;
        }
        errno::set_from_host();
        return -1;
    }
}

unsafe fn raw_write_all(fd: i32, out: &[u8]) -> bool {
    let mut off = 0;
    while off < out.len() {
        let r = libc::write(fd, out[off..].as_ptr() as *const c_void, out.len() - off);
        if r < 0 {
            let e = *libc::__error();
            if e == libc::EINTR {
                continue;
            }
            if (e == libc::EAGAIN || e == libc::EWOULDBLOCK) && wait_pipe_fd_ready(fd, libc::POLLOUT, e) {
                continue;
            }
            errno::set_from_host();
            return false;
        }
        if r == 0 {
            errno::set(EPIPE);
            return false;
        }
        off += r as usize;
    }
    true
}

/// Raw read with CRLF->LF and Unicode (`_O_WTEXT`/`_O_U16TEXT`/`_O_U8TEXT`) translation.
pub unsafe fn read_fd(fd: i32, buf: *mut u8, n: usize) -> isize {
    read_fd_tracked(fd, buf, n, false).0
}

/// Like [`read_fd`], and when `track_pos` is true on a seekable translated descriptor,
/// also returns a `len + 1` table mapping each translated buffer index to the physical
/// file byte offset where its source character starts (with the final entry equal to the
/// physical offset after the last translated byte).
unsafe fn read_fd_tracked(fd: i32, buf: *mut u8, n: usize, track_pos: bool) -> (isize, Vec<i64>) {
    let info = fd_info(fd);
    match info.mode {
        TextMode::Binary => {
            let mut pend = take_u8_pending(fd);
            if pend.raw.is_empty() {
                return (raw_read_retry(fd, buf, n), Vec::new());
            }
            let take = pend.raw.len().min(n);
            std::ptr::copy_nonoverlapping(pend.raw.as_ptr(), buf, take);
            pend.raw.drain(..take);
            store_u8_pending(fd, pend);
            (take as isize, Vec::new())
        }
        TextMode::AnsiText => {
            let mut pend = take_u8_pending(fd);
            if info.raw {
                if pend.raw.is_empty() {
                    return (raw_read_retry(fd, buf, n), Vec::new());
                }
                let take = pend.raw.len().min(n);
                std::ptr::copy_nonoverlapping(pend.raw.as_ptr(), buf, take);
                pend.raw.drain(..take);
                store_u8_pending(fd, pend);
                return (take as isize, Vec::new());
            }
            let raw_fd_pos = if track_pos { libc::lseek(fd, 0, libc::SEEK_CUR) } else { -1 };
            let start_pos = if raw_fd_pos >= 0 { (raw_fd_pos - pend.raw.len() as i64).max(0) } else { -1 };
            let total = if !pend.raw.is_empty() {
                let take = pend.raw.len().min(n);
                std::ptr::copy_nonoverlapping(pend.raw.as_ptr(), buf, take);
                pend.raw.drain(..take);
                take
            } else {
                let r = raw_read_retry(fd, buf, n);
                if r <= 0 {
                    store_u8_pending(fd, pend);
                    return (r, Vec::new());
                }
                r as usize
            };
            let mut src_pos = if start_pos >= 0 { Vec::with_capacity(total + 1) } else { Vec::new() };
            let mut extra_fd = 0usize;
            let s = std::slice::from_raw_parts_mut(buf, total);
            let mut w = 0;
            let mut i = 0;
            while i < s.len() {
                if s[i] == b'\r' {
                    if i + 1 < s.len() {
                        if s[i + 1] == b'\n' {
                            if start_pos >= 0 {
                                src_pos.push(start_pos + i as i64);
                            }
                            s[w] = b'\n';
                            w += 1;
                            i += 2;
                            continue;
                        }
                    } else if !pend.raw.is_empty() {
                        if pend.raw[0] == b'\n' {
                            pend.raw.remove(0);
                            if start_pos >= 0 {
                                src_pos.push(start_pos + i as i64);
                            }
                            s[w] = b'\n';
                            w += 1;
                            i += 1;
                            extra_fd = 1;
                            continue;
                        }
                    } else {
                        let mut nb = 0u8;
                        let k = raw_read_retry(fd, &mut nb, 1);
                        if k == 1 {
                            if nb == b'\n' {
                                if start_pos >= 0 {
                                    src_pos.push(start_pos + i as i64);
                                }
                                s[w] = b'\n';
                                w += 1;
                                i += 1;
                                extra_fd = 1;
                                continue;
                            }
                            if libc::lseek(fd, -1, libc::SEEK_CUR) < 0 {
                                pend.raw.push(nb);
                            }
                        }
                    }
                }
                if start_pos >= 0 {
                    src_pos.push(start_pos + i as i64);
                }
                s[w] = s[i];
                w += 1;
                i += 1;
            }
            store_u8_pending(fd, pend);
            if start_pos >= 0 {
                src_pos.push(start_pos + (total + extra_fd) as i64);
            }
            (w as isize, src_pos)
        }
        TextMode::Utf16Le => {
            // MSVC UCRT requires even byte count for UTF-16 Unicode modes.
            if n % 2 != 0 {
                errno::set(EINVAL);
                return (-1, Vec::new());
            }
            if n == 0 {
                return (0, Vec::new());
            }
            let mut pend = take_u8_pending(fd);
            let raw_fd_pos = if track_pos && !info.raw { libc::lseek(fd, 0, libc::SEEK_CUR) } else { -1 };
            let start_pos = if raw_fd_pos >= 0 { (raw_fd_pos - pend.raw.len() as i64).max(0) } else { -1 };
            let mut total = 0usize;
            if !pend.raw.is_empty() {
                let take = pend.raw.len().min(n);
                std::ptr::copy_nonoverlapping(pend.raw.as_ptr(), buf, take);
                pend.raw.drain(..take);
                total = take;
            }
            if total == 0 || (total < n && total % 2 == 1) {
                let r = raw_read_retry(fd, buf.add(total), n - total);
                if r < 0 {
                    if total > 0 {
                        let mut restored = Vec::with_capacity(total + pend.raw.len());
                        restored.extend_from_slice(std::slice::from_raw_parts(buf, total));
                        restored.extend_from_slice(&pend.raw);
                        pend.raw = restored;
                    }
                    store_u8_pending(fd, pend);
                    return (-1, Vec::new());
                }
                total += r as usize;
            }
            if total == 0 {
                store_u8_pending(fd, pend);
                return (0, Vec::new());
            }
            // If an odd byte arrived, read 1 more byte to complete the UTF-16LE code unit;
            // if EOF/short, preserve the stray partial byte in per-fd pending state.
            if total % 2 == 1 {
                let mut extra = 0u8;
                if total < n && raw_read_retry(fd, &mut extra, 1) == 1 {
                    *buf.add(total) = extra;
                    total += 1;
                } else {
                    let stray = *buf.add(total - 1);
                    pend.raw.insert(0, stray);
                    total -= 1;
                }
            }
            if info.raw {
                store_u8_pending(fd, pend);
                return (total as isize, Vec::new());
            }
            let units = total / 2;
            let mut src_pos = if start_pos >= 0 { Vec::with_capacity(total + 1) } else { Vec::new() };
            let mut extra_fd = 0usize;
            let mut w = 0usize;
            let mut i = 0usize;
            while i < units {
                let unit_phys = start_pos + (i * 2) as i64;
                let u = u16::from_le_bytes([*buf.add(i * 2), *buf.add(i * 2 + 1)]);
                if u == 0x000D {
                    if i + 1 < units {
                        let next_u = u16::from_le_bytes([*buf.add((i + 1) * 2), *buf.add((i + 1) * 2 + 1)]);
                        if next_u == 0x000A {
                            let lf = 0x000Au16.to_le_bytes();
                            *buf.add(w * 2) = lf[0];
                            *buf.add(w * 2 + 1) = lf[1];
                            if start_pos >= 0 {
                                src_pos.push(unit_phys);
                                src_pos.push(unit_phys);
                            }
                            w += 1;
                            i += 2;
                            continue;
                        }
                    } else if pend.raw.len() >= 2 {
                        if u16::from_le_bytes([pend.raw[0], pend.raw[1]]) == 0x000A {
                            pend.raw.drain(..2);
                            let lf = 0x000Au16.to_le_bytes();
                            *buf.add(w * 2) = lf[0];
                            *buf.add(w * 2 + 1) = lf[1];
                            if start_pos >= 0 {
                                src_pos.push(unit_phys);
                                src_pos.push(unit_phys);
                            }
                            w += 1;
                            i += 1;
                            extra_fd += 2;
                            continue;
                        }
                    } else {
                        let mut peek = [0u8; 2];
                        let pre = pend.raw.len();
                        if pre == 1 {
                            peek[0] = pend.raw[0];
                        }
                        let mut have = pre;
                        let mut pk_total = 0usize;
                        while have < 2 {
                            let pk = raw_read_retry(fd, peek.as_mut_ptr().add(have), 2 - have);
                            if pk <= 0 {
                                break;
                            }
                            have += pk as usize;
                            pk_total += pk as usize;
                        }
                        if have == 2 && u16::from_le_bytes(peek) == 0x000A {
                            pend.raw.clear();
                            let lf = 0x000Au16.to_le_bytes();
                            *buf.add(w * 2) = lf[0];
                            *buf.add(w * 2 + 1) = lf[1];
                            if start_pos >= 0 {
                                src_pos.push(unit_phys);
                                src_pos.push(unit_phys);
                            }
                            w += 1;
                            i += 1;
                            extra_fd += 2;
                            continue;
                        }
                        // Non-LF lookahead (or partial 1-byte read): rewind seekable fd, or retain
                        // in per-fd pending input when seeking is unavailable (e.g. pipes/FIFOs).
                        if pk_total > 0 {
                            let from_fd = &peek[pre..have];
                            if pre > 0 || libc::lseek(fd, -(pk_total as i64), libc::SEEK_CUR) < 0 {
                                pend.raw.extend_from_slice(from_fd);
                            }
                        }
                    }
                }
                let bytes = u.to_le_bytes();
                *buf.add(w * 2) = bytes[0];
                *buf.add(w * 2 + 1) = bytes[1];
                if start_pos >= 0 {
                    src_pos.push(unit_phys);
                    src_pos.push(unit_phys);
                }
                w += 1;
                i += 1;
            }
            store_u8_pending(fd, pend);
            if start_pos >= 0 {
                src_pos.push(start_pos + (units * 2 + extra_fd) as i64);
            }
            ((w * 2) as isize, src_pos)
        }
        TextMode::Utf8 => {
            if n % 2 != 0 {
                errno::set(EINVAL);
                return (-1, Vec::new());
            }
            if n == 0 {
                return (0, Vec::new());
            }
            let max_u16 = n / 2;
            let mut pend = take_u8_pending(fd);
            let raw_fd_pos = if track_pos { libc::lseek(fd, 0, libc::SEEK_CUR) } else { -1 };
            let start_pos = if raw_fd_pos >= 0 { (raw_fd_pos - pend.raw.len() as i64).max(0) } else { -1 };
            let mut src_pos = if start_pos >= 0 { Vec::with_capacity(n + 1) } else { Vec::new() };
            let mut extra_fd = 0usize;
            let mut out_u16: Vec<u16> = Vec::with_capacity(max_u16);
            if let Some(lo) = pend.low_surrogate.take() {
                out_u16.push(lo);
                if start_pos >= 0 {
                    src_pos.push(start_pos);
                    src_pos.push(start_pos);
                }
            }
            let mut raw_buf = std::mem::take(&mut pend.raw);
            if out_u16.len() < max_u16 && raw_buf.is_empty() {
                let mut tmp = vec![0u8; max_u16.max(4)];
                let r = raw_read_retry(fd, tmp.as_mut_ptr(), tmp.len());
                if r < 0 {
                    store_u8_pending(fd, U8Pending { raw: raw_buf, low_surrogate: None });
                    return if out_u16.is_empty() {
                        (-1, Vec::new())
                    } else {
                        for (idx, u) in out_u16.iter().enumerate() {
                            let bytes = u.to_le_bytes();
                            *buf.add(idx * 2) = bytes[0];
                            *buf.add(idx * 2 + 1) = bytes[1];
                        }
                        if start_pos >= 0 {
                            src_pos.push(start_pos);
                        }
                        ((out_u16.len() * 2) as isize, src_pos)
                    };
                }
                raw_buf.extend_from_slice(&tmp[..r as usize]);
            }
            let mut consumed = 0usize;
            while consumed < raw_buf.len() && out_u16.len() < max_u16 {
                let char_phys = start_pos + (consumed + extra_fd) as i64;
                let rem = &raw_buf[consumed..];
                if !info.raw && rem[0] == b'\r' {
                    if rem.len() >= 2 {
                        if rem[1] == b'\n' {
                            out_u16.push(b'\n' as u16);
                            if start_pos >= 0 {
                                src_pos.push(char_phys);
                                src_pos.push(char_phys);
                            }
                            consumed += 2;
                            continue;
                        }
                    } else {
                        let mut nb = 0u8;
                        let pk = raw_read_retry(fd, &mut nb, 1);
                        if pk == 1 {
                            if nb == b'\n' {
                                out_u16.push(b'\n' as u16);
                                if start_pos >= 0 {
                                    src_pos.push(char_phys);
                                    src_pos.push(char_phys);
                                }
                                consumed += 1;
                                extra_fd += 1;
                                continue;
                            }
                            raw_buf.push(nb);
                            out_u16.push(b'\r' as u16);
                            if start_pos >= 0 {
                                src_pos.push(char_phys);
                                src_pos.push(char_phys);
                            }
                            consumed += 1;
                            continue;
                        }
                    }
                }
                let b0 = rem[0];
                let need = if b0 < 0x80 {
                    1
                } else if (0xC2..=0xDF).contains(&b0) {
                    2
                } else if (0xE0..=0xEF).contains(&b0) {
                    3
                } else if (0xF0..=0xF4).contains(&b0) {
                    4
                } else {
                    1
                };
                if rem.len() < need {
                    if out_u16.is_empty() {
                        // Fetch remaining bytes of the UTF-8 sequence.
                        let mut more = [0u8; 4];
                        let k = raw_read_retry(fd, more.as_mut_ptr(), need - rem.len());
                        if k > 0 {
                            raw_buf.extend_from_slice(&more[..k as usize]);
                            continue;
                        }
                        // EOF mid-sequence: emit replacement char and consume truncated bytes.
                        out_u16.push(0xFFFD);
                        if start_pos >= 0 {
                            src_pos.push(char_phys);
                            src_pos.push(char_phys);
                        }
                        consumed = raw_buf.len();
                    }
                    break;
                }
                let ch = std::str::from_utf8(&rem[..need]).ok().and_then(|s| s.chars().next()).unwrap_or('\u{FFFD}');
                let mut tmp = [0u16; 2];
                let enc = ch.encode_utf16(&mut tmp);
                if out_u16.len() + enc.len() > max_u16 {
                    // Only happens when a surrogate pair (2 units) lands on the last 1-unit slot.
                    // Emit the high surrogate now and retain the low surrogate for the next read
                    // so small (2-byte) or unbuffered reads never stall with 0 bytes (false EOF).
                    out_u16.push(enc[0]);
                    if start_pos >= 0 {
                        src_pos.push(char_phys);
                        src_pos.push(char_phys);
                    }
                    pend.low_surrogate = Some(enc[1]);
                    consumed += need;
                    break;
                }
                out_u16.extend_from_slice(enc);
                if start_pos >= 0 {
                    for _ in 0..(enc.len() * 2) {
                        src_pos.push(char_phys);
                    }
                }
                consumed += need;
            }
            if consumed < raw_buf.len() {
                let left = raw_buf.len() - consumed;
                // Rewind seekable regular files so `_tell`/`ftell` stay accurate; on pipes/ttys
                // where `lseek` fails with ESPIPE, retain the unread bytes in per-fd state.
                if pend.low_surrogate.is_some() || libc::lseek(fd, -(left as i64), libc::SEEK_CUR) < 0 {
                    pend.raw = raw_buf[consumed..].to_vec();
                }
            }
            store_u8_pending(fd, pend);
            for (idx, u) in out_u16.iter().enumerate() {
                let bytes = u.to_le_bytes();
                *buf.add(idx * 2) = bytes[0];
                *buf.add(idx * 2 + 1) = bytes[1];
            }
            if start_pos >= 0 {
                src_pos.push(start_pos + (consumed + extra_fd) as i64);
            }
            ((out_u16.len() * 2) as isize, src_pos)
        }
    }
}

/// Raw write with LF->CRLF and Unicode (`_O_WTEXT`/`_O_U16TEXT`/`_O_U8TEXT`) translation.
/// Returns source byte count on success, or `-1` on error.
pub unsafe fn write_fd(fd: i32, data: &[u8]) -> isize {
    let info = fd_info(fd);
    match info.mode {
        TextMode::Binary => {
            if raw_write_all(fd, data) { data.len() as isize } else { -1 }
        }
        TextMode::AnsiText => {
            let tmp;
            let out: &[u8] = if !info.raw && data.contains(&b'\n') {
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
            if raw_write_all(fd, out) { data.len() as isize } else { -1 }
        }
        TextMode::Utf16Le => {
            if data.len() % 2 != 0 {
                errno::set(EINVAL);
                return -1;
            }
            let tmp;
            let out: &[u8] = if !info.raw {
                let mut v = Vec::with_capacity(data.len() + 16);
                for chunk in data.chunks_exact(2) {
                    let u = u16::from_le_bytes([chunk[0], chunk[1]]);
                    if u == 0x000A {
                        v.extend_from_slice(&0x000Du16.to_le_bytes());
                    }
                    v.extend_from_slice(chunk);
                }
                tmp = v;
                &tmp
            } else {
                data
            };
            if raw_write_all(fd, out) { data.len() as isize } else { -1 }
        }
        TextMode::Utf8 => {
            if data.len() % 2 != 0 {
                errno::set(EINVAL);
                return -1;
            }
            let units: Vec<u16> = data.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
            let s = String::from_utf16_lossy(&units);
            let mut out = Vec::with_capacity(s.len() + 16);
            for b in s.as_bytes() {
                if !info.raw && *b == b'\n' {
                    out.push(b'\r');
                }
                out.push(*b);
            }
            if raw_write_all(fd, &out) { data.len() as isize } else { -1 }
        }
    }
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
    let is_tell = whence == libc::SEEK_CUR && off == 0;
    let r = libc::lseek(fd, off, whence);
    if r < 0 {
        errno::set_from_host();
        return -1;
    }
    if is_tell {
        let pend_bytes = FD_U8_PENDING
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|m| m.get(&fd))
            .map(|p| p.raw.len() as i64)
            .unwrap_or(0);
        return (r - pend_bytes).max(0);
    }
    clear_u8_pending(fd);
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
    clear_u8_pending(fd2);
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
    let old = i.win_mode;
    let new_mode = match mode {
        O_BINARY => TextMode::Binary,
        O_TEXT => TextMode::AnsiText,
        O_WTEXT | O_U16TEXT => TextMode::Utf16Le,
        O_U8TEXT => TextMode::Utf8,
        _ => {
            errno::set(EINVAL);
            super::runtime::invalid_parameter("_setmode");
            return -1;
        }
    };
    let path = FDS.lock().unwrap().get_or_insert_with(HashMap::new).get(&fd).and_then(|e| e.1.clone());
    set_fd_info(fd, FdInfo { mode: new_mode, win_mode: mode, ..i }, path);
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
    /// Physical file byte offset for each index in `rbuf` (length `rbuf.len() + 1`
    /// when populated for seekable translated modes, empty otherwise).
    rbuf_src_pos: Vec<i64>,
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
                    rbuf_src_pos: Vec::new(),
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

    /// Logical physical-file offset corresponding to the current read position,
    /// using the per-byte source-offset table when `rbuf` holds translated input.
    fn logical_read_pos(&self) -> Option<i64> {
        if !self.rbuf_src_pos.is_empty() && self.rpos < self.rbuf_src_pos.len() {
            if self.unget.is_empty() {
                return Some(self.rbuf_src_pos[self.rpos]);
            }
            if self.unget.len() <= self.rpos {
                return Some(self.rbuf_src_pos[self.rpos - self.unget.len()]);
            }
            let extra = (self.unget.len() - self.rpos) as i64;
            return Some((self.rbuf_src_pos[0] - extra).max(0));
        }
        None
    }

    /// Drop read-ahead, seeking the fd back so its position is logical.
    unsafe fn discard_read(&mut self) {
        if let Some(target) = self.logical_read_pos() {
            if self.rpos < self.rbuf.len() || !self.unget.is_empty() || has_u8_pending(self.fd) {
                libc::lseek(self.fd, target, libc::SEEK_SET);
                clear_u8_pending(self.fd);
            }
        } else {
            let pending = (self.rbuf.len() - self.rpos) + self.unget.len();
            if pending > 0 {
                libc::lseek(self.fd, -(pending as i64), libc::SEEK_CUR);
                clear_u8_pending(self.fd);
            }
        }
        self.rbuf.clear();
        self.rbuf_src_pos.clear();
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
        let cap = if self.mode == BufMode::None {
            if fd_info(self.fd).mode.is_unicode() { 4 } else { 1 }
        } else {
            (self.bufsize.max(512)) & !1
        };
        self.rbuf.resize(cap, 0);
        self.rbuf_src_pos.clear();
        self.rpos = 0;
        let (n, src_pos) = read_fd_tracked(self.fd, self.rbuf.as_mut_ptr(), cap, true);
        if n <= 0 {
            self.rbuf.clear();
            self.rbuf_src_pos.clear();
            if n == 0 {
                self.eof = true;
            } else {
                self.err = true;
            }
            return false;
        }
        self.rbuf.truncate(n as usize);
        self.rbuf_src_pos = src_pos;
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
                self.rbuf.clear();
                self.rbuf_src_pos.clear();
                self.rpos = 0;
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
        if !self.wbuf.is_empty() && !self.flush() {
            return -1;
        }
        if let Some(p) = self.logical_read_pos() {
            return p;
        }
        let pos = libc::lseek(self.fd, 0, libc::SEEK_CUR);
        if pos < 0 {
            errno::set_from_host();
            return -1;
        }
        let pend_u8 = FD_U8_PENDING
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|m| m.get(&self.fd))
            .map(|p| p.raw.len() as i64)
            .unwrap_or(0);
        pos - pend_u8 - (self.rbuf.len() - self.rpos) as i64 - self.unget.len() as i64
    }

    unsafe fn seek(&mut self, off: i64, whence: i32) -> i32 {
        if !self.flush() {
            return -1;
        }
        let (off, whence) = if whence == libc::SEEK_CUR {
            let cur = self.tell();
            if cur < 0 {
                return -1;
            }
            (cur + off, libc::SEEK_SET)
        } else {
            (off, whence)
        };
        if libc::lseek(self.fd, off, whence) < 0 {
            errno::set_from_host();
            return -1;
        }
        self.rbuf.clear();
        self.rbuf_src_pos.clear();
        self.rpos = 0;
        self.unget.clear();
        clear_u8_pending(self.fd);
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
    write_bom: bool,
    detect_bom: bool,
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
                if c == 'D' {
                    oflag |= O_TEMPORARY;
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
    let mut write_bom = false;
    let mut detect_bom = false;
    if let Some(c) = ccs {
        let norm = c.to_ascii_uppercase().replace(' ', "");
        let enc = norm.strip_prefix("CCS=")?;
        let unicode_flag = match enc {
            "UTF-8" => O_U8TEXT,
            "UTF-16LE" => O_U16TEXT,
            "UNICODE" => O_WTEXT,
            _ => return None,
        };
        // MSVC UCRT `fopen(..., "..., ccs=...")`: BOM takes precedence on existing files;
        // newly created/truncated writable files receive the corresponding BOM.
        oflag = (oflag & !(O_TEXT | O_BINARY | O_WTEXT | O_U16TEXT | O_U8TEXT)) | unicode_flag;
        write_bom = w;
        detect_bom = true;
    }
    Some(ModeFlags { oflag, readable: r, writable: w, write_bom, detect_bom })
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
                rbuf_src_pos: Vec::new(),
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
    let fd = open_host_bom(path, mf.oflag, 0x180, mf.write_bom, mf.detect_bom);
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
        if fd_info(s.fd).mode.is_unicode() {
            // Unicode stream (`_O_WTEXT` / `_O_U16TEXT` / `_O_U8TEXT`): `read_fd` yields UTF-16LE units.
            let lo = s.getc()?;
            let hi = s.getc()?;
            return Some(u16::from_le_bytes([lo, hi]) as u32);
        }
        let b0 = s.getc()?;
        let cp = super::locale::mb_cp();
        if b0 < 0x80 || cp == 0 {
            return Some(b0 as u32);
        }
        if cp == 65001 {
            let need = match b0 {
                0xC2..=0xDF => 1,
                0xE0..=0xEF => 2,
                0xF0..=0xF4 => 3,
                _ => {
                    s.err = true;
                    errno::set(EILSEQ);
                    return None;
                }
            };
            let mut v = vec![b0];
            for _ in 0..need {
                match s.getc() {
                    Some(b) => v.push(b),
                    None => {
                        s.err = true;
                        errno::set(EILSEQ);
                        return None;
                    }
                }
            }
            match std::str::from_utf8(&v) {
                Ok(st) => st.encode_utf16().next().map(|c| c as u32),
                Err(_) => {
                    s.err = true;
                    errno::set(EILSEQ);
                    None
                }
            }
        } else {
            let is_lead = crate::kernel32::string::IsDBCSLeadByteEx(cp, b0) != 0;
            let bytes = if is_lead {
                match s.getc() {
                    Some(b1) => vec![b0, b1],
                    None => {
                        s.err = true;
                        errno::set(EILSEQ);
                        return None;
                    }
                }
            } else {
                vec![b0]
            };
            match crate::kernel32::string::decode_cp_to_utf16(cp, &bytes) {
                Ok(w) if !w.is_empty() && !w.contains(&0xFFFD) => Some(w[0] as u32),
                _ => {
                    s.err = true;
                    errno::set(EILSEQ);
                    None
                }
            }
        }
    });
    r.flatten().unwrap_or(WEOF)
}
pub unsafe extern "win64" fn getwc(fp: *mut FILE) -> u32 {
    fgetwc(fp)
}
pub unsafe extern "win64" fn _fgetwc_nolock(fp: *mut FILE) -> u32 {
    fgetwc(fp)
}

fn stream_is_unicode(fp: *mut FILE) -> bool {
    with_stream(fp as *mut c_void, |s| fd_info(s.fd).mode.is_unicode()).unwrap_or(false)
}

/// Write wide code units to a `FILE*` stream (used by `fputws` and `vfwprintf`).
/// Unicode streams (`_O_WTEXT`/`_O_U16TEXT`/`_O_U8TEXT`) receive UTF-16LE bytes;
/// narrow text/binary streams encode via the active CRT `LC_CTYPE` code page.
pub unsafe fn write_stream_wide(fp: *mut c_void, units: &[u16]) -> i32 {
    let _l = Locked::new();
    with_stream(fp, |st| {
        let bytes = if fd_info(st.fd).mode.is_unicode() {
            let mut out = Vec::with_capacity(units.len() * 2);
            for &u in units {
                out.extend_from_slice(&u.to_le_bytes());
            }
            out
        } else {
            match super::convert::wc_encode_slice(units) {
                Ok(b) => b,
                Err(()) => {
                    st.err = true;
                    errno::set(EILSEQ);
                    return -1;
                }
            }
        };
        if st.write(&bytes) == bytes.len() {
            units.len() as i32
        } else {
            -1
        }
    })
    .unwrap_or(-1)
}

pub unsafe extern "win64" fn fputwc(c: u16, fp: *mut FILE) -> u32 {
    let _l = Locked::new();
    let ok = with_stream(fp as *mut c_void, |s| {
        let bytes = if fd_info(s.fd).mode.is_unicode() {
            c.to_le_bytes().to_vec()
        } else {
            match super::convert::wc_encode(c) {
                Ok(b) => b,
                Err(()) => {
                    s.err = true;
                    errno::set(EILSEQ);
                    return false;
                }
            }
        };
        s.write(&bytes) == bytes.len()
    })
    .unwrap_or(false);
    if ok { c as u32 } else { WEOF }
}
pub unsafe extern "win64" fn putwc(c: u16, fp: *mut FILE) -> u32 {
    fputwc(c, fp)
}
pub unsafe extern "win64" fn _fputwc_nolock(c: u16, fp: *mut FILE) -> u32 {
    fputwc(c, fp)
}

pub unsafe extern "win64" fn fputws(s: *const u16, fp: *mut FILE) -> i32 {
    if s.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    let units = crate::strings::read_wide_units_z(s, crate::strings::DEFAULT_MAX_STR_LEN);
    if write_stream_wide(fp as *mut c_void, &units) < 0 { -1 } else { 0 }
}

pub unsafe extern "win64" fn ungetwc(c: u32, fp: *mut FILE) -> u32 {
    if c == WEOF {
        return WEOF;
    }
    let _l = Locked::new();
    let unicode = stream_is_unicode(fp);
    let bytes: Vec<u8> = if unicode {
        (c as u16).to_le_bytes().to_vec()
    } else {
        match super::convert::wc_encode(c as u16) {
            Ok(b) => b,
            Err(()) => return WEOF,
        }
    };
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
    set_fd_info(fd, FdInfo { mode: TextMode::Binary, win_mode: O_BINARY, raw: false, temporary: false, osf_handle: None }, None);
    let _l = Locked::new();
    new_stream(fd, &ModeFlags { oflag: O_RDWR | O_BINARY, readable: true, writable: true, write_bom: false, detect_bom: false })
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

    #[test]
    fn std_fd_get_osfhandle_tracks_fd_independently_of_set_std_handle_and_dup2() {
        use crate::kernel32::file::{CreateFileW, GetStdHandle, SetStdHandle, WriteFile};
        use crate::kernel32::threads::{CloseHandle, DuplicateHandle};
        use crate::types::{HANDLE, INVALID_HANDLE_VALUE, TRUE};

        const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;

        struct StdFd1Guard {
            saved_posix_1: i32,
            saved_std_out: HANDLE,
        }
        impl Drop for StdFd1Guard {
            fn drop(&mut self) {
                unsafe {
                    SetStdHandle(STD_OUTPUT_HANDLE, self.saved_std_out);
                    let _ = libc::dup2(self.saved_posix_1, 1);
                    let prev = FDS.lock().unwrap().get_or_insert_with(HashMap::new).remove(&1);
                    if let Some((old_info, _)) = prev {
                        if let Some(old_h) = old_info.osf_handle {
                            let _ = crate::handles::close(old_h as HANDLE);
                        }
                    }
                    let _ = libc::close(self.saved_posix_1);
                }
            }
        }

        let dir = std::env::temp_dir().join(format!("aw2_std_osf_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        crate::path::add_mount("M:\\", &dir);

        unsafe {
            let create_win = |win_path: &str| -> HANDLE {
                let p = crate::strings::to_wide_z(win_path);
                let h = CreateFileW(
                    p.as_ptr(),
                    0xC000_0000,
                    3,
                    std::ptr::null_mut(),
                    2, // CREATE_ALWAYS
                    0x80,
                    std::ptr::null_mut(),
                );
                assert_ne!(h, INVALID_HANDLE_VALUE);
                h
            };

            let write_win = |h: HANDLE, data: &[u8]| {
                let mut written = 0u32;
                assert_eq!(
                    WriteFile(h, data.as_ptr() as *const c_void, data.len() as u32, &mut written, std::ptr::null_mut()),
                    TRUE
                );
                assert_eq!(written as usize, data.len());
            };

            let saved_posix_1 = libc::fcntl(1, libc::F_DUPFD_CLOEXEC, 3);
            assert!(saved_posix_1 >= 3);
            let saved_std_out = GetStdHandle(STD_OUTPUT_HANDLE);
            let _guard = StdFd1Guard { saved_posix_1, saved_std_out };

            // Open helper files before closing slot 1 so `CreateFileW` / `_open` cannot claim slot 1.
            let fd_extra = _open(b"M:\\extra.bin\0".as_ptr(), O_RDWR | O_CREAT | O_TRUNC | O_BINARY, 0x180);
            let h_alias = create_win("M:\\alias.bin");
            let fd_a = _open(b"M:\\fd1_a.bin\0".as_ptr(), O_RDWR | O_CREAT | O_TRUNC | O_BINARY, 0x180);
            let fd_b = _open(b"M:\\fd1_b.bin\0".as_ptr(), O_RDWR | O_CREAT | O_TRUNC | O_BINARY, 0x180);
            let h_win = create_win("M:\\std_win.bin");
            assert!(fd_extra >= 3 && fd_a >= 3 && fd_b >= 3);

            // 1. With standard slot 1 closed, exporting another fd via _get_osfhandle must use
            //    min fd 3 and leave closed slot 1 unpopulated; _open_osfhandle aliasing into a
            //    standard slot preserves the exact transferred Win32 handle.
            assert_eq!(_close(1), 0);
            assert!(!fd_valid(1));
            assert_eq!(_get_osfhandle(1), -1);
            assert_eq!(errno::get(), EBADF);

            let h_extra = _get_osfhandle(fd_extra) as HANDLE;
            assert_ne!(h_extra, INVALID_HANDLE_VALUE);
            let extra_obj = crate::kernel32::file::get_file(h_extra).expect("exported FileObj");
            assert!(extra_obj.fd() >= 3, "internal duplicate must not occupy closed std slot");
            assert_ne!(libc::fcntl(extra_obj.fd(), libc::F_GETFD) & libc::FD_CLOEXEC, 0);
            drop(extra_obj);
            assert!(!fd_valid(1), "exporting another fd must not populate closed slot 1");
            assert_eq!(_close(fd_extra), 0);

            let fd_alias = _open_osfhandle(h_alias as isize, O_BINARY);
            assert!((0..=2).contains(&fd_alias), "expected _open_osfhandle to reuse closed std slot, got {fd_alias}");
            assert_eq!(_get_osfhandle(fd_alias), h_alias as isize);
            write_win(h_alias, b"ALIAS");
            assert_eq!(_close(fd_alias), 0);
            assert!(crate::handles::get(h_alias).is_none());

            // 2. Bind POSIX fd 1 to file A and redirect Win32 STD_OUTPUT_HANDLE to a distinct file.
            //    _get_osfhandle(1) must track fd 1 (not STD_OUTPUT_HANDLE), stay cached/stable
            //    before and after SetStdHandle, support DuplicateHandle lifetime across _dup2,
            //    and track file B after _dup2(fd_b, 1).
            assert_eq!(_dup2(fd_a, 1), 0);

            assert_eq!(SetStdHandle(STD_OUTPUT_HANDLE, h_win), TRUE);
            assert_eq!(GetStdHandle(STD_OUTPUT_HANDLE), h_win);

            let h1_a = _get_osfhandle(1) as HANDLE;
            assert_ne!(h1_a, INVALID_HANDLE_VALUE);
            assert_ne!(h1_a, h_win);
            assert_ne!(h1_a, saved_std_out);
            assert_eq!(_get_osfhandle(1), h1_a as isize); // cached identity stable
            let h1_a_obj = crate::kernel32::file::get_file(h1_a).expect("fd 1 FileObj");
            assert!(h1_a_obj.fd() >= 3);
            assert_ne!(libc::fcntl(h1_a_obj.fd(), libc::F_GETFD) & libc::FD_CLOEXEC, 0);
            drop(h1_a_obj);

            let mut h1_a_dup: HANDLE = std::ptr::null_mut();
            assert_eq!(
                DuplicateHandle(
                    crate::handles::CURRENT_PROCESS,
                    h1_a,
                    crate::handles::CURRENT_PROCESS,
                    &mut h1_a_dup,
                    0,
                    0,
                    2, // DUPLICATE_SAME_ACCESS
                ),
                TRUE
            );
            assert!(!h1_a_dup.is_null() && h1_a_dup != h1_a);

            write_win(GetStdHandle(STD_OUTPUT_HANDLE), b"WIN-A");
            write_win(h1_a, b"FD1-A");

            // Redirect POSIX fd 1 to file B; old cached handle retires, DuplicateHandle sibling survives.
            assert_eq!(_dup2(fd_b, 1), 0);
            assert!(crate::handles::get(h1_a).is_none());
            assert!(crate::handles::get(h1_a_dup).is_some());
            write_win(h1_a_dup, b"+DUP");
            assert_eq!(CloseHandle(h1_a_dup), TRUE);

            assert_eq!(GetStdHandle(STD_OUTPUT_HANDLE), h_win);
            let h1_b = _get_osfhandle(1) as HANDLE;
            assert_ne!(h1_b, INVALID_HANDLE_VALUE);
            assert_ne!(h1_b, h1_a);
            assert_ne!(h1_b, h_win);
            assert_eq!(_get_osfhandle(1), h1_b as isize); // cached identity stable after _dup2
            let h1_b_obj = crate::kernel32::file::get_file(h1_b).expect("redirected fd 1 FileObj");
            assert!(h1_b_obj.fd() >= 3);
            drop(h1_b_obj);

            write_win(h1_b, b"FD1-B");
            write_win(GetStdHandle(STD_OUTPUT_HANDLE), b"+WIN-B");

            assert_eq!(_close(fd_a), 0);
            assert_eq!(_close(fd_b), 0);
            drop(_guard);
            assert_eq!(CloseHandle(h_win), TRUE);

            assert_eq!(std::fs::read(dir.join("alias.bin")).unwrap(), b"ALIAS");
            assert_eq!(std::fs::read(dir.join("std_win.bin")).unwrap(), b"WIN-A+WIN-B");
            assert_eq!(std::fs::read(dir.join("fd1_a.bin")).unwrap(), b"FD1-A+DUP");
            assert_eq!(std::fs::read(dir.join("fd1_b.bin")).unwrap(), b"FD1-B");
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unicode_text_modes_and_fopen_ccs_utf16le_utf8() {
        let dir = std::env::temp_dir().join(format!("aw2_stdio_u16_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        crate::path::add_mount("U:\\", &dir);

        unsafe {
            // 1. fopen with ccs=UTF-16LE writes UTF-16LE BOM + CRLF-translated UTF-16LE.
            let f = fopen(b"U:\\u16.txt\0".as_ptr(), b"w, ccs=UTF-16LE\0".as_ptr());
            assert!(!f.is_null());
            let wtext = crate::strings::to_wide_z("Hi\n世界");
            assert_eq!(fputws(wtext.as_ptr(), f), 0);
            assert_eq!(fclose(f), 0);

            let raw = std::fs::read(dir.join("u16.txt")).unwrap();
            let mut expected = vec![0xFFu8, 0xFE]; // UTF-16LE BOM
            for u in "Hi\r\n世界".encode_utf16() {
                expected.extend_from_slice(&u.to_le_bytes());
            }
            assert_eq!(raw, expected);

            // 2. BOM precedence: opening UTF-16LE BOM file with ccs=UTF-8 decodes as UTF-16LE!
            let f = fopen(b"U:\\u16.txt\0".as_ptr(), b"r, ccs=UTF-8\0".as_ptr());
            assert!(!f.is_null());
            let mut line = [0u16; 32];
            assert!(!fgetws(line.as_mut_ptr(), 32, f).is_null());
            assert_eq!(from_wide(line.as_ptr()), "Hi\n");
            let c0 = fgetwc(f);
            assert_eq!(c0, '世' as u32);
            assert_eq!(ungetwc(c0, f), c0);
            assert!(!fgetws(line.as_mut_ptr(), 32, f).is_null());
            assert_eq!(from_wide(line.as_ptr()), "世界");
            assert_eq!(fclose(f), 0);

            // 3. fopen with ccs=UTF-8 writes UTF-8 BOM + CRLF UTF-8 and reads wide chars back.
            let f = fopen(b"U:\\u8.txt\0".as_ptr(), b"w, ccs=UTF-8\0".as_ptr());
            assert!(!f.is_null());
            let w8 = crate::strings::to_wide_z("L1\nαβ");
            assert_eq!(fputws(w8.as_ptr(), f), 0);
            assert_eq!(fclose(f), 0);
            assert_eq!(std::fs::read(dir.join("u8.txt")).unwrap(), "\u{FEFF}L1\r\nαβ".as_bytes());

            let f = fopen(b"U:\\u8.txt\0".as_ptr(), b"r, ccs=UTF-8\0".as_ptr());
            assert!(!f.is_null());
            assert!(!fgetws(line.as_mut_ptr(), 32, f).is_null());
            assert_eq!(from_wide(line.as_ptr()), "L1\n");
            assert!(!fgetws(line.as_mut_ptr(), 32, f).is_null());
            assert_eq!(from_wide(line.as_ptr()), "αβ");
            assert_eq!(fclose(f), 0);

            // 4. Low-level _open(_O_U16TEXT) without BOM, CRLF translation, odd-count EINVAL, _setmode.
            let fd = _open(b"U:\\low16.txt\0".as_ptr(), O_RDWR | O_CREAT | O_TRUNC | O_U16TEXT, 0x180);
            assert!(fd >= 0);
            let payload: Vec<u8> = "A\nB".encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
            assert_eq!(_write(fd, payload.as_ptr() as *const c_void, payload.len() as u32), payload.len() as i32);
            // Odd byte count fails with EINVAL in Unicode text mode.
            assert_eq!(_write(fd, payload.as_ptr() as *const c_void, 3), -1);
            assert_eq!(errno::get(), EINVAL);
            assert_eq!(_lseek(fd, 0, 0), 0);
            let mut rbuf = [0u8; 16];
            let got = _read(fd, rbuf.as_mut_ptr() as *mut c_void, 16);
            assert_eq!(got, payload.len() as i32);
            assert_eq!(&rbuf[..got as usize], payload.as_slice());
            assert_eq!(_setmode(fd, O_BINARY), O_U16TEXT);
            assert_eq!(_setmode(fd, O_U8TEXT), O_BINARY);
            assert_eq!(_close(fd), 0);

            // 5. Invalid ccs value invokes invalid-parameter handler and fails with EINVAL.
            unsafe extern "win64" fn noop_iph(_: *const u16, _: *const u16, _: *const u16, _: u32, _: usize) {}
            let prev_iph = crate::ucrt::runtime::_set_invalid_parameter_handler(noop_iph as *const () as usize);
            assert!(fopen(b"U:\\bad.txt\0".as_ptr(), b"w, ccs=BOGUS\0".as_ptr()).is_null());
            assert_eq!(errno::get(), EINVAL);
            crate::ucrt::runtime::_set_invalid_parameter_handler(prev_iph);

            // 6. Unbuffered ccs=UTF-8 stream decodes supplementary UTF-16 surrogate pairs cleanly.
            let f = fopen(b"U:\\surr.txt\0".as_ptr(), b"w, ccs=UTF-8\0".as_ptr());
            assert!(!f.is_null());
            let w_surr = crate::strings::to_wide_z("A😀B");
            assert_eq!(fputws(w_surr.as_ptr(), f), 0);
            assert_eq!(fclose(f), 0);
            let f = fopen(b"U:\\surr.txt\0".as_ptr(), b"r, ccs=UTF-8\0".as_ptr());
            assert!(!f.is_null());
            assert_eq!(setvbuf(f, std::ptr::null_mut(), 4, 0), 0); // _IONBF
            let mut got_units = Vec::new();
            loop {
                let ch = fgetwc(f);
                if ch == WEOF {
                    break;
                }
                got_units.push(ch as u16);
            }
            assert_eq!(String::from_utf16_lossy(&got_units), "A😀B");
            assert_eq!(fclose(f), 0);

            // 7. Non-seekable pipe with _O_U8TEXT preserves unread bytes across small 2-byte reads
            //    and splits/resumes surrogate pairs cleanly.
            let mut pfds = [0i32; 2];
            assert_eq!(libc::pipe(pfds.as_mut_ptr()), 0);
            assert_eq!(libc::write(pfds[1], "ABCD😀".as_ptr() as *const c_void, 8), 8);
            libc::close(pfds[1]);
            assert_eq!(_setmode(pfds[0], O_U8TEXT), O_TEXT);
            let mut pipe_units = Vec::new();
            loop {
                let mut one = [0u8; 2];
                let n = _read(pfds[0], one.as_mut_ptr() as *mut c_void, 2);
                if n <= 0 {
                    break;
                }
                assert_eq!(n, 2);
                pipe_units.push(u16::from_le_bytes(one));
            }
            assert_eq!(String::from_utf16_lossy(&pipe_units), "ABCD😀");
            assert_eq!(_close(pfds[0]), 0);

            // 8. Querying position via _telli64 mid-surrogate preserves pending low surrogate!
            std::fs::write(dir.join("tell_surr.txt"), "😀B".as_bytes()).unwrap();
            let fd_s = _open(b"U:\\tell_surr.txt\0".as_ptr(), O_RDONLY | O_U8TEXT, 0);
            assert!(fd_s >= 0);
            let mut u = [0u8; 2];
            assert_eq!(_read(fd_s, u.as_mut_ptr() as *mut c_void, 2), 2);
            let hi = u16::from_le_bytes(u);
            assert!(_telli64(fd_s) >= 0);
            assert_eq!(_read(fd_s, u.as_mut_ptr() as *mut c_void, 2), 2);
            let lo = u16::from_le_bytes(u);
            assert_eq!(_read(fd_s, u.as_mut_ptr() as *mut c_void, 2), 2);
            let b_ch = u16::from_le_bytes(u);
            assert_eq!(String::from_utf16_lossy(&[hi, lo, b_ch]), "😀B");
            assert_eq!(_close(fd_s), 0);

            // 9. Buffered ccs=UTF-8 ftell/fgetpos/fseek/fsetpos and read->write discard_read
            //    track physical UTF-8 byte offsets accurately across BOM, multibyte, and CRLF.
            std::fs::write(dir.join("bom_pos.txt"), "\u{FEFF}ABCDαβ\r\n😀Z".as_bytes()).unwrap();
            let f = fopen(b"U:\\bom_pos.txt\0".as_ptr(), b"r+, ccs=UTF-8\0".as_ptr());
            assert!(!f.is_null());
            assert_eq!(fgetwc(f), 'A' as u32);
            let pos_b = ftell(f);
            assert_eq!(pos_b, 4); // 3-byte BOM + 1-byte 'A'
            let mut gpos_b = 0i64;
            assert_eq!(fgetpos(f, &mut gpos_b), 0);
            assert_eq!(gpos_b, 4);
            assert_eq!(fgetwc(f), 'B' as u32);
            assert_eq!(fgetwc(f), 'C' as u32);
            assert_eq!(fgetwc(f), 'D' as u32);
            assert_eq!(fgetwc(f), 'α' as u32);
            let pos_beta = ftell(f);
            assert_eq!(pos_beta, 9); // 3 + 4 + 2('α')
            assert_eq!(fgetwc(f), 'β' as u32);
            assert_eq!(fgetwc(f), '\n' as u32);
            let pos_emoji = ftell(f);
            assert_eq!(pos_emoji, 13); // 9 + 2('β') + 2("\r\n")
            // Restore saved position before 'B' via fsetpos -> reads 'B' without BOM corruption.
            assert_eq!(fsetpos(f, &gpos_b), 0);
            assert_eq!(fgetwc(f), 'B' as u32);
            // Restore saved position before '😀' via fseek -> reads surrogate pair + 'Z'.
            assert_eq!(fseek(f, pos_emoji, 0), 0);
            let em_hi = fgetwc(f) as u16;
            let em_lo = fgetwc(f) as u16;
            let z_ch = fgetwc(f) as u16;
            assert_eq!(String::from_utf16_lossy(&[em_hi, em_lo, z_ch]), "😀Z");
            // Read-then-write (discard_read) on r+ ccs=UTF-8 repositions at exact physical offset.
            assert_eq!(fseek(f, pos_b, 0), 0);
            assert_eq!(fgetwc(f), 'B' as u32); // consumes 'B' (offset 5), buffers rest
            assert_eq!(fputwc('X' as u16, f), 'X' as u32); // overwrites 'C' at physical offset 5
            assert_eq!(fclose(f), 0);
            assert_eq!(std::fs::read(dir.join("bom_pos.txt")).unwrap(), "\u{FEFF}ABXDαβ\r\n😀Z".as_bytes());

            // 10. Non-Unicode wide text stdio honors the active LC_CTYPE code page (CP1252 / CP932).
            assert!(!crate::ucrt::locale::setlocale(0, b"de-DE\0".as_ptr()).is_null());
            let f = fopen(b"U:\\cp1252.txt\0".as_ptr(), b"w\0".as_ptr());
            assert!(!f.is_null());
            assert_eq!(fputwc('€' as u16, f), '€' as u32);
            assert_eq!(fputwc('A' as u16, f), 'A' as u32);
            let w_euro = crate::strings::to_wide_z("€B");
            assert_eq!(fputws(w_euro.as_ptr(), f), 0);
            assert_eq!(fclose(f), 0);
            // CP1252 encodes '€' as single byte 0x80 (not UTF-8 E2 82 AC).
            assert_eq!(std::fs::read(dir.join("cp1252.txt")).unwrap(), vec![0x80, b'A', 0x80, b'B']);

            let f = fopen(b"U:\\cp1252.txt\0".as_ptr(), b"r\0".as_ptr());
            assert!(!f.is_null());
            assert_eq!(fgetwc(f), '€' as u32);
            assert_eq!(fgetwc(f), 'A' as u32);
            assert_eq!(ungetwc('€' as u32, f), '€' as u32);
            assert_eq!(fgetwc(f), '€' as u32);
            assert_eq!(fgetwc(f), '€' as u32);
            assert_eq!(fgetwc(f), 'B' as u32);
            assert_eq!(fgetwc(f), WEOF);
            assert_eq!(fclose(f), 0);

            // DBCS locale (ja-JP -> CP932) roundtrip via fputwc/fgetwc.
            assert!(!crate::ucrt::locale::setlocale(0, b"ja-JP\0".as_ptr()).is_null());
            let f = fopen(b"U:\\cp932.txt\0".as_ptr(), b"w\0".as_ptr());
            assert!(!f.is_null());
            assert_eq!(fputwc('あ' as u16, f), 'あ' as u32);
            assert_eq!(fputwc('Z' as u16, f), 'Z' as u32);
            assert_eq!(fclose(f), 0);
            assert_eq!(std::fs::read(dir.join("cp932.txt")).unwrap(), vec![0x82, 0xA0, b'Z']);
            let f = fopen(b"U:\\cp932.txt\0".as_ptr(), b"r\0".as_ptr());
            assert!(!f.is_null());
            assert_eq!(fgetwc(f), 'あ' as u32);
            assert_eq!(fgetwc(f), 'Z' as u32);
            assert_eq!(fclose(f), 0);

            assert!(!crate::ucrt::locale::setlocale(0, b"C\0".as_ptr()).is_null());

            // 11. Non-seekable FIFO with _O_U16TEXT preserves CR lookahead (full and partial units)
            //     and translates CRLF across 2-byte reads without losing data.
            let fifo_path = dir.join("u16_lookahead.fifo");
            let fifo_c = std::ffi::CString::new(fifo_path.as_os_str().as_encoded_bytes()).unwrap();
            assert_eq!(libc::mkfifo(fifo_c.as_ptr(), 0o600), 0);
            // Open O_RDWR on the FIFO via _open with O_U16TEXT (non-seekable, raw=false).
            let fd_fifo = _open(b"U:\\u16_lookahead.fifo\0".as_ptr(), O_RDWR | O_U16TEXT, 0);
            assert!(fd_fifo >= 0);
            let w_fifo = libc::open(fifo_c.as_ptr(), libc::O_WRONLY);
            assert!(w_fifo >= 0);

            // Case A: UTF-16LE "\rAB" read 2 bytes at a time -> retains 'A' lookahead on ESPIPE.
            let payload_rab: Vec<u8> = "\rAB".encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
            assert_eq!(
                libc::write(w_fifo, payload_rab.as_ptr() as *const c_void, payload_rab.len()),
                payload_rab.len() as isize
            );
            let mut got_rab = Vec::new();
            for _ in 0..3 {
                let mut u = [0u8; 2];
                assert_eq!(_read(fd_fifo, u.as_mut_ptr() as *mut c_void, 2), 2);
                got_rab.push(u16::from_le_bytes(u));
            }
            assert_eq!(String::from_utf16_lossy(&got_rab), "\rAB");

            // Case B: UTF-16LE "\r\n\r\r\nC" read 2 bytes at a time -> "\n\r\nC".
            let payload_crlf: Vec<u8> = "\r\n\r\r\nC".encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
            assert_eq!(
                libc::write(w_fifo, payload_crlf.as_ptr() as *const c_void, payload_crlf.len()),
                payload_crlf.len() as isize
            );
            let mut got_crlf = Vec::new();
            for _ in 0..4 {
                let mut u = [0u8; 2];
                assert_eq!(_read(fd_fifo, u.as_mut_ptr() as *mut c_void, 2), 2);
                got_crlf.push(u16::from_le_bytes(u));
            }
            assert_eq!(String::from_utf16_lossy(&got_crlf), "\n\r\nC");

            // Case C: Partial UTF-16LE lookahead on closed pipe: "\r" + single byte 0x44, then EOF;
            // and split partial delivery on a live pipe ("\r" + 0x44, then 0x00 -> "\rD").
            let mut pfds16 = [0i32; 2];
            assert_eq!(libc::pipe(pfds16.as_mut_ptr()), 0);
            // Force raw=false on the pipe fd (matching open_host_path on a non-seekable descriptor).
            set_fd_info(
                pfds16[0],
                FdInfo { mode: TextMode::Utf16Le, win_mode: O_U16TEXT, raw: false, temporary: false, osf_handle: None },
                None,
            );
            let writer_fd = pfds16[1];
            let writer = std::thread::spawn(move || {
                // Send "\r" (2 bytes) + low byte of 'D' (0x44), pause, then send high byte (0x00) + close.
                let part1 = [0x0Du8, 0x00, b'D'];
                assert_eq!(libc::write(writer_fd, part1.as_ptr() as *const c_void, part1.len()), 3);
                std::thread::sleep(std::time::Duration::from_millis(15));
                let part2 = [0x00u8];
                assert_eq!(libc::write(writer_fd, part2.as_ptr() as *const c_void, 1), 1);
                libc::close(writer_fd);
            });
            let mut u1 = [0u8; 2];
            let mut u2 = [0u8; 2];
            assert_eq!(_read(pfds16[0], u1.as_mut_ptr() as *mut c_void, 2), 2);
            assert_eq!(_read(pfds16[0], u2.as_mut_ptr() as *mut c_void, 2), 2);
            assert_eq!(_read(pfds16[0], u2.as_mut_ptr() as *mut c_void, 2), 0);
            writer.join().unwrap();
            assert_eq!(String::from_utf16_lossy(&[u16::from_le_bytes(u1), u16::from_le_bytes(u2)]), "\rD");
            assert_eq!(_close(pfds16[0]), 0);

            libc::close(w_fifo);
            assert_eq!(_close(fd_fifo), 0);

            // Case D: Non-seekable ANSI text descriptor preserves CR lookahead ("\rXY").
            let mut pfds_a = [0i32; 2];
            assert_eq!(libc::pipe(pfds_a.as_mut_ptr()), 0);
            set_fd_info(
                pfds_a[0],
                FdInfo { mode: TextMode::AnsiText, win_mode: O_TEXT, raw: false, temporary: false, osf_handle: None },
                None,
            );
            assert_eq!(libc::write(pfds_a[1], b"\rXY".as_ptr() as *const c_void, 3), 3);
            libc::close(pfds_a[1]);
            let mut got_a = Vec::new();
            loop {
                let mut b = 0u8;
                let k = _read(pfds_a[0], &mut b as *mut u8 as *mut c_void, 1);
                if k <= 0 {
                    break;
                }
                got_a.push(b);
            }
            assert_eq!(got_a, b"\rXY");
            assert_eq!(_close(pfds_a[0]), 0);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
