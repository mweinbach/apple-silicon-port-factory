//! GPU test: D3D12 mesh-shader pipelines (AS + MS + PS, and MS + PS) through the Metal Shader
//! Converter and a Metal 4 mesh render pipeline, rendered offscreen and read back.
//!
//! Fixture: tests/fixtures/mesh.hlsl (our own shader). Four quads, one per quadrant; the
//! amplification shader culls those whose bit is set in root constant 0, and root constant 1 is
//! the grey level of quad 3. Native arm64 only; skips without a Metal 4 device or the converter.

use std::ptr::NonNull;
use std::sync::Arc;

use d3d12_metal::mesh::{self, MeshDesc, MeshPipeline};
use d3d12_metal::state::{BlendDesc, DepthStencilState, RasterState};
use dxil_metal::dxbc;
use dxil_metal::root_signature::RootSignature;
use dxil_metal::StageInfo;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

const AS: &[u8] = include_bytes!("fixtures/mesh.as.dxil");
const MS: &[u8] = include_bytes!("fixtures/mesh.ms.dxil");
const MS_DIRECT: &[u8] = include_bytes!("fixtures/mesh_direct.ms.dxil");
const PS: &[u8] = include_bytes!("fixtures/mesh.ps.dxil");

const SIZE: usize = 64;
const DXGI_FORMAT_R8G8B8A8_UNORM: u32 = 28;
const CLEAR: [u8; 4] = [0, 0, 0, 255];

/// Fixtures are written unsigned by dxc on macOS; sign them like the game's shaders are.
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
    RootSignature::from_blob(&signed(AS)).expect("embedded root signature")
}

fn desc<'a>(amplification: Option<&'a [u8]>, mesh: &'a [u8], pixel: &'a [u8]) -> MeshDesc<'a> {
    let mut blend = BlendDesc::default();
    blend.RenderTarget[0].RenderTargetWriteMask = 0xf;
    MeshDesc {
        amplification,
        mesh,
        pixel: Some(pixel),
        blend,
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..Default::default() },
        depth_stencil: DepthStencilState::default(),
        rtv_formats: vec![DXGI_FORMAT_R8G8B8A8_UNORM],
        dsv_format: 0,
        sample_count: 1,
    }
}

struct Target {
    tex: Retained<ProtocolObject<dyn MTLTexture>>,
}

/// Renders one `DispatchMesh(groups, 1, 1)` (from a GPU argument buffer when `indirect`)
/// with root constants `[cull_mask, grey, 0, 0]` and returns the RGBA8 pixels.
fn render_with(p: &MeshPipeline, groups: u32, indirect: bool, cull_mask: u32, grey: u32) -> Vec<[u8; 4]> {
    let d = d3d12_metal::metal::device().unwrap();
    let dev = &d.dev;

    let td = unsafe { MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(MTLPixelFormat::RGBA8Unorm, SIZE, SIZE, false) };
    td.setUsage(MTLTextureUsage::RenderTarget);
    td.setStorageMode(MTLStorageMode::Shared);
    let target = Target { tex: dev.newTextureWithDescriptor(&td).expect("texture") };

    // Top-level argument buffer: root constants (param 0).
    let layout = p.rs.layout();
    let mut tlab = layout.new_argument_buffer();
    tlab.set_constants(layout, 0, 0, &[cull_mask, grey, 0, 0]);
    let ab = dev.newBufferWithLength_options(tlab.bytes.len().max(16), MTLResourceOptions::StorageModeShared).expect("buffer");
    unsafe { std::ptr::copy_nonoverlapping(tlab.bytes.as_ptr(), ab.contents().as_ptr() as *mut u8, tlab.bytes.len()) };

    // D3D12_DISPATCH_MESH_ARGUMENTS, at a 4-byte (not 16-byte) aligned offset.
    let args = dev.newBufferWithLength_options(64, MTLResourceOptions::StorageModeShared).expect("buffer");
    let words = [0u32, groups, 1, 1];
    unsafe { std::ptr::copy_nonoverlapping(words.as_ptr(), args.contents().as_ptr() as *mut u32, 4) };

    let rsd = MTLResidencySetDescriptor::new();
    let residency = dev.newResidencySetWithDescriptor_error(&rsd).expect("residency set");
    residency.addAllocation(ProtocolObject::from_ref(&*target.tex));
    residency.addAllocation(ProtocolObject::from_ref(&*ab));
    residency.addAllocation(ProtocolObject::from_ref(&*args));
    residency.commit();

    let queue = dev.newMTL4CommandQueue().expect("queue");
    let alloc = dev.newCommandAllocator().expect("allocator");
    let cb = dev.newCommandBuffer().expect("command buffer");
    let table = d3d12_metal::metal::binding::new_argument_table("gpu_mesh").expect("argument table");
    cb.beginCommandBufferWithAllocator(&alloc);
    cb.useResidencySet(&residency);

    let rp = MTL4RenderPassDescriptor::new();
    let ca = unsafe { rp.colorAttachments().objectAtIndexedSubscript(0) };
    ca.setTexture(Some(&target.tex));
    ca.setLoadAction(MTLLoadAction::Clear);
    ca.setStoreAction(MTLStoreAction::Store);
    let c = CLEAR.map(|v| v as f64 / 255.0);
    ca.setClearColor(MTLClearColor { red: c[0], green: c[1], blue: c[2], alpha: c[3] });
    let enc = cb.renderCommandEncoderWithDescriptor(&rp).expect("render encoder");
    unsafe { table.setAddress_atIndex(ab.gpuAddress(), dxil_metal::runtime::ARGUMENT_BUFFER_BIND_POINT as usize) };
    enc.setArgumentTable_atStages(&table, mesh::STAGES);
    mesh::apply_pipeline(&enc, p);
    enc.setViewport(MTLViewport { originX: 0.0, originY: 0.0, width: SIZE as f64, height: SIZE as f64, znear: 0.0, zfar: 1.0 });
    enc.setScissorRect(MTLScissorRect { x: 0, y: 0, width: SIZE, height: SIZE });
    if indirect {
        mesh::encode_dispatch_indirect(&enc, p, args.gpuAddress() + 4).expect("indirect dispatch");
    } else {
        mesh::encode_dispatch(&enc, p, groups, 1, 1).expect("dispatch");
    }
    enc.endEncoding();
    cb.endCommandBuffer();

    let ev = dev.newSharedEvent().expect("event");
    let mut cbs = [NonNull::from(&*cb)];
    unsafe { queue.commit_count(NonNull::new(cbs.as_mut_ptr()).unwrap(), 1) };
    queue.signalEvent_value(ProtocolObject::from_ref(&*ev), 1);
    assert!(ev.waitUntilSignaledValue_timeoutMS(1, 10_000), "GPU timeout");

    let mut px = vec![[0u8; 4]; SIZE * SIZE];
    let region = MTLRegion { origin: MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: SIZE, height: SIZE, depth: 1 } };
    unsafe { target.tex.getBytes_bytesPerRow_fromRegion_mipmapLevel(NonNull::new(px.as_mut_ptr().cast()).unwrap(), SIZE * 4, region, 0) };
    px
}

fn render(p: &MeshPipeline, groups: u32, cull_mask: u32, grey: u32) -> Vec<[u8; 4]> {
    render_with(p, groups, false, cull_mask, grey)
}

/// Pixel at the centre of quadrant `q` (0 top-left, 1 top-right, 2 bottom-left, 3 bottom-right).
fn quadrant(px: &[[u8; 4]], q: usize) -> [u8; 4] {
    let x = if q & 1 == 1 { SIZE * 3 / 4 } else { SIZE / 4 };
    let y = if q & 2 == 2 { SIZE * 3 / 4 } else { SIZE / 4 };
    px[y * SIZE + x]
}

fn expected(q: usize, grey: u8) -> [u8; 4] {
    match q {
        0 => [255, 0, 0, 255],
        1 => [0, 255, 0, 255],
        2 => [0, 0, 255, 255],
        _ => [grey, grey, grey, 255],
    }
}

fn assert_quadrants(px: &[[u8; 4]], visible: [bool; 4], grey: u8) {
    let got: Vec<[u8; 4]> = (0..4).map(|q| quadrant(px, q)).collect();
    let want: Vec<[u8; 4]> = (0..4).map(|q| if visible[q] { expected(q, grey) } else { CLEAR }).collect();
    assert_eq!(got, want, "quadrant centres (visible {visible:?})");
}

#[test]
fn amplification_mesh_pixel() {
    if !ready() {
        return;
    }
    let (a, m, p) = (signed(AS), signed(MS), signed(PS));
    let pso = MeshPipeline::new(root_signature(), &desc(Some(&a), &m, &p), "gpu_mesh AS+MS").expect("mesh pipeline");
    assert_eq!(pso.object_threads, Some([4, 1, 1]));
    assert_eq!(pso.mesh_threads, [4, 1, 1]);
    assert_eq!(pso.payload_bytes, 16, "payload is uint[4]");
    assert!(matches!(pso.mesh.reflection.info, StageInfo::Mesh { max_vertex_output_count: 4, max_primitive_output_count: 2, primitive_topology: 2, .. }), "{:?}", pso.mesh.reflection.info);

    // Everything visible; then cull quad 2 (the AS compacts the payload, so quad 3 moves to
    // mesh group 2); then cull all (DispatchMesh(0,...) from the AS).
    assert_quadrants(&render(&pso, 1, 0b0000, 128), [true; 4], 128);
    assert_quadrants(&render(&pso, 1, 0b0100, 200), [true, true, false, true], 200);
    assert_quadrants(&render(&pso, 1, 0b1111, 50), [false; 4], 50);
    assert_quadrants(&render_with(&pso, 1, true, 0b0001, 9), [false, true, true, true], 9);
}

#[test]
fn mesh_pixel_without_amplification() {
    if !ready() {
        return;
    }
    let (m, p) = (signed(MS_DIRECT), signed(PS));
    let pso = MeshPipeline::new(root_signature(), &desc(None, &m, &p), "gpu_mesh MS").expect("mesh pipeline");
    assert_eq!((pso.object_threads, pso.payload_bytes), (None, 0));
    // DispatchMesh(3, 1, 1) draws quads 0..3; the 4th stays clear.
    assert_quadrants(&render(&pso, 3, 0, 77), [true, true, true, false], 77);
    assert_quadrants(&render(&pso, 4, 0, 77), [true; 4], 77);
    // An empty dispatch is a no-op.
    assert_quadrants(&render(&pso, 0, 0, 77), [false; 4], 77);
    // ExecuteIndirect-style: the group count comes from a GPU buffer.
    assert_quadrants(&render_with(&pso, 2, true, 0, 77), [true, true, false, false], 77);
}
