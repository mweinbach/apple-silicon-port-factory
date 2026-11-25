use super::*;

#[test]
fn g39_fix1_opening_pre_raster_reads_folds_split_pend_and_mid_qb_stagesync_elides_sync_pre_raster_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{
        ComputePipeline, GraphicsDesc, GraphicsPipeline, READS_BUF, READS_TEX,
    };
    use crate::metal::views;
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignature,
        RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none()
        || !dxil_metal::converter_available()
        || !dxil_metal::dxilconv::available()
    {
        return;
    }
    const RGBA32_FLOAT: u32 = 2;

    let ps_cb0_dxbc = cb0_color_ps_dxbc();
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let gfx_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Constants {
                shader_register: 0,
                register_space: 0,
                num_32bit_values: 4,
            },
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let gdesc = GraphicsDesc {
        vs: &vs_dxil,
        ps: Some(&ps_cb0_dxbc),
        blend: color_blend,
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
        depth_stencil: DepthStencilState::default(),
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: 3,
        rtv_formats: vec![RGBA8_UNORM],
        dsv_format: 0,
        sample_count: 1,
    };
    let mut gp0 = GraphicsPipeline::new_with_opts(gfx_rs.clone(), &gdesc, "g39_f1_no_srv", true).unwrap();
    gp0.writes_uav = false;
    gp0.sm66_writes_uav = false;
    gp0.pre_raster_reads = 0;
    gp0.sm66_pre_raster_reads = 0;
    let pso_no_srv = Arc::new(gp0);

    let mut gp_buf = GraphicsPipeline::new_with_opts(gfx_rs.clone(), &gdesc, "g39_f1_buf_vs", true).unwrap();
    gp_buf.writes_uav = false;
    gp_buf.sm66_writes_uav = false;
    gp_buf.pre_raster_reads = READS_BUF;
    gp_buf.sm66_pre_raster_reads = READS_BUF;
    let pso_buf_vs = Arc::new(gp_buf);

    let mut gp_tex = GraphicsPipeline::new_with_opts(gfx_rs.clone(), &gdesc, "g39_f1_tex_vs", true).unwrap();
    gp_tex.writes_uav = false;
    gp_tex.sm66_writes_uav = false;
    gp_tex.pre_raster_reads = READS_TEX;
    gp_tex.sm66_pre_raster_reads = READS_TEX;
    let pso_tex_vs = Arc::new(gp_tex);

    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g39_f1_cs").unwrap());

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g39 fix1 heap").unwrap();
    let srv_tex = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET);
    let rt_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let uav_buf = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_rtb = test_buffer(1024, heap_type::READBACK, 0);
    for r in [&srv_tex, &rt_a, &rt_b, &uav_buf] {
        r.register_ptr();
    }
    unsafe {
        let e0 = views::srv_entry(Some(&srv_tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }

    let q = Queue::new(ListType::Direct, "g39 fix1 queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_render_split_stage_sync = true;
    l.opts.opt_mid_qb_stagesync = true;
    l.opts.opt_stagesync_barrier_v2 = false;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    // 1) Compute dispatch writes `uav_buf` (`stage_gen[STAGE_DISPATCH] == 1`).
    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_buf.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso);
    l.dispatch(1, 1, 1).unwrap();
    l.end_encoder();

    // 2) `uav_buf` transitions `UAV -> NON_PIXEL_SRV | PIXEL_SRV` (`soft[1] = PRE_RASTER`, `frag_after = shader`).
    //    Opening Pass 1 with `pso_buf_vs` (`READS_BUF`) MUST fold `soft[1]` directly into `vtx_after` inside
    //    `begin_barrier`, emitting ONE combined `PRE_RASTER | Fragment` barrier with `dv_defer == 0` and `dv_hit == 0`!
    let uav_id = &uav_buf as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(uav_id, STATE_UAV, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)], true, false);
    let dv_def0 = stats::c::dv_defer.load(Relaxed);
    let dv_hit0 = stats::c::dv_hit.load(Relaxed);
    let qb_frag0 = stats::c::q_barrier_frag.load(Relaxed);

    l.gfx_root.set_root_signature(gfx_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[0x3f80_0000, 0, 0, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![TargetView::rtv(&rt_a, None).unwrap()], None);
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_buf_vs.clone());
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::dv_defer.load(Relaxed) - dv_def0, 0, "opening READS_BUF draw must fold soft[1] instead of deferring");
    assert_eq!(stats::c::dv_hit.load(Relaxed) - dv_hit0, 0, "opening READS_BUF draw must emit 0 separate dv_hit barriers");
    assert_eq!(stats::c::q_barrier_frag.load(Relaxed) - qb_frag0, 1, "expected 1 combined opening barrier");
    assert_eq!(l.bar.vtx_defer, NO_VTX);

    // 3) Inside Pass 1 (`opt_mid_qb_stagesync == true`), `Dispatch` is ALREADY satisfied for `PRE_RASTER`
    //    (`enc_cur_sync[STAGE_VERTEX].gen[STAGE_DISPATCH] == stage_gen[STAGE_DISPATCH]`).
    //    If a deferred `(Dispatch, PRE_RASTER)` entry lands in `vtx_defer[1]`, `sync_pre_raster` MUST elide it
    //    via `unsatisfied_mid_qb_after` (`stagesync_skip`++, `dv_hit == 0`)!
    l.bar.vtx_defer[1] = Some((MTLStages::Dispatch, PRE_RASTER));
    let skip0 = stats::c::stagesync_skip.load(Relaxed);
    let dv_hit1 = stats::c::dv_hit.load(Relaxed);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::dv_hit.load(Relaxed) - dv_hit1, 0, "sync_pre_raster must elide already-synced vtx_defer under AW2_MID_QB_STAGESYNC=1");
    assert_eq!(stats::c::stagesync_skip.load(Relaxed) - skip0, 1);
    l.end_encoder();

    // 4) Conversely, when `rt_a` transitions `RTV -> NON_PIXEL_SRV | PIXEL_SRV` (`Fragment -> Fragment` + soft `PRE_RASTER`)
    //    and Pass 2 opens with `pso_no_srv` (`pre_raster_reads == 0`), `SplitPend` MUST NOT fold `Fragment` into `vtx_after`
    //    (`qb_frag_to_vtx == 0`, `dv_defer == 1`), and a subsequent `pso_tex_vs` draw in Pass 2 fires `dv_hit == 1`!
    let rt_a_id = &rt_a as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)], true, false);
    let f2v_0 = stats::c::qb_frag_to_vtx.load(Relaxed);
    let dv_def1 = stats::c::dv_defer.load(Relaxed);
    l.gfx_root.set_constants(0, 0, &[0, 0, 0x3f80_0000, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![TargetView::rtv(&rt_b, None).unwrap()], None);
    l.set_graphics_pipeline(pso_no_srv);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::qb_frag_to_vtx.load(Relaxed) - f2v_0, 0, "opening draw with pre_raster_reads == 0 must keep Fragment out of PRE_RASTER");
    assert_eq!(stats::c::dv_defer.load(Relaxed) - dv_def1, 1, "soft[0] must defer when opening draw has pre_raster_reads == 0");

    let dv_hit2 = stats::c::dv_hit.load(Relaxed);
    l.set_graphics_pipeline(pso_tex_vs);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::dv_hit.load(Relaxed) - dv_hit2, 1, "subsequent READS_TEX draw must fire deferred Fragment -> PRE_RASTER barrier");
    l.end_encoder();

    let rt_b_id = &rt_b as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(rt_b_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_rtb, fp), [0, 0, 0], CopyLoc::Subresource(&rt_b, 0), None).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb_rtb, 1)[0], 0xffff_0000, "rt_b must read back blue");
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g39 fix1 must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g39_fix2_compute_clear_batch_split_pend_isolates_disp_after_from_dsv_clear_fragment_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputePipeline, READS_BUF};
    use crate::metal::views;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    const RGBA32_FLOAT: u32 = 2;

    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let mut cp = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g39_f2_cs").unwrap();
    cp.access = ComputeAccess {
        uav_slots: cp.access.uav_slots,
        read_slots: None,
        bounded_read_slots: Vec::new(),
        unbounded_srv_classes: READS_BUF,
        heap_indexed_or_unknown: false,
        touches_textures: false,
        uav_classes: READS_BUF,
        read_classes: READS_BUF,
        ..ComputeAccess::opaque()
    };
    let cs_pso = Arc::new(cp);

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g39 fix2 heap").unwrap();
    let srv_tex = test_texture(4, 1, RGBA32_FLOAT, 0);
    let ds1 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let rt_simul = test_texture(
        4,
        4,
        RGBA8_UNORM,
        resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_SIMULTANEOUS_ACCESS,
    );
    let rt1 = test_texture(
        4,
        4,
        RGBA8_UNORM,
        resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS,
    );
    let rt2 = test_texture(
        4,
        4,
        RGBA8_UNORM,
        resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS,
    );
    let uav_buf = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb1 = test_buffer(1024, heap_type::READBACK, 0);
    let rb2 = test_buffer(1024, heap_type::READBACK, 0);
    for r in [&srv_tex, &ds1, &rt_simul, &rt1, &rt2, &uav_buf] {
        r.register_ptr();
    }
    unsafe {
        let e0 = views::srv_entry(Some(&srv_tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }

    let q = Queue::new(ListType::Direct, "g39 fix2 queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_render_split_stage_sync = true;
    l.opts.opt_clear_cs_exact_sync = true;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    // L2#0: Compute dispatch anchors `pre_list_all` on `STAGE_DISPATCH` and closes via `Barrier::Uav`.
    let uav_id = &uav_buf as *const Resource as usize;
    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_buf.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso.clone());
    l.dispatch(1, 1, 1).unwrap();
    l.end_encoder();

    // L2#1..L2#2: Hardware DSV + non-UAV SIMUL RTV `CLEAR` passes flush and leave `Fragment`
    // in `split_pend.frag_after` (`self.pend_after == Dispatch | Fragment`, `split_pend.disp_after == Dispatch`).
    let dsv1 = TargetView::dsv(&ds1, None).unwrap();
    let rtv_simul = TargetView::rtv(&rt_simul, None).unwrap();
    unsafe {
        l.clear_dsv(&dsv1, true, 0.25, false, 0, &[]).unwrap();
        l.clear_rtv(&rtv_simul, [0.5, 0.5, 0.5, 1.0], &[]).unwrap();
    }
    let ds1_id = &ds1 as *const Resource as usize;
    let rt_simul_id = &rt_simul as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Uav(uav_id),
            Barrier::Transition(ds1_id, STATE_DEPTH_WRITE, STATE_PIXEL_SRV),
            Barrier::Transition(rt_simul_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
        ],
        true,
        false,
    );
    assert!(l.bar.split_pend.frag_after.contains(MTLStages::Fragment));
    assert!(!l.bar.split_pend.disp_after.contains(MTLStages::Fragment));
    assert_eq!(l.bar.split_pend.disp_after, MTLStages::Dispatch);

    // L2#4: Open a synced compute encoder (`cs_defer_gfx` preserves `split_pend.frag_after == Fragment`
    // and `self.pend_after == Fragment`!), then flush 2 RTV clears via `compute_clear_batch` (`[aw2:clear_cs]`).
    l.set_compute_pipeline(cs_pso);
    l.dispatch(1, 1, 1).unwrap();
    assert!(matches!(l.enc, Encoder::Compute(_)));
    assert!(l.bar.pend_after.contains(MTLStages::Fragment), "legacy pend_after mirrors split_pend.frag_after");

    let rtv1 = TargetView::rtv(&rt1, None).unwrap();
    let rtv2 = TargetView::rtv(&rt2, None).unwrap();
    unsafe {
        l.clear_rtv(&rtv1, [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
        l.clear_rtv(&rtv2, [1.0, 0.0, 1.0, 1.0], &[]).unwrap();
    }
    let exact0 = stats::c::clear_cs_exact_sync.load(Relaxed);
    let mid_qb0 = stats::c::cs_mid_qb.load(Relaxed);
    let rt1_id = &rt1 as *const Resource as usize;
    let rt2_id = &rt2 as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(rt1_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
            Barrier::Transition(rt2_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
        ],
        true,
        false,
    );
    assert_eq!(stats::c::clear_cs_exact_sync.load(Relaxed) - exact0, 1, "expected clear_cs_exact_sync in L2#4");
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - mid_qb0, 0, "Fix #2 must NOT leak Fragment into split_pend.disp_after / cs_mid_qb");
    assert!(l.bar.split_pend.frag_after.contains(MTLStages::Fragment), "graphics frag_after must stay intact for later draw passes");
    l.end_encoder();

    l.barrier_list_with(
        &[
            Barrier::Transition(rt1_id, STATE_PIXEL_SRV, STATE_COPY_SOURCE),
            Barrier::Transition(rt2_id, STATE_PIXEL_SRV, STATE_COPY_SOURCE),
        ],
        true,
        false,
    );
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb1, fp), [0, 0, 0], CopyLoc::Subresource(&rt1, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb2, fp), [0, 0, 0], CopyLoc::Subresource(&rt2, 0), None).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb1, 1)[0], 0xff00_ff00, "rt1 compute clear must read back green");
    assert_eq!(buf_words(&rb2, 1)[0], 0xffff_00ff, "rt2 compute clear must read back magenta");
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g39 fix2 must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g39_fix3_per_resource_uav_passthrough_isolates_unrelated_uavs_and_clear_targets_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{
        ComputeAccess, ComputePipeline, GraphicsDesc, GraphicsPipeline, READS_BUF,
    };
    use crate::metal::views;
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignature,
        RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none()
        || !dxil_metal::converter_available()
        || !dxil_metal::dxilconv::available()
    {
        return;
    }
    const RGBA32_FLOAT: u32 = 2;

    let ps_cb0_dxbc = cb0_color_ps_dxbc();
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let gfx_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Constants {
                shader_register: 0,
                register_space: 0,
                num_32bit_values: 4,
            },
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let gdesc = GraphicsDesc {
        vs: &vs_dxil,
        ps: Some(&ps_cb0_dxbc),
        blend: color_blend,
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
        depth_stencil: DepthStencilState::default(),
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: 3,
        rtv_formats: vec![RGBA8_UNORM],
        dsv_format: 0,
        sample_count: 1,
    };
    let mut gp = GraphicsPipeline::new_with_opts(gfx_rs.clone(), &gdesc, "g39_f3_gfx", true).unwrap();
    gp.writes_uav = false;
    let pso_no_uav = Arc::new(gp);

    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let mut cp = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g39_f3_cs").unwrap();
    cp.access = ComputeAccess {
        uav_slots: cp.access.uav_slots,
        read_slots: None,
        bounded_read_slots: Vec::new(),
        unbounded_srv_classes: READS_BUF,
        heap_indexed_or_unknown: false,
        touches_textures: false,
        uav_classes: READS_BUF,
        read_classes: READS_BUF,
        ..ComputeAccess::opaque()
    };
    let cs_pso = Arc::new(cp);

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g39 fix3 heap").unwrap();
    let srv_tex = test_texture(4, 1, RGBA32_FLOAT, 0);
    let rt_clear = test_texture(
        4,
        4,
        RGBA8_UNORM,
        resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS,
    );
    let rt_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let uav_buf1 = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_buf2 = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_clear = test_buffer(1024, heap_type::READBACK, 0);
    let rb_rta = test_buffer(1024, heap_type::READBACK, 0);
    for r in [&srv_tex, &rt_clear, &rt_a, &rt_b, &uav_buf1, &uav_buf2] {
        r.register_ptr();
    }
    unsafe {
        let e0 = views::srv_entry(Some(&srv_tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rt_clear_id = &rt_clear as *const Resource as usize;
    let rt_a_id = &rt_a as *const Resource as usize;
    let uav1_id = &uav_buf1 as *const Resource as usize;
    let uav2_id = &uav_buf2 as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g39 fix3 queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_render_split_stage_sync = true;
    l.opts.opt_mid_qb_stagesync = true;
    l.opts.opt_split_res_soft_sync = true;
    l.opts.opt_cs_uav_producer_exact = true;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    // Part A (`L4#1..4 -> L4#5` standalone `CLEAR` target `RTV -> UAV` + `Barrier::Uav(0)` `cs_uav_mid_skip`):
    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_buf1.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso.clone());
    l.dispatch(1, 1, 1).unwrap();
    l.end_encoder();

    // Flush `rt_clear` via a standalone hardware `CLEAR` pass (`clear_rp_res_ids`).
    let rtv_clear = TargetView::rtv(&rt_clear, None).unwrap();
    unsafe { l.encode_rtv_clear_pass(&rtv_clear, [0.0, 1.0, 1.0, 1.0]).unwrap() };
    l.barrier_list_with(
        &[
            Barrier::Uav(uav1_id),
            Barrier::Transition(rt_clear_id, STATE_RENDER_TARGET, STATE_UAV),
        ],
        true,
        false,
    );
    assert!(l.bar.clear_rp_res_ids.contains(&rt_clear_id));
    assert!(l.bar.clear_tex_cs.contains(&rt_clear_id));
    assert!(
        l.bar.uav_passthrough.is_empty(),
        "standalone CLEAR target deferred to clear_tex_cs must NOT inject Fragment into uav_passthrough"
    );

    // Open `L4#5` buffer-only compute encoder, run dispatch, and emit `Barrier::Uav(0)` (`cs_uav_mid_skip`):
    l.compute_root.set_descriptor(1, uav_buf2.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso);
    l.dispatch(1, 1, 1).unwrap();
    let mid_skip0 = stats::c::cs_uav_mid_skip.load(Relaxed);
    l.barrier_list_with(&[Barrier::Uav(0)], true, true);
    assert_eq!(stats::c::cs_uav_mid_skip.load(Relaxed) - mid_skip0, 1);
    assert_eq!(
        l.bar.cs_uav_after,
        MTLStages::Dispatch,
        "cs_uav_mid_skip on Barrier::Uav(0) must NOT inherit Fragment from rt_clear"
    );
    l.end_encoder();
    assert_eq!(
        l.bar.split_pend.disp_after,
        MTLStages::Dispatch,
        "closing L4#5 must NOT leak Fragment into split_pend.disp_after"
    );

    // Yet a 0-dispatch `UAV -> COPY_SOURCE` on `rt_clear` STILL preserves `Fragment` via `clear_tex_cs`!
    l.barrier_list_with(&[Barrier::Transition(rt_clear_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    assert!(
        l.bar.split_pend.blit_after.contains(MTLStages::Fragment),
        "0-dispatch UAV -> COPY_SOURCE on clear_tex_cs target must still wait on Fragment"
    );
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_clear, fp), [0, 0, 0], CopyLoc::Subresource(&rt_clear, 0), None).unwrap();
    l.end_encoder();

    // Part B (Non-clear `rt_a` rendered in Draw Pass 1 `RTV -> UAV` in the SAME window as `uav_buf1` / `uav_buf2` / `Barrier::Uav(0)`):
    l.gfx_root.set_root_signature(gfx_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[0x3f80_0000, 0, 0x3f80_0000, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![TargetView::rtv(&rt_a, None).unwrap()], None);
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_no_uav.clone());
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();

    l.barrier_list_with(&[Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_UAV)], true, false);
    assert_eq!(l.uav_passthrough_for_transition(rt_a_id), MTLStages::Fragment);
    assert_eq!(l.uav_passthrough_for_transition(uav1_id), MTLStages::empty());
    assert_eq!(l.uav_passthrough_for_uav(uav2_id), MTLStages::empty());
    assert_eq!(l.uav_passthrough_for_uav(0), MTLStages::empty());
    // Clear pending barrier state from `rt_a`'s own `RTV -> UAV` so we measure ONLY what `uav_buf1`/`uav_buf2`/`Uav(0)` emit:
    l.clear_all_pend();

    l.barrier_list_with(
        &[
            Barrier::Transition(uav1_id, STATE_UAV, STATE_VB_CB | 0x200),
            Barrier::Uav(uav2_id),
            Barrier::Uav(0),
        ],
        true,
        true,
    );
    assert_eq!(
        l.bar.split_pend.vtx_after | l.bar.split_pend.soft_after[1],
        MTLStages::Dispatch,
        "unrelated Transition(uav_buf1, UAV, VB_CB|INDIRECT) must stay pure Dispatch while rt_a is in uav_passthrough"
    );
    assert_eq!(
        l.bar.split_pend.disp_after,
        MTLStages::Dispatch,
        "unrelated Barrier::Uav(uav_buf2) / Barrier::Uav(0) must stay pure Dispatch while rt_a is in uav_passthrough"
    );

    let f2v_0 = stats::c::qb_frag_to_vtx.load(Relaxed);
    l.set_render_targets(vec![TargetView::rtv(&rt_b, None).unwrap()], None);
    l.set_graphics_pipeline(pso_no_uav);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::qb_frag_to_vtx.load(Relaxed) - f2v_0, 0, "Fix #3 must emit 0 false qb_frag_to_vtx");
    l.end_encoder();

    // Meanwhile `rt_a` itself STILL preserves `Fragment` on 0-dispatch `UAV -> COPY_SOURCE` and consumes its entry!
    l.barrier_list_with(&[Barrier::Transition(rt_a_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    assert!(l.bar.split_pend.blit_after.contains(MTLStages::Fragment));
    assert!(l.bar.uav_passthrough.is_empty());
    l.copy_texture_region(CopyLoc::Footprint(&rb_rta, fp), [0, 0, 0], CopyLoc::Subresource(&rt_a, 0), None).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb_clear, 1)[0], 0xffff_ff00, "rt_clear must read back cyan");
    assert_eq!(buf_words(&rb_rta, 1)[0], 0xffff_00ff, "rt_a must read back magenta");
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g39 fix3 must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g40_split_compute_exact_and_render_dom_exact_fold_dominated_stages_and_isolate_substages_on_gpu() {
    use std::sync::atomic::Ordering::Relaxed;
    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "g40 split_cs & render_dom queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_split_compute_exact = true;
    l.opts.opt_render_dom_exact = true;

    let ua = resource_flags::ALLOW_UNORDERED_ACCESS;
    let up = test_buffer(64, heap_type::UPLOAD, 0);
    let blit_dst = test_buffer(64, heap_type::DEFAULT, 0);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rt_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | ua);
    let rt_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let uav_buf = test_buffer(64, heap_type::DEFAULT, ua);
    for r in [&up, &blit_dst, &rb, &rt_a, &rt_b, &uav_buf] {
        r.register_ptr();
    }
    unsafe {
        *(up.buffer().unwrap().contents().as_ptr() as *mut u32) = 0x1234_5678;
    }

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    // 1) Encoder #0 (Compute): Dispatch clear on `uav_buf` (records `stage_gen[Dispatch] == 1` with `pre_list_all`).
    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_desc[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_desc[16..20].copy_from_slice(&16u32.to_le_bytes());
    l.clear_uav(&uav_buf, Some(&uav_desc), ClearValue::Uint([0xaaaa_bbbb, 0, 0, 0]), &[]).unwrap();
    l.end_encoder();
    assert_eq!(l.bar.stage_gen[STAGE_DISPATCH], 1);

    // 2) Transition `uav_buf: UAV -> PIXEL_SRV` (`Dispatch -> Fragment`), then open Render Pass #1 on `rt_a`
    //    which waits on `Dispatch -> Fragment`, establishing `stage_sync[Fragment].gen[Dispatch] == 1`.
    let uav_id = &uav_buf as *const Resource as usize;
    let rt_a_id = &rt_a as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(uav_id, STATE_UAV, STATE_PIXEL_SRV)], true, false);
    let rtv_a = TargetView::rtv(&rt_a, None).unwrap();
    unsafe { l.clear_rtv(&rtv_a, [1.0, 0.0, 1.0, 1.0], &[]).unwrap() };
    l.set_render_targets(vec![rtv_a], None);
    let _ = unsafe { l.draw_encoder().unwrap() };
    l.end_encoder();
    assert_eq!(l.bar.stage_gen[STAGE_FRAGMENT], 1);
    assert_eq!(l.bar.stage_sync[STAGE_FRAGMENT].gen[STAGE_DISPATCH], 1);
    l.clear_all_pend();

    // 3) Queue BOTH:
    //    - `rt_a: RENDER_TARGET -> NON_PIXEL_SRV` (`Fragment -> Dispatch` + soft `PRE_RASTER`, `disp_after = Fragment`, `blit_after = empty`)
    //    - `uav_buf: PIXEL_SRV -> UNORDERED_ACCESS` (`Fragment` already; also test `Dispatch | Fragment` dominance!)
    l.add_pend(MTLStages::Dispatch, MTLStages::Dispatch | MTLStages::Fragment);
    l.barrier_list_with(&[Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)], true, false);
    assert_eq!(l.bar.split_pend.disp_after, MTLStages::Dispatch | MTLStages::Fragment);
    assert!(l.bar.split_pend.blit_after.is_empty());

    // 4) `g40_A` Sub-stage isolation: run a `Blit` copy (`up -> blit_dst`).
    //    Because `blit_after` is empty, `AW2_SPLIT_COMPUTE_EXACT=1` opens Compute for `Blit` with
    //    `enc_sync_after == empty`, keeping `disp_after = Dispatch | Fragment` pending (`split_cs_substage`++)!
    let sub0 = stats::c::split_cs_substage.load(Relaxed);
    l.copy_buffer(&blit_dst, 0, &up, 0, 16).unwrap();
    assert!(stats::c::split_cs_substage.load(Relaxed) > sub0, "Blit must not drain disp_after");
    assert_eq!(l.bar.enc_sync_after, MTLStages::empty());
    assert_eq!(l.bar.split_pend.disp_after, MTLStages::Dispatch | MTLStages::Fragment);

    // 5) Now run a `Dispatch` utility (`clear_uav`) on the SAME open Compute encoder:
    //    `compute_mid_queue_sync(Dispatch)` drains `disp_after = Dispatch | Fragment` and folds
    //    dominated `Dispatch` (`stage_sync[Fragment].gen[Dispatch] == 1 >= disp_req[Dispatch] == 1`),
    //    emitting ONLY `Fragment -> Dispatch` (`split_cs_dom_fold`++), then `clear_uav`'s own
    //    `produced(Dispatch, All)` stamps only `Dispatch`!
    let pre_frag_after = l.bar.split_pend.frag_after;
    let pre_frag_req = l.bar.split_pend.frag_req;
    let cfold0 = stats::c::split_cs_dom_fold.load(Relaxed);
    l.clear_uav(&uav_buf, Some(&uav_desc), ClearValue::Uint([0x5555_6666, 0, 0, 0]), &[]).unwrap();
    assert!(stats::c::split_cs_dom_fold.load(Relaxed) > cfold0, "Dispatch dominated by Fragment must fold in compute_mid_queue_sync");
    assert_eq!(l.bar.split_pend.disp_after, MTLStages::Dispatch, "Fragment must be drained from disp_after");
    l.end_encoder();
    assert_eq!(l.bar.stage_gen[STAGE_DISPATCH], 2, "new Dispatch bumped stage_gen[Dispatch] to 2");

    // 6) `g40_C` Generation-exact Render dominance fold:
    //    Restore the pre-`clear_uav` `frag_after` / `frag_req` (since a normal D3D12 `Dispatch`
    //    does not call `produced(Dispatch, All)`). Even though `stage_gen[STAGE_DISPATCH]` is
    //    now `2` (> `stage_sync[Fragment].gen[Dispatch] == 1`), `frag_req[Dispatch]` was stamped
    //    at `1` (BEFORE Dispatch #2 ran)! Opening Render Pass #2 on `rt_b` MUST still fold
    //    `Dispatch` under `Fragment` (`render_dom_fold`++)!
    l.bar.split_pend.frag_after = pre_frag_after;
    l.bar.split_pend.frag_req = pre_frag_req;
    l.sync_legacy_from_split();
    assert_eq!(l.bar.split_pend.frag_after, MTLStages::Dispatch | MTLStages::Fragment);
    assert_eq!(l.bar.split_pend.frag_req[STAGE_DISPATCH], 1);
    let rfold0 = stats::c::render_dom_fold.load(Relaxed);
    let rtv_b = TargetView::rtv(&rt_b, None).unwrap();
    unsafe { l.clear_rtv(&rtv_b, [0.0, 1.0, 1.0, 1.0], &[]).unwrap() };
    l.set_render_targets(vec![rtv_b], None);
    let _ = unsafe { l.draw_encoder().unwrap() };
    assert!(stats::c::render_dom_fold.load(Relaxed) > rfold0, "Render pass must fold gen-1 Dispatch under Fragment despite gen-2 Dispatch");
    l.end_encoder();

    // Read back `rt_a` to verify GPU execution and timestamp monotonicity.
    l.barrier_list_with(&[Barrier::Transition(rt_a_id, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV, STATE_COPY_SOURCE)], true, false);
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt_a, 0), None).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb, 1)[0], 0xffff_00ff);
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g40 split_cs & render_dom must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g40_msc_stage_usage_elides_cbv_only_uav_barriers_while_preserving_transition_hazards_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputePipeline, READS_BUF};
    use crate::metal::views;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    const RGBA32_FLOAT: u32 = 2;
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();

    // Simulate a compute shader refined by `AW2_MSC_STAGE_USAGE=1` that reads ONLY a CBV buffer
    // (`read_classes = READS_BUF`) and touches NO SRV/UAV buffers (`uav_srv_classes = 0`, `uav_classes = 0`),
    // plus a writer shader that writes a UAV buffer (`uav_srv_classes = READS_BUF`, `uav_classes = READS_BUF`).
    let mut cp_cbv = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g40_cbv_only").unwrap();
    cp_cbv.access = ComputeAccess {
        uav_slots: None,
        read_slots: None,
        bounded_read_slots: Vec::new(),
        unbounded_srv_classes: 0,
        uav_classes: 0,
        read_classes: READS_BUF,
        uav_srv_classes: 0,
        heap_indexed_or_unknown: true,
        touches_textures: false,
        msc_refined: true,
        msc_exact_classes: READS_BUF,
    };
    let pso_cbv_only = Arc::new(cp_cbv);

    let mut cp_writer = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g40_uav_writer").unwrap();
    cp_writer.access = ComputeAccess {
        uav_slots: None,
        read_slots: None,
        bounded_read_slots: Vec::new(),
        unbounded_srv_classes: READS_BUF,
        uav_classes: READS_BUF,
        read_classes: READS_BUF,
        uav_srv_classes: READS_BUF,
        heap_indexed_or_unknown: true,
        touches_textures: false,
        msc_refined: true,
        msc_exact_classes: READS_BUF,
    };
    let pso_uav_writer = Arc::new(cp_writer);

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g40 msc heap").unwrap();
    let srv_tex = test_texture(4, 1, RGBA32_FLOAT, 0);
    let uav_buf = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    srv_tex.register_ptr();
    uav_buf.register_ptr();
    unsafe {
        let e0 = views::srv_entry(Some(&srv_tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let uav_id = &uav_buf as *const Resource as usize;

    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_ib_tight = true;
    l.opts.opt_ib_exact_obl = true;
    l.opts.opt_msc_stage_usage = true;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_buf.gpu_va).unwrap();

    // 1) Dispatch UAV writer -> `Barrier::Uav(uav_id)` -> Dispatch CBV-only reader:
    //    Because `Barrier::Uav` orders UAV/SRV accesses (not CBVs, which require `Transition(UAV -> CBV)`),
    //    `pso_cbv_only` (`uav_srv_classes == 0`) MUST skip the intra-encoder barrier (`ib_class_skip`++, `enc_barrier == 0`)!
    l.set_compute_pipeline(pso_uav_writer);
    l.dispatch(1, 1, 1).unwrap();
    l.barrier_list_with(&[Barrier::Uav(uav_id)], true, false);
    let enc_b0 = stats::c::enc_barrier.load(Relaxed);
    let cls0 = stats::c::ib_class_skip.load(Relaxed);
    l.set_compute_pipeline(pso_cbv_only.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed) - enc_b0, 0, "CBV-only dispatch must not stall on Barrier::Uav");
    assert!(stats::c::ib_class_skip.load(Relaxed) > cls0);

    // 2) Guardrail: `Transition(uav_id, STATE_UAV, STATE_VERTEX_AND_CONSTANT_BUFFER)` MUST STILL fire
    //    `enc_barrier` before `pso_cbv_only` because `pso_cbv_only.read_classes == READS_BUF`!
    l.barrier_list_with(&[Barrier::Transition(uav_id, STATE_UAV, STATE_VB_CB)], true, false);
    let enc_b1 = stats::c::enc_barrier.load(Relaxed);
    l.set_compute_pipeline(pso_cbv_only);
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed) - enc_b1, 1, "Transition(UAV -> CBV) must still fire enc_barrier for CBV reader");
    l.close().unwrap();
}

#[test]
fn g41_vs_buf_defer_exact_skips_fullscreen_vs_barrier_and_fires_on_indexed_or_cbv_draw_on_gpu() {
    use crate::metal::pipeline::{vs_may_read_cbv_buffer, GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let ps_cb0_dxbc = cb0_color_ps_dxbc();
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let fs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Constants {
                shader_register: 0,
                register_space: 0,
                num_32bit_values: 4,
            },
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let cbv_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Descriptor(
                RootDescriptorKind::Cbv,
                RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
            ),
            visibility: 1, // D3D12_SHADER_VISIBILITY_VERTEX
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();

    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let gdesc = GraphicsDesc {
        vs: &vs_dxil,
        ps: Some(&ps_cb0_dxbc),
        blend: color_blend,
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
        depth_stencil: DepthStencilState::default(),
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: 3,
        rtv_formats: vec![RGBA8_UNORM],
        dsv_format: 0,
        sample_count: 1,
    };
    let pso_fs = Arc::new(GraphicsPipeline::new_with_opts(fs_rs.clone(), &gdesc, "g41_fs", true).unwrap());
    assert!(!pso_fs.vs_reads_cbv_or_ia);
    assert!(
        !vs_may_read_cbv_buffer(&pso_fs.vs, &ps_cb0_dxbc, &fs_rs),
        "inline constants alone must not mark VS CBV buffer read"
    );
    assert!(
        vs_may_read_cbv_buffer(&pso_fs.vs, &ps_cb0_dxbc, &cbv_rs),
        "root CBV descriptor matching cb0 must mark VS CBV buffer read"
    );
    let pso_cbv = Arc::new(GraphicsPipeline {
        pso: pso_fs.pso.clone(),
        rs: pso_fs.rs.clone(),
        raster: pso_fs.raster,
        blend: pso_fs.blend,
        depth_stencil: pso_fs.depth_stencil.clone(),
        depth_bounds: pso_fs.depth_bounds,
        needs_draw_params: pso_fs.needs_draw_params,
        topology_type: pso_fs.topology_type,
        strip_cut: pso_fs.strip_cut,
        rtv_formats: pso_fs.rtv_formats.clone(),
        dsv_format: pso_fs.dsv_format,
        sample_count: pso_fs.sample_count,
        vertex_slots: pso_fs.vertex_slots,
        vs: pso_fs.vs.clone(),
        ps: pso_fs.ps.clone(),
        gs_emu: None,
        tess_emu: None,
        pre_raster_reads: pso_fs.pre_raster_reads,
        vs_reads_cbv_or_ia: true,
        vs_writes_uav: false,
        ps_writes_uav: false,
        ps_read_classes: pso_fs.ps_read_classes,
        ps_srv_uav_classes: pso_fs.ps_srv_uav_classes,
        writes_uav: false,
        uses_uav: false,
        sm66_pre_raster_reads: pso_fs.sm66_pre_raster_reads,
        sm66_vs_reads_cbv_or_ia: true,
        sm66_writes_uav: false,
        sm66_narrowed: false,
        vs_access: crate::metal::pipeline::ComputeAccess::opaque(),
        ps_access: pso_fs.ps_access.clone(),
        vs_hk_profile: pso_fs.vs_hk_profile,
        ps_hk_profile: pso_fs.ps_hk_profile,
        icb_desc: None,
        icb_pso: std::sync::OnceLock::new(),
    });

    let ua = resource_flags::ALLOW_UNORDERED_ACCESS;
    let vb_buf = test_buffer(64, heap_type::DEFAULT, ua);
    let ib_up = test_buffer(64, heap_type::UPLOAD, 0);
    let rt_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&vb_buf, &ib_up, &rt_a, &rt_b, &rb] {
        r.register_ptr();
    }
    unsafe {
        let p = ib_up.buffer().unwrap().contents().as_ptr() as *mut u32;
        *p.add(0) = 0;
        *p.add(1) = 1;
        *p.add(2) = 2;
    }

    let q = Queue::new(ListType::Direct, "g41 vs_buf_defer queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_vs_buf_defer_exact = true;
    l.opts.opt_cs_to_ps_overlap_v2 = false;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_desc[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_desc[16..20].copy_from_slice(&16u32.to_le_bytes());
    l.clear_uav(&vb_buf, Some(&uav_desc), ClearValue::Uint([0x1122_3344, 0, 0, 0]), &[]).unwrap();
    l.end_encoder();
    l.clear_all_pend();
    l.bar.list_unwaited_gfx = false;

    // Transition `vb_buf: UAV -> VERTEX_AND_CONSTANT_BUFFER | INDEX_BUFFER | INDIRECT_ARGUMENT`:
    // Under `AW2_VS_BUF_DEFER_EXACT=1`, `PRE_RASTER` goes into `soft[1]` (`vs_buf_defer`++) instead of hard `vtx_after`!
    let vb_id = &vb_buf as *const Resource as usize;
    let def0 = stats::c::vs_buf_defer.load(Relaxed);
    l.barrier_list_with(
        &[Barrier::Transition(vb_id, STATE_UAV, STATE_VB_CB | 0x2 | 0x200)],
        true,
        false,
    );
    assert!(stats::c::vs_buf_defer.load(Relaxed) > def0, "vs_buf_defer must increment");
    assert!(l.bar.split_pend.vtx_after.is_empty(), "vtx_after must stay empty when deferred to soft[1]");
    assert_eq!(l.bar.split_pend.soft_after[1], MTLStages::Dispatch);

    // Pass #1: Full-screen non-indexed draw (`pso_fs`: `!vs_reads_cbv_or_ia`, `pre_raster_reads == 0`) on `rt_a`
    // MUST NOT stall on `Dispatch` (`enc_sync_before == Fragment`) and MUST carry `soft[1]` to the next pass (`dv_carry`++)!
    let carry0 = stats::c::dv_carry.load(Relaxed);
    l.gfx_root.set_root_signature(fs_rs.clone()).unwrap();
    l.gfx_root.set_constants(0, 0, &[0x3f80_0000, 0, 0x3f80_0000, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![TargetView::rtv(&rt_a, None).unwrap()], None);
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_fs.clone());
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(l.bar.enc_sync_before, MTLStages::Fragment, "full-screen VS draw must NOT wait in PRE_RASTER on VB/CB/IB Dispatch");
    l.end_encoder();
    assert!(stats::c::dv_carry.load(Relaxed) > carry0, "unused VS buffer obligation must carry across full-screen pass");
    assert!(l.bar.vtx_carry[1].is_some());

    // Pass #2: Indexed draw (`draw_indexed`, even with `pso_fs`) MUST demand `READS_BUF` and consume `vtx_carry[1]`!
    l.set_render_targets(vec![TargetView::rtv(&rt_b, None).unwrap()], None);
    l.gfx.ib = Some((ib_up.gpu_va, 12, R32_UINT));
    l.set_graphics_pipeline(pso_fs);
    l.draw_indexed(3, 1, 0, 0, 0).unwrap();
    assert!(l.bar.enc_sync_after.contains(MTLStages::Dispatch), "indexed draw must wait on carried VB/IB Dispatch");
    assert!(l.bar.enc_sync_before.intersects(PRE_RASTER), "indexed draw must wait in PRE_RASTER");
    assert!(l.bar.vtx_defer[1].is_none());
    l.end_encoder();

    // Also verify `pso_cbv` (non-indexed draw with `vs_reads_cbv_or_ia == true`) demands `READS_BUF` at pass open:
    l.clear_uav(&vb_buf, Some(&uav_desc), ClearValue::Uint([0x5566_7788, 0, 0, 0]), &[]).unwrap();
    l.end_encoder();
    l.clear_all_pend();
    l.barrier_list_with(&[Barrier::Transition(vb_id, STATE_UAV, STATE_VB_CB)], true, false);
    l.gfx_root.set_root_signature(fs_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 0x3f80_0000, 0, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![TargetView::rtv(&rt_b, None).unwrap()], None);
    l.set_graphics_pipeline(pso_cbv);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(l.bar.enc_sync_after.contains(MTLStages::Dispatch), "VS CBV draw must wait on VB/CB Dispatch");
    assert!(l.bar.enc_sync_before.intersects(PRE_RASTER), "VS CBV draw must wait in PRE_RASTER");
    l.end_encoder();

    let rt_a_id = &rt_a as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt_a, 0), None).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb, 1)[0], 0xffff_00ff);
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g41_A must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g41_cs_mid_qb_touch_defer_defers_untouched_and_buffer_only_sm66_transitions_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF, READS_TEX};
    use crate::metal::views;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();

    // 1) SM 6.6 bindless buffer-only compute shader (`heap_indexed_or_unknown = true`, `touches_textures = false`).
    let mut cp_sm66_buf = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g41_sm66_buf").unwrap();
    cp_sm66_buf.access = ComputeAccess {
        uav_slots: None,
        read_slots: None,
        bounded_read_slots: Vec::new(),
        unbounded_srv_classes: READS_BUF,
        uav_classes: READS_BUF,
        read_classes: READS_BUF,
        uav_srv_classes: READS_BUF,
        heap_indexed_or_unknown: true,
        touches_textures: false,
        msc_refined: false,
        msc_exact_classes: READS_BUF,
    };
    let pso_sm66_buf = Arc::new(cp_sm66_buf);

    // 2) Exact-touch compute shader (`uav_exact && full_read_exact`, reads slot 0 in heap).
    let mut cp_exact_tex = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g41_exact_tex").unwrap();
    cp_exact_tex.access = ComputeAccess {
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }]),
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
        unbounded_srv_classes: 0,
        uav_classes: READS_BUF,
        read_classes: READS_TEX,
        uav_srv_classes: READS_BUF | READS_TEX,
        heap_indexed_or_unknown: false,
        touches_textures: true,
        msc_refined: false,
        msc_exact_classes: READS_TEX,
    };
    let pso_exact_tex = Arc::new(cp_exact_tex);

    const RGBA32_FLOAT: u32 = 2;
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g41 mid_qb heap").unwrap();
    let other_tex = test_texture(4, 1, RGBA32_FLOAT, 0);
    let rt_tex = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET);
    let uav_buf = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_buf_d4 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb = test_buffer(512, heap_type::READBACK, 0);
    for r in [&other_tex, &rt_tex, &uav_buf, &uav_buf_d4, &rb] {
        r.register_ptr();
    }
    unsafe {
        // Heap slot 0 = `other_tex`, slot 1 = `rt_tex`.
        let e0 = views::srv_entry(Some(&other_tex), None).unwrap();
        let e1 = views::srv_entry(Some(&rt_tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        std::ptr::copy_nonoverlapping(e1.as_ptr(), (heap.cpu_base + 24) as *mut u8, 24);
    }
    let rt_id = &rt_tex as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g41 cs_mid_qb_defer queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_cs_mid_qb_touch_defer = true;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    // Render pass draws a real Fragment clear into `rt_tex`:
    let rtv = TargetView::rtv(&rt_tex, None).unwrap();
    unsafe {
        l.clear_rtv(&rtv, [1.0, 2.0, 3.0, 4.0], &[Rect { left: 0, top: 0, right: 4, bottom: 1 }])
            .unwrap();
    }
    l.end_encoder();
    l.clear_all_pend();

    // Open Compute encoder and run Dispatch #1:
    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap(); // slot 0 = `other_tex`
    l.compute_root.set_descriptor(1, uav_buf.gpu_va).unwrap();
    l.set_compute_pipeline(pso_sm66_buf.clone());
    l.dispatch(1, 1, 1).unwrap();

    // Mid-compute transition on `rt_tex: RENDER_TARGET -> NON_PIXEL_SRV`:
    // Must defer into `cs_mid_res_pend` without polluting `split_pend.disp_after`!
    l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    assert_eq!(l.bar.cs_mid_res_pend.len(), 1);
    assert!(l.bar.split_pend.disp_after.is_empty());

    // Dispatch #2 (`pso_sm66_buf`: SM 6.6 bindless buffer-only `!touches_textures`):
    // Must defer `rt_tex` (`cs_mid_qb_defer`++, `cs_mid_qb == 0`)!
    let mqb0 = stats::c::cs_mid_qb.load(Relaxed);
    let def0 = stats::c::cs_mid_qb_defer.load(Relaxed);
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - mqb0, 0, "SM 6.6 buffer-only dispatch must not fire cs_mid_qb for texture transition");
    assert!(stats::c::cs_mid_qb_defer.load(Relaxed) > def0);
    assert_eq!(l.bar.cs_mid_res_pend.len(), 1);

    // Dispatch #3 (`pso_exact_tex` reading `other_tex` at slot 0, NOT `rt_tex`):
    // Exact `op_touch` proves `rt_tex` is untouched (`cs_mid_qb_defer`++, `cs_mid_qb == 0`)!
    let def1 = stats::c::cs_mid_qb_defer.load(Relaxed);
    l.set_compute_pipeline(pso_exact_tex.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - mqb0, 0, "untouched texture SRV must stay deferred");
    assert!(stats::c::cs_mid_qb_defer.load(Relaxed) > def1);
    assert_eq!(l.bar.cs_mid_res_pend.len(), 1);

    // Dispatch #4 (`pso_exact_tex` pointed at slot 1 = `rt_tex`, writing `uav_buf_d4`):
    // Now `rt_tex` IS touched — `flush_cs_mid_res_for_op` MUST flush and emit `Fragment -> Dispatch` (`cs_mid_qb`++)!
    l.compute_root.set_table(0, heap.gpu_base + 24).unwrap();
    l.compute_root.set_descriptor(1, uav_buf_d4.gpu_va).unwrap();
    l.set_compute_pipeline(pso_exact_tex);
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - mqb0, 1, "touching rt_tex must fire deferred cs_mid_qb right in time");
    assert!(l.bar.cs_mid_res_pend.is_empty());

    let uav_d4_id = &uav_buf_d4 as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(uav_d4_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    l.copy_buffer(&rb, 0, &uav_buf_d4, 0, 16).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    let got = buf_words(&rb, 4);
    assert_eq!(got[0], 1.0f32.to_bits(), "shader must read cleared rt_tex.r == 1.0 after deferred barrier");
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g41_B must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g41_ei_dispatch_exact_elides_false_cross_encoder_and_intra_encoder_barriers_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::indirect::{arg as iarg, IndirectLayout};
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF};
    use crate::metal::views;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();

    let mut cp = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g41_ei_cs").unwrap();
    cp.access = ComputeAccess {
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }]),
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
        unbounded_srv_classes: 0,
        uav_classes: READS_BUF,
        read_classes: READS_BUF,
        uav_srv_classes: READS_BUF,
        heap_indexed_or_unknown: false,
        touches_textures: false,
        msc_refined: false,
        msc_exact_classes: READS_BUF,
    };
    let pso_cs = Arc::new(cp);

    const RGBA32_FLOAT: u32 = 2;
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g41 ei heap").unwrap();
    let srv_tex = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET);
    let args_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let count_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let uav_a = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_unrelated = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb = test_buffer(512, heap_type::READBACK, 0);
    for r in [&srv_tex, &args_buf, &count_buf, &uav_a, &uav_unrelated, &rb] {
        r.register_ptr();
    }
    unsafe {
        let e0 = views::srv_entry(Some(&srv_tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        let ap = args_buf.buffer().unwrap().contents().as_ptr() as *mut u32;
        *ap.add(0) = 1;
        *ap.add(1) = 1;
        *ap.add(2) = 1;
        let cp_ptr = count_buf.buffer().unwrap().contents().as_ptr() as *mut u32;
        *cp_ptr = 1;
    }
    let uav_a_id = &uav_a as *const Resource as usize;
    let unrelated_id = &uav_unrelated as *const Resource as usize;
    let layout = IndirectLayout::new(12, &[(iarg::DISPATCH, [0; 3])]).unwrap();

    let q = Queue::new(ListType::Direct, "g41 ei_dispatch_exact queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_ei_dispatch_exact = true;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    let rtv = TargetView::rtv(&srv_tex, None).unwrap();
    unsafe { l.clear_rtv(&rtv, [4.0, 5.0, 6.0, 7.0], &[]).unwrap() };
    let srv_id = &srv_tex as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(srv_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);

    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_a.gpu_va).unwrap();
    l.set_compute_pipeline(pso_cs.clone());

    // 1) Expanded `ExecuteIndirect` (with `count_buf` -> runs `ei_expand` + indirect dispatch):
    //    Must increment `ei_cs_exact` (>= 2: once for `ei_expand`, once for indirect dispatch),
    //    emit ZERO false `cs_mid_qb` between `ei_expand` and indirect dispatch, leave `split_pend`
    //    completely clean (NO false `vtx_after` / `frag_after` leaked to future render passes!),
    //    and preserve exact `ib_disp_uav == Some([uav_a])`!
    let ei0 = stats::c::ei_cs_exact.load(Relaxed);
    let mqb0 = stats::c::cs_mid_qb.load(Relaxed);
    l.execute_indirect(&layout, 2, args_buf.gpu_va, None, Some(count_buf.gpu_va)).unwrap();
    assert!(stats::c::ei_cs_exact.load(Relaxed) >= ei0 + 2, "ei_cs_exact must count both ei_expand and indirect dispatch");
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - mqb0, 0, "ei_expand -> indirect dispatch must not emit false cs_mid_qb");
    assert!((l.bar.split_pend.vtx_after | l.bar.split_pend.frag_after | l.bar.split_pend.disp_after).is_empty(), "expanded compute ExecuteIndirect must not leak false vtx/frag/disp obligations");
    let disp_uav = if l.opts.opt_ib_substage_narrow { l.ib.ib_sub[0].disp_uav.as_slice() } else { l.ib.ib_disp_uav.as_slice() };
    assert_eq!(disp_uav, Some(&[uav_a_id][..]), "ib_disp_uav must stay exact after ExecuteIndirect");

    // 2) `Barrier::Uav(unrelated_id)` + unexpanded `ExecuteIndirect` on `uav_a`:
    //    Because `ib_disp_uav` stayed exact (`[uav_a_id]`), `uav_unrelated` was never written so
    //    NO `enc_barrier` fires before the unexpanded `ExecuteIndirect`!
    let eb0 = stats::c::enc_barrier.load(Relaxed);
    l.barrier_list_with(&[Barrier::Uav(unrelated_id)], true, false);
    l.execute_indirect(&layout, 1, args_buf.gpu_va, None, None).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed) - eb0, 0, "unrelated UAV barrier after ExecuteIndirect must skip enc_barrier");

    // 3) Guardrail: `Barrier::Uav(uav_a_id)` before another `ExecuteIndirect` touching `uav_a` MUST fire `enc_barrier`!
    l.barrier_list_with(&[Barrier::Uav(uav_a_id)], true, false);
    let eb1 = stats::c::enc_barrier.load(Relaxed);
    l.execute_indirect(&layout, 1, args_buf.gpu_va, None, None).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed) - eb1, 1, "true UAV hazard on uav_a must fire enc_barrier");

    l.barrier_list_with(&[Barrier::Transition(uav_a_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    l.copy_buffer(&rb, 0, &uav_a, 0, 16).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb, 4)[0], 4.0f32.to_bits());
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g41_C must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g41_blit_list_narrow_v2_narrows_untainted_list_opening_blit_and_upgrades_on_tainted_or_dispatch_on_gpu() {
    use std::sync::atomic::Ordering::Relaxed;
    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "g41 blit_list_v2 queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_blit_list_narrow_v2 = true;

    let ua = resource_flags::ALLOW_UNORDERED_ACCESS;
    let up = test_buffer(64, heap_type::UPLOAD, 0);
    let dst1 = test_buffer(64, heap_type::DEFAULT, 0);
    let dst2 = test_buffer(64, heap_type::DEFAULT, ua);
    let rt_tainted = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb_buf = test_buffer(64, heap_type::READBACK, 0);
    let rb_tex = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&up, &dst1, &dst2, &rt_tainted, &rb_buf, &rb_tex] {
        r.register_ptr();
    }
    unsafe {
        *(up.buffer().unwrap().contents().as_ptr() as *mut u32) = 0xcafe_babe;
    }
    let dst1_id = &dst1 as *const Resource as usize;
    let dst2_id = &dst2 as *const Resource as usize;
    let rt_id = &rt_tainted as *const Resource as usize;

    // Pre-populate `rt_tainted` in a prior command list so it holds valid magenta pixels on GPU:
    let mut l_init = CommandList::new(ListType::Direct).unwrap();
    l_init.begin(&alloc).unwrap();
    let rtv_init = TargetView::rtv(&rt_tainted, None).unwrap();
    unsafe { l_init.clear_rtv(&rtv_init, [1.0, 0.0, 1.0, 1.0], &[]).unwrap() };
    l_init.close().unwrap();
    q.submit(&[&l_init.cb]);
    q.wait_idle();

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    // 1) Untainted `Transition(dst1, NON_PIXEL_SRV -> COPY_DEST)` + `up` (UPLOAD buffer):
    l.barrier_list_with(&[Barrier::Transition(dst1_id, STATE_NON_PIXEL_SRV, STATE_COPY_DEST)], true, false);
    let v2_0 = stats::c::blit_list_v2.load(Relaxed);
    l.copy_buffer(&dst1, 0, &up, 0, 16).unwrap();
    assert!(stats::c::blit_list_v2.load(Relaxed) > v2_0, "untainted list-opening Blit must increment blit_list_v2");
    assert_eq!(l.bar.enc_sync_after, COMPUTE_STAGES | PRE_RASTER, "untainted Blit must omit Fragment wait");
    assert!(l.bar.list_cs_unwaited_frag, "list_cs_unwaited_frag must track omitted Fragment wait");
    assert!(l.bar.list_unwaited_gfx);
    assert!(!l.bar.enc_cur_sync[5].pre_list_all);

    // 2) Second untainted Blit (`dst2: NON_PIXEL_SRV -> COPY_DEST`) in the same open encoder stays narrowed!
    let mqb0 = stats::c::cs_mid_qb.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(dst2_id, STATE_NON_PIXEL_SRV, STATE_COPY_DEST)], true, false);
    l.copy_buffer(&dst2, 0, &up, 0, 16).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - mqb0, 0, "second untainted Blit must stay narrowed");
    assert!(l.bar.list_cs_unwaited_frag);

    // 3) Guardrail A: Transitioning `rt_tainted` (`STATE_RENDER_TARGET -> STATE_COPY_SOURCE`) taints `rt_tainted`,
    //    so `copy_texture_region` touching `rt_tainted` MUST upgrade (`upgrade_blit_only_compute_sync`, `cs_mid_qb`++)!
    l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_tex, fp), [0, 0, 0], CopyLoc::Subresource(&rt_tainted, 0), None).unwrap();
    assert!(stats::c::cs_mid_qb.load(Relaxed) > mqb0, "tainted Blit must upgrade to All -> COMPUTE_STAGES");
    assert!(!l.bar.list_cs_unwaited_frag);
    assert!(l.bar.enc_cur_sync[5].pre_list_all);

    l.barrier_list_with(&[Barrier::Transition(dst1_id, STATE_COPY_DEST, STATE_COPY_SOURCE)], true, false);
    l.copy_buffer(&rb_buf, 0, &dst1, 0, 16).unwrap();
    l.close().unwrap();

    // 4) Guardrail B: On a second command list, untainted Blit followed by `Dispatch` (`clear_uav`)
    //    MUST also upgrade via `upgrade_blit_only_compute_sync`!
    let mut l2 = CommandList::new(ListType::Direct).unwrap();
    l2.opts.opt_blit_list_narrow_v2 = true;
    l2.begin(&alloc).unwrap();
    l2.barrier_list_with(&[Barrier::Transition(dst2_id, STATE_COPY_DEST, STATE_COPY_DEST)], true, false);
    l2.copy_buffer(&dst2, 0, &up, 0, 16).unwrap();
    assert!(l2.bar.list_cs_unwaited_frag);
    let mqb1 = stats::c::cs_mid_qb.load(Relaxed);
    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_desc[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_desc[16..20].copy_from_slice(&16u32.to_le_bytes());
    l2.barrier_list_with(&[Barrier::Transition(dst2_id, STATE_COPY_DEST, STATE_UAV)], true, false);
    l2.clear_uav(&dst2, Some(&uav_desc), ClearValue::Uint([0xdead_beef, 0, 0, 0]), &[]).unwrap();
    assert!(stats::c::cs_mid_qb.load(Relaxed) > mqb1, "Dispatch after narrowed Blit must upgrade to All -> COMPUTE_STAGES");
    assert!(!l2.bar.list_cs_unwaited_frag);
    l2.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb, &l2.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb_buf, 1)[0], 0xcafe_babe);
    assert_eq!(buf_words(&rb_tex, 1)[0], 0xffff_00ff);
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g41_D must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g42_list_start_substage_narrows_first_encoder_and_anchors_later_stages_on_gpu() {
    use std::sync::atomic::Ordering::Relaxed;
    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "g42 list_start_substage queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();

    let ua = resource_flags::ALLOW_UNORDERED_ACCESS;
    let rt1 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt2 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let uav1 = test_buffer(64, heap_type::DEFAULT, ua);
    let uav2 = test_buffer(64, heap_type::DEFAULT, ua);
    let rb_uav = test_buffer(64, heap_type::READBACK, 0);
    let rb_tex = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&rt1, &rt2, &uav1, &uav2, &rb_uav, &rb_tex] {
        r.register_ptr();
    }
    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_desc[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_desc[16..20].copy_from_slice(&16u32.to_le_bytes());

    let _stats_guard = gpu_stats_lock();

    // 1) Render-first list (`l1`): encoder #0 opens with `All -> PRE_RASTER | Fragment`
    //    (`list_unwaited_cs = true`, `q_barrier_all == 0`), then encoder #1 (compute)
    //    resolves `list_unwaited_cs` via `All -> COMPUTE_STAGES` and prefers `Vertex`
    //    as `in_list_pre_list_all_anchor`.
    let mut l1 = CommandList::new(ListType::Direct).unwrap();
    l1.opts.opt_list_start_substage = true;
    l1.begin(&alloc).unwrap();
    l1.pt = pt::PassLog::acquire();

    let ls0 = stats::c::list_start_substage.load(Relaxed);
    let rtv1 = TargetView::rtv(&rt1, None).unwrap();
    unsafe { l1.clear_rtv(&rtv1, [0.0, 1.0, 0.0, 1.0], &[Rect { left: 0, top: 0, right: 4, bottom: 4 }]).unwrap() };
    assert!(stats::c::list_start_substage.load(Relaxed) > ls0, "render #0 must increment list_start_substage");
    assert_eq!(l1.bar.enc_sync_after, MTLStages::All);
    assert_eq!(l1.bar.enc_sync_before, PRE_RASTER | MTLStages::Fragment, "render #0 must avoid All -> All");
    assert_eq!(l1.in_list_pre_list_all_anchor(), Some(MTLStages::Vertex), "anchor must prefer lightweight Vertex stage");

    let _ = l1.compute_op_tight(MTLStages::Dispatch, Some(&[]), Some(&[]), Some(&[])).unwrap();
    assert_eq!(l1.bar.enc_sync_after, MTLStages::Vertex, "compute #1 must anchor pre-list work on Vertex instead of All");
    assert_eq!(l1.bar.enc_sync_before, COMPUTE_STAGES);
    assert!(!l1.bar.list_unwaited_cs);
    l1.clear_uav(&uav1, Some(&uav_desc), ClearValue::Uint([0x1234_5678, 0, 0, 0]), &[]).unwrap();
    let uav1_id = &uav1 as *const Resource as usize;
    l1.barrier_list_with(&[Barrier::Transition(uav1_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    l1.copy_buffer(&rb_uav, 0, &uav1, 0, 16).unwrap();
    l1.close().unwrap();

    // 2) Compute-first list (`l2`): encoder #0 opens with `All -> COMPUTE_STAGES`,
    //    preserves `can_narrow_uav_to_cs()`, and encoder #1 (render) anchors pre-list
    //    work via `Dispatch -> PRE_RASTER | Fragment` instead of `All -> ...`!
    let mut l2 = CommandList::new(ListType::Direct).unwrap();
    l2.opts.opt_list_start_substage = true;
    l2.begin(&alloc).unwrap();
    l2.pt = pt::PassLog::acquire();

    let ls2 = stats::c::list_start_substage.load(Relaxed);
    l2.clear_uav(&uav2, Some(&uav_desc), ClearValue::Uint([0x9abc_def0, 0, 0, 0]), &[]).unwrap();
    assert!(stats::c::list_start_substage.load(Relaxed) > ls2, "compute #0 must increment list_start_substage");
    assert!(l2.bar.list_unwaited_gfx, "compute #0 must track unwaited graphics stages");
    assert!(!l2.bar.list_unwaited_cs);
    assert!(l2.can_narrow_uav_to_cs(), "list_start_substage must preserve can_narrow_uav_to_cs in compute #0");
    l2.end_encoder();
    assert_eq!(l2.in_list_pre_list_all_anchor(), Some(MTLStages::Dispatch));

    let rtv2 = TargetView::rtv(&rt2, None).unwrap();
    unsafe { l2.clear_rtv(&rtv2, [1.0, 0.0, 0.0, 1.0], &[Rect { left: 0, top: 0, right: 4, bottom: 4 }]).unwrap() };
    assert!(!l2.bar.list_unwaited_gfx, "render #1 must clear list_unwaited_gfx");
    assert_eq!(l2.bar.enc_sync_after, MTLStages::Dispatch, "render #1 must anchor pre-list work on Dispatch instead of All");
    l2.end_encoder();

    let rt2_id = &rt2 as *const Resource as usize;
    l2.barrier_list_with(&[Barrier::Transition(rt2_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l2.copy_texture_region(CopyLoc::Footprint(&rb_tex, fp), [0, 0, 0], CopyLoc::Subresource(&rt2, 0), None).unwrap();
    l2.close().unwrap();

    let log1 = l1.take_pass_log();
    let log2 = l2.take_pass_log();
    q.submit(&[&l1.cb, &l2.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb_uav, 1)[0], 0x1234_5678);
    assert_eq!(buf_words(&rb_tex, 1)[0], 0xff00_00ff);
    let logs: Vec<_> = [log1, log2].into_iter().flatten().collect();
    if !logs.is_empty() {
        let summary = pt::resolve_for_test(logs);
        assert_eq!(summary.bad, 0, "g42_A must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g42_sm66_gfx_stage_exact_skips_false_vs_tex_and_buf_stalls_and_uav_splits_on_gpu() {
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline, READS_BUF, READS_TEX};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootParameter, RootParameterKind, RootSignature,
        RootSignatureDesc, RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let ps_cb0_dxbc = cb0_color_ps_dxbc();
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Constants {
                shader_register: 0,
                register_space: 0,
                num_32bit_values: 4,
            },
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();

    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let gdesc = GraphicsDesc {
        vs: &vs_dxil,
        ps: Some(&ps_cb0_dxbc),
        blend: color_blend,
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
        depth_stencil: DepthStencilState::default(),
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: 3,
        rtv_formats: vec![RGBA8_UNORM],
        dsv_format: 0,
        sample_count: 1,
    };
    let pso_base = GraphicsPipeline::new_with_opts(rs.clone(), &gdesc, "g42_sm66_base", true).unwrap();
    // Simulate an SM 6.6 graphics PSO whose unnarrowed fields had conservative bindless
    // `READS_TEX | READS_BUF`, `vs_reads_cbv_or_ia = true`, and `writes_uav = true`,
    // while `metallib_res_desc_heap_unused` proved exact zero SM 6.6 heap/UAV usage!
    let pso_sm66 = Arc::new(GraphicsPipeline {
        pso: pso_base.pso.clone(),
        rs: pso_base.rs.clone(),
        raster: pso_base.raster,
        blend: pso_base.blend,
        depth_stencil: pso_base.depth_stencil.clone(),
        depth_bounds: pso_base.depth_bounds,
        needs_draw_params: pso_base.needs_draw_params,
        topology_type: pso_base.topology_type,
        strip_cut: pso_base.strip_cut,
        rtv_formats: pso_base.rtv_formats.clone(),
        dsv_format: pso_base.dsv_format,
        sample_count: pso_base.sample_count,
        vertex_slots: pso_base.vertex_slots,
        vs: pso_base.vs.clone(),
        ps: pso_base.ps.clone(),
        gs_emu: None,
        tess_emu: None,
        pre_raster_reads: READS_TEX | READS_BUF,
        vs_reads_cbv_or_ia: true,
        vs_writes_uav: false,
        ps_writes_uav: false,
        ps_read_classes: pso_base.ps_read_classes,
        ps_srv_uav_classes: pso_base.ps_srv_uav_classes,
        writes_uav: true,
        uses_uav: false,
        sm66_pre_raster_reads: 0,
        sm66_vs_reads_cbv_or_ia: false,
        sm66_writes_uav: false,
        sm66_narrowed: true,
        vs_access: pso_base.vs_access.clone(),
        ps_access: pso_base.ps_access.clone(),
        vs_hk_profile: pso_base.vs_hk_profile,
        ps_hk_profile: pso_base.ps_hk_profile,
        icb_desc: None,
        icb_pso: std::sync::OnceLock::new(),
    });

    let ua = resource_flags::ALLOW_UNORDERED_ACCESS;
    let tex_srv = test_texture(4, 4, RGBA8_UNORM, ua);
    let vb_buf = test_buffer(64, heap_type::DEFAULT, ua);
    let ib_up = test_buffer(64, heap_type::UPLOAD, 0);
    let rt_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&tex_srv, &vb_buf, &ib_up, &rt_a, &rt_b, &rb] {
        r.register_ptr();
    }
    unsafe {
        let p = ib_up.buffer().unwrap().contents().as_ptr() as *mut u32;
        *p.add(0) = 0;
        *p.add(1) = 1;
        *p.add(2) = 2;
    }

    let q = Queue::new(ListType::Direct, "g42 sm66_gfx_exact queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_sm66_gfx_stage_exact = true;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_desc[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_desc[16..20].copy_from_slice(&16u32.to_le_bytes());
    l.clear_uav(&vb_buf, Some(&uav_desc), ClearValue::Uint([0x7777_8888, 0, 0, 0]), &[]).unwrap();
    l.end_encoder();
    l.clear_all_pend();
    l.bar.list_unwaited_gfx = false;

    // Transition BOTH a texture (`UAV -> PIXEL_SRV | NON_PIXEL_SRV`) and a buffer (`UAV -> VB_CB | IB`):
    let tex_id = &tex_srv as *const Resource as usize;
    let vb_id = &vb_buf as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(tex_id, STATE_UAV, STATE_PIXEL_SRV | STATE_NON_PIXEL_SRV),
            Barrier::Transition(vb_id, STATE_UAV, STATE_VB_CB | 0x2),
        ],
        true,
        false,
    );

    // Pass #1: SM 6.6 full-screen draw (`pso_sm66`) MUST skip BOTH `READS_TEX` and `READS_BUF`
    // pre-raster waits (`enc_sync_before == Fragment`), increment `sm66_gfx_exact`, keep `!pass_uav`,
    // AND keep the render pass open across a mid-pass `Barrier::Uav(0)`!
    let sm0 = stats::c::sm66_gfx_exact.load(Relaxed);
    l.gfx_root.set_root_signature(rs.clone()).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 0x3f80_0000, 0x3f80_0000, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![TargetView::rtv(&rt_a, None).unwrap()], None);
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_sm66.clone());
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(stats::c::sm66_gfx_exact.load(Relaxed) > sm0, "sm66_gfx_exact must increment");
    assert_eq!(l.bar.enc_sync_before, MTLStages::Fragment, "SM 6.6 heap-free VS must not stall PRE_RASTER on tex or buf");
    assert!(!l.gfx.pass_uav, "SM 6.6 non-UAV PSO must keep pass_uav == false");
    assert!(!l.bar.uav_gfx_dirty, "SM 6.6 non-UAV PSO must keep uav_gfx_dirty == false");

    // Mid-pass `Barrier::Uav(0)` + second draw MUST stay in the SAME render encoder (no UAV split)!
    let r_open0 = stats::c::draw_pass.load(Relaxed);
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::draw_pass.load(Relaxed) - r_open0, 0, "non-UAV SM 6.6 PSO must not split pass on Barrier::Uav(0)");
    l.end_encoder();
    assert!(l.bar.vtx_carry[0].is_some() && l.bar.vtx_carry[1].is_some(), "both READS_TEX and READS_BUF must carry across SM 6.6 pass");

    // Guardrail: Pass #2 indexed draw (`draw_indexed`) with `pso_sm66` MUST demand `READS_BUF`
    // and consume `vtx_carry[1]` in `PRE_RASTER`!
    l.set_render_targets(vec![TargetView::rtv(&rt_b, None).unwrap()], None);
    l.gfx.ib = Some((ib_up.gpu_va, 12, R32_UINT));
    l.set_graphics_pipeline(pso_sm66);
    l.draw_indexed(3, 1, 0, 0, 0).unwrap();
    assert!(l.bar.enc_sync_after.contains(MTLStages::Dispatch), "indexed SM 6.6 draw must wait on carried IB Dispatch");
    assert!(l.bar.enc_sync_before.intersects(PRE_RASTER), "indexed SM 6.6 draw must wait in PRE_RASTER");
    l.end_encoder();

    let rt_a_id = &rt_a as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt_a, 0), None).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb, 1)[0], 0xffff_ff00);
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g42_B must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g42_cs_mid_qb_exact_v2_preserves_uav0_clear_overlap_and_defers_inline_constant_dispatches_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF};
    use crate::metal::views;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();

    // 1) Inline-constant-only SM 6.6 shader (`read_classes = READS_BUF` conservatively,
    //    `msc_exact_classes = 0`, `uav_slots = Some([RootVa(1)])`, `touches_textures = false`):
    let mut cp_inline = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g42_inline_cs").unwrap();
    cp_inline.access = ComputeAccess {
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: Vec::new(),
        unbounded_srv_classes: 0,
        uav_classes: READS_BUF,
        read_classes: READS_BUF,
        uav_srv_classes: 0,
        heap_indexed_or_unknown: true,
        touches_textures: false,
        msc_refined: false,
        msc_exact_classes: 0,
    };
    let pso_inline = Arc::new(cp_inline);

    // 2) Buffer-reading SM 6.6 shader (`msc_exact_classes = READS_BUF`):
    let mut cp_buf_reader = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g42_buf_reader_cs").unwrap();
    cp_buf_reader.access = ComputeAccess {
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: Vec::new(),
        unbounded_srv_classes: READS_BUF,
        uav_classes: READS_BUF,
        read_classes: READS_BUF,
        uav_srv_classes: READS_BUF,
        heap_indexed_or_unknown: true,
        touches_textures: false,
        msc_refined: false,
        msc_exact_classes: READS_BUF,
    };
    let pso_buf_reader = Arc::new(cp_buf_reader);

    const RGBA32_FLOAT: u32 = 2;
    const D32_FLOAT: u32 = 40;
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g42 v2 heap").unwrap();
    let srv_tex = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET);
    let dsv_tex = test_texture(4, 4, D32_FLOAT, resource_flags::ALLOW_DEPTH_STENCIL);
    let up_cb = test_buffer(64, heap_type::UPLOAD, 0);
    let cb_buf = test_buffer(64, heap_type::DEFAULT, 0);
    let uav_a = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_b = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb = test_buffer(512, heap_type::READBACK, 0);
    for r in [&srv_tex, &dsv_tex, &up_cb, &cb_buf, &uav_a, &uav_b, &rb] {
        r.register_ptr();
    }
    unsafe {
        let e0 = views::srv_entry(Some(&srv_tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        *(up_cb.buffer().unwrap().contents().as_ptr() as *mut u32) = 0x55aa_55aa;
    }

    let q = Queue::new(ListType::Direct, "g42 cs_mid_qb_v2 queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_cs_mid_qb_exact_v2 = true;
    l.opts.opt_uav0_clear_overlap = true;
    l.opts.opt_clear_keep_smart = true;
    l.opts.opt_clear_keep_dsv = false;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    // Populate `srv_tex` (`Fragment`) and `cb_buf` (`Blit`) in earlier closed encoders:
    let rtv = TargetView::rtv(&srv_tex, None).unwrap();
    unsafe { l.clear_rtv(&rtv, [9.0, 0.0, 0.0, 1.0], &[Rect { left: 0, top: 0, right: 4, bottom: 1 }]).unwrap() };
    l.end_encoder();
    let srv_id = &srv_tex as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(srv_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    l.copy_buffer(&cb_buf, 0, &up_cb, 0, 16).unwrap();
    l.end_encoder();

    // Queue a deferred DSV clear BEFORE opening Compute:
    let dsv = TargetView::dsv(&dsv_tex, None).unwrap();
    unsafe { l.clear_dsv(&dsv, true, 1.0, false, 0, &[]).unwrap() };
    assert_eq!(l.clears.pending_clears.len(), 1);

    // Part 1: Open Compute encoder, run Dispatch #1 writing `uav_a`, then issue an
    // intra-encoder `Transition(uav_a, UAV -> NON_PIXEL_SRV)` + `Barrier::Uav(0)`:
    // Under `g42_C`, the already-satisfied `Dispatch -> Dispatch` part goes to `cs_end_after[0]`
    // (`o.disp_after` is empty in `cs_mid_res_pend`!) so `Barrier::Uav(0)` flushing the DSV clear
    // STILL triggers `uav0_clear_overlap` (`pend_after == Dispatch`, overlapping the CLEAR pass)!
    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_a.gpu_va).unwrap();
    l.set_compute_pipeline(pso_inline.clone());
    l.dispatch(1, 1, 1).unwrap();

    let uav_a_id = &uav_a as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(uav_a_id, STATE_UAV, STATE_NON_PIXEL_SRV)], true, false);
    assert!(
        l.bar.cs_mid_res_pend.iter().all(|o| o.disp_after.is_empty()),
        "intra-encoder Dispatch -> Dispatch must NOT leave unsynced disp_after in cs_mid_res_pend"
    );
    assert!(l.bar.cs_end_after[0].contains(MTLStages::Dispatch), "cs_end_after[0] must record cross-encoder Dispatch obligation");

    let ov0 = stats::c::uav0_clear_overlap.load(Relaxed);
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(stats::c::uav0_clear_overlap.load(Relaxed) > ov0, "uav0_clear_overlap must stay alive under g42_C");
    assert_eq!(l.bar.split_pend.disp_after, MTLStages::Dispatch, "post-Uav(0) disp_after must narrow to Dispatch");

    // Re-open Compute after `Barrier::Uav(0)` (overlapping the DSV clear pass!), then test Part 2:
    // Mid-compute cross-encoder `Transition(cb_buf, COPY_DEST -> VB_CB)` (`Blit -> Dispatch` on a BUFFER!):
    // Dispatch #2 (`pso_inline`, `msc_exact_classes == 0`, `uav_slots = [uav_b]`) MUST defer `cb_buf`!
    l.compute_root.set_descriptor(1, uav_a.gpu_va).unwrap();
    l.set_compute_pipeline(pso_inline.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(l.bar.enc_sync_after, MTLStages::Dispatch, "post-clear compute encoder must overlap CLEAR pass");

    let cb_id = &cb_buf as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(cb_id, STATE_COPY_DEST, STATE_VB_CB)], true, false);
    assert!(!l.bar.cs_mid_res_pend.is_empty());
    let mqb0 = stats::c::cs_mid_qb.load(Relaxed);
    let v2_0 = stats::c::cs_mid_qb_v2.load(Relaxed);
    l.compute_root.set_descriptor(1, uav_b.gpu_va).unwrap();
    l.set_compute_pipeline(pso_inline);
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - mqb0, 0, "inline-constant-only shader must defer buffer transition");
    assert!(stats::c::cs_mid_qb_v2.load(Relaxed) > v2_0, "cs_mid_qb_v2 must increment");
    assert_eq!(l.bar.cs_mid_res_pend.len(), 1);

    // Guardrail: Dispatch #3 (`pso_buf_reader`, `msc_exact_classes == READS_BUF`) MUST flush and fire `cs_mid_qb`!
    l.set_compute_pipeline(pso_buf_reader);
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - mqb0, 1, "buffer-reading shader must fire deferred cs_mid_qb");
    assert!(l.bar.cs_mid_res_pend.is_empty());

    // Intra-encoder `Transition(uav_a, NON_PIXEL_SRV -> UAV)` -> `Transition(uav_a, UAV -> NON_PIXEL_SRV)`
    // records `Dispatch` in `cs_end_after[0]`, and `end_encoder()` flushes it into `split_pend.disp_after`:
    l.barrier_list_with(&[Barrier::Transition(uav_a_id, STATE_NON_PIXEL_SRV, STATE_UAV)], true, false);
    l.barrier_list_with(&[Barrier::Transition(uav_a_id, STATE_UAV, STATE_NON_PIXEL_SRV)], true, false);
    l.end_encoder();
    assert!(l.bar.split_pend.disp_after.contains(MTLStages::Dispatch), "end_encoder must flush cs_end_after[0] into split_pend");

    let uav_b_id = &uav_b as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(uav_b_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    l.copy_buffer(&rb, 0, &uav_b, 0, 16).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb, 4)[0], 9.0f32.to_bits());
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g42_C must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g42_ib_substage_narrow_isolates_dispatch_and_blit_intra_encoder_barriers_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF};
    use crate::metal::views;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();

    // Exact buffer-reading/writing shader (`read_slots = [TableEntry(0, 0)]`, `uav_slots = [RootVa(1)]`):
    let mut cp = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g42_ib_sub_cs").unwrap();
    cp.access = ComputeAccess {
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }]),
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
        unbounded_srv_classes: 0,
        uav_classes: READS_BUF,
        read_classes: READS_BUF,
        uav_srv_classes: READS_BUF,
        heap_indexed_or_unknown: false,
        touches_textures: false,
        msc_refined: true,
        msc_exact_classes: READS_BUF,
    };
    let pso_cs = Arc::new(cp);

    const RGBA32_FLOAT: u32 = 2;
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g42 ib_sub heap").unwrap();
    let srv_tex = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET);
    let ua = resource_flags::ALLOW_UNORDERED_ACCESS;
    let buf_a = test_buffer(512, heap_type::DEFAULT, ua);
    let buf_b = test_buffer(512, heap_type::DEFAULT, ua);
    let buf_out = test_buffer(512, heap_type::DEFAULT, ua);
    let rb_a = test_buffer(512, heap_type::READBACK, 0);
    let rb_b = test_buffer(512, heap_type::READBACK, 0);
    let rb_out = test_buffer(512, heap_type::READBACK, 0);
    for r in [&srv_tex, &buf_a, &buf_b, &buf_out, &rb_a, &rb_b, &rb_out] {
        r.register_ptr();
    }
    unsafe {
        let e0 = views::srv_entry(Some(&srv_tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_desc[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_desc[16..20].copy_from_slice(&16u32.to_le_bytes());

    let q = Queue::new(ListType::Direct, "g42 ib_substage_narrow queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_ib_tight = true;
    l.opts.opt_ib_exact_obl = true;
    l.opts.opt_ib_substage_narrow = true;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    let rtv = TargetView::rtv(&srv_tex, None).unwrap();
    unsafe { l.clear_rtv(&rtv, [12.0, 0.0, 0.0, 1.0], &[Rect { left: 0, top: 0, right: 4, bottom: 1 }]).unwrap() };
    l.end_encoder();
    let srv_id = &srv_tex as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(srv_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);

    // Inside a single unified `Encoder::Compute`:
    // 1) Dispatch writes `buf_a` (`0xaaaa_1111`) and `buf_b` (`0xbbbb_2222`).
    l.clear_uav(&buf_a, Some(&uav_desc), ClearValue::Uint([0xaaaa_1111, 0, 0, 0]), &[]).unwrap();
    l.clear_uav(&buf_b, Some(&uav_desc), ClearValue::Uint([0xbbbb_2222, 0, 0, 0]), &[]).unwrap();
    assert!(l.ib.ib_sub[0].stages.contains(MTLStages::Dispatch));
    assert!(l.ib.ib_sub[1].stages.contains(MTLStages::Dispatch));

    // 2) `Transition(buf_a, UAV -> COPY_SOURCE)` + `copy_buffer(&rb_a, &buf_a)`:
    //    Emits narrowed `Dispatch -> Blit` (`ib_substage_narrow`++, `enc_barrier`++), clearing
    //    `Dispatch` from `ib_sub[1]` (`Blit`) while keeping `ib_sub[0]` (`Dispatch`) intact!
    let buf_a_id = &buf_a as *const Resource as usize;
    let buf_b_id = &buf_b as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(buf_a_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    let eb0 = stats::c::enc_barrier.load(Relaxed);
    let ib_n0 = stats::c::ib_substage_narrow.load(Relaxed);
    l.copy_buffer(&rb_a, 0, &buf_a, 0, 16).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed) - eb0, 1, "first Blit after Dispatch must emit Dispatch -> Blit");
    assert!(stats::c::ib_substage_narrow.load(Relaxed) > ib_n0, "ib_substage_narrow must increment");
    assert!(!l.ib.ib_sub[1].stages.contains(MTLStages::Dispatch), "ib_sub[1] (Blit) must clear satisfied Dispatch stage");
    assert!(l.ib.ib_sub[0].stages.contains(MTLStages::Dispatch), "ib_sub[0] (Dispatch) must retain unsatisfied Dispatch stage");

    // 3) `Transition(buf_b, UAV -> NON_PIXEL_SRV | COPY_SOURCE)`:
    //    3a) `copy_buffer(&rb_b, &buf_b)` (`Blit`) SKIPS redundant `Dispatch -> Blit` (`enc_barrier == 0`,
    //        `ib_skip_stage`++) because `ib_sub[1]` (`Blit`) already synced `Dispatch -> Blit`!
    l.barrier_list_with(
        &[Barrier::Transition(buf_b_id, STATE_UAV, STATE_NON_PIXEL_SRV | STATE_COPY_SOURCE)],
        true,
        false,
    );
    let eb1 = stats::c::enc_barrier.load(Relaxed);
    let sk0 = stats::c::ib_skip_stage.load(Relaxed);
    l.copy_buffer(&rb_b, 0, &buf_b, 0, 16).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed) - eb1, 0, "second Blit must skip already-satisfied Dispatch -> Blit");
    assert!(stats::c::ib_skip_stage.load(Relaxed) > sk0);

    //    3b) Guardrail: touching `buf_b` in `Dispatch` MUST fire `Dispatch -> Dispatch` because `ib_sub[0]` retained `buf_b`!
    l.barrier_list_with(&[Barrier::Transition(buf_b_id, STATE_NON_PIXEL_SRV | STATE_COPY_SOURCE, STATE_UAV)], true, false);
    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, buf_out.gpu_va).unwrap();
    // Unrelated `buf_out` must NOT fire `enc_barrier`:
    let eb2 = stats::c::enc_barrier.load(Relaxed);
    l.set_compute_pipeline(pso_cs.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed) - eb2, 0, "unrelated buf_out dispatch must not stall");

    // Touching `buf_b` in `Dispatch` MUST fire `Dispatch -> Dispatch` because `ib_sub[0]` retained `buf_b`!
    let eb3 = stats::c::enc_barrier.load(Relaxed);
    let ib_n1 = stats::c::ib_substage_narrow.load(Relaxed);
    l.compute_root.set_descriptor(1, buf_b.gpu_va).unwrap();
    l.set_compute_pipeline(pso_cs);
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed) - eb3, 1, "ib_sub[0] must preserve Dispatch -> Dispatch hazard on buf_b");
    assert!(stats::c::ib_substage_narrow.load(Relaxed) > ib_n1, "Dispatch -> Dispatch must also narrow via ib_substage_narrow");

    let buf_out_id = &buf_out as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(buf_out_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    l.copy_buffer(&rb_out, 0, &buf_out, 0, 16).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb_a, 1)[0], 0xaaaa_1111);
    assert_eq!(buf_words(&rb_b, 1)[0], 0xbbbb_2222);
    assert_eq!(buf_words(&rb_out, 1)[0], 12.0f32.to_bits());
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g42_D must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn g43_passtiming_per_heap_isolation_lazy_empty_list_and_zero_work_coalesce_on_gpu() {
    if crate::metal::device().is_none() {
        return;
    }
    let (heaps, per_heap) = pt::dedicated_heap_pool_counts().expect("passtiming heap pool");
    assert_eq!(heaps, 8, "passtiming must allocate strictly 8 MTL4CounterHeaps to leave >=23 AGX heaps for D3D12 CreateQueryHeap");
    assert_eq!(per_heap, 8, "each 4096-entry counter heap must provide 8 slices of 512 timestamps (64 total)");

    // Regression check for frame-0 AGX counter-heap limit (~31-32 heaps/process):
    // Northlight's D3D12 `CreateQueryHeap(TIMESTAMP, 128)` must succeed alongside `passtiming`.
    let mut d3d_qheaps = Vec::new();
    for _ in 0..8 {
        d3d_qheaps.push(
            crate::metal::query::QueryHeap::new(crate::metal::query::heap_type::TIMESTAMP, 128)
                .expect("D3D12 TIMESTAMP QueryHeap(128) must succeed alongside 8-heap passtiming pool"),
        );
    }

    let q = Queue::new(ListType::Direct, "g43 passtiming isolation queue").unwrap();
    let alloc_empty = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l_empty = CommandList::new(ListType::Direct).unwrap();
    l_empty.begin(&alloc_empty).unwrap();
    l_empty.pt_armed = true;
    l_empty.close().unwrap();
    assert!(
        l_empty.take_pass_log().is_none(),
        "empty command list must lazily skip PassLog acquisition so 0-work lists never consume counter heaps"
    );

    // Pack 3 active PassLogs onto the SAME MTL4CounterHeap (`active_count[h] == 3`), drop a
    // used sibling PassLog (`used > 0`) BEFORE resolving `log1` and `log2` to prove quiescent
    // deferred invalidation never zeroes sibling slices on the same heap, and verify 0-work
    // `copy_buffer` compute encoders coalesce with `bad == 0`!
    let src = test_buffer(256, heap_type::UPLOAD, 0);
    let mid = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let dst = test_buffer(256, heap_type::READBACK, 0);
    for r in [&src, &mid, &dst] {
        r.register_ptr();
    }
    unsafe {
        *(src.buffer().unwrap().contents().as_ptr() as *mut u32) = 0xfeed_c0de;
    }
    let mid_id = &mid as *const Resource as usize;

    let a_sib = CommandAllocator::new(ListType::Direct).unwrap();
    let a1 = CommandAllocator::new(ListType::Direct).unwrap();
    let a2 = CommandAllocator::new(ListType::Direct).unwrap();

    let mut l_sib = CommandList::new(ListType::Direct).unwrap();
    l_sib.begin(&a_sib).unwrap();
    l_sib.pt_armed = true;
    l_sib.copy_buffer(&mid, 64, &src, 0, 64).unwrap();
    l_sib.close().unwrap();
    let mut log_sib = l_sib.take_pass_log().expect("sibling list must acquire PassLog");

    let mut l1 = CommandList::new(ListType::Direct).unwrap();
    l1.begin(&a1).unwrap();
    l1.pt_armed = true;
    l1.copy_buffer(&mid, 0, &src, 0, 64).unwrap();
    l1.barrier_list_with(&[Barrier::Transition(mid_id, STATE_COPY_DEST, STATE_COPY_SOURCE)], true, true);
    l1.close().unwrap();
    let mut log1 = l1.take_pass_log().expect("non-empty list l1 must acquire PassLog");

    let mut l2 = CommandList::new(ListType::Direct).unwrap();
    l2.begin(&a2).unwrap();
    l2.pt_armed = true;
    l2.copy_buffer(&dst, 0, &mid, 0, 64).unwrap();
    l2.close().unwrap();
    let mut log2 = l2.take_pass_log().expect("non-empty list l2 must acquire PassLog");

    assert!(
        std::ptr::eq(log_sib.heap(), log1.heap()) && std::ptr::eq(log1.heap(), log2.heap()),
        "concurrent lists must pack onto the same MTL4CounterHeap"
    );

    log_sib.submitted(pt::mach_now(), 0, 0);
    log1.submitted(pt::mach_now(), 0, 0);
    log2.submitted(pt::mach_now(), 0, 0);
    q.submit(&[&l_sib.cb, &l1.cb, &l2.cb]);
    q.wait_idle();
    // Drop `log_sib` (`used > 0`) BEFORE resolving `log1` and `log2` on the same counter heap:
    // quiescent invalidation MUST defer until `log1` and `log2` also drop (`active_count == 0`)!
    drop(log_sib);

    assert_eq!(buf_words(&dst, 1)[0], 0xfeed_c0de);
    let summary = pt::resolve_for_test(vec![log1, log2]);
    assert_eq!(
        summary.bad, 0,
        "quiescent-heap PassLog isolation + 0-work compute coalescing must yield 0 bad timestamps; raw_ts={:?}",
        summary.raw_ts
    );
    assert!(summary.raw_ts.len() >= 2, "both copy encoders must resolve valid GPU intervals");
    drop(d3d_qheaps);
}

#[test]
fn g43_stage_a_sm66_cs_stage_exact_narrows_dead_ro_classes_without_cs_end_after_side_effect_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputePipeline, READS_BUF, READS_TEX};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let pso_base = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g43_a_cs").unwrap();
    // Model an SM 6.6 compute PSO where DXIL declared both `READS_TEX | READS_BUF`
    // (`msc_refined = false`), while `exact_classes_from_metallib` proved `msc_exact_classes == READS_TEX`
    // (zero buffer SRV/CBV reads!).
    let pso_sm66 = Arc::new(ComputePipeline {
        pso: pso_base.pso.clone(),
        rs: pso_base.rs.clone(),
        shader: pso_base.shader.clone(),
        threadgroup: pso_base.threadgroup,
        hk_profile: pso_base.hk_profile,
        access: ComputeAccess {
            uav_slots: pso_base.access.uav_slots.clone(),
            read_slots: None,
            bounded_read_slots: Vec::new(),
            heap_indexed_or_unknown: true,
            touches_textures: true,
            unbounded_srv_classes: READS_TEX,
            uav_classes: READS_BUF,
            read_classes: READS_TEX | READS_BUF,
            msc_exact_classes: READS_TEX,
            uav_srv_classes: 0,
            msc_refined: false,
        },
    });

    let tex = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let buf_written = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let buf_uav = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let buf_src = test_buffer(512, heap_type::UPLOAD, 0);
    let rb = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex, &buf_written, &buf_uav, &buf_src, &rb] {
        r.register_ptr();
    }
    unsafe {
        *(buf_src.buffer().unwrap().contents().as_ptr() as *mut u32) = 0x1234_5678;
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g43_a heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rtv = TargetView::rtv(&tex, None).unwrap();
    let tex_id = &tex as *const Resource as usize;
    let buf_written_id = &buf_written as *const Resource as usize;
    let buf_uav_id = &buf_uav as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g43_a queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |sm66_cs_on: bool| -> (u64, u64, MTLStages, Vec<u32>) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_sm66_cs_stage_exact = sm66_cs_on;
        let eb0 = stats::c::enc_barrier.load(Relaxed);
        let st0 = stats::c::sm66_cs_exact.load(Relaxed);

        l.begin(&alloc).unwrap();
        unsafe { l.clear_rtv(&rtv, [2.0, 4.0, 6.0, 1.0], &[]).unwrap() };
        l.barrier_list_with(&[Barrier::Transition(tex_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
        // Open compute encoder with `Blit` write to `buf_written`, then transition `buf_written`
        // `COPY_DEST -> NON_PIXEL_SRV`: adds `OblKind::ClassSrv(READS_BUF)` `Blit -> Dispatch`!
        l.copy_buffer(&buf_written, 0, &buf_src, 0, 64).unwrap();
        l.barrier_list_with(&[Barrier::Transition(buf_written_id, STATE_COPY_DEST, STATE_NON_PIXEL_SRV)], true, false);

        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, buf_uav.gpu_va).unwrap();
        l.set_compute_pipeline(pso_sm66.clone());
        l.dispatch(1, 1, 1).unwrap();
        let cs_end0 = l.bar.cs_end_after[0];
        let eb_delta = stats::c::enc_barrier.load(Relaxed) - eb0;
        let st_delta = stats::c::sm66_cs_exact.load(Relaxed) - st0;

        l.barrier_list_with(&[Barrier::Transition(buf_uav_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
        l.copy_buffer(&rb, 0, &buf_uav, 0, 64).unwrap();
        l.close().unwrap();
        q.submit(&[&l.cb]);
        q.wait_idle();
        alloc.reset();
        (eb_delta, st_delta, cs_end0, buf_words(&rb, 4))
    };

    let (eb_off, st_off, cs_end_off, ref_words) = run_case(false);
    let (eb_on, st_on, cs_end_on, opt_words) = run_case(true);
    assert_eq!(eb_off, 1, "without AW2_SM66_CS_STAGE_EXACT, dead READS_BUF triggers Blit -> Dispatch enc_barrier");
    assert_eq!(st_off, 0);
    assert_eq!(eb_on, 0, "AW2_SM66_CS_STAGE_EXACT=1 must skip false Blit -> Dispatch enc_barrier on dead READS_BUF");
    assert!(st_on >= 1, "sm66_cs_exact counter must increment");
    assert_eq!(cs_end_on, cs_end_off, "g43_A must NOT pollute cs_end_after (zero g42_C side-effects)");
    assert_eq!(opt_words, ref_words);
    assert_eq!(f32::from_bits(opt_words[0]), 2.0);
}

#[test]
fn g43_stage_b_list_start_render_split_splits_first_render_and_protects_later_vertex_and_compute_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputePipeline, GraphicsDesc, GraphicsPipeline, READS_TEX};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignature,
        RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let gfx_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Constants {
                shader_register: 0,
                register_space: 0,
                num_32bit_values: 2,
            },
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let pso_no_tex = Arc::new(
        GraphicsPipeline::new(
            gfx_rs.clone(),
            &GraphicsDesc {
                vs: &vs_dxil,
                ps: None,
                blend: BlendDesc::default(),
                sample_mask: u32::MAX,
                raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
                depth_stencil: DepthStencilState {
                    depth_compare: MTLCompareFunction::Always,
                    depth_write: true,
                    ..DepthStencilState::default()
                },
                input_layout: Vec::new(),
                strip_cut: 0,
                topology_type: 3,
                rtv_formats: Vec::new(),
                dsv_format: D32_S8,
                sample_count: 1,
            },
            "g43_b_pso_no_tex",
        )
        .unwrap(),
    );
    let pso_vs_tex = Arc::new(GraphicsPipeline {
        pso: pso_no_tex.pso.clone(),
        rs: pso_no_tex.rs.clone(),
        raster: pso_no_tex.raster,
        blend: pso_no_tex.blend,
        depth_stencil: pso_no_tex.depth_stencil.clone(),
        depth_bounds: pso_no_tex.depth_bounds,
        needs_draw_params: pso_no_tex.needs_draw_params,
        topology_type: pso_no_tex.topology_type,
        strip_cut: pso_no_tex.strip_cut,
        rtv_formats: pso_no_tex.rtv_formats.clone(),
        dsv_format: pso_no_tex.dsv_format,
        sample_count: pso_no_tex.sample_count,
        vertex_slots: pso_no_tex.vertex_slots,
        vs: pso_no_tex.vs.clone(),
        ps: pso_no_tex.ps.clone(),
        gs_emu: None,
        tess_emu: None,
        pre_raster_reads: READS_TEX,
        vs_reads_cbv_or_ia: false,
        vs_writes_uav: false,
        ps_writes_uav: false,
        ps_read_classes: 0,
        ps_srv_uav_classes: 0,
        writes_uav: false,
        uses_uav: false,
        sm66_pre_raster_reads: READS_TEX,
        sm66_vs_reads_cbv_or_ia: false,
        sm66_writes_uav: false,
        sm66_narrowed: false,
        vs_access: crate::metal::pipeline::ComputeAccess::opaque(),
        ps_access: None,
        vs_hk_profile: pso_no_tex.vs_hk_profile,
        ps_hk_profile: pso_no_tex.ps_hk_profile,
        icb_desc: None,
        icb_pso: std::sync::OnceLock::new(),
    });

    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g43_b_cs").unwrap());

    let tex = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let ds1 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let ds2 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let uav_buf = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_buf = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex, &ds1, &ds2, &uav_buf, &rb_buf] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g43_b heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rtv = TargetView::rtv(&tex, None).unwrap();
    let dsv1 = TargetView::dsv(&ds1, None).unwrap();
    let dsv2 = TargetView::dsv(&ds2, None).unwrap();
    let tex_id = &tex as *const Resource as usize;
    let uav_id = &uav_buf as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g43_b queue").unwrap();
    let a1 = CommandAllocator::new(ListType::Direct).unwrap();
    let a2 = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    // List 1: Render-pass RTV clear (`Fragment` write on `tex`) + transition to `NON_PIXEL_SRV | PIXEL_SRV`.
    let mut l1 = CommandList::new(ListType::Direct).unwrap();
    l1.begin(&a1).unwrap();
    unsafe { l1.clear_rtv(&rtv, [9.0, 18.0, 27.0, 1.0], &[]).unwrap() };
    l1.barrier_list_with(
        &[Barrier::Transition(tex_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)],
        true,
        true,
    );
    l1.close().unwrap();

    // List 2 (`AW2_LIST_START_RENDER_SPLIT=1`):
    let rsplit0 = stats::c::list_start_rsplit.load(Relaxed);
    let mut l2 = CommandList::new(ListType::Direct).unwrap();
    l2.opts.opt_list_start_render_split = true;
    // Isolate from process-global ANY_FRAG_BUF_WRITE latched by earlier PSO tests.
    l2.opts.test_force_frag_buf_write = Some(false);
    l2.begin(&a2).unwrap();
    l2.gfx_root.set_root_signature(gfx_rs).unwrap();
    l2.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
    l2.set_render_targets(Vec::new(), Some(dsv1));
    l2.gfx.topology = 4;
    l2.set_graphics_pipeline(pso_no_tex);
    l2.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.25, zfar: 1.0 }]);
    l2.draw(3, 1, 0, 0).unwrap();

    assert!(stats::c::list_start_rsplit.load(Relaxed) > rsplit0, "list_start_rsplit must fire on render-first list");
    assert!(!l2.bar.enc_cur_sync[0].pre_list_all, "PRE_RASTER must NOT stall on pre-list Fragment at Pass 1 open");
    assert!(l2.bar.enc_cur_sync[STAGE_FRAGMENT].pre_list_all, "Fragment MUST anchor pre_list_all at Pass 1 open");
    assert!(l2.bar.list_unwaited_vtx_frag && l2.bar.list_cs_unwaited_frag);

    l2.end_encoder();
    assert_eq!(
        l2.bar.vtx_carry[0],
        Some((MTLStages::Fragment, PRE_RASTER)),
        "Pass 1 close must downgrade carried vtx_defer[0] from All -> Fragment once Pass 1 Fragment anchored pre_list_all"
    );

    l2.set_descriptor_heaps(heap.gpu_base, 0);
    l2.compute_root.set_root_signature(cs_rs).unwrap();
    l2.compute_root.set_table(0, heap.gpu_base).unwrap();
    l2.compute_root.set_descriptor(1, uav_buf.gpu_va).unwrap();
    l2.set_compute_pipeline(cs_pso);
    l2.dispatch(1, 1, 1).unwrap();
    assert!(
        l2.bar.enc_cur_sync[STAGE_DISPATCH].pre_list_all && !l2.bar.list_cs_unwaited_frag,
        "compute reading pre-list Fragment texture after split render pass must wait on transitive Fragment anchor"
    );

    l2.set_render_targets(Vec::new(), Some(dsv2));
    l2.set_graphics_pipeline(pso_vs_tex);
    l2.draw(3, 1, 0, 0).unwrap();
    assert!(!l2.bar.list_unwaited_vtx_frag, "texture-reading VS draw must satisfy carried Fragment -> PRE_RASTER obligation");

    l2.barrier_list_with(&[Barrier::Transition(uav_id, STATE_UAV, STATE_COPY_SOURCE)], true, true);
    l2.copy_buffer(&rb_buf, 0, &uav_buf, 0, 64).unwrap();
    l2.close().unwrap();

    q.submit(&[&l1.cb, &l2.cb]);
    q.wait_idle();
    let vals: Vec<f32> = buf_words(&rb_buf, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&vals[..], &[9.0, 18.0, 27.0, 1.0]);
}

#[test]
fn g43_stage_c_cs_mid_qb_defer_v3_defers_disjoint_mid_compute_barrier_and_preserves_uav0_overlap_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::ComputePipeline;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g43_c_cs").unwrap());

    // Non-ShaderWrite textures force render-pass clears (`Fragment` stage!).
    let tex_a = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_b = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let uav_a = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_b = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_a = test_buffer(512, heap_type::READBACK, 0);
    let rb_b = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &uav_a, &uav_b, &rb_a, &rb_b] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g43_c heap").unwrap();
    let stride: usize = 24;
    unsafe {
        let ea = crate::metal::views::srv_entry(Some(&tex_a), None).unwrap();
        let eb = crate::metal::views::srv_entry(Some(&tex_b), None).unwrap();
        std::ptr::copy_nonoverlapping(ea.as_ptr(), heap.cpu_base as *mut u8, 24);
        std::ptr::copy_nonoverlapping(eb.as_ptr(), (heap.cpu_base + stride) as *mut u8, 24);
    }
    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let uav_a_id = &uav_a as *const Resource as usize;
    let uav_b_id = &uav_b as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g43_c queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut l_init = CommandList::new(ListType::Direct).unwrap();
    l_init.begin(&alloc_init).unwrap();
    unsafe { l_init.encode_rtv_clear_pass(&rtv_a, [3.0, 6.0, 9.0, 1.0]).unwrap() };
    l_init.barrier_list_with(&[Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    l_init.close().unwrap();

    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_cs_mid_qb_defer_v3 = true;
    let v3_0 = stats::c::cs_mid_qb_v3.load(Relaxed);

    l.begin(&alloc).unwrap();
    // 1) Open Compute #0 (`All -> COMPUTE_STAGES`, `list_unwaited_cs = false`) reading pre-list `tex_a`, then close it.
    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_a.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso);
    l.dispatch(1, 1, 1).unwrap();
    l.end_encoder();

    // 2) Render Pass produces `tex_b` (`stage_gen[Fragment] == 1`) WITHOUT transitioning `tex_b` yet!
    l.bar.in_list_gfx_non_clear = true;
    unsafe { l.encode_rtv_clear_pass(&rtv_b, [5.0, 10.0, 15.0, 1.0]).unwrap() };

    // 3) Open Compute #1 with Dispatch #1 reading `tex_a` (`enc_cur_sync[Dispatch].gen[Fragment] == 0 < 1`!).
    l.dispatch(1, 1, 1).unwrap();
    assert!(matches!(l.enc, Encoder::Compute(_)));
    assert_eq!(l.bar.enc_cur_sync[STAGE_DISPATCH].gen[STAGE_FRAGMENT], 0);

    // Mid-compute transition of `tex_b` (`RENDER_TARGET -> NON_PIXEL_SRV`, `req[Fragment] = 1`):
    l.barrier_list_with(&[Barrier::Transition(tex_b_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    assert!(l.bar.cs_mid_v3_poisoned == [false; 3] && !l.bar.cs_mid_v3_items.is_empty());

    // Dispatch #2 STILL binds `tex_a` (disjoint from `tex_b`!): `g43_C` MUST defer the mid-compute
    // `Fragment -> Dispatch` queue barrier (`cs_mid_qb` stays 0 here, `cs_mid_qb_v3` increments)!
    let qbm0 = stats::c::cs_mid_qb.load(Relaxed);
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - qbm0, 0, "disjoint dispatch on tex_a must defer mid-CS queue barrier");
    assert!(stats::c::cs_mid_qb_v3.load(Relaxed) > v3_0, "cs_mid_qb_v3 must increment on disjoint deferral");
    assert!(!l.bar.split_pend.disp_after.is_empty(), "unsatisfied tex_b obligation must remain armed in split_pend.disp_after");

    // Dispatch #3 binds `tex_b` (`heap.gpu_base + stride`): MUST emit the `Fragment -> Dispatch` mid-CS queue barrier!
    let qbm1 = stats::c::cs_mid_qb.load(Relaxed);
    l.compute_root.set_table(0, heap.gpu_base + stride as u64).unwrap();
    l.compute_root.set_descriptor(1, uav_b.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - qbm1, 1, "touching tex_b must emit exact deferred Fragment -> Dispatch mid-CS barrier");
    assert!(l.bar.split_pend.disp_after.is_empty() && l.bar.cs_mid_v3_items.is_empty());

    l.barrier_list_with(
        &[
            Barrier::Transition(uav_a_id, STATE_UAV, STATE_COPY_SOURCE),
            Barrier::Transition(uav_b_id, STATE_UAV, STATE_COPY_SOURCE),
        ],
        true,
        false,
    );
    l.copy_buffer(&rb_a, 0, &uav_a, 0, 64).unwrap();
    l.copy_buffer(&rb_b, 0, &uav_b, 0, 64).unwrap();
    l.close().unwrap();

    q.submit(&[&l_init.cb, &l.cb]);
    q.wait_idle();
    let va: Vec<f32> = buf_words(&rb_a, 4).into_iter().map(f32::from_bits).collect();
    let vb: Vec<f32> = buf_words(&rb_b, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&va[..], &[3.0, 6.0, 9.0, 1.0]);
    assert_eq!(&vb[..], &[5.0, 10.0, 15.0, 1.0]);
}

#[test]
fn g43_stage_d_rp_keep_write_exact_eliminates_rp1_false_poisoning_and_enforces_d3_soundness_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputePipeline, GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignature,
        RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let gfx_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Constants {
                shader_register: 0,
                register_space: 0,
                num_32bit_values: 2,
            },
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let gfx_pso = Arc::new(
        GraphicsPipeline::new(
            gfx_rs.clone(),
            &GraphicsDesc {
                vs: &vs_dxil,
                ps: None,
                blend: BlendDesc::default(),
                sample_mask: u32::MAX,
                raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
                depth_stencil: DepthStencilState {
                    depth_compare: MTLCompareFunction::Always,
                    depth_write: true,
                    ..DepthStencilState::default()
                },
                input_layout: Vec::new(),
                strip_cut: 0,
                topology_type: 3,
                rtv_formats: Vec::new(),
                dsv_format: D32_S8,
                sample_count: 1,
            },
            "g43_d_gfx",
        )
        .unwrap(),
    );
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g43_d_cs").unwrap());

    let tex_rp0 = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_unattached = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let ds_rp1 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let uav_out = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_out = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex_rp0, &tex_unattached, &ds_rp1, &uav_out, &rb_out] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g43_d heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex_rp0), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rtv0 = TargetView::rtv(&tex_rp0, None).unwrap();
    let dsv1 = TargetView::dsv(&ds_rp1, None).unwrap();
    let tex_rp0_id = &tex_rp0 as *const Resource as usize;
    let tex_unatt_id = &tex_unattached as *const Resource as usize;
    let uav_out_id = &uav_out as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g43_d queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |exact_on: bool| -> (MTLStages, u64, Vec<f32>) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_rp_keep_write_exact = exact_on;
        let rpk0 = stats::c::rp_keep_exact.load(Relaxed);

        l.begin(&alloc).unwrap();
        // RP_0 (`stage_gen[Fragment] == 1`): produce `tex_rp0` (`[4, 8, 12, 1]`).
        unsafe { l.encode_rtv_clear_pass(&rtv0, [4.0, 8.0, 12.0, 1.0]).unwrap() };

        // RP_1 opens with depth-only draw (`ds_rp1`, `pso` has `ps: None`, `pre_raster_reads == 0`,
        // `ps_read_classes == 0`, `!uses_uav`):
        l.gfx_root.set_root_signature(gfx_rs.clone()).unwrap();
        l.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
        l.set_render_targets(Vec::new(), Some(dsv1));
        l.gfx.topology = 4;
        l.set_graphics_pipeline(gfx_pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.5, zfar: 1.0 }]);
        l.draw(3, 1, 0, 0).unwrap();

        l.barrier_list_with(&[Barrier::Transition(tex_rp0_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
        l.barrier_list_with(&[Barrier::Transition(tex_unatt_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
        assert!(l.gfx.draw_pass, "both unattached transitions must keep RP_1 open");

        let ds_rp1_id = &ds_rp1 as *const Resource as usize;
        assert!(l.bar.pass_targets.contains(&ds_rp1_id));

        // Close RP_1 (`stage_gen[Fragment]` advances `1 -> 2`, `ds_rp1_id` moves into `self.written`):
        l.end_encoder();
        assert!(l.bar.written.contains(&ds_rp1_id));
        let disp_req_frag = l.bar.split_pend.disp_req[STAGE_FRAGMENT];
        let frag_req_frag = l.bar.split_pend.frag_req[STAGE_FRAGMENT];
        assert_eq!(
            disp_req_frag,
            if exact_on { 1 } else { 2 },
            "g43_D must stamp kept rb_keep transition with stage_gen (1) instead of cur_producer_gen (2)"
        );
        assert_eq!(
            frag_req_frag,
            if exact_on { 1 } else { 2 },
            "g43_D must keep untouched rb_keep_write transition at req (1) instead of poisoning with RP_1 (2)"
        );
        if exact_on {
            // D3 soundness check: if an open pass already synced `gen 1` (`RP_0`) but NOT `gen 2` (`RP_1`),
            // an elided `req[Fragment] == 1` mid-pass barrier MUST NOT clear `ds_rp1_id` from `l.written`!
            let mut saved_sync = l.bar.enc_cur_sync;
            saved_sync[STAGE_FRAGMENT].pre_list_all = true;
            saved_sync[STAGE_FRAGMENT].gen[STAGE_FRAGMENT] = 1;
            l.bar.enc_cur_sync = saved_sync;
            let mut test_sp = SplitPend::default();
            let mut req1 = [0u32; 7];
            req1[STAGE_FRAGMENT] = 1;
            test_sp.add_with_req(MTLStages::Fragment, MTLStages::Fragment, [MTLStages::empty(); 2], req1);
            let saved_sp = std::mem::replace(&mut l.bar.split_pend, test_sp);
            l.sync_legacy_from_split();
            l.collect_batch_qbarriers(0);
            assert!(
                l.bar.written.contains(&ds_rp1_id),
                "D3 invariant: elided gen-1 Fragment barrier must NOT clear gen-2 written target"
            );
            l.bar.enc_cur_sync = [StageSync::default(); 7];
            l.bar.split_pend = saved_sp;
            l.sync_legacy_from_split();
        }

        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, uav_out.gpu_va).unwrap();
        l.set_compute_pipeline(cs_pso.clone());
        l.dispatch(1, 1, 1).unwrap();
        let cs_sync = l.bar.enc_sync_after;
        let rpk_delta = stats::c::rp_keep_exact.load(Relaxed) - rpk0;

        l.barrier_list_with(&[Barrier::Transition(uav_out_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
        l.copy_buffer(&rb_out, 0, &uav_out, 0, 64).unwrap();
        l.close().unwrap();

        q.submit(&[&l.cb]);
        q.wait_idle();
        alloc.reset();
        let vals: Vec<f32> = buf_words(&rb_out, 4).into_iter().map(f32::from_bits).collect();
        (cs_sync, rpk_delta, vals)
    };

    let (_sync_off, rpk_off, ref_vals) = run_case(false);
    let (_sync_on, rpk_on, opt_vals) = run_case(true);
    assert_eq!(rpk_off, 0);
    assert!(rpk_on >= 2, "rp_keep_exact must increment for both rb_keep and untouched rb_keep_write");
    assert_eq!(opt_vals, ref_vals);
    assert_eq!(&opt_vals[..], &[4.0, 8.0, 12.0, 1.0]);
}

#[test]
fn g44_passtiming_dsv_only_and_color_clears_stamp_valid_timestamps_across_heap_reuse_on_gpu() {
    if crate::metal::device().is_none() {
        return;
    }
    let (heaps, per_heap) = pt::dedicated_heap_pool_counts().expect("passtiming heap pool");
    assert_eq!(heaps, 8, "HEAPS must remain strictly 8");
    assert_eq!(per_heap, 8, "each 4096-entry heap must hold 8 slices of 512 timestamps");

    let q = Queue::new(ListType::Direct, "g44 dsv clear passtiming queue").unwrap();
    let ds_shadow = test_texture(128, 64, 40, resource_flags::ALLOW_DEPTH_STENCIL); // D32_FLOAT
    let ds_gbuf = test_texture(64, 64, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let rt_nows = test_texture(96, 64, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let src = test_buffer(256, heap_type::UPLOAD, 0);
    let dst = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    for r in [&ds_shadow, &ds_gbuf, &rt_nows, &src, &dst] {
        r.register_ptr();
    }
    let dsv_shadow = TargetView::dsv(&ds_shadow, None).unwrap();
    let dsv_gbuf = TargetView::dsv(&ds_gbuf, None).unwrap();
    let rtv_color = TargetView::rtv(&rt_nows, None).unwrap();

    // Run 10 full submit+resolve+drop cycles across the 8-heap pool so quiescent heap
    // invalidation zeroes heaps repeatedly while verifying BOTH DSV-only `CLEAR` passes
    // (`Vertex`/`Vertex`) and color `CLEAR` passes (`Fragment`/`Fragment`) resolve `bad == 0`
    // and `kind_n[CLEAR] == 3` every single frame!
    for iter in 0..10 {
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        unsafe {
            l.clear_dsv(&dsv_shadow, true, 1.0, false, 0, &[]).unwrap();
            l.clear_dsv(&dsv_gbuf, true, 0.0, true, 0, &[]).unwrap();
            l.clear_rtv(&rtv_color, [0.25, 0.5, 0.75, 1.0], &[]).unwrap();
        }
        l.copy_buffer(&dst, 0, &src, 0, 64).unwrap();
        l.barrier_list_with(&[Barrier::Full], true, false);
        l.close().unwrap();

        let mut log = l.take_pass_log().expect("list with 3 clears + blit must acquire PassLog");
        log.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();

        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(
            summary.bad, 0,
            "iter {iter}: DSV-only + color CLEAR passes must resolve with 0 bad timestamps; raw_ts={:x?}",
            summary.raw_ts
        );
        assert_eq!(
            summary.kind_n[pt::kind::CLEAR as usize], 3,
            "iter {iter}: all 3 CLEAR passes (2 DSV-only + 1 RTV) must record non-zero timestamps; kind_n={:?}",
            summary.kind_n
        );
    }
}

#[test]
fn g44_rp_keep_write_exact_ps_uav_preserves_cs_uav_mid_skip_under_dom_fold_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputePipeline, GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignature,
        RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let gfx_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Constants {
                shader_register: 0,
                register_space: 0,
                num_32bit_values: 2,
            },
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let pso_base = GraphicsPipeline::new(
        gfx_rs.clone(),
        &GraphicsDesc {
            vs: &vs_dxil,
            ps: None,
            blend: BlendDesc::default(),
            sample_mask: u32::MAX,
            raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
            depth_stencil: DepthStencilState {
                depth_compare: MTLCompareFunction::Always,
                depth_write: true,
                ..DepthStencilState::default()
            },
            input_layout: Vec::new(),
            strip_cut: 0,
            topology_type: 3,
            rtv_formats: Vec::new(),
            dsv_format: D32_S8,
            sample_count: 1,
        },
        "g44_ps_uav_base",
    )
    .unwrap();
    assert!(
        pso_base.sm66_narrowed,
        "MSC+AIR reflection on SM 6.5/6.6 VS must set sm66_narrowed so sm66_gfx_exact counts active exactness"
    );
    // PS-UAV PSO (`vs_writes_uav = false`, `ps_writes_uav = true`, `sm66_writes_uav = true`):
    let pso_ps_uav = Arc::new(GraphicsPipeline {
        pso: pso_base.pso.clone(),
        rs: pso_base.rs.clone(),
        raster: pso_base.raster,
        blend: pso_base.blend,
        depth_stencil: pso_base.depth_stencil.clone(),
        depth_bounds: pso_base.depth_bounds,
        needs_draw_params: pso_base.needs_draw_params,
        topology_type: pso_base.topology_type,
        strip_cut: pso_base.strip_cut,
        rtv_formats: pso_base.rtv_formats.clone(),
        dsv_format: pso_base.dsv_format,
        sample_count: pso_base.sample_count,
        vertex_slots: pso_base.vertex_slots,
        vs: pso_base.vs.clone(),
        ps: pso_base.ps.clone(),
        gs_emu: None,
        tess_emu: None,
        pre_raster_reads: 0,
        vs_reads_cbv_or_ia: false,
        vs_writes_uav: false,
        ps_writes_uav: true,
        ps_read_classes: 0,
        ps_srv_uav_classes: 0,
        writes_uav: true,
        uses_uav: true,
        sm66_pre_raster_reads: 0,
        sm66_vs_reads_cbv_or_ia: false,
        sm66_writes_uav: true,
        sm66_narrowed: true,
        vs_access: pso_base.vs_access.clone(),
        ps_access: pso_base.ps_access.clone(),
        vs_hk_profile: pso_base.vs_hk_profile,
        ps_hk_profile: pso_base.ps_hk_profile,
        icb_desc: None,
        icb_pso: std::sync::OnceLock::new(),
    });

    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g44_ps_uav_cs").unwrap());

    let tex_srv = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_ps_uav = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_UNORDERED_ACCESS);
    let ds = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let uav_a = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_b = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex_srv, &tex_ps_uav, &ds, &uav_a, &uav_b, &rb] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g44 ps_uav heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex_srv), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rtv_srv = TargetView::rtv(&tex_srv, None).unwrap();
    let dsv = TargetView::dsv(&ds, None).unwrap();
    let tex_srv_id = &tex_srv as *const Resource as usize;
    let tex_ps_uav_id = &tex_ps_uav as *const Resource as usize;
    let uav_b_id = &uav_b as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g44 ps_uav queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut l = CommandList::new(ListType::Direct).unwrap();
    assert!(l.opts.opt_rp_keep_write_exact, "AW2_RP_KEEP_WRITE_EXACT must default ON in g44");
    assert!(l.opts.opt_sm66_cs_stage_exact, "AW2_SM66_CS_STAGE_EXACT must default ON in g44");

    let sm66_gfx0 = stats::c::sm66_gfx_exact.load(Relaxed);
    let mid_skip0 = stats::c::cs_uav_mid_skip.load(Relaxed);
    let mid_qb0 = stats::c::cs_mid_qb.load(Relaxed);

    l.begin(&alloc).unwrap();
    unsafe { l.encode_rtv_clear_pass(&rtv_srv, [7.0, 14.0, 21.0, 1.0]).unwrap() };
    l.barrier_list_with(&[Barrier::Transition(tex_srv_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);

    // PS-UAV render pass (`vs_writes_uav = false`, `ps_writes_uav = true`):
    l.gfx_root.set_root_signature(gfx_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
    l.set_render_targets(Vec::new(), Some(dsv));
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_ps_uav);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.5, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();

    assert!(stats::c::sm66_gfx_exact.load(Relaxed) > sm66_gfx0, "sm66_gfx_exact must increment on MSC-proven PSO");
    assert_eq!(l.bar.gfx_uav_gen, 1);
    assert_eq!(l.bar.gfx_uav_stage_gen[0], 0, "VS did not write UAV so gfx_uav_stage_gen[Vertex] must remain 0");
    assert!(l.bar.gfx_uav_stage_gen[STAGE_FRAGMENT] > 0, "PS wrote UAV so gfx_uav_stage_gen[Fragment] must advance");

    // Subsequent render pass waits on `PRE_RASTER` (`STATE_NON_PIXEL_SRV -> STATE_RENDER_TARGET`),
    // making `Fragment` dominate `PRE_RASTER`, then transition `tex_ps_uav` `UAV -> NON_PIXEL_SRV`:
    l.barrier_list_with(&[Barrier::Transition(tex_srv_id, STATE_NON_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
    unsafe { l.encode_rtv_clear_pass(&rtv_srv, [7.0, 14.0, 21.0, 1.0]).unwrap() };
    l.barrier_list_with(
        &[
            Barrier::Transition(tex_srv_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
            Barrier::Transition(tex_ps_uav_id, STATE_UAV, STATE_NON_PIXEL_SRV),
        ],
        true,
        false,
    );

    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_a.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso);
    l.dispatch(1, 1, 1).unwrap();

    assert_eq!(l.bar.enc_sync_after, MTLStages::Fragment, "compute open must fold dominated PRE_RASTER into Fragment");
    assert!(
        l.open_compute_disp_uav_synced(),
        "folded Fragment wait must satisfy gfx_uav_gen for PS-UAV pass under RP_KEEP_WRITE_EXACT"
    );

    // Mid-compute `Barrier::Uav(0)` MUST hit `cs_uav_mid_skip` (zero `cs_mid_qb`!)!
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    l.compute_root.set_descriptor(1, uav_b.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();

    assert!(stats::c::cs_uav_mid_skip.load(Relaxed) > mid_skip0, "cs_uav_mid_skip must fire after PS-UAV folded Fragment sync");
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed) - mid_qb0, 0, "no false mid-compute queue barrier may fire on Barrier::Uav(0)");

    l.barrier_list_with(&[Barrier::Transition(uav_b_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    l.copy_buffer(&rb, 0, &uav_b, 0, 64).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();
    let vals: Vec<f32> = buf_words(&rb, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&vals[..], &[7.0, 14.0, 21.0, 1.0]);
}

#[test]
fn g44_stage_a_ib_uav0_unbounded_exact_expands_past_32_uavs_and_splits_as_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::ComputePipeline;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
        RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let cs_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Uav,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g44_a_cs").unwrap());

    let tex = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    tex.register_ptr();
    let uav_bufs: Vec<Resource> = (0..76)
        .map(|_| test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS))
        .collect();
    for b in &uav_bufs {
        b.register_ptr();
    }
    let rb0 = test_buffer(256, heap_type::READBACK, 0);
    let rb_last = test_buffer(256, heap_type::READBACK, 0);
    rb0.register_ptr();
    rb_last.register_ptr();

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g44_a heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rtv = TargetView::rtv(&tex, None).unwrap();
    let tex_id = &tex as *const Resource as usize;
    let q = Queue::new(ListType::Direct, "g44_a queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    // Execute 76 disjoint UAV dispatches (`> 32`, matching `L1#0`'s 76 `transformCS` dispatches!)
    // interleaved with `Barrier::Uav(0)`, then also verify positive WAW hazard on `uav_bufs[0]`
    // STILL fires `enc_barrier`!
    let run_case = |unbounded_on: bool| -> (u64, u64, u64, Vec<f32>, Vec<f32>) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_ib_uav0_unbounded_exact = unbounded_on;
        let eb0 = stats::c::enc_barrier.load(Relaxed);
        let ex0 = stats::c::ib_uav0_exact.load(Relaxed);

        l.begin(&alloc).unwrap();
        unsafe { l.encode_rtv_clear_pass(&rtv, [11.0, 22.0, 33.0, 1.0]).unwrap() };
        l.barrier_list_with(&[Barrier::Transition(tex_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);

        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.set_compute_pipeline(cs_pso.clone());

        for b in &uav_bufs {
            l.compute_root.set_descriptor(1, b.gpu_va).unwrap();
            l.dispatch(1, 1, 1).unwrap();
            l.barrier_list_with(&[Barrier::Uav(0)], true, false);
        }
        let disjoint_eb = stats::c::enc_barrier.load(Relaxed) - eb0;
        let ex_delta = stats::c::ib_uav0_exact.load(Relaxed) - ex0;

        // Positive safety guardrail: re-dispatching onto ALREADY-WRITTEN `uav_bufs.last()`
        // after `Barrier::Uav(0)` MUST fire `enc_barrier`!
        let eb_waw0 = stats::c::enc_barrier.load(Relaxed);
        l.compute_root.set_descriptor(1, uav_bufs.last().unwrap().gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let waw_eb = stats::c::enc_barrier.load(Relaxed) - eb_waw0;

        let id0 = &uav_bufs[0] as *const Resource as usize;
        let id_last = uav_bufs.last().unwrap() as *const Resource as usize;
        l.barrier_list_with(
            &[
                Barrier::Transition(id0, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(id_last, STATE_UAV, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb0, 0, &uav_bufs[0], 0, 64).unwrap();
        l.copy_buffer(&rb_last, 0, uav_bufs.last().unwrap(), 0, 64).unwrap();
        l.close().unwrap();

        q.submit(&[&l.cb]);
        q.wait_idle();
        alloc.reset();
        let v0: Vec<f32> = buf_words(&rb0, 4).into_iter().map(f32::from_bits).collect();
        let vl: Vec<f32> = buf_words(&rb_last, 4).into_iter().map(f32::from_bits).collect();
        let _ = &uav_bufs;
        (disjoint_eb, waw_eb, ex_delta, v0, vl)
    };

    let (disjoint_off, waw_off, ex_off, ref0, refl) = run_case(false);
    let (disjoint_on, waw_on, ex_on, opt0, optl) = run_case(true);
    assert_eq!(disjoint_off, 2, "legacy 32-UAV cap must collapse to wildcard enc_barriers every >32 dispatches");
    assert_eq!(ex_off, 0);
    assert_eq!(disjoint_on, 0, "g44_A must expand all 76 disjoint UAVs with 0 false enc_barriers");
    assert_eq!(waw_off, 1);
    assert_eq!(waw_on, 1, "g44_A must still emit enc_barrier on true WAW UAV hazard");
    assert!(ex_on >= 43, "ib_uav0_exact must count >32 UAV expansions (got {ex_on})");
    assert_eq!(opt0, ref0);
    assert_eq!(optl, refl);
    assert_eq!(&optl[..], &[11.0, 22.0, 33.0, 1.0]);
}
