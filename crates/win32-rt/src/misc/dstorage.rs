//! Native `dstorage.dll` / `dstoragecore.dll` (Microsoft DirectStorage 1.0 - 1.2)
//! implementation for Apple Silicon SSD + Metal 4 Unified Memory.
//!
//! Implements:
//! - Genuine Microsoft GDeflate v1 (`TileStreamHeader` + `tileOffsets[]` + 64 KiB
//!   raw-DEFLATE tiles via `libz.dylib`, `windowBits = -15`) compression and
//!   decompression (`IDStorageCompressionCodec` and queue request decompression).
//! - Real request queueing, `CancelRequestsWithTag(mask, value)`, `Close()`,
//!   `Query(DSTORAGE_QUEUE_INFO)`, and FIFO execution on `Submit()` (plus
//!   auto-submit when the pending queue reaches threshold).
//! - Both `DSTORAGE_REQUEST_SOURCE_FILE` (`pread` via `FileExt::read_exact_at`)
//!   and `DSTORAGE_REQUEST_SOURCE_MEMORY` sources.
//! - Destinations: `MEMORY`, D3D12 `BUFFER`, `TEXTURE_REGION`,
//!   `MULTIPLE_SUBRESOURCES`, `MULTIPLE_SUBRESOURCES_RANGE`, and `TILES`
//!   (64 KiB standard tile geometry translated to `CopyTextureRegion` boxes)
//!   through `crate::d3d12` / `d3d12_metal` (zero `objc2-metal` imports).
//! - Real `IDStorageCustomDecompressionQueue` / `IDStorageCustomDecompressionQueue1`
//!   COM interfaces (via `IDStorageFactory::QueryInterface`) for custom
//!   compression formats (`>= 0x80`) with async worker event signaling,
//!   `GetRequests` / `GetRequests1`, and `SetRequestResults` completion & error
//!   propagation.
//! - Real synchronization: `IDStorageStatusArray` (`IsComplete` / `GetHResult`),
//!   `ID3D12Fence` signaling via `crate::d3d12::fence`, Win32 event signaling via
//!   `crate::kernel32::threads::SetEvent`, and real `DSTORAGE_ERROR_RECORD` +
//!   `GetErrorEvent` diagnostics.

use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::fs::File;
use std::io::ErrorKind;
use std::os::unix::fs::{FileExt, MetadataExt};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::layout::{copyable_footprints, plane_format, PlacedFootprint};
use d3d12_metal::metal::cmdlist::{
    split_subresource, CommandAllocator as MtlCommandAllocator, CommandList as MtlCommandList,
    CopyLoc,
};
use d3d12_metal::metal::queue::{ListType, Queue as MtlQueue};
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::types::{dim, heap_type, HeapProperties, ResourceDesc};

use crate::d3d12::com::{guid, Guid, Obj, IID_IUnknown};
use crate::handles::Event;
use crate::misc::zlib1::{
    host_deflate, host_deflateEnd, host_deflateInit2_, host_inflate, host_inflateEnd,
    host_inflateInit2_, host_zlibVersion, MacZStream,
};
use crate::types::{errno_to_win32, unix_to_filetime, FALSE, HANDLE};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ============================================================================
// DirectStorage HRESULTs & Constants (dstorage.h / dstorageerr.h)
// ============================================================================

const S_OK: i32 = 0;
const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;
const E_NOINTERFACE: i32 = 0x8000_4002u32 as i32;
const E_POINTER: i32 = 0x8000_4003u32 as i32;
const E_FAIL: i32 = 0x8000_4005u32 as i32;
const E_PENDING: i32 = 0x8000_000Au32 as i32;
const E_HANDLE: i32 = 0x8007_0006u32 as i32;
const E_OUTOFMEMORY: i32 = 0x8007_000Eu32 as i32;
const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;

pub const E_DSTORAGE_INVALID_QUEUE_CAPACITY: i32 = 0x8924_0003u32 as i32;
pub const E_DSTORAGE_END_OF_FILE: i32 = 0x8924_0007u32 as i32;
pub const E_DSTORAGE_DECOMPRESSION_ERROR: i32 = 0x8924_0030u32 as i32;

pub const DSTORAGE_MIN_QUEUE_CAPACITY: u16 = 0x80;
pub const DSTORAGE_MAX_QUEUE_CAPACITY: u16 = 0x2000;
pub const DSTORAGE_MAX_PATH: usize = 260;
pub const DSTORAGE_REQUEST_MAX_NAME: usize = 64;

pub const DSTORAGE_REQUEST_SOURCE_FILE: u32 = 0;
pub const DSTORAGE_REQUEST_SOURCE_MEMORY: u32 = 1;

pub const DSTORAGE_REQUEST_DESTINATION_MEMORY: u32 = 0;
pub const DSTORAGE_REQUEST_DESTINATION_BUFFER: u32 = 1;
pub const DSTORAGE_REQUEST_DESTINATION_TEXTURE_REGION: u32 = 2;
pub const DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES: u32 = 3;
pub const DSTORAGE_REQUEST_DESTINATION_TILES: u32 = 4;
pub const DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES_RANGE: u32 = 5;

pub const DSTORAGE_COMPRESSION_FORMAT_NONE: u8 = 0;
pub const DSTORAGE_COMPRESSION_FORMAT_GDEFLATE: u8 = 1;
pub const DSTORAGE_CUSTOM_COMPRESSION_0: u8 = 0x80;

pub const DSTORAGE_COMPRESSION_FASTEST: i32 = -1;
pub const DSTORAGE_COMPRESSION_DEFAULT: i32 = 0;
pub const DSTORAGE_COMPRESSION_BEST_RATIO: i32 = 1;

pub const DSTORAGE_COMPRESSION_SUPPORT_NONE: u32 = 0x0;
pub const DSTORAGE_COMPRESSION_SUPPORT_CPU_FALLBACK: u32 = 0x4;

pub const DSTORAGE_COMMAND_TYPE_NONE: i32 = -1;
pub const DSTORAGE_COMMAND_TYPE_REQUEST: i32 = 0;
pub const DSTORAGE_COMMAND_TYPE_STATUS: i32 = 1;
pub const DSTORAGE_COMMAND_TYPE_SIGNAL: i32 = 2;
pub const DSTORAGE_COMMAND_TYPE_EVENT: i32 = 3;

pub const DSTORAGE_CUSTOM_DECOMPRESSION_FLAG_NONE: u32 = 0x0;
pub const DSTORAGE_CUSTOM_DECOMPRESSION_FLAG_DEST_IN_UPLOAD_HEAP: u32 = 0x1;

pub const DSTORAGE_GET_REQUEST_FLAG_SELECT_CUSTOM: u32 = 0x1;
pub const DSTORAGE_GET_REQUEST_FLAG_SELECT_BUILTIN: u32 = 0x2;
pub const DSTORAGE_GET_REQUEST_FLAG_SELECT_ALL: u32 =
    DSTORAGE_GET_REQUEST_FLAG_SELECT_CUSTOM | DSTORAGE_GET_REQUEST_FLAG_SELECT_BUILTIN;

const DSTORAGE_DEBUG_SHOW_ERRORS: u32 = 0x1;
const D3D12_TILE_SIZE_BYTES: usize = 65536;

// Official DirectStorage 1.0 - 1.2 COM Interface IDs
pub const IID_IDSTORAGE_FACTORY: Guid = guid("6924ea0c-c3cd-4826-b10a-f64f4ed927c1");
pub const IID_IDSTORAGE_FILE: Guid = guid("5de95e7b-955a-4868-a73c-243b29f4b8da");
pub const IID_IDSTORAGE_STATUS_ARRAY: Guid = guid("82397587-7cd5-453b-a02e-31379bd64656");
pub const IID_IDSTORAGE_QUEUE: Guid = guid("cfdbd83f-9e06-4fda-8ea5-69042137f49b");
pub const IID_IDSTORAGE_QUEUE1: Guid = guid("dd2f482c-5eff-41e8-9c9e-d2374b278128");
pub const IID_IDSTORAGE_QUEUE2: Guid = guid("b1c9d643-3a49-44a2-b46f-653649470d18");
pub const IID_IDSTORAGE_COMPRESSION_CODEC: Guid = guid("84ef5121-9b43-4d03-b5c1-cc34606b262d");
pub const IID_IDSTORAGE_CUSTOM_DECOMPRESSION_QUEUE: Guid =
    guid("97179b2f-2c21-49ca-8291-4e1bf4a160df");
pub const IID_IDSTORAGE_CUSTOM_DECOMPRESSION_QUEUE1: Guid =
    guid("0d47c6c9-e61a-4706-93b4-68bfe3f4aa4a");

const FILE_IIDS: &[Guid] = &[IID_IDSTORAGE_FILE];
const STATUS_ARRAY_IIDS: &[Guid] = &[IID_IDSTORAGE_STATUS_ARRAY];
const QUEUE_IIDS: &[Guid] = &[IID_IDSTORAGE_QUEUE, IID_IDSTORAGE_QUEUE1, IID_IDSTORAGE_QUEUE2];
const CODEC_IIDS: &[Guid] = &[IID_IDSTORAGE_COMPRESSION_CODEC];
const CUSTOM_DECOMP_IIDS: &[Guid] = &[
    IID_IDSTORAGE_CUSTOM_DECOMPRESSION_QUEUE,
    IID_IDSTORAGE_CUSTOM_DECOMPRESSION_QUEUE1,
];

#[inline]
fn hresult_from_win32(win32_err: u32) -> i32 {
    if win32_err == 0 {
        S_OK
    } else {
        (0x8007_0000u32 | (win32_err & 0xFFFF)) as i32
    }
}

fn io_error_to_hresult(err: &std::io::Error) -> i32 {
    if err.kind() == ErrorKind::UnexpectedEof {
        return E_DSTORAGE_END_OF_FILE;
    }
    if let Some(code) = err.raw_os_error() {
        return hresult_from_win32(errno_to_win32(code));
    }
    match err.kind() {
        ErrorKind::NotFound => hresult_from_win32(crate::types::err::ERROR_FILE_NOT_FOUND),
        ErrorKind::PermissionDenied => hresult_from_win32(crate::types::err::ERROR_ACCESS_DENIED),
        _ => E_FAIL,
    }
}

// ============================================================================
// DirectStorage Windows x64 ABI Structures
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_QUEUE_DESC {
    pub source_type: u64,
    pub capacity: u16,
    pub priority: i8,
    pub _pad0: [u8; 5],
    pub name: *const u8,
    pub device: usize,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_QUEUE_DESC>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_QUEUE_INFO {
    pub desc: DSTORAGE_QUEUE_DESC,
    pub empty_slot_count: u16,
    pub request_count_until_auto_submit: u16,
    pub _pad0: u32,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_QUEUE_INFO>() == 40);

/// Exact MSVC x64 layout of `DSTORAGE_REQUEST_OPTIONS` (16 bytes):
/// - byte 0: `CompressionFormat : 8` (`UINT8`)
/// - bytes 1..8: `Reserved1[7]`
/// - bytes 8..16: `SourceType : 1 | DestinationType : 7 | Reserved : 48` (`UINT64`)
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DSTORAGE_REQUEST_OPTIONS {
    pub compression_format: u8,
    pub reserved1: [u8; 7],
    pub source_dest_flags: u64,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_REQUEST_OPTIONS>() == 16);

impl DSTORAGE_REQUEST_OPTIONS {
    pub const fn new(compression_format: u8, source_type: u32, destination_type: u32) -> Self {
        Self {
            compression_format,
            reserved1: [0; 7],
            source_dest_flags: ((source_type as u64) & 0x1)
                | (((destination_type as u64) & 0x7F) << 1),
        }
    }

    #[inline]
    pub const fn source_type(&self) -> u32 {
        (self.source_dest_flags & 0x1) as u32
    }

    #[inline]
    pub const fn destination_type(&self) -> u32 {
        ((self.source_dest_flags >> 1) & 0x7F) as u32
    }
}

/// Exact MSVC x64 layout of `DSTORAGE_REQUEST` (104 bytes = `0x68`).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_REQUEST {
    pub options: DSTORAGE_REQUEST_OPTIONS, // +0x00 (16 bytes)
    pub source: [u8; 24],                  // +0x10 (24 bytes: union DSTORAGE_SOURCE)
    pub destination: [u8; 40],             // +0x28 (40 bytes: union DSTORAGE_DESTINATION)
    pub uncompressed_size: u32,            // +0x50 (4 bytes)
    pub _pad0: u32,                        // +0x54 (4 bytes)
    pub cancellation_tag: u64,             // +0x58 (8 bytes)
    pub name: *const u8,                   // +0x60 (8 bytes)
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_REQUEST>() == 104);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_ERROR_PARAMETERS_REQUEST {
    pub filename: [u16; DSTORAGE_MAX_PATH],
    pub request_name: [u8; DSTORAGE_REQUEST_MAX_NAME],
    pub request: DSTORAGE_REQUEST,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_ERROR_PARAMETERS_REQUEST>() == 688);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_ERROR_PARAMETERS_STATUS {
    pub status_array: usize,
    pub index: u32,
    pub _pad0: u32,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_ERROR_PARAMETERS_STATUS>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_ERROR_PARAMETERS_SIGNAL {
    pub fence: usize,
    pub value: u64,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_ERROR_PARAMETERS_SIGNAL>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_ERROR_PARAMETERS_EVENT {
    pub handle: HANDLE,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_ERROR_PARAMETERS_EVENT>() == 8);

#[repr(C)]
#[derive(Clone, Copy)]
pub union DSTORAGE_ERROR_PARAMETERS_UNION {
    pub request: DSTORAGE_ERROR_PARAMETERS_REQUEST,
    pub status: DSTORAGE_ERROR_PARAMETERS_STATUS,
    pub signal: DSTORAGE_ERROR_PARAMETERS_SIGNAL,
    pub event: DSTORAGE_ERROR_PARAMETERS_EVENT,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_ERROR_PARAMETERS_UNION>() == 688);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_ERROR_FIRST_FAILURE {
    pub hresult: i32,
    pub command_type: i32,
    pub params: DSTORAGE_ERROR_PARAMETERS_UNION,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_ERROR_FIRST_FAILURE>() == 696);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_ERROR_RECORD {
    pub failure_count: u32,
    pub _pad0: u32,
    pub first_failure: DSTORAGE_ERROR_FIRST_FAILURE,
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_ERROR_RECORD>() == 704);

/// Exact MSVC x64 layout of `DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST` (48 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
    pub id: u64,                   // +0x00
    pub compression_format: u8,    // +0x08
    pub reserved: [u8; 3],         // +0x09
    pub flags: u32,                // +0x0C
    pub src_size: u64,             // +0x10
    pub src_buffer: *const c_void, // +0x18
    pub dst_size: u64,             // +0x20
    pub dst_buffer: *mut c_void,   // +0x28
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST>() == 48);

/// Exact MSVC x64 layout of `DSTORAGE_CUSTOM_DECOMPRESSION_RESULT` (16 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DSTORAGE_CUSTOM_DECOMPRESSION_RESULT {
    pub id: u64,     // +0x00
    pub result: i32, // +0x08
    pub _pad0: u32,  // +0x0C
}
const _: () = assert!(std::mem::size_of::<DSTORAGE_CUSTOM_DECOMPRESSION_RESULT>() == 16);

#[derive(Clone, Copy, Debug)]
struct D3D12Box {
    left: u32,
    top: u32,
    front: u32,
    right: u32,
    bottom: u32,
    back: u32,
}

#[derive(Clone, Copy, Debug)]
struct TileCoord {
    x: u32,
    y: u32,
    z: u32,
    subresource: u32,
}

#[derive(Clone, Copy, Debug)]
struct TileRegionSize {
    num_tiles: u32,
    use_box: bool,
    width: u32,
    height: u16,
    depth: u16,
}

#[inline]
fn le_u16(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([buf[off], buf[off + 1]])
}

#[inline]
fn le_u32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}

#[inline]
fn le_u64(buf: &[u8], off: usize) -> u64 {
    u64::from_le_bytes([
        buf[off],
        buf[off + 1],
        buf[off + 2],
        buf[off + 3],
        buf[off + 4],
        buf[off + 5],
        buf[off + 6],
        buf[off + 7],
    ])
}

#[inline]
fn le_usize(buf: &[u8], off: usize) -> usize {
    le_u64(buf, off) as usize
}

// ============================================================================
// Microsoft GDeflate v1 Codec (libz.dylib raw DEFLATE, windowBits = -15)
// ============================================================================

const GDEFLATE_ID: u8 = 0x04;
const GDEFLATE_MAGIC: u8 = GDEFLATE_ID ^ 0xFF; // 0xFB
const GDEFLATE_TILE_SIZE_IDX_64K: u32 = 1;
pub const GDEFLATE_TILE_SIZE: usize = 65536;
const GDEFLATE_HEADER_SIZE: usize = 8;

const Z_OK: i32 = 0;
const Z_STREAM_END: i32 = 1;
const Z_FINISH: i32 = 4;
const Z_DEFLATED: i32 = 8;
const Z_DEFAULT_STRATEGY: i32 = 0;
const RAW_DEFLATE_WINDOW_BITS: i32 = -15;
const RAW_DEFLATE_MEM_LEVEL: i32 = 8;

struct DeflateStream(MacZStream);

impl DeflateStream {
    fn new(level: i32) -> Result<Self, i32> {
        let mut strm: MacZStream = unsafe { std::mem::zeroed() };
        let rc = unsafe {
            host_deflateInit2_(
                &mut strm,
                level,
                Z_DEFLATED,
                RAW_DEFLATE_WINDOW_BITS,
                RAW_DEFLATE_MEM_LEVEL,
                Z_DEFAULT_STRATEGY,
                host_zlibVersion(),
                std::mem::size_of::<MacZStream>() as i32,
            )
        };
        if rc != Z_OK {
            return Err(E_FAIL);
        }
        Ok(Self(strm))
    }
}

impl Drop for DeflateStream {
    fn drop(&mut self) {
        unsafe {
            let _ = host_deflateEnd(&mut self.0);
        }
    }
}

struct InflateStream(MacZStream);

impl InflateStream {
    fn new() -> Result<Self, i32> {
        let mut strm: MacZStream = unsafe { std::mem::zeroed() };
        let rc = unsafe {
            host_inflateInit2_(
                &mut strm,
                RAW_DEFLATE_WINDOW_BITS,
                host_zlibVersion(),
                std::mem::size_of::<MacZStream>() as i32,
            )
        };
        if rc != Z_OK {
            return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
        }
        Ok(Self(strm))
    }
}

impl Drop for InflateStream {
    fn drop(&mut self) {
        unsafe {
            let _ = host_inflateEnd(&mut self.0);
        }
    }
}

fn raw_deflate(src: &[u8], level: i32) -> Result<Vec<u8>, i32> {
    let mut ds = DeflateStream::new(level)?;
    let cap = src.len().saturating_add(src.len() >> 3).saturating_add(64);
    let mut out = vec![0u8; cap];
    ds.0.next_in = src.as_ptr();
    ds.0.avail_in = u32::try_from(src.len()).map_err(|_| E_INVALIDARG)?;
    loop {
        let produced = ds.0.total_out;
        if produced == out.len() {
            let new_len = out.len().saturating_mul(2).max(64);
            out.resize(new_len, 0);
        }
        let rem = out.len() - produced;
        ds.0.next_out = unsafe { out.as_mut_ptr().add(produced) };
        ds.0.avail_out = rem.min(u32::MAX as usize) as u32;
        let rc = unsafe { host_deflate(&mut ds.0, Z_FINISH) };
        match rc {
            Z_STREAM_END => {
                let final_len = ds.0.total_out;
                out.truncate(final_len);
                return Ok(out);
            }
            Z_OK => {
                if ds.0.avail_out > 0 {
                    return Err(E_FAIL);
                }
            }
            _ => return Err(E_FAIL),
        }
    }
}

fn raw_inflate(src: &[u8], dst: &mut [u8]) -> Result<usize, i32> {
    if src.is_empty() {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    let mut is = InflateStream::new()?;
    is.0.next_in = src.as_ptr();
    is.0.avail_in = u32::try_from(src.len()).map_err(|_| E_DSTORAGE_DECOMPRESSION_ERROR)?;
    is.0.next_out = dst.as_mut_ptr();
    is.0.avail_out = u32::try_from(dst.len()).map_err(|_| E_DSTORAGE_DECOMPRESSION_ERROR)?;
    let rc = unsafe { host_inflate(&mut is.0, Z_FINISH) };
    if rc != Z_STREAM_END {
        return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
    }
    let produced = is.0.total_out;
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
    let num_tiles = u16::from_le_bytes([src[2], src[3]]) as usize;
    let bitfield = u32::from_le_bytes([src[4], src[5], src[6], src[7]]);
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
    let read_tile_entry = |idx: usize| -> u32 {
        let off = GDEFLATE_HEADER_SIZE + idx * 4;
        le_u32(src, off)
    };
    let last_tile_comp_size = read_tile_entry(0) as usize;
    let mut out = vec![0u8; hdr.total_uncomp];
    for i in 0..hdr.num_tiles {
        let rel_start = if i == 0 {
            0usize
        } else {
            read_tile_entry(i) as usize
        };
        let rel_end = if i + 1 < hdr.num_tiles {
            read_tile_entry(i + 1) as usize
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
// COM Object Infrastructure (Refcounted, VA_DISPATCH-registered, Unaligned-Safe)
// ============================================================================

#[repr(C)]
struct ComObj<T> {
    _vtable: *const usize,
    refs: AtomicU32,
    inner: T,
}

fn register_vtable<const N: usize>(slots: [usize; N]) -> [usize; N] {
    for &addr in &slots {
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
    slots
}

unsafe fn iid_allowed(riid: usize, allowed: &[Guid], allow_null: bool) -> Result<bool, i32> {
    if riid == 0 {
        return if allow_null { Ok(true) } else { Err(E_POINTER) };
    }
    let iid = std::ptr::read_unaligned(riid as *const Guid);
    Ok(iid == IID_IUnknown || allowed.contains(&iid))
}

unsafe fn com_from_iface<'a, T>(ptr: usize, expected_vtable: *const usize) -> Option<&'a ComObj<T>> {
    if ptr == 0 {
        return None;
    }
    let vt = std::ptr::read_unaligned(ptr as *const *const usize);
    if vt == expected_vtable {
        Some(&*(ptr as *const ComObj<T>))
    } else {
        None
    }
}

unsafe fn qi_impl<T>(this: usize, riid: usize, ppv: *mut usize, allowed: &[Guid]) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, 0);
    if this == 0 {
        return E_INVALIDARG;
    }
    match iid_allowed(riid, allowed, false) {
        Ok(true) => {
            let obj = &*(this as *const ComObj<T>);
            obj.refs.fetch_add(1, Ordering::AcqRel);
            std::ptr::write_unaligned(ppv, this);
            S_OK
        }
        Ok(false) => E_NOINTERFACE,
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn com_add_ref_typed<T>(this: usize) -> u32 {
    if this == 0 {
        return 0;
    }
    let obj = &*(this as *const ComObj<T>);
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn com_release_typed<T>(this: usize) -> u32 {
    if this == 0 {
        return 0;
    }
    let obj = &*(this as *const ComObj<T>);
    let prev = obj.refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(this as *mut ComObj<T>));
    }
    next
}

// ============================================================================
// Guest String Helpers (Unaligned-Safe)
// ============================================================================

unsafe fn read_wide_z(ptr: *const u16, max_len: usize) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let mut units = Vec::new();
    for i in 0..max_len {
        let ch = std::ptr::read_unaligned(ptr.add(i));
        if ch == 0 {
            break;
        }
        units.push(ch);
    }
    String::from_utf16_lossy(&units)
}

unsafe fn read_cstr_bytes(ptr: *const u8, max_len: usize) -> Vec<u8> {
    if ptr.is_null() {
        return Vec::new();
    }
    let mut bytes = Vec::new();
    for i in 0..max_len {
        let b = std::ptr::read_unaligned(ptr.add(i));
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    bytes
}

// ============================================================================
// IDStorageFile (IUnknown + Close + GetFileInformation)
// ============================================================================

struct DStorageFileState {
    guest_path: String,
    file: Mutex<Option<File>>,
}

struct FileObj {
    state: Arc<DStorageFileState>,
}

impl DStorageFileState {
    fn read_exact_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, i32> {
        let guard = self.file.lock().unwrap();
        let Some(ref f) = *guard else {
            return Err(E_HANDLE);
        };
        if len == 0 {
            return Ok(Vec::new());
        }
        let end = offset.checked_add(len as u64).ok_or(E_DSTORAGE_END_OF_FILE)?;
        let file_len = f.metadata().map_err(|e| io_error_to_hresult(&e))?.len();
        if end > file_len {
            return Err(E_DSTORAGE_END_OF_FILE);
        }
        let mut buf = vec![0u8; len];
        f.read_exact_at(&mut buf, offset)
            .map_err(|e| io_error_to_hresult(&e))?;
        Ok(buf)
    }

    fn close(&self) {
        *self.file.lock().unwrap() = None;
    }
}

unsafe extern "win64" fn ds_file_qi(this: usize, riid: usize, ppv: *mut usize) -> i32 {
    qi_impl::<FileObj>(this, riid, ppv, FILE_IIDS)
}

unsafe extern "win64" fn ds_file_close(this: usize) {
    if let Some(obj) = com_from_iface::<FileObj>(this, file_vtable().as_ptr()) {
        obj.inner.state.close();
    }
}

unsafe extern "win64" fn ds_file_get_info(this: usize, info_out: *mut u8) -> i32 {
    if info_out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = com_from_iface::<FileObj>(this, file_vtable().as_ptr()) else {
        return E_INVALIDARG;
    };
    let guard = obj.inner.state.file.lock().unwrap();
    let Some(ref f) = *guard else {
        return E_HANDLE;
    };
    let meta = match f.metadata() {
        Ok(m) => m,
        Err(e) => return io_error_to_hresult(&e),
    };
    let size = meta.len();
    let ft_create = unix_to_filetime(meta.ctime(), meta.ctime_nsec());
    let ft_access = unix_to_filetime(meta.atime(), meta.atime_nsec());
    let ft_write = unix_to_filetime(meta.mtime(), meta.mtime_nsec());

    // BY_HANDLE_FILE_INFORMATION is 52 bytes.
    std::ptr::write_bytes(info_out, 0, 52);
    std::ptr::write_unaligned(info_out as *mut u32, 0x80); // FILE_ATTRIBUTE_NORMAL
    std::ptr::write_unaligned(info_out.add(4) as *mut u32, ft_create as u32);
    std::ptr::write_unaligned(info_out.add(8) as *mut u32, (ft_create >> 32) as u32);
    std::ptr::write_unaligned(info_out.add(12) as *mut u32, ft_access as u32);
    std::ptr::write_unaligned(info_out.add(16) as *mut u32, (ft_access >> 32) as u32);
    std::ptr::write_unaligned(info_out.add(20) as *mut u32, ft_write as u32);
    std::ptr::write_unaligned(info_out.add(24) as *mut u32, (ft_write >> 32) as u32);
    std::ptr::write_unaligned(info_out.add(28) as *mut u32, meta.dev() as u32);
    std::ptr::write_unaligned(info_out.add(32) as *mut u32, (size >> 32) as u32);
    std::ptr::write_unaligned(info_out.add(36) as *mut u32, size as u32);
    std::ptr::write_unaligned(info_out.add(40) as *mut u32, meta.nlink() as u32);
    std::ptr::write_unaligned(info_out.add(44) as *mut u32, (meta.ino() >> 32) as u32);
    std::ptr::write_unaligned(info_out.add(48) as *mut u32, meta.ino() as u32);
    S_OK
}

fn file_vtable() -> &'static [usize; 5] {
    static VTBL: OnceLock<[usize; 5]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            ds_file_qi as *const () as usize,
            com_add_ref_typed::<FileObj> as *const () as usize,
            com_release_typed::<FileObj> as *const () as usize,
            ds_file_close as *const () as usize,
            ds_file_get_info as *const () as usize,
        ])
    })
}

// ============================================================================
// IDStorageStatusArray (IUnknown + IsComplete + GetHResult)
// ============================================================================

#[derive(Clone, Copy)]
struct StatusEntry {
    complete: bool,
    hresult: i32,
}

struct StatusArrayState {
    entries: Mutex<Vec<StatusEntry>>,
}

struct StatusArrayObj {
    state: Arc<StatusArrayState>,
}

impl StatusArrayState {
    fn new(capacity: usize) -> Self {
        Self {
            entries: Mutex::new(vec![
                StatusEntry {
                    complete: true,
                    hresult: S_OK,
                };
                capacity
            ]),
        }
    }

    fn mark_pending(&self, index: u32) -> bool {
        let mut entries = self.entries.lock().unwrap();
        let Some(entry) = entries.get_mut(index as usize) else {
            return false;
        };
        entry.complete = false;
        entry.hresult = E_PENDING;
        true
    }

    fn complete(&self, index: u32, hresult: i32) -> bool {
        let mut entries = self.entries.lock().unwrap();
        let Some(entry) = entries.get_mut(index as usize) else {
            return false;
        };
        entry.hresult = hresult;
        entry.complete = true;
        true
    }

    fn is_complete(&self, index: u32) -> bool {
        self.entries
            .lock()
            .unwrap()
            .get(index as usize)
            .is_some_and(|e| e.complete)
    }

    fn get_hresult(&self, index: u32) -> i32 {
        match self.entries.lock().unwrap().get(index as usize) {
            Some(e) if e.complete => e.hresult,
            Some(_) => E_PENDING,
            None => E_INVALIDARG,
        }
    }
}

unsafe extern "win64" fn ds_status_qi(this: usize, riid: usize, ppv: *mut usize) -> i32 {
    qi_impl::<StatusArrayObj>(this, riid, ppv, STATUS_ARRAY_IIDS)
}

unsafe extern "win64" fn ds_status_is_complete(this: usize, index: u32) -> u8 {
    let Some(obj) = com_from_iface::<StatusArrayObj>(this, status_vtable().as_ptr()) else {
        return 0;
    };
    obj.inner.state.is_complete(index) as u8
}

unsafe extern "win64" fn ds_status_get_hresult(this: usize, index: u32) -> i32 {
    let Some(obj) = com_from_iface::<StatusArrayObj>(this, status_vtable().as_ptr()) else {
        return E_INVALIDARG;
    };
    obj.inner.state.get_hresult(index)
}

fn status_vtable() -> &'static [usize; 5] {
    static VTBL: OnceLock<[usize; 5]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            ds_status_qi as *const () as usize,
            com_add_ref_typed::<StatusArrayObj> as *const () as usize,
            com_release_typed::<StatusArrayObj> as *const () as usize,
            ds_status_is_complete as *const () as usize,
            ds_status_get_hresult as *const () as usize,
        ])
    })
}

// ============================================================================
// GPU Copy Uploader (via d3d12_metal safe Rust APIs — zero objc2-metal)
// ============================================================================

const MAX_STAGED_UPLOAD_BYTES: u64 = 64 * 1024 * 1024;

/// Standard D3D12 64 KiB tile dimensions in texels `(tile_w, tile_h, tile_d)`.
/// Returns `None` for formats without a standard 64 KiB tile shape.
fn standard_tile_shape(desc: &ResourceDesc, plane: u32) -> Option<(u32, u32, u32)> {
    let fi = d3d12_metal::format::info(desc.Format);
    if fi.bytes == 0 {
        return None;
    }
    let bw = fi.block_w.max(1);
    let bh = fi.block_h.max(1);
    let bpe = match (desc.Format, plane) {
        (44..=47, 0) | (19..=22, 0) => 4u32,
        (44..=47, _) | (19..=22, _) => 1u32,
        _ => fi.bytes,
    };
    let (mut tw, mut th, td) = match desc.Dimension {
        dim::TEXTURE3D => match bpe {
            1 => (64 * bw, 32 * bh, 32),
            2 => (32 * bw, 32 * bh, 32),
            4 => (32 * bw, 32 * bh, 16),
            8 => (32 * bw, 16 * bh, 16),
            16 => (16 * bw, 16 * bh, 16),
            _ => return None,
        },
        dim::TEXTURE2D => match bpe {
            1 => (256 * bw, 256 * bh, 1),
            2 => (256 * bw, 128 * bh, 1),
            4 => (128 * bw, 128 * bh, 1),
            8 => (128 * bw, 64 * bh, 1),
            16 => (64 * bw, 64 * bh, 1),
            _ => return None,
        },
        dim::TEXTURE1D => match bpe {
            1 | 2 | 4 | 8 | 16 => ((D3D12_TILE_SIZE_BYTES as u32 / bpe) * bw, 1, 1),
            _ => return None,
        },
        _ => return None,
    };
    if desc.Dimension == dim::TEXTURE2D {
        match desc.SampleCount.max(1) {
            1 => {}
            2 => tw = (tw / 2).max(bw),
            4 => {
                tw = (tw / 2).max(bw);
                th = (th / 2).max(bh);
            }
            8 => {
                tw = (tw / 4).max(bw);
                th = (th / 2).max(bh);
            }
            16 => {
                tw = (tw / 4).max(bw);
                th = (th / 4).max(bh);
            }
            _ => return None,
        }
    }
    Some((tw, th, td))
}

struct CopyUploader {
    queue: MtlQueue,
    alloc: MtlCommandAllocator,
    cl: MtlCommandList,
    open: bool,
    staging: Vec<Arc<MtlResource>>,
    staged_bytes: u64,
}

impl CopyUploader {
    fn new() -> Result<Self, i32> {
        let queue = MtlQueue::new(ListType::Copy, "DirectStorage copy").ok_or(E_FAIL)?;
        let alloc = MtlCommandAllocator::new(ListType::Copy).ok_or(E_OUTOFMEMORY)?;
        let cl = MtlCommandList::new(ListType::Copy).ok_or(E_OUTOFMEMORY)?;
        Ok(Self {
            queue,
            alloc,
            cl,
            open: false,
            staging: Vec::new(),
            staged_bytes: 0,
        })
    }

    fn ensure_open(&mut self) -> Result<(), i32> {
        if !self.open {
            self.alloc.reset();
            self.cl.begin(&self.alloc).map_err(|_| E_FAIL)?;
            self.open = true;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), i32> {
        if !self.open {
            return Ok(());
        }
        let close_res = self.cl.close();
        self.open = false;
        if close_res.is_err() {
            self.staging.clear();
            self.staged_bytes = 0;
            return Err(E_FAIL);
        }
        let v = self.queue.submit(&[&self.cl.cb]);
        self.queue.timeline.wait_cpu(v, u64::MAX);
        self.staging.clear();
        self.staged_bytes = 0;
        Ok(())
    }

    fn alloc_upload_buffer(size: u64, label: &str) -> Result<Arc<MtlResource>, i32> {
        let desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: size.max(16),
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties {
            Type: heap_type::UPLOAD,
            ..Default::default()
        };
        let r = MtlResource::new_committed_default(&desc, &hp, label).map_err(|e| match e {
            d3d12_metal::metal::resource::CreateError::OutOfMemory(_) => E_OUTOFMEMORY,
            _ => E_FAIL,
        })?;
        let arc = Arc::new(r);
        arc.register_ptr();
        Ok(arc)
    }

    fn maybe_flush_staging(&mut self) -> Result<(), i32> {
        let limit = (STAGING_BUFFER_SIZE.load(Ordering::Relaxed) as u64)
            .clamp(1024 * 1024, MAX_STAGED_UPLOAD_BYTES);
        if self.staged_bytes >= limit {
            self.flush()?;
        }
        Ok(())
    }

    fn upload_buffer(&mut self, dst: &Arc<MtlResource>, offset: u64, data: &[u8]) -> Result<(), i32> {
        if data.is_empty() {
            return Ok(());
        }
        if let Some(cpu) = dst.cpu_ptr() {
            self.flush()?;
            unsafe {
                std::ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    cpu.add(offset as usize),
                    data.len(),
                );
            }
            return Ok(());
        }
        let up = Self::alloc_upload_buffer(data.len() as u64, "dstorage buf upload")?;
        let cpu = up.cpu_ptr().ok_or(E_FAIL)?;
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), cpu, data.len());
        }
        self.ensure_open()?;
        self.cl.barrier();
        self.cl
            .copy_buffer(dst, offset, &up, 0, data.len() as u64)
            .map_err(|_| E_FAIL)?;
        self.cl.barrier();
        self.staged_bytes = self.staged_bytes.saturating_add(up.desc.Width);
        self.staging.push(up);
        self.maybe_flush_staging()
    }

    fn upload_texture_region(
        &mut self,
        dst: &Arc<MtlResource>,
        subresource: u32,
        region: D3D12Box,
        data: &[u8],
    ) -> Result<(), i32> {
        if dst.desc.is_buffer() || subresource >= dst.desc.subresource_count() {
            return Err(E_INVALIDARG);
        }
        if region.right <= region.left
            || region.bottom <= region.top
            || region.back <= region.front
        {
            return Err(E_INVALIDARG);
        }
        let (mip, _slice, plane) = split_subresource(dst, subresource);
        let mw = (dst.desc.Width >> mip).max(1) as u32;
        let mh = (dst.desc.Height >> mip).max(1);
        let md = if dst.desc.Dimension == dim::TEXTURE3D {
            (dst.desc.depth() >> mip).max(1)
        } else {
            1
        };
        let fi = d3d12_metal::format::info(dst.desc.Format);
        if fi.bytes == 0 {
            return Err(E_INVALIDARG);
        }
        let bw = fi.block_w.max(1);
        let bh = fi.block_h.max(1);
        let bpe = match (dst.desc.Format, plane) {
            (44..=47, 0) | (19..=22, 0) => 4usize,
            (44..=47, _) | (19..=22, _) => 1usize,
            _ => fi.bytes as usize,
        };
        if region.left % bw != 0
            || region.top % bh != 0
            || region.right > mw.next_multiple_of(bw)
            || region.bottom > mh.next_multiple_of(bh)
            || region.back > md
        {
            return Err(E_INVALIDARG);
        }

        let w = region.right - region.left;
        let h = region.bottom - region.top;
        let d = region.back - region.front;
        let row_bytes = (w.div_ceil(bw) as usize)
            .checked_mul(bpe)
            .ok_or(E_INVALIDARG)?;
        let num_rows = h.div_ceil(bh) as usize;
        let slice_bytes = row_bytes.checked_mul(num_rows).ok_or(E_INVALIDARG)?;
        let needed_bytes = slice_bytes.checked_mul(d as usize).ok_or(E_INVALIDARG)?;
        if data.len() < needed_bytes {
            return Err(E_INVALIDARG);
        }

        let dst_row_pitch = row_bytes.next_multiple_of(256).max(256);
        let dst_slice_pitch = dst_row_pitch.checked_mul(num_rows).ok_or(E_OUTOFMEMORY)?;
        let total = dst_slice_pitch
            .checked_mul(d as usize)
            .ok_or(E_OUTOFMEMORY)?;
        let up = Self::alloc_upload_buffer(total as u64, "dstorage tex region upload")?;
        let cpu = up.cpu_ptr().ok_or(E_FAIL)?;
        for z in 0..d as usize {
            for y in 0..num_rows {
                let soff = z * slice_bytes + y * row_bytes;
                let doff = z * dst_slice_pitch + y * dst_row_pitch;
                unsafe {
                    std::ptr::copy_nonoverlapping(data.as_ptr().add(soff), cpu.add(doff), row_bytes);
                }
            }
        }
        let fp = PlacedFootprint {
            offset: 0,
            format: plane_format(dst.desc.Format, plane),
            width: w.next_multiple_of(bw),
            height: h.next_multiple_of(bh),
            depth: d,
            row_pitch: dst_row_pitch as u32,
            _pad: 0,
        };
        self.ensure_open()?;
        self.cl.barrier();
        self.cl
            .copy_texture_region(
                CopyLoc::Subresource(dst, subresource),
                [region.left, region.top, region.front],
                CopyLoc::Footprint(&up, fp),
                None,
            )
            .map_err(|_| E_FAIL)?;
        self.cl.barrier();
        self.staged_bytes = self.staged_bytes.saturating_add(up.desc.Width);
        self.staging.push(up);
        self.maybe_flush_staging()
    }

    fn upload_multiple_subresources(
        &mut self,
        dst: &Arc<MtlResource>,
        first: u32,
        count: u32,
        data: &[u8],
    ) -> Result<(), i32> {
        if dst.desc.is_buffer() {
            return Err(E_INVALIDARG);
        }
        if count == 0 {
            return Ok(());
        }
        let total_subs = dst.desc.subresource_count();
        if first.checked_add(count).is_none_or(|end| end > total_subs) {
            return Err(E_INVALIDARG);
        }
        let (layouts, total_aligned) =
            copyable_footprints(&dst.desc, first, count, 0).ok_or(E_INVALIDARG)?;

        let mut needed_bytes = 0usize;
        for lay in &layouts {
            let sub_bytes = (lay.row_size as usize)
                .checked_mul(lay.num_rows as usize)
                .and_then(|v| v.checked_mul(lay.footprint.depth.max(1) as usize))
                .ok_or(E_INVALIDARG)?;
            needed_bytes = needed_bytes.checked_add(sub_bytes).ok_or(E_INVALIDARG)?;
        }
        if data.len() < needed_bytes {
            return Err(E_INVALIDARG);
        }

        let padded_end = layouts
            .last()
            .and_then(|l| {
                let slice_sz = (l.footprint.row_pitch as u64)
                    .checked_mul(l.num_rows as u64)?
                    .checked_mul(l.footprint.depth.max(1) as u64)?;
                l.footprint.offset.checked_add(slice_sz)
            })
            .unwrap_or(total_aligned)
            .max(total_aligned);

        let up = Self::alloc_upload_buffer(padded_end, "dstorage multi-subresource upload")?;
        let cpu = up.cpu_ptr().ok_or(E_FAIL)?;

        let mut src_cursor = 0usize;
        for lay in &layouts {
            let row_bytes = lay.row_size as usize;
            let num_rows = lay.num_rows as usize;
            let depth = lay.footprint.depth.max(1) as usize;
            let dst_row_pitch = lay.footprint.row_pitch as usize;
            let dst_slice_pitch = dst_row_pitch * num_rows;
            let base_off = lay.footprint.offset as usize;
            for z in 0..depth {
                for y in 0..num_rows {
                    let doff = base_off + z * dst_slice_pitch + y * dst_row_pitch;
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            data.as_ptr().add(src_cursor),
                            cpu.add(doff),
                            row_bytes,
                        );
                    }
                    src_cursor += row_bytes;
                }
            }
        }

        self.ensure_open()?;
        self.cl.barrier();
        for (i, lay) in layouts.iter().enumerate() {
            let sub = first + i as u32;
            self.cl
                .copy_texture_region(
                    CopyLoc::Subresource(dst, sub),
                    [0, 0, 0],
                    CopyLoc::Footprint(&up, lay.footprint),
                    None,
                )
                .map_err(|_| E_FAIL)?;
        }
        self.cl.barrier();
        self.staged_bytes = self.staged_bytes.saturating_add(up.desc.Width);
        self.staging.push(up);
        self.maybe_flush_staging()
    }

    fn upload_tiles(
        &mut self,
        dst: &Arc<MtlResource>,
        coord: TileCoord,
        region: TileRegionSize,
        data: &[u8],
    ) -> Result<(), i32> {
        // Tiled buffers: linear 64 KiB tiles along X on subresource 0.
        if dst.desc.is_buffer() {
            if coord.y != 0 || coord.z != 0 || coord.subresource != 0 {
                return Err(E_INVALIDARG);
            }
            let num_tiles = if region.use_box {
                if region.num_tiles == 0
                    && region.width == 0
                    && region.height == 0
                    && region.depth == 0
                {
                    return Ok(());
                }
                if region.width == 0 || region.height != 1 || region.depth != 1 {
                    return Err(E_INVALIDARG);
                }
                if region.num_tiles != 0 && region.num_tiles != region.width {
                    return Err(E_INVALIDARG);
                }
                region.width
            } else {
                region.num_tiles
            };
            if num_tiles == 0 {
                return Ok(());
            }
            let offset = (coord.x as u64)
                .checked_mul(D3D12_TILE_SIZE_BYTES as u64)
                .ok_or(E_INVALIDARG)?;
            let total_bytes = (num_tiles as u64)
                .checked_mul(D3D12_TILE_SIZE_BYTES as u64)
                .ok_or(E_INVALIDARG)?;
            if offset >= dst.desc.Width {
                return Err(E_INVALIDARG);
            }
            let copy_len = total_bytes.min(dst.desc.Width - offset) as usize;
            if data.len() < copy_len {
                return Err(E_INVALIDARG);
            }
            return self.upload_buffer(dst, offset, &data[..copy_len]);
        }

        if coord.subresource >= dst.desc.subresource_count() {
            return Err(E_INVALIDARG);
        }
        let (mip, _slice, plane) = split_subresource(dst, coord.subresource);
        let mw = (dst.desc.Width >> mip).max(1) as u32;
        let mh = (dst.desc.Height >> mip).max(1);
        let md = if dst.desc.Dimension == dim::TEXTURE3D {
            (dst.desc.depth() >> mip).max(1)
        } else {
            1
        };
        let fi = d3d12_metal::format::info(dst.desc.Format);
        if fi.bytes == 0 {
            return Err(E_INVALIDARG);
        }
        let bw = fi.block_w.max(1);
        let bh = fi.block_h.max(1);
        let bpe = match (dst.desc.Format, plane) {
            (44..=47, 0) | (19..=22, 0) => 4usize,
            (44..=47, _) | (19..=22, _) => 1usize,
            _ => fi.bytes as usize,
        };
        let (tile_w, tile_h, tile_d) = standard_tile_shape(&dst.desc, plane).ok_or(E_INVALIDARG)?;
        let sub_tiles_w = mw.div_ceil(tile_w);
        let sub_tiles_h = mh.div_ceil(tile_h);
        let sub_tiles_d = md.div_ceil(tile_d);
        if coord.x >= sub_tiles_w || coord.y >= sub_tiles_h || coord.z >= sub_tiles_d {
            return Err(E_INVALIDARG);
        }

        // Resolve tile coordinate list in D3D12 CopyTiles traversal order (X, then Y, then Z).
        let mut tiles: Vec<(u32, u32, u32)> = Vec::new();
        if region.use_box {
            if region.num_tiles == 0
                && region.width == 0
                && region.height == 0
                && region.depth == 0
            {
                return Ok(());
            }
            let rw = region.width;
            let rh = region.height as u32;
            let rd = region.depth as u32;
            if rw == 0 || rh == 0 || rd == 0 {
                return Err(E_INVALIDARG);
            }
            let box_tiles = rw
                .checked_mul(rh)
                .and_then(|v| v.checked_mul(rd))
                .ok_or(E_INVALIDARG)?;
            if region.num_tiles != 0 && region.num_tiles != box_tiles {
                return Err(E_INVALIDARG);
            }
            if coord.x.checked_add(rw).is_none_or(|e| e > sub_tiles_w)
                || coord.y.checked_add(rh).is_none_or(|e| e > sub_tiles_h)
                || coord.z.checked_add(rd).is_none_or(|e| e > sub_tiles_d)
            {
                return Err(E_INVALIDARG);
            }
            tiles.reserve(box_tiles as usize);
            for dz in 0..rd {
                for dy in 0..rh {
                    for dx in 0..rw {
                        tiles.push((coord.x + dx, coord.y + dy, coord.z + dz));
                    }
                }
            }
        } else {
            if region.num_tiles == 0 {
                return Ok(());
            }
            tiles.reserve(region.num_tiles as usize);
            let (mut tx, mut ty, mut tz) = (coord.x, coord.y, coord.z);
            for _ in 0..region.num_tiles {
                if tz >= sub_tiles_d {
                    return Err(E_INVALIDARG);
                }
                tiles.push((tx, ty, tz));
                tx += 1;
                if tx >= sub_tiles_w {
                    tx = 0;
                    ty += 1;
                    if ty >= sub_tiles_h {
                        ty = 0;
                        tz += 1;
                    }
                }
            }
        }

        let needed_bytes = tiles
            .len()
            .checked_mul(D3D12_TILE_SIZE_BYTES)
            .ok_or(E_INVALIDARG)?;
        if data.len() < needed_bytes {
            return Err(E_INVALIDARG);
        }

        let tile_row_bytes = (tile_w / bw) as usize * bpe;
        let tile_rows = (tile_h / bh) as usize;
        let tile_slice_bytes = tile_row_bytes * tile_rows;
        let mw_aligned = mw.next_multiple_of(bw);
        let mh_aligned = mh.next_multiple_of(bh);
        let copy_fmt = plane_format(dst.desc.Format, plane);

        struct TileCopyPlan {
            tile_idx: usize,
            dst_box: D3D12Box,
            copy_row_bytes: usize,
            copy_rows: usize,
            dst_row_pitch: usize,
            dst_slice_pitch: usize,
            stage_off: usize,
        }

        let mut cursor = 0usize;
        while cursor < tiles.len() {
            let mut plans: Vec<TileCopyPlan> = Vec::new();
            let mut stage_total = 0usize;

            while cursor < tiles.len() {
                let (tx, ty, tz) = tiles[cursor];
                let left = tx * tile_w;
                let top = ty * tile_h;
                let front = tz * tile_d;
                let right = (left + tile_w).min(mw_aligned);
                let bottom = (top + tile_h).min(mh_aligned);
                let back = (front + tile_d).min(md);
                let w = right - left;
                let h = bottom - top;
                let d = back - front;
                let copy_row_bytes = (w.div_ceil(bw) as usize)
                    .checked_mul(bpe)
                    .ok_or(E_INVALIDARG)?;
                let copy_rows = h.div_ceil(bh) as usize;
                let dst_row_pitch = copy_row_bytes.next_multiple_of(256).max(256);
                let dst_slice_pitch = dst_row_pitch.checked_mul(copy_rows).ok_or(E_OUTOFMEMORY)?;
                let tile_staged = dst_slice_pitch
                    .checked_mul(d as usize)
                    .ok_or(E_OUTOFMEMORY)?;
                let stage_off = stage_total.next_multiple_of(512);
                let next_total = stage_off.checked_add(tile_staged).ok_or(E_OUTOFMEMORY)?;
                if !plans.is_empty() && next_total as u64 > MAX_STAGED_UPLOAD_BYTES {
                    break;
                }
                plans.push(TileCopyPlan {
                    tile_idx: cursor,
                    dst_box: D3D12Box {
                        left,
                        top,
                        front,
                        right,
                        bottom,
                        back,
                    },
                    copy_row_bytes,
                    copy_rows,
                    dst_row_pitch,
                    dst_slice_pitch,
                    stage_off,
                });
                stage_total = next_total;
                cursor += 1;
            }

            let up = Self::alloc_upload_buffer(stage_total as u64, "dstorage tiles upload")?;
            let cpu = up.cpu_ptr().ok_or(E_FAIL)?;

            for plan in &plans {
                let tile_src = &data[plan.tile_idx * D3D12_TILE_SIZE_BYTES
                    ..(plan.tile_idx + 1) * D3D12_TILE_SIZE_BYTES];
                let depth = (plan.dst_box.back - plan.dst_box.front) as usize;
                for z in 0..depth {
                    for y in 0..plan.copy_rows {
                        let soff = z * tile_slice_bytes + y * tile_row_bytes;
                        let doff = plan.stage_off + z * plan.dst_slice_pitch + y * plan.dst_row_pitch;
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                tile_src.as_ptr().add(soff),
                                cpu.add(doff),
                                plan.copy_row_bytes,
                            );
                        }
                    }
                }
            }

            self.ensure_open()?;
            self.cl.barrier();
            for plan in &plans {
                let b = plan.dst_box;
                let fp = PlacedFootprint {
                    offset: plan.stage_off as u64,
                    format: copy_fmt,
                    width: b.right - b.left,
                    height: b.bottom - b.top,
                    depth: b.back - b.front,
                    row_pitch: plan.dst_row_pitch as u32,
                    _pad: 0,
                };
                self.cl
                    .copy_texture_region(
                        CopyLoc::Subresource(dst, coord.subresource),
                        [b.left, b.top, b.front],
                        CopyLoc::Footprint(&up, fp),
                        None,
                    )
                    .map_err(|_| E_FAIL)?;
            }
            self.cl.barrier();
            self.staged_bytes = self.staged_bytes.saturating_add(up.desc.Width);
            self.staging.push(up);
            self.maybe_flush_staging()?;
        }

        Ok(())
    }
}

impl Drop for CopyUploader {
    fn drop(&mut self) {
        if self.open {
            let _ = self.cl.close();
            self.open = false;
        }
        self.queue.wait_idle();
        self.staging.clear();
        self.staged_bytes = 0;
    }
}

// ============================================================================
// IDStorageQueue / IDStorageQueue1 / IDStorageQueue2
// ============================================================================

enum QueuedSource {
    Memory {
        ptr: usize,
        size: u32,
    },
    File {
        file: Option<Arc<DStorageFileState>>,
        offset: u64,
        size: u32,
    },
    Invalid,
}

enum QueuedDest {
    Memory {
        ptr: usize,
        size: u32,
    },
    Buffer {
        resource: Option<Arc<MtlResource>>,
        offset: u64,
        size: u32,
    },
    TextureRegion {
        resource: Option<Arc<MtlResource>>,
        subresource_index: u32,
        region: D3D12Box,
    },
    MultipleSubresources {
        resource: Option<Arc<MtlResource>>,
        first_subresource: u32,
    },
    MultipleSubresourcesRange {
        resource: Option<Arc<MtlResource>>,
        first_subresource: u32,
        num_subresources: u32,
    },
    Tiles {
        resource: Option<Arc<MtlResource>>,
        coord: TileCoord,
        region: TileRegionSize,
    },
    Invalid,
}

struct QueuedRequest {
    compression_format: u8,
    source: QueuedSource,
    dest: QueuedDest,
    uncompressed_size: u32,
    cancellation_tag: u64,
    request_name: Vec<u8>,
    filename: String,
    raw_request: [u8; std::mem::size_of::<DSTORAGE_REQUEST>()],
}

impl QueuedRequest {
    fn read_source_bytes(&self) -> Result<Vec<u8>, i32> {
        match &self.source {
            QueuedSource::Memory { ptr, size } => {
                let sz = *size as usize;
                if sz == 0 {
                    Ok(Vec::new())
                } else if *ptr == 0 {
                    Err(E_INVALIDARG)
                } else {
                    let slice = unsafe { std::slice::from_raw_parts(*ptr as *const u8, sz) };
                    Ok(slice.to_vec())
                }
            }
            QueuedSource::File { file, offset, size } => {
                let Some(ref f) = file else {
                    return Err(E_INVALIDARG);
                };
                f.read_exact_at(*offset, *size as usize)
            }
            QueuedSource::Invalid => Err(E_INVALIDARG),
        }
    }

    fn expected_uncompressed_size(&self) -> Result<usize, i32> {
        if self.uncompressed_size > 0 {
            return Ok(self.uncompressed_size as usize);
        }
        match &self.dest {
            QueuedDest::Memory { size, .. } | QueuedDest::Buffer { size, .. } => Ok(*size as usize),
            QueuedDest::Tiles { region, .. } => {
                let n = if region.use_box {
                    if region.num_tiles != 0 {
                        region.num_tiles
                    } else {
                        region
                            .width
                            .checked_mul(region.height as u32)
                            .and_then(|v| v.checked_mul(region.depth as u32))
                            .ok_or(E_INVALIDARG)?
                    }
                } else {
                    region.num_tiles
                };
                (n as usize)
                    .checked_mul(D3D12_TILE_SIZE_BYTES)
                    .ok_or(E_INVALIDARG)
            }
            _ => Ok(0),
        }
    }
}

struct OwnedD3D12Fence {
    ptr: usize,
    owned: bool,
}

impl Drop for OwnedD3D12Fence {
    fn drop(&mut self) {
        if self.owned && self.ptr != 0 {
            unsafe {
                Obj::release(self.ptr as *mut Obj<crate::d3d12::fence::Fence>);
            }
            self.ptr = 0;
        }
    }
}

enum QueueItem {
    Request(QueuedRequest),
    Status {
        array_ptr: usize,
        array: Option<Arc<StatusArrayState>>,
        index: u32,
        valid_slot: bool,
    },
    Signal {
        fence: OwnedD3D12Fence,
        value: u64,
    },
    Event {
        handle: usize,
    },
}

enum CustomDecompCompletion {
    Pending,
    Done {
        hresult: i32,
        decompressed: Vec<u8>,
    },
}

struct PendingCustomRequest {
    req: QueuedRequest,
    completion: Arc<Mutex<CustomDecompCompletion>>,
}

enum SubmittedEntry {
    Item(QueueItem),
    Dispatching,
    InFlightCustom(PendingCustomRequest),
}

enum FrontAction {
    Item(QueueItem),
    CustomDone(QueuedRequest, i32, Vec<u8>),
}

enum FirstFailureDetail {
    Request {
        filename: String,
        request_name: Vec<u8>,
        raw_request: [u8; std::mem::size_of::<DSTORAGE_REQUEST>()],
    },
    Status {
        status_array: usize,
        index: u32,
    },
    Signal {
        fence: usize,
        value: u64,
    },
    Event {
        handle: usize,
    },
}

struct FirstFailureRecord {
    hresult: i32,
    command_type: i32,
    detail: FirstFailureDetail,
}

struct QueueInner {
    pending: VecDeque<QueueItem>,
    submitted: VecDeque<SubmittedEntry>,
    in_flight: usize,
    closed: bool,
    failure_count: u32,
    first_failure: Option<FirstFailureRecord>,
    batch_first_error: Option<i32>,
}

struct QueueState {
    source_type: u64,
    capacity: u16,
    auto_submit_threshold: u16,
    priority: i8,
    name_buf: Option<Box<[u8]>>,
    device: usize,
    error_event: Arc<Event>,
    inner: Mutex<QueueInner>,
    exec_lock: Mutex<()>,
    uploader: Mutex<Option<CopyUploader>>,
}

struct QueueObj {
    state: Arc<QueueState>,
}

impl QueueState {
    fn desc(&self) -> DSTORAGE_QUEUE_DESC {
        DSTORAGE_QUEUE_DESC {
            source_type: self.source_type,
            capacity: self.capacity,
            priority: self.priority,
            _pad0: [0; 5],
            name: self
                .name_buf
                .as_ref()
                .map_or(std::ptr::null(), |b| b.as_ptr()),
            device: self.device,
        }
    }

    fn record_error(&self, hresult: i32, command_type: i32, detail: FirstFailureDetail) {
        if DEBUG_FLAGS.load(Ordering::Relaxed) & DSTORAGE_DEBUG_SHOW_ERRORS != 0 {
            log::warn!(
                target: "dstorage",
                "DirectStorage queue error: cmd_type={command_type} hr={hresult:#010x}"
            );
        }
        let mut st = self.inner.lock().unwrap();
        st.failure_count = st.failure_count.saturating_add(1);
        if st.first_failure.is_none() {
            st.first_failure = Some(FirstFailureRecord {
                hresult,
                command_type,
                detail,
            });
        }
        if st.batch_first_error.is_none() {
            st.batch_first_error = Some(hresult);
        }
        drop(st);
        self.error_event.set();
    }

    fn flush_gpu_uploads(&self) -> Result<(), i32> {
        let mut up_guard = self.uploader.lock().unwrap();
        if let Some(ref mut uploader) = *up_guard {
            uploader.flush()?;
        }
        Ok(())
    }

    fn with_uploader<F>(&self, f: F) -> Result<(), i32>
    where
        F: FnOnce(&mut CopyUploader) -> Result<(), i32>,
    {
        let mut up_guard = self.uploader.lock().unwrap();
        if up_guard.is_none() {
            *up_guard = Some(CopyUploader::new()?);
        }
        let uploader = up_guard.as_mut().unwrap();
        f(uploader)
    }

    fn deliver_payload(&self, req: &QueuedRequest, payload: &[u8]) -> Result<(), i32> {
        match &req.dest {
            QueuedDest::Memory { ptr, size } => {
                let dst_size = *size as usize;
                if payload.len() > dst_size {
                    return Err(E_INVALIDARG);
                }
                if !payload.is_empty() {
                    if *ptr == 0 {
                        return Err(E_INVALIDARG);
                    }
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            payload.as_ptr(),
                            *ptr as *mut u8,
                            payload.len(),
                        );
                    }
                }
                Ok(())
            }
            QueuedDest::Buffer {
                resource,
                offset,
                size,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                if !mtl_res.desc.is_buffer() {
                    return Err(E_INVALIDARG);
                }
                let copy_len = *size as usize;
                let end = offset.checked_add(copy_len as u64).ok_or(E_INVALIDARG)?;
                if end > mtl_res.desc.Width || payload.len() < copy_len {
                    return Err(E_INVALIDARG);
                }
                if copy_len == 0 {
                    return Ok(());
                }
                if let Some(cpu) = mtl_res.cpu_ptr() {
                    self.flush_gpu_uploads()?;
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            payload.as_ptr(),
                            cpu.add(*offset as usize),
                            copy_len,
                        );
                    }
                    Ok(())
                } else {
                    self.with_uploader(|up| up.upload_buffer(mtl_res, *offset, &payload[..copy_len]))
                }
            }
            QueuedDest::TextureRegion {
                resource,
                subresource_index,
                region,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                self.with_uploader(|up| {
                    up.upload_texture_region(mtl_res, *subresource_index, *region, payload)
                })
            }
            QueuedDest::MultipleSubresources {
                resource,
                first_subresource,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                if mtl_res.desc.is_buffer() {
                    return Err(E_INVALIDARG);
                }
                let total = mtl_res.desc.subresource_count();
                if *first_subresource >= total {
                    return Err(E_INVALIDARG);
                }
                let count = total - *first_subresource;
                self.with_uploader(|up| {
                    up.upload_multiple_subresources(mtl_res, *first_subresource, count, payload)
                })
            }
            QueuedDest::MultipleSubresourcesRange {
                resource,
                first_subresource,
                num_subresources,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                self.with_uploader(|up| {
                    up.upload_multiple_subresources(
                        mtl_res,
                        *first_subresource,
                        *num_subresources,
                        payload,
                    )
                })
            }
            QueuedDest::Tiles {
                resource,
                coord,
                region,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                self.with_uploader(|up| up.upload_tiles(mtl_res, *coord, *region, payload))
            }
            QueuedDest::Invalid => Err(E_INVALIDARG),
        }
    }

    fn execute_non_custom_request(&self, req: &QueuedRequest) -> Result<(), i32> {
        // 1. Read source bytes
        let raw_bytes = req.read_source_bytes()?;

        // 2. Decompress payload (never silently memcpy on decompression failure!)
        let payload: Vec<u8> = match req.compression_format {
            DSTORAGE_COMPRESSION_FORMAT_NONE => raw_bytes,
            DSTORAGE_COMPRESSION_FORMAT_GDEFLATE => {
                let decompressed = decompress_gdeflate_to_vec(&raw_bytes)?;
                if req.uncompressed_size > 0
                    && decompressed.len() != req.uncompressed_size as usize
                {
                    return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
                }
                decompressed
            }
            _ => return Err(E_INVALIDARG),
        };

        // 3. Deliver payload to destination
        self.deliver_payload(req, &payload)
    }

    fn execute_item(&self, item: QueueItem) {
        match &item {
            QueueItem::Request(req) => {
                if let Err(hr) = self.execute_non_custom_request(req) {
                    self.record_error(
                        hr,
                        DSTORAGE_COMMAND_TYPE_REQUEST,
                        FirstFailureDetail::Request {
                            filename: req.filename.clone(),
                            request_name: req.request_name.clone(),
                            raw_request: req.raw_request,
                        },
                    );
                }
            }
            QueueItem::Status {
                array_ptr,
                array,
                index,
                valid_slot,
            } => {
                let array_ptr = *array_ptr;
                let index = *index;
                let valid_slot = *valid_slot;
                let flush_err = self.flush_gpu_uploads().err();
                let batch_hr = {
                    let mut st = self.inner.lock().unwrap();
                    let prev = st.batch_first_error.take();
                    prev.or(flush_err).unwrap_or(S_OK)
                };
                if let Some(err_hr) = flush_err {
                    self.record_error(
                        err_hr,
                        DSTORAGE_COMMAND_TYPE_STATUS,
                        FirstFailureDetail::Status {
                            status_array: array_ptr,
                            index,
                        },
                    );
                }
                if valid_slot {
                    if let Some(sa) = array {
                        if !sa.complete(index, batch_hr) {
                            self.record_error(
                                E_INVALIDARG,
                                DSTORAGE_COMMAND_TYPE_STATUS,
                                FirstFailureDetail::Status {
                                    status_array: array_ptr,
                                    index,
                                },
                            );
                        }
                    }
                } else {
                    self.record_error(
                        E_INVALIDARG,
                        DSTORAGE_COMMAND_TYPE_STATUS,
                        FirstFailureDetail::Status {
                            status_array: array_ptr,
                            index,
                        },
                    );
                }
            }
            QueueItem::Signal { fence, value } => {
                let fence_ptr = fence.ptr;
                let value = *value;
                if let Err(hr) = self.flush_gpu_uploads() {
                    self.record_error(
                        hr,
                        DSTORAGE_COMMAND_TYPE_SIGNAL,
                        FirstFailureDetail::Signal {
                            fence: fence_ptr,
                            value,
                        },
                    );
                }
                let sig_hr = unsafe { signal_d3d12_fence(fence_ptr, value) };
                if sig_hr != S_OK {
                    self.record_error(
                        sig_hr,
                        DSTORAGE_COMMAND_TYPE_SIGNAL,
                        FirstFailureDetail::Signal {
                            fence: fence_ptr,
                            value,
                        },
                    );
                }
            }
            QueueItem::Event { handle } => {
                let handle = *handle;
                if let Err(hr) = self.flush_gpu_uploads() {
                    self.record_error(
                        hr,
                        DSTORAGE_COMMAND_TYPE_EVENT,
                        FirstFailureDetail::Event { handle },
                    );
                }
                if handle == 0 {
                    self.record_error(
                        E_HANDLE,
                        DSTORAGE_COMMAND_TYPE_EVENT,
                        FirstFailureDetail::Event { handle },
                    );
                } else {
                    let ok = unsafe { crate::kernel32::threads::SetEvent(handle as HANDLE) };
                    if ok == FALSE {
                        self.record_error(
                            E_HANDLE,
                            DSTORAGE_COMMAND_TYPE_EVENT,
                            FirstFailureDetail::Event { handle },
                        );
                    }
                }
            }
        }
    }

    fn drain_submitted(self: &Arc<Self>) {
        let _exec = self.exec_lock.lock().unwrap();
        let mut did_work = false;

        loop {
            // Phase 1: dispatch any contiguous leading custom-decompression requests
            // so caller worker threads can pull an entire batch via GetRequests/GetRequests1.
            loop {
                let next_custom = {
                    let mut st = self.inner.lock().unwrap();
                    if st.closed {
                        return;
                    }
                    let mut idx = 0usize;
                    while idx < st.submitted.len() {
                        match &st.submitted[idx] {
                            SubmittedEntry::InFlightCustom(_) => idx += 1,
                            SubmittedEntry::Item(QueueItem::Request(req))
                                if req.compression_format >= DSTORAGE_CUSTOM_COMPRESSION_0 =>
                            {
                                break;
                            }
                            _ => {
                                idx = usize::MAX;
                                break;
                            }
                        }
                    }
                    if idx < st.submitted.len() {
                        let slot = std::mem::replace(
                            &mut st.submitted[idx],
                            SubmittedEntry::Dispatching,
                        );
                        match slot {
                            SubmittedEntry::Item(QueueItem::Request(req)) => Some((idx, req)),
                            _ => unreachable!(),
                        }
                    } else {
                        None
                    }
                };
                let Some((idx, req)) = next_custom else {
                    break;
                };

                let prep = req.read_source_bytes().and_then(|raw| {
                    let dst_size = req.expected_uncompressed_size()?;
                    Ok((raw, dst_size))
                });

                let mut st = self.inner.lock().unwrap();
                if st.closed {
                    return;
                }
                match prep {
                    Ok((raw_bytes, dst_size)) => {
                        let completion = Arc::new(Mutex::new(CustomDecompCompletion::Pending));
                        custom_decomp_state().enqueue(
                            req.compression_format,
                            raw_bytes,
                            dst_size,
                            Arc::clone(self),
                            Arc::clone(&completion),
                        );
                        st.submitted[idx] = SubmittedEntry::InFlightCustom(PendingCustomRequest {
                            req,
                            completion,
                        });
                    }
                    Err(err_hr) => {
                        let completion = Arc::new(Mutex::new(CustomDecompCompletion::Done {
                            hresult: err_hr,
                            decompressed: Vec::new(),
                        }));
                        st.submitted[idx] = SubmittedEntry::InFlightCustom(PendingCustomRequest {
                            req,
                            completion,
                        });
                    }
                }
            }

            // Phase 2: pop and execute the front entry if ready (FIFO order).
            let front_action = {
                let mut st = self.inner.lock().unwrap();
                if st.closed {
                    return;
                }
                match st.submitted.front() {
                    None => break,
                    Some(SubmittedEntry::InFlightCustom(pending)) => {
                        let mut guard = pending.completion.lock().unwrap();
                        if matches!(*guard, CustomDecompCompletion::Pending) {
                            break;
                        }
                        let done = std::mem::replace(&mut *guard, CustomDecompCompletion::Pending);
                        drop(guard);
                        let Some(SubmittedEntry::InFlightCustom(p)) = st.submitted.pop_front()
                        else {
                            unreachable!();
                        };
                        match done {
                            CustomDecompCompletion::Done {
                                hresult,
                                decompressed,
                            } => FrontAction::CustomDone(p.req, hresult, decompressed),
                            CustomDecompCompletion::Pending => unreachable!(),
                        }
                    }
                    Some(SubmittedEntry::Item(_)) => {
                        let Some(SubmittedEntry::Item(item)) = st.submitted.pop_front() else {
                            unreachable!();
                        };
                        FrontAction::Item(item)
                    }
                    Some(SubmittedEntry::Dispatching) => unreachable!(),
                }
            };

            did_work = true;
            match front_action {
                FrontAction::CustomDone(req, hr, decompressed) => {
                    if hr < 0 {
                        self.record_error(
                            hr,
                            DSTORAGE_COMMAND_TYPE_REQUEST,
                            FirstFailureDetail::Request {
                                filename: req.filename.clone(),
                                request_name: req.request_name.clone(),
                                raw_request: req.raw_request,
                            },
                        );
                    } else if let Err(dest_hr) = self.deliver_payload(&req, &decompressed) {
                        self.record_error(
                            dest_hr,
                            DSTORAGE_COMMAND_TYPE_REQUEST,
                            FirstFailureDetail::Request {
                                filename: req.filename.clone(),
                                request_name: req.request_name.clone(),
                                raw_request: req.raw_request,
                            },
                        );
                    }
                }
                FrontAction::Item(item) => {
                    self.execute_item(item);
                }
            }

            let mut st = self.inner.lock().unwrap();
            st.in_flight = st.in_flight.saturating_sub(1);
        }

        // Ensure any trailing GPU uploads committed in this pass are landed before returning.
        if did_work {
            if let Err(hr) = self.flush_gpu_uploads() {
                log::warn!(target: "dstorage", "DirectStorage trailing GPU copy flush failed: {hr:#010x}");
            }
        }
    }

    fn execute_submit(self: &Arc<Self>) {
        let moved = {
            let mut st = self.inner.lock().unwrap();
            if st.closed || st.pending.is_empty() {
                false
            } else {
                let count = st.pending.len();
                let drained: Vec<QueueItem> = st.pending.drain(..).collect();
                for item in drained {
                    st.submitted.push_back(SubmittedEntry::Item(item));
                }
                st.in_flight = st.in_flight.saturating_add(count);
                true
            }
        };
        if moved {
            self.drain_submitted();
        }
    }

    fn enqueue_item(self: &Arc<Self>, item: QueueItem) {
        let should_auto_submit = {
            let mut st = self.inner.lock().unwrap();
            if st.closed {
                return;
            }
            st.pending.push_back(item);
            st.pending.len() >= self.auto_submit_threshold as usize
        };
        if should_auto_submit {
            self.execute_submit();
        }
    }
}

unsafe fn signal_d3d12_fence(fence_ptr: usize, value: u64) -> i32 {
    if fence_ptr == 0 {
        return E_INVALIDARG;
    }
    if let Some(fence) = crate::d3d12::fence::from_ptr(fence_ptr as *mut c_void) {
        fence.f.signal_cpu(value);
        return S_OK;
    }
    // Fallback for proxy / foreign COM fence wrappers: ID3D12Fence::Signal is vtable slot 10.
    let vtbl = std::ptr::read_unaligned(fence_ptr as *const *const usize);
    if vtbl.is_null() {
        return E_INVALIDARG;
    }
    let signal_fn_addr = std::ptr::read_unaligned(vtbl.add(10));
    if signal_fn_addr == 0 {
        return E_INVALIDARG;
    }
    let signal_fn: unsafe extern "win64" fn(usize, u64) -> i32 = std::mem::transmute(signal_fn_addr);
    signal_fn(fence_ptr, value)
}

unsafe extern "win64" fn ds_queue_qi(this: usize, riid: usize, ppv: *mut usize) -> i32 {
    qi_impl::<QueueObj>(this, riid, ppv, QUEUE_IIDS)
}

unsafe extern "win64" fn ds_queue_enqueue_request(this: usize, req_ptr: usize) {
    if req_ptr == 0 {
        return;
    }
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        return;
    };
    let q = &obj.inner.state;
    let raw_request = std::ptr::read_unaligned(
        req_ptr as *const [u8; std::mem::size_of::<DSTORAGE_REQUEST>()],
    );
    let req = std::ptr::read_unaligned(req_ptr as *const DSTORAGE_REQUEST);

    let compression_format = req.options.compression_format;
    let source_type = req.options.source_type();
    let dest_type = req.options.destination_type();

    if source_type as u64 != q.source_type {
        log::debug!(
            target: "dstorage",
            "EnqueueRequest source_type ({source_type}) differs from queue source_type ({})",
            q.source_type
        );
    }

    let request_name = read_cstr_bytes(req.name, DSTORAGE_REQUEST_MAX_NAME - 1);
    let mut filename = String::new();

    let source = match source_type {
        DSTORAGE_REQUEST_SOURCE_MEMORY => QueuedSource::Memory {
            ptr: le_usize(&req.source, 0),
            size: le_u32(&req.source, 8),
        },
        DSTORAGE_REQUEST_SOURCE_FILE => {
            let file_com = le_usize(&req.source, 0);
            let offset = le_u64(&req.source, 8);
            let size = le_u32(&req.source, 16);
            let file_arc = com_from_iface::<FileObj>(file_com, file_vtable().as_ptr())
                .map(|f| f.inner.state.clone());
            if let Some(ref f) = file_arc {
                filename = f.guest_path.clone();
            }
            QueuedSource::File {
                file: file_arc,
                offset,
                size,
            }
        }
        _ => QueuedSource::Invalid,
    };

    let dest = match dest_type {
        DSTORAGE_REQUEST_DESTINATION_MEMORY => QueuedDest::Memory {
            ptr: le_usize(&req.destination, 0),
            size: le_u32(&req.destination, 8),
        },
        DSTORAGE_REQUEST_DESTINATION_BUFFER => {
            let res_ptr = le_usize(&req.destination, 0);
            let offset = le_u64(&req.destination, 8);
            let size = le_u32(&req.destination, 16);
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::Buffer {
                resource,
                offset,
                size,
            }
        }
        DSTORAGE_REQUEST_DESTINATION_TEXTURE_REGION => {
            let res_ptr = le_usize(&req.destination, 0);
            let subresource_index = le_u32(&req.destination, 8);
            let region = D3D12Box {
                left: le_u32(&req.destination, 12),
                top: le_u32(&req.destination, 16),
                front: le_u32(&req.destination, 20),
                right: le_u32(&req.destination, 24),
                bottom: le_u32(&req.destination, 28),
                back: le_u32(&req.destination, 32),
            };
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::TextureRegion {
                resource,
                subresource_index,
                region,
            }
        }
        DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES => {
            let res_ptr = le_usize(&req.destination, 0);
            let first_subresource = le_u32(&req.destination, 8);
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::MultipleSubresources {
                resource,
                first_subresource,
            }
        }
        DSTORAGE_REQUEST_DESTINATION_TILES => {
            let res_ptr = le_usize(&req.destination, 0);
            let coord = TileCoord {
                x: le_u32(&req.destination, 8),
                y: le_u32(&req.destination, 12),
                z: le_u32(&req.destination, 16),
                subresource: le_u32(&req.destination, 20),
            };
            let region = TileRegionSize {
                num_tiles: le_u32(&req.destination, 24),
                use_box: le_u32(&req.destination, 28) != 0,
                width: le_u32(&req.destination, 32),
                height: le_u16(&req.destination, 36),
                depth: le_u16(&req.destination, 38),
            };
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::Tiles {
                resource,
                coord,
                region,
            }
        }
        DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES_RANGE => {
            let res_ptr = le_usize(&req.destination, 0);
            let first_subresource = le_u32(&req.destination, 8);
            let num_subresources = le_u32(&req.destination, 12);
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::MultipleSubresourcesRange {
                resource,
                first_subresource,
                num_subresources,
            }
        }
        _ => QueuedDest::Invalid,
    };

    q.enqueue_item(QueueItem::Request(QueuedRequest {
        compression_format,
        source,
        dest,
        uncompressed_size: req.uncompressed_size,
        cancellation_tag: req.cancellation_tag,
        request_name,
        filename,
        raw_request,
    }));
}

unsafe extern "win64" fn ds_queue_enqueue_status(
    this: usize,
    status_array: usize,
    index: u32,
) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        return;
    };
    let array = com_from_iface::<StatusArrayObj>(status_array, status_vtable().as_ptr())
        .map(|s| s.inner.state.clone());
    let valid_slot = array
        .as_ref()
        .is_some_and(|sa| sa.mark_pending(index));
    obj.inner.state.enqueue_item(QueueItem::Status {
        array_ptr: status_array,
        array,
        index,
        valid_slot,
    });
}

unsafe extern "win64" fn ds_queue_enqueue_signal(this: usize, fence_ptr: usize, value: u64) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        return;
    };
    let owned_d3d12_fence = if fence_ptr != 0 {
        if let Some(f) = Obj::<crate::d3d12::fence::Fence>::from_iface(fence_ptr as *mut c_void) {
            Obj::add_ref(f);
            true
        } else {
            false
        }
    } else {
        false
    };
    obj.inner.state.enqueue_item(QueueItem::Signal {
        fence: OwnedD3D12Fence {
            ptr: fence_ptr,
            owned: owned_d3d12_fence,
        },
        value,
    });
}

unsafe extern "win64" fn ds_queue_submit(this: usize) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        return;
    };
    obj.inner.state.execute_submit();
}

unsafe extern "win64" fn ds_queue_cancel_requests(this: usize, mask: u64, value: u64) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        return;
    };
    let mut st = obj.inner.state.inner.lock().unwrap();
    st.pending.retain(|item| match item {
        QueueItem::Request(req) => (req.cancellation_tag & mask) != value,
        _ => true,
    });
}

unsafe extern "win64" fn ds_queue_close(this: usize) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        return;
    };
    let q = &obj.inner.state;
    {
        let mut st = q.inner.lock().unwrap();
        st.closed = true;
        st.pending.clear();
        st.submitted.clear();
        st.in_flight = 0;
    }
    custom_decomp_state().cancel_pending_for_queue(q);
    let _exec = q.exec_lock.lock().unwrap();
    *q.uploader.lock().unwrap() = None;
}

unsafe extern "win64" fn ds_queue_get_error_event(this: usize) -> HANDLE {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        return std::ptr::null_mut();
    };
    crate::handles::insert(obj.inner.state.error_event.clone())
}

unsafe extern "win64" fn ds_queue_retrieve_error_record(
    this: usize,
    record_out: *mut DSTORAGE_ERROR_RECORD,
) {
    if record_out.is_null() {
        return;
    }
    let mut rec: DSTORAGE_ERROR_RECORD = std::mem::zeroed();
    rec.first_failure.command_type = DSTORAGE_COMMAND_TYPE_NONE;
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        std::ptr::write_unaligned(record_out, rec);
        return;
    };
    let q = &obj.inner.state;

    let (failure_count, first_failure) = {
        let mut st = q.inner.lock().unwrap();
        let count = st.failure_count;
        st.failure_count = 0;
        let first = st.first_failure.take();
        (count, first)
    };
    q.error_event.reset();

    rec.failure_count = failure_count;
    if let Some(first) = first_failure {
        rec.first_failure.hresult = first.hresult;
        rec.first_failure.command_type = first.command_type;
        match first.detail {
            FirstFailureDetail::Request {
                filename,
                request_name,
                raw_request,
            } => {
                let mut req_params: DSTORAGE_ERROR_PARAMETERS_REQUEST = std::mem::zeroed();
                for (i, unit) in filename
                    .encode_utf16()
                    .take(DSTORAGE_MAX_PATH - 1)
                    .enumerate()
                {
                    req_params.filename[i] = unit;
                }
                let copy_name = request_name.len().min(DSTORAGE_REQUEST_MAX_NAME - 1);
                req_params.request_name[..copy_name].copy_from_slice(&request_name[..copy_name]);
                req_params.request =
                    std::ptr::read_unaligned(raw_request.as_ptr() as *const DSTORAGE_REQUEST);
                rec.first_failure.params.request = req_params;
            }
            FirstFailureDetail::Status {
                status_array,
                index,
            } => {
                rec.first_failure.params.status = DSTORAGE_ERROR_PARAMETERS_STATUS {
                    status_array,
                    index,
                    _pad0: 0,
                };
            }
            FirstFailureDetail::Signal { fence, value } => {
                rec.first_failure.params.signal = DSTORAGE_ERROR_PARAMETERS_SIGNAL { fence, value };
            }
            FirstFailureDetail::Event { handle } => {
                rec.first_failure.params.event = DSTORAGE_ERROR_PARAMETERS_EVENT {
                    handle: handle as HANDLE,
                };
            }
        }
    }
    std::ptr::write_unaligned(record_out, rec);
}

unsafe extern "win64" fn ds_queue_query(this: usize, info_out: *mut DSTORAGE_QUEUE_INFO) {
    if info_out.is_null() {
        return;
    }
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        std::ptr::write_bytes(info_out as *mut u8, 0, std::mem::size_of::<DSTORAGE_QUEUE_INFO>());
        return;
    };
    let q = &obj.inner.state;
    let (used, pending_len) = {
        let st = q.inner.lock().unwrap();
        let p = st.pending.len();
        (p.saturating_add(st.in_flight), p)
    };
    let cap = q.capacity as usize;
    let empty_slots = cap.saturating_sub(used).min(u16::MAX as usize) as u16;
    let until_auto = (q.auto_submit_threshold as usize)
        .saturating_sub(pending_len)
        .min(empty_slots as usize) as u16;
    let info = DSTORAGE_QUEUE_INFO {
        desc: q.desc(),
        empty_slot_count: empty_slots,
        request_count_until_auto_submit: until_auto,
        _pad0: 0,
    };
    std::ptr::write_unaligned(info_out, info);
}

unsafe extern "win64" fn ds_queue_enqueue_set_event(this: usize, handle: HANDLE) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable().as_ptr()) else {
        return;
    };
    obj.inner.state.enqueue_item(QueueItem::Event {
        handle: handle as usize,
    });
}

extern "win64" fn ds_queue_get_compression_support(_this: usize, format: u8) -> u32 {
    match format {
        DSTORAGE_COMPRESSION_FORMAT_GDEFLATE => DSTORAGE_COMPRESSION_SUPPORT_CPU_FALLBACK,
        _ => DSTORAGE_COMPRESSION_SUPPORT_NONE,
    }
}

fn queue_vtable() -> &'static [usize; 14] {
    static VTBL: OnceLock<[usize; 14]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            ds_queue_qi as *const () as usize,
            com_add_ref_typed::<QueueObj> as *const () as usize,
            com_release_typed::<QueueObj> as *const () as usize,
            ds_queue_enqueue_request as *const () as usize,
            ds_queue_enqueue_status as *const () as usize,
            ds_queue_enqueue_signal as *const () as usize,
            ds_queue_submit as *const () as usize,
            ds_queue_cancel_requests as *const () as usize,
            ds_queue_close as *const () as usize,
            ds_queue_get_error_event as *const () as usize,
            ds_queue_retrieve_error_record as *const () as usize,
            ds_queue_query as *const () as usize,
            ds_queue_enqueue_set_event as *const () as usize,
            ds_queue_get_compression_support as *const () as usize,
        ])
    })
}

// ============================================================================
// IDStorageCustomDecompressionQueue / IDStorageCustomDecompressionQueue1
// Vtable order (dstorage.h):
//   0: QueryInterface, 1: AddRef, 2: Release,
//   3: GetEvent, 4: GetRequests, 5: SetRequestResults,
//   6: GetRequests1 (IDStorageCustomDecompressionQueue1)
// ============================================================================

/// Owned heap byte buffer whose raw address remains valid across moves / HashMap
/// rehashes (no `&mut` aliasing while caller worker threads write into `DstBuffer`).
struct RawHeapBuf {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

unsafe impl Send for RawHeapBuf {}

impl RawHeapBuf {
    fn from_vec(mut v: Vec<u8>) -> Self {
        let ptr = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();
        std::mem::forget(v);
        Self { ptr, len, cap }
    }

    fn into_vec(self) -> Vec<u8> {
        let v = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) };
        std::mem::forget(self);
        v
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn as_const_ptr(&self) -> *const c_void {
        if self.len == 0 {
            std::ptr::null()
        } else {
            self.ptr as *const c_void
        }
    }

    #[inline]
    fn as_mut_ptr(&self) -> *mut c_void {
        if self.len == 0 {
            std::ptr::null_mut()
        } else {
            self.ptr as *mut c_void
        }
    }
}

impl Drop for RawHeapBuf {
    fn drop(&mut self) {
        unsafe {
            drop(Vec::from_raw_parts(self.ptr, self.len, self.cap));
        }
    }
}

struct CustomDecompJob {
    id: u64,
    compression_format: u8,
    src_buf: RawHeapBuf,
    dst_buf: RawHeapBuf,
    queue: Arc<QueueState>,
    completion: Arc<Mutex<CustomDecompCompletion>>,
}

struct CustomDecompInner {
    pending: VecDeque<CustomDecompJob>,
    in_flight: HashMap<u64, CustomDecompJob>,
}

struct CustomDecompQueueState {
    next_id: AtomicU64,
    event: Arc<Event>,
    inner: Mutex<CustomDecompInner>,
}

impl CustomDecompQueueState {
    fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            event: Arc::new(Event::new(true, false, None)),
            inner: Mutex::new(CustomDecompInner {
                pending: VecDeque::new(),
                in_flight: HashMap::new(),
            }),
        }
    }

    fn enqueue(
        &self,
        compression_format: u8,
        src_bytes: Vec<u8>,
        dst_size: usize,
        queue: Arc<QueueState>,
        completion: Arc<Mutex<CustomDecompCompletion>>,
    ) {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let job = CustomDecompJob {
            id,
            compression_format,
            src_buf: RawHeapBuf::from_vec(src_bytes),
            dst_buf: RawHeapBuf::from_vec(vec![0u8; dst_size]),
            queue,
            completion,
        };
        let mut st = self.inner.lock().unwrap();
        st.pending.push_back(job);
        self.event.set();
    }

    fn cancel_pending_for_queue(&self, queue: &Arc<QueueState>) {
        let mut st = self.inner.lock().unwrap();
        st.pending.retain(|job| !Arc::ptr_eq(&job.queue, queue));
        if st.pending.is_empty() {
            self.event.reset();
        }
    }

    unsafe fn get_requests(
        &self,
        flags: u32,
        max_requests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        num_requests: *mut u32,
    ) -> i32 {
        if num_requests.is_null() {
            return E_INVALIDARG;
        }
        std::ptr::write_unaligned(num_requests, 0);
        if max_requests > 0 && requests.is_null() {
            return E_INVALIDARG;
        }
        if flags == 0 || (flags & !DSTORAGE_GET_REQUEST_FLAG_SELECT_ALL) != 0 {
            return E_INVALIDARG;
        }
        if max_requests == 0 || (flags & DSTORAGE_GET_REQUEST_FLAG_SELECT_CUSTOM) == 0 {
            return S_OK;
        }

        let mut st = self.inner.lock().unwrap();
        let mut count = 0u32;
        while count < max_requests {
            let Some(job) = st.pending.pop_front() else {
                break;
            };
            let raw = DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
                id: job.id,
                compression_format: job.compression_format,
                reserved: [0; 3],
                flags: DSTORAGE_CUSTOM_DECOMPRESSION_FLAG_NONE,
                src_size: job.src_buf.len() as u64,
                src_buffer: job.src_buf.as_const_ptr(),
                dst_size: job.dst_buf.len() as u64,
                dst_buffer: job.dst_buf.as_mut_ptr(),
            };
            st.in_flight.insert(job.id, job);
            std::ptr::write_unaligned(requests.add(count as usize), raw);
            count += 1;
        }
        if st.pending.is_empty() {
            self.event.reset();
        }
        drop(st);
        std::ptr::write_unaligned(num_requests, count);
        S_OK
    }

    unsafe fn set_request_results(
        &self,
        num_results: u32,
        results: *const DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
    ) -> i32 {
        if num_results == 0 {
            return S_OK;
        }
        if results.is_null() {
            return E_INVALIDARG;
        }

        let mut parsed = Vec::with_capacity(num_results as usize);
        for i in 0..num_results as usize {
            parsed.push(std::ptr::read_unaligned(results.add(i)));
        }

        let mut affected_queues: Vec<Arc<QueueState>> = Vec::new();
        {
            let mut st = self.inner.lock().unwrap();
            // Validate all IDs exist and are distinct before mutating state.
            for (idx, r) in parsed.iter().enumerate() {
                if !st.in_flight.contains_key(&r.id)
                    || parsed[..idx].iter().any(|prev| prev.id == r.id)
                {
                    return E_INVALIDARG;
                }
            }
            for r in parsed {
                let job = st.in_flight.remove(&r.id).unwrap();
                let decompressed = if r.result >= 0 {
                    job.dst_buf.into_vec()
                } else {
                    Vec::new()
                };
                *job.completion.lock().unwrap() = CustomDecompCompletion::Done {
                    hresult: r.result,
                    decompressed,
                };
                if !affected_queues
                    .iter()
                    .any(|q| Arc::ptr_eq(q, &job.queue))
                {
                    affected_queues.push(job.queue);
                }
            }
        }

        // Resume each affected queue outside custom_queue's mutex to preserve lock order.
        for q in affected_queues {
            q.drain_submitted();
        }
        S_OK
    }
}

fn custom_decomp_state() -> &'static CustomDecompQueueState {
    static STATE: OnceLock<CustomDecompQueueState> = OnceLock::new();
    STATE.get_or_init(CustomDecompQueueState::new)
}

struct CustomDecompQueueObj;

unsafe extern "win64" fn ds_custom_decomp_qi(this: usize, riid: usize, ppv: *mut usize) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, 0);
    if this == 0 {
        return E_INVALIDARG;
    }
    if riid == 0 {
        return E_POINTER;
    }
    let iid = std::ptr::read_unaligned(riid as *const Guid);
    if CUSTOM_DECOMP_IIDS.contains(&iid) {
        let obj = &*(this as *const ComObj<CustomDecompQueueObj>);
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this);
        S_OK
    } else if iid == IID_IUnknown || iid == IID_IDSTORAGE_FACTORY {
        std::ptr::write_unaligned(ppv, factory_singleton());
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn ds_custom_decomp_release(this: usize) -> u32 {
    if this == 0 {
        return 0;
    }
    let obj = &*(this as *const ComObj<CustomDecompQueueObj>);
    let mut cur = obj.refs.load(Ordering::Acquire);
    while cur > 1 {
        match obj.refs.compare_exchange_weak(
            cur,
            cur - 1,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return cur - 1,
            Err(v) => cur = v,
        }
    }
    1
}

unsafe extern "win64" fn ds_custom_decomp_get_event(this: usize) -> HANDLE {
    if com_from_iface::<CustomDecompQueueObj>(this, custom_decomp_vtable().as_ptr()).is_none() {
        return std::ptr::null_mut();
    }
    crate::handles::insert(custom_decomp_state().event.clone())
}

unsafe extern "win64" fn ds_custom_decomp_get_requests(
    this: usize,
    max_requests: u32,
    requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
    num_requests: *mut u32,
) -> i32 {
    if com_from_iface::<CustomDecompQueueObj>(this, custom_decomp_vtable().as_ptr()).is_none() {
        return E_INVALIDARG;
    }
    custom_decomp_state().get_requests(
        DSTORAGE_GET_REQUEST_FLAG_SELECT_CUSTOM,
        max_requests,
        requests,
        num_requests,
    )
}

unsafe extern "win64" fn ds_custom_decomp_set_request_results(
    this: usize,
    num_results: u32,
    results: *const DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
) -> i32 {
    if com_from_iface::<CustomDecompQueueObj>(this, custom_decomp_vtable().as_ptr()).is_none() {
        return E_INVALIDARG;
    }
    custom_decomp_state().set_request_results(num_results, results)
}

unsafe extern "win64" fn ds_custom_decomp_get_requests1(
    this: usize,
    flags: u32,
    max_requests: u32,
    requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
    num_requests: *mut u32,
) -> i32 {
    if com_from_iface::<CustomDecompQueueObj>(this, custom_decomp_vtable().as_ptr()).is_none() {
        return E_INVALIDARG;
    }
    custom_decomp_state().get_requests(flags, max_requests, requests, num_requests)
}

fn custom_decomp_vtable() -> &'static [usize; 7] {
    static VTBL: OnceLock<[usize; 7]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            ds_custom_decomp_qi as *const () as usize,
            com_add_ref_typed::<CustomDecompQueueObj> as *const () as usize,
            ds_custom_decomp_release as *const () as usize,
            ds_custom_decomp_get_event as *const () as usize,
            ds_custom_decomp_get_requests as *const () as usize,
            ds_custom_decomp_set_request_results as *const () as usize,
            ds_custom_decomp_get_requests1 as *const () as usize,
        ])
    })
}

fn custom_decomp_singleton() -> usize {
    static QUEUE: OnceLock<usize> = OnceLock::new();
    let ptr = *QUEUE.get_or_init(|| {
        let _ = custom_decomp_state();
        Box::into_raw(Box::new(ComObj {
            _vtable: custom_decomp_vtable().as_ptr(),
            refs: AtomicU32::new(1),
            inner: CustomDecompQueueObj,
        })) as usize
    });
    unsafe {
        let obj = &*(ptr as *const ComObj<CustomDecompQueueObj>);
        obj.refs.fetch_add(1, Ordering::AcqRel);
    }
    ptr
}

// ============================================================================
// IDStorageFactory
// Vtable order (dstorage.h):
//   0: QueryInterface, 1: AddRef, 2: Release,
//   3: CreateQueue, 4: OpenFile, 5: CreateStatusArray,
//   6: SetDebugFlags, 7: SetStagingBufferSize
// ============================================================================

static DEBUG_FLAGS: AtomicU32 = AtomicU32::new(0);
static STAGING_BUFFER_SIZE: AtomicU32 = AtomicU32::new(32 * 1024 * 1024);

struct FactoryObj;

unsafe extern "win64" fn ds_factory_qi(this: usize, riid: usize, ppv: *mut usize) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, 0);
    if this == 0 {
        return E_INVALIDARG;
    }
    if riid == 0 {
        return E_POINTER;
    }
    let iid = std::ptr::read_unaligned(riid as *const Guid);
    if iid == IID_IUnknown || iid == IID_IDSTORAGE_FACTORY {
        let obj = &*(this as *const ComObj<FactoryObj>);
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this);
        S_OK
    } else if CUSTOM_DECOMP_IIDS.contains(&iid) {
        std::ptr::write_unaligned(ppv, custom_decomp_singleton());
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn ds_factory_release(this: usize) -> u32 {
    if this == 0 {
        return 0;
    }
    let obj = &*(this as *const ComObj<FactoryObj>);
    // Process-wide singleton: keep a floor refcount of 1 so the static factory stays valid.
    let mut cur = obj.refs.load(Ordering::Acquire);
    while cur > 1 {
        match obj.refs.compare_exchange_weak(
            cur,
            cur - 1,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return cur - 1,
            Err(v) => cur = v,
        }
    }
    1
}

unsafe extern "win64" fn ds_factory_create_queue(
    _this: usize,
    desc_ptr: *const DSTORAGE_QUEUE_DESC,
    riid: usize,
    ppv: *mut usize,
) -> i32 {
    if ppv.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(ppv, 0);
    if desc_ptr.is_null() {
        return E_INVALIDARG;
    }
    match iid_allowed(riid, QUEUE_IIDS, true) {
        Ok(true) => {}
        Ok(false) => return E_NOINTERFACE,
        Err(hr) => return hr,
    }

    let desc = std::ptr::read_unaligned(desc_ptr);
    if desc.source_type != DSTORAGE_REQUEST_SOURCE_FILE as u64
        && desc.source_type != DSTORAGE_REQUEST_SOURCE_MEMORY as u64
    {
        return E_INVALIDARG;
    }
    if desc.capacity == 0 || desc.capacity > DSTORAGE_MAX_QUEUE_CAPACITY {
        return E_DSTORAGE_INVALID_QUEUE_CAPACITY;
    }

    let name_buf = if desc.name.is_null() {
        None
    } else {
        let mut bytes = read_cstr_bytes(desc.name, 255);
        bytes.push(0);
        Some(bytes.into_boxed_slice())
    };
    let auto_submit_threshold = (desc.capacity / 2).max(64).min(desc.capacity);
    let state = Arc::new(QueueState {
        source_type: desc.source_type,
        capacity: desc.capacity,
        auto_submit_threshold,
        priority: desc.priority,
        name_buf,
        device: desc.device,
        error_event: Arc::new(Event::new(true, false, None)),
        inner: Mutex::new(QueueInner {
            pending: VecDeque::new(),
            submitted: VecDeque::new(),
            in_flight: 0,
            closed: false,
            failure_count: 0,
            first_failure: None,
            batch_first_error: None,
        }),
        exec_lock: Mutex::new(()),
        uploader: Mutex::new(None),
    });
    let queue_obj = Box::into_raw(Box::new(ComObj {
        _vtable: queue_vtable().as_ptr(),
        refs: AtomicU32::new(1),
        inner: QueueObj { state },
    })) as usize;

    std::ptr::write_unaligned(ppv, queue_obj);
    S_OK
}

unsafe extern "win64" fn ds_factory_open_file(
    _this: usize,
    path_w: *const u16,
    riid: usize,
    ppv: *mut usize,
) -> i32 {
    if ppv.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(ppv, 0);
    if path_w.is_null() {
        return E_INVALIDARG;
    }
    match iid_allowed(riid, FILE_IIDS, true) {
        Ok(true) => {}
        Ok(false) => return E_NOINTERFACE,
        Err(hr) => return hr,
    }

    let guest_path = read_wide_z(path_w, 4096);
    if guest_path.is_empty() {
        return E_INVALIDARG;
    }
    let host_path = match crate::path::to_host(&guest_path) {
        Some(p) if p.exists() => p,
        other => {
            let direct = PathBuf::from(&guest_path);
            if direct.exists() {
                direct
            } else {
                other.unwrap_or(direct)
            }
        }
    };
    let file = match File::open(&host_path) {
        Ok(f) => f,
        Err(e) => return io_error_to_hresult(&e),
    };

    let state = Arc::new(DStorageFileState {
        guest_path,
        file: Mutex::new(Some(file)),
    });
    let obj = Box::into_raw(Box::new(ComObj {
        _vtable: file_vtable().as_ptr(),
        refs: AtomicU32::new(1),
        inner: FileObj { state },
    })) as usize;
    std::ptr::write_unaligned(ppv, obj);
    S_OK
}

unsafe extern "win64" fn ds_factory_create_status_array(
    _this: usize,
    capacity: u32,
    _name: *const u8,
    riid: usize,
    ppv: *mut usize,
) -> i32 {
    if ppv.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(ppv, 0);
    if capacity == 0 {
        return E_INVALIDARG;
    }
    match iid_allowed(riid, STATUS_ARRAY_IIDS, true) {
        Ok(true) => {}
        Ok(false) => return E_NOINTERFACE,
        Err(hr) => return hr,
    }

    let state = Arc::new(StatusArrayState::new(capacity as usize));
    let obj = Box::into_raw(Box::new(ComObj {
        _vtable: status_vtable().as_ptr(),
        refs: AtomicU32::new(1),
        inner: StatusArrayObj { state },
    })) as usize;
    std::ptr::write_unaligned(ppv, obj);
    S_OK
}

unsafe extern "win64" fn ds_factory_set_debug_flags(_this: usize, flags: u32) {
    DEBUG_FLAGS.store(flags, Ordering::Relaxed);
}

unsafe extern "win64" fn ds_factory_set_staging_buffer_size(_this: usize, size: u32) -> i32 {
    STAGING_BUFFER_SIZE.store(size, Ordering::Relaxed);
    S_OK
}

fn factory_vtable() -> &'static [usize; 8] {
    static VTBL: OnceLock<[usize; 8]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            ds_factory_qi as *const () as usize,
            com_add_ref_typed::<FactoryObj> as *const () as usize,
            ds_factory_release as *const () as usize,
            ds_factory_create_queue as *const () as usize,
            ds_factory_open_file as *const () as usize,
            ds_factory_create_status_array as *const () as usize,
            ds_factory_set_debug_flags as *const () as usize,
            ds_factory_set_staging_buffer_size as *const () as usize,
        ])
    })
}

fn factory_singleton() -> usize {
    static FACTORY: OnceLock<usize> = OnceLock::new();
    let ptr = *FACTORY.get_or_init(|| {
        Box::into_raw(Box::new(ComObj {
            _vtable: factory_vtable().as_ptr(),
            refs: AtomicU32::new(1),
            inner: FactoryObj,
        })) as usize
    });
    unsafe {
        let obj = &*(ptr as *const ComObj<FactoryObj>);
        obj.refs.fetch_add(1, Ordering::AcqRel);
    }
    ptr
}

// ============================================================================
// IDStorageCompressionCodec (IUnknown + CompressBuffer + DecompressBuffer + CompressBufferBound)
// ============================================================================

struct CompressionCodecObj;

unsafe extern "win64" fn ds_codec_qi(this: usize, riid: usize, ppv: *mut usize) -> i32 {
    qi_impl::<CompressionCodecObj>(this, riid, ppv, CODEC_IIDS)
}

unsafe extern "win64" fn ds_codec_compress_buffer(
    _this: usize,
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
    std::ptr::write_unaligned(compressed_size_out, 0);
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
    std::ptr::write_unaligned(compressed_size_out, out.len());
    S_OK
}

unsafe extern "win64" fn ds_codec_decompress_buffer(
    _this: usize,
    compressed: *const u8,
    compressed_size: usize,
    uncompressed: *mut u8,
    uncompressed_buf_size: usize,
    uncompressed_size_out: *mut usize,
) -> i32 {
    if uncompressed_size_out.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(uncompressed_size_out, 0);
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
            std::ptr::write_unaligned(uncompressed_size_out, written);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds_codec_compress_buffer_bound(
    _this: usize,
    uncompressed_size: usize,
) -> usize {
    compress_gdeflate_bound(uncompressed_size)
}

fn codec_vtable() -> &'static [usize; 6] {
    static VTBL: OnceLock<[usize; 6]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            ds_codec_qi as *const () as usize,
            com_add_ref_typed::<CompressionCodecObj> as *const () as usize,
            com_release_typed::<CompressionCodecObj> as *const () as usize,
            ds_codec_compress_buffer as *const () as usize,
            ds_codec_decompress_buffer as *const () as usize,
            ds_codec_compress_buffer_bound as *const () as usize,
        ])
    })
}

// ============================================================================
// Exported C APIs (dstorage.dll / dstoragecore.dll)
// ============================================================================

pub extern "win64" fn DStorageSetConfiguration(_cfg: usize) -> i32 {
    S_OK
}

pub extern "win64" fn DStorageSetConfiguration1(_cfg: usize) -> i32 {
    S_OK
}

pub extern "win64" fn DStorageGetFactory(riid: usize, ppv: *mut usize) -> i32 {
    if ppv.is_null() {
        return E_INVALIDARG;
    }
    unsafe {
        std::ptr::write_unaligned(ppv, 0);
        if riid == 0 {
            std::ptr::write_unaligned(ppv, factory_singleton());
            return S_OK;
        }
        let iid = std::ptr::read_unaligned(riid as *const Guid);
        if iid == IID_IUnknown || iid == IID_IDSTORAGE_FACTORY {
            std::ptr::write_unaligned(ppv, factory_singleton());
            S_OK
        } else if CUSTOM_DECOMP_IIDS.contains(&iid) {
            std::ptr::write_unaligned(ppv, custom_decomp_singleton());
            S_OK
        } else {
            E_NOINTERFACE
        }
    }
}

pub extern "win64" fn DStorageCreateCompressionCodec(
    format: u8,
    _num_threads: u32,
    riid: usize,
    ppv: *mut usize,
) -> i32 {
    if ppv.is_null() {
        return E_INVALIDARG;
    }
    unsafe {
        std::ptr::write_unaligned(ppv, 0);
    }
    match format {
        DSTORAGE_COMPRESSION_FORMAT_GDEFLATE => {}
        DSTORAGE_COMPRESSION_FORMAT_NONE => return E_INVALIDARG,
        _ => {
            crate::stub!(
                "DStorageCreateCompressionCodec: unsupported format {format:#x} -> E_NOTIMPL"
            );
            return E_NOTIMPL;
        }
    }
    unsafe {
        match iid_allowed(riid, CODEC_IIDS, true) {
            Ok(true) => {}
            Ok(false) => return E_NOINTERFACE,
            Err(hr) => return hr,
        }
        let obj = Box::into_raw(Box::new(ComObj {
            _vtable: codec_vtable().as_ptr(),
            refs: AtomicU32::new(1),
            inner: CompressionCodecObj,
        })) as usize;
        std::ptr::write_unaligned(ppv, obj);
    }
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{WAIT_OBJECT_0, WAIT_TIMEOUT};

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

        // Unsupported / custom compression codecs must fail honestly.
        let mut bad_codec = 0usize;
        assert_eq!(
            DStorageCreateCompressionCodec(DSTORAGE_COMPRESSION_FORMAT_NONE, 1, 0, &mut bad_codec),
            E_INVALIDARG
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
}
