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

/// `ID3D12RootSignatureDeserializer` and `ID3D12VersionedRootSignatureDeserializer`.
///
/// Materializes exact Windows x64 ABI structs (`D3D12_ROOT_SIGNATURE_DESC` /
/// `D3D12_VERSIONED_ROOT_SIGNATURE_DESC`) in pinned heap storage owned by the COM
/// object so returned pointers stay valid for the object's lifetime.
mod deserializer {
    use super::com::*;
    use super::vtables::{
        IID_ID3D12RootSignatureDeserializer, IID_ID3D12VersionedRootSignatureDeserializer,
    };
    use crate::types::*;
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRangeType, RootDescriptorKind, RootParameterKind,
        RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::ffi::c_void;
    use std::sync::{Mutex, OnceLock};

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct RawRange10 {
        range_type: u32,
        num_descriptors: u32,
        base_shader_register: u32,
        register_space: u32,
        offset_in_descriptors_from_table_start: u32,
    }
    const _: () = assert!(std::mem::size_of::<RawRange10>() == 20);

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct RawRange11 {
        range_type: u32,
        num_descriptors: u32,
        base_shader_register: u32,
        register_space: u32,
        flags: u32,
        offset_in_descriptors_from_table_start: u32,
    }
    const _: () = assert!(std::mem::size_of::<RawRange11>() == 24);

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct RawRootParam {
        parameter_type: u32,
        _pad0: u32,
        u0: u32,
        u1: u32,
        u2: u64,
        shader_visibility: u32,
        _pad1: u32,
    }
    const _: () = assert!(std::mem::size_of::<RawRootParam>() == 32);

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct RawStaticSampler10 {
        filter: u32,
        address_u: u32,
        address_v: u32,
        address_w: u32,
        mip_lod_bias: f32,
        max_anisotropy: u32,
        comparison_func: u32,
        border_color: u32,
        min_lod: f32,
        max_lod: f32,
        shader_register: u32,
        register_space: u32,
        shader_visibility: u32,
    }
    const _: () = assert!(std::mem::size_of::<RawStaticSampler10>() == 52);

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct RawStaticSampler12 {
        base: RawStaticSampler10,
        flags: u32,
    }
    const _: () = assert!(std::mem::size_of::<RawStaticSampler12>() == 56);

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub(super) struct RawRootSigDesc {
        num_parameters: u32,
        _pad0: u32,
        p_parameters: *const RawRootParam,
        num_static_samplers: u32,
        _pad1: u32,
        p_static_samplers: *const c_void,
        flags: u32,
        _pad2: u32,
    }
    const _: () = assert!(std::mem::size_of::<RawRootSigDesc>() == 40);

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub(super) struct RawVersionedRootSigDesc {
        version: u32,
        _pad: u32,
        desc: RawRootSigDesc,
    }
    const _: () = assert!(std::mem::size_of::<RawVersionedRootSigDesc>() == 48);

    #[allow(dead_code)] // Owns heap storage referenced by `RawRootParam::u2` pointers.
    enum RangeStorage {
        V10(Box<[RawRange10]>),
        V11(Box<[RawRange11]>),
    }

    #[allow(dead_code)] // Owns heap storage referenced by `RawRootSigDesc::p_static_samplers`.
    enum SamplerStorage {
        V10(Box<[RawStaticSampler10]>),
        V12(Box<[RawStaticSampler12]>),
    }

    /// Heap-stable storage for one materialized `D3D12_VERSIONED_ROOT_SIGNATURE_DESC`.
    /// All internal pointers point into the owned `Box<[_]>` buffers stored alongside it.
    struct MaterializedDesc {
        versioned: Box<RawVersionedRootSigDesc>,
        _params: Box<[RawRootParam]>,
        _ranges: Vec<RangeStorage>,
        _samplers: SamplerStorage,
    }
    // SAFETY: Immutable after construction; pointers point strictly into owned boxes.
    unsafe impl Send for MaterializedDesc {}
    unsafe impl Sync for MaterializedDesc {}

    fn static_sampler_10(s: &StaticSampler) -> RawStaticSampler10 {
        RawStaticSampler10 {
            filter: s.filter,
            address_u: s.address_u,
            address_v: s.address_v,
            address_w: s.address_w,
            mip_lod_bias: s.mip_lod_bias,
            max_anisotropy: s.max_anisotropy,
            comparison_func: s.comparison_func,
            border_color: s.border_color,
            min_lod: s.min_lod,
            max_lod: s.max_lod,
            shader_register: s.shader_register,
            register_space: s.register_space,
            shader_visibility: s.visibility,
        }
    }

    impl MaterializedDesc {
        fn build(desc: &RootSignatureDesc, target: RootSignatureVersion) -> Result<Self, HRESULT> {
            // Down-converting a v1.2 root signature that uses v1.2-only static sampler flags
            // cannot be faithfully represented in v1.0 or v1.1.
            if target < RootSignatureVersion::V1_2
                && desc.static_samplers.iter().any(|s| s.flags != 0)
            {
                return Err(E_INVALIDARG);
            }

            let target_v10 = target == RootSignatureVersion::V1_0;
            let src_v10 = desc.version == RootSignatureVersion::V1_0;
            let mut ranges_keep: Vec<RangeStorage> = Vec::new();
            let mut params = Vec::with_capacity(desc.parameters.len());

            for p in &desc.parameters {
                let raw = match &p.kind {
                    RootParameterKind::DescriptorTable(rs) => {
                        let ptr: *const c_void = if rs.is_empty() {
                            std::ptr::null()
                        } else if target_v10 {
                            let boxed: Box<[RawRange10]> = rs
                                .iter()
                                .map(|r| RawRange10 {
                                    range_type: r.range_type as u32,
                                    num_descriptors: r.num_descriptors,
                                    base_shader_register: r.base_shader_register,
                                    register_space: r.register_space,
                                    offset_in_descriptors_from_table_start: r
                                        .offset_in_descriptors_from_table_start,
                                })
                                .collect::<Vec<_>>()
                                .into_boxed_slice();
                            let ptr = boxed.as_ptr() as *const c_void;
                            ranges_keep.push(RangeStorage::V10(boxed));
                            ptr
                        } else {
                            let boxed: Box<[RawRange11]> = rs
                                .iter()
                                .map(|r| {
                                    let flags = if src_v10 {
                                        if r.range_type == DescriptorRangeType::Sampler {
                                            rs_flags::RANGE_V1_0_DEFAULT_SAMPLER
                                        } else {
                                            rs_flags::RANGE_V1_0_DEFAULT_CBV_SRV_UAV
                                        }
                                    } else {
                                        r.flags
                                    };
                                    RawRange11 {
                                        range_type: r.range_type as u32,
                                        num_descriptors: r.num_descriptors,
                                        base_shader_register: r.base_shader_register,
                                        register_space: r.register_space,
                                        flags,
                                        offset_in_descriptors_from_table_start: r
                                            .offset_in_descriptors_from_table_start,
                                    }
                                })
                                .collect::<Vec<_>>()
                                .into_boxed_slice();
                            let ptr = boxed.as_ptr() as *const c_void;
                            ranges_keep.push(RangeStorage::V11(boxed));
                            ptr
                        };
                        RawRootParam {
                            parameter_type: 0,
                            _pad0: 0,
                            u0: rs.len() as u32,
                            u1: 0,
                            u2: ptr as usize as u64,
                            shader_visibility: p.visibility,
                            _pad1: 0,
                        }
                    }
                    RootParameterKind::Constants {
                        shader_register,
                        register_space,
                        num_32bit_values,
                    } => RawRootParam {
                        parameter_type: 1,
                        _pad0: 0,
                        u0: *shader_register,
                        u1: *register_space,
                        u2: *num_32bit_values as u64,
                        shader_visibility: p.visibility,
                        _pad1: 0,
                    },
                    RootParameterKind::Descriptor(kind, d) => {
                        let ty = match kind {
                            RootDescriptorKind::Cbv => 2,
                            RootDescriptorKind::Srv => 3,
                            RootDescriptorKind::Uav => 4,
                        };
                        let flags = if target_v10 {
                            0
                        } else if src_v10 {
                            rs_flags::ROOT_DESCRIPTOR_V1_0_DEFAULT
                        } else {
                            d.flags
                        };
                        RawRootParam {
                            parameter_type: ty,
                            _pad0: 0,
                            u0: d.shader_register,
                            u1: d.register_space,
                            u2: flags as u64,
                            shader_visibility: p.visibility,
                            _pad1: 0,
                        }
                    }
                };
                params.push(raw);
            }

            let params_box = params.into_boxed_slice();
            let p_parameters = if params_box.is_empty() {
                std::ptr::null()
            } else {
                params_box.as_ptr()
            };

            let (samplers_storage, p_static_samplers) = if target == RootSignatureVersion::V1_2 {
                let boxed: Box<[RawStaticSampler12]> = desc
                    .static_samplers
                    .iter()
                    .map(|s| RawStaticSampler12 { base: static_sampler_10(s), flags: s.flags })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                let ptr = if boxed.is_empty() {
                    std::ptr::null()
                } else {
                    boxed.as_ptr() as *const c_void
                };
                (SamplerStorage::V12(boxed), ptr)
            } else {
                let boxed: Box<[RawStaticSampler10]> = desc
                    .static_samplers
                    .iter()
                    .map(static_sampler_10)
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                let ptr = if boxed.is_empty() {
                    std::ptr::null()
                } else {
                    boxed.as_ptr() as *const c_void
                };
                (SamplerStorage::V10(boxed), ptr)
            };

            let versioned = Box::new(RawVersionedRootSigDesc {
                version: target as u32,
                _pad: 0,
                desc: RawRootSigDesc {
                    num_parameters: params_box.len() as u32,
                    _pad0: 0,
                    p_parameters,
                    num_static_samplers: desc.static_samplers.len() as u32,
                    _pad1: 0,
                    p_static_samplers,
                    flags: desc.flags,
                    _pad2: 0,
                },
            });

            Ok(Self {
                versioned,
                _params: params_box,
                _ranges: ranges_keep,
                _samplers: samplers_storage,
            })
        }

        fn versioned_ptr(&self) -> *const RawVersionedRootSigDesc {
            &*self.versioned as *const RawVersionedRootSigDesc
        }

        fn desc_ptr(&self) -> *const RawRootSigDesc {
            &self.versioned.desc as *const RawRootSigDesc
        }
    }

    pub struct RootSigDeserializer {
        v10: MaterializedDesc,
    }

    static DESER_CLASS: OnceLock<Class> = OnceLock::new();
    impl ComClass for RootSigDeserializer {
        const TRACE_RELEASE: bool = false;
        fn class() -> &'static Class {
            class_once(&DESER_CLASS, || {
                let mut s = iunknown_slots::<RootSigDeserializer>();
                s.push(slot!("GetRootSignatureDesc" => get_root_sig_desc));
                Class {
                    name: "ID3D12RootSignatureDeserializer",
                    vtbl: build_vtable("ID3D12RootSignatureDeserializer", &s),
                    iids: &[IID_ID3D12RootSignatureDeserializer],
                }
            })
        }
    }

    unsafe extern "win64" fn get_root_sig_desc(
        this: *mut Obj<RootSigDeserializer>,
    ) -> *const RawRootSigDesc {
        Obj::get(this).v10.desc_ptr()
    }

    pub struct VersionedRootSigDeserializer {
        parsed: RootSignatureDesc,
        unconverted: MaterializedDesc,
        converted: Mutex<[Option<MaterializedDesc>; 3]>,
    }

    static VDESER_CLASS: OnceLock<Class> = OnceLock::new();
    impl ComClass for VersionedRootSigDeserializer {
        const TRACE_RELEASE: bool = false;
        fn class() -> &'static Class {
            class_once(&VDESER_CLASS, || {
                let mut s = iunknown_slots::<VersionedRootSigDeserializer>();
                s.push(slot!("GetRootSignatureDescAtVersion" => get_desc_at_version));
                s.push(slot!("GetUnconvertedRootSignatureDesc" => get_unconverted_desc));
                Class {
                    name: "ID3D12VersionedRootSignatureDeserializer",
                    vtbl: build_vtable("ID3D12VersionedRootSignatureDeserializer", &s),
                    iids: &[IID_ID3D12VersionedRootSignatureDeserializer],
                }
            })
        }
    }

    unsafe extern "win64" fn get_unconverted_desc(
        this: *mut Obj<VersionedRootSigDeserializer>,
    ) -> *const RawVersionedRootSigDesc {
        Obj::get(this).unconverted.versioned_ptr()
    }

    unsafe extern "win64" fn get_desc_at_version(
        this: *mut Obj<VersionedRootSigDeserializer>,
        convert_to_version: u32,
        out_desc: *mut *const RawVersionedRootSigDesc,
    ) -> HRESULT {
        if !out_desc.is_null() {
            std::ptr::write_unaligned(out_desc, std::ptr::null());
        }
        if out_desc.is_null() {
            return E_INVALIDARG;
        }
        let Some(target) = RootSignatureVersion::from_raw(convert_to_version) else {
            return E_INVALIDARG;
        };
        let d = Obj::get(this);
        if target == d.parsed.version {
            std::ptr::write_unaligned(out_desc, d.unconverted.versioned_ptr());
            return S_OK;
        }
        let idx = (target as usize) - 1;
        let mut cache = d.converted.lock().unwrap();
        if cache[idx].is_none() {
            match MaterializedDesc::build(&d.parsed, target) {
                Ok(m) => cache[idx] = Some(m),
                Err(hr) => return hr,
            }
        }
        let ptr = cache[idx].as_ref().unwrap().versioned_ptr();
        std::ptr::write_unaligned(out_desc, ptr);
        S_OK
    }

    pub unsafe fn create_v10(
        data: *const c_void,
        size: usize,
        riid: *const Guid,
        out: *mut *mut c_void,
    ) -> HRESULT {
        if !out.is_null() {
            std::ptr::write_unaligned(out, std::ptr::null_mut());
        }
        if data.is_null() || size == 0 || riid.is_null() || out.is_null() {
            return E_INVALIDARG;
        }
        let bytes = std::slice::from_raw_parts(data as *const u8, size);
        let parsed = match RootSignatureDesc::from_blob(bytes) {
            Ok(d) => d,
            Err(e) => return super::failed!(E_INVALIDARG, "D3D12CreateRootSignatureDeserializer: {e}"),
        };
        let v10 = match MaterializedDesc::build(&parsed, RootSignatureVersion::V1_0) {
            Ok(m) => m,
            Err(hr) => return hr,
        };
        let obj = Obj::new(RootSigDeserializer { v10 });
        let hr = query_out(obj as *mut c_void, riid, out);
        Obj::release(obj);
        hr
    }

    pub unsafe fn create_versioned(
        data: *const c_void,
        size: usize,
        riid: *const Guid,
        out: *mut *mut c_void,
    ) -> HRESULT {
        if !out.is_null() {
            std::ptr::write_unaligned(out, std::ptr::null_mut());
        }
        if data.is_null() || size == 0 || riid.is_null() || out.is_null() {
            return E_INVALIDARG;
        }
        let bytes = std::slice::from_raw_parts(data as *const u8, size);
        let parsed = match RootSignatureDesc::from_blob(bytes) {
            Ok(d) => d,
            Err(e) => {
                return super::failed!(
                    E_INVALIDARG,
                    "D3D12CreateVersionedRootSignatureDeserializer: {e}"
                )
            }
        };
        let unconverted = match MaterializedDesc::build(&parsed, parsed.version) {
            Ok(m) => m,
            Err(hr) => return hr,
        };
        let obj = Obj::new(VersionedRootSigDeserializer {
            parsed,
            unconverted,
            converted: Mutex::new([None, None, None]),
        });
        let hr = query_out(obj as *mut c_void, riid, out);
        Obj::release(obj);
        hr
    }
}

pub unsafe extern "win64" fn D3D12CreateRootSignatureDeserializer(
    data: *const c_void,
    size: usize,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    register_names();
    deserializer::create_v10(data, size, riid, out)
}

pub unsafe extern "win64" fn D3D12CreateVersionedRootSignatureDeserializer(
    data: *const c_void,
    size: usize,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    register_names();
    deserializer::create_versioned(data, size, riid, out)
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

#[cfg(test)]
mod tests {
    use super::*;
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };

    unsafe fn vtbl_slot(obj: *mut c_void, slot: usize) -> *const () {
        let vt = *(obj as *const *const *const ());
        *vt.add(slot)
    }

    #[test]
    fn root_signature_deserializer_v10_round_trips_and_validates() {
        let orig = RootSignatureDesc {
            version: RootSignatureVersion::V1_0,
            parameters: vec![
                RootParameter {
                    kind: RootParameterKind::DescriptorTable(vec![
                        DescriptorRange {
                            range_type: DescriptorRangeType::Srv,
                            num_descriptors: 4,
                            base_shader_register: 1,
                            register_space: 0,
                            flags: 0,
                            offset_in_descriptors_from_table_start: 0,
                        },
                        DescriptorRange {
                            range_type: DescriptorRangeType::Sampler,
                            num_descriptors: 2,
                            base_shader_register: 0,
                            register_space: 1,
                            flags: 0,
                            offset_in_descriptors_from_table_start:
                                rs_flags::DESCRIPTOR_RANGE_OFFSET_APPEND,
                        },
                    ]),
                    visibility: 5,
                },
                RootParameter {
                    kind: RootParameterKind::Constants {
                        shader_register: 2,
                        register_space: 0,
                        num_32bit_values: 6,
                    },
                    visibility: 1,
                },
                RootParameter {
                    kind: RootParameterKind::Descriptor(
                        RootDescriptorKind::Cbv,
                        RootDescriptor { shader_register: 0, register_space: 3, flags: 0 },
                    ),
                    visibility: 0,
                },
            ],
            static_samplers: vec![StaticSampler {
                shader_register: 4,
                register_space: 1,
                visibility: 5,
                ..Default::default()
            }],
            flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        };
        let blob = orig.to_blob();

        unsafe {
            // Invalid args return E_INVALIDARG.
            let mut out: *mut c_void = 1usize as *mut c_void;
            assert_eq!(
                D3D12CreateRootSignatureDeserializer(
                    std::ptr::null(),
                    0,
                    &IID_ID3D12RootSignatureDeserializer,
                    &mut out,
                ),
                E_INVALIDARG
            );
            assert!(out.is_null());
            let garbage = [0xAAu8; 16];
            assert_eq!(
                D3D12CreateRootSignatureDeserializer(
                    garbage.as_ptr() as *const c_void,
                    garbage.len(),
                    &IID_ID3D12RootSignatureDeserializer,
                    &mut out,
                ),
                E_INVALIDARG
            );
            // Wrong IID -> E_NOINTERFACE.
            assert_eq!(
                D3D12CreateRootSignatureDeserializer(
                    blob.as_ptr() as *const c_void,
                    blob.len(),
                    &IID_ID3D12VersionedRootSignatureDeserializer,
                    &mut out,
                ),
                E_NOINTERFACE
            );

            assert_eq!(
                D3D12CreateRootSignatureDeserializer(
                    blob.as_ptr() as *const c_void,
                    blob.len(),
                    &IID_ID3D12RootSignatureDeserializer,
                    &mut out,
                ),
                S_OK
            );
            assert!(!out.is_null());

            let get_desc: unsafe extern "win64" fn(*mut c_void) -> *const u8 =
                std::mem::transmute(vtbl_slot(out, 3));
            let desc_ptr = get_desc(out);
            assert!(!desc_ptr.is_null());

            // Re-parse the raw Windows ABI struct through d3d12_metal::rootsig::desc_from_mem!
            let parsed = d3d12_metal::rootsig::desc_from_mem(RootSignatureVersion::V1_0, desc_ptr)
                .expect("desc_from_mem");
            assert_eq!(parsed.version, RootSignatureVersion::V1_0);
            assert_eq!(parsed.flags, orig.flags);
            assert_eq!(parsed.parameters.len(), 3);
            assert_eq!(parsed.static_samplers.len(), 1);
            assert_eq!(parsed.static_samplers[0].shader_register, 4);
            assert_eq!(parsed.static_samplers[0].register_space, 1);

            com_release(out);
        }
    }

    #[test]
    fn versioned_root_signature_deserializer_converts_between_versions() {
        let orig_v10 = RootSignatureDesc {
            version: RootSignatureVersion::V1_0,
            parameters: vec![
                RootParameter {
                    kind: RootParameterKind::DescriptorTable(vec![
                        DescriptorRange {
                            range_type: DescriptorRangeType::Srv,
                            num_descriptors: 3,
                            base_shader_register: 0,
                            register_space: 0,
                            flags: 0,
                            offset_in_descriptors_from_table_start: 0,
                        },
                        DescriptorRange {
                            range_type: DescriptorRangeType::Sampler,
                            num_descriptors: 1,
                            base_shader_register: 0,
                            register_space: 0,
                            flags: 0,
                            offset_in_descriptors_from_table_start: 0,
                        },
                    ]),
                    visibility: 0,
                },
                RootParameter {
                    kind: RootParameterKind::Descriptor(
                        RootDescriptorKind::Uav,
                        RootDescriptor { shader_register: 1, register_space: 2, flags: 0 },
                    ),
                    visibility: 5,
                },
            ],
            static_samplers: vec![StaticSampler::default()],
            flags: 0x1,
        };
        let blob10 = orig_v10.to_blob();

        unsafe {
            let mut deser: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3D12CreateVersionedRootSignatureDeserializer(
                    blob10.as_ptr() as *const c_void,
                    blob10.len(),
                    &IID_ID3D12VersionedRootSignatureDeserializer,
                    &mut deser,
                ),
                S_OK
            );
            assert!(!deser.is_null());

            let get_at: unsafe extern "win64" fn(*mut c_void, u32, *mut *const u8) -> HRESULT =
                std::mem::transmute(vtbl_slot(deser, 3));
            let get_unconv: unsafe extern "win64" fn(*mut c_void) -> *const u8 =
                std::mem::transmute(vtbl_slot(deser, 4));

            let unconv_ptr = get_unconv(deser);
            assert!(!unconv_ptr.is_null());
            let unconv = d3d12_metal::rootsig::desc_from_versioned(unconv_ptr).unwrap();
            assert_eq!(unconv.version, RootSignatureVersion::V1_0);

            // Up-convert 1.0 -> 1.1: must populate default v1.0 volatility flags.
            let mut v11_ptr: *const u8 = std::ptr::null();
            assert_eq!(get_at(deser, RootSignatureVersion::V1_1 as u32, &mut v11_ptr), S_OK);
            let v11 = d3d12_metal::rootsig::desc_from_versioned(v11_ptr).unwrap();
            assert_eq!(v11.version, RootSignatureVersion::V1_1);
            match &v11.parameters[0].kind {
                RootParameterKind::DescriptorTable(ranges) => {
                    assert_eq!(ranges[0].flags, rs_flags::RANGE_V1_0_DEFAULT_CBV_SRV_UAV);
                    assert_eq!(ranges[1].flags, rs_flags::RANGE_V1_0_DEFAULT_SAMPLER);
                }
                other => panic!("unexpected param 0: {other:?}"),
            }
            match &v11.parameters[1].kind {
                RootParameterKind::Descriptor(RootDescriptorKind::Uav, d) => {
                    assert_eq!(d.flags, rs_flags::ROOT_DESCRIPTOR_V1_0_DEFAULT);
                }
                other => panic!("unexpected param 1: {other:?}"),
            }

            // Up-convert 1.0 -> 1.2: static sampler gets flags == 0.
            let mut v12_ptr: *const u8 = std::ptr::null();
            assert_eq!(get_at(deser, RootSignatureVersion::V1_2 as u32, &mut v12_ptr), S_OK);
            let v12 = d3d12_metal::rootsig::desc_from_versioned(v12_ptr).unwrap();
            assert_eq!(v12.version, RootSignatureVersion::V1_2);
            assert_eq!(v12.static_samplers.len(), 1);
            assert_eq!(v12.static_samplers[0].flags, 0);

            // Invalid version / null out -> E_INVALIDARG.
            let mut bad_ptr: *const u8 = 1usize as *const u8;
            assert_eq!(get_at(deser, 99, &mut bad_ptr), E_INVALIDARG);
            assert!(bad_ptr.is_null());
            assert_eq!(get_at(deser, 2, std::ptr::null_mut()), E_INVALIDARG);

            com_release(deser);

            // v1.2 blob with non-zero static sampler flags: succeeds at v1.2, rejects down-conversion to v1.0/v1.1.
            let orig_v12 = RootSignatureDesc {
                version: RootSignatureVersion::V1_2,
                parameters: vec![],
                static_samplers: vec![StaticSampler {
                    flags: rs_flags::SAMPLER_FLAG_UINT_BORDER_COLOR,
                    ..Default::default()
                }],
                flags: 0,
            };
            let blob12 = orig_v12.to_blob();
            let mut deser12: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3D12CreateVersionedRootSignatureDeserializer(
                    blob12.as_ptr() as *const c_void,
                    blob12.len(),
                    &IID_ID3D12VersionedRootSignatureDeserializer,
                    &mut deser12,
                ),
                S_OK
            );
            let get_at12: unsafe extern "win64" fn(*mut c_void, u32, *mut *const u8) -> HRESULT =
                std::mem::transmute(vtbl_slot(deser12, 3));
            let mut p12: *const u8 = std::ptr::null();
            assert_eq!(get_at12(deser12, RootSignatureVersion::V1_2 as u32, &mut p12), S_OK);
            let parsed12 = d3d12_metal::rootsig::desc_from_versioned(p12).unwrap();
            assert_eq!(parsed12.static_samplers[0].flags, rs_flags::SAMPLER_FLAG_UINT_BORDER_COLOR);
            assert_eq!(get_at12(deser12, RootSignatureVersion::V1_1 as u32, &mut p12), E_INVALIDARG);
            assert_eq!(get_at12(deser12, RootSignatureVersion::V1_0 as u32, &mut p12), E_INVALIDARG);
            com_release(deser12);
        }
    }

    #[test]
    fn create_graphics_and_stream_tessellation_psos() {
        if !dxil_metal::converter_available() || d3d12_metal::metal::device().is_none() {
            return;
        }
        fn sign(raw: &[u8]) -> Vec<u8> {
            let mut v = raw.to_vec();
            dxil_metal::dxbc::sign_container(&mut v).unwrap();
            v
        }
        let vs = sign(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../d3d12-metal/tests/fixtures/tess_tri.vs.dxil"
        )));
        let hs = sign(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../d3d12-metal/tests/fixtures/tess_tri.hs.dxil"
        )));
        let ds = sign(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../d3d12-metal/tests/fixtures/tess_tri.ds.dxil"
        )));
        let gs = sign(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../d3d12-metal/tests/fixtures/tess_swizzle.gs.dxil"
        )));
        let ps = sign(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../d3d12-metal/tests/fixtures/tess_color.ps.dxil"
        )));

        let il = [
            pso::InputElementDesc {
                name: b"POSITION\0".as_ptr(),
                semantic_index: 0,
                format: 2,
                input_slot: 0,
                aligned_byte_offset: 0,
                slot_class: 0,
                step_rate: 0,
            },
            pso::InputElementDesc {
                name: b"COLOR\0".as_ptr(),
                semantic_index: 0,
                format: 2,
                input_slot: 0,
                aligned_byte_offset: 16,
                slot_class: 0,
                step_rate: 0,
            },
        ];

        unsafe {
            let mut rs_com: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                pso::create_root_signature(vs.as_ptr(), vs.len(), &IID_ID3D12RootSignature, &mut rs_com),
                S_OK
            );
            assert!(!rs_com.is_null());

            // Build a 656-byte D3D12_GRAPHICS_PIPELINE_STATE_DESC buffer.
            let mk_desc = |hs_s: Option<&[u8]>, ds_s: Option<&[u8]>, gs_s: Option<&[u8]>, so_entries: u32| -> [u8; 656] {
                let mut d = [0u8; 656];
                let put_u64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());
                let put_u32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
                let put_bc = |b: &mut [u8], o: usize, s: Option<&[u8]>| {
                    if let Some(s) = s {
                        put_u64(b, o, s.as_ptr() as u64);
                        put_u64(b, o + 8, s.len() as u64);
                    }
                };
                put_u64(&mut d, 0, rs_com as u64);
                put_bc(&mut d, 8, Some(&vs));
                put_bc(&mut d, 24, Some(&ps));
                put_bc(&mut d, 40, ds_s);
                put_bc(&mut d, 56, hs_s);
                put_bc(&mut d, 72, gs_s);
                put_u32(&mut d, 96, so_entries);
                // BlendDesc default write mask = 0xF for RT0 (offset 120 + 8 + 36 = 164? Write full BlendDesc).
                let mut blend = d3d12_metal::state::BlendDesc::default();
                blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
                d[120..120 + std::mem::size_of::<d3d12_metal::state::BlendDesc>()]
                    .copy_from_slice(std::slice::from_raw_parts(
                        &blend as *const _ as *const u8,
                        std::mem::size_of::<d3d12_metal::state::BlendDesc>(),
                    ));
                put_u32(&mut d, 448, u32::MAX); // SampleMask
                put_u32(&mut d, 452, 3); // FillMode = SOLID
                put_u32(&mut d, 456, 1); // CullMode = NONE
                put_u64(&mut d, 552, il.as_ptr() as u64);
                put_u32(&mut d, 560, il.len() as u32);
                put_u32(&mut d, 572, d3d12_metal::topology::topology_type::PATCH);
                put_u32(&mut d, 576, 1); // NumRenderTargets
                put_u32(&mut d, 580, 28); // RTVFormats[0] = RGBA8_UNORM
                put_u32(&mut d, 616, 1); // SampleCount
                d
            };

            // 1. Mismatched HS-only / DS-only must fail honestly with E_INVALIDARG.
            let mut out: *mut c_void = std::ptr::null_mut();
            let d_hs_only = mk_desc(Some(&hs), None, None, 0);
            assert_eq!(pso::create_graphics_pipeline(d_hs_only.as_ptr(), &IID_ID3D12PipelineState, &mut out), E_INVALIDARG);
            let d_ds_only = mk_desc(None, Some(&ds), None, 0);
            assert_eq!(pso::create_graphics_pipeline(d_ds_only.as_ptr(), &IID_ID3D12PipelineState, &mut out), E_INVALIDARG);

            // 2. Stream output stays honestly E_NOTIMPL.
            let d_so = mk_desc(Some(&hs), Some(&ds), None, 1);
            assert_eq!(pso::create_graphics_pipeline(d_so.as_ptr(), &IID_ID3D12PipelineState, &mut out), E_NOTIMPL);

            // 3. Valid VS+HS+DS+PS via CreateGraphicsPipelineState.
            let d_tess = mk_desc(Some(&hs), Some(&ds), None, 0);
            assert_eq!(pso::create_graphics_pipeline(d_tess.as_ptr(), &IID_ID3D12PipelineState, &mut out), S_OK);
            assert!(!out.is_null());
            {
                let p = pso::pso_from_ptr(out).unwrap();
                let pso::PipelineKind::Graphics(ref g) = p.kind else { panic!("expected graphics PSO") };
                assert!(g.tess_emu.is_some(), "CreateGraphicsPipelineState must produce tess_emu");
                assert!(g.tess_emu.as_ref().unwrap().gs.is_none());
            }
            com_release(out);

            // 4. Valid VS+HS+DS+GS+PS via CreatePipelineState (D3D12_PIPELINE_STATE_STREAM_DESC).
            let mut stream = Vec::<u8>::new();
            let push_sub = |buf: &mut Vec<u8>, ty: u32, align: usize, payload: &[u8]| {
                buf.extend_from_slice(&ty.to_le_bytes());
                while buf.len() % align != 0 {
                    buf.push(0);
                }
                buf.extend_from_slice(payload);
                while buf.len() % 8 != 0 {
                    buf.push(0);
                }
            };
            let bc_bytes = |s: &[u8]| -> [u8; 16] {
                let mut b = [0u8; 16];
                b[0..8].copy_from_slice(&(s.as_ptr() as u64).to_le_bytes());
                b[8..16].copy_from_slice(&(s.len() as u64).to_le_bytes());
                b
            };
            push_sub(&mut stream, 0, 8, &(rs_com as u64).to_le_bytes()); // ROOT_SIGNATURE
            push_sub(&mut stream, 1, 8, &bc_bytes(&vs)); // VS
            push_sub(&mut stream, 2, 8, &bc_bytes(&ps)); // PS
            push_sub(&mut stream, 3, 8, &bc_bytes(&ds)); // DS
            push_sub(&mut stream, 4, 8, &bc_bytes(&hs)); // HS
            push_sub(&mut stream, 5, 8, &bc_bytes(&gs)); // GS
            let mut il_sub = [0u8; 16];
            il_sub[0..8].copy_from_slice(&(il.as_ptr() as u64).to_le_bytes());
            il_sub[8..12].copy_from_slice(&(il.len() as u32).to_le_bytes());
            push_sub(&mut stream, 12, 8, &il_sub); // INPUT_LAYOUT
            push_sub(&mut stream, 14, 4, &d3d12_metal::topology::topology_type::PATCH.to_le_bytes());
            let mut rtv_sub = [0u8; 36];
            rtv_sub[0..4].copy_from_slice(&28u32.to_le_bytes());
            rtv_sub[32..36].copy_from_slice(&1u32.to_le_bytes());
            push_sub(&mut stream, 15, 4, &rtv_sub); // RENDER_TARGET_FORMATS

            let sdesc = pso::StreamDesc { size: stream.len(), stream: stream.as_ptr() };
            let mut out_stream: *mut c_void = std::ptr::null_mut();
            assert_eq!(pso::create_pipeline_state(&sdesc, &IID_ID3D12PipelineState, &mut out_stream), S_OK);
            assert!(!out_stream.is_null());
            {
                let p = pso::pso_from_ptr(out_stream).unwrap();
                let pso::PipelineKind::Graphics(ref g) = p.kind else { panic!("expected graphics PSO") };
                let emu = g.tess_emu.as_ref().expect("stream PSO must produce tess_emu");
                assert!(emu.gs.is_some(), "stream PSO must preserve GS");
            }
            com_release(out_stream);
            com_release(rs_com);
        }
    }
}
