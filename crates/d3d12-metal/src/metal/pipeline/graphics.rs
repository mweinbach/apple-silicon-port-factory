//! Graphics and geometry-emulated pipelines, depth-stencil state cache, and
//! input-layout vertex format helpers.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use dxil_metal::pso_cache::PipelineCache;
use dxil_metal::root_signature::RootSignature;
use dxil_metal::{
    CompileOptions, CompiledShader, InputElement, ShaderPipeline, ShaderStage, StageInMode,
    StageInfo,
};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSArray, NSString};
use objc2_metal::*;

use crate::state::{BlendDesc, BlendState, DepthStencilState, RasterState};

use super::dxil_hk::HandleKindProfile;
use super::opts::{
    gfx_lazy_icb, msc_stage_usage, note_frag_buf_write, shader_immutable_opt, shader_pso_opt_v2,
    sm66_gfx_stage_exact, RenderPsoStateKey, IMMUTABLE_OPT_KEY_SALT, SHADER_PSO_OPT_V2_SALT,
};
use super::reflection::{ComputeAccess, ShaderStageSummary, READS_BUF, READS_TEX};
use super::{
    apply_immutable_pipeline_options, compile, function_descriptor, library, pso_cache,
    shader_function_descriptor, specialized_function_descriptor, MtlRenderPipeline,
};

pub type MtlDepthStencil = ProtocolObject<dyn MTLDepthStencilState>;

/// Metal buffer index of input-assembler slot `slot` (converter
/// `kIRVertexBufferBindPoint` convention).
pub fn vertex_buffer_index(slot: u32) -> usize {
    dxil_metal::runtime::VERTEX_BUFFER_BIND_POINT as usize + slot as usize
}

/// Highest usable IA slot (Metal has 31 buffer bind points).
pub const MAX_VERTEX_SLOTS: u32 = 24;

/// Maximum number of simultaneous color render targets in D3D12 and Metal.
pub const MAX_RENDER_TARGETS: usize = 8;

/// Maximum Metal vertex attribute index (`0..=30`).
const MAX_VERTEX_ATTRIBUTES: usize = 31;

/// Validates D3D12 render-target formats, MSAA sample count, and dual-source blend rules,
/// and disables fixed-function blending on non-blendable attachments (e.g. `UINT`/`SINT`
/// integer MRTs when `IndependentBlendEnable = FALSE` broadcasts RT0's blend state, or
/// 32-bit float attachments on devices without `float32_blend`) while preserving each
/// attachment's color write mask.
pub(crate) fn resolve_render_targets_and_blend_with(
    blend_desc: &BlendDesc,
    rtv_formats: &[u32],
    sample_count: u32,
    float32_blend: bool,
    supports_sample_count: impl FnOnce(u32) -> bool,
) -> Result<(BlendState, [MTLPixelFormat; MAX_RENDER_TARGETS], usize), String> {
    if rtv_formats.len() > MAX_RENDER_TARGETS {
        return Err(format!(
            "too many render targets: {} (max {MAX_RENDER_TARGETS})",
            rtv_formats.len()
        ));
    }
    let samples = sample_count.max(1);
    if !supports_sample_count(samples) {
        return Err(format!("unsupported raster sample count {samples}"));
    }

    let mut mtl_formats = [MTLPixelFormat::Invalid; MAX_RENDER_TARGETS];
    let mut active_rtvs = 0usize;
    for (i, &dxgi) in rtv_formats.iter().enumerate() {
        if dxgi == 0 {
            continue;
        }
        if !crate::format::is_render_target_format(dxgi) {
            return Err(format!(
                "render target {i} format {dxgi} ({}) is not color-renderable",
                crate::format::name(dxgi)
            ));
        }
        mtl_formats[i] = crate::format::info(dxgi).mtl;
        active_rtvs += 1;
    }

    let mut blend = crate::state::blend_state(blend_desc);
    let mut sanitized = false;
    for (i, &dxgi) in rtv_formats.iter().enumerate() {
        if mtl_formats[i] == MTLPixelFormat::Invalid || !blend.attachments[i].enabled {
            continue;
        }
        if !crate::format::is_blendable_format(dxgi, float32_blend) {
            let write_mask = blend.attachments[i].write_mask;
            blend.attachments[i] = crate::state::ColorBlend {
                write_mask,
                ..crate::state::ColorBlend::default()
            };
            sanitized = true;
            let info = crate::format::info(dxgi);
            let is_integer = matches!(info.num, crate::format::Num::Uint | crate::format::Num::Sint);
            if blend_desc.IndependentBlendEnable == 0 && is_integer {
                log::debug!(
                    target: "d3d",
                    "render PSO: disabling broadcast blend on integer RT {i} ({})",
                    crate::format::name(dxgi)
                );
            } else {
                log::warn!(
                    target: "d3d",
                    "render PSO: disabling blend on non-blendable RT {i} ({}, independent={}, float32_blend={float32_blend})",
                    crate::format::name(dxgi),
                    blend_desc.IndependentBlendEnable != 0
                );
            }
        }
    }

    if sanitized || blend.dual_source {
        let mut dual_source = false;
        let mut uses_blend_constant = false;
        for (i, &pf) in mtl_formats.iter().enumerate().take(rtv_formats.len()) {
            if pf != MTLPixelFormat::Invalid && blend.attachments[i].enabled {
                dual_source |= blend.attachments[i].uses_dual_source();
                uses_blend_constant |= blend.attachments[i].uses_blend_constant();
            }
        }
        blend.dual_source = dual_source;
        if sanitized {
            blend.uses_blend_constant = uses_blend_constant;
        }
    }

    for (i, &pf) in mtl_formats.iter().enumerate().take(rtv_formats.len()) {
        if pf != MTLPixelFormat::Invalid
            && blend.attachments[i].enabled
            && blend.attachments[i].uses_dual_source()
            && (i != 0 || active_rtvs > 1)
        {
            return Err(format!(
                "dual-source blending is only valid on render target 0 with a single active color attachment (rt {i}, active_rtvs {active_rtvs})"
            ));
        }
    }

    Ok((blend, mtl_formats, rtv_formats.len()))
}

/// Device-backed wrapper around [`resolve_render_targets_and_blend_with`].
pub(crate) fn resolve_render_targets_and_blend(
    blend_desc: &BlendDesc,
    rtv_formats: &[u32],
    sample_count: u32,
) -> Result<(BlendState, [MTLPixelFormat; MAX_RENDER_TARGETS], usize), String> {
    let dev = crate::metal::device();
    let float32_blend = dev.map_or(cfg!(target_os = "macos"), |d| d.float32_blend);
    resolve_render_targets_and_blend_with(blend_desc, rtv_formats, sample_count, float32_blend, |s| {
        dev.map_or(matches!(s, 1 | 2 | 4), |d| d.supports_sample_count(s))
    })
}

/// Applies validated color-attachment pixel formats and blend states to a Metal 4
/// render or mesh pipeline descriptor's color attachment array.
pub(crate) fn apply_color_attachments(
    ca: &MTL4RenderPipelineColorAttachmentDescriptorArray,
    mtl_formats: &[MTLPixelFormat; MAX_RENDER_TARGETS],
    rtv_count: usize,
    blend: &BlendState,
    pso_opt_v2: bool,
) {
    for (i, &pf) in mtl_formats.iter().enumerate().take(rtv_count) {
        let a = unsafe { ca.objectAtIndexedSubscript(i) };
        a.setPixelFormat(pf);
        if pf != MTLPixelFormat::Invalid {
            blend.attachments[i].apply_to_with_opt(&a, pso_opt_v2);
        }
    }
}

/// `D3D12_GRAPHICS_PIPELINE_STATE_DESC` (or a flattened stream), parsed.
pub struct GraphicsDesc<'a> {
    pub vs: &'a [u8],
    pub ps: Option<&'a [u8]>,
    pub blend: BlendDesc,
    pub sample_mask: u32,
    pub raster: RasterState,
    pub depth_stencil: DepthStencilState,
    pub input_layout: Vec<InputElement>,
    pub strip_cut: u32,
    pub topology_type: u32,
    /// `NumRenderTargets` entries.
    pub rtv_formats: Vec<u32>,
    pub dsv_format: u32,
    pub sample_count: u32,
}

/// Metal 4 object+mesh geometry shader emulation metadata attached to a [`GraphicsPipeline`]
/// built via [`GraphicsPipeline::new_with_geometry`].
pub struct GeometryEmulation {
    pub gs: Arc<CompiledShader>,
    pub stage_in: Option<Arc<CompiledShader>>,
    pub vertex_output_size_in_bytes: u32,
    pub max_input_primitives_per_mesh_threadgroup: u32,
    pub instance_count: u32,
}

/// Metal 4 object+mesh tessellation emulation metadata attached to a [`GraphicsPipeline`]
/// built via [`GraphicsPipeline::new_with_tessellation`] (VS+HS in object stage,
/// tessellator+DS+(optional GS or passthrough) in mesh stage, optional PS in fragment stage).
pub struct TessellationEmulation {
    pub hs: Arc<CompiledShader>,
    pub ds: Arc<CompiledShader>,
    pub gs: Option<Arc<CompiledShader>>,
    pub stage_in: Option<Arc<CompiledShader>>,
    /// `IRTessellatorOutputPrimitive`: 1=Point, 2=Line, 3=TriangleCW, 4=TriangleCCW.
    pub output_primitive: u32,
    pub vs_output_size_in_bytes: u32,
    pub gs_max_input_primitives_per_mesh_threadgroup: u32,
    pub hs_max_patches_per_object_threadgroup: u32,
    pub hs_input_control_point_count: u32,
    pub hs_max_object_threads_per_patch: u32,
    pub hs_max_tessellation_factor: f32,
    pub gs_instance_count: u32,
}

pub struct GraphicsPipeline {
    pub pso: Retained<MtlRenderPipeline>,
    pub rs: Arc<RootSignature>,
    pub raster: RasterState,
    pub blend: BlendState,
    pub depth_stencil: Retained<MtlDepthStencil>,
    pub depth_bounds: bool,
    pub needs_draw_params: bool,
    pub topology_type: u32,
    pub strip_cut: u32,
    pub rtv_formats: Vec<u32>,
    pub dsv_format: u32,
    pub sample_count: u32,
    /// IA slots the vertex descriptor reads (bit per slot).
    pub vertex_slots: u32,
    pub vs: Arc<CompiledShader>,
    pub ps: Option<Arc<CompiledShader>>,
    /// Resource classes the vertex stage may read through SRVs/UAVs
    /// ([`super::stage_reads_resources`]).
    pub pre_raster_reads: u8,
    /// Whether the vertex stage reads any IA vertex buffer slot (`vertex_slots != 0`)
    /// or any non-inline CBV resource (`AW2_VS_BUF_DEFER_EXACT=1`).
    pub vs_reads_cbv_or_ia: bool,
    /// The vertex or pixel stage may write through UAVs ([`super::stage_writes_uav`]).
    pub writes_uav: bool,
    /// Zero-salt MSC + AIR reflection exact stage flags (`AW2_SM66_GFX_STAGE_EXACT=1`).
    pub sm66_pre_raster_reads: u8,
    pub sm66_vs_reads_cbv_or_ia: bool,
    pub sm66_writes_uav: bool,
    pub sm66_narrowed: bool,
    /// Whether the vertex stage alone may write through UAVs (`AW2_LIST_START_RENDER_SPLIT=1`).
    pub vs_writes_uav: bool,
    /// Whether the pixel stage alone may write through UAVs (`AW2_RP_KEEP_WRITE_EXACT=1`).
    pub ps_writes_uav: bool,
    /// Resource classes (`READS_TEX | READS_BUF`, including CBVs) the pixel stage may read
    /// (`0` for depth-only `ps == None` passes; `AW2_RP_KEEP_WRITE_EXACT=1`).
    pub ps_read_classes: u8,
    /// SRV/UAV-only resource classes (`READS_TEX | READS_BUF`, excluding CBVs) the pixel stage
    /// may read (`AW2_PS_SRV_CLASS_EXACT=1`).
    pub ps_srv_uav_classes: u8,
    /// Whether either stage may read or write any UAV (`AW2_RP_KEEP_WRITE_EXACT=1`).
    pub uses_uav: bool,
    /// Zero-salt per-stage binding profiles (`AW2_GFX_DESC_TABLE_EXACT=1`).
    pub vs_access: ComputeAccess,
    pub ps_access: Option<ComputeAccess>,
    /// Embedded handle-kind profiles (eliminating `HK_PROFILE_REGISTRY.read()` on draw).
    pub vs_hk_profile: HandleKindProfile,
    pub ps_hk_profile: Option<HandleKindProfile>,
    /// Present when this pipeline emulates a D3D11/D3D12 geometry shader on a Metal 4
    /// mesh render pipeline (object = VS + linked stage-in, mesh = GS, fragment = PS).
    pub gs_emu: Option<GeometryEmulation>,
    /// Present when this pipeline emulates D3D11/D3D12 Hull+Domain tessellation (+ optional GS)
    /// on a Metal 4 mesh render pipeline.
    pub tess_emu: Option<TessellationEmulation>,
    pub(crate) icb_desc: Option<(Retained<MTL4RenderPipelineDescriptor>, u128)>,
    pub(crate) icb_pso: OnceLock<Result<Retained<MtlRenderPipeline>, String>>,
}
unsafe impl Send for GraphicsPipeline {}
unsafe impl Sync for GraphicsPipeline {}

struct DsCache(Mutex<HashMap<DepthStencilState, Retained<MtlDepthStencil>>>);
unsafe impl Send for DsCache {}
unsafe impl Sync for DsCache {}

/// The `MTLDepthStencilState` for `s` (cached for the process lifetime).
pub fn depth_stencil_state(s: &DepthStencilState) -> Result<Retained<MtlDepthStencil>, String> {
    static C: OnceLock<DsCache> = OnceLock::new();
    let c = C.get_or_init(|| DsCache(Mutex::new(HashMap::new())));
    {
        let m = c.0.lock().unwrap();
        if let Some(d) = m.get(s) {
            return Ok(d.clone());
        }
    }
    let d = crate::metal::device().ok_or("no device")?;
    let desc = MTLDepthStencilDescriptor::new();
    s.apply_to(&desc);
    let st = d
        .dev
        .newDepthStencilStateWithDescriptor(&desc)
        .ok_or_else(|| format!("newDepthStencilState({s:?})"))?;
    let mut m = c.0.lock().unwrap();
    Ok(m.entry(*s).or_insert(st).clone())
}

fn semantic_matches(reflected: &str, name: &str, index: u32) -> bool {
    // The converter reports e.g. "TEXCOORD3" (index appended).
    let n = name.len();
    reflected.len() >= n
        && reflected[..n].eq_ignore_ascii_case(name)
        && reflected[n..].parse::<u32>().unwrap_or(0) == index
        && (reflected.len() > n || index == 0)
}

/// Infers a 4-component DXGI format (`R32G32B32A32_FLOAT` = 2, `_UINT` = 3, `_SINT` = 4)
/// for a vertex shader input from the DXBC `ISG1` / `ISGN` input signature chunk when the
/// PSO's `input_layout` omits that attribute.
pub fn infer_vertex_input_format(vs_dxil: &[u8], reflected_name: &str) -> u32 {
    let rd32 = |b: &[u8], o: usize| b.get(o..o + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap()));
    if let Ok(c) = dxil_metal::dxbc::Container::parse(vs_dxil) {
        for (fcc, stride, name_off_pos, idx_pos, comp_pos) in [
            (*b"ISG1", 32usize, 4usize, 8usize, 16usize),
            (*b"ISGN", 24usize, 0usize, 4usize, 12usize),
        ] {
            if let Some(d) = c.part(fcc) {
                let count = rd32(d, 0).unwrap_or(0) as usize;
                for i in 0..count {
                    let base = 8 + i * stride;
                    let (Some(noff), Some(idx), Some(comp)) = (
                        rd32(d, base + name_off_pos),
                        rd32(d, base + idx_pos),
                        rd32(d, base + comp_pos),
                    ) else {
                        break;
                    };
                    if let Some(tail) = d.get(noff as usize..) {
                        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
                        let sem = std::str::from_utf8(&tail[..end]).unwrap_or("");
                        if semantic_matches(reflected_name, sem, idx) {
                            return match comp {
                                1 => 3, // DXGI_FORMAT_R32G32B32A32_UINT
                                2 => 4, // DXGI_FORMAT_R32G32B32A32_SINT
                                _ => 2, // DXGI_FORMAT_R32G32B32A32_FLOAT
                            };
                        }
                    }
                }
            }
        }
    }
    2 // DXGI_FORMAT_R32G32B32A32_FLOAT
}

impl GraphicsPipeline {
    pub fn new(
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        label: &str,
    ) -> Result<GraphicsPipeline, String> {
        Self::new_with_pso_opts(
            rs,
            d,
            label,
            gfx_lazy_icb(),
            shader_immutable_opt(),
            shader_pso_opt_v2(),
        )
    }

    /// Build a graphics pipeline that includes a D3D11/D3D12 Geometry Shader (`gs`) using
    /// Apple Metal Shader Converter geometry emulation on a Metal 4 mesh render pipeline:
    /// VS (`<name>.dxil_irconverter_object_shader`) + linked stage-in function in the object
    /// stage, GS in the mesh stage, and optional PS in the fragment stage.
    pub fn new_with_geometry(
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        gs_dxil: &[u8],
        label: &str,
    ) -> Result<GraphicsPipeline, String> {
        let immut_opt = shader_immutable_opt();
        let pso_opt_v2 = shader_pso_opt_v2();
        let rs_for_manifest = rs.clone();
        let out = Self::new_geometry_in_cache(
            pso_cache()?,
            rs,
            d,
            gs_dxil,
            label,
            immut_opt,
            pso_opt_v2,
        )?;
        crate::metal::pso_manifest::note_geometry(
            &rs_for_manifest,
            d,
            gs_dxil,
            immut_opt,
            pso_opt_v2,
        );
        Ok(out)
    }

    pub(crate) fn new_geometry_in_cache(
        cache: &PipelineCache,
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        gs_dxil: &[u8],
        label: &str,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<GraphicsPipeline, String> {
        let (blend, mtl_rtvs, rtv_count) =
            resolve_render_targets_and_blend(&d.blend, &d.rtv_formats, d.sample_count)?;
        if !blend.unsupported_logic_ops.is_empty() {
            log::warn!(
                target: "d3d",
                "geometry PSO: unsupported logic ops {:?} (blending off)",
                blend.unsupported_logic_ops.as_slice()
            );
        }
        let mut vs_opts = CompileOptions::default();
        vs_opts.geometry_tessellation_emulation = true;
        vs_opts.stage_in_mode = StageInMode::SeparateFunction;
        vs_opts.input_topology = Some(crate::topology::input_topology(d.topology_type));
        let vs = compile(ShaderStage::Vertex, d.vs, &rs, &vs_opts)?;
        let StageInfo::Vertex { needs_draw_params, vertex_output_size_in_bytes, inputs, .. } =
            &vs.reflection.info
        else {
            return Err(format!(
                "geometry PSO: vertex shader without vertex reflection: {:?}",
                vs.reflection.info
            ));
        };
        let vertex_output_size = *vertex_output_size_in_bytes;

        let mut layout = d.input_layout.clone();
        dxil_metal::compiler::resolve_append_aligned(
            &mut layout,
            crate::format_ext::vertex_format_size,
        );
        let mut vertex_slots = 0u32;
        for inp in inputs {
            if let Some(e) = layout
                .iter()
                .find(|e| semantic_matches(&inp.name, &e.semantic_name, e.semantic_index))
            {
                if e.input_slot < MAX_VERTEX_SLOTS {
                    vertex_slots |= 1 << e.input_slot;
                }
            } else {
                layout.push(InputElement {
                    semantic_name: inp.name.clone(),
                    semantic_index: 0,
                    format: infer_vertex_input_format(d.vs, &inp.name),
                    input_slot: MAX_VERTEX_SLOTS - 1,
                    aligned_byte_offset: 0,
                    input_slot_class: 1,
                    instance_data_step_rate: 0,
                });
                vertex_slots |= 1 << (MAX_VERTEX_SLOTS - 1);
            }
        }
        let stage_in = if !inputs.is_empty() || !layout.is_empty() {
            let sp = ShaderPipeline::global().map_err(|e| format!("shader pipeline: {e}"))?;
            Some(
                sp.stage_in(&vs, &layout, &vs_opts)
                    .map_err(|e| format!("stage-in synthesis: {e}"))?,
            )
        } else {
            // Even for input-less VS, synthesize stage-in if MSC succeeds (mirrors IRRuntimeNewGeometryEmulationPipeline).
            ShaderPipeline::global().ok().and_then(|sp| sp.stage_in(&vs, &[], &vs_opts).ok())
        };

        let mut gs_opts = CompileOptions::default();
        gs_opts.geometry_tessellation_emulation = true;
        let gs = compile(ShaderStage::Geometry, gs_dxil, &rs, &gs_opts)?;
        let (max_input_prims, gs_instance_count) = match &gs.reflection.info {
            StageInfo::Geometry {
                max_input_primitives_per_mesh_threadgroup,
                instance_count,
                ..
            } => ((*max_input_primitives_per_mesh_threadgroup).max(1), (*instance_count).max(1)),
            other => {
                return Err(format!(
                    "geometry PSO: geometry shader without geometry reflection: {other:?}"
                ));
            }
        };

        let mut ps_opts = crate::state::pixel_compile_options(&blend, &d.rtv_formats);
        let full =
            if d.sample_count >= 32 { u32::MAX } else { (1u32 << d.sample_count.max(1)) - 1 };
        if d.sample_mask & full != full {
            ps_opts.sample_mask = Some(d.sample_mask);
        }
        let ps = match d.ps {
            Some(b) => Some(compile(ShaderStage::Fragment, b, &rs, &ps_opts)?),
            None => None,
        };

        let desc = MTL4MeshRenderPipelineDescriptor::new();
        let vlib = library(&vs)?;
        let obj_name = format!("{}.dxil_irconverter_object_shader", vs.function_name);
        let tess_off: [u8; 1] = [0];
        let obj_fn = specialized_function_descriptor(
            &vlib,
            &obj_name,
            &vs,
            &[("tessellationEnabled", MTLDataType::Bool, &tess_off)],
        );
        desc.setObjectFunctionDescriptor(Some(&obj_fn));

        if let Some(si) = &stage_in {
            let silib = library(si)?;
            let sifn: Retained<MTL4FunctionDescriptor> =
                Retained::into_super(function_descriptor(&silib, &si.function_name));
            let linked = NSArray::from_retained_slice(&[sifn]);
            let linking = MTL4StaticLinkingDescriptor::new();
            linking.setFunctionDescriptors(Some(&linked));
            desc.setObjectStaticLinkingDescriptor(Some(&linking));
        }

        let glib = library(&gs)?;
        let vs_out_bytes = (vertex_output_size as i32).to_ne_bytes();
        let so_off: [u8; 1] = [0];
        let mesh_fn = specialized_function_descriptor(
            &glib,
            &gs.function_name,
            &gs,
            &[
                ("tessellationEnabled", MTLDataType::Bool, &tess_off),
                ("streamOutEnabled", MTLDataType::Bool, &so_off),
                ("vertex_shader_output_size_fc", MTLDataType::Int, &vs_out_bytes),
            ],
        );
        desc.setMeshFunctionDescriptor(Some(&mesh_fn));

        if let Some(ps) = &ps {
            let plib = library(ps)?;
            desc.setFragmentFunctionDescriptor(Some(&shader_function_descriptor(&plib, ps)));
        }

        if !label.is_empty() {
            desc.setLabel(Some(&NSString::from_str(label)));
        }
        desc.setPayloadMemoryLength(16384);
        desc.setMaxTotalThreadgroupsPerMeshGrid(crate::mesh::MAX_MESH_GRID_THREADGROUPS as usize);
        if immut_opt {
            apply_immutable_pipeline_options(&desc);
            desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Disabled);
            desc.setSupportObjectBinaryLinking(false);
            desc.setSupportMeshBinaryLinking(false);
            desc.setSupportFragmentBinaryLinking(false);
            desc.setColorAttachmentMappingState(
                MTL4LogicalToPhysicalColorAttachmentMappingState::Identity,
            );
        }
        if pso_opt_v2 {
            unsafe { desc.setMaxVertexAmplificationCount(1) };
            desc.setAlphaToOneState(MTL4AlphaToOneState::Disabled);
        }

        let ca = desc.colorAttachments();
        apply_color_attachments(&ca, &mtl_rtvs, rtv_count, &blend, pso_opt_v2);
        unsafe { desc.setRasterSampleCount(d.sample_count.max(1) as usize) };
        desc.setAlphaToCoverageState(if blend.alpha_to_coverage {
            MTL4AlphaToCoverageState::Enabled
        } else {
            MTL4AlphaToCoverageState::Disabled
        });

        let mut key = dxil_metal::hash::KeyHasher::new("d3d12-metal gs-emu pso v2");
        key.u128(vs.key)
            .u128(gs.key)
            .u128(stage_in.as_ref().map_or(0, |s| s.key))
            .u128(ps.as_ref().map_or(0, |p| p.key))
            .u32(vertex_output_size);
        RenderPsoStateKey::new(blend, &d.rtv_formats, d.sample_count, d.topology_type)
            .write_to(&mut key);
        let mut pso_key = key.finish();
        if immut_opt {
            pso_key ^= IMMUTABLE_OPT_KEY_SALT;
        }
        if pso_opt_v2 {
            pso_key ^= SHADER_PSO_OPT_V2_SALT;
        }
        let pso = cache
            .render_pipeline(Some(pso_key), &desc)
            .map_err(|e| format!("geometry render pipeline: {e}"))?;
        let ds = if d.dsv_format == 0 {
            DepthStencilState::without_depth_stencil()
        } else {
            d.depth_stencil
        };
        let vs_sum = ShaderStageSummary::new(d.vs, &vs, &rs);
        let gs_sum = ShaderStageSummary::new(gs_dxil, &gs, &rs);
        let ps_sum = ps.as_ref().zip(d.ps).map(|(p, b)| (p, b, ShaderStageSummary::new(b, p, &rs)));
        let pre_raster_reads = vs_sum.stage_reads_resources(&vs, pso_opt_v2)
            | gs_sum.stage_reads_resources(&gs, pso_opt_v2);
        let vs_writes_uav = vs_sum.shader_writes_uav(true) || gs_sum.shader_writes_uav(true);
        let ps_writes_uav = ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_uav(true));
        let writes_uav = vs_writes_uav || ps_writes_uav;
        let (ps_srv_uav_classes, ps_read_classes) = match &ps_sum {
            Some((p, _, s)) => {
                let srv_uav = s.stage_srv_uav_classes(p, &rs, true);
                let mut m = srv_uav;
                if s.stage_may_read_cbv(&rs, true) {
                    m |= READS_BUF;
                }
                (srv_uav, m)
            }
            None => (0, 0),
        };
        let uses_uav = writes_uav
            || vs_sum.shader_uses_uav(&rs)
            || gs_sum.shader_uses_uav(&rs)
            || ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_uses_uav(&rs));
        let (vs_access, vs_hk_profile) = vs_sum.for_stage(d.vs, &rs, &vs, true);
        let (ps_access, ps_hk_profile) = match &ps_sum {
            Some((p, b, s)) => {
                let (acc, hk) = s.for_stage(b, &rs, p, false);
                (Some(acc), Some(hk))
            }
            None => (None, None),
        };
        Ok(GraphicsPipeline {
            pso,
            rs,
            raster: d.raster,
            blend,
            depth_stencil: depth_stencil_state(&ds)?,
            depth_bounds: ds.depth_bounds,
            needs_draw_params: *needs_draw_params,
            topology_type: d.topology_type,
            strip_cut: d.strip_cut,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count.max(1),
            vertex_slots,
            pre_raster_reads,
            vs_reads_cbv_or_ia: true,
            writes_uav,
            sm66_pre_raster_reads: pre_raster_reads,
            sm66_vs_reads_cbv_or_ia: true,
            sm66_writes_uav: writes_uav,
            sm66_narrowed: false,
            vs_writes_uav,
            ps_writes_uav,
            ps_read_classes,
            ps_srv_uav_classes,
            uses_uav,
            vs_access,
            ps_access,
            vs_hk_profile,
            ps_hk_profile,
            vs,
            ps,
            gs_emu: Some(GeometryEmulation {
                gs,
                stage_in,
                vertex_output_size_in_bytes: vertex_output_size,
                max_input_primitives_per_mesh_threadgroup: max_input_prims,
                instance_count: gs_instance_count,
            }),
            tess_emu: None,
            icb_desc: None,
            icb_pso: OnceLock::new(),
        })
    }

    /// Build a graphics pipeline that emulates D3D11/D3D12 Hull + Domain tessellation
    /// (with an optional Geometry Shader) using Apple Metal Shader Converter's Metal 4
    /// object+mesh tessellation emulation pipeline.
    pub fn new_with_tessellation(
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        hs_dxil: &[u8],
        ds_dxil: &[u8],
        gs_dxil: Option<&[u8]>,
        label: &str,
    ) -> Result<GraphicsPipeline, String> {
        let immut_opt = shader_immutable_opt();
        let pso_opt_v2 = shader_pso_opt_v2();
        let rs_for_manifest = rs.clone();
        let out = Self::new_tessellation_in_cache(
            pso_cache()?,
            rs,
            d,
            hs_dxil,
            ds_dxil,
            gs_dxil,
            label,
            immut_opt,
            pso_opt_v2,
        )?;
        crate::metal::pso_manifest::note_tessellation(
            &rs_for_manifest,
            d,
            hs_dxil,
            ds_dxil,
            gs_dxil,
            immut_opt,
            pso_opt_v2,
        );
        Ok(out)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_tessellation_in_cache(
        cache: &PipelineCache,
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        hs_dxil: &[u8],
        ds_dxil: &[u8],
        gs_dxil: Option<&[u8]>,
        label: &str,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<GraphicsPipeline, String> {
        let (blend, mtl_rtvs, rtv_count) =
            resolve_render_targets_and_blend(&d.blend, &d.rtv_formats, d.sample_count)?;
        if !blend.unsupported_logic_ops.is_empty() {
            log::warn!(
                target: "d3d",
                "tessellation PSO: unsupported logic ops {:?} (blending off)",
                blend.unsupported_logic_ops.as_slice()
            );
        }
        let mut vs_opts = CompileOptions::default();
        vs_opts.geometry_tessellation_emulation = true;
        vs_opts.stage_in_mode = StageInMode::SeparateFunction;
        vs_opts.input_topology = Some(dxil_metal::InputTopology::Patch);
        let vs = compile(ShaderStage::Vertex, d.vs, &rs, &vs_opts)?;
        let StageInfo::Vertex { needs_draw_params, vertex_output_size_in_bytes, inputs, .. } =
            &vs.reflection.info
        else {
            return Err(format!(
                "tessellation PSO: vertex shader without vertex reflection: {:?}",
                vs.reflection.info
            ));
        };
        let vertex_output_size = *vertex_output_size_in_bytes;

        let mut layout = d.input_layout.clone();
        dxil_metal::compiler::resolve_append_aligned(
            &mut layout,
            crate::format_ext::vertex_format_size,
        );
        let mut vertex_slots = 0u32;
        for inp in inputs {
            if let Some(e) = layout
                .iter()
                .find(|e| semantic_matches(&inp.name, &e.semantic_name, e.semantic_index))
            {
                if e.input_slot < MAX_VERTEX_SLOTS {
                    vertex_slots |= 1 << e.input_slot;
                }
            } else {
                layout.push(InputElement {
                    semantic_name: inp.name.clone(),
                    semantic_index: 0,
                    format: infer_vertex_input_format(d.vs, &inp.name),
                    input_slot: MAX_VERTEX_SLOTS - 1,
                    aligned_byte_offset: 0,
                    input_slot_class: 1,
                    instance_data_step_rate: 0,
                });
                vertex_slots |= 1 << (MAX_VERTEX_SLOTS - 1);
            }
        }
        let stage_in = if !inputs.is_empty() || !layout.is_empty() {
            let sp = ShaderPipeline::global().map_err(|e| format!("shader pipeline: {e}"))?;
            Some(
                sp.stage_in(&vs, &layout, &vs_opts)
                    .map_err(|e| format!("stage-in synthesis: {e}"))?,
            )
        } else {
            ShaderPipeline::global().ok().and_then(|sp| sp.stage_in(&vs, &[], &vs_opts).ok())
        };

        let mut hs_opts = CompileOptions::default();
        hs_opts.geometry_tessellation_emulation = true;
        hs_opts.input_topology = Some(dxil_metal::InputTopology::Patch);
        let hs = compile(ShaderStage::Hull, hs_dxil, &rs, &hs_opts)?;
        let StageInfo::Hull {
            max_patches_per_object_threadgroup,
            max_object_threads_per_patch,
            input_control_point_count,
            tessellator_output_primitive,
            max_tessellation_factor,
            ..
        } = &hs.reflection.info
        else {
            return Err(format!(
                "tessellation PSO: hull shader without hull reflection: {:?}",
                hs.reflection.info
            ));
        };
        let hs_max_patches = (*max_patches_per_object_threadgroup).max(1);
        let hs_threads_per_patch = (*max_object_threads_per_patch).max(1);
        let hs_input_cps = (*input_control_point_count).max(1);
        let output_primitive = *tessellator_output_primitive;
        let max_tess_factor = if max_tessellation_factor.is_finite() && *max_tessellation_factor > 0.0 {
            *max_tessellation_factor
        } else {
            64.0
        };

        let mut ds_opts = CompileOptions::default();
        ds_opts.geometry_tessellation_emulation = true;
        let ds = compile(ShaderStage::Domain, ds_dxil, &rs, &ds_opts)?;
        let ds_max_input_prims = match &ds.reflection.info {
            StageInfo::Domain { max_input_prims_per_mesh_threadgroup, .. } => {
                (*max_input_prims_per_mesh_threadgroup).max(1)
            }
            other => {
                return Err(format!(
                    "tessellation PSO: domain shader without domain reflection: {other:?}"
                ));
            }
        };

        let (gs, gs_max_input_prims, gs_instance_count) = match gs_dxil {
            Some(gb) => {
                let mut gs_opts = CompileOptions::default();
                gs_opts.geometry_tessellation_emulation = true;
                let g = compile(ShaderStage::Geometry, gb, &rs, &gs_opts)?;
                let (mp, ic) = match &g.reflection.info {
                    StageInfo::Geometry {
                        max_input_primitives_per_mesh_threadgroup,
                        instance_count,
                        ..
                    } => ((*max_input_primitives_per_mesh_threadgroup).max(1), (*instance_count).max(1)),
                    other => {
                        return Err(format!(
                            "tessellation PSO: geometry shader without geometry reflection: {other:?}"
                        ));
                    }
                };
                (Some(g), mp, ic)
            }
            None => (None, ds_max_input_prims, 1),
        };

        let mut ps_opts = crate::state::pixel_compile_options(&blend, &d.rtv_formats);
        let full =
            if d.sample_count >= 32 { u32::MAX } else { (1u32 << d.sample_count.max(1)) - 1 };
        if d.sample_mask & full != full {
            ps_opts.sample_mask = Some(d.sample_mask);
        }
        let ps = match d.ps {
            Some(b) => Some(compile(ShaderStage::Fragment, b, &rs, &ps_opts)?),
            None => None,
        };

        let desc = MTL4MeshRenderPipelineDescriptor::new();
        let vlib = library(&vs)?;
        let hlib = library(&hs)?;
        let dlib = library(&ds)?;

        // Object stage: VS wrapper specialized with tessellationEnabled = true,
        // statically linked with stage-in function + irconverter_hull_shader.
        let tess_on: [u8; 1] = [1];
        let obj_name = format!("{}.dxil_irconverter_object_shader", vs.function_name);
        let obj_fn = specialized_function_descriptor(
            &vlib,
            &obj_name,
            &vs,
            &[("tessellationEnabled", MTLDataType::Bool, &tess_on)],
        );
        desc.setObjectFunctionDescriptor(Some(&obj_fn));

        let vs_out_bytes = (vertex_output_size as i32).to_ne_bytes();
        let max_tess_bytes = max_tess_factor.to_ne_bytes();
        let hull_fn = specialized_function_descriptor(
            &hlib,
            "irconverter_hull_shader",
            &hs,
            &[
                ("vertex_shader_output_size_fc", MTLDataType::Int, &vs_out_bytes),
                ("max_tessellation_factor_fc", MTLDataType::Float, &max_tess_bytes),
            ],
        );
        let mut obj_linked: Vec<Retained<MTL4FunctionDescriptor>> = Vec::with_capacity(2);
        if let Some(si) = &stage_in {
            let silib = library(si)?;
            obj_linked.push(Retained::into_super(function_descriptor(&silib, &si.function_name)));
        }
        obj_linked.push(hull_fn);
        let obj_linking = MTL4StaticLinkingDescriptor::new();
        obj_linking.setFunctionDescriptors(Some(&NSArray::from_retained_slice(&obj_linked)));
        desc.setObjectStaticLinkingDescriptor(Some(&obj_linking));

        // Mesh stage: either real GS (tessellationEnabled = true) or MSC's passthrough
        // geometry shader inside domainLibrary, statically linked with irconverter_tessellator
        // + irconverter_dxil_domain_shader.
        let mesh_fn = if let Some(g) = &gs {
            let glib = library(g)?;
            let so_off: [u8; 1] = [0];
            specialized_function_descriptor(
                &glib,
                &g.function_name,
                g,
                &[
                    ("tessellationEnabled", MTLDataType::Bool, &tess_on),
                    ("streamOutEnabled", MTLDataType::Bool, &so_off),
                    ("vertex_shader_output_size_fc", MTLDataType::Int, &vs_out_bytes),
                ],
            )
        } else {
            let passthrough_name = match output_primitive {
                1 => dxil_metal::runtime::POINT_PASSTHROUGH_GEOMETRY_SHADER,
                2 => dxil_metal::runtime::LINE_PASSTHROUGH_GEOMETRY_SHADER,
                _ => dxil_metal::runtime::TRIANGLE_PASSTHROUGH_GEOMETRY_SHADER,
            };
            Retained::into_super(function_descriptor(&dlib, passthrough_name))
        };
        desc.setMeshFunctionDescriptor(Some(&mesh_fn));

        let tess_fn = specialized_function_descriptor(
            &hlib,
            "irconverter_tessellator",
            &hs,
            &[
                ("vertex_shader_output_size_fc", MTLDataType::Int, &vs_out_bytes),
                ("max_tessellation_factor_fc", MTLDataType::Float, &max_tess_bytes),
            ],
        );
        let domain_fn = specialized_function_descriptor(
            &dlib,
            "irconverter_dxil_domain_shader",
            &ds,
            &[],
        );
        let mesh_linked = NSArray::from_retained_slice(&[tess_fn, domain_fn]);
        let mesh_linking = MTL4StaticLinkingDescriptor::new();
        mesh_linking.setFunctionDescriptors(Some(&mesh_linked));
        desc.setMeshStaticLinkingDescriptor(Some(&mesh_linking));

        if let Some(ps) = &ps {
            let plib = library(ps)?;
            desc.setFragmentFunctionDescriptor(Some(&shader_function_descriptor(&plib, ps)));
        }

        if !label.is_empty() {
            desc.setLabel(Some(&NSString::from_str(label)));
        }
        desc.setPayloadMemoryLength(16384);
        desc.setMaxTotalThreadgroupsPerMeshGrid(crate::mesh::MAX_MESH_GRID_THREADGROUPS as usize);
        if immut_opt {
            apply_immutable_pipeline_options(&desc);
            desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Disabled);
            desc.setSupportObjectBinaryLinking(false);
            desc.setSupportMeshBinaryLinking(false);
            desc.setSupportFragmentBinaryLinking(false);
            desc.setColorAttachmentMappingState(
                MTL4LogicalToPhysicalColorAttachmentMappingState::Identity,
            );
        }
        if pso_opt_v2 {
            unsafe { desc.setMaxVertexAmplificationCount(1) };
            desc.setAlphaToOneState(MTL4AlphaToOneState::Disabled);
        }

        let ca = desc.colorAttachments();
        apply_color_attachments(&ca, &mtl_rtvs, rtv_count, &blend, pso_opt_v2);
        unsafe { desc.setRasterSampleCount(d.sample_count.max(1) as usize) };
        desc.setAlphaToCoverageState(if blend.alpha_to_coverage {
            MTL4AlphaToCoverageState::Enabled
        } else {
            MTL4AlphaToCoverageState::Disabled
        });

        let mut key = dxil_metal::hash::KeyHasher::new("d3d12-metal tess-emu pso v1");
        key.u128(vs.key)
            .u128(hs.key)
            .u128(ds.key)
            .u128(gs.as_ref().map_or(0, |g| g.key))
            .u128(stage_in.as_ref().map_or(0, |s| s.key))
            .u128(ps.as_ref().map_or(0, |p| p.key))
            .u32(vertex_output_size)
            .u32(output_primitive)
            .u32(max_tess_factor.to_bits());
        RenderPsoStateKey::new(blend, &d.rtv_formats, d.sample_count, d.topology_type)
            .write_to(&mut key);
        let mut pso_key = key.finish();
        if immut_opt {
            pso_key ^= IMMUTABLE_OPT_KEY_SALT;
        }
        if pso_opt_v2 {
            pso_key ^= SHADER_PSO_OPT_V2_SALT;
        }
        let pso = cache
            .render_pipeline(Some(pso_key), &desc)
            .map_err(|e| format!("tessellation render pipeline: {e}"))?;

        let ds_state = if d.dsv_format == 0 {
            DepthStencilState::without_depth_stencil()
        } else {
            d.depth_stencil
        };
        let vs_sum = ShaderStageSummary::new(d.vs, &vs, &rs);
        let hs_sum = ShaderStageSummary::new(hs_dxil, &hs, &rs);
        let ds_sum = ShaderStageSummary::new(ds_dxil, &ds, &rs);
        let gs_sum = gs.as_ref().zip(gs_dxil).map(|(g, b)| (g, ShaderStageSummary::new(b, g, &rs)));
        let ps_sum = ps.as_ref().zip(d.ps).map(|(p, b)| (p, b, ShaderStageSummary::new(b, p, &rs)));
        let pre_raster_reads = vs_sum.stage_reads_resources(&vs, pso_opt_v2)
            | hs_sum.stage_reads_resources(&hs, pso_opt_v2)
            | ds_sum.stage_reads_resources(&ds, pso_opt_v2)
            | gs_sum.as_ref().map_or(0, |(g, s)| s.stage_reads_resources(g, pso_opt_v2));
        let vs_writes_uav = vs_sum.shader_writes_uav(true)
            || hs_sum.shader_writes_uav(true)
            || ds_sum.shader_writes_uav(true)
            || gs_sum.as_ref().is_some_and(|(_, s)| s.shader_writes_uav(true));
        let ps_writes_uav = ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_uav(true));
        let writes_uav = vs_writes_uav || ps_writes_uav;
        let (ps_srv_uav_classes, ps_read_classes) = match &ps_sum {
            Some((p, _, s)) => {
                let srv_uav = s.stage_srv_uav_classes(p, &rs, true);
                let mut m = srv_uav;
                if s.stage_may_read_cbv(&rs, true) {
                    m |= READS_BUF;
                }
                (srv_uav, m)
            }
            None => (0, 0),
        };
        let uses_uav = writes_uav
            || vs_sum.shader_uses_uav(&rs)
            || hs_sum.shader_uses_uav(&rs)
            || ds_sum.shader_uses_uav(&rs)
            || gs_sum.as_ref().is_some_and(|(_, s)| s.shader_uses_uav(&rs))
            || ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_uses_uav(&rs));
        let (vs_access, vs_hk_profile) = vs_sum.for_stage(d.vs, &rs, &vs, true);
        let (ps_access, ps_hk_profile) = match &ps_sum {
            Some((p, b, s)) => {
                let (acc, hk) = s.for_stage(b, &rs, p, false);
                (Some(acc), Some(hk))
            }
            None => (None, None),
        };
        Ok(GraphicsPipeline {
            pso,
            rs,
            raster: d.raster,
            blend,
            depth_stencil: depth_stencil_state(&ds_state)?,
            depth_bounds: ds_state.depth_bounds,
            needs_draw_params: *needs_draw_params,
            topology_type: d.topology_type,
            strip_cut: d.strip_cut,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count.max(1),
            vertex_slots,
            pre_raster_reads,
            vs_reads_cbv_or_ia: true,
            writes_uav,
            sm66_pre_raster_reads: pre_raster_reads,
            sm66_vs_reads_cbv_or_ia: true,
            sm66_writes_uav: writes_uav,
            sm66_narrowed: false,
            vs_writes_uav,
            ps_writes_uav,
            ps_read_classes,
            ps_srv_uav_classes,
            uses_uav,
            vs_access,
            ps_access,
            vs_hk_profile,
            ps_hk_profile,
            vs,
            ps,
            gs_emu: None,
            tess_emu: Some(TessellationEmulation {
                hs,
                ds,
                gs,
                stage_in,
                output_primitive,
                vs_output_size_in_bytes: vertex_output_size,
                gs_max_input_primitives_per_mesh_threadgroup: gs_max_input_prims,
                hs_max_patches_per_object_threadgroup: hs_max_patches,
                hs_input_control_point_count: hs_input_cps,
                hs_max_object_threads_per_patch: hs_threads_per_patch,
                hs_max_tessellation_factor: max_tess_factor,
                gs_instance_count,
            }),
            icb_desc: None,
            icb_pso: OnceLock::new(),
        })
    }

    pub fn new_with_opts(
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        label: &str,
        lazy_icb: bool,
    ) -> Result<GraphicsPipeline, String> {
        Self::new_with_pso_opts(
            rs,
            d,
            label,
            lazy_icb,
            shader_immutable_opt(),
            shader_pso_opt_v2(),
        )
    }

    pub fn new_with_all_opts(
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        label: &str,
        lazy_icb: bool,
        immut_opt: bool,
    ) -> Result<GraphicsPipeline, String> {
        Self::new_with_pso_opts(rs, d, label, lazy_icb, immut_opt, shader_pso_opt_v2())
    }

    pub fn new_with_pso_opts(
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        label: &str,
        lazy_icb: bool,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<GraphicsPipeline, String> {
        let rs_for_manifest = rs.clone();
        let out = Self::new_in_cache(pso_cache()?, rs, d, label, lazy_icb, immut_opt, pso_opt_v2)?;
        crate::metal::pso_manifest::note_graphics(
            &rs_for_manifest,
            d,
            lazy_icb,
            immut_opt,
            pso_opt_v2,
            out.icb_desc.as_ref().map(|(_, k)| *k),
        );
        Ok(out)
    }

    pub(crate) fn new_in_cache(
        cache: &PipelineCache,
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        label: &str,
        lazy_icb: bool,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<GraphicsPipeline, String> {
        let (blend, mtl_rtvs, rtv_count) =
            resolve_render_targets_and_blend(&d.blend, &d.rtv_formats, d.sample_count)?;
        if !blend.unsupported_logic_ops.is_empty() {
            log::warn!(
                target: "d3d",
                "graphics PSO: unsupported logic ops {:?} (blending off)",
                blend.unsupported_logic_ops.as_slice()
            );
        }
        let vs = compile(ShaderStage::Vertex, d.vs, &rs, &CompileOptions::default())?;
        let StageInfo::Vertex { needs_draw_params, inputs, .. } = &vs.reflection.info else {
            return Err(format!("vertex shader without vertex reflection: {:?}", vs.reflection.info));
        };
        let mut ps_opts = crate::state::pixel_compile_options(&blend, &d.rtv_formats);
        let full =
            if d.sample_count >= 32 { u32::MAX } else { (1u32 << d.sample_count.max(1)) - 1 };
        if d.sample_mask & full != full {
            ps_opts.sample_mask = Some(d.sample_mask);
        }
        let ps = match d.ps {
            Some(b) => Some(compile(ShaderStage::Fragment, b, &rs, &ps_opts)?),
            None => None,
        };

        let desc = MTL4RenderPipelineDescriptor::new();
        let vlib = library(&vs)?;
        desc.setVertexFunctionDescriptor(Some(&shader_function_descriptor(&vlib, &vs)));
        if let Some(ps) = &ps {
            let plib = library(ps)?;
            desc.setFragmentFunctionDescriptor(Some(&shader_function_descriptor(&plib, ps)));
        }
        if !label.is_empty() {
            desc.setLabel(Some(&NSString::from_str(label)));
        } else if let Some(l) = crate::metal::gpucapture::pipeline_label(
            label,
            "gfx",
            vs.key as u64 ^ ps.as_ref().map_or(0, |s| (s.key as u64).rotate_left(1)),
        ) {
            desc.setLabel(Some(&NSString::from_str(&l)));
        }
        if immut_opt {
            apply_immutable_pipeline_options(&desc);
            desc.setSupportVertexBinaryLinking(false);
            desc.setSupportFragmentBinaryLinking(false);
            desc.setColorAttachmentMappingState(
                MTL4LogicalToPhysicalColorAttachmentMappingState::Identity,
            );
        }
        if pso_opt_v2 {
            unsafe { desc.setMaxVertexAmplificationCount(1) };
            desc.setAlphaToOneState(MTL4AlphaToOneState::Disabled);
        }

        // Vertex fetch: match the layout's elements to the shader's inputs.
        // If a PSO's input layout omits a declared VS input (e.g. an unused attribute or a
        // synthesized default layout), bind a constant step-rate fallback on slot MAX_VERTEX_SLOTS - 1
        // with the scalar format inferred from the DXBC input signature.
        let mut layout = d.input_layout.clone();
        dxil_metal::compiler::resolve_append_aligned(
            &mut layout,
            crate::format_ext::vertex_format_size,
        );
        desc.setSupportIndirectCommandBuffers(if lazy_icb {
            MTL4IndirectCommandBufferSupportState::Disabled
        } else {
            MTL4IndirectCommandBufferSupportState::Enabled
        });
        let mut key = dxil_metal::hash::KeyHasher::new("d3d12-metal graphics pso v3");
        key.u128(vs.key).u128(ps.as_ref().map_or(0, |p| p.key));
        let mut vertex_slots = 0u32;
        if !inputs.is_empty() {
            let vd = MTLVertexDescriptor::new();
            for inp in inputs {
                let fallback;
                let e = match layout
                    .iter()
                    .find(|e| semantic_matches(&inp.name, &e.semantic_name, e.semantic_index))
                {
                    Some(e) => e,
                    None => {
                        fallback = InputElement {
                            semantic_name: inp.name.clone(),
                            semantic_index: 0,
                            format: infer_vertex_input_format(d.vs, &inp.name),
                            input_slot: MAX_VERTEX_SLOTS - 1,
                            aligned_byte_offset: 0,
                            input_slot_class: 1,
                            instance_data_step_rate: 0,
                        };
                        &fallback
                    }
                };
                if e.input_slot >= MAX_VERTEX_SLOTS {
                    return Err(format!(
                        "input slot {} (max {})",
                        e.input_slot,
                        MAX_VERTEX_SLOTS - 1
                    ));
                }
                let fmt = crate::format_ext::vertex_format(e.format).ok_or_else(|| {
                    format!("vertex format {} ({})", e.format, crate::format::info(e.format).name)
                })?;
                let attr_idx = dxil_metal::runtime::STAGE_IN_ATTRIBUTE_START_INDEX as usize
                    + inp.attribute_index as usize;
                if attr_idx >= MAX_VERTEX_ATTRIBUTES {
                    return Err(format!(
                        "vertex attribute index {attr_idx} (max {})",
                        MAX_VERTEX_ATTRIBUTES - 1
                    ));
                }
                let a = unsafe { vd.attributes().objectAtIndexedSubscript(attr_idx) };
                a.setFormat(fmt);
                unsafe {
                    a.setOffset(e.aligned_byte_offset as usize);
                    a.setBufferIndex(vertex_buffer_index(e.input_slot));
                }
                let l = unsafe {
                    vd.layouts().objectAtIndexedSubscript(vertex_buffer_index(e.input_slot))
                };
                unsafe { l.setStride(usize::MAX) }; // MTLBufferLayoutStrideDynamic
                if e.input_slot_class == 1 {
                    if e.instance_data_step_rate == 0 {
                        l.setStepFunction(MTLVertexStepFunction::Constant);
                        unsafe { l.setStepRate(0) };
                    } else {
                        l.setStepFunction(MTLVertexStepFunction::PerInstance);
                        unsafe { l.setStepRate(e.instance_data_step_rate as usize) };
                    }
                }
                vertex_slots |= 1 << e.input_slot;
                key.u32(inp.attribute_index as u32)
                    .u32(e.format)
                    .u32(e.input_slot)
                    .u32(e.aligned_byte_offset)
                    .u32(e.input_slot_class)
                    .u32(e.instance_data_step_rate);
            }
            desc.setVertexDescriptor(Some(&vd));
        }

        let ca = desc.colorAttachments();
        apply_color_attachments(&ca, &mtl_rtvs, rtv_count, &blend, pso_opt_v2);
        unsafe { desc.setRasterSampleCount(d.sample_count.max(1) as usize) };
        desc.setAlphaToCoverageState(if blend.alpha_to_coverage {
            MTL4AlphaToCoverageState::Enabled
        } else {
            MTL4AlphaToCoverageState::Disabled
        });
        desc.setInputPrimitiveTopology(crate::topology::topology_class(d.topology_type));
        RenderPsoStateKey::new(blend, &d.rtv_formats, d.sample_count, d.topology_type)
            .write_to(&mut key);

        let mut base_key = key.finish();
        if immut_opt {
            base_key ^= IMMUTABLE_OPT_KEY_SALT;
        }
        if pso_opt_v2 {
            base_key ^= SHADER_PSO_OPT_V2_SALT;
        }
        let primary_key = if lazy_icb { base_key ^ 0x4e4f_5f49_4342_5f31_u128 } else { base_key };
        let pso = cache
            .render_pipeline(Some(primary_key), &desc)
            .map_err(|e| format!("render pipeline: {e}"))?;
        let icb_desc = lazy_icb.then_some((desc, base_key));
        let ds = if d.dsv_format == 0 {
            DepthStencilState::without_depth_stencil()
        } else {
            d.depth_stencil
        };
        if ds.depth_bounds {
            log::warn!(
                target: "d3d",
                "graphics PSO enables the depth bounds test, which is reported unsupported; ignored"
            );
        }
        let vs_sum = ShaderStageSummary::new(d.vs, &vs, &rs);
        let ps_sum = ps.as_ref().zip(d.ps).map(|(p, b)| (p, b, ShaderStageSummary::new(b, p, &rs)));
        let msc_usage = msc_stage_usage();
        let sm66_gfx = sm66_gfx_stage_exact();
        let base_use_msc = pso_opt_v2 || msc_usage;
        let dxil_pre_raster = vs_sum.stage_reads_resources(&vs, false);
        let base_pre_raster = if base_use_msc {
            vs_sum.stage_reads_resources(&vs, true)
        } else {
            dxil_pre_raster
        };
        let sm66_pre_raster_reads = vs_sum.stage_srv_uav_classes(&vs, &rs, true);
        let dxil_vs_cbv = vertex_slots != 0 || vs_sum.stage_may_read_cbv(&rs, false);
        let base_vs_cbv = dxil_vs_cbv;
        let sm66_vs_reads_cbv_or_ia = vertex_slots != 0 || vs_sum.stage_may_read_cbv(&rs, true);
        let dxil_writes_uav = vs_sum.shader_writes_uav(false)
            || ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_uav(false));
        let vs_writes_uav = vs_sum.shader_writes_uav(true);
        let ps_writes_uav = ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_uav(true));
        let base_writes_uav = if base_use_msc {
            vs_writes_uav || ps_writes_uav
        } else {
            dxil_writes_uav
        };
        if ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_writes_buf_uav()) {
            note_frag_buf_write();
        }
        let sm66_writes_uav = vs_writes_uav || ps_writes_uav;
        let pre_raster_reads = if sm66_gfx { sm66_pre_raster_reads } else { base_pre_raster };
        let vs_reads_cbv_or_ia = if sm66_gfx { sm66_vs_reads_cbv_or_ia } else { base_vs_cbv };
        let writes_uav = if sm66_gfx { sm66_writes_uav } else { base_writes_uav };
        let (ps_srv_uav_classes, ps_read_classes) = match &ps_sum {
            Some((p, _, s)) => {
                let srv_uav = s.stage_srv_uav_classes(p, &rs, true);
                let mut m = srv_uav;
                if s.stage_may_read_cbv(&rs, true) {
                    m |= READS_BUF;
                }
                (srv_uav, m)
            }
            None => (0, 0),
        };
        let uses_uav = writes_uav
            || vs_sum.shader_uses_uav(&rs)
            || ps_sum.as_ref().is_some_and(|(_, _, s)| s.shader_uses_uav(&rs));
        let both_classes = READS_TEX | READS_BUF;
        let msc_proven =
            vs_sum.msc_proven() && ps_sum.as_ref().is_none_or(|(_, _, s)| s.msc_proven());
        let sm66_narrowed = sm66_pre_raster_reads != dxil_pre_raster
            || sm66_vs_reads_cbv_or_ia != dxil_vs_cbv
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
        let (vs_access, vs_hk_profile) = vs_sum.for_stage(d.vs, &rs, &vs, true);
        let (ps_access, ps_hk_profile) = match &ps_sum {
            Some((p, b, s)) => {
                let (acc, hk) = s.for_stage(b, &rs, p, false);
                (Some(acc), Some(hk))
            }
            None => (None, None),
        };
        Ok(GraphicsPipeline {
            pso,
            rs,
            raster: d.raster,
            blend,
            depth_stencil: depth_stencil_state(&ds)?,
            depth_bounds: ds.depth_bounds,
            needs_draw_params: *needs_draw_params,
            topology_type: d.topology_type,
            strip_cut: d.strip_cut,
            rtv_formats: d.rtv_formats.clone(),
            dsv_format: d.dsv_format,
            sample_count: d.sample_count.max(1),
            vertex_slots,
            pre_raster_reads,
            vs_reads_cbv_or_ia,
            writes_uav,
            sm66_pre_raster_reads,
            sm66_vs_reads_cbv_or_ia,
            sm66_writes_uav,
            sm66_narrowed,
            vs_writes_uav,
            ps_writes_uav,
            ps_read_classes,
            ps_srv_uav_classes,
            uses_uav,
            vs_access,
            ps_access,
            vs_hk_profile,
            ps_hk_profile,
            vs: vs.clone(),
            ps,
            gs_emu: None,
            tess_emu: None,
            icb_desc,
            icb_pso: OnceLock::new(),
        })
    }

    /// Returns an ICB-capable `MTLRenderPipelineState` (`supportIndirectCommandBuffers == Enabled`),
    /// compiling and caching it on first demand when `AW2_GFX_LAZY_ICB=1` is active.
    pub fn icb_pso(&self) -> Result<&MtlRenderPipeline, String> {
        let Some((desc, icb_key)) = &self.icb_desc else {
            return Ok(&self.pso);
        };
        self.icb_pso
            .get_or_init(|| {
                desc.setSupportIndirectCommandBuffers(
                    MTL4IndirectCommandBufferSupportState::Enabled,
                );
                let pso = pso_cache()?
                    .render_pipeline(Some(*icb_key), desc)
                    .map_err(|e| format!("render pipeline (icb): {e}"))?;
                crate::metal::pso_manifest::note_graphics_icb(*icb_key);
                Ok(pso)
            })
            .as_deref()
            .map_err(Clone::clone)
    }

    /// Prewarms the ICB-capable `MTLRenderPipelineState` variant directly into `cache`
    /// during cold-device shader prewarming so `icb_pso()` never compiles on demand mid-gameplay.
    pub(crate) fn prewarm_icb_in_cache(&self, cache: &PipelineCache) -> Result<(), String> {
        let Some((desc, icb_key)) = &self.icb_desc else {
            return Ok(());
        };
        desc.setSupportIndirectCommandBuffers(MTL4IndirectCommandBufferSupportState::Enabled);
        cache
            .render_pipeline(Some(*icb_key), desc)
            .map(|_| ())
            .map_err(|e| format!("render pipeline (icb prewarm): {e}"))
    }
}
