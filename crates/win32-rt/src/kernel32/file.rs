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
const DARWIN_F_SETNOSIGPIPE: libc::c_int = 73;
#[cfg(target_vendor = "apple")]
const DARWIN_SO_NOSIGPIPE: libc::c_int = 0x1022;

fn ntstatus_to_win32_local(s: usize) -> u32 {
    match s {
        STATUS_SUCCESS => 0,
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

fn set_nosigpipe_pipe(fd: i32) {
    #[cfg(target_vendor = "apple")]
    unsafe {
        let _ = libc::fcntl(fd, DARWIN_F_SETNOSIGPIPE, 1);
    }
    #[cfg(not(target_vendor = "apple"))]
    let _ = fd;
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
    iocp: Mutex<Option<(Arc<IoCompletionPort>, usize)>>,
    tp_io: Mutex<Option<Arc<TpIo>>>,
    completion_modes: AtomicU8,
    /// Manual-reset event representing handle completion state (used when
    /// `GetOverlappedResultEx` waits on the file handle itself with `ov.h_event == NULL`).
    io_event: Arc<Event>,
    /// Unread peek buffer for pipes (`PeekNamedPipe` fills; `ReadFile`/`ReadFileEx` drains first).
    peek_buf: Mutex<VecDeque<u8>>,
    /// Lazily started kqueue directory watcher for `ReadDirectoryChangesW`.
    dir_watcher: Mutex<Option<Arc<DirWatcher>>>,
    /// Present for server-side named pipe handles created via `CreateNamedPipeW`/`A`.
    named_pipe_server: Option<Arc<NamedPipeServer>>,
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
            iocp: Mutex::new(None),
            tp_io: Mutex::new(None),
            completion_modes: AtomicU8::new(0),
            io_event: Arc::new(Event::new(true, true, None)),
            peek_buf: Mutex::new(VecDeque::new()),
            dir_watcher: Mutex::new(None),
            named_pipe_server: None,
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
    /// Return the active raw POSIX fd for I/O, or a Win32 pipe state error if
    /// this is an unconnected named-pipe server.
    fn io_fd(&self) -> Result<i32, u32> {
        if let Some(ref srv) = self.named_pipe_server {
            srv.io_fd()
        } else {
            Ok(self.file.as_raw_fd())
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
                let _ = if self.kind == FileKind::Dir { std::fs::remove_dir(p) } else { std::fs::remove_file(p) };
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
        Some(&*self.io_event)
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

/// Wrap an existing POSIX fd in a non-owning `FileObj` handle (used by CRT `_get_osfhandle` / inherited stdio).
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

fn is_console(h: HANDLE) -> bool {
    get_file(h).is_some_and(|f| f.kind == FileKind::Console)
}

pub unsafe extern "win64" fn GetConsoleMode(h: HANDLE, mode: *mut u32) -> BOOL {
    if !is_console(h) {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    if !mode.is_null() {
        // ENABLE_PROCESSED_OUTPUT | ENABLE_WRAP_AT_EOL_OUTPUT | VT processing
        *mode = 0x7;
    }
    TRUE
}

pub unsafe extern "win64" fn SetConsoleMode(h: HANDLE, _mode: u32) -> BOOL {
    if !is_console(h) {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
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
    ACP
}
pub unsafe extern "win64" fn GetConsoleOutputCP() -> u32 {
    ACP
}
pub unsafe extern "win64" fn SetConsoleCP(_cp: u32) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetConsoleOutputCP(_cp: u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn AllocConsole() -> BOOL {
    stub!("no separate console window; stdout/stderr of the loader act as the console");
    TRUE
}
pub unsafe extern "win64" fn FreeConsole() -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn AttachConsole(_pid: u32) -> BOOL {
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
    // COORD dwSize, COORD cursor, WORD attr, SMALL_RECT window, COORD max
    let vals: [u16; 11] = [120, 9000, 0, 0, 7, 0, 0, 119, 49, 120, 50];
    std::ptr::copy_nonoverlapping(vals.as_ptr(), info, 11);
    TRUE
}

pub unsafe extern "win64" fn SetConsoleTextAttribute(_h: HANDLE, _a: u16) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetConsoleTitleA(t: LPCSTR) -> BOOL {
    log::info!("SetConsoleTitle: {}", from_ansi(t));
    TRUE
}
pub unsafe extern "win64" fn SetConsoleTitleW(t: LPCWSTR) -> BOOL {
    log::info!("SetConsoleTitle: {}", from_wide(t));
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
pub(crate) const STATUS_PENDING: usize = 0x103;
pub(crate) const STATUS_END_OF_FILE: usize = 0xC000_0011;
pub(crate) const STATUS_CANCELLED: usize = 0xC000_0120;

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
unsafe fn store_ov_internal(ov: *mut OVERLAPPED, val: usize) {
    let p = std::ptr::addr_of_mut!((*ov).internal);
    if p.is_aligned() {
        std::sync::atomic::AtomicUsize::from_ptr(p).store(val, Ordering::Release);
    } else {
        std::sync::atomic::fence(Ordering::Release);
        std::ptr::write_unaligned(p, val);
    }
}

#[inline]
unsafe fn load_ov_internal(ov: *mut OVERLAPPED) -> usize {
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
unsafe fn store_ov_internal_high(ov: *mut OVERLAPPED, val: usize) {
    let p = std::ptr::addr_of_mut!((*ov).internal_high);
    if p.is_aligned() {
        std::ptr::write_volatile(p, val);
    } else {
        std::ptr::write_unaligned(p, val);
    }
}

#[inline]
unsafe fn load_ov_internal_high(ov: *mut OVERLAPPED) -> usize {
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
    store_ov_internal_high(ov, 0);
    store_ov_internal(ov, STATUS_PENDING);
    let ev = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).h_event));
    if !ev.is_null() {
        let ev_h = (ev as usize & !1) as HANDLE;
        if let Some(e) = handles::get_as::<Event>(ev_h) {
            e.reset();
        }
    }
}

unsafe fn complete_overlapped(ov: *mut OVERLAPPED, status: usize, n: usize) {
    store_ov_internal_high(ov, n);
    store_ov_internal(ov, status);
    let ev = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).h_event));
    if !ev.is_null() {
        // Low bit set = "don't signal a completion port"; still a valid event.
        let ev = (ev as usize & !1) as HANDLE;
        if let Some(e) = handles::get_as::<Event>(ev) {
            e.set();
        }
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

/// Try a non-blocking read on a pipe/socket fd without toggling `O_NONBLOCK` on
/// a potentially shared descriptor.
/// Returns `Ok(Some(n))` (`n == 0` means EOF/broken pipe), `Ok(None)` if would block,
/// or `Err(errno)`.
unsafe fn try_nonblocking_pipe_read(fd: i32, buf: *mut u8, n: usize) -> Result<Option<usize>, i32> {
    if n == 0 {
        return Ok(Some(0));
    }
    // Try MSG_DONTWAIT first for socketpair-backed named pipes.
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
    // Anonymous POSIX pipe: poll(0) + FIONREAD tells us how much is readable without blocking.
    let mut pfd = libc::pollfd { fd, events: libc::POLLIN, revents: 0 };
    let pr = loop {
        let r = libc::poll(&mut pfd, 1, 0);
        if r < 0 && last_errno() == libc::EINTR {
            continue;
        }
        break r;
    };
    if pr < 0 {
        return Err(last_errno());
    }
    if pr == 0 || (pfd.revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR | libc::POLLNVAL)) == 0 {
        return Ok(None);
    }
    let mut avail: libc::c_int = 0;
    let has_fionread = libc::ioctl(fd, libc::FIONREAD, &mut avail) == 0;
    if has_fionread && avail == 0 {
        if (pfd.revents & (libc::POLLHUP | libc::POLLERR)) != 0 {
            return Ok(Some(0)); // EOF / broken pipe
        }
        return Ok(None);
    }
    let to_read = if has_fionread && avail > 0 { n.min(avail as usize) } else { n };
    loop {
        let r = libc::read(fd, buf as *mut c_void, to_read);
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

/// Try a non-blocking write on a pipe/socket fd.
/// Returns `Ok(Some(n))`, `Ok(None)` if would block, or `Err(errno)`.
unsafe fn try_nonblocking_pipe_write(fd: i32, buf: *const u8, n: usize) -> Result<Option<usize>, i32> {
    if n == 0 {
        return Ok(Some(0));
    }
    // Try MSG_DONTWAIT first for socketpair-backed named pipes.
    loop {
        let r = libc::send(fd, buf as *const c_void, n, libc::MSG_DONTWAIT);
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
    // Anonymous POSIX pipe: check POLLOUT first; temporarily set O_NONBLOCK only for large writes.
    let mut pfd = libc::pollfd { fd, events: libc::POLLOUT, revents: 0 };
    let pr = loop {
        let r = libc::poll(&mut pfd, 1, 0);
        if r < 0 && last_errno() == libc::EINTR {
            continue;
        }
        break r;
    };
    if pr < 0 {
        return Err(last_errno());
    }
    if pr == 0 {
        return Ok(None);
    }
    if (pfd.revents & (libc::POLLERR | libc::POLLHUP | libc::POLLNVAL)) != 0 && (pfd.revents & libc::POLLOUT) == 0 {
        return Err(libc::EPIPE);
    }
    let old_fl = libc::fcntl(fd, libc::F_GETFL);
    let toggled = old_fl >= 0 && (old_fl & libc::O_NONBLOCK) == 0;
    if toggled {
        let _ = libc::fcntl(fd, libc::F_SETFL, old_fl | libc::O_NONBLOCK);
    }
    let res = loop {
        let r = libc::write(fd, buf as *const c_void, n);
        if r >= 0 {
            break Ok(Some(r as usize));
        }
        let e = last_errno();
        if e == libc::EINTR {
            continue;
        }
        if e == libc::EAGAIN || e == libc::EWOULDBLOCK {
            break Ok(None);
        }
        break Err(e);
    };
    if toggled {
        let _ = libc::fcntl(fd, libc::F_SETFL, old_fl);
    }
    res
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
        let fd = match f.io_fd() {
            Ok(fd) => fd,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
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
                complete_overlapped_io(h, ov, STATUS_SUCCESS, from_peek, true);
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
                    complete_overlapped_io(h, ov, STATUS_SUCCESS, got, true);
                    return TRUE;
                }
                Ok(None) => {
                    f.io_event.reset();
                    prepare_overlapped_pending(ov);
                    pipe_reactor().submit(PendingPipeOp::new_read(
                        h,
                        Arc::clone(&f),
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
        // Synchronous blocking pipe read.
        let r = loop {
            let k = libc::read(fd, buf, n as usize);
            if k < 0 && last_errno() == libc::EINTR {
                continue;
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
                complete_overlapped_io(h, ov, STATUS_SUCCESS, got, true);
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
    if n > 0 && buf.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if f.kind == FileKind::Pipe {
        let fd = match f.io_fd() {
            Ok(fd) => fd,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
        if n == 0 {
            if !ov.is_null() {
                complete_overlapped_io(h, ov, STATUS_SUCCESS, 0, true);
            }
            return TRUE;
        }
        if !ov.is_null() {
            match try_nonblocking_pipe_write(fd, buf as *const u8, n as usize) {
                Ok(Some(k)) => {
                    if !written.is_null() {
                        std::ptr::write_unaligned(written, k as u32);
                    }
                    complete_overlapped_io(h, ov, STATUS_SUCCESS, k, true);
                    return TRUE;
                }
                Ok(None) => {
                    f.io_event.reset();
                    prepare_overlapped_pending(ov);
                    pipe_reactor().submit(PendingPipeOp::new_write(
                        h,
                        Arc::clone(&f),
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
        // Synchronous blocking pipe write (write full buffer).
        let mut done = 0usize;
        while done < n as usize {
            let r = libc::write(fd, (buf as *const u8).add(done) as *const c_void, n as usize - done);
            if r < 0 {
                let e = last_errno();
                if e == libc::EINTR {
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
                complete_overlapped_io(h, ov, STATUS_SUCCESS, k, true);
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
        let fd = match f.io_fd() {
            Ok(fd) => fd,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
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
    let caller = crate::thread::ensure_current_thread_object();
    let caller_tid = teb::thread_id();

    if f.kind == FileKind::Pipe {
        let fd = match f.io_fd() {
            Ok(fd) => fd,
            Err(e) => {
                set_err(e);
                return FALSE;
            }
        };
        if n == 0 {
            store_ov_internal_high(ov, 0);
            store_ov_internal(ov, STATUS_SUCCESS);
            if let Some(cb) = completion_routine {
                crate::thread::queue_io_completion_apc(&caller, cb, 0, 0, ov as usize);
            }
            set_err(0);
            return TRUE;
        }
        match try_nonblocking_pipe_write(fd, buf as *const u8, n as usize) {
            Ok(Some(k)) => {
                store_ov_internal_high(ov, k);
                store_ov_internal(ov, STATUS_SUCCESS);
                if let Some(cb) = completion_routine {
                    crate::thread::queue_io_completion_apc(&caller, cb, 0, k as u32, ov as usize);
                }
                set_err(0);
                return TRUE;
            }
            Ok(None) => {
                store_ov_internal_high(ov, 0);
                store_ov_internal(ov, STATUS_PENDING);
                pipe_reactor().submit(PendingPipeOp::new_write(
                    h,
                    Arc::clone(&f),
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
        STATUS_SUCCESS => TRUE,
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

pub unsafe extern "win64" fn GetFileInformationByHandleEx(h: HANDLE, class: u32, out: *mut u8, size: u32) -> BOOL {
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
            // FILE_BASIC_INFO
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
            // FILE_STANDARD_INFO
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
        2 => {
            // FILE_NAME_INFO: path without the drive letter.
            let p = f.win_path.get(2..).unwrap_or("").to_string();
            let w = to_wide(&p);
            if !need(4) {
                return FALSE;
            }
            (out as *mut u32).write_unaligned((w.len() * 2) as u32);
            let room = (size as usize - 4) / 2;
            std::ptr::copy_nonoverlapping(w.as_ptr(), out.add(4) as *mut u16, w.len().min(room));
            if w.len() > room {
                set_err(err::ERROR_MORE_DATA);
                return FALSE;
            }
        }
        9 => {
            // FILE_ATTRIBUTE_TAG_INFO
            if !need(8) {
                return FALSE;
            }
            (out as *mut u32).write_unaligned(meta_attrs(&m, &name));
            (out.add(4) as *mut u32).write_unaligned(0);
        }
        17 => {
            // FILE_ALIGNMENT_INFO
            if !need(4) {
                return FALSE;
            }
            (out as *mut u32).write_unaligned(0);
        }
        16 => {
            // FILE_STORAGE_INFO: report 4K physical sectors.
            if !need(28) {
                return FALSE;
            }
            let vals: [u32; 7] = [512, 4096, 4096, 4096, 0x1 | 0x2 | 0x8, 0, 0];
            std::ptr::copy_nonoverlapping(vals.as_ptr() as *const u8, out, 28);
        }
        18 => {
            // FILE_ID_INFO
            if !need(24) {
                return FALSE;
            }
            std::ptr::write_bytes(out, 0, 24);
            put_ft(out, m.dev() as u64);
            put_ft(out.add(8), m.ino());
        }
        _ => {
            stub!("GetFileInformationByHandleEx class {class} not supported");
            set_err(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn SetFileInformationByHandle(h: HANDLE, class: u32, info: *const u8, size: u32) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    match class {
        0 => {
            // FILE_BASIC_INFO: times (0 = don't change).
            let a = (info.add(8) as *const i64).read_unaligned();
            let w = (info.add(16) as *const i64).read_unaligned();
            set_times(&f, a, w)
        }
        3 => {
            // FILE_RENAME_INFO { BOOLEAN ReplaceIfExists; HANDLE RootDirectory; DWORD FileNameLength; WCHAR FileName[] }
            let replace = *info != 0;
            let len = (info.add(16) as *const u32).read_unaligned() as usize / 2;
            let name = String::from_utf16_lossy(std::slice::from_raw_parts(info.add(20) as *const u16, len));
            let Some(src) = &f.host_path else {
                set_err(err::ERROR_INVALID_HANDLE);
                return FALSE;
            };
            let dst = match host_path(&name) {
                Ok(p) => p,
                Err(e) => {
                    set_err(e);
                    return FALSE;
                }
            };
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
            // FILE_DISPOSITION_INFO(_EX)
            let del = if class == 4 { *info != 0 } else { (info as *const u32).read_unaligned() & 1 != 0 };
            *f.delete_on_close.lock().unwrap() = del;
            TRUE
        }
        5 | 6 => {
            // FILE_ALLOCATION_INFO / FILE_END_OF_FILE_INFO
            let len = (info as *const i64).read_unaligned();
            if class == 5 {
                return TRUE; // allocation hint only
            }
            if libc::ftruncate(f.fd(), len) != 0 {
                set_err(errno_to_win32(last_errno()));
                return FALSE;
            }
            TRUE
        }
        _ => {
            let _ = size;
            stub!("SetFileInformationByHandle class {class} not supported");
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
        if p.is_dir() {
            Err(std::io::Error::from_raw_os_error(libc::EEXIST))
        } else {
            std::fs::create_dir(p)
        }
    })
}
pub unsafe extern "win64" fn CreateDirectoryA(name: LPCSTR, _sec: *mut c_void) -> BOOL {
    path_op(&from_ansi(name), |p| {
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
    path_op(&from_wide(name), |p| match std::fs::remove_dir(p) {
        Ok(()) => Ok(()),
        Err(e) if e.raw_os_error() == Some(libc::ENOENT) => Ok(()),
        Err(e) => Err(e),
    })
}
pub unsafe extern "win64" fn RemoveDirectoryA(name: LPCSTR) -> BOOL {
    path_op(&from_ansi(name), |p| match std::fs::remove_dir(p) {
        Ok(()) => Ok(()),
        Err(e) if e.raw_os_error() == Some(libc::ENOENT) => Ok(()),
        Err(e) => Err(e),
    })
}
pub unsafe extern "win64" fn DeleteFileW(name: LPCWSTR) -> BOOL {
    path_op(&from_wide(name), |p| {
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

pub fn move_file(src: &str, dst: &str, flags: u32) -> BOOL {
    let (s, d) = match (host_path(src), host_path(dst)) {
        (Ok(s), Ok(d)) => (s, d),
        (Err(e), _) | (_, Err(e)) => {
            set_err(e);
            return FALSE;
        }
    };
    if !s.exists() {
        set_err(not_found_err(&s));
        return FALSE;
    }
    if d.exists() && flags & MOVEFILE_REPLACE_EXISTING == 0 {
        set_err(err::ERROR_ALREADY_EXISTS);
        return FALSE;
    }
    match std::fs::rename(&s, &d) {
        Ok(()) => TRUE,
        Err(e) if e.raw_os_error() == Some(libc::EXDEV) && flags & MOVEFILE_COPY_ALLOWED != 0 => {
            match std::fs::copy(&s, &d).and_then(|_| std::fs::remove_file(&s)) {
                Ok(()) => TRUE,
                Err(e) => {
                    log::warn!("move_file({src:?} -> {dst:?}) cross-device copy failed: {e}");
                    set_err(io_err(&e));
                    FALSE
                }
            }
        }
        Err(e) => {
            log::warn!("move_file({src:?} -> {dst:?}) failed: {e}");
            set_err(io_err(&e));
            FALSE
        }
    }
}

pub unsafe extern "win64" fn MoveFileExW(src: LPCWSTR, dst: LPCWSTR, flags: u32) -> BOOL {
    if dst.is_null() {
        stub!("MoveFileExW delete-on-reboot ignored");
        return TRUE;
    }
    move_file(&from_wide(src), &from_wide(dst), flags)
}
pub unsafe extern "win64" fn MoveFileExA(src: LPCSTR, dst: LPCSTR, flags: u32) -> BOOL {
    if dst.is_null() {
        stub!("MoveFileExA delete-on-reboot ignored");
        return TRUE;
    }
    move_file(&from_ansi(src), &from_ansi(dst), flags)
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

enum IocpDequeue {
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
        if alertable && crate::thread::has_pending_apc() {
            crate::thread::run_pending_apcs();
            return IocpDequeue::IoCompletion;
        }
        let deadline = (timeout_ms != INFINITE).then(|| Instant::now() + Duration::from_millis(timeout_ms as u64));
        let ready_obj: Arc<dyn KObject> = Arc::clone(&self.ready) as Arc<dyn KObject>;
        loop {
            let out = self.try_pop_up_to(max);
            if !out.is_empty() {
                return IocpDequeue::Packets(out);
            }
            if timeout_ms == 0 {
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

/// Shared registry for non-`FileObj` I/O handles (such as Winsock2 `SOCKET` handles)
/// so `CreateIoCompletionPort`, `SetFileCompletionNotificationModes`, `CancelIo`/`CancelIoEx`,
/// `GetOverlappedResultEx`, and `CreateThreadpoolIo` work uniformly across files and sockets.
struct ExternalIoEntry {
    iocp: Option<(Arc<IoCompletionPort>, usize)>,
    tp_io: Option<Arc<TpIo>>,
    completion_modes: u8,
    event: Arc<Event>,
}

fn external_io_table() -> &'static Mutex<std::collections::HashMap<usize, ExternalIoEntry>> {
    static T: OnceLock<Mutex<std::collections::HashMap<usize, ExternalIoEntry>>> = OnceLock::new();
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
    t.entry(h as usize).or_insert_with(|| ExternalIoEntry {
        iocp: None,
        tp_io: None,
        completion_modes: 0,
        event: Arc::new(Event::new(true, true, None)),
    });
}

pub(crate) fn unregister_external_io_handle(h: HANDLE) {
    external_io_table().lock().unwrap().remove(&(h as usize));
}

pub(crate) fn external_io_event(h: HANDLE) -> Option<Arc<Event>> {
    external_io_table().lock().unwrap().get(&(h as usize)).map(|e| Arc::clone(&e.event))
}

#[allow(dead_code)]
pub(crate) fn handle_has_completion_sink(h: HANDLE) -> bool {
    if let Some(f) = get_file(h) {
        return f.iocp.lock().unwrap().is_some() || f.tp_io.lock().unwrap().is_some();
    }
    external_io_table()
        .lock()
        .unwrap()
        .get(&(h as usize))
        .is_some_and(|e| e.iocp.is_some() || e.tp_io.is_some())
}

/// Bind a `TpIo` completion sink to a `FileObj` or registered external I/O handle (e.g. `SOCKET`).
/// Fails with `ERROR_INVALID_HANDLE` if `h` is not a valid I/O handle, or `ERROR_INVALID_PARAMETER`
/// if `h` is already bound to an IOCP or another `TpIo`.
pub(crate) fn bind_threadpool_io(h: HANDLE, tp_io: Arc<TpIo>) -> Result<(), u32> {
    if let Some(f) = get_file(h) {
        let iocp_g = f.iocp.lock().unwrap();
        let mut tp_g = f.tp_io.lock().unwrap();
        if iocp_g.is_some() || tp_g.is_some() {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        *tp_g = Some(tp_io);
        return Ok(());
    }
    let mut ext = external_io_table().lock().unwrap();
    let Some(entry) = ext.get_mut(&(h as usize)) else {
        return Err(err::ERROR_INVALID_HANDLE);
    };
    if entry.iocp.is_some() || entry.tp_io.is_some() {
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    entry.tp_io = Some(tp_io);
    Ok(())
}

/// Detach a `TpIo` sink from `h` when `CloseThreadpoolIo` is called.
pub(crate) fn unbind_threadpool_io(h: HANDLE, tp_io: &Arc<TpIo>) {
    if let Some(f) = get_file(h) {
        let mut tp_g = f.tp_io.lock().unwrap();
        if tp_g.as_ref().is_some_and(|cur| Arc::ptr_eq(cur, tp_io)) {
            *tp_g = None;
        }
        return;
    }
    let mut ext = external_io_table().lock().unwrap();
    if let Some(entry) = ext.get_mut(&(h as usize)) {
        if entry.tp_io.as_ref().is_some_and(|cur| Arc::ptr_eq(cur, tp_io)) {
            entry.tp_io = None;
        }
    }
}

/// Complete an overlapped I/O operation on either a `FileObj` or a registered external I/O handle (e.g. SOCKET).
/// Atomically publishes `InternalHigh` and `Internal` (Release), signals `ov.h_event` (or the handle's event),
/// and posts to any associated IOCP or dispatches to any bound `TpIo`.
pub(crate) unsafe fn complete_overlapped_io(
    h: HANDLE,
    ov: *mut OVERLAPPED,
    nt_status: usize,
    bytes: usize,
    synchronous: bool,
) -> bool {
    const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u8 = 0x1;
    const FILE_SKIP_SET_EVENT_ON_HANDLE: u8 = 0x2;
    if ov.is_null() {
        return false;
    }
    store_ov_internal_high(ov, bytes);
    store_ov_internal(ov, nt_status);

    let raw_ev = std::ptr::read_unaligned(std::ptr::addr_of!((*ov).h_event)) as usize;
    let suppress_iocp = (raw_ev & 1) != 0;
    let ev_h = (raw_ev & !1) as HANDLE;
    if !ev_h.is_null() {
        if let Some(e) = handles::get_as::<Event>(ev_h) {
            e.set();
        }
    }

    let (modes, iocp_binding, tp_io_binding, handle_event) = if let Some(f) = get_file(h) {
        (
            f.completion_modes.load(Ordering::Relaxed),
            f.iocp.lock().unwrap().clone(),
            f.tp_io.lock().unwrap().clone(),
            Some(Arc::clone(&f.io_event)),
        )
    } else {
        let t = external_io_table().lock().unwrap();
        if let Some(e) = t.get(&(h as usize)) {
            (e.completion_modes, e.iocp.clone(), e.tp_io.clone(), Some(Arc::clone(&e.event)))
        } else {
            (0, None, None, None)
        }
    };

    if let Some(ev) = handle_event {
        if !(synchronous && (modes & FILE_SKIP_SET_EVENT_ON_HANDLE) != 0) {
            ev.set();
        }
    }

    if suppress_iocp {
        return false;
    }
    if synchronous && nt_status == STATUS_SUCCESS && (modes & FILE_SKIP_COMPLETION_PORT_ON_SUCCESS) != 0 {
        return false;
    }
    if let Some((iocp, key)) = iocp_binding {
        iocp.post(IocpPacket {
            bytes: bytes as u32,
            key,
            overlapped: ov as usize,
            internal_status: nt_status,
        });
        return true;
    }
    if let Some(tp_io) = tp_io_binding {
        return tp_io.dispatch_completion(ov as usize, ntstatus_to_win32_local(nt_status), bytes);
    }
    false
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
        if let Some(f) = get_file(file_handle) {
            let mut iocp_g = f.iocp.lock().unwrap();
            let tp_g = f.tp_io.lock().unwrap();
            if tp_g.is_some() {
                drop(tp_g);
                drop(iocp_g);
                if existing_port.is_null() {
                    handles::close(port_handle);
                }
                set_err(err::ERROR_INVALID_PARAMETER);
                return std::ptr::null_mut();
            }
            *iocp_g = Some((iocp, completion_key));
        } else {
            let mut ext = external_io_table().lock().unwrap();
            if let Some(entry) = ext.get_mut(&(file_handle as usize)) {
                if entry.tp_io.is_some() {
                    drop(ext);
                    if existing_port.is_null() {
                        handles::close(port_handle);
                    }
                    set_err(err::ERROR_INVALID_PARAMETER);
                    return std::ptr::null_mut();
                }
                entry.iocp = Some((iocp, completion_key));
            } else {
                drop(ext);
                if existing_port.is_null() {
                    handles::close(port_handle);
                }
                set_err(err::ERROR_INVALID_HANDLE);
                return std::ptr::null_mut();
            }
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
        if pkt.internal_status != STATUS_SUCCESS {
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
    if let Some(f) = get_file(file_handle) {
        f.completion_modes.store(flags, std::sync::atomic::Ordering::Relaxed);
        return TRUE;
    }
    if let Some(ext) = external_io_table().lock().unwrap().get_mut(&(file_handle as usize)) {
        ext.completion_modes = flags;
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

fn snapshot_dir_rec(root: &Path, rel_prefix: &str, subtree: bool, out: &mut BTreeMap<String, DirSnapEntry>) {
    let dir_path = if rel_prefix.is_empty() {
        root.to_path_buf()
    } else {
        root.join(rel_prefix.replace('\\', "/"))
    };
    let Ok(rd) = std::fs::read_dir(&dir_path) else {
        return;
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
            snapshot_dir_rec(root, &rel, true, out);
        }
    }
}

fn snapshot_dir(root: &Path, subtree: bool) -> BTreeMap<String, DirSnapEntry> {
    let mut map = BTreeMap::new();
    snapshot_dir_rec(root, "", subtree, &mut map);
    map
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
            DirWatchDelivery::Overlapped { ov, .. } => unsafe {
                let _ = complete_overlapped_io(
                    self.handle as HANDLE,
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
            let initial_snap = snapshot_dir(&root, subtree);
            let mut watched_by_path: HashMap<String, (i32, u64, u64)> = HashMap::new();
            let mut path_by_fd: HashMap<i32, String> = HashMap::new();
            sync_kqueue_vnodes(kq, &root, &initial_snap, &mut watched_by_path, &mut path_by_fd);

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
                    dir_watch_kqueue_loop(w_thread, kq, wake_r, root, initial_snap, watched_by_path, path_by_fd);
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
fn open_vnode_evtonly(path: &Path) -> Option<i32> {
    let c = std::ffi::CString::new(path.as_os_str().as_encoded_bytes()).ok()?;
    let fd = unsafe { libc::open(c.as_ptr(), DARWIN_O_EVTONLY | libc::O_CLOEXEC) };
    if fd < 0 {
        None
    } else {
        Some(fd)
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

#[cfg(target_vendor = "apple")]
fn sync_kqueue_vnodes(
    kq: i32,
    root: &Path,
    snap: &BTreeMap<String, DirSnapEntry>,
    watched_by_path: &mut HashMap<String, (i32, u64, u64)>,
    path_by_fd: &mut HashMap<i32, String>,
) {
    let mut desired: Vec<(String, u64, u64)> = Vec::new();
    if let Ok(rm) = std::fs::symlink_metadata(root) {
        desired.push((String::new(), rm.dev(), rm.ino()));
    }
    // Prioritize directories first so subtree watching always covers every subdirectory,
    // then add files up to MAX_WATCHED_VNODE_FDS so in-place file writes trigger immediately.
    for (rel, ent) in snap {
        if ent.is_dir {
            desired.push((rel.clone(), ent.dev, ent.ino));
        }
    }
    for (rel, ent) in snap {
        if !ent.is_dir {
            if desired.len() >= MAX_WATCHED_VNODE_FDS {
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
    for (rel, dev, ino) in desired {
        if watched_by_path.contains_key(&rel) {
            continue;
        }
        let full = if rel.is_empty() {
            root.to_path_buf()
        } else {
            root.join(rel.replace('\\', "/"))
        };
        if let Some(fd) = open_vnode_evtonly(&full) {
            if register_vnode_kevent(kq, fd) {
                watched_by_path.insert(rel.clone(), (fd, dev, ino));
                path_by_fd.insert(fd, rel);
            } else {
                unsafe { libc::close(fd) };
            }
        }
    }
}

#[cfg(target_vendor = "apple")]
fn dir_watch_kqueue_loop(
    watcher: Arc<DirWatcher>,
    kq: i32,
    wake_r: i32,
    root: PathBuf,
    mut prev_snap: BTreeMap<String, DirSnapEntry>,
    mut watched_by_path: HashMap<String, (i32, u64, u64)>,
    mut path_by_fd: HashMap<i32, String>,
) {
    let mut cur_subtree = watcher.state.lock().unwrap().subtree;
    let mut events = [unsafe { std::mem::zeroed::<libc::kevent>() }; 64];

    loop {
        if watcher.closed.load(Ordering::Acquire) {
            break;
        }
        let n = unsafe { libc::kevent(kq, std::ptr::null(), 0, events.as_mut_ptr(), events.len() as i32, std::ptr::null()) };
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
            prev_snap = snapshot_dir(&root, true);
            sync_kqueue_vnodes(kq, &root, &prev_snap, &mut watched_by_path, &mut path_by_fd);
        }

        if saw_vnode {
            let new_snap = snapshot_dir(&root, cur_subtree);
            let changes = diff_dir_snapshots(&prev_snap, &new_snap, &vnode_written, &vnode_attrib);
            prev_snap = new_snap;
            // Register new vnodes BEFORE completing any request so subsequent watches never miss mutations.
            sync_kqueue_vnodes(kq, &root, &prev_snap, &mut watched_by_path, &mut path_by_fd);

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

    // Check if buffered changes already exist from between consecutive ReadDirectoryChangesW calls.
    let immediate: Option<(usize, usize)> = {
        let mut st = watcher.state.lock().unwrap();
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

    if let Some((status, bytes)) = immediate {
        if ov.is_null() {
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
        if !cb.is_null() {
            let caller = crate::thread::ensure_current_thread_object();
            let cb_fn: crate::thread::IoCompletionRoutine = std::mem::transmute(cb);
            store_ov_internal_high(ov, bytes);
            store_ov_internal(ov, status);
            crate::thread::queue_io_completion_apc(&caller, cb_fn, ntstatus_to_win32_local(status), bytes as u32, ov as usize);
            set_err(0);
            return TRUE;
        }
        let _ = complete_overlapped_io(h, ov, status, bytes, false);
        set_err(0);
        return TRUE;
    }

    let tid = teb::thread_id();
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
        watcher.state.lock().unwrap().requests.push_back(req);
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
        store_ov_internal_high(ov, 0);
        store_ov_internal(ov, STATUS_PENDING);
        let req = Arc::new(DirWatchRequest {
            handle: h as usize,
            buf: buf as usize,
            len: len as usize,
            filter,
            completed: AtomicBool::new(false),
            delivery: DirWatchDelivery::Apc { ov: ov as usize, cb: cb_fn, thread: caller, tid },
        });
        watcher.state.lock().unwrap().requests.push_back(req);
        set_err(0);
        TRUE
    } else {
        f.io_event.reset();
        prepare_overlapped_pending(ov);
        let req = Arc::new(DirWatchRequest {
            handle: h as usize,
            buf: buf as usize,
            len: len as usize,
            filter,
            completed: AtomicBool::new(false),
            delivery: DirWatchDelivery::Overlapped { ov: ov as usize, tid },
        });
        watcher.state.lock().unwrap().requests.push_back(req);
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
}

struct PendingPipeOp {
    handle: usize,
    file: Arc<FileObj>,
    fd: i32,
    kind: PipeOpKind,
    buf: usize,
    len: usize,
    completed: AtomicBool,
    delivery: PipeDelivery,
}

impl PendingPipeOp {
    fn new_read(handle: HANDLE, file: Arc<FileObj>, fd: i32, buf: usize, len: usize, delivery: PipeDelivery) -> Arc<Self> {
        Arc::new(PendingPipeOp {
            handle: handle as usize,
            file,
            fd,
            kind: PipeOpKind::Read,
            buf,
            len,
            completed: AtomicBool::new(false),
            delivery,
        })
    }

    fn new_write(handle: HANDLE, file: Arc<FileObj>, fd: i32, buf: usize, len: usize, delivery: PipeDelivery) -> Arc<Self> {
        Arc::new(PendingPipeOp {
            handle: handle as usize,
            file,
            fd,
            kind: PipeOpKind::Write,
            buf,
            len,
            completed: AtomicBool::new(false),
            delivery,
        })
    }

    fn tid(&self) -> u32 {
        match &self.delivery {
            PipeDelivery::Overlapped { tid, .. } | PipeDelivery::Apc { tid, .. } => *tid,
        }
    }

    fn ov_addr(&self) -> usize {
        match &self.delivery {
            PipeDelivery::Overlapped { ov, .. } | PipeDelivery::Apc { ov, .. } => *ov,
        }
    }

    fn finish_once(&self, nt_status: usize, bytes: usize) -> bool {
        if self.completed.swap(true, Ordering::AcqRel) {
            return false;
        }
        match &self.delivery {
            PipeDelivery::Overlapped { ov, .. } => unsafe {
                let _ = complete_overlapped_io(
                    self.handle as HANDLE,
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
        }
        true
    }
}

struct PipeIoReactor {
    ops: Mutex<Vec<Arc<PendingPipeOp>>>,
    wake_w: i32,
}

impl PipeIoReactor {
    fn poke(&self) {
        let b = 1u8;
        unsafe {
            let _ = libc::write(self.wake_w, &b as *const u8 as *const c_void, 1);
        }
    }

    fn submit(&self, op: Arc<PendingPipeOp>) {
        self.ops.lock().unwrap().push(op);
        self.poke();
    }

    fn cancel_ops(&self, handle: usize, ov: Option<usize>, filter_tid: Option<u32>) -> usize {
        let to_cancel: Vec<Arc<PendingPipeOp>> = {
            let mut ops = self.ops.lock().unwrap();
            let mut matched = Vec::new();
            let mut i = 0;
            while i < ops.len() {
                let o = &ops[i];
                let h_ok = o.handle == handle;
                let ov_ok = ov.map_or(true, |target| o.ov_addr() == target);
                let tid_ok = filter_tid.map_or(true, |t| o.tid() == t);
                if h_ok && ov_ok && tid_ok {
                    matched.push(ops.swap_remove(i));
                } else {
                    i += 1;
                }
            }
            matched
        };
        let mut count = 0;
        for op in to_cancel {
            if op.finish_once(STATUS_CANCELLED, 0) {
                count += 1;
            }
        }
        if count > 0 {
            self.poke();
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
            ops: Mutex::new(Vec::new()),
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
                        let mut ops = reactor.ops.lock().unwrap();
                        ops.retain(|o| !o.completed.load(Ordering::Acquire));
                        for o in ops.iter() {
                            let events = match o.kind {
                                PipeOpKind::Read => libc::POLLIN,
                                PipeOpKind::Write => libc::POLLOUT,
                            };
                            pfds.push(libc::pollfd { fd: o.fd, events, revents: 0 });
                            snap.push(Arc::clone(o));
                        }
                    }
                    let pr = unsafe { libc::poll(pfds.as_mut_ptr(), pfds.len() as libc::nfds_t, -1) };
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
                        let revents = pfds[idx + 1].revents;
                        if revents == 0 || op.completed.load(Ordering::Acquire) {
                            continue;
                        }
                        let ready_result: Option<(usize, usize)> = match op.kind {
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
                                match unsafe { try_nonblocking_pipe_write(op.fd, op.buf as *const u8, op.len) } {
                                    Ok(Some(k)) => Some((STATUS_SUCCESS, k)),
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
                        };
                        if let Some((status, bytes)) = ready_result {
                            {
                                let mut ops = reactor.ops.lock().unwrap();
                                ops.retain(|o| !Arc::ptr_eq(o, op));
                            }
                            op.finish_once(status, bytes);
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
    Connected(File),
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
            ServerPipeConn::Connected(f) => Some(f.as_raw_fd()),
            _ => None,
        }
    }

    fn io_fd(&self) -> Result<i32, u32> {
        let st = self.state.lock().unwrap();
        match &st.conn {
            ServerPipeConn::Connected(f) => Ok(f.as_raw_fd()),
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
            PendingConnect::Overlapped { handle: h, ov: o, completed, .. } => {
                if !completed.swap(true, Ordering::AcqRel) {
                    unsafe {
                        let _ = complete_overlapped_io(h as HANDLE, o as *mut OVERLAPPED, STATUS_CANCELLED, 0, false);
                    }
                    1
                } else {
                    0
                }
            }
        }
    }

    fn close_server(self: &Arc<Self>) {
        let pending = {
            let mut st = self.state.lock().unwrap();
            st.conn = ServerPipeConn::Disconnected;
            st.pending_connect.take()
        };
        if let Some(pc) = pending {
            match pc {
                PendingConnect::Sync { done, .. } => {
                    let (m, cv) = &*done;
                    *m.lock().unwrap() = Some(STATUS_CANCELLED);
                    cv.notify_all();
                }
                PendingConnect::Overlapped { handle: h, ov: o, completed, .. } => {
                    if !completed.swap(true, Ordering::AcqRel) {
                        unsafe {
                            let _ = complete_overlapped_io(h as HANDLE, o as *mut OVERLAPPED, STATUS_CANCELLED, 0, false);
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
            let empty = srvs.is_empty();
            drop(srvs);
            entry.cv.notify_all();
            if empty {
                map.remove(&self.norm_name);
            }
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
            st.conn = ServerPipeConn::Connected(unsafe { File::from_raw_fd(fds[0]) });
            let pending = st.pending_connect.take();
            drop(st);

            if let Some(pc) = pending {
                match pc {
                    PendingConnect::Sync { done, .. } => {
                        let (m, cv) = &*done;
                        *m.lock().unwrap() = Some(STATUS_SUCCESS);
                        cv.notify_all();
                    }
                    PendingConnect::Overlapped { handle: h, ov: o, completed, .. } => {
                        if !completed.swap(true, Ordering::AcqRel) {
                            unsafe {
                                let _ = complete_overlapped_io(
                                    h as HANDLE,
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

pub unsafe extern "win64" fn CreatePipe(rd: *mut HANDLE, wr: *mut HANDLE, sec: *mut c_void, _size: u32) -> BOOL {
    if rd.is_null() || wr.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut fds = [0i32; 2];
    if libc::pipe(fds.as_mut_ptr()) != 0 {
        set_err(errno_to_win32(last_errno()));
        return FALSE;
    }
    set_cloexec(fds[0]);
    set_cloexec(fds[1]);
    set_nosigpipe_pipe(fds[0]);
    set_nosigpipe_pipe(fds[1]);
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
    let fd = match f.io_fd() {
        Ok(fd) => fd,
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

    let mut pb = f.peek_buf.lock().unwrap();
    let want = if !buf.is_null() && size > 0 { size as usize } else { 0 };
    if pb.len() < want {
        let need = want - pb.len();
        let mut tmp = vec![0u8; need];
        match try_nonblocking_pipe_read(fd, tmp.as_mut_ptr(), need) {
            Ok(Some(0)) => {
                if pb.is_empty() {
                    set_err(err::ERROR_BROKEN_PIPE);
                    return FALSE;
                }
            }
            Ok(Some(got)) => {
                pb.extend(&tmp[..got]);
            }
            Ok(None) => {}
            Err(e) => {
                if pb.is_empty() {
                    set_err(if e == libc::EPIPE || e == libc::ECONNRESET { err::ERROR_BROKEN_PIPE } else { errno_to_win32(e) });
                    return FALSE;
                }
            }
        }
    }

    let mut kernel_avail: libc::c_int = 0;
    let fionread_ok = libc::ioctl(fd, libc::FIONREAD, &mut kernel_avail) == 0;
    if !fionread_ok {
        if pb.is_empty() {
            set_err(err::ERROR_BROKEN_PIPE);
            return FALSE;
        }
        kernel_avail = 0;
    }
    let total_avail = pb.len().saturating_add(kernel_avail.max(0) as usize);

    // If nothing buffered in userspace or kernel, detect peer close (POLLHUP) as ERROR_BROKEN_PIPE.
    if total_avail == 0 {
        let mut pfd = libc::pollfd { fd, events: libc::POLLIN, revents: 0 };
        let pr = libc::poll(&mut pfd, 1, 0);
        if pr > 0 && (pfd.revents & (libc::POLLHUP | libc::POLLERR)) != 0 {
            set_err(err::ERROR_BROKEN_PIPE);
            return FALSE;
        }
    }

    let copied = if want > 0 {
        let k = pb.len().min(want);
        let dst = buf as *mut u8;
        for i in 0..k {
            std::ptr::write_unaligned(dst.add(i), pb[i]);
        }
        k
    } else {
        0
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
    let entry = {
        let mut reg = named_pipe_registry().lock().unwrap();
        Arc::clone(reg.entry(norm_name).or_insert_with(|| {
            Arc::new(NamedPipeRegistryEntry {
                servers: Mutex::new(Vec::new()),
                cv: Condvar::new(),
            })
        }))
    };
    entry.servers.lock().unwrap().push(Arc::clone(&server));
    entry.cv.notify_all();

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

    let mut st = srv.state.lock().unwrap();
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
            if let Some(entry) = named_pipe_registry().lock().unwrap().get(&srv.norm_name).cloned() {
                entry.cv.notify_all();
            }
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
        drop(st);
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
        f.io_event.reset();
        prepare_overlapped_pending(ov);
        st.pending_connect = Some(PendingConnect::Overlapped {
            handle: h as usize,
            ov: ov as usize,
            tid,
            completed: Arc::new(AtomicBool::new(false)),
        });
        drop(st);
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
    let _ = pipe_reactor().cancel_ops(h as usize, None, None);
    f.peek_buf.lock().unwrap().clear();
    {
        let mut st = srv.state.lock().unwrap();
        st.conn = ServerPipeConn::Listening;
    }
    if let Some(entry) = named_pipe_registry().lock().unwrap().get(&srv.norm_name).cloned() {
        entry.cv.notify_all();
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
    if !proc_.is_null() && proc_ != handles::CURRENT_PROCESS {
        stub!("MapViewOfFile3 on another process");
        set_err(err::ERROR_ACCESS_DENIED);
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
// DeviceIoControl
// ---------------------------------------------------------------------------

const IOCTL_STORAGE_QUERY_PROPERTY: u32 = 0x002D_1400;
const IOCTL_STORAGE_GET_DEVICE_NUMBER: u32 = 0x002D_1080;
const IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS: u32 = 0x0056_0000;
const IOCTL_DISK_GET_DRIVE_GEOMETRY: u32 = 0x0007_0000;

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
    let Some(_f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let out8 = out as *mut u8;
    let put = |data: &[u8]| -> BOOL {
        let n = data.len().min(out_len as usize);
        if n < data.len() && n < 8 {
            set_err(err::ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), out8, n);
        if !ret.is_null() {
            *ret = n as u32;
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
    match code {
        IOCTL_STORAGE_QUERY_PROPERTY if in_len >= 8 => {
            let prop = *(inp as *const u32);
            let qtype = *(inp as *const u32).add(1);
            log::debug!("DeviceIoControl(IOCTL_STORAGE_QUERY_PROPERTY, prop={prop}, type={qtype})");
            if qtype == 1 {
                // PropertyExistsQuery
                return if matches!(prop, 0 | 1 | 6 | 7 | 8) { put(&[]) } else { fail_not_supported() };
            }
            match prop {
                0 => {
                    // STORAGE_DEVICE_DESCRIPTOR. The host disk is an Apple NVMe SSD.
                    let mut d = vec![0u8; 40];
                    let strings = ["Apple\0", "APPLE SSD\0", "1.0\0", "0000000000000001\0"];
                    let mut offs = [0u32; 4];
                    for (i, s) in strings.iter().enumerate() {
                        offs[i] = d.len() as u32;
                        d.extend_from_slice(s.as_bytes());
                    }
                    let total = d.len() as u32;
                    d[0..4].copy_from_slice(&40u32.to_le_bytes());
                    d[4..8].copy_from_slice(&total.to_le_bytes());
                    d[8] = 0; // DeviceType (disk)
                    d[10] = 0; // RemovableMedia
                    d[11] = 1; // CommandQueueing
                    for (i, o) in offs.iter().enumerate() {
                        d[12 + i * 4..16 + i * 4].copy_from_slice(&o.to_le_bytes());
                    }
                    d[28..32].copy_from_slice(&17u32.to_le_bytes()); // BusTypeNvme
                    put(&d)
                }
                1 => {
                    // STORAGE_ADAPTER_DESCRIPTOR (32 bytes)
                    let mut d = [0u8; 32];
                    d[0..4].copy_from_slice(&32u32.to_le_bytes());
                    d[4..8].copy_from_slice(&32u32.to_le_bytes());
                    d[8..12].copy_from_slice(&(1u32 << 20).to_le_bytes()); // MaximumTransferLength
                    d[12..16].copy_from_slice(&256u32.to_le_bytes()); // MaximumPhysicalPages
                    d[16..20].copy_from_slice(&3u32.to_le_bytes()); // AlignmentMask
                    d[22] = 1; // CommandQueueing
                    d[25] = 17; // BusType = NVMe
                    put(&d)
                }
                6 => {
                    // STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR
                    let vals: [u32; 7] = [28, 28, 64, 0, 4096, 0, 512];
                    put(std::slice::from_raw_parts(vals.as_ptr() as *const u8, 28))
                }
                7 => {
                    // DEVICE_SEEK_PENALTY_DESCRIPTOR: SSDs incur no seek penalty (true on Apple storage).
                    let mut d = [0u8; 12];
                    d[0..4].copy_from_slice(&12u32.to_le_bytes());
                    d[4..8].copy_from_slice(&12u32.to_le_bytes());
                    d[8] = 0;
                    put(&d)
                }
                8 => {
                    // DEVICE_TRIM_DESCRIPTOR: TRIM enabled.
                    let mut d = [0u8; 12];
                    d[0..4].copy_from_slice(&12u32.to_le_bytes());
                    d[4..8].copy_from_slice(&12u32.to_le_bytes());
                    d[8] = 1;
                    put(&d)
                }
                _ => {
                    stub!("IOCTL_STORAGE_QUERY_PROPERTY property {prop} unsupported");
                    fail_not_supported()
                }
            }
        }
        IOCTL_STORAGE_GET_DEVICE_NUMBER => {
            // STORAGE_DEVICE_NUMBER { DeviceType=FILE_DEVICE_DISK(7), DeviceNumber=0, PartitionNumber=1 }
            let vals: [u32; 3] = [7, 0, 1];
            put(std::slice::from_raw_parts(vals.as_ptr() as *const u8, 12))
        }
        IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS => {
            // VOLUME_DISK_EXTENTS { NumberOfDiskExtents=1, pad, DISK_EXTENT{ DiskNumber=0, Start=0, Length } }
            let mut d = [0u8; 32];
            d[0..4].copy_from_slice(&1u32.to_le_bytes());
            d[24..32].copy_from_slice(&(1u64 << 40).to_le_bytes());
            put(&d)
        }
        IOCTL_DISK_GET_DRIVE_GEOMETRY => {
            // DISK_GEOMETRY: Cylinders, MediaType=FixedMedia(12), TracksPerCylinder, SectorsPerTrack, BytesPerSector
            let mut d = [0u8; 24];
            d[0..8].copy_from_slice(&(1u64 << 20).to_le_bytes());
            d[8..12].copy_from_slice(&12u32.to_le_bytes());
            d[12..16].copy_from_slice(&255u32.to_le_bytes());
            d[16..20].copy_from_slice(&63u32.to_le_bytes());
            d[20..24].copy_from_slice(&512u32.to_le_bytes());
            put(&d)
        }
        _ => {
            stub!("DeviceIoControl code {code:#x} unsupported");
            fail_not_supported()
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
}
