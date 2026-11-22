//! Exact Windows x64 Direct3D 11.0 / 11.1 structs, constants, enums, and GUIDs.

use crate::d3d12::com::{guid, name_iid, Guid};
use crate::types::{BOOL, HRESULT};
use std::ffi::c_void;
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Feature levels, driver types, SDK version, creation flags
// ---------------------------------------------------------------------------

pub const D3D_FEATURE_LEVEL_9_1: u32 = 0x9100;
pub const D3D_FEATURE_LEVEL_9_2: u32 = 0x9200;
pub const D3D_FEATURE_LEVEL_9_3: u32 = 0x9300;
pub const D3D_FEATURE_LEVEL_10_0: u32 = 0xa000;
pub const D3D_FEATURE_LEVEL_10_1: u32 = 0xa100;
pub const D3D_FEATURE_LEVEL_11_0: u32 = 0xb000;
pub const D3D_FEATURE_LEVEL_11_1: u32 = 0xb100;
pub const D3D_FEATURE_LEVEL_12_0: u32 = 0xc000;
pub const D3D_FEATURE_LEVEL_12_1: u32 = 0xc100;

pub const D3D11_SDK_VERSION: u32 = 7;

pub mod driver_type {
    pub const UNKNOWN: u32 = 0;
    pub const HARDWARE: u32 = 1;
    pub const REFERENCE: u32 = 2;
    pub const NULL: u32 = 3;
    pub const SOFTWARE: u32 = 4;
    pub const WARP: u32 = 5;
}

pub mod create_device_flag {
    pub const SINGLETHREADED: u32 = 0x1;
    pub const DEBUG: u32 = 0x2;
    pub const SWITCH_TO_REF: u32 = 0x4;
    pub const PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: u32 = 0x8;
    pub const BGRA_SUPPORT: u32 = 0x20;
    pub const DEBUGGABLE: u32 = 0x40;
    pub const PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: u32 = 0x80;
    pub const DISABLE_GPU_TIMEOUT: u32 = 0x100;
    pub const VIDEO_SUPPORT: u32 = 0x800;
}

pub const D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS: HRESULT = 0x887C_0001u32 as i32;
pub const D3D11_ERROR_FILE_NOT_FOUND: HRESULT = 0x887C_0002u32 as i32;
pub const D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS: HRESULT = 0x887C_0003u32 as i32;
pub const D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD: HRESULT = 0x887C_0004u32 as i32;
pub const E_HANDLE: HRESULT = 0x8007_0006u32 as i32;
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: HRESULT = 0x887A_002Cu32 as i32;

// ---------------------------------------------------------------------------
// D3D11 slot limits
// ---------------------------------------------------------------------------

pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: usize = 14;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: usize = 128;
pub const D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT: usize = 16;
pub const D3D11_PS_CS_UAV_REGISTER_COUNT: usize = 8;
pub const D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT: usize = 8;
pub const D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: usize = 32;
pub const D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: usize = 16;
pub const D3D11_SO_BUFFER_SLOT_COUNT: usize = 4;

// ---------------------------------------------------------------------------
// Usage, Bind, CPU access, Misc, Map, Clear, Query, Feature enums
// ---------------------------------------------------------------------------

pub mod usage {
    pub const DEFAULT: u32 = 0;
    pub const IMMUTABLE: u32 = 1;
    pub const DYNAMIC: u32 = 2;
    pub const STAGING: u32 = 3;
}

pub mod bind_flag {
    pub const VERTEX_BUFFER: u32 = 0x1;
    pub const INDEX_BUFFER: u32 = 0x2;
    pub const CONSTANT_BUFFER: u32 = 0x4;
    pub const SHADER_RESOURCE: u32 = 0x8;
    pub const STREAM_OUTPUT: u32 = 0x10;
    pub const RENDER_TARGET: u32 = 0x20;
    pub const DEPTH_STENCIL: u32 = 0x40;
    pub const UNORDERED_ACCESS: u32 = 0x80;
}

pub mod cpu_access {
    pub const WRITE: u32 = 0x10000;
    pub const READ: u32 = 0x20000;
}

pub mod misc_flag {
    pub const GENERATE_MIPS: u32 = 0x1;
    pub const SHARED: u32 = 0x2;
    pub const TEXTURECUBE: u32 = 0x4;
    pub const DRAWINDIRECT_ARGS: u32 = 0x10;
    pub const BUFFER_ALLOW_RAW_VIEWS: u32 = 0x20;
    pub const BUFFER_STRUCTURED: u32 = 0x40;
    pub const RESOURCE_CLAMP: u32 = 0x80;
    pub const SHARED_KEYEDMUTEX: u32 = 0x100;
    pub const GDI_COMPATIBLE: u32 = 0x200;
    pub const SHARED_NTHANDLE: u32 = 0x800;
}

pub mod dxgi_map {
    pub const READ: u32 = 1;
    pub const WRITE: u32 = 2;
    pub const DISCARD: u32 = 4;
}

pub mod dxgi_shared_resource {
    pub const READ: u32 = 0x8000_0000;
    pub const WRITE: u32 = 1;
}

pub mod map_type {
    pub const READ: u32 = 1;
    pub const WRITE: u32 = 2;
    pub const READ_WRITE: u32 = 3;
    pub const WRITE_DISCARD: u32 = 4;
    pub const WRITE_NO_OVERWRITE: u32 = 5;
}

pub mod map_flag {
    pub const DO_NOT_WAIT: u32 = 0x100000;
}

pub mod resource_dim {
    pub const UNKNOWN: u32 = 0;
    pub const BUFFER: u32 = 1;
    pub const TEXTURE1D: u32 = 2;
    pub const TEXTURE2D: u32 = 3;
    pub const TEXTURE3D: u32 = 4;
}

pub mod clear_flag {
    pub const DEPTH: u32 = 0x1;
    pub const STENCIL: u32 = 0x2;
}

pub mod context_type {
    pub const IMMEDIATE: u32 = 0;
    pub const DEFERRED: u32 = 1;
}

pub mod async_getdata_flag {
    pub const DO_NOT_FLUSH: u32 = 0x1;
}

pub mod query_kind {
    pub const EVENT: u32 = 0;
    pub const OCCLUSION: u32 = 1;
    pub const TIMESTAMP: u32 = 2;
    pub const TIMESTAMP_DISJOINT: u32 = 3;
    pub const PIPELINE_STATISTICS: u32 = 4;
    pub const OCCLUSION_PREDICATE: u32 = 5;
    pub const SO_STATISTICS: u32 = 6;
    pub const SO_OVERFLOW_PREDICATE: u32 = 7;
    pub const SO_STATISTICS_STREAM0: u32 = 8;
    pub const SO_OVERFLOW_PREDICATE_STREAM0: u32 = 9;
    pub const SO_STATISTICS_STREAM1: u32 = 10;
    pub const SO_OVERFLOW_PREDICATE_STREAM1: u32 = 11;
    pub const SO_STATISTICS_STREAM2: u32 = 12;
    pub const SO_OVERFLOW_PREDICATE_STREAM2: u32 = 13;
    pub const SO_STATISTICS_STREAM3: u32 = 14;
    pub const SO_OVERFLOW_PREDICATE_STREAM3: u32 = 15;
}

pub mod feature {
    pub const THREADING: u32 = 0;
    pub const DOUBLES: u32 = 1;
    pub const FORMAT_SUPPORT: u32 = 2;
    pub const FORMAT_SUPPORT2: u32 = 3;
    pub const D3D10_X_HARDWARE_OPTIONS: u32 = 4;
    pub const D3D11_OPTIONS: u32 = 5;
    pub const ARCHITECTURE_INFO: u32 = 6;
    pub const D3D9_OPTIONS: u32 = 7;
    pub const SHADER_MIN_PRECISION_SUPPORT: u32 = 8;
    pub const D3D9_SHADOW_SUPPORT: u32 = 9;
    pub const D3D11_OPTIONS1: u32 = 10;
    pub const D3D9_SIMPLE_INSTANCING_SUPPORT: u32 = 11;
    pub const MARKER_SUPPORT: u32 = 12;
    pub const D3D9_OPTIONS1: u32 = 13;
    pub const D3D11_OPTIONS2: u32 = 14;
    pub const D3D11_OPTIONS3: u32 = 15;
    pub const GPU_VIRTUAL_ADDRESS_SUPPORT: u32 = 16;
    pub const D3D11_OPTIONS4: u32 = 17;
    pub const SHADER_CACHE: u32 = 18;
    pub const D3D11_OPTIONS5: u32 = 19;
}

pub mod srv_dim {
    pub const UNKNOWN: u32 = 0;
    pub const BUFFER: u32 = 1;
    pub const TEXTURE1D: u32 = 2;
    pub const TEXTURE1DARRAY: u32 = 3;
    pub const TEXTURE2D: u32 = 4;
    pub const TEXTURE2DARRAY: u32 = 5;
    pub const TEXTURE2DMS: u32 = 6;
    pub const TEXTURE2DMSARRAY: u32 = 7;
    pub const TEXTURE3D: u32 = 8;
    pub const TEXTURECUBE: u32 = 9;
    pub const TEXTURECUBEARRAY: u32 = 10;
    pub const BUFFEREX: u32 = 11;
}

pub mod uav_dim {
    pub const UNKNOWN: u32 = 0;
    pub const BUFFER: u32 = 1;
    pub const TEXTURE1D: u32 = 2;
    pub const TEXTURE1DARRAY: u32 = 3;
    pub const TEXTURE2D: u32 = 4;
    pub const TEXTURE2DARRAY: u32 = 5;
    pub const TEXTURE3D: u32 = 8;
}

pub mod uav_flag {
    pub const RAW: u32 = 0x1;
    pub const APPEND: u32 = 0x2;
    pub const COUNTER: u32 = 0x4;
}

/// Sentinel passed in `pUAVInitialCounts` to leave a UAV's structure counter unchanged.
pub const D3D11_UAV_COUNTER_KEEP: u32 = 0xFFFF_FFFF;

pub mod rtv_dim {
    pub const UNKNOWN: u32 = 0;
    pub const BUFFER: u32 = 1;
    pub const TEXTURE1D: u32 = 2;
    pub const TEXTURE1DARRAY: u32 = 3;
    pub const TEXTURE2D: u32 = 4;
    pub const TEXTURE2DARRAY: u32 = 5;
    pub const TEXTURE2DMS: u32 = 6;
    pub const TEXTURE2DMSARRAY: u32 = 7;
    pub const TEXTURE3D: u32 = 8;
}

pub mod dsv_dim {
    pub const UNKNOWN: u32 = 0;
    pub const TEXTURE1D: u32 = 1;
    pub const TEXTURE1DARRAY: u32 = 2;
    pub const TEXTURE2D: u32 = 3;
    pub const TEXTURE2DARRAY: u32 = 4;
    pub const TEXTURE2DMS: u32 = 5;
    pub const TEXTURE2DMSARRAY: u32 = 6;
}

pub mod topology {
    pub const UNDEFINED: u32 = 0;
    pub const POINTLIST: u32 = 1;
    pub const LINELIST: u32 = 2;
    pub const LINESTRIP: u32 = 3;
    pub const TRIANGLELIST: u32 = 4;
    pub const TRIANGLESTRIP: u32 = 5;
    pub const TRIANGLEFAN: u32 = 6;
    pub const LINELIST_ADJ: u32 = 10;
    pub const LINESTRIP_ADJ: u32 = 11;
    pub const TRIANGLELIST_ADJ: u32 = 12;
    pub const TRIANGLESTRIP_ADJ: u32 = 13;
}

// ---------------------------------------------------------------------------
// GUIDs / IIDs
// ---------------------------------------------------------------------------

pub const IID_ID3D11DeviceChild: Guid = guid("1841e5c8-16b0-489b-bcc8-44cfb0d5deae");
pub const IID_ID3D11Asynchronous: Guid = guid("4b35d0cd-1e15-4258-9c98-1b1333f6dd3b");
pub const IID_ID3D11Query: Guid = guid("d6c00747-87b7-425e-b84d-44d108560afd");
pub const IID_ID3D11Predicate: Guid = guid("9eb576dd-9f77-4d86-81aa-8bab5fe490e2");
pub const IID_ID3D11Resource: Guid = guid("dc8e63f3-d12b-4952-b47b-5e45026a862d");
pub const IID_ID3D11Buffer: Guid = guid("48570b85-d1ee-4fcd-a250-eb350722b037");
pub const IID_ID3D11Texture1D: Guid = guid("f8fb5c27-c6b3-4f75-a4c8-439af2ef564c");
pub const IID_ID3D11Texture2D: Guid = guid("6f15aaf2-d208-4e89-9ab4-489535d34f9c");
pub const IID_ID3D11Texture3D: Guid = guid("037e866e-f56d-4357-a8af-9dabbe6e250e");
pub const IID_ID3D11View: Guid = guid("839d1216-bb2e-412b-b7f4-a9dbebe08ed1");
pub const IID_ID3D11ShaderResourceView: Guid = guid("b0e06fe0-8192-4e1a-b1ca-36d7414710b2");
pub const IID_ID3D11RenderTargetView: Guid = guid("dfdba067-0b8d-4865-875b-d7b4516cc164");
pub const IID_ID3D11DepthStencilView: Guid = guid("9fdac92a-1876-48c3-afad-25b94f84a9b6");
pub const IID_ID3D11UnorderedAccessView: Guid = guid("28acf509-7f5c-48f6-8611-f316010a6380");
pub const IID_ID3D11VertexShader: Guid = guid("3b301d64-d678-4289-8897-22f8928b72f3");
pub const IID_ID3D11GeometryShader: Guid = guid("38325b96-effb-4022-ba02-2e795b70275c");
pub const IID_ID3D11PixelShader: Guid = guid("ea82e40d-51dc-4f33-93d4-db7c9125ae8c");
pub const IID_ID3D11ComputeShader: Guid = guid("4f5b196e-c2bd-495e-bd01-1fded38e4969");
pub const IID_ID3D11InputLayout: Guid = guid("e4819ddc-4cf0-4025-bd26-5de82a3e07b7");
pub const IID_ID3D11BlendState: Guid = guid("75b68faa-347d-4159-8f45-a0640f01cd9a");
pub const IID_ID3D11BlendState1: Guid = guid("cc86fabe-da55-401d-85e7-e3c9de2877e9");
pub const IID_ID3D11DepthStencilState: Guid = guid("03823efb-8d8f-4e1c-9aa2-f64bb2cbfdf1");
pub const IID_ID3D11RasterizerState: Guid = guid("9bb4ab81-ab1a-4d8f-b506-fc04200b6ee7");
pub const IID_ID3D11RasterizerState1: Guid = guid("1217d7a6-5039-418c-b042-9cbe256afd6e");
pub const IID_ID3D11SamplerState: Guid = guid("da6fea51-564c-4487-9810-f0d0f9b4e3a5");
pub const IID_ID3D11CommandList: Guid = guid("a24bc4d1-769e-43f7-8013-98ff566c18e2");
pub const IID_ID3D11DeviceContext: Guid = guid("c0bfa96c-e089-44fb-8eaf-26f8796190da");
pub const IID_ID3D11DeviceContext1: Guid = guid("bb2c6faa-b5fb-4082-8e6b-388b8cfa90e1");
pub const IID_ID3D11Device: Guid = guid("db6f6ddb-ac77-4e88-8253-819df9bbf140");
pub const IID_ID3D11Device1: Guid = guid("a04bfb29-08ef-43d6-a49c-a9bdbdcbe686");

pub const IID_IDXGIDevice: Guid = guid("54ec77fa-1377-44e6-8c32-88fd5f44c84c");
pub const IID_IDXGIDevice1: Guid = guid("77db970f-6276-48ba-ba28-070143b4392c");
pub const IID_IDXGIDevice2: Guid = guid("05008617-fbfd-4051-a790-144884b4f6a9");
pub const IID_IDXGIDevice3: Guid = guid("6007896c-3244-4afd-bf18-a6d3beda5023");
pub const IID_IDXGIResource: Guid = guid("035f3ab4-482e-4e50-b41f-8a7f8bd8960b");
pub const IID_IDXGIResource1: Guid = guid("30961379-4609-4a41-998e-54fe567ee0c1");
pub const IID_IDXGISurface: Guid = guid("cafcb56c-6ac3-4889-bf47-9e23bbd260ec");
pub const IID_IDXGISurface1: Guid = guid("4ae63092-6327-4c1b-80ae-bfe12ea32b86");
pub const IID_IDXGISurface2: Guid = guid("aba496dd-b617-4cb8-a866-bc44d7eb1fa2");

pub fn register_names() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        for (g, n) in [
            (IID_ID3D11DeviceChild, "ID3D11DeviceChild"),
            (IID_ID3D11Asynchronous, "ID3D11Asynchronous"),
            (IID_ID3D11Query, "ID3D11Query"),
            (IID_ID3D11Predicate, "ID3D11Predicate"),
            (IID_ID3D11Resource, "ID3D11Resource"),
            (IID_ID3D11Buffer, "ID3D11Buffer"),
            (IID_ID3D11Texture1D, "ID3D11Texture1D"),
            (IID_ID3D11Texture2D, "ID3D11Texture2D"),
            (IID_ID3D11Texture3D, "ID3D11Texture3D"),
            (IID_ID3D11View, "ID3D11View"),
            (IID_ID3D11ShaderResourceView, "ID3D11ShaderResourceView"),
            (IID_ID3D11RenderTargetView, "ID3D11RenderTargetView"),
            (IID_ID3D11DepthStencilView, "ID3D11DepthStencilView"),
            (IID_ID3D11UnorderedAccessView, "ID3D11UnorderedAccessView"),
            (IID_ID3D11VertexShader, "ID3D11VertexShader"),
            (IID_ID3D11GeometryShader, "ID3D11GeometryShader"),
            (IID_ID3D11PixelShader, "ID3D11PixelShader"),
            (IID_ID3D11ComputeShader, "ID3D11ComputeShader"),
            (IID_ID3D11InputLayout, "ID3D11InputLayout"),
            (IID_ID3D11BlendState, "ID3D11BlendState"),
            (IID_ID3D11BlendState1, "ID3D11BlendState1"),
            (IID_ID3D11DepthStencilState, "ID3D11DepthStencilState"),
            (IID_ID3D11RasterizerState, "ID3D11RasterizerState"),
            (IID_ID3D11RasterizerState1, "ID3D11RasterizerState1"),
            (IID_ID3D11SamplerState, "ID3D11SamplerState"),
            (IID_ID3D11CommandList, "ID3D11CommandList"),
            (IID_ID3D11DeviceContext, "ID3D11DeviceContext"),
            (IID_ID3D11DeviceContext1, "ID3D11DeviceContext1"),
            (IID_ID3D11Device, "ID3D11Device"),
            (IID_ID3D11Device1, "ID3D11Device1"),
            (IID_IDXGIDevice, "IDXGIDevice"),
            (IID_IDXGIDevice1, "IDXGIDevice1"),
            (IID_IDXGIDevice2, "IDXGIDevice2"),
            (IID_IDXGIDevice3, "IDXGIDevice3"),
            (IID_IDXGIResource, "IDXGIResource"),
            (IID_IDXGIResource1, "IDXGIResource1"),
            (IID_IDXGISurface, "IDXGISurface"),
            (IID_IDXGISurface1, "IDXGISurface1"),
            (IID_IDXGISurface2, "IDXGISurface2"),
        ] {
            name_iid(g, n);
        }
        crate::dxgi::register_names();
    });
}

// ---------------------------------------------------------------------------
// Structs (exact Windows x64 ABI layouts)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D3D11_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: u32,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub StructureByteStride: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_BUFFER_DESC>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D3D11_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: u32,
    pub Usage: u32,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_TEXTURE1D_DESC>() == 32);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
const _: () = assert!(std::mem::size_of::<DXGI_SAMPLE_DESC>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D3D11_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: u32,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub Usage: u32,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_TEXTURE2D_DESC>() == 44);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D3D11_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: u32,
    pub Usage: u32,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_TEXTURE3D_DESC>() == 36);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub pSysMem: *const c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_SUBRESOURCE_DATA>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub pData: *mut c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_MAPPED_SUBRESOURCE>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_BOX>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
const _: () = assert!(std::mem::size_of::<D3D11_VIEWPORT>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: *const u8,
    pub SemanticIndex: u32,
    pub Format: u32,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: u32,
    pub InstanceDataStepRate: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_INPUT_ELEMENT_DESC>() == 32);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub Format: u32,
    pub ViewDimension: u32,
    pub u: [u32; 3],
}
const _: () = assert!(std::mem::size_of::<D3D11_RENDER_TARGET_VIEW_DESC>() == 20);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Format: u32,
    pub ViewDimension: u32,
    pub Flags: u32,
    pub u: [u32; 3],
}
const _: () = assert!(std::mem::size_of::<D3D11_DEPTH_STENCIL_VIEW_DESC>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Format: u32,
    pub ViewDimension: u32,
    pub u: [u32; 4],
}
const _: () = assert!(std::mem::size_of::<D3D11_SHADER_RESOURCE_VIEW_DESC>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: u32,
    pub ViewDimension: u32,
    pub u: [u32; 3],
}
const _: () = assert!(std::mem::size_of::<D3D11_UNORDERED_ACCESS_VIEW_DESC>() == 20);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_SAMPLER_DESC {
    pub Filter: u32,
    pub AddressU: u32,
    pub AddressV: u32,
    pub AddressW: u32,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: u32,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
const _: () = assert!(std::mem::size_of::<D3D11_SAMPLER_DESC>() == 52);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: BOOL,
    pub SrcBlend: u32,
    pub DestBlend: u32,
    pub BlendOp: u32,
    pub SrcBlendAlpha: u32,
    pub DestBlendAlpha: u32,
    pub BlendOpAlpha: u32,
    pub RenderTargetWriteMask: u8,
}
const _: () = assert!(std::mem::size_of::<D3D11_RENDER_TARGET_BLEND_DESC>() == 32);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct D3D11_BLEND_DESC {
    pub AlphaToCoverageEnable: BOOL,
    pub IndependentBlendEnable: BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}
const _: () = assert!(std::mem::size_of::<D3D11_BLEND_DESC>() == 264);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: BOOL,
    pub LogicOpEnable: BOOL,
    pub SrcBlend: u32,
    pub DestBlend: u32,
    pub BlendOp: u32,
    pub SrcBlendAlpha: u32,
    pub DestBlendAlpha: u32,
    pub BlendOpAlpha: u32,
    pub LogicOp: u32,
    pub RenderTargetWriteMask: u8,
}
const _: () = assert!(std::mem::size_of::<D3D11_RENDER_TARGET_BLEND_DESC1>() == 40);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct D3D11_BLEND_DESC1 {
    pub AlphaToCoverageEnable: BOOL,
    pub IndependentBlendEnable: BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC1; 8],
}
const _: () = assert!(std::mem::size_of::<D3D11_BLEND_DESC1>() == 328);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RASTERIZER_DESC {
    pub FillMode: u32,
    pub CullMode: u32,
    pub FrontCounterClockwise: BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: BOOL,
    pub ScissorEnable: BOOL,
    pub MultisampleEnable: BOOL,
    pub AntialiasedLineEnable: BOOL,
}
const _: () = assert!(std::mem::size_of::<D3D11_RASTERIZER_DESC>() == 40);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RASTERIZER_DESC1 {
    pub FillMode: u32,
    pub CullMode: u32,
    pub FrontCounterClockwise: BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: BOOL,
    pub ScissorEnable: BOOL,
    pub MultisampleEnable: BOOL,
    pub AntialiasedLineEnable: BOOL,
    pub ForcedSampleCount: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_RASTERIZER_DESC1>() == 44);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: u32,
    pub StencilDepthFailOp: u32,
    pub StencilPassOp: u32,
    pub StencilFunc: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_DEPTH_STENCILOP_DESC>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub DepthEnable: BOOL,
    pub DepthWriteMask: u32,
    pub DepthFunc: u32,
    pub StencilEnable: BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D11_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D11_DEPTH_STENCILOP_DESC,
}
const _: () = assert!(std::mem::size_of::<D3D11_DEPTH_STENCIL_DESC>() == 52);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct D3D11_QUERY_DESC {
    pub Query: u32,
    pub MiscFlags: u32,
}
const _: () = assert!(std::mem::size_of::<D3D11_QUERY_DESC>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: BOOL,
}
const _: () = assert!(std::mem::size_of::<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
    pub HSInvocations: u64,
    pub DSInvocations: u64,
    pub CSInvocations: u64,
}
const _: () = assert!(std::mem::size_of::<D3D11_QUERY_DATA_PIPELINE_STATISTICS>() == 88);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct D3D11_COUNTER_INFO {
    pub LastDeviceDependentCounter: u32,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
const _: () = assert!(std::mem::size_of::<D3D11_COUNTER_INFO>() == 12);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: u32,
    pub SampleDesc: DXGI_SAMPLE_DESC,
}
const _: () = assert!(std::mem::size_of::<DXGI_SURFACE_DESC>() == 20);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
const _: () = assert!(std::mem::size_of::<DXGI_MAPPED_RECT>() == 16);

// ---------------------------------------------------------------------------
// Conversion helpers to d3d12-metal types
// ---------------------------------------------------------------------------

impl D3D11_BLEND_DESC {
    pub fn default_d3d11() -> Self {
        let rt = D3D11_RENDER_TARGET_BLEND_DESC {
            BlendEnable: 0,
            SrcBlend: 2,      // ONE
            DestBlend: 1,     // ZERO
            BlendOp: 1,       // ADD
            SrcBlendAlpha: 2, // ONE
            DestBlendAlpha: 1,// ZERO
            BlendOpAlpha: 1,  // ADD
            RenderTargetWriteMask: 0xF,
        };
        Self { AlphaToCoverageEnable: 0, IndependentBlendEnable: 0, RenderTarget: [rt; 8] }
    }

    pub fn to_d3d12(&self) -> d3d12_metal::state::BlendDesc {
        let mut out = d3d12_metal::state::BlendDesc {
            AlphaToCoverageEnable: self.AlphaToCoverageEnable,
            IndependentBlendEnable: self.IndependentBlendEnable,
            RenderTarget: [d3d12_metal::state::RenderTargetBlendDesc::default(); 8],
        };
        for i in 0..8 {
            let r = &self.RenderTarget[i];
            out.RenderTarget[i] = d3d12_metal::state::RenderTargetBlendDesc {
                BlendEnable: r.BlendEnable,
                LogicOpEnable: 0,
                SrcBlend: r.SrcBlend,
                DestBlend: r.DestBlend,
                BlendOp: r.BlendOp,
                SrcBlendAlpha: r.SrcBlendAlpha,
                DestBlendAlpha: r.DestBlendAlpha,
                BlendOpAlpha: r.BlendOpAlpha,
                LogicOp: 4, // NOOP
                RenderTargetWriteMask: r.RenderTargetWriteMask,
            };
        }
        out
    }

    pub fn to_desc1(&self) -> D3D11_BLEND_DESC1 {
        let mut out = D3D11_BLEND_DESC1 {
            AlphaToCoverageEnable: self.AlphaToCoverageEnable,
            IndependentBlendEnable: self.IndependentBlendEnable,
            RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC1::default(); 8],
        };
        for i in 0..8 {
            let r = &self.RenderTarget[i];
            out.RenderTarget[i] = D3D11_RENDER_TARGET_BLEND_DESC1 {
                BlendEnable: r.BlendEnable,
                LogicOpEnable: 0,
                SrcBlend: r.SrcBlend,
                DestBlend: r.DestBlend,
                BlendOp: r.BlendOp,
                SrcBlendAlpha: r.SrcBlendAlpha,
                DestBlendAlpha: r.DestBlendAlpha,
                BlendOpAlpha: r.BlendOpAlpha,
                LogicOp: 4,
                RenderTargetWriteMask: r.RenderTargetWriteMask,
            };
        }
        out
    }
}

impl D3D11_BLEND_DESC1 {
    pub fn to_d3d12(&self) -> d3d12_metal::state::BlendDesc {
        let mut out = d3d12_metal::state::BlendDesc {
            AlphaToCoverageEnable: self.AlphaToCoverageEnable,
            IndependentBlendEnable: self.IndependentBlendEnable,
            RenderTarget: [d3d12_metal::state::RenderTargetBlendDesc::default(); 8],
        };
        for i in 0..8 {
            let r = &self.RenderTarget[i];
            out.RenderTarget[i] = d3d12_metal::state::RenderTargetBlendDesc {
                BlendEnable: r.BlendEnable,
                LogicOpEnable: r.LogicOpEnable,
                SrcBlend: r.SrcBlend,
                DestBlend: r.DestBlend,
                BlendOp: r.BlendOp,
                SrcBlendAlpha: r.SrcBlendAlpha,
                DestBlendAlpha: r.DestBlendAlpha,
                BlendOpAlpha: r.BlendOpAlpha,
                LogicOp: r.LogicOp,
                RenderTargetWriteMask: r.RenderTargetWriteMask,
            };
        }
        out
    }

    pub fn to_desc0(&self) -> D3D11_BLEND_DESC {
        let mut out = D3D11_BLEND_DESC {
            AlphaToCoverageEnable: self.AlphaToCoverageEnable,
            IndependentBlendEnable: self.IndependentBlendEnable,
            RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC::default(); 8],
        };
        for i in 0..8 {
            let r = &self.RenderTarget[i];
            out.RenderTarget[i] = D3D11_RENDER_TARGET_BLEND_DESC {
                BlendEnable: r.BlendEnable,
                SrcBlend: r.SrcBlend,
                DestBlend: r.DestBlend,
                BlendOp: r.BlendOp,
                SrcBlendAlpha: r.SrcBlendAlpha,
                DestBlendAlpha: r.DestBlendAlpha,
                BlendOpAlpha: r.BlendOpAlpha,
                RenderTargetWriteMask: r.RenderTargetWriteMask,
            };
        }
        out
    }
}

impl D3D11_RASTERIZER_DESC {
    pub fn default_d3d11() -> Self {
        Self {
            FillMode: 3, // SOLID
            CullMode: 3, // BACK
            FrontCounterClockwise: 0,
            DepthBias: 0,
            DepthBiasClamp: 0.0,
            SlopeScaledDepthBias: 0.0,
            DepthClipEnable: 1,
            ScissorEnable: 0,
            MultisampleEnable: 0,
            AntialiasedLineEnable: 0,
        }
    }

    pub fn to_desc1(&self) -> D3D11_RASTERIZER_DESC1 {
        D3D11_RASTERIZER_DESC1 {
            FillMode: self.FillMode,
            CullMode: self.CullMode,
            FrontCounterClockwise: self.FrontCounterClockwise,
            DepthBias: self.DepthBias,
            DepthBiasClamp: self.DepthBiasClamp,
            SlopeScaledDepthBias: self.SlopeScaledDepthBias,
            DepthClipEnable: self.DepthClipEnable,
            ScissorEnable: self.ScissorEnable,
            MultisampleEnable: self.MultisampleEnable,
            AntialiasedLineEnable: self.AntialiasedLineEnable,
            ForcedSampleCount: 0,
        }
    }
}

impl D3D11_RASTERIZER_DESC1 {
    pub fn to_desc0(&self) -> D3D11_RASTERIZER_DESC {
        D3D11_RASTERIZER_DESC {
            FillMode: self.FillMode,
            CullMode: self.CullMode,
            FrontCounterClockwise: self.FrontCounterClockwise,
            DepthBias: self.DepthBias,
            DepthBiasClamp: self.DepthBiasClamp,
            SlopeScaledDepthBias: self.SlopeScaledDepthBias,
            DepthClipEnable: self.DepthClipEnable,
            ScissorEnable: self.ScissorEnable,
            MultisampleEnable: self.MultisampleEnable,
            AntialiasedLineEnable: self.AntialiasedLineEnable,
        }
    }

    pub fn to_d3d12(&self) -> d3d12_metal::state::RasterizerDesc {
        d3d12_metal::state::RasterizerDesc {
            FillMode: self.FillMode,
            CullMode: self.CullMode,
            FrontCounterClockwise: self.FrontCounterClockwise,
            DepthBias: self.DepthBias,
            DepthBiasClamp: self.DepthBiasClamp,
            SlopeScaledDepthBias: self.SlopeScaledDepthBias,
            DepthClipEnable: self.DepthClipEnable,
            MultisampleEnable: self.MultisampleEnable,
            AntialiasedLineEnable: self.AntialiasedLineEnable,
            ForcedSampleCount: self.ForcedSampleCount,
            ConservativeRaster: 0,
        }
    }
}

impl D3D11_DEPTH_STENCIL_DESC {
    pub fn default_d3d11() -> Self {
        let op = D3D11_DEPTH_STENCILOP_DESC {
            StencilFailOp: 1,      // KEEP
            StencilDepthFailOp: 1, // KEEP
            StencilPassOp: 1,      // KEEP
            StencilFunc: 8,        // ALWAYS
        };
        Self {
            DepthEnable: 1,
            DepthWriteMask: 1, // ALL
            DepthFunc: 2,      // LESS
            StencilEnable: 0,
            StencilReadMask: 0xFF,
            StencilWriteMask: 0xFF,
            FrontFace: op,
            BackFace: op,
        }
    }

    pub fn to_d3d12(&self) -> d3d12_metal::state::DepthStencilDesc {
        d3d12_metal::state::DepthStencilDesc {
            DepthEnable: self.DepthEnable,
            DepthWriteMask: self.DepthWriteMask,
            DepthFunc: self.DepthFunc,
            StencilEnable: self.StencilEnable,
            StencilReadMask: self.StencilReadMask,
            StencilWriteMask: self.StencilWriteMask,
            FrontFace: d3d12_metal::state::DepthStencilOpDesc {
                StencilFailOp: self.FrontFace.StencilFailOp,
                StencilDepthFailOp: self.FrontFace.StencilDepthFailOp,
                StencilPassOp: self.FrontFace.StencilPassOp,
                StencilFunc: self.FrontFace.StencilFunc,
            },
            BackFace: d3d12_metal::state::DepthStencilOpDesc {
                StencilFailOp: self.BackFace.StencilFailOp,
                StencilDepthFailOp: self.BackFace.StencilDepthFailOp,
                StencilPassOp: self.BackFace.StencilPassOp,
                StencilFunc: self.BackFace.StencilFunc,
            },
        }
    }
}

impl D3D11_SAMPLER_DESC {
    pub fn to_d3d12(&self) -> d3d12_metal::metal::views::SamplerDesc {
        d3d12_metal::metal::views::SamplerDesc {
            filter: self.Filter,
            address: [self.AddressU, self.AddressV, self.AddressW],
            mip_lod_bias: self.MipLODBias,
            max_anisotropy: self.MaxAnisotropy,
            comparison: self.ComparisonFunc,
            border: self.BorderColor,
            min_lod: self.MinLOD,
            max_lod: self.MaxLOD,
        }
    }
}
