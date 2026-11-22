//! `mscoree.dll`: Windows .NET CLR Bootstrapper & MetaHost COM Bridge
//! (`ICLRMetaHost`, `ICLRRuntimeInfo`, `ICLRRuntimeHost`, `ICorRuntimeHost`, `IEnumUnknown`).
//!
//! Prefers a native host `libcoreclr.dylib` when installed via `crate::misc::coreclr`,
//! and seamlessly executes on the embedded pure-Rust ECMA-335 CLR/CIL runtime
//! (`crate::misc::clr`) when no external host runtime is present (clean macOS or iOS).

use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Mutex, Once, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// Standard Windows CLR COM GUIDs (little-endian byte layouts).
const IID_IUNKNOWN: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
];
const CLSID_COR_RUNTIME_HOST: [u8; 16] = [
    0x23, 0x67, 0x2F, 0xCB, 0x3A, 0xAB, 0xD2, 0x11, 0x9C, 0x40, 0x00, 0xC0, 0x4F, 0xA3, 0x0A, 0x3E,
];
const IID_ICOR_RUNTIME_HOST: [u8; 16] = [
    0x22, 0x67, 0x2F, 0xCB, 0x3A, 0xAB, 0xD2, 0x11, 0x9C, 0x40, 0x00, 0xC0, 0x4F, 0xA3, 0x0A, 0x3E,
];
const IID_ICLR_RUNTIME_HOST: [u8; 16] = [
    0x6C, 0xA0, 0xF1, 0x90, 0x12, 0x77, 0x62, 0x47, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02,
];

static REGISTER_VTABLES: Once = Once::new();
static RUNTIME_STARTED: AtomicBool = AtomicBool::new(false);
static DEFAULT_STARTUP_FLAGS: AtomicU32 = AtomicU32::new(0);
static LEGACY_V2_BOUND: AtomicBool = AtomicBool::new(false);
static HOST_CONTROL_PTR: AtomicUsize = AtomicUsize::new(0);
static LOADED_CALLBACK_PTR: AtomicUsize = AtomicUsize::new(0);
static LOGICAL_THREAD_LOCK_COUNT: AtomicU32 = AtomicU32::new(0);

fn default_startup_config() -> &'static Mutex<String> {
    static CFG: OnceLock<Mutex<String>> = OnceLock::new();
    CFG.get_or_init(|| Mutex::new(String::new()))
}

unsafe fn guid_eq(ptr: *const u8, expected: &[u8; 16]) -> bool {
    if ptr.is_null() {
        return false;
    }
    let mut buf = [0u8; 16];
    std::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), 16);
    buf == *expected
}

unsafe fn wants_cor_runtime_host(rclsid: *const u8, riid: *const u8) -> bool {
    guid_eq(rclsid, &CLSID_COR_RUNTIME_HOST) || guid_eq(riid, &IID_ICOR_RUNTIME_HOST)
}

#[repr(transparent)]
struct SyncCom<T>(T);
unsafe impl<T> Sync for SyncCom<T> {}

unsafe extern "win64" fn singleton_qi(
    this: *mut c_void,
    _riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, this);
    S_OK
}

unsafe extern "win64" fn host_qi(
    this: *mut c_void,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    if guid_eq(riid, &IID_ICOR_RUNTIME_HOST) {
        std::ptr::write_unaligned(ppv, get_cor_runtime_host_instance());
        return S_OK;
    }
    if guid_eq(riid, &IID_ICLR_RUNTIME_HOST) {
        std::ptr::write_unaligned(ppv, get_clr_runtime_host_instance());
        return S_OK;
    }
    if riid.is_null() || guid_eq(riid, &IID_IUNKNOWN) {
        std::ptr::write_unaligned(ppv, this);
        return S_OK;
    }
    std::ptr::write_unaligned(ppv, this);
    S_OK
}

unsafe extern "win64" fn singleton_addref(_this: *mut c_void) -> u32 {
    1
}

unsafe extern "win64" fn singleton_release(_this: *mut c_void) -> u32 {
    1
}

unsafe fn write_wide_out(s: &str, buf: LPWSTR, cch: *mut u32) -> HRESULT {
    if cch.is_null() {
        return E_POINTER;
    }
    let w = crate::strings::to_wide_z(s);
    let cap = std::ptr::read_unaligned(cch) as usize;
    std::ptr::write_unaligned(cch, w.len() as u32);
    if buf.is_null() || cap < w.len() {
        return 0x8007_007Au32 as i32; // HRESULT_FROM_WIN32(ERROR_INSUFFICIENT_BUFFER)
    }
    std::ptr::copy_nonoverlapping(w.as_ptr(), buf, w.len());
    S_OK
}

// ---------------------------------------------------------------------------
// ICLRControl COM object
// ---------------------------------------------------------------------------

#[repr(C)]
struct ClrControlVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    get_clr_manager: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    set_app_domain_manager_type: unsafe extern "win64" fn(*mut c_void, LPCWSTR, LPCWSTR) -> HRESULT,
}

#[repr(C)]
struct ClrControl {
    vtbl: *const ClrControlVtbl,
}

unsafe extern "win64" fn clr_ctrl_get_manager(
    _this: *mut c_void,
    _riid: *const u8,
    pp_obj: *mut *mut c_void,
) -> HRESULT {
    if pp_obj.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_obj, std::ptr::null_mut());
    E_NOINTERFACE
}

unsafe extern "win64" fn clr_ctrl_set_app_domain_manager(
    _this: *mut c_void,
    _asm: LPCWSTR,
    _ty: LPCWSTR,
) -> HRESULT {
    S_OK
}

static CLR_CONTROL_VTBL: ClrControlVtbl = ClrControlVtbl {
    query_interface: singleton_qi,
    add_ref: singleton_addref,
    release: singleton_release,
    get_clr_manager: clr_ctrl_get_manager,
    set_app_domain_manager_type: clr_ctrl_set_app_domain_manager,
};

static CLR_CONTROL_SINGLETON: SyncCom<ClrControl> = SyncCom(ClrControl {
    vtbl: &CLR_CONTROL_VTBL,
});

// ---------------------------------------------------------------------------
// _AppDomain COM wrapper (returned by ICorRuntimeHost::GetDefaultDomain / CreateDomain)
// ---------------------------------------------------------------------------

#[repr(C)]
struct AppDomainComVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
}

#[repr(C)]
struct AppDomainCom {
    vtbl: *const AppDomainComVtbl,
    ref_count: AtomicU32,
    domain_id: u32,
}

unsafe extern "win64" fn appdomain_addref(this: *mut c_void) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*(this as *const AppDomainCom);
    obj.ref_count.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn appdomain_release(this: *mut c_void) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*(this as *const AppDomainCom);
    let prev = obj.ref_count.fetch_sub(1, Ordering::AcqRel);
    if prev <= 1 {
        drop(Box::from_raw(this as *mut AppDomainCom));
        0
    } else {
        prev - 1
    }
}

static APPDOMAIN_VTBL: AppDomainComVtbl = AppDomainComVtbl {
    query_interface: singleton_qi,
    add_ref: appdomain_addref,
    release: appdomain_release,
};

fn alloc_app_domain_com(domain_id: u32) -> *mut c_void {
    ensure_vtables_registered();
    Box::into_raw(Box::new(AppDomainCom {
        vtbl: &APPDOMAIN_VTBL,
        ref_count: AtomicU32::new(1),
        domain_id,
    })) as *mut c_void
}

// ---------------------------------------------------------------------------
// IEnumUnknown COM object (returned by EnumerateInstalledRuntimes / Loaded)
// ---------------------------------------------------------------------------

#[repr(C)]
struct EnumUnknownVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    next: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void, *mut u32) -> HRESULT,
    skip: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
    reset: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    clone: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}

#[repr(C)]
struct RuntimeInfoEnum {
    vtbl: *const EnumUnknownVtbl,
    ref_count: AtomicU32,
    index: AtomicU32,
}

unsafe extern "win64" fn enum_addref(this: *mut c_void) -> u32 {
    let e = &*(this as *const RuntimeInfoEnum);
    e.ref_count.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn enum_release(this: *mut c_void) -> u32 {
    let e = &*(this as *const RuntimeInfoEnum);
    let prev = e.ref_count.fetch_sub(1, Ordering::AcqRel);
    if prev <= 1 {
        drop(Box::from_raw(this as *mut RuntimeInfoEnum));
        0
    } else {
        prev - 1
    }
}

unsafe extern "win64" fn enum_next(
    this: *mut c_void,
    celt: u32,
    rgelt: *mut *mut c_void,
    pcelt_fetched: *mut u32,
) -> HRESULT {
    if rgelt.is_null() {
        return E_POINTER;
    }
    let e = &*(this as *const RuntimeInfoEnum);
    let mut fetched = 0u32;
    if celt > 0 && e.index.load(Ordering::Acquire) == 0 {
        e.index.store(1, Ordering::Release);
        std::ptr::write_unaligned(rgelt, get_clr_runtime_info_instance());
        fetched = 1;
    }
    if !pcelt_fetched.is_null() {
        std::ptr::write_unaligned(pcelt_fetched, fetched);
    }
    if fetched == celt { S_OK } else { S_FALSE }
}

unsafe extern "win64" fn enum_skip(this: *mut c_void, celt: u32) -> HRESULT {
    let e = &*(this as *const RuntimeInfoEnum);
    let cur = e.index.load(Ordering::Acquire);
    let next = cur.saturating_add(celt).min(1);
    e.index.store(next, Ordering::Release);
    if cur + celt <= 1 { S_OK } else { S_FALSE }
}

unsafe extern "win64" fn enum_reset(this: *mut c_void) -> HRESULT {
    let e = &*(this as *const RuntimeInfoEnum);
    e.index.store(0, Ordering::Release);
    S_OK
}

unsafe extern "win64" fn enum_clone(this: *mut c_void, ppenum: *mut *mut c_void) -> HRESULT {
    if ppenum.is_null() {
        return E_POINTER;
    }
    let e = &*(this as *const RuntimeInfoEnum);
    let cloned = Box::into_raw(Box::new(RuntimeInfoEnum {
        vtbl: &ENUM_UNKNOWN_VTBL,
        ref_count: AtomicU32::new(1),
        index: AtomicU32::new(e.index.load(Ordering::Acquire)),
    })) as *mut c_void;
    std::ptr::write_unaligned(ppenum, cloned);
    S_OK
}

static ENUM_UNKNOWN_VTBL: EnumUnknownVtbl = EnumUnknownVtbl {
    query_interface: singleton_qi,
    add_ref: enum_addref,
    release: enum_release,
    next: enum_next,
    skip: enum_skip,
    reset: enum_reset,
    clone: enum_clone,
};

fn alloc_runtime_info_enum() -> *mut c_void {
    ensure_vtables_registered();
    Box::into_raw(Box::new(RuntimeInfoEnum {
        vtbl: &ENUM_UNKNOWN_VTBL,
        ref_count: AtomicU32::new(1),
        index: AtomicU32::new(0),
    })) as *mut c_void
}

// ---------------------------------------------------------------------------
// ICLRRuntimeHost COM object
// ---------------------------------------------------------------------------

#[repr(C)]
#[allow(dead_code)]
struct ClrRuntimeHostVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    start: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    stop: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    set_host_control: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    get_clr_control: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    unload_app_domain: unsafe extern "win64" fn(*mut c_void, u32, BOOL) -> HRESULT,
    execute_in_app_domain:
        unsafe extern "win64" fn(*mut c_void, u32, *const c_void, *mut c_void) -> HRESULT,
    get_current_app_domain_id: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT,
    execute_application: unsafe extern "win64" fn(
        *mut c_void,
        LPCWSTR,
        u32,
        *const LPCWSTR,
        u32,
        *const LPCWSTR,
        *mut i32,
    ) -> HRESULT,
    execute_in_default_app_domain: unsafe extern "win64" fn(
        *mut c_void,
        LPCWSTR,
        LPCWSTR,
        LPCWSTR,
        LPCWSTR,
        *mut u32,
    ) -> HRESULT,
}

#[repr(C)]
#[allow(dead_code)]
struct ClrRuntimeHost {
    vtbl: *const ClrRuntimeHostVtbl,
}

unsafe extern "win64" fn clr_host_start(_this: *mut c_void) -> HRESULT {
    let _ = super::clr::mono_jit_init("DefaultDomain");
    RUNTIME_STARTED.store(true, Ordering::Release);
    S_OK
}

unsafe extern "win64" fn clr_host_stop(_this: *mut c_void) -> HRESULT {
    RUNTIME_STARTED.store(false, Ordering::Release);
    S_OK
}

unsafe extern "win64" fn clr_host_set_host_control(
    _this: *mut c_void,
    ctrl: *mut c_void,
) -> HRESULT {
    HOST_CONTROL_PTR.store(ctrl as usize, Ordering::Release);
    S_OK
}

unsafe extern "win64" fn clr_host_get_clr_control(
    _this: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    ensure_vtables_registered();
    std::ptr::write_unaligned(
        out,
        (&CLR_CONTROL_SINGLETON.0 as *const ClrControl) as *mut c_void,
    );
    S_OK
}

unsafe extern "win64" fn clr_host_unload_app_domain(
    _this: *mut c_void,
    id: u32,
    _wait: BOOL,
) -> HRESULT {
    super::clr::mscoree_unload_domain(id)
}

unsafe extern "win64" fn clr_host_execute_in_app_domain(
    _this: *mut c_void,
    _id: u32,
    cb: *const c_void,
    cookie: *mut c_void,
) -> HRESULT {
    if cb.is_null() {
        return E_POINTER;
    }
    crate::call_guest8(cb as usize, cookie as u64, 0, 0, 0, 0, 0, 0, 0) as HRESULT
}

unsafe extern "win64" fn clr_host_get_current_app_domain_id(
    _this: *mut c_void,
    out: *mut u32,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, 1);
    S_OK
}

unsafe extern "win64" fn clr_host_execute_application(
    _this: *mut c_void,
    app_full_name: LPCWSTR,
    _manifest_count: u32,
    _manifest_paths: *const LPCWSTR,
    _activation_data_count: u32,
    _activation_data: *const LPCWSTR,
    ret_val: *mut i32,
) -> HRESULT {
    let Some(app) = crate::strings::from_wide_opt(app_full_name) else {
        return E_INVALIDARG;
    };
    let rc = super::coreclr::execute_managed_entry(&app);
    if !ret_val.is_null() {
        std::ptr::write_unaligned(ret_val, rc);
    }
    if rc < 0 && (rc as u32 & 0x8000_0000) != 0 {
        rc
    } else {
        S_OK
    }
}

unsafe extern "win64" fn clr_host_execute_in_default_app_domain(
    _this: *mut c_void,
    pwz_assembly_path: LPCWSTR,
    pwz_type_name: LPCWSTR,
    pwz_method_name: LPCWSTR,
    pwz_argument: LPCWSTR,
    p_return_value: *mut u32,
) -> HRESULT {
    let asm_path = crate::strings::from_wide_opt(pwz_assembly_path).unwrap_or_default();
    let ty_name = crate::strings::from_wide_opt(pwz_type_name).unwrap_or_default();
    let method_name = crate::strings::from_wide_opt(pwz_method_name).unwrap_or_default();
    let arg = crate::strings::from_wide_opt(pwz_argument).unwrap_or_default();
    super::coreclr::execute_managed_method(&asm_path, &ty_name, &method_name, &arg, p_return_value)
}

static CLR_RUNTIME_HOST_VTBL: ClrRuntimeHostVtbl = ClrRuntimeHostVtbl {
    query_interface: host_qi,
    add_ref: singleton_addref,
    release: singleton_release,
    start: clr_host_start,
    stop: clr_host_stop,
    set_host_control: clr_host_set_host_control,
    get_clr_control: clr_host_get_clr_control,
    unload_app_domain: clr_host_unload_app_domain,
    execute_in_app_domain: clr_host_execute_in_app_domain,
    get_current_app_domain_id: clr_host_get_current_app_domain_id,
    execute_application: clr_host_execute_application,
    execute_in_default_app_domain: clr_host_execute_in_default_app_domain,
};

static CLR_RUNTIME_HOST_SINGLETON: SyncCom<ClrRuntimeHost> = SyncCom(ClrRuntimeHost {
    vtbl: &CLR_RUNTIME_HOST_VTBL,
});

// ---------------------------------------------------------------------------
// ICorRuntimeHost COM object (classic .NET Framework hosting interface)
// ---------------------------------------------------------------------------

struct DomainEnumState {
    ids: Vec<u32>,
    pos: usize,
}

#[repr(C)]
#[allow(dead_code)]
struct CorRuntimeHostVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    create_logical_thread_state: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    delete_logical_thread_state: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    switch_in_logical_thread_state: unsafe extern "win64" fn(*mut c_void, *const u32) -> HRESULT,
    switch_out_logical_thread_state: unsafe extern "win64" fn(*mut c_void, *mut *mut u32) -> HRESULT,
    locks_held_by_logical_thread: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT,
    map_file: unsafe extern "win64" fn(*mut c_void, HANDLE, *mut usize) -> HRESULT,
    get_configuration: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    start: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    stop: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    create_domain:
        unsafe extern "win64" fn(*mut c_void, LPCWSTR, *mut c_void, *mut *mut c_void) -> HRESULT,
    get_default_domain: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    enum_domains: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    next_domain: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut *mut c_void) -> HRESULT,
    close_enum: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    create_domain_ex: unsafe extern "win64" fn(
        *mut c_void,
        LPCWSTR,
        *mut c_void,
        *mut c_void,
        *mut *mut c_void,
    ) -> HRESULT,
    create_domain_setup: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    create_evidence: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    unload_domain: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    current_domain: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
}

#[repr(C)]
#[allow(dead_code)]
struct CorRuntimeHost {
    vtbl: *const CorRuntimeHostVtbl,
}

unsafe extern "win64" fn cor_host_noop_ok(_this: *mut c_void) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn cor_host_switch_in(_this: *mut c_void, _cookie: *const u32) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn cor_host_switch_out(
    _this: *mut c_void,
    cookie: *mut *mut u32,
) -> HRESULT {
    if !cookie.is_null() {
        std::ptr::write_unaligned(cookie, std::ptr::null_mut());
    }
    S_OK
}

unsafe extern "win64" fn cor_host_locks_held(_this: *mut c_void, count: *mut u32) -> HRESULT {
    if count.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(count, LOGICAL_THREAD_LOCK_COUNT.load(Ordering::Acquire));
    S_OK
}

unsafe extern "win64" fn cor_host_map_file(
    _this: *mut c_void,
    _hfile: HANDLE,
    hmap: *mut usize,
) -> HRESULT {
    if hmap.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(hmap, 0);
    E_NOTIMPL
}

unsafe extern "win64" fn cor_host_get_config(
    _this: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    E_NOTIMPL
}

unsafe extern "win64" fn cor_host_create_domain(
    _this: *mut c_void,
    friendly_name: LPCWSTR,
    _identity: *mut c_void,
    app_domain: *mut *mut c_void,
) -> HRESULT {
    if app_domain.is_null() {
        return E_POINTER;
    }
    let name = crate::strings::from_wide_opt(friendly_name).unwrap_or_else(|| "AppDomain".into());
    let id = super::clr::mscoree_create_domain(&name);
    std::ptr::write_unaligned(app_domain, alloc_app_domain_com(id));
    S_OK
}

unsafe extern "win64" fn cor_host_get_default_domain(
    _this: *mut c_void,
    app_domain: *mut *mut c_void,
) -> HRESULT {
    if app_domain.is_null() {
        return E_POINTER;
    }
    let _ = super::clr::mono_jit_init("DefaultDomain");
    std::ptr::write_unaligned(app_domain, alloc_app_domain_com(1));
    S_OK
}

unsafe extern "win64" fn cor_host_enum_domains(
    _this: *mut c_void,
    h_enum: *mut *mut c_void,
) -> HRESULT {
    if h_enum.is_null() {
        return E_POINTER;
    }
    let ids = super::clr::mscoree_domain_ids();
    let state = Box::into_raw(Box::new(DomainEnumState { ids, pos: 0 })) as *mut c_void;
    std::ptr::write_unaligned(h_enum, state);
    S_OK
}

unsafe extern "win64" fn cor_host_next_domain(
    _this: *mut c_void,
    h_enum: *mut c_void,
    app_domain: *mut *mut c_void,
) -> HRESULT {
    if app_domain.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(app_domain, std::ptr::null_mut());
    if h_enum.is_null() {
        return E_INVALIDARG;
    }
    let st = &mut *(h_enum as *mut DomainEnumState);
    if st.pos < st.ids.len() {
        let id = st.ids[st.pos];
        st.pos += 1;
        std::ptr::write_unaligned(app_domain, alloc_app_domain_com(id));
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn cor_host_close_enum(_this: *mut c_void, h_enum: *mut c_void) -> HRESULT {
    if h_enum.is_null() {
        return E_INVALIDARG;
    }
    drop(Box::from_raw(h_enum as *mut DomainEnumState));
    S_OK
}

unsafe extern "win64" fn cor_host_create_domain_ex(
    this: *mut c_void,
    friendly_name: LPCWSTR,
    _setup: *mut c_void,
    evidence: *mut c_void,
    app_domain: *mut *mut c_void,
) -> HRESULT {
    cor_host_create_domain(this, friendly_name, evidence, app_domain)
}

unsafe extern "win64" fn cor_host_create_setup_or_evidence(
    _this: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, alloc_app_domain_com(0));
    S_OK
}

unsafe extern "win64" fn cor_host_unload_domain(
    _this: *mut c_void,
    app_domain: *mut c_void,
) -> HRESULT {
    if app_domain.is_null() {
        return E_INVALIDARG;
    }
    let obj = &*(app_domain as *const AppDomainCom);
    super::clr::mscoree_unload_domain(obj.domain_id)
}

static COR_RUNTIME_HOST_VTBL: CorRuntimeHostVtbl = CorRuntimeHostVtbl {
    query_interface: host_qi,
    add_ref: singleton_addref,
    release: singleton_release,
    create_logical_thread_state: cor_host_noop_ok,
    delete_logical_thread_state: cor_host_noop_ok,
    switch_in_logical_thread_state: cor_host_switch_in,
    switch_out_logical_thread_state: cor_host_switch_out,
    locks_held_by_logical_thread: cor_host_locks_held,
    map_file: cor_host_map_file,
    get_configuration: cor_host_get_config,
    start: clr_host_start,
    stop: clr_host_stop,
    create_domain: cor_host_create_domain,
    get_default_domain: cor_host_get_default_domain,
    enum_domains: cor_host_enum_domains,
    next_domain: cor_host_next_domain,
    close_enum: cor_host_close_enum,
    create_domain_ex: cor_host_create_domain_ex,
    create_domain_setup: cor_host_create_setup_or_evidence,
    create_evidence: cor_host_create_setup_or_evidence,
    unload_domain: cor_host_unload_domain,
    current_domain: cor_host_get_default_domain,
};

static COR_RUNTIME_HOST_SINGLETON: SyncCom<CorRuntimeHost> = SyncCom(CorRuntimeHost {
    vtbl: &COR_RUNTIME_HOST_VTBL,
});

// ---------------------------------------------------------------------------
// ICLRRuntimeInfo COM object
// ---------------------------------------------------------------------------

#[repr(C)]
#[allow(dead_code)]
struct ClrRuntimeInfoVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    get_version_string: unsafe extern "win64" fn(*mut c_void, LPWSTR, *mut u32) -> HRESULT,
    get_runtime_directory: unsafe extern "win64" fn(*mut c_void, LPWSTR, *mut u32) -> HRESULT,
    is_loaded: unsafe extern "win64" fn(*mut c_void, HANDLE, *mut BOOL) -> HRESULT,
    load_error_string: unsafe extern "win64" fn(*mut c_void, u32, LPWSTR, *mut u32, i32) -> HRESULT,
    load_library: unsafe extern "win64" fn(*mut c_void, LPCWSTR, *mut usize) -> HRESULT,
    get_proc_address: unsafe extern "win64" fn(*mut c_void, LPCSTR, *mut *mut c_void) -> HRESULT,
    get_interface:
        unsafe extern "win64" fn(*mut c_void, *const u8, *const u8, *mut *mut c_void) -> HRESULT,
    is_loadable: unsafe extern "win64" fn(*mut c_void, *mut BOOL) -> HRESULT,
    set_default_startup_flags: unsafe extern "win64" fn(*mut c_void, u32, LPCWSTR) -> HRESULT,
    get_default_startup_flags:
        unsafe extern "win64" fn(*mut c_void, *mut u32, LPWSTR, *mut u32) -> HRESULT,
    bind_as_legacy_v2_runtime: unsafe extern "win64" fn(*mut c_void) -> HRESULT,
    is_started: unsafe extern "win64" fn(*mut c_void, *mut BOOL, *mut u32) -> HRESULT,
}

#[repr(C)]
#[allow(dead_code)]
struct ClrRuntimeInfo {
    vtbl: *const ClrRuntimeInfoVtbl,
}

unsafe extern "win64" fn clr_info_get_version_string(
    _this: *mut c_void,
    buf: LPWSTR,
    cch: *mut u32,
) -> HRESULT {
    let (_, ver) = super::coreclr::runtime_dir_and_version();
    write_wide_out(&ver, buf, cch)
}

unsafe extern "win64" fn clr_info_get_runtime_directory(
    _this: *mut c_void,
    buf: LPWSTR,
    cch: *mut u32,
) -> HRESULT {
    let (dir, _) = super::coreclr::runtime_dir_and_version();
    let s = dir.to_string_lossy();
    write_wide_out(&s, buf, cch)
}

unsafe extern "win64" fn clr_info_is_loaded(
    _this: *mut c_void,
    _proc: HANDLE,
    loaded: *mut BOOL,
) -> HRESULT {
    if loaded.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(loaded, TRUE);
    S_OK
}

unsafe extern "win64" fn clr_info_load_error_string(
    _this: *mut c_void,
    res_id: u32,
    buf: LPWSTR,
    cch: *mut u32,
    _locale: i32,
) -> HRESULT {
    write_wide_out(&format!("CLR error 0x{res_id:08x}"), buf, cch)
}

unsafe extern "win64" fn clr_info_load_library(
    _this: *mut c_void,
    dll_name: LPCWSTR,
    hmod: *mut usize,
) -> HRESULT {
    if hmod.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(hmod, 0);
    let name = crate::strings::from_wide_opt(dll_name).unwrap_or_default();
    if let Some(h) = crate::try_loader().and_then(|l| l.load_library(&name, 0).ok()) {
        if h != 0 {
            std::ptr::write_unaligned(hmod, h);
            return S_OK;
        }
    }
    E_FAIL
}

unsafe extern "win64" fn clr_info_get_proc_address(
    _this: *mut c_void,
    proc_name: LPCSTR,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let name = crate::strings::from_ansi_opt(proc_name).unwrap_or_default();
    let ptr = crate::lookup("coreclr.dll", &name)
        .or_else(|| crate::lookup("mscoree.dll", &name))
        .map(|p| p as *mut c_void)
        .or_else(|| super::coreclr::native_dlsym(&name));
    match ptr {
        Some(p) if !p.is_null() => {
            crate::register_native_va(p as u64, p as usize);
            std::ptr::write_unaligned(out, p);
            S_OK
        }
        _ => E_FAIL,
    }
}

unsafe extern "win64" fn clr_info_get_interface(
    _this: *mut c_void,
    rclsid: *const u8,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    let inst = if wants_cor_runtime_host(rclsid, riid) {
        get_cor_runtime_host_instance()
    } else {
        get_clr_runtime_host_instance()
    };
    std::ptr::write_unaligned(ppv, inst);
    S_OK
}

unsafe extern "win64" fn clr_info_is_loadable(_this: *mut c_void, loadable: *mut BOOL) -> HRESULT {
    if loadable.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(loadable, TRUE);
    S_OK
}

unsafe extern "win64" fn clr_info_set_default_startup_flags(
    _this: *mut c_void,
    flags: u32,
    cfg: LPCWSTR,
) -> HRESULT {
    DEFAULT_STARTUP_FLAGS.store(flags, Ordering::Release);
    if let Some(s) = crate::strings::from_wide_opt(cfg) {
        *default_startup_config()
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = s;
    }
    S_OK
}

unsafe extern "win64" fn clr_info_get_default_startup_flags(
    _this: *mut c_void,
    flags: *mut u32,
    cfg: LPWSTR,
    cch: *mut u32,
) -> HRESULT {
    if !flags.is_null() {
        std::ptr::write_unaligned(flags, DEFAULT_STARTUP_FLAGS.load(Ordering::Acquire));
    }
    if !cch.is_null() {
        let s = default_startup_config()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        return write_wide_out(&s, cfg, cch);
    }
    S_OK
}

unsafe extern "win64" fn clr_info_bind_as_legacy_v2(_this: *mut c_void) -> HRESULT {
    LEGACY_V2_BOUND.store(true, Ordering::Release);
    S_OK
}

unsafe extern "win64" fn clr_info_is_started(
    _this: *mut c_void,
    started: *mut BOOL,
    flags: *mut u32,
) -> HRESULT {
    if !started.is_null() {
        let v = if RUNTIME_STARTED.load(Ordering::Acquire) {
            TRUE
        } else {
            FALSE
        };
        std::ptr::write_unaligned(started, v);
    }
    if !flags.is_null() {
        std::ptr::write_unaligned(flags, DEFAULT_STARTUP_FLAGS.load(Ordering::Acquire));
    }
    S_OK
}

static CLR_RUNTIME_INFO_VTBL: ClrRuntimeInfoVtbl = ClrRuntimeInfoVtbl {
    query_interface: singleton_qi,
    add_ref: singleton_addref,
    release: singleton_release,
    get_version_string: clr_info_get_version_string,
    get_runtime_directory: clr_info_get_runtime_directory,
    is_loaded: clr_info_is_loaded,
    load_error_string: clr_info_load_error_string,
    load_library: clr_info_load_library,
    get_proc_address: clr_info_get_proc_address,
    get_interface: clr_info_get_interface,
    is_loadable: clr_info_is_loadable,
    set_default_startup_flags: clr_info_set_default_startup_flags,
    get_default_startup_flags: clr_info_get_default_startup_flags,
    bind_as_legacy_v2_runtime: clr_info_bind_as_legacy_v2,
    is_started: clr_info_is_started,
};

static CLR_RUNTIME_INFO_SINGLETON: SyncCom<ClrRuntimeInfo> = SyncCom(ClrRuntimeInfo {
    vtbl: &CLR_RUNTIME_INFO_VTBL,
});

// ---------------------------------------------------------------------------
// ICLRMetaHost COM object
// ---------------------------------------------------------------------------

#[repr(C)]
#[allow(dead_code)]
struct ClrMetaHostVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    get_runtime:
        unsafe extern "win64" fn(*mut c_void, LPCWSTR, *const u8, *mut *mut c_void) -> HRESULT,
    get_version_from_file:
        unsafe extern "win64" fn(*mut c_void, LPCWSTR, LPWSTR, *mut u32) -> HRESULT,
    enumerate_installed_runtimes: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT,
    enumerate_loaded_runtimes:
        unsafe extern "win64" fn(*mut c_void, HANDLE, *mut *mut c_void) -> HRESULT,
    request_runtime_loaded_notification:
        unsafe extern "win64" fn(*mut c_void, *const c_void) -> HRESULT,
    query_legacy_v2_runtime_binding:
        unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    exit_process: unsafe extern "win64" fn(*mut c_void, i32) -> HRESULT,
}

#[repr(C)]
#[allow(dead_code)]
struct ClrMetaHost {
    vtbl: *const ClrMetaHostVtbl,
}

unsafe extern "win64" fn metahost_get_runtime(
    _this: *mut c_void,
    _pwz_version: LPCWSTR,
    _riid: *const u8,
    pp_runtime: *mut *mut c_void,
) -> HRESULT {
    if pp_runtime.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_runtime, get_clr_runtime_info_instance());
    S_OK
}

unsafe extern "win64" fn metahost_get_version_from_file(
    _this: *mut c_void,
    file_path: LPCWSTR,
    buf: LPWSTR,
    cch: *mut u32,
) -> HRESULT {
    if file_path.is_null() || cch.is_null() {
        return E_POINTER;
    }
    let path_s = crate::strings::from_wide(file_path);
    let host_pb = super::clr::translate_win_path(&path_s);
    let Ok(bytes) = std::fs::read(&host_pb) else {
        return super::clr::COR_E_FILENOTFOUND;
    };
    let Ok(ver) = super::clr::metadata::read_assembly_version_from_bytes(&bytes) else {
        return super::clr::COR_E_BADIMAGEFORMAT;
    };
    write_wide_out(&ver, buf, cch)
}

unsafe extern "win64" fn metahost_enum_installed(
    _this: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, alloc_runtime_info_enum());
    S_OK
}

unsafe extern "win64" fn metahost_enum_loaded(
    _this: *mut c_void,
    _proc: HANDLE,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, alloc_runtime_info_enum());
    S_OK
}

unsafe extern "win64" fn metahost_request_notification(
    _this: *mut c_void,
    cb: *const c_void,
) -> HRESULT {
    if cb.is_null() {
        return E_POINTER;
    }
    LOADED_CALLBACK_PTR.store(cb as usize, Ordering::Release);
    S_OK
}

unsafe extern "win64" fn metahost_query_legacy_v2(
    _this: *mut c_void,
    _riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, get_clr_runtime_info_instance());
    S_OK
}

unsafe extern "win64" fn metahost_exit_process(_this: *mut c_void, code: i32) -> HRESULT {
    std::process::exit(code);
}

static CLR_METAHOST_VTBL: ClrMetaHostVtbl = ClrMetaHostVtbl {
    query_interface: singleton_qi,
    add_ref: singleton_addref,
    release: singleton_release,
    get_runtime: metahost_get_runtime,
    get_version_from_file: metahost_get_version_from_file,
    enumerate_installed_runtimes: metahost_enum_installed,
    enumerate_loaded_runtimes: metahost_enum_loaded,
    request_runtime_loaded_notification: metahost_request_notification,
    query_legacy_v2_runtime_binding: metahost_query_legacy_v2,
    exit_process: metahost_exit_process,
};

static CLR_METAHOST_SINGLETON: SyncCom<ClrMetaHost> = SyncCom(ClrMetaHost {
    vtbl: &CLR_METAHOST_VTBL,
});

fn ensure_vtables_registered() {
    REGISTER_VTABLES.call_once(|| {
        let ptrs: &[*const ()] = &[
            singleton_qi as *const (),
            host_qi as *const (),
            singleton_addref as *const (),
            singleton_release as *const (),
            clr_ctrl_get_manager as *const (),
            clr_ctrl_set_app_domain_manager as *const (),
            appdomain_addref as *const (),
            appdomain_release as *const (),
            enum_addref as *const (),
            enum_release as *const (),
            enum_next as *const (),
            enum_skip as *const (),
            enum_reset as *const (),
            enum_clone as *const (),
            clr_host_start as *const (),
            clr_host_stop as *const (),
            clr_host_set_host_control as *const (),
            clr_host_get_clr_control as *const (),
            clr_host_unload_app_domain as *const (),
            clr_host_execute_in_app_domain as *const (),
            clr_host_get_current_app_domain_id as *const (),
            clr_host_execute_application as *const (),
            clr_host_execute_in_default_app_domain as *const (),
            cor_host_noop_ok as *const (),
            cor_host_switch_in as *const (),
            cor_host_switch_out as *const (),
            cor_host_locks_held as *const (),
            cor_host_map_file as *const (),
            cor_host_get_config as *const (),
            cor_host_create_domain as *const (),
            cor_host_get_default_domain as *const (),
            cor_host_enum_domains as *const (),
            cor_host_next_domain as *const (),
            cor_host_close_enum as *const (),
            cor_host_create_domain_ex as *const (),
            cor_host_create_setup_or_evidence as *const (),
            cor_host_unload_domain as *const (),
            clr_info_get_version_string as *const (),
            clr_info_get_runtime_directory as *const (),
            clr_info_is_loaded as *const (),
            clr_info_load_error_string as *const (),
            clr_info_load_library as *const (),
            clr_info_get_proc_address as *const (),
            clr_info_get_interface as *const (),
            clr_info_is_loadable as *const (),
            clr_info_set_default_startup_flags as *const (),
            clr_info_get_default_startup_flags as *const (),
            clr_info_bind_as_legacy_v2 as *const (),
            clr_info_is_started as *const (),
            metahost_get_runtime as *const (),
            metahost_get_version_from_file as *const (),
            metahost_enum_installed as *const (),
            metahost_enum_loaded as *const (),
            metahost_request_notification as *const (),
            metahost_query_legacy_v2 as *const (),
            metahost_exit_process as *const (),
        ];
        for p in ptrs {
            let addr = *p as usize;
            if addr != 0 {
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

pub(crate) unsafe fn get_clr_runtime_host_instance() -> *mut c_void {
    ensure_vtables_registered();
    (&CLR_RUNTIME_HOST_SINGLETON.0 as *const ClrRuntimeHost) as *mut c_void
}

pub(crate) unsafe fn get_cor_runtime_host_instance() -> *mut c_void {
    ensure_vtables_registered();
    (&COR_RUNTIME_HOST_SINGLETON.0 as *const CorRuntimeHost) as *mut c_void
}

unsafe fn get_clr_runtime_info_instance() -> *mut c_void {
    ensure_vtables_registered();
    (&CLR_RUNTIME_INFO_SINGLETON.0 as *const ClrRuntimeInfo) as *mut c_void
}

// ---------------------------------------------------------------------------
// Exported C entrypoints of mscoree.dll
// ---------------------------------------------------------------------------

/// Entrypoint jumped to by pure-IL `.NET` executables (`jmp [_CorExeMain]`).
pub unsafe extern "win64" fn _CorExeMain() -> i32 {
    let exe_path = crate::try_loader()
        .and_then(|l| l.module_path(None))
        .unwrap_or_default();
    log::info!("mscoree!_CorExeMain(exe={exe_path:?})");
    super::coreclr::execute_managed_entry(&exe_path)
}

pub unsafe extern "win64" fn _CorExeMain2(
    _image_addr: *mut u8,
    _size: u32,
    entry_name: LPWSTR,
    _loader_name: LPWSTR,
    _cmd_line: LPWSTR,
) -> i32 {
    let name = crate::strings::from_wide_opt(entry_name).unwrap_or_default();
    log::info!("mscoree!_CorExeMain2(entry={name:?})");
    super::coreclr::execute_managed_entry(&name)
}

/// Entrypoint jumped to by `.NET` / C++/CLI class library DLLs (`jmp [_CorDllMain]`).
pub unsafe extern "win64" fn _CorDllMain(
    _hinst: usize,
    reason: u32,
    _reserved: *mut c_void,
) -> BOOL {
    if reason == 1 {
        let _ = super::clr::mono_jit_init("DefaultDomain");
    }
    TRUE
}

pub unsafe extern "win64" fn _CorValidateImage(
    image_base: *mut *mut c_void,
    file_name: LPCWSTR,
) -> HRESULT {
    if let Some(fname) = crate::strings::from_wide_opt(file_name) {
        if !fname.is_empty() {
            let pb = super::clr::translate_win_path(&fname);
            let Ok(bytes) = std::fs::read(&pb) else {
                return super::clr::COR_E_FILENOTFOUND;
            };
            if super::clr::metadata::read_assembly_version_from_bytes(&bytes).is_err() {
                return super::clr::COR_E_BADIMAGEFORMAT;
            }
            return S_OK;
        }
    }
    if image_base.is_null() {
        return E_POINTER;
    }
    S_OK
}

pub unsafe extern "win64" fn _CorImageUnloading(_image_base: *mut c_void) {}

pub unsafe extern "win64" fn CoEEShutDownCOM() {}

pub unsafe extern "win64" fn CorExitProcess(exit_code: i32) {
    std::process::exit(exit_code);
}

pub unsafe extern "win64" fn CLRCreateInstance(
    _clsid: *const u8,
    _riid: *const u8,
    pp_interface: *mut *mut c_void,
) -> HRESULT {
    if pp_interface.is_null() {
        return E_POINTER;
    }
    ensure_vtables_registered();
    std::ptr::write_unaligned(
        pp_interface,
        (&CLR_METAHOST_SINGLETON.0 as *const ClrMetaHost) as *mut c_void,
    );
    S_OK
}

pub unsafe extern "win64" fn CorBindToRuntimeEx(
    _pwsz_version: LPCWSTR,
    _pwsz_build_flavor: LPCWSTR,
    startup_flags: u32,
    rclsid: *const u8,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    DEFAULT_STARTUP_FLAGS.store(startup_flags, Ordering::Release);
    let inst = if wants_cor_runtime_host(rclsid, riid) {
        get_cor_runtime_host_instance()
    } else {
        get_clr_runtime_host_instance()
    };
    std::ptr::write_unaligned(ppv, inst);
    S_OK
}

pub unsafe extern "win64" fn GetCORVersion(
    pb_buffer: LPWSTR,
    cch_buffer: u32,
    dw_length: *mut u32,
) -> HRESULT {
    let (_, ver) = super::coreclr::runtime_dir_and_version();
    let mut len = cch_buffer;
    let hr = write_wide_out(&ver, pb_buffer, &mut len);
    if !dw_length.is_null() {
        std::ptr::write_unaligned(dw_length, len);
    }
    hr
}

pub unsafe extern "win64" fn GetCORSystemDirectory(
    pbuffer: LPWSTR,
    cch_buffer: u32,
    dw_length: *mut u32,
) -> HRESULT {
    let (dir, _) = super::coreclr::runtime_dir_and_version();
    let mut len = cch_buffer;
    let hr = write_wide_out(&dir.to_string_lossy(), pbuffer, &mut len);
    if !dw_length.is_null() {
        std::ptr::write_unaligned(dw_length, len);
    }
    hr
}

pub unsafe extern "win64" fn GetRequestedRuntimeInfo(
    _p_exe: LPCWSTR,
    _pwsz_version: LPCWSTR,
    _p_configuration_file: LPCWSTR,
    _startup_flags: u32,
    _runtime_info_flags: u32,
    p_directory: LPWSTR,
    dw_directory: u32,
    dw_directory_length: *mut u32,
    p_version: LPWSTR,
    cch_buffer: u32,
    dwlength: *mut u32,
) -> HRESULT {
    let (dir, ver) = super::coreclr::runtime_dir_and_version();
    let mut dir_len = dw_directory;
    let hr_dir = write_wide_out(&dir.to_string_lossy(), p_directory, &mut dir_len);
    if !dw_directory_length.is_null() {
        std::ptr::write_unaligned(dw_directory_length, dir_len);
    }
    let mut ver_len = cch_buffer;
    let hr_ver = write_wide_out(&ver, p_version, &mut ver_len);
    if !dwlength.is_null() {
        std::ptr::write_unaligned(dwlength, ver_len);
    }
    if hr_dir != S_OK { hr_dir } else { hr_ver }
}

#[cfg(test)]
mod tests {
    use super::super::clr::metadata::elem_type;
    use super::super::clr::metadata::test_asm::{TestAsmBuilder, TestMethodBody};
    use super::*;

    #[test]
    fn mscoree_com_metahost_runtimehost_and_corruntimehost_end_to_end() {
        let dir = std::env::temp_dir().join(format!(
            "port_clr_mscoree_test_{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let dll_path = dir.join("MscoreePlugin.dll");

        let mut b = TestAsmBuilder::new("MscoreePlugin");
        let corlib = b.add_assembly_ref("mscorlib");
        let obj_ref = b.add_type_ref(corlib, "System", "Object");
        let str_ref = b.add_type_ref(corlib, "System", "String");
        let strlen_mr = b.add_member_ref_on_typeref(
            str_ref,
            "get_Length",
            &[0x20, 0x00, elem_type::I4],
        );
        let _td = b.add_type_def("PluginNs", "Entry", Some(obj_ref));
        // static int Run(string arg) -> arg.Length + 100
        let mut code = vec![0x02, 0x6F]; // ldarg.0; callvirt
        code.extend_from_slice(&strlen_mr.to_le_bytes());
        code.extend_from_slice(&[0x1F, 100, 0x58, 0x2A]); // ldc.i4.s 100; add; ret
        let _m = b.add_method(
            0x0016,
            0,
            "Run",
            &[0x00, 0x01, elem_type::I4, elem_type::STRING],
            Some(TestMethodBody::tiny(code)),
        );
        std::fs::write(&dll_path, b.build()).unwrap();

        unsafe {
            assert_eq!(CLRCreateInstance(std::ptr::null(), std::ptr::null(), std::ptr::null_mut()), E_POINTER);

            let mut metahost_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(CLRCreateInstance(std::ptr::null(), std::ptr::null(), &mut metahost_ptr), S_OK);
            let metahost = &*(metahost_ptr as *const ClrMetaHost);
            let mh_vt = &*metahost.vtbl;

            // GetVersionFromFile reads real BSJB metadata version from disk
            let dll_w = crate::strings::to_wide_z(&dll_path.to_string_lossy());
            let mut vbuf = [0u16; 64];
            let mut vlen = vbuf.len() as u32;
            assert_eq!(
                (mh_vt.get_version_from_file)(metahost_ptr, dll_w.as_ptr(), vbuf.as_mut_ptr(), &mut vlen),
                S_OK
            );
            assert_eq!(crate::strings::from_wide(vbuf.as_ptr()), "v4.0.30319");

            // Missing file fails honestly
            let missing_w = crate::strings::to_wide_z("/nonexistent/Missing.dll");
            vlen = vbuf.len() as u32;
            assert_eq!(
                (mh_vt.get_version_from_file)(metahost_ptr, missing_w.as_ptr(), vbuf.as_mut_ptr(), &mut vlen),
                super::super::clr::COR_E_FILENOTFOUND
            );

            // EnumerateInstalledRuntimes -> IEnumUnknown -> ICLRRuntimeInfo
            let mut enum_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!((mh_vt.enumerate_installed_runtimes)(metahost_ptr, &mut enum_ptr), S_OK);
            let en = &*(enum_ptr as *const RuntimeInfoEnum);
            let en_vt = &*en.vtbl;
            let mut info_ptr: *mut c_void = std::ptr::null_mut();
            let mut fetched = 0u32;
            assert_eq!((en_vt.next)(enum_ptr, 1, &mut info_ptr, &mut fetched), S_OK);
            assert_eq!(fetched, 1);
            assert!(!info_ptr.is_null());
            assert_eq!((en_vt.release)(enum_ptr), 0);

            // ICLRRuntimeInfo::GetInterface -> ICLRRuntimeHost
            let info = &*(info_ptr as *const ClrRuntimeInfo);
            let info_vt = &*info.vtbl;
            let mut host_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                (info_vt.get_interface)(
                    info_ptr,
                    std::ptr::null(),
                    IID_ICLR_RUNTIME_HOST.as_ptr(),
                    &mut host_ptr,
                ),
                S_OK
            );
            let host = &*(host_ptr as *const ClrRuntimeHost);
            let host_vt = &*host.vtbl;
            assert_eq!((host_vt.start)(host_ptr), S_OK);

            // ExecuteInDefaultAppDomain runs real CIL method `PluginNs.Entry::Run("Hello!")` -> 106
            let ty_w = crate::strings::to_wide_z("PluginNs.Entry");
            let meth_w = crate::strings::to_wide_z("Run");
            let arg_w = crate::strings::to_wide_z("Hello!");
            let mut ret_val = 0u32;
            assert_eq!(
                (host_vt.execute_in_default_app_domain)(
                    host_ptr,
                    dll_w.as_ptr(),
                    ty_w.as_ptr(),
                    meth_w.as_ptr(),
                    arg_w.as_ptr(),
                    &mut ret_val,
                ),
                S_OK
            );
            assert_eq!(ret_val, 106);

            // CorBindToRuntimeEx with IID_ICorRuntimeHost returns real ICorRuntimeHost
            let mut cor_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                CorBindToRuntimeEx(
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    CLSID_COR_RUNTIME_HOST.as_ptr(),
                    IID_ICOR_RUNTIME_HOST.as_ptr(),
                    &mut cor_ptr,
                ),
                S_OK
            );
            let cor = &*(cor_ptr as *const CorRuntimeHost);
            let cor_vt = &*cor.vtbl;
            let mut dom_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!((cor_vt.get_default_domain)(cor_ptr, &mut dom_ptr), S_OK);
            assert!(!dom_ptr.is_null());
            let dom = &*(dom_ptr as *const AppDomainCom);
            assert_eq!(((*dom.vtbl).release)(dom_ptr), 0);

            let mut dom_enum: *mut c_void = std::ptr::null_mut();
            assert_eq!((cor_vt.enum_domains)(cor_ptr, &mut dom_enum), S_OK);
            let mut next_dom: *mut c_void = std::ptr::null_mut();
            assert_eq!((cor_vt.next_domain)(cor_ptr, dom_enum, &mut next_dom), S_OK);
            assert!(!next_dom.is_null());
            let nd = &*(next_dom as *const AppDomainCom);
            assert_eq!(((*nd.vtbl).release)(next_dom), 0);
            assert_eq!((cor_vt.close_enum)(cor_ptr, dom_enum), S_OK);

            // GetCORVersion & _CorValidateImage succeed on valid assembly
            let mut ver_len = 0u32;
            assert_eq!(GetCORVersion(vbuf.as_mut_ptr(), vbuf.len() as u32, &mut ver_len), S_OK);
            assert!(ver_len > 1);
            assert_eq!(_CorValidateImage(std::ptr::null_mut(), dll_w.as_ptr()), S_OK);
            assert_eq!(
                _CorValidateImage(std::ptr::null_mut(), missing_w.as_ptr()),
                super::super::clr::COR_E_FILENOTFOUND
            );
            assert_eq!(_CorDllMain(0, 1, std::ptr::null_mut()), TRUE);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
