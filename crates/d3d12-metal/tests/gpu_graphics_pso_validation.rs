//! Metal 4 GPU integration tests for render-pipeline color attachment and blend validation
//! (`GraphicsPipeline::new`, `GraphicsPipeline::new_with_tessellation`, `MeshPipeline::new`)
//! under `MTL_DEBUG_LAYER=1`.
//!
//! Run with:
//! `MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_graphics_pso_validation --target aarch64-apple-darwin -- --test-threads=1`

use std::sync::Arc;

use d3d12_metal::mesh::{MeshDesc, MeshPipeline};
use d3d12_metal::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
use d3d12_metal::state::{blend, BlendDesc, DepthStencilState, RasterState, RenderTargetBlendDesc};
use d3d12_metal::topology::topology_type;
use dxil_metal::dxbc;
use dxil_metal::root_signature::{RootSignature, RootSignatureDesc, RootSignatureVersion};
use dxil_metal::InputElement;
use objc2_metal::*;

const VS_FULLSCREEN: &[u8] = include_bytes!("../../dxil-metal/tests/fixtures/entry_v2.vs.dxil");
const VS_TRI: &[u8] = include_bytes!("fixtures/tess_tri.vs.dxil");
const HS_TRI: &[u8] = include_bytes!("fixtures/tess_tri.hs.dxil");
const DS_TRI: &[u8] = include_bytes!("fixtures/tess_tri.ds.dxil");
const PS_COLOR: &[u8] = include_bytes!("fixtures/tess_color.ps.dxil");
const AS_MESH: &[u8] = include_bytes!("fixtures/mesh.as.dxil");
const MS_DIRECT: &[u8] = include_bytes!("fixtures/mesh_direct.ms.dxil");
const PS_MESH: &[u8] = include_bytes!("fixtures/mesh.ps.dxil");

const DXGI_FORMAT_UNKNOWN: u32 = 0;
const DXGI_FORMAT_R32G32B32A32_FLOAT: u32 = 2;
const DXGI_FORMAT_R32G32B32_FLOAT: u32 = 6;
const DXGI_FORMAT_R32G32_UINT: u32 = 17;
const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: u32 = 20;
const DXGI_FORMAT_R8G8B8A8_TYPELESS: u32 = 27;
const DXGI_FORMAT_R8G8B8A8_UNORM: u32 = 28;
const DXGI_FORMAT_R16G16_FLOAT: u32 = 34;
const DXGI_FORMAT_D32_FLOAT: u32 = 40;
const DXGI_FORMAT_R32_FLOAT: u32 = 41;
const DXGI_FORMAT_R32_UINT: u32 = 42;
const DXGI_FORMAT_R16_FLOAT: u32 = 54;
const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: u32 = 67;
const DXGI_FORMAT_BC1_UNORM: u32 = 71;

fn signed(b: &[u8]) -> Vec<u8> {
    let mut v = b.to_vec();
    dxbc::sign_container(&mut v).unwrap();
    v
}

fn ready() -> bool {
    if !dxil_metal::converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return false;
    }
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return false;
    }
    true
}

fn empty_root_signature() -> Arc<RootSignature> {
    RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: Vec::new(),
        static_samplers: Vec::new(),
        flags: 0,
    })
    .expect("empty root signature")
}

fn broadcast_alpha_blend() -> BlendDesc {
    let mut d = BlendDesc::default();
    d.IndependentBlendEnable = 0;
    d.RenderTarget[0] = RenderTargetBlendDesc {
        BlendEnable: 1,
        LogicOpEnable: 0,
        SrcBlend: blend::ONE,
        DestBlend: blend::INV_SRC_ALPHA,
        BlendOp: 1,
        SrcBlendAlpha: blend::INV_DEST_ALPHA,
        DestBlendAlpha: blend::ONE,
        BlendOpAlpha: 1,
        LogicOp: 4,
        RenderTargetWriteMask: 0xF,
    };
    d
}

fn tess_input_layout() -> Vec<InputElement> {
    vec![
        InputElement {
            semantic_name: "POSITION".into(),
            semantic_index: 0,
            format: DXGI_FORMAT_R32G32B32A32_FLOAT,
            input_slot: 0,
            aligned_byte_offset: 0,
            input_slot_class: 0,
            instance_data_step_rate: 0,
        },
        InputElement {
            semantic_name: "COLOR".into(),
            semantic_index: 0,
            format: DXGI_FORMAT_R32G32B32A32_FLOAT,
            input_slot: 0,
            aligned_byte_offset: 16,
            input_slot_class: 0,
            instance_data_step_rate: 0,
        },
    ]
}

#[test]
fn graphics_tess_and_mesh_psos_disable_blend_on_integer_mrts_without_metal_abort() {
    if !ready() {
        return;
    }

    let mixed_mrts = vec![
        DXGI_FORMAT_R32_FLOAT,
        DXGI_FORMAT_R16G16_FLOAT,
        DXGI_FORMAT_R32G32_UINT,
        DXGI_FORMAT_R32G32_UINT,
        DXGI_FORMAT_R32_UINT,
    ];

    // 1. Standard GraphicsPipeline::new (VS-only): Metal validates colorAttachments blendability
    //    even without a fragment function, so enabling blend on UINT slots would SIGABRT.
    let rs = empty_root_signature();
    let vs = signed(VS_FULLSCREEN);
    let desc = GraphicsDesc {
        vs: &vs,
        ps: None,
        blend: broadcast_alpha_blend(),
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..Default::default() },
        depth_stencil: DepthStencilState::default(),
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: topology_type::TRIANGLE,
        rtv_formats: mixed_mrts.clone(),
        dsv_format: DXGI_FORMAT_D32_FLOAT_S8X24_UINT,
        sample_count: 1,
    };
    let pso = GraphicsPipeline::new(rs.clone(), &desc, "mixed_float_uint_mrt_gfx")
        .expect("mixed float+uint MRT graphics PSO must build cleanly under MTL_DEBUG_LAYER=1");
    assert!(pso.blend.attachments[0].enabled);
    assert!(pso.blend.attachments[1].enabled);
    assert!(!pso.blend.attachments[2].enabled);
    assert!(!pso.blend.attachments[3].enabled);
    assert!(!pso.blend.attachments[4].enabled);
    assert_eq!(pso.blend.attachments[2].write_mask, MTLColorWriteMask::All);

    // IndependentBlendEnable = TRUE with BlendEnable = TRUE explicitly set on a UINT target
    // must also sanitize without aborting Metal while keeping the write mask intact.
    let mut indep = BlendDesc::default();
    indep.IndependentBlendEnable = 1;
    indep.RenderTarget[0] = RenderTargetBlendDesc {
        BlendEnable: 1,
        SrcBlend: blend::SRC_ALPHA,
        DestBlend: blend::INV_SRC_ALPHA,
        BlendOp: 1,
        SrcBlendAlpha: blend::ONE,
        DestBlendAlpha: blend::ZERO,
        BlendOpAlpha: 1,
        RenderTargetWriteMask: 0xF,
        ..Default::default()
    };
    indep.RenderTarget[1] = RenderTargetBlendDesc {
        BlendEnable: 1,
        SrcBlend: blend::ONE,
        DestBlend: blend::ONE,
        BlendOp: 1,
        SrcBlendAlpha: blend::ONE,
        DestBlendAlpha: blend::ONE,
        BlendOpAlpha: 1,
        RenderTargetWriteMask: 0x3, // R | G
        ..Default::default()
    };
    let desc_indep = GraphicsDesc {
        blend: indep,
        rtv_formats: vec![DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R32_UINT],
        ..desc
    };
    let pso_indep = GraphicsPipeline::new(rs.clone(), &desc_indep, "indep_uint_mrt")
        .expect("independent blend with UINT RT must build cleanly");
    assert!(pso_indep.blend.attachments[0].enabled);
    assert!(!pso_indep.blend.attachments[1].enabled);
    assert_eq!(
        pso_indep.blend.attachments[1].write_mask,
        MTLColorWriteMask::Red | MTLColorWriteMask::Green
    );

    // Depth-only PSO (ps = None) carrying color RTV formats with write_mask = None must succeed.
    let mut no_write = BlendDesc::default();
    no_write.RenderTarget[0].RenderTargetWriteMask = 0;
    let desc_depth_rtvs = GraphicsDesc {
        vs: &vs,
        ps: None,
        blend: no_write,
        sample_mask: u32::MAX,
        raster: RasterState::default(),
        depth_stencil: DepthStencilState::default(),
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: topology_type::TRIANGLE,
        rtv_formats: vec![DXGI_FORMAT_R16_FLOAT, DXGI_FORMAT_R16_FLOAT],
        dsv_format: DXGI_FORMAT_D32_FLOAT,
        sample_count: 1,
    };
    GraphicsPipeline::new(rs, &desc_depth_rtvs, "depth_only_with_rtvs")
        .expect("depth-only PSO with R16_FLOAT RTVs must build cleanly");

    // 2. Tessellation PSO (with real fragment shader) on the exact mixed float+uint MRT list.
    let tess_rs = RootSignature::from_blob(&signed(VS_TRI)).expect("tess root signature");
    let vs_tri = signed(VS_TRI);
    let hs_tri = signed(HS_TRI);
    let ds_tri = signed(DS_TRI);
    let ps_color = signed(PS_COLOR);
    let tess_desc = GraphicsDesc {
        vs: &vs_tri,
        ps: Some(&ps_color),
        blend: broadcast_alpha_blend(),
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..Default::default() },
        depth_stencil: DepthStencilState::default(),
        input_layout: tess_input_layout(),
        strip_cut: 0,
        topology_type: topology_type::PATCH,
        rtv_formats: mixed_mrts.clone(),
        dsv_format: DXGI_FORMAT_D32_FLOAT_S8X24_UINT,
        sample_count: 1,
    };
    let tess_pso = GraphicsPipeline::new_with_tessellation(
        tess_rs,
        &tess_desc,
        &hs_tri,
        &ds_tri,
        None,
        "tess_mixed_float_uint_mrt",
    )
    .expect("tessellation PSO with mixed float+uint MRT must build cleanly");
    assert!(tess_pso.blend.attachments[0].enabled);
    assert!(tess_pso.blend.attachments[1].enabled);
    assert!(!tess_pso.blend.attachments[2].enabled);
    assert!(!tess_pso.blend.attachments[3].enabled);
    assert!(!tess_pso.blend.attachments[4].enabled);

    // 3. MeshPipeline (with real fragment shader) on the exact mixed float+uint MRT list.
    let mesh_rs = RootSignature::from_blob(&signed(AS_MESH)).expect("mesh root signature");
    let ms = signed(MS_DIRECT);
    let ps_mesh = signed(PS_MESH);
    let mesh_desc = MeshDesc {
        amplification: None,
        mesh: &ms,
        pixel: Some(&ps_mesh),
        blend: broadcast_alpha_blend(),
        sample_mask: u32::MAX,
        raster: RasterState::default(),
        depth_stencil: DepthStencilState::default(),
        rtv_formats: mixed_mrts,
        dsv_format: DXGI_FORMAT_D32_FLOAT_S8X24_UINT,
        sample_count: 1,
    };
    let mesh_pso = MeshPipeline::new(mesh_rs, &mesh_desc, "mesh_mixed_float_uint_mrt")
        .expect("mesh PSO with mixed float+uint MRT must build cleanly");
    assert!(mesh_pso.blend.attachments[0].enabled);
    assert!(mesh_pso.blend.attachments[1].enabled);
    assert!(!mesh_pso.blend.attachments[2].enabled);
    assert!(!mesh_pso.blend.attachments[3].enabled);
    assert!(!mesh_pso.blend.attachments[4].enabled);
}

#[test]
fn graphics_and_mesh_psos_reject_invalid_formats_samples_and_dual_source_without_abort() {
    if !ready() {
        return;
    }
    let rs = empty_root_signature();
    let vs = signed(VS_FULLSCREEN);
    let base = || GraphicsDesc {
        vs: &vs,
        ps: None,
        blend: broadcast_alpha_blend(),
        sample_mask: u32::MAX,
        raster: RasterState::default(),
        depth_stencil: DepthStencilState::default(),
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: topology_type::TRIANGLE,
        rtv_formats: vec![DXGI_FORMAT_R8G8B8A8_UNORM],
        dsv_format: 0,
        sample_count: 1,
    };

    // Non-color-renderable RTV formats must return Err instead of aborting Metal validation.
    for bad_fmt in [
        DXGI_FORMAT_BC1_UNORM,
        DXGI_FORMAT_D32_FLOAT,
        DXGI_FORMAT_R32G32B32_FLOAT,
        DXGI_FORMAT_R8G8B8A8_TYPELESS,
        DXGI_FORMAT_R9G9B9E5_SHAREDEXP,
        999,
    ] {
        let mut d = base();
        d.rtv_formats = vec![bad_fmt];
        assert!(
            GraphicsPipeline::new(rs.clone(), &d, "bad_rtv").is_err(),
            "non-RT format {bad_fmt} must return Err"
        );
    }

    // >8 RTVs and unsupported sample counts must return Err cleanly.
    let mut too_many = base();
    too_many.rtv_formats = vec![DXGI_FORMAT_R8G8B8A8_UNORM; 9];
    assert!(GraphicsPipeline::new(rs.clone(), &too_many, "too_many_rtvs").is_err());

    let mut bad_samples = base();
    bad_samples.sample_count = 3;
    assert!(GraphicsPipeline::new(rs.clone(), &bad_samples, "bad_sample_count").is_err());

    // Dual-source blending with >1 active RT must return Err instead of aborting Metal.
    let mut dual_blend = BlendDesc::default();
    dual_blend.RenderTarget[0] = RenderTargetBlendDesc {
        BlendEnable: 1,
        SrcBlend: blend::ONE,
        DestBlend: blend::INV_SRC1_COLOR,
        BlendOp: 1,
        SrcBlendAlpha: blend::ONE,
        DestBlendAlpha: blend::ZERO,
        BlendOpAlpha: 1,
        RenderTargetWriteMask: 0xF,
        ..Default::default()
    };
    let mut bad_dual = base();
    bad_dual.blend = dual_blend;
    bad_dual.rtv_formats = vec![DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R8G8B8A8_UNORM];
    assert!(GraphicsPipeline::new(rs.clone(), &bad_dual, "bad_dual_source_mrt").is_err());

    // Sparse UNKNOWN slot in MRT with blend enabled must succeed.
    let mut sparse = base();
    sparse.rtv_formats = vec![DXGI_FORMAT_UNKNOWN, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R32_UINT];
    let sparse_pso = GraphicsPipeline::new(rs, &sparse, "sparse_mrt")
        .expect("sparse UNKNOWN + float + uint MRT must build cleanly");
    assert!(sparse_pso.blend.attachments[1].enabled);
    assert!(!sparse_pso.blend.attachments[2].enabled);

    // MeshPipeline rejects invalid RTV formats and unsupported sample counts cleanly.
    let mesh_rs = RootSignature::from_blob(&signed(AS_MESH)).expect("mesh root signature");
    let ms = signed(MS_DIRECT);
    let ps_mesh = signed(PS_MESH);
    let mesh_bad_fmt = MeshDesc {
        amplification: None,
        mesh: &ms,
        pixel: Some(&ps_mesh),
        blend: broadcast_alpha_blend(),
        sample_mask: u32::MAX,
        raster: RasterState::default(),
        depth_stencil: DepthStencilState::default(),
        rtv_formats: vec![DXGI_FORMAT_D32_FLOAT],
        dsv_format: 0,
        sample_count: 1,
    };
    assert!(MeshPipeline::new(mesh_rs.clone(), &mesh_bad_fmt, "mesh_bad_rtv").is_err());
    let mesh_bad_samples = MeshDesc {
        rtv_formats: vec![DXGI_FORMAT_R8G8B8A8_UNORM],
        sample_count: 3,
        ..mesh_bad_fmt
    };
    assert!(MeshPipeline::new(mesh_rs, &mesh_bad_samples, "mesh_bad_samples").is_err());
}
