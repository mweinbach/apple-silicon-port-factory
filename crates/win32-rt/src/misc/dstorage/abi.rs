//! DirectStorage 1.0 - 1.2 Windows x64 ABI constants, IIDs, HRESULTs, and
//! `#[repr(C)]` structures (`dstorage.h` / `dstorageerr.h`).

use std::ffi::c_void;
use std::io::ErrorKind;

use crate::com::{guid, read_guid, Guid, IID_IUnknown};
pub(super) use crate::types::{
    errno_to_win32, E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, E_POINTER,
    HANDLE, S_OK,
};

// ============================================================================
// DirectStorage HRESULTs & Constants (dstorage.h / dstorageerr.h)
// ============================================================================

pub(super) const E_PENDING: i32 = 0x8000_000Au32 as i32;
pub(super) const E_HANDLE: i32 = 0x8007_0006u32 as i32;

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

pub(super) const DSTORAGE_DEBUG_SHOW_ERRORS: u32 = 0x1;
pub(super) const D3D12_TILE_SIZE_BYTES: usize = 65536;

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

pub(super) const FILE_IIDS: &[Guid] = &[IID_IDSTORAGE_FILE];
pub(super) const STATUS_ARRAY_IIDS: &[Guid] = &[IID_IDSTORAGE_STATUS_ARRAY];
pub(super) const QUEUE_IIDS: &[Guid] =
    &[IID_IDSTORAGE_QUEUE, IID_IDSTORAGE_QUEUE1, IID_IDSTORAGE_QUEUE2];
pub(super) const CODEC_IIDS: &[Guid] = &[IID_IDSTORAGE_COMPRESSION_CODEC];
pub(super) const CUSTOM_DECOMP_IIDS: &[Guid] = &[
    IID_IDSTORAGE_CUSTOM_DECOMPRESSION_QUEUE,
    IID_IDSTORAGE_CUSTOM_DECOMPRESSION_QUEUE1,
];

#[inline]
pub(super) fn hresult_from_win32(win32_err: u32) -> i32 {
    if win32_err == 0 {
        S_OK
    } else {
        (0x8007_0000u32 | (win32_err & 0xFFFF)) as i32
    }
}

pub(super) fn io_error_to_hresult(err: &std::io::Error) -> i32 {
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

/// Check whether `riid` is NULL (accepted by DirectStorage factory/create helpers)
/// or matches `IID_IUnknown` or any IID in `allowed`.
#[inline]
pub(super) unsafe fn iid_matches_or_null(riid: usize, allowed: &[Guid]) -> bool {
    match read_guid(riid as *const Guid) {
        None => true,
        Some(iid) => iid == IID_IUnknown || allowed.contains(&iid),
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
pub(super) struct D3D12Box {
    pub(super) left: u32,
    pub(super) top: u32,
    pub(super) front: u32,
    pub(super) right: u32,
    pub(super) bottom: u32,
    pub(super) back: u32,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct TileCoord {
    pub(super) x: u32,
    pub(super) y: u32,
    pub(super) z: u32,
    pub(super) subresource: u32,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct TileRegionSize {
    pub(super) num_tiles: u32,
    pub(super) use_box: bool,
    pub(super) width: u32,
    pub(super) height: u16,
    pub(super) depth: u16,
}
