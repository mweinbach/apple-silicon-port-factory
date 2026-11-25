use std::ffi::c_void;
use std::sync::Arc;

use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::metal::cmdlist::{
    CommandAllocator as MtlCommandAllocator, CommandList as MtlCommandList, CopyLoc,
};
use d3d12_metal::metal::queue::{ListType, Queue as MtlQueue};
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::types::{dim, heap_type, HeapProperties, ResourceDesc};

use super::abi::{
    TileCoord, TileRegionSize, D3D12_TILE_SIZE_BYTES, E_INVALIDARG, E_NOTIMPL, E_PENDING, S_OK,
};
use super::gdeflate::{
    GDEFLATE_HEADER_SIZE, GDEFLATE_ID, GDEFLATE_MAGIC, GDEFLATE_TILE_SIZE_IDX_64K,
};
use super::queue::standard_tile_shape;
use super::*;
use crate::com::Guid;
use crate::d3d12::com::Obj;
use crate::types::{HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT};

unsafe fn vtbl_entry(obj: usize, slot: usize) -> usize {
    let vt = std::ptr::read_unaligned(obj as *const *const usize);
    std::ptr::read_unaligned(vt.add(slot))
}

unsafe fn com_qi(obj: usize, riid: &Guid, out: &mut usize) -> i32 {
    let f: unsafe extern "win64" fn(usize, usize, *mut usize) -> i32 =
        std::mem::transmute(vtbl_entry(obj, 0));
    f(obj, riid as *const Guid as usize, out as *mut usize)
}

unsafe fn com_release(obj: usize) -> u32 {
    let f: unsafe extern "win64" fn(usize) -> u32 = std::mem::transmute(vtbl_entry(obj, 2));
    f(obj)
}

unsafe fn custom_decomp_get_event(q: usize) -> HANDLE {
    let f: unsafe extern "win64" fn(usize) -> HANDLE = std::mem::transmute(vtbl_entry(q, 3));
    f(q)
}

unsafe fn custom_decomp_get_requests(
    q: usize,
    max_requests: u32,
    requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
    num_requests: &mut u32,
) -> i32 {
    let f: unsafe extern "win64" fn(
        usize,
        u32,
        *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        *mut u32,
    ) -> i32 = std::mem::transmute(vtbl_entry(q, 4));
    f(q, max_requests, requests, num_requests as *mut u32)
}

unsafe fn custom_decomp_set_results(
    q: usize,
    results: &[DSTORAGE_CUSTOM_DECOMPRESSION_RESULT],
) -> i32 {
    let f: unsafe extern "win64" fn(
        usize,
        u32,
        *const DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
    ) -> i32 = std::mem::transmute(vtbl_entry(q, 5));
    f(q, results.len() as u32, results.as_ptr())
}

unsafe fn custom_decomp_get_requests1(
    q: usize,
    flags: u32,
    max_requests: u32,
    requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
    num_requests: &mut u32,
) -> i32 {
    let f: unsafe extern "win64" fn(
        usize,
        u32,
        u32,
        *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        *mut u32,
    ) -> i32 = std::mem::transmute(vtbl_entry(q, 6));
    f(q, flags, max_requests, requests, num_requests as *mut u32)
}

unsafe fn factory_create_queue(
    factory: usize,
    desc: &DSTORAGE_QUEUE_DESC,
    riid: usize,
    out: &mut usize,
) -> i32 {
    let f: unsafe extern "win64" fn(usize, *const DSTORAGE_QUEUE_DESC, usize, *mut usize) -> i32 =
        std::mem::transmute(vtbl_entry(factory, 3));
    f(factory, desc as *const DSTORAGE_QUEUE_DESC, riid, out as *mut usize)
}

unsafe fn factory_open_file(
    factory: usize,
    path: &str,
    riid: usize,
    out: &mut usize,
) -> i32 {
    let wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
    let f: unsafe extern "win64" fn(usize, *const u16, usize, *mut usize) -> i32 =
        std::mem::transmute(vtbl_entry(factory, 4));
    f(factory, wide.as_ptr(), riid, out as *mut usize)
}

unsafe fn factory_create_status_array(
    factory: usize,
    capacity: u32,
    riid: usize,
    out: &mut usize,
) -> i32 {
    let f: unsafe extern "win64" fn(usize, u32, *const u8, usize, *mut usize) -> i32 =
        std::mem::transmute(vtbl_entry(factory, 5));
    f(factory, capacity, std::ptr::null(), riid, out as *mut usize)
}

unsafe fn file_get_info(file: usize, out: &mut [u8; 52]) -> i32 {
    let f: unsafe extern "win64" fn(usize, *mut u8) -> i32 =
        std::mem::transmute(vtbl_entry(file, 4));
    f(file, out.as_mut_ptr())
}

unsafe fn status_is_complete(sa: usize, index: u32) -> bool {
    let f: unsafe extern "win64" fn(usize, u32) -> u8 =
        std::mem::transmute(vtbl_entry(sa, 3));
    f(sa, index) != 0
}

unsafe fn status_get_hresult(sa: usize, index: u32) -> i32 {
    let f: unsafe extern "win64" fn(usize, u32) -> i32 =
        std::mem::transmute(vtbl_entry(sa, 4));
    f(sa, index)
}

unsafe fn queue_enqueue_request(q: usize, req: &DSTORAGE_REQUEST) {
    let f: unsafe extern "win64" fn(usize, usize) = std::mem::transmute(vtbl_entry(q, 3));
    f(q, req as *const DSTORAGE_REQUEST as usize);
}

unsafe fn queue_enqueue_status(q: usize, sa: usize, index: u32) {
    let f: unsafe extern "win64" fn(usize, usize, u32) =
        std::mem::transmute(vtbl_entry(q, 4));
    f(q, sa, index);
}

unsafe fn queue_enqueue_signal(q: usize, fence: usize, value: u64) {
    let f: unsafe extern "win64" fn(usize, usize, u64) =
        std::mem::transmute(vtbl_entry(q, 5));
    f(q, fence, value);
}

unsafe fn queue_submit(q: usize) {
    let f: unsafe extern "win64" fn(usize) = std::mem::transmute(vtbl_entry(q, 6));
    f(q);
}

unsafe fn queue_cancel(q: usize, mask: u64, value: u64) {
    let f: unsafe extern "win64" fn(usize, u64, u64) = std::mem::transmute(vtbl_entry(q, 7));
    f(q, mask, value);
}

unsafe fn queue_close(q: usize) {
    let f: unsafe extern "win64" fn(usize) = std::mem::transmute(vtbl_entry(q, 8));
    f(q);
}

unsafe fn queue_get_error_event(q: usize) -> HANDLE {
    let f: unsafe extern "win64" fn(usize) -> HANDLE = std::mem::transmute(vtbl_entry(q, 9));
    f(q)
}

unsafe fn queue_retrieve_error_record(q: usize) -> DSTORAGE_ERROR_RECORD {
    let mut rec: DSTORAGE_ERROR_RECORD = std::mem::zeroed();
    let f: unsafe extern "win64" fn(usize, *mut DSTORAGE_ERROR_RECORD) =
        std::mem::transmute(vtbl_entry(q, 10));
    f(q, &mut rec as *mut DSTORAGE_ERROR_RECORD);
    rec
}

unsafe fn queue_query(q: usize) -> DSTORAGE_QUEUE_INFO {
    let mut info: DSTORAGE_QUEUE_INFO = std::mem::zeroed();
    let f: unsafe extern "win64" fn(usize, *mut DSTORAGE_QUEUE_INFO) =
        std::mem::transmute(vtbl_entry(q, 11));
    f(q, &mut info as *mut DSTORAGE_QUEUE_INFO);
    info
}

unsafe fn queue_enqueue_set_event(q: usize, handle: HANDLE) {
    let f: unsafe extern "win64" fn(usize, HANDLE) = std::mem::transmute(vtbl_entry(q, 12));
    f(q, handle);
}

unsafe fn queue_get_compression_support(q: usize, fmt: u8) -> u32 {
    let f: extern "win64" fn(usize, u8) -> u32 = std::mem::transmute(vtbl_entry(q, 13));
    f(q, fmt)
}

fn make_mem_to_mem_request(
    compression: u8,
    src: &[u8],
    dst: &mut [u8],
    uncompressed_size: u32,
    tag: u64,
    name: Option<&[u8]>,
) -> DSTORAGE_REQUEST {
    let mut req = DSTORAGE_REQUEST {
        options: DSTORAGE_REQUEST_OPTIONS::new(
            compression,
            DSTORAGE_REQUEST_SOURCE_MEMORY,
            DSTORAGE_REQUEST_DESTINATION_MEMORY,
        ),
        source: [0u8; 24],
        destination: [0u8; 40],
        uncompressed_size,
        _pad0: 0,
        cancellation_tag: tag,
        name: name.map_or(std::ptr::null(), |s| s.as_ptr()),
    };
    req.source[0..8].copy_from_slice(&(src.as_ptr() as u64).to_le_bytes());
    req.source[8..12].copy_from_slice(&(src.len() as u32).to_le_bytes());
    req.destination[0..8].copy_from_slice(&(dst.as_mut_ptr() as u64).to_le_bytes());
    req.destination[8..12].copy_from_slice(&(dst.len() as u32).to_le_bytes());
    req
}

fn make_file_to_mem_request(
    compression: u8,
    file: usize,
    offset: u64,
    src_size: u32,
    dst: &mut [u8],
    uncompressed_size: u32,
    tag: u64,
) -> DSTORAGE_REQUEST {
    let mut req = DSTORAGE_REQUEST {
        options: DSTORAGE_REQUEST_OPTIONS::new(
            compression,
            DSTORAGE_REQUEST_SOURCE_FILE,
            DSTORAGE_REQUEST_DESTINATION_MEMORY,
        ),
        source: [0u8; 24],
        destination: [0u8; 40],
        uncompressed_size,
        _pad0: 0,
        cancellation_tag: tag,
        name: std::ptr::null(),
    };
    req.source[0..8].copy_from_slice(&(file as u64).to_le_bytes());
    req.source[8..16].copy_from_slice(&offset.to_le_bytes());
    req.source[16..20].copy_from_slice(&src_size.to_le_bytes());
    req.destination[0..8].copy_from_slice(&(dst.as_mut_ptr() as u64).to_le_bytes());
    req.destination[8..12].copy_from_slice(&(dst.len() as u32).to_le_bytes());
    req
}

fn make_mixed_test_data(len: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(len);
    let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
    for i in 0..len {
        // Mix compressible runs and pseudorandom blocks across 64 KiB tile boundaries.
        if (i / 4096) % 3 == 0 {
            data.push((i % 29) as u8);
        } else {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            data.push((x & 0xFF) as u8);
        }
    }
    data
}

#[test]
fn dstorage_factory_and_queue_work() {
    let mut factory = 0usize;
    assert_eq!(DStorageGetFactory(0, &mut factory), S_OK);
    assert_ne!(factory, 0);

    let mut codec = 0usize;
    assert_eq!(
        DStorageCreateCompressionCodec(DSTORAGE_COMPRESSION_FORMAT_GDEFLATE, 4, 0, &mut codec),
        S_OK
    );
    assert_ne!(codec, 0);

    // DirectStorage 1.2 only defines GDeflate (1) as a built-in codec:
    // NONE (0) -> E_INVALIDARG, reserved (2..=0x7F) & custom (>= 0x80) -> E_NOTIMPL.
    let mut bad_codec = 0usize;
    assert_eq!(
        DStorageCreateCompressionCodec(DSTORAGE_COMPRESSION_FORMAT_NONE, 1, 0, &mut bad_codec),
        E_INVALIDARG
    );
    assert_eq!(bad_codec, 0);
    assert_eq!(
        DStorageCreateCompressionCodec(2, 1, 0, &mut bad_codec),
        E_NOTIMPL
    );
    assert_eq!(bad_codec, 0);
    assert_eq!(
        DStorageCreateCompressionCodec(DSTORAGE_CUSTOM_COMPRESSION_0, 1, 0, &mut bad_codec),
        E_NOTIMPL
    );
    assert_eq!(bad_codec, 0);

    // Factory SetDebugFlags (slot 6) and SetStagingBufferSize (slot 7) ordering check.
    unsafe {
        let set_dbg: unsafe extern "win64" fn(usize, u32) =
            std::mem::transmute(vtbl_entry(factory, 6));
        let set_staging: unsafe extern "win64" fn(usize, u32) -> i32 =
            std::mem::transmute(vtbl_entry(factory, 7));
        set_dbg(factory, 0);
        assert_eq!(set_staging(factory, 16 * 1024 * 1024), S_OK);

        // Queue GetCompressionSupport must report CPU_FALLBACK for GDeflate and NONE for custom.
        let desc = DSTORAGE_QUEUE_DESC {
            source_type: DSTORAGE_REQUEST_SOURCE_MEMORY as u64,
            capacity: DSTORAGE_MIN_QUEUE_CAPACITY,
            priority: 0,
            _pad0: [0; 5],
            name: std::ptr::null(),
            device: 0,
        };
        let mut queue = 0usize;
        assert_eq!(
            factory_create_queue(
                factory,
                &desc,
                &IID_IDSTORAGE_QUEUE2 as *const Guid as usize,
                &mut queue
            ),
            S_OK
        );
        assert_eq!(
            queue_get_compression_support(queue, DSTORAGE_COMPRESSION_FORMAT_GDEFLATE),
            DSTORAGE_COMPRESSION_SUPPORT_CPU_FALLBACK
        );
        assert_eq!(
            queue_get_compression_support(queue, DSTORAGE_COMPRESSION_FORMAT_NONE),
            DSTORAGE_COMPRESSION_SUPPORT_NONE
        );
        assert_eq!(
            queue_get_compression_support(queue, DSTORAGE_CUSTOM_COMPRESSION_0),
            DSTORAGE_COMPRESSION_SUPPORT_NONE
        );
        com_release(queue);
        com_release(codec);
        com_release(factory);
    }
}

#[test]
fn gdeflate_multi_tile_round_trip_and_codec_vtable() {
    let mut codec = 0usize;
    assert_eq!(
        DStorageCreateCompressionCodec(
            DSTORAGE_COMPRESSION_FORMAT_GDEFLATE,
            4,
            &IID_IDSTORAGE_COMPRESSION_CODEC as *const Guid as usize,
            &mut codec
        ),
        S_OK
    );

    // Test 0 bytes, sub-tile, exact 1 tile (64 KiB), exact 2 tiles (128 KiB),
    // and multi-tile with non-zero remainder (> 3 tiles = 200,000 bytes).
    for &(len, setting) in &[
        (0usize, DSTORAGE_COMPRESSION_DEFAULT),
        (1234, DSTORAGE_COMPRESSION_FASTEST),
        (GDEFLATE_TILE_SIZE, DSTORAGE_COMPRESSION_DEFAULT),
        (GDEFLATE_TILE_SIZE * 2, DSTORAGE_COMPRESSION_BEST_RATIO),
        (200_000, DSTORAGE_COMPRESSION_DEFAULT),
    ] {
        let input = make_mixed_test_data(len);
        unsafe {
            let bound_fn: unsafe extern "win64" fn(usize, usize) -> usize =
                std::mem::transmute(vtbl_entry(codec, 5));
            let comp_fn: unsafe extern "win64" fn(
                usize,
                *const u8,
                usize,
                i32,
                *mut u8,
                usize,
                *mut usize,
            ) -> i32 = std::mem::transmute(vtbl_entry(codec, 3));
            let decomp_fn: unsafe extern "win64" fn(
                usize,
                *const u8,
                usize,
                *mut u8,
                usize,
                *mut usize,
            ) -> i32 = std::mem::transmute(vtbl_entry(codec, 4));

            let bound = bound_fn(codec, input.len());
            let mut compressed = vec![0u8; bound];
            let mut comp_size = 0usize;
            assert_eq!(
                comp_fn(
                    codec,
                    input.as_ptr(),
                    input.len(),
                    setting,
                    compressed.as_mut_ptr(),
                    compressed.len(),
                    &mut comp_size
                ),
                S_OK
            );
            assert!(comp_size >= GDEFLATE_HEADER_SIZE);
            assert!(comp_size <= bound);
            // Verify TileStreamHeader magic & tile metadata.
            assert_eq!(compressed[0], GDEFLATE_ID);
            assert_eq!(compressed[1], GDEFLATE_MAGIC);
            let expected_tiles = if len == 0 {
                0u16
            } else {
                len.div_ceil(GDEFLATE_TILE_SIZE) as u16
            };
            assert_eq!(u16::from_le_bytes([compressed[2], compressed[3]]), expected_tiles);
            let bitfield = u32::from_le_bytes([
                compressed[4],
                compressed[5],
                compressed[6],
                compressed[7],
            ]);
            assert_eq!(bitfield & 0x3, GDEFLATE_TILE_SIZE_IDX_64K);
            assert_eq!(bitfield >> 20, 0);

            let mut output = vec![0xAAu8; input.len()];
            let mut out_size = 0usize;
            assert_eq!(
                decomp_fn(
                    codec,
                    compressed.as_ptr(),
                    comp_size,
                    output.as_mut_ptr(),
                    output.len(),
                    &mut out_size
                ),
                S_OK
            );
            assert_eq!(out_size, input.len());
            assert_eq!(output, input, "GDeflate round-trip mismatch for len={len}");
        }
    }
    unsafe {
        com_release(codec);
    }
}

#[test]
fn gdeflate_corrupt_streams_fail_honestly_never_memcpy() {
    let original = make_mixed_test_data(150_000);
    let valid = compress_gdeflate(&original, DSTORAGE_COMPRESSION_DEFAULT).unwrap();

    // 1. Plain uncompressed/random garbage (not a GDeflate stream) must fail and NOT memcpy.
    let garbage = b"NOT_GDEFLATE_RAW_BYTES_MUST_NOT_BE_MEMCPIED_INTO_OUTPUT";
    let mut dst = vec![0x5Au8; garbage.len() + 64];
    let err = decompress_gdeflate(garbage, &mut dst).unwrap_err();
    assert_eq!(err, E_DSTORAGE_DECOMPRESSION_ERROR);
    assert!(dst.iter().all(|&b| b == 0x5A), "corrupt decode must not overwrite dst");

    // 2. Bad magic byte.
    let mut bad_magic = valid.clone();
    bad_magic[1] ^= 0x01;
    let mut out = vec![0x5Au8; original.len()];
    assert_eq!(
        decompress_gdeflate(&bad_magic, &mut out),
        Err(E_DSTORAGE_DECOMPRESSION_ERROR)
    );
    assert!(out.iter().all(|&b| b == 0x5A));

    // 3. Non-zero reserved bits in header.
    let mut bad_reserved = valid.clone();
    bad_reserved[7] = 0x80;
    assert_eq!(
        decompress_gdeflate(&bad_reserved, &mut out),
        Err(E_DSTORAGE_DECOMPRESSION_ERROR)
    );

    // 4. Truncated stream.
    assert_eq!(
        decompress_gdeflate(&valid[..valid.len() / 2], &mut out),
        Err(E_DSTORAGE_DECOMPRESSION_ERROR)
    );

    // 5. Bit-flipped compressed payload in tile 2 (last tile) must fail atomically.
    let mut bad_payload = valid.clone();
    let flip_idx = bad_payload.len() - 10;
    bad_payload[flip_idx] ^= 0xFF;
    bad_payload[flip_idx - 1] ^= 0x55;
    let mut out2 = vec![0x5Au8; original.len()];
    assert_eq!(
        decompress_gdeflate(&bad_payload, &mut out2),
        Err(E_DSTORAGE_DECOMPRESSION_ERROR)
    );
    assert!(
        out2.iter().all(|&b| b == 0x5A),
        "atomic decompression must leave dst untouched on tile error"
    );

    // 6. Undersized destination buffer must fail honestly.
    let mut too_small = vec![0x5Au8; original.len() - 1];
    assert_eq!(
        decompress_gdeflate(&valid, &mut too_small),
        Err(E_DSTORAGE_DECOMPRESSION_ERROR)
    );
    assert!(too_small.iter().all(|&b| b == 0x5A));
}

#[test]
fn queue_memory_file_sync_cancel_and_error_records() {
    let mut factory = 0usize;
    assert_eq!(
        DStorageGetFactory(&IID_IDSTORAGE_FACTORY as *const Guid as usize, &mut factory),
        S_OK
    );

    // Create a memory-source queue and a file-source queue.
    let mem_desc = DSTORAGE_QUEUE_DESC {
        source_type: DSTORAGE_REQUEST_SOURCE_MEMORY as u64,
        capacity: DSTORAGE_MIN_QUEUE_CAPACITY,
        priority: 0,
        _pad0: [0; 5],
        name: b"test-mem-queue\0".as_ptr(),
        device: 0,
    };
    let file_desc = DSTORAGE_QUEUE_DESC {
        source_type: DSTORAGE_REQUEST_SOURCE_FILE as u64,
        capacity: DSTORAGE_MIN_QUEUE_CAPACITY,
        priority: 0,
        _pad0: [0; 5],
        name: b"test-file-queue\0".as_ptr(),
        device: 0,
    };

    let mut mem_q = 0usize;
    let mut file_q = 0usize;
    let mut status = 0usize;
    unsafe {
        assert_eq!(
            factory_create_queue(
                factory,
                &mem_desc,
                &IID_IDSTORAGE_QUEUE2 as *const Guid as usize,
                &mut mem_q
            ),
            S_OK
        );
        assert_eq!(
            factory_create_queue(
                factory,
                &file_desc,
                &IID_IDSTORAGE_QUEUE2 as *const Guid as usize,
                &mut file_q
            ),
            S_OK
        );
        assert_eq!(
            factory_create_status_array(
                factory,
                4,
                &IID_IDSTORAGE_STATUS_ARRAY as *const Guid as usize,
                &mut status
            ),
            S_OK
        );

        // Prepare payloads (uncompressed + multi-tile > 64 KiB GDeflate).
        let raw_src = b"DirectStorage uncompressed memory payload 0123456789".to_vec();
        let mut raw_dst = vec![0u8; raw_src.len()];

        let big_plain = make_mixed_test_data(140_000);
        let big_comp = compress_gdeflate(&big_plain, DSTORAGE_COMPRESSION_DEFAULT).unwrap();
        let mut big_dst = vec![0u8; big_plain.len()];

        let cancel_src = b"SHOULD_BE_CANCELLED_BEFORE_SUBMIT".to_vec();
        let mut cancel_dst = vec![0xCCu8; cancel_src.len()];

        let req_raw = make_mem_to_mem_request(
            DSTORAGE_COMPRESSION_FORMAT_NONE,
            &raw_src,
            &mut raw_dst,
            raw_src.len() as u32,
            0x0001_0001,
            Some(b"raw_mem\0"),
        );
        let req_gdeflate = make_mem_to_mem_request(
            DSTORAGE_COMPRESSION_FORMAT_GDEFLATE,
            &big_comp,
            &mut big_dst,
            big_plain.len() as u32,
            0x0001_0002,
            Some(b"gdeflate_mem\0"),
        );
        let req_cancel = make_mem_to_mem_request(
            DSTORAGE_COMPRESSION_FORMAT_NONE,
            &cancel_src,
            &mut cancel_dst,
            cancel_src.len() as u32,
            0xDEAD_0042,
            Some(b"cancelled_req\0"),
        );

        let ev = crate::kernel32::threads::CreateEventA(
            std::ptr::null_mut(),
            1,
            0,
            std::ptr::null(),
        );
        assert!(!ev.is_null());

        // Enqueue all requests + status + set_event BEFORE Submit().
        queue_enqueue_request(mem_q, &req_raw);
        queue_enqueue_request(mem_q, &req_gdeflate);
        queue_enqueue_request(mem_q, &req_cancel);
        queue_enqueue_status(mem_q, status, 0);
        queue_enqueue_set_event(mem_q, ev);

        // Verify real Queue::Query counts before Submit().
        let info_before = queue_query(mem_q);
        assert_eq!(info_before.desc.capacity, DSTORAGE_MIN_QUEUE_CAPACITY);
        assert_eq!(info_before.empty_slot_count, DSTORAGE_MIN_QUEUE_CAPACITY - 5);
        assert_eq!(info_before.request_count_until_auto_submit, 64 - 5);

        // Status[0] must be pending and event unsignaled before Submit().
        assert!(!status_is_complete(status, 0));
        assert_eq!(status_get_hresult(status, 0), E_PENDING);
        assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev, 0), WAIT_TIMEOUT);

        // Cancel req_cancel by tag mask before Submit().
        queue_cancel(mem_q, 0xFFFF_0000, 0xDEAD_0000);
        let info_after_cancel = queue_query(mem_q);
        assert_eq!(
            info_after_cancel.empty_slot_count,
            DSTORAGE_MIN_QUEUE_CAPACITY - 4
        );

        // Submit and verify execution + sync.
        queue_submit(mem_q);

        let info_after_submit = queue_query(mem_q);
        assert_eq!(info_after_submit.empty_slot_count, DSTORAGE_MIN_QUEUE_CAPACITY);
        assert_eq!(raw_dst, raw_src);
        assert_eq!(big_dst, big_plain);
        assert!(
            cancel_dst.iter().all(|&b| b == 0xCC),
            "cancelled request must not modify destination"
        );
        assert!(status_is_complete(status, 0));
        assert_eq!(status_get_hresult(status, 0), S_OK);
        assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev, 0), WAIT_OBJECT_0);
        let clean_err = queue_retrieve_error_record(mem_q);
        assert_eq!(clean_err.failure_count, 0);
        assert_eq!(clean_err.first_failure.command_type, DSTORAGE_COMMAND_TYPE_NONE);
        crate::kernel32::threads::CloseHandle(ev);

        // ---- File-to-Memory uncompressed + GDeflate ----
        let tmp_path = std::env::temp_dir().join(format!(
            "dstorage_test_{}_{}.bin",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let mut file_blob = Vec::new();
        let raw_offset = 0u64;
        file_blob.extend_from_slice(&raw_src);
        let gdef_offset = file_blob.len() as u64;
        file_blob.extend_from_slice(&big_comp);
        std::fs::write(&tmp_path, &file_blob).unwrap();

        // Nonexistent file must fail honestly.
        let mut missing_file = 0usize;
        assert!(
            factory_open_file(
                factory,
                "/nonexistent/path/definitely_missing_dstorage_file.bin",
                &IID_IDSTORAGE_FILE as *const Guid as usize,
                &mut missing_file
            ) < 0
        );
        assert_eq!(missing_file, 0);

        let mut ds_file = 0usize;
        let tmp_str = tmp_path.to_string_lossy().into_owned();
        assert_eq!(
            factory_open_file(
                factory,
                &tmp_str,
                &IID_IDSTORAGE_FILE as *const Guid as usize,
                &mut ds_file
            ),
            S_OK
        );
        let mut file_info = [0u8; 52];
        assert_eq!(file_get_info(ds_file, &mut file_info), S_OK);
        let size_hi = u32::from_le_bytes(file_info[32..36].try_into().unwrap()) as u64;
        let size_lo = u32::from_le_bytes(file_info[36..40].try_into().unwrap()) as u64;
        assert_eq!((size_hi << 32) | size_lo, file_blob.len() as u64);

        let mut file_raw_dst = vec![0u8; raw_src.len()];
        let mut file_gdef_dst = vec![0u8; big_plain.len()];
        let freq1 = make_file_to_mem_request(
            DSTORAGE_COMPRESSION_FORMAT_NONE,
            ds_file,
            raw_offset,
            raw_src.len() as u32,
            &mut file_raw_dst,
            raw_src.len() as u32,
            1,
        );
        let freq2 = make_file_to_mem_request(
            DSTORAGE_COMPRESSION_FORMAT_GDEFLATE,
            ds_file,
            gdef_offset,
            big_comp.len() as u32,
            &mut file_gdef_dst,
            big_plain.len() as u32,
            2,
        );
        queue_enqueue_request(file_q, &freq1);
        queue_enqueue_request(file_q, &freq2);
        queue_enqueue_status(file_q, status, 1);
        queue_submit(file_q);

        assert!(status_is_complete(status, 1));
        assert_eq!(status_get_hresult(status, 1), S_OK);
        assert_eq!(file_raw_dst, raw_src);
        assert_eq!(file_gdef_dst, big_plain);

        // ---- Error record + GetErrorEvent + invalid Tiles + unknown built-in format ----
        let err_ev = queue_get_error_event(mem_q);
        assert!(!err_ev.is_null());
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(err_ev, 0),
            WAIT_TIMEOUT
        );

        let corrupt_src = b"CORRUPT_GDEFLATE_STREAM_PAYLOAD".to_vec();
        let mut corrupt_dst = vec![0x77u8; 64];
        let req_corrupt = make_mem_to_mem_request(
            DSTORAGE_COMPRESSION_FORMAT_GDEFLATE,
            &corrupt_src,
            &mut corrupt_dst,
            64,
            0x1111,
            Some(b"bad_gdef\0"),
        );
        // Null resource in TILES destination must fail honestly with E_INVALIDARG.
        let mut tiles_req = DSTORAGE_REQUEST {
            options: DSTORAGE_REQUEST_OPTIONS::new(
                DSTORAGE_COMPRESSION_FORMAT_NONE,
                DSTORAGE_REQUEST_SOURCE_MEMORY,
                DSTORAGE_REQUEST_DESTINATION_TILES,
            ),
            source: [0u8; 24],
            destination: [0u8; 40],
            uncompressed_size: 4,
            _pad0: 0,
            cancellation_tag: 0x2222,
            name: b"tiles_null_res\0".as_ptr(),
        };
        tiles_req.source[0..8].copy_from_slice(&(raw_src.as_ptr() as u64).to_le_bytes());
        tiles_req.source[8..12].copy_from_slice(&4u32.to_le_bytes());

        // Reserved unknown built-in compression format (0x02 < 0x80) must fail with E_INVALIDARG.
        let mut unknown_fmt_dst = vec![0u8; 16];
        let req_unknown_fmt = make_mem_to_mem_request(
            0x02,
            &raw_src[..16],
            &mut unknown_fmt_dst,
            16,
            0x3333,
            Some(b"unknown_fmt\0"),
        );

        queue_enqueue_request(mem_q, &req_corrupt);
        queue_enqueue_request(mem_q, &tiles_req);
        queue_enqueue_status(mem_q, status, 2);
        queue_enqueue_request(mem_q, &req_unknown_fmt);
        queue_enqueue_status(mem_q, status, 3);
        queue_submit(mem_q);

        // Corrupt GDeflate must not overwrite dst.
        assert!(corrupt_dst.iter().all(|&b| b == 0x77));
        // Status[2] reports first error of batch 1 (corrupt GDeflate), Status[3] reports E_INVALIDARG.
        assert!(status_is_complete(status, 2));
        assert_eq!(status_get_hresult(status, 2), E_DSTORAGE_DECOMPRESSION_ERROR);
        assert!(status_is_complete(status, 3));
        assert_eq!(status_get_hresult(status, 3), E_INVALIDARG);
        // Error event must now be signaled.
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(err_ev, 0),
            WAIT_OBJECT_0
        );
        let err_rec = queue_retrieve_error_record(mem_q);
        assert_eq!(err_rec.failure_count, 3);
        assert_eq!(err_rec.first_failure.hresult, E_DSTORAGE_DECOMPRESSION_ERROR);
        assert_eq!(err_rec.first_failure.command_type, DSTORAGE_COMMAND_TYPE_REQUEST);
        let req_name_bytes = &err_rec.first_failure.params.request.request_name;
        assert!(req_name_bytes.starts_with(b"bad_gdef\0"));
        // Retrieving the error record resets both failure_count and the error event.
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(err_ev, 0),
            WAIT_TIMEOUT
        );
        let err_rec2 = queue_retrieve_error_record(mem_q);
        assert_eq!(err_rec2.failure_count, 0);
        crate::kernel32::threads::CloseHandle(err_ev);

        // Close() drops pending requests without executing them.
        let mut closed_dst = vec![0x99u8; raw_src.len()];
        let req_after_close = make_mem_to_mem_request(
            DSTORAGE_COMPRESSION_FORMAT_NONE,
            &raw_src,
            &mut closed_dst,
            raw_src.len() as u32,
            0x4444,
            None,
        );
        queue_enqueue_request(mem_q, &req_after_close);
        queue_close(mem_q);
        queue_submit(mem_q);
        assert!(closed_dst.iter().all(|&b| b == 0x99));

        com_release(ds_file);
        com_release(status);
        com_release(file_q);
        com_release(mem_q);
        com_release(factory);
        let _ = std::fs::remove_file(&tmp_path);
    }
}

#[test]
fn dstorage_gpu_buffer_texture_and_fence_upload() {
    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let mut factory = 0usize;
        assert_eq!(DStorageGetFactory(0, &mut factory), S_OK);
        let desc = DSTORAGE_QUEUE_DESC {
            source_type: DSTORAGE_REQUEST_SOURCE_MEMORY as u64,
            capacity: DSTORAGE_MIN_QUEUE_CAPACITY,
            priority: 0,
            _pad0: [0; 5],
            name: std::ptr::null(),
            device: 0,
        };
        let mut q = 0usize;
        assert_eq!(factory_create_queue(factory, &desc, 0, &mut q), S_OK);

        // Create a DEFAULT heap D3D12 buffer and 2-mip 2D texture + real ID3D12Fence.
        let hp = HeapProperties {
            Type: heap_type::DEFAULT,
            ..Default::default()
        };
        let buf_desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: 4096,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let mut buf_res: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crate::d3d12::resource::create_committed(
                &hp,
                0,
                &buf_desc,
                0,
                std::ptr::null(),
                &crate::d3d12::vtables::IID_ID3D12Resource,
                &mut buf_res,
            ),
            S_OK
        );

        let tex_desc = ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Width: 8,
            Height: 8,
            DepthOrArraySize: 1,
            MipLevels: 2,
            Format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
            SampleCount: 1,
            ..Default::default()
        };
        let mut tex_res: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crate::d3d12::resource::create_committed(
                &hp,
                0,
                &tex_desc,
                0,
                std::ptr::null(),
                &crate::d3d12::vtables::IID_ID3D12Resource,
                &mut tex_res,
            ),
            S_OK
        );

        let mtl_fence = d3d12_metal::metal::fence::Fence::new(0).unwrap();
        let fence_obj = Obj::new(crate::d3d12::fence::Fence {
            f: mtl_fence,
            flags: 0,
        });

        // 1. GDeflate-compressed upload to D3D12 buffer at offset 256.
        let buf_plain = make_mixed_test_data(1024);
        let buf_comp = compress_gdeflate(&buf_plain, DSTORAGE_COMPRESSION_DEFAULT).unwrap();
        let mut req_buf = DSTORAGE_REQUEST {
            options: DSTORAGE_REQUEST_OPTIONS::new(
                DSTORAGE_COMPRESSION_FORMAT_GDEFLATE,
                DSTORAGE_REQUEST_SOURCE_MEMORY,
                DSTORAGE_REQUEST_DESTINATION_BUFFER,
            ),
            source: [0u8; 24],
            destination: [0u8; 40],
            uncompressed_size: buf_plain.len() as u32,
            _pad0: 0,
            cancellation_tag: 1,
            name: std::ptr::null(),
        };
        req_buf.source[0..8].copy_from_slice(&(buf_comp.as_ptr() as u64).to_le_bytes());
        req_buf.source[8..12].copy_from_slice(&(buf_comp.len() as u32).to_le_bytes());
        req_buf.destination[0..8].copy_from_slice(&(buf_res as u64).to_le_bytes());
        req_buf.destination[8..16].copy_from_slice(&256u64.to_le_bytes());
        req_buf.destination[16..20].copy_from_slice(&(buf_plain.len() as u32).to_le_bytes());

        // 2. Texture region upload (4x4 sub-region of mip 0, tightly packed RGBA8 = 64 bytes).
        let region_pixels = vec![0xABu8; 4 * 4 * 4];
        let mut req_tex_reg = DSTORAGE_REQUEST {
            options: DSTORAGE_REQUEST_OPTIONS::new(
                DSTORAGE_COMPRESSION_FORMAT_NONE,
                DSTORAGE_REQUEST_SOURCE_MEMORY,
                DSTORAGE_REQUEST_DESTINATION_TEXTURE_REGION,
            ),
            source: [0u8; 24],
            destination: [0u8; 40],
            uncompressed_size: region_pixels.len() as u32,
            _pad0: 0,
            cancellation_tag: 2,
            name: std::ptr::null(),
        };
        req_tex_reg.source[0..8]
            .copy_from_slice(&(region_pixels.as_ptr() as u64).to_le_bytes());
        req_tex_reg.source[8..12]
            .copy_from_slice(&(region_pixels.len() as u32).to_le_bytes());
        req_tex_reg.destination[0..8].copy_from_slice(&(tex_res as u64).to_le_bytes());
        req_tex_reg.destination[8..12].copy_from_slice(&0u32.to_le_bytes()); // subresource 0
        req_tex_reg.destination[12..16].copy_from_slice(&2u32.to_le_bytes()); // left
        req_tex_reg.destination[16..20].copy_from_slice(&2u32.to_le_bytes()); // top
        req_tex_reg.destination[20..24].copy_from_slice(&0u32.to_le_bytes()); // front
        req_tex_reg.destination[24..28].copy_from_slice(&6u32.to_le_bytes()); // right
        req_tex_reg.destination[28..32].copy_from_slice(&6u32.to_le_bytes()); // bottom
        req_tex_reg.destination[32..36].copy_from_slice(&1u32.to_le_bytes()); // back

        // 3. Multiple subresources upload (mip 0: 8x8x4 = 256 bytes, mip 1: 4x4x4 = 64 bytes).
        let mips_pixels = vec![0xCDu8; 8 * 8 * 4 + 4 * 4 * 4];
        let mut req_mips = DSTORAGE_REQUEST {
            options: DSTORAGE_REQUEST_OPTIONS::new(
                DSTORAGE_COMPRESSION_FORMAT_NONE,
                DSTORAGE_REQUEST_SOURCE_MEMORY,
                DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES,
            ),
            source: [0u8; 24],
            destination: [0u8; 40],
            uncompressed_size: mips_pixels.len() as u32,
            _pad0: 0,
            cancellation_tag: 3,
            name: std::ptr::null(),
        };
        req_mips.source[0..8].copy_from_slice(&(mips_pixels.as_ptr() as u64).to_le_bytes());
        req_mips.source[8..12].copy_from_slice(&(mips_pixels.len() as u32).to_le_bytes());
        req_mips.destination[0..8].copy_from_slice(&(tex_res as u64).to_le_bytes());
        req_mips.destination[8..12].copy_from_slice(&0u32.to_le_bytes()); // first_subresource

        // 4. Multiple subresources range upload (mip 1 only: 4x4x4 = 64 bytes).
        let mip1_pixels = vec![0xEFu8; 4 * 4 * 4];
        let mut req_mip_range = DSTORAGE_REQUEST {
            options: DSTORAGE_REQUEST_OPTIONS::new(
                DSTORAGE_COMPRESSION_FORMAT_NONE,
                DSTORAGE_REQUEST_SOURCE_MEMORY,
                DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES_RANGE,
            ),
            source: [0u8; 24],
            destination: [0u8; 40],
            uncompressed_size: mip1_pixels.len() as u32,
            _pad0: 0,
            cancellation_tag: 4,
            name: std::ptr::null(),
        };
        req_mip_range.source[0..8]
            .copy_from_slice(&(mip1_pixels.as_ptr() as u64).to_le_bytes());
        req_mip_range.source[8..12]
            .copy_from_slice(&(mip1_pixels.len() as u32).to_le_bytes());
        req_mip_range.destination[0..8].copy_from_slice(&(tex_res as u64).to_le_bytes());
        req_mip_range.destination[8..12].copy_from_slice(&1u32.to_le_bytes()); // first_subresource
        req_mip_range.destination[12..16].copy_from_slice(&1u32.to_le_bytes()); // num_subresources

        queue_enqueue_request(q, &req_buf);
        queue_enqueue_request(q, &req_tex_reg);
        queue_enqueue_request(q, &req_mips);
        queue_enqueue_request(q, &req_mip_range);
        queue_enqueue_signal(q, fence_obj as usize, 42);

        // Fence must not signal before Submit().
        assert_eq!(Obj::get(fence_obj).f.completed(), 0);

        queue_submit(q);

        assert_eq!(Obj::get(fence_obj).f.completed(), 42);
        let err_rec = queue_retrieve_error_record(q);
        assert_eq!(err_rec.failure_count, 0);

        // Verify GPU buffer received exact decompressed bytes at offset 256.
        let res_ref = crate::d3d12::resource::from_ptr(buf_res).unwrap();
        let landed = res_ref.r.cpu_bytes(256, buf_plain.len()).unwrap();
        assert_eq!(landed, buf_plain);

        Obj::release(fence_obj);
        crate::d3d12::com::com_release(tex_res);
        crate::d3d12::com::com_release(buf_res);
        com_release(q);
        com_release(factory);
    }
}

fn make_tiles_request(
    compression: u8,
    src: &[u8],
    res_ptr: *mut c_void,
    coord: TileCoord,
    region: TileRegionSize,
    uncompressed_size: u32,
    tag: u64,
    name: Option<&[u8]>,
) -> DSTORAGE_REQUEST {
    let mut req = DSTORAGE_REQUEST {
        options: DSTORAGE_REQUEST_OPTIONS::new(
            compression,
            DSTORAGE_REQUEST_SOURCE_MEMORY,
            DSTORAGE_REQUEST_DESTINATION_TILES,
        ),
        source: [0u8; 24],
        destination: [0u8; 40],
        uncompressed_size,
        _pad0: 0,
        cancellation_tag: tag,
        name: name.map_or(std::ptr::null(), |s| s.as_ptr()),
    };
    req.source[0..8].copy_from_slice(&(src.as_ptr() as u64).to_le_bytes());
    req.source[8..12].copy_from_slice(&(src.len() as u32).to_le_bytes());
    req.destination[0..8].copy_from_slice(&(res_ptr as u64).to_le_bytes());
    req.destination[8..12].copy_from_slice(&coord.x.to_le_bytes());
    req.destination[12..16].copy_from_slice(&coord.y.to_le_bytes());
    req.destination[16..20].copy_from_slice(&coord.z.to_le_bytes());
    req.destination[20..24].copy_from_slice(&coord.subresource.to_le_bytes());
    req.destination[24..28].copy_from_slice(&region.num_tiles.to_le_bytes());
    req.destination[28..32].copy_from_slice(&u32::from(region.use_box).to_le_bytes());
    req.destination[32..36].copy_from_slice(&region.width.to_le_bytes());
    req.destination[36..38].copy_from_slice(&region.height.to_le_bytes());
    req.destination[38..40].copy_from_slice(&region.depth.to_le_bytes());
    req
}

fn readback_tex2d_rgba8(tex: &Arc<MtlResource>, w: u32, h: u32) -> Vec<u8> {
    let row_bytes = (w as usize) * 4;
    let row_pitch = row_bytes.next_multiple_of(256).max(256);
    let total = (row_pitch * h as usize) as u64;
    let rb_desc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: total,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let rb_hp = HeapProperties {
        Type: heap_type::READBACK,
        ..Default::default()
    };
    let rb = Arc::new(MtlResource::new_committed_default(&rb_desc, &rb_hp, "dstorage rb").unwrap());
    rb.register_ptr();

    let queue = MtlQueue::new(ListType::Copy, "dstorage rb queue").unwrap();
    let alloc = MtlCommandAllocator::new(ListType::Copy).unwrap();
    let mut cl = MtlCommandList::new(ListType::Copy).unwrap();
    cl.begin(&alloc).unwrap();
    let fp = PlacedFootprint {
        offset: 0,
        format: 28,
        width: w,
        height: h,
        depth: 1,
        row_pitch: row_pitch as u32,
        _pad: 0,
    };
    cl.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(tex, 0), None)
        .unwrap();
    cl.close().unwrap();
    let v = queue.submit(&[&cl.cb]);
    queue.timeline.wait_cpu(v, u64::MAX);

    let raw = rb.cpu_bytes(0, total as usize).unwrap();
    let mut tightly_packed = Vec::with_capacity(row_bytes * h as usize);
    for y in 0..h as usize {
        let off = y * row_pitch;
        tightly_packed.extend_from_slice(&raw[off..off + row_bytes]);
    }
    tightly_packed
}

#[test]
fn dstorage_tiles_gpu_upload_and_readback() {
    // Verify D3D12 standard 64 KiB tile geometry across common 2D, BC, and 3D formats.
    let desc2d = |fmt: u32| ResourceDesc {
        Dimension: dim::TEXTURE2D,
        Width: 512,
        Height: 512,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: fmt,
        SampleCount: 1,
        ..Default::default()
    };
    let desc3d = |fmt: u32| ResourceDesc {
        Dimension: dim::TEXTURE3D,
        Width: 64,
        Height: 64,
        DepthOrArraySize: 32,
        MipLevels: 1,
        Format: fmt,
        SampleCount: 1,
        ..Default::default()
    };
    assert_eq!(standard_tile_shape(&desc2d(61), 0), Some((256, 256, 1))); // R8_UNORM (8 bpp)
    assert_eq!(standard_tile_shape(&desc2d(54), 0), Some((256, 128, 1))); // R16_FLOAT (16 bpp)
    assert_eq!(standard_tile_shape(&desc2d(28), 0), Some((128, 128, 1))); // R8G8B8A8_UNORM (32 bpp)
    assert_eq!(standard_tile_shape(&desc2d(10), 0), Some((128, 64, 1)));  // R16G16B16A16_FLOAT (64 bpp)
    assert_eq!(standard_tile_shape(&desc2d(2), 0), Some((64, 64, 1)));    // R32G32B32A32_FLOAT (128 bpp)
    assert_eq!(standard_tile_shape(&desc2d(71), 0), Some((512, 256, 1))); // BC1_UNORM (8 B/4x4)
    assert_eq!(standard_tile_shape(&desc2d(98), 0), Some((256, 256, 1))); // BC7_UNORM (16 B/4x4)
    assert_eq!(standard_tile_shape(&desc3d(28), 0), Some((32, 32, 16)));  // 3D RGBA8
    assert_eq!(standard_tile_shape(&desc3d(2), 0), Some((16, 16, 16)));   // 3D RGBA32F

    if d3d12_metal::metal::device().is_none() {
        return;
    }
    unsafe {
        let mut factory = 0usize;
        assert_eq!(DStorageGetFactory(0, &mut factory), S_OK);
        let qdesc = DSTORAGE_QUEUE_DESC {
            source_type: DSTORAGE_REQUEST_SOURCE_MEMORY as u64,
            capacity: DSTORAGE_MIN_QUEUE_CAPACITY,
            priority: 0,
            _pad0: [0; 5],
            name: b"tiles-queue\0".as_ptr(),
            device: 0,
        };
        let mut q = 0usize;
        let mut status = 0usize;
        assert_eq!(factory_create_queue(factory, &qdesc, 0, &mut q), S_OK);
        assert_eq!(factory_create_status_array(factory, 4, 0, &mut status), S_OK);

        // 256x256 RGBA8 = 2x2 grid of 128x128 standard 64 KiB tiles.
        let hp = HeapProperties {
            Type: heap_type::DEFAULT,
            ..Default::default()
        };
        let tex256_desc = ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Width: 256,
            Height: 256,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
            SampleCount: 1,
            ..Default::default()
        };
        let mut tex256_res: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crate::d3d12::resource::create_committed(
                &hp,
                0,
                &tex256_desc,
                0,
                std::ptr::null(),
                &crate::d3d12::vtables::IID_ID3D12Resource,
                &mut tex256_res,
            ),
            S_OK
        );

        // Fill each tile with a distinct RGBA pattern (128x128x4 = 65536 bytes per tile).
        let make_tile = |r: u8, g: u8, b: u8, a: u8| -> Vec<u8> {
            let mut v = vec![0u8; D3D12_TILE_SIZE_BYTES];
            for y in 0..128usize {
                for x in 0..128usize {
                    let off = (y * 128 + x) * 4;
                    v[off] = r.wrapping_add(x as u8);
                    v[off + 1] = g.wrapping_add(y as u8);
                    v[off + 2] = b;
                    v[off + 3] = a;
                }
            }
            v
        };
        let tile_00 = make_tile(0x10, 0x20, 0x30, 0xFF);
        let tile_10 = make_tile(0x40, 0x50, 0x60, 0xFF);
        let tile_01 = make_tile(0x70, 0x80, 0x90, 0xFF);
        let tile_11 = make_tile(0xA0, 0xB0, 0xC0, 0xFF);

        // 1. Upload tile (0,0) via UseBox=TRUE (1x1x1) compressed with GDeflate.
        let tile_00_comp = compress_gdeflate(&tile_00, DSTORAGE_COMPRESSION_FASTEST).unwrap();
        let req_00 = make_tiles_request(
            DSTORAGE_COMPRESSION_FORMAT_GDEFLATE,
            &tile_00_comp,
            tex256_res,
            TileCoord { x: 0, y: 0, z: 0, subresource: 0 },
            TileRegionSize { num_tiles: 1, use_box: true, width: 1, height: 1, depth: 1 },
            D3D12_TILE_SIZE_BYTES as u32,
            1,
            Some(b"tile_00\0"),
        );

        // 2. Upload tiles (1,0) and (0,1) via UseBox=FALSE linear wrap-around (NumTiles=2).
        let mut linear_pair = Vec::with_capacity(D3D12_TILE_SIZE_BYTES * 2);
        linear_pair.extend_from_slice(&tile_10);
        linear_pair.extend_from_slice(&tile_01);
        let req_lin = make_tiles_request(
            DSTORAGE_COMPRESSION_FORMAT_NONE,
            &linear_pair,
            tex256_res,
            TileCoord { x: 1, y: 0, z: 0, subresource: 0 },
            TileRegionSize { num_tiles: 2, use_box: false, width: 0, height: 0, depth: 0 },
            (D3D12_TILE_SIZE_BYTES * 2) as u32,
            2,
            Some(b"tile_lin\0"),
        );

        // 3. Upload tile (1,1) via UseBox=TRUE (1x1x1) uncompressed.
        let req_11 = make_tiles_request(
            DSTORAGE_COMPRESSION_FORMAT_NONE,
            &tile_11,
            tex256_res,
            TileCoord { x: 1, y: 1, z: 0, subresource: 0 },
            TileRegionSize { num_tiles: 1, use_box: true, width: 1, height: 1, depth: 1 },
            D3D12_TILE_SIZE_BYTES as u32,
            3,
            Some(b"tile_11\0"),
        );

        // 4. Partial edge tile upload on a 192x192 RGBA8 texture (tile (1,1) covers 64x64 pixels).
        let tex192_desc = ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Width: 192,
            Height: 192,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: 28,
            SampleCount: 1,
            ..Default::default()
        };
        let mut tex192_res: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            crate::d3d12::resource::create_committed(
                &hp,
                0,
                &tex192_desc,
                0,
                std::ptr::null(),
                &crate::d3d12::vtables::IID_ID3D12Resource,
                &mut tex192_res,
            ),
            S_OK
        );
        let req_edge = make_tiles_request(
            DSTORAGE_COMPRESSION_FORMAT_NONE,
            &tile_11,
            tex192_res,
            TileCoord { x: 1, y: 1, z: 0, subresource: 0 },
            TileRegionSize { num_tiles: 1, use_box: true, width: 1, height: 1, depth: 1 },
            D3D12_TILE_SIZE_BYTES as u32,
            4,
            Some(b"tile_edge\0"),
        );

        queue_enqueue_request(q, &req_00);
        queue_enqueue_request(q, &req_lin);
        queue_enqueue_request(q, &req_11);
        queue_enqueue_request(q, &req_edge);
        queue_enqueue_status(q, status, 0);
        queue_submit(q);

        assert!(status_is_complete(status, 0));
        assert_eq!(status_get_hresult(status, 0), S_OK);
        let err_rec = queue_retrieve_error_record(q);
        assert_eq!(err_rec.failure_count, 0);

        // Read back 256x256 GPU texture and verify every pixel in all 4 tile quadrants.
        let tex256_ref = crate::d3d12::resource::from_ptr(tex256_res).unwrap();
        let px256 = readback_tex2d_rgba8(&tex256_ref.r, 256, 256);
        let check_px = |px: &[u8], w: usize, x: usize, y: usize, expected: [u8; 4]| {
            let off = (y * w + x) * 4;
            assert_eq!(&px[off..off + 4], &expected, "pixel mismatch at ({x}, {y})");
        };
        // Sample corners & interior of all 4 tiles:
        check_px(&px256, 256, 0, 0, [0x10, 0x20, 0x30, 0xFF]);
        check_px(&px256, 256, 127, 127, [0x10u8.wrapping_add(127), 0x20u8.wrapping_add(127), 0x30, 0xFF]);
        check_px(&px256, 256, 128, 0, [0x40, 0x50, 0x60, 0xFF]);
        check_px(&px256, 256, 255, 63, [0x40u8.wrapping_add(127), 0x50u8.wrapping_add(63), 0x60, 0xFF]);
        check_px(&px256, 256, 0, 128, [0x70, 0x80, 0x90, 0xFF]);
        check_px(&px256, 256, 64, 255, [0x70u8.wrapping_add(64), 0x80u8.wrapping_add(127), 0x90, 0xFF]);
        check_px(&px256, 256, 128, 128, [0xA0, 0xB0, 0xC0, 0xFF]);
        check_px(&px256, 256, 255, 255, [0xA0u8.wrapping_add(127), 0xB0u8.wrapping_add(127), 0xC0, 0xFF]);

        // Read back 192x192 texture and verify partial edge tile (1,1) landed at (128..192, 128..192).
        let tex192_ref = crate::d3d12::resource::from_ptr(tex192_res).unwrap();
        let px192 = readback_tex2d_rgba8(&tex192_ref.r, 192, 192);
        check_px(&px192, 192, 128, 128, [0xA0, 0xB0, 0xC0, 0xFF]);
        check_px(&px192, 192, 191, 191, [0xA0u8.wrapping_add(63), 0xB0u8.wrapping_add(63), 0xC0, 0xFF]);

        // Out-of-bounds tile coordinate must fail honestly with E_INVALIDARG.
        let req_oob = make_tiles_request(
            DSTORAGE_COMPRESSION_FORMAT_NONE,
            &tile_00,
            tex256_res,
            TileCoord { x: 2, y: 0, z: 0, subresource: 0 },
            TileRegionSize { num_tiles: 1, use_box: true, width: 1, height: 1, depth: 1 },
            D3D12_TILE_SIZE_BYTES as u32,
            5,
            Some(b"tile_oob\0"),
        );
        queue_enqueue_request(q, &req_oob);
        queue_enqueue_status(q, status, 1);
        queue_submit(q);
        assert!(status_is_complete(status, 1));
        assert_eq!(status_get_hresult(status, 1), E_INVALIDARG);
        let oob_rec = queue_retrieve_error_record(q);
        assert_eq!(oob_rec.failure_count, 1);
        assert_eq!(oob_rec.first_failure.hresult, E_INVALIDARG);

        crate::d3d12::com::com_release(tex192_res);
        crate::d3d12::com::com_release(tex256_res);
        com_release(status);
        com_release(q);
        com_release(factory);
    }
}

#[test]
fn dstorage_custom_decompression_queue_async_roundtrip() {
    unsafe {
        let mut factory = 0usize;
        assert_eq!(
            DStorageGetFactory(&IID_IDSTORAGE_FACTORY as *const Guid as usize, &mut factory),
            S_OK
        );
        assert_ne!(factory, 0);

        // Obtain IDStorageCustomDecompressionQueue and Queue1 via Factory::QueryInterface
        // and directly via DStorageGetFactory.
        let mut custom_q = 0usize;
        let mut custom_q1 = 0usize;
        let mut custom_direct = 0usize;
        assert_eq!(
            com_qi(factory, &IID_IDSTORAGE_CUSTOM_DECOMPRESSION_QUEUE, &mut custom_q),
            S_OK
        );
        assert_eq!(
            com_qi(factory, &IID_IDSTORAGE_CUSTOM_DECOMPRESSION_QUEUE1, &mut custom_q1),
            S_OK
        );
        assert_eq!(
            DStorageGetFactory(
                &IID_IDSTORAGE_CUSTOM_DECOMPRESSION_QUEUE1 as *const Guid as usize,
                &mut custom_direct
            ),
            S_OK
        );
        assert_ne!(custom_q, 0);
        assert_eq!(custom_q, custom_q1);
        assert_eq!(custom_q, custom_direct);

        // Reverse QI from custom decompression queue back to IDStorageFactory.
        let mut factory_back = 0usize;
        assert_eq!(com_qi(custom_q, &IID_IDSTORAGE_FACTORY, &mut factory_back), S_OK);
        assert_eq!(factory_back, factory);
        com_release(factory_back);

        let custom_ev = custom_decomp_get_event(custom_q);
        assert!(!custom_ev.is_null());
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(custom_ev, 0),
            WAIT_TIMEOUT
        );

        let qdesc = DSTORAGE_QUEUE_DESC {
            source_type: DSTORAGE_REQUEST_SOURCE_MEMORY as u64,
            capacity: DSTORAGE_MIN_QUEUE_CAPACITY,
            priority: 0,
            _pad0: [0; 5],
            name: b"custom-decomp-q\0".as_ptr(),
            device: 0,
        };
        let mut q = 0usize;
        let mut status = 0usize;
        assert_eq!(factory_create_queue(factory, &qdesc, 0, &mut q), S_OK);
        assert_eq!(factory_create_status_array(factory, 4, 0, &mut status), S_OK);

        // Custom compression codec (format 0x80: XOR 0x5A, format 0x81: XOR 0xA5).
        let plain1 = make_mixed_test_data(4096);
        let encoded1: Vec<u8> = plain1.iter().map(|&b| b ^ 0x5A).collect();
        let mut dst1 = vec![0xEEu8; plain1.len()];

        let plain2 = make_mixed_test_data(2048);
        let encoded2: Vec<u8> = plain2.iter().map(|&b| b ^ 0xA5).collect();
        let mut dst2 = vec![0xDDu8; plain2.len()];

        let has_metal = d3d12_metal::metal::device().is_some();
        let mut gpu_buf: *mut c_void = std::ptr::null_mut();
        let mut fence_obj: *mut Obj<crate::d3d12::fence::Fence> = std::ptr::null_mut();
        if has_metal {
            let hp = HeapProperties {
                Type: heap_type::DEFAULT,
                ..Default::default()
            };
            let bdesc = ResourceDesc {
                Dimension: dim::BUFFER,
                Width: 4096,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleCount: 1,
                Layout: 1,
                ..Default::default()
            };
            assert_eq!(
                crate::d3d12::resource::create_committed(
                    &hp,
                    0,
                    &bdesc,
                    0,
                    std::ptr::null(),
                    &crate::d3d12::vtables::IID_ID3D12Resource,
                    &mut gpu_buf,
                ),
                S_OK
            );
            let mtl_fence = d3d12_metal::metal::fence::Fence::new(0).unwrap();
            fence_obj = Obj::new(crate::d3d12::fence::Fence {
                f: mtl_fence,
                flags: 0,
            });
        }

        let req1 = make_mem_to_mem_request(
            DSTORAGE_CUSTOM_COMPRESSION_0,
            &encoded1,
            &mut dst1,
            plain1.len() as u32,
            0xC001,
            Some(b"custom_mem\0"),
        );
        let req2 = if has_metal {
            let mut r = DSTORAGE_REQUEST {
                options: DSTORAGE_REQUEST_OPTIONS::new(
                    DSTORAGE_CUSTOM_COMPRESSION_0 + 1,
                    DSTORAGE_REQUEST_SOURCE_MEMORY,
                    DSTORAGE_REQUEST_DESTINATION_BUFFER,
                ),
                source: [0u8; 24],
                destination: [0u8; 40],
                uncompressed_size: plain2.len() as u32,
                _pad0: 0,
                cancellation_tag: 0xC002,
                name: b"custom_gpu_buf\0".as_ptr(),
            };
            r.source[0..8].copy_from_slice(&(encoded2.as_ptr() as u64).to_le_bytes());
            r.source[8..12].copy_from_slice(&(encoded2.len() as u32).to_le_bytes());
            r.destination[0..8].copy_from_slice(&(gpu_buf as u64).to_le_bytes());
            r.destination[8..16].copy_from_slice(&512u64.to_le_bytes());
            r.destination[16..20].copy_from_slice(&(plain2.len() as u32).to_le_bytes());
            r
        } else {
            make_mem_to_mem_request(
                DSTORAGE_CUSTOM_COMPRESSION_0 + 1,
                &encoded2,
                &mut dst2,
                plain2.len() as u32,
                0xC002,
                Some(b"custom_mem2\0"),
            )
        };

        let done_ev = crate::kernel32::threads::CreateEventA(
            std::ptr::null_mut(),
            1,
            0,
            std::ptr::null(),
        );
        assert!(!done_ev.is_null());

        queue_enqueue_request(q, &req1);
        queue_enqueue_request(q, &req2);
        queue_enqueue_status(q, status, 0);
        if has_metal {
            queue_enqueue_signal(q, fence_obj as usize, 100);
        }
        queue_enqueue_set_event(q, done_ev);

        // Submit() must return immediately and enqueue both custom requests onto the
        // custom decompression queue without signaling Status / Fence / Event yet.
        queue_submit(q);

        assert!(!status_is_complete(status, 0));
        assert_eq!(status_get_hresult(status, 0), E_PENDING);
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(done_ev, 0),
            WAIT_TIMEOUT
        );
        if has_metal {
            assert_eq!(Obj::get(fence_obj).f.completed(), 0);
        }
        let info_pending = queue_query(q);
        assert!(info_pending.empty_slot_count < DSTORAGE_MIN_QUEUE_CAPACITY);

        // Custom decompression event MUST now be signaled.
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(custom_ev, 0),
            WAIT_OBJECT_0
        );

        // Spawn a real worker thread to process GetRequests / GetRequests1 + SetRequestResults.
        let worker_q = custom_q;
        let worker_ev = custom_ev as usize;
        let worker_status = status;
        let worker = std::thread::spawn(move || {
            assert_eq!(
                crate::kernel32::threads::WaitForSingleObject(worker_ev as HANDLE, 5000),
                WAIT_OBJECT_0
            );

            // SELECT_BUILTIN must not pull custom requests or clear the event.
            let mut dummy: DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST = std::mem::zeroed();
            let mut builtin_cnt = 99u32;
            assert_eq!(
                custom_decomp_get_requests1(
                    worker_q,
                    DSTORAGE_GET_REQUEST_FLAG_SELECT_BUILTIN,
                    1,
                    &mut dummy,
                    &mut builtin_cnt
                ),
                S_OK
            );
            assert_eq!(builtin_cnt, 0);
            assert_eq!(
                crate::kernel32::threads::WaitForSingleObject(worker_ev as HANDLE, 0),
                WAIT_OBJECT_0
            );

            // Fetch first request via IDStorageCustomDecompressionQueue::GetRequests.
            let mut r0: DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST = std::mem::zeroed();
            let mut n0 = 0u32;
            assert_eq!(custom_decomp_get_requests(worker_q, 1, &mut r0, &mut n0), S_OK);
            assert_eq!(n0, 1);
            assert_eq!(r0.compression_format, DSTORAGE_CUSTOM_COMPRESSION_0);
            // Second request is still pending, so the event must remain signaled.
            assert_eq!(
                crate::kernel32::threads::WaitForSingleObject(worker_ev as HANDLE, 0),
                WAIT_OBJECT_0
            );

            // Fetch second request via IDStorageCustomDecompressionQueue1::GetRequests1(SELECT_ALL).
            let mut batch: [DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST; 4] = std::mem::zeroed();
            let mut n1 = 0u32;
            assert_eq!(
                custom_decomp_get_requests1(
                    worker_q,
                    DSTORAGE_GET_REQUEST_FLAG_SELECT_ALL,
                    4,
                    batch.as_mut_ptr(),
                    &mut n1
                ),
                S_OK
            );
            assert_eq!(n1, 1);
            let r1 = batch[0];
            assert_eq!(r1.compression_format, DSTORAGE_CUSTOM_COMPRESSION_0 + 1);
            // Queue is now drained, so event must be reset.
            assert_eq!(
                crate::kernel32::threads::WaitForSingleObject(worker_ev as HANDLE, 0),
                WAIT_TIMEOUT
            );

            // Decompress r1 (second request) FIRST to verify FIFO barrier waits for r0.
            let src1 = std::slice::from_raw_parts(r1.src_buffer as *const u8, r1.src_size as usize);
            let out1 = std::slice::from_raw_parts_mut(r1.dst_buffer as *mut u8, r1.dst_size as usize);
            for (d, &s) in out1.iter_mut().zip(src1.iter()) {
                *d = s ^ 0xA5;
            }
            assert_eq!(
                custom_decomp_set_results(
                    worker_q,
                    &[DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
                        id: r1.id,
                        result: S_OK,
                        _pad0: 0,
                    }]
                ),
                S_OK
            );
            // r0 is still pending so status[0] must NOT complete yet!
            assert!(!status_is_complete(worker_status, 0));

            // Now decompress r0 and submit its result.
            let src0 = std::slice::from_raw_parts(r0.src_buffer as *const u8, r0.src_size as usize);
            let out0 = std::slice::from_raw_parts_mut(r0.dst_buffer as *mut u8, r0.dst_size as usize);
            for (d, &s) in out0.iter_mut().zip(src0.iter()) {
                *d = s ^ 0x5A;
            }
            assert_eq!(
                custom_decomp_set_results(
                    worker_q,
                    &[DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
                        id: r0.id,
                        result: S_OK,
                        _pad0: 0,
                    }]
                ),
                S_OK
            );
        });

        // Wait for the queue's SetEvent completion signal.
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(done_ev, 5000),
            WAIT_OBJECT_0
        );
        worker.join().unwrap();

        assert!(status_is_complete(status, 0));
        assert_eq!(status_get_hresult(status, 0), S_OK);
        assert_eq!(dst1, plain1);
        if has_metal {
            assert_eq!(Obj::get(fence_obj).f.completed(), 100);
            let res_ref = crate::d3d12::resource::from_ptr(gpu_buf).unwrap();
            let landed = res_ref.r.cpu_bytes(512, plain2.len()).unwrap();
            assert_eq!(landed, plain2);
        } else {
            assert_eq!(dst2, plain2);
        }
        assert_eq!(queue_query(q).empty_slot_count, DSTORAGE_MIN_QUEUE_CAPACITY);
        assert_eq!(queue_retrieve_error_record(q).failure_count, 0);

        // ---- Failure propagation path via SetRequestResults(err_hr) ----
        let err_ev = queue_get_error_event(q);
        let fail_ev = crate::kernel32::threads::CreateEventA(
            std::ptr::null_mut(),
            1,
            0,
            std::ptr::null(),
        );
        let mut fail_dst = vec![0x99u8; 64];
        let req_fail = make_mem_to_mem_request(
            DSTORAGE_CUSTOM_COMPRESSION_0 + 2,
            &encoded1[..64],
            &mut fail_dst,
            64,
            0xC003,
            Some(b"custom_fail\0"),
        );
        queue_enqueue_request(q, &req_fail);
        queue_enqueue_status(q, status, 1);
        if has_metal {
            queue_enqueue_signal(q, fence_obj as usize, 200);
        }
        queue_enqueue_set_event(q, fail_ev);
        queue_submit(q);

        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(custom_ev, 0),
            WAIT_OBJECT_0
        );
        let mut rf: DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST = std::mem::zeroed();
        let mut nf = 0u32;
        assert_eq!(custom_decomp_get_requests(custom_q, 1, &mut rf, &mut nf), S_OK);
        assert_eq!(nf, 1);
        assert_eq!(rf.compression_format, DSTORAGE_CUSTOM_COMPRESSION_0 + 2);

        // Unknown request ID must fail honestly with E_INVALIDARG.
        assert_eq!(
            custom_decomp_set_results(
                custom_q,
                &[DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
                    id: 0xDEAD_BEEF_CAFE_0000,
                    result: S_OK,
                    _pad0: 0,
                }]
            ),
            E_INVALIDARG
        );

        // Report custom decompression failure HRESULT.
        assert_eq!(
            custom_decomp_set_results(
                custom_q,
                &[DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
                    id: rf.id,
                    result: E_DSTORAGE_DECOMPRESSION_ERROR,
                    _pad0: 0,
                }]
            ),
            S_OK
        );

        assert!(fail_dst.iter().all(|&b| b == 0x99), "failed custom decomp must not write dst");
        assert!(status_is_complete(status, 1));
        assert_eq!(status_get_hresult(status, 1), E_DSTORAGE_DECOMPRESSION_ERROR);
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(fail_ev, 0),
            WAIT_OBJECT_0
        );
        if has_metal {
            assert_eq!(Obj::get(fence_obj).f.completed(), 200);
        }
        assert_eq!(
            crate::kernel32::threads::WaitForSingleObject(err_ev, 0),
            WAIT_OBJECT_0
        );
        let rec = queue_retrieve_error_record(q);
        assert_eq!(rec.failure_count, 1);
        assert_eq!(rec.first_failure.hresult, E_DSTORAGE_DECOMPRESSION_ERROR);
        assert_eq!(rec.first_failure.command_type, DSTORAGE_COMMAND_TYPE_REQUEST);
        assert!(rec.first_failure.params.request.request_name.starts_with(b"custom_fail\0"));

        crate::kernel32::threads::CloseHandle(fail_ev);
        crate::kernel32::threads::CloseHandle(err_ev);
        crate::kernel32::threads::CloseHandle(done_ev);
        crate::kernel32::threads::CloseHandle(custom_ev);
        if has_metal {
            Obj::release(fence_obj);
            crate::d3d12::com::com_release(gpu_buf);
        }
        com_release(status);
        com_release(q);
        com_release(custom_direct);
        com_release(custom_q1);
        com_release(custom_q);
        com_release(factory);
    }
}
