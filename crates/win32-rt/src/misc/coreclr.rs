//! `coreclr.dll`: Modern .NET Core / .NET 5-9 CoreCLR Execution Engine Bridge.
//!
//! Prefers a native host `libcoreclr.dylib` when installed on disk (under
//! `PORT_CORECLR_PATH`, `AW2_CORECLR_PATH`, `$DOTNET_ROOT/shared/Microsoft.NETCore.App/*`,
//! `/usr/local/share/dotnet/shared/Microsoft.NETCore.App/*`,
//! `/opt/homebrew/share/dotnet/shared/Microsoft.NETCore.App/*`, or
//! `$HOME/.dotnet/shared/Microsoft.NETCore.App/*`).
//!
//! When no native `libcoreclr.dylib` is installed (clean macOS or iOS), runs directly
//! on the embedded pure-Rust ECMA-335 CLR/CIL runtime (`crate::misc::clr`) while
//! preserving honest error codes for missing assemblies, invalid images, or missing
//! types and methods.

use crate::types::*;
use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

pub const HOST_API_MISSING: HRESULT = 0x8000_8083u32 as i32;

static ERROR_WRITER: AtomicUsize = AtomicUsize::new(0);

type NativeCoreClrInitFn = unsafe extern "C" fn(
    *const c_char,
    *const c_char,
    c_int,
    *const *const c_char,
    *const *const c_char,
    *mut *mut c_void,
    *mut c_uint,
) -> i32;
type NativeCoreClrExecAsmFn = unsafe extern "C" fn(
    *mut c_void,
    c_uint,
    c_int,
    *const *const c_char,
    *const c_char,
    *mut c_uint,
) -> i32;
type NativeCoreClrCreateDelegateFn = unsafe extern "C" fn(
    *mut c_void,
    c_uint,
    *const c_char,
    *const c_char,
    *const c_char,
    *mut *mut c_void,
) -> i32;
type NativeCoreClrShutdownFn = unsafe extern "C" fn(*mut c_void, c_uint) -> i32;
type NativeCoreClrShutdown2Fn = unsafe extern "C" fn(*mut c_void, c_uint, *mut c_int) -> i32;
type NativeCoreClrSetErrorWriterFn = unsafe extern "C" fn(*const c_void) -> *const c_void;

struct NativeCoreClr {
    handle: usize,
    dylib_dir: PathBuf,
    version: String,
    initialize: Option<NativeCoreClrInitFn>,
    execute_assembly: Option<NativeCoreClrExecAsmFn>,
    create_delegate: Option<NativeCoreClrCreateDelegateFn>,
    shutdown: Option<NativeCoreClrShutdownFn>,
    shutdown_2: Option<NativeCoreClrShutdown2Fn>,
    set_error_writer: Option<NativeCoreClrSetErrorWriterFn>,
}

unsafe impl Send for NativeCoreClr {}
unsafe impl Sync for NativeCoreClr {}

unsafe fn resolve_sym<T: Copy>(handle: *mut c_void, name: &CStr) -> Option<T> {
    debug_assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<*mut c_void>());
    let p = libc::dlsym(handle, name.as_ptr());
    if p.is_null() {
        None
    } else {
        Some(std::mem::transmute_copy::<*mut c_void, T>(&p))
    }
}

/// Discover a native host `libcoreclr.dylib` on disk if installed.
pub fn find_host_libcoreclr() -> Option<PathBuf> {
    for env_key in ["PORT_CORECLR_PATH", "AW2_CORECLR_PATH"] {
        if let Some(p) = std::env::var_os(env_key) {
            let pb = PathBuf::from(p);
            if pb.is_file() {
                return Some(pb);
            }
        }
    }
    let mut roots = Vec::new();
    if let Some(root) = std::env::var_os("DOTNET_ROOT") {
        roots.push(PathBuf::from(root).join("shared/Microsoft.NETCore.App"));
    }
    roots.push(PathBuf::from(
        "/usr/local/share/dotnet/shared/Microsoft.NETCore.App",
    ));
    roots.push(PathBuf::from(
        "/opt/homebrew/share/dotnet/shared/Microsoft.NETCore.App",
    ));
    if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".dotnet/shared/Microsoft.NETCore.App"));
    }
    for root in roots {
        let Ok(rd) = std::fs::read_dir(&root) else {
            continue;
        };
        let mut versions: Vec<PathBuf> = rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.join("libcoreclr.dylib").is_file())
            .collect();
        versions.sort();
        if let Some(best) = versions.pop() {
            return Some(best.join("libcoreclr.dylib"));
        }
    }
    None
}

fn native_coreclr() -> Option<&'static NativeCoreClr> {
    static NATIVE: OnceLock<Option<NativeCoreClr>> = OnceLock::new();
    NATIVE
        .get_or_init(|| {
            let dylib = find_host_libcoreclr()?;
            let cpath = CString::new(dylib.to_string_lossy().as_bytes()).ok()?;
            let dylib_dir = dylib
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("/"));
            let version = dylib_dir
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "unknown".to_string());
            unsafe {
                let h = libc::dlopen(cpath.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
                if h.is_null() {
                    log::warn!("coreclr: dlopen({dylib:?}) failed");
                    return None;
                }
                log::info!("coreclr: loaded native {dylib:?} (version={version})");
                Some(NativeCoreClr {
                    handle: h as usize,
                    dylib_dir,
                    version,
                    initialize: resolve_sym(h, c"coreclr_initialize"),
                    execute_assembly: resolve_sym(h, c"coreclr_execute_assembly"),
                    create_delegate: resolve_sym(h, c"coreclr_create_delegate"),
                    shutdown: resolve_sym(h, c"coreclr_shutdown"),
                    shutdown_2: resolve_sym(h, c"coreclr_shutdown_2"),
                    set_error_writer: resolve_sym(h, c"coreclr_set_error_writer"),
                })
            }
        })
        .as_ref()
}

/// Active runtime directory and version (native `libcoreclr.dylib` if installed,
/// otherwise the embedded pure-Rust CLR runtime).
pub(crate) fn runtime_dir_and_version() -> (PathBuf, String) {
    if let Some(rt) = native_coreclr() {
        (rt.dylib_dir.clone(), rt.version.clone())
    } else {
        super::clr::embedded_runtime_dir_and_version()
    }
}

pub(crate) unsafe fn native_dlsym(name: &str) -> Option<*mut c_void> {
    let rt = native_coreclr()?;
    let cname = CString::new(name.as_bytes()).ok()?;
    let p = libc::dlsym(rt.handle as *mut c_void, cname.as_ptr());
    (!p.is_null()).then_some(p)
}

fn translate_win_path(p: &str) -> String {
    super::clr::translate_win_path(p)
        .to_string_lossy()
        .into_owned()
}

fn is_path_list_property(key: &str) -> bool {
    matches!(
        key,
        "TRUSTED_PLATFORM_ASSEMBLIES"
            | "APP_PATHS"
            | "APP_NI_PATHS"
            | "NATIVE_DLL_SEARCH_DIRECTORIES"
            | "PLATFORM_RESOURCE_ROOTS"
    )
}

fn translate_property_value(key: &str, val: &str) -> String {
    if is_path_list_property(key) {
        val.split(';')
            .filter(|s| !s.is_empty())
            .map(translate_win_path)
            .collect::<Vec<_>>()
            .join(":")
    } else if key == "APP_CONTEXT_BASE_DIRECTORY" {
        translate_win_path(val)
    } else {
        val.to_string()
    }
}

fn collect_tpa_list(runtime_dir: &std::path::Path, app_dir: Option<&std::path::Path>) -> String {
    let mut paths: Vec<String> = Vec::new();
    for dir in [Some(runtime_dir), app_dir].into_iter().flatten() {
        if let Ok(rd) = std::fs::read_dir(dir) {
            for entry in rd.filter_map(|e| e.ok()) {
                let p = entry.path();
                if p.extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("dll"))
                {
                    paths.push(p.to_string_lossy().into_owned());
                }
            }
        }
    }
    paths.sort();
    paths.dedup();
    paths.join(":")
}

unsafe extern "C" fn native_error_writer_bridge(msg: *const c_char) {
    let guest_cb = ERROR_WRITER.load(Ordering::Acquire);
    if guest_cb == 0 || msg.is_null() {
        return;
    }
    crate::call_guest8(guest_cb, msg as u64, 0, 0, 0, 0, 0, 0, 0);
}

/// Helper invoked by `mscoree!_CorExeMain` / `_CorExeMain2` when launching a pure-IL assembly.
pub(crate) unsafe fn execute_managed_entry(win_exe_path: &str) -> i32 {
    if let Some(rt) = native_coreclr() {
        if let (Some(init_fn), Some(exec_fn)) = (rt.initialize, rt.execute_assembly) {
            let host_exe = translate_win_path(win_exe_path);
            let host_path = PathBuf::from(&host_exe);
            if !host_path.is_file() {
                log::warn!("coreclr::execute_managed_entry: assembly {host_exe:?} not found on disk");
                return super::clr::COR_E_FILENOTFOUND;
            }
            let app_dir = host_path.parent();
            let tpa = collect_tpa_list(&rt.dylib_dir, app_dir);
            let Ok(exe_c) = CString::new(host_exe.as_bytes()) else {
                return E_INVALIDARG;
            };
            let Ok(tpa_v) = CString::new(tpa.as_bytes()) else {
                return E_INVALIDARG;
            };
            let keys = [c"TRUSTED_PLATFORM_ASSEMBLIES".as_ptr()];
            let vals = [tpa_v.as_ptr()];
            let mut handle: *mut c_void = std::ptr::null_mut();
            let mut domain_id: c_uint = 0;
            let hr = init_fn(
                exe_c.as_ptr(),
                c"port_app".as_ptr(),
                keys.len() as c_int,
                keys.as_ptr(),
                vals.as_ptr(),
                &mut handle,
                &mut domain_id,
            );
            if hr != S_OK || handle.is_null() {
                return hr;
            }
            let mut exit_code: c_uint = 0;
            let exec_hr = exec_fn(
                handle,
                domain_id,
                0,
                std::ptr::null(),
                exe_c.as_ptr(),
                &mut exit_code,
            );
            if let Some(shut2) = rt.shutdown_2 {
                let mut latched = 0i32;
                let _ = shut2(handle, domain_id, &mut latched);
            } else if let Some(shut) = rt.shutdown {
                let _ = shut(handle, domain_id);
            }
            return if exec_hr != S_OK {
                exec_hr
            } else {
                exit_code as i32
            };
        }
    }

    let host_path = super::clr::translate_win_path(win_exe_path);
    match super::clr::execute_entry_point_file(&host_path, &[]) {
        Ok(code) => code,
        Err(hr) => hr,
    }
}

/// Helper for `ICLRRuntimeHost::ExecuteInDefaultAppDomain`.
pub(crate) unsafe fn execute_managed_method(
    win_asm_path: &str,
    type_name: &str,
    method_name: &str,
    argument: &str,
    ret_val: *mut u32,
) -> HRESULT {
    if !ret_val.is_null() {
        std::ptr::write_unaligned(ret_val, 0);
    }
    if win_asm_path.is_empty() || type_name.is_empty() || method_name.is_empty() {
        return E_INVALIDARG;
    }

    if let Some(rt) = native_coreclr() {
        if let (Some(init_fn), Some(create_del)) = (rt.initialize, rt.create_delegate) {
            let host_asm = translate_win_path(win_asm_path);
            let host_path = PathBuf::from(&host_asm);
            let asm_name = host_path
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default();
            let tpa = collect_tpa_list(&rt.dylib_dir, host_path.parent());
            let (Ok(exe_c), Ok(asm_c), Ok(ty_c), Ok(meth_c), Ok(tpa_c)) = (
                CString::new(host_asm.as_bytes()),
                CString::new(asm_name.as_bytes()),
                CString::new(type_name.as_bytes()),
                CString::new(method_name.as_bytes()),
                CString::new(tpa.as_bytes()),
            ) else {
                return E_INVALIDARG;
            };
            let keys = [c"TRUSTED_PLATFORM_ASSEMBLIES".as_ptr()];
            let vals = [tpa_c.as_ptr()];
            let mut handle: *mut c_void = std::ptr::null_mut();
            let mut domain_id: c_uint = 0;
            let hr = init_fn(
                exe_c.as_ptr(),
                c"DefaultDomain".as_ptr(),
                keys.len() as c_int,
                keys.as_ptr(),
                vals.as_ptr(),
                &mut handle,
                &mut domain_id,
            );
            if hr != S_OK || handle.is_null() {
                return hr;
            }
            let mut del: *mut c_void = std::ptr::null_mut();
            let del_hr = create_del(
                handle,
                domain_id,
                asm_c.as_ptr(),
                ty_c.as_ptr(),
                meth_c.as_ptr(),
                &mut del,
            );
            if del_hr != S_OK || del.is_null() {
                if let Some(shut) = rt.shutdown {
                    let _ = shut(handle, domain_id);
                }
                return if del_hr == S_OK { E_FAIL } else { del_hr };
            }
            let arg_c = CString::new(argument.as_bytes()).unwrap_or_default();
            let entry: unsafe extern "C" fn(*const c_char) -> i32 = std::mem::transmute(del);
            let rc = entry(arg_c.as_ptr());
            if !ret_val.is_null() {
                std::ptr::write_unaligned(ret_val, rc as u32);
            }
            if let Some(shut) = rt.shutdown {
                let _ = shut(handle, domain_id);
            }
            return S_OK;
        }
    }

    let host_path = super::clr::translate_win_path(win_asm_path);
    match super::clr::execute_default_app_domain_method(&host_path, type_name, method_name, argument)
    {
        Ok(rc) => {
            if !ret_val.is_null() {
                std::ptr::write_unaligned(ret_val, rc);
            }
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe fn initialize_with_rt(
    rt: Option<&NativeCoreClr>,
    exe_path: LPCSTR,
    app_domain_friendly_name: LPCSTR,
    property_count: c_int,
    property_keys: *const LPCSTR,
    property_values: *const LPCSTR,
    host_handle: *mut *mut c_void,
    domain_id: *mut c_uint,
) -> HRESULT {
    if host_handle.is_null() || domain_id.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(host_handle, std::ptr::null_mut());
    std::ptr::write_unaligned(domain_id, 0);

    let exe_s = crate::strings::from_ansi_opt(exe_path).unwrap_or_default();
    let dom_s = crate::strings::from_ansi_opt(app_domain_friendly_name).unwrap_or_default();

    if let Some(native_fn) = rt.and_then(|r| r.initialize) {
        let host_exe = translate_win_path(&exe_s);
        let Ok(exe_c) = CString::new(host_exe.as_bytes()) else {
            return E_INVALIDARG;
        };
        let Ok(dom_c) = CString::new(dom_s.as_bytes()) else {
            return E_INVALIDARG;
        };
        let mut k_buf: Vec<CString> = Vec::new();
        let mut v_buf: Vec<CString> = Vec::new();
        if property_count > 0 && !property_keys.is_null() && !property_values.is_null() {
            for i in 0..(property_count as usize) {
                let kp = std::ptr::read_unaligned(property_keys.add(i));
                let vp = std::ptr::read_unaligned(property_values.add(i));
                let k = crate::strings::from_ansi_opt(kp).unwrap_or_default();
                let v = translate_property_value(
                    &k,
                    &crate::strings::from_ansi_opt(vp).unwrap_or_default(),
                );
                k_buf.push(CString::new(k.as_bytes()).unwrap_or_default());
                v_buf.push(CString::new(v.as_bytes()).unwrap_or_default());
            }
        }
        let k_ptrs: Vec<*const c_char> = k_buf.iter().map(|c| c.as_ptr()).collect();
        let v_ptrs: Vec<*const c_char> = v_buf.iter().map(|c| c.as_ptr()).collect();
        let mut out_handle: *mut c_void = std::ptr::null_mut();
        let mut out_dom: c_uint = 0;
        let hr = native_fn(
            exe_c.as_ptr(),
            dom_c.as_ptr(),
            k_ptrs.len() as c_int,
            k_ptrs.as_ptr(),
            v_ptrs.as_ptr(),
            &mut out_handle,
            &mut out_dom,
        );
        std::ptr::write_unaligned(host_handle, out_handle);
        std::ptr::write_unaligned(domain_id, out_dom);
        return hr;
    }

    let mut props = Vec::new();
    if property_count > 0 && !property_keys.is_null() && !property_values.is_null() {
        for i in 0..(property_count as usize) {
            let kp = std::ptr::read_unaligned(property_keys.add(i));
            let vp = std::ptr::read_unaligned(property_values.add(i));
            let k = crate::strings::from_ansi_opt(kp).unwrap_or_default();
            let v = crate::strings::from_ansi_opt(vp).unwrap_or_default();
            props.push((k, v));
        }
    }
    match super::clr::coreclr_initialize(&exe_s, &dom_s, &props) {
        Ok((h, dom)) => {
            std::ptr::write_unaligned(host_handle, h as *mut c_void);
            std::ptr::write_unaligned(domain_id, dom);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe fn execute_assembly_with_rt(
    rt: Option<&NativeCoreClr>,
    host_handle: *mut c_void,
    domain_id: c_uint,
    argc: c_int,
    argv: *const LPCSTR,
    managed_assembly_path: LPCSTR,
    exit_code: *mut c_uint,
) -> HRESULT {
    if !exit_code.is_null() {
        std::ptr::write_unaligned(exit_code, 0);
    }
    if host_handle.is_null() || managed_assembly_path.is_null() {
        return E_INVALIDARG;
    }
    let asm_raw = crate::strings::from_ansi(managed_assembly_path);

    if super::clr::coreclr_is_embedded_handle(host_handle as usize) {
        let asm_pb = super::clr::translate_win_path(&asm_raw);
        let mut args = Vec::new();
        if argc > 0 && !argv.is_null() {
            for i in 0..(argc as usize) {
                let ap = std::ptr::read_unaligned(argv.add(i));
                args.push(crate::strings::from_ansi_opt(ap).unwrap_or_default());
            }
        }
        return match super::clr::coreclr_execute_assembly(
            host_handle as usize,
            domain_id,
            &asm_pb,
            &args,
        ) {
            Ok(code) => {
                if !exit_code.is_null() {
                    std::ptr::write_unaligned(exit_code, code);
                }
                S_OK
            }
            Err(hr) => hr,
        };
    }

    let Some(native_fn) = rt.and_then(|r| r.execute_assembly) else {
        return E_INVALIDARG;
    };
    let asm_s = translate_win_path(&asm_raw);
    let Ok(asm_c) = CString::new(asm_s.as_bytes()) else {
        return E_INVALIDARG;
    };
    let mut c_args: Vec<CString> = Vec::new();
    if argc > 0 && !argv.is_null() {
        for i in 0..(argc as usize) {
            let ap = std::ptr::read_unaligned(argv.add(i));
            let s = crate::strings::from_ansi_opt(ap).unwrap_or_default();
            c_args.push(CString::new(s.as_bytes()).unwrap_or_default());
        }
    }
    let ptrs: Vec<*const c_char> = c_args.iter().map(|c| c.as_ptr()).collect();
    let mut out_code: c_uint = 0;
    let hr = native_fn(
        host_handle,
        domain_id,
        ptrs.len() as c_int,
        ptrs.as_ptr(),
        asm_c.as_ptr(),
        &mut out_code,
    );
    if !exit_code.is_null() {
        std::ptr::write_unaligned(exit_code, out_code);
    }
    hr
}

unsafe fn create_delegate_with_rt(
    rt: Option<&NativeCoreClr>,
    host_handle: *mut c_void,
    domain_id: c_uint,
    entry_point_assembly_name: LPCSTR,
    entry_point_type_name: LPCSTR,
    entry_point_method_name: LPCSTR,
    delegate: *mut *mut c_void,
) -> HRESULT {
    if delegate.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(delegate, std::ptr::null_mut());
    if host_handle.is_null() {
        return E_INVALIDARG;
    }

    if super::clr::coreclr_is_embedded_handle(host_handle as usize) {
        let asm = crate::strings::from_ansi_opt(entry_point_assembly_name).unwrap_or_default();
        let ty = crate::strings::from_ansi_opt(entry_point_type_name).unwrap_or_default();
        let meth = crate::strings::from_ansi_opt(entry_point_method_name).unwrap_or_default();
        return match super::clr::coreclr_create_delegate(
            host_handle as usize,
            domain_id,
            &asm,
            &ty,
            &meth,
        ) {
            Ok(fp) => {
                std::ptr::write_unaligned(delegate, fp as *mut c_void);
                S_OK
            }
            Err(hr) => hr,
        };
    }

    let Some(native_fn) = rt.and_then(|r| r.create_delegate) else {
        return E_INVALIDARG;
    };
    let mut out_del: *mut c_void = std::ptr::null_mut();
    let hr = native_fn(
        host_handle,
        domain_id,
        entry_point_assembly_name as *const c_char,
        entry_point_type_name as *const c_char,
        entry_point_method_name as *const c_char,
        &mut out_del,
    );
    if hr == S_OK && !out_del.is_null() {
        crate::register_native_va(out_del as u64, out_del as usize);
        std::ptr::write_unaligned(delegate, out_del);
    }
    hr
}

unsafe fn shutdown_with_rt(
    rt: Option<&NativeCoreClr>,
    host_handle: *mut c_void,
    domain_id: c_uint,
) -> HRESULT {
    if host_handle.is_null() {
        return E_INVALIDARG;
    }
    if super::clr::coreclr_is_embedded_handle(host_handle as usize) {
        return match super::clr::coreclr_shutdown(host_handle as usize, domain_id) {
            Ok(_) => S_OK,
            Err(hr) => hr,
        };
    }
    let Some(native_fn) = rt.and_then(|r| r.shutdown) else {
        return E_INVALIDARG;
    };
    native_fn(host_handle, domain_id)
}

unsafe fn shutdown_2_with_rt(
    rt: Option<&NativeCoreClr>,
    host_handle: *mut c_void,
    domain_id: c_uint,
    latched_exit_code: *mut c_int,
) -> HRESULT {
    if !latched_exit_code.is_null() {
        std::ptr::write_unaligned(latched_exit_code, 0);
    }
    if host_handle.is_null() {
        return E_INVALIDARG;
    }
    if super::clr::coreclr_is_embedded_handle(host_handle as usize) {
        return match super::clr::coreclr_shutdown(host_handle as usize, domain_id) {
            Ok(latched) => {
                if !latched_exit_code.is_null() {
                    std::ptr::write_unaligned(latched_exit_code, latched);
                }
                S_OK
            }
            Err(hr) => hr,
        };
    }
    let Some(native_fn) = rt.and_then(|r| r.shutdown_2) else {
        return E_INVALIDARG;
    };
    let mut latched = 0i32;
    let hr = native_fn(host_handle, domain_id, &mut latched);
    if !latched_exit_code.is_null() {
        std::ptr::write_unaligned(latched_exit_code, latched);
    }
    hr
}

// ---------------------------------------------------------------------------
// Exported Win64 coreclr.dll entrypoints
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn coreclr_initialize(
    exe_path: LPCSTR,
    app_domain_friendly_name: LPCSTR,
    property_count: c_int,
    property_keys: *const LPCSTR,
    property_values: *const LPCSTR,
    host_handle: *mut *mut c_void,
    domain_id: *mut c_uint,
) -> HRESULT {
    initialize_with_rt(
        native_coreclr(),
        exe_path,
        app_domain_friendly_name,
        property_count,
        property_keys,
        property_values,
        host_handle,
        domain_id,
    )
}

pub unsafe extern "win64" fn coreclr_execute_assembly(
    host_handle: *mut c_void,
    domain_id: c_uint,
    argc: c_int,
    argv: *const LPCSTR,
    managed_assembly_path: LPCSTR,
    exit_code: *mut c_uint,
) -> HRESULT {
    execute_assembly_with_rt(
        native_coreclr(),
        host_handle,
        domain_id,
        argc,
        argv,
        managed_assembly_path,
        exit_code,
    )
}

pub unsafe extern "win64" fn coreclr_create_delegate(
    host_handle: *mut c_void,
    domain_id: c_uint,
    entry_point_assembly_name: LPCSTR,
    entry_point_type_name: LPCSTR,
    entry_point_method_name: LPCSTR,
    delegate: *mut *mut c_void,
) -> HRESULT {
    create_delegate_with_rt(
        native_coreclr(),
        host_handle,
        domain_id,
        entry_point_assembly_name,
        entry_point_type_name,
        entry_point_method_name,
        delegate,
    )
}

pub unsafe extern "win64" fn coreclr_shutdown(
    host_handle: *mut c_void,
    domain_id: c_uint,
) -> HRESULT {
    shutdown_with_rt(native_coreclr(), host_handle, domain_id)
}

pub unsafe extern "win64" fn coreclr_shutdown_2(
    host_handle: *mut c_void,
    domain_id: c_uint,
    latched_exit_code: *mut c_int,
) -> HRESULT {
    shutdown_2_with_rt(native_coreclr(), host_handle, domain_id, latched_exit_code)
}

pub unsafe extern "win64" fn coreclr_set_error_writer(
    error_writer: *const c_void,
) -> *const c_void {
    let prev = ERROR_WRITER.swap(error_writer as usize, Ordering::AcqRel);
    if let Some(native_fn) = native_coreclr().and_then(|r| r.set_error_writer) {
        let bridge = if error_writer.is_null() {
            std::ptr::null()
        } else {
            native_error_writer_bridge as *const c_void
        };
        let _ = native_fn(bridge);
    }
    prev as *const c_void
}

pub unsafe extern "win64" fn GetCLRRuntimeHost(
    _riid: *const u8,
    pp_unk: *mut *mut c_void,
) -> HRESULT {
    if pp_unk.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(pp_unk, super::mscoree::get_clr_runtime_host_instance());
    S_OK
}

#[cfg(test)]
mod tests {
    use super::super::clr::metadata::elem_type;
    use super::super::clr::metadata::test_asm::{TestAsmBuilder, TestMethodBody};
    use super::*;

    #[test]
    fn coreclr_embedded_end_to_end_exec_delegate_and_errors() {
        let dir = std::env::temp_dir().join(format!(
            "port_clr_coreclr_test_{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let dll_path = dir.join("CoreClrDemo.dll");

        let mut b = TestAsmBuilder::new("CoreClrDemo");
        let corlib = b.add_assembly_ref("mscorlib");
        let obj_ref = b.add_type_ref(corlib, "System", "Object");
        let _td = b.add_type_def("CoreClrNs", "Runner", Some(obj_ref));
        // Main() -> 91
        let m_main = b.add_method(
            0x0016,
            0,
            "Main",
            &[0x00, 0x00, elem_type::I4],
            Some(TestMethodBody::tiny(vec![0x1F, 91, 0x2A])),
        );
        b.entry_point_method_rid = Some(m_main & 0x00FF_FFFF);
        // MulAdd(int x, int y, int z) -> x * y + z
        let _m_ma = b.add_method(
            0x0016,
            0,
            "MulAdd",
            &[0x00, 0x03, elem_type::I4, elem_type::I4, elem_type::I4, elem_type::I4],
            Some(TestMethodBody::tiny(vec![0x02, 0x03, 0x5A, 0x04, 0x58, 0x2A])),
        );
        std::fs::write(&dll_path, b.build()).unwrap();

        unsafe {
            let mut handle: *mut c_void = std::ptr::null_mut();
            let mut domain: c_uint = 0;

            assert_eq!(
                initialize_with_rt(
                    None,
                    b"App.exe\0".as_ptr(),
                    b"Demo\0".as_ptr(),
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    &mut domain,
                ),
                E_INVALIDARG
            );

            let tpa_k = CString::new("TRUSTED_PLATFORM_ASSEMBLIES").unwrap();
            let tpa_v = CString::new(dll_path.to_string_lossy().as_bytes()).unwrap();
            let keys = [tpa_k.as_ptr() as LPCSTR];
            let vals = [tpa_v.as_ptr() as LPCSTR];

            assert_eq!(
                initialize_with_rt(
                    None,
                    tpa_v.as_ptr() as LPCSTR,
                    b"CoreClrTestDomain\0".as_ptr(),
                    1,
                    keys.as_ptr(),
                    vals.as_ptr(),
                    &mut handle,
                    &mut domain,
                ),
                S_OK
            );
            assert!(!handle.is_null());
            assert!(domain > 0);

            // Missing assembly returns honest COR_E_FILENOTFOUND
            let mut exit_code: c_uint = 999;
            assert_eq!(
                execute_assembly_with_rt(
                    None,
                    handle,
                    domain,
                    0,
                    std::ptr::null(),
                    b"/nonexistent/Missing.dll\0".as_ptr(),
                    &mut exit_code,
                ),
                super::super::clr::COR_E_FILENOTFOUND
            );
            assert_eq!(exit_code, 0);

            // Real assembly executes Main() -> 91
            assert_eq!(
                execute_assembly_with_rt(
                    None,
                    handle,
                    domain,
                    0,
                    std::ptr::null(),
                    tpa_v.as_ptr() as LPCSTR,
                    &mut exit_code,
                ),
                S_OK
            );
            assert_eq!(exit_code, 91);

            // Create real callable delegate into CIL MulAdd
            let mut del: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                create_delegate_with_rt(
                    None,
                    handle,
                    domain,
                    b"CoreClrDemo\0".as_ptr(),
                    b"CoreClrNs.Runner\0".as_ptr(),
                    b"MulAdd\0".as_ptr(),
                    &mut del,
                ),
                S_OK
            );
            assert!(!del.is_null());
            let mul_add: unsafe extern "win64" fn(i32, i32, i32) -> i32 = std::mem::transmute(del);
            assert_eq!(mul_add(6, 7, 5), 47);

            // Missing type returns honest COR_E_TYPELOAD
            let mut bad_del: *mut c_void = 0x1usize as *mut c_void;
            assert_eq!(
                create_delegate_with_rt(
                    None,
                    handle,
                    domain,
                    b"CoreClrDemo\0".as_ptr(),
                    b"CoreClrNs.Missing\0".as_ptr(),
                    b"MulAdd\0".as_ptr(),
                    &mut bad_del,
                ),
                super::super::clr::COR_E_TYPELOAD
            );
            assert!(bad_del.is_null());

            // Shutdown_2 reports latched exit code 91
            let mut latched = 0i32;
            assert_eq!(shutdown_2_with_rt(None, handle, domain, &mut latched), S_OK);
            assert_eq!(latched, 91);

            // GetCLRRuntimeHost always succeeds
            let mut unk: *mut c_void = std::ptr::null_mut();
            assert_eq!(GetCLRRuntimeHost(std::ptr::null(), &mut unk), S_OK);
            assert!(!unk.is_null());
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
