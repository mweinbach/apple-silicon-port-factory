//! Files, directories, FindFirstFile, file mappings, console handles and
//! DeviceIoControl, implemented on host files via the [`crate::path`] mount
//! table.
//!
//! File handles are [`FileObj`]s in the generic handle table, so
//! `CloseHandle` / `DuplicateHandle` work uniformly. Overlapped I/O is
//! completed synchronously (allowed by the API contract: ReadFile returns
//! TRUE, `OVERLAPPED.Internal*` are filled and `hEvent` is signalled).

use crate::handles::{self, Event, KObject};
use crate::kernel32::threadpool::TpIo;
use crate::path;
use crate::strings::*;
use crate::teb;
use crate::thread::ThreadObj;
use crate::types::*;
use std::any::Any;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::ffi::c_void;
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::unix::fs::{FileExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

fn set_err(e: u32) {
    teb::set_last_error(e);
}

fn io_err(e: &std::io::Error) -> u32 {
    e.raw_os_error().map(errno_to_win32).unwrap_or(err::ERROR_INVALID_FUNCTION)
}

pub const INVALID_HANDLE_VALUE: HANDLE = usize::MAX as HANDLE;
pub const INVALID_FILE_ATTRIBUTES: u32 = 0xFFFF_FFFF;
pub const FILE_ATTRIBUTE_READONLY: u32 = 0x1;
pub const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
pub const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
pub const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x20;
pub const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
pub const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;

const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;
const GENERIC_ALL: u32 = 0x1000_0000;
const FILE_READ_DATA: u32 = 0x1;
const FILE_WRITE_DATA: u32 = 0x2;
const FILE_APPEND_DATA: u32 = 0x4;
const FILE_WRITE_EA: u32 = 0x10;
const FILE_WRITE_ATTRIBUTES: u32 = 0x100;
const DELETE_ACCESS: u32 = 0x0001_0000;
const WRITE_DAC: u32 = 0x0004_0000;
const WRITE_OWNER: u32 = 0x0008_0000;

const CREATE_NEW: u32 = 1;
const CREATE_ALWAYS: u32 = 2;
const OPEN_EXISTING: u32 = 3;
const OPEN_ALWAYS: u32 = 4;
const TRUNCATE_EXISTING: u32 = 5;

const FILE_FLAG_DELETE_ON_CLOSE: u32 = 0x0400_0000;
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;

const FILE_TYPE_UNKNOWN: u32 = 0;
const FILE_TYPE_DISK: u32 = 1;
const FILE_TYPE_CHAR: u32 = 2;
const FILE_TYPE_PIPE: u32 = 3;

const STATUS_NOTIFY_ENUM_DIR: usize = 0x0000_010C;
const STATUS_PIPE_BROKEN: usize = 0xC000_014B;

const ERROR_SEM_TIMEOUT: u32 = 121;
const ERROR_PIPE_BUSY: u32 = 231;
const ERROR_PIPE_NOT_CONNECTED: u32 = 233;
const ERROR_PIPE_CONNECTED: u32 = 535;
const ERROR_PIPE_LISTENING: u32 = 536;
const ERROR_OPERATION_ABORTED: u32 = 995;
const ERROR_NOTIFY_ENUM_DIR: u32 = 1022;

#[cfg(target_vendor = "apple")]
const DARWIN_O_EVTONLY: libc::c_int = 0x8000;
#[cfg(target_vendor = "apple")]
const DARWIN_SO_NOSIGPIPE: libc::c_int = 0x1022;

fn ntstatus_to_win32_local(s: usize) -> u32 {
    match s {
        STATUS_SUCCESS | STATUS_RECEIVE_EXPEDITED => 0,
        STATUS_PENDING => err::ERROR_IO_PENDING,
        STATUS_NOTIFY_ENUM_DIR => ERROR_NOTIFY_ENUM_DIR,
        STATUS_END_OF_FILE => err::ERROR_HANDLE_EOF,
        STATUS_CANCELLED => ERROR_OPERATION_ABORTED,
        STATUS_PIPE_BROKEN => err::ERROR_BROKEN_PIPE,
        other => crate::ntdll::ntstatus_to_win32(other as u32),
    }
}

fn set_cloexec(fd: i32) {
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFD);
        if flags >= 0 {
            let _ = libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC);
        }
    }
}

fn set_nosigpipe_sock(fd: i32) {
    #[cfg(target_vendor = "apple")]
    unsafe {
        let one: libc::c_int = 1;
        let _ = libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            DARWIN_SO_NOSIGPIPE,
            &one as *const _ as *const c_void,
            std::mem::size_of::<libc::c_int>() as libc::socklen_t,
        );
    }
    #[cfg(not(target_vendor = "apple"))]
    let _ = fd;
}

/// Extract `HANDLE_FLAG_INHERIT` from a Win64 `SECURITY_ATTRIBUTES` pointer
/// (`bInheritHandle: BOOL` sits at offset 16 on x64).
unsafe fn sec_attr_inherit_flags(sec: *mut c_void) -> u32 {
    if sec.is_null() {
        return 0;
    }
    let inherit = std::ptr::read_unaligned((sec as *const u8).add(16) as *const BOOL);
    if inherit != 0 {
        handles::HANDLE_FLAG_INHERIT
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// File objects
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Disk,
    Dir,
    Console,
    Pipe,
    Null,
    /// `\\.\PhysicalDriveN` / `\\.\C:` style volume handle (DeviceIoControl only).
    Device,
}

struct DirEnumEntry {
    name: String,
    attrs: u32,
    times: Times,
    size: u64,
    alloc_size: u64,
    ino: u64,
    reparse_tag: u32,
}

struct DirEnumState {
    entries: Vec<DirEnumEntry>,
    cursor: usize,
}

pub struct FileObj {
    file: ManuallyDrop<File>,
    owned: bool,
    pub kind: FileKind,
    /// Windows path the file was opened with (absolute).
    pub win_path: String,
    pub host_path: Option<PathBuf>,
    pub access: u32,
    pub flags: u32,
    pub append: bool,
    delete_on_close: Mutex<bool>,
    completion: Arc<IoCompletionBinding>,
    /// Unread peek buffer for pipes (`PeekNamedPipe` fills; `ReadFile`/`ReadFileEx` drains first).
    peek_buf: Mutex<VecDeque<u8>>,
    /// Lazily started kqueue directory watcher for `ReadDirectoryChangesW`.
    dir_watcher: Mutex<Option<Arc<DirWatcher>>>,
    /// Present for server-side named pipe handles created via `CreateNamedPipeW`/`A`.
    named_pipe_server: Option<Arc<NamedPipeServer>>,
    /// Stateful directory enumeration cursor for `GetFileInformationByHandleEx` dir classes.
    dir_enum: Mutex<Option<DirEnumState>>,
    /// Per-handle I/O priority hint (`FILE_IO_PRIORITY_HINT_INFO`, default `2` = `IoPriorityNormal`).
    io_priority: Mutex<u32>,
}

impl FileObj {
    fn new(file: File, kind: FileKind, win_path: String, host_path: Option<PathBuf>, access: u32, flags: u32) -> Self {
        FileObj {
            file: ManuallyDrop::new(file),
            owned: true,
            kind,
            win_path,
            host_path,
            access,
            flags,
            append: false,
            delete_on_close: Mutex::new(flags & FILE_FLAG_DELETE_ON_CLOSE != 0),
            completion: Arc::new(IoCompletionBinding::new()),
            peek_buf: Mutex::new(VecDeque::new()),
            dir_watcher: Mutex::new(None),
            named_pipe_server: None,
            dir_enum: Mutex::new(None),
            io_priority: Mutex::new(2),
        }
    }
    pub fn fd(&self) -> i32 {
        if let Some(ref srv) = self.named_pipe_server {
            return srv.connected_fd().unwrap_or(-1);
        }
        self.file.as_raw_fd()
    }
    pub fn file(&self) -> &File {
        &self.file
    }
    /// True if this file handle refers to a protected native host runner/bundle file or
    /// ancestor directory (verified by both stored host path and live POSIX fd inode/`F_GETPATH`).
    fn is_protected_native(&self) -> bool {
        if self.kind != FileKind::Disk && self.kind != FileKind::Dir {
            return false;
        }
        self.host_path
            .as_deref()
            .is_some_and(path::is_protected_host_remove_or_rename)
            || path::is_protected_fd(self.fd())
    }
    /// Return the active raw POSIX fd and optional shared named-pipe connection owner
    /// for pipe I/O, or a Win32 pipe state error if this is an unconnected named-pipe server.
    fn io_pipe_target(&self) -> Result<PipeIoTarget, u32> {
        if let Some(ref srv) = self.named_pipe_server {
            srv.io_conn().map(PipeIoTarget::ServerConn)
        } else {
            Ok(PipeIoTarget::Direct(self.file.as_raw_fd()))
        }
    }
    fn cancel_pending_io(&self, h: HANDLE, ov: Option<usize>, filter_tid: Option<u32>) -> usize {
        let mut count = 0usize;
        if let Some(w) = self.dir_watcher.lock().unwrap().clone() {
            count += w.cancel_requests(h as usize, ov, filter_tid);
        }
        count += pipe_reactor().cancel_ops(h as usize, ov, filter_tid);
        if let Some(ref srv) = self.named_pipe_server {
            count += srv.cancel_pending_connect(h as usize, ov, filter_tid);
        }
        count
    }
}

impl Drop for FileObj {
    fn drop(&mut self) {
        if let Some(w) = self.dir_watcher.lock().unwrap().take() {
            w.shutdown();
        }
        if let Some(ref srv) = self.named_pipe_server {
            srv.close_server();
        }
        if *self.delete_on_close.lock().unwrap() {
            if let Some(p) = &self.host_path {
                if !path::is_protected_host_remove_or_rename(p) {
                    let _ = if self.kind == FileKind::Dir { std::fs::remove_dir(p) } else { std::fs::remove_file(p) };
                }
            }
        }
        if self.owned {
            // SAFETY: dropped exactly once here.
            unsafe { ManuallyDrop::drop(&mut self.file) };
        }
    }
}

impl KObject for FileObj {
    fn type_name(&self) -> &'static str {
        "File"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn handles::Waitable> {
        Some(&*self.completion.io_event)
    }
}

/// Called by `CloseHandle` before the generic table removes the handle.
/// Cancels pending async I/O (directory watches, async pipe ops, pending
/// named-pipe accepts) while `h` is still live in the handle table so
/// `ERROR_OPERATION_ABORTED` completions still reach bound IOCP / `TpIo` sinks.
pub fn close_handle_hook(h: HANDLE) -> bool {
    if handles::is_protected_from_close(h) {
        return false;
    }
    if let Some(f) = get_file(h) {
        let _ = f.cancel_pending_io(h, None, None);
    }
    false
}

pub fn get_file(h: HANDLE) -> Option<Arc<FileObj>> {
    let h = resolve_std_pseudo(h);
    handles::get_as::<FileObj>(h)
}

/// Return `(raw_fd, FileKind)` for a `FileObj` handle (used by process launch & CRT `_open_osfhandle`).
pub(crate) fn file_fd_and_kind(h: HANDLE) -> Option<(i32, FileKind)> {
    get_file(h).map(|f| (f.fd(), f.kind))
}

/// Wrap an owned POSIX fd in an owning `FileObj` handle (used by CRT `_get_osfhandle`).
pub(crate) fn wrap_owned_fd(
    fd: std::os::fd::OwnedFd,
    kind: FileKind,
    win_path: String,
    access: u32,
) -> HANDLE {
    let o = FileObj::new(File::from(fd), kind, win_path, None, access, 0);
    handles::insert(Arc::new(o))
}

/// Wrap an existing POSIX fd in a non-owning `FileObj` handle (used by tests and non-owning wrappers).
#[allow(dead_code)]
pub(crate) fn wrap_non_owned_fd(fd: i32, kind: FileKind, win_path: String, access: u32) -> HANDLE {
    let mut o = FileObj::new(
        unsafe { File::from_raw_fd(fd) },
        kind,
        win_path,
        None,
        access,
        0,
    );
    o.owned = false;
    handles::insert(Arc::new(o))
}

/// Classify a POSIX fd into a `FileKind`.
pub(crate) fn classify_host_fd(fd: i32) -> FileKind {
    if unsafe { libc::isatty(fd) } == 1 {
        FileKind::Console
    } else {
        host_fd_kind(fd)
    }
}

/// Windows lets `GetStdHandle` values be passed directly; on our side they
/// are real table handles, but some code passes the STD_* constants by
/// mistake to WriteFile — Windows fails those, so do we.
fn resolve_std_pseudo(h: HANDLE) -> HANDLE {
    h
}

// ---------------------------------------------------------------------------
// Std handles / console
// ---------------------------------------------------------------------------

const STD_INPUT_HANDLE: u32 = -10i32 as u32;
const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
const STD_ERROR_HANDLE: u32 = -12i32 as u32;

fn std_handles() -> &'static Mutex<[HANDLE; 3]> {
    struct S(Mutex<[HANDLE; 3]>);
    unsafe impl Sync for S {}
    unsafe impl Send for S {}
    static H: OnceLock<S> = OnceLock::new();
    &H.get_or_init(|| {
        let mut hs = [std::ptr::null_mut(); 3];
        for (fd, slot) in hs.iter_mut().enumerate() {
            let tty = unsafe { libc::isatty(fd as i32) } == 1;
            let kind = if tty { FileKind::Console } else { host_fd_kind(fd as i32) };
            let mut o = FileObj::new(
                unsafe { File::from_raw_fd(fd as i32) },
                kind,
                ["CONIN$", "CONOUT$", "CONOUT$"][fd].into(),
                None,
                if fd == 0 { GENERIC_READ } else { GENERIC_WRITE },
                0,
            );
            o.owned = false;
            *slot = handles::insert(Arc::new(o));
        }
        S(Mutex::new(hs))
    })
    .0
}

fn host_fd_kind(fd: i32) -> FileKind {
    let mut st: libc::stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::fstat(fd, &mut st) } != 0 {
        return FileKind::Null;
    }
    match st.st_mode & libc::S_IFMT {
        libc::S_IFIFO | libc::S_IFSOCK => FileKind::Pipe,
        // `host_fd_kind` is only called for non-TTY fds; non-TTY character
        // devices like `/dev/null` are `FILE_TYPE_CHAR`, not Windows consoles.
        libc::S_IFCHR => FileKind::Null,
        libc::S_IFDIR => FileKind::Dir,
        _ => FileKind::Disk,
    }
}

pub fn std_handle(fd: usize) -> HANDLE {
    std_handles().lock().unwrap()[fd]
}

pub unsafe extern "win64" fn GetStdHandle(which: u32) -> HANDLE {
    let i = match which {
        STD_INPUT_HANDLE => 0,
        STD_OUTPUT_HANDLE => 1,
        STD_ERROR_HANDLE => 2,
        _ => {
            set_err(err::ERROR_INVALID_HANDLE);
            return INVALID_HANDLE_VALUE;
        }
    };
    std_handles().lock().unwrap()[i]
}

pub unsafe extern "win64" fn SetStdHandle(which: u32, h: HANDLE) -> BOOL {
    let i = match which {
        STD_INPUT_HANDLE => 0,
        STD_OUTPUT_HANDLE => 1,
        STD_ERROR_HANDLE => 2,
        _ => {
            set_err(err::ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };
    std_handles().lock().unwrap()[i] = h;
    TRUE
}

struct ConsoleSession {
    attached: bool,
    in_mode: u32,
    out_mode: u32,
    in_cp: u32,
    out_cp: u32,
    title: String,
    /// Handles created by the active `AllocConsole` / `AttachConsole` session so `FreeConsole` can detach cleanly.
    session_handles: Option<[usize; 3]>,
}

fn console_session() -> &'static Mutex<ConsoleSession> {
    static S: OnceLock<Mutex<ConsoleSession>> = OnceLock::new();
    S.get_or_init(|| {
        Mutex::new(ConsoleSession {
            attached: false,
            // ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT
            in_mode: 0x0007,
            // ENABLE_PROCESSED_OUTPUT | ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_VIRTUAL_TERMINAL_PROCESSING
            out_mode: 0x0007,
            in_cp: ACP,
            out_cp: ACP,
            title: String::new(),
            session_handles: None,
        })
    })
}

/// Active POSIX fd backing console / standard input (used by `ucrt::_getch` and `_kbhit`).
pub(crate) fn console_input_fd() -> i32 {
    let h = std_handle(0);
    get_file(h).map(|f| f.fd()).filter(|&fd| fd >= 0).unwrap_or(0)
}

#[cfg(target_os = "macos")]
fn open_console_pty() -> Option<(std::os::fd::OwnedFd, std::os::fd::OwnedFd)> {
    use std::os::fd::OwnedFd;
    unsafe {
        let master = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
        if master < 0 {
            return None;
        }
        set_cloexec(master);
        let master_fd = OwnedFd::from_raw_fd(master);
        if libc::grantpt(master) != 0 || libc::unlockpt(master) != 0 {
            return None;
        }
        let name = libc::ptsname(master);
        if name.is_null() {
            return None;
        }
        let slave = libc::open(name, libc::O_RDWR | libc::O_NOCTTY | libc::O_CLOEXEC);
        if slave < 0 {
            return None;
        }
        let slave_fd = OwnedFd::from_raw_fd(slave);
        Some((master_fd, slave_fd))
    }
}

fn is_console(h: HANDLE) -> bool {
    get_file(h).is_some_and(|f| f.kind == FileKind::Console)
}

pub unsafe extern "win64" fn GetConsoleMode(h: HANDLE, mode: *mut u32) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if f.kind != FileKind::Console {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    if !mode.is_null() {
        let st = console_session().lock().unwrap();
        let val = if (f.access & GENERIC_READ) != 0 && (f.access & GENERIC_WRITE) == 0 {
            st.in_mode
        } else {
            st.out_mode
        };
        std::ptr::write_unaligned(mode, val);
    }
    TRUE
}

pub unsafe extern "win64" fn SetConsoleMode(h: HANDLE, mode: u32) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if f.kind != FileKind::Console {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let mut st = console_session().lock().unwrap();
    if (f.access & GENERIC_READ) != 0 && (f.access & GENERIC_WRITE) == 0 {
        st.in_mode = mode;
    } else {
        st.out_mode = mode;
    }
    TRUE
}

pub unsafe extern "win64" fn GetNumberOfConsoleInputEvents(h: HANDLE, count: *mut u32) -> BOOL {
    if count.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if f.kind != FileKind::Console || (f.access & GENERIC_READ) == 0 {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let mut avail: libc::c_int = 0;
    let n = if libc::ioctl(f.fd(), libc::FIONREAD, &mut avail) == 0 && avail > 0 {
        avail as u32
    } else {
        0
    };
    std::ptr::write_unaligned(count, n);
    TRUE
}

pub unsafe extern "win64" fn GetConsoleCP() -> u32 {
    console_session().lock().unwrap().in_cp
}
pub unsafe extern "win64" fn GetConsoleOutputCP() -> u32 {
    console_session().lock().unwrap().out_cp
}
pub unsafe extern "win64" fn SetConsoleCP(cp: u32) -> BOOL {
    if !crate::kernel32::string::valid_cp(cp) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    console_session().lock().unwrap().in_cp = crate::kernel32::string::resolve_cp(cp);
    TRUE
}
pub unsafe extern "win64" fn SetConsoleOutputCP(cp: u32) -> BOOL {
    if !crate::kernel32::string::valid_cp(cp) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    console_session().lock().unwrap().out_cp = crate::kernel32::string::resolve_cp(cp);
    TRUE
}

/// Allocate a real POSIX PTY-backed console session (on macOS; pipe/stdio-backed on iOS)
/// and bind `STD_INPUT_HANDLE`/`STD_OUTPUT_HANDLE`/`STD_ERROR_HANDLE` to `FileKind::Console`
/// handles. Matches Windows: a process may own at most one console, so a second `AllocConsole`
/// without `FreeConsole` fails with `ERROR_ACCESS_DENIED`.
pub unsafe extern "win64" fn AllocConsole() -> BOOL {
    let mut st = console_session().lock().unwrap();
    if st.attached {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    let new_handles: [HANDLE; 3] = {
        #[cfg(target_os = "macos")]
        {
            if let Some((master_fd, slave_fd)) = open_console_pty() {
                use std::os::fd::AsRawFd;
                let sfd = slave_fd.as_raw_fd();
                // Keep console input connected to host stdin (dup of fd 0) so ReadConsole/_getch
                // can receive interactive/piped host input instead of blocking on an unfed PTY master,
                // while console stdout/stderr write to the real PTY slave drained to host stderr.
                let in_dup = libc::fcntl(0, libc::F_DUPFD_CLOEXEC, 3);
                let out_dup = libc::fcntl(sfd, libc::F_DUPFD_CLOEXEC, 3);
                let err_dup = libc::fcntl(sfd, libc::F_DUPFD_CLOEXEC, 3);
                drop(slave_fd);
                if in_dup >= 0 && out_dup >= 0 && err_dup >= 0 {
                    // Drain PTY master output to host stderr so guest console writes never block
                    // on PTY buffer capacity and remain visible in the host process logs.
                    let _ = std::thread::Builder::new()
                        .name("win32-console-pty".into())
                        .spawn(move || {
                            use std::io::Write;
                            let mfd = master_fd.as_raw_fd();
                            let mut buf = [0u8; 4096];
                            loop {
                                let n = libc::read(mfd, buf.as_mut_ptr() as *mut c_void, buf.len());
                                if n <= 0 {
                                    break;
                                }
                                let _ = std::io::stderr().write_all(&buf[..n as usize]);
                            }
                            drop(master_fd);
                        });
                    let h_in = handles::insert(Arc::new(FileObj::new(
                        File::from_raw_fd(in_dup),
                        FileKind::Console,
                        "CONIN$".into(),
                        None,
                        GENERIC_READ,
                        0,
                    )));
                    let h_out = handles::insert(Arc::new(FileObj::new(
                        File::from_raw_fd(out_dup),
                        FileKind::Console,
                        "CONOUT$".into(),
                        None,
                        GENERIC_WRITE,
                        0,
                    )));
                    let h_err = handles::insert(Arc::new(FileObj::new(
                        File::from_raw_fd(err_dup),
                        FileKind::Console,
                        "CONOUT$".into(),
                        None,
                        GENERIC_WRITE,
                        0,
                    )));
                    [h_in, h_out, h_err]
                } else {
                    for fd in [in_dup, out_dup, err_dup] {
                        if fd >= 0 {
                            libc::close(fd);
                        }
                    }
                    set_err(errno_to_win32(last_errno()));
                    return FALSE;
                }
            } else {
                // Fallback if PTY limit reached: bind console handles to dup'd host stdio.
                let mk = |fd: i32, name: &str, access: u32| -> Option<HANDLE> {
                    let d = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3);
                    if d < 0 {
                        return None;
                    }
                    Some(handles::insert(Arc::new(FileObj::new(
                        File::from_raw_fd(d),
                        FileKind::Console,
                        name.into(),
                        None,
                        access,
                        0,
                    ))))
                };
                match (mk(0, "CONIN$", GENERIC_READ), mk(1, "CONOUT$", GENERIC_WRITE), mk(2, "CONOUT$", GENERIC_WRITE)) {
                    (Some(a), Some(b), Some(c)) => [a, b, c],
                    _ => {
                        set_err(errno_to_win32(last_errno()));
                        return FALSE;
                    }
                }
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            let mk = |fd: i32, name: &str, access: u32| -> Option<HANDLE> {
                let d = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3);
                if d < 0 {
                    return None;
                }
                Some(handles::insert(Arc::new(FileObj::new(
                    File::from_raw_fd(d),
                    FileKind::Console,
                    name.into(),
                    None,
                    access,
                    0,
                ))))
            };
            match (mk(0, "CONIN$", GENERIC_READ), mk(1, "CONOUT$", GENERIC_WRITE), mk(2, "CONOUT$", GENERIC_WRITE)) {
                (Some(a), Some(b), Some(c)) => [a, b, c],
                _ => {
                    set_err(errno_to_win32(last_errno()));
                    return FALSE;
                }
            }
        }
    };
    {
        let mut hs = std_handles().lock().unwrap();
        *hs = new_handles;
    }
    st.attached = true;
    st.in_mode = 0x0007;
    st.out_mode = 0x0007;
    st.session_handles = Some([new_handles[0] as usize, new_handles[1] as usize, new_handles[2] as usize]);
    TRUE
}

pub unsafe extern "win64" fn FreeConsole() -> BOOL {
    let mut st = console_session().lock().unwrap();
    if !st.attached {
        // Microsoft Learn: FreeConsole succeeds even if the process is not attached to a console.
        return TRUE;
    }
    st.attached = false;
    if let Some(old) = st.session_handles.take() {
        let mut hs = std_handles().lock().unwrap();
        for (i, &h_raw) in old.iter().enumerate() {
            let h = h_raw as HANDLE;
            if hs[i] == h {
                // Restore non-owning host stdio fallback handle so post-FreeConsole writes don't crash.
                let fd = i as i32;
                let tty = libc::isatty(fd) == 1;
                let kind = if tty { FileKind::Console } else { host_fd_kind(fd) };
                let mut o = FileObj::new(
                    File::from_raw_fd(fd),
                    kind,
                    ["CONIN$", "CONOUT$", "CONOUT$"][i].into(),
                    None,
                    if i == 0 { GENERIC_READ } else { GENERIC_WRITE },
                    0,
                );
                o.owned = false;
                hs[i] = handles::insert(Arc::new(o));
            }
            let _ = handles::close(h);
        }
    }
    TRUE
}

pub unsafe extern "win64" fn AttachConsole(pid: u32) -> BOOL {
    const ATTACH_PARENT_PROCESS: u32 = 0xFFFF_FFFF;
    let mut st = console_session().lock().unwrap();
    if st.attached {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    if pid != ATTACH_PARENT_PROCESS && pid != std::process::id() {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let mk = |fd: i32, name: &str, access: u32| -> Option<HANDLE> {
        let d = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3);
        if d < 0 {
            return None;
        }
        Some(handles::insert(Arc::new(FileObj::new(
            File::from_raw_fd(d),
            FileKind::Console,
            name.into(),
            None,
            access,
            0,
        ))))
    };
    let (Some(a), Some(b), Some(c)) = (
        mk(0, "CONIN$", GENERIC_READ),
        mk(1, "CONOUT$", GENERIC_WRITE),
        mk(2, "CONOUT$", GENERIC_WRITE),
    ) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    {
        let mut hs = std_handles().lock().unwrap();
        *hs = [a, b, c];
    }
    st.attached = true;
    st.session_handles = Some([a as usize, b as usize, c as usize]);
    TRUE
}
pub unsafe extern "win64" fn GetConsoleWindow() -> HWND {
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn GetConsoleScreenBufferInfo(h: HANDLE, info: *mut u16) -> BOOL {
    if !is_console(h) {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    // Query real terminal window size via TIOCGWINSZ when available.
    let (cols, rows) = get_file(h)
        .and_then(|f| {
            let mut ws: libc::winsize = std::mem::zeroed();
            if libc::ioctl(f.fd(), libc::TIOCGWINSZ, &mut ws) == 0 && ws.ws_col > 0 && ws.ws_row > 0 {
                Some((ws.ws_col, ws.ws_row))
            } else {
                None
            }
        })
        .unwrap_or((120, 50));
    // COORD dwSize, COORD cursor, WORD attr, SMALL_RECT window, COORD max
    let vals: [u16; 11] = [cols, rows.max(9000), 0, 0, 7, 0, 0, cols.saturating_sub(1), rows.saturating_sub(1), cols, rows];
    std::ptr::copy_nonoverlapping(vals.as_ptr() as *const u8, info as *mut u8, 22);
    TRUE
}

pub unsafe extern "win64" fn SetConsoleTextAttribute(_h: HANDLE, _a: u16) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetConsoleTitleA(t: LPCSTR) -> BOOL {
    let s = from_ansi(t);
    log::info!("SetConsoleTitle: {s}");
    console_session().lock().unwrap().title = s;
    TRUE
}
pub unsafe extern "win64" fn SetConsoleTitleW(t: LPCWSTR) -> BOOL {
    let s = from_wide(t);
    log::info!("SetConsoleTitle: {s}");
    console_session().lock().unwrap().title = s;
    TRUE
}
pub unsafe extern "win64" fn SetConsoleScreenBufferSize(_h: HANDLE, _c: u32) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetConsoleWindowInfo(_h: HANDLE, _abs: BOOL, _r: *const u16) -> BOOL {
    TRUE
}
struct ConsoleCtrlState {
    handlers: Vec<usize>,
    ignore_ctrl_c: bool,
}

fn console_ctrl_state() -> &'static Mutex<ConsoleCtrlState> {
    static S: OnceLock<Mutex<ConsoleCtrlState>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(ConsoleCtrlState { handlers: Vec::new(), ignore_ctrl_c: false }))
}

pub unsafe extern "win64" fn SetConsoleCtrlHandler(handler: *const c_void, add: BOOL) -> BOOL {
    let mut st = console_ctrl_state().lock().unwrap();
    if handler.is_null() {
        st.ignore_ctrl_c = add != 0;
        return TRUE;
    }
    let addr = handler as usize;
    if add != 0 {
        st.handlers.push(addr);
        TRUE
    } else if let Some(pos) = st.handlers.iter().rposition(|&h| h == addr) {
        st.handlers.remove(pos);
        TRUE
    } else {
        set_err(err::ERROR_INVALID_PARAMETER);
        FALSE
    }
}

pub unsafe extern "win64" fn GenerateConsoleCtrlEvent(ctrl_event: u32, process_group_id: u32) -> BOOL {
    const CTRL_C_EVENT: u32 = 0;
    const CTRL_BREAK_EVENT: u32 = 1;
    if ctrl_event != CTRL_C_EVENT && ctrl_event != CTRL_BREAK_EVENT {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if process_group_id != 0 && process_group_id != std::process::id() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let snapshot: Vec<usize> = {
        let st = console_ctrl_state().lock().unwrap();
        if ctrl_event == CTRL_C_EVENT && st.ignore_ctrl_c {
            return TRUE;
        }
        st.handlers.iter().rev().copied().collect()
    };
    for func in snapshot {
        let handled = crate::call_guest8(func, ctrl_event as u64, 0, 0, 0, 0, 0, 0, 0) as BOOL;
        if handled != 0 {
            break;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn WriteConsoleW(h: HANDLE, buf: *const u16, n: u32, written: *mut u32, _r: *mut c_void) -> BOOL {
    let s = String::from_utf16_lossy(std::slice::from_raw_parts(buf, n as usize));
    let ok = WriteFile(h, s.as_ptr() as _, s.len() as u32, std::ptr::null_mut(), std::ptr::null_mut());
    if !written.is_null() {
        *written = if ok != 0 { n } else { 0 };
    }
    ok
}

pub unsafe extern "win64" fn WriteConsoleA(h: HANDLE, buf: *const u8, n: u32, written: *mut u32, _r: *mut c_void) -> BOOL {
    WriteFile(h, buf as _, n, written, std::ptr::null_mut())
}

pub unsafe extern "win64" fn ReadConsoleA(h: HANDLE, buf: *mut u8, n: u32, read: *mut u32, _c: *mut c_void) -> BOOL {
    ReadFile(h, buf as _, n, read, std::ptr::null_mut())
}

pub unsafe extern "win64" fn ReadConsoleW(h: HANDLE, buf: *mut u16, n: u32, read: *mut u32, _c: *mut c_void) -> BOOL {
    let mut tmp = vec![0u8; n as usize];
    let mut got = 0u32;
    if ReadFile(h, tmp.as_mut_ptr() as _, n, &mut got, std::ptr::null_mut()) == 0 {
        return FALSE;
    }
    let w = to_wide(&String::from_utf8_lossy(&tmp[..got as usize]));
    let k = w.len().min(n as usize);
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, k);
    if !read.is_null() {
        *read = k as u32;
    }
    TRUE
}

pub unsafe extern "win64" fn AreFileApisANSI() -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetFileApisToANSI() {}
pub unsafe extern "win64" fn SetHandleCount(n: u32) -> u32 {
    n
}

// ---------------------------------------------------------------------------
// CreateFile
// ---------------------------------------------------------------------------

/// Host path for a Windows path, or a Win32 error.
pub fn host_path(win: &str) -> Result<PathBuf, u32> {
    path::to_host(win).ok_or(err::ERROR_PATH_NOT_FOUND)
}

fn parent_exists(p: &Path) -> bool {
    p.parent().map_or(true, |d| d.is_dir())
}

fn not_found_err(p: &Path) -> u32 {
    if parent_exists(p) {
        err::ERROR_FILE_NOT_FOUND
    } else {
        err::ERROR_PATH_NOT_FOUND
    }
}

fn is_raw_device(name: &str) -> bool {
    let n = name.replace('/', "\\");
    let up = n.to_ascii_uppercase();
    up.starts_with("\\\\.\\PHYSICALDRIVE")
        || (up.starts_with("\\\\.\\") && up.len() == 6 && up.as_bytes()[5] == b':')
        || up.starts_with("\\\\?\\VOLUME{")
}

fn null_file() -> File {
    File::options().read(true).write(true).open("/dev/null").expect("/dev/null")
}

pub fn create_file(name: &str, access: u32, disposition: u32, flags: u32) -> Result<HANDLE, u32> {
    create_file_inner(name, access, disposition, flags, 0)
}

fn create_file_inner(name: &str, access: u32, disposition: u32, flags: u32, handle_flags: u32) -> Result<HANDLE, u32> {
    let wants_read = access & (GENERIC_READ | GENERIC_ALL | FILE_READ_DATA) != 0;
    let wants_write = access & (GENERIC_WRITE | GENERIC_ALL | FILE_WRITE_DATA | FILE_APPEND_DATA) != 0;

    if let Some(norm) = normalize_pipe_name(name) {
        return connect_named_pipe_client(&norm, name, access, flags, handle_flags);
    }

    if is_raw_device(name) {
        log::debug!("CreateFile: device handle for {name}");
        let o = FileObj::new(null_file(), FileKind::Device, name.into(), None, access, flags);
        return Ok(handles::insert_with_flags(Arc::new(o), handle_flags));
    }
    if let Some(dev) = path::device_name(name) {
        let (file, kind) = match dev {
            "NUL" => (null_file(), FileKind::Null),
            "CONIN$" => (unsafe { File::from_raw_fd(libc::dup(0)) }, FileKind::Console),
            _ => (unsafe { File::from_raw_fd(libc::dup(if wants_read && !wants_write { 0 } else { 1 })) }, FileKind::Console),
        };
        let o = FileObj::new(file, kind, dev.into(), None, access, flags);
        return Ok(handles::insert_with_flags(Arc::new(o), handle_flags));
    }

    let win = path::absolute(name);
    let host = host_path(&win)?;
    let exists = std::fs::symlink_metadata(&host).is_ok();
    let is_dir = host.is_dir();

    let mutating_access = wants_write
        || (access & (DELETE_ACCESS | WRITE_DAC | WRITE_OWNER | FILE_WRITE_ATTRIBUTES | FILE_WRITE_EA)) != 0;
    let mutating_disp = matches!(disposition, CREATE_NEW | CREATE_ALWAYS | TRUNCATE_EXISTING)
        || (disposition == OPEN_ALWAYS && !exists);
    let delete_on_close = (flags & FILE_FLAG_DELETE_ON_CLOSE) != 0;
    if (mutating_access || mutating_disp || delete_on_close) && path::is_protected_host_write(&host) {
        return Err(err::ERROR_ACCESS_DENIED);
    }
    if (delete_on_close || (access & DELETE_ACCESS) != 0) && path::is_protected_host_remove_or_rename(&host) {
        return Err(err::ERROR_ACCESS_DENIED);
    }

    if is_dir {
        if flags & FILE_FLAG_BACKUP_SEMANTICS == 0 {
            return Err(err::ERROR_ACCESS_DENIED);
        }
        if disposition == CREATE_NEW {
            return Err(err::ERROR_FILE_EXISTS);
        }
        let f = File::open(&host).map_err(|e| io_err(&e))?;
        let o = FileObj::new(f, FileKind::Dir, win, Some(host), access, flags);
        return Ok(handles::insert_with_flags(Arc::new(o), handle_flags));
    }

    let mut oo = File::options();
    oo.read(wants_read || !wants_write);
    oo.write(wants_write);
    match disposition {
        CREATE_NEW => {
            if exists {
                return Err(err::ERROR_FILE_EXISTS);
            }
            oo.write(true).create_new(true);
        }
        CREATE_ALWAYS => {
            oo.write(true).create(true).truncate(true);
        }
        OPEN_EXISTING => {
            if !exists {
                return Err(not_found_err(&host));
            }
        }
        OPEN_ALWAYS => {
            if wants_write {
                oo.create(true);
            } else if !exists {
                // Need write permission on the fd to create; reopen read-only after.
                File::options().write(true).create(true).truncate(false).open(&host).map_err(|e| io_err(&e))?;
            }
        }
        TRUNCATE_EXISTING => {
            if !exists {
                return Err(not_found_err(&host));
            }
            oo.write(true).truncate(true);
        }
        _ => return Err(err::ERROR_INVALID_PARAMETER),
    }
    if !parent_exists(&host) {
        return Err(err::ERROR_PATH_NOT_FOUND);
    }
    oo.mode(0o644);
    let f = oo.open(&host).map_err(|e| match e.raw_os_error() {
        Some(libc::ENOENT) => not_found_err(&host),
        _ => io_err(&e),
    })?;
    let mut o = FileObj::new(f, FileKind::Disk, win, Some(host), access, flags);
    o.append = access & FILE_APPEND_DATA != 0 && access & (FILE_WRITE_DATA | GENERIC_WRITE) == 0;
    let h = handles::insert_with_flags(Arc::new(o), handle_flags);
    // Windows reports ERROR_ALREADY_EXISTS on success for these dispositions.
    set_err(if exists && (disposition == CREATE_ALWAYS || disposition == OPEN_ALWAYS) {
        err::ERROR_ALREADY_EXISTS
    } else {
        0
    });
    Ok(h)
}

pub unsafe extern "win64" fn CreateFileW(
    name: LPCWSTR,
    access: u32,
    _share: u32,
    sec: *mut c_void,
    disposition: u32,
    flags: u32,
    _template: HANDLE,
) -> HANDLE {
    let n = from_wide(name);
    if let Some(h) = crate::misc::hid::create_file(&n, access, flags) {
        return h;
    }
    let hflags = sec_attr_inherit_flags(sec);
    match create_file_inner(&n, access, disposition, flags, hflags) {
        Ok(h) => {
            log::debug!("CreateFileW({n:?}, {access:#x}, disp={disposition}) -> {h:p}");
            h
        }
        Err(e) => {
            if e != err::ERROR_FILE_NOT_FOUND && e != err::ERROR_PATH_NOT_FOUND {
                log::warn!("CreateFileW({n:?}, {access:#x}, disp={disposition}, flags={flags:#x}) failed: err={e}");
            } else {
                log::debug!("CreateFileW({n:?}, {access:#x}, disp={disposition}) failed: {e}");
            }
            set_err(e);
            INVALID_HANDLE_VALUE
        }
    }
}

pub unsafe extern "win64" fn CreateFileA(
    name: LPCSTR,
    access: u32,
    share: u32,
    sec: *mut c_void,
    disposition: u32,
    flags: u32,
    template: HANDLE,
) -> HANDLE {
    let w = to_wide_z(&from_ansi(name));
    CreateFileW(w.as_ptr(), access, share, sec, disposition, flags, template)
}

pub unsafe extern "win64" fn CreateFile2(name: LPCWSTR, access: u32, share: u32, disposition: u32, params: *const u32) -> HANDLE {
    // CREATEFILE2_EXTENDED_PARAMETERS { dwSize, dwFileAttributes, dwFileFlags, ... }
    let flags = if params.is_null() { 0 } else { *params.add(1) | *params.add(2) };
    CreateFileW(name, access, share, std::ptr::null_mut(), disposition, flags, std::ptr::null_mut())
}

pub unsafe extern "win64" fn ReOpenFile(h: HANDLE, access: u32, _share: u32, flags: u32) -> HANDLE {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return INVALID_HANDLE_VALUE;
    };
    match create_file(&f.win_path, access, OPEN_EXISTING, flags) {
        Ok(h) => h,
        Err(e) => {
            set_err(e);
            INVALID_HANDLE_VALUE
        }
    }
}

// ---------------------------------------------------------------------------
// Read / write
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct OVERLAPPED {
    pub internal: usize,
    pub internal_high: usize,
    pub offset: u32,
    pub offset_high: u32,
    pub h_event: HANDLE,
}

pub(crate) const STATUS_SUCCESS: usize = 0;
/// Informational NTSTATUS (`ntstatus.h`) used by Winsock/AFD for successful OOB (`MSG_OOB`) receives.
pub(crate) const STATUS_RECEIVE_EXPEDITED: usize = 0x4000_0010;
pub(crate) const STATUS_PENDING: usize = 0x103;
pub(crate) const STATUS_END_OF_FILE: usize = 0xC000_0011;
pub(crate) const STATUS_CANCELLED: usize = 0xC000_0120;

#[inline]
pub(crate) fn is_io_success_status(s: usize) -> bool {
    s == STATUS_SUCCESS || s == STATUS_RECEIVE_EXPEDITED
}

/// Convert a Win32 / Winsock error code into an NTSTATUS suitable for `OVERLAPPED.Internal`.
pub(crate) fn win32_to_ntstatus(win32_err: u32) -> usize {
    match win32_err {
        0 => STATUS_SUCCESS,
        995 => STATUS_CANCELLED, // ERROR_OPERATION_ABORTED
        997 => STATUS_PENDING,   // ERROR_IO_PENDING
        38 => STATUS_END_OF_FILE,
        109 => 0xC000_014B,      // ERROR_BROKEN_PIPE -> STATUS_PIPE_BROKEN
        2 => 0xC000_0034,        // ERROR_FILE_NOT_FOUND -> STATUS_OBJECT_NAME_NOT_FOUND
        5 => 0xC000_0022,        // ERROR_ACCESS_DENIED -> STATUS_ACCESS_DENIED
        6 => 0xC000_0008,        // ERROR_INVALID_HANDLE -> STATUS_INVALID_HANDLE
        87 => 0xC000_000D,       // ERROR_INVALID_PARAMETER -> STATUS_INVALID_PARAMETER
        10054 => 0xC000_020D,    // WSAECONNRESET -> STATUS_CONNECTION_RESET
        10053 => 0xC000_0241,    // WSAECONNABORTED -> STATUS_CONNECTION_ABORTED
        10061 => 0xC000_0236,    // WSAECONNREFUSED -> STATUS_CONNECTION_REFUSED
        10060 => 0xC000_00B5,    // WSAETIMEDOUT -> STATUS_IO_TIMEOUT
        10057 => 0xC000_0128,    // WSAENOTCONN -> STATUS_FILE_CLOSED / not connected
        10058 => 0xC000_013C,    // WSAESHUTDOWN -> STATUS_REMOTE_DISCONNECT
        10035 => 0xC000_00B5,    // WSAEWOULDBLOCK
        10040 => 0x8000_0005,    // WSAEMSGSIZE -> STATUS_BUFFER_OVERFLOW
        _ => 0xC000_0001,        // STATUS_UNSUCCESSFUL
    }
}

#[inline]
pub(crate) unsafe fn store_ov_internal(ov: *mut OVERLAPPED, val: usize) {
    let p = std::ptr::addr_of_mut!((*ov).internal);
    if p.is_aligned() {
        std::sync::atomic::AtomicUsize::from_ptr(p).store(val, Ordering::Release);
    } else {
        std::sync::atomic::fence(Ordering::Release);
        std::ptr::write_unaligned(p, val);
    }
}

#[inline]
pub(crate) unsafe fn load_ov_internal(ov: *mut OVERLAPPED) -> usize {
    let p = std::ptr::addr_of_mut!((*ov).internal);
    if p.is_aligned() {
        std::sync::atomic::AtomicUsize::from_ptr(p).load(Ordering::Acquire)
    } else {
        let v = std::ptr::read_unaligned(p);
        std::sync::atomic::fence(Ordering::Acquire);
        v
    }
}

#[inline]
pub(crate) unsafe fn store_ov_internal_high(ov: *mut OVERLAPPED, val: usize) {
    let p = std::ptr::addr_of_mut!((*ov).internal_high);
    if p.is_aligned() {
        std::ptr::write_volatile(p, val);
    } else {
        std::ptr::write_unaligned(p, val);
    }
}

#[inline]
pub(crate) unsafe fn load_ov_internal_high(ov: *mut OVERLAPPED) -> usize {
    let p = std::ptr::addr_of!((*ov).internal_high);
    if p.is_aligned() {
        std::ptr::read_volatile(p)
    } else {
        std::ptr::read_unaligned(p)
    }
}

#[inline]
unsafe fn read_ov_offset(ov: *const OVERLAPPED) -> u64 {
    let lo = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).offset)) as u64;
    let hi = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).offset_high)) as u64;
    lo | (hi << 32)
}

/// Mark `ov` pending before starting async I/O and reset `ov.h_event` if present.
pub(crate) unsafe fn prepare_overlapped_pending(ov: *mut OVERLAPPED) {
    if ov.is_null() {
        return;
    }
    let ev = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).h_event));
    store_ov_internal_high(ov, 0);
    store_ov_internal(ov, STATUS_PENDING);
    if !ev.is_null() {
        let ev_h = (ev as usize & !1) as HANDLE;
        if let Some(e) = handles::get_as::<Event>(ev_h) {
            e.reset();
        }
    }
}

unsafe fn complete_overlapped(ov: *mut OVERLAPPED, status: usize, n: usize) {
    // Snapshot guest fields and resolve the explicit event handle BEFORE publishing
    // terminal `ov.internal` so a polling caller can immediately close the event handle
    // or free/reuse `ov` without racing with any lookup or guest-memory dereference.
    let raw_ev = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).h_event)) as usize;
    let ev_h = (raw_ev & !1) as HANDLE;
    let explicit_event = if !ev_h.is_null() {
        handles::get_as::<Event>(ev_h)
    } else {
        None
    };
    store_ov_internal_high(ov, n);
    store_ov_internal(ov, status);
    if let Some(e) = explicit_event {
        e.set();
    }
}

/// Drain up to `dst.len()` bytes from `f.peek_buf`.
fn drain_peek_buf(f: &FileObj, dst: &mut [u8]) -> usize {
    let mut pb = f.peek_buf.lock().unwrap();
    let k = pb.len().min(dst.len());
    for slot in dst.iter_mut().take(k) {
        *slot = pb.pop_front().unwrap();
    }
    k
}

/// Try a genuinely nonblocking read on a pipe/socket fd.
/// - For `AF_UNIX` `SOCK_STREAM` pipes (`CreatePipe`, `CreateNamedPipeW`/`A`), uses per-call
///   `recv(..., MSG_DONTWAIT)` without ever modifying open-file-description status flags
///   (`O_NONBLOCK`), so `DuplicateHandle` and inherited/redirected child stdio stay blocking.
/// - For external POSIX `S_IFIFO` descriptors (`ENOTSOCK`), ensures `O_NONBLOCK` is enabled
///   (without toggling it back off) so `read()` can never block the caller or shared pipe reactor.
/// Returns `Ok(Some(n))` (`n == 0` means EOF/broken pipe), `Ok(None)` if would block,
/// or `Err(errno)`.
unsafe fn try_nonblocking_pipe_read(fd: i32, buf: *mut u8, n: usize) -> Result<Option<usize>, i32> {
    if n == 0 {
        return Ok(Some(0));
    }
    // Try MSG_DONTWAIT first for socketpair-backed anonymous and named pipes.
    loop {
        let r = libc::recv(fd, buf as *mut c_void, n, libc::MSG_DONTWAIT);
        if r >= 0 {
            return Ok(Some(r as usize));
        }
        let e = last_errno();
        if e == libc::EINTR {
            continue;
        }
        if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
            return Ok(None);
        }
        if e == libc::ENOTSOCK {
            break;
        }
        return Err(e);
    }
    // External POSIX FIFO fallback: ensure O_NONBLOCK is set once (never toggled off) so
    // competing readers cannot cause a blocking read() between readiness check and read.
    let fl = libc::fcntl(fd, libc::F_GETFL);
    if fl < 0 {
        return Err(last_errno());
    }
    if (fl & libc::O_NONBLOCK) == 0 && libc::fcntl(fd, libc::F_SETFL, fl | libc::O_NONBLOCK) != 0 {
        return Err(last_errno());
    }
    loop {
        let r = libc::read(fd, buf as *mut c_void, n);
        if r >= 0 {
            return Ok(Some(r as usize));
        }
        let e = last_errno();
        if e == libc::EINTR {
            continue;
        }
        if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
            return Ok(None);
        }
        return Err(e);
    }
}

/// Try a genuinely nonblocking write on a pipe/socket fd.
/// On Darwin XNU, `AF_UNIX` `SOCK_STREAM` ignores `MSG_DONTWAIT` on `send()` when `O_NONBLOCK`
/// is clear and blocks in `sosend`; enabling `O_NONBLOCK` once (never toggling it back off) on
/// the write fd guarantees nonblocking `write()` for both socketpair-backed pipes and raw
/// `S_IFIFO` descriptors, while synchronous `WriteFile`/`ReadFile` handle `EAGAIN` via `poll`.
/// Returns `Ok(Some(n))`, `Ok(None)` if would block, or `Err(errno)`.
unsafe fn try_nonblocking_pipe_write(fd: i32, buf: *const u8, n: usize) -> Result<Option<usize>, i32> {
    if n == 0 {
        return Ok(Some(0));
    }
    let fl = libc::fcntl(fd, libc::F_GETFL);
    if fl < 0 {
        return Err(last_errno());
    }
    if (fl & libc::O_NONBLOCK) == 0 && libc::fcntl(fd, libc::F_SETFL, fl | libc::O_NONBLOCK) != 0 {
        return Err(last_errno());
    }
    loop {
        let r = libc::write(fd, buf as *const c_void, n);
        if r >= 0 {
            return Ok(Some(r as usize));
        }
        let e = last_errno();
        if e == libc::EINTR {
            continue;
        }
        if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
            return Ok(None);
        }
        return Err(e);
    }
}

pub unsafe extern "win64" fn ReadFile(h: HANDLE, buf: *mut c_void, n: u32, read: *mut u32, ov: *mut OVERLAPPED) -> BOOL {
    if let Some(r) = crate::misc::hid::read_file(h, buf, n, read, ov) {
        return r;
    }
    if !read.is_null() {
        std::ptr::write_unaligned(read, 0);
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if n > 0 && buf.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if f.kind == FileKind::Pipe {
        let target = match f.io_pipe_target() {
            Ok(t) => t,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
        let fd = target.fd();
        let slice = if n == 0 {
            &mut [][..]
        } else {
            std::slice::from_raw_parts_mut(buf as *mut u8, n as usize)
        };
        let from_peek = drain_peek_buf(&f, slice);
        if from_peek > 0 || n == 0 {
            if !read.is_null() {
                std::ptr::write_unaligned(read, from_peek as u32);
            }
            if !ov.is_null() {
                complete_overlapped_binding(Some(&f.completion), ov, STATUS_SUCCESS, from_peek, true);
            }
            return TRUE;
        }
        if !ov.is_null() {
            match try_nonblocking_pipe_read(fd, buf as *mut u8, n as usize) {
                Ok(Some(0)) => {
                    set_err(err::ERROR_BROKEN_PIPE);
                    return FALSE;
                }
                Ok(Some(got)) => {
                    if !read.is_null() {
                        std::ptr::write_unaligned(read, got as u32);
                    }
                    complete_overlapped_binding(Some(&f.completion), ov, STATUS_SUCCESS, got, true);
                    return TRUE;
                }
                Ok(None) => {
                    f.completion.io_event.reset();
                    prepare_overlapped_pending(ov);
                    pipe_reactor().submit(PendingPipeOp::new_read(
                        h,
                        Arc::clone(&f),
                        target.conn(),
                        fd,
                        buf as usize,
                        n as usize,
                        PipeDelivery::Overlapped { ov: ov as usize, tid: teb::thread_id() },
                    ));
                    set_err(err::ERROR_IO_PENDING);
                    return FALSE;
                }
                Err(e) => {
                    set_err(if e == libc::EPIPE || e == libc::ECONNRESET { err::ERROR_BROKEN_PIPE } else { errno_to_win32(e) });
                    return FALSE;
                }
            }
        }
        // Synchronous blocking pipe read (also waits via poll if fd has O_NONBLOCK).
        let r = loop {
            let k = libc::read(fd, buf, n as usize);
            if k < 0 {
                let e = last_errno();
                if e == libc::EINTR {
                    continue;
                }
                if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
                    let mut pfd = libc::pollfd { fd, events: libc::POLLIN, revents: 0 };
                    let pr = libc::poll(&mut pfd, 1, -1);
                    if pr < 0 && last_errno() != libc::EINTR {
                        break -1;
                    }
                    continue;
                }
            }
            break k;
        };
        if r < 0 {
            let e = last_errno();
            set_err(if e == libc::EPIPE || e == libc::ECONNRESET { err::ERROR_BROKEN_PIPE } else { errno_to_win32(e) });
            return FALSE;
        }
        if r == 0 {
            set_err(err::ERROR_BROKEN_PIPE);
            return FALSE;
        }
        if !read.is_null() {
            std::ptr::write_unaligned(read, r as u32);
        }
        return TRUE;
    }

    let slice = if n == 0 {
        &mut [][..]
    } else {
        std::slice::from_raw_parts_mut(buf as *mut u8, n as usize)
    };
    let io_scope = (f.kind == FileKind::Disk).then(|| crate::hitch::Scope::enter(crate::hitch::ActKind::FileIo, n as u64));
    let res = if !ov.is_null() && f.kind == FileKind::Disk {
        let off = read_ov_offset(ov);
        read_full_at(f.file(), slice, off)
    } else {
        use std::io::Read;
        let mut file = f.file();
        loop {
            match file.read(slice) {
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                r => break r,
            }
        }
    };
    if let Some(sc) = io_scope {
        let elapsed_ns = sc.elapsed_ns();
        drop(sc);
        let got_bytes = res.as_ref().copied().unwrap_or(0) as u64;
        crate::hitch::record_file_io("ReadFile", &f.win_path, got_bytes, elapsed_ns);
    }
    match res {
        Ok(got) => {
            if !read.is_null() {
                std::ptr::write_unaligned(read, got as u32);
            }
            if !ov.is_null() {
                if got == 0 && n > 0 && f.kind == FileKind::Disk {
                    complete_overlapped(ov, STATUS_END_OF_FILE, 0);
                    set_err(err::ERROR_HANDLE_EOF);
                    return FALSE;
                }
                complete_overlapped_binding(Some(&f.completion), ov, STATUS_SUCCESS, got, true);
            }
            TRUE
        }
        Err(e) => {
            log::warn!("ReadFile({:?}, n={n}) failed: {e}", f.win_path);
            set_err(io_err(&e));
            FALSE
        }
    }
}

fn read_full_at(f: &File, buf: &mut [u8], mut off: u64) -> std::io::Result<usize> {
    let mut done = 0;
    while done < buf.len() {
        match f.read_at(&mut buf[done..], off) {
            Ok(0) => break,
            Ok(k) => {
                done += k;
                off += k as u64;
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(done)
}

pub unsafe extern "win64" fn WriteFile(h: HANDLE, buf: *const c_void, n: u32, written: *mut u32, ov: *mut OVERLAPPED) -> BOOL {
    if let Some(r) = crate::misc::hid::write_file(h, buf, n, written, ov) {
        return r;
    }
    if !written.is_null() {
        std::ptr::write_unaligned(written, 0);
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if f.is_protected_native() {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    if n > 0 && buf.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if f.kind == FileKind::Pipe {
        let target = match f.io_pipe_target() {
            Ok(t) => t,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
        let fd = target.fd();
        if n == 0 {
            if !ov.is_null() {
                complete_overlapped_binding(Some(&f.completion), ov, STATUS_SUCCESS, 0, true);
            }
            return TRUE;
        }
        if !ov.is_null() {
            match pipe_reactor().admit_async_write(
                h,
                &f,
                target.conn(),
                fd,
                buf as *const u8,
                n as usize,
                ov,
                PipeDelivery::Overlapped { ov: ov as usize, tid: teb::thread_id() },
            ) {
                AsyncPipeWriteOutcome::Completed(done) => {
                    if !written.is_null() {
                        std::ptr::write_unaligned(written, done as u32);
                    }
                    complete_overlapped_binding(Some(&f.completion), ov, STATUS_SUCCESS, done, true);
                    return TRUE;
                }
                AsyncPipeWriteOutcome::Pending => {
                    set_err(err::ERROR_IO_PENDING);
                    return FALSE;
                }
                AsyncPipeWriteOutcome::Error(e) => {
                    set_err(e);
                    return FALSE;
                }
            }
        }
        // Synchronous blocking pipe write: wait for any earlier pending writes on the same
        // pipe/connection without holding reactor locks across blocking I/O, then write the
        // full buffer (waiting via poll if fd has O_NONBLOCK).
        let conn = target.conn();
        let _sync_guard = match pipe_reactor().acquire_sync_write_turn(h, &f, conn.clone(), fd) {
            Ok(g) => g,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
        let mut done = 0usize;
        while done < n as usize {
            if conn.as_ref().is_some_and(|c| c.is_disconnected()) {
                set_err(err::ERROR_BROKEN_PIPE);
                return FALSE;
            }
            let r = libc::write(fd, (buf as *const u8).add(done) as *const c_void, n as usize - done);
            if r < 0 {
                let e = last_errno();
                if e == libc::EINTR {
                    continue;
                }
                if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
                    let mut pfd = libc::pollfd { fd, events: libc::POLLOUT, revents: 0 };
                    let pr = libc::poll(&mut pfd, 1, -1);
                    if pr < 0 && last_errno() != libc::EINTR {
                        let pe = last_errno();
                        set_err(if pe == libc::EPIPE || pe == libc::ECONNRESET { err::ERROR_BROKEN_PIPE } else { errno_to_win32(pe) });
                        return FALSE;
                    }
                    continue;
                }
                set_err(if e == libc::EPIPE || e == libc::ECONNRESET { err::ERROR_BROKEN_PIPE } else { errno_to_win32(e) });
                return FALSE;
            }
            if r == 0 {
                set_err(err::ERROR_BROKEN_PIPE);
                return FALSE;
            }
            done += r as usize;
        }
        if !written.is_null() {
            std::ptr::write_unaligned(written, done as u32);
        }
        return TRUE;
    }

    let slice = if n == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(buf as *const u8, n as usize)
    };
    let io_scope = (f.kind == FileKind::Disk).then(|| crate::hitch::Scope::enter(crate::hitch::ActKind::FileIo, n as u64));
    let res = if !ov.is_null() && f.kind == FileKind::Disk {
        let off = read_ov_offset(ov);
        if off == u64::MAX {
            // 0xFFFFFFFF/0xFFFFFFFF = append
            use std::io::{Seek, Write};
            let mut file = f.file();
            file.seek(std::io::SeekFrom::End(0)).and_then(|_| file.write_all(slice)).map(|_| slice.len())
        } else {
            f.file().write_all_at(slice, off).map(|_| slice.len())
        }
    } else {
        use std::io::{Seek, Write};
        let mut file = f.file();
        if f.append {
            let _ = file.seek(std::io::SeekFrom::End(0));
        }
        file.write_all(slice).map(|_| slice.len())
    };
    if let Some(sc) = io_scope {
        let elapsed_ns = sc.elapsed_ns();
        drop(sc);
        let put_bytes = res.as_ref().copied().unwrap_or(0) as u64;
        crate::hitch::record_file_io("WriteFile", &f.win_path, put_bytes, elapsed_ns);
    }
    match res {
        Ok(k) => {
            if !written.is_null() {
                std::ptr::write_unaligned(written, k as u32);
            }
            if !ov.is_null() {
                complete_overlapped_binding(Some(&f.completion), ov, STATUS_SUCCESS, k, true);
            }
            TRUE
        }
        Err(e) => {
            log::warn!("WriteFile({:?}, n={n}) failed: {e}", f.win_path);
            set_err(io_err(&e));
            FALSE
        }
    }
}

pub unsafe extern "win64" fn ReadFileEx(
    h: HANDLE,
    buf: *mut c_void,
    n: u32,
    ov: *mut OVERLAPPED,
    completion_routine: Option<crate::thread::IoCompletionRoutine>,
) -> BOOL {
    if ov.is_null() || (n > 0 && buf.is_null()) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let caller = crate::thread::ensure_current_thread_object();
    let caller_tid = teb::thread_id();

    if f.kind == FileKind::Pipe {
        let target = match f.io_pipe_target() {
            Ok(t) => t,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
        let fd = target.fd();
        let slice = if n == 0 {
            &mut [][..]
        } else {
            std::slice::from_raw_parts_mut(buf as *mut u8, n as usize)
        };
        let from_peek = drain_peek_buf(&f, slice);
        if from_peek > 0 || n == 0 {
            store_ov_internal_high(ov, from_peek);
            store_ov_internal(ov, STATUS_SUCCESS);
            if let Some(cb) = completion_routine {
                crate::thread::queue_io_completion_apc(&caller, cb, 0, from_peek as u32, ov as usize);
            }
            set_err(0);
            return TRUE;
        }
        match try_nonblocking_pipe_read(fd, buf as *mut u8, n as usize) {
            Ok(Some(0)) => {
                store_ov_internal_high(ov, 0);
                store_ov_internal(ov, STATUS_PIPE_BROKEN);
                set_err(err::ERROR_BROKEN_PIPE);
                return FALSE;
            }
            Ok(Some(got)) => {
                store_ov_internal_high(ov, got);
                store_ov_internal(ov, STATUS_SUCCESS);
                if let Some(cb) = completion_routine {
                    crate::thread::queue_io_completion_apc(&caller, cb, 0, got as u32, ov as usize);
                }
                set_err(0);
                return TRUE;
            }
            Ok(None) => {
                store_ov_internal_high(ov, 0);
                store_ov_internal(ov, STATUS_PENDING);
                pipe_reactor().submit(PendingPipeOp::new_read(
                    h,
                    Arc::clone(&f),
                    target.conn(),
                    fd,
                    buf as usize,
                    n as usize,
                    PipeDelivery::Apc { ov: ov as usize, cb: completion_routine, thread: caller, tid: caller_tid },
                ));
                set_err(0);
                return TRUE;
            }
            Err(e) => {
                set_err(if e == libc::EPIPE || e == libc::ECONNRESET { err::ERROR_BROKEN_PIPE } else { errno_to_win32(e) });
                return FALSE;
            }
        }
    }

    let slice = if n == 0 {
        &mut [][..]
    } else {
        std::slice::from_raw_parts_mut(buf as *mut u8, n as usize)
    };
    let res = if f.kind == FileKind::Disk {
        let off = read_ov_offset(ov);
        read_full_at(f.file(), slice, off)
    } else {
        use std::io::Read;
        let mut file = f.file();
        loop {
            match file.read(slice) {
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                r => break r,
            }
        }
    };
    match res {
        Ok(got) => {
            if got == 0 && n > 0 && f.kind == FileKind::Disk {
                store_ov_internal_high(ov, 0);
                store_ov_internal(ov, STATUS_END_OF_FILE);
                set_err(err::ERROR_HANDLE_EOF);
                return FALSE;
            }
            store_ov_internal_high(ov, got);
            store_ov_internal(ov, STATUS_SUCCESS);
            if let Some(cb) = completion_routine {
                crate::thread::queue_io_completion_apc(&caller, cb, 0, got as u32, ov as usize);
            }
            set_err(0);
            TRUE
        }
        Err(e) => {
            set_err(io_err(&e));
            FALSE
        }
    }
}

pub unsafe extern "win64" fn WriteFileEx(
    h: HANDLE,
    buf: *const c_void,
    n: u32,
    ov: *mut OVERLAPPED,
    completion_routine: Option<crate::thread::IoCompletionRoutine>,
) -> BOOL {
    if ov.is_null() || (n > 0 && buf.is_null()) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if f.is_protected_native() {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    let caller = crate::thread::ensure_current_thread_object();
    let caller_tid = teb::thread_id();

    if f.kind == FileKind::Pipe {
        let target = match f.io_pipe_target() {
            Ok(t) => t,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
        let fd = target.fd();
        if n == 0 {
            store_ov_internal_high(ov, 0);
            store_ov_internal(ov, STATUS_SUCCESS);
            if let Some(cb) = completion_routine {
                crate::thread::queue_io_completion_apc(&caller, cb, 0, 0, ov as usize);
            }
            set_err(0);
            return TRUE;
        }
        let caller_for_done = Arc::clone(&caller);
        match pipe_reactor().admit_async_write(
            h,
            &f,
            target.conn(),
            fd,
            buf as *const u8,
            n as usize,
            ov,
            PipeDelivery::Apc { ov: ov as usize, cb: completion_routine, thread: caller, tid: caller_tid },
        ) {
            AsyncPipeWriteOutcome::Completed(done) => {
                store_ov_internal_high(ov, done);
                store_ov_internal(ov, STATUS_SUCCESS);
                if let Some(cb) = completion_routine {
                    crate::thread::queue_io_completion_apc(&caller_for_done, cb, 0, done as u32, ov as usize);
                }
                set_err(0);
                return TRUE;
            }
            AsyncPipeWriteOutcome::Pending => {
                set_err(0);
                return TRUE;
            }
            AsyncPipeWriteOutcome::Error(e) => {
                set_err(e);
                return FALSE;
            }
        }
    }

    let slice = if n == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(buf as *const u8, n as usize)
    };
    let res = if f.kind == FileKind::Disk {
        let off = read_ov_offset(ov);
        if off == u64::MAX {
            use std::io::{Seek, Write};
            let mut file = f.file();
            file.seek(std::io::SeekFrom::End(0)).and_then(|_| file.write_all(slice)).map(|_| slice.len())
        } else {
            f.file().write_all_at(slice, off).map(|_| slice.len())
        }
    } else {
        use std::io::{Seek, Write};
        let mut file = f.file();
        if f.append {
            let _ = file.seek(std::io::SeekFrom::End(0));
        }
        file.write_all(slice).map(|_| slice.len())
    };
    match res {
        Ok(k) => {
            store_ov_internal_high(ov, k);
            store_ov_internal(ov, STATUS_SUCCESS);
            if let Some(cb) = completion_routine {
                crate::thread::queue_io_completion_apc(&caller, cb, 0, k as u32, ov as usize);
            }
            set_err(0);
            TRUE
        }
        Err(e) => {
            set_err(io_err(&e));
            FALSE
        }
    }
}

pub unsafe extern "win64" fn GetOverlappedResult(h: HANDLE, ov: *mut OVERLAPPED, n: *mut u32, wait: BOOL) -> BOOL {
    GetOverlappedResultEx(h, ov, n, if wait != 0 { INFINITE } else { 0 }, FALSE)
}

/// Disk I/O completes synchronously; pipe/device/socket/directory-watch I/O may
/// be pending, completed from another thread (buffer, InternalHigh, then a
/// release store of Internal, then the event).
pub unsafe extern "win64" fn GetOverlappedResultEx(h: HANDLE, ov: *mut OVERLAPPED, n: *mut u32, ms: u32, alertable: BOOL) -> BOOL {
    if ov.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let deadline = (ms != INFINITE).then(|| Instant::now() + Duration::from_millis(ms as u64));
    while load_ov_internal(ov) == STATUS_PENDING {
        if ms == 0 {
            set_err(err::ERROR_IO_INCOMPLETE);
            return FALSE;
        }
        let remaining = match deadline {
            None => None,
            Some(dl) => {
                let now = Instant::now();
                if now >= dl {
                    set_err(WAIT_TIMEOUT);
                    return FALSE;
                }
                Some(dl - now)
            }
        };
        let ev = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).h_event));
        let obj: Option<Arc<dyn KObject>> = if !ev.is_null() {
            handles::get((ev as usize & !1) as HANDLE)
        } else if let Some(ext_ev) = external_io_event(h) {
            Some(ext_ev as Arc<dyn KObject>)
        } else {
            handles::get(h)
        };
        let Some(obj) = obj else {
            set_err(err::ERROR_INVALID_HANDLE);
            return FALSE;
        };
        match handles::wait_for_duration(std::slice::from_ref(&obj), false, remaining, alertable != 0) {
            handles::WaitResult::Timeout => {
                if load_ov_internal(ov) != STATUS_PENDING {
                    break;
                }
                set_err(WAIT_TIMEOUT);
                return FALSE;
            }
            handles::WaitResult::IoCompletion => {
                set_err(WAIT_IO_COMPLETION);
                return FALSE;
            }
            handles::WaitResult::Failed(e) => {
                set_err(e);
                return FALSE;
            }
            _ => {
                // If waiting on a shared handle event while another op completed,
                // yield briefly to avoid busy-spinning until our own `ov` completes.
                if load_ov_internal(ov) == STATUS_PENDING && ev.is_null() {
                    std::thread::yield_now();
                }
            }
        }
    }
    let status = load_ov_internal(ov);
    if !n.is_null() {
        std::ptr::write_unaligned(n, load_ov_internal_high(ov) as u32);
    }
    match status {
        STATUS_SUCCESS | STATUS_RECEIVE_EXPEDITED => TRUE,
        STATUS_END_OF_FILE => {
            set_err(err::ERROR_HANDLE_EOF);
            FALSE
        }
        STATUS_PENDING => {
            set_err(err::ERROR_IO_INCOMPLETE);
            FALSE
        }
        s => {
            set_err(ntstatus_to_win32_local(s));
            FALSE
        }
    }
}

pub unsafe extern "win64" fn CancelIo(h: HANDLE) -> BOOL {
    if let Some(r) = crate::misc::hid::cancel_io(h, std::ptr::null_mut(), false) {
        return r;
    }
    if let Some(f) = get_file(h) {
        let tid = teb::thread_id();
        let _ = f.cancel_pending_io(h, None, Some(tid));
        return TRUE;
    }
    let hooks = external_cancel_hooks().lock().unwrap().clone();
    for hook in hooks {
        if let Some(_cancelled) = hook(h, std::ptr::null_mut(), false) {
            return TRUE;
        }
    }
    set_err(err::ERROR_INVALID_HANDLE);
    FALSE
}
pub unsafe extern "win64" fn CancelIoEx(h: HANDLE, ov: *mut OVERLAPPED) -> BOOL {
    if let Some(r) = crate::misc::hid::cancel_io(h, ov, true) {
        return r;
    }
    if let Some(f) = get_file(h) {
        let ov_filter = (!ov.is_null()).then_some(ov as usize);
        let n = f.cancel_pending_io(h, ov_filter, None);
        if n > 0 {
            return TRUE;
        }
        set_err(err::ERROR_NOT_FOUND);
        return FALSE;
    }
    let hooks = external_cancel_hooks().lock().unwrap().clone();
    for hook in hooks {
        if let Some(cancelled) = hook(h, ov, true) {
            if cancelled {
                return TRUE;
            } else {
                set_err(err::ERROR_NOT_FOUND);
                return FALSE;
            }
        }
    }
    set_err(err::ERROR_NOT_FOUND);
    FALSE
}
pub unsafe extern "win64" fn CancelSynchronousIo(_h: HANDLE) -> BOOL {
    set_err(err::ERROR_NOT_FOUND);
    FALSE
}

pub unsafe extern "win64" fn FlushFileBuffers(h: HANDLE) -> BOOL {
    match get_file(h) {
        Some(f) => {
            if f.kind == FileKind::Disk {
                let _ = f.file().sync_data();
            }
            TRUE
        }
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn LockFile(h: HANDLE, _off_lo: u32, _off_hi: u32, _len_lo: u32, _len_hi: u32) -> BOOL {
    if get_file(h).is_none() {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn LockFileEx(h: HANDLE, _flags: u32, _res: u32, _len_lo: u32, _len_hi: u32, ov: *mut OVERLAPPED) -> BOOL {
    if get_file(h).is_none() {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    if !ov.is_null() {
        complete_overlapped(ov, STATUS_SUCCESS, 0);
    }
    TRUE
}

pub unsafe extern "win64" fn UnlockFile(h: HANDLE, _off_lo: u32, _off_hi: u32, _len_lo: u32, _len_hi: u32) -> BOOL {
    if get_file(h).is_none() {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn UnlockFileEx(h: HANDLE, _res: u32, _len_lo: u32, _len_hi: u32, _ov: *mut OVERLAPPED) -> BOOL {
    if get_file(h).is_none() {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    TRUE
}

// ---------------------------------------------------------------------------
// File pointer / size / info
// ---------------------------------------------------------------------------

fn seek(f: &FileObj, dist: i64, method: u32) -> Result<u64, u32> {
    let whence = match method {
        0 => libc::SEEK_SET,
        1 => libc::SEEK_CUR,
        2 => libc::SEEK_END,
        _ => return Err(err::ERROR_INVALID_PARAMETER),
    };
    let r = unsafe { libc::lseek(f.fd(), dist, whence) };
    if r < 0 {
        let e = last_errno();
        Err(if e == libc::EINVAL { err::ERROR_NEGATIVE_SEEK } else { errno_to_win32(e) })
    } else {
        Ok(r as u64)
    }
}

pub unsafe extern "win64" fn SetFilePointerEx(h: HANDLE, dist: i64, newpos: *mut i64, method: u32) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    match seek(&f, dist, method) {
        Ok(p) => {
            if !newpos.is_null() {
                *newpos = p as i64;
            }
            TRUE
        }
        Err(e) => {
            set_err(e);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn SetFilePointer(h: HANDLE, lo: i32, hi: *mut i32, method: u32) -> u32 {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return u32::MAX;
    };
    let dist = if hi.is_null() { lo as i64 } else { ((*hi as i64) << 32) | (lo as u32 as i64) };
    match seek(&f, dist, method) {
        Ok(p) => {
            if !hi.is_null() {
                *hi = (p >> 32) as i32;
            }
            set_err(0);
            p as u32
        }
        Err(e) => {
            set_err(e);
            u32::MAX
        }
    }
}

pub unsafe extern "win64" fn SetEndOfFile(h: HANDLE) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if f.is_protected_native() {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    let pos = libc::lseek(f.fd(), 0, libc::SEEK_CUR);
    if pos < 0 || libc::ftruncate(f.fd(), pos) != 0 {
        set_err(errno_to_win32(last_errno()));
        return FALSE;
    }
    TRUE
}

fn file_size(f: &FileObj) -> Result<u64, u32> {
    f.file().metadata().map(|m| m.len()).map_err(|e| io_err(&e))
}

pub unsafe extern "win64" fn GetFileSizeEx(h: HANDLE, out: *mut i64) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    match file_size(&f) {
        Ok(s) => {
            *out = s as i64;
            TRUE
        }
        Err(e) => {
            set_err(e);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn GetFileSize(h: HANDLE, hi: *mut u32) -> u32 {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return u32::MAX;
    };
    match file_size(&f) {
        Ok(s) => {
            if !hi.is_null() {
                *hi = (s >> 32) as u32;
            }
            set_err(0);
            s as u32
        }
        Err(e) => {
            set_err(e);
            u32::MAX
        }
    }
}

pub unsafe extern "win64" fn GetFileType(h: HANDLE) -> u32 {
    match get_file(h) {
        Some(f) => {
            set_err(0);
            match f.kind {
                FileKind::Disk | FileKind::Dir | FileKind::Device => FILE_TYPE_DISK,
                FileKind::Console | FileKind::Null => FILE_TYPE_CHAR,
                FileKind::Pipe => FILE_TYPE_PIPE,
            }
        }
        None => {
            set_err(err::ERROR_INVALID_HANDLE);
            FILE_TYPE_UNKNOWN
        }
    }
}

pub struct Times {
    pub create: u64,
    pub access: u64,
    pub write: u64,
    pub change: u64,
}

pub fn meta_times(m: &std::fs::Metadata) -> Times {
    let birth = unix_to_filetime(m.st_birthtime(), m.st_birthtime_nsec());
    Times {
        create: if m.st_birthtime() != 0 { birth } else { unix_to_filetime(m.ctime(), m.ctime_nsec()) },
        access: unix_to_filetime(m.atime(), m.atime_nsec()),
        write: unix_to_filetime(m.mtime(), m.mtime_nsec()),
        change: unix_to_filetime(m.ctime(), m.ctime_nsec()),
    }
}

trait BirthTime {
    fn st_birthtime(&self) -> i64;
    fn st_birthtime_nsec(&self) -> i64;
}
impl BirthTime for std::fs::Metadata {
    fn st_birthtime(&self) -> i64 {
        self.created()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map_or(0, |d| d.as_secs() as i64)
    }
    fn st_birthtime_nsec(&self) -> i64 {
        self.created()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map_or(0, |d| d.subsec_nanos() as i64)
    }
}

pub fn meta_attrs(m: &std::fs::Metadata, name: &str) -> u32 {
    let mut a = 0;
    if m.is_dir() {
        a |= FILE_ATTRIBUTE_DIRECTORY;
    } else {
        a |= FILE_ATTRIBUTE_ARCHIVE;
    }
    if m.permissions().mode() & 0o200 == 0 {
        a |= FILE_ATTRIBUTE_READONLY;
    }
    if name.starts_with('.') && name != "." && name != ".." {
        a |= FILE_ATTRIBUTE_HIDDEN;
    }
    a
}

unsafe fn put_ft(p: *mut u8, v: u64) {
    (p as *mut u64).write_unaligned(v);
}

pub unsafe extern "win64" fn GetFileInformationByHandle(h: HANDLE, out: *mut u8) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let m = match f.file().metadata() {
        Ok(m) => m,
        Err(e) => {
            set_err(io_err(&e));
            return FALSE;
        }
    };
    let t = meta_times(&m);
    std::ptr::write_bytes(out, 0, 52);
    let name = f.host_path.as_ref().and_then(|p| p.file_name()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    (out as *mut u32).write_unaligned(meta_attrs(&m, &name));
    put_ft(out.add(4), t.create);
    put_ft(out.add(12), t.access);
    put_ft(out.add(20), t.write);
    (out.add(28) as *mut u32).write_unaligned(m.dev() as u32);
    (out.add(32) as *mut u32).write_unaligned((m.len() >> 32) as u32);
    (out.add(36) as *mut u32).write_unaligned(m.len() as u32);
    (out.add(40) as *mut u32).write_unaligned(m.nlink() as u32);
    (out.add(44) as *mut u32).write_unaligned((m.ino() >> 32) as u32);
    (out.add(48) as *mut u32).write_unaligned(m.ino() as u32);
    TRUE
}

const IO_REPARSE_TAG_SYMLINK: u32 = 0xA000_000C;

fn collect_dir_enum_entries(dir: &Path) -> std::io::Result<Vec<DirEnumEntry>> {
    let mut out = Vec::new();
    // Windows directory enumeration starts with "." and ".." for non-root directories.
    if let Ok(m) = std::fs::metadata(dir) {
        let t = meta_times(&m);
        out.push(DirEnumEntry {
            name: ".".into(),
            attrs: FILE_ATTRIBUTE_DIRECTORY,
            times: t,
            size: 0,
            alloc_size: 0,
            ino: m.ino(),
            reparse_tag: 0,
        });
        let parent_m = dir.parent().and_then(|p| std::fs::metadata(p).ok()).unwrap_or(m);
        let pt = meta_times(&parent_m);
        out.push(DirEnumEntry {
            name: "..".into(),
            attrs: FILE_ATTRIBUTE_DIRECTORY,
            times: pt,
            size: 0,
            alloc_size: 0,
            ino: parent_m.ino(),
            reparse_tag: 0,
        });
    }
    let mut children = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let Ok(entry) = entry else { continue };
        let name = entry.file_name().to_string_lossy().into_owned();
        let Ok(sm) = std::fs::symlink_metadata(entry.path()) else { continue };
        let is_symlink = sm.file_type().is_symlink();
        let m = if is_symlink { std::fs::metadata(entry.path()).unwrap_or(sm) } else { sm };
        let mut attrs = meta_attrs(&m, &name);
        let reparse_tag = if is_symlink {
            attrs |= FILE_ATTRIBUTE_REPARSE_POINT;
            IO_REPARSE_TAG_SYMLINK
        } else {
            0
        };
        let len = if m.is_dir() { 0 } else { m.len() };
        let alloc_size = if m.is_dir() { 0 } else { (m.blocks() * 512).max(len) };
        children.push(DirEnumEntry {
            name,
            attrs,
            times: meta_times(&m),
            size: len,
            alloc_size,
            ino: m.ino(),
            reparse_tag,
        });
    }
    children.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    out.extend(children);
    Ok(out)
}

#[derive(Clone, Copy)]
enum DirInfoLayout {
    IdBoth, // class 10/11: header 104 bytes
    Full,   // class 14/15: header 68 bytes
    IdExtd, // class 19/20: header 88 bytes
}

unsafe fn fill_dir_info_by_handle(f: &FileObj, layout: DirInfoLayout, restart: bool, out: *mut u8, size: u32) -> BOOL {
    if f.kind != FileKind::Dir {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(host_dir) = &f.host_path else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let mut guard = f.dir_enum.lock().unwrap();
    if restart || guard.is_none() {
        match collect_dir_enum_entries(host_dir) {
            Ok(entries) => *guard = Some(DirEnumState { entries, cursor: 0 }),
            Err(e) => {
                set_err(io_err(&e));
                return FALSE;
            }
        }
    }
    let st = guard.as_mut().unwrap();
    if st.cursor >= st.entries.len() {
        set_err(err::ERROR_NO_MORE_FILES);
        return FALSE;
    }
    let header_len: usize = match layout {
        DirInfoLayout::IdBoth => 104,
        DirInfoLayout::Full => 68,
        DirInfoLayout::IdExtd => 88,
    };
    let cap = size as usize;
    let mut off = 0usize;
    let mut last_off: Option<usize> = None;
    while st.cursor < st.entries.len() {
        let e = &st.entries[st.cursor];
        let w = to_wide(&e.name);
        let name_bytes = w.len() * 2;
        let rec_len = header_len + name_bytes;
        let aligned_len = (rec_len + 7) & !7;
        let needed = if off == 0 { rec_len } else { aligned_len };
        if off + rec_len > cap {
            if last_off.is_none() {
                set_err(err::ERROR_MORE_DATA);
                return FALSE;
            }
            break;
        }
        if let Some(prev) = last_off {
            (out.add(prev) as *mut u32).write_unaligned((off - prev) as u32);
        }
        let dst = out.add(off);
        std::ptr::write_bytes(dst, 0, header_len.min(cap - off));
        (dst.add(4) as *mut u32).write_unaligned(st.cursor as u32); // FileIndex
        put_ft(dst.add(8), e.times.create);
        put_ft(dst.add(16), e.times.access);
        put_ft(dst.add(24), e.times.write);
        put_ft(dst.add(32), e.times.change);
        put_ft(dst.add(40), e.size);
        put_ft(dst.add(48), e.alloc_size);
        (dst.add(56) as *mut u32).write_unaligned(e.attrs);
        (dst.add(60) as *mut u32).write_unaligned(name_bytes as u32);
        (dst.add(64) as *mut u32).write_unaligned(0); // EaSize
        match layout {
            DirInfoLayout::IdBoth => {
                // ShortNameLength @68 = 0, ShortName[12] @70..94 = 0, FileId @96
                (dst.add(96) as *mut u64).write_unaligned(e.ino);
                std::ptr::copy_nonoverlapping(w.as_ptr() as *const u8, dst.add(104), name_bytes);
            }
            DirInfoLayout::Full => {
                std::ptr::copy_nonoverlapping(w.as_ptr() as *const u8, dst.add(68), name_bytes);
            }
            DirInfoLayout::IdExtd => {
                (dst.add(68) as *mut u32).write_unaligned(e.reparse_tag);
                (dst.add(72) as *mut u64).write_unaligned(e.ino);
                std::ptr::copy_nonoverlapping(w.as_ptr() as *const u8, dst.add(88), name_bytes);
            }
        }
        last_off = Some(off);
        st.cursor += 1;
        let next = off + needed;
        if next >= cap {
            break;
        }
        off = (off + rec_len + 7) & !7;
    }
    TRUE
}

/// Populate Windows `FILE_REMOTE_PROTOCOL_INFO` (116 bytes) in exact Win32 ABI field order:
/// `@0 USHORT StructureVersion`, `@2 USHORT StructureSize`, `@4 ULONG Protocol`,
/// `@8 USHORT ProtocolMajorVersion`, `@10 USHORT ProtocolMinorVersion`.
unsafe fn write_remote_protocol_info(out: *mut u8, fs_type: &str) {
    std::ptr::write_bytes(out, 0, 116);
    (out as *mut u16).write_unaligned(2); // StructureVersion (@0)
    (out.add(2) as *mut u16).write_unaligned(116); // StructureSize (@2)
    // WNNC_NET_LANMAN (0x0002_0000) for SMB, WNNC_NET_SUN_PC_NFS (0x0007_0000) for NFS.
    let proto = if fs_type.eq_ignore_ascii_case("nfs") { 0x0007_0000u32 } else { 0x0002_0000u32 };
    (out.add(4) as *mut u32).write_unaligned(proto); // Protocol (@4)
    (out.add(8) as *mut u16).write_unaligned(3); // ProtocolMajorVersion (@8)
    (out.add(10) as *mut u16).write_unaligned(0); // ProtocolMinorVersion (@10)
}

pub unsafe extern "win64" fn GetFileInformationByHandleEx(h: HANDLE, class: u32, out: *mut u8, size: u32) -> BOOL {
    if out.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    // Directory enumeration classes manage their own cursor and metadata.
    match class {
        10 => return fill_dir_info_by_handle(&f, DirInfoLayout::IdBoth, false, out, size),
        11 => return fill_dir_info_by_handle(&f, DirInfoLayout::IdBoth, true, out, size),
        14 => return fill_dir_info_by_handle(&f, DirInfoLayout::Full, false, out, size),
        15 => return fill_dir_info_by_handle(&f, DirInfoLayout::Full, true, out, size),
        19 => return fill_dir_info_by_handle(&f, DirInfoLayout::IdExtd, false, out, size),
        20 => return fill_dir_info_by_handle(&f, DirInfoLayout::IdExtd, true, out, size),
        _ => {}
    }
    let m = match f.file().metadata() {
        Ok(m) => m,
        Err(e) => {
            set_err(io_err(&e));
            return FALSE;
        }
    };
    let name = f.host_path.as_ref().and_then(|p| p.file_name()).map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let need = |n: u32| -> bool {
        if size < n {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            false
        } else {
            true
        }
    };
    match class {
        0 => {
            // FileBasicInfo (40 bytes)
            if !need(40) {
                return FALSE;
            }
            let t = meta_times(&m);
            std::ptr::write_bytes(out, 0, 40);
            put_ft(out, t.create);
            put_ft(out.add(8), t.access);
            put_ft(out.add(16), t.write);
            put_ft(out.add(24), t.change);
            (out.add(32) as *mut u32).write_unaligned(meta_attrs(&m, &name));
        }
        1 => {
            // FileStandardInfo (24 bytes)
            if !need(24) {
                return FALSE;
            }
            std::ptr::write_bytes(out, 0, 24);
            put_ft(out, (m.blocks() * 512).max(m.len()));
            put_ft(out.add(8), m.len());
            (out.add(16) as *mut u32).write_unaligned(m.nlink() as u32);
            *out.add(20) = *f.delete_on_close.lock().unwrap() as u8;
            *out.add(21) = m.is_dir() as u8;
        }
        2 | 24 => {
            // FileNameInfo (2) / FileNormalizedNameInfo (24): path without the drive letter.
            let p = f.win_path.get(2..).unwrap_or("").to_string();
            let w = to_wide(&p);
            if !need(4) {
                return FALSE;
            }
            (out as *mut u32).write_unaligned((w.len() * 2) as u32);
            let room = (size as usize - 4) / 2;
            std::ptr::copy_nonoverlapping(w.as_ptr() as *const u8, out.add(4), w.len().min(room) * 2);
            if w.len() > room {
                set_err(err::ERROR_MORE_DATA);
                return FALSE;
            }
        }
        7 => {
            // FileStreamInfo: NTFS exposes default "::$DATA" stream for regular files;
            // directories have no unnamed data stream and return ERROR_HANDLE_EOF.
            if m.is_dir() {
                set_err(err::ERROR_HANDLE_EOF);
                return FALSE;
            }
            let stream_name = to_wide("::$DATA");
            let name_bytes = stream_name.len() * 2;
            let total = 24 + name_bytes;
            if (size as usize) < total {
                set_err(err::ERROR_MORE_DATA);
                return FALSE;
            }
            std::ptr::write_bytes(out, 0, 24);
            (out as *mut u32).write_unaligned(0); // NextEntryOffset
            (out.add(4) as *mut u32).write_unaligned(name_bytes as u32);
            put_ft(out.add(8), m.len());
            put_ft(out.add(16), (m.blocks() * 512).max(m.len()));
            std::ptr::copy_nonoverlapping(stream_name.as_ptr() as *const u8, out.add(24), name_bytes);
        }
        8 => {
            // FileCompressionInfo (16 bytes)
            if !need(16) {
                return FALSE;
            }
            std::ptr::write_bytes(out, 0, 16);
            put_ft(out, m.len());
            // CompressionFormat = COMPRESSION_FORMAT_NONE (0)
        }
        9 => {
            // FileAttributeTagInfo (8 bytes)
            if !need(8) {
                return FALSE;
            }
            let is_symlink = f
                .host_path
                .as_deref()
                .and_then(|p| std::fs::symlink_metadata(p).ok())
                .is_some_and(|sm| sm.file_type().is_symlink());
            let mut attrs = meta_attrs(&m, &name);
            let tag = if is_symlink {
                attrs |= FILE_ATTRIBUTE_REPARSE_POINT;
                IO_REPARSE_TAG_SYMLINK
            } else {
                0
            };
            (out as *mut u32).write_unaligned(attrs);
            (out.add(4) as *mut u32).write_unaligned(tag);
        }
        13 => {
            // FileRemoteProtocolInfo (116 bytes): Windows returns ERROR_INVALID_PARAMETER for
            // files on local volumes (only populated for SMB/NFS/WebDAV network shares).
            let mut sfs: libc::statfs = std::mem::zeroed();
            let is_remote = libc::fstatfs(f.fd(), &mut sfs) == 0 && (sfs.f_flags & libc::MNT_LOCAL as u32) == 0;
            if !is_remote {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            if !need(116) {
                return FALSE;
            }
            let fs_type = std::ffi::CStr::from_ptr(sfs.f_fstypename.as_ptr()).to_string_lossy();
            write_remote_protocol_info(out, &fs_type);
        }
        16 => {
            // FileStorageInfo (28 bytes): report real volume sector sizes via statfs/IOKit.
            if !need(28) {
                return FALSE;
            }
            let vol = query_volume_storage_info(&f);
            let flags = 0x1u32 | 0x2 | if !vol.incurs_seek_penalty { 0x8 } else { 0 };
            let vals: [u32; 7] = [
                vol.logical_sector_size,
                vol.physical_sector_size,
                vol.physical_sector_size,
                vol.physical_sector_size,
                flags,
                0,
                0,
            ];
            std::ptr::copy_nonoverlapping(vals.as_ptr() as *const u8, out, 28);
        }
        17 => {
            // FileAlignmentInfo (4 bytes): FILE_BYTE_ALIGNMENT (0)
            if !need(4) {
                return FALSE;
            }
            (out as *mut u32).write_unaligned(0);
        }
        18 => {
            // FileIdInfo (24 bytes)
            if !need(24) {
                return FALSE;
            }
            std::ptr::write_bytes(out, 0, 24);
            put_ft(out, m.dev() as u64);
            put_ft(out.add(8), m.ino());
        }
        23 => {
            // FileCaseSensitiveInfo (4 bytes): query real volume case sensitivity via fpathconf.
            if !need(4) {
                return FALSE;
            }
            let cs = libc::fpathconf(f.fd(), libc::_PC_CASE_SENSITIVE);
            let flags: u32 = if cs > 0 { 1 } else { 0 };
            (out as *mut u32).write_unaligned(flags);
        }
        // Write-only classes (3=Rename, 4=Disposition, 5=Allocation, 6=EndOfFile, 12=IoPriorityHint,
        // 21=DispositionEx, 22=RenameEx) and out-of-range values (>= MaximumFileInfoByHandleClass)
        // fail with ERROR_INVALID_PARAMETER on Windows.
        _ => {
            set_err(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn SetFileInformationByHandle(h: HANDLE, class: u32, info: *const u8, size: u32) -> BOOL {
    if info.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    match class {
        0 => {
            // FileBasicInfo (40 bytes): times (0 or -1 = don't change) and optional read-only attribute.
            if size < 40 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            if f.is_protected_native() {
                set_err(err::ERROR_ACCESS_DENIED);
                return FALSE;
            }
            let a = (info.add(8) as *const i64).read_unaligned();
            let w = (info.add(16) as *const i64).read_unaligned();
            let attrs = (info.add(32) as *const u32).read_unaligned();
            if attrs != 0 {
                if let Some(ref p) = f.host_path {
                    if let Ok(m) = std::fs::metadata(p) {
                        let mut perm = m.permissions();
                        let mode = perm.mode();
                        perm.set_mode(if attrs & FILE_ATTRIBUTE_READONLY != 0 { mode & !0o222 } else { mode | 0o200 });
                        let _ = std::fs::set_permissions(p, perm);
                    }
                }
            }
            set_times(&f, a, w)
        }
        3 | 22 => {
            // FileRenameInfo (3) / FileRenameInfoEx (22):
            // x64 layout: { DWORD Flags/ReplaceIfExists (@0); HANDLE RootDirectory (@8); DWORD FileNameLength (@16); WCHAR FileName[] (@20) }
            if size < 22 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let replace = if class == 3 {
                *info != 0
            } else {
                ((info as *const u32).read_unaligned() & 0x1) != 0
            };
            let root_dir = (info.add(8) as *const HANDLE).read_unaligned();
            let name_bytes = (info.add(16) as *const u32).read_unaligned() as usize;
            if name_bytes == 0 || name_bytes % 2 != 0 || 20 + name_bytes > size as usize {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let len = name_bytes / 2;
            let raw_units = crate::strings::read_wide_slice(info.add(20) as *const u16, len);
            let rel_name = String::from_utf16_lossy(&raw_units);
            let target_win = if !root_dir.is_null() {
                let Some(root_obj) = get_file(root_dir) else {
                    set_err(err::ERROR_INVALID_HANDLE);
                    return FALSE;
                };
                format!("{}\\{}", root_obj.win_path.trim_end_matches('\\'), rel_name.trim_start_matches('\\'))
            } else {
                rel_name
            };
            let Some(src) = &f.host_path else {
                set_err(err::ERROR_INVALID_HANDLE);
                return FALSE;
            };
            let dst = match host_path(&target_win) {
                Ok(p) => p,
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            };
            if f.is_protected_native()
                || path::is_protected_host_remove_or_rename(src)
                || path::is_protected_host_remove_or_rename(&dst)
            {
                set_err(err::ERROR_ACCESS_DENIED);
                return FALSE;
            }
            if !replace && dst.exists() {
                set_err(err::ERROR_ALREADY_EXISTS);
                return FALSE;
            }
            match std::fs::rename(src, &dst) {
                Ok(()) => TRUE,
                Err(e) => {
                    set_err(io_err(&e));
                    FALSE
                }
            }
        }
        4 | 21 => {
            // FileDispositionInfo (4, 1 byte) / FileDispositionInfoEx (21, 4 bytes)
            let min_sz = if class == 4 { 1 } else { 4 };
            if size < min_sz {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let del = if class == 4 { *info != 0 } else { (info as *const u32).read_unaligned() & 1 != 0 };
            if del && f.is_protected_native() {
                set_err(err::ERROR_ACCESS_DENIED);
                return FALSE;
            }
            *f.delete_on_close.lock().unwrap() = del;
            TRUE
        }
        5 | 6 => {
            // FileAllocationInfo (5) / FileEndOfFileInfo (6), both 8 bytes
            if size < 8 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let len = (info as *const i64).read_unaligned();
            if len < 0 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            if f.is_protected_native() {
                set_err(err::ERROR_ACCESS_DENIED);
                return FALSE;
            }
            if class == 5 {
                let mut st: libc::stat = std::mem::zeroed();
                let (cur_len, cur_alloc) = if libc::fstat(f.fd(), &mut st) == 0 {
                    (st.st_size, (st.st_blocks as i64).saturating_mul(512).max(st.st_size))
                } else {
                    let l = f.file().metadata().map(|m| m.len() as i64).unwrap_or(0);
                    (l, l)
                };
                if len < cur_len {
                    // Windows shrinks EndOfFile when AllocationSize is set smaller than EOF.
                    if libc::ftruncate(f.fd(), len) != 0 {
                        set_err(errno_to_win32(last_errno()));
                        return FALSE;
                    }
                } else if len > cur_alloc {
                    #[cfg(target_vendor = "apple")]
                    {
                        let mut store = libc::fstore_t {
                            fst_flags: libc::F_ALLOCATECONTIG,
                            fst_posmode: libc::F_PEOFPOSMODE,
                            fst_offset: 0,
                            fst_length: len - cur_alloc,
                            fst_bytesalloc: 0,
                        };
                        if libc::fcntl(f.fd(), libc::F_PREALLOCATE, &mut store) == -1 {
                            store.fst_flags = libc::F_ALLOCATEALL;
                            if libc::fcntl(f.fd(), libc::F_PREALLOCATE, &mut store) == -1 {
                                set_err(errno_to_win32(last_errno()));
                                return FALSE;
                            }
                        }
                    }
                }
                return TRUE;
            }
            if libc::ftruncate(f.fd(), len) != 0 {
                set_err(errno_to_win32(last_errno()));
                return FALSE;
            }
            TRUE
        }
        12 => {
            // FileIoPriorityHintInfo (4 bytes): 0=VeryLow, 1=Low, 2=Normal
            if size < 4 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let prio = (info as *const u32).read_unaligned();
            if prio >= 3 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            *f.io_priority.lock().unwrap() = prio;
            TRUE
        }
        23 => {
            // FileCaseSensitiveInfo (4 bytes): APFS volume case-sensitivity is fixed at format time;
            // succeed if matching the volume's actual mode, else fail with ERROR_NOT_SUPPORTED.
            if size < 4 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let req = (info as *const u32).read_unaligned();
            if (req & !1) != 0 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let actual = if libc::fpathconf(f.fd(), libc::_PC_CASE_SENSITIVE) > 0 { 1 } else { 0 };
            if req == actual {
                TRUE
            } else {
                set_err(err::ERROR_NOT_SUPPORTED);
                FALSE
            }
        }
        // Read-only classes and out-of-range values fail with ERROR_INVALID_PARAMETER on Windows.
        _ => {
            set_err(err::ERROR_INVALID_PARAMETER);
            FALSE
        }
    }
}

unsafe fn set_times(f: &FileObj, access: i64, write: i64) -> BOOL {
    let conv = |ft: i64| -> libc::timespec {
        if ft <= 0 {
            libc::timespec { tv_sec: 0, tv_nsec: libc::UTIME_OMIT }
        } else {
            let t = (ft - EPOCH_DIFF_100NS as i64).max(0);
            libc::timespec { tv_sec: t.div_euclid(10_000_000), tv_nsec: t.rem_euclid(10_000_000) * 100 }
        }
    };
    let ts = [conv(access), conv(write)];
    if libc::futimens(f.fd(), ts.as_ptr()) != 0 {
        let e = last_errno();
        if e == libc::EACCES || e == libc::EPERM || e == libc::EROFS || e == libc::EINVAL {
            log::debug!("futimens({:?}) ignored errno={e}", f.win_path);
            return TRUE;
        }
        log::warn!("futimens({:?}) failed: errno={e}", f.win_path);
        set_err(errno_to_win32(e));
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn SetFileTime(h: HANDLE, _c: *const FILETIME, a: *const FILETIME, w: *const FILETIME) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if f.is_protected_native() {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    let g = |p: *const FILETIME| if p.is_null() { 0 } else { (*p).to_u64() as i64 };
    set_times(&f, g(a), g(w))
}

pub unsafe extern "win64" fn GetFileTime(h: HANDLE, c: *mut FILETIME, a: *mut FILETIME, w: *mut FILETIME) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let Ok(m) = f.file().metadata() else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let t = meta_times(&m);
    for (p, v) in [(c, t.create), (a, t.access), (w, t.write)] {
        if !p.is_null() {
            *p = FILETIME::from_u64(v);
        }
    }
    TRUE
}

pub unsafe extern "win64" fn GetFinalPathNameByHandleW(h: HANDLE, buf: *mut u16, cap: u32, flags: u32) -> u32 {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return 0;
    };
    // VOLUME_NAME_DOS (0) gets the \\?\ prefix; VOLUME_NAME_NONE (4) drops the drive.
    let s = if flags & 4 != 0 { f.win_path.get(2..).unwrap_or("").to_string() } else { format!("\\\\?\\{}", f.win_path) };
    let need = s.encode_utf16().count() as u32;
    if need + 1 > cap {
        return need + 1;
    }
    write_wide_buf(&s, buf, cap as usize);
    need
}

pub unsafe extern "win64" fn GetFinalPathNameByHandleA(h: HANDLE, buf: *mut u8, cap: u32, flags: u32) -> u32 {
    let mut w = vec![0u16; 1024];
    let n = GetFinalPathNameByHandleW(h, w.as_mut_ptr(), 1024, flags);
    if n == 0 || n > 1024 {
        return n;
    }
    let s = String::from_utf16_lossy(&w[..n as usize]);
    if s.len() as u32 + 1 > cap {
        return s.len() as u32 + 1;
    }
    write_ansi_buf(&s, buf, cap as usize);
    s.len() as u32
}

// ---------------------------------------------------------------------------
// Path-based operations
// ---------------------------------------------------------------------------

fn win_attrs(win: &str) -> Result<(u32, std::fs::Metadata), u32> {
    let host = host_path(win)?;
    let m = std::fs::metadata(&host).map_err(|e| match e.raw_os_error() {
        Some(libc::ENOENT) | Some(libc::ENOTDIR) => not_found_err(&host),
        _ => io_err(&e),
    })?;
    let name = host.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    Ok((meta_attrs(&m, &name), m))
}

pub unsafe extern "win64" fn GetFileAttributesW(name: LPCWSTR) -> u32 {
    let n = from_wide(name);
    match win_attrs(&n) {
        Ok((a, _)) => a,
        Err(e) => {
            log::trace!("GetFileAttributesW({n:?}) -> error {e}");
            set_err(e);
            INVALID_FILE_ATTRIBUTES
        }
    }
}

pub unsafe extern "win64" fn GetFileAttributesA(name: LPCSTR) -> u32 {
    let w = to_wide_z(&from_ansi(name));
    GetFileAttributesW(w.as_ptr())
}

pub unsafe extern "win64" fn GetFileAttributesExW(name: LPCWSTR, _level: u32, out: *mut u8) -> BOOL {
    let n = from_wide(name);
    match win_attrs(&n) {
        Ok((a, m)) => {
            let t = meta_times(&m);
            (out as *mut u32).write_unaligned(a);
            put_ft(out.add(4), t.create);
            put_ft(out.add(12), t.access);
            put_ft(out.add(20), t.write);
            let len = if m.is_dir() { 0 } else { m.len() };
            (out.add(28) as *mut u32).write_unaligned((len >> 32) as u32);
            (out.add(32) as *mut u32).write_unaligned(len as u32);
            TRUE
        }
        Err(e) => {
            set_err(e);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn GetFileAttributesExA(name: LPCSTR, level: u32, out: *mut u8) -> BOOL {
    let w = to_wide_z(&from_ansi(name));
    GetFileAttributesExW(w.as_ptr(), level, out)
}

pub unsafe extern "win64" fn SetFileAttributesW(name: LPCWSTR, attrs: u32) -> BOOL {
    let n = from_wide(name);
    let host = match host_path(&n) {
        Ok(h) => h,
        Err(e) => {
            set_err(e);
            return FALSE;
        }
    };
    if path::is_protected_host_write(&host) {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    let Ok(m) = std::fs::metadata(&host) else {
        set_err(not_found_err(&host));
        return FALSE;
    };
    let mut perm = m.permissions();
    let mode = perm.mode();
    perm.set_mode(if attrs & FILE_ATTRIBUTE_READONLY != 0 { mode & !0o222 } else { mode | 0o200 });
    let _ = std::fs::set_permissions(&host, perm);
    TRUE
}

fn path_op(name: &str, op: impl FnOnce(&Path) -> std::io::Result<()>) -> BOOL {
    let host = match host_path(name) {
        Ok(h) => h,
        Err(e) => {
            set_err(e);
            return FALSE;
        }
    };
    match op(&host) {
        Ok(()) => TRUE,
        Err(e) => {
            let win_err = match e.raw_os_error() {
                Some(libc::ENOENT) => not_found_err(&host),
                Some(libc::EEXIST) | Some(libc::EISDIR) => err::ERROR_ALREADY_EXISTS,
                Some(libc::ENOTEMPTY) => err::ERROR_DIR_NOT_EMPTY,
                _ => io_err(&e),
            };
            if win_err != err::ERROR_ALREADY_EXISTS && win_err != err::ERROR_FILE_NOT_FOUND && win_err != err::ERROR_PATH_NOT_FOUND {
                log::warn!("path_op({name:?} -> {host:?}) failed: {e} (win_err={win_err})");
            }
            set_err(win_err);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn CreateDirectoryW(name: LPCWSTR, _sec: *mut c_void) -> BOOL {
    path_op(&from_wide(name), |p| {
        if path::is_protected_host_write(p) {
            return Err(std::io::Error::from_raw_os_error(libc::EACCES));
        }
        if p.is_dir() {
            Err(std::io::Error::from_raw_os_error(libc::EEXIST))
        } else {
            std::fs::create_dir(p)
        }
    })
}
pub unsafe extern "win64" fn CreateDirectoryA(name: LPCSTR, _sec: *mut c_void) -> BOOL {
    path_op(&from_ansi(name), |p| {
        if path::is_protected_host_write(p) {
            return Err(std::io::Error::from_raw_os_error(libc::EACCES));
        }
        if p.is_dir() {
            Err(std::io::Error::from_raw_os_error(libc::EEXIST))
        } else {
            std::fs::create_dir(p)
        }
    })
}
pub unsafe extern "win64" fn CreateDirectoryExW(
    _template_dir: LPCWSTR,
    new_dir: LPCWSTR,
    sec: *mut c_void,
) -> BOOL {
    CreateDirectoryW(new_dir, sec)
}
pub unsafe extern "win64" fn CreateDirectoryExA(
    _template_dir: LPCSTR,
    new_dir: LPCSTR,
    sec: *mut c_void,
) -> BOOL {
    CreateDirectoryA(new_dir, sec)
}
pub unsafe extern "win64" fn RemoveDirectoryW(name: LPCWSTR) -> BOOL {
    path_op(&from_wide(name), |p| {
        if path::is_protected_host_remove_or_rename(p) {
            return Err(std::io::Error::from_raw_os_error(libc::EACCES));
        }
        match std::fs::remove_dir(p) {
            Ok(()) => Ok(()),
            Err(e) if e.raw_os_error() == Some(libc::ENOENT) => Ok(()),
            Err(e) => Err(e),
        }
    })
}
pub unsafe extern "win64" fn RemoveDirectoryA(name: LPCSTR) -> BOOL {
    path_op(&from_ansi(name), |p| {
        if path::is_protected_host_remove_or_rename(p) {
            return Err(std::io::Error::from_raw_os_error(libc::EACCES));
        }
        match std::fs::remove_dir(p) {
            Ok(()) => Ok(()),
            Err(e) if e.raw_os_error() == Some(libc::ENOENT) => Ok(()),
            Err(e) => Err(e),
        }
    })
}
pub unsafe extern "win64" fn DeleteFileW(name: LPCWSTR) -> BOOL {
    path_op(&from_wide(name), |p| {
        if path::is_protected_host_remove_or_rename(p) {
            return Err(std::io::Error::from_raw_os_error(libc::EACCES));
        }
        if p.is_dir() {
            Err(std::io::Error::from_raw_os_error(libc::EACCES))
        } else {
            std::fs::remove_file(p)
        }
    })
}
pub unsafe extern "win64" fn DeleteFileA(name: LPCSTR) -> BOOL {
    let w = to_wide_z(&from_ansi(name));
    DeleteFileW(w.as_ptr())
}

const MOVEFILE_REPLACE_EXISTING: u32 = 1;
const MOVEFILE_COPY_ALLOWED: u32 = 2;
const MOVEFILE_DELAY_UNTIL_REBOOT: u32 = 4;
const MOVEFILE_WRITE_THROUGH: u32 = 8;

pub fn move_file(src: &str, dst: &str, flags: u32) -> BOOL {
    let (s, d) = match (host_path(src), host_path(dst)) {
        (Ok(s), Ok(d)) => (s, d),
        (Err(e), _) | (_, Err(e)) => {
            set_err(e);
            return FALSE;
        }
    };
    if path::is_protected_host_remove_or_rename(&s) || path::is_protected_host_remove_or_rename(&d) {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    if !s.exists() {
        set_err(not_found_err(&s));
        return FALSE;
    }
    if d.exists() && flags & MOVEFILE_REPLACE_EXISTING == 0 {
        set_err(err::ERROR_ALREADY_EXISTS);
        return FALSE;
    }
    let ok = match std::fs::rename(&s, &d) {
        Ok(()) => true,
        Err(e) if e.raw_os_error() == Some(libc::EXDEV) && flags & MOVEFILE_COPY_ALLOWED != 0 => {
            match std::fs::copy(&s, &d).and_then(|_| std::fs::remove_file(&s)) {
                Ok(()) => true,
                Err(e) => {
                    log::warn!("move_file({src:?} -> {dst:?}) cross-device copy failed: {e}");
                    set_err(io_err(&e));
                    false
                }
            }
        }
        Err(e) => {
            log::warn!("move_file({src:?} -> {dst:?}) failed: {e}");
            set_err(io_err(&e));
            false
        }
    };
    if ok && (flags & MOVEFILE_WRITE_THROUGH) != 0 {
        if let Ok(f) = File::open(&d) {
            let _ = f.sync_all();
        }
    }
    ok as BOOL
}

fn move_file_ex(src: Option<String>, dst: Option<String>, flags: u32) -> BOOL {
    let Some(src_s) = src else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    if (flags & MOVEFILE_DELAY_UNTIL_REBOOT) != 0 {
        // Microsoft Learn (`MoveFileExW`):
        // - `MOVEFILE_DELAY_UNTIL_REBOOT` cannot be combined with `MOVEFILE_COPY_ALLOWED`.
        // - Scheduling a reboot-time rename/delete writes `PendingFileRenameOperations` under
        //   HKLM and requires Administrator/LocalSystem privilege; a standard non-elevated user
        //   process receives `FALSE` with `ERROR_ACCESS_DENIED` (5).
        if (flags & MOVEFILE_COPY_ALLOWED) != 0 {
            set_err(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    let Some(dst_s) = dst else {
        // `lpNewFileName == NULL` is only valid with `MOVEFILE_DELAY_UNTIL_REBOOT`.
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    move_file(&src_s, &dst_s, flags)
}

pub unsafe extern "win64" fn MoveFileExW(src: LPCWSTR, dst: LPCWSTR, flags: u32) -> BOOL {
    move_file_ex(from_wide_opt(src), from_wide_opt(dst), flags)
}
pub unsafe extern "win64" fn MoveFileExA(src: LPCSTR, dst: LPCSTR, flags: u32) -> BOOL {
    move_file_ex(from_ansi_opt(src), from_ansi_opt(dst), flags)
}
pub unsafe extern "win64" fn MoveFileW(src: LPCWSTR, dst: LPCWSTR) -> BOOL {
    move_file(&from_wide(src), &from_wide(dst), MOVEFILE_COPY_ALLOWED)
}
pub unsafe extern "win64" fn MoveFileA(src: LPCSTR, dst: LPCSTR) -> BOOL {
    move_file(&from_ansi(src), &from_ansi(dst), MOVEFILE_COPY_ALLOWED)
}

pub unsafe extern "win64" fn CopyFileW(src: LPCWSTR, dst: LPCWSTR, fail_if_exists: BOOL) -> BOOL {
    let src_s = from_wide(src);
    let dst_s = from_wide(dst);
    let (s, d) = match (host_path(&src_s), host_path(&dst_s)) {
        (Ok(s), Ok(d)) => (s, d),
        (Err(e), _) | (_, Err(e)) => {
            set_err(e);
            return FALSE;
        }
    };
    if path::is_protected_host_write(&d) {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    if fail_if_exists != 0 && d.exists() {
        set_err(err::ERROR_FILE_EXISTS);
        return FALSE;
    }
    match std::fs::copy(&s, &d) {
        Ok(_) => TRUE,
        Err(e) => {
            log::warn!("CopyFileW({src_s:?} -> {dst_s:?}) failed: {e}");
            set_err(if e.raw_os_error() == Some(libc::ENOENT) { not_found_err(&s) } else { io_err(&e) });
            FALSE
        }
    }
}

pub unsafe extern "win64" fn CopyFileA(src: LPCSTR, dst: LPCSTR, fail_if_exists: BOOL) -> BOOL {
    let ws = to_wide_z(&from_ansi(src));
    let wd = to_wide_z(&from_ansi(dst));
    CopyFileW(ws.as_ptr(), wd.as_ptr(), fail_if_exists)
}

pub unsafe extern "win64" fn CopyFileExW(
    src: LPCWSTR,
    dst: LPCWSTR,
    _progress_routine: *const c_void,
    _data: *mut c_void,
    _cancel: *mut BOOL,
    copy_flags: u32,
) -> BOOL {
    const COPY_FILE_FAIL_IF_EXISTS: u32 = 0x1;
    CopyFileW(src, dst, (copy_flags & COPY_FILE_FAIL_IF_EXISTS != 0) as BOOL)
}

pub unsafe extern "win64" fn ReplaceFileW(
    replaced: LPCWSTR,
    replacement: LPCWSTR,
    backup: LPCWSTR,
    _flags: u32,
    _exclude: *mut c_void,
    _reserved: *mut c_void,
) -> BOOL {
    if !backup.is_null() {
        let _ = CopyFileW(replaced, backup, FALSE);
    }
    MoveFileExW(replacement, replaced, MOVEFILE_REPLACE_EXISTING | MOVEFILE_COPY_ALLOWED)
}

#[derive(Clone, Copy)]
pub(crate) struct IocpPacket {
    pub bytes: u32,
    pub key: usize,
    pub overlapped: usize,
    pub internal_status: usize,
}

pub(crate) enum IocpDequeue {
    Packets(Vec<IocpPacket>),
    Timeout,
    IoCompletion,
}

pub(crate) struct IoCompletionPort {
    queue: std::sync::Mutex<std::collections::VecDeque<IocpPacket>>,
    /// Auto-reset event signaled whenever packets are available; enables
    /// alertable waits in `GetQueuedCompletionStatusEx` via `handles::wait_for_duration`
    /// so queued APCs wake blocked IOCP waiters immediately without polling.
    ready: Arc<Event>,
}

impl handles::KObject for IoCompletionPort {
    fn type_name(&self) -> &'static str {
        "IoCompletionPort"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl IoCompletionPort {
    fn new() -> Self {
        Self {
            queue: std::sync::Mutex::new(std::collections::VecDeque::new()),
            ready: Arc::new(Event::new(false, false, None)),
        }
    }

    pub(crate) fn post(&self, pkt: IocpPacket) {
        {
            let mut q = self.queue.lock().unwrap();
            q.push_back(pkt);
        }
        self.ready.set();
    }

    fn try_pop_up_to(&self, max: usize) -> Vec<IocpPacket> {
        let mut q = self.queue.lock().unwrap();
        if q.is_empty() {
            return Vec::new();
        }
        let n = q.len().min(max.max(1));
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(pkt) = q.pop_front() {
                out.push(pkt);
            }
        }
        let more = !q.is_empty();
        drop(q);
        if more {
            self.ready.set();
        }
        out
    }

    fn dequeue_up_to(&self, max: usize, timeout_ms: u32, alertable: bool) -> IocpDequeue {
        let timeout = (timeout_ms != INFINITE).then(|| Duration::from_millis(timeout_ms as u64));
        self.dequeue_up_to_duration(max, timeout, alertable)
    }

    pub(crate) fn dequeue_up_to_duration(
        &self,
        max: usize,
        timeout: Option<Duration>,
        alertable: bool,
    ) -> IocpDequeue {
        if alertable && crate::thread::has_pending_apc() {
            crate::thread::run_pending_apcs();
            return IocpDequeue::IoCompletion;
        }
        let is_zero = matches!(timeout, Some(d) if d.is_zero());
        let deadline = timeout.and_then(|d| if d.is_zero() { None } else { Some(Instant::now() + d) });
        let ready_obj: Arc<dyn KObject> = Arc::clone(&self.ready) as Arc<dyn KObject>;
        loop {
            let out = self.try_pop_up_to(max);
            if !out.is_empty() {
                return IocpDequeue::Packets(out);
            }
            if is_zero {
                return IocpDequeue::Timeout;
            }
            let remaining = match deadline {
                None => None,
                Some(dl) => {
                    let now = Instant::now();
                    if now >= dl {
                        return IocpDequeue::Timeout;
                    }
                    Some(dl - now)
                }
            };
            match handles::wait_for_duration(std::slice::from_ref(&ready_obj), false, remaining, alertable) {
                handles::WaitResult::IoCompletion => return IocpDequeue::IoCompletion,
                handles::WaitResult::Timeout => {
                    let out = self.try_pop_up_to(max);
                    if !out.is_empty() {
                        return IocpDequeue::Packets(out);
                    }
                    return IocpDequeue::Timeout;
                }
                handles::WaitResult::Object(_) => continue,
                _ => return IocpDequeue::Timeout,
            }
        }
    }
}

/// Shared completion sink state for both `FileObj` and external I/O handles (such as Winsock2
/// `SOCKET` handles). Held behind an `Arc` without any back-reference to `FileObj` so in-flight
/// pending operations can retain completion sinks across concurrent `CloseHandle`/`closesocket`
/// without creating reference cycles.
pub(crate) struct IoCompletionBinding {
    iocp: Mutex<Option<(Arc<IoCompletionPort>, usize)>>,
    tp_io: Mutex<Option<Arc<TpIo>>>,
    completion_modes: AtomicU8,
    /// Manual-reset event representing handle completion state (used when
    /// `GetOverlappedResultEx` waits on the handle itself with `ov.h_event == NULL`).
    io_event: Arc<Event>,
}

struct CapturedCompletionBindings {
    modes: u8,
    iocp: Option<(Arc<IoCompletionPort>, usize)>,
    tp_io: Option<Arc<TpIo>>,
    handle_event: Arc<Event>,
}

impl IoCompletionBinding {
    fn new() -> Self {
        Self {
            iocp: Mutex::new(None),
            tp_io: Mutex::new(None),
            completion_modes: AtomicU8::new(0),
            io_event: Arc::new(Event::new(true, true, None)),
        }
    }

    fn snapshot(&self) -> CapturedCompletionBindings {
        CapturedCompletionBindings {
            modes: self.completion_modes.load(Ordering::Relaxed),
            iocp: self.iocp.lock().unwrap().clone(),
            tp_io: self.tp_io.lock().unwrap().clone(),
            handle_event: Arc::clone(&self.io_event),
        }
    }

    fn has_sink(&self) -> bool {
        self.iocp.lock().unwrap().is_some() || self.tp_io.lock().unwrap().is_some()
    }

    fn bind_iocp(&self, iocp: Arc<IoCompletionPort>, key: usize) -> Result<(), u32> {
        let mut iocp_g = self.iocp.lock().unwrap();
        let tp_g = self.tp_io.lock().unwrap();
        if tp_g.is_some() {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        *iocp_g = Some((iocp, key));
        Ok(())
    }

    fn bind_tp_io(&self, tp_io: Arc<TpIo>) -> Result<(), u32> {
        let iocp_g = self.iocp.lock().unwrap();
        let mut tp_g = self.tp_io.lock().unwrap();
        if iocp_g.is_some() || tp_g.is_some() {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        *tp_g = Some(tp_io);
        Ok(())
    }

    fn unbind_tp_io(&self, tp_io: &Arc<TpIo>) {
        let mut tp_g = self.tp_io.lock().unwrap();
        if tp_g.as_ref().is_some_and(|cur| Arc::ptr_eq(cur, tp_io)) {
            *tp_g = None;
        }
    }
}

fn external_io_table() -> &'static Mutex<std::collections::HashMap<usize, Arc<IoCompletionBinding>>> {
    static T: OnceLock<Mutex<std::collections::HashMap<usize, Arc<IoCompletionBinding>>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

pub(crate) type ExternalCancelFn = unsafe fn(HANDLE, *mut OVERLAPPED, bool) -> Option<bool>;

fn external_cancel_hooks() -> &'static Mutex<Vec<ExternalCancelFn>> {
    static H: OnceLock<Mutex<Vec<ExternalCancelFn>>> = OnceLock::new();
    H.get_or_init(|| Mutex::new(Vec::new()))
}

pub(crate) fn register_external_cancel_hook(f: ExternalCancelFn) {
    let mut hooks = external_cancel_hooks().lock().unwrap();
    if !hooks.iter().any(|&h| h as usize == f as usize) {
        hooks.push(f);
    }
}

pub(crate) fn register_external_io_handle(h: HANDLE) {
    let mut t = external_io_table().lock().unwrap();
    t.entry(h as usize)
        .or_insert_with(|| Arc::new(IoCompletionBinding::new()));
}

pub(crate) fn unregister_external_io_handle(h: HANDLE) {
    external_io_table().lock().unwrap().remove(&(h as usize));
}

pub(crate) fn lookup_io_completion_binding(h: HANDLE) -> Option<Arc<IoCompletionBinding>> {
    if let Some(f) = get_file(h) {
        return Some(Arc::clone(&f.completion));
    }
    external_io_table().lock().unwrap().get(&(h as usize)).cloned()
}

pub(crate) fn external_io_event(h: HANDLE) -> Option<Arc<Event>> {
    external_io_table()
        .lock()
        .unwrap()
        .get(&(h as usize))
        .map(|e| Arc::clone(&e.io_event))
}

#[allow(dead_code)]
pub(crate) fn handle_has_completion_sink(h: HANDLE) -> bool {
    lookup_io_completion_binding(h).is_some_and(|b| b.has_sink())
}

/// Bind a `TpIo` completion sink to a `FileObj` or registered external I/O handle (e.g. `SOCKET`).
/// Fails with `ERROR_INVALID_HANDLE` if `h` is not a valid I/O handle, or `ERROR_INVALID_PARAMETER`
/// if `h` is already bound to an IOCP or another `TpIo`.
pub(crate) fn bind_threadpool_io(h: HANDLE, tp_io: Arc<TpIo>) -> Result<(), u32> {
    let Some(binding) = lookup_io_completion_binding(h) else {
        return Err(err::ERROR_INVALID_HANDLE);
    };
    binding.bind_tp_io(tp_io)
}

/// Detach a `TpIo` sink from `h` when `CloseThreadpoolIo` is called.
pub(crate) fn unbind_threadpool_io(h: HANDLE, tp_io: &Arc<TpIo>) {
    if let Some(binding) = lookup_io_completion_binding(h) {
        binding.unbind_tp_io(tp_io);
    }
}

#[inline(always)]
unsafe fn complete_overlapped_binding_with_hook<F: FnOnce()>(
    binding: Option<&IoCompletionBinding>,
    ov: *mut OVERLAPPED,
    nt_status: usize,
    bytes: usize,
    synchronous: bool,
    after_signal: F,
) -> bool {
    const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u8 = 0x1;
    const FILE_SKIP_SET_EVENT_ON_HANDLE: u8 = 0x2;
    if ov.is_null() {
        return false;
    }
    // Capture guest OVERLAPPED fields, explicit event object, completion modes, handle event,
    // and strong IOCP/TpIo references BEFORE publishing terminal `ov.internal` or signaling any
    // event so a consumer waking on `ov.internal` or `ov.h_event` can immediately close handles
    // or free/reuse `ov` without racing with any subsequent handle-table or guest dereference.
    let ov_addr = ov as usize;
    let raw_ev = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).h_event)) as usize;
    let suppress_iocp = (raw_ev & 1) != 0;
    let ev_h = (raw_ev & !1) as HANDLE;
    let explicit_event = if !ev_h.is_null() {
        handles::get_as::<Event>(ev_h)
    } else {
        None
    };
    let captured = binding.map(IoCompletionBinding::snapshot);

    store_ov_internal_high(ov, bytes);
    store_ov_internal(ov, nt_status);

    // Post-publication: zero guest-memory dereferences and zero handle-table lookups.
    if let Some(e) = explicit_event {
        e.set();
    }
    after_signal();

    let Some(c) = captured else {
        return false;
    };
    if (c.modes & FILE_SKIP_SET_EVENT_ON_HANDLE) == 0 {
        c.handle_event.set();
    }

    if suppress_iocp {
        return false;
    }
    if synchronous && is_io_success_status(nt_status) && (c.modes & FILE_SKIP_COMPLETION_PORT_ON_SUCCESS) != 0 {
        return false;
    }
    if let Some((iocp, key)) = c.iocp {
        iocp.post(IocpPacket {
            bytes: bytes as u32,
            key,
            overlapped: ov_addr,
            internal_status: nt_status,
        });
        return true;
    }
    if let Some(tp_io) = c.tp_io {
        return tp_io.dispatch_completion(ov_addr, ntstatus_to_win32_local(nt_status), bytes);
    }
    false
}

/// Complete an overlapped I/O operation using a retained `IoCompletionBinding`.
pub(crate) unsafe fn complete_overlapped_binding(
    binding: Option<&IoCompletionBinding>,
    ov: *mut OVERLAPPED,
    nt_status: usize,
    bytes: usize,
    synchronous: bool,
) -> bool {
    complete_overlapped_binding_with_hook(binding, ov, nt_status, bytes, synchronous, || {})
}

/// Complete an overlapped I/O operation on either a `FileObj` or a registered external I/O handle (e.g. SOCKET).
/// Captures all completion bindings and events before atomically publishing `InternalHigh`/`Internal`,
/// signaling `ov.h_event` (and the handle's event), and posting to any associated IOCP or `TpIo`.
pub(crate) unsafe fn complete_overlapped_io(
    h: HANDLE,
    ov: *mut OVERLAPPED,
    nt_status: usize,
    bytes: usize,
    synchronous: bool,
) -> bool {
    let binding = lookup_io_completion_binding(h);
    complete_overlapped_binding(binding.as_deref(), ov, nt_status, bytes, synchronous)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: usize,
    pub lpOverlapped: *mut OVERLAPPED,
    pub Internal: usize,
    pub dwNumberOfBytesTransferred: u32,
}

pub unsafe extern "win64" fn CreateIoCompletionPort(
    file_handle: HANDLE,
    existing_port: HANDLE,
    completion_key: usize,
    _concurrent_threads: u32,
) -> HANDLE {
    let (port_handle, iocp) = if !existing_port.is_null() {
        let Some(p) = handles::get_as::<IoCompletionPort>(existing_port) else {
            set_err(err::ERROR_INVALID_HANDLE);
            return std::ptr::null_mut();
        };
        (existing_port, p)
    } else {
        let p = std::sync::Arc::new(IoCompletionPort::new());
        let p2 = std::sync::Arc::clone(&p);
        let h = handles::insert(p2);
        (h, p)
    };
    if !file_handle.is_null() && file_handle != INVALID_HANDLE_VALUE {
        let Some(binding) = lookup_io_completion_binding(file_handle) else {
            if existing_port.is_null() {
                handles::close(port_handle);
            }
            set_err(err::ERROR_INVALID_HANDLE);
            return std::ptr::null_mut();
        };
        if let Err(e) = binding.bind_iocp(iocp, completion_key) {
            if existing_port.is_null() {
                handles::close(port_handle);
            }
            set_err(e);
            return std::ptr::null_mut();
        }
    }
    port_handle
}

pub unsafe extern "win64" fn PostQueuedCompletionStatus(
    port: HANDLE,
    bytes: u32,
    key: usize,
    overlapped: *mut OVERLAPPED,
) -> BOOL {
    let Some(iocp) = handles::get_as::<IoCompletionPort>(port) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    iocp.post(IocpPacket {
        bytes,
        key,
        overlapped: overlapped as usize,
        internal_status: STATUS_SUCCESS,
    });
    TRUE
}

pub unsafe extern "win64" fn GetQueuedCompletionStatus(
    port: HANDLE,
    bytes: *mut u32,
    key: *mut usize,
    overlapped: *mut *mut OVERLAPPED,
    timeout_ms: u32,
) -> BOOL {
    let Some(iocp) = handles::get_as::<IoCompletionPort>(port) else {
        if !overlapped.is_null() {
            std::ptr::write_unaligned(overlapped, std::ptr::null_mut());
        }
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let mut got = match iocp.dequeue_up_to(1, timeout_ms, false) {
        IocpDequeue::Packets(v) => v,
        IocpDequeue::Timeout | IocpDequeue::IoCompletion => Vec::new(),
    };
    if let Some(pkt) = got.pop() {
        if !bytes.is_null() {
            std::ptr::write_unaligned(bytes, pkt.bytes);
        }
        if !key.is_null() {
            std::ptr::write_unaligned(key, pkt.key);
        }
        if !overlapped.is_null() {
            std::ptr::write_unaligned(overlapped, pkt.overlapped as *mut OVERLAPPED);
        }
        if !is_io_success_status(pkt.internal_status) {
            set_err(ntstatus_to_win32_local(pkt.internal_status));
            return FALSE;
        }
        TRUE
    } else {
        if !bytes.is_null() {
            std::ptr::write_unaligned(bytes, 0);
        }
        if !key.is_null() {
            std::ptr::write_unaligned(key, 0);
        }
        if !overlapped.is_null() {
            std::ptr::write_unaligned(overlapped, std::ptr::null_mut());
        }
        set_err(WAIT_TIMEOUT);
        FALSE
    }
}

pub unsafe extern "win64" fn GetQueuedCompletionStatusEx(
    port: HANDLE,
    entries: *mut OVERLAPPED_ENTRY,
    count: u32,
    num_removed: *mut u32,
    timeout_ms: u32,
    alertable: BOOL,
) -> BOOL {
    if entries.is_null() || count == 0 || num_removed.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(iocp) = handles::get_as::<IoCompletionPort>(port) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    match iocp.dequeue_up_to(count as usize, timeout_ms, alertable != 0) {
        IocpDequeue::IoCompletion => {
            std::ptr::write_unaligned(num_removed, 0);
            set_err(WAIT_IO_COMPLETION);
            FALSE
        }
        IocpDequeue::Timeout => {
            std::ptr::write_unaligned(num_removed, 0);
            set_err(WAIT_TIMEOUT);
            FALSE
        }
        IocpDequeue::Packets(got) => {
            for (i, pkt) in got.iter().enumerate() {
                std::ptr::write_unaligned(
                    entries.add(i),
                    OVERLAPPED_ENTRY {
                        lpCompletionKey: pkt.key,
                        lpOverlapped: pkt.overlapped as *mut OVERLAPPED,
                        Internal: pkt.internal_status,
                        dwNumberOfBytesTransferred: pkt.bytes,
                    },
                );
            }
            std::ptr::write_unaligned(num_removed, got.len() as u32);
            TRUE
        }
    }
}

pub unsafe extern "win64" fn SetFileCompletionNotificationModes(
    file_handle: HANDLE,
    flags: u8,
) -> BOOL {
    if file_handle.is_null() || file_handle == INVALID_HANDLE_VALUE {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    // FILE_SKIP_COMPLETION_PORT_ON_SUCCESS (0x1) | FILE_SKIP_SET_EVENT_ON_HANDLE (0x2)
    if (flags & !0x3) != 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if let Some(binding) = lookup_io_completion_binding(file_handle) {
        binding.completion_modes.store(flags, Ordering::Relaxed);
        return TRUE;
    }
    set_err(err::ERROR_INVALID_HANDLE);
    FALSE
}

pub unsafe extern "win64" fn CreateHardLinkW(new: LPCWSTR, existing: LPCWSTR, _sec: *mut c_void) -> BOOL {
    let (n, e) = match (host_path(&from_wide(new)), host_path(&from_wide(existing))) {
        (Ok(n), Ok(e)) => (n, e),
        (Err(x), _) | (_, Err(x)) => {
            set_err(x);
            return FALSE;
        }
    };
    if path::is_protected_host_write(&n) || path::is_protected_host_alias_target(&e) {
        set_err(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }
    match std::fs::hard_link(&e, &n) {
        Ok(()) => TRUE,
        Err(x) => {
            set_err(io_err(&x));
            FALSE
        }
    }
}

pub unsafe extern "win64" fn CreateSymbolicLinkW(link: LPCWSTR, target: LPCWSTR, _flags: u32) -> u8 {
    let (l, t) = match (host_path(&from_wide(link)), host_path(&from_wide(target))) {
        (Ok(l), Ok(t)) => (l, t),
        (Err(x), _) | (_, Err(x)) => {
            set_err(x);
            return 0;
        }
    };
    if path::is_protected_host_write(&l) || path::is_protected_host_alias_target(&t) {
        set_err(err::ERROR_ACCESS_DENIED);
        return 0;
    }
    match std::os::unix::fs::symlink(&t, &l) {
        Ok(()) => 1,
        Err(x) => {
            set_err(io_err(&x));
            0
        }
    }
}

pub unsafe extern "win64" fn GetFullPathNameW(name: LPCWSTR, cap: u32, buf: *mut u16, file_part: *mut *mut u16) -> u32 {
    let full = path::absolute(&from_wide(name));
    let w = to_wide(&full);
    if w.len() as u32 + 1 > cap {
        return w.len() as u32 + 1;
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
    *buf.add(w.len()) = 0;
    if !file_part.is_null() {
        *file_part = match w.iter().rposition(|&c| c == b'\\' as u16) {
            Some(i) if i + 1 < w.len() => buf.add(i + 1),
            _ => std::ptr::null_mut(),
        };
    }
    w.len() as u32
}

pub unsafe extern "win64" fn GetFullPathNameA(name: LPCSTR, cap: u32, buf: *mut u8, file_part: *mut *mut u8) -> u32 {
    let full = path::absolute(&from_ansi(name));
    if full.len() as u32 + 1 > cap {
        return full.len() as u32 + 1;
    }
    write_ansi_buf(&full, buf, cap as usize);
    if !file_part.is_null() {
        *file_part = match full.rfind('\\') {
            Some(i) if i + 1 < full.len() => buf.add(i + 1),
            _ => std::ptr::null_mut(),
        };
    }
    full.len() as u32
}

pub unsafe extern "win64" fn GetLongPathNameW(short: LPCWSTR, buf: *mut u16, cap: u32) -> u32 {
    // We never generate 8.3 names; the long name is the input, if it exists.
    let s = from_wide(short);
    if let Err(e) = win_attrs(&s) {
        set_err(e);
        return 0;
    }
    let n = s.encode_utf16().count() as u32;
    if n + 1 > cap {
        return n + 1;
    }
    write_wide_buf(&s, buf, cap as usize);
    n
}

pub unsafe extern "win64" fn GetShortPathNameW(long: LPCWSTR, buf: *mut u16, cap: u32) -> u32 {
    GetLongPathNameW(long, buf, cap)
}

pub unsafe extern "win64" fn GetCurrentDirectoryW(cap: u32, buf: *mut u16) -> u32 {
    let c = path::cwd();
    let n = c.encode_utf16().count() as u32;
    if n + 1 > cap {
        return n + 1;
    }
    write_wide_buf(&c, buf, cap as usize);
    n
}

pub unsafe extern "win64" fn GetCurrentDirectoryA(cap: u32, buf: *mut u8) -> u32 {
    let c = path::cwd();
    if c.len() as u32 + 1 > cap {
        return c.len() as u32 + 1;
    }
    write_ansi_buf(&c, buf, cap as usize);
    c.len() as u32
}

pub unsafe extern "win64" fn SetCurrentDirectoryW(p: LPCWSTR) -> BOOL {
    let s = from_wide(p);
    match host_path(&s) {
        Ok(h) if h.is_dir() => {
            path::set_cwd(&s);
            TRUE
        }
        Ok(h) => {
            set_err(not_found_err(&h));
            FALSE
        }
        Err(e) => {
            set_err(e);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn SetCurrentDirectoryA(p: LPCSTR) -> BOOL {
    let w = to_wide_z(&from_ansi(p));
    SetCurrentDirectoryW(w.as_ptr())
}

/// `%TEMP%` (from the environment set up by the loader), with trailing `\`.
pub fn temp_dir() -> String {
    let t = crate::process::get_env("TMP")
        .or_else(|| crate::process::get_env("TEMP"))
        .unwrap_or_else(|| "C:\\Windows\\Temp".into());
    let mut t = path::absolute(&t);
    if !t.ends_with('\\') {
        t.push('\\');
    }
    if let Ok(h) = host_path(&t) {
        let _ = std::fs::create_dir_all(h);
    }
    t
}

pub unsafe extern "win64" fn GetTempPathW(cap: u32, buf: *mut u16) -> u32 {
    let t = temp_dir();
    let n = t.encode_utf16().count() as u32;
    if n + 1 > cap {
        return n + 1;
    }
    write_wide_buf(&t, buf, cap as usize);
    n
}

pub unsafe extern "win64" fn GetTempPathA(cap: u32, buf: *mut u8) -> u32 {
    let t = temp_dir();
    if t.len() as u32 + 1 > cap {
        return t.len() as u32 + 1;
    }
    write_ansi_buf(&t, buf, cap as usize);
    t.len() as u32
}

pub unsafe extern "win64" fn GetTempPath2W(cap: u32, buf: *mut u16) -> u32 {
    GetTempPathW(cap, buf)
}

pub unsafe extern "win64" fn GetTempFileNameW(dir: LPCWSTR, prefix: LPCWSTR, unique: u32, out: *mut u16) -> u32 {
    let d = from_wide(dir);
    let p: String = from_wide(prefix).chars().take(3).collect();
    let mut n = if unique != 0 { unique & 0xFFFF } else { (std::process::id() ^ (now_filetime() as u32)) & 0xFFFF };
    loop {
        let name = format!("{}\\{}{:X}.tmp", d.trim_end_matches('\\'), p, n);
        if unique != 0 {
            write_wide_buf(&name, out, 260);
            return n;
        }
        if let Ok(h) = host_path(&name) {
            if File::options().write(true).create_new(true).open(&h).is_ok() {
                write_wide_buf(&name, out, 260);
                return n;
            }
            if !parent_exists(&h) {
                set_err(err::ERROR_PATH_NOT_FOUND);
                return 0;
            }
        } else {
            set_err(err::ERROR_PATH_NOT_FOUND);
            return 0;
        }
        n = (n + 1) & 0xFFFF;
        if n == 0 {
            n = 1;
        }
    }
}

pub unsafe extern "win64" fn GetTempFileNameA(dir: LPCSTR, prefix: LPCSTR, unique: u32, out: *mut u8) -> u32 {
    let d = to_wide_z(&from_ansi(dir));
    let p = to_wide_z(&from_ansi(prefix));
    let mut w = [0u16; 260];
    let r = GetTempFileNameW(d.as_ptr(), p.as_ptr(), unique, w.as_mut_ptr());
    if r != 0 {
        write_ansi_buf(&from_wide(w.as_ptr()), out, 260);
    }
    r
}

pub unsafe extern "win64" fn GetDiskFreeSpaceExW(dir: LPCWSTR, avail: *mut u64, total: *mut u64, free: *mut u64) -> BOOL {
    let d = if dir.is_null() { path::cwd() } else { from_wide(dir) };
    let host = match host_path(&d) {
        Ok(h) => h,
        Err(e) => {
            set_err(e);
            return FALSE;
        }
    };
    let c = std::ffi::CString::new(host.as_os_str().as_encoded_bytes()).unwrap_or_default();
    let mut s: libc::statvfs = std::mem::zeroed();
    if libc::statvfs(c.as_ptr(), &mut s) != 0 {
        set_err(errno_to_win32(last_errno()));
        return FALSE;
    }
    let f = s.f_frsize as u64;
    if !avail.is_null() {
        *avail = s.f_bavail as u64 * f;
    }
    if !total.is_null() {
        *total = s.f_blocks as u64 * f;
    }
    if !free.is_null() {
        *free = s.f_bfree as u64 * f;
    }
    TRUE
}

pub unsafe extern "win64" fn GetDiskFreeSpaceExA(dir: LPCSTR, avail: *mut u64, total: *mut u64, free: *mut u64) -> BOOL {
    let w = if dir.is_null() { None } else { Some(to_wide_z(&from_ansi(dir))) };
    GetDiskFreeSpaceExW(w.as_ref().map_or(std::ptr::null(), |w| w.as_ptr()), avail, total, free)
}

pub unsafe extern "win64" fn GetDiskFreeSpaceW(
    dir: LPCWSTR,
    sectors_per_cluster: *mut u32,
    bytes_per_sector: *mut u32,
    free_clusters: *mut u32,
    total_clusters: *mut u32,
) -> BOOL {
    let mut avail = 0u64;
    let mut total = 0u64;
    if GetDiskFreeSpaceExW(dir, &mut avail, &mut total, std::ptr::null_mut()) == 0 {
        return FALSE;
    }
    let bps = 4096u64;
    let spc = 8u64;
    let cluster_bytes = bps * spc;
    if !sectors_per_cluster.is_null() {
        *sectors_per_cluster = spc as u32;
    }
    if !bytes_per_sector.is_null() {
        *bytes_per_sector = bps as u32;
    }
    if !free_clusters.is_null() {
        *free_clusters = (avail / cluster_bytes).min(u32::MAX as u64) as u32;
    }
    if !total_clusters.is_null() {
        *total_clusters = (total / cluster_bytes).min(u32::MAX as u64) as u32;
    }
    TRUE
}

pub unsafe extern "win64" fn GetDiskFreeSpaceA(
    dir: LPCSTR,
    sectors_per_cluster: *mut u32,
    bytes_per_sector: *mut u32,
    free_clusters: *mut u32,
    total_clusters: *mut u32,
) -> BOOL {
    let w = if dir.is_null() { None } else { Some(to_wide_z(&from_ansi(dir))) };
    GetDiskFreeSpaceW(w.as_ref().map_or(std::ptr::null(), |w| w.as_ptr()), sectors_per_cluster, bytes_per_sector, free_clusters, total_clusters)
}

pub unsafe extern "win64" fn GetDriveTypeW(_root: LPCWSTR) -> u32 {
    3 // DRIVE_FIXED
}

pub unsafe extern "win64" fn GetDriveTypeA(_root: LPCSTR) -> u32 {
    3 // DRIVE_FIXED
}

pub unsafe extern "win64" fn GetLogicalDrives() -> u32 {
    let mut mask = 1u32 << 2; // C: always present
    for (win_prefix, _) in path::list_mounts() {
        let b = win_prefix.as_bytes();
        if b.len() >= 2 && b[1] == b':' {
            let up = b[0].to_ascii_uppercase();
            if up.is_ascii_uppercase() {
                mask |= 1u32 << (up - b'A');
            }
        }
    }
    mask
}

fn mounted_drive_letters() -> Vec<char> {
    let mask = unsafe { GetLogicalDrives() };
    let mut drives = Vec::new();
    for i in 0..26u8 {
        if mask & (1u32 << i) != 0 {
            drives.push((b'A' + i) as char);
        }
    }
    drives
}

pub unsafe extern "win64" fn GetLogicalDriveStringsW(buf_len: u32, buf: LPWSTR) -> u32 {
    let mut wide: Vec<u16> = Vec::new();
    for d in mounted_drive_letters() {
        wide.push(d as u16);
        wide.push(b':' as u16);
        wide.push(b'\\' as u16);
        wide.push(0);
    }
    let chars_no_final_nul = wide.len() as u32;
    wide.push(0); // final terminating NUL
    if buf.is_null() || (buf_len as usize) < wide.len() {
        return wide.len() as u32;
    }
    for (i, &c) in wide.iter().enumerate() {
        std::ptr::write_unaligned(buf.add(i), c);
    }
    chars_no_final_nul
}

pub unsafe extern "win64" fn GetLogicalDriveStringsA(buf_len: u32, buf: LPSTR) -> u32 {
    let mut bytes: Vec<u8> = Vec::new();
    for d in mounted_drive_letters() {
        bytes.push(d as u8);
        bytes.push(b':');
        bytes.push(b'\\');
        bytes.push(0);
    }
    let chars_no_final_nul = bytes.len() as u32;
    bytes.push(0);
    if buf.is_null() || (buf_len as usize) < bytes.len() {
        return bytes.len() as u32;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
    chars_no_final_nul
}

pub unsafe extern "win64" fn QueryDosDeviceW(device_name: LPCWSTR, target_path: LPWSTR, max_chars: u32) -> u32 {
    if target_path.is_null() || max_chars == 0 {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    let mut wide: Vec<u16> = Vec::new();
    if device_name.is_null() {
        for d in mounted_drive_letters() {
            wide.push(d as u16);
            wide.push(b':' as u16);
            wide.push(0);
        }
        wide.push(0);
    } else {
        let dev = from_wide(device_name);
        let b = dev.as_bytes();
        if b.len() == 2 && b[1] == b':' && b[0].is_ascii_alphabetic() {
            let target = path::dos_device_target(b[0] as char);
            wide.extend(target.encode_utf16());
            wide.push(0);
            wide.push(0);
        } else {
            set_err(err::ERROR_FILE_NOT_FOUND);
            return 0;
        }
    }
    if (max_chars as usize) < wide.len() {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    for (i, &c) in wide.iter().enumerate() {
        std::ptr::write_unaligned(target_path.add(i), c);
    }
    set_err(0);
    wide.len() as u32
}

pub unsafe extern "win64" fn QueryDosDeviceA(device_name: LPCSTR, target_path: LPSTR, max_chars: u32) -> u32 {
    if target_path.is_null() || max_chars == 0 {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    let mut bytes: Vec<u8> = Vec::new();
    if device_name.is_null() {
        for d in mounted_drive_letters() {
            bytes.push(d as u8);
            bytes.push(b':');
            bytes.push(0);
        }
        bytes.push(0);
    } else {
        let dev = from_ansi(device_name);
        let b = dev.as_bytes();
        if b.len() == 2 && b[1] == b':' && b[0].is_ascii_alphabetic() {
            let target = path::dos_device_target(b[0] as char);
            bytes.extend_from_slice(target.as_bytes());
            bytes.push(0);
            bytes.push(0);
        } else {
            set_err(err::ERROR_FILE_NOT_FOUND);
            return 0;
        }
    }
    if (max_chars as usize) < bytes.len() {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return 0;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), target_path, bytes.len());
    set_err(0);
    bytes.len() as u32
}

fn volume_root_for(win_path: &str) -> String {
    let abs = path::absolute(win_path);
    let b = abs.as_bytes();
    if b.len() >= 2 && b[1] == b':' {
        format!("{}:\\", (b[0] as char).to_ascii_uppercase())
    } else {
        "C:\\".to_string()
    }
}

pub unsafe extern "win64" fn GetVolumePathNameW(
    file_name: LPCWSTR,
    vol_path: *mut u16,
    cap: u32,
) -> BOOL {
    if file_name.is_null() || vol_path.is_null() || cap == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let root = volume_root_for(&from_wide(file_name));
    let needed = root.encode_utf16().count() + 1;
    if (cap as usize) < needed {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    write_wide_buf(&root, vol_path, cap as usize);
    TRUE
}

pub unsafe extern "win64" fn GetVolumePathNameA(
    file_name: LPCSTR,
    vol_path: *mut u8,
    cap: u32,
) -> BOOL {
    if file_name.is_null() || vol_path.is_null() || cap == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let root = volume_root_for(&from_ansi(file_name));
    if (cap as usize) < root.len() + 1 {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    write_ansi_buf(&root, vol_path, cap as usize);
    TRUE
}

pub unsafe extern "win64" fn GetVolumeNameForVolumeMountPointW(
    mount_point: LPCWSTR,
    vol_name: *mut u16,
    cap: u32,
) -> BOOL {
    if mount_point.is_null() || vol_name.is_null() || cap == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let guid = "\\\\?\\Volume{00000000-0000-0000-0000-000000000001}\\";
    if (cap as usize) < guid.len() + 1 {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    write_wide_buf(guid, vol_name, cap as usize);
    TRUE
}

pub unsafe extern "win64" fn GetVolumeInformationW(
    _root: LPCWSTR,
    name: *mut u16,
    name_cap: u32,
    serial: *mut u32,
    max_comp: *mut u32,
    fs_flags: *mut u32,
    fs_name: *mut u16,
    fs_cap: u32,
) -> BOOL {
    if !name.is_null() {
        write_wide_trunc("Macintosh HD", name, name_cap as usize);
    }
    if !serial.is_null() {
        std::ptr::write_unaligned(serial, 0xA3C2_0001);
    }
    if !max_comp.is_null() {
        std::ptr::write_unaligned(max_comp, 255);
    }
    if !fs_flags.is_null() {
        std::ptr::write_unaligned(fs_flags, 0x3 | 0x4 | 0x8); // case preserved, unicode, persistent ACLs
    }
    if !fs_name.is_null() {
        write_wide_trunc("NTFS", fs_name, fs_cap as usize);
    }
    TRUE
}

pub unsafe extern "win64" fn GetVolumeInformationA(
    _root: LPCSTR,
    name: *mut u8,
    name_cap: u32,
    serial: *mut u32,
    max_comp: *mut u32,
    fs_flags: *mut u32,
    fs_name: *mut u8,
    fs_cap: u32,
) -> BOOL {
    if !name.is_null() && name_cap > 0 {
        write_ansi_buf("Macintosh HD", name, name_cap as usize);
    }
    if !serial.is_null() {
        std::ptr::write_unaligned(serial, 0xA3C2_0001);
    }
    if !max_comp.is_null() {
        std::ptr::write_unaligned(max_comp, 255);
    }
    if !fs_flags.is_null() {
        std::ptr::write_unaligned(fs_flags, 0x3 | 0x4 | 0x8);
    }
    if !fs_name.is_null() && fs_cap > 0 {
        write_ansi_buf("NTFS", fs_name, fs_cap as usize);
    }
    TRUE
}

pub unsafe extern "win64" fn GetVolumeInformationByHandleW(
    file: HANDLE,
    name: *mut u16,
    name_cap: u32,
    serial: *mut u32,
    max_comp: *mut u32,
    fs_flags: *mut u32,
    fs_name: *mut u16,
    fs_cap: u32,
) -> BOOL {
    if file.is_null() || file == INVALID_HANDLE_VALUE {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    GetVolumeInformationW(std::ptr::null(), name, name_cap, serial, max_comp, fs_flags, fs_name, fs_cap)
}

// ---------------------------------------------------------------------------
// FindFirstFile
// ---------------------------------------------------------------------------

struct FindEntry {
    name: String,
    attrs: u32,
    times: Times,
    size: u64,
}

pub struct FindObj {
    entries: Mutex<std::collections::VecDeque<FindEntry>>,
}

impl KObject for FindObj {
    fn type_name(&self) -> &'static str {
        "Find"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Case-insensitive Windows wildcard match (`*`, `?`; `*.*` matches all).
pub fn wildcard_match(pat: &str, name: &str) -> bool {
    if pat == "*" || pat == "*.*" {
        return true;
    }
    let p: Vec<char> = pat.chars().flat_map(|c| c.to_lowercase()).collect();
    let n: Vec<char> = name.chars().flat_map(|c| c.to_lowercase()).collect();
    fn m(p: &[char], n: &[char]) -> bool {
        match p.first() {
            None => n.is_empty(),
            Some('*') => (0..=n.len()).any(|i| m(&p[1..], &n[i..])),
            Some('?') => !n.is_empty() && m(&p[1..], &n[1..]) || n.is_empty() && m(&p[1..], n),
            Some(c) => n.first() == Some(c) && m(&p[1..], &n[1..]),
        }
    }
    // "foo.*" also matches "foo" on Windows.
    m(&p, &n) || (pat.ends_with(".*") && m(&p[..p.len() - 2], &n))
}

fn find_first(pattern: &str) -> Result<FindObj, u32> {
    let full = path::absolute(pattern);
    let (dir, pat) = match full.rfind('\\') {
        Some(i) => (&full[..i.max(3).min(full.len())], &full[i + 1..]),
        None => (full.as_str(), "*"),
    };
    let dir = if dir.len() == 2 { format!("{dir}\\") } else { dir.to_string() };
    let host = host_path(&dir)?;
    let mut entries = std::collections::VecDeque::new();
    let has_wild = pat.contains(['*', '?']);
    if !has_wild {
        // Exact name lookup (case-insensitive via the mount table).
        let p = host_path(&full)?;
        let m = std::fs::metadata(&p).map_err(|_| not_found_err(&p))?;
        let name = p.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| pat.to_string());
        entries.push_back(FindEntry { attrs: meta_attrs(&m, &name), times: meta_times(&m), size: m.len(), name });
        return Ok(FindObj { entries: Mutex::new(entries) });
    }
    let rd = std::fs::read_dir(&host).map_err(|e| match e.raw_os_error() {
        Some(libc::ENOENT) | Some(libc::ENOTDIR) => err::ERROR_PATH_NOT_FOUND,
        _ => io_err(&e),
    })?;
    let is_root = dir.len() <= 3;
    if !is_root {
        if let Ok(m) = std::fs::metadata(&host) {
            for dot in [".", ".."] {
                if wildcard_match(pat, dot) {
                    entries.push_back(FindEntry { name: dot.into(), attrs: FILE_ATTRIBUTE_DIRECTORY, times: meta_times(&m), size: 0 });
                }
            }
        }
    }
    let mut v: Vec<FindEntry> = rd
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if !wildcard_match(pat, &name) {
                return None;
            }
            let m = std::fs::metadata(e.path()).ok()?;
            Some(FindEntry { attrs: meta_attrs(&m, &name), times: meta_times(&m), size: if m.is_dir() { 0 } else { m.len() }, name })
        })
        .collect();
    v.sort_by_key(|e| e.name.to_lowercase());
    entries.extend(v);
    if entries.is_empty() {
        return Err(err::ERROR_FILE_NOT_FOUND);
    }
    Ok(FindObj { entries: Mutex::new(entries) })
}

unsafe fn fill_find_data_w(e: &FindEntry, out: *mut u8) {
    std::ptr::write_bytes(out, 0, 592);
    (out as *mut u32).write_unaligned(e.attrs);
    put_ft(out.add(4), e.times.create);
    put_ft(out.add(12), e.times.access);
    put_ft(out.add(20), e.times.write);
    (out.add(28) as *mut u32).write_unaligned((e.size >> 32) as u32);
    (out.add(32) as *mut u32).write_unaligned(e.size as u32);
    write_wide_trunc(&e.name, out.add(44) as *mut u16, 260);
}

unsafe fn fill_find_data_a(e: &FindEntry, out: *mut u8) {
    // WIN32_FIND_DATAA: same header, cFileName[260] at 44, cAlternateFileName[14]; 320 bytes.
    std::ptr::write_bytes(out, 0, 320);
    (out as *mut u32).write_unaligned(e.attrs);
    put_ft(out.add(4), e.times.create);
    put_ft(out.add(12), e.times.access);
    put_ft(out.add(20), e.times.write);
    (out.add(28) as *mut u32).write_unaligned((e.size >> 32) as u32);
    (out.add(32) as *mut u32).write_unaligned(e.size as u32);
    write_ansi_trunc(&e.name, out.add(44), 260);
}

unsafe fn find_first_common(pattern: String, out: *mut u8, wide: bool) -> HANDLE {
    match find_first(&pattern) {
        Ok(f) => {
            let first = f.entries.lock().unwrap().pop_front().unwrap();
            if wide {
                fill_find_data_w(&first, out);
            } else {
                fill_find_data_a(&first, out);
            }
            handles::insert(Arc::new(f))
        }
        Err(e) => {
            log::trace!("FindFirstFile({pattern:?}) -> {e}");
            set_err(e);
            INVALID_HANDLE_VALUE
        }
    }
}

pub unsafe extern "win64" fn FindFirstFileW(pattern: LPCWSTR, out: *mut u8) -> HANDLE {
    find_first_common(from_wide(pattern), out, true)
}
pub unsafe extern "win64" fn FindFirstFileA(pattern: LPCSTR, out: *mut u8) -> HANDLE {
    find_first_common(from_ansi(pattern), out, false)
}
pub unsafe extern "win64" fn FindFirstFileExW(pattern: LPCWSTR, _level: u32, out: *mut u8, search_op: u32, _filter: *mut c_void, _flags: u32) -> HANDLE {
    let h = find_first_common(from_wide(pattern), out, true);
    if search_op == 1 && h != INVALID_HANDLE_VALUE {
        // FindExSearchLimitToDirectories is advisory; callers re-check attrs.
    }
    h
}
pub unsafe extern "win64" fn FindFirstFileExA(pattern: LPCSTR, _level: u32, out: *mut u8, _op: u32, _filter: *mut c_void, _flags: u32) -> HANDLE {
    find_first_common(from_ansi(pattern), out, false)
}

unsafe fn find_next(h: HANDLE, out: *mut u8, wide: bool) -> BOOL {
    let Some(f) = handles::get_as::<FindObj>(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let next = f.entries.lock().unwrap().pop_front();
    match next {
        Some(e) => {
            if wide {
                fill_find_data_w(&e, out);
            } else {
                fill_find_data_a(&e, out);
            }
            TRUE
        }
        None => {
            set_err(err::ERROR_NO_MORE_FILES);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn FindNextFileW(h: HANDLE, out: *mut u8) -> BOOL {
    find_next(h, out, true)
}
pub unsafe extern "win64" fn FindNextFileA(h: HANDLE, out: *mut u8) -> BOOL {
    find_next(h, out, false)
}
pub unsafe extern "win64" fn FindClose(h: HANDLE) -> BOOL {
    if handles::get_as::<FindObj>(h).is_some() && handles::close(h) {
        TRUE
    } else {
        set_err(err::ERROR_INVALID_HANDLE);
        FALSE
    }
}

// ---------------------------------------------------------------------------
// Native Darwin kqueue Directory Watching (ReadDirectoryChangesW &
// FindFirstChangeNotificationW/Next/Close)
// ---------------------------------------------------------------------------

const FILE_NOTIFY_CHANGE_FILE_NAME: u32 = 0x0000_0001;
const FILE_NOTIFY_CHANGE_DIR_NAME: u32 = 0x0000_0002;
const FILE_NOTIFY_CHANGE_ATTRIBUTES: u32 = 0x0000_0004;
const FILE_NOTIFY_CHANGE_SIZE: u32 = 0x0000_0008;
const FILE_NOTIFY_CHANGE_LAST_WRITE: u32 = 0x0000_0010;
const FILE_NOTIFY_CHANGE_LAST_ACCESS: u32 = 0x0000_0020;
const FILE_NOTIFY_CHANGE_CREATION: u32 = 0x0000_0040;
const FILE_NOTIFY_CHANGE_SECURITY: u32 = 0x0000_0100;

const FILE_ACTION_ADDED: u32 = 1;
const FILE_ACTION_REMOVED: u32 = 2;
const FILE_ACTION_MODIFIED: u32 = 3;
const FILE_ACTION_RENAMED_OLD_NAME: u32 = 4;
const FILE_ACTION_RENAMED_NEW_NAME: u32 = 5;

const MAX_WATCHED_VNODE_FDS: usize = 512;
#[cfg(target_vendor = "apple")]
const DIR_WATCH_FD_HEADROOM: usize = 64;
#[cfg(target_vendor = "apple")]
const DIR_WATCH_EMFILE_BACKOFF_FDS: usize = 16;
#[cfg(target_vendor = "apple")]
const DIR_WATCH_RESCAN_INTERVAL: Duration = Duration::from_millis(500);
const MAX_PENDING_DIR_CHANGES: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq)]
struct DirSnapEntry {
    is_dir: bool,
    dev: u64,
    ino: u64,
    size: u64,
    mtime: (i64, i64),
    atime: (i64, i64),
    ctime: (i64, i64),
    birthtime: (i64, i64),
    mode: u32,
}

#[derive(Clone, Debug)]
struct DirChange {
    action: u32,
    rel_path: String,
    filter_mask: u32,
}

fn dir_entry_from_meta(m: &std::fs::Metadata) -> DirSnapEntry {
    DirSnapEntry {
        is_dir: m.is_dir(),
        dev: m.dev(),
        ino: m.ino(),
        size: if m.is_dir() { 0 } else { m.len() },
        mtime: (m.mtime(), m.mtime_nsec()),
        atime: (m.atime(), m.atime_nsec()),
        ctime: (m.ctime(), m.ctime_nsec()),
        birthtime: (m.st_birthtime(), m.st_birthtime_nsec()),
        mode: m.permissions().mode(),
    }
}

fn snapshot_dir_rec(
    root: &Path,
    rel_prefix: &str,
    subtree: bool,
    out: &mut BTreeMap<String, DirSnapEntry>,
) -> Result<(), i32> {
    let dir_path = if rel_prefix.is_empty() {
        root.to_path_buf()
    } else {
        root.join(rel_prefix.replace('\\', "/"))
    };
    let rd = match std::fs::read_dir(&dir_path) {
        Ok(rd) => rd,
        Err(e) => {
            let raw = e.raw_os_error().unwrap_or(libc::EIO);
            if raw == libc::ENOENT || (!rel_prefix.is_empty() && (raw == libc::ENOTDIR || raw == libc::EACCES || raw == libc::EPERM)) {
                return Ok(());
            }
            return Err(raw);
        }
    };
    for entry in rd.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let rel = if rel_prefix.is_empty() {
            name
        } else {
            format!("{rel_prefix}\\{name}")
        };
        let Ok(meta) = std::fs::symlink_metadata(entry.path()) else {
            continue;
        };
        let snap = dir_entry_from_meta(&meta);
        let is_dir = snap.is_dir;
        out.insert(rel.clone(), snap);
        if is_dir && subtree {
            snapshot_dir_rec(root, &rel, true, out)?;
        }
    }
    Ok(())
}

fn snapshot_dir(root: &Path, subtree: bool) -> Result<BTreeMap<String, DirSnapEntry>, i32> {
    let mut map = BTreeMap::new();
    snapshot_dir_rec(root, "", subtree, &mut map)?;
    Ok(map)
}

fn parent_of_rel(rel: &str) -> &str {
    match rel.rfind('\\') {
        Some(i) => &rel[..i],
        None => "",
    }
}

fn diff_dir_snapshots(
    old: &BTreeMap<String, DirSnapEntry>,
    new: &BTreeMap<String, DirSnapEntry>,
    vnode_written: &HashSet<String>,
    vnode_attrib: &HashSet<String>,
) -> Vec<DirChange> {
    let mut removed: Vec<(&String, &DirSnapEntry)> = Vec::new();
    let mut added: Vec<(&String, &DirSnapEntry)> = Vec::new();

    for (k, v) in old {
        if !new.contains_key(k) {
            removed.push((k, v));
        }
    }
    for (k, v) in new {
        if !old.contains_key(k) {
            added.push((k, v));
        }
    }

    let mut out = Vec::new();
    let mut used_removed = vec![false; removed.len()];
    let mut used_added = vec![false; added.len()];

    // Match renames within the same parent directory by (dev, ino, is_dir).
    for (ri, (rpath, rent)) in removed.iter().enumerate() {
        if rent.ino == 0 {
            continue;
        }
        let rparent = parent_of_rel(rpath);
        for (ai, (apath, aent)) in added.iter().enumerate() {
            if used_added[ai] {
                continue;
            }
            if rent.dev == aent.dev
                && rent.ino == aent.ino
                && rent.is_dir == aent.is_dir
                && parent_of_rel(apath) == rparent
            {
                used_removed[ri] = true;
                used_added[ai] = true;
                let mask = if rent.is_dir {
                    FILE_NOTIFY_CHANGE_DIR_NAME
                } else {
                    FILE_NOTIFY_CHANGE_FILE_NAME
                };
                out.push(DirChange {
                    action: FILE_ACTION_RENAMED_OLD_NAME,
                    rel_path: (*rpath).clone(),
                    filter_mask: mask,
                });
                out.push(DirChange {
                    action: FILE_ACTION_RENAMED_NEW_NAME,
                    rel_path: (*apath).clone(),
                    filter_mask: mask,
                });
                break;
            }
        }
    }

    for (ri, (rpath, rent)) in removed.iter().enumerate() {
        if !used_removed[ri] {
            let mask = if rent.is_dir {
                FILE_NOTIFY_CHANGE_DIR_NAME
            } else {
                FILE_NOTIFY_CHANGE_FILE_NAME
            };
            out.push(DirChange {
                action: FILE_ACTION_REMOVED,
                rel_path: (*rpath).clone(),
                filter_mask: mask,
            });
        }
    }

    for (ai, (apath, aent)) in added.iter().enumerate() {
        if !used_added[ai] {
            let mask = if aent.is_dir {
                FILE_NOTIFY_CHANGE_DIR_NAME
            } else {
                FILE_NOTIFY_CHANGE_FILE_NAME
            };
            out.push(DirChange {
                action: FILE_ACTION_ADDED,
                rel_path: (*apath).clone(),
                filter_mask: mask,
            });
        }
    }

    for (path, new_e) in new {
        let Some(old_e) = old.get(path) else {
            continue;
        };
        if old_e.is_dir != new_e.is_dir {
            out.push(DirChange {
                action: FILE_ACTION_REMOVED,
                rel_path: path.clone(),
                filter_mask: if old_e.is_dir { FILE_NOTIFY_CHANGE_DIR_NAME } else { FILE_NOTIFY_CHANGE_FILE_NAME },
            });
            out.push(DirChange {
                action: FILE_ACTION_ADDED,
                rel_path: path.clone(),
                filter_mask: if new_e.is_dir { FILE_NOTIFY_CHANGE_DIR_NAME } else { FILE_NOTIFY_CHANGE_FILE_NAME },
            });
            continue;
        }
        let mut mask = 0u32;
        if !new_e.is_dir {
            let vnode_w = vnode_written.contains(path);
            if old_e.size != new_e.size {
                mask |= FILE_NOTIFY_CHANGE_SIZE | FILE_NOTIFY_CHANGE_LAST_WRITE;
            }
            if old_e.mtime != new_e.mtime || vnode_w || old_e.ino != new_e.ino {
                mask |= FILE_NOTIFY_CHANGE_LAST_WRITE;
                if vnode_w {
                    mask |= FILE_NOTIFY_CHANGE_SIZE;
                }
            }
            if old_e.atime != new_e.atime {
                mask |= FILE_NOTIFY_CHANGE_LAST_ACCESS;
            }
            if old_e.birthtime != new_e.birthtime {
                mask |= FILE_NOTIFY_CHANGE_CREATION;
            }
            if old_e.mode != new_e.mode || vnode_attrib.contains(path) {
                mask |= FILE_NOTIFY_CHANGE_ATTRIBUTES | FILE_NOTIFY_CHANGE_SECURITY;
            }
            if old_e.ctime != new_e.ctime && mask == 0 {
                mask |= FILE_NOTIFY_CHANGE_ATTRIBUTES;
            }
        } else {
            // For directories, only report attribute/security changes (child add/remove
            // are already reported as child ADDED/REMOVED records).
            if old_e.mode != new_e.mode || vnode_attrib.contains(path) {
                mask |= FILE_NOTIFY_CHANGE_ATTRIBUTES | FILE_NOTIFY_CHANGE_SECURITY;
            }
        }
        if mask != 0 {
            out.push(DirChange {
                action: FILE_ACTION_MODIFIED,
                rel_path: path.clone(),
                filter_mask: mask,
            });
        }
    }

    out
}

/// Serialize `changes` into a guest `FILE_NOTIFY_INFORMATION` buffer.
/// Returns `Ok(bytes_written)` or `Err(())` on buffer overflow.
unsafe fn write_notify_buffer(buf: *mut u8, buf_len: usize, changes: &[DirChange]) -> Result<usize, ()> {
    if changes.is_empty() {
        return Ok(0);
    }
    let mut encoded: Vec<(u32, Vec<u16>)> = Vec::with_capacity(changes.len());
    let mut total_needed = 0usize;
    for (idx, c) in changes.iter().enumerate() {
        let w: Vec<u16> = c.rel_path.encode_utf16().collect();
        let name_bytes = w.len() * 2;
        let rec_len = 12 + name_bytes;
        if idx + 1 < changes.len() {
            total_needed += (rec_len + 3) & !3;
        } else {
            total_needed += rec_len;
        }
        encoded.push((c.action, w));
    }
    if total_needed > buf_len {
        return Err(());
    }
    let mut off = 0usize;
    for (idx, (action, w)) in encoded.iter().enumerate() {
        let name_bytes = w.len() * 2;
        let rec_len = 12 + name_bytes;
        let stride = (rec_len + 3) & !3;
        let next_off = if idx + 1 < encoded.len() { stride as u32 } else { 0 };
        let dst = buf.add(off);
        std::ptr::write_unaligned(dst as *mut u32, next_off);
        std::ptr::write_unaligned(dst.add(4) as *mut u32, *action);
        std::ptr::write_unaligned(dst.add(8) as *mut u32, name_bytes as u32);
        for (j, &ch) in w.iter().enumerate() {
            std::ptr::write_unaligned(dst.add(12 + j * 2) as *mut u16, ch);
        }
        if idx + 1 < encoded.len() {
            for pad in rec_len..stride {
                std::ptr::write_unaligned(dst.add(pad), 0u8);
            }
            off += stride;
        } else {
            off += rec_len;
        }
    }
    Ok(off)
}

enum DirWatchDelivery {
    Sync {
        done: Arc<(Mutex<Option<(usize, u32)>>, Condvar)>,
        tid: u32,
    },
    Apc {
        ov: usize,
        cb: crate::thread::IoCompletionRoutine,
        thread: Arc<ThreadObj>,
        tid: u32,
    },
    Overlapped {
        ov: usize,
        tid: u32,
        binding: Arc<IoCompletionBinding>,
    },
}

struct DirWatchRequest {
    handle: usize,
    buf: usize,
    len: usize,
    filter: u32,
    completed: AtomicBool,
    delivery: DirWatchDelivery,
}

impl DirWatchRequest {
    fn tid(&self) -> u32 {
        match &self.delivery {
            DirWatchDelivery::Sync { tid, .. }
            | DirWatchDelivery::Apc { tid, .. }
            | DirWatchDelivery::Overlapped { tid, .. } => *tid,
        }
    }

    fn ov_addr(&self) -> Option<usize> {
        match &self.delivery {
            DirWatchDelivery::Sync { .. } => None,
            DirWatchDelivery::Apc { ov, .. } | DirWatchDelivery::Overlapped { ov, .. } => Some(*ov),
        }
    }

    /// Complete this request strictly once (idempotent via `completed` swap).
    fn finish_once(&self, nt_status: usize, bytes: usize) -> bool {
        if self.completed.swap(true, Ordering::AcqRel) {
            return false;
        }
        match &self.delivery {
            DirWatchDelivery::Sync { done, .. } => {
                let (m, cv) = &**done;
                let mut g = m.lock().unwrap();
                *g = Some((nt_status, bytes as u32));
                cv.notify_all();
            }
            DirWatchDelivery::Apc { ov, cb, thread, .. } => unsafe {
                let ov_ptr = *ov as *mut OVERLAPPED;
                store_ov_internal_high(ov_ptr, bytes);
                store_ov_internal(ov_ptr, nt_status);
                let win_err = ntstatus_to_win32_local(nt_status);
                crate::thread::queue_io_completion_apc(thread, *cb, win_err, bytes as u32, *ov);
            },
            DirWatchDelivery::Overlapped { ov, binding, .. } => unsafe {
                let _ = complete_overlapped_binding(
                    Some(binding),
                    *ov as *mut OVERLAPPED,
                    nt_status,
                    bytes,
                    false,
                );
            },
        }
        true
    }
}

struct DirWatcherState {
    subtree: bool,
    requests: VecDeque<Arc<DirWatchRequest>>,
    pending_changes: VecDeque<DirChange>,
    overflow: bool,
    /// Optional `FindFirstChangeNotification` event + filter.
    notify_event: Option<(Arc<Event>, u32)>,
}

struct DirWatcher {
    state: Mutex<DirWatcherState>,
    closed: AtomicBool,
    wake_w: Mutex<Option<i32>>,
}

impl DirWatcher {
    fn poke(&self) {
        if let Some(fd) = *self.wake_w.lock().unwrap() {
            let b = 1u8;
            unsafe {
                let _ = libc::write(fd, &b as *const u8 as *const c_void, 1);
            }
        }
    }

    fn shutdown(&self) {
        if self.closed.swap(true, Ordering::AcqRel) {
            return;
        }
        let cancelled: Vec<Arc<DirWatchRequest>> = {
            let mut st = self.state.lock().unwrap();
            st.requests.drain(..).collect()
        };
        for req in cancelled {
            req.finish_once(STATUS_CANCELLED, 0);
        }
        self.poke();
    }

    fn cancel_requests(&self, handle: usize, ov: Option<usize>, filter_tid: Option<u32>) -> usize {
        let to_cancel: Vec<Arc<DirWatchRequest>> = {
            let mut st = self.state.lock().unwrap();
            let mut matched = Vec::new();
            let mut i = 0;
            while i < st.requests.len() {
                let r = &st.requests[i];
                let h_ok = r.handle == handle;
                let ov_ok = ov.map_or(true, |target| r.ov_addr() == Some(target));
                let tid_ok = filter_tid.map_or(true, |t| r.tid() == t);
                if h_ok && ov_ok && tid_ok {
                    if let Some(req) = st.requests.remove(i) {
                        matched.push(req);
                    }
                } else {
                    i += 1;
                }
            }
            matched
        };
        let mut count = 0;
        for req in to_cancel {
            if req.finish_once(STATUS_CANCELLED, 0) {
                count += 1;
            }
        }
        if count > 0 {
            self.poke();
        }
        count
    }

    fn start(root: PathBuf, subtree: bool, notify_event: Option<(Arc<Event>, u32)>) -> Result<Arc<Self>, u32> {
        #[cfg(target_vendor = "apple")]
        {
            let kq = unsafe { libc::kqueue() };
            if kq < 0 {
                return Err(errno_to_win32(last_errno()));
            }
            set_cloexec(kq);
            let mut wake = [0i32; 2];
            if unsafe { libc::pipe(wake.as_mut_ptr()) } != 0 {
                let e = last_errno();
                unsafe { libc::close(kq) };
                return Err(errno_to_win32(e));
            }
            for &fd in &wake {
                set_cloexec(fd);
                unsafe {
                    let fl = libc::fcntl(fd, libc::F_GETFL);
                    if fl >= 0 {
                        let _ = libc::fcntl(fd, libc::F_SETFL, fl | libc::O_NONBLOCK);
                    }
                }
            }
            let wake_r = wake[0];
            let wake_w = wake[1];

            // Register wakeup pipe read end in kqueue.
            let wake_ev = libc::kevent {
                ident: wake_r as usize,
                filter: libc::EVFILT_READ,
                flags: libc::EV_ADD | libc::EV_ENABLE | libc::EV_CLEAR,
                fflags: 0,
                data: 0,
                udata: std::ptr::null_mut(),
            };
            if unsafe { libc::kevent(kq, &wake_ev, 1, std::ptr::null_mut(), 0, std::ptr::null()) } < 0 {
                let e = last_errno();
                unsafe {
                    libc::close(kq);
                    libc::close(wake_r);
                    libc::close(wake_w);
                }
                return Err(errno_to_win32(e));
            }

            // Take initial snapshot and attach initial vnode watches SYNCHRONOUSLY before returning
            // so any filesystem change immediately after ReadDirectoryChangesW/FindFirstChangeNotificationW
            // is guaranteed to be observed.
            let (initial_snap, snap_failed) = match snapshot_dir(&root, subtree) {
                Ok(s) => (s, false),
                Err(_) => (BTreeMap::new(), true),
            };
            let mut vnode_budget = effective_vnode_fd_budget(MAX_WATCHED_VNODE_FDS);
            let mut watched_by_path: HashMap<String, (i32, u64, u64)> = HashMap::new();
            let mut path_by_fd: HashMap<i32, String> = HashMap::new();
            let initial_needs_rescan = sync_kqueue_vnodes(
                kq,
                &root,
                &initial_snap,
                &mut watched_by_path,
                &mut path_by_fd,
                &mut vnode_budget,
            ) || snap_failed;

            let watcher = Arc::new(DirWatcher {
                state: Mutex::new(DirWatcherState {
                    subtree,
                    requests: VecDeque::new(),
                    pending_changes: VecDeque::new(),
                    overflow: false,
                    notify_event,
                }),
                closed: AtomicBool::new(false),
                wake_w: Mutex::new(Some(wake_w)),
            });

            let w_thread = Arc::clone(&watcher);
            let initial_fds: Vec<i32> = watched_by_path.values().map(|&(fd, _, _)| fd).collect();
            let spawn_res = std::thread::Builder::new()
                .name("win32-dir-watch".into())
                .spawn(move || {
                    dir_watch_kqueue_loop(
                        w_thread,
                        kq,
                        wake_r,
                        root,
                        initial_snap,
                        watched_by_path,
                        path_by_fd,
                        vnode_budget,
                        initial_needs_rescan,
                    );
                });
            if let Err(e) = spawn_res {
                for fd in initial_fds {
                    unsafe { libc::close(fd) };
                }
                unsafe {
                    libc::close(kq);
                    libc::close(wake_r);
                    libc::close(wake_w);
                }
                *watcher.wake_w.lock().unwrap() = None;
                return Err(io_err(&e));
            }
            Ok(watcher)
        }
        #[cfg(not(target_vendor = "apple"))]
        {
            let _ = (root, subtree, notify_event);
            Err(err::ERROR_NOT_SUPPORTED)
        }
    }
}

impl Drop for DirWatcher {
    fn drop(&mut self) {
        self.shutdown();
        if let Some(fd) = self.wake_w.lock().unwrap().take() {
            unsafe { libc::close(fd) };
        }
    }
}

#[cfg(target_vendor = "apple")]
fn effective_vnode_fd_budget(max_budget: usize) -> usize {
    let mut rl = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut rl) } != 0 || rl.rlim_cur == libc::RLIM_INFINITY {
        return max_budget;
    }
    let soft = usize::try_from(rl.rlim_cur).unwrap_or(usize::MAX);
    // Keep at least half of RLIMIT_NOFILE (and DIR_WATCH_FD_HEADROOM when soft allows) free
    // so snapshot_dir() and other process descriptors are never starved.
    let safe_cap = (soft / 2).min(soft.saturating_sub(DIR_WATCH_FD_HEADROOM).max(1));
    max_budget.min(safe_cap.max(1))
}

#[cfg(target_vendor = "apple")]
fn open_vnode_evtonly(path: &Path) -> Result<i32, i32> {
    let c = std::ffi::CString::new(path.as_os_str().as_encoded_bytes()).map_err(|_| libc::EINVAL)?;
    let fd = unsafe { libc::open(c.as_ptr(), DARWIN_O_EVTONLY | libc::O_CLOEXEC) };
    if fd < 0 {
        Err(last_errno())
    } else {
        Ok(fd)
    }
}

#[cfg(target_vendor = "apple")]
fn register_vnode_kevent(kq: i32, fd: i32) -> bool {
    let ev = libc::kevent {
        ident: fd as usize,
        filter: libc::EVFILT_VNODE,
        flags: libc::EV_ADD | libc::EV_ENABLE | libc::EV_CLEAR,
        fflags: libc::NOTE_WRITE
            | libc::NOTE_DELETE
            | libc::NOTE_EXTEND
            | libc::NOTE_ATTRIB
            | libc::NOTE_LINK
            | libc::NOTE_RENAME
            | libc::NOTE_REVOKE,
        data: 0,
        udata: std::ptr::null_mut(),
    };
    unsafe { libc::kevent(kq, &ev, 1, std::ptr::null_mut(), 0, std::ptr::null()) >= 0 }
}

/// Synchronize kqueue `EVFILT_VNODE` descriptors against `root` and `snap`, prioritizing
/// directories before files within `*vnode_budget` (bounded by `MAX_WATCHED_VNODE_FDS` and
/// process `RLIMIT_NOFILE` headroom). On `EMFILE`/`ENFILE`, releases a small slice of
/// non-root file descriptors and reduces `*vnode_budget` persistently to prevent FD starvation
/// or open/close churn. Returns `true` when root or any snapshotted entry lacks an active
/// registered vnode fd, signaling that periodic rescan is needed.
#[cfg(target_vendor = "apple")]
fn sync_kqueue_vnodes(
    kq: i32,
    root: &Path,
    snap: &BTreeMap<String, DirSnapEntry>,
    watched_by_path: &mut HashMap<String, (i32, u64, u64)>,
    path_by_fd: &mut HashMap<i32, String>,
    vnode_budget: &mut usize,
) -> bool {
    let limit = (*vnode_budget).min(effective_vnode_fd_budget(MAX_WATCHED_VNODE_FDS));
    let mut desired: Vec<(String, u64, u64)> = Vec::new();
    if let Ok(rm) = std::fs::symlink_metadata(root) {
        desired.push((String::new(), rm.dev(), rm.ino()));
    }
    // Prioritize directories first so subtree watching covers subdirectories within the
    // bounded fd budget, then add files up to `limit`.
    for (rel, ent) in snap {
        if ent.is_dir {
            if desired.len() >= limit {
                break;
            }
            desired.push((rel.clone(), ent.dev, ent.ino));
        }
    }
    for (rel, ent) in snap {
        if !ent.is_dir {
            if desired.len() >= limit {
                break;
            }
            desired.push((rel.clone(), ent.dev, ent.ino));
        }
    }
    let desired_map: HashMap<String, (u64, u64)> = desired.iter().map(|(p, d, i)| (p.clone(), (*d, *i))).collect();

    // Close removed or replaced (different inode) vnode fds.
    let stale: Vec<String> = watched_by_path
        .iter()
        .filter_map(|(p, &(_fd, dev, ino))| match desired_map.get(p) {
            Some(&(d, i)) if d == dev && i == ino => None,
            _ => Some(p.clone()),
        })
        .collect();
    for p in stale {
        if let Some((fd, _, _)) = watched_by_path.remove(&p) {
            path_by_fd.remove(&fd);
            unsafe { libc::close(fd) };
        }
    }

    // Open and register new vnode fds.
    let mut hit_fd_limit = false;
    for (rel, dev, ino) in desired {
        if watched_by_path.contains_key(&rel) {
            continue;
        }
        let full = if rel.is_empty() {
            root.to_path_buf()
        } else {
            root.join(rel.replace('\\', "/"))
        };
        match open_vnode_evtonly(&full) {
            Ok(fd) => {
                if register_vnode_kevent(kq, fd) {
                    watched_by_path.insert(rel.clone(), (fd, dev, ino));
                    path_by_fd.insert(fd, rel);
                } else {
                    unsafe { libc::close(fd) };
                }
            }
            Err(libc::EMFILE | libc::ENFILE) => {
                hit_fd_limit = true;
                break;
            }
            Err(_) => {}
        }
    }

    if hit_fd_limit {
        // Release a small slice of non-root file descriptors first (preserving root & dirs)
        // and clamp this watcher's budget so subsequent rescans do not thrash descriptors.
        let mut to_release: Vec<String> = watched_by_path
            .keys()
            .filter(|k| !k.is_empty() && !snap.get(*k).is_some_and(|e| e.is_dir))
            .cloned()
            .collect();
        to_release.sort();
        for p in to_release.into_iter().rev().take(DIR_WATCH_EMFILE_BACKOFF_FDS) {
            if let Some((fd, _, _)) = watched_by_path.remove(&p) {
                path_by_fd.remove(&fd);
                unsafe { libc::close(fd) };
            }
        }
        *vnode_budget = watched_by_path.len().max(1);
    }

    // `watched_by_path` keys are a subset of `{""} U snap.keys()`; full coverage holds iff
    // root (`""`) and every snapshotted entry have an active registered descriptor.
    watched_by_path.len() < snap.len() + 1 || !watched_by_path.contains_key("")
}

#[cfg(target_vendor = "apple")]
#[allow(clippy::too_many_arguments)]
fn dir_watch_kqueue_loop(
    watcher: Arc<DirWatcher>,
    kq: i32,
    wake_r: i32,
    root: PathBuf,
    mut prev_snap: BTreeMap<String, DirSnapEntry>,
    mut watched_by_path: HashMap<String, (i32, u64, u64)>,
    mut path_by_fd: HashMap<i32, String>,
    mut vnode_budget: usize,
    mut needs_rescan: bool,
) {
    let mut cur_subtree = watcher.state.lock().unwrap().subtree;
    let mut events = [unsafe { std::mem::zeroed::<libc::kevent>() }; 64];
    let mut next_rescan: Option<Instant> =
        needs_rescan.then(|| Instant::now() + DIR_WATCH_RESCAN_INTERVAL);

    loop {
        if watcher.closed.load(Ordering::Acquire) {
            break;
        }
        let mut ts_storage = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        let ts_ptr = if let Some(dl) = next_rescan {
            let rem = dl.saturating_duration_since(Instant::now());
            ts_storage.tv_sec = rem.as_secs() as libc::time_t;
            ts_storage.tv_nsec = rem.subsec_nanos() as libc::c_long;
            &ts_storage as *const libc::timespec
        } else {
            std::ptr::null()
        };
        let n = unsafe {
            libc::kevent(
                kq,
                std::ptr::null(),
                0,
                events.as_mut_ptr(),
                events.len() as i32,
                ts_ptr,
            )
        };
        if n < 0 {
            if last_errno() == libc::EINTR {
                continue;
            }
            break;
        }
        if watcher.closed.load(Ordering::Acquire) {
            break;
        }

        let mut woke_by_pipe = false;
        let mut saw_vnode = false;
        let mut vnode_written: HashSet<String> = HashSet::new();
        let mut vnode_attrib: HashSet<String> = HashSet::new();

        let mut record_events = |evs: &[libc::kevent]| {
            for ev in evs {
                if ev.filter == libc::EVFILT_READ && ev.ident == wake_r as usize {
                    woke_by_pipe = true;
                } else if ev.filter == libc::EVFILT_VNODE {
                    saw_vnode = true;
                    let fd = ev.ident as i32;
                    if let Some(rel) = path_by_fd.get(&fd) {
                        if !rel.is_empty() {
                            if (ev.fflags & (libc::NOTE_WRITE | libc::NOTE_EXTEND)) != 0 {
                                vnode_written.insert(rel.clone());
                            }
                            if (ev.fflags & (libc::NOTE_ATTRIB | libc::NOTE_LINK)) != 0 {
                                vnode_attrib.insert(rel.clone());
                            }
                        }
                    }
                }
            }
        };

        if n > 0 {
            record_events(&events[..n as usize]);

            // Drain any immediately queued events (e.g. rename firing both NOTE_RENAME on file and NOTE_WRITE on dir).
            let zero_ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
            for _ in 0..8 {
                let more = unsafe { libc::kevent(kq, std::ptr::null(), 0, events.as_mut_ptr(), events.len() as i32, &zero_ts) };
                if more <= 0 {
                    break;
                }
                record_events(&events[..more as usize]);
            }
        }

        if woke_by_pipe {
            let mut drain = [0u8; 64];
            while unsafe { libc::read(wake_r, drain.as_mut_ptr() as *mut c_void, drain.len()) } > 0 {}
        }

        if watcher.closed.load(Ordering::Acquire) {
            break;
        }

        // Handle subtree promotion if a subsequent ReadDirectoryChangesW enabled subtree watching.
        let want_subtree = watcher.state.lock().unwrap().subtree;
        if want_subtree && !cur_subtree {
            cur_subtree = true;
            if let Ok(snap) = snapshot_dir(&root, true) {
                prev_snap = snap;
                needs_rescan = sync_kqueue_vnodes(
                    kq,
                    &root,
                    &prev_snap,
                    &mut watched_by_path,
                    &mut path_by_fd,
                    &mut vnode_budget,
                );
            } else {
                needs_rescan = true;
            }
            next_rescan = if needs_rescan {
                Some(next_rescan.unwrap_or_else(|| Instant::now() + DIR_WATCH_RESCAN_INTERVAL))
            } else {
                None
            };
        }

        let rescan_due = next_rescan.is_some_and(|dl| Instant::now() >= dl);
        if saw_vnode || rescan_due {
            let new_snap = match snapshot_dir(&root, cur_subtree) {
                Ok(snap) => snap,
                Err(e) => {
                    if e == libc::EMFILE || e == libc::ENFILE {
                        vnode_budget = watched_by_path
                            .len()
                            .saturating_sub(DIR_WATCH_EMFILE_BACKOFF_FDS)
                            .max(1);
                        let _ = sync_kqueue_vnodes(
                            kq,
                            &root,
                            &prev_snap,
                            &mut watched_by_path,
                            &mut path_by_fd,
                            &mut vnode_budget,
                        );
                    }
                    next_rescan = Some(Instant::now() + DIR_WATCH_RESCAN_INTERVAL);
                    continue;
                }
            };
            let changes = diff_dir_snapshots(&prev_snap, &new_snap, &vnode_written, &vnode_attrib);
            prev_snap = new_snap;
            // Register new vnodes BEFORE completing any request so subsequent watches never miss mutations.
            needs_rescan = sync_kqueue_vnodes(
                kq,
                &root,
                &prev_snap,
                &mut watched_by_path,
                &mut path_by_fd,
                &mut vnode_budget,
            );
            next_rescan = needs_rescan.then(|| Instant::now() + DIR_WATCH_RESCAN_INTERVAL);

            if !changes.is_empty() {
                let mut to_complete: Vec<(Arc<DirWatchRequest>, usize, usize)> = Vec::new();
                let mut signal_event: Option<Arc<Event>> = None;
                {
                    let mut st = watcher.state.lock().unwrap();
                    if let Some((ref ev, nfilter)) = st.notify_event {
                        if changes.iter().any(|c| (c.filter_mask & nfilter) != 0) {
                            signal_event = Some(Arc::clone(ev));
                        }
                    }
                    // Queue changes and attempt to satisfy the front waiting request.
                    for c in changes {
                        if st.pending_changes.len() < MAX_PENDING_DIR_CHANGES {
                            st.pending_changes.push_back(c);
                        } else {
                            st.overflow = true;
                        }
                    }
                    while let Some(req) = st.requests.front().cloned() {
                        if req.completed.load(Ordering::Acquire) {
                            st.requests.pop_front();
                            continue;
                        }
                        if st.overflow {
                            st.overflow = false;
                            st.pending_changes.clear();
                            st.requests.pop_front();
                            to_complete.push((req, STATUS_NOTIFY_ENUM_DIR, 0));
                            continue;
                        }
                        let matching: Vec<DirChange> = st
                            .pending_changes
                            .iter()
                            .filter(|c| (c.filter_mask & req.filter) != 0)
                            .cloned()
                            .collect();
                        if matching.is_empty() {
                            // Discard non-matching changes only if no other request needs them.
                            if st.requests.len() == 1 {
                                st.pending_changes.clear();
                            }
                            break;
                        }
                        st.pending_changes.clear();
                        st.requests.pop_front();
                        let res = unsafe { write_notify_buffer(req.buf as *mut u8, req.len, &matching) };
                        match res {
                            Ok(bytes) => to_complete.push((req, STATUS_SUCCESS, bytes)),
                            Err(()) => to_complete.push((req, STATUS_NOTIFY_ENUM_DIR, 0)),
                        }
                    }
                }
                if let Some(ev) = signal_event {
                    ev.set();
                }
                for (req, status, bytes) in to_complete {
                    req.finish_once(status, bytes);
                }
            }
        }
    }

    // Cleanup all fds on exit.
    for (_, (fd, _, _)) in watched_by_path {
        unsafe { libc::close(fd) };
    }
    unsafe {
        libc::close(kq);
        libc::close(wake_r);
    }
}

pub unsafe extern "win64" fn ReadDirectoryChangesW(
    h: HANDLE,
    buf: *mut c_void,
    len: u32,
    subtree: BOOL,
    filter: u32,
    ret: *mut u32,
    ov: *mut OVERLAPPED,
    cb: *const c_void,
) -> BOOL {
    if !ret.is_null() {
        std::ptr::write_unaligned(ret, 0);
    }
    if buf.is_null() || len < 12 || filter == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if f.kind != FileKind::Dir {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(root) = f.host_path.clone() else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };

    let watcher = {
        let mut wg = f.dir_watcher.lock().unwrap();
        if let Some(existing) = wg.as_ref() {
            let w = Arc::clone(existing);
            if subtree != 0 {
                let mut st = w.state.lock().unwrap();
                if !st.subtree {
                    st.subtree = true;
                    drop(st);
                    w.poke();
                }
            }
            w
        } else {
            match DirWatcher::start(root, subtree != 0, None) {
                Ok(w) => {
                    *wg = Some(Arc::clone(&w));
                    w
                }
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            }
        }
    };

    let tid = teb::thread_id();
    // Atomically check buffered changes and, if none match, enqueue the request under a single
    // `watcher.state` lock acquisition so kqueue events arriving concurrently cannot be stranded
    // in `pending_changes`. Prepare pending OVERLAPPED state BEFORE enqueuing so the watcher
    // thread can safely complete immediately after we drop the lock, and release the lock BEFORE
    // delivering any completion notification.
    let consume_buffered = |st: &mut DirWatcherState| -> Option<(usize, usize)> {
        while st.requests.front().is_some_and(|r| r.completed.load(Ordering::Acquire)) {
            st.requests.pop_front();
        }
        if !st.requests.is_empty() {
            return None;
        }
        if st.overflow {
            st.overflow = false;
            st.pending_changes.clear();
            Some((STATUS_NOTIFY_ENUM_DIR, 0))
        } else if !st.pending_changes.is_empty() {
            let matching: Vec<DirChange> = st
                .pending_changes
                .drain(..)
                .filter(|c| (c.filter_mask & filter) != 0)
                .collect();
            if matching.is_empty() {
                None
            } else {
                match write_notify_buffer(buf as *mut u8, len as usize, &matching) {
                    Ok(bytes) => Some((STATUS_SUCCESS, bytes)),
                    Err(()) => Some((STATUS_NOTIFY_ENUM_DIR, 0)),
                }
            }
        } else {
            None
        }
    };

    if ov.is_null() {
        let done = Arc::new((Mutex::new(None), Condvar::new()));
        let req = Arc::new(DirWatchRequest {
            handle: h as usize,
            buf: buf as usize,
            len: len as usize,
            filter,
            completed: AtomicBool::new(false),
            delivery: DirWatchDelivery::Sync { done: Arc::clone(&done), tid },
        });
        let immediate = {
            let mut st = watcher.state.lock().unwrap();
            if let Some(res) = consume_buffered(&mut st) {
                Some(res)
            } else {
                st.requests.push_back(req);
                None
            }
        };
        if let Some((status, bytes)) = immediate {
            if !ret.is_null() {
                std::ptr::write_unaligned(ret, bytes as u32);
            }
            if status == STATUS_SUCCESS {
                set_err(0);
                return TRUE;
            }
            set_err(ntstatus_to_win32_local(status));
            return FALSE;
        }
        // Drop `Arc<FileObj>` before blocking so `CloseHandle(h)` can drop & cancel cleanly.
        drop(f);
        let (m, cv) = &*done;
        let mut g = m.lock().unwrap();
        let (status, bytes) = loop {
            if let Some(res) = *g {
                break res;
            }
            g = cv.wait(g).unwrap();
        };
        if !ret.is_null() {
            std::ptr::write_unaligned(ret, bytes);
        }
        if status == STATUS_SUCCESS {
            set_err(0);
            TRUE
        } else {
            set_err(ntstatus_to_win32_local(status));
            FALSE
        }
    } else if !cb.is_null() {
        let caller = crate::thread::ensure_current_thread_object();
        let cb_fn: crate::thread::IoCompletionRoutine = std::mem::transmute(cb);
        // APC delivery mode: do NOT touch `ov.h_event` or post to IOCP/TpIo.
        // Publish STATUS_PENDING before `req` can be observed by the watcher thread.
        store_ov_internal_high(ov, 0);
        store_ov_internal(ov, STATUS_PENDING);
        let req = Arc::new(DirWatchRequest {
            handle: h as usize,
            buf: buf as usize,
            len: len as usize,
            filter,
            completed: AtomicBool::new(false),
            delivery: DirWatchDelivery::Apc {
                ov: ov as usize,
                cb: cb_fn,
                thread: Arc::clone(&caller),
                tid,
            },
        });
        let immediate = {
            let mut st = watcher.state.lock().unwrap();
            if let Some(res) = consume_buffered(&mut st) {
                Some(res)
            } else {
                st.requests.push_back(req);
                None
            }
        };
        if let Some((status, bytes)) = immediate {
            store_ov_internal_high(ov, bytes);
            store_ov_internal(ov, status);
            crate::thread::queue_io_completion_apc(&caller, cb_fn, ntstatus_to_win32_local(status), bytes as u32, ov as usize);
        }
        set_err(0);
        TRUE
    } else {
        // Publish pending state before `req` can be observed by the watcher thread.
        f.completion.io_event.reset();
        prepare_overlapped_pending(ov);
        let req = Arc::new(DirWatchRequest {
            handle: h as usize,
            buf: buf as usize,
            len: len as usize,
            filter,
            completed: AtomicBool::new(false),
            delivery: DirWatchDelivery::Overlapped {
                ov: ov as usize,
                tid,
                binding: Arc::clone(&f.completion),
            },
        });
        let immediate = {
            let mut st = watcher.state.lock().unwrap();
            if let Some(res) = consume_buffered(&mut st) {
                Some(res)
            } else {
                st.requests.push_back(req);
                None
            }
        };
        if let Some((status, bytes)) = immediate {
            let _ = complete_overlapped_binding(Some(&f.completion), ov, status, bytes, false);
        }
        // Windows `ReadDirectoryChangesW` returns non-zero (TRUE) for async overlapped start!
        set_err(0);
        TRUE
    }
}

struct ChangeNotifObj {
    event: Arc<Event>,
    watcher: Arc<DirWatcher>,
}

impl Drop for ChangeNotifObj {
    fn drop(&mut self) {
        self.watcher.shutdown();
    }
}

impl KObject for ChangeNotifObj {
    fn type_name(&self) -> &'static str {
        "ChangeNotification"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn waitable(&self) -> Option<&dyn handles::Waitable> {
        Some(&*self.event)
    }
}

pub unsafe extern "win64" fn FindFirstChangeNotificationW(p: LPCWSTR, sub: BOOL, filter: u32) -> HANDLE {
    if p.is_null() || filter == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return INVALID_HANDLE_VALUE;
    }
    let win = path::absolute(&from_wide(p));
    let host = match host_path(&win) {
        Ok(h) => h,
        Err(e) => {
            set_err(e);
            return INVALID_HANDLE_VALUE;
        }
    };
    if !host.is_dir() {
        set_err(not_found_err(&host));
        return INVALID_HANDLE_VALUE;
    }
    let ev = Arc::new(Event::new(true, false, None));
    let watcher = match DirWatcher::start(host, sub != 0, Some((Arc::clone(&ev), filter))) {
        Ok(w) => w,
        Err(e) => {
            set_err(e);
            return INVALID_HANDLE_VALUE;
        }
    };
    handles::insert(Arc::new(ChangeNotifObj { event: ev, watcher }))
}

pub unsafe extern "win64" fn FindFirstChangeNotificationA(p: LPCSTR, sub: BOOL, f: u32) -> HANDLE {
    if p.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return INVALID_HANDLE_VALUE;
    }
    let w = to_wide_z(&from_ansi(p));
    FindFirstChangeNotificationW(w.as_ptr(), sub, f)
}

pub unsafe extern "win64" fn FindNextChangeNotification(h: HANDLE) -> BOOL {
    let Some(obj) = handles::get_as::<ChangeNotifObj>(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    obj.watcher.state.lock().unwrap().pending_changes.clear();
    obj.event.reset();
    TRUE
}

pub unsafe extern "win64" fn FindCloseChangeNotification(h: HANDLE) -> BOOL {
    if handles::get_as::<ChangeNotifObj>(h).is_some() && handles::close(h) {
        TRUE
    } else {
        set_err(err::ERROR_INVALID_HANDLE);
        FALSE
    }
}

// ---------------------------------------------------------------------------
// Pipes (Anonymous & Named Pipes + Event-Driven Async Overlapped Pipe Reactor)
// ---------------------------------------------------------------------------

enum PipeOpKind {
    Read,
    Write,
    /// Non-polled FIFO placeholder held by an active or waiting synchronous `WriteFile`
    /// so neither earlier async writes are bypassed nor later async writes leapfrog it.
    SyncWriteBarrier,
}

enum PipeDelivery {
    Overlapped {
        ov: usize,
        tid: u32,
    },
    Apc {
        ov: usize,
        cb: Option<crate::thread::IoCompletionRoutine>,
        thread: Arc<ThreadObj>,
        tid: u32,
    },
    SyncBarrier,
}

enum AsyncPipeWriteOutcome {
    Completed(usize),
    Pending,
    Error(u32),
}

struct SyncPipeWriteGuard {
    reactor: &'static PipeIoReactor,
    op: Arc<PendingPipeOp>,
}

impl Drop for SyncPipeWriteGuard {
    fn drop(&mut self) {
        self.op.done.store(true, Ordering::Release);
        {
            let mut st = self.reactor.state.lock().unwrap();
            st.ops.retain(|o| !Arc::ptr_eq(o, &self.op));
            self.reactor.gen_cv.notify_all();
        }
        self.reactor.poke();
    }
}

/// True when `op` targets the same logical pipe write stream as `(file, conn)`:
/// - Named-pipe server connections match by shared `Arc<PipeConn>` (so duplicated handles share
///   one write queue while distinct reconnect generations remain isolated).
/// - Direct pipes (anonymous pipes and named-pipe clients) match by shared `Arc<FileObj>`.
fn same_pipe_write_stream(
    op: &PendingPipeOp,
    file: &Arc<FileObj>,
    conn: Option<&Arc<PipeConn>>,
) -> bool {
    match (op.conn.as_ref(), conn) {
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        (None, None) => Arc::ptr_eq(&op.file, file),
        _ => false,
    }
}

fn has_prior_pipe_write(
    ops: &[Arc<PendingPipeOp>],
    file: &Arc<FileObj>,
    conn: Option<&Arc<PipeConn>>,
) -> bool {
    ops.iter().any(|op| {
        matches!(op.kind, PipeOpKind::Write | PipeOpKind::SyncWriteBarrier)
            && !op.done.load(Ordering::Acquire)
            && same_pipe_write_stream(op, file, conn)
    })
}

struct PipeOpState {
    completed: bool,
    written: usize,
}

/// Stable shared connection identity and fd owner for a single named-pipe server session.
struct PipeConn {
    file: File,
    disconnected: AtomicBool,
}

impl PipeConn {
    fn new(file: File) -> Arc<Self> {
        Arc::new(Self {
            file,
            disconnected: AtomicBool::new(false),
        })
    }

    fn fd(&self) -> i32 {
        self.file.as_raw_fd()
    }

    fn is_disconnected(&self) -> bool {
        self.disconnected.load(Ordering::Acquire)
    }

    fn mark_disconnected(&self) {
        if !self.disconnected.swap(true, Ordering::AcqRel) {
            unsafe {
                let _ = libc::shutdown(self.file.as_raw_fd(), libc::SHUT_RDWR);
            }
        }
    }
}

enum PipeIoTarget {
    Direct(i32),
    ServerConn(Arc<PipeConn>),
}

impl PipeIoTarget {
    fn fd(&self) -> i32 {
        match self {
            PipeIoTarget::Direct(fd) => *fd,
            PipeIoTarget::ServerConn(c) => c.fd(),
        }
    }

    fn conn(&self) -> Option<Arc<PipeConn>> {
        match self {
            PipeIoTarget::Direct(_) => None,
            PipeIoTarget::ServerConn(c) => Some(Arc::clone(c)),
        }
    }
}

struct PendingPipeOp {
    handle: usize,
    file: Arc<FileObj>,
    /// Shared named-pipe server connection when operating on a server pipe handle.
    /// Retains the connection's `File` lifetime across in-flight reactor steps and lets
    /// `DisconnectNamedPipe` cancel every op on the connection across duplicated handles.
    conn: Option<Arc<PipeConn>>,
    fd: i32,
    kind: PipeOpKind,
    buf: usize,
    len: usize,
    /// Serializes reactor guest-buffer I/O with cancellation (`CancelIo`/`CancelIoEx`/`CloseHandle`/`DisconnectNamedPipe`)
    /// so completion is published only after no reactor step can touch guest memory again.
    state: Mutex<PipeOpState>,
    /// Lock-free mirror of `state.completed` for `reactor.ops.retain` without nesting locks.
    done: AtomicBool,
    delivery: PipeDelivery,
}

impl PendingPipeOp {
    fn new_read(
        handle: HANDLE,
        file: Arc<FileObj>,
        conn: Option<Arc<PipeConn>>,
        fd: i32,
        buf: usize,
        len: usize,
        delivery: PipeDelivery,
    ) -> Arc<Self> {
        Arc::new(PendingPipeOp {
            handle: handle as usize,
            file,
            conn,
            fd,
            kind: PipeOpKind::Read,
            buf,
            len,
            state: Mutex::new(PipeOpState { completed: false, written: 0 }),
            done: AtomicBool::new(false),
            delivery,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn new_write_at(
        handle: HANDLE,
        file: Arc<FileObj>,
        conn: Option<Arc<PipeConn>>,
        fd: i32,
        buf: usize,
        len: usize,
        written: usize,
        delivery: PipeDelivery,
    ) -> Arc<Self> {
        Arc::new(PendingPipeOp {
            handle: handle as usize,
            file,
            conn,
            fd,
            kind: PipeOpKind::Write,
            buf,
            len,
            state: Mutex::new(PipeOpState { completed: false, written }),
            done: AtomicBool::new(false),
            delivery,
        })
    }

    fn new_sync_write_barrier(
        handle: HANDLE,
        file: Arc<FileObj>,
        conn: Option<Arc<PipeConn>>,
        fd: i32,
    ) -> Arc<Self> {
        Arc::new(PendingPipeOp {
            handle: handle as usize,
            file,
            conn,
            fd,
            kind: PipeOpKind::SyncWriteBarrier,
            buf: 0,
            len: 0,
            state: Mutex::new(PipeOpState { completed: false, written: 0 }),
            done: AtomicBool::new(false),
            delivery: PipeDelivery::SyncBarrier,
        })
    }

    fn tid(&self) -> u32 {
        match &self.delivery {
            PipeDelivery::Overlapped { tid, .. } | PipeDelivery::Apc { tid, .. } => *tid,
            PipeDelivery::SyncBarrier => 0,
        }
    }

    fn ov_addr(&self) -> usize {
        match &self.delivery {
            PipeDelivery::Overlapped { ov, .. } | PipeDelivery::Apc { ov, .. } => *ov,
            PipeDelivery::SyncBarrier => 0,
        }
    }

    /// Publish terminal status and notification. Must only be called by the single thread
    /// that transitioned `self.state.completed` from `false` to `true`, AFTER releasing `self.state`.
    fn publish_completion(&self, nt_status: usize, bytes: usize) {
        match &self.delivery {
            PipeDelivery::Overlapped { ov, .. } => unsafe {
                let _ = complete_overlapped_binding(
                    Some(&self.file.completion),
                    *ov as *mut OVERLAPPED,
                    nt_status,
                    bytes,
                    false,
                );
            },
            PipeDelivery::Apc { ov, cb, thread, .. } => unsafe {
                let ov_ptr = *ov as *mut OVERLAPPED;
                store_ov_internal_high(ov_ptr, bytes);
                store_ov_internal(ov_ptr, nt_status);
                if let Some(func) = *cb {
                    let win_err = ntstatus_to_win32_local(nt_status);
                    crate::thread::queue_io_completion_apc(thread, func, win_err, bytes as u32, *ov);
                }
            },
            PipeDelivery::SyncBarrier => {}
        }
    }
}

struct PipeReactorState {
    ops: Vec<Arc<PendingPipeOp>>,
    poll_gen: u64,
}

struct PipeIoReactor {
    state: Mutex<PipeReactorState>,
    gen_cv: Condvar,
    wake_w: i32,
}

impl PipeIoReactor {
    fn poke(&self) {
        let b = 1u8;
        unsafe {
            let _ = libc::write(self.wake_w, &b as *const u8 as *const c_void, 1);
        }
    }

    /// Atomically admit an overlapped `WriteFile` or `WriteFileEx` on a pipe under `self.state`:
    /// - If an earlier write on the same pipe/connection is still queued or active, enqueue
    ///   strictly behind it without touching the fd.
    /// - Otherwise attempt a nonblocking write loop while holding `self.state` so no concurrent
    ///   writer can interleave bytes; if `EAGAIN` occurs, enqueue the remainder in FIFO order.
    /// All completion notifications happen strictly outside `self.state` in the caller.
    #[allow(clippy::too_many_arguments)]
    unsafe fn admit_async_write(
        &self,
        handle: HANDLE,
        file: &Arc<FileObj>,
        conn: Option<Arc<PipeConn>>,
        fd: i32,
        buf: *const u8,
        total: usize,
        ov: *mut OVERLAPPED,
        delivery: PipeDelivery,
    ) -> AsyncPipeWriteOutcome {
        let mut st = self.state.lock().unwrap();
        if conn.as_ref().is_some_and(|c| c.is_disconnected()) {
            return AsyncPipeWriteOutcome::Error(err::ERROR_BROKEN_PIPE);
        }
        let mut written_so_far = 0usize;
        if !has_prior_pipe_write(&st.ops, file, conn.as_ref()) {
            while written_so_far < total {
                match try_nonblocking_pipe_write(fd, buf.add(written_so_far), total - written_so_far) {
                    Ok(Some(0)) => {
                        return AsyncPipeWriteOutcome::Error(err::ERROR_BROKEN_PIPE);
                    }
                    Ok(Some(k)) => {
                        written_so_far += k;
                    }
                    Ok(None) => break,
                    Err(e) => {
                        return AsyncPipeWriteOutcome::Error(if e == libc::EPIPE || e == libc::ECONNRESET {
                            err::ERROR_BROKEN_PIPE
                        } else {
                            errno_to_win32(e)
                        });
                    }
                }
            }
            if written_so_far >= total {
                return AsyncPipeWriteOutcome::Completed(written_so_far);
            }
        }
        match &delivery {
            PipeDelivery::Overlapped { .. } => {
                file.completion.io_event.reset();
                prepare_overlapped_pending(ov);
            }
            PipeDelivery::Apc { .. } => {
                store_ov_internal_high(ov, 0);
                store_ov_internal(ov, STATUS_PENDING);
            }
            PipeDelivery::SyncBarrier => {}
        }
        st.ops.push(PendingPipeOp::new_write_at(
            handle,
            Arc::clone(file),
            conn,
            fd,
            buf as usize,
            total,
            written_so_far,
            delivery,
        ));
        drop(st);
        self.poke();
        AsyncPipeWriteOutcome::Pending
    }

    /// Enqueue a non-polled FIFO write barrier for a synchronous `WriteFile` and wait (releasing
    /// `self.state` while waiting and before any blocking `write`/`poll`) until all earlier writes
    /// on the same pipe/connection finish or are cancelled.
    fn acquire_sync_write_turn(
        &'static self,
        handle: HANDLE,
        file: &Arc<FileObj>,
        conn: Option<Arc<PipeConn>>,
        fd: i32,
    ) -> Result<SyncPipeWriteGuard, u32> {
        let barrier = PendingPipeOp::new_sync_write_barrier(handle, Arc::clone(file), conn.clone(), fd);
        let mut st = self.state.lock().unwrap();
        if conn.as_ref().is_some_and(|c| c.is_disconnected()) {
            return Err(err::ERROR_BROKEN_PIPE);
        }
        st.ops.push(Arc::clone(&barrier));
        let guard = SyncPipeWriteGuard { reactor: self, op: Arc::clone(&barrier) };
        loop {
            if conn.as_ref().is_some_and(|c| c.is_disconnected()) {
                drop(st);
                return Err(err::ERROR_BROKEN_PIPE);
            }
            let prior = match st.ops.iter().position(|o| Arc::ptr_eq(o, &barrier)) {
                Some(idx) => has_prior_pipe_write(&st.ops[..idx], file, conn.as_ref()),
                None => false,
            };
            if !prior {
                break;
            }
            st = self.gen_cv.wait(st).unwrap();
        }
        drop(st);
        Ok(guard)
    }

    fn submit(&self, op: Arc<PendingPipeOp>) {
        let already_disconnected = {
            let mut st = self.state.lock().unwrap();
            if op.conn.as_ref().is_some_and(|c| c.is_disconnected()) {
                true
            } else {
                st.ops.push(Arc::clone(&op));
                false
            }
        };
        if already_disconnected {
            let should_publish = {
                let mut ost = op.state.lock().unwrap();
                if ost.completed {
                    false
                } else {
                    ost.completed = true;
                    op.done.store(true, Ordering::Release);
                    true
                }
            };
            if should_publish {
                op.publish_completion(STATUS_CANCELLED, 0);
            }
        } else {
            self.poke();
        }
    }

    /// Two-phase synchronized cancellation:
    /// 1. Keep `to_cancel` in `self.state.ops` while acquiring each `op.state` so any in-flight
    ///    reactor step on `op` finishes before a successor write can advance.
    /// 2. Remove the now-completed ops from `self.state.ops` (preserving FIFO order) and notify
    ///    `gen_cv` for any waiting synchronous writers.
    /// 3. Publish `STATUS_CANCELLED` strictly outside both `self.state` and `op.state`.
    fn finalize_cancelled_ops(&self, to_cancel: Vec<Arc<PendingPipeOp>>) -> usize {
        if to_cancel.is_empty() {
            return 0;
        }
        let mut publish = Vec::with_capacity(to_cancel.len());
        for op in &to_cancel {
            let should_publish = {
                let mut ost = op.state.lock().unwrap();
                if ost.completed {
                    false
                } else {
                    ost.completed = true;
                    op.done.store(true, Ordering::Release);
                    true
                }
            };
            if should_publish {
                publish.push(Arc::clone(op));
            }
        }
        {
            let mut st = self.state.lock().unwrap();
            st.ops.retain(|o| !to_cancel.iter().any(|c| Arc::ptr_eq(o, c)));
            self.gen_cv.notify_all();
        }
        let count = publish.len();
        for op in publish {
            op.publish_completion(STATUS_CANCELLED, 0);
        }
        count
    }

    fn cancel_ops(&self, handle: usize, ov: Option<usize>, filter_tid: Option<u32>) -> usize {
        let to_cancel: Vec<Arc<PendingPipeOp>> = {
            let st = self.state.lock().unwrap();
            st.ops
                .iter()
                .filter(|o| {
                    !matches!(o.kind, PipeOpKind::SyncWriteBarrier)
                        && o.handle == handle
                        && ov.map_or(true, |target| o.ov_addr() == target)
                        && filter_tid.map_or(true, |t| o.tid() == t)
                })
                .cloned()
                .collect()
        };
        let count = self.finalize_cancelled_ops(to_cancel);
        if count > 0 {
            self.poke();
        }
        count
    }

    /// Cancel and synchronize every pending or in-flight operation bound to `conn`
    /// across all duplicated numeric handles, wake any synchronous writers waiting on `conn`,
    /// then wait one reactor cycle so any reactor snapshot drops its `Arc<PipeConn>` promptly.
    fn cancel_ops_for_conn(&self, conn: &Arc<PipeConn>) -> usize {
        let to_cancel: Vec<Arc<PendingPipeOp>> = {
            let st = self.state.lock().unwrap();
            self.gen_cv.notify_all();
            st.ops
                .iter()
                .filter(|o| {
                    !matches!(o.kind, PipeOpKind::SyncWriteBarrier)
                        && o.conn.as_ref().is_some_and(|c| Arc::ptr_eq(c, conn))
                })
                .cloned()
                .collect()
        };
        let count = self.finalize_cancelled_ops(to_cancel);
        let gen_before = {
            let st = self.state.lock().unwrap();
            st.poll_gen
        };
        self.poke();
        let mut st = self.state.lock().unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(50);
        while st.poll_gen == gen_before {
            let rem = deadline.saturating_duration_since(std::time::Instant::now());
            if rem.is_zero() {
                break;
            }
            let (next, res) = self.gen_cv.wait_timeout(st, rem).unwrap();
            st = next;
            if res.timed_out() {
                break;
            }
        }
        count
    }
}

fn pipe_reactor() -> &'static PipeIoReactor {
    static R: OnceLock<&'static PipeIoReactor> = OnceLock::new();
    R.get_or_init(|| {
        let mut wake = [0i32; 2];
        assert_eq!(unsafe { libc::pipe(wake.as_mut_ptr()) }, 0, "pipe reactor wakeup pipe");
        for &fd in &wake {
            set_cloexec(fd);
            unsafe {
                let fl = libc::fcntl(fd, libc::F_GETFL);
                if fl >= 0 {
                    let _ = libc::fcntl(fd, libc::F_SETFL, fl | libc::O_NONBLOCK);
                }
            }
        }
        let wake_r = wake[0];
        let reactor: &'static PipeIoReactor = Box::leak(Box::new(PipeIoReactor {
            state: Mutex::new(PipeReactorState { ops: Vec::new(), poll_gen: 0 }),
            gen_cv: Condvar::new(),
            wake_w: wake[1],
        }));
        std::thread::Builder::new()
            .name("win32-pipe-io".into())
            .spawn(move || {
                let mut pfds: Vec<libc::pollfd> = Vec::new();
                let mut snap: Vec<Arc<PendingPipeOp>> = Vec::new();
                loop {
                    pfds.clear();
                    snap.clear();
                    pfds.push(libc::pollfd { fd: wake_r, events: libc::POLLIN, revents: 0 });
                    {
                        let mut st = reactor.state.lock().unwrap();
                        st.poll_gen = st.poll_gen.wrapping_add(1);
                        reactor.gen_cv.notify_all();
                        st.ops.retain(|o| !o.done.load(Ordering::Acquire));
                        for (i, o) in st.ops.iter().enumerate() {
                            let events = match o.kind {
                                PipeOpKind::Read => libc::POLLIN,
                                PipeOpKind::Write => {
                                    if has_prior_pipe_write(&st.ops[..i], &o.file, o.conn.as_ref()) {
                                        continue;
                                    }
                                    libc::POLLOUT
                                }
                                PipeOpKind::SyncWriteBarrier => continue,
                            };
                            pfds.push(libc::pollfd { fd: o.fd, events, revents: 0 });
                            snap.push(Arc::clone(o));
                        }
                    }
                    // Check buffered FIFO peek bytes or disconnected server connections outside
                    // `reactor.state` so locks never nest and buffered reads never stall in poll().
                    let mut immediate_ready = false;
                    for op in &snap {
                        if op.done.load(Ordering::Acquire) {
                            continue;
                        }
                        if op.conn.as_ref().is_some_and(|c| c.is_disconnected()) {
                            immediate_ready = true;
                            break;
                        }
                        if matches!(op.kind, PipeOpKind::Read)
                            && op.len > 0
                            && !op.file.peek_buf.lock().unwrap().is_empty()
                        {
                            immediate_ready = true;
                            break;
                        }
                    }
                    let timeout_ms = if immediate_ready { 0 } else { -1 };
                    let pr = unsafe { libc::poll(pfds.as_mut_ptr(), pfds.len() as libc::nfds_t, timeout_ms) };
                    if pr < 0 {
                        if last_errno() == libc::EINTR {
                            continue;
                        }
                        break;
                    }
                    if (pfds[0].revents & libc::POLLIN) != 0 {
                        let mut drain = [0u8; 64];
                        while unsafe { libc::read(wake_r, drain.as_mut_ptr() as *mut c_void, drain.len()) } > 0 {}
                    }
                    for (idx, op) in snap.iter().enumerate() {
                        if op.done.load(Ordering::Acquire) {
                            continue;
                        }
                        let revents = pfds[idx + 1].revents;
                        let is_disc = op.conn.as_ref().is_some_and(|c| c.is_disconnected());
                        let has_peek = matches!(op.kind, PipeOpKind::Read)
                            && op.len > 0
                            && !op.file.peek_buf.lock().unwrap().is_empty();
                        if revents == 0 && !is_disc && !has_peek {
                            continue;
                        }
                        // Hold `op.state` across the entire non-blocking guest buffer access so
                        // `cancel_ops` / `cancel_ops_for_conn` cannot publish `STATUS_CANCELLED`
                        // while we touch `op.buf`.
                        let finished: Option<(usize, usize)> = {
                            let mut st = op.state.lock().unwrap();
                            if st.completed {
                                None
                            } else if op.conn.as_ref().is_some_and(|c| c.is_disconnected()) {
                                st.completed = true;
                                op.done.store(true, Ordering::Release);
                                Some((STATUS_CANCELLED, 0))
                            } else {
                                let step_res: Option<(usize, usize)> = match op.kind {
                                    PipeOpKind::Read => {
                                        let dst = unsafe { std::slice::from_raw_parts_mut(op.buf as *mut u8, op.len) };
                                        let from_peek = drain_peek_buf(&op.file, dst);
                                        if from_peek > 0 {
                                            Some((STATUS_SUCCESS, from_peek))
                                        } else {
                                            match unsafe { try_nonblocking_pipe_read(op.fd, op.buf as *mut u8, op.len) } {
                                                Ok(Some(0)) => Some((STATUS_PIPE_BROKEN, 0)),
                                                Ok(Some(got)) => Some((STATUS_SUCCESS, got)),
                                                Ok(None) => {
                                                    if (revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL)) != 0 {
                                                        Some((STATUS_PIPE_BROKEN, 0))
                                                    } else {
                                                        None
                                                    }
                                                }
                                                Err(e) => {
                                                    let w = if e == libc::EPIPE || e == libc::ECONNRESET {
                                                        err::ERROR_BROKEN_PIPE
                                                    } else {
                                                        errno_to_win32(e)
                                                    };
                                                    Some((win32_to_ntstatus(w), 0))
                                                }
                                            }
                                        }
                                    }
                                    PipeOpKind::Write => {
                                        let mut made_progress = false;
                                        loop {
                                            let rem = op.len.saturating_sub(st.written);
                                            if rem == 0 {
                                                break Some((STATUS_SUCCESS, st.written));
                                            }
                                            let ptr = unsafe { (op.buf as *const u8).add(st.written) };
                                            match unsafe { try_nonblocking_pipe_write(op.fd, ptr, rem) } {
                                                Ok(Some(0)) => break Some((STATUS_PIPE_BROKEN, st.written)),
                                                Ok(Some(k)) => {
                                                    st.written += k;
                                                    made_progress = true;
                                                    if st.written >= op.len {
                                                        break Some((STATUS_SUCCESS, st.written));
                                                    }
                                                }
                                                Ok(None) => {
                                                    if !made_progress
                                                        && (revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL)) != 0
                                                    {
                                                        break Some((STATUS_PIPE_BROKEN, st.written));
                                                    }
                                                    break None;
                                                }
                                                Err(e) => {
                                                    let w = if e == libc::EPIPE || e == libc::ECONNRESET {
                                                        err::ERROR_BROKEN_PIPE
                                                    } else {
                                                        errno_to_win32(e)
                                                    };
                                                    break Some((win32_to_ntstatus(w), st.written));
                                                }
                                            }
                                        }
                                    }
                                    PipeOpKind::SyncWriteBarrier => None,
                                };
                                if let Some(res) = step_res {
                                    st.completed = true;
                                    op.done.store(true, Ordering::Release);
                                    Some(res)
                                } else {
                                    None
                                }
                            }
                        };
                        if let Some((status, bytes)) = finished {
                            let was_write = matches!(op.kind, PipeOpKind::Write);
                            {
                                let mut st = reactor.state.lock().unwrap();
                                st.ops.retain(|o| !Arc::ptr_eq(o, op));
                                if was_write {
                                    reactor.gen_cv.notify_all();
                                }
                            }
                            op.publish_completion(status, bytes);
                        }
                    }
                    snap.clear();
                }
            })
            .expect("spawn win32-pipe-io thread");
        reactor
    })
}

// ---------------------------------------------------------------------------
// In-process Named Pipe Registry (`\\.\pipe\<name>` via AF_UNIX socketpair)
// ---------------------------------------------------------------------------

fn normalize_pipe_name(name: &str) -> Option<String> {
    let n = name.replace('/', "\\");
    let lower = n.to_ascii_lowercase();
    if let Some(rest) = lower.strip_prefix("\\\\.\\pipe\\") {
        if !rest.is_empty() {
            return Some(rest.to_string());
        }
    }
    if let Some(rest) = lower.strip_prefix("\\\\?\\pipe\\") {
        if !rest.is_empty() {
            return Some(rest.to_string());
        }
    }
    None
}

enum ServerPipeConn {
    Listening,
    Connected(Arc<PipeConn>),
    Disconnected,
}

enum PendingConnect {
    Sync {
        handle: usize,
        tid: u32,
        done: Arc<(Mutex<Option<usize>>, Condvar)>,
    },
    Overlapped {
        handle: usize,
        ov: usize,
        tid: u32,
        completed: Arc<AtomicBool>,
        binding: Arc<IoCompletionBinding>,
    },
}

struct ServerPipeInner {
    conn: ServerPipeConn,
    pending_connect: Option<PendingConnect>,
}

struct NamedPipeServer {
    norm_name: String,
    state: Mutex<ServerPipeInner>,
}

impl NamedPipeServer {
    fn connected_fd(&self) -> Option<i32> {
        let st = self.state.lock().unwrap();
        match &st.conn {
            ServerPipeConn::Connected(c) => Some(c.fd()),
            _ => None,
        }
    }

    fn io_conn(&self) -> Result<Arc<PipeConn>, u32> {
        let st = self.state.lock().unwrap();
        match &st.conn {
            ServerPipeConn::Connected(c) => Ok(Arc::clone(c)),
            ServerPipeConn::Listening => Err(ERROR_PIPE_LISTENING),
            ServerPipeConn::Disconnected => Err(ERROR_PIPE_NOT_CONNECTED),
        }
    }

    fn cancel_pending_connect(&self, handle: usize, ov: Option<usize>, filter_tid: Option<u32>) -> usize {
        let taken = {
            let mut st = self.state.lock().unwrap();
            let matches = match &st.pending_connect {
                Some(PendingConnect::Sync { handle: h, tid, .. }) => {
                    *h == handle && ov.is_none() && filter_tid.map_or(true, |t| *tid == t)
                }
                Some(PendingConnect::Overlapped { handle: h, ov: o, tid, .. }) => {
                    *h == handle
                        && ov.map_or(true, |target| *o == target)
                        && filter_tid.map_or(true, |t| *tid == t)
                }
                None => false,
            };
            if matches {
                st.pending_connect.take()
            } else {
                None
            }
        };
        let Some(pc) = taken else {
            return 0;
        };
        match pc {
            PendingConnect::Sync { done, .. } => {
                let (m, cv) = &*done;
                *m.lock().unwrap() = Some(STATUS_CANCELLED);
                cv.notify_all();
                1
            }
            PendingConnect::Overlapped { ov: o, completed, binding, .. } => {
                if !completed.swap(true, Ordering::AcqRel) {
                    unsafe {
                        let _ = complete_overlapped_binding(Some(&binding), o as *mut OVERLAPPED, STATUS_CANCELLED, 0, false);
                    }
                    1
                } else {
                    0
                }
            }
        }
    }

    fn close_server(self: &Arc<Self>) {
        let (old_conn, pending) = {
            let mut st = self.state.lock().unwrap();
            let prev = std::mem::replace(&mut st.conn, ServerPipeConn::Disconnected);
            let conn = match prev {
                ServerPipeConn::Connected(c) => Some(c),
                _ => None,
            };
            (conn, st.pending_connect.take())
        };
        if let Some(conn) = old_conn {
            conn.mark_disconnected();
            let _ = pipe_reactor().cancel_ops_for_conn(&conn);
        }
        if let Some(pc) = pending {
            match pc {
                PendingConnect::Sync { done, .. } => {
                    let (m, cv) = &*done;
                    *m.lock().unwrap() = Some(STATUS_CANCELLED);
                    cv.notify_all();
                }
                PendingConnect::Overlapped { ov: o, completed, binding, .. } => {
                    if !completed.swap(true, Ordering::AcqRel) {
                        unsafe {
                            let _ = complete_overlapped_binding(Some(&binding), o as *mut OVERLAPPED, STATUS_CANCELLED, 0, false);
                        }
                    }
                }
            }
        }
        let reg = named_pipe_registry();
        let mut map = reg.lock().unwrap();
        if let Some(entry) = map.get(&self.norm_name).cloned() {
            let mut srvs = entry.servers.lock().unwrap();
            srvs.retain(|s| !Arc::ptr_eq(s, self));
            if srvs.is_empty() {
                map.remove(&self.norm_name);
            }
            entry.cv.notify_all();
        }
    }
}

struct NamedPipeRegistryEntry {
    servers: Mutex<Vec<Arc<NamedPipeServer>>>,
    cv: Condvar,
}

fn named_pipe_registry() -> &'static Mutex<HashMap<String, Arc<NamedPipeRegistryEntry>>> {
    static REG: OnceLock<Mutex<HashMap<String, Arc<NamedPipeRegistryEntry>>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Wake `WaitNamedPipeW` waiters while holding `entry.servers` so notifications cannot
/// land in the check-to-sleep window. Lock order is `registry -> entry.servers -> srv.state`;
/// never call while holding `srv.state`.
fn notify_pipe_waiters(norm_name: &str) {
    let entry = named_pipe_registry().lock().unwrap().get(norm_name).cloned();
    if let Some(entry) = entry {
        let _srvs = entry.servers.lock().unwrap();
        entry.cv.notify_all();
    }
}

fn connect_named_pipe_client(
    norm_name: &str,
    orig_name: &str,
    access: u32,
    flags: u32,
    handle_flags: u32,
) -> Result<HANDLE, u32> {
    let entry = {
        let map = named_pipe_registry().lock().unwrap();
        map.get(norm_name).cloned()
    };
    let Some(entry) = entry else {
        return Err(err::ERROR_FILE_NOT_FOUND);
    };
    let servers = entry.servers.lock().unwrap().clone();
    if servers.is_empty() {
        return Err(err::ERROR_FILE_NOT_FOUND);
    }

    for srv in servers {
        let mut st = srv.state.lock().unwrap();
        if matches!(st.conn, ServerPipeConn::Listening) {
            let mut fds = [0i32; 2];
            if unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) } != 0 {
                return Err(errno_to_win32(last_errno()));
            }
            for &fd in &fds {
                set_cloexec(fd);
                set_nosigpipe_sock(fd);
            }
            st.conn = ServerPipeConn::Connected(PipeConn::new(unsafe { File::from_raw_fd(fds[0]) }));
            let pending = st.pending_connect.take();
            drop(st);

            if let Some(pc) = pending {
                match pc {
                    PendingConnect::Sync { done, .. } => {
                        let (m, cv) = &*done;
                        *m.lock().unwrap() = Some(STATUS_SUCCESS);
                        cv.notify_all();
                    }
                    PendingConnect::Overlapped { ov: o, completed, binding, .. } => {
                        if !completed.swap(true, Ordering::AcqRel) {
                            unsafe {
                                let _ = complete_overlapped_binding(
                                    Some(&binding),
                                    o as *mut OVERLAPPED,
                                    STATUS_SUCCESS,
                                    0,
                                    false,
                                );
                            }
                        }
                    }
                }
            }
            let client = FileObj::new(
                unsafe { File::from_raw_fd(fds[1]) },
                FileKind::Pipe,
                orig_name.to_string(),
                None,
                access,
                flags,
            );
            return Ok(handles::insert_with_flags(Arc::new(client), handle_flags));
        }
    }
    Err(ERROR_PIPE_BUSY)
}

pub unsafe extern "win64" fn CreatePipe(rd: *mut HANDLE, wr: *mut HANDLE, sec: *mut c_void, size: u32) -> BOOL {
    if rd.is_null() || wr.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    // Use an AF_UNIX SOCK_STREAM socketpair so overlapped/reactor/peek reads & writes can use
    // per-syscall `MSG_DONTWAIT` without ever modifying shared `O_NONBLOCK` status flags on
    // duplicated (`DuplicateHandle`) or child-inherited (`CreateProcessW`) descriptors.
    let mut fds = [0i32; 2];
    if libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) != 0 {
        set_err(errno_to_win32(last_errno()));
        return FALSE;
    }
    let buf_sz: libc::c_int = if size == 0 {
        65_536
    } else {
        (size as i64).clamp(4096, 262_144) as libc::c_int
    };
    for &fd in &fds {
        set_cloexec(fd);
        set_nosigpipe_sock(fd);
        let _ = libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_SNDBUF,
            &buf_sz as *const libc::c_int as *const c_void,
            std::mem::size_of::<libc::c_int>() as libc::socklen_t,
        );
        let _ = libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_RCVBUF,
            &buf_sz as *const libc::c_int as *const c_void,
            std::mem::size_of::<libc::c_int>() as libc::socklen_t,
        );
    }
    let hflags = sec_attr_inherit_flags(sec);
    let r = FileObj::new(File::from_raw_fd(fds[0]), FileKind::Pipe, "\\\\.\\pipe\\anon".into(), None, GENERIC_READ, 0);
    let w = FileObj::new(File::from_raw_fd(fds[1]), FileKind::Pipe, "\\\\.\\pipe\\anon".into(), None, GENERIC_WRITE, 0);
    std::ptr::write_unaligned(rd, handles::insert_with_flags(Arc::new(r), hflags));
    std::ptr::write_unaligned(wr, handles::insert_with_flags(Arc::new(w), hflags));
    TRUE
}

pub unsafe extern "win64" fn PeekNamedPipe(
    h: HANDLE,
    buf: *mut c_void,
    size: u32,
    read: *mut u32,
    avail: *mut u32,
    left: *mut u32,
) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if f.kind != FileKind::Pipe {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let target = match f.io_pipe_target() {
        Ok(t) => t,
        Err(ERROR_PIPE_LISTENING) => {
            if !read.is_null() {
                std::ptr::write_unaligned(read, 0);
            }
            if !avail.is_null() {
                std::ptr::write_unaligned(avail, 0);
            }
            if !left.is_null() {
                std::ptr::write_unaligned(left, 0);
            }
            return TRUE;
        }
        Err(e) => {
            set_err(e);
            return FALSE;
        }
    };
    let fd = target.fd();

    let want = if !buf.is_null() && size > 0 { size as usize } else { 0 };
    let mut buffered_new_fifo_bytes = false;
    let res: Result<(usize, usize), u32> = (|| {
        let mut pb = f.peek_buf.lock().unwrap();
        let mut copied = 0usize;
        let mut peeked_kernel = 0usize;

        if want > 0 {
            let from_pb = pb.len().min(want);
            let dst = buf as *mut u8;
            for i in 0..from_pb {
                std::ptr::write_unaligned(dst.add(i), pb[i]);
            }
            copied = from_pb;

            if want > from_pb {
                let need = want - from_pb;
                // Prefer non-consuming MSG_PEEK on socketpair-backed pipes so pending overlapped
                // reads on the same pipe continue seeing kernel readiness and all bytes intact.
                let mut is_raw_fifo = false;
                loop {
                    let r = libc::recv(
                        fd,
                        dst.add(from_pb) as *mut c_void,
                        need,
                        libc::MSG_PEEK | libc::MSG_DONTWAIT,
                    );
                    if r > 0 {
                        peeked_kernel = r as usize;
                        copied = from_pb + peeked_kernel;
                        break;
                    }
                    if r == 0 {
                        if pb.is_empty() {
                            return Err(err::ERROR_BROKEN_PIPE);
                        }
                        break;
                    }
                    let e = last_errno();
                    if e == libc::EINTR {
                        continue;
                    }
                    if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
                        break;
                    }
                    if e == libc::ENOTSOCK {
                        is_raw_fifo = true;
                        break;
                    }
                    if pb.is_empty() {
                        return Err(if e == libc::EPIPE || e == libc::ECONNRESET {
                            err::ERROR_BROKEN_PIPE
                        } else {
                            errno_to_win32(e)
                        });
                    }
                    break;
                }

                if is_raw_fifo {
                    // Raw POSIX FIFO fallback: MSG_PEEK is unavailable, so buffer peeked bytes
                    // into `f.peek_buf` and wake `pipe_reactor()` after releasing `pb`.
                    let mut tmp = vec![0u8; need];
                    match try_nonblocking_pipe_read(fd, tmp.as_mut_ptr(), need) {
                        Ok(Some(0)) => {
                            if pb.is_empty() {
                                return Err(err::ERROR_BROKEN_PIPE);
                            }
                        }
                        Ok(Some(got)) => {
                            pb.extend(&tmp[..got]);
                            buffered_new_fifo_bytes = true;
                            for i in from_pb..pb.len().min(want) {
                                std::ptr::write_unaligned(dst.add(i), pb[i]);
                            }
                            copied = pb.len().min(want);
                        }
                        Ok(None) => {}
                        Err(e) => {
                            if pb.is_empty() {
                                return Err(if e == libc::EPIPE || e == libc::ECONNRESET {
                                    err::ERROR_BROKEN_PIPE
                                } else {
                                    errno_to_win32(e)
                                });
                            }
                        }
                    }
                }
            }
        }

        let mut kernel_avail: libc::c_int = 0;
        let fionread_ok = libc::ioctl(fd, libc::FIONREAD, &mut kernel_avail) == 0;
        if !fionread_ok {
            if pb.is_empty() && peeked_kernel == 0 {
                return Err(err::ERROR_BROKEN_PIPE);
            }
            kernel_avail = 0;
        }
        let total_avail = pb
            .len()
            .saturating_add((kernel_avail.max(0) as usize).max(peeked_kernel));

        // If nothing buffered in userspace or kernel, detect peer close (POLLHUP / EOF) as ERROR_BROKEN_PIPE.
        if total_avail == 0 {
            let mut pfd = libc::pollfd { fd, events: libc::POLLIN, revents: 0 };
            let pr = libc::poll(&mut pfd, 1, 0);
            if pr > 0 {
                if (pfd.revents & (libc::POLLHUP | libc::POLLERR)) != 0 {
                    return Err(err::ERROR_BROKEN_PIPE);
                }
                if (pfd.revents & libc::POLLIN) != 0 {
                    let mut probe = 0u8;
                    let rr = libc::recv(
                        fd,
                        &mut probe as *mut u8 as *mut c_void,
                        1,
                        libc::MSG_PEEK | libc::MSG_DONTWAIT,
                    );
                    if rr == 0 {
                        return Err(err::ERROR_BROKEN_PIPE);
                    }
                }
            }
        }

        Ok((copied, total_avail))
    })();

    if buffered_new_fifo_bytes {
        // Wake the pipe reactor outside `f.peek_buf` so any pending FIFO read immediately
        // drains the newly buffered bytes without waiting for another write.
        pipe_reactor().poke();
    }

    let (copied, total_avail) = match res {
        Ok(v) => v,
        Err(e) => {
            set_err(e);
            return FALSE;
        }
    };

    if !read.is_null() {
        std::ptr::write_unaligned(read, copied as u32);
    }
    if !avail.is_null() {
        std::ptr::write_unaligned(avail, total_avail as u32);
    }
    if !left.is_null() {
        std::ptr::write_unaligned(left, total_avail.saturating_sub(copied) as u32);
    }
    TRUE
}

pub unsafe extern "win64" fn CreateNamedPipeW(
    name: LPCWSTR,
    open_mode: u32,
    _pipe_mode: u32,
    _max_instances: u32,
    _out_buf: u32,
    _in_buf: u32,
    _timeout: u32,
    sec: *mut c_void,
) -> HANDLE {
    if name.is_null() {
        set_err(err::ERROR_INVALID_NAME);
        return INVALID_HANDLE_VALUE;
    }
    let pipe_name = from_wide(name);
    let Some(norm_name) = normalize_pipe_name(&pipe_name) else {
        set_err(err::ERROR_INVALID_NAME);
        return INVALID_HANDLE_VALUE;
    };
    let server = Arc::new(NamedPipeServer {
        norm_name: norm_name.clone(),
        state: Mutex::new(ServerPipeInner {
            conn: ServerPipeConn::Listening,
            pending_connect: None,
        }),
    });
    {
        let mut reg = named_pipe_registry().lock().unwrap();
        let entry = Arc::clone(reg.entry(norm_name).or_insert_with(|| {
            Arc::new(NamedPipeRegistryEntry {
                servers: Mutex::new(Vec::new()),
                cv: Condvar::new(),
            })
        }));
        let mut srvs = entry.servers.lock().unwrap();
        srvs.push(Arc::clone(&server));
        entry.cv.notify_all();
    }

    let hflags = sec_attr_inherit_flags(sec);
    let mut obj = FileObj::new(
        null_file(),
        FileKind::Pipe,
        pipe_name,
        None,
        GENERIC_READ | GENERIC_WRITE,
        open_mode,
    );
    obj.named_pipe_server = Some(server);
    handles::insert_with_flags(Arc::new(obj), hflags)
}

pub unsafe extern "win64" fn CreateNamedPipeA(
    name: LPCSTR,
    open_mode: u32,
    pipe_mode: u32,
    max_instances: u32,
    out_buf: u32,
    in_buf: u32,
    timeout: u32,
    sec: *mut c_void,
) -> HANDLE {
    if name.is_null() {
        set_err(err::ERROR_INVALID_NAME);
        return INVALID_HANDLE_VALUE;
    }
    let w = to_wide_z(&from_ansi(name));
    CreateNamedPipeW(w.as_ptr(), open_mode, pipe_mode, max_instances, out_buf, in_buf, timeout, sec)
}

pub unsafe extern "win64" fn ConnectNamedPipe(h: HANDLE, ov: *mut OVERLAPPED) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let Some(srv) = f.named_pipe_server.clone() else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };

    let (became_listening, wait_done) = {
        let mut st = srv.state.lock().unwrap();
        let mut became_listening = false;
        match st.conn {
            ServerPipeConn::Connected(_) => {
                drop(st);
                if !ov.is_null() {
                    complete_overlapped(ov, STATUS_SUCCESS, 0);
                }
                set_err(ERROR_PIPE_CONNECTED);
                return FALSE;
            }
            ServerPipeConn::Disconnected => {
                st.conn = ServerPipeConn::Listening;
                became_listening = true;
            }
            ServerPipeConn::Listening => {}
        }

        let tid = teb::thread_id();
        if ov.is_null() {
            let done = Arc::new((Mutex::new(None), Condvar::new()));
            st.pending_connect = Some(PendingConnect::Sync {
                handle: h as usize,
                tid,
                done: Arc::clone(&done),
            });
            (became_listening, Some(done))
        } else {
            f.completion.io_event.reset();
            prepare_overlapped_pending(ov);
            st.pending_connect = Some(PendingConnect::Overlapped {
                handle: h as usize,
                ov: ov as usize,
                tid,
                completed: Arc::new(AtomicBool::new(false)),
                binding: Arc::clone(&f.completion),
            });
            (became_listening, None)
        }
    };
    if became_listening {
        notify_pipe_waiters(&srv.norm_name);
    }
    if let Some(done) = wait_done {
        drop(f);
        let (m, cv) = &*done;
        let mut g = m.lock().unwrap();
        let status = loop {
            if let Some(s) = *g {
                break s;
            }
            g = cv.wait(g).unwrap();
        };
        if status == STATUS_SUCCESS {
            set_err(0);
            TRUE
        } else {
            set_err(ntstatus_to_win32_local(status));
            FALSE
        }
    } else {
        set_err(err::ERROR_IO_PENDING);
        FALSE
    }
}

pub unsafe extern "win64" fn DisconnectNamedPipe(h: HANDLE) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let Some(srv) = f.named_pipe_server.clone() else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    // Transition through `Disconnected` first so no new client can attach while we cancel
    // and drain all operations belonging to the old shared connection across every duplicate handle.
    let old_conn = {
        let mut st = srv.state.lock().unwrap();
        let prev = std::mem::replace(&mut st.conn, ServerPipeConn::Disconnected);
        match prev {
            ServerPipeConn::Connected(c) => Some(c),
            _ => None,
        }
    };
    if let Some(conn) = old_conn {
        conn.mark_disconnected();
        let _ = pipe_reactor().cancel_ops_for_conn(&conn);
        drop(conn);
    } else {
        let _ = pipe_reactor().cancel_ops(h as usize, None, None);
    }
    f.peek_buf.lock().unwrap().clear();
    let entry = named_pipe_registry().lock().unwrap().get(&srv.norm_name).cloned();
    if let Some(entry) = entry {
        let _srvs = entry.servers.lock().unwrap();
        {
            let mut st = srv.state.lock().unwrap();
            if matches!(st.conn, ServerPipeConn::Disconnected) {
                st.conn = ServerPipeConn::Listening;
            }
        }
        entry.cv.notify_all();
    } else {
        let mut st = srv.state.lock().unwrap();
        if matches!(st.conn, ServerPipeConn::Disconnected) {
            st.conn = ServerPipeConn::Listening;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn SetNamedPipeHandleState(h: HANDLE, _mode: *const u32, _max_count: *const u32, _timeout: *const u32) -> BOOL {
    if get_file(h).is_none() {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn GetNamedPipeInfo(h: HANDLE, flags: *mut u32, out_sz: *mut u32, in_sz: *mut u32, max_inst: *mut u32) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    if !flags.is_null() {
        let fl = if f.named_pipe_server.is_some() { 0x1 /* PIPE_SERVER_END */ } else { 0x0 /* PIPE_CLIENT_END */ };
        std::ptr::write_unaligned(flags, fl);
    }
    if !out_sz.is_null() {
        std::ptr::write_unaligned(out_sz, 65536);
    }
    if !in_sz.is_null() {
        std::ptr::write_unaligned(in_sz, 65536);
    }
    if !max_inst.is_null() {
        std::ptr::write_unaligned(max_inst, 255);
    }
    TRUE
}

pub unsafe extern "win64" fn WaitNamedPipeW(name: LPCWSTR, timeout: u32) -> BOOL {
    if name.is_null() {
        set_err(err::ERROR_INVALID_NAME);
        return FALSE;
    }
    let Some(norm) = normalize_pipe_name(&from_wide(name)) else {
        set_err(err::ERROR_INVALID_NAME);
        return FALSE;
    };
    let entry = {
        let map = named_pipe_registry().lock().unwrap();
        map.get(&norm).cloned()
    };
    let Some(entry) = entry else {
        set_err(err::ERROR_FILE_NOT_FOUND);
        return FALSE;
    };
    let deadline = match timeout {
        INFINITE => None,
        0 => Some(Instant::now() + Duration::from_millis(5000)),
        ms => Some(Instant::now() + Duration::from_millis(ms as u64)),
    };
    let mut srvs = entry.servers.lock().unwrap();
    loop {
        if srvs.is_empty() {
            set_err(err::ERROR_FILE_NOT_FOUND);
            return FALSE;
        }
        if srvs.iter().any(|s| matches!(s.state.lock().unwrap().conn, ServerPipeConn::Listening)) {
            return TRUE;
        }
        match deadline {
            None => {
                srvs = entry.cv.wait(srvs).unwrap();
            }
            Some(dl) => {
                let now = Instant::now();
                if now >= dl {
                    set_err(ERROR_SEM_TIMEOUT);
                    return FALSE;
                }
                let (g, res) = entry.cv.wait_timeout(srvs, dl - now).unwrap();
                srvs = g;
                if res.timed_out() {
                    if srvs.is_empty() {
                        set_err(err::ERROR_FILE_NOT_FOUND);
                        return FALSE;
                    }
                    if srvs.iter().any(|s| matches!(s.state.lock().unwrap().conn, ServerPipeConn::Listening)) {
                        return TRUE;
                    }
                    set_err(ERROR_SEM_TIMEOUT);
                    return FALSE;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// File mappings
// ---------------------------------------------------------------------------

pub struct MappingObj {
    file: File,
    size: u64,
    writable: bool,
    exec: bool,
    name: Option<String>,
    /// Original Windows path if backed by a file handle (`None` for pagefile/anonymous sections).
    win_path: Option<String>,
}

impl KObject for MappingObj {
    fn type_name(&self) -> &'static str {
        "Section"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
struct MappedView {
    len: usize,
    win_path: Option<String>,
}

fn views() -> &'static Mutex<std::collections::BTreeMap<usize, MappedView>> {
    static V: OnceLock<Mutex<std::collections::BTreeMap<usize, MappedView>>> = OnceLock::new();
    V.get_or_init(Default::default)
}

/// If `addr` lies inside a mapped file view, returns `Some(Some(win_path))` for a
/// file-backed section or `Some(None)` for an anonymous/pagefile-backed section.
pub fn mapped_file_path_at(addr: usize) -> Option<Option<String>> {
    let map = views().lock().unwrap();
    let (&base, view) = map.range(..=addr).next_back()?;
    (addr < base.saturating_add(view.len)).then(|| view.win_path.clone())
}

fn anon_backing(size: u64) -> std::io::Result<File> {
    let dir = std::env::temp_dir();
    for i in 0..100u32 {
        let p = dir.join(format!("aw2-section-{}-{}-{}", std::process::id(), now_filetime(), i));
        if let Ok(f) = File::options().read(true).write(true).create_new(true).open(&p) {
            let _ = std::fs::remove_file(&p);
            f.set_len(size)?;
            return Ok(f);
        }
    }
    Err(std::io::Error::from_raw_os_error(libc::EEXIST))
}

pub fn mapping_size(h: HANDLE) -> Option<u64> {
    handles::get_as::<MappingObj>(h).map(|m| m.size)
}

pub unsafe fn create_mapping(h: HANDLE, protect: u32, size: u64, name: Option<String>) -> HANDLE {
    if let Some(n) = &name {
        if let Some(o) = handles::named_objects().lock().unwrap().get(n).cloned() {
            if o.as_any().is::<MappingObj>() {
                set_err(err::ERROR_ALREADY_EXISTS);
                return handles::insert(o);
            }
            set_err(6); // ERROR_INVALID_HANDLE (name collision with another type)
            return std::ptr::null_mut();
        }
    }
    let prot = protect & 0xFF;
    let writable = matches!(prot, 0x04 | 0x08 | 0x40 | 0x80);
    let exec = matches!(prot, 0x10 | 0x20 | 0x40 | 0x80);
    let (file, size, win_path) = if h == INVALID_HANDLE_VALUE || h.is_null() {
        if size == 0 {
            set_err(err::ERROR_INVALID_PARAMETER);
            return std::ptr::null_mut();
        }
        match anon_backing(size) {
            Ok(f) => (f, size, None),
            Err(e) => {
                set_err(io_err(&e));
                return std::ptr::null_mut();
            }
        }
    } else {
        let Some(f) = get_file(h) else {
            set_err(err::ERROR_INVALID_HANDLE);
            return std::ptr::null_mut();
        };
        if writable && f.is_protected_native() {
            set_err(err::ERROR_ACCESS_DENIED);
            return std::ptr::null_mut();
        }
        let cur = f.file().metadata().map(|m| m.len()).unwrap_or(0);
        if size > cur && writable {
            let _ = f.file().set_len(size);
        }
        let dup = match f.file().try_clone() {
            Ok(d) => d,
            Err(e) => {
                set_err(io_err(&e));
                return std::ptr::null_mut();
            }
        };
        let wp = (f.kind == FileKind::Disk && !f.win_path.is_empty()).then(|| f.win_path.clone());
        (dup, if size == 0 { cur } else { size }, wp)
    };
    if size == 0 {
        set_err(1006); // ERROR_FILE_INVALID: mapping an empty file
        return std::ptr::null_mut();
    }
    let obj: Arc<dyn KObject> = Arc::new(MappingObj { file, size, writable, exec, name: name.clone(), win_path });
    if let Some(n) = name {
        handles::named_objects().lock().unwrap().insert(n, obj.clone());
    }
    set_err(0);
    handles::insert(obj)
}

pub unsafe extern "win64" fn CreateFileMappingW(h: HANDLE, _sec: *mut c_void, protect: u32, hi: u32, lo: u32, name: LPCWSTR) -> HANDLE {
    create_mapping(h, protect, ((hi as u64) << 32) | lo as u64, from_wide_opt(name))
}
pub unsafe extern "win64" fn CreateFileMappingA(h: HANDLE, _sec: *mut c_void, protect: u32, hi: u32, lo: u32, name: LPCSTR) -> HANDLE {
    create_mapping(h, protect, ((hi as u64) << 32) | lo as u64, from_ansi_opt(name))
}
pub unsafe extern "win64" fn CreateFileMapping2(
    h: HANDLE,
    _sec: *mut c_void,
    _desired_access: u32,
    page_protect: u32,
    _alloc_attrs: u32,
    max_size: u64,
    name: LPCWSTR,
    _ext: *mut c_void,
    _ext_count: u32,
) -> HANDLE {
    create_mapping(h, page_protect, max_size, from_wide_opt(name))
}
pub unsafe extern "win64" fn CreateFileMappingFromApp(
    h: HANDLE,
    _sec: *mut c_void,
    page_protect: u32,
    max_size: u64,
    name: LPCWSTR,
) -> HANDLE {
    create_mapping(h, page_protect, max_size, from_wide_opt(name))
}
pub unsafe extern "win64" fn OpenFileMappingW(_access: u32, _inherit: BOOL, name: LPCWSTR) -> HANDLE {
    let n = from_wide(name);
    match handles::named_objects().lock().unwrap().get(&n).cloned() {
        Some(o) if o.as_any().is::<MappingObj>() => handles::insert(o),
        _ => {
            set_err(err::ERROR_FILE_NOT_FOUND);
            std::ptr::null_mut()
        }
    }
}

const FILE_MAP_COPY: u32 = 0x1;
const FILE_MAP_WRITE: u32 = 0x2;
const FILE_MAP_EXECUTE: u32 = 0x20;

pub unsafe fn map_view(h: HANDLE, access: u32, offset: u64, bytes: usize, at: usize) -> *mut c_void {
    let Some(m) = handles::get_as::<MappingObj>(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return std::ptr::null_mut();
    };
    if offset > m.size {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let len = if bytes == 0 { (m.size - offset) as usize } else { bytes };
    let copy = access & FILE_MAP_COPY != 0;
    let write = access & FILE_MAP_WRITE != 0 || copy;
    if write && !m.writable && !copy {
        set_err(err::ERROR_ACCESS_DENIED);
        return std::ptr::null_mut();
    }
    let mut prot = libc::PROT_READ;
    if write {
        prot |= libc::PROT_WRITE;
    }
    let is_exec_view = access & FILE_MAP_EXECUTE != 0 && m.exec;
    #[cfg(target_arch = "x86_64")]
    if is_exec_view {
        prot |= libc::PROT_EXEC;
    }
    let flags = if copy { libc::MAP_PRIVATE } else { libc::MAP_SHARED } | if at != 0 { libc::MAP_FIXED } else { 0 };
    let p = libc::mmap(at as *mut c_void, len, prot, flags, m.file.as_raw_fd(), offset as i64);
    if p == libc::MAP_FAILED {
        set_err(errno_to_win32(last_errno()));
        return std::ptr::null_mut();
    }
    let win_prot = match (is_exec_view, write, copy) {
        (true, _, true) => crate::kernel32::memory::PAGE_EXECUTE_WRITECOPY,
        (true, true, false) => crate::kernel32::memory::PAGE_EXECUTE_READWRITE,
        (true, false, false) => crate::kernel32::memory::PAGE_EXECUTE_READ,
        (false, _, true) => crate::kernel32::memory::PAGE_WRITECOPY,
        (false, true, false) => crate::kernel32::memory::PAGE_READWRITE,
        (false, false, false) => crate::kernel32::memory::PAGE_READONLY,
    };
    crate::kernel32::memory::register_mapped_view(p as usize, len, win_prot);
    log::debug!("MapViewOfFile({:?}, off={offset:#x}, len={len:#x}, exec={is_exec_view}) -> {p:p}", m.name);
    views().lock().unwrap().insert(p as usize, MappedView { len, win_path: m.win_path.clone() });
    p
}

pub unsafe extern "win64" fn MapViewOfFile(h: HANDLE, access: u32, hi: u32, lo: u32, bytes: usize) -> *mut c_void {
    map_view(h, access, ((hi as u64) << 32) | lo as u64, bytes, 0)
}
pub unsafe extern "win64" fn MapViewOfFileEx(h: HANDLE, access: u32, hi: u32, lo: u32, bytes: usize, at: *mut c_void) -> *mut c_void {
    map_view(h, access, ((hi as u64) << 32) | lo as u64, bytes, at as usize)
}
pub unsafe extern "win64" fn MapViewOfFileFromApp(h: HANDLE, access: u32, offset: u64, bytes: usize) -> *mut c_void {
    map_view(h, access, offset, bytes, 0)
}
pub unsafe extern "win64" fn MapViewOfFile3(
    h: HANDLE,
    proc_: HANDLE,
    base_address: *mut c_void,
    offset: u64,
    view_size: usize,
    _alloc_type: u32,
    page_protect: u32,
    _ext: *mut c_void,
    _ext_count: u32,
) -> *mut c_void {
    if !proc_.is_null()
        && proc_ != handles::CURRENT_PROCESS
        && handles::get_as::<crate::kernel32::process::ProcessObj>(proc_).is_none()
    {
        // Microsoft Learn (`MapViewOfFile3`): requires `PROCESS_VM_OPERATION` on `Process`.
        // Mapping views into another OS process is disallowed on macOS/iOS (SIP / hardened
        // runtime without `task_for_pid`), matching Windows returning `ERROR_ACCESS_DENIED`
        // for a remote process handle without `PROCESS_VM_OPERATION`, and `ERROR_INVALID_HANDLE`
        // when `Process` is not a valid process handle.
        if handles::get_as::<crate::kernel32::process::ChildProcessObj>(proc_).is_some() {
            set_err(err::ERROR_ACCESS_DENIED);
        } else {
            set_err(err::ERROR_INVALID_HANDLE);
        }
        return std::ptr::null_mut();
    }
    let Some(m) = handles::get_as::<MappingObj>(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return std::ptr::null_mut();
    };
    if offset > m.size {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let len = if view_size == 0 { (m.size - offset) as usize } else { view_size };
    let prot_byte = page_protect & 0xFF;
    let copy = matches!(prot_byte, 0x08 | 0x80);
    let write = matches!(prot_byte, 0x04 | 0x08 | 0x40 | 0x80);
    if write && !m.writable && !copy {
        set_err(err::ERROR_ACCESS_DENIED);
        return std::ptr::null_mut();
    }
    let host_prot = crate::kernel32::memory::win_prot_to_host(page_protect);
    #[cfg(not(target_arch = "x86_64"))]
    let host_prot = host_prot & !libc::PROT_EXEC;
    let at = base_address as usize;
    let flags = (if copy { libc::MAP_PRIVATE } else { libc::MAP_SHARED }) | if at != 0 { libc::MAP_FIXED } else { 0 };
    let p = libc::mmap(at as *mut c_void, len, host_prot, flags, m.file.as_raw_fd(), offset as i64);
    if p == libc::MAP_FAILED {
        set_err(errno_to_win32(last_errno()));
        return std::ptr::null_mut();
    }
    let addr = p as usize;
    crate::kernel32::memory::register_mapped_view(addr, len, page_protect);
    views().lock().unwrap().insert(addr, MappedView { len, win_path: m.win_path.clone() });
    p
}
pub unsafe extern "win64" fn MapViewOfFile3FromApp(
    h: HANDLE,
    proc_: HANDLE,
    base_address: *mut c_void,
    offset: u64,
    view_size: usize,
    alloc_type: u32,
    page_protect: u32,
    ext: *mut c_void,
    ext_count: u32,
) -> *mut c_void {
    MapViewOfFile3(h, proc_, base_address, offset, view_size, alloc_type, page_protect, ext, ext_count)
}
pub unsafe extern "win64" fn UnmapViewOfFile(p: *const c_void) -> BOOL {
    let addr = p as usize;
    let view = views().lock().unwrap().remove(&addr);
    match view {
        Some(v) => {
            crate::kernel32::memory::unregister_mapped_view(addr, v.len);
            libc::munmap(p as *mut c_void, v.len);
            TRUE
        }
        None => {
            set_err(err::ERROR_INVALID_ADDRESS);
            FALSE
        }
    }
}
pub unsafe extern "win64" fn UnmapViewOfFile2(_proc: HANDLE, p: *const c_void, f: u32) -> BOOL {
    UnmapViewOfFileEx(p, f)
}
pub unsafe extern "win64" fn UnmapViewOfFileEx(p: *const c_void, f: u32) -> BOOL {
    let addr = p as usize;
    let view = views().lock().unwrap().remove(&addr);
    match view {
        Some(v) => {
            if f & crate::kernel32::memory::MEM_PRESERVE_PLACEHOLDER != 0 {
                // Replace the mapped view with a reserved placeholder again.
                let _ = libc::mmap(
                    addr as *mut c_void,
                    v.len,
                    libc::PROT_NONE,
                    libc::MAP_PRIVATE | libc::MAP_ANON | libc::MAP_FIXED,
                    -1,
                    0,
                );
            } else {
                libc::munmap(p as *mut c_void, v.len);
            }
            TRUE
        }
        None => {
            set_err(err::ERROR_INVALID_ADDRESS);
            FALSE
        }
    }
}
pub unsafe extern "win64" fn FlushViewOfFile(p: *const c_void, n: usize) -> BOOL {
    let len = if n == 0 { views().lock().unwrap().get(&(p as usize)).map(|v| v.len).unwrap_or(0) } else { n };
    if len > 0 {
        let page = p as usize & !0xFFF;
        libc::msync(page as *mut c_void, len + (p as usize - page), libc::MS_ASYNC);
    }
    TRUE
}

// ---------------------------------------------------------------------------
// DeviceIoControl & real volume / IOKit storage inspection
// ---------------------------------------------------------------------------

const IOCTL_STORAGE_QUERY_PROPERTY: u32 = 0x002D_1400;
const IOCTL_STORAGE_GET_DEVICE_NUMBER: u32 = 0x002D_1080;
const IOCTL_STORAGE_CHECK_VERIFY: u32 = 0x002D_4800;
const IOCTL_STORAGE_CHECK_VERIFY2: u32 = 0x002D_0800;
const IOCTL_STORAGE_GET_MEDIA_TYPES_EX: u32 = 0x002D_0C04;
const IOCTL_STORAGE_GET_HOTPLUG_INFO: u32 = 0x002D_0C14;
const IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS: u32 = 0x0056_0000;
const IOCTL_VOLUME_IS_OFFLINE: u32 = 0x0056_0008;
const IOCTL_DISK_GET_DRIVE_GEOMETRY: u32 = 0x0007_0000;
const IOCTL_DISK_GET_DRIVE_GEOMETRY_EX: u32 = 0x0007_00A0;
const IOCTL_DISK_IS_WRITABLE: u32 = 0x0007_0024;
const IOCTL_DISK_GET_PARTITION_INFO_EX: u32 = 0x0007_0048;
const IOCTL_DISK_CHECK_VERIFY: u32 = 0x0007_4800;
const IOCTL_DISK_GET_LENGTH_INFO: u32 = 0x0007_405C;

const FSCTL_LOCK_VOLUME: u32 = 0x0009_0018;
const FSCTL_UNLOCK_VOLUME: u32 = 0x0009_001C;
const FSCTL_IS_VOLUME_MOUNTED: u32 = 0x0009_0028;
const FSCTL_GET_COMPRESSION: u32 = 0x0009_003C;
const FSCTL_SET_COMPRESSION: u32 = 0x0009_C040;
const FSCTL_GET_NTFS_VOLUME_DATA: u32 = 0x0009_0064;
const FSCTL_GET_REPARSE_POINT: u32 = 0x0009_00A8;
const FSCTL_SET_SPARSE: u32 = 0x0009_00C4;
const FSCTL_SET_ZERO_DATA: u32 = 0x0009_80C8;
const FSCTL_QUERY_ALLOCATED_RANGES: u32 = 0x0009_40CF;

const ERROR_NOT_A_REPARSE_POINT: u32 = 4390;
const ERROR_INVALID_REPARSE_DATA: u32 = 4392;

#[derive(Clone, Debug)]
struct VolumeStorageInfo {
    vendor: String,
    product: String,
    revision: String,
    serial: String,
    bus_type: u32,
    removable: bool,
    incurs_seek_penalty: bool,
    trim_enabled: bool,
    writable: bool,
    logical_sector_size: u32,
    physical_sector_size: u32,
    max_transfer_length: u32,
    max_physical_pages: u32,
    alignment_mask: u32,
    disk_number: u32,
    partition_number: u32,
    partition_offset: u64,
    volume_size_bytes: u64,
    volume_free_bytes: u64,
    cluster_size: u32,
    volume_serial: u64,
    media_uuid: String,
}

impl Default for VolumeStorageInfo {
    fn default() -> Self {
        Self {
            vendor: "Apple".into(),
            product: "APPLE SSD".into(),
            revision: "1.0".into(),
            serial: "0000000000000001".into(),
            bus_type: 17, // BusTypeNvme
            removable: false,
            incurs_seek_penalty: false,
            trim_enabled: true,
            writable: true,
            logical_sector_size: 4096,
            physical_sector_size: 4096,
            max_transfer_length: 1 << 20,
            max_physical_pages: 256,
            alignment_mask: 3,
            disk_number: 0,
            partition_number: 1,
            partition_offset: 0,
            volume_size_bytes: 0,
            volume_free_bytes: 0,
            cluster_size: 4096,
            volume_serial: 0,
            media_uuid: String::new(),
        }
    }
}

#[cfg(target_os = "macos")]
mod iokit_storage {
    use super::VolumeStorageInfo;
    use std::ffi::{c_char, c_void, CStr, CString};
    use std::sync::OnceLock;

    const K_CFSTRING_ENCODING_UTF8: u32 = 0x0800_0100;
    const K_CFNUMBER_SINT64: isize = 4;
    const K_IOREG_ITERATE_RECURSIVELY: u32 = 0x0000_0001;
    const K_IOREG_ITERATE_PARENTS: u32 = 0x0000_0002;

    struct IoKitApis {
        io_bsd_name_matching: unsafe extern "C" fn(u32, u32, *const c_char) -> *mut c_void,
        io_service_get_matching_service: unsafe extern "C" fn(u32, *mut c_void) -> u32,
        io_reg_search_cf_prop: unsafe extern "C" fn(u32, *const c_char, *const c_void, *const c_void, u32) -> *const c_void,
        io_object_release: unsafe extern "C" fn(u32) -> i32,
        cf_release: unsafe extern "C" fn(*const c_void),
        cf_dict_get_value: unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void,
        cf_string_create: unsafe extern "C" fn(*const c_void, *const c_char, u32) -> *const c_void,
        cf_string_get_cstr: unsafe extern "C" fn(*const c_void, *mut c_char, isize, u32) -> u8,
        cf_number_get_value: unsafe extern "C" fn(*const c_void, isize, *mut c_void) -> u8,
        cf_boolean_get_value: unsafe extern "C" fn(*const c_void) -> u8,
        cf_get_type_id: unsafe extern "C" fn(*const c_void) -> usize,
        cf_string_type_id: unsafe extern "C" fn() -> usize,
        cf_number_type_id: unsafe extern "C" fn() -> usize,
        cf_boolean_type_id: unsafe extern "C" fn() -> usize,
        cf_dict_type_id: unsafe extern "C" fn() -> usize,
    }

    fn apis() -> Option<&'static IoKitApis> {
        static APIS: OnceLock<Option<IoKitApis>> = OnceLock::new();
        APIS.get_or_init(|| unsafe {
            let iokit = libc::dlopen(
                c"/System/Library/Frameworks/IOKit.framework/IOKit".as_ptr(),
                libc::RTLD_LAZY | libc::RTLD_LOCAL,
            );
            let cf = libc::dlopen(
                c"/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation".as_ptr(),
                libc::RTLD_LAZY | libc::RTLD_LOCAL,
            );
            if iokit.is_null() || cf.is_null() {
                return None;
            }
            macro_rules! sym {
                ($lib:expr, $name:expr, $ty:ty) => {{
                    let p = libc::dlsym($lib, $name.as_ptr());
                    if p.is_null() {
                        return None;
                    }
                    std::mem::transmute::<*mut c_void, $ty>(p)
                }};
            }
            Some(IoKitApis {
                io_bsd_name_matching: sym!(iokit, c"IOBSDNameMatching", unsafe extern "C" fn(u32, u32, *const c_char) -> *mut c_void),
                io_service_get_matching_service: sym!(iokit, c"IOServiceGetMatchingService", unsafe extern "C" fn(u32, *mut c_void) -> u32),
                io_reg_search_cf_prop: sym!(
                    iokit,
                    c"IORegistryEntrySearchCFProperty",
                    unsafe extern "C" fn(u32, *const c_char, *const c_void, *const c_void, u32) -> *const c_void
                ),
                io_object_release: sym!(iokit, c"IOObjectRelease", unsafe extern "C" fn(u32) -> i32),
                cf_release: sym!(cf, c"CFRelease", unsafe extern "C" fn(*const c_void)),
                cf_dict_get_value: sym!(cf, c"CFDictionaryGetValue", unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void),
                cf_string_create: sym!(cf, c"CFStringCreateWithCString", unsafe extern "C" fn(*const c_void, *const c_char, u32) -> *const c_void),
                cf_string_get_cstr: sym!(cf, c"CFStringGetCString", unsafe extern "C" fn(*const c_void, *mut c_char, isize, u32) -> u8),
                cf_number_get_value: sym!(cf, c"CFNumberGetValue", unsafe extern "C" fn(*const c_void, isize, *mut c_void) -> u8),
                cf_boolean_get_value: sym!(cf, c"CFBooleanGetValue", unsafe extern "C" fn(*const c_void) -> u8),
                cf_get_type_id: sym!(cf, c"CFGetTypeID", unsafe extern "C" fn(*const c_void) -> usize),
                cf_string_type_id: sym!(cf, c"CFStringGetTypeID", unsafe extern "C" fn() -> usize),
                cf_number_type_id: sym!(cf, c"CFNumberGetTypeID", unsafe extern "C" fn() -> usize),
                cf_boolean_type_id: sym!(cf, c"CFBooleanGetTypeID", unsafe extern "C" fn() -> usize),
                cf_dict_type_id: sym!(cf, c"CFDictionaryGetTypeID", unsafe extern "C" fn() -> usize),
            })
        })
        .as_ref()
    }

    unsafe fn cf_str(a: &IoKitApis, v: *const c_void) -> Option<String> {
        if v.is_null() || (a.cf_get_type_id)(v) != (a.cf_string_type_id)() {
            return None;
        }
        let mut buf = [0 as c_char; 256];
        if (a.cf_string_get_cstr)(v, buf.as_mut_ptr(), buf.len() as isize, K_CFSTRING_ENCODING_UTF8) != 0 {
            Some(CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned())
        } else {
            None
        }
    }

    unsafe fn cf_i64(a: &IoKitApis, v: *const c_void) -> Option<i64> {
        if v.is_null() || (a.cf_get_type_id)(v) != (a.cf_number_type_id)() {
            return None;
        }
        let mut out = 0i64;
        if (a.cf_number_get_value)(v, K_CFNUMBER_SINT64, &mut out as *mut i64 as *mut c_void) != 0 {
            Some(out)
        } else {
            None
        }
    }

    unsafe fn cf_bool(a: &IoKitApis, v: *const c_void) -> Option<bool> {
        if v.is_null() || (a.cf_get_type_id)(v) != (a.cf_boolean_type_id)() {
            return None;
        }
        Some((a.cf_boolean_get_value)(v) != 0)
    }

    unsafe fn dict_val(a: &IoKitApis, dict: *const c_void, key: &CStr) -> *const c_void {
        if dict.is_null() || (a.cf_get_type_id)(dict) != (a.cf_dict_type_id)() {
            return std::ptr::null();
        }
        let cf_k = (a.cf_string_create)(std::ptr::null(), key.as_ptr(), K_CFSTRING_ENCODING_UTF8);
        if cf_k.is_null() {
            return std::ptr::null();
        }
        let v = (a.cf_dict_get_value)(dict, cf_k);
        (a.cf_release)(cf_k);
        v
    }

    unsafe fn search_prop(a: &IoKitApis, svc: u32, key: &CStr, opts: u32) -> *const c_void {
        let cf_k = (a.cf_string_create)(std::ptr::null(), key.as_ptr(), K_CFSTRING_ENCODING_UTF8);
        if cf_k.is_null() {
            return std::ptr::null();
        }
        let v = (a.io_reg_search_cf_prop)(svc, c"IOService".as_ptr(), cf_k, std::ptr::null(), opts);
        (a.cf_release)(cf_k);
        v
    }

    pub fn enrich_from_bsd_name(bsd_name: &str, info: &mut VolumeStorageInfo) {
        let Some(a) = apis() else { return };
        let Ok(c_bsd) = CString::new(bsd_name) else { return };
        unsafe {
            let matching = (a.io_bsd_name_matching)(0, 0, c_bsd.as_ptr());
            if matching.is_null() {
                return;
            }
            let svc = (a.io_service_get_matching_service)(0, matching);
            if svc == 0 {
                return;
            }
            let up = K_IOREG_ITERATE_RECURSIVELY | K_IOREG_ITERATE_PARENTS;

            // Direct IOMedia properties
            let v_unit = search_prop(a, svc, c"BSD Unit", 0);
            if let Some(u) = cf_i64(a, v_unit) {
                if u >= 0 {
                    info.disk_number = u as u32;
                }
            }
            if !v_unit.is_null() {
                (a.cf_release)(v_unit);
            }

            let v_part = search_prop(a, svc, c"Partition ID", 0);
            if let Some(p) = cf_i64(a, v_part) {
                if p > 0 {
                    info.partition_number = p as u32;
                }
            }
            if !v_part.is_null() {
                (a.cf_release)(v_part);
            }

            let v_base = search_prop(a, svc, c"Base", 0);
            if let Some(b) = cf_i64(a, v_base) {
                if b >= 0 {
                    info.partition_offset = b as u64;
                }
            }
            if !v_base.is_null() {
                (a.cf_release)(v_base);
            }

            let v_uuid = search_prop(a, svc, c"UUID", 0);
            if let Some(u) = cf_str(a, v_uuid) {
                info.media_uuid = u;
            }
            if !v_uuid.is_null() {
                (a.cf_release)(v_uuid);
            }

            let v_rem = search_prop(a, svc, c"Removable", up);
            if let Some(r) = cf_bool(a, v_rem) {
                info.removable = r;
            }
            if !v_rem.is_null() {
                (a.cf_release)(v_rem);
            }

            // Block/sector sizes & transfer limits from IOBlockStorageDevice ancestors
            for (key, dst) in [
                (c"Logical Block Size", &mut info.logical_sector_size),
                (c"Physical Block Size", &mut info.physical_sector_size),
                (c"IOMaximumByteCountRead", &mut info.max_transfer_length),
                (c"IOMaximumSegmentCountRead", &mut info.max_physical_pages),
            ] {
                let v = search_prop(a, svc, key, up);
                if let Some(n) = cf_i64(a, v) {
                    if n > 0 {
                        *dst = (n as u64).min(u32::MAX as u64) as u32;
                    }
                }
                if !v.is_null() {
                    (a.cf_release)(v);
                }
            }

            let v_align = search_prop(a, svc, c"IOMinimumSegmentAlignmentByteCount", up);
            if let Some(n) = cf_i64(a, v_align) {
                if n > 0 {
                    info.alignment_mask = (n as u32).saturating_sub(1);
                }
            }
            if !v_align.is_null() {
                (a.cf_release)(v_align);
            }

            // Device Characteristics dictionary: Vendor, Product, Revision, Serial, Medium Type
            let dev_chars = search_prop(a, svc, c"Device Characteristics", up);
            if !dev_chars.is_null() {
                if let Some(s) = cf_str(a, dict_val(a, dev_chars, c"Vendor Name")).filter(|s| !s.is_empty()) {
                    info.vendor = s;
                }
                if let Some(s) = cf_str(a, dict_val(a, dev_chars, c"Product Name")).filter(|s| !s.is_empty()) {
                    info.product = s;
                }
                if let Some(s) = cf_str(a, dict_val(a, dev_chars, c"Product Revision Level")).filter(|s| !s.is_empty()) {
                    info.revision = s;
                }
                if let Some(s) = cf_str(a, dict_val(a, dev_chars, c"Serial Number")).filter(|s| !s.is_empty()) {
                    info.serial = s;
                }
                if let Some(m) = cf_str(a, dict_val(a, dev_chars, c"Medium Type")) {
                    info.incurs_seek_penalty = m.eq_ignore_ascii_case("Rotational");
                }
                (a.cf_release)(dev_chars);
            }

            // Protocol Characteristics: Physical Interconnect -> STORAGE_BUS_TYPE
            let proto_chars = search_prop(a, svc, c"Protocol Characteristics", up);
            if !proto_chars.is_null() {
                if let Some(interconnect) = cf_str(a, dict_val(a, proto_chars, c"Physical Interconnect")) {
                    let lc = interconnect.to_ascii_lowercase();
                    info.bus_type = if lc.contains("nvme") || lc.contains("apple fabric") || lc.contains("pci") {
                        17 // BusTypeNvme
                    } else if lc.contains("usb") {
                        7 // BusTypeUsb
                    } else if lc.contains("sata") {
                        11 // BusTypeSata
                    } else if lc.contains("secure digital") || lc == "sd" {
                        12 // BusTypeSd
                    } else if lc.contains("virtual") || lc.contains("file") {
                        15 // BusTypeFileBackedVirtual
                    } else if lc.contains("sas") {
                        10 // BusTypeSas
                    } else {
                        17
                    };
                }
                (a.cf_release)(proto_chars);
            }

            // IOStorageFeatures: Unmap -> TRIM support
            let features = search_prop(a, svc, c"IOStorageFeatures", up);
            if !features.is_null() {
                if let Some(unmap) = cf_bool(a, dict_val(a, features, c"Unmap")) {
                    info.trim_enabled = unmap;
                }
                (a.cf_release)(features);
            }

            (a.io_object_release)(svc);
        }
    }
}

unsafe fn query_volume_storage_info(f: &FileObj) -> VolumeStorageInfo {
    let mut info = VolumeStorageInfo::default();
    let mut sfs: libc::statfs = std::mem::zeroed();
    let got_sfs = if f.kind == FileKind::Disk || f.kind == FileKind::Dir {
        libc::fstatfs(f.fd(), &mut sfs) == 0
    } else {
        // Resolve volume device handles (e.g. `\\.\C:`, `\\.\PhysicalDrive0`, `\\?\Volume{...}`)
        // to the mounted host directory for that drive letter (or C:\ -> /).
        let drive_win = if f.win_path.len() >= 6 && f.win_path.starts_with("\\\\.\\") && f.win_path.as_bytes()[5] == b':' {
            format!("{}:\\", f.win_path.as_bytes()[4] as char)
        } else {
            "C:\\".to_string()
        };
        let host = path::to_host(&drive_win).unwrap_or_else(|| PathBuf::from("/"));
        if let Ok(c) = std::ffi::CString::new(host.as_os_str().as_encoded_bytes()) {
            libc::statfs(c.as_ptr(), &mut sfs) == 0
        } else {
            false
        }
    };
    if got_sfs {
        let bsize = (sfs.f_bsize as u64).max(512);
        info.cluster_size = bsize as u32;
        info.logical_sector_size = (bsize as u32).min(4096);
        info.physical_sector_size = bsize as u32;
        info.volume_size_bytes = (sfs.f_blocks as u64).saturating_mul(bsize);
        info.volume_free_bytes = (sfs.f_bfree as u64).saturating_mul(bsize);
        info.writable = (sfs.f_flags & libc::MNT_RDONLY as u32) == 0;
        let fsid_words = std::ptr::read_unaligned(&sfs.f_fsid as *const libc::fsid_t as *const [u32; 2]);
        info.volume_serial = ((fsid_words[1] as u64) << 32) | (fsid_words[0] as u64);

        let mnt_from = std::ffi::CStr::from_ptr(sfs.f_mntfromname.as_ptr()).to_string_lossy();
        if let Some(bsd) = mnt_from.strip_prefix("/dev/") {
            // APFS snapshot mounts look like `/dev/disk3s1s1` or `com.apple.os.update-...@/dev/disk3s1`;
            // extract the base BSD device name (e.g. `disk3s1` or `disk3`).
            let bsd_clean = bsd.split('@').next_back().unwrap_or(bsd).strip_prefix("/dev/").unwrap_or(bsd);
            // Parse diskN and sM numbers as fallback if IOKit doesn't report them.
            if let Some(rest) = bsd_clean.strip_prefix("disk") {
                let mut parts = rest.split('s');
                if let Some(d) = parts.next().and_then(|x| x.parse::<u32>().ok()) {
                    info.disk_number = d;
                }
                if let Some(p) = parts.next().and_then(|x| x.parse::<u32>().ok()) {
                    info.partition_number = p;
                }
            }
            #[cfg(target_os = "macos")]
            {
                iokit_storage::enrich_from_bsd_name(bsd_clean, &mut info);
            }
        }
    }
    if info.volume_size_bytes == 0 {
        info.volume_size_bytes = 1u64 << 40;
    }
    info
}

pub unsafe extern "win64" fn DeviceIoControl(
    h: HANDLE,
    code: u32,
    inp: *const c_void,
    in_len: u32,
    out: *mut c_void,
    out_len: u32,
    ret: *mut u32,
    ov: *mut OVERLAPPED,
) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let out8 = out as *mut u8;
    let put = |data: &[u8]| -> BOOL {
        if !data.is_empty() && out8.is_null() {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        let n = data.len().min(out_len as usize);
        if n < data.len() && n < 8 {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        if n > 0 {
            std::ptr::copy_nonoverlapping(data.as_ptr(), out8, n);
        }
        if !ret.is_null() {
            std::ptr::write_unaligned(ret, n as u32);
        }
        if !ov.is_null() {
            complete_overlapped(ov, STATUS_SUCCESS, n);
        }
        if n < data.len() {
            set_err(err::ERROR_MORE_DATA);
            return FALSE;
        }
        TRUE
    };
    let put_exact = |data: &[u8]| -> BOOL {
        if (out_len as usize) < data.len() || (!data.is_empty() && out8.is_null()) {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        if !data.is_empty() {
            std::ptr::copy_nonoverlapping(data.as_ptr(), out8, data.len());
        }
        if !ret.is_null() {
            std::ptr::write_unaligned(ret, data.len() as u32);
        }
        if !ov.is_null() {
            complete_overlapped(ov, STATUS_SUCCESS, data.len());
        }
        TRUE
    };
    match code {
        IOCTL_STORAGE_QUERY_PROPERTY => {
            if inp.is_null() || in_len < 8 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let prop = std::ptr::read_unaligned(inp as *const u32);
            let qtype = std::ptr::read_unaligned((inp as *const u8).add(4) as *const u32);
            log::debug!("DeviceIoControl(IOCTL_STORAGE_QUERY_PROPERTY, prop={prop}, type={qtype})");
            let supported = matches!(prop, 0 | 1 | 2 | 3 | 4 | 6 | 7 | 8 | 9 | 11 | 12 | 14 | 17);
            if qtype == 1 {
                // PropertyExistsQuery
                return if supported { put_exact(&[]) } else { fail_not_supported() };
            }
            if qtype != 0 {
                // PropertyMaskQuery / PropertyQueryMaxDefined
                return fail_not_supported();
            }
            if !supported {
                // Standard Windows storage stack returns ERROR_NOT_SUPPORTED (50) for properties
                // not exposed by the underlying disk/controller (e.g. RPMB, inline crypto, miniport).
                return fail_not_supported();
            }
            let vol = query_volume_storage_info(&f);
            match prop {
                0 => {
                    // StorageDeviceProperty -> STORAGE_DEVICE_DESCRIPTOR (header 40 bytes + NUL strings)
                    let mut d = vec![0u8; 40];
                    let strings = [
                        format!("{}\0", vol.vendor),
                        format!("{}\0", vol.product),
                        format!("{}\0", vol.revision),
                        format!("{}\0", vol.serial),
                    ];
                    let mut offs = [0u32; 4];
                    for (i, s) in strings.iter().enumerate() {
                        offs[i] = d.len() as u32;
                        d.extend_from_slice(s.as_bytes());
                    }
                    let total = d.len() as u32;
                    d[0..4].copy_from_slice(&40u32.to_le_bytes());
                    d[4..8].copy_from_slice(&total.to_le_bytes());
                    d[8] = 0; // DeviceType (DIRECT_ACCESS_DEVICE)
                    d[10] = vol.removable as u8;
                    d[11] = 1; // CommandQueueing
                    for (i, o) in offs.iter().enumerate() {
                        d[12 + i * 4..16 + i * 4].copy_from_slice(&o.to_le_bytes());
                    }
                    d[28..32].copy_from_slice(&vol.bus_type.to_le_bytes());
                    put(&d)
                }
                1 => {
                    // StorageAdapterProperty -> STORAGE_ADAPTER_DESCRIPTOR (32 bytes)
                    let mut d = [0u8; 32];
                    d[0..4].copy_from_slice(&32u32.to_le_bytes());
                    d[4..8].copy_from_slice(&32u32.to_le_bytes());
                    d[8..12].copy_from_slice(&vol.max_transfer_length.to_le_bytes());
                    d[12..16].copy_from_slice(&vol.max_physical_pages.to_le_bytes());
                    d[16..20].copy_from_slice(&vol.alignment_mask.to_le_bytes());
                    d[22] = 1; // CommandQueueing
                    d[25] = vol.bus_type as u8;
                    put(&d)
                }
                2 => {
                    // StorageDeviceIdProperty -> STORAGE_DEVICE_ID_DESCRIPTOR (12 bytes + STORAGE_IDENTIFIER)
                    let id_bytes = if !vol.media_uuid.is_empty() { vol.media_uuid.as_bytes() } else { vol.serial.as_bytes() };
                    let ident_len = (16 + id_bytes.len() + 3) & !3;
                    let total = 12 + ident_len;
                    let mut d = vec![0u8; total];
                    d[0..4].copy_from_slice(&12u32.to_le_bytes()); // Version
                    d[4..8].copy_from_slice(&(total as u32).to_le_bytes()); // Size
                    d[8..12].copy_from_slice(&1u32.to_le_bytes()); // NumberOfIdentifiers
                    // STORAGE_IDENTIFIER @12: CodeSet=Ascii(2), Type=VendorId(1), IdentifierSize, NextOffset=0, Association=Device(0)
                    d[12..16].copy_from_slice(&2u32.to_le_bytes());
                    d[16..20].copy_from_slice(&1u32.to_le_bytes());
                    d[20..22].copy_from_slice(&(id_bytes.len() as u16).to_le_bytes());
                    d[22..24].copy_from_slice(&0u16.to_le_bytes());
                    d[24..28].copy_from_slice(&0u32.to_le_bytes());
                    d[28..28 + id_bytes.len()].copy_from_slice(id_bytes);
                    put(&d)
                }
                3 => {
                    // StorageDeviceUniqueIdProperty -> STORAGE_DEVICE_UNIQUE_IDENTIFIER (20 bytes)
                    let mut d = [0u8; 20];
                    d[0..4].copy_from_slice(&20u32.to_le_bytes());
                    d[4..8].copy_from_slice(&20u32.to_le_bytes());
                    put(&d)
                }
                4 => {
                    // StorageDeviceWriteCacheProperty -> STORAGE_WRITE_CACHE_PROPERTY (28 bytes)
                    let mut d = [0u8; 28];
                    d[0..4].copy_from_slice(&28u32.to_le_bytes()); // Version
                    d[4..8].copy_from_slice(&28u32.to_le_bytes()); // Size
                    d[8..12].copy_from_slice(&2u32.to_le_bytes()); // WriteCacheTypeWriteBack
                    d[12..16].copy_from_slice(&1u32.to_le_bytes()); // WriteCacheEnableEnabled
                    d[16..20].copy_from_slice(&0u32.to_le_bytes()); // WriteCacheChangeUnknown
                    d[20..24].copy_from_slice(&1u32.to_le_bytes()); // WriteThroughSupported
                    d[24] = 1; // FlushCacheSupported
                    put(&d)
                }
                6 => {
                    // StorageAccessAlignmentProperty -> STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR (28 bytes)
                    let vals: [u32; 7] = [
                        28,
                        28,
                        64, // BytesPerCacheLine
                        0,  // BytesOffsetForCacheAlignment
                        vol.logical_sector_size,
                        vol.physical_sector_size,
                        0,  // BytesOffsetForSectorAlignment
                    ];
                    put(std::slice::from_raw_parts(vals.as_ptr() as *const u8, 28))
                }
                7 => {
                    // StorageDeviceSeekPenaltyProperty -> DEVICE_SEEK_PENALTY_DESCRIPTOR (12 bytes)
                    let mut d = [0u8; 12];
                    d[0..4].copy_from_slice(&12u32.to_le_bytes());
                    d[4..8].copy_from_slice(&12u32.to_le_bytes());
                    d[8] = vol.incurs_seek_penalty as u8;
                    put(&d)
                }
                8 => {
                    // StorageDeviceTrimProperty -> DEVICE_TRIM_DESCRIPTOR (12 bytes)
                    let mut d = [0u8; 12];
                    d[0..4].copy_from_slice(&12u32.to_le_bytes());
                    d[4..8].copy_from_slice(&12u32.to_le_bytes());
                    d[8] = vol.trim_enabled as u8;
                    put(&d)
                }
                9 => {
                    // StorageDeviceWriteAggregationProperty -> DEVICE_WRITE_AGGREGATION_DESCRIPTOR (12 bytes)
                    let mut d = [0u8; 12];
                    d[0..4].copy_from_slice(&12u32.to_le_bytes());
                    d[4..8].copy_from_slice(&12u32.to_le_bytes());
                    d[8] = 1; // BenefitsFromWriteAggregation
                    put(&d)
                }
                11 => {
                    // StorageDevicePowerProperty -> DEVICE_POWER_DESCRIPTOR (20 bytes)
                    let mut d = [0u8; 20];
                    d[0..4].copy_from_slice(&20u32.to_le_bytes());
                    d[4..8].copy_from_slice(&20u32.to_le_bytes());
                    d[8] = 1; // DeviceAttentionSupported
                    put(&d)
                }
                12 => {
                    // StorageDeviceCopyOffloadProperty -> DEVICE_COPY_OFFLOAD_DESCRIPTOR (40 bytes)
                    let mut d = [0u8; 40];
                    d[0..4].copy_from_slice(&40u32.to_le_bytes());
                    d[4..8].copy_from_slice(&40u32.to_le_bytes());
                    d[8..12].copy_from_slice(&vol.max_transfer_length.to_le_bytes());
                    put(&d)
                }
                14 => {
                    // StorageDeviceMediumProductType -> STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR (12 bytes)
                    let mut d = [0u8; 12];
                    d[0..4].copy_from_slice(&12u32.to_le_bytes());
                    d[4..8].copy_from_slice(&12u32.to_le_bytes());
                    // 0x04 = NVMe SSD / generic flash
                    d[8..12].copy_from_slice(&(if vol.incurs_seek_penalty { 0u32 } else { 4u32 }).to_le_bytes());
                    put(&d)
                }
                17 => {
                    // StorageDeviceIoCapabilityProperty -> STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR (16 bytes)
                    let mut d = [0u8; 16];
                    d[0..4].copy_from_slice(&16u32.to_le_bytes());
                    d[4..8].copy_from_slice(&16u32.to_le_bytes());
                    d[8..12].copy_from_slice(&64u32.to_le_bytes());  // LunMaxIoCount
                    d[12..16].copy_from_slice(&256u32.to_le_bytes()); // AdapterMaxIoCount
                    put(&d)
                }
                _ => fail_not_supported(),
            }
        }
        IOCTL_STORAGE_GET_DEVICE_NUMBER => {
            let vol = query_volume_storage_info(&f);
            // STORAGE_DEVICE_NUMBER { DeviceType=FILE_DEVICE_DISK(7), DeviceNumber, PartitionNumber }
            let vals: [u32; 3] = [7, vol.disk_number, vol.partition_number];
            put_exact(std::slice::from_raw_parts(vals.as_ptr() as *const u8, 12))
        }
        IOCTL_STORAGE_CHECK_VERIFY | IOCTL_STORAGE_CHECK_VERIFY2 | IOCTL_DISK_CHECK_VERIFY => {
            if out_len >= 4 && !out8.is_null() {
                put_exact(&0u32.to_le_bytes())
            } else {
                put_exact(&[])
            }
        }
        IOCTL_STORAGE_GET_MEDIA_TYPES_EX => {
            let vol = query_volume_storage_info(&f);
            // GET_MEDIA_TYPES (8 byte header + 1 DEVICE_MEDIA_INFO = 32 bytes -> 40 bytes)
            let bps = vol.logical_sector_size.max(512);
            let cyls = (vol.volume_size_bytes / (bps as u64 * 255 * 63)).max(1);
            let mut d = [0u8; 40];
            d[0..4].copy_from_slice(&7u32.to_le_bytes()); // FILE_DEVICE_DISK
            d[4..8].copy_from_slice(&1u32.to_le_bytes()); // MediaInfoCount = 1
            d[8..16].copy_from_slice(&cyls.to_le_bytes());
            d[16..20].copy_from_slice(&12u32.to_le_bytes()); // FixedMedia
            d[20..24].copy_from_slice(&255u32.to_le_bytes());
            d[24..28].copy_from_slice(&63u32.to_le_bytes());
            d[28..32].copy_from_slice(&bps.to_le_bytes());
            d[32..36].copy_from_slice(&1u32.to_le_bytes()); // NumberMediaSides
            d[36..40].copy_from_slice(&(0x2u32 | if vol.writable { 0x4 } else { 0 }).to_le_bytes());
            put(&d)
        }
        IOCTL_STORAGE_GET_HOTPLUG_INFO => {
            let vol = query_volume_storage_info(&f);
            // STORAGE_HOTPLUG_INFO (8 bytes)
            let mut d = [0u8; 8];
            d[0..4].copy_from_slice(&8u32.to_le_bytes());
            d[4] = vol.removable as u8;
            d[5] = 0; // MediaHotplug
            d[6] = vol.removable as u8; // DeviceHotplug
            d[7] = 0; // WriteCacheEnableOverride
            put_exact(&d)
        }
        IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS => {
            let vol = query_volume_storage_info(&f);
            // VOLUME_DISK_EXTENTS { NumberOfDiskExtents=1, pad, DISK_EXTENT{ DiskNumber, pad, StartingOffset, ExtentLength } }
            let mut d = [0u8; 32];
            d[0..4].copy_from_slice(&1u32.to_le_bytes());
            d[8..12].copy_from_slice(&vol.disk_number.to_le_bytes());
            d[16..24].copy_from_slice(&vol.partition_offset.to_le_bytes());
            d[24..32].copy_from_slice(&vol.volume_size_bytes.to_le_bytes());
            put(&d)
        }
        IOCTL_VOLUME_IS_OFFLINE => {
            // Microsoft Learn: succeeds if the volume is offline; fails with ERROR_INVALID_FUNCTION if online.
            set_err(err::ERROR_INVALID_FUNCTION);
            FALSE
        }
        IOCTL_DISK_GET_DRIVE_GEOMETRY | IOCTL_DISK_GET_DRIVE_GEOMETRY_EX => {
            let vol = query_volume_storage_info(&f);
            let bps = vol.logical_sector_size.max(512);
            let cyls = (vol.volume_size_bytes / (bps as u64 * 255 * 63)).max(1);
            let mut d = [0u8; 32];
            d[0..8].copy_from_slice(&cyls.to_le_bytes());
            d[8..12].copy_from_slice(&(if vol.removable { 11u32 } else { 12u32 }).to_le_bytes());
            d[12..16].copy_from_slice(&255u32.to_le_bytes());
            d[16..20].copy_from_slice(&63u32.to_le_bytes());
            d[20..24].copy_from_slice(&bps.to_le_bytes());
            if code == IOCTL_DISK_GET_DRIVE_GEOMETRY {
                put_exact(&d[..24])
            } else {
                d[24..32].copy_from_slice(&(vol.volume_size_bytes as i64).to_le_bytes());
                put(&d)
            }
        }
        IOCTL_DISK_GET_LENGTH_INFO => {
            let len = if f.kind == FileKind::Disk {
                f.file().metadata().map(|m| m.len()).unwrap_or(0)
            } else {
                query_volume_storage_info(&f).volume_size_bytes
            };
            put_exact(&(len as i64).to_le_bytes())
        }
        IOCTL_DISK_IS_WRITABLE => {
            let vol = query_volume_storage_info(&f);
            if vol.writable {
                put_exact(&[])
            } else {
                set_err(err::ERROR_WRITE_PROTECT);
                FALSE
            }
        }
        IOCTL_DISK_GET_PARTITION_INFO_EX => {
            let vol = query_volume_storage_info(&f);
            // PARTITION_INFORMATION_EX (144 bytes): PartitionStyle=PARTITION_STYLE_GPT(1)
            let mut d = [0u8; 144];
            d[0..4].copy_from_slice(&1u32.to_le_bytes()); // PARTITION_STYLE_GPT
            d[8..16].copy_from_slice(&(vol.partition_offset as i64).to_le_bytes());
            d[16..24].copy_from_slice(&(vol.volume_size_bytes as i64).to_le_bytes());
            d[24..28].copy_from_slice(&vol.partition_number.to_le_bytes());
            d[28] = 0; // RewritePartition
            // GPT PartitionType GUID (Basic Data Partition: EBD0A0A2-B9E5-4433-87C0-68B6B72699C7) @32..48
            d[32..48].copy_from_slice(&[
                0xA2, 0xA0, 0xD0, 0xEB, 0xE5, 0xB9, 0x33, 0x44, 0x87, 0xC0, 0x68, 0xB6, 0xB7, 0x26, 0x99, 0xC7,
            ]);
            put_exact(&d)
        }
        FSCTL_GET_REPARSE_POINT => {
            let Some(ref p) = f.host_path else {
                set_err(ERROR_NOT_A_REPARSE_POINT);
                return FALSE;
            };
            let Ok(target) = std::fs::read_link(p) else {
                set_err(ERROR_NOT_A_REPARSE_POINT);
                return FALSE;
            };
            let (flags, sub_name, print_name) = if target.is_relative() {
                let rel = target.to_string_lossy().replace('/', "\\");
                (1u32, rel.clone(), rel) // SYMLINK_FLAG_RELATIVE
            } else {
                let win_target = path::to_win(&target)
                    .or_else(|| std::fs::canonicalize(&target).ok().and_then(|c| path::to_win(&c)));
                let Some(win_target) = win_target else {
                    set_err(ERROR_INVALID_REPARSE_DATA);
                    return FALSE;
                };
                (0u32, format!("\\??\\{win_target}"), win_target)
            };
            let sub_w = to_wide(&sub_name);
            let prt_w = to_wide(&print_name);
            let sub_bytes = sub_w.len() * 2;
            let prt_bytes = prt_w.len() * 2;
            let data_len = 12 + sub_bytes + prt_bytes;
            let mut d = vec![0u8; 8 + data_len];
            d[0..4].copy_from_slice(&IO_REPARSE_TAG_SYMLINK.to_le_bytes());
            d[4..6].copy_from_slice(&(data_len as u16).to_le_bytes());
            // SymbolicLinkReparseBuffer @8: SubstituteNameOffset(0), SubstituteNameLength, PrintNameOffset, PrintNameLength, Flags
            d[8..10].copy_from_slice(&0u16.to_le_bytes());
            d[10..12].copy_from_slice(&(sub_bytes as u16).to_le_bytes());
            d[12..14].copy_from_slice(&(sub_bytes as u16).to_le_bytes());
            d[14..16].copy_from_slice(&(prt_bytes as u16).to_le_bytes());
            d[16..20].copy_from_slice(&flags.to_le_bytes());
            for (i, &u) in sub_w.iter().enumerate() {
                d[20 + i * 2..22 + i * 2].copy_from_slice(&u.to_le_bytes());
            }
            for (i, &u) in prt_w.iter().enumerate() {
                d[20 + sub_bytes + i * 2..22 + sub_bytes + i * 2].copy_from_slice(&u.to_le_bytes());
            }
            put_exact(&d)
        }
        FSCTL_SET_SPARSE => {
            if f.kind != FileKind::Disk || f.is_protected_native() {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            // APFS/HFS+ natively support sparse holes without an explicit toggle.
            put_exact(&[])
        }
        FSCTL_QUERY_ALLOCATED_RANGES => {
            if inp.is_null() || in_len < 16 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let req_off = std::ptr::read_unaligned(inp as *const i64);
            let req_len = std::ptr::read_unaligned((inp as *const u8).add(8) as *const i64);
            if req_off < 0 || req_len < 0 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let file_len = f.file().metadata().map(|m| m.len() as i64).unwrap_or(0);
            let range_end = req_off.saturating_add(req_len).min(file_len);
            if req_off >= range_end {
                return put_exact(&[]);
            }
            // Query real allocated ranges via SEEK_DATA/SEEK_HOLE (preserving file offset).
            let saved_pos = libc::lseek(f.fd(), 0, libc::SEEK_CUR);
            let mut ranges: Vec<(i64, i64)> = Vec::new();
            let mut cur = req_off;
            while cur < range_end {
                let data_start = libc::lseek(f.fd(), cur, libc::SEEK_DATA);
                if data_start < 0 || data_start >= range_end {
                    break;
                }
                let hole_start = libc::lseek(f.fd(), data_start, libc::SEEK_HOLE);
                let end = if hole_start < 0 || hole_start > range_end { range_end } else { hole_start };
                if end > data_start {
                    ranges.push((data_start, end - data_start));
                }
                if hole_start < 0 || end >= range_end {
                    break;
                }
                cur = end;
            }
            if saved_pos >= 0 {
                libc::lseek(f.fd(), saved_pos, libc::SEEK_SET);
            }
            let max_entries = (out_len as usize) / 16;
            let count = ranges.len().min(max_entries);
            let mut buf = Vec::with_capacity(count * 16);
            for &(o, l) in &ranges[..count] {
                buf.extend_from_slice(&o.to_le_bytes());
                buf.extend_from_slice(&l.to_le_bytes());
            }
            if !buf.is_empty() && out8.is_null() {
                set_err(err::ERROR_INSUFFICIENT_BUFFER);
                return FALSE;
            }
            if !buf.is_empty() {
                std::ptr::copy_nonoverlapping(buf.as_ptr(), out8, buf.len());
            }
            if !ret.is_null() {
                std::ptr::write_unaligned(ret, buf.len() as u32);
            }
            if !ov.is_null() {
                complete_overlapped(ov, STATUS_SUCCESS, buf.len());
            }
            if count < ranges.len() {
                set_err(err::ERROR_MORE_DATA);
                return FALSE;
            }
            TRUE
        }
        FSCTL_SET_ZERO_DATA => {
            if inp.is_null() || in_len < 16 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            if f.is_protected_native() {
                set_err(err::ERROR_ACCESS_DENIED);
                return FALSE;
            }
            let start = std::ptr::read_unaligned(inp as *const i64);
            let beyond = std::ptr::read_unaligned((inp as *const u8).add(8) as *const i64);
            if start < 0 || beyond < start {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let file_len = f.file().metadata().map(|m| m.len() as i64).unwrap_or(0);
            let end = beyond.min(file_len);
            if end > start {
                let zeros = [0u8; 4096];
                let mut pos = start as u64;
                let end_u = end as u64;
                while pos < end_u {
                    let chunk = ((end_u - pos) as usize).min(zeros.len());
                    if let Err(e) = f.file().write_all_at(&zeros[..chunk], pos) {
                        set_err(io_err(&e));
                        return FALSE;
                    }
                    pos += chunk as u64;
                }
            }
            put_exact(&[])
        }
        FSCTL_GET_COMPRESSION => {
            // COMPRESSION_FORMAT_NONE (0)
            put_exact(&0u16.to_le_bytes())
        }
        FSCTL_SET_COMPRESSION => {
            if inp.is_null() || in_len < 2 {
                set_err(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let fmt = std::ptr::read_unaligned(inp as *const u16);
            if fmt == 0 {
                put_exact(&[])
            } else {
                set_err(err::ERROR_NOT_SUPPORTED);
                FALSE
            }
        }
        FSCTL_IS_VOLUME_MOUNTED => put_exact(&[]),
        FSCTL_GET_NTFS_VOLUME_DATA => {
            let vol = query_volume_storage_info(&f);
            let bps = vol.logical_sector_size.max(512) as u64;
            let bpc = vol.cluster_size.max(bps as u32) as u64;
            let total_sectors = vol.volume_size_bytes / bps;
            let total_clusters = vol.volume_size_bytes / bpc;
            let free_clusters = vol.volume_free_bytes / bpc;
            // NTFS_VOLUME_DATA_BUFFER (96 bytes)
            let mut d = [0u8; 96];
            d[0..8].copy_from_slice(&vol.volume_serial.to_le_bytes());
            d[8..16].copy_from_slice(&(total_sectors as i64).to_le_bytes());
            d[16..24].copy_from_slice(&(total_clusters as i64).to_le_bytes());
            d[24..32].copy_from_slice(&(free_clusters as i64).to_le_bytes());
            d[40..44].copy_from_slice(&(bps as u32).to_le_bytes());
            d[44..48].copy_from_slice(&(bpc as u32).to_le_bytes());
            d[48..52].copy_from_slice(&1024u32.to_le_bytes()); // BytesPerFileRecordSegment
            d[52..56].copy_from_slice(&0u32.to_le_bytes()); // ClustersPerFileRecordSegment
            put_exact(&d)
        }
        FSCTL_LOCK_VOLUME | FSCTL_UNLOCK_VOLUME => {
            // Locking/dismounting a mounted OS volume requires exclusive admin access on Windows.
            set_err(err::ERROR_ACCESS_DENIED);
            FALSE
        }
        _ => {
            // Windows I/O Manager returns STATUS_INVALID_DEVICE_REQUEST -> ERROR_INVALID_FUNCTION (1)
            // for control codes not handled by the target file/device driver.
            log::debug!("DeviceIoControl: unrecognized control code {code:#x} -> ERROR_INVALID_FUNCTION");
            set_err(err::ERROR_INVALID_FUNCTION);
            FALSE
        }
    }
}

fn fail_not_supported() -> BOOL {
    set_err(err::ERROR_NOT_SUPPORTED);
    FALSE
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wildcards() {
        assert!(wildcard_match("*.pak", "Data.PAK"));
        assert!(wildcard_match("*", "x"));
        assert!(wildcard_match("a?c", "ABC"));
        assert!(!wildcard_match("*.rmdp", "file.bin"));
        assert!(wildcard_match("foo.*", "foo"));
    }

    #[test]
    fn iocp_receives_overlapped_read_completion() {
        const FILE_FLAG_OVERLAPPED: u32 = 0x4000_0000;
        crate::path::add_mount("U:\\", std::path::Path::new("/"));
        unsafe {
            let tmp = std::env::temp_dir().join(format!("win32_rt_iocp_test_{}.bin", std::process::id()));
            std::fs::write(&tmp, b"iocp-payload").unwrap();
            let win_p = crate::strings::to_wide_z(&format!("U:{}", tmp.to_string_lossy().replace('/', "\\")));
            let h = CreateFileW(win_p.as_ptr(), GENERIC_READ, 7, std::ptr::null_mut(), OPEN_EXISTING, FILE_FLAG_OVERLAPPED, std::ptr::null_mut());
            assert_ne!(h, INVALID_HANDLE_VALUE);
            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());
            assert_eq!(SetFileCompletionNotificationModes(h, 0x2), TRUE);
            assert_eq!(CreateIoCompletionPort(h, iocp, 0xBEEF, 0), iocp);

            let mut ov: OVERLAPPED = std::mem::zeroed();
            let mut buf = [0u8; 12];
            let mut n = 0u32;
            assert_eq!(ReadFile(h, buf.as_mut_ptr() as *mut c_void, 12, &mut n, &mut ov), TRUE);
            assert_eq!(&buf, b"iocp-payload");

            let mut entry: OVERLAPPED_ENTRY = std::mem::zeroed();
            let mut removed = 0u32;
            assert_eq!(GetQueuedCompletionStatusEx(iocp, &mut entry, 1, &mut removed, 500, FALSE), TRUE);
            assert_eq!(removed, 1);
            assert_eq!(entry.lpCompletionKey, 0xBEEF);
            assert_eq!(entry.lpOverlapped, &mut ov as *mut OVERLAPPED);
            assert_eq!(entry.dwNumberOfBytesTransferred, 12);

            // Setting FILE_SKIP_COMPLETION_PORT_ON_SUCCESS (0x1) suppresses IOCP posting.
            assert_eq!(SetFileCompletionNotificationModes(h, 0x3), TRUE);
            let mut ov2: OVERLAPPED = std::mem::zeroed();
            assert_eq!(ReadFile(h, buf.as_mut_ptr() as *mut c_void, 12, &mut n, &mut ov2), TRUE);
            assert_eq!(GetQueuedCompletionStatusEx(iocp, &mut entry, 1, &mut removed, 0, FALSE), FALSE);

            crate::kernel32::threads::CloseHandle(h);
            crate::kernel32::threads::CloseHandle(iocp);
            let _ = std::fs::remove_file(&tmp);
        }
    }

    #[test]
    fn query_dos_device_and_drive_strings() {
        unsafe {
            let c_dev = to_wide_z("C:");
            let mut buf = [0u16; 64];
            let n = QueryDosDeviceW(c_dev.as_ptr(), buf.as_mut_ptr(), buf.len() as u32);
            assert!(n > 2);
            assert_eq!(from_wide(buf.as_ptr()), "\\Device\\HarddiskVolume1");
            assert_eq!(buf[n as usize - 1], 0);
            assert_eq!(buf[n as usize - 2], 0);

            let mut drives = [0u16; 64];
            let dn = GetLogicalDriveStringsW(drives.len() as u32, drives.as_mut_ptr());
            assert!(dn >= 4);
            let s = String::from_utf16_lossy(&drives[..dn as usize]);
            assert!(s.contains("C:\\\0"));
        }
    }

    fn parse_notify_records(buf: &[u8], bytes: usize) -> Vec<(u32, String)> {
        let mut out = Vec::new();
        if bytes == 0 {
            return out;
        }
        let mut off = 0usize;
        loop {
            assert!(off + 12 <= bytes);
            let next = u32::from_le_bytes(buf[off..off + 4].try_into().unwrap()) as usize;
            let action = u32::from_le_bytes(buf[off + 4..off + 8].try_into().unwrap());
            let name_len = u32::from_le_bytes(buf[off + 8..off + 12].try_into().unwrap()) as usize;
            assert!(off + 12 + name_len <= bytes);
            let u16s: Vec<u16> = buf[off + 12..off + 12 + name_len]
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            out.push((action, String::from_utf16_lossy(&u16s)));
            if next == 0 {
                break;
            }
            off += next;
        }
        out
    }

    #[test]
    fn read_write_file_ex_apc_issuing_thread_only_and_cancel() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::{AtomicU32, AtomicUsize};

        static APC_CALLS: AtomicU32 = AtomicU32::new(0);
        static APC_ERR: AtomicU32 = AtomicU32::new(u32::MAX);
        static APC_BYTES: AtomicU32 = AtomicU32::new(0);
        static APC_OV: AtomicUsize = AtomicUsize::new(0);
        static APC_TID: AtomicU32 = AtomicU32::new(0);

        unsafe extern "win64" fn completion_cb(err_code: u32, bytes: u32, ov: usize) {
            APC_ERR.store(err_code, Ordering::Release);
            APC_BYTES.store(bytes, Ordering::Release);
            APC_OV.store(ov, Ordering::Release);
            APC_TID.store(teb::thread_id(), Ordering::Release);
            APC_CALLS.fetch_add(1, Ordering::AcqRel);
        }

        APC_CALLS.store(0, Ordering::Release);
        crate::path::add_mount("U:\\", std::path::Path::new("/"));

        unsafe {
            // Validation errors
            assert_eq!(ReadFileEx(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, std::ptr::null_mut(), Some(completion_cb)), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
            let mut dummy_ov: OVERLAPPED = std::mem::zeroed();
            assert_eq!(ReadFileEx(INVALID_HANDLE_VALUE, &mut dummy_ov as *mut _ as *mut c_void, 4, &mut dummy_ov, Some(completion_cb)), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_HANDLE);

            let tmp = std::env::temp_dir().join(format!("win32_rt_rwex_{}.bin", std::process::id()));
            let _ = std::fs::remove_file(&tmp);
            let win_p = to_wide_z(&format!("U:{}", tmp.to_string_lossy().replace('/', "\\")));
            let h = CreateFileW(
                win_p.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                7,
                std::ptr::null_mut(),
                CREATE_ALWAYS,
                0x4000_0000, // FILE_FLAG_OVERLAPPED
                std::ptr::null_mut(),
            );
            assert_ne!(h, INVALID_HANDLE_VALUE);

            // Bind IOCP to prove ReadFileEx/WriteFileEx do NOT post to IOCP or signal ov.h_event!
            let iocp = CreateIoCompletionPort(h, std::ptr::null_mut(), 0xCAFE, 1);
            assert!(!iocp.is_null());
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());

            let issuing_tid = teb::thread_id();
            let payload = b"hello-ex-apc";
            let mut wov: OVERLAPPED = std::mem::zeroed();
            wov.h_event = ev;
            assert_eq!(
                WriteFileEx(h, payload.as_ptr() as *const c_void, payload.len() as u32, &mut wov, Some(completion_cb)),
                TRUE
            );
            // Must NOT have run yet before alertable wait, must NOT signal h_event or IOCP.
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 0);
            assert_eq!(WaitForSingleObject(ev, 0), WAIT_TIMEOUT);
            let mut entry: OVERLAPPED_ENTRY = std::mem::zeroed();
            let mut removed = 0u32;
            assert_eq!(GetQueuedCompletionStatusEx(iocp, &mut entry, 1, &mut removed, 0, FALSE), FALSE);

            // A DIFFERENT thread entering an alertable wait must NOT run our APC!
            std::thread::spawn(|| {
                let _ = crate::thread::ensure_current_thread_object();
                let r = SleepEx(10, TRUE);
                assert_eq!(r, 0, "non-issuing thread must not steal issuing thread's APC");
            })
            .join()
            .unwrap();
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 0);

            // Issuing thread enters alertable SleepEx -> APC runs on issuing thread!
            assert_eq!(SleepEx(500, TRUE), WAIT_IO_COMPLETION);
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(APC_BYTES.load(Ordering::Acquire), payload.len() as u32);
            assert_eq!(APC_OV.load(Ordering::Acquire), &mut wov as *mut OVERLAPPED as usize);
            assert_eq!(APC_TID.load(Ordering::Acquire), issuing_tid);
            assert_eq!(WaitForSingleObject(ev, 0), WAIT_TIMEOUT);

            // ReadFileEx at offset 0 -> delivers APC via alertable GetQueuedCompletionStatusEx!
            let mut rbuf = [0u8; 32];
            let mut rov: OVERLAPPED = std::mem::zeroed();
            rov.h_event = ev;
            assert_eq!(
                ReadFileEx(h, rbuf.as_mut_ptr() as *mut c_void, payload.len() as u32, &mut rov, Some(completion_cb)),
                TRUE
            );
            assert_eq!(GetQueuedCompletionStatusEx(iocp, &mut entry, 1, &mut removed, 500, TRUE), FALSE);
            assert_eq!(teb::last_error(), WAIT_IO_COMPLETION);
            assert_eq!(removed, 0);
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 2);
            assert_eq!(APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(APC_BYTES.load(Ordering::Acquire), payload.len() as u32);
            assert_eq!(&rbuf[..payload.len()], payload);

            // ReadFileEx past EOF on disk returns FALSE + ERROR_HANDLE_EOF.
            let mut eof_ov: OVERLAPPED = std::mem::zeroed();
            eof_ov.offset = 1024;
            assert_eq!(
                ReadFileEx(h, rbuf.as_mut_ptr() as *mut c_void, 8, &mut eof_ov, Some(completion_cb)),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_HANDLE_EOF);
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 2);

            // Pending ReadFileEx on an empty pipe cancelled by CancelIoEx delivers ERROR_OPERATION_ABORTED via APC!
            let mut prd: HANDLE = std::ptr::null_mut();
            let mut pwr: HANDLE = std::ptr::null_mut();
            assert_eq!(CreatePipe(&mut prd, &mut pwr, std::ptr::null_mut(), 0), TRUE);
            let mut pov: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                ReadFileEx(prd, rbuf.as_mut_ptr() as *mut c_void, 16, &mut pov, Some(completion_cb)),
                TRUE
            );
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 2);
            assert_eq!(CancelIoEx(prd, &mut pov), TRUE);
            assert_eq!(SleepEx(500, TRUE), WAIT_IO_COMPLETION);
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 3);
            assert_eq!(APC_ERR.load(Ordering::Acquire), ERROR_OPERATION_ABORTED);
            assert_eq!(APC_BYTES.load(Ordering::Acquire), 0);
            assert_eq!(APC_OV.load(Ordering::Acquire), &mut pov as *mut OVERLAPPED as usize);

            // Alertable GetQueuedCompletionStatusEx wakes up WHILE BLOCKED when an APC is queued from another thread!
            let iocp_usize = iocp as usize;
            let pwr_usize = pwr as usize;
            let mut pov2: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                ReadFileEx(prd, rbuf.as_mut_ptr() as *mut c_void, 5, &mut pov2, Some(completion_cb)),
                TRUE
            );
            let writer_th = std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(35));
                let mut w = 0u32;
                assert_eq!(WriteFile(pwr_usize as HANDLE, b"wake!".as_ptr() as *const c_void, 5, &mut w, std::ptr::null_mut()), TRUE);
            });
            let t0 = Instant::now();
            assert_eq!(GetQueuedCompletionStatusEx(iocp_usize as HANDLE, &mut entry, 1, &mut removed, 5000, TRUE), FALSE);
            let elapsed = t0.elapsed();
            writer_th.join().unwrap();
            assert_eq!(teb::last_error(), WAIT_IO_COMPLETION);
            assert!(elapsed < Duration::from_secs(2), "alertable GQCSEx should wake immediately on APC, took {elapsed:?}");
            assert_eq!(APC_CALLS.load(Ordering::Acquire), 4);
            assert_eq!(APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(APC_BYTES.load(Ordering::Acquire), 5);
            assert_eq!(&rbuf[..5], b"wake!");

            assert_eq!(CloseHandle(prd), TRUE);
            assert_eq!(CloseHandle(pwr), TRUE);
            assert_eq!(CloseHandle(ev), TRUE);
            assert_eq!(CloseHandle(iocp), TRUE);
            assert_eq!(CloseHandle(h), TRUE);
            let _ = std::fs::remove_file(&tmp);
        }
    }

    #[test]
    fn read_directory_changes_all_modes_subtree_and_cancel() {
        use crate::kernel32::threadpool::{
            CloseThreadpoolIo, CreateThreadpoolIo, StartThreadpoolIo, WaitForThreadpoolIoCallbacks,
        };
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::{AtomicU32, AtomicUsize};

        crate::path::add_mount("U:\\", std::path::Path::new("/"));
        let dir = std::env::temp_dir().join(format!("win32_rt_dirwatch_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        let win_dir = to_wide_z(&format!("U:{}", dir.to_string_lossy().replace('/', "\\")));
        let all_filter = FILE_NOTIFY_CHANGE_FILE_NAME
            | FILE_NOTIFY_CHANGE_DIR_NAME
            | FILE_NOTIFY_CHANGE_SIZE
            | FILE_NOTIFY_CHANGE_LAST_WRITE
            | FILE_NOTIFY_CHANGE_ATTRIBUTES;

        let open_dir = || unsafe {
            let h = CreateFileW(
                win_dir.as_ptr(),
                FILE_READ_DATA,
                7,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS | 0x4000_0000,
                std::ptr::null_mut(),
            );
            assert_ne!(h, INVALID_HANDLE_VALUE);
            h
        };

        unsafe {
            // 1. Synchronous mode: create, modify, rename, delete.
            let h_sync = open_dir();
            let dir_c = dir.clone();
            let mut sync_buf = [0u8; 4096];
            let mut sync_ret = 0u32;
            let mutater = std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(30));
                std::fs::write(dir_c.join("alpha.txt"), b"v1").unwrap();
            });
            assert_eq!(
                ReadDirectoryChangesW(
                    h_sync,
                    sync_buf.as_mut_ptr() as *mut c_void,
                    sync_buf.len() as u32,
                    FALSE,
                    all_filter,
                    &mut sync_ret,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                ),
                TRUE
            );
            mutater.join().unwrap();
            let recs = parse_notify_records(&sync_buf, sync_ret as usize);
            assert!(recs.iter().any(|(a, p)| *a == FILE_ACTION_ADDED && p == "alpha.txt"), "got {recs:?}");

            // 2. Overlapped + hEvent: modify, rename, delete.
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            let mut ov: OVERLAPPED = std::mem::zeroed();
            ov.h_event = ev;
            let mut ev_buf = [0u8; 4096];
            assert_eq!(
                ReadDirectoryChangesW(
                    h_sync,
                    ev_buf.as_mut_ptr() as *mut c_void,
                    ev_buf.len() as u32,
                    FALSE,
                    all_filter,
                    std::ptr::null_mut(),
                    &mut ov,
                    std::ptr::null(),
                ),
                TRUE
            );
            std::fs::write(dir.join("alpha.txt"), b"v2-longer").unwrap();
            assert_eq!(WaitForSingleObject(ev, 3000), WAIT_OBJECT_0);
            let mut n = 0u32;
            assert_eq!(GetOverlappedResult(h_sync, &mut ov, &mut n, FALSE), TRUE);
            let recs = parse_notify_records(&ev_buf, n as usize);
            assert!(recs.iter().any(|(a, p)| *a == FILE_ACTION_MODIFIED && p == "alpha.txt"), "got {recs:?}");

            // Rename alpha.txt -> beta.txt
            // Note: `std::fs::write` above does open(O_TRUNC) + write(), which can legitimately
            // produce a trailing FILE_ACTION_MODIFIED on alpha.txt via EVFILT_VNODE before the
            // rename event arrives; collect batches until the rename pair is delivered.
            let mut rename_recs: Vec<(u32, String)> = Vec::new();
            let mut renamed = false;
            for attempt in 0..4 {
                assert_eq!(
                    ReadDirectoryChangesW(
                        h_sync,
                        ev_buf.as_mut_ptr() as *mut c_void,
                        ev_buf.len() as u32,
                        FALSE,
                        all_filter,
                        std::ptr::null_mut(),
                        &mut ov,
                        std::ptr::null(),
                    ),
                    TRUE
                );
                if attempt == 0 {
                    std::fs::rename(dir.join("alpha.txt"), dir.join("beta.txt")).unwrap();
                }
                assert_eq!(WaitForSingleObject(ev, 3000), WAIT_OBJECT_0);
                assert_eq!(GetOverlappedResult(h_sync, &mut ov, &mut n, FALSE), TRUE);
                let batch = parse_notify_records(&ev_buf, n as usize);
                rename_recs.extend(batch);
                if rename_recs.iter().any(|(a, p)| *a == FILE_ACTION_RENAMED_OLD_NAME && p == "alpha.txt")
                    && rename_recs.iter().any(|(a, p)| *a == FILE_ACTION_RENAMED_NEW_NAME && p == "beta.txt")
                {
                    renamed = true;
                    break;
                }
                assert!(
                    rename_recs.iter().all(|(a, p)| *a == FILE_ACTION_MODIFIED && p == "alpha.txt"),
                    "unexpected non-modify record before rename: {rename_recs:?}"
                );
            }
            assert!(renamed, "expected rename pair, got {rename_recs:?}");

            // Delete beta.txt
            assert_eq!(
                ReadDirectoryChangesW(
                    h_sync,
                    ev_buf.as_mut_ptr() as *mut c_void,
                    ev_buf.len() as u32,
                    FALSE,
                    all_filter,
                    std::ptr::null_mut(),
                    &mut ov,
                    std::ptr::null(),
                ),
                TRUE
            );
            std::fs::remove_file(dir.join("beta.txt")).unwrap();
            assert_eq!(WaitForSingleObject(ev, 3000), WAIT_OBJECT_0);
            assert_eq!(GetOverlappedResult(h_sync, &mut ov, &mut n, FALSE), TRUE);
            let recs = parse_notify_records(&ev_buf, n as usize);
            assert!(recs.iter().any(|(a, p)| *a == FILE_ACTION_REMOVED && p == "beta.txt"), "got {recs:?}");

            // Non-subtree must NOT trigger on sub/nested.txt!
            assert_eq!(
                ReadDirectoryChangesW(
                    h_sync,
                    ev_buf.as_mut_ptr() as *mut c_void,
                    ev_buf.len() as u32,
                    FALSE,
                    all_filter,
                    std::ptr::null_mut(),
                    &mut ov,
                    std::ptr::null(),
                ),
                TRUE
            );
            std::fs::write(dir.join("sub").join("nested.txt"), b"hi").unwrap();
            assert_eq!(WaitForSingleObject(ev, 120), WAIT_TIMEOUT, "non-subtree watch must ignore sub/nested.txt");
            // CancelIoEx cancels pending watch -> ERROR_OPERATION_ABORTED exactly once.
            assert_eq!(CancelIoEx(h_sync, &mut ov), TRUE);
            assert_eq!(WaitForSingleObject(ev, 1000), WAIT_OBJECT_0);
            assert_eq!(GetOverlappedResult(h_sync, &mut ov, &mut n, FALSE), FALSE);
            assert_eq!(teb::last_error(), ERROR_OPERATION_ABORTED);
            assert_eq!(CloseHandle(h_sync), TRUE);
            assert_eq!(CloseHandle(ev), TRUE);

            // 3. Overlapped + IOCP + subtree watching + CancelIoEx exactly-once check.
            let h_iocp = open_dir();
            let iocp = CreateIoCompletionPort(h_iocp, std::ptr::null_mut(), 0xD1A0, 1);
            assert!(!iocp.is_null());
            let mut iocp_buf = [0u8; 4096];
            let mut ov_i: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                ReadDirectoryChangesW(
                    h_iocp,
                    iocp_buf.as_mut_ptr() as *mut c_void,
                    iocp_buf.len() as u32,
                    TRUE,
                    all_filter,
                    std::ptr::null_mut(),
                    &mut ov_i,
                    std::ptr::null(),
                ),
                TRUE
            );
            std::fs::write(dir.join("sub").join("deep.txt"), b"sub").unwrap();
            let mut entry: OVERLAPPED_ENTRY = std::mem::zeroed();
            let mut removed = 0u32;
            assert_eq!(GetQueuedCompletionStatusEx(iocp, &mut entry, 1, &mut removed, 3000, FALSE), TRUE);
            assert_eq!(removed, 1);
            assert_eq!(entry.lpCompletionKey, 0xD1A0);
            let recs = parse_notify_records(&iocp_buf, entry.dwNumberOfBytesTransferred as usize);
            assert!(recs.iter().any(|(a, p)| *a == FILE_ACTION_ADDED && p == "sub\\deep.txt"), "got {recs:?}");

            // Queue another watch, then CancelIoEx + CloseHandle: must get EXACTLY ONE cancelled packet!
            let mut ov_c: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                ReadDirectoryChangesW(
                    h_iocp,
                    iocp_buf.as_mut_ptr() as *mut c_void,
                    iocp_buf.len() as u32,
                    TRUE,
                    all_filter,
                    std::ptr::null_mut(),
                    &mut ov_c,
                    std::ptr::null(),
                ),
                TRUE
            );
            assert_eq!(CancelIoEx(h_iocp, &mut ov_c), TRUE);
            assert_eq!(CloseHandle(h_iocp), TRUE);
            let mut bytes_c = 0u32;
            let mut key_c = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes_c, &mut key_c, &mut ov_out, 2000), FALSE);
            assert_eq!(teb::last_error(), ERROR_OPERATION_ABORTED);
            assert_eq!(ov_out, &mut ov_c as *mut OVERLAPPED);
            // No duplicate second completion packet!
            assert_eq!(GetQueuedCompletionStatusEx(iocp, &mut entry, 1, &mut removed, 50, FALSE), FALSE);
            assert_eq!(teb::last_error(), WAIT_TIMEOUT);
            assert_eq!(CloseHandle(iocp), TRUE);

            // 4. Overlapped + APC completion routine.
            static DIR_APC_CALLS: AtomicU32 = AtomicU32::new(0);
            static DIR_APC_BYTES: AtomicU32 = AtomicU32::new(0);
            unsafe extern "win64" fn dir_apc_cb(err_code: u32, bytes: u32, _ov: usize) {
                if err_code == 0 {
                    DIR_APC_BYTES.store(bytes, Ordering::Release);
                }
                DIR_APC_CALLS.fetch_add(1, Ordering::AcqRel);
            }
            DIR_APC_CALLS.store(0, Ordering::Release);
            let h_apc = open_dir();
            let mut apc_buf = [0u8; 4096];
            let mut ov_a: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                ReadDirectoryChangesW(
                    h_apc,
                    apc_buf.as_mut_ptr() as *mut c_void,
                    apc_buf.len() as u32,
                    FALSE,
                    all_filter,
                    std::ptr::null_mut(),
                    &mut ov_a,
                    dir_apc_cb as *const c_void,
                ),
                TRUE
            );
            std::fs::write(dir.join("apc_file.txt"), b"x").unwrap();
            assert_eq!(SleepEx(3000, TRUE), WAIT_IO_COMPLETION);
            assert_eq!(DIR_APC_CALLS.load(Ordering::Acquire), 1);
            let recs = parse_notify_records(&apc_buf, DIR_APC_BYTES.load(Ordering::Acquire) as usize);
            assert!(recs.iter().any(|(a, p)| *a == FILE_ACTION_ADDED && p == "apc_file.txt"), "got {recs:?}");
            assert_eq!(CloseHandle(h_apc), TRUE);

            // 5. Overlapped + CreateThreadpoolIo.
            static DIR_TP_CALLS: AtomicU32 = AtomicU32::new(0);
            static DIR_TP_BYTES: AtomicUsize = AtomicUsize::new(0);
            unsafe extern "win64" fn dir_tp_cb(
                _inst: *mut c_void,
                _ctx: *mut c_void,
                _ov: *mut c_void,
                io_res: u32,
                bytes: usize,
                _pio: *mut c_void,
            ) {
                if io_res == 0 {
                    DIR_TP_BYTES.store(bytes, Ordering::Release);
                }
                DIR_TP_CALLS.fetch_add(1, Ordering::AcqRel);
            }
            DIR_TP_CALLS.store(0, Ordering::Release);
            let h_tp = open_dir();
            let pio = CreateThreadpoolIo(h_tp, dir_tp_cb as *const c_void, std::ptr::null_mut(), std::ptr::null_mut());
            assert!(!pio.is_null());
            let mut tp_buf = [0u8; 4096];
            let mut ov_tp: OVERLAPPED = std::mem::zeroed();
            StartThreadpoolIo(pio);
            assert_eq!(
                ReadDirectoryChangesW(
                    h_tp,
                    tp_buf.as_mut_ptr() as *mut c_void,
                    tp_buf.len() as u32,
                    FALSE,
                    all_filter,
                    std::ptr::null_mut(),
                    &mut ov_tp,
                    std::ptr::null(),
                ),
                TRUE
            );
            std::fs::write(dir.join("tp_file.txt"), b"y").unwrap();
            let dl = Instant::now() + Duration::from_secs(3);
            while DIR_TP_CALLS.load(Ordering::Acquire) == 0 && Instant::now() < dl {
                std::thread::sleep(Duration::from_millis(5));
            }
            WaitForThreadpoolIoCallbacks(pio, FALSE);
            assert_eq!(DIR_TP_CALLS.load(Ordering::Acquire), 1);
            let recs = parse_notify_records(&tp_buf, DIR_TP_BYTES.load(Ordering::Acquire));
            assert!(recs.iter().any(|(a, p)| *a == FILE_ACTION_ADDED && p == "tp_file.txt"), "got {recs:?}");
            CloseThreadpoolIo(pio);
            assert_eq!(CloseHandle(h_tp), TRUE);

            // 6. FindFirstChangeNotificationW / FindNextChangeNotification / FindCloseChangeNotification.
            let ch = FindFirstChangeNotificationW(win_dir.as_ptr(), FALSE, FILE_NOTIFY_CHANGE_FILE_NAME);
            assert_ne!(ch, INVALID_HANDLE_VALUE);
            assert_eq!(WaitForSingleObject(ch, 0), WAIT_TIMEOUT);
            std::fs::write(dir.join("notif1.txt"), b"1").unwrap();
            assert_eq!(WaitForSingleObject(ch, 3000), WAIT_OBJECT_0);
            assert_eq!(FindNextChangeNotification(ch), TRUE);
            assert_eq!(WaitForSingleObject(ch, 0), WAIT_TIMEOUT);
            std::fs::write(dir.join("notif2.txt"), b"2").unwrap();
            assert_eq!(WaitForSingleObject(ch, 3000), WAIT_OBJECT_0);
            assert_eq!(FindCloseChangeNotification(ch), TRUE);

            let _ = std::fs::remove_dir_all(&dir);
        }
    }

    #[test]
    fn pipes_inherit_peek_named_pipe_and_overlapped_connect() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};

        #[repr(C)]
        struct SecAttr {
            len: u32,
            _pad: u32,
            sd: usize,
            inherit: BOOL,
        }

        unsafe {
            // 1. CreatePipe honors SECURITY_ATTRIBUTES.bInheritHandle and sets FD_CLOEXEC.
            let mut sa = SecAttr { len: 24, _pad: 0, sd: 0, inherit: TRUE };
            let mut rd: HANDLE = std::ptr::null_mut();
            let mut wr: HANDLE = std::ptr::null_mut();
            assert_eq!(CreatePipe(&mut rd, &mut wr, &mut sa as *mut _ as *mut c_void, 0), TRUE);
            assert_eq!(handles::get_flags(rd), Some(handles::HANDLE_FLAG_INHERIT));
            assert_eq!(handles::get_flags(wr), Some(handles::HANDLE_FLAG_INHERIT));
            let (rfd, _) = file_fd_and_kind(rd).unwrap();
            let (wfd, _) = file_fd_and_kind(wr).unwrap();
            assert_ne!(libc::fcntl(rfd, libc::F_GETFD) & libc::FD_CLOEXEC, 0);
            assert_ne!(libc::fcntl(wfd, libc::F_GETFD) & libc::FD_CLOEXEC, 0);

            // 2. PeekNamedPipe copies data non-destructively and subsequent ReadFile gets full payload.
            let payload = b"peek-test-payload";
            let mut w = 0u32;
            assert_eq!(WriteFile(wr, payload.as_ptr() as *const c_void, payload.len() as u32, &mut w, std::ptr::null_mut()), TRUE);
            assert_eq!(w as usize, payload.len());
            let mut pbuf = [0u8; 4];
            let mut pread = 0u32;
            let mut pavail = 0u32;
            let mut pleft = 0u32;
            assert_eq!(
                PeekNamedPipe(rd, pbuf.as_mut_ptr() as *mut c_void, 4, &mut pread, &mut pavail, &mut pleft),
                TRUE
            );
            assert_eq!(pread, 4);
            assert_eq!(pavail as usize, payload.len());
            assert_eq!(pleft as usize, payload.len() - 4);
            assert_eq!(&pbuf, b"peek");

            // ReadFile must still return the full payload starting with "peek"!
            let mut rbuf = [0u8; 32];
            let mut r1 = 0u32;
            assert_eq!(ReadFile(rd, rbuf.as_mut_ptr() as *mut c_void, 32, &mut r1, std::ptr::null_mut()), TRUE);
            let mut total = rbuf[..r1 as usize].to_vec();
            while total.len() < payload.len() {
                let mut r2 = 0u32;
                assert_eq!(ReadFile(rd, rbuf.as_mut_ptr() as *mut c_void, 32, &mut r2, std::ptr::null_mut()), TRUE);
                total.extend_from_slice(&rbuf[..r2 as usize]);
            }
            assert_eq!(&total, payload);

            // Closing write end -> PeekNamedPipe & ReadFile report ERROR_BROKEN_PIPE.
            assert_eq!(CloseHandle(wr), TRUE);
            assert_eq!(PeekNamedPipe(rd, std::ptr::null_mut(), 0, std::ptr::null_mut(), &mut pavail, std::ptr::null_mut()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_BROKEN_PIPE);
            assert_eq!(ReadFile(rd, rbuf.as_mut_ptr() as *mut c_void, 8, &mut r1, std::ptr::null_mut()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_BROKEN_PIPE);
            assert_eq!(CloseHandle(rd), TRUE);

            // 3. Named pipe server/client + overlapped ConnectNamedPipe + bidirectional I/O + DisconnectNamedPipe + sync ConnectNamedPipe.
            let pname = to_wide_z(&format!("\\\\.\\pipe\\win32_rt_test_{}", std::process::id()));
            let srv = CreateNamedPipeW(
                pname.as_ptr(),
                0x4000_0003, // PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED
                0,
                2,
                4096,
                4096,
                1000,
                std::ptr::null_mut(),
            );
            assert_ne!(srv, INVALID_HANDLE_VALUE);
            assert_eq!(WaitNamedPipeW(pname.as_ptr(), 100), TRUE);

            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            let mut cov: OVERLAPPED = std::mem::zeroed();
            cov.h_event = ev;
            assert_eq!(ConnectNamedPipe(srv, &mut cov), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);
            assert_eq!(WaitForSingleObject(ev, 0), WAIT_TIMEOUT);

            let cli = CreateFileW(
                pname.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                0,
                std::ptr::null_mut(),
            );
            assert_ne!(cli, INVALID_HANDLE_VALUE);
            assert_eq!(WaitForSingleObject(ev, 2000), WAIT_OBJECT_0);
            let mut n = 0u32;
            assert_eq!(GetOverlappedResult(srv, &mut cov, &mut n, FALSE), TRUE);

            // Client -> Server & Server -> Client
            assert_eq!(WriteFile(cli, b"ping".as_ptr() as *const c_void, 4, &mut w, std::ptr::null_mut()), TRUE);
            let mut sbuf = [0u8; 8];
            assert_eq!(ReadFile(srv, sbuf.as_mut_ptr() as *mut c_void, 8, &mut r1, std::ptr::null_mut()), TRUE);
            assert_eq!(&sbuf[..r1 as usize], b"ping");
            assert_eq!(WriteFile(srv, b"pong".as_ptr() as *const c_void, 4, &mut w, std::ptr::null_mut()), TRUE);
            assert_eq!(ReadFile(cli, sbuf.as_mut_ptr() as *mut c_void, 8, &mut r1, std::ptr::null_mut()), TRUE);
            assert_eq!(&sbuf[..r1 as usize], b"pong");

            // DisconnectNamedPipe closes server side -> client gets ERROR_BROKEN_PIPE.
            assert_eq!(DisconnectNamedPipe(srv), TRUE);
            assert_eq!(ReadFile(cli, sbuf.as_mut_ptr() as *mut c_void, 8, &mut r1, std::ptr::null_mut()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_BROKEN_PIPE);
            assert_eq!(CloseHandle(cli), TRUE);

            // Synchronous ConnectNamedPipe on a second client from another thread.
            let pname2 = pname.clone();
            let cli_th = std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(25));
                let c = CreateFileW(
                    pname2.as_ptr(),
                    GENERIC_READ | GENERIC_WRITE,
                    0,
                    std::ptr::null_mut(),
                    OPEN_EXISTING,
                    0,
                    std::ptr::null_mut(),
                );
                assert_ne!(c, INVALID_HANDLE_VALUE);
                c as usize
            });
            assert_eq!(ConnectNamedPipe(srv, std::ptr::null_mut()), TRUE);
            let cli2 = cli_th.join().unwrap() as HANDLE;
            assert_eq!(CloseHandle(cli2), TRUE);
            assert_eq!(DisconnectNamedPipe(srv), TRUE);

            // Overlapped ConnectNamedPipe cancelled via CancelIoEx.
            assert_eq!(ConnectNamedPipe(srv, &mut cov), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);
            assert_eq!(CancelIoEx(srv, &mut cov), TRUE);
            assert_eq!(WaitForSingleObject(ev, 1000), WAIT_OBJECT_0);
            assert_eq!(GetOverlappedResult(srv, &mut cov, &mut n, FALSE), FALSE);
            assert_eq!(teb::last_error(), ERROR_OPERATION_ABORTED);

            assert_eq!(CloseHandle(ev), TRUE);
            assert_eq!(CloseHandle(srv), TRUE);
        }
    }

    #[test]
    fn console_ctrl_handler_lifo_ignore_and_removal() {
        static LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

        unsafe extern "win64" fn h_first(ev: u32) -> BOOL {
            LOG.lock().unwrap().push((1, ev));
            TRUE // stops propagation
        }
        unsafe extern "win64" fn h_second(ev: u32) -> BOOL {
            LOG.lock().unwrap().push((2, ev));
            FALSE // continue to next handler
        }
        unsafe extern "win64" fn h_unregistered(_ev: u32) -> BOOL {
            TRUE
        }

        unsafe {
            LOG.lock().unwrap().clear();
            // Reset ignore flag and register two handlers.
            assert_eq!(SetConsoleCtrlHandler(std::ptr::null(), FALSE), TRUE);
            assert_eq!(SetConsoleCtrlHandler(h_first as *const c_void, TRUE), TRUE);
            assert_eq!(SetConsoleCtrlHandler(h_second as *const c_void, TRUE), TRUE);

            // LIFO: h_second runs first (returns FALSE), then h_first (returns TRUE).
            assert_eq!(GenerateConsoleCtrlEvent(0, 0), TRUE);
            assert_eq!(*LOG.lock().unwrap(), vec![(2, 0), (1, 0)]);

            // SetConsoleCtrlHandler(NULL, TRUE) ignores CTRL_C_EVENT (0) but still delivers CTRL_BREAK_EVENT (1).
            LOG.lock().unwrap().clear();
            assert_eq!(SetConsoleCtrlHandler(std::ptr::null(), TRUE), TRUE);
            assert_eq!(GenerateConsoleCtrlEvent(0, 0), TRUE);
            assert!(LOG.lock().unwrap().is_empty());
            assert_eq!(GenerateConsoleCtrlEvent(1, 0), TRUE);
            assert_eq!(*LOG.lock().unwrap(), vec![(2, 1), (1, 1)]);

            // Re-enable CTRL_C_EVENT and remove h_second.
            assert_eq!(SetConsoleCtrlHandler(std::ptr::null(), FALSE), TRUE);
            assert_eq!(SetConsoleCtrlHandler(h_second as *const c_void, FALSE), TRUE);
            assert_eq!(SetConsoleCtrlHandler(h_unregistered as *const c_void, FALSE), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);

            LOG.lock().unwrap().clear();
            assert_eq!(GenerateConsoleCtrlEvent(0, 0), TRUE);
            assert_eq!(*LOG.lock().unwrap(), vec![(1, 0)]);

            // Clean up.
            assert_eq!(SetConsoleCtrlHandler(h_first as *const c_void, FALSE), TRUE);
        }
    }

    #[test]
    fn non_tty_dev_null_is_not_console_and_console_input_events_validates_handle() {
        unsafe {
            let null_f = std::fs::File::open("/dev/null").expect("open /dev/null");
            assert_eq!(classify_host_fd(null_f.as_raw_fd()), FileKind::Null);
            let h_null = wrap_non_owned_fd(null_f.as_raw_fd(), classify_host_fd(null_f.as_raw_fd()), "NUL".into(), GENERIC_READ);
            assert_eq!(GetFileType(h_null), FILE_TYPE_CHAR);
            let mut mode = 0u32;
            assert_eq!(GetConsoleMode(h_null, &mut mode), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_HANDLE);
            let mut count = 99u32;
            assert_eq!(GetNumberOfConsoleInputEvents(h_null, &mut count), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_HANDLE);

            // Write-only console handle (CONOUT$) is rejected for input event count.
            let h_conout = wrap_non_owned_fd(null_f.as_raw_fd(), FileKind::Console, "CONOUT$".into(), GENERIC_WRITE);
            assert_eq!(GetConsoleMode(h_conout, &mut mode), TRUE);
            assert_eq!(GetNumberOfConsoleInputEvents(h_conout, &mut count), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_HANDLE);

            // Null output pointer returns ERROR_INVALID_PARAMETER.
            let h_conin = wrap_non_owned_fd(null_f.as_raw_fd(), FileKind::Console, "CONIN$".into(), GENERIC_READ);
            assert_eq!(GetNumberOfConsoleInputEvents(h_conin, std::ptr::null_mut()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Readable console handle succeeds.
            assert_eq!(GetNumberOfConsoleInputEvents(h_conin, &mut count), TRUE);
            assert_eq!(count, 0);

            assert_eq!(crate::kernel32::threads::CloseHandle(h_conin), TRUE);
            assert_eq!(crate::kernel32::threads::CloseHandle(h_conout), TRUE);
            assert_eq!(crate::kernel32::threads::CloseHandle(h_null), TRUE);
        }
    }

    #[test]
    fn overlapped_pipe_write_backpressure_full_transfer_writefile_and_writefileex() {
        use crate::kernel32::threads::CloseHandle;
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::{AtomicU32, AtomicUsize};

        // 512 KiB easily exceeds macOS's 64 KiB kernel pipe buffer, forcing partial non-blocking
        // writes and multiple POLLOUT wakeups in the pipe reactor.
        const PAYLOAD_LEN: usize = 512 * 1024;
        let payload: Vec<u8> = (0..PAYLOAD_LEN).map(|i| ((i.wrapping_mul(31) ^ (i >> 7)) & 0xFF) as u8).collect();

        unsafe {
            // 1. Overlapped WriteFile + IOCP: must return ERROR_IO_PENDING on backpressure and
            // complete only once ALL 512 KiB have been written intact.
            let (mut rd, mut wr) = (std::ptr::null_mut(), std::ptr::null_mut());
            assert_eq!(CreatePipe(&mut rd, &mut wr, std::ptr::null_mut(), 0), TRUE);
            let iocp = CreateIoCompletionPort(wr, std::ptr::null_mut(), 0x5120, 1);
            assert!(!iocp.is_null());

            let mut ov: OVERLAPPED = std::mem::zeroed();
            let mut immediate = 0u32;
            let rc = WriteFile(wr, payload.as_ptr() as *const c_void, PAYLOAD_LEN as u32, &mut immediate, &mut ov);
            assert_eq!(rc, FALSE, "512 KiB pipe write should go pending under backpressure");
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            let rd_addr = rd as usize;
            let reader = std::thread::spawn(move || {
                let mut acc = Vec::with_capacity(PAYLOAD_LEN);
                let mut chunk = [0u8; 4096];
                while acc.len() < PAYLOAD_LEN {
                    let mut n = 0u32;
                    let ok = ReadFile(rd_addr as HANDLE, chunk.as_mut_ptr() as *mut c_void, chunk.len() as u32, &mut n, std::ptr::null_mut());
                    assert_eq!(ok, TRUE);
                    assert!(n > 0);
                    acc.extend_from_slice(&chunk[..n as usize]);
                }
                acc
            });

            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 5000), TRUE);
            assert_eq!(key, 0x5120);
            assert_eq!(ov_out, &mut ov as *mut OVERLAPPED);
            assert_eq!(bytes as usize, PAYLOAD_LEN);
            let mut res_bytes = 0u32;
            assert_eq!(GetOverlappedResult(wr, &mut ov, &mut res_bytes, FALSE), TRUE);
            assert_eq!(res_bytes as usize, PAYLOAD_LEN);

            let received = reader.join().unwrap();
            assert_eq!(received, payload, "overlapped WriteFile must transfer all 512 KiB intact");
            assert_eq!(CloseHandle(iocp), TRUE);
            assert_eq!(CloseHandle(rd), TRUE);
            assert_eq!(CloseHandle(wr), TRUE);

            // 2. WriteFileEx + APC completion routine under the same 512 KiB backpressure.
            static WFEX_CALLS: AtomicU32 = AtomicU32::new(0);
            static WFEX_ERR: AtomicU32 = AtomicU32::new(u32::MAX);
            static WFEX_BYTES: AtomicU32 = AtomicU32::new(0);
            static WFEX_OV: AtomicUsize = AtomicUsize::new(0);

            unsafe extern "win64" fn wfex_cb(err_code: u32, transferred: u32, ov_addr: usize) {
                WFEX_ERR.store(err_code, Ordering::Release);
                WFEX_BYTES.store(transferred, Ordering::Release);
                WFEX_OV.store(ov_addr, Ordering::Release);
                WFEX_CALLS.fetch_add(1, Ordering::AcqRel);
            }

            WFEX_CALLS.store(0, Ordering::Release);
            WFEX_ERR.store(u32::MAX, Ordering::Release);
            WFEX_BYTES.store(0, Ordering::Release);
            WFEX_OV.store(0, Ordering::Release);

            let (mut rd2, mut wr2) = (std::ptr::null_mut(), std::ptr::null_mut());
            assert_eq!(CreatePipe(&mut rd2, &mut wr2, std::ptr::null_mut(), 0), TRUE);
            let mut ov2: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                WriteFileEx(wr2, payload.as_ptr() as *const c_void, PAYLOAD_LEN as u32, &mut ov2, Some(wfex_cb)),
                TRUE
            );
            assert_eq!(load_ov_internal(&mut ov2), STATUS_PENDING);

            let rd2_addr = rd2 as usize;
            let reader2 = std::thread::spawn(move || {
                let mut acc = Vec::with_capacity(PAYLOAD_LEN);
                let mut chunk = [0u8; 4096];
                while acc.len() < PAYLOAD_LEN {
                    let mut n = 0u32;
                    let ok = ReadFile(rd2_addr as HANDLE, chunk.as_mut_ptr() as *mut c_void, chunk.len() as u32, &mut n, std::ptr::null_mut());
                    assert_eq!(ok, TRUE);
                    assert!(n > 0);
                    acc.extend_from_slice(&chunk[..n as usize]);
                }
                acc
            });

            assert_eq!(SleepEx(5000, TRUE), WAIT_IO_COMPLETION);
            assert_eq!(WFEX_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(WFEX_ERR.load(Ordering::Acquire), 0);
            assert_eq!(WFEX_BYTES.load(Ordering::Acquire) as usize, PAYLOAD_LEN);
            assert_eq!(WFEX_OV.load(Ordering::Acquire), &mut ov2 as *mut OVERLAPPED as usize);

            let received2 = reader2.join().unwrap();
            assert_eq!(received2, payload, "WriteFileEx must transfer all 512 KiB intact");
            assert_eq!(CloseHandle(rd2), TRUE);
            assert_eq!(CloseHandle(wr2), TRUE);
        }
    }

    #[test]
    fn pipe_cancel_serialized_with_reactor_and_reentry_exactly_once() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::{AtomicU32, AtomicUsize};

        static REENTRY_HANDLE: AtomicUsize = AtomicUsize::new(0);
        static REENTRY_CALLS: AtomicU32 = AtomicU32::new(0);
        static REENTRY_CANCEL_RET: AtomicU32 = AtomicU32::new(99);
        static REENTRY_CANCEL_ERR: AtomicU32 = AtomicU32::new(0);

        unsafe extern "win64" fn reentry_apc_cb(err_code: u32, _bytes: u32, ov_addr: usize) {
            assert_eq!(err_code, ERROR_OPERATION_ABORTED);
            let h = REENTRY_HANDLE.load(Ordering::Acquire) as HANDLE;
            let rc = CancelIoEx(h, ov_addr as *mut OVERLAPPED);
            REENTRY_CANCEL_RET.store(rc as u32, Ordering::Release);
            REENTRY_CANCEL_ERR.store(teb::last_error(), Ordering::Release);
            REENTRY_CALLS.fetch_add(1, Ordering::AcqRel);
        }

        unsafe {
            // (a) Re-entrant CancelIoEx from inside completion callback must not deadlock and
            // must return FALSE/ERROR_NOT_FOUND, with completion firing strictly once.
            let (mut rd, mut wr) = (std::ptr::null_mut(), std::ptr::null_mut());
            assert_eq!(CreatePipe(&mut rd, &mut wr, std::ptr::null_mut(), 0), TRUE);
            REENTRY_HANDLE.store(rd as usize, Ordering::Release);
            REENTRY_CALLS.store(0, Ordering::Release);

            let mut buf = [0u8; 32];
            let mut ov: OVERLAPPED = std::mem::zeroed();
            assert_eq!(ReadFileEx(rd, buf.as_mut_ptr() as *mut c_void, 32, &mut ov, Some(reentry_apc_cb)), TRUE);
            assert_eq!(CancelIoEx(rd, &mut ov), TRUE);
            assert_eq!(SleepEx(2000, TRUE), WAIT_IO_COMPLETION);
            assert_eq!(REENTRY_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(REENTRY_CANCEL_RET.load(Ordering::Acquire), FALSE as u32);
            assert_eq!(REENTRY_CANCEL_ERR.load(Ordering::Acquire), err::ERROR_NOT_FOUND);
            assert_eq!(CloseHandle(rd), TRUE);
            assert_eq!(CloseHandle(wr), TRUE);

            // (b) Repeated race between peer WriteFile and CancelIoEx on overlapped ReadFile:
            // verifies strictly-once IOCP completion, no post-cancel buffer mutation, and safe
            // immediate reuse/overwrite of OVERLAPPED.h_event once terminal ov.internal is observed.
            let (mut rd2, mut wr2) = (std::ptr::null_mut(), std::ptr::null_mut());
            assert_eq!(CreatePipe(&mut rd2, &mut wr2, std::ptr::null_mut(), 0), TRUE);
            let iocp = CreateIoCompletionPort(rd2, std::ptr::null_mut(), 0xCA01, 1);
            assert!(!iocp.is_null());
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!ev.is_null());

            let pattern = [0x5Au8; 64];
            for iter in 0..40usize {
                let mut rbuf = [0xCCu8; 64];
                let mut ov_r: OVERLAPPED = std::mem::zeroed();
                ov_r.h_event = ev;
                let mut got = 0u32;
                let r = ReadFile(rd2, rbuf.as_mut_ptr() as *mut c_void, 64, &mut got, &mut ov_r);
                if r == FALSE {
                    assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);
                    let wr_addr = wr2 as usize;
                    let writer = std::thread::spawn(move || {
                        let mut wn = 0u32;
                        let _ = WriteFile(
                            wr_addr as HANDLE,
                            pattern.as_ptr() as *const c_void,
                            pattern.len() as u32,
                            &mut wn,
                            std::ptr::null_mut(),
                        );
                    });
                    if iter % 2 == 0 {
                        std::thread::yield_now();
                    }
                    let cancelled = CancelIoEx(rd2, &mut ov_r);

                    // Wait for terminal ov.internal publication, then immediately clobber h_event
                    // (simulating caller freeing/reusing OVERLAPPED as soon as internal != STATUS_PENDING).
                    while load_ov_internal(&mut ov_r) == STATUS_PENDING {
                        std::thread::yield_now();
                    }
                    let final_status = load_ov_internal(&mut ov_r);
                    let final_bytes = load_ov_internal_high(&mut ov_r);
                    std::ptr::write_unaligned(
                        std::ptr::addr_of_mut!(ov_r.h_event),
                        0xDEAD_BEEFusize as HANDLE,
                    );

                    // If cancelled, buffer must NEVER be mutated by the reactor afterward.
                    if final_status == STATUS_CANCELLED {
                        assert_eq!(cancelled, TRUE);
                        assert_eq!(final_bytes, 0);
                        assert!(rbuf.iter().all(|&b| b == 0xCC), "reactor wrote into buffer after STATUS_CANCELLED");
                    } else {
                        assert_eq!(final_status, STATUS_SUCCESS);
                        assert!(final_bytes > 0 && final_bytes <= 64);
                        assert!(rbuf[..final_bytes].iter().all(|&b| b == 0x5A));
                    }

                    // Exactly one IOCP completion packet must arrive.
                    let mut p_bytes = 0u32;
                    let mut p_key = 0usize;
                    let mut p_ov: *mut OVERLAPPED = std::ptr::null_mut();
                    let ok = GetQueuedCompletionStatus(iocp, &mut p_bytes, &mut p_key, &mut p_ov, 3000);
                    assert_eq!(p_key, 0xCA01);
                    assert_eq!(p_ov, &mut ov_r as *mut OVERLAPPED);
                    if final_status == STATUS_CANCELLED {
                        assert_eq!(ok, FALSE);
                        assert_eq!(teb::last_error(), ERROR_OPERATION_ABORTED);
                    } else {
                        assert_eq!(ok, TRUE);
                        assert_eq!(p_bytes as usize, final_bytes);
                    }
                    assert_eq!(WaitForSingleObject(ev, 2000), WAIT_OBJECT_0);
                    writer.join().unwrap();

                    // Drain any unread bytes left in pipe before the next iteration.
                    let mut avail = 0u32;
                    while PeekNamedPipe(rd2, std::ptr::null_mut(), 0, std::ptr::null_mut(), &mut avail, std::ptr::null_mut()) == TRUE && avail > 0 {
                        let mut drain = [0u8; 64];
                        let mut dn = 0u32;
                        let _ = ReadFile(rd2, drain.as_mut_ptr() as *mut c_void, drain.len() as u32, &mut dn, std::ptr::null_mut());
                    }
                } else {
                    // Synchronous completion still posts 1 IOCP packet; dequeue it.
                    let mut p_bytes = 0u32;
                    let mut p_key = 0usize;
                    let mut p_ov: *mut OVERLAPPED = std::ptr::null_mut();
                    assert_eq!(GetQueuedCompletionStatus(iocp, &mut p_bytes, &mut p_key, &mut p_ov, 3000), TRUE);
                }
            }

            // Confirm zero duplicate/stray IOCP packets remain.
            let mut extra_entry: OVERLAPPED_ENTRY = std::mem::zeroed();
            let mut extra_removed = 0u32;
            assert_eq!(GetQueuedCompletionStatusEx(iocp, &mut extra_entry, 1, &mut extra_removed, 0, FALSE), FALSE);
            assert_eq!(teb::last_error(), WAIT_TIMEOUT);

            assert_eq!(CloseHandle(ev), TRUE);
            assert_eq!(CloseHandle(iocp), TRUE);
            assert_eq!(CloseHandle(rd2), TRUE);
            assert_eq!(CloseHandle(wr2), TRUE);
        }
    }

    #[test]
    fn pipe_nonblocking_read_never_stalls_reactor_on_competing_reader_and_preserves_sync_eof() {
        use crate::kernel32::threads::{CloseHandle, DuplicateHandle};

        unsafe {
            // (a) CreatePipe + DuplicateHandle competing reader must never block the shared
            // PipeIoReactor or stall unrelated pipes, and must never set O_NONBLOCK on the fd.
            let (mut rd_a, mut wr_a) = (std::ptr::null_mut(), std::ptr::null_mut());
            let (mut rd_b, mut wr_b) = (std::ptr::null_mut(), std::ptr::null_mut());
            assert_eq!(CreatePipe(&mut rd_a, &mut wr_a, std::ptr::null_mut(), 0), TRUE);
            assert_eq!(CreatePipe(&mut rd_b, &mut wr_b, std::ptr::null_mut(), 0), TRUE);

            let cur_proc = handles::CURRENT_PROCESS;
            let mut rd_a_dup: HANDLE = std::ptr::null_mut();
            assert_eq!(
                DuplicateHandle(cur_proc, rd_a, cur_proc, &mut rd_a_dup, 0, FALSE, 2 /* DUPLICATE_SAME_ACCESS */),
                TRUE
            );
            assert!(!rd_a_dup.is_null());

            let (fd_a, _) = file_fd_and_kind(rd_a).unwrap();
            let (fd_a_dup, _) = file_fd_and_kind(rd_a_dup).unwrap();
            // Empty nonblocking read returns Ok(None) immediately and leaves O_NONBLOCK clear.
            let mut probe = [0u8; 8];
            assert_eq!(try_nonblocking_pipe_read(fd_a, probe.as_mut_ptr(), probe.len()), Ok(None));
            assert_eq!(libc::fcntl(fd_a, libc::F_GETFL) & libc::O_NONBLOCK, 0);
            assert_eq!(libc::fcntl(fd_a_dup, libc::F_GETFL) & libc::O_NONBLOCK, 0);

            let iocp = CreateIoCompletionPort(rd_a, std::ptr::null_mut(), 0xA001, 1);
            assert!(!iocp.is_null());
            assert_eq!(CreateIoCompletionPort(rd_b, iocp, 0xB002, 0), iocp);

            // Race Pipe A's overlapped reactor read against a competing reader on rd_a_dup,
            // while verifying Pipe B's overlapped read completes promptly without stalling.
            for iter in 0..25usize {
                let mut buf_a = [0u8; 16];
                let mut ov_a: OVERLAPPED = std::mem::zeroed();
                let mut got_a = 0u32;
                assert_eq!(ReadFile(rd_a, buf_a.as_mut_ptr() as *mut c_void, 16, &mut got_a, &mut ov_a), FALSE);
                assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

                let mut buf_b = [0u8; 16];
                let mut ov_b: OVERLAPPED = std::mem::zeroed();
                let mut got_b = 0u32;
                assert_eq!(ReadFile(rd_b, buf_b.as_mut_ptr() as *mut c_void, 16, &mut got_b, &mut ov_b), FALSE);
                assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

                // Write 1 byte to Pipe A, then immediately try to steal it via the duplicate handle.
                let mut wn = 0u32;
                let byte = [(iter & 0xFF) as u8];
                assert_eq!(WriteFile(wr_a, byte.as_ptr() as *const c_void, 1, &mut wn, std::ptr::null_mut()), TRUE);
                let mut stolen = [0u8; 4];
                let stole = matches!(try_nonblocking_pipe_read(fd_a_dup, stolen.as_mut_ptr(), 4), Ok(Some(1)));

                // Write to unrelated Pipe B: if the reactor blocked on Pipe A after losing the
                // race, Pipe B's completion would hang!
                let tag = b"pipe-b!";
                assert_eq!(WriteFile(wr_b, tag.as_ptr() as *const c_void, tag.len() as u32, &mut wn, std::ptr::null_mut()), TRUE);

                // If the duplicate handle stole the byte, cancel Pipe A's still-pending read;
                // CancelIoEx must return immediately without blocking on a stuck reactor!
                if stole {
                    let _ = CancelIoEx(rd_a, &mut ov_a);
                }

                let mut saw_a = false;
                let mut saw_b = false;
                for _ in 0..2 {
                    let mut p_bytes = 0u32;
                    let mut p_key = 0usize;
                    let mut p_ov: *mut OVERLAPPED = std::ptr::null_mut();
                    let ok = GetQueuedCompletionStatus(iocp, &mut p_bytes, &mut p_key, &mut p_ov, 3000);
                    if p_key == 0xB002 {
                        assert_eq!(ok, TRUE);
                        assert_eq!(p_ov, &mut ov_b as *mut OVERLAPPED);
                        assert_eq!(p_bytes as usize, tag.len());
                        assert_eq!(&buf_b[..tag.len()], tag);
                        saw_b = true;
                    } else if p_key == 0xA001 {
                        assert_eq!(p_ov, &mut ov_a as *mut OVERLAPPED);
                        if ok == TRUE {
                            assert_eq!(p_bytes, 1);
                            assert_eq!(buf_a[0], byte[0]);
                        } else {
                            assert_eq!(teb::last_error(), ERROR_OPERATION_ABORTED);
                        }
                        saw_a = true;
                    } else {
                        panic!("unexpected IOCP key {p_key:#x}");
                    }
                }
                assert!(saw_a && saw_b);
            }

            // (b) Synchronous ReadFile on duplicate pipe handle still blocks for data and
            // reports ERROR_BROKEN_PIPE cleanly when the write end closes.
            assert_eq!(libc::fcntl(fd_a_dup, libc::F_GETFL) & libc::O_NONBLOCK, 0);
            let dup_addr = rd_a_dup as usize;
            let sync_reader = std::thread::spawn(move || {
                let mut out = [0u8; 32];
                let mut n1 = 0u32;
                let ok1 = ReadFile(dup_addr as HANDLE, out.as_mut_ptr() as *mut c_void, 32, &mut n1, std::ptr::null_mut());
                assert_eq!(ok1, TRUE);
                let first = out[..n1 as usize].to_vec();
                let mut n2 = 0u32;
                let ok2 = ReadFile(dup_addr as HANDLE, out.as_mut_ptr() as *mut c_void, 32, &mut n2, std::ptr::null_mut());
                assert_eq!(ok2, FALSE);
                (first, teb::last_error())
            });
            let msg = b"sync-wait-ok";
            let mut wn = 0u32;
            assert_eq!(WriteFile(wr_a, msg.as_ptr() as *const c_void, msg.len() as u32, &mut wn, std::ptr::null_mut()), TRUE);
            assert_eq!(CloseHandle(wr_a), TRUE);
            let (first, eof_err) = sync_reader.join().unwrap();
            assert_eq!(&first, msg);
            assert_eq!(eof_err, err::ERROR_BROKEN_PIPE);

            assert_eq!(CloseHandle(rd_a_dup), TRUE);
            assert_eq!(CloseHandle(rd_a), TRUE);
            assert_eq!(CloseHandle(rd_b), TRUE);
            assert_eq!(CloseHandle(wr_b), TRUE);
            assert_eq!(CloseHandle(iocp), TRUE);

            // (c) External raw POSIX S_IFIFO fallback: nonblocking read never blocks on empty
            // FIFO, and synchronous ReadFile still waits for data and returns ERROR_BROKEN_PIPE on EOF.
            let mut raw_fds = [-1i32; 2];
            assert_eq!(libc::pipe(raw_fds.as_mut_ptr()), 0);
            set_cloexec(raw_fds[0]);
            set_cloexec(raw_fds[1]);
            let h_fifo_rd = handles::insert(Arc::new(FileObj::new(
                File::from_raw_fd(raw_fds[0]),
                FileKind::Pipe,
                "\\\\.\\pipe\\raw-fifo-rd".into(),
                None,
                GENERIC_READ,
                0,
            )));
            let h_fifo_wr = handles::insert(Arc::new(FileObj::new(
                File::from_raw_fd(raw_fds[1]),
                FileKind::Pipe,
                "\\\\.\\pipe\\raw-fifo-wr".into(),
                None,
                GENERIC_WRITE,
                0,
            )));
            assert_eq!(try_nonblocking_pipe_read(raw_fds[0], probe.as_mut_ptr(), 8), Ok(None));

            let fifo_rd_addr = h_fifo_rd as usize;
            let fifo_sync = std::thread::spawn(move || {
                let mut b = [0u8; 16];
                let mut n1 = 0u32;
                assert_eq!(ReadFile(fifo_rd_addr as HANDLE, b.as_mut_ptr() as *mut c_void, 16, &mut n1, std::ptr::null_mut()), TRUE);
                let got = b[..n1 as usize].to_vec();
                let mut n2 = 0u32;
                assert_eq!(ReadFile(fifo_rd_addr as HANDLE, b.as_mut_ptr() as *mut c_void, 16, &mut n2, std::ptr::null_mut()), FALSE);
                (got, teb::last_error())
            });
            assert_eq!(WriteFile(h_fifo_wr, b"fifo-ok".as_ptr() as *const c_void, 7, &mut wn, std::ptr::null_mut()), TRUE);
            assert_eq!(CloseHandle(h_fifo_wr), TRUE);
            let (fifo_got, fifo_eof_err) = fifo_sync.join().unwrap();
            assert_eq!(&fifo_got, b"fifo-ok");
            assert_eq!(fifo_eof_err, err::ERROR_BROKEN_PIPE);
            assert_eq!(CloseHandle(h_fifo_rd), TRUE);
        }
    }

    #[test]
    fn read_directory_changes_atomic_buffered_and_live_registration_all_modes() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::{AtomicU32, AtomicUsize};

        let dir = std::env::temp_dir().join(format!("win32_rt_dirwatch_atomic_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        crate::path::add_mount("W:\\", std::path::Path::new("/"));
        let win_dir = to_wide_z(&format!("W:{}", dir.to_string_lossy().replace('/', "\\")));
        let filter = FILE_NOTIFY_CHANGE_FILE_NAME | FILE_NOTIFY_CHANGE_LAST_WRITE | FILE_NOTIFY_CHANGE_SIZE;

        unsafe {
            let open_dir = || {
                let h = CreateFileW(
                    win_dir.as_ptr(),
                    GENERIC_READ,
                    7,
                    std::ptr::null_mut(),
                    OPEN_EXISTING,
                    FILE_FLAG_BACKUP_SEMANTICS | 0x4000_0000, // FILE_FLAG_OVERLAPPED
                    std::ptr::null_mut(),
                );
                assert_ne!(h, INVALID_HANDLE_VALUE);
                h
            };

            let wait_for_buffered = |h: HANDLE| {
                let f = get_file(h).unwrap();
                let w = f.dir_watcher.lock().unwrap().clone().expect("watcher started");
                let dl = Instant::now() + Duration::from_secs(3);
                while Instant::now() < dl {
                    if !w.state.lock().unwrap().pending_changes.is_empty() {
                        return;
                    }
                    std::thread::yield_now();
                }
                panic!("timed out waiting for watcher to buffer filesystem change");
            };

            // (a) Event-before-register (buffered change consumed atomically) across Sync, APC, and Overlapped+IOCP.
            let h = open_dir();
            let iocp = CreateIoCompletionPort(h, std::ptr::null_mut(), 0xD777, 1);
            assert!(!iocp.is_null());

            // Prime the watcher with one live overlapped watch so DirWatcher is running.
            let mut buf = [0u8; 4096];
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            let mut ov: OVERLAPPED = std::mem::zeroed();
            ov.h_event = ev;
            assert_eq!(
                ReadDirectoryChangesW(h, buf.as_mut_ptr() as *mut c_void, buf.len() as u32, FALSE, filter, std::ptr::null_mut(), &mut ov, std::ptr::null()),
                TRUE
            );
            std::fs::write(dir.join("prime.txt"), b"0").unwrap();
            let mut p_bytes = 0u32;
            let mut p_key = 0usize;
            let mut p_ov: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut p_bytes, &mut p_key, &mut p_ov, 3000), TRUE);
            assert_eq!(WaitForSingleObject(ev, 2000), WAIT_OBJECT_0);

            // 1. Buffered -> Overlapped+IOCP: mutate while NO request is registered, wait until
            // buffered, then register overlapped read; must complete immediately with NO extra mutation.
            std::fs::write(dir.join("buffered_ov.txt"), b"1").unwrap();
            wait_for_buffered(h);
            assert_eq!(
                ReadDirectoryChangesW(h, buf.as_mut_ptr() as *mut c_void, buf.len() as u32, FALSE, filter, std::ptr::null_mut(), &mut ov, std::ptr::null()),
                TRUE
            );
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut p_bytes, &mut p_key, &mut p_ov, 2000), TRUE);
            assert_eq!(p_key, 0xD777);
            assert_eq!(p_ov, &mut ov as *mut OVERLAPPED);
            assert_eq!(WaitForSingleObject(ev, 1000), WAIT_OBJECT_0);
            let recs = parse_notify_records(&buf, p_bytes as usize);
            assert!(recs.iter().any(|(_, p)| p == "buffered_ov.txt"), "got {recs:?}");

            // 2. Buffered -> APC: mutate while NO request is registered, wait until buffered, then
            // register APC read; must queue APC immediately without touching ov.h_event.
            static ATOMIC_APC_CALLS: AtomicU32 = AtomicU32::new(0);
            static ATOMIC_APC_BYTES: AtomicU32 = AtomicU32::new(0);
            static ATOMIC_APC_OV: AtomicUsize = AtomicUsize::new(0);
            unsafe extern "win64" fn atomic_apc_cb(err_code: u32, bytes: u32, ov_addr: usize) {
                assert_eq!(err_code, 0);
                ATOMIC_APC_BYTES.store(bytes, Ordering::Release);
                ATOMIC_APC_OV.store(ov_addr, Ordering::Release);
                ATOMIC_APC_CALLS.fetch_add(1, Ordering::AcqRel);
            }
            ATOMIC_APC_CALLS.store(0, Ordering::Release);
            let apc_ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            let mut ov_apc: OVERLAPPED = std::mem::zeroed();
            ov_apc.h_event = apc_ev;
            std::fs::write(dir.join("buffered_apc.txt"), b"2").unwrap();
            wait_for_buffered(h);
            assert_eq!(
                ReadDirectoryChangesW(
                    h,
                    buf.as_mut_ptr() as *mut c_void,
                    buf.len() as u32,
                    FALSE,
                    filter,
                    std::ptr::null_mut(),
                    &mut ov_apc,
                    atomic_apc_cb as *const c_void,
                ),
                TRUE
            );
            assert_eq!(SleepEx(2000, TRUE), WAIT_IO_COMPLETION);
            assert_eq!(ATOMIC_APC_CALLS.load(Ordering::Acquire), 1);
            assert_eq!(ATOMIC_APC_OV.load(Ordering::Acquire), &mut ov_apc as *mut OVERLAPPED as usize);
            assert_eq!(WaitForSingleObject(apc_ev, 0), WAIT_TIMEOUT, "APC mode must not signal ov.h_event");
            let recs = parse_notify_records(&buf, ATOMIC_APC_BYTES.load(Ordering::Acquire) as usize);
            assert!(recs.iter().any(|(_, p)| p == "buffered_apc.txt"), "got {recs:?}");

            // 3. Buffered -> Sync: mutate while NO request is registered, wait until buffered, then
            // call synchronous ReadDirectoryChangesW; must return immediately with the buffered record.
            std::fs::write(dir.join("buffered_sync.txt"), b"3").unwrap();
            wait_for_buffered(h);
            let mut sync_bytes = 0u32;
            assert_eq!(
                ReadDirectoryChangesW(h, buf.as_mut_ptr() as *mut c_void, buf.len() as u32, FALSE, filter, &mut sync_bytes, std::ptr::null_mut(), std::ptr::null()),
                TRUE
            );
            let recs = parse_notify_records(&buf, sync_bytes as usize);
            assert!(recs.iter().any(|(_, p)| p == "buffered_sync.txt"), "got {recs:?}");

            // (b) Concurrent mutation-vs-registration stress across Sync, APC, and Overlapped+IOCP:
            // exercises both event-before-register and register-before-event with zero hangs.
            for i in 0..10usize {
                let name = format!("race_ov_{i}.txt");
                let p = dir.join(&name);
                let mut ov_r: OVERLAPPED = std::mem::zeroed();
                let t = std::thread::spawn(move || {
                    std::fs::write(p, b"r").unwrap();
                });
                assert_eq!(
                    ReadDirectoryChangesW(h, buf.as_mut_ptr() as *mut c_void, buf.len() as u32, FALSE, filter, std::ptr::null_mut(), &mut ov_r, std::ptr::null()),
                    TRUE
                );
                assert_eq!(GetQueuedCompletionStatus(iocp, &mut p_bytes, &mut p_key, &mut p_ov, 3000), TRUE);
                assert_eq!(p_ov, &mut ov_r as *mut OVERLAPPED);
                t.join().unwrap();

                let name_a = format!("race_apc_{i}.txt");
                let pa = dir.join(&name_a);
                ATOMIC_APC_CALLS.store(0, Ordering::Release);
                let mut ov_ra: OVERLAPPED = std::mem::zeroed();
                let ta = std::thread::spawn(move || {
                    std::fs::write(pa, b"a").unwrap();
                });
                assert_eq!(
                    ReadDirectoryChangesW(
                        h,
                        buf.as_mut_ptr() as *mut c_void,
                        buf.len() as u32,
                        FALSE,
                        filter,
                        std::ptr::null_mut(),
                        &mut ov_ra,
                        atomic_apc_cb as *const c_void,
                    ),
                    TRUE
                );
                assert_eq!(SleepEx(3000, TRUE), WAIT_IO_COMPLETION);
                assert_eq!(ATOMIC_APC_CALLS.load(Ordering::Acquire), 1);
                ta.join().unwrap();

                let name_s = format!("race_sync_{i}.txt");
                let ps = dir.join(&name_s);
                let mut sb = 0u32;
                let ts = std::thread::spawn(move || {
                    std::fs::write(ps, b"s").unwrap();
                });
                assert_eq!(
                    ReadDirectoryChangesW(h, buf.as_mut_ptr() as *mut c_void, buf.len() as u32, FALSE, filter, &mut sb, std::ptr::null_mut(), std::ptr::null()),
                    TRUE
                );
                assert!(sb > 0);
                ts.join().unwrap();
            }

            assert_eq!(CloseHandle(apc_ev), TRUE);
            assert_eq!(CloseHandle(ev), TRUE);
            assert_eq!(CloseHandle(h), TRUE);
            assert_eq!(CloseHandle(iocp), TRUE);
            let _ = std::fs::remove_dir_all(&dir);
        }
    }

    #[test]
    fn complete_overlapped_captures_iocp_and_handle_event_before_publication() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};
        const WAIT_OBJECT_0: u32 = 0;
        const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u8 = 0x1;
        const FILE_SKIP_SET_EVENT_ON_HANDLE: u8 = 0x2;

        unsafe {
            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());

            // 1. FileObj (pipe) path: consumer wakes on hEvent right after terminal Internal is
            // published, immediately closes both the file handle and the event handle, and mutates
            // `ov.h_event` (with low bit set) in valid stack storage. Completion must STILL set the
            // retained handle event and post the original completion key/bytes to IOCP.
            let mut rd: HANDLE = std::ptr::null_mut();
            let mut wr: HANDLE = std::ptr::null_mut();
            assert_eq!(CreatePipe(&mut rd, &mut wr, std::ptr::null_mut(), 4096), TRUE);
            let file_key = 0x1122_3344usize;
            assert_eq!(CreateIoCompletionPort(rd, iocp, file_key, 0), iocp);
            let file_binding = lookup_io_completion_binding(rd).unwrap();
            file_binding.io_event.reset();

            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!ev.is_null());
            let mut ov: OVERLAPPED = std::mem::zeroed();
            ov.h_event = ev;
            prepare_overlapped_pending(&mut ov);

            let ov_ptr = &mut ov as *mut OVERLAPPED;
            let posted = complete_overlapped_binding_with_hook(
                Some(&file_binding),
                ov_ptr,
                STATUS_SUCCESS,
                321,
                false,
                || {
                    // Publication boundary: Internal/InternalHigh and explicit hEvent are already
                    // visible, while handle_event and IOCP delivery have not run yet.
                    assert_eq!(load_ov_internal(ov_ptr), STATUS_SUCCESS);
                    assert_eq!(load_ov_internal_high(ov_ptr), 321);
                    assert_eq!(WaitForSingleObject(ev, 0), WAIT_OBJECT_0);
                    assert_eq!(CloseHandle(rd), TRUE);
                    assert_eq!(CloseHandle(ev), TRUE);
                    // Mutate `ov.h_event` (including low-bit suppress flag) to prove no post-publish
                    // guest OVERLAPPED dereference occurs.
                    (*ov_ptr).h_event = (0xDEAD_BEEF_usize | 1) as HANDLE;
                },
            );
            assert!(posted);
            assert!(get_file(rd).is_none(), "file handle must already be closed");
            assert!(
                file_binding.io_event.try_take(),
                "retained handle event must still be signaled after CloseHandle(rd)"
            );

            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 1000), TRUE);
            assert_eq!(key, file_key);
            assert_eq!(bytes, 321);
            assert_eq!(ov_out, ov_ptr);

            // Also verify PendingPipeOp's retained Arc<FileObj> completes cleanly even if CloseHandle
            // already removed the handle BEFORE `complete_overlapped_binding` is entered.
            let mut ov_late: OVERLAPPED = std::mem::zeroed();
            prepare_overlapped_pending(&mut ov_late);
            let ov_late_ptr = &mut ov_late as *mut OVERLAPPED;
            assert!(complete_overlapped_binding(
                Some(&file_binding),
                ov_late_ptr,
                STATUS_SUCCESS,
                77,
                false,
            ));
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 1000), TRUE);
            assert_eq!(key, file_key);
            assert_eq!(bytes, 77);
            assert_eq!(ov_out, ov_late_ptr);
            assert_eq!(CloseHandle(wr), TRUE);

            // 2. External/socket handle path: unregistering the handle immediately after hEvent
            // signal must still deliver the original IOCP packet and handle event.
            let fake_sock = 0x00EE_0001usize as HANDLE;
            register_external_io_handle(fake_sock);
            let sock_key = 0x5566_7788usize;
            assert_eq!(CreateIoCompletionPort(fake_sock, iocp, sock_key, 0), iocp);
            let sock_binding = lookup_io_completion_binding(fake_sock).unwrap();
            sock_binding.io_event.reset();

            let sock_ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!sock_ev.is_null());
            let mut ov_s: OVERLAPPED = std::mem::zeroed();
            ov_s.h_event = sock_ev;
            prepare_overlapped_pending(&mut ov_s);
            let ov_s_ptr = &mut ov_s as *mut OVERLAPPED;

            let posted_s = complete_overlapped_binding_with_hook(
                Some(&sock_binding),
                ov_s_ptr,
                STATUS_SUCCESS,
                654,
                false,
                || {
                    assert_eq!(load_ov_internal(ov_s_ptr), STATUS_SUCCESS);
                    assert_eq!(load_ov_internal_high(ov_s_ptr), 654);
                    assert_eq!(WaitForSingleObject(sock_ev, 0), WAIT_OBJECT_0);
                    unregister_external_io_handle(fake_sock);
                    assert_eq!(CloseHandle(sock_ev), TRUE);
                    (*ov_s_ptr).h_event = (0xCAFE_BABE_usize | 1) as HANDLE;
                },
            );
            assert!(posted_s);
            assert!(lookup_io_completion_binding(fake_sock).is_none());
            assert!(sock_binding.io_event.try_take());

            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 1000), TRUE);
            assert_eq!(key, sock_key);
            assert_eq!(bytes, 654);
            assert_eq!(ov_out, ov_s_ptr);

            // 3. Suppression rules (SetFileCompletionNotificationModes):
            //    (a) low bit on hEvent suppresses IOCP while still signaling the explicit event;
            //    (b) FILE_SKIP_SET_EVENT_ON_HANDLE suppresses the implicit handle event on both
            //        pending and synchronous completions (never suppressing explicit hEvent);
            //    (c) FILE_SKIP_COMPLETION_PORT_ON_SUCCESS suppresses IOCP only on sync completions.
            let fake_sock2 = 0x00EE_0002usize as HANDLE;
            register_external_io_handle(fake_sock2);
            assert_eq!(CreateIoCompletionPort(fake_sock2, iocp, 0x9999, 0), iocp);
            assert_eq!(
                SetFileCompletionNotificationModes(
                    fake_sock2,
                    FILE_SKIP_COMPLETION_PORT_ON_SUCCESS | FILE_SKIP_SET_EVENT_ON_HANDLE,
                ),
                TRUE
            );
            let b2 = lookup_io_completion_binding(fake_sock2).unwrap();
            b2.io_event.reset();
            let ev2 = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            let mut ov_sup: OVERLAPPED = std::mem::zeroed();
            ov_sup.h_event = (ev2 as usize | 1) as HANDLE;
            prepare_overlapped_pending(&mut ov_sup);
            assert!(!complete_overlapped_io(fake_sock2, &mut ov_sup, STATUS_SUCCESS, 12, false));
            assert_eq!(WaitForSingleObject(ev2, 0), WAIT_OBJECT_0);
            assert!(!b2.io_event.try_take());

            // Pending completion with explicit event (no low bit): explicit event is set,
            // implicit handle event remains clear, and exactly one IOCP packet is posted
            // despite FILE_SKIP_COMPLETION_PORT_ON_SUCCESS (0x1) also being set.
            assert_eq!(crate::kernel32::threads::ResetEvent(ev2), TRUE);
            b2.io_event.reset();
            let mut ov_pend: OVERLAPPED = std::mem::zeroed();
            ov_pend.h_event = ev2;
            prepare_overlapped_pending(&mut ov_pend);
            assert!(complete_overlapped_io(fake_sock2, &mut ov_pend, STATUS_SUCCESS, 56, false));
            assert_eq!(WaitForSingleObject(ev2, 0), WAIT_OBJECT_0);
            assert!(!b2.io_event.try_take());
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 1000), TRUE);
            assert_eq!(key, 0x9999);
            assert_eq!(bytes, 56);
            assert_eq!(ov_out, &mut ov_pend as *mut OVERLAPPED);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 20), FALSE);

            b2.io_event.reset();
            let mut ov_sync: OVERLAPPED = std::mem::zeroed();
            prepare_overlapped_pending(&mut ov_sync);
            assert!(!complete_overlapped_io(fake_sock2, &mut ov_sync, STATUS_SUCCESS, 34, true));
            assert!(!b2.io_event.try_take());
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 20), FALSE);

            unregister_external_io_handle(fake_sock2);
            assert_eq!(CloseHandle(ev2), TRUE);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn complete_overlapped_captures_tp_io_before_publication_and_drains_wait() {
        use crate::kernel32::threadpool::{
            CloseThreadpoolIo, CreateThreadpoolIo, StartThreadpoolIo, TpIo, WaitForThreadpoolIoCallbacks,
        };
        use crate::kernel32::threads::{CloseHandle, CreateEventW, WaitForSingleObject};
        use std::sync::atomic::AtomicUsize;

        const WAIT_OBJECT_0: u32 = 0;

        struct TpCapture {
            calls: AtomicUsize,
            last_ov: AtomicUsize,
            last_res: AtomicUsize,
            last_bytes: AtomicUsize,
        }

        unsafe extern "win64" fn tp_io_cb(
            _inst: *mut c_void,
            ctx: *mut c_void,
            ov: *mut c_void,
            io_result: usize,
            bytes: usize,
            _pio: *mut c_void,
        ) {
            let cap = &*(ctx as *const TpCapture);
            cap.last_ov.store(ov as usize, Ordering::Release);
            cap.last_res.store(io_result, Ordering::Release);
            cap.last_bytes.store(bytes, Ordering::Release);
            cap.calls.fetch_add(1, Ordering::AcqRel);
        }

        unsafe {
            // 1. FileObj + TpIo: closing the file handle immediately upon hEvent signal must
            // still dispatch the TpIo callback and drain WaitForThreadpoolIoCallbacks.
            let mut rd: HANDLE = std::ptr::null_mut();
            let mut wr: HANDLE = std::ptr::null_mut();
            assert_eq!(CreatePipe(&mut rd, &mut wr, std::ptr::null_mut(), 4096), TRUE);
            let cap_file = Box::new(TpCapture {
                calls: AtomicUsize::new(0),
                last_ov: AtomicUsize::new(0),
                last_res: AtomicUsize::new(usize::MAX),
                last_bytes: AtomicUsize::new(0),
            });
            let pio_file = CreateThreadpoolIo(
                rd,
                tp_io_cb as *const c_void,
                &*cap_file as *const TpCapture as *mut c_void,
                std::ptr::null_mut(),
            );
            assert!(!pio_file.is_null());
            StartThreadpoolIo(pio_file);

            let file_binding = lookup_io_completion_binding(rd).unwrap();
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            let mut ov: OVERLAPPED = std::mem::zeroed();
            ov.h_event = ev;
            prepare_overlapped_pending(&mut ov);
            let ov_ptr = &mut ov as *mut OVERLAPPED;

            let dispatched = complete_overlapped_binding_with_hook(
                Some(&file_binding),
                ov_ptr,
                STATUS_SUCCESS,
                987,
                false,
                || {
                    assert_eq!(WaitForSingleObject(ev, 0), WAIT_OBJECT_0);
                    assert_eq!(CloseHandle(rd), TRUE);
                    assert_eq!(CloseHandle(ev), TRUE);
                    (*ov_ptr).h_event = (0xBEEF_usize | 1) as HANDLE;
                },
            );
            assert!(dispatched);

            // Bounded WaitForThreadpoolIoCallbacks: must drain promptly (no stuck pending_io).
            let pio_addr = pio_file as usize;
            let (tx, rx) = std::sync::mpsc::channel();
            let waiter = std::thread::spawn(move || {
                WaitForThreadpoolIoCallbacks(pio_addr as *mut TpIo, FALSE);
                let _ = tx.send(());
            });
            rx.recv_timeout(std::time::Duration::from_secs(3))
                .expect("WaitForThreadpoolIoCallbacks must drain after CloseHandle at publication boundary");
            waiter.join().unwrap();

            assert_eq!(cap_file.calls.load(Ordering::Acquire), 1);
            assert_eq!(cap_file.last_ov.load(Ordering::Acquire), ov_ptr as usize);
            assert_eq!(cap_file.last_res.load(Ordering::Acquire), 0);
            assert_eq!(cap_file.last_bytes.load(Ordering::Acquire), 987);
            CloseThreadpoolIo(pio_file);
            assert_eq!(CloseHandle(wr), TRUE);

            // 2. External/socket handle + TpIo: unregistering at the publication boundary must
            // likewise dispatch the TpIo callback and drain WaitForThreadpoolIoCallbacks.
            let fake_sock = 0x00EE_0003usize as HANDLE;
            register_external_io_handle(fake_sock);
            let cap_ext = Box::new(TpCapture {
                calls: AtomicUsize::new(0),
                last_ov: AtomicUsize::new(0),
                last_res: AtomicUsize::new(usize::MAX),
                last_bytes: AtomicUsize::new(0),
            });
            let pio_ext = CreateThreadpoolIo(
                fake_sock,
                tp_io_cb as *const c_void,
                &*cap_ext as *const TpCapture as *mut c_void,
                std::ptr::null_mut(),
            );
            assert!(!pio_ext.is_null());
            StartThreadpoolIo(pio_ext);

            let ext_binding = lookup_io_completion_binding(fake_sock).unwrap();
            let ev_ext = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            let mut ov_ext: OVERLAPPED = std::mem::zeroed();
            ov_ext.h_event = ev_ext;
            prepare_overlapped_pending(&mut ov_ext);
            let ov_ext_ptr = &mut ov_ext as *mut OVERLAPPED;

            let dispatched_ext = complete_overlapped_binding_with_hook(
                Some(&ext_binding),
                ov_ext_ptr,
                STATUS_SUCCESS,
                432,
                false,
                || {
                    assert_eq!(WaitForSingleObject(ev_ext, 0), WAIT_OBJECT_0);
                    unregister_external_io_handle(fake_sock);
                    assert_eq!(CloseHandle(ev_ext), TRUE);
                    (*ov_ext_ptr).h_event = (0xFACE_usize | 1) as HANDLE;
                },
            );
            assert!(dispatched_ext);

            let pio_ext_addr = pio_ext as usize;
            let (tx2, rx2) = std::sync::mpsc::channel();
            let waiter2 = std::thread::spawn(move || {
                WaitForThreadpoolIoCallbacks(pio_ext_addr as *mut TpIo, FALSE);
                let _ = tx2.send(());
            });
            rx2.recv_timeout(std::time::Duration::from_secs(3))
                .expect("WaitForThreadpoolIoCallbacks must drain on external handle close at publication boundary");
            waiter2.join().unwrap();

            assert_eq!(cap_ext.calls.load(Ordering::Acquire), 1);
            assert_eq!(cap_ext.last_ov.load(Ordering::Acquire), ov_ext_ptr as usize);
            assert_eq!(cap_ext.last_res.load(Ordering::Acquire), 0);
            assert_eq!(cap_ext.last_bytes.load(Ordering::Acquire), 432);
            CloseThreadpoolIo(pio_ext);
        }
    }

    #[test]
    fn protected_native_runner_bundle_and_aot_cache_block_guest_writes_renames_deletes_and_aliases() {
        use crate::kernel32::threads::CloseHandle;
        let root = std::env::temp_dir().join(format!("port_factory_vfs_guard_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let app = root.join("MockApp.app");
        let contents = app.join("Contents");
        let macos = contents.join("MacOS");
        let frameworks = contents.join("Frameworks");
        let codesig = contents.join("_CodeSignature");
        let resources = contents.join("Resources");
        let aot_bundle = resources.join("AotCache");
        let game_dir = resources.join("Game");
        let host_aot = root.join("HostAotCache");
        let guest_work = root.join("GuestWork");

        for d in [&macos, &frameworks, &codesig, &aot_bundle, &game_dir, &host_aot, &guest_work] {
            std::fs::create_dir_all(d).unwrap();
        }
        let runner = macos.join("port-runner");
        let dylib = frameworks.join("libmetalirconverter.dylib");
        let plist = contents.join("Info.plist");
        let sig_res = codesig.join("CodeResources");
        let manifest = resources.join("game.toml");
        let bundled_aot = aot_bundle.join("mod.aot");
        let host_aot_file = host_aot.join("cached.aot");
        let game_save = game_dir.join("save.dat");
        let guest_file = guest_work.join("ordinary.txt");

        std::fs::write(&runner, b"INERT-RUNNER-FIXTURE").unwrap();
        std::fs::write(&dylib, b"INERT-DYLIB-FIXTURE").unwrap();
        std::fs::write(&plist, b"<plist/>").unwrap();
        std::fs::write(&sig_res, b"sig").unwrap();
        std::fs::write(&manifest, b"name = \"Mock\"\n").unwrap();
        std::fs::write(&bundled_aot, b"AOT1").unwrap();
        std::fs::write(&host_aot_file, b"AOT2").unwrap();
        std::fs::write(&game_save, b"save-v1").unwrap();
        std::fs::write(&guest_file, b"guest-v1").unwrap();

        // Pre-existing host symlink & hardlink aliases outside the bundle pointing to the runner.
        let ext_symlink = guest_work.join("symlink_to_runner");
        let ext_hardlink = guest_work.join("hardlink_to_runner");
        std::os::unix::fs::symlink(&runner, &ext_symlink).unwrap();
        std::fs::hard_link(&runner, &ext_hardlink).unwrap();

        let _guard = path::tests::protect_native_runner_scoped_for_test("V:\\", &root, &runner);
        path::protect_native_subtree(&host_aot);

        unsafe {
            // 1. CreateFileW write / truncate / create-always / delete-on-close blocked across
            //    direct paths, case variants, `..` traversal, bundle files, AOT cache, and aliases.
            for win_target in [
                r"V:\MockApp.app\Contents\MacOS\port-runner",
                r"V:\MOCKAPP.APP\CONTENTS\MACOS\PORT-RUNNER",
                r"V:\GuestWork\..\MockApp.app\Contents\MacOS\port-runner",
                r"V:\MockApp.app\Contents\MacOS\new_helper",
                r"V:\MockApp.app\Contents\Frameworks\libmetalirconverter.dylib",
                r"V:\MockApp.app\Contents\Info.plist",
                r"V:\MockApp.app\Contents\_CodeSignature\CodeResources",
                r"V:\MockApp.app\Contents\Resources\game.toml",
                r"V:\MockApp.app\Contents\Resources\AotCache\mod.aot",
                r"V:\HostAotCache\cached.aot",
                r"V:\HostAotCache\new_entry.aot",
                r"V:\GuestWork\symlink_to_runner",
                r"V:\GuestWork\hardlink_to_runner",
            ] {
                let w = to_wide_z(win_target);
                let h = CreateFileW(
                    w.as_ptr(),
                    GENERIC_WRITE,
                    0,
                    std::ptr::null_mut(),
                    OPEN_ALWAYS,
                    0,
                    std::ptr::null_mut(),
                );
                assert_eq!(h, INVALID_HANDLE_VALUE, "write open must be blocked for {win_target}");
                assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED, "expected ACCESS_DENIED for {win_target}");

                let h_del = CreateFileW(
                    w.as_ptr(),
                    GENERIC_READ,
                    0,
                    std::ptr::null_mut(),
                    OPEN_EXISTING,
                    FILE_FLAG_DELETE_ON_CLOSE,
                    std::ptr::null_mut(),
                );
                assert_eq!(h_del, INVALID_HANDLE_VALUE, "DELETE_ON_CLOSE must be blocked for {win_target}");
                assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            }

            // 2. Delete, rename/replace (target, source, and ancestor dir), copy-over, rmdir,
            //    SetFileAttributes, hardlink, and symlink creation all fail with ACCESS_DENIED.
            let w_runner = to_wide_z(r"V:\MockApp.app\Contents\MacOS\port-runner");
            let w_guest = to_wide_z(r"V:\GuestWork\ordinary.txt");
            let w_dst = to_wide_z(r"V:\GuestWork\moved_out");
            let w_app = to_wide_z(r"V:\MockApp.app");
            let w_contents = to_wide_z(r"V:\MockApp.app\Contents");
            let w_macos = to_wide_z(r"V:\MockApp.app\Contents\MacOS");

            assert_eq!(DeleteFileW(w_runner.as_ptr()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(MoveFileExW(w_runner.as_ptr(), w_dst.as_ptr(), MOVEFILE_REPLACE_EXISTING), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(MoveFileExW(w_guest.as_ptr(), w_runner.as_ptr(), MOVEFILE_REPLACE_EXISTING), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(MoveFileExW(w_app.as_ptr(), w_dst.as_ptr(), 0), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(MoveFileExW(w_contents.as_ptr(), w_dst.as_ptr(), 0), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(ReplaceFileW(w_runner.as_ptr(), w_guest.as_ptr(), std::ptr::null(), 0, std::ptr::null_mut(), std::ptr::null_mut()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(CopyFileW(w_guest.as_ptr(), w_runner.as_ptr(), FALSE), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(RemoveDirectoryW(w_macos.as_ptr()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(RemoveDirectoryW(w_app.as_ptr()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(SetFileAttributesW(w_runner.as_ptr(), FILE_ATTRIBUTE_NORMAL), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);

            let w_new_hl = to_wide_z(r"V:\GuestWork\new_hl");
            let w_new_sl = to_wide_z(r"V:\GuestWork\new_sl");
            assert_eq!(CreateHardLinkW(w_new_hl.as_ptr(), w_runner.as_ptr(), std::ptr::null_mut()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(CreateSymbolicLinkW(w_new_sl.as_ptr(), w_runner.as_ptr(), 0), 0);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(CreateSymbolicLinkW(w_new_sl.as_ptr(), w_macos.as_ptr(), 1), 0);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);

            // 3. Handle-based mutations on a read-only handle to the protected runner all fail closed.
            let h_ro = CreateFileW(
                w_runner.as_ptr(),
                GENERIC_READ,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                0,
                std::ptr::null_mut(),
            );
            assert_ne!(h_ro, INVALID_HANDLE_VALUE, "read-only open stays allowed");
            let mut written = 0u32;
            assert_eq!(WriteFile(h_ro, b"X".as_ptr() as *const c_void, 1, &mut written, std::ptr::null_mut()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(SetEndOfFile(h_ro), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            let del_flag: u8 = 1;
            assert_eq!(SetFileInformationByHandle(h_ro, 4, &del_flag, 1), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            let eof_zero: i64 = 0;
            assert_eq!(SetFileInformationByHandle(h_ro, 6, &eof_zero as *const i64 as *const u8, 8), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            let mut rename_buf = vec![0u8; 20 + 64];
            rename_buf[0] = 1;
            let dst_u16 = to_wide(r"V:\GuestWork\renamed_via_handle");
            (rename_buf.as_mut_ptr().add(16) as *mut u32).write_unaligned((dst_u16.len() * 2) as u32);
            std::ptr::copy_nonoverlapping(dst_u16.as_ptr() as *const u8, rename_buf.as_mut_ptr().add(20), dst_u16.len() * 2);
            assert_eq!(SetFileInformationByHandle(h_ro, 3, rename_buf.as_ptr(), rename_buf.len() as u32), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert!(CreateFileMappingW(h_ro, std::ptr::null_mut(), 0x04, 0, 0, std::ptr::null()).is_null());
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(ReOpenFile(h_ro, GENERIC_WRITE, 0, 0), INVALID_HANDLE_VALUE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(CloseHandle(h_ro), TRUE);

            // 4. UCRT paths (_open write, _unlink, rename) and shell32 SHFileOperationW also refuse protected files.
            assert_eq!(crate::ucrt::stdio::open_host(r"V:\MockApp.app\Contents\MacOS\port-runner", 0x0002, 0o644), -1);
            assert_eq!(crate::ucrt::stdio::open_host(r"V:\GuestWork\symlink_to_runner", 0x0002, 0o644), -1);
            assert_eq!(crate::ucrt::env::_wunlink(w_runner.as_ptr()), -1);
            assert_eq!(crate::ucrt::env::_wrename(w_runner.as_ptr(), w_dst.as_ptr()), -1);
            assert_eq!(crate::ucrt::env::_wrename(w_guest.as_ptr(), w_runner.as_ptr()), -1);

            let mut sh_from = to_wide(r"V:\MockApp.app\Contents\MacOS\port-runner");
            sh_from.extend_from_slice(&[0, 0]);
            let mut sh_op = [0u8; 56];
            (sh_op.as_mut_ptr().add(8) as *mut u32).write_unaligned(3); // FO_DELETE
            (sh_op.as_mut_ptr().add(16) as *mut *const u16).write_unaligned(sh_from.as_ptr());
            assert_eq!(
                crate::misc::shell32::SHFileOperationW(sh_op.as_mut_ptr()),
                err::ERROR_ACCESS_DENIED as i32
            );

            // 5. Ordinary guest data (both in GuestWork and in MockApp.app/Contents/Resources/Game)
            //    remains fully writable, renamable, and removable.
            for writable_win in [
                r"V:\GuestWork\ordinary.txt",
                r"V:\GuestWork\created_by_guest.txt",
                r"V:\MockApp.app\Contents\Resources\Game\save.dat",
                r"V:\MockApp.app\Contents\Resources\Game\new_save.dat",
            ] {
                let w = to_wide_z(writable_win);
                let h = CreateFileW(
                    w.as_ptr(),
                    GENERIC_READ | GENERIC_WRITE,
                    0,
                    std::ptr::null_mut(),
                    CREATE_ALWAYS,
                    0,
                    std::ptr::null_mut(),
                );
                assert_ne!(h, INVALID_HANDLE_VALUE, "ordinary guest path must remain writable: {writable_win}");
                let mut nw = 0u32;
                assert_eq!(WriteFile(h, b"ok".as_ptr() as *const c_void, 2, &mut nw, std::ptr::null_mut()), TRUE);
                assert_eq!(nw, 2);
                assert_eq!(CloseHandle(h), TRUE);
            }
            let w_new_save = to_wide_z(r"V:\MockApp.app\Contents\Resources\Game\new_save.dat");
            let w_moved_save = to_wide_z(r"V:\MockApp.app\Contents\Resources\Game\moved_save.dat");
            assert_eq!(MoveFileExW(w_new_save.as_ptr(), w_moved_save.as_ptr(), MOVEFILE_REPLACE_EXISTING), TRUE);
            assert_eq!(DeleteFileW(w_moved_save.as_ptr()), TRUE);
        }

        // Protected runner binary remained completely untouched.
        assert_eq!(std::fs::read(&runner).unwrap(), b"INERT-RUNNER-FIXTURE");
        assert_eq!(std::fs::read(&dylib).unwrap(), b"INERT-DYLIB-FIXTURE");
        assert_eq!(std::fs::read(&host_aot_file).unwrap(), b"AOT2");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn peek_named_pipe_preserves_bytes_for_pending_reads_socketpair_and_raw_fifo() {
        use crate::kernel32::threads::CloseHandle;
        use crate::kernel32::time::SleepEx;
        use std::sync::atomic::AtomicUsize;

        const WAIT_IO_COMPLETION: u32 = 0xC0;
        static PEEK_APC_BYTES: AtomicUsize = AtomicUsize::new(0);
        static PEEK_APC_ERR: AtomicUsize = AtomicUsize::new(usize::MAX);
        unsafe extern "win64" fn peek_apc_cb(err_code: u32, bytes: u32, _ov: usize) {
            PEEK_APC_ERR.store(err_code as usize, Ordering::Release);
            PEEK_APC_BYTES.store(bytes as usize, Ordering::Release);
        }

        unsafe {
            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());

            // 1. Socketpair-backed pipe (CreatePipe): PeekNamedPipe uses MSG_PEEK without
            //    consuming kernel bytes or filling `peek_buf`, so repeated peeks see the same
            //    data and a pending overlapped ReadFile completes with no second write.
            let mut rd: HANDLE = std::ptr::null_mut();
            let mut wr: HANDLE = std::ptr::null_mut();
            assert_eq!(CreatePipe(&mut rd, &mut wr, std::ptr::null_mut(), 4096), TRUE);
            assert_eq!(CreateIoCompletionPort(rd, iocp, 0x9101, 0), iocp);

            // Repeated peeks before any read must leave `peek_buf` empty and preserve exact counts.
            let mut nw = 0u32;
            let msg1 = b"PEEK-VISIBLE";
            assert_eq!(WriteFile(wr, msg1.as_ptr() as *const c_void, msg1.len() as u32, &mut nw, std::ptr::null_mut()), TRUE);
            for _ in 0..2 {
                let mut pbuf = [0u8; 4];
                let (mut pread, mut pavail, mut pleft) = (0u32, 0u32, 0u32);
                assert_eq!(
                    PeekNamedPipe(rd, pbuf.as_mut_ptr() as *mut c_void, 4, &mut pread, &mut pavail, &mut pleft),
                    TRUE
                );
                assert_eq!(pread, 4);
                assert_eq!(pavail, msg1.len() as u32);
                assert_eq!(pleft, (msg1.len() - 4) as u32);
                assert_eq!(&pbuf, b"PEEK");
                assert!(
                    get_file(rd).unwrap().peek_buf.lock().unwrap().is_empty(),
                    "socketpair PeekNamedPipe must not consume kernel bytes into peek_buf"
                );
            }
            let mut rbuf1 = [0u8; 32];
            let mut nr = 0u32;
            assert_eq!(ReadFile(rd, rbuf1.as_mut_ptr() as *mut c_void, rbuf1.len() as u32, &mut nr, std::ptr::null_mut()), TRUE);
            assert_eq!(&rbuf1[..nr as usize], msg1);

            // Pending overlapped ReadFile + write + immediate PeekNamedPipe (no second write!).
            let mut ov_rd: OVERLAPPED = std::mem::zeroed();
            let mut rbuf2 = [0u8; 32];
            assert_eq!(ReadFile(rd, rbuf2.as_mut_ptr() as *mut c_void, rbuf2.len() as u32, std::ptr::null_mut(), &mut ov_rd), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            let msg2 = b"AFTER-PEEK";
            assert_eq!(WriteFile(wr, msg2.as_ptr() as *const c_void, msg2.len() as u32, &mut nw, std::ptr::null_mut()), TRUE);
            let mut pbuf2 = [0u8; 16];
            let (mut pread2, mut pavail2, mut pleft2) = (0u32, 0u32, 0u32);
            let _ = PeekNamedPipe(rd, pbuf2.as_mut_ptr() as *mut c_void, 10, &mut pread2, &mut pavail2, &mut pleft2);

            let (mut bytes, mut key, mut ov_out) = (0u32, 0usize, std::ptr::null_mut());
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(key, 0x9101);
            assert_eq!(ov_out, &mut ov_rd as *mut OVERLAPPED);
            assert_eq!(&rbuf2[..bytes as usize], msg2);

            // Pending ReadFileEx (APC) also completes cleanly after PeekNamedPipe with no second write.
            PEEK_APC_BYTES.store(0, Ordering::Release);
            PEEK_APC_ERR.store(usize::MAX, Ordering::Release);
            let mut ov_apc: OVERLAPPED = std::mem::zeroed();
            let mut apc_buf = [0u8; 32];
            assert_eq!(
                ReadFileEx(rd, apc_buf.as_mut_ptr() as *mut c_void, apc_buf.len() as u32, &mut ov_apc, Some(peek_apc_cb)),
                TRUE
            );
            let msg_apc = b"APC-PEEK";
            assert_eq!(WriteFile(wr, msg_apc.as_ptr() as *const c_void, msg_apc.len() as u32, &mut nw, std::ptr::null_mut()), TRUE);
            let _ = PeekNamedPipe(rd, pbuf2.as_mut_ptr() as *mut c_void, 8, &mut pread2, &mut pavail2, &mut pleft2);
            assert_eq!(SleepEx(2000, TRUE), WAIT_IO_COMPLETION);
            assert_eq!(PEEK_APC_ERR.load(Ordering::Acquire), 0);
            assert_eq!(&apc_buf[..PEEK_APC_BYTES.load(Ordering::Acquire)], msg_apc);

            assert_eq!(CloseHandle(wr), TRUE);
            assert_eq!(PeekNamedPipe(rd, std::ptr::null_mut(), 0, std::ptr::null_mut(), &mut pavail2, std::ptr::null_mut()), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_BROKEN_PIPE);
            assert_eq!(CloseHandle(rd), TRUE);

            // 2. Raw POSIX FIFO (`libc::pipe`, ENOTSOCK fallback): PeekNamedPipe buffers into
            //    `peek_buf` and wakes `pipe_reactor()`, so a pending overlapped ReadFile completes
            //    deterministically without any further write.
            let mut fds = [-1i32; 2];
            assert_eq!(libc::pipe(fds.as_mut_ptr()), 0);
            set_cloexec(fds[0]);
            set_cloexec(fds[1]);
            let rd_fifo = wrap_non_owned_fd(fds[0], FileKind::Pipe, "\\\\.\\pipe\\raw_fifo".into(), GENERIC_READ);
            assert_eq!(CreateIoCompletionPort(rd_fifo, iocp, 0x9102, 0), iocp);

            // Repeated peeks on raw FIFO return identical data and counts before sync ReadFile drains.
            let fifo_m1 = b"RAW-FIFO-1";
            assert_eq!(libc::write(fds[1], fifo_m1.as_ptr() as *const c_void, fifo_m1.len()), fifo_m1.len() as isize);
            for _ in 0..2 {
                let mut fb = [0u8; 4];
                let (mut r, mut a, mut l) = (0u32, 0u32, 0u32);
                assert_eq!(PeekNamedPipe(rd_fifo, fb.as_mut_ptr() as *mut c_void, 4, &mut r, &mut a, &mut l), TRUE);
                assert_eq!((r, a, l), (4, fifo_m1.len() as u32, (fifo_m1.len() - 4) as u32));
                assert_eq!(&fb, b"RAW-");
            }
            let mut fdrain = [0u8; 32];
            let mut got_sync = 0usize;
            while got_sync < fifo_m1.len() {
                let mut nread = 0u32;
                assert_eq!(
                    ReadFile(
                        rd_fifo,
                        fdrain[got_sync..].as_mut_ptr() as *mut c_void,
                        (fdrain.len() - got_sync) as u32,
                        &mut nread,
                        std::ptr::null_mut(),
                    ),
                    TRUE
                );
                got_sync += nread as usize;
            }
            assert_eq!(&fdrain[..got_sync], fifo_m1);

            // Now pend an overlapped ReadFile on the empty FIFO, write once, and PeekNamedPipe ALL
            // bytes into `peek_buf`. Pending ReadFile must still wake and complete with no second write!
            let mut ov_fifo: OVERLAPPED = std::mem::zeroed();
            let mut fifo_out = [0u8; 32];
            assert_eq!(
                ReadFile(rd_fifo, fifo_out.as_mut_ptr() as *mut c_void, fifo_out.len() as u32, std::ptr::null_mut(), &mut ov_fifo),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            let fifo_m2 = b"FIFO-DATA";
            assert_eq!(libc::write(fds[1], fifo_m2.as_ptr() as *const c_void, fifo_m2.len()), fifo_m2.len() as isize);
            let mut fpeek = [0u8; 16];
            let (mut fr, mut fa, mut fl) = (0u32, 0u32, 0u32);
            let _ = PeekNamedPipe(rd_fifo, fpeek.as_mut_ptr() as *mut c_void, fifo_m2.len() as u32, &mut fr, &mut fa, &mut fl);

            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(key, 0x9102);
            assert_eq!(ov_out, &mut ov_fifo as *mut OVERLAPPED);
            assert_eq!(&fifo_out[..bytes as usize], fifo_m2);

            assert_eq!(CloseHandle(rd_fifo), TRUE);
            assert_eq!(libc::close(fds[0]), 0);
            assert_eq!(libc::close(fds[1]), 0);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn disconnect_named_pipe_cancels_duplicate_handle_io_and_isolates_reconnect() {
        use crate::kernel32::process::GetCurrentProcess;
        use crate::kernel32::threads::{CloseHandle, DuplicateHandle};

        unsafe {
            let name = to_wide_z("\\\\.\\pipe\\aw2_disconnect_dup_conn_test");
            let srv1 = CreateNamedPipeW(
                name.as_ptr(),
                0x0000_0003 | 0x4000_0000, // PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED
                0,
                1,
                4096,
                4096,
                0,
                std::ptr::null_mut(),
            );
            assert_ne!(srv1, INVALID_HANDLE_VALUE);

            let mut srv2: HANDLE = std::ptr::null_mut();
            let cur_proc = GetCurrentProcess();
            assert_eq!(
                DuplicateHandle(cur_proc, srv1, cur_proc, &mut srv2, 0, FALSE, 2 /* DUPLICATE_SAME_ACCESS */),
                TRUE
            );
            assert!(!srv2.is_null());
            assert_ne!(srv1, srv2);

            let iocp = CreateIoCompletionPort(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0, 1);
            assert!(!iocp.is_null());
            assert_eq!(CreateIoCompletionPort(srv1, iocp, 0x9201, 0), iocp);

            // Connect client 1.
            let cli1 = CreateFileW(
                name.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                0,
                std::ptr::null_mut(),
            );
            assert_ne!(cli1, INVALID_HANDLE_VALUE);

            // 1. Per-handle CancelIoEx remains scoped to the handle passed to it (srv2 only, not srv1).
            let mut ov_srv1: OVERLAPPED = std::mem::zeroed();
            let mut ov_srv2_cancel: OVERLAPPED = std::mem::zeroed();
            let mut buf_srv1 = [0u8; 16];
            let mut buf_srv2 = [0u8; 16];
            assert_eq!(ReadFile(srv1, buf_srv1.as_mut_ptr() as *mut c_void, 16, std::ptr::null_mut(), &mut ov_srv1), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);
            assert_eq!(ReadFile(srv2, buf_srv2.as_mut_ptr() as *mut c_void, 16, std::ptr::null_mut(), &mut ov_srv2_cancel), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            assert_eq!(CancelIoEx(srv2, std::ptr::null_mut()), TRUE);
            let (mut bytes, mut key, mut ov_out) = (0u32, 0usize, std::ptr::null_mut());
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), FALSE);
            assert_eq!(ov_out, &mut ov_srv2_cancel as *mut OVERLAPPED);
            assert_eq!(teb::last_error(), ERROR_OPERATION_ABORTED);

            let msg_keep = b"FOR-SRV1";
            let mut nw = 0u32;
            assert_eq!(WriteFile(cli1, msg_keep.as_ptr() as *const c_void, msg_keep.len() as u32, &mut nw, std::ptr::null_mut()), TRUE);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(ov_out, &mut ov_srv1 as *mut OVERLAPPED);
            assert_eq!(&buf_srv1[..bytes as usize], msg_keep);

            // 2. Submit BOTH a pending overlapped read AND a pending backpressured overlapped write
            //    through duplicate handle `srv2`, then call `DisconnectNamedPipe(srv1)`.
            let mut old_rd_buf = [0x5Au8; 32];
            let mut ov_dup_rd: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                ReadFile(srv2, old_rd_buf.as_mut_ptr() as *mut c_void, old_rd_buf.len() as u32, std::ptr::null_mut(), &mut ov_dup_rd),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            // Large overlapped write on `srv2` fills the socketpair send buffer and pends.
            let big_wr = vec![0xA5u8; 512 * 1024];
            let mut ov_dup_wr: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                WriteFile(srv2, big_wr.as_ptr() as *const c_void, big_wr.len() as u32, std::ptr::null_mut(), &mut ov_dup_wr),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            // Capture a reference to the active connection to also verify post-disconnect submit safety.
            let old_conn = get_file(srv1)
                .unwrap()
                .named_pipe_server
                .as_ref()
                .unwrap()
                .io_conn()
                .unwrap();

            // Disconnect through `srv1` — MUST cancel all ops on the shared connection (including `srv2`!).
            assert_eq!(DisconnectNamedPipe(srv1), TRUE);
            assert!(old_conn.is_disconnected());

            let mut saw_rd_abort = false;
            let mut saw_wr_abort = false;
            for _ in 0..2 {
                assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), FALSE);
                assert_eq!(key, 0x9201);
                assert_eq!(teb::last_error(), ERROR_OPERATION_ABORTED);
                if ov_out == &mut ov_dup_rd as *mut OVERLAPPED {
                    saw_rd_abort = true;
                } else if ov_out == &mut ov_dup_wr as *mut OVERLAPPED {
                    saw_wr_abort = true;
                } else {
                    panic!("unexpected overlapped {ov_out:p}");
                }
            }
            assert!(saw_rd_abort && saw_wr_abort);

            // Submitting an op bound to an already-disconnected `PipeConn` (racing with DisconnectNamedPipe)
            // must immediately complete with ERROR_OPERATION_ABORTED and never target any new connection.
            let mut late_buf = [0x33u8; 16];
            let mut ov_late: OVERLAPPED = std::mem::zeroed();
            prepare_overlapped_pending(&mut ov_late);
            pipe_reactor().submit(PendingPipeOp::new_read(
                srv2,
                get_file(srv2).unwrap(),
                Some(Arc::clone(&old_conn)),
                old_conn.fd(),
                late_buf.as_mut_ptr() as usize,
                late_buf.len(),
                PipeDelivery::Overlapped { ov: &mut ov_late as *mut OVERLAPPED as usize, tid: teb::thread_id() },
            ));
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), FALSE);
            assert_eq!(ov_out, &mut ov_late as *mut OVERLAPPED);
            assert_eq!(teb::last_error(), ERROR_OPERATION_ABORTED);
            drop(old_conn);

            // Drain/close client 1; once buffered bytes are drained, client 1 observes ERROR_BROKEN_PIPE.
            assert_eq!(CloseHandle(cli1), TRUE);

            // 3. Reconnect a brand new client through duplicate handle `srv2`: old buffers stay
            //    untouched and the new connection exchanges data cleanly across both handles.
            let mut ov_conn2: OVERLAPPED = std::mem::zeroed();
            assert_eq!(ConnectNamedPipe(srv2, &mut ov_conn2), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            let cli2 = CreateFileW(
                name.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                0,
                std::ptr::null_mut(),
            );
            assert_ne!(cli2, INVALID_HANDLE_VALUE);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(ov_out, &mut ov_conn2 as *mut OVERLAPPED);

            let new_msg = b"NEW-CONN-DATA";
            assert_eq!(WriteFile(cli2, new_msg.as_ptr() as *const c_void, new_msg.len() as u32, &mut nw, std::ptr::null_mut()), TRUE);
            let mut new_rd = [0u8; 32];
            let mut nr = 0u32;
            assert_eq!(ReadFile(srv1, new_rd.as_mut_ptr() as *mut c_void, new_rd.len() as u32, &mut nr, std::ptr::null_mut()), TRUE);
            assert_eq!(&new_rd[..nr as usize], new_msg);
            assert_eq!(old_rd_buf, [0x5Au8; 32], "disconnected op buffer must never receive bytes from new connection");
            assert_eq!(late_buf, [0x33u8; 16], "late disconnected op buffer must remain untouched");

            let srv_reply = b"SRV-TO-CLI2";
            assert_eq!(WriteFile(srv2, srv_reply.as_ptr() as *const c_void, srv_reply.len() as u32, &mut nw, std::ptr::null_mut()), TRUE);
            let mut cli2_rd = [0u8; 32];
            assert_eq!(ReadFile(cli2, cli2_rd.as_mut_ptr() as *mut c_void, cli2_rd.len() as u32, &mut nr, std::ptr::null_mut()), TRUE);
            assert_eq!(&cli2_rd[..nr as usize], srv_reply);

            // DisconnectNamedPipe on non-pipe handle fails honestly with ERROR_INVALID_HANDLE.
            assert_eq!(DisconnectNamedPipe(iocp), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_HANDLE);

            assert_eq!(CloseHandle(cli2), TRUE);
            assert_eq!(DisconnectNamedPipe(srv1), TRUE);
            assert_eq!(CloseHandle(srv2), TRUE);
            assert_eq!(CloseHandle(srv1), TRUE);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn pipe_writes_serialize_fifo_across_mixed_apis_duplicates_cancel_and_distinct_pipes() {
        use crate::kernel32::threads::{CloseHandle, DuplicateHandle};
        use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering as AOrd};
        static APC_CALLS: AtomicUsize = AtomicUsize::new(0);
        static APC_ERR: AtomicU32 = AtomicU32::new(u32::MAX);
        static APC_BYTES: AtomicU32 = AtomicU32::new(0);
        static APC_OV: AtomicUsize = AtomicUsize::new(0);

        unsafe extern "win64" fn on_write_apc(err_code: u32, bytes: u32, ov: usize) {
            APC_ERR.store(err_code, AOrd::SeqCst);
            APC_BYTES.store(bytes, AOrd::SeqCst);
            APC_OV.store(ov, AOrd::SeqCst);
            APC_CALLS.fetch_add(1, AOrd::SeqCst);
        }

        unsafe {
            APC_CALLS.store(0, AOrd::SeqCst);
            APC_ERR.store(u32::MAX, AOrd::SeqCst);
            APC_BYTES.store(0, AOrd::SeqCst);
            APC_OV.store(0, AOrd::SeqCst);

            let pname: Vec<u16> = format!(
                "\\\\.\\pipe\\win32rt_write_fifo_test_{}\0",
                std::process::id()
            )
            .encode_utf16()
            .collect();
            let srv1 = CreateNamedPipeW(
                pname.as_ptr(),
                0x3 | 0x4000_0000, // PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED
                0,
                1,
                4096,
                4096,
                0,
                std::ptr::null_mut(),
            );
            assert_ne!(srv1, INVALID_HANDLE_VALUE);

            let mut srv2: HANDLE = std::ptr::null_mut();
            let cur_proc = crate::kernel32::process::GetCurrentProcess();
            assert_eq!(DuplicateHandle(cur_proc, srv1, cur_proc, &mut srv2, 0, FALSE, 2), TRUE);
            assert!(!srv2.is_null());
            assert_ne!(srv2, srv1);

            let iocp = CreateIoCompletionPort(srv1, std::ptr::null_mut(), 0x5150, 0);
            assert!(!iocp.is_null());

            let mut ov_conn: OVERLAPPED = std::mem::zeroed();
            assert_eq!(ConnectNamedPipe(srv1, &mut ov_conn), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            let cli = CreateFileW(
                pname.as_ptr(),
                0xC000_0000,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                std::ptr::null_mut(),
            );
            assert_ne!(cli, INVALID_HANDLE_VALUE);

            let mut bytes = 0u32;
            let mut key = 0usize;
            let mut ov_out: *mut OVERLAPPED = std::ptr::null_mut();
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), TRUE);
            assert_eq!(ov_out, &mut ov_conn as *mut OVERLAPPED);

            // 1. Backpressure on pipe 1: W1 (overlapped WriteFile on srv1), W2 (overlapped
            //    WriteFile on duplicate srv2), W3 (WriteFileEx APC on srv1), W4 (synchronous
            //    WriteFile on srv2 from another thread).
            let w1 = vec![0xA1u8; 256 * 1024];
            let w2 = vec![0xB2u8; 16 * 1024];
            let w3 = vec![0xC3u8; 8 * 1024];
            let w4 = vec![0xD4u8; 4 * 1024];

            let mut ov1: OVERLAPPED = std::mem::zeroed();
            let mut ov2: OVERLAPPED = std::mem::zeroed();
            let mut ov3: OVERLAPPED = std::mem::zeroed();
            let mut nw = 0u32;

            assert_eq!(
                WriteFile(srv1, w1.as_ptr() as *const c_void, w1.len() as u32, &mut nw, &mut ov1),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            // W2 on duplicate handle MUST queue behind W1 instead of bypassing it.
            assert_eq!(
                WriteFile(srv2, w2.as_ptr() as *const c_void, w2.len() as u32, &mut nw, &mut ov2),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            // W3 via WriteFileEx MUST also queue behind W2 (still STATUS_PENDING).
            assert_eq!(
                WriteFileEx(srv1, w3.as_ptr() as *const c_void, w3.len() as u32, &mut ov3, Some(on_write_apc)),
                TRUE
            );
            assert_eq!(load_ov_internal(&mut ov3), STATUS_PENDING);

            // W4 via synchronous WriteFile on duplicate handle from another thread must wait
            // behind W1..W3 without starving unrelated pipes.
            let sync_done = Arc::new(AtomicBool::new(false));
            let sync_done_bg = Arc::clone(&sync_done);
            let w4_bg = w4.clone();
            let srv2_raw = srv2 as usize;
            let sync_thread = std::thread::spawn(move || {
                let mut sw = 0u32;
                let ok = WriteFile(
                    srv2_raw as HANDLE,
                    w4_bg.as_ptr() as *const c_void,
                    w4_bg.len() as u32,
                    &mut sw,
                    std::ptr::null_mut(),
                );
                sync_done_bg.store(true, AOrd::Release);
                (ok, sw)
            });

            std::thread::sleep(std::time::Duration::from_millis(30));
            assert!(
                !sync_done.load(AOrd::Acquire),
                "synchronous WriteFile must wait behind earlier pending async writes"
            );

            // Distinct pipe P2 must remain completely responsive while P1's writes are backed up.
            let mut p2_r: HANDLE = std::ptr::null_mut();
            let mut p2_w: HANDLE = std::ptr::null_mut();
            assert_eq!(CreatePipe(&mut p2_r, &mut p2_w, std::ptr::null_mut(), 0), TRUE);
            let p2_msg = b"DISTINCT-PIPE-OK";
            let mut p2_nw = 0u32;
            assert_eq!(
                WriteFile(p2_w, p2_msg.as_ptr() as *const c_void, p2_msg.len() as u32, &mut p2_nw, std::ptr::null_mut()),
                TRUE
            );
            let mut p2_buf = [0u8; 32];
            let mut p2_nr = 0u32;
            assert_eq!(
                ReadFile(p2_r, p2_buf.as_mut_ptr() as *mut c_void, p2_buf.len() as u32, &mut p2_nr, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(&p2_buf[..p2_nr as usize], p2_msg);
            assert_eq!(CloseHandle(p2_r), TRUE);
            assert_eq!(CloseHandle(p2_w), TRUE);

            // Drain all bytes on the client end in a reader thread.
            let total_expected = w1.len() + w2.len() + w3.len() + w4.len();
            let cli_raw = cli as usize;
            let reader = std::thread::spawn(move || {
                let mut got = Vec::with_capacity(total_expected);
                let mut chunk = [0u8; 8192];
                while got.len() < total_expected {
                    let want = (total_expected - got.len()).min(chunk.len()) as u32;
                    let mut nr = 0u32;
                    let ok = ReadFile(cli_raw as HANDLE, chunk.as_mut_ptr() as *mut c_void, want, &mut nr, std::ptr::null_mut());
                    assert_eq!(ok, TRUE);
                    assert!(nr > 0);
                    got.extend_from_slice(&chunk[..nr as usize]);
                }
                got
            });

            let mut seen_ov = Vec::new();
            for _ in 0..2 {
                assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 5000), TRUE);
                seen_ov.push((ov_out as usize, bytes));
            }
            assert_eq!(
                seen_ov,
                vec![
                    (&mut ov1 as *mut OVERLAPPED as usize, w1.len() as u32),
                    (&mut ov2 as *mut OVERLAPPED as usize, w2.len() as u32),
                ],
                "overlapped WriteFile completions must fire in FIFO order"
            );

            // Deliver WriteFileEx APC completion for W3.
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
            while APC_CALLS.load(AOrd::SeqCst) == 0 && std::time::Instant::now() < deadline {
                let _ = crate::kernel32::time::SleepEx(50, TRUE);
            }
            assert_eq!(APC_CALLS.load(AOrd::SeqCst), 1);
            assert_eq!(APC_ERR.load(AOrd::SeqCst), 0);
            assert_eq!(APC_BYTES.load(AOrd::SeqCst), w3.len() as u32);
            assert_eq!(APC_OV.load(AOrd::SeqCst), &mut ov3 as *mut OVERLAPPED as usize);

            let (sync_ok, sync_written) = sync_thread.join().unwrap();
            assert_eq!(sync_ok, TRUE);
            assert_eq!(sync_written, w4.len() as u32);

            let received = reader.join().unwrap();
            assert_eq!(received.len(), total_expected);
            let off1 = w1.len();
            let off2 = off1 + w2.len();
            let off3 = off2 + w3.len();
            assert!(received[..off1].iter().all(|&b| b == 0xA1), "W1 bytes must remain contiguous");
            assert!(received[off1..off2].iter().all(|&b| b == 0xB2), "W2 bytes must follow W1");
            assert!(received[off2..off3].iter().all(|&b| b == 0xC3), "W3 bytes must follow W2");
            assert!(received[off3..].iter().all(|&b| b == 0xD4), "W4 sync bytes must follow W3");

            // 2. Canceling the head pending write advances the queued successor cleanly.
            let c1 = vec![0xE1u8; 256 * 1024];
            let c2 = vec![0xF6u8; 8 * 1024];
            let mut ov_c1: OVERLAPPED = std::mem::zeroed();
            let mut ov_c2: OVERLAPPED = std::mem::zeroed();
            assert_eq!(
                WriteFile(srv1, c1.as_ptr() as *const c_void, c1.len() as u32, &mut nw, &mut ov_c1),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);
            assert_eq!(
                WriteFile(srv2, c2.as_ptr() as *const c_void, c2.len() as u32, &mut nw, &mut ov_c2),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_IO_PENDING);

            assert_eq!(CancelIoEx(srv1, &mut ov_c1), TRUE);
            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 2000), FALSE);
            assert_eq!(teb::last_error(), ERROR_OPERATION_ABORTED);
            assert_eq!(ov_out, &mut ov_c1 as *mut OVERLAPPED);
            assert_eq!(load_ov_internal(&mut ov_c1), STATUS_CANCELLED);

            // Drain client while C2 completes; after C2 completes, write an 8-byte sentinel so
            // the reader thread exits deterministically with zero TOCTOU.
            const SENTINEL: &[u8; 8] = b"C2-DONE!";
            let drain_thread = std::thread::spawn(move || {
                let mut all = Vec::new();
                let mut chunk = [0u8; 8192];
                loop {
                    let mut nr = 0u32;
                    let ok = ReadFile(
                        cli_raw as HANDLE,
                        chunk.as_mut_ptr() as *mut c_void,
                        chunk.len() as u32,
                        &mut nr,
                        std::ptr::null_mut(),
                    );
                    if ok == FALSE || nr == 0 {
                        break;
                    }
                    all.extend_from_slice(&chunk[..nr as usize]);
                    if all.ends_with(SENTINEL) {
                        break;
                    }
                }
                all
            });

            assert_eq!(GetQueuedCompletionStatus(iocp, &mut bytes, &mut key, &mut ov_out, 3000), TRUE);
            assert_eq!(ov_out, &mut ov_c2 as *mut OVERLAPPED);
            assert_eq!(bytes, c2.len() as u32);
            assert_eq!(load_ov_internal(&mut ov_c2), STATUS_SUCCESS);
            assert_eq!(
                WriteFile(srv1, SENTINEL.as_ptr() as *const c_void, SENTINEL.len() as u32, &mut nw, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(nw, SENTINEL.len() as u32);

            let drained = drain_thread.join().unwrap();
            assert!(drained.ends_with(SENTINEL));
            let without_sentinel = &drained[..drained.len() - SENTINEL.len()];
            assert!(without_sentinel.len() >= c2.len());
            let tail = &without_sentinel[without_sentinel.len() - c2.len()..];
            assert!(tail.iter().all(|&b| b == 0xF6), "queued successor C2 must deliver intact after head cancel");

            assert_eq!(CloseHandle(cli), TRUE);
            assert_eq!(DisconnectNamedPipe(srv1), TRUE);
            assert_eq!(CloseHandle(srv2), TRUE);
            assert_eq!(CloseHandle(srv1), TRUE);
            assert_eq!(CloseHandle(iocp), TRUE);
        }
    }

    #[test]
    fn wait_named_pipe_w_synchronizes_disconnect_infinite_timeout_and_close() {
        use crate::kernel32::threads::CloseHandle;
        use std::sync::{mpsc, Barrier};

        unsafe {
            let pname = to_wide_z(&format!("\\\\.\\pipe\\aw2_wait_sync_test_{}", std::process::id()));
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

            // 1. Stress WaitNamedPipeW(INFINITE) on a connected server transitioning back to
            //    Listening via DisconnectNamedPipe across tight barrier-synchronized rounds and
            //    asleep-waiter rounds, guarded by a bounded watchdog so any lost wakeup fails fast.
            for round in 0..40 {
                let cli = CreateFileW(
                    pname.as_ptr(),
                    GENERIC_READ | GENERIC_WRITE,
                    0,
                    std::ptr::null_mut(),
                    OPEN_EXISTING,
                    0,
                    std::ptr::null_mut(),
                );
                assert_ne!(cli, INVALID_HANDLE_VALUE);

                let barrier = Arc::new(Barrier::new(2));
                let b_wait = Arc::clone(&barrier);
                let pname_w = pname.clone();
                let (tx, rx) = mpsc::channel();
                let waiter = std::thread::spawn(move || {
                    b_wait.wait();
                    let ok = WaitNamedPipeW(pname_w.as_ptr(), INFINITE);
                    let e = teb::last_error();
                    let _ = tx.send((ok, e));
                });

                barrier.wait();
                if round % 10 == 0 {
                    std::thread::sleep(Duration::from_millis(5));
                }
                assert_eq!(CloseHandle(cli), TRUE);
                assert_eq!(DisconnectNamedPipe(srv), TRUE);

                let (ok, e) = rx
                    .recv_timeout(Duration::from_secs(5))
                    .unwrap_or_else(|_| panic!("round {round}: WaitNamedPipeW(INFINITE) hung (lost wakeup)"));
                assert_eq!(ok, TRUE, "round {round}: WaitNamedPipeW failed with {e}");
                waiter.join().unwrap();
            }

            // 2. Finite timeout returns ERROR_SEM_TIMEOUT while the only server stays connected.
            let cli = CreateFileW(
                pname.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                0,
                std::ptr::null_mut(),
            );
            assert_ne!(cli, INVALID_HANDLE_VALUE);
            {
                let pname_w = pname.clone();
                let (tx, rx) = mpsc::channel();
                let waiter = std::thread::spawn(move || {
                    let ok = WaitNamedPipeW(pname_w.as_ptr(), 40);
                    let e = teb::last_error();
                    let _ = tx.send((ok, e));
                });
                let (ok, e) = rx
                    .recv_timeout(Duration::from_secs(5))
                    .expect("finite WaitNamedPipeW hung");
                assert_eq!(ok, FALSE);
                assert_eq!(e, ERROR_SEM_TIMEOUT);
                waiter.join().unwrap();
            }

            // 3. Closing the last connected server wakes WaitNamedPipeW(INFINITE) with ERROR_FILE_NOT_FOUND.
            {
                let barrier = Arc::new(Barrier::new(2));
                let b_wait = Arc::clone(&barrier);
                let pname_w = pname.clone();
                let (tx, rx) = mpsc::channel();
                let waiter = std::thread::spawn(move || {
                    b_wait.wait();
                    let ok = WaitNamedPipeW(pname_w.as_ptr(), INFINITE);
                    let e = teb::last_error();
                    let _ = tx.send((ok, e));
                });
                barrier.wait();
                std::thread::sleep(Duration::from_millis(5));
                assert_eq!(CloseHandle(srv), TRUE);

                let (ok, e) = rx
                    .recv_timeout(Duration::from_secs(5))
                    .expect("WaitNamedPipeW(INFINITE) hung after last server closed");
                assert_eq!(ok, FALSE);
                assert_eq!(e, err::ERROR_FILE_NOT_FOUND);
                waiter.join().unwrap();
            }

            assert_eq!(WaitNamedPipeW(pname.as_ptr(), 40), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_FILE_NOT_FOUND);
            assert_eq!(CloseHandle(cli), TRUE);
        }
    }

    #[cfg(target_vendor = "apple")]
    #[test]
    fn dir_watch_periodic_rescan_covers_files_beyond_vnode_fd_cap() {
        use crate::kernel32::threads::{CloseHandle, CreateEventW, ResetEvent, WaitForSingleObject};
        use std::io::{Seek, SeekFrom, Write};

        let dir = std::env::temp_dir().join(format!("aw2_dirwatch_cap_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // Populate 520 files (> MAX_WATCHED_VNODE_FDS = 512). `BTreeMap` orders lexicographically,
        // so root (1) + `f_0000.txt`..`f_0510.txt` (511) exhaust MAX_WATCHED_VNODE_FDS and
        // `z_omitted_target.txt` is deterministically omitted from kqueue vnode descriptors.
        for i in 0..519 {
            std::fs::write(dir.join(format!("f_{i:04}.txt")), b"a").unwrap();
        }
        let target_path = dir.join("z_omitted_target.txt");
        std::fs::write(&target_path, b"init").unwrap();

        // Preopen the writable target file before attaching watches so in-place writes never
        // depend on spare per-process fds while the watcher holds up to MAX_WATCHED_VNODE_FDS.
        let mut target_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&target_path)
            .unwrap();

        path::add_mount("U:\\", &dir);
        let win_dir = to_wide_z("U:\\");

        let parse_notify_records = |buf: &[u8], len: usize| -> Vec<(u32, String)> {
            let mut out = Vec::new();
            let mut off = 0usize;
            while off + 12 <= len {
                let next = u32::from_le_bytes(buf[off..off + 4].try_into().unwrap()) as usize;
                let action = u32::from_le_bytes(buf[off + 4..off + 8].try_into().unwrap());
                let name_bytes = u32::from_le_bytes(buf[off + 8..off + 12].try_into().unwrap()) as usize;
                let units: Vec<u16> = buf[off + 12..off + 12 + name_bytes]
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect();
                out.push((action, String::from_utf16_lossy(&units)));
                if next == 0 {
                    break;
                }
                off += next;
            }
            out
        };

        unsafe {
            // 1. ReadDirectoryChangesW: in-place updates on the deterministically omitted file
            //    (no rename, no sibling creation) must deliver FILE_ACTION_MODIFIED for both
            //    FILE_NOTIFY_CHANGE_SIZE and FILE_NOTIFY_CHANGE_LAST_WRITE via periodic rescan.
            let h_dir = CreateFileW(
                win_dir.as_ptr(),
                GENERIC_READ,
                0x7, // FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE
                std::ptr::null_mut(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS | 0x4000_0000, // FILE_FLAG_OVERLAPPED
                std::ptr::null_mut(),
            );
            assert_ne!(h_dir, INVALID_HANDLE_VALUE);
            let ev = CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null());
            assert!(!ev.is_null());

            // 1a. Size + LastWrite growth in-place.
            let mut buf = [0u8; 4096];
            let mut ov: OVERLAPPED = std::mem::zeroed();
            ov.h_event = ev;
            assert_eq!(
                ReadDirectoryChangesW(
                    h_dir,
                    buf.as_mut_ptr() as *mut c_void,
                    buf.len() as u32,
                    FALSE,
                    FILE_NOTIFY_CHANGE_SIZE,
                    std::ptr::null_mut(),
                    &mut ov,
                    std::ptr::null(),
                ),
                TRUE
            );
            target_file.seek(SeekFrom::Start(0)).unwrap();
            target_file.write_all(b"updated-size-payload").unwrap();
            target_file.flush().unwrap();

            assert_eq!(
                WaitForSingleObject(ev, 3000),
                WAIT_OBJECT_0,
                "ReadDirectoryChangesW(SIZE) missed in-place update on file beyond vnode cap"
            );
            let mut got = 0u32;
            assert_eq!(GetOverlappedResult(h_dir, &mut ov, &mut got, FALSE), TRUE);
            let recs = parse_notify_records(&buf, got as usize);
            assert!(
                recs.iter().any(|(a, p)| *a == FILE_ACTION_MODIFIED && p == "z_omitted_target.txt"),
                "expected FILE_ACTION_MODIFIED for z_omitted_target.txt (SIZE), got {recs:?}"
            );
            assert!(
                !recs.iter().any(|(a, _)| *a == FILE_ACTION_REMOVED || *a == FILE_ACTION_ADDED),
                "unexpected spurious add/remove under FD pressure: {recs:?}"
            );

            // 1b. Same-size in-place LastWrite update (no size change, no rename/create).
            assert_eq!(ResetEvent(ev), TRUE);
            buf.fill(0);
            ov = std::mem::zeroed();
            ov.h_event = ev;
            assert_eq!(
                ReadDirectoryChangesW(
                    h_dir,
                    buf.as_mut_ptr() as *mut c_void,
                    buf.len() as u32,
                    FALSE,
                    FILE_NOTIFY_CHANGE_LAST_WRITE,
                    std::ptr::null_mut(),
                    &mut ov,
                    std::ptr::null(),
                ),
                TRUE
            );
            std::thread::sleep(Duration::from_millis(5));
            target_file.seek(SeekFrom::Start(0)).unwrap();
            target_file.write_all(b"UPDATED-SIZE-PAYLOAD").unwrap();
            target_file.flush().unwrap();

            assert_eq!(
                WaitForSingleObject(ev, 3000),
                WAIT_OBJECT_0,
                "ReadDirectoryChangesW(LAST_WRITE) missed same-size in-place update beyond vnode cap"
            );
            got = 0;
            assert_eq!(GetOverlappedResult(h_dir, &mut ov, &mut got, FALSE), TRUE);
            let recs2 = parse_notify_records(&buf, got as usize);
            assert!(
                recs2.iter().any(|(a, p)| *a == FILE_ACTION_MODIFIED && p == "z_omitted_target.txt"),
                "expected FILE_ACTION_MODIFIED for z_omitted_target.txt (LAST_WRITE), got {recs2:?}"
            );

            assert_eq!(CloseHandle(ev), TRUE);
            assert_eq!(CloseHandle(h_dir), TRUE);
            // Allow first watcher thread to exit and release its vnode fds before next watch.
            std::thread::sleep(Duration::from_millis(50));

            // 2. FindFirstChangeNotificationW sequentially for SIZE and LAST_WRITE on the same
            //    omitted late-sort file.
            let ch_size = FindFirstChangeNotificationW(win_dir.as_ptr(), FALSE, FILE_NOTIFY_CHANGE_SIZE);
            assert_ne!(ch_size, INVALID_HANDLE_VALUE);
            assert_eq!(WaitForSingleObject(ch_size, 0), WAIT_TIMEOUT);
            target_file.seek(SeekFrom::Start(0)).unwrap();
            target_file.write_all(b"updated-size-payload-grown").unwrap();
            target_file.flush().unwrap();
            assert_eq!(
                WaitForSingleObject(ch_size, 3000),
                WAIT_OBJECT_0,
                "FindFirstChangeNotificationW(SIZE) missed in-place growth beyond vnode cap"
            );
            assert_eq!(FindCloseChangeNotification(ch_size), TRUE);
            std::thread::sleep(Duration::from_millis(50));

            let ch_write = FindFirstChangeNotificationW(win_dir.as_ptr(), FALSE, FILE_NOTIFY_CHANGE_LAST_WRITE);
            assert_ne!(ch_write, INVALID_HANDLE_VALUE);
            assert_eq!(WaitForSingleObject(ch_write, 0), WAIT_TIMEOUT);
            std::thread::sleep(Duration::from_millis(5));
            target_file.seek(SeekFrom::Start(0)).unwrap();
            target_file.write_all(b"UPDATED-SIZE-PAYLOAD-GROWN").unwrap();
            target_file.flush().unwrap();
            assert_eq!(
                WaitForSingleObject(ch_write, 3000),
                WAIT_OBJECT_0,
                "FindFirstChangeNotificationW(LAST_WRITE) missed in-place write beyond vnode cap"
            );
            assert_eq!(FindCloseChangeNotification(ch_write), TRUE);
        }

        drop(target_file);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn alloc_free_attach_console_lifecycle() {
        unsafe {
            assert_eq!(FreeConsole(), TRUE);
            assert_eq!(AllocConsole(), TRUE);
            // Second AllocConsole while attached fails with ERROR_ACCESS_DENIED.
            assert_eq!(AllocConsole(), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);

            let h_in = GetStdHandle(STD_INPUT_HANDLE);
            let h_out = GetStdHandle(STD_OUTPUT_HANDLE);
            assert!(is_console(h_in));
            assert!(is_console(h_out));

            let mut mode = 0u32;
            assert_eq!(SetConsoleMode(h_out, 0x0005), TRUE);
            assert_eq!(GetConsoleMode(h_out, &mut mode), TRUE);
            assert_eq!(mode, 0x0005);

            assert_eq!(SetConsoleCP(1252), TRUE);
            assert_eq!(GetConsoleCP(), 1252);
            assert_eq!(SetConsoleOutputCP(65001), TRUE);
            assert_eq!(GetConsoleOutputCP(), 65001);
            assert_eq!(SetConsoleCP(999999), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
            assert_eq!(SetConsoleCP(ACP), TRUE);

            assert_eq!(FreeConsole(), TRUE);
            // AttachConsole with bogus pid fails with ERROR_INVALID_HANDLE.
            assert_eq!(AttachConsole(0xDEAD_BEEF), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_HANDLE);
            // AttachConsole to parent succeeds once, then fails with ERROR_ACCESS_DENIED.
            assert_eq!(AttachConsole(0xFFFF_FFFF), TRUE);
            assert_eq!(AttachConsole(0xFFFF_FFFF), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(FreeConsole(), TRUE);
        }
    }

    #[test]
    fn file_info_by_handle_ex_all_classes() {
        let dir = std::env::temp_dir().join(format!("win32_rt_finfo_ex_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file_p = dir.join("sample.dat");
        std::fs::write(&file_p, b"0123456789abcdef").unwrap();
        let link_p = dir.join("sym.dat");
        let _ = std::os::unix::fs::symlink(&file_p, &link_p);
        crate::path::add_mount("K:\\", &dir);

        unsafe {
            let wf = to_wide_z("K:\\sample.dat");
            let h = CreateFileW(wf.as_ptr(), GENERIC_READ | GENERIC_WRITE, 7, std::ptr::null_mut(), OPEN_EXISTING, 0x80, std::ptr::null_mut());
            assert_ne!(h, INVALID_HANDLE_VALUE);

            // Class 7: FileStreamInfo -> "::$DATA"
            let mut sbuf = [0u8; 64];
            assert_eq!(GetFileInformationByHandleEx(h, 7, sbuf.as_mut_ptr(), sbuf.len() as u32), TRUE);
            let name_len = (sbuf.as_ptr().add(4) as *const u32).read_unaligned() as usize;
            let stream_sz = (sbuf.as_ptr().add(8) as *const u64).read_unaligned();
            let sname_u16 = crate::strings::read_wide_slice(sbuf.as_ptr().add(24) as *const u16, name_len / 2);
            assert_eq!(String::from_utf16_lossy(&sname_u16), "::$DATA");
            assert_eq!(stream_sz, 16);

            // Class 8: FileCompressionInfo
            let mut cbuf = [0u8; 16];
            assert_eq!(GetFileInformationByHandleEx(h, 8, cbuf.as_mut_ptr(), 16), TRUE);
            assert_eq!((cbuf.as_ptr() as *const u64).read_unaligned(), 16);
            assert_eq!((cbuf.as_ptr().add(8) as *const u16).read_unaligned(), 0);

            // Class 13: FileRemoteProtocolInfo fails on local volume with ERROR_INVALID_PARAMETER
            let mut rbuf = [0u8; 128];
            assert_eq!(GetFileInformationByHandleEx(h, 13, rbuf.as_mut_ptr(), 128), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Class 16: FileStorageInfo reports real volume sector sizes (>= 512)
            let mut stbuf = [0u8; 28];
            assert_eq!(GetFileInformationByHandleEx(h, 16, stbuf.as_mut_ptr(), 28), TRUE);
            let logical_bps = (stbuf.as_ptr() as *const u32).read_unaligned();
            let phys_bps = (stbuf.as_ptr().add(4) as *const u32).read_unaligned();
            assert!(logical_bps >= 512 && phys_bps >= 512);

            // Class 23: FileCaseSensitiveInfo Get & Set
            let mut cs = 0u32;
            assert_eq!(GetFileInformationByHandleEx(h, 23, &mut cs as *mut u32 as *mut u8, 4), TRUE);
            assert!(cs == 0 || cs == 1);
            assert_eq!(SetFileInformationByHandle(h, 23, &cs as *const u32 as *const u8, 4), TRUE);
            let flipped = cs ^ 1;
            assert_eq!(SetFileInformationByHandle(h, 23, &flipped as *const u32 as *const u8, 4), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_NOT_SUPPORTED);

            // Class 24: FileNormalizedNameInfo
            let mut nbuf = [0u8; 128];
            assert_eq!(GetFileInformationByHandleEx(h, 24, nbuf.as_mut_ptr(), 128), TRUE);
            let nlen = (nbuf.as_ptr() as *const u32).read_unaligned() as usize;
            let nstr = String::from_utf16_lossy(&crate::strings::read_wide_slice(nbuf.as_ptr().add(4) as *const u16, nlen / 2));
            assert_eq!(nstr, "\\sample.dat");

            // Class 12: FileIoPriorityHintInfo (Set)
            let prio_low = 1u32;
            assert_eq!(SetFileInformationByHandle(h, 12, &prio_low as *const u32 as *const u8, 4), TRUE);
            let prio_bad = 9u32;
            assert_eq!(SetFileInformationByHandle(h, 12, &prio_bad as *const u32 as *const u8, 4), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Invalid class fails with ERROR_INVALID_PARAMETER
            assert_eq!(GetFileInformationByHandleEx(h, 99, nbuf.as_mut_ptr(), 64), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
            assert_eq!(SetFileInformationByHandle(h, 99, nbuf.as_ptr(), 64), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Directory enumeration classes 10/11 (IdBoth), 14/15 (Full), 19/20 (IdExtd)
            let wd = to_wide_z("K:\\");
            let hd = CreateFileW(wd.as_ptr(), GENERIC_READ, 7, std::ptr::null_mut(), OPEN_EXISTING, FILE_FLAG_BACKUP_SEMANTICS, std::ptr::null_mut());
            assert_ne!(hd, INVALID_HANDLE_VALUE);
            for (restart_cls, next_cls, name_off) in [(11u32, 10u32, 104usize), (15, 14, 68), (20, 19, 88)] {
                let mut dbuf = vec![0u8; 4096];
                let mut names = Vec::new();
                let mut cls = restart_cls;
                while GetFileInformationByHandleEx(hd, cls, dbuf.as_mut_ptr(), dbuf.len() as u32) != 0 {
                    let mut off = 0usize;
                    loop {
                        let next_off = (dbuf.as_ptr().add(off) as *const u32).read_unaligned() as usize;
                        let flen = (dbuf.as_ptr().add(off + 60) as *const u32).read_unaligned() as usize;
                        let u = crate::strings::read_wide_slice(dbuf.as_ptr().add(off + name_off) as *const u16, flen / 2);
                        names.push(String::from_utf16_lossy(&u));
                        if next_off == 0 {
                            break;
                        }
                        off += next_off;
                    }
                    cls = next_cls;
                }
                assert_eq!(teb::last_error(), err::ERROR_NO_MORE_FILES);
                assert!(names.iter().any(|n| n == "sample.dat"), "class {restart_cls} missed sample.dat: {names:?}");
            }

            // Class 22: FileRenameInfoEx relative to RootDirectory
            let mut ren = vec![0u8; 64];
            (ren.as_mut_ptr() as *mut u32).write_unaligned(1); // FILE_RENAME_FLAG_REPLACE_IF_EXISTS
            (ren.as_mut_ptr().add(8) as *mut HANDLE).write_unaligned(hd);
            let new_w = to_wide("renamed.dat");
            (ren.as_mut_ptr().add(16) as *mut u32).write_unaligned((new_w.len() * 2) as u32);
            std::ptr::copy_nonoverlapping(new_w.as_ptr() as *const u8, ren.as_mut_ptr().add(20), new_w.len() * 2);
            assert_eq!(SetFileInformationByHandle(h, 22, ren.as_ptr(), ren.len() as u32), TRUE);
            assert!(dir.join("renamed.dat").exists());

            // Class 5: FileAllocationInfo repeated same-size reservation does not grow allocation.
            let wp = to_wide_z("K:\\prealloc.dat");
            let hp = CreateFileW(wp.as_ptr(), GENERIC_READ | GENERIC_WRITE, 7, std::ptr::null_mut(), CREATE_ALWAYS, 0x80, std::ptr::null_mut());
            assert_ne!(hp, INVALID_HANDLE_VALUE);
            let want_alloc: i64 = 65536;
            assert_eq!(SetFileInformationByHandle(hp, 5, &want_alloc as *const i64 as *const u8, 8), TRUE);
            let f_pre = get_file(hp).unwrap();
            let mut st1: libc::stat = std::mem::zeroed();
            assert_eq!(libc::fstat(f_pre.fd(), &mut st1), 0);
            let alloc1 = (st1.st_blocks as i64) * 512;
            assert!(alloc1 >= want_alloc, "first preallocate should reserve >= {want_alloc}, got {alloc1}");
            assert_eq!(SetFileInformationByHandle(hp, 5, &want_alloc as *const i64 as *const u8, 8), TRUE);
            let mut st2: libc::stat = std::mem::zeroed();
            assert_eq!(libc::fstat(f_pre.fd(), &mut st2), 0);
            let alloc2 = (st2.st_blocks as i64) * 512;
            assert_eq!(alloc2, alloc1, "repeated FileAllocationInfo must not over-allocate");
            drop(f_pre);
            crate::kernel32::threads::CloseHandle(hp);

            crate::kernel32::threads::CloseHandle(h);
            crate::kernel32::threads::CloseHandle(hd);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn move_file_ex_delay_until_reboot_and_null_dst() {
        let dir = std::env::temp_dir().join(format!("win32_rt_moveex_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.txt"), b"hello").unwrap();
        crate::path::add_mount("R:\\", &dir);

        unsafe {
            let src_w = to_wide_z("R:\\a.txt");
            let dst_w = to_wide_z("R:\\b.txt");
            // MOVEFILE_DELAY_UNTIL_REBOOT as non-admin returns FALSE + ERROR_ACCESS_DENIED.
            assert_eq!(MoveFileExW(src_w.as_ptr(), std::ptr::null(), MOVEFILE_DELAY_UNTIL_REBOOT), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            assert_eq!(MoveFileExA(b"R:\\a.txt\0".as_ptr(), std::ptr::null(), MOVEFILE_DELAY_UNTIL_REBOOT), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_ACCESS_DENIED);
            // Combining MOVEFILE_DELAY_UNTIL_REBOOT with MOVEFILE_COPY_ALLOWED -> ERROR_INVALID_PARAMETER.
            assert_eq!(
                MoveFileExW(src_w.as_ptr(), dst_w.as_ptr(), MOVEFILE_DELAY_UNTIL_REBOOT | MOVEFILE_COPY_ALLOWED),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
            // NULL dst without MOVEFILE_DELAY_UNTIL_REBOOT -> ERROR_INVALID_PARAMETER.
            assert_eq!(MoveFileExW(src_w.as_ptr(), std::ptr::null(), 0), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
            assert_eq!(MoveFileExA(b"R:\\a.txt\0".as_ptr(), std::ptr::null(), 0), FALSE);
            assert_eq!(teb::last_error(), err::ERROR_INVALID_PARAMETER);
            // Normal move with MOVEFILE_WRITE_THROUGH succeeds.
            assert_eq!(MoveFileExW(src_w.as_ptr(), dst_w.as_ptr(), MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH), TRUE);
            assert!(dir.join("b.txt").exists());
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn map_view_of_file3_process_handle_semantics() {
        unsafe {
            let hmap = CreateFileMappingW(INVALID_HANDLE_VALUE, std::ptr::null_mut(), 0x04, 0, 4096, std::ptr::null());
            assert!(!hmap.is_null());

            // Own process handle from OpenProcess succeeds.
            let h_self = crate::kernel32::process::OpenProcess(0x001F_0FFF, FALSE, std::process::id());
            assert!(!h_self.is_null());
            let p = MapViewOfFile3(hmap, h_self, std::ptr::null_mut(), 0, 4096, 0, 0x04, std::ptr::null_mut(), 0);
            assert!(!p.is_null());
            assert_eq!(UnmapViewOfFile(p), TRUE);
            crate::kernel32::threads::CloseHandle(h_self);

            // Bogus handle fails with ERROR_INVALID_HANDLE.
            let bad = MapViewOfFile3(hmap, 0x12345 as HANDLE, std::ptr::null_mut(), 0, 4096, 0, 0x04, std::ptr::null_mut(), 0);
            assert!(bad.is_null());
            assert_eq!(teb::last_error(), err::ERROR_INVALID_HANDLE);

            crate::kernel32::threads::CloseHandle(hmap);
        }
    }

    #[test]
    fn device_io_control_real_storage_and_fsctls() {
        let dir = std::env::temp_dir().join(format!("win32_rt_ioctl_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file_p = dir.join("disk.bin");
        std::fs::write(&file_p, vec![0x5Au8; 8192]).unwrap();
        let link_p = dir.join("link.bin");
        std::os::unix::fs::symlink(&file_p, &link_p).unwrap();
        crate::path::add_mount("V:\\", &dir);

        unsafe {
            let dev_w = to_wide_z("\\\\.\\V:");
            let hdev = CreateFileW(dev_w.as_ptr(), 0, 3, std::ptr::null_mut(), OPEN_EXISTING, 0, std::ptr::null_mut());
            assert_ne!(hdev, INVALID_HANDLE_VALUE);

            // IOCTL_STORAGE_QUERY_PROPERTY: StorageDeviceProperty (0), WriteCache (4), IoCapability (17)
            for (prop, min_sz) in [(0u32, 40u32), (1, 32), (4, 28), (6, 28), (7, 12), (8, 12), (17, 16)] {
                let query = [prop, 0u32, 0u32]; // PropertyStandardQuery
                let mut out = [0u8; 256];
                let mut ret = 0u32;
                assert_eq!(
                    DeviceIoControl(
                        hdev,
                        IOCTL_STORAGE_QUERY_PROPERTY,
                        query.as_ptr() as *const c_void,
                        12,
                        out.as_mut_ptr() as *mut c_void,
                        out.len() as u32,
                        &mut ret,
                        std::ptr::null_mut(),
                    ),
                    TRUE,
                    "IOCTL_STORAGE_QUERY_PROPERTY prop={prop}"
                );
                assert!(ret >= min_sz, "prop={prop} ret={ret} < {min_sz}");
            }

            // Unsupported property ID returns FALSE + ERROR_NOT_SUPPORTED.
            let bad_q = [50u32, 0, 0];
            let mut out = [0u8; 64];
            let mut ret = 0u32;
            assert_eq!(
                DeviceIoControl(
                    hdev,
                    IOCTL_STORAGE_QUERY_PROPERTY,
                    bad_q.as_ptr() as *const c_void,
                    12,
                    out.as_mut_ptr() as *mut c_void,
                    64,
                    &mut ret,
                    std::ptr::null_mut(),
                ),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_NOT_SUPPORTED);

            // IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS returns real volume size > 0
            let mut ext = [0u8; 32];
            assert_eq!(
                DeviceIoControl(hdev, IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS, std::ptr::null(), 0, ext.as_mut_ptr() as _, 32, &mut ret, std::ptr::null_mut()),
                TRUE
            );
            let vol_len = (ext.as_ptr().add(24) as *const u64).read_unaligned();
            assert!(vol_len > 0);

            // IOCTL_DISK_GET_DRIVE_GEOMETRY_EX, IOCTL_DISK_GET_LENGTH_INFO, FSCTL_GET_NTFS_VOLUME_DATA
            let mut gex = [0u8; 32];
            assert_eq!(
                DeviceIoControl(hdev, IOCTL_DISK_GET_DRIVE_GEOMETRY_EX, std::ptr::null(), 0, gex.as_mut_ptr() as _, 32, &mut ret, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!((gex.as_ptr().add(24) as *const i64).read_unaligned() as u64, vol_len);

            let mut ntfs = [0u8; 96];
            assert_eq!(
                DeviceIoControl(hdev, FSCTL_GET_NTFS_VOLUME_DATA, std::ptr::null(), 0, ntfs.as_mut_ptr() as _, 96, &mut ret, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(ret, 96);

            // Unrecognized IOCTL returns FALSE + ERROR_INVALID_FUNCTION
            assert_eq!(
                DeviceIoControl(hdev, 0x00FF_1234, std::ptr::null(), 0, std::ptr::null_mut(), 0, &mut ret, std::ptr::null_mut()),
                FALSE
            );
            assert_eq!(teb::last_error(), err::ERROR_INVALID_FUNCTION);
            crate::kernel32::threads::CloseHandle(hdev);

            // File-level FSCTLs: FSCTL_SET_ZERO_DATA, FSCTL_QUERY_ALLOCATED_RANGES, FSCTL_GET_REPARSE_POINT
            let wf = to_wide_z("V:\\disk.bin");
            let hf = CreateFileW(wf.as_ptr(), GENERIC_READ | GENERIC_WRITE, 7, std::ptr::null_mut(), OPEN_EXISTING, 0x80, std::ptr::null_mut());
            assert_ne!(hf, INVALID_HANDLE_VALUE);
            // Non-reparse point fails with ERROR_NOT_A_REPARSE_POINT (4390).
            assert_eq!(
                DeviceIoControl(hf, FSCTL_GET_REPARSE_POINT, std::ptr::null(), 0, out.as_mut_ptr() as _, 64, &mut ret, std::ptr::null_mut()),
                FALSE
            );
            assert_eq!(teb::last_error(), ERROR_NOT_A_REPARSE_POINT);

            let zero_req: [i64; 2] = [100, 200];
            assert_eq!(
                DeviceIoControl(hf, FSCTL_SET_ZERO_DATA, zero_req.as_ptr() as _, 16, std::ptr::null_mut(), 0, &mut ret, std::ptr::null_mut()),
                TRUE
            );
            let data = std::fs::read(&file_p).unwrap();
            assert_eq!(data[99], 0x5A);
            assert!(data[100..200].iter().all(|&b| b == 0));
            assert_eq!(data[200], 0x5A);

            let q_range: [i64; 2] = [0, 8192];
            let mut r_out = [0i64; 4];
            assert_eq!(
                DeviceIoControl(hf, FSCTL_QUERY_ALLOCATED_RANGES, q_range.as_ptr() as _, 16, r_out.as_mut_ptr() as _, 32, &mut ret, std::ptr::null_mut()),
                TRUE
            );
            assert!(ret >= 16 && r_out[1] > 0);
            crate::kernel32::threads::CloseHandle(hf);

            // Symlink FSCTL_GET_REPARSE_POINT translates absolute host target into guest namespace.
            let wl = to_wide_z("V:\\link.bin");
            let hl = CreateFileW(wl.as_ptr(), GENERIC_READ, 7, std::ptr::null_mut(), OPEN_EXISTING, 0x80, std::ptr::null_mut());
            assert_ne!(hl, INVALID_HANDLE_VALUE);
            let mut rp = [0u8; 512];
            assert_eq!(
                DeviceIoControl(hl, FSCTL_GET_REPARSE_POINT, std::ptr::null(), 0, rp.as_mut_ptr() as _, 512, &mut ret, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!((rp.as_ptr() as *const u32).read_unaligned(), IO_REPARSE_TAG_SYMLINK);
            let sub_off = (rp.as_ptr().add(8) as *const u16).read_unaligned() as usize;
            let sub_len = (rp.as_ptr().add(10) as *const u16).read_unaligned() as usize;
            let prt_off = (rp.as_ptr().add(12) as *const u16).read_unaligned() as usize;
            let prt_len = (rp.as_ptr().add(14) as *const u16).read_unaligned() as usize;
            let sym_flags = (rp.as_ptr().add(16) as *const u32).read_unaligned();
            assert_eq!(sym_flags, 0); // absolute symlink
            let sub_s = String::from_utf16_lossy(&crate::strings::read_wide_slice(rp.as_ptr().add(20 + sub_off) as *const u16, sub_len / 2));
            let prt_s = String::from_utf16_lossy(&crate::strings::read_wide_slice(rp.as_ptr().add(20 + prt_off) as *const u16, prt_len / 2));
            assert_eq!(sub_s, "\\??\\V:\\disk.bin");
            assert_eq!(prt_s, "V:\\disk.bin");
            crate::kernel32::threads::CloseHandle(hl);

            // Relative symlink sets SYMLINK_FLAG_RELATIVE (1) and preserves relative path.
            let rel_link = dir.join("rel.bin");
            std::os::unix::fs::symlink("disk.bin", &rel_link).unwrap();
            let wr = to_wide_z("V:\\rel.bin");
            let hr = CreateFileW(wr.as_ptr(), GENERIC_READ, 7, std::ptr::null_mut(), OPEN_EXISTING, 0x80, std::ptr::null_mut());
            assert_ne!(hr, INVALID_HANDLE_VALUE);
            rp.fill(0);
            assert_eq!(
                DeviceIoControl(hr, FSCTL_GET_REPARSE_POINT, std::ptr::null(), 0, rp.as_mut_ptr() as _, 512, &mut ret, std::ptr::null_mut()),
                TRUE
            );
            assert_eq!((rp.as_ptr().add(16) as *const u32).read_unaligned(), 1);
            let r_sub_len = (rp.as_ptr().add(10) as *const u16).read_unaligned() as usize;
            let r_sub_s = String::from_utf16_lossy(&crate::strings::read_wide_slice(rp.as_ptr().add(20) as *const u16, r_sub_len / 2));
            assert_eq!(r_sub_s, "disk.bin");
            crate::kernel32::threads::CloseHandle(hr);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn remote_protocol_info_matches_win32_abi_header_order() {
        unsafe {
            let mut buf = [0xAAu8; 116];
            write_remote_protocol_info(buf.as_mut_ptr(), "smbfs");
            assert_eq!((buf.as_ptr() as *const u16).read_unaligned(), 2); // StructureVersion @0
            assert_eq!((buf.as_ptr().add(2) as *const u16).read_unaligned(), 116); // StructureSize @2
            assert_eq!((buf.as_ptr().add(4) as *const u32).read_unaligned(), 0x0002_0000); // WNNC_NET_LANMAN @4
            assert_eq!((buf.as_ptr().add(8) as *const u16).read_unaligned(), 3); // ProtocolMajorVersion @8
            assert_eq!((buf.as_ptr().add(10) as *const u16).read_unaligned(), 0); // ProtocolMinorVersion @10

            write_remote_protocol_info(buf.as_mut_ptr(), "nfs");
            assert_eq!((buf.as_ptr() as *const u16).read_unaligned(), 2);
            assert_eq!((buf.as_ptr().add(2) as *const u16).read_unaligned(), 116);
            assert_eq!((buf.as_ptr().add(4) as *const u32).read_unaligned(), 0x0007_0000); // WNNC_NET_SUN_PC_NFS @4
        }
    }
}
