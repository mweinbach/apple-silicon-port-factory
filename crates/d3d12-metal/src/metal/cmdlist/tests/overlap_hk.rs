//! PS-to-CS overlap, handle-kind disambiguation, placed-heap tracking, g49–g51, and mipgen GPU tests.

use super::*;

#[test]
fn g49_env_defaults_are_off_and_exe_zero_winners_stay_on() {
    if crate::metal::device().is_none() {
        return;
    }
    let l = CommandList::new(ListType::Direct).unwrap();
    assert!(l.opts.opt_gfx_desc_table_exact, "AW2_GFX_DESC_TABLE_EXACT must remain Default-ON");
    assert!(l.opts.opt_cs_to_ps_overlap_v2, "AW2_CS_TO_PS_OVERLAP_V2 must remain Default-ON");
    assert!(!l.opts.opt_ps_to_cs_overlap, "AW2_PS_TO_CS_OVERLAP must default OFF");
    assert!(!l.opts.opt_gfx_to_ps_overlap_v2, "AW2_GFX_TO_PS_OVERLAP_V2 must default OFF");
    assert!(!l.opts.opt_clear_zero_qb_coalesce, "AW2_CLEAR_ZERO_QB_COALESCE must default OFF");
    assert!(!l.opts.opt_cs_intra_barrier_v2, "AW2_CS_INTRA_BARRIER_V2 must default OFF");
}

#[test]
fn g49_a_ps_to_cs_overlap_defers_untouched_rt_and_syncs_consumer_plus_passthrough() {
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF, READS_TEX};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let cs_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 2,
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
    let mut pso_raw = ComputePipeline::new(rs.clone(), &cs_dxil, "g49_a_cs").unwrap();
    pso_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: 0,
        unbounded_srv_classes: 0,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: Some(vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 0 },
            ComputeBind::TableEntry { param: 0, byte_offset: 24 },
        ]),
        bounded_read_slots: vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 0 },
            ComputeBind::TableEntry { param: 0, byte_offset: 24 },
        ],
    };
    let pso = Arc::new(pso_raw);

    let tex_a = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_b = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let b_srv = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_disjoint = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_out_a = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_disjoint = test_buffer(512, heap_type::READBACK, 0);
    let rb_a = test_buffer(512, heap_type::READBACK, 0);
    let rb_pt = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &b_srv, &b_disjoint, &b_out_a, &rb_disjoint, &rb_a, &rb_pt] {
        r.register_ptr();
    }
    let heap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g49_a heap",
    )
    .unwrap();
    unsafe {
        // Slots [0, 1] = [tex_b, b_srv]; Slots [2, 3] = [tex_a, b_srv]
        let eb = crate::metal::views::srv_entry(Some(&tex_b), None).unwrap();
        std::ptr::copy_nonoverlapping(eb.as_ptr(), heap.cpu_base as *mut u8, 24);
        let p1 = (heap.cpu_base + 24) as *mut u64;
        p1.write_unaligned(b_srv.gpu_va);
        p1.add(1).write_unaligned(0);
        p1.add(2).write_unaligned(64);

        let ea = crate::metal::views::srv_entry(Some(&tex_a), None).unwrap();
        std::ptr::copy_nonoverlapping(ea.as_ptr(), (heap.cpu_base + 48) as *mut u8, 24);
        let p3 = (heap.cpu_base + 72) as *mut u64;
        p3.write_unaligned(b_srv.gpu_va);
        p3.add(1).write_unaligned(0);
        p3.add(2).write_unaligned(64);
    }
    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let b_dis_id = &b_disjoint as *const Resource as usize;
    let b_out_a_id = &b_out_a as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g49_a queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    // Pre-populate tex_b = [5, 10, 15, 1]:
    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_b, [5.0, 10.0, 15.0, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_b_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_case = |enable_a: bool| -> (u64, bool, u64, [f32; 4], [f32; 4], [f32; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_ps_to_cs_overlap = enable_a;
        let p0 = stats::c::ps_to_cs_overlap.load(Relaxed);
        let mq0 = stats::c::cs_mid_qb.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        // Steady-state Direct list with non-clear graphics passes (`!track_exact_clear`):
        l.bar.in_list_gfx_non_clear = true;
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(rs.clone()).unwrap();
        l.set_compute_pipeline(pso.clone());

        // Anchor pre-list compute stages first so subsequent compute encoders use `!list_unwaited_cs`:
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_disjoint.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        // Render Pass writes tex_a = [2, 4, 6, 1], then transitions tex_a RENDER_TARGET -> NON_PIXEL_SRV:
        unsafe { l.encode_rtv_clear_pass(&rtv_a, [2.0, 4.0, 6.0, 1.0]).unwrap() };
        l.end_encoder();
        l.barrier_list_with(
            &[Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)],
            true,
            false,
        );

        // Compute Encoder #2 opens with Dispatch 1 reading ONLY [tex_b, b_srv], disjoint from tex_a!
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_disjoint.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let open_skipped_frag = !l.bar.enc_sync_after.contains(MTLStages::Fragment);

        // Dispatch 2 in the SAME Compute Encoder switches table 0 to [tex_a, b_srv] -> MUST emit mid-QB(Fragment, Dispatch)!
        l.compute_root.set_table(0, heap.gpu_base + 48).unwrap();
        l.compute_root.set_descriptor(1, b_out_a.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        // Now test cs_disp_passthrough (including R2R NON_PIXEL_SRV -> PIXEL_SRV -> COPY_SOURCE):
        l.barrier_list_with(&[Barrier::Transition(tex_a_id, STATE_NON_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
        unsafe { l.encode_rtv_clear_pass(&rtv_a, [7.0, 8.0, 9.0, 1.0]).unwrap() };
        l.end_encoder();
        l.barrier_list_with(&[Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_disjoint.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();
        // R2R NON_PIXEL_SRV -> PIXEL_SRV must NOT be elided when cs_disp_passthrough holds Fragment!
        l.barrier_list_with(&[Barrier::Transition(tex_a_id, STATE_NON_PIXEL_SRV, STATE_PIXEL_SRV)], true, false);
        assert!(
            !enable_a || l.bar.split_pend.frag_after.contains(MTLStages::Fragment),
            "cs_disp_passthrough must prevent R2R elision on NON_PIXEL_SRV -> PIXEL_SRV"
        );
        // And NON_PIXEL_SRV -> COPY_SOURCE must propagate Fragment into blit_after!
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_a_id, STATE_NON_PIXEL_SRV, STATE_COPY_SOURCE),
                Barrier::Transition(b_dis_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(b_out_a_id, STATE_UAV, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        assert!(
            !enable_a || l.bar.split_pend.blit_after.contains(MTLStages::Fragment),
            "cs_disp_passthrough must propagate Fragment across NON_PIXEL_SRV -> COPY_SOURCE when Dispatch skipped Fragment"
        );
        let fp = PlacedFootprint { offset: 0, format: 2, width: 4, height: 1, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_pt, fp), [0, 0, 0], CopyLoc::Subresource(&tex_a, 0), None).unwrap();
        l.copy_buffer(&rb_disjoint, 0, &b_disjoint, 0, 16).unwrap();
        l.copy_buffer(&rb_a, 0, &b_out_a, 0, 16).unwrap();
        l.close().unwrap();

        let p_d = stats::c::ps_to_cs_overlap.load(Relaxed) - p0;
        let mq_d = stats::c::cs_mid_qb.load(Relaxed) - mq0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g49_A must produce 0 bad timestamps");
        alloc.reset();

        let vd: Vec<f32> = buf_words(&rb_disjoint, 4).into_iter().map(f32::from_bits).collect();
        let va: Vec<f32> = buf_words(&rb_a, 4).into_iter().map(f32::from_bits).collect();
        let vp: Vec<f32> = buf_words(&rb_pt, 4).into_iter().map(f32::from_bits).collect();
        (p_d, open_skipped_frag, mq_d, [vd[0], vd[1], vd[2], vd[3]], [va[0], va[1], va[2], va[3]], [vp[0], vp[1], vp[2], vp[3]])
    };

    let (p_off, skip_off, mq_off, vd_off, va_off, vp_off) = run_case(false);
    let (p_on, skip_on, mq_on, vd_on, va_on, vp_on) = run_case(true);
    assert_eq!(p_off, 0);
    assert!(!skip_off, "baseline blocked disjoint Compute Encoder on Fragment");
    assert!(skip_on, "g49_A must let disjoint opening Dispatch skip Fragment queue barrier");
    assert!(p_on >= 2, "ps_to_cs_overlap must increment on both disjoint compute encoders: got {p_on}");
    assert!(mq_on > mq_off, "g49_A must emit mid-compute queue barrier when Dispatch 2 binds tex_a");
    assert_eq!(vd_on, vd_off);
    assert_eq!(va_on, va_off);
    assert_eq!(vp_on, vp_off);
    assert_eq!(vd_on, [5.0, 10.0, 15.0, 1.0]);
    assert_eq!(va_on, [2.0, 4.0, 6.0, 1.0]);
    assert_eq!(vp_on, [7.0, 8.0, 9.0, 1.0]);
}

#[test]
fn g49_b_gfx_to_ps_overlap_v2_list_start_skips_readonly_srv_and_syncs_frag_writable_tex() {
    use crate::metal::pipeline::{
        ComputeAccess, ComputeBind, GraphicsDesc, GraphicsPipeline, READS_TEX,
    };
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let ps_cb0_dxbc = cb0_color_ps_dxbc();
    let vs_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Cbv,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 5, // PIXEL -> pre-raster reads = 0 on Draw 0
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 1, // VERTEX
            },
        ],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let desc = GraphicsDesc {
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
    let mut pso0_raw = GraphicsPipeline::new(rs.clone(), &desc, "g49_b_draw0").unwrap();
    pso0_raw.pre_raster_reads = 0;
    pso0_raw.sm66_pre_raster_reads = 0;
    pso0_raw.vs_reads_cbv_or_ia = false;
    pso0_raw.sm66_vs_reads_cbv_or_ia = false;
    let pso_draw0 = Arc::new(pso0_raw);

    let mut pso1_raw = GraphicsPipeline::new(rs.clone(), &desc, "g49_b_draw1_tex").unwrap();
    pso1_raw.pre_raster_reads = READS_TEX;
    pso1_raw.sm66_pre_raster_reads = READS_TEX;
    pso1_raw.vs_reads_cbv_or_ia = false;
    pso1_raw.sm66_vs_reads_cbv_or_ia = false;
    pso1_raw.vs_access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: 0,
        read_classes: READS_TEX,
        msc_exact_classes: READS_TEX,
        uav_srv_classes: READS_TEX,
        unbounded_srv_classes: 0,
        uav_slots: Some(Vec::new()),
        read_slots: Some(vec![ComputeBind::TableEntry { param: 1, byte_offset: 0 }]),
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 1, byte_offset: 0 }],
    };
    let pso_vs_tex = Arc::new(pso1_raw);

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let tex_ro = test_texture(4, 4, RGBA8_UNORM, 0); // Read-only SRV texture (Flags == 0)
    let tex_rt_cap = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_resolve_dst = test_texture(4, 4, RGBA8_UNORM, 0);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &tex_ro, &tex_rt_cap, &tex_resolve_dst, &rt, &rb] {
        r.register_ptr();
    }
    note_frag_resolve_dst(&tex_resolve_dst as *const Resource as usize);
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let heap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g49_b heap",
    )
    .unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex_ro), None).unwrap();
        let e1 = crate::metal::views::srv_entry(Some(&tex_rt_cap), None).unwrap();
        let e2 = crate::metal::views::srv_entry(Some(&tex_resolve_dst), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        std::ptr::copy_nonoverlapping(e1.as_ptr(), (heap.cpu_base + 24) as *mut u8, 24);
        std::ptr::copy_nonoverlapping(e2.as_ptr(), (heap.cpu_base + 48) as *mut u8, 24);
    }
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let rt_id = &rt as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g49_b queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |enable_b: bool, legacy_rsplit: bool, slot_second_draw: u64| -> (u64, bool, bool, u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_gfx_to_ps_overlap_v2 = enable_b;
        l.opts.opt_list_start_render_split = legacy_rsplit;
        l.opts.test_force_frag_buf_write = Some(!legacy_rsplit);
        let b0 = stats::c::gfx_to_ps_overlap_v2.load(Relaxed);
        let hit0 = stats::c::dv_hit.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_table(1, heap.gpu_base).unwrap();
        l.gfx.topology = 4;
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
        l.set_render_targets(vec![rtv], None);

        // Draw 0: READS_TEX == 0 -> opens list-start render pass with vtx_open = COMPUTE_STAGES | PRE_RASTER
        l.set_graphics_pipeline(pso_draw0.clone());
        l.draw(3, 1, 0, 0).unwrap();

        // Draw 1: VS binds read-only SRV texture `tex_ro` (Flags == 0):
        l.set_graphics_pipeline(pso_vs_tex.clone());
        l.gfx_root.set_table(1, heap.gpu_base).unwrap();
        l.draw(3, 1, 0, 0).unwrap();
        let draw1_skipped_all = !l.bar.enc_cur_sync[0].pre_list_all;

        // Draw 2: VS binds `slot_second_draw` (`tex_rt_cap` or `tex_resolve_dst`) -> MUST emit `All -> PRE_RASTER`!
        l.gfx_root.set_table(1, heap.gpu_base + slot_second_draw).unwrap();
        l.draw(3, 1, 0, 0).unwrap();
        let draw2_synced_all = l.bar.enc_cur_sync[0].pre_list_all;

        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();

        let b_d = stats::c::gfx_to_ps_overlap_v2.load(Relaxed) - b0;
        let hit_d = stats::c::dv_hit.load(Relaxed) - hit0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g49_B must produce 0 bad timestamps");
        alloc.reset();
        (b_d, draw1_skipped_all, draw2_synced_all, hit_d, buf_words(&rb, 1)[0])
    };

    let (b_off, skip1_off, sync2_off, hit_off, px_off) = run_case(false, false, 24);
    let (b_leg, skip1_leg, sync2_leg, hit_leg, px_leg) = run_case(false, true, 24);
    let (b_on, skip1_on, sync2_on, hit_on, px_on) = run_case(true, false, 24);
    let (b_res, skip1_res, sync2_res, hit_res, px_res) = run_case(true, false, 48);
    assert_eq!(b_off, 0);
    assert!(!skip1_off && sync2_off && hit_off == 0, "exe-zero baseline pays All -> PRE_RASTER at pass open");
    assert_eq!(b_leg, 0);
    assert!(!skip1_leg && sync2_leg && hit_leg == 1, "legacy list_start_render_split poisoned vtx_defer[0] and fired dv_hit on Draw 1");
    assert!(skip1_on, "g49_B must split ListStart even under sticky frag_buf_write and skip All -> PRE_RASTER on Draw 1 reading pure read-only SRV texture");
    assert!(sync2_on, "g49_B must emit All -> PRE_RASTER on Draw 2 reading ALLOW_RENDER_TARGET texture");
    assert!(b_on >= 2, "gfx_to_ps_overlap_v2 must increment on ListStart split + Draw 1 skip: got {b_on}");
    assert_eq!(hit_on, 1);
    assert!(skip1_res && sync2_res && b_res >= 2 && hit_res == 1, "note_frag_resolve_dst texture must also trigger All -> PRE_RASTER");
    assert_eq!((px_leg, px_on, px_res), (px_off, px_off, px_off));
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g49_c_clear_zero_qb_coalesce_isolates_untouched_rt_from_sibling_transition() {
    use std::sync::atomic::Ordering::Relaxed;
    let Some((rs, pso)) = build_g48_vs_srv_ps_cb0_pso("g49_c_gfx") else { return };

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let rt_anchor = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_c = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb_a = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_b = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_c = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_vs, &rt_anchor, &rt_a, &rt_b, &rt_c, &rb_a, &rb_b, &rb_c] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let rtv_anchor = TargetView::rtv(&rt_anchor, None).unwrap();
    let rtv_a = TargetView::rtv(&rt_a, None).unwrap();
    let rtv_b = TargetView::rtv(&rt_b, None).unwrap();
    let rtv_c = TargetView::rtv(&rt_c, None).unwrap();
    let anchor_id = &rt_anchor as *const Resource as usize;
    let a_id = &rt_a as *const Resource as usize;
    let b_id = &rt_b as *const Resource as usize;
    let c_id = &rt_c as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g49_c queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |enable_c: bool| -> (u64, u64, bool, u32, u32, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_clear_zero_qb_coalesce = enable_c;
        let c0 = stats::c::clear_zero_qb_coalesce.load(Relaxed);
        let qf0 = stats::c::q_barrier_frag.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // Render Pass #0 anchors pre-list stages AND advances stage_gen[STAGE_FRAGMENT] to 1:
        l.set_render_targets(vec![rtv_anchor], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        // Transition rt_anchor -> PIXEL_SRV and rt_a PIXEL_SRV -> RENDER_TARGET (producing Fragment -> Fragment on rt_a),
        // while rt_b and rt_c were ALREADY in RENDER_TARGET with ZERO pending transitions!
        l.barrier_list_with(
            &[
                Barrier::Transition(anchor_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
                Barrier::Transition(a_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET),
            ],
            true,
            false,
        );
        unsafe {
            l.clear_rtv(&rtv_b, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
            l.clear_rtv(&rtv_c, [0.0, 0.0, 1.0, 1.0], &[]).unwrap();
        }
        // Flush clears on rt_b & rt_c BEFORE touching rt_a:
        let qf_before_clear = stats::c::q_barrier_frag.load(Relaxed);
        l.flush_clears(&stats::c::cf_close, |p| [b_id, c_id].contains(&p.view.resource)).unwrap();
        let clear_qf = stats::c::q_barrier_frag.load(Relaxed) - qf_before_clear;
        // rt_a's Fragment obligation MUST remain intact in split_pend.frag_after when enable_c is true!
        let rt_a_still_pending = l.bar.split_pend.frag_after.contains(MTLStages::Fragment);

        // Now draw into rt_a (consuming rt_a's Fragment obligation):
        l.set_render_targets(vec![rtv_a], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        l.barrier_list_with(
            &[
                Barrier::Transition(a_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
                Barrier::Transition(b_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
                Barrier::Transition(c_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_a, fp), [0, 0, 0], CopyLoc::Subresource(&rt_a, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_b, fp), [0, 0, 0], CopyLoc::Subresource(&rt_b, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_c, fp), [0, 0, 0], CopyLoc::Subresource(&rt_c, 0), None).unwrap();
        l.close().unwrap();

        let c_d = stats::c::clear_zero_qb_coalesce.load(Relaxed) - c0;
        let _ = qf0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g49_C must produce 0 bad timestamps");
        alloc.reset();
        (c_d, clear_qf, rt_a_still_pending, buf_words(&rb_a, 1)[0], buf_words(&rb_b, 1)[0], buf_words(&rb_c, 1)[0])
    };

    let (c_off, qf_off, pend_off, pa_off, pb_off, pc_off) = run_case(false);
    let (c_on, qf_on, pend_on, pa_on, pb_on, pc_on) = run_case(true);
    assert_eq!(c_off, 0);
    assert_eq!(qf_off, 1, "baseline forced untouched rt_b/rt_c clear pass to emit Fragment queue barrier");
    assert!(!pend_off, "baseline drained split_pend.frag_after on rt_b/rt_c clear pass");
    assert_eq!(qf_on, 0, "g49_C must emit zero queue barrier on coalesced clear pass for untouched rt_b/rt_c");
    assert!(pend_on, "g49_C must preserve rt_a's pending Fragment obligation in split_pend.frag_after");
    assert!(c_on >= 2, "clear_zero_qb_coalesce must increment on isolated clear + coalesced pass: got {c_on}");
    assert_eq!((pa_on, pb_on, pc_on), (pa_off, pb_off, pc_off));
    assert_eq!(pa_on, 0xff00_ff00);
    assert_eq!(pb_on, 0xff00_00ff);
    assert_eq!(pc_on, 0xffff_0000);
}

#[test]
fn g49_d_cs_intra_barrier_v2_class_touch_and_pure_war_uav_write_only() {
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF, READS_TEX};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let cs_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 2,
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
    let mut pso_raw = ComputePipeline::new(rs.clone(), &cs_dxil, "g49_d_cs").unwrap();
    pso_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF | READS_TEX,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: READS_BUF,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 0 },
            ComputeBind::TableEntry { param: 0, byte_offset: 24 },
        ],
    };
    let pso_unb_buf = Arc::new(ComputePipeline {
        pso: pso_raw.pso.clone(),
        rs: pso_raw.rs.clone(),
        threadgroup: pso_raw.threadgroup,
        shader: pso_raw.shader.clone(),
        hk_profile: pso_raw.hk_profile,
        access: ComputeAccess {
            touches_textures: true,
            heap_indexed_or_unknown: false,
            msc_refined: true,
            uav_classes: READS_BUF,
            read_classes: READS_BUF | READS_TEX,
            msc_exact_classes: READS_BUF | READS_TEX,
            uav_srv_classes: READS_BUF,
            unbounded_srv_classes: READS_BUF,
            uav_slots: None,
            read_slots: None,
            bounded_read_slots: vec![
                ComputeBind::TableEntry { param: 0, byte_offset: 0 },
                ComputeBind::TableEntry { param: 0, byte_offset: 24 },
            ],
        },
    });
    let pso = Arc::new(pso_raw);

    let tex = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_other = test_texture(4, 1, 2, resource_flags::ALLOW_UNORDERED_ACCESS);
    let up_other = test_buffer(512, heap_type::UPLOAD, 0);
    let b_srv = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b0 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b1 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb0 = test_buffer(512, heap_type::READBACK, 0);
    let rb1 = test_buffer(512, heap_type::READBACK, 0);
    let rb_srv = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex, &tex_other, &up_other, &b_srv, &b0, &b1, &rb0, &rb1, &rb_srv] {
        r.register_ptr();
    }
    let heap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g49_d heap",
    )
    .unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        let p1 = (heap.cpu_base + 24) as *mut u64;
        p1.write_unaligned(b_srv.gpu_va);
        p1.add(1).write_unaligned(0);
        p1.add(2).write_unaligned(64);
    }
    let rtv_tex = TargetView::rtv(&tex, None).unwrap();
    let tex_id = &tex as *const Resource as usize;
    let tex_other_id = &tex_other as *const Resource as usize;
    let b_srv_id = &b_srv as *const Resource as usize;
    let b0_id = &b0 as *const Resource as usize;
    let b1_id = &b1 as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g49_d queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_tex, [4.0, 8.0, 12.0, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_case = |enable_d: bool| -> (u64, u64, u64, u64, [f32; 4], [f32; 4], [f32; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_cs_intra_barrier_v2 = enable_d;
        let d0 = stats::c::cs_intra_barrier_v2.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.set_compute_pipeline(pso.clone());

        // Dispatch 1 reads `[tex, b_srv]` and writes `b0`:
        l.compute_root.set_descriptor(1, b0.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();

        // 1) Pure WAR `Transition(b_srv, NON_PIXEL_SRV -> UAV)`:
        let eb_war_skip0 = stats::c::enc_barrier.load(Relaxed);
        l.barrier_list_with(&[Barrier::Transition(b_srv_id, STATE_NON_PIXEL_SRV, STATE_UAV)], true, false);
        l.compute_root.set_descriptor(1, b1.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let eb_war_skip = stats::c::enc_barrier.load(Relaxed) - eb_war_skip0;

        let eb_war_fire0 = stats::c::enc_barrier.load(Relaxed);
        l.compute_root.set_descriptor(1, b_srv.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let eb_war_fire = stats::c::enc_barrier.load(Relaxed) - eb_war_fire0;

        // 2) Dispatch 4 uses `pso_unb_buf` (`uav_slots = None`, `uav_classes = READS_BUF`):
        l.set_compute_pipeline(pso_unb_buf.clone());
        l.compute_root.set_descriptor(1, b1.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let eb_tex0 = stats::c::enc_barrier.load(Relaxed);
        l.barrier_list_with(&[Barrier::Transition(tex_other_id, STATE_UAV, STATE_COPY_DEST)], true, false);
        let fp_other = PlacedFootprint { offset: 0, format: 2, width: 4, height: 1, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Subresource(&tex_other, 0), [0, 0, 0], CopyLoc::Footprint(&up_other, fp_other), None).unwrap();
        let eb_tex = stats::c::enc_barrier.load(Relaxed) - eb_tex0;

        l.barrier_list_with(
            &[
                Barrier::Transition(b0_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(b1_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(b_srv_id, STATE_UAV, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb0, 0, &b0, 0, 16).unwrap();
        l.copy_buffer(&rb1, 0, &b1, 0, 16).unwrap();
        l.copy_buffer(&rb_srv, 0, &b_srv, 0, 16).unwrap();
        l.close().unwrap();

        let d_delta = stats::c::cs_intra_barrier_v2.load(Relaxed) - d0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g49_D must produce 0 bad timestamps");
        alloc.reset();

        let v0: Vec<f32> = buf_words(&rb0, 4).into_iter().map(f32::from_bits).collect();
        let v1: Vec<f32> = buf_words(&rb1, 4).into_iter().map(f32::from_bits).collect();
        let vs: Vec<f32> = buf_words(&rb_srv, 4).into_iter().map(f32::from_bits).collect();
        (d_delta, eb_tex, eb_war_skip, eb_war_fire, [v0[0], v0[1], v0[2], v0[3]], [v1[0], v1[1], v1[2], v1[3]], [vs[0], vs[1], vs[2], vs[3]])
    };

    let (d_off, eb_tex_off, eb_skip_off, eb_fire_off, r0_off, r1_off, rs_off) = run_case(false);
    let (d_on, eb_tex_on, eb_skip_on, eb_fire_on, r0_on, r1_on, rs_on) = run_case(true);
    assert_eq!(d_off, 0);
    assert_eq!(eb_tex_off, 1, "baseline emitted false-positive intra-CS barrier on untouched texture due to unbounded buffer SRV");
    assert_eq!(eb_skip_off, 1, "baseline emitted intra-CS barrier on SRV-only reader after pure WAR NON_PIXEL_SRV->UAV");
    assert_eq!(eb_fire_off, 0);
    assert_eq!(eb_tex_on, 0, "g49_D must skip intra-CS barrier on untouched texture via disp_read_cls[0]");
    assert_eq!(eb_skip_on, 0, "g49_D OblKind::UavWriteOnly must skip intra-CS barrier when next dispatch only writes disjoint UAV b1");
    assert_eq!(eb_fire_on, 1, "g49_D OblKind::UavWriteOnly must fire intra-CS barrier when Dispatch 4 binds b_srv as UAV");
    assert!(d_on >= 2, "cs_intra_barrier_v2 must increment: got {d_on}");
    assert_eq!((r0_on, r1_on, rs_on), (r0_off, r1_off, rs_off));
    assert_eq!(r0_on, [4.0, 8.0, 12.0, 1.0]);
    assert_eq!(r1_on, [4.0, 8.0, 12.0, 1.0]);
    assert_eq!(rs_on, [4.0, 8.0, 12.0, 1.0]);
}

#[test]
fn g49_combo_all_four_with_default_on_cs_to_ps_v2_and_gfx_desc_exact() {
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF, READS_TEX};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    let Some((gfx_rs, gfx_pso)) = build_g48_vs_srv_ps_cb0_pso("g49_combo_gfx") else { return };
    let cs_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
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
                    num_descriptors: 2,
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
    let mut cs_raw = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g49_combo_cs").unwrap();
    cs_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: READS_BUF,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 0 },
            ComputeBind::TableEntry { param: 0, byte_offset: 24 },
        ],
    };
    let cs_pso = Arc::new(cs_raw);

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let tex_a = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_b = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let rt_draw = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_zero_qb = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let b_srv = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_dis = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_out = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_out = test_buffer(512, heap_type::READBACK, 0);
    let rb_draw = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_clear = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_vs, &tex_a, &tex_b, &rt_draw, &rt_zero_qb, &b_srv, &b_dis, &b_out, &rb_out, &rb_draw, &rb_clear] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let heap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g49_combo heap",
    )
    .unwrap();
    unsafe {
        let eb = crate::metal::views::srv_entry(Some(&tex_b), None).unwrap();
        std::ptr::copy_nonoverlapping(eb.as_ptr(), heap.cpu_base as *mut u8, 24);
        let p1 = (heap.cpu_base + 24) as *mut u64;
        p1.write_unaligned(b_srv.gpu_va);
        p1.add(1).write_unaligned(0);
        p1.add(2).write_unaligned(64);

        let ea = crate::metal::views::srv_entry(Some(&tex_a), None).unwrap();
        std::ptr::copy_nonoverlapping(ea.as_ptr(), (heap.cpu_base + 48) as *mut u8, 24);
        let p3 = (heap.cpu_base + 72) as *mut u64;
        p3.write_unaligned(b_srv.gpu_va);
        p3.add(1).write_unaligned(0);
        p3.add(2).write_unaligned(64);
    }
    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let rtv_draw = TargetView::rtv(&rt_draw, None).unwrap();
    let rtv_zero = TargetView::rtv(&rt_zero_qb, None).unwrap();
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let rt_draw_id = &rt_draw as *const Resource as usize;
    let rt_zero_id = &rt_zero_qb as *const Resource as usize;
    let b_srv_id = &b_srv as *const Resource as usize;
    let b_out_id = &b_out as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g49_combo queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_b, [6.0, 7.0, 8.0, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_b_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_combo = |on: bool| -> ([f32; 4], u32, u32, u64) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        assert!(l.opts.opt_cs_to_ps_overlap_v2 && l.opts.opt_gfx_desc_table_exact);
        l.opts.opt_ps_to_cs_overlap = on;
        l.opts.opt_gfx_to_ps_overlap_v2 = on;
        l.opts.opt_clear_zero_qb_coalesce = on;
        l.opts.opt_cs_intra_barrier_v2 = on;
        let s0 = stats::c::ps_to_cs_overlap.load(Relaxed)
            + stats::c::clear_zero_qb_coalesce.load(Relaxed)
            + stats::c::cs_intra_barrier_v2.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.set_compute_pipeline(cs_pso.clone());
        l.gfx_root.set_root_signature(gfx_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(gfx_pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // Anchor Render draw (exercises g49_B ListStart split + sets in_list_gfx_non_clear):
        l.set_render_targets(vec![rtv_draw], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        l.barrier_list_with(&[Barrier::Transition(rt_draw_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV)], true, false);

        // Anchor CS:
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        unsafe { l.encode_rtv_clear_pass(&rtv_a, [1.5, 2.5, 3.5, 1.0]).unwrap() };
        l.end_encoder();
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
                Barrier::Transition(rt_draw_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET),
            ],
            true,
            false,
        );
        // Zero-QB clear on untouched rt_zero_qb while rt_draw has pending Fragment obligation:
        unsafe { l.clear_rtv(&rtv_zero, [0.0, 0.0, 1.0, 1.0], &[]).unwrap() };
        l.flush_clears(&stats::c::cf_close, |p| p.view.resource == rt_zero_id).unwrap();

        // Disjoint CS dispatch (g49_A) + pure WAR transition (g49_D) + consuming CS dispatch:
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.barrier_list_with(&[Barrier::Transition(b_srv_id, STATE_NON_PIXEL_SRV, STATE_UAV)], true, false);
        l.compute_root.set_table(0, heap.gpu_base + 48).unwrap();
        l.compute_root.set_descriptor(1, b_out.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        // Render draw into rt_draw:
        l.set_render_targets(vec![rtv_draw], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        l.barrier_list_with(
            &[
                Barrier::Transition(b_out_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(rt_draw_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
                Barrier::Transition(rt_zero_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb_out, 0, &b_out, 0, 16).unwrap();
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_draw, fp), [0, 0, 0], CopyLoc::Subresource(&rt_draw, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_clear, fp), [0, 0, 0], CopyLoc::Subresource(&rt_zero_qb, 0), None).unwrap();
        l.close().unwrap();

        let s_d = (stats::c::ps_to_cs_overlap.load(Relaxed)
            + stats::c::clear_zero_qb_coalesce.load(Relaxed)
            + stats::c::cs_intra_barrier_v2.load(Relaxed))
            - s0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g49 combo must produce 0 bad timestamps");
        alloc.reset();

        let vo: Vec<f32> = buf_words(&rb_out, 4).into_iter().map(f32::from_bits).collect();
        ([vo[0], vo[1], vo[2], vo[3]], buf_words(&rb_draw, 1)[0], buf_words(&rb_clear, 1)[0], s_d)
    };

    let (vo_off, pd_off, pc_off, sd_off) = run_combo(false);
    let (vo_on, pd_on, pc_on, sd_on) = run_combo(true);
    assert_eq!(sd_off, 0);
    assert!(sd_on >= 3, "g49 combo must fire multiple g49 levers together: got {sd_on}");
    assert_eq!(vo_on, vo_off);
    assert_eq!(pd_on, pd_off);
    assert_eq!(pc_on, pc_off);
    assert_eq!(vo_on, [1.5, 2.5, 3.5, 1.0]);
    assert_eq!(pd_on, 0xff00_ff00);
    assert_eq!(pc_on, 0xffff_0000);
}

#[test]
fn g50_knobs_default_off_for_seq_ab() {
    let unset = [
        "AW2_SM66_HANDLE_KIND_EXACT",
        "AW2_PS_TO_CS_OVERLAP_V2",
        "AW2_GFX_OVERLAP_CAPPED",
        "AW2_CS_FAST_BIND_EXACT",
    ]
    .iter()
    .all(|k| std::env::var_os(k).is_none());
    if !unset {
        return;
    }
    assert!(!opt_sm66_handle_kind_exact_env());
    assert!(!opt_ps_to_cs_overlap_v2_env());
    assert!(!opt_gfx_overlap_capped_env());
    assert!(!opt_cs_fast_bind_exact_env());
    if crate::metal::device().is_some() {
        let l = CommandList::new(ListType::Direct).unwrap();
        assert!(!l.opts.opt_sm66_handle_kind_exact, "AW2_SM66_HANDLE_KIND_EXACT must default OFF");
        assert!(!l.opts.opt_ps_to_cs_overlap_v2, "AW2_PS_TO_CS_OVERLAP_V2 must default OFF");
        assert!(!l.opts.opt_gfx_overlap_capped, "AW2_GFX_OVERLAP_CAPPED must default OFF");
        assert!(!l.opts.opt_cs_fast_bind_exact, "AW2_CS_FAST_BIND_EXACT must default OFF");
    }
}

#[test]
fn g50_placed_heap_tracker_disjoint_vs_overlapping_and_retired() {
    if crate::metal::device().is_none() {
        return;
    }
    let _guard = gpu_stats_lock();
    let prev_global = crate::metal::resource::swap_any_placed_alias_for_test(false);

    let hdesc = crate::types::HeapDesc {
        SizeInBytes: 4 * 1024 * 1024,
        Properties: test_heap(heap_type::DEFAULT),
        Alignment: 65536,
        Flags: 0,
    };
    let heap = Arc::new(crate::metal::heap::Heap::new(&hdesc, "g50 placed tracker heap").unwrap());
    let bdesc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: 4096,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
        ..Default::default()
    };

    // 1) Disjoint placed allocations on the same MTLHeap must NOT flag `may_alias_placed()`.
    let b_dis0 = Resource::new_placed(&heap, 0, &bdesc, MTLTextureUsage::empty(), "g50 dis0").unwrap();
    let b_dis1 = Resource::new_placed(&heap, 256 * 1024, &bdesc, MTLTextureUsage::empty(), "g50 dis1").unwrap();
    assert!(b_dis0.placed_in.is_some() && b_dis1.placed_in.is_some());
    assert!(!b_dis0.may_alias_placed(), "disjoint placed buffer 0 must not flag may_alias_placed");
    assert!(!b_dis1.may_alias_placed(), "disjoint placed buffer 1 must not flag may_alias_placed");
    assert!(!crate::metal::resource::any_placed_alias());

    // 2) Overlapping placed allocations on the same MTLHeap MUST flag BOTH resources AND `any_placed_alias()`.
    let b_ov0 = Resource::new_placed(&heap, 512 * 1024, &bdesc, MTLTextureUsage::empty(), "g50 ov0").unwrap();
    assert!(!b_ov0.may_alias_placed());
    let b_ov1 = Resource::new_placed(&heap, 512 * 1024, &bdesc, MTLTextureUsage::empty(), "g50 ov1").unwrap();
    assert!(b_ov0.may_alias_placed(), "existing overlapping placed buffer must latch may_alias_placed");
    assert!(b_ov1.may_alias_placed(), "new overlapping placed buffer must latch may_alias_placed");
    assert!(crate::metal::resource::any_placed_alias());
    // Disjoint neighbors on the same heap remain unpoisoned!
    assert!(!b_dis0.may_alias_placed());
    assert!(!b_dis1.may_alias_placed());

    // 3) Dropping `b_ov1` retires its span into `retired` (grace window of 2 submit epochs):
    drop(b_ov1);
    let b_reuse_immediate =
        Resource::new_placed(&heap, 512 * 1024 + 128 * 1024, &bdesc, MTLTextureUsage::empty(), "g50 clean").unwrap();
    assert!(!b_reuse_immediate.may_alias_placed());
    assert!(b_ov0.may_alias_placed());

    drop(b_reuse_immediate);
    drop(b_ov0);
    let b_retired_overlap =
        Resource::new_placed(&heap, 512 * 1024, &bdesc, MTLTextureUsage::empty(), "g50 retired_ov").unwrap();
    assert!(
        b_retired_overlap.may_alias_placed(),
        "reuse within 2 submit epochs of drop must conservatively flag may_alias_placed"
    );
    drop(b_retired_overlap);

    crate::metal::resource::swap_any_placed_alias_for_test(prev_global);
}

#[test]
fn g50_a_sm66_handle_kind_flt_vs_int_and_tex2d_vs_3d_gpu_readback() {
    use crate::metal::pipeline::{
        register_shader_hk_profile_for_test, ComputeAccess, ComputeBind, ComputePipeline,
        HandleKindProfile, READS_BUF, READS_TEX,
    };
    use crate::metal::resource::{HK_CBV_BUF, HK_RAW_STRUCT_BUF, HK_TEX2D_FLT};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() {
        return;
    }
    let dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 2,
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

    let mut pso_raw = ComputePipeline::new(rs.clone(), &dxil, "g50_a_cs").unwrap();
    pso_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: true,
        msc_refined: true,
        uav_classes: READS_BUF | READS_TEX,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF | READS_TEX,
        unbounded_srv_classes: READS_BUF | READS_TEX,
        uav_slots: None,
        read_slots: None,
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
    };
    let shader_key = pso_raw.shader.key;
    let pso = Arc::new(pso_raw);
    register_shader_hk_profile_for_test(
        shader_key,
        HandleKindProfile {
            uav_write_kinds: HK_RAW_STRUCT_BUF,
            uav_srv_kinds: HK_RAW_STRUCT_BUF | HK_TEX2D_FLT,
            ro_srv_kinds: HK_TEX2D_FLT | HK_RAW_STRUCT_BUF,
            read_kinds: HK_TEX2D_FLT | HK_RAW_STRUCT_BUF | HK_CBV_BUF,
            unbounded_srv_kinds: HK_TEX2D_FLT,
        },
    );

    let tex2d_flt = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex2d_uint = test_texture(4, 1, R32_UINT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex3d_desc = ResourceDesc {
        Dimension: dim::TEXTURE3D,
        Width: 4,
        Height: 1,
        DepthOrArraySize: 2,
        MipLevels: 1,
        Format: 2,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
        ..Default::default()
    };
    let tex3d_flt = Resource::new_committed(
        &tex3d_desc,
        &test_heap(heap_type::DEFAULT),
        MTLTextureUsage::empty(),
        "g50_a tex3d",
    )
    .unwrap();
    let up_uint = test_buffer(512, heap_type::UPLOAD, 0);
    let b_out0 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_out1 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb0 = test_buffer(512, heap_type::READBACK, 0);
    let rb1 = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex2d_flt, &tex2d_uint, &tex3d_flt, &up_uint, &b_out0, &b_out1, &rb0, &rb1] {
        r.register_ptr();
    }
    let _ = crate::metal::views::uav_entry(Some(&tex2d_uint), None).unwrap();
    let _ = crate::metal::views::uav_entry(Some(&tex3d_flt), None).unwrap();

    let heap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        4,
        true,
        "g50_a heap",
    )
    .unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex2d_flt), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rtv_flt = TargetView::rtv(&tex2d_flt, None).unwrap();
    let tex2d_flt_id = &tex2d_flt as *const Resource as usize;
    let tex2d_uint_id = &tex2d_uint as *const Resource as usize;
    let tex3d_flt_id = &tex3d_flt as *const Resource as usize;
    let b_out0_id = &b_out0 as *const Resource as usize;
    let b_out1_id = &b_out1 as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g50_a queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_flt, [3.0, 6.0, 9.0, 1.0]).unwrap() };
    init.barrier_list_with(
        &[Barrier::Transition(tex2d_flt_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)],
        true,
        false,
    );
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_case = |enable_a: bool| -> (u64, u64, [f32; 4], [f32; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_sm66_handle_kind_exact = enable_a;
        let a0 = stats::c::sm66_handle_kind_exact.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.set_compute_pipeline(pso.clone());

        // Dispatch 1 reads `tex2d_flt` (`HK_TEX2D_FLT`) and writes `b_out0` (`HK_RAW_STRUCT_BUF`).
        l.compute_root.set_descriptor(1, b_out0.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();

        let eb0 = stats::c::enc_barrier.load(Relaxed);
        l.barrier_list_with(
            &[
                Barrier::Uav(tex3d_flt_id),
                Barrier::Transition(tex2d_uint_id, STATE_UAV, STATE_COPY_DEST),
            ],
            true,
            false,
        );
        l.compute_root.set_descriptor(1, b_out1.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let fp_uint = PlacedFootprint { offset: 0, format: R32_UINT, width: 4, height: 1, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(
            CopyLoc::Subresource(&tex2d_uint, 0),
            [0, 0, 0],
            CopyLoc::Footprint(&up_uint, fp_uint),
            None,
        )
        .unwrap();
        let eb_false_pos = stats::c::enc_barrier.load(Relaxed) - eb0;

        l.barrier_list_with(
            &[
                Barrier::Transition(b_out0_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(b_out1_id, STATE_UAV, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb0, 0, &b_out0, 0, 16).unwrap();
        l.copy_buffer(&rb1, 0, &b_out1, 0, 16).unwrap();
        l.close().unwrap();

        let a_d = stats::c::sm66_handle_kind_exact.load(Relaxed) - a0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g50_A must produce 0 bad timestamps");
        alloc.reset();

        let v0: Vec<f32> = buf_words(&rb0, 4).into_iter().map(f32::from_bits).collect();
        let v1: Vec<f32> = buf_words(&rb1, 4).into_iter().map(f32::from_bits).collect();
        (a_d, eb_false_pos, [v0[0], v0[1], v0[2], v0[3]], [v1[0], v1[1], v1[2], v1[3]])
    };

    let (a_off, eb_off, v0_off, v1_off) = run_case(false);
    let (a_on, eb_on, v0_on, v1_on) = run_case(true);
    assert_eq!(a_off, 0);
    assert!(eb_off >= 2, "baseline must emit false-positive intra-CS barriers on tex3d_flt & tex2d_uint: got {eb_off}");
    assert_eq!(eb_on, 0, "g50_A must eliminate both false-positive intra-CS barriers via HK_TEX2D_FLT disambiguation");
    assert!(a_on >= 2, "sm66_handle_kind_exact must increment: got {a_on}");
    assert_eq!((v0_on, v1_on), (v0_off, v1_off));
    assert_eq!(v0_on, [3.0, 6.0, 9.0, 1.0]);
    assert_eq!(v1_on, [3.0, 6.0, 9.0, 1.0]);
}

#[test]
fn g50_b_ps_to_cs_placed_non_alias_and_uav0_unpoison_gpu_readback() {
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF, READS_TEX};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() {
        return;
    }
    let dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 2,
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
    let mut pso_raw = ComputePipeline::new(rs.clone(), &dxil, "g50_b_cs").unwrap();
    pso_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: 0,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }]),
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
    };
    let pso = Arc::new(pso_raw);

    // Two DISJOINT placed textures on the same placement MTLHeap (`!may_alias_placed()`).
    let hdesc = crate::types::HeapDesc {
        SizeInBytes: 4 * 1024 * 1024,
        Properties: test_heap(heap_type::DEFAULT),
        Alignment: 65536,
        Flags: 0,
    };
    let pheap = Arc::new(crate::metal::heap::Heap::new(&hdesc, "g50_b placed heap").unwrap());
    let tdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 2,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let tex_a = Resource::new_placed(&pheap, 0, &tdesc, MTLTextureUsage::empty(), "g50_b tex_a").unwrap();
    let tex_b = Resource::new_placed(&pheap, 256 * 1024, &tdesc, MTLTextureUsage::empty(), "g50_b tex_b").unwrap();
    assert!(!tex_a.may_alias_placed() && !tex_b.may_alias_placed());

    let b_dis = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_out = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_out = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &b_dis, &b_out, &rb_out] {
        r.register_ptr();
    }
    let dheap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g50_b desc heap",
    )
    .unwrap();
    unsafe {
        let eb = crate::metal::views::srv_entry(Some(&tex_b), None).unwrap();
        std::ptr::copy_nonoverlapping(eb.as_ptr(), dheap.cpu_base as *mut u8, 24);
        std::ptr::copy_nonoverlapping(eb.as_ptr(), (dheap.cpu_base + 24) as *mut u8, 24);
        let ea = crate::metal::views::srv_entry(Some(&tex_a), None).unwrap();
        std::ptr::copy_nonoverlapping(ea.as_ptr(), (dheap.cpu_base + 48) as *mut u8, 24);
        std::ptr::copy_nonoverlapping(ea.as_ptr(), (dheap.cpu_base + 72) as *mut u8, 24);
    }
    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let b_out_id = &b_out as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g50_b queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_b, [9.0, 9.0, 9.0, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_b_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_case = |enable_b_v2: bool| -> (u64, bool, [f32; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_ps_to_cs_overlap = !enable_b_v2;
        l.opts.opt_ps_to_cs_overlap_v2 = enable_b_v2;
        let b0 = stats::c::ps_to_cs_overlap_v2.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.bar.in_list_gfx_non_clear = true;
        l.set_descriptor_heaps(dheap.gpu_base, 0);
        l.compute_root.set_root_signature(rs.clone()).unwrap();
        l.set_compute_pipeline(pso.clone());

        // Anchor CS dispatch so `in_list_cs` is non-zero:
        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        unsafe { l.encode_rtv_clear_pass(&rtv_a, [2.0, 5.0, 8.0, 1.0]).unwrap() };
        l.end_encoder();
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
                Barrier::Uav(0),
            ],
            true,
            false,
        );

        // Disjoint Dispatch 1 (reads `tex_b`, NOT `tex_a`):
        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let kept_across_disjoint = !l.bar.enc_cs_soft_disp.is_empty();

        // Consuming Dispatch 2 (reads `tex_a`, writes `b_out`): must flush `Fragment -> Dispatch` mid-encoder!
        l.compute_root.set_table(0, dheap.gpu_base + 48).unwrap();
        l.compute_root.set_descriptor(1, b_out.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        assert!(l.bar.enc_cs_soft_disp.is_empty(), "consuming dispatch must flush enc_cs_soft_disp");
        l.end_encoder();

        l.barrier_list_with(&[Barrier::Transition(b_out_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
        l.copy_buffer(&rb_out, 0, &b_out, 0, 16).unwrap();
        l.close().unwrap();

        let b_d = stats::c::ps_to_cs_overlap_v2.load(Relaxed) - b0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g50_B must produce 0 bad timestamps");
        alloc.reset();

        let vo: Vec<f32> = buf_words(&rb_out, 4).into_iter().map(f32::from_bits).collect();
        (b_d, kept_across_disjoint, [vo[0], vo[1], vo[2], vo[3]])
    };

    let (b_v1, kept_v1, vo_v1) = run_case(false);
    let (b_v2, kept_v2, vo_v2) = run_case(true);
    assert_eq!(b_v1, 0);
    assert!(!kept_v1, "g49_A blocked placed resources and poisoned cs_soft_disp on Barrier::Uav(0)");
    assert!(kept_v2, "g50_B must preserve placed non-aliased cs_soft_disp_items across Barrier::Uav(0) and disjoint Dispatch 1");
    assert!(b_v2 >= 1, "ps_to_cs_overlap_v2 must increment: got {b_v2}");
    assert_eq!(vo_v2, vo_v1);
    assert_eq!(vo_v2, [2.0, 5.0, 8.0, 1.0]);
}

#[test]
fn g50_c_gfx_overlap_placed_liststart_and_chain_cap_gpu_readback() {
    use std::sync::atomic::Ordering::Relaxed;

    let Some((gfx_rs, gfx_pso)) = build_g48_vs_srv_ps_cb0_pso("g50_c_gfx") else { return };

    let hdesc = crate::types::HeapDesc {
        SizeInBytes: 4 * 1024 * 1024,
        Properties: test_heap(heap_type::DEFAULT),
        Alignment: 65536,
        Flags: 0,
    };
    let pheap = Arc::new(crate::metal::heap::Heap::new(&hdesc, "g50_c placed heap").unwrap());
    let rtdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: RGBA8_UNORM,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let rt0 = Resource::new_placed(&pheap, 0, &rtdesc, MTLTextureUsage::empty(), "g50_c rt0").unwrap();
    let rt1 = Resource::new_placed(&pheap, 256 * 1024, &rtdesc, MTLTextureUsage::empty(), "g50_c rt1").unwrap();
    let rt2 = Resource::new_placed(&pheap, 512 * 1024, &rtdesc, MTLTextureUsage::empty(), "g50_c rt2").unwrap();
    let rt3 = Resource::new_placed(&pheap, 768 * 1024, &rtdesc, MTLTextureUsage::empty(), "g50_c rt3").unwrap();
    assert!(!rt0.may_alias_placed() && !rt1.may_alias_placed() && !rt2.may_alias_placed() && !rt3.may_alias_placed());

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let rb_rt = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&rt0, &rt1, &rt2, &rt3, &cb_up, &buf_vs, &rb_rt] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let rtv0 = TargetView::rtv(&rt0, None).unwrap();
    let rtv1 = TargetView::rtv(&rt1, None).unwrap();
    let rtv2 = TargetView::rtv(&rt2, None).unwrap();
    let rtv3 = TargetView::rtv(&rt3, None).unwrap();
    let rt0_id = &rt0 as *const Resource as usize;
    let rt1_id = &rt1 as *const Resource as usize;
    let rt2_id = &rt2 as *const Resource as usize;
    let rt3_id = &rt3 as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g50_c queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |enable_c: bool| -> (u64, u8, u8, u8, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_gfx_overlap_capped = enable_c;
        let c0 = stats::c::gfx_overlap_capped.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.gfx_root.set_root_signature(gfx_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(gfx_pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // 1) ListStart `Transition(rt0, PIXEL_SRV -> RENDER_TARGET)` on a placed non-aliased RT:
        l.barrier_list_with(&[Barrier::Transition(rt0_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
        l.set_render_targets(vec![rtv0], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        // 2) In-list `Fragment -> NON_PIXEL_SRV | PIXEL_SRV` chain cap (`gfx_ps_overlap_chain < 2`):
        l.barrier_list_with(&[Barrier::Transition(rt0_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)], true, false);
        l.set_render_targets(vec![rtv1], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        let ch1 = l.rp_sync.gfx_ps_overlap_chain;

        // Hop 2: rt1 -> SRV, draw into rt2 (chain 1 -> 2 when enabled).
        l.barrier_list_with(&[Barrier::Transition(rt1_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)], true, false);
        l.set_render_targets(vec![rtv2], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        let ch2 = l.rp_sync.gfx_ps_overlap_chain;

        // Hop 3: rt2 -> SRV, draw into rt3: at `chain == 2`, `gfx_to_ps_overlap_active()` becomes FALSE:
        l.barrier_list_with(&[Barrier::Transition(rt2_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)], true, false);
        l.set_render_targets(vec![rtv3], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        let ch3 = l.rp_sync.gfx_ps_overlap_chain;

        l.barrier_list_with(&[Barrier::Transition(rt3_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp), [0, 0, 0], CopyLoc::Subresource(&rt3, 0), None).unwrap();
        l.close().unwrap();

        let c_d = stats::c::gfx_overlap_capped.load(Relaxed) - c0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g50_C must produce 0 bad timestamps");
        alloc.reset();

        (c_d, ch1, ch2, ch3, buf_words(&rb_rt, 1)[0])
    };

    let (c_off, ch1_off, ch2_off, ch3_off, px_off) = run_case(false);
    let (c_on, ch1_on, ch2_on, ch3_on, px_on) = run_case(true);
    assert_eq!((c_off, ch1_off, ch2_off, ch3_off), (0, 0, 0, 0));
    assert!(c_on >= 2, "gfx_overlap_capped must increment on placed ListStart split + chain deferral: got {c_on}");
    assert_eq!(ch1_on, 1, "first in-list Fragment->PS deferral must advance chain to 1");
    assert_eq!(ch2_on, 2, "second in-list Fragment->PS deferral must advance chain to 2");
    assert_eq!(ch3_on, 0, "third hop must hit depth cap 2, emit Fragment->PRE_RASTER sync, and reset chain to 0");
    assert_eq!(px_on, px_off);
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g50_d_cs_fast_bind_placed_exact_no_enc_barrier_inflation_gpu_readback() {
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF, READS_TEX};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() {
        return;
    }
    let dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 2,
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

    let mut pso_raw = ComputePipeline::new(rs.clone(), &dxil, "g50_d_cs").unwrap();
    pso_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: READS_BUF,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 0 },
            ComputeBind::TableEntry { param: 0, byte_offset: 24 },
        ],
    };
    let pso_unb_buf = Arc::new(ComputePipeline {
        pso: pso_raw.pso.clone(),
        rs: pso_raw.rs.clone(),
        shader: pso_raw.shader.clone(),
        threadgroup: pso_raw.threadgroup,
        hk_profile: pso_raw.hk_profile,
        access: ComputeAccess {
            touches_textures: true,
            heap_indexed_or_unknown: false,
            msc_refined: true,
            uav_classes: READS_BUF,
            read_classes: READS_BUF | READS_TEX,
            msc_exact_classes: READS_BUF | READS_TEX,
            uav_srv_classes: READS_BUF | READS_TEX,
            unbounded_srv_classes: READS_BUF,
            uav_slots: None,
            read_slots: None,
            bounded_read_slots: vec![
                ComputeBind::TableEntry { param: 0, byte_offset: 0 },
                ComputeBind::TableEntry { param: 0, byte_offset: 24 },
            ],
        },
    });
    let pso = Arc::new(pso_raw);

    // Disjoint placed texture `tex_other` + disjoint placed buffers `b_srv`, `b0`, `b1`.
    let hdesc = crate::types::HeapDesc {
        SizeInBytes: 4 * 1024 * 1024,
        Properties: test_heap(heap_type::DEFAULT),
        Alignment: 65536,
        Flags: 0,
    };
    let pheap = Arc::new(crate::metal::heap::Heap::new(&hdesc, "g50_d placed heap").unwrap());
    let tdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 2,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
        ..Default::default()
    };
    let bdesc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: 512,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
        ..Default::default()
    };
    let tex_other = Resource::new_placed(&pheap, 0, &tdesc, MTLTextureUsage::empty(), "g50_d tex_other").unwrap();
    let b_srv = Resource::new_placed(&pheap, 256 * 1024, &bdesc, MTLTextureUsage::empty(), "g50_d b_srv").unwrap();
    let b0 = Resource::new_placed(&pheap, 512 * 1024, &bdesc, MTLTextureUsage::empty(), "g50_d b0").unwrap();
    let b1 = Resource::new_placed(&pheap, 768 * 1024, &bdesc, MTLTextureUsage::empty(), "g50_d b1").unwrap();
    assert!(!tex_other.may_alias_placed() && !b_srv.may_alias_placed() && !b0.may_alias_placed() && !b1.may_alias_placed());

    let tex = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let up_other = test_buffer(512, heap_type::UPLOAD, 0);
    let rb0 = test_buffer(512, heap_type::READBACK, 0);
    let rb1 = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex, &tex_other, &b_srv, &b0, &b1, &up_other, &rb0, &rb1] {
        r.register_ptr();
    }
    let dheap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g50_d desc heap",
    )
    .unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), dheap.cpu_base as *mut u8, 24);
        let p1 = (dheap.cpu_base + 24) as *mut u64;
        p1.write_unaligned(b_srv.gpu_va);
        p1.add(1).write_unaligned(0);
        p1.add(2).write_unaligned(64);
    }
    let rtv_tex = TargetView::rtv(&tex, None).unwrap();
    let tex_id = &tex as *const Resource as usize;
    let tex_other_id = &tex_other as *const Resource as usize;
    let b0_id = &b0 as *const Resource as usize;
    let b1_id = &b1 as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g50_d queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_tex, [7.0, 11.0, 13.0, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_case = |enable_d: bool| -> (u64, u64, u64, [f32; 4], [f32; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_cs_fast_bind_exact = enable_d;
        let d0 = stats::c::cs_fast_bind_exact.load(Relaxed);
        let eb_total0 = stats::c::enc_barrier.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.set_descriptor_heaps(dheap.gpu_base, 0);
        l.compute_root.set_root_signature(rs.clone()).unwrap();
        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.set_compute_pipeline(pso.clone());

        // Two back-to-back dispatches with identical bindings to exercise `CsBindCache` hit:
        l.compute_root.set_descriptor(1, b0.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.barrier_list_with(&[Barrier::Uav(0)], true, false);
        l.dispatch(1, 1, 1).unwrap();

        // Unbounded buffer-UAV dispatch (`uav_slots = None`, `uav_classes = READS_BUF`), followed by
        // `Transition(placed tex_other, UAV -> COPY_DEST)` + `copy_texture_region`:
        l.set_compute_pipeline(pso_unb_buf.clone());
        l.compute_root.set_descriptor(1, b1.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let eb_tex0 = stats::c::enc_barrier.load(Relaxed);
        l.barrier_list_with(&[Barrier::Transition(tex_other_id, STATE_UAV, STATE_COPY_DEST)], true, false);
        let fp_other = PlacedFootprint { offset: 0, format: 2, width: 4, height: 1, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(
            CopyLoc::Subresource(&tex_other, 0),
            [0, 0, 0],
            CopyLoc::Footprint(&up_other, fp_other),
            None,
        )
        .unwrap();
        let eb_tex = stats::c::enc_barrier.load(Relaxed) - eb_tex0;
        l.end_encoder();

        l.barrier_list_with(
            &[
                Barrier::Transition(b0_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(b1_id, STATE_UAV, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb0, 0, &b0, 0, 16).unwrap();
        l.copy_buffer(&rb1, 0, &b1, 0, 16).unwrap();
        l.close().unwrap();

        let d_d = stats::c::cs_fast_bind_exact.load(Relaxed) - d0;
        let eb_total = stats::c::enc_barrier.load(Relaxed) - eb_total0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g50_D must produce 0 bad timestamps");
        alloc.reset();

        let v0: Vec<f32> = buf_words(&rb0, 4).into_iter().map(f32::from_bits).collect();
        let v1: Vec<f32> = buf_words(&rb1, 4).into_iter().map(f32::from_bits).collect();
        (d_d, eb_tex, eb_total, [v0[0], v0[1], v0[2], v0[3]], [v1[0], v1[1], v1[2], v1[3]])
    };

    let (d_off, eb_tex_off, eb_tot_off, r0_off, r1_off) = run_case(false);
    let (d_on, eb_tex_on, eb_tot_on, r0_on, r1_on) = run_case(true);
    assert_eq!(d_off, 0);
    assert_eq!(eb_tex_off, 1, "baseline emitted false-positive Dispatch->Blit barrier on placed tex_other");
    assert_eq!(eb_tex_on, 0, "g50_D must skip false-positive Dispatch->Blit barrier on non-aliased placed tex_other");
    assert!(eb_tot_on < eb_tot_off, "g50_D must strictly reduce enc_barrier without Uav(0) split inflation: {eb_tot_on} vs {eb_tot_off}");
    assert!(d_on >= 3, "cs_fast_bind_exact must increment on CsBindCache + class narrowing: got {d_on}");
    assert_eq!((r0_on, r1_on), (r0_off, r1_off));
    assert_eq!(r0_on, [7.0, 11.0, 13.0, 1.0]);
    assert_eq!(r1_on, [7.0, 11.0, 13.0, 1.0]);
}

#[test]
fn g50_abcd_combined_gpu_readback_and_bad_zero() {
    use crate::metal::pipeline::{
        register_shader_hk_profile_for_test, ComputeAccess, ComputeBind, ComputePipeline,
        HandleKindProfile, READS_BUF, READS_TEX,
    };
    use crate::metal::resource::{HK_CBV_BUF, HK_RAW_STRUCT_BUF, HK_TEX2D_FLT};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    let Some((gfx_rs, gfx_pso)) = build_g48_vs_srv_ps_cb0_pso("g50_combo_gfx") else { return };
    let cs_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
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
                    num_descriptors: 2,
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
    let mut cs_raw = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g50_combo_cs").unwrap();
    cs_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: 0,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }]),
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
    };
    let cs_key = cs_raw.shader.key;
    let cs_unb_pso = Arc::new(ComputePipeline {
        pso: cs_raw.pso.clone(),
        rs: cs_raw.rs.clone(),
        shader: cs_raw.shader.clone(),
        threadgroup: cs_raw.threadgroup,
        hk_profile: cs_raw.hk_profile,
        access: ComputeAccess {
            touches_textures: true,
            heap_indexed_or_unknown: false,
            msc_refined: true,
            uav_classes: READS_BUF,
            read_classes: READS_BUF | READS_TEX,
            msc_exact_classes: READS_BUF | READS_TEX,
            uav_srv_classes: READS_BUF | READS_TEX,
            unbounded_srv_classes: READS_TEX,
            uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
            read_slots: None,
            bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
        },
    });
    let cs_pso = Arc::new(cs_raw);
    register_shader_hk_profile_for_test(
        cs_key,
        HandleKindProfile {
            uav_write_kinds: HK_RAW_STRUCT_BUF,
            uav_srv_kinds: HK_RAW_STRUCT_BUF | HK_TEX2D_FLT,
            ro_srv_kinds: HK_TEX2D_FLT | HK_RAW_STRUCT_BUF,
            read_kinds: HK_TEX2D_FLT | HK_RAW_STRUCT_BUF | HK_CBV_BUF,
            unbounded_srv_kinds: HK_TEX2D_FLT,
        },
    );

    let hdesc = crate::types::HeapDesc {
        SizeInBytes: 4 * 1024 * 1024,
        Properties: test_heap(heap_type::DEFAULT),
        Alignment: 65536,
        Flags: 0,
    };
    let pheap = Arc::new(crate::metal::heap::Heap::new(&hdesc, "g50_combo pheap").unwrap());
    let tdesc_f32 = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 2,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let rtdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: RGBA8_UNORM,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let tex_a = Resource::new_placed(&pheap, 0, &tdesc_f32, MTLTextureUsage::empty(), "g50_combo tex_a").unwrap();
    let tex_b = Resource::new_placed(&pheap, 256 * 1024, &tdesc_f32, MTLTextureUsage::empty(), "g50_combo tex_b").unwrap();
    let rt_draw = Resource::new_placed(&pheap, 512 * 1024, &rtdesc, MTLTextureUsage::empty(), "g50_combo rt_draw").unwrap();
    let tex_uint = test_texture(4, 1, R32_UINT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let _ = crate::metal::views::uav_entry(Some(&tex_uint), None).unwrap();

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let b_dis = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_out = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_out = test_buffer(512, heap_type::READBACK, 0);
    let rb_draw = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &rt_draw, &tex_uint, &cb_up, &buf_vs, &b_dis, &b_out, &rb_out, &rb_draw] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let dheap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g50_combo desc heap",
    )
    .unwrap();
    unsafe {
        let eb = crate::metal::views::srv_entry(Some(&tex_b), None).unwrap();
        std::ptr::copy_nonoverlapping(eb.as_ptr(), dheap.cpu_base as *mut u8, 24);
        let ea = crate::metal::views::srv_entry(Some(&tex_a), None).unwrap();
        std::ptr::copy_nonoverlapping(ea.as_ptr(), (dheap.cpu_base + 48) as *mut u8, 24);
    }
    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let rtv_draw = TargetView::rtv(&rt_draw, None).unwrap();
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let rt_draw_id = &rt_draw as *const Resource as usize;
    let tex_uint_id = &tex_uint as *const Resource as usize;
    let b_out_id = &b_out as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g50_combo queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_b, [4.5, 5.5, 6.5, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_b_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_combo = |on: bool| -> ([f32; 4], u32, [u64; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_sm66_handle_kind_exact = on;
        l.opts.opt_ps_to_cs_overlap_v2 = on;
        l.opts.opt_gfx_overlap_capped = on;
        l.opts.opt_cs_fast_bind_exact = on;
        let c0 = [
            stats::c::sm66_handle_kind_exact.load(Relaxed),
            stats::c::ps_to_cs_overlap_v2.load(Relaxed),
            stats::c::gfx_overlap_capped.load(Relaxed),
            stats::c::cs_fast_bind_exact.load(Relaxed),
        ];

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.set_descriptor_heaps(dheap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.set_compute_pipeline(cs_pso.clone());
        l.gfx_root.set_root_signature(gfx_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(gfx_pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // Placed ListStart split (`g50_C`):
        l.barrier_list_with(&[Barrier::Transition(rt_draw_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
        l.set_render_targets(vec![rtv_draw], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        // Anchor CS + `HK_TEX2D_INT` SRV->COPY_DEST (`g50_A` + `g50_D`):
        l.set_compute_pipeline(cs_unb_pso.clone());
        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.barrier_list_with(&[Barrier::Transition(tex_uint_id, STATE_NON_PIXEL_SRV, STATE_COPY_DEST)], true, false);
        l.set_compute_pipeline(cs_pso.clone());
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        // Placed RTV clear + `Barrier::Uav(0)` (`g50_B`):
        unsafe { l.encode_rtv_clear_pass(&rtv_a, [1.25, 2.25, 3.25, 1.0]).unwrap() };
        l.end_encoder();
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
                Barrier::Uav(0),
            ],
            true,
            false,
        );
        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.compute_root.set_table(0, dheap.gpu_base + 48).unwrap();
        l.compute_root.set_descriptor(1, b_out.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        l.barrier_list_with(
            &[
                Barrier::Transition(b_out_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(rt_draw_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb_out, 0, &b_out, 0, 16).unwrap();
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_draw, fp), [0, 0, 0], CopyLoc::Subresource(&rt_draw, 0), None).unwrap();
        l.close().unwrap();

        let cd = [
            stats::c::sm66_handle_kind_exact.load(Relaxed) - c0[0],
            stats::c::ps_to_cs_overlap_v2.load(Relaxed) - c0[1],
            stats::c::gfx_overlap_capped.load(Relaxed) - c0[2],
            stats::c::cs_fast_bind_exact.load(Relaxed) - c0[3],
        ];
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g50 combined must produce 0 bad timestamps");
        alloc.reset();

        let vo: Vec<f32> = buf_words(&rb_out, 4).into_iter().map(f32::from_bits).collect();
        ([vo[0], vo[1], vo[2], vo[3]], buf_words(&rb_draw, 1)[0], cd)
    };

    let (vo_off, pd_off, cd_off) = run_combo(false);
    let (vo_on, pd_on, cd_on) = run_combo(true);
    assert_eq!(cd_off, [0, 0, 0, 0]);
    assert!(
        cd_on.iter().all(|&c| c >= 1),
        "all four g50 counters must fire in g50_abcd_combined: got {cd_on:?}"
    );
    assert_eq!(vo_on, vo_off);
    assert_eq!(pd_on, pd_off);
    assert_eq!(vo_on, [1.25, 2.25, 3.25, 1.0]);
    assert_eq!(pd_on, 0xff00_ff00);
}

#[test]
fn g51_default_knobs_off_and_baseline_unchanged() {
    if std::env::var_os("AW2_SM66_HANDLE_KIND_V2").is_none() {
        assert!(!opt_sm66_handle_kind_v2_env());
    }
    if std::env::var_os("AW2_PS_TO_CS_HK_OVERLAP").is_none() {
        assert!(!opt_ps_to_cs_hk_overlap_env());
    }
    if std::env::var_os("AW2_GFX_OVERLAP_FRAG_CAP").is_none() {
        assert!(!opt_gfx_overlap_frag_cap_env());
    }
    if std::env::var_os("PORT_CS_BIND_CACHE").is_none() && std::env::var_os("AW2_CS_BIND_CACHE").is_none() {
        assert!(opt_cs_bind_cache_env());
    }
    if std::env::var_os("PORT_CS_BIND_INTRA_SAFE").is_none() && std::env::var_os("AW2_CS_BIND_INTRA_SAFE").is_none() {
        assert!(!opt_cs_bind_intra_safe_env());
    }
    if let Some(l) = CommandList::new(ListType::Direct) {
        if std::env::var_os("AW2_SM66_HANDLE_KIND_V2").is_none() {
            assert!(!l.opts.opt_sm66_handle_kind_v2);
        }
        if std::env::var_os("AW2_PS_TO_CS_HK_OVERLAP").is_none() {
            assert!(!l.opts.opt_ps_to_cs_hk_overlap);
        }
        if std::env::var_os("AW2_GFX_OVERLAP_FRAG_CAP").is_none() {
            assert!(!l.opts.opt_gfx_overlap_frag_cap);
        }
        if std::env::var_os("PORT_CS_BIND_CACHE").is_none() && std::env::var_os("AW2_CS_BIND_CACHE").is_none() {
            assert!(l.opts.opt_cs_bind_cache);
        }
        if std::env::var_os("PORT_CS_BIND_INTRA_SAFE").is_none() && std::env::var_os("AW2_CS_BIND_INTRA_SAFE").is_none() {
            assert!(!l.opts.opt_cs_bind_intra_safe);
        }
        if !l.opts.opt_sm66_handle_kind_v2 && !l.opts.opt_ps_to_cs_hk_overlap && !l.opts.opt_cs_bind_intra_safe {
            assert!(!l.hk_v2_active());
        }
    }
}

#[test]
fn g51_a_sm66_handle_kind_v2_unb_narrow_and_ro_war_and_util_clear() {
    use crate::metal::pipeline::{
        register_shader_hk_profile_for_test, ComputeAccess, ComputeBind, ComputePipeline,
        HandleKindProfile, READS_BUF, READS_TEX,
    };
    use crate::metal::resource::{HK_CBV_BUF, HK_RAW_STRUCT_BUF, HK_TEX2D_FLT, HK_TEX3D_FLT};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 2,
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

    let mut pso_raw = ComputePipeline::new(rs.clone(), &dxil, "g51_a_cs").unwrap();
    pso_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF | READS_TEX,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF | READS_TEX,
        unbounded_srv_classes: READS_BUF | READS_TEX,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
    };
    let shader_key = u128::MAX - 510;
    Arc::make_mut(&mut pso_raw.shader).key = shader_key;
    let pso = Arc::new(pso_raw);
    register_shader_hk_profile_for_test(
        shader_key,
        HandleKindProfile {
            uav_write_kinds: HK_RAW_STRUCT_BUF | HK_TEX3D_FLT,
            uav_srv_kinds: HK_RAW_STRUCT_BUF | HK_TEX3D_FLT | HK_TEX2D_FLT,
            ro_srv_kinds: HK_TEX2D_FLT | HK_RAW_STRUCT_BUF,
            read_kinds: HK_TEX2D_FLT | HK_RAW_STRUCT_BUF | HK_TEX3D_FLT | HK_CBV_BUF,
            unbounded_srv_kinds: HK_RAW_STRUCT_BUF,
        },
    );

    let tex_bound = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_disjoint = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex3d_desc = ResourceDesc {
        Dimension: dim::TEXTURE3D,
        Width: 4,
        Height: 1,
        DepthOrArraySize: 2,
        MipLevels: 1,
        Format: 2,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_UNORDERED_ACCESS,
        ..Default::default()
    };
    let tex3d_war = Resource::new_committed(
        &tex3d_desc,
        &test_heap(heap_type::DEFAULT),
        MTLTextureUsage::empty(),
        "g51_a tex3d_war",
    )
    .unwrap();
    let tex2d_uav_other = test_texture(4, 1, R32_UINT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let _ = crate::metal::views::uav_entry(Some(&tex3d_war), None).unwrap();
    let _ = crate::metal::views::uav_entry(Some(&tex2d_uav_other), None).unwrap();

    let b_clear = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_out0 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_out1 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let up_tex = test_buffer(512, heap_type::UPLOAD, 0);
    let rb0 = test_buffer(512, heap_type::READBACK, 0);
    let rb1 = test_buffer(512, heap_type::READBACK, 0);
    for r in [
        &tex_bound,
        &tex_disjoint,
        &tex3d_war,
        &tex2d_uav_other,
        &b_clear,
        &b_out0,
        &b_out1,
        &up_tex,
        &rb0,
        &rb1,
    ] {
        r.register_ptr();
    }

    let dheap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g51_a dheap",
    )
    .unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex_bound), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), dheap.cpu_base as *mut u8, 24);
    }
    let shader_unb_buf_key = u128::MAX - 51;
    let mut sh_unb = pso.shader.clone();
    Arc::make_mut(&mut sh_unb).key = shader_unb_buf_key;
    let pso_unb_buf = Arc::new(ComputePipeline {
        pso: pso.pso.clone(),
        rs: pso.rs.clone(),
        shader: sh_unb,
        threadgroup: pso.threadgroup,
        hk_profile: pso.hk_profile,
        access: ComputeAccess {
            touches_textures: true,
            heap_indexed_or_unknown: true,
            msc_refined: true,
            uav_classes: READS_BUF | READS_TEX,
            read_classes: READS_BUF | READS_TEX,
            msc_exact_classes: READS_BUF | READS_TEX,
            uav_srv_classes: READS_BUF | READS_TEX,
            unbounded_srv_classes: READS_BUF,
            uav_slots: None,
            read_slots: None,
            bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
        },
    });
    register_shader_hk_profile_for_test(
        shader_unb_buf_key,
        HandleKindProfile {
            uav_write_kinds: HK_RAW_STRUCT_BUF,
            uav_srv_kinds: HK_RAW_STRUCT_BUF | HK_TEX2D_FLT,
            ro_srv_kinds: HK_TEX2D_FLT,
            read_kinds: HK_TEX2D_FLT | HK_RAW_STRUCT_BUF | HK_CBV_BUF,
            unbounded_srv_kinds: HK_RAW_STRUCT_BUF,
        },
    );
    let uav_clear_desc = {
        let mut d = [0u8; 40];
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[16..20].copy_from_slice(&16u32.to_le_bytes());
        d[20..24].copy_from_slice(&4u32.to_le_bytes());
        d
    };

    let rtv_bound = TargetView::rtv(&tex_bound, None).unwrap();
    let rtv_disjoint = TargetView::rtv(&tex_disjoint, None).unwrap();
    let tex_bound_id = &tex_bound as *const Resource as usize;
    let tex_disjoint_id = &tex_disjoint as *const Resource as usize;
    let tex3d_war_id = &tex3d_war as *const Resource as usize;
    let tex2d_uav_other_id = &tex2d_uav_other as *const Resource as usize;
    let b_out0_id = &b_out0 as *const Resource as usize;
    let b_out1_id = &b_out1 as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g51_a queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_bound, [5.0, 10.0, 15.0, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_bound_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_case = |enable_a2: bool| -> (u64, u64, u64, u64, [f32; 4], [f32; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        // Exercise handle classification inside the optional per-resource queue-barrier
        // path in both arms. G51_A alone does not enable barrier deferral.
        l.opts.opt_ib_bindless_war = true;
        l.opts.opt_cs_mid_qb_exact_v2 = true;
        l.opts.opt_sm66_handle_kind_v2 = enable_a2;
        let a0 = stats::c::sm66_handle_kind_v2.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.set_descriptor_heaps(dheap.gpu_base, 0);
        l.compute_root.set_root_signature(rs.clone()).unwrap();
        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.set_compute_pipeline(pso.clone());

        // Anchor CS dispatch, then RTV clear on `tex_disjoint` so `Fragment` gen advances:
        l.compute_root.set_descriptor(1, b_out0.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();
        l.bar.in_list_gfx_non_clear = true;
        unsafe { l.encode_rtv_clear_pass(&rtv_disjoint, [1.0, 2.0, 3.0, 1.0]).unwrap() };
        l.end_encoder();

        // 1) Open CS #2 with Dispatch 1 (`tex_bound`), then mid-CS `Transition(tex_disjoint, RT -> NON_PIXEL_SRV)`
        //    + WAR `Transition(tex3d_war, NON_PIXEL_SRV -> UAV)` before Dispatch 2 (`tex_bound` -> `b_out1`):
        l.compute_root.set_descriptor(1, b_out0.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        if enable_a2 {
            // The modeled shader can write Tex3D through a UAV but cannot read it through an SRV.
            let (disp_uav_hk, disp_ro_hk) = if l.opts.opt_ib_substage_narrow {
                (l.ib.ib_sub[0].disp_uav_hk, l.ib.ib_sub[0].disp_ro_hk)
            } else {
                (l.ib.ib_disp_uav_hk, l.ib.ib_disp_ro_hk)
            };
            assert_ne!(disp_uav_hk & HK_TEX3D_FLT, 0);
            assert_eq!(disp_ro_hk & HK_TEX3D_FLT, 0);
        }
        let mqb0 = stats::c::cs_mid_qb.load(Relaxed);
        let eb0 = stats::c::enc_barrier.load(Relaxed);
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_disjoint_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
                Barrier::Transition(tex3d_war_id, STATE_NON_PIXEL_SRV, STATE_UAV),
            ],
            true,
            false,
        );
        l.compute_root.set_descriptor(1, b_out1.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let mqb_d = stats::c::cs_mid_qb.load(Relaxed) - mqb0;
        let eb_war = stats::c::enc_barrier.load(Relaxed) - eb0;
        l.end_encoder();

        // 2) The fixture reads a float Texture2D. An integer-only texture cannot be that SRV.
        l.set_compute_pipeline(pso_unb_buf.clone());
        l.compute_root.set_descriptor(1, b_out1.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.clear_uav(
            &b_clear,
            Some(&uav_clear_desc),
            ClearValue::Uint([7, 7, 7, 7]),
            &[],
        )
        .unwrap();
        let eb1 = stats::c::enc_barrier.load(Relaxed);
        l.barrier_list_with(
            &[Barrier::Transition(tex2d_uav_other_id, STATE_UAV, STATE_COPY_DEST)],
            true,
            false,
        );
        let fp_t = PlacedFootprint { offset: 0, format: R32_UINT, width: 4, height: 1, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(
            CopyLoc::Subresource(&tex2d_uav_other, 0),
            [0, 0, 0],
            CopyLoc::Footprint(&up_tex, fp_t),
            None,
        )
        .unwrap();
        let eb_util = stats::c::enc_barrier.load(Relaxed) - eb1;
        l.end_encoder();

        l.barrier_list_with(
            &[
                Barrier::Transition(b_out0_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(b_out1_id, STATE_UAV, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb0, 0, &b_out0, 0, 16).unwrap();
        l.copy_buffer(&rb1, 0, &b_out1, 0, 16).unwrap();
        l.close().unwrap();

        let a_d = stats::c::sm66_handle_kind_v2.load(Relaxed) - a0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g51_A must produce 0 bad timestamps");
        alloc.reset();

        let v0: Vec<f32> = buf_words(&rb0, 4).into_iter().map(f32::from_bits).collect();
        let v1: Vec<f32> = buf_words(&rb1, 4).into_iter().map(f32::from_bits).collect();
        (a_d, mqb_d, eb_war, eb_util, [v0[0], v0[1], v0[2], v0[3]], [v1[0], v1[1], v1[2], v1[3]])
    };

    let (a_off, mqb_off, war_off, util_off, v0_off, v1_off) = run_case(false);
    let (a_on, mqb_on, war_on, util_on, v0_on, v1_on) = run_case(true);
    assert_eq!(a_off, 0);
    assert_eq!(mqb_off, 1, "baseline stalled on disjoint Tex2D_FLT mid-CS transition due to un-narrowed unbounded_srv_classes");
    assert_eq!(war_off, 0, "the following dispatch writes only its bound buffer");
    assert_eq!(util_off, 1, "baseline poisoned ib_disp_uav_hk to HK_ALL on clear_uav_uint");
    assert_eq!(mqb_on, 0, "g51_A must defer disjoint Tex2D_FLT mid-CS transition via unbounded_srv_kinds narrowing");
    assert_eq!(war_on, 0, "unrelated Tex3D transition must not add a WAR barrier");
    assert_eq!(util_on, 0, "g51_A must keep clear_uav_uint HK_RAW_STRUCT_BUF exact");
    assert!(a_on >= 3, "sm66_handle_kind_v2 must increment across all 3 mechanisms: got {a_on}");
    assert_eq!((v0_on, v1_on), (v0_off, v1_off));
    assert_eq!(v0_on, [5.0, 10.0, 15.0, 1.0]);
    assert_eq!(v1_on, [5.0, 10.0, 15.0, 1.0]);
}

#[test]
fn g51_b_ps_to_cs_hk_overlap_defers_tex2d_rt_for_buf_or_tex3d_cs() {
    use crate::metal::pipeline::{
        register_shader_hk_profile_for_test, ComputeAccess, ComputeBind, ComputePipeline,
        HandleKindProfile, READS_BUF, READS_TEX,
    };
    use crate::metal::resource::{HK_CBV_BUF, HK_RAW_STRUCT_BUF, HK_TEX2D_FLT, HK_TEX3D_FLT};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 2,
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

    let mut pso_raw = ComputePipeline::new(rs.clone(), &dxil, "g51_b_cs").unwrap();
    pso_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: READS_BUF | READS_TEX,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
    };
    let shader_key = u128::MAX - 511;
    Arc::make_mut(&mut pso_raw.shader).key = shader_key;
    let pso = Arc::new(pso_raw);
    register_shader_hk_profile_for_test(
        shader_key,
        HandleKindProfile {
            uav_write_kinds: HK_RAW_STRUCT_BUF,
            uav_srv_kinds: HK_RAW_STRUCT_BUF | HK_TEX2D_FLT | HK_TEX3D_FLT,
            ro_srv_kinds: HK_TEX2D_FLT | HK_TEX3D_FLT | HK_RAW_STRUCT_BUF,
            read_kinds: HK_TEX2D_FLT | HK_TEX3D_FLT | HK_RAW_STRUCT_BUF | HK_CBV_BUF,
            unbounded_srv_kinds: HK_RAW_STRUCT_BUF | HK_TEX3D_FLT,
        },
    );

    let hdesc = crate::types::HeapDesc {
        SizeInBytes: 4 * 1024 * 1024,
        Properties: test_heap(heap_type::DEFAULT),
        Alignment: 65536,
        Flags: 0,
    };
    let pheap = Arc::new(crate::metal::heap::Heap::new(&hdesc, "g51_b pheap").unwrap());
    let tdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 2,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let tex_a = Resource::new_placed(&pheap, 0, &tdesc, MTLTextureUsage::empty(), "g51_b tex_a").unwrap();
    let tex_b = Resource::new_placed(&pheap, 256 * 1024, &tdesc, MTLTextureUsage::empty(), "g51_b tex_b").unwrap();
    let b_dis = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_out = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_out = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &b_dis, &b_out, &rb_out] {
        r.register_ptr();
    }
    let dheap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g51_b dheap",
    )
    .unwrap();
    unsafe {
        let eb = crate::metal::views::srv_entry(Some(&tex_b), None).unwrap();
        std::ptr::copy_nonoverlapping(eb.as_ptr(), dheap.cpu_base as *mut u8, 24);
        let ea = crate::metal::views::srv_entry(Some(&tex_a), None).unwrap();
        std::ptr::copy_nonoverlapping(ea.as_ptr(), (dheap.cpu_base + 48) as *mut u8, 24);
    }
    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let b_out_id = &b_out as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g51_b queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_b, [9.0, 9.0, 9.0, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_b_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_case = |enable_b: bool| -> (u64, bool, [f32; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_ps_to_cs_overlap_v2 = !enable_b;
        l.opts.opt_ps_to_cs_hk_overlap = enable_b;
        let b0 = stats::c::ps_to_cs_hk_overlap.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.bar.in_list_gfx_non_clear = true;
        l.set_descriptor_heaps(dheap.gpu_base, 0);
        l.compute_root.set_root_signature(rs.clone()).unwrap();
        l.set_compute_pipeline(pso.clone());

        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        unsafe { l.encode_rtv_clear_pass(&rtv_a, [4.0, 8.0, 12.0, 1.0]).unwrap() };
        l.end_encoder();
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
                Barrier::Uav(0),
            ],
            true,
            false,
        );

        // Disjoint Dispatch 1 (reads `tex_b` + unbounded `Tex3D`/`StructBuf`, NOT `tex_a`!):
        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let kept = !l.bar.enc_cs_soft_disp.is_empty();

        // Consuming Dispatch 2 (binds `tex_a`): flushes `Fragment -> Dispatch` mid-encoder!
        l.compute_root.set_table(0, dheap.gpu_base + 48).unwrap();
        l.compute_root.set_descriptor(1, b_out.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        assert!(l.bar.enc_cs_soft_disp.is_empty());
        l.end_encoder();

        l.barrier_list_with(&[Barrier::Transition(b_out_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
        l.copy_buffer(&rb_out, 0, &b_out, 0, 16).unwrap();
        l.close().unwrap();

        let b_d = stats::c::ps_to_cs_hk_overlap.load(Relaxed) - b0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g51_B must produce 0 bad timestamps");
        alloc.reset();

        let vo: Vec<f32> = buf_words(&rb_out, 4).into_iter().map(f32::from_bits).collect();
        (b_d, kept, [vo[0], vo[1], vo[2], vo[3]])
    };

    let (b_off, kept_off, vo_off) = run_case(false);
    let (b_on, kept_on, vo_on) = run_case(true);
    assert_eq!(b_off, 0);
    assert!(!kept_off, "ps_to_cs_overlap_v2 without HK v2 stalled on unbounded READS_TEX");
    assert!(kept_on, "g51_B must overlap placed Tex2D_FLT RT across unbounded Tex3D/StructBuf CS");
    assert!(b_on >= 2, "ps_to_cs_hk_overlap must increment on HK disambiguation + soft-disp deferral: got {b_on}");
    assert_eq!(vo_on, vo_off);
    assert_eq!(vo_on, [4.0, 8.0, 12.0, 1.0]);
}

#[test]
fn g51_c_gfx_overlap_frag_cap_1hop_without_vtx_flush_fabrication() {
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    let Some((gfx_rs, gfx_pso)) = build_g48_vs_srv_ps_cb0_pso("g51_c_gfx") else { return };
    let vs_dxil =
        sign_dxil(include_bytes!("../../../../tests/fixtures/read_texture.vs.dxil"));
    let tex_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::Descriptor(RootDescriptorKind::Cbv, RootDescriptor {
                    shader_register: 0, register_space: 0, flags: 0,
                }),
                visibility: 5,
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv, num_descriptors: 1,
                    base_shader_register: 0, register_space: 0, flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 1,
            },
        ],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    }).unwrap();
    let mut blend = BlendDesc::default();
    blend.RenderTarget[0].RenderTargetWriteMask = 0xf;
    let tex_pso = Arc::new(GraphicsPipeline::new(
        tex_rs.clone(),
        &GraphicsDesc {
            vs: &vs_dxil, ps: Some(&cb0_color_ps_dxbc()), blend,
            sample_mask: u32::MAX, raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
            depth_stencil: DepthStencilState::default(), input_layout: Vec::new(),
            strip_cut: 0, topology_type: 3, rtv_formats: vec![RGBA8_UNORM], dsv_format: 0, sample_count: 1,
        },
        "g51_c_vs_texture",
    ).unwrap());
    assert_ne!(tex_pso.pre_raster_reads & crate::metal::pipeline::READS_TEX, 0);
    let vs_const_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let const_rs = RootSignature::new(RootSignatureDesc {
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
    let pso_no_vtx_reads = Arc::new(
        GraphicsPipeline::new(
            const_rs.clone(),
            &GraphicsDesc {
                vs: &vs_const_dxil,
                ps: None,
                blend: BlendDesc::default(),
                sample_mask: u32::MAX,
                raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
                depth_stencil: DepthStencilState {
                    depth_compare: MTLCompareFunction::Less,
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
            "g51_c_depth_noreads",
        )
        .unwrap(),
    );
    assert_eq!(pso_no_vtx_reads.pre_raster_reads, 0, "entry_v2 root-constants VS must have pre_raster_reads == 0");

    let hdesc = crate::types::HeapDesc {
        SizeInBytes: 4 * 1024 * 1024,
        Properties: test_heap(heap_type::DEFAULT),
        Alignment: 65536,
        Flags: 0,
    };
    let pheap = Arc::new(crate::metal::heap::Heap::new(&hdesc, "g51_c pheap").unwrap());
    let rtdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: RGBA8_UNORM,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let rt0 = Resource::new_placed(&pheap, 0, &rtdesc, MTLTextureUsage::empty(), "g51_c rt0").unwrap();
    let rt1 = Resource::new_placed(&pheap, 256 * 1024, &rtdesc, MTLTextureUsage::empty(), "g51_c rt1").unwrap();
    let rt2 = Resource::new_placed(&pheap, 512 * 1024, &rtdesc, MTLTextureUsage::empty(), "g51_c rt2").unwrap();
    let ds = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let rb_rt = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&rt0, &rt1, &rt2, &ds, &cb_up, &buf_vs, &rb_rt] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let rtv0 = TargetView::rtv(&rt0, None).unwrap();
    let rtv1 = TargetView::rtv(&rt1, None).unwrap();
    let rtv2 = TargetView::rtv(&rt2, None).unwrap();
    let dsv = TargetView::dsv(&ds, None).unwrap();
    let rt0_id = &rt0 as *const Resource as usize;
    let rt1_id = &rt1 as *const Resource as usize;
    let rt2_id = &rt2 as *const Resource as usize;

    let input = test_texture(1, 1, RGBA8_UNORM, 0);
    let upload = test_buffer(256, heap_type::UPLOAD, 0);
    unsafe { upload.buffer().unwrap().contents().as_ptr().cast::<u32>().write(0xff00_ff00) };
    input.register_ptr();
    let input_id = &input as *const Resource as usize;
    let heap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV, 1, true, "g51_c_vs_texture",
    ).unwrap();
    let entry = crate::metal::views::srv_entry(Some(&input), None).unwrap();
    unsafe { std::ptr::copy_nonoverlapping(entry.as_ptr(), heap.cpu_base as *mut u8, 24) };
    let q = Queue::new(ListType::Direct, "g51_c queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc).unwrap();
    let footprint = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 1, height: 1, depth: 1, row_pitch: 256, _pad: 0 };
    init.copy_texture_region(CopyLoc::Subresource(&input, 0), [0, 0, 0], CopyLoc::Footprint(&upload, footprint), None).unwrap();
    init.barrier_list_with(&[Barrier::Transition(input_id, STATE_COPY_DEST, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc.reset();

    let run_case = |enable_c: bool| -> (u64, u8, u8, u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_gfx_overlap_frag_cap = enable_c;
        let c0 = stats::c::gfx_overlap_frag_cap.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        unsafe { l.clear_dsv(&dsv, true, 0.9, false, 0, &[]).unwrap() };
        l.gfx_root.set_root_signature(gfx_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(gfx_pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // 1) Placed ListStart split (`PIXEL_SRV -> RENDER_TARGET`):
        l.barrier_list_with(&[Barrier::Transition(rt0_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
        l.set_render_targets(vec![rtv0], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        // 2) Hop 1 (`rt0: RT -> NON_PIXEL_SRV | PIXEL_SRV`, draw into `rt1`):
        l.barrier_list_with(&[Barrier::Transition(rt0_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)], true, false);
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.gfx_root.set_root_signature(tex_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_table(1, heap.gpu_base).unwrap();
        l.set_graphics_pipeline(tex_pso.clone());
        l.set_render_targets(vec![rtv1], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        let ch1 = l.rp_sync.gfx_ps_overlap_chain;

        // 3) Anti-fabrication check at `chain == 1`:
        l.barrier_list_with(&[Barrier::Transition(rt1_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
        let qb0 = stats::c::q_barrier_all.load(Relaxed) + stats::c::qb_frag_to_vtx.load(Relaxed);
        l.gfx_root.set_root_signature(const_rs.clone()).unwrap();
        l.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
        l.set_render_targets(Vec::new(), Some(dsv));
        l.set_graphics_pipeline(pso_no_vtx_reads.clone());
        l.draw(3, 1, 0, 0).unwrap();
        let qb_noread = (stats::c::q_barrier_all.load(Relaxed) + stats::c::qb_frag_to_vtx.load(Relaxed)) - qb0;
        l.end_encoder();

        // 4) Hop 2 with `need_vtx == true` at `chain == 1`:
        l.gfx_root.set_root_signature(tex_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_table(1, heap.gpu_base).unwrap();
        l.set_graphics_pipeline(tex_pso.clone());
        l.set_render_targets(vec![rtv2], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        let ch2 = l.rp_sync.gfx_ps_overlap_chain;

        l.barrier_list_with(&[Barrier::Transition(rt2_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp), [0, 0, 0], CopyLoc::Subresource(&rt2, 0), None).unwrap();
        l.close().unwrap();

        let c_d = stats::c::gfx_overlap_frag_cap.load(Relaxed) - c0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g51_C must produce 0 bad timestamps");
        alloc.reset();

        (c_d, ch1, ch2, qb_noread, buf_words(&rb_rt, 1)[0])
    };

    let (c_off, ch1_off, ch2_off, qb_off, px_off) = run_case(false);
    let (c_on, ch1_on, ch2_on, qb_on, px_on) = run_case(true);
    assert_eq!((c_off, ch1_off, ch2_off), (0, 0, 0));
    assert_eq!(qb_on, qb_off, "g51_C must never fabricate Fragment->PRE_RASTER flushes when pre_raster_reads == 0");
    assert_eq!(qb_on, 0);
    assert_eq!(ch1_on, 1, "disjoint t0 texture should spend one fragment deferral");
    assert_eq!(ch2_on, 0);
    assert!(c_on > 0, "placed ListStart must exercise the fragment-overlap path");
    assert_eq!(px_on, px_off);
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g51_d_cs_bind_intra_safe_tier1_gen_and_desc_copy_invalidation() {
    use crate::metal::descriptor::bump_desc_heap_gen;
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline, READS_BUF, READS_TEX};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,
        address_u: 3,
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Srv,
                    num_descriptors: 2,
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

    let mut pso_raw = ComputePipeline::new(rs.clone(), &dxil, "g51_d_cs").unwrap();
    pso_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: READS_BUF,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
    };
    let pso = Arc::new(pso_raw);

    let tex_a = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_b = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let b0 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b1 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb0 = test_buffer(512, heap_type::READBACK, 0);
    let rb1 = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &b0, &b1, &rb0, &rb1] {
        r.register_ptr();
    }
    let dheap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g51_d dheap",
    )
    .unwrap();
    let entry_a = crate::metal::views::srv_entry(Some(&tex_a), None).unwrap();
    let entry_b = crate::metal::views::srv_entry(Some(&tex_b), None).unwrap();
    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let b0_id = &b0 as *const Resource as usize;
    let b1_id = &b1 as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g51_d queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |enable_d: bool| {
        let mut list = CommandList::new(ListType::Direct).unwrap();
        list.opts.opt_cs_bind_intra_safe = enable_d;
        let hits = stats::c::cs_bind_intra_safe.load(Relaxed);
        for (entry, view, resource, output, output_id, readback, color) in [
            (&entry_a, &rtv_a, tex_a_id, &b0, b0_id, &rb0, [3.0, 6.0, 9.0, 1.0]),
            (&entry_b, &rtv_b, tex_b_id, &b1, b1_id, &rb1, [12.0, 15.0, 18.0, 1.0]),
        ] {
            // The preceding submission has finished consuming this shader-visible slot.
            unsafe { std::ptr::copy_nonoverlapping(entry.as_ptr(), dheap.cpu_base as *mut u8, 24) };
            bump_desc_heap_gen();
            if enable_d && resource == tex_b_id {
                let cache = &list.binds.cs_bind_cache;
                let matches = |epoch| cache.matches_gen(
                    cache.pso_ptr, cache.want_spans, cache.reg_gen,
                    epoch, cache.root_gen, cache.heap0_base,
                );
                assert!(matches(cache.desc_gen));
                assert!(!matches(crate::metal::descriptor::desc_heap_gen()));
            }

            list.begin(&alloc).unwrap();
            list.pt_armed = true;
            list.set_descriptor_heaps(dheap.gpu_base, 0);
            list.compute_root.set_root_signature(rs.clone()).unwrap();
            list.compute_root.set_table(0, dheap.gpu_base).unwrap();
            list.compute_root.set_descriptor(1, output.gpu_va).unwrap();
            list.set_compute_pipeline(pso.clone());
            unsafe { list.encode_rtv_clear_pass(view, color).unwrap() };
            list.barrier_list_with(
                &[Barrier::Transition(resource, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false,
            );
            list.dispatch(1, 1, 1).unwrap();
            list.barrier_list_with(&[Barrier::Uav(output_id)], true, false);
            list.dispatch(1, 1, 1).unwrap();
            if enable_d {
                assert_eq!(list.binds.cs_bind_cache.reads.as_slice(), &[resource]);
            }
            list.barrier_list_with(
                &[Barrier::Transition(output_id, STATE_UAV, STATE_COPY_SOURCE)], true, false,
            );
            list.copy_buffer(readback, 0, output, 0, 16).unwrap();
            list.close().unwrap();
            let mut log = list.take_pass_log().unwrap();
            log.submitted(pt::mach_now(), 0, 0);
            q.submit(&[&list.cb]);
            q.wait_idle();
            assert_eq!(pt::resolve_for_test(vec![log]).bad, 0);
            assert_eq!(buf_words(readback, 4), color.map(f32::to_bits));
            alloc.reset();
        }
        stats::c::cs_bind_intra_safe.load(Relaxed) - hits
    };
    assert_eq!(run_case(false), 0);
    assert!(run_case(true) >= 2);

    let mut aliased = CommandList::new(ListType::Direct).unwrap();
    aliased.opts.opt_cs_bind_intra_safe = true;
    let previous = crate::metal::resource::swap_any_placed_alias_for_test(true);
    let use_extra_hazards = aliased.cs_bind_hazards_active();
    crate::metal::resource::swap_any_placed_alias_for_test(previous);
    assert!(!use_extra_hazards);
}

#[test]
fn g51_abcd_combined_smoke() {
    use crate::metal::pipeline::{
        register_shader_hk_profile_for_test, ComputeAccess, ComputeBind, ComputePipeline,
        HandleKindProfile, READS_BUF, READS_TEX,
    };
    use crate::metal::resource::{HK_CBV_BUF, HK_RAW_STRUCT_BUF, HK_TEX2D_FLT, HK_TEX3D_FLT};
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    let Some((gfx_rs, gfx_pso)) = build_g48_vs_srv_ps_cb0_pso("g51_combo_gfx") else { return };
    let cs_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));
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
                    num_descriptors: 2,
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

    let mut cs_raw = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g51_combo_cs").unwrap();
    cs_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: READS_BUF | READS_TEX,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
    };
    let cs_key = u128::MAX - 512;
    Arc::make_mut(&mut cs_raw.shader).key = cs_key;
    let cs_pso = Arc::new(cs_raw);
    register_shader_hk_profile_for_test(
        cs_key,
        HandleKindProfile {
            uav_write_kinds: HK_RAW_STRUCT_BUF,
            uav_srv_kinds: HK_RAW_STRUCT_BUF | HK_TEX2D_FLT | HK_TEX3D_FLT,
            ro_srv_kinds: HK_TEX2D_FLT | HK_TEX3D_FLT | HK_RAW_STRUCT_BUF,
            read_kinds: HK_TEX2D_FLT | HK_TEX3D_FLT | HK_RAW_STRUCT_BUF | HK_CBV_BUF,
            unbounded_srv_kinds: HK_RAW_STRUCT_BUF | HK_TEX3D_FLT,
        },
    );

    let hdesc = crate::types::HeapDesc {
        SizeInBytes: 4 * 1024 * 1024,
        Properties: test_heap(heap_type::DEFAULT),
        Alignment: 65536,
        Flags: 0,
    };
    let pheap = Arc::new(crate::metal::heap::Heap::new(&hdesc, "g51_combo pheap").unwrap());
    let tdesc_f32 = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 2,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let rtdesc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: RGBA8_UNORM,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let tex_a = Resource::new_placed(&pheap, 0, &tdesc_f32, MTLTextureUsage::empty(), "g51_combo tex_a").unwrap();
    let tex_b = Resource::new_placed(&pheap, 256 * 1024, &tdesc_f32, MTLTextureUsage::empty(), "g51_combo tex_b").unwrap();
    let rt_draw = Resource::new_placed(&pheap, 512 * 1024, &rtdesc, MTLTextureUsage::empty(), "g51_combo rt_draw").unwrap();
    let b_clear = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let b_dis = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b_out = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_out = test_buffer(512, heap_type::READBACK, 0);
    let rb_draw = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &rt_draw, &b_clear, &cb_up, &buf_vs, &b_dis, &b_out, &rb_out, &rb_draw] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let dheap = crate::metal::descriptor::DescriptorHeap::new(
        crate::metal::descriptor::heap_kind::CBV_SRV_UAV,
        8,
        true,
        "g51_combo dheap",
    )
    .unwrap();
    let uav_clear_desc = {
        let mut d = [0u8; 40];
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[16..20].copy_from_slice(&16u32.to_le_bytes());
        d[20..24].copy_from_slice(&4u32.to_le_bytes());
        d
    };
    unsafe {
        let eb = crate::metal::views::srv_entry(Some(&tex_b), None).unwrap();
        std::ptr::copy_nonoverlapping(eb.as_ptr(), dheap.cpu_base as *mut u8, 24);
        let ea = crate::metal::views::srv_entry(Some(&tex_a), None).unwrap();
        std::ptr::copy_nonoverlapping(ea.as_ptr(), (dheap.cpu_base + 48) as *mut u8, 24);
    }
    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let rtv_draw = TargetView::rtv(&rt_draw, None).unwrap();
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let rt_draw_id = &rt_draw as *const Resource as usize;
    let b_out_id = &b_out as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g51_combo queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_b, [6.5, 7.5, 8.5, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_b_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_combo = |on: bool| -> ([f32; 4], u32, [u64; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_sm66_handle_kind_v2 = on;
        l.opts.opt_ps_to_cs_hk_overlap = false;
        l.opts.opt_gfx_overlap_frag_cap = on;
        l.opts.opt_cs_bind_intra_safe = on;
        let c0 = [
            stats::c::sm66_handle_kind_v2.load(Relaxed),
            stats::c::ps_to_cs_hk_overlap.load(Relaxed),
            stats::c::gfx_overlap_frag_cap.load(Relaxed),
            stats::c::cs_bind_intra_safe.load(Relaxed),
        ];

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.set_descriptor_heaps(dheap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.set_compute_pipeline(cs_pso.clone());
        l.gfx_root.set_root_signature(gfx_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(gfx_pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // Placed ListStart split (`g51_C`):
        l.barrier_list_with(&[Barrier::Transition(rt_draw_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
        l.set_render_targets(vec![rtv_draw], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        // `clear_uav` (`g51_A` exact HK) + repeated CS dispatches (`g51_D` Tier-1 CsBindCache):
        l.clear_uav(
            &b_clear,
            Some(&uav_clear_desc),
            ClearValue::Uint([3, 3, 3, 3]),
            &[],
        )
        .unwrap();
        l.set_compute_pipeline(cs_pso.clone());
        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.barrier_list_with(&[Barrier::Uav(0)], true, false);
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        // Enable `g51_B` too so all 4 levers run simultaneously on the placed RTV -> CS overlap:
        l.opts.opt_ps_to_cs_hk_overlap = on;
        unsafe { l.encode_rtv_clear_pass(&rtv_a, [2.75, 3.75, 4.75, 1.0]).unwrap() };
        l.end_encoder();
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
                Barrier::Uav(0),
            ],
            true,
            false,
        );
        l.compute_root.set_table(0, dheap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, b_dis.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.compute_root.set_table(0, dheap.gpu_base + 48).unwrap();
        l.compute_root.set_descriptor(1, b_out.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        l.barrier_list_with(
            &[
                Barrier::Transition(b_out_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(rt_draw_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb_out, 0, &b_out, 0, 16).unwrap();
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_draw, fp), [0, 0, 0], CopyLoc::Subresource(&rt_draw, 0), None).unwrap();
        l.close().unwrap();

        let cd = [
            stats::c::sm66_handle_kind_v2.load(Relaxed) - c0[0],
            stats::c::ps_to_cs_hk_overlap.load(Relaxed) - c0[1],
            stats::c::gfx_overlap_frag_cap.load(Relaxed) - c0[2],
            stats::c::cs_bind_intra_safe.load(Relaxed) - c0[3],
        ];
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g51 combined must produce 0 bad timestamps");
        alloc.reset();

        let vo: Vec<f32> = buf_words(&rb_out, 4).into_iter().map(f32::from_bits).collect();
        ([vo[0], vo[1], vo[2], vo[3]], buf_words(&rb_draw, 1)[0], cd)
    };

    let (vo_off, pd_off, cd_off) = run_combo(false);
    let (vo_on, pd_on, cd_on) = run_combo(true);
    assert_eq!(cd_off, [0, 0, 0, 0]);
    assert!(
        cd_on.iter().all(|&c| c >= 1),
        "all four g51 counters must fire in g51_abcd_combined_smoke: got {cd_on:?}"
    );
    assert_eq!(vo_on, vo_off);
    assert_eq!(pd_on, pd_off);
    assert_eq!(vo_on, [2.75, 3.75, 4.75, 1.0]);
    assert_eq!(pd_on, 0xff00_ff00);
}

#[test]
fn generate_mips_2d_and_array_and_write_buffer_u32_on_gpu() {
    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "genmips test q").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut cl = CommandList::new(ListType::Direct).unwrap();

    // 1. write_buffer_u32 at offset 0 and offset 12
    let dst_buf = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_buf = test_buffer(64, heap_type::READBACK, 0);

    // 2. 4x4 3-mip 2D texture (RGBA8_UNORM = 28)
    let td2d = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 3,
        Format: 28,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let tex2d = Resource::new_committed_default(&td2d, &test_heap(heap_type::DEFAULT), "genmips 2d").unwrap();
    let (lays2d, tot2d) = crate::layout::copyable_footprints(&tex2d.desc, 0, 3, 0).unwrap();
    let up2d = test_buffer(tot2d.max(256), heap_type::UPLOAD, 0);
    let rb2d = test_buffer(tot2d.max(256), heap_type::READBACK, 0);

    // Fill 4x4 mip 0 with 4 solid 2x2 quadrants: TL=Red, TR=Green, BL=Blue, BR=White.
    let red = 0xFF00_00FFu32;
    let green = 0xFF00_FF00u32;
    let blue = 0xFFFF_0000u32;
    let white = 0xFFFF_FFFFu32;
    unsafe {
        let base = up2d.cpu_ptr().unwrap().add(lays2d[0].footprint.offset as usize);
        let rp = lays2d[0].footprint.row_pitch as usize;
        for y in 0..4usize {
            let row = std::slice::from_raw_parts_mut(base.add(y * rp) as *mut u32, 4);
            for x in 0..4usize {
                row[x] = match (x >= 2, y >= 2) {
                    (false, false) => red,
                    (true, false) => green,
                    (false, true) => blue,
                    (true, true) => white,
                };
            }
        }
    }

    // 3. 2-slice 4x4 3-mip Texture2DArray (slice 0 = red, slice 1 = blue)
    let tdarr = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 2,
        MipLevels: 3,
        Format: 28,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let tex_arr = Resource::new_committed_default(&tdarr, &test_heap(heap_type::DEFAULT), "genmips 2da").unwrap();
    let (lays_arr, tot_arr) = crate::layout::copyable_footprints(&tex_arr.desc, 0, 6, 0).unwrap();
    let rb_arr = test_buffer(tot_arr.max(256), heap_type::READBACK, 0);

    cl.begin(&alloc).unwrap();
    cl.write_buffer_u32(&dst_buf, 0, 0x1234_5678).unwrap();
    cl.write_buffer_u32(&dst_buf, 12, 0xCAFE_BABE).unwrap();
    cl.copy_buffer(&rb_buf, 0, &dst_buf, 0, 16).unwrap();

    cl.copy_texture_region(CopyLoc::Subresource(&tex2d, 0), [0, 0, 0], CopyLoc::Footprint(&up2d, lays2d[0].footprint), None).unwrap();
    cl.generate_mips(&tex2d, 28, 0, u32::MAX, 0, 1).unwrap();
    for mip in 0..3u32 {
        cl.copy_texture_region(CopyLoc::Footprint(&rb2d, lays2d[mip as usize].footprint), [0, 0, 0], CopyLoc::Subresource(&tex2d, mip), None).unwrap();
    }

    // Clear slice 0 mip 0 to red and slice 1 mip 0 to blue via deferred RTV clears, then generate_mips!
    let rtv0 = TargetView::rtv(&tex_arr, Some(&[28, 5, 0, 0, 1, 0])).unwrap();
    let rtv1 = TargetView::rtv(&tex_arr, Some(&[28, 5, 0, 1, 1, 0])).unwrap();
    unsafe {
        cl.clear_rtv(&rtv0, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
        cl.clear_rtv(&rtv1, [0.0, 0.0, 1.0, 1.0], &[]).unwrap();
    }
    cl.generate_mips(&tex_arr, 28, 0, u32::MAX, 0, 2).unwrap();
    for sub in 0..6u32 {
        cl.copy_texture_region(CopyLoc::Footprint(&rb_arr, lays_arr[sub as usize].footprint), [0, 0, 0], CopyLoc::Subresource(&tex_arr, sub), None).unwrap();
    }

    cl.close().unwrap();
    q.submit(&[&cl.cb]);
    q.wait_idle();

    let bw = buf_words(&rb_buf, 4);
    assert_eq!(bw[0], 0x1234_5678);
    assert_eq!(bw[3], 0xCAFE_BABE);

    unsafe {
        let p2d = rb2d.cpu_ptr().unwrap();
        // Mip 1 (2x2): exact 4 quadrants
        let m1_base = p2d.add(lays2d[1].footprint.offset as usize);
        let m1_rp = lays2d[1].footprint.row_pitch as usize;
        let row0 = std::slice::from_raw_parts(m1_base as *const u32, 2);
        let row1 = std::slice::from_raw_parts(m1_base.add(m1_rp) as *const u32, 2);
        assert_eq!(row0, &[red, green], "mip 1 row 0");
        assert_eq!(row1, &[blue, white], "mip 1 row 1");
        // Mip 2 (1x1): average of (R,G,B,W) = (128, 128, 128, 255) +- 1
        let m2_px = *(p2d.add(lays2d[2].footprint.offset as usize) as *const u32);
        let (r, g, b, a) = (m2_px & 0xFF, (m2_px >> 8) & 0xFF, (m2_px >> 16) & 0xFF, (m2_px >> 24) & 0xFF);
        assert!((127..=128).contains(&r) && (127..=128).contains(&g) && (127..=128).contains(&b) && a == 255, "mip 2 = ({r},{g},{b},{a})");

        // Texture2DArray: slice 0 mips 0..2 == red, slice 1 mips 0..2 == blue
        let parr = rb_arr.cpu_ptr().unwrap();
        for mip in 0..3usize {
            let s0 = *(parr.add(lays_arr[mip].footprint.offset as usize) as *const u32);
            let s1 = *(parr.add(lays_arr[3 + mip].footprint.offset as usize) as *const u32);
            assert_eq!(s0, red, "array slice 0 mip {mip}");
            assert_eq!(s1, blue, "array slice 1 mip {mip}");
        }
    }
}
