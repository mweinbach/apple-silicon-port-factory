//! Microsoft GDeflate v1 codec (`TileStreamHeader` + `tileOffsets[]` + 64 KiB
//! raw-DEFLATE tiles via [`crate::misc::zlib1::ZlibStream`]) and the
//! `IDStorageCompressionCodec` COM object.

use std::ffi::c_void;
use std::sync::OnceLock;

use super::abi::*;
use crate::com::{com_add_ref, com_release, qi_helper, register_vtable_once, ComObj, Guid};
use crate::misc::zlib1::ZlibStream;
use crate::strings::{le_u16, le_u32, wr};

pub(super) const GDEFLATE_ID: u8 = 0x04;
pub(super) const GDEFLATE_MAGIC: u8 = GDEFLATE_ID ^ 0xFF; // 0xFB
pub(super) const GDEFLATE_TILE_SIZE_IDX_64K: u32 = 1;
pub const GDEFLATE_TILE_SIZE: usize = 65536;
pub(super) const GDEFLATE_HEADER_SIZE: usize = 8;

const RAW_DEFLATE_WINDOW_BITS: i32 = -15;

fn raw_deflate(src: &[u8], level: i32) -> Result<Vec<u8>, i32> {
    if u32::try_from(src.len()).is_err() {
        return Err(E_INVALIDARG);
    }
    ZlibStream::deflate_all(src, level, RAW_DEFLATE_WINDOW_BITS).map_err(|_| E_FAIL)
}

fn raw_inflate(src: &[u8], dst: &mut [u8]) -> Result<usize, i32> {
    if src.is_empty() {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    let produced = ZlibStream::inflate_into(src, dst, RAW_DEFLATE_WINDOW_BITS)
        .map_err(|_| E_DSTORAGE_DECOMPRESSION_ERROR)?;
    if produced != dst.len() {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    Ok(produced)
}

struct GDeflateHeader {
    num_tiles: usize,
    last_tile_uncomp: usize,
    total_uncomp: usize,
    data_start: usize,
}

fn parse_gdeflate_header(src: &[u8]) -> Result<GDeflateHeader, i32> {
    if src.len() < GDEFLATE_HEADER_SIZE {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    let id = src[0];
    let magic = src[1];
    if id != GDEFLATE_ID || magic != GDEFLATE_MAGIC {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    let num_tiles = le_u16(src, 2).ok_or(E_DSTORAGE_DECOMPRESSION_ERROR)? as usize;
    let bitfield = le_u32(src, 4).ok_or(E_DSTORAGE_DECOMPRESSION_ERROR)?;
    let tile_size_idx = bitfield & 0x3;
    let last_tile_size_field = ((bitfield >> 2) & 0x3_FFFF) as usize;
    let reserved = bitfield >> 20;

    if reserved != 0 || tile_size_idx != GDEFLATE_TILE_SIZE_IDX_64K {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    if last_tile_size_field >= GDEFLATE_TILE_SIZE {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    if num_tiles == 0 {
        if last_tile_size_field != 0 {
            return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
        }
        return Ok(GDeflateHeader {
            num_tiles: 0,
            last_tile_uncomp: 0,
            total_uncomp: 0,
            data_start: GDEFLATE_HEADER_SIZE,
        });
    }
    let table_bytes = num_tiles
        .checked_mul(4)
        .ok_or(E_DSTORAGE_DECOMPRESSION_ERROR)?;
    let data_start = GDEFLATE_HEADER_SIZE
        .checked_add(table_bytes)
        .ok_or(E_DSTORAGE_DECOMPRESSION_ERROR)?;
    if src.len() < data_start {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    let last_tile_uncomp = if last_tile_size_field == 0 {
        GDEFLATE_TILE_SIZE
    } else {
        last_tile_size_field
    };
    let total_uncomp = (num_tiles - 1)
        .checked_mul(GDEFLATE_TILE_SIZE)
        .and_then(|v| v.checked_add(last_tile_uncomp))
        .ok_or(E_DSTORAGE_DECOMPRESSION_ERROR)?;
    Ok(GDeflateHeader {
        num_tiles,
        last_tile_uncomp,
        total_uncomp,
        data_start,
    })
}

/// Conservative upper bound on the compressed byte length of a GDeflate v1 stream.
pub fn compress_gdeflate_bound(uncompressed_size: usize) -> usize {
    if uncompressed_size == 0 {
        return GDEFLATE_HEADER_SIZE;
    }
    let num_tiles = uncompressed_size.div_ceil(GDEFLATE_TILE_SIZE);
    GDEFLATE_HEADER_SIZE
        .saturating_add(num_tiles.saturating_mul(4 + 64))
        .saturating_add(uncompressed_size)
}

/// Compress `src` into a Microsoft GDeflate v1 `TileStream` using 64 KiB raw DEFLATE tiles.
pub fn compress_gdeflate(src: &[u8], compression_setting: i32) -> Result<Vec<u8>, i32> {
    let level = match compression_setting {
        DSTORAGE_COMPRESSION_FASTEST => 1,
        DSTORAGE_COMPRESSION_BEST_RATIO => 9,
        _ => 6,
    };
    if src.is_empty() {
        let mut out = Vec::with_capacity(GDEFLATE_HEADER_SIZE);
        out.push(GDEFLATE_ID);
        out.push(GDEFLATE_MAGIC);
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&GDEFLATE_TILE_SIZE_IDX_64K.to_le_bytes());
        return Ok(out);
    }
    let num_tiles = src.len().div_ceil(GDEFLATE_TILE_SIZE);
    let num_tiles_u16 = u16::try_from(num_tiles).map_err(|_| E_INVALIDARG)?;
    let mut tile_offsets = vec![0u32; num_tiles];
    let mut tile_data = Vec::with_capacity(src.len().min(GDEFLATE_TILE_SIZE * 4));
    for (i, chunk) in src.chunks(GDEFLATE_TILE_SIZE).enumerate() {
        let rel_off = u32::try_from(tile_data.len()).map_err(|_| E_INVALIDARG)?;
        let comp = raw_deflate(chunk, level)?;
        if i > 0 {
            tile_offsets[i] = rel_off;
        }
        if i + 1 == num_tiles {
            tile_offsets[0] = u32::try_from(comp.len()).map_err(|_| E_INVALIDARG)?;
        }
        tile_data.extend_from_slice(&comp);
    }
    let last_len = src.len() - (num_tiles - 1) * GDEFLATE_TILE_SIZE;
    let last_tile_field = if last_len == GDEFLATE_TILE_SIZE {
        0u32
    } else {
        last_len as u32
    };
    let bitfield = (GDEFLATE_TILE_SIZE_IDX_64K & 0x3) | ((last_tile_field & 0x3_FFFF) << 2);

    let mut out = Vec::with_capacity(GDEFLATE_HEADER_SIZE + num_tiles * 4 + tile_data.len());
    out.push(GDEFLATE_ID);
    out.push(GDEFLATE_MAGIC);
    out.extend_from_slice(&num_tiles_u16.to_le_bytes());
    out.extend_from_slice(&bitfield.to_le_bytes());
    for off in tile_offsets {
        out.extend_from_slice(&off.to_le_bytes());
    }
    out.extend_from_slice(&tile_data);
    Ok(out)
}

/// Decompress a Microsoft GDeflate v1 `TileStream` into a newly allocated `Vec<u8>`.
/// Never falls back to `memcpy` on corrupt streams.
pub fn decompress_gdeflate_to_vec(src: &[u8]) -> Result<Vec<u8>, i32> {
    let hdr = parse_gdeflate_header(src)?;
    if hdr.num_tiles == 0 {
        return Ok(Vec::new());
    }
    let read_tile_entry = |idx: usize| -> Result<u32, i32> {
        let off = GDEFLATE_HEADER_SIZE + idx * 4;
        le_u32(src, off).ok_or(E_DSTORAGE_DECOMPRESSION_ERROR)
    };
    let last_tile_comp_size = read_tile_entry(0)? as usize;
    let mut out = vec![0u8; hdr.total_uncomp];
    for i in 0..hdr.num_tiles {
        let rel_start = if i == 0 {
            0usize
        } else {
            read_tile_entry(i)? as usize
        };
        let rel_end = if i + 1 < hdr.num_tiles {
            read_tile_entry(i + 1)? as usize
        } else {
            rel_start
                .checked_add(last_tile_comp_size)
                .ok_or(E_DSTORAGE_DECOMPRESSION_ERROR)?
        };
        if rel_start >= rel_end {
            return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
        }
        let abs_start = hdr
            .data_start
            .checked_add(rel_start)
            .ok_or(E_DSTORAGE_DECOMPRESSION_ERROR)?;
        let abs_end = hdr
            .data_start
            .checked_add(rel_end)
            .ok_or(E_DSTORAGE_DECOMPRESSION_ERROR)?;
        if abs_end > src.len() {
            return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
        }
        let tile_src = &src[abs_start..abs_end];
        let tile_uncomp_len = if i + 1 < hdr.num_tiles {
            GDEFLATE_TILE_SIZE
        } else {
            hdr.last_tile_uncomp
        };
        let out_off = i * GDEFLATE_TILE_SIZE;
        let tile_dst = &mut out[out_off..out_off + tile_uncomp_len];
        let written = raw_inflate(tile_src, tile_dst)?;
        if written != tile_uncomp_len {
            return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
        }
    }
    Ok(out)
}

/// Decompress a Microsoft GDeflate v1 `TileStream` into `dst` atomically.
/// On any failure, `dst` is left untouched and an error `HRESULT` is returned.
pub fn decompress_gdeflate(src: &[u8], dst: &mut [u8]) -> Result<usize, i32> {
    let hdr = parse_gdeflate_header(src)?;
    if hdr.total_uncomp > dst.len() {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    let decompressed = decompress_gdeflate_to_vec(src)?;
    dst[..decompressed.len()].copy_from_slice(&decompressed);
    Ok(decompressed.len())
}

// ============================================================================
// IDStorageCompressionCodec (IUnknown + CompressBuffer + DecompressBuffer + CompressBufferBound)
// ============================================================================

struct CompressionCodecObj;

unsafe extern "win64" fn ds_codec_qi(
    this: *mut ComObj<CompressionCodecObj>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    qi_helper(this, riid, ppv, CODEC_IIDS)
}

unsafe extern "win64" fn ds_codec_compress_buffer(
    _this: *mut c_void,
    uncompressed: *const u8,
    uncompressed_size: usize,
    setting: i32,
    compressed: *mut u8,
    compressed_buf_size: usize,
    compressed_size_out: *mut usize,
) -> i32 {
    if compressed_size_out.is_null() {
        return E_INVALIDARG;
    }
    wr(compressed_size_out, 0);
    if (uncompressed_size > 0 && uncompressed.is_null())
        || (compressed_buf_size > 0 && compressed.is_null())
    {
        return E_INVALIDARG;
    }
    let src = if uncompressed_size == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(uncompressed, uncompressed_size)
    };
    let out = match compress_gdeflate(src, setting) {
        Ok(v) => v,
        Err(hr) => return hr,
    };
    if out.len() > compressed_buf_size {
        return E_INVALIDARG;
    }
    if !out.is_empty() {
        std::ptr::copy_nonoverlapping(out.as_ptr(), compressed, out.len());
    }
    wr(compressed_size_out, out.len());
    S_OK
}

unsafe extern "win64" fn ds_codec_decompress_buffer(
    _this: *mut c_void,
    compressed: *const u8,
    compressed_size: usize,
    uncompressed: *mut u8,
    uncompressed_buf_size: usize,
    uncompressed_size_out: *mut usize,
) -> i32 {
    if uncompressed_size_out.is_null() {
        return E_INVALIDARG;
    }
    wr(uncompressed_size_out, 0);
    if (compressed_size > 0 && compressed.is_null())
        || (uncompressed_buf_size > 0 && uncompressed.is_null())
    {
        return E_INVALIDARG;
    }
    let src = if compressed_size == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(compressed, compressed_size)
    };
    let dst = if uncompressed_buf_size == 0 {
        &mut [][..]
    } else {
        std::slice::from_raw_parts_mut(uncompressed, uncompressed_buf_size)
    };
    match decompress_gdeflate(src, dst) {
        Ok(written) => {
            wr(uncompressed_size_out, written);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds_codec_compress_buffer_bound(
    _this: *mut c_void,
    uncompressed_size: usize,
) -> usize {
    compress_gdeflate_bound(uncompressed_size)
}

fn codec_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 6]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            ds_codec_qi as *const () as usize,
            com_add_ref::<CompressionCodecObj> as *const () as usize,
            com_release::<CompressionCodecObj> as *const () as usize,
            ds_codec_compress_buffer as *const () as usize,
            ds_codec_decompress_buffer as *const () as usize,
            ds_codec_compress_buffer_bound as *const () as usize,
        ]
    })
}

pub(super) fn create_compression_codec() -> usize {
    ComObj::new(codec_vtable(), CompressionCodecObj) as usize
}
