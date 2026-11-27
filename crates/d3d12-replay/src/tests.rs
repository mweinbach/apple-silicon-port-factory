//! End-to-end tests over an in-memory trace.

use d3d12_metal::trace::{encode_input_layout, Call, GraphicsPipeline, InputElement as TraceInput, Record, U32s, U64s, Writer};
use dxil_metal::root_signature::flags::DESCRIPTOR_RANGE_OFFSET_APPEND;
use dxil_metal::*;

use crate::replay::{self, Backend, Handled, Kind, NullBackend, Objects};
use crate::{shaders, stats};

const FIXTURE: &[u8] = include_bytes!("../../dxil-metal/tests/fixtures/static_samplers.cs.dxil");

/// The fixture is written unsigned; sign it like the game's shaders are.
fn fixture_cs() -> Vec<u8> {
    let mut b = FIXTURE.to_vec();
    let size = u32::from_le_bytes(b[24..28].try_into().unwrap()) as usize;
    let sum = hash::dxbc_checksum(&b[20..size]);
    b[4..20].copy_from_slice(&sum);
    b
}

/// t0 table, root UAV u0, static samplers s0/s1: what the fixture uses.
fn root_signature_blob() -> Vec<u8> {
    let range = DescriptorRange {
        range_type: DescriptorRangeType::Srv,
        num_descriptors: 1,
        base_shader_register: 0,
        register_space: 0,
        flags: 0,
        offset_in_descriptors_from_table_start: DESCRIPTOR_RANGE_OFFSET_APPEND,
    };
    let s = |reg| StaticSampler { address_u: 3, address_v: 3, address_w: 3, shader_register: reg, ..Default::default() };
    RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter { kind: RootParameterKind::DescriptorTable(vec![range]), visibility: 0 },
            RootParameter {
                kind: RootParameterKind::Descriptor(RootDescriptorKind::Uav, RootDescriptor { shader_register: 0, register_space: 0, flags: 0 }),
                visibility: 0,
            },
        ],
        static_samplers: vec![s(0), s(1)],
        flags: 0,
    }
    .to_blob()
}

fn put(b: &mut [u8], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

/// D3D12_BLEND_DESC with RT0 = opaque (IndependentBlendEnable = FALSE), optionally with a
/// logic op Metal can't do.
fn blend_blob(logic_and: bool) -> Vec<u8> {
    let mut b = vec![0u8; 328];
    let rt = 8;
    for (off, v) in [(8, 2), (12, 1), (16, 1), (20, 2), (24, 1), (28, 1), (32, 4)] {
        put(&mut b, rt + off, v); // ONE, ZERO, ADD, ONE, ZERO, ADD, NOOP
    }
    b[rt + 36] = 0xF;
    if logic_and {
        put(&mut b, rt + 4, 1); // LogicOpEnable
        put(&mut b, rt + 32, 6); // D3D12_LOGIC_OP_AND
    }
    b
}

/// D3D12_RASTERIZER_DESC: solid, cull back, depth clip.
fn raster_blob(conservative: bool) -> Vec<u8> {
    let mut b = vec![0u8; 44];
    put(&mut b, 0, 3);
    put(&mut b, 4, 3);
    put(&mut b, 24, 1);
    put(&mut b, 40, conservative as u32);
    b
}

/// D3D12_DEPTH_STENCIL_DESC: depth LESS with writes, stencil off.
fn depth_blob() -> Vec<u8> {
    let mut b = vec![0u8; 52];
    put(&mut b, 0, 1);
    put(&mut b, 4, 1);
    put(&mut b, 8, 2);
    for face in [20, 36] {
        for (k, v) in [1, 1, 1, 8].into_iter().enumerate() {
            put(&mut b, face + 4 * k, v);
        }
    }
    b
}

fn input_layout(slot: u32) -> Vec<u8> {
    let e = |name: &str, format, offset| TraceInput {
        semantic_name: name.into(),
        semantic_index: 0,
        format,
        input_slot: slot,
        aligned_byte_offset: offset,
        input_slot_class: 0,
        instance_data_step_rate: 0,
    };
    // R32G32B32_FLOAT at 0, R32G32_FLOAT appended (-> 12).
    encode_input_layout(&[e("POSITION", 6, 0), e("TEXCOORD", 16, 0xFFFF_FFFF)])
}

struct Blobs {
    cs: Vec<u8>,
    rs: Vec<u8>,
    blend: Vec<u8>,
    blend_logic: Vec<u8>,
    raster: Vec<u8>,
    raster_cons: Vec<u8>,
    depth: Vec<u8>,
    il: Vec<u8>,
    il_bad: Vec<u8>,
}

impl Blobs {
    fn new() -> Self {
        Self {
            cs: fixture_cs(),
            rs: root_signature_blob(),
            blend: blend_blob(false),
            blend_logic: blend_blob(true),
            raster: raster_blob(false),
            raster_cons: raster_blob(true),
            depth: depth_blob(),
            il: input_layout(0),
            il_bad: input_layout(30),
        }
    }
}

const DEV: u64 = 0x1000;
const RS: u64 = 0x2000;
const CPSO: u64 = 0x3000;
const GOOD: u64 = 0x3100;
const BAD: u64 = 0x3200;
const ALLOC: u64 = 0x4000;
const LIST: u64 = 0x5000;
const QUEUE: u64 = 0x6000;
const SWAP: u64 = 0x7000;
const BACKBUFFER: u64 = 0x7100;

fn gfx<'a>(b: &'a Blobs, good: bool, rtv: &'a [u32]) -> GraphicsPipeline<'a> {
    GraphicsPipeline {
        root_signature: RS,
        vs: &[],
        ps: &[],
        ds: &[],
        hs: &[],
        gs: &[],
        stream_output: &[],
        blend: if good { &b.blend } else { &b.blend_logic },
        sample_mask: !0,
        rasterizer: if good { &b.raster } else { &b.raster_cons },
        raster_version: 0,
        depth_stencil: &b.depth,
        depth_stencil_version: 0,
        input_layout: if good { &b.il } else { &b.il_bad },
        ib_strip_cut_value: 0,
        primitive_topology_type: 3,
        rtv_formats: U32s::from_slice(rtv),
        dsv_format: 40,
        sample_count: 1,
        sample_quality: 0,
        node_mask: 0,
        flags: 0,
    }
}

const GOOD_RTV: [u32; 2] = [28, 57]; // R8G8B8A8_UNORM, R16_UINT
const BAD_RTV: [u32; 1] = [0xFFFF];
const LISTS: [u64; 1] = [LIST];

fn calls(b: &Blobs) -> Vec<Call<'_>> {
    vec![
        /* 0 */ Call::CreateDevice { device: DEV, feature_level: 0xc200 },
        /* 1 */ Call::CreateRootSignature { device: DEV, root_signature: RS, blob: &b.rs },
        /* 2 */ Call::CreateComputePipelineState { device: DEV, pso: CPSO, root_signature: RS, cs: &b.cs, flags: 0 },
        /* 3 */ Call::CreateGraphicsPipelineState { device: DEV, pso: GOOD, desc: gfx(b, true, &GOOD_RTV) },
        /* 4 */ Call::CreateGraphicsPipelineState { device: DEV, pso: BAD, desc: gfx(b, false, &BAD_RTV) },
        /* 5 */ Call::CreateCommandAllocator { device: DEV, allocator: ALLOC, list_type: 0 },
        /* 6 */ Call::CreateCommandList { device: DEV, list: LIST, list_type: 0, allocator: ALLOC, initial_pso: 0 },
        /* 7 */ Call::SetComputeRootSignature { list: LIST, root_signature: RS },
        /* 8 */ Call::SetPipelineState { list: LIST, pso: CPSO },
        /* 9 */ Call::Dispatch { list: LIST, x: 1, y: 1, z: 1 },
        /* 10 */ Call::Close { list: LIST },
        /* 11 */ Call::Dispatch { list: LIST, x: 1, y: 1, z: 1 }, // closed list
        /* 12 */ Call::CreateCommandQueue { device: DEV, queue: QUEUE, desc: &[0; 16] },
        /* 13 */ Call::ExecuteCommandLists { queue: QUEUE, lists: U64s::from_slice(&LISTS) },
        /* 14 */ Call::Release { object: CPSO },
        /* 15 */ Call::Reset { list: LIST, allocator: ALLOC, pso: CPSO }, // released PSO
        /* 16 */ Call::DrawInstanced { list: LIST, vertex_count: 3, instance_count: 1, start_vertex: 0, start_instance: 0 }, // PSO gone, no RS
        /* 17 */ Call::Release { object: 0xdead }, // unknown
        /* 18 */ Call::CreateSwapChain { queue: QUEUE, swapchain: SWAP, desc: &[0; 48] },
        /* 19 */ Call::GetSwapChainBuffer { swapchain: SWAP, index: 0, resource: BACKBUFFER },
        /* 20 */ Call::GetSwapChainBuffer { swapchain: SWAP, index: 0, resource: BACKBUFFER }, // same pointer again: fine
        /* 21 */ Call::Present { swapchain: SWAP, sync_interval: 1, flags: 0 },
        /* 22 */ Call::FrameEnd { frame: 0 },
    ]
}

fn trace_bytes(b: &Blobs) -> Vec<u8> {
    let mut w = Writer::new(Vec::new()).unwrap();
    for c in calls(b) {
        w.write(&c, 0).unwrap();
    }
    w.into_inner()
}

#[test]
fn stats_pass() {
    let b = Blobs::new();
    let t = trace_bytes(&b);
    let s = stats::stats(&t, None).unwrap();
    assert_eq!(s.records, 23);
    assert_eq!((s.frames, s.dispatches, s.draws), (1, 2, 1));
    assert_eq!((s.created, s.released), (9, 2));
    assert!(!s.truncated && s.decode_errors.is_empty());
    assert_eq!(s.per_op["CreateGraphicsPipelineState"], 2);
    let text = s.to_string();
    assert!(text.contains("frames 1"), "{text}");

    let until = stats::stats(&t, Some(4)).unwrap();
    assert_eq!(until.records, 5);

    let mut out = Vec::new();
    stats::dump(&t[..t.len() - 2], None, &mut out).unwrap();
    let out = String::from_utf8(out).unwrap();
    assert_eq!(out.lines().count(), 23, "{out}"); // 22 records + truncation note
    assert!(out.contains("CreateComputePipelineState") && out.contains("items>"), "{out}");
    assert!(out.ends_with("(trace truncated: torn final record)\n"));
}

#[test]
fn replay_validation() {
    let b = Blobs::new();
    let t = trace_bytes(&b);
    let rep = replay::replay(&t, &mut NullBackend, None, 100).unwrap();
    let msgs: Vec<String> = rep.warnings.items.iter().map(|(i, m)| format!("#{i}: {m}")).collect();
    let has = |i: u64, s: &str| rep.warnings.items.iter().any(|(j, m)| *j == i && m.contains(s));
    assert!(has(11, "is closed"), "{msgs:#?}");
    assert!(has(15, "was released"), "{msgs:#?}");
    assert!(has(16, "is gone"), "{msgs:#?}");
    assert!(has(16, "no graphics root signature"), "{msgs:#?}");
    assert!(has(17, "unknown id"), "{msgs:#?}");
    assert_eq!(rep.warnings.total, 5, "{msgs:#?}");
    assert_eq!((rep.records, rep.frames, rep.done, rep.released), (23, 1, 23, 1));
    // The live table at the end: everything but the released compute PSO.
    assert_eq!(rep.live_at_end.get(&Kind::ComputePso), None);
    assert_eq!(rep.live_at_end[&Kind::GraphicsPso], 2);
    assert_eq!(rep.live_at_end[&Kind::Resource], 1);

    // Backends see creations before the call and releases after it.
    struct Probe(Vec<String>);
    impl Backend for Probe {
        fn name(&self) -> &'static str {
            "probe"
        }
        fn call(&mut self, rec: &Record, objs: &Objects) -> Result<Handled, String> {
            match rec.call {
                Call::CreateComputePipelineState { pso, .. } | Call::Release { object: pso } if pso == CPSO => {
                    self.0.push(format!("{}:{:?}", rec.call.name(), objs.get(pso).map(|o| o.kind)));
                }
                Call::Dispatch { .. } => return Err("not yet".into()),
                _ => {}
            }
            Ok(if matches!(rec.call, Call::Present { .. }) { Handled::Skipped } else { Handled::Done })
        }
    }
    let mut probe = Probe(vec![]);
    let rep = replay::replay(&t, &mut probe, Some(21), 100).unwrap();
    assert_eq!(probe.0, ["CreateComputePipelineState:Some(ComputePso)", "Release:Some(ComputePso)"]);
    assert_eq!(rep.failed.total, 2);
    assert_eq!(rep.skipped["Present"], 1);
    assert_eq!(rep.records, 22);
}

#[test]
fn graphics_state_translation() {
    let b = Blobs::new();
    let (st, unsupported) = shaders::translate_graphics_state(&gfx(&b, true, &GOOD_RTV)).unwrap();
    assert!(unsupported.is_empty());
    assert_eq!(st.int_rt_mask, 0b10);
    assert_eq!(st.input_layout[1].aligned_byte_offset, 12);
    assert_eq!(st.color_formats.len(), 2);
    assert!(!st.blend.dual_source && !st.blend.attachments[0].enabled);
    assert!(st.raster.depth_clip == objc2_metal::MTLDepthClipMode::Clip);

    let errs = shaders::translate_graphics_state(&gfx(&b, false, &BAD_RTV)).unwrap_err();
    let joined = errs.join("\n");
    assert!(joined.contains("RT0 format"), "{joined}");
    assert!(joined.contains("vertex buffer slot 30"), "{joined}");

    let mut short = gfx(&b, true, &GOOD_RTV);
    short.blend = &b.blend[..100];
    assert!(shaders::translate_graphics_state(&short).unwrap_err()[0].contains("BlendDesc"));
}

#[test]
fn bytecode_detection() {
    assert_eq!(shaders::bytecode_kind(&fixture_cs()), shaders::Bytecode::Dxil);
    let sm5 = dxbc::build_container(&[(*b"ISGN", &[0; 8]), (*b"SHEX", &[0; 16])]);
    assert_eq!(shaders::bytecode_kind(&sm5), shaders::Bytecode::Dxbc);
    assert_eq!(shaders::bytecode_kind(b"garbage"), shaders::Bytecode::Unknown);
}

#[test]
fn shaders_pass() {
    if !converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return;
    }
    let b = Blobs::new();
    let t = trace_bytes(&b);
    let p = ShaderPipeline::new(CacheConfig::memory_only()).unwrap();
    let rep = shaders::validate(&t, &p, &shaders::Options { jobs: 2, ..Default::default() }).unwrap();
    let text = rep.to_string();
    assert_eq!((rep.root_signatures, rep.root_signature_failures.total), (1, 0), "{text}");
    assert_eq!((rep.graphics, rep.compute), (2, 1), "{text}");
    assert_eq!(rep.stages_compiled, 1, "{text}");
    // Both graphics PSOs fail (no vertex shader / bad state); the compute PSO converts.
    assert_eq!(rep.pipelines_failed, 2, "{text}");
    assert!(rep.failures.items.iter().all(|(i, _)| *i == 3 || *i == 4), "{text}");
    assert!(rep.failures.items.iter().any(|(i, m)| *i == 3 && m.contains("no vertex shader")), "{text}");
    assert_eq!(rep.unsupported.get("blend logic ops"), Some(&1), "{text}");
    assert_eq!(rep.unsupported.get("conservative rasterization"), Some(&1), "{text}");
}

// ---- mesh-shader PSOs -------------------------------------------------------------------------

fn signed(b: &[u8]) -> Vec<u8> {
    let mut v = b.to_vec();
    dxbc::sign_container(&mut v).unwrap();
    v
}

/// Root signature + mesh PSO (AS/MS/PS fixture from d3d12-metal) + a DispatchMesh, plus one
/// DispatchMesh with a graphics PSO bound (a validation warning).
fn mesh_trace(b: &Blobs, amp: &[u8], ms: &[u8], ps: &[u8], rs: &[u8]) -> Vec<u8> {
    let rtv = [28u32];
    let desc = GraphicsPipeline {
        root_signature: 0x20,
        vs: &[],
        ps,
        ds: &[],
        hs: &[],
        gs: &[],
        stream_output: &[],
        blend: &b.blend,
        sample_mask: !0,
        rasterizer: &b.raster,
        raster_version: 0,
        depth_stencil: &b.depth,
        depth_stencil_version: 0,
        input_layout: &[],
        ib_strip_cut_value: 0,
        primitive_topology_type: 0,
        rtv_formats: U32s::from_slice(&rtv),
        dsv_format: 0,
        sample_count: 1,
        sample_quality: 0,
        node_mask: 0,
        flags: 0,
    };
    let calls = [
        Call::CreateDevice { device: 1, feature_level: 0xc200 },
        Call::CreateCommandAllocator { device: 1, allocator: 2, list_type: 0 },
        Call::CreateRootSignature { device: 1, root_signature: 0x20, blob: rs },
        Call::CreateMeshPipelineState { device: 1, pso: 0x30, amplification: amp, mesh: ms, desc },
        // Root signature 0: the fixture's resources don't match the mesh root signature.
        Call::CreateComputePipelineState { device: 1, pso: 0x31, root_signature: 0, cs: &b.cs, flags: 0 },
        Call::CreateCommandList { device: 1, list: 0x40, list_type: 0, allocator: 2, initial_pso: 0x30 },
        Call::SetGraphicsRootSignature { list: 0x40, root_signature: 0x20 },
        Call::DispatchMesh { list: 0x40, x: 1, y: 1, z: 1 },
        Call::SetPipelineState { list: 0x40, pso: 0x31 },
        Call::DispatchMesh { list: 0x40, x: 1, y: 1, z: 1 },
        Call::Close { list: 0x40 },
    ];
    let mut w = Writer::new(Vec::new()).unwrap();
    for c in &calls {
        w.write(c, 0).unwrap();
    }
    w.into_inner()
}

const MESH_AS: &[u8] = include_bytes!("../../d3d12-metal/tests/fixtures/mesh.as.dxil");
const MESH_MS: &[u8] = include_bytes!("../../d3d12-metal/tests/fixtures/mesh.ms.dxil");
const MESH_PS: &[u8] = include_bytes!("../../d3d12-metal/tests/fixtures/mesh.ps.dxil");

#[test]
fn mesh_pso_validation() {
    let b = Blobs::new();
    let (amp, ms, ps) = (signed(MESH_AS), signed(MESH_MS), signed(MESH_PS));
    let rs = RootSignatureDesc::from_blob(&amp).unwrap().to_blob();
    let t = mesh_trace(&b, &amp, &ms, &ps, &rs);

    let s = stats::stats(&t, None).unwrap();
    assert_eq!(s.draws, 2);
    let rep = replay::replay(&t, &mut NullBackend, None, 50).unwrap();
    let w: Vec<&String> = rep.warnings.items.iter().map(|(_, m)| m).collect();
    assert_eq!(w.len(), 1, "{w:?}");
    assert!(w[0].contains("DispatchMesh") && w[0].contains("ComputePso"), "{w:?}");

    if !converter_available() {
        eprintln!("skipping shader part: metal shader converter not available");
        return;
    }
    let p = ShaderPipeline::new(CacheConfig::memory_only()).unwrap();
    let rep = shaders::validate(&t, &p, &shaders::Options { jobs: 2, ..Default::default() }).unwrap();
    let text = rep.to_string();
    assert_eq!((rep.mesh, rep.compute, rep.pipelines_failed), (1, 1, 0), "{text}");
    // AS + MS + PS, and the compute shader.
    assert_eq!(rep.stages_compiled, 4, "{text}");
}

#[test]
fn accel_trace_validation() {
    let srcs = [0x4_0001_0000u64];
    let calls = [
        Call::CreateDevice { device: 1, feature_level: 0xc200 },
        Call::CreateCommandAllocator { device: 1, allocator: 2, list_type: 0 },
        Call::CreateCommandList { device: 1, list: 3, list_type: 0, allocator: 2, initial_pso: 0 },
        Call::BuildRaytracingAccelerationStructure {
            list: 3,
            dest: 0x4_0001_0000,
            source: 0,
            scratch: 0x4_0002_0000,
            inputs_type: 1,
            flags: 4,
            num_descs: 1,
            descs_layout: 0,
            instances_va: 0,
            geometries: &[0; 56],
            postbuild: &[0; 16],
        },
        Call::EmitRaytracingAccelerationStructurePostbuildInfo {
            list: 3,
            dest_buffer: 0x4_0003_0000,
            info_type: 0,
            sources: U64s::from_slice(&srcs),
        },
        Call::CopyRaytracingAccelerationStructure {
            list: 3,
            dest: 0x4_0004_0000,
            source: 0x4_0001_0000,
            mode: 1,
        },
        Call::Close { list: 3 },
        // Copy on a closed list should warn.
        Call::CopyRaytracingAccelerationStructure {
            list: 3,
            dest: 0x4_0004_0000,
            source: 0x4_0001_0000,
            mode: 1,
        },
    ];
    let mut w = Writer::new(Vec::new()).unwrap();
    for c in &calls {
        w.write(c, 0).unwrap();
    }
    let t = w.into_inner();
    let rep = replay::replay(&t, &mut NullBackend, None, 50).unwrap();
    assert_eq!(rep.records, 8);
    assert_eq!(rep.warnings.total, 1, "{:?}", rep.warnings.items);
    assert!(rep.warnings.items[0].1.contains("is closed"));
}

#[test]
fn rt_pipeline_trace_validation() {
    let rs = root_signature_blob();
    let calls = [
        Call::CreateDevice { device: 1, feature_level: 0xc200 },
        Call::CreateCommandAllocator { device: 1, allocator: 2, list_type: 0 },
        Call::CreateRootSignature { device: 1, root_signature: 0x20, blob: &rs },
        Call::CreateStateObject {
            device: 1,
            state_object: 0x100,
            state_object_type: 0,
            num_subobjects: 2,
            num_exports: 3,
            num_hit_groups: 1,
        },
        Call::AddToStateObject {
            device: 1,
            state_object: 0x101,
            base: 0x100,
            state_object_type: 3,
            num_subobjects: 4,
            num_exports: 4,
            num_hit_groups: 2,
        },
        Call::CreateCommandList { device: 1, list: 3, list_type: 0, allocator: 2, initial_pso: 0 },
        Call::SetComputeRootSignature { list: 3, root_signature: 0x20 },
        Call::SetPipelineState1 { list: 3, state_object: 0x101 },
        Call::DispatchRays {
            list: 3,
            width: 64,
            height: 64,
            depth: 1,
            desc: &[0; 104],
        },
        Call::Release { object: 0x101 },
        // DispatchRays after releasing the bound StateObject should warn.
        Call::DispatchRays {
            list: 3,
            width: 64,
            height: 64,
            depth: 1,
            desc: &[0; 104],
        },
        Call::Close { list: 3 },
    ];
    let mut w = Writer::new(Vec::new()).unwrap();
    for c in &calls {
        w.write(c, 0).unwrap();
    }
    let t = w.into_inner();
    let s = stats::stats(&t, None).unwrap();
    assert_eq!(s.dispatches, 2);
    assert_eq!(s.created, 6);
    assert_eq!(s.released, 1);

    let rep = replay::replay(&t, &mut NullBackend, None, 50).unwrap();
    assert_eq!(rep.records, 12);
    assert_eq!(rep.created.get(&Kind::StateObject), Some(&2));
    assert_eq!(rep.live_at_end.get(&Kind::StateObject), Some(&1));
    assert_eq!(rep.warnings.total, 1, "{:?}", rep.warnings.items);
    assert!(rep.warnings.items[0].1.contains("DispatchRays") && rep.warnings.items[0].1.contains("is gone"));
}

#[test]
fn predication_clear_state_stencil_and_clear_rects_validation() {
    let b = Blobs::new();
    let rect_bytes = [1u8, 0, 0, 0, 2, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0]; // 16-byte Rect { left: 1, top: 2, right: 5, bottom: 6 }
    let calls = [
        /* 0 */ Call::CreateDevice { device: DEV, feature_level: 0xc200 },
        /* 1 */ Call::CreateRootSignature { device: DEV, root_signature: RS, blob: &b.rs },
        /* 2 */ Call::CreateGraphicsPipelineState { device: DEV, pso: GOOD, desc: gfx(&b, true, &GOOD_RTV) },
        /* 3 */ Call::CreateComputePipelineState { device: DEV, pso: CPSO, root_signature: RS, cs: &b.cs, flags: 0 },
        /* 4 */ Call::CreateCommittedResource {
            device: DEV,
            resource: 0x2200,
            heap_properties: &[1, 0, 0, 0],
            heap_flags: 0,
            desc: &[0; 56],
            initial_state: 0,
            clear_value: None,
            gpu_va: 0x4_0000_0000,
        },
        /* 5 */ Call::CreateCommandAllocator { device: DEV, allocator: ALLOC, list_type: 0 },
        /* 6 */ Call::CreateCommandList { device: DEV, list: LIST, list_type: 0, allocator: ALLOC, initial_pso: GOOD },
        /* 7 */ Call::SetGraphicsRootSignature { list: LIST, root_signature: RS },
        /* 8 */ Call::OMSetFrontAndBackStencilRef { list: LIST, front: 0x12, back: 0x34 },
        /* 9 */ Call::SetPredication { list: LIST, buffer: 0x2200, aligned_offset: 64, operation: 0 },
        /* 10 */ Call::ClearRenderTargetView { list: LIST, rtv: 0x9000, color: [0.0, 1.0, 0.0, 1.0], rects: &rect_bytes },
        /* 11 */ Call::ClearDepthStencilView { list: LIST, dsv: 0x9100, flags: 3, depth: 1.0, stencil: 0x7F, rects: &rect_bytes },
        /* 12 */ Call::DrawInstanced { list: LIST, vertex_count: 3, instance_count: 1, start_vertex: 0, start_instance: 0 },
        // ClearState binds CPSO and clears graphics_root_signature / compute_root_signature.
        /* 13 */ Call::ClearState { list: LIST, pso: CPSO },
        // Dispatch without re-setting compute root signature should warn.
        /* 14 */ Call::Dispatch { list: LIST, x: 1, y: 1, z: 1 },
        /* 15 */ Call::SetComputeRootSignature { list: LIST, root_signature: RS },
        /* 16 */ Call::Dispatch { list: LIST, x: 1, y: 1, z: 1 },
        // Disable predication (buffer = 0).
        /* 17 */ Call::SetPredication { list: LIST, buffer: 0, aligned_offset: 0, operation: 0 },
        // Malformed ClearRenderTargetView rects blob (7 bytes, not a multiple of 16) should warn.
        /* 18 */ Call::ClearRenderTargetView { list: LIST, rtv: 0x9000, color: [0.0; 4], rects: &[1, 2, 3, 4, 5, 6, 7] },
        /* 19 */ Call::Close { list: LIST },
    ];
    let mut w = Writer::new(Vec::new()).unwrap();
    for c in &calls {
        w.write(c, 0).unwrap();
    }
    let t = w.into_inner();
    let rep = replay::replay(&t, &mut NullBackend, None, 50).unwrap();
    assert_eq!(rep.records, 20);
    assert_eq!(rep.decode_errors.total, 0);
    assert_eq!(rep.warnings.total, 2, "{:?}", rep.warnings.items);
    assert!(rep.warnings.items.iter().any(|(i, m)| *i == 14 && m.contains("no compute root signature")), "{:?}", rep.warnings.items);
    assert!(rep.warnings.items.iter().any(|(i, m)| *i == 18 && m.contains("rects blob")), "{:?}", rep.warnings.items);
}

// ---- Metal 4 backend, golden comparison, synthetic traces & benchmark tests -------------------

#[test]
fn golden_png_and_hash_roundtrip() {
    use crate::golden::{self, FrameImage};

    let (w, h) = (16u32, 12u32);
    let mut rgb = vec![0u8; (w * h * 3) as usize];
    for y in 0..h as usize {
        for x in 0..w as usize {
            let i = (y * w as usize + x) * 3;
            rgb[i] = (x * 16) as u8;
            rgb[i + 1] = (y * 20) as u8;
            rgb[i + 2] = ((x ^ y) * 11) as u8;
        }
    }
    let img = FrameImage { width: w, height: h, rgb };
    let png = img.to_png();
    let decoded = golden::decode_png_rgb8(&png).expect("decode generated PNG");
    assert_eq!(decoded, img);

    let dir = std::env::temp_dir().join(format!("d3d12-replay-golden-unit-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let png_path = dir.join("ref.png");
    let hash_path = dir.join("ref.hash");

    golden::write_golden(&png_path, &img).unwrap();
    golden::write_golden(&hash_path, &img).unwrap();

    let c1 = golden::verify_golden(&png_path, &img, 0).unwrap();
    assert!(c1.passed, "{c1}");
    let c2 = golden::verify_golden(&hash_path, &img, 0).unwrap();
    assert!(c2.passed, "{c2}");

    let mut mutated = img.clone();
    mutated.rgb[0] = mutated.rgb[0].wrapping_add(10);
    let c3 = golden::verify_golden(&png_path, &mutated, 0).unwrap();
    assert!(!c3.passed, "mutated image must fail exact PNG golden");
    let c4 = golden::verify_golden(&png_path, &mutated, 10).unwrap();
    assert!(c4.passed, "mutated image within max_delta=10 should pass: {c4}");
    let c5 = golden::verify_golden(&hash_path, &mutated, 0).unwrap();
    assert!(!c5.passed, "mutated image must fail hash golden");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn synthetic_traces_validate_on_null_backend() {
    use crate::synthetic;

    for &name in synthetic::SYNTHETIC_NAMES {
        let trace = synthetic::generate_by_name(name).unwrap_or_else(|e| panic!("{name}: {e}"));
        let st = stats::stats(&trace, None).unwrap();
        assert!(st.decode_errors.is_empty(), "{name}: decode errors {:?}", st.decode_errors);
        assert!(!st.truncated, "{name}: truncated");
        assert!(st.frames >= 1, "{name}: expected >= 1 frame");
        let rep = replay::replay(&trace, &mut NullBackend, None, 100).unwrap();
        assert_eq!(rep.decode_errors.total, 0, "{name}: {rep}");
        assert_eq!(rep.warnings.total, 0, "{name}: warnings {:?}", rep.warnings.items);
        assert_eq!(rep.failed.total, 0, "{name}: {rep}");
    }
}

fn metal_ready() -> bool {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return false;
    }
    true
}

fn metal_and_converter_ready() -> bool {
    if !metal_ready() {
        return false;
    }
    if !converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return false;
    }
    true
}

fn pixel_at(img: &crate::golden::FrameImage, x: u32, y: u32) -> [u8; 3] {
    let i = ((y * img.width + x) * 3) as usize;
    [img.rgb[i], img.rgb[i + 1], img.rgb[i + 2]]
}

#[test]
fn metal_backend_clear_resolve_and_golden() {
    if !metal_ready() {
        return;
    }
    use crate::{golden, metal_backend::MetalBackend, synthetic};

    let trace = synthetic::clear_resolve(128);
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    assert_eq!(rep.warnings.total, 0, "{rep}");
    assert_eq!(rep.frames, 1, "{rep}");

    let img = backend.capture_frame().unwrap();
    assert_eq!((img.width, img.height), (128, 128));
    // Outer background: [0.1, 0.2, 0.8] -> (26, 51, 204); inner rect: [0.9, 0.8, 0.1] -> (230, 204, 26).
    let outer = pixel_at(&img, 4, 4);
    let inner = pixel_at(&img, 64, 64);
    assert!(outer[0].abs_diff(26) <= 1 && outer[1].abs_diff(51) <= 1 && outer[2].abs_diff(204) <= 1, "outer={outer:?}");
    assert!(inner[0].abs_diff(230) <= 1 && inner[1].abs_diff(204) <= 1 && inner[2].abs_diff(26) <= 1, "inner={inner:?}");

    let dir = std::env::temp_dir().join(format!("d3d12-replay-clear-golden-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let png_path = dir.join("clear_resolve.png");
    let hash_path = dir.join("clear_resolve.hash");
    golden::write_golden(&png_path, &img).unwrap();
    golden::write_golden(&hash_path, &img).unwrap();

    // Replay a second time and verify deterministic match against both golden formats.
    let mut backend2 = MetalBackend::new().unwrap();
    let rep2 = replay::replay(&trace, &mut backend2, None, 100).unwrap();
    assert_eq!(rep2.failed.total, 0, "{rep2}");
    let img2 = backend2.capture_frame().unwrap();
    assert!(golden::verify_golden(&png_path, &img2, 0).unwrap().passed);
    assert!(golden::verify_golden(&hash_path, &img2, 0).unwrap().passed);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn metal_backend_barrier_chains_and_golden() {
    if !metal_ready() {
        return;
    }
    use crate::{metal_backend::MetalBackend, synthetic};

    let trace = synthetic::barrier_chains(32);
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    assert_eq!(rep.warnings.total, 0, "{rep}");

    let img = backend.capture_frame().unwrap();
    assert_eq!((img.width, img.height), (64, 64));
    // Outer texels come from TEX_A UAV clear [0.0, 0.75, 0.25] copied to backbuffer.
    let outer = pixel_at(&img, 4, 4);
    assert!(outer[0] == 0 && outer[1].abs_diff(191) <= 1 && outer[2].abs_diff(64) <= 1, "outer={outer:?}");
    // Center [16..48, 16..48] comes from clipped RTV clear [1.0, 0.5, 0.0].
    let center = pixel_at(&img, 32, 32);
    assert!(center[0] == 255 && center[1].abs_diff(128) <= 1 && center[2] == 0, "center={center:?}");
}

#[test]
fn metal_backend_many_draws_and_golden() {
    if !metal_and_converter_ready() {
        return;
    }
    use crate::{metal_backend::MetalBackend, synthetic};

    let trace = synthetic::many_draws(64, 128);
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    assert_eq!(rep.warnings.total, 0, "{rep}");

    let img = backend.capture_frame().unwrap();
    assert_eq!((img.width, img.height), (128, 128));
    // mesh.hlsl draws 4 quadrants: q0 top-left red, q1 top-right green, q2 bottom-left blue, q3 bottom-right grey=192.
    assert_eq!(pixel_at(&img, 32, 32), [255, 0, 0], "q0 top-left");
    assert_eq!(pixel_at(&img, 96, 32), [0, 255, 0], "q1 top-right");
    assert_eq!(pixel_at(&img, 32, 96), [0, 0, 255], "q2 bottom-left");
    assert_eq!(pixel_at(&img, 96, 96), [192, 192, 192], "q3 bottom-right");
}

#[test]
fn metal_backend_descriptor_churn() {
    if !metal_and_converter_ready() {
        return;
    }
    use crate::{metal_backend::MetalBackend, synthetic};

    let trace = synthetic::descriptor_churn(64);
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    assert_eq!(rep.warnings.total, 0, "{rep}");

    let img = backend.capture_frame().unwrap();
    let px = pixel_at(&img, 16, 16);
    assert!(px[0].abs_diff(26) <= 1 && px[1].abs_diff(102) <= 1 && px[2].abs_diff(179) <= 1, "px={px:?}");

    // Confirm the compute shader wrote non-zero sampled texels into OUT_BUF (0x2002).
    let out_buf = backend.resource(0x2002).expect("OUT_BUF live");
    let slice = out_buf.shared_slice(0, 512).expect("OUT_BUF CPU readable");
    let floats: Vec<f32> = slice
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    // Every alpha channel written by static_samplers.cs.dxil samples our texture (alpha = 1.0).
    for i in 0..32 {
        assert!((floats[i * 4 + 3] - 1.0).abs() < 1e-3, "sample {i} alpha={}", floats[i * 4 + 3]);
    }
}

#[test]
fn metal_backend_indirect_draws_and_golden() {
    if !metal_and_converter_ready() {
        return;
    }
    use crate::{metal_backend::MetalBackend, synthetic};

    let trace = synthetic::indirect_draws(32, 128);
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    assert_eq!(rep.warnings.total, 0, "{rep}");

    let img = backend.capture_frame().unwrap();
    assert_eq!((img.width, img.height), (128, 128));
    // Final ExecuteIndirect command sets cullMask=0, grey=160.
    assert_eq!(pixel_at(&img, 32, 32), [255, 0, 0], "q0 top-left");
    assert_eq!(pixel_at(&img, 96, 32), [0, 255, 0], "q1 top-right");
    assert_eq!(pixel_at(&img, 32, 96), [0, 0, 255], "q2 bottom-left");
    assert_eq!(pixel_at(&img, 96, 96), [160, 160, 160], "q3 bottom-right");
}

#[test]
fn bench_timing_stats_math() {
    use crate::bench::TimingStats;

    let st = TimingStats::from_samples(vec![10.0, 10.0, 10.2, 9.8, 10.1, 9.9, 10.0]);
    assert_eq!(st.samples.len(), 7);
    assert!((st.min_ms - 9.8).abs() < 1e-9);
    assert!((st.median_ms - 10.0).abs() < 1e-9);
    assert!((st.mean_ms - 10.0).abs() < 1e-9);
    assert!((st.max_ms - 10.2).abs() < 1e-9);
    assert!(st.cv_pct < 2.0, "cv_pct={}", st.cv_pct);
    assert!(st.mad_cv_pct < 2.0, "mad_cv_pct={}", st.mad_cv_pct);

    // One large outlier inflates raw CV while MAD robust CV stays tight and honest.
    let spiked = TimingStats::from_samples(vec![10.0, 10.0, 10.1, 9.9, 10.0, 10.1, 9.9, 25.0]);
    assert!(spiked.cv_pct > 30.0, "expected raw CV to reflect outlier honestly: {}", spiked.cv_pct);
    assert!(spiked.mad_cv_pct < 3.0, "expected MAD CV to resist single outlier: {}", spiked.mad_cv_pct);
}

#[test]
fn metal_backend_bench_repeatable_timing() {
    if !metal_and_converter_ready() {
        return;
    }
    use crate::{bench, synthetic};

    // Blended full-target mesh draws force real TBDR fragment execution on every layer
    // (cannot be HSR-culled) with minimal CPU overhead, keeping AGX in a steady active P-state.
    let trace = synthetic::many_draws_blended(8, 128, 2048);
    let opts = bench::BenchOptions { iterations: 15, warmup: 16, until: None };
    let (rep1, _b1) = bench::run_bench(&trace, &opts).unwrap();
    let (rep2, mut backend) = bench::run_bench(&trace, &opts).unwrap();

    for (label, rep) in [("run1", &rep1), ("run2", &rep2)] {
        assert!(rep.failures.is_empty(), "{label}: {rep}");
        assert_eq!(rep.gpu_busy.samples.len(), 15);
        assert_eq!(rep.frame_time.samples.len(), 15);
        assert!(rep.gpu_busy.min_ms > 0.0, "{label}: expected positive gpu_busy min_ms: {rep}");
        assert!(rep.gpu_busy.median_ms > 0.0, "{label}: expected positive gpu_busy median_ms: {rep}");
        assert!(
            rep.frame_time.median_ms >= rep.gpu_busy.min_ms * 0.9,
            "{label}: wall-clock frame_ms should bound hardware gpu_busy_ms:\n{rep}"
        );
    }

    // Bounded < 15% back-to-back repeatability and robust MAD CV to stay deterministic even on
    // shared interactive developer Macs with concurrent WindowServer/simulator GPU load.
    let max_pct = 15.0;
    let min_avg = 0.5 * (rep1.gpu_busy.min_ms + rep2.gpu_busy.min_ms);
    let min_diff_pct = ((rep1.gpu_busy.min_ms - rep2.gpu_busy.min_ms).abs() / min_avg) * 100.0;
    let med_avg = 0.5 * (rep1.gpu_busy.median_ms + rep2.gpu_busy.median_ms);
    let med_diff_pct = ((rep1.gpu_busy.median_ms - rep2.gpu_busy.median_ms).abs() / med_avg) * 100.0;
    assert!(
        min_diff_pct.min(med_diff_pct) < max_pct,
        "expected back-to-back gpu_busy repeatability < {max_pct}%, got min_diff={:.2}% median_diff={:.2}%\nrun1:\n{rep1}run2:\n{rep2}",
        min_diff_pct,
        med_diff_pct
    );

    let best_mad_cv = rep1.gpu_busy.mad_cv_pct.min(rep2.gpu_busy.mad_cv_pct);
    assert!(
        best_mad_cv < max_pct,
        "expected gpu_busy robust CV (MAD) < {max_pct}%, got {:.2}% (run1={:.2}% run2={:.2}%)\nrun1:\n{rep1}run2:\n{rep2}",
        best_mad_cv,
        rep1.gpu_busy.mad_cv_pct,
        rep2.gpu_busy.mad_cv_pct
    );

    // Final rendered frame from the bench backend must still be pixel-exact.
    let img = backend.capture_frame().unwrap();
    assert_eq!(pixel_at(&img, 512, 512), [255, 0, 0]);
    assert_eq!(pixel_at(&img, 1536, 1536), [192, 192, 192]);
}

#[test]
fn indirect_va_offsets_tracks_all_address_bearing_arguments() {
    use d3d12_metal::metal::indirect::arg as ia;

    let args = [
        (ia::CONSTANT, [0, 0, 2]),               // offset 0, size 8
        (ia::VERTEX_BUFFER_VIEW, [0, 0, 0]),     // offset 8, size 16 (VA at 8)
        (ia::INDEX_BUFFER_VIEW, [0, 0, 0]),      // offset 24, size 16 (VA at 24)
        (ia::INCREMENTING_CONSTANT, [1, 0, 0]),  // offset 40, size 0
        (ia::CONSTANT_BUFFER_VIEW, [2, 0, 0]),   // offset 40, size 8 (VA at 40)
        (ia::SHADER_RESOURCE_VIEW, [3, 0, 0]),   // offset 48, size 8 (VA at 48)
        (ia::UNORDERED_ACCESS_VIEW, [4, 0, 0]),  // offset 56, size 8 (VA at 56)
        (ia::DRAW_INDEXED, [0, 0, 0]),           // offset 64, size 20
    ];
    assert_eq!(crate::metal_backend::indirect_va_offsets(&args), vec![8, 24, 40, 48, 56]);
}

#[test]
fn metal_backend_indirect_args_remap_embedded_gpu_vas() {
    if !metal_and_converter_ready() {
        return;
    }
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::layout::PlacedFootprint;
    use d3d12_metal::metal::descriptor::{heap_kind, RESOURCE_DESCRIPTOR_SIZE};
    use d3d12_metal::metal::indirect::arg as ia;
    use d3d12_metal::metal::views::SamplerDesc;
    use d3d12_metal::types::{
        dim, heap_type, resource_flags, CommandQueueDesc, DescriptorHeapDesc, HeapProperties, ResourceDesc,
    };
    use dxil_metal::root_signature::flags::DESCRIPTOR_RANGE_OFFSET_APPEND;
    use dxil_metal::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignatureVersion,
    };

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const STATIC_CS: &[u8] = include_bytes!("../../dxil-metal/tests/fixtures/static_samplers.cs.dxil");
    let cs = signed(STATIC_CS);
    let srv_range = DescriptorRange {
        range_type: DescriptorRangeType::Srv,
        num_descriptors: 1,
        base_shader_register: 0,
        register_space: 0,
        flags: 0,
        offset_in_descriptors_from_table_start: DESCRIPTOR_RANGE_OFFSET_APPEND,
    };
    let samp_range = DescriptorRange {
        range_type: DescriptorRangeType::Sampler,
        num_descriptors: 2,
        base_shader_register: 0,
        register_space: 0,
        flags: 0,
        offset_in_descriptors_from_table_start: DESCRIPTOR_RANGE_OFFSET_APPEND,
    };
    let rs_blob = RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter { kind: RootParameterKind::DescriptorTable(vec![srv_range]), visibility: 0 },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
            RootParameter { kind: RootParameterKind::DescriptorTable(vec![samp_range]), visibility: 0 },
        ],
        static_samplers: vec![],
        flags: 0,
    }
    .to_blob();

    const D: u64 = 0x5000;
    const Q: u64 = 0x5100;
    const A: u64 = 0x5200;
    const L: u64 = 0x5300;
    const R_SIG: u64 = 0x5400;
    const P: u64 = 0x5500;
    const CSIG: u64 = 0x5600;
    const TEX: u64 = 0x5700;
    const UP_TEX: u64 = 0x5701;
    const OUT_BUF: u64 = 0x5702;
    const ARGS_BUF: u64 = 0x5703;
    const SRV_HEAP: u64 = 0x5800;
    const SAMP_HEAP: u64 = 0x5801;
    const SRV_CPU: u64 = 0x7a00_0000;
    const SRV_GPU: u64 = 0x8a00_0000;
    const SAMP_CPU: u64 = 0x7b00_0000;
    const SAMP_GPU: u64 = 0x8b00_0000;
    // Deliberately synthetic capture-time GPU VA (invalid in live Metal unless remapped).
    const CAPTURED_OUT_VA: u64 = 0x4_9000_0000;
    const CAPTURED_UP_VA: u64 = 0x4_9100_0000;
    const CAPTURED_ARGS_VA: u64 = 0x4_9200_0000;

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let tdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 28,
        SampleCount: 1,
        ..Default::default()
    };
    let buf_desc = |sz: u64, flags: u32| ResourceDesc {
        Dimension: dim::BUFFER,
        Width: sz,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        Flags: flags,
        ..Default::default()
    };
    let up_desc = buf_desc(256 * 4, 0);
    let out_desc = buf_desc(1024, resource_flags::ALLOW_UNORDERED_ACCESS);
    let args_desc = buf_desc(64, 0);

    let srv_hd = DescriptorHeapDesc {
        Type: heap_kind::CBV_SRV_UAV,
        NumDescriptors: 2,
        Flags: 1,
        NodeMask: 0,
    };
    let samp_hd = DescriptorHeapDesc {
        Type: heap_kind::SAMPLER,
        NumDescriptors: 2,
        Flags: 1,
        NodeMask: 0,
    };
    let mut samp = SamplerDesc {
        filter: 0,
        address: [1, 1, 1],
        mip_lod_bias: 0.0,
        max_anisotropy: 1,
        comparison: 1,
        border: [0.0; 4],
        min_lod: 0.0,
        max_lod: 0.0,
    };
    let mut texels = vec![0u8; 256 * 4];
    for y in 0..4usize {
        for x in 0..4usize {
            let o = y * 256 + x * 4;
            texels[o..o + 4].copy_from_slice(&[80, 160, 240, 255]);
        }
    }
    // Command signature: root UAV at param 1 (8 bytes) + Dispatch (12 bytes) = 20-byte stride.
    let mut sig_bytes = Vec::new();
    for w in [ia::UNORDERED_ACCESS_VIEW, 1, 0, 0, ia::DISPATCH, 0, 0, 0] {
        sig_bytes.extend_from_slice(&w.to_le_bytes());
    }
    // Argument buffer embeds CAPTURED_OUT_VA + Dispatch(1, 1, 1).
    let mut arg_bytes = Vec::with_capacity(20);
    arg_bytes.extend_from_slice(&CAPTURED_OUT_VA.to_le_bytes());
    for w in [1u32, 1, 1] {
        arg_bytes.extend_from_slice(&w.to_le_bytes());
    }

    let mut dst_loc = vec![0u8; 48];
    dst_loc[0..8].copy_from_slice(&TEX.to_le_bytes());
    let mut src_loc = vec![0u8; 48];
    src_loc[0..8].copy_from_slice(&UP_TEX.to_le_bytes());
    src_loc[8..12].copy_from_slice(&1u32.to_le_bytes());
    let fp = PlacedFootprint { offset: 0, format: 28, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    src_loc[16..48].copy_from_slice(bytes_of(&fp));

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateRootSignature { device: D, root_signature: R_SIG, blob: &rs_blob }, 0).unwrap();
    w.write(&Call::CreateComputePipelineState { device: D, pso: P, root_signature: R_SIG, cs: &cs, flags: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandSignature { device: D, signature: CSIG, byte_stride: 20, root_signature: R_SIG, arguments: &sig_bytes }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: TEX, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&tdesc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: UP_TEX, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&up_desc), initial_state: 0, clear_value: None, gpu_va: CAPTURED_UP_VA }, 0).unwrap();
    w.write(&Call::WriteResource { resource: UP_TEX, subresource: 0, offset: 0, data: &texels }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: OUT_BUF, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&out_desc), initial_state: 0, clear_value: None, gpu_va: CAPTURED_OUT_VA }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: ARGS_BUF, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&args_desc), initial_state: 0, clear_value: None, gpu_va: CAPTURED_ARGS_VA }, 0).unwrap();
    w.write(&Call::WriteResource { resource: ARGS_BUF, subresource: 0, offset: 0, data: &arg_bytes }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: D, heap: SRV_HEAP, desc: bytes_of(&srv_hd), cpu_start: SRV_CPU, gpu_start: SRV_GPU }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: D, heap: SAMP_HEAP, desc: bytes_of(&samp_hd), cpu_start: SAMP_CPU, gpu_start: SAMP_GPU }, 0).unwrap();
    w.write(&Call::CreateShaderResourceView { resource: TEX, desc: None, dest: SRV_CPU }, 0).unwrap();
    w.write(&Call::CreateSampler { desc: bytes_of(&samp), dest: SAMP_CPU }, 0).unwrap();
    samp.filter = 0x15;
    w.write(&Call::CreateSampler { desc: bytes_of(&samp), dest: SAMP_CPU + RESOURCE_DESCRIPTOR_SIZE as u64 }, 0).unwrap();

    let heaps = [SRV_HEAP, SAMP_HEAP];
    let lists = [L];
    w.write(&Call::CreateCommandList { device: D, list: L, list_type: 0, allocator: A, initial_pso: P }, 0).unwrap();
    w.write(&Call::CopyTextureRegion { list: L, dst: &dst_loc, dst_x: 0, dst_y: 0, dst_z: 0, src: &src_loc, src_box: None }, 0).unwrap();
    w.write(&Call::SetDescriptorHeaps { list: L, heaps: U64s::from_slice(&heaps) }, 0).unwrap();
    w.write(&Call::SetComputeRootSignature { list: L, root_signature: R_SIG }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: L, compute: true, param: 0, gpu_handle: SRV_GPU }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: L, compute: true, param: 2, gpu_handle: SAMP_GPU }, 0).unwrap();
    // Root UAV param 1 is NOT set via SetRootView — it comes strictly from the indirect argument buffer's embedded VA!
    w.write(&Call::ExecuteIndirect { list: L, signature: CSIG, max_count: 1, args: ARGS_BUF, args_offset: 0, count: 0, count_offset: 0 }, 0).unwrap();
    w.write(&Call::Close { list: L }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q, lists: U64s::from_slice(&lists) }, 0).unwrap();

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    assert_eq!(rep.warnings.total, 0, "{rep}");

    let out_buf = backend.resource(OUT_BUF).expect("OUT_BUF live");
    assert_ne!(out_buf.gpu_va, CAPTURED_OUT_VA, "live buffer VA should differ from captured VA");
    let slice = out_buf.shared_slice(0, 512).expect("OUT_BUF CPU readable");
    let floats: Vec<f32> = slice
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    for i in 0..32 {
        assert!(
            (floats[i * 4 + 3] - 1.0).abs() < 1e-3,
            "sample {i} alpha={} (indirect embedded UAV VA was not relocated)",
            floats[i * 4 + 3]
        );
    }
}

#[test]
fn metal_backend_bench_recycles_timestamp_batches_and_reports_overflow() {
    if !metal_ready() {
        return;
    }
    use crate::{bench, synthetic};

    let trace = synthetic::clear_resolve(64);

    // 1. Timer capacity = 6 slots (3 timed command lists per batch) across 8 iterations forces
    //    multiple bounded batch resolves and must return real non-zero gpu_busy_ms for all 8.
    let opts = bench::BenchOptions { iterations: 8, warmup: 2, until: None };
    let (rep, _backend) = bench::run_bench_with_timer_capacity(&trace, &opts, 6).unwrap();
    assert!(rep.failures.is_empty(), "{rep}");
    assert_eq!(rep.gpu_busy.samples.len(), 8);
    for (i, &ms) in rep.gpu_busy.samples.iter().enumerate() {
        assert!(ms > 0.0, "iteration {i} reported non-positive gpu_busy_ms={ms}: {rep}");
    }

    // 2. Timer capacity = 1 slot (smaller than one Reset+Close pair) must fail explicitly
    //    instead of silently dropping timestamps and returning 0 ms.
    let small_opts = bench::BenchOptions { iterations: 2, warmup: 0, until: None };
    let err = bench::run_bench_with_timer_capacity(&trace, &small_opts, 1)
        .err()
        .expect("expected explicit error when timestamp heap capacity is smaller than one command list");
    let msg = err.to_string();
    assert!(
        msg.contains("timestamp") && (msg.contains("overflow") || msg.contains("capacity")),
        "unexpected error message: {msg}"
    );
}

#[test]
fn bench_slice_excludes_trailing_partial_frame_and_teardown() {
    use crate::{bench, synthetic};
    use d3d12_metal::trace::{Call, Reader};

    // Append trailing partial-frame Reset + Release teardown after the complete frame.
    let mut trace = synthetic::clear_resolve(64);
    let base_count = Reader::new(&trace).unwrap().count();
    {
        let mut w = Writer::new(Vec::new()).unwrap();
        for rec in Reader::new(&trace).unwrap() {
            let r = rec.unwrap();
            w.write(&r.call, r.thread).unwrap();
        }
        // Trailing partial frame + teardown Release that would break iteration 2 if included.
        w.write(&Call::Reset { list: 0x1300, allocator: 0x1200, pso: 0 }, 0).unwrap();
        w.write(&Call::Release { object: 0x1300 }, 0).unwrap();
        w.write(&Call::Release { object: 0x1600 }, 0).unwrap();
        trace = w.into_inner();
    }
    let recs: Vec<_> = Reader::new(&trace).unwrap().map(|r| r.unwrap()).collect();
    let (start, end, _) = bench::find_frame_slice(&recs).unwrap();
    assert_eq!(end, base_count, "frame_end must stop at FrameEnd and exclude trailing partial/teardown records");
    assert!(start < end);

    if !metal_ready() {
        return;
    }
    let opts = bench::BenchOptions { iterations: 4, warmup: 2, until: None };
    let (rep, mut backend) = bench::run_bench(&trace, &opts).unwrap();
    assert!(rep.failures.is_empty(), "{rep}");
    assert_eq!(rep.frame_records, end - start);
    let img = backend.capture_frame().unwrap();
    assert_eq!((img.width, img.height), (64, 64));
}

#[test]
fn metal_backend_write_resource_waits_for_pending_gpu_reads() {
    if !metal_ready() {
        return;
    }
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::types::{dim, heap_type, CommandQueueDesc, HeapProperties, ResourceDesc};

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const D: u64 = 0x6000;
    const Q: u64 = 0x6100;
    const A1: u64 = 0x6200;
    const A2: u64 = 0x6201;
    const L1: u64 = 0x6300;
    const L2: u64 = 0x6301;
    const UP: u64 = 0x6400;
    const DST1: u64 = 0x6401;
    const DST2: u64 = 0x6402;
    const SZ: usize = 64 * 1024;

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let hp_rb = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
    let bdesc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: SZ as u64,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let pat_a = vec![0xAAu8; SZ];
    let pat_b = vec![0xBBu8; SZ];
    let l1s = [L1];
    let l2s = [L2];

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A1, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A2, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: UP, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_a000_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: DST1, heap_properties: bytes_of(&hp_rb), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_a100_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: DST2, heap_properties: bytes_of(&hp_rb), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_a200_0000 }, 0).unwrap();
    // Submission 1: write 0xAA to UP and copy UP -> DST1 asynchronously.
    w.write(&Call::WriteResource { resource: UP, subresource: 0, offset: 0, data: &pat_a }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L1, list_type: 0, allocator: A1, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: L1, dst: DST1, dst_offset: 0, src: UP, src_offset: 0, size: SZ as u64 }, 0).unwrap();
    w.write(&Call::Close { list: L1 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q, lists: U64s::from_slice(&l1s) }, 0).unwrap();
    // Submission 2: immediately reuse UP with 0xBB (no CPU fence wait in trace) and copy UP -> DST2.
    w.write(&Call::WriteResource { resource: UP, subresource: 0, offset: 0, data: &pat_b }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L2, list_type: 0, allocator: A2, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: L2, dst: DST2, dst_offset: 0, src: UP, src_offset: 0, size: SZ as u64 }, 0).unwrap();
    w.write(&Call::Close { list: L2 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q, lists: U64s::from_slice(&l2s) }, 0).unwrap();

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    let d1 = backend.resource(DST1).unwrap().shared_slice(0, SZ).unwrap();
    let d2 = backend.resource(DST2).unwrap().shared_slice(0, SZ).unwrap();
    assert!(d1.iter().all(|&b| b == 0xAA), "DST1 corrupted by unsynchronized WriteResource reuse");
    assert!(d2.iter().all(|&b| b == 0xBB), "DST2 did not receive second WriteResource payload");
}

#[test]
fn metal_backend_bench_restores_fence_timelines_between_iterations() {
    if !metal_ready() {
        return;
    }
    use crate::bench;
    use d3d12_metal::types::{dim, heap_type, CommandQueueDesc, HeapProperties, ResourceDesc};

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const D: u64 = 0x7000;
    const Q_PROD: u64 = 0x7100;
    const Q_CONS: u64 = 0x7101;
    const A_PROD: u64 = 0x7200;
    const A_CONS: u64 = 0x7201;
    const L_PROD: u64 = 0x7300;
    const L_CONS: u64 = 0x7301;
    const F: u64 = 0x7400;
    const SRC: u64 = 0x7500;
    const MID: u64 = 0x7501;
    const DST: u64 = 0x7502;
    const SZ: usize = 4096;

    let qd_direct = CommandQueueDesc { Type: 0, ..Default::default() };
    let qd_compute = CommandQueueDesc { Type: 2, ..Default::default() };
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let hp_rb = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
    let bdesc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: SZ as u64,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let payload = vec![0x5Au8; SZ];
    let lp = [L_PROD];
    let lc = [L_CONS];

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q_PROD, desc: bytes_of(&qd_compute) }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q_CONS, desc: bytes_of(&qd_direct) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A_PROD, list_type: 2 }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A_CONS, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateFence { device: D, fence: F, initial_value: 0, flags: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: SRC, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_b000_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: MID, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_b100_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: DST, heap_properties: bytes_of(&hp_rb), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_b200_0000 }, 0).unwrap();
    w.write(&Call::WriteResource { resource: SRC, subresource: 0, offset: 0, data: &payload }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L_PROD, list_type: 2, allocator: A_PROD, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::Close { list: L_PROD }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L_CONS, list_type: 0, allocator: A_CONS, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::Close { list: L_CONS }, 0).unwrap();

    // Repeatable cross-queue frame: producer signals F=1, consumer waits F=1.
    w.write(&Call::Reset { list: L_PROD, allocator: A_PROD, pso: 0 }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: L_PROD, dst: MID, dst_offset: 0, src: SRC, src_offset: 0, size: SZ as u64 }, 0).unwrap();
    w.write(&Call::Close { list: L_PROD }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q_PROD, lists: U64s::from_slice(&lp) }, 0).unwrap();
    w.write(&Call::QueueSignal { queue: Q_PROD, fence: F, value: 1 }, 0).unwrap();

    w.write(&Call::QueueWait { queue: Q_CONS, fence: F, value: 1 }, 0).unwrap();
    w.write(&Call::Reset { list: L_CONS, allocator: A_CONS, pso: 0 }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: L_CONS, dst: DST, dst_offset: 0, src: MID, src_offset: 0, size: SZ as u64 }, 0).unwrap();
    w.write(&Call::Close { list: L_CONS }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q_CONS, lists: U64s::from_slice(&lc) }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();

    let trace = w.into_inner();
    let opts = bench::BenchOptions { iterations: 5, warmup: 3, until: None };
    let (rep, backend) = bench::run_bench(&trace, &opts).unwrap();
    assert!(rep.failures.is_empty(), "{rep}");
    assert_eq!(rep.gpu_busy.samples.len(), 5);
    assert_eq!(backend.snapshot_fence_values(), vec![(F, 1)]);
    let dst = backend.resource(DST).unwrap().shared_slice(0, SZ).unwrap();
    assert!(dst.iter().all(|&b| b == 0x5A), "cross-queue synced copy produced wrong bytes");
}

#[test]
fn metal_backend_indirect_args_relocate_after_deferred_write_resource() {
    if !metal_and_converter_ready() {
        return;
    }
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::layout::PlacedFootprint;
    use d3d12_metal::metal::descriptor::{heap_kind, RESOURCE_DESCRIPTOR_SIZE};
    use d3d12_metal::metal::indirect::arg as ia;
    use d3d12_metal::metal::views::SamplerDesc;
    use d3d12_metal::types::{
        dim, heap_type, resource_flags, CommandQueueDesc, DescriptorHeapDesc, HeapProperties, ResourceDesc,
    };
    use dxil_metal::root_signature::flags::DESCRIPTOR_RANGE_OFFSET_APPEND;
    use dxil_metal::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignatureVersion,
    };

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const STATIC_CS: &[u8] = include_bytes!("../../dxil-metal/tests/fixtures/static_samplers.cs.dxil");
    let cs = signed(STATIC_CS);
    let srv_range = DescriptorRange {
        range_type: DescriptorRangeType::Srv,
        num_descriptors: 1,
        base_shader_register: 0,
        register_space: 0,
        flags: 0,
        offset_in_descriptors_from_table_start: DESCRIPTOR_RANGE_OFFSET_APPEND,
    };
    let samp_range = DescriptorRange {
        range_type: DescriptorRangeType::Sampler,
        num_descriptors: 2,
        base_shader_register: 0,
        register_space: 0,
        flags: 0,
        offset_in_descriptors_from_table_start: DESCRIPTOR_RANGE_OFFSET_APPEND,
    };
    let rs_blob = RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter { kind: RootParameterKind::DescriptorTable(vec![srv_range]), visibility: 0 },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
            RootParameter { kind: RootParameterKind::DescriptorTable(vec![samp_range]), visibility: 0 },
        ],
        static_samplers: vec![],
        flags: 0,
    }
    .to_blob();

    const D: u64 = 0x8000;
    const Q: u64 = 0x8100;
    const A: u64 = 0x8200;
    const L: u64 = 0x8300;
    const R_SIG: u64 = 0x8400;
    const P: u64 = 0x8500;
    const CSIG: u64 = 0x8600;
    const TEX: u64 = 0x8700;
    const UP_TEX: u64 = 0x8701;
    const OUT_BUF: u64 = 0x8702;
    const ARGS_BUF: u64 = 0x8703;
    const SRV_HEAP: u64 = 0x8800;
    const SAMP_HEAP: u64 = 0x8801;
    const SRV_CPU: u64 = 0x7c00_0000;
    const SRV_GPU: u64 = 0x8c00_0000;
    const SAMP_CPU: u64 = 0x7d00_0000;
    const SAMP_GPU: u64 = 0x8d00_0000;
    const CAPTURED_OUT_VA: u64 = 0x4_c000_0000;

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let tdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 28,
        SampleCount: 1,
        ..Default::default()
    };
    let buf_desc = |sz: u64, flags: u32| ResourceDesc {
        Dimension: dim::BUFFER,
        Width: sz,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        Flags: flags,
        ..Default::default()
    };
    let up_desc = buf_desc(256 * 4, 0);
    let out_desc = buf_desc(1024, resource_flags::ALLOW_UNORDERED_ACCESS);
    let args_desc = buf_desc(64, 0);
    let srv_hd = DescriptorHeapDesc { Type: heap_kind::CBV_SRV_UAV, NumDescriptors: 2, Flags: 1, NodeMask: 0 };
    let samp_hd = DescriptorHeapDesc { Type: heap_kind::SAMPLER, NumDescriptors: 2, Flags: 1, NodeMask: 0 };
    let mut samp = SamplerDesc {
        filter: 0,
        address: [1, 1, 1],
        mip_lod_bias: 0.0,
        max_anisotropy: 1,
        comparison: 1,
        border: [0.0; 4],
        min_lod: 0.0,
        max_lod: 0.0,
    };
    let mut texels = vec![0u8; 256 * 4];
    for y in 0..4usize {
        for x in 0..4usize {
            let o = y * 256 + x * 4;
            texels[o..o + 4].copy_from_slice(&[32, 96, 192, 255]);
        }
    }
    let mut sig_bytes = Vec::new();
    for w in [ia::UNORDERED_ACCESS_VIEW, 1, 0, 0, ia::DISPATCH, 0, 0, 0] {
        sig_bytes.extend_from_slice(&w.to_le_bytes());
    }
    let mut arg_bytes = Vec::with_capacity(20);
    arg_bytes.extend_from_slice(&CAPTURED_OUT_VA.to_le_bytes());
    for w in [1u32, 1, 1] {
        arg_bytes.extend_from_slice(&w.to_le_bytes());
    }
    let mut dst_loc = vec![0u8; 48];
    dst_loc[0..8].copy_from_slice(&TEX.to_le_bytes());
    let mut src_loc = vec![0u8; 48];
    src_loc[0..8].copy_from_slice(&UP_TEX.to_le_bytes());
    src_loc[8..12].copy_from_slice(&1u32.to_le_bytes());
    let fp = PlacedFootprint { offset: 0, format: 28, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    src_loc[16..48].copy_from_slice(bytes_of(&fp));

    let heaps = [SRV_HEAP, SAMP_HEAP];
    let lists = [L];
    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateRootSignature { device: D, root_signature: R_SIG, blob: &rs_blob }, 0).unwrap();
    w.write(&Call::CreateComputePipelineState { device: D, pso: P, root_signature: R_SIG, cs: &cs, flags: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandSignature { device: D, signature: CSIG, byte_stride: 20, root_signature: R_SIG, arguments: &sig_bytes }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: TEX, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&tdesc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: UP_TEX, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&up_desc), initial_state: 0, clear_value: None, gpu_va: 0x4_c100_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: OUT_BUF, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&out_desc), initial_state: 0, clear_value: None, gpu_va: CAPTURED_OUT_VA }, 0).unwrap();
    // ARGS_BUF starts all-zeros at ExecuteIndirect recording time!
    w.write(&Call::CreateCommittedResource { device: D, resource: ARGS_BUF, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&args_desc), initial_state: 0, clear_value: None, gpu_va: 0x4_c200_0000 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: D, heap: SRV_HEAP, desc: bytes_of(&srv_hd), cpu_start: SRV_CPU, gpu_start: SRV_GPU }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: D, heap: SAMP_HEAP, desc: bytes_of(&samp_hd), cpu_start: SAMP_CPU, gpu_start: SAMP_GPU }, 0).unwrap();
    w.write(&Call::CreateShaderResourceView { resource: TEX, desc: None, dest: SRV_CPU }, 0).unwrap();
    w.write(&Call::CreateSampler { desc: bytes_of(&samp), dest: SAMP_CPU }, 0).unwrap();
    samp.filter = 0x15;
    w.write(&Call::CreateSampler { desc: bytes_of(&samp), dest: SAMP_CPU + RESOURCE_DESCRIPTOR_SIZE as u64 }, 0).unwrap();

    w.write(&Call::CreateCommandList { device: D, list: L, list_type: 0, allocator: A, initial_pso: P }, 0).unwrap();
    w.write(&Call::CopyTextureRegion { list: L, dst: &dst_loc, dst_x: 0, dst_y: 0, dst_z: 0, src: &src_loc, src_box: None }, 0).unwrap();
    w.write(&Call::SetDescriptorHeaps { list: L, heaps: U64s::from_slice(&heaps) }, 0).unwrap();
    w.write(&Call::SetComputeRootSignature { list: L, root_signature: R_SIG }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: L, compute: true, param: 0, gpu_handle: SRV_GPU }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: L, compute: true, param: 2, gpu_handle: SAMP_GPU }, 0).unwrap();
    w.write(&Call::ExecuteIndirect { list: L, signature: CSIG, max_count: 1, args: ARGS_BUF, args_offset: 0, count: 0, count_offset: 0 }, 0).unwrap();
    w.write(&Call::Close { list: L }, 0).unwrap();
    // Simulate `tr::sync_uploads()` emitting mapped upload bytes AFTER Close and BEFORE ExecuteCommandLists!
    w.write(&Call::WriteResource { resource: UP_TEX, subresource: 0, offset: 0, data: &texels }, 0).unwrap();
    w.write(&Call::WriteResource { resource: ARGS_BUF, subresource: 0, offset: 0, data: &arg_bytes }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q, lists: U64s::from_slice(&lists) }, 0).unwrap();

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    let out_buf = backend.resource(OUT_BUF).expect("OUT_BUF live");
    let slice = out_buf.shared_slice(0, 512).expect("OUT_BUF CPU readable");
    let floats: Vec<f32> = slice
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    for i in 0..32 {
        assert!(
            (floats[i * 4 + 3] - 1.0).abs() < 1e-3,
            "sample {i} alpha={} (deferred WriteResource indirect args were not refreshed before ExecuteCommandLists)",
            floats[i * 4 + 3]
        );
    }
}

#[test]
fn metal_backend_bench_rejects_skipped_incomplete_workload() {
    if !metal_ready() {
        return;
    }
    use crate::bench;
    use d3d12_metal::types::CommandQueueDesc;

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let lists = [3u64];
    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: 1, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: 1, queue: 2, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 4, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: 1, list: 3, list_type: 0, allocator: 4, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::Close { list: 3 }, 0).unwrap();
    // Frame contains unsupported DispatchRays -> MetalBackend returns Handled::Skipped.
    w.write(&Call::Reset { list: 3, allocator: 4, pso: 0 }, 0).unwrap();
    w.write(&Call::DispatchRays { list: 3, width: 64, height: 64, depth: 1, desc: &[0; 104] }, 0).unwrap();
    w.write(&Call::Close { list: 3 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();

    let trace = w.into_inner();
    let opts = bench::BenchOptions { iterations: 2, warmup: 0, until: None };
    let err = bench::run_bench(&trace, &opts)
        .err()
        .expect("bench must reject workloads with skipped unsupported calls");
    let msg = err.to_string();
    assert!(msg.contains("incomplete") && msg.contains("DispatchRays"), "unexpected error: {msg}");
}

#[test]
fn metal_backend_write_resource_does_not_deadlock_on_pending_cross_queue_wait() {
    if !metal_ready() {
        return;
    }
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::types::{dim, heap_type, CommandQueueDesc, HeapProperties, ResourceDesc};

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const D: u64 = 0x9000;
    const Q_GFX: u64 = 0x9100;
    const Q_COMP: u64 = 0x9101;
    const A_GFX: u64 = 0x9200;
    const A_COMP: u64 = 0x9201;
    const L_GFX: u64 = 0x9300;
    const L_COMP: u64 = 0x9301;
    const F: u64 = 0x9400;
    const UP: u64 = 0x9500;
    const MID: u64 = 0x9501;
    const DST: u64 = 0x9502;
    const SZ: usize = 4096;

    let qd_gfx = CommandQueueDesc { Type: 0, ..Default::default() };
    let qd_comp = CommandQueueDesc { Type: 2, ..Default::default() };
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let hp_rb = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
    let bdesc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: SZ as u64,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let payload = vec![0x3Cu8; SZ];
    let lg = [L_GFX];
    let lc = [L_COMP];

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q_GFX, desc: bytes_of(&qd_gfx) }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q_COMP, desc: bytes_of(&qd_comp) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A_GFX, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A_COMP, list_type: 2 }, 0).unwrap();
    w.write(&Call::CreateFence { device: D, fence: F, initial_value: 0, flags: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: UP, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_d000_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: MID, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_d100_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: DST, heap_properties: bytes_of(&hp_rb), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_d200_0000 }, 0).unwrap();
    // Consumer submits BEFORE producer and waits on fence value 1 (not signaled yet!).
    w.write(&Call::QueueWait { queue: Q_GFX, fence: F, value: 1 }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L_GFX, list_type: 0, allocator: A_GFX, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: L_GFX, dst: DST, dst_offset: 0, src: MID, src_offset: 0, size: SZ as u64 }, 0).unwrap();
    w.write(&Call::Close { list: L_GFX }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q_GFX, lists: U64s::from_slice(&lg) }, 0).unwrap();
    // WriteResource for the producer while Q_GFX is blocked waiting on F=1: must NOT deadlock!
    w.write(&Call::WriteResource { resource: UP, subresource: 0, offset: 0, data: &payload }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L_COMP, list_type: 2, allocator: A_COMP, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: L_COMP, dst: MID, dst_offset: 0, src: UP, src_offset: 0, size: SZ as u64 }, 0).unwrap();
    w.write(&Call::Close { list: L_COMP }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q_COMP, lists: U64s::from_slice(&lc) }, 0).unwrap();
    w.write(&Call::QueueSignal { queue: Q_COMP, fence: F, value: 1 }, 0).unwrap();

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    let dst = backend.resource(DST).unwrap().shared_slice(0, SZ).unwrap();
    assert!(dst.iter().all(|&b| b == 0x3C), "cross-queue consumer copy received wrong bytes");
}

#[test]
fn metal_backend_bench_instruments_already_open_list_with_zero_warmup() {
    if !metal_ready() {
        return;
    }
    use crate::bench;
    use d3d12_metal::metal::descriptor::heap_kind;
    use d3d12_metal::types::{dim, heap_type, resource_flags, CommandQueueDesc, DescriptorHeapDesc, HeapProperties, ResourceDesc};

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const D: u64 = 0xa000;
    const Q: u64 = 0xa100;
    const A: u64 = 0xa200;
    const L: u64 = 0xa300;
    const RT: u64 = 0xa400;
    const RTV_HEAP: u64 = 0xa500;
    const RTV_CPU: u64 = 0x7e00_0000;

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let rdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 128,
        Height: 128,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 28,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let rtv_hd = DescriptorHeapDesc { Type: heap_kind::RTV, NumDescriptors: 1, Flags: 0, NodeMask: 0 };
    let lists = [L];

    // Single-frame trace with NO Reset: CreateCommandList leaves L open at the end of setup.
    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: RT, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&rdesc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: D, heap: RTV_HEAP, desc: bytes_of(&rtv_hd), cpu_start: RTV_CPU, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateRenderTargetView { resource: RT, desc: None, dest: RTV_CPU }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L, list_type: 0, allocator: A, initial_pso: 0 }, 0).unwrap();
    // Frame body starts here with L already open from setup!
    w.write(&Call::ClearRenderTargetView { list: L, rtv: RTV_CPU, color: [0.2, 0.4, 0.6, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::Close { list: L }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();

    let trace = w.into_inner();
    let opts = bench::BenchOptions { iterations: 1, warmup: 0, until: None };
    let (rep, _backend) = bench::run_bench(&trace, &opts).unwrap();
    assert!(rep.failures.is_empty(), "{rep}");
    assert_eq!(rep.gpu_busy.samples.len(), 1);
    assert!(
        rep.gpu_busy.samples[0] > 0.0,
        "expected positive gpu_busy_ms for already-open list with warmup=0, got {}:\n{rep}",
        rep.gpu_busy.samples[0]
    );
}

#[test]
fn metal_backend_create_command_list1_with_zero_allocator() {
    if !metal_ready() {
        return;
    }
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::metal::descriptor::heap_kind;
    use d3d12_metal::types::{dim, heap_type, resource_flags, CommandQueueDesc, DescriptorHeapDesc, HeapProperties, ResourceDesc};

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const D: u64 = 0xb000;
    const Q: u64 = 0xb100;
    const A: u64 = 0xb200;
    const L: u64 = 0xb300;
    const RT: u64 = 0xb400;
    const RTV_HEAP: u64 = 0xb500;
    const RTV_CPU: u64 = 0x7f00_0000;

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let rdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 32,
        Height: 32,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 28,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let rtv_hd = DescriptorHeapDesc { Type: heap_kind::RTV, NumDescriptors: 1, Flags: 0, NodeMask: 0 };
    let lists = [L];

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A, list_type: 0 }, 0).unwrap();
    // CreateCommandList1 emits allocator = 0 (creates closed list without begin).
    w.write(&Call::CreateCommandList { device: D, list: L, list_type: 0, allocator: 0, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: RT, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&rdesc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: D, heap: RTV_HEAP, desc: bytes_of(&rtv_hd), cpu_start: RTV_CPU, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateRenderTargetView { resource: RT, desc: None, dest: RTV_CPU }, 0).unwrap();
    w.write(&Call::Reset { list: L, allocator: A, pso: 0 }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: L, rtv: RTV_CPU, color: [0.0, 1.0, 0.5, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::Close { list: L }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q, lists: U64s::from_slice(&lists) }, 0).unwrap();

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    assert_eq!(rep.warnings.total, 0, "{rep}");
    let img = backend.capture_frame().unwrap();
    let px = pixel_at(&img, 16, 16);
    assert!(px[0] == 0 && px[1] == 255 && px[2].abs_diff(128) <= 1, "px={px:?}");
}

#[test]
fn metal_backend_bench_rejects_repeated_submission_of_closed_list() {
    if !metal_ready() {
        return;
    }
    use crate::bench;
    use d3d12_metal::types::CommandQueueDesc;

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let lists = [3u64];
    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: 1, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: 1, queue: 2, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 4, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: 1, list: 3, list_type: 0, allocator: 0, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::Reset { list: 3, allocator: 4, pso: 0 }, 0).unwrap();
    w.write(&Call::Close { list: 3 }, 0).unwrap();
    // Submit the exact same closed command list twice without Reset.
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();

    let trace = w.into_inner();
    let opts = bench::BenchOptions { iterations: 2, warmup: 0, until: None };
    let err = bench::run_bench(&trace, &opts)
        .err()
        .expect("bench must reject repeated submission of the same closed list without Reset");
    let msg = err.to_string();
    assert!(msg.contains("repeated submission"), "unexpected error: {msg}");
}

#[test]
fn bench_slice_preserves_multi_list_recording_and_rejects_split_list() {
    use crate::bench;
    use d3d12_metal::metal::descriptor::heap_kind;
    use d3d12_metal::trace::Reader;
    use d3d12_metal::types::{dim, heap_type, resource_flags, CommandQueueDesc, DescriptorHeapDesc, HeapProperties, ResourceDesc};

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const D: u64 = 0xc000;
    const Q: u64 = 0xc100;
    const A1: u64 = 0xc200;
    const A2: u64 = 0xc201;
    const L1: u64 = 0xc300;
    const L2: u64 = 0xc301;
    const RT: u64 = 0xc400;
    const RTV_HEAP: u64 = 0xc500;
    const RTV_CPU: u64 = 0x7f10_0000;

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let rdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 32,
        Height: 32,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 28,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let rtv_hd = DescriptorHeapDesc { Type: heap_kind::RTV, NumDescriptors: 1, Flags: 0, NodeMask: 0 };
    let both = [L1, L2];

    // 1. Single-frame trace where L1 records ClearRTV BEFORE L2 calls Reset:
    //    frame_start must begin at L1's ClearRTV, NOT skip ahead to L2's Reset.
    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A1, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A2, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: RT, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&rdesc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: D, heap: RTV_HEAP, desc: bytes_of(&rtv_hd), cpu_start: RTV_CPU, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateRenderTargetView { resource: RT, desc: None, dest: RTV_CPU }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L1, list_type: 0, allocator: A1, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L2, list_type: 0, allocator: A2, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::Close { list: L2 }, 0).unwrap(); // empty setup Close on L2
    // Frame work begins on L1 BEFORE L2's Reset:
    w.write(&Call::ClearRenderTargetView { list: L1, rtv: RTV_CPU, color: [1.0, 0.0, 0.0, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::Reset { list: L2, allocator: A2, pso: 0 }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: L2, rtv: RTV_CPU, color: [0.0, 1.0, 0.0, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::Close { list: L1 }, 0).unwrap();
    w.write(&Call::Close { list: L2 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q, lists: U64s::from_slice(&both) }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();

    let trace = w.into_inner();
    let recs: Vec<_> = Reader::new(&trace).unwrap().map(|r| r.unwrap()).collect();
    let (start, end, reopen) = bench::find_frame_slice(&recs).unwrap();
    assert!(matches!(recs[start].call, Call::ClearRenderTargetView { list: L1, .. }), "got {:?}", recs[start].call.name());
    assert_eq!(reopen, vec![L1]);
    assert_eq!(end, recs.len());

    if metal_ready() {
        let opts = bench::BenchOptions { iterations: 3, warmup: 1, until: None };
        let (rep, mut backend) = bench::run_bench(&trace, &opts).unwrap();
        assert!(rep.failures.is_empty(), "{rep}");
        assert_eq!(rep.gpu_busy.samples.len(), 3);
        let img = backend.capture_frame().unwrap();
        assert_eq!(pixel_at(&img, 16, 16), [0, 255, 0]);
    }

    // 2. Multi-frame trace where L1 records ClearRTV in frame 0 and closes in frame 1 without Reset:
    //    must be rejected rather than silently dropping frame-0 bindings on iteration 2.
    let mut w2 = Writer::new(Vec::new()).unwrap();
    w2.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w2.write(&Call::CreateCommandAllocator { device: D, allocator: A1, list_type: 0 }, 0).unwrap();
    w2.write(&Call::CreateCommandList { device: D, list: L1, list_type: 0, allocator: A1, initial_pso: 0 }, 0).unwrap();
    w2.write(&Call::ClearRenderTargetView { list: L1, rtv: RTV_CPU, color: [1.0, 0.0, 0.0, 1.0], rects: &[] }, 0).unwrap();
    w2.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();
    w2.write(&Call::Close { list: L1 }, 0).unwrap();
    w2.write(&Call::FrameEnd { frame: 1 }, 0).unwrap();
    let split_trace = w2.into_inner();
    let split_recs: Vec<_> = Reader::new(&split_trace).unwrap().map(|r| r.unwrap()).collect();
    let err = bench::find_frame_slice(&split_recs)
        .err()
        .expect("must reject command list split across frame boundary");
    assert!(err.to_string().contains("recorded before frame_start"), "{err}");
}

#[test]
fn metal_backend_bench_measures_copy_queue_work() {
    if !metal_ready() {
        return;
    }
    use crate::bench;
    use d3d12_metal::types::{dim, heap_type, CommandQueueDesc, HeapProperties, ResourceDesc};

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const D: u64 = 0xd000;
    const Q_COPY: u64 = 0xd100;
    const A_COPY: u64 = 0xd200;
    const L_COPY: u64 = 0xd300;
    const SRC: u64 = 0xd400;
    const DST: u64 = 0xd401;
    const SZ: usize = 8 * 1024 * 1024;

    let qd_copy = CommandQueueDesc { Type: 3, ..Default::default() }; // D3D12_COMMAND_LIST_TYPE_COPY
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let hp_rb = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
    let bdesc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: SZ as u64,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let payload = vec![0x7Eu8; SZ];
    let lists = [L_COPY];

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q_COPY, desc: bytes_of(&qd_copy) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A_COPY, list_type: 3 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: SRC, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_e000_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: DST, heap_properties: bytes_of(&hp_rb), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_e100_0000 }, 0).unwrap();
    w.write(&Call::WriteResource { resource: SRC, subresource: 0, offset: 0, data: &payload }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L_COPY, list_type: 3, allocator: 0, initial_pso: 0 }, 0).unwrap();
    // Copy-queue-only frame body (16 x 8 MiB = 128 MiB so every iteration spans many GPU ticks):
    w.write(&Call::Reset { list: L_COPY, allocator: A_COPY, pso: 0 }, 0).unwrap();
    for _ in 0..16 {
        w.write(&Call::CopyBufferRegion { list: L_COPY, dst: DST, dst_offset: 0, src: SRC, src_offset: 0, size: SZ as u64 }, 0).unwrap();
    }
    w.write(&Call::Close { list: L_COPY }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q_COPY, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();

    let trace = w.into_inner();
    let opts = bench::BenchOptions { iterations: 4, warmup: 2, until: None };
    let (rep, backend) = bench::run_bench(&trace, &opts).unwrap();
    assert!(rep.failures.is_empty(), "{rep}");
    assert_eq!(rep.gpu_busy.samples.len(), 4);
    assert_eq!(backend.gpu_timer_pair_count(), 4, "Copy queue command lists must register timestamp pairs");
    assert!(rep.gpu_busy.max_ms > 0.0, "expected positive copy-queue gpu_busy_ms:\n{rep}");
    let dst = backend.resource(DST).unwrap().shared_slice(0, SZ).unwrap();
    assert!(dst.iter().all(|&b| b == 0x7E));
}

#[test]
fn metal_backend_bench_ignores_unsubmitted_closed_command_list() {
    if !metal_ready() {
        return;
    }
    use crate::bench;
    use d3d12_metal::types::{dim, heap_type, CommandQueueDesc, HeapProperties, ResourceDesc};

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const D: u64 = 0xe000;
    const Q: u64 = 0xe100;
    const A: u64 = 0xe200;
    const L_USED: u64 = 0xe300;
    const L_DISCARDED: u64 = 0xe301;
    const SRC: u64 = 0xe400;
    const DST: u64 = 0xe401;
    const SZ: usize = 4 * 1024 * 1024;

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let hp_rb = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
    let bdesc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: SZ as u64,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let payload = vec![0x42u8; SZ];
    let used = [L_USED];

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: SRC, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_f000_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: D, resource: DST, heap_properties: bytes_of(&hp_rb), heap_flags: 0, desc: bytes_of(&bdesc), initial_state: 0, clear_value: None, gpu_va: 0x4_f100_0000 }, 0).unwrap();
    w.write(&Call::WriteResource { resource: SRC, subresource: 0, offset: 0, data: &payload }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L_USED, list_type: 0, allocator: 0, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L_DISCARDED, list_type: 0, allocator: 0, initial_pso: 0 }, 0).unwrap();
    // Frame records L_DISCARDED and closes it without ever submitting it, plus submits L_USED.
    w.write(&Call::Reset { list: L_DISCARDED, allocator: A, pso: 0 }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: L_DISCARDED, dst: DST, dst_offset: 0, src: SRC, src_offset: 0, size: SZ as u64 }, 0).unwrap();
    w.write(&Call::Close { list: L_DISCARDED }, 0).unwrap();
    w.write(&Call::Reset { list: L_USED, allocator: A, pso: 0 }, 0).unwrap();
    for _ in 0..8 {
        w.write(&Call::CopyBufferRegion { list: L_USED, dst: DST, dst_offset: 0, src: SRC, src_offset: 0, size: SZ as u64 }, 0).unwrap();
    }
    w.write(&Call::Close { list: L_USED }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q, lists: U64s::from_slice(&used) }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();

    let trace = w.into_inner();
    let opts = bench::BenchOptions { iterations: 3, warmup: 1, until: None };
    let (rep, backend) = bench::run_bench(&trace, &opts).unwrap();
    assert!(rep.failures.is_empty(), "{rep}");
    // Only the 3 submitted L_USED recordings should be counted as timestamp pairs (not L_DISCARDED).
    assert_eq!(backend.gpu_timer_pair_count(), 3);
    assert!(rep.gpu_busy.max_ms > 0.0 && rep.gpu_busy.max_ms < 1000.0, "{rep}");
}

#[test]
fn metal_backend_bench_rejects_cross_frame_unsignaled_setup_wait() {
    if !metal_ready() {
        return;
    }
    use crate::bench;
    use d3d12_metal::types::CommandQueueDesc;

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const D: u64 = 0xf000;
    const Q_GFX: u64 = 0xf100;
    const Q_COMP: u64 = 0xf101;
    const A: u64 = 0xf200;
    const L: u64 = 0xf300;
    const F: u64 = 0xf400;

    let qd_gfx = CommandQueueDesc { Type: 0, ..Default::default() };
    let qd_comp = CommandQueueDesc { Type: 2, ..Default::default() };
    let lists = [L];

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q_GFX, desc: bytes_of(&qd_gfx) }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: D, queue: Q_COMP, desc: bytes_of(&qd_comp) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: D, allocator: A, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateFence { device: D, fence: F, initial_value: 0, flags: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: D, list: L, list_type: 0, allocator: A, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::Close { list: L }, 0).unwrap();
    // Penultimate frame (becomes setup prefix) submits work waiting on F=1 signaled in final frame:
    w.write(&Call::QueueWait { queue: Q_GFX, fence: F, value: 1 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: Q_GFX, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();
    // Final frame signals F=1:
    w.write(&Call::QueueSignal { queue: Q_COMP, fence: F, value: 1 }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 1 }, 0).unwrap();

    let trace = w.into_inner();
    let opts = bench::BenchOptions { iterations: 2, warmup: 0, until: None };
    let err = bench::run_bench(&trace, &opts)
        .err()
        .expect("bench must reject setup prefix with unsignaled cross-frame fence wait instead of deadlocking");
    assert!(err.to_string().contains("blocked on fence signals"), "{err}");

    // Wait-only setup queue: `QueueWait(F, 1)` is enqueued in setup without any `ExecuteCommandLists`
    // until the final frame; bench must still detect the unsatisfied wait before replacing `F`.
    let mut w2 = Writer::new(Vec::new()).unwrap();
    w2.write(&Call::CreateDevice { device: D, feature_level: 0xc200 }, 0).unwrap();
    w2.write(&Call::CreateCommandQueue { device: D, queue: Q_GFX, desc: bytes_of(&qd_gfx) }, 0).unwrap();
    w2.write(&Call::CreateCommandQueue { device: D, queue: Q_COMP, desc: bytes_of(&qd_comp) }, 0).unwrap();
    w2.write(&Call::CreateCommandAllocator { device: D, allocator: A, list_type: 0 }, 0).unwrap();
    w2.write(&Call::CreateFence { device: D, fence: F, initial_value: 0, flags: 0 }, 0).unwrap();
    w2.write(&Call::CreateCommandList { device: D, list: L, list_type: 0, allocator: 0, initial_pso: 0 }, 0).unwrap();
    w2.write(&Call::QueueWait { queue: Q_GFX, fence: F, value: 1 }, 0).unwrap();
    w2.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();
    // Final frame submits consumer work on Q_GFX and signals F=1 on Q_COMP:
    w2.write(&Call::Reset { list: L, allocator: A, pso: 0 }, 0).unwrap();
    w2.write(&Call::Close { list: L }, 0).unwrap();
    w2.write(&Call::ExecuteCommandLists { queue: Q_GFX, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w2.write(&Call::QueueSignal { queue: Q_COMP, fence: F, value: 1 }, 0).unwrap();
    w2.write(&Call::FrameEnd { frame: 1 }, 0).unwrap();

    let err2 = bench::run_bench(&w2.into_inner(), &opts)
        .err()
        .expect("bench must reject wait-only setup queue with unsatisfied cross-frame fence wait");
    assert!(err2.to_string().contains("blocked on fence signals"), "{err2}");
}

#[test]
fn metal_backend_replay_until_rejects_blocked_wait_without_hanging() {
    if !metal_ready() {
        return;
    }
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::types::CommandQueueDesc;

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    let qd_gfx = CommandQueueDesc { Type: 0, ..Default::default() };
    let qd_comp = CommandQueueDesc { Type: 2, ..Default::default() };
    let lists = [3u64];

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: 1, feature_level: 0xc200 }, 0).unwrap();               // 0
    w.write(&Call::CreateCommandQueue { device: 1, queue: 2, desc: bytes_of(&qd_gfx) }, 0).unwrap(); // 1
    w.write(&Call::CreateCommandQueue { device: 1, queue: 5, desc: bytes_of(&qd_comp) }, 0).unwrap(); // 2
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 4, list_type: 0 }, 0).unwrap(); // 3
    w.write(&Call::CreateFence { device: 1, fence: 6, initial_value: 0, flags: 0 }, 0).unwrap(); // 4
    w.write(&Call::CreateCommandList { device: 1, list: 3, list_type: 0, allocator: 4, initial_pso: 0 }, 0).unwrap(); // 5
    w.write(&Call::Close { list: 3 }, 0).unwrap();                                               // 6
    w.write(&Call::QueueWait { queue: 2, fence: 6, value: 1 }, 0).unwrap();                      // 7
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&lists) }, 0).unwrap(); // 8
    // Signal arrives at record 9; stopping at --until 8 leaves queue 2 blocked on fence 6=1.
    w.write(&Call::QueueSignal { queue: 5, fence: 6, value: 1 }, 0).unwrap();                    // 9

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, Some(8), 100).unwrap();
    assert_eq!(rep.failed.total, 1, "{rep}");
    assert!(
        rep.failed.items[0].1.contains("blocked on unsignaled fence waits"),
        "{:?}",
        rep.failed.items
    );
}

#[test]
fn bench_slice_rejects_list_recorded_in_setup_prefix() {
    use crate::bench;
    use d3d12_metal::trace::Reader;
    use d3d12_metal::types::CommandQueueDesc;

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let lists = [3u64];

    // Multi-frame trace: list 3 is recorded and closed in frame 0 (setup prefix), then submitted
    // for the first time in frame 1 (bench body) without being Reset/recorded inside frame 1.
    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: 1, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: 1, queue: 2, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 4, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: 1, list: 3, list_type: 0, allocator: 4, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::Close { list: 3 }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 1 }, 0).unwrap();

    let trace = w.into_inner();
    let recs: Vec<_> = Reader::new(&trace).unwrap().map(|r| r.unwrap()).collect();
    let err = bench::find_frame_slice(&recs)
        .err()
        .expect("must reject submitting a command list recorded entirely in setup prefix");
    assert!(err.to_string().contains("recorded before frame_start"), "{err}");
}

#[test]
fn metal_backend_rejects_duplicate_resource_id_and_stale_target_view() {
    if !metal_ready() {
        return;
    }
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::metal::descriptor::heap_kind;
    use d3d12_metal::types::{dim, heap_type, resource_flags, CommandQueueDesc, DescriptorHeapDesc, HeapProperties, ResourceDesc};

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let rdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 32,
        Height: 32,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 28,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let rtv_hd = DescriptorHeapDesc { Type: heap_kind::RTV, NumDescriptors: 1, Flags: 0, NodeMask: 0 };
    let rtv_cpu = 0x7f20_0000u64;

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: 1, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: 1, queue: 2, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 3, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: 1, list: 4, list_type: 0, allocator: 3, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 10, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&rdesc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: 1, heap: 20, desc: bytes_of(&rtv_hd), cpu_start: rtv_cpu, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateRenderTargetView { resource: 10, desc: None, dest: rtv_cpu }, 0).unwrap();
    // 1. Duplicate still-live resource ID 10 must be rejected without dropping the original resource.
    w.write(&Call::CreateCommittedResource { device: 1, resource: 10, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&rdesc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    // 2. Release resource 10, then attempt to ClearRenderTargetView on the now-stale RTV handle:
    //    must fail cleanly in resolve_target_view rather than dereferencing a non-live Resource*.
    w.write(&Call::Release { object: 10 }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: 4, rtv: rtv_cpu, color: [1.0, 0.0, 0.0, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::Close { list: 4 }, 0).unwrap();

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 2, "{rep}");
    assert!(
        rep.failed.items.iter().any(|(_, m)| m.contains("duplicate still-live resource id")),
        "{:?}",
        rep.failed.items
    );
    assert!(
        rep.failed.items.iter().any(|(_, m)| m.contains("no longer live")),
        "{:?}",
        rep.failed.items
    );
}

#[test]
fn metal_backend_retains_bound_dsv_after_release_and_drain() {
    if !metal_and_converter_ready() {
        return;
    }
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::metal::descriptor::heap_kind;
    use d3d12_metal::types::{
        dim, heap_type, resource_flags, CommandQueueDesc, DescriptorHeapDesc, HeapProperties, Rect,
        ResourceDesc, Viewport,
    };

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
    }

    const MS_DIRECT: &[u8] = include_bytes!("../../d3d12-metal/tests/fixtures/mesh_direct.ms.dxil");
    let ms = signed(MS_DIRECT);
    let ps = signed(MESH_PS);
    let rs_blob = RootSignatureDesc::from_blob(&ms).unwrap().to_blob();

    let mut blend = vec![0u8; 328];
    for (off, v) in [(8, 2u32), (12, 1), (16, 1), (20, 2), (24, 1), (28, 1), (32, 4)] {
        blend[8 + off..8 + off + 4].copy_from_slice(&v.to_le_bytes());
    }
    blend[8 + 36] = 0xF;
    let mut raster = vec![0u8; 44];
    raster[0..4].copy_from_slice(&3u32.to_le_bytes());
    raster[4..8].copy_from_slice(&1u32.to_le_bytes());
    raster[24..28].copy_from_slice(&1u32.to_le_bytes());
    let mut depth = vec![0u8; 52];
    depth[8..12].copy_from_slice(&8u32.to_le_bytes());
    for face in [20, 36] {
        for (k, v) in [1u32, 1, 1, 8].into_iter().enumerate() {
            depth[face + 4 * k..face + 4 * k + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
    let rtv_fmts = [28u32];
    let pso_desc = GraphicsPipeline {
        root_signature: 50,
        vs: &[],
        ps: &ps,
        ds: &[],
        hs: &[],
        gs: &[],
        stream_output: &[],
        blend: &blend,
        sample_mask: u32::MAX,
        rasterizer: &raster,
        raster_version: 0,
        depth_stencil: &depth,
        depth_stencil_version: 0,
        input_layout: &[],
        ib_strip_cut_value: 0,
        primitive_topology_type: 3,
        rtv_formats: U32s::from_slice(&rtv_fmts),
        dsv_format: 40, // DXGI_FORMAT_D32_FLOAT
        sample_count: 1,
        sample_quality: 0,
        node_mask: 0,
        flags: 0,
    };

    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let rt_desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 64,
        Height: 64,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 28,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let dsv_desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 64,
        Height: 64,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 40, // DXGI_FORMAT_D32_FLOAT
        SampleCount: 1,
        Flags: resource_flags::ALLOW_DEPTH_STENCIL,
        ..Default::default()
    };
    let up_desc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: 64,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let rtv_hd = DescriptorHeapDesc { Type: heap_kind::RTV, NumDescriptors: 1, Flags: 0, NodeMask: 0 };
    let dsv_hd = DescriptorHeapDesc { Type: heap_kind::DSV, NumDescriptors: 1, Flags: 0, NodeMask: 0 };
    let rtv_cpu = 0x7f30_0000u64;
    let dsv_cpu = 0x7f40_0000u64;
    let rtvs = [rtv_cpu];
    let vp = Viewport { x: 0.0, y: 0.0, w: 64.0, h: 64.0, min_depth: 0.0, max_depth: 1.0 };
    let sc = Rect { left: 0, top: 0, right: 64, bottom: 64 };
    let consts = [0u32, 192, 0, 0];
    let l_dummy = [5u64];
    let l_draw = [4u64];

    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: 1, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: 1, queue: 2, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 3, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 6, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateRootSignature { device: 1, root_signature: 50, blob: &rs_blob }, 0).unwrap();
    w.write(&Call::CreateMeshPipelineState { device: 1, pso: 51, amplification: &[], mesh: &ms, desc: pso_desc }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 10, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&rt_desc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 11, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&dsv_desc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 12, heap_properties: bytes_of(&hp_up), heap_flags: 0, desc: bytes_of(&up_desc), initial_state: 0, clear_value: None, gpu_va: 0x4_f500_0000 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: 1, heap: 20, desc: bytes_of(&rtv_hd), cpu_start: rtv_cpu, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: 1, heap: 21, desc: bytes_of(&dsv_hd), cpu_start: dsv_cpu, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateRenderTargetView { resource: 10, desc: None, dest: rtv_cpu }, 0).unwrap();
    w.write(&Call::CreateDepthStencilView { resource: 11, desc: None, dest: dsv_cpu }, 0).unwrap();
    // Bind RTV + DSV on list 4, then Release DSV 11, submit dummy list 5, and trigger WriteResource
    // so `drain_unblocked_queues()` clears `self.retired` BEFORE list 4 records its draw!
    w.write(&Call::CreateCommandList { device: 1, list: 4, list_type: 0, allocator: 3, initial_pso: 51 }, 0).unwrap();
    w.write(&Call::SetGraphicsRootSignature { list: 4, root_signature: 50 }, 0).unwrap();
    w.write(&Call::OMSetRenderTargets { list: 4, rtvs: U64s::from_slice(&rtvs), dsv: dsv_cpu }, 0).unwrap();
    w.write(&Call::Release { object: 11 }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: 1, list: 5, list_type: 0, allocator: 6, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::Close { list: 5 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&l_dummy) }, 0).unwrap();
    w.write(&Call::WriteResource { resource: 12, subresource: 0, offset: 0, data: &[1, 2, 3, 4] }, 0).unwrap();
    // Now draw on list 4 (which still has the DSV bound): must NOT UAF!
    w.write(&Call::RSSetViewports { list: 4, viewports: bytes_of(&vp) }, 0).unwrap();
    w.write(&Call::RSSetScissorRects { list: 4, rects: bytes_of(&sc) }, 0).unwrap();
    w.write(&Call::SetRoot32BitConstants { list: 4, compute: false, param: 0, dest_offset: 0, values: U32s::from_slice(&consts) }, 0).unwrap();
    w.write(&Call::DispatchMesh { list: 4, x: 4, y: 1, z: 1 }, 0).unwrap();
    w.write(&Call::Close { list: 4 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&l_draw) }, 0).unwrap();

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");
    let img = backend.capture_frame().unwrap();
    assert_eq!(pixel_at(&img, 16, 16), [255, 0, 0]);
    assert_eq!(pixel_at(&img, 48, 48), [192, 192, 192]);
}

#[test]
fn metal_backend_va_map_removes_released_buffers_and_preserves_aliases() {
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::types::{dim, heap_type, HeapDesc, HeapProperties, ResourceDesc};

    if d3d12_metal::metal::device().is_none() {
        return;
    }

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts((v as *const T).cast::<u8>(), std::mem::size_of::<T>()) }
    }

    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let buf_desc = |w: u64| ResourceDesc {
        Dimension: dim::BUFFER,
        Width: w,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let heap_desc = HeapDesc {
        SizeInBytes: 65536,
        Properties: hp_up,
        Alignment: 65536,
        Flags: 0,
    };

    // Case 1: release buffer 10 at 0x4_2000, then allocate larger buffer 11 spanning
    // 0x4_1000..0x4_4000; 0x4_2100 must translate into buffer 11 rather than released buffer 10.
    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: 1, feature_level: 0xc200 }, 0).unwrap();
    w.write(
        &Call::CreateCommittedResource {
            device: 1,
            resource: 10,
            heap_properties: bytes_of(&hp_up),
            heap_flags: 0,
            desc: bytes_of(&buf_desc(0x1000)),
            initial_state: 0,
            clear_value: None,
            gpu_va: 0x4_2000,
        },
        0,
    )
    .unwrap();
    w.write(&Call::Release { object: 10 }, 0).unwrap();
    w.write(
        &Call::CreateCommittedResource {
            device: 1,
            resource: 11,
            heap_properties: bytes_of(&hp_up),
            heap_flags: 0,
            desc: bytes_of(&buf_desc(0x3000)),
            initial_state: 0,
            clear_value: None,
            gpu_va: 0x4_1000,
        },
        0,
    )
    .unwrap();
    // Case 2: two live placed buffers aliasing the same recorded GPU VA 0x4_8000;
    // releasing one must keep the remaining live alias resolvable.
    w.write(&Call::CreateHeap { device: 1, heap: 20, desc: bytes_of(&heap_desc) }, 0).unwrap();
    w.write(
        &Call::CreatePlacedResource {
            device: 1,
            resource: 12,
            heap: 20,
            offset: 0,
            desc: bytes_of(&buf_desc(0x1000)),
            initial_state: 0,
            clear_value: None,
            gpu_va: 0x4_8000,
        },
        0,
    )
    .unwrap();
    w.write(
        &Call::CreatePlacedResource {
            device: 1,
            resource: 13,
            heap: 20,
            offset: 0,
            desc: bytes_of(&buf_desc(0x1000)),
            initial_state: 0,
            clear_value: None,
            gpu_va: 0x4_8000,
        },
        0,
    )
    .unwrap();
    w.write(&Call::Release { object: 12 }, 0).unwrap();

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");

    let live11 = backend.resource(11).expect("buffer 11 live").gpu_va;
    assert_eq!(backend.translate_gpu_va(0x4_2100), live11 + 0x1100);

    let live13 = backend.resource(13).expect("aliased buffer 13 live").gpu_va;
    assert_eq!(backend.translate_gpu_va(0x4_8100), live13 + 0x100);

    // Releasing the last alias removes 0x4_8000 from va_map completely.
    let mut w2 = Writer::new(Vec::new()).unwrap();
    w2.write(&Call::Release { object: 13 }, 0).unwrap();
    let rep2 = replay::replay(&w2.into_inner(), &mut backend, None, 100).unwrap();
    assert_eq!(rep2.failed.total, 0, "{rep2}");
    assert_eq!(backend.translate_gpu_va(0x4_8100), 0x4_8100);
}

#[test]
fn metal_backend_shader_visible_descriptor_overwrite_waits_for_prior_gpu_reads() {
    if !metal_ready() {
        return;
    }
    use crate::metal_backend::MetalBackend;
    use d3d12_metal::metal::descriptor::{heap_kind, RESOURCE_DESCRIPTOR_SIZE};
    use d3d12_metal::metal::views::SamplerDesc;
    use d3d12_metal::types::{
        dim, heap_type, resource_flags, CommandQueueDesc, DescriptorHeapDesc, HeapProperties,
        ResourceBarrier, ResourceDesc,
    };
    use dxil_metal::root_signature::flags::DESCRIPTOR_RANGE_OFFSET_APPEND;
    use dxil_metal::{
        dxbc, DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignatureDesc, RootSignatureVersion,
    };

    fn bytes_of<T: Copy>(v: &T) -> &[u8] {
        unsafe { std::slice::from_raw_parts((v as *const T).cast::<u8>(), std::mem::size_of::<T>()) }
    }
    fn transition(res: u64, before: u32, after: u32) -> ResourceBarrier {
        ResourceBarrier { Type: 0, Flags: 0, u: [res, (before as u64) << 32, after as u64] }
    }

    let mut cs = include_bytes!("../../dxil-metal/tests/fixtures/static_samplers.cs.dxil").to_vec();
    dxbc::sign_container(&mut cs).expect("sign DXBC fixture");
    let srv_range = DescriptorRange {
        range_type: DescriptorRangeType::Srv,
        num_descriptors: 1,
        base_shader_register: 0,
        register_space: 0,
        flags: 0,
        offset_in_descriptors_from_table_start: DESCRIPTOR_RANGE_OFFSET_APPEND,
    };
    let samp_range = DescriptorRange {
        range_type: DescriptorRangeType::Sampler,
        num_descriptors: 2,
        base_shader_register: 0,
        register_space: 0,
        flags: 0,
        offset_in_descriptors_from_table_start: DESCRIPTOR_RANGE_OFFSET_APPEND,
    };
    let rs_blob = RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter { kind: RootParameterKind::DescriptorTable(vec![srv_range]), visibility: 0 },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
            RootParameter { kind: RootParameterKind::DescriptorTable(vec![samp_range]), visibility: 0 },
        ],
        static_samplers: vec![],
        flags: 0,
    }
    .to_blob();

    let stride = RESOURCE_DESCRIPTOR_SIZE as u64;
    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let hp_rb = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
    let tex_desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 28,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let uav_desc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: 1024,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
        ..Default::default()
    };
    let rb_desc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: 1024,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let rtv_hd = DescriptorHeapDesc { Type: heap_kind::RTV, NumDescriptors: 2, Flags: 0, NodeMask: 0 };
    let stage_hd = DescriptorHeapDesc { Type: heap_kind::CBV_SRV_UAV, NumDescriptors: 2, Flags: 0, NodeMask: 0 };
    let gpu_srv_hd = DescriptorHeapDesc { Type: heap_kind::CBV_SRV_UAV, NumDescriptors: 2, Flags: 1, NodeMask: 0 };
    let gpu_samp_hd = DescriptorHeapDesc { Type: heap_kind::SAMPLER, NumDescriptors: 2, Flags: 1, NodeMask: 0 };
    let samp = SamplerDesc { address: [3, 3, 3], max_lod: f32::MAX, ..Default::default() };

    const RTV_CPU: u64 = 0x7a00_0000;
    const STAGE_CPU: u64 = 0x7b00_0000;
    const GPU_SRV_CPU: u64 = 0x7c00_0000;
    const GPU_SRV_GPU: u64 = 0x8c00_0000;
    const GPU_SAMP_CPU: u64 = 0x7d00_0000;
    const GPU_SAMP_GPU: u64 = 0x8d00_0000;
    let heaps = [22u64, 23u64];
    let l1 = [4u64];
    let l2 = [5u64];
    let l3 = [6u64];

    // Three submissions sharing shader-visible SRV slot 0 with ZERO WriteResource calls:
    //   1) slot 0 -> TEX_A (red)   -> 64 dispatches into RB_1
    //   2) CreateShaderResourceView overwrites slot 0 -> TEX_B (blue) -> 64 dispatches into RB_2
    //   3) CopyDescriptorsSimple overwrites slot 0 -> TEX_A (red) -> dispatches into RB_3
    let mut w = Writer::new(Vec::new()).unwrap();
    w.write(&Call::CreateDevice { device: 1, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: 1, queue: 2, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 3, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 30, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: 1, allocator: 31, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateRootSignature { device: 1, root_signature: 50, blob: &rs_blob }, 0).unwrap();
    w.write(&Call::CreateComputePipelineState { device: 1, pso: 51, root_signature: 50, cs: &cs, flags: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 10, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&tex_desc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 11, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&tex_desc), initial_state: 0, clear_value: None, gpu_va: 0 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 12, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&uav_desc), initial_state: 0, clear_value: None, gpu_va: 0x4_1000_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 13, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&uav_desc), initial_state: 0, clear_value: None, gpu_va: 0x4_1001_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 14, heap_properties: bytes_of(&hp_def), heap_flags: 0, desc: bytes_of(&uav_desc), initial_state: 0, clear_value: None, gpu_va: 0x4_1002_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 15, heap_properties: bytes_of(&hp_rb), heap_flags: 0, desc: bytes_of(&rb_desc), initial_state: 0, clear_value: None, gpu_va: 0x4_1003_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 16, heap_properties: bytes_of(&hp_rb), heap_flags: 0, desc: bytes_of(&rb_desc), initial_state: 0, clear_value: None, gpu_va: 0x4_1004_0000 }, 0).unwrap();
    w.write(&Call::CreateCommittedResource { device: 1, resource: 17, heap_properties: bytes_of(&hp_rb), heap_flags: 0, desc: bytes_of(&rb_desc), initial_state: 0, clear_value: None, gpu_va: 0x4_1005_0000 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: 1, heap: 20, desc: bytes_of(&rtv_hd), cpu_start: RTV_CPU, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: 1, heap: 21, desc: bytes_of(&stage_hd), cpu_start: STAGE_CPU, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: 1, heap: 22, desc: bytes_of(&gpu_srv_hd), cpu_start: GPU_SRV_CPU, gpu_start: GPU_SRV_GPU }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: 1, heap: 23, desc: bytes_of(&gpu_samp_hd), cpu_start: GPU_SAMP_CPU, gpu_start: GPU_SAMP_GPU }, 0).unwrap();
    w.write(&Call::CreateRenderTargetView { resource: 10, desc: None, dest: RTV_CPU }, 0).unwrap();
    w.write(&Call::CreateRenderTargetView { resource: 11, desc: None, dest: RTV_CPU + 32 }, 0).unwrap();
    w.write(&Call::CreateShaderResourceView { resource: 10, desc: None, dest: STAGE_CPU }, 0).unwrap();
    w.write(&Call::CreateSampler { desc: bytes_of(&samp), dest: GPU_SAMP_CPU }, 0).unwrap();
    w.write(&Call::CreateSampler { desc: bytes_of(&samp), dest: GPU_SAMP_CPU + stride }, 0).unwrap();

    // Submission 1: clear TEX_A red, TEX_B blue, point GPU_SRV_HEAP[0] -> TEX_A, dispatch 64x into RB_1.
    w.write(&Call::CreateShaderResourceView { resource: 10, desc: None, dest: GPU_SRV_CPU }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: 1, list: 4, list_type: 0, allocator: 3, initial_pso: 51 }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: 4, rtv: RTV_CPU, color: [1.0, 0.0, 0.0, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: 4, rtv: RTV_CPU + 32, color: [0.0, 0.0, 1.0, 1.0], rects: &[] }, 0).unwrap();
    let b_ab = [transition(10, 0x4, 0x40), transition(11, 0x4, 0x40)];
    let b_ab_bytes = unsafe { std::slice::from_raw_parts(b_ab.as_ptr().cast::<u8>(), std::mem::size_of_val(&b_ab)) };
    w.write(&Call::ResourceBarrier { list: 4, barriers: b_ab_bytes }, 0).unwrap();
    w.write(&Call::SetComputeRootSignature { list: 4, root_signature: 50 }, 0).unwrap();
    w.write(&Call::SetDescriptorHeaps { list: 4, heaps: U64s::from_slice(&heaps) }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: 4, compute: true, param: 0, gpu_handle: GPU_SRV_GPU }, 0).unwrap();
    w.write(&Call::SetRootView { list: 4, compute: true, param: 1, kind: 2, gpu_va: 0x4_1000_0000 }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: 4, compute: true, param: 2, gpu_handle: GPU_SAMP_GPU }, 0).unwrap();
    for _ in 0..64 {
        w.write(&Call::Dispatch { list: 4, x: 1, y: 1, z: 1 }, 0).unwrap();
    }
    w.write(&Call::ResourceBarrier { list: 4, barriers: bytes_of(&transition(12, 0x8, 0x800)) }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: 4, dst: 15, dst_offset: 0, src: 12, src_offset: 0, size: 64 }, 0).unwrap();
    w.write(&Call::Close { list: 4 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&l1) }, 0).unwrap();

    // Submission 2: overwrite GPU_SRV_HEAP[0] -> TEX_B (blue) immediately after submit 1.
    w.write(&Call::CreateShaderResourceView { resource: 11, desc: None, dest: GPU_SRV_CPU }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: 1, list: 5, list_type: 0, allocator: 30, initial_pso: 51 }, 0).unwrap();
    w.write(&Call::SetComputeRootSignature { list: 5, root_signature: 50 }, 0).unwrap();
    w.write(&Call::SetDescriptorHeaps { list: 5, heaps: U64s::from_slice(&heaps) }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: 5, compute: true, param: 0, gpu_handle: GPU_SRV_GPU }, 0).unwrap();
    w.write(&Call::SetRootView { list: 5, compute: true, param: 1, kind: 2, gpu_va: 0x4_1001_0000 }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: 5, compute: true, param: 2, gpu_handle: GPU_SAMP_GPU }, 0).unwrap();
    for _ in 0..64 {
        w.write(&Call::Dispatch { list: 5, x: 1, y: 1, z: 1 }, 0).unwrap();
    }
    w.write(&Call::ResourceBarrier { list: 5, barriers: bytes_of(&transition(13, 0x8, 0x800)) }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: 5, dst: 16, dst_offset: 0, src: 13, src_offset: 0, size: 64 }, 0).unwrap();
    w.write(&Call::Close { list: 5 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&l2) }, 0).unwrap();

    // Submission 3: overwrite GPU_SRV_HEAP[0] -> TEX_A (red) via CopyDescriptorsSimple immediately after submit 2.
    w.write(&Call::CopyDescriptorsSimple { count: 1, dest: GPU_SRV_CPU, src: STAGE_CPU, heap_type: heap_kind::CBV_SRV_UAV }, 0).unwrap();
    w.write(&Call::CreateCommandList { device: 1, list: 6, list_type: 0, allocator: 31, initial_pso: 51 }, 0).unwrap();
    w.write(&Call::SetComputeRootSignature { list: 6, root_signature: 50 }, 0).unwrap();
    w.write(&Call::SetDescriptorHeaps { list: 6, heaps: U64s::from_slice(&heaps) }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: 6, compute: true, param: 0, gpu_handle: GPU_SRV_GPU }, 0).unwrap();
    w.write(&Call::SetRootView { list: 6, compute: true, param: 1, kind: 2, gpu_va: 0x4_1002_0000 }, 0).unwrap();
    w.write(&Call::SetRootDescriptorTable { list: 6, compute: true, param: 2, gpu_handle: GPU_SAMP_GPU }, 0).unwrap();
    w.write(&Call::Dispatch { list: 6, x: 1, y: 1, z: 1 }, 0).unwrap();
    w.write(&Call::ResourceBarrier { list: 6, barriers: bytes_of(&transition(14, 0x8, 0x800)) }, 0).unwrap();
    w.write(&Call::CopyBufferRegion { list: 6, dst: 17, dst_offset: 0, src: 14, src_offset: 0, size: 64 }, 0).unwrap();
    w.write(&Call::Close { list: 6 }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: 2, lists: U64s::from_slice(&l3) }, 0).unwrap();

    let trace = w.into_inner();
    let mut backend = MetalBackend::new().unwrap();
    let rep = replay::replay(&trace, &mut backend, None, 100).unwrap();
    assert_eq!(rep.failed.total, 0, "{rep}");

    let read_rgba = |id: u64| -> [f32; 4] {
        let s = backend.resource(id).unwrap().shared_slice(0, 16).unwrap();
        [
            f32::from_le_bytes(s[0..4].try_into().unwrap()),
            f32::from_le_bytes(s[4..8].try_into().unwrap()),
            f32::from_le_bytes(s[8..12].try_into().unwrap()),
            f32::from_le_bytes(s[12..16].try_into().unwrap()),
        ]
    };
    let c1 = read_rgba(15);
    let c2 = read_rgba(16);
    let c3 = read_rgba(17);
    assert!(c1[0] > 0.9 && c1[2] < 0.1, "submission 1 must see red TEX_A, got {c1:?}");
    assert!(c2[2] > 0.9 && c2[0] < 0.1, "submission 2 must see blue TEX_B, got {c2:?}");
    assert!(c3[0] > 0.9 && c3[2] < 0.1, "submission 3 must see red TEX_A, got {c3:?}");
}

