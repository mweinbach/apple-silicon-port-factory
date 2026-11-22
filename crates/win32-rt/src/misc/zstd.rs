//! Honest stub for `zstd.dll` / `libzstd.dll`.
//!
//! Games shipping their own `zstd.dll` / `libzstd.dll` PE should execute the real PE DLL
//! under the x86-64 -> ARM64 translator rather than being overridden by this fallback stub.

#![allow(non_snake_case, unused_variables)]

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

/// Standard ZSTD generic error code (`(size_t)-1`, satisfies `ZSTD_isError(code) == 1`).
const ZSTD_ERROR_GENERIC: usize = usize::MAX;

pub extern "win64" fn ZSTD_versionNumber() -> u32 {
    10506 // 1.5.6
}

pub extern "win64" fn ZSTD_isError(code: usize) -> u32 {
    u32::from(code > (usize::MAX - 1024))
}

pub extern "win64" fn ZSTD_compressBound(src_size: usize) -> usize {
    src_size + (src_size >> 8) + 64
}

pub extern "win64" fn ZSTD_compress(
    _dst: *mut u8,
    _dst_capacity: usize,
    _src: *const u8,
    src_size: usize,
    _level: i32,
) -> usize {
    crate::stub!("zstd!ZSTD_compress(src_size={src_size})");
    ZSTD_ERROR_GENERIC
}

pub extern "win64" fn ZSTD_decompress(
    _dst: *mut u8,
    _dst_capacity: usize,
    _src: *const u8,
    compressed_size: usize,
) -> usize {
    crate::stub!("zstd!ZSTD_decompress(compressed_size={compressed_size})");
    ZSTD_ERROR_GENERIC
}
