//! GPU tests for D3D12 occlusion, binary-occlusion, and timestamp query heaps on Metal 4:
//!
//! 1. Large 1,048,576-entry `OCCLUSION` heap (8 MiB, 32 x 256 KiB chunks):
//!    - Counting occlusion (`D3D12_QUERY_TYPE_OCCLUSION`) across chunk boundaries
//!      (slots 32,767, 32,768, and 1,048,575) with canary verification proving
//!      slot 32,768 does not wrap around and corrupt slot 0.
//!    - Binary occlusion (`D3D12_QUERY_TYPE_BINARY_OCCLUSION`) returning strictly
//!      `0u64` or `1u64`, including multi-pass `Accumulate`, 0-draw queries,
//!      occluded draws (depth test fail), non-1 nonzero normalization, chunk-boundary
//!      resolve, and resolve -> 0-draw EndQuery WAR synchronization.
//! 2. Large 1,048,576-entry `TIMESTAMP` heap (8 MiB staged backing buffer) and
//!    `COPY_QUEUE_TIMESTAMP` heap:
//!    - Real Metal 4 GPU timestamps written both outside encoders and inside render
//!      passes at low and high indices (0, 1, 4096, 1,048,575), resolved on the
//!      same list and across command lists.
//!    - Scratch-ring wrap-around test writing > 4096 timestamps (5,000) in a single
//!      command list and verifying monotonic non-zero GPU ticks.
//!    - Same-list WAR reuse (`write -> resolve -> write -> resolve` on the same slot).
//!    - Small direct `TIMESTAMP` heap (`count = 128`) keeping the direct `MTL4CounterHeap` path.

use std::sync::Arc;

use d3d12_metal::metal::cmdlist::{CommandAllocator, CommandList, TargetView};
use d3d12_metal::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
use d3d12_metal::metal::query::{heap_type as qh_type, QueryHeap, QueryHeapError, MAX_VIS_CHUNK_ENTRIES};
use d3d12_metal::metal::queue::{ListType, Queue};
use d3d12_metal::metal::resource::Resource;
use d3d12_metal::state::{BlendDesc, DepthStencilState, RasterState};
use d3d12_metal::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};
use dxil_metal::dxbc;
use dxil_metal::root_signature::{RootSignature, RootSignatureDesc, RootSignatureVersion};
use objc2_metal::*;

const VS_FULLSCREEN: &[u8] = include_bytes!("../../dxil-metal/tests/fixtures/entry_v2.vs.dxil");
const D32_FLOAT: u32 = 40;

fn signed(b: &[u8]) -> Vec<u8> {
    let mut v = b.to_vec();
    dxbc::sign_container(&mut v).unwrap();
    v
}

fn heap(ty: u32) -> HeapProperties {
    HeapProperties { Type: ty, ..Default::default() }
}

fn readback_buffer(bytes: u64) -> Resource {
    let d = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: bytes,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let r = Resource::new_committed(&d, &heap(heap_type::READBACK), MTLTextureUsage::empty(), "query rb").unwrap();
    r.register_ptr();
    r
}

fn depth_texture(w: u64, h: u32) -> Resource {
    let d = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: w,
        Height: h,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: D32_FLOAT,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_DEPTH_STENCIL,
        ..Default::default()
    };
    let r = Resource::new_committed(&d, &heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "query dsv").unwrap();
    r.register_ptr();
    r
}

fn read_u64s(r: &Resource, n: usize) -> Vec<u64> {
    let b = r.buffer().unwrap();
    unsafe { std::slice::from_raw_parts(b.contents().as_ptr() as *const u64, n) }.to_vec()
}

fn build_depth_pso(depth_func: MTLCompareFunction, label: &str) -> Option<(Arc<RootSignature>, Arc<GraphicsPipeline>)> {
    if !dxil_metal::converter_available() {
        eprintln!("skipping: metal shader converter not available");
        return None;
    }
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: Vec::new(),
        static_samplers: Vec::new(),
        flags: 0,
    })
    .unwrap();
    let vs = signed(VS_FULLSCREEN);
    let ds = DepthStencilState {
        depth_compare: depth_func,
        depth_write: false,
        ..DepthStencilState::default()
    };
    let desc = GraphicsDesc {
        vs: &vs,
        ps: None,
        blend: BlendDesc::default(),
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
        depth_stencil: ds,
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: 3,
        rtv_formats: Vec::new(),
        dsv_format: D32_FLOAT,
        sample_count: 1,
    };
    let pso = Arc::new(GraphicsPipeline::new(rs.clone(), &desc, label).unwrap());
    Some((rs, pso))
}

#[test]
fn large_1m_occlusion_and_binary_occlusion_queries_on_gpu() {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return;
    }
    let Some((rs, pso_pass)) = build_depth_pso(MTLCompareFunction::Always, "occl_pass") else {
        return;
    };
    let Some((_, pso_fail)) = build_depth_pso(MTLCompareFunction::Never, "occl_fail") else {
        return;
    };

    const COUNT: u32 = 1_048_576;
    let qh = QueryHeap::try_new(qh_type::OCCLUSION, COUNT).expect("1M occlusion query heap");
    assert_eq!(qh.visibility_chunks.len(), 32);

    // Plant a canary at slot 0 (chunk 0) to verify drawing into slot 32,768 (chunk 1)
    // does NOT wrap around modulo 32,768 and clobber slot 0.
    // Also plant an arbitrary non-1 nonzero value at slot 32,765 and 0 at slot 32,766 to
    // verify `resolve_binary_occlusion` normalizes nonzero -> 1 and 0 -> 0 strictly.
    let (chunk0, _) = qh.visibility_slot(0).unwrap();
    unsafe {
        let p = chunk0.contents().as_ptr() as *mut u64;
        *p.add(0) = 0x1234_5678_9abc_def0;
        *p.add(32_765) = 0xdead_beef_cafe_0042;
        *p.add(32_766) = 0;
    }

    let dsv_tex = depth_texture(4, 4);
    let dsv = TargetView::dsv(&dsv_tex, None).unwrap();
    let rb = readback_buffer(64 * 8);
    let scratch_a = readback_buffer(16);
    let scratch_b = readback_buffer(16);
    let vp = MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 };

    let q = Queue::new(ListType::Direct, "gpu_queries occl queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.begin(&alloc).unwrap();
    l.gfx_root.set_root_signature(rs.clone()).unwrap();
    l.gfx.topology = 4;
    l.set_viewports(vec![vp]);
    l.set_render_targets(Vec::new(), Some(dsv));
    unsafe { l.clear_dsv(&dsv, true, 1.0, false, 0, &[]).unwrap() };

    // 1. Counting occlusion across chunk boundary (32,767 in chunk 0; 32,768 in chunk 1; 1,048,575 in chunk 31)
    l.set_graphics_pipeline(pso_pass.clone());
    l.begin_occlusion(&qh, MAX_VIS_CHUNK_ENTRIES - 1, false).unwrap(); // slot 32,767: 1 draw = 16 samples
    l.draw(3, 1, 0, 0).unwrap();
    l.end_occlusion(&qh, MAX_VIS_CHUNK_ENTRIES - 1).unwrap();

    l.begin_occlusion(&qh, MAX_VIS_CHUNK_ENTRIES, false).unwrap(); // slot 32,768: 2 draws = 32 samples
    l.draw(3, 1, 0, 0).unwrap();
    l.draw(3, 1, 0, 0).unwrap();
    l.end_occlusion(&qh, MAX_VIS_CHUNK_ENTRIES).unwrap();

    l.begin_occlusion(&qh, COUNT - 1, false).unwrap(); // slot 1,048,575: 1 draw = 16 samples
    l.draw(3, 1, 0, 0).unwrap();
    l.end_occlusion(&qh, COUNT - 1).unwrap();

    // 2. Binary occlusion queries:
    //    - slot 32,769: passing draw -> must resolve to 1
    //    - slot 32,770: depth-occluded draw -> must resolve to 0
    //    - slot 32,771: multi-pass Accumulate passing draws -> must resolve to 1 (not 2)
    //    - slot 32,772: 0-draw query -> must resolve to 0
    l.begin_occlusion(&qh, 32_769, true).unwrap();
    l.draw(3, 1, 0, 0).unwrap();
    l.end_occlusion(&qh, 32_769).unwrap();

    l.set_graphics_pipeline(pso_fail.clone());
    l.begin_occlusion(&qh, 32_770, true).unwrap();
    l.draw(3, 1, 0, 0).unwrap();
    l.end_occlusion(&qh, 32_770).unwrap();

    l.set_graphics_pipeline(pso_pass.clone());
    l.begin_occlusion(&qh, 32_771, true).unwrap();
    l.draw(3, 1, 0, 0).unwrap();
    // Force a render-pass split via an intervening buffer copy so the second draw
    // opens a fresh `MTLVisibilityResultType::Accumulate` pass on the same query.
    l.copy_buffer(&scratch_b, 0, &scratch_a, 0, 8).unwrap();
    l.draw(3, 1, 0, 0).unwrap();
    l.end_occlusion(&qh, 32_771).unwrap();
    // Close the open render pass before testing a 0-draw query outside any pass.
    l.copy_buffer(&scratch_b, 0, &scratch_a, 0, 8).unwrap();

    l.begin_occlusion(&qh, 32_772, true).unwrap();
    l.end_occlusion(&qh, 32_772).unwrap();

    // Resolve counting across chunk 0 -> chunk 1 boundary (slots 32,767..32,769 -> rb[0..2])
    l.resolve_occlusion(&qh, MAX_VIS_CHUNK_ENTRIES - 1, 2, &rb, 0).unwrap();
    // Resolve counting at last slot 1,048,575 -> rb[2]
    l.resolve_occlusion(&qh, COUNT - 1, 1, &rb, 16).unwrap();
    // Resolve canary slot 0 -> rb[3] (must remain untouched!)
    l.resolve_occlusion(&qh, 0, 1, &rb, 24).unwrap();
    // Resolve binary across chunk 0 -> chunk 1 boundary (slots 32,765..32,773 -> rb[4..12])
    // slots: 32765 (0xdead_beef.. -> 1), 32766 (0 -> 0), 32767 (16 -> 1), 32768 (32 -> 1),
    //        32769 (1 -> 1), 32770 (0 -> 0), 32771 (multi-pass -> 1), 32772 (0-draw -> 0)
    l.resolve_binary_occlusion(&qh, 32_765, 8, &rb, 32).unwrap();

    // WAR hazard test: immediately overwrite slot 32,769 with a 0-draw binary query in the same
    // command list and resolve it into rb[12]; rb[8] (earlier resolve of 32,769) must stay 1
    // while rb[12] must be 0.
    l.begin_occlusion(&qh, 32_769, true).unwrap();
    l.end_occlusion(&qh, 32_769).unwrap();
    l.resolve_binary_occlusion(&qh, 32_769, 1, &rb, 12 * 8).unwrap();

    l.close().unwrap();
    q.submit(&[&l.cb]);
    q.wait_idle();
    alloc.reset();

    let got = read_u64s(&rb, 13);
    assert_eq!(got[0], 16, "slot 32,767 counting must be 16, got {got:?}");
    assert_eq!(got[1], 32, "slot 32,768 counting (chunk 1) must be 32, got {got:?}");
    assert_eq!(got[2], 16, "slot 1,048,575 counting (chunk 31) must be 16, got {got:?}");
    assert_eq!(
        got[3], 0x1234_5678_9abc_def0,
        "slot 0 canary must not be clobbered by chunk 1 slot 32,768, got {:#x}",
        got[3]
    );
    assert_eq!(
        &got[4..12],
        &[1, 0, 1, 1, 1, 0, 1, 0],
        "binary occlusion resolves across chunk boundary must return strictly 0 or 1, got {:?}",
        &got[4..12]
    );
    assert_eq!(
        got[12], 0,
        "0-draw binary occlusion overwrite after resolve must read 0 while earlier resolve stays 1, got {got:?}"
    );
}

#[test]
fn large_1m_and_direct_timestamp_query_heaps_on_gpu() {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return;
    }

    const COUNT: u32 = 1_048_576;
    let ts_large = QueryHeap::try_new(qh_type::TIMESTAMP, COUNT).expect("1M timestamp query heap");
    assert!(ts_large.is_staged_timestamp());
    let ts_direct = QueryHeap::try_new(qh_type::TIMESTAMP, 128).expect("direct 128 timestamp heap");
    assert!(!ts_direct.is_staged_timestamp());
    let ts_copy = QueryHeap::try_new(qh_type::COPY_QUEUE_TIMESTAMP, 65_536)
        .expect("64K copy-queue timestamp heap");
    assert!(ts_copy.is_staged_timestamp());

    let dsv_tex = depth_texture(4, 4);
    let dsv = TargetView::dsv(&dsv_tex, None).unwrap();
    let rb = readback_buffer(64 * 8);
    let rb_wrap = readback_buffer(5_000 * 8);

    let q = Queue::new(ListType::Direct, "gpu_queries ts queue").unwrap();
    let alloc_a = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc_b = CommandAllocator::new(ListType::Direct).unwrap();

    // List A: write timestamps to direct heap, large 1M heap (outside and inside render pass,
    // including high indices 4096 and 1,048,575), same-list WAR reuse on slot 77, and leave
    // slot 500,000 to be resolved by List B.
    let mut la = CommandList::new(ListType::Direct).unwrap();
    la.begin(&alloc_a).unwrap();

    la.write_timestamp(&ts_direct, 0).unwrap();
    la.write_timestamp(&ts_large, 0).unwrap();
    unsafe { la.clear_dsv(&dsv, true, 1.0, false, 0, &[]).unwrap() };
    la.write_timestamp(&ts_large, 1).unwrap();
    la.write_timestamp(&ts_large, 4096).unwrap();
    la.write_timestamp(&ts_large, COUNT - 1).unwrap();
    la.write_timestamp(&ts_direct, 1).unwrap();

    la.resolve_timestamps(&ts_direct, 0, 2, &rb, 0).unwrap(); // rb[0..2]
    la.resolve_timestamps(&ts_large, 0, 2, &rb, 16).unwrap(); // rb[2..4]
    la.resolve_timestamps(&ts_large, 4096, 1, &rb, 32).unwrap(); // rb[4]
    la.resolve_timestamps(&ts_large, COUNT - 1, 1, &rb, 40).unwrap(); // rb[5]

    // Same-list WAR: write slot 77 -> resolve to rb[6] -> overwrite slot 77 -> resolve to rb[7]
    la.write_timestamp(&ts_large, 77).unwrap();
    la.resolve_timestamps(&ts_large, 77, 1, &rb, 48).unwrap();
    la.write_timestamp(&ts_large, 77).unwrap();
    la.resolve_timestamps(&ts_large, 77, 1, &rb, 56).unwrap();

    // Cross-list slot 500,000 (flushed by la.close())
    la.write_timestamp(&ts_large, 500_000).unwrap();
    // Copy-queue timestamp heap slot 65,535
    la.write_timestamp(&ts_copy, 65_535).unwrap();
    la.resolve_timestamps(&ts_copy, 65_535, 1, &rb, 72).unwrap(); // rb[9]
    la.close().unwrap();

    // List B: resolve slot 500,000 written by List A, plus stress-test >4096 timestamps (5,000)
    // in a single command list to exercise scratch-ring flushing and wrap-around.
    let mut lb = CommandList::new(ListType::Direct).unwrap();
    lb.begin(&alloc_b).unwrap();
    lb.resolve_timestamps(&ts_large, 500_000, 1, &rb, 64).unwrap(); // rb[8]
    for i in 0..5_000u32 {
        lb.write_timestamp(&ts_large, 10_000 + i).unwrap();
    }
    lb.resolve_timestamps(&ts_large, 10_000, 5_000, &rb_wrap, 0).unwrap();
    lb.close().unwrap();

    q.submit(&[&la.cb, &lb.cb]);
    q.wait_idle();
    alloc_a.reset();
    alloc_b.reset();

    let got = read_u64s(&rb, 10);
    assert!(got[0] > 0 && got[1] >= got[0], "direct timestamp heap must be monotonic non-zero: {:?}", &got[0..2]);
    assert!(
        got[2] > 0 && got[3] >= got[2] && got[4] >= got[3] && got[5] >= got[4],
        "1M staged timestamp heap slots 0, 1, 4096, 1048575 must be monotonic non-zero: {:?}",
        &got[2..6]
    );
    assert!(got[6] > 0 && got[7] >= got[6], "same-list WAR timestamp reuse must be monotonic non-zero: {:?}", &got[6..8]);
    assert!(got[8] >= got[5], "cross-list resolved timestamp slot 500,000 must be >= earlier slot: {:?}", &got[5..9]);
    assert!(got[9] > 0, "COPY_QUEUE_TIMESTAMP 64K slot must be non-zero: {}", got[9]);

    let wrap = read_u64s(&rb_wrap, 5_000);
    assert!(wrap[0] > 0, "first wrapped timestamp must be non-zero");
    for i in 1..wrap.len() {
        assert!(
            wrap[i] >= wrap[i - 1],
            "5,000 staged timestamps must be monotonically non-decreasing at index {i}: {} < {}",
            wrap[i],
            wrap[i - 1]
        );
    }
}

#[test]
fn query_heap_error_classification() {
    assert!(matches!(
        QueryHeap::try_new(qh_type::OCCLUSION, 0),
        Err(QueryHeapError::InvalidArg(_))
    ));
    assert!(matches!(
        QueryHeap::try_new(qh_type::TIMESTAMP, 0),
        Err(QueryHeapError::InvalidArg(_))
    ));
    assert!(matches!(
        QueryHeap::try_new(123, 16),
        Err(QueryHeapError::InvalidArg(_))
    ));
}
