//! `hostfxr.dll`: Modern .NET Core / .NET 5/6/7/8/9 Host FX Resolver Bridge.
//!
//! Prefers a native host `libhostfxr.dylib` via `dlopen` when installed on macOS,
//! forwarding all `hostfxr_*` entrypoints with Windows UTF-16 <-> Unix UTF-8 string
//! conversion and `crate::path::to_host` path translation.
//!
//! When no native `libhostfxr.dylib` is installed (clean macOS or iOS), runs directly
//! on the embedded pure-Rust ECMA-335 CLR/CIL runtime (`crate::misc::clr`): parsing
//! real `.runtimeconfig.json`, running real managed `Main` entry points, and returning
//! genuinely callable Win64 function pointers into managed CIL methods — while still
//! reporting honest error codes for missing configs, files, types, or methods.

use crate::types::*;
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

pub const STATUS_SUCCESS: i32 = 0;
pub const INVALID_ARG_FAILURE: i32 = 0x8000_8081u32 as i32;
pub const HOST_API_MISSING: i32 = 0x8000_8083u32 as i32;
pub const INVALID_CONFIG_FILE: i32 = 0x8000_8092u32 as i32;
pub const APP_ARG_NOT_RUNNABLE: i32 = 0x8000_8093u32 as i32;
pub const HOST_API_BUFFER_TOO_SMALL: i32 = 0x8000_8098u32 as i32;
pub const HOST_PROPERTY_NOT_FOUND: i32 = 0x8000_80A1u32 as i32;

static HOSTFXR_ERROR_WRITER: AtomicUsize = AtomicUsize::new(0);
static NATIVE_DEL_LOAD_AND_GET_FP: AtomicUsize = AtomicUsize::new(0);
static NATIVE_DEL_GET_FP: AtomicUsize = AtomicUsize::new(0);
static NATIVE_DEL_LOAD_ASM: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
struct WinHostfxrInitParams {
    size: usize,
    host_path: LPCWSTR,
    dotnet_root: LPCWSTR,
}

#[repr(C)]
#[allow(dead_code)]
struct NativeHostfxrInitParams {
    size: usize,
    host_path: *const c_char,
    dotnet_root: *const c_char,
}

type NativeInitCmdlineFn = unsafe extern "C" fn(
    c_int,
    *const *const c_char,
    *const NativeHostfxrInitParams,
    *mut *mut c_void,
) -> i32;
type NativeInitConfigFn = unsafe extern "C" fn(
    *const c_char,
    *const NativeHostfxrInitParams,
    *mut *mut c_void,
) -> i32;
type NativeGetDelegateFn = unsafe extern "C" fn(*mut c_void, i32, *mut *mut c_void) -> i32;
type NativeRunAppFn = unsafe extern "C" fn(*mut c_void) -> i32;
type NativeGetPropFn =
    unsafe extern "C" fn(*mut c_void, *const c_char, *mut *const c_char) -> i32;
type NativeSetPropFn = unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> i32;
type NativeGetPropsFn = unsafe extern "C" fn(
    *mut c_void,
    *mut usize,
    *mut *const c_char,
    *mut *const c_char,
) -> i32;
type NativeCloseFn = unsafe extern "C" fn(*mut c_void) -> i32;
type NativeSetErrorWriterFn = unsafe extern "C" fn(*const c_void) -> *const c_void;
type NativeMainFn = unsafe extern "C" fn(c_int, *const *const c_char) -> i32;
type NativeMainStartupInfoFn = unsafe extern "C" fn(
    c_int,
    *const *const c_char,
    *const c_char,
    *const c_char,
    *const c_char,
) -> i32;

type NativeLoadAsmAndGetFpFn = unsafe extern "C" fn(
    *const c_char,
    *const c_char,
    *const c_char,
    *const c_char,
    *mut c_void,
    *mut *mut c_void,
) -> i32;
type NativeGetFpFn = unsafe extern "C" fn(
    *const c_char,
    *const c_char,
    *const c_char,
    *mut c_void,
    *mut c_void,
    *mut *mut c_void,
) -> i32;
type NativeLoadAsmFn = unsafe extern "C" fn(*const c_char, *mut c_void, *mut c_void) -> i32;

struct NativeHostfxr {
    _handle: usize,
    initialize_for_dotnet_command_line: Option<NativeInitCmdlineFn>,
    initialize_for_runtime_config: Option<NativeInitConfigFn>,
    get_runtime_delegate: Option<NativeGetDelegateFn>,
    run_app: Option<NativeRunAppFn>,
    get_runtime_property_value: Option<NativeGetPropFn>,
    set_runtime_property_value: Option<NativeSetPropFn>,
    get_runtime_properties: Option<NativeGetPropsFn>,
    close: Option<NativeCloseFn>,
    set_error_writer: Option<NativeSetErrorWriterFn>,
    main: Option<NativeMainFn>,
    main_startupinfo: Option<NativeMainStartupInfoFn>,
}

unsafe impl Send for NativeHostfxr {}
unsafe impl Sync for NativeHostfxr {}

unsafe fn resolve_sym<T: Copy>(handle: *mut c_void, name: &CStr) -> Option<T> {
    debug_assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<*mut c_void>());
    let p = libc::dlsym(handle, name.as_ptr());
    if p.is_null() {
        None
    } else {
        Some(std::mem::transmute_copy::<*mut c_void, T>(&p))
    }
}

/// Discover a native host `libhostfxr.dylib` if installed on disk.
pub fn find_host_libhostfxr() -> Option<PathBuf> {
    for env_key in ["PORT_HOSTFXR_PATH", "AW2_HOSTFXR_PATH"] {
        if let Some(p) = std::env::var_os(env_key) {
            let pb = PathBuf::from(p);
            if pb.is_file() {
                return Some(pb);
            }
        }
    }
    let mut roots = Vec::new();
    if let Some(root) = std::env::var_os("DOTNET_ROOT") {
        roots.push(PathBuf::from(root).join("host/fxr"));
    }
    roots.push(PathBuf::from("/usr/local/share/dotnet/host/fxr"));
    roots.push(PathBuf::from("/opt/homebrew/share/dotnet/host/fxr"));
    if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".dotnet/host/fxr"));
    }
    for root in roots {
        let Ok(rd) = std::fs::read_dir(&root) else {
            continue;
        };
        let mut versions: Vec<PathBuf> = rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.join("libhostfxr.dylib").is_file())
            .collect();
        versions.sort();
        if let Some(best) = versions.pop() {
            return Some(best.join("libhostfxr.dylib"));
        }
    }
    None
}

fn native_hostfxr() -> Option<&'static NativeHostfxr> {
    static NATIVE: OnceLock<Option<NativeHostfxr>> = OnceLock::new();
    NATIVE
        .get_or_init(|| {
            let dylib = find_host_libhostfxr()?;
            let cpath = CString::new(dylib.to_string_lossy().as_bytes()).ok()?;
            unsafe {
                let h = libc::dlopen(cpath.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
                if h.is_null() {
                    log::warn!("hostfxr: dlopen({dylib:?}) failed");
                    return None;
                }
                log::info!("hostfxr: loaded native {dylib:?}");
                Some(NativeHostfxr {
                    _handle: h as usize,
                    initialize_for_dotnet_command_line: resolve_sym(
                        h,
                        c"hostfxr_initialize_for_dotnet_command_line",
                    ),
                    initialize_for_runtime_config: resolve_sym(
                        h,
                        c"hostfxr_initialize_for_runtime_config",
                    ),
                    get_runtime_delegate: resolve_sym(h, c"hostfxr_get_runtime_delegate"),
                    run_app: resolve_sym(h, c"hostfxr_run_app"),
                    get_runtime_property_value: resolve_sym(
                        h,
                        c"hostfxr_get_runtime_property_value",
                    ),
                    set_runtime_property_value: resolve_sym(
                        h,
                        c"hostfxr_set_runtime_property_value",
                    ),
                    get_runtime_properties: resolve_sym(h, c"hostfxr_get_runtime_properties"),
                    close: resolve_sym(h, c"hostfxr_close"),
                    set_error_writer: resolve_sym(h, c"hostfxr_set_error_writer"),
                    main: resolve_sym(h, c"hostfxr_main"),
                    main_startupinfo: resolve_sym(h, c"hostfxr_main_startupinfo"),
                })
            }
        })
        .as_ref()
}

/// Per-context cache of converted UTF-16 property strings returned to Windows callers.
fn wide_prop_cache() -> &'static Mutex<HashMap<usize, Vec<Vec<u16>>>> {
    static CACHE: OnceLock<Mutex<HashMap<usize, Vec<Vec<u16>>>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cache_wide_str(ctx: usize, s: &str) -> LPCWSTR {
    let wide = crate::strings::to_wide_z(s);
    let ptr = wide.as_ptr();
    wide_prop_cache()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .entry(ctx)
        .or_default()
        .push(wide);
    ptr
}

fn translate_win_path_to_utf8(win_path: &str) -> String {
    super::clr::translate_win_path(win_path)
        .to_string_lossy()
        .into_owned()
}

fn opt_cstring(s: Option<String>) -> Option<CString> {
    s.and_then(|v| CString::new(v.as_bytes()).ok())
}

unsafe fn read_win_init_params(parameters: *const c_void) -> (Option<PathBuf>, Option<PathBuf>) {
    if parameters.is_null() {
        return (None, None);
    }
    let win = std::ptr::read_unaligned(parameters as *const WinHostfxrInitParams);
    if win.size < std::mem::size_of::<WinHostfxrInitParams>() {
        return (None, None);
    }
    let host_p = crate::strings::from_wide_opt(win.host_path).map(|s| super::clr::translate_win_path(&s));
    let root_p = crate::strings::from_wide_opt(win.dotnet_root).map(|s| super::clr::translate_win_path(&s));
    (host_p, root_p)
}

unsafe fn convert_init_params(
    parameters: *const c_void,
) -> Option<(Option<CString>, Option<CString>, NativeHostfxrInitParams)> {
    if parameters.is_null() {
        return None;
    }
    let win = std::ptr::read_unaligned(parameters as *const WinHostfxrInitParams);
    if win.size < std::mem::size_of::<WinHostfxrInitParams>() {
        return None;
    }
    let host_c = opt_cstring(
        crate::strings::from_wide_opt(win.host_path).map(|s| translate_win_path_to_utf8(&s)),
    );
    let root_c = opt_cstring(
        crate::strings::from_wide_opt(win.dotnet_root).map(|s| translate_win_path_to_utf8(&s)),
    );
    let native = NativeHostfxrInitParams {
        size: std::mem::size_of::<NativeHostfxrInitParams>(),
        host_path: host_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        dotnet_root: root_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
    };
    Some((host_c, root_c, native))
}

unsafe extern "C" fn native_error_writer_bridge(message: *const c_char) {
    let guest_cb = HOSTFXR_ERROR_WRITER.load(Ordering::Acquire);
    if guest_cb == 0 || message.is_null() {
        return;
    }
    let msg = CStr::from_ptr(message).to_string_lossy();
    let wide = crate::strings::to_wide_z(&msg);
    crate::call_guest8(guest_cb, wide.as_ptr() as u64, 0, 0, 0, 0, 0, 0, 0);
}

// ---------------------------------------------------------------------------
// Native libhostfxr delegate adapters (UTF-16 <-> UTF-8)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn hdt_load_assembly_and_get_function_pointer_adapter(
    assembly_path: LPCWSTR,
    type_name: LPCWSTR,
    method_name: LPCWSTR,
    delegate_type_name: LPCWSTR,
    reserved: *mut c_void,
    delegate: *mut *mut c_void,
) -> i32 {
    if delegate.is_null() {
        return INVALID_ARG_FAILURE;
    }
    std::ptr::write_unaligned(delegate, std::ptr::null_mut());
    let faddr = NATIVE_DEL_LOAD_AND_GET_FP.load(Ordering::Acquire);
    if faddr == 0 {
        return embedded_load_assembly_and_get_fp(
            assembly_path,
            type_name,
            method_name,
            delegate_type_name,
            reserved,
            delegate,
        );
    }
    let asm_c = opt_cstring(
        crate::strings::from_wide_opt(assembly_path).map(|s| translate_win_path_to_utf8(&s)),
    );
    let ty_c = opt_cstring(crate::strings::from_wide_opt(type_name));
    let m_c = opt_cstring(crate::strings::from_wide_opt(method_name));
    let dt_is_unmanaged = delegate_type_name as usize == usize::MAX;
    let dt_c = if dt_is_unmanaged {
        None
    } else {
        opt_cstring(crate::strings::from_wide_opt(delegate_type_name))
    };
    let native_fn: NativeLoadAsmAndGetFpFn = std::mem::transmute(faddr);
    let mut out_fn: *mut c_void = std::ptr::null_mut();
    let rc = native_fn(
        asm_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        ty_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        m_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        if dt_is_unmanaged {
            usize::MAX as *const c_char
        } else {
            dt_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
        },
        reserved,
        &mut out_fn,
    );
    if rc == STATUS_SUCCESS && !out_fn.is_null() {
        crate::register_native_va(out_fn as u64, out_fn as usize);
        std::ptr::write_unaligned(delegate, out_fn);
    }
    rc
}

unsafe extern "win64" fn hdt_get_function_pointer_adapter(
    type_name: LPCWSTR,
    method_name: LPCWSTR,
    delegate_type_name: LPCWSTR,
    load_context: *mut c_void,
    reserved: *mut c_void,
    delegate: *mut *mut c_void,
) -> i32 {
    if delegate.is_null() {
        return INVALID_ARG_FAILURE;
    }
    std::ptr::write_unaligned(delegate, std::ptr::null_mut());
    let faddr = NATIVE_DEL_GET_FP.load(Ordering::Acquire);
    if faddr == 0 {
        return embedded_get_fp(
            type_name,
            method_name,
            delegate_type_name,
            load_context,
            reserved,
            delegate,
        );
    }
    let ty_c = opt_cstring(crate::strings::from_wide_opt(type_name));
    let m_c = opt_cstring(crate::strings::from_wide_opt(method_name));
    let dt_is_unmanaged = delegate_type_name as usize == usize::MAX;
    let dt_c = if dt_is_unmanaged {
        None
    } else {
        opt_cstring(crate::strings::from_wide_opt(delegate_type_name))
    };
    let native_fn: NativeGetFpFn = std::mem::transmute(faddr);
    let mut out_fn: *mut c_void = std::ptr::null_mut();
    let rc = native_fn(
        ty_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        m_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        if dt_is_unmanaged {
            usize::MAX as *const c_char
        } else {
            dt_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
        },
        load_context,
        reserved,
        &mut out_fn,
    );
    if rc == STATUS_SUCCESS && !out_fn.is_null() {
        crate::register_native_va(out_fn as u64, out_fn as usize);
        std::ptr::write_unaligned(delegate, out_fn);
    }
    rc
}

unsafe extern "win64" fn hdt_load_assembly_adapter(
    assembly_path: LPCWSTR,
    load_context: *mut c_void,
    reserved: *mut c_void,
) -> i32 {
    let faddr = NATIVE_DEL_LOAD_ASM.load(Ordering::Acquire);
    if faddr == 0 {
        return embedded_load_assembly(assembly_path, load_context, reserved);
    }
    let asm_c = opt_cstring(
        crate::strings::from_wide_opt(assembly_path).map(|s| translate_win_path_to_utf8(&s)),
    );
    let native_fn: NativeLoadAsmFn = std::mem::transmute(faddr);
    native_fn(
        asm_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        load_context,
        reserved,
    )
}

// ---------------------------------------------------------------------------
// Embedded pure-Rust CLR runtime delegates
// ---------------------------------------------------------------------------

unsafe extern "win64" fn embedded_load_assembly_and_get_fp(
    assembly_path: LPCWSTR,
    type_name: LPCWSTR,
    method_name: LPCWSTR,
    delegate_type_name: LPCWSTR,
    _reserved: *mut c_void,
    delegate: *mut *mut c_void,
) -> i32 {
    if delegate.is_null() {
        return INVALID_ARG_FAILURE;
    }
    std::ptr::write_unaligned(delegate, std::ptr::null_mut());
    let Some(asm_s) = crate::strings::from_wide_opt(assembly_path) else {
        return INVALID_ARG_FAILURE;
    };
    let Some(ty_s) = crate::strings::from_wide_opt(type_name) else {
        return INVALID_ARG_FAILURE;
    };
    let Some(m_s) = crate::strings::from_wide_opt(method_name) else {
        return INVALID_ARG_FAILURE;
    };
    let asm_pb = super::clr::translate_win_path(&asm_s);
    let is_component = delegate_type_name.is_null();
    match super::clr::hostfxr_load_assembly_and_get_fp(Some(&asm_pb), &ty_s, &m_s, is_component) {
        Ok(fp) => {
            std::ptr::write_unaligned(delegate, fp as *mut c_void);
            STATUS_SUCCESS
        }
        Err(rc) => rc,
    }
}

unsafe extern "win64" fn embedded_get_fp(
    type_name: LPCWSTR,
    method_name: LPCWSTR,
    delegate_type_name: LPCWSTR,
    _load_context: *mut c_void,
    _reserved: *mut c_void,
    delegate: *mut *mut c_void,
) -> i32 {
    if delegate.is_null() {
        return INVALID_ARG_FAILURE;
    }
    std::ptr::write_unaligned(delegate, std::ptr::null_mut());
    let Some(ty_s) = crate::strings::from_wide_opt(type_name) else {
        return INVALID_ARG_FAILURE;
    };
    let Some(m_s) = crate::strings::from_wide_opt(method_name) else {
        return INVALID_ARG_FAILURE;
    };
    let is_component = delegate_type_name.is_null();
    match super::clr::hostfxr_load_assembly_and_get_fp(None, &ty_s, &m_s, is_component) {
        Ok(fp) => {
            std::ptr::write_unaligned(delegate, fp as *mut c_void);
            STATUS_SUCCESS
        }
        Err(rc) => rc,
    }
}

unsafe extern "win64" fn embedded_load_assembly(
    assembly_path: LPCWSTR,
    _load_context: *mut c_void,
    _reserved: *mut c_void,
) -> i32 {
    let Some(asm_s) = crate::strings::from_wide_opt(assembly_path) else {
        return INVALID_ARG_FAILURE;
    };
    let pb = super::clr::translate_win_path(&asm_s);
    super::clr::hostfxr_load_assembly(&pb)
}

unsafe extern "win64" fn embedded_load_assembly_bytes(
    assembly_bytes: *const u8,
    assembly_bytes_len: usize,
    _symbols_bytes: *const u8,
    _symbols_bytes_len: usize,
    _load_context: *mut c_void,
    _reserved: *mut c_void,
) -> i32 {
    if assembly_bytes.is_null() || assembly_bytes_len == 0 {
        return INVALID_ARG_FAILURE;
    }
    let slice = std::slice::from_raw_parts(assembly_bytes, assembly_bytes_len);
    super::clr::hostfxr_load_assembly_bytes(slice)
}

// ---------------------------------------------------------------------------
// Internal implementation (native hostfxr first, embedded CLR fallback)
// ---------------------------------------------------------------------------

unsafe fn init_for_cmdline_with_rt(
    rt: Option<&NativeHostfxr>,
    argc: c_int,
    argv: *const LPCWSTR,
    parameters: *const c_void,
    host_context_handle: *mut *mut c_void,
) -> i32 {
    if host_context_handle.is_null() {
        return INVALID_ARG_FAILURE;
    }
    std::ptr::write_unaligned(host_context_handle, std::ptr::null_mut());

    if let Some(native_fn) = rt.and_then(|r| r.initialize_for_dotnet_command_line) {
        let mut c_args: Vec<CString> = Vec::new();
        if argc > 0 && !argv.is_null() {
            for i in 0..(argc as usize) {
                let wptr = std::ptr::read_unaligned(argv.add(i));
                let s = crate::strings::from_wide_opt(wptr)
                    .map(|v| translate_win_path_to_utf8(&v))
                    .unwrap_or_default();
                c_args.push(CString::new(s.as_bytes()).unwrap_or_default());
            }
        }
        let ptrs: Vec<*const c_char> = c_args.iter().map(|c| c.as_ptr()).collect();
        let params_box = convert_init_params(parameters);
        let params_ptr = params_box
            .as_ref()
            .map_or(std::ptr::null(), |(_, _, p)| p as *const NativeHostfxrInitParams);
        let mut out_ctx: *mut c_void = std::ptr::null_mut();
        let rc = native_fn(ptrs.len() as c_int, ptrs.as_ptr(), params_ptr, &mut out_ctx);
        std::ptr::write_unaligned(host_context_handle, out_ctx);
        return rc;
    }

    let mut args = Vec::new();
    if argc > 0 && !argv.is_null() {
        for i in 0..(argc as usize) {
            let wptr = std::ptr::read_unaligned(argv.add(i));
            if let Some(s) = crate::strings::from_wide_opt(wptr) {
                args.push(s);
            }
        }
    }
    let (host_p, root_p) = read_win_init_params(parameters);
    match super::clr::hostfxr_init_for_cmdline(&args, host_p.as_deref(), root_p.as_deref()) {
        Ok(h) => {
            std::ptr::write_unaligned(host_context_handle, h as *mut c_void);
            STATUS_SUCCESS
        }
        Err(rc) => rc,
    }
}

unsafe fn init_for_config_with_rt(
    rt: Option<&NativeHostfxr>,
    runtime_config_path: LPCWSTR,
    parameters: *const c_void,
    host_context_handle: *mut *mut c_void,
) -> i32 {
    if host_context_handle.is_null() {
        return INVALID_ARG_FAILURE;
    }
    std::ptr::write_unaligned(host_context_handle, std::ptr::null_mut());
    let Some(cfg_s) = crate::strings::from_wide_opt(runtime_config_path) else {
        return INVALID_ARG_FAILURE;
    };

    if let Some(native_fn) = rt.and_then(|r| r.initialize_for_runtime_config) {
        let host_cfg = translate_win_path_to_utf8(&cfg_s);
        let Ok(cfg_c) = CString::new(host_cfg.as_bytes()) else {
            return INVALID_ARG_FAILURE;
        };
        let params_box = convert_init_params(parameters);
        let params_ptr = params_box
            .as_ref()
            .map_or(std::ptr::null(), |(_, _, p)| p as *const NativeHostfxrInitParams);
        let mut out_ctx: *mut c_void = std::ptr::null_mut();
        let rc = native_fn(cfg_c.as_ptr(), params_ptr, &mut out_ctx);
        std::ptr::write_unaligned(host_context_handle, out_ctx);
        return rc;
    }

    let cfg_pb = super::clr::translate_win_path(&cfg_s);
    let (host_p, root_p) = read_win_init_params(parameters);
    match super::clr::hostfxr_init_for_config(&cfg_pb, host_p.as_deref(), root_p.as_deref()) {
        Ok(h) => {
            std::ptr::write_unaligned(host_context_handle, h as *mut c_void);
            STATUS_SUCCESS
        }
        Err(rc) => rc,
    }
}

unsafe fn get_delegate_with_rt(
    rt: Option<&NativeHostfxr>,
    host_context_handle: *mut c_void,
    delegate_type: i32,
    delegate: *mut *mut c_void,
) -> i32 {
    if delegate.is_null() {
        return INVALID_ARG_FAILURE;
    }
    std::ptr::write_unaligned(delegate, std::ptr::null_mut());
    if host_context_handle.is_null() {
        return INVALID_ARG_FAILURE;
    }

    let is_embedded = super::clr::hostfxr_is_embedded_handle(host_context_handle as usize);
    if !is_embedded {
        if let Some(native_fn) = rt.and_then(|r| r.get_runtime_delegate) {
            let mut native_del: *mut c_void = std::ptr::null_mut();
            let rc = native_fn(host_context_handle, delegate_type, &mut native_del);
            if rc != STATUS_SUCCESS || native_del.is_null() {
                return if rc == STATUS_SUCCESS {
                    HOST_API_MISSING
                } else {
                    rc
                };
            }
            let out_ptr = match delegate_type {
                5 => {
                    NATIVE_DEL_LOAD_AND_GET_FP.store(native_del as usize, Ordering::Release);
                    hdt_load_assembly_and_get_function_pointer_adapter as *const () as usize
                }
                6 => {
                    NATIVE_DEL_GET_FP.store(native_del as usize, Ordering::Release);
                    hdt_get_function_pointer_adapter as *const () as usize
                }
                7 => {
                    NATIVE_DEL_LOAD_ASM.store(native_del as usize, Ordering::Release);
                    hdt_load_assembly_adapter as *const () as usize
                }
                _ => native_del as usize,
            };
            crate::register_native_va(out_ptr as u64, out_ptr);
            std::ptr::write_unaligned(delegate, out_ptr as *mut c_void);
            return rc;
        }
        return INVALID_ARG_FAILURE;
    }

    let out_ptr = match delegate_type {
        5 => embedded_load_assembly_and_get_fp as *const () as usize,
        6 => embedded_get_fp as *const () as usize,
        7 => embedded_load_assembly as *const () as usize,
        8 => embedded_load_assembly_bytes as *const () as usize,
        _ => return INVALID_ARG_FAILURE,
    };
    crate::register_native_va(out_ptr as u64, out_ptr);
    std::ptr::write_unaligned(delegate, out_ptr as *mut c_void);
    STATUS_SUCCESS
}

unsafe fn run_app_with_rt(rt: Option<&NativeHostfxr>, host_context_handle: *mut c_void) -> i32 {
    if host_context_handle.is_null() {
        return INVALID_ARG_FAILURE;
    }
    if super::clr::hostfxr_is_embedded_handle(host_context_handle as usize) {
        return super::clr::hostfxr_run_app(host_context_handle as usize);
    }
    if let Some(native_fn) = rt.and_then(|r| r.run_app) {
        return native_fn(host_context_handle);
    }
    INVALID_ARG_FAILURE
}

unsafe fn get_prop_with_rt(
    rt: Option<&NativeHostfxr>,
    host_context_handle: *mut c_void,
    name: LPCWSTR,
    value: *mut LPCWSTR,
) -> i32 {
    if value.is_null() {
        return INVALID_ARG_FAILURE;
    }
    std::ptr::write_unaligned(value, std::ptr::null());
    if host_context_handle.is_null() || name.is_null() {
        return INVALID_ARG_FAILURE;
    }
    let key = crate::strings::from_wide(name);

    if super::clr::hostfxr_is_embedded_handle(host_context_handle as usize) {
        return match super::clr::hostfxr_get_property(host_context_handle as usize, &key) {
            Ok(v) => {
                let wptr = cache_wide_str(host_context_handle as usize, &v);
                std::ptr::write_unaligned(value, wptr);
                STATUS_SUCCESS
            }
            Err(rc) => rc,
        };
    }

    let Some(native_fn) = rt.and_then(|r| r.get_runtime_property_value) else {
        return INVALID_ARG_FAILURE;
    };
    let Ok(key_c) = CString::new(key.as_bytes()) else {
        return INVALID_ARG_FAILURE;
    };
    let mut val_c: *const c_char = std::ptr::null();
    let rc = native_fn(host_context_handle, key_c.as_ptr(), &mut val_c);
    if rc == STATUS_SUCCESS && !val_c.is_null() {
        let val_s = CStr::from_ptr(val_c).to_string_lossy();
        let wptr = cache_wide_str(host_context_handle as usize, &val_s);
        std::ptr::write_unaligned(value, wptr);
    }
    rc
}

unsafe fn set_prop_with_rt(
    rt: Option<&NativeHostfxr>,
    host_context_handle: *mut c_void,
    name: LPCWSTR,
    value: LPCWSTR,
) -> i32 {
    if host_context_handle.is_null() || name.is_null() {
        return INVALID_ARG_FAILURE;
    }
    let key = crate::strings::from_wide(name);
    let val_opt = crate::strings::from_wide_opt(value);

    if super::clr::hostfxr_is_embedded_handle(host_context_handle as usize) {
        return super::clr::hostfxr_set_property(
            host_context_handle as usize,
            &key,
            val_opt.as_deref(),
        );
    }

    let Some(native_fn) = rt.and_then(|r| r.set_runtime_property_value) else {
        return INVALID_ARG_FAILURE;
    };
    let Ok(key_c) = CString::new(key.as_bytes()) else {
        return INVALID_ARG_FAILURE;
    };
    let val_c = opt_cstring(val_opt);
    native_fn(
        host_context_handle,
        key_c.as_ptr(),
        val_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
    )
}

unsafe fn get_props_with_rt(
    rt: Option<&NativeHostfxr>,
    host_context_handle: *mut c_void,
    count: *mut usize,
    keys: *mut LPCWSTR,
    values: *mut LPCWSTR,
) -> i32 {
    if count.is_null() {
        return INVALID_ARG_FAILURE;
    }
    if host_context_handle.is_null() {
        std::ptr::write_unaligned(count, 0);
        return INVALID_ARG_FAILURE;
    }

    if super::clr::hostfxr_is_embedded_handle(host_context_handle as usize) {
        let caller_cap = std::ptr::read_unaligned(count);
        let pairs = match super::clr::hostfxr_get_properties(host_context_handle as usize) {
            Ok(p) => p,
            Err(rc) => {
                std::ptr::write_unaligned(count, 0);
                return rc;
            }
        };
        let needed = pairs.len();
        std::ptr::write_unaligned(count, needed);
        if keys.is_null() || values.is_null() || caller_cap < needed {
            return if needed == 0 {
                STATUS_SUCCESS
            } else {
                HOST_API_BUFFER_TOO_SMALL
            };
        }
        let ctx_id = host_context_handle as usize;
        for (i, (k, v)) in pairs.iter().enumerate() {
            std::ptr::write_unaligned(keys.add(i), cache_wide_str(ctx_id, k));
            std::ptr::write_unaligned(values.add(i), cache_wide_str(ctx_id, v));
        }
        return STATUS_SUCCESS;
    }

    let Some(native_fn) = rt.and_then(|r| r.get_runtime_properties) else {
        std::ptr::write_unaligned(count, 0);
        return INVALID_ARG_FAILURE;
    };
    let caller_cap = std::ptr::read_unaligned(count);
    let mut needed: usize = 0;
    let rc_probe = native_fn(
        host_context_handle,
        &mut needed,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    if rc_probe != STATUS_SUCCESS && rc_probe != HOST_API_BUFFER_TOO_SMALL {
        std::ptr::write_unaligned(count, needed);
        return rc_probe;
    }
    std::ptr::write_unaligned(count, needed);
    if keys.is_null() || values.is_null() || caller_cap < needed {
        return if needed == 0 {
            STATUS_SUCCESS
        } else {
            HOST_API_BUFFER_TOO_SMALL
        };
    }
    let mut c_keys = vec![std::ptr::null::<c_char>(); needed];
    let mut c_vals = vec![std::ptr::null::<c_char>(); needed];
    let mut actual = needed;
    let rc = native_fn(
        host_context_handle,
        &mut actual,
        c_keys.as_mut_ptr(),
        c_vals.as_mut_ptr(),
    );
    std::ptr::write_unaligned(count, actual);
    if rc == STATUS_SUCCESS {
        let ctx_id = host_context_handle as usize;
        for i in 0..actual {
            let k_str = if c_keys[i].is_null() {
                String::new()
            } else {
                CStr::from_ptr(c_keys[i]).to_string_lossy().into_owned()
            };
            let v_str = if c_vals[i].is_null() {
                String::new()
            } else {
                CStr::from_ptr(c_vals[i]).to_string_lossy().into_owned()
            };
            std::ptr::write_unaligned(keys.add(i), cache_wide_str(ctx_id, &k_str));
            std::ptr::write_unaligned(values.add(i), cache_wide_str(ctx_id, &v_str));
        }
    }
    rc
}

unsafe fn close_with_rt(rt: Option<&NativeHostfxr>, host_context_handle: *mut c_void) -> i32 {
    if host_context_handle.is_null() {
        return INVALID_ARG_FAILURE;
    }
    wide_prop_cache()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&(host_context_handle as usize));
    if super::clr::hostfxr_is_embedded_handle(host_context_handle as usize) {
        return super::clr::hostfxr_close(host_context_handle as usize);
    }
    let Some(native_fn) = rt.and_then(|r| r.close) else {
        return INVALID_ARG_FAILURE;
    };
    native_fn(host_context_handle)
}

// ---------------------------------------------------------------------------
// Exported Win64 hostfxr.dll entrypoints
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn hostfxr_initialize_for_dotnet_command_line(
    argc: c_int,
    argv: *const LPCWSTR,
    parameters: *const c_void,
    host_context_handle: *mut *mut c_void,
) -> i32 {
    init_for_cmdline_with_rt(native_hostfxr(), argc, argv, parameters, host_context_handle)
}

pub unsafe extern "win64" fn hostfxr_initialize_for_runtime_config(
    runtime_config_path: LPCWSTR,
    parameters: *const c_void,
    host_context_handle: *mut *mut c_void,
) -> i32 {
    init_for_config_with_rt(
        native_hostfxr(),
        runtime_config_path,
        parameters,
        host_context_handle,
    )
}

pub unsafe extern "win64" fn hostfxr_get_runtime_delegate(
    host_context_handle: *mut c_void,
    delegate_type: i32,
    delegate: *mut *mut c_void,
) -> i32 {
    get_delegate_with_rt(native_hostfxr(), host_context_handle, delegate_type, delegate)
}

pub unsafe extern "win64" fn hostfxr_run_app(host_context_handle: *mut c_void) -> i32 {
    run_app_with_rt(native_hostfxr(), host_context_handle)
}

pub unsafe extern "win64" fn hostfxr_get_runtime_property_value(
    host_context_handle: *mut c_void,
    name: LPCWSTR,
    value: *mut LPCWSTR,
) -> i32 {
    get_prop_with_rt(native_hostfxr(), host_context_handle, name, value)
}

pub unsafe extern "win64" fn hostfxr_set_runtime_property_value(
    host_context_handle: *mut c_void,
    name: LPCWSTR,
    value: LPCWSTR,
) -> i32 {
    set_prop_with_rt(native_hostfxr(), host_context_handle, name, value)
}

pub unsafe extern "win64" fn hostfxr_get_runtime_properties(
    host_context_handle: *mut c_void,
    count: *mut usize,
    keys: *mut LPCWSTR,
    values: *mut LPCWSTR,
) -> i32 {
    get_props_with_rt(native_hostfxr(), host_context_handle, count, keys, values)
}

pub unsafe extern "win64" fn hostfxr_close(host_context_handle: *mut c_void) -> i32 {
    close_with_rt(native_hostfxr(), host_context_handle)
}

pub unsafe extern "win64" fn hostfxr_set_error_writer(
    error_writer: *const c_void,
) -> *const c_void {
    let prev = HOSTFXR_ERROR_WRITER.swap(error_writer as usize, Ordering::AcqRel);
    if let Some(native_fn) = native_hostfxr().and_then(|r| r.set_error_writer) {
        let bridge = if error_writer.is_null() {
            std::ptr::null()
        } else {
            native_error_writer_bridge as *const c_void
        };
        let _ = native_fn(bridge);
    }
    prev as *const c_void
}

pub unsafe extern "win64" fn hostfxr_main(argc: c_int, argv: *const LPCWSTR) -> i32 {
    if let Some(native_fn) = native_hostfxr().and_then(|r| r.main) {
        let mut c_args: Vec<CString> = Vec::new();
        if argc > 0 && !argv.is_null() {
            for i in 0..(argc as usize) {
                let wptr = std::ptr::read_unaligned(argv.add(i));
                let s = crate::strings::from_wide_opt(wptr)
                    .map(|v| translate_win_path_to_utf8(&v))
                    .unwrap_or_default();
                c_args.push(CString::new(s.as_bytes()).unwrap_or_default());
            }
        }
        let ptrs: Vec<*const c_char> = c_args.iter().map(|c| c.as_ptr()).collect();
        return native_fn(ptrs.len() as c_int, ptrs.as_ptr());
    }
    let mut ctx: *mut c_void = std::ptr::null_mut();
    let rc = init_for_cmdline_with_rt(None, argc, argv, std::ptr::null(), &mut ctx);
    if rc != STATUS_SUCCESS || ctx.is_null() {
        return rc;
    }
    let run_rc = run_app_with_rt(None, ctx);
    let _ = close_with_rt(None, ctx);
    run_rc
}

pub unsafe extern "win64" fn hostfxr_main_startupinfo(
    argc: c_int,
    argv: *const LPCWSTR,
    host_path: LPCWSTR,
    dotnet_root: LPCWSTR,
    app_path: LPCWSTR,
) -> i32 {
    if let Some(native_fn) = native_hostfxr().and_then(|r| r.main_startupinfo) {
        let mut c_args: Vec<CString> = Vec::new();
        if argc > 0 && !argv.is_null() {
            for i in 0..(argc as usize) {
                let wptr = std::ptr::read_unaligned(argv.add(i));
                let s = crate::strings::from_wide_opt(wptr)
                    .map(|v| translate_win_path_to_utf8(&v))
                    .unwrap_or_default();
                c_args.push(CString::new(s.as_bytes()).unwrap_or_default());
            }
        }
        let ptrs: Vec<*const c_char> = c_args.iter().map(|c| c.as_ptr()).collect();
        let host_c = opt_cstring(
            crate::strings::from_wide_opt(host_path).map(|s| translate_win_path_to_utf8(&s)),
        );
        let root_c = opt_cstring(
            crate::strings::from_wide_opt(dotnet_root).map(|s| translate_win_path_to_utf8(&s)),
        );
        let app_c = opt_cstring(
            crate::strings::from_wide_opt(app_path).map(|s| translate_win_path_to_utf8(&s)),
        );
        return native_fn(
            ptrs.len() as c_int,
            ptrs.as_ptr(),
            host_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            root_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            app_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        );
    }
    let Some(app_s) = crate::strings::from_wide_opt(app_path) else {
        return hostfxr_main(argc, argv);
    };
    let mut args = vec![app_s];
    if argc > 1 && !argv.is_null() {
        for i in 1..(argc as usize) {
            let wptr = std::ptr::read_unaligned(argv.add(i));
            if let Some(s) = crate::strings::from_wide_opt(wptr) {
                args.push(s);
            }
        }
    }
    let app_pb = super::clr::translate_win_path(&args[0]);
    match super::clr::execute_entry_point_file(&app_pb, &args[1..]) {
        Ok(code) => code,
        Err(hr) => hr,
    }
}

#[cfg(test)]
mod tests {
    use super::super::clr::metadata::elem_type;
    use super::super::clr::metadata::test_asm::{TestAsmBuilder, TestMethodBody};
    use super::*;

    #[test]
    fn hostfxr_embedded_rejects_missing_and_invalid_inputs() {
        unsafe {
            let missing = crate::strings::to_wide_z("/nonexistent/Missing.runtimeconfig.json");
            let mut ctx: *mut c_void = 0xdeadusize as *mut c_void;

            assert_eq!(
                init_for_config_with_rt(None, missing.as_ptr(), std::ptr::null(), std::ptr::null_mut()),
                INVALID_ARG_FAILURE
            );
            assert_eq!(
                init_for_config_with_rt(None, missing.as_ptr(), std::ptr::null(), &mut ctx),
                INVALID_CONFIG_FILE
            );
            assert!(ctx.is_null());

            let argv = [missing.as_ptr()];
            ctx = 0xdeadusize as *mut c_void;
            assert_eq!(
                init_for_cmdline_with_rt(None, 1, argv.as_ptr(), std::ptr::null(), &mut ctx),
                APP_ARG_NOT_RUNNABLE
            );
            assert!(ctx.is_null());

            let mut del: *mut c_void = 0xdeadusize as *mut c_void;
            assert_eq!(
                get_delegate_with_rt(None, 0x1000usize as *mut c_void, 5, &mut del),
                INVALID_ARG_FAILURE
            );
            assert!(del.is_null());
            assert_eq!(run_app_with_rt(None, 0x1000usize as *mut c_void), INVALID_ARG_FAILURE);
            assert_eq!(close_with_rt(None, 0x1000usize as *mut c_void), INVALID_ARG_FAILURE);
        }
    }

    #[test]
    fn hostfxr_embedded_end_to_end_config_run_and_delegate() {
        let dir = std::env::temp_dir().join(format!(
            "port_clr_hostfxr_test_{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let dll_path = dir.join("HostFxrApp.dll");
        let cfg_path = dir.join("HostFxrApp.runtimeconfig.json");

        let mut b = TestAsmBuilder::new("HostFxrApp");
        let corlib = b.add_assembly_ref("mscorlib");
        let obj_ref = b.add_type_ref(corlib, "System", "Object");
        let _td = b.add_type_def("HostFxrDemo", "Entry", Some(obj_ref));
        // Main() -> returns 77
        let m_main = b.add_method(
            0x0016,
            0,
            "Main",
            &[0x00, 0x00, elem_type::I4],
            Some(TestMethodBody::tiny(vec![0x1F, 77, 0x2A])),
        );
        b.entry_point_method_rid = Some(m_main & 0x00FF_FFFF);
        // Compute(int a, int b) -> a * 3 + b
        let _m_comp = b.add_method(
            0x0016,
            0,
            "Compute",
            &[0x00, 0x02, elem_type::I4, elem_type::I4, elem_type::I4],
            Some(TestMethodBody::tiny(vec![0x02, 0x19, 0x5A, 0x03, 0x58, 0x2A])),
        );
        std::fs::write(&dll_path, b.build()).unwrap();
        std::fs::write(
            &cfg_path,
            r#"{
                "runtimeOptions": {
                    "tfm": "net8.0",
                    "framework": { "name": "Microsoft.NETCore.App", "version": "8.0.0" },
                    "configProperties": { "Demo.Flag": true, "Demo.Greeting": "HelloFXR" }
                }
            }"#,
        )
        .unwrap();

        unsafe {
            let cfg_w = crate::strings::to_wide_z(&cfg_path.to_string_lossy());
            let mut ctx: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                init_for_config_with_rt(None, cfg_w.as_ptr(), std::ptr::null(), &mut ctx),
                STATUS_SUCCESS
            );
            assert!(!ctx.is_null());

            // Read property parsed from .runtimeconfig.json
            let prop_k = crate::strings::to_wide_z("Demo.Greeting");
            let mut prop_v: LPCWSTR = std::ptr::null();
            assert_eq!(
                get_prop_with_rt(None, ctx, prop_k.as_ptr(), &mut prop_v),
                STATUS_SUCCESS
            );
            assert_eq!(crate::strings::from_wide(prop_v), "HelloFXR");

            // Missing property returns honest HOST_PROPERTY_NOT_FOUND
            let bad_k = crate::strings::to_wide_z("DoesNotExist");
            assert_eq!(
                get_prop_with_rt(None, ctx, bad_k.as_ptr(), &mut prop_v),
                HOST_PROPERTY_NOT_FOUND
            );
            assert!(prop_v.is_null());

            // Set custom property & enumerate all properties
            let custom_k = crate::strings::to_wide_z("Custom.Key");
            let custom_v = crate::strings::to_wide_z("CustomVal");
            assert_eq!(
                set_prop_with_rt(None, ctx, custom_k.as_ptr(), custom_v.as_ptr()),
                STATUS_SUCCESS
            );
            let mut count = 0usize;
            assert_eq!(
                get_props_with_rt(None, ctx, &mut count, std::ptr::null_mut(), std::ptr::null_mut()),
                HOST_API_BUFFER_TOO_SMALL
            );
            assert!(count >= 4);

            // Execute Main via hostfxr_run_app
            assert_eq!(run_app_with_rt(None, ctx), 77);

            // Obtain load_assembly_and_get_function_pointer delegate (type 5)
            let mut loader_del: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_delegate_with_rt(None, ctx, 5, &mut loader_del), STATUS_SUCCESS);
            assert!(!loader_del.is_null());

            type LoadAndGetFp = unsafe extern "win64" fn(
                LPCWSTR,
                LPCWSTR,
                LPCWSTR,
                LPCWSTR,
                *mut c_void,
                *mut *mut c_void,
            ) -> i32;
            let load_and_get: LoadAndGetFp = std::mem::transmute(loader_del);

            let dll_w = crate::strings::to_wide_z(&dll_path.to_string_lossy());
            let ty_w = crate::strings::to_wide_z("HostFxrDemo.Entry, HostFxrApp");
            let m_w = crate::strings::to_wide_z("Compute");
            let mut fn_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                load_and_get(
                    dll_w.as_ptr(),
                    ty_w.as_ptr(),
                    m_w.as_ptr(),
                    usize::MAX as LPCWSTR,
                    std::ptr::null_mut(),
                    &mut fn_ptr,
                ),
                STATUS_SUCCESS
            );
            assert!(!fn_ptr.is_null());

            // Invoke the managed CIL method directly through its Win64 trampoline!
            let compute: unsafe extern "win64" fn(i32, i32) -> i32 = std::mem::transmute(fn_ptr);
            assert_eq!(compute(10, 12), 42);

            // Nonexistent method fails honestly
            let missing_m = crate::strings::to_wide_z("NoSuchMethod");
            let mut bad_fp: *mut c_void = 0x1usize as *mut c_void;
            assert_eq!(
                load_and_get(
                    dll_w.as_ptr(),
                    ty_w.as_ptr(),
                    missing_m.as_ptr(),
                    usize::MAX as LPCWSTR,
                    std::ptr::null_mut(),
                    &mut bad_fp,
                ),
                super::super::clr::COR_E_MISSINGMETHOD
            );
            assert!(bad_fp.is_null());

            assert_eq!(close_with_rt(None, ctx), STATUS_SUCCESS);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
