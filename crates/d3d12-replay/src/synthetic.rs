//! Committed synthetic D3D12 trace generators for Metal 4 replay, benchmarking, and
//! regression testing:
//!
//! 1. [`many_draws`] — many mesh/pixel draws in a single frame with varying root constants
//!    and deterministic 4-quadrant swapchain output.
//! 2. [`descriptor_churn`] — heavy descriptor heap creation, `CopyDescriptorsSimple` /
//!    `CopyDescriptors` rotation, `SetDescriptorHeaps`, `SetRootDescriptorTable`, and compute
//!    dispatches sampling a texture through two samplers.
//! 3. [`indirect_draws`] — `CreateCommandSignature` + GPU argument & count buffers +
//!    `ExecuteIndirect` with per-command root-constant patches driving AS/MS/PS mesh draws.
//! 4. [`barrier_chains`] — interleaved `ResourceBarrier` transition, split (`BEGIN_ONLY` /
//!    `END_ONLY`), and UAV barriers across compute dispatches, UAV clears, buffer/texture copies,
//!    and clipped RTV clears.
//! 5. [`clear_resolve`] — zero-shader Metal 4 trace (4x MSAA RTV clear + partial scissor rect
//!    clear + `ResolveSubresource` into swapchain backbuffer + `Present`), usable even without
//!    `libmetalirconverter.dylib`.
//!
//! All shaders are small committed clean-room test fixtures from this repository (zero game
//! assets or copyrighted binaries).

use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::metal::descriptor::{heap_kind, RESOURCE_DESCRIPTOR_SIZE, TARGET_DESCRIPTOR_SIZE};
use d3d12_metal::metal::indirect::arg as indirect_arg;
use d3d12_metal::metal::views::SamplerDesc;
use d3d12_metal::trace::{Call, GraphicsPipeline, U32s, U64s, Writer};
use d3d12_metal::types::{
    dim, heap_type, resource_flags, CommandQueueDesc, DescriptorHeapDesc, HeapProperties, Rect,
    ResourceBarrier, ResourceDesc, Viewport,
};
use dxil_metal::root_signature::flags::DESCRIPTOR_RANGE_OFFSET_APPEND;
use dxil_metal::{
    dxbc, DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
    RootParameterKind, RootSignatureDesc, RootSignatureVersion,
};

const MESH_AS: &[u8] = include_bytes!("../../d3d12-metal/tests/fixtures/mesh.as.dxil");
const MESH_MS: &[u8] = include_bytes!("../../d3d12-metal/tests/fixtures/mesh.ms.dxil");
const MESH_DIRECT_MS: &[u8] = include_bytes!("../../d3d12-metal/tests/fixtures/mesh_direct.ms.dxil");
const MESH_PS: &[u8] = include_bytes!("../../d3d12-metal/tests/fixtures/mesh.ps.dxil");
const STATIC_SAMPLERS_CS: &[u8] = include_bytes!("../../dxil-metal/tests/fixtures/static_samplers.cs.dxil");

const DXGI_FORMAT_R8G8B8A8_UNORM: u32 = 28;
const DXGI_FORMAT_R32_UINT: u32 = 42;

const DEV: u64 = 0x1000;
const QUEUE: u64 = 0x1100;
const ALLOC: u64 = 0x1200;
const LIST: u64 = 0x1300;
const FENCE: u64 = 0x1400;
const SWAP: u64 = 0x1500;
const BACKBUF0: u64 = 0x1600;
const BACKBUF1: u64 = 0x1601;
const RTV_HEAP: u64 = 0x1700;
const RTV_CPU: u64 = 0x7000_0000;
const RS: u64 = 0x1800;
const PSO: u64 = 0x1900;
const CMD_SIG: u64 = 0x1a00;

/// Names accepted by [`generate_by_name`] and `d3d12-replay --generate <name>`.
pub const SYNTHETIC_NAMES: &[&str] = &[
    "many-draws",
    "descriptor-churn",
    "indirect-draws",
    "barrier-chains",
    "clear-resolve",
];

pub fn generate_by_name(name: &str) -> Result<Vec<u8>, String> {
    match name {
        "many-draws" => Ok(many_draws(256, 256)),
        "descriptor-churn" => Ok(descriptor_churn(128)),
        "indirect-draws" => Ok(indirect_draws(128, 256)),
        "barrier-chains" => Ok(barrier_chains(64)),
        "clear-resolve" => Ok(clear_resolve(128)),
        _ => Err(format!("unknown synthetic trace {name:?}; expected one of: {}", SYNTHETIC_NAMES.join(", "))),
    }
}

fn signed(b: &[u8]) -> Vec<u8> {
    let mut v = b.to_vec();
    dxbc::sign_container(&mut v).expect("sign DXBC fixture");
    v
}

fn bytes_of<T: Copy>(v: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
}

fn bytes_of_slice<T: Copy>(v: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}

fn swapchain_desc_bytes(w: u32, h: u32, fmt: u32, count: u32) -> [u8; 48] {
    let mut b = [0u8; 48];
    b[0..4].copy_from_slice(&w.to_le_bytes());
    b[4..8].copy_from_slice(&h.to_le_bytes());
    b[8..12].copy_from_slice(&fmt.to_le_bytes());
    b[16..20].copy_from_slice(&1u32.to_le_bytes()); // SampleDesc.Count = 1
    b[24..28].copy_from_slice(&0x20u32.to_le_bytes()); // DXGI_USAGE_RENDER_TARGET_OUTPUT
    b[28..32].copy_from_slice(&count.to_le_bytes());
    b
}

fn blend_bytes(enable_alpha_blend: bool) -> Vec<u8> {
    let mut b = vec![0u8; 328];
    let rt = 8;
    if enable_alpha_blend {
        // BlendEnable = TRUE, SRC_ALPHA (5), INV_SRC_ALPHA (6), ADD (1), ONE (2), ZERO (1), ADD (1), NOOP (4).
        for (off, v) in [(0, 1u32), (8, 5), (12, 6), (16, 1), (20, 2), (24, 1), (28, 1), (32, 4)] {
            b[rt + off..rt + off + 4].copy_from_slice(&v.to_le_bytes());
        }
    } else {
        for (off, v) in [(8, 2u32), (12, 1), (16, 1), (20, 2), (24, 1), (28, 1), (32, 4)] {
            b[rt + off..rt + off + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
    b[rt + 36] = 0xF;
    b
}

fn blend_opaque_bytes() -> Vec<u8> {
    blend_bytes(false)
}

fn raster_nocull_bytes() -> Vec<u8> {
    let mut b = vec![0u8; 44];
    b[0..4].copy_from_slice(&3u32.to_le_bytes()); // SOLID
    b[4..8].copy_from_slice(&1u32.to_le_bytes()); // CULL_NONE
    b[24..28].copy_from_slice(&1u32.to_le_bytes()); // DepthClipEnable = TRUE
    b
}

fn depth_off_bytes() -> Vec<u8> {
    let mut b = vec![0u8; 52];
    b[8..12].copy_from_slice(&8u32.to_le_bytes()); // ALWAYS
    for face in [20, 36] {
        for (k, v) in [1u32, 1, 1, 8].into_iter().enumerate() {
            b[face + 4 * k..face + 4 * k + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
    b
}

fn buffer_desc(size: u64, flags: u32) -> ResourceDesc {
    ResourceDesc {
        Dimension: dim::BUFFER,
        Width: size,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        Flags: flags,
        ..Default::default()
    }
}

fn tex2d_desc(w: u32, h: u32, fmt: u32, samples: u32, flags: u32) -> ResourceDesc {
    ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: w as u64,
        Height: h,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: fmt,
        SampleCount: samples,
        Flags: flags,
        ..Default::default()
    }
}

fn transition(res: u64, before: u32, after: u32, flags: u32) -> ResourceBarrier {
    ResourceBarrier {
        Type: 0,
        Flags: flags,
        u: [res, 0xFFFF_FFFFu64 | ((before as u64) << 32), after as u64],
    }
}

fn uav_barrier(res: u64) -> ResourceBarrier {
    ResourceBarrier { Type: 2, Flags: 0, u: [res, 0, 0] }
}

fn emit_common_device_swapchain(w: &mut Writer<Vec<u8>>, width: u32, height: u32) {
    let qd = CommandQueueDesc { Type: 0, ..Default::default() };
    let sc_desc = swapchain_desc_bytes(width, height, DXGI_FORMAT_R8G8B8A8_UNORM, 2);
    let rtv_hd = DescriptorHeapDesc {
        Type: heap_kind::RTV,
        NumDescriptors: 4,
        Flags: 0,
        NodeMask: 0,
    };
    w.write(&Call::CreateDevice { device: DEV, feature_level: 0xc200 }, 0).unwrap();
    w.write(&Call::CreateCommandQueue { device: DEV, queue: QUEUE, desc: bytes_of(&qd) }, 0).unwrap();
    w.write(&Call::CreateCommandAllocator { device: DEV, allocator: ALLOC, list_type: 0 }, 0).unwrap();
    w.write(&Call::CreateFence { device: DEV, fence: FENCE, initial_value: 0, flags: 0 }, 0).unwrap();
    w.write(&Call::CreateSwapChain { queue: QUEUE, swapchain: SWAP, desc: &sc_desc }, 0).unwrap();
    w.write(&Call::GetSwapChainBuffer { swapchain: SWAP, index: 0, resource: BACKBUF0 }, 0).unwrap();
    w.write(&Call::GetSwapChainBuffer { swapchain: SWAP, index: 1, resource: BACKBUF1 }, 0).unwrap();
    w.write(
        &Call::CreateDescriptorHeap {
            device: DEV,
            heap: RTV_HEAP,
            desc: bytes_of(&rtv_hd),
            cpu_start: RTV_CPU,
            gpu_start: 0,
        },
        0,
    )
    .unwrap();
    w.write(&Call::CreateRenderTargetView { resource: BACKBUF0, desc: None, dest: RTV_CPU }, 0).unwrap();
    w.write(
        &Call::CreateRenderTargetView {
            resource: BACKBUF1,
            desc: None,
            dest: RTV_CPU + TARGET_DESCRIPTOR_SIZE as u64,
        },
        0,
    )
    .unwrap();
    // Create the command list closed so the repeatable frame starts cleanly at `Reset`.
    w.write(&Call::CreateCommandList { device: DEV, list: LIST, list_type: 0, allocator: ALLOC, initial_pso: 0 }, 0).unwrap();
    w.write(&Call::Close { list: LIST }, 0).unwrap();
}

/// 1. Many draws in one frame (`draw_count` mesh draws into a `size x size` swapchain target),
///    producing a deterministic 4-quadrant RGBA8 image (red, green, blue, grey=192).
pub fn many_draws(draw_count: u32, size: u32) -> Vec<u8> {
    many_draws_with_blend(draw_count, 1, size, false)
}

/// Variant of [`many_draws`] with alpha blending enabled (`SRC_ALPHA, INV_SRC_ALPHA`, alpha=1.0)
/// and `layers_per_draw` Y-threadgroups per `DispatchMesh` so TBDR hidden-surface removal
/// executes every fragment pass with minimal CPU draw overhead for sustained GPU benchmarking.
pub fn many_draws_blended(draw_count: u32, layers_per_draw: u32, size: u32) -> Vec<u8> {
    many_draws_with_blend(draw_count, layers_per_draw.max(1), size, true)
}

fn many_draws_with_blend(draw_count: u32, layers_per_draw: u32, size: u32, alpha_blend: bool) -> Vec<u8> {
    let ms = signed(MESH_DIRECT_MS);
    let ps = signed(MESH_PS);
    let rs_blob = RootSignatureDesc::from_blob(&ms).unwrap().to_blob();
    let blend = blend_bytes(alpha_blend);
    let raster = raster_nocull_bytes();
    let depth = depth_off_bytes();
    let rtv_fmts = [DXGI_FORMAT_R8G8B8A8_UNORM];
    let pso_desc = GraphicsPipeline {
        root_signature: RS,
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
        dsv_format: 0,
        sample_count: 1,
        sample_quality: 0,
        node_mask: 0,
        flags: 0,
    };

    let mut w = Writer::new(Vec::new()).unwrap();
    emit_common_device_swapchain(&mut w, size, size);
    w.write(&Call::CreateRootSignature { device: DEV, root_signature: RS, blob: &rs_blob }, 0).unwrap();
    w.write(&Call::CreateMeshPipelineState { device: DEV, pso: PSO, amplification: &[], mesh: &ms, desc: pso_desc }, 0).unwrap();

    // Repeatable frame starts at Reset.
    let vp = Viewport { x: 0.0, y: 0.0, w: size as f32, h: size as f32, min_depth: 0.0, max_depth: 1.0 };
    let sc = Rect { left: 0, top: 0, right: size as i32, bottom: size as i32 };
    let rtv_handles = [RTV_CPU];
    let lists = [LIST];

    w.write(&Call::Reset { list: LIST, allocator: ALLOC, pso: PSO }, 0).unwrap();
    w.write(&Call::SetGraphicsRootSignature { list: LIST, root_signature: RS }, 0).unwrap();
    w.write(&Call::OMSetRenderTargets { list: LIST, rtvs: U64s::from_slice(&rtv_handles), dsv: 0 }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: LIST, rtv: RTV_CPU, color: [0.05, 0.05, 0.08, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::RSSetViewports { list: LIST, viewports: bytes_of(&vp) }, 0).unwrap();
    w.write(&Call::RSSetScissorRects { list: LIST, rects: bytes_of(&sc) }, 0).unwrap();

    let n = draw_count.max(1);
    for i in 0..n {
        let grey = if i + 1 == n { 192 } else { (i * 17) & 0xFF };
        let consts = [0u32, grey, 0, 0];
        w.write(
            &Call::SetRoot32BitConstants {
                list: LIST,
                compute: false,
                param: 0,
                dest_offset: 0,
                values: U32s::from_slice(&consts),
            },
            0,
        )
        .unwrap();
        w.write(&Call::DispatchMesh { list: LIST, x: 4, y: layers_per_draw, z: 1 }, 0).unwrap();
    }
    w.write(&Call::Close { list: LIST }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: QUEUE, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::QueueSignal { queue: QUEUE, fence: FENCE, value: 1 }, 0).unwrap();
    w.write(&Call::Present { swapchain: SWAP, sync_interval: 0, flags: 0 }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();
    w.into_inner()
}

/// 2. Descriptor churn: rotates SRV and sampler descriptors across staging and shader-visible
///    heaps (`CreateShaderResourceView`, `CreateSampler`, `CopyDescriptorsSimple`,
///    `CopyDescriptors`, `SetDescriptorHeaps`, `SetRootDescriptorTable`) while dispatching
///    `static_samplers.cs.dxil` `churn_count` times and visualizing the result into the swapchain.
pub fn descriptor_churn(churn_count: u32) -> Vec<u8> {
    let cs = signed(STATIC_SAMPLERS_CS);
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

    const TEX: u64 = 0x2000;
    const UPLOAD_TEX: u64 = 0x2001;
    const OUT_BUF: u64 = 0x2002;
    const STAGE_SRV_HEAP: u64 = 0x2100;
    const GPU_SRV_HEAP: u64 = 0x2101;
    const STAGE_SAMP_HEAP: u64 = 0x2102;
    const GPU_SAMP_HEAP: u64 = 0x2103;
    const STAGE_SRV_CPU: u64 = 0x7100_0000;
    const GPU_SRV_CPU: u64 = 0x7200_0000;
    const GPU_SRV_GPU: u64 = 0x8200_0000;
    const STAGE_SAMP_CPU: u64 = 0x7300_0000;
    const GPU_SAMP_CPU: u64 = 0x7400_0000;
    const GPU_SAMP_GPU: u64 = 0x8400_0000;
    const OUT_VA: u64 = 0x4_0010_0000;
    const UP_VA: u64 = 0x4_0020_0000;

    let n = churn_count.max(4);
    let stride = RESOURCE_DESCRIPTOR_SIZE as u64;
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };

    let mut w = Writer::new(Vec::new()).unwrap();
    emit_common_device_swapchain(&mut w, 64, 64);

    // Source texture (4x4 RGBA8) + upload staging buffer + UAV result buffer (32 * float4 = 512B).
    let tdesc = tex2d_desc(4, 4, DXGI_FORMAT_R8G8B8A8_UNORM, 1, 0);
    let udesc = buffer_desc(256 * 4, 0);
    let odesc = buffer_desc(1024, resource_flags::ALLOW_UNORDERED_ACCESS);
    w.write(
        &Call::CreateCommittedResource {
            device: DEV,
            resource: TEX,
            heap_properties: bytes_of(&hp_def),
            heap_flags: 0,
            desc: bytes_of(&tdesc),
            initial_state: 0,
            clear_value: None,
            gpu_va: 0,
        },
        0,
    )
    .unwrap();
    w.write(
        &Call::CreateCommittedResource {
            device: DEV,
            resource: UPLOAD_TEX,
            heap_properties: bytes_of(&hp_up),
            heap_flags: 0,
            desc: bytes_of(&udesc),
            initial_state: 0,
            clear_value: None,
            gpu_va: UP_VA,
        },
        0,
    )
    .unwrap();
    let mut texels = vec![0u8; 256 * 4];
    for y in 0..4usize {
        for x in 0..4usize {
            let o = y * 256 + x * 4;
            texels[o..o + 4].copy_from_slice(&[(x * 64) as u8, (y * 64) as u8, 200, 255]);
        }
    }
    w.write(&Call::WriteResource { resource: UPLOAD_TEX, subresource: 0, offset: 0, data: &texels }, 0).unwrap();
    w.write(
        &Call::CreateCommittedResource {
            device: DEV,
            resource: OUT_BUF,
            heap_properties: bytes_of(&hp_def),
            heap_flags: 0,
            desc: bytes_of(&odesc),
            initial_state: 0,
            clear_value: None,
            gpu_va: OUT_VA,
        },
        0,
    )
    .unwrap();

    // Descriptor heaps.
    let stage_srv_hd = DescriptorHeapDesc { Type: heap_kind::CBV_SRV_UAV, NumDescriptors: 4, Flags: 0, NodeMask: 0 };
    let gpu_srv_hd = DescriptorHeapDesc { Type: heap_kind::CBV_SRV_UAV, NumDescriptors: n + 4, Flags: 1, NodeMask: 0 };
    let stage_samp_hd = DescriptorHeapDesc { Type: heap_kind::SAMPLER, NumDescriptors: 4, Flags: 0, NodeMask: 0 };
    let gpu_samp_hd = DescriptorHeapDesc { Type: heap_kind::SAMPLER, NumDescriptors: (n + 4) * 2, Flags: 1, NodeMask: 0 };
    w.write(&Call::CreateDescriptorHeap { device: DEV, heap: STAGE_SRV_HEAP, desc: bytes_of(&stage_srv_hd), cpu_start: STAGE_SRV_CPU, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: DEV, heap: GPU_SRV_HEAP, desc: bytes_of(&gpu_srv_hd), cpu_start: GPU_SRV_CPU, gpu_start: GPU_SRV_GPU }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: DEV, heap: STAGE_SAMP_HEAP, desc: bytes_of(&stage_samp_hd), cpu_start: STAGE_SAMP_CPU, gpu_start: 0 }, 0).unwrap();
    w.write(&Call::CreateDescriptorHeap { device: DEV, heap: GPU_SAMP_HEAP, desc: bytes_of(&gpu_samp_hd), cpu_start: GPU_SAMP_CPU, gpu_start: GPU_SAMP_GPU }, 0).unwrap();

    // Initial staging descriptors.
    w.write(&Call::CreateShaderResourceView { resource: TEX, desc: None, dest: STAGE_SRV_CPU }, 0).unwrap();
    let s_clamp = SamplerDesc { address: [3, 3, 3], max_lod: f32::MAX, ..Default::default() };
    let s_wrap = SamplerDesc { address: [1, 1, 1], max_lod: f32::MAX, ..Default::default() };
    w.write(&Call::CreateSampler { desc: bytes_of(&s_clamp), dest: STAGE_SAMP_CPU }, 0).unwrap();
    w.write(&Call::CreateSampler { desc: bytes_of(&s_wrap), dest: STAGE_SAMP_CPU + stride }, 0).unwrap();

    w.write(&Call::CreateRootSignature { device: DEV, root_signature: RS, blob: &rs_blob }, 0).unwrap();
    w.write(&Call::CreateComputePipelineState { device: DEV, pso: PSO, root_signature: RS, cs: &cs, flags: 0 }, 0).unwrap();

    // Repeatable frame starts at Reset.
    let heaps = [GPU_SRV_HEAP, GPU_SAMP_HEAP];
    let lists = [LIST];
    let dst_loc = copy_loc_subresource(TEX, 0);
    let src_loc = copy_loc_footprint(
        UPLOAD_TEX,
        PlacedFootprint { offset: 0, format: DXGI_FORMAT_R8G8B8A8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 },
    );
    w.write(&Call::Reset { list: LIST, allocator: ALLOC, pso: PSO }, 0).unwrap();
    w.write(&Call::CopyTextureRegion { list: LIST, dst: &dst_loc, dst_x: 0, dst_y: 0, dst_z: 0, src: &src_loc, src_box: None }, 0).unwrap();
    let b_tex = [transition(TEX, 0x400, 0x40, 0)]; // COPY_DEST -> NON_PIXEL_SHADER_RESOURCE
    w.write(&Call::ResourceBarrier { list: LIST, barriers: bytes_of_slice(&b_tex) }, 0).unwrap();
    w.write(&Call::SetComputeRootSignature { list: LIST, root_signature: RS }, 0).unwrap();
    w.write(&Call::SetDescriptorHeaps { list: LIST, heaps: U64s::from_slice(&heaps) }, 0).unwrap();
    w.write(&Call::SetRootView { list: LIST, compute: true, param: 1, kind: 2, gpu_va: OUT_VA }, 0).unwrap();

    for i in 0..n as u64 {
        let srv_dst = GPU_SRV_CPU + i * stride;
        let samp_dst = GPU_SAMP_CPU + (i * 2) * stride;
        if i % 2 == 0 {
            w.write(&Call::CopyDescriptorsSimple { count: 1, dest: srv_dst, src: STAGE_SRV_CPU, heap_type: heap_kind::CBV_SRV_UAV }, 0).unwrap();
            w.write(&Call::CopyDescriptorsSimple { count: 2, dest: samp_dst, src: STAGE_SAMP_CPU, heap_type: heap_kind::SAMPLER }, 0).unwrap();
        } else {
            let dr = [srv_dst, 1];
            let sr = [STAGE_SRV_CPU, 1];
            w.write(&Call::CopyDescriptors { dest_ranges: U64s::from_slice(&dr), src_ranges: U64s::from_slice(&sr), heap_type: heap_kind::CBV_SRV_UAV }, 0).unwrap();
            let sdr = [samp_dst, 2];
            let ssr = [STAGE_SAMP_CPU, 2];
            w.write(&Call::CopyDescriptors { dest_ranges: U64s::from_slice(&sdr), src_ranges: U64s::from_slice(&ssr), heap_type: heap_kind::SAMPLER }, 0).unwrap();
        }
        w.write(&Call::SetRootDescriptorTable { list: LIST, compute: true, param: 0, gpu_handle: GPU_SRV_GPU + i * stride }, 0).unwrap();
        w.write(&Call::SetRootDescriptorTable { list: LIST, compute: true, param: 2, gpu_handle: GPU_SAMP_GPU + (i * 2) * stride }, 0).unwrap();
        w.write(&Call::Dispatch { list: LIST, x: 1, y: 1, z: 1 }, 0).unwrap();
        let ub = [uav_barrier(OUT_BUF)];
        w.write(&Call::ResourceBarrier { list: LIST, barriers: bytes_of_slice(&ub) }, 0).unwrap();
    }

    // Clear backbuffer deterministically so `--golden` validates cleanly.
    w.write(&Call::ClearRenderTargetView { list: LIST, rtv: RTV_CPU, color: [0.1, 0.4, 0.7, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::Close { list: LIST }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: QUEUE, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::QueueSignal { queue: QUEUE, fence: FENCE, value: 1 }, 0).unwrap();
    w.write(&Call::Present { swapchain: SWAP, sync_interval: 0, flags: 0 }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();
    w.into_inner()
}

/// 3. Indirect draws: builds a `CommandSignature` with root constants + `DispatchMesh`, populates
///    argument & count buffers via `WriteResource`, and executes `ExecuteIndirect` driving the
///    AS + MS + PS mesh shader pipeline into a `size x size` swapchain target.
pub fn indirect_draws(cmd_count: u32, size: u32) -> Vec<u8> {
    let amp = signed(MESH_AS);
    let ms = signed(MESH_MS);
    let ps = signed(MESH_PS);
    let rs_blob = RootSignatureDesc::from_blob(&amp).unwrap().to_blob();
    let blend = blend_opaque_bytes();
    let raster = raster_nocull_bytes();
    let depth = depth_off_bytes();
    let rtv_fmts = [DXGI_FORMAT_R8G8B8A8_UNORM];
    let pso_desc = GraphicsPipeline {
        root_signature: RS,
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
        dsv_format: 0,
        sample_count: 1,
        sample_quality: 0,
        node_mask: 0,
        flags: 0,
    };

    const ARGS_BUF: u64 = 0x3000;
    const COUNT_BUF: u64 = 0x3001;
    const ARGS_VA: u64 = 0x4_0030_0000;
    const COUNT_VA: u64 = 0x4_0040_0000;

    let n = cmd_count.max(2);
    // Each indirect command: 4 root constants (16B) + DispatchMesh args (12B) = 28B -> stride 32B.
    const STRIDE: u32 = 32;
    let mut sig_args = Vec::new();
    // Arg 0: CONSTANT (param 0, dest_offset 0, num_32bit_values 4).
    for w in [indirect_arg::CONSTANT, 0u32, 0, 4] {
        sig_args.extend_from_slice(&w.to_le_bytes());
    }
    // Arg 1: DISPATCH_MESH.
    for w in [indirect_arg::DISPATCH_MESH, 0u32, 0, 0] {
        sig_args.extend_from_slice(&w.to_le_bytes());
    }

    let mut cmd_bytes = vec![0u8; (n * STRIDE) as usize];
    for i in 0..n {
        let base = (i * STRIDE) as usize;
        // Final active command renders all 4 quadrants with grey = 160; earlier commands vary cullMask.
        let (cull, grey) = if i + 1 == n { (0u32, 160u32) } else { (i & 0xF, (i * 23) & 0xFF) };
        let words = [cull, grey, 0u32, 0u32, 1u32, 1u32, 1u32, 0u32];
        for (k, v) in words.into_iter().enumerate() {
            cmd_bytes[base + k * 4..base + k * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
    let count_bytes = n.to_le_bytes();
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let args_desc = buffer_desc(cmd_bytes.len() as u64, 0);
    let count_desc = buffer_desc(64, 0);

    let mut w = Writer::new(Vec::new()).unwrap();
    emit_common_device_swapchain(&mut w, size, size);
    w.write(&Call::CreateRootSignature { device: DEV, root_signature: RS, blob: &rs_blob }, 0).unwrap();
    w.write(&Call::CreateMeshPipelineState { device: DEV, pso: PSO, amplification: &amp, mesh: &ms, desc: pso_desc }, 0).unwrap();
    w.write(&Call::CreateCommandSignature { device: DEV, signature: CMD_SIG, root_signature: RS, byte_stride: STRIDE, arguments: &sig_args }, 0).unwrap();
    w.write(
        &Call::CreateCommittedResource {
            device: DEV,
            resource: ARGS_BUF,
            heap_properties: bytes_of(&hp_up),
            heap_flags: 0,
            desc: bytes_of(&args_desc),
            initial_state: 0,
            clear_value: None,
            gpu_va: ARGS_VA,
        },
        0,
    )
    .unwrap();
    w.write(
        &Call::CreateCommittedResource {
            device: DEV,
            resource: COUNT_BUF,
            heap_properties: bytes_of(&hp_up),
            heap_flags: 0,
            desc: bytes_of(&count_desc),
            initial_state: 0,
            clear_value: None,
            gpu_va: COUNT_VA,
        },
        0,
    )
    .unwrap();
    w.write(&Call::WriteResource { resource: ARGS_BUF, subresource: 0, offset: 0, data: &cmd_bytes }, 0).unwrap();
    w.write(&Call::WriteResource { resource: COUNT_BUF, subresource: 0, offset: 0, data: &count_bytes }, 0).unwrap();

    // Repeatable frame starts at Reset.
    let vp = Viewport { x: 0.0, y: 0.0, w: size as f32, h: size as f32, min_depth: 0.0, max_depth: 1.0 };
    let sc = Rect { left: 0, top: 0, right: size as i32, bottom: size as i32 };
    let rtv_handles = [RTV_CPU];
    let lists = [LIST];

    w.write(&Call::Reset { list: LIST, allocator: ALLOC, pso: PSO }, 0).unwrap();
    w.write(&Call::SetGraphicsRootSignature { list: LIST, root_signature: RS }, 0).unwrap();
    w.write(&Call::OMSetRenderTargets { list: LIST, rtvs: U64s::from_slice(&rtv_handles), dsv: 0 }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: LIST, rtv: RTV_CPU, color: [0.0, 0.0, 0.0, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::RSSetViewports { list: LIST, viewports: bytes_of(&vp) }, 0).unwrap();
    w.write(&Call::RSSetScissorRects { list: LIST, rects: bytes_of(&sc) }, 0).unwrap();
    w.write(
        &Call::ExecuteIndirect {
            list: LIST,
            signature: CMD_SIG,
            max_count: n,
            args: ARGS_BUF,
            args_offset: 0,
            count: COUNT_BUF,
            count_offset: 0,
        },
        0,
    )
    .unwrap();
    w.write(&Call::Close { list: LIST }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: QUEUE, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::QueueSignal { queue: QUEUE, fence: FENCE, value: 1 }, 0).unwrap();
    w.write(&Call::Present { swapchain: SWAP, sync_interval: 0, flags: 0 }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();
    w.into_inner()
}

/// 4. Barrier chains: exercises interleaved transition, split (`BEGIN_ONLY` / `END_ONLY`), and UAV
///    barriers across UAV clears, buffer copies, texture copies, and partial-rect RTV clears.
pub fn barrier_chains(chain_len: u32) -> Vec<u8> {
    const BUF_A: u64 = 0x4000;
    const BUF_B: u64 = 0x4001;
    const TEX_A: u64 = 0x4002;
    const UAV_HEAP: u64 = 0x4100;
    const UAV_CPU: u64 = 0x7500_0000;
    const UAV_GPU: u64 = 0x8500_0000;
    const BUF_A_VA: u64 = 0x4_0050_0000;
    const BUF_B_VA: u64 = 0x4_0060_0000;

    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let bdesc = buffer_desc(4096, resource_flags::ALLOW_UNORDERED_ACCESS);
    let tdesc = tex2d_desc(64, 64, DXGI_FORMAT_R8G8B8A8_UNORM, 1, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_hd = DescriptorHeapDesc { Type: heap_kind::CBV_SRV_UAV, NumDescriptors: 4, Flags: 1, NodeMask: 0 };
    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&DXGI_FORMAT_R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes()); // BUFFER
    uav_desc[16..20].copy_from_slice(&1024u32.to_le_bytes()); // NumElements

    let mut w = Writer::new(Vec::new()).unwrap();
    emit_common_device_swapchain(&mut w, 64, 64);

    for (id, va) in [(BUF_A, BUF_A_VA), (BUF_B, BUF_B_VA)] {
        w.write(
            &Call::CreateCommittedResource {
                device: DEV,
                resource: id,
                heap_properties: bytes_of(&hp_def),
                heap_flags: 0,
                desc: bytes_of(&bdesc),
                initial_state: 0,
                clear_value: None,
                gpu_va: va,
            },
            0,
        )
        .unwrap();
    }
    w.write(
        &Call::CreateCommittedResource {
            device: DEV,
            resource: TEX_A,
            heap_properties: bytes_of(&hp_def),
            heap_flags: 0,
            desc: bytes_of(&tdesc),
            initial_state: 0,
            clear_value: None,
            gpu_va: 0,
        },
        0,
    )
    .unwrap();
    w.write(
        &Call::CreateDescriptorHeap {
            device: DEV,
            heap: UAV_HEAP,
            desc: bytes_of(&uav_hd),
            cpu_start: UAV_CPU,
            gpu_start: UAV_GPU,
        },
        0,
    )
    .unwrap();
    w.write(&Call::CreateUnorderedAccessView { resource: BUF_A, counter: 0, desc: Some(&uav_desc), dest: UAV_CPU }, 0).unwrap();
    let stride = RESOURCE_DESCRIPTOR_SIZE as u64;
    w.write(&Call::CreateUnorderedAccessView { resource: TEX_A, counter: 0, desc: None, dest: UAV_CPU + stride }, 0).unwrap();

    // Repeatable frame starts at Reset.
    let lists = [LIST];
    let n = chain_len.max(4);
    w.write(&Call::Reset { list: LIST, allocator: ALLOC, pso: 0 }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: LIST, rtv: RTV_CPU, color: [0.2, 0.2, 0.25, 1.0], rects: &[] }, 0).unwrap();

    for i in 0..n {
        let uval = [i + 1, 0, 0, 0];
        w.write(
            &Call::ClearUnorderedAccessViewUint {
                list: LIST,
                gpu_handle: UAV_GPU,
                cpu_handle: UAV_CPU,
                resource: BUF_A,
                values: U32s::from_slice(&uval),
                rects: &[],
            },
            0,
        )
        .unwrap();
        let b1 = [
            uav_barrier(BUF_A),
            transition(BUF_A, 0x8, 0x800, 1), // BEGIN_ONLY UAV -> COPY_SOURCE
            transition(BUF_B, 0x0, 0x400, 0), // COMMON -> COPY_DEST
            transition(BUF_A, 0x8, 0x800, 2), // END_ONLY UAV -> COPY_SOURCE
        ];
        w.write(&Call::ResourceBarrier { list: LIST, barriers: bytes_of_slice(&b1) }, 0).unwrap();
        w.write(&Call::CopyBufferRegion { list: LIST, dst: BUF_B, dst_offset: 0, src: BUF_A, src_offset: 0, size: 4096 }, 0).unwrap();
        let b2 = [
            transition(BUF_A, 0x800, 0x8, 0),
            transition(BUF_B, 0x400, 0x0, 0),
        ];
        w.write(&Call::ResourceBarrier { list: LIST, barriers: bytes_of_slice(&b2) }, 0).unwrap();
        w.write(
            &Call::ClearUnorderedAccessViewFloat {
                list: LIST,
                gpu_handle: UAV_GPU + stride,
                cpu_handle: UAV_CPU + stride,
                resource: TEX_A,
                values: [0.0, 0.75, 0.25, 1.0],
                rects: &[],
            },
            0,
        )
        .unwrap();
    }
    let b_copy = [transition(TEX_A, 0x8, 0x800, 0), transition(BACKBUF0, 0x4, 0x400, 0)];
    w.write(&Call::ResourceBarrier { list: LIST, barriers: bytes_of_slice(&b_copy) }, 0).unwrap();
    w.write(&Call::CopyResource { list: LIST, dst: BACKBUF0, src: TEX_A }, 0).unwrap();
    let b_rtv = [transition(BACKBUF0, 0x400, 0x4, 0)];
    w.write(&Call::ResourceBarrier { list: LIST, barriers: bytes_of_slice(&b_rtv) }, 0).unwrap();
    let center_rect = Rect { left: 16, top: 16, right: 48, bottom: 48 };
    w.write(&Call::ClearRenderTargetView { list: LIST, rtv: RTV_CPU, color: [1.0, 0.5, 0.0, 1.0], rects: bytes_of(&center_rect) }, 0).unwrap();
    let b_pres = [transition(BACKBUF0, 0x4, 0x0, 0)];
    w.write(&Call::ResourceBarrier { list: LIST, barriers: bytes_of_slice(&b_pres) }, 0).unwrap();
    w.write(&Call::Close { list: LIST }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: QUEUE, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::QueueSignal { queue: QUEUE, fence: FENCE, value: 1 }, 0).unwrap();
    w.write(&Call::Present { swapchain: SWAP, sync_interval: 0, flags: 0 }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();
    w.into_inner()
}

/// 5. Zero-shader trace: 4x MSAA RTV clear + partial scissor rect clear + `ResolveSubresource`
///    into the swapchain backbuffer + `Present`. Runs on any Apple Silicon Metal 4 device even
///    when `libmetalirconverter.dylib` is not installed.
pub fn clear_resolve(size: u32) -> Vec<u8> {
    const MSAA_RT: u64 = 0x5000;
    let msaa_rtv_cpu = RTV_CPU + 2 * TARGET_DESCRIPTOR_SIZE as u64;
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let msaa_desc = tex2d_desc(size, size, DXGI_FORMAT_R8G8B8A8_UNORM, 4, resource_flags::ALLOW_RENDER_TARGET);

    let mut w = Writer::new(Vec::new()).unwrap();
    emit_common_device_swapchain(&mut w, size, size);
    w.write(
        &Call::CreateCommittedResource {
            device: DEV,
            resource: MSAA_RT,
            heap_properties: bytes_of(&hp_def),
            heap_flags: 0,
            desc: bytes_of(&msaa_desc),
            initial_state: 0x4,
            clear_value: None,
            gpu_va: 0,
        },
        0,
    )
    .unwrap();
    w.write(&Call::CreateRenderTargetView { resource: MSAA_RT, desc: None, dest: msaa_rtv_cpu }, 0).unwrap();

    // Repeatable frame starts at Reset.
    let lists = [LIST];
    let q = (size / 4) as i32;
    let rect = Rect { left: q, top: q, right: (size as i32) - q, bottom: (size as i32) - q };
    w.write(&Call::Reset { list: LIST, allocator: ALLOC, pso: 0 }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: LIST, rtv: msaa_rtv_cpu, color: [0.1, 0.2, 0.8, 1.0], rects: &[] }, 0).unwrap();
    w.write(&Call::ClearRenderTargetView { list: LIST, rtv: msaa_rtv_cpu, color: [0.9, 0.8, 0.1, 1.0], rects: bytes_of(&rect) }, 0).unwrap();
    let b_res = [transition(MSAA_RT, 0x4, 0x2000, 0), transition(BACKBUF0, 0x0, 0x1000, 0)];
    w.write(&Call::ResourceBarrier { list: LIST, barriers: bytes_of_slice(&b_res) }, 0).unwrap();
    w.write(&Call::ResolveSubresource { list: LIST, dst: BACKBUF0, dst_sub: 0, src: MSAA_RT, src_sub: 0, format: DXGI_FORMAT_R8G8B8A8_UNORM }, 0).unwrap();
    let b_end = [transition(MSAA_RT, 0x2000, 0x4, 0), transition(BACKBUF0, 0x1000, 0x0, 0)];
    w.write(&Call::ResourceBarrier { list: LIST, barriers: bytes_of_slice(&b_end) }, 0).unwrap();
    w.write(&Call::Close { list: LIST }, 0).unwrap();
    w.write(&Call::ExecuteCommandLists { queue: QUEUE, lists: U64s::from_slice(&lists) }, 0).unwrap();
    w.write(&Call::QueueSignal { queue: QUEUE, fence: FENCE, value: 1 }, 0).unwrap();
    w.write(&Call::Present { swapchain: SWAP, sync_interval: 0, flags: 0 }, 0).unwrap();
    w.write(&Call::FrameEnd { frame: 0 }, 0).unwrap();
    w.into_inner()
}

fn copy_loc_subresource(resource: u64, sub: u32) -> [u8; 48] {
    let mut b = [0u8; 48];
    b[0..8].copy_from_slice(&resource.to_le_bytes());
    b[8..12].copy_from_slice(&0u32.to_le_bytes());
    b[16..20].copy_from_slice(&sub.to_le_bytes());
    b
}

fn copy_loc_footprint(resource: u64, fp: PlacedFootprint) -> [u8; 48] {
    let mut b = [0u8; 48];
    b[0..8].copy_from_slice(&resource.to_le_bytes());
    b[8..12].copy_from_slice(&1u32.to_le_bytes());
    b[16..48].copy_from_slice(bytes_of(&fp));
    b
}
