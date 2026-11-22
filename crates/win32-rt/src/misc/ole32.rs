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
const CO_E_CLASSSTRING: HRESULT = 0x8004_01F3u32 as i32;
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

pub unsafe extern "win64" fn CoCreateInstance(
    clsid: *const u8,
    outer: *mut c_void,
    _ctx: u32,
    iid: *const u8,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    // Dynamic class factories registered by game targets / plugins take precedence.
    if let Ok(factories) = EXTRA_COM_FACTORIES.read() {
        for factory in factories.iter().rev() {
            if let Some(hr) = factory(clsid, iid, outer, out) {
                return hr;
            }
        }
    }
    // Class registry: in-process servers implemented in Rust.
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
    log::debug!(
        "CoCreateInstance: unregistered COM class {} (iid {}) -> REGDB_E_CLASSNOTREG",
        if clsid.is_null() { "NULL".into() } else { guid_string(clsid) },
        if iid.is_null() { "NULL".into() } else { guid_string(iid) }
    );
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
    register_vtable_ptrs(&[
        ftm_unk_qi as *const () as usize,
        ftm_unk_add_ref as *const () as usize,
        ftm_unk_release as *const () as usize,
        ftm_marshal_qi as *const () as usize,
        ftm_marshal_add_ref as *const () as usize,
        ftm_marshal_release as *const () as usize,
        ftm_get_unmarshal_class as *const () as usize,
        ftm_get_marshal_size_max as *const () as usize,
        ftm_marshal_interface as *const () as usize,
        ftm_unmarshal_interface as *const () as usize,
        ftm_release_marshal_data as *const () as usize,
        ftm_disconnect_object as *const () as usize,
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
    register_vtable_ptrs(&[
        cf_qi as *const () as usize,
        cf_add_ref as *const () as usize,
        cf_release as *const () as usize,
        cf_create_instance as *const () as usize,
        cf_lock_server as *const () as usize,
    ]);
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
    if !is_registered_com_class(&clsid) {
        log::debug!(
            "CoGetClassObject: unregistered COM class {} -> REGDB_E_CLASSNOTREG",
            guid_string(rclsid)
        );
        return REGDB_E_CLASSNOTREG;
    }
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
    S_OK
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
    register_vtable_ptrs(&[
        stream_qi as *const () as usize,
        stream_add_ref as *const () as usize,
        stream_release as *const () as usize,
        stream_read as *const () as usize,
        stream_write as *const () as usize,
        stream_seek as *const () as usize,
        stream_set_size as *const () as usize,
        stream_copy_to as *const () as usize,
        stream_commit as *const () as usize,
        stream_revert as *const () as usize,
        stream_lock_region as *const () as usize,
        stream_unlock_region as *const () as usize,
        stream_stat as *const () as usize,
        stream_clone as *const () as usize,
    ]);
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
// WinRT basics (`Ro*`, `Windows*String*`)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn RoInitialize(_init_type: u32) -> HRESULT {
    com_init()
}

pub unsafe extern "win64" fn RoUninitialize() {
    com_uninit()
}

pub unsafe extern "win64" fn RoGetActivationFactory(
    _activatable_class_id: usize,
    _iid: *const u8,
    factory: *mut *mut c_void,
) -> HRESULT {
    if !factory.is_null() {
        std::ptr::write_unaligned(factory, std::ptr::null_mut());
    }
    log::debug!("RoGetActivationFactory: class not registered -> REGDB_E_CLASSNOTREG");
    REGDB_E_CLASSNOTREG
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
    register_vtable_ptrs(&[
        malloc_qi as *const () as usize,
        malloc_add_ref as *const () as usize,
        malloc_release as *const () as usize,
        malloc_alloc as *const () as usize,
        malloc_realloc as *const () as usize,
        malloc_free as *const () as usize,
        malloc_get_size as *const () as usize,
        malloc_did_alloc as *const () as usize,
        malloc_heap_minimize as *const () as usize,
    ]);
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
}
