//! End-to-end Metal 4 PSO compilation test against real Alan Wake 2 shaders:
//!
//! 1. **PE-embedded shaders** extracted from `AlanWake2.exe`, `NRD.dll`, and
//!    `amd_fidelityfx_upscaler_dx12.dll` (`dxil_metal::pe::extract_pe_shaders`),
//!    compiled through [`ComputePipeline::new`] and [`GraphicsPipeline::new`].
//! 2. **Shipped `.binrfx` shader bundles** in `data/shaders/build/pc_dx12/` covering:
//!    - Compute PSOs (`cs_6_*`) via [`ComputePipeline::new`]
//!    - Traditional graphics PSOs (`vs_6_*` + `ps_6_*` color passes and VS-only depth passes,
//!      both with explicit `InputElement` layouts and with synthesized fallback attributes)
//!      via [`GraphicsPipeline::new`]
//!    - Mesh shader PSOs (`ms_6_*` + `ps_6_*` color passes and MS-only depth passes)
//!      via [`MeshPipeline::new`]
//!
//! Skips gracefully when the Metal 4 device, Metal Shader Converter, or game directory
//! is unavailable.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use d3d12_metal::mesh::{MeshDesc, MeshPipeline};
use d3d12_metal::metal::pipeline::{infer_vertex_input_format, ComputePipeline, GraphicsDesc, GraphicsPipeline};
use d3d12_metal::state::{BlendDesc, DepthStencilState, RasterState};
use aw2_data::{discover_game_dir, BinRfx};
use dxil_metal::dxbc::{Container, ProgramKind, ResourceBinding};
use dxil_metal::pe::extract_pe_shaders;
use dxil_metal::root_signature::{flags, DescriptorRangeType, RootSignature, RootSignatureDesc};
use dxil_metal::{converter_available, dxilconv, InputElement};
use objc2_metal::*;

const RGBA8_UNORM: u32 = 28;
const RGBA16_FLOAT: u32 = 10;
const RGBA32_UINT: u32 = 3;
const RGBA32_SINT: u32 = 4;
const D32_FLOAT: u32 = 40;

fn game_dir() -> Option<PathBuf> {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return None;
    }
    if !converter_available() {
        eprintln!("skipping: Metal Shader Converter unavailable");
        return None;
    }
    let dir = std::env::var("AW2_GAME_DIR").ok().map(PathBuf::from).or_else(discover_game_dir);
    if dir.is_none() {
        eprintln!("skipping: Alan Wake 2 game directory not found");
    }
    dir
}

/// Extracts `PSV0`/`RDAT` resource bindings from a DXBC container (converting SM4/SM5 via
/// `dxilconv` first if needed).
fn shader_bindings(dxil: &[u8]) -> Option<Vec<ResourceBinding>> {
    let parsed = Container::parse(dxil).ok()?;
    if parsed.is_sm4_bytecode() {
        let converted = dxilconv::convert_signed(dxil).ok()?;
        Container::parse(&converted).ok()?.resource_bindings()
    } else {
        parsed.resource_bindings()
    }
}

/// Synthesizes a covering [`RootSignature`] across one or more shader stages by coalescing
/// overlapping `(range_type, space)` register intervals.
fn covering_root_signature(stages: &[&[u8]]) -> Result<Arc<RootSignature>, String> {
    let mut by_type_space: BTreeMap<(u32, u32), Vec<(u32, u32)>> = BTreeMap::new();
    for dxil in stages {
        if let Ok(Some(embedded)) = RootSignatureDesc::from_dxil(dxil) {
            if stages.len() == 1 {
                return RootSignature::new(embedded).map_err(|e| e.to_string());
            }
        }
        let bindings = shader_bindings(dxil).ok_or_else(|| "missing PSV0/RDAT resource bindings".to_string())?;
        for b in bindings {
            by_type_space.entry((b.range_type, b.space)).or_default().push((b.lower_bound, b.upper_bound));
        }
    }
    let mut coalesced = Vec::new();
    for ((raw_ty, space), mut intervals) in by_type_space {
        let Some(ty) = DescriptorRangeType::from_raw(raw_ty) else { continue };
        intervals.sort_by_key(|&(lo, _)| lo);
        let mut merged: Vec<(u32, u32)> = Vec::new();
        for (lo, hi) in intervals {
            if let Some(last) = merged.last_mut() {
                if last.1 == u32::MAX || lo <= last.1.saturating_add(1) {
                    last.1 = if last.1 == u32::MAX || hi == u32::MAX { u32::MAX } else { last.1.max(hi) };
                    continue;
                }
            }
            merged.push((lo, hi));
        }
        for (lo, hi) in merged {
            let count = if hi == u32::MAX { flags::UNBOUNDED } else { hi.saturating_sub(lo) + 1 };
            coalesced.push((ty, space, lo, count));
        }
    }
    let desc = RootSignatureDesc::covering_bindings(coalesced);
    RootSignature::new(desc).map_err(|e| e.to_string())
}

/// Infers `RTVFormats` from a pixel shader's `OSG1`/`OSGN` output signature so integer
/// `SV_Target` outputs get `RGBA32_UINT`/`RGBA32_SINT` and float outputs get `RGBA8_UNORM`.
fn infer_rtv_formats(ps_dxil: &[u8]) -> Vec<u32> {
    let rd32 = |b: &[u8], o: usize| b.get(o..o + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap()));
    let mut rts = BTreeMap::<u32, u32>::new();
    if let Ok(c) = Container::parse(ps_dxil) {
        for (fcc, stride, name_off_pos, idx_pos, sys_pos, comp_pos) in [
            (*b"OSG1", 32usize, 4usize, 8usize, 12usize, 16usize),
            (*b"OSGN", 24usize, 0usize, 4usize, 8usize, 12usize),
        ] {
            if let Some(d) = c.part(fcc) {
                let count = rd32(d, 0).unwrap_or(0) as usize;
                for i in 0..count {
                    let base = 8 + i * stride;
                    let (Some(noff), Some(idx), Some(sys), Some(comp)) = (
                        rd32(d, base + name_off_pos),
                        rd32(d, base + idx_pos),
                        rd32(d, base + sys_pos),
                        rd32(d, base + comp_pos),
                    ) else {
                        break;
                    };
                    let sem = d
                        .get(noff as usize..)
                        .and_then(|t| std::str::from_utf8(&t[..t.iter().position(|&b| b == 0).unwrap_or(t.len())]).ok())
                        .unwrap_or("");
                    // D3D_NAME_TARGET == 64 or SV_Target semantic
                    if (sys == 64 || sem.eq_ignore_ascii_case("SV_Target")) && idx < 8 {
                        let fmt = match comp {
                            1 => RGBA32_UINT,
                            2 => RGBA32_SINT,
                            _ => RGBA8_UNORM,
                        };
                        rts.insert(idx, fmt);
                    }
                }
                if !rts.is_empty() {
                    break;
                }
            }
        }
    }
    if rts.is_empty() {
        return vec![RGBA8_UNORM];
    }
    let max_idx = *rts.keys().max().unwrap();
    (0..=max_idx).map(|i| rts.get(&i).copied().unwrap_or(RGBA8_UNORM)).collect()
}

/// Builds an explicit `InputElement` list from a vertex shader's `ISG1`/`ISGN` chunk.
fn infer_input_layout(vs_dxil: &[u8]) -> Vec<InputElement> {
    let rd32 = |b: &[u8], o: usize| b.get(o..o + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap()));
    let mut out = Vec::new();
    if let Ok(c) = Container::parse(vs_dxil) {
        for (fcc, stride, name_off_pos, idx_pos, sys_pos) in [
            (*b"ISG1", 32usize, 4usize, 8usize, 12usize),
            (*b"ISGN", 24usize, 0usize, 4usize, 8usize),
        ] {
            if let Some(d) = c.part(fcc) {
                let count = rd32(d, 0).unwrap_or(0) as usize;
                for i in 0..count {
                    let base = 8 + i * stride;
                    let (Some(noff), Some(idx), Some(sys)) = (
                        rd32(d, base + name_off_pos),
                        rd32(d, base + idx_pos),
                        rd32(d, base + sys_pos),
                    ) else {
                        break;
                    };
                    if sys != 0 {
                        continue; // Skip SV_VertexID / SV_InstanceID system values
                    }
                    let sem = d
                        .get(noff as usize..)
                        .and_then(|t| std::str::from_utf8(&t[..t.iter().position(|&b| b == 0).unwrap_or(t.len())]).ok())
                        .unwrap_or("");
                    if !sem.is_empty() {
                        let reflected = format!("{sem}{idx}");
                        out.push(InputElement {
                            semantic_name: sem.to_string(),
                            semantic_index: idx,
                            format: infer_vertex_input_format(vs_dxil, &reflected),
                            input_slot: 0,
                            aligned_byte_offset: 0xFFFF_FFFF,
                            input_slot_class: 0,
                            instance_data_step_rate: 0,
                        });
                    }
                }
                if !out.is_empty() {
                    break;
                }
            }
        }
    }
    out
}

fn default_blend() -> BlendDesc {
    let mut b = BlendDesc::default();
    for rt in &mut b.RenderTarget {
        rt.RenderTargetWriteMask = 0xF;
    }
    b
}

/// True if the shader is eligible for Apple Silicon Metal 4 compilation (SM <= 6.6, not Wave64).
fn is_apple_silicon_compatible(dxbc: &[u8]) -> bool {
    let Ok(c) = Container::parse(dxbc) else { return false };
    if c.is_sm4_bytecode() {
        return dxilconv::available();
    }
    let Some(p) = c.program() else { return false };
    if p.major > 6 || (p.major == 6 && p.minor > 6) {
        return false;
    }
    if let Some((min_wave, _)) = c.expected_wave_lane_count() {
        if min_wave == 64 {
            return false;
        }
    }
    true
}

#[test]
fn pe_embedded_compute_and_graphics_psos() {
    let Some(game) = game_dir() else { return };

    let mut total_compute = 0usize;
    let mut total_graphics = 0usize;

    for bin_name in ["AlanWake2.exe", "NRD.dll", "amd_fidelityfx_upscaler_dx12.dll"] {
        let path = game.join(bin_name);
        if !path.exists() {
            continue;
        }
        let bytes = std::fs::read(&path).expect("read PE binary");
        let extracted = extract_pe_shaders(bin_name, &bytes);
        assert!(!extracted.is_empty(), "{bin_name}: expected embedded shaders");

        let mut cs_blobs = Vec::new();
        let mut vs_blobs = Vec::new();
        let mut ps_blobs = Vec::new();

        for s in extracted {
            if !is_apple_silicon_compatible(&s.dxbc_bytes) {
                continue;
            }
            let Ok(c) = Container::parse(&s.dxbc_bytes) else { continue };
            match c.program().map(|p| p.kind) {
                Some(ProgramKind::Compute) => cs_blobs.push(s.dxbc_bytes),
                Some(ProgramKind::Vertex) => vs_blobs.push(s.dxbc_bytes),
                Some(ProgramKind::Pixel) => ps_blobs.push(s.dxbc_bytes),
                _ => {}
            }
        }

        // Compile up to 12 compute PSOs per PE binary (covers AlanWake2.exe, NRD.dll, FSR2 DLL).
        let mut bin_cs = 0usize;
        for (i, cs) in cs_blobs.iter().take(12).enumerate() {
            let rs = covering_root_signature(&[cs]).unwrap_or_else(|e| panic!("{bin_name} cs[{i}] root sig: {e}"));
            let pso = ComputePipeline::new(rs, cs, &format!("{bin_name}_cs_{i}"))
                .unwrap_or_else(|e| panic!("{bin_name} cs[{i}] ComputePipeline::new: {e}"));
            assert!(pso.pso.maxTotalThreadsPerThreadgroup() > 0);
            assert!(pso.threadgroup.iter().all(|&n| n >= 1));
            bin_cs += 1;
            total_compute += 1;
        }

        // Compile graphics PSOs for PE binaries that embed VS/PS shaders (AlanWake2.exe).
        let mut bin_gfx = 0usize;
        for (i, vs) in vs_blobs.iter().take(8).enumerate() {
            let ps = ps_blobs.get(i % ps_blobs.len().max(1)).map(Vec::as_slice);
            let stages: Vec<&[u8]> = match ps {
                Some(p) => vec![vs.as_slice(), p],
                None => vec![vs.as_slice()],
            };
            let rs = covering_root_signature(&stages).unwrap_or_else(|e| panic!("{bin_name} gfx[{i}] root sig: {e}"));
            let rtv_formats = ps.map(infer_rtv_formats).unwrap_or_else(|| vec![RGBA8_UNORM]);
            let desc = GraphicsDesc {
                vs,
                ps,
                blend: default_blend(),
                sample_mask: u32::MAX,
                raster: RasterState::default(),
                depth_stencil: DepthStencilState::without_depth_stencil(),
                input_layout: infer_input_layout(vs),
                strip_cut: 0,
                topology_type: 3, // TRIANGLE
                rtv_formats,
                dsv_format: 0,
                sample_count: 1,
            };
            let _pso = GraphicsPipeline::new(rs, &desc, &format!("{bin_name}_gfx_{i}"))
                .unwrap_or_else(|e| panic!("{bin_name} gfx[{i}] GraphicsPipeline::new: {e}"));
            bin_gfx += 1;
            total_graphics += 1;
        }

        eprintln!("{bin_name}: compiled {bin_cs} compute PSOs, {bin_gfx} graphics PSOs");
    }

    assert!(total_compute >= 20, "expected >= 20 PE compute PSOs, got {total_compute}");
    assert!(total_graphics >= 1, "expected >= 1 PE graphics PSO, got {total_graphics}");
}

#[test]
fn binrfx_compute_graphics_and_mesh_psos() {
    let Some(game) = game_dir() else { return };
    let rfx_dir = game.join("data/shaders/build/pc_dx12");

    let bundle_names = [
        "ambientconvolution.binrfx",
        "autoexposurehistogram.binrfx",
        "billboard.binrfx",
        "bink.binrfx",
        "bloomx86CS.binrfx",
        "blur.binrfx",
        "character.binrfx",
        "cloth.binrfx",
        "cluster_operations.binrfx",
        "CoherentGT.binrfx",
        "decal.binrfx",
        "eye.binrfx",
        "ffx_fsr2_rcas_pass.binrfx",
        "flare.binrfx",
        "gpupipe_culling_meshlet_debug.binrfx",
        "hair.binrfx",
        "imgui.binrfx",
        "mesh_decal.binrfx",
        "mesh_visibility.binrfx",
        "particlemesh.binrfx",
        "skydome.binrfx",
    ];

    let mut compute_count = 0usize;
    let mut graphics_color_count = 0usize;
    let mut graphics_depth_count = 0usize;
    let mut graphics_fallback_layout_count = 0usize;
    let mut mesh_color_count = 0usize;
    let mut mesh_depth_count = 0usize;

    for bname in bundle_names {
        let path = rfx_dir.join(bname);
        if !path.exists() {
            continue;
        }
        let rfx = BinRfx::open(&path).unwrap_or_else(|e| panic!("{bname}: {e}"));

        let mut cs_progs = Vec::new();
        let mut vs_progs = Vec::new();
        let mut ps_progs = Vec::new();
        let mut ms_progs = Vec::new();

        for prog in &rfx.programs {
            if prog.entry_point.is_empty() || !is_apple_silicon_compatible(&prog.dxbc_bytes) {
                continue;
            }
            let Ok(c) = Container::parse(&prog.dxbc_bytes) else { continue };
            let Some(info) = c.program() else { continue };
            match info.kind {
                ProgramKind::Compute => cs_progs.push(prog),
                ProgramKind::Vertex => vs_progs.push(prog),
                ProgramKind::Pixel => ps_progs.push(prog),
                ProgramKind::Mesh => ms_progs.push(prog),
                _ => {}
            }
        }

        // Helper: find the PS program in `ps_progs` whose entry point stem matches `stage_entry`
        // (e.g. `character_lightVS` / `character_lightMS` -> `character_lightPS`).
        let find_matching_ps = |stage_entry: &str, suffix: &str| {
            if let Some(stem) = stage_entry.strip_suffix(suffix) {
                let target = format!("{stem}PS");
                if let Some(p) = ps_progs.iter().copied().find(|p| p.entry_point == target) {
                    return Some(p);
                }
            }
            if ps_progs.len() == 1 && (vs_progs.len() + ms_progs.len() <= 2) {
                return ps_progs.first().copied();
            }
            None
        };

        // 1. Compute PSOs
        for prog in cs_progs.iter().take(4) {
            let rs = covering_root_signature(&[&prog.dxbc_bytes])
                .unwrap_or_else(|e| panic!("{bname} {} root sig: {e}", prog.entry_point));
            let pso = ComputePipeline::new(rs, &prog.dxbc_bytes, &format!("{bname}:{}", prog.entry_point))
                .unwrap_or_else(|e| panic!("{bname} {} ComputePipeline::new: {e}", prog.entry_point));
            assert!(pso.pso.maxTotalThreadsPerThreadgroup() > 0);
            assert!(pso.threadgroup.iter().all(|&n| n >= 1));
            compute_count += 1;
        }

        // 2. Traditional Graphics PSOs (VS + PS color pass, VS + PS fallback layout, VS-only depth pass)
        for (i, vs) in vs_progs.iter().take(4).enumerate() {
            if let Some(ps) = find_matching_ps(&vs.entry_point, "VS") {
                let rs = covering_root_signature(&[&vs.dxbc_bytes, &ps.dxbc_bytes])
                    .unwrap_or_else(|e| panic!("{bname} {}+{} root sig: {e}", vs.entry_point, ps.entry_point));
                let rtv_formats = infer_rtv_formats(&ps.dxbc_bytes);
                let desc = GraphicsDesc {
                    vs: &vs.dxbc_bytes,
                    ps: Some(&ps.dxbc_bytes),
                    blend: default_blend(),
                    sample_mask: u32::MAX,
                    raster: RasterState::default(),
                    depth_stencil: DepthStencilState::default(),
                    input_layout: infer_input_layout(&vs.dxbc_bytes),
                    strip_cut: 0,
                    topology_type: 3, // TRIANGLE
                    rtv_formats: rtv_formats.clone(),
                    dsv_format: D32_FLOAT,
                    sample_count: 1,
                };
                let _pso = GraphicsPipeline::new(
                    rs.clone(),
                    &desc,
                    &format!("{bname}:{}+{}", vs.entry_point, ps.entry_point),
                )
                .unwrap_or_else(|e| panic!("{bname} {}+{} GraphicsPipeline::new: {e}", vs.entry_point, ps.entry_point));
                graphics_color_count += 1;

                // Also verify empty input_layout fallback attribute synthesis on the first matched VS+PS pair of each bundle.
                if i == 0 && rtv_formats.iter().all(|&f| f == RGBA8_UNORM) {
                    let fallback_desc = GraphicsDesc {
                        input_layout: Vec::new(),
                        rtv_formats: vec![RGBA16_FLOAT; rtv_formats.len()],
                        ..desc
                    };
                    let _pso_fallback = GraphicsPipeline::new(
                        rs,
                        &fallback_desc,
                        &format!("{bname}:{}+{}:fallback_ia", vs.entry_point, ps.entry_point),
                    )
                    .unwrap_or_else(|e| panic!("{bname} fallback IA GraphicsPipeline::new: {e}"));
                    graphics_fallback_layout_count += 1;
                }
            }

            // VS-only depth prepass PSO (ps: None, rtv_formats: empty, dsv_format: D32_FLOAT)
            if i == 0 {
                let rs_vs = covering_root_signature(&[&vs.dxbc_bytes])
                    .unwrap_or_else(|e| panic!("{bname} {} depth-only root sig: {e}", vs.entry_point));
                let depth_desc = GraphicsDesc {
                    vs: &vs.dxbc_bytes,
                    ps: None,
                    blend: default_blend(),
                    sample_mask: u32::MAX,
                    raster: RasterState::default(),
                    depth_stencil: DepthStencilState::default(),
                    input_layout: infer_input_layout(&vs.dxbc_bytes),
                    strip_cut: 0,
                    topology_type: 3,
                    rtv_formats: Vec::new(),
                    dsv_format: D32_FLOAT,
                    sample_count: 1,
                };
                let _pso_depth = GraphicsPipeline::new(
                    rs_vs,
                    &depth_desc,
                    &format!("{bname}:{}:depth_only", vs.entry_point),
                )
                .unwrap_or_else(|e| panic!("{bname} {} depth-only GraphicsPipeline::new: {e}", vs.entry_point));
                graphics_depth_count += 1;
            }
        }

        // 3. Mesh Shader PSOs (MS + PS color pass and MS-only depth pass)
        for (i, ms) in ms_progs.iter().take(4).enumerate() {
            if let Some(ps) = find_matching_ps(&ms.entry_point, "MS") {
                let rs = covering_root_signature(&[&ms.dxbc_bytes, &ps.dxbc_bytes])
                    .unwrap_or_else(|e| panic!("{bname} {}+{} mesh root sig: {e}", ms.entry_point, ps.entry_point));
                let rtv_formats = infer_rtv_formats(&ps.dxbc_bytes);
                let desc = MeshDesc {
                    amplification: None,
                    mesh: &ms.dxbc_bytes,
                    pixel: Some(&ps.dxbc_bytes),
                    blend: default_blend(),
                    sample_mask: u32::MAX,
                    raster: RasterState::default(),
                    depth_stencil: DepthStencilState::default(),
                    rtv_formats,
                    dsv_format: D32_FLOAT,
                    sample_count: 1,
                };
                let pso = MeshPipeline::new(
                    rs,
                    &desc,
                    &format!("{bname}:{}+{}", ms.entry_point, ps.entry_point),
                )
                .unwrap_or_else(|e| panic!("{bname} {}+{} MeshPipeline::new: {e}", ms.entry_point, ps.entry_point));
                assert!(pso.mesh_threads.iter().all(|&n| n >= 1));
                mesh_color_count += 1;
            }

            if i == 0 {
                let rs_ms = covering_root_signature(&[&ms.dxbc_bytes])
                    .unwrap_or_else(|e| panic!("{bname} {} mesh depth-only root sig: {e}", ms.entry_point));
                let depth_desc = MeshDesc {
                    amplification: None,
                    mesh: &ms.dxbc_bytes,
                    pixel: None,
                    blend: default_blend(),
                    sample_mask: u32::MAX,
                    raster: RasterState::default(),
                    depth_stencil: DepthStencilState::default(),
                    rtv_formats: Vec::new(),
                    dsv_format: D32_FLOAT,
                    sample_count: 1,
                };
                let pso_depth = MeshPipeline::new(
                    rs_ms,
                    &depth_desc,
                    &format!("{bname}:{}:mesh_depth_only", ms.entry_point),
                )
                .unwrap_or_else(|e| panic!("{bname} {} depth-only MeshPipeline::new: {e}", ms.entry_point));
                assert!(pso_depth.mesh_threads.iter().all(|&n| n >= 1));
                mesh_depth_count += 1;
            }
        }
    }

    eprintln!(
        "binrfx Metal 4 PSOs compiled: {compute_count} compute, {graphics_color_count} gfx (color), \
         {graphics_depth_count} gfx (depth-only), {graphics_fallback_layout_count} gfx (fallback IA), \
         {mesh_color_count} mesh (color), {mesh_depth_count} mesh (depth-only)"
    );

    assert!(compute_count >= 15, "expected >= 15 compute PSOs, got {compute_count}");
    assert!(graphics_color_count >= 10, "expected >= 10 graphics color PSOs, got {graphics_color_count}");
    assert!(graphics_depth_count >= 5, "expected >= 5 graphics depth-only PSOs, got {graphics_depth_count}");
    assert!(graphics_fallback_layout_count >= 5, "expected >= 5 graphics fallback-IA PSOs, got {graphics_fallback_layout_count}");
    assert!(mesh_color_count >= 5, "expected >= 5 mesh color PSOs, got {mesh_color_count}");
    assert!(mesh_depth_count >= 3, "expected >= 3 mesh depth-only PSOs, got {mesh_depth_count}");
}

/// Synthesizes a minimal MSL vertex shader library whose stage-out struct matches the
/// `ISG1`/`ISGN` input signature of `ps_dxil`, so any standalone pixel shader permutation
/// can be linked into a full `MTLRenderPipelineState` on the Metal 4 compiler.
fn synthesize_matching_vs_library(ps_dxil: &[u8]) -> Result<objc2::rc::Retained<d3d12_metal::metal::pipeline::MtlLibrary>, String> {
    let rd32 = |b: &[u8], o: usize| b.get(o..o + 4).map(|s| u32::from_le_bytes(s.try_into().unwrap()));
    let mut fields = Vec::new();
    let mut inits = Vec::new();

    if let Ok(c) = Container::parse(ps_dxil) {
        for (fcc, stride, name_off_pos, idx_pos, sys_pos, comp_pos, mask_pos) in [
            (*b"ISG1", 32usize, 4usize, 8usize, 12usize, 16usize, 24usize),
            (*b"ISGN", 24usize, 0usize, 4usize, 8usize, 12usize, 20usize),
        ] {
            if let Some(d) = c.part(fcc) {
                let count = rd32(d, 0).unwrap_or(0) as usize;
                for i in 0..count {
                    let base = 8 + i * stride;
                    let (Some(noff), Some(idx), Some(sys), Some(comp)) = (
                        rd32(d, base + name_off_pos),
                        rd32(d, base + idx_pos),
                        rd32(d, base + sys_pos),
                        rd32(d, base + comp_pos),
                    ) else {
                        break;
                    };
                    let mask = d.get(base + mask_pos).copied().unwrap_or(0xF) & 0xF;
                    // Packed signature elements (e.g. `.zw` with mask 0b1100) have `mask.count_ones()`
                    // components in the translated MSL `[[user(...)]]` attribute.
                    let ncomp = mask.count_ones().clamp(1, 4);
                    let sem = d
                        .get(noff as usize..)
                        .and_then(|t| std::str::from_utf8(&t[..t.iter().position(|&b| b == 0).unwrap_or(t.len())]).ok())
                        .unwrap_or("");
                    if sys == 1 || sem.eq_ignore_ascii_case("SV_Position") {
                        continue; // position is always emitted below
                    }
                    if matches!(sys, 7 | 9 | 10 | 11 | 12) {
                        // SV_PrimitiveID (7), SV_IsFrontFace (9), SV_SampleIndex (10),
                        // SV_Coverage (11), SV_InnerCoverage (12) are fragment/rasterizer built-ins.
                        continue;
                    }
                    if sys == 4 || sem.eq_ignore_ascii_case("SV_RenderTargetArrayIndex") {
                        fields.push(format!("    uint v_{i} [[render_target_array_index]];"));
                        inits.push(format!("    out.v_{i} = 0u;"));
                        continue;
                    }
                    if sys == 5 || sem.eq_ignore_ascii_case("SV_ViewportArrayIndex") {
                        fields.push(format!("    uint v_{i} [[viewport_array_index]];"));
                        inits.push(format!("    out.v_{i} = 0u;"));
                        continue;
                    }
                    if sys == 2 || sys == 3 || sem.eq_ignore_ascii_case("SV_ClipDistance") || sem.eq_ignore_ascii_case("SV_CullDistance") {
                        continue;
                    }
                    let scalar = match comp {
                        1 => "uint",
                        2 => "int",
                        _ => "float",
                    };
                    let msl_ty = if ncomp == 1 {
                        scalar.to_string()
                    } else {
                        format!("{scalar}{ncomp}")
                    };
                    let attr_user = format!("user({}{idx})", sem.to_ascii_lowercase());
                    let flat = if comp == 1 || comp == 2 { " [[flat]]" } else { "" };
                    fields.push(format!("    {msl_ty} v_{i} [[{attr_user}]]{flat};"));
                    inits.push(format!("    out.v_{i} = {msl_ty}(0);"));
                }
                break;
            }
        }
    }

    let mut msl = String::from("#include <metal_stdlib>\nusing namespace metal;\nstruct VsOut {\n    float4 pos [[position]];\n");
    for f in &fields {
        msl.push_str(f);
        msl.push('\n');
    }
    msl.push_str("};\nvertex VsOut vs_stub() {\n    VsOut out;\n    out.pos = float4(0.0, 0.0, 0.0, 1.0);\n");
    for init in &inits {
        msl.push_str(init);
        msl.push('\n');
    }
    msl.push_str("    return out;\n}\n");

    d3d12_metal::metal::pipeline::library_from_source("vs_stub", &msl)
        .map_err(|e| format!("library_from_source: {e}\nMSL:\n{msl}"))
}

#[test]
fn exhaustive_corpus_metal4_pso_validation() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    let Some(game) = game_dir() else { return };
    let pipeline = dxil_metal::ShaderPipeline::global().expect("ShaderPipeline::global");
    let pso_cache = d3d12_metal::metal::pipeline::pso_cache().expect("pso_cache");
    let compile_opts = dxil_metal::CompileOptions::default();

    let rfx_dir = game.join("data/shaders/build/pc_dx12");
    let rfx_paths = dxil_metal::rfx::list_rfx_bundles(&rfx_dir, None).expect("list_rfx_bundles");
    assert_eq!(rfx_paths.len(), 153, "expected 153 .binrfx bundles");

    let mut rfx_active_exports = 0usize;
    let mut pe_active_exports = 0usize;

    let mut seen_keys = std::collections::HashSet::<u128>::new();
    let mut unique_jobs: Vec<(String, Vec<u8>, Arc<dxil_metal::CompiledShader>, u8)> = Vec::new();
    let mut failures = Vec::<String>::new();

    let mut collect_shader = |label: String, dxbc: &[u8], compiled: Result<Arc<dxil_metal::CompiledShader>, String>| {
        let shader = match compiled {
            Ok(s) => s,
            Err(e) => {
                failures.push(format!("{label}: compile failed: {e}"));
                return;
            }
        };
        if seen_keys.insert(shader.key) {
            unique_jobs.push((label, dxbc.to_vec(), shader, 0));
        }
    };

    // 1. All 153 .binrfx bundles (1,906 active SM <= 6.6 exports)
    for path in &rfx_paths {
        let rfx = dxil_metal::rfx::RfxBundle::open(path).expect("open .binrfx");
        for prog in &rfx.programs {
            let rs = RootSignatureDesc::covering_dxil(&prog.dxbc_bytes)
                .and_then(|d| pipeline.root_signature(&d.to_blob()).ok());
            for export in prog.exports() {
                if export.is_sm69_ser() {
                    continue;
                }
                rfx_active_exports += 1;
                let req = dxil_metal::CompileRequest::new(&prog.dxbc_bytes, rs.as_deref(), &compile_opts)
                    .entry_point(&export.entry_point);
                let label = format!("{}:{}", rfx.file_name, export.entry_point);
                collect_shader(label, &prog.dxbc_bytes, pipeline.compile(&req).map_err(|e| e.to_string()));
            }
        }
    }

    // 2. All 10 PE binaries in $AW2_GAME_DIR (1,405 active Wave32/unpinned exports)
    let pe_paths = dxil_metal::pe::list_pe_binaries(&game, None).expect("list_pe_binaries");
    for path in &pe_paths {
        let Ok(bytes) = std::fs::read(path) else { continue };
        let bin_name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let shaders = extract_pe_shaders(&bin_name, &bytes);
        for sh in shaders {
            if sh.is_wave64 {
                continue;
            }
            for export in sh.exports() {
                pe_active_exports += 1;
                let label = format!("{bin_name}@{:#x}:{}", sh.file_offset, export.entry_point);
                let compiled = if sh.has_rts0 {
                    pipeline
                        .root_signature(&sh.dxbc_bytes)
                        .ok()
                        .and_then(|rs| {
                            let req = dxil_metal::CompileRequest::new(&sh.dxbc_bytes, Some(rs.as_ref()), &compile_opts)
                                .entry_point(&export.entry_point);
                            pipeline.compile(&req).ok()
                        })
                } else {
                    None
                }
                .ok_or_else(|| "no embedded RTS0".to_string())
                .or_else(|_| {
                    let rs = RootSignatureDesc::covering_dxil(&sh.dxbc_bytes)
                        .and_then(|d| pipeline.root_signature(&d.to_blob()).ok());
                    let req = dxil_metal::CompileRequest::new(&sh.dxbc_bytes, rs.as_deref(), &compile_opts)
                        .entry_point(&export.entry_point);
                    pipeline.compile(&req).map_err(|e| e.to_string())
                });

                collect_shader(label, &sh.dxbc_bytes, compiled);
            }
        }
    }

    // 3. All recorded manifest shaders (runtime PSO compile requests from the game)
    if let Some(disk_dir) = dxil_metal::CacheConfig::from_env().disk_dir {
        let mdir = disk_dir.join("manifest");
        if let Ok(m) = dxil_metal::manifest::Manifest::load(&mdir) {
            for e in &m.entries {
                let Ok(dxil) = m.dxil(e) else { continue };
                if let Ok(c) = dxil_metal::dxbc::Container::parse(&dxil) {
                    if c.program().is_some_and(|p| p.major == 6 && p.minor >= 9) {
                        continue;
                    }
                }
                let rs = e
                    .root_signature
                    .and_then(|h| m.root_signature_blob(h).ok())
                    .and_then(|b| pipeline.root_signature(&b).ok());
                let lrs = e
                    .local_root_signature
                    .and_then(|h| m.root_signature_blob(h).ok())
                    .and_then(|b| pipeline.root_signature(&b).ok());
                let mut req = dxil_metal::CompileRequest::new(&dxil, rs.as_deref(), &e.options);
                if let Some(ep) = &e.entry_point {
                    req = req.entry_point(ep);
                }
                if let Some(st) = e.expected_stage {
                    req = req.stage(st);
                }
                if let Some(lrs) = lrs.as_deref() {
                    req = req.local_root_signature(lrs);
                }
                if let Ok(shader) = pipeline.compile(&req) {
                    if seen_keys.insert(shader.key) {
                        unique_jobs.push((format!("manifest:{:032x}", e.key), dxil, shader, e.options.int_rt_mask));
                    }
                }
            }
        }
    }

    let cs_psos = AtomicUsize::new(0);
    let vs_psos = AtomicUsize::new(0);
    let ms_psos = AtomicUsize::new(0);
    let ps_psos = AtomicUsize::new(0);
    let rt_libs = AtomicUsize::new(0);
    let next = AtomicUsize::new(0);
    let pso_failures = Mutex::new(Vec::<String>::new());
    let threads = std::thread::available_parallelism().map_or(8, |n| n.get());

    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let idx = next.fetch_add(1, Ordering::Relaxed);
                let Some((label, dxbc, shader, int_rt_mask)) = unique_jobs.get(idx) else { break };

                let lib = match d3d12_metal::metal::pipeline::library(shader) {
                    Ok(l) => l,
                    Err(e) => {
                        pso_failures.lock().unwrap().push(format!("{label}: library load failed: {e}"));
                        continue;
                    }
                };
                let fn_desc = d3d12_metal::metal::pipeline::shader_function_descriptor(&lib, shader);

                match shader.stage {
                    dxil_metal::ShaderStage::Compute => {
                        let desc = MTL4ComputePipelineDescriptor::new();
                        desc.setComputeFunctionDescriptor(Some(&fn_desc));
                        if let Err(e) = pso_cache.compute_pipeline(Some(shader.key), &desc) {
                            pso_failures.lock().unwrap().push(format!("{label} (Compute): {e}"));
                        } else {
                            cs_psos.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    dxil_metal::ShaderStage::Vertex => {
                        let rs = match covering_root_signature(&[dxbc]) {
                            Ok(r) => r,
                            Err(e) => {
                                pso_failures.lock().unwrap().push(format!("{label} (Vertex RS): {e}"));
                                continue;
                            }
                        };
                        let gdesc = GraphicsDesc {
                            vs: dxbc,
                            ps: None,
                            blend: default_blend(),
                            sample_mask: u32::MAX,
                            raster: RasterState::default(),
                            depth_stencil: DepthStencilState::default(),
                            input_layout: infer_input_layout(dxbc),
                            strip_cut: 0,
                            topology_type: 3,
                            rtv_formats: Vec::new(),
                            dsv_format: D32_FLOAT,
                            sample_count: 1,
                        };
                        if let Err(e) = GraphicsPipeline::new(rs, &gdesc, label) {
                            pso_failures.lock().unwrap().push(format!("{label} (Vertex PSO): {e}"));
                        } else {
                            vs_psos.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    dxil_metal::ShaderStage::Mesh => {
                        let rs = match covering_root_signature(&[dxbc]) {
                            Ok(r) => r,
                            Err(e) => {
                                pso_failures.lock().unwrap().push(format!("{label} (Mesh RS): {e}"));
                                continue;
                            }
                        };
                        let mdesc = MeshDesc {
                            amplification: None,
                            mesh: dxbc,
                            pixel: None,
                            blend: default_blend(),
                            sample_mask: u32::MAX,
                            raster: RasterState::default(),
                            depth_stencil: DepthStencilState::default(),
                            rtv_formats: Vec::new(),
                            dsv_format: D32_FLOAT,
                            sample_count: 1,
                        };
                        if let Err(e) = MeshPipeline::new(rs, &mdesc, label) {
                            pso_failures.lock().unwrap().push(format!("{label} (Mesh PSO): {e}"));
                        } else {
                            ms_psos.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    dxil_metal::ShaderStage::Fragment => {
                        match synthesize_matching_vs_library(dxbc) {
                            Ok(vlib) => {
                                let desc = MTL4RenderPipelineDescriptor::new();
                                let vfn = d3d12_metal::metal::pipeline::function_descriptor(&vlib, "vs_stub");
                                desc.setVertexFunctionDescriptor(Some(&vfn));
                                desc.setFragmentFunctionDescriptor(Some(&fn_desc));
                                let n_rt = match shader.reflection.info {
                                    dxil_metal::StageInfo::Fragment { num_render_targets, .. } => {
                                        (num_render_targets.max(0) as usize).min(8)
                                    }
                                    _ => 1,
                                };
                                let ca = desc.colorAttachments();
                                for i in 0..n_rt.max(1) {
                                    let a = unsafe { ca.objectAtIndexedSubscript(i) };
                                    if (*int_rt_mask & (1 << i)) != 0 {
                                        a.setPixelFormat(MTLPixelFormat::RGBA32Uint);
                                    } else {
                                        a.setPixelFormat(MTLPixelFormat::RGBA8Unorm);
                                    }
                                }
                                unsafe { desc.setRasterSampleCount(1) };
                                desc.setInputPrimitiveTopology(MTLPrimitiveTopologyClass::Triangle);
                                if let Err(e) = pso_cache.render_pipeline(Some(shader.key ^ 0x5053_5F53_5455_42u128), &desc) {
                                    pso_failures.lock().unwrap().push(format!("{label} (Fragment PSO): {e}"));
                                } else {
                                    ps_psos.fetch_add(1, Ordering::Relaxed);
                                }
                            }
                            Err(e) => pso_failures.lock().unwrap().push(format!("{label} (Fragment VS stub): {e}")),
                        }
                    }
                    s if s.is_ray_tracing() || matches!(s, dxil_metal::ShaderStage::Amplification) => {
                        rt_libs.fetch_add(1, Ordering::Relaxed);
                    }
                    other => {
                        pso_failures.lock().unwrap().push(format!("{label}: unexpected stage {other:?}"));
                    }
                }
            });
        }
    });

    failures.extend(pso_failures.into_inner().unwrap());
    d3d12_metal::metal::pipeline::flush_pso_cache();

    let cs_psos = cs_psos.load(Ordering::Relaxed);
    let vs_psos = vs_psos.load(Ordering::Relaxed);
    let ms_psos = ms_psos.load(Ordering::Relaxed);
    let ps_psos = ps_psos.load(Ordering::Relaxed);
    let rt_libs = rt_libs.load(Ordering::Relaxed);

    eprintln!(
        "Exhaustive Metal 4 corpus validation: rfx_active={rfx_active_exports}, pe_active={pe_active_exports}, \
         total_active={}, unique_keys={}, cs_psos={cs_psos}, vs_psos={vs_psos}, ms_psos={ms_psos}, \
         ps_psos={ps_psos}, rt_libs={rt_libs}, failures={}",
        rfx_active_exports + pe_active_exports,
        seen_keys.len(),
        failures.len()
    );
    for f in failures.iter().take(25) {
        eprintln!("  FAILURE: {f}");
    }
    assert_eq!(rfx_active_exports, 1906, "expected 1,906 active .binrfx exports");
    assert_eq!(pe_active_exports, 1405, "expected 1,405 active PE exports");
    assert!(failures.is_empty(), "{} Metal 4 PSO failures out of {} unique shaders", failures.len(), seen_keys.len());
}

#[test]
fn pe_middleware_compute_dispatch_executes_on_gpu() {
    use d3d12_metal::metal::clear::ClearValue;
    use d3d12_metal::metal::cmdlist::{CommandAllocator, CommandList};
    use d3d12_metal::metal::descriptor::{heap_kind, DescriptorHeap, RESOURCE_DESCRIPTOR_SIZE};
    use d3d12_metal::metal::queue::{ListType, Queue};
    use d3d12_metal::metal::resource::Resource;
    use d3d12_metal::metal::views::{self, SamplerDesc};
    use d3d12_metal::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};
    use dxil_metal::root_signature::RootParameterLayoutKind;

    let Some(game) = game_dir() else { return };

    // Allocate Metal 4 queue, command allocator, and command list on the D3D12-on-Metal backend.
    let queue = Queue::new(ListType::Direct, "pe_middleware_compute_queue").expect("Metal 4 Queue");
    let alloc = CommandAllocator::new(ListType::Direct).expect("Metal 4 CommandAllocator");
    let mut cmd = CommandList::new(ListType::Direct).expect("Metal 4 CommandList");

    // Allocate dummy CBV/SRV/UAV buffer (64 KiB), readback buffer (256 B), and 64x64 RGBA16Float 2D UAV texture.
    const BUF_SIZE: u64 = 65_536;
    const R32_TYPELESS: u32 = 39;
    let dummy_buf = Resource::new_committed(
        &ResourceDesc {
            Dimension: dim::BUFFER,
            Width: BUF_SIZE,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
            ..Default::default()
        },
        &HeapProperties { Type: heap_type::DEFAULT, ..Default::default() },
        MTLTextureUsage::empty(),
        "pe_compute_dummy_buf",
    )
    .expect("dummy CBV/SRV/UAV buffer");

    let readback_buf = Resource::new_committed(
        &ResourceDesc {
            Dimension: dim::BUFFER,
            Width: 256,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            Flags: 0,
            ..Default::default()
        },
        &HeapProperties { Type: heap_type::READBACK, ..Default::default() },
        MTLTextureUsage::empty(),
        "pe_compute_readback_buf",
    )
    .expect("readback buffer");

    let dummy_tex = Resource::new_committed(
        &ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Width: 64,
            Height: 64,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: RGBA16_FLOAT,
            SampleCount: 1,
            Flags: resource_flags::ALLOW_UNORDERED_ACCESS | resource_flags::ALLOW_RENDER_TARGET,
            ..Default::default()
        },
        &HeapProperties { Type: heap_type::DEFAULT, ..Default::default() },
        MTLTextureUsage::empty(),
        "pe_compute_dummy_tex2d",
    )
    .expect("dummy 2D texture");

    // Create shader-visible CBV/SRV/UAV and Sampler descriptor heaps.
    let res_heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 512, true, "pe_compute_res_heap")
        .expect("shader-visible CBV_SRV_UAV heap");
    let samp_heap = DescriptorHeap::new(heap_kind::SAMPLER, 64, true, "pe_compute_samp_heap")
        .expect("shader-visible SAMPLER heap");

    // Populate a hybrid 24-byte IRDescriptorTableEntry in every slot of `res_heap`:
    // - bytes[0..8]:   gpu_va of `dummy_buf` (used by CBV / raw / structured buffer SRVs & UAVs)
    // - bytes[8..16]:  gpuResourceID of `dummy_tex` (used by Texture2D SRVs & UAVs)
    // - bytes[16..24]: BUF_SIZE in low 32 bits (valid buffer size for bounds checks, and ~0.0f min_lod_clamp for textures)
    let tex_entry = views::uav_entry(Some(&dummy_tex), None).expect("dummy texture UAV entry");
    let mut hybrid_entry = [0u8; 24];
    hybrid_entry[0..8].copy_from_slice(&dummy_buf.gpu_va.to_le_bytes());
    hybrid_entry[8..16].copy_from_slice(&tex_entry[8..16]);
    hybrid_entry[16..24].copy_from_slice(&BUF_SIZE.to_le_bytes());
    for slot in 0..512usize {
        let dst = (res_heap.cpu_base + slot * RESOURCE_DESCRIPTOR_SIZE as usize) as *mut [u8; 24];
        unsafe { dst.write(hybrid_entry) };
    }

    let samp_entry = views::sampler_entry(&SamplerDesc::default()).expect("default sampler entry");
    for slot in 0..64usize {
        let dst = (samp_heap.cpu_base + slot * RESOURCE_DESCRIPTOR_SIZE as usize) as *mut [u8; 24];
        unsafe { dst.write(samp_entry) };
    }

    // Collect embedded compute shaders from AlanWake2.exe, NRD.dll, and amd_fidelityfx_upscaler_dx12.dll.
    let mut dispatched_psos: Vec<(String, Arc<ComputePipeline>)> = Vec::new();
    for bin_name in ["AlanWake2.exe", "NRD.dll", "amd_fidelityfx_upscaler_dx12.dll"] {
        let path = game.join(bin_name);
        if !path.exists() {
            continue;
        }
        let bytes = std::fs::read(&path).expect("read PE binary");
        let extracted = extract_pe_shaders(bin_name, &bytes);
        let mut count_for_bin = 0usize;

        for sh in extracted {
            if !is_apple_silicon_compatible(&sh.dxbc_bytes) {
                continue;
            }
            let Ok(c) = Container::parse(&sh.dxbc_bytes) else { continue };
            if c.program().map(|p| p.kind) != Some(ProgramKind::Compute) {
                continue;
            }

            // Use the embedded RTS0 root signature when present (e.g. FSR2 DLL), else synthesize a covering RS.
            let rs = if sh.has_rts0 {
                RootSignature::from_blob(&sh.dxbc_bytes)
                    .ok()
                    .or_else(|| covering_root_signature(&[&sh.dxbc_bytes]).ok())
            } else {
                covering_root_signature(&[&sh.dxbc_bytes]).ok()
            };
            let Some(rs) = rs else { continue };

            let label = format!("{bin_name}@{:#x}", sh.file_offset);
            let Ok(pso) = ComputePipeline::new(rs, &sh.dxbc_bytes, &label) else {
                continue;
            };

            dispatched_psos.push((label, Arc::new(pso)));
            count_for_bin += 1;
            if count_for_bin >= 2 {
                break;
            }
        }
        assert!(count_for_bin >= 1, "{bin_name}: expected at least 1 dispatchable compute shader");
    }

    // Record command list: initialize dummy resources, bind heaps + root sig + PSO + descriptors/constants, dispatch(1, 1, 1).
    let mut raw_uav_desc = [0u8; 40];
    raw_uav_desc[0..4].copy_from_slice(&R32_TYPELESS.to_le_bytes());
    raw_uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes()); // BUFFER
    raw_uav_desc[16..20].copy_from_slice(&(BUF_SIZE as u32 / 4).to_le_bytes());
    raw_uav_desc[32..36].copy_from_slice(&1u32.to_le_bytes()); // RAW

    cmd.begin(&alloc).expect("CommandList::begin");
    cmd.clear_uav(&dummy_buf, Some(&raw_uav_desc), ClearValue::Uint([0, 0, 0, 0]), &[])
        .expect("clear dummy buffer");
    cmd.clear_uav(&dummy_tex, None, ClearValue::Float([0.0, 0.0, 0.0, 1.0]), &[])
        .expect("clear dummy texture");
    cmd.set_descriptor_heaps(res_heap.gpu_base, samp_heap.gpu_base);

    for (label, pso) in &dispatched_psos {
        cmd.compute_root
            .set_root_signature(pso.rs.clone())
            .unwrap_or_else(|e| panic!("{label}: set_root_signature: {e}"));
        for (i, param) in pso.rs.layout().parameters.iter().enumerate() {
            match &param.kind {
                RootParameterLayoutKind::DescriptorTable { sampler: false, .. } => {
                    cmd.compute_root
                        .set_table(i as u32, res_heap.gpu_base)
                        .unwrap_or_else(|e| panic!("{label}: set_table({i}): {e}"));
                }
                RootParameterLayoutKind::DescriptorTable { sampler: true, .. } => {
                    cmd.compute_root
                        .set_table(i as u32, samp_heap.gpu_base)
                        .unwrap_or_else(|e| panic!("{label}: set_sampler_table({i}): {e}"));
                }
                RootParameterLayoutKind::Descriptor { .. } => {
                    cmd.compute_root
                        .set_descriptor(i as u32, dummy_buf.gpu_va)
                        .unwrap_or_else(|e| panic!("{label}: set_descriptor({i}): {e}"));
                }
                RootParameterLayoutKind::Constants { num_32bit_values, .. } => {
                    let zeros = vec![0u32; *num_32bit_values as usize];
                    cmd.compute_root
                        .set_constants(i as u32, 0, &zeros)
                        .unwrap_or_else(|e| panic!("{label}: set_constants({i}): {e}"));
                }
            }
        }
        cmd.set_compute_pipeline(pso.clone());
        cmd.dispatch(1, 1, 1)
            .unwrap_or_else(|e| panic!("{label}: dispatch(1, 1, 1): {e}"));
        cmd.barrier();
    }

    // Write sentinel word after dispatches and copy to readback to verify GPU completion.
    let mut sentinel_uav = [0u8; 40];
    sentinel_uav[0..4].copy_from_slice(&R32_TYPELESS.to_le_bytes());
    sentinel_uav[4..8].copy_from_slice(&1u32.to_le_bytes()); // BUFFER
    sentinel_uav[8..16].copy_from_slice(&60u64.to_le_bytes()); // word 60 (offset 240)
    sentinel_uav[16..20].copy_from_slice(&4u32.to_le_bytes());
    sentinel_uav[32..36].copy_from_slice(&1u32.to_le_bytes()); // RAW
    cmd.clear_uav(&dummy_buf, Some(&sentinel_uav), ClearValue::Uint([0xCAFE_BABE, 0, 0, 0]), &[])
        .expect("write completion sentinel");
    cmd.copy_buffer(&readback_buf, 0, &dummy_buf, 0, 256)
        .expect("copy dummy_buf to readback_buf");
    cmd.close().expect("CommandList::close");

    let fence_val = queue.submit(&[&cmd.cb]);
    queue.wait_idle();
    assert!(
        queue.timeline.completed() >= fence_val,
        "GPU timeline should complete submitted fence value {fence_val}"
    );

    let rb_words = unsafe {
        std::slice::from_raw_parts(readback_buf.buffer().unwrap().contents().as_ptr() as *const u32, 64)
    };
    assert_eq!(rb_words[60], 0xCAFE_BABE, "GPU readback sentinel should match after compute dispatches");
    eprintln!(
        "pe_middleware_compute_dispatch_executes_on_gpu: executed {} PE compute shaders on Metal 4 GPU ({:?})",
        dispatched_psos.len(),
        dispatched_psos.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
    );
}


