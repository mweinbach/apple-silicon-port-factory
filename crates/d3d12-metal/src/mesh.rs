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
//! The device reports `MeshShaderTier` 1 only when [`enabled`] (`AW2_MESH_SHADERS=1`).
//!
//! Limits that differ from D3D12:
//!
//! * An amplification shader may launch at most [`MAX_MESH_GRID_THREADGROUPS`] mesh
//!   threadgroups per object threadgroup. D3D12 allows 2^22, Metal 1024; larger grids are
//!   clamped by Metal.
//! * The payload is capped at 16 KB on both APIs.

use std::sync::Arc;

use dxil_metal::root_signature::RootSignature;
use dxil_metal::{CompileOptions, CompiledShader, ShaderStage, StageInfo};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::*;

use crate::metal::pipeline::{compile, depth_stencil_state, library, pso_cache, shader_function_descriptor, MtlDepthStencil, MtlRenderPipeline};
use crate::state::{BlendDesc, BlendState, DepthStencilState, RasterState};

/// `AW2_MESH_SHADERS=1`: report `D3D12_MESH_SHADER_TIER_1` and accept AS/MS pipelines.
pub fn enabled() -> bool {
    std::env::var("AW2_MESH_SHADERS").is_ok_and(|v| v == "1")
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

pub struct MeshPipeline {
    pub pso: Retained<MtlRenderPipeline>,
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

        let desc = MTL4MeshRenderPipelineDescriptor::new();
        let mut key = dxil_metal::hash::KeyHasher::new("d3d12-metal mesh pso v2");
        if let Some(a) = &amplification {
            desc.setObjectFunctionDescriptor(Some(&shader_function_descriptor(&*library(a)?, a)));
            key.u128(a.key);
        }
        desc.setMeshFunctionDescriptor(Some(&shader_function_descriptor(&*library(&mesh)?, &mesh)));
        key.u128(mesh.key);
        if let Some(p) = &pixel {
            desc.setFragmentFunctionDescriptor(Some(&shader_function_descriptor(&*library(p)?, p)));
            key.u128(p.key);
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
            desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Disabled);
            desc.setSupportObjectBinaryLinking(false);
            desc.setSupportMeshBinaryLinking(false);
            desc.setSupportFragmentBinaryLinking(false);
            desc.setColorAttachmentMappingState(MTL4LogicalToPhysicalColorAttachmentMappingState::Identity);
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
        desc.setAlphaToCoverageState(if blend.alpha_to_coverage { MTL4AlphaToCoverageState::Enabled } else { MTL4AlphaToCoverageState::Disabled });
        crate::metal::pipeline::RenderPsoStateKey::new(blend, &d.rtv_formats, d.sample_count, payload_bytes)
            .write_to(&mut key);
        let mut pso_key = key.finish();
        if immut_opt {
            pso_key ^= crate::metal::pipeline::IMMUTABLE_OPT_KEY_SALT;
        }
        if pso_opt_v2 {
            pso_key ^= crate::metal::pipeline::SHADER_PSO_OPT_V2_SALT;
        }

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
}
