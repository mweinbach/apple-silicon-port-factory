//! ucrt environment (`getenv` & co., `_getcwd`) and filesystem helpers
//! (`_stat64` family, `_access`, `_mkdir`, `remove`, `rename`, ...).
//! Paths are Windows paths mapped through [`crate::path`].

use super::errno::{self, EACCES, EINVAL, ENOENT, ERANGE};
use crate::process as pr;
use crate::strings::{from_ansi, from_wide, leak_ansi_z, leak_wide_z};
use std::collections::HashMap;
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Environment
// ---------------------------------------------------------------------------

/// Stable pointers for getenv results: (NAME, value) -> leaked string.
static GETENV_CACHE: Mutex<Option<HashMap<(String, String), (usize, usize)>>> = Mutex::new(None);

fn cached(name: &str, value: &str) -> (usize, usize) {
    let mut g = GETENV_CACHE.lock().unwrap();
    let m = g.get_or_insert_with(HashMap::new);
    *m.entry((name.to_ascii_uppercase(), value.to_string()))
        .or_insert_with(|| (leak_ansi_z(value) as usize, leak_wide_z(value) as usize))
}

pub unsafe extern "win64" fn getenv(name: *const u8) -> *mut u8 {
    if name.is_null() {
        errno::set(EINVAL);
        return std::ptr::null_mut();
    }
    let n = from_ansi(name);
    match pr::get_env(&n) {
        Some(v) => cached(&n, &v).0 as *mut u8,
        None => std::ptr::null_mut(),
    }
}
pub unsafe extern "win64" fn _wgetenv(name: *const u16) -> *mut u16 {
    if name.is_null() {
        errno::set(EINVAL);
        return std::ptr::null_mut();
    }
    let n = from_wide(name);
    match pr::get_env(&n) {
        Some(v) => cached(&n, &v).1 as *mut u16,
        None => std::ptr::null_mut(),
    }
}

pub unsafe extern "win64" fn getenv_s(req: *mut usize, buf: *mut u8, n: usize, name: *const u8) -> i32 {
    if req.is_null() || name.is_null() || (buf.is_null() && n > 0) {
        errno::set(EINVAL);
        return EINVAL;
    }
    match pr::get_env(&from_ansi(name)) {
        None => {
            *req = 0;
            if !buf.is_null() && n > 0 {
                *buf = 0;
            }
            0
        }
        Some(v) => {
            *req = v.len() + 1;
            if buf.is_null() || n == 0 {
                return 0;
            }
            if v.len() + 1 > n {
                *buf = 0;
                errno::set(ERANGE);
                return ERANGE;
            }
            std::ptr::copy_nonoverlapping(v.as_ptr(), buf, v.len());
            *buf.add(v.len()) = 0;
            0
        }
    }
}

pub unsafe extern "win64" fn _wgetenv_s(req: *mut usize, buf: *mut u16, n: usize, name: *const u16) -> i32 {
    if req.is_null() || name.is_null() || (buf.is_null() && n > 0) {
        errno::set(EINVAL);
        return EINVAL;
    }
    match pr::get_env(&from_wide(name)) {
        None => {
            *req = 0;
            if !buf.is_null() && n > 0 {
                *buf = 0;
            }
            0
        }
        Some(v) => {
            let w: Vec<u16> = v.encode_utf16().collect();
            *req = w.len() + 1;
            if buf.is_null() || n == 0 {
                return 0;
            }
            if w.len() + 1 > n {
                *buf = 0;
                errno::set(ERANGE);
                return ERANGE;
            }
            std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
            *buf.add(w.len()) = 0;
            0
        }
    }
}

pub unsafe extern "win64" fn _dupenv_s(out: *mut *mut u8, len: *mut usize, name: *const u8) -> i32 {
    if out.is_null() || name.is_null() {
        return EINVAL;
    }
    match pr::get_env(&from_ansi(name)) {
        None => {
            *out = std::ptr::null_mut();
            if !len.is_null() {
                *len = 0;
            }
            0
        }
        Some(v) => {
            let p = crate::heap::alloc(v.len() + 1, false) as *mut u8;
            std::ptr::copy_nonoverlapping(v.as_ptr(), p, v.len());
            *p.add(v.len()) = 0;
            *out = p;
            if !len.is_null() {
                *len = v.len() + 1;
            }
            0
        }
    }
}

fn putenv_kv(name: &str, value: &str) -> i32 {
    if name.is_empty() || name.contains('=') {
        errno::set(EINVAL);
        return -1;
    }
    pr::set_env(name, if value.is_empty() { None } else { Some(value) });
    unsafe { super::runtime::rebuild_environ() };
    0
}

pub unsafe extern "win64" fn _putenv(s: *const u8) -> i32 {
    if s.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    let s = from_ansi(s);
    match s.split_once('=') {
        Some((k, v)) => putenv_kv(k, v),
        None => {
            errno::set(EINVAL);
            -1
        }
    }
}
pub unsafe extern "win64" fn _wputenv(s: *const u16) -> i32 {
    if s.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    let s = from_wide(s);
    match s.split_once('=') {
        Some((k, v)) => putenv_kv(k, v),
        None => {
            errno::set(EINVAL);
            -1
        }
    }
}
pub unsafe extern "win64" fn _putenv_s(k: *const u8, v: *const u8) -> i32 {
    if k.is_null() || v.is_null() {
        return EINVAL;
    }
    if putenv_kv(&from_ansi(k), &from_ansi(v)) != 0 {
        EINVAL
    } else {
        0
    }
}
pub unsafe extern "win64" fn _wputenv_s(k: *const u16, v: *const u16) -> i32 {
    if k.is_null() || v.is_null() {
        return EINVAL;
    }
    if putenv_kv(&from_wide(k), &from_wide(v)) != 0 {
        EINVAL
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// Current directory
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn _getcwd(buf: *mut u8, max: i32) -> *mut u8 {
    let c = crate::path::cwd();
    let need = c.len() + 1;
    if buf.is_null() {
        let n = need.max(max.max(0) as usize);
        let p = crate::heap::alloc(n, false) as *mut u8;
        std::ptr::copy_nonoverlapping(c.as_ptr(), p, c.len());
        *p.add(c.len()) = 0;
        return p;
    }
    if (max as usize) < need {
        errno::set(ERANGE);
        return std::ptr::null_mut();
    }
    std::ptr::copy_nonoverlapping(c.as_ptr(), buf, c.len());
    *buf.add(c.len()) = 0;
    buf
}

pub unsafe extern "win64" fn _wgetcwd(buf: *mut u16, max: i32) -> *mut u16 {
    let w: Vec<u16> = crate::path::cwd().encode_utf16().collect();
    let need = w.len() + 1;
    if buf.is_null() {
        let n = need.max(max.max(0) as usize);
        let p = crate::heap::alloc(n * 2, false) as *mut u16;
        std::ptr::copy_nonoverlapping(w.as_ptr(), p, w.len());
        *p.add(w.len()) = 0;
        return p;
    }
    if (max as usize) < need {
        errno::set(ERANGE);
        return std::ptr::null_mut();
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
    *buf.add(w.len()) = 0;
    buf
}

fn chdir_str(p: &str) -> i32 {
    match crate::path::to_host(p) {
        Some(h) if h.is_dir() => {
            crate::path::set_cwd(p);
            0
        }
        _ => {
            errno::set(ENOENT);
            -1
        }
    }
}
pub unsafe extern "win64" fn _chdir(p: *const u8) -> i32 {
    if p.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    chdir_str(&from_ansi(p))
}
pub unsafe extern "win64" fn _wchdir(p: *const u16) -> i32 {
    if p.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    chdir_str(&from_wide(p))
}

// ---------------------------------------------------------------------------
// stat family
// ---------------------------------------------------------------------------

struct StatInfo {
    mode: u16,
    size: i64,
    atime: i64,
    mtime: i64,
    ctime: i64,
    rdev_dev: u32,
}

fn mode_bits(st: &libc::stat, win_path: Option<&str>) -> u16 {
    let fmt = st.st_mode as u32 & libc::S_IFMT as u32;
    let mut m: u16 = match fmt {
        x if x == libc::S_IFDIR as u32 => 0x4000,
        x if x == libc::S_IFREG as u32 => 0x8000,
        x if x == libc::S_IFIFO as u32 => 0x1000,
        _ => 0x2000,
    };
    let mut perm: u16 = 0x100; // read
    if st.st_mode as u32 & 0o200 != 0 {
        perm |= 0x80;
    }
    let exec_ext = win_path.is_some_and(|p| {
        let l = p.to_ascii_lowercase();
        [".exe", ".com", ".bat", ".cmd"].iter().any(|e| l.ends_with(e))
    });
    if m == 0x4000 || exec_ext {
        perm |= 0x40;
    }
    m |= perm | (perm >> 3) | (perm >> 6);
    m
}

fn stat_host(st: &libc::stat, win: Option<&str>) -> StatInfo {
    let drive = win.and_then(|p| p.as_bytes().first().copied()).filter(|c| c.is_ascii_alphabetic()).map(|c| (c.to_ascii_uppercase() - b'A') as u32);
    StatInfo {
        mode: mode_bits(st, win),
        size: st.st_size,
        atime: st.st_atime,
        mtime: st.st_mtime,
        ctime: st.st_ctime,
        rdev_dev: drive.unwrap_or(2),
    }
}

fn stat_path(win: &str) -> Option<StatInfo> {
    let full = crate::path::absolute(win);
    let Some(h) = crate::path::to_host(&full) else {
        errno::set(ENOENT);
        return None;
    };
    let c = std::ffi::CString::new(h.as_os_str().as_encoded_bytes()).ok()?;
    let mut st: libc::stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::stat(c.as_ptr(), &mut st) } != 0 {
        errno::set_from_host();
        return None;
    }
    Some(stat_host(&st, Some(&full)))
}

fn stat_fd(fd: i32) -> Option<StatInfo> {
    let mut st: libc::stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::fstat(fd, &mut st) } != 0 {
        errno::set(super::errno::EBADF);
        return None;
    }
    let mut i = stat_host(&st, None);
    if i.mode & 0xF000 != 0x8000 {
        i.rdev_dev = fd as u32;
    } else {
        i.rdev_dev = 0;
    }
    Some(i)
}

/// Layouts: (size field offset, size is 64-bit, times are 64-bit).
#[derive(Clone, Copy)]
enum Layout {
    S64,    // _stat64: size i64 @24, times i64 @32
    S64i32, // _stat64i32: size i32 @20, times i64 @24
    S32i64, // _stat32i64: size i64 @24, times i32 @32
    S32,    // _stat32: size i32 @20, times i32 @24
}

unsafe fn fill(p: *mut u8, i: &StatInfo, l: Layout) {
    let w16 = |o: usize, v: u16| std::ptr::write_unaligned(p.add(o) as *mut u16, v);
    let w32 = |o: usize, v: u32| std::ptr::write_unaligned(p.add(o) as *mut u32, v);
    let w64 = |o: usize, v: u64| std::ptr::write_unaligned(p.add(o) as *mut u64, v);
    let size = match l {
        Layout::S64 => 56,
        Layout::S64i32 | Layout::S32i64 => 48,
        Layout::S32 => 36,
    };
    std::ptr::write_bytes(p, 0, size);
    w32(0, i.rdev_dev);
    w16(6, i.mode);
    w16(8, 1);
    w32(16, i.rdev_dev);
    match l {
        Layout::S64 => {
            w64(24, i.size as u64);
            w64(32, i.atime as u64);
            w64(40, i.mtime as u64);
            w64(48, i.ctime as u64);
        }
        Layout::S64i32 => {
            w32(20, i.size as u32);
            w64(24, i.atime as u64);
            w64(32, i.mtime as u64);
            w64(40, i.ctime as u64);
        }
        Layout::S32i64 => {
            w64(24, i.size as u64);
            w32(32, i.atime as u32);
            w32(36, i.mtime as u32);
            w32(40, i.ctime as u32);
        }
        Layout::S32 => {
            w32(20, i.size as u32);
            w32(24, i.atime as u32);
            w32(28, i.mtime as u32);
            w32(32, i.ctime as u32);
        }
    }
}

macro_rules! stat_fns {
    ($($name:ident, $wname:ident, $fname:ident => $layout:expr;)*) => {$(
        pub unsafe extern "win64" fn $name(path: *const u8, buf: *mut u8) -> i32 {
            if path.is_null() || buf.is_null() { errno::set(EINVAL); return -1; }
            match stat_path(&from_ansi(path)) { Some(i) => { fill(buf, &i, $layout); 0 } None => -1 }
        }
        pub unsafe extern "win64" fn $wname(path: *const u16, buf: *mut u8) -> i32 {
            if path.is_null() || buf.is_null() { errno::set(EINVAL); return -1; }
            match stat_path(&from_wide(path)) { Some(i) => { fill(buf, &i, $layout); 0 } None => -1 }
        }
        pub unsafe extern "win64" fn $fname(fd: i32, buf: *mut u8) -> i32 {
            if buf.is_null() { errno::set(EINVAL); return -1; }
            match stat_fd(fd) { Some(i) => { fill(buf, &i, $layout); 0 } None => -1 }
        }
    )*};
}

// @exports _stat64 _wstat64 _fstat64 _stat64i32 _wstat64i32 _fstat64i32 _stat32i64 _wstat32i64 _fstat32i64 _stat32 _wstat32 _fstat32
stat_fns! {
    _stat64, _wstat64, _fstat64 => Layout::S64;
    _stat64i32, _wstat64i32, _fstat64i32 => Layout::S64i32;
    _stat32i64, _wstat32i64, _fstat32i64 => Layout::S32i64;
    _stat32, _wstat32, _fstat32 => Layout::S32;
}

// ---------------------------------------------------------------------------
// access / mkdir / rmdir / remove / rename
// ---------------------------------------------------------------------------

fn access_str(p: &str, mode: i32) -> i32 {
    let Some(i) = stat_path(p) else { return errno::ENOENT };
    if mode & 2 != 0 && i.mode & 0x80 == 0 {
        return EACCES;
    }
    if mode & !6 != 0 {
        return EINVAL;
    }
    0
}
pub unsafe extern "win64" fn _access(p: *const u8, mode: i32) -> i32 {
    if p.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    match access_str(&from_ansi(p), mode) {
        0 => 0,
        e => {
            errno::set(e);
            -1
        }
    }
}
pub unsafe extern "win64" fn _waccess(p: *const u16, mode: i32) -> i32 {
    if p.is_null() {
        errno::set(EINVAL);
        return -1;
    }
    match access_str(&from_wide(p), mode) {
        0 => 0,
        e => {
            errno::set(e);
            -1
        }
    }
}
pub unsafe extern "win64" fn _access_s(p: *const u8, mode: i32) -> i32 {
    if p.is_null() {
        return EINVAL;
    }
    access_str(&from_ansi(p), mode)
}
pub unsafe extern "win64" fn _waccess_s(p: *const u16, mode: i32) -> i32 {
    if p.is_null() {
        return EINVAL;
    }
    access_str(&from_wide(p), mode)
}

fn io_err(e: std::io::Error) -> i32 {
    let code = e.raw_os_error().map(errno::from_host).unwrap_or(EINVAL);
    errno::set(code);
    -1
}

fn host_or_enoent(p: &str) -> Option<std::path::PathBuf> {
    let h = crate::path::to_host(p);
    if h.is_none() {
        errno::set(ENOENT);
    }
    h
}

fn mkdir_str(p: &str) -> i32 {
    let Some(h) = host_or_enoent(p) else { return -1 };
    if crate::path::is_protected_host_write(&h) {
        errno::set(EACCES);
        return -1;
    }
    std::fs::create_dir(&h).map(|_| 0).unwrap_or_else(io_err)
}
fn rmdir_str(p: &str) -> i32 {
    let Some(h) = host_or_enoent(p) else { return -1 };
    if crate::path::is_protected_host_remove_or_rename(&h) {
        errno::set(EACCES);
        return -1;
    }
    std::fs::remove_dir(&h).map(|_| 0).unwrap_or_else(io_err)
}
fn remove_str(p: &str) -> i32 {
    let Some(h) = host_or_enoent(p) else { return -1 };
    if crate::path::is_protected_host_remove_or_rename(&h) {
        errno::set(EACCES);
        return -1;
    }
    if h.is_dir() {
        errno::set(EACCES);
        return -1;
    }
    std::fs::remove_file(&h).map(|_| 0).unwrap_or_else(io_err)
}
fn rename_str(a: &str, b: &str) -> i32 {
    let (Some(ha), Some(hb)) = (host_or_enoent(a), host_or_enoent(b)) else { return -1 };
    if crate::path::is_protected_host_remove_or_rename(&ha)
        || crate::path::is_protected_host_remove_or_rename(&hb)
    {
        errno::set(EACCES);
        return -1;
    }
    if !ha.exists() {
        errno::set(ENOENT);
        return -1;
    }
    // Windows rename() fails if the destination exists.
    if hb.exists() && !hb.as_os_str().eq_ignore_ascii_case(ha.as_os_str()) {
        errno::set(EACCES);
        return -1;
    }
    std::fs::rename(&ha, &hb).map(|_| 0).unwrap_or_else(io_err)
}

pub unsafe extern "win64" fn _mkdir(p: *const u8) -> i32 {
    mkdir_str(&from_ansi(p))
}
pub unsafe extern "win64" fn _wmkdir(p: *const u16) -> i32 {
    mkdir_str(&from_wide(p))
}
pub unsafe extern "win64" fn _rmdir(p: *const u8) -> i32 {
    rmdir_str(&from_ansi(p))
}
pub unsafe extern "win64" fn _wrmdir(p: *const u16) -> i32 {
    rmdir_str(&from_wide(p))
}
pub unsafe extern "win64" fn remove(p: *const u8) -> i32 {
    remove_str(&from_ansi(p))
}
pub unsafe extern "win64" fn _wremove(p: *const u16) -> i32 {
    remove_str(&from_wide(p))
}
pub unsafe extern "win64" fn _unlink(p: *const u8) -> i32 {
    remove_str(&from_ansi(p))
}
pub unsafe extern "win64" fn _wunlink(p: *const u16) -> i32 {
    remove_str(&from_wide(p))
}
pub unsafe extern "win64" fn rename(a: *const u8, b: *const u8) -> i32 {
    rename_str(&from_ansi(a), &from_ansi(b))
}
pub unsafe extern "win64" fn _wrename(a: *const u16, b: *const u16) -> i32 {
    rename_str(&from_wide(a), &from_wide(b))
}

pub unsafe extern "win64" fn _fullpath(buf: *mut u8, p: *const u8, max: usize) -> *mut u8 {
    let full = crate::path::absolute(&from_ansi(p));
    if buf.is_null() {
        return leak_ansi_z(&full);
    }
    if full.len() + 1 > max {
        errno::set(ERANGE);
        return std::ptr::null_mut();
    }
    crate::strings::write_ansi_trunc(&full, buf, max);
    buf
}
pub unsafe extern "win64" fn _wfullpath(buf: *mut u16, p: *const u16, max: usize) -> *mut u16 {
    let full = crate::path::absolute(&from_wide(p));
    if buf.is_null() {
        return leak_wide_z(&full);
    }
    if full.encode_utf16().count() + 1 > max {
        errno::set(ERANGE);
        return std::ptr::null_mut();
    }
    crate::strings::write_wide_trunc(&full, buf, max);
    buf
}

fn split_win_path(s: &str) -> (&str, &str, &str, &str) {
    let bytes = s.as_bytes();
    let (drive, rest) = if bytes.len() >= 2 && bytes[1] == b':' {
        (&s[..2], &s[2..])
    } else {
        ("", s)
    };
    let last_sep = rest.rfind(['\\', '/']);
    let (dir, filename) = match last_sep {
        Some(idx) => (&rest[..=idx], &rest[idx + 1..]),
        None => ("", rest),
    };
    let (fname, ext) = match filename.rfind('.') {
        Some(dot) => (&filename[..dot], &filename[dot..]),
        None => (filename, ""),
    };
    (drive, dir, fname, ext)
}

pub unsafe extern "win64" fn _splitpath_s(
    path: *const u8,
    drive: *mut u8,
    drive_sz: usize,
    dir: *mut u8,
    dir_sz: usize,
    fname: *mut u8,
    fname_sz: usize,
    ext: *mut u8,
    ext_sz: usize,
) -> i32 {
    if path.is_null() {
        return EINVAL;
    }
    let s = from_ansi(path);
    let (d, dr, fnm, ex) = split_win_path(&s);
    if !drive.is_null() && drive_sz > 0 {
        crate::strings::write_ansi_trunc(d, drive, drive_sz);
    }
    if !dir.is_null() && dir_sz > 0 {
        crate::strings::write_ansi_trunc(dr, dir, dir_sz);
    }
    if !fname.is_null() && fname_sz > 0 {
        crate::strings::write_ansi_trunc(fnm, fname, fname_sz);
    }
    if !ext.is_null() && ext_sz > 0 {
        crate::strings::write_ansi_trunc(ex, ext, ext_sz);
    }
    0
}

pub unsafe extern "win64" fn _splitpath(
    path: *const u8,
    drive: *mut u8,
    dir: *mut u8,
    fname: *mut u8,
    ext: *mut u8,
) {
    let _ = _splitpath_s(path, drive, 4, dir, 256, fname, 256, ext, 256);
}

pub unsafe extern "win64" fn _wsplitpath_s(
    path: *const u16,
    drive: *mut u16,
    drive_sz: usize,
    dir: *mut u16,
    dir_sz: usize,
    fname: *mut u16,
    fname_sz: usize,
    ext: *mut u16,
    ext_sz: usize,
) -> i32 {
    if path.is_null() {
        return EINVAL;
    }
    let s = from_wide(path);
    let (d, dr, fnm, ex) = split_win_path(&s);
    if !drive.is_null() && drive_sz > 0 {
        crate::strings::write_wide_trunc(d, drive, drive_sz);
    }
    if !dir.is_null() && dir_sz > 0 {
        crate::strings::write_wide_trunc(dr, dir, dir_sz);
    }
    if !fname.is_null() && fname_sz > 0 {
        crate::strings::write_wide_trunc(fnm, fname, fname_sz);
    }
    if !ext.is_null() && ext_sz > 0 {
        crate::strings::write_wide_trunc(ex, ext, ext_sz);
    }
    0
}

fn narrow_dir_ends_with_sep(dir: &[u8]) -> bool {
    if dir.is_empty() {
        return false;
    }
    let cp = unsafe { super::locale::_getmbcp().max(0) as u32 };
    if cp == 0 || cp == 65001 {
        return matches!(dir.last(), Some(b'\\' | b'/'));
    }
    // Walk DBCS character boundaries so a 0x5C trail byte (e.g. CP932 '表' = [0x95, 0x5C])
    // is not mistaken for a trailing backslash separator.
    let mut i = 0usize;
    let mut last_single = None;
    while i < dir.len() {
        if unsafe { crate::kernel32::string::IsDBCSLeadByteEx(cp, dir[i]) != 0 } && i + 1 < dir.len() {
            last_single = None;
            i += 2;
        } else {
            last_single = Some(dir[i]);
            i += 1;
        }
    }
    matches!(last_single, Some(b'\\' | b'/'))
}

fn compose_win_path_bytes(
    drive: Option<&[u8]>,
    dir: Option<&[u8]>,
    fname: Option<&[u8]>,
    ext: Option<&[u8]>,
) -> Vec<u8> {
    let mut out = Vec::new();
    if let Some(d) = drive {
        if let Some(&b0) = d.first() {
            out.push(b0);
            out.push(b':');
        }
    }
    if let Some(dr) = dir {
        if !dr.is_empty() {
            out.extend_from_slice(dr);
            if !narrow_dir_ends_with_sep(dr) {
                out.push(b'\\');
            }
        }
    }
    if let Some(f) = fname {
        out.extend_from_slice(f);
    }
    if let Some(e) = ext {
        if !e.is_empty() {
            if e[0] != b'.' {
                out.push(b'.');
            }
            out.extend_from_slice(e);
        }
    }
    out
}

fn compose_win_path_wide(
    drive: Option<&[u16]>,
    dir: Option<&[u16]>,
    fname: Option<&[u16]>,
    ext: Option<&[u16]>,
) -> Vec<u16> {
    let mut out = Vec::new();
    if let Some(d) = drive {
        if let Some(&u0) = d.first() {
            out.push(u0);
            out.push(b':' as u16);
        }
    }
    if let Some(dr) = dir {
        if !dr.is_empty() {
            out.extend_from_slice(dr);
            if !matches!(dr.last().copied(), Some(0x005C | 0x002F)) {
                out.push(b'\\' as u16);
            }
        }
    }
    if let Some(f) = fname {
        out.extend_from_slice(f);
    }
    if let Some(e) = ext {
        if !e.is_empty() {
            if e[0] != b'.' as u16 {
                out.push(b'.' as u16);
            }
            out.extend_from_slice(e);
        }
    }
    out
}

pub unsafe extern "win64" fn _makepath_s(
    path: *mut u8,
    size: usize,
    drive: *const u8,
    dir: *const u8,
    fname: *const u8,
    ext: *const u8,
) -> i32 {
    if path.is_null() || size == 0 {
        errno::set(EINVAL);
        super::runtime::invalid_parameter("_makepath_s");
        return EINVAL;
    }
    let d = if drive.is_null() { None } else { Some(crate::strings::cstr_slice(drive)) };
    let dr = if dir.is_null() { None } else { Some(crate::strings::cstr_slice(dir)) };
    let f = if fname.is_null() { None } else { Some(crate::strings::cstr_slice(fname)) };
    let e = if ext.is_null() { None } else { Some(crate::strings::cstr_slice(ext)) };
    let bytes = compose_win_path_bytes(d, dr, f, e);
    if bytes.len() + 1 > size {
        *path = 0;
        errno::set(ERANGE);
        super::runtime::invalid_parameter("_makepath_s");
        return ERANGE;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), path, bytes.len());
    *path.add(bytes.len()) = 0;
    0
}

pub unsafe extern "win64" fn _makepath(
    path: *mut u8,
    drive: *const u8,
    dir: *const u8,
    fname: *const u8,
    ext: *const u8,
) {
    let _ = _makepath_s(path, usize::MAX, drive, dir, fname, ext);
}

pub unsafe extern "win64" fn _wmakepath_s(
    path: *mut u16,
    size: usize,
    drive: *const u16,
    dir: *const u16,
    fname: *const u16,
    ext: *const u16,
) -> i32 {
    if path.is_null() || size == 0 {
        errno::set(EINVAL);
        super::runtime::invalid_parameter("_wmakepath_s");
        return EINVAL;
    }
    let d = if drive.is_null() {
        None
    } else {
        Some(crate::strings::read_wide_units_z(drive, crate::strings::DEFAULT_MAX_STR_LEN))
    };
    let dr = if dir.is_null() {
        None
    } else {
        Some(crate::strings::read_wide_units_z(dir, crate::strings::DEFAULT_MAX_STR_LEN))
    };
    let f = if fname.is_null() {
        None
    } else {
        Some(crate::strings::read_wide_units_z(fname, crate::strings::DEFAULT_MAX_STR_LEN))
    };
    let e = if ext.is_null() {
        None
    } else {
        Some(crate::strings::read_wide_units_z(ext, crate::strings::DEFAULT_MAX_STR_LEN))
    };
    let units = compose_win_path_wide(d.as_deref(), dr.as_deref(), f.as_deref(), e.as_deref());
    if units.len() + 1 > size {
        path.write_unaligned(0);
        errno::set(ERANGE);
        super::runtime::invalid_parameter("_wmakepath_s");
        return ERANGE;
    }
    for (i, &u) in units.iter().enumerate() {
        path.add(i).write_unaligned(u);
    }
    path.add(units.len()).write_unaligned(0);
    0
}

pub unsafe extern "win64" fn _wmakepath(
    path: *mut u16,
    drive: *const u16,
    dir: *const u16,
    fname: *const u16,
    ext: *const u16,
) {
    let _ = _wmakepath_s(path, usize::MAX, drive, dir, fname, ext);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stat_and_dirs() {
        let dir = std::env::temp_dir().join("aw2_env_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("f.bin"), b"12345").unwrap();
        crate::path::add_mount("S:\\", &dir);
        unsafe {
            let mut buf = [0u8; 56];
            assert_eq!(_stat64(b"S:\\f.bin\0".as_ptr(), buf.as_mut_ptr()), 0);
            assert_eq!(u16::from_le_bytes([buf[6], buf[7]]) & 0xF000, 0x8000);
            assert_eq!(i64::from_le_bytes(buf[24..32].try_into().unwrap()), 5);
            assert_eq!(_stat64(b"S:\\nope\0".as_ptr(), buf.as_mut_ptr()), -1);
            assert_eq!(errno::get(), ENOENT);
            assert_eq!(_mkdir(b"S:\\d\0".as_ptr()), 0);
            assert_eq!(_mkdir(b"S:\\d\0".as_ptr()), -1);
            assert_eq!(_access(b"S:\\d\0".as_ptr(), 0), 0);
            assert_eq!(rename(b"S:\\f.bin\0".as_ptr(), b"S:\\g.bin\0".as_ptr()), 0);
            assert_eq!(remove(b"S:\\g.bin\0".as_ptr()), 0);
            assert_eq!(_rmdir(b"S:\\d\0".as_ptr()), 0);
        }
    }

    #[test]
    fn makepath_narrow_wide_unaligned_and_errors() {
        unsafe extern "win64" fn noop_iph(
            _: *const u16,
            _: *const u16,
            _: *const u16,
            _: u32,
            _: usize,
        ) {}
        struct IphGuard(usize);
        impl Drop for IphGuard {
            fn drop(&mut self) {
                unsafe {
                    crate::ucrt::runtime::_set_invalid_parameter_handler(self.0);
                }
            }
        }

        unsafe {
            let _iph = IphGuard(crate::ucrt::runtime::_set_invalid_parameter_handler(
                noop_iph as *const () as usize,
            ));

            // Narrow _makepath_s adds ':' to drive, '\\' to dir without separator, '.' to ext without dot.
            let mut out = [0u8; 64];
            assert_eq!(
                _makepath_s(
                    out.as_mut_ptr(),
                    out.len(),
                    b"C\0".as_ptr(),
                    b"\\Shaders\\DXIL\0".as_ptr(),
                    b"pixel\0".as_ptr(),
                    b"hlsl\0".as_ptr(),
                ),
                0
            );
            assert_eq!(from_ansi(out.as_ptr()), "C:\\Shaders\\DXIL\\pixel.hlsl");

            // Already-formatted drive/dir/ext are not doubled; NULL components are omitted cleanly.
            assert_eq!(
                _makepath_s(
                    out.as_mut_ptr(),
                    out.len(),
                    b"D:\0".as_ptr(),
                    b"sub/\0".as_ptr(),
                    std::ptr::null(),
                    b".bin\0".as_ptr(),
                ),
                0
            );
            assert_eq!(from_ansi(out.as_ptr()), "D:sub/.bin");

            // Narrow _makepath_s preserves arbitrary non-UTF-8 bytes verbatim (e.g. CP932)
            // without UTF-8 replacement expansion, and recognizes DBCS 0x5C trail bytes.
            let prev_mbcp = crate::ucrt::locale::_getmbcp();
            assert_eq!(crate::ucrt::locale::_setmbcp(932), 0);
            let mut exact = [0u8; 8]; // "C:\x95\x5C\\\x82\xA0\0" = 8 bytes
            let cp932_dir: [u8; 3] = [0x95, 0x5C, 0]; // '表' ends with 0x5C trail byte, not '\\'
            let cp932_fname: [u8; 3] = [0x82, 0xA0, 0]; // 'あ'
            assert_eq!(
                _makepath_s(
                    exact.as_mut_ptr(),
                    exact.len(),
                    b"C\0".as_ptr(),
                    cp932_dir.as_ptr(),
                    cp932_fname.as_ptr(),
                    std::ptr::null(),
                ),
                0
            );
            assert_eq!(&exact, &[b'C', b':', 0x95, 0x5C, b'\\', 0x82, 0xA0, 0]);
            assert_eq!(crate::ucrt::locale::_setmbcp(prev_mbcp), 0);

            // Wide _wmakepath_s with unaligned input & output buffers.
            let write_odd_wide = |s: &str| -> Vec<u8> {
                let units: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
                let mut raw = vec![0xAAu8; 1 + units.len() * 2];
                for (i, &u) in units.iter().enumerate() {
                    raw[1 + i * 2..1 + i * 2 + 2].copy_from_slice(&u.to_le_bytes());
                }
                raw
            };
            let rd = write_odd_wide("E");
            let rdir = write_odd_wide("\\game\\bin");
            let rf = write_odd_wide("d3d");
            let re = write_odd_wide("dll");
            let mut raw_out = [0xEEu8; 129];
            let odd_out = raw_out.as_mut_ptr().add(1) as *mut u16;
            assert!(!odd_out.is_aligned());
            assert_eq!(
                _wmakepath_s(
                    odd_out,
                    64,
                    rd.as_ptr().add(1) as *const u16,
                    rdir.as_ptr().add(1) as *const u16,
                    rf.as_ptr().add(1) as *const u16,
                    re.as_ptr().add(1) as *const u16,
                ),
                0
            );
            assert_eq!(from_wide(odd_out), "E:\\game\\bin\\d3d.dll");

            // Overflow clears path[0], sets ERANGE, and returns ERANGE; null/zero returns EINVAL.
            odd_out.write_unaligned(b'X' as u16);
            errno::set(0);
            assert_eq!(
                _wmakepath_s(
                    odd_out,
                    4,
                    rd.as_ptr().add(1) as *const u16,
                    rdir.as_ptr().add(1) as *const u16,
                    rf.as_ptr().add(1) as *const u16,
                    re.as_ptr().add(1) as *const u16,
                ),
                ERANGE
            );
            assert_eq!(odd_out.read_unaligned(), 0);
            assert_eq!(errno::get(), ERANGE);

            errno::set(0);
            assert_eq!(
                _wmakepath_s(
                    std::ptr::null_mut(),
                    16,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                ),
                EINVAL
            );
            assert_eq!(errno::get(), EINVAL);
        }
    }
}
