//! Native `dstorage.dll` / `dstoragecore.dll` (Microsoft DirectStorage 1.0 - 1.2)
//! implementation for Apple Silicon SSD + Metal 4 Unified Memory.
//!
//! Implements:
//! - Genuine Microsoft GDeflate v1 (`TileStreamHeader` + `tileOffsets[]` + 64 KiB
//!   raw-DEFLATE tiles via [`crate::misc::zlib1::ZlibStream`]) compression and
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

mod abi;
mod factory;
mod gdeflate;
mod queue;

pub use abi::*;
pub use gdeflate::{
    compress_gdeflate, compress_gdeflate_bound, decompress_gdeflate, decompress_gdeflate_to_vec,
    GDEFLATE_TILE_SIZE,
};

use crate::com::{read_guid, Guid, IID_IUnknown};
use crate::strings::wr;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ============================================================================
// Exported C APIs (dstorage.dll / dstoragecore.dll)
// ============================================================================

pub extern "win64" fn DStorageSetConfiguration(_cfg: usize) -> i32 {
    abi::S_OK
}

pub extern "win64" fn DStorageSetConfiguration1(_cfg: usize) -> i32 {
    abi::S_OK
}

pub extern "win64" fn DStorageGetFactory(riid: usize, ppv: *mut usize) -> i32 {
    if ppv.is_null() {
        return abi::E_INVALIDARG;
    }
    unsafe {
        wr(ppv, 0);
        let Some(iid) = read_guid(riid as *const Guid) else {
            wr(ppv, factory::factory_singleton());
            return abi::S_OK;
        };
        if iid == IID_IUnknown || iid == IID_IDSTORAGE_FACTORY {
            wr(ppv, factory::factory_singleton());
            abi::S_OK
        } else if abi::CUSTOM_DECOMP_IIDS.contains(&iid) {
            wr(ppv, queue::custom_decomp_singleton());
            abi::S_OK
        } else {
            abi::E_NOINTERFACE
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
        return abi::E_INVALIDARG;
    }
    unsafe {
        wr(ppv, 0);
    }
    match format {
        DSTORAGE_COMPRESSION_FORMAT_GDEFLATE => {}
        DSTORAGE_COMPRESSION_FORMAT_NONE => return abi::E_INVALIDARG,
        _ => {
            crate::stub!(
                "DStorageCreateCompressionCodec: unsupported format {format:#x} -> E_NOTIMPL"
            );
            return abi::E_NOTIMPL;
        }
    }
    unsafe {
        if !abi::iid_matches_or_null(riid, abi::CODEC_IIDS) {
            return abi::E_NOINTERFACE;
        }
        wr(ppv, gdeflate::create_compression_codec());
    }
    abi::S_OK
}

#[cfg(test)]
mod tests;
