//! Modules: LoadLibrary / GetModuleHandle / GetProcAddress / resources.

use crate::strings::*;
use crate::types::*;
use crate::ProcName;
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

// ---------------------------------------------------------------------------
// LoadLibraryEx / DLL search path flags & process-wide search state
// ---------------------------------------------------------------------------

pub const DONT_RESOLVE_DLL_REFERENCES: u32 = 0x0000_0001;
pub const LOAD_LIBRARY_AS_DATAFILE: u32 = 0x0000_0002;
pub const LOAD_WITH_ALTERED_SEARCH_PATH: u32 = 0x0000_0008;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: u32 = 0x0000_0010;
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: u32 = 0x0000_0020;
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: u32 = 0x0000_0040;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: u32 = 0x0000_0080;
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: u32 = 0x0000_0100;
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: u32 = 0x0000_0200;
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: u32 = 0x0000_0400;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: u32 = 0x0000_1000;
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: u32 = 0x0000_2000;

/// Valid flags accepted by `SetDefaultDllDirectories`.
pub const VALID_DEFAULT_DLL_DIRECTORIES_MASK: u32 = LOAD_LIBRARY_SEARCH_APPLICATION_DIR
    | LOAD_LIBRARY_SEARCH_USER_DIRS
    | LOAD_LIBRARY_SEARCH_SYSTEM32
    | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS;

/// Search-directory override flags accepted by `LoadLibraryExW`.
pub const LOAD_LIBRARY_SEARCH_FLAGS_MASK: u32 = LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR
    | LOAD_LIBRARY_SEARCH_APPLICATION_DIR
    | LOAD_LIBRARY_SEARCH_USER_DIRS
    | LOAD_LIBRARY_SEARCH_SYSTEM32
    | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS;

/// Datafile / image-resource mapping flags (skip imports, TLS, and DllMain).
pub const DATAFILE_LOAD_FLAGS_MASK: u32 =
    LOAD_LIBRARY_AS_DATAFILE | LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE | LOAD_LIBRARY_AS_IMAGE_RESOURCE;

/// Snapshot of the process-wide Win32 DLL search-path state configured via
/// `SetDefaultDllDirectories`, `AddDllDirectory`, `RemoveDllDirectory`, and
/// `SetDllDirectoryW/A`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DllSearchConfig {
    /// Non-zero `LOAD_LIBRARY_SEARCH_*` mask set by `SetDefaultDllDirectories`, if any.
    pub default_dirs_mask: Option<u32>,
    /// Legacy `SetDllDirectoryW/A` state:
    /// - `None`: default (CWD is included in standard legacy search order)
    /// - `Some(None)`: `SetDllDirectory("")` removed CWD from standard search order
    /// - `Some(Some(win_dir))`: `SetDllDirectory(dir)` replaced CWD with `win_dir`
    pub legacy_dll_dir: Option<Option<String>>,
    /// User directories added via `AddDllDirectory` (normalized Windows paths).
    pub added_dirs: Vec<String>,
}

struct DllSearchState {
    default_dirs_mask: Option<u32>,
    legacy_dll_dir: Option<Option<String>>,
    added_dirs: Vec<(usize, String)>,
}

impl DllSearchState {
    const fn new() -> Self {
        Self {
            default_dirs_mask: None,
            legacy_dll_dir: None,
            added_dirs: Vec::new(),
        }
    }
}

static DLL_SEARCH_STATE: RwLock<DllSearchState> = RwLock::new(DllSearchState::new());
static NEXT_DLL_DIR_COOKIE: AtomicUsize = AtomicUsize::new(0x1_0000);

/// Snapshot current process-wide DLL search configuration for the PE loader.
pub fn dll_search_config() -> DllSearchConfig {
    let st = DLL_SEARCH_STATE.read().unwrap();
    DllSearchConfig {
        default_dirs_mask: st.default_dirs_mask,
        legacy_dll_dir: st.legacy_dll_dir.clone(),
        added_dirs: st.added_dirs.iter().map(|(_, dir)| dir.clone()).collect(),
    }
}

/// Reset process-wide DLL search-path state to Windows startup defaults (for tests).
pub fn reset_dll_search_state() {
    let mut st = DLL_SEARCH_STATE.write().unwrap();
    st.default_dirs_mask = None;
    st.legacy_dll_dir = None;
    st.added_dirs.clear();
}

/// True if `p` is a syntactically absolute Windows path (`C:\...`, `\\server\...`,
/// or `\\?\C:\...` / `\??\C:\...` / `\Device\HarddiskVolume...\...`).
pub fn is_absolute_win_path(p: &str) -> bool {
    let trimmed = p.trim();
    if trimmed.is_empty() {
        return false;
    }
    let norm = crate::path::normalize(trimmed);
    let b = norm.as_bytes();
    (b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && b[2] == b'\\')
        || norm.starts_with("\\\\")
}

/// Validate `LoadLibraryExW` flag combinations against documented Windows rules.
pub fn validate_load_library_flags(name: &str, flags: u32) -> Result<(), u32> {
    // LOAD_WITH_ALTERED_SEARCH_PATH cannot be combined with any LOAD_LIBRARY_SEARCH_* flag.
    if (flags & LOAD_WITH_ALTERED_SEARCH_PATH) != 0
        && (flags & LOAD_LIBRARY_SEARCH_FLAGS_MASK) != 0
    {
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    // LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR requires a fully qualified path.
    if (flags & LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR) != 0 && !is_absolute_win_path(name) {
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    Ok(())
}

fn set_err(e: u32) {
    crate::teb::set_last_error(e);
}

pub unsafe fn load_library(name: &str, flags: u32) -> HMODULE {
    load_library_with_search_path(name, flags, None)
}

pub unsafe fn load_library_with_search_path(
    name: &str,
    flags: u32,
    search_path: Option<&str>,
) -> HMODULE {
    if name.is_empty() {
        set_err(err::ERROR_MOD_NOT_FOUND);
        return std::ptr::null_mut();
    }
    if let Err(e) = validate_load_library_flags(name, flags) {
        log::warn!("LoadLibraryEx({name:?}, 0x{flags:x}) invalid flags: error {e}");
        set_err(e);
        return std::ptr::null_mut();
    }
    match crate::loader().load_library_with_search_path(name, flags, search_path) {
        Ok(h) => h as HMODULE,
        Err(e) => {
            log::warn!("LoadLibrary({name:?}, 0x{flags:x}) failed: error {e}");
            set_err(e);
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn LoadLibraryA(name: LPCSTR) -> HMODULE {
    if name.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    load_library(&read_ansi_z(name, DEFAULT_MAX_STR_LEN), 0)
}
pub unsafe extern "win64" fn LoadLibraryW(name: LPCWSTR) -> HMODULE {
    if name.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    load_library(&read_wide_z(name, DEFAULT_MAX_STR_LEN), 0)
}
pub unsafe extern "win64" fn LoadLibraryExA(name: LPCSTR, file: HANDLE, flags: u32) -> HMODULE {
    if name.is_null() || !file.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    load_library(&read_ansi_z(name, DEFAULT_MAX_STR_LEN), flags)
}
pub unsafe extern "win64" fn LoadLibraryExW(name: LPCWSTR, file: HANDLE, flags: u32) -> HMODULE {
    if name.is_null() || !file.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    load_library(&read_wide_z(name, DEFAULT_MAX_STR_LEN), flags)
}
pub unsafe extern "win64" fn LoadPackagedLibrary(name: LPCWSTR, _r: u32) -> HMODULE {
    if name.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    load_library(&read_wide_z(name, DEFAULT_MAX_STR_LEN), 0)
}

pub unsafe extern "win64" fn FreeLibrary(h: HMODULE) -> BOOL {
    crate::loader().free_library(h as usize) as BOOL
}

pub unsafe extern "win64" fn FreeLibraryAndExitThread(h: HMODULE, code: u32) {
    FreeLibrary(h);
    crate::thread::exit_current_thread(code);
}

pub unsafe extern "win64" fn FreeLibraryWhenCallbackReturns(_inst: *mut c_void, h: HMODULE) {
    FreeLibrary(h);
}

pub unsafe extern "win64" fn DisableThreadLibraryCalls(h: HMODULE) -> BOOL {
    crate::loader().disable_thread_library_calls(h as usize) as BOOL
}

fn get_module_handle(name: Option<String>) -> HMODULE {
    match crate::loader().get_module_handle(name.as_deref()) {
        Some(h) => {
            log::trace!("GetModuleHandle({name:?}) -> 0x{h:x}");
            h as HMODULE
        }
        None => {
            log::debug!("GetModuleHandle({name:?}) -> NULL (not loaded)");
            set_err(err::ERROR_MOD_NOT_FOUND);
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn GetModuleHandleA(name: LPCSTR) -> HMODULE {
    get_module_handle(from_ansi_opt(name))
}
pub unsafe extern "win64" fn GetModuleHandleW(name: LPCWSTR) -> HMODULE {
    get_module_handle(from_wide_opt(name))
}

const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4;

unsafe fn get_module_handle_ex(flags: u32, name: Option<String>, addr: usize, out: *mut HMODULE) -> BOOL {
    let h = if flags & GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS != 0 {
        crate::loader().module_from_address(addr).map(|h| h as HMODULE).unwrap_or(std::ptr::null_mut())
    } else {
        get_module_handle(name)
    };
    if !out.is_null() {
        *out = h;
    }
    if h.is_null() {
        set_err(err::ERROR_MOD_NOT_FOUND);
        FALSE
    } else {
        TRUE
    }
}

pub unsafe extern "win64" fn GetModuleHandleExA(flags: u32, name: LPCSTR, out: *mut HMODULE) -> BOOL {
    let n = if flags & GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS != 0 { None } else { from_ansi_opt(name) };
    get_module_handle_ex(flags, n, name as usize, out)
}
pub unsafe extern "win64" fn GetModuleHandleExW(flags: u32, name: LPCWSTR, out: *mut HMODULE) -> BOOL {
    let n = if flags & GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS != 0 { None } else { from_wide_opt(name) };
    get_module_handle_ex(flags, n, name as usize, out)
}

/// Log a missing procedure lookup (`GetProcAddress` / `LdrGetProcedureAddress`) once per
/// distinct `(module, proc)` pair at warning level, demoting repeat probes to debug level.
pub(crate) fn warn_proc_not_found_once(api: &str, mod_name: &str, proc_name: &str) {
    static SEEN: std::sync::Mutex<Option<std::collections::HashSet<(String, String)>>> =
        std::sync::Mutex::new(None);
    let first = {
        let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        g.get_or_insert_with(std::collections::HashSet::new)
            .insert((mod_name.to_ascii_lowercase(), proc_name.to_string()))
    };
    if first {
        log::warn!("{api}({mod_name}, {proc_name}) -> NULL (ERROR_PROC_NOT_FOUND)");
    } else {
        log::debug!("{api}({mod_name}, {proc_name}) -> NULL");
    }
}

pub unsafe extern "win64" fn GetProcAddress(h: HMODULE, name: LPCSTR) -> *const c_void {
    if name.is_null() {
        set_err(err::ERROR_PROC_NOT_FOUND);
        return std::ptr::null();
    }
    let owned_name: String;
    let pn = if (name as usize) < 0x10000 {
        ProcName::Ordinal(name as usize as u16)
    } else {
        let bytes = read_ansi_bytes_z(name, DEFAULT_MAX_STR_LEN);
        let Ok(s) = String::from_utf8(bytes) else {
            set_err(err::ERROR_PROC_NOT_FOUND);
            return std::ptr::null();
        };
        owned_name = s;
        ProcName::Name(owned_name.as_str())
    };
    let h = if h.is_null() { crate::loader().get_module_handle(None).unwrap_or(0) } else { h as usize };
    match crate::loader().get_proc_address(h, pn) {
        Some(p) => {
            crate::debug_assert_guest_callable(p, "kernel32::GetProcAddress");
            log::trace!("GetProcAddress(0x{h:x}, {pn}) -> 0x{p:x}");
            p as *const c_void
        }
        None => {
            let mod_name = crate::loader().module_path(Some(h)).unwrap_or_else(|| format!("0x{h:x}"));
            warn_proc_not_found_once("GetProcAddress", &mod_name, &pn.to_string());
            set_err(err::ERROR_PROC_NOT_FOUND);
            std::ptr::null()
        }
    }
}

pub unsafe extern "win64" fn GetModuleFileNameW(h: HMODULE, buf: LPWSTR, size: u32) -> u32 {
    let h = (!h.is_null()).then_some(h as usize);
    let Some(path) = crate::loader().module_path(h) else {
        set_err(err::ERROR_MOD_NOT_FOUND);
        return 0;
    };
    let (trunc, n) = write_wide_trunc(&path, buf, size as usize);
    if trunc {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return size;
    }
    set_err(0);
    n as u32
}

pub unsafe extern "win64" fn GetModuleFileNameA(h: HMODULE, buf: LPSTR, size: u32) -> u32 {
    let h = (!h.is_null()).then_some(h as usize);
    let Some(path) = crate::loader().module_path(h) else {
        set_err(err::ERROR_MOD_NOT_FOUND);
        return 0;
    };
    let (trunc, n) = write_ansi_trunc(&path, buf, size as usize);
    if trunc {
        set_err(err::ERROR_INSUFFICIENT_BUFFER);
        return size;
    }
    set_err(0);
    n as u32
}

pub unsafe extern "win64" fn K32GetModuleFileNameExW(_p: HANDLE, h: HMODULE, buf: LPWSTR, size: u32) -> u32 {
    GetModuleFileNameW(h, buf, size)
}

pub unsafe extern "win64" fn GetModuleFileNameExW(p: HANDLE, h: HMODULE, buf: LPWSTR, size: u32) -> u32 {
    K32GetModuleFileNameExW(p, h, buf, size)
}

pub unsafe extern "win64" fn K32GetModuleFileNameExA(_p: HANDLE, h: HMODULE, buf: LPSTR, size: u32) -> u32 {
    GetModuleFileNameA(h, buf, size)
}

pub unsafe extern "win64" fn GetModuleFileNameExA(p: HANDLE, h: HMODULE, buf: LPSTR, size: u32) -> u32 {
    K32GetModuleFileNameExA(p, h, buf, size)
}

pub unsafe extern "win64" fn K32GetModuleBaseNameW(_p: HANDLE, h: HMODULE, buf: LPWSTR, size: u32) -> u32 {
    let h = (!h.is_null()).then_some(h as usize);
    let Some(path) = crate::loader().module_path(h) else { return 0 };
    let base = path.rsplit('\\').next().unwrap_or(&path).to_string();
    write_wide_trunc(&base, buf, size as usize).1 as u32
}

pub unsafe extern "win64" fn GetModuleBaseNameW(p: HANDLE, h: HMODULE, buf: LPWSTR, size: u32) -> u32 {
    K32GetModuleBaseNameW(p, h, buf, size)
}

pub unsafe extern "win64" fn K32GetModuleBaseNameA(_p: HANDLE, h: HMODULE, buf: LPSTR, size: u32) -> u32 {
    let h = (!h.is_null()).then_some(h as usize);
    let Some(path) = crate::loader().module_path(h) else { return 0 };
    let base = path.rsplit('\\').next().unwrap_or(&path).to_string();
    write_ansi_trunc(&base, buf, size as usize).1 as u32
}

pub unsafe extern "win64" fn GetModuleBaseNameA(p: HANDLE, h: HMODULE, buf: LPSTR, size: u32) -> u32 {
    K32GetModuleBaseNameA(p, h, buf, size)
}

pub unsafe extern "win64" fn K32GetModuleInformation(_p: HANDLE, h: HMODULE, info: *mut usize, _cb: u32) -> BOOL {
    let base = h as usize;
    let size = image_size(base).unwrap_or(0);
    let entry = image_entry(base).unwrap_or(0);
    *info = base;
    *(info.add(1) as *mut u32) = size;
    *info.add(2) = entry;
    TRUE
}

pub unsafe extern "win64" fn GetModuleInformation(p: HANDLE, h: HMODULE, info: *mut usize, cb: u32) -> BOOL {
    K32GetModuleInformation(p, h, info, cb)
}

pub unsafe extern "win64" fn K32EnumProcessModules(_p: HANDLE, out: *mut HMODULE, cb: u32, needed: *mut u32) -> BOOL {
    let mut bases: Vec<usize> = crate::try_loader()
        .map(|l| l.loaded_modules().into_iter().map(|(b, _, _)| b).collect())
        .unwrap_or_default();
    if bases.is_empty() {
        if let Some(main_h) = crate::try_loader().and_then(|l| l.get_module_handle(None)) {
            bases.push(main_h);
        }
    }
    let total_bytes = (bases.len() * std::mem::size_of::<HMODULE>()) as u32;
    if !needed.is_null() {
        std::ptr::write_unaligned(needed, total_bytes);
    }
    if !out.is_null() {
        let cap = (cb as usize) / std::mem::size_of::<HMODULE>();
        for (i, &b) in bases.iter().take(cap).enumerate() {
            std::ptr::write_unaligned(out.add(i), b as HMODULE);
        }
    }
    TRUE
}

pub unsafe extern "win64" fn EnumProcessModules(p: HANDLE, out: *mut HMODULE, cb: u32, needed: *mut u32) -> BOOL {
    K32EnumProcessModules(p, out, cb, needed)
}

pub unsafe extern "win64" fn K32EnumProcessModulesEx(p: HANDLE, out: *mut HMODULE, cb: u32, needed: *mut u32, _filter: u32) -> BOOL {
    K32EnumProcessModules(p, out, cb, needed)
}

pub unsafe extern "win64" fn EnumProcessModulesEx(p: HANDLE, out: *mut HMODULE, cb: u32, needed: *mut u32, filter: u32) -> BOOL {
    K32EnumProcessModulesEx(p, out, cb, needed, filter)
}

pub unsafe extern "win64" fn K32GetMappedFileNameW(p: HANDLE, addr: *mut c_void, buf: LPWSTR, size: u32) -> u32 {
    if buf.is_null() || size == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let mut raw = vec![0u8; 16 + 1024 * 2];
    let mut ret_len: usize = 0;
    let st = crate::ntdll::NtQueryVirtualMemory(p, addr, 2, raw.as_mut_ptr() as *mut c_void, raw.len(), &mut ret_len);
    if st == crate::ntdll::STATUS_BUFFER_OVERFLOW && ret_len > raw.len() {
        raw.resize(ret_len, 0);
        let st2 = crate::ntdll::NtQueryVirtualMemory(p, addr, 2, raw.as_mut_ptr() as *mut c_void, raw.len(), &mut ret_len);
        if st2 != crate::ntdll::STATUS_SUCCESS {
            set_err(crate::ntdll::ntstatus_to_win32(st2 as u32));
            return 0;
        }
    } else if st != crate::ntdll::STATUS_SUCCESS {
        set_err(crate::ntdll::ntstatus_to_win32(st as u32));
        return 0;
    }
    let byte_len = std::ptr::read_unaligned(raw.as_ptr() as *const u16) as usize;
    let str_ptr = std::ptr::read_unaligned(raw.as_ptr().add(8) as *const *const u16);
    let words = byte_len / 2;
    let mut s_u16 = Vec::with_capacity(words);
    for i in 0..words {
        s_u16.push(std::ptr::read_unaligned(str_ptr.add(i)));
    }
    let s = String::from_utf16_lossy(&s_u16);
    let (_, n) = write_wide_trunc(&s, buf, size as usize);
    set_err(0);
    n as u32
}

pub unsafe extern "win64" fn GetMappedFileNameW(p: HANDLE, addr: *mut c_void, buf: LPWSTR, size: u32) -> u32 {
    K32GetMappedFileNameW(p, addr, buf, size)
}

pub unsafe extern "win64" fn K32GetMappedFileNameA(p: HANDLE, addr: *mut c_void, buf: LPSTR, size: u32) -> u32 {
    if buf.is_null() || size == 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return 0;
    }
    let mut tmp = vec![0u16; (size as usize).max(260)];
    let n = K32GetMappedFileNameW(p, addr, tmp.as_mut_ptr(), tmp.len() as u32);
    if n == 0 {
        return 0;
    }
    let s = String::from_utf16_lossy(&tmp[..n as usize]);
    write_ansi_trunc(&s, buf, size as usize).1 as u32
}

pub unsafe extern "win64" fn GetMappedFileNameA(p: HANDLE, addr: *mut c_void, buf: LPSTR, size: u32) -> u32 {
    K32GetMappedFileNameA(p, addr, buf, size)
}

fn set_dll_directory_impl(dir: Option<String>) -> BOOL {
    let mut st = DLL_SEARCH_STATE.write().unwrap();
    match dir {
        None => {
            // NULL resets to default search order (restores CWD, clears legacy dir).
            st.legacy_dll_dir = None;
        }
        Some(s) if s.is_empty() => {
            // Empty string removes CWD from the default DLL search order.
            st.legacy_dll_dir = Some(None);
        }
        Some(s) => {
            let abs = crate::path::absolute(&s);
            st.legacy_dll_dir = Some(Some(abs));
        }
    }
    set_err(err::ERROR_SUCCESS);
    TRUE
}

pub unsafe extern "win64" fn SetDllDirectoryW(p: LPCWSTR) -> BOOL {
    set_dll_directory_impl(read_wide_z_opt(p, DEFAULT_MAX_STR_LEN))
}
pub unsafe extern "win64" fn SetDllDirectoryA(p: LPCSTR) -> BOOL {
    set_dll_directory_impl(read_ansi_z_opt(p, DEFAULT_MAX_STR_LEN))
}
pub unsafe extern "win64" fn SetDefaultDllDirectories(directory_flags: u32) -> BOOL {
    if directory_flags == 0 || (directory_flags & !VALID_DEFAULT_DLL_DIRECTORIES_MASK) != 0 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    DLL_SEARCH_STATE.write().unwrap().default_dirs_mask = Some(directory_flags);
    set_err(err::ERROR_SUCCESS);
    TRUE
}
pub unsafe extern "win64" fn AddDllDirectory(new_directory: LPCWSTR) -> *mut c_void {
    let Some(raw) = read_wide_z_opt(new_directory, DEFAULT_MAX_STR_LEN) else {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    };
    let trimmed = raw.trim();
    if trimmed.is_empty() || !is_absolute_win_path(trimmed) {
        set_err(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let norm = crate::path::normalize(trimmed);
    let cookie = NEXT_DLL_DIR_COOKIE.fetch_add(1, Ordering::Relaxed);
    DLL_SEARCH_STATE.write().unwrap().added_dirs.push((cookie, norm));
    set_err(err::ERROR_SUCCESS);
    cookie as *mut c_void
}
pub unsafe extern "win64" fn RemoveDllDirectory(cookie: *mut c_void) -> BOOL {
    if cookie.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let id = cookie as usize;
    let mut st = DLL_SEARCH_STATE.write().unwrap();
    if let Some(pos) = st.added_dirs.iter().position(|(c, _)| *c == id) {
        st.added_dirs.remove(pos);
        set_err(err::ERROR_SUCCESS);
        TRUE
    } else {
        set_err(err::ERROR_INVALID_PARAMETER);
        FALSE
    }
}

// ---------------------------------------------------------------------------
// PE header helpers on mapped images
// ---------------------------------------------------------------------------

unsafe fn nt_headers(base: usize) -> Option<usize> {
    if base == 0 || *(base as *const u16) != 0x5A4D {
        return None;
    }
    let nt = base + *((base + 0x3C) as *const u32) as usize;
    (*(nt as *const u32) == 0x4550).then_some(nt)
}

pub unsafe fn image_size(base: usize) -> Option<u32> {
    let nt = nt_headers(base)?;
    Some(*((nt + 0x18 + 56) as *const u32))
}

pub unsafe fn image_entry(base: usize) -> Option<usize> {
    let nt = nt_headers(base)?;
    let rva = *((nt + 0x18 + 16) as *const u32);
    Some(if rva == 0 { 0 } else { base + rva as usize })
}

/// (rva, size) of a data directory in a mapped image.
pub unsafe fn data_dir(base: usize, idx: usize) -> Option<(u32, u32)> {
    let nt = nt_headers(base)?;
    let opt = nt + 0x18;
    let count = *((opt + 108) as *const u32) as usize;
    if idx >= count {
        return None;
    }
    let d = opt + 112 + idx * 8;
    let rva = *(d as *const u32);
    let size = *((d + 4) as *const u32);
    (rva != 0).then_some((rva, size))
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

/// A resource identifier: integer (`MAKEINTRESOURCE`) or string.
enum ResId {
    Int(u16),
    Str(String),
}

unsafe fn res_id_w(p: LPCWSTR) -> ResId {
    if (p as usize) < 0x10000 {
        return ResId::Int(p as usize as u16);
    }
    let s = from_wide(p);
    if let Some(n) = s.strip_prefix('#').and_then(|n| n.parse::<u16>().ok()) {
        return ResId::Int(n);
    }
    ResId::Str(s)
}

unsafe fn res_id_a(p: LPCSTR) -> ResId {
    if (p as usize) < 0x10000 {
        return ResId::Int(p as usize as u16);
    }
    let s = from_ansi(p);
    if let Some(n) = s.strip_prefix('#').and_then(|n| n.parse::<u16>().ok()) {
        return ResId::Int(n);
    }
    ResId::Str(s)
}

/// Find a directory entry by id in a resource directory at `dir`.
unsafe fn res_find(root: usize, dir: usize, id: &ResId) -> Option<u32> {
    let named = std::ptr::read_unaligned((dir + 12) as *const u16) as usize;
    let ids = std::ptr::read_unaligned((dir + 14) as *const u16) as usize;
    let entries = dir + 16;
    for i in 0..named + ids {
        let e = entries + i * 8;
        let name = std::ptr::read_unaligned(e as *const u32);
        let off = std::ptr::read_unaligned((e + 4) as *const u32);
        let matches = match id {
            ResId::Int(n) => name & 0x8000_0000 == 0 && name as u16 == *n,
            ResId::Str(s) => {
                if name & 0x8000_0000 == 0 {
                    false
                } else {
                    let p = root + (name & 0x7FFF_FFFF) as usize;
                    let len = std::ptr::read_unaligned(p as *const u16) as usize;
                    let mut w = Vec::with_capacity(len);
                    for k in 0..len {
                        w.push(std::ptr::read_unaligned((p + 2 + k * 2) as *const u16));
                    }
                    String::from_utf16_lossy(&w).eq_ignore_ascii_case(s)
                }
            }
        };
        if matches {
            return Some(off);
        }
    }
    None
}

unsafe fn res_first(dir: usize) -> Option<u32> {
    let n = std::ptr::read_unaligned((dir + 12) as *const u16) as usize
        + std::ptr::read_unaligned((dir + 14) as *const u16) as usize;
    (n > 0).then(|| std::ptr::read_unaligned((dir + 16 + 4) as *const u32))
}

/// Enumerate every resource data entry of integer type `ty` (both named and numeric-ID resources),
/// returning `(data_ptr, byte_len)` pairs.
///
/// When `lang != 0`, restricts lookup strictly to that explicit LANGID and returns
/// `Err(1815)` (`ERROR_RESOURCE_LANG_NOT_FOUND`) if resources of type `ty` exist but none
/// matches `lang`. Language fallback (`0` -> `0x0409` -> first entry) is reserved for `lang == 0`.
pub(crate) unsafe fn find_all_resources_of_type(
    h: HMODULE,
    ty: u16,
    lang: u16,
) -> Result<Vec<(*const u8, usize)>, u32> {
    const ERROR_RESOURCE_LANG_NOT_FOUND: u32 = 1815;
    let base = if h.is_null() {
        crate::try_loader().and_then(|l| l.get_module_handle(None)).unwrap_or(0)
    } else {
        (h as usize) & !3
    };
    if base == 0 {
        return Ok(Vec::new());
    }
    let Some((rva, dir_size)) = data_dir(base, 2) else {
        return Ok(Vec::new());
    };
    let dir_limit = if dir_size == 0 { usize::MAX } else { dir_size as usize };
    let in_dir = |off: usize, len: usize| off.checked_add(len).is_some_and(|end| end <= dir_limit);
    if !in_dir(0, 16) {
        return Ok(Vec::new());
    }
    let root = base + rva as usize;
    let Some(t) = res_find(root, root, &ResId::Int(ty)) else {
        return Ok(Vec::new());
    };
    if (t & 0x8000_0000) == 0 {
        return Ok(Vec::new());
    }
    let t_off = (t & 0x7FFF_FFFF) as usize;
    if !in_dir(t_off, 16) {
        return Ok(Vec::new());
    }
    let tdir = root + t_off;
    let named = std::ptr::read_unaligned((tdir + 12) as *const u16) as usize;
    let ids = std::ptr::read_unaligned((tdir + 14) as *const u16) as usize;
    let total = named.saturating_add(ids);
    let mut out = Vec::new();
    let mut saw_resource = false;
    for i in 0..total {
        let e_off = match t_off.checked_add(16).and_then(|x| x.checked_add(i * 8)) {
            Some(o) if in_dir(o, 8) => o,
            _ => break,
        };
        saw_resource = true;
        let off = std::ptr::read_unaligned((root + e_off + 4) as *const u32);
        let data_off = if (off & 0x8000_0000) == 0 {
            if lang != 0 {
                continue;
            }
            off as usize
        } else {
            let l_off = (off & 0x7FFF_FFFF) as usize;
            if !in_dir(l_off, 16) {
                continue;
            }
            let ldir = root + l_off;
            let picked = if lang != 0 {
                res_find(root, ldir, &ResId::Int(lang))
            } else {
                res_find(root, ldir, &ResId::Int(0))
                    .or_else(|| res_find(root, ldir, &ResId::Int(0x0409)))
                    .or_else(|| res_first(ldir))
            };
            match picked {
                Some(m) if (m & 0x8000_0000) == 0 => m as usize,
                _ => continue,
            }
        };
        if !in_dir(data_off, 8) {
            continue;
        }
        let entry = root + data_off;
        let data_rva = std::ptr::read_unaligned(entry as *const u32) as usize;
        let data_size = std::ptr::read_unaligned((entry + 4) as *const u32) as usize;
        if data_rva == 0 || data_size == 0 {
            continue;
        }
        out.push(((base + data_rva) as *const u8, data_size));
    }
    if out.is_empty() && lang != 0 && saw_resource {
        return Err(ERROR_RESOURCE_LANG_NOT_FOUND);
    }
    Ok(out)
}

unsafe fn find_resource(h: HMODULE, ty: ResId, name: ResId, lang: u16) -> *mut c_void {
    // Strip low 2 datafile/image-resource tag bits (`LDR_IS_DATAFILE` / `LDR_IS_IMAGEMAPPING`).
    let base = if h.is_null() {
        crate::loader().get_module_handle(None).unwrap_or(0)
    } else {
        (h as usize) & !3
    };
    let Some((rva, _)) = data_dir(base, 2) else {
        set_err(err::ERROR_RESOURCE_DATA_NOT_FOUND);
        return std::ptr::null_mut();
    };
    let root = base + rva as usize;
    let Some(t) = res_find(root, root, &ty) else {
        set_err(err::ERROR_RESOURCE_TYPE_NOT_FOUND);
        return std::ptr::null_mut();
    };
    let tdir = root + (t & 0x7FFF_FFFF) as usize;
    let Some(n) = res_find(root, tdir, &name) else {
        set_err(err::ERROR_RESOURCE_NAME_NOT_FOUND);
        return std::ptr::null_mut();
    };
    let ldir = root + (n & 0x7FFF_FFFF) as usize;
    let l = res_find(root, ldir, &ResId::Int(lang))
        .or_else(|| res_find(root, ldir, &ResId::Int(0x409)))
        .or_else(|| res_find(root, ldir, &ResId::Int(0)))
        .or_else(|| res_first(ldir));
    match l {
        Some(off) if off & 0x8000_0000 == 0 => (root + off as usize) as *mut c_void,
        _ => {
            set_err(1815); // ERROR_RESOURCE_LANG_NOT_FOUND
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn FindResourceW(h: HMODULE, name: LPCWSTR, ty: LPCWSTR) -> *mut c_void {
    find_resource(h, res_id_w(ty), res_id_w(name), 0)
}
pub unsafe extern "win64" fn FindResourceA(h: HMODULE, name: LPCSTR, ty: LPCSTR) -> *mut c_void {
    find_resource(h, res_id_a(ty), res_id_a(name), 0)
}
pub unsafe extern "win64" fn FindResourceExW(h: HMODULE, ty: LPCWSTR, name: LPCWSTR, lang: u16) -> *mut c_void {
    find_resource(h, res_id_w(ty), res_id_w(name), lang)
}
pub unsafe extern "win64" fn FindResourceExA(h: HMODULE, ty: LPCSTR, name: LPCSTR, lang: u16) -> *mut c_void {
    find_resource(h, res_id_a(ty), res_id_a(name), lang)
}

pub unsafe extern "win64" fn LoadResource(h: HMODULE, res: *mut c_void) -> *mut c_void {
    if res.is_null() {
        return std::ptr::null_mut();
    }
    let base = if h.is_null() {
        crate::loader().get_module_handle(None).unwrap_or(0)
    } else {
        (h as usize) & !3
    };
    (base + std::ptr::read_unaligned(res as *const u32) as usize) as *mut c_void
}

pub unsafe extern "win64" fn LockResource(h: *mut c_void) -> *mut c_void {
    h
}

pub unsafe extern "win64" fn FreeResource(_h: *mut c_void) -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn SizeofResource(_h: HMODULE, res: *mut c_void) -> u32 {
    if res.is_null() {
        return 0;
    }
    *((res as usize + 4) as *const u32)
}

// ---------------------------------------------------------------------------
// Activation Contexts (CreateActCtx / ActivateActCtx / DeactivateActCtx)
// ---------------------------------------------------------------------------

struct ActCtx {
    refs: std::sync::atomic::AtomicU32,
}

static NEXT_ACTCTX_COOKIE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0x100);

thread_local! {
    static ACTCTX_STACK: std::cell::RefCell<Vec<(usize, HANDLE)>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub unsafe extern "win64" fn CreateActCtxW(actctx: *const c_void) -> HANDLE {
    if actctx.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return INVALID_HANDLE_VALUE;
    }
    let cb_size = std::ptr::read_unaligned(actctx as *const u32);
    if cb_size < 8 {
        set_err(err::ERROR_INVALID_PARAMETER);
        return INVALID_HANDLE_VALUE;
    }
    let boxed = Box::new(ActCtx { refs: std::sync::atomic::AtomicU32::new(1) });
    set_err(0);
    Box::into_raw(boxed) as HANDLE
}

pub unsafe extern "win64" fn CreateActCtxA(actctx: *const c_void) -> HANDLE {
    CreateActCtxW(actctx)
}

pub unsafe extern "win64" fn AddRefActCtx(h: HANDLE) {
    if !h.is_null() && h != INVALID_HANDLE_VALUE {
        let ctx = &*(h as *const ActCtx);
        ctx.refs.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

pub unsafe extern "win64" fn ReleaseActCtx(h: HANDLE) {
    if !h.is_null() && h != INVALID_HANDLE_VALUE {
        let ctx = &*(h as *const ActCtx);
        if ctx.refs.fetch_sub(1, std::sync::atomic::Ordering::AcqRel) == 1 {
            drop(Box::from_raw(h as *mut ActCtx));
        }
    }
}

pub unsafe extern "win64" fn ActivateActCtx(h: HANDLE, cookie_out: *mut usize) -> BOOL {
    if cookie_out.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    if h == INVALID_HANDLE_VALUE {
        set_err(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    let cookie = NEXT_ACTCTX_COOKIE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    ACTCTX_STACK.with(|s| s.borrow_mut().push((cookie, h)));
    std::ptr::write_unaligned(cookie_out, cookie);
    set_err(0);
    TRUE
}

pub unsafe extern "win64" fn DeactivateActCtx(flags: u32, cookie: usize) -> BOOL {
    const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1;
    let ok = ACTCTX_STACK.with(|s| {
        let mut st = s.borrow_mut();
        if let Some(pos) = st.iter().rposition(|&(c, _)| c == cookie) {
            if pos + 1 == st.len() || (flags & DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION) != 0 {
                st.truncate(pos);
                return true;
            }
        }
        false
    });
    if ok {
        set_err(0);
        TRUE
    } else {
        set_err(err::ERROR_INVALID_PARAMETER);
        FALSE
    }
}

pub unsafe extern "win64" fn GetCurrentActCtx(out: *mut HANDLE) -> BOOL {
    if out.is_null() {
        set_err(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let h = ACTCTX_STACK.with(|s| s.borrow().last().map(|&(_, h)| h).unwrap_or(std::ptr::null_mut()));
    if !h.is_null() {
        AddRefActCtx(h);
    }
    std::ptr::write_unaligned(out, h);
    set_err(0);
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actctx_lifecycle_roundtrip() {
        unsafe {
            let mut desc = [0u8; 56];
            let desc_len = desc.len() as u32;
            desc[0..4].copy_from_slice(&desc_len.to_le_bytes());
            let h = CreateActCtxW(desc.as_ptr() as *const c_void);
            assert!(!h.is_null() && h != INVALID_HANDLE_VALUE);

            let mut cookie = 0usize;
            assert_eq!(ActivateActCtx(h, &mut cookie), TRUE);
            assert_ne!(cookie, 0);

            let mut cur: HANDLE = std::ptr::null_mut();
            assert_eq!(GetCurrentActCtx(&mut cur), TRUE);
            assert_eq!(cur, h);
            ReleaseActCtx(cur);

            assert_eq!(DeactivateActCtx(0, cookie), TRUE);
            assert_eq!(GetCurrentActCtx(&mut cur), TRUE);
            assert!(cur.is_null());

            ReleaseActCtx(h);
        }
    }

    #[test]
    fn dll_directory_search_state_validation_and_lifecycle() {
        reset_dll_search_state();
        unsafe {
            // 1. SetDefaultDllDirectories rejects 0 and invalid flags (including DLL_LOAD_DIR).
            assert_eq!(SetDefaultDllDirectories(0), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            assert_eq!(SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            assert_eq!(SetDefaultDllDirectories(0x8000_0000), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Valid combinations succeed.
            assert_eq!(
                SetDefaultDllDirectories(
                    LOAD_LIBRARY_SEARCH_APPLICATION_DIR | LOAD_LIBRARY_SEARCH_USER_DIRS
                ),
                TRUE
            );
            assert_eq!(
                dll_search_config().default_dirs_mask,
                Some(LOAD_LIBRARY_SEARCH_APPLICATION_DIR | LOAD_LIBRARY_SEARCH_USER_DIRS)
            );

            assert_eq!(SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS), TRUE);
            assert_eq!(
                dll_search_config().default_dirs_mask,
                Some(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS)
            );

            // 2. AddDllDirectory rejects NULL, empty, whitespace, and relative paths.
            assert!(AddDllDirectory(std::ptr::null()).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            let empty = to_wide_z("");
            assert!(AddDllDirectory(empty.as_ptr()).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            let rel = to_wide_z("plugins\\sub");
            assert!(AddDllDirectory(rel.as_ptr()).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Valid absolute paths return unique non-null cookies and normalize separators.
            let dir1 = to_wide_z("C:/Game/plugins/one");
            let dir2 = to_wide_z("C:\\Game\\plugins\\two");
            let c1 = AddDllDirectory(dir1.as_ptr());
            let c2 = AddDllDirectory(dir2.as_ptr());
            assert!(!c1.is_null() && !c2.is_null() && c1 != c2);
            assert_eq!(
                dll_search_config().added_dirs,
                vec!["C:\\Game\\plugins\\one", "C:\\Game\\plugins\\two"]
            );

            // RemoveDllDirectory removes valid cookies and rejects invalid/already-removed cookies.
            assert_eq!(RemoveDllDirectory(std::ptr::null_mut()), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            assert_eq!(RemoveDllDirectory(0xDEADusize as *mut c_void), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            assert_eq!(RemoveDllDirectory(c1), TRUE);
            assert_eq!(dll_search_config().added_dirs, vec!["C:\\Game\\plugins\\two"]);

            // Double removal fails with ERROR_INVALID_PARAMETER.
            assert_eq!(RemoveDllDirectory(c1), FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            assert_eq!(RemoveDllDirectory(c2), TRUE);
            assert!(dll_search_config().added_dirs.is_empty());

            // 3. SetDllDirectoryW/A: set directory, empty removes CWD, NULL restores default.
            let leg = to_wide_z("C:\\Legacy\\Bin");
            assert_eq!(SetDllDirectoryW(leg.as_ptr()), TRUE);
            assert_eq!(
                dll_search_config().legacy_dll_dir,
                Some(Some("C:\\Legacy\\Bin".to_string()))
            );

            assert_eq!(SetDllDirectoryW(empty.as_ptr()), TRUE);
            assert_eq!(dll_search_config().legacy_dll_dir, Some(None));

            assert_eq!(SetDllDirectoryW(std::ptr::null()), TRUE);
            assert_eq!(dll_search_config().legacy_dll_dir, None);

            let leg_a = b"C:\\LegacyA\\Bin\0";
            assert_eq!(SetDllDirectoryA(leg_a.as_ptr()), TRUE);
            assert_eq!(
                dll_search_config().legacy_dll_dir,
                Some(Some("C:\\LegacyA\\Bin".to_string()))
            );
            assert_eq!(SetDllDirectoryA(std::ptr::null()), TRUE);
            assert_eq!(dll_search_config().legacy_dll_dir, None);
        }
        reset_dll_search_state();
    }

    #[test]
    fn load_library_flag_validation_and_ldr_bounded_strings() {
        use std::sync::{Mutex, OnceLock};

        struct RecordedCall {
            name: String,
            flags: u32,
            search_path: Option<String>,
        }
        struct MockLoader {
            calls: Mutex<Vec<RecordedCall>>,
        }
        impl crate::LoaderOps for MockLoader {
            fn load_library(&self, name: &str, flags: u32) -> Result<usize, u32> {
                self.load_library_with_search_path(name, flags, None)
            }
            fn load_library_with_search_path(
                &self,
                name: &str,
                flags: u32,
                search_path: Option<&str>,
            ) -> Result<usize, u32> {
                self.calls.lock().unwrap().push(RecordedCall {
                    name: name.to_string(),
                    flags,
                    search_path: search_path.map(str::to_string),
                });
                if name.eq_ignore_ascii_case("mock_plugin.dll") {
                    Ok(0x1000_0000)
                } else {
                    Err(err::ERROR_MOD_NOT_FOUND)
                }
            }
            fn free_library(&self, hmod: usize) -> bool {
                hmod == 0x1000_0000
            }
            fn get_module_handle(&self, name: Option<&str>) -> Option<usize> {
                match name {
                    // Never return an unmapped fake pointer for `None`: other unit tests in the
                    // same process may call `FindResourceW(NULL, ...)` against this OnceLock loader.
                    None => None,
                    Some(n) if n.eq_ignore_ascii_case("mock_plugin.dll") => Some(0x1000_0000),
                    _ => None,
                }
            }
            fn get_proc_address(&self, hmod: usize, name: ProcName<'_>) -> Option<usize> {
                if hmod != 0x1000_0000 {
                    return None;
                }
                match name {
                    ProcName::Name("ExactSym") => Some(0x1000_1111),
                    ProcName::Name("DataSym") => Some(0x1000_2222),
                    ProcName::Ordinal(42) => Some(0x1000_3333),
                    _ => None,
                }
            }
            fn module_path(&self, hmod: Option<usize>) -> Option<String> {
                match hmod {
                    Some(0x1000_0000) => Some("C:\\Game\\mock_plugin.dll".into()),
                    _ => None,
                }
            }
            fn module_from_address(&self, _addr: usize) -> Option<usize> {
                None
            }
            fn thread_attach(&self, _teb: *mut u8, _is_initial: bool) {}
            fn thread_detach(&self, _teb: *mut u8) {}
            fn lookup_function_entry(
                &self,
                _pc: u64,
            ) -> Option<(u64, *const pe_image::RuntimeFunction)> {
                None
            }
            fn disable_thread_library_calls(&self, _hmod: usize) -> bool {
                true
            }
            fn symbolize(&self, addr: usize) -> String {
                format!("0x{addr:x}")
            }
        }

        static MOCK: OnceLock<&'static MockLoader> = OnceLock::new();
        let mock = *MOCK.get_or_init(|| {
            let leaked: &'static MockLoader = Box::leak(Box::new(MockLoader {
                calls: Mutex::new(Vec::new()),
            }));
            if crate::try_loader().is_none() {
                crate::set_loader(leaked);
            }
            leaked
        });

        unsafe {
            // Invalid flag combination: LOAD_WITH_ALTERED_SEARCH_PATH + LOAD_LIBRARY_SEARCH_*
            let name_w = to_wide_z("C:\\Game\\mock_plugin.dll");
            let h_bad = LoadLibraryExW(
                name_w.as_ptr(),
                std::ptr::null_mut(),
                LOAD_WITH_ALTERED_SEARCH_PATH | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
            );
            assert!(h_bad.is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Invalid flag combination: LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR with non-absolute path
            let rel_w = to_wide_z("plugins\\mock_plugin.dll");
            let h_rel = LoadLibraryExW(
                rel_w.as_ptr(),
                std::ptr::null_mut(),
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
            );
            assert!(h_rel.is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Non-null hFile fails with ERROR_INVALID_PARAMETER.
            let h_file = LoadLibraryExW(name_w.as_ptr(), 1usize as HANDLE, 0);
            assert!(h_file.is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // Null name fails with ERROR_INVALID_PARAMETER.
            assert!(LoadLibraryW(std::ptr::null()).is_null());
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // LdrGetProcedureAddress honors ANSI_STRING.Length (non-NUL-terminated slice!).
            let non_nul_buf = b"ExactSymTRAILING_GARBAGE";
            #[repr(C)]
            struct AnsiString {
                len: u16,
                max: u16,
                pad: u32,
                buf: *const u8,
            }
            let ansi = AnsiString {
                len: 8, // "ExactSym" only!
                max: non_nul_buf.len() as u16,
                pad: 0,
                buf: non_nul_buf.as_ptr(),
            };
            let mut out_fn: *const c_void = std::ptr::null();
            let st = crate::ntdll::LdrGetProcedureAddress(
                0x1000_0000 as HMODULE,
                &ansi as *const AnsiString as *const u8,
                0,
                &mut out_fn,
            );
            assert_eq!(st, crate::ntdll::STATUS_SUCCESS);
            assert_eq!(out_fn as usize, 0x1000_1111);

            // LdrGetProcedureAddress by ordinal
            out_fn = std::ptr::null();
            let st_ord = crate::ntdll::LdrGetProcedureAddress(
                0x1000_0000 as HMODULE,
                std::ptr::null(),
                42,
                &mut out_fn,
            );
            assert_eq!(st_ord, crate::ntdll::STATUS_SUCCESS);
            assert_eq!(out_fn as usize, 0x1000_3333);

            // Missing symbol -> STATUS_ENTRYPOINT_NOT_FOUND
            let st_miss = crate::ntdll::LdrGetProcedureAddress(
                0x1000_0000 as HMODULE,
                std::ptr::null(),
                99,
                &mut out_fn,
            );
            assert_eq!(st_miss, 0xC000_0139u32 as i32);
            assert!(out_fn.is_null());

            // LdrGetDllHandle honors UNICODE_STRING.len (non-NUL-terminated slice!).
            let non_nul_w: Vec<u16> = "mock_plugin.dllGARBAGE".encode_utf16().collect();
            let us_mod = crate::ntdll::UNICODE_STRING {
                len: ("mock_plugin.dll".len() * 2) as u16,
                max: (non_nul_w.len() * 2) as u16,
                buf: non_nul_w.as_ptr() as *mut u16,
            };
            let mut got_h: HMODULE = std::ptr::null_mut();
            assert_eq!(
                crate::ntdll::LdrGetDllHandle(
                    std::ptr::null(),
                    std::ptr::null(),
                    &us_mod,
                    &mut got_h
                ),
                crate::ntdll::STATUS_SUCCESS
            );
            assert_eq!(got_h as usize, 0x1000_0000);

            // LdrLoadDll forwards PathToFile search hint and bit-0 search flags pointer.
            let search_hint = to_wide_z("C:\\CustomSearch;C:\\AltSearch");
            let mut ldr_h: HMODULE = std::ptr::null_mut();
            let fl = LOAD_LIBRARY_SEARCH_USER_DIRS;
            assert_eq!(
                crate::ntdll::LdrLoadDll(search_hint.as_ptr(), &fl, &us_mod, &mut ldr_h),
                crate::ntdll::STATUS_SUCCESS
            );
            assert_eq!(ldr_h as usize, 0x1000_0000);

            let bit0_flags: u32 = LOAD_LIBRARY_SEARCH_SYSTEM32;
            let bit0_ptr = ((&bit0_flags as *const u32 as usize) | 1) as LPCWSTR;
            assert_eq!(
                crate::ntdll::LdrLoadDll(bit0_ptr, std::ptr::null(), &us_mod, &mut ldr_h),
                crate::ntdll::STATUS_SUCCESS
            );
            let calls = mock.calls.lock().unwrap();
            assert!(calls.len() >= 2);
            let c_hint = &calls[calls.len() - 2];
            assert_eq!(c_hint.name, "mock_plugin.dll");
            assert_eq!(c_hint.flags, LOAD_LIBRARY_SEARCH_USER_DIRS);
            assert_eq!(
                c_hint.search_path.as_deref(),
                Some("C:\\CustomSearch;C:\\AltSearch")
            );
            let c_bit0 = &calls[calls.len() - 1];
            assert_eq!(c_bit0.flags, LOAD_LIBRARY_SEARCH_SYSTEM32);
            assert_eq!(c_bit0.search_path, None);

            // K32GetModuleFileNameExA / GetModuleFileNameExA return ANSI module path.
            let mut abuf = [0u8; 64];
            let n = K32GetModuleFileNameExA(
                crate::handles::CURRENT_PROCESS,
                0x1000_0000 as HMODULE,
                abuf.as_mut_ptr(),
                abuf.len() as u32,
            );
            assert_eq!(n as usize, "C:\\Game\\mock_plugin.dll".len());
            assert_eq!(from_ansi(abuf.as_ptr()), "C:\\Game\\mock_plugin.dll");
            assert_eq!(
                GetModuleFileNameExA(
                    crate::handles::CURRENT_PROCESS,
                    0xDEAD_BEEF as HMODULE,
                    abuf.as_mut_ptr(),
                    abuf.len() as u32,
                ),
                0
            );
            assert_eq!(crate::teb::last_error(), err::ERROR_MOD_NOT_FOUND);
        }
    }
}
