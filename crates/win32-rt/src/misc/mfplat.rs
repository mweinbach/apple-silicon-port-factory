//! `mfplat.dll`: Native Microsoft Media Foundation platform implementation.
//!
//! Provides real ref-counted Media Foundation COM objects (with every vtable slot
//! registered via [`crate::com::register_vtable`]):
//! - Ref-counted `MFStartup` / `MFShutdown`.
//! - `MFCreateMemoryBuffer` / `MFCreateAlignedMemoryBuffer` / `MFCreate2DMediaBuffer`
//!   (`IMFMediaBuffer` + `IMF2DBuffer` + `IMF2DBuffer2`).
//! - `MFCreateSample` (`IMFSample` inheriting `IMFAttributes`: buffer list management,
//!   contiguous flatten, timestamps, duration, flags, total length, `CopyToBuffer`).
//! - `MFCreateMediaType` & `MFCreateAttributes` (`IMFMediaType` / `IMFAttributes`
//!   reentrant-locked GUID key-value store supporting UINT32, UINT64, Double, GUID,
//!   UTF-16 String, Blob, and `IUnknown` items + `PROPVARIANT` round-trip).
//! - `MFCopyImage` (strided 2D row copy supporting positive and negative pitches).
//! - `MFCreateDXGIDeviceManager` (`IMFDXGIDeviceManager`: `ResetDevice`,
//!   `OpenDeviceHandle`, `CloseDeviceHandle`, `TestDevice`, `LockDevice`,
//!   `UnlockDevice`, `GetVideoService`).
//! - `MFCreateDXGISurfaceBuffer` (`IMFMediaBuffer` + `IMF2DBuffer2` + `IMFDXGIBuffer`,
//!   wrapping real `IDXGISurface` / D3D11 textures or D3D12 buffers).

mod attributes;
mod buffer;
mod dxgi_buffer;
mod dxgi_manager;
mod sample;

use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::com::{guid, Guid};
use crate::types::{BOOL, HRESULT, S_OK};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ============================================================================
// Media Foundation HRESULTs, Constants & IIDs (mfobjects.h / mferror.h)
// ============================================================================

pub const E_HANDLE: HRESULT = 0x8007_0006u32 as HRESULT;
pub const E_UNEXPECTED: HRESULT = 0x8000_FFFFu32 as HRESULT;
pub const STRSAFE_E_INSUFFICIENT_BUFFER: HRESULT = 0x8007_007Au32 as HRESULT;

pub const MF_E_BUFFERTOOSMALL: HRESULT = 0xC00D_36B1u32 as HRESULT;
pub const MF_E_INVALIDREQUEST: HRESULT = 0xC00D_36B2u32 as HRESULT;
pub const MF_E_NOT_INITIALIZED: HRESULT = 0xC00D_36B6u32 as HRESULT;
pub const MF_E_INVALIDTYPE: HRESULT = 0xC00D_36BDu32 as HRESULT;
pub const MF_E_INVALIDINDEX: HRESULT = 0xC00D_36BFu32 as HRESULT;
pub const MF_E_NO_SAMPLE_TIMESTAMP: HRESULT = 0xC00D_36C8u32 as HRESULT;
pub const MF_E_NO_SAMPLE_DURATION: HRESULT = 0xC00D_36C9u32 as HRESULT;
pub const MF_E_BAD_STARTUP_VERSION: HRESULT = 0xC00D_36E3u32 as HRESULT;
pub const MF_E_ATTRIBUTENOTFOUND: HRESULT = 0xC00D_36E6u32 as HRESULT;
pub const MF_E_NOT_FOUND: HRESULT = 0xC00D_36D5u32 as HRESULT;

pub const MF_E_DXGI_DEVICE_NOT_INITIALIZED: HRESULT = 0x8004_1000u32 as HRESULT;
pub const MF_E_DXGI_NEW_VIDEO_DEVICE: HRESULT = 0x8004_1001u32 as HRESULT;
pub const MF_E_DXGI_VIDEO_DEVICE_LOCKED: HRESULT = 0x8004_1002u32 as HRESULT;

pub const MF_API_VERSION: u32 = 0x0070;
pub const MF_SDK_VERSION: u32 = 0x0002;
pub const MF_VERSION: u32 = (MF_SDK_VERSION << 16) | MF_API_VERSION;
pub const MFSTARTUP_FULL: u32 = 0;
pub const MFSTARTUP_NOSOCKET: u32 = 1;
pub const MFSTARTUP_LITE: u32 = 1;

// MF_ATTRIBUTE_TYPE (VARTYPE values)
pub const MF_ATTRIBUTE_UINT32: u32 = 19; // VT_UI4
pub const MF_ATTRIBUTE_UINT64: u32 = 21; // VT_UI8
pub const MF_ATTRIBUTE_DOUBLE: u32 = 5; // VT_R8
pub const MF_ATTRIBUTE_GUID: u32 = 72; // VT_CLSID
pub const MF_ATTRIBUTE_STRING: u32 = 31; // VT_LPWSTR
pub const MF_ATTRIBUTE_BLOB: u32 = 4113; // VT_VECTOR | VT_UI1
pub const MF_ATTRIBUTE_IUNKNOWN: u32 = 13; // VT_UNKNOWN

// MF_ATTRIBUTES_MATCH_TYPE
pub const MF_ATTRIBUTES_MATCH_OUR_ITEMS: u32 = 0;
pub const MF_ATTRIBUTES_MATCH_THEIR_ITEMS: u32 = 1;
pub const MF_ATTRIBUTES_MATCH_ALL_ITEMS: u32 = 2;
pub const MF_ATTRIBUTES_MATCH_INTERSECTION: u32 = 3;
pub const MF_ATTRIBUTES_MATCH_SMALLER: u32 = 4;

// MF_MEDIATYPE_EQUAL_*
pub const MF_MEDIATYPE_EQUAL_MAJOR_TYPES: u32 = 0x0000_0001;
pub const MF_MEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 0x0000_0002;
pub const MF_MEDIATYPE_EQUAL_FORMAT_DATA: u32 = 0x0000_0004;
pub const MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 0x0000_0008;

// Well-known MF MediaType attribute GUIDs
pub const MF_MT_MAJOR_TYPE: Guid = guid("48eba18e-f8c9-4687-bf11-0a74c9f96a8f");
pub const MF_MT_SUBTYPE: Guid = guid("f7e34c9a-42e8-4714-b74b-cb29d72c35e5");
pub const MF_MT_ALL_SAMPLES_INDEPENDENT: Guid = guid("c9173739-5e56-461c-b713-46fb995cb95f");
pub const MF_MT_FIXED_SIZE_SAMPLES: Guid = guid("b8ebefaf-b718-4e04-b0a9-116775e3321b");
pub const MF_MT_USER_DATA: Guid = guid("b6bc765f-4c3b-40a4-bd51-2535b66fe09d");

// Official Media Foundation Interface IDs (verified against mfobjects.h / WinMD)
pub const IID_IMFAttributes: Guid = guid("2cd2d921-c447-44a7-a13c-4adabfc247e3");
pub const IID_IMFMediaType: Guid = guid("44ae0fa8-ea31-4109-8d2e-4cae4997c555");
pub const IID_IMFMediaBuffer: Guid = guid("045fa593-8799-42b8-bc8d-8968c6453507");
pub const IID_IMF2DBuffer: Guid = guid("7dc9d5f9-9ed9-44ec-9bbf-0600bb589fbb");
pub const IID_IMF2DBuffer2: Guid = guid("33ae5ea6-4316-436f-8ddd-d73d22f829ec");
pub const IID_IMFSample: Guid = guid("c40a00f2-b93a-4d80-ae8c-5a1c634f58e4");
pub const IID_IMFDXGIDeviceManager: Guid = guid("eb533d5d-2db6-40f8-97a9-494692014f07");
pub const IID_IMFDXGIBuffer: Guid = guid("e7174cfa-1c9e-48b1-8866-626226bfc258");

// ============================================================================
// Platform Lifecycle & Win64 Export Facade
// ============================================================================

static MF_STARTUP_REFS: AtomicU32 = AtomicU32::new(0);

/// Current `MFStartup` reference count (useful for diagnostics and tests).
pub fn startup_ref_count() -> u32 {
    MF_STARTUP_REFS.load(Ordering::Acquire)
}

pub unsafe extern "win64" fn MFStartup(version: u32, dw_flags: u32) -> HRESULT {
    let api_ver = version & 0xFFFF;
    let sdk_ver = version >> 16;
    if api_ver != MF_API_VERSION || (sdk_ver != 1 && sdk_ver != 2) {
        log::warn!(target: "mfplat", "MFStartup(version={version:#x}, flags={dw_flags:#x}) -> MF_E_BAD_STARTUP_VERSION");
        return MF_E_BAD_STARTUP_VERSION;
    }
    let count = MF_STARTUP_REFS.fetch_add(1, Ordering::AcqRel) + 1;
    log::debug!(target: "mfplat", "MFStartup(version={version:#x}, flags={dw_flags:#x}) -> S_OK (refs={count})");
    S_OK
}

pub unsafe extern "win64" fn MFShutdown() -> HRESULT {
    let prev = MF_STARTUP_REFS
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| Some(n.saturating_sub(1)))
        .unwrap_or(0);
    log::debug!(target: "mfplat", "MFShutdown() -> S_OK (refs={})", prev.saturating_sub(1));
    S_OK
}

pub unsafe extern "win64" fn MFCopyImage(
    p_dest: *mut u8,
    l_dest_stride: i32,
    p_src: *const u8,
    l_src_stride: i32,
    dw_width_in_bytes: u32,
    dw_lines: u32,
) -> HRESULT {
    buffer::copy_image_rows(p_dest, l_dest_stride, p_src, l_src_stride, dw_width_in_bytes, dw_lines)
}

pub unsafe extern "win64" fn MFCreateAttributes(
    pp_mf_attributes: *mut *mut c_void,
    c_initial_size: u32,
) -> HRESULT {
    attributes::create_attributes(pp_mf_attributes, c_initial_size)
}

pub unsafe extern "win64" fn MFCreateMediaType(pp_mf_type: *mut *mut c_void) -> HRESULT {
    attributes::create_media_type(pp_mf_type)
}

pub unsafe extern "win64" fn MFCreateMemoryBuffer(
    cb_max_length: u32,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    buffer::create_memory_buffer(cb_max_length, pp_buffer)
}

pub unsafe extern "win64" fn MFCreateAlignedMemoryBuffer(
    cb_max_length: u32,
    cb_alignment: u32,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    buffer::create_aligned_memory_buffer(cb_max_length, cb_alignment, pp_buffer)
}

pub unsafe extern "win64" fn MFCreate2DMediaBuffer(
    dw_width: u32,
    dw_height: u32,
    dw_fourcc: u32,
    f_bottom_up: BOOL,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    buffer::create_2d_media_buffer(dw_width, dw_height, dw_fourcc, f_bottom_up, pp_buffer)
}

pub unsafe extern "win64" fn MFCreateSample(pp_imf_sample: *mut *mut c_void) -> HRESULT {
    sample::create_sample(pp_imf_sample)
}

pub unsafe extern "win64" fn MFCreateDXGIDeviceManager(
    reset_token: *mut u32,
    pp_device_manager: *mut *mut c_void,
) -> HRESULT {
    dxgi_manager::create_dxgi_device_manager(reset_token, pp_device_manager)
}

pub unsafe extern "win64" fn MFCreateDXGISurfaceBuffer(
    riid: *const Guid,
    punk_surface: *mut c_void,
    u_subresource_index: u32,
    f_bottom_up_when_linear: BOOL,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    dxgi_buffer::create_dxgi_surface_buffer(
        riid,
        punk_surface,
        u_subresource_index,
        f_bottom_up_when_linear,
        pp_buffer,
    )
}

// ============================================================================
// Unit Tests (must remain at the bottom for win32-rt/build.rs export scanner)
// ============================================================================

#[cfg(test)]
mod tests;
