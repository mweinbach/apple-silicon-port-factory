//! GPU test: D3D12 Hull + Domain tessellation (+ optional Geometry Shader) through the Metal
//! Shader Converter and Metal 4 object+mesh tessellation emulation (`GraphicsPipeline::new_with_tessellation`),
//! rendered offscreen through `CommandList` and verified on readback under `MTL_DEBUG_LAYER=1`.
//!
//! Fixtures: `tests/fixtures/tessellation.hlsl` (SM 6.6 with PSV0 v2 and non-"main" entry points).
//! Covers:
//! 1. Tri-domain SM 6.6 (`VS_Tri` + `HS_Tri` + `DS_Tri` + `PS_Color`): root constants in HS/DS,
//!    root SRV `StructuredBuffer<float4>` in DS (bind point 3), `DrawInstanced`, tess factor = 0
//!    culling, `DrawIndexedInstanced` (`R16_UINT` with non-zero start/base + `R32_UINT`), and
//!    `ExecuteIndirect` (`Work::Draw` + `Work::DrawIndexed` with count buffer & root-constant patch).
//! 2. Quad-domain SM 6.6 (`VS_Quad` + `HS_Quad` + `DS_Quad` + `PS_Color`, `fractional_even`,
//!    `PATCHLIST_4`): verifies bilinear quadrant gradients and tess factor = 0 culling.
//! 3. Chained Geometry Shader (`VS_Tri` + `HS_Tri` + `DS_Tri` + `GS_Swizzle` + `PS_Color`):
//!    verifies GS runs after DS by swapping R and B channels.
//! 4. Depth-only tessellation (`ps = None`, `D32_FLOAT` DSV): verifies tessellated depth writes.

use std::sync::Arc;

use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::metal::cmdlist::{CommandAllocator, CommandList, CopyLoc, TargetView};
use d3d12_metal::metal::indirect::{arg, IndirectLayout};
use d3d12_metal::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
use d3d12_metal::metal::queue::{ListType, Queue};
use d3d12_metal::metal::resource::Resource;
use d3d12_metal::state::{BlendDesc, DepthStencilState, RasterState};
use d3d12_metal::topology::{d3d as topo, topology_type};
use d3d12_metal::types::{dim, heap_type, resource_flags, HeapProperties, Rect, ResourceDesc};
use dxil_metal::dxbc;
use dxil_metal::root_signature::RootSignature;
use dxil_metal::InputElement;
use objc2_metal::*;

const VS_TRI: &[u8] = include_bytes!("fixtures/tess_tri.vs.dxil");
const HS_TRI: &[u8] = include_bytes!("fixtures/tess_tri.hs.dxil");
const DS_TRI: &[u8] = include_bytes!("fixtures/tess_tri.ds.dxil");
const VS_QUAD: &[u8] = include_bytes!("fixtures/tess_quad.vs.dxil");
const HS_QUAD: &[u8] = include_bytes!("fixtures/tess_quad.hs.dxil");
const DS_QUAD: &[u8] = include_bytes!("fixtures/tess_quad.ds.dxil");
const GS_SWIZZLE: &[u8] = include_bytes!("fixtures/tess_swizzle.gs.dxil");
const PS_COLOR: &[u8] = include_bytes!("fixtures/tess_color.ps.dxil");

const SIZE: u32 = 32;
const ROW_PITCH: u32 = 256;
const DXGI_FORMAT_R8G8B8A8_UNORM: u32 = 28;
const DXGI_FORMAT_R32G32B32A32_FLOAT: u32 = 2;
const DXGI_FORMAT_D32_FLOAT: u32 = 40;
const DXGI_FORMAT_R32_UINT: u32 = 42;
const DXGI_FORMAT_R16_UINT: u32 = 57;

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 4],
    col: [f32; 4],
}

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

fn root_signature() -> Arc<RootSignature> {
    RootSignature::from_blob(&signed(VS_TRI)).expect("embedded root signature")
}

fn input_layout() -> Vec<InputElement> {
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

fn graphics_desc<'a>(vs: &'a [u8], ps: Option<&'a [u8]>, rtv_formats: Vec<u32>, dsv_format: u32) -> GraphicsDesc<'a> {
    let mut blend = BlendDesc::default();
    blend.RenderTarget[0].RenderTargetWriteMask = 0xf;
    let mut depth_stencil = DepthStencilState::default();
    if dsv_format != 0 {
        depth_stencil.depth_write = true;
        depth_stencil.depth_compare = MTLCompareFunction::Less;
    }
    GraphicsDesc {
        vs,
        ps,
        blend,
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..Default::default() },
        depth_stencil,
        input_layout: input_layout(),
        strip_cut: 0,
        topology_type: topology_type::PATCH,
        rtv_formats,
        dsv_format,
        sample_count: 1,
    }
}

fn upload_buf<T: Copy>(data: &[T], label: &str) -> Resource {
    let byte_len = std::mem::size_of_val(data).max(16) as u64;
    let rd = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: byte_len,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let hp = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let r = Resource::new_committed(&rd, &hp, MTLTextureUsage::empty(), label).unwrap();
    if !data.is_empty() {
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr() as *const u8,
                r.buffer().unwrap().contents().as_ptr() as *mut u8,
                std::mem::size_of_val(data),
            );
        }
    }
    r
}

fn readback_buf(size: u64, label: &str) -> Resource {
    let rd = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: size,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let hp = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
    Resource::new_committed(&rd, &hp, MTLTextureUsage::empty(), label).unwrap()
}

fn rt_texture(label: &str) -> Resource {
    let rd = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: SIZE as u64,
        Height: SIZE,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    Resource::new_committed(&rd, &hp, MTLTextureUsage::empty(), label).unwrap()
}

fn dsv_texture(label: &str) -> Resource {
    let rd = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: SIZE as u64,
        Height: SIZE,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: DXGI_FORMAT_D32_FLOAT,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_DEPTH_STENCIL,
        ..Default::default()
    };
    let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    Resource::new_committed(&rd, &hp, MTLTextureUsage::empty(), label).unwrap()
}

fn f32s_to_u32s(v: [f32; 8]) -> [u32; 8] {
    v.map(f32::to_bits)
}

fn run(f: impl FnOnce(&mut CommandList)) {
    let q = Queue::new(ListType::Direct, "gpu_tess_q").unwrap();
    let a = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.begin(&a).unwrap();
    f(&mut l);
    l.close().unwrap();
    q.submit(&[&l.cb]);
    q.wait_idle();
}

fn read_rgba(rb: &Resource, x: u32, y: u32) -> [u8; 4] {
    let ptr = rb.buffer().unwrap().contents().as_ptr() as *const u8;
    let off = (y * ROW_PITCH + x * 4) as usize;
    unsafe { [ptr.add(off).read(), ptr.add(off + 1).read(), ptr.add(off + 2).read(), ptr.add(off + 3).read()] }
}

fn read_f32(rb: &Resource, x: u32, y: u32) -> f32 {
    let ptr = rb.buffer().unwrap().contents().as_ptr() as *const u8;
    let off = (y * ROW_PITCH + x * 4) as usize;
    unsafe { std::ptr::read_unaligned(ptr.add(off) as *const f32) }
}

fn setup_rt_pass(l: &mut CommandList, rt: &Resource, pso: &Arc<GraphicsPipeline>, rs: &Arc<RootSignature>) {
    let rtv = TargetView::rtv(rt, None).unwrap();
    unsafe { l.clear_rtv(&rtv, [0.0, 0.0, 0.0, 1.0], &[]).unwrap() };
    l.set_render_targets(vec![rtv], None);
    l.set_viewports(vec![MTLViewport {
        originX: 0.0,
        originY: 0.0,
        width: SIZE as f64,
        height: SIZE as f64,
        znear: 0.0,
        zfar: 1.0,
    }]);
    l.set_scissors(vec![Rect { left: 0, top: 0, right: SIZE as i32, bottom: SIZE as i32 }]);
    l.gfx_root.set_root_signature(rs.clone()).unwrap();
    l.set_graphics_pipeline(pso.clone());
}

fn copy_back_rgba(l: &mut CommandList, rb: &Resource, rt: &Resource) {
    let fp = PlacedFootprint {
        offset: 0,
        format: DXGI_FORMAT_R8G8B8A8_UNORM,
        width: SIZE,
        height: SIZE,
        depth: 1,
        row_pitch: ROW_PITCH,
        _pad: 0,
    };
    l.copy_texture_region(CopyLoc::Footprint(rb, fp), [0, 0, 0], CopyLoc::Subresource(rt, 0), None)
        .unwrap();
}

#[test]
fn tri_domain_draw_indexed_and_indirect() {
    if !ready() {
        return;
    }
    let (vs, hs, ds, ps) = (signed(VS_TRI), signed(HS_TRI), signed(DS_TRI), signed(PS_COLOR));
    // Verify fixtures exercise the PSV0-v2 !dx.entryPoints resolution path.
    assert_eq!(dxbc::Container::parse(&hs).unwrap().psv_entry_point(), None);
    assert_eq!(dxbc::Container::parse(&hs).unwrap().entry_point().as_deref(), Some("HS_Tri"));

    let rs = root_signature();
    let gdesc = graphics_desc(&vs, Some(&ps), vec![DXGI_FORMAT_R8G8B8A8_UNORM], 0);
    let pso = Arc::new(
        GraphicsPipeline::new_with_tessellation(rs.clone(), &gdesc, &hs, &ds, None, "gpu_tess_tri")
            .expect("tri tessellation PSO"),
    );
    assert!(pso.tess_emu.is_some());

    // Vertex 0 is a dummy sentinel so BaseVertexLocation = 1 is genuinely tested on indexed draws.
    let verts = [
        Vertex { pos: [0.0, 0.0, 0.0, 1.0], col: [0.0, 0.0, 0.0, 1.0] },
        Vertex { pos: [-1.0, -1.0, 0.5, 1.0], col: [1.0, 1.0, 1.0, 1.0] },
        Vertex { pos: [-1.0, 3.0, 0.5, 1.0], col: [1.0, 1.0, 1.0, 1.0] },
        Vertex { pos: [3.0, -1.0, 0.5, 1.0], col: [1.0, 1.0, 1.0, 1.0] },
    ];
    let vb = upload_buf(&verts, "tess_tri_vb");
    // Domain-shader SRV: adds green (0, 1, 0) when gColor.a == 1.0.
    let srv_buf = upload_buf(&[[0.0f32, 1.0, 0.0, 1.0]], "tess_tri_srv");
    let ib16 = upload_buf(&[99u16, 0, 1, 2], "tess_tri_ib16"); // start_index=1, base_vertex=1 -> [1,2,3]
    let ib32 = upload_buf(&[1u32, 2, 3], "tess_tri_ib32");
    let rt = rt_texture("tess_tri_rt");
    let rb = readback_buf((ROW_PITCH * SIZE) as u64, "tess_tri_rb");

    // 1. Non-indexed DrawInstanced: gColor=(1,0,0,1) + SRV=(0,1,0)*1 -> yellow (255, 255, 0, 255).
    run(|l| {
        setup_rt_pass(l, &rt, &pso, &rs);
        l.gfx.vbs[0] = (vb.gpu_va, std::mem::size_of_val(&verts) as u32, std::mem::size_of::<Vertex>() as u32);
        l.gfx.topology = topo::PATCHLIST_1 + 2;
        l.gfx_root.set_constants(0, 0, &f32s_to_u32s([3.0, 3.0, 3.0, 3.0, 1.0, 0.0, 0.0, 1.0])).unwrap();
        l.gfx_root.set_descriptor(1, srv_buf.gpu_va).unwrap();
        l.draw(3, 1, 1, 0).unwrap();
        copy_back_rgba(l, &rb, &rt);
    });
    assert_eq!(read_rgba(&rb, SIZE / 2, SIZE / 2), [255, 255, 0, 255], "DrawInstanced must combine root constants + DS SRV");

    // 2. Tessellation factor 0 culls the patch -> remains cleared black.
    run(|l| {
        setup_rt_pass(l, &rt, &pso, &rs);
        l.gfx.vbs[0] = (vb.gpu_va, std::mem::size_of_val(&verts) as u32, std::mem::size_of::<Vertex>() as u32);
        l.gfx.topology = topo::PATCHLIST_1 + 2;
        l.gfx_root.set_constants(0, 0, &f32s_to_u32s([0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])).unwrap();
        l.gfx_root.set_descriptor(1, srv_buf.gpu_va).unwrap();
        l.draw(3, 1, 1, 0).unwrap();
        copy_back_rgba(l, &rb, &rt);
    });
    assert_eq!(read_rgba(&rb, SIZE / 2, SIZE / 2), [0, 0, 0, 255], "tess factor 0 must cull patch");

    // 3. Indexed DrawIndexedInstanced (R16_UINT, start_index=1, base_vertex=1) -> pure blue (0, 0, 255, 255).
    run(|l| {
        setup_rt_pass(l, &rt, &pso, &rs);
        l.gfx.vbs[0] = (vb.gpu_va, std::mem::size_of_val(&verts) as u32, std::mem::size_of::<Vertex>() as u32);
        l.gfx.ib = Some((ib16.gpu_va, 8, DXGI_FORMAT_R16_UINT));
        l.gfx.topology = topo::PATCHLIST_1 + 2;
        l.gfx_root.set_constants(0, 0, &f32s_to_u32s([4.0, 4.0, 4.0, 4.0, 0.0, 0.0, 1.0, 0.0])).unwrap();
        l.gfx_root.set_descriptor(1, srv_buf.gpu_va).unwrap();
        l.draw_indexed(3, 1, 1, 1, 0).unwrap();
        copy_back_rgba(l, &rb, &rt);
    });
    assert_eq!(read_rgba(&rb, SIZE / 2, SIZE / 2), [0, 0, 255, 255], "R16_UINT DrawIndexedInstanced with offsets");

    // 4. Indexed DrawIndexedInstanced (R32_UINT) -> cyan (0, 255, 255, 255).
    run(|l| {
        setup_rt_pass(l, &rt, &pso, &rs);
        l.gfx.vbs[0] = (vb.gpu_va, std::mem::size_of_val(&verts) as u32, std::mem::size_of::<Vertex>() as u32);
        l.gfx.ib = Some((ib32.gpu_va, 12, DXGI_FORMAT_R32_UINT));
        l.gfx.topology = topo::PATCHLIST_1 + 2;
        l.gfx_root.set_constants(0, 0, &f32s_to_u32s([2.0, 2.0, 2.0, 2.0, 0.0, 0.0, 1.0, 1.0])).unwrap();
        l.gfx_root.set_descriptor(1, srv_buf.gpu_va).unwrap();
        l.draw_indexed(3, 1, 0, 0, 0).unwrap();
        copy_back_rgba(l, &rb, &rt);
    });
    assert_eq!(read_rgba(&rb, SIZE / 2, SIZE / 2), [0, 255, 255, 255], "R32_UINT DrawIndexedInstanced");

    // 5. ExecuteIndirect (Work::DrawIndexed + root constant patch + count buffer = 1) on the
    // reused render target: verifies both indirect root-constant patching and Fragment -> Blit
    // synchronization when a compute expansion encoder precedes the tessellation render pass.
    let layout = IndirectLayout::new(
        36, // 4 patched root constants (16B) + DrawIndexed args (20B)
        &[(arg::CONSTANT, [0, 4, 4]), (arg::DRAW_INDEXED, [0; 3])],
    )
    .unwrap();
    let count_buf = upload_buf(&[1u32], "tess_ind_count");
    let cases: [([f32; 4], [u8; 4]); 3] = [
        ([1.0, 0.0, 1.0, 0.0], [255, 0, 255, 255]), // magenta
        ([1.0, 1.0, 0.0, 0.0], [255, 255, 0, 255]), // yellow
        ([0.0, 1.0, 0.0, 0.0], [0, 255, 0, 255]),   // green
    ];
    for (col, want) in cases {
        let ind_cmd: [u32; 9] = [
            col[0].to_bits(), col[1].to_bits(), col[2].to_bits(), col[3].to_bits(),
            3, 1, 0, 0, 0, // index_count=3, instances=1, start_idx=0, base_vtx=0, start_inst=0
        ];
        let ind_buf = upload_buf(&ind_cmd, "tess_ind_args");
        run(|l| {
            setup_rt_pass(l, &rt, &pso, &rs);
            l.gfx.vbs[0] = (vb.gpu_va, std::mem::size_of_val(&verts) as u32, std::mem::size_of::<Vertex>() as u32);
            l.gfx.ib = Some((ib32.gpu_va, 12, DXGI_FORMAT_R32_UINT));
            l.gfx.topology = topo::PATCHLIST_1 + 2;
            l.gfx_root.set_constants(0, 0, &f32s_to_u32s([2.0, 2.0, 2.0, 2.0, 0.0, 0.0, 0.0, 0.0])).unwrap();
            l.gfx_root.set_descriptor(1, srv_buf.gpu_va).unwrap();
            l.execute_indirect(&layout, 1, ind_buf.gpu_va, None, Some(count_buf.gpu_va)).unwrap();
            copy_back_rgba(l, &rb, &rt);
        });
        assert_eq!(read_rgba(&rb, SIZE / 2, SIZE / 2), want, "ExecuteIndirect DrawIndexed with root constant patch");
    }

    // 6. ExecuteIndirect (Work::Draw, non-indexed) -> pure red (255, 0, 0, 255).
    let draw_layout = IndirectLayout::new(16, &[(arg::DRAW, [0; 3])]).unwrap();
    let draw_ind_buf = upload_buf(&[3u32, 1, 1, 0], "tess_draw_ind_args");
    run(|l| {
        setup_rt_pass(l, &rt, &pso, &rs);
        l.gfx.vbs[0] = (vb.gpu_va, std::mem::size_of_val(&verts) as u32, std::mem::size_of::<Vertex>() as u32);
        l.gfx.topology = topo::PATCHLIST_1 + 2;
        l.gfx_root.set_constants(0, 0, &f32s_to_u32s([2.0, 2.0, 2.0, 2.0, 1.0, 0.0, 0.0, 0.0])).unwrap();
        l.gfx_root.set_descriptor(1, srv_buf.gpu_va).unwrap();
        l.execute_indirect(&draw_layout, 1, draw_ind_buf.gpu_va, None, None).unwrap();
        copy_back_rgba(l, &rb, &rt);
    });
    assert_eq!(read_rgba(&rb, SIZE / 2, SIZE / 2), [255, 0, 0, 255], "ExecuteIndirect non-indexed Draw");
}

#[test]
fn quad_domain_fractional_even() {
    if !ready() {
        return;
    }
    let (vs, hs, ds, ps) = (signed(VS_QUAD), signed(HS_QUAD), signed(DS_QUAD), signed(PS_COLOR));
    let rs = root_signature();
    let gdesc = graphics_desc(&vs, Some(&ps), vec![DXGI_FORMAT_R8G8B8A8_UNORM], 0);
    let pso = Arc::new(
        GraphicsPipeline::new_with_tessellation(rs.clone(), &gdesc, &hs, &ds, None, "gpu_tess_quad")
            .expect("quad tessellation PSO"),
    );

    // Fullscreen quad: TL red, TR green, BL blue, BR white (D3D clip: y=+1 top, y=-1 bottom).
    let verts = [
        Vertex { pos: [-1.0, 1.0, 0.5, 1.0], col: [1.0, 0.0, 0.0, 1.0] },
        Vertex { pos: [1.0, 1.0, 0.5, 1.0], col: [0.0, 1.0, 0.0, 1.0] },
        Vertex { pos: [-1.0, -1.0, 0.5, 1.0], col: [0.0, 0.0, 1.0, 1.0] },
        Vertex { pos: [1.0, -1.0, 0.5, 1.0], col: [1.0, 1.0, 1.0, 1.0] },
    ];
    let vb = upload_buf(&verts, "tess_quad_vb");
    let rt = rt_texture("tess_quad_rt");
    let rb = readback_buf((ROW_PITCH * SIZE) as u64, "tess_quad_rb");

    run(|l| {
        setup_rt_pass(l, &rt, &pso, &rs);
        l.gfx.vbs[0] = (vb.gpu_va, std::mem::size_of_val(&verts) as u32, std::mem::size_of::<Vertex>() as u32);
        l.gfx.topology = topo::PATCHLIST_1 + 3; // 4-control-point patchlist
        l.gfx_root.set_constants(0, 0, &f32s_to_u32s([4.5, 4.5, 4.5, 4.5, 1.0, 1.0, 1.0, 0.0])).unwrap();
        l.draw(4, 1, 0, 0).unwrap();
        copy_back_rgba(l, &rb, &rt);
    });
    let tl = read_rgba(&rb, 1, 1);
    let tr = read_rgba(&rb, SIZE - 2, 1);
    let bl = read_rgba(&rb, 1, SIZE - 2);
    let br = read_rgba(&rb, SIZE - 2, SIZE - 2);
    assert!(tl[0] > 200 && tl[1] < 50 && tl[2] < 50, "TL must be red, got {tl:?}");
    assert!(tr[1] > 200 && tr[0] < 50 && tr[2] < 50, "TR must be green, got {tr:?}");
    assert!(bl[2] > 200 && bl[0] < 50 && bl[1] < 50, "BL must be blue, got {bl:?}");
    assert!(br[0] > 200 && br[1] > 200 && br[2] > 200, "BR must be white, got {br:?}");
}

#[test]
fn tessellation_with_geometry_shader() {
    if !ready() {
        return;
    }
    let (vs, hs, ds, gs, ps) = (
        signed(VS_TRI),
        signed(HS_TRI),
        signed(DS_TRI),
        signed(GS_SWIZZLE),
        signed(PS_COLOR),
    );
    let rs = root_signature();
    let gdesc = graphics_desc(&vs, Some(&ps), vec![DXGI_FORMAT_R8G8B8A8_UNORM], 0);
    let pso = Arc::new(
        GraphicsPipeline::new_with_tessellation(rs.clone(), &gdesc, &hs, &ds, Some(&gs), "gpu_tess_gs")
            .expect("VS+HS+DS+GS+PS PSO"),
    );
    assert!(pso.tess_emu.as_ref().unwrap().gs.is_some());

    let verts = [
        Vertex { pos: [-1.0, -1.0, 0.5, 1.0], col: [1.0, 1.0, 1.0, 1.0] },
        Vertex { pos: [-1.0, 3.0, 0.5, 1.0], col: [1.0, 1.0, 1.0, 1.0] },
        Vertex { pos: [3.0, -1.0, 0.5, 1.0], col: [1.0, 1.0, 1.0, 1.0] },
    ];
    let vb = upload_buf(&verts, "tess_gs_vb");
    let srv_buf = upload_buf(&[[0.0f32; 4]], "tess_gs_srv");
    let rt = rt_texture("tess_gs_rt");
    let rb = readback_buf((ROW_PITCH * SIZE) as u64, "tess_gs_rb");

    // Input color is pure red (1, 0, 0); GS_Swizzle swaps R <-> B so output must be pure blue!
    run(|l| {
        setup_rt_pass(l, &rt, &pso, &rs);
        l.gfx.vbs[0] = (vb.gpu_va, std::mem::size_of_val(&verts) as u32, std::mem::size_of::<Vertex>() as u32);
        l.gfx.topology = topo::PATCHLIST_1 + 2;
        l.gfx_root.set_constants(0, 0, &f32s_to_u32s([3.0, 3.0, 3.0, 3.0, 1.0, 0.0, 0.0, 0.0])).unwrap();
        l.gfx_root.set_descriptor(1, srv_buf.gpu_va).unwrap();
        l.draw(3, 1, 0, 0).unwrap();
        copy_back_rgba(l, &rb, &rt);
    });
    assert_eq!(read_rgba(&rb, SIZE / 2, SIZE / 2), [0, 0, 255, 255], "GS_Swizzle after DS must swap red to blue");
}

#[test]
fn tessellation_depth_only() {
    if !ready() {
        return;
    }
    let (vs, hs, ds) = (signed(VS_TRI), signed(HS_TRI), signed(DS_TRI));
    let rs = root_signature();
    let gdesc = graphics_desc(&vs, None, vec![], DXGI_FORMAT_D32_FLOAT);
    let pso = Arc::new(
        GraphicsPipeline::new_with_tessellation(rs.clone(), &gdesc, &hs, &ds, None, "gpu_tess_depth")
            .expect("depth-only tessellation PSO"),
    );

    let verts = [
        Vertex { pos: [-1.0, -1.0, 0.25, 1.0], col: [1.0; 4] },
        Vertex { pos: [-1.0, 3.0, 0.25, 1.0], col: [1.0; 4] },
        Vertex { pos: [3.0, -1.0, 0.25, 1.0], col: [1.0; 4] },
    ];
    let vb = upload_buf(&verts, "tess_depth_vb");
    let srv_buf = upload_buf(&[[0.0f32; 4]], "tess_depth_srv");
    let dsv_tex = dsv_texture("tess_depth_dsv");
    let rb = readback_buf((ROW_PITCH * SIZE) as u64, "tess_depth_rb");

    run(|l| {
        let dsv = TargetView::dsv(&dsv_tex, None).unwrap();
        unsafe { l.clear_dsv(&dsv, true, 1.0, false, 0, &[]).unwrap() };
        l.set_render_targets(vec![], Some(dsv));
        l.set_viewports(vec![MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: SIZE as f64,
            height: SIZE as f64,
            znear: 0.0,
            zfar: 1.0,
        }]);
        l.set_scissors(vec![Rect { left: 0, top: 0, right: SIZE as i32, bottom: SIZE as i32 }]);
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.set_graphics_pipeline(pso.clone());
        l.gfx.vbs[0] = (vb.gpu_va, std::mem::size_of_val(&verts) as u32, std::mem::size_of::<Vertex>() as u32);
        l.gfx.topology = topo::PATCHLIST_1 + 2;
        l.gfx_root.set_constants(0, 0, &f32s_to_u32s([2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 0.0])).unwrap();
        l.gfx_root.set_descriptor(1, srv_buf.gpu_va).unwrap();
        l.draw(3, 1, 0, 0).unwrap();
        let fp = PlacedFootprint {
            offset: 0,
            format: DXGI_FORMAT_D32_FLOAT,
            width: SIZE,
            height: SIZE,
            depth: 1,
            row_pitch: ROW_PITCH,
            _pad: 0,
        };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&dsv_tex, 0), None)
            .unwrap();
    });
    let depth = read_f32(&rb, SIZE / 2, SIZE / 2);
    assert!((depth - 0.25).abs() < 1e-4, "depth-only tessellation must write z=0.25, got {depth}");
}
