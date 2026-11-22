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

use super::errno::{self, EBADF, EINVAL, ENOENT};
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
    let i = FdInfo { text: true, raw, temporary: false };
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
    let mut f = match oflag & 3 {
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
    set_fd_info(fd, FdInfo { text, raw: false, temporary: oflag & O_TEMPORARY != 0 }, Some(host.to_path_buf()));
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
    let r = libc::close(fd);
    if let Some((i, Some(p))) = ent {
        if i.temporary {
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

/// Raw read with CRLF->LF translation in text mode.
pub unsafe fn read_fd(fd: i32, buf: *mut u8, n: usize) -> isize {
    let info = fd_info(fd);
    let r = libc::read(fd, buf as *mut c_void, n);
    if r <= 0 || !info.text || info.raw {
        if r < 0 {
            errno::set_from_host();
        }
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
            if *libc::__error() == libc::EINTR {
                continue;
            }
            errno::set_from_host();
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
    set_fd_info(n, FdInfo { temporary: false, ..i }, None);
    n
}

pub unsafe extern "win64" fn _dup2(fd: i32, fd2: i32) -> i32 {
    if !fd_valid(fd) {
        return bad_fd();
    }
    if libc::dup2(fd, fd2) < 0 {
        errno::set_from_host();
        return -1;
    }
    let i = fd_info(fd);
    set_fd_info(fd2, FdInfo { temporary: false, ..i }, None);
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
    set_fd_info(fd, FdInfo { text: false, raw: false, temporary: false }, None);
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
}
