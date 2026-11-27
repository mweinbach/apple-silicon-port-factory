//! Direct3D 11 shader objects, fixed-function state objects, queries,
//! deferred command lists, and canonical D3D11->D3D12 root signatures.

use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::metal::pipeline::{msc_used_resources, ComputePipeline, GraphicsPipeline};
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
pub const HS_RES_TABLE_LEN: u32 = RS_CBV_COUNT + RS_SRV_COUNT; // 142
pub const DS_RES_TABLE_LEN: u32 = RS_CBV_COUNT + RS_SRV_COUNT; // 142
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
/// - Param 6 (HS): `[CBV b0..13, SRV t0..127]`
/// - Param 7 (HS): `[Sampler s0..15]`
/// - Param 8 (DS): `[CBV b0..13, SRV t0..127]`
/// - Param 9 (DS): `[Sampler s0..15]`
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
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![
                    range(DescriptorRangeType::Cbv, RS_CBV_COUNT),
                    range(DescriptorRangeType::Srv, RS_SRV_COUNT),
                ]),
                visibility: 2, // HULL
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Sampler, RS_SAMPLER_COUNT)]),
                visibility: 2, // HULL
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![
                    range(DescriptorRangeType::Cbv, RS_CBV_COUNT),
                    range(DescriptorRangeType::Srv, RS_SRV_COUNT),
                ]),
                visibility: 3, // DOMAIN
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![range(DescriptorRangeType::Sampler, RS_SAMPLER_COUNT)]),
                visibility: 3, // DOMAIN
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
// Per-stage reflected descriptor usage (sizes tables from MSC reflection)
// ---------------------------------------------------------------------------

/// Reflected descriptor slots accessed by one compiled shader stage within our
/// canonical D3D11 root signatures (`[CBV b0..13, SRV t0..127, (UAV u0..7)]` +
/// `[Sampler s0..15]`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StageUsage {
    pub cbv_mask: u16,
    pub srv_mask: u128,
    pub uav_mask: u8,
    pub samp_mask: u16,
    pub res_table_len: u32,
    pub samp_table_len: u32,
}

impl StageUsage {
    pub const EMPTY: Self = Self {
        cbv_mask: 0,
        srv_mask: 0,
        uav_mask: 0,
        samp_mask: 0,
        res_table_len: 0,
        samp_table_len: 0,
    };

    pub fn full(max_res_len: u32, has_uavs: bool) -> Self {
        Self {
            cbv_mask: (1u16 << RS_CBV_COUNT) - 1,
            srv_mask: u128::MAX,
            uav_mask: if has_uavs { u8::MAX } else { 0 },
            samp_mask: u16::MAX,
            res_table_len: max_res_len,
            samp_table_len: RS_SAMPLER_COUNT,
        }
    }

    pub fn from_compiled_shader(
        sh: &dxil_metal::CompiledShader,
        res_param: u32,
        samp_param: u32,
        max_res_len: u32,
        has_uavs: bool,
    ) -> Self {
        let Some(used) = msc_used_resources(sh.reflection.json.as_str()) else {
            return Self::full(max_res_len, has_uavs);
        };
        let mut out = Self::EMPTY;
        for u in used {
            if !u.is_in_grs {
                continue;
            }
            if u.binding_index == res_param {
                if u.table_length == 0 {
                    continue;
                }
                if u.table_start_index == u32::MAX || u.table_length == u32::MAX {
                    out.cbv_mask = (1u16 << RS_CBV_COUNT) - 1;
                    out.srv_mask = u128::MAX;
                    out.uav_mask = if has_uavs { u8::MAX } else { 0 };
                    out.res_table_len = max_res_len;
                    continue;
                }
                let end = u.table_start_index.saturating_add(u.table_length).min(max_res_len);
                out.res_table_len = out.res_table_len.max(end);
                for idx in u.table_start_index..end {
                    if idx < RS_CBV_COUNT {
                        out.cbv_mask |= 1u16 << idx;
                    } else if idx < RS_CBV_COUNT + RS_SRV_COUNT {
                        out.srv_mask |= 1u128 << (idx - RS_CBV_COUNT);
                    } else if has_uavs && idx < RS_CBV_COUNT + RS_SRV_COUNT + RS_UAV_COUNT {
                        out.uav_mask |= 1u8 << (idx - RS_CBV_COUNT - RS_SRV_COUNT);
                    }
                }
            } else if u.binding_index == samp_param {
                if u.table_length == 0 {
                    continue;
                }
                if u.table_start_index == u32::MAX || u.table_length == u32::MAX {
                    out.samp_mask = u16::MAX;
                    out.samp_table_len = RS_SAMPLER_COUNT;
                    continue;
                }
                let end = u.table_start_index.saturating_add(u.table_length).min(RS_SAMPLER_COUNT);
                out.samp_table_len = out.samp_table_len.max(end);
                for idx in u.table_start_index..end {
                    out.samp_mask |= 1u16 << idx;
                }
            }
        }
        out
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GfxPsoUsage {
    pub vs: StageUsage,
    pub ps: StageUsage,
    pub gs: StageUsage,
    pub hs: StageUsage,
    pub ds: StageUsage,
}

impl GfxPsoUsage {
    pub fn from_pipeline(pso: &GraphicsPipeline) -> Self {
        let vs = StageUsage::from_compiled_shader(&pso.vs, 0, 1, VS_RES_TABLE_LEN, false);
        let ps = match &pso.ps {
            Some(s) => StageUsage::from_compiled_shader(s, 2, 3, PS_RES_TABLE_LEN, true),
            None => StageUsage::EMPTY,
        };
        let gs_sh = pso
            .gs_emu
            .as_ref()
            .map(|g| &g.gs)
            .or_else(|| pso.tess_emu.as_ref().and_then(|t| t.gs.as_ref()));
        let gs = match gs_sh {
            Some(s) => StageUsage::from_compiled_shader(s, 4, 5, GS_RES_TABLE_LEN, false),
            None => StageUsage::EMPTY,
        };
        let (hs, ds) = match &pso.tess_emu {
            Some(t) => (
                StageUsage::from_compiled_shader(&t.hs, 6, 7, HS_RES_TABLE_LEN, false),
                StageUsage::from_compiled_shader(&t.ds, 8, 9, DS_RES_TABLE_LEN, false),
            ),
            None => (StageUsage::EMPTY, StageUsage::EMPTY),
        };
        Self { vs, ps, gs, hs, ds }
    }
}

pub struct CachedGfxPso {
    pub pso: Arc<GraphicsPipeline>,
    pub usage: GfxPsoUsage,
}

pub struct CachedComputePso {
    pub pso: Arc<ComputePipeline>,
    pub usage: StageUsage,
}

// ---------------------------------------------------------------------------
// Shaders: VertexShader, PixelShader, ComputeShader (7 slots each)
// ---------------------------------------------------------------------------

pub struct ShaderBlob {
    pub device: Arc<DeviceCore>,
    pub id: u32,
    pub bytecode: Vec<u8>,
    pub hash: u128,
}

pub struct ComputeShaderInner {
    pub device: Arc<DeviceCore>,
    pub bytecode: Vec<u8>,
    pub hash: u128,
    pso: OnceLock<Result<Arc<CachedComputePso>, String>>,
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

    pub fn pipeline(&self) -> Result<Arc<CachedComputePso>, String> {
        self.pso
            .get_or_init(|| {
                let pso = Arc::new(ComputePipeline::new(
                    self.device.compute_rs.clone(),
                    &self.bytecode,
                    &format!("d3d11_cs_{:016x}", self.hash as u64),
                )?);
                let usage = StageUsage::from_compiled_shader(&pso.shader, 0, 1, CS_RES_TABLE_LEN, true);
                Ok(Arc::new(CachedComputePso { pso, usage }))
            })
            .clone()
    }
}

pub struct D3d11VertexShader {
    pub inner: Arc<ShaderBlob>,
}
pub struct D3d11HullShader {
    pub inner: Arc<ShaderBlob>,
}
pub struct D3d11DomainShader {
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
impl HasDeviceCore for D3d11HullShader {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.inner.device }
}
impl HasDeviceCore for D3d11DomainShader {
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
static HS_CLASS: OnceLock<Class> = OnceLock::new();
static DOMAIN_SHADER_CLASS: OnceLock<Class> = OnceLock::new();
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

impl ComClass for D3d11HullShader {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&HS_CLASS, || {
            let s = child_slots::<D3d11HullShader>();
            debug_assert_eq!(s.len(), 7);
            Class {
                name: "ID3D11HullShader",
                vtbl: build_vtable("ID3D11HullShader", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11HullShader],
            }
        })
    }
}

impl ComClass for D3d11DomainShader {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&DOMAIN_SHADER_CLASS, || {
            let s = child_slots::<D3d11DomainShader>();
            debug_assert_eq!(s.len(), 7);
            Class {
                name: "ID3D11DomainShader",
                vtbl: build_vtable("ID3D11DomainShader", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11DomainShader],
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
    pub id: u32,
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
    pub id: u32,
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
    /// Canonical non-scissor rasterizer state ID (`ScissorEnable` is dynamic via `set_scissors`).
    pub id: u32,
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
    /// Canonical depth-stencil state ID (`StencilRef` is dynamic via `set_stencil_ref`).
    pub id: u32,
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
    pub pipeline_stats: D3D11_QUERY_DATA_PIPELINE_STATISTICS,
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

/// Compact integer-ID key for D3D11's `device.gfx_psos` cache.
///
/// Replaces the previous 544-byte key (which copied and hashed full 328-byte `BlendDesc`
/// and 52-byte `DepthStencilDesc` structs plus six `u128` hashes per draw) with canonical
/// per-device integer IDs:
/// - `ScissorEnable`, `BlendFactor`, and `StencilRef` are dynamic encoder state and excluded.
/// - `ds_id` is forced to `0` whenever `dsv_format == 0` (Metal ignores DS state with no DSV).
/// - `raster_id` and `ds_id` (when `dsv_format != 0`) remain in the key because
///   `d3d12_metal::CommandList::apply_dynamic` applies `p.raster` / `p.depth_stencil` from
///   `GraphicsPipeline`, while `d3d12-metal`'s underlying `PipelineCache` shares the compiled
///   `MTL4RenderPipelineState` across raster/DS variants.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GraphicsPsoKey {
    pub vs_id: u32,
    pub hs_id: u32,
    pub ds_id: u32,
    pub gs_id: u32,
    pub ps_id: u32,
    pub il_id: u32,
    pub blend_id: u32,
    pub sample_mask: u32,
    pub raster_id: u32,
    pub ds_state_id: u32,
    pub rtv_formats: [u16; 8],
    pub dsv_format: u16,
    pub topology_type: u8,
    pub strip_cut: u8,
    pub num_rtvs: u8,
    pub sample_count: u8,
}

/// Exact non-scissor key for interning `ID3D11RasterizerState` objects into canonical IDs.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RasterKey {
    pub fill_mode: u32,
    pub cull_mode: u32,
    pub front_ccw: i32,
    pub depth_bias: i32,
    pub depth_bias_clamp_bits: u32,
    pub slope_scaled_depth_bias_bits: u32,
    pub depth_clip_enable: i32,
    pub multisample_enable: i32,
    pub antialiased_line_enable: i32,
    pub forced_sample_count: u32,
}

impl RasterKey {
    pub fn from_desc1(d: &D3D11_RASTERIZER_DESC1) -> Self {
        Self {
            fill_mode: d.FillMode,
            cull_mode: d.CullMode,
            front_ccw: d.FrontCounterClockwise,
            depth_bias: d.DepthBias,
            depth_bias_clamp_bits: d.DepthBiasClamp.to_bits(),
            slope_scaled_depth_bias_bits: d.SlopeScaledDepthBias.to_bits(),
            depth_clip_enable: d.DepthClipEnable,
            multisample_enable: d.MultisampleEnable,
            antialiased_line_enable: d.AntialiasedLineEnable,
            forced_sample_count: d.ForcedSampleCount,
        }
    }
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
pub unsafe fn hs_from_ptr(p: *mut c_void) -> Option<Arc<ShaderBlob>> {
    Obj::<D3d11HullShader>::from_iface(p).map(|o| Obj::get(o).inner.clone())
}
pub unsafe fn domain_shader_from_ptr(p: *mut c_void) -> Option<Arc<ShaderBlob>> {
    Obj::<D3d11DomainShader>::from_iface(p).map(|o| Obj::get(o).inner.clone())
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

// ---------------------------------------------------------------------------
// ID3DDeviceContextState (7 slots: ID3D11DeviceChild only)
// ---------------------------------------------------------------------------

pub struct D3d11DeviceContextState {
    pub device: Arc<DeviceCore>,
    pub feature_level: u32,
    pub state: Mutex<super::context::ContextState>,
}

impl HasDeviceCore for D3d11DeviceContextState {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.device }
}

static CTX_STATE_CLASS: OnceLock<Class> = OnceLock::new();
impl ComClass for D3d11DeviceContextState {
    const TRACE_RELEASE: bool = false;
    fn class() -> &'static Class {
        class_once(&CTX_STATE_CLASS, || {
            let s = child_slots::<D3d11DeviceContextState>();
            debug_assert_eq!(s.len(), 7);
            Class {
                name: "ID3DDeviceContextState",
                vtbl: build_vtable("ID3DDeviceContextState", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3DDeviceContextState],
            }
        })
    }
}

impl D3d11DeviceContextState {
    pub fn new_com(
        device: Arc<DeviceCore>,
        feature_level: u32,
        initial: super::context::ContextState,
    ) -> *mut Obj<Self> {
        Obj::new(Self { device, feature_level, state: Mutex::new(initial) })
    }
}
