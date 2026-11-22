//! Modules: LoadLibrary / GetModuleHandle / GetProcAddress / resources.

use crate::strings::*;
use crate::types::*;
use crate::ProcName;
use std::ffi::c_void;

fn set_err(e: u32) {
    crate::teb::set_last_error(e);
}

pub unsafe fn load_library(name: &str, flags: u32) -> HMODULE {
    match crate::loader().load_library(name, flags) {
        Ok(h) => h as HMODULE,
        Err(e) => {
            log::warn!("LoadLibrary({name:?}, 0x{flags:x}) failed: error {e}");
            set_err(e);
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn LoadLibraryA(name: LPCSTR) -> HMODULE {
    load_library(&from_ansi(name), 0)
}
pub unsafe extern "win64" fn LoadLibraryW(name: LPCWSTR) -> HMODULE {
    load_library(&from_wide(name), 0)
}
pub unsafe extern "win64" fn LoadLibraryExA(name: LPCSTR, _file: HANDLE, flags: u32) -> HMODULE {
    load_library(&from_ansi(name), flags)
}
pub unsafe extern "win64" fn LoadLibraryExW(name: LPCWSTR, _file: HANDLE, flags: u32) -> HMODULE {
    load_library(&from_wide(name), flags)
}
pub unsafe extern "win64" fn LoadPackagedLibrary(name: LPCWSTR, _r: u32) -> HMODULE {
    load_library(&from_wide(name), 0)
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

pub unsafe extern "win64" fn GetProcAddress(h: HMODULE, name: LPCSTR) -> *const c_void {
    let pn = if (name as usize) < 0x10000 {
        ProcName::Ordinal(name as usize as u16)
    } else {
        ProcName::Name(std::str::from_utf8_unchecked(cstr_slice(name)))
    };
    let h = if h.is_null() { crate::loader().get_module_handle(None).unwrap_or(0) } else { h as usize };
    match crate::loader().get_proc_address(h, pn) {
        Some(p) => {
            log::trace!("GetProcAddress(0x{h:x}, {pn}) -> 0x{p:x}");
            p as *const c_void
        }
        None => {
            let mod_name = crate::loader().module_path(Some(h)).unwrap_or_else(|| format!("0x{h:x}"));
            log::debug!("GetProcAddress({mod_name}, {pn}) -> NULL");
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

pub unsafe extern "win64" fn SetDllDirectoryW(_p: LPCWSTR) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetDllDirectoryA(_p: LPCSTR) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn SetDefaultDllDirectories(_f: u32) -> BOOL {
    TRUE
}
pub unsafe extern "win64" fn AddDllDirectory(_p: LPCWSTR) -> *mut c_void {
    1 as *mut c_void
}
pub unsafe extern "win64" fn RemoveDllDirectory(_c: *mut c_void) -> BOOL {
    TRUE
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
    let named = *((dir + 12) as *const u16) as usize;
    let ids = *((dir + 14) as *const u16) as usize;
    let entries = dir + 16;
    for i in 0..named + ids {
        let e = entries + i * 8;
        let name = *(e as *const u32);
        let off = *((e + 4) as *const u32);
        let matches = match id {
            ResId::Int(n) => name & 0x8000_0000 == 0 && name as u16 == *n,
            ResId::Str(s) => {
                if name & 0x8000_0000 == 0 {
                    false
                } else {
                    let p = root + (name & 0x7FFF_FFFF) as usize;
                    let len = *(p as *const u16) as usize;
                    let w = std::slice::from_raw_parts((p + 2) as *const u16, len);
                    String::from_utf16_lossy(w).eq_ignore_ascii_case(s)
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
    let n = *((dir + 12) as *const u16) as usize + *((dir + 14) as *const u16) as usize;
    (n > 0).then(|| *((dir + 16 + 4) as *const u32))
}

unsafe fn find_resource(h: HMODULE, ty: ResId, name: ResId, lang: u16) -> *mut c_void {
    let base = if h.is_null() { crate::loader().get_module_handle(None).unwrap_or(0) } else { h as usize };
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
    let base = if h.is_null() { crate::loader().get_module_handle(None).unwrap_or(0) } else { h as usize };
    (base + *(res as *const u32) as usize) as *mut c_void
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
}
