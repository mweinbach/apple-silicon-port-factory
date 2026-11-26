use super::opts::*;
use super::state::*;
use super::*;
use crate::layout::PlacedFootprint;
use crate::metal::clear::ClearValue;
use crate::metal::encstats as stats;
use crate::metal::passtiming as pt;
use crate::metal::pipeline::GraphicsPipeline;
use crate::metal::queue::{ListType, Queue};
use crate::metal::resource::Resource;
use crate::types::{dim, heap_type, resource_flags, HeapProperties, Rect, ResourceDesc};
use objc2_metal::{MTLCompareFunction, MTLCullMode, MTLStages, MTLTextureUsage, MTLViewport};
use std::sync::Arc;

mod barriers;
mod clears_icb;
mod compute_ib;
mod overlap_hk;

const RGBA8_UNORM: u32 = 28;
const R32_UINT: u32 = 42;
const D32_S8: u32 = 20;
const R32_FLOAT: u32 = 41;

// D3D12_RESOURCE_STATES constants.
const STATE_COMMON: u32 = 0x0;
const STATE_VB_CB: u32 = 0x1;
const STATE_IB: u32 = 0x2;
const STATE_RENDER_TARGET: u32 = 0x4;
const STATE_UAV: u32 = 0x8;
const STATE_DEPTH_WRITE: u32 = 0x10;
const STATE_DEPTH_READ: u32 = 0x20;
const STATE_NON_PIXEL_SRV: u32 = 0x40;
const STATE_PIXEL_SRV: u32 = 0x80;
const STATE_INDIRECT_ARG: u32 = 0x200;
const STATE_COPY_DEST: u32 = 0x400;
const STATE_COPY_SOURCE: u32 = 0x800;
const STATE_GENERIC_READ: u32 =
    STATE_VB_CB | STATE_IB | STATE_NON_PIXEL_SRV | STATE_PIXEL_SRV | STATE_INDIRECT_ARG | STATE_COPY_SOURCE;

fn test_heap(ty: u32) -> HeapProperties {
    HeapProperties { Type: ty, ..Default::default() }
}

fn test_buffer(size: u64, ty: u32, flags: u32) -> Resource {
    let d = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: size,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        Flags: flags,
        ..Default::default()
    };
    Resource::new_committed(&d, &test_heap(ty), MTLTextureUsage::empty(), "cmdlist test buf").unwrap()
}

fn test_texture(w: u64, h: u32, format: u32, flags: u32) -> Resource {
    let d = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: w,
        Height: h,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: format,
        SampleCount: 1,
        Flags: flags,
        ..Default::default()
    };
    Resource::new_committed(&d, &test_heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "cmdlist test tex").unwrap()
}

fn buf_words(r: &Resource, n: usize) -> Vec<u32> {
    let b = r.buffer().unwrap();
    unsafe { std::slice::from_raw_parts(b.contents().as_ptr() as *const u32, n) }.to_vec()
}

fn gpu_stats_lock() -> std::sync::MutexGuard<'static, ()> {
    stats::test_lock()
}

fn sign_dxil(raw: &[u8]) -> Vec<u8> {
    let mut b = raw.to_vec();
    let size = u32::from_le_bytes(b[24..28].try_into().unwrap()) as usize;
    let sum = dxil_metal::hash::dxbc_checksum(&b[20..size]);
    b[4..20].copy_from_slice(&sum);
    b
}

fn cb0_color_ps_dxbc() -> Vec<u8> {
    let to_bytes = |w: &[u32]| -> Vec<u8> { w.iter().flat_map(|x| x.to_le_bytes()).collect() };
    let mut empty_s = [0u8; 8];
    empty_s[4] = 8;
    let shex = to_bytes(&[
        0x0000_0050, 16,
        0x0400_0059, 0x0020_8e46, 0, 1,
        0x0300_0065, 0x0010_20f2, 0,
        0x0600_0036, 0x0010_20f2, 0, 0x0020_8e46, 0, 0,
        0x0100_003e,
    ]);
    let mut osgn = to_bytes(&[1, 8, 32, 0, 0, 3, 0]);
    osgn.extend_from_slice(&[0x0f, 0, 0, 0]);
    osgn.extend_from_slice(b"SV_Target\0\0\0");
    dxil_metal::dxbc::build_container(&[
        (*b"ISGN", &empty_s),
        (*b"OSGN", &osgn),
        (*b"SHEX", &shex),
    ])
}

fn build_g48_vs_srv_ps_cb0_pso(
    label: &str,
) -> Option<(
    Arc<dxil_metal::root_signature::RootSignature>,
    Arc<GraphicsPipeline>,
)> {
    use crate::metal::pipeline::{
        ComputeAccess, ComputeBind, GraphicsDesc, GraphicsPipeline, READS_BUF,
    };
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion,
    };
    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return None;
    }
    let ps_cb0_dxbc = cb0_color_ps_dxbc();
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
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
                    RootDescriptorKind::Srv,
                    RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                ),
                visibility: 1,
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
        label,
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
    Some((rs, Arc::new(pso_raw)))
}

#[test]
fn read_to_read_classification() {
    // Pure read-to-read transitions must classify as elidable.
    assert!(is_read_to_read_transition(STATE_PIXEL_SRV, STATE_NON_PIXEL_SRV));
    assert!(is_read_to_read_transition(STATE_NON_PIXEL_SRV, STATE_PIXEL_SRV | STATE_NON_PIXEL_SRV));
    assert!(is_read_to_read_transition(STATE_DEPTH_READ, STATE_DEPTH_READ | STATE_PIXEL_SRV));
    assert!(is_read_to_read_transition(STATE_COPY_SOURCE, STATE_GENERIC_READ));
    assert!(is_read_to_read_transition(STATE_INDIRECT_ARG, STATE_VB_CB | STATE_NON_PIXEL_SRV));

    // COMMON/PRESENT (0) and any write state (RTV, UAV, DSV write, CopyDest) MUST NOT be elided.
    assert!(!is_read_to_read_transition(STATE_COMMON, STATE_PIXEL_SRV));
    assert!(!is_read_to_read_transition(STATE_PIXEL_SRV, STATE_COMMON));
    assert!(!is_read_to_read_transition(STATE_RENDER_TARGET, STATE_PIXEL_SRV));
    assert!(!is_read_to_read_transition(STATE_PIXEL_SRV, STATE_RENDER_TARGET));
    assert!(!is_read_to_read_transition(STATE_UAV, STATE_NON_PIXEL_SRV));
    assert!(!is_read_to_read_transition(STATE_NON_PIXEL_SRV, STATE_UAV));
    assert!(!is_read_to_read_transition(STATE_DEPTH_WRITE, STATE_DEPTH_READ));
    assert!(!is_read_to_read_transition(STATE_DEPTH_READ, STATE_DEPTH_WRITE));
    assert!(!is_read_to_read_transition(STATE_COPY_DEST, STATE_GENERIC_READ));
    // Unknown reserved bits must stay conservative (false).
    assert!(!is_read_to_read_transition(0x8000_0000, STATE_PIXEL_SRV));
}

#[test]
fn r2r_elide_skips_read_transitions_and_preserves_write_hazards() {
    if crate::metal::device().is_none() {
        return;
    }
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    let tex = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_id = &tex as *const Resource as usize;
    let b0 = test_buffer(64, heap_type::UPLOAD, 0);
    let b1 = test_buffer(64, heap_type::DEFAULT, 0);

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    // Open a compute encoder (consumes the list-start barrier: enc_sync_after == All).
    l.copy_buffer(&b1, 0, &b0, 0, 16).unwrap();
    assert!(!l.bar.needs_barrier);
    assert!(l.bar.pend_after.is_empty());

    // Default off (`elide_r2r = false`): PIXEL_SRV -> NON_PIXEL_SRV sets pending Fragment -> soft PreRaster.
    l.barrier_list_with(&[Barrier::Transition(tex_id, STATE_PIXEL_SRV, STATE_NON_PIXEL_SRV)], false, false);
    assert_eq!(l.bar.pend_after, MTLStages::Fragment);
    assert_eq!(l.bar.pend_before_soft[0], PRE_RASTER);
    l.clear_all_pend();

    // Enabled (`elide_r2r = true`): pure read-to-read and no-op transitions leave pending stages empty
    // and do not record intra-compute obligations.
    l.barrier_list_with(
        &[
            Barrier::Transition(tex_id, STATE_PIXEL_SRV, STATE_NON_PIXEL_SRV),
            Barrier::Transition(tex_id, STATE_DEPTH_READ, STATE_DEPTH_READ),
        ],
        true,
        false,
    );
    assert!(l.bar.pend_after.is_empty());
    assert!(l.bar.pend_before.is_empty());
    assert_eq!(l.bar.pend_before_soft, [MTLStages::empty(); 2]);
    assert!(l.ib.ib_obl.is_empty());

    // Mixed batch (R2R + real RTV -> PIXEL_SRV write-to-read hazard): R2R is skipped, but RTV -> SRV
    // MUST emit Fragment -> Fragment stages!
    let other = test_texture(8, 8, RGBA8_UNORM, 0);
    let other_id = &other as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(other_id, STATE_NON_PIXEL_SRV, STATE_COPY_SOURCE),
            Barrier::Transition(tex_id, STATE_RENDER_TARGET, STATE_PIXEL_SRV),
        ],
        true,
        false,
    );
    assert_eq!(l.bar.pend_after, MTLStages::Fragment);
    assert_eq!(l.bar.pend_before, MTLStages::Fragment);

    // Guardrail: if `tex` has a deferred clear pending in `pending_clears`, even an R2R-looking
    // transition naming `tex` must NOT elide so the deferred clear flushes before readers!
    let rtv = TargetView::rtv(&tex, None).unwrap();
    unsafe { l.clear_rtv(&rtv, [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };
    assert_eq!(l.clears.pending_clears.len(), 1);
    l.barrier_list_with(&[Barrier::Transition(tex_id, STATE_PIXEL_SRV, STATE_COPY_SOURCE)], true, false);
    assert!(l.clears.pending_clears.is_empty(), "deferred clear on transitioned resource must flush");
    l.close().unwrap();
}

#[test]
fn r2r_elide_keeps_open_render_pass_even_for_attached_dsv_read_transitions() {
    if crate::metal::device().is_none() {
        return;
    }
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    let ds = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let ds_id = &ds as *const Resource as usize;
    // Read-only DSV (flags = DSV_READ_ONLY_DEPTH | DSV_READ_ONLY_STENCIL = 3).
    let dsv_ro = TargetView::dsv(&ds, Some(&[D32_S8, 3, 3, 0, 0, 0])).unwrap();

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.set_render_targets(Vec::new(), Some(dsv_ro));
    let _ = unsafe { l.draw_encoder().unwrap() };
    assert!(l.gfx.draw_pass);

    // Without R2R elision, transitioning the attached read-only DSV splits the draw pass (`draw_pass = false`).
    l.barrier_list_with(&[Barrier::Transition(ds_id, STATE_DEPTH_READ, STATE_DEPTH_READ | STATE_NON_PIXEL_SRV)], false, false);
    assert!(!l.gfx.draw_pass, "default path splits draw pass when attached DSV transitions");

    // Re-open draw pass; with R2R elision enabled, the read-to-read transition keeps the pass open!
    let _ = unsafe { l.draw_encoder().unwrap() };
    assert!(l.gfx.draw_pass);
    l.barrier_list_with(&[Barrier::Transition(ds_id, STATE_DEPTH_READ, STATE_DEPTH_READ | STATE_NON_PIXEL_SRV)], true, false);
    assert!(l.gfx.draw_pass, "R2R elision must keep the open draw pass alive");

    // A real write transition on the attached DSV (`DEPTH_READ -> DEPTH_WRITE`) MUST split the draw pass!
    l.barrier_list_with(&[Barrier::Transition(ds_id, STATE_DEPTH_READ, STATE_DEPTH_WRITE)], true, false);
    assert!(!l.gfx.draw_pass, "write transition on attached DSV must split the draw pass");
    l.close().unwrap();
}

#[test]
fn tight_stages_keeps_deferred_clears_across_null_uav_and_folds_into_draw_pass() {
    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "tight_stages test queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();

    let ds = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let rt_nows = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb_rt = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_d = test_buffer(256 * 4, heap_type::READBACK, 0);

    let dsv = TargetView::dsv(&ds, None).unwrap();
    let rtv = TargetView::rtv(&rt_nows, None).unwrap();

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    unsafe {
        l.clear_dsv(&dsv, true, 0.625, true, 0x42, &[]).unwrap();
        l.clear_rtv(&rtv, [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
    }
    assert_eq!(l.clears.pending_clears.len(), 2);

    // Null UAV barrier (`Barrier::Uav(0)`) with `tight_stages = true` keeps both deferred clears pending
    // instead of forcing `cp_dsv` + `cp_rtv_nows` standalone clear passes!
    l.barrier_list_with(&[Barrier::Uav(0)], false, true);
    assert_eq!(l.clears.pending_clears.len(), 2, "deferred RTV/DSV clears must survive null UAV barrier");

    // Opening the draw pass attaching `rtv` + `dsv` folds both deferred clears into MTLLoadAction::Clear!
    l.set_render_targets(vec![rtv], Some(dsv));
    let _ = unsafe { l.draw_encoder().unwrap() };
    assert!(l.clears.pending_clears.is_empty(), "deferred clears must fold into draw pass loadAction");

    // Transition to COPY_SOURCE and read back on the GPU to prove the folded clears executed properly.
    let rt_id = &rt_nows as *const Resource as usize;
    let ds_id = &ds as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    let fp_rt = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp_rt), [0, 0, 0], CopyLoc::Subresource(&rt_nows, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_d, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
    l.close().unwrap();
    q.submit(&[&l.cb]);
    q.wait_idle();

    let px = buf_words(&rb_rt, 4);
    assert_eq!(px[0], 0xff00_ff00);
    let d0 = f32::from_bits(buf_words(&rb_d, 1)[0]);
    assert!((d0 - 0.625).abs() < 1e-5, "folded depth clear mismatch: got {d0}");
}

#[test]
fn tight_stages_masked_upscale_and_uav_clear_preserve_gpu_hazards() {
    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "tight_stages util queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();

    let ua = resource_flags::ALLOW_UNORDERED_ACCESS;
    let buf = test_buffer(128, heap_type::DEFAULT, ua);
    let rb_buf = test_buffer(128, heap_type::READBACK, 0);
    let src_tex = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let dst_tex = test_texture(8, 8, RGBA8_UNORM, ua);
    let rb_tex = test_buffer(256 * 8, heap_type::READBACK, 0);

    let mut uav_all = [0u8; 40];
    uav_all[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_all[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_all[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_all[16..20].copy_from_slice(&32u32.to_le_bytes());

    let mut uav_mid = [0u8; 40];
    uav_mid[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_mid[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_mid[8..16].copy_from_slice(&8u64.to_le_bytes());
    uav_mid[16..20].copy_from_slice(&8u32.to_le_bytes());

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    // Back-to-back overlapping UAV clears on the same buffer without an explicit ResourceBarrier,
    // followed by a CopyBufferRegion: `tight_stages = true` must emit an intra-encoder barrier
    // on `buf` (`ib_note`) and NOT leave `needs_barrier = true` (`qb_clear_uav`).
    l.clear_uav_with(&buf, Some(&uav_all), ClearValue::Uint([0x1111_1111, 0, 0, 0]), &[], true).unwrap();
    assert!(!l.bar.needs_barrier, "tight_stages clear_uav must use stage-masked barriers instead of FullWhy::ClearUav");
    l.clear_uav_with(&buf, Some(&uav_mid), ClearValue::Uint([0x2222_2222, 0, 0, 0]), &[], true).unwrap();
    assert!(!l.bar.needs_barrier);
    l.copy_buffer(&rb_buf, 0, &buf, 0, 128).unwrap();

    // Deferred RTV clear on `src_tex` -> `upscale_resource_with(..., true)` -> readback `dst_tex`.
    let rtv_src = TargetView::rtv(&src_tex, None).unwrap();
    unsafe { l.clear_rtv(&rtv_src, [0.0, 0.0, 1.0, 1.0], &[]).unwrap() };
    l.upscale_resource_with(&dst_tex, &src_tex, [4, 4], [8, 8], true).unwrap();
    assert!(!l.bar.needs_barrier, "tight_stages upscale_resource must not set FullWhy::ClearUav");
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_tex, fp), [0, 0, 0], CopyLoc::Subresource(&dst_tex, 0), None).unwrap();

    l.close().unwrap();
    q.submit(&[&l.cb]);
    q.wait_idle();

    let bw = buf_words(&rb_buf, 32);
    for (i, &w) in bw.iter().enumerate() {
        let want = if (8..16).contains(&i) { 0x2222_2222 } else { 0x1111_1111 };
        assert_eq!(w, want, "word {i}");
    }
    let px = buf_words(&rb_tex, 64 * 8);
    for y in 0..8 {
        for x in 0..8 {
            assert_eq!(px[y * 64 + x], 0xffff_0000, "upscaled texel ({x},{y})");
        }
    }
}

#[test]
fn default_barrier_env_promotes_r2r_and_util_stage_masks_while_uav0_flushes_clears() {
    // Without env overrides, R2R elision and utility dispatch stage-masking are default ON,
    // while null-UAV clear deferral (`AW2_BARRIER_UAV0_KEEP`) stays default OFF.
    if std::env::var_os("AW2_BARRIER_R2R_ELIDE").is_none() {
        assert!(barrier_r2r_elide());
    }
    if std::env::var_os("AW2_BARRIER_TIGHT_STAGES").is_none() {
        assert!(barrier_tight_stages());
    }
    if std::env::var_os("AW2_BARRIER_UAV0_KEEP").is_none() {
        assert!(!barrier_uav0_keep());
    }
    if crate::metal::device().is_none()
        || std::env::var_os("AW2_BARRIER_R2R_ELIDE").is_some()
        || std::env::var_os("AW2_BARRIER_TIGHT_STAGES").is_some()
        || std::env::var_os("AW2_BARRIER_UAV0_KEEP").is_some()
    {
        return;
    }

    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    let ds = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let ds_id = &ds as *const Resource as usize;
    let dsv_ro = TargetView::dsv(&ds, Some(&[D32_S8, 3, 3, 0, 0, 0])).unwrap();
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let ua = resource_flags::ALLOW_UNORDERED_ACCESS;
    let buf = test_buffer(64, heap_type::DEFAULT, ua);
    let dst_tex = test_texture(8, 8, RGBA8_UNORM, ua);

    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_desc[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_desc[16..20].copy_from_slice(&16u32.to_le_bytes());

    l.begin(&alloc).unwrap();
    // 1) Default `barrier_list` elides pure read-to-read transitions and keeps an open draw pass alive.
    l.set_render_targets(Vec::new(), Some(dsv_ro));
    let _ = unsafe { l.draw_encoder().unwrap() };
    assert!(l.gfx.draw_pass);
    l.barrier_list(&[Barrier::Transition(ds_id, STATE_DEPTH_READ, STATE_DEPTH_READ | STATE_NON_PIXEL_SRV)]);
    assert!(l.gfx.draw_pass, "default barrier_list must elide R2R transitions on attached read-only DSV");
    l.end_encoder();

    // 2) Default `barrier_list` flushes deferred RTV clears on `Barrier::Uav(0)` (`uav0_keep = false`).
    unsafe { l.clear_rtv(&rtv, [0.25, 0.5, 0.75, 1.0], &[]).unwrap() };
    assert_eq!(l.clears.pending_clears.len(), 1);
    l.barrier_list(&[Barrier::Uav(0)]);
    assert!(l.clears.pending_clears.is_empty(), "default barrier_list must flush deferred clears on Barrier::Uav(0)");
    l.clear_all_pend();

    // 3) Default `clear_uav` and `upscale_resource` use stage-masked barriers (`!needs_barrier`).
    l.clear_uav(&buf, Some(&uav_desc), ClearValue::Uint([0xdead_beef, 0, 0, 0]), &[]).unwrap();
    assert!(!l.bar.needs_barrier, "default clear_uav must use stage-masked barriers");
    l.upscale_resource(&dst_tex, &rt, [4, 4], [8, 8]).unwrap();
    assert!(!l.bar.needs_barrier, "default upscale_resource must use stage-masked barriers");
    l.close().unwrap();
}

#[test]
fn passtiming_zero_draw_and_synth_keys_resolve_cleanly_on_gpu() {
    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "g30 passtiming test queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();

    let msaa_desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: RGBA8_UNORM,
        SampleCount: 4,
        Flags: resource_flags::ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    let msaa_src = Resource::new_committed(&msaa_desc, &test_heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "g30 msaa").unwrap();
    let rt_nows = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_sw = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let ds = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let dst_up = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_buf = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_buf = test_buffer(64, heap_type::READBACK, 0);

    let rtv_msaa = TargetView::rtv(&msaa_src, None).unwrap();
    let rtv_nows = TargetView::rtv(&rt_nows, None).unwrap();
    let rtv_sw = TargetView::rtv(&rt_sw, None).unwrap();
    let dsv = TargetView::dsv(&ds, None).unwrap();

    let mut uav_desc = [0u8; 40];
    uav_desc[0..4].copy_from_slice(&R32_UINT.to_le_bytes());
    uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes());
    uav_desc[8..16].copy_from_slice(&0u64.to_le_bytes());
    uav_desc[16..20].copy_from_slice(&16u32.to_le_bytes());

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.pt = pt::PassLog::acquire();
    assert!(l.pt.is_some(), "expected free counter-heap slice");

    // 1) Three 0-draw `CLEAR` render passes (4x MSAA RTV, non-ShaderWrite RTV, and DSV).
    unsafe {
        l.clear_rtv(&rtv_msaa, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
        l.clear_rtv(&rtv_nows, [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
        l.clear_dsv(&dsv, true, 0.5, true, 0x7, &[]).unwrap();
    }
    let msaa_id = &msaa_src as *const Resource as usize;
    let rt_nows_id = &rt_nows as *const Resource as usize;
    let ds_id = &ds as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(msaa_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(rt_nows_id, STATE_RENDER_TARGET, STATE_COPY_DEST),
            Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_DEPTH_READ),
        ],
        true,
        true,
    );

    // 2) 0-draw `UTIL` render pass (`resolve_subresource` 4x -> 1x).
    l.resolve_subresource(&rt_nows, 0, &msaa_src, 0, RGBA8_UNORM).unwrap();

    // 3) 0-draw `DRAW` render pass (opens with Vertex timestamp, closes with 0 draws + Fragment timestamp).
    l.set_render_targets(vec![rtv_nows], None);
    let _ = unsafe { l.draw_encoder().unwrap() };
    l.end_encoder();

    // 4) `compute_clear_batch` (`ShaderWrite` RTV clear) -> `CLEAR_CS`.
    unsafe { l.clear_rtv(&rtv_sw, [0.0, 0.0, 1.0, 1.0], &[]).unwrap() };
    let rt_sw_id = &rt_sw as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(rt_sw_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, true);
    l.end_encoder();

    // 5) Standalone `clear_uav` -> `CLEAR_UAV`.
    l.clear_uav(&uav_buf, Some(&uav_desc), ClearValue::Uint([0xabcd_1234, 0, 0, 0]), &[]).unwrap();
    l.end_encoder();

    // 6) Standalone `upscale_resource` -> `UPSCALE`.
    l.upscale_resource(&dst_up, &rt_sw, [4, 4], [8, 8]).unwrap();
    l.end_encoder();

    // 7) Standalone `copy_buffer` -> `COPY_BLIT`.
    l.copy_buffer(&rb_buf, 0, &uav_buf, 0, 64).unwrap();
    l.close().unwrap();

    let log = l.take_pass_log().expect("closed list must yield PassLog");
    q.submit(&[&l.cb]);
    q.wait_idle();

    let summary = pt::resolve_for_test(vec![log]);
    assert_eq!(summary.bad, 0, "0-draw TBDR passes must resolve with bad == 0; raw_ts={:?}", summary.raw_ts);
    let want_clear_passes = if l.opts.opt_uav0_clear_overlap { 2 } else { 3 };
    assert_eq!(summary.kind_n[pt::kind::CLEAR as usize], want_clear_passes);
    assert_eq!(summary.kind_n[pt::kind::UTIL as usize], 1);
    assert_eq!(summary.kind_n[pt::kind::DRAW as usize], 1);
    assert_eq!(summary.kind_n[pt::kind::COMPUTE as usize], 4);

    for &(_, key, _, _) in &summary.shaders {
        // Only the empty 0-draw `DRAW` pass (which never bound a graphics PSO) has key 0;
        // every clear/util/compute encoder must carry its `aw2:*` synthetic key!
        if key != 0 {
            assert!(pt::synth::name(key).is_some(), "expected synthetic tag for {key:#x}");
        }
    }
    for &(k, _, _, _) in &summary.shaders {
        if k != pt::kind::DRAW {
            assert!(summary.shaders.iter().filter(|&&(kk, _, _, _)| kk == k).all(|&(_, key, _, _)| key != 0));
        }
    }
    for want in [
        pt::synth::CLEAR_PASS,
        pt::synth::RESOLVE,
        pt::synth::CLEAR_CS,
        pt::synth::CLEAR_UAV,
        pt::synth::UPSCALE,
        pt::synth::COPY_BLIT,
    ] {
        assert!(
            summary.shaders.iter().any(|&(_, key, n, _)| key == want && n >= 1),
            "missing synthetic key {:016x} ({:?}) in {:?}",
            want,
            pt::synth::name(want),
            summary.shaders
        );
    }
    assert_eq!(buf_words(&rb_buf, 1)[0], 0xabcd_1234);
}

#[test]
fn ib_tight_skips_disjoint_barriers_and_orders_true_raw_war_waw_and_blit_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputeBind, ComputePipeline};
    use crate::metal::views;
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    const RGBA32_FLOAT: u32 = 2;
    let cs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/static_samplers.cs.dxil"));

    let clamp_point = |reg: u32| StaticSampler {
        filter: 0,     // MIN_MAG_MIP_POINT
        address_u: 3,  // CLAMP
        address_v: 3,
        address_w: 3,
        shader_register: reg,
        ..StaticSampler::default()
    };

    // 1) Bounded SRV table (`t0`) + root UAV (`u0`).
    let rs_bounded = RootSignature::new(RootSignatureDesc {
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

    let pso_bounded = Arc::new(ComputePipeline::new(rs_bounded.clone(), &cs_dxil, "g30_cs_bounded").unwrap());
    assert_eq!(pso_bounded.access.uav_slots, Some(vec![ComputeBind::RootVa { param: 1 }]));
    assert_eq!(pso_bounded.access.read_slots, Some(vec![ComputeBind::TableEntry { param: 0, byte_offset: 0 }]));
    assert!(!pso_bounded.access.heap_indexed_or_unknown);

    // 2) `AW2_CS_PSO_TG` (`pin_tg = true`) with table-bound SRV (`t0`) + table-bound UAV (`u0`).
    let rs_table = RootSignature::new(RootSignatureDesc {
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
    let pso_pin_tg = Arc::new(ComputePipeline::new_with_opts(rs_table.clone(), &cs_dxil, "g30_cs_pin_tg", true).unwrap());
    assert_eq!(pso_pin_tg.threadgroup, [16, 1, 1]);

    let q = Queue::new(ListType::Direct, "g30 ib_tight queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_ib_tight = true;
    l.opts.opt_ib_bindless_war = false;

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g30 test heap").unwrap();
    let write_entry = |slot: usize, entry: &[u8; 24]| unsafe {
        std::ptr::copy_nonoverlapping(entry.as_ptr(), (heap.cpu_base + slot * 24) as *mut u8, 24);
    };

    // Upload buffer with two 4x1 RGBA32Float rows: row 0 = [2, 3, 4, 1], row 1 = [7, 8, 9, 1].
    let up = test_buffer(512, heap_type::UPLOAD, 0);
    unsafe {
        let p = up.buffer().unwrap().contents().as_ptr() as *mut f32;
        for x in 0..4 {
            let b0 = x * 4;
            *p.add(b0) = 2.0;
            *p.add(b0 + 1) = 3.0;
            *p.add(b0 + 2) = 4.0;
            *p.add(b0 + 3) = 1.0;
            let b1 = 64 + x * 4; // 256-byte row offset
            *p.add(b1) = 7.0;
            *p.add(b1 + 1) = 8.0;
            *p.add(b1 + 2) = 9.0;
            *p.add(b1 + 3) = 1.0;
        }
    }

    let tex_a = test_texture(4, 1, RGBA32_FLOAT, 0);
    let tex_b = test_texture(4, 1, RGBA32_FLOAT, 0);
    let tex_clear = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_unused = test_texture(4, 1, RGBA32_FLOAT, 0);
    let out_a = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let out_b = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let out_c = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_a = test_buffer(512, heap_type::READBACK, 0);
    let rb_b = test_buffer(512, heap_type::READBACK, 0);
    let rb_c = test_buffer(512, heap_type::READBACK, 0);
    for r in [&up, &tex_a, &tex_b, &tex_clear, &tex_unused, &out_a, &out_b, &out_c, &rb_a, &rb_b, &rb_c] {
        r.register_ptr();
    }

    write_entry(0, &views::srv_entry(Some(&tex_a), None).unwrap());
    write_entry(1, &views::srv_entry(Some(&tex_b), None).unwrap());
    write_entry(2, &views::srv_entry(Some(&tex_clear), None).unwrap());
    let mut uav_struct = [0u8; 40];
    uav_struct[4..8].copy_from_slice(&1u32.to_le_bytes()); // BUFFER
    uav_struct[16..20].copy_from_slice(&32u32.to_le_bytes()); // NumElements = 32
    uav_struct[20..24].copy_from_slice(&16u32.to_le_bytes()); // StructureByteStride = 16
    write_entry(3, &views::uav_entry(Some(&out_b), Some(&uav_struct)).unwrap());

    let fp0 = PlacedFootprint { offset: 0, format: RGBA32_FLOAT, width: 4, height: 1, depth: 1, row_pitch: 256, _pad: 0 };
    let fp1 = PlacedFootprint { offset: 256, format: RGBA32_FLOAT, width: 4, height: 1, depth: 1, row_pitch: 256, _pad: 0 };
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_b_id = &tex_b as *const Resource as usize;
    let tex_clear_id = &tex_clear as *const Resource as usize;
    let tex_unused_id = &tex_unused as *const Resource as usize;
    let out_a_id = &out_a as *const Resource as usize;
    let out_b_id = &out_b as *const Resource as usize;
    let out_c_id = &out_c as *const Resource as usize;

    let _stats_guard = gpu_stats_lock();
    l.opts.opt_ib_substage_narrow = false;
    l.begin(&alloc).unwrap();
    l.set_descriptor_heaps(heap.gpu_base, 0);

    // A) Two back-to-back `CopyTextureRegion` uploads (`tex_a`, `tex_b`) must NOT set
    // `produced(Blit, All)` or fire an intra-encoder barrier between them!
    let enc_b0 = stats::c::enc_barrier.load(Relaxed);
    l.copy_texture_region(CopyLoc::Subresource(&tex_a, 0), [0, 0, 0], CopyLoc::Footprint(&up, fp0), None).unwrap();
    l.copy_texture_region(CopyLoc::Subresource(&tex_b, 0), [0, 0, 0], CopyLoc::Footprint(&up, fp1), None).unwrap();
    assert_eq!(stats::c::enc_barrier.load(Relaxed), enc_b0, "back-to-back CopyTextureRegion uploads must emit 0 intra-encoder barriers");

    // B) Transition ONLY `tex_unused` (COPY_DEST -> NON_PIXEL_SRV): since `tex_unused` was never
    // touched in this encoder, `ib_obl_tight` stays empty!
    l.barrier_list_with(&[Barrier::Transition(tex_unused_id, STATE_COPY_DEST, STATE_NON_PIXEL_SRV)], true, true);
    assert!(l.ib.ib_obl_tight.is_empty(), "untouched resource transition must not add intra-compute obligation");

    // C) Transition `tex_b` (COPY_DEST -> NON_PIXEL_SRV), then dispatch reading `tex_a`? Wait:
    // first transition `tex_a` AND `tex_b` one by one to prove disjoint skip vs true RAW!
    l.barrier_list_with(
        &[
            Barrier::Transition(tex_a_id, STATE_COPY_DEST, STATE_NON_PIXEL_SRV),
            Barrier::Transition(tex_b_id, STATE_COPY_DEST, STATE_NON_PIXEL_SRV),
        ],
        true,
        true,
    );
    assert_eq!(l.ib.ib_obl_tight.len(), 2, "both blit-uploaded textures must record (Blit -> Dispatch) obligations");

    // Dispatch 1 (`pso_bounded`): reads `tex_a` (slot 0), writes `out_a` (root UAV).
    // Must fire `Blit -> Dispatch` (`ib_masked`) and consume `tex_a`'s obligation while leaving `tex_b`'s!
    let ib_masked0 = stats::c::ib_masked.load(Relaxed);
    l.compute_root.set_root_signature(rs_bounded.clone()).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, out_a.gpu_va).unwrap();
    l.set_compute_pipeline(pso_bounded.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::ib_masked.load(Relaxed) > ib_masked0, "Blit -> Dispatch RAW must emit stage-masked intra-encoder barrier");
    // Notice: `Blit -> Dispatch` satisfied BOTH `tex_a` and `tex_b` because both had `after=Blit, before=Dispatch`!
    assert!(l.ib.ib_obl_tight.is_empty(), "Blit -> Dispatch barrier satisfies all pending Blit -> Dispatch obligations");

    // D) Disjoint `Barrier::Uav(out_a_id)` before Dispatch 2 (`pso_pin_tg` reading `tex_b`, writing `out_b`):
    // `out_a` was written by Dispatch 1, so `Uav(out_a)` adds `(out_a, Dispatch -> Dispatch)`.
    // Dispatch 2 only touches `{tex_b, out_b}`, so `compute_op_tight` MUST skip the barrier (`ib_skip_touch`)
    // AND keep `(out_a, Dispatch -> Dispatch)` pending in `ib_obl_tight`!
    l.barrier_list_with(&[Barrier::Uav(out_a_id)], true, true);
    assert_eq!(l.ib.ib_obl_tight.len(), 1);
    let skip_touch0 = stats::c::ib_skip_touch.load(Relaxed);
    let enc_b1 = stats::c::enc_barrier.load(Relaxed);
    l.compute_root.set_root_signature(rs_table.clone()).unwrap();
    l.compute_root.set_table(0, heap.gpu_base + 24).unwrap(); // slot 1 = tex_b
    l.compute_root.set_table(1, heap.gpu_base + 3 * 24).unwrap(); // slot 3 = out_b
    l.set_compute_pipeline(pso_pin_tg.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::ib_skip_touch.load(Relaxed) > skip_touch0, "disjoint Dispatch must increment ib_skip_touch");
    assert_eq!(stats::c::enc_barrier.load(Relaxed), enc_b1, "disjoint Dispatch must emit 0 enc_barrier");
    assert_eq!(l.ib.ib_obl_tight.len(), 1, "unmatched obligation on out_a must remain pending for future consumers");

    // E) `Blit` consumer (`CopyBufferRegion` reading `out_a`): `(out_a, Dispatch -> Dispatch)` does not
    // match `Blit`, so `ib_skip_stage` fires; then `Transition(out_a, UAV, COPY_SOURCE)` adds
    // `(out_a, Dispatch -> Blit)` which DOES fire `Dispatch -> Blit` on `copy_buffer`!
    l.barrier_list_with(&[Barrier::Transition(out_a_id, STATE_UAV, STATE_COPY_SOURCE)], true, true);
    l.barrier_list_with(&[Barrier::Transition(out_b_id, STATE_UAV, STATE_COPY_SOURCE)], true, true);
    let ib_masked1 = stats::c::ib_masked.load(Relaxed);
    l.copy_buffer(&rb_a, 0, &out_a, 0, 512).unwrap();
    assert!(stats::c::ib_masked.load(Relaxed) > ib_masked1, "UAV -> COPY_SOURCE must fire Dispatch -> Blit barrier");
    l.copy_buffer(&rb_b, 0, &out_b, 0, 512).unwrap();

    // F) `compute_clear_batch` (`tex_clear` cleared to `[5, 6, 10, 1]`) -> `Transition(RTV -> NON_PIXEL_SRV)`
    // -> Unbounded-SRV PSO profile (`read_slots = None`, `bounded_read_slots = [t0]`, `!heap_indexed_or_unknown`):
    // Even with `opt_ib_bindless_war = true`, `op_exact` MUST be `false` so incoming `CLEAR_CS -> Dispatch`
    // RAW barrier fires (`ib_masked`), AND outgoing `ib_disp_read` stays bounded (`Some`) only when
    // `opt_ib_bindless_war == true`!
    let rtv_clear = TargetView::rtv(&tex_clear, None).unwrap();
    unsafe { l.clear_rtv(&rtv_clear, [5.0, 6.0, 10.0, 1.0], &[]).unwrap() };
    l.barrier_list_with(&[Barrier::Transition(tex_clear_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, true);
    assert!(!l.ib.ib_obl_tight.is_empty(), "compute_clear_batch must record per-resource Dispatch obligation");

    let pso_unbounded_srv = Arc::new(ComputePipeline {
        pso: pso_bounded.pso.clone(),
        rs: pso_bounded.rs.clone(),
        threadgroup: pso_bounded.threadgroup,
        shader: pso_bounded.shader.clone(),
        hk_profile: pso_bounded.hk_profile,
        access: ComputeAccess {
            uav_slots: pso_bounded.access.uav_slots.clone(),
            read_slots: None,
            bounded_read_slots: pso_bounded.access.bounded_read_slots.clone(),
            unbounded_srv_classes: crate::metal::pipeline::READS_TEX,
            heap_indexed_or_unknown: false,
            touches_textures: true,
            ..ComputeAccess::opaque()
        },
    });

    // First with `opt_ib_bindless_war = true`: incoming RAW barrier MUST still fire (`op_exact == false`),
    // while `ib_disp_read` stays `Some` (bounded)!
    l.opts.opt_ib_bindless_war = true;
    let ib_masked2 = stats::c::ib_masked.load(Relaxed);
    l.compute_root.set_root_signature(rs_bounded.clone()).unwrap();
    l.compute_root.set_table(0, heap.gpu_base + 2 * 24).unwrap(); // slot 2 = tex_clear
    l.compute_root.set_descriptor(1, out_c.gpu_va).unwrap();
    l.set_compute_pipeline(pso_unbounded_srv.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::ib_masked.load(Relaxed) > ib_masked2, "unbounded-SRV shader must NEVER miss incoming RAW barrier");
    assert!(l.ib.ib_disp_read.is_exact(), "opt_ib_bindless_war=true must keep ib_disp_read exact for bounded slots");

    // Contrast with `opt_ib_bindless_war = false`: `ib_disp_read` conservatively poisons to `None`.
    l.opts.opt_ib_bindless_war = false;
    l.barrier_list_with(&[Barrier::Uav(out_c_id)], true, true);
    l.dispatch(1, 1, 1).unwrap();
    assert!(!l.ib.ib_disp_read.is_exact(), "opt_ib_bindless_war=false must poison ib_disp_read to None on unbounded SRV");

    l.barrier_list_with(&[Barrier::Transition(out_c_id, STATE_UAV, STATE_COPY_SOURCE)], true, true);
    l.copy_buffer(&rb_c, 0, &out_c, 0, 512).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    let fa: Vec<f32> = buf_words(&rb_a, 128).into_iter().map(f32::from_bits).collect();
    let fb: Vec<f32> = buf_words(&rb_b, 128).into_iter().map(f32::from_bits).collect();
    let fc: Vec<f32> = buf_words(&rb_c, 128).into_iter().map(f32::from_bits).collect();
    assert_eq!(&fa[0..4], &[2.0, 3.0, 4.0, 1.0]);
    assert_eq!(&fb[0..4], &[7.0, 8.0, 9.0, 1.0]);
    assert_eq!(&fc[0..4], &[5.0, 6.0, 10.0, 1.0]);
}

#[test]
fn rp_keep_write_and_smart_uav0_keep_pass_and_preserve_war_and_uav_splits_on_gpu() {
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState, RenderTargetBlendDesc};
    use dxil_metal::root_signature::{flags as rs_flags, RootSignature, RootSignatureDesc, RootSignatureVersion};
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let rs_empty = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: Vec::new(),
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();

    let mut blend = BlendDesc::default();
    blend.RenderTarget[0] = RenderTargetBlendDesc { RenderTargetWriteMask: 0xf, ..Default::default() };
    let gdesc = GraphicsDesc {
        vs: &vs_dxil,
        ps: None,
        blend,
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
        depth_stencil: DepthStencilState { depth_compare: MTLCompareFunction::Less, depth_write: true, ..DepthStencilState::default() },
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: 3, // TRIANGLE
        rtv_formats: vec![RGBA8_UNORM],
        dsv_format: D32_S8,
        sample_count: 1,
    };
    let pso_no_uav = Arc::new(GraphicsPipeline::new(rs_empty.clone(), &gdesc, "g30_vs_depth").unwrap());
    assert!(!pso_no_uav.writes_uav);
    let pso_writes_uav = Arc::new(GraphicsPipeline {
        pso: pso_no_uav.pso.clone(),
        rs: pso_no_uav.rs.clone(),
        raster: pso_no_uav.raster,
        blend: pso_no_uav.blend,
        depth_stencil: pso_no_uav.depth_stencil.clone(),
        depth_bounds: pso_no_uav.depth_bounds,
        needs_draw_params: pso_no_uav.needs_draw_params,
        topology_type: pso_no_uav.topology_type,
        strip_cut: pso_no_uav.strip_cut,
        rtv_formats: pso_no_uav.rtv_formats.clone(),
        dsv_format: pso_no_uav.dsv_format,
        sample_count: pso_no_uav.sample_count,
        vertex_slots: pso_no_uav.vertex_slots,
        vs: pso_no_uav.vs.clone(),
        ps: pso_no_uav.ps.clone(),
        gs_emu: None,
        tess_emu: None,
        pre_raster_reads: pso_no_uav.pre_raster_reads,
        vs_reads_cbv_or_ia: pso_no_uav.vs_reads_cbv_or_ia,
        vs_writes_uav: false,
        ps_writes_uav: true,
        ps_read_classes: pso_no_uav.ps_read_classes,
        ps_srv_uav_classes: pso_no_uav.ps_srv_uav_classes,
        writes_uav: true,
        uses_uav: true,
        sm66_pre_raster_reads: pso_no_uav.sm66_pre_raster_reads,
        sm66_vs_reads_cbv_or_ia: pso_no_uav.sm66_vs_reads_cbv_or_ia,
        sm66_writes_uav: true,
        sm66_narrowed: false,
        vs_access: pso_no_uav.vs_access.clone(),
        ps_access: pso_no_uav.ps_access.clone(),
        vs_hk_profile: pso_no_uav.vs_hk_profile,
        ps_hk_profile: pso_no_uav.ps_hk_profile,
        icb_desc: None,
        icb_pso: std::sync::OnceLock::new(),
    });

    let q = Queue::new(ListType::Direct, "g30 rp_keep_write queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_clear_keep_smart = true;
    l.opts.opt_rp_keep_write = true;

    let rt_sw = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let ds = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let other_rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let other_uav = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_rt = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_d = test_buffer(256 * 4, heap_type::READBACK, 0);

    let rtv_sw = TargetView::rtv(&rt_sw, None).unwrap();
    let dsv = TargetView::dsv(&ds, None).unwrap();
    let other_rt_id = &other_rt as *const Resource as usize;
    let other_uav_id = &other_uav as *const Resource as usize;

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();

    // 1) `opt_clear_keep_smart`: at `Barrier::Uav(0)`, flush DSV clear outside render pass
    // while retaining `ShaderWrite` colour RTV clear (`cf_smart_keep`) to fold into the draw pass!
    unsafe {
        l.clear_dsv(&dsv, true, 0.75, true, 0x33, &[]).unwrap();
        l.clear_rtv(&rtv_sw, [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
    }
    assert_eq!(l.clears.pending_clears.len(), 2);
    let smart0 = stats::c::cf_smart_keep.load(Relaxed);
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(stats::c::cf_smart_keep.load(Relaxed) > smart0, "ShaderWrite RTV clear must increment cf_smart_keep");
    assert_eq!(l.clears.pending_clears.len(), 1, "DSV clear must flush while ShaderWrite RTV clear stays pending");
    assert!(matches!(l.clears.pending_clears[0].kind, ClearKind::Color(_)));

    // 2) Open draw pass with `pso_no_uav` (`!writes_uav`): folds `rt_sw` green clear and draws
    // fullscreen triangle at `z = 0.0` overwriting depth `0.75 -> 0.0`.
    l.gfx_root.set_root_signature(rs_empty).unwrap();
    l.set_render_targets(vec![rtv_sw], Some(dsv));
    l.gfx.topology = 4; // D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST
    l.set_graphics_pipeline(pso_no_uav);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(l.clears.pending_clears.is_empty(), "smart-kept RTV clear must fold into draw pass loadAction");
    assert!(l.gfx.draw_pass);
    assert!(!l.gfx.pass_uav);

    // 3) `opt_rp_keep_write`: unattached write transition `PIXEL_SRV -> RENDER_TARGET` keeps
    // the draw pass open, does NOT pollute `pend_after` (`0` spurious `rb_mid_qb`), and records
    // WAR stages `(Fragment -> Fragment)` in `(rp_write_after, rp_write_before)`.
    let keep_w0 = stats::c::rb_keep_write.load(Relaxed);
    let mid_qb0 = stats::c::rb_mid_qb.load(Relaxed);
    let pre_pend = l.bar.pend_after;
    let pre_split = l.bar.split_pend;
    l.barrier_list_with(&[Barrier::Transition(other_rt_id, STATE_PIXEL_SRV, STATE_RENDER_TARGET)], true, false);
    assert!(l.gfx.draw_pass, "unattached RTV transition must keep draw pass open");
    assert!(stats::c::rb_keep_write.load(Relaxed) > keep_w0);
    assert!(!l.gfx.pass_uav_hazard);
    assert!(
        l.bar.split_pend.vtx_after.is_empty()
            && l.bar.split_pend.frag_after.is_empty()
            && l.bar.split_pend == pre_split
            && l.bar.pend_after == pre_pend,
        "kept write transition must defer WAR stages until end_encoder"
    );
    assert_eq!(l.rp_sync.rp_write_after, MTLStages::Fragment);
    assert_eq!(l.rp_sync.rp_write_before, MTLStages::Fragment);

    // 4) `Barrier::Uav(0)` and unattached `PIXEL_SRV -> UAV` keep the pass open while `!pass_uav`
    // and arm `pass_uav_hazard = true`.
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(l.gfx.draw_pass);
    assert!(l.gfx.pass_uav_hazard);
    l.barrier_list_with(&[Barrier::Transition(other_uav_id, STATE_PIXEL_SRV, STATE_UAV)], true, false);
    assert!(l.gfx.draw_pass);
    assert!(l.rp_sync.rp_write_before.contains(MTLStages::Dispatch));
    assert_eq!(stats::c::rb_mid_qb.load(Relaxed), mid_qb0, "kept write/UAV barriers must not trigger rb_mid_qb");

    // 5) Drawing with `pso_writes_uav` (`writes_uav = true`) while `pass_uav_hazard == true`
    // MUST split the pass first (`rb_uav_split`), flushing `rp_write_after/before` cleanly!
    let uav_split0 = stats::c::rb_uav_split.load(Relaxed);
    l.set_graphics_pipeline(pso_writes_uav);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(stats::c::rb_uav_split.load(Relaxed) > uav_split0, "UAV-writing draw after kept UAV hazard must split pass");
    assert!(l.gfx.draw_pass);
    assert!(l.gfx.pass_uav);
    assert!(!l.gfx.pass_uav_hazard);
    assert!(l.rp_sync.rp_write_after.is_empty(), "end_encoder on rb_uav_split must flush rp_write_after");

    // Once `pass_uav == true`, a subsequent `Barrier::Uav(0)` MUST split the render pass!
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(!l.gfx.draw_pass, "Barrier::Uav while pass_uav == true must split draw pass");

    // Read back colour + depth on the GPU.
    let rt_sw_id = &rt_sw as *const Resource as usize;
    let ds_id = &ds as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(rt_sw_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    let fp_rt = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp_rt), [0, 0, 0], CopyLoc::Subresource(&rt_sw, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_d, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    let px = buf_words(&rb_rt, 4);
    assert_eq!(px[0], 0xff00_ff00, "folded smart-kept RTV clear must produce green");
    let d0 = f32::from_bits(buf_words(&rb_d, 1)[0]);
    assert!(d0.abs() < 1e-6, "fullscreen triangle draw at z=0 must overwrite 0.75 depth clear: got {d0}");
}

#[test]
fn g35_defaults_promote_g34_winners_and_keep_g35_knobs_opt_in() {
    // Promoted g30 + g31 + g32 + g33 + g34 winners MUST default to ON when unset.
    if std::env::var_os("AW2_IB_TIGHT").is_none() {
        assert!(opt_ib_tight_env());
    }
    if std::env::var_os("AW2_IB_BINDLESS_WAR").is_none() {
        assert!(!opt_ib_bindless_war_env());
    }
    if std::env::var_os("AW2_RP_KEEP_WRITE").is_none() {
        assert!(opt_rp_keep_write_env());
    }
    if std::env::var_os("AW2_CLEAR_KEEP_SMART").is_none() {
        assert!(opt_clear_keep_smart_env());
    }
    if std::env::var_os("AW2_CS_PSO_TG").is_none() {
        assert!(crate::metal::pipeline::cs_pso_tg());
    }
    if std::env::var_os("AW2_ICB_BATCH").is_none() {
        assert!(opt_icb_batch_env());
    }
    if std::env::var_os("AW2_IB_EXACT_OBL").is_none() {
        assert!(opt_ib_exact_obl_env());
    }
    if std::env::var_os("AW2_GFX_LAZY_ICB").is_none() {
        assert!(crate::metal::pipeline::gfx_lazy_icb());
    }
    if std::env::var_os("AW2_FAST_BIND_PSO").is_none() {
        assert!(opt_fast_bind_pso_env());
        assert!(crate::metal::pipeline::fast_bind_pso());
    }
    if std::env::var_os("AW2_PRESENT_OFF_MAIN").is_none() {
        assert!(crate::metal::swapchain::present_off_main());
    }
    if std::env::var_os("AW2_UAV0_CLEAR_OVERLAP").is_none() {
        assert!(opt_uav0_clear_overlap_env());
    }
    if std::env::var_os("AW2_SHADER_IMMUTABLE_OPT").is_none() {
        assert!(crate::metal::pipeline::shader_immutable_opt());
    }
    if std::env::var_os("AW2_FAST_ENC_STATE").is_none() {
        assert!(opt_fast_enc_state_env());
    }
    if std::env::var_os("AW2_CLEAR_OVERLAP_EXACT").is_none() {
        assert!(opt_clear_overlap_exact_env());
    }
    if std::env::var_os("AW2_SUBRANGE_UAV_BARRIER").is_none() {
        assert!(opt_subrange_uav_barrier_env());
    }
    // Non-promoted g31/g32/g33/g34 knobs MUST remain Default-OFF.
    if std::env::var_os("AW2_CLEAR_KEEP_DSV").is_none() {
        assert!(!opt_clear_keep_dsv_env());
    }
    if std::env::var_os("AW2_LIST_BARRIER_TIGHT").is_none() {
        assert!(!opt_list_barrier_tight_env());
    }
    if std::env::var_os("AW2_BARRIER_TIGHT_V2").is_none() {
        assert!(!opt_barrier_tight_v2_env());
    }
    if std::env::var_os("PORT_EMPTY_ENC_ELIDE").is_none() && std::env::var_os("AW2_EMPTY_ENC_ELIDE").is_none() {
        assert!(opt_empty_enc_elide_env());
    }
    // Promoted g35 winners (`AW2_SUBMIT_COALESCE`, `AW2_COPY_QUEUE_LIGHT`) MUST default ON.
    if std::env::var_os("AW2_SUBMIT_COALESCE").is_none() {
        assert!(crate::metal::queue::submit_coalesce());
    }
    if std::env::var_os("AW2_COPY_QUEUE_LIGHT").is_none() {
        assert!(opt_copy_queue_light_env());
    }
    // Non-promoted g35 knobs MUST remain Default-OFF.
    if std::env::var_os("AW2_STAGESYNC_BARRIER_V2").is_none() {
        assert!(!opt_stagesync_barrier_v2_env());
    }
    if std::env::var_os("AW2_SHADER_PSO_OPT_V2").is_none() {
        assert!(!crate::metal::pipeline::shader_pso_opt_v2());
    }
    // Promoted winners MUST default ON; non-promoted knobs stay OFF.
    if std::env::var_os("AW2_CLEAR_CS_EXACT_SYNC").is_none() {
        assert!(opt_clear_cs_exact_sync_env());
    }
    if std::env::var_os("AW2_MSC_COMPUTE_ACCESS").is_none() {
        assert!(!crate::metal::pipeline::msc_compute_access());
    }
    if std::env::var_os("PORT_PRESENT_BLIT_NARROW").is_none() && std::env::var_os("AW2_PRESENT_BLIT_NARROW").is_none() {
        assert!(crate::metal::swapchain::present_blit_narrow());
    }
    if std::env::var_os("PORT_ICB_FAST_V3").is_none() && std::env::var_os("AW2_ICB_FAST_V3").is_none() {
        assert!(crate::metal::indirect_icb::fast_v3());
    }
    // g37 non-promoted knob remains default OFF (`== "1"`).
    if std::env::var_os("AW2_FENCE_SIGNAL_COALESCE").is_none() {
        assert!(!crate::metal::queue::fence_signal_coalesce());
    }
    if std::env::var_os("PORT_RO_DSV_DONTCARE").is_none() && std::env::var_os("AW2_RO_DSV_DONTCARE").is_none() {
        assert!(opt_ro_dsv_dontcare_env());
    }
    // g39_D promoted knobs MUST default to ON (`!= "0"`).
    if std::env::var_os("AW2_RENDER_SPLIT_STAGE_SYNC").is_none() {
        assert!(opt_render_split_stage_sync_env());
    }
    if std::env::var_os("AW2_MID_QB_STAGESYNC").is_none() {
        assert!(opt_mid_qb_stagesync_env());
    }
    if std::env::var_os("AW2_SPLIT_RES_SOFT_SYNC").is_none() {
        assert!(opt_split_res_soft_sync_env());
    }
    if std::env::var_os("AW2_CS_UAV_PRODUCER_EXACT").is_none() {
        assert!(opt_cs_uav_producer_exact_env());
    }
    // Promoted g40 winners (`AW2_SPLIT_COMPUTE_EXACT`, `AW2_RENDER_DOM_EXACT`) MUST default ON (`!= "0"`),
    // while non-promoted g40 knobs (`AW2_MSC_STAGE_USAGE`, `AW2_AIR_NO_RES_HEAP`) remain OFF (`== "1"`).
    if std::env::var_os("AW2_SPLIT_COMPUTE_EXACT").is_none() {
        assert!(opt_split_compute_exact_env());
    }
    if std::env::var_os("AW2_MSC_STAGE_USAGE").is_none() {
        assert!(!crate::metal::pipeline::msc_stage_usage());
    }
    if std::env::var_os("AW2_RENDER_DOM_EXACT").is_none() {
        assert!(opt_render_dom_exact_env());
    }
    if std::env::var_os("AW2_AIR_NO_RES_HEAP").is_none() {
        assert!(!crate::metal::pipeline::air_no_res_heap());
    }
    // g41 proven winners (`g41_A`, `g41_C`) promoted to Default-ON; `g41_B`/`g41_D` stay OFF.
    if std::env::var_os("AW2_VS_BUF_DEFER_EXACT").is_none() {
        assert!(opt_vs_buf_defer_exact_env());
    }
    if std::env::var_os("AW2_CS_MID_QB_TOUCH_DEFER").is_none() {
        assert!(!opt_cs_mid_qb_touch_defer_env());
    }
    if std::env::var_os("AW2_EI_DISPATCH_EXACT").is_none() {
        assert!(opt_ei_dispatch_exact_env());
    }
    if std::env::var_os("AW2_BLIT_LIST_NARROW_V2").is_none() {
        assert!(!opt_blit_list_narrow_v2_env());
    }
    // g42 proven winners (`g42_A`, `g42_B`, `g42_D`) promoted to Default-ON; `g42_C` stays OFF.
    if std::env::var_os("AW2_LIST_START_SUBSTAGE").is_none() {
        assert!(opt_list_start_substage_env());
    }
    if std::env::var_os("AW2_SM66_GFX_STAGE_EXACT").is_none() {
        assert!(crate::metal::pipeline::sm66_gfx_stage_exact());
    }
    if std::env::var_os("AW2_CS_MID_QB_EXACT_V2").is_none() {
        assert!(!crate::metal::pipeline::cs_mid_qb_exact_v2());
    }
    if std::env::var_os("AW2_IB_SUBSTAGE_NARROW").is_none() {
        assert!(opt_ib_substage_narrow_env());
    }
    // Promoted g43 winners (`g43_A`, `g43_D`) default to ON (`!= "0"`), while `g43_B`/`g43_C` stay OFF (`== "1"`).
    if std::env::var_os("AW2_SM66_CS_STAGE_EXACT").is_none() {
        assert!(crate::metal::pipeline::sm66_cs_stage_exact());
    }
    if std::env::var_os("AW2_LIST_START_RENDER_SPLIT").is_none() {
        assert!(!opt_list_start_render_split_env());
    }
    if std::env::var_os("AW2_CS_MID_QB_DEFER_V3").is_none() {
        assert!(!opt_cs_mid_qb_defer_v3_env());
    }
    if std::env::var_os("AW2_RP_KEEP_WRITE_EXACT").is_none() {
        assert!(opt_rp_keep_write_exact_env());
    }
    // g44 candidate knobs MUST default to OFF (`== "1"`) for clean A/B gating.
    if std::env::var_os("AW2_IB_UAV0_UNBOUNDED_EXACT").is_none() {
        assert!(!opt_ib_uav0_unbounded_exact_env());
    }
    if std::env::var_os("AW2_CLEAR_PRELIST_OVERLAP").is_none() {
        assert!(!opt_clear_prelist_overlap_env());
    }
    if std::env::var_os("AW2_CS_MID_QB_UNPOISON_V4").is_none() {
        assert!(!opt_cs_mid_qb_unpoison_v4_env());
    }
    if std::env::var_os("AW2_PS_SRV_CLASS_EXACT").is_none() {
        assert!(opt_ps_srv_class_exact_env());
    }
}

#[test]
fn clear_keep_dsv_retains_dsv_and_nows_clears_across_uav0_without_splitting_compute_on_gpu() {
    use std::sync::atomic::Ordering::Relaxed;
    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "g31 clear_keep_dsv queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_clear_keep_smart = true;
    l.opts.opt_clear_keep_dsv = true;

    let ds = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let ds2 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let rt_nows = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let up = test_buffer(64, heap_type::UPLOAD, 0);
    let mid = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_buf = test_buffer(64, heap_type::READBACK, 0);
    let rb_rt = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_d = test_buffer(256 * 4, heap_type::READBACK, 0);
    unsafe {
        *(up.buffer().unwrap().contents().as_ptr() as *mut u32) = 0x7788_99aa;
    }

    let dsv = TargetView::dsv(&ds, None).unwrap();
    let dsv2 = TargetView::dsv(&ds2, None).unwrap();
    let rtv_nows = TargetView::rtv(&rt_nows, None).unwrap();

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    // Queue deferred DSV + non-ShaderWrite RTV clears, then open a compute encoder
    // and cross `Barrier::Uav(0)` (modeling `L4` skinning + shadow-map clear).
    unsafe {
        l.clear_dsv(&dsv, true, 0.375, true, 0x55, &[]).unwrap();
        l.clear_rtv(&rtv_nows, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
    }
    let dsv_keep0 = stats::c::cf_dsv_keep.load(Relaxed);
    l.copy_buffer(&mid, 0, &up, 0, 16).unwrap();
    let cs_enc_ptr0 = match &l.enc {
        Encoder::Compute(e) => &**e as *const _ as usize,
        _ => panic!("expected open compute encoder"),
    };
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    l.barrier_list_with(&[Barrier::Transition(&mid as *const Resource as usize, STATE_COPY_DEST, STATE_COPY_SOURCE)], true, false);
    l.copy_buffer(&rb_buf, 0, &mid, 0, 16).unwrap();
    let cs_enc_ptr1 = match &l.enc {
        Encoder::Compute(e) => &**e as *const _ as usize,
        _ => panic!("expected compute encoder to remain open"),
    };

    assert_eq!(
        cs_enc_ptr1,
        cs_enc_ptr0,
        "opt_clear_keep_dsv must NOT split the open compute encoder at Barrier::Uav(0)"
    );
    assert!(stats::c::cf_dsv_keep.load(Relaxed) > dsv_keep0, "expected cf_dsv_keep increment");
    assert_eq!(l.clears.pending_clears.len(), 2, "DSV + non-ShaderWrite RTV clears must stay pending");

    // Guardrail: an explicit Transition naming `ds2` MUST still flush `ds2` immediately!
    unsafe { l.clear_dsv(&dsv2, true, 0.125, false, 0, &[]).unwrap() };
    assert_eq!(l.clears.pending_clears.len(), 3);
    let ds2_id = &ds2 as *const Resource as usize;
    l.barrier_list_with(&[Barrier::Transition(ds2_id, STATE_DEPTH_WRITE, STATE_DEPTH_READ)], true, false);
    assert_eq!(l.clears.pending_clears.len(), 2, "explicit Transition naming ds2 must flush ds2 clear");

    // Fold `dsv` + `rtv_nows` into `draw_encoder()` loadAction and verify GPU readback.
    l.set_render_targets(vec![rtv_nows], Some(dsv));
    let _ = unsafe { l.draw_encoder().unwrap() };
    assert!(l.clears.pending_clears.is_empty(), "retained clears must fold into draw_encoder loadAction");

    let rt_id = &rt_nows as *const Resource as usize;
    let ds_id = &ds as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE),
        ],
        true,
        false,
    );
    let fp_rt = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp_rt), [0, 0, 0], CopyLoc::Subresource(&rt_nows, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_d, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb_buf, 1)[0], 0x7788_99aa);
    assert_eq!(buf_words(&rb_rt, 1)[0], 0xff00_00ff, "folded RTV clear must produce red");
    let d0 = f32::from_bits(buf_words(&rb_d, 1)[0]);
    assert!((d0 - 0.375).abs() < 1e-5, "folded DSV clear mismatch: got {d0}");
}

#[test]
fn ib_exact_obl_skips_false_uav_and_vb_barriers_for_bindless_srv_while_ordering_true_hazards_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{ComputeAccess, ComputePipeline, READS_BUF, READS_TEX};
    use crate::metal::views;
    use dxil_metal::root_signature::{
        DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion, StaticSampler,
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
    let rs = RootSignature::new(RootSignatureDesc {
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

    let pso_base = ComputePipeline::new(rs.clone(), &cs_dxil, "g31_exact_obl_base").unwrap();
    // Model `skinning:transformCS`: exact `uav_slots` (`u0`), exact `bounded_read_slots` (`t0`),
    // `read_slots = None` due to an unbounded Buffer SRV table (`unbounded_srv_classes = READS_BUF`).
    let pso_skinning_like = Arc::new(ComputePipeline {
        pso: pso_base.pso,
        rs: pso_base.rs,
        threadgroup: pso_base.threadgroup,
        shader: pso_base.shader,
        hk_profile: pso_base.hk_profile,
        access: ComputeAccess {
            uav_slots: pso_base.access.uav_slots,
            read_slots: None,
            bounded_read_slots: pso_base.access.bounded_read_slots,
            unbounded_srv_classes: READS_BUF,
            heap_indexed_or_unknown: false,
            touches_textures: true,
            ..ComputeAccess::opaque()
        },
    });
    assert_eq!(READS_TEX & pso_skinning_like.access.unbounded_srv_classes, 0);

    let q = Queue::new(ListType::Direct, "g31 exact_obl queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_ib_tight = true;
    l.opts.opt_ib_bindless_war = true;
    l.opts.opt_ib_exact_obl = true;

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 8, true, "g31 exact_obl heap").unwrap();
    let write_entry = |slot: usize, entry: &[u8; 24]| unsafe {
        std::ptr::copy_nonoverlapping(entry.as_ptr(), (heap.cpu_base + slot * 24) as *mut u8, 24);
    };

    let tex_src = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_other = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let vb0 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let vb1 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let vb2 = test_buffer(512, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb0 = test_buffer(512, heap_type::READBACK, 0);
    let rb1 = test_buffer(512, heap_type::READBACK, 0);
    let rb2 = test_buffer(512, heap_type::READBACK, 0);
    for r in [&tex_src, &tex_other, &vb0, &vb1, &vb2, &rb0, &rb1, &rb2] {
        r.register_ptr();
    }
    write_entry(0, &views::srv_entry(Some(&tex_src), None).unwrap());
    write_entry(1, &views::srv_entry(Some(&tex_other), None).unwrap());

    let rtv_src = TargetView::rtv(&tex_src, None).unwrap();
    let rtv_other = TargetView::rtv(&tex_other, None).unwrap();
    let tex_src_id = &tex_src as *const Resource as usize;
    let tex_other_id = &tex_other as *const Resource as usize;
    let vb0_id = &vb0 as *const Resource as usize;
    let vb1_id = &vb1 as *const Resource as usize;
    let vb2_id = &vb2 as *const Resource as usize;

    let _stats_guard = gpu_stats_lock();
    l.opts.opt_ib_substage_narrow = false;
    l.begin(&alloc).unwrap();
    l.set_descriptor_heaps(heap.gpu_base, 0);

    // Compute-clear `tex_src` -> `[11, 22, 33, 1]`, transition to SRV, and run Dispatch 1 writing `vb0`.
    unsafe { l.clear_rtv(&rtv_src, [11.0, 22.0, 33.0, 1.0], &[]).unwrap() };
    l.barrier_list_with(&[Barrier::Transition(tex_src_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, true);
    l.compute_root.set_root_signature(rs.clone()).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap(); // slot 0 = tex_src
    l.compute_root.set_descriptor(1, vb0.gpu_va).unwrap();
    l.set_compute_pipeline(pso_skinning_like.clone());
    l.dispatch(1, 1, 1).unwrap();

    // Now record the 3 exact obligation classes that `g30` falsely fired on `skinning:transformCS`:
    // 1) `Barrier::Uav(0)` -> expanded per-resource `OblKind::UavOnly` on `vb0`
    // 2) `Transition(vb0, UAV -> VERTEX_AND_CONSTANT_BUFFER)` -> `OblKind::UavOrCbv` on `vb0`
    // 3) Compute-clear `tex_other` + `Transition(tex_other, RTV -> NON_PIXEL_SRV)` -> `OblKind::ClassSrv(READS_TEX)`
    unsafe { l.clear_rtv(&rtv_other, [44.0, 55.0, 66.0, 1.0], &[]).unwrap() };
    l.set_compute_pipeline(pso_skinning_like.clone());
    l.barrier_list_with(
        &[
            Barrier::Uav(0),
            Barrier::Transition(vb0_id, STATE_UAV, STATE_VB_CB),
            Barrier::Transition(tex_other_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
        ],
        true,
        true,
    );
    let obl_before_d2 = l.ib.ib_obl_tight.len();
    assert!(obl_before_d2 >= 2, "expected pending obligations on vb0 and tex_other");

    // Dispatch 2 reads `tex_src` (slot 0) and writes `vb1` (`u0 != vb0`, `t0 != tex_other`):
    // ALL pending obligations MUST be skipped (`ib_obl_tight` unconsumed, `vb0` retained in `ib_disp_uav`, `ib_skip_exact` > 0)!
    let skip_ex0 = stats::c::ib_skip_exact.load(Relaxed);
    l.compute_root.set_table(0, heap.gpu_base).unwrap(); // slot 0 = tex_src
    l.compute_root.set_descriptor(1, vb1.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(
        l.ib.ib_obl_tight.len(),
        obl_before_d2,
        "false UavOnly/UavOrCbv/ClassSrv(TEX) obligations must be skipped and stay pending"
    );
    assert!(
        l.ib.ib_disp_uav.as_slice().is_some_and(|v| v.contains(&vb0_id) && v.contains(&vb1_id)),
        "skipping enc_barrier must preserve prior ib_disp_uav touch set"
    );
    assert!(stats::c::ib_skip_exact.load(Relaxed) > skip_ex0, "expected ib_skip_exact increment");

    // Guardrail 1 (True bounded Texture SRV RAW): Dispatch 3 binds `tex_other` at bounded `t0` (slot 1)!
    // `tex_other` is in `ex.bounded_reads`, so `ClassSrv(READS_TEX)` MUST fire `enc_barrier` (consuming `ib_obl_tight`)!
    let enc_b1 = stats::c::enc_barrier.load(Relaxed);
    l.compute_root.set_table(0, heap.gpu_base + 24).unwrap(); // slot 1 = tex_other
    l.compute_root.set_descriptor(1, vb2.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert!(l.ib.ib_obl_tight.is_empty(), "true bounded Texture SRV RAW must fire enc_barrier and consume obligations");
    assert!(stats::c::enc_barrier.load(Relaxed) > enc_b1);

    // Guardrail 2 (True UAV WAW): record `Barrier::Uav(0)` (expands to `vb2`), then dispatch writing `vb2` again!
    l.barrier_list_with(&[Barrier::Uav(0)], true, true);
    assert_eq!(l.ib.ib_obl_tight.len(), 1);
    let enc_b2 = stats::c::enc_barrier.load(Relaxed);
    l.dispatch(1, 1, 1).unwrap();
    assert!(l.ib.ib_obl_tight.is_empty(), "true UAV WAW on u0 must fire enc_barrier");
    assert!(stats::c::enc_barrier.load(Relaxed) > enc_b2);

    // Guardrail 3 (True bindless Buffer SRV RAW): `Transition(vb2, UAV -> NON_PIXEL_SRV)` produces
    // `OblKind::ClassSrv(READS_BUF)`, which intersects `unbounded_srv_classes = READS_BUF` -> MUST fire!
    l.barrier_list_with(&[Barrier::Transition(vb2_id, STATE_UAV, STATE_NON_PIXEL_SRV)], true, true);
    assert_eq!(l.ib.ib_obl_tight.len(), 1);
    let enc_b3 = stats::c::enc_barrier.load(Relaxed);
    l.compute_root.set_descriptor(1, vb1.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert!(l.ib.ib_obl_tight.is_empty(), "ClassSrv(READS_BUF) against unbounded Buffer SRV must fire enc_barrier");
    assert!(stats::c::enc_barrier.load(Relaxed) > enc_b3);

    l.barrier_list_with(
        &[
            Barrier::Transition(vb0_id, STATE_VB_CB, STATE_COPY_SOURCE),
            Barrier::Transition(vb1_id, STATE_UAV, STATE_COPY_SOURCE),
            Barrier::Transition(vb2_id, STATE_NON_PIXEL_SRV, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    l.copy_buffer(&rb0, 0, &vb0, 0, 64).unwrap();
    l.copy_buffer(&rb1, 0, &vb1, 0, 64).unwrap();
    l.copy_buffer(&rb2, 0, &vb2, 0, 64).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    let f0: Vec<f32> = buf_words(&rb0, 4).into_iter().map(f32::from_bits).collect();
    let f1: Vec<f32> = buf_words(&rb1, 4).into_iter().map(f32::from_bits).collect();
    let f2: Vec<f32> = buf_words(&rb2, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&f0[..], &[11.0, 22.0, 33.0, 1.0]);
    assert_eq!(&f1[..], &[44.0, 55.0, 66.0, 1.0]);
    assert_eq!(&f2[..], &[44.0, 55.0, 66.0, 1.0]);
}

#[test]
fn icb_batch_and_lazy_icb_hoist_multi_pso_expansions_with_exact_gpu_readback_parity() {
    use crate::metal::indirect::{arg, IndirectLayout};
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootParameter, RootParameterKind, RootSignature, RootSignatureDesc,
        RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let rs = RootSignature::new(RootSignatureDesc {
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

    let make_desc = |cmp: MTLCompareFunction| GraphicsDesc {
        vs: &vs_dxil,
        ps: None,
        blend: BlendDesc::default(),
        sample_mask: u32::MAX,
        raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
        depth_stencil: DepthStencilState { depth_compare: cmp, depth_write: true, ..DepthStencilState::default() },
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: 3, // TRIANGLE
        rtv_formats: Vec::new(),
        dsv_format: D32_S8,
        sample_count: 1,
    };

    // Compile all 3 PSOs with `lazy_icb = true` (`AW2_GFX_LAZY_ICB=1`):
    // - `pso_direct`: used ONLY for direct `draw_indexed` -> `icb_pso` must remain uninitialized!
    // - `pso_less` & `pso_always`: used for `ExecuteIndirect` ICB draws -> `icb_pso` lazily compiled on demand!
    let pso_direct = Arc::new(GraphicsPipeline::new_with_opts(rs.clone(), &make_desc(MTLCompareFunction::Less), "g31_pso_direct", true).unwrap());
    let pso_less = Arc::new(GraphicsPipeline::new_with_opts(rs.clone(), &make_desc(MTLCompareFunction::Less), "g31_pso_less", true).unwrap());
    let pso_always = Arc::new(GraphicsPipeline::new_with_opts(rs.clone(), &make_desc(MTLCompareFunction::Always), "g31_pso_always", true).unwrap());
    assert!(pso_direct.icb_pso.get().is_none());
    assert!(pso_less.icb_pso.get().is_none());
    assert!(pso_always.icb_pso.get().is_none());

    // Indirect signatures:
    // 1) `layout_draw`: 2 root constants + D3D12_DRAW_ARGUMENTS (stride = 24)
    // 2) `layout_idx`:  2 root constants + D3D12_DRAW_INDEXED_ARGUMENTS (stride = 28)
    let layout_draw = IndirectLayout::new(24, &[(arg::CONSTANT, [0, 0, 2]), (arg::DRAW, [0; 3])]).unwrap();
    let layout_idx = IndirectLayout::new(28, &[(arg::CONSTANT, [0, 0, 2]), (arg::DRAW_INDEXED, [0; 3])]).unwrap();

    // Index buffer `[0u16, 1u16, 2u16, 0u16]`.
    let ib_buf = test_buffer(64, heap_type::UPLOAD, 0);
    unsafe {
        let p = ib_buf.buffer().unwrap().contents().as_ptr() as *mut u16;
        *p.add(0) = 0;
        *p.add(1) = 1;
        *p.add(2) = 2;
        *p.add(3) = 0;
    }

    // Upload indirect commands + count buffers:
    // - `args_q0`: 2 Draw commands (`max_count = 2`, `count_buf = 1`), so slot 1 MUST be culled by count!
    // - `args_q2_src` -> copied mid-list to GPU `args_q2_gpu`, then transitioned mid-pass `COPY_DEST -> INDIRECT_ARG`!
    //   Contains 2 DrawIndexed commands: cmd 0 has `InstanceCount = 0` (culled!), cmd 1 has `InstanceCount = 1` (active!).
    // - `args_q3`: 1 Draw command (`max_count = 1`, `count_buf = 1`).
    let args_up = test_buffer(512, heap_type::UPLOAD, 0);
    let args_q2_gpu = test_buffer(128, heap_type::DEFAULT, 0);
    unsafe {
        let base = args_up.buffer().unwrap().contents().as_ptr() as *mut u32;
        // Offset 0: `args_q0` (2 * 24B = 48B = 12 u32s)
        let q0: [u32; 12] = [
            10, 20, 3, 1, 0, 0, // cmd 0: active (3 verts, 1 inst)
            30, 40, 3, 1, 0, 0, // cmd 1: must be culled by count_1 == 1!
        ];
        std::ptr::copy_nonoverlapping(q0.as_ptr(), base, q0.len());
        // Offset 64 (u32 idx 16): `count_1 = 1`, Offset 80 (u32 idx 20): `count_2 = 2`
        *base.add(16) = 1;
        *base.add(20) = 2;
        // Offset 128 (u32 idx 32): `args_q2` (2 * 28B = 56B = 14 u32s)
        let q2: [u32; 14] = [
            50, 60, 3, 0, 0, 0, 0, // cmd 0: InstanceCount == 0 (culled!)
            70, 80, 3, 1, 0, 0, 0, // cmd 1: active DrawIndexed (3 indices, 1 inst)
        ];
        std::ptr::copy_nonoverlapping(q2.as_ptr(), base.add(32), q2.len());
        // Offset 256 (u32 idx 64): `args_q3` (1 * 24B = 24B = 6 u32s)
        let q3: [u32; 6] = [90, 99, 3, 1, 0, 0];
        std::ptr::copy_nonoverlapping(q3.as_ptr(), base.add(64), q3.len());
    }

    let q = Queue::new(ListType::Direct, "g31 icb_batch queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _stats_guard = gpu_stats_lock();

    // Warmup pass on `alloc` that populates the first ICB slots with active commands, then `alloc.reset()`,
    // proving `encode_v2`'s `rc.reset()` clears stale recycled ICB commands on culled lanes!
    {
        let mut warm = CommandList::new(ListType::Direct).unwrap();
        let ds_w = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
        let dsv_w = TargetView::dsv(&ds_w, None).unwrap();
        warm.begin(&alloc).unwrap();
        unsafe { warm.clear_dsv(&dsv_w, true, 0.9, false, 0, &[]).unwrap() };
        warm.gfx_root.set_root_signature(rs.clone()).unwrap();
        warm.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
        warm.set_render_targets(Vec::new(), Some(dsv_w));
        warm.gfx.topology = 4;
        warm.set_graphics_pipeline(pso_less.clone());
        warm.execute_indirect(&layout_draw, 2, args_up.gpu_va, None, Some(args_up.gpu_va + 80)).unwrap();
        warm.close().unwrap();
        q.submit(&[&warm.cb]);
        q.wait_idle();
        alloc.reset();
    }

    let run_pass = |batch_on: bool| -> Vec<f32> {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.opts.opt_icb_batch = batch_on;
        let ds = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
        let rb_d = test_buffer(256 * 8, heap_type::READBACK, 0);
        let dsv = TargetView::dsv(&ds, None).unwrap();
        let args_q2_id = &args_q2_gpu as *const Resource as usize;

        l.begin(&alloc).unwrap();
        // Copy `args_q2` into `args_q2_gpu` BEFORE opening the render pass, but intentionally
        // delay the `COPY_DEST -> INDIRECT_ARG` transition until MID-PASS to test kept barrier hoisting!
        l.copy_buffer(&args_q2_gpu, 0, &args_up, 128, 64).unwrap();
        unsafe { l.clear_dsv(&dsv, true, 0.9, false, 0, &[]).unwrap() };

        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
        l.set_render_targets(Vec::new(), Some(dsv));
        l.gfx.topology = 4; // TRIANGLELIST
        l.gfx.ib = Some((ib_buf.gpu_va, 64, 57)); // R16_UINT

        // 1) Quadrant Q0 `[0..4, 0..4]` -> `ExecuteIndirect` #1 (`pso_less`, `znear = 0.2`, `count = 1` of `2`).
        l.set_graphics_pipeline(pso_less.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.2, zfar: 1.0 }]);
        l.set_scissors(vec![Rect { left: 0, top: 0, right: 4, bottom: 4 }]);
        l.execute_indirect(&layout_draw, 2, args_up.gpu_va, None, Some(args_up.gpu_va + 64)).unwrap();

        // 2) Quadrant Q1 `[4..8, 0..4]` -> Interleaved direct `draw_indexed` with `pso_direct` at `znear = 0.4`.
        l.set_graphics_pipeline(pso_direct.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.4, zfar: 1.0 }]);
        l.set_scissors(vec![Rect { left: 4, top: 0, right: 8, bottom: 4 }]);
        l.draw_indexed(3, 1, 0, 0, 0).unwrap();

        // 3) Mid-pass kept barrier `COPY_DEST -> INDIRECT_ARG` on `args_q2_gpu`, then Quadrant Q2 `[0..4, 4..8]`
        //    -> `ExecuteIndirect` #2 (`pso_less`, `DRAW_INDEXED`, `znear = 0.6`, cmd 0 `InstanceCount = 0`, cmd 1 active).
        l.barrier_list_with(&[Barrier::Transition(args_q2_id, STATE_COPY_DEST, STATE_INDIRECT_ARG)], true, false);
        assert!(l.gfx.draw_pass, "unattached COPY_DEST -> INDIRECT_ARG must keep draw pass open");
        l.set_graphics_pipeline(pso_less.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.6, zfar: 1.0 }]);
        l.set_scissors(vec![Rect { left: 0, top: 4, right: 4, bottom: 8 }]);
        l.execute_indirect(&layout_idx, 2, args_q2_gpu.gpu_va, None, Some(args_up.gpu_va + 80)).unwrap();

        // 4) Quadrant Q3 `[4..8, 4..8]`: first direct `draw` at `znear = 0.15` (`pso_direct`), THEN overwrite
        //    with `pso_always` (`MTLCompareFunction::Always`!) `ExecuteIndirect` #3 at `znear = 0.35`, proving
        //    exact draw ordering (`direct -> ICB`) AND multi-PSO depth-stencil switching!
        l.set_graphics_pipeline(pso_direct.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.15, zfar: 1.0 }]);
        l.set_scissors(vec![Rect { left: 4, top: 4, right: 8, bottom: 8 }]);
        l.draw(3, 1, 0, 0).unwrap();

        l.set_graphics_pipeline(pso_always.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.35, zfar: 1.0 }]);
        l.execute_indirect(&layout_draw, 1, args_up.gpu_va + 256, None, Some(args_up.gpu_va + 64)).unwrap();

        let ds_id = &ds as *const Resource as usize;
        l.barrier_list_with(&[Barrier::Transition(ds_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE)], true, false);
        let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_d, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
        l.close().unwrap();

        q.submit(&[&l.cb]);
        q.wait_idle();
        alloc.reset();

        let words = buf_words(&rb_d, 64 * 8);
        let mut out = Vec::with_capacity(64);
        for y in 0..8 {
            for x in 0..8 {
                out.push(f32::from_bits(words[y * 64 + x]));
            }
        }
        out
    };

    let ref_depths = run_pass(false);
    let hoist0 = stats::c::icb_hoist_saved.load(Relaxed);
    let opt_depths = run_pass(true);
    let hoist_delta = stats::c::icb_hoist_saved.load(Relaxed) - hoist0;

    assert_eq!(hoist_delta, 2, "3 hoisted ExecuteIndirect calls in 1 pass must save K - 1 = 2 mid-pass barriers");
    assert!(pso_direct.icb_pso.get().is_none(), "direct-only PSO must never compile an ICB PSO variant");
    assert!(pso_less.icb_pso.get().is_some(), "ICB-used PSO must lazily compile and cache its ICB PSO variant");
    assert!(pso_always.icb_pso.get().is_some(), "second ICB PSO must lazily compile and cache its ICB PSO variant");

    for y in 0..8 {
        for x in 0..8 {
            let want = match (x < 4, y < 4) {
                (true, true) => 0.2,
                (false, true) => 0.4,
                (true, false) => 0.6,
                (false, false) => 0.35,
            };
            let got_ref = ref_depths[y * 8 + x];
            let got_opt = opt_depths[y * 8 + x];
            assert!((got_ref - want).abs() < 1e-5, "ref pixel ({x},{y}): got {got_ref}, want {want}");
            assert_eq!(
                got_opt.to_bits(),
                got_ref.to_bits(),
                "AW2_ICB_BATCH=1 pixel ({x},{y}) mismatch vs reference: opt={got_opt} ref={got_ref}"
            );
        }
    }
}

#[test]
fn list_barrier_tight_replaces_list_start_full_sync_and_orders_cross_list_hazards_on_gpu() {
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

    // Compute PSO reading `t0` (`RGBA32_FLOAT` 2D texture) and writing `u0` (`float4` buffer).
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
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g32_list_tight_cs").unwrap());

    // Depth-only graphics PSO for verifying split `Vertex` vs `Fragment` render list-start sync.
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
            "g32_list_tight_gfx",
        )
        .unwrap(),
    );

    let q = Queue::new(ListType::Direct, "g32 list_barrier_tight queue").unwrap();
    let a1 = CommandAllocator::new(ListType::Direct).unwrap();
    let a2 = CommandAllocator::new(ListType::Direct).unwrap();
    let a3 = CommandAllocator::new(ListType::Direct).unwrap();

    let tex = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET); // NOT ShaderWrite -> render-pass RTV clear (`Fragment` write!)
    let ds = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let uav_buf = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_buf = test_buffer(256, heap_type::READBACK, 0);
    for r in [&tex, &ds, &uav_buf, &rb_buf] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g32 list_tight heap").unwrap();
    unsafe {
        let e0 = views::srv_entry(Some(&tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }

    let rtv = TargetView::rtv(&tex, None).unwrap();
    let dsv = TargetView::dsv(&ds, None).unwrap();
    let tex_id = &tex as *const Resource as usize;
    let uav_id = &uav_buf as *const Resource as usize;

    let _stats_guard = gpu_stats_lock();
    let qb_tight0 = stats::c::qb_list_tight.load(Relaxed);
    let qb_trail0 = stats::c::qb_list_trail.load(Relaxed);

    // List 1: Render-pass RTV clear (`Fragment` write on non-ShaderWrite `tex`), then
    // trailing `Transition(tex, RENDER_TARGET -> NON_PIXEL_SRV)` before `close()`!
    let mut l1 = CommandList::new(ListType::Direct).unwrap();
    l1.opts.opt_list_barrier_tight = true;
    l1.begin(&a1).unwrap();
    assert!(!l1.bar.needs_barrier, "AW2_LIST_BARRIER_TIGHT=1 must NOT arm FullWhy::ListStart");
    unsafe { l1.clear_rtv(&rtv, [7.0, 14.0, 21.0, 1.0], &[]).unwrap() };
    l1.barrier_list_with(&[Barrier::Transition(tex_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, true);
    l1.close().unwrap();
    assert!(stats::c::qb_list_trail.load(Relaxed) > qb_trail0, "trailing transition at close() must emit producer barrier");

    // List 2: Starts with Compute reading `tex` (written in List 1 `Fragment`!) and writing `uav_buf`,
    // followed by a Render pass drawing into `ds` (split `Vertex` vs `Fragment` sync, 0 `qb_frag_to_vtx`!).
    let mut l2 = CommandList::new(ListType::Direct).unwrap();
    l2.opts.opt_list_barrier_tight = true;
    l2.begin(&a2).unwrap();
    assert!(!l2.bar.needs_barrier, "AW2_LIST_BARRIER_TIGHT=1 must NOT arm FullWhy::ListStart on l2");
    l2.set_descriptor_heaps(heap.gpu_base, 0);
    l2.compute_root.set_root_signature(cs_rs).unwrap();
    l2.compute_root.set_table(0, heap.gpu_base).unwrap();
    l2.compute_root.set_descriptor(1, uav_buf.gpu_va).unwrap();
    l2.set_compute_pipeline(cs_pso);
    l2.dispatch(1, 1, 1).unwrap();
    assert_eq!(
        l2.bar.enc_sync_after,
        COMPUTE_STAGES | PRE_RASTER,
        "compute-first list must wait only on COMPUTE_STAGES | PRE_RASTER (Fragment ordered by List 1 close producer barrier)"
    );

    let f2v_before_rp = stats::c::qb_frag_to_vtx.load(Relaxed);
    l2.gfx_root.set_root_signature(gfx_rs).unwrap();
    l2.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
    l2.set_render_targets(Vec::new(), Some(dsv));
    l2.gfx.topology = 4;
    l2.set_graphics_pipeline(gfx_pso);
    l2.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.25, zfar: 1.0 }]);
    l2.draw(3, 1, 0, 0).unwrap();
    assert_eq!(
        stats::c::qb_frag_to_vtx.load(Relaxed),
        f2v_before_rp,
        "split Vertex/Fragment list-start render sync must not emit qb_frag_to_vtx"
    );
    l2.barrier_list_with(&[Barrier::Transition(uav_id, STATE_UAV, STATE_COPY_SOURCE)], true, true);
    l2.close().unwrap();

    // List 3: Starts directly with a `Blit` copy (`copy_buffer`, modeling `L3#0`),
    // which MUST promote `list_sync_compute` to `MTLStages::All`!
    let mut l3 = CommandList::new(ListType::Direct).unwrap();
    l3.opts.opt_list_barrier_tight = true;
    l3.begin(&a3).unwrap();
    assert!(!l3.bar.needs_barrier, "AW2_LIST_BARRIER_TIGHT=1 must NOT arm FullWhy::ListStart on l3");
    l3.copy_buffer(&rb_buf, 0, &uav_buf, 0, 64).unwrap();
    assert_eq!(l3.bar.enc_sync_after, MTLStages::All, "Blit-first list must promote list_sync_compute to All");
    l3.close().unwrap();

    assert!(stats::c::qb_list_tight.load(Relaxed) >= qb_tight0 + 4);

    q.submit(&[&l1.cb, &l2.cb, &l3.cb]);
    q.wait_idle();

    let vals: Vec<f32> = buf_words(&rb_buf, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&vals[..], &[7.0, 14.0, 21.0, 1.0]);
}

#[test]
fn barrier_tight_v2_eliminates_false_frag_to_vtx_on_kept_writes_and_generic_read_textures_on_gpu() {
    use crate::metal::pipeline::{ComputeAccess, ComputePipeline, GraphicsDesc, GraphicsPipeline, READS_BUF, READS_TEX};
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
    let tex_a = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_b = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_mid = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_common = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let buf_uav = test_buffer(256, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    for r in [&tex_a, &tex_b, &tex_mid, &tex_common, &buf_uav] {
        r.register_ptr();
    }
    let tex_a_id = &tex_a as *const Resource as usize;
    let tex_mid_id = &tex_mid as *const Resource as usize;
    let tex_common_id = &tex_common as *const Resource as usize;
    let buf_uav_id = &buf_uav as *const Resource as usize;

    // 1) Unit check on `filter_res_state_bits`: `STATE_GENERIC_READ` (`0xAC3`) on a 2D texture
    // drops buffer-only `0x1 | 0x2 | 0x200 = 0x203` bits -> `0x8C0`, while preserving `0xAC3` on buffers!
    assert_eq!(filter_res_state_bits(tex_a_id, STATE_GENERIC_READ), 0x8C0);
    assert_eq!(filter_res_state_bits(buf_uav_id, STATE_GENERIC_READ), STATE_GENERIC_READ);

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
    let ds1 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let ds2 = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let pso_no_srv_vs = Arc::new(
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
                rtv_formats: vec![RGBA8_UNORM],
                dsv_format: D32_S8,
                sample_count: 1,
            },
            "g32_v2_pso",
        )
        .unwrap(),
    );
    assert_eq!(pso_no_srv_vs.pre_raster_reads, 0);

    let q = Queue::new(ListType::Direct, "g32 barrier_tight_v2 queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_barrier_tight_v2 = true;
    l.opts.opt_rp_keep_write = true;
    l.opts.opt_ib_tight = true;
    l.opts.opt_ib_exact_obl = true;

    let rtv_a = TargetView::rtv(&tex_a, None).unwrap();
    let rtv_b = TargetView::rtv(&tex_b, None).unwrap();
    let dsv1 = TargetView::dsv(&ds1, None).unwrap();
    let dsv2 = TargetView::dsv(&ds2, None).unwrap();

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    // Consume initial list-start barrier on a tiny setup clear so Pass 1 -> Pass 2 measures pure mid-list barriers.
    unsafe {
        l.clear_dsv(&dsv1, true, 1.0, false, 0, &[]).unwrap();
        l.clear_dsv(&dsv2, true, 1.0, false, 0, &[]).unwrap();
        l.clear_rtv(&rtv_a, [0.25, 0.5, 0.75, 1.0], &[]).unwrap();
        l.clear_rtv(&rtv_b, [0.5, 0.25, 0.75, 1.0], &[]).unwrap();
    }
    l.gfx_root.set_root_signature(gfx_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
    l.set_render_targets(vec![rtv_a], Some(dsv1));
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_no_srv_vs.clone());
    l.draw(3, 1, 0, 0).unwrap();

    let f2v_0 = stats::c::qb_frag_to_vtx.load(Relaxed);
    let dv_def0 = stats::c::dv_defer.load(Relaxed);
    let keep_w0 = stats::c::rb_keep_write.load(Relaxed);

    // Inside Pass 1: record kept `Barrier::Uav(0)`, `Transition(buf_uav, PIXEL_SRV -> UAV)`,
    // and unattached non-simul texture `Transition(tex_common, RTV -> STATE_COMMON)` (`s1 == 0`)!
    l.barrier_list_with(
        &[
            Barrier::Uav(0),
            Barrier::Transition(buf_uav_id, STATE_PIXEL_SRV, STATE_UAV),
            Barrier::Transition(tex_common_id, STATE_RENDER_TARGET, STATE_COMMON),
        ],
        true,
        true,
    );
    assert!(l.gfx.draw_pass, "kept UAV + UAV write + unattached COMMON texture transitions must keep Pass 1 open");
    assert!(stats::c::rb_keep_write.load(Relaxed) > keep_w0);
    assert!(
        !l.rp_sync.rp_write_before.intersects(PRE_RASTER),
        "rp_write_before must NOT contain hard PRE_RASTER stages under AW2_BARRIER_TIGHT_V2=1"
    );
    assert!(
        (l.rp_sync.rp_write_soft[0] | l.rp_sync.rp_write_soft[1]).contains(PRE_RASTER),
        "rp_write_soft must capture soft PRE_RASTER stages"
    );

    // Mid-pass `GENERIC_READ` texture transition (`rb_keep`) followed by another draw in Pass 1:
    // `render_mid_barrier` must defer `PRE_RASTER` into `vtx_defer` (`dv_defer`) instead of stalling `Vertex`!
    l.barrier_list_with(&[Barrier::Transition(tex_mid_id, STATE_RENDER_TARGET, STATE_GENERIC_READ)], true, true);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(stats::c::dv_defer.load(Relaxed) > dv_def0, "mid-pass GENERIC_READ texture must defer PRE_RASTER into vtx_defer");

    // Switch to Pass 2 (`rtv_b`, `dsv2`) and also transition `tex_a` `RENDER_TARGET -> STATE_GENERIC_READ`!
    // Because `pso_no_srv_vs.pre_raster_reads == 0`, NEITHER Pass 1's `rp_write_soft` NOR `tex_a`'s `GENERIC_READ`
    // may emit a `qb_frag_to_vtx` stall!
    l.set_render_targets(vec![rtv_b], Some(dsv2));
    l.barrier_list_with(&[Barrier::Transition(tex_a_id, STATE_RENDER_TARGET, STATE_GENERIC_READ)], true, true);
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(
        stats::c::qb_frag_to_vtx.load(Relaxed),
        f2v_0,
        "AW2_BARRIER_TIGHT_V2=1 must eliminate false qb_frag_to_vtx across kept UAV/write and GENERIC_READ texture barriers"
    );

    // Verify `note_compute_barriers_tight` classifies `Transition(tex_a, RTV -> STATE_GENERIC_READ)`
    // as `OblKind::ClassSrv(READS_TEX)` (NOT `OblKind::Any`) so unrelated dispatches skip `enc_barrier`!
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
    let cs_base = ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g32_v2_cs").unwrap();
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
    let tex_cs = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    tex_cs.register_ptr();
    let rtv_cs = TargetView::rtv(&tex_cs, None).unwrap();
    let tex_cs_id = &tex_cs as *const Resource as usize;
    unsafe { l.clear_rtv(&rtv_cs, [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };
    l.barrier_list_with(&[Barrier::Transition(tex_cs_id, STATE_RENDER_TARGET, STATE_GENERIC_READ)], true, true);
    assert!(
        l.ib.ib_obl_tight.iter().any(|o| o.r == tex_cs_id && o.kind == OblKind::ClassSrv(READS_TEX)),
        "GENERIC_READ texture transition must classify as OblKind::ClassSrv(READS_TEX)"
    );
    let skip_ex0 = stats::c::ib_skip_exact.load(Relaxed);
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, 0).unwrap();
    l.compute_root.set_descriptor(1, buf_uav.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso);
    l.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::ib_skip_exact.load(Relaxed) > skip_ex0, "unrelated dispatch must skip GENERIC_READ texture obligation via ib_skip_exact");

    l.close().unwrap();
    q.submit(&[&l.cb]);
    q.wait_idle();
}

#[test]
fn fast_bind_pso_deduplicates_table_and_vb_binds_and_executes_specialized_clear_kernels_on_gpu() {
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() {
        return;
    }
    let q = Queue::new(ListType::Direct, "g32 fast_bind_pso queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_fast_bind_pso = true;

    let tex0 = test_texture(20, 20, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex1 = test_texture(20, 20, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let tex_up = test_texture(32, 32, RGBA8_UNORM, resource_flags::ALLOW_UNORDERED_ACCESS);
    let fill_buf = test_buffer(4096, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_buf = test_buffer(4096, heap_type::READBACK, 0);
    let rb_tex0 = test_buffer(256 * 20, heap_type::READBACK, 0);
    let rb_tex1 = test_buffer(256 * 20, heap_type::READBACK, 0);
    let rb_up = test_buffer(256 * 32, heap_type::READBACK, 0);

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();

    // 1) Verify dirty-tracked `set_table_addr` and `set_table_vb` skip redundant writes
    // and re-bind cleanly after value change or `invalidate_table_cache()`.
    let skip0 = stats::c::bind_skip.load(Relaxed);
    l.set_table_addr(0x1000, 0);
    l.set_table_addr(0x1000, 0);
    l.set_table_addr(0x2000, 2);
    l.set_table_addr(0x2000, 2);
    l.set_table_vb(0x3000, 16, 0);
    l.set_table_vb(0x3000, 16, 0);
    assert_eq!(stats::c::bind_skip.load(Relaxed) - skip0, 3, "unchanged slot/VB bindings must increment bind_skip");
    l.set_table_vb(0x3000, 32, 0); // stride changed -> must NOT skip!
    assert_eq!(stats::c::bind_skip.load(Relaxed) - skip0, 3);
    l.invalidate_table_cache();
    l.set_table_addr(0x1000, 0); // cache invalidated -> must NOT skip!
    assert_eq!(stats::c::bind_skip.load(Relaxed) - skip0, 3);

    // 2) Execute multi-target `compute_clear_batch` (`[16, 16, 1]` on `20x20` non-multiple size to test edge bounds!),
    // `[256, 1, 1]` `aw2_fill` buffer UAV clear on 1024 dwords (`4096` bytes), and `[16, 16, 1]` `upscale_resource` (`20x20 -> 32x32`).
    let rtv0 = TargetView::rtv(&tex0, None).unwrap();
    let rtv1 = TargetView::rtv(&tex1, None).unwrap();
    unsafe {
        l.clear_rtv(&rtv0, [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
        l.clear_rtv(&rtv1, [0.0, 0.0, 1.0, 1.0], &[]).unwrap();
    }
    // Flush both RTV clears together through `compute_clear_batch` (deduplicating `util_table` + `aw2_clear_2d_f` `[16,16,1]` PSO!).
    let tex0_id = &tex0 as *const Resource as usize;
    let tex1_id = &tex1 as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(tex0_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV),
            Barrier::Transition(tex1_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );

    // Raw buffer UAV clear (`dimension = 1`, `count = 1024`, `flags = 1` RAW) -> `[256, 1, 1]` `aw2_fill`.
    let mut raw_uav_desc = [0u8; 40];
    raw_uav_desc[0..4].copy_from_slice(&42u32.to_le_bytes()); // R32_TYPELESS / R32_UINT
    raw_uav_desc[4..8].copy_from_slice(&1u32.to_le_bytes()); // BUFFER
    raw_uav_desc[16..20].copy_from_slice(&1024u32.to_le_bytes()); // NumElements = 1024
    raw_uav_desc[32..36].copy_from_slice(&1u32.to_le_bytes()); // RAW flag
    l.clear_uav(&fill_buf, Some(&raw_uav_desc), ClearValue::Uint([0xdead_beef; 4]), &[]).unwrap();

    // Bilinear 2D upscale `tex0` (`20x20` green) -> `tex_up` (`32x32`) with `[16, 16, 1]` `aw2_upscale_2d_f`.
    l.upscale_resource(&tex_up, &tex0, [20, 20], [32, 32]).unwrap();

    let fill_id = &fill_buf as *const Resource as usize;
    let up_id = &tex_up as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(tex0_id, STATE_NON_PIXEL_SRV, STATE_COPY_SOURCE),
            Barrier::Transition(fill_id, STATE_UAV, STATE_COPY_SOURCE),
            Barrier::Transition(up_id, STATE_UAV, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    let fp20 = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 20, height: 20, depth: 1, row_pitch: 256, _pad: 0 };
    let fp32 = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 32, height: 32, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_buffer(&rb_buf, 0, &fill_buf, 0, 4096).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_tex0, fp20), [0, 0, 0], CopyLoc::Subresource(&tex0, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_tex1, fp20), [0, 0, 0], CopyLoc::Subresource(&tex1, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_up, fp32), [0, 0, 0], CopyLoc::Subresource(&tex_up, 0), None).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    let words = buf_words(&rb_buf, 1024);
    assert!(words.iter().all(|&w| w == 0xdead_beef), "all 1024 dwords must equal 0xdead_beef under [256,1,1] aw2_fill");
    let p0 = buf_words(&rb_tex0, 64 * 20);
    let p1 = buf_words(&rb_tex1, 64 * 20);
    assert_eq!(p0[0], 0xff00_ff00);
    assert_eq!(p0[19 * 64 + 19], 0xff00_ff00, "bottom-right (19,19) pixel of 20x20 texture must be cleared cleanly");
    assert_eq!(p1[0], 0xffff_0000);
    assert_eq!(p1[19 * 64 + 19], 0xffff_0000);
    let pup = buf_words(&rb_up, 64 * 32);
    assert_eq!(pup[0], 0xff00_ff00);
    assert_eq!(pup[31 * 64 + 31], 0xff00_ff00, "bottom-right (31,31) pixel of 32x32 upscaled texture must be green");
}

#[test]
fn empty_enc_elide_skips_zero_work_lists_and_narrows_blit_first_list_start_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::fence::Fence;
    use crate::metal::pipeline::ComputePipeline;
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
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g33_stage_a_cs").unwrap());

    let q = Queue::new(ListType::Direct, "g33 stage A queue").unwrap();
    let f = Fence::new(0).unwrap();
    let a0 = CommandAllocator::new(ListType::Direct).unwrap();
    let a1 = CommandAllocator::new(ListType::Direct).unwrap();
    let a2 = CommandAllocator::new(ListType::Direct).unwrap();
    let a3 = CommandAllocator::new(ListType::Direct).unwrap();
    let a4 = CommandAllocator::new(ListType::Direct).unwrap();

    let tex = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let up_buf = test_buffer(64, heap_type::UPLOAD, 0);
    let safe_dst = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let common_dst = test_buffer(64, heap_type::DEFAULT, 0);
    let uav_out = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_safe = test_buffer(64, heap_type::READBACK, 0);
    let rb_uav = test_buffer(64, heap_type::READBACK, 0);
    let clear_only_tex = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb_clear = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&tex, &up_buf, &safe_dst, &common_dst, &uav_out, &rb_safe, &rb_uav, &clear_only_tex, &rb_clear] {
        r.register_ptr();
    }
    unsafe {
        *(up_buf.buffer().unwrap().contents().as_ptr() as *mut u32) = 0x55aa_1234;
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g33 stage A heap").unwrap();
    unsafe {
        let e0 = views::srv_entry(Some(&tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }

    // 1) Zero-work command lists (`L0`, `L1`, `L2`) with state binds + barriers MUST elide on `close()`.
    let _stats_guard = gpu_stats_lock();
    let elide0 = stats::c::empty_list_elided.load(Relaxed);
    let mut l0 = CommandList::new(ListType::Direct).unwrap();
    let mut l1 = CommandList::new(ListType::Direct).unwrap();
    l0.opts.opt_empty_enc_elide = true;
    l1.opts.opt_empty_enc_elide = true;
    l0.begin(&a0).unwrap();
    l0.set_descriptor_heaps(heap.gpu_base, 0);
    l0.barrier_list_with(&[Barrier::Uav(0)], true, true);
    l0.close().unwrap();
    assert!(!l0.has_commands(), "zero-work list L0 must set has_commands() == false");

    l1.begin(&a1).unwrap();
    l1.barrier_list_with(
        &[Barrier::Transition(&safe_dst as *const Resource as usize, STATE_UAV, STATE_COPY_DEST)],
        true,
        true,
    );
    l1.close().unwrap();
    assert!(!l1.has_commands(), "barrier-only list L1 must set has_commands() == false");
    assert_eq!(stats::c::empty_list_elided.load(Relaxed) - elide0, 2);

    // Verify `qw_elided` stays 100% ON (same-queue Signal -> Wait elision preserved!).
    q.signal(&f, 11);
    let qwe0 = stats::c::qw_elided.load(Relaxed);
    q.wait(&f, 11);
    assert_eq!(stats::c::qw_elided.load(Relaxed) - qwe0, 1, "qw_elided must stay enabled");

    // Guardrail 1: a list whose ONLY work is a deferred `clear_rtv` (flushed at `close()`) MUST NOT elide!
    let mut l_clear = CommandList::new(ListType::Direct).unwrap();
    l_clear.opts.opt_empty_enc_elide = true;
    l_clear.begin(&a2).unwrap();
    let rtv_co = TargetView::rtv(&clear_only_tex, None).unwrap();
    unsafe { l_clear.clear_rtv(&rtv_co, [1.0, 0.0, 0.0, 1.0], &[]).unwrap() };
    l_clear.close().unwrap();
    assert!(l_clear.has_commands(), "deferred-clear-only list flushed at close() must NOT elide");

    // 2) `L3` `Blit`-first list with proven non-Fragment transition (`NON_PIXEL_SRV -> COPY_DEST`) on `safe_dst`
    // + `UPLOAD` `up_buf`: narrows `ListStart` to `(COMPUTE_STAGES | PRE_RASTER) -> Blit`, then
    // upgrades cleanly (`cs_mid_qb`) before `Dispatch` in the same encoder!
    let narrow0 = stats::c::blit_list_narrow.load(Relaxed);
    let mut l3 = CommandList::new(ListType::Direct).unwrap();
    l3.opts.opt_empty_enc_elide = true;
    l3.begin(&a3).unwrap();
    let safe_id = &safe_dst as *const Resource as usize;
    let tex_id = &tex as *const Resource as usize;
    let uav_id = &uav_out as *const Resource as usize;
    let clear_id = &clear_only_tex as *const Resource as usize;
    l3.barrier_list_with(&[Barrier::Transition(safe_id, STATE_NON_PIXEL_SRV, STATE_COPY_DEST)], true, true);
    l3.copy_buffer(&safe_dst, 0, &up_buf, 0, 16).unwrap();
    assert!(!l3.bar.needs_barrier);
    assert_eq!(stats::c::blit_list_narrow.load(Relaxed) - narrow0, 1);
    assert_eq!(
        l3.bar.enc_sync_after,
        COMPUTE_STAGES | PRE_RASTER,
        "non-Fragment-proven Blit-first list must skip waiting on prior Fragment stage"
    );
    assert!(l3.bar.enc_blit_only_sync);
    assert!(l3.bar.list_cs_unwaited_frag);

    // Now clear `tex` via compute + run `Dispatch` in the same encoder: must upgrade sync to `MTLStages::All`!
    let mid0 = stats::c::cs_mid_qb.load(Relaxed);
    let rtv_tex = TargetView::rtv(&tex, None).unwrap();
    unsafe { l3.clear_rtv(&rtv_tex, [9.0, 18.0, 27.0, 1.0], &[]).unwrap() };
    l3.barrier_list_with(&[Barrier::Transition(tex_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)], true, true);
    l3.set_descriptor_heaps(heap.gpu_base, 0);
    l3.compute_root.set_root_signature(cs_rs).unwrap();
    l3.compute_root.set_table(0, heap.gpu_base).unwrap();
    l3.compute_root.set_descriptor(1, uav_out.gpu_va).unwrap();
    l3.set_compute_pipeline(cs_pso);
    l3.dispatch(1, 1, 1).unwrap();
    assert!(stats::c::cs_mid_qb.load(Relaxed) > mid0, "Dispatch after narrowed Blit must upgrade compute sync");
    assert_eq!(l3.bar.enc_sync_after, MTLStages::All);
    assert!(!l3.bar.enc_blit_only_sync);

    l3.barrier_list_with(
        &[
            Barrier::Transition(safe_id, STATE_COPY_DEST, STATE_COPY_SOURCE),
            Barrier::Transition(uav_id, STATE_UAV, STATE_COPY_SOURCE),
            Barrier::Transition(clear_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    let fp4 = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l3.copy_buffer(&rb_safe, 0, &safe_dst, 0, 16).unwrap();
    l3.copy_buffer(&rb_uav, 0, &uav_out, 0, 64).unwrap();
    l3.copy_texture_region(CopyLoc::Footprint(&rb_clear, fp4), [0, 0, 0], CopyLoc::Subresource(&clear_only_tex, 0), None).unwrap();
    l3.close().unwrap();

    // Guardrail 2: a `Blit`-first list copying into an un-transitioned `DEFAULT` buffer (`COMMON -> COPY_DEST`)
    // MUST keep `after = MTLStages::All` (`before = MTLStages::Blit`, `!list_cs_unwaited_frag`)!
    let mut l4 = CommandList::new(ListType::Direct).unwrap();
    l4.opts.opt_empty_enc_elide = true;
    l4.begin(&a4).unwrap();
    l4.copy_buffer(&common_dst, 0, &up_buf, 0, 16).unwrap();
    assert_eq!(l4.bar.enc_sync_after, MTLStages::All, "implicit COMMON -> COPY_DEST Blit must wait on All (incl. Fragment)");
    assert!(l4.bar.enc_blit_only_sync);
    assert!(!l4.bar.list_cs_unwaited_frag);
    l4.close().unwrap();

    q.submit(&[&l_clear.cb, &l3.cb, &l4.cb]);
    q.wait_idle();

    assert_eq!(buf_words(&rb_safe, 1)[0], 0x55aa_1234);
    assert_eq!(buf_words(&rb_clear, 1)[0], 0xff00_00ff);
    let vals: Vec<f32> = buf_words(&rb_uav, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&vals[..], &[9.0, 18.0, 27.0, 1.0]);
}

#[test]
fn uav0_clear_overlap_narrows_post_clear_dispatch_wait_and_coalesces_layered_and_mrt_clears_on_gpu() {
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
    let cs_pso = Arc::new(ComputePipeline::new(cs_rs.clone(), &cs_dxil, "g33_stage_b_cs").unwrap());

    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let uav_gfx_rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![RootParameter {
            kind: RootParameterKind::Descriptor(
                RootDescriptorKind::Uav,
                RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
            ),
            visibility: 0,
        }],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();
    let pso_base = GraphicsPipeline::new(
        uav_gfx_rs.clone(),
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
        "g33_stage_b_uav_gfx",
    )
    .unwrap();
    let pso_writes_uav = Arc::new(GraphicsPipeline {
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
        pre_raster_reads: crate::metal::pipeline::READS_BUF,
        vs_reads_cbv_or_ia: true,
        vs_writes_uav: false,
        ps_writes_uav: true,
        ps_read_classes: pso_base.ps_read_classes,
        ps_srv_uav_classes: pso_base.ps_srv_uav_classes,
        writes_uav: true,
        uses_uav: true,
        sm66_pre_raster_reads: crate::metal::pipeline::READS_BUF,
        sm66_vs_reads_cbv_or_ia: true,
        sm66_writes_uav: true,
        sm66_narrowed: false,
        vs_access: crate::metal::pipeline::ComputeAccess::opaque(),
        ps_access: None,
        vs_hk_profile: pso_base.vs_hk_profile,
        ps_hk_profile: pso_base.ps_hk_profile,
        icb_desc: None,
        icb_pso: std::sync::OnceLock::new(),
    });
    assert!(pso_writes_uav.writes_uav);

    let q = Queue::new(ListType::Direct, "g33 stage B queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.opts.opt_uav0_clear_overlap = true;
    l.opts.opt_clear_keep_smart = true;

    // 4-slice `Texture2DArray` DSV (`8x8x4`) + 2 non-ShaderWrite RTVs (`8x8`) + 1 single DSV (`8x8`).
    let ds_arr_desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 8,
        Height: 8,
        DepthOrArraySize: 4,
        MipLevels: 1,
        Format: D32_S8,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_DEPTH_STENCIL,
        ..Default::default()
    };
    let ds_arr = Resource::new_committed(&ds_arr_desc, &test_heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "g33 ds_arr").unwrap();
    let rt_a = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt_b = test_texture(8, 8, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let ds_single = test_texture(8, 8, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let tex_srv = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_buf0 = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let uav_buf1 = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);

    let rb_arr: [Resource; 4] = std::array::from_fn(|_| test_buffer(256 * 8, heap_type::READBACK, 0));
    let rb_rta = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rb_rtb = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rb_dss = test_buffer(256 * 8, heap_type::READBACK, 0);
    let rb_uav1 = test_buffer(64, heap_type::READBACK, 0);

    for r in [&ds_arr, &rt_a, &rt_b, &ds_single, &tex_srv, &uav_buf0, &uav_buf1] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g33 stage B heap").unwrap();
    unsafe {
        let e0 = views::srv_entry(Some(&tex_srv), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.set_descriptor_heaps(heap.gpu_base, 0);

    // Populate `tex_srv` via compute clear, transition to SRV, then queue 7 non-compute clears:
    // - 4 slices of `ds_arr` (cleared in reverse order `3, 2, 1, 0` to verify slice sorting!)
    // - `rt_a` (red), `rt_b` (blue), and `ds_single` (`depth = 0.625`)
    let rtv_srv = TargetView::rtv(&tex_srv, None).unwrap();
    unsafe { l.clear_rtv(&rtv_srv, [3.0, 6.0, 9.0, 1.0], &[]).unwrap() };
    l.barrier_list_with(
        &[Barrier::Transition(&tex_srv as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)],
        true,
        true,
    );
    for s in (0..4u32).rev() {
        let v = TargetView::dsv(&ds_arr, Some(&[D32_S8, 4, 0, 0, s, 1])).unwrap();
        unsafe { l.clear_dsv(&v, true, 0.25, true, 0x11, &[]).unwrap() };
    }
    let rtv_a = TargetView::rtv(&rt_a, None).unwrap();
    let rtv_b = TargetView::rtv(&rt_b, None).unwrap();
    let dsv_s = TargetView::dsv(&ds_single, None).unwrap();
    unsafe {
        l.clear_rtv(&rtv_a, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
        l.clear_rtv(&rtv_b, [0.0, 0.0, 1.0, 1.0], &[]).unwrap();
        l.clear_dsv(&dsv_s, true, 0.625, false, 0, &[]).unwrap();
    }
    assert_eq!(l.clears.pending_clears.len(), 7);

    // `L4#0`: run Compute `Dispatch` #0 (opens `L4#0` consuming `FullWhy::ListStart` so `enc_sync_after == All`).
    l.compute_root.set_root_signature(cs_rs.clone()).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_buf0.gpu_va).unwrap();
    l.set_compute_pipeline(cs_pso.clone());
    l.dispatch(1, 1, 1).unwrap();
    assert!(!l.bar.uav_gfx_dirty);

    // `Barrier::Uav(0)` flushes all 7 non-compute clears:
    // - 4 slices of `ds_arr` merge into 1 layered clear pass (`clear_pass_merged += 3`)
    // - `rt_a` + `rt_b` + `ds_single` merge into 1 MRT+DSV clear pass (`clear_pass_merged += 2`)
    // Total: 2 clear passes instead of 7 (`clear_pass_merged == 5`), AND `pend_after == Dispatch`!
    let cp0 = stats::c::clear_pass.load(Relaxed);
    let merged0 = stats::c::clear_pass_merged.load(Relaxed);
    let ov0 = stats::c::uav0_clear_overlap.load(Relaxed);
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(stats::c::clear_pass.load(Relaxed) >= cp0 + 2, "7 clears must coalesce into 2 render passes");
    assert_eq!(stats::c::clear_pass_merged.load(Relaxed) - merged0, 5, "expected 5 merged clear passes");
    assert_eq!(stats::c::uav0_clear_overlap.load(Relaxed) - ov0, 1);
    assert_eq!(
        l.bar.pend_after,
        MTLStages::Dispatch,
        "post-Uav(0) pend_after must narrow to Dispatch so L4#5 overlaps CLEAR passes"
    );

    // `L4#5`: Dispatch #1 opens a new compute encoder waiting ONLY on `Dispatch` (NOT `Fragment`!),
    // while `l.written` retains `ds_single` so the subsequent render pass still synchronizes `qb_written`!
    l.compute_root.set_descriptor(1, uav_buf1.gpu_va).unwrap();
    l.dispatch(1, 1, 1).unwrap();
    assert_eq!(l.bar.enc_sync_after, MTLStages::Dispatch);
    let ds_single_id = &ds_single as *const Resource as usize;
    assert!(l.bar.written.contains(&ds_single_id), "cleared DSV must remain in written until a Fragment wait");

    // Attach `ds_single` in a UAV-writing draw (`znear = 0.125 < 0.625` -> overwrites `ds_single` to `0.125`),
    // verifying `qb_written` orders the render pass behind the standalone `CLEAR` pass AND sets `uav_gfx_dirty = true`!
    let qbw0 = stats::c::qb_written.load(Relaxed);
    l.gfx_root.set_root_signature(uav_gfx_rs).unwrap();
    l.gfx_root.set_descriptor(0, uav_buf0.gpu_va).unwrap();
    l.set_render_targets(Vec::new(), Some(dsv_s));
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso_writes_uav);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 8.0, znear: 0.125, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(stats::c::qb_written.load(Relaxed) > qbw0, "draw pass attaching cleared DSV must emit qb_written");
    assert!(l.bar.uav_gfx_dirty, "UAV-writing draw must mark uav_gfx_dirty");

    // Guardrail: if we close the render pass, open a compute encoder that does NOT wait on `Fragment`,
    // queue another DSV clear, and hit `Barrier::Uav(0)` while `uav_gfx_dirty == true`, it MUST NOT drop `Fragment`!
    l.end_encoder();
    assert!(l.bar.uav_gfx_dirty);
    unsafe { l.clear_dsv(&dsv_s, true, 0.5, false, 0, &[]).unwrap() };
    l.dispatch(1, 1, 1).unwrap();
    assert!(l.bar.uav_gfx_dirty, "compute encoder without Fragment wait must keep uav_gfx_dirty == true");
    l.barrier_list_with(&[Barrier::Uav(0)], true, false);
    assert!(
        l.bar.pend_after.contains(MTLStages::Fragment | PRE_RASTER),
        "when uav_gfx_dirty is true, Barrier::Uav(0) must preserve Fragment | PRE_RASTER in pend_after"
    );

    let ds_arr_id = &ds_arr as *const Resource as usize;
    let rt_a_id = &rt_a as *const Resource as usize;
    let rt_b_id = &rt_b as *const Resource as usize;
    let uav1_id = &uav_buf1 as *const Resource as usize;
    l.barrier_list_with(
        &[
            Barrier::Transition(ds_arr_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE),
            Barrier::Transition(rt_a_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(rt_b_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
            Barrier::Transition(ds_single_id, STATE_DEPTH_WRITE, STATE_COPY_SOURCE),
            Barrier::Transition(uav1_id, STATE_UAV, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    let fp_rgba = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
    let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 8, height: 8, depth: 1, row_pitch: 256, _pad: 0 };
    for s in 0..4u32 {
        // Plane 0 subresource index for mip 0, slice `s` on `MipLevels=1, ArraySize=4` is `s`.
        l.copy_texture_region(CopyLoc::Footprint(&rb_arr[s as usize], fp_d), [0, 0, 0], CopyLoc::Subresource(&ds_arr, s), None).unwrap();
    }
    l.copy_texture_region(CopyLoc::Footprint(&rb_rta, fp_rgba), [0, 0, 0], CopyLoc::Subresource(&rt_a, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_rtb, fp_rgba), [0, 0, 0], CopyLoc::Subresource(&rt_b, 0), None).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_dss, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds_single, 0), None).unwrap();
    l.copy_buffer(&rb_uav1, 0, &uav_buf1, 0, 64).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    for (s, rb) in rb_arr.iter().enumerate() {
        let d = f32::from_bits(buf_words(rb, 1)[0]);
        assert!((d - 0.25).abs() < 1e-5, "layered DSV slice {s} depth mismatch: got {d}");
    }
    assert_eq!(buf_words(&rb_rta, 1)[0], 0xff00_00ff, "coalesced rt_a must be red");
    assert_eq!(buf_words(&rb_rtb, 1)[0], 0xffff_0000, "coalesced rt_b must be blue");
    let ds_final = f32::from_bits(buf_words(&rb_dss, 1)[0]);
    assert!((ds_final - 0.5).abs() < 1e-5, "final ds_single depth mismatch: got {ds_final}");
    let u1: Vec<f32> = buf_words(&rb_uav1, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&u1[..], &[3.0, 6.0, 9.0, 1.0]);
}

#[test]
fn shader_immutable_opt_compiles_specialized_cs_and_gfx_psos_with_salted_cache_keys_on_gpu() {
    use crate::metal::descriptor::{heap_kind, DescriptorHeap};
    use crate::metal::pipeline::{pso_cache, ComputePipeline, GraphicsDesc, GraphicsPipeline};
    use crate::metal::views;
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
        RootDescriptorKind, RootParameter, RootParameterKind, RootSignature,
        RootSignatureDesc, RootSignatureVersion, StaticSampler,
    };

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    const RGBA32_FLOAT: u32 = 2;

    // 1) Verify `pso_cache().library_from_source_with_opts(..., true)` compiles fast-math MSL cleanly.
    let fast_lib = pso_cache()
        .unwrap()
        .library_from_source_with_opts(
            "g33_fast_math_probe",
            "#include <metal_stdlib>\nusing namespace metal;\nkernel void g33_noop(device uint* out [[buffer(0)]]) { out[0] = 42u; }\n",
            true,
        )
        .unwrap();
    assert!(fast_lib.newFunctionWithName(&objc2_foundation::NSString::from_str("g33_noop")).is_some());

    // 2) Compile `immut_opt = false` vs `immut_opt = true` `ComputePipeline` and `GraphicsPipeline`
    //    and verify salted `pso_cache` keys keep them isolated while caching repeat lookups!
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

    let cs_std = ComputePipeline::new_with_all_opts(cs_rs.clone(), &cs_dxil, "g33_cs_std", true, false).unwrap();
    let cs_imm1 = ComputePipeline::new_with_all_opts(cs_rs.clone(), &cs_dxil, "g33_cs_imm", true, true).unwrap();
    let cs_imm2 = ComputePipeline::new_with_all_opts(cs_rs.clone(), &cs_dxil, "g33_cs_imm2", true, true).unwrap();
    let cs_std_ptr = &*cs_std.pso as *const _ as usize;
    let cs_imm1_ptr = &*cs_imm1.pso as *const _ as usize;
    let cs_imm2_ptr = &*cs_imm2.pso as *const _ as usize;
    assert_ne!(cs_std_ptr, cs_imm1_ptr, "IMMUTABLE_OPT_KEY_SALT must isolate specialized CS PSO from standard CS PSO");
    assert_eq!(cs_imm1_ptr, cs_imm2_ptr, "repeat specialized CS PSO creation must hit pso_cache");

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
    let gfx_desc = GraphicsDesc {
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
    };
    let gfx_std = GraphicsPipeline::new_with_all_opts(gfx_rs.clone(), &gfx_desc, "g33_gfx_std", true, false).unwrap();
    let gfx_imm = Arc::new(GraphicsPipeline::new_with_all_opts(gfx_rs.clone(), &gfx_desc, "g33_gfx_imm", true, true).unwrap());
    assert_ne!(
        &*gfx_std.pso as *const _ as usize,
        &*gfx_imm.pso as *const _ as usize,
        "IMMUTABLE_OPT_KEY_SALT must isolate specialized GFX PSO from standard GFX PSO"
    );
    // Also verify lazy ICB variant compiles cleanly with immutable specialization!
    let _ = gfx_imm.icb_pso().unwrap();

    // 3) Execute specialized CS + GFX PSOs on the real Metal 4 GPU and verify exact output!
    let q = Queue::new(ListType::Direct, "g33 stage C queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();

    let tex = test_texture(4, 1, RGBA32_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS);
    let ds = test_texture(4, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
    let uav_buf = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_uav = test_buffer(64, heap_type::READBACK, 0);
    let rb_d = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&tex, &ds, &uav_buf, &rb_uav, &rb_d] {
        r.register_ptr();
    }
    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 4, true, "g33 stage C heap").unwrap();
    unsafe {
        let e0 = views::srv_entry(Some(&tex), None).unwrap();
        std::ptr::copy_nonoverlapping(e0.as_ptr(), heap.cpu_base as *mut u8, 24);
    }

    let _stats_guard = gpu_stats_lock();
    l.begin(&alloc).unwrap();
    l.set_descriptor_heaps(heap.gpu_base, 0);
    let rtv = TargetView::rtv(&tex, None).unwrap();
    let dsv = TargetView::dsv(&ds, None).unwrap();
    unsafe {
        l.clear_rtv(&rtv, [12.0, 24.0, 36.0, 1.0], &[]).unwrap();
        l.clear_dsv(&dsv, true, 0.8, false, 0, &[]).unwrap();
    }
    l.barrier_list_with(
        &[Barrier::Transition(&tex as *const Resource as usize, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)],
        true,
        true,
    );
    l.compute_root.set_root_signature(cs_rs).unwrap();
    l.compute_root.set_table(0, heap.gpu_base).unwrap();
    l.compute_root.set_descriptor(1, uav_buf.gpu_va).unwrap();
    l.set_compute_pipeline(Arc::new(cs_imm1));
    l.dispatch(1, 1, 1).unwrap();

    l.gfx_root.set_root_signature(gfx_rs).unwrap();
    l.gfx_root.set_constants(0, 0, &[1, 2]).unwrap();
    l.set_render_targets(Vec::new(), Some(dsv));
    l.gfx.topology = 4;
    l.set_graphics_pipeline(gfx_imm);
    l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 4.0, height: 4.0, znear: 0.3, zfar: 1.0 }]);
    l.draw(3, 1, 0, 0).unwrap();

    l.barrier_list_with(
        &[
            Barrier::Transition(&uav_buf as *const Resource as usize, STATE_UAV, STATE_COPY_SOURCE),
            Barrier::Transition(&ds as *const Resource as usize, STATE_DEPTH_WRITE, STATE_COPY_SOURCE),
        ],
        true,
        true,
    );
    let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_buffer(&rb_uav, 0, &uav_buf, 0, 64).unwrap();
    l.copy_texture_region(CopyLoc::Footprint(&rb_d, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
    l.close().unwrap();

    q.submit(&[&l.cb]);
    q.wait_idle();

    let vals: Vec<f32> = buf_words(&rb_uav, 4).into_iter().map(f32::from_bits).collect();
    assert_eq!(&vals[..], &[12.0, 24.0, 36.0, 1.0]);
    let d0 = f32::from_bits(buf_words(&rb_d, 1)[0]);
    assert!((d0 - 0.3).abs() < 1e-5, "specialized GFX PSO depth mismatch: got {d0}");
}

#[test]
fn fast_enc_state_deduplicates_tlab_uploads_draw_params_and_cross_pso_render_state_on_gpu() {
    use crate::metal::indirect::{arg, IndirectLayout};
    use crate::metal::pipeline::{GraphicsDesc, GraphicsPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::{
        flags as rs_flags, RootDescriptor, RootDescriptorKind, RootParameter,
        RootParameterKind, RootSignature, RootSignatureDesc, RootSignatureVersion,
    };
    use std::sync::atomic::Ordering::Relaxed;

    if crate::metal::device().is_none() || !dxil_metal::converter_available() {
        return;
    }
    let vs_dxil = sign_dxil(include_bytes!("../../../../../dxil-metal/tests/fixtures/entry_v2.vs.dxil"));
    let rs = RootSignature::new(RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: vec![
            RootParameter {
                kind: RootParameterKind::Constants {
                    shader_register: 0,
                    register_space: 0,
                    num_32bit_values: 2,
                },
                visibility: 0,
            },
            RootParameter {
                kind: RootParameterKind::Descriptor(
                    RootDescriptorKind::Cbv,
                    RootDescriptor { shader_register: 1, register_space: 0, flags: 0 },
                ),
                visibility: 0,
            },
        ],
        static_samplers: Vec::new(),
        flags: rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    })
    .unwrap();

    let make_desc = |cmp: MTLCompareFunction, mask: u32| GraphicsDesc {
        vs: &vs_dxil,
        ps: None,
        blend: BlendDesc::default(),
        sample_mask: mask,
        raster: RasterState { cull: MTLCullMode::None, ..RasterState::default() },
        depth_stencil: DepthStencilState {
            depth_compare: cmp,
            depth_write: true,
            ..DepthStencilState::default()
        },
        input_layout: Vec::new(),
        strip_cut: 0,
        topology_type: 3,
        rtv_formats: Vec::new(),
        dsv_format: D32_S8,
        sample_count: 1,
    };
    // `pso_a` and `pso_b` are distinct `GraphicsPipeline`s (`immut_opt = false` vs `true`, so distinct
    // `MTLRenderPipelineState`s!) that share identical `RasterState` + `DepthStencilState` (`Less`).
    // `pso_always` has `MTLCompareFunction::Always`.
    let pso_a = Arc::new(
        GraphicsPipeline::new_with_all_opts(rs.clone(), &make_desc(MTLCompareFunction::Less, u32::MAX), "g33_pso_a", true, false)
            .unwrap(),
    );
    let pso_b = Arc::new(
        GraphicsPipeline::new_with_all_opts(rs.clone(), &make_desc(MTLCompareFunction::Less, u32::MAX), "g33_pso_b", true, true)
            .unwrap(),
    );
    let pso_always = Arc::new(GraphicsPipeline::new(rs.clone(), &make_desc(MTLCompareFunction::Always, u32::MAX), "g33_pso_always").unwrap());
    assert_ne!(&*pso_a.pso as *const _ as usize, &*pso_b.pso as *const _ as usize);

    let layout_draw = IndirectLayout::new(24, &[(arg::CONSTANT, [0, 0, 2]), (arg::DRAW, [0; 3])]).unwrap();
    let args_up = test_buffer(128, heap_type::UPLOAD, 0);
    unsafe {
        let p = args_up.buffer().unwrap().contents().as_ptr() as *mut u32;
        let cmd: [u32; 6] = [1, 2, 3, 1, 0, 0];
        std::ptr::copy_nonoverlapping(cmd.as_ptr(), p, 6);
        *p.add(16) = 1; // count = 1 at offset 64
    }

    let q = Queue::new(ListType::Direct, "g33 stage D queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let _stats_guard = gpu_stats_lock();

    let run_pass = |fast_on: bool| -> Vec<f32> {
        let mut l = CommandList::new(ListType::Direct).unwrap();
        l.set_fast_enc_state(fast_on);
        l.opts.opt_icb_batch = false;

        let ds = test_texture(8, 4, D32_S8, resource_flags::ALLOW_DEPTH_STENCIL);
        let rb_d = test_buffer(256 * 4, heap_type::READBACK, 0);
        let dsv = TargetView::dsv(&ds, None).unwrap();

        l.begin(&alloc).unwrap();
        unsafe { l.clear_dsv(&dsv, true, 0.9, false, 0, &[]).unwrap() };

        // 1) RootState byte-exact deduplication + `upload_draw_params` ring deduplication:
        l.gfx_root.set_root_signature(rs.clone()).unwrap();
        l.gfx_root.set_constants(0, 0, &[10, 20]).unwrap();
        l.gfx_root.set_descriptor(1, args_up.gpu_va).unwrap();
        let root_va0 = l.flush_root(false).unwrap();
        let dp_bytes0 = [3u32, 1, 0, 0, 0].map(u32::to_le_bytes).concat();
        let dp_va0 = l.upload_draw_params(&dp_bytes0).unwrap();
        let skip_before_root = stats::c::enc_state_skip.load(Relaxed);
        // Re-setting identical constants, descriptor, & draw_params must NOT mark `dirty` or upload!
        l.gfx_root.set_constants(0, 0, &[10, 20]).unwrap();
        l.gfx_root.set_descriptor(1, args_up.gpu_va).unwrap();
        let root_va1 = l.flush_root(false).unwrap();
        let dp_va1 = l.upload_draw_params(&dp_bytes0).unwrap();
        if fast_on {
            assert_eq!(root_va1, root_va0, "identical RootState writes must reuse TLAB VA");
            assert_eq!(dp_va1, dp_va0, "identical RuntimeDrawParams must reuse ring upload VA");
            assert_eq!(stats::c::enc_state_skip.load(Relaxed) - skip_before_root, 3);
        }
        // Changing a constant or draw_params MUST upload fresh ring data!
        l.gfx_root.set_constants(0, 0, &[10, 21]).unwrap();
        let root_va2 = l.flush_root(false).unwrap();
        let dp_bytes2 = [3u32, 2, 0, 0, 0].map(u32::to_le_bytes).concat();
        let dp_va2 = l.upload_draw_params(&dp_bytes2).unwrap();
        assert_ne!(root_va2, root_va0, "changed RootState constant must upload a fresh TLAB");
        assert_ne!(dp_va2, dp_va0, "changed RuntimeDrawParams must upload a fresh ring entry");

        // 2) Immediate `begin_draw` across `pso_a -> pso_b` sharing identical RasterState, DepthStencilState,
        //    viewport, scissor, blend, and stencil_ref:
        l.set_render_targets(Vec::new(), Some(dsv));
        l.gfx.topology = 4;
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 4.0, znear: 0.25, zfar: 1.0 }]);
        l.set_scissors(vec![Rect { left: 0, top: 0, right: 4, bottom: 4 }]);
        l.set_graphics_pipeline(pso_a.clone());
        l.draw(3, 1, 0, 0).unwrap();

        let skip_before_d2 = stats::c::enc_state_skip.load(Relaxed);
        l.set_graphics_pipeline(pso_b.clone()); // switches PSO, keeps raster + DS + viewport + scissor + stencil + blend!
        l.draw(3, 1, 0, 0).unwrap();
        if fast_on {
            // Skips: 6 (`raster`, `ds`, `stencil_ref`, `blend`, `viewport`, `scissor`)!
            assert!(
                stats::c::enc_state_skip.load(Relaxed) >= skip_before_d2 + 6,
                "switching pso_a -> pso_b with identical fixed/dynamic state must skip >= 6 redundant state calls"
            );
        }

        // 3) Enable `opt_icb_batch` for the right half `[4..8, 0..4]`: first draw at `znear = 0.15` (`pso_b`),
        //    then overwrite via `ExecuteIndirect` with `pso_always` (`Always`) at `znear = 0.45`,
        //    verifying `flush_draw_batch` + `EncRenderCache` honor DS & scissor/viewport changes!
        l.opts.opt_icb_batch = true;
        l.set_scissors(vec![Rect { left: 4, top: 0, right: 8, bottom: 4 }]);
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 4.0, znear: 0.15, zfar: 1.0 }]);
        l.draw(3, 1, 0, 0).unwrap();

        l.set_graphics_pipeline(pso_always.clone());
        l.set_viewports(vec![MTLViewport { originX: 0.0, originY: 0.0, width: 8.0, height: 4.0, znear: 0.45, zfar: 1.0 }]);
        l.execute_indirect(&layout_draw, 1, args_up.gpu_va, None, Some(args_up.gpu_va + 64)).unwrap();

        l.barrier_list_with(
            &[Barrier::Transition(&ds as *const Resource as usize, STATE_DEPTH_WRITE, STATE_COPY_SOURCE)],
            true,
            true,
        );
        let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 8, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_d, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
        l.close().unwrap();

        q.submit(&[&l.cb]);
        q.wait_idle();
        alloc.reset();

        let words = buf_words(&rb_d, 64 * 4);
        let mut out = Vec::with_capacity(32);
        for y in 0..4 {
            for x in 0..8 {
                out.push(f32::from_bits(words[y * 64 + x]));
            }
        }
        out
    };

    let ref_d = run_pass(false);
    let opt_d = run_pass(true);
    for y in 0..4 {
        for x in 0..8 {
            let want = if x < 4 { 0.25 } else { 0.45 };
            let got_ref = ref_d[y * 8 + x];
            let got_opt = opt_d[y * 8 + x];
            assert!((got_ref - want).abs() < 1e-5, "ref ({x},{y}): got {got_ref}, want {want}");
            assert_eq!(got_opt.to_bits(), got_ref.to_bits(), "AW2_FAST_ENC_STATE=1 pixel ({x},{y}) mismatch");
        }
    }
}

#[test]
fn partial_barrier_stage_selection_and_fallbacks_on_gpu() {
    use crate::mesh::{MeshDesc, MeshPipeline};
    use crate::state::{BlendDesc, DepthStencilState, RasterState};
    use dxil_metal::root_signature::RootSignature;
    use std::sync::atomic::Ordering::Relaxed;

    let Some((vs_rs, vs_pso)) = build_g48_vs_srv_ps_cb0_pso("partial_bar_vs") else { return };
    let as_dxil = sign_dxil(include_bytes!("../../../../tests/fixtures/mesh.as.dxil"));
    let ms_dxil = sign_dxil(include_bytes!("../../../../tests/fixtures/mesh.ms.dxil"));
    let ms_direct_dxil = sign_dxil(include_bytes!("../../../../tests/fixtures/mesh_direct.ms.dxil"));
    let ps_dxil = sign_dxil(include_bytes!("../../../../tests/fixtures/mesh.ps.dxil"));
    let mesh_rs = RootSignature::from_blob(&as_dxil).unwrap();
    let mut blend = BlendDesc::default();
    blend.RenderTarget[0].RenderTargetWriteMask = 0xf;
    let ms_only_pso = Arc::new(
        MeshPipeline::new(
            mesh_rs.clone(),
            &MeshDesc {
                amplification: None,
                mesh: &ms_direct_dxil,
                pixel: Some(&ps_dxil),
                blend,
                sample_mask: u32::MAX,
                raster: RasterState { cull: MTLCullMode::None, ..Default::default() },
                depth_stencil: DepthStencilState::default(),
                rtv_formats: vec![RGBA8_UNORM],
                dsv_format: 0,
                sample_count: 1,
            },
            "partial_bar_ms_only",
        )
        .unwrap(),
    );
    let as_ms_pso = Arc::new(
        MeshPipeline::new(
            mesh_rs.clone(),
            &MeshDesc {
                amplification: Some(&as_dxil),
                mesh: &ms_dxil,
                pixel: Some(&ps_dxil),
                blend,
                sample_mask: u32::MAX,
                raster: RasterState { cull: MTLCullMode::None, ..Default::default() },
                depth_stencil: DepthStencilState::default(),
                rtv_formats: vec![RGBA8_UNORM],
                dsv_format: 0,
                sample_count: 1,
            },
            "partial_bar_as_ms",
        )
        .unwrap(),
    );

    let q = Queue::new(ListType::Direct, "partial_bar queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_uav = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_uav = test_texture(
        4,
        4,
        RGBA8_UNORM,
        resource_flags::ALLOW_UNORDERED_ACCESS | resource_flags::ALLOW_RENDER_TARGET,
    );
    let rb = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_uav, &rt, &tex_uav, &rb] {
        r.register_ptr();
    }
    let buf_id = &buf_uav as *const Resource as usize;
    let rt_id = &rt as *const Resource as usize;
    let tex_id = &tex_uav as *const Resource as usize;
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let raw_uav_desc = {
        let mut d = [0u8; 40];
        d[0..4].copy_from_slice(&39u32.to_le_bytes());
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[16..20].copy_from_slice(&16u32.to_le_bytes());
        d[32..36].copy_from_slice(&1u32.to_le_bytes());
        d
    };

    let _guard = gpu_stats_lock();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.begin(&alloc).unwrap();

    // 1) Compute -> Vertex: VS draw narrows pre-raster sync to `Vertex` (omits `Object | Mesh`)
    //    and records omitted stages in `enc_pre_unwaited_stages`.
    l.clear_uav(&buf_uav, Some(&raw_uav_desc), ClearValue::Uint([1; 4]), &[]).unwrap();
    l.barrier_list_with(&[Barrier::Transition(buf_id, STATE_UAV, STATE_NON_PIXEL_SRV)], true, false);
    l.gfx_root.set_root_signature(vs_rs.clone()).unwrap();
    l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
    l.gfx_root.set_descriptor(1, buf_uav.gpu_va).unwrap();
    l.set_render_targets(vec![rtv], None);
    l.gfx.topology = 4;
    l.set_graphics_pipeline(vs_pso.clone());
    l.set_viewports(vec![MTLViewport {
        originX: 0.0,
        originY: 0.0,
        width: 4.0,
        height: 4.0,
        znear: 0.0,
        zfar: 1.0,
    }]);
    l.draw(3, 1, 0, 0).unwrap();
    assert!(
        l.bar.enc_sync_before.contains(MTLStages::Vertex)
            && !l.bar.enc_sync_before.intersects(MTLStages::Object | MTLStages::Mesh),
        "VS draw must narrow pre-raster barrier to Vertex, got {:?}",
        l.bar.enc_sync_before
    );
    assert!(
        l.bar.enc_pre_unwaited_stages.contains(MTLStages::Object | MTLStages::Mesh),
        "omitted Object|Mesh stages must be tracked for mid-pass mesh draw sync"
    );

    // 2) Switching from VS draw to Mesh-only draw inside the SAME render pass triggers
    //    `ensure_pre_raster_stage_synced` for `Mesh` (leaving `Object` unwaited until AS+MS!).
    let mid0 = stats::c::rb_mid_qb.load(Relaxed);
    l.gfx_root.set_root_signature(mesh_rs.clone()).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 128, 0, 0]).unwrap();
    l.set_mesh_pipeline(ms_only_pso.clone());
    l.dispatch_mesh(1, 1, 1).unwrap();
    assert!(
        stats::c::rb_mid_qb.load(Relaxed) > mid0,
        "switching to Mesh draw in same pass must sync omitted Mesh stage"
    );
    assert!(
        !l.bar.enc_pre_unwaited_stages.contains(MTLStages::Mesh)
            && l.bar.enc_pre_unwaited_stages.contains(MTLStages::Object),
        "Mesh-only draw must consume Mesh and leave Object unwaited, got {:?}",
        l.bar.enc_pre_unwaited_stages
    );

    // 3) Switching to AS+MS draw inside the SAME render pass consumes `Object` too.
    l.set_mesh_pipeline(as_ms_pso.clone());
    l.dispatch_mesh(1, 1, 1).unwrap();
    assert!(
        l.bar.enc_pre_unwaited_stages.is_empty(),
        "AS+MS draw must consume remaining Object stage"
    );

    // 4) Compute -> Fragment only: `UAV -> PIXEL_SRV` targets Fragment without stalling Vertex.
    l.end_encoder();
    l.clear_uav(&tex_uav, None, ClearValue::Float([0.25, 0.5, 0.75, 1.0]), &[]).unwrap();
    l.barrier_list_with(&[Barrier::Transition(tex_id, STATE_UAV, STATE_PIXEL_SRV)], true, false);
    assert!(
        l.bar.split_pend.frag_after.contains(MTLStages::Dispatch)
            && l.bar.split_pend.vtx_after.is_empty(),
        "UAV -> PIXEL_SRV must target Fragment without pre-raster stall"
    );
    l.gfx_root.set_root_signature(vs_rs.clone()).unwrap();
    l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
    l.gfx_root.set_descriptor(1, buf_uav.gpu_va).unwrap();
    l.set_graphics_pipeline(vs_pso.clone());
    l.draw(3, 1, 0, 0).unwrap();
    assert_eq!(
        l.bar.enc_sync_before,
        MTLStages::Fragment,
        "Compute -> Fragment draw must narrow before-mask to Fragment only"
    );

    // 5) Compute -> Mesh-only opening a fresh render pass narrows before-mask to `Mesh` only;
    //    Compute -> AS+MS narrows to `Object | Mesh`.
    //    Use `begin_mesh(true)` (indirect) so pre-raster buffer reads consume the Compute->Mesh sync.
    l.end_encoder();
    l.clear_uav(&buf_uav, Some(&raw_uav_desc), ClearValue::Uint([2; 4]), &[]).unwrap();
    l.barrier_list_with(&[Barrier::Transition(buf_id, STATE_UAV, STATE_NON_PIXEL_SRV)], true, false);
    l.gfx_root.set_root_signature(mesh_rs.clone()).unwrap();
    l.gfx_root.set_constants(0, 0, &[0, 200, 0, 0]).unwrap();
    l.set_mesh_pipeline(ms_only_pso.clone());
    let (e_ms, p_ms) = l.begin_mesh(true).unwrap();
    crate::mesh::encode_dispatch(&e_ms, &p_ms, 1, 1, 1).unwrap();
    assert!(
        l.bar.enc_sync_before.contains(MTLStages::Mesh)
            && !l.bar.enc_sync_before.intersects(MTLStages::Vertex | MTLStages::Object),
        "Mesh-only pass must narrow before-mask to Mesh, got {:?}",
        l.bar.enc_sync_before
    );
    l.end_encoder();
    l.clear_uav(&buf_uav, Some(&raw_uav_desc), ClearValue::Uint([3; 4]), &[]).unwrap();
    l.barrier_list_with(&[Barrier::Transition(buf_id, STATE_UAV, STATE_NON_PIXEL_SRV)], true, false);
    l.set_mesh_pipeline(as_ms_pso.clone());
    let (e_as, p_as) = l.begin_mesh(true).unwrap();
    crate::mesh::encode_dispatch(&e_as, &p_as, 1, 1, 1).unwrap();
    assert!(
        l.bar.enc_sync_before.contains(MTLStages::Object | MTLStages::Mesh)
            && !l.bar.enc_sync_before.contains(MTLStages::Vertex),
        "AS+MS pass must narrow before-mask to Object|Mesh, got {:?}",
        l.bar.enc_sync_before
    );

    // 6) Draw -> Compute (Blit vs Dispatch) + read->read / same-state no-hazard elision.
    //    Consume step 5's leftover NON_PIXEL_SRV Dispatch obligation first so step 6 starts clean.
    l.end_encoder();
    l.bar.split_pend.disp_after = MTLStages::empty();
    l.barrier_list_with(
        &[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)],
        true,
        false,
    );
    assert!(
        l.bar.split_pend.blit_after.contains(MTLStages::Fragment)
            && l.bar.split_pend.disp_after.is_empty(),
        "RT -> COPY_SOURCE must target Blit without stalling Dispatch"
    );
    let fp = PlacedFootprint {
        offset: 0,
        format: RGBA8_UNORM,
        width: 4,
        height: 4,
        depth: 1,
        row_pitch: 256,
        _pad: 0,
    };
    l.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(&rt, 0), None)
        .unwrap();
    l.end_encoder();
    l.barrier_list_with(
        &[Barrier::Transition(rt_id, STATE_RENDER_TARGET, STATE_NON_PIXEL_SRV)],
        true,
        false,
    );
    assert!(
        l.bar.split_pend.disp_after.contains(MTLStages::Fragment)
            && l.bar.split_pend.blit_after.is_empty(),
        "RT -> NON_PIXEL_SRV must target Dispatch without stalling Blit"
    );
    let r2r0 = stats::c::rb_r2r_elided.load(Relaxed);
    let noop0 = stats::c::rb_noop.load(Relaxed);
    l.barrier_list_with(
        &[
            Barrier::Transition(tex_id, STATE_PIXEL_SRV, STATE_NON_PIXEL_SRV),
            Barrier::Transition(tex_id, STATE_PIXEL_SRV, STATE_PIXEL_SRV),
        ],
        true,
        false,
    );
    assert!(stats::c::rb_r2r_elided.load(Relaxed) > r2r0, "read->read must be elided");
    assert!(stats::c::rb_noop.load(Relaxed) > noop0, "same-state transition must be elided");

    // 7) Fallbacks: `Barrier::Uav`, `Barrier::Full` (aliasing), and unknown `COMMON` (`s0 == 0`) / `r == 0`.
    l.barrier_list_with(&[Barrier::Transition(0, STATE_COMMON, STATE_COPY_DEST)], true, false);
    assert_eq!(l.bar.split_pend.blit_after, MTLStages::All, "COMMON / r==0 must fall back to All");
    l.barrier_list_with(&[Barrier::Uav(buf_id)], true, false);
    assert!(!l.bar.split_pend.disp_after.is_empty(), "Barrier::Uav must arm Dispatch sync");
    l.barrier_list_with(&[Barrier::Full], true, false);
    assert!(l.bar.needs_barrier, "Barrier::Full (aliasing) must arm full barrier");

    l.close().unwrap();
    q.submit(&[&l.cb]);
    q.wait_idle();
    alloc.reset();
}

#[test]
fn split_barrier_begin_end_same_list_and_cross_list_on_gpu() {
    let Some((rs, pso)) = build_g48_vs_srv_ps_cb0_pso("split_bar") else { return };
    let q = Queue::new(ListType::Direct, "split_bar queue").unwrap();
    let alloc1 = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc2 = CommandAllocator::new(ListType::Direct).unwrap();
    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_srv = test_buffer(64, heap_type::UPLOAD, 0);
    let rt1 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt2 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rb1 = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb2 = test_buffer(256 * 4, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_srv, &rt1, &rt2, &rb1, &rb2] {
        r.register_ptr();
    }
    unsafe {
        let cp = cb_up.buffer().unwrap().contents().as_ptr() as *mut f32;
        *cp.add(0) = 0.0;
        *cp.add(1) = 1.0;
        *cp.add(2) = 0.0;
        *cp.add(3) = 1.0;
    }
    let rt1_id = &rt1 as *const Resource as usize;
    let rt2_id = &rt2 as *const Resource as usize;
    let rtv1 = TargetView::rtv(&rt1, None).unwrap();
    let rtv2 = TargetView::rtv(&rt2, None).unwrap();
    let fp = PlacedFootprint {
        offset: 0,
        format: RGBA8_UNORM,
        width: 4,
        height: 4,
        depth: 1,
        row_pitch: 256,
        _pad: 0,
    };

    // List 1: draw rt1 -> SplitBegin(rt1, RT->COPY_SOURCE) -> draw rt2 -> SplitEnd(rt1) -> copy rt1 ->
    //         SplitBegin(rt2, RT->COPY_SOURCE) (cross-list split!) -> close().
    let mut l1 = CommandList::new(ListType::Direct).unwrap();
    l1.begin(&alloc1).unwrap();
    l1.gfx_root.set_root_signature(rs.clone()).unwrap();
    l1.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
    l1.gfx_root.set_descriptor(1, buf_srv.gpu_va).unwrap();
    l1.set_render_targets(vec![rtv1], None);
    l1.gfx.topology = 4;
    l1.set_graphics_pipeline(pso.clone());
    l1.set_viewports(vec![MTLViewport {
        originX: 0.0,
        originY: 0.0,
        width: 4.0,
        height: 4.0,
        znear: 0.0,
        zfar: 1.0,
    }]);
    l1.draw(3, 1, 0, 0).unwrap();
    l1.barrier_list_with(
        &[Barrier::SplitBegin(rt1_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)],
        true,
        false,
    );
    assert_eq!(l1.bar.split_barriers.len(), 1, "SplitBegin must record pending split transition");
    assert!(
        l1.bar.split_pend.blit_after.is_empty(),
        "SplitBegin must not emit consumer barrier before SplitEnd"
    );
    // Unrelated render pass on rt2 runs between SplitBegin(rt1) and SplitEnd(rt1).
    l1.set_render_targets(vec![rtv2], None);
    l1.draw(3, 1, 0, 0).unwrap();
    // Unattached read-to-read SplitBegin must elide without splitting the open render pass!
    l1.barrier_list_with(
        &[Barrier::SplitBegin(rt1_id, STATE_PIXEL_SRV, STATE_NON_PIXEL_SRV)],
        true,
        false,
    );
    assert!(matches!(l1.enc, Encoder::Render(_)), "r2r SplitBegin must keep render pass open");
    // Now SplitEnd(rt1) consumes the recorded SplitBegin(rt1) with rt1's earlier gen!
    l1.barrier_list_with(
        &[Barrier::SplitEnd(rt1_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)],
        true,
        false,
    );
    assert!(l1.bar.split_barriers.is_empty(), "SplitEnd must consume matching SplitBegin");
    assert!(
        l1.bar.split_pend.blit_after.contains(MTLStages::Fragment),
        "SplitEnd must arm Fragment -> Blit"
    );
    l1.copy_texture_region(CopyLoc::Footprint(&rb1, fp), [0, 0, 0], CopyLoc::Subresource(&rt1, 0), None)
        .unwrap();
    // Cross-list split: SplitBegin(rt2) at end of L1, SplitEnd(rt2) at start of L2.
    l1.barrier_list_with(
        &[Barrier::SplitBegin(rt2_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)],
        true,
        false,
    );
    l1.close().unwrap();

    let mut l2 = CommandList::new(ListType::Direct).unwrap();
    l2.begin(&alloc2).unwrap();
    l2.barrier_list_with(
        &[Barrier::SplitEnd(rt2_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE)],
        true,
        false,
    );
    l2.copy_texture_region(CopyLoc::Footprint(&rb2, fp), [0, 0, 0], CopyLoc::Subresource(&rt2, 0), None)
        .unwrap();
    l2.close().unwrap();

    q.submit(&[&l1.cb, &l2.cb]);
    q.wait_idle();
    assert_eq!(buf_words(&rb1, 1)[0], 0xff00_ff00, "same-list split barrier copy must read green");
    assert_eq!(buf_words(&rb2, 1)[0], 0xff00_ff00, "cross-list split barrier copy must read green");
    alloc1.reset();
    alloc2.reset();
}

#[test]
fn list_start_explicit_blit_narrowing_and_cross_list_trailing_guard_on_gpu() {
    let q = Queue::new(ListType::Direct, "list_start_blit queue").unwrap();
    let alloc1 = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc2 = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc3 = CommandAllocator::new(ListType::Direct).unwrap();
    let alloc4 = CommandAllocator::new(ListType::Direct).unwrap();

    let up_tex = test_buffer(256 * 4, heap_type::UPLOAD, 0);
    let rt1 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let rt2 = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_common = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let tex_simul = test_texture(
        4,
        4,
        RGBA8_UNORM,
        resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_SIMULTANEOUS_ACCESS,
    );
    let uav_buf = test_buffer(64, heap_type::DEFAULT, resource_flags::ALLOW_UNORDERED_ACCESS);
    let rb_tex = test_buffer(256 * 4, heap_type::READBACK, 0);
    let rb_buf = test_buffer(64, heap_type::READBACK, 0);
    for r in [&up_tex, &rt1, &rt2, &tex_common, &tex_simul, &uav_buf, &rb_tex, &rb_buf] {
        r.register_ptr();
    }
    unsafe {
        let p = up_tex.buffer().unwrap().contents().as_ptr() as *mut u32;
        for row in 0..4 {
            for col in 0..4 {
                *p.add(row * 64 + col) = 0xff33_6699;
            }
        }
    }
    let rt1_id = &rt1 as *const Resource as usize;
    let rt2_id = &rt2 as *const Resource as usize;
    let common_id = &tex_common as *const Resource as usize;
    let simul_id = &tex_simul as *const Resource as usize;
    let uav_id = &uav_buf as *const Resource as usize;
    let fp = PlacedFootprint {
        offset: 0,
        format: RGBA8_UNORM,
        width: 4,
        height: 4,
        depth: 1,
        row_pitch: 256,
        _pad: 0,
    };
    let raw_uav_desc = {
        let mut d = [0u8; 40];
        d[0..4].copy_from_slice(&39u32.to_le_bytes());
        d[4..8].copy_from_slice(&1u32.to_le_bytes());
        d[16..20].copy_from_slice(&16u32.to_le_bytes());
        d[32..36].copy_from_slice(&1u32.to_le_bytes());
        d
    };

    // Part A: `L1` runs `Dispatch` (`clear_uav`), closes with a trailing narrowing transition
    // `Transition(uav_buf, UAV -> RENDER_TARGET)` (not consumed in `L1`).
    // `emit_close_trailing_producer_barriers()` MUST publish `Dispatch -> Blit` at `L1.close()`.
    let mut l1 = CommandList::new(ListType::Direct).unwrap();
    l1.begin(&alloc1).unwrap();
    l1.clear_uav(&uav_buf, Some(&raw_uav_desc), ClearValue::Uint([0xdead_beef; 4]), &[]).unwrap();
    l1.barrier_list_with(&[Barrier::Transition(uav_id, STATE_UAV, STATE_RENDER_TARGET)], true, false);
    l1.close().unwrap();

    // Part B: `L2` starts with `Transition(rt1, RENDER_TARGET -> COPY_DEST)` + `Transition(rt2, RENDER_TARGET -> COPY_DEST)`
    // + `Transition(uav_buf, RENDER_TARGET -> COPY_SOURCE)`.
    // Opening Blit (`rt1 <- up_tex`) MUST narrow `ListStart` to `Fragment -> Blit` (omitting `Dispatch`!).
    let mut l2 = CommandList::new(ListType::Direct).unwrap();
    l2.begin(&alloc2).unwrap();
    l2.barrier_list_with(
        &[
            Barrier::Transition(rt1_id, STATE_RENDER_TARGET, STATE_COPY_DEST),
            Barrier::Transition(rt2_id, STATE_RENDER_TARGET, STATE_COPY_DEST),
            Barrier::Transition(uav_id, STATE_RENDER_TARGET, STATE_COPY_SOURCE),
        ],
        true,
        false,
    );
    l2.copy_texture_region(CopyLoc::Subresource(&rt1, 0), [0, 0, 0], CopyLoc::Footprint(&up_tex, fp), None)
        .unwrap();
    assert_eq!(
        l2.bar.enc_sync_after,
        MTLStages::Fragment,
        "explicit RT->COPY_DEST list-start Blit must narrow after-mask to Fragment only"
    );
    assert_eq!(l2.bar.enc_sync_before, MTLStages::Blit);
    assert!(
        l2.bar.list_blit_unwaited_stages.contains(MTLStages::Dispatch),
        "omitted Dispatch stage must be tracked in list_blit_unwaited_stages"
    );
    // Second explicit RT->COPY_DEST Blit stays narrowed; copying `uav_buf` reads back `0xdead_beef`
    // safely because L1 emitted the trailing producer barrier at close!
    l2.copy_texture_region(CopyLoc::Subresource(&rt2, 0), [0, 0, 0], CopyLoc::Footprint(&up_tex, fp), None)
        .unwrap();
    assert!(l2.bar.list_blit_unwaited_stages.contains(MTLStages::Dispatch));
    l2.copy_buffer(&rb_buf, 0, &uav_buf, 0, 64).unwrap();
    // Copy rt1 back to readback to verify upload pixels too.
    l2.barrier_list_with(&[Barrier::Transition(rt1_id, STATE_COPY_DEST, STATE_COPY_SOURCE)], true, false);
    l2.copy_texture_region(CopyLoc::Footprint(&rb_tex, fp), [0, 0, 0], CopyLoc::Subresource(&rt1, 0), None)
        .unwrap();
    l2.close().unwrap();

    // Part C: `COMMON` (`s0 == 0`) and `ALLOW_SIMULTANEOUS_ACCESS` must taint and fall back to `All -> Blit`.
    let mut l3 = CommandList::new(ListType::Direct).unwrap();
    l3.begin(&alloc3).unwrap();
    l3.barrier_list_with(&[Barrier::Transition(common_id, STATE_COMMON, STATE_COPY_DEST)], true, false);
    l3.copy_texture_region(CopyLoc::Subresource(&tex_common, 0), [0, 0, 0], CopyLoc::Footprint(&up_tex, fp), None)
        .unwrap();
    assert_eq!(l3.bar.enc_sync_after, MTLStages::All, "COMMON s0 must fall back to All -> Blit");
    l3.close().unwrap();

    let mut l4 = CommandList::new(ListType::Direct).unwrap();
    l4.begin(&alloc4).unwrap();
    l4.barrier_list_with(
        &[Barrier::Transition(simul_id, STATE_RENDER_TARGET, STATE_COPY_DEST)],
        true,
        false,
    );
    l4.copy_texture_region(CopyLoc::Subresource(&tex_simul, 0), [0, 0, 0], CopyLoc::Footprint(&up_tex, fp), None)
        .unwrap();
    assert_eq!(
        l4.bar.enc_sync_after,
        MTLStages::All,
        "ALLOW_SIMULTANEOUS_ACCESS must fall back to All -> Blit"
    );
    l4.close().unwrap();

    q.submit(&[&l1.cb, &l2.cb, &l3.cb, &l4.cb]);
    q.wait_idle();
    assert_eq!(
        buf_words(&rb_buf, 1)[0],
        0xdead_beef,
        "trailing producer barrier must order L1 Dispatch before L2 narrowed Blit"
    );
    assert_eq!(buf_words(&rb_tex, 1)[0], 0xff33_6699, "narrowed list-start texture copy must match");
    alloc1.reset();
    alloc2.reset();
    alloc3.reset();
    alloc4.reset();
}

#[test]
fn occlusion_query_same_pass_reset_coalescing_and_resolve_on_gpu() {
    use crate::metal::query::{heap_type as qh_type, QueryHeap};
    use std::sync::atomic::Ordering::Relaxed;

    let Some((rs, pso)) = build_g48_vs_srv_ps_cb0_pso("occl_coalesce") else { return };
    let q = Queue::new(ListType::Direct, "occl_coalesce queue").unwrap();
    let alloc = CommandAllocator::new(ListType::Direct).unwrap();
    let cb_up = test_buffer(256, heap_type::UPLOAD, 0);
    let buf_srv = test_buffer(64, heap_type::UPLOAD, 0);
    let rt = test_texture(4, 4, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET);
    let dst_rb = test_buffer(64, heap_type::READBACK, 0);
    for r in [&cb_up, &buf_srv, &rt, &dst_rb] {
        r.register_ptr();
    }
    let rtv = TargetView::rtv(&rt, None).unwrap();
    let qh = QueryHeap::new(qh_type::OCCLUSION, 8).unwrap();
    let vis_buf = qh.visibility.as_ref().unwrap();
    unsafe {
        let vp = vis_buf.contents().as_ptr() as *mut u64;
        for i in 0..6 {
            *vp.add(i) = 555 + i as u64;
        }
    }

    let _guard = gpu_stats_lock();
    let qb_int0 = stats::c::qb_internal.load(Relaxed);
    let cs_enc0 = stats::c::compute_enc.load(Relaxed);

    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.begin(&alloc).unwrap();
    l.gfx_root.set_root_signature(rs.clone()).unwrap();
    l.gfx_root.set_descriptor(0, cb_up.gpu_va).unwrap();
    l.gfx_root.set_descriptor(1, buf_srv.gpu_va).unwrap();
    l.set_render_targets(vec![rtv], None);
    l.gfx.topology = 4;
    l.set_graphics_pipeline(pso.clone());
    l.set_viewports(vec![MTLViewport {
        originX: 0.0,
        originY: 0.0,
        width: 4.0,
        height: 4.0,
        znear: 0.0,
        zfar: 1.0,
    }]);

    // Queries 0, 1, 3 run consecutively on the same targets -> must coalesce into ONE Reset pass
    // with zero `FullWhy::Internal` barriers and zero `FILL_OCCL` compute encoders!
    // Slot 2 is untouched (must stay 557!).
    l.begin_occlusion(&qh, 0, false).unwrap();
    l.draw(3, 1, 0, 0).unwrap();
    l.draw(3, 1, 0, 0).unwrap();
    l.end_occlusion(&qh, 0).unwrap();

    let enc_ptr_0 = match &l.enc {
        Encoder::Render(e) => &**e as *const _ as usize,
        _ => panic!("expected open render pass after query 0"),
    };
    l.begin_occlusion(&qh, 1, false).unwrap();
    l.draw(3, 1, 0, 0).unwrap();
    l.end_occlusion(&qh, 1).unwrap();
    let enc_ptr_1 = match &l.enc {
        Encoder::Render(e) => &**e as *const _ as usize,
        _ => panic!("expected same render pass after query 1"),
    };
    assert_eq!(enc_ptr_0, enc_ptr_1, "consecutive occlusion queries must coalesce into one render pass");

    // Slot 3: enabled and disabled inside the same Reset pass with 0 draws -> resets to 0.
    l.begin_occlusion(&qh, 3, false).unwrap();
    l.end_occlusion(&qh, 3).unwrap();

    // Slot 4: spans TWO render passes (`Reset` pass 1 + `Accumulate` pass 2) -> accumulates 2 draws!
    l.begin_occlusion(&qh, 4, false).unwrap();
    l.draw(3, 1, 0, 0).unwrap();
    l.end_encoder();
    l.draw(3, 1, 0, 0).unwrap();
    l.end_occlusion(&qh, 4).unwrap();
    l.end_encoder();

    assert_eq!(
        stats::c::compute_enc.load(Relaxed) - cs_enc0,
        0,
        "coalesced occlusion queries must not open FILL_OCCL compute encoders"
    );

    // Slot 5: 0-draw BeginQuery..EndQuery while NO render pass is open -> zeroes slot via Blit
    // without emitting `FullWhy::Internal`.
    l.begin_occlusion(&qh, 5, false).unwrap();
    l.end_occlusion(&qh, 5).unwrap();

    // Resolve all 6 slots to readback buffer via stage-masked Blit (zero `FullWhy::Internal`).
    l.resolve_occlusion(&qh, 0, 6, &dst_rb, 0).unwrap();
    l.close().unwrap();

    assert_eq!(
        stats::c::qb_internal.load(Relaxed) - qb_int0,
        0,
        "occlusion queries and ResolveQueryData must emit zero FullWhy::Internal barriers"
    );

    q.submit(&[&l.cb]);
    q.wait_idle();
    alloc.reset();

    let got: [u64; 6] = unsafe {
        let p = dst_rb.buffer().unwrap().contents().as_ptr() as *const u64;
        [*p.add(0), *p.add(1), *p.add(2), *p.add(3), *p.add(4), *p.add(5)]
    };
    assert_eq!(got[1], 16, "slot 1 (1 draw on 4x4 RT) must count 16 samples, got {got:?}");
    assert_eq!(got[0], 32, "slot 0 (2 draws in same Reset pass) must count 32 samples, got {got:?}");
    assert_eq!(got[2], 557, "untouched slot 2 must remain 557, got {got:?}");
    assert_eq!(got[3], 0, "0-draw slot 3 in Reset pass must reset to 0, got {got:?}");
    assert_eq!(got[4], 32, "multi-pass slot 4 (Reset + Accumulate) must count 32 samples, got {got:?}");
    assert_eq!(got[5], 0, "0-draw slot 5 outside render pass must zero via Blit, got {got:?}");
}
