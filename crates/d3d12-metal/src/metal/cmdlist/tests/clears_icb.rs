//! Clear overlap, descriptor-table exactness, ICB prepass, and g44–g48 GPU tests.

use super::*;

#[test]
fn g44_stage_b_clear_prelist_overlap_parallelizes_flushed_dsv_clears_with_opening_compute_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::ComputePipeline;
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
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g44_b_cs").unwrap());

    let q = Queue::new(ListType::Direct, "g44_b queue").unwrap();
    let alloc_pre = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();

    let tex_srv = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let ds = test_texture(64, 64, 40, resource_flags::ALLOW_DEPTH_STENCIL);
    let uav = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_uav = test_buffer(256, heap_type::READBACK, 0);
    let rb_ds = test_buffer(256 * 64, heap_type::READBACK, 0);
    for r in [&tex_srv, &ds, &uav, &rb_uav, &rb_ds] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g44_b heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex_srv), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rtv_srv = TargetView::rtv(&tex_srv, None).unwrap();
    let dsv = TargetView::dsv(&ds, None).unwrap();
    let tex_srv_id = &tex_srv as *const Resource as usize;
    let ds_id = &ds as *const Resource as usize;
    let uav_id = &uav as *const Resource as usize;

    let _guard = gpu_stats_lock();

    // Pre-list `l0` writes `0.25` to `ds` and `[9.0, 18.0, 27.0, 1.0]` to `tex_srv` so `l1`
    // MUST protect pre-list->in-list ordering while overlapping `L1#1` `CLEAR` with `L1#0` `Compute`!
    let run_case = |overlap_on: bool| -> (MTLStages, u64, u32, Vec<f32>) {
        let mut l0 = CommandList::new(ListType::Direct).unwrap();
        l0.begin(&alloc_pre).unwrap();
        unsafe {
            l0.encode_dsv_clear_pass(&dsv, Some(0.25), None).unwrap();
            l0.encode_rtv_clear_pass(&rtv_srv, [9.0, 18.0, 27.0, 1.0]).unwrap();
        }
        l0.barrier_list_with(&[Barrier::Transition(tex_srv_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
        l0.close().unwrap();

        let mut l1 = CommandList::new(ListType::Direct).unwrap();
        l1.opts.opt_clear_prelist_overlap = overlap_on;
        l1.opts.opt_clear_keep_dsv = false; // ensure `Barrier::Uav(0)` flushes the DSV clear like `L1#0 -> L1#1`!
        let cpo0 = stats::c::clear_prelist_overlap.load(Relaxed);

        l1.begin(&alloc).unwrap();
        l1.pt_armed = true;
        // Queue deferred DSV clear (`0.75`) BEFORE `L1#0` compute opens (exact `L1` frame pattern!):
        unsafe { l1.clear_dsv(&dsv, true, 0.75, false, 0, &[]).unwrap() };
        assert_eq!(l1.clears.pending_clears.len(), 1);

        // `L1#0` compute opens and runs a normal `ComputePipeline` dispatch:
        l1.set_descriptor_heaps(heap.gpu_base, 0);
        l1.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l1.compute_root.set_table(0, heap.gpu_base).unwrap();
        l1.compute_root.set_descriptor(1, uav.gpu_va).unwrap();
        l1.set_compute_pipeline(cs_pso.clone());
        l1.dispatch(1, 1, 1).unwrap();
        if overlap_on {
            assert!(!l1.bar.list_unwaited_gfx, "g44_B must fence All -> All at L1#0 open so pending clears do not stall behind L1#0");
        } else {
            assert!(l1.bar.list_unwaited_gfx);
        }

        // `Barrier::Uav(0)` flushes `L1#1` DSV `CLEAR` pass:
        l1.barrier_list_with(&[Barrier::Uav(0)], true, false);
        let clear_sync_after = l1.bar.enc_sync_after;
        let cpo_delta = stats::c::clear_prelist_overlap.load(Relaxed) - cpo0;

        l1.barrier_list_with(
            &[
                Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE),
                Barrier::Transition(uav_id, STATE_UAV, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        let fp = PlacedFootprint { offset: 0, format: 40, width: 64, height: 64, depth: 1, row_pitch: 256, _pad: 0 };
        l1.copy_texture_region(CopyLoc::Footprint(&rb_ds, fp), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
        l1.copy_buffer(&rb_uav, 0, &uav, 0, 64).unwrap();
        l1.close().unwrap();

        let mut log = l1.take_pass_log().unwrap();
        log.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l0.cb, &l1.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g44_B must produce 0 bad timestamps");
        alloc_pre.reset();
        alloc.reset();
        let vu: Vec<f32> = buf_words(&rb_uav, 4).into_iter().map(f32::from_bits).collect();
        (clear_sync_after, cpo_delta, buf_words(&rb_ds, 1)[0], vu)
    };

    let (sync_off, cpo_off, ds_off, uav_off) = run_case(false);
    let (sync_on, cpo_on, ds_on, uav_on) = run_case(true);
    assert_eq!(sync_off, MTLStages::Dispatch, "without g44_B, L1#1 CLEAR pass stalls on L1#0 Dispatch");
    assert_eq!(cpo_off, 0);
    assert_eq!(sync_on, MTLStages::empty(), "with g44_B, L1#1 CLEAR pass opens with sync=0 (parallel with L1#0!)");
    assert_eq!(cpo_on, 1);
    assert_eq!(ds_off, 0.75f32.to_bits());
    assert_eq!(ds_on, 0.75f32.to_bits());
    assert_eq!(&uav_off[..], &[9.0, 18.0, 27.0, 1.0]);
    assert_eq!(&uav_on[..], &[9.0, 18.0, 27.0, 1.0]);
}

#[test]
fn g44_stage_c_cs_mid_qb_unpoison_v4_unpoisons_blit_substage_and_common_and_uav0_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::ComputePipeline;
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
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g44_c_cs").unwrap());

    let tex_a = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_b = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_blit_pending = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let uav_a = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_mid = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_d = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_b = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_d = test_buffer(512, heap_type::READBACK, 0);
    let rb_b = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &tex_blit_pending, &uav_a, &uav_mid, &uav_d, &uav_b, &rb_d, &rb_b] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g44_c heap").unwrap();
    let stride: usize = 24;
    unsafe {
        let ea = crate::metal::views::srv_entry(Some(&tex_a), None).unwrap();
        let eb = crate::metal::views::srv_entry(Some(&tex_b), None).unwrap();
        std::ptr::copy_nonoverlapping(ea.as_ptr(), heap.cpu_base as *mut u8, 24);
        std::ptr::copy_nonoverlapping(eb.as_ptr(), (heap.cpu_base + stride) as *mut u8, 24);
    }
    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let rtv_blit = TargetView::rtv(&tex_blit_pending, None).unwrap();
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let tex_blit_id = &tex_blit_pending as *const Resource as usize;
    let uav_a_id = &uav_a as *const Resource as usize;
    let uav_d_id = &uav_d as *const Resource as usize;
    let uav_b_id = &uav_b as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g44_c queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    // Reproduce all 3 root causes that made `g43_C` log `0.0` in gameplay:
    // (1) Residual `split_pend.blit_after` (`RENDER_TARGET -> COPY_SOURCE`) still pending when Compute opens for `Dispatch`;
    // (2) Mid-compute batch mixing `Barrier::Uav(0)` (already Dispatch-satisfied!) + `Transition(tex_b, RENDER_TARGET, COMMON)` (`s1 == 0`)!
    let run_case = |v4_on: bool| -> (u64, u64, u64, Vec<f32>, Vec<f32>) {
        let mut l_init = CommandList::new(ListType::Direct).unwrap();
        l_init.begin(&alloc_init).unwrap();
        unsafe { l_init.encode_rtv_clear_pass(&rtv_a, [2.0, 4.0, 6.0, 1.0]).unwrap() };
        l_init.barrier_list_with(&[Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
        l_init.close().unwrap();

        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_cs_mid_qb_defer_v3 = true;
        l.opts.opt_cs_mid_qb_unpoison_v4 = v4_on;
        let v4_0 = stats::c::cs_mid_qb_v4.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, uav_a.gpu_va).unwrap();
        l.set_compute_pipeline(cs_pso.clone());
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();
        l.barrier_list_with(&[Barrier::Transition(uav_a_id, STATE_UAV, STATE_NON_PIXEL_SRV)], true, false);

        // Render passes produce `tex_blit_pending` (transitioned to `COPY_SOURCE` -> `split_pend.blit_after != empty`!)
        // and `tex_b` (not transitioned yet -> `stage_gen[Fragment]` advances!).
        l.bar.in_list_gfx_non_clear = true;
        unsafe {
            l.encode_rtv_clear_pass(&rtv_blit, [1.0, 0.0, 0.0, 1.0]).unwrap();
            l.encode_rtv_clear_pass(&rtv_b, [6.0, 12.0, 18.0, 1.0]).unwrap();
        }
        l.barrier_list_with(&[Barrier::Transition(tex_blit_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        assert!(!l.bar.split_pend.blit_after.is_empty());

        // Compute #1 opens for `Dispatch` (`split_pend.blit_after` remains non-empty!):
        l.compute_root.set_descriptor(1, uav_mid.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        if v4_on {
            assert!(!l.bar.cs_mid_v3_poisoned[0], "g44_C must keep Dispatch unpoisoned when only Blit has residual split_pend");
            assert!(l.bar.cs_mid_v3_poisoned[1], "g44_C must keep Blit poisoned while split_pend.blit_after is pending");
        } else {
            assert!(l.bar.cs_mid_v3_poisoned[0], "legacy g43_C poisoned all substages whenever split_pend.has_compute()");
        }

        // Mid-compute batch mixing `Barrier::Uav(0)` + `Transition(tex_b, RENDER_TARGET, COMMON)` (`s1 == 0`)!
        l.barrier_list_with(
            &[
                Barrier::Uav(0),
                Barrier::Transition(tex_b_id, STATE_RENDER_TARGET, STATE_COMMON),
            ],
            true,
            false,
        );

        // Disjoint Dispatch #2 (still reading `tex_a`, NOT `tex_b`; writing `uav_d` disjoint from `uav_mid` and `uav_b`):
        let qbm0 = stats::c::cs_mid_qb.load(Relaxed);
        l.compute_root.set_descriptor(1, uav_d.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let qbm_disjoint = stats::c::cs_mid_qb.load(Relaxed) - qbm0;
        let v4_delta = stats::c::cs_mid_qb_v4.load(Relaxed) - v4_0;

        // Positive safety guardrail: Dispatch #3 reading `tex_b` (`heap.gpu_base + stride`) and writing `uav_b` MUST emit `cs_mid_qb`!
        let qbm1 = stats::c::cs_mid_qb.load(Relaxed);
        l.compute_root.set_table(0, heap.gpu_base + stride as u64).unwrap();
        l.compute_root.set_descriptor(1, uav_b.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let qbm_touch = stats::c::cs_mid_qb.load(Relaxed) - qbm1;

        l.barrier_list_with(
            &[
                Barrier::Transition(uav_d_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(uav_b_id, STATE_UAV, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb_d, 0, &uav_d, 0, 64).unwrap();
        l.copy_buffer(&rb_b, 0, &uav_b, 0, 64).unwrap();
        l.close().unwrap();

        q.submit(&[&l_init.cb, &l.cb]);
        q.wait_idle();
        alloc_init.reset();
        alloc.reset();
        let vd: Vec<f32> = buf_words(&rb_d, 4).into_iter().map(f32::from_bits).collect();
        let vb: Vec<f32> = buf_words(&rb_b, 4).into_iter().map(f32::from_bits).collect();
        (qbm_disjoint, qbm_touch, v4_delta, vd, vb)
    };

    let (disjoint_v3, touch_v3, v4_off, ref_d, ref_b) = run_case(false);
    let (disjoint_v4, touch_v4, v4_on_cnt, opt_d, opt_b) = run_case(true);
    assert_eq!(disjoint_v3, 1, "legacy g43_C was poisoned and fired cs_mid_qb on disjoint dispatch");
    assert_eq!(touch_v3, 0);
    assert_eq!(v4_off, 0);
    assert_eq!(disjoint_v4, 0, "g44_C must defer mid-CS queue barrier across disjoint dispatch");
    assert_eq!(touch_v4, 1, "g44_C must emit exact Fragment -> Dispatch mid-CS barrier when tex_b is bound");
    assert!(v4_on_cnt >= 1, "cs_mid_qb_v4 counter must increment");
    assert_eq!(opt_d, ref_d);
    assert_eq!(&opt_d[..], &[2.0, 4.0, 6.0, 1.0]);
    assert_eq!(opt_b, ref_b);
    assert_eq!(&opt_b[..], &[6.0, 12.0, 18.0, 1.0]);
}

#[test]
fn g44_stage_d_ps_srv_class_exact_separates_cbv_ia_from_srv_uav_and_keeps_ro_buf_common_on_gpu() {
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline, READS_BUF, READS_TEX};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootDescriptor, RootDescriptorKind, RootParameter, RootParameterKind,
        RootSignature, RootSignatureDesc, RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let ps_cb0_dxbc = cb0_color_ps_dxbc();
    let vs_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    // Root CBV at `b0` (`ps_read_classes == READS_BUF`, `ps_srv_uav_classes == 0`!):
    let cbv_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Descriptor(
                RootDescriptorKind::Cbv,
                RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
            ),
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let pso_cbv_only = Arc::new(
        GraphicsPipeline::new(
            cbv_rs.clone(),
            &GraphicsDesc {
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
            },
            "g44_d_cbv_only",
        )
        .unwrap(),
    );
    assert_eq!(pso_cbv_only.ps_read_classes, READS_BUF, "root CBV PS must set ps_read_classes == READS_BUF");
    assert_eq!(pso_cbv_only.ps_srv_uav_classes, 0, "root CBV PS without SRV/UAV buffers must have ps_srv_uav_classes == 0");

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let ib_up = test_buffer(64, heap_type::UPLOAD, 0);
    let srv_buf = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let ro_common_buf = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_write_buf = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &ib_up, &srv_buf, &ro_common_buf, &uav_write_buf, &rt, &rb] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
        let ip = ib_up.buffer().unwrap().contents().as_ptr() as *mut u32;
        *ip.add(0) = 0;
        *ip.add(1) = 1;
        *ip.add(2) = 2;
    }
    let srv_buf_id = &srv_buf as *const Resource as usize;
    let ro_common_id = &ro_common_buf as *const Resource as usize;
    let uav_write_id = &uav_write_buf as *const Resource as usize;
    let cb_up_id = &cb_up as *const Resource as usize;
    let rt_id = &rt as *const Resource as usize;
    let rtv = TargetView::rtv(&rt, None).unwrap();

    let q = Queue::new(ListType::Direct, "g44_d queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |exact_on: bool| -> (u32, u32, bool, bool, u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_ps_srv_class_exact = exact_on;
        l.opts.opt_gfx_desc_table_exact = false;
        l.opts.opt_rp_split_exact_keep = false;
        let pse0 = stats::c::ps_srv_class_exact.load(Relaxed);

        l.begin(&alloc).unwrap();
        // Indexed draw (`eff_reads` has `READS_BUF` from IB!) with `pso_cbv_only` (`ps_read_classes == READS_BUF`,
        // `ps_srv_uav_classes == 0`, `sm66_pre_raster_reads == 0`):
        l.gfx_root.set_root_signature(cbv_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.set_render_targets(vec![rtv], None);
        l.gfx.topology = 4;
        l.gfx.ib = Some((ib_up.gpu_va, 12, R32_UINT));
        l.set_graphics_pipeline(pso_cbv_only.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
        l.draw_indexed(3, 1, 0, 0, 0).unwrap();

        // 1) Kept `NON_PIXEL_SRV | PIXEL_SRV -> UAV` on `srv_buf` (pure SRV state, NO `STATE_VB_CB | STATE_IB`!):
        l.barrier_list_with(
            &[Barrier::Transition(srv_buf_id, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV, STATE_UAV)],
            true,
            false,
        );
        let kw_srv_touch = l.rp_kept_item_touch_stages(&l.rp_sync.rp_kept_writes[0], MTLStages::Vertex | MTLStages::Fragment, false);

        // 2) Kept `NON_PIXEL_SRV -> COMMON` (`s1 == 0`, read-only `s0`) on `ro_common_buf`:
        l.barrier_list_with(
            &[Barrier::Transition(ro_common_id, STATE_NON_PIXEL_SRV, STATE_COMMON)],
            true,
            false,
        );
        let kept_after_ro_common = l.gfx.draw_pass;

        // 3) Positive safety guardrail A: `VB_CB -> UAV` on `cb_up` MUST bump `Fragment` & `Vertex`!
        if !l.gfx.draw_pass {
            l.draw_indexed(3, 1, 0, 0, 0).unwrap();
        }
        l.barrier_list_with(&[Barrier::Transition(cb_up_id, STATE_VB_CB, STATE_UAV)], true, false);
        assert!(l.gfx.draw_pass);
        let kw_cbv_touch = l.rp_kept_item_touch_stages(l.rp_sync.rp_kept_writes.last().unwrap(), MTLStages::Vertex | MTLStages::Fragment, false);

        // 4) Positive safety guardrail B: `UAV -> COMMON` (`s0` is a WRITE state!) MUST still split the pass!
        let keeps_uav_to_common = l.render_barrier_keeps(&[Barrier::Transition(uav_write_id, STATE_UAV, STATE_COMMON)], true);

        l.end_encoder();
        let pse_delta = stats::c::ps_srv_class_exact.load(Relaxed) - pse0;

        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();

        q.submit(&[&l.cb]);
        q.wait_idle();
        alloc.reset();
        let _ = READS_TEX;
        (
            kw_srv_touch.bits() as u32,
            kw_cbv_touch.bits() as u32,
            kept_after_ro_common,
            keeps_uav_to_common,
            pse_delta,
            buf_words(&rb, 1)[0],
        )
    };

    let (srv_touch_off, cbv_touch_off, kept_ro_off, keeps_uav_off, pse_off, px_off) = run_case(false);
    let (srv_touch_on, cbv_touch_on, kept_ro_on, keeps_uav_on, pse_on, px_on) = run_case(true);
    assert_ne!(srv_touch_off, 0, "legacy ps_read_classes falsely touched Vertex | Fragment on SRV buffer transition");
    assert!(!kept_ro_off, "legacy render_barrier_keeps split render pass on NON_PIXEL_SRV -> COMMON buffer transition");
    assert_eq!(pse_off, 0);
    assert_eq!(srv_touch_on, 0, "g44_D must prove 0 render-stage touch on SRV buffer when VS/PS only use CBV+IB");
    assert_ne!(cbv_touch_off, 0);
    assert_eq!(cbv_touch_on, cbv_touch_off, "g44_D must still report Vertex | Fragment touch when s0 includes STATE_VB_CB");
    assert!(kept_ro_on, "g44_D must keep render pass open across read-only buffer -> COMMON transition");
    assert!(!keeps_uav_off && !keeps_uav_on, "UAV -> COMMON write-state buffer transition must still split render pass");
    assert!(pse_on >= 2, "ps_srv_class_exact must count both SRV touch narrowing and kept RO->COMMON transition");
    assert_eq!(px_off, 0xff00_ff00);
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g45_defaults_promote_ps_srv_class_exact_and_keep_g45_knobs_opt_in() {
    if std::env::var_os("AW2_PS_SRV_CLASS_EXACT").is_none() {
        assert!(opt_ps_srv_class_exact_env());
    }
    if std::env::var_os("AW2_IB_UAV0_UNBOUNDED_EXACT").is_none() {
        assert!(!opt_ib_uav0_unbounded_exact_env());
    }
    if std::env::var_os("AW2_CLEAR_PRELIST_OVERLAP").is_none() {
        assert!(!opt_clear_prelist_overlap_env());
    }
    if std::env::var_os("AW2_CS_MID_QB_UNPOISON_V4").is_none() {
        assert!(!opt_cs_mid_qb_unpoison_v4_env());
    }
    if std::env::var_os("AW2_DESC_TABLE_RES_EXACT").is_none() {
        assert!(!opt_desc_table_res_exact_env());
    }
    if std::env::var_os("AW2_GFX_DESC_TABLE_EXACT").is_none() {
        assert!(opt_gfx_desc_table_exact_env());
    }
    if std::env::var_os("AW2_CS_MID_QB_RES_DEFER_V5").is_none() {
        assert!(!opt_cs_mid_qb_res_defer_v5_env());
    }
    if std::env::var_os("PORT_ICB_ENC_BARRIER_EXACT").is_none() && std::env::var_os("AW2_ICB_ENC_BARRIER_EXACT").is_none() {
        assert!(opt_icb_enc_barrier_exact_env());
    }
    if let Some(l) = CommandList::new(ListType::Direct) {
        if std::env::var_os("AW2_PS_SRV_CLASS_EXACT").is_none() {
            assert!(l.opts.opt_ps_srv_class_exact);
        }
        if std::env::var_os("AW2_DESC_TABLE_RES_EXACT").is_none() {
            assert!(!l.opts.opt_desc_table_res_exact);
        }
        if std::env::var_os("AW2_GFX_DESC_TABLE_EXACT").is_none() {
            assert!(l.opts.opt_gfx_desc_table_exact);
        }
        if std::env::var_os("AW2_CS_MID_QB_RES_DEFER_V5").is_none() {
            assert!(!l.opts.opt_cs_mid_qb_res_defer_v5);
        }
        if std::env::var_os("PORT_ICB_ENC_BARRIER_EXACT").is_none() && std::env::var_os("AW2_ICB_ENC_BARRIER_EXACT").is_none() {
            assert!(l.opts.opt_icb_enc_barrier_exact);
        }
    }
}

#[test]
fn g45_stage_a_desc_table_res_exact_elides_disjoint_uav_and_bounded_class_barriers_and_guards_true_hazards_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
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

    // Model `transformCS` (`1f2c9f23402660a0`): exact UAV (`uav_slots = Some([RootVa(1)])`),
    // bounded texture SRV (`TableEntry(0, 0)`), and unbounded buffer SRV table (`unbounded_srv_classes = READS_BUF`,
    // `read_slots = None`, `heap_indexed_or_unknown = false`).
    let mut cp_partial = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g45_a_partial").unwrap();
    cp_partial.access = ComputeAccess {
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
        unbounded_srv_classes: READS_BUF,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF | READS_TEX,
        heap_indexed_or_unknown: false,
        touches_textures: true,
        msc_refined: true,
        msc_exact_classes: READS_BUF | READS_TEX,
    };
    let pso_partial = Arc::new(cp_partial);

    let tex_bound = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_common_other = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let up_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let uav_other = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_out = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb = test_buffer(256, heap_type::READBACK, 0);
    for r in [&tex_bound, &tex_common_other, &up_buf, &uav_other, &uav_out, &rb] {
        r.register_ptr();
    }
    unsafe {
        *(up_buf.buffer().unwrap().contents().as_ptr() as *mut u32) = 0x55aa_1122;
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g45_a heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex_bound), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rtv_bound = TargetView::rtv(&tex_bound, None).unwrap();
    let rtv_other = TargetView::rtv(&tex_common_other, None).unwrap();
    let tex_bound_id = &tex_bound as *const Resource as usize;
    let tex_other_id = &tex_common_other as *const Resource as usize;
    let uav_other_id = &uav_other as *const Resource as usize;
    let uav_out_id = &uav_out as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g45_a queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |exact_on: bool| -> (u64, u64, u64, Vec<f32>) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_desc_table_res_exact = exact_on;
        let cnt0 = stats::c::desc_table_cs_exact.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        unsafe {
            l.encode_rtv_clear_pass(&rtv_bound, [3.0, 6.0, 9.0, 1.0]).unwrap();
            l.encode_rtv_clear_pass(&rtv_other, [1.0, 2.0, 3.0, 1.0]).unwrap();
        }
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_bound_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
                Barrier::Transition(tex_other_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );

        // Open compute encoder with Blit upload to `uav_other` + Blit read from `tex_common_other`,
        // then `COPY_DEST -> UAV` (`OblKind::UavOnly`) + `COPY_SOURCE -> COMMON` (`OblKind::Any`):
        l.copy_buffer(&uav_other, 0, &up_buf, 0, 16).unwrap();
        let fp_tmp = PlacedFootprint { offset: 128, format: 2, width: 4, height: 1, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp_tmp), [0, 0, 0], CopyLoc::Subresource(&tex_common_other, 0), None).unwrap();
        l.barrier_list_with(
            &[
                Barrier::Transition(uav_other_id, STATE_COPY_DEST, STATE_UAV),
                Barrier::Transition(tex_other_id, STATE_COPY_SOURCE, STATE_COMMON),
            ],
            true,
            false,
        );

        // Dispatch #1 (`pso_partial` writing `uav_out` != `uav_other`, reading `tex_bound` != `tex_common_other`):
        let eb0 = stats::c::enc_barrier.load(Relaxed);
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, uav_out.gpu_va).unwrap();
        l.set_compute_pipeline(pso_partial.clone());
        l.dispatch(1, 1, 1).unwrap();
        let eb_disjoint = stats::c::enc_barrier.load(Relaxed) - eb0;

        // Positive safety guardrail: Dispatch #2 writing `uav_other` MUST fire the deferred `Blit -> Dispatch` `enc_barrier`!
        let eb1 = stats::c::enc_barrier.load(Relaxed);
        l.compute_root.set_descriptor(1, uav_other.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let eb_true_hazard = stats::c::enc_barrier.load(Relaxed) - eb1;
        let cnt_delta = stats::c::desc_table_cs_exact.load(Relaxed) - cnt0;

        l.barrier_list_with(&[Barrier::Transition(uav_out_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
        l.copy_buffer(&rb, 0, &uav_out, 0, 64).unwrap();
        l.close().unwrap();

        let mut log = l.take_pass_log().unwrap();
        log.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g45_A must produce 0 bad timestamps");
        alloc.reset();
        let vals: Vec<f32> = buf_words(&rb, 4).into_iter().map(f32::from_bits).collect();
        (eb_disjoint, eb_true_hazard, cnt_delta, vals)
    };

    let (disjoint_off, hazard_off, cnt_off, ref_vals) = run_case(false);
    let (disjoint_on, hazard_on, cnt_on, opt_vals) = run_case(true);
    assert_eq!(disjoint_off, 1, "legacy OblKind::Any fired enc_barrier on disjoint partial-exact dispatch");
    assert_eq!(hazard_off, 0);
    assert_eq!(cnt_off, 0);
    assert_eq!(disjoint_on, 0, "g45_A must skip false enc_barrier on disjoint UAV + bounded texture class");
    assert_eq!(hazard_on, 1, "g45_A must fire Blit -> Dispatch enc_barrier when uav_other is actually bound");
    assert!(cnt_on >= 2, "desc_table_cs_exact must count both OblKind::UavOnly and bounded-class Any elision");
    assert_eq!(opt_vals, ref_vals);
    assert_eq!(&opt_vals[..], &[3.0, 6.0, 9.0, 1.0]);
}

#[test]
fn g45_stage_b_gfx_desc_table_exact_resolves_per_resource_vs_ps_ia_binds_and_guards_bound_resources_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, GraphicsDesc, GraphicsPipeline, READS_BUF, READS_TEX};
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
    let gfx_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Cbv,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 0,
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
                visibility: 0,
            },
        ],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let pso_base = GraphicsPipeline::new(
        gfx_rs.clone(),
        &GraphicsDesc {
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
        },
        "g45_b_base",
    )
    .unwrap();
    // Model a graphics PSO whose PS reads root CBV `param 0` + descriptor-table SRV `TableEntry(1, 0)` (`READS_TEX`):
    let pso = Arc::new(GraphicsPipeline {
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
        ps_writes_uav: false,
        ps_read_classes: READS_BUF | READS_TEX,
        ps_srv_uav_classes: READS_TEX,
        writes_uav: false,
        uses_uav: false,
        sm66_pre_raster_reads: 0,
        sm66_vs_reads_cbv_or_ia: false,
        sm66_writes_uav: false,
        sm66_narrowed: true,
        vs_access: pso_base.vs_access.clone(),
        ps_access: Some(ComputeAccess {
            uav_slots: Some(Vec::new()),
            read_slots: Some(vec![
                ComputeBind::RootVa { param: 0 },
                ComputeBind::TableEntry { param: 1, byte_offset: 0 },
            ]),
            bounded_read_slots: vec![
                ComputeBind::RootVa { param: 0 },
                ComputeBind::TableEntry { param: 1, byte_offset: 0 },
            ],
            unbounded_srv_classes: 0,
            uav_classes: 0,
            read_classes: READS_BUF | READS_TEX,
            uav_srv_classes: READS_TEX,
            heap_indexed_or_unknown: false,
            touches_textures: true,
            msc_refined: true,
            msc_exact_classes: READS_BUF | READS_TEX,
        }),
        vs_hk_profile: pso_base.vs_hk_profile,
        ps_hk_profile: pso_base.ps_hk_profile,
        icb_desc: None,
        icb_pso: std::sync::OnceLock::new(),
    });

    let cb_bound = test_buffer(256, heap_type::UPLOAD, 0);
    let cb_unbound = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let ib_bound = test_buffer(64, heap_type::UPLOAD, 0);
    let tex_bound = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_unbound = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_bound, &cb_unbound, &ib_bound, &tex_bound, &tex_unbound, &rt, &rb] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_bound.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 1.0;
        *cp.add(1) = 0.0;
        *cp.add(2) = 1.0;
        *cp.add(3) = 1.0;
        let ip = ib_bound.buffer().unwrap().contents().as_ptr() as *mut u32;
        *ip.add(0) = 0;
        *ip.add(1) = 1;
        *ip.add(2) = 2;
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g45_b heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex_bound), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }
    let rtv_init = TargetView::rtv(&tex_bound, None).unwrap();
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let tex_bound_id = &tex_bound as *const Resource as usize;
    let tex_unbound_id = &tex_unbound as *const Resource as usize;
    let cb_bound_id = &cb_bound as *const Resource as usize;
    let cb_unbound_id = &cb_unbound as *const Resource as usize;
    let ib_bound_id = &ib_bound as *const Resource as usize;
    let rt_id = &rt as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g45_b queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |exact_on: bool| -> (u32, u32, u32, u32, u32, u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_gfx_desc_table_exact = exact_on;
        let cnt0 = stats::c::gfx_desc_table_exact.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        unsafe { l.encode_rtv_clear_pass(&rtv_init, [0.5, 0.5, 0.5, 1.0]).unwrap() };
        l.barrier_list_with(&[Barrier::Transition(tex_bound_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV)], true, false);

        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.gfx_root.set_root_signature(gfx_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_bound.gpu_va).unwrap();
        l.gfx_root.set_table(1, heap.gpu_base).unwrap();
        l.set_render_targets(vec![rtv], None);
        l.gfx.topology = 4;
        l.gfx.ib = Some((ib_bound.gpu_va, 12, R32_UINT));
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
        l.draw_indexed(3, 1, 0, 0, 0).unwrap();

        // 1) Kept transitions on UNBOUND `tex_unbound` (`PIXEL_SRV -> RENDER_TARGET`) and UNBOUND `cb_unbound` (`VB_CB -> UAV`):
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_unbound_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET),
                Barrier::Transition(cb_unbound_id, STATE_VB_CB, STATE_UAV),
            ],
            true,
            false,
        );
        let t_unbound_tex = l.rp_kept_item_touch_stages(&l.rp_sync.rp_kept_writes[0], MTLStages::Vertex | MTLStages::Fragment, false);
        let t_unbound_buf = l.rp_kept_item_touch_stages(&l.rp_sync.rp_kept_writes[1], MTLStages::Vertex | MTLStages::Fragment, false);

        // 2) Positive safety guardrails on BOUND `tex_bound`, BOUND `cb_bound`, and BOUND `ib_bound`:
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_bound_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET),
                Barrier::Transition(cb_bound_id, STATE_VB_CB, STATE_COPY_DEST),
                Barrier::Transition(ib_bound_id, STATE_IB, STATE_COPY_DEST),
            ],
            true,
            false,
        );
        let t_bound_tex = l.rp_kept_item_touch_stages(&l.rp_sync.rp_kept_writes[2], MTLStages::Vertex | MTLStages::Fragment, false);
        let t_bound_cb = l.rp_kept_item_touch_stages(&l.rp_sync.rp_kept_writes[3], MTLStages::Vertex | MTLStages::Fragment, false);
        let t_bound_ib = l.rp_kept_item_touch_stages(&l.rp_sync.rp_kept_writes[4], MTLStages::Vertex | MTLStages::Fragment, false);

        l.end_encoder();
        let cnt_delta = stats::c::gfx_desc_table_exact.load(Relaxed) - cnt0;

        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();

        let mut log = l.take_pass_log().unwrap();
        log.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g45_B must produce 0 bad timestamps");
        alloc.reset();
        (
            t_unbound_tex.bits() as u32,
            t_unbound_buf.bits() as u32,
            t_bound_tex.bits() as u32,
            t_bound_cb.bits() as u32,
            t_bound_ib.bits() as u32,
            cnt_delta,
            buf_words(&rb, 1)[0],
        )
    };

    let (ut_off, ub_off, bt_off, _bc_off, bi_off, cnt_off, px_off) = run_case(false);
    let (ut_on, ub_on, bt_on, bc_on, bi_on, cnt_on, px_on) = run_case(true);
    assert_ne!(ut_off, 0, "class-only tracking falsely reported Fragment touch on unbound texture");
    assert_ne!(ub_off, 0, "class-only tracking falsely reported Vertex|Fragment touch on unbound buffer");
    assert_eq!(cnt_off, 0);
    assert_eq!(ut_on, 0, "g45_B must prove 0 render-stage touch on unbound texture");
    assert_eq!(ub_on, 0, "g45_B must prove 0 render-stage touch on unbound CBV buffer");
    assert_eq!(bt_on, bt_off, "g45_B must preserve Fragment touch on bound SRV texture");
    assert_eq!(bc_on, MTLStages::Fragment.bits() as u32, "g45_B must preserve Fragment touch on bound PS root CBV");
    assert_eq!(bi_on, bi_off, "g45_B must preserve Vertex touch on bound index buffer");
    assert!(cnt_on >= 2, "gfx_desc_table_exact must increment for narrowed kept transitions");
    assert_eq!(px_off, 0xffff_00ff);
    assert_eq!(px_on, 0xffff_00ff);
}

#[test]
fn g45_stage_c_cs_mid_qb_res_defer_v5_survives_blit_substage_clear_cs_and_partitions_mixed_batches_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::ComputePipeline;
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
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g45_c_cs").unwrap());

    let tex_safe = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_defer = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let rt_sw = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let up_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let blit_dst = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_init = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_d1 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_d2 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb1 = test_buffer(512, heap_type::READBACK, 0);
    let rb2 = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex_safe, &tex_defer, &rt_sw, &up_buf, &blit_dst, &uav_init, &uav_d1, &uav_d2, &rb1, &rb2] {
        r.register_ptr();
    }
    unsafe {
        *(up_buf.buffer().unwrap().contents().as_ptr() as *mut u32) = 0x1234_5678;
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g45_c heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex_safe), None).unwrap();
        let e1 = crate::metal::views::srv_entry(Some(&tex_defer), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        std::ptr::copy_nonoverlapping(e1.as_ptr(), (heap.cpu_base + 24) as *mut u8, 24);
    }
    let rtv_safe = TargetView::rtv(&tex_safe, None).unwrap();
    let rtv_defer = TargetView::rtv(&tex_defer, None).unwrap();
    let rtv_sw = TargetView::rtv(&rt_sw, None).unwrap();
    let tex_safe_id = &tex_safe as *const Resource as usize;
    let tex_defer_id = &tex_defer as *const Resource as usize;
    let rt_sw_id = &rt_sw as *const Resource as usize;
    let uav_d1_id = &uav_d1 as *const Resource as usize;
    let uav_d2_id = &uav_d2 as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g45_c queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |v5_on: bool| -> (u64, u64, u64, Vec<f32>, Vec<f32>) {
        let mut l_init = CommandList::new(ListType::Direct).unwrap();
        l_init.begin(&alloc_init).unwrap();
        unsafe { l_init.encode_rtv_clear_pass(&rtv_safe, [1.0, 2.0, 3.0, 1.0]).unwrap() };
        l_init.barrier_list_with(&[Barrier::Transition(tex_safe_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
        l_init.close().unwrap();

        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_cs_mid_qb_defer_v3 = true;
        l.opts.opt_cs_mid_qb_unpoison_v4 = true;
        l.opts.opt_cs_mid_qb_res_defer_v5 = v5_on;
        let v5_0 = stats::c::cs_mid_qb_v5.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        // Anchor Compute #0 on pre-list work:
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, uav_init.gpu_va).unwrap();
        l.set_compute_pipeline(cs_pso.clone());
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        // Render pass produces `tex_defer` (`stage_gen[Fragment]` advances!) and transitions
        // `tex_defer: RENDER_TARGET -> NON_PIXEL_SRV` BEFORE Compute #1 opens:
        l.bar.in_list_gfx_non_clear = true;
        unsafe { l.encode_rtv_clear_pass(&rtv_defer, [8.0, 16.0, 24.0, 1.0]).unwrap() };
        l.barrier_list_with(&[Barrier::Transition(tex_defer_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);

        l.copy_buffer(&blit_dst, 0, &up_buf, 0, 16).unwrap();
        unsafe { l.clear_rtv(&rtv_sw, [0.0, 1.0, 0.0, 1.0], &[]).unwrap() };
        l.barrier_list_with(&[Barrier::Transition(rt_sw_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
        let mqb0 = stats::c::cs_mid_qb.load(Relaxed);
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, uav_d1.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let mqb_d1 = stats::c::cs_mid_qb.load(Relaxed) - mqb0;
        let frag_gen_after_d1 = u64::from(l.bar.enc_cur_sync[STAGE_DISPATCH].gen[STAGE_FRAGMENT]);

        // Positive safety guardrail: Dispatch #2 binding `tex_defer` (`heap + 24`) MUST emit `Fragment -> Dispatch` `cs_mid_qb`!
        let mqb1 = stats::c::cs_mid_qb.load(Relaxed);
        l.compute_root.set_table(0, heap.gpu_base + 24).unwrap();
        l.compute_root.set_descriptor(1, uav_d2.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let mqb_touch = stats::c::cs_mid_qb.load(Relaxed) - mqb1;
        let frag_gen_after_d2 = u64::from(l.bar.enc_cur_sync[STAGE_DISPATCH].gen[STAGE_FRAGMENT]);
        assert_eq!(frag_gen_after_d2, 1, "Dispatch #2 must have Fragment gen 1 synced");
        let v5_delta = stats::c::cs_mid_qb_v5.load(Relaxed) - v5_0;

        l.barrier_list_with(
            &[
                Barrier::Transition(uav_d1_id, STATE_UAV, STATE_COPY_SOURCE),
                Barrier::Transition(uav_d2_id, STATE_UAV, STATE_COPY_SOURCE),
            ],
            true,
            false,
        );
        l.copy_buffer(&rb1, 0, &uav_d1, 0, 64).unwrap();
        l.copy_buffer(&rb2, 0, &uav_d2, 0, 64).unwrap();
        l.close().unwrap();

        let mut log = l.take_pass_log().unwrap();
        log.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l_init.cb, &l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g45_C must produce 0 bad timestamps");
        alloc_init.reset();
        alloc.reset();
        let v1: Vec<f32> = buf_words(&rb1, 4).into_iter().map(f32::from_bits).collect();
        let v2: Vec<f32> = buf_words(&rb2, 4).into_iter().map(f32::from_bits).collect();
        (frag_gen_after_d1 + mqb_d1, mqb_touch, v5_delta, v1, v2)
    };

    let (premature_v4, touch_v4, v5_off, ref_v1, ref_v2) = run_case(false);
    let (premature_v5, touch_v5, v5_on_cnt, opt_v1, opt_v2) = run_case(true);
    assert!(premature_v4 >= 1, "v4 stalled on Fragment prematurely during compute_clear_batch");
    assert_eq!(touch_v4, 0, "v4 already consumed Fragment before Dispatch #2");
    assert_eq!(v5_off, 0);
    assert_eq!(premature_v5, 0, "g45_C must keep Fragment overlapping across Blit-open + compute_clear_batch + disjoint Dispatch #1");
    assert_eq!(touch_v5, 1, "g45_C must emit deferred Fragment -> Dispatch barrier when tex_defer is bound on Dispatch #2");
    assert!(v5_on_cnt >= 3, "cs_mid_qb_v5 must count partition + cs_end_res deferral + Dispatch #1 deferral");
    assert_eq!(opt_v1, ref_v1);
    assert_eq!(&opt_v1[..], &[1.0, 2.0, 3.0, 1.0]);
    assert_eq!(opt_v2, ref_v2);
    assert_eq!(&opt_v2[..], &[8.0, 16.0, 24.0, 1.0]);
}

#[test]
fn g45_stage_d_icb_enc_barrier_exact_strips_mesh_object_and_frag_on_depth_only_icb_and_keeps_tex_common_on_gpu() {
    use crate::metal::indirect::{arg as iarg, IndirectLayout};
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline, READS_TEX};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let vs_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
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
    let pso_depth = Arc::new(
        GraphicsPipeline::new(
            gfx_rs.clone(),
            &GraphicsDesc {
                vs: &vs_dxil,
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
            "g45_d_depth_icb",
        )
        .unwrap(),
    );

    let args_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let count_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let ds = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let tex_unatt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb_d = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&args_buf, &count_buf, &ds, &tex_unatt, &rb_d] {
        r.register_ptr();
    }
    unsafe {
        let ap = args_buf.buffer().unwrap().contents().as_ptr() as *mut u32;
        *ap.add(0) = 3; // vertex_count
        *ap.add(1) = 1; // instance_count
        *ap.add(2) = 0; // start_vertex
        *ap.add(3) = 0; // start_instance
        let cp = count_buf.buffer().unwrap().contents().as_ptr() as *mut u32;
        *cp = 1;
    }
    let dsv = TargetView::dsv(&ds, None).unwrap();
    let ds_id = &ds as *const Resource as usize;
    let tex_unatt_id = &tex_unatt as *const Resource as usize;
    let layout = IndirectLayout::new(16, &[(iarg::DRAW, [0; 3])]).unwrap();

    let q = Queue::new(ListType::Direct, "g45_d queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |exact_on: bool| -> (bool, bool, u64, f32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_icb_enc_barrier_exact = exact_on;
        let cnt0 = stats::c::icb_enc_barrier_exact.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        unsafe { l.clear_dsv(&dsv, true, 0.75, false, 0, &[]).unwrap() };

        l.gfx_root.set_root_signature(gfx_rs.clone()).unwrap();
        l.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
        l.set_render_targets(Vec::new(), Some(dsv));
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso_depth.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
        l.execute_indirect(&layout, 1, args_buf.gpu_va, None, Some(count_buf.gpu_va)).unwrap();

        // Mid-pass unattached non-SIMUL texture `PIXEL_SRV -> STATE_COMMON` (`s1 == 0`) transition:
        l.barrier_list_with(&[Barrier::Transition(tex_unatt_id, STATE_PIXEL_SRV, STATE_COMMON)], true, false);
        let kept_tex_common = l.gfx.draw_pass;
        l.end_encoder();

        // Outside pass: non-SIMUL texture `RENDER_TARGET -> STATE_COMMON` defers `PRE_RASTER` to `soft[0]`:
        l.clear_all_pend();
        l.barrier_list_with(&[Barrier::Transition(tex_unatt_id, STATE_RENDER_TARGET, STATE_COMMON)], true, false);
        let soft_tex_deferred = !l.bar.split_pend.vtx_after.contains(MTLStages::Fragment)
            && l.bar.split_pend.soft_after[0].contains(MTLStages::Fragment);
        let _ = READS_TEX;
        let cnt_delta = stats::c::icb_enc_barrier_exact.load(Relaxed) - cnt0;

        l.barrier_list_with(&[Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE)], true, false);
        let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_d, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
        l.close().unwrap();

        let mut log = l.take_pass_log().unwrap();
        log.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g45_D must produce 0 bad timestamps");
        alloc.reset();
        (kept_tex_common, soft_tex_deferred, cnt_delta, f32::from_bits(buf_words(&rb_d, 1)[0]))
    };

    let (kept_off, soft_off, cnt_off, d_off) = run_case(false);
    let (kept_on, soft_on, cnt_on, d_on) = run_case(true);
    assert!(!kept_off && !soft_off);
    assert_eq!(cnt_off, 0);
    assert!(kept_on, "g45_D must keep render pass open across unattached non-SIMUL texture -> COMMON transition");
    assert!(soft_on, "g45_D must defer PRE_RASTER to soft[0] on non-SIMUL texture -> COMMON transition");
    assert!(cnt_on >= 3, "icb_enc_barrier_exact must count ICB barrier narrowing + kept/soft texture COMMON transitions");
    assert!(d_off.abs() < 1e-6, "depth-only ICB draw at z=0 must overwrite 0.75 depth clear: got {d_off}");
    assert!(d_on.abs() < 1e-6, "depth-only ICB draw under g45_D must overwrite 0.75 depth clear: got {d_on}");
}

#[test]
fn g46_defaults_promote_gfx_desc_table_exact_and_keep_g46_knobs_opt_in() {
    if std::env::var_os("AW2_GFX_DESC_TABLE_EXACT").is_none() {
        assert!(opt_gfx_desc_table_exact_env());
    }
    if std::env::var_os("AW2_DESC_TABLE_RES_EXACT").is_none() {
        assert!(!opt_desc_table_res_exact_env());
    }
    if std::env::var_os("AW2_CS_MID_QB_RES_DEFER_V5").is_none() {
        assert!(!opt_cs_mid_qb_res_defer_v5_env());
    }
    if std::env::var_os("PORT_ICB_ENC_BARRIER_EXACT").is_none() && std::env::var_os("AW2_ICB_ENC_BARRIER_EXACT").is_none() {
        assert!(opt_icb_enc_barrier_exact_env());
    }
    if std::env::var_os("PORT_ICB_PREPASS_COMPUTE").is_none() && std::env::var_os("AW2_ICB_PREPASS_COMPUTE").is_none() {
        assert!(!opt_icb_prepass_compute_env());
    }
    if std::env::var_os("AW2_DESC_TABLE_CS_FAST").is_none() {
        assert!(!opt_desc_table_cs_fast_env());
    }
    if std::env::var_os("PORT_RP_SPLIT_EXACT_KEEP").is_none() && std::env::var_os("AW2_RP_SPLIT_EXACT_KEEP").is_none() {
        assert!(opt_rp_split_exact_keep_env());
    }
    if std::env::var_os("PORT_CLEAR_CS_TILE4").is_none() && std::env::var_os("AW2_CLEAR_CS_TILE4").is_none() {
        assert!(opt_clear_cs_tile4_env());
    }
    if let Some(l) = CommandList::new(ListType::Direct) {
        if std::env::var_os("AW2_GFX_DESC_TABLE_EXACT").is_none() {
            assert!(l.opts.opt_gfx_desc_table_exact);
        }
        if std::env::var_os("PORT_ICB_PREPASS_COMPUTE").is_none() && std::env::var_os("AW2_ICB_PREPASS_COMPUTE").is_none() {
            assert!(!l.opts.opt_icb_prepass_compute);
        }
        if std::env::var_os("AW2_DESC_TABLE_CS_FAST").is_none() {
            assert!(!l.opts.opt_desc_table_cs_fast);
        }
        if std::env::var_os("PORT_RP_SPLIT_EXACT_KEEP").is_none() && std::env::var_os("AW2_RP_SPLIT_EXACT_KEEP").is_none() {
            assert!(l.opts.opt_rp_split_exact_keep);
        }
        if std::env::var_os("PORT_CLEAR_CS_TILE4").is_none() && std::env::var_os("AW2_CLEAR_CS_TILE4").is_none() {
            assert!(l.opts.opt_clear_cs_tile4);
        }
    }
}

#[test]
fn g46_stage_a_icb_prepass_compute_clamps_indirect_threadgroups_eliminates_inpass_barrier_and_preserves_draws_on_gpu() {
    use crate::metal::indirect::{arg as iarg, IndirectLayout};
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootDescriptor, RootDescriptorKind, RootParameter, RootParameterKind,
        RootSignature, RootSignatureDesc, RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let ps_cb0_dxbc = cb0_color_ps_dxbc();
    let vs_dxil =
        sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let gfx_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Descriptor(
                RootDescriptorKind::Cbv,
                RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
            ),
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let pso = Arc::new(
        GraphicsPipeline::new(
            gfx_rs.clone(),
            &GraphicsDesc {
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
            },
            "g46_a_color_icb",
        )
        .unwrap(),
    );

    let max_count = 512u32;
    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let args_buf = test_buffer((max_count as u64) * 16, heap_type::UPLOAD, 0);
    let count_zero_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let count_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &args_buf, &count_zero_buf, &count_buf, &rt, &rb] {
        r.register_ptr();
    }
    unsafe {
        let cp_cb = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp_cb.add(0) = 0.0;
        *cp_cb.add(1) = 1.0;
        *cp_cb.add(2) = 0.0;
        *cp_cb.add(3) = 1.0;
        let ap = args_buf.buffer().unwrap().contents().as_ptr() as *mut u32;
        for i in 0..(max_count as usize) * 4 {
            *ap.add(i) = 0;
        }
        // Slot 0 draws full-screen green triangle; slots 1..512 are beyond count=1.
        *ap.add(0) = 3;
        *ap.add(1) = 1;
        *ap.add(4) = 3;
        *ap.add(5) = 1;
        let cp0 = count_zero_buf.buffer().unwrap().contents().as_ptr() as *mut u32;
        *cp0 = 0;
        let cp = count_buf.buffer().unwrap().contents().as_ptr() as *mut u32;
        *cp = 1;
    }
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let rt_id = &rt as *const Resource as usize;
    let layout = IndirectLayout::new(16, &[(iarg::DRAW, [0; 3])]).unwrap();

    let q = Queue::new(ListType::Direct, "g46_a queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |prepass_on: bool| -> (u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_icb_prepass_compute = prepass_on;
        let pre0 = stats::c::icb_prepass_cs.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        unsafe { l.clear_rtv(&rtv, [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };

        l.gfx_root.set_root_signature(gfx_rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.set_render_targets(vec![rtv], None);
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
        // Two consecutive dynamic-count ExecuteIndirect draws sharing one ICB chunk:
        // draw 1 takes location=0 with live count=0 (draws nothing);
        // draw 2 takes location=512 with live count=1 (draws green triangle).
        l.execute_indirect(&layout, max_count, args_buf.gpu_va, None, Some(count_zero_buf.gpu_va)).unwrap();
        l.execute_indirect(&layout, max_count, args_buf.gpu_va, None, Some(count_buf.gpu_va)).unwrap();
        if prepass_on {
            assert!(l.icb.icb_prepass_pending, "large dynamic-count ICB before pass open must stage pre-pass compute");
            assert!(!l.gfx.draw_pass, "render pass must stay unopened while staging ICB pre-pass compute");
        }

        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();

        let pre_delta = stats::c::icb_prepass_cs.load(Relaxed) - pre0;
        let mut log = l.take_pass_log().unwrap();
        log.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g46_A must produce 0 bad timestamps");
        alloc.reset();
        (pre_delta, buf_words(&rb, 1)[0])
    };

    let (pre_off, px_off) = run_case(false);
    let (pre_on, px_on) = run_case(true);
    assert_eq!(pre_off, 0);
    assert!(pre_on >= 2, "icb_prepass_cs must increment for both batched ICB draws");
    assert_eq!(px_off, 0xff00_ff00);
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g46_stage_b_desc_table_cs_fast_caches_binds_defers_mid_cs_uav_and_preserves_hazards_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
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
    let mut cp = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g46_b_cs").unwrap();
    cp.access = ComputeAccess {
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }]),
        bounded_read_slots: vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }],
        unbounded_srv_classes: 0,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF | READS_TEX,
        heap_indexed_or_unknown: false,
        touches_textures: true,
        msc_refined: true,
        msc_exact_classes: READS_TEX,
    };
    let pso = Arc::new(cp);

    let tex_bound = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let tex_defer = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let b2 = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_init = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_out = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb = test_buffer(256, heap_type::READBACK, 0);
    for r in [&tex_bound, &tex_defer, &b2, &uav_init, &uav_out, &rb] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g46_b heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex_bound), None).unwrap();
        let e1 = crate::metal::views::srv_entry(Some(&tex_defer), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        std::ptr::copy_nonoverlapping(e1.as_ptr(), (heap.cpu_base + 24) as *mut u8, 24);
    }
    let rtv_bound = TargetView::rtv(&tex_bound, None).unwrap();
    let rtv_defer = TargetView::rtv(&tex_defer, None).unwrap();
    let tex_bound_id = &tex_bound as *const Resource as usize;
    let tex_defer_id = &tex_defer as *const Resource as usize;
    let b2_id = &b2 as *const Resource as usize;
    let uav_out_id = &uav_out as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g46_b queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |fast_on: bool| -> (u64, u64, Vec<f32>) {
        let mut l_init = CommandList::new(ListType::Direct).unwrap();
        l_init.begin(&alloc_init).unwrap();
        unsafe { l_init.encode_rtv_clear_pass(&rtv_bound, [3.0, 6.0, 9.0, 1.0]).unwrap() };
        l_init.barrier_list_with(&[Barrier::Transition(tex_bound_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
        l_init.close().unwrap();

        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_desc_table_cs_fast = fast_on;
        let fast0 = stats::c::desc_table_cs_fast.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        // Anchor Compute #0, then Render pass producing `tex_defer` (`Fragment` gen advances!):
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, uav_init.gpu_va).unwrap();
        l.set_compute_pipeline(pso.clone());
        l.dispatch(1, 1, 1).unwrap();
        l.end_encoder();

        l.bar.in_list_gfx_non_clear = true;
        unsafe { l.encode_rtv_clear_pass(&rtv_defer, [8.0, 16.0, 24.0, 1.0]).unwrap() };

        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.compute_root.set_descriptor(1, uav_out.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();

        let mqb0 = stats::c::cs_mid_qb.load(Relaxed);
        l.barrier_list_with(
            &[
                Barrier::Transition(tex_defer_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
                Barrier::Uav(b2_id),
            ],
            true,
            false,
        );
        l.dispatch(1, 1, 1).unwrap();
        let mqb_disjoint = stats::c::cs_mid_qb.load(Relaxed) - mqb0;
        let fast_delta = stats::c::desc_table_cs_fast.load(Relaxed) - fast0;

        l.barrier_list_with(&[Barrier::Transition(uav_out_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
        l.copy_buffer(&rb, 0, &uav_out, 0, 64).unwrap();
        l.close().unwrap();

        let mut log = l.take_pass_log().unwrap();
        log.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l_init.cb, &l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g46_B must produce 0 bad timestamps");
        alloc_init.reset();
        alloc.reset();
        let vals: Vec<f32> = buf_words(&rb, 4).into_iter().map(f32::from_bits).collect();
        (fast_delta, mqb_disjoint, vals)
    };

    let (fast_off, mqb_off, ref_vals) = run_case(false);
    let (fast_on, mqb_on, opt_vals) = run_case(true);
    assert_eq!(fast_off, 0);
    assert_eq!(mqb_off, 1, "baseline fired mid-CS queue barrier on mixed batch with Barrier::Uav(b2)");
    assert_eq!(mqb_on, 0, "g46_B must defer mixed batch with disjoint Barrier::Uav(b2) without mid-CS QB");
    assert!(fast_on >= 2, "desc_table_cs_fast must count mid-CS UAV deferral + CsBindCache hit: got {fast_on}");
    assert_eq!(opt_vals, ref_vals);
    assert_eq!(&opt_vals[..], &[3.0, 6.0, 9.0, 1.0]);
}

#[test]
fn g46_stage_c_rp_split_exact_keep_coalesces_transient_rt_rebinds_and_disjoint_gfx_uavs_on_gpu() {
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, GraphicsDesc, GraphicsPipeline, READS_BUF};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootDescriptor, RootDescriptorKind, RootParameter, RootParameterKind,
        RootSignature, RootSignatureDesc, RootSignatureVersion,
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
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let mut pso_raw = GraphicsPipeline::new(
        rs.clone(),
        &GraphicsDesc {
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
        },
        "g46_c_gfx",
    )
    .unwrap();
    pso_raw.writes_uav = true;
    pso_raw.sm66_writes_uav = true;
    pso_raw.vs_writes_uav = true;
    pso_raw.uses_uav = true;
    pso_raw.vs_access = ComputeAccess {
        touches_textures: false,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF,
        msc_exact_classes: 0,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: 0,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: Some(Vec::new()),
        bounded_read_slots: Vec::new(),
    };
    let pso = Arc::new(pso_raw);

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let rt0 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt1 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let uav_a = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_b = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_c = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_rt = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &rt0, &rt1, &uav_a, &uav_b, &uav_c, &rb_rt] {
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
    let rt0_id = &rt0 as *const Resource as usize;
    let uav_a_id = &uav_a as *const Resource as usize;
    let uav_b_id = &uav_b as *const Resource as usize;
    let uav_c_id = &uav_c as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g46_c queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |keep_on: bool| -> (u64, u64, bool, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_rp_split_exact_keep = keep_on;
        let dp0 = stats::c::draw_pass.load(Relaxed);
        let cnt0 = stats::c::rp_split_exact_keep.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        unsafe { l.clear_rtv(&rtv0, [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };

        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, uav_a.gpu_va).unwrap();
        l.set_render_targets(vec![rtv0], None);
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // Draw 1 records `uav_a` in `enc_uav_write_exact_res`:
        l.draw(3, 1, 0, 0).unwrap();
        // Transient target rebind to `rtv1` then back to `rtv0` with zero draws in between:
        l.set_render_targets(vec![rtv1], None);
        l.set_render_targets(vec![rtv0], None);
        // Disjoint `Barrier::Uav(uav_b)` + unattached buffer `COPY_DEST -> COMMON` on `uav_c`:
        l.barrier_list_with(
            &[
                Barrier::Uav(uav_b_id),
                Barrier::Transition(uav_c_id, STATE_COPY_DEST, STATE_COMMON),
            ],
            true,
            false,
        );
        // Draw 2 again binds `uav_a` (`uav_a != uav_b`):
        l.draw(3, 1, 0, 0).unwrap();
        let dp_coalesced = stats::c::draw_pass.load(Relaxed) - dp0;

        // Positive safety guardrail: `Barrier::Uav(uav_a_id)` MUST split because `uav_a` WAS written in this pass!
        let keeps_written_uav = l.render_barrier_keeps(&[Barrier::Uav(uav_a_id)], true);

        let cnt_delta = stats::c::rp_split_exact_keep.load(Relaxed) - cnt0;
        l.barrier_list_with(&[Barrier::Transition(rt0_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp), [0, 0, 0], CopyLoc::Subresource(&rt0, 0), None).unwrap();
        l.close().unwrap();

        let mut log = l.take_pass_log().unwrap();
        log.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "g46_C must produce 0 bad timestamps");
        alloc.reset();
        (dp_coalesced, cnt_delta, keeps_written_uav, buf_words(&rb_rt, 1)[0])
    };

    let (dp_off, cnt_off, keeps_written_off, px_off) = run_case(false);
    let (dp_on, cnt_on, keeps_written_on, px_on) = run_case(true);
    assert_eq!(cnt_off, 0);
    assert!(dp_off >= 2, "baseline must split draw pass on transient RT rebind + UAV barrier: got {dp_off}");
    assert_eq!(dp_on, 1, "g46_C must keep a single draw pass across transient RT rebind + disjoint UAV/COMMON barriers");
    assert!(cnt_on >= 3, "rp_split_exact_keep must count transient RT keep + disjoint UAV/COMMON keeps: got {cnt_on}");
    assert!(!keeps_written_off && !keeps_written_on, "Barrier::Uav on actually written uav_a must still split");
    assert_eq!(px_off, 0xff00_ff00);
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g46_stage_d_clear_cs_tile4_clears_odd_textures_and_unaligned_tail_buffers_exact_on_gpu() {
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "g46_d queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();

    // Odd 7x5 2D UAV texture + 259-dword (1036-byte) buffer UAV to test exact boundary clamping:
    let tex = test_texture(7, 5, RGBA8_UNORM, resource_flags::ALLOW_UNORDERED_ACCESS);
    let buf = test_buffer(300 * 4, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_tex = test_buffer(5 * 256, heap_type::READBACK, 0);
    let rb_buf = test_buffer(300 * 4, heap_type::READBACK, 0);
    for r in [&tex, &buf, &rb_tex, &rb_buf] {
        r.register_ptr();
    }
    let tex_id = &tex as *const Resource as usize;
    let buf_id = &buf as *const Resource as usize;

    let raw_uav_desc = |num_elems: u32| -> [u8; 40] {
        let mut d = [0u8; 40];
        d[0..4].copy_from_slice(&41u32.to_le_bytes()); // R32_FLOAT / R32_TYPELESS compatible
        d[0..4].copy_from_slice(&39u32.to_le_bytes()); // DXGI_FORMAT_R32_TYPELESS = 39
        d[4..8].copy_from_slice(&1u32.to_le_bytes()); // D3D12_UAV_DIMENSION_BUFFER = 1
        d[16..20].copy_from_slice(&num_elems.to_le_bytes());
        d[32..36].copy_from_slice(&1u32.to_le_bytes()); // D3D12_BUFFER_UAV_FLAG_RAW = 1
        d
    };

    let _guard = gpu_stats_lock();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_clear_cs_tile4 = true;
    let cnt0 = stats::c::clear_cs_tile4.load(Relaxed);

    l.begin(&alloc).unwrap();
    l.pt_armed = true;
    // Fill all 300 dwords with sentinel 0xdead_beef, then clear only first 259 dwords to 0x1234_5678:
    l.clear_uav(&buf, Some(&raw_uav_desc(300)), ClearValue::Uint([0xdead_beef; 4]), &[]).unwrap();
    l.barrier_list_with(&[Barrier::Uav(buf_id)], true, false);
    l.clear_uav(&buf, Some(&raw_uav_desc(259)), ClearValue::Uint([0x1234_5678; 4]), &[]).unwrap();
    l.clear_uav(&tex, None, ClearValue::Float([0.0, 1.0, 0.0, 1.0]), &[]).unwrap();

    let cnt_delta = stats::c::clear_cs_tile4.load(Relaxed) - cnt0;
    assert!(cnt_delta >= 3, "clear_cs_tile4 must count vectorized buffer fills + 2x2 texture clear: got {cnt_delta}");

    l.barrier_list_with(
        &[
            Barrier::Transition(buf_id, STATE_UAV, STATE_COPY_SOURCE),
            Barrier::Transition(tex_id, STATE_UAV, STATE_COPY_SOURCE),
        ],
        true,
        false,
    );
    l.copy_buffer(&rb_buf, 0, &buf, 0, 300 * 4).unwrap();
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 7, height: 5, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_tex, fp), [0, 0, 0], CopyLoc::Subresource(&tex, 0), None).unwrap();
    l.close().unwrap();

    let mut log = l.take_pass_log().unwrap();
    log.submitted(pt::mach_now(), 0, 0);
    q.submit(&[&l.cb]);
    q.wait_idle();
    let summary = pt::resolve_for_test(vec![log]);
    assert_eq!(summary.bad, 0, "g46_D must produce 0 bad timestamps");
    alloc.reset();

    let words = buf_words(&rb_buf, 300);
    for (i, &w) in words[..259].iter().enumerate() {
        assert_eq!(w, 0x1234_5678, "word {i} must be cleared to 0x1234_5678");
    }
    for (i, &w) in words[259..].iter().enumerate() {
        assert_eq!(w, 0xdead_beef, "tail sentinel word {} must remain untouched", 259 + i);
    }
    let tex_words = buf_words(&rb_tex, 5 * 64);
    for y in 0..5 {
        for x in 0..7 {
            assert_eq!(tex_words[y * 64 + x], 0xff00_ff00, "pixel ({x},{y}) of odd 7x5 texture must be cleared");
        }
    }
}

#[test]
fn g47_defaults_keep_g46_off_gfx_exact_on_and_g47_off() {
    if crate::metal::device().is_none() {
        return;
    }
    let l = CommandList::new(ListType::Direct).unwrap();
    assert!(l.opts.opt_gfx_desc_table_exact, "AW2_GFX_DESC_TABLE_EXACT must remain Default-ON");
    assert!(!l.opts.opt_icb_prepass_compute, "PORT_ICB_PREPASS_COMPUTE must remain Default-OFF");
    assert!(!l.opts.opt_desc_table_cs_fast, "AW2_DESC_TABLE_CS_FAST must remain Default-OFF");
    assert!(l.opts.opt_rp_split_exact_keep, "PORT_RP_SPLIT_EXACT_KEEP must default ON");
    assert!(l.opts.opt_clear_cs_tile4, "PORT_CLEAR_CS_TILE4 must default ON");
    assert!(!l.opts.opt_list_boundary_overlap, "AW2_LIST_BOUNDARY_OVERLAP must default OFF");
    assert!(!l.opts.opt_cs_intra_barrier_exact, "AW2_CS_INTRA_BARRIER_EXACT must default OFF");
    assert!(!l.opts.opt_clear_pass_fold_v2, "AW2_CLEAR_PASS_FOLD_V2 must default OFF");
    assert!(!l.opts.opt_cs_to_ps_overlap, "AW2_CS_TO_PS_OVERLAP must default OFF");
}

#[test]
fn g47_stage_a_list_boundary_overlap_dominance_and_gpu_readback() {
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootDescriptor, RootDescriptorKind, RootParameter, RootParameterKind,
        RootSignature, RootSignatureDesc, RootSignatureVersion,
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
        parameters: vec![RootParameter {
            kind: RootParameterKind::Descriptor(
                RootDescriptorKind::Cbv,
                RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
            ),
            visibility: 5, // D3D12_SHADER_VISIBILITY_PIXEL = 5 -> VS reads 0 classes!
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let pso = Arc::new(
        GraphicsPipeline::new(
            rs.clone(),
            &GraphicsDesc {
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
            },
            "g47_a_gfx",
        )
        .unwrap(),
    );

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let src_buf = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let dst_buf = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_buf = test_buffer(64, heap_type::READBACK, 0);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb_rt = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &src_buf, &dst_buf, &rb_buf, &rt, &rb_rt] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let src_id = &src_buf as *const Resource as usize;
    let dst_id = &dst_buf as *const Resource as usize;
    let rt_id = &rt as *const Resource as usize;
    let raw_uav_desc = {
        let mut d = [0u8; 40];
        d[0..4].copy_from_slice(&39u32.to_le_bytes());
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[16..20].copy_from_slice(&16u32.to_le_bytes());
        d[32..36].copy_from_slice(&1u32.to_le_bytes());
        d
    };

    let q = Queue::new(ListType::Direct, "g47_a queue").unwrap();
    let alloc1 = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc2 = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc3 = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut l1 = CommandList::new(ListType::Direct).unwrap();
    l1.begin(&alloc1).unwrap();
    l1.pt_armed = true;
    l1.clear_uav(&src_buf, Some(&raw_uav_desc), ClearValue::Uint([0x99aa_bbcc; 4]), &[]).unwrap();
    l1.barrier_list_with(&[Barrier::Transition(src_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    l1.close().unwrap();

    let mut l2 = CommandList::new(ListType::Direct).unwrap();
    l2.opts.opt_list_boundary_overlap = true;
    let ov0 = stats::c::list_boundary_overlap.load(Relaxed);
    l2.begin(&alloc2).unwrap();
    l2.pt_armed = true;
    // Transition in L2 marks BOTH dst_id and src_id nonfrag-safe before L2's opening Blit!
    l2.barrier_list_with(
        &[
            Barrier::Transition(dst_id, STATE_UAV, STATE_COPY_DEST),
            Barrier::Transition(src_id, STATE_NON_PIXEL_SRV, STATE_COPY_SOURCE),
        ],
        true,
        false,
    );
    l2.copy_buffer(&dst_buf, 0, &src_buf, 0, 64).unwrap();
    assert!(
        l2.bar.enc_cur_sync[STAGE_BLIT].pre_list_stages.contains(COMPUTE_STAGES | PRE_RASTER),
        "opening nonfrag-safe Blit must stamp pre_list_stages on STAGE_BLIT"
    );
    l2.barrier_list_with(&[Barrier::Transition(dst_id, STATE_COPY_DEST, STATE_COPY_SOURCE)], true, false);
    l2.copy_buffer(&rb_buf, 0, &dst_buf, 0, 64).unwrap();
    l2.close().unwrap();

    let mut l3 = CommandList::new(ListType::Direct).unwrap();
    l3.opts.opt_list_boundary_overlap = true;
    l3.begin(&alloc3).unwrap();
    l3.pt_armed = true;
    l3.gfx_root.set_root_signature(rs.clone()).unwrap();
    l3.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
    l3.set_render_targets(vec![rtv], None);
    l3.gfx.topology = 4;
    l3.set_graphics_pipeline(pso.clone());
    l3.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
    l3.draw(3, 1, 0, 0).unwrap();
    assert!(
        l3.bar.enc_cur_sync[0].pre_list_stages.contains(COMPUTE_STAGES | PRE_RASTER),
        "list_start_render_split under g47_A must stamp pre_list_stages on Vertex"
    );
    l3.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l3.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
    l3.close().unwrap();

    let ov_delta = stats::c::list_boundary_overlap.load(Relaxed) - ov0;
    assert!(ov_delta >= 1, "list_boundary_overlap must count partial pre-list stamp/fold: got {ov_delta}");

    let mut logs = Vec::new();
    for l in [&mut l1, &mut l2, &mut l3] {
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        logs.push(lg);
    }
    q.submit(&[&l1.cb, &l2.cb, &l3.cb]);
    q.wait_idle();
    let summary = pt::resolve_for_test(logs);
    assert_eq!(summary.bad, 0, "g47_A must produce 0 bad timestamps");
    alloc1.reset();
    alloc2.reset();
    alloc3.reset();

    assert_eq!(buf_words(&rb_buf, 1)[0], 0x99aa_bbcc);
    assert_eq!(buf_words(&rb_rt, 1)[0], 0xff00_ff00);
}

#[test]
fn g47_stage_b_cs_intra_barrier_exact_skips_disjoint_and_orders_real_hazard_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
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
    let mut pso_raw = ComputePipeline::new(rs.clone(), &cs_dxil, "g47_b_cs").unwrap();
    pso_raw.access = ComputeAccess {
        touches_textures: true,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: READS_BUF,
        read_classes: READS_BUF | READS_TEX,
        msc_exact_classes: READS_BUF | READS_TEX,
        uav_srv_classes: READS_BUF | READS_TEX,
        unbounded_srv_classes: READS_BUF | READS_TEX,
        uav_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        read_slots: None,
        bounded_read_slots: vec![
            ComputeBind::TableEntry { param: 0, byte_offset: 0 },
            ComputeBind::TableEntry { param: 0, byte_offset: 24 },
        ],
    };
    let pso = Arc::new(pso_raw);

    let tex = test_texture(4, 1, 2, resource_flags::ALLOW_RENDER_TARGET);
    let b_srv = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b0 = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let b1 = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb0 = test_buffer(256, heap_type::READBACK, 0);
    let rb1 = test_buffer(256, heap_type::READBACK, 0);
    let rb_srv = test_buffer(256, heap_type::READBACK, 0);
    for r in [&tex, &b_srv, &b0, &b1, &rb0, &rb1, &rb_srv] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g47_b heap").unwrap();
    unsafe {
        let e0 = crate::metal::views::srv_entry(Some(&tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        let p1 = (heap.cpu_base + 24) as *mut u64;
        p1.write_unaligned(b_srv.gpu_va);
        p1.add(1).write_unaligned(0);
        p1.add(2).write_unaligned(64);
    }
    let rtv_tex = TargetView::rtv(&tex, None).unwrap();
    let b_srv_id = &b_srv as *const Resource as usize;
    let b0_id = &b0 as *const Resource as usize;
    let b1_id = &b1 as *const Resource as usize;
    let tex_id = &tex as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g47_b queue").unwrap();
    let alloc_init = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let mut init = CommandList::new(ListType::Direct).unwrap();
    init.begin(&alloc_init).unwrap();
    unsafe { init.encode_rtv_clear_pass(&rtv_tex, [3.0, 6.0, 9.0, 1.0]).unwrap() };
    init.barrier_list_with(&[Barrier::Transition(tex_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    init.close().unwrap();
    q.submit(&[&init.cb]);
    q.wait_idle();
    alloc_init.reset();

    let run_case = |exact_on: bool| -> (u64, u64, u64, [f32; 4], [f32; 4], [f32; 4]) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_cs_intra_barrier_exact = exact_on;
        let ex0 = stats::c::cs_intra_barrier_exact.load(Relaxed);
        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.set_descriptor_heaps(heap.gpu_base, 0);
        l.compute_root.set_root_signature(rs.clone()).unwrap();
        l.compute_root.set_table(0, heap.gpu_base).unwrap();
        l.set_compute_pipeline(pso.clone());

        // Dispatch 1 reads [tex, b_srv] and writes b0:
        l.compute_root.set_descriptor(1, b0.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let eb0 = stats::c::enc_barrier.load(Relaxed);
        // Transition(b_srv, NON_PIXEL_SRV, UAV) creates WAR obligation on b_srv:
        l.barrier_list_with(&[Barrier::Transition(b_srv_id, STATE_NON_PIXEL_SRV, STATE_UAV)], true, false);
        // Dispatch 2 & Dispatch 3 write b1 (disjoint from b_srv!), with Dispatch 3 hitting CsBindCache:
        l.compute_root.set_descriptor(1, b1.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let eb_disjoint = stats::c::enc_barrier.load(Relaxed) - eb0;
        // Dispatch 4 now actually binds b_srv as UAV — under g47_B, the deferred OblKind::UavOnly
        // obligation on b_srv MUST fire here!
        let eb_war0 = stats::c::enc_barrier.load(Relaxed);
        l.compute_root.set_descriptor(1, b_srv.gpu_va).unwrap();
        l.dispatch(1, 1, 1).unwrap();
        let eb_war = stats::c::enc_barrier.load(Relaxed) - eb_war0;

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

        let ex_delta = stats::c::cs_intra_barrier_exact.load(Relaxed) - ex0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g47_B must produce 0 bad timestamps");
        alloc.reset();

        let v0: Vec<f32> = buf_words(&rb0, 4).into_iter().map(f32::from_bits).collect();
        let v1: Vec<f32> = buf_words(&rb1, 4).into_iter().map(f32::from_bits).collect();
        let vs: Vec<f32> = buf_words(&rb_srv, 4).into_iter().map(f32::from_bits).collect();
        (ex_delta, eb_disjoint, eb_war, [v0[0], v0[1], v0[2], v0[3]], [v1[0], v1[1], v1[2], v1[3]], [vs[0], vs[1], vs[2], vs[3]])
    };

    let (ex_off, eb_off, eb_war_off, r0_off, r1_off, rs_off) = run_case(false);
    let (ex_on, eb_on, eb_war_on, r0_on, r1_on, rs_on) = run_case(true);
    assert_eq!(ex_off, 0);
    assert_eq!(eb_off, 1, "baseline emitted intra-CS barrier on Dispatch 2 for disjoint NON_PIXEL_SRV->UAV obligation");
    assert_eq!(eb_war_off, 0, "baseline already drained obligation prematurely on Dispatch 2");
    assert_eq!(eb_on, 0, "g47_B must skip intra-CS barrier across Dispatches 2 & 3 on disjoint UAV [b1]");
    assert_eq!(eb_war_on, 1, "g47_B must fire deferred intra-CS WAR barrier when Dispatch 4 binds b_srv as UAV");
    assert!(ex_on >= 2, "cs_intra_barrier_exact must increment on exact OblKind + CsBindCache hit: got {ex_on}");
    assert_eq!(r0_on, r0_off);
    assert_eq!(r1_on, r1_off);
    assert_eq!(rs_on, rs_off);
    assert_eq!(r0_on, [3.0, 6.0, 9.0, 1.0]);
    assert_eq!(r1_on, [3.0, 6.0, 9.0, 1.0]);
    assert_eq!(rs_on, [3.0, 6.0, 9.0, 1.0]);
}

#[test]
fn g47_stage_c_clear_pass_fold_v2_uav0_keep_multislice_and_hazard_isolation_on_gpu() {
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootDescriptor, RootDescriptorKind, RootParameter, RootParameterKind,
        RootSignature, RootSignatureDesc, RootSignatureVersion,
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
        parameters: vec![RootParameter {
            kind: RootParameterKind::Descriptor(
                RootDescriptorKind::Cbv,
                RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
            ),
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let pso = Arc::new(
        GraphicsPipeline::new(
            rs.clone(),
            &GraphicsDesc {
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
            },
            "g47_c_gfx",
        )
        .unwrap(),
    );

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    // Non-UAV RTV (`ALLOW_RENDER_TARGET` only, so `can_compute_clear` is false!):
    let rt_draw = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_standalone = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let uav_buf = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_draw = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_standalone = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &rt_draw, &rt_standalone, &uav_buf, &rb_draw, &rb_standalone] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let rtv_draw = TargetView::rtv(&rt_draw, None).unwrap();
    let rtv_standalone = TargetView::rtv(&rt_standalone, None).unwrap();
    let rt_draw_id = &rt_draw as *const Resource as usize;
    let rt_standalone_id = &rt_standalone as *const Resource as usize;
    let uav_id = &uav_buf as *const Resource as usize;
    let raw_uav_desc = {
        let mut d = [0u8; 40];
        d[0..4].copy_from_slice(&39u32.to_le_bytes());
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[16..20].copy_from_slice(&16u32.to_le_bytes());
        d[32..36].copy_from_slice(&1u32.to_le_bytes());
        d
    };

    let q = Queue::new(ListType::Direct, "g47_c queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |fold_v2: bool| -> (u64, u64, u64, u32, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_clear_pass_fold_v2 = fold_v2;
        // Simulate baseline where `opt_uav0_clear_overlap` is OFF to verify `g47_C` retains non-UAV RTV across `Barrier::Uav(0)`!
        l.opts.opt_uav0_clear_overlap = false;
        let f0 = stats::c::clear_pass_fold_v2.load(Relaxed);
        let cs_enc0 = stats::c::compute_enc.load(Relaxed);
        let cp0 = stats::c::clear_pass.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        unsafe {
            l.clear_rtv(&rtv_draw, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
            l.clear_rtv(&rtv_standalone, [0.0, 0.0, 1.0, 1.0], &[]).unwrap();
        }
        l.clear_uav(&uav_buf, Some(&raw_uav_desc), ClearValue::Uint([0x77; 4]), &[]).unwrap();
        l.barrier_list_with(&[Barrier::Uav(0)], true, false);
        l.clear_uav(&uav_buf, Some(&raw_uav_desc), ClearValue::Uint([0x88; 4]), &[]).unwrap();
        // Post-clear Dispatch -> SRV transition on unrelated `uav_buf`:
        l.barrier_list_with(
            &[Barrier::Transition(uav_id, STATE_UAV, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)],
            true,
            false,
        );
        // Flush `rt_standalone` via transition to COPY_SOURCE (`CLEAR` pass must NOT drain `uav_buf`'s Dispatch->SRV obligation):
        l.barrier_list_with(&[Barrier::Transition(rt_standalone_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let cs_enc_before_copy = stats::c::compute_enc.load(Relaxed) - cs_enc0;

        // `rt_draw` folds its clear into this Draw pass (clipped to 2x2 so we verify BOTH cleared red and drawn green pixels!):
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.set_render_targets(vec![rtv_draw], None);
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 2.0, height: 2.0, znear: 0.0, zfar: 1.0 }]);
        l.draw(3, 1, 0, 0).unwrap();

        l.barrier_list_with(&[Barrier::Transition(rt_draw_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_draw, fp), [0, 0, 0], CopyLoc::Subresource(&rt_draw, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_standalone, fp), [0, 0, 0], CopyLoc::Subresource(&rt_standalone, 0), None).unwrap();
        l.close().unwrap();

        let f_delta = stats::c::clear_pass_fold_v2.load(Relaxed) - f0;
        let cp_delta = stats::c::clear_pass.load(Relaxed) - cp0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g47_C must produce 0 bad timestamps");
        alloc.reset();

        let draw_words = buf_words(&rb_draw, 4);
        // Pixel (0,0) = green (0xff00_ff00), Pixel (3,0) = cleared red (0xff00_00ff)
        let combined = draw_words[0] ^ draw_words[3].rotate_left(1);
        let st_px = buf_words(&rb_standalone, 1)[0];
        (f_delta, cs_enc_before_copy, cp_delta, combined, st_px)
    };

    let (f_off, cs_off, cp_off, px_off, st_off) = run_case(false);
    let (f_on, cs_on, cp_on, px_on, st_on) = run_case(true);
    assert_eq!(f_off, 0);
    assert_eq!(cs_off, 2, "without UAV(0) non-UAV clear retention, Barrier::Uav(0) split the compute encoder");
    assert_eq!(cs_on, 1, "g47_C must keep non-UAV RTV clears across Barrier::Uav(0) in 1 compute encoder");
    assert_eq!(cp_off, 2, "without UAV(0) retention, both non-UAV RTV clears became standalone clear passes");
    assert_eq!(cp_on, 1, "g47_C must fold rt_draw clear into Draw pass, leaving only rt_standalone clear pass");
    assert!(f_on >= 1, "clear_pass_fold_v2 counter must increment: got {f_on}");
    assert_eq!(px_on, px_off);
    assert_eq!(st_on, st_off);
    assert_eq!(st_on, 0xffff_0000);
}

#[test]
fn g47_stage_d_cs_to_ps_overlap_per_res_soft_defer_and_vs_hazard_on_gpu() {
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, GraphicsDesc, GraphicsPipeline, READS_BUF};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootDescriptor, RootDescriptorKind, RootParameter, RootParameterKind,
        RootSignature, RootSignatureDesc, RootSignatureVersion,
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
                visibility: 0, // ALL -> both VS and PS bind param 0 or param 1
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Srv,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 1, // VERTEX
            },
        ],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let mut color_blend = BlendDesc::default();
    color_blend.RenderTarget[0].RenderTargetWriteMask = 0xF;
    let mut pso_raw = GraphicsPipeline::new(
        rs.clone(),
        &GraphicsDesc {
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
        },
        "g47_d_gfx",
    )
    .unwrap();
    pso_raw.pre_raster_reads = READS_BUF;
    pso_raw.sm66_pre_raster_reads = READS_BUF;
    pso_raw.vs_access = ComputeAccess {
        touches_textures: false,
        heap_indexed_or_unknown: false,
        msc_refined: true,
        uav_classes: 0,
        read_classes: READS_BUF,
        msc_exact_classes: READS_BUF,
        uav_srv_classes: READS_BUF,
        unbounded_srv_classes: 0,
        uav_slots: Some(Vec::new()),
        read_slots: Some(vec![ComputeBind::RootVa { param: 1 }]),
        bounded_read_slots: vec![ComputeBind::RootVa { param: 1 }],
    };
    let pso = Arc::new(pso_raw);

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let buf_cs = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_anchor = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_vs, &buf_cs, &rt_anchor, &rt, &rb] {
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
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let rt_id = &rt as *const Resource as usize;
    let cs_id = &buf_cs as *const Resource as usize;
    let raw_uav_desc = {
        let mut d = [0u8; 40];
        d[0..4].copy_from_slice(&39u32.to_le_bytes());
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[16..20].copy_from_slice(&16u32.to_le_bytes());
        d[32..36].copy_from_slice(&1u32.to_le_bytes());
        d
    };

    let q = Queue::new(ListType::Direct, "g47_d queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |overlap_on: bool| -> (u64, bool, u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_cs_to_ps_overlap_v2 = false;
        l.opts.opt_cs_to_ps_overlap = overlap_on;
        let ov0 = stats::c::cs_to_ps_overlap.load(Relaxed);
        let hit0 = stats::c::dv_hit.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // Pass #0 (e.g. G-buffer draw) anchors pre-list render stages (`list_unwaited_gfx = false`):
        l.set_render_targets(vec![rtv_anchor], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        // Compute #1 writes `buf_cs` (`stage_gen[STAGE_DISPATCH]` advances!):
        l.clear_uav(&buf_cs, Some(&raw_uav_desc), ClearValue::Uint([0x42; 4]), &[]).unwrap();
        // Transition `buf_cs` to `NON_PIXEL_SRV | PIXEL_SRV` (`frag_after = Dispatch`, `soft_after[1] = Dispatch`):
        l.barrier_list_with(
            &[Barrier::Transition(cs_id, STATE_UAV, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV)],
            true,
            false,
        );

        // Pass #1 Draw 1: VS binds `buf_vs` (disjoint from `buf_cs`!), so under `g47_D` `Vertex` does NOT wait on Compute #1's `Dispatch`!
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.set_render_targets(vec![rtv], None);
        l.draw(3, 1, 0, 0).unwrap();
        let vtx_did_not_wait_disp = l.bar.enc_cur_sync[0].gen[STAGE_DISPATCH] < l.bar.stage_gen[STAGE_DISPATCH];

        // Pass #1 Draw 2 in the SAME pass: VS now binds `buf_cs`, so `sync_pre_raster` MUST fire `dv_hit`!
        l.gfx_root.set_descriptor(1, buf_cs.gpu_va).unwrap();
        l.draw(3, 1, 0, 0).unwrap();

        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();

        let ov_delta = stats::c::cs_to_ps_overlap.load(Relaxed) - ov0;
        let hit_delta = stats::c::dv_hit.load(Relaxed) - hit0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g47_D must produce 0 bad timestamps");
        alloc.reset();
        (ov_delta, vtx_did_not_wait_disp, hit_delta, buf_words(&rb, 1)[0])
    };

    let (ov_off, overlap_off, hit_off, px_off) = run_case(false);
    let (ov_on, overlap_on, hit_on, px_on) = run_case(true);
    assert_eq!(ov_off, 0);
    assert!(!overlap_off, "baseline forced Vertex to wait on Dispatch at pass open because VS read READS_BUF");
    assert!(overlap_on, "g47_D must let Draw 1 Vertex overlap Dispatch when VS only binds disjoint buf_vs");
    assert_eq!(hit_off, 0);
    assert_eq!(hit_on, 1, "g47_D must still emit deferred Vertex barrier before Draw 2 when VS binds buf_cs");
    assert!(ov_on >= 1, "cs_to_ps_overlap must increment: got {ov_on}");
    assert_eq!(px_on, px_off);
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g48_defaults_promote_cs_to_ps_overlap_v2_and_keep_other_knobs_off() {
    if crate::metal::device().is_none() {
        return;
    }
    let l = CommandList::new(ListType::Direct).unwrap();
    assert!(l.opts.opt_gfx_desc_table_exact, "AW2_GFX_DESC_TABLE_EXACT must remain Default-ON");
    assert!(l.opts.opt_cs_to_ps_overlap_v2, "AW2_CS_TO_PS_OVERLAP_V2 promoted to Default-ON");
    assert!(!l.opts.opt_icb_prepass_compute);
    assert!(!l.opts.opt_desc_table_cs_fast);
    assert!(l.opts.opt_rp_split_exact_keep);
    assert!(l.opts.opt_clear_cs_tile4);
    assert!(!l.opts.opt_list_boundary_overlap);
    assert!(!l.opts.opt_cs_intra_barrier_exact);
    assert!(!l.opts.opt_clear_pass_fold_v2);
    assert!(!l.opts.opt_cs_to_ps_overlap);
    assert!(!l.opts.opt_gfx_to_ps_overlap);
    assert!(!l.opts.opt_clear_isolate_coalesce);
    assert!(
        l.opts.opt_queue_start_stage_exact,
        "PORT_QUEUE_START_STAGE_EXACT promoted to Default-ON"
    );
}

#[test]
fn g48_a_cs_to_ps_overlap_v2_unpoisoned_compute_mid_and_uav_skip() {
    use std::sync::atomic::Ordering::Relaxed;
    let Some((rs, pso)) = build_g48_vs_srv_ps_cb0_pso("g48_a_mid_gfx") else { return };

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let buf_cs = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let buf_other = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_anchor = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_vs, &buf_cs, &buf_other, &rt_anchor, &rt, &rb] {
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
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let rt_id = &rt as *const Resource as usize;
    let cs_id = &buf_cs as *const Resource as usize;
    let other_id = &buf_other as *const Resource as usize;
    let raw_uav_desc = {
        let mut d = [0u8; 40];
        d[0..4].copy_from_slice(&39u32.to_le_bytes());
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[16..20].copy_from_slice(&16u32.to_le_bytes());
        d[32..36].copy_from_slice(&1u32.to_le_bytes());
        d
    };

    let q = Queue::new(ListType::Direct, "g48_a_mid queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |enable_v2: bool| -> (u64, bool, u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_cs_to_ps_overlap = false;
        l.opts.opt_cs_to_ps_overlap_v2 = enable_v2;
        l.opts.opt_subrange_uav_barrier = true;
        l.opts.opt_cs_mid_qb_res_defer_v5 = false;
        let v2_0 = stats::c::cs_to_ps_overlap_v2.load(Relaxed);
        let hit0 = stats::c::dv_hit.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // Anchor pre-list render stages:
        l.set_render_targets(vec![rtv_anchor], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        // Compute #1 writes `buf_cs`, `Barrier::Uav(cs_id)` triggers `cs_uav_mid_skip`,
        // then `Transition(cs_id -> SRV)` + Compute #2 writing `buf_other` exercises `compute_mid_queue_sync_with`!
        l.clear_uav(&buf_cs, Some(&raw_uav_desc), ClearValue::Uint([0x42; 4]), &[]).unwrap();
        l.barrier_list_with(&[Barrier::Uav(cs_id)], true, false);
        l.barrier_list_with(
            &[
                Barrier::Transition(cs_id, STATE_UAV, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV),
                Barrier::Transition(other_id, STATE_COPY_DEST, STATE_UAV),
            ],
            true,
            false,
        );
        l.clear_uav(&buf_other, Some(&raw_uav_desc), ClearValue::Uint([0x11; 4]), &[]).unwrap();
        l.barrier_list_with(&[Barrier::Transition(other_id, STATE_UAV, STATE_PIXEL_SRV)], true, false);

        // Render Pass Draw 1: VS only binds `buf_vs` (disjoint from `buf_cs`).
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.set_render_targets(vec![rtv], None);
        l.draw(3, 1, 0, 0).unwrap();
        let vtx_overlapped = l.bar.enc_cur_sync[0].gen[STAGE_DISPATCH] < l.bar.stage_gen[STAGE_DISPATCH];

        // Draw 2 in same pass: VS binds `buf_cs` -> `sync_pre_raster` MUST emit `dv_hit`!
        l.gfx_root.set_descriptor(1, buf_cs.gpu_va).unwrap();
        l.draw(3, 1, 0, 0).unwrap();

        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();

        let v2_d = stats::c::cs_to_ps_overlap_v2.load(Relaxed) - v2_0;
        let hit_d = stats::c::dv_hit.load(Relaxed) - hit0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g48_A must produce 0 bad timestamps");
        alloc.reset();
        (v2_d, vtx_overlapped, hit_d, buf_words(&rb, 1)[0])
    };

    let (v2_off, ov_off, hit_off, px_off) = run_case(false);
    let (v2_on, ov_on, hit_on, px_on) = run_case(true);
    assert_eq!(v2_off, 0);
    assert!(!ov_off);
    assert!(ov_on, "g48_A must keep soft items unpoisoned across cs_uav_mid_skip + compute_mid_queue_sync_with");
    assert!(v2_on >= 1, "cs_to_ps_overlap_v2 must increment: got {v2_on}");
    assert_eq!(hit_off, 0);
    assert_eq!(hit_on, 1);
    assert_eq!(px_on, px_off);
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g48_a_cs_to_ps_overlap_v2_clear_uav_with_v5_defer() {
    use std::sync::atomic::Ordering::Relaxed;
    let Some((rs, pso)) = build_g48_vs_srv_ps_cb0_pso("g48_a_v5_gfx") else { return };

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let buf_cs = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_anchor = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_vs, &buf_cs, &rt_anchor, &rt, &rb] {
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
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let rt_id = &rt as *const Resource as usize;
    let raw_uav_desc = {
        let mut d = [0u8; 40];
        d[0..4].copy_from_slice(&39u32.to_le_bytes());
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[16..20].copy_from_slice(&16u32.to_le_bytes());
        d[32..36].copy_from_slice(&1u32.to_le_bytes());
        d
    };

    let q = Queue::new(ListType::Direct, "g48_a_v5 queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |enable_v2: bool| -> (u64, bool, u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_cs_to_ps_overlap = false;
        l.opts.opt_cs_to_ps_overlap_v2 = enable_v2;
        l.opts.opt_cs_mid_qb_res_defer_v5 = true;
        let v2_0 = stats::c::cs_to_ps_overlap_v2.load(Relaxed);
        let hit0 = stats::c::dv_hit.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        l.set_render_targets(vec![rtv_anchor], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        // Same-state UAV clear under `opt_cs_mid_qb_res_defer_v5 = true`:
        l.clear_uav(&buf_cs, Some(&raw_uav_desc), ClearValue::Uint([0x42; 4]), &[]).unwrap();

        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.set_render_targets(vec![rtv], None);
        l.draw(3, 1, 0, 0).unwrap();
        let vtx_overlapped = l.bar.enc_cur_sync[0].gen[STAGE_DISPATCH] < l.bar.stage_gen[STAGE_DISPATCH];

        l.gfx_root.set_descriptor(1, buf_cs.gpu_va).unwrap();
        l.draw(3, 1, 0, 0).unwrap();

        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();

        let v2_d = stats::c::cs_to_ps_overlap_v2.load(Relaxed) - v2_0;
        let hit_d = stats::c::dv_hit.load(Relaxed) - hit0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0);
        alloc.reset();
        (v2_d, vtx_overlapped, hit_d, buf_words(&rb, 1)[0])
    };

    let (v2_off, ov_off, hit_off, px_off) = run_case(false);
    let (v2_on, ov_on, hit_on, px_on) = run_case(true);
    assert_eq!(v2_off, 0);
    assert!(!ov_off, "without g48_A, util_dispatch_with under v5 forced hard PRE_RASTER wait");
    assert!(ov_on, "g48_A must defer PRE_RASTER per-resource even when opt_cs_mid_qb_res_defer_v5 is active");
    assert!(v2_on >= 1);
    assert_eq!(hit_off, 0);
    assert_eq!(hit_on, 1);
    assert_eq!(px_on, px_off);
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g48_b_gfx_to_ps_overlap_defers_fragment_producer_for_disjoint_vs() {
    use std::sync::atomic::Ordering::Relaxed;
    let Some((rs, pso)) = build_g48_vs_srv_ps_cb0_pso("g48_b_gfx") else { return };

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_prod = test_buffer(64, heap_type::UPLOAD, 0);
    let buf_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let rt0 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt1 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_prod, &buf_vs, &rt0, &rt1, &rb] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let prod_id = &buf_prod as *const Resource as usize;
    let rt0_id = &rt0 as *const Resource as usize;
    let rt1_id = &rt1 as *const Resource as usize;
    let rtv0 = TargetView::rtv(&rt0, None).unwrap();
    let rtv1 = TargetView::rtv(&rt1, None).unwrap();

    let q = Queue::new(ListType::Direct, "g48_b queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |enable_gfx_ov: bool| -> (u64, bool, u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_gfx_to_ps_overlap = enable_gfx_ov;
        let gov0 = stats::c::gfx_to_ps_overlap.load(Relaxed);
        let hit0 = stats::c::dv_hit.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // Render Pass #1 runs `Vertex | Fragment` and anchors pre-list render stages:
        l.set_render_targets(vec![rtv0], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();

        // Transition `buf_prod` from `STATE_COMMON` (`s0 == 0` -> `st_after = All`) to `NON_PIXEL_SRV | PIXEL_SRV`!
        l.barrier_list_with(
            &[
                Barrier::Transition(rt0_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
                Barrier::Transition(prod_id, STATE_COMMON, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV),
            ],
            true,
            false,
        );

        // Render Pass #2 Draw 1: VS only binds `buf_vs`, disjoint from `buf_prod`.
        l.set_render_targets(vec![rtv1], None);
        l.draw(3, 1, 0, 0).unwrap();
        let vtx_overlapped_frag = l.bar.enc_cur_sync[0].gen[STAGE_FRAGMENT] < l.bar.stage_gen[STAGE_FRAGMENT];

        // Draw 2 in Pass #2: VS now binds `buf_prod` -> `sync_pre_raster` MUST emit `dv_hit`!
        l.gfx_root.set_descriptor(1, buf_prod.gpu_va).unwrap();
        l.draw(3, 1, 0, 0).unwrap();

        l.barrier_list_with(&[Barrier::Transition(rt1_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt1, 0), None).unwrap();
        l.close().unwrap();

        let gov_d = stats::c::gfx_to_ps_overlap.load(Relaxed) - gov0;
        let hit_d = stats::c::dv_hit.load(Relaxed) - hit0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g48_B must produce 0 bad timestamps");
        alloc.reset();
        (gov_d, vtx_overlapped_frag, hit_d, buf_words(&rb, 1)[0])
    };

    let (gov_off, ov_off, hit_off, px_off) = run_case(false);
    let (gov_on, ov_on, hit_on, px_on) = run_case(true);
    assert_eq!(gov_off, 0);
    assert!(!ov_off, "baseline poisoned MTLStages::All soft obligations and blocked Vertex on Fragment");
    assert!(ov_on, "g48_B must let Pass #2 Draw 1 Vertex overlap Pass #1 Fragment");
    assert!(gov_on >= 1, "gfx_to_ps_overlap must increment: got {gov_on}");
    assert_eq!(hit_off, 0);
    assert_eq!(hit_on, 1, "g48_B must emit deferred barrier before Draw 2 when VS binds buf_prod");
    assert_eq!(px_on, px_off);
    assert_eq!(px_on, 0xff00_ff00);
}

#[test]
fn g48_c_clear_isolate_coalesce_consumes_frag_rt_pend_and_merges_slices() {
    use std::sync::atomic::Ordering::Relaxed;
    if crate::metal::device().is_none() {
        return;
    }
    let d_arr = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 2,
        MipLevels: 1,
        Format: RGBA8_UNORM,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let rt_arr = Resource::new_committed(&d_arr, &test_heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "g48c_rt_arr").unwrap();
    let rt_other = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb0 = test_buffer(1024, heap_type::READBACK, 0);
    let rb1 = test_buffer(1024, heap_type::READBACK, 0);
    for r in [&rt_arr, &rt_other, &rb0, &rb1] {
        r.register_ptr();
    }
    let rt_arr_id = &rt_arr as *const Resource as usize;
    let rt_other_id = &rt_other as *const Resource as usize;
    let rtv_arr_all = TargetView::rtv(&rt_arr, None).unwrap();
    let rtv_other = TargetView::rtv(&rt_other, None).unwrap();

    let q = Queue::new(ListType::Direct, "g48_c queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |enable_c: bool| -> (u64, u64, u64, u32, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_clear_isolate_coalesce = enable_c;
        let c0 = stats::c::clear_isolate_coalesce.load(Relaxed);
        let qbf0 = stats::c::q_barrier_frag.load(Relaxed);
        let cfc0 = stats::c::cf_close.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;

        // Seed prior Fragment producer so `PIXEL_SRV -> RENDER_TARGET` has non-empty `frag_after = Fragment`.
        unsafe {
            l.clear_rtv(&rtv_other, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
        }
        l.barrier_list_with(&[Barrier::Transition(rt_other_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV)], true, false);

        // Transition `rt_arr` `PIXEL_SRV -> RENDER_TARGET` (`frag_rt_pend` records ONLY `rt_arr_id`!),
        // clear 2 slices (`defer_clear` per slice under `g48_C`), then verify `Barrier::Uav(0)` flushes immediately!
        l.barrier_list_with(&[Barrier::Transition(rt_arr_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
        unsafe {
            l.clear_rtv(&rtv_arr_all, [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
        }
        l.barrier_list_with(&[Barrier::Uav(0)], true, false);
        assert!(l.clears.pending_clears.is_empty(), "g48_C must NOT retain clears across UAV(0)");

        // Re-clear `rt_arr` after `PIXEL_SRV -> RENDER_TARGET` to exercise standalone CLEAR pass `frag_rt_pend` consumption!
        l.barrier_list_with(&[Barrier::Transition(rt_arr_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
        unsafe {
            l.clear_rtv(&rtv_arr_all, [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
        }
        l.barrier_list_with(&[Barrier::Transition(rt_arr_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);

        // Open another pass on `rt_other`: if `frag_rt_pend` was consumed by `rt_arr`'s CLEAR pass, no extra `q_barrier_frag` leaks!
        l.barrier_list_with(&[Barrier::Transition(rt_other_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
        unsafe {
            l.clear_rtv(&rtv_other, [0.0, 0.0, 1.0, 1.0], &[]).unwrap();
        }
        l.barrier_list_with(&[Barrier::Transition(rt_other_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV)], true, false);

        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb0, fp), [0, 0, 0], CopyLoc::Subresource(&rt_arr, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb1, fp), [0, 0, 0], CopyLoc::Subresource(&rt_arr, 1), None).unwrap();
        l.close().unwrap();

        let c_d = stats::c::clear_isolate_coalesce.load(Relaxed) - c0;
        let qbf_d = stats::c::q_barrier_frag.load(Relaxed) - qbf0;
        let cfc_d = stats::c::cf_close.load(Relaxed) - cfc0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g48_C must produce 0 bad timestamps");
        alloc.reset();
        (c_d, qbf_d, cfc_d, buf_words(&rb0, 1)[0], buf_words(&rb1, 1)[0])
    };

    let (c_off, qbf_off, cfc_off, s0_off, s1_off) = run_case(false);
    let (c_on, qbf_on, cfc_on, s0_on, s1_on) = run_case(true);
    assert_eq!(c_off, 0);
    assert!(c_on >= 1, "clear_isolate_coalesce must increment: got {c_on}");
    assert_eq!(cfc_off, 0);
    assert_eq!(cfc_on, 0, "g48_C must NOT retain clears across UAV(0) into cf_close");
    assert!(qbf_on <= qbf_off, "g48_C must not increase q_barrier_frag ({qbf_on} vs {qbf_off})");
    assert_eq!(s0_on, s0_off);
    assert_eq!(s1_on, s1_off);
    assert_eq!(s0_on, 0xff00_ff00);
    assert_eq!(s1_on, 0xff00_ff00);
}

#[test]
fn g48_d_queue_start_stage_exact_fresh_upload_blit_no_mid_qb() {
    use std::sync::atomic::Ordering::Relaxed;
    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "g48_d_blit queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let src = test_buffer(64, heap_type::UPLOAD, 0);
    let dst = test_buffer(64, heap_type::DEFAULT, 0);
    let dst2 = test_buffer(64, heap_type::DEFAULT, 0);
    let rb = test_buffer(64, heap_type::READBACK, 0);
    unsafe {
        let p = src.buffer().unwrap().contents().as_ptr() as *mut u32;
        for i in 0..16 {
            std::ptr::write_volatile(p.add(i), 0x1234_5678);
        }
    }
    for r in [&src, &dst, &dst2, &rb] {
        r.register_ptr();
    }
    let dst_id = &dst as *const Resource as usize;
    let dst2_id = &dst2 as *const Resource as usize;
    assert!(src.is_fresh_nonfrag_blit_safe());
    assert!(dst.is_fresh_nonfrag_blit_safe());
    assert!(dst2.is_fresh_nonfrag_blit_safe());

    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_queue_start_stage_exact = true;
    let qs0 = stats::c::queue_start_stage_exact.load(Relaxed);
    let mqb0 = stats::c::cs_mid_qb.load(Relaxed);

    l.begin(&alloc).unwrap();
    l.pt_armed = true;
    l.barrier_list_with(&[Barrier::Transition(dst_id, STATE_COMMON, STATE_COPY_DEST)], true, false);
    l.copy_buffer(&dst, 0, &src, 0, 64).unwrap();
    assert!(l.bar.list_cs_unwaited_frag, "narrowed L0#0 upload Blit must track omitted Fragment wait");
    l.barrier_list_with(&[Barrier::Transition(dst2_id, STATE_COMMON, STATE_COPY_DEST)], true, false);
    l.copy_buffer(&dst2, 0, &src, 0, 64).unwrap();
    l.barrier_list_with(&[Barrier::Transition(dst2_id, STATE_COPY_DEST, STATE_COPY_SOURCE)], true, false);
    l.copy_buffer(&rb, 0, &dst2, 0, 64).unwrap();
    l.close().unwrap();

    let qs_d = stats::c::queue_start_stage_exact.load(Relaxed) - qs0;
    let mqb_d = stats::c::cs_mid_qb.load(Relaxed) - mqb0;
    assert!(qs_d >= 1, "queue_start_stage_exact must increment for fresh L0#0 upload Blit");
    assert_eq!(mqb_d, 0, "multi-copy fresh upload Blit narrowing must NOT emit compensatory cs_mid_qb");
    assert!(!dst.is_fresh_nonfrag_blit_safe(), "dst must be marked gpu_used after copy_buffer");
    assert!(!dst2.is_fresh_nonfrag_blit_safe(), "dst2 must be marked gpu_used after copy_buffer");

    let mut lg = l.take_pass_log().unwrap();
    lg.submitted(pt::mach_now(), 0, 0);
    q.submit_with_logs(&[&l.cb], Vec::new());
    q.wait_idle();
    let summary = pt::resolve_for_test(vec![lg]);
    assert_eq!(summary.bad, 0, "g48_D Blit test must produce 0 bad timestamps");
    assert_eq!(buf_words(&rb, 1)[0], 0x1234_5678);
    alloc.reset();
}

#[test]
fn g48_d_queue_start_stage_exact_draw0_render_split_with_sticky_frag_buf() {
    use std::sync::atomic::Ordering::Relaxed;
    let Some((rs, pso)) = build_g48_vs_srv_ps_cb0_pso("g48_d_draw0_gfx") else { return };

    // Make `any_frag_buf_write()` sticky-true (just like gameplay once any PS UAV buffer is compiled).
    crate::metal::pipeline::note_frag_buf_write();

    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_ro_vs = test_buffer(64, heap_type::UPLOAD, 0);
    let buf_uav_vs = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_ro_vs, &buf_uav_vs, &rt, &rb] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let rt_id = &rt as *const Resource as usize;
    let rtv = TargetView::rtv(&rt, None).unwrap();

    let q = Queue::new(ListType::Direct, "g48_d_draw0 queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _guard = gpu_stats_lock();

    let run_case = |enable_d: bool| -> (u64, bool, u64, u32) {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_queue_start_stage_exact = enable_d;
        let qs0 = stats::c::queue_start_stage_exact.load(Relaxed);
        let hit0 = stats::c::dv_hit.load(Relaxed);

        l.begin(&alloc).unwrap();
        l.pt_armed = true;
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
        l.gfx_root.set_descriptor(1, buf_ro_vs.gpu_va).unwrap();
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);

        // Draw 0 at list start (`L3#0`/`L4#0`): VS reads `buf_ro_vs` (`ALLOW_UNORDERED_ACCESS == 0`).
        l.set_render_targets(vec![rtv], None);
        l.draw(3, 1, 0, 0).unwrap();
        let draw0_vtx_skipped_frag = !l.bar.enc_cur_sync[0].pre_list_stages.contains(MTLStages::Fragment);

        // Draw 1 in the SAME pass: VS reads `buf_uav_vs` -> armed `vtx_defer` MUST emit `dv_hit`!
        l.gfx_root.set_descriptor(1, buf_uav_vs.gpu_va).unwrap();
        l.draw(3, 1, 0, 0).unwrap();
        let draw1_vtx_synced_frag = l.bar.enc_cur_sync[0].pre_list_stages.contains(MTLStages::Fragment);
        assert!(draw1_vtx_synced_frag, "Draw 1 must have pre-list Fragment synced on Vertex");

        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();

        let qs_d = stats::c::queue_start_stage_exact.load(Relaxed) - qs0;
        let hit_d = stats::c::dv_hit.load(Relaxed) - hit0;
        let mut lg = l.take_pass_log().unwrap();
        lg.submitted(pt::mach_now(), 0, 0);
        q.submit(&[&l.cb]);
        q.wait_idle();
        let summary = pt::resolve_for_test(vec![lg]);
        assert_eq!(summary.bad, 0, "g48_D Draw 0 split test must produce 0 bad timestamps");
        alloc.reset();
        (qs_d, draw0_vtx_skipped_frag, hit_d, buf_words(&rb, 1)[0])
    };

    let (qs_off, skip_off, hit_off, px_off) = run_case(false);
    let (qs_on, skip_on, hit_on, px_on) = run_case(true);
    assert_eq!(qs_off, 0);
    assert!(!skip_off, "baseline blocked Draw 0 Vertex on Fragment due to sticky any_frag_buf_write()");
    assert!(skip_on, "g48_D must let Draw 0 Vertex skip pre-list Fragment when Draw 0 binds only read-only buffers");
    assert!(qs_on >= 1, "queue_start_stage_exact must increment: got {qs_on}");
    assert_eq!(hit_off, 0);
    assert_eq!(hit_on, 1, "g48_D must fire deferred Fragment -> PRE_RASTER before Draw 1");
    assert_eq!(px_on, px_off);
    assert_eq!(px_on, 0xff00_ff00);
}
