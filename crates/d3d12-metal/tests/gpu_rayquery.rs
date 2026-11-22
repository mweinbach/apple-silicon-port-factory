//! GPU test: inline ray queries (DXR 1.1 `RayQuery`, cs_6_5) through the Metal Shader Converter
//! against Metal 4 acceleration structures, following docs/design/DXR.md:
//! a 64-byte `IRRaytracingAccelerationStructureGPUHeader` at the D3D VA, reached through an SRV
//! descriptor-table entry (`gpuVA` = header VA) and through a root SRV (the header VA itself).
//!
//! Scene: a BLAS quad (2 triangles, x,y in [0,1], z = 0, non-opaque) and a TLAS with 2 instances:
//! InstanceID 10 at the origin (mask 0x01, contribution 3) and InstanceID 20 translated by (2, 0, 1)
//! (mask 0x02, contribution 7). Rays go along +z from z = -1.
//!
//! Fixture: tests/fixtures/rayquery.hlsl (our own shader). Native arm64 only; skips without a
//! Metal 4 device with ray tracing, or without the converter.

use std::ptr::NonNull;
use std::sync::Arc;

use d3d12_metal::metal::pipeline::ComputePipeline;
use dxil_metal::dxbc;
use dxil_metal::root_signature::RootSignature;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

const CS: &[u8] = include_bytes!("fixtures/rayquery.cs.dxil");

type Buffer = Retained<ProtocolObject<dyn MTLBuffer>>;
type Accel = Retained<ProtocolObject<dyn MTLAccelerationStructure>>;

const RAY_FLAG_FORCE_OPAQUE: u32 = 0x1;
const RAY_FLAG_CULL_NON_OPAQUE: u32 = 0x80;
const COMMITTED_NOTHING: u32 = 0;
const COMMITTED_TRIANGLE_HIT: u32 = 1;

/// Ray origins (x, y); see `expected` for what each one hits.
const RAYS: [[f32; 2]; 8] = [[0.25, 0.25], [0.75, 0.75], [1.5, 0.5], [2.25, 0.25], [2.75, 0.75], [-0.5, 0.5], [0.25, 0.6], [2.5, 0.4]];

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
    let Some(d) = d3d12_metal::metal::device() else {
        eprintln!("skipping: no Metal 4 device");
        return false;
    };
    if !d.dev.supportsRaytracing() {
        eprintln!("skipping: device has no ray tracing support");
        return false;
    }
    true
}

fn shared(bytes: &[u8]) -> Buffer {
    let dev = &d3d12_metal::metal::device().unwrap().dev;
    let b = dev.newBufferWithLength_options(bytes.len().max(16), MTLResourceOptions::StorageModeShared).expect("buffer");
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), b.contents().as_ptr() as *mut u8, bytes.len()) };
    b
}

fn bytes_of<T: Copy>(v: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast(), std::mem::size_of_val(v)) }
}

fn range(b: &Buffer, offset: u64, len: u64) -> MTL4BufferRange {
    MTL4BufferRange { bufferAddress: b.gpuAddress() + offset, length: len }
}

/// `MTLIndirectAccelerationStructureInstanceDescriptor` (72 bytes), transform as written by D3D
/// (3x4 row-major; the TLAS uses `instanceTransformationMatrixLayout = RowMajor`).
#[repr(C)]
#[derive(Clone, Copy)]
struct IndirectInstance {
    transform: [[f32; 4]; 3],
    options: u32,
    mask: u32,
    ift_offset: u32,
    user_id: u32,
    accel_id: u64,
}
const _: () = assert!(std::mem::size_of::<IndirectInstance>() == 72);

fn resource_id(a: &Accel) -> u64 {
    let id = a.gpuResourceID();
    unsafe { std::mem::transmute::<MTLResourceID, u64>(id) }
}

struct Scene {
    _blas: Accel,
    tlas: Accel,
    /// Header (64 bytes) + instance contributions, i.e. the D3D "result buffer" at the TLAS VA.
    header: Buffer,
    keep: Vec<Buffer>,
}

fn new_accel(sizes: &MTLAccelerationStructureSizes) -> Accel {
    let dev = &d3d12_metal::metal::device().unwrap().dev;
    dev.newAccelerationStructureWithSize(sizes.accelerationStructureSize).expect("acceleration structure")
}

fn sizes(desc: &MTL4AccelerationStructureDescriptor) -> MTLAccelerationStructureSizes {
    let dev = &d3d12_metal::metal::device().unwrap().dev;
    dev.accelerationStructureSizesWithDescriptor(desc)
}

struct Gpu {
    queue: Retained<ProtocolObject<dyn MTL4CommandQueue>>,
    alloc: Retained<ProtocolObject<dyn MTL4CommandAllocator>>,
    residency: Retained<ProtocolObject<dyn MTLResidencySet>>,
}

impl Gpu {
    fn new() -> Gpu {
        let dev = &d3d12_metal::metal::device().unwrap().dev;
        let rsd = MTLResidencySetDescriptor::new();
        Gpu {
            queue: dev.newMTL4CommandQueue().expect("queue"),
            alloc: dev.newCommandAllocator().expect("allocator"),
            residency: dev.newResidencySetWithDescriptor_error(&rsd).expect("residency set"),
        }
    }

    fn resident(&self, a: &ProtocolObject<dyn MTLAllocation>) {
        self.residency.addAllocation(a);
    }

    /// Records `f` on one compute encoder, submits and waits.
    fn run(&self, f: impl FnOnce(&ProtocolObject<dyn MTL4ComputeCommandEncoder>)) {
        let d = d3d12_metal::metal::device().unwrap();
        let dev = &d.dev;
        self.residency.commit();
        let cb = dev.newCommandBuffer().expect("command buffer");
        cb.beginCommandBufferWithAllocator(&self.alloc);
        cb.useResidencySet(&self.residency);
        let enc = cb.computeCommandEncoder().expect("compute encoder");
        f(&enc);
        enc.endEncoding();
        cb.endCommandBuffer();
        d.commit_residency();
        let ev = dev.newSharedEvent().expect("event");
        let mut cbs = [NonNull::from(&*cb)];
        unsafe { self.queue.commit_count(NonNull::new(cbs.as_mut_ptr()).unwrap(), 1) };
        self.queue.signalEvent_value(ProtocolObject::from_ref(&*ev), 1);
        assert!(ev.waitUntilSignaledValue_timeoutMS(1, 10_000), "GPU timeout");
    }
}

fn barrier(enc: &ProtocolObject<dyn MTL4ComputeCommandEncoder>, after: MTLStages, before: MTLStages) {
    enc.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(after, before, MTL4VisibilityOptions::Device);
}

/// Builds the BLAS and TLAS directly (CPU-written Metal instance descriptors) and writes the
/// header for the TLAS on the CPU. This validates the MSC header model independently of the
/// D3D-facing code in `d3d12_metal::rt`.
fn build_scene_direct(gpu: &Gpu) -> Scene {
    let verts: [[f32; 3]; 4] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
    let indices: [u16; 6] = [0, 1, 2, 1, 3, 2];
    let vb = shared(bytes_of(&verts));
    let ib = shared(bytes_of(&indices));

    let tri = MTL4AccelerationStructureTriangleGeometryDescriptor::new();
    unsafe {
        tri.setVertexBuffer(range(&vb, 0, 48));
        tri.setVertexStride(12);
        tri.setIndexBuffer(range(&ib, 0, 12));
        tri.setTriangleCount(2);
    }
    tri.setVertexFormat(MTLAttributeFormat::Float3);
    tri.setIndexType(MTLIndexType::UInt16);
    tri.setOpaque(false);
    let blas_desc = MTL4PrimitiveAccelerationStructureDescriptor::new();
    let geoms = objc2_foundation::NSArray::from_retained_slice(&[Retained::into_super(tri)]);
    blas_desc.setGeometryDescriptors(Some(&geoms));
    let blas_sizes = sizes(&blas_desc);
    let blas = new_accel(&blas_sizes);

    let id = resource_id(&blas);
    let instances = [
        IndirectInstance { transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]], options: 0, mask: 0x01, ift_offset: 0, user_id: 10, accel_id: id },
        IndirectInstance { transform: [[1.0, 0.0, 0.0, 2.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 1.0]], options: 0, mask: 0x02, ift_offset: 0, user_id: 20, accel_id: id },
    ];
    let inst = shared(bytes_of(&instances));
    let tlas_desc = MTL4InstanceAccelerationStructureDescriptor::new();
    unsafe {
        tlas_desc.setInstanceDescriptorBuffer(range(&inst, 0, 144));
        tlas_desc.setInstanceDescriptorStride(72);
        tlas_desc.setInstanceCount(2);
    }
    tlas_desc.setInstanceDescriptorType(MTLAccelerationStructureInstanceDescriptorType::Indirect);
    tlas_desc.setInstanceTransformationMatrixLayout(MTLMatrixLayout::RowMajor);
    let tlas_sizes = sizes(&tlas_desc);
    let tlas = new_accel(&tlas_sizes);

    let scratch_len = blas_sizes.buildScratchBufferSize.max(tlas_sizes.buildScratchBufferSize).max(16);
    let scratch = shared(&vec![0u8; scratch_len]);

    // The D3D result buffer at the TLAS VA: header + contributions.
    let header = shared(&[0u8; 256]);
    let mut h = [0u64; 8];
    h[0] = resource_id(&tlas);
    h[1] = header.gpuAddress() + 64;
    let mut bytes = bytes_of(&h).to_vec();
    bytes.extend_from_slice(bytes_of(&[3u32, 7u32]));
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), header.contents().as_ptr() as *mut u8, bytes.len()) };

    for b in [&vb, &ib, &inst, &scratch, &header] {
        gpu.resident(ProtocolObject::from_ref(&**b));
    }
    gpu.resident(ProtocolObject::from_ref(&*blas));
    gpu.resident(ProtocolObject::from_ref(&*tlas));

    gpu.run(|enc| unsafe {
        enc.buildAccelerationStructure_descriptor_scratchBuffer(&blas, &blas_desc, range(&scratch, 0, scratch_len as u64));
        barrier(enc, MTLStages::AccelerationStructure, MTLStages::AccelerationStructure);
        enc.buildAccelerationStructure_descriptor_scratchBuffer(&tlas, &tlas_desc, range(&scratch, 0, scratch_len as u64));
    });
    Scene { _blas: blas, tlas, header, keep: vec![vb, ib, inst, scratch] }
}

/// One ray-query dispatch over `RAYS`; returns `Out` as 2 uint4 per ray.
fn trace(gpu: &Gpu, p: &ComputePipeline, scene: &Scene, use_root: bool, flags: u32, mask: u32) -> Vec<[u32; 4]> {
    let _ = &scene.tlas;
    // A one-entry descriptor heap: IRDescriptorTableEntry { gpuVA, textureViewID, metadata }.
    let heap = shared(bytes_of(&[scene.header.gpuAddress(), 0u64, 0u64]));
    let rays = shared(bytes_of(&RAYS));
    let out = shared(&[0xAAu8; 16 * 16]);

    let layout = p.rs.layout();
    let mut tlab = layout.new_argument_buffer();
    tlab.set_descriptor_table(layout, 0, heap.gpuAddress());
    tlab.set_root_descriptor(layout, 1, scene.header.gpuAddress());
    tlab.set_root_descriptor(layout, 2, out.gpuAddress());
    tlab.set_constants(layout, 3, 0, &[use_root as u32, flags, mask, 0]);
    tlab.set_root_descriptor(layout, 4, rays.gpuAddress());
    let ab = shared(&tlab.bytes);
    for b in [&heap, &rays, &out, &ab] {
        gpu.resident(ProtocolObject::from_ref(&**b));
    }

    let table = d3d12_metal::metal::binding::new_argument_table("gpu_rayquery").expect("argument table");
    gpu.run(|enc| {
        unsafe { table.setAddress_atIndex(ab.gpuAddress(), dxil_metal::runtime::ARGUMENT_BUFFER_BIND_POINT as usize) };
        enc.setArgumentTable(Some(&table));
        enc.setComputePipelineState(&p.pso);
        let tg = p.threadgroup;
        enc.dispatchThreadgroups_threadsPerThreadgroup(MTLSize { width: 1, height: 1, depth: 1 }, MTLSize { width: tg[0] as usize, height: tg[1] as usize, depth: tg[2] as usize });
    });
    unsafe { std::slice::from_raw_parts(out.contents().as_ptr() as *const [u32; 4], 16) }.to_vec()
}

/// What ray `i` should report: `None` for a miss, else (InstanceID, InstanceIndex, PrimitiveIndex,
/// t, barycentrics, contribution).
#[derive(Debug, PartialEq, Clone, Copy)]
struct Hit {
    id: u32,
    index: u32,
    prim: u32,
    t: f32,
    bary: [f32; 2],
    contribution: u32,
}

fn reference(i: usize, mask: u32) -> Option<Hit> {
    let [x, y] = RAYS[i];
    let (inst, lx, z) = if (0.0..=1.0).contains(&x) { (0u32, x, 0.0) } else if (2.0..=3.0).contains(&x) { (1, x - 2.0, 1.0) } else { return None };
    if !(0.0..=1.0).contains(&y) || mask & (1 << inst) == 0 {
        return None;
    }
    // Triangle 0 = (0,0),(1,0),(0,1); triangle 1 = (1,0),(1,1),(0,1). D3D barycentrics (u, v)
    // weight vertices 1 and 2.
    let (prim, bary) = if lx + y <= 1.0 { (0, [lx, y]) } else { (1, [lx + y - 1.0, 1.0 - lx]) };
    Some(Hit { id: [10, 20][inst as usize], index: inst, prim, t: 1.0 + z, bary, contribution: [3, 7][inst as usize] })
}

fn decode(out: &[[u32; 4]], i: usize) -> (u32, Option<Hit>, bool) {
    let (a, b) = (out[2 * i], out[2 * i + 1]);
    if a[0] != COMMITTED_TRIANGLE_HIT {
        return (a[0], None, false);
    }
    let hit = Hit { id: a[1], index: a[2], prim: a[3], t: f32::from_bits(b[0]), bary: [f32::from_bits(b[1]), f32::from_bits(b[2])], contribution: b[3] & 0x7fff_ffff };
    (a[0], Some(hit), b[3] >> 31 != 0)
}

fn close(a: Option<Hit>, b: Option<Hit>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            let eq = |x: f32, y: f32| (x - y).abs() < 1e-4;
            (a.id, a.index, a.prim, a.contribution) == (b.id, b.index, b.prim, b.contribution) && eq(a.t, b.t) && eq(a.bary[0], b.bary[0]) && eq(a.bary[1], b.bary[1])
        }
        _ => false,
    }
}

fn check(out: &[[u32; 4]], mask: u32, cull_all: bool, what: &str) {
    for i in 0..RAYS.len() {
        let want = if cull_all { None } else { reference(i, mask) };
        let (status, got, front) = decode(out, i);
        let want_status = if want.is_some() { COMMITTED_TRIANGLE_HIT } else { COMMITTED_NOTHING };
        assert_eq!(status, want_status, "{what}: ray {i} {:?} status (out {:?})", RAYS[i], &out[2 * i..2 * i + 2]);
        assert!(close(got, want), "{what}: ray {i} {:?}: got {got:?}, want {want:?}", RAYS[i]);
        // Looking along +z, both triangles wind counter-clockwise: back faces in D3D.
        if got.is_some() {
            assert!(!front, "{what}: ray {i} reports a front face");
        }
    }
}

#[test]
fn rayquery_direct_tlas() {
    if !ready() {
        return;
    }
    let rs = RootSignature::from_blob(&signed(CS)).expect("embedded root signature");
    let p = ComputePipeline::new(Arc::clone(&rs), &signed(CS), "gpu_rayquery").expect("compute pipeline");
    assert_eq!(p.threadgroup, [8, 1, 1]);
    let gpu = Gpu::new();
    let scene = build_scene_direct(&gpu);
    let _ = &scene.keep;

    check(&trace(&gpu, &p, &scene, false, 0, 0xFF), 0xFF, false, "table SRV");
    check(&trace(&gpu, &p, &scene, true, 0, 0xFF), 0xFF, false, "root SRV");
    check(&trace(&gpu, &p, &scene, true, 0, 0x01), 0x01, false, "mask 0x01");
    check(&trace(&gpu, &p, &scene, false, RAY_FLAG_FORCE_OPAQUE, 0xFF), 0xFF, false, "force opaque");
    check(&trace(&gpu, &p, &scene, false, RAY_FLAG_CULL_NON_OPAQUE, 0xFF), 0xFF, true, "cull non-opaque");
}

#[test]
fn rayquery_via_rt_accel() {
    if !ready() {
        return;
    }
    use d3d12_metal::metal::binding::{new_argument_table, UploadRing};
    use d3d12_metal::rt::accel::{encode_build, encode_copy, encode_postbuild, entry_info, evict_range, prebuild_info, result_size, stats, BuildArgs, Ctx};
    use d3d12_metal::rt::d3d::{build_flags, copy_mode, postbuild, Geometry, Inputs, InstanceDesc, PostbuildInfoDesc, TrianglesDesc};

    let rs = RootSignature::from_blob(&signed(CS)).expect("embedded root signature");
    let p = ComputePipeline::new(Arc::clone(&rs), &signed(CS), "gpu_rayquery_accel").expect("compute pipeline");
    let gpu = Gpu::new();
    let dev_res = &d3d12_metal::metal::device().unwrap().residency;
    gpu.queue.addResidencySet(dev_res);

    let verts: [[f32; 3]; 4] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
    let indices: [u16; 6] = [0, 1, 2, 1, 3, 2];
    let vb = shared(bytes_of(&verts));
    let ib = shared(bytes_of(&indices));
    gpu.resident(ProtocolObject::from_ref(&*vb));
    gpu.resident(ProtocolObject::from_ref(&*ib));

    let blas_inputs = Inputs::Bottom {
        flags: build_flags::ALLOW_COMPACTION | build_flags::PREFER_FAST_TRACE,
        geometries: vec![Geometry::Triangles {
            flags: 0,
            desc: TrianglesDesc {
                Transform3x4: 0,
                IndexFormat: 57,
                VertexFormat: 6,
                IndexCount: 6,
                VertexCount: 4,
                IndexBuffer: ib.gpuAddress(),
                VertexBufferStart: vb.gpuAddress(),
                VertexBufferStride: 12,
            },
        }],
    };
    let blas_pre = prebuild_info(&blas_inputs).expect("BLAS prebuild");
    assert_eq!(blas_pre.ResultDataMaxSizeInBytes, 256);

    let tlas_inputs_probe = Inputs::Top { flags: build_flags::ALLOW_UPDATE, count: 2, instances: 0, pointers: false };
    let tlas_pre = prebuild_info(&tlas_inputs_probe).expect("TLAS prebuild");
    assert_eq!(tlas_pre.ResultDataMaxSizeInBytes, 256);

    let blas_buf = shared(&vec![0u8; blas_pre.ResultDataMaxSizeInBytes as usize + 256]);
    let tlas_buf = shared(&vec![0u8; tlas_pre.ResultDataMaxSizeInBytes as usize + 256]);
    let clone_buf = shared(&vec![0u8; tlas_pre.ResultDataMaxSizeInBytes as usize + 256]);
    let compact_blas_buf = shared(&vec![0u8; blas_pre.ResultDataMaxSizeInBytes as usize + 256]);
    let empty_blas_buf = shared(&vec![0u8; 512]);
    let scratch_len = (blas_pre.ScratchDataSizeInBytes.max(tlas_pre.ScratchDataSizeInBytes) as usize + 4096).max(4096);
    let scratch = shared(&vec![0u8; scratch_len]);
    let pb_buf = shared(&[0u8; 64]);

    for b in [&blas_buf, &tlas_buf, &clone_buf, &compact_blas_buf, &empty_blas_buf, &scratch, &pb_buf] {
        gpu.resident(ProtocolObject::from_ref(&**b));
    }

    let align256 = |va: u64| va.next_multiple_of(256);
    let blas_va = align256(blas_buf.gpuAddress());
    let tlas_va = align256(tlas_buf.gpuAddress());
    let clone_va = align256(clone_buf.gpuAddress());
    let compact_blas_va = align256(compact_blas_buf.gpuAddress());
    let empty_blas_va = align256(empty_blas_buf.gpuAddress());
    let scratch_va = align256(scratch.gpuAddress());

    let inst_descs = [
        InstanceDesc::new([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]], 10, 0x01, 3, 0, blas_va),
        InstanceDesc::new([[1.0, 0.0, 0.0, 2.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 1.0]], 20, 0x02, 7, 0, blas_va),
    ];
    let inst_buf = shared(bytes_of(&inst_descs));
    gpu.resident(ProtocolObject::from_ref(&*inst_buf));

    let tlas_inputs = Inputs::Top { flags: build_flags::ALLOW_UPDATE, count: 2, instances: inst_buf.gpuAddress(), pointers: false };

    let util_table = new_argument_table("rt_util").expect("util table");
    let mut ring = UploadRing::default();
    let mut keep = Vec::new();

    // 1. Build BLAS (with COMPACTED_SIZE postbuild) and TLAS (with CURRENT_SIZE postbuild).
    let pb_blas = [PostbuildInfoDesc { DestBuffer: pb_buf.gpuAddress(), InfoType: postbuild::COMPACTED_SIZE, _pad: 0 }];
    let pb_tlas = PostbuildInfoDesc { DestBuffer: pb_buf.gpuAddress() + 8, InfoType: postbuild::CURRENT_SIZE, _pad: 0 };
    gpu.run(|enc| {
        let mut ctx = Ctx { enc, table: &util_table, ring: &mut ring, keep: &mut keep };
        encode_build(&mut ctx, &BuildArgs { dest: blas_va, source: 0, scratch: scratch_va, inputs: blas_inputs.clone(), owner: 1 }, &pb_blas).expect("encode BLAS");
        barrier(enc, MTLStages::AccelerationStructure | MTLStages::Dispatch, MTLStages::AccelerationStructure | MTLStages::Dispatch);
        encode_build(&mut ctx, &BuildArgs { dest: tlas_va, source: 0, scratch: scratch_va, inputs: tlas_inputs.clone(), owner: 2 }, &[]).expect("encode TLAS");
        encode_postbuild(&mut ctx, &pb_tlas, &[tlas_va]).expect("encode postbuild");
    });

    let pb_words = unsafe { std::slice::from_raw_parts(pb_buf.contents().as_ptr() as *const u64, 2) };
    assert_eq!(pb_words[0], result_size(0), "BLAS compacted size");
    assert_eq!(pb_words[1], result_size(2), "TLAS current size");
    assert!(stats().live >= 2 && stats().private_bytes > 0);

    // Helper that traces using our D3D result buffer VA (via both descriptor table and root SRV).
    let trace_va = |va: u64, use_root: bool, flags: u32, mask: u32| -> Vec<[u32; 4]> {
        d3d12_metal::metal::device().unwrap().commit_residency();
        let heap = shared(bytes_of(&[va, 0u64, 0u64]));
        let rays = shared(bytes_of(&RAYS));
        let out = shared(&[0xAAu8; 16 * 16]);
        let layout = p.rs.layout();
        let mut tlab = layout.new_argument_buffer();
        tlab.set_descriptor_table(layout, 0, heap.gpuAddress());
        tlab.set_root_descriptor(layout, 1, va);
        tlab.set_root_descriptor(layout, 2, out.gpuAddress());
        tlab.set_constants(layout, 3, 0, &[use_root as u32, flags, mask, 0]);
        tlab.set_root_descriptor(layout, 4, rays.gpuAddress());
        let ab = shared(&tlab.bytes);
        for b in [&heap, &rays, &out, &ab] {
            gpu.resident(ProtocolObject::from_ref(&**b));
        }
        let table = new_argument_table("gpu_rayquery_va").expect("argument table");
        gpu.run(|enc| {
            enc.barrierAfterQueueStages_beforeStages_visibilityOptions(MTLStages::All, MTLStages::All, MTL4VisibilityOptions::Device);
            unsafe { table.setAddress_atIndex(ab.gpuAddress(), dxil_metal::runtime::ARGUMENT_BUFFER_BIND_POINT as usize) };
            enc.setArgumentTable(Some(&table));
            enc.setComputePipelineState(&p.pso);
            let tg = p.threadgroup;
            enc.dispatchThreadgroups_threadsPerThreadgroup(MTLSize { width: 1, height: 1, depth: 1 }, MTLSize { width: tg[0] as usize, height: tg[1] as usize, depth: tg[2] as usize });
        });
        unsafe { std::slice::from_raw_parts(out.contents().as_ptr() as *const [u32; 4], 16) }.to_vec()
    };

    // Make the device residency set (which holds the private MTLAccelerationStructures) active on our test queue.
    gpu.queue.addResidencySet(dev_res);

    check(&trace_va(tlas_va, false, 0, 0xFF), 0xFF, false, "accel table SRV");
    check(&trace_va(tlas_va, true, 0, 0xFF), 0xFF, false, "accel root SRV");

    // 2. CLONE the TLAS and COMPACT the BLAS; verify the cloned TLAS has its own header & contributions.
    gpu.run(|enc| {
        let mut ctx = Ctx { enc, table: &util_table, ring: &mut ring, keep: &mut keep };
        encode_copy(&mut ctx, clone_va, tlas_va, copy_mode::CLONE, 3).expect("clone TLAS");
        encode_copy(&mut ctx, compact_blas_va, blas_va, copy_mode::COMPACT, 4).expect("compact BLAS");
    });
    assert_eq!(entry_info(clone_va).map(|(top, n, _)| (top, n)), Some((true, 2)));
    assert_eq!(entry_info(compact_blas_va).map(|(top, n, _)| (top, n)), Some((false, 0)));
    check(&trace_va(clone_va, false, 0, 0xFF), 0xFF, false, "cloned TLAS");

    // 3. Inactive instance (null BLAS VA = 0) in slot 0: instance 1 must still report InstanceIndex == 1
    // and contribution == 7, while rays over [0,1] miss. Also test compacted BLAS in instance 1.
    let inst_inactive = [
        InstanceDesc::new([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]], 10, 0xFF, 3, 0, 0),
        InstanceDesc::new([[1.0, 0.0, 0.0, 2.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 1.0]], 20, 0x02, 7, 0, compact_blas_va),
    ];
    let inst_inactive_buf = shared(bytes_of(&inst_inactive));
    gpu.resident(ProtocolObject::from_ref(&*inst_inactive_buf));
    let tlas_inactive_inputs = Inputs::Top { flags: build_flags::ALLOW_UPDATE, count: 2, instances: inst_inactive_buf.gpuAddress(), pointers: false };
    gpu.run(|enc| {
        let mut ctx = Ctx { enc, table: &util_table, ring: &mut ring, keep: &mut keep };
        encode_build(&mut ctx, &BuildArgs { dest: tlas_va, source: 0, scratch: scratch_va, inputs: tlas_inactive_inputs.clone(), owner: 2 }, &[]).expect("rebuild TLAS with inactive slot 0");
    });
    // Mask 0x02 matches what reference() produces when only instance 1 is active!
    check(&trace_va(tlas_va, false, 0, 0xFF), 0x02, false, "inactive instance slot 0");

    // 4. PERFORM_UPDATE (refit) after translating instance 1 to z = 2.0 (so t becomes 3.0 instead of 2.0),
    // and restoring instance 0.
    let inst_refit = [
        InstanceDesc::new([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]], 10, 0x01, 3, 0, compact_blas_va),
        InstanceDesc::new([[1.0, 0.0, 0.0, 2.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 2.0]], 20, 0x02, 7, 0, compact_blas_va),
    ];
    let inst_refit_buf = shared(bytes_of(&inst_refit));
    gpu.resident(ProtocolObject::from_ref(&*inst_refit_buf));
    let tlas_refit_inputs = Inputs::Top { flags: build_flags::ALLOW_UPDATE | build_flags::PERFORM_UPDATE, count: 2, instances: inst_refit_buf.gpuAddress(), pointers: false };
    gpu.run(|enc| {
        let mut ctx = Ctx { enc, table: &util_table, ring: &mut ring, keep: &mut keep };
        encode_build(&mut ctx, &BuildArgs { dest: tlas_va, source: tlas_va, scratch: scratch_va, inputs: tlas_refit_inputs, owner: 2 }, &[]).expect("refit TLAS");
    });
    let refit_out = trace_va(tlas_va, true, 0, 0xFF);
    let (_, hit3, _) = decode(&refit_out, 3);
    assert!((hit3.expect("ray 3 hit after refit").t - 3.0).abs() < 1e-4, "refit updated t: {hit3:?}");

    // 5. Empty BLAS (0 triangles) builds cleanly and misses without crashing.
    let empty_inputs = Inputs::Bottom { flags: 0, geometries: Vec::new() };
    let inst_empty = [InstanceDesc::new([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]], 10, 0xFF, 3, 0, empty_blas_va)];
    let inst_empty_buf = shared(bytes_of(&inst_empty));
    gpu.resident(ProtocolObject::from_ref(&*inst_empty_buf));
    gpu.run(|enc| {
        let mut ctx = Ctx { enc, table: &util_table, ring: &mut ring, keep: &mut keep };
        encode_build(&mut ctx, &BuildArgs { dest: empty_blas_va, source: 0, scratch: scratch_va, inputs: empty_inputs, owner: 5 }, &[]).expect("empty BLAS");
        barrier(enc, MTLStages::Dispatch, MTLStages::Dispatch);
        let top_empty = Inputs::Top { flags: 0, count: 1, instances: inst_empty_buf.gpuAddress(), pointers: false };
        encode_build(&mut ctx, &BuildArgs { dest: tlas_va, source: 0, scratch: scratch_va, inputs: top_empty, owner: 2 }, &[]).expect("TLAS with empty BLAS");
    });
    check(&trace_va(tlas_va, false, 0, 0xFF), 0, true, "empty BLAS misses");

    // Clean up registry entries created by this test.
    for va in [blas_va, tlas_va, clone_va, compact_blas_va, empty_blas_va] {
        drop(evict_range(va, 256));
    }
    drop(keep);
}

#[test]
fn rayquery_via_command_list() {
    if !ready() {
        return;
    }
    use d3d12_metal::metal::cmdlist::{CommandAllocator, CommandList};
    use d3d12_metal::metal::descriptor::{heap_kind, DescriptorHeap};
    use d3d12_metal::metal::queue::{ListType, Queue};
    use d3d12_metal::metal::resource::Resource;
    use d3d12_metal::metal::views::srv_entry;
    use d3d12_metal::rt::accel::{entry_info, prebuild_info, result_size, BuildArgs};
    use d3d12_metal::rt::d3d::{build_flags, copy_mode, postbuild, BuildInputs, Geometry, Inputs, InstanceDesc, PostbuildInfoDesc, TrianglesDesc, TYPE_BOTTOM_LEVEL};
    use d3d12_metal::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};

    let rs = RootSignature::from_blob(&signed(CS)).expect("embedded root signature");
    let p = Arc::new(ComputePipeline::new(Arc::clone(&rs), &signed(CS), "gpu_rayquery_cmdlist").expect("compute pipeline"));

    let q = Queue::new(ListType::Direct, "rt test queue").expect("queue");
    let alloc = CommandAllocator::new(ListType::Direct).expect("allocator");
    let mut list = CommandList::new(ListType::Direct).expect("command list");

    let mk_buf = |bytes: &[u8]| -> Resource {
        let len = bytes.len().max(256) as u64;
        let d = ResourceDesc { Dimension: dim::BUFFER, Width: len, Height: 1, DepthOrArraySize: 1, MipLevels: 1, SampleCount: 1, Layout: 1, Flags: resource_flags::ALLOW_UNORDERED_ACCESS, ..Default::default() };
        let hp = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
        let r = Resource::new_committed(&d, &hp, MTLTextureUsage::empty(), "rt buf").expect("buffer");
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), r.cpu_ptr().unwrap(), bytes.len()) };
        r
    };

    let verts: [[f32; 3]; 4] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];
    let indices: [u16; 6] = [0, 1, 2, 1, 3, 2];
    let vb = mk_buf(bytes_of(&verts));
    let ib = mk_buf(bytes_of(&indices));

    let g = Geometry::Triangles {
        flags: 0,
        desc: TrianglesDesc {
            Transform3x4: 0,
            IndexFormat: 57,
            VertexFormat: 6,
            IndexCount: 6,
            VertexCount: 4,
            IndexBuffer: ib.gpu_va,
            VertexBufferStart: vb.gpu_va,
            VertexBufferStride: 12,
        },
    };
    let gd = [g.to_desc()];
    let raw_blas = BuildInputs { Type: TYPE_BOTTOM_LEVEL, Flags: build_flags::ALLOW_COMPACTION, NumDescs: 1, DescsLayout: 0, Descs: gd.as_ptr() as u64 };
    let blas_inputs = unsafe { Inputs::from_raw(&raw_blas) }.expect("Inputs::from_raw");
    let blas_pre = prebuild_info(&blas_inputs).unwrap();

    let blas_res = mk_buf(&vec![0u8; blas_pre.ResultDataMaxSizeInBytes as usize]);
    let inst_descs = [
        InstanceDesc::new([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]], 10, 0x01, 3, 0, blas_res.gpu_va),
        InstanceDesc::new([[1.0, 0.0, 0.0, 2.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 1.0]], 20, 0x02, 7, 0, blas_res.gpu_va),
    ];
    let inst_res = mk_buf(bytes_of(&inst_descs));
    let tlas_inputs = Inputs::Top { flags: build_flags::ALLOW_UPDATE, count: 2, instances: inst_res.gpu_va, pointers: false };
    let tlas_pre = prebuild_info(&tlas_inputs).unwrap();
    let tlas_res = mk_buf(&vec![0u8; tlas_pre.ResultDataMaxSizeInBytes as usize]);
    let clone_res = mk_buf(&vec![0u8; tlas_pre.ResultDataMaxSizeInBytes as usize]);
    let scratch_res = mk_buf(&vec![0u8; blas_pre.ScratchDataSizeInBytes.max(tlas_pre.ScratchDataSizeInBytes) as usize + 4096]);
    let pb_res = mk_buf(&[0u8; 64]);
    let rays_res = mk_buf(bytes_of(&RAYS));
    let out_res = mk_buf(&[0xAAu8; 16 * 16]);

    // Descriptor heap with D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE (dimension = 11).
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "rt srv heap").unwrap();
    let mut srv_desc = [0u8; 40];
    srv_desc[4..8].copy_from_slice(&11u32.to_le_bytes());
    srv_desc[16..24].copy_from_slice(&clone_res.gpu_va.to_le_bytes());
    let entry = srv_entry(None, Some(&srv_desc)).expect("AS SRV entry");
    unsafe { std::ptr::copy_nonoverlapping(entry.as_ptr(), heap.cpu_base as *mut u8, entry.len()) };

    list.begin(&alloc).unwrap();
    list.build_accel(&BuildArgs { dest: blas_res.gpu_va, source: 0, scratch: scratch_res.gpu_va, inputs: blas_inputs, owner: 1 }, &[]).unwrap();
    list.barrier();
    list.build_accel(&BuildArgs { dest: tlas_res.gpu_va, source: 0, scratch: scratch_res.gpu_va, inputs: tlas_inputs, owner: 2 }, &[]).unwrap();
    list.emit_accel_postbuild(&PostbuildInfoDesc { DestBuffer: pb_res.gpu_va, InfoType: postbuild::CURRENT_SIZE, _pad: 0 }, &[tlas_res.gpu_va]).unwrap();
    list.barrier();
    list.copy_accel(clone_res.gpu_va, tlas_res.gpu_va, copy_mode::CLONE, 3).unwrap();
    list.barrier();

    list.set_descriptor_heaps(heap.gpu_base, 0);
    list.set_compute_pipeline(p);
    list.compute_root.set_root_signature(rs).unwrap();
    list.compute_root.set_table(0, heap.gpu_base).unwrap();
    list.compute_root.set_descriptor(1, clone_res.gpu_va).unwrap();
    list.compute_root.set_descriptor(2, out_res.gpu_va).unwrap();
    list.compute_root.set_constants(3, 0, &[0, 0, 0xFF, 0]).unwrap();
    list.compute_root.set_descriptor(4, rays_res.gpu_va).unwrap();
    list.dispatch(1, 1, 1).unwrap();
    list.close().unwrap();

    q.submit(&[&list.cb]);
    q.wait_idle();
    alloc.reset();

    let pb_val = unsafe { *(pb_res.cpu_ptr().unwrap() as *const u64) };
    assert_eq!(pb_val, result_size(2));
    let out = unsafe { std::slice::from_raw_parts(out_res.cpu_ptr().unwrap() as *const [u32; 4], 16) };
    check(out, 0xFF, false, "CommandList AS build + clone + SRV dispatch");

    // Dropping the buffer resources evicts their registry entries automatically.
    let clone_va = clone_res.gpu_va;
    assert!(entry_info(clone_va).is_some());
    drop(clone_res);
    assert!(entry_info(clone_va).is_none(), "Resource::drop evicted AS entry");
}


