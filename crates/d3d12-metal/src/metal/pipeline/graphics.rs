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
        Self::new_geometry_in_cache(
            pso_cache()?,
            rs,
            d,
            gs_dxil,
            label,
            shader_immutable_opt(),
            shader_pso_opt_v2(),
        )
    }

    fn new_geometry_in_cache(
        cache: &PipelineCache,
        rs: Arc<RootSignature>,
        d: &GraphicsDesc,
        gs_dxil: &[u8],
        label: &str,
        immut_opt: bool,
        pso_opt_v2: bool,
    ) -> Result<GraphicsPipeline, String> {
        let blend = crate::state::blend_state(&d.blend);
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
        let blend = crate::state::blend_state(&d.blend);
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
                let a = unsafe {
                    vd.attributes().objectAtIndexedSubscript(
                        dxil_metal::runtime::STAGE_IN_ATTRIBUTE_START_INDEX as usize
                            + inp.attribute_index as usize,
                    )
                };
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
                pso_cache()?
                    .render_pipeline(Some(*icb_key), desc)
                    .map_err(|e| format!("render pipeline (icb): {e}"))
            })
            .as_deref()
            .map_err(Clone::clone)
    }
}
