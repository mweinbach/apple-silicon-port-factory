//! Direct3D 11 shader objects, fixed-function state objects, queries,
//! deferred command lists, and canonical D3D11->D3D12 root signatures.

use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::metal::pipeline::ComputePipeline;
use d3d12_metal::metal::query::QueryHeap;
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::metal::views;
use dxil_metal::root_signature::{
    flags as rs_flags, DescriptorRange, DescriptorRangeType, RootParameter, RootParameterKind,
    RootSignature, RootSignatureDesc, RootSignatureVersion,
};
use dxil_metal::InputElement;

use super::com::*;
use super::context::RecordedCmd;
use super::device::DeviceCore;
use super::types::*;

// ---------------------------------------------------------------------------
// Canonical D3D11 Root Signatures (flat register tables for VS/PS and CS)
// ---------------------------------------------------------------------------

/// Descriptor counts per D3D11 shader stage in our canonical root signatures.
pub const RS_CBV_COUNT: u32 = D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT as u32; // 14
pub const RS_SRV_COUNT: u32 = D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT as u32; // 128
pub const RS_UAV_COUNT: u32 = D3D11_PS_CS_UAV_REGISTER_COUNT as u32; // 8
pub const RS_SAMPLER_COUNT: u32 = D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT as u32; // 16

pub const VS_RES_TABLE_LEN: u32 = RS_CBV_COUNT + RS_SRV_COUNT; // 142
pub const PS_RES_TABLE_LEN: u32 = RS_CBV_COUNT + RS_SRV_COUNT + RS_UAV_COUNT; // 150
pub const GS_RES_TABLE_LEN: u32 = RS_CBV_COUNT + RS_SRV_COUNT; // 142
pub const CS_RES_TABLE_LEN: u32 = RS_CBV_COUNT + RS_SRV_COUNT + RS_UAV_COUNT; // 150

fn range(ty: DescriptorRangeType, n: u32) -> DescriptorRange {
    DescriptorRange {
        range_type: ty,
        num_descriptors: n,
        base_shader_register: 0,
        register_space: 0,
        flags: if ty == DescriptorRangeType::Sampler {
            rs_flags::RANGE_V1_0_DEFAULT_SAMPLER
        } else {
            rs_flags::RANGE_V1_0_DEFAULT_CBV_SRV_UAV
        },
        offset_in_descriptors_from_table_start: rs_flags::DESCRIPTOR_RANGE_OFFSET_APPEND,
    }
}

/// Canonical Graphics root signature:
/// - Param 0 (VS): `[CBV b0..13, SRV t0..127]`
/// - Param 1 (VS): `[Sampler s0..15]`
/// - Param 2 (PS): `[CBV b0..13, SRV t0..127, UAV u0..7]`
/// - Param 3 (PS): `[Sampler s0..15]`
/// - Param 4 (GS): `[CBV b0..13, SRV t0..127]`
/// - Param 5 (GS): `[Sampler s0..15]`
pub fn build_graphics_root_signature() -> Arc<RootSignature> {
    let desc = RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![
                    range(DescriptorRangeType::Cbv, RS_CBV_COUNT),
                    range(DescriptorRangeType::Srv, RS_SRV_COUNT),
                ]),
                visibility: 1, // VERTEX
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Sampler, RS_SAMPLER_COUNT)]),
                visibility: 1, // VERTEX
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![
                    range(DescriptorRangeType::Cbv, RS_CBV_COUNT),
                    range(DescriptorRangeType::Srv, RS_SRV_COUNT),
                    range(DescriptorRangeType::Uav, RS_UAV_COUNT),
                ]),
                visibility: 5, // PIXEL
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Sampler, RS_SAMPLER_COUNT)]),
                visibility: 5, // PIXEL
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![
                    range(DescriptorRangeType::Cbv, RS_CBV_COUNT),
                    range(DescriptorRangeType::Srv, RS_SRV_COUNT),
                ]),
                visibility: 4, // GEOMETRY
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Sampler, RS_SAMPLER_COUNT)]),
                visibility: 4, // GEOMETRY
            },
        ],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    };
    RootSignature::new_deferred(desc)
}

/// Canonical Compute root signature:
/// - Param 0 (ALL): `[CBV b0..13, SRV t0..127, UAV u0..7]`
/// - Param 1 (ALL): `[Sampler s0..15]`
pub fn build_compute_root_signature() -> Arc<RootSignature> {
    let desc = RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![
                    range(DescriptorRangeType::Cbv, RS_CBV_COUNT),
                    range(DescriptorRangeType::Srv, RS_SRV_COUNT),
                    range(DescriptorRangeType::Uav, RS_UAV_COUNT),
                ]),
                visibility: 0, // ALL
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Sampler, RS_SAMPLER_COUNT)]),
                visibility: 0, // ALL
            },
        ],
        static_samplers: Vec::new(),
        flags: 0,
    };
    RootSignature::new_deferred(desc)
}

// ---------------------------------------------------------------------------
// Shaders: VertexShader, PixelShader, ComputeShader (7 slots each)
// ---------------------------------------------------------------------------

pub struct ShaderBlob {
    pub device: Arc<DeviceCore>,
    pub bytecode: Vec<u8>,
    pub hash: u128,
}

pub struct ComputeShaderInner {
    pub device: Arc<DeviceCore>,
    pub bytecode: Vec<u8>,
    pub hash: u128,
    pso: OnceLock<Result<Arc<ComputePipeline>, String>>,
}

impl ComputeShaderInner {
    pub fn new(device: Arc<DeviceCore>, bytecode: Vec<u8>) -> Self {
        let hash = dxil_metal::hash::hash128(&bytecode);
        let s = Self { device, bytecode, hash, pso: OnceLock::new() };
        // Eagerly compile when both Metal Shader Converter and (if SM4/5) dxilconv are ready.
        let is_sm4 = dxil_metal::dxbc::Container::parse(&s.bytecode)
            .map(|c| c.is_sm4_bytecode())
            .unwrap_or(false);
        if dxil_metal::converter_available() && (!is_sm4 || dxil_metal::dxilconv::available()) {
            let _ = s.pipeline();
        }
        s
    }

    pub fn pipeline(&self) -> Result<Arc<ComputePipeline>, String> {
        self.pso
            .get_or_init(|| {
                ComputePipeline::new(
                    self.device.compute_rs.clone(),
                    &self.bytecode,
                    &format!("d3d11_cs_{:016x}", self.hash as u64),
                )
                .map(Arc::new)
            })
            .clone()
    }
}

pub struct D3d11VertexShader {
    pub inner: Arc<ShaderBlob>,
}
pub struct D3d11GeometryShader {
    pub inner: Arc<ShaderBlob>,
}
pub struct D3d11PixelShader {
    pub inner: Arc<ShaderBlob>,
}
pub struct D3d11ComputeShader {
    pub inner: Arc<ComputeShaderInner>,
}

impl HasDeviceCore for D3d11VertexShader {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}
impl HasDeviceCore for D3d11GeometryShader {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}
impl HasDeviceCore for D3d11PixelShader {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}
impl HasDeviceCore for D3d11ComputeShader {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

static VS_CLASS: OnceLock<Class> = OnceLock::new();
static GS_CLASS: OnceLock<Class> = OnceLock::new();
static PS_CLASS: OnceLock<Class> = OnceLock::new();
static CS_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11VertexShader {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&VS_CLASS, || {
            let s = child_slots::<D3d11VertexShader>();
            debug_assert_eq!(s.len(), 7);
            Class {
                name: "ID3D11VertexShader",
                vtbl: build_vtable("ID3D11VertexShader", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11VertexShader],
            }
        })
    }
}

impl ComClass for D3d11GeometryShader {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&GS_CLASS, || {
            let s = child_slots::<D3d11GeometryShader>();
            debug_assert_eq!(s.len(), 7);
            Class {
                name: "ID3D11GeometryShader",
                vtbl: build_vtable("ID3D11GeometryShader", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11GeometryShader],
            }
        })
    }
}

impl ComClass for D3d11PixelShader {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&PS_CLASS, || {
            let s = child_slots::<D3d11PixelShader>();
            debug_assert_eq!(s.len(), 7);
            Class {
                name: "ID3D11PixelShader",
                vtbl: build_vtable("ID3D11PixelShader", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11PixelShader],
            }
        })
    }
}

impl ComClass for D3d11ComputeShader {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&CS_CLASS, || {
            let s = child_slots::<D3d11ComputeShader>();
            debug_assert_eq!(s.len(), 7);
            Class {
                name: "ID3D11ComputeShader",
                vtbl: build_vtable("ID3D11ComputeShader", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11ComputeShader],
            }
        })
    }
}

// ---------------------------------------------------------------------------
// InputLayout (7 slots)
// ---------------------------------------------------------------------------

pub struct InputLayoutInner {
    pub device: Arc<DeviceCore>,
    pub elements: Vec<InputElement>,
    pub hash: u128,
}

pub struct D3d11InputLayout {
    pub inner: Arc<InputLayoutInner>,
}

impl HasDeviceCore for D3d11InputLayout {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

static IL_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11InputLayout {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&IL_CLASS, || {
            let s = child_slots::<D3d11InputLayout>();
            debug_assert_eq!(s.len(), 7);
            Class {
                name: "ID3D11InputLayout",
                vtbl: build_vtable("ID3D11InputLayout", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11InputLayout],
            }
        })
    }
}

// ---------------------------------------------------------------------------
// BlendState (9 slots: ID3D11BlendState [0..7] + ID3D11BlendState1 [8])
// ---------------------------------------------------------------------------

pub struct BlendStateInner {
    pub device: Arc<DeviceCore>,
    pub desc0: D3D11_BLEND_DESC,
    pub desc1: D3D11_BLEND_DESC1,
    pub d3d12: d3d12_metal::state::BlendDesc,
}

pub struct D3d11BlendState {
    pub inner: Arc<BlendStateInner>,
}

impl HasDeviceCore for D3d11BlendState {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

unsafe extern "win64" fn blend_get_desc(this: *mut Obj<D3d11BlendState>, out: *mut D3D11_BLEND_DESC) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc0);
    }
}
unsafe extern "win64" fn blend_get_desc1(this: *mut Obj<D3d11BlendState>, out: *mut D3D11_BLEND_DESC1) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc1);
    }
}

static BLEND_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11BlendState {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&BLEND_CLASS, || {
            let mut s = child_slots::<D3d11BlendState>();
            s.push(("GetDesc", Some(blend_get_desc as *const ())));
            s.push(("GetDesc1", Some(blend_get_desc1 as *const ())));
            debug_assert_eq!(s.len(), 9);
            Class {
                name: "ID3D11BlendState1",
                vtbl: build_vtable("ID3D11BlendState1", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11BlendState, IID_ID3D11BlendState1],
            }
        })
    }
}

// ---------------------------------------------------------------------------
// RasterizerState (9 slots: ID3D11RasterizerState [0..7] + ID3D11RasterizerState1 [8])
// ---------------------------------------------------------------------------

pub struct RasterizerStateInner {
    pub device: Arc<DeviceCore>,
    pub desc0: D3D11_RASTERIZER_DESC,
    pub desc1: D3D11_RASTERIZER_DESC1,
    pub scissor_enable: bool,
    pub mtl: d3d12_metal::state::RasterState,
    pub hash: u64,
}

pub struct D3d11RasterizerState {
    pub inner: Arc<RasterizerStateInner>,
}

impl HasDeviceCore for D3d11RasterizerState {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

unsafe extern "win64" fn rast_get_desc(this: *mut Obj<D3d11RasterizerState>, out: *mut D3D11_RASTERIZER_DESC) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc0);
    }
}
unsafe extern "win64" fn rast_get_desc1(this: *mut Obj<D3d11RasterizerState>, out: *mut D3D11_RASTERIZER_DESC1) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc1);
    }
}

static RAST_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11RasterizerState {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&RAST_CLASS, || {
            let mut s = child_slots::<D3d11RasterizerState>();
            s.push(("GetDesc", Some(rast_get_desc as *const ())));
            s.push(("GetDesc1", Some(rast_get_desc1 as *const ())));
            debug_assert_eq!(s.len(), 9);
            Class {
                name: "ID3D11RasterizerState1",
                vtbl: build_vtable("ID3D11RasterizerState1", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11RasterizerState, IID_ID3D11RasterizerState1],
            }
        })
    }
}

// ---------------------------------------------------------------------------
// DepthStencilState (8 slots)
// ---------------------------------------------------------------------------

pub struct DepthStencilStateInner {
    pub device: Arc<DeviceCore>,
    pub desc: D3D11_DEPTH_STENCIL_DESC,
    pub mtl: d3d12_metal::state::DepthStencilState,
}

pub struct D3d11DepthStencilState {
    pub inner: Arc<DepthStencilStateInner>,
}

impl HasDeviceCore for D3d11DepthStencilState {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

unsafe extern "win64" fn ds_get_desc(this: *mut Obj<D3d11DepthStencilState>, out: *mut D3D11_DEPTH_STENCIL_DESC) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc);
    }
}

static DS_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11DepthStencilState {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&DS_CLASS, || {
            let mut s = child_slots::<D3d11DepthStencilState>();
            s.push(("GetDesc", Some(ds_get_desc as *const ())));
            debug_assert_eq!(s.len(), 8);
            Class {
                name: "ID3D11DepthStencilState",
                vtbl: build_vtable("ID3D11DepthStencilState", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11DepthStencilState],
            }
        })
    }
}

// ---------------------------------------------------------------------------
// SamplerState (8 slots)
// ---------------------------------------------------------------------------

pub struct SamplerStateInner {
    pub device: Arc<DeviceCore>,
    pub desc: D3D11_SAMPLER_DESC,
    pub entry: views::Entry,
}

pub struct D3d11SamplerState {
    pub inner: Arc<SamplerStateInner>,
}

impl HasDeviceCore for D3d11SamplerState {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

unsafe extern "win64" fn samp_get_desc(this: *mut Obj<D3d11SamplerState>, out: *mut D3D11_SAMPLER_DESC) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc);
    }
}

static SAMP_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11SamplerState {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&SAMP_CLASS, || {
            let mut s = child_slots::<D3d11SamplerState>();
            s.push(("GetDesc", Some(samp_get_desc as *const ())));
            debug_assert_eq!(s.len(), 8);
            Class {
                name: "ID3D11SamplerState",
                vtbl: build_vtable("ID3D11SamplerState", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11SamplerState],
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Query / Predicate (9 slots: 0..6 DeviceChild, 7 GetDataSize, 8 GetDesc)
// ---------------------------------------------------------------------------

pub struct QueryRuntime {
    pub ended: bool,
    pub submitted: bool,
    pub timeline_value: u64,
    pub heap: Option<Arc<QueryHeap>>,
    pub slot: u32,
    pub readback: Option<Arc<MtlResource>>,
}

pub struct QueryInner {
    pub device: Arc<DeviceCore>,
    pub desc: D3D11_QUERY_DESC,
    pub is_predicate: bool,
    pub rt: Mutex<QueryRuntime>,
}

impl Drop for QueryInner {
    fn drop(&mut self) {
        if self.desc.Query == query_kind::TIMESTAMP {
            if let Ok(mut rt) = self.rt.lock() {
                if let Some(heap) = rt.heap.take() {
                    self.device.free_timestamp_query_slot(&heap, rt.slot);
                }
            }
        }
    }
}

impl QueryInner {
    pub fn data_size(&self) -> u32 {
        match self.desc.Query {
            query_kind::EVENT | query_kind::OCCLUSION_PREDICATE | query_kind::SO_OVERFLOW_PREDICATE
            | query_kind::SO_OVERFLOW_PREDICATE_STREAM0..=query_kind::SO_OVERFLOW_PREDICATE_STREAM3 => 4,
            query_kind::OCCLUSION | query_kind::TIMESTAMP => 8,
            query_kind::TIMESTAMP_DISJOINT => std::mem::size_of::<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT>() as u32,
            query_kind::PIPELINE_STATISTICS => std::mem::size_of::<D3D11_QUERY_DATA_PIPELINE_STATISTICS>() as u32,
            query_kind::SO_STATISTICS | query_kind::SO_STATISTICS_STREAM0..=query_kind::SO_STATISTICS_STREAM3 => 16,
            _ => 8,
        }
    }
}

pub struct D3d11Query {
    pub inner: Arc<QueryInner>,
}

impl HasDeviceCore for D3d11Query {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

unsafe extern "win64" fn query_get_data_size(this: *mut Obj<D3d11Query>) -> u32 {
    Obj::get(this).inner.data_size()
}
unsafe extern "win64" fn query_get_desc(this: *mut Obj<D3d11Query>, out: *mut D3D11_QUERY_DESC) {
    if !out.is_null() {
        std::ptr::write_unaligned(out, Obj::get(this).inner.desc);
    }
}

static QUERY_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11Query {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&QUERY_CLASS, || {
            let mut s = child_slots::<D3d11Query>();
            s.push(("GetDataSize", Some(query_get_data_size as *const ())));
            s.push(("GetDesc", Some(query_get_desc as *const ())));
            debug_assert_eq!(s.len(), 9);
            Class {
                name: "ID3D11Query",
                vtbl: build_vtable("ID3D11Query", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11Asynchronous, IID_ID3D11Query, IID_ID3D11Predicate],
            }
        })
    }
}

// ---------------------------------------------------------------------------
// ID3D11CommandList (8 slots: 0..6 DeviceChild, 7 GetContextFlags)
// ---------------------------------------------------------------------------

pub struct CommandListInner {
    pub device: Arc<DeviceCore>,
    pub context_flags: u32,
    pub cmds: Vec<RecordedCmd>,
}

pub struct D3d11CommandList {
    pub inner: Arc<CommandListInner>,
}

impl HasDeviceCore for D3d11CommandList {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}

unsafe extern "win64" fn cl_get_context_flags(this: *mut Obj<D3d11CommandList>) -> u32 {
    Obj::get(this).inner.context_flags
}

static CL_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11CommandList {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&CL_CLASS, || {
            let mut s = child_slots::<D3d11CommandList>();
            s.push(("GetContextFlags", Some(cl_get_context_flags as *const ())));
            debug_assert_eq!(s.len(), 8);
            Class {
                name: "ID3D11CommandList",
                vtbl: build_vtable("ID3D11CommandList", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11CommandList],
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Graphics PSO cache key
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GraphicsPsoKey {
    pub vs_hash: u128,
    pub gs_hash: u128,
    pub ps_hash: u128,
    pub il_hash: u128,
    pub blend: d3d12_metal::state::BlendDesc,
    pub sample_mask: u32,
    pub raster_hash: u64,
    pub depth_stencil: d3d12_metal::state::DepthStencilDesc,
    pub topology_type: u32,
    pub strip_cut: u32,
    pub rtv_formats: [u32; 8],
    pub num_rtvs: u8,
    pub dsv_format: u32,
    pub sample_count: u32,
}

/// Hash a `D3D11_RASTERIZER_DESC1` deterministically for PSO cache keying.
pub fn hash_raster_desc(d: &D3D11_RASTERIZER_DESC1) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mix = |h: &mut u64, v: u32| {
        *h ^= v as u64;
        *h = h.wrapping_mul(0x0100_0000_01b3);
    };
    mix(&mut h, d.FillMode);
    mix(&mut h, d.CullMode);
    mix(&mut h, d.FrontCounterClockwise as u32);
    mix(&mut h, d.DepthBias as u32);
    mix(&mut h, d.DepthBiasClamp.to_bits());
    mix(&mut h, d.SlopeScaledDepthBias.to_bits());
    mix(&mut h, d.DepthClipEnable as u32);
    mix(&mut h, d.MultisampleEnable as u32);
    mix(&mut h, d.AntialiasedLineEnable as u32);
    mix(&mut h, d.ForcedSampleCount);
    h
}

/// Extract `Arc` helpers from COM pointers.
pub unsafe fn vs_from_ptr(p: *mut c_void) -> Option<Arc<ShaderBlob>> {
    Obj::<D3d11VertexShader>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn gs_from_ptr(p: *mut c_void) -> Option<Arc<ShaderBlob>> {
    Obj::<D3d11GeometryShader>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn ps_from_ptr(p: *mut c_void) -> Option<Arc<ShaderBlob>> {
    Obj::<D3d11PixelShader>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn cs_from_ptr(p: *mut c_void) -> Option<Arc<ComputeShaderInner>> {
    Obj::<D3d11ComputeShader>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn il_from_ptr(p: *mut c_void) -> Option<Arc<InputLayoutInner>> {
    Obj::<D3d11InputLayout>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn blend_from_ptr(p: *mut c_void) -> Option<Arc<BlendStateInner>> {
    Obj::<D3d11BlendState>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn rast_from_ptr(p: *mut c_void) -> Option<Arc<RasterizerStateInner>> {
    Obj::<D3d11RasterizerState>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn ds_from_ptr(p: *mut c_void) -> Option<Arc<DepthStencilStateInner>> {
    Obj::<D3d11DepthStencilState>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn samp_from_ptr(p: *mut c_void) -> Option<Arc<SamplerStateInner>> {
    Obj::<D3d11SamplerState>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn query_from_ptr(p: *mut c_void) -> Option<Arc<QueryInner>> {
    Obj::<D3d11Query>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn cmdlist_from_ptr(p: *mut c_void) -> Option<Arc<CommandListInner>> {
    Obj::<D3d11CommandList>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
