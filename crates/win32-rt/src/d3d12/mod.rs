//! d3d12.dll: COM shims over `d3d12-metal` (D3D12 on Metal 4).
//!
//! * Every object has one *superset* vtable generated from the headers
//!   (`vtables.rs`): the game calls newer slots through older interface
//!   pointers, so all slots exist; unimplemented ones trap with their name.
//! * Methods returning structs use the MSVC hidden return pointer
//!   (`this, *mut Ret, args...` → returns the pointer).
//! * Every failing HRESULT is logged (the game ignores most of them).
//!
//! Never loads D3D12Core.dll; the Agility SDK exports the game might probe
//! (`D3D12SDKVersion`) are the game's own.

pub mod cmdlist;
pub mod com;
pub mod cs;
pub mod descheap;
pub mod device;
pub mod fence;
pub mod heap;
pub mod mtl;
pub mod pso;
pub mod queue;
pub mod resource;
pub mod swapchain;
pub mod tr;
pub mod vtables;

use com::*;
use std::ffi::c_void;
use std::sync::OnceLock;
use vtables::*;

use crate::types::*;

pub static EXPORTS: &[crate::Export] = exports![
    D3D12CreateDevice,
    D3D12GetDebugInterface,
    D3D12SerializeRootSignature,
    D3D12SerializeVersionedRootSignature,
    D3D12CreateRootSignatureDeserializer,
    D3D12CreateVersionedRootSignatureDeserializer,
    D3D12EnableExperimentalFeatures,
    D3D12GetInterface,
];

/// d3d12.dll ordinals (NRI imports #101 / #102).
pub static ORDINALS: &[(u16, &str)] = &[
    (101, "D3D12CreateDevice"),
    (102, "D3D12GetDebugInterface"),
    (107, "D3D12CreateRootSignatureDeserializer"),
    (108, "D3D12CreateVersionedRootSignatureDeserializer"),
    (110, "D3D12EnableExperimentalFeatures"),
    (115, "D3D12SerializeRootSignature"),
    (116, "D3D12SerializeVersionedRootSignature"),
];

pub fn register_names() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        for (g, n) in IID_NAMES {
            name_iid(*g, n);
        }
        crate::dxgi::register_names();
    });
}

/// Log a failing HRESULT (the game silently ignores most failures).
pub fn fail(what: std::fmt::Arguments<'_>, hr: HRESULT) -> HRESULT {
    log::warn!(target: "d3d", "{what} -> {hr:#010x}");
    hr
}

macro_rules! failed {
    ($hr:expr, $($arg:tt)*) => {
        $crate::d3d12::fail(format_args!($($arg)*), $hr)
    };
}
pub(crate) use failed;

// ---------------------------------------------------------------------------
// Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn D3D12CreateDevice(adapter: *mut c_void, min_level: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    register_names();
    device::create_device(adapter, min_level, riid, out)
}

/// No debug layer (`ID3D12Debug*`, DRED settings): E_NOINTERFACE, as on a
/// system without the Graphics Tools optional feature.
pub unsafe extern "win64" fn D3D12GetDebugInterface(riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    register_names();
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    log::info!(target: "d3d", "D3D12GetDebugInterface({:?}) -> E_NOINTERFACE (no debug layer)", Guid::read(riid).unwrap_or_default());
    E_NOINTERFACE
}

pub unsafe extern "win64" fn D3D12GetInterface(clsid: *const Guid, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    register_names();
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    failed!(E_NOINTERFACE, "D3D12GetInterface({:?}, {:?}) (no SDK configuration / device factory)", Guid::read(clsid).unwrap_or_default(), Guid::read(riid).unwrap_or_default())
}

pub unsafe extern "win64" fn D3D12EnableExperimentalFeatures(n: u32, iids: *const Guid, _cfg: *mut c_void, _sizes: *mut u32) -> HRESULT {
    let list: Vec<Guid> = if iids.is_null() { vec![] } else { std::slice::from_raw_parts(iids, n as usize).to_vec() };
    failed!(E_NOINTERFACE, "D3D12EnableExperimentalFeatures({list:?}) (no experimental features)")
}

/// `ID3DBlob` holding a serialized root signature / error string.
pub mod blob {
    use super::com::*;
    use std::ffi::c_void;
    use std::sync::OnceLock;

    pub const IID_ID3D10Blob: Guid = guid("8ba5fb08-5195-40e2-ac58-0d989c3a0102");

    pub struct Blob(pub Vec<u8>);
    static CLASS: OnceLock<Class> = OnceLock::new();
    impl ComClass for Blob {
        fn class() -> &'static Class {
            class_once(&CLASS, || {
                let mut s = iunknown_slots::<Blob>();
                s.push(slot!("GetBufferPointer" => get_buffer_pointer));
                s.push(slot!("GetBufferSize" => get_buffer_size));
                Class { name: "ID3DBlob", vtbl: build_vtable("ID3DBlob", &s), iids: &[IID_ID3D10Blob] }
            })
        }
    }
    unsafe extern "win64" fn get_buffer_pointer(this: *mut Obj<Blob>) -> *mut c_void {
        (*this).inner.0.as_ptr() as *mut c_void
    }
    unsafe extern "win64" fn get_buffer_size(this: *mut Obj<Blob>) -> usize {
        (*this).inner.0.len()
    }
    pub fn new(bytes: Vec<u8>) -> *mut c_void {
        Obj::new(Blob(bytes)) as *mut c_void
    }
    /// Error blob: NUL-terminated ASCII message.
    pub fn error(msg: &str) -> *mut c_void {
        let mut b = msg.as_bytes().to_vec();
        b.push(0);
        new(b)
    }
}

unsafe fn serialize(desc: Result<dxil_metal::root_signature::RootSignatureDesc, String>, out: *mut *mut c_void, err: *mut *mut c_void, what: &str) -> HRESULT {
    if !err.is_null() {
        *err = std::ptr::null_mut();
    }
    if out.is_null() {
        return failed!(E_INVALIDARG, "{what}: null ppBlob");
    }
    *out = std::ptr::null_mut();
    match desc {
        Ok(d) => {
            log::debug!(target: "d3d", "{what}: {} parameters, {} static samplers, flags {:#x}", d.parameters.len(), d.static_samplers.len(), d.flags);
            *out = blob::new(d.to_blob());
            S_OK
        }
        Err(e) => {
            if !err.is_null() {
                *err = blob::error(&e);
            }
            failed!(E_INVALIDARG, "{what}: {e}")
        }
    }
}

pub unsafe extern "win64" fn D3D12SerializeRootSignature(desc: *const c_void, version: u32, out: *mut *mut c_void, err: *mut *mut c_void) -> HRESULT {
    use dxil_metal::root_signature::RootSignatureVersion;
    let d = match RootSignatureVersion::from_raw(version) {
        Some(RootSignatureVersion::V1_0) => d3d12_metal::rootsig::desc_from_mem(RootSignatureVersion::V1_0, desc as *const u8),
        _ => Err(format!("D3D12SerializeRootSignature takes only version 1.0 (got {version})")),
    };
    serialize(d, out, err, "D3D12SerializeRootSignature")
}

pub unsafe extern "win64" fn D3D12SerializeVersionedRootSignature(desc: *const c_void, out: *mut *mut c_void, err: *mut *mut c_void) -> HRESULT {
    let d = d3d12_metal::rootsig::desc_from_versioned(desc as *const u8);
    serialize(d, out, err, "D3D12SerializeVersionedRootSignature")
}

pub unsafe extern "win64" fn D3D12CreateRootSignatureDeserializer(_data: *const c_void, size: usize, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    stub!("D3D12CreateRootSignatureDeserializer({size} bytes, {:?}) -> E_NOTIMPL", Guid::read(riid).unwrap_or_default());
    E_NOTIMPL
}

pub unsafe extern "win64" fn D3D12CreateVersionedRootSignatureDeserializer(_data: *const c_void, size: usize, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    stub!("D3D12CreateVersionedRootSignatureDeserializer({size} bytes, {:?}) -> E_NOTIMPL", Guid::read(riid).unwrap_or_default());
    E_NOTIMPL
}

/// Pure-integer leaf D3D12 COM vtable methods (<= 8 integer args, no XMM args, never
/// re-enter guest code) eligible for `DF_PLAIN` direct calls from translated code.
pub fn plain_com_fns() -> &'static [usize] {
    static FNS: OnceLock<Box<[usize]>> = OnceLock::new();
    FNS.get_or_init(|| {
        let mut v = Vec::new();
        v.extend_from_slice(cmdlist::plain_fns());
        v.extend_from_slice(descheap::plain_fns());
        v.extend_from_slice(queue::plain_fns());
        v.extend_from_slice(fence::plain_fns());
        v.extend_from_slice(resource::plain_fns());
        v.into_boxed_slice()
    })
}
