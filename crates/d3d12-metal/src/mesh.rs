//! D3D12 mesh-shader pipelines (amplification + mesh + pixel) on Metal 4 mesh render pipelines.
//!
//! * The Metal Shader Converter turns an amplification shader (`as_6_5`) into a Metal **object**
//!   function, and a mesh shader (`ms_6_5`) into a **mesh** function. The payload size and the
//!   threadgroup sizes come from their reflection.
//! * `ID3D12GraphicsCommandList6::DispatchMesh(x, y, z)` maps to
//!   `drawMeshThreadgroups(x,y,z, AS numthreads, MS numthreads)`. Without an amplification stage,
//!   the grid launches mesh threadgroups directly.
//! * Bindings are the same as for every other stage (heaps at 0/1, top-level argument buffer
//!   at 2). The argument table must be set for `STAGES` (object | mesh | fragment).
//!
//! The device reports `MeshShaderTier` 1 by default ([`enabled`]; set
//! `PORT_MESH_SHADERS=0` or `AW2_MESH_SHADERS=0` to disable).
//!
//! Limits that differ from D3D12:
//!
//! * An amplification shader may launch at most [`MAX_MESH_GRID_THREADGROUPS`] mesh
//!   threadgroups per object threadgroup. D3D12 allows 2^22, Metal 1024; larger grids are
//!   clamped by Metal.
//! * The payload is capped at 16 KB on both APIs.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use dxil_metal::root_signature::RootSignature;
use dxil_metal::{CompileOptions, CompiledShader, ShaderStage, StageInfo};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::*;

use crate::metal::indirect::{KernelPatch, MAX_PATCHES};
use crate::metal::indirect_icb::{MtlComputePso, MtlIcb};
use crate::metal::pipeline::{compile, depth_stencil_state, library, pso_cache, shader_function_descriptor, MtlDepthStencil, MtlRenderPipeline};
use crate::state::{BlendDesc, BlendState, DepthStencilState, RasterState};

/// `PORT_MESH_SHADERS` (legacy alias `AW2_MESH_SHADERS`, default **on**; `=0` disables):
/// report `D3D12_MESH_SHADER_TIER_1` and accept AS/MS pipelines on Metal 4.
pub fn enabled() -> bool {
    crate::metal::env_canonical_or_alias("PORT_MESH_SHADERS", "AW2_MESH_SHADERS")
        .is_none_or(|v| crate::metal::parse_bool_knob(&v))
}

/// Render stages that read the argument table in a mesh pass.
pub const STAGES: MTLRenderStages = MTLRenderStages::Object.union(MTLRenderStages::Mesh).union(MTLRenderStages::Fragment);

/// D3D12: each `DispatchMesh` dimension is at most 65535...
pub const MAX_DISPATCH_DIM: u32 = 65535;
/// ...and the product at most 2^22.
pub const MAX_DISPATCH_GROUPS: u64 = 1 << 22;
/// Metal's limit on mesh threadgroups launched by one object threadgroup.
pub const MAX_MESH_GRID_THREADGROUPS: u32 = 1024;
/// Payload limit on both APIs.
pub const MAX_PAYLOAD_BYTES: u32 = 16384;

/// Pipeline-stream subobjects of a mesh PSO (`D3D12_PIPELINE_STATE_STREAM_DESC` with AS/MS).
pub struct MeshDesc<'a> {
    /// Amplification shader (`D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_AS`, 24), optional.
    pub amplification: Option<&'a [u8]>,
    /// Mesh shader (`..._MS`, 25).
    pub mesh: &'a [u8],
    pub pixel: Option<&'a [u8]>,
    pub blend: BlendDesc,
    pub sample_mask: u32,
    pub raster: RasterState,
    pub depth_stencil: DepthStencilState,
    /// `NumRenderTargets` entries.
    pub rtv_formats: Vec<u32>,
    pub dsv_format: u32,
    pub sample_count: u32,
}

/// Distinct `PipelineCache` archive key salt for the ICB-capable mesh PSO variant
/// (`MTL4IndirectCommandBufferSupportState::Enabled`).
const MESH_ICB_PSO_KEY_SALT: u128 = 0x9b5a_2d41_e7c8_304f_61c2_88b0_4f19_a73d;

struct SendMeshIcbDesc(Retained<MTL4MeshRenderPipelineDescriptor>);
unsafe impl Send for SendMeshIcbDesc {}
unsafe impl Sync for SendMeshIcbDesc {}

pub struct MeshPipeline {
    pub pso: Retained<MtlRenderPipeline>,
    /// Retained descriptor + archive key for lazy ICB-capable compilation ([`Self::icb_pso`]).
    icb_desc: Option<(SendMeshIcbDesc, u128)>,
    icb_pso: OnceLock<Result<Retained<MtlRenderPipeline>, String>>,
    pub rs: Arc<RootSignature>,
    pub raster: RasterState,
    pub blend: BlendState,
    pub depth_stencil: Retained<MtlDepthStencil>,
    pub depth_bounds: bool,
    pub rtv_formats: Vec<u32>,
    pub dsv_format: u32,
    pub sample_count: u32,
    /// Amplification `numthreads`, if there is an amplification stage.
    pub object_threads: Option<[u32; 3]>,
    /// Mesh `numthreads`.
    pub mesh_threads: [u32; 3],
    /// `payloadMemoryLength` (0 without an amplification stage).
    pub payload_bytes: u32,
    pub amplification: Option<Arc<CompiledShader>>,
    pub mesh: Arc<CompiledShader>,
    pub pixel: Option<Arc<CompiledShader>>,
    /// Resource classes the object or mesh stage may read through SRVs/UAVs
    /// ([`crate::metal::pipeline::stage_reads_resources`]).
    pub pre_raster_reads: u8,
    /// Whether the object, mesh, or pixel stage may write through UAVs.
    pub writes_uav: bool,
    /// Zero-salt MSC + AIR reflection exact stage flags (`AW2_SM66_GFX_STAGE_EXACT=1`).
    pub sm66_pre_raster_reads: u8,
    pub sm66_ms_as_reads_cbv: bool,
    pub sm66_writes_uav: bool,
    pub sm66_narrowed: bool,
    /// Whether the object or mesh stage alone may write through UAVs (`AW2_LIST_START_RENDER_SPLIT=1`).
    pub ms_as_writes_uav: bool,
    /// Whether the pixel stage alone may write through UAVs (`AW2_RP_KEEP_WRITE_EXACT=1`).
    pub ps_writes_uav: bool,
    /// Resource classes (`READS_TEX | READS_BUF`, including CBVs) the pixel stage may read
    /// (`0` for depth-only `pixel == None` passes; `AW2_RP_KEEP_WRITE_EXACT=1`).
    pub ps_read_classes: u8,
    /// SRV/UAV-only resource classes (`READS_TEX | READS_BUF`, excluding CBVs) the pixel stage
    /// may read (`AW2_PS_SRV_CLASS_EXACT=1`).
    pub ps_srv_uav_classes: u8,
    /// Whether object, mesh, or pixel stage may read or write any UAV (`AW2_RP_KEEP_WRITE_EXACT=1`).
    pub uses_uav: bool,
    /// Zero-salt per-stage binding profiles (`AW2_GFX_DESC_TABLE_EXACT=1`).
    pub as_access: Option<crate::metal::pipeline::ComputeAccess>,
    pub ms_access: crate::metal::pipeline::ComputeAccess,
    pub ps_access: Option<crate::metal::pipeline::ComputeAccess>,
    /// Embedded handle-kind profiles (eliminating `HK_PROFILE_REGISTRY.read()` on draw).
    pub as_hk_profile: Option<crate::metal::pipeline::HandleKindProfile>,
    pub ms_hk_profile: crate::metal::pipeline::HandleKindProfile,
    pub ps_hk_profile: Option<crate::metal::pipeline::HandleKindProfile>,
}
unsafe impl Send for MeshPipeline {}
unsafe impl Sync for MeshPipeline {}

fn product(t: [u32; 3]) -> u32 {
    t[0].saturating_mul(t[1]).saturating_mul(t[2])
}

fn mtl_size(t: [u32; 3]) -> MTLSize {
    MTLSize { width: t[0] as usize, height: t[1] as usize, depth: t[2] as usize }
}

impl MeshPipeline {
    pub fn new(rs: Arc<RootSignature>, d: &MeshDesc, label: &str) -> Result<MeshPipeline, String> {
        Self::new_with_all_opts(
            rs,
            d,
            label,
            crate::metal::pipeline::shader_immutable_opt(),
            crate::metal::pipeline::shader_pso_opt_v2(),
        )
    }

    pub fn new_with_opts(
        rs: Arc<RootSignature>,
        d: &MeshDesc,
        label: &str,
        immut_opt: bool,
    ) -> Result<MeshPipeline, String> {
        Self::new_with_all_opts(rs, d, label, immut_opt, crate::metal::pipeline::shader_pso_opt_v2())
    }

    pub fn new_with_all_opts(
        rs: Arc<RootSignature>,
        d: &MeshDesc,
        label: &str,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<MeshPipeline, String> {
        let rs_for_manifest = rs.clone();
        let out = Self::new_in_cache(pso_cache()?, rs, d, label, immut_opt, pso_opt_v2)?;
        crate::metal::pso_manifest::note_mesh(&rs_for_manifest, d, immut_opt, pso_opt_v2);
        Ok(out)
    }

    pub(crate) fn new_in_cache(
        cache: &dxil_metal::pso_cache::PipelineCache,
        rs: Arc<RootSignature>,
        d: &MeshDesc,
        label: &str,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<MeshPipeline, String> {
        let blend = crate::state::blend_state(&d.blend);
        if !blend.unsupported_logic_ops.is_empty() {
            log::warn!(target: "d3d", "mesh PSO: unsupported logic ops {:?} (blending off)", blend.unsupported_logic_ops.as_slice());
        }
        let amplification = d.amplification.map(|b| compile(ShaderStage::Amplification, b, &rs, &CompileOptions::default())).transpose()?;
        let mesh = compile(ShaderStage::Mesh, d.mesh, &rs, &CompileOptions::default())?;
        let mut ps_opts = crate::state::pixel_compile_options(&blend, &d.rtv_formats);
        let full = if d.sample_count >= 32 { u32::MAX } else { (1u32 << d.sample_count.max(1)) - 1 };
        if d.sample_mask & full != full {
            ps_opts.sample_mask = Some(d.sample_mask);
        }
        let pixel = d.pixel.map(|b| compile(ShaderStage::Fragment, b, &rs, &ps_opts)).transpose()?;

        let (mesh_threads, mesh_payload) = match mesh.reflection.info {
            StageInfo::Mesh { threadgroup_size, max_payload_size_in_bytes, .. } => (threadgroup_size, max_payload_size_in_bytes),
            ref other => return Err(format!("mesh shader without mesh reflection: {other:?}")),
        };
        let (object_threads, payload_bytes) = match amplification.as_ref().map(|a| &a.reflection.info) {
            None => (None, 0),
            Some(StageInfo::Amplification { threadgroup_size, max_payload_size_in_bytes }) => {
                (Some(*threadgroup_size), (*max_payload_size_in_bytes).max(mesh_payload))
            }
            Some(other) => return Err(format!("amplification shader without amplification reflection: {other:?}")),
        };
        if payload_bytes > MAX_PAYLOAD_BYTES {
            return Err(format!("payload of {payload_bytes} bytes exceeds {MAX_PAYLOAD_BYTES}"));
        }
        if product(mesh_threads) == 0 || object_threads.is_some_and(|t| product(t) == 0) {
            return Err(format!("empty threadgroup (AS {object_threads:?}, MS {mesh_threads:?})"));
        }

        let obj_fn = match &amplification {
            Some(a) => Some(shader_function_descriptor(&*library(a)?, a)),
            None => None,
        };
        let mesh_fn = shader_function_descriptor(&*library(&mesh)?, &mesh);
        let frag_fn = match &pixel {
            Some(p) => Some(shader_function_descriptor(&*library(p)?, p)),
            None => None,
        };
        let build_desc = |icb_support: bool| -> Retained<MTL4MeshRenderPipelineDescriptor> {
            let desc = MTL4MeshRenderPipelineDescriptor::new();
            if let Some(f) = &obj_fn {
                desc.setObjectFunctionDescriptor(Some(f));
            }
            desc.setMeshFunctionDescriptor(Some(&mesh_fn));
            if let Some(f) = &frag_fn {
                desc.setFragmentFunctionDescriptor(Some(f));
            }
            if !label.is_empty() {
                desc.setLabel(Some(&NSString::from_str(label)));
            }
            if let Some(t) = object_threads {
                let obj_total = product(t) as usize;
                desc.setMaxTotalThreadsPerObjectThreadgroup(obj_total);
                desc.setRequiredThreadsPerObjectThreadgroup(mtl_size(t));
                if immut_opt {
                    desc.setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth(obj_total.is_multiple_of(32));
                }
                // Metal wants a multiple of 16 bytes.
                desc.setPayloadMemoryLength(payload_bytes.next_multiple_of(16) as usize);
                desc.setMaxTotalThreadgroupsPerMeshGrid(MAX_MESH_GRID_THREADGROUPS as usize);
            }
            let mesh_total = product(mesh_threads) as usize;
            desc.setMaxTotalThreadsPerMeshThreadgroup(mesh_total);
            desc.setRequiredThreadsPerMeshThreadgroup(mtl_size(mesh_threads));
            if immut_opt {
                desc.setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth(mesh_total.is_multiple_of(32));
                crate::metal::pipeline::apply_immutable_pipeline_options(&desc);
                desc.setSupportObjectBinaryLinking(false);
                desc.setSupportMeshBinaryLinking(false);
                desc.setSupportFragmentBinaryLinking(false);
                desc.setColorAttachmentMappingState(MTL4LogicalToPhysicalColorAttachmentMappingState::Identity);
            }
            if icb_support {
                desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Enabled);
            } else if immut_opt {
                desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Disabled);
            }
            if pso_opt_v2 {
                unsafe { desc.setMaxVertexAmplificationCount(1) };
                desc.setAlphaToOneState(MTL4AlphaToOneState::Disabled);
            }
            let ca = desc.colorAttachments();
            for (i, f) in d.rtv_formats.iter().enumerate().take(8) {
                let a = unsafe { ca.objectAtIndexedSubscript(i) };
                let pf = crate::format::info(*f).mtl;
                a.setPixelFormat(pf);
                if pf != MTLPixelFormat::Invalid {
                    blend.attachments[i].apply_to_with_opt(&a, pso_opt_v2);
                }
            }
            unsafe { desc.setRasterSampleCount(d.sample_count.max(1) as usize) };
            desc.setAlphaToCoverageState(if blend.alpha_to_coverage {
                MTL4AlphaToCoverageState::Enabled
            } else {
                MTL4AlphaToCoverageState::Disabled
            });
            desc
        };

        let mut key = dxil_metal::hash::KeyHasher::new("d3d12-metal mesh pso v2");
        if let Some(a) = &amplification {
            key.u128(a.key);
        }
        key.u128(mesh.key);
        if let Some(p) = &pixel {
            key.u128(p.key);
        }
        crate::metal::pipeline::RenderPsoStateKey::new(blend, &d.rtv_formats, d.sample_count, payload_bytes)
            .write_to(&mut key);
        let mut pso_key = key.finish();
        if immut_opt {
            pso_key ^= crate::metal::pipeline::IMMUTABLE_OPT_KEY_SALT;
        }
        if pso_opt_v2 {
            pso_key ^= crate::metal::pipeline::SHADER_PSO_OPT_V2_SALT;
        }

        let desc = build_desc(false);
        let icb_desc = Some((SendMeshIcbDesc(build_desc(true)), pso_key ^ MESH_ICB_PSO_KEY_SALT));
        let pso = cache.render_pipeline(Some(pso_key), &desc).map_err(|e| format!("mesh render pipeline: {e}"))?;
        let ms_sum = crate::metal::pipeline::ShaderStageSummary::new(d.mesh, &mesh, &rs);
        let as_sum = amplification
            .as_ref()
            .zip(d.amplification)
            .map(|(a, b)| (a, b, crate::metal::pipeline::ShaderStageSummary::new(b, a, &rs)));
        let ps_sum = pixel
            .as_ref()
            .zip(d.pixel)
            .map(|(p, b)| (p, b, crate::metal::pipeline::ShaderStageSummary::new(b, p, &rs)));
        let msc_usage = crate::metal::pipeline::msc_stage_usage();
        let sm66_gfx = crate::metal::pipeline::sm66_gfx_stage_exact();
        let base_use_msc = pso_opt_v2 || msc_usage;
        let dxil_pre_raster = ms_sum.stage_reads_resources(&mesh, false)
            | as_sum.as_ref().map_or(0, |(a, _, s)| s.stage_reads_resources(a, false));
        let base_pre_raster = if base_use_msc {
            ms_sum.stage_reads_resources(&mesh, true)
                | as_sum.as_ref().map_or(0, |(a, _, s)| s.stage_reads_resources(a, true))
        } else {
            dxil_pre_raster
        };
        let sm66_pre_raster_reads = ms_sum.stage_srv_uav_classes(&mesh, &rs, true)
            | as_sum.as_ref().map_or(0, |(a, _, s)| s.stage_srv_uav_classes(a, &rs, true));
        let dxil_ms_as_cbv = ms_sum.stage_may_read_cbv(&rs, false)
            || as_sum.as_ref().is_some_and(|(_, _, s)| s.stage_may_read_cbv(&rs, false));
        let sm66_ms_as_reads_cbv = ms_sum.stage_may_read_cbv(&rs, true)
            || as_sum.as_ref().is_some_and(|(_, _, s)| s.stage_may_read_cbv(&rs, true));
        let ms_as_writes_uav = ms_sum.shader_writes_uav(true)
            || as_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_uav(true));
        let ps_writes_uav = ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_uav(true));
        if ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_buf_uav()) {
            crate::metal::pipeline::note_frag_buf_write();
        }
        let sm66_writes_uav = ms_as_writes_uav || ps_writes_uav;
        let dxil_writes_uav = ms_sum.shader_writes_uav(false)
            || as_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_uav(false))
            || ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_uav(false));
        let base_writes_uav = if base_use_msc { sm66_writes_uav } else { dxil_writes_uav };
        let pre_raster_reads = if sm66_gfx { sm66_pre_raster_reads } else { base_pre_raster };
        let writes_uav = if sm66_gfx { sm66_writes_uav } else { base_writes_uav };
        let (ps_srv_uav_classes, ps_read_classes) = match &ps_sum {
            Some((p, _, s)) => {
                let srv_uav = s.stage_srv_uav_classes(p, &rs, true);
                let mut m = srv_uav;
                if s.stage_may_read_cbv(&rs, true) {
                    m |= crate::metal::pipeline::READS_BUF;
                }
                (srv_uav, m)
            }
            None => (0, 0),
        };
        let uses_uav = writes_uav
            || ms_sum.shader_uses_uav(&rs)
            || as_sum.as_ref().is_some_and(|(_, _, s)| s.shader_uses_uav(&rs))
            || ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_uses_uav(&rs));
        let both_classes = crate::metal::pipeline::READS_TEX | crate::metal::pipeline::READS_BUF;
        let msc_proven = ms_sum.msc_proven()
            && as_sum.as_ref().is_none_or(|(_, _, s)| s.msc_proven())
            && ps_sum.as_ref().is_none_or(|(_, _, s)| s.msc_proven());
        let sm66_narrowed = sm66_pre_raster_reads != dxil_pre_raster
            || sm66_ms_as_reads_cbv != dxil_ms_as_cbv
            || sm66_writes_uav != dxil_writes_uav
            || (msc_proven
                && (sm66_pre_raster_reads != both_classes
                    || !sm66_writes_uav
                    || ps_read_classes != both_classes
                    || !uses_uav));
        if msc_usage
            && !pso_opt_v2
            && (pre_raster_reads != both_classes || !writes_uav)
        {
            crate::metal::encstats::inc(&crate::metal::encstats::c::msc_stage_usage_hit);
        }
        let (as_access, as_hk_profile) = match &as_sum {
            Some((a, b, s)) => {
                let (acc, hk) = s.for_stage(b, &rs, a, false);
                (Some(acc), Some(hk))
            }
            None => (None, None),
        };
        let (ms_access, ms_hk_profile) = ms_sum.for_stage(d.mesh, &rs, &mesh, false);
        let (ps_access, ps_hk_profile) = match &ps_sum {
            Some((p, b, s)) => {
                let (acc, hk) = s.for_stage(b, &rs, p, false);
                (Some(acc), Some(hk))
            }
            None => (None, None),
        };
        let ds = if d.dsv_format == 0 { DepthStencilState::without_depth_stencil() } else { d.depth_stencil };
        Ok(MeshPipeline {
            pre_raster_reads,
            writes_uav,
            sm66_pre_raster_reads,
            sm66_ms_as_reads_cbv,
            sm66_writes_uav,
            sm66_narrowed,
            ms_as_writes_uav,
            ps_writes_uav,
            ps_read_classes,
            ps_srv_uav_classes,
            uses_uav,
            as_access,
            ms_access,
            ps_access,
            as_hk_profile,
            ms_hk_profile,
            ps_hk_profile,
            pso,
            icb_desc,
            icb_pso: OnceLock::new(),
            rs,
            raster: d.raster,
            blend,
            depth_stencil: depth_stencil_state(&ds)?,
            depth_bounds: ds.depth_bounds,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count.max(1),
            object_threads,
            mesh_threads,
            payload_bytes,
            amplification,
            mesh,
            pixel,
        })
    }

    /// Lazily compile and return the `MTL4IndirectCommandBufferSupportState::Enabled`
    /// variant of this mesh pipeline for GPU-encoded `ExecuteIndirect(DispatchMesh)` ICBs.
    pub fn icb_pso(&self) -> Result<&MtlRenderPipeline, String> {
        let res = self.icb_pso.get_or_init(|| {
            let (desc, key) = self
                .icb_desc
                .as_ref()
                .ok_or_else(|| "mesh pipeline missing ICB descriptor".to_string())?;
            pso_cache()?
                .render_pipeline(Some(*key), &desc.0)
                .map_err(|e| format!("mesh ICB render pipeline: {e}"))
        });
        res.as_deref().map_err(|e| e.clone())
    }
}

// ---------------------------------------------------------------------------
// GPU-encoded Mesh ExecuteIndirect ICB (`MTLIndirectCommandTypeDrawMeshThreadgroups`)
// ---------------------------------------------------------------------------

/// Parameters uploaded for the GPU mesh ICB expander (`aw2_ei_mesh_icb_vs` / `aw2_ei_mesh_icb`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MeshIcbParams {
    /// `MTLResourceID` of the mesh `MTLIndirectCommandBuffer`.
    pub icb: u64,
    pub args: u64,
    pub count: u64,
    pub tlab_out: u64,
    pub tlab_in: u64,
    pub pred: u64,
    /// `MTLIndirectCommandBufferExecutionRange` (`{ location, length }`) written by thread 0.
    pub range_out: u64,
    pub heap: u64,
    pub sampler_heap: u64,
    pub tlab_static: u64,
    pub stride: u32,
    pub max_count: u32,
    pub work_offset: u32,
    pub location: u32,
    pub tlab_size: u32,
    pub tlab_stride: u32,
    pub npatch: u32,
    pub pred_op: u32,
    pub obj_threads: [u32; 3],
    pub mesh_threads: [u32; 3],
    pub has_object: u32,
    pub has_ps: u32,
    pub patches: [KernelPatch; MAX_PATCHES],
}
const _: () = assert!(std::mem::size_of::<MeshIcbParams>() == 10 * 8 + 16 * 4 + MAX_PATCHES * 16);

impl Default for MeshIcbParams {
    fn default() -> Self {
        // SAFETY: plain-old-data.
        unsafe { std::mem::zeroed() }
    }
}

impl MeshIcbParams {
    pub fn bytes(&self) -> &[u8] {
        // SAFETY: `MeshIcbParams` is `#[repr(C)]` with no padding bytes.
        unsafe {
            std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<Self>())
        }
    }
}

const MESH_ICB_SRC: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct Patch { uint src; uint dst; uint dwords; uint kind; };
struct MeshIcbParams {
    command_buffer icb;
    ulong args, count, tlab_out, tlab_in, pred, range_out;
    ulong heap, sampler_heap, tlab_static;
    uint stride, max_count, work_offset, location, tlab_size, tlab_stride, npatch, pred_op;
    uint obj_x, obj_y, obj_z;
    uint mesh_x, mesh_y, mesh_z;
    uint has_object, has_ps;
    Patch patches[16];
};

template <typename T> static inline device T* ptr(ulong va) { return reinterpret_cast<device T*>(va); }
static inline uint rd(device const uchar* p, uint off) { return *(device const uint*)(p + off); }

static inline void bind_mesh_buf(thread render_command& rc, ulong va, uint index, uint has_object, uint has_ps) {
    if (va != 0ul) {
        if (has_object != 0u) rc.set_object_buffer(ptr<uchar>(va), index);
        rc.set_mesh_buffer(ptr<uchar>(va), index);
        if (has_ps != 0u) rc.set_fragment_buffer(ptr<uchar>(va), index);
    }
}

static void encode_mesh(constant MeshIcbParams& p, uint i) {
    uint n = p.count != 0ul ? min(ptr<uint>(p.count)[0], p.max_count) : p.max_count;
    if (p.pred != 0ul) {
        ulong pv = ptr<ulong>(p.pred)[0];
        if ((p.pred_op == 0u && pv == 0ul) || (p.pred_op != 0u && pv != 0ul)) n = 0u;
    }
    if (i == 0u) {
        device uint* r = ptr<uint>(p.range_out);
        r[0] = p.location;
        r[1] = n;
    }
    if (i >= n) return;
    device const uchar* cmd = ptr<uchar>(p.args) + (ulong)i * p.stride;
    uint gx = rd(cmd, p.work_offset);
    uint gy = rd(cmd, p.work_offset + 4u);
    uint gz = rd(cmd, p.work_offset + 8u);

    // Recycled StorageModePrivate ICB slots retain prior commands, so empty grids must reset.
    render_command rc(p.icb, p.location + i);
    if (gx == 0u || gy == 0u || gz == 0u) {
        rc.reset();
        return;
    }

    ulong tl = p.tlab_static;
    if (p.tlab_out != 0ul) {
        device uchar* t = ptr<uchar>(p.tlab_out) + (ulong)i * p.tlab_stride;
        device const uchar* tin = ptr<uchar>(p.tlab_in);
        uint b16 = p.tlab_size & ~15u;
        for (uint b = 0u; b < b16; b += 16u) *(device uint4*)(t + b) = *(device const uint4*)(tin + b);
        for (uint b = b16; b < p.tlab_size; b += 4u) *(device uint*)(t + b) = *(device const uint*)(tin + b);
        for (uint k = 0u; k < p.npatch; ++k) {
            Patch q = p.patches[k];
            for (uint w = 0u; w < q.dwords; ++w) {
                *(device uint*)(t + q.dst + 4u * w) = q.kind == 1u ? i : rd(cmd, q.src + 4u * w);
            }
        }
        tl = p.tlab_out + (ulong)i * p.tlab_stride;
    }

    bind_mesh_buf(rc, p.heap, 0u, p.has_object, p.has_ps);
    bind_mesh_buf(rc, p.sampler_heap, 1u, p.has_object, p.has_ps);
    bind_mesh_buf(rc, tl, 2u, p.has_object, p.has_ps);
    uint3 obj_tg = p.has_object != 0u ? uint3(p.obj_x, p.obj_y, p.obj_z) : uint3(1u, 1u, 1u);
    rc.draw_mesh_threadgroups(uint3(gx, gy, gz), obj_tg, uint3(p.mesh_x, p.mesh_y, p.mesh_z));
}

kernel void aw2_ei_mesh_icb(constant MeshIcbParams& p [[buffer(0)]], uint i [[thread_position_in_grid]]) {
    encode_mesh(p, i);
}

vertex void aw2_ei_mesh_icb_vs(constant MeshIcbParams& p [[buffer(0)]], uint i [[vertex_id]]) {
    encode_mesh(p, i);
}
"#;

struct MeshIcbLib(Result<Retained<crate::metal::pipeline::MtlLibrary>, String>);
unsafe impl Send for MeshIcbLib {}
unsafe impl Sync for MeshIcbLib {}

fn mesh_icb_library() -> Result<&'static crate::metal::pipeline::MtlLibrary, String> {
    static L: OnceLock<MeshIcbLib> = OnceLock::new();
    L.get_or_init(|| MeshIcbLib(crate::metal::pipeline::library_from_source("mesh indirect icb kernel", MESH_ICB_SRC)))
        .0
        .as_deref()
        .map_err(|e| e.clone())
}

struct MeshIcbKernel(Result<Retained<MtlComputePso>, String>);
unsafe impl Send for MeshIcbKernel {}
unsafe impl Sync for MeshIcbKernel {}

/// Compute fallback expander for `PORT_EI_INPASS=0` mesh `ExecuteIndirect` ICBs.
pub fn mesh_icb_kernel() -> Result<&'static MtlComputePso, String> {
    let tg = crate::metal::pipeline::fast_bind_pso().then_some([64, 1, 1]);
    static K: OnceLock<MeshIcbKernel> = OnceLock::new();
    let k = K.get_or_init(|| {
        MeshIcbKernel(mesh_icb_library().and_then(|lib| {
            crate::metal::pipeline::compute_pipeline_from_function_tg(lib, "aw2_ei_mesh_icb", tg)
        }))
    });
    k.0.as_deref().map_err(|e| e.clone())
}

struct MeshExpanders(Mutex<HashMap<(Vec<u32>, u32), Retained<MtlRenderPipeline>>>);
unsafe impl Send for MeshExpanders {}
unsafe impl Sync for MeshExpanders {}

/// In-pass `Point` vertex-stage expander (`aw2_ei_mesh_icb_vs`) for a render pass with
/// `rtv_formats` (DXGI) and `sample_count`, encoding `MTLIndirectCommandTypeDrawMeshThreadgroups`
/// commands without splitting the open render pass.
pub fn mesh_icb_expander(
    rtv_formats: &[u32],
    sample_count: u32,
) -> Result<Retained<MtlRenderPipeline>, String> {
    static E: OnceLock<MeshExpanders> = OnceLock::new();
    let e = E.get_or_init(|| MeshExpanders(Mutex::new(HashMap::new())));
    let key = (rtv_formats.to_vec(), sample_count.max(1));
    if let Some(p) = e.0.lock().unwrap().get(&key) {
        return Ok(p.clone());
    }
    let lib = mesh_icb_library()?;
    let desc = MTL4RenderPipelineDescriptor::new();
    desc.setVertexFunctionDescriptor(Some(&crate::metal::pipeline::function_descriptor(
        lib,
        "aw2_ei_mesh_icb_vs",
    )));
    desc.setRasterizationEnabled(false);
    desc.setLabel(Some(&NSString::from_str("aw2 ExecuteIndirect mesh expander")));
    let ca = desc.colorAttachments();
    for (i, f) in rtv_formats.iter().enumerate().take(8) {
        let a = unsafe { ca.objectAtIndexedSubscript(i) };
        a.setPixelFormat(crate::format::info(*f).mtl);
    }
    unsafe { desc.setRasterSampleCount(key.1 as usize) };
    desc.setInputPrimitiveTopology(MTLPrimitiveTopologyClass::Point);
    let mut h = dxil_metal::hash::KeyHasher::new("d3d12-metal ei mesh expander v1");
    h.bytes(format!("{:?}|{}", key.0, key.1).as_bytes());
    let pso = pso_cache()?
        .render_pipeline(Some(h.finish()), &desc)
        .map_err(|err| format!("ExecuteIndirect mesh expander: {err}"))?;
    e.0.lock().unwrap().insert(key, pso.clone());
    Ok(pso)
}

struct MeshIcbChunk {
    icb: Retained<MtlIcb>,
    cap: u32,
    off: u32,
    has_object: bool,
}
unsafe impl Send for MeshIcbChunk {}

impl Drop for MeshIcbChunk {
    fn drop(&mut self) {
        if let Some(d) = crate::metal::device() {
            d.remove_resident(ProtocolObject::from_ref(&*self.icb));
        }
    }
}

/// Ref-counted owner of a [`MeshIcbChunk`].
///
/// Both the recording-level table ([`ACTIVE_MESH_ICBS`], held until `CommandList::begin`
/// or `CommandAllocator::reset`/`Drop`) and every outstanding GPU submission's
/// [`MeshIcbLease`] hold an [`Arc<MeshIcbChunkGuard>`]. The chunk is only recycled back
/// into [`MESH_ICB_POOL`] (or dropped to remove residency) once the recording is released
/// **and** every submitted GPU execution referencing it has completed.
struct MeshIcbChunkGuard {
    chunk: Mutex<Option<MeshIcbChunk>>,
}

impl Drop for MeshIcbChunkGuard {
    fn drop(&mut self) {
        let Some(mut c) = self.chunk.lock().unwrap().take() else {
            return;
        };
        c.off = 0;
        let mut pool = MESH_ICB_POOL.lock().unwrap();
        if pool.len() < MAX_POOLED_MESH_ICBS {
            pool.push(c);
        }
    }
}

struct RecordedMeshIcbs {
    alloc_key: usize,
    chunks: Vec<Arc<MeshIcbChunkGuard>>,
}

/// GPU-completion-guarded lease returned to [`Queue::defer_release`] on every submission
/// of a command buffer carrying mesh ICBs.
pub(crate) struct MeshIcbLease {
    _chunks: Vec<Arc<MeshIcbChunkGuard>>,
}
unsafe impl Send for MeshIcbLease {}

const MAX_POOLED_MESH_ICBS: usize = 16;

static MESH_ICB_POOL: Mutex<Vec<MeshIcbChunk>> = Mutex::new(Vec::new());
static ACTIVE_MESH_ICBS: OnceLock<Mutex<HashMap<usize, RecordedMeshIcbs>>> = OnceLock::new();
static ACTIVE_MESH_ICB_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Fast lock-free check used on queue submit and allocator/list reset so non-mesh paths
/// pay one atomic load.
#[inline]
pub(crate) fn has_active_mesh_icbs() -> bool {
    ACTIVE_MESH_ICB_COUNT.load(Ordering::Acquire) > 0
}

/// Retain a [`MeshIcbLease`] for one GPU submission of `cb_ptr` without releasing
/// recording-level ownership, so a closed command list submitted multiple times without
/// `Reset` tracks every outstanding execution and never recycles its ICBs prematurely.
pub(crate) fn retain_mesh_icbs_for_cb(cb_ptr: usize) -> Option<MeshIcbLease> {
    if !has_active_mesh_icbs() {
        return None;
    }
    let map = ACTIVE_MESH_ICBS.get()?;
    let guard = map.lock().unwrap();
    let rec = guard.get(&cb_ptr)?;
    if rec.chunks.is_empty() {
        None
    } else {
        Some(MeshIcbLease { _chunks: rec.chunks.clone() })
    }
}

/// Release recording-level mesh ICB ownership for `cb_ptr` when `CommandList::begin`
/// starts a new recording on `cb_ptr`. Any in-flight GPU submission still holds its own
/// [`MeshIcbLease`] clone until GPU completion.
pub(crate) fn release_mesh_icbs_for_cb(cb_ptr: usize) {
    if !has_active_mesh_icbs() {
        return;
    }
    let Some(map) = ACTIVE_MESH_ICBS.get() else {
        return;
    };
    let removed = map.lock().unwrap().remove(&cb_ptr);
    if removed.is_some() {
        ACTIVE_MESH_ICB_COUNT.fetch_sub(1, Ordering::AcqRel);
    }
    drop(removed);
}

/// Release recording-level mesh ICB ownership for every command buffer recorded on
/// `alloc_key` when `CommandAllocator::reset` or `Drop` runs. Any in-flight GPU
/// submission still holds its own [`MeshIcbLease`] clone until GPU completion.
pub(crate) fn release_mesh_icbs_for_alloc(alloc_key: usize) {
    if alloc_key == 0 || !has_active_mesh_icbs() {
        return;
    }
    let Some(map) = ACTIVE_MESH_ICBS.get() else {
        return;
    };
    let mut removed: Vec<RecordedMeshIcbs> = Vec::new();
    {
        let mut guard = map.lock().unwrap();
        let keys: Vec<usize> = guard
            .iter()
            .filter_map(|(&k, v)| (v.alloc_key == alloc_key).then_some(k))
            .collect();
        for k in keys {
            if let Some(rec) = guard.remove(&k) {
                ACTIVE_MESH_ICB_COUNT.fetch_sub(1, Ordering::AcqRel);
                removed.push(rec);
            }
        }
    }
    drop(removed);
}

/// Allocate `count` mesh ICB command slots (`MTLIndirectCommandTypeDrawMeshThreadgroups`)
/// for command buffer `cb_ptr` (recorded on `alloc_key`), packing multiple draws per
/// command buffer into one ICB and recycling completed chunks from [`MESH_ICB_POOL`].
pub(crate) fn alloc_mesh_icb(
    cb_ptr: usize,
    alloc_key: usize,
    count: u32,
    has_object: bool,
) -> Result<(Retained<MtlIcb>, u32), String> {
    let need = count.max(1);
    let map = ACTIVE_MESH_ICBS.get_or_init(|| Mutex::new(HashMap::new()));
    {
        let guard = map.lock().unwrap();
        if let Some(rec) = guard.get(&cb_ptr) {
            if rec.alloc_key == alloc_key {
                for g in &rec.chunks {
                    if Arc::strong_count(g) == 1 {
                        let mut slot = g.chunk.lock().unwrap();
                        if let Some(c) = slot.as_mut() {
                            if c.has_object == has_object && c.cap.saturating_sub(c.off) >= need {
                                let loc = c.off;
                                c.off += need;
                                return Ok((c.icb.clone(), loc));
                            }
                        }
                    }
                }
            }
        }
    }

    // Try recycling a GPU-completed chunk from the pool.
    let reused = {
        let mut pool = MESH_ICB_POOL.lock().unwrap();
        let best = pool
            .iter()
            .enumerate()
            .filter(|(_, c)| c.has_object == has_object && c.cap >= need)
            .min_by_key(|(_, c)| c.cap)
            .map(|(i, _)| i);
        best.map(|i| pool.swap_remove(i))
    };

    let mut chunk = match reused {
        Some(mut c) => {
            c.off = 0;
            c
        }
        None => {
            let d = crate::metal::device().ok_or("no Metal device")?;
            let cap = need.next_power_of_two().max(256);
            let desc = MTLIndirectCommandBufferDescriptor::new();
            desc.setCommandTypes(MTLIndirectCommandType::DrawMeshThreadgroups);
            desc.setInheritPipelineState(true);
            desc.setInheritBuffers(false);
            desc.setInheritDepthStencilState(true);
            desc.setInheritDepthBias(true);
            desc.setInheritDepthClipMode(true);
            desc.setInheritCullMode(true);
            desc.setInheritFrontFacingWinding(true);
            desc.setInheritTriangleFillMode(true);
            unsafe {
                desc.setMaxObjectBufferBindCount(if has_object { 3 } else { 0 });
                desc.setMaxMeshBufferBindCount(3);
                desc.setMaxFragmentBufferBindCount(3);
            }
            let opts = MTLResourceOptions::StorageModePrivate
                | MTLResourceOptions::HazardTrackingModeUntracked;
            let icb = unsafe {
                d.dev
                    .newIndirectCommandBufferWithDescriptor_maxCommandCount_options(
                        &desc,
                        cap as usize,
                        opts,
                    )
            }
            .ok_or_else(|| format!("newIndirectCommandBuffer(DrawMeshThreadgroups, {cap}) failed"))?;
            icb.setLabel(Some(&NSString::from_str("aw2 ExecuteIndirect Mesh ICB")));
            d.add_resident(ProtocolObject::from_ref(&*icb));
            MeshIcbChunk { icb, cap, off: 0, has_object }
        }
    };

    let loc = chunk.off;
    chunk.off += need;
    let out = chunk.icb.clone();
    let wrapped = Arc::new(MeshIcbChunkGuard { chunk: Mutex::new(Some(chunk)) });
    let mut guard = map.lock().unwrap();
    match guard.entry(cb_ptr) {
        std::collections::hash_map::Entry::Vacant(v) => {
            v.insert(RecordedMeshIcbs { alloc_key, chunks: vec![wrapped] });
            ACTIVE_MESH_ICB_COUNT.fetch_add(1, Ordering::AcqRel);
        }
        std::collections::hash_map::Entry::Occupied(mut o) => {
            let rec = o.get_mut();
            if rec.alloc_key != alloc_key {
                rec.alloc_key = alloc_key;
                rec.chunks.clear();
            }
            rec.chunks.push(wrapped);
        }
    }
    Ok((out, loc))
}

/// Validates `DispatchMesh` arguments against the D3D12 limits. `Ok(false)` means an empty
/// dispatch, which is a no-op.
pub fn check_dispatch(x: u32, y: u32, z: u32) -> Result<bool, String> {
    if x == 0 || y == 0 || z == 0 {
        return Ok(false);
    }
    if x > MAX_DISPATCH_DIM || y > MAX_DISPATCH_DIM || z > MAX_DISPATCH_DIM {
        return Err(format!("DispatchMesh({x}, {y}, {z}): dimension over {MAX_DISPATCH_DIM}"));
    }
    if x as u64 * y as u64 * z as u64 > MAX_DISPATCH_GROUPS {
        return Err(format!("DispatchMesh({x}, {y}, {z}): more than 2^22 threadgroups"));
    }
    Ok(true)
}

/// Encodes `DispatchMesh(x, y, z)` with `p`, which the caller has set on `enc` (pipeline state,
/// raster and depth-stencil state, argument table for [`STAGES`]).
pub fn encode_dispatch(enc: &ProtocolObject<dyn MTL4RenderCommandEncoder>, p: &MeshPipeline, x: u32, y: u32, z: u32) -> Result<(), String> {
    if !check_dispatch(x, y, z)? {
        return Ok(());
    }
    let object = mtl_size(p.object_threads.unwrap_or([1, 1, 1]));
    enc.drawMeshThreadgroups_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(mtl_size([x, y, z]), object, mtl_size(p.mesh_threads));
    Ok(())
}

/// Encodes one indirect `DispatchMesh` (an `ExecuteIndirect` command with a
/// `D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_MESH` argument). `args_va` is the GPU address of a
/// `D3D12_DISPATCH_MESH_ARGUMENTS` ({x, y, z} u32), which has the same layout as
/// `MTLDispatchThreadgroupsIndirectArguments`, so Metal reads it in place. It must be
/// 4-byte aligned, and resident. D3D12's limits aren't checked: the GPU writes the values.
pub fn encode_dispatch_indirect(enc: &ProtocolObject<dyn MTL4RenderCommandEncoder>, p: &MeshPipeline, args_va: u64) -> Result<(), String> {
    if !args_va.is_multiple_of(4) {
        return Err(format!("DispatchMesh indirect arguments at {args_va:#x} are not 4-byte aligned"));
    }
    let object = mtl_size(p.object_threads.unwrap_or([1, 1, 1]));
    enc.drawMeshThreadgroupsWithIndirectBuffer_threadsPerObjectThreadgroup_threadsPerMeshThreadgroup(args_va, object, mtl_size(p.mesh_threads));
    Ok(())
}

/// Sets `p`'s pipeline-level state on `enc`: the pipeline, rasterizer and depth-stencil
/// state. Dynamic state (viewports, scissors, stencil ref, blend factor) is the command
/// list's job, exactly as for graphics PSOs.
pub fn apply_pipeline(enc: &ProtocolObject<dyn MTL4RenderCommandEncoder>, p: &MeshPipeline) {
    enc.setRenderPipelineState(&p.pso);
    p.raster.apply(enc);
    enc.setDepthStencilState(Some(&p.depth_stencil));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_limits() {
        assert_eq!(check_dispatch(0, 1, 1), Ok(false));
        assert_eq!(check_dispatch(1, 1, 1), Ok(true));
        assert_eq!(check_dispatch(65535, 64, 1), Ok(true));
        assert!(check_dispatch(65536, 1, 1).is_err());
        assert!(check_dispatch(65535, 65535, 1).is_err());
        assert_eq!(check_dispatch(1 << 11, 1 << 11, 1), Ok(true));
        assert!(check_dispatch(1 << 11, 1 << 11, 2).is_err());
    }

    #[test]
    fn stages_cover_object_mesh_fragment() {
        assert!(STAGES.contains(MTLRenderStages::Object | MTLRenderStages::Mesh | MTLRenderStages::Fragment));
        assert!(!STAGES.contains(MTLRenderStages::Vertex));
    }

    #[test]
    fn enabled_default_on_and_env_precedence() {
        let _lock = crate::metal::env_test_lock();
        let prev_p = std::env::var_os("PORT_MESH_SHADERS");
        let prev_a = std::env::var_os("AW2_MESH_SHADERS");
        struct Restore(Option<std::ffi::OsString>, Option<std::ffi::OsString>);
        impl Drop for Restore {
            fn drop(&mut self) {
                match &self.0 {
                    Some(v) => std::env::set_var("PORT_MESH_SHADERS", v),
                    None => std::env::remove_var("PORT_MESH_SHADERS"),
                }
                match &self.1 {
                    Some(v) => std::env::set_var("AW2_MESH_SHADERS", v),
                    None => std::env::remove_var("AW2_MESH_SHADERS"),
                }
            }
        }
        let _restore = Restore(prev_p, prev_a);

        std::env::remove_var("PORT_MESH_SHADERS");
        std::env::remove_var("AW2_MESH_SHADERS");
        assert!(enabled(), "mesh shaders must default ON");

        // Legacy alias can disable.
        std::env::set_var("AW2_MESH_SHADERS", "0");
        assert!(!enabled());

        // Canonical overrides legacy alias in both directions.
        std::env::set_var("PORT_MESH_SHADERS", "1");
        assert!(enabled());

        std::env::set_var("AW2_MESH_SHADERS", "1");
        std::env::set_var("PORT_MESH_SHADERS", "0");
        assert!(!enabled());

        // Empty canonical falls back to legacy alias.
        std::env::set_var("PORT_MESH_SHADERS", "");
        assert!(enabled());
    }

    #[test]
    fn execute_indirect_mesh_icb_in_pass_count_patches_and_excess_rtv() {
        use crate::metal::cmdlist::{CommandAllocator, CommandList, TargetView};
        use crate::metal::indirect::{arg, IndirectLayout};
        use crate::metal::queue::{ListType, Queue};
        use crate::metal::resource::Resource;
        use crate::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};

        if !dxil_metal::converter_available() || crate::metal::device().is_none() {
            return;
        }
        const AS_BLOB: &[u8] = include_bytes!("../tests/fixtures/mesh.as.dxil");
        const MS_BLOB: &[u8] = include_bytes!("../tests/fixtures/mesh.ms.dxil");
        const MS_DIRECT_BLOB: &[u8] = include_bytes!("../tests/fixtures/mesh_direct.ms.dxil");
        const PS_BLOB: &[u8] = include_bytes!("../tests/fixtures/mesh.ps.dxil");
        const SIZE: usize = 64;
        const RGBA8: u32 = 28;

        let signed = |b: &[u8]| -> Vec<u8> {
            let mut v = b.to_vec();
            dxil_metal::dxbc::sign_container(&mut v).unwrap();
            v
        };
        let as_s = signed(AS_BLOB);
        let ms_s = signed(MS_BLOB);
        let msd_s = signed(MS_DIRECT_BLOB);
        let ps_s = signed(PS_BLOB);
        let rs = RootSignature::from_blob(&as_s).expect("mesh test root sig");
        let mut blend = BlendDesc::default();
        blend.RenderTarget[0].RenderTargetWriteMask = 0xf;
        let mk_pso = |amp: Option<&[u8]>, ms: &[u8]| -> Arc<MeshPipeline> {
            let md = MeshDesc {
                amplification: amp,
                mesh: ms,
                pixel: Some(&ps_s),
                blend,
                sample_mask: u32::MAX,
                raster: RasterState { cull: MTLCullMode::None, ..Default::default() },
                depth_stencil: DepthStencilState::default(),
                rtv_formats: vec![RGBA8],
                dsv_format: 0,
                sample_count: 1,
            };
            Arc::new(MeshPipeline::new(rs.clone(), &md, "mesh icb test pso").unwrap())
        };
        let pso_as = mk_pso(Some(&as_s), &ms_s);
        let pso_ms = mk_pso(None, &msd_s);

        let mk_tex = |label: &str| -> Resource {
            let d = ResourceDesc {
                Dimension: dim::TEXTURE2D,
                Width: SIZE as u64,
                Height: SIZE as u32,
                DepthOrArraySize: 1,
                MipLevels: 1,
                Format: RGBA8,
                SampleCount: 1,
                Flags: resource_flags::ALLOW_RENDER_TARGET,
                ..Default::default()
            };
            let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
            Resource::new_committed_default(&d, &hp, label).unwrap()
        };
        let mk_buf = |ty: u32, bytes: u64, label: &str| -> Resource {
            let d = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: bytes,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            let hp = HeapProperties { Type: ty, ..Default::default() };
            Resource::new_committed_default(&d, &hp, label).unwrap()
        };

        let rtv0 = mk_tex("mesh icb rtv0");
        // Bind a SECOND RTV via OMSetRenderTargets while drawing with a 1-RTV mesh PSO
        // to exercise both Task 2 (in-pass mesh ICB) and Task 4 (MTL_DEBUG_LAYER excess-RTV fix).
        let rtv1 = mk_tex("mesh icb excess rtv1");
        let args_buf = mk_buf(heap_type::UPLOAD, 256, "mesh icb args");
        let count_buf = mk_buf(heap_type::UPLOAD, 64, "mesh icb count");
        let rb0 = mk_buf(heap_type::READBACK, (SIZE * SIZE * 4) as u64, "mesh icb rb0");
        let rb1 = mk_buf(heap_type::READBACK, (SIZE * SIZE * 4) as u64, "mesh icb rb1");

        // Command layout: 4 root constants (param 0, dest 0, 4 dwords) + DispatchMesh (12 bytes) = 28B, stride 32B.
        let layout = IndirectLayout::new(
            32,
            &[
                (arg::CONSTANT, [0, 0, 4]),
                (arg::DISPATCH_MESH, [0, 0, 0]),
            ],
        )
        .unwrap();
        // Cmd 0: empty grid (0, 1, 1) -> must be reset/skipped by GPU expander.
        // Cmd 1: AS+MS dispatch (4, 1, 1) with cull_mask = 0b0010 (cull Q1), grey = 180.
        // Cmd 2: beyond count (= 2) -> would draw all 4 quads with grey = 250 if count ignored!
        // Cmd 3: beyond count (= 2).
        let cmds: [[u32; 8]; 4] = [
            [0b0000, 250, 0, 0, 0, 1, 1, 0],
            [0b0010, 180, 0, 0, 4, 1, 1, 0],
            [0b0000, 250, 0, 0, 4, 1, 1, 0],
            [0b0000, 250, 0, 0, 4, 1, 1, 0],
        ];
        unsafe {
            std::ptr::copy_nonoverlapping(
                cmds.as_ptr().cast::<u8>(),
                args_buf.cpu_ptr().unwrap(),
                std::mem::size_of_val(&cmds),
            );
            *(count_buf.cpu_ptr().unwrap() as *mut u32) = 2;
        }

        let q = Queue::new(ListType::Direct, "mesh icb test q").unwrap();
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();

        let tv0 = TargetView::rtv(&rtv0, None).unwrap();
        let tv1 = TargetView::rtv(&rtv1, None).unwrap();

        l.begin(&alloc).unwrap();
        unsafe {
            l.clear_rtv(&tv0, [0.0, 0.0, 0.0, 1.0], &[]).unwrap();
            l.clear_rtv(&tv1, [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
        }
        l.set_render_targets(vec![tv0, tv1], None);
        l.set_viewports_raw(&[(0.0, 0.0, SIZE as f32, SIZE as f32, 0.0, 1.0)]);
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_constants(0, 0, &[0, 0, 0, 0]).unwrap();
        l.set_mesh_pipeline(pso_as.clone());

        // ExecuteIndirect(DispatchMesh) with count buffer + root-constant patch (AS+MS+PS).
        l.execute_indirect(
            &layout,
            4,
            args_buf.gpu_va,
            None,
            Some(count_buf.gpu_va),
        )
        .unwrap();

        // Now test MS-only (`has_object == false`) in the SAME open render pass without splitting:
        // Cmd 0 only draws Q3 with grey = 96; actually mesh_direct draws all 4 quads with grey = const[1].
        let args_ms = mk_buf(heap_type::UPLOAD, 128, "mesh icb args ms-only");
        let count_ms = mk_buf(heap_type::UPLOAD, 64, "mesh icb count ms-only");
        let cmds_ms: [[u32; 8]; 2] = [
            // Empty grid (skipped) -> keeps AS+MS output intact!
            [0, 12, 0, 0, 0, 1, 1, 0],
            // Beyond count=1 -> would overwrite Q1 if count ignored!
            [0, 240, 0, 0, 4, 1, 1, 0],
        ];
        unsafe {
            std::ptr::copy_nonoverlapping(
                cmds_ms.as_ptr().cast::<u8>(),
                args_ms.cpu_ptr().unwrap(),
                std::mem::size_of_val(&cmds_ms),
            );
            *(count_ms.cpu_ptr().unwrap() as *mut u32) = 1;
        }
        l.set_mesh_pipeline(pso_ms.clone());
        l.execute_indirect(
            &layout,
            2,
            args_ms.gpu_va,
            None,
            Some(count_ms.gpu_va),
        )
        .unwrap();

        l.barrier_list(&[
            crate::metal::cmdlist::Barrier::Transition(&rtv0 as *const Resource as usize, 0x4, 0x800),
            crate::metal::cmdlist::Barrier::Transition(&rtv1 as *const Resource as usize, 0x4, 0x800),
        ]);
        let fp = crate::layout::PlacedFootprint {
            offset: 0,
            format: RGBA8,
            width: SIZE as u32,
            height: SIZE as u32,
            depth: 1,
            row_pitch: (SIZE * 4) as u32,
            _pad: 0,
        };
        l.copy_texture_region(
            crate::metal::cmdlist::CopyLoc::Footprint(&rb0, fp),
            [0, 0, 0],
            crate::metal::cmdlist::CopyLoc::Subresource(&rtv0, 0),
            None,
        )
        .unwrap();
        l.copy_texture_region(
            crate::metal::cmdlist::CopyLoc::Footprint(&rb1, fp),
            [0, 0, 0],
            crate::metal::cmdlist::CopyLoc::Subresource(&rtv1, 0),
            None,
        )
        .unwrap();
        l.close().unwrap();
        let _ = q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();

        let px0 = unsafe {
            std::slice::from_raw_parts(rb0.cpu_ptr().unwrap() as *const [u8; 4], SIZE * SIZE)
        };
        let px1 = unsafe {
            std::slice::from_raw_parts(rb1.cpu_ptr().unwrap() as *const [u8; 4], SIZE * SIZE)
        };
        let quad = |px: &[[u8; 4]], qi: usize| -> [u8; 4] {
            let x = if qi & 1 == 1 { SIZE * 3 / 4 } else { SIZE / 4 };
            let y = if qi & 2 == 2 { SIZE * 3 / 4 } else { SIZE / 4 };
            px[y * SIZE + x]
        };
        // Also test:
        // (1) GPU-written DEFAULT args/count/pred buffers synchronized before the in-pass
        //     Vertex expander (P1 review fix), and
        // (2) repeated submission of the same closed mesh CommandList without Reset,
        //     retaining mesh ICB ownership until CommandList::begin / CommandAllocator::reset.
        {
            alloc.reset();
            let rtv_ms = mk_tex("mesh icb rtv_ms");
            let rb_ms = mk_buf(heap_type::READBACK, (SIZE * SIZE * 4) as u64, "mesh icb rb_ms");
            let args_up = mk_buf(heap_type::UPLOAD, 64, "mesh icb args_up");
            let cnt_up = mk_buf(heap_type::UPLOAD, 64, "mesh icb cnt_up");
            let pred_up = mk_buf(heap_type::UPLOAD, 64, "mesh icb pred_up");
            let args_def = mk_buf(heap_type::DEFAULT, 64, "mesh icb args_def");
            let cnt_def = mk_buf(heap_type::DEFAULT, 64, "mesh icb cnt_def");
            let pred_def = mk_buf(heap_type::DEFAULT, 64, "mesh icb pred_def");
            let c_m: [[u32; 8]; 1] = [[0, 123, 0, 0, 4, 1, 1, 0]];
            unsafe {
                std::ptr::copy_nonoverlapping(c_m.as_ptr().cast::<u8>(), args_up.cpu_ptr().unwrap(), 32);
                *(cnt_up.cpu_ptr().unwrap() as *mut u32) = 1;
                // D3D12_PREDICATION_OP_EQUAL_ZERO (0): pred == 1 -> do NOT skip.
                *(pred_up.cpu_ptr().unwrap() as *mut u64) = 1;
            }
            let tvm = TargetView::rtv(&rtv_ms, None).unwrap();
            l.begin(&alloc).unwrap();
            l.copy_buffer(&args_def, 0, &args_up, 0, 32).unwrap();
            l.copy_buffer(&cnt_def, 0, &cnt_up, 0, 4).unwrap();
            l.copy_buffer(&pred_def, 0, &pred_up, 0, 8).unwrap();
            // Transition COPY_DEST (0x400) -> INDIRECT_ARGUMENT (0x200) before in-pass expansion.
            l.barrier_list(&[
                crate::metal::cmdlist::Barrier::Transition(&args_def as *const Resource as usize, 0x400, 0x200),
                crate::metal::cmdlist::Barrier::Transition(&cnt_def as *const Resource as usize, 0x400, 0x200),
                crate::metal::cmdlist::Barrier::Transition(&pred_def as *const Resource as usize, 0x400, 0x200),
            ]);
            unsafe { l.clear_rtv(&tvm, [0.0, 0.0, 0.0, 1.0], &[]).unwrap(); }
            l.set_render_targets(vec![tvm], None);
            l.set_viewports_raw(&[(0.0, 0.0, SIZE as f32, SIZE as f32, 0.0, 1.0)]);
            l.gfx_root.set_root_signature(rs.clone()).unwrap();
            l.gfx_root.set_constants(0, 0, &[0, 0, 0, 0]).unwrap();
            l.set_mesh_pipeline(pso_ms.clone());
            l.set_predication(Some((pred_def.gpu_va, 0)));
            l.execute_indirect(&layout, 1, args_def.gpu_va, None, Some(cnt_def.gpu_va)).unwrap();
            l.set_predication(None);
            l.barrier_list(&[crate::metal::cmdlist::Barrier::Transition(
                &rtv_ms as *const Resource as usize,
                0x4,
                0x800,
            )]);
            l.copy_texture_region(
                crate::metal::cmdlist::CopyLoc::Footprint(&rb_ms, fp),
                [0, 0, 0],
                crate::metal::cmdlist::CopyLoc::Subresource(&rtv_ms, 0),
                None,
            ).unwrap();
            l.close().unwrap();

            // 1st submission of closed list: recording ownership must stay active afterward.
            let cb_ptr = Retained::as_ptr(&l.cb) as usize;
            let _ = q.execute_command_lists(&mut [&mut l]);
            q.wait_idle();
            assert!(
                has_active_mesh_icbs(),
                "closed mesh command list must retain recording-level ICB ownership after submission"
            );
            let pxm = unsafe { std::slice::from_raw_parts(rb_ms.cpu_ptr().unwrap() as *const [u8; 4], SIZE * SIZE) };
            assert_eq!(quad(pxm, 0), [255, 0, 0, 255], "MS-only Q0 red");
            assert_eq!(quad(pxm, 1), [0, 255, 0, 255], "MS-only Q1 green");
            assert_eq!(quad(pxm, 2), [0, 0, 255, 255], "MS-only Q2 blue");
            assert_eq!(quad(pxm, 3), [123, 123, 123, 255], "MS-only Q3 patched grey=123");

            // Simulate a second outstanding submission lease on the still-closed list:
            // resetting the allocator drops recording-level ownership, while the chunk must
            // stay out of `MESH_ICB_POOL` until the second lease drops too.
            let lease2 = retain_mesh_icbs_for_cb(cb_ptr)
                .expect("closed mesh command list must yield a lease on repeated retention");
            let pool_before_reset = MESH_ICB_POOL.lock().unwrap().len();
            alloc.reset();
            assert!(
                !has_active_mesh_icbs(),
                "CommandAllocator::reset must release recording-level mesh ICB ownership"
            );
            assert_eq!(
                MESH_ICB_POOL.lock().unwrap().len(),
                pool_before_reset,
                "chunk must not recycle while an outstanding submission lease is still alive"
            );
            drop(lease2);
            assert!(
                MESH_ICB_POOL.lock().unwrap().len() > pool_before_reset
                    || MESH_ICB_POOL.lock().unwrap().len() == MAX_POOLED_MESH_ICBS,
                "dropping the final submission lease must recycle the mesh ICB chunk"
            );
        }
        assert_eq!(quad(px0, 0), [255, 0, 0, 255], "Q0 red");
        assert_eq!(quad(px0, 1), [0, 0, 0, 255], "Q1 must stay culled (count=2 + cull_mask=0b0010)");
        assert_eq!(quad(px0, 2), [0, 0, 255, 255], "Q2 blue");
        assert_eq!(quad(px0, 3), [180, 180, 180, 255], "Q3 must have patched grey=180");
        // Excess RTV1 must preserve its deferred green clear untouched!
        assert_eq!(quad(px1, 0), [0, 255, 0, 255], "excess RTV1 must preserve its green clear");
    }
}
