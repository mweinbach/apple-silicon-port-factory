//! Files, directories, FindFirstFile, file mappings, console handles and
//! DeviceIoControl, implemented on host files via the [`crate::path`] mount
//! table.
//!
//! File handles are [`FileObj`]s in the generic handle table, so
//! `CloseHandle` / `DuplicateHandle` work uniformly. Overlapped I/O is
//! completed synchronously (allowed by the API contract: ReadFile returns
//! TRUE, `OVERLAPPED.Internal*` are filled and `hEvent` is signalled).

use crate::handles::{self, Event, KObject};
use crate::path;
use crate::strings::*;
use crate::teb;
use crate::types::*;
use std::any::Any;
use std::ffi::c_void;
use std::fs::File;
use std::mem::ManuallyDrop;
use std::os::unix::fs::{FileExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

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
    completion_modes: std::sync::atomic::AtomicU8,
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
            completion_modes: std::sync::atomic::AtomicU8::new(0),
        }
    }
    fn notify_iocp(&self, ov: *mut OVERLAPPED, bytes: usize) {
        const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u8 = 0x1;
        if ov.is_null() {
            return;
        }
        // Per Win32: low bit of OVERLAPPED.hEvent suppresses IOCP notification, and
        // FILE_SKIP_COMPLETION_PORT_ON_SUCCESS skips posting on synchronous completion.
        let ev = unsafe { (*ov).h_event as usize };
        if (ev & 1) != 0 {
            return;
        }
        if (self.completion_modes.load(std::sync::atomic::Ordering::Relaxed) & FILE_SKIP_COMPLETION_PORT_ON_SUCCESS) != 0 {
            return;
        }
        if let Some((iocp, key)) = self.iocp.lock().unwrap().as_ref().cloned() {
            iocp.post(IocpPacket { bytes: bytes as u32, key, overlapped: ov as usize });
        }
    }
    pub fn fd(&self) -> i32 {
        self.file.as_raw_fd()
    }
    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for FileObj {
    fn drop(&mut self) {
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
        Some(self)
    }
}

/// A file handle is signalled whenever no I/O is pending (always, for us).
impl handles::Waitable for FileObj {
    fn is_signaled(&self, _g: &handles::WaitGuard, _tid: u32) -> bool {
        true
    }
    fn acquire(&self, _g: &handles::WaitGuard, _tid: u32) -> bool {
        false
    }
}

/// Called by `CloseHandle` before the generic table; nothing special needed
/// since file handles live in the table and close their fd on drop.
pub fn close_handle_hook(_h: HANDLE) -> bool {
    false
}

pub fn get_file(h: HANDLE) -> Option<Arc<FileObj>> {
    let h = resolve_std_pseudo(h);
    handles::get_as::<FileObj>(h)
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
        libc::S_IFCHR => FileKind::Console,
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
pub unsafe extern "win64" fn SetConsoleCtrlHandler(_h: *const c_void, _add: BOOL) -> BOOL {
    stub!("console control handlers are never invoked");
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
    let wants_read = access & (GENERIC_READ | GENERIC_ALL | FILE_READ_DATA) != 0;
    let wants_write = access & (GENERIC_WRITE | GENERIC_ALL | FILE_WRITE_DATA | FILE_APPEND_DATA) != 0;

    if is_raw_device(name) {
        log::debug!("CreateFile: device handle for {name}");
        let o = FileObj::new(null_file(), FileKind::Device, name.into(), None, access, flags);
        return Ok(handles::insert(Arc::new(o)));
    }
    if let Some(dev) = path::device_name(name) {
        let (file, kind) = match dev {
            "NUL" => (null_file(), FileKind::Null),
            "CONIN$" => (unsafe { File::from_raw_fd(libc::dup(0)) }, FileKind::Console),
            _ => (unsafe { File::from_raw_fd(libc::dup(if wants_read && !wants_write { 0 } else { 1 })) }, FileKind::Console),
        };
        let o = FileObj::new(file, kind, dev.into(), None, access, flags);
        return Ok(handles::insert(Arc::new(o)));
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
        return Ok(handles::insert(Arc::new(o)));
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
    let h = handles::insert(Arc::new(o));
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
    _sec: *mut c_void,
    disposition: u32,
    flags: u32,
    _template: HANDLE,
) -> HANDLE {
    let n = from_wide(name);
    if let Some(h) = crate::misc::hid::create_file(&n, access, flags) {
        return h;
    }
    match create_file(&n, access, disposition, flags) {
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

const STATUS_SUCCESS: usize = 0;
const STATUS_END_OF_FILE: usize = 0xC000_0011;

unsafe fn complete_overlapped(ov: *mut OVERLAPPED, status: usize, n: usize) {
    (*ov).internal = status;
    (*ov).internal_high = n;
    let ev = (*ov).h_event;
    if !ev.is_null() {
        // Low bit set = "don't signal a completion port"; still a valid event.
        let ev = (ev as usize & !1) as HANDLE;
        if let Some(e) = handles::get_as::<Event>(ev) {
            e.set();
        }
    }
}

pub unsafe extern "win64" fn ReadFile(h: HANDLE, buf: *mut c_void, n: u32, read: *mut u32, ov: *mut OVERLAPPED) -> BOOL {
    if let Some(r) = crate::misc::hid::read_file(h, buf, n, read, ov) {
        return r;
    }
    if !read.is_null() {
        *read = 0;
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let slice = std::slice::from_raw_parts_mut(buf as *mut u8, n as usize);
    let io_scope = (f.kind == FileKind::Disk).then(|| crate::hitch::Scope::enter(crate::hitch::ActKind::FileIo, n as u64));
    let res = if !ov.is_null() && f.kind == FileKind::Disk {
        let off = (*ov).offset as u64 | (((*ov).offset_high as u64) << 32);
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
                *read = got as u32;
            }
            if !ov.is_null() {
                if got == 0 && n > 0 && f.kind == FileKind::Disk {
                    complete_overlapped(ov, STATUS_END_OF_FILE, 0);
                    set_err(err::ERROR_HANDLE_EOF);
                    return FALSE;
                }
                complete_overlapped(ov, STATUS_SUCCESS, got);
                f.notify_iocp(ov, got);
            }
            TRUE
        }
        Err(e) => {
            if f.kind == FileKind::Pipe && e.raw_os_error() == Some(libc::EPIPE) {
                set_err(err::ERROR_BROKEN_PIPE);
            } else {
                log::warn!("ReadFile({:?}, n={n}) failed: {e}", f.win_path);
                set_err(io_err(&e));
            }
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
        *written = 0;
    }
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let slice = std::slice::from_raw_parts(buf as *const u8, n as usize);
    let io_scope = (f.kind == FileKind::Disk).then(|| crate::hitch::Scope::enter(crate::hitch::ActKind::FileIo, n as u64));
    let res = if !ov.is_null() && f.kind == FileKind::Disk {
        let off = (*ov).offset as u64 | (((*ov).offset_high as u64) << 32);
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
                *written = k as u32;
            }
            if !ov.is_null() {
                complete_overlapped(ov, STATUS_SUCCESS, k);
                f.notify_iocp(ov, k);
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

const STATUS_PENDING: usize = 0x103;

pub unsafe extern "win64" fn GetOverlappedResult(h: HANDLE, ov: *mut OVERLAPPED, n: *mut u32, wait: BOOL) -> BOOL {
    GetOverlappedResultEx(h, ov, n, if wait != 0 { INFINITE } else { 0 }, FALSE)
}

/// Disk/pipe I/O completes synchronously; device I/O (misc::hid) may still
/// be pending, completed from another thread (buffer, InternalHigh, then a
/// release store of Internal, then the event).
pub unsafe extern "win64" fn GetOverlappedResultEx(h: HANDLE, ov: *mut OVERLAPPED, n: *mut u32, ms: u32, alertable: BOOL) -> BOOL {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let internal = || AtomicUsize::from_ptr(std::ptr::addr_of_mut!((*ov).internal)).load(Ordering::Acquire);
    if internal() == STATUS_PENDING {
        if ms == 0 {
            set_err(err::ERROR_IO_INCOMPLETE);
            return FALSE;
        }
        let ev = (*ov).h_event;
        let target = if ev.is_null() { h } else { (ev as usize & !1) as HANDLE };
        let Some(obj) = handles::get(target) else {
            set_err(err::ERROR_INVALID_HANDLE);
            return FALSE;
        };
        match handles::wait_for(&[obj], false, ms, alertable != 0) {
            handles::WaitResult::Timeout => {
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
            _ => {}
        }
    }
    let status = internal();
    if !n.is_null() {
        *n = std::ptr::read_volatile(std::ptr::addr_of!((*ov).internal_high)) as u32;
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
            set_err(crate::ntdll::ntstatus_to_win32(s as u32));
            FALSE
        }
    }
}

pub unsafe extern "win64" fn CancelIo(h: HANDLE) -> BOOL {
    if let Some(r) = crate::misc::hid::cancel_io(h, std::ptr::null_mut(), false) {
        return r;
    }
    TRUE // file I/O completes synchronously; nothing to cancel
}
pub unsafe extern "win64" fn CancelIoEx(h: HANDLE, ov: *mut OVERLAPPED) -> BOOL {
    if let Some(r) = crate::misc::hid::cancel_io(h, ov, true) {
        return r;
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
struct IocpPacket {
    bytes: u32,
    key: usize,
    overlapped: usize,
}

struct IoCompletionPort {
    queue: std::sync::Mutex<std::collections::VecDeque<IocpPacket>>,
    cv: std::sync::Condvar,
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
            cv: std::sync::Condvar::new(),
        }
    }

    fn post(&self, pkt: IocpPacket) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(pkt);
        self.cv.notify_one();
    }

    fn dequeue_up_to(&self, max: usize, timeout_ms: u32) -> Vec<IocpPacket> {
        let mut q = self.queue.lock().unwrap();
        let deadline = if timeout_ms == u32::MAX {
            None
        } else {
            Some(std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms as u64))
        };
        while q.is_empty() {
            match deadline {
                None => {
                    q = self.cv.wait(q).unwrap();
                }
                Some(dl) => {
                    let now = std::time::Instant::now();
                    if now >= dl {
                        return Vec::new();
                    }
                    let (g, res) = self.cv.wait_timeout(q, dl - now).unwrap();
                    q = g;
                    if res.timed_out() && q.is_empty() {
                        return Vec::new();
                    }
                }
            }
        }
        let n = q.len().min(max.max(1));
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(pkt) = q.pop_front() {
                out.push(pkt);
            }
        }
        out
    }
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
            *f.iocp.lock().unwrap() = Some((iocp, completion_key));
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
    iocp.post(IocpPacket { bytes, key, overlapped: overlapped as usize });
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
    let mut got = iocp.dequeue_up_to(1, timeout_ms);
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
    if alertable != 0 && crate::thread::has_pending_apc() {
        crate::thread::run_pending_apcs();
        std::ptr::write_unaligned(num_removed, 0);
        set_err(WAIT_IO_COMPLETION);
        return FALSE;
    }
    let got = iocp.dequeue_up_to(count as usize, timeout_ms);
    if got.is_empty() {
        std::ptr::write_unaligned(num_removed, 0);
        set_err(WAIT_TIMEOUT);
        return FALSE;
    }
    for (i, pkt) in got.iter().enumerate() {
        std::ptr::write_unaligned(
            entries.add(i),
            OVERLAPPED_ENTRY {
                lpCompletionKey: pkt.key,
                lpOverlapped: pkt.overlapped as *mut OVERLAPPED,
                Internal: 0,
                dwNumberOfBytesTransferred: pkt.bytes,
            },
        );
    }
    std::ptr::write_unaligned(num_removed, got.len() as u32);
    TRUE
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
    }
    TRUE
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

pub unsafe extern "win64" fn ReadDirectoryChangesW(
    _h: HANDLE,
    _buf: *mut c_void,
    _len: u32,
    _subtree: BOOL,
    _filter: u32,
    ret: *mut u32,
    ov: *mut OVERLAPPED,
    _cb: *const c_void,
) -> BOOL {
    if !ret.is_null() {
        *ret = 0;
    }
    if !ov.is_null() {
        // Leave overlapped request in pending state so async directory watchers wait on their event without spinning.
        (*ov).internal = STATUS_PENDING;
        (*ov).internal_high = 0;
        set_err(err::ERROR_IO_PENDING);
        return FALSE;
    }
    set_err(err::ERROR_NOT_SUPPORTED);
    FALSE
}

pub unsafe extern "win64" fn FindFirstChangeNotificationW(_p: LPCWSTR, _sub: BOOL, _f: u32) -> HANDLE {
    // Return a non-signaled manual-reset event handle so file-watcher threads in IDEs/non-gaming apps can WaitForMultipleObjects cleanly.
    crate::kernel32::threads::CreateEventW(std::ptr::null_mut(), TRUE, FALSE, std::ptr::null())
}

pub unsafe extern "win64" fn FindFirstChangeNotificationA(p: LPCSTR, sub: BOOL, f: u32) -> HANDLE {
    let w = to_wide_z(&from_ansi(p));
    FindFirstChangeNotificationW(w.as_ptr(), sub, f)
}

pub unsafe extern "win64" fn FindNextChangeNotification(h: HANDLE) -> BOOL {
    crate::kernel32::threads::ResetEvent(h)
}

pub unsafe extern "win64" fn FindCloseChangeNotification(h: HANDLE) -> BOOL {
    handles::close(h) as BOOL
}

// ---------------------------------------------------------------------------
// Pipes (Anonymous & Named Pipes for CLI / Subprocess / IPC)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn CreatePipe(rd: *mut HANDLE, wr: *mut HANDLE, _sec: *mut c_void, _size: u32) -> BOOL {
    let mut fds = [0i32; 2];
    if libc::pipe(fds.as_mut_ptr()) != 0 {
        set_err(errno_to_win32(last_errno()));
        return FALSE;
    }
    let r = FileObj::new(File::from_raw_fd(fds[0]), FileKind::Pipe, "\\\\.\\pipe\\anon".into(), None, GENERIC_READ, 0);
    let w = FileObj::new(File::from_raw_fd(fds[1]), FileKind::Pipe, "\\\\.\\pipe\\anon".into(), None, GENERIC_WRITE, 0);
    *rd = handles::insert(Arc::new(r));
    *wr = handles::insert(Arc::new(w));
    TRUE
}

pub unsafe extern "win64" fn PeekNamedPipe(h: HANDLE, buf: *mut c_void, size: u32, read: *mut u32, avail: *mut u32, left: *mut u32) -> BOOL {
    let Some(f) = get_file(h) else {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let mut n: libc::c_int = 0;
    if libc::ioctl(f.fd(), libc::FIONREAD, &mut n) != 0 {
        set_err(err::ERROR_BROKEN_PIPE);
        return FALSE;
    }
    if !buf.is_null() && size > 0 && n > 0 {
        stub!("PeekNamedPipe does not copy data");
    }
    if !read.is_null() {
        *read = 0;
    }
    if !avail.is_null() {
        *avail = n as u32;
    }
    if !left.is_null() {
        *left = 0;
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
    _sec: *mut c_void,
) -> HANDLE {
    let pipe_name = from_wide(name);
    let mut fds = [0i32; 2];
    if libc::pipe(fds.as_mut_ptr()) != 0 {
        set_err(errno_to_win32(last_errno()));
        return INVALID_HANDLE_VALUE;
    }
    // Keep read end and close extra write fd once wrapped
    let _ = libc::close(fds[1]);
    let obj = FileObj::new(
        File::from_raw_fd(fds[0]),
        FileKind::Pipe,
        pipe_name,
        None,
        GENERIC_READ | GENERIC_WRITE,
        open_mode,
    );
    handles::insert(Arc::new(obj))
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
    let w = to_wide_z(&from_ansi(name));
    CreateNamedPipeW(w.as_ptr(), open_mode, pipe_mode, max_instances, out_buf, in_buf, timeout, sec)
}

pub unsafe extern "win64" fn ConnectNamedPipe(_h: HANDLE, ov: *mut OVERLAPPED) -> BOOL {
    if !ov.is_null() {
        (*ov).internal = STATUS_PENDING;
        set_err(err::ERROR_IO_PENDING);
        return FALSE;
    }
    set_err(535); // ERROR_PIPE_CONNECTED
    FALSE
}

pub unsafe extern "win64" fn DisconnectNamedPipe(_h: HANDLE) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetNamedPipeHandleState(_h: HANDLE, _mode: *const u32, _max_count: *const u32, _timeout: *const u32) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn GetNamedPipeInfo(_h: HANDLE, flags: *mut u32, out_sz: *mut u32, in_sz: *mut u32, max_inst: *mut u32) -> BOOL {
    if !flags.is_null() {
        *flags = 0;
    }
    if !out_sz.is_null() {
        *out_sz = 65536;
    }
    if !in_sz.is_null() {
        *in_sz = 65536;
    }
    if !max_inst.is_null() {
        *max_inst = 255;
    }
    TRUE
}

pub unsafe extern "win64" fn WaitNamedPipeW(_name: LPCWSTR, _timeout: u32) -> BOOL {
    TRUE
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
}
