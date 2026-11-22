//! GPU test through the layer's command list: ClearUnorderedAccessView on
//! typed/raw buffer views and a 2D texture (with a rect), and
//! ResolveSubresource of a cleared 4x MSAA render target. Results are copied
//! to READBACK buffers and checked on the CPU. Skips without a Metal 4 device.

use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::metal::clear::ClearValue;
use d3d12_metal::metal::cmdlist::{CommandAllocator, CommandList, CopyLoc, TargetView};
use d3d12_metal::metal::queue::{ListType, Queue};
use d3d12_metal::metal::resource::Resource;
use d3d12_metal::types::{dim, heap_type, resource_flags, HeapProperties, Rect, ResourceDesc};
use objc2_metal::*;

const R32_TYPELESS: u32 = 39;
const R32_UINT: u32 = 42;
const RGBA8_UNORM: u32 = 28;

fn heap(ty: u32) -> HeapProperties {
    HeapProperties { Type: ty, ..Default::default() }
}

fn buffer(size: u64, ty: u32, flags: u32) -> Resource {
    let d = ResourceDesc { Dimension: dim::BUFFER, Width: size, Height: 1, DepthOrArraySize: 1, MipLevels: 1, SampleCount: 1, Layout: 1, Flags: flags, ..Default::default() };
    Resource::new_committed(&d, &heap(ty), MTLTextureUsage::empty(), "test buffer").unwrap()
}

fn texture(w: u64, h: u32, samples: u32, flags: u32) -> Resource {
    let d = ResourceDesc { Dimension: dim::TEXTURE2D, Width: w, Height: h, DepthOrArraySize: 1, MipLevels: 1, Format: RGBA8_UNORM, SampleCount: samples, Flags: flags, ..Default::default() };
    Resource::new_committed(&d, &heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "test texture").unwrap()
}

/// `D3D12_UNORDERED_ACCESS_VIEW_DESC` of a buffer.
fn buffer_uav(format: u32, first: u64, n: u32, flags: u32) -> [u8; 40] {
    let mut d = [0u8; 40];
    d[0..4].copy_from_slice(&format.to_le_bytes());
    d[4..8].copy_from_slice(&1u32.to_le_bytes());
    d[8..16].copy_from_slice(&first.to_le_bytes());
    d[16..20].copy_from_slice(&n.to_le_bytes());
    d[32..36].copy_from_slice(&flags.to_le_bytes());
    d
}

fn words(r: &Resource, n: usize) -> Vec<u32> {
    let b = r.buffer().unwrap();
    unsafe { std::slice::from_raw_parts(b.contents().as_ptr() as *const u32, n) }.to_vec()
}

fn run(f: impl FnOnce(&mut CommandList)) {
    let q = Queue::new(ListType::Direct, "test queue").unwrap();
    let a = CommandAllocator::new(ListType::Direct).unwrap();
    let mut l = CommandList::new(ListType::Direct).unwrap();
    l.begin(&a).unwrap();
    f(&mut l);
    l.close().unwrap();
    q.submit(&[&l.cb]);
    q.wait_idle();
}

fn readback_texture(l: &mut CommandList, rb: &Resource, t: &Resource, w: u32, h: u32) {
    let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: w, height: h, depth: 1, row_pitch: 256, _pad: 0 };
    l.copy_texture_region(CopyLoc::Footprint(rb, fp), [0, 0, 0], CopyLoc::Subresource(t, 0), None).unwrap();
}

#[test]
fn clear_uav_buffers_and_texture() {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return;
    }
    let ua = resource_flags::ALLOW_UNORDERED_ACCESS;
    let buf = buffer(256, heap_type::DEFAULT, ua);
    let rb = buffer(256, heap_type::READBACK, 0);
    let tex = texture(8, 8, 1, ua);
    let trb = buffer(256 * 8, heap_type::READBACK, 0);
    run(|l| {
        // Typed R32_UINT over the whole buffer, then raw words 4..12.
        l.clear_uav(&buf, Some(&buffer_uav(R32_UINT, 0, 64, 0)), ClearValue::Uint([5, 0, 0, 0]), &[]).unwrap();
        l.clear_uav(&buf, Some(&buffer_uav(R32_TYPELESS, 4, 8, 1)), ClearValue::Uint([0xDEAD_BEEF, 1, 2, 3]), &[]).unwrap();
        l.copy_buffer(&rb, 0, &buf, 0, 256).unwrap();
        // Whole texture (no desc) to 0 (bit-precise uint), then a float rect.
        l.clear_uav(&tex, None, ClearValue::Uint([0, 0, 0, 0]), &[]).unwrap();
        l.clear_uav(&tex, None, ClearValue::Float([1.0, 0.0, 0.5, 1.0]), &[Rect { left: 2, top: 2, right: 6, bottom: 4 }]).unwrap();
        readback_texture(l, &trb, &tex, 8, 8);
    });
    let w = words(&rb, 64);
    for (i, &x) in w.iter().enumerate() {
        assert_eq!(x, if (4..12).contains(&i) { 0xDEAD_BEEF } else { 5 }, "word {i}");
    }
    let px = words(&trb, 64 * 8);
    for y in 0..8 {
        for x in 0..8 {
            let want = if (2..6).contains(&x) && (2..4).contains(&y) { 0xff80_00ff } else { 0 };
            assert_eq!(px[y * 64 + x], want, "texel ({x},{y})");
        }
    }
}

#[test]
fn resolve_msaa() {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return;
    }
    let ms = texture(4, 4, 4, resource_flags::ALLOW_RENDER_TARGET);
    let dst = texture(4, 4, 1, 0);
    let rb = buffer(256 * 4, heap_type::READBACK, 0);
    run(|l| {
        let v = TargetView::rtv(&ms, None).unwrap();
        unsafe { l.clear_rtv(&v, [0.0, 1.0, 0.0, 1.0], &[]).unwrap() };
        l.resolve_subresource(&dst, 0, &ms, 0, RGBA8_UNORM).unwrap();
        readback_texture(l, &rb, &dst, 4, 4);
    });
    let px = words(&rb, 64 * 4);
    for y in 0..4 {
        for x in 0..4 {
            assert_eq!(px[y * 64 + x], 0xff00_ff00, "texel ({x},{y})");
        }
    }
}

#[test]
fn clear_rtv_and_dsv_rects_and_slices() {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return;
    }
    // 1. Multi-slice RTV clear + scissor rect clear preserving prior contents.
    let arr_desc = ResourceDesc {
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
    let arr = Resource::new_committed(&arr_desc, &heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "rtv array").unwrap();
    let rb0 = buffer(256 * 4, heap_type::READBACK, 0);
    let rb1 = buffer(256 * 4, heap_type::READBACK, 0);

    // 2. D32_FLOAT_S8X24_UINT DSV full clear + scissor rect clear + planar readback.
    const D32_S8: u32 = 20;
    const R32_FLOAT: u32 = 41;
    const R8_UINT: u32 = 62;
    let ds_desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: D32_S8,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_DEPTH_STENCIL,
        ..Default::default()
    };
    let ds = Resource::new_committed(&ds_desc, &heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "dsv").unwrap();
    let rb_depth = buffer(256 * 4, heap_type::READBACK, 0);
    let rb_stencil = buffer(256 * 4, heap_type::READBACK, 0);

    run(|l| {
        let v_all = TargetView::rtv(&arr, None).unwrap();
        assert_eq!(v_all.slices, 2);
        let rect = [Rect { left: 1, top: 1, right: 3, bottom: 3 }];
        unsafe {
            l.clear_rtv(&v_all, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
            l.clear_rtv(&v_all, [0.0, 1.0, 0.0, 1.0], &rect).unwrap();
        }
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb0, fp), [0, 0, 0], CopyLoc::Subresource(&arr, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb1, fp), [0, 0, 0], CopyLoc::Subresource(&arr, 1), None).unwrap();

        let dsv = TargetView::dsv(&ds, None).unwrap();
        unsafe {
            l.clear_dsv(&dsv, true, 1.0, true, 0xAA, &[]).unwrap();
            l.clear_dsv(&dsv, true, 0.25, true, 0x55, &rect).unwrap();
        }
        let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        let fp_s = PlacedFootprint { offset: 0, format: R8_UINT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        // Subresource 0 = depth plane, subresource 1 = stencil plane.
        l.copy_texture_region(CopyLoc::Footprint(&rb_depth, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_stencil, fp_s), [0, 0, 0], CopyLoc::Subresource(&ds, 1), None).unwrap();
    });

    for (slice, rb) in [(0, &rb0), (1, &rb1)] {
        let px = words(rb, 64 * 4);
        for y in 0..4 {
            for x in 0..4 {
                let inside = (1..3).contains(&x) && (1..3).contains(&y);
                let want = if inside { 0xff00_ff00 } else { 0xff00_00ff };
                assert_eq!(px[y * 64 + x], want, "slice {slice} ({x},{y})");
            }
        }
    }

    let d_words = words(&rb_depth, 64 * 4);
    let s_bytes = unsafe { std::slice::from_raw_parts(rb_stencil.buffer().unwrap().contents().as_ptr() as *const u8, 256 * 4) };
    for y in 0..4 {
        for x in 0..4 {
            let inside = (1..3).contains(&x) && (1..3).contains(&y);
            let got_d = f32::from_bits(d_words[y * 64 + x]);
            let want_d = if inside { 0.25 } else { 1.0 };
            assert!((got_d - want_d).abs() < 1e-5, "depth ({x},{y}): got {got_d}, want {want_d}");
            let got_s = s_bytes[y * 256 + x];
            let want_s = if inside { 0x55 } else { 0xAA };
            assert_eq!(got_s, want_s, "stencil ({x},{y})");
        }
    }
}

#[test]
fn copy_texture_bc1_and_planar_depth_stencil_and_typeless() {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return;
    }
    use d3d12_metal::metal::cmdlist::CopyBox;
    const BC1_UNORM: u32 = 71;
    const RGBA8_SRGB: u32 = 29;
    const D32_S8: u32 = 20;
    const R32_FLOAT: u32 = 41;
    const R8_UINT: u32 = 62;

    // 1. BC1 texture with 3 mips (8x8, 4x4, 2x2): upload mip 2 (2x2 physical, 4x4 block box) and read back.
    let bc_desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 8,
        Height: 8,
        DepthOrArraySize: 1,
        MipLevels: 3,
        Format: BC1_UNORM,
        SampleCount: 1,
        ..Default::default()
    };
    let bc_tex = Resource::new_committed(&bc_desc, &heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "bc1").unwrap();
    let up = buffer(256, heap_type::UPLOAD, 0);
    let rb = buffer(256, heap_type::READBACK, 0);
    unsafe {
        let p = up.buffer().unwrap().contents().as_ptr() as *mut u64;
        *p = 0x1122_3344_5566_7788;
    }

    // 2. Cross-format copy (RGBA8_UNORM -> RGBA8_SRGB).
    let t_unorm = texture(4, 4, 1, resource_flags::ALLOW_RENDER_TARGET);
    let srgb_desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: RGBA8_SRGB,
        SampleCount: 1,
        ..Default::default()
    };
    let t_srgb = Resource::new_committed(&srgb_desc, &heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "srgb").unwrap();
    let rb_srgb = buffer(256 * 4, heap_type::READBACK, 0);

    // 3. Planar D32_S8 single-plane texture-to-texture copy (copy only stencil plane from ds_src to ds_dst).
    let ds_desc = ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 4,
        Height: 4,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: D32_S8,
        SampleCount: 1,
        Flags: resource_flags::ALLOW_DEPTH_STENCIL,
        ..Default::default()
    };
    let ds_src = Resource::new_committed(&ds_desc, &heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "ds_src").unwrap();
    let ds_dst = Resource::new_committed(&ds_desc, &heap(heap_type::DEFAULT), MTLTextureUsage::empty(), "ds_dst").unwrap();
    let rb_dst_d = buffer(256 * 4, heap_type::READBACK, 0);
    let rb_dst_s = buffer(256 * 4, heap_type::READBACK, 0);

    run(|l| {
        let fp_bc = PlacedFootprint { offset: 0, format: BC1_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        let b4 = CopyBox { left: 0, top: 0, front: 0, right: 4, bottom: 4, back: 1 };
        l.copy_texture_region(CopyLoc::Subresource(&bc_tex, 2), [0, 0, 0], CopyLoc::Footprint(&up, fp_bc), Some(b4)).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb, fp_bc), [0, 0, 0], CopyLoc::Subresource(&bc_tex, 2), Some(b4)).unwrap();

        let rtv = TargetView::rtv(&t_unorm, None).unwrap();
        unsafe { l.clear_rtv(&rtv, [0.0, 0.0, 1.0, 1.0], &[]).unwrap() };
        l.copy_resource(&t_srgb, &t_unorm).unwrap();
        readback_texture(l, &rb_srgb, &t_srgb, 4, 4);

        let dsv_src = TargetView::dsv(&ds_src, None).unwrap();
        let dsv_dst = TargetView::dsv(&ds_dst, None).unwrap();
        unsafe {
            l.clear_dsv(&dsv_src, true, 0.75, true, 0x33, &[]).unwrap();
            l.clear_dsv(&dsv_dst, true, 0.125, true, 0x77, &[]).unwrap();
        }
        // Copy only plane 1 (stencil, subresource 1) from ds_src to ds_dst; ds_dst depth must remain 0.125.
        l.copy_texture_region(CopyLoc::Subresource(&ds_dst, 1), [0, 0, 0], CopyLoc::Subresource(&ds_src, 1), None).unwrap();
        let fp_d = PlacedFootprint { offset: 0, format: R32_FLOAT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        let fp_s = PlacedFootprint { offset: 0, format: R8_UINT, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_dst_d, fp_d), [0, 0, 0], CopyLoc::Subresource(&ds_dst, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_dst_s, fp_s), [0, 0, 0], CopyLoc::Subresource(&ds_dst, 1), None).unwrap();
    });

    let bc_out = unsafe { *(rb.buffer().unwrap().contents().as_ptr() as *const u64) };
    assert_eq!(bc_out, 0x1122_3344_5566_7788);

    let srgb_px = words(&rb_srgb, 64 * 4);
    assert_eq!(srgb_px[0], 0xffff_0000);

    let dst_d = f32::from_bits(words(&rb_dst_d, 1)[0]);
    let dst_s = unsafe { *(rb_dst_s.buffer().unwrap().contents().as_ptr() as *const u8) };
    assert!((dst_d - 0.125).abs() < 1e-5, "depth should stay 0.125, got {dst_d}");
    assert_eq!(dst_s, 0x33, "stencil should be copied from ds_src (0x33)");
}

#[test]
fn deferred_clears_land_before_copies() {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return;
    }
    // Full-surface clears are deferred; a copy reading the cleared source or
    // overwriting the cleared destination (no barrier in between) must see
    // them land first.
    let rt = resource_flags::ALLOW_RENDER_TARGET;
    let a = texture(4, 4, 1, rt);
    let b = texture(4, 4, 1, rt);
    let c = texture(4, 4, 1, rt);
    let rb_b = buffer(256 * 4, heap_type::READBACK, 0);
    let rb_c = buffer(256 * 4, heap_type::READBACK, 0);
    run(|l| {
        let (va, vb, vc) = (TargetView::rtv(&a, None).unwrap(), TargetView::rtv(&b, None).unwrap(), TargetView::rtv(&c, None).unwrap());
        unsafe {
            l.clear_rtv(&va, [1.0, 0.0, 0.0, 1.0], &[]).unwrap();
            l.clear_rtv(&vb, [0.0, 0.0, 1.0, 1.0], &[]).unwrap();
            l.clear_rtv(&vc, [0.0, 1.0, 0.0, 1.0], &[]).unwrap();
        }
        l.copy_resource(&b, &a).unwrap();
        let fp = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 4, height: 4, depth: 1, row_pitch: 256, _pad: 0 };
        let bx = d3d12_metal::metal::cmdlist::CopyBox { left: 0, top: 0, front: 0, right: 2, bottom: 2, back: 1 };
        l.copy_texture_region(CopyLoc::Subresource(&c, 0), [2, 2, 0], CopyLoc::Subresource(&a, 0), Some(bx)).unwrap();
        readback_texture(l, &rb_b, &b, 4, 4);
        l.copy_texture_region(CopyLoc::Footprint(&rb_c, fp), [0, 0, 0], CopyLoc::Subresource(&c, 0), None).unwrap();
    });
    let pb = words(&rb_b, 64 * 4);
    let pc = words(&rb_c, 64 * 4);
    for y in 0..4 {
        for x in 0..4 {
            assert_eq!(pb[y * 64 + x], 0xff00_00ff, "copy_resource dst ({x},{y})");
            let want_c = if x >= 2 && y >= 2 { 0xff00_00ff } else { 0xff00_ff00 };
            assert_eq!(pc[y * 64 + x], want_c, "copy_texture_region dst ({x},{y})");
        }
    }
}

#[test]
fn headless_swapchain_resize_and_present() {
    if d3d12_metal::metal::device().is_none() {
        eprintln!("skipping: no Metal 4 device");
        return;
    }
    let q = std::sync::Arc::new(Queue::new(ListType::Direct, "sc queue").unwrap());
    let noop = |f: Box<dyn FnOnce() + Send + '_>| f();
    let mut sc = unsafe { d3d12_metal::metal::swapchain::Swapchain::new(std::ptr::null_mut(), q, 64, 64, 87, 2, &noop).unwrap() };
    assert_eq!(sc.buffers.len(), 2);
    sc.present(0).unwrap();
    assert_eq!(sc.current, 1);
    sc.resize(128, 96, 28, 3, &noop).unwrap();
    assert_eq!((sc.width, sc.height, sc.buffers.len(), sc.current), (128, 96, 3, 0));
    sc.present(0).unwrap();
    assert_eq!(sc.current, 1);
}

