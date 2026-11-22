//! dbghelp.dll: crash-reporting helpers.
//!
//! The function-table / module-base callbacks are real (backed by the loader
//! module list and `.pdata`). Symbol lookup, StackWalk64 and minidumps are not
//! implemented and fail with a STUB log; crashes are reported by our own
//! signal handler instead.

use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

static OPTIONS: AtomicU32 = AtomicU32::new(0);

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

pub unsafe extern "win64" fn SymGetModuleBase64(_process: HANDLE, addr: u64) -> u64 {
    crate::try_loader().and_then(|l| l.module_from_address(addr as usize)).unwrap_or(0) as u64
}

pub unsafe extern "win64" fn SymLoadModule64(
    _process: HANDLE,
    _file: HANDLE,
    _image_name: LPCSTR,
    _module_name: LPCSTR,
    base_of_dll: u64,
    _size_of_dll: u32,
) -> u64 {
    stub!("SymLoadModule64 -> 0");
    crate::teb::set_last_error(ERROR_CALL_NOT_IMPLEMENTED);
    let _ = base_of_dll;
    0
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

// @exports SymInitialize SymInitializeW SymCleanup SymFromAddr SymFromAddrW SymGetSymFromAddr64 SymGetLineFromAddr64 SymGetLineFromAddrW64 SymGetSearchPath SymGetSearchPathW SymSetSearchPath SymSetSearchPathW SymGetModuleInfo64 SymGetModuleInfoW64 StackWalk64 MiniDumpWriteDump
failing!(BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED;
    SymInitialize, SymInitializeW, SymCleanup, SymFromAddr, SymFromAddrW,
    SymGetSymFromAddr64, SymGetLineFromAddr64, SymGetLineFromAddrW64,
    SymGetSearchPath, SymGetSearchPathW, SymSetSearchPath, SymSetSearchPathW,
    SymGetModuleInfo64, SymGetModuleInfoW64, StackWalk64, MiniDumpWriteDump);
