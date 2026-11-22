//! `mono_2_0_bdwgc.dll` (`mono-2.0-bdwgc.dll`, `mono.dll`, `mono-2.0-sgen.dll`):
//! Unity / Mono Embedding Runtime Bridge.
//!
//! Prefers a native host Mono dylib (`libmonobdwgc-2.0.dylib`, `libmonosgen-2.0.dylib`,
//! or `libmono-2.0.dylib`) via `dlopen` when installed on macOS.
//!
//! When no native host Mono runtime is installed (clean macOS or iOS), runs directly
//! on the embedded pure-Rust ECMA-335 CLR/CIL runtime (`crate::misc::clr`) with
//! ABI-compatible `MonoObject`/`MonoString`/`MonoArray`/`GCHandle` objects, real CIL
//! bytecode execution, fields, properties, method descriptors, and internal calls —
//! while still returning honest error codes for missing files or nonexistent types/methods.

use crate::types::*;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

/// `MonoImageOpenStatus::MONO_IMAGE_ERROR_ERRNO` (`mono/metadata/image.h`).
const MONO_IMAGE_ERROR_ERRNO: i32 = 1;

static UNITY_VPRINTF_FN: AtomicUsize = AtomicUsize::new(0);

type FnSetDirs = unsafe extern "C" fn(*const c_char, *const c_char);
type FnSetAsmPath = unsafe extern "C" fn(*const c_char);
type FnConfigParse = unsafe extern "C" fn(*const c_char);
type FnJitInit = unsafe extern "C" fn(*const c_char) -> *mut c_void;
type FnJitInitVersion = unsafe extern "C" fn(*const c_char, *const c_char) -> *mut c_void;
type FnJitCleanup = unsafe extern "C" fn(*mut c_void);
type FnJitExec =
    unsafe extern "C" fn(*mut c_void, *mut c_void, c_int, *const *mut c_char) -> c_int;
type FnGetPtr0 = unsafe extern "C" fn() -> *mut c_void;
type FnDomainSet = unsafe extern "C" fn(*mut c_void, c_int) -> c_int;
type FnPtrToPtr = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
type FnVoidPtr = unsafe extern "C" fn(*mut c_void);
type FnDomainAsmOpen = unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void;
type FnAsmOpen = unsafe extern "C" fn(*const c_char, *mut i32) -> *mut c_void;
type FnImageGetName = unsafe extern "C" fn(*mut c_void) -> *const c_char;
type FnClassFromName =
    unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_void;
type FnClassGetMethod = unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> *mut c_void;
type FnClassGetMember = unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void;
type FnMethodDescNew = unsafe extern "C" fn(*const c_char, c_int) -> *mut c_void;
type FnPtr2ToPtr = unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void;
type FnRuntimeInvoke =
    unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *mut c_void, *mut *mut c_void) -> *mut c_void;
type FnAddInternalCall = unsafe extern "C" fn(*const c_char, *const c_void);
type FnStringNew = unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void;
type FnStringNewLen = unsafe extern "C" fn(*mut c_void, *const c_char, u32) -> *mut c_void;
type FnStringNewUtf16 = unsafe extern "C" fn(*mut c_void, *const u16, i32) -> *mut c_void;
type FnStringToUtf8 = unsafe extern "C" fn(*mut c_void) -> *mut c_char;
type FnStringChars = unsafe extern "C" fn(*mut c_void) -> *mut u16;
type FnStringLen = unsafe extern "C" fn(*mut c_void) -> c_int;
type FnFieldVal = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void);
type FnValueBox = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> *mut c_void;
type FnArrayNew = unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> *mut c_void;
type FnArrayLen = unsafe extern "C" fn(*mut c_void) -> usize;
type FnArrayAddr = unsafe extern "C" fn(*mut c_void, c_int, usize) -> *mut u8;
type FnArrayElemSize = unsafe extern "C" fn(*mut c_void) -> i32;
type FnGcHandleNew = unsafe extern "C" fn(*mut c_void, c_int) -> u32;
type FnGcHandleTarget = unsafe extern "C" fn(u32) -> *mut c_void;
type FnGcHandleFree = unsafe extern "C" fn(u32);
type FnIterMember = unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> *mut c_void;
type FnImageOpenData =
    unsafe extern "C" fn(*const u8, u32, c_int, *mut i32, c_int) -> *mut c_void;
type FnAsmLoadFrom =
    unsafe extern "C" fn(*mut c_void, *const c_char, *mut i32, c_int) -> *mut c_void;
type FnSetVprintf = unsafe extern "C" fn(*const c_void);

struct NativeMono {
    _handle: usize,
    mono_set_dirs: Option<FnSetDirs>,
    mono_set_assemblies_path: Option<FnSetAsmPath>,
    mono_config_parse: Option<FnConfigParse>,
    mono_jit_init: Option<FnJitInit>,
    mono_jit_init_version: Option<FnJitInitVersion>,
    mono_jit_cleanup: Option<FnJitCleanup>,
    mono_jit_exec: Option<FnJitExec>,
    mono_domain_get: Option<FnGetPtr0>,
    mono_get_root_domain: Option<FnGetPtr0>,
    mono_domain_set: Option<FnDomainSet>,
    mono_thread_attach: Option<FnPtrToPtr>,
    mono_thread_detach: Option<FnVoidPtr>,
    mono_thread_current: Option<FnGetPtr0>,
    mono_domain_assembly_open: Option<FnDomainAsmOpen>,
    mono_assembly_open: Option<FnAsmOpen>,
    mono_assembly_get_image: Option<FnPtrToPtr>,
    mono_get_corlib: Option<FnGetPtr0>,
    mono_image_get_name: Option<FnImageGetName>,
    mono_image_open_from_data_full: Option<FnImageOpenData>,
    mono_assembly_load_from_full: Option<FnAsmLoadFrom>,
    mono_class_from_name: Option<FnClassFromName>,
    mono_class_get_name: Option<FnImageGetName>,
    mono_class_get_namespace: Option<FnImageGetName>,
    mono_class_vtable: Option<FnPtr2ToPtr>,
    mono_class_get_method_from_name: Option<FnClassGetMethod>,
    mono_class_get_field_from_name: Option<FnClassGetMember>,
    mono_class_get_property_from_name: Option<FnClassGetMember>,
    mono_class_get_methods: Option<FnIterMember>,
    mono_class_get_fields: Option<FnIterMember>,
    mono_class_get_properties: Option<FnIterMember>,
    mono_method_get_name: Option<FnImageGetName>,
    mono_field_get_name: Option<FnImageGetName>,
    mono_property_get_name: Option<FnImageGetName>,
    mono_method_desc_new: Option<FnMethodDescNew>,
    mono_method_desc_search_in_class: Option<FnPtr2ToPtr>,
    mono_method_desc_search_in_image: Option<FnPtr2ToPtr>,
    mono_method_desc_free: Option<FnVoidPtr>,
    mono_runtime_invoke: Option<FnRuntimeInvoke>,
    mono_add_internal_call: Option<FnAddInternalCall>,
    mono_string_new: Option<FnStringNew>,
    mono_string_new_len: Option<FnStringNewLen>,
    mono_string_new_utf16: Option<FnStringNewUtf16>,
    mono_string_to_utf8: Option<FnStringToUtf8>,
    mono_string_chars: Option<FnStringChars>,
    mono_string_length: Option<FnStringLen>,
    mono_free: Option<FnVoidPtr>,
    mono_object_new: Option<FnPtr2ToPtr>,
    mono_runtime_object_init: Option<FnVoidPtr>,
    mono_runtime_class_init: Option<FnVoidPtr>,
    mono_object_unbox: Option<FnPtrToPtr>,
    mono_object_get_class: Option<FnPtrToPtr>,
    mono_value_box: Option<FnValueBox>,
    mono_field_get_value: Option<FnFieldVal>,
    mono_field_set_value: Option<FnFieldVal>,
    mono_field_static_get_value: Option<FnFieldVal>,
    mono_field_static_set_value: Option<FnFieldVal>,
    mono_property_get_get_method: Option<FnPtrToPtr>,
    mono_property_get_set_method: Option<FnPtrToPtr>,
    mono_array_new: Option<FnArrayNew>,
    mono_array_length: Option<FnArrayLen>,
    mono_array_addr_with_size: Option<FnArrayAddr>,
    mono_array_element_size: Option<FnArrayElemSize>,
    mono_gchandle_new: Option<FnGcHandleNew>,
    mono_gchandle_new_weakref: Option<FnGcHandleNew>,
    mono_gchandle_get_target: Option<FnGcHandleTarget>,
    mono_gchandle_free: Option<FnGcHandleFree>,
    mono_get_object_class: Option<FnGetPtr0>,
    mono_get_int32_class: Option<FnGetPtr0>,
    mono_get_uint32_class: Option<FnGetPtr0>,
    mono_get_int64_class: Option<FnGetPtr0>,
    mono_get_byte_class: Option<FnGetPtr0>,
    mono_get_boolean_class: Option<FnGetPtr0>,
    mono_get_single_class: Option<FnGetPtr0>,
    mono_get_double_class: Option<FnGetPtr0>,
    mono_get_string_class: Option<FnGetPtr0>,
    mono_unity_set_vprintf_func: Option<FnSetVprintf>,
}

unsafe impl Send for NativeMono {}
unsafe impl Sync for NativeMono {}

unsafe fn resolve_sym<T: Copy>(handle: *mut c_void, name: &CStr) -> Option<T> {
    debug_assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<*mut c_void>());
    let p = libc::dlsym(handle, name.as_ptr());
    if p.is_null() {
        None
    } else {
        Some(std::mem::transmute_copy::<*mut c_void, T>(&p))
    }
}

/// Discover a native host Mono dylib (`libmonobdwgc-2.0.dylib`, `libmonosgen-2.0.dylib`,
/// or `libmono-2.0.dylib`) if installed on disk.
pub fn find_host_libmono() -> Option<PathBuf> {
    for env_key in ["PORT_MONO_PATH", "AW2_MONO_PATH"] {
        if let Some(p) = std::env::var_os(env_key) {
            let pb = PathBuf::from(p);
            if pb.is_file() {
                return Some(pb);
            }
        }
    }
    for candidate in [
        "/opt/homebrew/lib/libmonobdwgc-2.0.dylib",
        "/opt/homebrew/lib/libmonosgen-2.0.dylib",
        "/opt/homebrew/lib/libmono-2.0.dylib",
        "/usr/local/lib/libmonobdwgc-2.0.dylib",
        "/usr/local/lib/libmonosgen-2.0.dylib",
        "/usr/local/lib/libmono-2.0.dylib",
        "/Library/Frameworks/Mono.framework/Versions/Current/lib/libmonobdwgc-2.0.dylib",
        "/Library/Frameworks/Mono.framework/Versions/Current/lib/libmonosgen-2.0.dylib",
        "/Library/Frameworks/Mono.framework/Versions/Current/lib/libmono-2.0.dylib",
    ] {
        let p = PathBuf::from(candidate);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn native_mono() -> Option<&'static NativeMono> {
    static NATIVE: OnceLock<Option<NativeMono>> = OnceLock::new();
    NATIVE
        .get_or_init(|| {
            let dylib = find_host_libmono()?;
            let cpath = CString::new(dylib.to_string_lossy().as_bytes()).ok()?;
            unsafe {
                let h = libc::dlopen(cpath.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL);
                if h.is_null() {
                    log::warn!("mono: dlopen({dylib:?}) failed");
                    return None;
                }
                log::info!("mono: loaded native {dylib:?}");
                Some(NativeMono {
                    _handle: h as usize,
                    mono_set_dirs: resolve_sym(h, c"mono_set_dirs"),
                    mono_set_assemblies_path: resolve_sym(h, c"mono_set_assemblies_path"),
                    mono_config_parse: resolve_sym(h, c"mono_config_parse"),
                    mono_jit_init: resolve_sym(h, c"mono_jit_init"),
                    mono_jit_init_version: resolve_sym(h, c"mono_jit_init_version"),
                    mono_jit_cleanup: resolve_sym(h, c"mono_jit_cleanup"),
                    mono_jit_exec: resolve_sym(h, c"mono_jit_exec"),
                    mono_domain_get: resolve_sym(h, c"mono_domain_get"),
                    mono_get_root_domain: resolve_sym(h, c"mono_get_root_domain"),
                    mono_domain_set: resolve_sym(h, c"mono_domain_set"),
                    mono_thread_attach: resolve_sym(h, c"mono_thread_attach"),
                    mono_thread_detach: resolve_sym(h, c"mono_thread_detach"),
                    mono_thread_current: resolve_sym(h, c"mono_thread_current"),
                    mono_domain_assembly_open: resolve_sym(h, c"mono_domain_assembly_open"),
                    mono_assembly_open: resolve_sym(h, c"mono_assembly_open"),
                    mono_assembly_get_image: resolve_sym(h, c"mono_assembly_get_image"),
                    mono_get_corlib: resolve_sym(h, c"mono_get_corlib"),
                    mono_image_get_name: resolve_sym(h, c"mono_image_get_name"),
                    mono_image_open_from_data_full: resolve_sym(h, c"mono_image_open_from_data_full"),
                    mono_assembly_load_from_full: resolve_sym(h, c"mono_assembly_load_from_full"),
                    mono_class_from_name: resolve_sym(h, c"mono_class_from_name"),
                    mono_class_get_name: resolve_sym(h, c"mono_class_get_name"),
                    mono_class_get_namespace: resolve_sym(h, c"mono_class_get_namespace"),
                    mono_class_vtable: resolve_sym(h, c"mono_class_vtable"),
                    mono_class_get_method_from_name: resolve_sym(
                        h,
                        c"mono_class_get_method_from_name",
                    ),
                    mono_class_get_field_from_name: resolve_sym(
                        h,
                        c"mono_class_get_field_from_name",
                    ),
                    mono_class_get_property_from_name: resolve_sym(
                        h,
                        c"mono_class_get_property_from_name",
                    ),
                    mono_class_get_methods: resolve_sym(h, c"mono_class_get_methods"),
                    mono_class_get_fields: resolve_sym(h, c"mono_class_get_fields"),
                    mono_class_get_properties: resolve_sym(h, c"mono_class_get_properties"),
                    mono_method_get_name: resolve_sym(h, c"mono_method_get_name"),
                    mono_field_get_name: resolve_sym(h, c"mono_field_get_name"),
                    mono_property_get_name: resolve_sym(h, c"mono_property_get_name"),
                    mono_method_desc_new: resolve_sym(h, c"mono_method_desc_new"),
                    mono_method_desc_search_in_class: resolve_sym(
                        h,
                        c"mono_method_desc_search_in_class",
                    ),
                    mono_method_desc_search_in_image: resolve_sym(
                        h,
                        c"mono_method_desc_search_in_image",
                    ),
                    mono_method_desc_free: resolve_sym(h, c"mono_method_desc_free"),
                    mono_runtime_invoke: resolve_sym(h, c"mono_runtime_invoke"),
                    mono_add_internal_call: resolve_sym(h, c"mono_add_internal_call"),
                    mono_string_new: resolve_sym(h, c"mono_string_new"),
                    mono_string_new_len: resolve_sym(h, c"mono_string_new_len"),
                    mono_string_new_utf16: resolve_sym(h, c"mono_string_new_utf16"),
                    mono_string_to_utf8: resolve_sym(h, c"mono_string_to_utf8"),
                    mono_string_chars: resolve_sym(h, c"mono_string_chars"),
                    mono_string_length: resolve_sym(h, c"mono_string_length"),
                    mono_free: resolve_sym(h, c"mono_free")
                        .or_else(|| resolve_sym(h, c"g_free")),
                    mono_object_new: resolve_sym(h, c"mono_object_new"),
                    mono_runtime_object_init: resolve_sym(h, c"mono_runtime_object_init"),
                    mono_runtime_class_init: resolve_sym(h, c"mono_runtime_class_init"),
                    mono_object_unbox: resolve_sym(h, c"mono_object_unbox"),
                    mono_object_get_class: resolve_sym(h, c"mono_object_get_class"),
                    mono_value_box: resolve_sym(h, c"mono_value_box"),
                    mono_field_get_value: resolve_sym(h, c"mono_field_get_value"),
                    mono_field_set_value: resolve_sym(h, c"mono_field_set_value"),
                    mono_field_static_get_value: resolve_sym(h, c"mono_field_static_get_value"),
                    mono_field_static_set_value: resolve_sym(h, c"mono_field_static_set_value"),
                    mono_property_get_get_method: resolve_sym(h, c"mono_property_get_get_method"),
                    mono_property_get_set_method: resolve_sym(h, c"mono_property_get_set_method"),
                    mono_array_new: resolve_sym(h, c"mono_array_new"),
                    mono_array_length: resolve_sym(h, c"mono_array_length"),
                    mono_array_addr_with_size: resolve_sym(h, c"mono_array_addr_with_size"),
                    mono_array_element_size: resolve_sym(h, c"mono_array_element_size"),
                    mono_gchandle_new: resolve_sym(h, c"mono_gchandle_new"),
                    mono_gchandle_new_weakref: resolve_sym(h, c"mono_gchandle_new_weakref"),
                    mono_gchandle_get_target: resolve_sym(h, c"mono_gchandle_get_target"),
                    mono_gchandle_free: resolve_sym(h, c"mono_gchandle_free"),
                    mono_get_object_class: resolve_sym(h, c"mono_get_object_class"),
                    mono_get_int32_class: resolve_sym(h, c"mono_get_int32_class"),
                    mono_get_uint32_class: resolve_sym(h, c"mono_get_uint32_class"),
                    mono_get_int64_class: resolve_sym(h, c"mono_get_int64_class"),
                    mono_get_byte_class: resolve_sym(h, c"mono_get_byte_class"),
                    mono_get_boolean_class: resolve_sym(h, c"mono_get_boolean_class"),
                    mono_get_single_class: resolve_sym(h, c"mono_get_single_class"),
                    mono_get_double_class: resolve_sym(h, c"mono_get_double_class"),
                    mono_get_string_class: resolve_sym(h, c"mono_get_string_class"),
                    mono_unity_set_vprintf_func: resolve_sym(h, c"mono_unity_set_vprintf_func"),
                })
            }
        })
        .as_ref()
}

fn translate_win_path(p: &str) -> String {
    super::clr::translate_win_path(p)
        .to_string_lossy()
        .into_owned()
}

unsafe fn opt_translated_cstr(p: LPCSTR) -> Option<CString> {
    crate::strings::from_ansi_opt(p)
        .map(|s| translate_win_path(&s))
        .and_then(|s| CString::new(s.as_bytes()).ok())
}

// ---------------------------------------------------------------------------
// Internal helpers allowing unconditional testing of the embedded runtime
// ---------------------------------------------------------------------------

unsafe fn jit_init_with_rt(rt: Option<&NativeMono>, domain_name: LPCSTR) -> *mut c_void {
    if let Some(f) = rt.and_then(|m| m.mono_jit_init) {
        return f(domain_name as *const c_char);
    }
    let name = crate::strings::from_ansi_opt(domain_name).unwrap_or_else(|| "RootDomain".into());
    super::clr::mono_jit_init(&name)
}

unsafe fn assembly_open_with_rt(
    rt: Option<&NativeMono>,
    filename: LPCSTR,
    status: *mut i32,
) -> *mut c_void {
    if let Some(f) = rt.and_then(|m| m.mono_assembly_open) {
        let path_c = opt_translated_cstr(filename);
        let mut out_status = 0i32;
        let res = f(
            path_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            if status.is_null() {
                std::ptr::null_mut()
            } else {
                &mut out_status
            },
        );
        if !status.is_null() {
            std::ptr::write_unaligned(status, out_status);
        }
        return res;
    }

    let Some(fname) = crate::strings::from_ansi_opt(filename) else {
        if !status.is_null() {
            std::ptr::write_unaligned(status, MONO_IMAGE_ERROR_ERRNO);
        }
        return std::ptr::null_mut();
    };
    let pb = super::clr::translate_win_path(&fname);
    match super::clr::mono_assembly_open(&pb) {
        Ok(asm) => {
            if !status.is_null() {
                std::ptr::write_unaligned(status, super::clr::MONO_IMAGE_OK);
            }
            asm
        }
        Err(st) => {
            if !status.is_null() {
                std::ptr::write_unaligned(status, st);
            }
            std::ptr::null_mut()
        }
    }
}

unsafe fn string_new_with_rt(
    rt: Option<&NativeMono>,
    domain: *mut c_void,
    text: LPCSTR,
) -> *mut c_void {
    if let Some(f) = rt.and_then(|m| m.mono_string_new) {
        return f(domain, text as *const c_char);
    }
    if text.is_null() {
        return std::ptr::null_mut();
    }
    let s = crate::strings::from_ansi(text);
    super::clr::mono_string_new(&s)
}

unsafe fn string_to_utf8_with_rt(rt: Option<&NativeMono>, string_obj: *mut c_void) -> *mut c_char {
    if string_obj.is_null() {
        return std::ptr::null_mut();
    }
    if let Some(f) = rt.and_then(|m| m.mono_string_to_utf8) {
        return f(string_obj);
    }
    super::clr::mono_string_to_utf8(string_obj)
}

unsafe fn jit_exec_with_rt(
    rt: Option<&NativeMono>,
    domain: *mut c_void,
    assembly: *mut c_void,
    argc: c_int,
    argv: *const *mut c_char,
) -> c_int {
    if let Some(f) = rt.and_then(|m| m.mono_jit_exec) {
        return f(domain, assembly, argc, argv);
    }
    let mut args = Vec::new();
    if argc > 0 && !argv.is_null() {
        for i in 0..(argc as usize) {
            let p = std::ptr::read_unaligned(argv.add(i));
            if !p.is_null() {
                args.push(CStr::from_ptr(p).to_string_lossy().into_owned());
            }
        }
    }
    super::clr::mono_jit_exec(domain, assembly, &args)
}

unsafe fn domain_set_with_rt(rt: Option<&NativeMono>, domain: *mut c_void, force: BOOL) -> BOOL {
    if let Some(f) = rt.and_then(|m| m.mono_domain_set) {
        return if f(domain, force) != 0 { TRUE } else { FALSE };
    }
    if super::clr::mono_domain_set(domain) {
        TRUE
    } else {
        FALSE
    }
}

// ---------------------------------------------------------------------------
// Exported Win64 mono-2.0-bdwgc.dll entrypoints
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn mono_set_dirs(assembly_dir: LPCSTR, config_dir: LPCSTR) {
    if let Some(f) = native_mono().and_then(|m| m.mono_set_dirs) {
        let asm_c = opt_translated_cstr(assembly_dir);
        let cfg_c = opt_translated_cstr(config_dir);
        f(
            asm_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            cfg_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        );
        return;
    }
    let ad = crate::strings::from_ansi_opt(assembly_dir).map(|s| super::clr::translate_win_path(&s));
    let cd = crate::strings::from_ansi_opt(config_dir).map(|s| super::clr::translate_win_path(&s));
    super::clr::mono_set_dirs(ad.as_deref(), cd.as_deref());
}

pub unsafe extern "win64" fn mono_set_assemblies_path(path: LPCSTR) {
    if let Some(f) = native_mono().and_then(|m| m.mono_set_assemblies_path) {
        let translated = crate::strings::from_ansi_opt(path).map(|p| {
            p.split(';')
                .filter(|s| !s.is_empty())
                .map(translate_win_path)
                .collect::<Vec<_>>()
                .join(":")
        });
        let path_c = translated.and_then(|s| CString::new(s.as_bytes()).ok());
        f(path_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()));
        return;
    }
    if let Some(p) = crate::strings::from_ansi_opt(path) {
        for part in p.split([';', ':']).filter(|s| !s.is_empty()) {
            let pb = super::clr::translate_win_path(part);
            super::clr::mono_set_dirs(Some(&pb), None);
        }
    }
}

pub unsafe extern "win64" fn mono_config_parse(filename: LPCSTR) {
    if let Some(f) = native_mono().and_then(|m| m.mono_config_parse) {
        let path_c = opt_translated_cstr(filename);
        f(path_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()));
    }
}

pub unsafe extern "win64" fn mono_jit_init(domain_name: LPCSTR) -> *mut c_void {
    jit_init_with_rt(native_mono(), domain_name)
}

pub unsafe extern "win64" fn mono_jit_init_version(
    domain_name: LPCSTR,
    runtime_version: LPCSTR,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_jit_init_version) {
        return f(domain_name as *const c_char, runtime_version as *const c_char);
    }
    jit_init_with_rt(None, domain_name)
}

pub unsafe extern "win64" fn mono_jit_cleanup(domain: *mut c_void) {
    if let Some(f) = native_mono().and_then(|m| m.mono_jit_cleanup) {
        if !domain.is_null() {
            f(domain);
        }
    }
}

pub unsafe extern "win64" fn mono_jit_exec(
    domain: *mut c_void,
    assembly: *mut c_void,
    argc: c_int,
    argv: *const *mut c_char,
) -> c_int {
    jit_exec_with_rt(native_mono(), domain, assembly, argc, argv)
}

pub unsafe extern "win64" fn mono_domain_get() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_domain_get) {
        return f();
    }
    super::clr::mono_domain_get()
}

pub unsafe extern "win64" fn mono_get_root_domain() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_root_domain) {
        return f();
    }
    super::clr::mono_get_root_domain()
}

pub unsafe extern "win64" fn mono_domain_set(domain: *mut c_void, force: BOOL) -> BOOL {
    domain_set_with_rt(native_mono(), domain, force)
}

pub unsafe extern "win64" fn mono_thread_attach(domain: *mut c_void) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_thread_attach) {
        return f(domain);
    }
    super::clr::mono_thread_attach(domain)
}

pub unsafe extern "win64" fn mono_thread_detach(thread: *mut c_void) {
    if let Some(f) = native_mono().and_then(|m| m.mono_thread_detach) {
        if !thread.is_null() {
            f(thread);
        }
        return;
    }
    super::clr::mono_thread_detach(thread);
}

pub unsafe extern "win64" fn mono_thread_current() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_thread_current) {
        return f();
    }
    super::clr::mono_thread_current()
}

pub unsafe extern "win64" fn mono_domain_assembly_open(
    domain: *mut c_void,
    name: LPCSTR,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_domain_assembly_open) {
        let path_c = opt_translated_cstr(name);
        return f(
            domain,
            path_c.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
        );
    }
    if domain.is_null() {
        return std::ptr::null_mut();
    }
    assembly_open_with_rt(None, name, std::ptr::null_mut())
}

pub unsafe extern "win64" fn mono_assembly_open(
    filename: LPCSTR,
    status: *mut i32,
) -> *mut c_void {
    assembly_open_with_rt(native_mono(), filename, status)
}

pub unsafe extern "win64" fn mono_image_open_from_data_full(
    data: *const u8,
    data_len: u32,
    need_copy: BOOL,
    status: *mut i32,
    refonly: BOOL,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_image_open_from_data_full) {
        let mut st = 0i32;
        let res = f(
            data,
            data_len,
            need_copy,
            if status.is_null() { std::ptr::null_mut() } else { &mut st },
            refonly,
        );
        if !status.is_null() {
            std::ptr::write_unaligned(status, st);
        }
        return res;
    }
    if data.is_null() || data_len == 0 {
        if !status.is_null() {
            std::ptr::write_unaligned(status, super::clr::MONO_IMAGE_IMAGE_INVALID);
        }
        return std::ptr::null_mut();
    }
    let slice = std::slice::from_raw_parts(data, data_len as usize);
    match super::clr::mono_image_open_from_data(slice) {
        Ok(img) => {
            if !status.is_null() {
                std::ptr::write_unaligned(status, super::clr::MONO_IMAGE_OK);
            }
            img
        }
        Err(st) => {
            if !status.is_null() {
                std::ptr::write_unaligned(status, st);
            }
            std::ptr::null_mut()
        }
    }
}

pub unsafe extern "win64" fn mono_assembly_load_from_full(
    image: *mut c_void,
    fname: LPCSTR,
    status: *mut i32,
    refonly: BOOL,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_assembly_load_from_full) {
        let mut st = 0i32;
        let res = f(
            image,
            fname as *const c_char,
            if status.is_null() { std::ptr::null_mut() } else { &mut st },
            refonly,
        );
        if !status.is_null() {
            std::ptr::write_unaligned(status, st);
        }
        return res;
    }
    let asm = super::clr::mono_assembly_from_image(image);
    if !status.is_null() {
        std::ptr::write_unaligned(
            status,
            if asm.is_null() {
                super::clr::MONO_IMAGE_IMAGE_INVALID
            } else {
                super::clr::MONO_IMAGE_OK
            },
        );
    }
    asm
}

pub unsafe extern "win64" fn mono_assembly_get_image(assembly: *mut c_void) -> *mut c_void {
    if assembly.is_null() {
        return std::ptr::null_mut();
    }
    if let Some(f) = native_mono().and_then(|m| m.mono_assembly_get_image) {
        return f(assembly);
    }
    super::clr::mono_assembly_get_image(assembly)
}

pub unsafe extern "win64" fn mono_get_corlib() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_corlib) {
        return f();
    }
    super::clr::mono_get_corlib()
}

pub unsafe extern "win64" fn mono_image_get_name(image: *mut c_void) -> *const c_char {
    if image.is_null() {
        return std::ptr::null();
    }
    if let Some(f) = native_mono().and_then(|m| m.mono_image_get_name) {
        return f(image);
    }
    super::clr::mono_image_get_name(image)
}

pub unsafe extern "win64" fn mono_class_from_name(
    image: *mut c_void,
    name_space: LPCSTR,
    name: LPCSTR,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_from_name) {
        return f(image, name_space as *const c_char, name as *const c_char);
    }
    let ns = crate::strings::from_ansi_opt(name_space).unwrap_or_default();
    let Some(n) = crate::strings::from_ansi_opt(name) else {
        return std::ptr::null_mut();
    };
    super::clr::mono_class_from_name(image, &ns, &n)
}

pub unsafe extern "win64" fn mono_class_get_name(klass: *mut c_void) -> *const c_char {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_get_name) {
        return f(klass);
    }
    super::clr::mono_class_get_name(klass)
}

pub unsafe extern "win64" fn mono_class_get_namespace(klass: *mut c_void) -> *const c_char {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_get_namespace) {
        return f(klass);
    }
    super::clr::mono_class_get_namespace(klass)
}

pub unsafe extern "win64" fn mono_class_vtable(
    domain: *mut c_void,
    klass: *mut c_void,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_vtable) {
        return f(domain, klass);
    }
    super::clr::mono_class_vtable(klass)
}

pub unsafe extern "win64" fn mono_class_get_method_from_name(
    klass: *mut c_void,
    name: LPCSTR,
    param_count: c_int,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_get_method_from_name) {
        return f(klass, name as *const c_char, param_count);
    }
    let Some(n) = crate::strings::from_ansi_opt(name) else {
        return std::ptr::null_mut();
    };
    super::clr::mono_class_get_method_from_name(klass, &n, param_count)
}

pub unsafe extern "win64" fn mono_class_get_field_from_name(
    klass: *mut c_void,
    name: LPCSTR,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_get_field_from_name) {
        return f(klass, name as *const c_char);
    }
    let Some(n) = crate::strings::from_ansi_opt(name) else {
        return std::ptr::null_mut();
    };
    super::clr::mono_class_get_field_from_name(klass, &n)
}

pub unsafe extern "win64" fn mono_class_get_property_from_name(
    klass: *mut c_void,
    name: LPCSTR,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_get_property_from_name) {
        return f(klass, name as *const c_char);
    }
    let Some(n) = crate::strings::from_ansi_opt(name) else {
        return std::ptr::null_mut();
    };
    super::clr::mono_class_get_property_from_name(klass, &n)
}

pub unsafe extern "win64" fn mono_class_get_methods(
    klass: *mut c_void,
    iter: *mut *mut c_void,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_get_methods) {
        return f(klass, iter);
    }
    super::clr::mono_class_iter_methods(klass, iter)
}

pub unsafe extern "win64" fn mono_class_get_fields(
    klass: *mut c_void,
    iter: *mut *mut c_void,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_get_fields) {
        return f(klass, iter);
    }
    super::clr::mono_class_iter_fields(klass, iter)
}

pub unsafe extern "win64" fn mono_class_get_properties(
    klass: *mut c_void,
    iter: *mut *mut c_void,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_class_get_properties) {
        return f(klass, iter);
    }
    super::clr::mono_class_iter_properties(klass, iter)
}

pub unsafe extern "win64" fn mono_method_get_name(method: *mut c_void) -> *const c_char {
    if let Some(f) = native_mono().and_then(|m| m.mono_method_get_name) {
        return f(method);
    }
    super::clr::mono_method_get_name(method)
}

pub unsafe extern "win64" fn mono_field_get_name(field: *mut c_void) -> *const c_char {
    if let Some(f) = native_mono().and_then(|m| m.mono_field_get_name) {
        return f(field);
    }
    super::clr::mono_field_get_name(field)
}

pub unsafe extern "win64" fn mono_property_get_name(prop: *mut c_void) -> *const c_char {
    if let Some(f) = native_mono().and_then(|m| m.mono_property_get_name) {
        return f(prop);
    }
    super::clr::mono_property_get_name(prop)
}

pub unsafe extern "win64" fn mono_method_desc_new(
    name: LPCSTR,
    include_namespace: BOOL,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_method_desc_new) {
        return f(name as *const c_char, include_namespace);
    }
    let Some(spec) = crate::strings::from_ansi_opt(name) else {
        return std::ptr::null_mut();
    };
    super::clr::mono_method_desc_new(&spec, include_namespace != 0)
}

pub unsafe extern "win64" fn mono_method_desc_search_in_class(
    desc: *mut c_void,
    klass: *mut c_void,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_method_desc_search_in_class) {
        return f(desc, klass);
    }
    super::clr::mono_method_desc_search_in_class(desc, klass)
}

pub unsafe extern "win64" fn mono_method_desc_search_in_image(
    desc: *mut c_void,
    image: *mut c_void,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_method_desc_search_in_image) {
        return f(desc, image);
    }
    super::clr::mono_method_desc_search_in_image(desc, image)
}

pub unsafe extern "win64" fn mono_method_desc_free(desc: *mut c_void) {
    if let Some(f) = native_mono().and_then(|m| m.mono_method_desc_free) {
        if !desc.is_null() {
            f(desc);
        }
        return;
    }
    super::clr::mono_method_desc_free(desc);
}

pub unsafe extern "win64" fn mono_runtime_invoke(
    method: *mut c_void,
    obj: *mut c_void,
    params: *mut *mut c_void,
    exc: *mut *mut c_void,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_runtime_invoke) {
        let mut out_exc: *mut c_void = std::ptr::null_mut();
        let res = f(
            method,
            obj,
            params,
            if exc.is_null() {
                std::ptr::null_mut()
            } else {
                &mut out_exc
            },
        );
        if !exc.is_null() {
            std::ptr::write_unaligned(exc, out_exc);
        }
        return res;
    }
    super::clr::mono_runtime_invoke(method, obj, params, exc)
}

pub unsafe extern "win64" fn mono_add_internal_call(name: LPCSTR, method: *const c_void) {
    if let Some(f) = native_mono().and_then(|m| m.mono_add_internal_call) {
        f(name as *const c_char, method);
        return;
    }
    if let Some(n) = crate::strings::from_ansi_opt(name) {
        super::clr::mono_add_internal_call(&n, method);
    }
}

pub unsafe extern "win64" fn mono_string_new(domain: *mut c_void, text: LPCSTR) -> *mut c_void {
    string_new_with_rt(native_mono(), domain, text)
}

pub unsafe extern "win64" fn mono_string_new_len(
    domain: *mut c_void,
    text: LPCSTR,
    length: u32,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_string_new_len) {
        return f(domain, text as *const c_char, length);
    }
    if text.is_null() {
        return std::ptr::null_mut();
    }
    let slice = std::slice::from_raw_parts(text, length as usize);
    let s = String::from_utf8_lossy(slice);
    super::clr::mono_string_new(&s)
}

pub unsafe extern "win64" fn mono_string_new_utf16(
    domain: *mut c_void,
    text: LPCWSTR,
    len: i32,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_string_new_utf16) {
        return f(domain, text, len);
    }
    if text.is_null() || len < 0 {
        return std::ptr::null_mut();
    }
    let mut u16s = Vec::with_capacity(len as usize);
    for i in 0..(len as usize) {
        u16s.push(std::ptr::read_unaligned(text.add(i)));
    }
    super::clr::mono_string_new_utf16(&u16s)
}

pub unsafe extern "win64" fn mono_string_to_utf8(string_obj: *mut c_void) -> *mut c_char {
    string_to_utf8_with_rt(native_mono(), string_obj)
}

pub unsafe extern "win64" fn mono_string_chars(string_obj: *mut c_void) -> *mut u16 {
    if let Some(f) = native_mono().and_then(|m| m.mono_string_chars) {
        return f(string_obj);
    }
    super::clr::mono_string_chars(string_obj)
}

pub unsafe extern "win64" fn mono_string_length(string_obj: *mut c_void) -> c_int {
    if let Some(f) = native_mono().and_then(|m| m.mono_string_length) {
        return f(string_obj);
    }
    super::clr::mono_string_length(string_obj)
}

pub unsafe extern "win64" fn mono_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    if let Some(f) = native_mono().and_then(|m| m.mono_free) {
        f(ptr);
        return;
    }
    libc::free(ptr);
}

pub unsafe extern "win64" fn mono_object_new(
    domain: *mut c_void,
    klass: *mut c_void,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_object_new) {
        return f(domain, klass);
    }
    super::clr::mono_object_new(klass)
}

pub unsafe extern "win64" fn mono_runtime_object_init(this_obj: *mut c_void) {
    if this_obj.is_null() {
        return;
    }
    if let Some(f) = native_mono().and_then(|m| m.mono_runtime_object_init) {
        f(this_obj);
        return;
    }
    super::clr::mono_runtime_object_init(this_obj);
}

pub unsafe extern "win64" fn mono_runtime_class_init(vtable: *mut c_void) {
    if vtable.is_null() {
        return;
    }
    if let Some(f) = native_mono().and_then(|m| m.mono_runtime_class_init) {
        f(vtable);
        return;
    }
    super::clr::mono_runtime_class_init(vtable);
}

pub unsafe extern "win64" fn mono_object_unbox(obj: *mut c_void) -> *mut c_void {
    if obj.is_null() {
        return std::ptr::null_mut();
    }
    if let Some(f) = native_mono().and_then(|m| m.mono_object_unbox) {
        return f(obj);
    }
    super::clr::mono_object_unbox(obj)
}

pub unsafe extern "win64" fn mono_object_get_class(obj: *mut c_void) -> *mut c_void {
    if obj.is_null() {
        return std::ptr::null_mut();
    }
    if let Some(f) = native_mono().and_then(|m| m.mono_object_get_class) {
        return f(obj);
    }
    super::clr::mono_object_get_class(obj)
}

pub unsafe extern "win64" fn mono_value_box(
    domain: *mut c_void,
    klass: *mut c_void,
    val: *mut c_void,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_value_box) {
        return f(domain, klass, val);
    }
    super::clr::mono_value_box(klass, val)
}

pub unsafe extern "win64" fn mono_field_get_value(
    obj: *mut c_void,
    field: *mut c_void,
    val: *mut c_void,
) {
    if let Some(f) = native_mono().and_then(|m| m.mono_field_get_value) {
        f(obj, field, val);
        return;
    }
    super::clr::mono_field_get_value(obj, field, val);
}

pub unsafe extern "win64" fn mono_field_set_value(
    obj: *mut c_void,
    field: *mut c_void,
    val: *mut c_void,
) {
    if let Some(f) = native_mono().and_then(|m| m.mono_field_set_value) {
        f(obj, field, val);
        return;
    }
    super::clr::mono_field_set_value(obj, field, val);
}

pub unsafe extern "win64" fn mono_field_static_get_value(
    vt: *mut c_void,
    field: *mut c_void,
    val: *mut c_void,
) {
    if let Some(f) = native_mono().and_then(|m| m.mono_field_static_get_value) {
        f(vt, field, val);
        return;
    }
    super::clr::mono_field_static_get_value(vt, field, val);
}

pub unsafe extern "win64" fn mono_field_static_set_value(
    vt: *mut c_void,
    field: *mut c_void,
    val: *mut c_void,
) {
    if let Some(f) = native_mono().and_then(|m| m.mono_field_static_set_value) {
        f(vt, field, val);
        return;
    }
    super::clr::mono_field_static_set_value(vt, field, val);
}

pub unsafe extern "win64" fn mono_property_get_get_method(prop: *mut c_void) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_property_get_get_method) {
        return f(prop);
    }
    super::clr::mono_property_get_get_method(prop)
}

pub unsafe extern "win64" fn mono_property_get_set_method(prop: *mut c_void) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_property_get_set_method) {
        return f(prop);
    }
    super::clr::mono_property_get_set_method(prop)
}

pub unsafe extern "win64" fn mono_array_new(
    domain: *mut c_void,
    eclass: *mut c_void,
    n: usize,
) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_array_new) {
        return f(domain, eclass, n);
    }
    super::clr::mono_array_new(eclass, n)
}

pub unsafe extern "win64" fn mono_array_length(array: *mut c_void) -> usize {
    if let Some(f) = native_mono().and_then(|m| m.mono_array_length) {
        return f(array);
    }
    super::clr::mono_array_length(array)
}

pub unsafe extern "win64" fn mono_array_addr_with_size(
    array: *mut c_void,
    size: c_int,
    idx: usize,
) -> *mut u8 {
    if let Some(f) = native_mono().and_then(|m| m.mono_array_addr_with_size) {
        return f(array, size, idx);
    }
    super::clr::mono_array_addr_with_size(array, size, idx)
}

pub unsafe extern "win64" fn mono_array_element_size(eclass: *mut c_void) -> i32 {
    if let Some(f) = native_mono().and_then(|m| m.mono_array_element_size) {
        return f(eclass);
    }
    super::clr::mono_array_element_size(eclass)
}

pub unsafe extern "win64" fn mono_gchandle_new(obj: *mut c_void, pinned: BOOL) -> u32 {
    if let Some(f) = native_mono().and_then(|m| m.mono_gchandle_new) {
        return f(obj, pinned);
    }
    super::clr::mono_gchandle_new(obj, pinned != 0)
}

pub unsafe extern "win64" fn mono_gchandle_new_weakref(
    obj: *mut c_void,
    track_resurrection: BOOL,
) -> u32 {
    if let Some(f) = native_mono().and_then(|m| m.mono_gchandle_new_weakref) {
        return f(obj, track_resurrection);
    }
    super::clr::mono_gchandle_new_weakref(obj, track_resurrection != 0)
}

pub unsafe extern "win64" fn mono_gchandle_get_target(gchandle: u32) -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_gchandle_get_target) {
        return f(gchandle);
    }
    super::clr::mono_gchandle_get_target(gchandle)
}

pub unsafe extern "win64" fn mono_gchandle_free(gchandle: u32) {
    if let Some(f) = native_mono().and_then(|m| m.mono_gchandle_free) {
        f(gchandle);
        return;
    }
    super::clr::mono_gchandle_free(gchandle);
}

pub unsafe extern "win64" fn mono_get_object_class() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_object_class) {
        return f();
    }
    super::clr::mono_builtin_class(0)
}

pub unsafe extern "win64" fn mono_get_int32_class() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_int32_class) {
        return f();
    }
    super::clr::mono_builtin_class(1)
}

pub unsafe extern "win64" fn mono_get_uint32_class() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_uint32_class) {
        return f();
    }
    super::clr::mono_builtin_class(2)
}

pub unsafe extern "win64" fn mono_get_int64_class() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_int64_class) {
        return f();
    }
    super::clr::mono_builtin_class(3)
}

pub unsafe extern "win64" fn mono_get_byte_class() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_byte_class) {
        return f();
    }
    super::clr::mono_builtin_class(4)
}

pub unsafe extern "win64" fn mono_get_boolean_class() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_boolean_class) {
        return f();
    }
    super::clr::mono_builtin_class(5)
}

pub unsafe extern "win64" fn mono_get_single_class() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_single_class) {
        return f();
    }
    super::clr::mono_builtin_class(6)
}

pub unsafe extern "win64" fn mono_get_double_class() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_double_class) {
        return f();
    }
    super::clr::mono_builtin_class(7)
}

pub unsafe extern "win64" fn mono_get_string_class() -> *mut c_void {
    if let Some(f) = native_mono().and_then(|m| m.mono_get_string_class) {
        return f();
    }
    super::clr::mono_builtin_class(8)
}

pub unsafe extern "win64" fn mono_unity_set_vprintf_func(fn_ptr: *const c_void) {
    UNITY_VPRINTF_FN.store(fn_ptr as usize, Ordering::Release);
    if let Some(f) = native_mono().and_then(|m| m.mono_unity_set_vprintf_func) {
        f(fn_ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::super::clr::metadata::elem_type;
    use super::super::clr::metadata::test_asm::{TestAsmBuilder, TestMethodBody};
    use super::*;

    unsafe extern "win64" fn test_icall_add_with_reentrant_string(a: i32, b: i32) -> i32 {
        // Prove re-entrant Mono API call inside an internal call does not deadlock!
        let s = super::super::clr::mono_string_new("reentrant_ok");
        assert!(!s.is_null());
        assert_eq!(super::super::clr::mono_string_length(s), 12);
        a + b + 100
    }

    #[test]
    fn mono_embedded_full_lifecycle_classes_fields_props_arrays_gchandles_icalls() {
        let dir = std::env::temp_dir().join(format!(
            "port_clr_mono_test_{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let dll_path = dir.join("UnityGameScript.dll");

        let mut b = TestAsmBuilder::new("UnityGameScript");
        let corlib = b.add_assembly_ref("mscorlib");
        let obj_ref = b.add_type_ref(corlib, "System", "Object");
        let td = b.add_type_def("Game", "Player", Some(obj_ref));
        let f_hp = b.add_field(0x0006, "Health", elem_type::I4);

        // Method 1: .ctor() sets Health = 50
        let mut ctor_code = vec![0x02, 0x1F, 50, 0x7D]; // ldarg.0; ldc.i4.s 50; stfld Health
        ctor_code.extend_from_slice(&f_hp.to_le_bytes());
        ctor_code.push(0x2A);
        let _m_ctor = b.add_method(
            0x1886,
            0,
            ".ctor",
            &[0x20, 0x00, elem_type::VOID],
            Some(TestMethodBody::tiny(ctor_code)),
        );

        // Method 2: get_Hp() -> this.Health
        let mut get_code = vec![0x02, 0x7B]; // ldarg.0; ldfld Health
        get_code.extend_from_slice(&f_hp.to_le_bytes());
        get_code.push(0x2A);
        let m_get = b.add_method(
            0x0886,
            0,
            "get_Hp",
            &[0x20, 0x00, elem_type::I4],
            Some(TestMethodBody::tiny(get_code)),
        );

        // Method 3: set_Hp(int v) -> this.Health = v
        let mut set_code = vec![0x02, 0x03, 0x7D]; // ldarg.0; ldarg.1; stfld Health
        set_code.extend_from_slice(&f_hp.to_le_bytes());
        set_code.push(0x2A);
        let m_set = b.add_method(
            0x0886,
            0,
            "set_Hp",
            &[0x20, 0x01, elem_type::VOID, elem_type::I4],
            Some(TestMethodBody::tiny(set_code)),
        );

        // Property `Hp` wired to get_Hp / set_Hp
        let _p_hp = b.add_property(
            td,
            "Hp",
            &[0x28, 0x00, elem_type::I4],
            Some(m_get & 0x00FF_FFFF),
            Some(m_set & 0x00FF_FFFF),
        );

        // Method 4: Internal call `NativeAdd(int a, int b) -> int`
        let m_icall = b.add_method(
            0x0016,
            0x1000, // MethodImplAttributes::InternalCall
            "NativeAdd",
            &[0x00, 0x02, elem_type::I4, elem_type::I4, elem_type::I4],
            None,
        );

        // Method 5: Main() -> calls NativeAdd(15, 27) => 142
        let mut main_code = vec![0x1F, 15, 0x1F, 27, 0x28];
        main_code.extend_from_slice(&m_icall.to_le_bytes());
        main_code.push(0x2A);
        let m_main = b.add_method(
            0x0016,
            0,
            "Main",
            &[0x00, 0x00, elem_type::I4],
            Some(TestMethodBody::tiny(main_code)),
        );
        b.entry_point_method_rid = Some(m_main & 0x00FF_FFFF);

        std::fs::write(&dll_path, b.build()).unwrap();

        unsafe {
            let dom = jit_init_with_rt(None, b"UnityDomain\0".as_ptr());
            assert!(!dom.is_null());
            assert_eq!(domain_set_with_rt(None, dom, TRUE), TRUE);
            assert_eq!(domain_set_with_rt(None, std::ptr::null_mut(), TRUE), FALSE);

            // Missing file returns NULL + MONO_IMAGE_ERROR_ERRNO
            let mut st = 0i32;
            assert!(
                assembly_open_with_rt(None, b"/nonexistent/Missing.dll\0".as_ptr(), &mut st).is_null()
            );
            assert_eq!(st, MONO_IMAGE_ERROR_ERRNO);

            // Open synthesized assembly
            let dll_c = CString::new(dll_path.to_string_lossy().as_bytes()).unwrap();
            let asm = assembly_open_with_rt(None, dll_c.as_ptr() as LPCSTR, &mut st);
            assert!(!asm.is_null());
            assert_eq!(st, 0);

            let img = super::super::clr::mono_assembly_get_image(asm);
            assert!(!img.is_null());
            assert_eq!(CStr::from_ptr(super::super::clr::mono_image_get_name(img)), c"UnityGameScript");

            // Nonexistent class returns NULL
            assert!(super::super::clr::mono_class_from_name(img, "Game", "Nope").is_null());

            let klass = super::super::clr::mono_class_from_name(img, "Game", "Player");
            assert!(!klass.is_null());

            // Allocate object + invoke default constructor (.ctor sets Health = 50)
            let obj = super::super::clr::mono_object_new(klass);
            assert!(!obj.is_null());
            super::super::clr::mono_runtime_object_init(obj);
            assert_eq!(super::super::clr::mono_object_get_class(obj), klass);

            // Field get / set
            let f_ptr = super::super::clr::mono_class_get_field_from_name(klass, "Health");
            assert!(!f_ptr.is_null());
            let mut hp = 0i32;
            super::super::clr::mono_field_get_value(obj, f_ptr, &mut hp as *mut i32 as *mut c_void);
            assert_eq!(hp, 50);

            // Property getter / setter via mono_runtime_invoke + mono_object_unbox
            let prop = super::super::clr::mono_class_get_property_from_name(klass, "Hp");
            assert!(!prop.is_null());
            let setter = super::super::clr::mono_property_get_set_method(prop);
            let getter = super::super::clr::mono_property_get_get_method(prop);
            assert!(!setter.is_null() && !getter.is_null());

            let mut new_hp = 125i32;
            let mut set_params = [&mut new_hp as *mut i32 as *mut c_void];
            let mut exc: *mut c_void = std::ptr::null_mut();
            let _ = super::super::clr::mono_runtime_invoke(
                setter,
                obj,
                set_params.as_mut_ptr(),
                &mut exc,
            );
            assert!(exc.is_null());

            let boxed = super::super::clr::mono_runtime_invoke(
                getter,
                obj,
                std::ptr::null_mut(),
                &mut exc,
            );
            assert!(exc.is_null());
            assert!(!boxed.is_null());
            let unboxed = super::super::clr::mono_object_unbox(boxed) as *const i32;
            assert_eq!(std::ptr::read_unaligned(unboxed), 125);

            // MethodDesc search
            let desc = super::super::clr::mono_method_desc_new("Game.Player:get_Hp()", true);
            assert_eq!(super::super::clr::mono_method_desc_search_in_image(desc, img), getter);
            super::super::clr::mono_method_desc_free(desc);

            // Strings (ABI-compatible UTF-16 + UTF-8 roundtrip)
            let s_obj = string_new_with_rt(None, dom, b"HelloUnity\0".as_ptr());
            assert!(!s_obj.is_null());
            assert_eq!(super::super::clr::mono_string_length(s_obj), 10);
            let u8p = string_to_utf8_with_rt(None, s_obj);
            assert_eq!(CStr::from_ptr(u8p), c"HelloUnity");
            libc::free(u8p as *mut c_void);

            // Arrays + GCHandle
            let i32_cls = super::super::clr::mono_builtin_class(1);
            let arr = super::super::clr::mono_array_new(i32_cls, 4);
            assert_eq!(super::super::clr::mono_array_length(arr), 4);
            let elem2 = super::super::clr::mono_array_addr_with_size(arr, 4, 2) as *mut i32;
            std::ptr::write_unaligned(elem2, 999);
            assert_eq!(std::ptr::read_unaligned(elem2), 999);

            let gh = super::super::clr::mono_gchandle_new(obj, true);
            assert_ne!(gh, 0);
            assert_eq!(super::super::clr::mono_gchandle_get_target(gh), obj);
            super::super::clr::mono_gchandle_free(gh);
            assert!(super::super::clr::mono_gchandle_get_target(gh).is_null());

            // Internal call registration + mono_jit_exec running Main() -> 142
            super::super::clr::mono_add_internal_call(
                "Game.Player::NativeAdd",
                test_icall_add_with_reentrant_string as *const c_void,
            );
            assert_eq!(jit_exec_with_rt(None, dom, asm, 0, std::ptr::null()), 142);
            assert_eq!(
                jit_exec_with_rt(None, std::ptr::null_mut(), std::ptr::null_mut(), 0, std::ptr::null()),
                -1
            );
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
