//! Native `zstd.dll` / `libzstd.dll` implementation backed by the vendored
//! BSD-3-Clause Meta Zstandard library (`zstd-sys`).
//!
//! Matches upstream `zstd.h` C API semantics, frame format, and error codes
//! (`ZSTD_isError`, `ZSTD_compressBound`, `ZSTD_compress`, `ZSTD_decompress`,
//! `ZSTD_versionNumber`).

#![allow(non_snake_case)]

use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

pub extern "win64" fn ZSTD_versionNumber() -> u32 {
    unsafe { zstd_sys::ZSTD_versionNumber() }
}

pub extern "win64" fn ZSTD_isError(code: usize) -> u32 {
    unsafe { zstd_sys::ZSTD_isError(code) as u32 }
}

pub extern "win64" fn ZSTD_compressBound(src_size: usize) -> usize {
    unsafe { zstd_sys::ZSTD_compressBound(src_size) }
}

/// Standard Zstandard generic error code (`(size_t)-ZSTD_error_GENERIC`, satisfies `ZSTD_isError == 1`).
const ZSTD_ERROR_GENERIC: usize = usize::MAX;

pub extern "win64" fn ZSTD_compress(
    dst: *mut u8,
    dst_capacity: usize,
    src: *const u8,
    src_size: usize,
    level: i32,
) -> usize {
    if (dst.is_null() && dst_capacity > 0) || (src.is_null() && src_size > 0) {
        return ZSTD_ERROR_GENERIC;
    }
    unsafe {
        zstd_sys::ZSTD_compress(
            dst as *mut c_void,
            dst_capacity,
            src as *const c_void,
            src_size,
            level,
        )
    }
}

pub extern "win64" fn ZSTD_decompress(
    dst: *mut u8,
    dst_capacity: usize,
    src: *const u8,
    compressed_size: usize,
) -> usize {
    if (dst.is_null() && dst_capacity > 0) || (src.is_null() && compressed_size > 0) {
        return ZSTD_ERROR_GENERIC;
    }
    unsafe {
        zstd_sys::ZSTD_decompress(
            dst as *mut c_void,
            dst_capacity,
            src as *const c_void,
            compressed_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zstd_version_and_bound_match_libzstd() {
        let ver = ZSTD_versionNumber();
        assert!(ver >= 10500, "expected libzstd >= 1.5.0, got {ver}");
        assert_eq!(ZSTD_isError(0), 0);
        assert_eq!(ZSTD_isError(12345), 0);
        assert_eq!(ZSTD_isError(usize::MAX), 1);
        assert!(ZSTD_compressBound(0) > 0);
        assert!(ZSTD_compressBound(4096) >= 4096);
    }

    #[test]
    fn zstd_compress_decompress_round_trip() {
        let input = b"The quick brown fox jumps over the lazy dog. ".repeat(128);
        let bound = ZSTD_compressBound(input.len());
        let mut compressed = vec![0u8; bound];

        for level in [1, 3, 9] {
            let csize = ZSTD_compress(
                compressed.as_mut_ptr(),
                compressed.len(),
                input.as_ptr(),
                input.len(),
                level,
            );
            assert_eq!(ZSTD_isError(csize), 0, "compress failed at level {level}: {csize:#x}");
            assert!(csize > 0 && csize < input.len());

            let mut out = vec![0u8; input.len()];
            let dsize = ZSTD_decompress(
                out.as_mut_ptr(),
                out.len(),
                compressed.as_ptr(),
                csize,
            );
            assert_eq!(ZSTD_isError(dsize), 0, "decompress failed: {dsize:#x}");
            assert_eq!(dsize, input.len());
            assert_eq!(out, input);
        }
    }

    #[test]
    fn zstd_empty_payload_round_trip() {
        let bound = ZSTD_compressBound(0);
        let mut compressed = vec![0u8; bound];
        let csize = ZSTD_compress(compressed.as_mut_ptr(), compressed.len(), std::ptr::null(), 0, 3);
        assert_eq!(ZSTD_isError(csize), 0);
        assert!(csize > 0);

        let mut out = [0u8; 16];
        let dsize = ZSTD_decompress(out.as_mut_ptr(), out.len(), compressed.as_ptr(), csize);
        assert_eq!(ZSTD_isError(dsize), 0);
        assert_eq!(dsize, 0);
    }

    #[test]
    fn zstd_error_paths_match_upstream() {
        let input = b"hello world repeated ".repeat(64);
        let bound = ZSTD_compressBound(input.len());
        let mut compressed = vec![0u8; bound];

        // Destination too small during compression -> ZSTD error.
        let tiny_rc = ZSTD_compress(compressed.as_mut_ptr(), 4, input.as_ptr(), input.len(), 3);
        assert_eq!(ZSTD_isError(tiny_rc), 1);

        let csize = ZSTD_compress(
            compressed.as_mut_ptr(),
            compressed.len(),
            input.as_ptr(),
            input.len(),
            3,
        );
        assert_eq!(ZSTD_isError(csize), 0);

        // Destination too small during decompression -> ZSTD error.
        let mut small = vec![0u8; 8];
        let small_rc = ZSTD_decompress(small.as_mut_ptr(), small.len(), compressed.as_ptr(), csize);
        assert_eq!(ZSTD_isError(small_rc), 1);

        // Corrupt frame -> ZSTD error.
        let corrupt = [0xDEu8, 0xAD, 0xBE, 0xEF, 0x00, 0x11, 0x22, 0x33];
        let mut out = vec![0u8; 64];
        let bad_rc = ZSTD_decompress(out.as_mut_ptr(), out.len(), corrupt.as_ptr(), corrupt.len());
        assert_eq!(ZSTD_isError(bad_rc), 1);

        // Null source with non-zero length -> ZSTD error.
        let null_rc = ZSTD_decompress(out.as_mut_ptr(), out.len(), std::ptr::null(), 16);
        assert_eq!(ZSTD_isError(null_rc), 1);
    }
}
