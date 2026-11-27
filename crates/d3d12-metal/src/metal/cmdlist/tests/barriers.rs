use super::*;

#[test]
fn clear_overlap_exact_prevents_fragment_reinjection_in_post_clear_compute_and_orders_true_clear_consumers_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputePipeline, GraphicsDesc, GraphicsPipeline, READS_BUF};
    use crate::metal::views;
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

    let cs_base = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g34_stage_a_cs").unwrap();
    // Model `skinning:transformCS`: bounded `t0` Texture SRV + exact `u0` UAV + unbounded Buffer SRVs (`READS_BUF`).
    let cs_skinning_like = Arc::new(ComputePipeline {
        pso: cs_base.pso,
        rs: cs_base.rs,
        threadgroup: cs_base.threadgroup,
        shader: cs_base.shader,
        hk_profile: cs_base.hk_profile,
        access: ComputeAccess {
            uav_slots: cs_base.access.uav_slots,
            read_slots: None,
            bounded_read_slots: cs_base.access.bounded_read_slots,
            unbounded_srv_classes: READS_BUF,
            heap_indexed_or_unknown: false,
            touches_textures: true,
            ..ComputeAccess::opaque()
        },
    });

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
            "g34_stage_a_gfx",
        )
        .unwrap(),
    );

    let q = Queue::new(ListType::Direct, "g34 stage A queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_uav0_clear_overlap = true;
    l.opts.opt_clear_overlap_exact = true;

    let tex_src = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let ds_shadow = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let rt_shadow = test_texture(8, 8, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET); // non-ShaderWrite -> CLEAR render pass!
    let ds_draw = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let pre_buf = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let out_buf0 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let out_buf2 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let out_clear_consumer = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_pre = test_buffer(512, heap_type::READBACK, 0);
    let rb_out2 = test_buffer(512, heap_type::READBACK, 0);
    let rb_cc = test_buffer(512, heap_type::READBACK, 0);
    let rb_ds = test_buffer(256 * 8, heap_type::READBACK, 0);

    for r in [&tex_src, &ds_shadow, &rt_shadow, &ds_draw, &pre_buf, &out_buf0, &out_buf2, &out_clear_consumer] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g34 stage A heap").unwrap();
    unsafe {
        let e0 = views::srv_entry(Some(&tex_src), None).unwrap();
        let e1 = views::srv_entry(Some(&rt_shadow), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        std::ptr::copy_nonoverlapping(e1.as_ptr(), (heap.cpu_base + 24) as *mut u8, 24);
    }

    let ds_shadow_id = &ds_shadow as *const Resource as usize;
    let rt_shadow_id = &rt_shadow as *const Resource as usize;
    let pre_buf_id = &pre_buf as *const Resource as usize;
    let out2_id = &out_buf2 as *const Resource as usize;
    let cc_id = &out_clear_consumer as *const Resource as usize;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.set_descriptor_heaps(heap.gpu_base, 0);

    // Populate `tex_src` via compute clear, transition to SRV, and queue non-compute DSV + RTV clears.
    let rtv_src = TargetView::rtv(&tex_src, None).unwrap();
    let dsv_sh = TargetView::dsv(&ds_shadow, None).unwrap();
    let rtv_sh = TargetView::rtv(&rt_shadow, None).unwrap();
    let dsv_dr = TargetView::dsv(&ds_draw, None).unwrap();
    unsafe {
        l.clear_rtv(&rtv_src, [5.0, 10.0, 15.0, 1.0], &[]).unwrap();
    }
    l.barrier_list_with(
        &[Barrier::Transition(&tex_src as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)],
        true,
        true,
    );
    unsafe {
        l.clear_dsv(&dsv_sh, true, 0.25, false, 0, &[]).unwrap();
        l.clear_rtv(&rtv_sh, [21.0, 42.0, 63.0, 1.0], &[]).unwrap();
        l.clear_dsv(&dsv_dr, true, 0.875, false, 0, &[]).unwrap();
    }

    // `L4#0`: Dispatch #0 consumes `FullWhy::ListStart` (`All -> All`) and runs `Dispatch`,
    // anchoring all pre-list GPU work at `stage_sync[STAGE_DISP]` once `L4#0` closes!
    l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap(); // slot 0 = tex_src
    l.compute_root.set_descriptor(1, out_buf0.gpu_va).unwrap();
    l.set_compute_pipeline(cs_skinning_like.clone());
    l.dispatch(1, 1, 1).unwrap();

    // `Barrier::Uav(0)` closes `L4#0` (`pre_list_anchor_stages() == Dispatch`) and flushes the
    // coalesced `CLEAR` passes (`ds_shadow`, `rt_shadow`, `ds_draw` recorded in `clear_rp_res_ids`).
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert_eq!(l.pre_list_anchor_stages(), MTLStages::Dispatch);
    assert!(l.bar.clear_rp_res_ids.contains(&ds_shadow_id) && l.bar.clear_rp_res_ids.contains(&rt_shadow_id));
    assert_eq!(l.bar.pend_after, MTLStages::Dispatch);

    // Emit the exact post-clear transitions that regressed `g33_B`:
    // 1) `ds_shadow`: `DEPTH_WRITE -> NON_PIXEL_SRV | PIXEL_SRV` (clear target -> deferred into `clear_tex_cs` + `clear_tex_gfx_before`!)
    // 2) `pre_buf`:   `PIXEL_SRV -> UAV` (pre-list resource -> `Fragment` already transitively anchored by `L4#0` `Dispatch`!)
    let ex0 = stats::c::clear_overlap_exact.load(Relaxed);
    l.barrier_list_with(
        &[
            Barrier::Transition(ds_shadow_id, STATE_DEPTH_WRITE, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV),
            Barrier::Transition(pre_buf_id, STATE_PIXEL_SRV, STATE_UAV),
        ],
        true,
        false,
    );
    assert!(stats::c::clear_overlap_exact.load(Relaxed) > ex0);
    assert_eq!(
        l.bar.pend_after,
        MTLStages::Dispatch,
        "AW2_CLEAR_OVERLAP_EXACT=1 must NOT re-inject Fragment into pend_after before L4#5"
    );
    assert!(l.bar.clear_tex_cs.contains(&ds_shadow_id), "cleared DSV SRV transition must track ds_shadow in clear_tex_cs");
    assert!(l.bar.clear_tex_gfx_before.contains(MTLStages::Fragment));

    // `L4#5`: Dispatch #1 -> mid-encoder `Barrier::Uav(0)` + `Barrier::Uav(pre_buf_id)` -> Dispatch #2!
    // Neither `L4#5` opening NOR mid-encoder `Barrier::Uav` may inject `Fragment` or `cs_mid_qb`!
    let mid0 = stats::c::cs_mid_qb.load(Relaxed);
    l.compute_root.set_descriptor(1, pre_buf.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(l.bar.enc_sync_after, MTLStages::Dispatch, "L4#5 must open waiting ONLY on Dispatch, overlapping CLEAR passes");
    assert!(l.bar.clear_tex_cs.contains(&ds_shadow_id), "unrelated skinning dispatch must NOT consume clear_tex_cs");

    l.barrier_list_with(&[Barrier::Uav(0), Barrier::Uav(pre_buf_id)], true, false);
    l.compute_root.set_descriptor(1, out_buf2.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(l.bar.enc_sync_after, MTLStages::Dispatch, "mid-encoder Barrier::Uav must NOT upgrade L4#5 to Fragment/AccelerationStructure");
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed), mid0, "0 cs_mid_qb stalls allowed across L4#5 skinning dispatches");

    // Guardrail 1 (True compute consumer of cleared `rt_shadow`):
    // Transition `rt_shadow` `RENDER_TARGET -> NON_PIXEL_SRV`, then bind `rt_shadow` (heap slot 1) in `dispatch`!
    l.barrier_list_with(&[Barrier::Transition(rt_shadow_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    assert!(l.bar.clear_tex_cs.contains(&rt_shadow_id));
    l.compute_root.set_table(0, heap.gpu_base + 24).unwrap(); // slot 1 = rt_shadow!
    l.compute_root.set_descriptor(1, out_clear_consumer.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert!(!l.bar.clear_tex_cs.contains(&rt_shadow_id), "binding rt_shadow in dispatch must consume clear_tex_cs entry");
    assert!(
        l.bar.enc_sync_after.contains(MTLStages::Fragment),
        "true compute consumer of cleared texture must upgrade sync to wait on Fragment"
    );

    // Guardrail 2 (Subsequent graphics draw pass flushes `clear_tex_gfx_before` and sets `in_list_gfx_non_clear`):
    l.gfx_root.set_root_signature(gfx_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
    l.set_render_targets(Vec::new(), Some(dsv_dr));
    l.gfx.topology = 4;
    l.set_graphics_pipeline(gfx_pso);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.5, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(l.bar.in_list_gfx_non_clear);
    assert!(l.bar.clear_tex_gfx_before.is_empty(), "opening draw pass must flush clear_tex_gfx_before");

    l.barrier_list_with(
        &[
            Barrier::Transition(ds_shadow_id, STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV, STATE_COPY_SOURCE),
            Barrier::Transition(pre_buf_id, STATE_UAV, STATE_COPY_SOURCE),
            Barrier::Transition(out2_id, STATE_UAV, STATE_COPY_SOURCE),
            Barrier::Transition(cc_id, STATE_UAV, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_ds, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds_shadow, 0), None).unwrap();
    l.copy_buffer(&rb_pre, 0, &pre_buf, 0, 64).unwrap();
    l.copy_buffer(&rb_out2, 0, &out_buf2, 0, 64).unwrap();
    l.copy_buffer(&rb_cc, 0, &out_clear_consumer, 0, 64).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    let d0 = f32::from_bits(buf_words(&rb_ds, 1)[0]);
    assert!((d0 - 0.25).abs() < 1e-5, "cleared ds_shadow depth mismatch: got {d0}");
    let v_pre: Vec<f32> = buf_words(&rb_pre, 4).into_iter().map(f32::from_bits).collect();
    let v_out2: Vec<f32> = buf_words(&rb_out2, 4).into_iter().map(f32::from_bits).collect();
    let v_cc: Vec<f32> = buf_words(&rb_cc, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&v_pre[..], &[5.0, 10.0, 15.0, 1.0]);
    assert_eq!(&v_out2[..], &[5.0, 10.0, 15.0, 1.0]);
    assert_eq!(&v_cc[..], &[21.0, 42.0, 63.0, 1.0]);
}

#[test]
fn subrange_uav_barrier_skips_disjoint_suballocations_and_mid_cs_uav_qb_while_ordering_overlaps_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::ComputePipeline;
    use crate::metal::views;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootParameter, RootParameterKind, RootSignature,
        RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    const RGBA32_FLOAT: u32 = 2;

    // Bind BOTH `t0` (param 0) and `u0` (param 1) through descriptor tables so `u0` carries
    // exact `[FirstElement, NumElements, StructureByteStride]` byte sub-range metadata in `heap`!
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
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Uav,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g34_stage_b_cs").unwrap());

    let q = Queue::new(ListType::Direct, "g34 stage B queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_subrange_uav_barrier = true;

    let tex_a = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_b = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_c = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_non_cs = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    // Single 2048-byte `ID3D12Resource` buffer sub-allocated across 3 structured UAV views (`stride = 16`, `NumElements = 32` -> 512B):
    // - slot 3: `[0, 512)`   (`FirstElement = 0`)
    // - slot 4: `[512, 1024)` (`FirstElement = 32` — disjoint from `[0, 512)`!)
    // - slot 5: `[256, 768)` (`FirstElement = 16` — overlaps BOTH `[0, 512)` and `[512, 1024)`!)
    let shared_buf = test_buffer(2048, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_buf = test_buffer(2048, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &tex_c, &rt_non_cs, &shared_buf, &rb_buf] {
        r.register_ptr();
    }

    let make_struct_uav = |first_elem: u64, num_elems: u32, stride: u32| -> [u8; 40] {
        let mut d = [0u8; 40];
        d[4..8].copy_from_slice(&1u32.to_le_bytes()); // D3D12_UAV_DIMENSION_BUFFER = 1
        d[8..16].copy_from_slice(&first_elem.to_le_bytes());
        d[16..20].copy_from_slice(&num_elems.to_le_bytes());
        d[20..24].copy_from_slice(&stride.to_le_bytes());
        d
    };

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g34 stage B heap").unwrap();
    let write_entry = |slot: usize, entry: &[u8; 24]| unsafe {
        std::ptr::copy_nonoverlapping(entry.as_ptr(), (heap.cpu_base + slot * 24) as *mut u8, 24);
    };
    write_entry(0, &views::srv_entry(Some(&tex_a), None).unwrap());
    write_entry(1, &views::srv_entry(Some(&tex_b), None).unwrap());
    write_entry(2, &views::srv_entry(Some(&tex_c), None).unwrap());
    write_entry(3, &views::uav_entry(Some(&shared_buf), Some(&make_struct_uav(0, 32, 16))).unwrap());
    write_entry(4, &views::uav_entry(Some(&shared_buf), Some(&make_struct_uav(32, 32, 16))).unwrap());
    write_entry(5, &views::uav_entry(Some(&shared_buf), Some(&make_struct_uav(16, 32, 16))).unwrap());

    let shared_id = &shared_buf as *const Resource as usize;
    let _stats_guard = gpu_stats_lock();
    l.opts.opt_ib_substage_narrow = false;
    l.begin(&alloc).unwrap();
    l.set_descriptor_heaps(heap.gpu_base, 0);

    // Populate `tex_a`, `tex_b`, `tex_c`, then run a render-pass RTV clear on `rt_non_cs` (`in_list_gfx_non_clear` / post-RTV compute encoder)
    // so the subsequent compute encoder opens WITHOUT `All` (`enc_sync_after == Dispatch`, `gfx_uav_gen == 0`) —
    // the exact condition that previously triggered 15.6 `cs_mid_qb/f` on `Barrier::Uav`!
    unsafe {
        l.clear_rtv(&TargetView::rtv(&tex_a, None).unwrap(), [10.0, 20.0, 30.0, 1.0], &[]).unwrap();
        l.clear_rtv(&TargetView::rtv(&tex_b, None).unwrap(), [40.0, 50.0, 60.0, 1.0], &[]).unwrap();
        l.clear_rtv(&TargetView::rtv(&tex_c, None).unwrap(), [70.0, 80.0, 90.0, 1.0], &[]).unwrap();
    }
    l.barrier_list_with(
        &[
            Barrier::Transition(&tex_a as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
            Barrier::Transition(&tex_b as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
            Barrier::Transition(&tex_c as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
        ],
        true,
        true,
    );
    l.end_encoder();
    l.clear_all_pend();
    // Force a non-compute render-pass clear via `Barrier::Uav(0)` to close the initial
    // `All`-synced compute encoder and open a fresh stage-masked (`enc_sync_after == Dispatch`)
    // compute encoder — the exact D3D12 condition that previously fired `cs_mid_qb`!
    unsafe {
        l.clear_rtv(&TargetView::rtv(&rt_non_cs, None).unwrap(), [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
    }
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(matches!(l.enc, Encoder::None));
    assert_eq!(l.bar.pend_after, MTLStages::Dispatch);

    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.set_compute_pipeline(cs_pso);

    // Dispatch #1 writes sub-allocation `[0, 512)` (heap slot 3) from `tex_a` (heap slot 0).
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_table(1, heap.gpu_base + 3 * 24).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(l.bar.enc_sync_after, MTLStages::Dispatch);
    assert!(l.open_compute_disp_uav_synced(), "post-clear compute encoder must satisfy open_compute_disp_uav_synced()");

    // Record `Barrier::Uav(shared_id)` + `Barrier::Uav(0)` inside the open compute encoder, then
    // Dispatch #2 writing DISJOINT sub-allocation `[512, 1024)` (heap slot 4) from `tex_b` (heap slot 1):
    // - `cs_uav_mid_skip` MUST defer cross-encoder `pend_after/before` (`cs_mid_qb` delta == 0!)
    // - `subrange_uav_skip` MUST skip `enc_barrier` (`enc_barrier` delta == 0!)
    let sub_skip0 = stats::c::subrange_uav_skip.load(Relaxed);
    let mid_skip0 = stats::c::cs_uav_mid_skip.load(Relaxed);
    let enc_b0 = stats::c::enc_barrier.load(Relaxed);
    let cs_mid0 = stats::c::cs_mid_qb.load(Relaxed);

    let pre_pend = l.bar.pend_after;
    l.barrier_list_with(&[Barrier::Uav(shared_id)], true, false);
    assert!(stats::c::cs_uav_mid_skip.load(Relaxed) > mid_skip0, "Barrier::Uav in synced compute encoder must defer cross-encoder sync");
    assert!(
        l.bar.split_pend.disp_after.is_empty() && l.bar.split_pend.blit_after.is_empty() && l.bar.pend_after == pre_pend,
        "compute pend_after must stay empty inside synced compute encoder"
    );

    l.compute_root.set_table(0, heap.gpu_base + 24).unwrap(); // tex_b
    l.compute_root.set_table(1, heap.gpu_base + 4 * 24).unwrap(); // `[512, 1024)` — disjoint from `[0, 512)`!
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed), cs_mid0, "0 cs_mid_qb stalls allowed on intra-compute Barrier::Uav");
    assert_eq!(stats::c::enc_barrier.load(Relaxed), enc_b0, "disjoint sub-allocation [512,1024) must skip enc_barrier");
    assert!(stats::c::subrange_uav_skip.load(Relaxed) > sub_skip0, "expected subrange_uav_skip increment");
    assert!(!l.ib.ib_obl_tight.is_empty(), "unconsumed [0,512) obligation must stay pending for overlapping consumers");

    // Guardrail 1: record `Barrier::Uav(0)` (expands `[0, 1024)`), then Dispatch #3 writing OVERLAPPING
    // `[256, 768)` (heap slot 5) from `tex_c` (heap slot 2) -> MUST emit `enc_barrier`!
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    let enc_b1 = stats::c::enc_barrier.load(Relaxed);
    l.compute_root.set_table(0, heap.gpu_base + 2 * 24).unwrap(); // tex_c
    l.compute_root.set_table(1, heap.gpu_base + 5 * 24).unwrap(); // `[256, 768)` — overlaps!
    l.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::enc_barrier.load(Relaxed) > enc_b1, "overlapping [256,768) sub-allocation MUST emit enc_barrier");
    assert!(l.ib.ib_obl_tight.is_empty(), "firing enc_barrier must consume pending intra-compute obligations");

    // Guardrail 2: `end_encoder()` MUST flush deferred `cs_uav_after/before` into `pend_after/before`!
    l.barrier_list_with(&[Barrier::Uav(shared_id)], true, false);
    assert!(!l.bar.cs_uav_after.is_empty());
    l.end_encoder();
    assert!(l.bar.cs_uav_after.is_empty(), "end_encoder must flush deferred cs_uav_after");
    assert!(l.bar.pend_after.contains(MTLStages::Dispatch), "flushed cs_uav_after must populate pend_after for subsequent encoders");

    l.barrier_list_with(&[Barrier::Transition(shared_id, STATE_UAV, STATE_COPY_SOURCE)], true, true);
    l.copy_buffer(&rb_buf, 0, &shared_buf, 0, 1024).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    // Verify byte-exact GPU memory across all 64 `float4` elements (`[0, 1024)`):
    // - elements  0..16 (`[0, 256)`):   `[10, 20, 30, 1]` (Dispatch #1)
    // - elements 16..48 (`[256, 768)`): `[70, 80, 90, 1]` (Dispatch #3 overwrite ordered after #1 & #2!)
    // - elements 48..64 (`[768, 1024)`):`[40, 50, 60, 1]` (Dispatch #2)
    let floats: Vec<f32> = buf_words(&rb_buf, 256).into_iter().map(f32::from_bits).collect();
    for elem in 0..64 {
        let got = &floats[elem * 4..elem * 4 + 4];
        let want: &[f32; 4] = if elem < 16 {
            &[10.0, 20.0, 30.0, 1.0]
        } else if elem < 48 {
            &[70.0, 80.0, 90.0, 1.0]
        } else {
            &[40.0, 50.0, 60.0, 1.0]
        };
        assert_eq!(got, want, "shared_buf float4[{elem}] mismatch");
    }
}

#[test]
fn copy_queue_light_narrows_copy_list_barriers_to_blit_without_touching_direct_queue_on_gpu() {
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() {
        return;
    }
    let q_copy = Queue::new(ListType::Copy, "g34 stage D copy queue").unwrap();
    let a_copy = CommandAllocator::new(ListType::Copy).unwrap();
    let mut l_copy = CommandList::new(ListType::Copy).unwrap();
    l_copy.opts.opt_copy_queue_light = true;

    let up_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let mid_buf = test_buffer(64, heap_type::DEFAULT, 0);
    let rb_buf = test_buffer(64, heap_type::READBACK, 0);
    unsafe {
        let p = up_buf.buffer().unwrap().contents().as_ptr() as *mut u32;
        *p.add(0) = 0x1122_3344;
        *p.add(1) = 0x5566_7788;
    }
    // Simulate a previously GPU-used buffer so `PORT_QUEUE_START_STAGE_EXACT`'s fresh-upload
    // shortcut does not elide the `COMMON -> COPY_DEST` barrier before `opt_copy_queue_light` runs.
    mid_buf.mark_gpu_used();
    let mid_id = &mid_buf as *const Resource as usize;

    let _stats_guard = gpu_stats_lock();
    let cql0 = stats::c::copy_q_light.load(Relaxed);

    l_copy.begin(&a_copy).unwrap();
    // Copy #1 on `ListType::Copy`: `FullWhy::ListStart` + implicit `COMMON` transition MUST narrow to `Blit -> Blit`!
    l_copy.barrier_list_with(&[Barrier::Transition(mid_id, STATE_COMMON, STATE_COPY_DEST)], true, true);
    l_copy.copy_buffer(&mid_buf, 0, &up_buf, 0, 8).unwrap();
    assert_eq!(l_copy.bar.enc_sync_after, MTLStages::Blit, "ListType::Copy ListStart must narrow to MTLStages::Blit");
    assert!(stats::c::copy_q_light.load(Relaxed) >= cql0 + 2);

    // Chained Copy #2 + `Barrier::Full` inside the same `ListType::Copy` encoder (`mid_buf -> rb_buf`):
    // ordered via `Blit -> Blit` without ever escalating `enc_sync_after` to `MTLStages::All`!
    l_copy.barrier_list_with(&[Barrier::Transition(mid_id, STATE_COPY_DEST, STATE_COPY_SOURCE)], true, true);
    l_copy.barrier();
    l_copy.copy_buffer(&rb_buf, 0, &mid_buf, 0, 8).unwrap();
    assert_eq!(l_copy.bar.enc_sync_after, MTLStages::Blit, "mid-list Copy transitions/Full must stay Blit -> Blit");
    assert!(stats::c::copy_q_light.load(Relaxed) >= cql0 + 4);
    l_copy.close().unwrap();

    q_copy.submit(&[&l_copy.cb]);
    q_copy.wait_idle();
    assert_eq!(&buf_words(&rb_buf, 2)[..], &[0x1122_3344, 0x5566_7788]);

    // Guardrail: `ListType::Direct` (`q0`) MUST remain untouched (`enc_sync_after == All`) even when `opt_copy_queue_light = true`!
    let a_dir = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l_dir = CommandList::new(ListType::Direct).unwrap();
    l_dir.opts.opt_copy_queue_light = true;
    l_dir.begin(&a_dir).unwrap();
    l_dir.copy_buffer(&mid_buf, 0, &up_buf, 0, 8).unwrap();
    assert_eq!(l_dir.bar.enc_sync_after, MTLStages::All, "AW2_COPY_QUEUE_LIGHT=1 must NOT alter ListType::Direct (q0)");
    l_dir.close().unwrap();
}

#[test]
fn stagesync_barrier_v2_skips_transitive_mid_pass_barriers_and_keeps_clear_tex_across_buffer_only_cs_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputePipeline, GraphicsDesc, GraphicsPipeline, READS_BUF};
    use crate::metal::views;
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

    let cs_base = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g35_stage_c_cs").unwrap();
    // Model SM 6.6 bindless buffer-only `skinning:transformCS` (`heap_indexed_or_unknown = true`,
    // `"ResourceUsages": 6` -> `touches_textures = false`!).
    let cs_buf_only = Arc::new(ComputePipeline {
        pso: cs_base.pso.clone(),
        rs: cs_base.rs.clone(),
        threadgroup: cs_base.threadgroup,
        shader: cs_base.shader.clone(),
        hk_profile: cs_base.hk_profile,
        access: ComputeAccess {
            uav_slots: cs_base.access.uav_slots.clone(),
            read_slots: None,
            bounded_read_slots: Vec::new(),
            unbounded_srv_classes: READS_BUF,
            heap_indexed_or_unknown: true,
            touches_textures: false,
            ..ComputeAccess::opaque()
        },
    });
    // True texture-reading compute shader (`touches_textures = true`).
    let cs_tex_reader = Arc::new(ComputePipeline {
        pso: cs_base.pso,
        rs: cs_base.rs,
        threadgroup: cs_base.threadgroup,
        shader: cs_base.shader,
        hk_profile: cs_base.hk_profile,
        access: ComputeAccess {
            uav_slots: cs_base.access.uav_slots,
            read_slots: None,
            bounded_read_slots: cs_base.access.bounded_read_slots,
            unbounded_srv_classes: READS_BUF,
            heap_indexed_or_unknown: false,
            touches_textures: true,
            ..ComputeAccess::opaque()
        },
    });

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
            "g35_stage_c_gfx",
        )
        .unwrap(),
    );

    let q = Queue::new(ListType::Direct, "g35 stage C queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_stagesync_barrier_v2 = true;

    let tex_src = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_simul = test_texture(
        4,
        1,
        RGBA32_FLOAT,
        resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_SIMULTANEOUS_ACCESS,
    );
    let ds_pass1 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let ds_pass2 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let pre_vb = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let out_buf0 = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let out_skin = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let out_simul = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_skin = test_buffer(256, heap_type::READBACK, 0);
    let rb_simul = test_buffer(256, heap_type::READBACK, 0);
    let rb_d1 = test_buffer(256 * 4, heap_type::READBACK, 0);

    for r in [&tex_src, &rt_simul, &ds_pass1, &ds_pass2, &pre_vb, &out_buf0, &out_skin, &out_simul] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g35 stage C heap").unwrap();
    unsafe {
        let e0 = views::srv_entry(Some(&tex_src), None).unwrap();
        let e1 = views::srv_entry(Some(&rt_simul), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
        std::ptr::copy_nonoverlapping(e1.as_ptr(), (heap.cpu_base + 24) as *mut u8, 24);
    }

    let rt_simul_id = &rt_simul as *const Resource as usize;
    let pre_vb_id = &pre_vb as *const Resource as usize;
    let out_skin_id = &out_skin as *const Resource as usize;
    let out_simul_id = &out_simul as *const Resource as usize;
    let ds1_id = &ds_pass1 as *const Resource as usize;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.set_descriptor_heaps(heap.gpu_base, 0);

    unsafe {
        l.clear_rtv(&TargetView::rtv(&tex_src, None).unwrap(), [8.0, 16.0, 24.0, 1.0], &[]).unwrap();
    }
    l.barrier_list_with(
        &[Barrier::Transition(&tex_src as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)],
        true,
        true,
    );
    unsafe {
        l.clear_rtv(&TargetView::rtv(&rt_simul, None).unwrap(), [7.0, 14.0, 21.0, 1.0], &[]).unwrap();
        l.clear_dsv(&TargetView::dsv(&ds_pass1, None).unwrap(), true, 0.8, false, 0, &[]).unwrap();
        l.clear_dsv(&TargetView::dsv(&ds_pass2, None).unwrap(), true, 0.9, false, 0, &[]).unwrap();
    }

    // `L4#0`: Dispatch #0 consumes `FullWhy::ListStart` (`All -> All`).
    l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, out_buf0.gpu_va).unwrap();
    l.set_compute_pipeline(cs_tex_reader.clone());
    l.dispatch(1, 1, 1).unwrap();

    // `Barrier::Uav(0)` closes `L4#0` (`stage_gen[Dispatch] == 1`) and flushes `CLEAR` render passes (`stage_gen[Fragment] >= 1`).
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(l.bar.clear_rp_res_ids.contains(&rt_simul_id), "non-UAV SIMUL RTV must clear via CLEAR render pass");

    // Transition `rt_simul` (`ALLOW_SIMULTANEOUS_ACCESS`) `RTV -> PIXEL_SRV`: under `opt_stagesync_barrier_v2`,
    // `track_exact_clear` MUST defer `rt_simul` into `clear_tex_cs` without injecting `Fragment` into `pend_after`!
    l.barrier_list_with(&[Barrier::Transition(rt_simul_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV)], true, false);
    assert!(l.bar.clear_tex_cs.contains(&rt_simul_id), "SIMUL clear target must defer into clear_tex_cs");
    assert_eq!(l.bar.pend_after, MTLStages::Dispatch);

    // `L4#4`: run buffer-only SM 6.6 `cs_buf_only` (`!touches_textures`, `heap_indexed_or_unknown = true`):
    // MUST keep `clear_tex_cs` un-flushed (`clear_tex_cs_keep`++) and open `L4#4` waiting ONLY on `Dispatch`!
    let keep0 = stats::c::clear_tex_cs_keep.load(Relaxed);
    let mid0 = stats::c::cs_mid_qb.load(Relaxed);
    l.compute_root.set_descriptor(1, out_skin.gpu_va).unwrap();
    l.set_compute_pipeline(cs_buf_only.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::clear_tex_cs_keep.load(Relaxed) - keep0, 1, "buffer-only SM 6.6 CS must keep clear_tex_cs");
    assert!(l.bar.clear_tex_cs.contains(&rt_simul_id));
    assert_eq!(l.bar.enc_sync_after, MTLStages::Dispatch);

    // Mid-encoder transition `pre_vb: STATE_VB_CB -> STATE_UAV` (`qa = Vertex | Dispatch`) in `L4#4`:
    // `track_exact_clear` + `StageSync` keep `cs_mid_qb` at 0!
    l.barrier_list_with(&[Barrier::Transition(pre_vb_id, STATE_VB_CB, STATE_UAV)], true, false);
    l.compute_root.set_descriptor(1, pre_vb.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed), mid0, "transitively satisfied Vertex|Dispatch must NOT emit cs_mid_qb");

    // Guardrail 1: bind texture-reading `cs_tex_reader` (`touches_textures = true`) reading `rt_simul` (slot 1):
    // MUST flush `rt_simul` from `clear_tex_cs` and upgrade `L4#4` to wait on `Fragment`!
    l.compute_root.set_table(0, heap.gpu_base + 24).unwrap();
    l.compute_root.set_descriptor(1, out_simul.gpu_va).unwrap();
    l.set_compute_pipeline(cs_tex_reader);
    l.dispatch(1, 1, 1).unwrap();
    assert!(!l.bar.clear_tex_cs.contains(&rt_simul_id), "texture-reading CS binding rt_simul must consume clear_tex_cs");
    assert!(l.bar.enc_sync_after.contains(MTLStages::Fragment), "true texture consumer must emit Fragment -> Dispatch");

    // Close `L4#4` (`stage_gen[Dispatch]` advances, `stage_sync[Dispatch]` now dominates `Fragment`!).
    // Transition `out_skin` `UAV -> PIXEL_SRV` + `out_simul` `UAV -> NON_PIXEL_SRV`, then open Draw Pass 1:
    // `prune_dominated_after` prunes dominated `Fragment` when `Dispatch` already waited on `stage_gen[Fragment]`,
    // AND a subsequent kept mid-pass transition of `pre_vb` (`UAV -> PIXEL_SRV`, produced in `L4#4`) MUST skip `rb_mid_qb`!
    l.barrier_list_with(&[Barrier::Transition(out_skin_id, STATE_UAV, STATE_PIXEL_SRV)], true, false);
    l.gfx_root.set_root_signature(gfx_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
    l.set_render_targets(Vec::new(), Some(TargetView::dsv(&ds_pass1, None).unwrap()));
    l.gfx.topology = 4;
    l.set_graphics_pipeline(gfx_pso.clone());
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.25, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();

    let rb_mid0 = stats::c::rb_mid_qb.load(Relaxed);
    let skip1 = stats::c::stagesync_skip.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(pre_vb_id, STATE_UAV, STATE_PIXEL_SRV)], true, false);
    assert!(l.gfx.draw_pass, "unattached buffer UAV -> PIXEL_SRV must keep draw pass open");
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::rb_mid_qb.load(Relaxed), rb_mid0, "already-synced Dispatch producer must skip rb_mid_qb");
    assert!(stats::c::stagesync_skip.load(Relaxed) > skip1, "expected stagesync_skip on redundant rb_mid_qb");

    // Guardrail 2: switch to Pass 2 (`ds_pass2`) WITHOUT transitioning `ds_pass1` yet (`Pass 1` closes and increments
    // `stage_gen[Fragment]`, while `Pass 2` opens with `0` barriers so `Pass 2` has NOT waited on `Pass 1`'s `Fragment`!).
    // Now transition `ds_pass1: DEPTH_WRITE -> PIXEL_SRV` mid-pass in `Pass 2`: `rb_mid_qb` MUST fire!
    l.set_render_targets(Vec::new(), Some(TargetView::dsv(&ds_pass2, None).unwrap()));
    l.draw(3, 1, 0, 0).unwrap();
    let rb_mid1 = stats::c::rb_mid_qb.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(ds1_id, STATE_DEPTH_WRITE, STATE_PIXEL_SRV)], true, false);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(stats::c::rb_mid_qb.load(Relaxed) > rb_mid1, "un-waited prior render pass Fragment write MUST emit rb_mid_qb");

    // Post-draw (`in_list_gfx_non_clear == true`) 2-hop transitive `StageSync` (`Fragment -> Blit -> Dispatch`):
    let ds2_id = &ds_pass2 as *const Resource as usize;
    let out_buf0_id = &out_buf0 as *const Resource as usize;
    l.end_encoder();
    l.barrier_list_with(&[Barrier::Transition(ds2_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE)], true, false);
    l.copy_buffer(&out_buf0, 0, &pre_vb, 0, 16).unwrap();
    l.end_encoder();

    l.barrier_list_with(&[Barrier::Transition(out_buf0_id, STATE_COPY_DEST, STATE_NON_PIXEL_SRV)], true, true);
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, pre_vb.gpu_va).unwrap();
    l.set_compute_pipeline(cs_buf_only);
    l.dispatch(1, 1, 1).unwrap(); // opens `C_post2` with `enc_sync_after == Blit` ONLY!
    assert_eq!(l.bar.enc_sync_after, MTLStages::Blit);

    let cs_mid2 = stats::c::cs_mid_qb.load(Relaxed);
    let skip2 = stats::c::stagesync_skip.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(ds2_id, STATE_DEPTH_WRITE, STATE_NON_PIXEL_SRV)], true, false);
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::cs_mid_qb.load(Relaxed), cs_mid2, "2-hop transitive Fragment->Blit->Dispatch must skip cs_mid_qb");
    assert!(stats::c::stagesync_skip.load(Relaxed) > skip2, "expected stagesync_skip on 2-hop transitive cs_mid_qb");

    l.barrier_list_with(
        &[
            Barrier::Transition(out_skin_id, STATE_PIXEL_SRV, STATE_COPY_SOURCE),
            Barrier::Transition(out_simul_id, STATE_UAV, STATE_COPY_SOURCE),
            Barrier::Transition(ds1_id, STATE_PIXEL_SRV, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_buffer(&rb_skin, 0, &out_skin, 0, 64).unwrap();
    l.copy_buffer(&rb_simul, 0, &out_simul, 0, 64).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_d1, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds_pass1, 0), None).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    let v_skin: Vec<f32> = buf_words(&rb_skin, 4).into_iter().map(f32::from_bits).collect();
    let v_simul: Vec<f32> = buf_words(&rb_simul, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&v_skin[..], &[8.0, 16.0, 24.0, 1.0]);
    assert_eq!(&v_simul[..], &[7.0, 14.0, 21.0, 1.0]);
    let d1 = f32::from_bits(buf_words(&rb_d1, 1)[0]);
    assert!((d1 - 0.25).abs() < 1e-5, "ds_pass1 depth mismatch: got {d1}");
}

#[test]
fn shader_pso_opt_v2_narrows_sm66_uav_and_preraster_reads_and_specializes_pso_blend_on_gpu() {
    use crate::mesh::{MeshDesc, MeshPipeline};
    use crate::metal::pipeline::{
        msc_resource_usages, shader_writes_uav, stage_reads_resources_with, GraphicsDesc,
        GraphicsPipeline, READS_BUF, READS_TEX,
    };
    use crate::state::{blend, blend_state, BlendDesc, DepthStencilState, RasterState, RenderTargetBlendDesc};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };
    use dxil_metal::{CompileOptions, CompiledShader, ShaderReflection, ShaderStage, StageInfo};
    use objc2_metal::{MTL4BlendState, MTL4RenderPipelineColorAttachmentDescriptor, MTLColorWriteMask};
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }

    // 1) Unit-verify MSC `"ResourceUsages"` parsing + `shader_writes_uav` + `stage_reads_resources_with`.
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let as_dxil = sign_dxil(include_bytes!("../../../../tests/fixtures/mesh.as.dxil"));
    let ms_dxil = sign_dxil(include_bytes!("../../../../tests/fixtures/mesh.ms.dxil"));
    let ps_dxil = sign_dxil(include_bytes!("../../../../tests/fixtures/mesh.ps.dxil"));
    let make_fake_sh = |json: &str| CompiledShader {
        metallib: Vec::new(),
        function_name: "main".into(),
        stage: ShaderStage::Vertex,
        reflection: ShaderReflection {
            stage: ShaderStage::Vertex,
            function_name: "main".into(),
            resources: Vec::new(),
            needs_function_constants: false,
            function_constants: Vec::new(),
            info: StageInfo::Vertex {
                instance_id_index: -1,
                vertex_id_index: -1,
                vertex_output_size_in_bytes: 16,
                needs_draw_params: false,
                inputs: Vec::new(),
            },
            json: json.into(),
        },
        key: 0,
    };
    let sh_buf_srv = make_fake_sh(r#"{"ShaderType":"Vertex","ResourceUsages":2}"#);
    let sh_buf_uav_wo = make_fake_sh(r#"{"ShaderType":"Vertex","ResourceUsages":4}"#);
    let sh_tex_srv = make_fake_sh(r#"{"ShaderType":"Pixel","ResourceUsages":10}"#);
    let sh_tex_uav = make_fake_sh(r#"{"ShaderType":"Pixel","ResourceUsages":16}"#);
    let sh_rayquery = make_fake_sh(r#"{"ShaderType":"Vertex","ResourceUsages":32}"#);
    let sh_legacy = make_fake_sh(r#"{"ShaderType":"Vertex"}"#);

    assert_eq!(msc_resource_usages(&sh_buf_srv), Some(2));
    assert_eq!(msc_resource_usages(&sh_legacy), None);
    assert!(!shader_writes_uav(&sh_buf_srv, &vs_dxil, true));
    assert!(!shader_writes_uav(&sh_tex_srv, &vs_dxil, true));
    assert!(shader_writes_uav(&sh_buf_uav_wo, &vs_dxil, true));
    assert!(shader_writes_uav(&sh_tex_uav, &vs_dxil, true));
    assert_eq!(stage_reads_resources_with(&sh_buf_srv, &vs_dxil, true), READS_BUF, "SM 6.6 buffer-only VS must drop READS_TEX");
    assert_eq!(stage_reads_resources_with(&sh_buf_uav_wo, &vs_dxil, true), READS_BUF, "write-only UAV buffer in VS must retain READS_BUF");
    assert_eq!(stage_reads_resources_with(&sh_rayquery, &vs_dxil, true), READS_BUF, "AccelerationStructure rayquery in VS must retain READS_BUF");
    assert_eq!(stage_reads_resources_with(&sh_tex_srv, &vs_dxil, true), READS_BUF | READS_TEX);

    // 2) Unit-verify `ColorBlend::apply_to_with_opt` disables dead blend preambles while keeping real blends enabled.
    let mut bd = BlendDesc { IndependentBlendEnable: 1, ..BlendDesc::default() };
    // MRT0: `BlendEnable = 1`, `1 * Src + 0 * Dst` (`One/Zero/Add`), `WriteMask = 0xF` -> effective blend no-op!
    bd.RenderTarget[0] = RenderTargetBlendDesc {
        BlendEnable: 1,
        SrcBlend: blend::ONE,
        DestBlend: blend::ZERO,
        BlendOp: 1, // ADD
        SrcBlendAlpha: blend::ONE,
        DestBlendAlpha: blend::ZERO,
        BlendOpAlpha: 1, // ADD
        RenderTargetWriteMask: 0xF,
        ..Default::default()
    };
    // MRT1: `BlendEnable = 1`, `SrcAlpha / InvSrcAlpha`, `WriteMask = 0` -> masked-out slot!
    bd.RenderTarget[1] = RenderTargetBlendDesc {
        BlendEnable: 1,
        SrcBlend: blend::SRC_ALPHA,
        DestBlend: blend::INV_SRC_ALPHA,
        BlendOp: 1,
        SrcBlendAlpha: blend::ONE,
        DestBlendAlpha: blend::ZERO,
        BlendOpAlpha: 1,
        RenderTargetWriteMask: 0,
        ..Default::default()
    };
    // MRT2: `BlendEnable = 1`, `SrcAlpha / InvSrcAlpha`, `WriteMask = 0xF` -> genuine blend!
    bd.RenderTarget[2] = RenderTargetBlendDesc {
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
    let bs = blend_state(&bd);
    let ca = MTL4RenderPipelineColorAttachmentDescriptor::new();
    bs.attachments[0].apply_to_with_opt(&ca, false);
    assert_eq!(ca.blendingState(), MTL4BlendState::Enabled);
    bs.attachments[0].apply_to_with_opt(&ca, true);
    assert_eq!(ca.blendingState(), MTL4BlendState::Disabled, "One/Zero/Add blend must disable MTL4BlendState under opt_v2");
    bs.attachments[1].apply_to_with_opt(&ca, true);
    assert_eq!(ca.blendingState(), MTL4BlendState::Disabled, "WriteMask=0 MRT slot must disable MTL4BlendState under opt_v2");
    assert_eq!(ca.writeMask(), MTLColorWriteMask::None, "WriteMask=0 must remain None so attachment is untouched");
    bs.attachments[2].apply_to_with_opt(&ca, true);
    assert_eq!(ca.blendingState(), MTL4BlendState::Enabled, "non-trivial alpha blend must stay Enabled");

    // 3) Real-GPU Metal 4 execution of BOTH `GraphicsPipeline::new_with_pso_opts(..., true)` AND
    //    `MeshPipeline::new_with_all_opts(..., true)` with MRT0 (`One/Zero/Add`) and MRT1 (`WriteMask = 0`)
    //    + mid-pass `Barrier::Uav(0)` keeping draw/mesh passes open (`draw_pass_no_uav`++)!
    let _ = CompileOptions::default();
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
    let mesh_rs = RootSignature::from_blob(&as_dxil).unwrap();

    let mut mrt_bd = BlendDesc { IndependentBlendEnable: 1, ..BlendDesc::default() };
    mrt_bd.RenderTarget[0] = bd.RenderTarget[0];
    mrt_bd.RenderTarget[1] = bd.RenderTarget[1];
    let gdesc = GraphicsDesc {
        vs: &vs_dxil,
        ps: None,
        blend: mrt_bd,
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
        rtv_formats: vec![RGBA8_UNORM, RGBA8_UNORM],
        dsv_format: D32_S8,
        sample_count: 1,
    };
    let pso_std = GraphicsPipeline::new_with_pso_opts(rs.clone(), &gdesc, "g35_pso_std", true, true, false).unwrap();
    let pso_v2_a = Arc::new(GraphicsPipeline::new_with_pso_opts(rs.clone(), &gdesc, "g35_pso_v2", true, true, true).unwrap());
    let pso_v2_b = GraphicsPipeline::new_with_pso_opts(rs.clone(), &gdesc, "g35_pso_v2_hit", true, true, true).unwrap();
    assert_ne!(
        &*pso_std.pso as *const _ as usize,
        &*pso_v2_a.pso as *const _ as usize,
        "SHADER_PSO_OPT_V2_SALT must isolate specialized GraphicsPipeline from standard PSO"
    );
    assert_eq!(
        &*pso_v2_a.pso as *const _ as usize,
        &*pso_v2_b.pso as *const _ as usize,
        "repeat GraphicsPipeline::new_with_pso_opts(..., true) must hit pso_cache"
    );
    assert!(!pso_v2_a.writes_uav, "entry_v2 VS must be proven !writes_uav");

    let mdesc = MeshDesc {
        amplification: Some(&as_dxil),
        mesh: &ms_dxil,
        pixel: Some(&ps_dxil),
        blend: mrt_bd,
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
        depth_stencil: DepthStencilState::default(),
        rtv_formats: vec![RGBA8_UNORM, RGBA8_UNORM],
        dsv_format: 0,
        sample_count: 1,
    };
    let mesh_std = MeshPipeline::new_with_all_opts(mesh_rs.clone(), &mdesc, "g35_mesh_std", true, false).unwrap();
    let mesh_v2_a = Arc::new(MeshPipeline::new_with_all_opts(mesh_rs.clone(), &mdesc, "g35_mesh_v2", true, true).unwrap());
    let mesh_v2_b = MeshPipeline::new_with_all_opts(mesh_rs.clone(), &mdesc, "g35_mesh_v2_hit", true, true).unwrap();
    assert_ne!(
        &*mesh_std.pso as *const _ as usize,
        &*mesh_v2_a.pso as *const _ as usize,
        "SHADER_PSO_OPT_V2_SALT must isolate specialized MeshPipeline from standard PSO"
    );
    assert_eq!(
        &*mesh_v2_a.pso as *const _ as usize,
        &*mesh_v2_b.pso as *const _ as usize,
        "repeat MeshPipeline::new_with_all_opts(..., true) must hit pso_cache"
    );
    assert!(!mesh_v2_a.writes_uav, "mesh AS+MS+PS must be proven !writes_uav");

    let q = Queue::new(ListType::Direct, "g35 stage D queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    let rt0 = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt1 = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let ds = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let rb0 = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rb1 = test_buffer(256 * 8, heap_type::READBACK, 0);
    rt0.register_ptr();
    rt1.register_ptr();
    ds.register_ptr();
    let rtv0 = TargetView::rtv(&rt0, None).unwrap();
    let rtv1 = TargetView::rtv(&rt1, None).unwrap();
    let dsv = TargetView::dsv(&ds, None).unwrap();

    let _stats_guard = gpu_stats_lock();
    let split0 = stats::c::app_barrier_split.load(Relaxed);
    let no_uav0 = stats::c::draw_pass_no_uav.load(Relaxed);

    l.begin(&alloc).unwrap();
    unsafe {
        // Clear MRT0 to magenta (`0xffff00ff`) and MRT1 (`WriteMask = 0`) to blue (`0xffff0000`).
        l.clear_rtv(&rtv0, [1.0, 0.0, 1.0, 1.0], &[]).unwrap();
        l.clear_rtv(&rtv1, [0.0, 0.0, 1.0, 1.0], &[]).unwrap();
        l.clear_dsv(&dsv, true, 0.9, false, 0, &[]).unwrap();
    }
    // Pass 1: GraphicsPipeline (`pso_v2_a`) + mid-pass `Barrier::Uav(0)` keeping draw pass open.
    l.gfx_root.set_root_signature(rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 0, 0, 0]).unwrap();
    l.set_render_targets(vec![rtv0.clone(), rtv1.clone()], Some(dsv));
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_v2_a);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.25, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(l.gfx.draw_pass, "non-UAV-writing GraphicsPipeline must keep draw pass open across Barrier::Uav(0)");
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::app_barrier_split.load(Relaxed), split0);

    // Pass 2: MeshPipeline (`mesh_v2_a`) writing MRT0 via `One/Zero/Add` (`MTL4BlendState::Disabled`)
    // while MRT1 (`WriteMask = 0`, `MTL4BlendState::Disabled`) preserves cleared blue, AND mid-pass
    // `Barrier::Uav(0)` keeps mesh pass open (`!mesh_v2_a.writes_uav`)!
    l.gfx_root.set_root_signature(mesh_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 128, 0, 0]).unwrap();
    l.set_render_targets(vec![rtv0, rtv1], None);
    l.set_mesh_pipeline(mesh_v2_a);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.0, zfar: 1.0 }]);
    l.dispatch_mesh(1, 1, 1).unwrap();
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(l.gfx.draw_pass, "non-UAV-writing MeshPipeline must keep draw pass open across Barrier::Uav(0)");
    l.dispatch_mesh(1, 1, 1).unwrap();
    assert_eq!(stats::c::app_barrier_split.load(Relaxed), split0);

    let rt0_id = &rt0 as *const Resource as usize;
    let rt1_id = &rt1 as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(rt0_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(rt1_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    assert!(stats::c::draw_pass_no_uav.load(Relaxed) >= no_uav0 + 2, "both UAV-free draw/mesh passes must increment draw_pass_no_uav");

    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb0, fp), [0, 0, 0], CopyLoc::Subresource(&rt0, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb1, fp), [0, 0, 0], CopyLoc::Subresource(&rt1, 0), None).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    // Top-left pixel (0, 0) of `mesh.hlsl` is quad 0 = red (`0xff0000ff`), replacing cleared magenta (`0xffff00ff`)!
    // MRT1 (`WriteMask = 0`) must stay cleared blue (`0xffff0000`)!
    let px0 = buf_words(&rb0, 1)[0];
    let px1 = buf_words(&rb1, 1)[0];
    assert_eq!(px0, 0xff00_00ff, "MRT0 One/Zero/Add overwrite must render quad 0 red");
    assert_eq!(px1, 0xffff_0000, "MRT1 WriteMask=0 with disabled blend must preserve cleared blue");
}

#[test]
fn msc_compute_access_unlocks_exact_slots_subrange_and_class_skips_while_ordering_hazards_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputePipeline, READS_BUF, READS_TEX};
    use crate::metal::views;
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion,
        StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    const RGBA32_FLOAT: u32 = 2;

    // Bind `static_samplers.cs.dxil` under an UNBOUNDED root signature (modeling Northlight's
    // `fillLightBufferDirectLightCS`: unbounded SRV table `param 0` + unbounded UAV table `param 1`).
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
                    num_descriptors: rs_flags::UNBOUNDED,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                    range_type: DescriptorRangeType::Uav,
                    num_descriptors: rs_flags::UNBOUNDED,
                    base_shader_register: 0,
                    register_space: 0,
                    flags: 0,
                    offset_in_descriptors_from_table_start: 0,
                }]),
                visibility: 0,
            },
        ],
        static_samplers: vec![clamp_point(0), clamp_point(1)],
        flags: 0,
    })
    .unwrap();

    let base = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g36_stage_a_unbounded").unwrap();
    // Model unbounded HLSL resource arrays (`RWTexture2D[]` / `StructuredBuffer[]` in PSV0 ->
    // `ComputeAccess::opaque()`), refined via MSC's live `"UsedResources"` + `"ResourceUsages"` JSON!
    let acc_on = ComputeAccess::refine_with_msc(
        ComputeAccess::opaque(),
        &cs_dxil,
        &cs_rs,
        &base.shader.reflection.json,
        crate::metal::pipeline::msc_resource_usages(&base.shader),
    );
    assert!(acc_on.msc_refined, "MSC UsedResources must refine unbounded PSV0 arrays to exact slots");
    assert!(acc_on.uav_slots.is_some());
    assert!(acc_on.read_slots.is_some());
    let pso_msc = Arc::new(ComputePipeline {
        pso: base.pso.clone(),
        rs: base.rs.clone(),
        threadgroup: base.threadgroup,
        shader: base.shader.clone(),
        hk_profile: base.hk_profile,
        access: acc_on,
    });

    // Also build SM 6.6 class-typed profiles (`uav_slots = None`, `read_slots = None`) to verify
    // `ib_class_skip`, `!touches_textures` `clear_tex_cs_keep` (with `opt_stagesync_barrier_v2 = false`),
    // and WAR class-hazard preservation!
    let pso_sm66_buf_rw = Arc::new(ComputePipeline {
        pso: base.pso.clone(),
        rs: base.rs.clone(),
        threadgroup: base.threadgroup,
        shader: base.shader.clone(),
        hk_profile: base.hk_profile,
        access: ComputeAccess {
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
        },
    });
    let pso_sm66_tex_rw = Arc::new(ComputePipeline {
        pso: base.pso.clone(),
        rs: base.rs.clone(),
        threadgroup: base.threadgroup,
        shader: base.shader.clone(),
        hk_profile: base.hk_profile,
        access: ComputeAccess {
            uav_slots: None,
            read_slots: None,
            bounded_read_slots: Vec::new(),
            unbounded_srv_classes: READS_TEX,
            uav_classes: READS_TEX,
            read_classes: READS_TEX,
            uav_srv_classes: READS_TEX,
            heap_indexed_or_unknown: true,
            touches_textures: true,
            msc_refined: false,
            msc_exact_classes: READS_TEX,
        },
    });
    let pso_sm66_buf_ro = Arc::new(ComputePipeline {
        pso: base.pso,
        rs: base.rs,
        threadgroup: base.threadgroup,
        shader: base.shader,
        hk_profile: base.hk_profile,
        access: ComputeAccess {
            uav_slots: None,
            read_slots: None,
            bounded_read_slots: Vec::new(),
            unbounded_srv_classes: READS_BUF,
            uav_classes: 0,
            read_classes: READS_BUF,
            uav_srv_classes: READS_BUF,
            heap_indexed_or_unknown: true,
            touches_textures: false,
            msc_refined: false,
            msc_exact_classes: READS_BUF,
        },
    });

    let q = Queue::new(ListType::Direct, "g36 stage A queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_msc_compute_access = true;
    l.opts.opt_stagesync_barrier_v2 = false; // Prove `clear_tex_cs_keep` works standalone under Stage A!
    l.opts.opt_ib_tight = true;
    l.opts.opt_ib_exact_obl = true;
    l.opts.opt_subrange_uav_barrier = true;

    let tex_a = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_b = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_c = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_unrel = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_clear = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET); // non-UAV -> CLEAR render pass!
    let shared_buf = test_buffer(2048, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let scratch_buf = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_buf = test_buffer(2048, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &tex_c, &tex_unrel, &rt_clear, &shared_buf, &scratch_buf, &rb_buf] {
        r.register_ptr();
    }

    let make_struct_uav = |first_elem: u64, num_elems: u32, stride: u32| -> [u8; 40] {
        let mut d = [0u8; 40];
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[8..16].copy_from_slice(&first_elem.to_le_bytes());
        d[16..20].copy_from_slice(&num_elems.to_le_bytes());
        d[20..24].copy_from_slice(&stride.to_le_bytes());
        d
    };

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g36 stage A heap").unwrap();
    let write_entry = |slot: usize, entry: &[u8; 24]| unsafe {
        std::ptr::copy_nonoverlapping(entry.as_ptr(), (heap.cpu_base + slot * 24) as *mut u8, 24);
    };
    write_entry(0, &views::srv_entry(Some(&tex_a), None).unwrap());
    write_entry(1, &views::srv_entry(Some(&tex_b), None).unwrap());
    write_entry(2, &views::srv_entry(Some(&tex_c), None).unwrap());
    write_entry(3, &views::uav_entry(Some(&shared_buf), Some(&make_struct_uav(0, 32, 16))).unwrap());
    write_entry(4, &views::uav_entry(Some(&shared_buf), Some(&make_struct_uav(32, 32, 16))).unwrap());
    write_entry(5, &views::uav_entry(Some(&shared_buf), Some(&make_struct_uav(16, 32, 16))).unwrap());
    write_entry(6, &views::uav_entry(Some(&scratch_buf), Some(&make_struct_uav(0, 32, 16))).unwrap());

    let shared_id = &shared_buf as *const Resource as usize;
    let tex_unrel_id = &tex_unrel as *const Resource as usize;
    let rt_clear_id = &rt_clear as *const Resource as usize;

    let _stats_guard = gpu_stats_lock();
    l.opts.opt_ib_substage_narrow = false;
    l.begin(&alloc).unwrap();
    l.set_descriptor_heaps(heap.gpu_base, 0);

    unsafe {
        l.clear_rtv(&TargetView::rtv(&tex_a, None).unwrap(), [11.0, 22.0, 33.0, 1.0], &[]).unwrap();
        l.clear_rtv(&TargetView::rtv(&tex_b, None).unwrap(), [44.0, 55.0, 66.0, 1.0], &[]).unwrap();
        l.clear_rtv(&TargetView::rtv(&tex_c, None).unwrap(), [77.0, 88.0, 99.0, 1.0], &[]).unwrap();
    }
    l.barrier_list_with(
        &[
            Barrier::Transition(&tex_a as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
            Barrier::Transition(&tex_b as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
            Barrier::Transition(&tex_c as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
        ],
        true,
        true,
    );

    l.compute_root.set_root_signature(cs_rs).unwrap();

    // 1) Unbounded-RS `pso_msc` dispatches writing disjoint `[0, 512)` and `[512, 1024)` sub-allocations:
    //    MSC refinement unlocks `subrange_uav_skip` (`enc_barrier` delta == 0, `msc_access_exact >= 2`)!
    let msc_ex0 = stats::c::msc_access_exact.load(Relaxed);
    let sub_skip0 = stats::c::subrange_uav_skip.load(Relaxed);
    l.set_compute_pipeline(pso_msc.clone());
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_table(1, heap.gpu_base + 3 * 24).unwrap();
    l.dispatch(1, 1, 1).unwrap();

    let enc_b0 = stats::c::enc_barrier.load(Relaxed);
    l.barrier_list_with(&[Barrier::Uav(shared_id)], true, false);
    l.compute_root.set_table(0, heap.gpu_base + 24).unwrap();
    l.compute_root.set_table(1, heap.gpu_base + 4 * 24).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed), enc_b0, "MSC-refined unbounded UAV table must skip disjoint subrange barrier");
    assert!(stats::c::subrange_uav_skip.load(Relaxed) > sub_skip0);
    assert!(stats::c::msc_access_exact.load(Relaxed) >= msc_ex0 + 2);

    // Guardrail A: overlapping `[256, 768)` UAV write MUST still fire `enc_barrier`!
    l.barrier_list_with(&[Barrier::Uav(shared_id)], true, false);
    let enc_b1 = stats::c::enc_barrier.load(Relaxed);
    l.compute_root.set_table(0, heap.gpu_base + 2 * 24).unwrap();
    l.compute_root.set_table(1, heap.gpu_base + 5 * 24).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::enc_barrier.load(Relaxed) > enc_b1, "overlapping subrange UAV write MUST emit enc_barrier");

    // 2) Close encoder so `ib` resets and `stage_gen[STAGE_DISPATCH] >= 1` anchors pre-list overlap;
    //    flush `rt_clear` via `CLEAR` render pass (`Barrier::Uav(0)`) -> `RTV -> NON_PIXEL_SRV` into `clear_tex_cs`,
    //    then verify SM 6.6 buffer-only `pso_sm66_buf_rw` retains `clear_tex_cs` (`clear_tex_cs_keep`++)
    //    AND skips texture barriers via `ib_class_skip`!
    l.end_encoder();
    unsafe {
        l.clear_rtv(&TargetView::rtv(&rt_clear, None).unwrap(), [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
    }
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    l.barrier_list_with(&[Barrier::Transition(rt_clear_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, false);
    assert!(l.bar.clear_tex_cs.contains(&rt_clear_id));
    let keep0 = stats::c::clear_tex_cs_keep.load(Relaxed);
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_table(1, heap.gpu_base + 6 * 24).unwrap();
    l.set_compute_pipeline(pso_sm66_buf_rw.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::clear_tex_cs_keep.load(Relaxed) > keep0, "Stage A must retain clear_tex_cs on !touches_textures without stagesync_v2");
    assert!(l.bar.clear_tex_cs.contains(&rt_clear_id));

    // Producer-side class filter: non-SIMUL texture `Transition(UAV -> NON_PIXEL_SRV)` and `Barrier::Uav(tex_unrel)`
    // after buffer-only dispatch MUST be skipped (`ib_class_skip >= 2`, `ib_obl_tight` stays empty)!
    let cls0 = stats::c::ib_class_skip.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(tex_unrel_id, STATE_UAV, STATE_NON_PIXEL_SRV)], true, true);
    l.barrier_list_with(&[Barrier::Uav(tex_unrel_id)], true, false);
    assert!(stats::c::ib_class_skip.load(Relaxed) >= cls0 + 2);
    assert!(l.ib.ib_obl_tight.is_empty(), "texture barriers after buffer-only CS must add 0 obligations");

    // Consumer-side `Barrier::Uav(0)` class-disjoint skip: buffer-only producer -> `Barrier::Uav(0)` -> texture-only consumer!
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert_eq!(l.ib.ib_obl_tight.len(), 1);
    let enc_b2 = stats::c::enc_barrier.load(Relaxed);
    let cls1 = stats::c::ib_class_skip.load(Relaxed);
    l.set_compute_pipeline(pso_sm66_tex_rw);
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed), enc_b2, "class-disjoint Uav(0) (BUF -> TEX) must skip enc_barrier");
    assert!(stats::c::ib_class_skip.load(Relaxed) > cls1);

    // Guardrail B (WAR hazard): buffer-read-only producer (`uav_classes = 0, read_classes = READS_BUF`)
    // -> `Barrier::Uav(0)` -> buffer-writing consumer (`uav_classes = READS_BUF`) MUST fire `enc_barrier`!
    l.end_encoder();
    l.set_compute_pipeline(pso_sm66_buf_ro);
    l.dispatch(1, 1, 1).unwrap();
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    let enc_b3 = stats::c::enc_barrier.load(Relaxed);
    l.set_compute_pipeline(pso_sm66_buf_rw);
    l.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::enc_barrier.load(Relaxed) > enc_b3, "buffer read -> Uav(0) -> buffer write WAR hazard MUST emit enc_barrier");

    l.barrier_list_with(&[Barrier::Transition(shared_id, STATE_UAV, STATE_COPY_SOURCE)], true, true);
    l.copy_buffer(&rb_buf, 0, &shared_buf, 0, 1024).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    let floats: Vec<f32> = buf_words(&rb_buf, 256).into_iter().map(f32::from_bits).collect();
    for elem in 0..64 {
        let got = &floats[elem * 4..elem * 4 + 4];
        let want: &[f32; 4] = if elem < 16 {
            &[11.0, 22.0, 33.0, 1.0]
        } else if elem < 48 {
            &[77.0, 88.0, 99.0, 1.0]
        } else {
            &[44.0, 55.0, 66.0, 1.0]
        };
        assert_eq!(got, want, "shared_buf float4[{elem}] mismatch");
    }
}

#[test]
fn clear_cs_exact_sync_overlaps_dsv_clear_passes_and_guards_in_list_conflicts_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputePipeline, GraphicsDesc, GraphicsPipeline};
    use crate::metal::views;
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
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g36_stage_b_cs").unwrap());

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
            "g36_stage_b_gfx",
        )
        .unwrap(),
    );

    let q = Queue::new(ListType::Direct, "g36 stage B queue").unwrap();
    let a_prev = CommandAllocator::new(ListType::Direct).unwrap();
    let a_l2 = CommandAllocator::new(ListType::Direct).unwrap();
    let a_nodisp = CommandAllocator::new(ListType::Direct).unwrap();

    let tex_srv = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_post = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_nodisp = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let ds0 = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let ds1 = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let uav_buf = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_a = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_b = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_d0 = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rb_d1 = test_buffer(256 * 8, heap_type::READBACK, 0);
    for r in [&tex_srv, &rt_a, &rt_b, &rt_post, &rt_nodisp, &ds0, &ds1, &uav_buf] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g36 stage B heap").unwrap();
    unsafe {
        let e0 = views::srv_entry(Some(&tex_srv), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }

    // Prior command list `l_prev` populates `tex_srv` and pre-fills `rt_a` & `rt_b` to red (`[1,0,0,1]`).
    let mut l_prev = CommandList::new(ListType::Direct).unwrap();
    l_prev.begin(&a_prev).unwrap();
    unsafe {
        l_prev.clear_rtv(&TargetView::rtv(&tex_srv, None).unwrap(), [1.0, 2.0, 3.0, 1.0], &[]).unwrap();
        l_prev.clear_rtv(&TargetView::rtv(&rt_a, None).unwrap(), [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
        l_prev.clear_rtv(&TargetView::rtv(&rt_b, None).unwrap(), [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
    }
    l_prev.barrier_list_with(
        &[
            Barrier::Transition(&tex_srv as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
            Barrier::Transition(&rt_a as *const Resource as usize, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
            Barrier::Transition(&rt_b as *const Resource as usize, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
        ],
        true,
        true,
    );
    l_prev.close().unwrap();

    // Guardrail A: list with NO prior `Dispatch` (`stage_gen[STAGE_DISPATCH] == 0`) MUST NOT take `clear_cs_exact_sync`!
    let _stats_guard = gpu_stats_lock();
    let ex0 = stats::c::clear_cs_exact_sync.load(Relaxed);
    let mut l_nodisp = CommandList::new(ListType::Direct).unwrap();
    l_nodisp.opts.opt_clear_cs_exact_sync = true;
    l_nodisp.begin(&a_nodisp).unwrap();
    unsafe {
        l_nodisp.clear_dsv(&TargetView::dsv(&ds0, None).unwrap(), true, 0.5, false, 0, &[]).unwrap();
    }
    l_nodisp.barrier_list_with(&[Barrier::Transition(&ds0 as *const Resource as usize, STATE_DEPTH_WRITE, STATE_DEPTH_READ)], true, false);
    assert_eq!(l_nodisp.bar.stage_gen[STAGE_DISPATCH], 0);
    unsafe {
        l_nodisp.clear_rtv(&TargetView::rtv(&rt_nodisp, None).unwrap(), [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
    }
    l_nodisp.barrier_list_with(&[Barrier::Transition(&rt_nodisp as *const Resource as usize, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    assert_eq!(stats::c::clear_cs_exact_sync.load(Relaxed), ex0, "must NOT fire clear_cs_exact_sync when stage_gen[STAGE_DISPATCH] == 0");
    assert!(l_nodisp.bar.enc_sync_after.contains(MTLStages::Fragment));
    l_nodisp.close().unwrap();

    // Main `L2` pattern: `L2#0` `Dispatch` (`All -> All`) -> `L2#1..L2#2` DSV `clear_pass`es -> `L2#4` `compute_clear_batch`!
    let mut l2 = CommandList::new(ListType::Direct).unwrap();
    l2.opts.opt_clear_cs_exact_sync = true;
    l2.begin(&a_l2).unwrap();
    l2.set_descriptor_heaps(heap.gpu_base, 0);
    l2.compute_root.set_root_signature(cs_rs).unwrap();
    l2.compute_root.set_table(0, heap.gpu_base).unwrap();
    l2.compute_root.set_descriptor(1, uav_buf.gpu_va).unwrap();
    l2.set_compute_pipeline(cs_pso);
    l2.dispatch(1, 1, 1).unwrap(); // `L2#0` anchors `pre_list_all` on `STAGE_DISPATCH`!

    let ds0_id = &ds0 as *const Resource as usize;
    let ds1_id = &ds1 as *const Resource as usize;
    let rt_a_id = &rt_a as *const Resource as usize;
    let rt_b_id = &rt_b as *const Resource as usize;
    let rt_post_id = &rt_post as *const Resource as usize;

    // `L2#1` & `L2#2`: standalone DSV hardware `clear_pass`es (`Fragment` attachment clears).
    unsafe {
        l2.clear_dsv(&TargetView::dsv(&ds0, None).unwrap(), true, 0.25, false, 0, &[]).unwrap();
    }
    l2.barrier_list_with(&[Barrier::Transition(ds0_id, STATE_DEPTH_WRITE, STATE_DEPTH_READ)], true, false);
    unsafe {
        l2.clear_dsv(&TargetView::dsv(&ds1, None).unwrap(), true, 0.75, false, 0, &[]).unwrap();
    }
    l2.barrier_list_with(&[Barrier::Transition(ds1_id, STATE_DEPTH_WRITE, STATE_DEPTH_READ)], true, false);
    assert!(l2.bar.stage_sync[STAGE_DISPATCH].pre_list_all);

    // Transition `rt_a` & `rt_b` `PIXEL_SRV -> RENDER_TARGET`, then `ClearRenderTargetView` x2 (`L2#4`):
    // MUST open `L2#4` with `enc_sync_after == MTLStages::Dispatch` (NOT `Fragment`!), overlapping `ds0`/`ds1` clears!
    l2.barrier_list_with(
        &[
            Barrier::Transition(rt_a_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET),
            Barrier::Transition(rt_b_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET),
        ],
        true,
        false,
    );
    let ex1 = stats::c::clear_cs_exact_sync.load(Relaxed);
    unsafe {
        l2.clear_rtv(&TargetView::rtv(&rt_a, None).unwrap(), [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
        l2.clear_rtv(&TargetView::rtv(&rt_b, None).unwrap(), [0.0, 0.0, 1.0, 1.0], &[]).unwrap();
    }
    l2.barrier_list_with(
        &[
            Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(rt_b_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
        ],
        true,
        false,
    );
    assert_eq!(stats::c::clear_cs_exact_sync.load(Relaxed) - ex1, 1, "L2#4 compute_clear_batch must fire clear_cs_exact_sync");
    assert_eq!(
        l2.bar.enc_sync_after,
        MTLStages::Dispatch,
        "L2#4 compute_clear_batch must wait ONLY on Dispatch (not in-list DSV Fragment clears)"
    );
    assert!(l2.bar.enc_cur_sync[STAGE_DISPATCH].pre_list_all);

    // Guardrail B: once an in-list draw pass runs (`in_list_gfx_non_clear == true`), a subsequent
    // `compute_clear_batch` MUST fall back to `Fragment -> Dispatch`!
    l2.gfx_root.set_root_signature(gfx_rs).unwrap();
    l2.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
    let dsv0_rw = TargetView::dsv(&ds0, None).unwrap();
    l2.barrier_list_with(&[Barrier::Transition(ds0_id, STATE_DEPTH_READ, STATE_DEPTH_WRITE)], true, false);
    l2.set_render_targets(Vec::new(), Some(dsv0_rw));
    l2.gfx.topology = 4;
    l2.set_graphics_pipeline(gfx_pso);
    l2.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.25, zfar: 1.0 }]);
    l2.draw(3, 1, 0, 0).unwrap();
    l2.end_encoder();
    assert!(l2.bar.in_list_gfx_non_clear);

    let ex2 = stats::c::clear_cs_exact_sync.load(Relaxed);
    unsafe {
        l2.clear_rtv(&TargetView::rtv(&rt_post, None).unwrap(), [1.0, 1.0, 0.0, 1.0], &[]).unwrap();
    }
    l2.barrier_list_with(&[Barrier::Transition(rt_post_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    assert_eq!(stats::c::clear_cs_exact_sync.load(Relaxed), ex2, "post-draw compute_clear_batch must NOT skip Fragment sync");
    assert!(l2.bar.enc_sync_after.contains(MTLStages::Fragment));

    l2.barrier_list_with(
        &[
            Barrier::Transition(ds0_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE),
            Barrier::Transition(ds1_id, STATE_DEPTH_READ, STATE_COPY_SOURCE),
        ],
        true,
        false,
    );
    let fp_rt = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
    l2.copy_texture_region(CopyLoc::Footprint(&rb_a, fp_rt), [0, 0, 0], CopyLoc::Subresource(&rt_a, 0), None).unwrap();
    l2.copy_texture_region(CopyLoc::Footprint(&rb_b, fp_rt), [0, 0, 0], CopyLoc::Subresource(&rt_b, 0), None).unwrap();
    l2.copy_texture_region(CopyLoc::Footprint(&rb_d0, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds0, 0), None).unwrap();
    l2.copy_texture_region(CopyLoc::Footprint(&rb_d1, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds1, 0), None).unwrap();
    l2.close().unwrap();

    q.submit(&[&l_prev.cb, &l_nodisp.cb, &l2.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb_a, 1)[0], 0xff00_ff00, "rt_a must clear to green over prior red");
    assert_eq!(buf_words(&rb_b, 1)[0], 0xffff_0000, "rt_b must clear to blue over prior red");
    let d0 = f32::from_bits(buf_words(&rb_d0, 1)[0]);
    let d1 = f32::from_bits(buf_words(&rb_d1, 1)[0]);
    assert!((d0 - 0.25).abs() < 1e-5, "ds0 depth mismatch: got {d0}");
    assert!((d1 - 0.75).abs() < 1e-5, "ds1 depth mismatch: got {d1}");
}

#[test]
fn icb_fast_v3_executes_ps_and_vs_only_depth_draws_across_recycled_icb_ring_on_gpu() {
    use crate::metal::indirect::{arg, IndirectLayout};
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none()
        || !dxil_metal::converter_available()
        || !dxil_metal::dxilconv::available()
    {
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
    let mut gp_color = GraphicsPipeline::new_with_opts(
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
        "g36_icb_v3_color",
        true,
    )
    .unwrap();
    gp_color.needs_draw_params = true;
    let pso_color = Arc::new(gp_color);
    let pso_depth = Arc::new(
        GraphicsPipeline::new_with_opts(
            rs.clone(),
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
            "g36_icb_v3_depth",
            true,
        )
        .unwrap(),
    );
    assert!(pso_color.ps.is_some());
    assert!(pso_depth.ps.is_none());
    assert!(pso_color.needs_draw_params);

    // Indirect layout: 4 root-constant dwords (`cb0[0].xyzw`) + `D3D12_DRAW_ARGUMENTS` (stride = 32B).
    let layout = IndirectLayout::new(32, &[(arg::CONSTANT, [0, 0, 4]), (arg::DRAW, [0; 3])]).unwrap();
    let args_up = test_buffer(256, heap_type::UPLOAD, 0);
    unsafe {
        let p = args_up.buffer().unwrap().contents().as_ptr() as *mut u32;
        // Offset 0: green `[0.0, 1.0, 0.0, 1.0]` + `Draw(3, 1, 0, 0)`
        let cmd_green: [u32; 8] = [0, 0x3f80_0000, 0, 0x3f80_0000, 3, 1, 0, 0];
        std::ptr::copy_nonoverlapping(cmd_green.as_ptr(), p, 8);
        // Offset 32: blue `[0.0, 0.0, 1.0, 1.0]` + `Draw(3, 1, 0, 0)`
        let cmd_blue: [u32; 8] = [0, 0, 0x3f80_0000, 0x3f80_0000, 3, 1, 0, 0];
        std::ptr::copy_nonoverlapping(cmd_blue.as_ptr(), p.add(8), 8);
        // Offset 64: `count = 1`
        *p.add(16) = 1;
    }

    let q = Queue::new(ListType::Direct, "g36 stage D queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let rt = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let ds = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let rb_rt = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rb_ds = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rt_id = &rt as *const Resource as usize;
    let ds_id = &ds as *const Resource as usize;
    let fp_rt = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
    let fp_ds = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };

    let _stats_guard = gpu_stats_lock();
    let v3_0 = stats::c::icb_v3_fast.load(Relaxed);

    // Pass 1 (batched `opt_icb_fast_v3 = true`, `has_ps = true`): slot 0 renders green via fragment TLAB.
    {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_icb_batch = true;
        l.opts.opt_icb_fast_v3 = true;
        l.begin(&alloc).unwrap();
        let rtv = TargetView::rtv(&rt, None).unwrap();
        unsafe { l.clear_rtv(&rtv, [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_constants(0, 0, &[0; 4]).unwrap();
        l.set_render_targets(vec![rtv], None);
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso_color.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.0, zfar: 1.0 }]);
        l.execute_indirect(&layout, 1, args_up.gpu_va, None, Some(args_up.gpu_va + 64)).unwrap();
        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        l.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp_rt), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();
        q.submit(&[&l.cb]);
        q.wait_idle();
        assert_eq!(buf_words(&rb_rt, 1)[0], 0xff00_ff00, "Pass 1 PS ICB v3 draw must output green");
    }

    // Recycle `alloc` (`IcbRing` resets back to slot 0!), then run Pass 2: 2 hoisted VS-only depth ICB draws
    // (`ps.is_none()`, `has_ps = false` -> skips `set_fragment_buffer` and uses `Vertex -> Vertex` hoisted barrier!).
    alloc.reset();
    {
        let hoist0 = stats::c::icb_hoist_saved.load(Relaxed);
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_icb_batch = true;
        l.opts.opt_icb_fast_v3 = true;
        l.begin(&alloc).unwrap();
        let dsv = TargetView::dsv(&ds, None).unwrap();
        unsafe { l.clear_dsv(&dsv, true, 0.95, false, 0, &[]).unwrap() };
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_constants(0, 0, &[0; 4]).unwrap();
        l.set_render_targets(Vec::new(), Some(dsv));
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso_depth);
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.375, zfar: 1.0 }]);
        l.set_scissors(vec![Rect { left: 0, top: 0, right: 8, bottom: 4 }]);
        l.execute_indirect(&layout, 1, args_up.gpu_va, None, Some(args_up.gpu_va + 64)).unwrap();

        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.625, zfar: 1.0 }]);
        l.set_scissors(vec![Rect { left: 0, top: 4, right: 8, bottom: 8 }]);
        l.execute_indirect(&layout, 1, args_up.gpu_va, None, Some(args_up.gpu_va + 64)).unwrap();

        l.barrier_list_with(&[Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE)], true, false);
        l.copy_texture_region(CopyLoc::Footprint(&rb_ds, fp_ds), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
        l.close().unwrap();
        q.submit(&[&l.cb]);
        q.wait_idle();

        assert_eq!(stats::c::icb_hoist_saved.load(Relaxed) - hoist0, 1);
        let words = buf_words(&rb_ds, 64 * 8);
        let d_top = f32::from_bits(words[0]);
        let d_bot = f32::from_bits(words[4 * 64]);
        assert!((d_top - 0.375).abs() < 1e-5, "VS-only depth top half mismatch: {d_top}");
        assert!((d_bot - 0.625).abs() < 1e-5, "VS-only depth bottom half mismatch: {d_bot}");
    }

    // Recycle `alloc` AGAIN back to slot 0, then run Pass 3 (`opt_icb_batch = false`, `has_ps = true`)
    // with blue root constants (`args_up + 32`), proving slot 0 re-binds `fragment_buffer(2)` cleanly!
    alloc.reset();
    {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_icb_batch = false;
        l.opts.opt_icb_fast_v3 = true;
        l.begin(&alloc).unwrap();
        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_COPY_SOURCE, STATE_RENDER_TARGET)], true, false);
        let rtv = TargetView::rtv(&rt, None).unwrap();
        unsafe { l.clear_rtv(&rtv, [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };
        l.gfx_root.set_root_signature(rs).unwrap();
        l.gfx_root.set_constants(0, 0, &[0; 4]).unwrap();
        l.set_render_targets(vec![rtv], None);
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso_color);
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.0, zfar: 1.0 }]);
        l.execute_indirect(&layout, 1, args_up.gpu_va + 32, None, Some(args_up.gpu_va + 64)).unwrap();
        l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        l.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp_rt), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
        l.close().unwrap();
        q.submit(&[&l.cb]);
        q.wait_idle();
        assert_eq!(buf_words(&rb_rt, 1)[0], 0xffff_0000, "Recycled slot 0 PS ICB v3 draw must output blue (not stale green)");
    }

    assert!(stats::c::icb_v3_fast.load(Relaxed) >= v3_0 + 4, "expected >= 4 icb_v3_fast dispatches");
}

#[test]
fn render_split_stage_sync_eliminates_false_frag_to_vtx_on_mixed_batches_and_by_written_on_gpu() {
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none()
        || !dxil_metal::converter_available()
        || !dxil_metal::dxilconv::available()
    {
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
    let pso = Arc::new(
        GraphicsPipeline::new_with_opts(
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
            "g37_split_sync_color",
            true,
        )
        .unwrap(),
    );

    let q = Queue::new(ListType::Direct, "g37 stage B queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let rt_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let vb_buf = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rt_a_id = &rt_a as *const Resource as usize;
    let rt_b_id = &rt_b as *const Resource as usize;
    let vb_id = &vb_buf as *const Resource as usize;
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };

    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_desc[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_desc[16..20].copy_from_slice(&16u32.to_le_bytes());

    let _stats_guard = gpu_stats_lock();

    // 1) Legacy mode (`opt_render_split_stage_sync = false`): a mixed batch with
    //    `UAV -> VB_CB` (`Dispatch -> PRE_RASTER`) + `RENDER_TARGET -> PIXEL_SRV` (`Fragment -> Fragment`)
    //    falsely unions `Fragment` into `PRE_RASTER` (`qb_frag_to_vtx == 1`).
    {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_render_split_stage_sync = false;
        l.begin(&alloc).unwrap();
        l.clear_uav_with(&vb_buf, Some(&uav_desc), ClearValue::Uint([0x3f80_0000, 0, 0, 0]), &[], true).unwrap();
        let rtv_a = TargetView::rtv(&rt_a, None).unwrap();
        let rtv_b = TargetView::rtv(&rt_b, None).unwrap();
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_constants(0, 0, &[0x3f80_0000, 0, 0, 0x3f80_0000]).unwrap();
        l.set_render_targets(vec![rtv_a], None);
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        l.clear_all_pend();

        let f2v_0 = stats::c::qb_frag_to_vtx.load(Relaxed);
        l.barrier_list_with(
            &[
                Barrier::Transition(vb_id, STATE_COPY_DEST, STATE_VB_CB),
                Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
            ],
            true,
            false,
        );
        l.gfx_root.set_constants(0, 0, &[0, 0x3f80_0000, 0, 0x3f80_0000]).unwrap();
        l.set_render_targets(vec![rtv_b], None);
        l.draw(3, 1, 0, 0).unwrap();
        assert_eq!(stats::c::qb_frag_to_vtx.load(Relaxed) - f2v_0, 1, "legacy mode leaks Fragment -> PRE_RASTER on mixed batch");
        l.close().unwrap();
        q.submit(&[&l.cb]);
        q.wait_idle();
    }

    // 2) Split mode (`opt_render_split_stage_sync = true`):
    //    (a) Mixed batch `[COPY_DEST -> VB_CB, RENDER_TARGET -> PIXEL_SRV]` emits split
    //        `Blit -> (PRE_RASTER | Fragment)` + `Fragment -> Fragment` with `qb_frag_to_vtx == 0`!
    //    (b) `by_written` same-state RTV attachment alongside `COPY_DEST -> INDEX_BUFFER` (`0x2`) +
    //        `RENDER_TARGET -> NON_PIXEL_SRV` orders `Blit -> PRE_RASTER` + `Fragment -> Fragment`
    //        (`qb_frag_to_vtx == 0`) AND preserves `disp_after == Fragment` for Compute!
    alloc.reset();
    {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_render_split_stage_sync = true;
        l.begin(&alloc).unwrap();
        l.clear_uav_with(&vb_buf, Some(&uav_desc), ClearValue::Uint([0x3f80_0000, 0, 0, 0]), &[], true).unwrap();
        let rtv_a = TargetView::rtv(&rt_a, None).unwrap();
        let rtv_b = TargetView::rtv(&rt_b, None).unwrap();
        l.gfx_root.set_root_signature(rs).unwrap();
        l.gfx_root.set_constants(0, 0, &[0x3f80_0000, 0, 0, 0x3f80_0000]).unwrap();
        l.set_render_targets(vec![rtv_a], None);
        l.gfx.topology = 4;
        l.set_graphics_pipeline(pso);
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        l.clear_all_pend();

        let f2v_0 = stats::c::qb_frag_to_vtx.load(Relaxed);
        let split_0 = stats::c::render_split_sync.load(Relaxed);
        l.barrier_list_with(
            &[
                Barrier::Transition(vb_id, STATE_COPY_DEST, STATE_VB_CB),
                Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
            ],
            true,
            false,
        );
        l.gfx_root.set_constants(0, 0, &[0, 0x3f80_0000, 0, 0x3f80_0000]).unwrap();
        l.set_render_targets(vec![rtv_b], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        assert_eq!(stats::c::qb_frag_to_vtx.load(Relaxed) - f2v_0, 0, "split mode must emit 0 false qb_frag_to_vtx on mixed batch");
        assert!(stats::c::render_split_sync.load(Relaxed) > split_0, "split mode must increment render_split_sync");

        // Case (b): `rt_b` is in `self.written`; `vb_buf` transitions `COPY_DEST -> INDEX_BUFFER` (`0x2`, `PRE_RASTER`)
        // and `rt_a` transitions `RENDER_TARGET -> NON_PIXEL_SRV` (`disp_after = Fragment`, `soft_before = PRE_RASTER`).
        // Re-attaching `rt_b` must emit `Blit -> PRE_RASTER` + `Fragment -> Fragment` (`qb_frag_to_vtx == 0`)
        // while keeping `disp_after = Fragment`!
        l.bar.written.push(rt_b_id);
        l.barrier_list_with(
            &[
                Barrier::Transition(vb_id, STATE_COPY_DEST, 0x2),
                Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
            ],
            true,
            false,
        );
        let f2v_1 = stats::c::qb_frag_to_vtx.load(Relaxed);
        l.gfx_root.set_constants(0, 0, &[0, 0, 0x3f80_0000, 0x3f80_0000]).unwrap();
        l.set_render_targets(vec![rtv_b], None);
        l.draw(3, 1, 0, 0).unwrap();
        l.end_encoder();
        assert_eq!(stats::c::qb_frag_to_vtx.load(Relaxed) - f2v_1, 0, "by_written + INDEX_BUFFER + NON_PIXEL_SRV must not leak Fragment -> PRE_RASTER");
        assert!(l.bar.split_pend.disp_after.contains(MTLStages::Fragment), "disp_after=Fragment must remain intact for subsequent Compute encoder");

        l.barrier_list_with(&[Barrier::Transition(rt_b_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt_b, 0), None).unwrap();
        l.close().unwrap();
        q.submit(&[&l.cb]);
        q.wait_idle();
        assert_eq!(buf_words(&rb, 1)[0], 0xffff_0000, "final blue draw on rt_b must resolve cleanly");
    }
}

#[test]
fn mid_qb_stagesync_skips_satisfied_mid_pass_barriers_with_zero_bad_timestamps_on_gpu() {
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none()
        || !dxil_metal::converter_available()
        || !dxil_metal::dxilconv::available()
    {
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
    let pso = Arc::new(
        GraphicsPipeline::new_with_opts(
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
            "g37_mid_qb_stagesync",
            true,
        )
        .unwrap(),
    );

    let q = Queue::new(ListType::Direct, "g37 stage C queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let src_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let buf1 = test_buffer(64, heap_type::DEFAULT, 0);
    let buf2 = test_buffer(64, heap_type::DEFAULT, 0);
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rt_id = &rt as *const Resource as usize;
    let buf1_id = &buf1 as *const Resource as usize;
    let buf2_id = &buf2 as *const Resource as usize;
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };

    let _stats_guard = gpu_stats_lock();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_stagesync_barrier_v2 = false;
    l.opts.opt_mid_qb_stagesync = true;
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    // 1) Compute/Blit encoder copies into both `buf1` and `buf2` (`Blit` gen advances once on `end_encoder`,
    //    anchoring `stage_sync[Blit].pre_list_all = true`).
    l.copy_buffer(&buf1, 0, &src_buf, 0, 16).unwrap();
    l.copy_buffer(&buf2, 0, &src_buf, 0, 16).unwrap();
    l.end_encoder();
    l.clear_all_pend();

    // 2) Transition `buf1: COPY_DEST -> PIXEL_SRV` and open the render pass:
    //    `begin_barrier` emits `Blit -> Fragment` (WITHOUT `prune_dominated_after`, so `stagesync_narrow == 0`)
    //    and records `enc_cur_sync[Fragment]` synced with `stage_gen[Blit]`.
    let narrow0 = stats::c::stagesync_narrow.load(Relaxed);
    let skip0 = stats::c::stagesync_skip.load(Relaxed);
    let mid0 = stats::c::rb_mid_qb.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(buf1_id, STATE_COPY_DEST, STATE_PIXEL_SRV)], true, false);
    let rtv = TargetView::rtv(&rt, None).unwrap();
    l.gfx_root.set_root_signature(rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 0x3f80_0000, 0, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![rtv], None);
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();

    // 3) Mid-pass `Transition(buf2, COPY_DEST, PIXEL_SRV)` is kept inside the open draw pass (`rb_keep`).
    //    Because `Blit` is ALREADY transitively satisfied on `Fragment` for this encoder,
    //    `render_mid_barrier` skips the redundant mid-pass queue barrier (`stagesync_skip += 1`, `rb_mid_qb == 0`)!
    l.barrier_list_with(&[Barrier::Transition(buf2_id, STATE_COPY_DEST, STATE_PIXEL_SRV)], true, false);
    assert!(l.gfx.draw_pass, "unattached COPY_DEST -> PIXEL_SRV must keep draw pass open");
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::rb_mid_qb.load(Relaxed) - mid0, 0, "satisfied mid-pass render barrier must be skipped");
    assert_eq!(stats::c::stagesync_skip.load(Relaxed) - skip0, 1, "mid_qb_stagesync must increment stagesync_skip");
    assert_eq!(stats::c::stagesync_narrow.load(Relaxed) - narrow0, 0, "begin_barrier prune_dominated_after must remain OFF");

    l.end_encoder();
    l.barrier_list_with(&[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb, 1)[0], 0xff00_ff00, "green draw output must resolve cleanly");
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "mid_qb_stagesync must produce 0 inverted timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn ro_dsv_dontcare_skips_tile_store_and_waw_barriers_while_preserving_depth_stencil_contents_on_gpu() {
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState, StencilFace};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };
    use objc2_metal::MTLStencilOperation;
    use std::sync::atomic::Ordering::Relaxed;

    const D32_FLOAT: u32 = 40;
    const R8_UINT: u32 = 62;

    if crate::metal::device().is_none()
        || !dxil_metal::converter_available()
        || !dxil_metal::dxilconv::available()
    {
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

    // PSO 1: writable depth (top-half setup on D32_S8).
    let pso_write_d = Arc::new(
        GraphicsPipeline::new_with_opts(
            rs.clone(),
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
            "g37_ro_dsv_write",
            true,
        )
        .unwrap(),
    );

    // PSO 2: read-only depth (`Less`, `depth_write = false`) + read-only stencil (`Equal`, `write_mask = 0`).
    let ro_stencil_face = StencilFace {
        compare: MTLCompareFunction::Equal,
        fail: MTLStencilOperation::Keep,
        depth_fail: MTLStencilOperation::Keep,
        pass: MTLStencilOperation::Keep,
        read_mask: 0xff,
        write_mask: 0,
    };
    let pso_ro_ds = Arc::new(
        GraphicsPipeline::new_with_opts(
            rs.clone(),
            &GraphicsDesc {
                vs: &vs_dxil,
                ps: Some(&ps_cb0_dxbc),
                blend: color_blend,
                sample_mask: u32::MAX,
                raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
                depth_stencil: DepthStencilState {
                    depth_compare: MTLCompareFunction::Less,
                    depth_write: false,
                    front: Some(ro_stencil_face),
                    back: Some(ro_stencil_face),
                    ..DepthStencilState::default()
                },
                input_layout: Vec::new(),
                strip_cut: 0,
                topology_type: 3,
                rtv_formats: vec![RGBA8_UNORM],
                dsv_format: D32_S8,
                sample_count: 1,
            },
            "g37_ro_dsv_test",
            true,
        )
        .unwrap(),
    );

    // PSO 3: read-only depth + WRITABLE stencil (`IncrementClamp`, `write_mask = 0xff`) for mixed DSV flags = 0x1!
    let rw_stencil_face = StencilFace {
        compare: MTLCompareFunction::Equal,
        fail: MTLStencilOperation::Keep,
        depth_fail: MTLStencilOperation::Keep,
        pass: MTLStencilOperation::IncrementClamp,
        read_mask: 0xff,
        write_mask: 0xff,
    };
    let pso_ro_d_rw_s = Arc::new(
        GraphicsPipeline::new_with_opts(
            rs.clone(),
            &GraphicsDesc {
                vs: &vs_dxil,
                ps: Some(&ps_cb0_dxbc),
                blend: color_blend,
                sample_mask: u32::MAX,
                raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
                depth_stencil: DepthStencilState {
                    depth_compare: MTLCompareFunction::Less,
                    depth_write: false,
                    front: Some(rw_stencil_face),
                    back: Some(rw_stencil_face),
                    ..DepthStencilState::default()
                },
                input_layout: Vec::new(),
                strip_cut: 0,
                topology_type: 3,
                rtv_formats: vec![RGBA8_UNORM],
                dsv_format: D32_S8,
                sample_count: 1,
            },
            "g37_ro_d_rw_s",
            true,
        )
        .unwrap(),
    );

    // PSO 4: single-plane `D32_FLOAT` read-only depth (`flags = DSV_READ_ONLY_DEPTH = 0x1` only).
    let pso_ro_d32 = Arc::new(
        GraphicsPipeline::new_with_opts(
            rs.clone(),
            &GraphicsDesc {
                vs: &vs_dxil,
                ps: Some(&ps_cb0_dxbc),
                blend: color_blend,
                sample_mask: u32::MAX,
                raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
                depth_stencil: DepthStencilState {
                    depth_compare: MTLCompareFunction::Less,
                    depth_write: false,
                    ..DepthStencilState::default()
                },
                input_layout: Vec::new(),
                strip_cut: 0,
                topology_type: 3,
                rtv_formats: vec![RGBA8_UNORM],
                dsv_format: D32_FLOAT,
                sample_count: 1,
            },
            "g37_ro_d32",
            true,
        )
        .unwrap(),
    );

    let q = Queue::new(ListType::Direct, "g37 stage D queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let ds = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let d32 = test_texture(8, 8, D32_FLOAT, resource_flags::ALLOW_DEPTH_STENCIL);
    let rt_a = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_b = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_c = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb_rt_a = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rb_rt_b = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rb_depth = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rb_stencil = test_buffer(256 * 8, heap_type::READBACK, 0);

    let ds_id = &ds as *const Resource as usize;
    let d32_id = &d32 as *const Resource as usize;
    let rt_a_id = &rt_a as *const Resource as usize;
    let rt_b_id = &rt_b as *const Resource as usize;
    let fp_rgba = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
    let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
    let fp_s = PlacedFootprint { offset: 0, format: R8_UINT, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };

    let dsv_rw = TargetView::dsv(&ds, None).unwrap();
    let dsv_ro_both = TargetView::dsv(&ds, Some(&[D32_S8, 3, 3, 0, 0, 0])).unwrap();
    let dsv_ro_d_only = TargetView::dsv(&ds, Some(&[D32_S8, 3, 1, 0, 0, 0])).unwrap();
    let d32_ro = TargetView::dsv(&d32, Some(&[D32_FLOAT, 3, 1, 0, 0, 0])).unwrap();
    let rtv_a = TargetView::rtv(&rt_a, None).unwrap();
    let rtv_b = TargetView::rtv(&rt_b, None).unwrap();
    let rtv_c = TargetView::rtv(&rt_c, None).unwrap();

    let _stats_guard = gpu_stats_lock();
    let dc0 = stats::c::ro_dsv_dontcare.load(Relaxed);

    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_ro_dsv_dontcare = true;
    l.begin(&alloc).unwrap();

    // 1) Initialize `ds` (`D32_S8`): clear depth = 0.5, stencil = 0x2A, and write depth = 0.25 on top half (y=0..4).
    unsafe {
        l.clear_dsv(&dsv_rw, true, 0.5, true, 0x2A, &[]).unwrap();
        l.clear_rtv(&rtv_a, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
        l.clear_rtv(&rtv_b, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
        l.clear_rtv(&rtv_c, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
        l.clear_dsv(&TargetView::dsv(&d32, None).unwrap(), true, 0.5, false, 0, &[]).unwrap();
    }
    l.gfx_root.set_root_signature(rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 0x3f80_0000, 0, 0x3f80_0000]).unwrap();
    l.set_render_targets(Vec::new(), Some(dsv_rw));
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_write_d);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.25, zfar: 1.0 }]);
    l.set_scissors(vec![Rect { left: 0, top: 0, right: 8, bottom: 4 }]);
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();

    l.barrier_list_with(&[Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_DEPTH_READ)], true, false);

    // 2) Two back-to-back read-only DSV passes (`dsv_ro_both`, flags = 0x3) into distinct RTVs (`rt_a`, `rt_b`):
    //    - Pass A draws green at z=0.375 with stencil_ref=0x2A: fails top half (0.375 >= 0.25 -> stays red), passes bottom half (0.375 < 0.5 -> green)!
    //    - Pass B draws blue at z=0.125 with stencil_ref=0x2A: passes BOTH halves (0.125 < 0.25 & 0.5 -> blue), WITHOUT emitting a false `qb_written` barrier!
    let qbw0 = stats::c::qb_written.load(Relaxed);
    l.set_render_targets(vec![rtv_a], Some(dsv_ro_both));
    l.set_graphics_pipeline(pso_ro_ds.clone());
    l.set_stencil_ref(0x2A);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.375, zfar: 1.0 }]);
    l.set_scissors(vec![Rect { left: 0, top: 0, right: 8, bottom: 8 }]);
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();
    assert!(!l.bar.written.contains(&ds_id), "read-only DSV must NOT be added to self.written");

    l.gfx_root.set_constants(0, 0, &[0, 0, 0x3f80_0000, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![rtv_b], Some(dsv_ro_both));
    l.set_graphics_pipeline(pso_ro_ds);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.125, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();
    assert_eq!(stats::c::qb_written.load(Relaxed) - qbw0, 0, "back-to-back read-only DSV passes must emit 0 qb_written");
    assert_eq!(stats::c::ro_dsv_dontcare.load(Relaxed) - dc0, 4, "2 passes x 2 planes (depth + stencil) = 4 ro_dsv_dontcare");

    // 3) Mixed `DSV_READ_ONLY_DEPTH` ONLY (`flags = 0x1`) on `D32_S8`: depth plane uses `DontCare` (+1),
    //    stencil plane uses `Store` (increments 0x2A -> 0x2B where depth z=0.375 passes on bottom half!),
    //    and `ds_id` IS recorded in `self.written` because stencil was written!
    l.barrier_list_with(&[Barrier::Transition(ds_id, STATE_DEPTH_READ, STATE_DEPTH_WRITE)], true, false);
    let dc1 = stats::c::ro_dsv_dontcare.load(Relaxed);
    l.set_render_targets(vec![rtv_c], Some(dsv_ro_d_only));
    l.set_graphics_pipeline(pso_ro_d_rw_s);
    l.set_stencil_ref(0x2A);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.375, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();
    assert_eq!(stats::c::ro_dsv_dontcare.load(Relaxed) - dc1, 1, "mixed DSV_READ_ONLY_DEPTH on D32_S8 must DontCare depth only");
    assert!(l.bar.written.contains(&ds_id), "mixed DSV with writable stencil MUST record ds_id in self.written");

    // 4) Single-plane `D32_FLOAT` with `DSV_READ_ONLY_DEPTH` ONLY (`flags = 0x1`):
    //    `has_stencil == false`, so `d32_id` is completely read-only (`!written.contains(&d32_id)`),
    //    whereas legacy `v.flags & 0x3 != 0x3` falsely treated it as written!
    l.barrier_list_with(&[Barrier::Transition(d32_id, STATE_DEPTH_WRITE, STATE_DEPTH_READ)], true, false);
    let dc2 = stats::c::ro_dsv_dontcare.load(Relaxed);
    l.set_render_targets(vec![rtv_c], Some(d32_ro));
    l.set_graphics_pipeline(pso_ro_d32);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.25, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();
    assert_eq!(stats::c::ro_dsv_dontcare.load(Relaxed) - dc2, 1, "single-plane D32_FLOAT with DSV_READ_ONLY_DEPTH must use DontCare");
    assert!(!l.bar.written.contains(&d32_id), "single-plane D32_FLOAT with DSV_READ_ONLY_DEPTH must NOT be added to self.written");

    // Read back `rt_a`, `rt_b`, and both planes of `ds` (`D32_S8`) to prove 100% VRAM preservation!
    l.barrier_list_with(
        &[
            Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(rt_b_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE),
        ],
        true,
        false,
    );
    l.copy_texture_region(CopyLoc::Footprint(&rb_rt_a, fp_rgba), [0, 0, 0], CopyLoc::Subresource(&rt_a, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_rt_b, fp_rgba), [0, 0, 0], CopyLoc::Subresource(&rt_b, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_depth, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_stencil, fp_s), [0, 0, 0], CopyLoc::Subresource(&ds, 1), None).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    let px_a = buf_words(&rb_rt_a, 64 * 8);
    assert_eq!(px_a[0], 0xff00_00ff, "rt_a top half must fail depth test and stay red");
    assert_eq!(px_a[4 * 64], 0xff00_ff00, "rt_a bottom half must pass depth+stencil test and turn green");

    let px_b = buf_words(&rb_rt_b, 64 * 8);
    assert_eq!(px_b[0], 0xffff_0000, "rt_b top half must pass depth+stencil in 2nd RO pass (VRAM preserved!)");
    assert_eq!(px_b[4 * 64], 0xffff_0000, "rt_b bottom half must pass depth+stencil in 2nd RO pass");

    let d_words = buf_words(&rb_depth, 64 * 8);
    assert!((f32::from_bits(d_words[0]) - 0.25).abs() < 1e-5, "depth plane top half must remain 0.25 after DontCare RO passes");
    assert!((f32::from_bits(d_words[4 * 64]) - 0.5).abs() < 1e-5, "depth plane bottom half must remain 0.5 after DontCare RO passes");

    let s_words = buf_words(&rb_stencil, 64 * 8);
    assert_eq!(s_words[0] & 0xff, 0x2A, "stencil top half (failed depth in step 3) must remain 0x2A");
    assert_eq!(s_words[4 * 64] & 0xff, 0x2B, "stencil bottom half (passed depth in step 3) must store incremented 0x2B");
}

#[test]
fn split_res_soft_sync_defers_generic_read_and_kept_uav_while_keeping_common_tex_strict_on_gpu() {
    use crate::metal::pipeline::{
        ComputeAccess, ComputePipeline, GraphicsDesc, GraphicsPipeline, READS_BUF, READS_TEX,
    };
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
        depth_stencil: DepthStencilState {
            depth_compare: MTLCompareFunction::Always,
            depth_write: true,
            ..DepthStencilState::default()
        },
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: 3,
        rtv_formats: vec![RGBA8_UNORM],
        dsv_format: D32_S8,
        sample_count: 1,
    };
    let mut gp_no_srv = GraphicsPipeline::new_with_opts(gfx_rs.clone(), &gdesc, "g38_c_no_srv", true).unwrap();
    gp_no_srv.writes_uav = false;
    gp_no_srv.sm66_writes_uav = false;
    gp_no_srv.sm66_pre_raster_reads = 0;
    let pso_no_srv_vs = Arc::new(gp_no_srv);
    assert_eq!(pso_no_srv_vs.pre_raster_reads, 0);
    let mut gp_tex_vs = GraphicsPipeline::new_with_opts(gfx_rs.clone(), &gdesc, "g38_c_tex_vs", true).unwrap();
    gp_tex_vs.writes_uav = false;
    gp_tex_vs.sm66_writes_uav = false;
    gp_tex_vs.pre_raster_reads = READS_TEX;
    gp_tex_vs.sm66_pre_raster_reads = READS_TEX;
    let pso_tex_vs = Arc::new(gp_tex_vs);

    let tex_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_mid = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_common = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_cs = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let ds1 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let ds2 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let buf_uav = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_b = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&tex_a, &tex_b, &tex_mid, &tex_common, &tex_cs, &ds1, &ds2, &buf_uav] {
        r.register_ptr();
    }
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let tex_mid_id = &tex_mid as *const Resource as usize;
    let tex_common_id = &tex_common as *const Resource as usize;
    let tex_cs_id = &tex_cs as *const Resource as usize;
    let buf_uav_id = &buf_uav as *const Resource as usize;

    let q = Queue::new(ListType::Direct, "g38 stage C queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_split_res_soft_sync = true;
    l.opts.opt_barrier_tight_v2 = false; // Prove Stage C works standalone with `common_tex_keep` strictly OFF!
    l.opts.opt_render_split_stage_sync = true;
    l.opts.opt_rp_keep_write = true;
    l.opts.opt_ib_tight = true;
    l.opts.opt_ib_exact_obl = true;

    let _stats_guard = gpu_stats_lock();
    l.opts.opt_icb_enc_barrier_exact = false;
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();

    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let dsv1 = TargetView::dsv(&ds1, None).unwrap();
    let dsv2 = TargetView::dsv(&ds2, None).unwrap();
    unsafe {
        l.clear_dsv(&dsv1, true, 1.0, false, 0, &[]).unwrap();
        l.clear_dsv(&dsv2, true, 1.0, false, 0, &[]).unwrap();
        l.clear_rtv(&rtv_a, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
        l.clear_rtv(&rtv_b, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
    }
    l.gfx_root.set_root_signature(gfx_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 0x3f80_0000, 0, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![rtv_a], Some(dsv1));
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_no_srv_vs.clone());
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();

    let f2v_0 = stats::c::qb_frag_to_vtx.load(Relaxed);
    let dv_def0 = stats::c::dv_defer.load(Relaxed);
    let soft0 = stats::c::res_soft_sync.load(Relaxed);

    // 1) Kept `Barrier::Uav(0)` + `Transition(buf_uav, PIXEL_SRV -> UAV)` inside Pass 1:
    //    routes `PRE_RASTER` into `rp_write_soft` without hard `rp_write_before & PRE_RASTER`!
    l.barrier_list_with(
        &[
            Barrier::Uav(0),
            Barrier::Transition(buf_uav_id, STATE_PIXEL_SRV, STATE_UAV),
        ],
        true,
        true,
    );
    assert!(l.gfx.draw_pass);
    assert!(!l.rp_sync.rp_write_before.intersects(PRE_RASTER));
    assert!((l.rp_sync.rp_write_soft[0] | l.rp_sync.rp_write_soft[1]).contains(PRE_RASTER));

    // 2) Mid-pass `Transition(tex_mid, RTV -> STATE_GENERIC_READ)` inside Pass 1 (`render_split_sync_active()`):
    //    strips buffer-only `0x203` bits, defers `PRE_RASTER` into `vtx_defer` (`dv_defer`), and emits 0 `qb_frag_to_vtx`!
    l.barrier_list_with(&[Barrier::Transition(tex_mid_id, STATE_RENDER_TARGET, STATE_GENERIC_READ)], true, true);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(stats::c::dv_defer.load(Relaxed) > dv_def0, "mid-pass GENERIC_READ texture must defer PRE_RASTER into vtx_defer");
    assert!(stats::c::res_soft_sync.load(Relaxed) >= soft0 + 3, "expected res_soft_sync increments");

    // 3) Switch to Pass 2 (`rtv_b`, `dsv2`) + `Transition(tex_a, RTV -> STATE_GENERIC_READ)`:
    //    0 `qb_frag_to_vtx` when `pre_raster_reads == 0`, then `dv_hit` fires when `pso_tex_vs` (`READS_TEX`) draws!
    l.set_render_targets(vec![rtv_b.clone()], Some(dsv2));
    l.barrier_list_with(&[Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_GENERIC_READ)], true, true);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::qb_frag_to_vtx.load(Relaxed), f2v_0, "Stage C must emit 0 false qb_frag_to_vtx when pre_raster_reads == 0");

    let hit0 = stats::c::dv_hit.load(Relaxed);
    l.set_graphics_pipeline(pso_tex_vs);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(stats::c::dv_hit.load(Relaxed) > hit0, "VS reading READS_TEX must fire deferred vtx_defer barrier");

    // 4) Strict `s1 == 0` guardrail: `Transition(tex_common, RTV -> STATE_COMMON)` (`s1 == 0`) MUST split Pass 2!
    let split0 = stats::c::app_barrier_split.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(tex_common_id, STATE_RENDER_TARGET, STATE_COMMON)], true, true);
    assert!(!l.gfx.draw_pass, "common_tex_keep (s1 == 0) MUST stay strictly OFF under AW2_SPLIT_RES_SOFT_SYNC=1");
    assert_eq!(stats::c::app_barrier_split.load(Relaxed) - split0, 1);

    // 5) Compute `note_compute_barriers_tight`: `Transition(tex_cs, RTV -> STATE_GENERIC_READ)` classifies as
    //    `OblKind::ClassSrv(READS_TEX)` so unrelated buffer-only dispatches skip `enc_barrier`!
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
    let cs_base = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g38_c_cs").unwrap();
    let cs_pso = Arc::new(ComputePipeline {
        pso: cs_base.pso,
        rs: cs_base.rs,
        threadgroup: cs_base.threadgroup,
        shader: cs_base.shader,
        hk_profile: cs_base.hk_profile,
        access: ComputeAccess {
            uav_slots: cs_base.access.uav_slots,
            read_slots: None,
            bounded_read_slots: Vec::new(),
            unbounded_srv_classes: READS_BUF,
            heap_indexed_or_unknown: false,
            touches_textures: false,
            ..ComputeAccess::opaque()
        },
    });
    unsafe { l.clear_rtv(&TargetView::rtv(&tex_cs, None).unwrap(), [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };
    l.barrier_list_with(&[Barrier::Transition(tex_cs_id, STATE_RENDER_TARGET, STATE_GENERIC_READ)], true, true);
    assert!(
        l.ib.ib_obl_tight.iter().any(|o| o.r == tex_cs_id && o.kind == OblKind::ClassSrv(READS_TEX)),
        "GENERIC_READ texture in compute must classify as OblKind::ClassSrv(READS_TEX)"
    );
    let skip_ex0 = stats::c::ib_skip_exact.load(Relaxed);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, 0).unwrap();
    l.compute_root.set_descriptor(1, buf_uav.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso);
    l.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::ib_skip_exact.load(Relaxed) > skip_ex0);

    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.barrier_list_with(&[Barrier::Transition(tex_b_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)], true, false);
    l.copy_texture_region(CopyLoc::Footprint(&rb_b, fp), [0, 0, 0], CopyLoc::Subresource(&tex_b, 0), None).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb_b, 1)[0], 0xff00_ff00, "Pass 2 green draw must resolve cleanly");
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "Stage C must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}

#[test]
fn cs_uav_producer_exact_narrows_uav_transitions_and_preserves_gfx_and_passthrough_guards_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputePipeline, GraphicsDesc, GraphicsPipeline};
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
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g38_d_cs").unwrap());

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
    let mut gp_no_uav = GraphicsPipeline::new_with_opts(gfx_rs.clone(), &gdesc, "g38_d_gfx_no_uav", true).unwrap();
    gp_no_uav.writes_uav = false;
    gp_no_uav.sm66_writes_uav = false;
    let pso_no_uav = Arc::new(gp_no_uav);
    let mut gp_uav = GraphicsPipeline::new_with_opts(gfx_rs.clone(), &gdesc, "g38_d_gfx_uav", true).unwrap();
    gp_uav.writes_uav = true;
    gp_uav.sm66_writes_uav = true;
    let pso_writes_uav = Arc::new(gp_uav);

    let tex_srv = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_c = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let uav_buf1 = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_buf2 = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_uav1 = test_buffer(256, heap_type::READBACK, 0);
    let rb_rta = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&tex_srv, &rt_a, &rt_b, &rt_c, &uav_buf1, &uav_buf2] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g38 stage D heap").unwrap();
    unsafe {
        let e0 = views::srv_entry(Some(&tex_srv), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }

    let q = Queue::new(ListType::Direct, "g38 stage D queue").unwrap();
    let a_init = CommandAllocator::new(ListType::Direct).unwrap();
    let a_main = CommandAllocator::new(ListType::Direct).unwrap();
    let tex_srv_id = &tex_srv as *const Resource as usize;
    let rt_a_id = &rt_a as *const Resource as usize;
    let uav1_id = &uav_buf1 as *const Resource as usize;
    let uav2_id = &uav_buf2 as *const Resource as usize;

    // Seed `tex_srv` = `[10.0, 20.0, 30.0, 1.0]` in `l_init`.
    let mut l_init = CommandList::new(ListType::Direct).unwrap();
    l_init.begin(&a_init).unwrap();
    unsafe {
        l_init.clear_rtv(&TargetView::rtv(&tex_srv, None).unwrap(), [10.0, 20.0, 30.0, 1.0], &[]).unwrap();
    }
    l_init.barrier_list_with(&[Barrier::Transition(tex_srv_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, true);
    l_init.close().unwrap();

    let _stats_guard = gpu_stats_lock();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_cs_uav_producer_exact = true;
    l.opts.opt_render_split_stage_sync = true;
    l.opts.opt_mid_qb_stagesync = true;
    l.begin(&a_main).unwrap();
    l.pt = pt::PassLog::acquire();

    // Guardrail 1: before any `Dispatch` (`stage_gen[STAGE_DISPATCH] == 0`), `can_narrow_uav_to_cs()` MUST be false!
    l.bar.needs_barrier = false;
    let prod_ex0 = stats::c::cs_uav_prod_exact.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(uav1_id, STATE_UAV, STATE_VB_CB)], true, false);
    assert_eq!(stats::c::cs_uav_prod_exact.load(Relaxed), prod_ex0, "must NOT narrow UAV producer before any synced Dispatch");
    assert!((l.bar.split_pend.vtx_after | l.bar.split_pend.soft_after[1]).contains(MTLStages::Fragment));
    l.clear_all_pend();
    l.bar.needs_barrier = true;

    // 1) Compute encoder consumes `FullWhy::ListStart` (`All -> All`) and writes `uav_buf1` & `uav_buf2`
    //    (`stage_gen[STAGE_DISPATCH] == 1`, `stage_sync[STAGE_DISPATCH].pre_list_all == true`).
    l.set_descriptor_heaps(heap.gpu_base, 0);
    l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_buf1.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso.clone());
    l.dispatch(1, 1, 1).unwrap();
    l.compute_root.set_descriptor(1, uav_buf2.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    l.end_encoder();

    // 2) Non-UAV draw Pass 1 (`!writes_uav`, `gfx_uav_gen == 0`) draws green to `rt_a` (`stage_gen[Fragment]` advances!).
    l.gfx_root.set_root_signature(gfx_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 0x3f80_0000, 0, 0x3f80_0000]).unwrap();
    l.set_render_targets(vec![TargetView::rtv(&rt_a, None).unwrap()], None);
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_no_uav.clone());
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.0, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();
    assert_eq!(l.bar.gfx_uav_gen, 0, "non-UAV draw pass must keep gfx_uav_gen == 0");

    // 3) Transition `uav_buf1: STATE_UAV -> STATE_VB_CB | INDIRECT_ARGUMENT (0x200)` + `Barrier::Uav(uav2_id)`:
    //    Stage D narrows producer from `shader` (incl. `Fragment`) to `MTLStages::Dispatch` (`cs_uav_prod_exact >= 2`,
    //    `qb_frag_to_vtx == 0` on Pass 2 opening)!
    let f2v_0 = stats::c::qb_frag_to_vtx.load(Relaxed);
    let prod_ex1 = stats::c::cs_uav_prod_exact.load(Relaxed);
    l.barrier_list_with(
        &[
            Barrier::Transition(uav1_id, STATE_UAV, STATE_VB_CB | 0x200),
            Barrier::Uav(uav2_id),
        ],
        true,
        false,
    );
    assert!(stats::c::cs_uav_prod_exact.load(Relaxed) >= prod_ex1 + 2, "expected cs_uav_prod_exact increments on Transition + Barrier::Uav");
    assert_eq!(l.bar.split_pend.vtx_after | l.bar.split_pend.soft_after[1], MTLStages::Dispatch, "UAV -> VB_CB|INDIRECT_ARG producer must narrow to Dispatch only");

    // Open Pass 2 (`rt_b`): waits on `Dispatch` only (`qb_frag_to_vtx == 0`), AND mid-pass `Transition(uav2_id, UAV -> PIXEL_SRV)`
    // also narrows to `Dispatch` (already satisfied on Pass 2!) -> skips `rb_mid_qb` (`stagesync_skip`++)!
    l.set_render_targets(vec![TargetView::rtv(&rt_b, None).unwrap()], None);
    l.set_graphics_pipeline(pso_no_uav);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::qb_frag_to_vtx.load(Relaxed) - f2v_0, 0, "Stage D must eliminate false Fragment -> PRE_RASTER on UAV -> VB_CB|INDIRECT");

    let rb_mid0 = stats::c::rb_mid_qb.load(Relaxed);
    let skip0 = stats::c::stagesync_skip.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(uav2_id, STATE_UAV, STATE_PIXEL_SRV)], true, false);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(stats::c::rb_mid_qb.load(Relaxed) - rb_mid0, 0, "Stage B + Stage D synergy must skip mid-pass UAV -> SRV barrier when Dispatch is already synced");
    assert!(stats::c::stagesync_skip.load(Relaxed) > skip0);
    l.end_encoder();

    // Guardrail 2 (0-dispatch culled `RTV -> UAV -> COPY_SOURCE` pass-through):
    // `rt_a` transitions `RENDER_TARGET -> UAV` (`uav_passthrough` captures `(rt_a_id, Fragment)`),
    // 0 dispatches execute, and `rt_a` transitions `UAV -> COPY_SOURCE`: MUST preserve `Fragment`!
    l.barrier_list_with(&[Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_UAV)], true, false);
    assert!(
        l.uav_passthrough_for_transition(rt_a_id).contains(MTLStages::Fragment),
        "RTV -> UAV must record Fragment for rt_a in per-resource uav_passthrough"
    );
    assert!(
        l.uav_passthrough_for_transition(uav1_id).is_empty(),
        "RTV -> UAV on rt_a must NOT leak Fragment to unrelated uav_buf1"
    );
    assert!(
        l.source_stages_for_transition(rt_a_id, STATE_UAV, STATE_UAV, false)
            .contains(MTLStages::Fragment),
        "source_stages_for_transition on UAV must retain Fragment via uav_passthrough before any Dispatch runs"
    );
    l.barrier_list_with(&[Barrier::Transition(rt_a_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    assert!(
        l.bar.split_pend.blit_after.contains(MTLStages::Fragment),
        "0-dispatch RTV -> UAV -> COPY_SOURCE pass-through MUST retain Fragment producer"
    );
    assert!(
        l.uav_passthrough_for_transition(rt_a_id).is_empty(),
        "exiting STATE_UAV on rt_a must consume its uav_passthrough entry"
    );
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_rta, fp), [0, 0, 0], CopyLoc::Subresource(&rt_a, 0), None).unwrap();

    // Also verify that an unconsumed `uav_passthrough` entry is cleared when a synced `Dispatch` runs after `Fragment` wait!
    let rt_b_id = &rt_b as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(rt_b_id, STATE_RENDER_TARGET, STATE_UAV)], true, false);
    assert!(l.uav_passthrough_for_transition(rt_b_id).contains(MTLStages::Fragment));
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_buf2.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso);
    l.dispatch(1, 1, 1).unwrap();
    assert!(l.bar.uav_passthrough.is_empty(), "synced Dispatch after Fragment wait must clear uav_passthrough");
    l.end_encoder();

    // Guardrail 3 (`writes_uav == true` graphics PSO sets `gfx_uav_gen > 0` and disables `can_narrow_uav_to_cs()`):
    l.set_render_targets(vec![TargetView::rtv(&rt_c, None).unwrap()], None);
    l.set_graphics_pipeline(pso_writes_uav);
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();
    assert!(l.bar.gfx_uav_gen > 0);
    assert!(!l.can_narrow_uav_to_cs(), "gfx_uav_gen > 0 must disable UAV-to-CS narrowing");

    let prod_ex2 = stats::c::cs_uav_prod_exact.load(Relaxed);
    l.barrier_list_with(&[Barrier::Transition(uav1_id, STATE_VB_CB | 0x200, STATE_COPY_SOURCE)], true, false);
    l.barrier_list_with(&[Barrier::Transition(uav2_id, STATE_UAV, STATE_COPY_SOURCE)], true, false);
    assert_eq!(stats::c::cs_uav_prod_exact.load(Relaxed), prod_ex2, "must NOT increment cs_uav_prod_exact after gfx UAV write");
    assert!(l.bar.split_pend.blit_after.contains(MTLStages::Fragment));

    l.copy_buffer(&rb_uav1, 0, &uav_buf1, 0, 64).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log();
    q.submit(&[&l_init.cb, &l.cb]);
    q.wait_idle();

    let vals: Vec<f32> = buf_words(&rb_uav1, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&vals[..], &[10.0, 20.0, 30.0, 1.0], "uav_buf1 GPU readback mismatch");
    assert_eq!(buf_words(&rb_rta, 1)[0], 0xff00_ff00, "0-dispatch pass-through rt_a copy must read green");
    if let Some(log) = log {
        let summary = pt::resolve_for_test(vec![log]);
        assert_eq!(summary.bad, 0, "Stage D must produce 0 inverted GPU timestamps; raw_ts={:?}", summary.raw_ts);
    }
}
