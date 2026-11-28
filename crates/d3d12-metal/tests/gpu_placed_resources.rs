//! Metal 4 GPU integration tests for D3D12 placed-resource alignment rules
//! (`GetResourceAllocationInfo` / `D3D12_SMALL_RESOURCE_PLACEMENT_ALIGNMENT` /
//! `Resource::new_placed_default`) under `MTL_DEBUG_LAYER=1`.
//!
//! Run with:
//! `MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_placed_resources --target aarch64-apple-darwin -- --test-threads=1`

use d3d12_metal::metal::heap::Heap;
use d3d12_metal::metal::resource::{
    allocation_info, CreateError, Resource, DEFAULT_ALIGNMENT, MSAA_ALIGNMENT, SMALL_ALIGNMENT,
    SMALL_MSAA_ALIGNMENT,
};
use d3d12_metal::types::{dim, heap_type, resource_flags, HeapDesc, HeapProperties, ResourceDesc};
use std::sync::Arc;

fn default_heap(size: u64) -> Arc<Heap> {
    let hd = HeapDesc {
        SizeInBytes: size,
        Properties: HeapProperties { Type: heap_type::DEFAULT, ..Default::default() },
        Alignment: DEFAULT_ALIGNMENT,
        Flags: 0,
    };
    Arc::new(Heap::new(&hd, "placed_align_test_heap").expect("create placement heap"))
}

fn tex2d_desc(w: u64, h: u32, arr: u16, mips: u16, dxgi: u32, samples: u32, flags: u32, align: u64) -> ResourceDesc {
    ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Alignment: align,
        Width: w,
        Height: h,
        DepthOrArraySize: arr,
        MipLevels: mips,
        Format: dxgi,
        SampleCount: samples,
        SampleQuality: 0,
        Layout: 0,
        Flags: flags,
    }
}

fn buf_desc(bytes: u64, align: u64) -> ResourceDesc {
    ResourceDesc {
        Dimension: dim::BUFFER,
        Alignment: align,
        Width: bytes,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 0,
        SampleCount: 1,
        SampleQuality: 0,
        Layout: 1,
        Flags: 0,
    }
}

#[test]
fn small_alignment_probe_rejects_16kb_metal_texture_and_falls_back_to_64kb() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }

    // 128x128x32 mips=4 R8_UNORM requires 16 KiB placement alignment on Apple Silicon Metal.
    // Requesting D3D12_SMALL_RESOURCE_PLACEMENT_ALIGNMENT (4 KiB) MUST fail allocation_info
    // so callers observing SizeInBytes == UINT64_MAX fall back to Alignment = 0 (64 KiB).
    let probe_4k = tex2d_desc(128, 128, 32, 4, 61, 1, 0, SMALL_ALIGNMENT);
    let err = allocation_info(&probe_4k).expect_err("16 KiB-aligned texture must reject 4 KiB small alignment");
    assert!(matches!(err, CreateError::InvalidArg(_)), "unexpected error: {err:?}");

    // Fallback to Alignment = 0 returns DEFAULT_ALIGNMENT (64 KiB) and places cleanly.
    let fallback = tex2d_desc(128, 128, 32, 4, 61, 1, 0, 0);
    let (size, align) = allocation_info(&fallback).expect("default alignment must succeed");
    assert_eq!(align, DEFAULT_ALIGNMENT);
    assert!(size >= 128 * 128 * 32);
    assert_eq!(size % DEFAULT_ALIGNMENT, 0);

    let heap = default_heap(size + DEFAULT_ALIGNMENT * 2);

    // Misaligned 4 KiB offset (0x2a000) must return InvalidArg instead of aborting in MTLDebugHeap.
    let bad = Resource::new_placed_default(&heap, 0x2a000, &probe_4k, "bad_4k_array");
    assert!(matches!(bad, Err(CreateError::InvalidArg(_))), "expected InvalidArg for 0x2a000 offset");
    let bad_zero = Resource::new_placed_default(&heap, 0x2a000, &fallback, "bad_offset_default");
    assert!(matches!(bad_zero, Err(CreateError::InvalidArg(_))), "expected InvalidArg for unaligned offset");

    // 64 KiB-aligned placement succeeds cleanly under MTL_DEBUG_LAYER=1.
    let ok = Resource::new_placed_default(&heap, DEFAULT_ALIGNMENT, &fallback, "ok_64k_array")
        .expect("64 KiB-aligned placed texture array must succeed");
    assert!(ok.texture().is_some());
}

#[test]
fn small_4kb_alignment_works_for_genuinely_small_textures_and_rejects_forbidden_cases() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }

    // Small 64x64 R8_UNORM texture (Metal align = 128 B <= 4 KiB): 4 KiB alignment succeeds
    // and can be placed at a 4 KiB-aligned offset (0x1000) under MTL_DEBUG_LAYER=1.
    let small = tex2d_desc(64, 64, 1, 1, 61, 1, 0, SMALL_ALIGNMENT);
    let (size, align) = allocation_info(&small).expect("64x64 R8_UNORM supports 4 KiB alignment");
    assert_eq!(align, SMALL_ALIGNMENT);
    assert_eq!(size % SMALL_ALIGNMENT, 0);

    let heap = default_heap(DEFAULT_ALIGNMENT);
    let placed = Resource::new_placed_default(&heap, SMALL_ALIGNMENT, &small, "placed_small_4k")
        .expect("4 KiB-aligned placement of small texture must succeed");
    assert!(placed.texture().is_some());

    // Placing past heap end must return InvalidArg instead of aborting in MTLDebugHeap.
    let oob = Resource::new_placed_default(&heap, DEFAULT_ALIGNMENT - SMALL_ALIGNMENT, &small, "placed_oob");
    if size > SMALL_ALIGNMENT {
        assert!(matches!(oob, Err(CreateError::InvalidArg(_))));
    }

    // Render target and depth-stencil textures cannot use 4 KiB small alignment in D3D12.
    let rt_4k = tex2d_desc(64, 64, 1, 1, 28, 1, resource_flags::ALLOW_RENDER_TARGET, SMALL_ALIGNMENT);
    assert!(matches!(allocation_info(&rt_4k), Err(CreateError::InvalidArg(_))));
    let ds_4k = tex2d_desc(64, 64, 1, 1, 40, 1, resource_flags::ALLOW_DEPTH_STENCIL, SMALL_ALIGNMENT);
    assert!(matches!(allocation_info(&ds_4k), Err(CreateError::InvalidArg(_))));

    // Buffers cannot use 4 KiB alignment or bogus intermediate alignments.
    assert!(matches!(allocation_info(&buf_desc(4096, SMALL_ALIGNMENT)), Err(CreateError::InvalidArg(_))));
    assert!(matches!(allocation_info(&buf_desc(4096, 16384)), Err(CreateError::InvalidArg(_))));
    let (b_sz, b_al) = allocation_info(&buf_desc(4096, 0)).expect("buffer default alignment");
    assert_eq!((b_sz, b_al), (DEFAULT_ALIGNMENT, DEFAULT_ALIGNMENT));

    // Explicit 4 MiB alignment is permitted on non-MSAA textures by D3D12.
    let explicit_4m = tex2d_desc(64, 64, 1, 1, 28, 1, 0, MSAA_ALIGNMENT);
    let (sz_4m, al_4m) = allocation_info(&explicit_4m).expect("explicit 4 MiB non-MSAA texture alignment");
    assert_eq!(al_4m, MSAA_ALIGNMENT);
    assert_eq!(sz_4m % MSAA_ALIGNMENT, 0);
    let heap_4m = default_heap(sz_4m);
    let placed_4m = Resource::new_placed_default(&heap_4m, 0, &explicit_4m, "placed_non_msaa_4m")
        .expect("4 MiB-aligned placement of non-MSAA texture must succeed");
    assert!(placed_4m.texture().is_some());

    // Illegal non-D3D12 texture alignment (e.g. 16 KiB) must be rejected.
    let bogus = tex2d_desc(64, 64, 1, 1, 61, 1, 0, 16384);
    assert!(matches!(allocation_info(&bogus), Err(CreateError::InvalidArg(_))));

    // MSAA textures: 4 KiB rejected; 0 -> 4 MiB; 64 KiB small-MSAA allowed for RT/DSV with
    // UNKNOWN layout (and rejected for non-UNKNOWN layouts).
    let msaa_4k = tex2d_desc(64, 64, 1, 1, 28, 4, resource_flags::ALLOW_RENDER_TARGET, SMALL_ALIGNMENT);
    assert!(matches!(allocation_info(&msaa_4k), Err(CreateError::InvalidArg(_))));
    let msaa_rt_64k = tex2d_desc(64, 64, 1, 1, 28, 4, resource_flags::ALLOW_RENDER_TARGET, SMALL_MSAA_ALIGNMENT);
    let (msaa_small_sz, msaa_small_al) =
        allocation_info(&msaa_rt_64k).expect("small MSAA render target supports 64 KiB alignment");
    assert_eq!(msaa_small_al, SMALL_MSAA_ALIGNMENT);
    assert_eq!(msaa_small_sz % SMALL_MSAA_ALIGNMENT, 0);
    let msaa_heap = default_heap(msaa_small_sz);
    let placed_msaa = Resource::new_placed_default(&msaa_heap, 0, &msaa_rt_64k, "placed_msaa_64k")
        .expect("64 KiB-aligned small MSAA RT placement must succeed");
    assert!(placed_msaa.texture().is_some());
    let mut msaa_bad_layout = msaa_rt_64k;
    msaa_bad_layout.Layout = 1; // ROW_MAJOR
    assert!(matches!(allocation_info(&msaa_bad_layout), Err(CreateError::InvalidArg(_))));
    let msaa_def = tex2d_desc(64, 64, 1, 1, 28, 4, resource_flags::ALLOW_RENDER_TARGET, 0);
    let (_, msaa_al) = allocation_info(&msaa_def).expect("MSAA default alignment");
    assert_eq!(msaa_al, MSAA_ALIGNMENT);
}
