//! dbghelp.dll: crash-reporting helpers.
//!
//! The function-table / module-base callbacks are real (backed by the loader
//! module list and `.pdata`). Symbol lookup, StackWalk64 and minidumps are not
//! implemented and fail with a STUB log; crashes are reported by our own
//! signal handler instead.

use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;
const ERROR_MOD_NOT_FOUND: u32 = 126;

static OPTIONS: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Debug)]
struct SymModule {
    base: u64,
    size: u32,
    module_name: String,
    image_name: String,
}

#[derive(Default)]
struct SymState {
    search_path: String,
    modules: Vec<SymModule>,
}

fn sym_table() -> &'static Mutex<HashMap<usize, SymState>> {
    static T: OnceLock<Mutex<HashMap<usize, SymState>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

fn proc_key(process: HANDLE) -> usize {
    process as usize
}

/// Returns the new option mask (as Windows does).
pub unsafe extern "win64" fn SymSetOptions(opts: u32) -> u32 {
    OPTIONS.store(opts, Ordering::Relaxed);
    opts
}

pub unsafe extern "win64" fn SymGetOptions() -> u32 {
    OPTIONS.load(Ordering::Relaxed)
}

pub unsafe extern "win64" fn SymFunctionTableAccess64(_process: HANDLE, addr: u64) -> *mut c_void {
    match crate::unwind::lookup_function_entry(addr) {
        Some((_, rf)) => rf as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

pub unsafe extern "win64" fn SymGetModuleBase64(process: HANDLE, addr: u64) -> u64 {
    if let Some(b) = crate::try_loader().and_then(|l| l.module_from_address(addr as usize)) {
        return b as u64;
    }
    let tbl = sym_table().lock().unwrap();
    if let Some(st) = tbl.get(&proc_key(process)) {
        for m in &st.modules {
            let end = m.base.saturating_add(m.size.max(1) as u64);
            if addr >= m.base && addr < end {
                return m.base;
            }
        }
    }
    0
}

fn init_sym_state(process: HANDLE, search_path: Option<String>, invade: bool) -> BOOL {
    let mut st = SymState {
        search_path: search_path.unwrap_or_else(|| ".".to_string()),
        modules: Vec::new(),
    };
    if invade {
        if let Some(loader) = crate::try_loader() {
            for (base, size, path) in loader.loaded_modules() {
                let mod_name = path
                    .rsplit(['\\', '/'])
                    .next()
                    .unwrap_or(&path)
                    .trim_end_matches(".dll")
                    .trim_end_matches(".exe")
                    .to_string();
                st.modules.push(SymModule {
                    base: base as u64,
                    size,
                    module_name: mod_name,
                    image_name: path,
                });
            }
        }
    }
    sym_table().lock().unwrap().insert(proc_key(process), st);
    TRUE
}

pub unsafe extern "win64" fn SymInitialize(
    process: HANDLE,
    user_search_path: LPCSTR,
    f_invade_process: BOOL,
) -> BOOL {
    init_sym_state(process, crate::strings::from_ansi_opt(user_search_path), f_invade_process != 0)
}

pub unsafe extern "win64" fn SymInitializeW(
    process: HANDLE,
    user_search_path: LPCWSTR,
    f_invade_process: BOOL,
) -> BOOL {
    init_sym_state(process, crate::strings::from_wide_opt(user_search_path), f_invade_process != 0)
}

pub unsafe extern "win64" fn SymCleanup(process: HANDLE) -> BOOL {
    sym_table().lock().unwrap().remove(&proc_key(process));
    TRUE
}

pub unsafe extern "win64" fn SymGetSearchPath(
    process: HANDLE,
    search_path: LPSTR,
    search_path_len: u32,
) -> BOOL {
    if search_path.is_null() || search_path_len == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let path = sym_table()
        .lock()
        .unwrap()
        .get(&proc_key(process))
        .map(|s| s.search_path.clone())
        .unwrap_or_else(|| ".".to_string());
    let bytes = path.as_bytes();
    let copy_n = bytes.len().min((search_path_len - 1) as usize);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), search_path, copy_n);
    std::ptr::write_unaligned(search_path.add(copy_n), 0);
    TRUE
}

pub unsafe extern "win64" fn SymGetSearchPathW(
    process: HANDLE,
    search_path: LPWSTR,
    search_path_len: u32,
) -> BOOL {
    if search_path.is_null() || search_path_len == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let path = sym_table()
        .lock()
        .unwrap()
        .get(&proc_key(process))
        .map(|s| s.search_path.clone())
        .unwrap_or_else(|| ".".to_string());
    crate::strings::write_wide_trunc(&path, search_path, search_path_len as usize);
    TRUE
}

pub unsafe extern "win64" fn SymSetSearchPath(process: HANDLE, search_path: LPCSTR) -> BOOL {
    let s = crate::strings::from_ansi_opt(search_path).unwrap_or_default();
    let mut tbl = sym_table().lock().unwrap();
    tbl.entry(proc_key(process)).or_default().search_path = s;
    TRUE
}

pub unsafe extern "win64" fn SymSetSearchPathW(process: HANDLE, search_path: LPCWSTR) -> BOOL {
    let s = crate::strings::from_wide_opt(search_path).unwrap_or_default();
    let mut tbl = sym_table().lock().unwrap();
    tbl.entry(proc_key(process)).or_default().search_path = s;
    TRUE
}

pub unsafe extern "win64" fn SymLoadModule64(
    process: HANDLE,
    _file: HANDLE,
    image_name: LPCSTR,
    module_name: LPCSTR,
    base_of_dll: u64,
    size_of_dll: u32,
) -> u64 {
    let img = crate::strings::from_ansi_opt(image_name).unwrap_or_default();
    let mname = crate::strings::from_ansi_opt(module_name).unwrap_or_else(|| {
        img.rsplit(['\\', '/']).next().unwrap_or(&img).to_string()
    });
    let base = if base_of_dll != 0 {
        base_of_dll
    } else {
        crate::try_loader()
            .and_then(|l| l.get_module_handle(if img.is_empty() { None } else { Some(&img) }))
            .unwrap_or(0) as u64
    };
    if base == 0 {
        crate::teb::set_last_error(ERROR_MOD_NOT_FOUND);
        return 0;
    }
    let mut tbl = sym_table().lock().unwrap();
    let st = tbl.entry(proc_key(process)).or_default();
    st.modules.retain(|m| m.base != base);
    st.modules.push(SymModule {
        base,
        size: size_of_dll,
        module_name: mname,
        image_name: img,
    });
    base
}

fn find_sym_module(process: HANDLE, addr: u64) -> Option<SymModule> {
    if let Some(loader) = crate::try_loader() {
        for (base, size, path) in loader.loaded_modules() {
            let b = base as u64;
            let end = b.saturating_add(size as u64);
            if addr >= b && addr < end {
                let mod_name = path
                    .rsplit(['\\', '/'])
                    .next()
                    .unwrap_or(&path)
                    .to_string();
                return Some(SymModule {
                    base: b,
                    size,
                    module_name: mod_name,
                    image_name: path,
                });
            }
        }
    }
    let tbl = sym_table().lock().unwrap();
    tbl.get(&proc_key(process)).and_then(|st| {
        st.modules
            .iter()
            .find(|m| addr >= m.base && addr < m.base.saturating_add(m.size.max(1) as u64))
            .cloned()
    })
}

pub unsafe extern "win64" fn SymGetModuleInfo64(
    process: HANDLE,
    qw_addr: u64,
    module_info: *mut u8,
) -> BOOL {
    if module_info.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(m) = find_sym_module(process, qw_addr) else {
        crate::teb::set_last_error(ERROR_MOD_NOT_FOUND);
        return FALSE;
    };
    let cb = std::ptr::read_unaligned(module_info as *const u32) as usize;
    if cb < 584 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    std::ptr::write_bytes(module_info.add(4), 0, cb - 4);
    std::ptr::write_unaligned(module_info.add(8) as *mut u64, m.base);
    std::ptr::write_unaligned(module_info.add(16) as *mut u32, m.size);
    std::ptr::write_unaligned(module_info.add(32) as *mut u32, 3); // SymExport
    let copy_ascii = |dst: *mut u8, cap: usize, s: &str| {
        let n = s.len().min(cap - 1);
        std::ptr::copy_nonoverlapping(s.as_bytes().as_ptr(), dst, n);
        std::ptr::write_unaligned(dst.add(n), 0);
    };
    copy_ascii(module_info.add(36), 32, &m.module_name);
    copy_ascii(module_info.add(68), 256, &m.image_name);
    copy_ascii(module_info.add(324), 256, &m.image_name);
    TRUE
}

pub unsafe extern "win64" fn SymGetModuleInfoW64(
    process: HANDLE,
    qw_addr: u64,
    module_info: *mut u8,
) -> BOOL {
    if module_info.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(m) = find_sym_module(process, qw_addr) else {
        crate::teb::set_last_error(ERROR_MOD_NOT_FOUND);
        return FALSE;
    };
    let cb = std::ptr::read_unaligned(module_info as *const u32) as usize;
    if cb < 1124 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    std::ptr::write_bytes(module_info.add(4), 0, cb - 4);
    std::ptr::write_unaligned(module_info.add(8) as *mut u64, m.base);
    std::ptr::write_unaligned(module_info.add(16) as *mut u32, m.size);
    std::ptr::write_unaligned(module_info.add(32) as *mut u32, 3); // SymExport
    crate::strings::write_wide_trunc(&m.module_name, module_info.add(36) as *mut u16, 32);
    crate::strings::write_wide_trunc(&m.image_name, module_info.add(100) as *mut u16, 256);
    crate::strings::write_wide_trunc(&m.image_name, module_info.add(612) as *mut u16, 256);
    TRUE
}

fn resolve_symbol_name(process: HANDLE, addr: u64) -> (String, u64, u64) {
    let mod_base = unsafe { SymGetModuleBase64(process, addr) };
    let sym = crate::symbolize(addr as usize);
    let disp = if mod_base != 0 && addr >= mod_base {
        addr - mod_base
    } else {
        0
    };
    (sym, mod_base, disp)
}

pub unsafe extern "win64" fn SymFromAddr(
    process: HANDLE,
    addr: u64,
    displacement: *mut u64,
    symbol: *mut u8,
) -> BOOL {
    if symbol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let (name, mod_base, disp) = resolve_symbol_name(process, addr);
    if !displacement.is_null() {
        std::ptr::write_unaligned(displacement, disp);
    }
    let max_name = std::ptr::read_unaligned(symbol.add(80) as *const u32) as usize;
    std::ptr::write_unaligned(symbol.add(32) as *mut u64, mod_base);
    std::ptr::write_unaligned(symbol.add(56) as *mut u64, addr);
    let copy_n = if max_name > 0 {
        name.len().min(max_name - 1)
    } else {
        0
    };
    std::ptr::write_unaligned(symbol.add(76) as *mut u32, copy_n as u32);
    if max_name > 0 {
        std::ptr::copy_nonoverlapping(name.as_bytes().as_ptr(), symbol.add(84), copy_n);
        std::ptr::write_unaligned(symbol.add(84 + copy_n), 0);
    }
    TRUE
}

pub unsafe extern "win64" fn SymFromAddrW(
    process: HANDLE,
    addr: u64,
    displacement: *mut u64,
    symbol: *mut u8,
) -> BOOL {
    if symbol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let (name, mod_base, disp) = resolve_symbol_name(process, addr);
    if !displacement.is_null() {
        std::ptr::write_unaligned(displacement, disp);
    }
    let max_name = std::ptr::read_unaligned(symbol.add(80) as *const u32) as usize;
    std::ptr::write_unaligned(symbol.add(32) as *mut u64, mod_base);
    std::ptr::write_unaligned(symbol.add(56) as *mut u64, addr);
    let units: Vec<u16> = name.encode_utf16().collect();
    let copy_n = if max_name > 0 {
        units.len().min(max_name - 1)
    } else {
        0
    };
    std::ptr::write_unaligned(symbol.add(76) as *mut u32, copy_n as u32);
    if max_name > 0 {
        let dst = symbol.add(84) as *mut u16;
        for (i, &u) in units[..copy_n].iter().enumerate() {
            std::ptr::write_unaligned(dst.add(i), u);
        }
        std::ptr::write_unaligned(dst.add(copy_n), 0);
    }
    TRUE
}

pub unsafe extern "win64" fn SymGetSymFromAddr64(
    process: HANDLE,
    addr: u64,
    displacement: *mut u64,
    symbol: *mut u8,
) -> BOOL {
    if symbol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    // IMAGEHLP_SYMBOL64: +0 SizeOfStruct(u32), +8 Address(u64), +16 Size(u32),
    // +20 Flags(u32), +24 MaxNameLength(u32), +28 Name[1].
    let (name, _mod_base, disp) = resolve_symbol_name(process, addr);
    if !displacement.is_null() {
        std::ptr::write_unaligned(displacement, disp);
    }
    let max_name = std::ptr::read_unaligned(symbol.add(24) as *const u32) as usize;
    std::ptr::write_unaligned(symbol.add(8) as *mut u64, addr);
    if max_name > 0 {
        let copy_n = name.len().min(max_name - 1);
        std::ptr::copy_nonoverlapping(name.as_bytes().as_ptr(), symbol.add(28), copy_n);
        std::ptr::write_unaligned(symbol.add(28 + copy_n), 0);
    }
    TRUE
}

pub unsafe extern "win64" fn EnumerateLoadedModules64(
    _process: HANDLE,
    callback: *const c_void,
    user_context: *mut c_void,
) -> BOOL {
    if callback.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mods = crate::try_loader().map(|l| l.loaded_modules()).unwrap_or_default();
    for (base, size, path) in mods {
        let mut cstr = path.into_bytes();
        cstr.push(0);
        let cont = crate::call_guest8(
            callback as usize,
            cstr.as_ptr() as u64,
            base as u64,
            size as u64,
            user_context as u64,
            0,
            0,
            0,
            0,
        );
        if cont == 0 {
            break;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn EnumerateLoadedModulesW64(
    _process: HANDLE,
    callback: *const c_void,
    user_context: *mut c_void,
) -> BOOL {
    if callback.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mods = crate::try_loader().map(|l| l.loaded_modules()).unwrap_or_default();
    for (base, size, path) in mods {
        let wide = crate::strings::to_wide_z(&path);
        let cont = crate::call_guest8(
            callback as usize,
            wide.as_ptr() as u64,
            base as u64,
            size as u64,
            user_context as u64,
            0,
            0,
            0,
            0,
        );
        if cont == 0 {
            break;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn SymEnumerateModulesW64(
    process: HANDLE,
    callback: *const c_void,
    user_context: *mut c_void,
) -> BOOL {
    if callback.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let _ = process;
    let mods = crate::try_loader().map(|l| l.loaded_modules()).unwrap_or_default();
    for (base, _size, path) in mods {
        let mod_name = path.rsplit(['\\', '/']).next().unwrap_or(&path);
        let wide = crate::strings::to_wide_z(mod_name);
        // PSYM_ENUMMODULES_CALLBACKW64(PCWSTR ModuleName, DWORD64 BaseOfDll, PVOID UserContext)
        let cont = crate::call_guest8(
            callback as usize,
            wide.as_ptr() as u64,
            base as u64,
            user_context as u64,
            0,
            0,
            0,
            0,
            0,
        );
        if cont == 0 {
            break;
        }
    }
    TRUE
}

// @exports SymGetLineFromAddr64 SymGetLineFromAddrW64 StackWalk64 MiniDumpWriteDump
failing!(BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED;
    SymGetLineFromAddr64, SymGetLineFromAddrW64,
    StackWalk64, MiniDumpWriteDump);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sym_lifecycle_search_path_module_and_addr_work() {
        unsafe {
            let proc = 0x1234usize as HANDLE;
            assert_eq!(SymInitialize(proc, c"C:\\Symbols".as_ptr() as *const u8, FALSE), TRUE);

            let mut path_buf = [0u8; 64];
            assert_eq!(SymGetSearchPath(proc, path_buf.as_mut_ptr(), 64), TRUE);
            assert_eq!(crate::strings::from_ansi(path_buf.as_ptr()), "C:\\Symbols");

            let new_w = crate::strings::to_wide_z("C:\\AltSymbols");
            assert_eq!(SymSetSearchPathW(proc, new_w.as_ptr()), TRUE);
            let mut wbuf = [0u16; 64];
            assert_eq!(SymGetSearchPathW(proc, wbuf.as_mut_ptr(), 64), TRUE);
            assert_eq!(crate::strings::from_wide(wbuf.as_ptr()), "C:\\AltSymbols");

            let base = SymLoadModule64(
                proc,
                std::ptr::null_mut(),
                c"C:\\Game\\app.exe".as_ptr() as *const u8,
                c"app".as_ptr() as *const u8,
                0x14000_0000,
                0x20000,
            );
            assert_eq!(base, 0x14000_0000);
            assert_eq!(SymGetModuleBase64(proc, 0x14000_1234), 0x14000_0000);

            let mut mod_info = vec![0u8; 600];
            (mod_info.as_mut_ptr() as *mut u32).write_unaligned(600);
            assert_eq!(SymGetModuleInfo64(proc, 0x14000_1234, mod_info.as_mut_ptr()), TRUE);
            assert_eq!((mod_info.as_ptr().add(8) as *const u64).read_unaligned(), 0x14000_0000);
            assert_eq!(crate::strings::from_ansi(mod_info.as_ptr().add(36)), "app");

            let mut sym = vec![0u8; 88 + 128];
            (sym.as_mut_ptr() as *mut u32).write_unaligned(88);
            (sym.as_mut_ptr().add(80) as *mut u32).write_unaligned(128);
            let mut disp = 0u64;
            assert_eq!(SymFromAddr(proc, 0x14000_1234, &mut disp, sym.as_mut_ptr()), TRUE);
            assert_eq!(disp, 0x1234);
            assert_eq!((sym.as_ptr().add(32) as *const u64).read_unaligned(), 0x14000_0000);

            assert_eq!(SymCleanup(proc), TRUE);
        }
    }
}
