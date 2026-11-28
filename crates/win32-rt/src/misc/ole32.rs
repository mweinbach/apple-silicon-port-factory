//! ole32.dll: COM runtime, HGLOBAL-backed `IStream`, `IMarshal` free-threaded
//! marshaler, `IClassFactory`, `PropVariantClear`, `ReleaseStgMedium`, and
//! per-HWND `IDropTarget` registration.

use super::{guid_string, new_guid};
use crate::types::*;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const REGDB_E_CLASSNOTREG: HRESULT = 0x8004_0154u32 as i32;
#[allow(dead_code)]
const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110u32 as i32;
const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111u32 as i32;
const CO_E_CLASSSTRING: HRESULT = 0x8004_01F3u32 as i32;
const CLSCTX_INPROC_SERVER: u32 = 0x1;
const STG_E_INVALIDFUNCTION: HRESULT = 0x8003_0001u32 as i32;
const STG_E_INVALIDPOINTER: HRESULT = 0x8003_0009u32 as i32;
const STG_E_INVALIDPARAMETER: HRESULT = 0x8003_0057u32 as i32;
const STG_E_MEDIUMFULL: HRESULT = 0x8003_0070u32 as i32;
const STG_E_INVALIDFLAG: HRESULT = 0x8003_00FFu32 as i32;
const DRAGDROP_E_NOTREGISTERED: HRESULT = 0x8004_0100u32 as i32;
const DRAGDROP_E_ALREADYREGISTERED: HRESULT = 0x8004_0101u32 as i32;
const DRAGDROP_E_INVALIDHWND: HRESULT = 0x8004_0102u32 as i32;

const GMEM_MOVEABLE: u32 = 0x0002;
const GMEM_ZEROINIT: u32 = 0x0040;

const STREAM_SEEK_SET: u32 = 0;
const STREAM_SEEK_CUR: u32 = 1;
const STREAM_SEEK_END: u32 = 2;

const STGTY_STREAM: u32 = 2;
const STGM_READWRITE: u32 = 0x0000_0002;
const STATFLAG_DEFAULT: u32 = 0;
const STATFLAG_NONAME: u32 = 1;

const TYMED_NULL: u32 = 0;
const TYMED_HGLOBAL: u32 = 1;
const TYMED_FILE: u32 = 2;
const TYMED_ISTREAM: u32 = 4;
const TYMED_ISTORAGE: u32 = 8;

const MSHLFLAGS_NORMAL: u32 = 0;
const FTM_MAGIC: u32 = 0x4654_4D31; // "FTM1"

const IID_IUNKNOWN_BYTES: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
];
const IID_ICLASSFACTORY_BYTES: [u8; 16] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
];
const IID_IMALLOC_BYTES: [u8; 16] = [
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
];
const IID_IMARSHAL_BYTES: [u8; 16] = [
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
];
const IID_ISTREAM_BYTES: [u8; 16] = [
    0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
];
const IID_ISEQUENTIALSTREAM_BYTES: [u8; 16] = [
    0x30, 0x3A, 0x73, 0x0C, 0x1C, 0x2A, 0xCE, 0x11, 0xAD, 0xE5, 0x00, 0xAA, 0x00, 0x44, 0x77, 0x3D,
];
const IID_IAGILEOBJECT_BYTES: [u8; 16] = [
    0x94, 0x2B, 0xEA, 0x94, 0xCC, 0xE9, 0xE0, 0x49, 0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90,
];
const CLSID_INPROCFREEMARSHALER_BYTES: [u8; 16] = [
    0x3A, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46,
];
const CLSID_MMDEVICEENUMERATOR_BYTES: [u8; 16] = [
    0x95, 0x03, 0xDE, 0xBC, 0x2F, 0xE5, 0x7C, 0x46, 0x8E, 0x3D, 0xC4, 0x57, 0x92, 0x91, 0x69, 0x2E,
];

thread_local! {
    static COM_INIT: Cell<u32> = const { Cell::new(0) };
}

fn register_vtable_ptrs(ptrs: &[usize]) {
    for &addr in ptrs {
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
}

/// Invoke `IUnknown::QueryInterface` (slot 0) on any COM object (guest or native).
unsafe fn com_call_qi(obj: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if obj.is_null() {
        return E_POINTER;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return E_POINTER;
    }
    let method = std::ptr::read_unaligned(vtbl.add(0));
    if method == 0 {
        return E_NOINTERFACE;
    }
    crate::call_guest8(method, obj as u64, riid as u64, ppv as u64, 0, 0, 0, 0, 0) as HRESULT
}

/// Invoke `IUnknown::AddRef` (slot 1) on any COM object (guest or native).
unsafe fn com_call_add_ref(obj: *mut c_void) -> u32 {
    if obj.is_null() {
        return 0;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return 0;
    }
    let method = std::ptr::read_unaligned(vtbl.add(1));
    if method == 0 {
        return 0;
    }
    crate::call_guest8(method, obj as u64, 0, 0, 0, 0, 0, 0, 0) as u32
}

/// Invoke `IUnknown::Release` (slot 2) on any COM object (guest or native).
unsafe fn com_call_release(obj: *mut c_void) -> u32 {
    if obj.is_null() {
        return 0;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const *const usize);
    if vtbl.is_null() {
        return 0;
    }
    let method = std::ptr::read_unaligned(vtbl.add(2));
    if method == 0 {
        return 0;
    }
    crate::call_guest8(method, obj as u64, 0, 0, 0, 0, 0, 0, 0) as u32
}

/// Invoke `ISequentialStream::Read` (slot 3) on any `IStream*` (guest or native).
unsafe fn com_stream_read(pstm: *mut c_void, pv: *mut c_void, cb: u32, pcb_read: *mut u32) -> HRESULT {
    let vtbl = std::ptr::read_unaligned(pstm as *const *const usize);
    if vtbl.is_null() {
        return STG_E_INVALIDPOINTER;
    }
    let method = std::ptr::read_unaligned(vtbl.add(3));
    if method == 0 {
        return E_NOTIMPL;
    }
    crate::call_guest8(method, pstm as u64, pv as u64, cb as u64, pcb_read as u64, 0, 0, 0, 0) as HRESULT
}

/// Invoke `ISequentialStream::Write` (slot 4) on any `IStream*` (guest or native).
unsafe fn com_stream_write(pstm: *mut c_void, pv: *const c_void, cb: u32, pcb_written: *mut u32) -> HRESULT {
    let vtbl = std::ptr::read_unaligned(pstm as *const *const usize);
    if vtbl.is_null() {
        return STG_E_INVALIDPOINTER;
    }
    let method = std::ptr::read_unaligned(vtbl.add(4));
    if method == 0 {
        return E_NOTIMPL;
    }
    crate::call_guest8(method, pstm as u64, pv as u64, cb as u64, pcb_written as u64, 0, 0, 0, 0) as HRESULT
}

unsafe fn read_guid_bytes(p: *const u8) -> Option<[u8; 16]> {
    if p.is_null() {
        return None;
    }
    let mut b = [0u8; 16];
    std::ptr::copy_nonoverlapping(p, b.as_mut_ptr(), 16);
    Some(b)
}

pub unsafe extern "win64" fn CoCreateGuid(g: *mut u8) -> HRESULT {
    if g.is_null() {
        return E_INVALIDARG;
    }
    let v = new_guid();
    std::ptr::copy_nonoverlapping(v.as_ptr(), g, 16);
    S_OK
}

fn com_init() -> HRESULT {
    COM_INIT.with(|c| {
        let n = c.get();
        c.set(n + 1);
        if n == 0 {
            S_OK
        } else {
            S_FALSE
        }
    })
}

fn com_uninit() {
    COM_INIT.with(|c| c.set(c.get().saturating_sub(1)));
}

pub unsafe extern "win64" fn CoInitialize(_r: *mut c_void) -> HRESULT {
    com_init()
}

pub unsafe extern "win64" fn CoInitializeEx(_r: *mut c_void, _model: u32) -> HRESULT {
    com_init()
}

pub unsafe extern "win64" fn CoUninitialize() {
    com_uninit()
}

pub unsafe extern "win64" fn OleInitialize(_r: *mut c_void) -> HRESULT {
    com_init()
}

pub unsafe extern "win64" fn OleUninitialize() {
    com_uninit()
}

/// Dynamic COM class factory registered by middleware or game targets.
pub type ComClassFactory =
    unsafe fn(clsid: *const u8, iid: *const u8, outer: *mut c_void, out: *mut *mut c_void) -> Option<HRESULT>;

static EXTRA_COM_FACTORIES: std::sync::RwLock<Vec<ComClassFactory>> = std::sync::RwLock::new(Vec::new());

/// Register an additional in-process COM class factory checked by [`CoCreateInstance`].
pub fn register_com_factory(factory: ComClassFactory) {
    EXTRA_COM_FACTORIES.write().unwrap().push(factory);
}

struct RegisteredClassObject {
    cookie: u32,
    clsid: [u8; 16],
    punk: Arc<crate::com::ComPtr>,
    cls_context: u32,
    _flags: u32,
}

static NEXT_CLASS_REG_COOKIE: AtomicU32 = AtomicU32::new(1);
static REGISTERED_CLASS_OBJECTS: LazyLock<Mutex<Vec<RegisteredClassObject>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));
static INPROC_SERVER_DLLS: LazyLock<Mutex<HashMap<[u8; 16], String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Associate a COM CLSID with an in-process server DLL exporting `DllGetClassObject`.
#[allow(dead_code)]
pub fn register_inproc_server_dll(clsid: crate::com::Guid, dll_name: impl Into<String>) {
    INPROC_SERVER_DLLS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(clsid.to_bytes(), dll_name.into());
}

#[allow(dead_code)]
pub fn unregister_inproc_server_dll(clsid: crate::com::Guid) {
    INPROC_SERVER_DLLS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&clsid.to_bytes());
}

fn find_registered_class_object(clsid: &[u8; 16], ctx: u32) -> Option<Arc<crate::com::ComPtr>> {
    let regs = REGISTERED_CLASS_OBJECTS.lock().unwrap_or_else(|e| e.into_inner());
    regs.iter()
        .rev()
        .find(|e| e.clsid == *clsid && (ctx == 0 || e.cls_context == 0 || (e.cls_context & ctx) != 0))
        .map(|e| Arc::clone(&e.punk))
}

unsafe fn try_inproc_dll_get_class_object(
    clsid: &[u8; 16],
    ctx: u32,
    riid: *const u8,
    out: *mut *mut c_void,
) -> Option<HRESULT> {
    if ctx != 0 && (ctx & CLSCTX_INPROC_SERVER) == 0 {
        return None;
    }
    let dll_name = INPROC_SERVER_DLLS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(clsid)
        .cloned()?;
    let loader = crate::try_loader()?;
    let hmod = loader
        .get_module_handle(Some(&dll_name))
        .or_else(|| loader.load_library(&dll_name, 0).ok())?;
    let proc_addr = loader.get_proc_address(hmod, crate::ProcName::Name("DllGetClassObject"))?;
    let hr = crate::call_guest8(
        proc_addr,
        clsid.as_ptr() as u64,
        riid as u64,
        out as u64,
        0,
        0,
        0,
        0,
        0,
    ) as HRESULT;
    Some(hr)
}

pub unsafe extern "win64" fn CoRegisterClassObject(
    rclsid: *const u8,
    p_unk: *mut c_void,
    dw_cls_context: u32,
    flags: u32,
    lpdw_register: *mut u32,
) -> HRESULT {
    if lpdw_register.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(lpdw_register, 0);
    let Some(clsid) = read_guid_bytes(rclsid) else {
        return E_INVALIDARG;
    };
    if p_unk.is_null() {
        return E_INVALIDARG;
    }
    // Retain via AddRef strictly OUTSIDE any mutex.
    let Some(retained) = crate::com::ComPtr::retain(p_unk) else {
        return E_INVALIDARG;
    };
    let cookie = NEXT_CLASS_REG_COOKIE.fetch_add(1, Ordering::Relaxed);
    REGISTERED_CLASS_OBJECTS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(RegisteredClassObject {
            cookie,
            clsid,
            punk: Arc::new(retained),
            cls_context: dw_cls_context,
            _flags: flags,
        });
    std::ptr::write_unaligned(lpdw_register, cookie);
    S_OK
}

pub unsafe extern "win64" fn CoRevokeClassObject(dw_register: u32) -> HRESULT {
    if dw_register == 0 {
        return E_INVALIDARG;
    }
    let removed = {
        let mut regs = REGISTERED_CLASS_OBJECTS.lock().unwrap_or_else(|e| e.into_inner());
        regs.iter()
            .position(|e| e.cookie == dw_register)
            .map(|i| regs.remove(i))
    };
    // Drop Arc<ComPtr> (potentially calling guest Release) strictly OUTSIDE mutex.
    match removed {
        Some(entry) => {
            drop(entry);
            S_OK
        }
        None => E_INVALIDARG,
    }
}

pub unsafe extern "win64" fn CoCreateInstance(
    clsid: *const u8,
    outer: *mut c_void,
    ctx: u32,
    iid: *const u8,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    // 1. Registered in-proc class objects (CoRegisterClassObject) take precedence.
    if let Some(clsid_bytes) = read_guid_bytes(clsid) {
        if let Some(reg_punk) = find_registered_class_object(&clsid_bytes, ctx) {
            let mut cf_ptr: *mut c_void = std::ptr::null_mut();
            let qhr = com_call_qi(reg_punk.as_ptr(), IID_ICLASSFACTORY_BYTES.as_ptr(), &mut cf_ptr);
            if qhr < 0 || cf_ptr.is_null() {
                return if qhr < 0 { qhr } else { E_NOINTERFACE };
            }
            let hr = crate::com::com_invoke_hresult(
                cf_ptr,
                &crate::com::IID_IClassFactory,
                3, // IClassFactory::CreateInstance
                &[outer as u64, iid as u64, out as u64],
            );
            com_call_release(cf_ptr);
            return hr;
        }
    }
    // 2. Dynamic class factories registered by game targets / plugins.
    if let Ok(factories) = EXTRA_COM_FACTORIES.read() {
        for factory in factories.iter().rev() {
            if let Some(hr) = factory(clsid, iid, outer, out) {
                return hr;
            }
        }
    }
    // 3. Built-in Rust in-process COM classes.
    if let Some(hr) = super::dinput8::create_instance(clsid, iid, outer, out) {
        return hr;
    }
    if let Some(hr) = super::msctf::create_instance(clsid, iid, outer, out) {
        return hr;
    }
    if let Some(clsid_bytes) = read_guid_bytes(clsid) {
        if clsid_bytes == CLSID_INPROCFREEMARSHALER_BYTES {
            let mut ftm_unk: *mut c_void = std::ptr::null_mut();
            let hr = CoCreateFreeThreadedMarshaler(outer, &mut ftm_unk);
            if hr != S_OK {
                return hr;
            }
            if let Some(iid_bytes) = read_guid_bytes(iid) {
                if iid_bytes == IID_IUNKNOWN_BYTES {
                    std::ptr::write_unaligned(out, ftm_unk);
                    return S_OK;
                }
                let qhr = com_call_qi(ftm_unk, iid, out);
                com_call_release(ftm_unk);
                return qhr;
            }
            com_call_release(ftm_unk);
            return E_INVALIDARG;
        }
    }
    if let (Some(c), Some(i)) = (
        crate::audio::com::GUID::read(clsid as *const crate::audio::com::GUID),
        crate::audio::com::GUID::read(iid as *const crate::audio::com::GUID),
    ) {
        let known = c == crate::audio::com::CLSID_MMDeviceEnumerator;
        if known && !outer.is_null() {
            return crate::audio::com::CLASS_E_NOAGGREGATION;
        }
        if let Some(hr) = crate::audio::create_instance(&c, &i, out) {
            return hr;
        }
    }
    // 4. Registered in-proc server DLL exporting DllGetClassObject.
    if let Some(clsid_bytes) = read_guid_bytes(clsid) {
        let mut cf_ptr: *mut c_void = std::ptr::null_mut();
        if let Some(ghr) = try_inproc_dll_get_class_object(
            &clsid_bytes,
            ctx,
            IID_ICLASSFACTORY_BYTES.as_ptr(),
            &mut cf_ptr,
        ) {
            if ghr < 0 || cf_ptr.is_null() {
                return if ghr < 0 { ghr } else { CLASS_E_CLASSNOTAVAILABLE };
            }
            let hr = crate::com::com_invoke_hresult(
                cf_ptr,
                &crate::com::IID_IClassFactory,
                3,
                &[outer as u64, iid as u64, out as u64],
            );
            com_call_release(cf_ptr);
            return hr;
        }
    }
    let clsid_g = read_guid_bytes(clsid).map(crate::com::Guid::from_bytes);
    let iid_g = read_guid_bytes(iid).map(crate::com::Guid::from_bytes);
    crate::com::warn_unknown_clsid_once("CoCreateInstance", clsid_g.as_ref(), iid_g.as_ref());
    REGDB_E_CLASSNOTREG
}

pub unsafe extern "win64" fn CoSetProxyBlanket(
    p_proxy: *mut c_void,
    _dw_authn_svc: u32,
    _dw_authz_svc: u32,
    _p_server_princ_name: *mut u16,
    _dw_authn_level: u32,
    _dw_imp_level: u32,
    _p_auth_info: *mut c_void,
    _dw_capabilities: u32,
) -> HRESULT {
    if p_proxy.is_null() {
        return E_INVALIDARG;
    }
    S_OK
}

pub unsafe extern "win64" fn CoLockObjectExternal(
    p_unk: *mut c_void,
    f_lock: BOOL,
    _f_last_unlock_releases: BOOL,
) -> HRESULT {
    if p_unk.is_null() {
        return E_INVALIDARG;
    }
    if f_lock != 0 {
        com_call_add_ref(p_unk);
    } else {
        com_call_release(p_unk);
    }
    S_OK
}

// ---------------------------------------------------------------------------
// Free-Threaded Marshaler (`CoCreateFreeThreadedMarshaler` / `IMarshal`)
// ---------------------------------------------------------------------------

#[repr(C)]
struct IMarshalVtbl {
    query_interface: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut c_void) -> u32,
    release: unsafe extern "win64" fn(*mut c_void) -> u32,
    get_unmarshal_class: unsafe extern "win64" fn(
        *mut c_void,
        *const u8,
        *mut c_void,
        u32,
        *mut c_void,
        u32,
        *mut u8,
    ) -> HRESULT,
    get_marshal_size_max: unsafe extern "win64" fn(
        *mut c_void,
        *const u8,
        *mut c_void,
        u32,
        *mut c_void,
        u32,
        *mut u32,
    ) -> HRESULT,
    marshal_interface: unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        *const u8,
        *mut c_void,
        u32,
        *mut c_void,
        u32,
    ) -> HRESULT,
    unmarshal_interface: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const u8, *mut *mut c_void) -> HRESULT,
    release_marshal_data: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT,
    disconnect_object: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT,
}

/// In-process Free-Threaded Marshaler supporting COM aggregation (`punkOuter`).
/// Offset 0 is the non-delegating `IUnknown` (also exposing `IMarshal` methods
/// when unaggregated), and offset 8 is the delegating `IMarshal` sub-interface.
#[repr(C)]
struct FtmObj {
    unk_vtbl: *const IMarshalVtbl,
    marshal_vtbl: *const IMarshalVtbl,
    refs: AtomicU32,
    outer: *mut c_void,
}

unsafe fn ftm_from_marshal(iface: *mut c_void) -> *mut FtmObj {
    (iface as *mut u8).sub(std::mem::size_of::<*const IMarshalVtbl>()) as *mut FtmObj
}

unsafe extern "win64" fn ftm_unk_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid_bytes(riid) else {
        return E_INVALIDARG;
    };
    let obj = &*(this as *const FtmObj);
    if iid == IID_IUNKNOWN_BYTES {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this);
        S_OK
    } else if iid == IID_IMARSHAL_BYTES || iid == IID_IAGILEOBJECT_BYTES {
        if !obj.outer.is_null() {
            com_call_add_ref(obj.outer);
            let m_ptr = (this as *mut u8).add(std::mem::size_of::<*const IMarshalVtbl>()) as *mut c_void;
            std::ptr::write_unaligned(ppv, m_ptr);
        } else {
            obj.refs.fetch_add(1, Ordering::AcqRel);
            std::ptr::write_unaligned(ppv, this);
        }
        S_OK
    } else {
        crate::com::warn_qi_failed_once("FreeThreadedMarshaler", &crate::com::Guid::from_bytes(iid));
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn ftm_unk_add_ref(this: *mut c_void) -> u32 {
    let obj = &*(this as *const FtmObj);
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn ftm_unk_release(this: *mut c_void) -> u32 {
    let ptr = this as *mut FtmObj;
    let remaining = (&*ptr).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if remaining == 0 {
        drop(Box::from_raw(ptr));
    }
    remaining
}

unsafe extern "win64" fn ftm_marshal_qi(this: *mut c_void, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    let obj = &*ftm_from_marshal(this);
    if !obj.outer.is_null() {
        return com_call_qi(obj.outer, riid, ppv);
    }
    ftm_unk_qi(ftm_from_marshal(this) as *mut c_void, riid, ppv)
}

unsafe extern "win64" fn ftm_marshal_add_ref(this: *mut c_void) -> u32 {
    let obj = &*ftm_from_marshal(this);
    if !obj.outer.is_null() {
        return com_call_add_ref(obj.outer);
    }
    ftm_unk_add_ref(ftm_from_marshal(this) as *mut c_void)
}

unsafe extern "win64" fn ftm_marshal_release(this: *mut c_void) -> u32 {
    let obj = &*ftm_from_marshal(this);
    if !obj.outer.is_null() {
        return com_call_release(obj.outer);
    }
    ftm_unk_release(ftm_from_marshal(this) as *mut c_void)
}

unsafe extern "win64" fn ftm_get_unmarshal_class(
    _this: *mut c_void,
    _riid: *const u8,
    _pv: *mut c_void,
    _dw_dest_context: u32,
    _pv_dest_context: *mut c_void,
    _mshlflags: u32,
    p_cid: *mut u8,
) -> HRESULT {
    if p_cid.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::copy_nonoverlapping(CLSID_INPROCFREEMARSHALER_BYTES.as_ptr(), p_cid, 16);
    S_OK
}

unsafe extern "win64" fn ftm_get_marshal_size_max(
    _this: *mut c_void,
    _riid: *const u8,
    _pv: *mut c_void,
    _dw_dest_context: u32,
    _pv_dest_context: *mut c_void,
    _mshlflags: u32,
    p_size: *mut u32,
) -> HRESULT {
    if p_size.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(p_size, 16);
    S_OK
}

unsafe extern "win64" fn ftm_marshal_interface(
    _this: *mut c_void,
    p_stm: *mut c_void,
    _riid: *const u8,
    pv: *mut c_void,
    _dw_dest_context: u32,
    _pv_dest_context: *mut c_void,
    mshlflags: u32,
) -> HRESULT {
    if p_stm.is_null() || pv.is_null() {
        return E_INVALIDARG;
    }
    let mut payload = [0u8; 16];
    payload[0..4].copy_from_slice(&FTM_MAGIC.to_le_bytes());
    payload[4..8].copy_from_slice(&mshlflags.to_le_bytes());
    payload[8..16].copy_from_slice(&(pv as usize as u64).to_le_bytes());
    let mut written: u32 = 0;
    let hr = com_stream_write(p_stm, payload.as_ptr() as *const c_void, 16, &mut written);
    if hr != S_OK {
        return hr;
    }
    if written != 16 {
        return STG_E_MEDIUMFULL;
    }
    com_call_add_ref(pv);
    S_OK
}

unsafe extern "win64" fn ftm_unmarshal_interface(
    _this: *mut c_void,
    p_stm: *mut c_void,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    if p_stm.is_null() || riid.is_null() {
        return E_INVALIDARG;
    }
    let mut payload = [0u8; 16];
    let mut read: u32 = 0;
    let hr = com_stream_read(p_stm, payload.as_mut_ptr() as *mut c_void, 16, &mut read);
    if hr < 0 || read != 16 {
        return if hr < 0 { hr } else { E_FAIL };
    }
    let magic = u32::from_le_bytes(payload[0..4].try_into().unwrap());
    if magic != FTM_MAGIC {
        return E_FAIL;
    }
    let mshlflags = u32::from_le_bytes(payload[4..8].try_into().unwrap());
    let pv = u64::from_le_bytes(payload[8..16].try_into().unwrap()) as usize as *mut c_void;
    if pv.is_null() {
        return E_FAIL;
    }
    let qhr = com_call_qi(pv, riid, ppv);
    if mshlflags == MSHLFLAGS_NORMAL {
        com_call_release(pv);
    }
    qhr
}

unsafe extern "win64" fn ftm_release_marshal_data(_this: *mut c_void, p_stm: *mut c_void) -> HRESULT {
    if p_stm.is_null() {
        return E_INVALIDARG;
    }
    let mut payload = [0u8; 16];
    let mut read: u32 = 0;
    let hr = com_stream_read(p_stm, payload.as_mut_ptr() as *mut c_void, 16, &mut read);
    if hr < 0 || read != 16 {
        return if hr < 0 { hr } else { E_FAIL };
    }
    let magic = u32::from_le_bytes(payload[0..4].try_into().unwrap());
    if magic != FTM_MAGIC {
        return E_FAIL;
    }
    let pv = u64::from_le_bytes(payload[8..16].try_into().unwrap()) as usize as *mut c_void;
    if !pv.is_null() {
        com_call_release(pv);
    }
    S_OK
}

unsafe extern "win64" fn ftm_disconnect_object(_this: *mut c_void, _dw_reserved: u32) -> HRESULT {
    S_OK
}

static FTM_UNK_VTBL: IMarshalVtbl = IMarshalVtbl {
    query_interface: ftm_unk_qi,
    add_ref: ftm_unk_add_ref,
    release: ftm_unk_release,
    get_unmarshal_class: ftm_get_unmarshal_class,
    get_marshal_size_max: ftm_get_marshal_size_max,
    marshal_interface: ftm_marshal_interface,
    unmarshal_interface: ftm_unmarshal_interface,
    release_marshal_data: ftm_release_marshal_data,
    disconnect_object: ftm_disconnect_object,
};

static FTM_MARSHAL_VTBL: IMarshalVtbl = IMarshalVtbl {
    query_interface: ftm_marshal_qi,
    add_ref: ftm_marshal_add_ref,
    release: ftm_marshal_release,
    get_unmarshal_class: ftm_get_unmarshal_class,
    get_marshal_size_max: ftm_get_marshal_size_max,
    marshal_interface: ftm_marshal_interface,
    unmarshal_interface: ftm_unmarshal_interface,
    release_marshal_data: ftm_release_marshal_data,
    disconnect_object: ftm_disconnect_object,
};

fn register_ftm_vtables() {
    unsafe {
        crate::com::register_host_vtable(
            crate::com::IID_IMarshal,
            (&FTM_MARSHAL_VTBL as *const IMarshalVtbl) as *const usize,
            9,
        );
    }
    register_vtable_ptrs(&[
        ftm_unk_qi as *const () as usize,
        ftm_unk_add_ref as *const () as usize,
        ftm_unk_release as *const () as usize,
    ]);
}

pub unsafe extern "win64" fn CoCreateFreeThreadedMarshaler(
    outer: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    register_ftm_vtables();
    let obj = Box::into_raw(Box::new(FtmObj {
        unk_vtbl: &FTM_UNK_VTBL,
        marshal_vtbl: &FTM_MARSHAL_VTBL,
        refs: AtomicU32::new(1),
        outer,
    }));
    std::ptr::write_unaligned(out, obj as *mut c_void);
    S_OK
}

pub unsafe extern "win64" fn CoInitializeSecurity(
    _sec_desc: *mut c_void,
    _auth_svc: i32,
    _as_auth_svc: *mut c_void,
    _reserved1: *mut c_void,
    _authn_level: u32,
    _imp_level: u32,
    _auth_list: *mut c_void,
    _capabilities: u32,
    _reserved3: *mut c_void,
) -> HRESULT {
    S_OK
}

pub unsafe extern "win64" fn CoFreeUnusedLibraries() {}

// ---------------------------------------------------------------------------
// `IClassFactory` and `CoGetClassObject`
// ---------------------------------------------------------------------------

#[repr(C)]
struct IClassFactoryVtbl {
    query_interface: unsafe extern "win64" fn(*mut ClassFactoryObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut ClassFactoryObj) -> u32,
    release: unsafe extern "win64" fn(*mut ClassFactoryObj) -> u32,
    create_instance: unsafe extern "win64" fn(
        *mut ClassFactoryObj,
        *mut c_void,
        *const u8,
        *mut *mut c_void,
    ) -> HRESULT,
    lock_server: unsafe extern "win64" fn(*mut ClassFactoryObj, BOOL) -> HRESULT,
}

#[repr(C)]
struct ClassFactoryObj {
    vtbl: *const IClassFactoryVtbl,
    refs: AtomicU32,
    clsid: [u8; 16],
    ctx: u32,
}

static SERVER_LOCKS: AtomicU32 = AtomicU32::new(0);

unsafe extern "win64" fn cf_qi(
    this: *mut ClassFactoryObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid_bytes(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN_BYTES || iid == IID_ICLASSFACTORY_BYTES {
        (&*this).refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        crate::com::warn_qi_failed_once("IClassFactory", &crate::com::Guid::from_bytes(iid));
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn cf_add_ref(this: *mut ClassFactoryObj) -> u32 {
    (&*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn cf_release(this: *mut ClassFactoryObj) -> u32 {
    let remaining = (&*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if remaining == 0 {
        drop(Box::from_raw(this));
    }
    remaining
}

unsafe extern "win64" fn cf_create_instance(
    this: *mut ClassFactoryObj,
    outer: *mut c_void,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let obj = &*this;
    CoCreateInstance(obj.clsid.as_ptr(), outer, obj.ctx, riid, ppv)
}

unsafe extern "win64" fn cf_lock_server(_this: *mut ClassFactoryObj, f_lock: BOOL) -> HRESULT {
    if f_lock != 0 {
        SERVER_LOCKS.fetch_add(1, Ordering::Relaxed);
    } else {
        let _ = SERVER_LOCKS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
            Some(v.saturating_sub(1))
        });
    }
    S_OK
}

static CLASS_FACTORY_VTBL: IClassFactoryVtbl = IClassFactoryVtbl {
    query_interface: cf_qi,
    add_ref: cf_add_ref,
    release: cf_release,
    create_instance: cf_create_instance,
    lock_server: cf_lock_server,
};

fn register_class_factory_vtable() {
    unsafe {
        crate::com::register_host_vtable(
            crate::com::IID_IClassFactory,
            (&CLASS_FACTORY_VTBL as *const IClassFactoryVtbl) as *const usize,
            5,
        );
    }
}

unsafe fn is_registered_com_class(clsid: &[u8; 16]) -> bool {
    if *clsid == CLSID_MMDEVICEENUMERATOR_BYTES
        || *clsid == super::dinput8::CLSID_DIRECTINPUT8
        || *clsid == CLSID_INPROCFREEMARSHALER_BYTES
    {
        return true;
    }
    {
        let probe_iid = [0u8; 16];
        let mut tmp: *mut c_void = std::ptr::null_mut();
        if let Some(hr) = super::msctf::create_instance(
            clsid.as_ptr(),
            probe_iid.as_ptr(),
            std::ptr::null_mut(),
            &mut tmp,
        ) {
            if hr >= 0 && !tmp.is_null() {
                com_call_release(tmp);
            }
            return true;
        }
    }
    if let Ok(factories) = EXTRA_COM_FACTORIES.read() {
        let probe_iid = [0u8; 16];
        for factory in factories.iter().rev() {
            let mut tmp: *mut c_void = std::ptr::null_mut();
            if let Some(hr) = factory(clsid.as_ptr(), probe_iid.as_ptr(), std::ptr::null_mut(), &mut tmp) {
                if hr >= 0 && !tmp.is_null() {
                    com_call_release(tmp);
                }
                return true;
            }
        }
    }
    false
}

pub unsafe extern "win64" fn CoGetClassObject(
    rclsid: *const u8,
    ctx: u32,
    _server_info: *mut c_void,
    riid: *const u8,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let (Some(clsid), Some(iid)) = (read_guid_bytes(rclsid), read_guid_bytes(riid)) else {
        return E_INVALIDARG;
    };
    // 1. Registered in-proc class objects (CoRegisterClassObject): QI via call_guest8 outside lock.
    if let Some(reg_punk) = find_registered_class_object(&clsid, ctx) {
        return com_call_qi(reg_punk.as_ptr(), riid, out);
    }
    // 2. Built-in Rust in-process COM classes.
    if is_registered_com_class(&clsid) {
        if iid != IID_ICLASSFACTORY_BYTES && iid != IID_IUNKNOWN_BYTES {
            return E_NOINTERFACE;
        }
        register_class_factory_vtable();
        let factory = Box::into_raw(Box::new(ClassFactoryObj {
            vtbl: &CLASS_FACTORY_VTBL,
            refs: AtomicU32::new(1),
            clsid,
            ctx,
        }));
        std::ptr::write_unaligned(out, factory as *mut c_void);
        return S_OK;
    }
    // 3. Registered in-proc server DLL exporting DllGetClassObject.
    if let Some(hr) = try_inproc_dll_get_class_object(&clsid, ctx, riid, out) {
        return hr;
    }
    let clsid_g = crate::com::Guid::from_bytes(clsid);
    let iid_g = crate::com::Guid::from_bytes(iid);
    crate::com::warn_unknown_clsid_once("CoGetClassObject", Some(&clsid_g), Some(&iid_g));
    REGDB_E_CLASSNOTREG
}

pub unsafe extern "win64" fn CoTaskMemAlloc(n: usize) -> *mut c_void {
    crate::heap::alloc(n, false)
}

pub unsafe extern "win64" fn CoTaskMemRealloc(p: *mut c_void, n: usize) -> *mut c_void {
    if p.is_null() {
        return crate::heap::alloc(n, false);
    }
    crate::heap::realloc(p, n, false, false)
}

pub unsafe extern "win64" fn CoTaskMemFree(p: *mut c_void) {
    if !p.is_null() {
        crate::heap::free(p);
    }
}

pub unsafe extern "win64" fn StringFromGUID2(g: *const u8, buf: *mut u16, cch: i32) -> i32 {
    if g.is_null() || buf.is_null() || cch <= 0 {
        return 0;
    }
    let s = guid_string(g);
    if (cch as usize) < s.len() + 1 {
        return 0;
    }
    crate::strings::write_wide_trunc(&s, buf, cch as usize);
    (s.len() + 1) as i32
}

pub unsafe extern "win64" fn StringFromCLSID(rclsid: *const u8, lplpsz: *mut LPWSTR) -> HRESULT {
    if lplpsz.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(lplpsz, std::ptr::null_mut());
    if rclsid.is_null() {
        return E_INVALIDARG;
    }
    let s = guid_string(rclsid);
    let units: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let mem = CoTaskMemAlloc(units.len() * 2) as *mut u16;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    std::ptr::copy_nonoverlapping(units.as_ptr() as *const u8, mem as *mut u8, units.len() * 2);
    std::ptr::write_unaligned(lplpsz, mem);
    S_OK
}

// ---------------------------------------------------------------------------
// 14-slot Win64 COM `IStream` backed by `GlobalAlloc` / `GlobalReAlloc` / `GlobalFree`
// ---------------------------------------------------------------------------

#[repr(C)]
struct STATSTG {
    pwcs_name: LPWSTR,
    ty: u32,
    _pad0: u32,
    cb_size: u64,
    mtime: FILETIME,
    ctime: FILETIME,
    atime: FILETIME,
    grf_mode: u32,
    grf_locks_supported: u32,
    clsid: [u8; 16],
    grf_state_bits: u32,
    reserved: u32,
}
const _: () = assert!(std::mem::size_of::<STATSTG>() == 80);

#[repr(C)]
struct IStreamVtbl {
    query_interface: unsafe extern "win64" fn(*mut HGlobalStreamObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut HGlobalStreamObj) -> u32,
    release: unsafe extern "win64" fn(*mut HGlobalStreamObj) -> u32,
    read: unsafe extern "win64" fn(*mut HGlobalStreamObj, *mut c_void, u32, *mut u32) -> HRESULT,
    write: unsafe extern "win64" fn(*mut HGlobalStreamObj, *const c_void, u32, *mut u32) -> HRESULT,
    seek: unsafe extern "win64" fn(*mut HGlobalStreamObj, i64, u32, *mut u64) -> HRESULT,
    set_size: unsafe extern "win64" fn(*mut HGlobalStreamObj, u64) -> HRESULT,
    copy_to: unsafe extern "win64" fn(*mut HGlobalStreamObj, *mut c_void, u64, *mut u64, *mut u64) -> HRESULT,
    commit: unsafe extern "win64" fn(*mut HGlobalStreamObj, u32) -> HRESULT,
    revert: unsafe extern "win64" fn(*mut HGlobalStreamObj) -> HRESULT,
    lock_region: unsafe extern "win64" fn(*mut HGlobalStreamObj, u64, u64, u32) -> HRESULT,
    unlock_region: unsafe extern "win64" fn(*mut HGlobalStreamObj, u64, u64, u32) -> HRESULT,
    stat: unsafe extern "win64" fn(*mut HGlobalStreamObj, *mut u8, u32) -> HRESULT,
    clone: unsafe extern "win64" fn(*mut HGlobalStreamObj, *mut *mut c_void) -> HRESULT,
}
const _: () = assert!(std::mem::size_of::<IStreamVtbl>() == 14 * std::mem::size_of::<usize>());

struct HGlobalBacking {
    hglobal: usize,
    size: usize,
    capacity: usize,
    delete_on_release: bool,
    ctime: u64,
    mtime: u64,
    atime: u64,
}

impl Drop for HGlobalBacking {
    fn drop(&mut self) {
        if self.delete_on_release && self.hglobal != 0 {
            unsafe {
                crate::kernel32::memory::GlobalFree(self.hglobal as *mut c_void);
            }
            self.hglobal = 0;
        }
    }
}

impl HGlobalBacking {
    unsafe fn resize(&mut self, new_size: usize) -> Result<(), HRESULT> {
        let old_size = self.size;
        let alloc_bytes = new_size.max(1);
        let new_h = if self.hglobal == 0 {
            crate::kernel32::memory::GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, alloc_bytes)
        } else {
            crate::kernel32::memory::GlobalReAlloc(
                self.hglobal as *mut c_void,
                alloc_bytes,
                GMEM_MOVEABLE | GMEM_ZEROINIT,
            )
        };
        if new_h.is_null() {
            return Err(E_OUTOFMEMORY);
        }
        let new_cap = crate::kernel32::memory::GlobalSize(new_h).max(alloc_bytes);
        if new_size > old_size {
            let zero_end = new_size.min(self.capacity);
            if zero_end > old_size {
                let locked = crate::kernel32::memory::GlobalLock(new_h) as *mut u8;
                if !locked.is_null() {
                    std::ptr::write_bytes(locked.add(old_size), 0, zero_end - old_size);
                    crate::kernel32::memory::GlobalUnlock(new_h);
                }
            }
        }
        self.hglobal = new_h as usize;
        self.size = new_size;
        self.capacity = new_cap;
        self.mtime = now_filetime();
        Ok(())
    }
}

#[repr(C)]
struct HGlobalStreamObj {
    vtbl: *const IStreamVtbl,
    refs: AtomicU32,
    pos: AtomicU64,
    backing: Arc<Mutex<HGlobalBacking>>,
}

static LIVE_HGLOBAL_STREAMS: LazyLock<Mutex<HashSet<usize>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

fn register_stream_vtable() {
    unsafe {
        crate::com::register_host_vtable(
            crate::com::IID_IStream,
            (&ISTREAM_VTBL as *const IStreamVtbl) as *const usize,
            14,
        );
    }
}

fn new_hglobal_stream_obj(backing: Arc<Mutex<HGlobalBacking>>, initial_pos: u64) -> *mut HGlobalStreamObj {
    register_stream_vtable();
    let ptr = Box::into_raw(Box::new(HGlobalStreamObj {
        vtbl: &ISTREAM_VTBL,
        refs: AtomicU32::new(1),
        pos: AtomicU64::new(initial_pos),
        backing,
    }));
    LIVE_HGLOBAL_STREAMS.lock().unwrap().insert(ptr as usize);
    ptr
}

unsafe extern "win64" fn stream_qi(
    this: *mut HGlobalStreamObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid_bytes(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN_BYTES || iid == IID_ISEQUENTIALSTREAM_BYTES || iid == IID_ISTREAM_BYTES {
        (&*this).refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        crate::com::warn_qi_failed_once("IStream", &crate::com::Guid::from_bytes(iid));
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn stream_add_ref(this: *mut HGlobalStreamObj) -> u32 {
    (&*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn stream_release(this: *mut HGlobalStreamObj) -> u32 {
    let remaining = (&*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if remaining == 0 {
        LIVE_HGLOBAL_STREAMS.lock().unwrap().remove(&(this as usize));
        drop(Box::from_raw(this));
    }
    remaining
}

unsafe extern "win64" fn stream_read(
    this: *mut HGlobalStreamObj,
    pv: *mut c_void,
    cb: u32,
    pcb_read: *mut u32,
) -> HRESULT {
    if !pcb_read.is_null() {
        std::ptr::write_unaligned(pcb_read, 0);
    }
    if pv.is_null() && cb > 0 {
        return STG_E_INVALIDPOINTER;
    }
    let obj = &*this;
    let mut backing = obj.backing.lock().unwrap();
    let pos = obj.pos.load(Ordering::Acquire);
    let pos_usize = usize::try_from(pos).unwrap_or(usize::MAX);
    let avail = backing.size.saturating_sub(pos_usize);
    let to_read = (cb as usize).min(avail);
    if to_read > 0 && backing.hglobal != 0 {
        let locked = crate::kernel32::memory::GlobalLock(backing.hglobal as *mut c_void) as *const u8;
        if !locked.is_null() {
            std::ptr::copy_nonoverlapping(locked.add(pos_usize), pv as *mut u8, to_read);
            crate::kernel32::memory::GlobalUnlock(backing.hglobal as *mut c_void);
        }
    }
    obj.pos.store(pos.saturating_add(to_read as u64), Ordering::Release);
    backing.atime = now_filetime();
    if !pcb_read.is_null() {
        std::ptr::write_unaligned(pcb_read, to_read as u32);
    }
    if to_read == cb as usize {
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn stream_write(
    this: *mut HGlobalStreamObj,
    pv: *const c_void,
    cb: u32,
    pcb_written: *mut u32,
) -> HRESULT {
    if !pcb_written.is_null() {
        std::ptr::write_unaligned(pcb_written, 0);
    }
    if pv.is_null() && cb > 0 {
        return STG_E_INVALIDPOINTER;
    }
    let obj = &*this;
    let mut backing = obj.backing.lock().unwrap();
    let pos = obj.pos.load(Ordering::Acquire);
    let Some(end_u64) = pos.checked_add(cb as u64) else {
        return STG_E_MEDIUMFULL;
    };
    let Ok(pos_usize) = usize::try_from(pos) else {
        return STG_E_MEDIUMFULL;
    };
    let Ok(end_usize) = usize::try_from(end_u64) else {
        return STG_E_MEDIUMFULL;
    };
    if end_usize > backing.size {
        if let Err(hr) = backing.resize(end_usize) {
            return hr;
        }
    }
    if cb > 0 && backing.hglobal != 0 {
        let locked = crate::kernel32::memory::GlobalLock(backing.hglobal as *mut c_void) as *mut u8;
        if locked.is_null() {
            return E_OUTOFMEMORY;
        }
        std::ptr::copy_nonoverlapping(pv as *const u8, locked.add(pos_usize), cb as usize);
        crate::kernel32::memory::GlobalUnlock(backing.hglobal as *mut c_void);
    }
    obj.pos.store(end_u64, Ordering::Release);
    backing.mtime = now_filetime();
    if !pcb_written.is_null() {
        std::ptr::write_unaligned(pcb_written, cb);
    }
    S_OK
}

unsafe extern "win64" fn stream_seek(
    this: *mut HGlobalStreamObj,
    dlib_move: i64,
    dw_origin: u32,
    plib_new_position: *mut u64,
) -> HRESULT {
    let obj = &*this;
    let base: i128 = match dw_origin {
        STREAM_SEEK_SET => 0,
        STREAM_SEEK_CUR => obj.pos.load(Ordering::Acquire) as i128,
        STREAM_SEEK_END => obj.backing.lock().unwrap().size as i128,
        _ => return STG_E_INVALIDFUNCTION,
    };
    let target = base + (dlib_move as i128);
    if target < 0 || target > i64::MAX as i128 {
        return STG_E_INVALIDFUNCTION;
    }
    let new_pos = target as u64;
    obj.pos.store(new_pos, Ordering::Release);
    if !plib_new_position.is_null() {
        std::ptr::write_unaligned(plib_new_position, new_pos);
    }
    S_OK
}

unsafe extern "win64" fn stream_set_size(this: *mut HGlobalStreamObj, lib_new_size: u64) -> HRESULT {
    let Ok(ns) = usize::try_from(lib_new_size) else {
        return STG_E_MEDIUMFULL;
    };
    let obj = &*this;
    let mut backing = obj.backing.lock().unwrap();
    match backing.resize(ns) {
        Ok(()) => S_OK,
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn stream_copy_to(
    this: *mut HGlobalStreamObj,
    pstm: *mut c_void,
    cb: u64,
    pcb_read: *mut u64,
    pcb_written: *mut u64,
) -> HRESULT {
    if !pcb_read.is_null() {
        std::ptr::write_unaligned(pcb_read, 0);
    }
    if !pcb_written.is_null() {
        std::ptr::write_unaligned(pcb_written, 0);
    }
    if pstm.is_null() {
        return STG_E_INVALIDPOINTER;
    }
    let snapshot: Vec<u8> = {
        let obj = &*this;
        let backing = obj.backing.lock().unwrap();
        let pos = obj.pos.load(Ordering::Acquire);
        let pos_usize = usize::try_from(pos).unwrap_or(usize::MAX);
        let avail = backing.size.saturating_sub(pos_usize);
        let to_copy = (cb.min(avail as u64)) as usize;
        let mut buf = vec![0u8; to_copy];
        if to_copy > 0 && backing.hglobal != 0 {
            let locked = crate::kernel32::memory::GlobalLock(backing.hglobal as *mut c_void) as *const u8;
            if !locked.is_null() {
                std::ptr::copy_nonoverlapping(locked.add(pos_usize), buf.as_mut_ptr(), to_copy);
                crate::kernel32::memory::GlobalUnlock(backing.hglobal as *mut c_void);
            }
        }
        obj.pos.store(pos.saturating_add(to_copy as u64), Ordering::Release);
        buf
    };

    let total_read = snapshot.len() as u64;
    let mut total_written: u64 = 0;
    let mut offset = 0usize;
    let mut final_hr = S_OK;
    while offset < snapshot.len() {
        let chunk = (snapshot.len() - offset).min(u32::MAX as usize) as u32;
        let mut written: u32 = 0;
        let hr = com_stream_write(
            pstm,
            snapshot.as_ptr().add(offset) as *const c_void,
            chunk,
            &mut written,
        );
        total_written = total_written.saturating_add(written as u64);
        offset += chunk as usize;
        if hr < 0 {
            final_hr = hr;
            break;
        }
    }
    if !pcb_read.is_null() {
        std::ptr::write_unaligned(pcb_read, total_read);
    }
    if !pcb_written.is_null() {
        std::ptr::write_unaligned(pcb_written, total_written);
    }
    final_hr
}

unsafe extern "win64" fn stream_commit(_this: *mut HGlobalStreamObj, _grf_commit_flags: u32) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn stream_revert(_this: *mut HGlobalStreamObj) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn stream_lock_region(
    _this: *mut HGlobalStreamObj,
    _lib_offset: u64,
    _cb: u64,
    _dw_lock_type: u32,
) -> HRESULT {
    STG_E_INVALIDFUNCTION
}

unsafe extern "win64" fn stream_unlock_region(
    _this: *mut HGlobalStreamObj,
    _lib_offset: u64,
    _cb: u64,
    _dw_lock_type: u32,
) -> HRESULT {
    STG_E_INVALIDFUNCTION
}

unsafe extern "win64" fn stream_stat(
    this: *mut HGlobalStreamObj,
    pstatstg: *mut u8,
    grf_stat_flag: u32,
) -> HRESULT {
    if pstatstg.is_null() {
        return STG_E_INVALIDPOINTER;
    }
    if grf_stat_flag != STATFLAG_DEFAULT && grf_stat_flag != STATFLAG_NONAME {
        return STG_E_INVALIDFLAG;
    }
    let obj = &*this;
    let backing = obj.backing.lock().unwrap();
    let st = STATSTG {
        pwcs_name: std::ptr::null_mut(),
        ty: STGTY_STREAM,
        _pad0: 0,
        cb_size: backing.size as u64,
        mtime: FILETIME::from_u64(backing.mtime),
        ctime: FILETIME::from_u64(backing.ctime),
        atime: FILETIME::from_u64(backing.atime),
        grf_mode: STGM_READWRITE,
        grf_locks_supported: 0,
        clsid: [0u8; 16],
        grf_state_bits: 0,
        reserved: 0,
    };
    std::ptr::write_unaligned(pstatstg as *mut STATSTG, st);
    S_OK
}

unsafe extern "win64" fn stream_clone(this: *mut HGlobalStreamObj, ppstm: *mut *mut c_void) -> HRESULT {
    if ppstm.is_null() {
        return STG_E_INVALIDPOINTER;
    }
    std::ptr::write_unaligned(ppstm, std::ptr::null_mut());
    let obj = &*this;
    let cloned = new_hglobal_stream_obj(Arc::clone(&obj.backing), obj.pos.load(Ordering::Acquire));
    std::ptr::write_unaligned(ppstm, cloned as *mut c_void);
    S_OK
}

static ISTREAM_VTBL: IStreamVtbl = IStreamVtbl {
    query_interface: stream_qi,
    add_ref: stream_add_ref,
    release: stream_release,
    read: stream_read,
    write: stream_write,
    seek: stream_seek,
    set_size: stream_set_size,
    copy_to: stream_copy_to,
    commit: stream_commit,
    revert: stream_revert,
    lock_region: stream_lock_region,
    unlock_region: stream_unlock_region,
    stat: stream_stat,
    clone: stream_clone,
};

pub unsafe extern "win64" fn CreateStreamOnHGlobal(
    hglobal: *mut c_void,
    delete_on_release: BOOL,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let (h, size, capacity) = if hglobal.is_null() {
        let alloc = crate::kernel32::memory::GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, 0);
        if alloc.is_null() {
            return E_OUTOFMEMORY;
        }
        let cap = crate::kernel32::memory::GlobalSize(alloc).max(1);
        (alloc, 0usize, cap)
    } else {
        if !crate::heap::is_ours(hglobal) {
            return E_INVALIDARG;
        }
        let sz = crate::kernel32::memory::GlobalSize(hglobal);
        (hglobal, sz, sz)
    };
    let now = now_filetime();
    let backing = Arc::new(Mutex::new(HGlobalBacking {
        hglobal: h as usize,
        size,
        capacity,
        delete_on_release: delete_on_release != 0,
        ctime: now,
        mtime: now,
        atime: now,
    }));
    let stm = new_hglobal_stream_obj(backing, 0);
    std::ptr::write_unaligned(out, stm as *mut c_void);
    S_OK
}

pub unsafe extern "win64" fn GetHGlobalFromStream(
    pstm: *mut c_void,
    phglobal: *mut *mut c_void,
) -> HRESULT {
    if pstm.is_null() || phglobal.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(phglobal, std::ptr::null_mut());
    let is_ours = LIVE_HGLOBAL_STREAMS.lock().unwrap().contains(&(pstm as usize));
    if !is_ours {
        return E_INVALIDARG;
    }
    let obj = &*(pstm as *const HGlobalStreamObj);
    let h = obj.backing.lock().unwrap().hglobal as *mut c_void;
    std::ptr::write_unaligned(phglobal, h);
    S_OK
}

// ---------------------------------------------------------------------------
// `PropVariantClear`
// ---------------------------------------------------------------------------

/// Clears a `PROPVARIANT`, freeing BSTRs, releasing COM interfaces, freeing
/// `VT_VECTOR` buffers & inner elements, `SAFEARRAY`s, `CLIPDATA`, strings,
/// CLSIDs, and BLOBs.
pub unsafe extern "win64" fn PropVariantClear(pv: *mut u8) -> HRESULT {
    if pv.is_null() {
        return S_OK;
    }
    let vt = std::ptr::read_unaligned(pv as *const u16);

    const VT_EMPTY: u16 = 0;
    const VT_NULL: u16 = 1;
    const VT_I2: u16 = 2;
    const VT_I4: u16 = 3;
    const VT_R4: u16 = 4;
    const VT_R8: u16 = 5;
    const VT_CY: u16 = 6;
    const VT_DATE: u16 = 7;
    const VT_BSTR: u16 = 8;
    const VT_DISPATCH: u16 = 9;
    const VT_ERROR: u16 = 10;
    const VT_BOOL: u16 = 11;
    const VT_VARIANT: u16 = 12;
    const VT_UNKNOWN: u16 = 13;
    const VT_DECIMAL: u16 = 14;
    const VT_I1: u16 = 16;
    const VT_UI1: u16 = 17;
    const VT_UI2: u16 = 18;
    const VT_UI4: u16 = 19;
    const VT_I8: u16 = 20;
    const VT_UI8: u16 = 21;
    const VT_INT: u16 = 22;
    const VT_UINT: u16 = 23;
    const VT_LPSTR: u16 = 30;
    const VT_LPWSTR: u16 = 31;
    const VT_FILETIME: u16 = 64;
    const VT_BLOB: u16 = 65;
    const VT_STREAM: u16 = 66;
    const VT_STORAGE: u16 = 67;
    const VT_STREAMED_OBJECT: u16 = 68;
    const VT_STORED_OBJECT: u16 = 69;
    const VT_BLOB_OBJECT: u16 = 70;
    const VT_CF: u16 = 71;
    const VT_CLSID: u16 = 72;
    const VT_VERSIONED_STREAM: u16 = 73;
    const VT_BSTR_BLOB: u16 = 0x0FFF;
    const VT_VECTOR: u16 = 0x1000;
    const VT_ARRAY: u16 = 0x2000;
    const VT_BYREF: u16 = 0x4000;

    let mut hr = S_OK;

    if (vt & VT_BYREF) != 0 {
        // By-ref variants do not own their referenced target.
    } else if (vt & VT_VECTOR) != 0 {
        let base = vt & 0x0FFF;
        let c_elems = std::ptr::read_unaligned(pv.add(8) as *const u32) as usize;
        let p_elems = std::ptr::read_unaligned(pv.add(16) as *const *mut u8);
        if !p_elems.is_null() {
            match base {
                VT_BSTR => {
                    for i in 0..c_elems {
                        let bstr = std::ptr::read_unaligned(p_elems.add(i * 8) as *const *mut u16);
                        if !bstr.is_null() {
                            super::oleaut32::SysFreeString(bstr);
                        }
                    }
                }
                VT_LPSTR | VT_LPWSTR => {
                    for i in 0..c_elems {
                        let s = std::ptr::read_unaligned(p_elems.add(i * 8) as *const *mut c_void);
                        if !s.is_null() {
                            CoTaskMemFree(s);
                        }
                    }
                }
                VT_VARIANT => {
                    for i in 0..c_elems {
                        let elem_hr = PropVariantClear(p_elems.add(i * 24));
                        if elem_hr < 0 && hr == S_OK {
                            hr = elem_hr;
                        }
                    }
                }
                VT_CF => {
                    // CLIPDATA { cbSize: u32 @+0, ulClipFmt: i32 @+4, pClipData: *mut u8 @+8 }
                    for i in 0..c_elems {
                        let clip_data = std::ptr::read_unaligned(p_elems.add(i * 16 + 8) as *const *mut c_void);
                        if !clip_data.is_null() {
                            CoTaskMemFree(clip_data);
                        }
                    }
                }
                VT_UNKNOWN | VT_DISPATCH => {
                    for i in 0..c_elems {
                        let obj = std::ptr::read_unaligned(p_elems.add(i * 8) as *const *mut c_void);
                        if !obj.is_null() {
                            com_call_release(obj);
                        }
                    }
                }
                VT_I1 | VT_UI1 | VT_I2 | VT_UI2 | VT_I4 | VT_UI4 | VT_I8 | VT_UI8 | VT_R4 | VT_R8
                | VT_BOOL | VT_ERROR | VT_CY | VT_DATE | VT_FILETIME | VT_CLSID | VT_INT | VT_UINT => {}
                _ => {
                    hr = STG_E_INVALIDPARAMETER;
                }
            }
            CoTaskMemFree(p_elems as *mut c_void);
        }
    } else if (vt & VT_ARRAY) != 0 {
        let psa = std::ptr::read_unaligned(pv.add(8) as *const *mut u8);
        if !psa.is_null() {
            let sa_hr = super::oleaut32::SafeArrayDestroy(psa);
            if sa_hr < 0 {
                hr = sa_hr;
            }
        }
    } else {
        match vt {
            VT_EMPTY | VT_NULL | VT_I2 | VT_I4 | VT_R4 | VT_R8 | VT_CY | VT_DATE | VT_ERROR
            | VT_BOOL | VT_DECIMAL | VT_I1 | VT_UI1 | VT_UI2 | VT_UI4 | VT_I8 | VT_UI8 | VT_INT
            | VT_UINT | VT_FILETIME => {}
            VT_BSTR => {
                let bstr = std::ptr::read_unaligned(pv.add(8) as *const *mut u16);
                if !bstr.is_null() {
                    super::oleaut32::SysFreeString(bstr);
                }
            }
            VT_DISPATCH | VT_UNKNOWN | VT_STREAM | VT_STORAGE | VT_STREAMED_OBJECT | VT_STORED_OBJECT => {
                let obj = std::ptr::read_unaligned(pv.add(8) as *const *mut c_void);
                if !obj.is_null() {
                    com_call_release(obj);
                }
            }
            VT_LPSTR | VT_LPWSTR | VT_CLSID => {
                let p = std::ptr::read_unaligned(pv.add(8) as *const *mut c_void);
                CoTaskMemFree(p);
            }
            // BLOB { ULONG cbSize @+8; BYTE *pBlobData @+16 }
            VT_BLOB | VT_BLOB_OBJECT | VT_BSTR_BLOB => {
                let p = std::ptr::read_unaligned(pv.add(16) as *const *mut c_void);
                CoTaskMemFree(p);
            }
            VT_CF => {
                let clip = std::ptr::read_unaligned(pv.add(8) as *const *mut u8);
                if !clip.is_null() {
                    let clip_data = std::ptr::read_unaligned(clip.add(8) as *const *mut c_void);
                    CoTaskMemFree(clip_data);
                    CoTaskMemFree(clip as *mut c_void);
                }
            }
            VT_VERSIONED_STREAM => {
                let vs = std::ptr::read_unaligned(pv.add(8) as *const *mut u8);
                if !vs.is_null() {
                    let pstm = std::ptr::read_unaligned(vs.add(16) as *const *mut c_void);
                    if !pstm.is_null() {
                        com_call_release(pstm);
                    }
                    CoTaskMemFree(vs as *mut c_void);
                }
            }
            _ => {
                hr = STG_E_INVALIDPARAMETER;
            }
        }
    }

    std::ptr::write_bytes(pv, 0, 24);
    hr
}

// ---------------------------------------------------------------------------
// Per-HWND `IDropTarget` registration (`RegisterDragDrop` / `RevokeDragDrop`)
// ---------------------------------------------------------------------------

static DRAG_DROP_TARGETS: LazyLock<Mutex<HashMap<usize, usize>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub unsafe extern "win64" fn RegisterDragDrop(hwnd: HWND, p_drop_target: *mut c_void) -> HRESULT {
    if hwnd.is_null() {
        return DRAGDROP_E_INVALIDHWND;
    }
    if p_drop_target.is_null() {
        return E_INVALIDARG;
    }
    {
        let mut map = DRAG_DROP_TARGETS.lock().unwrap();
        if map.contains_key(&(hwnd as usize)) {
            return DRAGDROP_E_ALREADYREGISTERED;
        }
        map.insert(hwnd as usize, p_drop_target as usize);
    }
    com_call_add_ref(p_drop_target);
    S_OK
}

pub unsafe extern "win64" fn RevokeDragDrop(hwnd: HWND) -> HRESULT {
    if hwnd.is_null() {
        return DRAGDROP_E_INVALIDHWND;
    }
    let removed = DRAG_DROP_TARGETS.lock().unwrap().remove(&(hwnd as usize));
    match removed {
        Some(target) => {
            com_call_release(target as *mut c_void);
            S_OK
        }
        None => DRAGDROP_E_NOTREGISTERED,
    }
}

// ---------------------------------------------------------------------------
// `ReleaseStgMedium`
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn ReleaseStgMedium(pmedium: *mut c_void) {
    if pmedium.is_null() {
        return;
    }
    let m = pmedium as *mut u8;
    let tymed = std::ptr::read_unaligned(m as *const u32);
    let data = std::ptr::read_unaligned(m.add(8) as *const *mut c_void);
    let punk = std::ptr::read_unaligned(m.add(16) as *const *mut c_void);

    if !punk.is_null() {
        com_call_release(punk);
    } else {
        match tymed {
            TYMED_NULL => {}
            TYMED_HGLOBAL => {
                if !data.is_null() {
                    crate::kernel32::memory::GlobalFree(data);
                }
            }
            TYMED_FILE => {
                if !data.is_null() {
                    let win_path = crate::strings::from_wide(data as *const u16);
                    if let Some(host_path) = crate::path::to_host(&win_path) {
                        let _ = std::fs::remove_file(host_path);
                    }
                    if crate::heap::is_ours(data) {
                        CoTaskMemFree(data);
                    }
                }
            }
            TYMED_ISTREAM | TYMED_ISTORAGE => {
                if !data.is_null() {
                    com_call_release(data);
                }
            }
            _ => {}
        }
    }

    std::ptr::write_unaligned(m as *mut u32, TYMED_NULL);
    std::ptr::write_unaligned(m.add(8) as *mut usize, 0);
    std::ptr::write_unaligned(m.add(16) as *mut usize, 0);
}

// ---------------------------------------------------------------------------
// WinRT runtime activation (`Ro*`, `Windows*String*`) & real Foundation classes
// ---------------------------------------------------------------------------

const TYPE_E_TYPEMISMATCH: HRESULT = 0x8002_8CA0u32 as i32;

const TRUST_LEVEL_BASE: i32 = 0;

static WINRT_FACTORIES: LazyLock<Mutex<HashMap<String, Arc<crate::com::ComPtr>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Register an in-process WinRT activation factory for `class_name` (checked by
/// [`RoGetActivationFactory`] and [`RoActivateInstance`]).
#[allow(dead_code)]
pub fn register_winrt_activation_factory(class_name: impl Into<String>, factory: crate::com::ComPtr) {
    let old = WINRT_FACTORIES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(class_name.into(), Arc::new(factory));
    // Drop any previous factory strictly outside the lock.
    drop(old);
}

#[allow(dead_code)]
pub fn unregister_winrt_activation_factory(class_name: &str) {
    let old = WINRT_FACTORIES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(class_name);
    drop(old);
}

unsafe fn hstring_to_rust(h: usize) -> String {
    if h == 0 {
        return String::new();
    }
    let mut len: u32 = 0;
    let buf = WindowsGetStringRawBuffer(h, &mut len);
    if buf.is_null() || len == 0 {
        return String::new();
    }
    let mut units = Vec::with_capacity(len as usize);
    for i in 0..len as usize {
        units.push(std::ptr::read_unaligned(buf.add(i)));
    }
    String::from_utf16_lossy(&units)
}

unsafe fn rust_to_hstring(s: &str) -> Result<usize, HRESULT> {
    if s.is_empty() {
        return Ok(0);
    }
    let units: Vec<u16> = s.encode_utf16().collect();
    let mut out: usize = 0;
    let hr = WindowsCreateString(units.as_ptr(), units.len() as u32, &mut out);
    if hr < 0 { Err(hr) } else { Ok(out) }
}

unsafe fn write_iids(iid_count: *mut u32, iids: *mut *mut crate::com::Guid, list: &[crate::com::Guid]) -> HRESULT {
    if iid_count.is_null() || iids.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(iid_count, 0);
    std::ptr::write_unaligned(iids, std::ptr::null_mut());
    if list.is_empty() {
        return S_OK;
    }
    let byte_len = list.len() * std::mem::size_of::<crate::com::Guid>();
    let mem = CoTaskMemAlloc(byte_len) as *mut crate::com::Guid;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    for (i, g) in list.iter().enumerate() {
        std::ptr::write_unaligned(mem.add(i), *g);
    }
    std::ptr::write_unaligned(iid_count, list.len() as u32);
    std::ptr::write_unaligned(iids, mem);
    S_OK
}

// --- Windows.Foundation.PropertyValue (`IPropertyValue` + `IPropertyValueStatics`) ---

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WinRtPropertyType {
    Empty = 0,
    UInt8 = 1,
    Int16 = 2,
    UInt16 = 3,
    Int32 = 4,
    UInt32 = 5,
    Int64 = 6,
    UInt64 = 7,
    Single = 8,
    Double = 9,
    Char16 = 10,
    Boolean = 11,
    String = 12,
    Inspectable = 13,
    DateTime = 14,
    TimeSpan = 15,
    Guid = 16,
    Point = 17,
    Size = 18,
    Rect = 19,
    UInt8Array = 1025,
    Int16Array = 1026,
    UInt16Array = 1027,
    Int32Array = 1028,
    UInt32Array = 1029,
    Int64Array = 1030,
    UInt64Array = 1031,
    SingleArray = 1032,
    DoubleArray = 1033,
    Char16Array = 1034,
    BooleanArray = 1035,
    StringArray = 1036,
    InspectableArray = 1037,
    DateTimeArray = 1038,
    TimeSpanArray = 1039,
    GuidArray = 1040,
    PointArray = 1041,
    SizeArray = 1042,
    RectArray = 1043,
}

enum PropValData {
    Empty,
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Single(f32),
    Double(f64),
    Char16(u16),
    Boolean(bool),
    String(String),
    #[allow(dead_code)]
    Inspectable(Option<crate::com::ComPtr>),
    DateTime(i64),
    TimeSpan(i64),
    Guid(crate::com::Guid),
    Point([f32; 2]),
    Size([f32; 2]),
    Rect([f32; 4]),
    UInt8Array(Vec<u8>),
    Int16Array(Vec<i16>),
    UInt16Array(Vec<u16>),
    Int32Array(Vec<i32>),
    UInt32Array(Vec<u32>),
    Int64Array(Vec<i64>),
    UInt64Array(Vec<u64>),
    SingleArray(Vec<f32>),
    DoubleArray(Vec<f64>),
    Char16Array(Vec<u16>),
    BooleanArray(Vec<u8>),
    StringArray(Vec<String>),
    InspectableArray(Vec<Option<crate::com::ComPtr>>),
    GuidArray(Vec<crate::com::Guid>),
    DateTimeArray(Vec<i64>),
    TimeSpanArray(Vec<i64>),
    PointArray(Vec<[f32; 2]>),
    SizeArray(Vec<[f32; 2]>),
    RectArray(Vec<[f32; 4]>),
}

impl PropValData {
    fn prop_type(&self) -> WinRtPropertyType {
        match self {
            Self::Empty => WinRtPropertyType::Empty,
            Self::UInt8(_) => WinRtPropertyType::UInt8,
            Self::Int16(_) => WinRtPropertyType::Int16,
            Self::UInt16(_) => WinRtPropertyType::UInt16,
            Self::Int32(_) => WinRtPropertyType::Int32,
            Self::UInt32(_) => WinRtPropertyType::UInt32,
            Self::Int64(_) => WinRtPropertyType::Int64,
            Self::UInt64(_) => WinRtPropertyType::UInt64,
            Self::Single(_) => WinRtPropertyType::Single,
            Self::Double(_) => WinRtPropertyType::Double,
            Self::Char16(_) => WinRtPropertyType::Char16,
            Self::Boolean(_) => WinRtPropertyType::Boolean,
            Self::String(_) => WinRtPropertyType::String,
            Self::Inspectable(_) => WinRtPropertyType::Inspectable,
            Self::DateTime(_) => WinRtPropertyType::DateTime,
            Self::TimeSpan(_) => WinRtPropertyType::TimeSpan,
            Self::Guid(_) => WinRtPropertyType::Guid,
            Self::Point(_) => WinRtPropertyType::Point,
            Self::Size(_) => WinRtPropertyType::Size,
            Self::Rect(_) => WinRtPropertyType::Rect,
            Self::UInt8Array(_) => WinRtPropertyType::UInt8Array,
            Self::Int16Array(_) => WinRtPropertyType::Int16Array,
            Self::UInt16Array(_) => WinRtPropertyType::UInt16Array,
            Self::Int32Array(_) => WinRtPropertyType::Int32Array,
            Self::UInt32Array(_) => WinRtPropertyType::UInt32Array,
            Self::Int64Array(_) => WinRtPropertyType::Int64Array,
            Self::UInt64Array(_) => WinRtPropertyType::UInt64Array,
            Self::SingleArray(_) => WinRtPropertyType::SingleArray,
            Self::DoubleArray(_) => WinRtPropertyType::DoubleArray,
            Self::Char16Array(_) => WinRtPropertyType::Char16Array,
            Self::BooleanArray(_) => WinRtPropertyType::BooleanArray,
            Self::StringArray(_) => WinRtPropertyType::StringArray,
            Self::InspectableArray(_) => WinRtPropertyType::InspectableArray,
            Self::GuidArray(_) => WinRtPropertyType::GuidArray,
            Self::DateTimeArray(_) => WinRtPropertyType::DateTimeArray,
            Self::TimeSpanArray(_) => WinRtPropertyType::TimeSpanArray,
            Self::PointArray(_) => WinRtPropertyType::PointArray,
            Self::SizeArray(_) => WinRtPropertyType::SizeArray,
            Self::RectArray(_) => WinRtPropertyType::RectArray,
        }
    }

    fn is_numeric_scalar(&self) -> bool {
        matches!(
            self,
            Self::UInt8(_)
                | Self::Int16(_)
                | Self::UInt16(_)
                | Self::Int32(_)
                | Self::UInt32(_)
                | Self::Int64(_)
                | Self::UInt64(_)
                | Self::Single(_)
                | Self::Double(_)
        )
    }
}

type PropValObj = crate::com::ComObj<PropValData>;

unsafe extern "win64" fn pv_qi(
    this: *mut PropValObj,
    riid: *const crate::com::Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    crate::com::qi_helper(
        this,
        riid,
        ppv,
        &[crate::com::IID_IInspectable, crate::com::IID_IPropertyValue],
    )
}

unsafe extern "win64" fn pv_get_iids(
    _this: *mut PropValObj,
    iid_count: *mut u32,
    iids: *mut *mut crate::com::Guid,
) -> HRESULT {
    write_iids(iid_count, iids, &[crate::com::IID_IPropertyValue])
}

unsafe extern "win64" fn pv_get_runtime_class_name(
    _this: *mut PropValObj,
    class_name: *mut usize,
) -> HRESULT {
    if class_name.is_null() {
        return E_POINTER;
    }
    match rust_to_hstring("Windows.Foundation.PropertyValue") {
        Ok(h) => {
            std::ptr::write_unaligned(class_name, h);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn pv_get_trust_level(_this: *mut PropValObj, trust: *mut i32) -> HRESULT {
    if trust.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(trust, TRUST_LEVEL_BASE);
    S_OK
}

unsafe extern "win64" fn pv_get_type(this: *mut PropValObj, out: *mut i32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, crate::com::ComObj::inner(this).prop_type() as i32);
    S_OK
}

unsafe extern "win64" fn pv_get_is_numeric(this: *mut PropValObj, out: *mut u8) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, crate::com::ComObj::inner(this).is_numeric_scalar() as u8);
    S_OK
}

unsafe extern "win64" fn pv_get_u8(this: *mut PropValObj, out: *mut u8) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::UInt8(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_i16(this: *mut PropValObj, out: *mut i16) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Int16(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_u16(this: *mut PropValObj, out: *mut u16) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::UInt16(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_i32(this: *mut PropValObj, out: *mut i32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Int32(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_u32(this: *mut PropValObj, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::UInt32(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_i64(this: *mut PropValObj, out: *mut i64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Int64(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_u64(this: *mut PropValObj, out: *mut u64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::UInt64(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_single(this: *mut PropValObj, out: *mut f32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Single(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_double(this: *mut PropValObj, out: *mut f64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Double(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_char16(this: *mut PropValObj, out: *mut u16) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Char16(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_boolean(this: *mut PropValObj, out: *mut u8) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Boolean(v) => {
            std::ptr::write_unaligned(out, *v as u8);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_string(this: *mut PropValObj, out: *mut usize) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, 0);
    match crate::com::ComObj::inner(this) {
        PropValData::String(s) => match rust_to_hstring(s) {
            Ok(h) => {
                std::ptr::write_unaligned(out, h);
                S_OK
            }
            Err(hr) => hr,
        },
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_guid(this: *mut PropValObj, out: *mut crate::com::Guid) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Guid(g) => {
            std::ptr::write_unaligned(out, *g);
            S_OK
        }
        _ => TYPE_E_TYPEMISMATCH,
    }
}

unsafe extern "win64" fn pv_get_date_time(this: *mut PropValObj, out: *mut i64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::DateTime(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => {
            std::ptr::write_unaligned(out, 0);
            TYPE_E_TYPEMISMATCH
        }
    }
}

unsafe extern "win64" fn pv_get_time_span(this: *mut PropValObj, out: *mut i64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::TimeSpan(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => {
            std::ptr::write_unaligned(out, 0);
            TYPE_E_TYPEMISMATCH
        }
    }
}

unsafe extern "win64" fn pv_get_point(this: *mut PropValObj, out: *mut [f32; 2]) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Point(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => {
            std::ptr::write_unaligned(out, [0.0; 2]);
            TYPE_E_TYPEMISMATCH
        }
    }
}

unsafe extern "win64" fn pv_get_size(this: *mut PropValObj, out: *mut [f32; 2]) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Size(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => {
            std::ptr::write_unaligned(out, [0.0; 2]);
            TYPE_E_TYPEMISMATCH
        }
    }
}

unsafe extern "win64" fn pv_get_rect(this: *mut PropValObj, out: *mut [f32; 4]) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match crate::com::ComObj::inner(this) {
        PropValData::Rect(v) => {
            std::ptr::write_unaligned(out, *v);
            S_OK
        }
        _ => {
            std::ptr::write_unaligned(out, [0.0; 4]);
            TYPE_E_TYPEMISMATCH
        }
    }
}

unsafe fn pv_write_pod_array<T: Copy>(
    items: &[T],
    out_len: *mut u32,
    out_ptr: *mut *mut c_void,
) -> HRESULT {
    if items.is_empty() {
        std::ptr::write_unaligned(out_len, 0);
        std::ptr::write_unaligned(out_ptr, std::ptr::null_mut());
        return S_OK;
    }
    let Some(byte_len) = items.len().checked_mul(std::mem::size_of::<T>()) else {
        return E_OUTOFMEMORY;
    };
    let mem = CoTaskMemAlloc(byte_len) as *mut T;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    for (i, item) in items.iter().enumerate() {
        std::ptr::write_unaligned(mem.add(i), *item);
    }
    std::ptr::write_unaligned(out_len, items.len() as u32);
    std::ptr::write_unaligned(out_ptr, mem as *mut c_void);
    S_OK
}

macro_rules! define_pv_pod_array_getter {
    ($fn_name:ident, $variant:ident) => {
        unsafe extern "win64" fn $fn_name(
            this: *mut PropValObj,
            out_len: *mut u32,
            out_ptr: *mut *mut c_void,
        ) -> HRESULT {
            if !out_len.is_null() {
                std::ptr::write_unaligned(out_len, 0);
            }
            if !out_ptr.is_null() {
                std::ptr::write_unaligned(out_ptr, std::ptr::null_mut());
            }
            if out_len.is_null() || out_ptr.is_null() {
                return E_POINTER;
            }
            match crate::com::ComObj::inner(this) {
                PropValData::$variant(items) => pv_write_pod_array(items, out_len, out_ptr),
                _ => TYPE_E_TYPEMISMATCH,
            }
        }
    };
}

define_pv_pod_array_getter!(pv_get_u8_array, UInt8Array);
define_pv_pod_array_getter!(pv_get_i16_array, Int16Array);
define_pv_pod_array_getter!(pv_get_u16_array, UInt16Array);
define_pv_pod_array_getter!(pv_get_i32_array, Int32Array);
define_pv_pod_array_getter!(pv_get_u32_array, UInt32Array);
define_pv_pod_array_getter!(pv_get_i64_array, Int64Array);
define_pv_pod_array_getter!(pv_get_u64_array, UInt64Array);
define_pv_pod_array_getter!(pv_get_single_array, SingleArray);
define_pv_pod_array_getter!(pv_get_double_array, DoubleArray);
define_pv_pod_array_getter!(pv_get_char16_array, Char16Array);
define_pv_pod_array_getter!(pv_get_boolean_array, BooleanArray);
define_pv_pod_array_getter!(pv_get_guid_array, GuidArray);
define_pv_pod_array_getter!(pv_get_date_time_array, DateTimeArray);
define_pv_pod_array_getter!(pv_get_time_span_array, TimeSpanArray);
define_pv_pod_array_getter!(pv_get_point_array, PointArray);
define_pv_pod_array_getter!(pv_get_size_array, SizeArray);
define_pv_pod_array_getter!(pv_get_rect_array, RectArray);

unsafe extern "win64" fn pv_get_string_array(
    this: *mut PropValObj,
    out_len: *mut u32,
    out_ptr: *mut *mut c_void,
) -> HRESULT {
    if !out_len.is_null() {
        std::ptr::write_unaligned(out_len, 0);
    }
    if !out_ptr.is_null() {
        std::ptr::write_unaligned(out_ptr, std::ptr::null_mut());
    }
    if out_len.is_null() || out_ptr.is_null() {
        return E_POINTER;
    }
    let PropValData::StringArray(items) = crate::com::ComObj::inner(this) else {
        return TYPE_E_TYPEMISMATCH;
    };
    if items.is_empty() {
        return S_OK;
    }
    let Some(byte_len) = items.len().checked_mul(std::mem::size_of::<usize>()) else {
        return E_OUTOFMEMORY;
    };
    let mem = CoTaskMemAlloc(byte_len) as *mut usize;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    let mut created = 0usize;
    for s in items {
        match rust_to_hstring(s) {
            Ok(h) => {
                std::ptr::write_unaligned(mem.add(created), h);
                created += 1;
            }
            Err(hr) => {
                for j in 0..created {
                    let h = std::ptr::read_unaligned(mem.add(j));
                    WindowsDeleteString(h);
                }
                CoTaskMemFree(mem as *mut c_void);
                return hr;
            }
        }
    }
    std::ptr::write_unaligned(out_len, items.len() as u32);
    std::ptr::write_unaligned(out_ptr, mem as *mut c_void);
    S_OK
}

unsafe extern "win64" fn pv_get_inspectable_array(
    this: *mut PropValObj,
    out_len: *mut u32,
    out_ptr: *mut *mut c_void,
) -> HRESULT {
    if !out_len.is_null() {
        std::ptr::write_unaligned(out_len, 0);
    }
    if !out_ptr.is_null() {
        std::ptr::write_unaligned(out_ptr, std::ptr::null_mut());
    }
    if out_len.is_null() || out_ptr.is_null() {
        return E_POINTER;
    }
    let PropValData::InspectableArray(items) = crate::com::ComObj::inner(this) else {
        return TYPE_E_TYPEMISMATCH;
    };
    if items.is_empty() {
        return S_OK;
    }
    let Some(byte_len) = items.len().checked_mul(std::mem::size_of::<*mut c_void>()) else {
        return E_OUTOFMEMORY;
    };
    let mem = CoTaskMemAlloc(byte_len) as *mut *mut c_void;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    for (i, item) in items.iter().enumerate() {
        let ptr = match item {
            Some(cp) => {
                crate::com::com_call_add_ref(cp.as_ptr());
                cp.as_ptr()
            }
            None => std::ptr::null_mut(),
        };
        std::ptr::write_unaligned(mem.add(i), ptr);
    }
    std::ptr::write_unaligned(out_len, items.len() as u32);
    std::ptr::write_unaligned(out_ptr, mem as *mut c_void);
    S_OK
}

fn pv_vtbl() -> *const usize {
    static VT: std::sync::OnceLock<[usize; 45]> = std::sync::OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                pv_qi as *const () as usize,
                crate::com::com_add_ref::<PropValData> as *const () as usize,
                crate::com::com_release::<PropValData> as *const () as usize,
                pv_get_iids as *const () as usize,
                pv_get_runtime_class_name as *const () as usize,
                pv_get_trust_level as *const () as usize,
                pv_get_type as *const () as usize,
                pv_get_is_numeric as *const () as usize,
                pv_get_u8 as *const () as usize,
                pv_get_i16 as *const () as usize,
                pv_get_u16 as *const () as usize,
                pv_get_i32 as *const () as usize,
                pv_get_u32 as *const () as usize,
                pv_get_i64 as *const () as usize,
                pv_get_u64 as *const () as usize,
                pv_get_single as *const () as usize,
                pv_get_double as *const () as usize,
                pv_get_char16 as *const () as usize,
                pv_get_boolean as *const () as usize,
                pv_get_string as *const () as usize,
                pv_get_guid as *const () as usize,
                pv_get_date_time as *const () as usize,
                pv_get_time_span as *const () as usize,
                pv_get_point as *const () as usize,
                pv_get_size as *const () as usize,
                pv_get_rect as *const () as usize,
                pv_get_u8_array as *const () as usize,
                pv_get_i16_array as *const () as usize,
                pv_get_u16_array as *const () as usize,
                pv_get_i32_array as *const () as usize,
                pv_get_u32_array as *const () as usize,
                pv_get_i64_array as *const () as usize,
                pv_get_u64_array as *const () as usize,
                pv_get_single_array as *const () as usize,
                pv_get_double_array as *const () as usize,
                pv_get_char16_array as *const () as usize,
                pv_get_boolean_array as *const () as usize,
                pv_get_string_array as *const () as usize,
                pv_get_inspectable_array as *const () as usize,
                pv_get_guid_array as *const () as usize,
                pv_get_date_time_array as *const () as usize,
                pv_get_time_span_array as *const () as usize,
                pv_get_point_array as *const () as usize,
                pv_get_size_array as *const () as usize,
                pv_get_rect_array as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { crate::com::register_host_vtable(crate::com::IID_IPropertyValue, ptr, 45) };
    ptr
}

fn new_prop_val(data: PropValData) -> *mut c_void {
    crate::com::ComObj::new(pv_vtbl(), data) as *mut c_void
}

/// Dual-interface static factory for `Windows.Foundation.PropertyValue`:
/// offset 0 = `IActivationFactory`, offset 8 = `IPropertyValueStatics`.
#[repr(C)]
struct PropValFactoryObj {
    act_vtbl: *const usize,
    statics_vtbl: *const usize,
    refs: AtomicU32,
}

unsafe fn pvf_from_statics(iface: *mut c_void) -> *mut PropValFactoryObj {
    (iface as *mut u8).sub(std::mem::size_of::<*const usize>()) as *mut PropValFactoryObj
}

unsafe extern "win64" fn pvf_act_qi(
    this: *mut PropValFactoryObj,
    riid: *const crate::com::Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(ppv);
    let Some(iid) = crate::com::read_guid(riid) else {
        return E_POINTER;
    };
    if iid == crate::com::IID_IUnknown
        || iid == crate::com::IID_IInspectable
        || iid == crate::com::IID_IActivationFactory
    {
        (*this).refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else if iid == crate::com::IID_IPropertyValueStatics {
        (*this).refs.fetch_add(1, Ordering::AcqRel);
        let sec = std::ptr::addr_of!((*this).statics_vtbl) as *mut c_void;
        std::ptr::write_unaligned(ppv, sec);
        S_OK
    } else {
        crate::com::warn_qi_failed_once("WinRTActivationFactory", &iid);
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn pvf_act_add_ref(this: *mut PropValFactoryObj) -> u32 {
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn pvf_act_release(this: *mut PropValFactoryObj) -> u32 {
    let rem = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}

unsafe extern "win64" fn pvf_get_iids(
    _this: *mut c_void,
    iid_count: *mut u32,
    iids: *mut *mut crate::com::Guid,
) -> HRESULT {
    write_iids(
        iid_count,
        iids,
        &[crate::com::IID_IActivationFactory, crate::com::IID_IPropertyValueStatics],
    )
}

unsafe extern "win64" fn pvf_get_runtime_class_name(_this: *mut c_void, class_name: *mut usize) -> HRESULT {
    if class_name.is_null() {
        return E_POINTER;
    }
    match rust_to_hstring("Windows.Foundation.PropertyValue") {
        Ok(h) => {
            std::ptr::write_unaligned(class_name, h);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn pvf_get_trust_level(_this: *mut c_void, trust: *mut i32) -> HRESULT {
    if trust.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(trust, TRUST_LEVEL_BASE);
    S_OK
}

unsafe extern "win64" fn pvf_activate_instance(_this: *mut PropValFactoryObj, out: *mut *mut c_void) -> HRESULT {
    crate::com::write_null_out(out);
    E_NOTIMPL
}

unsafe extern "win64" fn pvs_qi(
    this: *mut c_void,
    riid: *const crate::com::Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    pvf_act_qi(pvf_from_statics(this), riid, ppv)
}

unsafe extern "win64" fn pvs_add_ref(this: *mut c_void) -> u32 {
    pvf_act_add_ref(pvf_from_statics(this))
}

unsafe extern "win64" fn pvs_release(this: *mut c_void) -> u32 {
    pvf_act_release(pvf_from_statics(this))
}

unsafe extern "win64" fn pvs_create_empty(_this: *mut c_void, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Empty));
    S_OK
}

unsafe extern "win64" fn pvs_create_u8(_this: *mut c_void, v: u8, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::UInt8(v)));
    S_OK
}

unsafe extern "win64" fn pvs_create_i16(_this: *mut c_void, v: i16, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Int16(v)));
    S_OK
}

unsafe extern "win64" fn pvs_create_u16(_this: *mut c_void, v: u16, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::UInt16(v)));
    S_OK
}

unsafe extern "win64" fn pvs_create_i32(_this: *mut c_void, v: i32, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Int32(v)));
    S_OK
}

unsafe extern "win64" fn pvs_create_u32(_this: *mut c_void, v: u32, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::UInt32(v)));
    S_OK
}

unsafe extern "win64" fn pvs_create_i64(_this: *mut c_void, v: i64, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Int64(v)));
    S_OK
}

unsafe extern "win64" fn pvs_create_u64(_this: *mut c_void, v: u64, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::UInt64(v)));
    S_OK
}

unsafe extern "win64" fn pvs_create_single(_this: *mut c_void, v: f32, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Single(v)));
    S_OK
}
crate::win64_bridge!(fn pvs_create_single(*mut c_void, f32, *mut *mut c_void) -> HRESULT);

unsafe extern "win64" fn pvs_create_double(_this: *mut c_void, v: f64, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Double(v)));
    S_OK
}
crate::win64_bridge!(fn pvs_create_double(*mut c_void, f64, *mut *mut c_void) -> HRESULT);

unsafe extern "win64" fn pvs_create_char16(_this: *mut c_void, v: u16, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Char16(v)));
    S_OK
}

unsafe extern "win64" fn pvs_create_boolean(_this: *mut c_void, v: u8, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Boolean(v != 0)));
    S_OK
}

unsafe extern "win64" fn pvs_create_string(_this: *mut c_void, h: usize, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let s = hstring_to_rust(h);
    std::ptr::write_unaligned(out, new_prop_val(PropValData::String(s)));
    S_OK
}

unsafe extern "win64" fn pvs_create_inspectable(
    _this: *mut c_void,
    val: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let retained = crate::com::ComPtr::retain(val);
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Inspectable(retained)));
    S_OK
}

unsafe extern "win64" fn pvs_create_guid(
    _this: *mut c_void,
    g_ptr: *const crate::com::Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(g) = crate::com::read_guid(g_ptr) else {
        return E_POINTER;
    };
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Guid(g)));
    S_OK
}

fn pvf_act_vtbl() -> *const usize {
    static VT: std::sync::OnceLock<[usize; 7]> = std::sync::OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                pvf_act_qi as *const () as usize,
                pvf_act_add_ref as *const () as usize,
                pvf_act_release as *const () as usize,
                pvf_get_iids as *const () as usize,
                pvf_get_runtime_class_name as *const () as usize,
                pvf_get_trust_level as *const () as usize,
                pvf_activate_instance as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { crate::com::register_host_vtable(crate::com::IID_IActivationFactory, ptr, 7) };
    ptr
}

unsafe extern "win64" fn pvs_create_date_time(
    _this: *mut c_void,
    v: i64,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::DateTime(v)));
    S_OK
}

unsafe extern "win64" fn pvs_create_time_span(
    _this: *mut c_void,
    v: i64,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::TimeSpan(v)));
    S_OK
}

/// Win64 passes the 8-byte `Windows.Foundation.Point { f32 x, f32 y }` struct by value in a GPR.
unsafe extern "win64" fn pvs_create_point(
    _this: *mut c_void,
    point_bits: u64,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let x = f32::from_bits(point_bits as u32);
    let y = f32::from_bits((point_bits >> 32) as u32);
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Point([x, y])));
    S_OK
}

/// Win64 passes the 8-byte `Windows.Foundation.Size { f32 w, f32 h }` struct by value in a GPR.
unsafe extern "win64" fn pvs_create_size(
    _this: *mut c_void,
    size_bits: u64,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let w = f32::from_bits(size_bits as u32);
    let h = f32::from_bits((size_bits >> 32) as u32);
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Size([w, h])));
    S_OK
}

/// Win64 passes the 16-byte `Windows.Foundation.Rect` struct indirectly by pointer.
unsafe extern "win64" fn pvs_create_rect(
    _this: *mut c_void,
    rect_ptr: *const [f32; 4],
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(out);
    if rect_ptr.is_null() {
        return E_POINTER;
    }
    let rect = std::ptr::read_unaligned(rect_ptr);
    std::ptr::write_unaligned(out, new_prop_val(PropValData::Rect(rect)));
    S_OK
}

unsafe fn pvs_read_pod_array<T: Copy>(len: u32, data: *const c_void) -> Vec<T> {
    let n = len as usize;
    let mut out = Vec::with_capacity(n);
    let src = data as *const T;
    for i in 0..n {
        out.push(std::ptr::read_unaligned(src.add(i)));
    }
    out
}

macro_rules! define_pvs_pod_array_creator {
    ($fn_name:ident, $variant:ident, $elem:ty) => {
        unsafe extern "win64" fn $fn_name(
            _this: *mut c_void,
            len: u32,
            data: *const c_void,
            out: *mut *mut c_void,
        ) -> HRESULT {
            if out.is_null() {
                return E_POINTER;
            }
            crate::com::write_null_out(out);
            if len > 0 && data.is_null() {
                return E_POINTER;
            }
            let items = pvs_read_pod_array::<$elem>(len, data);
            std::ptr::write_unaligned(out, new_prop_val(PropValData::$variant(items)));
            S_OK
        }
    };
}

define_pvs_pod_array_creator!(pvs_create_u8_array, UInt8Array, u8);
define_pvs_pod_array_creator!(pvs_create_i16_array, Int16Array, i16);
define_pvs_pod_array_creator!(pvs_create_u16_array, UInt16Array, u16);
define_pvs_pod_array_creator!(pvs_create_i32_array, Int32Array, i32);
define_pvs_pod_array_creator!(pvs_create_u32_array, UInt32Array, u32);
define_pvs_pod_array_creator!(pvs_create_i64_array, Int64Array, i64);
define_pvs_pod_array_creator!(pvs_create_u64_array, UInt64Array, u64);
define_pvs_pod_array_creator!(pvs_create_single_array, SingleArray, f32);
define_pvs_pod_array_creator!(pvs_create_double_array, DoubleArray, f64);
define_pvs_pod_array_creator!(pvs_create_char16_array, Char16Array, u16);
define_pvs_pod_array_creator!(pvs_create_guid_array, GuidArray, crate::com::Guid);
define_pvs_pod_array_creator!(pvs_create_date_time_array, DateTimeArray, i64);
define_pvs_pod_array_creator!(pvs_create_time_span_array, TimeSpanArray, i64);
define_pvs_pod_array_creator!(pvs_create_point_array, PointArray, [f32; 2]);
define_pvs_pod_array_creator!(pvs_create_size_array, SizeArray, [f32; 2]);
define_pvs_pod_array_creator!(pvs_create_rect_array, RectArray, [f32; 4]);

unsafe extern "win64" fn pvs_create_boolean_array(
    _this: *mut c_void,
    len: u32,
    data: *const c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(out);
    if len > 0 && data.is_null() {
        return E_POINTER;
    }
    let raw = pvs_read_pod_array::<u8>(len, data);
    let normalized = raw.into_iter().map(|b| u8::from(b != 0)).collect();
    std::ptr::write_unaligned(out, new_prop_val(PropValData::BooleanArray(normalized)));
    S_OK
}

unsafe extern "win64" fn pvs_create_string_array(
    _this: *mut c_void,
    len: u32,
    data: *const c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(out);
    if len > 0 && data.is_null() {
        return E_POINTER;
    }
    let n = len as usize;
    let src = data as *const usize;
    let mut items = Vec::with_capacity(n);
    for i in 0..n {
        let h = std::ptr::read_unaligned(src.add(i));
        items.push(hstring_to_rust(h));
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::StringArray(items)));
    S_OK
}

unsafe extern "win64" fn pvs_create_inspectable_array(
    _this: *mut c_void,
    len: u32,
    data: *const c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(out);
    if len > 0 && data.is_null() {
        return E_POINTER;
    }
    let n = len as usize;
    let src = data as *const *mut c_void;
    let mut items = Vec::with_capacity(n);
    for i in 0..n {
        let elem = std::ptr::read_unaligned(src.add(i));
        items.push(crate::com::ComPtr::retain(elem));
    }
    std::ptr::write_unaligned(out, new_prop_val(PropValData::InspectableArray(items)));
    S_OK
}

fn pvs_vtbl() -> *const usize {
    static VT: std::sync::OnceLock<[usize; 45]> = std::sync::OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                pvs_qi as *const () as usize,
                pvs_add_ref as *const () as usize,
                pvs_release as *const () as usize,
                pvf_get_iids as *const () as usize,
                pvf_get_runtime_class_name as *const () as usize,
                pvf_get_trust_level as *const () as usize,
                pvs_create_empty as *const () as usize,
                pvs_create_u8 as *const () as usize,
                pvs_create_i16 as *const () as usize,
                pvs_create_u16 as *const () as usize,
                pvs_create_i32 as *const () as usize,
                pvs_create_u32 as *const () as usize,
                pvs_create_i64 as *const () as usize,
                pvs_create_u64 as *const () as usize,
                crate::bridge_entry!(pvs_create_single) as *const () as usize,
                crate::bridge_entry!(pvs_create_double) as *const () as usize,
                pvs_create_char16 as *const () as usize,
                pvs_create_boolean as *const () as usize,
                pvs_create_string as *const () as usize,
                pvs_create_inspectable as *const () as usize,
                pvs_create_guid as *const () as usize,
                pvs_create_date_time as *const () as usize,
                pvs_create_time_span as *const () as usize,
                pvs_create_point as *const () as usize,
                pvs_create_size as *const () as usize,
                pvs_create_rect as *const () as usize,
                pvs_create_u8_array as *const () as usize,
                pvs_create_i16_array as *const () as usize,
                pvs_create_u16_array as *const () as usize,
                pvs_create_i32_array as *const () as usize,
                pvs_create_u32_array as *const () as usize,
                pvs_create_i64_array as *const () as usize,
                pvs_create_u64_array as *const () as usize,
                pvs_create_single_array as *const () as usize,
                pvs_create_double_array as *const () as usize,
                pvs_create_char16_array as *const () as usize,
                pvs_create_boolean_array as *const () as usize,
                pvs_create_string_array as *const () as usize,
                pvs_create_inspectable_array as *const () as usize,
                pvs_create_guid_array as *const () as usize,
                pvs_create_date_time_array as *const () as usize,
                pvs_create_time_span_array as *const () as usize,
                pvs_create_point_array as *const () as usize,
                pvs_create_size_array as *const () as usize,
                pvs_create_rect_array as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { crate::com::register_host_vtable(crate::com::IID_IPropertyValueStatics, ptr, 45) };
    ptr
}

unsafe fn create_property_value_factory(riid: *const crate::com::Guid, out: *mut *mut c_void) -> HRESULT {
    let f = Box::into_raw(Box::new(PropValFactoryObj {
        act_vtbl: pvf_act_vtbl(),
        statics_vtbl: pvs_vtbl(),
        refs: AtomicU32::new(1),
    }));
    let hr = pvf_act_qi(f, riid, out);
    pvf_act_release(f);
    hr
}

// --- Windows.Foundation.Uri (`IUriRuntimeClass` + `IUriRuntimeClassFactory`) ---

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedUri {
    raw: String,
    scheme: String,
    user: String,
    password: String,
    host: String,
    port: i32,
    path: String,
    query: String,
    fragment: String,
}

fn remove_dot_segments(path: &str) -> String {
    let mut input = path;
    let mut out = String::with_capacity(path.len());
    while !input.is_empty() {
        if let Some(rest) = input.strip_prefix("../") {
            input = rest;
        } else if let Some(rest) = input.strip_prefix("./") {
            input = rest;
        } else if input.starts_with("/./") {
            input = &input[2..];
        } else if input == "/." {
            input = "/";
        } else if input.starts_with("/../") {
            input = &input[3..];
            if let Some(slash) = out.rfind('/') {
                out.truncate(slash);
            } else {
                out.clear();
            }
        } else if input == "/.." {
            input = "/";
            if let Some(slash) = out.rfind('/') {
                out.truncate(slash);
            } else {
                out.clear();
            }
        } else if input == "." || input == ".." {
            input = "";
        } else {
            let seg_end = if let Some(after_leading) = input.strip_prefix('/') {
                match after_leading.find('/') {
                    Some(i) => i + 1,
                    None => input.len(),
                }
            } else {
                input.find('/').unwrap_or(input.len())
            };
            out.push_str(&input[..seg_end]);
            input = &input[seg_end..];
        }
    }
    out
}

impl ParsedUri {
    fn parse(raw: &str) -> Result<Self, HRESULT> {
        let trimmed = raw.trim();
        let Some(colon) = trimmed.find(':') else {
            return Err(E_INVALIDARG);
        };
        let scheme = trimmed[..colon].to_ascii_lowercase();
        if scheme.is_empty()
            || !scheme.bytes().next().is_some_and(|b| b.is_ascii_alphabetic())
            || !scheme.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'-' | b'.'))
        {
            return Err(E_INVALIDARG);
        }
        let mut rest = &trimmed[colon + 1..];
        let (rest_no_frag, fragment) = match rest.find('#') {
            Some(i) => (&rest[..i], rest[i..].to_string()),
            None => (rest, String::new()),
        };
        let (rest_no_query, query) = match rest_no_frag.find('?') {
            Some(i) => (&rest_no_frag[..i], rest_no_frag[i..].to_string()),
            None => (rest_no_frag, String::new()),
        };
        rest = rest_no_query;

        let default_port = match scheme.as_str() {
            "http" | "ws" => 80,
            "https" | "wss" => 443,
            "ftp" => 21,
            _ => 0,
        };

        let (mut user, mut password, mut host, mut port) = (String::new(), String::new(), String::new(), default_port);
        let path: String;
        if let Some(after_slashes) = rest.strip_prefix("//") {
            let (authority, p) = match after_slashes.find('/') {
                Some(i) => (&after_slashes[..i], &after_slashes[i..]),
                None => (after_slashes, "/"),
            };
            path = p.to_string();
            let hostport = match authority.rfind('@') {
                Some(at) => {
                    let userinfo = &authority[..at];
                    match userinfo.find(':') {
                        Some(c) => {
                            user = userinfo[..c].to_string();
                            password = userinfo[c + 1..].to_string();
                        }
                        None => user = userinfo.to_string(),
                    }
                    &authority[at + 1..]
                }
                None => authority,
            };
            if hostport.starts_with('[') {
                // IPv6 literal
                if let Some(end) = hostport.find(']') {
                    host = hostport[..=end].to_ascii_lowercase();
                    if let Some(pstr) = hostport[end + 1..].strip_prefix(':') {
                        port = pstr.parse::<u16>().map(|v| v as i32).map_err(|_| E_INVALIDARG)?;
                    }
                } else {
                    return Err(E_INVALIDARG);
                }
            } else if let Some(c) = hostport.rfind(':') {
                host = hostport[..c].to_ascii_lowercase();
                let pstr = &hostport[c + 1..];
                if !pstr.is_empty() {
                    port = pstr.parse::<u16>().map(|v| v as i32).map_err(|_| E_INVALIDARG)?;
                }
            } else {
                host = hostport.to_ascii_lowercase();
            }
        } else {
            path = rest.to_string();
        }

        Ok(Self {
            raw: trimmed.to_string(),
            scheme,
            user,
            password,
            host,
            port,
            path,
            query,
            fragment,
        })
    }

    fn absolute_uri(&self) -> String {
        let mut s = format!("{}:", self.scheme);
        if !self.host.is_empty() {
            s.push_str("//");
            if !self.user.is_empty() {
                s.push_str(&self.user);
                if !self.password.is_empty() {
                    s.push(':');
                    s.push_str(&self.password);
                }
                s.push('@');
            }
            s.push_str(&self.host);
            let default_port = match self.scheme.as_str() {
                "http" | "ws" => 80,
                "https" | "wss" => 443,
                "ftp" => 21,
                _ => 0,
            };
            if self.port != 0 && self.port != default_port {
                s.push(':');
                s.push_str(&self.port.to_string());
            }
        }
        s.push_str(&self.path);
        s.push_str(&self.query);
        s.push_str(&self.fragment);
        s
    }

    fn extension(&self) -> String {
        let seg = self.path.rsplit('/').next().unwrap_or("");
        match seg.rfind('.') {
            Some(dot) if dot > 0 => seg[dot..].to_string(),
            _ => String::new(),
        }
    }

    fn combine(&self, rel: &str) -> Result<Self, HRESULT> {
        let rel = rel.trim();
        let (rel_path, rel_suffix) = match rel.find(['?', '#']) {
            Some(i) => (&rel[..i], &rel[i..]),
            None => (rel, ""),
        };
        if rel_path.is_empty() {
            let base_prefix = if rel_suffix.starts_with('?') {
                match self.raw.find(['?', '#']) {
                    Some(i) => &self.raw[..i],
                    None => self.raw.as_str(),
                }
            } else {
                match self.raw.find('#') {
                    Some(i) => &self.raw[..i],
                    None => self.raw.as_str(),
                }
            };
            let combined = format!("{base_prefix}{rel_suffix}");
            return Self::parse(&combined);
        }
        if rel_path.starts_with("//") {
            let combined = format!("{}:{rel}", self.scheme);
            return Self::parse(&combined);
        }
        // RFC 3986 §3.1 / §4.2: a ':' in the first path segment (before any '/', '?', '#')
        // denotes a scheme; delegate to Self::parse so valid schemes succeed and malformed
        // scheme candidates fail honestly with E_INVALIDARG.
        let first_seg = match rel_path.find('/') {
            Some(i) => &rel_path[..i],
            None => rel_path,
        };
        if first_seg.contains(':') {
            return Self::parse(rel);
        }
        let (base_origin, has_authority) = match self.raw.find(':') {
            Some(colon) => {
                let after_colon = &self.raw[colon + 1..];
                if let Some(after_slashes) = after_colon.strip_prefix("//") {
                    let auth_len = after_slashes.find(['/', '?', '#']).unwrap_or(after_slashes.len());
                    (&self.raw[..colon + 3 + auth_len], true)
                } else {
                    (&self.raw[..=colon], false)
                }
            }
            None => (self.raw.as_str(), false),
        };
        let merged_path = if rel_path.starts_with('/') {
            rel_path.to_string()
        } else {
            let base_dir = match self.path.rfind('/') {
                Some(i) => &self.path[..=i],
                None if has_authority => "/",
                None => "",
            };
            format!("{base_dir}{rel_path}")
        };
        let clean_path = remove_dot_segments(&merged_path);
        let combined = format!("{base_origin}{clean_path}{rel_suffix}");
        Self::parse(&combined)
    }
}

type UriObj = crate::com::ComObj<ParsedUri>;

unsafe extern "win64" fn uri_qi(
    this: *mut UriObj,
    riid: *const crate::com::Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    crate::com::qi_helper(
        this,
        riid,
        ppv,
        &[crate::com::IID_IInspectable, crate::com::IID_IUriRuntimeClass],
    )
}

unsafe extern "win64" fn uri_get_iids(
    _this: *mut UriObj,
    iid_count: *mut u32,
    iids: *mut *mut crate::com::Guid,
) -> HRESULT {
    write_iids(iid_count, iids, &[crate::com::IID_IUriRuntimeClass])
}

unsafe extern "win64" fn uri_get_runtime_class_name(_this: *mut UriObj, class_name: *mut usize) -> HRESULT {
    if class_name.is_null() {
        return E_POINTER;
    }
    match rust_to_hstring("Windows.Foundation.Uri") {
        Ok(h) => {
            std::ptr::write_unaligned(class_name, h);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn uri_get_trust_level(_this: *mut UriObj, trust: *mut i32) -> HRESULT {
    if trust.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(trust, TRUST_LEVEL_BASE);
    S_OK
}

unsafe fn write_hstring_out(s: &str, out: *mut usize) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    match rust_to_hstring(s) {
        Ok(h) => {
            std::ptr::write_unaligned(out, h);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn uri_get_absolute(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).absolute_uri(), out)
}
unsafe extern "win64" fn uri_get_display(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).absolute_uri(), out)
}
unsafe extern "win64" fn uri_get_domain(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).host, out)
}
unsafe extern "win64" fn uri_get_extension(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).extension(), out)
}
unsafe extern "win64" fn uri_get_fragment(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).fragment, out)
}
unsafe extern "win64" fn uri_get_host(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).host, out)
}
unsafe extern "win64" fn uri_get_password(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).password, out)
}
unsafe extern "win64" fn uri_get_path(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).path, out)
}
unsafe extern "win64" fn uri_get_query(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).query, out)
}
unsafe extern "win64" fn uri_get_query_parsed(_this: *mut UriObj, out: *mut *mut c_void) -> HRESULT {
    crate::com::write_null_out(out);
    E_NOTIMPL
}
unsafe extern "win64" fn uri_get_raw(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).raw, out)
}
unsafe extern "win64" fn uri_get_scheme(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).scheme, out)
}
unsafe extern "win64" fn uri_get_user(this: *mut UriObj, out: *mut usize) -> HRESULT {
    write_hstring_out(&crate::com::ComObj::inner(this).user, out)
}
unsafe extern "win64" fn uri_get_port(this: *mut UriObj, out: *mut i32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, crate::com::ComObj::inner(this).port);
    S_OK
}
unsafe extern "win64" fn uri_get_suspicious(_this: *mut UriObj, out: *mut u8) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, 0);
    S_OK
}
unsafe extern "win64" fn uri_equals(this: *mut UriObj, other: *mut c_void, out: *mut u8) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let eq = match crate::com::com_from_iface::<ParsedUri>(other, uri_vtbl()) {
        Some(o) => crate::com::ComObj::inner(this).absolute_uri() == o.inner.absolute_uri(),
        None => false,
    };
    std::ptr::write_unaligned(out, eq as u8);
    S_OK
}
unsafe extern "win64" fn uri_combine(
    this: *mut UriObj,
    rel: usize,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(out);
    let rel_s = hstring_to_rust(rel);
    match crate::com::ComObj::inner(this).combine(&rel_s) {
        Ok(u) => {
            std::ptr::write_unaligned(out, crate::com::ComObj::new(uri_vtbl(), u) as *mut c_void);
            S_OK
        }
        Err(hr) => hr,
    }
}

fn uri_vtbl() -> *const usize {
    static VT: std::sync::OnceLock<[usize; 23]> = std::sync::OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                uri_qi as *const () as usize,
                crate::com::com_add_ref::<ParsedUri> as *const () as usize,
                crate::com::com_release::<ParsedUri> as *const () as usize,
                uri_get_iids as *const () as usize,
                uri_get_runtime_class_name as *const () as usize,
                uri_get_trust_level as *const () as usize,
                uri_get_absolute as *const () as usize,
                uri_get_display as *const () as usize,
                uri_get_domain as *const () as usize,
                uri_get_extension as *const () as usize,
                uri_get_fragment as *const () as usize,
                uri_get_host as *const () as usize,
                uri_get_password as *const () as usize,
                uri_get_path as *const () as usize,
                uri_get_query as *const () as usize,
                uri_get_query_parsed as *const () as usize,
                uri_get_raw as *const () as usize,
                uri_get_scheme as *const () as usize,
                uri_get_user as *const () as usize,
                uri_get_port as *const () as usize,
                uri_get_suspicious as *const () as usize,
                uri_equals as *const () as usize,
                uri_combine as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { crate::com::register_host_vtable(crate::com::IID_IUriRuntimeClass, ptr, 23) };
    ptr
}

#[repr(C)]
struct UriFactoryObj {
    act_vtbl: *const usize,
    uri_factory_vtbl: *const usize,
    refs: AtomicU32,
}

unsafe fn urif_from_sec(iface: *mut c_void) -> *mut UriFactoryObj {
    (iface as *mut u8).sub(std::mem::size_of::<*const usize>()) as *mut UriFactoryObj
}

unsafe extern "win64" fn urif_act_qi(
    this: *mut UriFactoryObj,
    riid: *const crate::com::Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(ppv);
    let Some(iid) = crate::com::read_guid(riid) else {
        return E_POINTER;
    };
    if iid == crate::com::IID_IUnknown
        || iid == crate::com::IID_IInspectable
        || iid == crate::com::IID_IActivationFactory
    {
        (*this).refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else if iid == crate::com::IID_IUriRuntimeClassFactory {
        (*this).refs.fetch_add(1, Ordering::AcqRel);
        let sec = std::ptr::addr_of!((*this).uri_factory_vtbl) as *mut c_void;
        std::ptr::write_unaligned(ppv, sec);
        S_OK
    } else {
        crate::com::warn_qi_failed_once("WinRTActivationFactory", &iid);
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn urif_act_add_ref(this: *mut UriFactoryObj) -> u32 {
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn urif_act_release(this: *mut UriFactoryObj) -> u32 {
    let rem = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if rem == 0 {
        drop(Box::from_raw(this));
    }
    rem
}

unsafe extern "win64" fn urif_get_iids(
    _this: *mut c_void,
    iid_count: *mut u32,
    iids: *mut *mut crate::com::Guid,
) -> HRESULT {
    write_iids(
        iid_count,
        iids,
        &[crate::com::IID_IActivationFactory, crate::com::IID_IUriRuntimeClassFactory],
    )
}

unsafe extern "win64" fn urif_get_runtime_class_name(_this: *mut c_void, class_name: *mut usize) -> HRESULT {
    write_hstring_out("Windows.Foundation.Uri", class_name)
}

unsafe extern "win64" fn urif_get_trust_level(_this: *mut c_void, trust: *mut i32) -> HRESULT {
    if trust.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(trust, TRUST_LEVEL_BASE);
    S_OK
}

unsafe extern "win64" fn urif_activate_instance(_this: *mut UriFactoryObj, out: *mut *mut c_void) -> HRESULT {
    crate::com::write_null_out(out);
    E_NOTIMPL
}

unsafe extern "win64" fn urif_sec_qi(
    this: *mut c_void,
    riid: *const crate::com::Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    urif_act_qi(urif_from_sec(this), riid, ppv)
}

unsafe extern "win64" fn urif_sec_add_ref(this: *mut c_void) -> u32 {
    urif_act_add_ref(urif_from_sec(this))
}

unsafe extern "win64" fn urif_sec_release(this: *mut c_void) -> u32 {
    urif_act_release(urif_from_sec(this))
}

unsafe extern "win64" fn urif_create_uri(
    _this: *mut c_void,
    uri_hstr: usize,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(out);
    let raw = hstring_to_rust(uri_hstr);
    match ParsedUri::parse(&raw) {
        Ok(u) => {
            std::ptr::write_unaligned(out, crate::com::ComObj::new(uri_vtbl(), u) as *mut c_void);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn urif_create_with_relative_uri(
    _this: *mut c_void,
    base_hstr: usize,
    rel_hstr: usize,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(out);
    let base_raw = hstring_to_rust(base_hstr);
    let rel_raw = hstring_to_rust(rel_hstr);
    let Ok(base) = ParsedUri::parse(&base_raw) else {
        return E_INVALIDARG;
    };
    match base.combine(&rel_raw) {
        Ok(u) => {
            std::ptr::write_unaligned(out, crate::com::ComObj::new(uri_vtbl(), u) as *mut c_void);
            S_OK
        }
        Err(hr) => hr,
    }
}

fn urif_act_vtbl() -> *const usize {
    static VT: std::sync::OnceLock<[usize; 7]> = std::sync::OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                urif_act_qi as *const () as usize,
                urif_act_add_ref as *const () as usize,
                urif_act_release as *const () as usize,
                urif_get_iids as *const () as usize,
                urif_get_runtime_class_name as *const () as usize,
                urif_get_trust_level as *const () as usize,
                urif_activate_instance as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { crate::com::register_host_vtable(crate::com::IID_IActivationFactory, ptr, 7) };
    ptr
}

fn urif_sec_vtbl() -> *const usize {
    static VT: std::sync::OnceLock<[usize; 8]> = std::sync::OnceLock::new();
    let ptr = VT
        .get_or_init(|| {
            [
                urif_sec_qi as *const () as usize,
                urif_sec_add_ref as *const () as usize,
                urif_sec_release as *const () as usize,
                urif_get_iids as *const () as usize,
                urif_get_runtime_class_name as *const () as usize,
                urif_get_trust_level as *const () as usize,
                urif_create_uri as *const () as usize,
                urif_create_with_relative_uri as *const () as usize,
            ]
        })
        .as_ptr();
    unsafe { crate::com::register_host_vtable(crate::com::IID_IUriRuntimeClassFactory, ptr, 8) };
    ptr
}

unsafe fn create_uri_factory(riid: *const crate::com::Guid, out: *mut *mut c_void) -> HRESULT {
    let f = Box::into_raw(Box::new(UriFactoryObj {
        act_vtbl: urif_act_vtbl(),
        uri_factory_vtbl: urif_sec_vtbl(),
        refs: AtomicU32::new(1),
    }));
    let hr = urif_act_qi(f, riid, out);
    urif_act_release(f);
    hr
}

pub unsafe extern "win64" fn RoInitialize(_init_type: u32) -> HRESULT {
    com_init()
}

pub unsafe extern "win64" fn RoUninitialize() {
    com_uninit()
}

pub unsafe extern "win64" fn RoGetActivationFactory(
    activatable_class_id: usize,
    iid: *const u8,
    factory: *mut *mut c_void,
) -> HRESULT {
    if factory.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(factory, std::ptr::null_mut());
    if iid.is_null() {
        return E_INVALIDARG;
    }
    let class_name = hstring_to_rust(activatable_class_id);
    if class_name.is_empty() {
        return REGDB_E_CLASSNOTREG;
    }

    // 1. Dynamically registered WinRT activation factories (outside lock before QI).
    let custom = WINRT_FACTORIES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&class_name)
        .cloned();
    if let Some(f) = custom {
        return com_call_qi(f.as_ptr(), iid, factory);
    }

    // 2. Real built-in Rust WinRT Foundation classes.
    match class_name.as_str() {
        "Windows.Foundation.PropertyValue" => {
            create_property_value_factory(iid as *const crate::com::Guid, factory)
        }
        "Windows.Foundation.Uri" => {
            create_uri_factory(iid as *const crate::com::Guid, factory)
        }
        _ => {
            crate::com::warn_unknown_winrt_class_once("RoGetActivationFactory", &class_name);
            REGDB_E_CLASSNOTREG
        }
    }
}

pub unsafe extern "win64" fn RoActivateInstance(
    activatable_class_id: usize,
    instance: *mut *mut c_void,
) -> HRESULT {
    if instance.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(instance, std::ptr::null_mut());
    let mut factory: *mut c_void = std::ptr::null_mut();
    let act_iid = crate::com::IID_IActivationFactory.to_bytes();
    let hr = RoGetActivationFactory(activatable_class_id, act_iid.as_ptr(), &mut factory);
    if hr < 0 || factory.is_null() {
        return if hr < 0 { hr } else { REGDB_E_CLASSNOTREG };
    }
    let ahr = crate::com::com_invoke_hresult(
        factory,
        &crate::com::IID_IActivationFactory,
        6, // IActivationFactory::ActivateInstance
        &[instance as u64],
    );
    com_call_release(factory);
    ahr
}

pub unsafe extern "win64" fn WindowsDuplicateString(string: usize, new_string: *mut usize) -> HRESULT {
    if new_string.is_null() {
        return E_INVALIDARG;
    }
    if string == 0 {
        std::ptr::write_unaligned(new_string, 0);
        return S_OK;
    }
    let mut len: u32 = 0;
    let buf = WindowsGetStringRawBuffer(string, &mut len);
    WindowsCreateString(buf, len, new_string)
}

pub unsafe extern "win64" fn WindowsGetStringLen(string: usize) -> u32 {
    if string == 0 {
        return 0;
    }
    std::ptr::read_unaligned(string as *const u32)
}

pub unsafe extern "win64" fn WindowsIsStringEmpty(string: usize) -> BOOL {
    (WindowsGetStringLen(string) == 0) as BOOL
}

pub unsafe extern "win64" fn WindowsCreateString(
    source_string: LPCWSTR,
    length: u32,
    string: *mut usize,
) -> HRESULT {
    if string.is_null() {
        return E_INVALIDARG;
    }
    if length == 0 {
        std::ptr::write_unaligned(string, 0);
        return S_OK;
    }
    if source_string.is_null() {
        std::ptr::write_unaligned(string, 0);
        return E_POINTER;
    }
    let bytes = 4 + (length as usize + 1) * 2;
    let p = crate::heap::alloc(bytes, true) as *mut u8;
    if p.is_null() {
        std::ptr::write_unaligned(string, 0);
        return E_OUTOFMEMORY;
    }
    std::ptr::write_unaligned(p as *mut u32, length);
    std::ptr::copy_nonoverlapping(source_string as *const u8, p.add(4), length as usize * 2);
    std::ptr::write_unaligned(p.add(4 + length as usize * 2) as *mut u16, 0);
    std::ptr::write_unaligned(string, p as usize);
    S_OK
}

pub unsafe extern "win64" fn WindowsCreateStringReference(
    source_string: LPCWSTR,
    length: u32,
    hstring_header: *mut u8,
    string: *mut usize,
) -> HRESULT {
    if string.is_null() || hstring_header.is_null() {
        return E_INVALIDARG;
    }
    WindowsCreateString(source_string, length, string)
}

pub unsafe extern "win64" fn WindowsDeleteString(string: usize) -> HRESULT {
    if string != 0 {
        crate::heap::free(string as *mut c_void);
    }
    S_OK
}

pub unsafe extern "win64" fn WindowsGetStringRawBuffer(string: usize, length: *mut u32) -> LPCWSTR {
    static EMPTY: u16 = 0;
    if string == 0 {
        if !length.is_null() {
            std::ptr::write_unaligned(length, 0);
        }
        return &EMPTY;
    }
    let p = string as *const u8;
    if !length.is_null() {
        std::ptr::write_unaligned(length, std::ptr::read_unaligned(p as *const u32));
    }
    p.add(4) as LPCWSTR
}

fn parse_guid_str(s: &str) -> Option<[u8; 16]> {
    let s = s.trim();
    let s = s.strip_prefix('{').and_then(|s| s.strip_suffix('}')).unwrap_or(s);
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5
        || parts[0].len() != 8
        || parts[1].len() != 4
        || parts[2].len() != 4
        || parts[3].len() != 4
        || parts[4].len() != 12
    {
        return None;
    }
    let d1 = u32::from_str_radix(parts[0], 16).ok()?;
    let d2 = u16::from_str_radix(parts[1], 16).ok()?;
    let d3 = u16::from_str_radix(parts[2], 16).ok()?;
    let d4_hi = u16::from_str_radix(parts[3], 16).ok()?;
    let d4_lo = u64::from_str_radix(parts[4], 16).ok()?;
    let mut g = [0u8; 16];
    g[0..4].copy_from_slice(&d1.to_le_bytes());
    g[4..6].copy_from_slice(&d2.to_le_bytes());
    g[6..8].copy_from_slice(&d3.to_le_bytes());
    g[8..10].copy_from_slice(&d4_hi.to_be_bytes());
    let lo_bytes = d4_lo.to_be_bytes();
    g[10..16].copy_from_slice(&lo_bytes[2..8]);
    Some(g)
}

pub unsafe extern "win64" fn CLSIDFromString(lpsz: LPCWSTR, pclsid: *mut u8) -> HRESULT {
    if pclsid.is_null() {
        return E_INVALIDARG;
    }
    if lpsz.is_null() {
        std::ptr::write_bytes(pclsid, 0, 16);
        return S_OK;
    }
    let s = crate::strings::from_wide(lpsz);
    match parse_guid_str(&s) {
        Some(g) => {
            std::ptr::copy_nonoverlapping(g.as_ptr(), pclsid, 16);
            S_OK
        }
        None => {
            std::ptr::write_bytes(pclsid, 0, 16);
            CO_E_CLASSSTRING
        }
    }
}

pub unsafe extern "win64" fn IIDFromString(lpsz: LPCWSTR, lpiid: *mut u8) -> HRESULT {
    if lpiid.is_null() {
        return E_INVALIDARG;
    }
    if lpsz.is_null() {
        std::ptr::write_bytes(lpiid, 0, 16);
        return E_INVALIDARG;
    }
    let s = crate::strings::from_wide(lpsz);
    match parse_guid_str(&s) {
        Some(g) => {
            std::ptr::copy_nonoverlapping(g.as_ptr(), lpiid, 16);
            S_OK
        }
        None => {
            std::ptr::write_bytes(lpiid, 0, 16);
            E_INVALIDARG
        }
    }
}

// ---------------------------------------------------------------------------
// Static IMalloc singleton for CoGetMalloc
// ---------------------------------------------------------------------------

#[repr(C)]
struct IMallocVtbl {
    query_interface: unsafe extern "win64" fn(*mut IMallocObj, *const u8, *mut *mut c_void) -> HRESULT,
    add_ref: unsafe extern "win64" fn(*mut IMallocObj) -> u32,
    release: unsafe extern "win64" fn(*mut IMallocObj) -> u32,
    alloc: unsafe extern "win64" fn(*mut IMallocObj, usize) -> *mut c_void,
    realloc: unsafe extern "win64" fn(*mut IMallocObj, *mut c_void, usize) -> *mut c_void,
    free: unsafe extern "win64" fn(*mut IMallocObj, *mut c_void),
    get_size: unsafe extern "win64" fn(*mut IMallocObj, *mut c_void) -> usize,
    did_alloc: unsafe extern "win64" fn(*mut IMallocObj, *mut c_void) -> i32,
    heap_minimize: unsafe extern "win64" fn(*mut IMallocObj),
}

#[repr(C)]
struct IMallocObj {
    vtbl: *const IMallocVtbl,
}

unsafe impl Sync for IMallocObj {}

unsafe extern "win64" fn malloc_qi(
    this: *mut IMallocObj,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid_bytes(riid) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUNKNOWN_BYTES || iid == IID_IMALLOC_BYTES {
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        crate::com::warn_qi_failed_once("IMalloc", &crate::com::Guid::from_bytes(iid));
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn malloc_add_ref(_this: *mut IMallocObj) -> u32 {
    1
}

unsafe extern "win64" fn malloc_release(_this: *mut IMallocObj) -> u32 {
    1
}

unsafe extern "win64" fn malloc_alloc(_this: *mut IMallocObj, cb: usize) -> *mut c_void {
    CoTaskMemAlloc(cb)
}

unsafe extern "win64" fn malloc_realloc(
    _this: *mut IMallocObj,
    pv: *mut c_void,
    cb: usize,
) -> *mut c_void {
    CoTaskMemRealloc(pv, cb)
}

unsafe extern "win64" fn malloc_free(_this: *mut IMallocObj, pv: *mut c_void) {
    CoTaskMemFree(pv);
}

unsafe extern "win64" fn malloc_get_size(_this: *mut IMallocObj, pv: *mut c_void) -> usize {
    if pv.is_null() {
        0
    } else {
        crate::heap::size(pv).unwrap_or(usize::MAX)
    }
}

unsafe extern "win64" fn malloc_did_alloc(_this: *mut IMallocObj, pv: *mut c_void) -> i32 {
    if pv.is_null() {
        -1
    } else if crate::heap::is_ours(pv) {
        1
    } else {
        0
    }
}

unsafe extern "win64" fn malloc_heap_minimize(_this: *mut IMallocObj) {}

static IMALLOC_VTBL: IMallocVtbl = IMallocVtbl {
    query_interface: malloc_qi,
    add_ref: malloc_add_ref,
    release: malloc_release,
    alloc: malloc_alloc,
    realloc: malloc_realloc,
    free: malloc_free,
    get_size: malloc_get_size,
    did_alloc: malloc_did_alloc,
    heap_minimize: malloc_heap_minimize,
};

static IMALLOC_INSTANCE: IMallocObj = IMallocObj { vtbl: &IMALLOC_VTBL };

pub unsafe extern "win64" fn CoGetMalloc(dw_mem_context: u32, pp_malloc: *mut *mut c_void) -> HRESULT {
    if pp_malloc.is_null() {
        return E_INVALIDARG;
    }
    if dw_mem_context != 1 {
        std::ptr::write_unaligned(pp_malloc, std::ptr::null_mut());
        return E_INVALIDARG;
    }
    crate::com::register_host_vtable(
        crate::com::IID_IMalloc,
        (&IMALLOC_VTBL as *const IMallocVtbl) as *const usize,
        9,
    );
    std::ptr::write_unaligned(pp_malloc, (&IMALLOC_INSTANCE as *const IMallocObj) as *mut c_void);
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;

    #[repr(C)]
    struct MockUnkVtbl {
        qi: unsafe extern "win64" fn(*mut MockUnk, *const u8, *mut *mut c_void) -> HRESULT,
        add_ref: unsafe extern "win64" fn(*mut MockUnk) -> u32,
        release: unsafe extern "win64" fn(*mut MockUnk) -> u32,
    }

    #[repr(C)]
    struct MockUnk {
        vtbl: *const MockUnkVtbl,
        refs: AtomicU32,
        released_count: AtomicU32,
    }

    unsafe extern "win64" fn mock_qi(this: *mut MockUnk, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
        if ppv.is_null() {
            return E_POINTER;
        }
        std::ptr::write_unaligned(ppv, std::ptr::null_mut());
        if read_guid_bytes(riid) == Some(IID_IUNKNOWN_BYTES) {
            (&*this).refs.fetch_add(1, Ordering::AcqRel);
            std::ptr::write_unaligned(ppv, this as *mut c_void);
            S_OK
        } else {
            E_NOINTERFACE
        }
    }

    unsafe extern "win64" fn mock_add_ref(this: *mut MockUnk) -> u32 {
        (&*this).refs.fetch_add(1, Ordering::AcqRel) + 1
    }

    unsafe extern "win64" fn mock_release(this: *mut MockUnk) -> u32 {
        let obj = &*this;
        obj.released_count.fetch_add(1, Ordering::AcqRel);
        obj.refs.fetch_sub(1, Ordering::AcqRel) - 1
    }

    static MOCK_VTBL: MockUnkVtbl = MockUnkVtbl {
        qi: mock_qi,
        add_ref: mock_add_ref,
        release: mock_release,
    };

    #[test]
    fn istream_hglobal_full_roundtrip() {
        unsafe {
            let mut pstm: *mut c_void = std::ptr::null_mut();
            assert_eq!(CreateStreamOnHGlobal(std::ptr::null_mut(), TRUE, &mut pstm), S_OK);
            assert!(!pstm.is_null());
            let stm = pstm as *mut HGlobalStreamObj;
            let vt = &*(*stm).vtbl;

            // QI for IStream & ISequentialStream & IUnknown
            let mut qi_out: *mut c_void = std::ptr::null_mut();
            assert_eq!((vt.query_interface)(stm, IID_ISTREAM_BYTES.as_ptr(), &mut qi_out), S_OK);
            assert_eq!(qi_out, pstm);
            assert_eq!((vt.release)(stm), 1);

            assert_eq!((vt.query_interface)(stm, IID_ISEQUENTIALSTREAM_BYTES.as_ptr(), &mut qi_out), S_OK);
            assert_eq!((vt.release)(stm), 1);

            // Write payload
            let msg = b"Hello, Win64 IStream!";
            let mut written = 0u32;
            assert_eq!((vt.write)(stm, msg.as_ptr() as *const c_void, msg.len() as u32, &mut written), S_OK);
            assert_eq!(written, msg.len() as u32);

            // Stat reports exact byte size
            let mut stat_buf = [0u8; 80];
            assert_eq!((vt.stat)(stm, stat_buf.as_mut_ptr(), STATFLAG_NONAME), S_OK);
            let st = std::ptr::read_unaligned(stat_buf.as_ptr() as *const STATSTG);
            assert_eq!(st.ty, STGTY_STREAM);
            assert_eq!(st.cb_size, msg.len() as u64);

            // GetHGlobalFromStream returns live HGLOBAL with exact contents
            let mut hglob: *mut c_void = std::ptr::null_mut();
            assert_eq!(GetHGlobalFromStream(pstm, &mut hglob), S_OK);
            assert!(!hglob.is_null());
            assert_eq!(crate::kernel32::memory::GlobalSize(hglob), msg.len());
            let locked = crate::kernel32::memory::GlobalLock(hglob) as *const u8;
            assert_eq!(std::slice::from_raw_parts(locked, msg.len()), msg);

            // Seek back to start and Read
            let mut new_pos = 99u64;
            assert_eq!((vt.seek)(stm, 0, STREAM_SEEK_SET, &mut new_pos), S_OK);
            assert_eq!(new_pos, 0);

            let mut buf = [0u8; 32];
            let mut read = 0u32;
            assert_eq!((vt.read)(stm, buf.as_mut_ptr() as *mut c_void, 32, &mut read), S_FALSE);
            assert_eq!(read as usize, msg.len());
            assert_eq!(&buf[..msg.len()], msg);

            // Clone shares backing with independent seek pointer
            assert_eq!((vt.seek)(stm, 7, STREAM_SEEK_SET, &mut new_pos), S_OK);
            let mut cloned_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!((vt.clone)(stm, &mut cloned_ptr), S_OK);
            let cloned = cloned_ptr as *mut HGlobalStreamObj;
            let cvt = &*(*cloned).vtbl;
            let mut tail = [0u8; 14];
            let mut cread = 0u32;
            assert_eq!((cvt.read)(cloned, tail.as_mut_ptr() as *mut c_void, 14, &mut cread), S_OK);
            assert_eq!(&tail, b"Win64 IStream!");

            // CopyTo into a second stream
            let mut dst_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(CreateStreamOnHGlobal(std::ptr::null_mut(), TRUE, &mut dst_ptr), S_OK);
            assert_eq!((vt.seek)(stm, 0, STREAM_SEEK_SET, std::ptr::null_mut()), S_OK);
            let (mut cr, mut cw) = (0u64, 0u64);
            assert_eq!((vt.copy_to)(stm, dst_ptr, 100, &mut cr, &mut cw), S_OK);
            assert_eq!(cr, msg.len() as u64);
            assert_eq!(cw, msg.len() as u64);

            // Commit/Revert/LockRegion/UnlockRegion/SetSize
            assert_eq!((vt.commit)(stm, 0), S_OK);
            assert_eq!((vt.revert)(stm), S_OK);
            assert_eq!((vt.lock_region)(stm, 0, 4, 2), STG_E_INVALIDFUNCTION);
            assert_eq!((vt.unlock_region)(stm, 0, 4, 2), STG_E_INVALIDFUNCTION);
            assert_eq!((vt.set_size)(stm, 5), S_OK);
            assert_eq!((vt.stat)(stm, stat_buf.as_mut_ptr(), STATFLAG_NONAME), S_OK);
            assert_eq!(std::ptr::read_unaligned(stat_buf.as_ptr() as *const STATSTG).cb_size, 5);

            // Release all streams
            assert_eq!((cvt.release)(cloned), 0);
            assert_eq!((vt.release)(stm), 0);
            let dst = dst_ptr as *mut HGlobalStreamObj;
            assert_eq!(((*(*dst).vtbl).release)(dst), 0);
            // Invalid stream rejected by GetHGlobalFromStream
            assert_eq!(GetHGlobalFromStream(pstm, &mut hglob), E_INVALIDARG);

            // delete_on_release = FALSE keeps HGLOBAL alive after Release, and seek-past-EOF zero-fills
            let h_keep = crate::kernel32::memory::GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, 4);
            let mut pkeep: *mut c_void = std::ptr::null_mut();
            assert_eq!(CreateStreamOnHGlobal(h_keep, FALSE, &mut pkeep), S_OK);
            let kstm = pkeep as *mut HGlobalStreamObj;
            let kvt = &*(*kstm).vtbl;
            assert_eq!((kvt.seek)(kstm, 10, STREAM_SEEK_SET, std::ptr::null_mut()), S_OK);
            assert_eq!((kvt.write)(kstm, b"XY".as_ptr() as *const c_void, 2, std::ptr::null_mut()), S_OK);
            let mut h_after: *mut c_void = std::ptr::null_mut();
            assert_eq!(GetHGlobalFromStream(pkeep, &mut h_after), S_OK);
            assert_eq!((kvt.release)(kstm), 0);
            let raw = std::slice::from_raw_parts(crate::kernel32::memory::GlobalLock(h_after) as *const u8, 12);
            assert_eq!(&raw[..10], &[0u8; 10]);
            assert_eq!(&raw[10..12], b"XY");
            assert!(crate::kernel32::memory::GlobalFree(h_after).is_null());
        }
    }

    #[test]
    fn propvariant_clear_handles_bstr_unknown_and_vectors() {
        unsafe {
            // VT_BSTR
            let w = crate::strings::to_wide_z("hello bstr");
            let bstr = super::super::oleaut32::SysAllocString(w.as_ptr());
            let mut pv = [0u8; 24];
            std::ptr::write_unaligned(pv.as_mut_ptr() as *mut u16, 8u16); // VT_BSTR
            std::ptr::write_unaligned(pv.as_mut_ptr().add(8) as *mut *mut u16, bstr);
            assert_eq!(PropVariantClear(pv.as_mut_ptr()), S_OK);
            assert_eq!(pv, [0u8; 24]);

            // VT_UNKNOWN releases COM object
            let mut mock = MockUnk {
                vtbl: &MOCK_VTBL,
                refs: AtomicU32::new(2),
                released_count: AtomicU32::new(0),
            };
            std::ptr::write_unaligned(pv.as_mut_ptr() as *mut u16, 13u16); // VT_UNKNOWN
            std::ptr::write_unaligned(pv.as_mut_ptr().add(8) as *mut *mut MockUnk, &mut mock);
            assert_eq!(PropVariantClear(pv.as_mut_ptr()), S_OK);
            assert_eq!(mock.refs.load(Ordering::Acquire), 1);
            assert_eq!(mock.released_count.load(Ordering::Acquire), 1);

            // VT_VECTOR | VT_LPWSTR frees inner elements and array buffer
            let arr = CoTaskMemAlloc(16) as *mut *mut u16;
            let s1 = crate::audio::com::co_wstr("alpha");
            let s2 = crate::audio::com::co_wstr("beta");
            std::ptr::write_unaligned(arr.add(0), s1);
            std::ptr::write_unaligned(arr.add(1), s2);
            std::ptr::write_unaligned(pv.as_mut_ptr() as *mut u16, 0x1000u16 | 31u16);
            std::ptr::write_unaligned(pv.as_mut_ptr().add(8) as *mut u32, 2u32);
            std::ptr::write_unaligned(pv.as_mut_ptr().add(16) as *mut *mut *mut u16, arr);
            assert_eq!(PropVariantClear(pv.as_mut_ptr()), S_OK);
            assert_eq!(pv, [0u8; 24]);
        }
    }

    #[test]
    fn ftm_lock_proxy_dragdrop_stgmedium_and_class_object() {
        unsafe {
            // CoSetProxyBlanket
            assert_eq!(
                CoSetProxyBlanket(std::ptr::null_mut(), 0, 0, std::ptr::null_mut(), 0, 0, std::ptr::null_mut(), 0),
                E_INVALIDARG
            );
            let mut mock = MockUnk {
                vtbl: &MOCK_VTBL,
                refs: AtomicU32::new(1),
                released_count: AtomicU32::new(0),
            };
            let mock_ptr = (&mut mock as *mut MockUnk) as *mut c_void;
            assert_eq!(
                CoSetProxyBlanket(mock_ptr, 0, 0, std::ptr::null_mut(), 0, 0, std::ptr::null_mut(), 0),
                S_OK
            );

            // CoLockObjectExternal
            assert_eq!(CoLockObjectExternal(mock_ptr, TRUE, FALSE), S_OK);
            assert_eq!(mock.refs.load(Ordering::Acquire), 2);
            assert_eq!(CoLockObjectExternal(mock_ptr, FALSE, TRUE), S_OK);
            assert_eq!(mock.refs.load(Ordering::Acquire), 1);

            // RegisterDragDrop / RevokeDragDrop
            let fake_hwnd = 0x1234_5678usize as HWND;
            assert_eq!(RegisterDragDrop(fake_hwnd, mock_ptr), S_OK);
            assert_eq!(mock.refs.load(Ordering::Acquire), 2);
            assert_eq!(RegisterDragDrop(fake_hwnd, mock_ptr), DRAGDROP_E_ALREADYREGISTERED);
            assert_eq!(RevokeDragDrop(fake_hwnd), S_OK);
            assert_eq!(mock.refs.load(Ordering::Acquire), 1);
            assert_eq!(RevokeDragDrop(fake_hwnd), DRAGDROP_E_NOTREGISTERED);

            // CoCreateFreeThreadedMarshaler marshal/unmarshal roundtrip through IStream
            let mut ftm: *mut c_void = std::ptr::null_mut();
            assert_eq!(CoCreateFreeThreadedMarshaler(std::ptr::null_mut(), &mut ftm), S_OK);
            let mut imarshal: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(ftm, IID_IMARSHAL_BYTES.as_ptr(), &mut imarshal), S_OK);
            let mvt = &*(*(imarshal as *const *const IMarshalVtbl));

            let mut stm: *mut c_void = std::ptr::null_mut();
            assert_eq!(CreateStreamOnHGlobal(std::ptr::null_mut(), TRUE, &mut stm), S_OK);
            assert_eq!(
                (mvt.marshal_interface)(
                    imarshal,
                    stm,
                    IID_IUNKNOWN_BYTES.as_ptr(),
                    mock_ptr,
                    3,
                    std::ptr::null_mut(),
                    MSHLFLAGS_NORMAL
                ),
                S_OK
            );
            assert_eq!(mock.refs.load(Ordering::Acquire), 2);
            // Rewind stream and unmarshal
            let stm_obj = stm as *mut HGlobalStreamObj;
            assert_eq!(((*(*stm_obj).vtbl).seek)(stm_obj, 0, STREAM_SEEK_SET, std::ptr::null_mut()), S_OK);
            let mut unmarshaled: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                (mvt.unmarshal_interface)(imarshal, stm, IID_IUNKNOWN_BYTES.as_ptr(), &mut unmarshaled),
                S_OK
            );
            assert_eq!(unmarshaled, mock_ptr);
            assert_eq!(mock.refs.load(Ordering::Acquire), 2); // QI added 1, normal unmarshal released 1
            com_call_release(unmarshaled);
            assert_eq!(mock.refs.load(Ordering::Acquire), 1);
            com_call_release(imarshal);
            com_call_release(ftm);

            // ReleaseStgMedium with TYMED_ISTREAM & TYMED_HGLOBAL & pUnkForRelease
            let mut med = [0u8; 24];
            std::ptr::write_unaligned(med.as_mut_ptr() as *mut u32, TYMED_ISTREAM);
            std::ptr::write_unaligned(med.as_mut_ptr().add(8) as *mut *mut c_void, stm);
            ReleaseStgMedium(med.as_mut_ptr() as *mut c_void);
            assert_eq!(std::ptr::read_unaligned(med.as_ptr() as *const u32), TYMED_NULL);

            let h = crate::kernel32::memory::GlobalAlloc(GMEM_MOVEABLE, 32);
            std::ptr::write_unaligned(med.as_mut_ptr() as *mut u32, TYMED_HGLOBAL);
            std::ptr::write_unaligned(med.as_mut_ptr().add(8) as *mut *mut c_void, h);
            ReleaseStgMedium(med.as_mut_ptr() as *mut c_void);
            assert_eq!(std::ptr::read_unaligned(med.as_ptr() as *const u32), TYMED_NULL);

            com_call_add_ref(mock_ptr);
            std::ptr::write_unaligned(med.as_mut_ptr() as *mut u32, TYMED_HGLOBAL);
            std::ptr::write_unaligned(med.as_mut_ptr().add(16) as *mut *mut c_void, mock_ptr);
            ReleaseStgMedium(med.as_mut_ptr() as *mut c_void);
            assert_eq!(mock.refs.load(Ordering::Acquire), 1);

            // Aggregated CoCreateFreeThreadedMarshaler delegates IMarshal refcounting to outer
            let mut agg_ftm: *mut c_void = std::ptr::null_mut();
            assert_eq!(CoCreateFreeThreadedMarshaler(mock_ptr, &mut agg_ftm), S_OK);
            assert_eq!(mock.refs.load(Ordering::Acquire), 1);
            let mut agg_m: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_call_qi(agg_ftm, IID_IMARSHAL_BYTES.as_ptr(), &mut agg_m), S_OK);
            assert_eq!(mock.refs.load(Ordering::Acquire), 2);
            assert_eq!(com_call_add_ref(agg_m), 3);
            assert_eq!(com_call_release(agg_m), 2);
            assert_eq!(com_call_release(agg_m), 1);
            assert_eq!(com_call_release(agg_ftm), 0);
            assert_eq!(mock.refs.load(Ordering::Acquire), 1);

            // ReleaseStgMedium with TYMED_FILE deletes the file and frees its path string
            let tmp_dir = std::env::temp_dir().join(format!("win32_ole32_stg_{}", std::process::id()));
            let _ = std::fs::create_dir_all(&tmp_dir);
            let tmp_file = tmp_dir.join("medium.tmp");
            std::fs::write(&tmp_file, b"temp").unwrap();
            crate::path::add_mount("O:\\StgTest", &tmp_dir);
            let w_path = crate::audio::com::co_wstr("O:\\StgTest\\medium.tmp");
            std::ptr::write_unaligned(med.as_mut_ptr() as *mut u32, TYMED_FILE);
            std::ptr::write_unaligned(med.as_mut_ptr().add(8) as *mut *mut u16, w_path);
            std::ptr::write_unaligned(med.as_mut_ptr().add(16) as *mut usize, 0);
            ReleaseStgMedium(med.as_mut_ptr() as *mut c_void);
            assert!(!tmp_file.exists());
            let _ = std::fs::remove_dir_all(&tmp_dir);

            // CoGetClassObject returns real IClassFactory for CLSID_MMDeviceEnumerator
            let mut cf_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                CoGetClassObject(
                    CLSID_MMDEVICEENUMERATOR_BYTES.as_ptr(),
                    1,
                    std::ptr::null_mut(),
                    IID_ICLASSFACTORY_BYTES.as_ptr(),
                    &mut cf_ptr
                ),
                S_OK
            );
            let cf = cf_ptr as *mut ClassFactoryObj;
            let cfvt = &*(*cf).vtbl;
            assert_eq!((cfvt.lock_server)(cf, TRUE), S_OK);
            assert_eq!((cfvt.lock_server)(cf, FALSE), S_OK);
            let mut mmdev: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                (cfvt.create_instance)(cf, std::ptr::null_mut(), IID_IUNKNOWN_BYTES.as_ptr(), &mut mmdev),
                S_OK
            );
            assert!(!mmdev.is_null());
            com_call_release(mmdev);
            assert_eq!((cfvt.release)(cf), 0);
        }
    }

    #[test]
    fn co_register_class_object_get_create_and_revoke_with_guest_factory() {
        const CLSID_APP_WIDGET: crate::com::Guid =
            crate::com::guid("55aa1122-3344-5566-7788-99aabbccdd01");
        const IID_APP_WIDGET: crate::com::Guid =
            crate::com::guid("55aa1122-3344-5566-7788-99aabbccdd02");
        const IID_UNSUPPORTED: crate::com::Guid =
            crate::com::guid("55aa1122-3344-5566-7788-99aabbccdd99");

        #[repr(C)]
        struct GuestWidget {
            vtbl: *const usize,
            refs: AtomicU32,
            magic: u32,
        }

        unsafe extern "win64" fn widget_qi(
            this: *mut GuestWidget,
            riid: *const crate::com::Guid,
            ppv: *mut *mut c_void,
        ) -> HRESULT {
            if ppv.is_null() {
                return E_POINTER;
            }
            crate::com::write_null_out(ppv);
            let Some(g) = crate::com::read_guid(riid) else {
                return E_INVALIDARG;
            };
            if g == crate::com::IID_IUnknown || g == IID_APP_WIDGET {
                (*this).refs.fetch_add(1, Ordering::AcqRel);
                std::ptr::write_unaligned(ppv, this as *mut c_void);
                S_OK
            } else {
                E_NOINTERFACE
            }
        }
        unsafe extern "win64" fn widget_add_ref(this: *mut GuestWidget) -> u32 {
            (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
        }
        unsafe extern "win64" fn widget_release(this: *mut GuestWidget) -> u32 {
            let rem = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
            if rem == 0 {
                drop(Box::from_raw(this));
            }
            rem
        }
        fn widget_vtbl() -> *const usize {
            static VT: std::sync::OnceLock<[usize; 3]> = std::sync::OnceLock::new();
            VT.get_or_init(|| {
                [
                    widget_qi as *const () as usize,
                    widget_add_ref as *const () as usize,
                    widget_release as *const () as usize,
                ]
            })
            .as_ptr()
        }

        #[repr(C)]
        struct GuestFactory {
            vtbl: *const usize,
            refs: AtomicU32,
            created_count: AtomicU32,
        }

        unsafe extern "win64" fn gf_qi(
            this: *mut GuestFactory,
            riid: *const crate::com::Guid,
            ppv: *mut *mut c_void,
        ) -> HRESULT {
            if ppv.is_null() {
                return E_POINTER;
            }
            crate::com::write_null_out(ppv);
            let Some(g) = crate::com::read_guid(riid) else {
                return E_INVALIDARG;
            };
            if g == crate::com::IID_IUnknown || g == crate::com::IID_IClassFactory {
                (*this).refs.fetch_add(1, Ordering::AcqRel);
                std::ptr::write_unaligned(ppv, this as *mut c_void);
                S_OK
            } else {
                E_NOINTERFACE
            }
        }
        unsafe extern "win64" fn gf_add_ref(this: *mut GuestFactory) -> u32 {
            (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
        }
        unsafe extern "win64" fn gf_release(this: *mut GuestFactory) -> u32 {
            (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1
        }
        unsafe extern "win64" fn gf_create_instance(
            this: *mut GuestFactory,
            outer: *mut c_void,
            riid: *const crate::com::Guid,
            ppv: *mut *mut c_void,
        ) -> HRESULT {
            if !outer.is_null() {
                return CLASS_E_NOAGGREGATION;
            }
            (*this).created_count.fetch_add(1, Ordering::AcqRel);
            let w = Box::into_raw(Box::new(GuestWidget {
                vtbl: widget_vtbl(),
                refs: AtomicU32::new(1),
                magic: 0x1337,
            }));
            let hr = widget_qi(w, riid, ppv);
            widget_release(w);
            hr
        }
        unsafe extern "win64" fn gf_lock_server(_this: *mut GuestFactory, _lock: BOOL) -> HRESULT {
            S_OK
        }

        let gf_vtbl: [usize; 5] = [
            gf_qi as *const () as usize,
            gf_add_ref as *const () as usize,
            gf_release as *const () as usize,
            gf_create_instance as *const () as usize,
            gf_lock_server as *const () as usize,
        ];
        let mut gf = GuestFactory {
            vtbl: gf_vtbl.as_ptr(),
            refs: AtomicU32::new(1),
            created_count: AtomicU32::new(0),
        };
        let gf_ptr = (&mut gf as *mut GuestFactory) as *mut c_void;
        let clsid_bytes = CLSID_APP_WIDGET.to_bytes();
        let iid_widget_bytes = IID_APP_WIDGET.to_bytes();
        let iid_bad_bytes = IID_UNSUPPORTED.to_bytes();

        unsafe {
            // Before registration -> REGDB_E_CLASSNOTREG.
            let mut obj: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                CoCreateInstance(clsid_bytes.as_ptr(), std::ptr::null_mut(), CLSCTX_INPROC_SERVER, iid_widget_bytes.as_ptr(), &mut obj),
                REGDB_E_CLASSNOTREG
            );

            // Register guest factory -> AddRef increments 1 -> 2.
            let mut cookie = 0u32;
            assert_eq!(
                CoRegisterClassObject(clsid_bytes.as_ptr(), gf_ptr, CLSCTX_INPROC_SERVER, 1, &mut cookie),
                S_OK
            );
            assert_ne!(cookie, 0);
            assert_eq!(gf.refs.load(Ordering::Acquire), 2);

            // CoGetClassObject returns guest IClassFactory and rejects unsupported IID with E_NOINTERFACE.
            let mut got_cf: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                CoGetClassObject(clsid_bytes.as_ptr(), CLSCTX_INPROC_SERVER, std::ptr::null_mut(), IID_ICLASSFACTORY_BYTES.as_ptr(), &mut got_cf),
                S_OK
            );
            assert_eq!(got_cf, gf_ptr);
            assert_eq!(gf.refs.load(Ordering::Acquire), 3);
            com_call_release(got_cf);
            assert_eq!(gf.refs.load(Ordering::Acquire), 2);

            assert_eq!(
                CoGetClassObject(clsid_bytes.as_ptr(), CLSCTX_INPROC_SERVER, std::ptr::null_mut(), iid_bad_bytes.as_ptr(), &mut got_cf),
                E_NOINTERFACE
            );

            // CoCreateInstance invokes guest IClassFactory::CreateInstance via call_guest8.
            assert_eq!(
                CoCreateInstance(clsid_bytes.as_ptr(), std::ptr::null_mut(), CLSCTX_INPROC_SERVER, iid_widget_bytes.as_ptr(), &mut obj),
                S_OK
            );
            assert!(!obj.is_null());
            assert_eq!((*(obj as *const GuestWidget)).magic, 0x1337);
            assert_eq!(gf.created_count.load(Ordering::Acquire), 1);
            assert_eq!(com_call_release(obj), 0);

            // Unsupported IID on CoCreateInstance -> E_NOINTERFACE.
            assert_eq!(
                CoCreateInstance(clsid_bytes.as_ptr(), std::ptr::null_mut(), CLSCTX_INPROC_SERVER, iid_bad_bytes.as_ptr(), &mut obj),
                E_NOINTERFACE
            );

            // Revoke releases factory ref (2 -> 1) and makes class unregistered again.
            assert_eq!(CoRevokeClassObject(cookie), S_OK);
            assert_eq!(gf.refs.load(Ordering::Acquire), 1);
            assert_eq!(CoRevokeClassObject(cookie), E_INVALIDARG);
            assert_eq!(
                CoCreateInstance(clsid_bytes.as_ptr(), std::ptr::null_mut(), CLSCTX_INPROC_SERVER, iid_widget_bytes.as_ptr(), &mut obj),
                REGDB_E_CLASSNOTREG
            );
        }
    }

    #[test]
    fn winrt_activation_property_value_uri_custom_factory_and_unregistered_rejection() {
        unsafe {
            // 1. Unavailable WinRT class strictly returns REGDB_E_CLASSNOTREG.
            let missing_id = rust_to_hstring("Windows.System.NonExistentRuntimeClass").unwrap();
            let act_iid = crate::com::IID_IActivationFactory.to_bytes();
            let mut out: *mut c_void = std::ptr::null_mut();
            assert_eq!(RoGetActivationFactory(missing_id, act_iid.as_ptr(), &mut out), REGDB_E_CLASSNOTREG);
            assert!(out.is_null());
            assert_eq!(RoActivateInstance(missing_id, &mut out), REGDB_E_CLASSNOTREG);
            WindowsDeleteString(missing_id);

            // 2. Real Windows.Foundation.PropertyValue activation + IInspectable + IPropertyValue.
            let pv_class = rust_to_hstring("Windows.Foundation.PropertyValue").unwrap();
            let pvs_iid = crate::com::IID_IPropertyValueStatics.to_bytes();
            let mut pvs: *mut c_void = std::ptr::null_mut();
            assert_eq!(RoGetActivationFactory(pv_class, pvs_iid.as_ptr(), &mut pvs), S_OK);
            assert!(!pvs.is_null());

            // Unsupported IID on real WinRT class -> E_NOINTERFACE.
            let bad_iid = crate::com::guid("deadbeef-1111-2222-3333-444444444444").to_bytes();
            let mut bad_out: *mut c_void = std::ptr::null_mut();
            assert_eq!(RoGetActivationFactory(pv_class, bad_iid.as_ptr(), &mut bad_out), E_NOINTERFACE);

            // Create Int32 PropertyValue via typed COM slot 10 (CreateInt32).
            let mut pv_i32: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    10,
                    &[(-42i32) as i64 as u64, &mut pv_i32 as *mut *mut c_void as u64]
                ),
                S_OK
            );
            assert!(!pv_i32.is_null());
            let mut ptype = 0i32;
            let mut is_num = 0u8;
            let mut got_i32 = 0i32;
            assert_eq!(
                crate::com::com_invoke_hresult(pv_i32, &crate::com::IID_IPropertyValue, 6, &[&mut ptype as *mut i32 as u64]),
                S_OK
            );
            assert_eq!(ptype, WinRtPropertyType::Int32 as i32);
            assert_eq!(
                crate::com::com_invoke_hresult(pv_i32, &crate::com::IID_IPropertyValue, 7, &[&mut is_num as *mut u8 as u64]),
                S_OK
            );
            assert_eq!(is_num, 1);
            assert_eq!(
                crate::com::com_invoke_hresult(pv_i32, &crate::com::IID_IPropertyValue, 11, &[&mut got_i32 as *mut i32 as u64]),
                S_OK
            );
            assert_eq!(got_i32, -42);

            // Verify IInspectable::GetRuntimeClassName & GetTrustLevel & GetIids.
            let mut rcn: usize = 0;
            assert_eq!(
                crate::com::com_invoke_hresult(pv_i32, &crate::com::IID_IInspectable, 4, &[&mut rcn as *mut usize as u64]),
                S_OK
            );
            assert_eq!(hstring_to_rust(rcn), "Windows.Foundation.PropertyValue");
            WindowsDeleteString(rcn);

            let mut trust = -1i32;
            assert_eq!(
                crate::com::com_invoke_hresult(pv_i32, &crate::com::IID_IInspectable, 5, &[&mut trust as *mut i32 as u64]),
                S_OK
            );
            assert_eq!(trust, TRUST_LEVEL_BASE);

            let mut iid_cnt = 0u32;
            let mut iid_arr: *mut crate::com::Guid = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pv_i32,
                    &crate::com::IID_IInspectable,
                    3,
                    &[&mut iid_cnt as *mut u32 as u64, &mut iid_arr as *mut *mut crate::com::Guid as u64]
                ),
                S_OK
            );
            assert_eq!(iid_cnt, 1);
            assert_eq!(std::ptr::read_unaligned(iid_arr), crate::com::IID_IPropertyValue);
            CoTaskMemFree(iid_arr as *mut c_void);

            // Create String PropertyValue via slot 18.
            let hello = rust_to_hstring("WinRT works").unwrap();
            let mut pv_str: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    18,
                    &[hello as u64, &mut pv_str as *mut *mut c_void as u64]
                ),
                S_OK
            );
            let mut got_hstr: usize = 0;
            assert_eq!(
                crate::com::com_invoke_hresult(pv_str, &crate::com::IID_IPropertyValue, 19, &[&mut got_hstr as *mut usize as u64]),
                S_OK
            );
            assert_eq!(hstring_to_rust(got_hstr), "WinRT works");
            assert_eq!(WindowsGetStringLen(got_hstr), 11);
            assert_eq!(WindowsIsStringEmpty(got_hstr), FALSE);
            WindowsDeleteString(got_hstr);
            WindowsDeleteString(hello);
            assert_eq!(com_call_release(pv_str), 0);
            assert_eq!(com_call_release(pv_i32), 0);
            assert_eq!(com_call_release(pvs), 0);
            WindowsDeleteString(pv_class);

            // 3. Real Windows.Foundation.Uri activation & RFC 3986 parsing.
            let uri_class = rust_to_hstring("Windows.Foundation.Uri").unwrap();
            let urif_iid = crate::com::IID_IUriRuntimeClassFactory.to_bytes();
            let mut urif: *mut c_void = std::ptr::null_mut();
            assert_eq!(RoGetActivationFactory(uri_class, urif_iid.as_ptr(), &mut urif), S_OK);
            let raw_uri = rust_to_hstring("https://example.com:8443/api/v1/doc.json?ok=1#top").unwrap();
            let mut uri_obj: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    urif,
                    &crate::com::IID_IUriRuntimeClassFactory,
                    6, // CreateUri
                    &[raw_uri as u64, &mut uri_obj as *mut *mut c_void as u64]
                ),
                S_OK
            );
            let mut scheme_h: usize = 0;
            let mut host_h: usize = 0;
            let mut path_h: usize = 0;
            let mut ext_h: usize = 0;
            let mut port = 0i32;
            assert_eq!(crate::com::com_invoke_hresult(uri_obj, &crate::com::IID_IUriRuntimeClass, 17, &[&mut scheme_h as *mut usize as u64]), S_OK);
            assert_eq!(crate::com::com_invoke_hresult(uri_obj, &crate::com::IID_IUriRuntimeClass, 11, &[&mut host_h as *mut usize as u64]), S_OK);
            assert_eq!(crate::com::com_invoke_hresult(uri_obj, &crate::com::IID_IUriRuntimeClass, 13, &[&mut path_h as *mut usize as u64]), S_OK);
            assert_eq!(crate::com::com_invoke_hresult(uri_obj, &crate::com::IID_IUriRuntimeClass, 9, &[&mut ext_h as *mut usize as u64]), S_OK);
            assert_eq!(crate::com::com_invoke_hresult(uri_obj, &crate::com::IID_IUriRuntimeClass, 19, &[&mut port as *mut i32 as u64]), S_OK);
            assert_eq!(hstring_to_rust(scheme_h), "https");
            assert_eq!(hstring_to_rust(host_h), "example.com");
            assert_eq!(hstring_to_rust(path_h), "/api/v1/doc.json");
            assert_eq!(hstring_to_rust(ext_h), ".json");
            assert_eq!(port, 8443);
            for h in [scheme_h, host_h, path_h, ext_h, raw_uri, uri_class] {
                WindowsDeleteString(h);
            }
            assert_eq!(com_call_release(uri_obj), 0);
            assert_eq!(com_call_release(urif), 0);

            // 4. Custom registered WinRT activation factory exercised through RoActivateInstance.
            struct CustomActInner;
            unsafe extern "win64" fn custom_act_qi(
                this: *mut crate::com::ComObj<CustomActInner>,
                riid: *const crate::com::Guid,
                ppv: *mut *mut c_void,
            ) -> HRESULT {
                crate::com::qi_helper(
                    this,
                    riid,
                    ppv,
                    &[crate::com::IID_IInspectable, crate::com::IID_IActivationFactory],
                )
            }
            unsafe extern "win64" fn custom_act_activate(
                _this: *mut crate::com::ComObj<CustomActInner>,
                out: *mut *mut c_void,
            ) -> HRESULT {
                std::ptr::write_unaligned(out, new_prop_val(PropValData::UInt32(9001)));
                S_OK
            }
            let custom_vtbl: [usize; 7] = [
                custom_act_qi as *const () as usize,
                crate::com::com_add_ref::<CustomActInner> as *const () as usize,
                crate::com::com_release::<CustomActInner> as *const () as usize,
                pvf_get_iids as *const () as usize,
                pvf_get_runtime_class_name as *const () as usize,
                pvf_get_trust_level as *const () as usize,
                custom_act_activate as *const () as usize,
            ];
            let cf_raw = crate::com::ComObj::new(custom_vtbl.as_ptr(), CustomActInner) as *mut c_void;
            register_winrt_activation_factory(
                "Test.Custom.Activatable",
                crate::com::ComPtr::from_raw(cf_raw).unwrap(),
            );
            let cname = rust_to_hstring("Test.Custom.Activatable").unwrap();
            let mut inst: *mut c_void = std::ptr::null_mut();
            assert_eq!(RoActivateInstance(cname, &mut inst), S_OK);
            assert!(!inst.is_null());
            let mut got_u32 = 0u32;
            assert_eq!(
                crate::com::com_invoke_hresult(inst, &crate::com::IID_IPropertyValue, 12, &[&mut got_u32 as *mut u32 as u64]),
                S_OK
            );
            assert_eq!(got_u32, 9001);
            assert_eq!(com_call_release(inst), 0);
            WindowsDeleteString(cname);
            unregister_winrt_activation_factory("Test.Custom.Activatable");
        }
    }

    #[test]
    fn winrt_property_value_and_statics_complete_vtable_abi_order_and_final_slots() {
        unsafe {
            // Verify full WinMD ABI method order & count (6 IInspectable + 39 own = 45 slots each).
            let pv_desc = crate::com::lookup_interface(&crate::com::IID_IPropertyValue)
                .expect("IPropertyValue descriptor must be registered");
            let pvs_desc = crate::com::lookup_interface(&crate::com::IID_IPropertyValueStatics)
                .expect("IPropertyValueStatics descriptor must be registered");
            let expected_pv = [
                "get_Type",
                "get_IsNumericScalar",
                "GetUInt8",
                "GetInt16",
                "GetUInt16",
                "GetInt32",
                "GetUInt32",
                "GetInt64",
                "GetUInt64",
                "GetSingle",
                "GetDouble",
                "GetChar16",
                "GetBoolean",
                "GetString",
                "GetGuid",
                "GetDateTime",
                "GetTimeSpan",
                "GetPoint",
                "GetSize",
                "GetRect",
                "GetUInt8Array",
                "GetInt16Array",
                "GetUInt16Array",
                "GetInt32Array",
                "GetUInt32Array",
                "GetInt64Array",
                "GetUInt64Array",
                "GetSingleArray",
                "GetDoubleArray",
                "GetChar16Array",
                "GetBooleanArray",
                "GetStringArray",
                "GetInspectableArray",
                "GetGuidArray",
                "GetDateTimeArray",
                "GetTimeSpanArray",
                "GetPointArray",
                "GetSizeArray",
                "GetRectArray",
            ];
            let expected_pvs = [
                "CreateEmpty",
                "CreateUInt8",
                "CreateInt16",
                "CreateUInt16",
                "CreateInt32",
                "CreateUInt32",
                "CreateInt64",
                "CreateUInt64",
                "CreateSingle",
                "CreateDouble",
                "CreateChar16",
                "CreateBoolean",
                "CreateString",
                "CreateInspectable",
                "CreateGuid",
                "CreateDateTime",
                "CreateTimeSpan",
                "CreatePoint",
                "CreateSize",
                "CreateRect",
                "CreateUInt8Array",
                "CreateInt16Array",
                "CreateUInt16Array",
                "CreateInt32Array",
                "CreateUInt32Array",
                "CreateInt64Array",
                "CreateUInt64Array",
                "CreateSingleArray",
                "CreateDoubleArray",
                "CreateChar16Array",
                "CreateBooleanArray",
                "CreateStringArray",
                "CreateInspectableArray",
                "CreateGuidArray",
                "CreateDateTimeArray",
                "CreateTimeSpanArray",
                "CreatePointArray",
                "CreateSizeArray",
                "CreateRectArray",
            ];
            assert_eq!(pv_desc.slots.len(), 39);
            assert_eq!(pvs_desc.slots.len(), 39);
            for (i, &name) in expected_pv.iter().enumerate() {
                assert_eq!(pv_desc.slots[i].slot, 6 + i);
                assert_eq!(pv_desc.slots[i].name, name);
            }
            for (i, &name) in expected_pvs.iter().enumerate() {
                assert_eq!(pvs_desc.slots[i].slot, 6 + i);
                assert_eq!(pvs_desc.slots[i].name, name);
            }

            // Activate Windows.Foundation.PropertyValue as IActivationFactory, then QI to IPropertyValueStatics.
            let pv_class = rust_to_hstring("Windows.Foundation.PropertyValue").unwrap();
            let act_iid = crate::com::IID_IActivationFactory.to_bytes();
            let mut factory: *mut c_void = std::ptr::null_mut();
            assert_eq!(RoGetActivationFactory(pv_class, act_iid.as_ptr(), &mut factory), S_OK);
            assert!(!factory.is_null());

            let mut pvs: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    factory,
                    &crate::com::IID_IUnknown,
                    0, // QueryInterface
                    &[
                        &crate::com::IID_IPropertyValueStatics as *const crate::com::Guid as u64,
                        &mut pvs as *mut *mut c_void as u64,
                    ],
                ),
                S_OK
            );
            assert!(!pvs.is_null());

            // Verify all 45 vtable slots on IPropertyValueStatics are populated and non-null.
            let pvs_vtbl_ptr = *(pvs as *const *const usize);
            for slot in 0..45usize {
                assert_ne!(
                    *pvs_vtbl_ptr.add(slot),
                    0,
                    "IPropertyValueStatics vtable slot {slot} must be populated"
                );
            }

            // Create a boxed Int32 via slot 10 and QI it to IPropertyValue.
            let mut pv_unk: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    10,
                    &[1234u64, &mut pv_unk as *mut *mut c_void as u64],
                ),
                S_OK
            );
            let mut pv: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pv_unk,
                    &crate::com::IID_IUnknown,
                    0,
                    &[
                        &crate::com::IID_IPropertyValue as *const crate::com::Guid as u64,
                        &mut pv as *mut *mut c_void as u64,
                    ],
                ),
                S_OK
            );
            assert!(!pv.is_null());

            // Verify all 45 vtable slots on IPropertyValue are populated and non-null.
            let pv_vtbl_ptr = *(pv as *const *const usize);
            for slot in 0..45usize {
                assert_ne!(
                    *pv_vtbl_ptr.add(slot),
                    0,
                    "IPropertyValue vtable slot {slot} must be populated"
                );
            }

            // Existing scalar getter (slot 11: GetInt32) remains intact.
            let mut got_i32 = 0i32;
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pv,
                    &crate::com::IID_IPropertyValue,
                    11,
                    &[&mut got_i32 as *mut i32 as u64],
                ),
                S_OK
            );
            assert_eq!(got_i32, 1234);

            // Exercise struct getters on a mismatched Int32 PropertyValue -> TYPE_E_TYPEMISMATCH.
            let mut dt = 0x1122_3344_5566_7788i64;
            assert_eq!(
                crate::com::com_invoke_hresult(pv, &crate::com::IID_IPropertyValue, 21, &[&mut dt as *mut i64 as u64]),
                TYPE_E_TYPEMISMATCH
            );
            assert_eq!(dt, 0);
            assert_eq!(
                crate::com::com_invoke_hresult(pv, &crate::com::IID_IPropertyValue, 21, &[0]),
                E_POINTER
            );

            let mut rect = [1.0f32, 2.0, 3.0, 4.0];
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pv,
                    &crate::com::IID_IPropertyValue,
                    25, // GetRect
                    &[&mut rect as *mut [f32; 4] as u64],
                ),
                TYPE_E_TYPEMISMATCH
            );
            assert_eq!(rect, [0.0; 4]);

            // Exercise array getters on mismatched Int32 PropertyValue -> TYPE_E_TYPEMISMATCH.
            for slot in [26usize, 37, 38, 39, 42, 43, 44] {
                let mut len = 0xCAFE_BABEu32;
                let mut ptr = 0xDEAD_BEEFusize as *mut c_void;
                assert_eq!(
                    crate::com::com_invoke_hresult(
                        pv,
                        &crate::com::IID_IPropertyValue,
                        slot,
                        &[&mut len as *mut u32 as u64, &mut ptr as *mut *mut c_void as u64],
                    ),
                    TYPE_E_TYPEMISMATCH,
                    "IPropertyValue slot {slot} should return TYPE_E_TYPEMISMATCH for Int32"
                );
                assert_eq!(len, 0, "IPropertyValue slot {slot} must zero out_len");
                assert!(ptr.is_null(), "IPropertyValue slot {slot} must null out_ptr");

                // Null out_ptr must still zero out_len and return E_POINTER.
                len = 99;
                assert_eq!(
                    crate::com::com_invoke_hresult(
                        pv,
                        &crate::com::IID_IPropertyValue,
                        slot,
                        &[&mut len as *mut u32 as u64, 0],
                    ),
                    E_POINTER
                );
                assert_eq!(len, 0);
            }

            // Round-trip DateTime (slot 21), TimeSpan (22), Point (23), Size (24), Rect (25).
            let mut created = std::ptr::null_mut::<c_void>();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    21, // CreateDateTime
                    &[133_000_000_000_000_000i64 as u64, &mut created as *mut *mut c_void as u64],
                ),
                S_OK
            );
            let mut ptype = 0i32;
            let mut is_num = 1u8;
            let mut got_dt = 0i64;
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 6, &[&mut ptype as *mut i32 as u64]), S_OK);
            assert_eq!(ptype, WinRtPropertyType::DateTime as i32);
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 7, &[&mut is_num as *mut u8 as u64]), S_OK);
            assert_eq!(is_num, 0);
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 21, &[&mut got_dt as *mut i64 as u64]), S_OK);
            assert_eq!(got_dt, 133_000_000_000_000_000);
            assert_eq!(com_call_release(created), 0);

            created = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    22, // CreateTimeSpan
                    &[(-5_000_000i64) as u64, &mut created as *mut *mut c_void as u64],
                ),
                S_OK
            );
            let mut got_ts = 0i64;
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 6, &[&mut ptype as *mut i32 as u64]), S_OK);
            assert_eq!(ptype, WinRtPropertyType::TimeSpan as i32);
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 22, &[&mut got_ts as *mut i64 as u64]), S_OK);
            assert_eq!(got_ts, -5_000_000);
            assert_eq!(com_call_release(created), 0);

            created = std::ptr::null_mut();
            let pt_bits = (1.5f32.to_bits() as u64) | ((2.25f32.to_bits() as u64) << 32);
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    23, // CreatePoint
                    &[pt_bits, &mut created as *mut *mut c_void as u64],
                ),
                S_OK
            );
            let mut got_pt = [0.0f32; 2];
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 6, &[&mut ptype as *mut i32 as u64]), S_OK);
            assert_eq!(ptype, WinRtPropertyType::Point as i32);
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 23, &[&mut got_pt as *mut [f32; 2] as u64]), S_OK);
            assert_eq!(got_pt, [1.5, 2.25]);
            assert_eq!(com_call_release(created), 0);

            created = std::ptr::null_mut();
            let sz_bits = (640.0f32.to_bits() as u64) | ((480.0f32.to_bits() as u64) << 32);
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    24, // CreateSize
                    &[sz_bits, &mut created as *mut *mut c_void as u64],
                ),
                S_OK
            );
            let mut got_sz = [0.0f32; 2];
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 6, &[&mut ptype as *mut i32 as u64]), S_OK);
            assert_eq!(ptype, WinRtPropertyType::Size as i32);
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 24, &[&mut got_sz as *mut [f32; 2] as u64]), S_OK);
            assert_eq!(got_sz, [640.0, 480.0]);
            assert_eq!(com_call_release(created), 0);

            created = std::ptr::null_mut();
            let sample_rect = [10.0f32, 20.0, 100.0, 200.0];
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    25, // CreateRect
                    &[&sample_rect as *const [f32; 4] as u64, &mut created as *mut *mut c_void as u64],
                ),
                S_OK
            );
            let mut got_rect = [0.0f32; 4];
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 6, &[&mut ptype as *mut i32 as u64]), S_OK);
            assert_eq!(ptype, WinRtPropertyType::Rect as i32);
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 25, &[&mut got_rect as *mut [f32; 4] as u64]), S_OK);
            assert_eq!(got_rect, sample_rect);
            assert_eq!(com_call_release(created), 0);

            // CreateRect with null rect pointer returns E_POINTER and nulls out-param.
            created = 0xDEAD_BEEFusize as *mut c_void;
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    25,
                    &[0, &mut created as *mut *mut c_void as u64],
                ),
                E_POINTER
            );
            assert!(created.is_null());

            // Error validation on array creators: null data with len > 0 or null out-param -> E_POINTER.
            for slot in [26usize, 37, 38, 39, 42, 43, 44] {
                created = 0xDEAD_BEEFusize as *mut c_void;
                assert_eq!(
                    crate::com::com_invoke_hresult(
                        pvs,
                        &crate::com::IID_IPropertyValueStatics,
                        slot,
                        &[2, 0, &mut created as *mut *mut c_void as u64],
                    ),
                    E_POINTER
                );
                assert!(created.is_null());
                assert_eq!(
                    crate::com::com_invoke_hresult(
                        pvs,
                        &crate::com::IID_IPropertyValueStatics,
                        slot,
                        &[0, 0, 0],
                    ),
                    E_POINTER
                );
            }

            // Round-trip Int32Array (slot 29), BooleanArray (36), StringArray (37), InspectableArray (38), RectArray (44).
            let i32s = [7i32, -42, 1024];
            created = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    29, // CreateInt32Array
                    &[i32s.len() as u64, i32s.as_ptr() as u64, &mut created as *mut *mut c_void as u64],
                ),
                S_OK
            );
            assert_eq!(crate::com::com_invoke_hresult(created, &crate::com::IID_IPropertyValue, 6, &[&mut ptype as *mut i32 as u64]), S_OK);
            assert_eq!(ptype, WinRtPropertyType::Int32Array as i32);
            let mut out_len = 0u32;
            let mut out_ptr = std::ptr::null_mut::<c_void>();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    created,
                    &crate::com::IID_IPropertyValue,
                    29, // GetInt32Array
                    &[&mut out_len as *mut u32 as u64, &mut out_ptr as *mut *mut c_void as u64],
                ),
                S_OK
            );
            assert_eq!(out_len, 3);
            assert_eq!(std::slice::from_raw_parts(out_ptr as *const i32, 3), &i32s);
            CoTaskMemFree(out_ptr);
            assert_eq!(com_call_release(created), 0);

            let raw_bools = [0u8, 5u8, 1u8];
            created = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    36, // CreateBooleanArray
                    &[raw_bools.len() as u64, raw_bools.as_ptr() as u64, &mut created as *mut *mut c_void as u64],
                ),
                S_OK
            );
            assert_eq!(
                crate::com::com_invoke_hresult(
                    created,
                    &crate::com::IID_IPropertyValue,
                    36, // GetBooleanArray
                    &[&mut out_len as *mut u32 as u64, &mut out_ptr as *mut *mut c_void as u64],
                ),
                S_OK
            );
            assert_eq!(out_len, 3);
            assert_eq!(std::slice::from_raw_parts(out_ptr as *const u8, 3), &[0, 1, 1]);
            CoTaskMemFree(out_ptr);
            assert_eq!(com_call_release(created), 0);

            let h1 = rust_to_hstring("alpha").unwrap();
            let h2 = rust_to_hstring("").unwrap();
            let h3 = rust_to_hstring("omega").unwrap();
            let hstrs = [h1, h2, h3];
            created = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    37, // CreateStringArray
                    &[hstrs.len() as u64, hstrs.as_ptr() as u64, &mut created as *mut *mut c_void as u64],
                ),
                S_OK
            );
            WindowsDeleteString(h1);
            WindowsDeleteString(h2);
            WindowsDeleteString(h3);
            assert_eq!(
                crate::com::com_invoke_hresult(
                    created,
                    &crate::com::IID_IPropertyValue,
                    37, // GetStringArray
                    &[&mut out_len as *mut u32 as u64, &mut out_ptr as *mut *mut c_void as u64],
                ),
                S_OK
            );
            assert_eq!(out_len, 3);
            let got_hs = std::slice::from_raw_parts(out_ptr as *const usize, 3);
            assert_eq!(hstring_to_rust(got_hs[0]), "alpha");
            assert_eq!(hstring_to_rust(got_hs[1]), "");
            assert_eq!(hstring_to_rust(got_hs[2]), "omega");
            for &h in got_hs {
                WindowsDeleteString(h);
            }
            CoTaskMemFree(out_ptr);
            assert_eq!(com_call_release(created), 0);

            // InspectableArray retains COM elements and AddRefs copies on GetInspectableArray.
            let elem_arr = [pv, std::ptr::null_mut()];
            created = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    pvs,
                    &crate::com::IID_IPropertyValueStatics,
                    38, // CreateInspectableArray
                    &[elem_arr.len() as u64, elem_arr.as_ptr() as u64, &mut created as *mut *mut c_void as u64],
                ),
                S_OK
            );
            assert_eq!(
                crate::com::com_invoke_hresult(
                    created,
                    &crate::com::IID_IPropertyValue,
                    38, // GetInspectableArray
                    &[&mut out_len as *mut u32 as u64, &mut out_ptr as *mut *mut c_void as u64],
                ),
                S_OK
            );
            assert_eq!(out_len, 2);
            let got_insps = std::slice::from_raw_parts(out_ptr as *const *mut c_void, 2);
            assert_eq!(got_insps[0], pv);
            assert!(got_insps[1].is_null());
            let _ = com_call_release(got_insps[0]);
            CoTaskMemFree(out_ptr);
            assert_eq!(com_call_release(created), 0);

            // Raw vtable call on final slot (44: CreateRectArray / GetRectArray) including 0-len and non-empty.
            type GetRectArrayFn = unsafe extern "win64" fn(*mut c_void, *mut u32, *mut *mut c_void) -> HRESULT;
            type CreateRectArrayFn = unsafe extern "win64" fn(*mut c_void, u32, *const c_void, *mut *mut c_void) -> HRESULT;
            let raw_get_rect_array: GetRectArrayFn = std::mem::transmute(*pv_vtbl_ptr.add(44));
            let raw_create_rect_array: CreateRectArrayFn = std::mem::transmute(*pvs_vtbl_ptr.add(44));
            let mut raw_len = 77u32;
            let mut raw_ptr = 0x1234usize as *mut c_void;
            assert_eq!(raw_get_rect_array(pv, &mut raw_len, &mut raw_ptr), TYPE_E_TYPEMISMATCH);
            assert_eq!(raw_len, 0);
            assert!(raw_ptr.is_null());

            created = std::ptr::null_mut();
            assert_eq!(raw_create_rect_array(pvs, 0, std::ptr::null(), &mut created), S_OK);
            assert!(!created.is_null());
            raw_len = 99;
            raw_ptr = 0x1234usize as *mut c_void;
            assert_eq!(raw_get_rect_array(created, &mut raw_len, &mut raw_ptr), S_OK);
            assert_eq!(raw_len, 0);
            assert!(raw_ptr.is_null());
            assert_eq!(com_call_release(created), 0);

            let rects = [[1.0f32, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]];
            created = std::ptr::null_mut();
            assert_eq!(raw_create_rect_array(pvs, 2, rects.as_ptr() as *const c_void, &mut created), S_OK);
            assert_eq!(raw_get_rect_array(created, &mut raw_len, &mut raw_ptr), S_OK);
            assert_eq!(raw_len, 2);
            assert_eq!(std::slice::from_raw_parts(raw_ptr as *const [f32; 4], 2), &rects);
            CoTaskMemFree(raw_ptr);
            assert_eq!(com_call_release(created), 0);

            assert_eq!(com_call_release(pv), 1);
            assert_eq!(com_call_release(pv_unk), 0);
            assert_eq!(com_call_release(pvs), 1);
            assert_eq!(com_call_release(factory), 0);
            WindowsDeleteString(pv_class);
        }
    }

    #[test]
    fn winrt_uri_combine_preserves_base_path_for_query_and_fragment_references() {
        unsafe {
            let uri_class = rust_to_hstring("Windows.Foundation.Uri").unwrap();
            let urif_iid = crate::com::IID_IUriRuntimeClassFactory.to_bytes();
            let mut urif: *mut c_void = std::ptr::null_mut();
            assert_eq!(RoGetActivationFactory(uri_class, urif_iid.as_ptr(), &mut urif), S_OK);

            // (base, rel, expected_abs, expected_raw, expected_path, expected_query, expected_frag)
            let cases: &[(&str, &str, &str, &str, &str, &str, &str)] = &[
                // Reviewer doc.json examples: query-only and fragment-only keep doc.json.
                (
                    "https://example.com/a/doc.json",
                    "?k=v",
                    "https://example.com/a/doc.json?k=v",
                    "https://example.com/a/doc.json?k=v",
                    "/a/doc.json",
                    "?k=v",
                    "",
                ),
                (
                    "https://example.com/a/doc.json",
                    "#sec",
                    "https://example.com/a/doc.json#sec",
                    "https://example.com/a/doc.json#sec",
                    "/a/doc.json",
                    "",
                    "#sec",
                ),
                // Replacing existing fragment preserves base query, userinfo, and port.
                (
                    "https://user:pw@example.com:8443/a/doc.json?old=1#old",
                    "#new",
                    "https://user:pw@example.com:8443/a/doc.json?old=1#new",
                    "https://user:pw@example.com:8443/a/doc.json?old=1#new",
                    "/a/doc.json",
                    "?old=1",
                    "#new",
                ),
                // Query + fragment replaces both existing query and fragment.
                (
                    "https://example.com/a/doc.json?old=1#old",
                    "?new=2#frag",
                    "https://example.com/a/doc.json?new=2#frag",
                    "https://example.com/a/doc.json?new=2#frag",
                    "/a/doc.json",
                    "?new=2",
                    "#frag",
                ),
                // Explicitly empty '?' / '#' / '?#' vs empty reference "".
                (
                    "https://example.com/a/doc.json?old=1#old",
                    "?",
                    "https://example.com/a/doc.json?",
                    "https://example.com/a/doc.json?",
                    "/a/doc.json",
                    "?",
                    "",
                ),
                (
                    "https://example.com/a/doc.json?old=1#old",
                    "#",
                    "https://example.com/a/doc.json?old=1#",
                    "https://example.com/a/doc.json?old=1#",
                    "/a/doc.json",
                    "?old=1",
                    "#",
                ),
                (
                    "https://example.com/a/doc.json?old=1#old",
                    "?#",
                    "https://example.com/a/doc.json?#",
                    "https://example.com/a/doc.json?#",
                    "/a/doc.json",
                    "?",
                    "#",
                ),
                (
                    "https://example.com/a/doc.json?old=1#old",
                    "",
                    "https://example.com/a/doc.json?old=1",
                    "https://example.com/a/doc.json?old=1",
                    "/a/doc.json",
                    "?old=1",
                    "",
                ),
                // Embedded '://' inside query or fragment must not be treated as an absolute URI.
                (
                    "https://example.com/a/doc.json",
                    "?next=https://other.example/x",
                    "https://example.com/a/doc.json?next=https://other.example/x",
                    "https://example.com/a/doc.json?next=https://other.example/x",
                    "/a/doc.json",
                    "?next=https://other.example/x",
                    "",
                ),
                (
                    "https://example.com/a/doc.json?a=1",
                    "#https://other.example/x",
                    "https://example.com/a/doc.json?a=1#https://other.example/x",
                    "https://example.com/a/doc.json?a=1#https://other.example/x",
                    "/a/doc.json",
                    "?a=1",
                    "#https://other.example/x",
                ),
                // Scheme-relative (network-path) references inherit only the base scheme.
                (
                    "https://example.com:8443/a/doc.json?old=1#old",
                    "//cdn.example.net/assets/app.js",
                    "https://cdn.example.net/assets/app.js",
                    "https://cdn.example.net/assets/app.js",
                    "/assets/app.js",
                    "",
                    "",
                ),
                (
                    "https://example.com:8443/a/doc.json?old=1#old",
                    "//cdn.example.net:9443/p/http://origin/img.png?v=2#hero",
                    "https://cdn.example.net:9443/p/http://origin/img.png?v=2#hero",
                    "https://cdn.example.net:9443/p/http://origin/img.png?v=2#hero",
                    "/p/http://origin/img.png",
                    "?v=2",
                    "#hero",
                ),
                (
                    "http://example.com/a/doc.json",
                    "//cdn.example.net?v=1#top",
                    "http://cdn.example.net/?v=1#top",
                    "http://cdn.example.net?v=1#top",
                    "/",
                    "?v=1",
                    "#top",
                ),
                // Single-slash root-relative reference keeps base authority/port.
                (
                    "https://example.com:8443/a/doc.json?old=1#old",
                    "/root/other.json?k=1#s",
                    "https://example.com:8443/root/other.json?k=1#s",
                    "https://example.com:8443/root/other.json?k=1#s",
                    "/root/other.json",
                    "?k=1",
                    "#s",
                ),
                // Normal relative-file (including embedded '://' in query) and absolute controls.
                (
                    "https://example.com:8443/a/doc.json?old=1#old",
                    "other.json?next=https://other.example/x",
                    "https://example.com:8443/a/other.json?next=https://other.example/x",
                    "https://example.com:8443/a/other.json?next=https://other.example/x",
                    "/a/other.json",
                    "?next=https://other.example/x",
                    "",
                ),
                // Non-authority schemes (mailto, urn, custom with uppercase/punctuation).
                (
                    "https://example.com/a/doc.json",
                    "mailto:user@example.com?subject=hi",
                    "mailto:user@example.com?subject=hi",
                    "mailto:user@example.com?subject=hi",
                    "user@example.com",
                    "?subject=hi",
                    "",
                ),
                (
                    "https://example.com/a/doc.json",
                    "urn:isbn:0451450523",
                    "urn:isbn:0451450523",
                    "urn:isbn:0451450523",
                    "isbn:0451450523",
                    "",
                    "",
                ),
                (
                    "https://example.com/a/doc.json",
                    "Foo+Bar.v1-test:opaque/part?q=1#f",
                    "foo+bar.v1-test:opaque/part?q=1#f",
                    "Foo+Bar.v1-test:opaque/part?q=1#f",
                    "opaque/part",
                    "?q=1",
                    "#f",
                ),
                // Relative colons in non-initial segments, dot-segment resolution, and userinfo preservation.
                (
                    "https://example.com/a/doc.json",
                    "dir/item:123?k=a:b#c:d",
                    "https://example.com/a/dir/item:123?k=a:b#c:d",
                    "https://example.com/a/dir/item:123?k=a:b#c:d",
                    "/a/dir/item:123",
                    "?k=a:b",
                    "#c:d",
                ),
                (
                    "https://user:pw@example.com:8443/a/doc.json?old=1#old",
                    "./item:123",
                    "https://user:pw@example.com:8443/a/item:123",
                    "https://user:pw@example.com:8443/a/item:123",
                    "/a/item:123",
                    "",
                    "",
                ),
                (
                    "https://user:pw@example.com:8443/a/doc.json?old=1#old",
                    "../b/c.json",
                    "https://user:pw@example.com:8443/b/c.json",
                    "https://user:pw@example.com:8443/b/c.json",
                    "/b/c.json",
                    "",
                    "",
                ),
                (
                    "https://example.com/a/doc.json",
                    "https://other.example/x?q=1#f",
                    "https://other.example/x?q=1#f",
                    "https://other.example/x?q=1#f",
                    "/x",
                    "?q=1",
                    "#f",
                ),
            ];

            let check_uri = |obj: *mut c_void, label: &str, exp_abs: &str, exp_raw: &str, exp_path: &str, exp_q: &str, exp_f: &str| {
                let mut abs_h = 0usize;
                let mut raw_h = 0usize;
                let mut path_h = 0usize;
                let mut query_h = 0usize;
                let mut frag_h = 0usize;
                assert_eq!(crate::com::com_invoke_hresult(obj, &crate::com::IID_IUriRuntimeClass, 6, &[&mut abs_h as *mut usize as u64]), S_OK, "{label} AbsoluteUri");
                assert_eq!(crate::com::com_invoke_hresult(obj, &crate::com::IID_IUriRuntimeClass, 16, &[&mut raw_h as *mut usize as u64]), S_OK, "{label} RawUri");
                assert_eq!(crate::com::com_invoke_hresult(obj, &crate::com::IID_IUriRuntimeClass, 13, &[&mut path_h as *mut usize as u64]), S_OK, "{label} Path");
                assert_eq!(crate::com::com_invoke_hresult(obj, &crate::com::IID_IUriRuntimeClass, 14, &[&mut query_h as *mut usize as u64]), S_OK, "{label} Query");
                assert_eq!(crate::com::com_invoke_hresult(obj, &crate::com::IID_IUriRuntimeClass, 10, &[&mut frag_h as *mut usize as u64]), S_OK, "{label} Fragment");
                assert_eq!(hstring_to_rust(abs_h), exp_abs, "{label} AbsoluteUri");
                assert_eq!(hstring_to_rust(raw_h), exp_raw, "{label} RawUri");
                assert_eq!(hstring_to_rust(path_h), exp_path, "{label} Path");
                assert_eq!(hstring_to_rust(query_h), exp_q, "{label} Query");
                assert_eq!(hstring_to_rust(frag_h), exp_f, "{label} Fragment");
                for h in [abs_h, raw_h, path_h, query_h, frag_h] {
                    WindowsDeleteString(h);
                }
            };

            for &(base, rel, exp_abs, exp_raw, exp_path, exp_q, exp_f) in cases {
                let base_h = rust_to_hstring(base).unwrap();
                let rel_h = rust_to_hstring(rel).unwrap();

                // Entry point 1: IUriRuntimeClass::CombineUri (slot 22).
                let mut base_obj: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    crate::com::com_invoke_hresult(
                        urif,
                        &crate::com::IID_IUriRuntimeClassFactory,
                        6, // CreateUri
                        &[base_h as u64, &mut base_obj as *mut *mut c_void as u64],
                    ),
                    S_OK
                );
                let mut combined_obj: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    crate::com::com_invoke_hresult(
                        base_obj,
                        &crate::com::IID_IUriRuntimeClass,
                        22, // CombineUri
                        &[rel_h as u64, &mut combined_obj as *mut *mut c_void as u64],
                    ),
                    S_OK,
                    "CombineUri({base:?}, {rel:?})"
                );
                check_uri(combined_obj, &format!("CombineUri({base:?}, {rel:?})"), exp_abs, exp_raw, exp_path, exp_q, exp_f);
                assert_eq!(com_call_release(combined_obj), 0);
                assert_eq!(com_call_release(base_obj), 0);

                // Entry point 2: IUriRuntimeClassFactory::CreateWithRelativeUri (slot 7).
                let mut factory_combined: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    crate::com::com_invoke_hresult(
                        urif,
                        &crate::com::IID_IUriRuntimeClassFactory,
                        7, // CreateWithRelativeUri
                        &[base_h as u64, rel_h as u64, &mut factory_combined as *mut *mut c_void as u64],
                    ),
                    S_OK,
                    "CreateWithRelativeUri({base:?}, {rel:?})"
                );
                check_uri(factory_combined, &format!("CreateWithRelativeUri({base:?}, {rel:?})"), exp_abs, exp_raw, exp_path, exp_q, exp_f);
                assert_eq!(com_call_release(factory_combined), 0);

                WindowsDeleteString(rel_h);
                WindowsDeleteString(base_h);
            }

            // Malformed scheme candidates (':' in first path segment failing RFC 3986 §3.1)
            // must fail honestly with E_INVALIDARG on both WinRT entry points.
            let base_h = rust_to_hstring("https://example.com/a/doc.json").unwrap();
            let mut base_obj: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_invoke_hresult(
                    urif,
                    &crate::com::IID_IUriRuntimeClassFactory,
                    6, // CreateUri
                    &[base_h as u64, &mut base_obj as *mut *mut c_void as u64],
                ),
                S_OK
            );
            for bad_rel in ["1bad:foo", ":missing-scheme", "bad_scheme:foo"] {
                let rel_h = rust_to_hstring(bad_rel).unwrap();

                let mut out_comb: *mut c_void = 0xDEAD_BEEFusize as *mut c_void;
                assert_eq!(
                    crate::com::com_invoke_hresult(
                        base_obj,
                        &crate::com::IID_IUriRuntimeClass,
                        22, // CombineUri
                        &[rel_h as u64, &mut out_comb as *mut *mut c_void as u64],
                    ),
                    E_INVALIDARG,
                    "CombineUri should reject malformed scheme {bad_rel:?}"
                );
                assert!(out_comb.is_null());

                let mut out_fac: *mut c_void = 0xDEAD_BEEFusize as *mut c_void;
                assert_eq!(
                    crate::com::com_invoke_hresult(
                        urif,
                        &crate::com::IID_IUriRuntimeClassFactory,
                        7, // CreateWithRelativeUri
                        &[base_h as u64, rel_h as u64, &mut out_fac as *mut *mut c_void as u64],
                    ),
                    E_INVALIDARG,
                    "CreateWithRelativeUri should reject malformed scheme {bad_rel:?}"
                );
                assert!(out_fac.is_null());

                WindowsDeleteString(rel_h);
            }
            assert_eq!(com_call_release(base_obj), 0);
            WindowsDeleteString(base_h);

            assert_eq!(com_call_release(urif), 0);
            WindowsDeleteString(uri_class);
        }
    }

    #[test]
    fn string_from_clsid_formats_and_validates() {
        unsafe {
            assert_eq!(
                StringFromCLSID(IID_IUNKNOWN_BYTES.as_ptr(), std::ptr::null_mut()),
                E_POINTER
            );
            let mut out: LPWSTR = 0x1usize as LPWSTR;
            assert_eq!(StringFromCLSID(std::ptr::null(), &mut out), E_INVALIDARG);
            assert!(out.is_null());

            assert_eq!(StringFromCLSID(IID_IUNKNOWN_BYTES.as_ptr(), &mut out), S_OK);
            assert!(!out.is_null());
            let s = crate::strings::from_wide(out);
            let mut g2 = [0u16; 64];
            assert_eq!(StringFromGUID2(IID_IUNKNOWN_BYTES.as_ptr(), g2.as_mut_ptr(), 64), 39);
            assert_eq!(s, crate::strings::from_wide(g2.as_ptr()));
            assert_eq!(s, "{00000000-0000-0000-C000-000000000046}");
            CoTaskMemFree(out as *mut c_void);
        }
    }
}
