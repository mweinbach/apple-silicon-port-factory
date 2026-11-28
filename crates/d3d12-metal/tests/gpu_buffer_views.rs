//! Metal 4 GPU integration tests for D3D12 buffer SRV/UAV semantics and
//! `MTLTextureTypeTextureBuffer` validation (width limits, offsets, overhang clamping,
//! swizzles, unsupported formats, and direct vs pooled view validation).
//!
//! Run with:
//! `MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_buffer_views --target aarch64-apple-darwin -- --test-threads=1`

use d3d12_metal::metal::descriptor::{heap_kind, DescriptorHeap};
use d3d12_metal::metal::queue::{self, ListType, Queue};
use d3d12_metal::metal::resource::{Resource, MAX_TEXTURE_BUFFER_WIDTH, SWIZZLE_IDENTITY};
use d3d12_metal::metal::views;
use d3d12_metal::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};
use objc2_foundation::NSString;
use objc2_metal::*;

const DEFAULT_MAPPING: u32 = 0x1688;

fn buf_desc(bytes: u64, flags: u32) -> ResourceDesc {
    ResourceDesc {
        Dimension: dim::BUFFER,
        Width: bytes,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        Flags: flags,
        ..Default::default()
    }
}

fn tex2d_desc(w: u64, h: u32, dxgi: u32) -> ResourceDesc {
    ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: w,
        Height: h,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: dxgi,
        SampleCount: 1,
        Layout: 0,
        Flags: 0,
        ..Default::default()
    }
}

fn make_buf_srv(dxgi: u32, mapping: u32, first: u64, num: u32, stride: u32, flags: u32) -> [u8; 40] {
    let mut d = [0u8; 40];
    d[0..4].copy_from_slice(&dxgi.to_le_bytes());
    d[4..8].copy_from_slice(&1u32.to_le_bytes()); // D3D12_SRV_DIMENSION_BUFFER
    d[8..12].copy_from_slice(&mapping.to_le_bytes());
    d[16..24].copy_from_slice(&first.to_le_bytes());
    d[24..28].copy_from_slice(&num.to_le_bytes());
    d[28..32].copy_from_slice(&stride.to_le_bytes());
    d[32..36].copy_from_slice(&flags.to_le_bytes());
    d
}

fn make_buf_uav(dxgi: u32, first: u64, num: u32, stride: u32, flags: u32) -> [u8; 40] {
    let mut d = [0u8; 40];
    d[0..4].copy_from_slice(&dxgi.to_le_bytes());
    d[4..8].copy_from_slice(&1u32.to_le_bytes()); // D3D12_UAV_DIMENSION_BUFFER
    d[8..16].copy_from_slice(&first.to_le_bytes());
    d[16..20].copy_from_slice(&num.to_le_bytes());
    d[20..24].copy_from_slice(&stride.to_le_bytes());
    d[32..36].copy_from_slice(&flags.to_le_bytes());
    d
}

fn dispatch_compute(q: &Queue, src: &str, fn_name: &str, heap_gpu: u64, out_gpu: u64) {
    let dev = d3d12_metal::metal::device().expect("metal device");
    let lib = dev
        .dev
        .newLibraryWithSource_options_error(&NSString::from_str(src), None)
        .expect("compile test MSL");
    let func = lib.newFunctionWithName(&NSString::from_str(fn_name)).expect("find kernel");
    let pso = dev.dev.newComputePipelineStateWithFunction_error(&func).expect("create PSO");

    dev.commit_residency();
    let alloc = dev.dev.newCommandAllocator().expect("command allocator");
    let cb = dev.dev.newCommandBuffer().expect("command buffer");
    cb.beginCommandBufferWithAllocator(&alloc);

    let atd = MTL4ArgumentTableDescriptor::new();
    atd.setMaxBufferBindCount(2);
    let at = dev.dev.newArgumentTableWithDescriptor_error(&atd).expect("argument table");

    let enc = cb.computeCommandEncoder().expect("compute encoder");
    enc.setComputePipelineState(&pso);
    unsafe {
        at.setAddress_atIndex(heap_gpu, 0);
        at.setAddress_atIndex(out_gpu, 1);
    }
    enc.setArgumentTable(Some(&at));
    enc.dispatchThreadgroups_threadsPerThreadgroup(
        MTLSize { width: 1, height: 1, depth: 1 },
        MTLSize { width: 1, height: 1, depth: 1 },
    );
    enc.endEncoding();
    cb.endCommandBuffer();

    q.submit(&[&*cb]);
    q.wait_idle();
}

#[test]
fn oversized_typed_buffer_srv_clamps_to_max_width_and_reads_on_gpu() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    queue::collect_all();
    let q = Queue::new(ListType::Direct, "gpu_buf_views_oversize").unwrap();
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let hp_rb = HeapProperties { Type: heap_type::READBACK, ..Default::default() };

    // 256 MiB + 4 KiB buffer: with DXGI_FORMAT_R8_UINT (bpe = 1), its total element count
    // exceeds Apple Silicon's maxTextureBufferWidth (268,435,456 = 2^28) while using only ~256 MiB.
    let total_elems = MAX_TEXTURE_BUFFER_WIDTH + 4096;
    let big = Resource::new_committed_default(&buf_desc(total_elems, 0), &hp_up, "oversize_tbuf").unwrap();
    unsafe {
        let p = big.cpu_ptr().unwrap();
        *p.add(0) = 0x11;
        *p.add(1) = 0x22;
        *p.add((MAX_TEXTURE_BUFFER_WIDTH - 1) as usize) = 0xAB;
        *p.add(MAX_TEXTURE_BUFFER_WIDTH as usize) = 0xCD;
        *p.add((MAX_TEXTURE_BUFFER_WIDTH + 3) as usize) = 0xEF;
    }

    // 1. Oversized full-buffer view (FirstElement=0, NumElements=2^28 + 4096):
    //    must clamp to 2^28 without aborting in Metal validation.
    let full_desc = make_buf_srv(62, DEFAULT_MAPPING, 0, total_elems as u32, 0, 0); // R8_UINT
    let e_full = views::srv_entry(Some(&big), Some(&full_desc)).expect("oversized typed buffer SRV must succeed");
    let full_va = u64::from_le_bytes(e_full[0..8].try_into().unwrap());
    let full_tv = u64::from_le_bytes(e_full[8..16].try_into().unwrap());
    let full_meta = u64::from_le_bytes(e_full[16..24].try_into().unwrap());
    assert_eq!(full_va, big.gpu_va);
    assert_ne!(full_tv, 0);
    assert_eq!(full_meta & 0xffff_ffff, MAX_TEXTURE_BUFFER_WIDTH);
    assert_eq!((full_meta >> 32) & 0xff, 0);
    assert_eq!(full_meta >> 63, 1);

    // 2. High-offset view starting at FirstElement=2^28 (offset 256 MiB, NumElements=64):
    //    must NOT be clamped and must read the exact bytes past 2^28.
    let high_desc = make_buf_srv(62, DEFAULT_MAPPING, MAX_TEXTURE_BUFFER_WIDTH, 64, 0, 0);
    let e_high = views::srv_entry(Some(&big), Some(&high_desc)).expect("high-offset typed buffer SRV must succeed");
    let high_va = u64::from_le_bytes(e_high[0..8].try_into().unwrap());
    let high_tv = u64::from_le_bytes(e_high[8..16].try_into().unwrap());
    let high_meta = u64::from_le_bytes(e_high[16..24].try_into().unwrap());
    assert_eq!(high_va, big.gpu_va + MAX_TEXTURE_BUFFER_WIDTH);
    assert_ne!(high_tv, 0);
    assert_eq!(high_meta & 0xffff_ffff, 64);

    // Direct (non-pooled) view at exact MAX_TEXTURE_BUFFER_WIDTH must also pass Metal debug layer.
    let direct = big.buffer_texture_view(MTLPixelFormat::R8Uint, 0, MAX_TEXTURE_BUFFER_WIDTH, false);
    assert!(direct.is_some(), "direct texture-buffer view at exact max width must succeed");

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 2, true, "oversize_heap").unwrap();
    unsafe {
        let base = heap.cpu_base as *mut u8;
        std::ptr::copy_nonoverlapping(e_full.as_ptr(), base, 24);
        std::ptr::copy_nonoverlapping(e_high.as_ptr(), base.add(24), 24);
    }
    let out = Resource::new_committed_default(&buf_desc(64, 0), &hp_rb, "oversize_out").unwrap();

    let src = r#"
        #include <metal_stdlib>
        using namespace metal;
        struct BufEntry {
            uint64_t gpu_va;
            texture_buffer<uint, access::read> tbuf;
            uint64_t meta;
        };
        kernel void probe_oversize(
            device const uchar* heap [[buffer(0)]],
            device uint* out [[buffer(1)]]
        ) {
            device const BufEntry* full_v = reinterpret_cast<device const BufEntry*>(heap);
            device const BufEntry* high_v = reinterpret_cast<device const BufEntry*>(heap + 24);
            out[0] = full_v->tbuf.read(0u).x;
            out[1] = full_v->tbuf.read(1u).x;
            out[2] = full_v->tbuf.read(268435455u).x;
            out[3] = full_v->tbuf.read(268435456u).x;
            out[4] = high_v->tbuf.read(0u).x;
            out[5] = high_v->tbuf.read(3u).x;
            out[6] = full_v->tbuf.get_width();
            out[7] = high_v->tbuf.get_width();
        }
    "#;
    dispatch_compute(&q, src, "probe_oversize", heap.gpu_base, out.gpu_va);

    let got = unsafe { std::ptr::read_unaligned(out.cpu_ptr().unwrap() as *const [u32; 8]) };
    assert_eq!(got[0], 0x11, "full view index 0");
    assert_eq!(got[1], 0x22, "full view index 1");
    assert_eq!(got[2], 0xAB, "full view last in-range index (2^28 - 1)");
    assert_eq!(got[3], 0x00, "full view index 2^28 must safely read 0");
    assert_eq!(got[4], 0xCD, "high-offset view index 0 (= buffer byte 2^28)");
    assert_eq!(got[5], 0xEF, "high-offset view index 3 (= buffer byte 2^28 + 3)");
    assert_eq!(got[6], MAX_TEXTURE_BUFFER_WIDTH as u32, "clamped hardware width");
    assert_eq!(got[7], 64, "high-offset hardware width");
}

#[test]
fn typed_buffer_offset_overhang_swizzle_and_empty_work_on_gpu() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    queue::collect_all();
    let q = Queue::new(ListType::Direct, "gpu_buf_views_semantics").unwrap();
    let hp_up = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let hp_rb = HeapProperties { Type: heap_type::READBACK, ..Default::default() };

    // 64-byte buffer = 16 R32_UINT / RGBA8 elements.
    let buf = Resource::new_committed_default(&buf_desc(64, 0), &hp_up, "sem_buf").unwrap();
    unsafe {
        let p = buf.cpu_ptr().unwrap() as *mut u32;
        for i in 0..16u32 {
            p.add(i as usize).write(0x1000 + i);
        }
        // Element 0 as RGBA8 bytes: R=0x40 (~0.251), G=0x80 (~0.502), B=0xC0 (~0.753), A=0x20 (~0.125).
        // Keep elements 3..7 as known u32s for the unaligned FirstElement=3 test.
        p.add(0).write(u32::from_le_bytes([0x40, 0x80, 0xC0, 0x20]));
    }

    // 1. Unaligned FirstElement=3, NumElements=4 (R32_UINT -> aligned=0, el_off=3).
    let d_off = make_buf_srv(42, DEFAULT_MAPPING, 3, 4, 0, 0);
    let e_off = views::srv_entry(Some(&buf), Some(&d_off)).unwrap();
    let off_meta = u64::from_le_bytes(e_off[16..24].try_into().unwrap());
    assert_eq!((off_meta >> 32) & 0xff, 3, "el_off must be 3");

    // 2. Overhanging NumElements=1000 on 16-element buffer (R32_UINT -> clamped to 16).
    let d_over = make_buf_srv(42, DEFAULT_MAPPING, 0, 1000, 0, 0);
    let e_over = views::srv_entry(Some(&buf), Some(&d_over)).unwrap();
    let over_meta = u64::from_le_bytes(e_over[16..24].try_into().unwrap());
    assert_eq!(over_meta & 0xffff_ffff, 64, "overhang must clamp to buffer capacity");

    // 3. Custom Shader4ComponentMapping (R<-B(2), G<-G(1), B<-R(0), A<-FORCE_1(5) = 0x1A0A) on RGBA8_UNORM.
    let map_bgr1 = (2u32) | (1 << 3) | (0 << 6) | (5 << 9) | (1 << 12);
    let d_swz = make_buf_srv(28, map_bgr1, 0, 4, 0, 0);
    let e_swz = views::srv_entry(Some(&buf), Some(&d_swz)).unwrap();

    // 4. Zero-initialized Shader4ComponentMapping (0) must behave as identity, not (R,R,R,R).
    let d_zero_map = make_buf_srv(28, 0, 0, 4, 0, 0);
    let e_zero_map = views::srv_entry(Some(&buf), Some(&d_zero_map)).unwrap();

    // 5. B8G8R8X8_UNORM (dxgi 88) read-only SRV must force alpha to 1.0 and swap R/B.
    let d_bgrx = make_buf_srv(88, DEFAULT_MAPPING, 0, 4, 0, 0);
    let e_bgrx = views::srv_entry(Some(&buf), Some(&d_bgrx)).unwrap();

    // 6. Empty (NumElements=0) and past-end (FirstElement=16) views encode null descriptors.
    assert_eq!(views::srv_entry(Some(&buf), Some(&make_buf_srv(42, DEFAULT_MAPPING, 0, 0, 0, 0))).unwrap(), [0u8; 24]);
    assert_eq!(views::srv_entry(Some(&buf), Some(&make_buf_srv(42, DEFAULT_MAPPING, 16, 4, 0, 0))).unwrap(), [0u8; 24]);
    assert_eq!(views::srv_entry(Some(&buf), Some(&make_buf_srv(42, DEFAULT_MAPPING, u64::MAX, 4, 0, 0))).unwrap(), [0u8; 24]);

    let heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 5, true, "sem_heap").unwrap();
    unsafe {
        let base = heap.cpu_base as *mut u8;
        for (i, e) in [e_off, e_over, e_swz, e_zero_map, e_bgrx].iter().enumerate() {
            std::ptr::copy_nonoverlapping(e.as_ptr(), base.add(i * 24), 24);
        }
    }
    let out = Resource::new_committed_default(&buf_desc(128, 0), &hp_rb, "sem_out").unwrap();

    let src = r#"
        #include <metal_stdlib>
        using namespace metal;
        struct UintEntry {
            uint64_t gpu_va;
            texture_buffer<uint, access::read> tbuf;
            uint64_t meta;
        };
        struct FloatEntry {
            uint64_t gpu_va;
            texture_buffer<float, access::read> tbuf;
            uint64_t meta;
        };
        struct Out {
            uint4 vals;
            float4 swz;
            float4 zero_map;
            float4 bgrx;
        };
        kernel void probe_sem(
            device const uchar* heap [[buffer(0)]],
            device Out* out [[buffer(1)]]
        ) {
            device const UintEntry* off_v = reinterpret_cast<device const UintEntry*>(heap);
            device const UintEntry* over_v = reinterpret_cast<device const UintEntry*>(heap + 24);
            device const FloatEntry* swz_v = reinterpret_cast<device const FloatEntry*>(heap + 48);
            device const FloatEntry* zmap_v = reinterpret_cast<device const FloatEntry*>(heap + 72);
            device const FloatEntry* bgrx_v = reinterpret_cast<device const FloatEntry*>(heap + 96);
            uint el_off = uint((off_v->meta >> 32) & 0xffull);
            out->vals = uint4(
                off_v->tbuf.read(0u + el_off).x,
                off_v->tbuf.read(3u + el_off).x,
                over_v->tbuf.read(15u).x,
                over_v->tbuf.read(16u).x
            );
            out->swz = swz_v->tbuf.read(0u);
            out->zero_map = zmap_v->tbuf.read(0u);
            out->bgrx = bgrx_v->tbuf.read(0u);
        }
    "#;
    dispatch_compute(&q, src, "probe_sem", heap.gpu_base, out.gpu_va);

    let (vals, swz, zmap, bgrx) = unsafe {
        let p = out.cpu_ptr().unwrap();
        (
            std::ptr::read_unaligned(p as *const [u32; 4]),
            std::ptr::read_unaligned(p.add(16) as *const [f32; 4]),
            std::ptr::read_unaligned(p.add(32) as *const [f32; 4]),
            std::ptr::read_unaligned(p.add(48) as *const [f32; 4]),
        )
    };
    assert_eq!(vals, [0x1003, 0x1006, 0x100F, 0], "unaligned FirstElement & overhang reads");

    let f = |b: u8| b as f32 / 255.0;
    // Custom swizzle R<-B(0xC0), G<-G(0x80), B<-R(0x40), A<-1.0
    assert!((swz[0] - f(0xC0)).abs() < 0.01 && (swz[1] - f(0x80)).abs() < 0.01 && (swz[2] - f(0x40)).abs() < 0.01 && (swz[3] - 1.0).abs() < 0.01, "swz={swz:?}");
    // Zero mapping = identity RGBA
    assert!((zmap[0] - f(0x40)).abs() < 0.01 && (zmap[1] - f(0x80)).abs() < 0.01 && (zmap[2] - f(0xC0)).abs() < 0.01 && (zmap[3] - f(0x20)).abs() < 0.01, "zmap={zmap:?}");
    // B8G8R8X8_UNORM reads bytes [B=0x40, G=0x80, R=0xC0, X=0x20] -> (R=0xC0, G=0x80, B=0x40, A=1.0)
    assert!((bgrx[0] - f(0xC0)).abs() < 0.01 && (bgrx[1] - f(0x80)).abs() < 0.01 && (bgrx[2] - f(0x40)).abs() < 0.01 && (bgrx[3] - 1.0).abs() < 0.01, "bgrx={bgrx:?}");

    // Direct non-pooled swizzled view also succeeds and differs from identity.
    let direct_swz = buf
        .buffer_texture_view_swizzled(MTLPixelFormat::RGBA8Unorm, 0, 4, false, 0x0102_0304)
        .expect("direct swizzled texture-buffer view must succeed");
    assert_eq!(direct_swz.width(), 4);

    // Typed buffer UAV with unaligned FirstElement=3 and overhanging NumElements=100 writes
    // into element 3 without Metal validation errors and drops out-of-range writes cleanly.
    let uav_buf = Resource::new_committed_default(
        &buf_desc(64, resource_flags::ALLOW_UNORDERED_ACCESS),
        &hp_rb,
        "sem_uav_buf",
    )
    .unwrap();
    unsafe {
        std::ptr::write_bytes(uav_buf.cpu_ptr().unwrap(), 0, 64);
    }
    let d_uav = make_buf_uav(42, 3, 100, 0, 0); // R32_UINT, first=3, num=100 -> clamped to 16 elems (13 visible)
    let e_uav = views::uav_entry(Some(&uav_buf), Some(&d_uav)).unwrap();
    let uav_meta = u64::from_le_bytes(e_uav[16..24].try_into().unwrap());
    assert_eq!(uav_meta & 0xffff_ffff, 64);
    assert_eq!((uav_meta >> 32) & 0xff, 3);
    assert_eq!(uav_meta >> 63, 1);

    let uav_heap = DescriptorHeap::new(heap_kind::CBV_SRV_UAV, 1, true, "sem_uav_heap").unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(e_uav.as_ptr(), uav_heap.cpu_base as *mut u8, 24);
    }
    let uav_src = r#"
        #include <metal_stdlib>
        using namespace metal;
        struct UavEntry {
            uint64_t gpu_va;
            texture_buffer<uint, access::read_write> tbuf;
            uint64_t meta;
        };
        kernel void write_tbuf(
            device const UavEntry* heap [[buffer(0)]],
            device uint* unused_out [[buffer(1)]]
        ) {
            uint el_off = uint((heap[0].meta >> 32) & 0xffull);
            heap[0].tbuf.write(uint4(0xCAFEBABEu, 0, 0, 0), 0u + el_off);
            heap[0].tbuf.write(uint4(0xDEADBEEFu, 0, 0, 0), 12u + el_off);
            // Out-of-bounds index (13 + 3 = 16 == width) must be dropped safely.
            heap[0].tbuf.write(uint4(0xBAD0BAD0u, 0, 0, 0), 13u + el_off);
            unused_out[0] = heap[0].tbuf.get_width();
        }
    "#;
    dispatch_compute(&q, uav_src, "write_tbuf", uav_heap.gpu_base, out.gpu_va);
    let uav_words = unsafe { std::ptr::read_unaligned(uav_buf.cpu_ptr().unwrap() as *const [u32; 16]) };
    assert_eq!(uav_words[0], 0);
    assert_eq!(uav_words[3], 0xCAFE_BABE, "typed UAV write at index 0 + el_off(3)");
    assert_eq!(uav_words[15], 0xDEAD_BEEF, "typed UAV write at last in-bounds index 12 + el_off(3)");
    let uav_width = unsafe { std::ptr::read_unaligned(out.cpu_ptr().unwrap() as *const u32) };
    assert_eq!(uav_width, 16, "overhanging typed UAV width must clamp to 16 elements");
}

#[test]
fn invalid_buffer_views_fail_cleanly_without_metal_abort() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    queue::collect_all();
    let hp_def = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
    let buf = Resource::new_committed_default(
        &buf_desc(256, resource_flags::ALLOW_UNORDERED_ACCESS),
        &hp_def,
        "invalid_views_buf",
    )
    .unwrap();
    let tex = Resource::new_committed_default(&tex2d_desc(16, 16, 28), &hp_def, "invalid_views_tex").unwrap();

    // Formats that would SIGABRT in Metal validation if passed to textureBufferDescriptor:
    // BC1_UNORM(71), D32_FLOAT(40), R8G8B8A8_UNORM_SRGB(29), B8G8R8A8_UNORM_SRGB(91),
    // R9G9B9E5_SHAREDEXP(67), B5G6R5_UNORM(85), B5G5R5A1_UNORM(86), R32_TYPELESS(39, non-raw), UNKNOWN(0).
    for dxgi in [0u32, 29, 39, 40, 67, 71, 85, 86, 91] {
        assert!(
            views::srv_entry(Some(&buf), Some(&make_buf_srv(dxgi, DEFAULT_MAPPING, 0, 8, 0, 0))).is_err(),
            "SRV with unsupported typed buffer format {dxgi} must fail cleanly"
        );
        assert!(
            views::uav_entry(Some(&buf), Some(&make_buf_uav(dxgi, 0, 8, 0, 0))).is_err(),
            "UAV with unsupported typed buffer format {dxgi} must fail cleanly"
        );
    }

    // BGRA8_UNORM (87) is valid for read-only typed SRV, rejected cleanly for writable typed UAV.
    assert!(views::srv_entry(Some(&buf), Some(&make_buf_srv(87, DEFAULT_MAPPING, 0, 8, 0, 0))).is_ok());
    assert!(views::uav_entry(Some(&buf), Some(&make_buf_uav(87, 0, 8, 0, 0))).is_err());

    // Raw buffer with non-zero stride must fail cleanly.
    assert!(views::srv_entry(Some(&buf), Some(&make_buf_srv(39, DEFAULT_MAPPING, 0, 8, 16, 1))).is_err());
    assert!(views::uav_entry(Some(&buf), Some(&make_buf_uav(39, 0, 8, 16, 1))).is_err());

    // Buffer view of a texture resource must fail cleanly.
    assert!(views::srv_entry(Some(&tex), Some(&make_buf_srv(42, DEFAULT_MAPPING, 0, 8, 0, 0))).is_err());
    assert!(views::srv_entry(Some(&tex), Some(&make_buf_srv(39, DEFAULT_MAPPING, 0, 8, 0, 1))).is_err());
    assert!(views::uav_entry(Some(&tex), Some(&make_buf_uav(42, 0, 8, 0, 0))).is_err());

    // Direct low-level Resource::buffer_texture_view(_id) precondition checks must also return None
    // without triggering Metal assertions under MTL_DEBUG_LAYER=1.
    assert!(buf.buffer_texture_view_id(MTLPixelFormat::RGBA8Unorm, 0, 0, false).is_none());
    assert!(buf.buffer_texture_view(MTLPixelFormat::RGBA8Unorm, 0, 0, false).is_none());
    assert!(buf.buffer_texture_view_id(MTLPixelFormat::RGBA8Unorm, 0, MAX_TEXTURE_BUFFER_WIDTH + 1, false).is_none());
    assert!(buf.buffer_texture_view(MTLPixelFormat::RGBA8Unorm, 0, MAX_TEXTURE_BUFFER_WIDTH + 1, false).is_none());
    assert!(buf.buffer_texture_view_id(MTLPixelFormat::RGBA8Unorm, 4, 4, false).is_none()); // unaligned offset
    assert!(buf.buffer_texture_view(MTLPixelFormat::RGBA8Unorm, 4, 4, false).is_none());
    assert!(buf.buffer_texture_view_id(MTLPixelFormat::RGBA8Unorm, 0, 65, false).is_none()); // > 256 bytes
    assert!(buf.buffer_texture_view(MTLPixelFormat::RGBA8Unorm, 0, 65, false).is_none());
    assert!(buf.buffer_texture_view_id(MTLPixelFormat::BC1_RGBA, 0, 4, false).is_none());
    assert!(buf.buffer_texture_view_id(MTLPixelFormat::Depth32Float, 0, 4, false).is_none());
    assert!(buf.buffer_texture_view_id(MTLPixelFormat::RGBA8Unorm_sRGB, 0, 4, false).is_none());
    assert!(buf.buffer_texture_view_id(MTLPixelFormat::BGRA8Unorm, 0, 4, true).is_none());
    assert!(buf.buffer_texture_view_id_swizzled(MTLPixelFormat::RGBA8Unorm, 0, 4, true, 0x0502_0304).is_none());
    // Valid baseline succeeds.
    assert!(buf.buffer_texture_view_id_swizzled(MTLPixelFormat::RGBA8Unorm, 0, 4, false, SWIZZLE_IDENTITY).is_some());
}
